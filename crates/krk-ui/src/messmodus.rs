//! Der Messmodus: der Ablauf der Fruehmessung, ohne eine Zeile AppKit.
//!
//! Dieses Modul haelt, was kein AppKit beruehrt: welche Messung als naechste
//! dran ist, wann eine begonnene Messung zu Ende ist, die zwanzig
//! Wiederholungen je Groesse und die Ausgabe der Einzelwerte. Die andere Seite
//! der Grenze liegt in [`crate::appkit`]; herueber kommen zwei gewoehnliche
//! Rust-Werte, die Bildwiederholrate als Zahl und die Zeitpunkte der
//! Bildgrenzen. **In dieser Datei steht keine `use objc2`-Zeile**, und das ist
//! nachpruefbar, nicht nur gemeint.
//!
//! # Zwei Aufgaben, weil sie zwei verschiedene Dinge messen
//!
//! - [`Aufgabe::Start`] misst L4 und braucht dafuer **einen Prozessstart je
//!   Wiederholung**. Die Anwendung meldet den Zeitpunkt, an dem die Oberflaeche
//!   bedienbar ist, und beendet sich. Die Spanne selbst zieht der aeussere
//!   Aufrufer, weil nur er den Zeitpunkt vor dem Start kennt.
//! - [`Aufgabe::Spannen`] misst L1, L2, L3 und L10 **innerhalb eines
//!   Prozesses**, weil alle vier Spannen in einer laufenden Anwendung liegen.
//!
//! # Wie eine Spanne hier zustande kommt
//!
//! Jede Messung beginnt an einem Ausloeser und endet an einer Bildgrenze.
//!
//! ```text
//! Ausloeser (Zeitgeber, 97 ms)                Ende (Bildgrenze, ~60/s)
//!   ordner_lesen(A)      ──────────────────>  erste Zeile im Modell   = L2
//!                        ──────────────────>  Lesevorgang beendet     = L3
//!   ordner_lesen(100k)   ──────────────────>  erste Zeile im Modell   = L10
//!   Pfeil ab in die Ereignisschlange ──────>  Auswahl umgesprungen    = L1
//! ```
//!
//! **Warum der Ausloeser an einem eigenen Zeitgeber haengt und nicht an der
//! Bildgrenze.** Loeste die Bildgrenze selbst den Tastendruck aus, laege
//! zwischen Druck und naechster Bildgrenze immer genau ein volles Bild, und L1
//! haette bei 60 Hz konstant 16,7 ms — nicht gemessen, sondern gebaut. Der
//! Zeitgeber laeuft deshalb mit [`AUSLOESETAKT`], einer Spanne, die kein
//! Vielfaches der Bildlaenge ist; ueber zwanzig Wiederholungen wandert der
//! Druckzeitpunkt damit durch das Bild, und das 95. Perzentil sagt etwas.
//!
//! Jede Spanne ist auf ein Bild genau. Das ist keine Ungenauigkeit der
//! Messstrecke, sondern die Sache selbst: vor der naechsten Bildgrenze ist
//! nichts zu sehen.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Wie oft jede Messgroesse wiederholt wird. C8 schreibt zwanzig vor.
pub const WIEDERHOLUNGEN: usize = 20;

/// Die Spanne zwischen zwei Ausloesern, in Sekunden.
///
/// 97 ms sind bei 60 Hz 5,82 Bilder und bei 120 Hz 11,64. Beide Male kein
/// ganzes Vielfaches, damit der Ausloesezeitpunkt durch das Bild wandert statt
/// an einer Stelle zu kleben. Zugleich lang genug, dass ein Lesevorgang auf dem
/// Ordner mit 10.000 Eintraegen dazwischen fertig wird.
pub const AUSLOESETAKT: f64 = 0.097;

/// Wie lange eine begonnene Messung hoechstens dauern darf.
///
/// **Die Uhr und nicht der Bildzaehler.** Die erste Fassung dieser Schranke
/// zaehlte Bildgrenzen, und genau daran ist sie am 260803 gescheitert: der
/// `CADisplayLink` hoerte mitten in einer Messreihe auf zu takten, damit zaehlte
/// nichts mehr, und der Lauf stand still, bis der aeussere Aufrufer ihn nach
/// fuenf Minuten abschoss. Eine Schranke, die dasselbe Ereignis zaehlt, dessen
/// Ausbleiben sie abfangen soll, kann nicht greifen. Der Ausloesetakt laeuft
/// unabhaengig vom Bildtakt und prueft die Uhr.
const GEDULD: Duration = Duration::from_secs(10);

