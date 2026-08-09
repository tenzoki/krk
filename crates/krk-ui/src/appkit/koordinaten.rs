//! Der Wechsel zwischen den beiden Textkoordinaten: Byteversaetze eines
//! UTF-8-Textes und die UTF-16-Einheiten, in denen AppKit zaehlt.
//!
//! ```text
//!  krk_core::text  ──> Byteversatz ──> in_utf16 ──────> NSRange
//!  (Zeilenindex,                                        (Auswahl, Bildlauf,
//!   Suchtreffer)   <── Byteversatz <── in_bytes  <────   Zeilenkasten)
//! ```
//!
//! **Eine Umrechnung und keine zwei.** Sie stand bis zum 260810 als
//! `anfaenge_in_utf16` in [`super::nummernspalte`] und war dort privat; der
//! Zeilensprung, die Suche und die Auskunft ueber die Zeile der Schreibmarke
//! brauchen dieselbe Rechnung, und der Defekt
//! `issues/260810-0036_*_dem-editor-fehlt-die-auskunft-ueber-die-zeile-der-schreibmarke.md`
//! haelt fest, warum ein zweiter Rechenweg keiner sein darf: der Modulkopf von
//! `krk_core::text` sagt zu, dass jeder Versatz auf einer **Zeichengrenze**
//! liegt, und zwei Rechenwege muessten diese Zusage doppelt tragen.
//!
//! **Keine Zeile AppKit**, obwohl das Modul unter `appkit/` liegt. Es rechnet
//! auf einer Zeichenkette und braucht kein Fenster; seine Pruefungen stehen
//! deshalb am Dateiende und nicht unter `Nutzerarbeit`. Hier liegt es
//! trotzdem, weil die zweite Koordinate **AppKits** ist: ausserhalb dieses
//! Teilbaums gibt es niemanden, der in UTF-16-Einheiten zaehlt.
//!
//! # Warum ueberhaupt gerechnet wird
//!
//! Rust misst eine Zeichenkette in Bytes, AppKits Textsystem in
//! UTF-16-Einheiten. Ein Umlaut kostet zwei Bytes und eine Einheit, ein
//! Bildzeichen vier Bytes und zwei. Ohne den Wechsel truege jede Zeile hinter
//! dem ersten Zeichen ausserhalb von ASCII eine falsche Nummer, und die
//! Schreibmarke eines Zeilensprungs landete hinter dem ersten Umlaut an der
//! falschen Stelle.
//!
//! # Was die beiden Richtungen zusichern
//!
//! [`in_utf16`] verlangt aufsteigende Byteversaetze auf Zeichengrenzen und
//! liefert genauso viele Einheitenversaetze zurueck, wie es Byteversaetze
//! bekommen hat. **Wiederholungen sind zugelassen**: der Zeilensprung fragt
//! nach einer Stelle ohne Ausdehnung und reicht denselben Versatz zweimal
//! herein.
//!
//! [`in_bytes`] nimmt jede Einheitenstelle entgegen, auch eine mitten in einem
//! Ersatzzeichenpaar, und liefert dafuer den Anfang des Zeichens, in dem sie
//! liegt. Damit liegt das Ergebnis **immer** auf einer Zeichengrenze, und die
//! Zusage aus `krk_core::text` haelt auch fuer eine Schreibmarke, die AppKit
//! irgendwohin gesetzt hat.

/// Dieselben Versaetze in AppKits Koordinate.
///
/// `byteversaetze` zaehlt Bytes, aufsteigend und auf Zeichengrenzen; das
/// Ergebnis zaehlt UTF-16-Einheiten und ist genauso lang. Ein Versatz, der auf
/// keiner Zeichengrenze liegt, wird uebergangen und bekommt am Ende das
/// Textende zugewiesen — falsch, aber nicht abstuerzend; die Aufrufer nehmen
/// ihre Versaetze aus `krk_core::text` und geben die Zusage nicht auf.
///
/// **Ein Koordinatenwechsel und keine Zaehlung.** Welche Stellen gemeint sind,
/// sagt allein der Aufrufer; diese Funktion laeuft einmal ueber den Text und
/// zaehlt dabei Einheiten statt Bytes.
pub fn in_utf16(text: &str, byteversaetze: &[usize]) -> Vec<usize> {
    let mut umgerechnet = Vec::with_capacity(byteversaetze.len());
    let mut naechster = 0usize;
    let mut gezaehlt = 0usize;
    for (byte, zeichen) in text.char_indices() {
        // `while` und nicht `if`: derselbe Versatz darf mehrfach hereinkommen,
        // und eine Stelle ohne Ausdehnung ist genau das (Anfang gleich Ende).
        while byteversaetze.get(naechster) == Some(&byte) {
            umgerechnet.push(gezaehlt);
            naechster += 1;
        }
        gezaehlt += zeichen.len_utf16();
    }
    // Ein Versatz am Textende hat kein Zeichen mehr hinter sich und kommt im
    // Durchgang oben nicht vor: das Ende des letzten Treffers, die leere letzte
    // Zeile eines Textes, der auf einem Umbruch endet, und die einzige Zeile
    // des leeren Textes.
    while naechster < byteversaetze.len() {
        umgerechnet.push(gezaehlt);
        naechster += 1;
    }
    umgerechnet
}

