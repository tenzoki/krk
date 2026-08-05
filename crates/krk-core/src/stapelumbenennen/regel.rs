//! Das Regelmodell: Suchen und Ersetzen im Namen, dazu eine fortlaufende
//! Nummerierung (C4).
//!
//! Zwei Regelarten, in dieser Reihenfolge angewandt:
//!
//! ```text
//!  "IMG_4711.jpg"  ──Suchen/Ersetzen──> "Urlaub .jpg"  ──Nummer──> "Urlaub 007.jpg"
//!                    ("IMG_4711" → "Urlaub ")            (ab 7, 3 Stellen)
//! ```
//!
//! **Die Nummer haengt an den Stamm und nicht an den ganzen Namen.** Eine
//! Kopie, die `foto.jpg007` hiesse, waere keine Bilddatei mehr. Getrennt wird
//! am letzten Punkt: `archiv.tar.gz` hat den Stamm `archiv.tar`, und
//! `.gitignore` ist ein Stamm ohne Endung.
//!
//! **Die Trennung steht nicht hier.** Sie steht in
//! [`crate::operation::umbenennen::namen_teilen`], zusammen mit der
//! Namenspruefung und dem freien Namen, und dieses Modul ruft sie. Bis zum
//! 260805 zog es sie ein zweites Mal, ueber `Path::file_stem` und
//! `Path::extension`; beide lieferten dasselbe, und das ist die Lage, in der
//! eine Abweichung spaeter unbemerkt entsteht
//! (`issues/260804-2040_c_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen.md`).
//!
//! **Ein Trennzeichen vor der Nummer setzt KRK nicht.** Wer `Urlaub 007.jpg`
//! will, ersetzt nach `Urlaub ` mit dem Leerzeichen am Ende; wer `Urlaub007.jpg`
//! will, laesst es weg. Ein festes Trennzeichen waere eine Entscheidung, die die
//! Regel dem Nutzer abnimmt, ohne dass er sie zuruecknehmen koennte.

use std::fmt;

use crate::operation::umbenennen::namen_teilen;

/// Wie viele Stellen eine fortlaufende Nummer hoechstens tragen darf.
///
/// Neun Stellen fassen jede Zahl, die in eine `u32` passt. Die Grenze steht
/// nicht gegen einen erwarteten Fall, sondern gegen den Vertipper, der aus
/// einer Stellenzahl eine Zeichenkette von Millionen Nullen macht.
pub const HOECHSTE_STELLENZAHL: u8 = 9;

/// Die Stellenzahl, die eine Nummerierung ohne eigene Angabe bekommt.
///
/// Eine Stelle heisst: keine fuehrenden Nullen. Das ist der Zustand, den der
/// Nutzer sieht, wenn er nur einen Startwert eintippt.
const VORGABE_STELLEN: u8 = 1;

/// Eine fortlaufende Nummerierung mit wählbarer Stellenzahl und wählbarem
/// Startwert (C4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nummerierung {
    /// Die Nummer des ersten Eintrags in der Sichtreihenfolge.
    pub start: u32,
    /// Wie viele Stellen die Nummer traegt, mit fuehrenden Nullen aufgefuellt.
    ///
    /// Eine laengere Zahl wird **nicht** abgeschnitten: aus 1.000 wird bei zwei
    /// Stellen `1000` und nicht `00`. Eine abgeschnittene Nummer waere ein
    /// stiller Namenskonflikt.
    pub stellen: u8,
}

impl Nummerierung {
    /// Eine Nummerierung ab diesem Startwert mit dieser Stellenzahl.
    pub fn neu(start: u32, stellen: u8) -> Self {
        Self {
            start,
            stellen: stellen.clamp(1, HOECHSTE_STELLENZAHL),
        }
    }

