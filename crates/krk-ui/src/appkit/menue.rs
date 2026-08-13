//! Das Hauptmenue: die Umsetzung von [`crate::menuemodell`] in AppKit.
//!
//! Der Technologieentscheid bringt keinen Oberflaechenbau mit: es gibt kein
//! `MainMenu.nib`, aus dem AppKit das Menue laedt. Jeder Eintrag entsteht
//! deshalb im Programm.
//!
//! **Was in der Leiste steht, entscheidet diese Datei seit der Runde 7 nicht
//! mehr.** [`crate::menuemodell::aufbau`] rechnet die Leiste aus der Belegung
//! aus — neun Obermenues, zweiundachtzig Eintraege —, und [`hauptmenue`] setzt
//! den Wert in `NSMenu` und `NSMenuItem` um. Bis dahin standen hier drei
//! Untermenues und zehn Eintraege als Programmtext, und ihre Reihenfolge war
//! allein am laufenden Buendel nachzusehen. Der Gewinn ist nicht die Zahl,
//! sondern dass die Kriterien C2.1 bis C2.4 und C2.9 jetzt ohne Fenster und
//! ohne Hauptfaden pruefbar sind.
//!
//! Jeder Eintrag bekommt als Ziel `nil` und laeuft damit ueber die
//! Antwortkette. `cut:`, `copy:` und `paste:` erreichen den Feldeditor des
//! Textfeldes mit dem Fokus beziehungsweise die Textflaeche des Editors, und
//! [`KRK_KOMMANDO`] wie `tastenbelegungSichern:` erreichen den
//! Anwendungsdelegierten, an dem die Kette endet. Ein fest gesetztes Ziel wuerde
//! die Kette umgehen und einen Eintrag auch dann aktiv lassen, wenn niemand ihn
//! beantworten kann.
//!
//! # Jeder Eintrag mit Kommando traegt einen Selektor, und es ist derselbe
//!
//! Fuenfundsiebzig Selektoren waeren fuenfundsiebzig Methoden am
//! Anwendungsdelegierten, jede mit demselben Rumpf. Ein Eintrag mit Kommando
//! traegt deshalb [`KRK_KOMMANDO`] und im `tag` den Index seines Kommandos in
//! [`Kommando::KENNUNGEN`]; die Uebersetzung in beide Richtungen steht in
//! [`tag_des_kommandos`] und [`kommando_zum_tag`]. Sie steht hier und nicht im
//! Modell, weil `tag` ein AppKit-Begriff ist.
//!
//! **Damit laeuft ein Menueeintrag ueber `kommando_ausfuehren`, also ueber
//! denselben einen Ausfuehrungsweg wie ein Tastendruck** (C2.14), und die
//! Ausgrauung ueber `validateMenuItem:` fragt dieselbe Regel wie der
//! Ereignisabgriff (C2.16). Beide stehen am Anwendungsdelegierten; diese Datei
//! kennt sie nicht.
//!
//! **Fuer `selectAll:`, `undo:` und `redo:` stand hier bis zum 260811 derselbe
//! Satz, und fuer alle drei war er zu stark.** Was wirklich antwortet, ist
//! gemessen und steht unten unter `# Wer die sechs Textbefehle beantwortet`.
//!
//! # Eine Quelle, zwei sichtbare Wege
//!
//! **Ein Eintrag traegt bewusst gar keine Kennung**, und er ist der einzige:
//! "Tastenbelegung als Markdown sichern" im Anwendungsmenue (Runde 3). Er steht
//! deshalb im Modell als `Eintrag::Sonderposten` und nicht als Befehl. Ein
//! Kuerzel waere nach dem Nutzerentscheid vom 260805-0000 zwingend ein
//! Belegungseintrag mit `gehalten_von = "menue"` geworden und haette damit die
//! Bauform geaendert, nicht nur die Bequemlichkeit; der Nutzer hat den Eintrag
//! am 260811-0110 ausdruecklich ohne Kuerzel bestellt. Er steht **vor** dem
//! Beenden, durch einen Trenner davon geschieden, weil das Beenden auf dem Mac
//! unten steht. Beantwortet wird er am Anwendungsdelegierten
//! (`tastenbelegungSichern:`), der die Ausgabe an [`crate::belegungsausgabe`]
//! weiterreicht.
//!
//! **Kein Kuerzel steht hier als Zeichenkette, ohne Ausnahme.** Jedes kommt aus
//! der Belegung, ueber das Modell. Damit ist `resources/default-keymap.toml` auch fuer
//! das Menue die alleinige Quelle: die Konflikterkennung aus C3 sieht jede
//! Kombination, der Nutzer kann jede umbelegen, und eine Umbelegung wirkt auf
//! beide Wege. Nutzerentscheid vom 260805-0000,
//! `decisions/260805-0000_*_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`.
//!
//! Bis zum 260805-0820 stand hier eine Ausnahme: der Eintrag "KRK beenden"
//! trug Cmd+Q als Zeichenkette im Programmtext, weil die Belegungsdatei die
//! Funktion `beenden` noch nicht fuehrte. Sie fuehrt sie seither, und die
//! Ausnahme ist mit ihr weggefallen
//! (`issues/260805-0753_*_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md`).
//!
//! Welchen der beiden Wege ein Tastendruck geht, entscheidet der Fokus. Der
//! Ereignisabgriff aus [`super::ereignisse`] sieht ihn vor der Menuebehandlung
//! von `NSApplication`. Steht die Schreibmarke in einem Textfeld, kehrt er
//! sofort zurueck und reicht weiter; dann wirkt das Menue. Steht sie im
//! Dateifenster, schlaegt er in der Belegung nach — und die vom Menue
//! gehaltenen Funktionen sieht er dabei nicht, weil `Belegung::nachschlag` sie
//! ueberspringt. Die sechs Textbefehle laufen deshalb auch im Dateifenster ins
//! Menue und von dort die Antwortkette hinunter, wo heute niemand `paste:`
//! beantwortet und der Eintrag folglich grau ist. Genau das ist der
//! Einhaengepunkt der spaeteren Dateizwischenablage: sie beantwortet `copy:`
//! und `paste:` am Dateifenster und braucht dafuer weder einen zweiten
//! Menueeintrag noch eine zweite Zeile in der Belegung.
//!
//! # Warum es das Menue "Bearbeiten" ueberhaupt gibt
//!
//! Auf dem Mac liegen Cmd+X, Cmd+C, Cmd+V und Cmd+A fuer Textfelder nicht im
//! Textsystem, sondern als Menuekuerzel. Ohne dieses Menue erreicht kein
//! Tastendruck `cut:`, `copy:`, `paste:` und `selectAll:`, und C2 sagt fuer die
//! Pfadeingabe ausdruecklich zu, dass der Nutzer einen Pfad **einfuegt**.
//! Gemessen am 260804-1309 am laufenden Buendel: Pfad in der Zwischenablage,
//! `shift+cmd+g`, `cmd+v` gesendet, Feld unveraendert. Defekt
//! `issues/260804-1309_*_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`.
//!
//! **`undo:` und `redo:` liegen genauso.** Die `NSTextView` des Editors bringt
//! ihren Rueckgaengigverwalter zwar mit, benutzt ihn aber erst, wenn
//! `allowsUndo` gesetzt ist; das geschieht in `super::editor`, und diese beiden
//! Eintraege sind die zweite Haelfte derselben Sache. Cmd+Z und Shift+Cmd+Z
//! erreichen den Verwalter naemlich nur ueber ein Menuekuerzel. Ohne die
//! beiden Eintraege "Rueckgaengig" und "Wiederholen" haette der Editor kein
//! Rueckgaengig, und ohne `allowsUndo` blieben die Eintraege grau. Sie stehen
//! an der Mac-ueblichen Stelle ganz oben im Untermenue, durch einen Trenner
//! von den vier Zwischenablage-Befehlen getrennt.
//!
//! # Wer die sechs Textbefehle beantwortet
//!
//! Die sechs sind die einzigen der 79 Funktionen ohne
//! [`krk_core::tasten::Kommando`] und damit ohne Wirkungsbereich. Wo sie wirken,
//! entscheidet zur Laufzeit die Antwortkette, in die die Belegung keine Eingabe
//! hat. Die Tastenbelegung als Markdown-Datei aus der Runde 3 muss es dem Nutzer
//! trotzdem in ihre dritte Spalte schreiben, und **statt zu naehern, ist
//! gemessen worden**: `die_sechs_zugestellten_textbefehle_werden_von_diesen_klassen_beantwortet`
//! unter `mod tests` fragt das Objective-C-Laufzeitsystem ueber
//! `AnyClass::responds_to`, welche der sechs moeglichen Ersthelferklassen
//! welchen Selektor beantwortet. Die Antwort braucht keine Instanz, keinen
//! Hauptfaden und keinen Vordergrund, und sie laeuft von jetzt an mit.
//!
//! ```text
//! Selektor      antwortet an        traegt die Methode
//! cut:          NSTextView          NSText
//! copy:         NSTextView          NSText
//! paste:        NSTextView          NSText
//! selectAll:    NSTableView         NSTableView
//!               NSTextView          NSText
//! undo:         NSWindow            NSWindow
//! redo:         NSWindow            NSWindow
//! ```
//!
//! Gemessen am 260811 auf macOS 15.7.7, gegen `NSTableView`, `NSTextView`,
//! `NSTextField`, `NSScrollView`, `NSWindow` und `NSApplication`. Drei Befunde
//! stehen darin, und zwei davon widersprechen dem, was oben stand.
//!
//! **Erstens: die vier Zwischenablage-Befehle haengen an `NSText`.** Nicht an
//! `NSTextView`, und `NSTextField` beantwortet keinen von ihnen. Das ist der
//! Beleg fuer den Satz oben, das Menue erreiche den **Feldeditor** des
//! Textfeldes: der Feldeditor ist eine `NSTextView` und bringt `NSText` mit, das
//! Textfeld selbst nicht.
//!
//! **Zweitens: `NSTableView` beantwortet `selectAll:` von sich aus.** Die
//! Lesezeichen- und Geraeteleiste ist eine `NSTableView`, und mit dem Fokus dort
//! weist der stumme Fokusvorbehalt `alle_markieren` ab, sodass der Tastendruck
//! unveraendert an AppKit geht und den Menueeintrag erreicht. Der Eintrag ist in
//! der Leiste also **bedienbar**, und "Textfelder und Editor" ist fuer diesen
//! einen der sechs keine gemessene Aussage mehr. Ob er dort auch etwas
//! **bewirkt**, ist eine zweite Frage; sie braucht eine Instanz und ist hier
//! nicht entschieden. Die Auswahleinstellung der drei Tabellen steht in
//! [`super::leiste`], [`super::belegungsansicht`] und [`super::tabelle`].
//!
//! **Drittens: `undo:` und `redo:` stehen an `NSWindow` und nicht an
//! `NSTextView`.** `responds_to` liefert `false` fuer einen Selektor, den eine
//! Klasse ueber Weiterleitung beantwortet, und genau das ist hier der Fall: das
//! Fenster traegt beide Methoden und reicht sie an den Rueckgaengigverwalter des
//! Ersthelfers weiter. Ein `false` an der Textklasse belegt deshalb **nicht**,
//! dass im Editor niemand antwortet. Was dort antwortet, steht zwei Abschnitte
//! weiter oben: die `NSTextView` bringt ihren Verwalter mit und benutzt ihn,
//! sobald [`super::editor`] `allowsUndo` setzt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSMenu`, `NSMenuItem`, `NSString`, `NSNumber`, `NSDictionary` und
//! `NSUserDefaults` stehen seit macOS 10.0 zur Verfuegung, ebenso die sechs
//! Klassen, die die Messung oben abfragt: `NSTableView`, `NSTextView`,
//! `NSTextField`, `NSScrollView`, `NSWindow` und `NSApplication`. Das Buendel
//! zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS 15
//! hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! # Drei Eintraege trugen einen eigenen Selektor, und warum keiner mehr
//!
//! "Fenster schliessen", "Fenster einblenden" und "KRK beenden" liefen bis zur
//! Runde 7 ueber `fensterSchliessen:`, `fensterEinblenden:` und `beenden:` am
//! Anwendungsdelegierten. Sie liefen damit **an** `kommando_ausfuehren`
//! **vorbei**, und mit einem Kuerzel an jedem der zweiundachtzig Eintraege waere
//! daraus eine Regel geworden statt einer Ausnahme; C2.14 schliesst genau diese
//! Luecke. Die drei tragen seither [`KRK_KOMMANDO`] wie jeder andere Befehl.
//!
//! **Was die drei eigenen Selektoren abwehren sollten, wehren sie weiterhin
//! ab.** Der Grund fuer sie war nie die Ausfuehrung, sondern die Zweitform, die
//! AppKit zu einem bestimmten **Systemselektor** von sich aus dazustellt:
//!
//! - Zu `performClose:` stellt es "Close All" auf Opt+Shift+Cmd+W dazu, mit
//!   englischer Beschriftung und einer Kombination, die niemand aus der Belegung
//!   setzen oder umbelegen kann (gemessen am 260804-1040 im signierten Buendel,
//!   `issues/260804-1040_*_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md`).
//! - Zu `terminate:` stellt es "Quit and Keep Windows" auf Opt+Cmd+Q dazu. Sie
//!   erscheint spaeter als die von "Close All": nicht schon nach
//!   `finishLaunching`, sondern erst an der wirklich laufenden Anwendung,
//!   weshalb `--menue-protokoll` sie nicht sah und der Befund vom 260805-0753
//!   ueber die Bedienungshilfen kam
//!   (`issues/260805-0753_*_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md`).
//!
//! [`KRK_KOMMANDO`] ist so wenig `performClose:` und `terminate:` wie
//! `fensterSchliessen:` und `beenden:` es waren; der Delegierte ruft beide
//! Systemselektoren weiterhin selbst, an `NSWindow` beziehungsweise
//! `NSApplication`. Dass keine Zweitform zurueckkehrt, ist am laufenden Buendel
//! nachzusehen und steht in der Abnahmeliste der Runde 7.
//!
//! # Die zwei Kuerzel des Fenstermenues, und warum sie so liegen
//!
//! **Cmd+W gehoert dem Tab, nicht dem Fenster.** So fuehrt es
//! `resources/default-keymap.toml` seit Schritt 9 unter `tab_schliessen`, und
//! der Nutzer hat es am 260804 bestaetigt. Der Menueeintrag "Fenster
//! schliessen" ist deshalb mit Schritt 12 von Cmd+W auf Shift+Cmd+W gewichen,
//! wie Webbrowser es halten.
//!
//! **Cmd+N holt das geschlossene Fenster zurueck.** Bis Schritt 12 lief KRK
//! nach dem Schliessen des Fensters weiter, mit Menueleiste und ohne jeden Weg
//! zu einem Fenster; fuer eine Anwendung, deren erste Maxime die
//! Tastatursteuerung ist, lag damit ein Kuerzel in Reichweite, das sie
//! unbedienbar machte. Der Nutzer hat am 260804-0830 Moeglichkeit 2 aus
//! `decisions/260803-2007_*_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
//! gewaehlt. Der Eintrag heisst "Fenster einblenden" und nicht "Neues Fenster",
//! weil er keines anlegt: KRK haelt in dieser Runde genau ein
//! Anwendungsfenster, es ueberlebt sein Schliessen, und der Eintrag holt es
//! nach vorn. Die Runde, die mehrere Fenster einfuehrt, benennt ihn um und
//! behaelt das Kuerzel.

