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
//!
//! # Zwei Abnahmemasse
//!
//! Die Auswertung der Fruehmessung weiter unten kennt seit dem 260803-1810 zwei
//! Masse, und welches gilt, haengt an der Art der Zusage: eine zugesagte Dauer
//! wird ueber das Perzentil abgenommen, eine zugesagte Bildgrenze ueber den
//! Anteil der Eingaben, die ihr naechstes Bild erreichen. Seit dem 260807-0832
//! fordern L1 und L9 dabei **verschiedene** Anteile, und L9 traegt daneben eine
//! Obergrenze je Einzelwert. Jede Zusage traegt ihr Mass deshalb vollstaendig
//! selbst; siehe [`Abnahmemass`].

use std::fmt::Write as _;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use krk_core::verzeichnis::leser::{Lesevorgang, Meldung};
use krk_core::verzeichnis::modell::Ordnermodell;

use crate::bericht;
use crate::fixture;

/// Wie oft jede Messung wiederholt wird. C8 schreibt zwanzig vor.
pub const WIEDERHOLUNGEN: usize = 20;

/// Der Anteil, fuer den die acht Zusagen aus C8 gelten, die eine Dauer zusagen.
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

/// Wie eine Zusage aus C8 abgenommen wird.
///
/// **Zwei Masse, und welches gilt, haengt an der Art der Zusage.** Acht der
/// zehn Zusagen sagen eine **Dauer** zu, die der Nutzer abwartet: ein Ordner
/// ist gelesen, ein Fenster steht. Dort ist das 95. Perzentil der gemessenen
/// Spanne das richtige Mass, weil die Spanne selbst das Erlebnis ist. L1 und L9
/// sagen etwas anderes zu, naemlich dass die Reaktion **im naechsten Bild**
/// erscheint. Oberhalb dieser Grenze kann die Maschine nicht besser werden,
/// unterhalb kann der Mensch nicht unterscheiden; das Perzentil einer solchen
/// Spanne misst, an welcher Stelle des Bildes der Tastendruck eintraf, und
/// nicht, wie schnell KRK ist.
///
/// Der Nutzer hat das am 260803-1810 entschieden. Herleitung im Spec unter C8,
/// Absaetze ab `Warum L1 und L9 den Anteil zaehlen und nicht die Spanne`, und
/// im Datensatz
/// `decisions/260803-1755_*_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`.
///
/// **L1 und L9 teilen ihre Schwelle seit dem 260807-0832 nicht mehr.** Bis dahin
/// forderten beide 95 Prozent im ersten Bild, und die Zahl stand als Konstante
/// neben dieser Aufzaehlung. Der Nutzer hat L9 an jenem Tag auf 85 Prozent
/// gesenkt und ihm dafuer eine zweite Haelfte gegeben: keine einzige Eingabe
/// liegt ueber zwei Bildlaengen. Damit gehen die beiden Werte auseinander, und
/// eine gemeinsame Konstante waere ab da schlicht falsch. Datensatz
/// `decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abnahmemass {
    /// Das 95. Perzentil der Runde liegt hoechstens bei dieser Grenze.
    Perzentil(Duration),
    /// Mindestens `mindestanteil_prozent` der Einzelwerte einer Runde liegen
    /// hoechstens bei einer Bildlaenge, und keiner liegt ueber
    /// `obergrenze_bilder` Bildlaengen.
    ///
    /// Alle drei Angaben stehen hier und nicht beim Aufrufer, damit eine Zusage
    /// ihr Mass vollstaendig traegt: [`Zusage::gehalten_in`] braucht dann kein
    /// zweites Argument, das bei acht der zehn Zusagen ohnehin unbenutzt bliebe.
    AnteilImBild {
        /// Der Kehrwert der Bildwiederholrate. Bis hierher gilt eine Eingabe
        /// als im naechsten Bild erschienen.
        bildlaenge: Duration,
        /// Wie viel Prozent der Eingaben ihr naechstes Bild erreichen muessen.
        ///
        /// Als ganze Zahl gefuehrt, damit das Urteil ohne Fliesskommavergleich
        /// feststeht: 17 von 20 sind genau 85 Prozent und halten.
        mindestanteil_prozent: usize,
        /// Wie viele Bildlaengen ein **einzelner** Wert hoechstens betragen darf.
        ///
        /// `None` heisst: die Zusage kennt keine Obergrenze je Einzelwert, es
        /// zaehlt allein der Anteil. So ist L1 gefasst. L9 traegt seit dem
        /// 260807-0832 `Some(2)`.
        obergrenze_bilder: Option<u32>,
    },
    /// Der Bericht nennt die Zahl, das Gate fragt sie nicht ab.
    Keine,
}

