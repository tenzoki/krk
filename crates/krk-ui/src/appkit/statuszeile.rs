//! Die Statuszeile am Fuss eines Dateifensters.
//!
//! Sie ist der einzige Weg, auf dem KRK dem Nutzer eine laufende Meldung zeigt.
//! Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: der
//! Ordner ohne Leserecht meldet sich hier, und allein der fehlende
//! Tastenabgriff bricht mit einem Hinweisfenster ab, weil er die Anwendung als
//! ganze betrifft.
//!
//! Vorher lief beides ueber `eprintln!`. Eine ueber den Finder gestartete
//! Anwendung hat keine Standardfehlerausgabe, LaunchServices haengt sie ins
//! Leere; in der einzigen Betriebsart, die die Abnahme zulaesst, war die
//! Fehlerbehandlung damit still. C1 verlangt seit dem 260804-0830
//! ausdruecklich, dass KRK keine Meldung an den Nutzer ueber die
//! Standardfehlerausgabe gibt.
//!
//! Seit dem 260804-1832 traegt sie eine zweite Art von Meldung: den Stand einer
//! laufenden Dateioperation aus C4. Der Nutzer hat den Fortschritt aus dem
//! Blatt hierher verlegt
//! (`decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`),
//! weil ein Blatt das Fenster sperrt, das C4 bedienbar zusagt, und auf dem
//! Referenzgeraet 354 bis 403 ms zum Aufgehen braucht, waehrend L8 200 ms
//! zusagt.
//!
//! **Die Art steht in der Signatur und nicht in einer zweiten Funktion.** Ein
//! Fortschritt ist kein Fehler und wird nicht rot; eine zweite Funktion neben
//! [`Statuszeile::zeigen`] waeren zwei Wahrheiten darueber, was in der Zeile
//! steht.
//!
//! Was diese Zeile in dieser Runde **nicht** traegt: den Lesefortschritt und die
//! Zahl der Eintraege. C1 sagt beides nicht zu; sie kommen in einer spaeteren
//! Runde in dieselbe Zeile und nicht in eine zweite daneben.
//!
//! # Wer die Zeile bekommt, wenn mehrere zugleich etwas zu sagen haben
//!
//! Die Auswahl steht in [`zeile`], einer Funktion ohne AppKit, damit sie
//! pruefbar ist. Die Lebensdauern der vier Quellen mit eigenem Feld stehen bei
//! ihren Feldern in `DateifensterQuelle`; hier steht allein die Rangfolge.
//!
//! **Der fuenfte Rang hat als einziger kein Feld.** Der Markierungsstand aus
//! C2 wird bei jedem Schreiben der Zeile aus dem Ordnermodell des sichtbaren
//! Tabs gerechnet, statt gesetzt und geloescht zu werden; die Begruendung
//! steht bei `DateifensterQuelle::markierungsstand_text`.

use objc2::rc::Retained;
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSFont, NSTextAlignment, NSTextField, NSView,
};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString, ns_string};

/// Die Hoehe der Zeile in Punkten.
///
/// Eine Zeile in der kleinen Systemschrift mit etwas Luft darum.
pub const HOEHE: f64 = 18.0;

/// Der Abstand vom linken Rand, damit der Text nicht an der Trennlinie klebt.
pub const EINZUG: f64 = 6.0;

/// Was fuer eine Meldung gerade in der Zeile steht.
///
/// Zwei Werte, weil die Zeile seit dem 260804-1832 zwei Sorten traegt. Sie
/// unterscheiden sich allein in der Farbe: ein Fehler ist rot, damit ihn der
/// Nutzer neben einer leeren Liste nicht uebersieht, und ein Fortschritt ist es
/// nicht, weil er keiner ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// Ein Fehler oder ein Hinweis, den der Nutzer bemerken soll.
    Fehler,
    /// Der Stand einer laufenden Dateioperation (C4).
    Vorgang,
}