use core::ffi::CStr;

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
use objc2_foundation::{
    MainThreadMarker, NSDictionary, NSInteger, NSNumber, NSString, NSUserDefaults, ns_string,
};

use krk_core::tasten::parser::{self, Taste};
use krk_core::tasten::{Belegung, Kombination, Kommando, ModMaske};

use crate::menuemodell::{self, Eintrag};

/// Haelt macOS davon ab, dem Menue "Bearbeiten" eigene Eintraege dazuzustellen.
///
/// **Zu rufen vor `NSApplication`, sonst ist es zu spaet.**
///
/// macOS haengt an ein Menue "Bearbeiten" von sich aus "Emoji & Symbols" und
/// "Start Dictation…" an, und der erste traegt ein Kuerzel. Eine Kombination,
/// die macOS waehlt, steht in keiner Tastenliste, wird von der
/// Konflikterkennung nicht gesehen und ist nicht umbelegbar — genau der blinde
/// Fleck, den der Nutzerentscheid vom 260805-0000 schliesst. Sie muss deshalb
/// nicht bloss unbeachtet bleiben, sondern verschwinden.
///
/// **Die beiden Schluessel in `resources/Info.plist` reichten dafuer nicht.**
/// Am 260805 trug das signierte Buendel `NSDisabledCharacterPaletteMenuItem`
/// und `NSDisabledDictationMenuItem` in seiner `Info.plist` beide auf `true`,
/// und `--menue-protokoll` zeigte trotzdem "Start Dictation…" und drei Formen
/// von "Emoji & Symbols", darunter Cmd+Leertaste sichtbar und
/// Ctrl+Cmd+Leertaste verdeckt. Dieselben beiden Namen als **Nutzervorgabe**
/// wirkten: mit
/// `-NSDisabledCharacterPaletteMenuItem YES -NSDisabledDictationMenuItem YES`
/// auf der Befehlszeile war das Menue leer bis auf die vier eigenen Eintraege.
/// AppKit liest die beiden also aus `NSUserDefaults` und nicht aus der
/// Bundle-Beschreibung; diese Funktion stellt sie dort in die
/// Registrierungsebene, die jede Nutzereinstellung ueberschreiben kann.
///
/// **Die Messung steht, ihre Nachstellung nicht mehr.** Die beiden Schluessel
/// sind am 260805-0820 aus `resources/Info.plist` entfernt, weil sie neben der
/// Stelle standen, die die Sache traegt; ein `plutil -extract` gegen ein neu
/// gebautes `KRK.app/Contents/Info.plist` findet sie seither nicht. Wer den
/// Befund nachstellen will, braucht ein Buendel mit den Schluesseln. Beide
/// Messungen, die des `coder` mit YES und die Gegenprobe des `ontocoder` mit
/// NO, stehen vollstaendig in
/// `issues/260805-0753_*_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`.
///
/// **Der dritte Zusatz ist ein Untermenue "AutoFill", und er hat einen eigenen
/// Schluessel mit umgekehrtem Sinn.** Er kam am 260805-1455 mit dem ersten
/// `make menue` zum Vorschein: ein Trenner und darunter ein Eintrag mit dem
/// Selektor `submenuAction:`, den `hauptmenue` nicht anlegt. Er traegt keine
/// Kombination und bricht deshalb keine Zusage aus C3; er zeigt dem Nutzer
/// aber ein Untermenue, das ein Dateimanager ohne Formularfelder nicht
/// bedienen kann, und "supersimpel" spricht dagegen. Der im Defekt vermutete
/// Name `NSDisabledAutoFillMenuItem` wirkt nicht — am 260806-1203 am gebauten
/// Buendel gemessen, das Menue trug den Eintrag unveraendert. Der Name, der
/// wirkt, ist `NSAutoFillSystemInsertMenuEnabled`, und er wird **verneint**
/// statt bejaht: mit `-NSAutoFillSystemInsertMenuEnabled NO` auf der
/// Befehlszeile gab `--menue-protokoll` weder den Trenner noch den Eintrag
/// aus. Deshalb stehen hier zwei Wahrheitswerte und nicht ein gemeinsamer.
/// Gefunden ist er nicht geraten, sondern in den Zeichenketten des
/// dyld-Zwischenspeichers gesucht: von den Namen der Form
/// `NS…AutoFill…`/`NS…Disabled…` ist er der einzige, der ein vom System
/// eingefuegtes Menue benennt.
pub fn systemzusaetze_unterdruecken() {
    let ja = NSNumber::new_bool(true);
    let nein = NSNumber::new_bool(false);
    let vorgaben = NSDictionary::from_slices(
        &[
            &*NSString::from_str("NSDisabledCharacterPaletteMenuItem"),
            &*NSString::from_str("NSDisabledDictationMenuItem"),
            &*NSString::from_str("NSAutoFillSystemInsertMenuEnabled"),
        ],
        &[
            ja.as_ref() as &AnyObject,
            ja.as_ref() as &AnyObject,
            nein.as_ref() as &AnyObject,
        ],
    );
    // SAFETY: `registerDefaults:` nimmt ein `NSDictionary` mit Zeichenketten
    // als Schluesseln entgegen und kopiert es; genau das steht hier. Es
    // schreibt nichts auf die Platte, sondern fuellt die unterste Ebene der
    // Nutzervorgaben, die jede Einstellung des Nutzers ueberschreibt.
    unsafe { NSUserDefaults::standardUserDefaults().registerDefaults(&vorgaben) };
}