impl Abnahmemass {
    /// Das Mass in Worten, wie der Bericht es je Zeile ausweist.
    ///
    /// Damit steht in jeder Zeile der Zahlentabelle, nach welcher Regel sie
    /// beurteilt ist, statt dass der Leser es aus der Kennung erschliessen muss.
    /// Eine Zusage mit Obergrenze nennt beide Haelften; nur die eine zu nennen
    /// hiesse, das Urteil auf halber Grundlage auszuweisen.
    pub fn beschreibung(self) -> String {
        match self {
            Self::Perzentil(grenze) => format!("p95 <= {} ms", grenze.as_millis()),
            Self::AnteilImBild {
                mindestanteil_prozent,
                obergrenze_bilder: None,
                ..
            } => format!(">= {mindestanteil_prozent} % im Bild"),
            Self::AnteilImBild {
                mindestanteil_prozent,
                obergrenze_bilder: Some(bilder),
                ..
            } => format!(">= {mindestanteil_prozent} %, <= {bilder} Bilder"),
            Self::Keine => "keine".to_owned(),
        }
    }
}

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
    /// Nach welcher Regel diese Zusage abgenommen wird.
    pub mass: Abnahmemass,
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

    /// Je Runde: wie viele Eingaben ihr naechstes Bild erreicht haben, und wie
    /// viele es waren.
    ///
    /// `None`, wenn diese Zusage nicht ueber den Anteil abgenommen wird. Eine
    /// Eingabe erreicht ihr naechstes Bild, wenn die Spanne vom Zeitstempel des
    /// Tastenereignisses bis zum Ende des Zeichendurchgangs hoechstens eine
    /// Bildlaenge betraegt; ist sie groesser, wird die Aenderung erst mit dem
    /// uebernaechsten Bild sichtbar.
    pub fn im_bild(&self) -> Option<Vec<(usize, usize)>> {
        let Abnahmemass::AnteilImBild { bildlaenge, .. } = self.mass else {
            return None;
        };
        Some(
            self.runden
                .iter()
                .map(|werte| {
                    let erreicht = werte.iter().filter(|wert| **wert <= bildlaenge).count();
                    (erreicht, werte.len())
                })
                .collect(),
        )
    }

    /// Der Anteil erreichter Bilder je Runde, in Prozent.
    pub fn anteile_im_bild(&self) -> Option<Vec<f64>> {
        self.im_bild().map(|runden| {
            runden
                .into_iter()
                .map(|(erreicht, gesamt)| anteil_prozent(erreicht, gesamt))
                .collect()
        })
    }

    /// Der schlechteste Anteil ueber alle Runden, in Prozent.
    ///
    /// An dieser Zahl haengt das Urteil, denn gehalten heisst in jeder Runde
    /// gehalten.
    pub fn schlechtester_anteil(&self) -> Option<f64> {
        self.anteile_im_bild()
            .map(|anteile| anteile.into_iter().fold(f64::INFINITY, f64::min))
            .filter(|wert| wert.is_finite())
    }

    /// Je Runde: der groesste Einzelwert, ausgedrueckt in Bildlaengen.
    ///
    /// Die zweite Haelfte der Zusage L9 haengt an dieser Zahl: keine Eingabe
    /// darf ueber zwei Bildlaengen liegen. Sie steht auch dort, wo keine
    /// Obergrenze gilt, denn sie sagt dem Leser, wie weit die verpassten
    /// Eingaben ihr Bild verpasst haben. `None`, wenn diese Zusage nicht ueber
    /// den Anteil abgenommen wird. Eine Runde ohne Werte hat den Hoechstwert
    /// null.
    pub fn hoechstwerte_in_bildern(&self) -> Option<Vec<f64>> {
        let Abnahmemass::AnteilImBild { bildlaenge, .. } = self.mass else {
            return None;
        };
        Some(
            self.runden
                .iter()
                .map(|werte| {
                    in_bildern(werte.iter().copied().max().unwrap_or_default(), bildlaenge)
                })
                .collect(),
        )
    }

    /// Der groesste Einzelwert ueber alle Runden, in Bildlaengen.
    ///
    /// An dieser Zahl haengt das Urteil ueber die Obergrenze, denn gehalten
    /// heisst in jeder Runde gehalten.
    pub fn hoechstwert_in_bildern(&self) -> Option<f64> {
        self.hoechstwerte_in_bildern()
            .map(|runden| runden.into_iter().fold(0.0, f64::max))
    }

    /// In wie vielen Runden die Zusage gehalten hat, und wie viele es waren.
    ///
    /// `None`, wenn das Gate diese Zusage nicht abfragt.
    pub fn gehalten_in(&self) -> Option<(usize, usize)> {
        match self.mass {
            Abnahmemass::Perzentil(grenze) => {
                let perzentile = self.perzentile();
                let gehalten = perzentile.iter().filter(|wert| **wert <= grenze).count();
                Some((gehalten, perzentile.len()))
            }
            Abnahmemass::AnteilImBild {
                bildlaenge,
                mindestanteil_prozent,
                obergrenze_bilder,
            } => {
                // Beide Haelften muessen halten, und beide in derselben Runde:
                // der Anteil im ersten Bild und, wo die Zusage eine nennt, die
                // Obergrenze je Einzelwert.
                let obergrenze = obergrenze_bilder.map(|bilder| bildlaenge * bilder);
                let gehalten = self
                    .runden
                    .iter()
                    .filter(|werte| {
                        // Eine Runde ohne Werte haelt nicht.
                        if werte.is_empty() {
                            return false;
                        }
                        let erreicht = werte.iter().filter(|wert| **wert <= bildlaenge).count();
                        // Ganzzahlig verglichen, damit genau 17 von 20 als 85
                        // Prozent halten und das Urteil nicht an einer Rundung
                        // im letzten Bit haengt.
                        let anteil_haelt = erreicht * 100 >= werte.len() * mindestanteil_prozent;
                        let grenze_haelt = obergrenze
                            .is_none_or(|grenze| werte.iter().all(|wert| *wert <= grenze));
                        anteil_haelt && grenze_haelt
                    })
                    .count();
                Some((gehalten, self.runden.len()))
            }
            Abnahmemass::Keine => None,
        }
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

/// Der Anteil in Prozent. Eine Runde ohne Werte hat den Anteil null.
pub(crate) fn anteil_prozent(erreicht: usize, gesamt: usize) -> f64 {
    if gesamt == 0 {
        return 0.0;
    }
    100.0 * erreicht as f64 / gesamt as f64
}

/// Eine Spanne, ausgedrueckt in Bildlaengen.
///
/// Das ist die Einheit, in der die Obergrenze je Einzelwert zugesagt ist, und
/// deshalb die Einheit, in der der Bericht sie ausweist: "1.15 Bilder" laesst
/// sich gegen "hoechstens 2 Bilder" halten, "19.153 ms" erst nach einer
/// Kopfrechnung, die die Bildlaenge des Geraets kennt.
fn in_bildern(spanne: Duration, bildlaenge: Duration) -> f64 {
    if bildlaenge.is_zero() {
        return 0.0;
    }
    spanne.as_secs_f64() / bildlaenge.as_secs_f64()
}

/// Die Laenge eines Bildes, gebildet aus der gemeldeten Bildwiederholrate.
///
/// **Fehlt die Rate, bricht die Auswertung ab, statt 60 Hz zu unterstellen.**
/// Seit dem 260803-1810 ist die Rate nicht mehr nur eine Angabe im
/// Bedingungskopf, sondern Bestandteil des Urteils ueber L1: die Bildlaenge ist
/// ihr Kehrwert, und an ihr entscheidet sich je Einzelwert, ob eine Eingabe ihr
/// naechstes Bild erreicht hat. Dieselbe Haltung wie bei `--kalt` ohne Rechte
/// und bei einem Fenster ohne Bildschirm. Plan S8, Punkt 2 der Umstellung, und
/// `### Frage 5`.
fn bildlaenge_bilden(rate: Option<i64>) -> io::Result<(i64, Duration)> {
    match rate {
        Some(hertz) if hertz > 0 => Ok((hertz, Duration::from_secs_f64(1.0 / hertz as f64))),
        Some(hertz) => Err(io::Error::other(format!(
            "die Anwendung hat eine Bildwiederholrate von {hertz} Hz gemeldet. Daraus \
             laesst sich keine Bildlaenge bilden, und ohne Bildlaenge ist L1 nicht \
             abnehmbar."
        ))),
        None => Err(io::Error::other(
            "die Anwendung hat keine Bildwiederholrate gemeldet. L1 nimmt seit dem \
             260803-1810 ueber den Anteil der Eingaben ab, die ihr naechstes Bild \
             erreichen, und die Bildlaenge ist der Kehrwert dieser Rate. Die Auswertung \
             bricht deshalb ab, statt 60 Hz zu unterstellen.",
        )),
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
    /// `issues/260803-1309_*_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`.
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
    ///
    /// Keine Option: ohne die Rate gibt es kein Urteil ueber L1, und
    /// [`bildlaenge_bilden`] bricht dann ab, bevor ein Ergebnis entsteht.
    pub bildwiederholrate: i64,
    /// Eine Bildlaenge, der Kehrwert der Rate. Am Referenzgeraet 16,667 ms.
    ///
    /// Steht neben der Rate, weil der Bedingungskopf beide nennt und weil L1
    /// gegen diese Zahl abgenommen wird. Beide entstehen in [`Durchstich::fahren`]
    /// aus einem Aufruf von [`bildlaenge_bilden`].
    pub bildlaenge: Duration,
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

        let (bildwiederholrate, bildlaenge) = bildlaenge_bilden(rate)?;

        Ok(Durchstichergebnis {
            bildwiederholrate,
            bildlaenge,
            zusagen: vec![
                Zusage {
                    kennung: "L1",
                    was: "Tastendruck bis Ende des Zeichendurchgangs",
                    // Seit dem 260803-1810 nicht mehr 16 ms auf das Perzentil,
                    // sondern der Anteil der Eingaben, die ihr Bild erreichen.
                    // L1 fordert 95 Prozent und kennt keine Obergrenze je
                    // Einzelwert; das ist der Unterschied zu L9.
                    mass: Abnahmemass::AnteilImBild {
                        bildlaenge,
                        mindestanteil_prozent: 95,
                        obergrenze_bilder: None,
                    },
                    runden: sammeln(|runde| &runde.l1),
                },
                Zusage {
                    kennung: "L2",
                    was: "Pruefordner A: erste Bildschirmseite",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l2),
                },
                Zusage {
                    kennung: "L3",
                    was: "Pruefordner A: vollstaendig gelesen und sortiert",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(400)),
                    runden: sammeln(|runde| &runde.l3),
                },
                Zusage {
                    kennung: "L4",
                    was: "Prozessstart bis bedienbares Fenster",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(1000)),
                    runden: sammeln(|runde| &runde.l4),
                },
                Zusage {
                    kennung: "L10",
                    was: "100.000 Eintraege: erste Bildschirmseite",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l10_erste),
                },
                Zusage {
                    kennung: "L10b",
                    was: "100.000 Eintraege: vollstaendig gelesen (Beigabe)",
                    // C8 sagt hierfuer 4 s warm zu. Das Gate von Schritt 8
                    // fragt die Zahl nicht ab; der Bericht nennt sie, weil sie
                    // ohnehin anfaellt.
                    mass: Abnahmemass::Keine,
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
    fn starts_messen(&self) -> io::Result<Vec<Duration>> {
        let ordner = self.ordner_a.display().to_string();
        starts_messen(
            &self.programm,
            &["--messmodus", "start", "--ordner", &ordner],
            self.wiederholungen,
        )
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

/// Je Wiederholung ein Prozessstart, gemessen bis zur gemeldeten Bedienbarkeit.
///
/// Die Spanne beginnt unmittelbar vor `spawn` und endet an dem Zeitpunkt,
/// den die Anwendung meldet, sobald ihre Oberflaeche bedienbar ist. Beide
/// Zeitpunkte kommen von derselben Uhr desselben Geraets.
///
/// **Was der Anfang einschliesst.** Er liegt einen Wimpernschlag vor dem
/// eigentlichen Prozessstart, weil `fork` und `exec` noch hineinfallen. Die
/// Zahl ist damit eher zu gross als zu klein, und das ist die richtige
/// Richtung fuer eine Abnahme.
fn starts_messen(
    programm: &Path,
    argumente: &[&str],
    wiederholungen: usize,
) -> io::Result<Vec<Duration>> {
    let mut werte = Vec::with_capacity(wiederholungen);
    for nummer in 1..=wiederholungen {
        let vorher = SystemTime::now();
        let ausgang = lauf_fahren(programm, argumente, FRIST_START)?;
        let nanos = zahl_lesen(&ausgang.messzeilen, "bedienbar").ok_or_else(|| {
            io::Error::other(format!(
                "Start {nummer} von {wiederholungen} hat keinen Zeitpunkt gemeldet.{}",
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

// ---------------------------------------------------------------------------
// Der Gesamtlauf ueber alle zehn Zusagen (Schritt 21)
// ---------------------------------------------------------------------------

/// Wie lange der Sitzungslauf der Anwendung hoechstens dauern darf.
///
/// Der laengste Teil sind die zwanzig L8/L9-Wiederholungen: je eine begonnene
/// Kopie von 10.000 Eintraegen, ihr Abbruch, das Leeren des Kopierziels und
/// die Auffrischung. Zehn Minuten lassen dafuer reichlich Luft und fangen
/// trotzdem einen haengenden Lauf ab.
const FRIST_SITZUNG: Duration = Duration::from_secs(600);

/// Der Startwert des L6-Unterordners; die Startwerte 1 bis 3 tragen die drei
/// Pruefordner aus C8.
const STARTWERT_L6: u64 = 4;

/// Wie viele Eintraege der L6-Unterordner traegt: die Obergrenze der Zusage.
const EINTRAEGE_L6: usize = 1_000;

/// Der Gesamtlauf: alle zehn Zusagen aus C8 in einem Bericht.
///
/// Drei Strecken laufen zusammen, und die Zusammenfuehrung ist der Zweck des
/// Laufs: die Sitzungsstrecke der Anwendung (L1, L5, L6, L7, L8, L9), die
/// L4-Starts auf der Pruefsitzung, und die kopflose Strecke aus S3 fuer L2,
/// L3 und L10.
#[derive(Debug, Clone)]
pub struct Gesamtlauf {
    /// Das Binaerprogramm im Buendel, `KRK.app/Contents/MacOS/krk`.
    pub programm: PathBuf,
    /// Pruefordner A mit 10.000 Eintraegen.
    pub ordner_a: PathBuf,
    /// Pruefordner B mit 10.000 Eintraegen an einem anderen Pfad.
    pub ordner_b: PathBuf,
    /// Der Pruefordner mit 100.000 Eintraegen.
    pub ordner100k: PathBuf,
    /// Das Kopierziel fuer L8 und L9, auf demselben APFS-Datentraeger wie A.
    pub kopierziel: PathBuf,
    /// Wie oft jede Zusage innerhalb einer Runde gemessen wird. C8 sagt zwanzig.
    pub wiederholungen: usize,
    /// Wie oft die ganze Messung wiederholt wird.
    pub runden: usize,
}

/// Die Rohwerte einer Runde des Gesamtlaufs.
#[derive(Debug, Clone, Default)]
struct Gesamtrohrunde {
    l1: Vec<Duration>,
    l2: Vec<Duration>,
    l3: Vec<Duration>,
    l4: Vec<Duration>,
    l5_tab: Vec<Duration>,
    l5_fenster: Vec<Duration>,
    l6: Vec<Duration>,
    l7: Vec<Duration>,
    l8: Vec<Duration>,
    l9: Vec<Duration>,
    l10_erste: Vec<Duration>,
    l10_voll: Vec<Duration>,
}

/// Was der Gesamtlauf ergeben hat.
#[derive(Debug, Clone)]
pub struct Gesamtergebnis {
    /// Die Bildwiederholrate, wie die Anwendung sie aus `NSScreen` gelesen hat.
    pub bildwiederholrate: i64,
    /// Eine Bildlaenge, der Kehrwert der Rate.
    pub bildlaenge: Duration,
    /// Der Unterordner, an dem L6 gemessen wurde.
    pub unterordner: PathBuf,
    /// Die Systemlast unmittelbar vor dem Lauf (`sysctl vm.loadavg`).
    pub systemlast_vorher: String,
    /// Die Systemlast unmittelbar nach dem Lauf.
    pub systemlast_nachher: String,
    /// Die gemessenen Zusagen, in der Reihenfolge des Berichts.
    pub zusagen: Vec<Zusage>,
}

impl Gesamtergebnis {
    /// Ob jede abgefragte Zusage ihr Mass in jeder Runde haelt.
    pub fn bestanden(&self) -> bool {
        self.zusagen
            .iter()
            .all(|zusage| zusage.immer_gehalten() != Some(false))
    }
}

impl Gesamtlauf {
    /// Faehrt alle Runden und setzt das Ergebnis zusammen.
    pub fn fahren(&self) -> io::Result<Gesamtergebnis> {
        for ordner in [&self.ordner_a, &self.ordner_b, &self.ordner100k] {
            if !ordner.is_dir() {
                return Err(io::Error::other(format!(
                    "{} ist kein Verzeichnis",
                    ordner.display()
                )));
            }
        }
        kopierziel_pruefen(&self.ordner_a, &self.kopierziel)?;
        let unterordner = unterordner_sicherstellen(&self.ordner_a)?;
        let plan = plan_schreiben(self, &unterordner)?;
        // Vor der ersten Runde, nicht erst vor dem ersten Sitzungslauf: der
        // Waechter lebt bis zum Ende von `fahren` und spielt die Sitzung des
        // Nutzers auch dann zurueck, wenn eine Runde mit `?` abbricht oder der
        // Messende mit Strg+C dazwischenfaehrt.
        let _sitzung = Sitzungssicherung::anlegen()?;

        let systemlast_vorher = systemlast();
        let mut rohrunden = Vec::with_capacity(self.runden);
        let mut rate = None;
        for nummer in 1..=self.runden {
            eprintln!("krk-bench: Runde {nummer} von {}", self.runden);
            let (gemeldete_rate, runde) = self.eine_gesamtrunde(&plan)?;
            rate = rate.or(gemeldete_rate);
            rohrunden.push(runde);
        }
        let systemlast_nachher = systemlast();
        let _ = std::fs::remove_file(&plan);

        let sammeln = |waehlen: fn(&Gesamtrohrunde) -> &Vec<Duration>| -> Vec<Vec<Duration>> {
            rohrunden
                .iter()
                .map(|runde| waehlen(runde).clone())
                .collect()
        };
        let (bildwiederholrate, bildlaenge) = bildlaenge_bilden(rate)?;

        Ok(Gesamtergebnis {
            bildwiederholrate,
            bildlaenge,
            unterordner,
            systemlast_vorher,
            systemlast_nachher,
            zusagen: vec![
                Zusage {
                    kennung: "L1",
                    was: "Tastendruck bis Ende des Zeichendurchgangs",
                    // 95 Prozent im ersten Bild, keine Obergrenze je Einzelwert.
                    mass: Abnahmemass::AnteilImBild {
                        bildlaenge,
                        mindestanteil_prozent: 95,
                        obergrenze_bilder: None,
                    },
                    runden: sammeln(|runde| &runde.l1),
                },
                Zusage {
                    kennung: "L2",
                    was: "Pruefordner A: erste Bildschirmseite (Kernanteil, kopflos)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l2),
                },
                Zusage {
                    kennung: "L3",
                    was: "Pruefordner A: vollstaendig gelesen und sortiert (kopflos, warm)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(400)),
                    runden: sammeln(|runde| &runde.l3),
                },
                Zusage {
                    kennung: "L4",
                    was: "Prozessstart bis bedienbare Pruefsitzung (warm)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(1000)),
                    runden: sammeln(|runde| &runde.l4),
                },
                Zusage {
                    kennung: "L5",
                    was: "Wechsel auf den verdeckten Tab (Ordner bereits gelesen)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(50)),
                    runden: sammeln(|runde| &runde.l5_tab),
                },
                Zusage {
                    kennung: "L5",
                    was: "Wechsel des aktiven Dateifensters",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(50)),
                    runden: sammeln(|runde| &runde.l5_fenster),
                },
                Zusage {
                    kennung: "L6",
                    was: "Einstieg in den Unterordner mit 1.000 Eintraegen",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l6),
                },
                Zusage {
                    kennung: "L7",
                    was: "Vorschau des ausgewaehlten Eintrags sichtbar",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l7),
                },
                Zusage {
                    kennung: "L8",
                    was: "Kopie gestartet bis Fortschritt in der Statuszeile",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(200)),
                    runden: sammeln(|runde| &runde.l8),
                },
                Zusage {
                    kennung: "L9",
                    was: "Tastendruck waehrend laufender Kopie, bis Ende des Zeichendurchgangs",
                    // Seit dem 260807-0832 zweiteilig und nicht mehr dasselbe
                    // Mass wie L1: mindestens 85 Prozent erreichen das erste
                    // Bild, und jede Eingabe erreicht spaetestens das zweite.
                    mass: Abnahmemass::AnteilImBild {
                        bildlaenge,
                        mindestanteil_prozent: 85,
                        obergrenze_bilder: Some(2),
                    },
                    runden: sammeln(|runde| &runde.l9),
                },
                Zusage {
                    kennung: "L10",
                    was: "100.000 Eintraege: erste Bildschirmseite (Kernanteil, kopflos)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(100)),
                    runden: sammeln(|runde| &runde.l10_erste),
                },
                Zusage {
                    kennung: "L10",
                    was: "100.000 Eintraege: vollstaendig gelesen (kopflos, warm)",
                    mass: Abnahmemass::Perzentil(Duration::from_millis(4000)),
                    runden: sammeln(|runde| &runde.l10_voll),
                },
            ],
        })
    }

    /// Eine Runde: der Sitzungslauf, die L4-Starts, die kopflose Strecke.
    fn eine_gesamtrunde(&self, plan: &Path) -> io::Result<(Option<i64>, Gesamtrohrunde)> {
        // Zuerst der Sitzungslauf: er stellt die Pruefsitzung her und
        // schreibt sie als session.toml, die die L4-Starts danach vorfinden.
        let (rate, zeilen) = self.sitzung_messen(plan)?;
        let hole = |name: &str| -> io::Result<Vec<Duration>> {
            let werte = werte_lesen(&zeilen, name);
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
        let l1 = hole("l1")?;
        let l5_tab = hole("l5-tab")?;
        let l5_fenster = hole("l5-fenster")?;
        let l6 = hole("l6")?;
        let l7 = hole("l7")?;
        let l8 = hole("l8")?;
        let l9 = hole("l9")?;

        let l4 = starts_messen(
            &self.programm,
            &["--messmodus", "sitzungsstart"],
            self.wiederholungen,
        )?;

        // Die kopflose Strecke aus S3: L2 und L3 auf Pruefordner A, L10 auf
        // dem grossen Ordner, beide warm, wie C8 die Zusagen stellt.
        let reihe_a = Messreihe::fahren(&self.ordner_a, Cache::Warm, self.wiederholungen)?;
        let reihe_gross = Messreihe::fahren(&self.ordner100k, Cache::Warm, self.wiederholungen)?;

        Ok((
            rate,
            Gesamtrohrunde {
                l1,
                l2: reihe_a.groessen[0].werte.clone(),
                l3: reihe_a.groessen[1].werte.clone(),
                l4,
                l5_tab,
                l5_fenster,
                l6,
                l7,
                l8,
                l9,
                l10_erste: reihe_gross.groessen[0].werte.clone(),
                l10_voll: reihe_gross.groessen[1].werte.clone(),
            },
        ))
    }

    /// Der Sitzungslauf der Anwendung: L1, L5, L6, L7, L8 und L9.
    fn sitzung_messen(&self, plan: &Path) -> io::Result<(Option<i64>, Vec<String>)> {
        let planpfad = plan.display().to_string();
        let ausgang = lauf_fahren(&self.programm, &["--messmodus", &planpfad], FRIST_SITZUNG)?;
        if !ausgang.messzeilen.iter().any(|zeile| zeile == "fertig") {
            return Err(io::Error::other(format!(
                "der Sitzungslauf der Anwendung ist nicht bis zum Ende gekommen.{}",
                ausgang.klage()
            )));
        }
        let rate = zahl_lesen(&ausgang.messzeilen, "bildwiederholrate").map(|zahl| zahl as i64);
        if rate.is_none() {
            return Err(io::Error::other(
                "die Anwendung hat keine Bildwiederholrate gemeldet. \
                 Ohne sie ist keine der Bildzusagen gegen ihre Herleitung pruefbar; \
                 es wird keine Zahl ausgegeben."
                    .to_owned(),
            ));
        }
        Ok((rate, ausgang.messzeilen))
    }
}

/// Die Sicherung der Nutzersitzung ueber einen Gesamtlauf hinweg.
///
/// **Warum es sie gibt.** Der Sitzungslauf stellt die Pruefsitzung aus C8 ueber
/// `session.toml` her, und zwar in der **echten Ablage des Nutzers**
/// (`Messplan::herstellen`): die L4-Starts danach muessen sie auf dem
/// gewoehnlichen Weg vorfinden, sonst maesse L4 etwas anderes als das
/// Wiederherstellen einer Sitzung. Der Lauf nimmt dem Nutzer damit aber seine
/// Tabs, Ordner und Breiten weg. Bis zum 260806 hat der Messende sie von Hand
/// gerettet; dieselbe Sorgfalt, die [`kopierziel_pruefen`] fuer fremden Inhalt
/// im Kopierziel aufbringt, gehoert auch hierher.
///
/// **Zurueckgespielt wird in [`Drop`] und nicht am Ende von
/// [`Gesamtlauf::fahren`].** Jede Runde kann mit `?` abbrechen, und gerade der
/// Abbruch ist der Fall, in dem eine Zeile am Ende der Funktion nicht mehr
/// liefe. Eine Panik wickelt ebenso ab und laeuft ueber dieselbe Bahn, weil der
/// Workspace kein `panic = "abort"` setzt.
///
/// **Ein Signal wickelt dagegen nicht ab.** SIGINT (Strg+C) und SIGTERM enden
/// ueber den Standardgriff, ohne dass ein einziges [`Drop`] liefe. Ein
/// Gesamtlauf faehrt Minuten bis Viertelstunden, und Strg+C ist der uebliche
/// Weg, ihn abzubrechen; deshalb haengt [`signalwache_starten`] fuer SIGINT,
/// SIGTERM und SIGHUP einen eigenen Griff ein, der ueber [`SICHERUNG`] an
/// dieselbe Sicherung kommt.
///
/// **Was ungedeckt bleibt**, und zwar vollstaendig aufgezaehlt:
///
/// - SIGKILL und SIGSTOP. Beide lassen sich nicht abfangen, von keinem
///   Programm.
/// - Ein Signal, das **nur** `krk-bench` erreicht und nicht den gerade
///   laufenden `krk`-Kindprozess: ein `kill` auf die eine Prozessnummer statt
///   Strg+C, das die ganze Vordergrundgruppe trifft. Der Waechter spielt dann
///   zwar zurueck, das weiterlaufende Kind schreibt beim Beenden aber wieder
///   die Pruefsitzung darueber.
struct Sitzungssicherung {
    /// Der Pfad der echten `session.toml`.
    pfad: PathBuf,
    /// Ihr Inhalt vor dem Lauf. `None` heisst: es gab noch keine, und
    /// zurueckzuspielen ist dann ihre Abwesenheit.
    vorher: Option<Vec<u8>>,
}

/// Die Sicherung an der Stelle, an der auch der Signalfaden sie erreicht.
///
/// [`Sitzungswaechter`] haelt sie nicht selbst, denn der Signalfaden kaeme an
/// den Stapel von [`Gesamtlauf::fahren`] nicht heran. Beide Wege zurueck nehmen
/// sie hier mit `take` heraus, und deshalb spielt genau einer von beiden
/// zurueck: wer zuerst kommt.
static SICHERUNG: Mutex<Option<Sitzungssicherung>> = Mutex::new(None);

/// Spielt den gesicherten Stand zurueck, sofern es noch niemand getan hat.
///
/// Der zweite Aufruf ist folgenlos, und das ist die Bedingung und nicht nur
/// eine angenehme Eigenschaft: haette der Signalfaden schon zurueckgespielt,
/// duerfte ein spaeter fallender [`Sitzungswaechter`] nicht noch einmal
/// schreiben.
fn sitzung_zurueckspielen() {
    let genommen = SICHERUNG
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .take();
    // Das Schloss ist mit dem Ende der Anweisung darueber schon wieder offen;
    // es wacht ueber das "genau einmal" und nicht ueber das Schreiben.
    drop(genommen);
}

/// Solange er lebt, steht die Pruefsitzung in der Ablage des Nutzers.
///
/// Sein Ende ist der regulaere Weg zurueck, und er greift auf denselben drei
/// Wegen wie bisher: am Ende von [`Gesamtlauf::fahren`], beim `?`-Abbruch einer
/// Runde und beim Abwickeln einer Panik.
struct Sitzungswaechter;

impl Drop for Sitzungswaechter {
    fn drop(&mut self) {
        sitzung_zurueckspielen();
    }
}

/// Haengt den Griff ein, der die Sitzung auch bei einem Signal zurueckspielt.
///
/// **Warum eine Kiste dafuer und nicht `libc` von Hand.** Ein Griff, der
/// unmittelbar im Signalkontext laeuft, duerfte weder eine Datei schreiben noch
/// melden; `signal-hook` schreibt dort nur in ein Selbstrohr und laesst das
/// Zurueckspielen auf diesem gewoehnlichen Faden geschehen. `krk-bench` behaelt
/// damit sein `#![deny(unsafe_code)]`, und der Grenzstein aus CLAUDE.md bleibt,
/// wo er steht: `unsafe` nur in `krk-core/src/verzeichnis/sys.rs` und
/// `krk-ui/src/appkit/mod.rs`.
///
/// **Warum das keine Messung verfaelscht.** Der Faden schlaeft im `read` auf
/// dem Rohr und kostet keine Rechenzeit; er wacht genau einmal auf, und dann
/// endet der Lauf ohnehin. `signal-hook` haengt seinen Griff mit `SA_RESTART`
/// ein, also bricht kein Systemaufruf der Messstrecke mit `EINTR` ab. Und der
/// Griff steht allein in `krk-bench`: `krk-ui` und `krk-core` sind
/// ausgelieferter Code und fassen kein Signal an.
fn signalwache_starten() -> io::Result<()> {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    use signal_hook::iterator::Signals;

    // SIGHUP steht daneben, weil er dieselbe Ursache hat: das Fenster, in dem
    // der Lauf faehrt, wird geschlossen statt mit Strg+C abgebrochen.
    let mut signale = Signals::new([SIGINT, SIGTERM, SIGHUP]).map_err(|fehler| {
        io::Error::other(format!(
            "der Signalgriff laesst sich nicht einhaengen: {fehler}. Ohne ihn \
             ueberlebte die Sitzung des Nutzers kein Strg+C, und der Lauf schriebe \
             gleich die Pruefsitzung darueber; es wird keine Zahl ausgegeben."
        ))
    })?;
    thread::Builder::new()
        .name("krk-bench-signalwache".to_owned())
        .spawn(move || {
            let Some(signal) = signale.forever().next() else {
                return;
            };
            sitzung_zurueckspielen();
            eprintln!(
                "krk-bench: Signal {signal} empfangen, der Lauf bricht ab. \
                 Die Sitzung des Nutzers steht wieder."
            );
            // 128 + Signalnummer ist der Ausgangswert, den eine Shell fuer
            // einen durch ein Signal beendeten Prozess ohnehin bildet; Strg+C
            // ergibt damit die gewohnte 130.
            std::process::exit(128 + signal);
        })?;
    Ok(())
}

impl Sitzungssicherung {
    /// Liest den vorigen Stand, bevor der Lauf ihn ueberschreibt.
    ///
    /// Eine Datei, die daliegt und sich **nicht lesen** laesst, bricht den Lauf
    /// ab. Weiterzumessen hiesse, eine Sitzung zu ueberschreiben, die sich
    /// nicht zurueckspielen laesst; eine Zahl ist das nicht wert. Aus demselben
    /// Grund bricht auch ein Signalgriff ab, der sich nicht einhaengen laesst.
    fn anlegen() -> io::Result<Sitzungswaechter> {
        let sicherung = Self::an(
            krk_core::ablage::Ablageort::im_benutzerverzeichnis()?
                .datei(krk_core::ablage::Datei::Sitzung),
        )?;
        *SICHERUNG
            .lock()
            .unwrap_or_else(|vergiftet| vergiftet.into_inner()) = Some(sicherung);
        // Erst ablegen, dann einhaengen: der Faden faende die Stelle sonst leer
        // vor. Scheitert das Einhaengen, faellt die eben abgelegte Sicherung
        // wieder weg, und es steht noch nichts, was zurueckzuspielen waere.
        let waechter = Sitzungswaechter;
        signalwache_starten()?;
        Ok(waechter)
    }

    /// Dieselbe Sicherung an einem frei gewaehlten Pfad.
    ///
    /// Keine Pruefhintertuer, sondern die Bedingung dafuer, dass das
    /// Zurueckspielen ueberhaupt ohne die echte Sitzung des Nutzers pruefbar
    /// ist — dieselbe Form, in der `krk_core::ablage::Ablageort::an` es haelt.
    fn an(pfad: PathBuf) -> io::Result<Self> {
        let vorher = match std::fs::read(&pfad) {
            Ok(inhalt) => Some(inhalt),
            Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => None,
            Err(fehler) => {
                return Err(io::Error::other(format!(
                    "die Sitzung des Nutzers ({}) liegt da, laesst sich aber nicht sichern: \
                     {fehler}. Der Messlauf ueberschreibt sie mit der Pruefsitzung und \
                     koennte sie danach nicht zurueckspielen; es wird keine Zahl ausgegeben.",
                    pfad.display()
                )));
            }
        };
        Ok(Self { pfad, vorher })
    }
}

impl Drop for Sitzungssicherung {
    fn drop(&mut self) {
        let ergebnis = match &self.vorher {
            Some(inhalt) => std::fs::write(&self.pfad, inhalt),
            None => match std::fs::remove_file(&self.pfad) {
                Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(()),
                anderes => anderes,
            },
        };
        // Gemeldet und nicht verschwiegen: der Nutzer muss erfahren, dass seine
        // Sitzung jetzt die Pruefsitzung ist. Ein Panic im Drop waere der
        // falsche Weg, er verdeckte den eigentlichen Fehler des Laufs.
        if let Err(fehler) = ergebnis {
            eprintln!(
                "krk-bench: die Sitzung des Nutzers liess sich nicht nach {} \
                 zurueckspielen: {fehler}. Dort steht jetzt die Pruefsitzung.",
                self.pfad.display()
            );
        }
    }
}

/// Prueft das Kopierziel: Verzeichnis, leer, derselbe Datentraeger wie A.
///
/// Legt es an, wenn es fehlt. **Ein Ziel auf einem anderen Datentraeger wird
/// nicht angenommen**, weil die duennbesetzten Pruefdateien dort als Nullen
/// ausgeschrieben wuerden; die Herleitung steht in `### Frage 5` des Plans.
/// Und es muss leer sein, weil der Messlauf es zwischen den Wiederholungen
/// leert und fremder Inhalt dabei nicht verschwinden darf.
pub fn kopierziel_pruefen(ordner_a: &Path, kopierziel: &Path) -> io::Result<()> {
    use std::os::unix::fs::MetadataExt as _;
    if !kopierziel.exists() {
        std::fs::create_dir_all(kopierziel)?;
    }
    if !kopierziel.is_dir() {
        return Err(io::Error::other(format!(
            "das Kopierziel {} ist kein Verzeichnis",
            kopierziel.display()
        )));
    }
    if std::fs::read_dir(kopierziel)?.next().is_some() {
        return Err(io::Error::other(format!(
            "das Kopierziel {} ist nicht leer. Der Messlauf leert es zwischen den \
             Wiederholungen; fremder Inhalt darf dabei nicht verschwinden.",
            kopierziel.display()
        )));
    }
    let geraet = |pfad: &Path| std::fs::metadata(pfad).map(|angaben| angaben.dev());
    if geraet(ordner_a)? != geraet(kopierziel)? {
        return Err(io::Error::other(format!(
            "das Kopierziel {} liegt auf einem anderen Datentraeger als Pruefordner A \
             ({}). L8 und L9 messen auf demselben APFS-Datentraeger; auf einem anderen \
             wuerden die duennbesetzten Pruefdateien als Nullen ausgeschrieben, und die \
             Zahl waere ein Durchsatz und keine Sichtbarkeitszusage. Es wird keine Zahl \
             ausgegeben.",
            kopierziel.display(),
            ordner_a.display()
        )));
    }
    Ok(())
}

/// Stellt den L6-Unterordner mit 1.000 Eintraegen neben Pruefordner A sicher.
///
/// Er entsteht nach demselben Verfahren wie die drei Pruefordner aus C8, mit
/// eigenem Startwert und Steckbrief, und wird wiederverwendet, wenn er schon
/// steht. Ein vorhandener Ordner ohne passenden Steckbrief wird abgewiesen,
/// statt auf unbekanntem Bestand zu messen.
fn unterordner_sicherstellen(ordner_a: &Path) -> io::Result<PathBuf> {
    let name = ordner_a
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "pruefordner".to_owned());
    let unterordner = ordner_a.with_file_name(format!("{name}-l6"));
    match fixture::steckbrief_lesen(&unterordner) {
        Some(brief) if brief.eintraege == EINTRAEGE_L6 => Ok(unterordner),
        Some(brief) => Err(io::Error::other(format!(
            "{} traegt laut Steckbrief {} Eintraege statt {EINTRAEGE_L6}. \
             Loesche den Ordner samt Steckbrief; der Lauf legt ihn neu an.",
            unterordner.display(),
            brief.eintraege
        ))),
        None if unterordner.exists() => Err(io::Error::other(format!(
            "{} steht ohne Steckbrief da; auf unbekanntem Bestand misst L6 nicht. \
             Loesche den Ordner; der Lauf legt ihn neu an.",
            unterordner.display()
        ))),
        None => {
            fixture::erzeugen(&unterordner, EINTRAEGE_L6, STARTWERT_L6)?;
            Ok(unterordner)
        }
    }
}

/// Schreibt den Messplan fuer den Sitzungslauf der Anwendung.
///
/// Der Abschnitt `[sitzung]` ist die Pruefsitzung aus C8 **in der
/// Serialisierung von `session.toml`**: zwei Dateifenster mit je zwei Tabs,
/// links A sichtbar und B dahinter, rechts umgekehrt, alle Bereiche sichtbar,
/// die Breiten im Auslieferungszustand. Serialisiert wird die Struktur aus
/// `krk-core/src/ablage/sitzung.rs`; ein zweites Format entsteht nicht.
fn plan_schreiben(lauf: &Gesamtlauf, unterordner: &Path) -> io::Result<PathBuf> {
    use krk_core::ablage::sitzung::{Dateifenster, Sitzung, Tab};

    let sitzung = Sitzung {
        fenster: [
            Dateifenster {
                aktiver_tab: 0,
                tabs: vec![Tab::auf(&lauf.ordner_a), Tab::auf(&lauf.ordner_b)],
            },
            Dateifenster {
                aktiver_tab: 0,
                tabs: vec![Tab::auf(&lauf.ordner_b), Tab::auf(&lauf.ordner_a)],
            },
        ],
        ..Sitzung::default()
    };

    let mut wurzel = toml::Table::new();
    wurzel.insert(
        "kopierziel".to_owned(),
        toml::Value::String(lauf.kopierziel.display().to_string()),
    );
    wurzel.insert(
        "unterordner".to_owned(),
        toml::Value::String(unterordner.display().to_string()),
    );
    wurzel.insert(
        "sitzung".to_owned(),
        toml::Value::try_from(&sitzung).map_err(io::Error::other)?,
    );
    let text = toml::to_string(&wurzel).map_err(io::Error::other)?;

    let pfad = std::env::temp_dir().join(format!("krk-messplan-{}.toml", std::process::id()));
    std::fs::write(&pfad, text)?;
    Ok(pfad)
}

/// Die Systemlast, wie `sysctl vm.loadavg` sie meldet.
///
/// Die neunte Kopfangabe seit dem 260804-2318: an ihr wird der
/// L4-Streuungsvergleich aus S22 pruefbar, ob eine Runde unter Fremdlast
/// lief.
pub fn systemlast() -> String {
    let roh = bericht::befehl_ausgabe("/usr/sbin/sysctl", &["-n", "vm.loadavg"]);
    if roh.is_empty() {
        "nicht ermittelt".to_owned()
    } else {
        roh
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
    text.push_str(ZWEI_MASSE);
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "Das 95. Perzentil steht je Runde einmal. Ausgewiesen sind das beste und"
    );
    let _ = writeln!(
        text,
        "das schlechteste der {} Runden; Median, Minimum und Maximum laufen ueber",
        lauf.runden
    );
    let _ = writeln!(
        text,
        "alle Einzelwerte aller Runden. Die Spalte \"im Bild\" traegt den Anteil der"
    );
    let _ = writeln!(
        text,
        "schlechtesten Runde, weil an ihr das Urteil haengt; die Spalte \"hoechstwert\""
    );
    let _ = writeln!(
        text,
        "traegt den groessten Einzelwert aller Runden in Bildlaengen, an dem die"
    );
    let _ = writeln!(text, "Obergrenze je Einzelwert haengt.");
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "{:<56}{:>13}{:>13}{:>12}{:>12}{:>12}{:>11}{:>14}{:>22}   Urteil",
        "Gemessene Groesse",
        "p95 bestes",
        "p95 schlecht",
        "Median",
        "Minimum",
        "Maximum",
        "im Bild",
        "hoechstwert",
        "Abnahme nach"
    );
    for zusage in &ergebnis.zusagen {
        let _ = writeln!(
            text,
            "{:<56}{:>13}{:>13}{:>12}{:>12}{:>12}{:>11}{:>14}{:>22}   {}",
            format!("{} — {}", zusage.kennung, zusage.was),
            bericht::spanne(zusage.bestes_perzentil()),
            bericht::spanne(zusage.schlechtestes_perzentil()),
            bericht::spanne(zusage.median()),
            bericht::spanne(zusage.minimum()),
            bericht::spanne(zusage.maximum()),
            match zusage.schlechtester_anteil() {
                Some(prozent) => format!("{prozent:.1} %"),
                None => "-".to_owned(),
            },
            match zusage.hoechstwert_in_bildern() {
                Some(bilder) => format!("{bilder:.2} Bilder"),
                None => "-".to_owned(),
            },
            zusage.mass.beschreibung(),
            urteil(zusage)
        );
    }
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "Urteil des Gates: {}",
        if ergebnis.bestanden() {
            "bestanden — jede der fuenf abgefragten Zusagen haelt ihr Mass in jeder Runde."
        } else {
            "NICHT bestanden — mindestens eine Zusage verfehlt ihr Mass in mindestens einer Runde."
        }
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Das 95. Perzentil Runde fuer Runde");
    let _ = writeln!(text, "----------------------------------");
    let _ = writeln!(
        text,
        "Fuer L1 eine Kennzahl ohne eigenes Urteil; das Urteil steht im Abschnitt darunter."
    );
    for zusage in &ergebnis.zusagen {
        let werte: Vec<String> = zusage
            .perzentile()
            .into_iter()
            .map(bericht::spanne)
            .collect();
        let _ = writeln!(text, "{:<8}{}", zusage.kennung, werte.join("  "));
    }
    let _ = writeln!(text);

    bericht::anteil_je_runde(&mut text, ergebnis.bildlaenge, &ergebnis.zusagen);

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
pub(crate) fn urteil(zusage: &Zusage) -> String {
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

/// Warum in einer Tabelle zwei Abnahmemasse nebeneinander stehen.
const ZWEI_MASSE: &str = "\
Zwei Abnahmemasse stehen nebeneinander, und die Spalte \"Abnahme nach\" nennt je
Zeile, welches gilt. Acht der zehn Zusagen aus C8 sagen eine Dauer zu, die der
Nutzer abwartet; sie werden ueber das 95. Perzentil der Runde abgenommen. L1 und
L9 sagen zu, dass die Reaktion im naechsten Bild erscheint; sie werden seit dem
260803-1810 ueber den Anteil der Eingaben abgenommen, die das erreichen. Die
Spalte \"im Bild\" traegt diesen Anteil und steht auf \"-\", wo das Mass nicht
gilt. Seit dem 260807-0832 fordern L1 und L9 dabei verschiedene Anteile, und L9
traegt daneben eine Obergrenze je Einzelwert: keine Eingabe liegt ueber zwei
Bildlaengen. Die Spalte \"hoechstwert\" traegt diese zweite Haelfte, ausgedrueckt
in Bildlaengen. Fuer L1 sind Perzentil, Median, Minimum und Maximum Kennzahlen
ohne eigenes Urteil.
";

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

**Das Bild selbst bleibt ungemessen, und eine Bildgrenze ist keine
Photonenmessung.** Eine Bildgrenze ist der Zeitpunkt, an dem das System sein
naechstes Bild vorbereitet, nicht der, an dem ein Pixel leuchtet. Aus dem
eigenen Prozess heraus ist der zweite nicht feststellbar. Das faellt beim
Anteilsmass staerker ins Gewicht als beim Perzentil, weil die Bildlaenge dort
die Grenze selbst ist: ein Wert dicht an der Grenze koennte bei einer echten
Bildschirmmessung auf die andere Seite fallen. Naeheres im Abschnitt Lesart.

**Der Bildtakt kann stehenbleiben.** Ein `CADisplayLink` taktet nur, solange das
Fenster sichtbar ist. Bleibt er stehen, bricht der Messlauf nach zehn Sekunden
mit einer Meldung ab und gibt keine Zahl aus; die Meldung nennt die Zahl der
seither eingegangenen Bildgrenzen und trennt damit ein stehendes Bild von einer
langsamen Oberflaeche.
";

fn rate_beschreiben(ergebnis: &Durchstichergebnis) -> String {
    format!(
        "{} Hz, gelesen aus NSScreen.maximumFramesPerSecond am Bildschirm des \
         gemessenen Fensters; eine Bildlaenge sind damit {}",
        ergebnis.bildwiederholrate,
        bericht::spanne(ergebnis.bildlaenge)
    )
}

/// Pfad, Startwert und Eintragszahl eines Pruefordners.
pub(crate) fn ordner_beschreiben(ordner: &Path) -> String {
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

L1 wird nicht darueber abgenommen. Der Nutzer hat das Abnahmemass am 260803-1810
geaendert: nicht mehr 16 ms fuer das 95. Perzentil, sondern der Anteil der
Eingaben, die ihr naechstes Bild erreichen. Zwei Gruende tragen die Aenderung.
Messtechnisch lagen die 16 ms innerhalb der Streuung ihres eigenen Verfahrens:
das 95. Perzentil der Wartezeit auf die naechste Bildgrenze liegt selbst fuer
eine Anwendung ohne jede Verarbeitungszeit bei rund 15,8 ms, und acht von
achtzehn Runden verfehlten die Zahl bei unveraendertem Programm. Wahrnehmbar ist
eine Spanne dieser Groesse ohnehin nicht; eine Zahl, die keine unterscheidbare
Eigenschaft beschreibt, taugt nicht als Abnahmekriterium. Die Vorschrift steht in
C8 unter \"Warum L1 und L9 den Anteil zaehlen und nicht die Spanne\", der
Datensatz ist 260803-1755_*_l1-verfehlt-die-16-ms-zusage-am-bildrand.md.

Die Bildlaenge ist der Kehrwert der Bildwiederholrate aus NSScreen, hier also
kein angenommener Wert. Fehlt die Rate, bricht die Auswertung ab und gibt keine
Zahl aus, statt 60 Hz zu unterstellen.

Eine Runde ist genau die Messung, die C8 vorschreibt: zwanzig Wiederholungen je
Zusage. Der Bericht faehrt mehrere Runden, weil ein Urteil, das von Runde zu
Runde wechselt, kein Urteil ist. Eine Zusage gilt hier nur dann als gehalten,
wenn sie es in jeder Runde tut.
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

    /// Eine Bildlaenge bei 60 Hz, auf die Nanosekunde gerundet.
    fn ein_bild() -> Duration {
        Duration::from_secs_f64(1.0 / 60.0)
    }

    /// Das Mass von L1: 95 Prozent im ersten Bild, keine Obergrenze.
    fn anteilszusage(runden: Vec<Vec<Duration>>) -> Zusage {
        Zusage {
            kennung: "L1",
            was: "Tastendruck bis Ende des Zeichendurchgangs",
            mass: Abnahmemass::AnteilImBild {
                bildlaenge: ein_bild(),
                mindestanteil_prozent: 95,
                obergrenze_bilder: None,
            },
            runden,
        }
    }

    /// Das Mass von L9 seit dem 260807-0832: 85 Prozent im ersten Bild, und
    /// keine Eingabe ueber zwei Bildlaengen.
    fn l9_zusage(runden: Vec<Vec<Duration>>) -> Zusage {
        Zusage {
            kennung: "L9",
            was: "Tastendruck waehrend laufender Kopie, bis Ende des Zeichendurchgangs",
            mass: Abnahmemass::AnteilImBild {
                bildlaenge: ein_bild(),
                mindestanteil_prozent: 85,
                obergrenze_bilder: Some(2),
            },
            runden,
        }
    }

    #[test]
    fn eine_bildlaenge_entsteht_nur_aus_einer_gemeldeten_rate() {
        let (hertz, bildlaenge) = bildlaenge_bilden(Some(60)).expect("60 Hz sind gueltig");
        assert_eq!(hertz, 60);
        assert!((bildlaenge.as_secs_f64() * 1_000.0 - 16.667).abs() < 0.001);

        // Ohne Rate wird nicht auf 60 Hz zurueckgefallen, sondern abgebrochen.
        let fehler = bildlaenge_bilden(None).expect_err("ohne Rate darf es keine Zahl geben");
        assert!(
            fehler.to_string().contains("keine Bildwiederholrate"),
            "unerwartete Meldung: {fehler}"
        );
        assert!(bildlaenge_bilden(Some(0)).is_err());
        assert!(bildlaenge_bilden(Some(-1)).is_err());
    }

    #[test]
    fn eine_eingabe_erreicht_ihr_bild_bis_genau_zur_bildlaenge() {
        // Genau eine Bildlaenge zaehlt noch als erreicht, ein Nanosekunde mehr
        // nicht: C8 sagt "hoechstens eine Bildlaenge".
        let zusage = anteilszusage(vec![vec![
            Duration::ZERO,
            ein_bild(),
            ein_bild() + Duration::from_nanos(1),
            ein_bild() * 2,
        ]]);
        assert_eq!(zusage.im_bild(), Some(vec![(2, 4)]));
        assert_eq!(zusage.schlechtester_anteil(), Some(50.0));
    }

    #[test]
    fn eine_von_zwanzig_darf_ihr_bild_verpassen() {
        let schnell = ms(8);
        let langsam = ein_bild() * 2;

        let neunzehn = {
            let mut werte = vec![schnell; 19];
            werte.push(langsam);
            anteilszusage(vec![werte])
        };
        assert_eq!(neunzehn.gehalten_in(), Some((1, 1)));
        assert_eq!(neunzehn.immer_gehalten(), Some(true));

        let achtzehn = {
            let mut werte = vec![schnell; 18];
            werte.push(langsam);
            werte.push(langsam);
            anteilszusage(vec![werte])
        };
        assert_eq!(achtzehn.gehalten_in(), Some((0, 1)));
        assert_eq!(achtzehn.immer_gehalten(), Some(false));
    }

    #[test]
    fn gehalten_heisst_auch_beim_anteil_in_jeder_runde_gehalten() {
        let gute_runde = vec![ms(8); 20];
        let schlechte_runde = vec![ein_bild() * 2; 20];
        let zusage = anteilszusage(vec![gute_runde.clone(), schlechte_runde, gute_runde]);

        assert_eq!(zusage.gehalten_in(), Some((2, 3)));
        assert_eq!(zusage.immer_gehalten(), Some(false));
        assert_eq!(zusage.schlechtester_anteil(), Some(0.0));

        // Das Perzentil bleibt als Kennzahl erhalten, faellt aber kein Urteil.
        assert!(zusage.bestes_perzentil() <= zusage.schlechtestes_perzentil());
    }

    /// Ein Einzelwert in Millisekunden, wie der Messbericht ihn ausweist.
    fn msf(millisekunden: f64) -> Duration {
        Duration::from_secs_f64(millisekunden / 1_000.0)
    }

    /// Die fuenf L9-Runden der Abnahmereihe vom 260805-2207.
    ///
    /// Wortgleich aus `messungen/260805-2207-MacBookPro15-1-abnahme.txt`
    /// uebernommen, Zeilen 288 bis 313. Diese hundert Werte sind der Anlass der
    /// neuen Fassung von L9: nach der alten haelt die Reihe in einer von fuenf
    /// Runden, nach der neuen in allen fuenf.
    fn l9_abnahmereihe() -> Vec<Vec<Duration>> {
        [
            [
                7.713, 18.137, 1.910, 14.317, 8.289, 3.405, 6.761, 5.099, 1.577, 2.592, 8.131,
                3.535, 7.232, 5.806, 6.201, 10.205, 19.153, 12.936, 5.300, 16.138,
            ],
            [
                2.737, 5.590, 10.375, 8.721, 7.420, 2.078, 7.057, 20.203, 12.711, 6.868, 14.839,
                9.607, 20.913, 1.178, 1.744, 10.884, 4.500, 20.898, 5.850, 9.757,
            ],
            [
                13.252, 10.059, 10.839, 11.677, 13.128, 18.961, 5.694, 6.950, 23.429, 12.537,
                4.062, 11.914, 15.748, 11.503, 10.105, 5.111, 5.542, 6.101, 3.127, 11.904,
            ],
            [
                12.565, 2.914, 4.974, 6.387, 1.390, 15.674, 5.257, 12.952, 7.846, 13.107, 7.134,
                2.016, 5.833, 2.749, 6.284, 4.663, 14.468, 7.308, 4.810, 9.415,
            ],
            [
                12.336, 17.469, 16.363, 13.817, 12.297, 14.453, 17.218, 18.825, 8.367, 7.901,
                13.211, 13.759, 7.635, 2.873, 13.056, 4.723, 11.756, 8.169, 1.648, 14.419,
            ],
        ]
        .iter()
        .map(|runde| runde.iter().copied().map(msf).collect())
        .collect()
    }

    #[test]
    fn l9_haelt_die_neue_fassung_in_allen_fuenf_gemessenen_runden() {
        let zusage = l9_zusage(l9_abnahmereihe());

        // Erste Haelfte: der Anteil im ersten Bild. Runde 2 und Runde 5 liegen
        // mit 17 von 20 genau auf den geforderten 85 Prozent und halten damit.
        assert_eq!(
            zusage.im_bild(),
            Some(vec![(18, 20), (17, 20), (18, 20), (20, 20), (17, 20)])
        );
        assert_eq!(
            zusage.anteile_im_bild(),
            Some(vec![90.0, 85.0, 90.0, 100.0, 85.0])
        );

        // Zweite Haelfte: der groesste Einzelwert je Runde, in Bildlaengen.
        // Keiner erreicht zwei, also erreicht jede Eingabe das zweite Bild.
        let hoechstwerte = zusage
            .hoechstwerte_in_bildern()
            .expect("L9 nimmt ueber den Anteil ab");
        for (nummer, (gemessen, groesster_wert)) in hoechstwerte
            .iter()
            .zip([19.153, 20.913, 23.429, 15.674, 18.825])
            .enumerate()
        {
            let erwartet = msf(groesster_wert).as_secs_f64() / ein_bild().as_secs_f64();
            assert!(
                (gemessen - erwartet).abs() < 1e-9,
                "Runde {}: {gemessen} statt {erwartet} Bildlaengen",
                nummer + 1
            );
            assert!(
                *gemessen < 2.0,
                "Runde {}: {gemessen} Bildlaengen reissen die Obergrenze",
                nummer + 1
            );
        }

        assert_eq!(zusage.gehalten_in(), Some((5, 5)));
        assert_eq!(zusage.immer_gehalten(), Some(true));
    }

    #[test]
    fn dieselbe_reihe_verfehlt_das_ungesenkte_mass() {
        // Bis zum 260807-0832 nahm L9 gegen dasselbe Mass ab wie L1. Nur die
        // vierte Runde haelt es; genau daran haengt der Nutzerentscheid.
        let zusage = anteilszusage(l9_abnahmereihe());

        assert_eq!(zusage.gehalten_in(), Some((1, 5)));
        assert_eq!(zusage.immer_gehalten(), Some(false));
        assert_eq!(urteil(&zusage), "VERFEHLT, gehalten in 1 von 5 Runden");
    }

    #[test]
    fn eine_eingabe_ueber_zwei_bildlaengen_reisst_l9_trotz_gehaltenem_anteil() {
        // Ein erfundener Fall, den die vorliegende Reihe nicht enthaelt: der
        // Anteil liegt mit 19 von 20 weit ueber den geforderten 85 Prozent, ein
        // einzelner Wert liegt aber jenseits des zweiten Bildes. Nach der neuen
        // Fassung ist die Runde damit verfehlt.
        let mut ueber = vec![ms(8); 19];
        ueber.push(ein_bild() * 2 + Duration::from_nanos(1));
        let zusage = l9_zusage(vec![ueber]);
        assert_eq!(zusage.im_bild(), Some(vec![(19, 20)]));
        assert_eq!(zusage.gehalten_in(), Some((0, 1)));

        // Genau zwei Bildlaengen halten noch: C8 sagt "spaetestens das zweite
        // Bild", nicht "vor dem zweiten Bild".
        let mut knapp = vec![ms(8); 19];
        knapp.push(ein_bild() * 2);
        assert_eq!(l9_zusage(vec![knapp]).immer_gehalten(), Some(true));

        // Und die erste Haelfte bleibt scharf: vier verpasste Bilder sind
        // 80 Prozent und damit unter den geforderten 85.
        let mut vier_verpasst = vec![ms(8); 16];
        vier_verpasst.extend(vec![ein_bild() + Duration::from_nanos(1); 4]);
        assert_eq!(l9_zusage(vec![vier_verpasst]).gehalten_in(), Some((0, 1)));
    }

    #[test]
    fn das_perzentilmass_bleibt_unberuehrt() {
        let zusage = Zusage {
            kennung: "L2",
            was: "Pruefordner A: erste Bildschirmseite",
            mass: Abnahmemass::Perzentil(ms(100)),
            runden: vec![vec![ms(40); 20], vec![ms(120); 20]],
        };
        assert_eq!(zusage.gehalten_in(), Some((1, 2)));
        assert_eq!(zusage.immer_gehalten(), Some(false));

        // Der Anteil im naechsten Bild gilt fuer diese Zusage nicht und wird
        // deshalb auch nicht ausgewiesen.
        assert_eq!(zusage.im_bild(), None);
        assert_eq!(zusage.schlechtester_anteil(), None);
        assert_eq!(zusage.hoechstwert_in_bildern(), None);
    }

    #[test]
    fn eine_zusage_ohne_mass_bekommt_kein_urteil() {
        let zusage = Zusage {
            kennung: "L10b",
            was: "100.000 Eintraege: vollstaendig gelesen (Beigabe)",
            mass: Abnahmemass::Keine,
            runden: vec![vec![ms(900); 20]],
        };
        assert_eq!(zusage.gehalten_in(), None);
        assert_eq!(zusage.immer_gehalten(), None);
        assert_eq!(urteil(&zusage), "nicht abgefragt");
    }

    #[test]
    fn jede_zeile_nennt_ihr_abnahmemass() {
        assert_eq!(
            Abnahmemass::Perzentil(ms(100)).beschreibung(),
            "p95 <= 100 ms"
        );
        assert_eq!(
            Abnahmemass::AnteilImBild {
                bildlaenge: ein_bild(),
                mindestanteil_prozent: 95,
                obergrenze_bilder: None,
            }
            .beschreibung(),
            ">= 95 % im Bild"
        );
        // L9 traegt zwei Haelften, und die Zeile nennt beide. Nur den Anteil zu
        // nennen hiesse, das Urteil auf halber Grundlage auszuweisen.
        assert_eq!(
            Abnahmemass::AnteilImBild {
                bildlaenge: ein_bild(),
                mindestanteil_prozent: 85,
                obergrenze_bilder: Some(2),
            }
            .beschreibung(),
            ">= 85 %, <= 2 Bilder"
        );
        assert_eq!(Abnahmemass::Keine.beschreibung(), "keine");
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
    fn das_kopierziel_muss_leer_sein_und_wird_notfalls_angelegt() {
        let wurzel = Wegwerfordner::neu("kopierziel");
        let a = wurzel.pfad().join("a");
        fs::create_dir_all(&a).expect("Anlegen gescheitert");
        let ziel = wurzel.pfad().join("ziel");

        // Fehlt es, wird es angelegt.
        kopierziel_pruefen(&a, &ziel).expect("ein fehlendes Ziel wird angelegt");
        assert!(ziel.is_dir());
        // Leer geht.
        kopierziel_pruefen(&a, &ziel).expect("ein leeres Ziel ist gueltig");
        // Gefuellt nicht: der Lauf leert es zwischen den Wiederholungen.
        fs::write(ziel.join("fremd.txt"), "fremd").expect("schreibbar");
        let fehler = kopierziel_pruefen(&a, &ziel).expect_err("haette scheitern muessen");
        assert!(
            fehler.to_string().contains("nicht leer"),
            "unerwartete Meldung: {fehler}"
        );
    }

    #[test]
    fn ein_kopierziel_auf_einem_anderen_datentraeger_wird_abgewiesen() {
        // `/dev` ist ein eigenes Dateisystem (devfs) und damit auf jedem
        // macOS ein anderes Geraet als der Wegwerfordner unter /tmp; ein
        // zweiter eingehaengter Datentraeger laesst sich in einer Pruefung
        // nicht voraussetzen.
        let wurzel = Wegwerfordner::neu("fremdes-geraet");
        let ziel = wurzel.pfad().join("ziel");
        fs::create_dir_all(&ziel).expect("Anlegen gescheitert");
        let fehler =
            kopierziel_pruefen(Path::new("/dev"), &ziel).expect_err("haette scheitern muessen");
        assert!(
            fehler.to_string().contains("anderen Datentraeger"),
            "unerwartete Meldung: {fehler}"
        );
    }

    #[test]
    fn der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung() {
        use krk_core::ablage::sitzung::Sitzung;

        let wurzel = Wegwerfordner::neu("messplan");
        let lauf = Gesamtlauf {
            programm: PathBuf::from("/egal/krk"),
            ordner_a: wurzel.pfad().join("a"),
            ordner_b: wurzel.pfad().join("b"),
            ordner100k: wurzel.pfad().join("gross"),
            kopierziel: wurzel.pfad().join("ziel"),
            wiederholungen: 20,
            runden: 1,
        };
        let unterordner = wurzel.pfad().join("a-l6");
        let pfad = plan_schreiben(&lauf, &unterordner).expect("der Plan ist schreibbar");
        let text = fs::read_to_string(&pfad).expect("lesbar");
        let _ = fs::remove_file(&pfad);

        // Der Abschnitt [sitzung] ist ueber dieselbe serde-Struktur lesbar,
        // die session.toml traegt: die eine Serialisierung, kein zweites
        // Format.
        let tabelle: toml::Table = toml::from_str(&text).expect("gueltiges TOML");
        let sitzung: Sitzung = tabelle["sitzung"]
            .clone()
            .try_into()
            .expect("die Pruefsitzung ist eine gewoehnliche Sitzung");
        assert_eq!(sitzung.fenster[0].tabs.len(), 2);
        assert_eq!(sitzung.fenster[0].tabs[0].ordner, lauf.ordner_a);
        assert_eq!(sitzung.fenster[0].tabs[1].ordner, lauf.ordner_b);
        // Rechts umgekehrt: B sichtbar, A dahinter.
        assert_eq!(sitzung.fenster[1].tabs[0].ordner, lauf.ordner_b);
        assert_eq!(sitzung.fenster[1].tabs[1].ordner, lauf.ordner_a);
        assert_eq!(
            tabelle["kopierziel"].as_str(),
            Some(lauf.kopierziel.display().to_string().as_str())
        );
        assert_eq!(
            tabelle["unterordner"].as_str(),
            Some(unterordner.display().to_string().as_str())
        );
    }

    /// Ein vorhandener Sitzungsstand steht nach dem Lauf wieder da.
    #[test]
    fn die_sitzungssicherung_spielt_den_vorigen_stand_zurueck() {
        let ordner = Wegwerfordner::neu("sitzung-zurueck");
        fs::create_dir_all(ordner.pfad()).expect("anlegbar");
        let pfad = ordner.pfad().join("session.toml");
        fs::write(&pfad, b"die Sitzung des Nutzers").expect("schreibbar");

        {
            let _sicherung = Sitzungssicherung::an(pfad.clone()).expect("sicherbar");
            // Der Lauf schreibt die Pruefsitzung darueber.
            fs::write(&pfad, b"die Pruefsitzung").expect("schreibbar");
        }

        assert_eq!(
            fs::read(&pfad).expect("lesbar"),
            b"die Sitzung des Nutzers".to_vec()
        );
    }

    /// Gab es keine Sitzung, ist die Abwesenheit der Stand, der zurueckkommt.
    #[test]
    fn ohne_vorigen_stand_bleibt_keine_pruefsitzung_liegen() {
        let ordner = Wegwerfordner::neu("sitzung-ohne");
        fs::create_dir_all(ordner.pfad()).expect("anlegbar");
        let pfad = ordner.pfad().join("session.toml");

        {
            let _sicherung = Sitzungssicherung::an(pfad.clone()).expect("sicherbar");
            fs::write(&pfad, b"die Pruefsitzung").expect("schreibbar");
        }

        assert!(
            !pfad.exists(),
            "die Pruefsitzung bleibt liegen, wo der Nutzer keine Sitzung hatte"
        );
    }

    /// Der Weg, den der Signalfaden geht, spielt zurueck und dann nichts mehr.
    ///
    /// Ein Signal laesst sich in einem Pruefprozess nicht ausloesen, ohne ihn zu
    /// beenden. Geprueft wird deshalb die Stelle, an der Signalfaden und
    /// [`Sitzungswaechter`] zusammenkommen — beide rufen
    /// [`sitzung_zurueckspielen`], und der zweite Aufruf muss folgenlos sein.
    #[test]
    fn der_signalweg_spielt_zurueck_und_dann_nichts_mehr() {
        let ordner = Wegwerfordner::neu("sitzung-signalweg");
        fs::create_dir_all(ordner.pfad()).expect("anlegbar");
        let pfad = ordner.pfad().join("session.toml");
        fs::write(&pfad, b"die Sitzung des Nutzers").expect("schreibbar");

        let sicherung = Sitzungssicherung::an(pfad.clone()).expect("sicherbar");
        *SICHERUNG.lock().expect("nicht vergiftet") = Some(sicherung);
        fs::write(&pfad, b"die Pruefsitzung").expect("schreibbar");

        // Der erste Aufruf ist der des Signalfadens.
        sitzung_zurueckspielen();
        assert_eq!(
            fs::read(&pfad).expect("lesbar"),
            b"die Sitzung des Nutzers".to_vec()
        );

        // Der zweite ist der des Waechters, der beim Abwickeln hinterherfaellt.
        // Er darf nichts mehr anfassen, sonst ueberschriebe ein spaetes Ende
        // einen Stand, den der Nutzer inzwischen selbst geschrieben hat.
        fs::write(&pfad, b"was der Nutzer inzwischen schrieb").expect("schreibbar");
        sitzung_zurueckspielen();
        assert_eq!(
            fs::read(&pfad).expect("lesbar"),
            b"was der Nutzer inzwischen schrieb".to_vec()
        );
    }

    #[test]
    fn die_systemlast_ist_keine_leere_angabe() {
        let last = systemlast();
        assert!(!last.is_empty());
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
