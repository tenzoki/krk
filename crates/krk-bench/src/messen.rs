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

use std::fmt::Write as _;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use krk_core::verzeichnis::leser::{Lesevorgang, Meldung};
use krk_core::verzeichnis::modell::Ordnermodell;

use crate::bericht;
use crate::fixture;

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

// ---------------------------------------------------------------------------
// Die Fruehmessung am Durchstich (Schritt 8)
// ---------------------------------------------------------------------------

/// Das Praefix, mit dem die Anwendung ihre Messzeilen kennzeichnet.
///
/// Dieselbe Zeichenkette steht in `crates/krk-ui/src/messmodus.rs`. Sie hier
/// noch einmal zu schreiben statt sie zu teilen ist Absicht: die beiden Kisten
/// haengen ueber einen Prozessaufruf zusammen und nicht ueber eine
/// Abhaengigkeit, und `krk-bench` von `krk-ui` abhaengig zu machen hiesse, das
/// Messwerkzeug an die Oberflaeche zu binden, die es messen soll.
const PRAEFIX: &str = "krk-messung";

/// Wie lange ein einzelner Start hoechstens bis zur Meldung brauchen darf.
const FRIST_START: Duration = Duration::from_secs(60);

/// Wie lange der Lauf der vier Spannen hoechstens dauern darf.
///
/// Zwanzig Wiederholungen auf dem Ordner mit 100.000 Eintraegen sind der
/// laengste Teil; bei den im Schritt 6 gemessenen 690 ms je Lauf sind das rund
/// 14 s. Fuenf Minuten lassen dafuer reichlich Luft und fangen trotzdem einen
/// haengenden Lauf ab.
const FRIST_SPANNEN: Duration = Duration::from_secs(300);

/// Eine Zusage aus C8 mit ihren gemessenen Werten.
///
/// **Warum die Werte nach Runden getrennt bleiben.** Eine Runde ist genau die
/// Messung, die C8 vorschreibt: zwanzig Wiederholungen, das 95. Perzentil
/// darueber. Ob eine Zusage haelt, entscheidet sich an diesem einen Wert — und
/// wenn er von Runde zu Runde ueber die Zusage hin und her wandert, dann ist
/// die Zusage eben nicht gehalten, und ein Bericht ueber eine einzelne Runde
/// wuerde das verdecken. Die Runden zusammenzuwerfen und ein Perzentil ueber
/// alles zu rechnen waere derselbe Fehler von der anderen Seite: es waere nicht
/// mehr die Messung, die C8 nennt.
#[derive(Debug, Clone)]
pub struct Zusage {
    /// Die Kennung aus C8, etwa `L1`.
    pub kennung: &'static str,
    /// Was gemessen wurde, in einem Satzteil.
    pub was: &'static str,
    /// Die zugesagte Obergrenze. `None` bei einer Zahl, die der Bericht nennt,
    /// ohne dass das Gate von Schritt 8 sie abfragt.
    pub schwelle: Option<Duration>,
    /// Die Einzelwerte, Runde fuer Runde.
    pub runden: Vec<Vec<Duration>>,
}

impl Zusage {
    /// Das 95. Perzentil je Runde, gegen das C8 abnimmt.
    pub fn perzentile(&self) -> Vec<Duration> {
        self.runden
            .iter()
            .map(|werte| perzentil(werte, PERZENTIL))
            .collect()
    }

    /// Das beste 95. Perzentil ueber alle Runden.
    pub fn bestes_perzentil(&self) -> Duration {
        self.perzentile().into_iter().min().unwrap_or_default()
    }

    /// Das schlechteste 95. Perzentil ueber alle Runden.
    pub fn schlechtestes_perzentil(&self) -> Duration {
        self.perzentile().into_iter().max().unwrap_or_default()
    }

    /// Der Median ueber alle Einzelwerte aller Runden.
    pub fn median(&self) -> Duration {
        median(&self.alle_werte())
    }

    /// Der kleinste gemessene Einzelwert.
    ///
    /// Bei einer Spanne, die an einer Bildgrenze endet, ist das der Lauf, dessen
    /// Bildgrenze am dichtesten hinter der fertigen Arbeit lag. Er kommt dem,
    /// was die Anwendung selbst kostet, am naechsten.
    pub fn minimum(&self) -> Duration {
        self.alle_werte().into_iter().min().unwrap_or_default()
    }