    /// Die Nummer des Eintrags an dieser Stelle des Stapels, ausgeschrieben.
    ///
    /// `lauf` zaehlt ab 0 in Sichtreihenfolge. Der Ueberlauf saettigt: ein
    /// Startwert nahe der Obergrenze von `u32` soll den Stapel nicht in einen
    /// Absturz laufen lassen.
    pub fn ziffern(self, lauf: u32) -> String {
        let breite = self.stellen as usize;
        format!("{:0breite$}", self.start.saturating_add(lauf))
    }
}

/// Eine Regel fuer das Umbenennen im Stapel (C4).
///
/// Beide Teile sind wahlfrei. Eine Regel ohne Suchtext und ohne Nummerierung
/// laesst jeden Namen, wie er ist; die Vorschau zeigt das, und die Ausfuehrung
/// benennt nichts um.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Regel {
    /// Was im Namen gesucht wird. Leer heisst: nichts ersetzen.
    pub suchen: String,
    /// Wodurch der Suchtext ersetzt wird. Leer heisst: streichen.
    pub ersetzen: String,
    /// Die fortlaufende Nummerierung, falls eine gewuenscht ist.
    pub nummerierung: Option<Nummerierung>,
}

impl Regel {
    /// Ob die Regel jeden Namen unveraendert liesse.
    pub fn ist_wirkungslos(&self) -> bool {
        self.suchen.is_empty() && self.nummerierung.is_none()
    }

    /// Der neue Name des Eintrags an der Stelle `lauf` des Stapels.
    ///
    /// `lauf` zaehlt ab 0 in Sichtreihenfolge. Gesucht wird ueber den ganzen
    /// Namen einschliesslich der Endung: wer `.jpeg` durch `.jpg` ersetzen
    /// will, soll das tun koennen.
    pub fn anwenden(&self, name: &str, lauf: u32) -> String {
        let ersetzt = if self.suchen.is_empty() {
            name.to_owned()
        } else {
            name.replace(&self.suchen, &self.ersetzen)
        };
        let Some(nummerierung) = self.nummerierung else {
            return ersetzt;
        };
        let (stamm, endung) = namen_teilen(&ersetzt);
        format!("{stamm}{}{endung}", nummerierung.ziffern(lauf))
    }

    /// Baut eine Regel aus den vier Eingabefeldern des Blattes.
    ///
    /// Ein leeres Feld "Nummer ab" heisst: keine Nummerierung, und dann bleibt
    /// die Stellenzahl unbeachtet. Eine unlesbare Zahl ist **kein** stiller
    /// Rueckfall auf "keine Nummerierung", sondern ein Fehler mit Grund: sonst
    /// tippte der Nutzer `7a` und saehe eine Vorschau ohne Nummern, ohne zu
    /// erfahren warum.
    pub fn aus_eingabe(
        suchen: &str,
        ersetzen: &str,
        nummer_ab: &str,
        stellen: &str,
    ) -> Result<Self, Regelfehler> {
        let nummer_ab = nummer_ab.trim();
        let stellen = stellen.trim();
        let nummerierung = if nummer_ab.is_empty() {
            None
        } else {
            let start = nummer_ab
                .parse::<u32>()
                .map_err(|_| Regelfehler::Startwert(nummer_ab.to_owned()))?;
            let stellenzahl = if stellen.is_empty() {
                VORGABE_STELLEN
            } else {
                let gelesen = stellen
                    .parse::<u8>()
                    .map_err(|_| Regelfehler::Stellenzahl(stellen.to_owned()))?;
                if gelesen == 0 || gelesen > HOECHSTE_STELLENZAHL {
                    return Err(Regelfehler::Stellenzahl(stellen.to_owned()));
                }
                gelesen
            };
            Some(Nummerierung::neu(start, stellenzahl))
        };
        Ok(Self {
            suchen: suchen.to_owned(),
            ersetzen: ersetzen.to_owned(),
            nummerierung,
        })
    }
}

/// Warum sich aus den Eingabefeldern keine Regel bauen laesst.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Regelfehler {
    /// Der Startwert der Nummerierung ist keine Zahl.
    Startwert(String),
    /// Die Stellenzahl ist keine Zahl oder liegt ausserhalb des Bereichs.
    Stellenzahl(String),
}

