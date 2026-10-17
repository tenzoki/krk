//! Die Aufbereitung einer Datei aus JSON-Zeilen (`.jsonl`, `.ndjson`) fuer
//! die Vorschau: jeder Datensatz eingerueckt und mehrzeilig, die Datensaetze
//! durch eine nummerierte Trennzeile voneinander abgesetzt.
//!
//! **Keine Zeile AppKit**, wie [`crate::markdown`] daneben: die Datei rechnet
//! einen Text aus einem Text, und gerufen wird sie allein auf dem Arbeitsfaden
//! der Vorschau (`vorschaumodell::laden`), also nie auf dem Hauptfaden.
//!
//! # Warum ein eigener Einruecker und keine Kiste
//!
//! Im Baum steht keine JSON-Kiste, und fuer diese eine Aufgabe braucht es
//! keine: eingerueckt wird zeichenweise nach der Struktur, und **kein Wert wird
//! gedeutet**. Eine Zahl, eine Zeichenkette samt ihren Escapes und ein Literal
//! erscheinen Byte fuer Byte so, wie sie in der Datei stehen; `1.50` bleibt
//! `1.50` und `"\u00e9"` bleibt `"\u00e9"`. Gedeutet waere es eine Umschrift
//! des Datensatzes und keine Ansicht davon. Geprueft wird dabei trotzdem die
//! ganze Grammatik, damit eine Zeile, die kein JSON ist, nicht halb
//! eingerueckt erscheint, sondern unveraendert und markiert.
//!
//! # Warum die Trennzeile ein Kommentar ist
//!
//! Die Ansicht faerbt den aufbereiteten Text mit der JSON-Definition der
//! Syntaxkiste ein ([`crate::hervorhebung`], die [`ENDUNGEN`] dafuer auf
//! `json` abbildet), in einem Durchgang ueber den ganzen Text. Eine Trennzeile
//! `// ── 3 ──` ist fuer diese Definition ein Zeilenkommentar und erscheint
//! in der gedaempften Farbe der Tafel. Aus demselben Grund steht eine
//! ungueltige Zeile **hinter** `// `: ohne das liesse etwa `kaputt {` den
//! Zerleger in einem offenen Objekt zurueck, und jeder folgende Datensatz
//! truege falsche Farben.

use std::path::Path;

use krk_core::sprache::{Text, satz, text};

/// Die Endungen, an denen eine Datei aus JSON-Zeilen erkannt wird, in
/// Kleinschreibung. Verglichen ohne Ruecksicht auf Gross- und Kleinschreibung.
///
/// **Die eine Liste**: die Vorschau fragt sie ueber [`ist_jsonzeilenpfad`],
/// die Einfaerbung ueber dieselbe Funktion, damit die aufbereitete Datei und
/// ihre Farben nie auseinanderlaufen.
pub const ENDUNGEN: [&str; 2] = ["jsonl", "ndjson"];

/// Der Einzug je Tiefe.
const EINZUG: &str = "  ";

/// Ob der Pfad auf eine der [`ENDUNGEN`] endet. Liest nichts von der Platte.
pub fn ist_jsonzeilenpfad(pfad: &Path) -> bool {
    pfad.extension().is_some_and(|endung| {
        let endung = endung.to_string_lossy();
        ENDUNGEN
            .iter()
            .any(|bekannt| endung.eq_ignore_ascii_case(bekannt))
    })
}