/// Die Meldung, mit der ein Messlauf ohne Bildschirm abbricht.
pub const OHNE_BILDSCHIRM: &str = "das Fenster steht auf keinem Bildschirm, \
     die Bildwiederholrate ist damit nicht erhebbar. Es wird keine Zahl \
     ausgegeben; auf den Hauptbildschirm weicht die Messung nicht aus.";

/// Die Befehlszeilenmarke, die den Messmodus einschaltet.
const MARKE: &str = "--messmodus";

/// Was der Messmodus zu tun hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Aufgabe {
    /// L4: melden, wann die Oberflaeche bedienbar ist, dann beenden.
    Start {
        /// Der Ordner, den das Fenster beim Start zeigt.
        ordner: PathBuf,
    },
    /// L1, L2, L3 und L10 an der laufenden Anwendung.
    Spannen {
        /// Pruefordner A mit 10.000 Eintraegen, fuer L1, L2 und L3.
        ordner_a: PathBuf,
        /// Der Ordner mit 100.000 Eintraegen, fuer L10.
        ordner100k: PathBuf,
    },
}

impl Aufgabe {
    /// Liest die Aufgabe aus der Befehlszeile.
    ///
    /// Liefert `Ok(None)`, wenn `--messmodus` gar nicht vorkommt: das ist der
    /// gewoehnliche Start und kein Fehler. Unbekannte Marken werden dabei
    /// uebergangen, weil LaunchServices einem ueber den Finder gestarteten
    /// Buendel eigene anhaengt.
    pub fn aus_argumenten(argumente: &[String]) -> Result<Option<Self>, String> {
        let Some(stelle) = argumente.iter().position(|marke| marke == MARKE) else {
            return Ok(None);
        };
        let art = argumente
            .get(stelle + 1)
            .ok_or_else(|| format!("{MARKE} braucht eine Aufgabe: start oder spannen"))?;

        match art.as_str() {
            "start" => Ok(Some(Aufgabe::Start {
                ordner: pfad(argumente, "--ordner")?,
            })),
            "spannen" => Ok(Some(Aufgabe::Spannen {
                ordner_a: pfad(argumente, "--ordner-a")?,
                ordner100k: pfad(argumente, "--ordner100k")?,
            })),
            andere => Err(format!(
                "{MARKE} kennt die Aufgabe {andere:?} nicht; es gibt start und spannen"
            )),
        }
    }

    /// Der Ordner, den das Fenster beim Start zeigt.
    pub fn startordner(&self) -> &Path {
        match self {
            Aufgabe::Start { ordner } => ordner,
            Aufgabe::Spannen { ordner_a, .. } => ordner_a,
        }
    }
}

/// Holt den Wert einer benannten Marke aus der Befehlszeile.
fn pfad(argumente: &[String], marke: &str) -> Result<PathBuf, String> {
    let stelle = argumente
        .iter()
        .position(|wort| wort == marke)
        .ok_or_else(|| format!("{marke} fehlt"))?;
    argumente
        .get(stelle + 1)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{marke} braucht einen Pfad"))
}

/// Was die Oberflaeche an einer Bildgrenze ueber sich sagt.
///
/// Drei Zahlen, kein AppKit-Wert. Sie sind alles, was der Ablauf braucht, um zu
/// erkennen, ob eine begonnene Messung zu Ende ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zustand {
    /// Wie viele Zeilen das Ordnermodell gerade traegt.
    pub zeilen: usize,
    /// Ob noch ein Lesevorgang laeuft.
    pub liest: bool,
    /// Welche Zeile ausgewaehlt ist; -1, wenn keine.
    pub auswahl: isize,
}

/// Was die Oberflaeche als naechstes tun soll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Anweisung {
    /// Nichts; eine Messung laeuft noch.
    Warten,
    /// Den genannten Ordner lesen.
    Lesen(PathBuf),
    /// Einen Tastendruck in die eigene Ereignisschlange stellen.
    Taste,
    /// Alles gemessen; ausgeben und beenden.
    Fertig,
    /// Abbrechen mit dieser Meldung.
    Abbruch(String),
}

