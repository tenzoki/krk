//! Wohin eine Textmarke fuehrt, nachdem die Datei sich geaendert hat (C6).
//!
//! ```text
//!  wiederfinden(stand, gemerkte Nummer, gemerkter Inhalt) ──> Markensprung
//!         │
//!         ├── Inhalt steht auf der Nummer ───────> Fund::Getroffen
//!         ├── Inhalt steht im Fenster ───────────> Fund::Verschoben
//!         └── Inhalt steht nicht im Fenster ─────> Fund::NichtGefunden
//!                                                  (Sprung an die Nummer)
//! ```
//!
//! # Die Regel und woher sie kommt
//!
//! Eine Textmarke haengt an einer Zeilennummer und dem Inhalt jener Zeile als
//! Probe. Der Sprung geht zur gemerkten Nummer, prueft den Inhalt und sucht
//! bei Abweichung in einem **festen Fenster von [`NAHFENSTER`] Zeilen in beide
//! Richtungen**. Wird der Inhalt dort nicht gefunden, springt die Marke
//! trotzdem, naemlich an die gemerkte Nummer, und der Aufrufer meldet in der
//! Statuszeile, dass die Stelle sich geaendert hat.
//!
//! Entschieden vom Nutzer am 260808-0017
//! (`decisions/260807-2147_*_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`,
//! Moeglichkeit 1). Der tragende Grund war nicht die Trefferquote, sondern die
//! Gueltigkeitspruefung der Leiste: **ungueltig heisst allein, dass die Datei
//! fehlt.** Die Leiste stellt diese Frage bei jedem Neuaufbau ihrer Liste fuer
//! jede Marke, und eine Antwort, die dafuer jede gemerkte Datei oeffnen und
//! lesen muesste, machte aus einer Frage an das Dateisystem einen Lesevorgang
//! je Marke. Deshalb steht die Suche hier und nicht in
//! [`crate::ablage::lesezeichen`]: dort wird gefragt, ob die Datei da ist,
//! hier wird gefragt, wo die Stelle hin ist, und das Zweite geschieht beim
//! Sprung und nur dort.
//!
//! # Der gemerkte Zeileninhalt ist keine eindeutige Kennung
//!
//! Das ist eine **Grenze der Faehigkeit** und keine Luecke der Umsetzung, und
//! sie ist ausgeschrieben statt verdeckt. Eine Marke auf einer Zeile, die in
//! der Datei mehrfach steht — eine schliessende Klammer, eine Leerzeile, ein
//! wiederkehrender Kopfkommentar —, kann nach einer Aenderung von aussen
//! **nicht zuverlaessig** wiedergefunden werden. Die Suche liefert dann den
//! Treffer, der der gemerkten Nummer am naechsten liegt, und das ist eine
//! nachvollziehbare Antwort, aber nicht notwendig die gemeinte. Keine
//! Reichweite und keine Suchrichtung aendert daran etwas; nur eine andere
//! Kennung taete es, und die Runde hat sich gegen eine solche entschieden
//! (Moeglichkeit 3 des Datensatzes haette drei Zeilen gemerkt und
//! `bookmarks.toml` dreimal so gross gemacht).
//!
//! # Warum hier nicht [`crate::text::suche`] steht
//!
//! [`crate::text::suche::alle`] findet **Teilzeichenfolgen**. Eine Marke
//! vergleicht **ganze Zeilen**: der gemerkte Inhalt `let x = 1;` darf nicht
//! auf eine Zeile treffen, die ihn nur enthaelt. Beide Module suchen deshalb
//! nach verschiedenen Dingen, und das eine ist kein Sonderfall des anderen.
//! Was dieses Modul **nicht** selbst weiss, ist, wo eine Zeile anfaengt und
//! aufhoert: das fragt es [`Zeilenindex::inhalt_der_zeile`], damit es keine
//! zweite Meinung darueber gibt, was eine Zeile beendet.

use super::zeilen::{Zeilenindex, Zeilensprung};

/// Wie weit die Suche in der Naehe reicht: so viele Zeilen in **jede**
/// Richtung um die gemerkte Nummer herum.
///
/// `inference:` Fuenfzig ist ein Vorschlag und keine gemessene Groesse. Sie
/// deckt die haeufige Aenderung ab, naemlich einen eingefuegten oder
/// geloeschten Abschnitt oberhalb der Marke, und verfehlt die seltene, naemlich
/// eine umgebaute Datei. **Wer die Zahl aendert, aendert eine Konstante und
/// keine Regel** — deshalb steht sie hier als Konstante und nicht als Literal
/// in [`wiederfinden`].
pub const NAHFENSTER: u32 = 50;