/// Der Sammelselektor jedes Menueeintrags, der ein [`Kommando`] traegt.
///
/// **Eine Methode statt fuenfundsiebzig.** Ein eigener Selektor je Befehl
/// waere eine Methode je Befehl am Anwendungsdelegierten, jede mit demselben
/// zwei Zeilen langen Rumpf. Welchen Befehl ein Eintrag meint, steht deshalb
/// nicht im Selektor, sondern in seinem `tag`; [`tag_des_kommandos`] setzt ihn,
/// [`kommando_zum_tag`] liest ihn zurueck.
///
/// **Nicht `representedObject`**, obwohl es der naheliegende Traeger waere: das
/// Feld nimmt ein Objective-C-Objekt entgegen und verlangte damit eine
/// Wrapperklasse um ein Rust-Enum, die dieser Baum sonst nirgends braucht. Der
/// `tag` ist eine Zahl, und [`Kommando::KENNUNGEN`] fuehrt jedes Kommando genau
/// einmal — sein Index ist im Prozess stabil und taugt ohne Umweg.
pub const KRK_KOMMANDO: &CStr = c"krkKommando:";

/// Baut das Hauptmenue der Anwendung aus der Belegung.
///
/// **Ein Umsetzer und kein Baumeister.** Was in der Leiste steht, rechnet
/// [`crate::menuemodell::aufbau`] ohne AppKit aus; hier entstehen daraus
/// `NSMenu` und `NSMenuItem` und sonst nichts. Bis zur Runde 7 stand die
/// Gliederung als Programmtext in dieser Funktion — drei Untermenues, zehn
/// Eintraege, jede Beschriftung und jede Kennung von Hand —, und ihre
/// Reihenfolge war allein am laufenden Buendel nachzusehen.
///
/// Gerufen wird sie an genau zwei Stellen: beim Start (`starten`) und nach einer
/// Aenderung in der Belegungsansicht (`Anwendungsdelegierter::menue_neu_bauen`).
/// Ein Kuerzel, das der Nutzer umbelegt, steht danach im Menue (C2.11).
pub fn hauptmenue(mtm: MainThreadMarker, belegung: &Belegung) -> Retained<NSMenu> {
    let hauptmenue = NSMenu::new(mtm);
    for obermenue in menuemodell::aufbau(belegung) {
        let eintraege: Vec<Retained<NSMenuItem>> = obermenue
            .eintraege
            .iter()
            .map(|eintrag| umsetzen(mtm, eintrag))
            .collect();
        hauptmenue.addItem(&untermenue(
            mtm,
            &NSString::from_str(obermenue.titel),
            &eintraege,
        ));
    }
    hauptmenue
}

