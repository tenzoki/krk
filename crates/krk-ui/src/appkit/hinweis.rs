//! Das modale Hinweisfenster: ein Titel, ein Satz, eine Schaltflaeche.
//!
//! ```text
//! zeigen(Titel, Satz) ──> NSAlert ──> runModal() ──> kehrt zurueck
//!                                         │
//!                          die ganze Anwendung steht solange still
//! ```
//!
//! Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: die
//! laufenden Fehler traegt die Statuszeile aus [`super::statuszeile`], und
//! **genau ein** Fehler bricht ab, der fehlende Tastenabgriff. Diese Huelle ist
//! sein Weg nach draussen.
//!
//! # Warum das kein Blatt ist
//!
//! [`super::blaetter`] haelt die Dialoge, die am oberen Rand des Fensters
//! herunterfahren, und ein Blatt ist in vier Punkten das Gegenteil dessen, was
//! hier gebraucht wird: es haengt an einem Fenster, es sperrt allein dieses und
//! nicht die Anwendung, es kehrt sofort zurueck und liefert seine Antwort
//! spaeter ueber einen Rueckruf, und mit dieser Antwort geht die Arbeit weiter.
//! Der Hinweis hier braucht kein Fenster — sein Aufrufer zeigt ihn, bevor das
//! Hauptfenster ueberhaupt vorn steht —, er sperrt die Anwendung, er kehrt erst
//! nach der Bestaetigung zurueck, und danach geht nichts weiter: er ist die
//! letzte Ausgabe vor dem Beenden. Ihn in [`super::blaetter::Blatt`]
//! unterzubringen hiesse, jenem Typ eine zweite Betriebsart zu geben, die mit
//! seinem Namen nichts mehr zu tun hat.
//!
//! **Braucht ein spaeterer Schritt ein zweites modales Hinweisfenster, nimmt er
//! diese Huelle und stellt keine daneben.** Schritt 6b des Plans schreibt das
//! ausdruecklich fest; die Aufteilung ist damit: modal und endgueltig hier,
//! am Fenster und mit Antwort in [`super::blaetter`].
//!
//! # Die Tastatur erreicht dieses Fenster, auch wenn KRK sie nicht liest
//!
//! Der Aufrufer dieser Runde meldet, dass der Ereignisabgriff aus
//! [`super::ereignisse`] nicht steht. Fuer den Hinweis aendert das nichts: der
//! Abgriff ist ein zusaetzlicher Mitleser ueber
//! `addLocalMonitorForEventsMatchingMask:handler:` und nicht der Zustellweg.
//! AppKit stellt den Tastendruck der hervorgehobenen Schaltflaeche weiterhin
//! selbst zu, und die Eingabetaste bestaetigt. Ein Hinweis, den nur die Maus
//! wegklicken kann, waere sonst die zweite Haelfte desselben Defekts.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSAlert`, `NSApplication` und `NSString` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `sharedApplication`, `setMessageText:`,
//! `setInformativeText:`, `setAlertStyle:`, `addButtonWithTitle:`, `runModal`
//! und die gesetzte Konstante `NSAlertStyleCritical`. Eine Methode ist juenger
//! als ihre Klasse: **`NSApplication.activate` steht seit macOS 14**
//! (`NSApplication.h`, `API_AVAILABLE(macos(14.0))`); es ist die Beruehrung, die
//! den Hinweis nach vorn holt, und die einzige dieser Datei ueber 10.0. Das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS
//! 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use objc2_app_kit::{NSAlert, NSAlertStyle, NSApplication};
use objc2_foundation::{MainThreadMarker, NSString, ns_string};

/// Zeigt den Hinweis modal fuer die ganze Anwendung und kehrt zurueck, sobald
/// der Nutzer bestaetigt hat.
///
/// `titel` steht fett in der ersten Zeile, `satz` darunter. Mehr nimmt die
/// Huelle nicht entgegen, und sie gibt nichts zurueck: sie stellt keine Frage,
/// also gibt es auch keine Antwort, an der sich ein Aufrufer verzweigen
/// koennte.
pub fn zeigen(mtm: MainThreadMarker, titel: &str, satz: &str) {
    let warnung = NSAlert::new(mtm);
    warnung.setMessageText(&NSString::from_str(titel));
    warnung.setInformativeText(&NSString::from_str(satz));
    // `Critical` und nicht die Vorgabe `Warning`: dieses Fenster erscheint nur,
    // wenn KRK danach aufhoert zu laufen. Dieselbe Wahl wie bei der lauten Form
    // der Loeschrueckfrage, und aus demselben Grund — das Warnzeichen des
    // Systems steht fuer den Vorgang, dessen Ruecknahme teuer oder unmoeglich
    // ist.
    warnung.setAlertStyle(NSAlertStyle::Critical);
    // Die eine Schaltflaeche ausdruecklich und nicht die Vorgabe von `NSAlert`:
    // deren Beschriftung kaeme aus der Lokalisierung von AppKit, und welche
    // Sprache die trifft, entscheidet dann das System und nicht KRK. `NSAlert`
    // gibt der ersten Schaltflaeche die Eingabetaste von sich aus; hier ist das
    // die gewuenschte Vorgabe, und anders als in [`super::blaetter`] muss ihr
    // nichts nachgeholfen werden.
    let _knopf = warnung.addButtonWithTitle(ns_string!("OK"));
    // Ohne diesen Aufruf stuende der Hinweis hinter der Anwendung, aus der
    // heraus KRK gestartet wurde: sein Aufrufer zeigt ihn, bevor
    // `makeKeyAndOrderFront` das Hauptfenster nach vorn holt, und bis dahin hat
    // KRK nichts auf dem Schirm, das die Aktivierung sonst besorgte.
    NSApplication::sharedApplication(mtm).activate();
    // Der Rueckgabewert nennt die gedrueckte Schaltflaeche. Es gibt eine.
    let _antwort = warnung.runModal();
}
