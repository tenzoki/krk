//! Das Hauptmenue, von Hand gebaut.
//!
//! Der Technologieentscheid bringt keinen Oberflaechenbau mit: es gibt kein
//! `MainMenu.nib`, aus dem AppKit das Menue laedt. Jeder Eintrag entsteht
//! deshalb hier im Programmtext.
//!
//! Runde 1, Schritt 6 braucht genau zwei Befehle: Beenden und Fenster
//! schliessen. Beide bekommen als Ziel `nil` und laufen damit ueber die
//! Antwortkette: `terminate:` erreicht `NSApplication`, `performClose:`
//! erreicht das Fenster mit dem Tastaturfokus. Ein fest gesetztes Ziel wuerde
//! die Kette umgehen und den Eintrag auch dann aktiv lassen, wenn gar kein
//! Fenster offen ist.

use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::{MainThreadOnly, sel};
use objc2_app_kit::{NSMenu, NSMenuItem};
use objc2_foundation::{MainThreadMarker, NSString, ns_string};

/// Baut das Hauptmenue der Anwendung.
pub fn hauptmenue(mtm: MainThreadMarker) -> Retained<NSMenu> {
    let hauptmenue = NSMenu::new(mtm);
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("KRK"),
        &[befehl(
            mtm,
            ns_string!("KRK beenden"),
            sel!(terminate:),
            ns_string!("q"),
        )],
    ));
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("Fenster"),
        &[befehl(
            mtm,
            ns_string!("Fenster schließen"),
            sel!(performClose:),
            ns_string!("w"),
        )],
    ));
    hauptmenue
}

/// Haengt ein benanntes Untermenue mit den genannten Befehlen unter einen
/// Eintrag der Menueleiste.
///
/// Die Menueleiste traegt keine Befehle selbst, sondern nur Eintraege mit
/// Untermenues; der Titel des ersten ersetzt macOS ohnehin durch den Namen aus
/// der `Info.plist`.
fn untermenue(
    mtm: MainThreadMarker,
    titel: &NSString,
    befehle: &[Retained<NSMenuItem>],
) -> Retained<NSMenuItem> {
    let eintrag = NSMenuItem::new(mtm);
    let menue = NSMenu::initWithTitle(NSMenu::alloc(mtm), titel);
    for befehl in befehle {
        menue.addItem(befehl);
    }
    eintrag.setSubmenu(Some(&menue));
    eintrag
}

/// Ein Menuebefehl mit Titel, Aktion und Tastenkuerzel.
///
/// Das Kuerzel traegt implizit die Befehlstaste; `NSMenuItem` setzt sie als
/// Vorbelegung der Zusatztastenmaske.
fn befehl(
    mtm: MainThreadMarker,
    titel: &NSString,
    aktion: Sel,
    kuerzel: &NSString,
) -> Retained<NSMenuItem> {
    // SAFETY: Titel und Kuerzel sind gueltige Zeichenketten, die Auswahl ist
    // ein statisches Selektorliteral. Ein Ziel setzt der Aufruf nicht, damit
    // die Antwortkette entscheidet.
    unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            titel,
            Some(aktion),
            kuerzel,
        )
    }
}