/// Was von den fuenf Quellen jetzt in der Zeile steht.
///
/// **Die eine Regel, und kein Sonderfall je Meldungsart.** Die Zeile traegt
/// einen Text. Steht mehr als eine Aussage, gewinnt die, die dem letzten Tun
/// des Nutzers am naechsten ist:
///
/// ```text
/// 1  Befehlsantwort    was KRK auf einen Tastenbefehl zu sagen hat
/// 2  Vorgangsanzeige   der Stand einer laufenden Operation
/// 3  Fenstermeldung    ein Ereignis am Fenster, das niemand angefordert hat
/// 4  Tabmeldung        der Zustand des sichtbaren Ordners
/// 5  Markierungsstand  was im sichtbaren Tab markiert ist
/// ```
///
/// **Der Markierungsstand steht unter der Tabmeldung und nicht neben ihr.**
/// Beide beschreiben einen Zustand des sichtbaren Tabs, aber mit
/// verschiedenen Lebensdauern: die Tabmeldung traegt einen Ordner, der sich
/// nicht lesen liess, und muss stehen bleiben, waehrend der Nutzer markiert
/// und die Markierung wieder aufhebt. Beide in ein Feld zu legen gaebe diesem
/// Feld zwei Loeschregeln, und das ist der Sonderfall, den diese Datei fuer
/// Befehlsantwort und Fenstermeldung schon einmal ausgeschlossen hat. Unter
/// der Tabmeldung steht er, weil ein nicht lesbarer Ordner ein Fehler ist und
/// eine Markierungszahl keiner; er ist der Ruhezustand der Zeile, und ein
/// Ruhezustand ist der unterste Rang.
///
/// Das ist dieselbe Ordnung, die S14 zwischen Fenster- und Tabmeldung gezogen
/// hat ("ein Ereignis ist neuer als ein Zustand"), zu Ende gefuehrt: eine
/// laufende Operation ist neuer als ein Ereignis, und die Antwort auf einen
/// Tastendruck, den der Nutzer eben gemacht hat, ist neuer als beides. S16b
/// hatte sie mit drei Raengen gebaut und die Befehlsantwort in die
/// Fenstermeldung gelegt; dort verschwand die Meldung "es laeuft bereits eine
/// Operation" hinter dem Fortschritt desselben Dateifensters
/// (`issues/260804-1915_o_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`).
///
/// **Verdraengt wird nichts geloescht.** Jede der vier oberen Quellen haelt
/// ihren Text in ihrem eigenen Feld, und jedes Feld hat genau eine
/// Loeschregel; der fuenfte Rang wird gerechnet und kann deshalb gar nicht
/// veralten. Eine verdraengte Aussage erscheint, sobald alles ueber ihr
/// gefallen ist: die Auswurfmeldung, die waehrend einer Kopie eintrifft, steht
/// auf Rang 3, wartet die Kopie und deren Abschlusstext (Rang 1) ab und ist mit
/// dem naechsten Tastenbefehl in der Zeile. Ein Zeitgeber ist dafuer nicht
/// noetig, weil jede Lebensdauer an einem Ereignis haengt und an keiner Uhr.
///
/// Die Art faellt mit dem Rang: Vorgangsanzeige und Markierungsstand sind
/// keine Fehler, die drei uebrigen sind welche.
pub fn zeile<'a>(
    befehlsantwort: Option<&'a str>,
    vorgangsanzeige: Option<&'a str>,
    fenstermeldung: Option<&'a str>,
    tabmeldung: Option<&'a str>,
    markierungsstand: Option<&'a str>,
) -> Option<(&'a str, Art)> {
    befehlsantwort
        .map(|text| (text, Art::Fehler))
        .or_else(|| vorgangsanzeige.map(|text| (text, Art::Vorgang)))
        .or_else(|| fenstermeldung.map(|text| (text, Art::Fehler)))
        .or_else(|| tabmeldung.map(|text| (text, Art::Fehler)))
        .or_else(|| markierungsstand.map(|text| (text, Art::Vorgang)))
}

/// Die Textzeile am Fuss eines Dateifensters.
pub struct Statuszeile {
    feld: Retained<NSTextField>,
}