    /// Der groesste gemessene Einzelwert.
    pub fn maximum(&self) -> Duration {
        self.alle_werte().into_iter().max().unwrap_or_default()
    }

    /// In wie vielen Runden die Zusage gehalten hat, und wie viele es waren.
    pub fn gehalten_in(&self) -> Option<(usize, usize)> {
        let grenze = self.schwelle?;
        let perzentile = self.perzentile();
        let gehalten = perzentile.iter().filter(|wert| **wert <= grenze).count();
        Some((gehalten, perzentile.len()))
    }

    /// Ob die Zusage in **jeder** Runde gehalten hat.
    ///
    /// Eine Zusage, die in vier von acht Runden haelt, ist nicht gehalten. Das
    /// ist der strengere und der einzig brauchbare Massstab: der Nutzer merkt
    /// nicht, in welcher Runde er sitzt.
    pub fn immer_gehalten(&self) -> Option<bool> {
        self.gehalten_in()
            .map(|(gehalten, runden)| gehalten == runden)
    }

    fn alle_werte(&self) -> Vec<Duration> {
        self.runden.iter().flatten().copied().collect()
    }
}

/// Was der Durchstich messen soll.
#[derive(Debug, Clone)]
pub struct Durchstich {
    /// Das Binaerprogramm im Buendel, `KRK.app/Contents/MacOS/krk`.
    ///
    /// Aufgerufen wird es unmittelbar und nicht ueber `open`: ein ueber `open`
    /// gestartetes Buendel hat keine Standardausgabe, und ohne die kaeme keine
    /// Zahl zurueck. Defekt
    /// `issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`.
    pub programm: PathBuf,
    /// Pruefordner A mit 10.000 Eintraegen.
    pub ordner_a: PathBuf,
    /// Der Pruefordner mit 100.000 Eintraegen.
    pub ordner100k: PathBuf,
    /// Wie oft jede Zusage innerhalb einer Runde gemessen wird. C8 sagt zwanzig.
    pub wiederholungen: usize,
    /// Wie oft die ganze Messung wiederholt wird.
    ///
    /// Eine Runde ist die Messung, die C8 beschreibt. Mehrere Runden
    /// beantworten die Frage daneben, die C8 nicht stellt und die eine Abnahme
    /// trotzdem braucht: ob dieselbe Messung morgen dasselbe Urteil faellt.
    pub runden: usize,
}

/// Die Rohwerte einer einzelnen Runde.
#[derive(Debug, Clone, Default)]
struct Rohrunde {
    l1: Vec<Duration>,
    l2: Vec<Duration>,
    l3: Vec<Duration>,
    l4: Vec<Duration>,
    l10_erste: Vec<Duration>,
    l10_voll: Vec<Duration>,
}

/// Was der Durchstich ergeben hat.
#[derive(Debug, Clone)]
pub struct Durchstichergebnis {
    /// Die Bildwiederholrate, wie die Anwendung sie aus `NSScreen` gelesen hat.
    pub bildwiederholrate: Option<i64>,
    /// Die gemessenen Zusagen, in der Reihenfolge des Berichts.
    pub zusagen: Vec<Zusage>,
}

impl Durchstichergebnis {
    /// Ob jede abgefragte Zusage ihre Zahl in jeder Runde haelt.
    pub fn bestanden(&self) -> bool {
        self.zusagen
            .iter()
            .all(|zusage| zusage.immer_gehalten() != Some(false))
    }
}

