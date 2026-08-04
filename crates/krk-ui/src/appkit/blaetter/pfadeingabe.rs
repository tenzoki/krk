//! Das Eingabeblatt der Pfadeingabe (C2).
//!
//! Es baut auf der Huelle aus [`super::Blatt`] auf und traegt allein das
//! Textfeld. Was mit dem eingegebenen Pfad geschieht, steht nicht hier: das
//! prueft und navigiert [`crate::kommandos::pfadeingabe`], die eine Stelle, die
//! das tut, und dieselbe, die der Sprung aus der Zwischenablage benutzt.
//!
//! Der Startwert ist der Ordner, den das Dateifenster gerade zeigt. Der Finder
//! macht es bei "Gehe zum Ordner" ebenso, und es spart dem Nutzer den
//! haeufigsten Fall, einen Nachbarordner zu erreichen.

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextField, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use super::Blatt;

/// Die Breite des Eingabefeldes in Punkten.
///
/// Sie bestimmt zugleich die Breite des Blattes: `NSAlert` waechst mit seiner
/// Beigabe. Ein absoluter Pfad ist lang, und ein Feld, in dem der Anfang
/// wegrollt, waere schwer zu pruefen.
const FELDBREITE: f64 = 420.0;

/// Die Hoehe einer Zeile im Eingabefeld in Punkten.
const FELDHOEHE: f64 = 24.0;

/// Zeigt die Pfadeingabe am Fenster und liefert den eingegebenen Pfad.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden, wenn der Nutzer
/// bestaetigt hat; bricht er ab, laeuft es gar nicht. Der Abbruch ist damit
/// kein Sonderfall mit eigener Meldung: er ist die Abwesenheit einer Eingabe.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    startwert: &str,
    fertig: impl Fn(String) + 'static,
) {
    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );
    feld.setStringValue(&NSString::from_str(startwert));
    // Der ganze Startwert steht ausgewaehlt da: wer einen anderen Pfad
    // eingeben will, tippt ihn einfach, wer den vorhandenen ergaenzen will,
    // drueckt zuerst Pfeil rechts.
    // SAFETY: `selectText:` ist eine gewoehnliche Aktion von `NSControl`; sie
    // stellt keine Bedingung an ihren Absender, und `None` ist der Wert, den
    // ein programmatischer Aufruf dafuer setzt.
    unsafe { feld.selectText(None) };

    let mut blatt = Blatt::neu(mtm, "Zu welchem Ordner?", "Gehe");
    blatt.textfeld_setzen(mtm, &feld);

    let feld: Retained<NSTextField> = feld;
    blatt.zeigen(fenster, move |bestaetigt| {
        if bestaetigt {
            fertig(feld.stringValue().to_string());
        }
    });
}
