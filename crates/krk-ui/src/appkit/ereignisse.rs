//! Der Ereignisabgriff: der einzige Eintrittspunkt fuer Tastendruecke.
//!
//! **Ein Abgriff, kein zweiter Weg.** Jeder Tastendruck laeuft durch den
//! lokalen Ereignisabgriff `NSEvent.addLocalMonitorForEventsMatchingMask`, und
//! keine Ansicht bekommt eine eigene `keyDown:`-Behandlung. Das ist die
//! Voraussetzung dafuer, dass die Belegung aus Schritt 11 spaeter wirklich
//! jede Taste traegt: eine Ansicht, die eine Taste selbst abfaengt, waere die
//! Sonderregel mit eigenem Rueckfallweg, die die Maxime "supersimpel"
//! ausschliesst.
//!
//! Der Abgriff ist **lokal** und nicht global. Ein globaler Abgriff sieht die
//! Tasten anderer Anwendungen und braucht dafuer die Freigabe fuer
//! Bedienungshilfen. Die Messung vom 260802-1137 hat belegt, dass der lokale
//! Abgriff einer gewoehnlichen Anwendung im Vordergrund auch die
//! Funktionstasten sieht; KRK braucht die Freigabe deshalb nicht.
//!
//! **Der Weg eines Tastendrucks**, vom Ereignis bis zur Ausfuehrung:
//!
//! ```text
//! NSEvent ──> Fokusvorbehalt ──> Tastendruck::aus_ereignis ──> Belegung::nachschlag
//!                  │                  (Maske normalisiert)          │
//!            Textfeld? ──> weiter                        Kommando ──┤
//!                                                      Sprungmarke ──> Zeichen
//!                                                                      │
//!                                                        Senke des Aufrufers
//! ```
//!
//! Trifft der Nachschlag und fuehrt die Senke das Kommando aus, schluckt der
//! Abgriff das Ereignis (er liefert `nil`); sonst reicht er es unveraendert
//! weiter, damit Cmd+Q, Shift+Cmd+W und die Texteingabe des Systems ihren
//! gewohnten Weg gehen.
//!
//! # Der Fokusvorbehalt
//!
//! **Tastenbefehle wirken im Dateifenster; Textfelder und Blaetter behalten
//! ihre AppKit-Bedeutung.** Der Abgriff sieht jeden Tastendruck der Anwendung,
//! gleich wo der Eingabefokus steht. C2 verlangt fuer jedes Textfeld die
//! gewohnte Mac-Bedeutung: Return bestaetigt, Cmd+Links und Cmd+Rechts bewegen
//! die Schreibmarke an Zeilenanfang und Zeilenende. Seit S11c liegt der Auf-
//! und Abstieg genau auf diesen beiden Kombinationen, und ohne den Vorbehalt
//! waere die Pfadeingabe aus C2 damit nicht bedienbar: das Blatt stuende offen,
//! und Cmd+Links wechselte hinter ihm den Ordner.
//!
//! Der Abgriff fragt deshalb **vor** dem Nachschlag, ob der Ersthelfer des
//! Schluesselfensters ein Textfeld ist, und reicht den Tastendruck in diesem
//! Fall unveraendert weiter. Der Vorbehalt sitzt hier und nicht je Blatt: die
//! fuenf Blaetter aus S16 und S17 erben ihn dadurch, ohne ihn zu wiederholen.
//! Gemeldet war das als
//! `issues/260804-1122_o_der-fokusvorbehalt-fuer-tastenbefehle-steht-nur-fuer-die-loeschtasten.md`.
//!
//! **Der Abgriff kennt kein Dateifenster.** Bis Schritt 11 reichte er das
//! Kommando unmittelbar an die eine Datenquelle weiter. Seit Schritt 12 gibt es
//! zwei Dateifenster und Kommandos, die keinem von beiden gehoeren, etwa das
//! Ein- und Ausblenden der Bereiche; er nimmt deshalb eine gewoehnliche
//! Rust-Senke entgegen und laesst den Aufrufer entscheiden, wohin ein Kommando
//! geht. Dieselbe Form wie [`super::bildtakt::Zeichenende`]. Sie haelt
//! zugleich die Modulordnung: `ereignisse` kennt `anwendung` nicht, und ein
//! Ring zwischen den beiden entsteht nicht.
//!
//! **Der Nachschlag geht seit Schritt 11 in die Belegung und nicht mehr in eine
//! verdrahtete Tabelle.** Die [`Belegung`] kommt beim Einrichten von aussen:
//! der Aufrufer laedt sie ueber [`belegung::fuer_den_betrieb`] und stellt die
//! Meldung, falls es eine gab, in die Statuszeile.
//!
//! **Geschluckt wird nur, was auch ausgefuehrt wurde.** Die Belegung kennt jede
//! Funktion aus C1 bis C7, gebaut ist in dieser Runde ein Teil davon. Eine
//! Taste, die einer noch ungebauten Funktion gehoert, geht deshalb unveraendert
//! weiter, statt ins Leere geschluckt zu werden; sonst naehme der Abgriff dem
//! Menue ein Kuerzel ab, ohne etwas an seine Stelle zu setzen.
//!
//! # Die Sprungmarke kommt als Zeichen und nicht als Kommando
//!
//! Eine Taste ohne Zusatztaste, die keiner Funktion gehoert, faellt im Kern auf
//! [`Nachschlag::Sprungmarke`]. Der Kern kennt allein den Tastencode und weiss
//! nicht, welches Zeichen darauf liegt; das weiss das Ereignis. Der Abgriff
//! reicht deshalb das Zeichen weiter, und die Regel, welche Zeichen ein
//! Dateiname tragen kann, steht in `krk_core::verzeichnis::sprungmarke`.

