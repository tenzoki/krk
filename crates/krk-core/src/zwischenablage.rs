//! Was in der Zwischenablage steht, gedeutet als Ziel (C10).
//!
//! ```text
//! appkit::zwischenablage::lesen ──> deuten ──> Ziel::Pfad ──> kommandos::pfadeingabe
//!   (NSPasteboard, zwei Sorten)              ──> Ziel::Web  ──> Systembrowser
//!                                            ──> Ziel::Nichts ─> Statuszeile
//!
//! appkit::zwischenablage::einfuegequelle ──> filtertext_aus ──> Ok(Text) ──> Filtertext des Tabs
//!   (NSPasteboard, Verweise vor Text)                        ──> Err(Hindernis) ──> Statuszeile
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
//!
//! # Eine zweite Deutung: was aus der Ablage in den Filter kommt (Runde 21)
//!
//! `cmd+v` im Dateifenster haengt an den Filtertext an, was die Zwischenablage
//! traegt — nicht wie es dort steht, sondern gereinigt. Die Reinigung ist
//! [`filtertext_aus`], und sie wohnt hier und nicht in `verzeichnis::filter`,
//! weil sie eine zweite Deutung desselben Gegenstands ist und
//! `verweis_zu_pfad` braucht, das die erste schon traegt. Ihre fuenf Schritte,
//! in der Reihenfolge von A3 des Specs der Runde 21:
//!
//! 1. Keine Quelle ist `KeinText`; mehrere Dateiverweise sind
//!    `MehrereVerweise(n)`, das Gegenstueck zum mehrzeiligen Text, denn der
//!    Finder legt beim Kopieren mehrerer Eintraege die Namen als Zeilen ab.
//!    Ein einzelner Verweis geht als Pfadtext weiter zu Schritt 4.
//! 2. Bei Text fallen die Zeilenenden am Ende, `\r\n` wie `\n`; ein aus einem
//!    Terminal kopierter Name bringt eines mit. Steht danach noch ein `\n`
//!    im Text, ist er `Mehrzeilig`, und eingefuegt wird nichts — ganz oder gar
//!    nicht, eine halbe erste Zeile waere ein Sonderfall mit eigener Regel.
//! 3. Ein `file:`-Verweis, den `verweis_zu_pfad` aufloest, ist als Pfad der
//!    Pfadtext, Prozentzeichen aufgeloest; jeder andere Text ist sein eigener
//!    Pfadtext. **Einen Zweig fuer `http:` gibt es nicht**: eine Adresse ist
//!    nach der naechsten Regel ein Pfad, und es bleibt, was nach dem letzten
//!    Schraegstrich steht.
//! 4. Vom Pfadtext bleibt das letzte nicht leere Stueck beim Teilen an `/`.
//!    **Eine Regel fuer Verweis, Pfadtext und Namen**: ein Text ohne `/` ist
//!    sein einziges Stueck und kommt ganz, `Ordner/` liefert `Ordner`, `/`
//!    allein liefert nichts. `Path::file_name` waere hier die falsche Wahl,
//!    weil es fuer `/` und `..` `None` liefert und damit eine zweite Regel.
//! 5. Aus dem Stueck faellt jedes Zeichen, das die Zeichenregel
//!    `traegt_ein_dateiname` abweist, und dazu der Doppelpunkt. Bleibt
//!    nichts, ist das `NichtsTragbar`.
//!
//! **Der Doppelpunkt faellt nur hier, und die Tipp-Regel bleibt, wie sie ist.**
//! Wer `:` tippt, bekommt ihn in den Filtertext, denn ein POSIX-Name traegt
//! ihn; ein eingefuegter Doppelpunkt stammt dagegen fast immer aus einem Pfad
//! in Finder-Schreibweise oder aus einem Link. Ein Schalter an der Kernregel
//! gaebe ihr zwei Bedeutungen und zwaenge die Tippsuche der Belegungsansicht,
//! einen Wert einzusetzen, den sie nicht meint; deshalb steht die vierte
//! Klasse als eine Zeile in [`tragbar`] daneben. Diese Datei ist damit der
//! dritte Rufer der Zeichenregel, und die Zaehlprobe in
//! `crates/krk-core/tests/verzeichnis.rs` fuehrt sie namentlich.
//!
//! `#[must_use]` steht **nicht** an `filtertext_aus`: `Result` traegt es in
//! der Standardbibliothek, und ein zweites am `fn` loeste
//! `clippy::double_must_use` aus. Ein Rufer, der die Antwort fallen liesse,
//! bekaeme die Warnung des Uebersetzers trotzdem.

