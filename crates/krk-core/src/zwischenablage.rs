//! Was in der Zwischenablage steht, gedeutet als Ziel (C10).
//!
//! ```text
//! appkit::zwischenablage::lesen ──> deuten ──> Ziel::Pfad ──> kommandos::pfadeingabe
//!   (NSPasteboard, zwei Sorten)              ──> Ziel::Web  ──> Systembrowser
//!                                            ──> Ziel::Nichts ─> Statuszeile
//! ```
//!
//! Diese Datei ist reines Rust und nennt keine `objc2`-Kiste. Sie bekommt eine
//! Zeichenkette und sagt, was KRK damit tut; wo die Zeichenkette herkommt und
//! wer den Browser aufruft, steht in `krk-ui/src/appkit/zwischenablage.rs`.
//!
//! # Eine Auswertung, drei Ausgaenge
//!
//! Die drei Ausgaenge sind kein Rueckfallweg voneinander, sie schliessen sich
//! aus. Ein `file:`-Verweis zaehlt als Pfad und nicht als Adresse, weil er
//! dasselbe benennt und nur anders geschrieben ist.
//!
//! **Zum Systembrowser gehen allein `http:` und `https:`.** Jedes andere Schema
//! ist nicht verwertbar. Der Grund ist C9: gaebe KRK ein `smb:` oder `ftp:` an
//! das System weiter, baute es ueber einen Umweg genau die Serververbindung
//! auf, die C9 ausschliesst. Ein zweiter Zweig fuer weitere Schemata entsteht
//! nicht.
//!
//! **Der Pfad muss absolut sein, und diese Regel ist geerbt.** Die Pfadeingabe
//! von Hand aus C2 verlangt sie schon; der Sprung aus der Zwischenablage ist
//! dieselbe Pruefung mit einem Wert aus einer anderen Quelle. Ein relativer
//! Pfad und ein blosser Dateiname sind damit nicht verwertbar.
//!
//! # Was ueberhaupt gelesen wird
//!
//! Text **und** Dateiverweis, nach dem Nutzerentscheid vom 260804
//! (`decisions/260804-0830_*_was-die-zwischenablage-auswertung-liest.md`). Die
//! Rangfolge, Dateiverweis vor Text, sitzt in der Oberflaeche, weil allein sie
//! das Pasteboard kennt. Hier laufen beide Sorten in dieselbe Auswertung: ein
//! Dateiverweis kommt als `file:`-Zeichenkette an und braucht keinen eigenen
//! Zweig.

use std::path::PathBuf;

/// Wohin der Inhalt der Zwischenablage zeigt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ziel {
    /// Ein absoluter lokaler Pfad, gewoehnlich oder als `file:`-Verweis
    /// geschrieben.
    Pfad(PathBuf),
    /// Eine `http:`- oder `https:`-Adresse fuer den Systembrowser.
    Web(String),
    /// Nichts, womit KRK etwas anfangen kann.
    Nichts,
}

/// Deutet den Inhalt der Zwischenablage (C10).
pub fn deuten(inhalt: &str) -> Ziel {
    let text = inhalt.trim();
    if text.is_empty() {
        return Ziel::Nichts;
    }
    if let Some(rest) = ohne_schema(text, "file") {
        return match verweis_zu_pfad(rest) {
            Some(pfad) => Ziel::Pfad(pfad),
            None => Ziel::Nichts,
        };
    }
    if ohne_schema(text, "http").is_some() || ohne_schema(text, "https").is_some() {
        return Ziel::Web(text.to_owned());
    }
    if text.starts_with('/') {
        return Ziel::Pfad(PathBuf::from(text));
    }
    Ziel::Nichts
}

/// Was hinter `<schema>:` steht, falls der Text mit diesem Schema anfaengt.
///
/// Ohne Ruecksicht auf Gross- und Kleinschreibung: RFC 3986 erklaert das Schema
/// fuer schreibungsunabhaengig, und eine aus einer Adresszeile kopierte
/// `HTTPS:`-Adresse ist dieselbe Adresse.
fn ohne_schema<'a>(text: &'a str, schema: &str) -> Option<&'a str> {
    let grenze = schema.len() + 1;
    let (anfang, rest) = text.split_at_checked(grenze)?;
    let passt = anfang
        .strip_suffix(':')
        .is_some_and(|gelesen| gelesen.eq_ignore_ascii_case(schema));
    passt.then_some(rest)
}

/// Der Pfad, den ein `file:`-Verweis benennt.
///
/// Angenommen werden die drei Schreibweisen, die auf einem Mac vorkommen:
/// `file:///Ordner/Datei`, `file://localhost/Ordner/Datei` und der verkuerzte
/// `file:/Ordner/Datei`. Ein Verweis auf einen anderen Rechner traegt einen
/// Rechnernamen und ist damit kein lokaler Pfad; er liefert `None` und endet
/// nach C9 bei "nichts Verwertbares".
fn verweis_zu_pfad(rest: &str) -> Option<PathBuf> {
    let pfad = match rest.strip_prefix("//") {
        Some(nach_den_strichen) => {
            let laenge = nach_den_strichen.find('/')?;
            let (rechner, pfad) = nach_den_strichen.split_at(laenge);
            if !rechner.is_empty() && !rechner.eq_ignore_ascii_case("localhost") {
                return None;
            }
            pfad
        }
        None => rest,
    };
    let entschluesselt = prozent_dekodieren(pfad)?;
    entschluesselt
        .starts_with('/')
        .then(|| PathBuf::from(entschluesselt))
}

/// Loest die Prozentschreibweise einer URL auf.
///
/// `None` fuer eine kaputte Folge, etwa ein `%` am Ende oder `%zz`, und fuer
/// ein Ergebnis, das kein UTF-8 ist. Beides still zu uebergehen hiesse, aus
/// einem beschaedigten Verweis einen Pfad zu machen, den es nicht gibt.
fn prozent_dekodieren(text: &str) -> Option<String> {
    let roh = text.as_bytes();
    let mut gelesen = Vec::with_capacity(roh.len());
    let mut stelle = 0;
    while stelle < roh.len() {
        if roh[stelle] == b'%' {
            let ziffern = text.get(stelle + 1..stelle + 3)?;
            gelesen.push(u8::from_str_radix(ziffern, 16).ok()?);
            stelle += 3;
        } else {
            gelesen.push(roh[stelle]);
            stelle += 1;
        }
    }
    String::from_utf8(gelesen).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_verweis_mit_leerzeichen_wird_aufgeloest() {
        assert_eq!(
            deuten("file:///Users/k1/Mein%20Ordner"),
            Ziel::Pfad(PathBuf::from("/Users/k1/Mein Ordner"))
        );
    }

    #[test]
    fn ein_verweis_auf_localhost_ist_derselbe_lokale_pfad() {
        assert_eq!(
            deuten("file://localhost/Users/k1"),
            Ziel::Pfad(PathBuf::from("/Users/k1"))
        );
    }

    #[test]
    fn ein_verweis_auf_einen_anderen_rechner_ist_kein_lokaler_pfad() {
        assert_eq!(deuten("file://fileserver/freigabe/datei.txt"), Ziel::Nichts);
    }

    #[test]
    fn eine_kaputte_prozentfolge_liefert_keinen_pfad() {
        assert_eq!(deuten("file:///Users/k1/%zz"), Ziel::Nichts);
        assert_eq!(deuten("file:///Users/k1/%"), Ziel::Nichts);
    }
}