use std::ptr::NonNull;

use block2::RcBlock;
use objc2::ClassType;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSApplication, NSEvent, NSEventMask, NSEventModifierFlags, NSEventType, NSText, NSTextField,
    NSTextView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSObjectProtocol, NSPoint, NSProcessInfo, NSString};

use krk_core::tasten::Belegung;
use krk_core::tasten::{Kombination, Kommando, Nachschlag, Tastendruck, code_von_pflicht};

/// Was der Abgriff an den Aufrufer weitergibt.
///
/// Zwei Sorten, weil ein Tastendruck zwei Dinge sein kann: eine nachgeschlagene
/// Funktion oder ein getipptes Zeichen fuer die Sprungmarke aus C2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eingabe {
    /// Eine belegte Kombination.
    Kommando(Kommando),
    /// Ein Zeichen fuer die Sprungmarke aus C2.
    ///
    /// Ob es ueberhaupt in den Puffer gehoert, entscheidet der Kern; der
    /// Abgriff reicht weiter, was das Ereignis traegt.
    Zeichen(char),
}

/// Ein eingerichteter Ereignisabgriff.
///
/// Der Abgriff bleibt bestehen, solange dieser Wert lebt. Wer ihn fallen
/// laesst, nimmt ihn damit zurueck.
pub struct Tastenabgriff {
    /// Das Merkzeichen, das AppKit beim Einrichten liefert. Es gibt nichts
    /// preis; es wird allein gebraucht, um den Abgriff wieder abzumelden.
    merkzeichen: Retained<AnyObject>,
}

impl Tastenabgriff {
    /// Richtet den Abgriff ein und leitet jedes gefundene Kommando an `senke`.
    ///
    /// Die Senke liefert zurueck, ob sie das Kommando ausgefuehrt hat; nur dann
    /// schluckt der Abgriff das Ereignis.
    ///
    /// Liefert `None`, wenn AppKit den Abgriff nicht einrichtet. Der Aufrufer
    /// meldet das; still ohne Tastatur weiterzulaufen waere der schlechteste
    /// aller Ausgaenge.
    ///
    /// `protokoll` schaltet den Modus `--tasten-protokoll`: jeder empfangene
    /// Tastendruck geht mit seinem Code und seiner normalisierten Maske auf die
    /// Standardausgabe, gleich ob die Belegung ihn kennt.
    pub fn einrichten(
        mtm: MainThreadMarker,
        belegung: Belegung,
        protokoll: bool,
        senke: impl Fn(Eingabe) -> bool + 'static,
    ) -> Option<Self> {
        let block = RcBlock::new(move |ereignis: NonNull<NSEvent>| -> *mut NSEvent {
            // SAFETY: AppKit reicht dem Block einen gueltigen Zeiger auf das
            // Ereignis, das fuer die Dauer des Aufrufs lebt.
            let geschluckt = behandeln(
                mtm,
                &senke,
                &belegung,
                unsafe { ereignis.as_ref() },
                protokoll,
            );
            if geschluckt {
                // `nil` heisst: das Ereignis geht nicht weiter.
                std::ptr::null_mut()
            } else {
                // Unveraendert weiterreichen. Der Zeiger ist derselbe, den
                // AppKit hereingegeben hat; er wechselt keinen Besitzer.
                ereignis.as_ptr()
            }
        });

        // SAFETY: Die Bindung stellt genau eine Bedingung, "`block` block's
        // return must be a valid pointer or null"
        // (`objc2-app-kit-0.3.2/src/generated/NSEvent.rs:1173-1175`). Der Block
        // oben liefert nichts anderes: entweder `null_mut`, oder den Zeiger,
        // den AppKit selbst hereingegeben hat und der fuer die Dauer des
        // Aufrufs gilt. Signatur und Lebensdauer stehen hier nicht als
        // Begruendung, weil die erste der Uebersetzer prueft und die zweite
        // `RcBlock` regelt.
        let merkzeichen = unsafe {
            NSEvent::addLocalMonitorForEventsMatchingMask_handler(NSEventMask::KeyDown, &block)
        }?;
        Some(Self { merkzeichen })
    }
}

