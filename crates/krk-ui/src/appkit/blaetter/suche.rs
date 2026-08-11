//! Das Eingabeblatt der Suche und des Ersetzens (C5).
//!
//! ```text
//!  ┌ Suchen nach:    [____________________] ┐  zwei Eingabefelder,
//!  │ Ersetzen durch: [____________________] ┘  ein Eingabewaechter
//!  └ [Abbrechen]                    [Suche]
//! ```
//!
//! **Ein Blatt fuer beide Befehle.** Der Spec traegt Suchen und Ersetzen unter
//! einem Buchstaben (C5), und der Ersatztext gehoert zum Suchtext: `cmd+f`
//! fragt nach beiden, `cmd+g` und `ctrl+cmd+g` gehen durch die Treffer,
//! `shift+cmd+r` und `ctrl+cmd+r` setzen den Ersatz ein. Ein zweites Blatt
//! allein fuer den Ersatztext waere eine zweite Stelle, an der der Nutzer
//! dieselbe Suche noch einmal beschreiben muesste.
//!
//! **Dieses Blatt sucht nicht.** Es liefert zwei Zeichenketten; gesucht und
//! ersetzt wird in `krk_core::text::suche`, gehalten wird der Suchlauf in
//! `crate::editormodell`. Gross- und Kleinschreibung, regulaere Ausdruecke und
//! die Suchrichtung sind nach dem Spec **nicht** festgelegt und kommen nicht
//! hinzu; deshalb traegt das Blatt kein einziges Kaestchen. Jeder Schalter
//! waere ein Bedienelement und ein Abnahmekriterium mehr.
//!
//! **Ein Blatt haelt genau einen Eingabewaechter, auch bei zwei Feldern.** Der
//! Grund steht im Modulkopf von [`super`]: der Waechter entscheidet nicht nach
//! Feld, sondern beantwortet zwei Tasten, und die bedeuten in jedem Feld
//! dasselbe. Das Stapel-Umbenennen macht es mit vier Feldern vor.
//!
//! **Die beiden Startwerte kommen von der letzten Suche.** Wer `cmd+f` ein
//! zweites Mal drueckt, findet seinen Suchtext ausgewaehlt vor und tippt
//! entweder einen neuen oder bestaetigt den alten. Dieselbe Wahl und derselbe
//! Grund wie beim Startwert der Pfadeingabe.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField`, `NSView`, `NSWindow` und `NSString` stehen seit macOS 10.0
//! zur Verfuegung, ebenso die Aufzaehlung `NSTextAlignment` und die Zugriffe
//! `selectText:`, `setNextKeyView:`, `alignment`, `stringValue` und `frame`.
//!
//! **Eine einzige Beruehrung ist juenger als 10.0**, und sie liegt unter dem
//! Zielsystem: `labelWithString:` steht seit 10.12 (`NSTextField.h`). Alles
//! Weitere, was dieses Blatt braucht — das Aufgehen am Fenster, der
//! Eingabewaechter, die Schaltflaechen —, geht durch [`super::Blatt`]; die
//! Untergrenzen dazu nennt der Modulkopf von [`super`] und nicht dieser.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextAlignment, NSTextField, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use super::Blatt;

/// Die Breite der Beigabe in Punkten.
///
/// Sie bestimmt zugleich die Breite des Blattes: `NSAlert` waechst mit seiner
/// Beigabe.
const BREITE: f64 = 420.0;

/// Die Breite der Beschriftungsspalte links.
const BESCHRIFTUNG: f64 = 120.0;

/// Die Hoehe einer Eingabezeile.
const ZEILENHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen den beiden Eingabezeilen.
const ZEILENABSTAND: f64 = 6.0;

/// Die Hoehe einer Beschriftung.
const BESCHRIFTUNGSHOEHE: f64 = 17.0;

/// Der Abstand zwischen Beschriftung und Feld.
const SPALTENABSTAND: f64 = 8.0;