impl Durchstich {
    /// Faehrt alle Runden und setzt das Ergebnis zusammen.
    pub fn fahren(&self) -> io::Result<Durchstichergebnis> {
        let mut rohrunden = Vec::with_capacity(self.runden);
        let mut rate = None;
        for nummer in 1..=self.runden {
            eprintln!("krk-bench: Runde {nummer} von {}", self.runden);
            let (gemeldete_rate, runde) = self.eine_runde()?;
            rate = rate.or(gemeldete_rate);
            rohrunden.push(runde);
        }

        let sammeln = |waehlen: fn(&Rohrunde) -> &Vec<Duration>| -> Vec<Vec<Duration>> {
            rohrunden
                .iter()
                .map(|runde| waehlen(runde).clone())
                .collect()
        };

        Ok(Durchstichergebnis {
            bildwiederholrate: rate,
            zusagen: vec![
                Zusage {
                    kennung: "L1",
                    was: "Tastendruck bis Ende des Zeichendurchgangs",
                    schwelle: Some(Duration::from_millis(16)),
                    runden: sammeln(|runde| &runde.l1),
                },
                Zusage {
                    kennung: "L2",
                    was: "Pruefordner A: erste Bildschirmseite",
                    schwelle: Some(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l2),
                },
                Zusage {
                    kennung: "L3",
                    was: "Pruefordner A: vollstaendig gelesen und sortiert",
                    schwelle: Some(Duration::from_millis(400)),
                    runden: sammeln(|runde| &runde.l3),
                },
                Zusage {
                    kennung: "L4",
                    was: "Prozessstart bis bedienbares Fenster",
                    schwelle: Some(Duration::from_millis(1000)),
                    runden: sammeln(|runde| &runde.l4),
                },
                Zusage {
                    kennung: "L10",
                    was: "100.000 Eintraege: erste Bildschirmseite",
                    schwelle: Some(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l10_erste),
                },
                Zusage {
                    kennung: "L10b",
                    was: "100.000 Eintraege: vollstaendig gelesen (Beigabe)",
                    // C8 sagt hierfuer 4 s warm zu. Das Gate von Schritt 8
                    // fragt die Zahl nicht ab; der Bericht nennt sie, weil sie
                    // ohnehin anfaellt.
                    schwelle: None,
                    runden: sammeln(|runde| &runde.l10_voll),
                },
            ],
        })
    }

    /// Eine Runde: zwanzig Prozessstarts fuer L4, ein Lauf fuer die uebrigen.
    fn eine_runde(&self) -> io::Result<(Option<i64>, Rohrunde)> {
        let l4 = self.starts_messen()?;
        let (rate, spannen) = self.spannen_messen()?;

        let hole = |name: &str| -> io::Result<Vec<Duration>> {
            let werte = werte_lesen(&spannen, name);
            if werte.len() != self.wiederholungen {
                return Err(io::Error::other(format!(
                    "die Anwendung hat fuer {name} {} Werte geliefert, erwartet waren {}. \
                     Die Reihe wird verworfen.",
                    werte.len(),
                    self.wiederholungen
                )));
            }
            Ok(werte)
        };

        Ok((
            rate,
            Rohrunde {
                l1: hole("l1")?,
                l2: hole("l2")?,
                l3: hole("l3")?,
                l4,
                l10_erste: hole("l10-erste")?,
                l10_voll: hole("l10-voll")?,
            },
        ))
    }

    /// L4: je Wiederholung ein Prozessstart.
    ///
    /// Die Spanne beginnt unmittelbar vor `spawn` und endet an dem Zeitpunkt,
    /// den die Anwendung meldet, sobald ihre Oberflaeche bedienbar ist. Beide
    /// Zeitpunkte kommen von derselben Uhr desselben Geraets.
    ///
    /// **Was der Anfang einschliesst.** Er liegt einen Wimpernschlag vor dem
    /// eigentlichen Prozessstart, weil `fork` und `exec` noch hineinfallen. Die
    /// Zahl ist damit eher zu gross als zu klein, und das ist die richtige
    /// Richtung fuer eine Abnahme.
    fn starts_messen(&self) -> io::Result<Vec<Duration>> {
        let ordner = self.ordner_a.display().to_string();
        let mut werte = Vec::with_capacity(self.wiederholungen);
        for nummer in 1..=self.wiederholungen {
            let vorher = SystemTime::now();
            let ausgang = lauf_fahren(
                &self.programm,
                &["--messmodus", "start", "--ordner", &ordner],
                FRIST_START,
            )?;
            let nanos = zahl_lesen(&ausgang.messzeilen, "bedienbar").ok_or_else(|| {
                io::Error::other(format!(
                    "Start {nummer} von {} hat keinen Zeitpunkt gemeldet.{}",
                    self.wiederholungen,
                    ausgang.klage()
                ))
            })?;
            let bedienbar = UNIX_EPOCH + Duration::from_nanos(nanos as u64);
            let spanne = bedienbar.duration_since(vorher).map_err(|_| {
                io::Error::other(format!(
                    "Start {nummer} meldet einen Zeitpunkt vor dem eigenen Start; \
                     die Uhr des Geraets ist waehrend der Messung gesprungen."
                ))
            })?;
            werte.push(spanne);
        }
        Ok(werte)
    }

