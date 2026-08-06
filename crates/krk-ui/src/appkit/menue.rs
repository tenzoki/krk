//! Das Hauptmenue, von Hand gebaut, mit den Kuerzeln aus der Belegung.
//!
//! Der Technologieentscheid bringt keinen Oberflaechenbau mit: es gibt kein
//! `MainMenu.nib`, aus dem AppKit das Menue laedt. Jeder Eintrag entsteht
//! deshalb hier im Programmtext.
//!
//! Drei Untermenues: KRK, Bearbeiten, Fenster. Jeder Eintrag bekommt als Ziel
//! `nil` und laeuft damit ueber die Antwortkette. `cut:`, `copy:`, `paste:` und
//! `selectAll:` erreichen den Feldeditor des Textfeldes mit dem Fokus, und
//! `beenden:`, `fensterEinblenden:` wie `fensterSchliessen:` erreichen den
//! Anwendungsdelegierten, an dem die Kette endet. Ein fest gesetztes Ziel
//! wuerde die Kette umgehen und einen Eintrag auch dann aktiv lassen, wenn
//! niemand ihn beantworten kann.
//!
//! # Eine Quelle, zwei sichtbare Wege
//!
//! **Kein Kuerzel steht hier als Zeichenkette, ohne Ausnahme.** [`hauptmenue`]
//! bekommt die Belegung gereicht und holt das Kuerzel jedes Eintrags unter
//! dessen Kennung aus ihr. Damit ist `resources/default-keymap.toml` auch fuer
//! das Menue die alleinige Quelle: die Konflikterkennung aus C3 sieht jede
//! Kombination, der Nutzer kann jede umbelegen, und eine Umbelegung wirkt auf
//! beide Wege. Nutzerentscheid vom 260805-0000,
//! `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`.
//!
//! Bis zum 260805-0820 stand hier eine Ausnahme: der Eintrag "KRK beenden"
//! trug Cmd+Q als Zeichenkette im Programmtext, weil die Belegungsdatei die
//! Funktion `beenden` noch nicht fuehrte. Sie fuehrt sie seither, und die
//! Ausnahme ist mit ihr weggefallen
//! (`issues/260805-0753_c_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md`).
//!
//! Welchen der beiden Wege ein Tastendruck geht, entscheidet der Fokus. Der
//! Ereignisabgriff aus [`super::ereignisse`] sieht ihn vor der Menuebehandlung
//! von `NSApplication`. Steht die Schreibmarke in einem Textfeld, kehrt er
//! sofort zurueck und reicht weiter; dann wirkt das Menue. Steht sie im
//! Dateifenster, schlaegt er in der Belegung nach — und die vom Menue
//! gehaltenen Funktionen sieht er dabei nicht, weil `Belegung::nachschlag` sie
//! ueberspringt. Die vier Textbefehle laufen deshalb auch im Dateifenster ins
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
//! `issues/260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`.
//!
//! # Zwei Eintraege tragen einen eigenen Selektor
//!
//! **"Fenster schliessen".** Bis Schritt 13c stand dort `performClose:`, und
//! AppKit stellte von sich aus eine Zweitform "Close All" auf Opt+Shift+Cmd+W
//! dazu, mit englischer Beschriftung und einer Kombination, die niemand aus der
//! Belegung setzen oder umbelegen kann (gemessen am 260804-1040 im signierten
//! Buendel,
//! `issues/260804-1040_c_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md`).
//! Der Eintrag traegt deshalb den eigenen Selektor `fensterSchliessen:` am
//! Anwendungsdelegierten, so wie "Fenster einblenden" ihn seit Schritt 12 hat;
//! der Delegierte ruft darauf `performClose:` am Fenster selbst, sodass sich am
//! Verhalten nichts aendert.
//!
//! **"KRK beenden", aus demselben Grund.** Zu `terminate:` stellt AppKit eine
//! Zweitform "Quit and Keep Windows" auf Opt+Cmd+Q dazu. Sie erscheint spaeter
//! als die von "Close All": nicht schon nach `finishLaunching`, sondern erst an
//! der wirklich laufenden Anwendung, weshalb `--menue-protokoll` sie nicht sah
//! und der Befund vom 260805-0753 ueber die Bedienungshilfen kam
//! (`issues/260805-0753_c_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md`).
//! Der Eintrag traegt seither `beenden:`, und der Delegierte ruft `terminate:`
//! an `NSApplication` selbst. Gegengeprueft wie bei "Close All", am selben Weg
//! wie der Befund: am 260805 traegt das Menue "KRK" der laufenden Anwendung
//! genau einen Eintrag, "KRK beenden" auf Cmd+Q; die Zweitform ist fort. Der
//! `inference:` des Defekts, die Zweitform haenge allein an `terminate:`, ist
//! damit nachgemessen.
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
//! `decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
//! gewaehlt. Der Eintrag heisst "Fenster einblenden" und nicht "Neues Fenster",
//! weil er keines anlegt: KRK haelt in dieser Runde genau ein
//! Anwendungsfenster, es ueberlebt sein Schliessen, und der Eintrag holt es
//! nach vorn. Die Runde, die mehrere Fenster einfuehrt, benennt ihn um und
//! behaelt das Kuerzel.

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};
use objc2::{MainThreadOnly, sel};
use objc2_app_kit::{NSEventModifierFlags, NSMenu, NSMenuItem};
use objc2_foundation::{
    MainThreadMarker, NSDictionary, NSNumber, NSString, NSUserDefaults, ns_string,
};