impl Drop for Tastenabgriff {
    fn drop(&mut self) {
        // SAFETY: Das Merkzeichen stammt aus
        // `addLocalMonitorForEventsMatchingMask:handler:` und ist damit von der
        // Art, die `removeMonitor:` erwartet.
        unsafe { NSEvent::removeMonitor(&self.merkzeichen) };
    }
}

/// Der virtuelle Tastencode von Pfeil ab.
///
/// Die Zahl steht hier nicht: sie kommt zur Uebersetzungszeit aus der einen
/// Tastentabelle des Kerns. Ein Tippfehler im Namen bricht den Bau ab, statt
/// eine zweite Wahrheit ueber denselben Tastencode anzulegen.
const CODE_PFEIL_AB: u16 = code_von_pflicht("down");

/// Das Zeichen, das AppKit einem Pfeil ab beilegt (`NSDownArrowFunctionKey`).
const ZEICHEN_PFEIL_AB: char = '\u{F701}';

/// Stellt ein Pfeil-ab-Ereignis in die eigene Ereignisschlange.
///
/// Die Messung von L1 braucht einen Tastendruck, den kein Mensch ausloest, und
/// sie braucht ihn zwanzigmal. Das Ereignis geht denselben Weg wie ein
/// koerperlicher Druck: ueber die Schlange der Anwendung in den lokalen
/// Abgriff oben, durch die Normalisierung und den Nachschlag im Kern bis in die
/// Datenquelle. Nichts an [`behandeln`] ist dafuer geaendert.
///
/// **Was das nicht belegt.** Dass eine koerperlich gedrueckte Taste dieselben
/// Ereignisse erzeugt, ist damit nicht gemessen. Die Marken `function` und
/// `numericPad` setzt dieser Aufruf selbst, weil AppKit sie bei den
/// Pfeiltasten setzt; belegt ist das aus der Messung vom 260802-1137 und nicht
/// aus dieser Sonde. Der Messbericht schreibt beides aus.
pub fn pfeil_ab_senden(mtm: MainThreadMarker, fenster: &NSWindow) {
    let zeichen = NSString::from_str(&ZEICHEN_PFEIL_AB.to_string());
    let ereignis = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
        NSEventType::KeyDown,
        NSPoint::ZERO,
        NSEventModifierFlags::Function | NSEventModifierFlags::NumericPad,
        NSProcessInfo::processInfo().systemUptime(),
        fenster.windowNumber(),
        None,
        &zeichen,
        &zeichen,
        false,
        CODE_PFEIL_AB,
    );
    match ereignis {
        // `atStart: false` haengt das Ereignis hinten an, wie es das System
        // mit einem echten Tastendruck tut. Vorn einzureihen wuerde die
        // Schlange umsortieren und damit etwas anderes messen.
        Some(ereignis) => NSApplication::sharedApplication(mtm).postEvent_atStart(&ereignis, false),
        // AppKit gibt hier nur bei einem falsch gebauten Ereignis `nil`
        // zurueck. Still weiterzumessen hiesse, eine Wiederholung zu zaehlen,
        // die nie stattgefunden hat.
        None => eprintln!("krk: das synthetische Tastenereignis liess sich nicht bauen"),
    }
}

