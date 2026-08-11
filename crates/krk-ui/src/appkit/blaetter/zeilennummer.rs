//! Das Eingabeblatt des Zeilensprungs (C5).
//!
//! Es baut auf der Huelle aus [`super::Blatt`] auf und traegt allein das
//! Textfeld, wie die Pfadeingabe aus C2 der Runde 1. Was mit der eingegebenen
//! Nummer geschieht, steht nicht hier: gerechnet wird in
//! `krk_core::text::zeilen`, und der Sprung selbst liegt bei
//! `crate::appkit::editor::Editorbereich::zeile_anspringen`, weil dort die
//! Textflaeche und der gehaltene Stand beieinander liegen.
//!
//! **Das Feld beginnt leer.** Der Startwert der Pfadeingabe spart dem Nutzer
//! den haeufigsten Fall, einen Nachbarordner zu erreichen; bei einer
//! Zeilennummer gibt es keinen solchen Fall. Die Nummer, auf der die
//! Schreibmarke gerade steht, waere der einzige Kandidat, und sie
//! vorzuschlagen hiesse, den haeufigsten Wunsch — irgendwo anders hin — mit
//! einer Zahl zu belegen, die der Nutzer erst wieder loeschen muss.
//!
//! **Solange das Blatt steht, gilt der Fokusvorbehalt des Ereignisabgriffs
//! unveraendert**, und das ist richtig so: Ersthelfer ist dann der Feldeditor
//! dieses Textfeldes und nicht die Textflaeche des Editors, die
//! Naemlichkeitsfrage aus S4 antwortet mit `false`, und die Befehle des Editors
//! wirken hier nicht. Das siebte Abnahmekriterium von C7 faellt daraus an und
//! ist nicht eigens gebaut.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField` (ueber `NSControl`, `NSView` und `NSResponder`) und
//! `NSWindow` stehen seit macOS 10.0 zur Verfuegung, ebenso `alloc`,
//! `initWithFrame:` und `stringValue`. `NSPoint`, `NSRect` und `NSSize` sind
//! blosse Strukturen und tragen keine Verfuegbarkeitsangabe;
//! `MainThreadMarker` gehoert `objc2` und nicht AppKit. Das Buendel zielt auf
//! 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS 15
//! hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme. Was `NSAlert` betrifft,
//! steht im Kopf von [`Blatt`].

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextField, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize};

use super::Blatt;

/// Die Breite des Eingabefeldes in Punkten.
///
/// Sie bestimmt zugleich die Breite des Blattes: `NSAlert` waechst mit seiner
/// Beigabe. Schmaler als das Feld der Pfadeingabe, weil eine Zeilennummer kurz
/// ist; die Frage darueber haelt das Blatt trotzdem breit genug, um in einer
/// Zeile zu stehen.
const FELDBREITE: f64 = 200.0;

/// Die Hoehe einer Zeile im Eingabefeld in Punkten.
const FELDHOEHE: f64 = 24.0;

/// Zeigt die Frage nach der Zeilennummer am Fenster.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden, wenn der Nutzer
/// bestaetigt hat; bricht er ab, laeuft es gar nicht. Der Abbruch ist damit
/// kein Sonderfall mit eigener Meldung: er ist die Abwesenheit einer Eingabe,
/// wie bei der Pfadeingabe.
pub fn zeigen(mtm: MainThreadMarker, fenster: &NSWindow, fertig: impl Fn(String) + 'static) {
    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );

    let mut blatt = Blatt::neu(mtm, "Zu welcher Zeile?", "Springe");
    blatt.textfeld_setzen(mtm, &feld);

    let feld: Retained<NSTextField> = feld;
    blatt.zeigen(fenster, move |bestaetigt| {
        if bestaetigt {
            fertig(feld.stringValue().to_string());
        }
    });
}