use krk_core::tasten::parser::{self, Taste};
use krk_core::tasten::{Belegung, Kombination, ModMaske};

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
/// `issues/260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`.
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

/// Baut das Hauptmenue der Anwendung aus der Belegung.
pub fn hauptmenue(mtm: MainThreadMarker, belegung: &Belegung) -> Retained<NSMenu> {
    let hauptmenue = NSMenu::new(mtm);
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("KRK"),
        &[befehl(
            mtm,
            belegung,
            ns_string!("KRK beenden"),
            sel!(beenden:),
            "beenden",
        )],
    ));
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("Bearbeiten"),
        &[
            befehl(
                mtm,
                belegung,
                ns_string!("Ausschneiden"),
                sel!(cut:),
                "text_ausschneiden",
            ),
            befehl(
                mtm,
                belegung,
                ns_string!("Kopieren"),
                sel!(copy:),
                "text_kopieren",
            ),
            befehl(
                mtm,
                belegung,
                ns_string!("Einfügen"),
                sel!(paste:),
                "text_einfuegen",
            ),
            befehl(
                mtm,
                belegung,
                ns_string!("Alles auswählen"),
                sel!(selectAll:),
                "text_alles_auswaehlen",
            ),
        ],
    ));
    hauptmenue.addItem(&untermenue(
        mtm,
        ns_string!("Fenster"),
        &[
            befehl(
                mtm,
                belegung,
                ns_string!("Fenster einblenden"),
                sel!(fensterEinblenden:),
                "fenster_einblenden",
            ),
            befehl(
                mtm,
                belegung,
                ns_string!("Fenster schließen"),
                sel!(fensterSchliessen:),
                "fenster_schliessen",
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

/// Ein Menuebefehl, dessen Kuerzel unter `kennung` in der Belegung steht.
///
/// Traegt die Funktion mehrere Kombinationen, nimmt der Eintrag die **erste**:
/// ein `NSMenuItem` haelt genau eine Tastenentsprechung. Der zweite Weg bleibt
/// ueber den Ereignisabgriff erreichbar und steht mit dem ersten in derselben
/// Zeile der Belegungsansicht, wie C3 es verlangt.
///
/// Traegt sie gar keine, bekommt der Eintrag keine: der Nutzer hat die Belegung
/// aufgehoben, und ein Kuerzel aus dem Programmtext daruebersetzen hiesse, die
/// Aufhebung zu uebergehen.
fn befehl(
    mtm: MainThreadMarker,
    belegung: &Belegung,
    titel: &NSString,
    aktion: Sel,
    kennung: &str,
) -> Retained<NSMenuItem> {
    let Some(funktion) = belegung.funktion(kennung) else {
        // Kein Nutzerfehler, sondern einer im Programm: die Kennung steht in
        // keiner Zeile von `resources/default-keymap.toml`. Ein Menue ohne
        // Kuerzel ist die vertretbare Folge, ein stilles Weglassen waere es
        // nicht.
        eprintln!("krk: die Belegung kennt keine Funktion namens {kennung}");
        return ohne_kuerzel(mtm, titel, aktion);
    };
    match funktion.tasten().first() {
        Some(kombination) => {
            let (kuerzel, zusatztasten) = appkit_paar(*kombination);
            roher_befehl(mtm, titel, aktion, &kuerzel, zusatztasten)
        }
        None => ohne_kuerzel(mtm, titel, aktion),
    }
}

/// Ein Menuebefehl ohne Tastenentsprechung.
fn ohne_kuerzel(mtm: MainThreadMarker, titel: &NSString, aktion: Sel) -> Retained<NSMenuItem> {
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
fn roher_befehl(
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
    use super::*;

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

    /// Die Kennungen, unter denen `hauptmenue` seine Kuerzel sucht, stehen in
    /// der Auslieferungsbelegung. Ohne diese Zusage faende der Aufbau sie beim
    /// Start nicht und schriebe eine Meldung, die niemand liest.
    #[test]
    fn jede_kennung_des_hauptmenues_steht_in_der_auslieferungsbelegung() {
        let belegung = Belegung::auslieferung();
        for kennung in [
            "beenden",
            "text_ausschneiden",
            "text_kopieren",
            "text_einfuegen",
            "text_alles_auswaehlen",
            "fenster_einblenden",
            "fenster_schliessen",
        ] {
            let Some(funktion) = belegung.funktion(kennung) else {
                panic!("die Auslieferungsbelegung kennt {kennung} nicht");
            };
            assert!(
                !funktion.tasten().is_empty(),
                "{kennung} traegt ab Werk keine Kombination"
            );
        }
    }
}
