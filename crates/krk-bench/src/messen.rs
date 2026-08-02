//! Die kopflose Messstrecke.
//!
//! Gemessen werden die beiden Spannen, die ohne Fenster feststellbar sind:
//!
//! - **Lesen bis zum ersten Stapel.** Vom Start des Lesevorgangs bis zu dem
//!   Zeitpunkt, an dem der erste Stapel im Ordnermodell steht. Das ist der
//!   Anteil, den der Kern an L2 hat ("erste Bildschirmseite sichtbar und
//!   bedienbar"); was das Zeichnen dazulegt, misst erst S21.
//! - **Vollstaendiges Lesen samt Sortierung.** Vom selben Start bis zum
//!   Abschluss des Modells, also einschliesslich des Sortierens. Das ist L3 fuer
//!   den Ordner mit 10.000 Eintraegen und L10 fuer den mit 100.000.
//!
//! # Wovon eine Zahl hier abhaengt
//!
//! Drei Dinge entscheiden, ob die Zahl am Ende etwas wert ist.
//!
//! **Der Cache-Zustand.** "Kalt" heisst laut Spec: erster Zugriff nach dem
//! Leeren des Dateisystem-Caches. Das gilt fuer **jede** der zwanzig
//! Wiederholungen, nicht nur fuer die erste — sonst waeren neunzehn davon warm
//! und stuenden trotzdem unter der Ueberschrift "kalt". Deshalb ruft
//! [`Messreihe::fahren`] im kalten Betrieb `purge` vor jedem einzelnen Lauf.
//! Im warmen Betrieb laeuft umgekehrt ein ungezaehlter Vorlauf, denn "warm"
//! heisst: jeder **weitere** Zugriff.
//!
//! **Die Vollstaendigkeit jedes Laufs.** Ein Lauf, der abbricht oder weniger
//! Eintraege liefert als der vorige, misst etwas anderes. Solche Laeufe zaehlen
//! nicht als schnell, sie zaehlen als Fehler und beenden die Messung.
//!
//! **Das Perzentil.** Die Zusagen aus C8 gelten fuer das 95. Perzentil, nicht
//! fuer den Mittelwert. Ein Mittelwert ueber zwanzig Laeufe verbirgt genau den
//! Ausreisser, den ein Nutzer bemerkt.

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use krk_core::verzeichnis::leser::{Lesevorgang, Meldung};
use krk_core::verzeichnis::modell::Ordnermodell;

/// Wie oft jede Messung wiederholt wird. C8 schreibt zwanzig vor.
pub const WIEDERHOLUNGEN: usize = 20;

/// Der Anteil, fuer den die Zusagen aus C8 gelten.
pub const PERZENTIL: f64 = 0.95;

/// Das Werkzeug, das den Dateisystem-Cache leert.
const PURGE: &str = "/usr/sbin/purge";

/// In welchem Cache-Zustand gemessen wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cache {
    /// Vor jedem Lauf wird der Dateisystem-Cache geleert.
    Kalt,
    /// Ein ungezaehlter Vorlauf waermt den Cache, danach wird gemessen.
    Warm,
}

impl Cache {
    /// Die Beschreibung fuer den Berichtskopf.
    pub fn beschreibung(self) -> &'static str {
        match self {
            Cache::Kalt => "kalt (purge vor jedem einzelnen Lauf)",
            Cache::Warm => "warm (ein ungezaehlter Vorlauf, danach nur weitere Zugriffe)",
        }
    }
}

/// Was ein einzelner Lauf ergeben hat.
#[derive(Debug, Clone, Copy)]
struct Lauf {
    bis_erster_stapel: Duration,
    bis_vollstaendig: Duration,
    eintraege: usize,
}

/// Die gesammelten Werte einer Messgroesse.
#[derive(Debug, Clone)]
pub struct Messgroesse {
    /// Wie die Messgroesse im Bericht heisst.
    pub name: &'static str,
    /// Auf welche Zusagen aus C8 sie zahlt.
    pub zusagen: &'static str,
    /// Die Einzelwerte in Laufreihenfolge.
    pub werte: Vec<Duration>,
}

impl Messgroesse {
    /// Das 95. Perzentil ueber alle Laeufe.
    pub fn perzentil95(&self) -> Duration {
        perzentil(&self.werte, PERZENTIL)
    }

    /// Der Median ueber alle Laeufe.
    pub fn median(&self) -> Duration {
        median(&self.werte)
    }