/// Ob die Marke ihre Stelle wiedergefunden hat.
///
/// Die Aufzaehlung ist vollstaendig und hat keinen Auffangzweig: der gemerkte
/// Inhalt stand auf der gemerkten Nummer, er stand im Fenster daneben, oder er
/// stand im Fenster nirgends. Ein vierter Fall entstuende nur mit einer
/// vierten Reichweite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fund {
    /// Der gemerkte Inhalt stand auf der gemerkten Nummer. Die Marke hat die
    /// Aenderung von aussen ueberlebt, oder es gab keine.
    Getroffen,
    /// Er stand nicht dort, wurde aber im Fenster wiedergefunden. Die Marke
    /// fuehrt an die gefundene Zeile.
    Verschoben,
    /// Er war im Fenster nirgends zu finden. Die Marke springt **trotzdem**,
    /// naemlich an die gemerkte Nummer, und C6 verlangt, dass der Aufrufer in
    /// der Statuszeile meldet, dass die Stelle sich geaendert hat, statt
    /// kommentarlos irgendwohin zu fuehren.
    NichtGefunden,
}

/// Wohin eine Textmarke fuehrt.
///
/// [`Markensprung::fund`] und die Lage in [`Markensprung::sprung`] sind zwei
/// **verschiedene** Auskuenfte und keine zwei Zweige derselben: die erste sagt,
/// ob der gemerkte Inhalt wiedergefunden wurde, die zweite, ob die
/// angesteuerte Zeilennummer im Text ueberhaupt vorkommt. Eine Marke auf Zeile
/// 500 einer inzwischen auf 100 Zeilen gekuerzten Datei traegt beide, und der
/// Aufrufer hat beides zu melden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Markensprung {
    /// Die Zeilennummer, auf die die Schreibmarke gehoert, ab 1 gezaehlt.
    pub zeile: u32,
    /// Versatz und Lage dieser Zeile.
    ///
    /// Er kommt aus [`Zeilenindex::anfang_der_zeile`] und aus keinem zweiten
    /// Weg: eine gemerkte Nummer ueber der Zeilenzahl landet damit am
    /// Dateiende, genau wie der Zeilensprung aus C5, und die Regel dafuer steht
    /// in [`super::zeilen`] und nur dort.
    pub sprung: Zeilensprung,
    /// Ob der gemerkte Inhalt dort stand, verschoben wiedergefunden wurde oder
    /// fehlt.
    pub fund: Fund,
}