/// Zeigt die Frage nach Such- und Ersatztext am Fenster.
///
/// Kehrt sofort zurueck. `fertig` bekommt den Suchtext und den Ersatztext, in
/// dieser Reihenfolge, und laeuft auf dem Hauptfaden, wenn der Nutzer
/// bestaetigt hat; bricht er ab, laeuft es gar nicht.
///
/// **Beide Texte gehen unveraendert hinaus**, ohne `trim` und ohne Wandlung.
/// Ein fuehrendes Leerzeichen ist im Suchtext ein Zeichen wie jedes andere,
/// anders als in einem Pfad. Die eine Wandlung, die der Ersatztext braucht,
/// steht in `krk_core::text::datei::in_gehaltene_form` und wird vom Modell
/// vorgenommen — vor dem Ersetzen und nicht danach.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    gesucht: &str,
    ersatz: &str,
    fertig: impl Fn(String, String) + 'static,
) {
    let hoehe = 2.0f64.mul_add(ZEILENHOEHE, ZEILENABSTAND);
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, hoehe)),
    );

    // Von unten nach oben, weil AppKit von unten nach oben misst.
    let ersatzfeld = eingabezeile(mtm, &beigabe, "Ersetzen durch:", 0.0, ersatz);
    let suchfeld = eingabezeile(
        mtm,
        &beigabe,
        "Suchen nach:",
        ZEILENHOEHE + ZEILENABSTAND,
        gesucht,
    );

    // Der ganze Suchtext steht ausgewaehlt da: wer einen anderen sucht, tippt
    // ihn einfach, wer den vorhandenen ergaenzen will, drueckt zuerst Pfeil
    // rechts. Dieselbe Wahl wie bei der Pfadeingabe.
    // SAFETY: `selectText:` ist eine gewoehnliche Aktion von `NSControl`; sie
    // stellt keine Bedingung an ihren Absender, und `None` ist der Wert, den
    // ein programmatischer Aufruf dafuer setzt.
    unsafe { suchfeld.selectText(None) };

    // Der Ring, den der Tabulator abgeht. Er steht ausdruecklich da und wird
    // nicht AppKit ueberlassen, aus demselben Grund wie beim
    // Stapel-Umbenennen: die Reihenfolge, in der der Nutzer die Felder
    // ausfuellt, ist nicht die, in der sie in der Beigabe haengen.
    // SAFETY: `setNextKeyView:` verlangt vom Nachfolger allein, dass er eine
    // Ansicht ist und lebt. Beide haengen in der Beigabe und leben, solange das
    // Blatt steht; die Kette ist geschlossen und verweist auf keine Ansicht
    // ausserhalb.
    unsafe {
        suchfeld.setNextKeyView(Some(&ersatzfeld));
        ersatzfeld.setNextKeyView(Some(&suchfeld));
    }

    let mut blatt = Blatt::neu(mtm, "Wonach suchen?", "Suche");
    // Die drei Schritte einzeln und nicht ueber `textfeld_setzen`: die Beigabe
    // ist der Rahmen um die beiden Felder und nicht eines davon, und
    // Ersthelfer ist das Suchfeld. Der Waechter ist fuer beide derselbe.
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&suchfeld);
    blatt.waechter_anhaengen(mtm, &suchfeld);
    blatt.waechter_anhaengen(mtm, &ersatzfeld);

    let suchfeld: Retained<NSTextField> = suchfeld;
    let ersatzfeld: Retained<NSTextField> = ersatzfeld;
    blatt.zeigen(fenster, move |bestaetigt| {
        if bestaetigt {
            fertig(
                suchfeld.stringValue().to_string(),
                ersatzfeld.stringValue().to_string(),
            );
        }
    });
}

/// Eine beschriftete Eingabezeile, in die Beigabe gehaengt.
///
/// Derselbe Zuschnitt wie im Stapel-Umbenennen; die Ansichten bekommen feste
/// Rahmen und keine Auslegeregeln, weil die Beigabe eines `NSAlert` nicht mit
/// dem Fenster waechst.
fn eingabezeile(
    mtm: MainThreadMarker,
    beigabe: &NSView,
    beschriftung: &str,
    unterkante: f64,
    startwert: &str,
) -> Retained<NSTextField> {
    let text = NSTextField::labelWithString(&NSString::from_str(beschriftung), mtm);
    text.setFrame(NSRect::new(
        NSPoint::new(0.0, unterkante + 3.0),
        NSSize::new(BESCHRIFTUNG - SPALTENABSTAND, BESCHRIFTUNGSHOEHE),
    ));
    text.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&text);

    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(
            NSPoint::new(BESCHRIFTUNG, unterkante),
            NSSize::new(BREITE - BESCHRIFTUNG, ZEILENHOEHE),
        ),
    );
    feld.setStringValue(&NSString::from_str(startwert));
    beigabe.addSubview(&feld);
    feld
}