use std::borrow::Cow;
use std::path::PathBuf;

use crate::verzeichnis::filter::traegt_ein_dateiname;

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

/// Was die Zwischenablage fuer das Einfuegen in den Filter hergibt (Runde 21).
///
/// Die Rangfolge, Dateiverweise vor Text, hat die Huelle schon getroffen:
/// `Verweise` heisst, es lagen welche da, gleich ob daneben eine Namenszeile
/// stand; `Text` heisst, es lag keiner da.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Einfuegequelle {
    /// Die Dateiverweise der Ablage, in ihrer Reihenfolge.
    Verweise(Vec<PathBuf>),
    /// Der Text der Ablage, wie er dort steht.
    Text(String),
    /// Weder Verweis noch Text, etwa nach dem Kopieren eines Bildes.
    Leer,
}

/// Warum ein Einfuegen nichts eingefuegt hat (Runde 21).
///
/// Vier Werte, vier Saetze der Statuszeile (A5 des Specs), in derselben
/// Reihenfolge; der Schreiber der Saetze in `krk-ui` verzweigt vollstaendig
/// und ohne Auffangzweig, ein fuenfter Wert haelt dort den Bau an.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Einfuegehindernis {
    /// Die Ablage traegt weder Text noch Verweis.
    KeinText,
    /// Der Text traegt ein Zeilenende, das nicht am Ende steht.
    Mehrzeilig,
    /// Mehr als ein Dateiverweis; die Zahl fuer den Satz.
    MehrereVerweise(usize),
    /// Nach der Reinigung bleibt kein Zeichen uebrig.
    NichtsTragbar,
}

/// Was aus der Ablage an den Filtertext kommt, oder warum nichts (Runde 21).
///
/// Die fuenf Schritte stehen im Modulkopf. Traegt bewusst kein
/// `#[must_use]`: `Result` bringt es mit, ein zweites waere
/// `clippy::double_must_use`.
pub fn filtertext_aus(quelle: &Einfuegequelle) -> Result<String, Einfuegehindernis> {
    let pfadtext: Cow<'_, str> = match quelle {
        Einfuegequelle::Leer => return Err(Einfuegehindernis::KeinText),
        Einfuegequelle::Verweise(verweise) => match verweise.as_slice() {
            // Eine leere Liste liefert die Huelle nicht; kaeme sie, traegt
            // die Ablage der Sache nach nichts.
            [] => return Err(Einfuegehindernis::KeinText),
            [einer] => einer.to_string_lossy(),
            mehrere => return Err(Einfuegehindernis::MehrereVerweise(mehrere.len())),
        },
        Einfuegequelle::Text(text) => {
            let rest = text.trim_end_matches(['\n', '\r']);
            if rest.contains('\n') {
                return Err(Einfuegehindernis::Mehrzeilig);
            }
            match ohne_schema(rest, "file").and_then(verweis_zu_pfad) {
                Some(pfad) => Cow::Owned(pfad.to_string_lossy().into_owned()),
                None => Cow::Borrowed(rest),
            }
        }
    };
    let gereinigt: String = letzter_bestandteil(&pfadtext)
        .chars()
        .filter(|zeichen| tragbar(*zeichen))
        .collect();
    if gereinigt.is_empty() {
        return Err(Einfuegehindernis::NichtsTragbar);
    }
    Ok(gereinigt)
}

/// Das letzte nicht leere Stueck beim Teilen an `/`; leer, wenn es keines gibt.
fn letzter_bestandteil(text: &str) -> &str {
    text.rsplit('/')
        .find(|stueck| !stueck.is_empty())
        .unwrap_or("")
}