/// Sucht die gemerkte Stelle im gehaltenen Stand und liefert, wohin die
/// Schreibmarke gehoert (C6).
///
/// `zeile` und `zeileninhalt` sind die beiden gemerkten Felder einer
/// Textmarke (`crate::ablage::lesezeichen::Ziel::Textstelle`); sie kommen als
/// einzelne Werte herein und nicht als Lesezeichen, damit die Textrechnung die
/// Ablage nicht kennen muss.
///
/// Die Reihenfolge ist bindend:
///
/// 1. Steht der gemerkte Inhalt auf der gemerkten Nummer, trifft die Marke
///    sofort. Keine Suche laeuft an, auch dann nicht, wenn derselbe Inhalt
///    daneben noch einmal steht.
/// 2. Sonst wird von der gemerkten Nummer aus nach aussen gesucht, Abstand fuer
///    Abstand, bis [`NAHFENSTER`]. **Der naechstgelegene Treffer gewinnt**;
///    bei gleichem Abstand nach oben und nach unten gewinnt die kleinere
///    Nummer, also der Treffer, der in der Datei zuerst steht. Diese Wahl ist
///    willkuerlich und nur deshalb festgelegt, damit sie wiederholbar ist: bei
///    gleichem Abstand gibt es keine bessere Antwort, und das ist dieselbe
///    Mehrdeutigkeit, die der Modulkopf als Grenze der Faehigkeit benennt.
/// 3. Bleibt die Suche ohne Treffer, fuehrt die Marke an die gemerkte Nummer,
///    mit [`Fund::NichtGefunden`].
///
/// Der Zeilenindex entsteht hier und wird nicht hereingereicht: ein Sprung
/// geschieht einmal je Tastendruck, und ein hereingereichter Index koennte zu
/// einem anderen Text gehoeren als der uebergebene.
pub fn wiederfinden(text: &str, zeile: u32, zeileninhalt: &str) -> Markensprung {
    let index = Zeilenindex::neu(text);
    let steht_auf =
        |nummer: u32| index.inhalt_der_zeile(text, nummer as usize) == Some(zeileninhalt);
    let sprung_auf = |nummer: u32, fund: Fund| Markensprung {
        zeile: nummer,
        sprung: index.anfang_der_zeile(nummer as usize),
        fund,
    };

    if steht_auf(zeile) {
        return sprung_auf(zeile, Fund::Getroffen);
    }
    for abstand in 1..=NAHFENSTER {
        // Erst nach oben, dann nach unten: das ist die Reihenfolge, in der die
        // beiden gleich weit entfernten Zeilen in der Datei stehen. Eine
        // Nummer unter 1 entfaellt ueber `checked_sub`, eine ueber der
        // Zeilenzahl beantwortet `inhalt_der_zeile` mit `None`.
        for nummer in [zeile.checked_sub(abstand), zeile.checked_add(abstand)]
            .into_iter()
            .flatten()
        {
            if steht_auf(nummer) {
                return sprung_auf(nummer, Fund::Verschoben);
            }
        }
    }
    sprung_auf(zeile, Fund::NichtGefunden)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::zeilen::Zeilenlage;

    /// Ein Text mit durchnummerierten Zeilen, damit jede Zeile ihren eigenen
    /// Inhalt hat und ein Treffer eindeutig ist.
    fn text_mit(zeilen: usize) -> String {
        (1..=zeilen)
            .map(|nummer| format!("Zeile {nummer}\n"))
            .collect()
    }

    /// Schiebt den Text um `zahl` Zeilen nach unten, indem oben Zeilen
    /// eingefuegt werden — die haeufige Aenderung von aussen.
    fn nach_unten_geschoben(text: &str, zahl: usize) -> String {
        let kopf: String = (1..=zahl).map(|_| "neu\n".to_owned()).collect();
        format!("{kopf}{text}")
    }

    #[test]
    fn eine_unveraenderte_datei_trifft_sofort() {
        let text = text_mit(200);
        let ergebnis = wiederfinden(&text, 118, "Zeile 118");
        assert_eq!(ergebnis.fund, Fund::Getroffen);
        assert_eq!(ergebnis.zeile, 118);
        assert_eq!(ergebnis.sprung.lage, Zeilenlage::Getroffen);
    }

    #[test]
    fn eine_um_zehn_zeilen_verschobene_stelle_wird_gefunden() {
        let text = nach_unten_geschoben(&text_mit(200), 10);
        let ergebnis = wiederfinden(&text, 118, "Zeile 118");
        assert_eq!(ergebnis.fund, Fund::Verschoben);
        assert_eq!(ergebnis.zeile, 128);
        let index = Zeilenindex::neu(&text);
        assert_eq!(index.inhalt_der_zeile(&text, 128), Some("Zeile 118"));
    }

    /// Sechzig Zeilen liegen ausserhalb von [`NAHFENSTER`]. Die Marke springt
    /// trotzdem, naemlich an die gemerkte Nummer, und traegt das Kennzeichen.
    #[test]
    fn eine_um_sechzig_zeilen_verschobene_stelle_wird_nicht_gefunden() {
        let text = nach_unten_geschoben(&text_mit(200), 60);
        let ergebnis = wiederfinden(&text, 118, "Zeile 118");
        assert_eq!(ergebnis.fund, Fund::NichtGefunden);
        assert_eq!(
            ergebnis.zeile, 118,
            "die Marke springt an die gemerkte Nummer"
        );
        assert_eq!(ergebnis.sprung.lage, Zeilenlage::Getroffen);
    }

    /// Die Grenze der Faehigkeit, an einer Probe festgemacht: ein Inhalt, der
    /// im Fenster zweimal steht, liefert den der gemerkten Nummer
    /// naechstliegenden — nachvollziehbar, aber nicht notwendig den gemeinten.
    #[test]
    fn kommt_der_inhalt_im_fenster_zweimal_vor_gewinnt_der_naechstliegende() {
        let mut text = text_mit(200);
        text = text.replace("Zeile 100\n", "}\n");
        text = text.replace("Zeile 130\n", "}\n");
        // Gemerkt war die Zeile 130; die 100 liegt im Fenster, aber weiter weg.
        let ergebnis = wiederfinden(&text, 125, "}");
        assert_eq!(ergebnis.fund, Fund::Verschoben);
        assert_eq!(ergebnis.zeile, 130);
    }

    /// Bei gleichem Abstand nach oben und nach unten gewinnt die kleinere
    /// Nummer. Die Wahl ist willkuerlich, aber festgelegt und wiederholbar.
    #[test]
    fn bei_gleichem_abstand_gewinnt_die_kleinere_nummer() {
        let mut text = text_mit(200);
        text = text.replace("Zeile 120\n", "}\n");
        text = text.replace("Zeile 130\n", "}\n");
        let ergebnis = wiederfinden(&text, 125, "}");
        assert_eq!(ergebnis.zeile, 120);
    }

    /// Das fuenfte Abnahmekriterium: eine gemerkte Nummer ueber der Zeilenzahl
    /// fuehrt an das Dateiende, und zwar ueber **dieselbe** Funktion wie der
    /// Zeilensprung aus C5. Die Probe prueft beide Wege gegeneinander.
    #[test]
    fn eine_nummer_ueber_der_zeilenzahl_landet_am_dateiende_wie_der_zeilensprung() {
        let text = text_mit(100);
        let ergebnis = wiederfinden(&text, 500, "Zeile 500");
        let zeilensprung = Zeilenindex::neu(&text).anfang_der_zeile(500);

        assert_eq!(ergebnis.sprung, zeilensprung, "kein zweiter Weg daneben");
        assert_eq!(ergebnis.sprung.lage, Zeilenlage::HinterDerLetzten);
        assert_eq!(ergebnis.sprung.versatz, text.len());
        assert_eq!(ergebnis.fund, Fund::NichtGefunden);
        assert_eq!(ergebnis.zeile, 500);
    }

    /// Steht der Inhalt auf der gemerkten Nummer, laeuft keine Suche an — auch
    /// dann nicht, wenn derselbe Inhalt daneben noch einmal steht.
    #[test]
    fn ein_treffer_auf_der_gemerkten_nummer_schlaegt_jeden_nachbarn() {
        let mut text = text_mit(200);
        text = text.replace("Zeile 124\n", "}\n");
        text = text.replace("Zeile 125\n", "}\n");
        let ergebnis = wiederfinden(&text, 125, "}");
        assert_eq!(ergebnis.fund, Fund::Getroffen);
        assert_eq!(ergebnis.zeile, 125);
    }

    /// Der Rand des Fensters, an beiden Enden geprueft: genau [`NAHFENSTER`]
    /// Zeilen weit wird gefunden, eine Zeile weiter nicht mehr.
    #[test]
    fn das_fenster_reicht_genau_fuenfzig_zeilen_weit() {
        let text = nach_unten_geschoben(&text_mit(200), NAHFENSTER as usize);
        assert_eq!(wiederfinden(&text, 118, "Zeile 118").fund, Fund::Verschoben);

        let text = nach_unten_geschoben(&text_mit(200), NAHFENSTER as usize + 1);
        assert_eq!(
            wiederfinden(&text, 118, "Zeile 118").fund,
            Fund::NichtGefunden
        );
    }

    /// Eine gemerkte Nummer 0 kommt aus keinem Anlegen, wohl aber aus einer von
    /// Hand geaenderten `bookmarks.toml`. Sie ist kein Fehler: der Sprung fuehrt
    /// an den Textanfang, und die Suche laeuft trotzdem.
    #[test]
    fn eine_gemerkte_nummer_null_fuehrt_an_den_textanfang_und_sucht_trotzdem() {
        let text = text_mit(200);
        let ohne_treffer = wiederfinden(&text, 0, "kommt nicht vor");
        assert_eq!(ohne_treffer.fund, Fund::NichtGefunden);
        assert_eq!(ohne_treffer.sprung.lage, Zeilenlage::VorDerErsten);
        assert_eq!(ohne_treffer.sprung.versatz, 0);

        let mit_treffer = wiederfinden(&text, 0, "Zeile 3");
        assert_eq!(mit_treffer.fund, Fund::Verschoben);
        assert_eq!(mit_treffer.zeile, 3);
    }

    /// Umlaute und Emojis: der Vergleich laeuft ueber ganze Zeilen, und der
    /// gelieferte Versatz liegt auf einer Zeichengrenze, wie der Modulkopf von
    /// [`crate::text`] es fuer jeden Versatz zusagt.
    #[test]
    fn der_versatz_liegt_auch_bei_mehrbytezeichen_auf_einer_zeichengrenze() {
        let text = "Größe\n🚀 Start\nEnde\n";
        let ergebnis = wiederfinden(text, 1, "🚀 Start");
        assert_eq!(ergebnis.fund, Fund::Verschoben);
        assert_eq!(ergebnis.zeile, 2);
        assert!(text.is_char_boundary(ergebnis.sprung.versatz));
        assert_eq!(&text[ergebnis.sprung.versatz..], "🚀 Start\nEnde\n");
    }
}