/// Ein Schritt des Ablaufs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Schritt {
    /// Ein ungezaehlter Lesevorgang, der den Cache waermt.
    ///
    /// C8 sagt: "warm" heisst jeder **weitere** Zugriff. Ohne diesen Vorlauf
    /// truege der erste von zwanzig Laeufen eine kalte Zahl in eine warme Reihe.
    Vorlauf(PathBuf),
    /// Pruefordner A lesen und L2 sowie L3 zaehlen.
    LesenA,
    /// Den Ordner mit 100.000 Eintraegen lesen und L10 zaehlen.
    Lesen100k,
    /// Einen Tastendruck absetzen und L1 zaehlen.
    Taste,
}

/// Wie eine laufende Messung gezaehlt wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zaehlung {
    /// Gar nicht: ein Vorlauf.
    Keine,
    /// Als L2 und L3.
    A,
    /// Als L10.
    Gross,
}

/// Was gerade laeuft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lage {
    /// Nichts; der naechste Schritt darf beginnen.
    Bereit,
    /// Ein Lesevorgang laeuft.
    Liest {
        t0: Instant,
        zaehlung: Zaehlung,
        erste_seite: Option<Duration>,
        bilder: u32,
    },
    /// Ein Tastendruck ist unterwegs.
    Taste {
        t0: Instant,
        auswahl_vorher: isize,
        bilder: u32,
    },
}

/// Die gesammelten Einzelwerte.
#[derive(Debug, Clone, Default)]
struct Werte {
    /// L1: Tastendruck bis Bildgrenze mit umgesprungener Auswahl.
    l1: Vec<Duration>,
    /// L2: Lesebeginn bis Bildgrenze mit erster Bildschirmseite, Ordner A.
    l2: Vec<Duration>,
    /// L3: Lesebeginn bis Bildgrenze mit vollstaendig gelesenem Ordner A.
    l3: Vec<Duration>,
    /// L10: wie L2, auf dem Ordner mit 100.000 Eintraegen.
    l10_erste: Vec<Duration>,
    /// Das vollstaendige Lesen desselben Ordners. C8 sagt dafuer 4 s warm zu;
    /// das Gate von Schritt 8 fragt die Zahl nicht ab, der Bericht nennt sie.
    l10_voll: Vec<Duration>,
}

/// Ein laufender Messlauf.
pub struct Messlauf {
    aufgabe: Aufgabe,
    schritte: Vec<Schritt>,
    stelle: usize,
    lage: Lage,
    werte: Werte,
    bildwiederholrate: Option<isize>,
    /// Nur fuer [`Aufgabe::Start`]: ob der Zeitpunkt schon gemeldet ist.
    gemeldet: bool,
}

impl Messlauf {
    /// Legt den Ablauf zur genannten Aufgabe an.
    pub fn neu(aufgabe: Aufgabe) -> Self {
        let schritte = match &aufgabe {
            Aufgabe::Start { .. } => Vec::new(),
            Aufgabe::Spannen {
                ordner_a,
                ordner100k,
            } => {
                let mut schritte = Vec::with_capacity(3 * WIEDERHOLUNGEN + 3);
                // Der Ordner, der beim Start ohnehin gelesen wird, ist noch
                // kein Vorlauf: er laeuft, bevor der Messlauf steht.
                schritte.push(Schritt::Vorlauf(ordner_a.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::LesenA, WIEDERHOLUNGEN));
                schritte.push(Schritt::Vorlauf(ordner100k.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::Lesen100k, WIEDERHOLUNGEN));
                // Vor den Tastendruecken zurueck auf den kleinen Ordner: L1
                // misst die Auswahlbewegung in einer stehenden Liste, nicht
                // waehrend eines Lesevorgangs.
                schritte.push(Schritt::Vorlauf(ordner_a.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::Taste, WIEDERHOLUNGEN));
                schritte
            }
        };
        Self {
            aufgabe,
            schritte,
            stelle: 0,
            lage: Lage::Bereit,
            werte: Werte::default(),
            bildwiederholrate: None,
            gemeldet: false,
        }
    }

    /// Haelt die Bildwiederholrate fest, die die Oberflaeche ausgelesen hat.
    pub fn rate_setzen(&mut self, hertz: isize) {
        self.bildwiederholrate = Some(hertz);
    }

