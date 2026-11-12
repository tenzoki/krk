//! Welches Zeilenende ein Text traegt, gelesen an seinem ersten `\n`.
//!
//! Die eine Stelle der Regel: wer in einen Text, der nicht in gehaltener Form
//! steht, neue Zeilen schreibt, fragt hier, in welcher Form sie enden, damit
//! ein Text mit `\r\n` keine gemischten Zeilenenden bekommt. Ein Stand in
//! gehaltener Form (siehe [`super::datei`]) traegt kein `\r`, und die Antwort
//! ist fuer ihn immer `\n`.
//!
//! **Die Regel liest Zeichen und keine Saetze.** Die Handlungen einer
//! Trenntexttabelle (`crate::trenntext::handlung`) fragen deshalb nicht hier,
//! sondern ihren Satzindex: ein Umbruch in einer angefuehrten Zelle und das
//! Ende einer `sep=`-Zeile sind dort keine Satzenden, hier aber das erste
//! `\n` des Textes.

/// `\r\n`, wenn das erste `\n` des Textes hinter einem `\r` steht, sonst
/// `\n`.
///
/// „Sonst“ heisst auch: ohne jedes `\n`, im leeren Text und bei einzelnen
/// `\r` als einzigen Zeilenenden. Allein das erste `\n` entscheidet; ein
/// Text mit gemischten Zeilenenden bekommt die Form seiner ersten Zeile.
#[must_use]
pub fn des_textes(text: &str) -> &'static str {
    match text.find('\n') {
        Some(stelle) if text[..stelle].ends_with('\r') => "\r\n",
        _ => "\n",
    }
}

#[cfg(test)]
mod proben {
    use super::des_textes;

    #[test]
    fn das_erste_zeilenende_entscheidet() {
        assert_eq!(des_textes("a\r\nb\r\n"), "\r\n");
        assert_eq!(des_textes("a\nb\n"), "\n");
        // Gemischt: die Form der ersten Zeile gilt, gleich was folgt.
        assert_eq!(des_textes("a\r\nb\nc\n"), "\r\n");
        assert_eq!(des_textes("a\nb\r\nc\r\n"), "\n");
        // Das Zeilenende steht allein am Anfang.
        assert_eq!(des_textes("\r\n"), "\r\n");
        assert_eq!(des_textes("\n"), "\n");
    }

    #[test]
    fn ohne_ein_lf_hinter_einem_cr_bleibt_es_bei_lf() {
        assert_eq!(des_textes(""), "\n");
        assert_eq!(des_textes("ohne Umbruch"), "\n");
        // Einzelne `\r` sind kein `\r\n`, auch als einzige Zeilenenden.
        assert_eq!(des_textes("a\rb\r"), "\n");
        // Ein `\r` frueher in der Zeile zaehlt nicht, allein das vor dem `\n`.
        assert_eq!(des_textes("a\rb\nc\r\n"), "\n");
        assert_eq!(des_textes("a\rb\r\nc\n"), "\r\n");
    }
}