    /// Der kleinste gemessene Wert.
    pub fn minimum(&self) -> Duration {
        self.werte.iter().copied().min().unwrap_or_default()
    }
}

/// Das Ergebnis einer vollstaendigen Messreihe.
#[derive(Debug, Clone)]
pub struct Messreihe {
    /// Der gemessene Ordner.
    pub ordner: PathBuf,
    /// Der Cache-Zustand, in dem gemessen wurde.
    pub cache: Cache,
    /// Die Zahl der gezaehlten Laeufe.
    pub wiederholungen: usize,
    /// Wie viele Eintraege jeder Lauf gelesen hat.
    pub eintraege: usize,
    /// Die beiden Messgroessen.
    pub groessen: Vec<Messgroesse>,
}

impl Messreihe {
    /// Faehrt die Messreihe.
    ///
    /// `wiederholungen` ist ausdruecklich ein Parameter und keine feste Zahl,
    /// damit die Proben die Strecke mit drei Laeufen durchfahren koennen. Der
    /// Aufrufer aus `main` setzt [`WIEDERHOLUNGEN`] ein, und der Bericht
    /// schreibt die tatsaechliche Zahl aus.
    pub fn fahren(ordner: &Path, cache: Cache, wiederholungen: usize) -> io::Result<Self> {
        if wiederholungen == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "eine Messreihe ohne Laeufe ergibt keine Zahl",
            ));
        }

        if cache == Cache::Warm {
            // Der ungezaehlte Vorlauf. Ohne ihn traegt der erste von zwanzig
            // Laeufen eine kalte Zahl in eine warme Reihe.
            einen_lauf_fahren(ordner)?;
        }

        let mut laeufe = Vec::with_capacity(wiederholungen);
        for nummer in 0..wiederholungen {
            if cache == Cache::Kalt {
                cache_leeren().map_err(|fehler| {
                    io::Error::other(format!(
                        "vor Lauf {} von {wiederholungen}: {fehler}",
                        nummer + 1
                    ))
                })?;
            }
            laeufe.push(einen_lauf_fahren(ordner)?);
        }

        let eintraege = laeufe[0].eintraege;
        for (nummer, lauf) in laeufe.iter().enumerate() {
            if lauf.eintraege != eintraege {
                return Err(io::Error::other(format!(
                    "Lauf {} hat {} Eintraege gelesen, Lauf 1 aber {eintraege}. \
                     Die Laeufe messen nicht dasselbe; die Reihe wird verworfen.",
                    nummer + 1,
                    lauf.eintraege
                )));
            }
        }

        Ok(Self {
            ordner: ordner.to_path_buf(),
            cache,
            wiederholungen,
            eintraege,
            groessen: vec![
                Messgroesse {
                    name: "Lesen bis zum ersten Stapel",
                    zusagen: "Anteil an L2",
                    werte: laeufe.iter().map(|lauf| lauf.bis_erster_stapel).collect(),
                },
                Messgroesse {
                    name: "Vollstaendiges Lesen samt Sortierung",
                    zusagen: "L3, L10",
                    werte: laeufe.iter().map(|lauf| lauf.bis_vollstaendig).collect(),
                },
            ],
        })
    }
}

/// Ein einzelner Lauf: Ordner lesen, Modell fuellen, Sortierung herstellen.
fn einen_lauf_fahren(ordner: &Path) -> io::Result<Lauf> {
    let generation = 1;
    let mut modell = Ordnermodell::neu(generation);
    let mut bis_erster_stapel = None;

    let beginn = Instant::now();
    let vorgang = Lesevorgang::starten(ordner.to_path_buf(), generation);
    loop {
        let meldung = vorgang.meldungen().recv().map_err(|_| {
            io::Error::other(format!(
                "der Leser von {} hat den Kanal geschlossen, ohne fertig zu melden",
                ordner.display()
            ))
        })?;
        match meldung {
            Meldung::Stapel {
                generation: gemeldet,
                eintraege,
            } => {
                debug_assert!(modell.gehoert_dazu(gemeldet));
                modell.anhaengen(eintraege);
                // Nach dem Anhaengen, nicht davor: erst dann steht die erste
                // Bildschirmseite im Modell, und genau das sagt L2 zu.
                if bis_erster_stapel.is_none() {
                    bis_erster_stapel = Some(beginn.elapsed());
                }
            }
            Meldung::Fertig { abschluss, .. } => {
                if !abschluss.ist_vollstaendig() {
                    return Err(io::Error::other(format!(
                        "der Lauf auf {} endete mit {abschluss:?} statt vollstaendig",
                        ordner.display()
                    )));
                }
                modell.abschliessen();
                break;
            }
        }
    }
    let bis_vollstaendig = beginn.elapsed();
    // Erst nach dem Stoppen der Uhr auf den Arbeitsfaden warten, damit die
    // Zeit fuer das Zusammenfuehren des Fadens nicht in der Messgroesse landet.
    vorgang.warten();

    let bis_erster_stapel = bis_erster_stapel.ok_or_else(|| {
        io::Error::other(format!(
            "{} hat keinen einzigen Stapel geliefert; ein leerer Ordner taugt nicht als Pruefordner",
            ordner.display()
        ))
    })?;

    Ok(Lauf {
        bis_erster_stapel,
        bis_vollstaendig,
        eintraege: modell.eintraege().len(),
    })
}