    /// Fragt, was als naechstes zu tun ist.
    ///
    /// Wird vom Ausloesetakt gerufen. Der Zeitpunkt, ab dem gemessen wird, ist
    /// der dieses Aufrufs und damit **vor** dem AppKit-Aufruf, den der Aufrufer
    /// gleich absetzt.
    pub fn naechster_schritt(&mut self, zustand: Zustand) -> Anweisung {
        // Die Startaufgabe hat keinen Ablauf: sie wartet auf die eine
        // Bildgrenze, an der die erste Bildschirmseite steht. Ohne diese Zeile
        // faende der Ausloesetakt eine leere Schrittliste vor und meldete
        // `Fertig`, sobald er vor der ersten Bildgrenze drankaeme — ein Rennen,
        // das ein langsamer Startordner gewinnt und das dann eine Messung ohne
        // Zahl ausgaebe.
        if matches!(self.aufgabe, Aufgabe::Start { .. }) {
            return Anweisung::Warten;
        }
        if let Some(grund) = self.haengt() {
            return Anweisung::Abbruch(grund);
        }
        if self.lage != Lage::Bereit {
            return Anweisung::Warten;
        }
        let Some(schritt) = self.schritte.get(self.stelle).cloned() else {
            return Anweisung::Fertig;
        };
        match schritt {
            Schritt::Vorlauf(pfad) => {
                self.lage = Lage::Liest {
                    t0: Instant::now(),
                    zaehlung: Zaehlung::Keine,
                    erste_seite: None,
                    bilder: 0,
                };
                Anweisung::Lesen(pfad)
            }
            Schritt::LesenA | Schritt::Lesen100k => {
                let (zaehlung, pfad) = match schritt {
                    Schritt::LesenA => (Zaehlung::A, self.ordner_a().to_path_buf()),
                    _ => (Zaehlung::Gross, self.ordner100k().to_path_buf()),
                };
                self.lage = Lage::Liest {
                    t0: Instant::now(),
                    zaehlung,
                    erste_seite: None,
                    bilder: 0,
                };
                Anweisung::Lesen(pfad)
            }
            Schritt::Taste => {
                if zustand.zeilen == 0 {
                    return Anweisung::Abbruch(
                        "die Liste ist leer; ein Tastendruck kann keine Auswahl bewegen".to_owned(),
                    );
                }
                self.lage = Lage::Taste {
                    t0: Instant::now(),
                    auswahl_vorher: zustand.auswahl,
                    bilder: 0,
                };
                Anweisung::Taste
            }
        }
    }

    /// Ob die laufende Messung ueber ihre Geduld hinaus ist.
    ///
    /// Geprueft wird an der Uhr und nicht am Bildzaehler; die Zahl der bisher
    /// eingegangenen Bildgrenzen steht trotzdem in der Meldung, weil sie die
    /// beiden Faelle trennt. Null Bildgrenzen heisst: der `CADisplayLink`
    /// taktet nicht, etwa weil das Fenster verdeckt ist. Viele Bildgrenzen
    /// heissen: die Oberflaeche taktet, kommt aber nicht ans Ziel.
    fn haengt(&self) -> Option<String> {
        let (t0, bilder, was) = match self.lage {
            Lage::Bereit => return None,
            Lage::Liest { t0, bilder, .. } => (t0, bilder, "ein Lesevorgang"),
            Lage::Taste { t0, bilder, .. } => (t0, bilder, "ein Tastendruck"),
        };
        if t0.elapsed() <= GEDULD {
            return None;
        }
        Some(format!(
            "{was} ist nach {} s nicht am Ziel; seit dem Beginn sind {bilder} Bildgrenzen \
             eingegangen{}",
            GEDULD.as_secs(),
            if bilder == 0 {
                ". Der Bildtakt steht, das Fenster ist vermutlich verdeckt oder der \
                 Bildschirm aus"
            } else {
                ""
            }
        ))
    }