    /// L1, L2, L3 und L10 in einem einzigen Prozess.
    fn spannen_messen(&self) -> io::Result<(Option<i64>, Vec<String>)> {
        let a = self.ordner_a.display().to_string();
        let gross = self.ordner100k.display().to_string();
        let ausgang = lauf_fahren(
            &self.programm,
            &[
                "--messmodus",
                "spannen",
                "--ordner-a",
                &a,
                "--ordner100k",
                &gross,
            ],
            FRIST_SPANNEN,
        )?;
        if !ausgang.messzeilen.iter().any(|zeile| zeile == "fertig") {
            return Err(io::Error::other(format!(
                "der Messlauf der Anwendung ist nicht bis zum Ende gekommen.{}",
                ausgang.klage()
            )));
        }
        let rate = zahl_lesen(&ausgang.messzeilen, "bildwiederholrate").map(|zahl| zahl as i64);
        if rate.is_none() {
            return Err(io::Error::other(
                "die Anwendung hat keine Bildwiederholrate gemeldet. \
                 Ohne sie ist L1 nicht gegen seine Herleitung pruefbar; \
                 es wird keine Zahl ausgegeben."
                    .to_owned(),
            ));
        }
        Ok((rate, ausgang.messzeilen))
    }
}

/// Was ein Lauf der Anwendung hinterlassen hat.
struct Ausgang {
    /// Die Zeilen der Standardausgabe, jeweils ohne das Praefix.
    messzeilen: Vec<String>,
    /// Die Fehlerausgabe, unveraendert.
    fehlerausgabe: String,
    /// Wie der Prozess geendet ist; `None`, wenn die Frist ihn beendet hat.
    ausgangswert: Option<ExitStatus>,
}

impl Ausgang {
    /// Ein Satzteil, der den Fehlschlag erklaert, fuer eine Fehlermeldung.
    fn klage(&self) -> String {
        let ende = match self.ausgangswert {
            Some(wert) => format!(" Der Prozess endete mit {wert}."),
            None => " Der Prozess hat die Frist ueberschritten und wurde beendet.".to_owned(),
        };
        let fehler = self.fehlerausgabe.trim();
        if fehler.is_empty() {
            ende
        } else {
            format!("{ende} Fehlerausgabe: {fehler}")
        }
    }
}

/// Startet die Anwendung, sammelt ihre Ausgabe und beendet sie nach der Frist.
fn lauf_fahren(programm: &Path, argumente: &[&str], frist: Duration) -> io::Result<Ausgang> {
    let mut kind = Command::new(programm)
        .args(argumente)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|fehler| {
            io::Error::other(format!(
                "{} laesst sich nicht starten ({fehler}). \
                 Baue das Buendel zuerst mit `cargo xtask bundle`.",
                programm.display()
            ))
        })?;

    let ausgabe = kind.stdout.take().expect("stdout wurde als Rohr angelegt");
    let fehlerrohr = kind.stderr.take().expect("stderr wurde als Rohr angelegt");
    let sammler = thread::spawn(move || {
        BufReader::new(ausgabe)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
    });
    let fehlersammler = thread::spawn(move || {
        let mut text = String::new();
        let _ = BufReader::new(fehlerrohr).read_to_string(&mut text);
        text
    });

    let ausgangswert = warten_bis(&mut kind, frist)?;
    let zeilen = sammler.join().unwrap_or_default();
    let fehlerausgabe = fehlersammler.join().unwrap_or_default();

    Ok(Ausgang {
        messzeilen: zeilen
            .iter()
            .filter_map(|zeile| zeile.strip_prefix(PRAEFIX))
            .map(|rest| rest.trim().to_owned())
            .collect(),
        fehlerausgabe,
        ausgangswert,
    })
}

/// Wartet auf das Ende des Prozesses, hoechstens aber die genannte Frist.
fn warten_bis(kind: &mut Child, frist: Duration) -> io::Result<Option<ExitStatus>> {
    let ende = Instant::now() + frist;
    loop {
        if let Some(wert) = kind.try_wait()? {
            return Ok(Some(wert));
        }
        if Instant::now() >= ende {
            kind.kill()?;
            kind.wait()?;
            return Ok(None);
        }
        thread::sleep(Duration::from_millis(5));
    }
}