/// Der Byteversatz zu einer Stelle in AppKits Koordinate.
///
/// Liegt die Stelle hinter dem Text, ist die Antwort das Textende. Liegt sie
/// mitten in einem Ersatzzeichenpaar — was AppKit nicht tut, was seine
/// Schnittstelle aber zulaesst —, ist die Antwort der Anfang jenes Zeichens.
/// Beide Antworten liegen auf einer Zeichengrenze, und das ist die Bedingung
/// jeder Verwendung in `krk_core::text`.
pub fn in_bytes(text: &str, utf16versatz: usize) -> usize {
    let mut gezaehlt = 0usize;
    for (byte, zeichen) in text.char_indices() {
        // Die gesuchte Stelle liegt in diesem Zeichen, sobald sie vor seinem
        // Ende liegt. Der Vergleich gegen das Ende und nicht gegen den Anfang
        // ist der ganze Unterschied: eine Stelle mitten in einem
        // Ersatzzeichenpaar liegt hinter dem Anfang des Bildzeichens und
        // trotzdem darin, und die Antwort ist sein Anfang.
        if utf16versatz < gezaehlt + zeichen.len_utf16() {
            return byte;
        }
        gezaehlt += zeichen.len_utf16();
    }
    text.len()
}

/// Der Koordinatenwechsel ist reine Rechnung und braucht kein Fenster; deshalb
/// stehen seine Pruefungen hier und nicht unter `Nutzerarbeit`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohne_zeichen_ausserhalb_von_ascii_sind_beide_koordinaten_gleich() {
        let text = "eins\nzwei\ndrei";
        assert_eq!(in_utf16(text, &[0, 5, 10]), vec![0, 5, 10]);
        assert_eq!(in_bytes(text, 5), 5);
    }

    /// Der Grund, aus dem der Wechsel ueberhaupt stattfindet: ein Umlaut kostet
    /// zwei Bytes und eine UTF-16-Einheit, ein Bildzeichen vier Bytes und zwei.
    #[test]
    fn umlaute_und_bildzeichen_verschieben_die_beiden_koordinaten_gegeneinander() {
        let text = "Äpfel\n🍎🍎\nEnde";
        // In Bytes: 6 fuer "Äpfel\n", danach 9 fuer die beiden Bildzeichen und
        // den Umbruch.
        assert_eq!(in_utf16(text, &[0, 7, 16]), vec![0, 6, 11]);
        assert_eq!(in_bytes(text, 6), 7);
        assert_eq!(in_bytes(text, 11), 16);
    }

    #[test]
    fn ein_versatz_am_textende_kommt_mit() {
        let text = "eins\n";
        assert_eq!(in_utf16(text, &[0, 5]), vec![0, 5]);
        assert_eq!(in_utf16("", &[0]), vec![0]);
        assert_eq!(in_bytes(text, 99), 5, "hinter dem Text endet es am Ende");
    }

    /// Die Stelle ohne Ausdehnung, mit der der Zeilensprung fragt.
    #[test]
    fn derselbe_versatz_darf_mehrfach_hereinkommen() {
        let text = "Äpfel\nBirnen";
        assert_eq!(in_utf16(text, &[7, 7]), vec![6, 6]);
        assert_eq!(
            in_utf16(text, &[text.len(), text.len()]),
            vec![12, 12],
            "auch am Textende"
        );
    }

    /// Eine Stelle mitten in einem Ersatzzeichenpaar faellt auf den Anfang des
    /// Zeichens zurueck und nicht mitten hinein.
    #[test]
    fn eine_stelle_im_ersatzzeichenpaar_faellt_auf_die_zeichengrenze() {
        let text = "a🍎b";
        assert_eq!(in_bytes(text, 1), 1, "der Anfang des Bildzeichens");
        assert_eq!(in_bytes(text, 2), 1, "die zweite Haelfte zaehlt nicht mit");
        assert_eq!(in_bytes(text, 3), 5, "das Zeichen dahinter");
        assert!(text.is_char_boundary(in_bytes(text, 2)));
    }

    /// Die Runde durch beide Richtungen laesst jeden Zeilenanfang, wo er war.
    #[test]
    fn beide_richtungen_zusammen_ergeben_die_ausgangsstelle() {
        let text = "Äpfel\n🍎🍎\nEnde";
        for versatz in (0..=text.len()).filter(|stelle| text.is_char_boundary(*stelle)) {
            let hin = in_utf16(text, &[versatz]);
            assert_eq!(in_bytes(text, hin[0]), versatz);
        }
    }
}
