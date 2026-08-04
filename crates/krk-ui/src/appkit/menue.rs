//! Das Hauptmenue, von Hand gebaut.
//!
//! Der Technologieentscheid bringt keinen Oberflaechenbau mit: es gibt kein
//! `MainMenu.nib`, aus dem AppKit das Menue laedt. Jeder Eintrag entsteht
//! deshalb hier im Programmtext.
//!
//! Drei Befehle: Beenden, Fenster einblenden, Fenster schliessen. Alle drei
//! bekommen als Ziel `nil` und laufen damit ueber die Antwortkette:
//! `terminate:` erreicht `NSApplication`, `performClose:` das Fenster mit dem
//! Tastaturfokus, und `fensterEinblenden:` den Anwendungsdelegierten, an dem
//! die Kette endet. Ein fest gesetztes Ziel wuerde die Kette umgehen und einen
//! Eintrag auch dann aktiv lassen, wenn niemand ihn beantworten kann.
//!
//! # Die zwei Kuerzel des Fenstermenues, und warum sie so liegen
//!
//! **Cmd+W gehoert dem Tab, nicht dem Fenster.** So fuehrt es
//! `resources/default-keymap.toml` seit Schritt 9 unter `tab_schliessen`, und
//! der Nutzer hat es am 260804 bestaetigt. Der Menueeintrag "Fenster
//! schliessen" ist deshalb mit Schritt 12 von Cmd+W auf **Shift+Cmd+W**
//! gewichen, wie Webbrowser es halten. Defekt
//! `issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`.
//!
//! **Cmd+N holt das geschlossene Fenster zurueck.** Bis Schritt 12 lief KRK
//! nach dem Schliessen des Fensters weiter, mit Menueleiste und ohne jeden Weg
//! zu einem Fenster; fuer eine Anwendung, deren erste Maxime die
//! Tastatursteuerung ist, lag damit ein Kuerzel in Reichweite, das sie
//! unbedienbar machte. Der Nutzer hat am 260804-0830 Moeglichkeit 2 aus
//! `decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
//! gewaehlt. Der Eintrag heisst **"Fenster einblenden"** und nicht "Neues
//! Fenster", weil er keines anlegt: KRK haelt in dieser Runde genau ein
//! Anwendungsfenster, es ueberlebt sein Schliessen, und der Eintrag holt es
//! nach vorn. Die Runde, die mehrere Fenster einfuehrt, benennt ihn um und
//! behaelt das Kuerzel.
//!
//! Dasselbe Kuerzel steht als `fenster_einblenden` in
//! `resources/default-keymap.toml`, seit Schritt 9b. Beides zugleich ist kein
//! Widerspruch und auch keine zweite Wahrheit: der Ereignisabgriff sieht jeden
//! Tastendruck vor der Menuebehandlung von `NSApplication`, fuehrt den Befehl
//! aus und schluckt das Ereignis. Der Menueeintrag traegt das Kuerzel also
//! sichtbar, ausgeloest wird er im Alltag ueber die Belegung, und der Nutzer
//! kann sie umbelegen, ohne dass der Menueweg verloren geht.

use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::{MainThreadOnly, sel};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
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
            NSEventModifierFlags::Command,
        )],
    ));
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("Fenster"),
        &[
            befehl(
                mtm,
                ns_string!("Fenster einblenden"),
                sel!(fensterEinblenden:),
                ns_string!("n"),
                NSEventModifierFlags::Command,
            ),
            befehl(
                mtm,
                ns_string!("Fenster schließen"),
                sel!(performClose:),
                ns_string!("w"),
                NSEventModifierFlags::Command | NSEventModifierFlags::Shift,
            ),
        ],
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

/// Ein Menuebefehl mit Titel, Aktion, Taste und Zusatztasten.
///
/// Die Zusatztasten stehen ausdruecklich da und nicht als Grossbuchstabe im
/// Kuerzel. `NSMenuItem` leitete aus einem `W` zwar dieselbe Anzeige ab, aber
/// der Diff dieser Datei soll zeigen, welche Zusatztaste gemeint ist; das
/// Abnahmekriterium von Schritt 12 liest ihn.
fn befehl(
    mtm: MainThreadMarker,
    titel: &NSString,
    aktion: Sel,
    kuerzel: &NSString,
    zusatztasten: NSEventModifierFlags,
) -> Retained<NSMenuItem> {
    // SAFETY: Titel und Kuerzel sind gueltige Zeichenketten, die Auswahl ist
    // ein statisches Selektorliteral. Ein Ziel setzt der Aufruf nicht, damit
    // die Antwortkette entscheidet.
    let eintrag = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            titel,
            Some(aktion),
            kuerzel,
        )
    };
    eintrag.setKeyEquivalentModifierMask(zusatztasten);
    eintrag
}