    /// Meldet eine Bildgrenze samt dem Zustand der Oberflaeche.
    ///
    /// Liefert `true`, sobald der Lauf beendet ist und der Aufrufer die
    /// Anwendung verlassen soll.
    pub fn bildgrenze(&mut self, jetzt: Instant, zustand: Zustand) -> bool {
        if matches!(self.aufgabe, Aufgabe::Start { .. }) {
            return self.bildgrenze_beim_start(zustand);
        }

        // Die Lage wird kopiert und am Ende zurueckgeschrieben. So bleibt sie
        // nicht ausgeliehen, waehrend die Werte wachsen und der Schritt
        // weiterrueckt.
        match self.lage {
            Lage::Bereit => false,
            Lage::Liest {
                t0,
                zaehlung,
                mut erste_seite,
                bilder,
            } => {
                if erste_seite.is_none() && zustand.zeilen > 0 {
                    erste_seite = Some(jetzt.saturating_duration_since(t0));
                }
                if zustand.liest {
                    self.lage = Lage::Liest {
                        t0,
                        zaehlung,
                        erste_seite,
                        bilder: bilder + 1,
                    };
                    return false;
                }
                let vollstaendig = jetzt.saturating_duration_since(t0);
                let erste = erste_seite.unwrap_or(vollstaendig);
                match zaehlung {
                    Zaehlung::Keine => {}
                    Zaehlung::A => {
                        self.werte.l2.push(erste);
                        self.werte.l3.push(vollstaendig);
                    }
                    Zaehlung::Gross => {
                        self.werte.l10_erste.push(erste);
                        self.werte.l10_voll.push(vollstaendig);
                    }
                }
                self.weiter();
                false
            }
            Lage::Taste {
                t0,
                auswahl_vorher,
                bilder,
            } => {
                if zustand.auswahl == auswahl_vorher {
                    self.lage = Lage::Taste {
                        t0,
                        auswahl_vorher,
                        bilder: bilder + 1,
                    };
                    return false;
                }
                self.werte.l1.push(jetzt.saturating_duration_since(t0));
                self.weiter();
                false
            }
        }
    }

    /// Die Bildgrenze in der Aufgabe [`Aufgabe::Start`].
    ///
    /// Bedienbar heisst hier: das Fenster steht, der Tastenabgriff ist
    /// eingerichtet, und die erste Bildschirmseite des Startordners ist im
    /// Modell, sichtbar spaetestens mit dieser Bildgrenze. Genau diese Lesart
    /// hat der Nutzer am 260802-1735 entschieden.
    fn bildgrenze_beim_start(&mut self, zustand: Zustand) -> bool {
        if self.gemeldet || zustand.zeilen == 0 {
            return false;
        }
        self.gemeldet = true;
        let jetzt = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        self.rate_ausgeben();
        melden(&format!("bedienbar {}", jetzt.as_nanos()));
        true
    }

    /// Ein Schritt ist erledigt.
    fn weiter(&mut self) {
        self.stelle += 1;
        self.lage = Lage::Bereit;
    }

    fn ordner_a(&self) -> &Path {
        match &self.aufgabe {
            Aufgabe::Spannen { ordner_a, .. } => ordner_a,
            Aufgabe::Start { ordner } => ordner,
        }
    }

    fn ordner100k(&self) -> &Path {
        match &self.aufgabe {
            Aufgabe::Spannen { ordner100k, .. } => ordner100k,
            Aufgabe::Start { ordner } => ordner,
        }
    }

    /// Schreibt die Bildwiederholrate auf die Standardausgabe.
    fn rate_ausgeben(&self) {
        match self.bildwiederholrate {
            Some(hertz) => melden(&format!("bildwiederholrate {hertz}")),
            // Erreichbar ist das nicht: ohne Rate bricht der Aufrufer ab, bevor
            // ein Messlauf beginnt. Eine erfundene 60 kaeme trotzdem nicht in
            // Frage.
            None => melden("bildwiederholrate unbekannt"),
        }
    }

    /// Schreibt jeden gemessenen Einzelwert auf die Standardausgabe.
    ///
    /// Ausgewertet wird woanders. Der Grund ist die Aufteilung der Strecke:
    /// L4 misst der aeussere Aufrufer ueber zwanzig Prozessstarts, L1 bis L10
    /// misst dieser Prozess, und **ein** Bericht ueber alle fuenf Zusagen
    /// entsteht nur, wo beide Haelften zusammenkommen. Das Perzentil und der
    /// Berichtskopf liegen deshalb in `krk-bench`, das beides schon fuehrt; ein
    /// zweiter Berichtsschreiber daneben waere eine zweite Wahrheit.
    pub fn ausgeben(&self) {
        self.rate_ausgeben();
        for (name, werte) in [
            ("l1", &self.werte.l1),
            ("l2", &self.werte.l2),
            ("l3", &self.werte.l3),
            ("l10-erste", &self.werte.l10_erste),
            ("l10-voll", &self.werte.l10_voll),
        ] {
            for wert in werte {
                melden(&format!("wert {name} {}", wert.as_nanos()));
            }
        }
        melden("fertig");
        let _ = std::io::stdout().flush();
    }
}

