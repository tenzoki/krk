//! Das Eingabeblatt fuer einen Namen (C4, Anlegen).
//!
//! **Ein Blatt fuer beide Anlegebefehle.** `f7` und `shift+cmd+n` legen einen
//! Ordner an, `ctrl+cmd+n` eine leere Datei; beide fragen dieselbe Frage, und
//! beide pruefen den Namen mit derselben Funktion. Zwei Blaetter dafuer waeren
//! zwei Wahrheiten darueber, was ein zulaessiger Name ist. Was mit dem Namen
//! geschieht, entscheidet der Befehl und nicht dieses Blatt: es liefert einen
//! gewoehnlichen Rust-Wert zurueck.
//!
//! ```text
//!  Kommando::OrdnerAnlegen ─┐
//!                           ├──> namenseingabe::zeigen ──> Result<String, Namensfehler>
//!  Kommando::DateiAnlegen ──┘                                      │
//!                       operation::ordner_anlegen / datei_anlegen <┘
//! ```
//!
//! # Warum das Blatt prueft, obwohl das Anlegen es auch tut
//!
//! [`name_pruefen`] laeuft hier **und** in `operation::anlegen`; das ist keine
//! zweite Pruefung, sondern dieselbe an zwei Stellen des Weges. Der Gewinn ist
//! der Grund im Klartext: `ordner_anlegen` liefert einen [`std::io::Error`],
//! und daraus laesst sich nicht mehr ablesen, ob der Name leer war oder einen
//! Schraegstrich trug. Der Nutzer liest hier "der Name ist leer" statt
//! "ungueltige Eingabe".
//!
//! Der Abbruch ist kein Sonderfall mit eigener Meldung, wie bei der
//! Pfadeingabe: er ist die Abwesenheit einer Eingabe, und `fertig` laeuft dann
//! gar nicht.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField` (ueber `NSControl`, `NSView` und `NSResponder`), `NSWindow`
//! und `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso `alloc`,
//! `initWithFrame:`, `setStringValue:` und `stringValue`. `NSPoint`, `NSRect`
//! und `NSSize` sind blosse Strukturen und tragen keine Verfuegbarkeitsangabe;
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
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use krk_core::operation::{Namensfehler, name_pruefen};

use super::Blatt;

/// Die Breite des Eingabefeldes in Punkten.
///
/// Schmaler als die der Pfadeingabe: ein Name ist kein Pfad, und ein Feld, das
/// dreimal so breit ist wie sein laengster erwarteter Inhalt, sieht aus wie ein
/// Fehler.
const FELDBREITE: f64 = 280.0;

/// Die Hoehe einer Zeile im Eingabefeld in Punkten.
const FELDHOEHE: f64 = 24.0;

/// Zeigt die Namenseingabe am Fenster und liefert den geprueften Namen.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden, wenn der Nutzer
/// bestaetigt hat; bricht er ab, laeuft es gar nicht. Der Name kommt getrimmt
/// an: fuehrende und schliessende Leerzeichen sind so gut wie immer ein
/// Versehen, und ein Ordner, den man von seinem Nachbarn nicht unterscheiden
/// kann, ist keine Hilfe.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    bestaetigen: &str,
    fertig: impl Fn(Result<String, Namensfehler>) + 'static,
) {
    frei_zeigen(mtm, fenster, frage, bestaetigen, "", move |name| {
        fertig(name_pruefen(&name).map(|()| name))
    });
}

/// Zeigt dieselbe Namenseingabe, ohne den Namen gegen das Dateisystem zu
/// pruefen.
///
/// Der Weg der Lesezeichen aus C5. Ein Lesezeichenname ist eine Beschriftung
/// und kein Eintrag im Dateisystem: "Projekte/2026" ist ein zulaessiger Name
/// dafuer, und [`name_pruefen`] wiese ihn ab. Welche Regel gilt, entscheidet
/// deshalb der Aufrufer; fuer das Lesezeichen ist es
/// `krk_core::ablage::lesezeichen::name_pruefen`.
///
/// **Ein Blatt und kein zweites.** [`zeigen`] laeuft ueber dieselbe Funktion
/// und legt seine Pruefung darum; zwei Eingabeblaetter fuer einen Namen waeren
/// zwei Erscheinungsbilder und zwei Tastaturbedienungen fuer dieselbe Frage.
///
/// `vorgabe` steht beim Aufgehen im Feld und ist ausgewaehlt: beim Umbenennen
/// ist es der alte Name, beim Anlegen der Name des Ordners. Wer sie behalten
/// will, bestaetigt; wer nicht, tippt darueber.
pub fn frei_zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    bestaetigen: &str,
    vorgabe: &str,
    fertig: impl Fn(String) + 'static,
) {
    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );
    if !vorgabe.is_empty() {
        feld.setStringValue(&NSString::from_str(vorgabe));
    }

    let mut blatt = Blatt::neu(mtm, frage, bestaetigen);
    blatt.textfeld_setzen(mtm, &feld);

    let feld: Retained<NSTextField> = feld;
    blatt.zeigen(fenster, move |bestaetigt| {
        if !bestaetigt {
            return;
        }
        fertig(feld.stringValue().to_string().trim().to_owned());
    });
}