/// Ein Eintrag des Modells als `NSMenuItem`.
///
/// **Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig.**
/// Bekommt [`crate::menuemodell::Eintrag`] eine neue Sorte, haelt der
/// Uebersetzer hier an und erzwingt eine bewusste Einordnung; ein Auffangzweig
/// liesse sie stillschweigend aus dem Menue verschwinden.
fn umsetzen(mtm: MainThreadMarker, eintrag: &Eintrag<'_>) -> Retained<NSMenuItem> {
    match eintrag {
        Eintrag::Befehl {
            beschriftung,
            kombination,
            kommando,
            ..
        } => {
            // Ohne Kommando bleibt der Eintrag ohne Aktion und damit grau. Der
            // Fall ist eine benannte Funktion, die diese Runde noch nicht
            // ausfuehrt; die Auslieferungsbelegung fuehrt keine solche.
            let posten = befehl(
                mtm,
                &NSString::from_str(beschriftung),
                kommando.map(|_| Sel::register(KRK_KOMMANDO)),
                *kombination,
            );
            if let Some(kommando) = kommando {
                posten.setTag(tag_des_kommandos(*kommando));
            }
            posten
        }
        Eintrag::Textbefehl {
            beschriftung,
            kombination,
            selektor,
            ..
        } => befehl(
            mtm,
            &NSString::from_str(beschriftung),
            Some(Sel::register(selektor)),
            *kombination,
        ),
        Eintrag::Sonderposten {
            beschriftung,
            selektor,
        } => ohne_kuerzel(
            mtm,
            &NSString::from_str(beschriftung),
            Some(Sel::register(selektor)),
        ),
        Eintrag::Trenner => NSMenuItem::separatorItem(mtm),
    }
}

/// Der `tag`, unter dem ein Menueeintrag sein Kommando traegt.
///
/// Der Index in [`Kommando::KENNUNGEN`]. Die Liste fuehrt jedes Kommando genau
/// einmal — `jedes_kommando_traegt_genau_einen_wirkungsbereich` in
/// `krk-core/tests/belegung.rs` haelt das fest —, und sie ist zur Uebersetzzeit
/// festgelegt; der Index ist damit im Prozess stabil.
pub fn tag_des_kommandos(kommando: Kommando) -> NSInteger {
    let stelle = Kommando::KENNUNGEN
        .iter()
        .position(|(gefuehrt, _)| *gefuehrt == kommando)
        .expect("jedes Kommando steht in KENNUNGEN");
    NSInteger::try_from(stelle).expect("KENNUNGEN ist kuerzer als isize::MAX")
}

/// Das Kommando zu einem `tag`, falls der `tag` eines benennt.
///
/// **Der Vorgabewert eines `tag` ist Null, und Null ist ein gueltiger Index.**
/// Wer diese Funktion ruft, hat deshalb vorher die Aktion des Eintrags gegen
/// [`KRK_KOMMANDO`] geprueft; ohne diese Frage bekaeme jeder fremde Eintrag das
/// erste Kommando der Liste zugesprochen. Der Rueckgabetyp bleibt trotzdem eine
/// Moeglichkeit und keine Zusicherung: ein `tag`, der aus der Liste faellt, ist
/// ein Programmfehler und darf keinen Absturz auf dem Referenzgeraet ausloesen.
pub fn kommando_zum_tag(tag: NSInteger) -> Option<Kommando> {
    let stelle = usize::try_from(tag).ok()?;
    Kommando::KENNUNGEN
        .get(stelle)
        .map(|(kommando, _)| *kommando)
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

/// Ein Menuebefehl mit der Kombination aus dem Modell, oder ohne.
///
/// Eine Huelle um [`roher_befehl`] und keine zweite Stelle, die ein
/// `NSMenuItem` anlegt: sie entscheidet allein, ob [`appkit_paar`] zu rufen ist.
///
/// Welche Kombination hier ankommt, hat [`crate::menuemodell`] entschieden: die
/// **erste** der Funktion, denn ein `NSMenuItem` haelt genau eine
/// Tastenentsprechung, und keine, wenn die Funktion keine traegt. Der zweite
/// Weg bleibt ueber den Ereignisabgriff erreichbar und steht mit dem ersten in
/// derselben Zeile der Belegungsansicht, wie C3 es verlangt.
fn befehl(
    mtm: MainThreadMarker,
    titel: &NSString,
    aktion: Option<Sel>,
    kombination: Option<Kombination>,
) -> Retained<NSMenuItem> {
    match kombination {
        Some(kombination) => {
            let (kuerzel, zusatztasten) = appkit_paar(kombination);
            roher_befehl(mtm, titel, aktion, &kuerzel, zusatztasten)
        }
        None => ohne_kuerzel(mtm, titel, aktion),
    }
}

/// Ein Menuebefehl ohne Tastenentsprechung.
fn ohne_kuerzel(
    mtm: MainThreadMarker,
    titel: &NSString,
    aktion: Option<Sel>,
) -> Retained<NSMenuItem> {
    roher_befehl(
        mtm,
        titel,
        aktion,
        ns_string!(""),
        NSEventModifierFlags::empty(),
    )
}

/// Ein Menuebefehl mit Titel, Aktion und dem fertigen AppKit-Paar.
///
/// Die eine Stelle, die ein `NSMenuItem` anlegt.
///
/// `aktion` ist eine Moeglichkeit: ein Eintrag ohne Aktion findet in der
/// Antwortkette niemanden und ist grau. Das ist die richtige Anzeige fuer eine
/// benannte Funktion, die diese Runde nicht ausfuehrt.
fn roher_befehl(
    mtm: MainThreadMarker,
    titel: &NSString,
    aktion: Option<Sel>,
    kuerzel: &NSString,
    zusatztasten: NSEventModifierFlags,
) -> Retained<NSMenuItem> {
    // SAFETY: Titel und Kuerzel sind gueltige Zeichenketten, die Auswahl ist
    // ein zur Uebersetzzeit bekannter Selektorname. Ein Ziel setzt der Aufruf
    // nicht, damit die Antwortkette entscheidet.
    let eintrag = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            titel,
            aktion,
            kuerzel,
        )
    };
    eintrag.setKeyEquivalentModifierMask(zusatztasten);
    eintrag
}

// ---------------------------------------------------------------------------
// Die eine Uebersetzung zwischen Kombination und AppKit-Paar
// ---------------------------------------------------------------------------
//
// Sie geht ueber die Tastentabelle aus `krk_core::tasten::parser` und steht
// genau hier. Eine zweite Uebersetzung, die etwa `shift+cmd+w` unmittelbar auf
// das Paar aus `w` und `Command | Shift` abbildete, entstuende als zweite
// Wahrheit ueber dieselbe Kombination.

/// Das AppKit-Paar zu einer Kombination: Kuerzelzeichen und Zusatztastenmaske.
fn appkit_paar(kombination: Kombination) -> (Retained<NSString>, NSEventModifierFlags) {
    let zeichen = match zeichen_der_taste(kombination.taste()) {
        Some(zeichen) => NSString::from_str(&zeichen.to_string()),
        // Kann mit der Tabelle von heute nicht eintreten; eine Pruefung unten
        // haelt das fest. Bleibt die Zuordnung eines Tages doch offen, verliert
        // der Eintrag sein Kuerzel und behaelt seine Beschriftung.
        None => {
            eprintln!(
                "krk: fuer die Taste {} gibt es kein Menuekuerzel",
                kombination.taste().name
            );
            NSString::from_str("")
        }
    };
    (zeichen, maske_nach_appkit(kombination.maske()))
}

/// Das Zeichen, mit dem AppKit diese Taste als Menuekuerzel fuehrt.
///
/// Drei Regeln, keine Liste von Sonderfaellen. Ein einbuchstabiger Name ist
/// sein eigenes Zeichen; das deckt die Buchstaben und die Ziffern ab. `f1` bis
/// `f12` liegen als `NSF1FunctionKey` aufwaerts lueckenlos hintereinander und
/// werden gerechnet. Die uebrigen Namen der Tabelle tragen die Zeichen, die
/// AppKit in `NSEvent.h` unter `NSUpArrowFunctionKey` und Nachbarn fuehrt,
/// beziehungsweise die alten Steuerzeichen.
fn zeichen_der_taste(taste: Taste) -> Option<char> {
    let name = taste.name;
    let mut zeichen = name.chars();
    if let (Some(einziges), None) = (zeichen.next(), zeichen.next()) {
        return Some(einziges);
    }
    if let Some(nummer) = name.strip_prefix('f')
        && let Ok(nummer) = nummer.parse::<u32>()
        && (1..=12).contains(&nummer)
    {
        // `NSF1FunctionKey` ist 0xF704, und die Reihe laeuft ohne Luecke bis
        // `NSF35FunctionKey`.
        return char::from_u32(0xF704 + nummer - 1);
    }
    let besonders = match name {
        "delete" => '\u{0008}', // NSBackspaceCharacter
        "return" => '\u{000D}', // NSCarriageReturnCharacter
        "tab" => '\u{0009}',    // NSTabCharacter
        "esc" => '\u{001B}',    // 0x1B, kein eigener AppKit-Name
        "space" => ' ',
        "up" => '\u{F700}',       // NSUpArrowFunctionKey
        "down" => '\u{F701}',     // NSDownArrowFunctionKey
        "left" => '\u{F702}',     // NSLeftArrowFunctionKey
        "right" => '\u{F703}',    // NSRightArrowFunctionKey
        "pageup" => '\u{F72C}',   // NSPageUpFunctionKey
        "pagedown" => '\u{F72D}', // NSPageDownFunctionKey
        "home" => '\u{F729}',     // NSHomeFunctionKey
        "end" => '\u{F72B}',      // NSEndFunctionKey
        _ => return None,
    };
    Some(besonders)
}