/// Die Zeichenregel des Filters, beim Einfuegen um den Doppelpunkt verschaerft.
///
/// Die eine Zeile, an der das Einfuegen vom Tippen abweicht; warum, steht im
/// Modulkopf.
fn tragbar(zeichen: char) -> bool {
    traegt_ein_dateiname(zeichen) && zeichen != ':'
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

    // Die Reinigung fuer das Einfuegen in den Filter (Runde 21, C4.2):
    // je Schritt und je Hindernis eine Probe.

    fn text(inhalt: &str) -> Result<String, Einfuegehindernis> {
        filtertext_aus(&Einfuegequelle::Text(inhalt.to_owned()))
    }

    #[test]
    fn ein_name_ohne_schraegstrich_kommt_ganz() {
        assert_eq!(text("Notizen.md"), Ok("Notizen.md".to_owned()));
    }

    #[test]
    fn von_einem_pfad_bleibt_der_letzte_bestandteil() {
        assert_eq!(text("/Users/k1/Notizen.md"), Ok("Notizen.md".to_owned()));
        assert_eq!(text("Ordner/"), Ok("Ordner".to_owned()));
    }

    #[test]
    fn ein_einzelner_verweis_liefert_seinen_namen() {
        let quelle = Einfuegequelle::Verweise(vec![PathBuf::from("/Users/k1/Mein Text.md")]);
        assert_eq!(filtertext_aus(&quelle), Ok("Mein Text.md".to_owned()));
    }

    #[test]
    fn ein_file_verweis_als_text_wird_aufgeloest() {
        assert_eq!(
            text("file:///Users/k1/Mein%20Text.md"),
            Ok("Mein Text.md".to_owned())
        );
    }

    #[test]
    fn ein_nicht_lokaler_verweis_geht_als_pfadtext_durch_dieselbe_regel() {
        assert_eq!(text("file://fileserver/x/y.md"), Ok("y.md".to_owned()));
    }

    #[test]
    fn eine_web_adresse_ist_ein_pfad_und_hat_keinen_eigenen_zweig() {
        assert_eq!(
            text("https://example.com/pfad/seite.html"),
            Ok("seite.html".to_owned())
        );
    }

    #[test]
    fn zeilenenden_am_ende_fallen() {
        assert_eq!(text("Name\n"), Ok("Name".to_owned()));
        assert_eq!(text("Name\r\n"), Ok("Name".to_owned()));
    }

    #[test]
    fn tabulator_und_doppelpunkt_fallen() {
        assert_eq!(text("a\tb:c"), Ok("abc".to_owned()));
    }

    #[test]
    fn ein_inneres_zeilenende_ist_mehrzeilig() {
        assert_eq!(
            text("erste Zeile\nzweite Zeile"),
            Err(Einfuegehindernis::Mehrzeilig)
        );
    }

    #[test]
    fn mehrere_verweise_werden_mit_ihrer_zahl_abgewiesen() {
        let quelle = Einfuegequelle::Verweise(vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ]);
        assert_eq!(
            filtertext_aus(&quelle),
            Err(Einfuegehindernis::MehrereVerweise(3))
        );
    }

    #[test]
    fn eine_leere_ablage_traegt_keinen_text() {
        assert_eq!(
            filtertext_aus(&Einfuegequelle::Leer),
            Err(Einfuegehindernis::KeinText)
        );
        assert_eq!(
            filtertext_aus(&Einfuegequelle::Verweise(Vec::new())),
            Err(Einfuegehindernis::KeinText)
        );
    }

    #[test]
    fn was_nach_der_reinigung_leer_ist_traegt_nichts() {
        assert_eq!(text("\t:\t"), Err(Einfuegehindernis::NichtsTragbar));
        assert_eq!(text("/"), Err(Einfuegehindernis::NichtsTragbar));
    }

    #[test]
    fn der_stern_und_das_leerzeichen_bleiben_stehen() {
        let name = "260503-1144_*_f1-zitadel-slot-rehost-and-swap-test.md";
        assert_eq!(text(name), Ok(name.to_owned()));
        assert_eq!(text("ab cd"), Ok("ab cd".to_owned()));
    }
}