/// Wertet ein Tastenereignis aus. Liefert, ob es geschluckt wurde.
fn behandeln(
    mtm: MainThreadMarker,
    senke: &impl Fn(Eingabe) -> bool,
    belegung: &Belegung,
    ereignis: &NSEvent,
    protokoll: bool,
) -> bool {
    // Der Fokusvorbehalt, vor dem Nachschlag. Siehe den Modulkopf: steht die
    // Schreibmarke in einem Textfeld, behaelt jede Taste ihre AppKit-Bedeutung.
    if ersthelfer_nimmt_text(mtm) {
        return false;
    }

    let druck = Tastendruck::aus_ereignis(ereignis.keyCode(), ereignis.modifierFlags().0 as u64);
    let nachschlag = belegung.nachschlag(druck);

    if protokoll {
        protokollieren(druck, nachschlag);
    }

    match nachschlag {
        // Belegt und gebaut. Eine Funktion ohne Kommando ist belegt, aber in
        // dieser Runde noch nicht gebaut; siehe den Modulkopf: geschluckt wird
        // nur, was auch ausgefuehrt wurde.
        Nachschlag::Funktion(funktion) => match funktion.kommando() {
            Some(kommando) => senke(Eingabe::Kommando(kommando)),
            None => false,
        },
        // Eine Taste ohne Zusatztaste, die keiner Funktion gehoert: das Tippen
        // der Anfangsbuchstaben aus C2. Ob das Zeichen in den Puffer gehoert,
        // entscheidet der Kern.
        Nachschlag::Sprungmarke => match getipptes_zeichen(ereignis) {
            Some(zeichen) => senke(Eingabe::Zeichen(zeichen)),
            None => false,
        },
        Nachschlag::Unbelegt => false,
    }
}

/// Ob der Ersthelfer des Schluesselfensters Text entgegennimmt.
///
/// Gefragt ist das **Schluesselfenster** und nicht das Hauptfenster: steht ein
/// Blatt am Fenster, ist dessen Panel das Schluesselfenster, und dort sitzt das
/// Textfeld der Pfadeingabe.
///
/// Ein `NSTextField` gibt beim Bearbeiten seinen Ersthelferrang an den
/// Feldeditor ab, einen gemeinsam genutzten `NSTextView`. Gefragt sind deshalb
/// beide Klassen: das Feld selbst, solange es nur ausgewaehlt ist, und der
/// Feldeditor, sobald die Schreibmarke darin steht. `NSText` deckt daneben die
/// aelteren Textklassen ab, die AppKit weiterhin fuehrt.
fn ersthelfer_nimmt_text(mtm: MainThreadMarker) -> bool {
    let Some(fenster) = NSApplication::sharedApplication(mtm).keyWindow() else {
        return false;
    };
    let Some(ersthelfer) = fenster.firstResponder() else {
        return false;
    };
    ersthelfer.isKindOfClass(NSTextView::class())
        || ersthelfer.isKindOfClass(NSTextField::class())
        || ersthelfer.isKindOfClass(NSText::class())
}

/// Das Zeichen, das dieses Ereignis traegt.
///
/// `None` fuer ein Ereignis ohne Zeichen, etwa eine reine Zusatztaste. Genommen
/// wird das **erste** Zeichen: eine Taste liefert in aller Regel genau eines,
/// und eine Folge aus mehreren stammt von einer Eingabemethode, deren Ergebnis
/// nicht in einen Suchpuffer gehoert.
fn getipptes_zeichen(ereignis: &NSEvent) -> Option<char> {
    let zeichen = ereignis.characters()?;
    zeichen.to_string().chars().next()
}