/// Bereitet den Text einer JSON-Zeilen-Datei fuer die Vorschau auf.
///
/// Jede nicht leere Zeile ist ein Datensatz; leere Zeilen und Zeilen aus
/// Leerraum fallen weg und zaehlen nicht mit. Vor jedem Datensatz steht die
/// Trennzeile `// ── <nummer> ──`, zwischen zwei Datensaetzen eine Leerzeile.
/// Ein gueltiger Datensatz folgt eingerueckt, ein ungueltiger unveraendert
/// hinter `// `, und seine Trennzeile traegt die Markierung.
///
/// `gekuerzt_bei` ist die Lesegrenze in MB, wenn der Text nur der Anfang der
/// Datei ist; dann steht der Hinweis darauf als erste Zeile. Dass der Text an
/// einem Zeilenende aufhoert, sorgt der Rufer.
pub fn aufbereiten(quelle: &str, gekuerzt_bei: Option<u64>) -> String {
    let quelle = quelle.strip_prefix('\u{feff}').unwrap_or(quelle);
    let mut aus = String::with_capacity(quelle.len() * 2);
    if let Some(grenze) = gekuerzt_bei {
        aus.push_str("// ");
        aus.push_str(&satz(
            Text::VorschauJsonzeilenGekuerzt,
            &[("grenze", &grenze)],
        ));
        aus.push_str("\n\n");
    }
    let mut nummer: u64 = 0;
    // `lines` nimmt `\n` und `\r\n` ab; eine Datei aus Windows erscheint damit
    // ohne Wagenruecklauf am Ende jedes Datensatzes.
    for zeile in quelle.lines() {
        if zeile.trim().is_empty() {
            continue;
        }
        nummer += 1;
        if nummer > 1 {
            aus.push('\n');
        }
        match einruecken(zeile) {
            Some(eingerueckt) => {
                aus.push_str(&format!("// ── {nummer} ──\n"));
                aus.push_str(&eingerueckt);
            }
            None => {
                aus.push_str(&format!(
                    "// ── {nummer} ── {}\n// {zeile}",
                    text(Text::VorschauJsonzeileUngueltig)
                ));
            }
        }
        aus.push('\n');
    }
    aus
}

/// Ein Stueck einer Zeile, wie der Zerleger es liefert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stueck<'a> {
    /// `{` oder `[`.
    Auf(u8),
    /// `}` oder `]`.
    Zu(u8),
    Doppelpunkt,
    Komma,
    /// Eine Zeichenkette samt ihren Anfuehrungszeichen, eine Zahl oder ein
    /// Literal, woertlich aus der Zeile.
    Wert(&'a str),
}

/// Was der Einruecker als naechstes Stueck erwartet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Erwartet {
    Wert,
    Schluessel,
    Doppelpunkt,
    /// Ein Komma oder das Ende des innersten Behaelters.
    KommaOderEnde,
    /// Der eine Wert der Zeile ist vollstaendig; jedes weitere Stueck ist
    /// ein Fehler.
    Fertig,
}

/// Rueckt einen Datensatz ein; `None`, wenn die Zeile kein JSON-Wert ist.
///
/// Leere Behaelter bleiben auf einer Zeile (`{}`, `[]`), jeder andere oeffnet
/// eine Ebene mit [`EINZUG`] je Tiefe. Das Ergebnis endet ohne Zeilenumbruch.
fn einruecken(zeile: &str) -> Option<String> {
    let stuecke = zerlegen(zeile)?;
    let mut aus = String::with_capacity(zeile.len() * 2);
    let mut stapel: Vec<u8> = Vec::new();
    let mut erwartet = Erwartet::Wert;
    let mut stelle = 0;
    while let Some(&stueck) = stuecke.get(stelle) {
        stelle += 1;
        erwartet = match (erwartet, stueck) {
            (Erwartet::Wert, Stueck::Wert(wert)) => {
                aus.push_str(wert);
                nach_dem_wert(&stapel)
            }
            (Erwartet::Wert, Stueck::Auf(auf)) => {
                let zu = gegenstueck(auf);
                if stuecke.get(stelle) == Some(&Stueck::Zu(zu)) {
                    stelle += 1;
                    aus.push(char::from(auf));
                    aus.push(char::from(zu));
                    nach_dem_wert(&stapel)
                } else {
                    stapel.push(auf);
                    aus.push(char::from(auf));
                    neue_zeile(&mut aus, stapel.len());
                    if auf == b'{' {
                        Erwartet::Schluessel
                    } else {
                        Erwartet::Wert
                    }
                }
            }
            (Erwartet::Schluessel, Stueck::Wert(schluessel)) if schluessel.starts_with('"') => {
                aus.push_str(schluessel);
                Erwartet::Doppelpunkt
            }
            (Erwartet::Doppelpunkt, Stueck::Doppelpunkt) => {
                aus.push_str(": ");
                Erwartet::Wert
            }
            (Erwartet::KommaOderEnde, Stueck::Komma) => {
                aus.push(',');
                neue_zeile(&mut aus, stapel.len());
                if stapel.last() == Some(&b'{') {
                    Erwartet::Schluessel
                } else {
                    Erwartet::Wert
                }
            }
            (Erwartet::KommaOderEnde, Stueck::Zu(zu))
                if stapel.last().copied().map(gegenstueck) == Some(zu) =>
            {
                stapel.pop();
                neue_zeile(&mut aus, stapel.len());
                aus.push(char::from(zu));
                nach_dem_wert(&stapel)
            }
            _ => return None,
        };
    }
    (erwartet == Erwartet::Fertig).then_some(aus)
}