/// Leert den Dateisystem-Cache.
///
/// Bricht ab, sobald etwas nicht stimmt, statt weiterzumessen. Der Grund steht
/// im Plan: eine warme Zahl unter der Ueberschrift "kalt" ist schlimmer als
/// gar keine Zahl, weil sie ein Gate besteht, das sie nicht bestehen duerfte.
/// Geprueft wird nicht nur der Rueckgabewert, sondern auch, ob `purge` etwas
/// auf die Fehlerausgabe geschrieben hat.
fn cache_leeren() -> io::Result<()> {
    let ausgabe = Command::new(PURGE).output().map_err(|fehler| {
        io::Error::other(format!(
            "{PURGE} laesst sich nicht aufrufen ({fehler}). \
             Ohne geleerten Cache gibt es keine kalte Messung."
        ))
    })?;

    let meldung = String::from_utf8_lossy(&ausgabe.stderr);
    let meldung = meldung.trim();
    if !ausgabe.status.success() || !meldung.is_empty() {
        return Err(io::Error::other(format!(
            "{PURGE} ist gescheitert ({}){}. \
             Der Aufruf braucht Rechte, die dieser Lauf nicht hat; \
             wiederhole ihn mit `sudo`. Es wird keine Zahl ausgegeben.",
            ausgabe.status,
            if meldung.is_empty() {
                String::new()
            } else {
                format!(": {meldung}")
            }
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Auswertung
// ---------------------------------------------------------------------------

/// Das Perzentil nach dem Verfahren des naechsten Rangs.
///
/// Der Wert an der Stelle `ceil(anteil * n)`, eins-basiert gezaehlt. Bei zwanzig
/// Laeufen und dem Anteil 0,95 ist das der neunzehnte Wert der sortierten
/// Reihe: hoechstens ein Lauf darf darueber liegen. Nicht interpoliert, weil
/// eine Zusage gegen einen wirklich gemessenen Lauf abgenommen werden soll und
/// nicht gegen einen gerechneten Zwischenwert.
pub fn perzentil(werte: &[Duration], anteil: f64) -> Duration {
    if werte.is_empty() {
        return Duration::ZERO;
    }
    let mut sortiert = werte.to_vec();
    sortiert.sort_unstable();
    let rang = (anteil * sortiert.len() as f64).ceil().max(1.0) as usize;
    sortiert[rang.min(sortiert.len()) - 1]
}

/// Der Median. Bei gerader Anzahl das Mittel der beiden mittleren Werte.
pub fn median(werte: &[Duration]) -> Duration {
    if werte.is_empty() {
        return Duration::ZERO;
    }
    let mut sortiert = werte.to_vec();
    sortiert.sort_unstable();
    let mitte = sortiert.len() / 2;
    if sortiert.len() % 2 == 1 {
        sortiert[mitte]
    } else {
        (sortiert[mitte - 1] + sortiert[mitte]) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static ZAEHLER: AtomicU64 = AtomicU64::new(0);

    struct Wegwerfordner {
        pfad: PathBuf,
    }

    impl Wegwerfordner {
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let mut pfad = std::env::temp_dir();
            pfad.push(format!(
                "krk-bench-messen-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&pfad);
            Self { pfad }
        }

        fn pfad(&self) -> &Path {
            &self.pfad
        }
    }

    impl Drop for Wegwerfordner {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.pfad);
            if let Ok(steckbrief) = fixture::steckbriefpfad(&self.pfad) {
                let _ = fs::remove_file(steckbrief);
            }
        }
    }

    fn ms(zahl: u64) -> Duration {
        Duration::from_millis(zahl)
    }

    #[test]
    fn das_perzentil_nimmt_den_naechsten_rang() {
        // Zwanzig Werte, 1 bis 20 ms. ceil(0,95 * 20) = 19, also der
        // neunzehnte der sortierten Reihe.
        let werte: Vec<Duration> = (1..=20).map(ms).collect();
        assert_eq!(perzentil(&werte, 0.95), ms(19));

        // Die Reihenfolge der Eingabe darf nichts aendern.
        let mut gemischt = werte.clone();
        gemischt.reverse();
        assert_eq!(perzentil(&gemischt, 0.95), ms(19));
    }

    #[test]
    fn ein_einziger_ausreisser_faellt_nicht_unter_den_tisch() {
        // Neunzehn schnelle Laeufe und einer, der aus der Reihe faellt: das
        // 95. Perzentil zeigt den Ausreisser noch nicht, der Unterschied zum
        // Median macht ihn aber sichtbar.
        let mut werte: Vec<Duration> = vec![ms(10); 19];
        werte.push(ms(900));
        assert_eq!(perzentil(&werte, 0.95), ms(10));
        assert_eq!(median(&werte), ms(10));

        // Zwei Ausreisser dagegen schlagen durch.
        werte[0] = ms(800);
        assert_eq!(perzentil(&werte, 0.95), ms(800));
    }

    #[test]
    fn das_perzentil_haelt_die_raender_aus() {
        assert_eq!(perzentil(&[], 0.95), Duration::ZERO);
        assert_eq!(perzentil(&[ms(5)], 0.95), ms(5));
        assert_eq!(perzentil(&[ms(5), ms(7)], 0.95), ms(7));
        assert_eq!(
            perzentil(&(1..=20).map(ms).collect::<Vec<_>>(), 1.0),
            ms(20)
        );
    }

    #[test]
    fn der_median_mittelt_bei_gerader_anzahl() {
        assert_eq!(median(&[ms(1), ms(2), ms(3)]), ms(2));
        assert_eq!(median(&[ms(10), ms(20), ms(30), ms(40)]), ms(25));
        assert_eq!(median(&[]), Duration::ZERO);
    }

    #[test]
    fn eine_messreihe_liefert_je_messgroesse_einen_wert_pro_lauf() {
        let ordner = Wegwerfordner::neu("kurze-reihe");
        fixture::erzeugen(ordner.pfad(), 3_000, 1).expect("Erzeugen gescheitert");

        let reihe = Messreihe::fahren(ordner.pfad(), Cache::Warm, 3).expect("Messen gescheitert");

        assert_eq!(reihe.wiederholungen, 3);
        assert_eq!(reihe.eintraege, 3_000, "die Reihe hat nicht alles gelesen");
        assert_eq!(reihe.groessen.len(), 2);
        for groesse in &reihe.groessen {
            assert_eq!(
                groesse.werte.len(),
                3,
                "{} hat zu wenige Werte",
                groesse.name
            );
            assert!(
                groesse.werte.iter().all(|wert| *wert > Duration::ZERO),
                "{} hat einen Wert von null",
                groesse.name
            );
            assert!(groesse.minimum() <= groesse.median());
            assert!(groesse.median() <= groesse.perzentil95());
        }

        // Der erste Stapel muss vor dem vollstaendigen Lesen dastehen. Waere es
        // umgekehrt, wuerde die Strecke zweimal dasselbe messen.
        for lauf in 0..3 {
            assert!(
                reihe.groessen[0].werte[lauf] <= reihe.groessen[1].werte[lauf],
                "der erste Stapel kam nach dem vollstaendigen Lesen"
            );
        }
    }

    #[test]
    fn eine_reihe_ohne_laeufe_wird_abgelehnt() {
        let ordner = Wegwerfordner::neu("null-laeufe");
        fixture::erzeugen(ordner.pfad(), 10, 1).expect("Erzeugen gescheitert");
        let fehler = Messreihe::fahren(ordner.pfad(), Cache::Warm, 0)
            .expect_err("das haette scheitern muessen");
        assert_eq!(fehler.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn ein_pfad_ohne_verzeichnis_liefert_keine_zahl() {
        let ordner = Wegwerfordner::neu("gibt-es-nicht");
        let fehler = Messreihe::fahren(ordner.pfad(), Cache::Warm, 1)
            .expect_err("das haette scheitern muessen");
        assert!(
            fehler.to_string().contains("keinen einzigen Stapel")
                || fehler.to_string().contains("vollstaendig"),
            "unerwartete Meldung: {fehler}"
        );
    }
}
