//! Der Name einer Datei, die in `minutes/` des Notizordners angelegt wird.
//!
//! Eine Regel, eine reine Funktion: [`protokollname`] setzt Datum und Uhrzeit
//! vor den getippten Namen und haengt `.md` an (C4.2, C4.4 und C4.5 des Spec
//! `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`, E9 des
//! Plans `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`).
//! **Die Uhr liest dieses Modul nicht**: die Zeit kommt als Wert herein, und
//! der Rufer erhebt sie im Augenblick des Bestaetigens. Ob der Ordner, in dem
//! angelegt wird, `minutes/` ist, fragt er vorher
//! [`Heimordner::ist_protokollordner`](super::Heimordner::ist_protokollordner);
//! diese Funktion fragt nichts und beruehrt die Platte nicht.

use crate::verzeichnis::sys::Ortszeit;

/// Die Endung, die jede Datei in `minutes/` traegt.
const ENDUNG: &str = ".md";

/// Der Name, unter dem eine Datei in `minutes/` entsteht:
/// `JJMMTT-HHMM-<name>.md`.
///
/// Jahr, Monat, Tag, Stunde und Minute stehen je zweistellig, das Jahr ohne
/// sein Jahrhundert; die Sekunde der Zeit zaehlt nicht. **`.md` wird
/// angehaengt, ausser der Name endet schon darauf**, ohne Ruecksicht auf
/// Gross- und Kleinschreibung, und dann bleibt seine Schreibweise stehen
/// (`Besprechung.MD` wird nicht zu `Besprechung.md`). Eine andere Endung
/// bleibt und bekommt `.md` dahinter: `liste.txt` ergibt `…-liste.txt.md`.
///
/// Der Name selbst wird nicht geprueft und nicht veraendert; was ein
/// zulaessiger Dateiname ist, entscheidet das Namensblatt vor diesem Aufruf.
#[must_use = "der gebildete Name ist die ganze Wirkung; angelegt wird hier nichts"]
pub fn protokollname(name: &str, zeit: Ortszeit) -> String {
    // Verglichen wird an den Bytes: `.md` ist ASCII, und ein Schnitt an
    // Zeichen koennte bei einem Namen mit Umlaut mitten in eines fallen.
    let bytes = name.as_bytes();
    let traegt_die_endung = bytes.len() >= ENDUNG.len()
        && bytes[bytes.len() - ENDUNG.len()..].eq_ignore_ascii_case(ENDUNG.as_bytes());
    let endung = if traegt_die_endung { "" } else { ENDUNG };
    format!(
        "{jahr:02}{monat:02}{tag:02}-{stunde:02}{minute:02}-{name}{endung}",
        jahr = zeit.jahr.rem_euclid(100),
        monat = zeit.monat,
        tag = zeit.tag,
        stunde = zeit.stunde,
        minute = zeit.minute,
    )
}