/// Liest die Zahl hinter einem Schluesselwort, etwa `bedienbar 1785…`.
fn zahl_lesen(zeilen: &[String], schluessel: &str) -> Option<u128> {
    zeilen
        .iter()
        .find_map(|zeile| zeile.strip_prefix(schluessel)?.trim().parse().ok())
}

/// Liest alle Einzelwerte einer Messgroesse, etwa `wert l1 16123456`.
fn werte_lesen(zeilen: &[String], name: &str) -> Vec<Duration> {
    let anfang = format!("wert {name} ");
    zeilen
        .iter()
        .filter_map(|zeile| zeile.strip_prefix(&anfang))
        .filter_map(|nanos| nanos.trim().parse::<u64>().ok())
        .map(Duration::from_nanos)
        .collect()
}

/// Setzt den Bericht der Fruehmessung zusammen.
pub fn durchstich_bericht(lauf: &Durchstich, ergebnis: &Durchstichergebnis) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "KRK — Messbericht der Fruehmessung (Schritt 8)");
    let _ = writeln!(text, "==============================================");
    let _ = writeln!(text);

    let _ = writeln!(text, "Bedingungen");
    let _ = writeln!(text, "-----------");
    let mut zeile = |name: &str, wert: &str| {
        let _ = writeln!(text, "{name:<22}{wert}");
    };
    zeile("Zeitpunkt", &bericht::zeitstempel(SystemTime::now()));
    zeile(
        "hw.model",
        &bericht::befehl_ausgabe("/usr/sbin/sysctl", &["-n", "hw.model"]),
    );
    zeile("sw_vers", &bericht::betriebssystem());
    zeile("Bildwiederholrate", &rate_beschreiben(ergebnis));
    zeile("Cache-Zustand", CACHE);
    zeile(
        "Wiederholungen",
        &format!(
            "{} je Zusage und Runde, {} Runden",
            lauf.wiederholungen, lauf.runden
        ),
    );
    zeile("Pruefordner A", &ordner_beschreiben(&lauf.ordner_a));
    zeile("Pruefordner 100k", &ordner_beschreiben(&lauf.ordner100k));
    zeile("Gemessenes Buendel", &lauf.programm.display().to_string());
    zeile("Sitzungslage L4", SITZUNGSLAGE);
    zeile(
        "Werkzeug",
        &format!(
            "krk-bench {}, Ziel {}-{}, {}",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::ARCH,
            std::env::consts::OS,
            bericht::bauart()
        ),
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Zahlen");
    let _ = writeln!(text, "------");
    let _ = writeln!(
        text,
        "Das 95. Perzentil steht je Runde einmal. Ausgewiesen sind das beste und"
    );
    let _ = writeln!(
        text,
        "das schlechteste der {} Runden; Median, Minimum und Maximum laufen ueber",
        lauf.runden
    );
    let _ = writeln!(text, "alle Einzelwerte aller Runden.");
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "{:<56}{:>13}{:>13}{:>12}{:>12}{:>12}{:>10}   Urteil",
        "Gemessene Groesse", "p95 bestes", "p95 schlecht", "Median", "Minimum", "Maximum", "Zusage"
    );
    for zusage in &ergebnis.zusagen {
        let _ = writeln!(
            text,
            "{:<56}{:>13}{:>13}{:>12}{:>12}{:>12}{:>10}   {}",
            format!("{} — {}", zusage.kennung, zusage.was),
            bericht::spanne(zusage.bestes_perzentil()),
            bericht::spanne(zusage.schlechtestes_perzentil()),
            bericht::spanne(zusage.median()),
            bericht::spanne(zusage.minimum()),
            bericht::spanne(zusage.maximum()),
            match zusage.schwelle {
                Some(grenze) => format!("{} ms", grenze.as_millis()),
                None => "keine".to_owned(),
            },
            urteil(zusage)
        );
    }
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "Urteil des Gates: {}",
        if ergebnis.bestanden() {
            "bestanden — jede der fuenf abgefragten Zusagen haelt ihre Zahl in jeder Runde."
        } else {
            "NICHT bestanden — mindestens eine Zusage verfehlt ihre Zahl in mindestens einer Runde."
        }
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Das 95. Perzentil Runde fuer Runde");
    let _ = writeln!(text, "----------------------------------");
    for zusage in &ergebnis.zusagen {
        let werte: Vec<String> = zusage
            .perzentile()
            .into_iter()
            .map(bericht::spanne)
            .collect();
        let _ = writeln!(text, "{:<8}{}", zusage.kennung, werte.join("  "));
    }
    let _ = writeln!(text);

    let _ = writeln!(text, "Einzelwerte");
    let _ = writeln!(text, "-----------");
    for zusage in &ergebnis.zusagen {
        let _ = writeln!(text, "{} ({}):", zusage.kennung, zusage.was);
        for (nummer, runde) in zusage.runden.iter().enumerate() {
            let werte: Vec<String> = runde.iter().copied().map(bericht::spanne).collect();
            let _ = writeln!(text, "  Runde {}:", nummer + 1);
            for buendel in werte.chunks(5) {
                let _ = writeln!(text, "    {}", buendel.join("  "));
            }
        }
    }
    let _ = writeln!(text);

    text.push_str(EINSCHRAENKUNGEN);
    let _ = writeln!(text);
    text.push_str(LESART);
    text
}