/// Das Praefix, an dem der aeussere Aufrufer eine Messzeile erkennt.
///
/// Die Anwendung schreibt auch anderes auf die Standardausgabe, etwa das
/// Tastenprotokoll. Ein Praefix trennt das Maschinenlesbare vom Uebrigen,
/// statt den Leser raten zu lassen.
pub const PRAEFIX: &str = "krk-messung";

fn melden(zeile: &str) {
    println!("{PRAEFIX} {zeile}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worte(zeile: &[&str]) -> Vec<String> {
        zeile.iter().map(|wort| (*wort).to_owned()).collect()
    }

    #[test]
    fn ohne_die_marke_gibt_es_keine_aufgabe() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&["--tasten-protokoll"])),
            Ok(None)
        );
        assert_eq!(Aufgabe::aus_argumenten(&[]), Ok(None));
    }

    #[test]
    fn die_startaufgabe_braucht_ihren_ordner() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "start", "--ordner", "/tmp/a"])),
            Ok(Some(Aufgabe::Start {
                ordner: PathBuf::from("/tmp/a")
            }))
        );
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus", "start"])).is_err());
    }

    #[test]
    fn die_spannenaufgabe_braucht_beide_ordner() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&[
                "--messmodus",
                "spannen",
                "--ordner-a",
                "/tmp/a",
                "--ordner100k",
                "/tmp/gross"
            ])),
            Ok(Some(Aufgabe::Spannen {
                ordner_a: PathBuf::from("/tmp/a"),
                ordner100k: PathBuf::from("/tmp/gross")
            }))
        );
        assert!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "spannen", "--ordner-a", "/tmp/a"]))
                .is_err()
        );
    }

    #[test]
    fn eine_unbekannte_aufgabe_ist_ein_fehler() {
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus", "alles"])).is_err());
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus"])).is_err());
    }

    fn spannenlauf() -> Messlauf {
        Messlauf::neu(Aufgabe::Spannen {
            ordner_a: PathBuf::from("/tmp/a"),
            ordner100k: PathBuf::from("/tmp/gross"),
        })
    }

    #[test]
    fn der_ablauf_traegt_drei_vorlaeufe_und_dreimal_zwanzig_messungen() {
        let lauf = spannenlauf();
        let zahl = |gesucht: &Schritt| lauf.schritte.iter().filter(|s| *s == gesucht).count();
        assert_eq!(zahl(&Schritt::LesenA), WIEDERHOLUNGEN);
        assert_eq!(zahl(&Schritt::Lesen100k), WIEDERHOLUNGEN);
        assert_eq!(zahl(&Schritt::Taste), WIEDERHOLUNGEN);
        assert_eq!(lauf.schritte.len(), 3 * WIEDERHOLUNGEN + 3);
        assert_eq!(
            lauf.schritte.first(),
            Some(&Schritt::Vorlauf(PathBuf::from("/tmp/a"))),
            "ohne Vorlauf traegt der erste Lauf eine kalte Zahl in eine warme Reihe"
        );
    }

    /// Ein Lesevorgang von Anfang bis Ende, Bildgrenze fuer Bildgrenze.
    #[test]
    fn ein_lesevorgang_liefert_erste_seite_und_vollstaendiges_lesen() {
        let mut lauf = spannenlauf();
        // Den Vorlauf abarbeiten.
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        lauf.bildgrenze(Instant::now(), fertig(10));
        assert_eq!(lauf.stelle, 1);

        // Die erste gezaehlte Messung.
        assert!(matches!(
            lauf.naechster_schritt(fertig(10)),
            Anweisung::Lesen(_)
        ));
        let start = Instant::now();
        // Bild 1: noch keine Zeile.
        assert!(!lauf.bildgrenze(start + ms(16), leer_und_liest()));
        // Bild 2: erste Zeilen stehen, es wird noch gelesen.
        assert!(!lauf.bildgrenze(start + ms(33), liest_mit(500)));
        // Bild 3: fertig.
        assert!(!lauf.bildgrenze(start + ms(66), fertig(10_000)));

        assert_eq!(lauf.werte.l2.len(), 1);
        assert_eq!(lauf.werte.l3.len(), 1);
        assert!(
            lauf.werte.l2[0] < lauf.werte.l3[0],
            "die erste Seite kam nach dem vollstaendigen Lesen"
        );
        assert_eq!(lauf.lage, Lage::Bereit);
    }

    #[test]
    fn eine_laufende_messung_nimmt_keinen_neuen_schritt_an() {
        let mut lauf = spannenlauf();
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        assert_eq!(lauf.naechster_schritt(liest_mit(5)), Anweisung::Warten);
    }

    #[test]
    fn ein_tastendruck_zaehlt_erst_wenn_die_auswahl_umspringt() {
        let mut lauf = spannenlauf();
        // Bis zum ersten Tastenschritt vorspulen.
        lauf.stelle = lauf.schritte.len() - WIEDERHOLUNGEN;
        assert_eq!(lauf.naechster_schritt(fertig(10_000)), Anweisung::Taste);
        let start = Instant::now();
        assert!(!lauf.bildgrenze(start + ms(8), fertig(10_000)));
        assert_eq!(lauf.werte.l1.len(), 0, "ohne Umsprung darf nichts zaehlen");
        assert!(!lauf.bildgrenze(start + ms(16), ausgewaehlt(0)));
        assert_eq!(lauf.werte.l1.len(), 1);
    }

    #[test]
    fn ein_leerer_ordner_laesst_keinen_tastendruck_zu() {
        let mut lauf = spannenlauf();
        lauf.stelle = lauf.schritte.len() - WIEDERHOLUNGEN;
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Abbruch(_)
        ));
    }

    #[test]
    fn nach_dem_letzten_schritt_ist_der_lauf_fertig() {
        let mut lauf = spannenlauf();
        lauf.stelle = lauf.schritte.len();
        assert_eq!(lauf.naechster_schritt(fertig(10)), Anweisung::Fertig);
    }

    #[test]
    fn die_startaufgabe_meldet_genau_einmal() {
        let mut lauf = Messlauf::neu(Aufgabe::Start {
            ordner: PathBuf::from("/tmp/a"),
        });
        lauf.rate_setzen(60);
        assert!(!lauf.bildgrenze(Instant::now(), leer()));
        assert!(lauf.bildgrenze(Instant::now(), fertig(10)));
        assert!(!lauf.bildgrenze(Instant::now(), fertig(10)));
    }

    /// Der Ausloesetakt darf der Startaufgabe nicht zuvorkommen.
    #[test]
    fn die_startaufgabe_wartet_auf_die_bildgrenze() {
        let mut lauf = Messlauf::neu(Aufgabe::Start {
            ordner: PathBuf::from("/tmp/a"),
        });
        for _ in 0..3 {
            assert_eq!(
                lauf.naechster_schritt(leer()),
                Anweisung::Warten,
                "der Ausloesetakt hat die Startaufgabe vorzeitig beendet"
            );
        }
    }

    #[test]
    fn eine_stehende_messung_wird_an_der_uhr_abgebrochen_und_nicht_am_bildzaehler() {
        let mut lauf = spannenlauf();
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        // Noch keine einzige Bildgrenze eingegangen, und trotzdem greift die
        // Schranke, sobald die Geduld um ist.
        assert_eq!(lauf.haengt(), None);
        lauf.lage = Lage::Liest {
            t0: Instant::now() - GEDULD - Duration::from_secs(1),
            zaehlung: Zaehlung::A,
            erste_seite: None,
            bilder: 0,
        };
        let grund = lauf.haengt().expect("die Schranke haette greifen muessen");
        assert!(
            grund.contains("0 Bildgrenzen") && grund.contains("Bildtakt steht"),
            "die Meldung trennt die beiden Faelle nicht: {grund}"
        );
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Abbruch(_)
        ));
    }

    fn ms(zahl: u64) -> Duration {
        Duration::from_millis(zahl)
    }

    fn leer() -> Zustand {
        Zustand {
            zeilen: 0,
            liest: false,
            auswahl: -1,
        }
    }

    fn leer_und_liest() -> Zustand {
        Zustand {
            zeilen: 0,
            liest: true,
            auswahl: -1,
        }
    }

    fn liest_mit(zeilen: usize) -> Zustand {
        Zustand {
            zeilen,
            liest: true,
            auswahl: -1,
        }
    }

    fn fertig(zeilen: usize) -> Zustand {
        Zustand {
            zeilen,
            liest: false,
            auswahl: -1,
        }
    }

    fn ausgewaehlt(zeile: isize) -> Zustand {
        Zustand {
            zeilen: 10_000,
            liest: false,
            auswahl: zeile,
        }
    }
}