impl fmt::Display for Regelfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regelfehler::Startwert(text) => {
                write!(ausgabe, "„{text}“ ist kein Startwert für die Nummerierung")
            }
            Regelfehler::Stellenzahl(text) => write!(
                ausgabe,
                "„{text}“ ist keine Stellenzahl zwischen 1 und {HOECHSTE_STELLENZAHL}"
            ),
        }
    }
}

impl std::error::Error for Regelfehler {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eine_regel_ohne_suchtext_und_ohne_nummer_laesst_alles_stehen() {
        let regel = Regel::default();
        assert!(regel.ist_wirkungslos());
        assert_eq!(regel.anwenden("bericht.txt", 0), "bericht.txt");
    }

    #[test]
    fn suchen_und_ersetzen_wirkt_auf_den_ganzen_namen() {
        let regel = Regel {
            suchen: ".jpeg".to_owned(),
            ersetzen: ".jpg".to_owned(),
            nummerierung: None,
        };
        assert_eq!(regel.anwenden("bild.jpeg", 0), "bild.jpg");
    }

    #[test]
    fn die_nummer_haengt_an_den_stamm_und_nicht_an_die_endung() {
        let regel = Regel {
            suchen: "IMG_4711".to_owned(),
            ersetzen: "Urlaub ".to_owned(),
            nummerierung: Some(Nummerierung::neu(7, 3)),
        };
        assert_eq!(regel.anwenden("IMG_4711.jpg", 0), "Urlaub 007.jpg");
        assert_eq!(regel.anwenden("IMG_4711.jpg", 3), "Urlaub 010.jpg");
    }

    #[test]
    fn ein_name_ohne_endung_bekommt_die_nummer_hinten() {
        let regel = Regel {
            suchen: String::new(),
            ersetzen: String::new(),
            nummerierung: Some(Nummerierung::neu(1, 2)),
        };
        assert_eq!(regel.anwenden("liesmich", 0), "liesmich01");
        assert_eq!(regel.anwenden(".gitignore", 0), ".gitignore01");
        assert_eq!(regel.anwenden("archiv.tar.gz", 0), "archiv.tar01.gz");
    }

    #[test]
    fn eine_zu_lange_nummer_wird_nicht_abgeschnitten() {
        let nummerierung = Nummerierung::neu(1_000, 2);
        assert_eq!(nummerierung.ziffern(0), "1000");
    }

    #[test]
    fn der_ueberlauf_saettigt_statt_umzulaufen() {
        let nummerierung = Nummerierung::neu(u32::MAX, 1);
        assert_eq!(nummerierung.ziffern(5), u32::MAX.to_string());
    }

    #[test]
    fn ein_leeres_feld_nummer_ab_heisst_keine_nummerierung() {
        let regel = Regel::aus_eingabe("a", "b", "  ", "3").expect("kein Fehler");
        assert_eq!(regel.nummerierung, None);
    }

    #[test]
    fn ein_unlesbarer_startwert_meldet_sich_statt_still_zu_verschwinden() {
        assert_eq!(
            Regel::aus_eingabe("", "", "7a", ""),
            Err(Regelfehler::Startwert("7a".to_owned()))
        );
        assert_eq!(
            Regel::aus_eingabe("", "", "7", "0"),
            Err(Regelfehler::Stellenzahl("0".to_owned()))
        );
        assert_eq!(
            Regel::aus_eingabe("", "", "7", "99"),
            Err(Regelfehler::Stellenzahl("99".to_owned()))
        );
    }

    #[test]
    fn ohne_stellenzahl_gilt_eine_stelle() {
        let regel = Regel::aus_eingabe("", "", "7", "").expect("kein Fehler");
        assert_eq!(regel.nummerierung, Some(Nummerierung::neu(7, 1)));
        assert_eq!(regel.anwenden("a.txt", 0), "a7.txt");
    }
}
