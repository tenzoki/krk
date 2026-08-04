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

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextField, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize};

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
    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );

    let mut blatt = Blatt::neu(mtm, frage, bestaetigen);
    blatt.textfeld_setzen(mtm, &feld);

    let feld: Retained<NSTextField> = feld;
    blatt.zeigen(fenster, move |bestaetigt| {
        if !bestaetigt {
            return;
        }
        let name = feld.stringValue().to_string().trim().to_owned();
        fertig(name_pruefen(&name).map(|()| name));
    });
}