/// Die Zusatztastenmaske als AppKit-Flaggen.
fn maske_nach_appkit(maske: ModMaske) -> NSEventModifierFlags {
    let mut flaggen = NSEventModifierFlags::empty();
    for (bit, appkit) in paare() {
        if maske.enthaelt(bit) {
            flaggen |= appkit;
        }
    }
    flaggen
}

/// Die Zusatztastenmaske aus AppKit-Flaggen.
///
/// Der Rueckweg fuer [`protokollieren`]: dort steht ein `NSMenuItem` und nicht
/// eine Kombination. Die Feststelltaste, der Zehnerblock, die Hilfetaste und
/// `function` fallen weg, wie in `krk_core::tasten::normalisierung`.
fn maske_aus_appkit(flaggen: NSEventModifierFlags) -> ModMaske {
    let mut maske = ModMaske::LEER;
    for (bit, appkit) in paare() {
        if flaggen.contains(appkit) {
            maske |= bit;
        }
    }
    maske
}

/// Die vier Zusatztasten der Schreibweise, je mit ihrer AppKit-Flagge.
///
/// Die Zuordnung steht hier einmal und traegt beide Richtungen. Die Bitwerte
/// selbst stehen nicht hier: `krk-core` fuehrt sie in
/// `normalisierung::roh`, und `ereignisse.rs` haelt sie in
/// `die_acht_rohen_bitwerte_des_kerns_stimmen_mit_appkit_ueberein` gegen genau
/// diese Kiste.
fn paare() -> [(ModMaske, NSEventModifierFlags); 4] {
    [
        (ModMaske::STEUERUNG, NSEventModifierFlags::Control),
        (ModMaske::WAHL, NSEventModifierFlags::Option),
        (ModMaske::UMSCHALT, NSEventModifierFlags::Shift),
        (ModMaske::BEFEHL, NSEventModifierFlags::Command),
    ]
}

// ---------------------------------------------------------------------------
// Die Befehlszeilenmarke --menue-protokoll
// ---------------------------------------------------------------------------

/// Schreibt jeden Eintrag des gebauten Hauptmenues auf die Standardausgabe.
///
/// **Die Pruefung liest aus, statt aufzuzaehlen.** Eine Aufzaehlung der heute
/// bekannten Zusaetze veraltet mit der naechsten macOS-Version, und genau
/// diesen Fall hat das Vorhaben mit "Close All" schon erlebt. Ausgegeben wird
/// deshalb, was am `NSMenu` wirklich haengt, einschliesslich der verdeckten
/// Zweitformen, die AppKit zu diesem Zeitpunkt schon dazugestellt hat. Welche
/// das sind und welche nicht, steht unter `# Was diese Marke nicht sieht`.
///
/// Je Zeile stehen das Untermenue, die Beschriftung, die Kombination in der
/// Schreibweise von `resources/default-keymap.toml`, das rohe AppKit-Paar aus
/// Zeichen und Maske sowie der Selektor. Die Kombination erlaubt den Vergleich
/// gegen die Belegungsdatei, das rohe Paar belegt, was der Eintrag wirklich
/// traegt.
///
/// Vor dem Auslesen bekommt jedes Untermenue ein
/// [`NSMenu::update`](objc2_app_kit::NSMenu::update): das ist der Aufruf, mit
/// dem AppKit ein Menue fuer die Anzeige herrichtet. Dass die dazugestellten
/// Zweitformen ueberhaupt schon dastehen, besorgt der Aufrufer mit
/// `finishLaunching`; die Begruendung samt Messung steht dort.
///
/// # Was diese Marke nicht sieht
///
/// **Die spaet gestellten Zweitformen.** AppKit stellt nicht alle Zusaetze zur
/// selben Zeit: "Close All" zu `performClose:` steht schon nach
/// `finishLaunching` da, "Quit and Keep Windows" zu `terminate:` nicht. Am
/// 260806 mit einer Sonde nachgemessen, die `terminate:` voruebergehend wieder
/// eintrug: die Zweitform erscheint **an keinem** Auslesezeitpunkt dieser
/// Marke — nicht unmittelbar nach `finishLaunching`, nicht nachdem die
/// Ereignisschleife 0,5 s und nicht nachdem sie 2 s gelaufen ist, und auch
/// nicht nach einem vorangestellten `activate`. Der Grund liegt in der Marke
/// selbst: sie oeffnet absichtlich kein Fenster und kehrt zurueck, und ohne
/// Fenster wird die Anwendung nicht aktiv (`isActive()` blieb in allen sechs
/// Messungen `false`). Ein spaeterer Auslesezeitpunkt macht sie also nicht
/// vollstaendig; wer sie vollstaendig haben will, muesste sie in eine
/// laufende Anwendung mit Fenster verlegen und damit zu etwas anderem machen.
///
/// **Was daraus folgt.** Eine Ausgabe ohne Auffaelligkeit belegt das
/// Abnahmekriterium von C3 nur zur Haelfte. Die zweite Haelfte, die
/// dazugestellten Zweitformen der laufenden Anwendung, ist am laufenden
/// Buendel ueber die Bedienungshilfen zu pruefen — so, wie der Befund vom
/// 260805-0753 entstanden ist. Gemeldet war die Luecke als
/// `issues/260805-0841_*_menue-protokoll-sieht-die-spaet-gestellten-zweitformen-nicht.md`.
pub fn protokollieren(hauptmenue: &NSMenu) {
    for oberer in hauptmenue.itemArray().iter() {
        let Some(untermenue) = oberer.submenu() else {
            continue;
        };
        untermenue.update();
        let titel = untermenue.title().to_string();
        for eintrag in untermenue.itemArray().iter() {
            println!("{}", protokollzeile(&titel, &eintrag));
        }
    }
}

/// Eine Zeile des Modus `--menue-protokoll`.
fn protokollzeile(untermenue: &str, eintrag: &NSMenuItem) -> String {
    if eintrag.isSeparatorItem() {
        return format!("menue=\"{untermenue}\" trenner");
    }
    let kuerzel = eintrag.keyEquivalent().to_string();
    let flaggen = eintrag.keyEquivalentModifierMask();
    let selektor = match eintrag.action() {
        Some(auswahl) => auswahl.name().to_string_lossy().into_owned(),
        None => "(keine Aktion)".to_owned(),
    };
    format!(
        "menue=\"{untermenue}\" eintrag=\"{}\" kombination={} kuerzel={:?} zusatztasten={} \
         zweitform={} verdeckt={} selektor={selektor}",
        eintrag.title(),
        geschriebene_kombination(&kuerzel, flaggen),
        kuerzel,
        flaggen.0,
        ja_nein(eintrag.isAlternate()),
        ja_nein(eintrag.isHidden()),
    )
}