impl Statuszeile {
    /// Baut eine leere Statuszeile.
    ///
    /// Sie entsteht ohne Groesse und bekommt ihre erste beim Einhaengen; die
    /// Autogroesse haelt sie danach am unteren Rand ueber die volle Breite.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
        let feld = NSTextField::labelWithString(ns_string!(""), mtm);
        feld.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(0.0, HOEHE)));
        feld.setFont(Some(&NSFont::systemFontOfSize(
            NSFont::smallSystemFontSize(),
        )));
        feld.setTextColor(Some(&NSColor::secondaryLabelColor()));
        feld.setAlignment(NSTextAlignment::Left);
        feld.setMaximumNumberOfLines(1);
        // Am unteren Rand festgemacht, in der Breite mitwachsend: der Abstand
        // nach oben ist beweglich, der nach unten nicht.
        feld.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMaxYMargin,
        );
        Self { feld }
    }

    /// Die Ansicht, die in das Dateifenster gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.feld
    }

    /// Zeigt eine Meldung der genannten Art an, oder leert die Zeile bei
    /// `None`.
    ///
    /// Ein Fehler faerbt die Zeile rot: ein Ordner ohne Leserecht ist ein
    /// Fehler und kein Hinweis, und eine graue Zeile am Fuss uebersieht der
    /// Nutzer neben einer leeren Liste genauso, wie er die
    /// Standardfehlerausgabe uebersehen hat. Ein Fortschritt bekommt die
    /// gewoehnliche Textfarbe; auffindbar ist der Abbruch bei ihm nicht ueber
    /// die Farbe, sondern weil die Zeile ihn benennt ("Esc bricht ab").
    pub fn zeigen(&self, meldung: Option<(&str, Art)>) {
        match meldung {
            Some((text, art)) => {
                self.feld.setStringValue(&NSString::from_str(text));
                let farbe = match art {
                    Art::Fehler => NSColor::systemRedColor(),
                    Art::Vorgang => NSColor::labelColor(),
                };
                self.feld.setTextColor(Some(&farbe));
            }
            None => {
                self.feld.setStringValue(ns_string!(""));
                self.feld
                    .setTextColor(Some(&NSColor::secondaryLabelColor()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Art, zeile};

    #[test]
    fn ohne_jede_quelle_bleibt_die_zeile_leer() {
        assert_eq!(zeile(None, None, None, None, None), None);
    }

    #[test]
    fn jede_quelle_steht_fuer_sich_allein_in_der_zeile() {
        assert_eq!(
            zeile(Some("Antwort"), None, None, None, None),
            Some(("Antwort", Art::Fehler))
        );
        assert_eq!(
            zeile(None, Some("Vorgang"), None, None, None),
            Some(("Vorgang", Art::Vorgang))
        );
        assert_eq!(
            zeile(None, None, Some("Ereignis"), None, None),
            Some(("Ereignis", Art::Fehler))
        );
        assert_eq!(
            zeile(None, None, None, Some("Zustand"), None),
            Some(("Zustand", Art::Fehler))
        );
    }

    /// Der Defekt vom 260804-1915: der zweite F5 meldete sich in dem
    /// Dateifenster, in dem der Fortschritt stand, und war dort unsichtbar.
    #[test]
    fn die_befehlsantwort_steht_ueber_dem_laufenden_vorgang() {
        assert_eq!(
            zeile(
                Some("es läuft bereits eine Operation: Kopieren"),
                Some("Kopieren: 8.189 Einträge …"),
                None,
                None,
                None
            ),
            Some(("es läuft bereits eine Operation: Kopieren", Art::Fehler))
        );
    }

    #[test]
    fn der_laufende_vorgang_steht_ueber_ereignis_und_zustand() {
        assert_eq!(
            zeile(
                None,
                Some("Kopieren: 8.189 Einträge …"),
                Some("Datenträger ausgeworfen"),
                Some("Ordner nicht lesbar"),
                Some("12 markiert, davon 3 Ordner, 4,2 MB")
            ),
            Some(("Kopieren: 8.189 Einträge …", Art::Vorgang))
        );
    }

    #[test]
    fn das_ereignis_am_fenster_steht_ueber_dem_zustand_des_ordners() {
        assert_eq!(
            zeile(
                None,
                None,
                Some("Datenträger ausgeworfen"),
                Some("Ordner nicht lesbar"),
                Some("12 markiert, davon 3 Ordner, 4,2 MB")
            ),
            Some(("Datenträger ausgeworfen", Art::Fehler))
        );
    }

    /// Der Defekt vom 260804-1915: der Abschlusstext ueberschrieb die waehrend
    /// der Kopie eingetroffene Auswurfmeldung. Er verdeckt sie jetzt, und sie
    /// steht wieder da, sobald er mit dem naechsten Befehl faellt.
    #[test]
    fn die_verdraengte_auswurfmeldung_erscheint_nach_dem_abschlusstext() {
        let auswurf = "Datenträger „Sicherung“ wurde ausgeworfen";
        let abschluss = "Kopieren abgebrochen: 9.175 Einträge übertragen";
        // Waehrend der Kopie: der Fortschritt gewinnt, die Auswurfmeldung
        // bleibt in ihrem Feld stehen.
        assert_eq!(
            zeile(
                None,
                Some("Kopieren: 9.131 Einträge …"),
                Some(auswurf),
                None,
                None
            ),
            Some(("Kopieren: 9.131 Einträge …", Art::Vorgang))
        );
        // Unmittelbar nach dem Bericht: der Abschlusstext ist die Antwort auf
        // den Befehl und steht oben.
        assert_eq!(
            zeile(Some(abschluss), None, Some(auswurf), None, None),
            Some((abschluss, Art::Fehler))
        );
        // Der naechste Tastenbefehl raeumt die Antwort weg; jetzt ist die
        // Auswurfmeldung an der Reihe, statt verloren zu sein.
        assert_eq!(
            zeile(None, None, Some(auswurf), None, None),
            Some((auswurf, Art::Fehler))
        );
    }

    /// Der fuenfte Rang aus S16c: er steht unter allen vieren.
    #[test]
    fn der_markierungsstand_steht_hinter_der_tabmeldung() {
        let markiert = "12 markiert, davon 3 Ordner, 4,2 MB";
        assert_eq!(
            zeile(
                None,
                None,
                None,
                Some("Ordner nicht lesbar"),
                Some(markiert)
            ),
            Some(("Ordner nicht lesbar", Art::Fehler)),
            "ein nicht lesbarer Ordner ist wichtiger als eine Markierungszahl"
        );
        assert_eq!(
            zeile(None, None, None, None, Some(markiert)),
            Some((markiert, Art::Vorgang)),
            "ohne Tabmeldung steht der Markierungsstand in der Zeile"
        );
    }

    /// Eine Markierungszahl ist kein Fehler und wird deshalb nicht rot.
    #[test]
    fn der_markierungsstand_gilt_nicht_als_fehler() {
        let (_, art) = zeile(
            None,
            None,
            None,
            None,
            Some("3 markiert, davon 0 Ordner, 6 KB"),
        )
        .expect("der Markierungsstand steht als einzige Quelle in der Zeile");
        assert_eq!(art, Art::Vorgang);
        assert_ne!(art, Art::Fehler);
    }
}