/// Schreibt eine Zeile des Modus `--tasten-protokoll`.
///
/// Auf die Standardausgabe, wie der Plan es vorschreibt. Sichtbar ist sie nur,
/// wenn KRK aus einem Terminal gestartet wurde: ein ueber `open` gestartetes
/// Buendel bekommt von LaunchServices keine.
///
/// Die Zeile nennt den Tastencode, weil die Abnahme von Schritt 7 daran haengt,
/// und daneben die Kombination in der Schreibweise von `keymap.toml`, damit der
/// Nutzer sie von hier in seine Belegung uebernehmen kann.
fn protokollieren(druck: Tastendruck, nachschlag: Nachschlag<'_>) {
    let kombination = match Kombination::aus_tastendruck(druck) {
        Some(kombination) => kombination.to_string(),
        None => "(kein Name in der Schreibweise)".to_owned(),
    };
    let funktion = match nachschlag {
        Nachschlag::Funktion(funktion) => funktion.kennung().to_owned(),
        Nachschlag::Sprungmarke => "(Sprungmarke)".to_owned(),
        Nachschlag::Unbelegt => "(unbelegt)".to_owned(),
    };
    println!(
        "tastencode={} maske={} kombination={kombination} funktion={funktion}",
        druck.code, druck.maske
    );
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::normalisierung::roh;

    use super::*;

    /// Die Gegenprobe zu den acht Bitwerten, die der Kern abgeschrieben fuehrt.
    ///
    /// `krk-core` darf `objc2-app-kit` nicht kennen; das ist die
    /// Architekturgrenze und bleibt so. Es fuehrt die Werte deshalb als nackte
    /// Zahlen, und bis hierher hat nichts sie mit ihrer Quelle verglichen: die
    /// Pruefungen in `krk-core` speisen dieselben Konstanten ein, die die
    /// Umsetzung liest, und bestaetigen sie damit gegen sich selbst. Stuende
    /// `BEFEHL` auf `1 << 21`, blieben sie gruen und KRK hielte den Zehnerblock
    /// fuer die Befehlstaste.
    ///
    /// `krk-ui` kennt beide Kisten und ist damit die eine Stelle, an der die
    /// Kopie gegen ihre Quelle zu halten ist, ohne die Grenze anzufassen und
    /// ohne eine zweite Wahrheit anzulegen. Diese Pruefung macht keinen
    /// Objective-C-Aufruf; sie liest zwei Konstanten.
    #[test]
    fn die_acht_rohen_bitwerte_des_kerns_stimmen_mit_appkit_ueberein() {
        let paare = [
            (
                "CapsLock",
                roh::FESTSTELLTASTE,
                NSEventModifierFlags::CapsLock,
            ),
            ("Shift", roh::UMSCHALT, NSEventModifierFlags::Shift),
            ("Control", roh::STEUERUNG, NSEventModifierFlags::Control),
            ("Option", roh::WAHL, NSEventModifierFlags::Option),
            ("Command", roh::BEFEHL, NSEventModifierFlags::Command),
            (
                "NumericPad",
                roh::ZEHNERBLOCK,
                NSEventModifierFlags::NumericPad,
            ),
            ("Help", roh::HILFE, NSEventModifierFlags::Help),
            ("Function", roh::FUNKTION, NSEventModifierFlags::Function),
        ];
        for (name, im_kern, in_appkit) in paare {
            assert_eq!(
                im_kern, in_appkit.0 as u64,
                "der Wert fuer {name} weicht von NSEventModifierFlags ab"
            );
        }
    }

    /// Der Weg, den `behandeln` geht, faengt bei dieser Umrechnung an.
    ///
    /// Ohne sie waere der Vergleich oben eine Behauptung ueber zwei Konstanten,
    /// die niemanden betrifft. `modifierFlags().0 as u64` ist die Stelle, an der
    /// die AppKit-Bits in den Kern laufen.
    ///
    /// Seit Schritt 11 prueft sie den ganzen Weg: die rohen Bits von AppKit
    /// gehen durch die Normalisierung in den Nachschlag der
    /// Auslieferungsbelegung, und heraus kommt das Kommando, das `behandeln` an
    /// die Datenquelle gibt.
    #[test]
    fn die_maske_eines_pfeils_kommt_leer_im_kern_an() {
        let wie_appkit_es_liefert =
            (NSEventModifierFlags::Function | NSEventModifierFlags::NumericPad).0 as u64;
        let druck = Tastendruck::aus_ereignis(CODE_PFEIL_AB, wie_appkit_es_liefert);

        assert!(druck.maske.ist_leer());
        let belegung = Belegung::auslieferung();
        let Nachschlag::Funktion(funktion) = belegung.nachschlag(druck) else {
            panic!("Pfeil ab ist in der Auslieferungsbelegung keiner Funktion zugeordnet");
        };
        assert_eq!(
            funktion.kommando(),
            Some(krk_core::tasten::Kommando::AuswahlRunter)
        );
    }
}