/// Das AppKit-Paar eines Eintrags in der Schreibweise der Belegungsdatei.
///
/// `(keines)` fuer einen Eintrag ohne Kuerzel, `(nicht in der Schreibweise)`
/// fuer eines, dessen Zeichen die Tastentabelle nicht kennt. Beide Antworten
/// sind fuer den Vergleich gegen `resources/default-keymap.toml` so gut wie
/// eine Kombination: die Datei kann sie nicht fuehren.
fn geschriebene_kombination(kuerzel: &str, flaggen: NSEventModifierFlags) -> String {
    if kuerzel.is_empty() {
        return "(keines)".to_owned();
    }
    let Some(zeichen) = kuerzel.chars().next() else {
        return "(keines)".to_owned();
    };
    // AppKit schreibt ein Kuerzel mit Umschalttaste gern als Grossbuchstaben.
    // Die Schreibweise der Belegungsdatei kennt nur den Tastennamen, und der
    // ist klein; die Umschalttaste steht dort in der Maske.
    let gesucht = zeichen.to_lowercase().next().unwrap_or(zeichen);
    let treffer = parser::TASTEN
        .into_iter()
        .find(|taste| zeichen_der_taste(*taste) == Some(gesucht));
    match treffer {
        Some(taste) => Kombination::neu(taste, maske_aus_appkit(flaggen)).to_string(),
        None => "(nicht in der Schreibweise)".to_owned(),
    }
}

/// "ja" oder "nein", fuer die Protokollzeile.
fn ja_nein(wahr: bool) -> &'static str {
    if wahr { "ja" } else { "nein" }
}

#[cfg(test)]
mod tests {
    use objc2::runtime::AnyClass;
    use objc2::{ClassType, sel};

    use crate::quellbaum::quelldateien;

    use super::*;
    use objc2_app_kit::{
        NSApplication, NSScrollView, NSTableView, NSTextField, NSTextView, NSWindow,
    };

    /// Die sechs Klassen, die in KRK einen Ersthelfer stellen koennen.
    ///
    /// `NSTableView` steht dreimal im Programm — die Leiste, die beiden
    /// Dateifenster und die Belegungsansicht —, `NSTextView` zweimal, naemlich
    /// als Textflaeche des Editors und als Feldeditor eines Textfeldes;
    /// `NSTextField` traegt die Blaetter. `NSScrollView`, `NSWindow` und
    /// `NSApplication` stehen dazwischen und am Ende der Antwortkette.
    fn ersthelferklassen() -> [(&'static str, &'static AnyClass); 6] {
        [
            ("NSTableView", <NSTableView as ClassType>::class()),
            ("NSTextView", <NSTextView as ClassType>::class()),
            ("NSTextField", <NSTextField as ClassType>::class()),
            ("NSScrollView", <NSScrollView as ClassType>::class()),
            ("NSWindow", <NSWindow as ClassType>::class()),
            ("NSApplication", <NSApplication as ClassType>::class()),
        ]
    }