/// Das Urteil zu einer Zusage, in Worten.
fn urteil(zusage: &Zusage) -> String {
    match zusage.gehalten_in() {
        None => "nicht abgefragt".to_owned(),
        Some((gehalten, runden)) if gehalten == runden => {
            format!("gehalten in allen {runden} Runden")
        }
        Some((gehalten, runden)) => format!("VERFEHLT, gehalten in {gehalten} von {runden} Runden"),
    }
}

/// Schreibt den Bericht in den Messungenordner und liefert seinen Pfad.
pub fn durchstich_schreiben(ziel: &Path, text: &str) -> io::Result<PathBuf> {
    if !ziel.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} gibt es nicht. Rufe das Werkzeug aus dem Projektwurzelverzeichnis \
                 auf oder nenne den Ordner ueber --ziel.",
                ziel.display()
            ),
        ));
    }
    let pfad = ziel.join(format!(
        "{}-durchstich.txt",
        bericht::kurzstempel(SystemTime::now())
    ));
    std::fs::write(&pfad, text)?;
    Ok(pfad)
}

/// Wie der Cache-Zustand im Kopf beschrieben wird.
const CACHE: &str = "warm (siehe Abschnitt Einschraenkungen: purge braucht \
Rechte, die dieser Lauf nicht hat)";

/// Unter welcher Sitzungslage L4 gemessen ist.
const SITZUNGSLAGE: &str = "ein Fenster, ein Dateifenster, keine wiederhergestellte Sitzung, \
Startordner ist Pruefordner A";

/// Was diese Messung nicht leisten kann, ausgeschrieben.
const EINSCHRAENKUNGEN: &str = "\
Einschraenkungen dieser Messung
-------------------------------
**L4 ist warm gemessen, C8 sagt Kaltstart.** \"Kalt\" heisst laut C8: erster
Zugriff nach dem Leeren des Dateisystem-Caches, und geleert wird er unter macOS
allein von `purge`, das Rechte braucht, die dieser Lauf nicht hat. Ein
Passwortdialog laesst sich in einem Messlauf nicht beantworten. Der Bericht
weist L4 deshalb als warm aus, statt eine warme Zahl unter die Ueberschrift
\"kalt\" zu setzen. Warm ist der leichtere Fall: die Zahl unten ist eine
Untergrenze fuer die Zusage, die C8 wirklich stellt. L2, L3 und L10 sind von der
Luecke nicht betroffen, weil C8 sie ohnehin warm zusagt.

**Die Sitzungslage aus C8 ist am Durchstich nicht herstellbar.** C8 schreibt fuer
L4 eine Pruefsitzung aus zwei Dateifenstern mit je zwei Tabs vor. Tabs gibt es
in KRK erst mit Schritt 12. Gemessen ist deshalb der Start des Buendels mit
einem Fenster und ohne wiederhergestellte Sitzung, auf Pruefordner A. Die
Abnahme gegen die Pruefsitzung leistet Schritt 22. Die gemessene Zahl ist damit
guenstiger als die spaetere: zwei sichtbare Tabs kosten zwei erste
Bildschirmseiten statt einer.

**Der koerperliche Tastendruck bleibt ungemessen.** L1 wird mit einem
synthetischen Ereignis ausgeloest; naeheres im Abschnitt Lesart.