/// Was nach einem vollstaendigen Wert kommt: das Ende der Zeile, wenn er auf
/// oberster Ebene stand, sonst ein Komma oder das Ende seines Behaelters.
fn nach_dem_wert(stapel: &[u8]) -> Erwartet {
    if stapel.is_empty() {
        Erwartet::Fertig
    } else {
        Erwartet::KommaOderEnde
    }
}

/// Die schliessende Klammer zu einer oeffnenden.
fn gegenstueck(auf: u8) -> u8 {
    if auf == b'{' { b'}' } else { b']' }
}

/// Ein Zeilenumbruch und der Einzug fuer `tiefe`.
fn neue_zeile(aus: &mut String, tiefe: usize) {
    aus.push('\n');
    for _ in 0..tiefe {
        aus.push_str(EINZUG);
    }
}

/// Zerlegt eine Zeile in ihre Stuecke; `None` bei einem Zeichen, das in JSON
/// ausserhalb einer Zeichenkette nicht stehen darf, bei einer ungueltigen
/// Zeichenkette und bei einem Literal, das weder `true`, `false`, `null` noch
/// eine Zahl ist.
fn zerlegen(zeile: &str) -> Option<Vec<Stueck<'_>>> {
    let bytes = zeile.as_bytes();
    let mut stuecke = Vec::new();
    let mut stelle = 0;
    while let Some(&byte) = bytes.get(stelle) {
        match byte {
            b' ' | b'\t' | b'\r' | b'\n' => stelle += 1,
            b'{' | b'[' => {
                stuecke.push(Stueck::Auf(byte));
                stelle += 1;
            }
            b'}' | b']' => {
                stuecke.push(Stueck::Zu(byte));
                stelle += 1;
            }
            b':' => {
                stuecke.push(Stueck::Doppelpunkt);
                stelle += 1;
            }
            b',' => {
                stuecke.push(Stueck::Komma);
                stelle += 1;
            }
            b'"' => {
                let ende = zeichenkette_ende(bytes, stelle)?;
                stuecke.push(Stueck::Wert(&zeile[stelle..ende]));
                stelle = ende;
            }
            _ => {
                let anfang = stelle;
                while bytes
                    .get(stelle)
                    .is_some_and(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'+' | b'-'))
                {
                    stelle += 1;
                }
                let literal = &zeile[anfang..stelle];
                if literal.is_empty() || !ist_literal(literal) {
                    return None;
                }
                stuecke.push(Stueck::Wert(literal));
            }
        }
    }
    Some(stuecke)
}