    /// Die sechs Selektoren der vom Menue zugestellten Textbefehle.
    ///
    /// Dieselben sechs, die [`hauptmenue`] unter "Bearbeiten" eintraegt, und
    /// dieselben sechs, die `resources/default-keymap.toml` mit
    /// `gehalten_von = "menue"` fuehrt. Sie tragen als einzige der 79
    /// Funktionen kein [`krk_core::tasten::Kommando`] und damit keinen
    /// Wirkungsbereich.
    fn die_sechs_zugestellten() -> [(&'static str, Sel); 6] {
        [
            ("cut:", sel!(cut:)),
            ("copy:", sel!(copy:)),
            ("paste:", sel!(paste:)),
            ("selectAll:", sel!(selectAll:)),
            ("undo:", sel!(undo:)),
            ("redo:", sel!(redo:)),
        ]
    }

    /// Was die Laufzeit auf die sechs antwortet, am 260811 gemessen.
    ///
    /// Je Selektor stehen die Klassen aus [`ersthelferklassen`], die ihn
    /// beantworten, in deren Reihenfolge, und zu jeder die Klasse ihrer
    /// Vererbungskette, die die Methode traegt. Eine Klasse, die hier nicht
    /// steht, antwortet nicht.
    ///
    /// Die Angabe der tragenden Klasse ist nicht Beiwerk: sie sagt, dass die
    /// vier Zwischenablage-Befehle nicht an `NSTextView` haengen, sondern an
    /// `NSText` — also an dem Teil der Kette, den auch der Feldeditor eines
    /// `NSTextField` mitbringt. Genau darauf ruht der Satz im Modulkopf, das
    /// Menue erreiche "den Feldeditor des Textfeldes beziehungsweise die
    /// Textflaeche des Editors".
    const GEMESSEN: [(&str, &[(&str, &str)]); 6] = [
        ("cut:", &[("NSTextView", "NSText")]),
        ("copy:", &[("NSTextView", "NSText")]),
        ("paste:", &[("NSTextView", "NSText")]),
        (
            "selectAll:",
            &[("NSTableView", "NSTableView"), ("NSTextView", "NSText")],
        ),
        ("undo:", &[("NSWindow", "NSWindow")]),
        ("redo:", &[("NSWindow", "NSWindow")]),
    ];

    /// Welche der sechs Klassen den Selektor beantworten, und woher.
    ///
    /// Gefragt wird das Objective-C-Laufzeitsystem ueber
    /// [`AnyClass::responds_to`], nicht eine Instanz: die Antwort braucht kein
    /// Fenster, keinen Hauptfaden und keinen Vordergrund. Die tragende Klasse
    /// ist die oberste der Vererbungskette, die noch antwortet;
    /// `class_respondsToSelector` sieht die Kette mit, und wer sie hinauflaeuft,
    /// findet die Stelle, an der die Methode wirklich steht.
    fn wer_antwortet(sel: Sel) -> Vec<(String, String)> {
        let mut gefunden = Vec::new();
        for (name, klasse) in ersthelferklassen() {
            if !klasse.responds_to(sel) {
                continue;
            }
            let mut traegerin = name.to_owned();
            let mut lauf = klasse.superclass();
            while let Some(oberklasse) = lauf {
                if oberklasse.responds_to(sel) {
                    traegerin = oberklasse.name().to_string_lossy().into_owned();
                }
                lauf = oberklasse.superclass();
            }
            gefunden.push((name.to_owned(), traegerin));
        }
        gefunden
    }

    /// Die Messung aus S1 der Runde 3, als mitlaufende Zusicherung.
    ///
    /// **Sie misst, statt zu naehern.** Fuer 73 der 79 Funktionen ist der
    /// Wirkungsbereich aus der Belegung entscheidbar; fuer diese sechs ist er es
    /// nicht, weil sie kein Kommando tragen und die Antwortkette von AppKit zur
    /// Laufzeit entscheidet, wo sie wirken. Was die Ausgabe aus C3 in ihre
    /// dritte Spalte schreibt, ruht deshalb auf dieser Zahlenreihe und nicht auf
    /// einer Ableitung aus der Zugehoerigkeit zum Menue "Bearbeiten".
    ///
    /// Schlaegt sie fehl, hat sich die Laufzeit geaendert und mit ihr die
    /// Auskunft, die KRK dem Nutzer ueber diese sechs Befehle gibt. Der Befund
    /// gehoert dann in die dritte Spalte und nicht in eine angepasste Erwartung.
    ///
    /// **Was sie nicht entscheidet, steht in
    /// [`ein_false_an_der_textklasse_entscheidet_fuer_undo_und_redo_nichts`] und
    /// in [`die_leiste_beantwortet_alles_auswaehlen_von_sich_aus`].**
    #[test]
    fn die_sechs_zugestellten_textbefehle_werden_von_diesen_klassen_beantwortet() {
        for (name, sel) in die_sechs_zugestellten() {
            let Some((_, erwartet)) = GEMESSEN.iter().find(|(gemessen, _)| *gemessen == name)
            else {
                panic!("fuer {name} steht keine Messung in GEMESSEN");
            };
            let gefunden = wer_antwortet(sel);
            let erwartet: Vec<(String, String)> = erwartet
                .iter()
                .map(|(klasse, traegerin)| ((*klasse).to_owned(), (*traegerin).to_owned()))
                .collect();
            assert_eq!(
                gefunden, erwartet,
                "{name} wird von anderen Klassen beantwortet als am 260811 gemessen"
            );
        }
    }

    /// Der Verdachtsfall aus dem Plan, und er trifft zu.
    ///
    /// `text_alles_auswaehlen` liegt auf `selectAll:`, und die Lesezeichen- und
    /// Geraeteleiste ist eine `NSTableView`. Sie beantwortet den Selektor **von
    /// sich aus**, aus einer Methode an `NSTableView` selbst und nicht aus einer
    /// geerbten. Mit dem Fokus in der Leiste weist der stumme Fokusvorbehalt das
    /// Kommando `alle_markieren` ab, der Tastendruck geht unveraendert an
    /// AppKit, und von dort erreicht er den Menueeintrag und die Antwortkette:
    /// der Eintrag ist dort also **bedienbar**. Im Dateifenster kommt der Druck
    /// nie so weit, weil `alle_markieren` ihn dort verbraucht.
    ///
    /// **Damit ist "Textfelder und Editor" fuer diesen einen der sechs keine
    /// gemessene Aussage mehr.** Ob der bedienbare Eintrag in der Leiste auch
    /// etwas **bewirkt**, entscheidet diese Probe nicht: dazu braucht es eine
    /// Instanz und damit den Hauptfaden. Was der Baum dazu haelt, ist die
    /// Auswahleinstellung — `super::super::leiste` und
    /// `super::super::belegungsansicht` setzen beide
    /// `setAllowsMultipleSelection(false)`, die Tabellen der Dateifenster setzen
    /// sie nicht und tragen die Vorgabe von `NSTableView`. Das ist eine
    /// Vermutung ueber die Wirkung und keine Messung.
    #[test]
    fn die_leiste_beantwortet_alles_auswaehlen_von_sich_aus() {
        let tabelle = <NSTableView as ClassType>::class();
        assert!(
            tabelle.responds_to(sel!(selectAll:)),
            "NSTableView beantwortet selectAll: nicht mehr — die dritte Spalte der \
             Tastenbelegung ist daraufhin neu zu entscheiden"
        );
        let traegerin = wer_antwortet(sel!(selectAll:))
            .into_iter()
            .find(|(klasse, _)| klasse == "NSTableView")
            .map(|(_, traegerin)| traegerin);
        assert_eq!(
            traegerin.as_deref(),
            Some("NSTableView"),
            "selectAll: kommt an der Tabelle nicht mehr aus NSTableView selbst"
        );
    }

    /// Fuer `undo:` und `redo:` sagt die Messung ausdruecklich weniger, als sie
    /// zu sagen scheint.
    ///
    /// [`AnyClass::responds_to`] liefert `false` fuer einen Selektor, den eine
    /// Klasse ueber Weiterleitung statt ueber eine eigene Methode beantwortet;
    /// die Schnittstelle nennt genau diesen Fall. Dass `NSTextView` auf `undo:`
    /// und `redo:` mit `false` antwortet, ist deshalb **kein** Beleg dafuer, dass
    /// im Editor niemand antwortet — und die Messung zeigt daneben, wer es tut:
    /// `NSWindow` traegt beide Methoden selbst und reicht sie an den
    /// Rueckgaengigverwalter des Ersthelfers weiter.
    ///
    /// Was der Baum dazu schon weiss, steht im Modulkopf: die `NSTextView` des
    /// Editors bringt ihren Verwalter mit und benutzt ihn, sobald `allowsUndo`
    /// gesetzt ist, und das geschieht in `super::super::editor`. Die Aussage
    /// "wirkt im Editor" ruht also auf dieser Zeile und nicht auf `responds_to`.
    #[test]
    fn ein_false_an_der_textklasse_entscheidet_fuer_undo_und_redo_nichts() {
        let textflaeche = <NSTextView as ClassType>::class();
        let fenster = <NSWindow as ClassType>::class();
        for sel in [sel!(undo:), sel!(redo:)] {
            assert!(
                !textflaeche.responds_to(sel),
                "NSTextView traegt {sel} nun selbst — der Weg ueber NSWindow ist \
                 damit nicht mehr die ganze Geschichte"
            );
            assert!(
                fenster.responds_to(sel),
                "NSWindow beantwortet {sel} nicht mehr — dann erreicht Cmd+Z den \
                 Rueckgaengigverwalter des Editors auf keinem gemessenen Weg"
            );
        }
    }

    /// Ohne diese Zusage verloere ein umbelegter Menueeintrag still sein
    /// Kuerzel.
    #[test]
    fn jede_taste_der_tabelle_hat_ein_menuekuerzel() {
        for taste in parser::TASTEN {
            assert!(
                zeichen_der_taste(taste).is_some(),
                "fuer {} gibt es kein Menuekuerzel",
                taste.name
            );
        }
    }

    /// Zwei Tasten auf ein Zeichen hiessen: das Protokoll kann die eine nicht
    /// von der anderen unterscheiden, und der Vergleich gegen die
    /// Belegungsdatei benennt die falsche Taste.
    #[test]
    fn keine_zwei_tasten_teilen_sich_ein_menuekuerzel() {
        for (stelle, taste) in parser::TASTEN.into_iter().enumerate() {
            for andere in parser::TASTEN.into_iter().skip(stelle + 1) {
                assert_ne!(
                    zeichen_der_taste(taste),
                    zeichen_der_taste(andere),
                    "{} und {} tragen dasselbe Menuekuerzel",
                    taste.name,
                    andere.name
                );
            }
        }
    }

    /// Der Weg, den das Protokoll geht: aus der Kombination in das AppKit-Paar
    /// und zurueck in dieselbe Kombination.
    #[test]
    fn jede_kombination_ueberlebt_den_weg_durch_das_appkit_paar() {
        let masken = [
            ModMaske::LEER,
            ModMaske::BEFEHL,
            ModMaske::UMSCHALT | ModMaske::BEFEHL,
            ModMaske::STEUERUNG | ModMaske::WAHL | ModMaske::UMSCHALT | ModMaske::BEFEHL,
        ];
        for taste in parser::TASTEN {
            for maske in masken {
                let kombination = Kombination::neu(taste, maske);
                let (kuerzel, flaggen) = appkit_paar(kombination);
                assert_eq!(
                    geschriebene_kombination(&kuerzel.to_string(), flaggen),
                    kombination.to_string(),
                    "{kombination} kommt aus dem AppKit-Paar anders zurueck"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Der `tag` als Traeger des Kommandos
    // -----------------------------------------------------------------------

    /// Jedes Kommando kommt aus seinem `tag` unveraendert zurueck.
    ///
    /// Der Weg, den ein Mausklick geht: [`hauptmenue`] setzt den `tag`,
    /// `krkKommando:` liest ihn. Ginge er fuer ein Kommando schief, fuehrte der
    /// Eintrag einen anderen Befehl aus als seine Aufschrift verspricht.
    #[test]
    fn jedes_kommando_ueberlebt_den_weg_durch_den_tag() {
        for (kommando, kennung) in Kommando::KENNUNGEN {
            assert_eq!(
                kommando_zum_tag(tag_des_kommandos(kommando)),
                Some(kommando),
                "{kennung} kommt aus seinem tag anders zurueck"
            );
        }
    }

    /// Kein zweites Kommando teilt sich einen `tag`.
    ///
    /// Der Index in [`Kommando::KENNUNGEN`] ist genau deshalb brauchbar, weil
    /// die Liste jedes Kommando einmal fuehrt. Faende sie eines zweimal, zeigten
    /// zwei Eintraege auf denselben Befehl und einer auf gar keinen.
    #[test]
    fn kein_zweites_kommando_teilt_sich_einen_tag() {
        let mut vergeben: Vec<NSInteger> = Kommando::KENNUNGEN
            .into_iter()
            .map(|(kommando, _)| tag_des_kommandos(kommando))
            .collect();
        let gezaehlt = vergeben.len();
        vergeben.sort_unstable();
        vergeben.dedup();
        assert_eq!(
            vergeben.len(),
            gezaehlt,
            "zwei Kommandos teilen sich einen tag"
        );
    }

    /// Ein `tag` ausserhalb der Liste benennt kein Kommando, und ein negativer
    /// auch nicht.
    ///
    /// Der Vorgabewert eines `tag` ist Null und benennt damit das **erste**
    /// Kommando; deshalb fragt `validateMenuItem:` zuerst nach der Aktion. Was
    /// diese Probe haelt, ist der andere Rand: aus einem `tag`, den niemand
    /// gesetzt hat, wird kein Absturz.
    #[test]
    fn ein_tag_ausserhalb_der_liste_benennt_kein_kommando() {
        let hinter_dem_ende =
            NSInteger::try_from(Kommando::KENNUNGEN.len()).expect("die Liste ist kurz");
        assert_eq!(kommando_zum_tag(hinter_dem_ende), None);
        assert_eq!(kommando_zum_tag(-1), None);
        assert!(kommando_zum_tag(hinter_dem_ende - 1).is_some());
    }

    // -----------------------------------------------------------------------
    // C2.10: eine Stelle legt an, eine Stelle uebersetzt
    // -----------------------------------------------------------------------

    /// Genau eine Stelle legt ein `NSMenuItem` an, und genau eine uebersetzt
    /// eine Kombination in das AppKit-Paar (C2.10).
    ///
    /// **Zwei Erklaerungszaehlungen.** Sie halten, was sie versprechen: eine
    /// zweite Stelle, die ein `NSMenuItem` baut oder eine Kombination von Hand
    /// auf Zeichen und Maske abbildet, laesst sie rot werden. Die Begruendung
    /// fuer die Unterscheidung zwischen Erklaerungs- und Aufruferzaehlung steht
    /// in [`crate::quellbaum`].
    ///
    /// Die Huellen [`befehl`] und [`ohne_kuerzel`] zaehlen nicht mit: sie legen
    /// nichts an, sondern rufen [`roher_befehl`]. `NSMenuItem::separatorItem`
    /// zaehlt ebenfalls nicht — AppKit haelt fuer den Trenner ein eigenes,
    /// gemeinsam benutztes Objekt, und der Aufruf baut keines.
    ///
    /// Die Nadeln stehen zusammengesetzt da, wie bei
    /// `es_gibt_genau_einen_menuebauer` in [`super::teilen`]: als ein Stueck
    /// geschrieben faenden sie sich selbst.
    #[test]
    fn es_gibt_eine_stelle_je_anlage_und_uebersetzung() {
        let dateien = quelldateien();
        for (nadel, was) in [
            (concat!("NSMenuItem", "::alloc("), "legt ein NSMenuItem an"),
            (
                concat!("initWithTitle_action_", "keyEquivalent("),
                "ruft den Erzeuger von NSMenuItem",
            ),
            (
                concat!("fn ", "appkit_paar("),
                "uebersetzt eine Kombination",
            ),
        ] {
            let gefunden: usize = dateien
                .iter()
                .map(|(_, inhalt)| inhalt.matches(nadel).count())
                .sum();
            assert_eq!(gefunden, 1, "es gibt nicht genau eine Stelle, die {was}");
        }
    }

    /// Die Ausgrauung eines Menueeintrags wird an genau einer Stelle
    /// entschieden, und es ist `validateMenuItem:` (C2.17).
    ///
    /// **Die Umkehrung von C2.5, und sie ist eine Zaehlung und keine Rechnung.**
    /// Dass der Abgriff und die Ausgrauung nie verschiedene Antworten geben,
    /// folgt daraus, dass beide dieselbe reine Funktion auf derselben `Lage`
    /// fragen; die Tafel aus 140 Faellen dazu steht in
    /// [`crate::kommandos::zulaessigkeit`], und die zwei Aufrufer zaehlt die
    /// Probe daneben. Was **hier** zu halten ist, ist die andere Haelfte: dass
    /// niemand die Freigabe eines Eintrags an einer zweiten Stelle **setzt**.
    /// Ein `setEnabled:` irgendwo im Baum uebersteuerte die Regel lautlos, und
    /// ein abgeschaltetes `setAutoenablesItems` naehme sie ganz weg — dann waere
    /// jeder Eintrag immer frei, und mit dem Fokus im Editor bewegte ein
    /// Auf-Pfeil die Dateiliste.
    #[test]
    fn die_freigabe_eines_eintrags_wird_nirgends_gesetzt() {
        let dateien = quelldateien();
        for nadel in [
            concat!("setEnabled", "("),
            concat!("setAutoenablesItems", "("),
        ] {
            let gefunden: Vec<&str> = dateien
                .iter()
                .filter(|(_, inhalt)| inhalt.contains(nadel))
                .map(|(name, _)| name.as_str())
                .collect();
            assert!(
                gefunden.is_empty(),
                "{nadel} steht in {gefunden:?} und uebersteuert damit die \
                 Zulaessigkeitsregel"
            );
        }
        // Gezaehlt wird die **Erklaerung** und nicht die Nennung: der Name
        // steht in etlichen Doc-Kommentaren dieser Runde, die Methode selbst
        // genau einmal.
        let pruefungen: usize = dateien
            .iter()
            .map(|(_, inhalt)| {
                inhalt
                    .matches(concat!("unsafe(method(validateMenu", "Item:))"))
                    .count()
            })
            .sum();
        assert_eq!(
            pruefungen, 1,
            "die Ausgrauung wird nicht an genau einer Stelle entschieden"
        );
    }

    // -----------------------------------------------------------------------
    // C2.14: ein Ausfuehrungsweg
    // -----------------------------------------------------------------------

    /// Der Anwendungsdelegierte fuehrt einen Befehl an genau drei Stellen aus,
    /// und alle drei rufen dieselbe Methode (C2.14).
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier, weil C2.14 die Zahl selbst
    /// zusagt.** Die drei sind die drei Wege, auf denen ein Befehl in KRK
    /// ausgeloest wird: der Tastendruck ueber `eingabe_ausfuehren`, der
    /// Menueeintrag ueber `krkKommando:` und der Klick in die Bereichsleiste
    /// ueber ihren Melder. Sie enden alle in `kommando_ausfuehren`; ein vierter
    /// Weg, der den Rumpf eines Befehls an dieser Stelle vorbei erreicht, laesst
    /// die Probe rot werden.
    ///
    /// Gezaehlt werden die zwei Empfaengernamen und nicht der blosse
    /// Methodenname: `kommando_ausfuehren` heisst daneben auch je eine Methode
    /// an der Tabelle, an der Leiste und an der Vorschau, an die der Delegierte
    /// weiterreicht. Das sind keine zweiten Ausfuehrungswege, sondern die
    /// Fortsetzung dieses einen.
    #[test]
    fn der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten() {
        let nadel = concat!("kommando_", "ausfuehren(");
        let aufrufe: usize = quelldateien()
            .iter()
            .map(|(_, inhalt)| {
                ["self.", "selbst."]
                    .into_iter()
                    .map(|empfaenger| inhalt.matches(&format!("{empfaenger}{nadel}")).count())
                    .sum::<usize>()
            })
            .sum();
        assert_eq!(
            aufrufe, 3,
            "der eine Ausfuehrungsweg hat nicht die drei Aufrufer Tastendruck, \
             Menueeintrag und Bereichsleiste"
        );
    }

    // -----------------------------------------------------------------------
    // C2.11: zwei Bauanlaesse
    // -----------------------------------------------------------------------

    /// Das Menue wird an genau zwei Anlaessen gebaut (C2.11).
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier, weil C2.11 die Zahl selbst
    /// zusagt.** Die zwei sind der Start und die Rueckkehr aus der
    /// Belegungsansicht; ein Kuerzel, das der Nutzer dort aendert, steht danach
    /// im Menue. Faellt der zweite Aufruf weg, bleibt die Aenderung bis zum
    /// naechsten Start unsichtbar — ohne dass irgendeine andere Probe es
    /// bemerkte.
    #[test]
    fn das_menue_wird_an_zwei_anlaessen_gebaut() {
        let nadel = concat!("haupt", "menue(");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(name, _)| name != "appkit/menue.rs")
            .map(|(_, inhalt)| inhalt.matches(nadel).count())
            .sum();
        assert_eq!(
            aufrufe, 2,
            "das Hauptmenue wird nicht an genau zwei Anlaessen gebaut"
        );
    }
}