**Der Bildtakt kann stehenbleiben.** Ein `CADisplayLink` taktet nur, solange das
Fenster sichtbar ist. Bleibt er stehen, bricht der Messlauf nach zehn Sekunden
mit einer Meldung ab und gibt keine Zahl aus; die Meldung nennt die Zahl der
seither eingegangenen Bildgrenzen und trennt damit ein stehendes Bild von einer
langsamen Oberflaeche.
";

fn rate_beschreiben(ergebnis: &Durchstichergebnis) -> String {
    match ergebnis.bildwiederholrate {
        Some(hertz) => format!(
            "{hertz} Hz, gelesen aus NSScreen.maximumFramesPerSecond \
             am Bildschirm des gemessenen Fensters"
        ),
        None => "nicht erhoben".to_owned(),
    }
}

/// Pfad, Startwert und Eintragszahl eines Pruefordners.
fn ordner_beschreiben(ordner: &Path) -> String {
    match fixture::steckbrief_lesen(ordner) {
        Some(brief) => format!(
            "{} (Startwert {}, {} Eintraege laut Steckbrief)",
            ordner.display(),
            brief.startwert,
            brief.eintraege
        ),
        None => format!(
            "{} (kein Steckbrief daneben; Startwert unbekannt)",
            ordner.display()
        ),
    }
}

/// Was der Bericht ueber sich selbst sagen muss, damit seine Zahlen lesbar sind.
const LESART: &str = "\
Lesart
------
Jede Spanne endet an einer Bildgrenze. Die Anwendung haengt dafuer einen
CADisplayLink an die Ansicht des Dateifensters; sein Rueckruf kommt einmal je
Bildwiederholung. Gemessen ist damit die Spanne bis zur ersten Bildgrenze, an
der die Aenderung im Modell steht — nicht der Zeitpunkt, an dem ein Pixel auf
dem Schirm ist. Aus dem eigenen Prozess heraus ist dieser Zeitpunkt nicht
feststellbar; der Plan nennt die Bildgrenze ausdruecklich als die erreichbare
Naeherung, statt eine Photonenmessung zu behaupten. Jede Zahl unten ist deshalb
auf ein Bild genau, bei 60 Hz also auf 16,7 ms.

L1 wird mit einem synthetischen Tastenereignis ausgeloest. Die Anwendung baut
ein NSEvent vom Typ keyDown mit dem Tastencode 125 (Pfeil ab) und stellt es
ueber NSApplication.postEvent:atStart: hinten in die eigene Ereignisschlange.
Von dort geht es denselben Weg wie ein koerperlicher Druck: durch den lokalen
Ereignisabgriff, die Normalisierung der Zusatztasten und den Nachschlag im Kern
bis in die Datenquelle des Dateifensters. Dass eine koerperlich gedrueckte
Taste dieselben Ereignisse erzeugt, ist damit nicht gemessen.

Der Ausloeser jeder Messung haengt an einem Zeitgeber mit 97 ms und nicht an
der Bildgrenze. Loeste die Bildgrenze selbst aus, laege zwischen Ausloeser und
naechster Bildgrenze immer genau ein volles Bild, und L1 haette bei 60 Hz
konstant 16,7 ms — nicht gemessen, sondern gebaut. 97 ms sind bei 60 Hz 5,82
Bilder; ueber zwanzig Wiederholungen wandert der Ausloesezeitpunkt damit durch
das Bild.

L4 misst der aeussere Aufrufer, nicht die Anwendung: er nimmt die Uhrzeit
unmittelbar vor dem Start des Prozesses, und die Anwendung meldet die Uhrzeit,
zu der ihre Oberflaeche bedienbar ist. Bedienbar heisst nach der
Nutzerentscheidung vom 260802-1735: Fenster steht, Tastenabgriff eingerichtet,
erste Bildschirmseite des Startordners sichtbar. Das vollstaendige Lesen laeuft
danach weiter und faellt unter L3.

Das 95. Perzentil ist der Wert des naechsten Rangs, nicht interpoliert: bei
zwanzig Laeufen der neunzehnte der sortierten Reihe.

Eine Runde ist genau die Messung, die C8 vorschreibt: zwanzig Wiederholungen je
Zusage, das 95. Perzentil darueber. Der Bericht faehrt mehrere Runden, weil ein
Urteil, das von Runde zu Runde wechselt, kein Urteil ist. Eine Zusage gilt hier
nur dann als gehalten, wenn sie es in jeder Runde tut.
";

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