/// Die Stelle hinter dem schliessenden Anfuehrungszeichen der Zeichenkette,
/// die bei `anfang` beginnt; `None`, wenn sie nicht schliesst, ein
/// Steuerzeichen enthaelt oder ein Escape traegt, das JSON nicht kennt.
///
/// Gearbeitet wird auf Bytes: jedes Byte eines mehrbytigen UTF-8-Zeichens ist
/// groesser als 0x7F und trifft deshalb weder `"` noch `\` noch ein
/// Steuerzeichen, und die Grenzen, an denen geschnitten wird, liegen immer
/// an einem `"`.
fn zeichenkette_ende(bytes: &[u8], anfang: usize) -> Option<usize> {
    let mut stelle = anfang + 1;
    loop {
        match *bytes.get(stelle)? {
            b'"' => return Some(stelle + 1),
            b'\\' => match *bytes.get(stelle + 1)? {
                b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => stelle += 2,
                b'u' => {
                    let ziffern = bytes.get(stelle + 2..stelle + 6)?;
                    if !ziffern.iter().all(u8::is_ascii_hexdigit) {
                        return None;
                    }
                    stelle += 6;
                }
                _ => return None,
            },
            byte if byte < 0x20 => return None,
            _ => stelle += 1,
        }
    }
}

/// Ob `literal` eines der drei Wortliterale oder eine Zahl nach der Grammatik
/// von JSON ist: `-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?`.
fn ist_literal(literal: &str) -> bool {
    if matches!(literal, "true" | "false" | "null") {
        return true;
    }
    let bytes = literal.as_bytes();
    let mut stelle = 0;
    if bytes.first() == Some(&b'-') {
        stelle += 1;
    }
    match bytes.get(stelle) {
        Some(b'0') => stelle += 1,
        Some(b'1'..=b'9') => stelle = ziffern_ab(bytes, stelle),
        _ => return false,
    }
    if bytes.get(stelle) == Some(&b'.') {
        let nach = ziffern_ab(bytes, stelle + 1);
        if nach == stelle + 1 {
            return false;
        }
        stelle = nach;
    }
    if matches!(bytes.get(stelle), Some(b'e' | b'E')) {
        stelle += 1;
        if matches!(bytes.get(stelle), Some(b'+' | b'-')) {
            stelle += 1;
        }
        let nach = ziffern_ab(bytes, stelle);
        if nach == stelle {
            return false;
        }
        stelle = nach;
    }
    stelle == bytes.len()
}

/// Die erste Stelle ab `anfang`, an der keine Ziffer steht.
fn ziffern_ab(bytes: &[u8], anfang: usize) -> usize {
    let mut stelle = anfang;
    while bytes.get(stelle).is_some_and(u8::is_ascii_digit) {
        stelle += 1;
    }
    stelle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_verschachtelter_datensatz_wird_mit_zwei_leerzeichen_eingerueckt() {
        assert_eq!(
            einruecken(r#"{"a":1,"b":[true,null,{"c":"d"}],"e":{}}"#).as_deref(),
            Some(concat!(
                "{\n",
                "  \"a\": 1,\n",
                "  \"b\": [\n",
                "    true,\n",
                "    null,\n",
                "    {\n",
                "      \"c\": \"d\"\n",
                "    }\n",
                "  ],\n",
                "  \"e\": {}\n",
                "}",
            ))
        );
    }

    #[test]
    fn leere_behaelter_bleiben_auf_einer_zeile() {
        assert_eq!(einruecken("{}").as_deref(), Some("{}"));
        assert_eq!(einruecken("[ ]").as_deref(), Some("[]"));
        assert_eq!(
            einruecken(r#"{"x":[],"y":{ }}"#).as_deref(),
            Some("{\n  \"x\": [],\n  \"y\": {}\n}")
        );
    }

    #[test]
    fn klammern_anfuehrungszeichen_und_rueckstriche_in_zeichenketten_bleiben_stehen() {
        let zeile = r#"{"k{":"a, b: [c] \"d\" \\ e\/f \u00e9"}"#;
        assert_eq!(
            einruecken(zeile).as_deref(),
            Some("{\n  \"k{\": \"a, b: [c] \\\"d\\\" \\\\ e\\/f \\u00e9\"\n}")
        );
    }

    #[test]
    fn unicode_erscheint_woertlich() {
        assert_eq!(
            einruecken("[\"Grüße 🌍\",\"日本\"]").as_deref(),
            Some("[\n  \"Grüße 🌍\",\n  \"日本\"\n]")
        );
    }

    #[test]
    fn zahlen_werden_nicht_gedeutet() {
        assert_eq!(
            einruecken("[1.50,-0,2E+10,1e-3]").as_deref(),
            Some("[\n  1.50,\n  -0,\n  2E+10,\n  1e-3\n]")
        );
        assert_eq!(einruecken("42").as_deref(), Some("42"));
        assert_eq!(einruecken(r#""nur""#).as_deref(), Some(r#""nur""#));
    }

    #[test]
    fn was_kein_json_ist_wird_abgewiesen() {
        for zeile in [
            "kaputt {",
            "{\"a\":1",
            "{\"a\":1,}",
            "[1,]",
            "[,1]",
            "{\"a\" 1}",
            "{a:1}",
            "{\"a\":1]",
            "[1] [2]",
            "\"offen",
            "\"\\x\"",
            "\"\\u12g4\"",
            "\"tab\there\"",
            "01",
            "1.",
            ".5",
            "-",
            "1e",
            "NaN",
            "True",
            "{'a':1}",
            "{\"a\":1}}",
            "}",
            ":",
        ] {
            assert_eq!(einruecken(zeile), None, "{zeile:?} ist kein JSON");
        }
    }

    #[test]
    fn datensaetze_tragen_eine_nummerierte_trennzeile_und_leere_zeilen_fallen_weg() {
        let quelle = "{\"a\":1}\n\n   \n[2]\r\n\"drei\"\n";
        assert_eq!(
            aufbereiten(quelle, None),
            concat!(
                "// ── 1 ──\n",
                "{\n  \"a\": 1\n}\n",
                "\n",
                "// ── 2 ──\n",
                "[\n  2\n]\n",
                "\n",
                "// ── 3 ──\n",
                "\"drei\"\n",
            )
        );
    }

    #[test]
    fn eine_ungueltige_zeile_erscheint_unveraendert_mit_markierung() {
        let quelle = "{\"a\":1}\nkaputt { \"x\n{\"b\":2}\n";
        let markierung = text(Text::VorschauJsonzeileUngueltig);
        assert_eq!(
            aufbereiten(quelle, None),
            format!(
                "// ── 1 ──\n{{\n  \"a\": 1\n}}\n\n// ── 2 ── {markierung}\n// kaputt {{ \"x\n\n// ── 3 ──\n{{\n  \"b\": 2\n}}\n"
            )
        );
    }

    #[test]
    fn die_kuerzung_steht_als_erste_zeile() {
        let aufbereitet = aufbereiten("[1]\n", Some(1));
        let hinweis = satz(Text::VorschauJsonzeilenGekuerzt, &[("grenze", &1)]);
        assert_eq!(
            aufbereitet,
            format!("// {hinweis}\n\n// ── 1 ──\n[\n  1\n]\n")
        );
        assert!(
            hinweis.contains("1 MB"),
            "die Grenze steht im Hinweis: {hinweis}"
        );
    }

    #[test]
    fn eine_leere_datei_bleibt_leer_und_das_bom_faellt_weg() {
        assert_eq!(aufbereiten("", None), "");
        assert_eq!(aufbereiten("\n\n", None), "");
        assert_eq!(aufbereiten("\u{feff}[]\n", None), "// ── 1 ──\n[]\n");
    }

    #[test]
    fn die_endungen_greifen_ohne_ruecksicht_auf_schreibung() {
        assert!(ist_jsonzeilenpfad(Path::new("/a/protokoll.jsonl")));
        assert!(ist_jsonzeilenpfad(Path::new("/a/PROTOKOLL.JSONL")));
        assert!(ist_jsonzeilenpfad(Path::new("/a/strom.ndjson")));
        assert!(!ist_jsonzeilenpfad(Path::new("/a/daten.json")));
        assert!(!ist_jsonzeilenpfad(Path::new("/a/jsonl")));
    }
}
