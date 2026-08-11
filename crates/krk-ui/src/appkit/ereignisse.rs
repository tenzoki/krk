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
//! **Der Weg eines Tastendrucks**, vom Ereignis bis zur Ausfuehrung, in der
//! Reihenfolge, die [`behandeln`] faehrt:
//!
//! ```text
//! NSEvent
//!    │
//!    ├─ Tastendruck::aus_ereignis ..... Maske normalisiert, Zeichen gemeldet
//!    │
//!    ├─ Faenger der Belegungsansicht .. nimmt er auf: Ereignis verbraucht
//!    │
//!    ├─ Fokusvorbehalt
//!    │    ├─ Ersthelfer = Textflaeche des Editors? ──ja──> weiter zum Nachschlag
//!    │    └─ sonst Textfeld oder Feldeditor? ───────ja──> unveraendert an AppKit
//!    │
//!    └─ Belegung::nachschlag
//!         ├─ Kommando ─────> Senke des Aufrufers ─┐
//!         ├─ Sprungmarke ──> Zeichen ──> Senke ───┤ steht ein Blatt?
//!         └─ unbelegt ─────> unveraendert an AppKit
//! ```
//!
//! Die letzte Frage stellt die Senke und nicht dieser Abgriff; siehe den
//! Abschnitt "Der Fokusvorbehalt" unten.
//!
//! Die Normalisierung steht **vor** dem Vorbehalt, weil der Faenger den rohen
//! [`Tastendruck`] braucht und vor beidem sitzt; bis zum 260808 zeigte das Bild
//! hier die umgekehrte Reihenfolge und beschrieb damit einen Weg, den der Code
//! nie gegangen ist.
//!
//! Trifft der Nachschlag und fuehrt die Senke das Kommando aus, schluckt der
//! Abgriff das Ereignis (er liefert `nil`); sonst reicht er es unveraendert
//! weiter, damit Cmd+Q, Shift+Cmd+W und die Texteingabe des Systems ihren
//! gewohnten Weg gehen.
//!
//! # Der Faenger: die Aufnahme der Belegungsansicht (C3)
//!
//! Die Belegungsansicht weist eine Kombination zu, indem der Nutzer sie
//! drueckt. Waehrend dieser Aufnahme darf der Tastendruck weder nachgeschlagen
//! noch weitergereicht werden: die gedrueckte Kombination ist Eingabe und kein
//! Befehl, und gerade eine schon vergebene muss die Zuweisung erreichen, damit
//! der Konflikt gemeldet wird, statt die Funktion auszuloesen. Der **Faenger**
//! steht deshalb vor dem Fokusvorbehalt und dem Nachschlag: liefert er `true`,
//! hat er den rohen [`Tastendruck`] uebernommen, und das Ereignis ist
//! verbraucht. Ausserhalb der Aufnahme liefert er `false` und aendert nichts.
//! Das bleibt **ein** Abgriff mit einem zweiten Abnehmer und wird kein zweiter
//! Weg: keine Ansicht bekommt eine eigene `keyDown:`-Behandlung, auch die
//! Belegungsansicht nicht.
//!
//! # Der Fokusvorbehalt
//!
//! **Tastenbefehle wirken im Dateifenster; Textfelder behalten ihre
//! AppKit-Bedeutung.** Der Abgriff sieht jeden Tastendruck der Anwendung,
//! gleich wo der Eingabefokus steht. C2 verlangt fuer jedes Textfeld die
//! gewohnte Mac-Bedeutung: Return bestaetigt, Cmd+Links und Cmd+Rechts bewegen
//! die Schreibmarke an Zeilenanfang und Zeilenende. Seit S11c liegt der Auf-
//! und Abstieg genau auf diesen beiden Kombinationen, und ohne den Vorbehalt
//! waere die Pfadeingabe aus C2 damit nicht bedienbar: das Blatt stuende offen,
//! und Cmd+Links wechselte hinter ihm den Ordner.
//!
//! Der Abgriff fragt deshalb **vor** dem Nachschlag, ob der Ersthelfer des
//! Schluesselfensters ein Textfeld ist, und reicht den Tastendruck in diesem
//! Fall unveraendert weiter. Der Vorbehalt sitzt hier und nicht je Feld: jedes
//! Textfeld des Programms erbt ihn dadurch, ohne ihn zu wiederholen — das Feld
//! eines Blattes so gut wie der Feldeditor einer Umbenennung in der Liste.
//! Gemeldet war das als
//! `issues/260804-1122_*_der-fokusvorbehalt-fuer-tastenbefehle-steht-nur-fuer-die-loeschtasten.md`.
//!
//! **Ein stehendes Blatt haelt dieser Vorbehalt nicht an, und er ist auch nicht
//! die Stelle, die es tun soll.** Er fragt nach dem Ersthelfer, und ein Blatt
//! ohne Textfeld hat als Ersthelfer eine Schaltflaeche; sein Tastendruck laeuft
//! also in den Nachschlag und erreicht die Senke. Angehalten wird er dort, beim
//! Anwendungsdelegierten: `Anwendungsdelegierter::kommando_ausfuehren` weist
//! jedes Kommando ausser dem Abbruch ab, solange ein Blatt am Fenster haengt
//! (`kommandos::operationen::waehrend_blatt_erlaubt`), und
//! `Anwendungsdelegierter::eingabe_ausfuehren` haelt daneben das getippte
//! Zeichen an. Zwei Stellen mit zwei verschiedenen Fragen und keine doppelte
//! Regel: hier geht es darum, **wem die Taste gehoert**, dort darum, **welcher
//! Befehl jetzt zulaessig ist**. Wer nur diese Datei liest, haelt den Vorbehalt
//! sonst fuer die einzige Sperre und schliesst daraus auf einen Defekt, den es
//! nicht gibt — genau so entstand
//! `issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`.
//!
//! **Die eine Ausnahme ist die Textflaeche des Editors.** Sie ist selbst eine
//! `NSTextView` und fiele damit unter den Vorbehalt; der Editor haette mit dem
//! Fokus in sich selbst keinen einzigen Tastenbefehl von KRK. Der Vorbehalt
//! bekommt deshalb keine zweite Regel daneben, sondern eine Ausnahme mit einem
//! Namen: der Ersthelfer behaelt seine AppKit-Bedeutung, ausser er ist dasselbe
//! Objekt wie die Textflaeche des Editors.
//!
//! **Gefragt ist die Naemlichkeit und nicht die Art.** Eine Frage nach der Art
//! kann zwei Objekte derselben Art nicht trennen, und der Feldeditor eines
//! Textfeldes ist dieselbe Art wie die Textflaeche des Editors: beide sind
//! `NSTextView`. Der Vergleich laeuft deshalb ueber die Objektgleichheit der
//! Objective-C-Zeiger und nicht ueber einen Klassennamen, ein Kennzeichen an
//! der Ansicht oder einen Gang durch den Ansichtsbaum. Er ist trennscharf, weil
//! ein Objekt mit genau einem anderen identisch ist, und vollstaendig, weil die
//! Frage fuer jeden Ersthelfer eine Antwort hat; eine Liste von Ausnahmen
//! entsteht nirgends. Die Pruefung auf die drei Textklassen bleibt daneben
//! unveraendert stehen und behaelt ihren Grund.
//!
//! **Der Abgriff kennt den Editor nicht.** Die Naemlichkeitsfrage kommt als
//! Abschluss von aussen, in derselben Form wie `faenger` und `senke` und von
//! derselben Stelle, dem Anwendungsdelegierten, der die Textflaeche ohnehin
//! haelt. Dieses Modul haelt sie nicht; es kennt allein die Frage, die jemand
//! anders beantwortet. Solange kein Editor gebaut ist, antwortet der Abschluss
//! immer mit `false`, und der Vorbehalt wirkt wie zuvor.
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
//! # Zwei Zeichen aus demselben Ereignis, und sie beantworten zwei Fragen
//!
//! Seit dem 260809 liest der Abgriff aus jedem Ereignis **zwei** Zeichen, und
//! sie sind nicht dasselbe:
//!
//! | Frage | Woher | Wofuer |
//! |---|---|---|
//! | Welche **Taste** wurde gedrueckt? | `charactersByApplyingModifiers:` mit leerer Maske, siehe [`gemeldetes_zeichen`] | der Nachschlag |
//! | Welches **Zeichen** hat der Nutzer getippt? | `characters`, siehe [`getipptes_zeichen`] | die Sprungmarke aus C2 |
//!
//! Die erste Frage ist neu. Bis dahin schlug der Abgriff auch Buchstaben ueber
//! den virtuellen Tastencode nach, und ein Tastencode benennt die **Stelle** auf
//! der Tastatur: auf einer deutschen Tastatur lag `cmd+y` unter der Aufschrift
//! Z, und `cmd+z` aus dem Hauptmenue stiess mit ihm zusammen. Buchstaben und
//! Ziffern gehen seither ueber das gemeldete Zeichen, die Funktionstasten
//! weiter ueber den Code. Die Regel und ihre Begruendung stehen im Kern
//! (`krk_core::tasten::parser`, `Tastenkennung`); hier steht nur, woher das
//! Zeichen kommt. Nutzerentscheid vom 260808-0155, `decisions/
//! 260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`.
//!
//! Die zweite Frage bleibt, wie sie war. Eine Taste ohne Zusatztaste, die keiner
//! Funktion gehoert, faellt im Kern auf [`Nachschlag::Sprungmarke`]; welches
//! Zeichen in den Suchpuffer geht, weiss das Ereignis, und die Regel, welche
//! Zeichen ein Dateiname tragen kann, steht in
//! `krk_core::verzeichnis::sprungmarke`. Getippt wird, was auf dem Bildschirm
//! stuende, samt Grossschreibung; nachgeschlagen wird die Taste. Ein
//! gemeinsames Zeichen fuer beides waere fuer eine der beiden Fragen die
//! falsche Antwort.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSEvent`, `NSApplication`, `NSWindow`, `NSResponder`, `NSText`,
//! `NSTextField` und `NSTextView` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `NSString` und `NSProcessInfo`. Vier Beruehrungen sind juenger als ihre
//! Klasse: `addLocalMonitorForEventsMatchingMask:handler:`, `removeMonitor:`
//! und `NSProcessInfo.systemUptime` seit 10.6,
//! `charactersByApplyingModifiers:` seit 10.15. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Der Rueckfall auf `charactersIgnoringModifiers` in
//! [`gemeldetes_zeichen`] ist keine Verfuegbarkeitspruefung.** 10.15 liegt
//! unter der Untergrenze, und `charactersByApplyingModifiers:` antwortet auf
//! jedem Zielsystem; der Rueckfall gilt den selbst gebauten Ereignissen aus
//! [`ereignis_senden`], und der Grund dafuer steht dort.

use std::ptr::NonNull;

use block2::RcBlock;
use objc2::ClassType;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSApplication, NSEvent, NSEventMask, NSEventModifierFlags, NSEventType, NSResponder, NSText,
    NSTextField, NSTextView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSObjectProtocol, NSPoint, NSProcessInfo, NSString};

use krk_core::tasten::Belegung;
use krk_core::tasten::normalisierung::{ModMaske, roh};
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
    /// `faenger` sieht jeden Tastendruck **vor** dem Nachschlag: die Aufnahme
    /// der Belegungsansicht aus C3, siehe den Modulkopf. Liefert er `true`,
    /// ist das Ereignis verbraucht.
    ///
    /// `ist_editorflaeche` beantwortet die eine Naemlichkeitsfrage des
    /// Fokusvorbehalts: ist dieser Ersthelfer dasselbe Objekt wie die
    /// Textflaeche des Editors? Liefert er `true`, behaelt der Ersthelfer seine
    /// AppKit-Bedeutung **nicht**, und der Tastendruck laeuft in den
    /// Nachschlag. Der Abschluss kommt von aussen, weil dieses Modul den Editor
    /// nicht kennt; siehe den Modulkopf.
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
        faenger: impl Fn(Tastendruck) -> bool + 'static,
        ist_editorflaeche: impl Fn(&NSResponder) -> bool + 'static,
        senke: impl Fn(Eingabe) -> bool + 'static,
    ) -> Option<Self> {
        let block = RcBlock::new(move |ereignis: NonNull<NSEvent>| -> *mut NSEvent {
            // SAFETY: AppKit reicht dem Block einen gueltigen Zeiger auf das
            // Ereignis, das fuer die Dauer des Aufrufs lebt.
            let geschluckt = behandeln(
                mtm,
                &faenger,
                &ist_editorflaeche,
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
    ereignis_senden(
        mtm,
        fenster,
        CODE_PFEIL_AB,
        NSEventModifierFlags::Function | NSEventModifierFlags::NumericPad,
        &ZEICHEN_PFEIL_AB.to_string(),
    );
}

/// Stellt die erste Kombination der genannten Funktion als synthetisches
/// Tastenereignis in die eigene Ereignisschlange (S21).
///
/// Der Weg ist derselbe wie bei [`pfeil_ab_senden`], die Kombination kommt
/// aber aus der **Belegung** statt aus einer festen Zahl: die Sitzungsstrecke
/// misst Funktionen, und welcher Tastendruck eine Funktion ausloest, weiss
/// allein die Belegung. Damit misst der Lauf auch unter einer umbelegten
/// `keymap.toml` die richtige Funktion — oder bricht ab, wenn sie keine
/// Kombination mehr traegt, statt eine falsche Taste zu druecken.
///
/// Die Zusatztastenmaske traegt genau die vier benannten Zusatztasten der
/// Kombination. Die Marken `function` und `numericPad`, die AppKit einem
/// koerperlichen Druck auf Pfeil- und Funktionstasten beilegt, fehlen mit
/// Absicht: die Normalisierung aus S7 streift sie vor dem Nachschlag ohnehin
/// ab, und das Ereignis wird vom eigenen Abgriff geschluckt, bevor ein
/// anderer Abnehmer sie saehe.
///
/// **Das Zeichen kommt mit, wo die Kombination eines traegt.** Buchstaben und
/// Ziffern werden ueber das gemeldete Zeichen nachgeschlagen; ein selbst
/// gebautes Ereignis ohne Zeichen faende die Funktion nicht mehr, sobald der
/// Nutzer eine gemessene Funktion auf einen Buchstaben umbelegt. Die
/// ausgelieferte Belegung fuehrt die beiden gemessenen Funktionen heute auf
/// `f5` und `down`; die Zeile haengt nicht daran.
pub fn funktion_senden(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    belegung: &Belegung,
    kennung: &str,
) -> Result<(), String> {
    let funktion = belegung
        .funktion(kennung)
        .ok_or_else(|| format!("die Belegung kennt keine Funktion {kennung:?}"))?;
    let kombination = funktion.tasten().first().copied().ok_or_else(|| {
        format!(
            "die Funktion {kennung:?} traegt keine Kombination; die Sitzungsstrecke \
             braucht eine Taste, die sie ausloest"
        )
    })?;
    let druck = kombination.tastendruck();
    ereignis_senden(
        mtm,
        fenster,
        druck.code,
        rohe_flaggen(druck.maske),
        &zeichen_des_ereignisses(kombination),
    );
    Ok(())
}

/// Die Zeichenkette, die ein selbst gebautes Ereignis fuer diese Kombination
/// traegt.
///
/// Leer fuer jede Taste, die ueber ihre Stelle nachgeschlagen wird: eine
/// Funktionstaste meldet ein Zeichen aus dem privaten Bereich von Unicode, und
/// es in das Ereignis zu schreiben hiesse, eine Angabe zu erfinden, die
/// niemand liest. Fuer eine Buchstaben- oder Zifferntaste steht hier genau das
/// Zeichen, unter dem der Nachschlag sie sucht.
fn zeichen_des_ereignisses(kombination: Kombination) -> String {
    kombination
        .taste()
        .zeichen()
        .map(String::from)
        .unwrap_or_default()
}

/// Die AppKit-Zusatztastenmaske zu einer normalisierten Maske des Kerns.
///
/// Die Umrechnung laeuft ueber die acht rohen Bitwerte, deren Gleichstand mit
/// `NSEventModifierFlags` die Pruefung unten haelt; eine zweite Wahrheit ueber
/// dieselben Bits entsteht nicht.
fn rohe_flaggen(maske: ModMaske) -> NSEventModifierFlags {
    let mut bits: u64 = 0;
    for (teil, bit) in [
        (ModMaske::BEFEHL, roh::BEFEHL),
        (ModMaske::STEUERUNG, roh::STEUERUNG),
        (ModMaske::WAHL, roh::WAHL),
        (ModMaske::UMSCHALT, roh::UMSCHALT),
    ] {
        if maske.enthaelt(teil) {
            bits |= bit;
        }
    }
    NSEventModifierFlags(bits as usize)
}

/// Baut ein Tastenereignis und stellt es hinten in die Ereignisschlange.
fn ereignis_senden(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    code: u16,
    flaggen: NSEventModifierFlags,
    zeichen: &str,
) {
    let zeichen = NSString::from_str(zeichen);
    let ereignis = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
        NSEventType::KeyDown,
        NSPoint::ZERO,
        flaggen,
        NSProcessInfo::processInfo().systemUptime(),
        fenster.windowNumber(),
        None,
        &zeichen,
        &zeichen,
        false,
        code,
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
    faenger: &impl Fn(Tastendruck) -> bool,
    ist_editorflaeche: &impl Fn(&NSResponder) -> bool,
    senke: &impl Fn(Eingabe) -> bool,
    belegung: &Belegung,
    ereignis: &NSEvent,
    protokoll: bool,
) -> bool {
    let druck = Tastendruck::aus_ereignis(
        ereignis.keyCode(),
        gemeldetes_zeichen(ereignis),
        ereignis.modifierFlags().0 as u64,
    );

    // Die Aufnahme der Belegungsansicht, vor allem anderen. Siehe den
    // Modulkopf: waehrend der Aufnahme ist der Tastendruck Eingabe und kein
    // Befehl, und auch der Fokusvorbehalt hat hier nichts zu sagen.
    if faenger(druck) {
        return true;
    }

    // Der Fokusvorbehalt, vor dem Nachschlag. Siehe den Modulkopf: steht die
    // Schreibmarke in einem Textfeld, behaelt jede Taste ihre AppKit-Bedeutung;
    // steht sie in der Textflaeche des Editors, nicht.
    if ersthelfer_gehoert_appkit(mtm, ist_editorflaeche) {
        return false;
    }

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

/// Ob der Ersthelfer des Schluesselfensters seine AppKit-Bedeutung behaelt.
///
/// Gefragt ist das **Schluesselfenster** und nicht das Hauptfenster: steht ein
/// Blatt am Fenster, ist dessen Panel das Schluesselfenster, und dort sitzt das
/// Textfeld der Pfadeingabe.
///
/// **Zuerst die Naemlichkeit, dann die Art.** Ist der Ersthelfer dasselbe
/// Objekt wie die Textflaeche des Editors, gehoert er nicht AppKit, und der
/// Tastendruck laeuft in den Nachschlag. Diese Frage steht vor der
/// Klassenpruefung, weil sie ihr sonst zum Opfer fiele: die Textflaeche des
/// Editors ist eine `NSTextView` wie der Feldeditor auch, und eine Frage nach
/// der Art kann die beiden nicht trennen. Siehe den Modulkopf.
///
/// Sonst gilt die Pruefung auf die drei Textklassen unveraendert. Ein
/// `NSTextField` gibt beim Bearbeiten seinen Ersthelferrang an den Feldeditor
/// ab, einen gemeinsam genutzten `NSTextView`. Gefragt sind deshalb beide
/// Klassen: das Feld selbst, solange es nur ausgewaehlt ist, und der
/// Feldeditor, sobald die Schreibmarke darin steht. `NSText` deckt daneben die
/// aelteren Textklassen ab, die AppKit weiterhin fuehrt.
fn ersthelfer_gehoert_appkit(
    mtm: MainThreadMarker,
    ist_editorflaeche: &impl Fn(&NSResponder) -> bool,
) -> bool {
    let Some(fenster) = NSApplication::sharedApplication(mtm).keyWindow() else {
        return false;
    };
    let Some(ersthelfer) = fenster.firstResponder() else {
        return false;
    };
    if ist_editorflaeche(&ersthelfer) {
        return false;
    }
    ersthelfer.isKindOfClass(NSTextView::class())
        || ersthelfer.isKindOfClass(NSTextField::class())
        || ersthelfer.isKindOfClass(NSText::class())
}

/// Das Zeichen, das die gedrueckte Taste **ohne Zusatztasten** meldet.
///
/// Die eine Stelle, an der die Tastaturbelegung des Geraets in den Nachschlag
/// eingeht. `charactersByApplyingModifiers:` beantwortet dieselbe Frage, die
/// `NSMenuItem.keyEquivalent` fuer das Hauptmenue schon beantwortet: welches
/// Zeichen steht auf dieser Taste? Mit einer leeren Maske gefragt, faellt
/// dabei auch die Umschalttaste weg, und `shift+cmd+1` meldet die `1` und nicht
/// das Ausrufezeichen, das darueber steht.
///
/// **Genommen wird das erste Zeichen.** Eine Taste liefert in aller Regel genau
/// eines; eine Folge aus mehreren stammt von einer Eingabemethode und ist kein
/// Tastenbefehl. Was davon als Kennung taugt, entscheidet danach
/// `krk_core::tasten::parser::zeichen_als_kennung`; hier wird nichts gefiltert
/// und nichts umgeschrieben.
///
/// **Der zweite Weg ist fuer die selbst gebauten Ereignisse.** AppKit
/// beantwortet `charactersByApplyingModifiers:` aus der Tastaturbelegung; fuer
/// ein Ereignis aus [`ereignis_senden`], das KRK selbst zusammensetzt, steht
/// die Antwort in den Zeichenketten, die es mitbekommen hat. Es ist dieselbe
/// Frage an dieselbe Taste, nicht ein zweites Verfahren:
/// `charactersIgnoringModifiers` ist die aeltere, groebere Form derselben
/// Auskunft.
fn gemeldetes_zeichen(ereignis: &NSEvent) -> Option<char> {
    erstes_zeichen(ereignis.charactersByApplyingModifiers(NSEventModifierFlags::empty()))
        .or_else(|| erstes_zeichen(ereignis.charactersIgnoringModifiers()))
}

/// Das erste Zeichen einer AppKit-Zeichenkette, falls sie eines traegt.
fn erstes_zeichen(text: Option<Retained<NSString>>) -> Option<char> {
    text?.to_string().chars().next()
}

/// Das Zeichen, das dieses Ereignis traegt.
///
/// Fuer die Sprungmarke aus C2, und deshalb ueber `characters` und nicht ueber
/// [`gemeldetes_zeichen`]: getippt wird, was auf dem Bildschirm stuende, samt
/// Grossschreibung. Der Nachschlag fragt die andere Frage, naemlich welche
/// Taste gedrueckt wurde.
///
/// `None` fuer ein Ereignis ohne Zeichen, etwa eine reine Zusatztaste. Genommen
/// wird das **erste** Zeichen: eine Taste liefert in aller Regel genau eines,
/// und eine Folge aus mehreren stammt von einer Eingabemethode, deren Ergebnis
/// nicht in einen Suchpuffer gehoert.
fn getipptes_zeichen(ereignis: &NSEvent) -> Option<char> {
    erstes_zeichen(ereignis.characters())
}

/// Schreibt eine Zeile des Modus `--tasten-protokoll`.
///
/// Auf die Standardausgabe, wie der Plan es vorschreibt. Sichtbar ist sie nur,
/// wenn KRK aus einem Terminal gestartet wurde: ein ueber `open` gestartetes
/// Buendel bekommt von LaunchServices keine.
///
/// Die Zeile nennt den Tastencode, weil die Abnahme von Schritt 7 daran haengt,
/// daneben das gemeldete Zeichen, weil der Nachschlag fuer Buchstaben und
/// Ziffern daran haengt, und die Kombination in der Schreibweise von
/// `keymap.toml`, damit der Nutzer sie von hier in seine Belegung uebernehmen
/// kann. Auf einer deutschen Tastatur laufen die ersten beiden auseinander, und
/// genau das soll die Zeile zeigen: die Taste mit der Aufschrift Y meldet
/// `tastencode=6` und `zeichen=y`.
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
    let zeichen = match druck.zeichen {
        Some(zeichen) => zeichen.to_string(),
        None => "(keins)".to_owned(),
    };
    println!(
        "tastencode={} zeichen={zeichen} maske={} kombination={kombination} funktion={funktion}",
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
        let druck = Tastendruck::aus_ereignis(CODE_PFEIL_AB, None, wie_appkit_es_liefert);

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

    /// Ein selbst gesendetes Ereignis findet seine Funktion wieder, auch wenn
    /// sie auf einem Buchstaben liegt.
    ///
    /// Der Rundlauf ohne AppKit: die Kombination geht ueber
    /// [`zeichen_des_ereignisses`] und [`rohe_flaggen`] in die Angaben, die
    /// [`ereignis_senden`] in das Ereignis schreibt, und von dort so zurueck in
    /// den Kern, wie [`behandeln`] sie wieder herausliest. Am Ende steht
    /// dieselbe Funktion.
    ///
    /// **Was hier nicht gemessen ist.** Ob `charactersByApplyingModifiers:` an
    /// einem selbst gebauten Ereignis antwortet, sagt diese Pruefung nicht: ein
    /// `NSEvent` laesst sich in ihr nicht bauen, weil AppKit dafuer den
    /// Hauptfaden und eine laufende Ereignisschleife braucht, und der Versuch
    /// haelt den Testlauf an. Gemessen ist der zweite Weg aus
    /// [`gemeldetes_zeichen`], `charactersIgnoringModifiers`, und der liest
    /// genau die Zeichenkette zurueck, die das Ereignis mitbekommen hat. Der
    /// erste Weg kann daneben nur dasselbe liefern oder nichts.
    ///
    /// Die Kombination ist gesucht und nicht hingeschrieben: die Messstrecke
    /// misst heute `f5` und `down`, und die Zusage handelt nicht davon.
    #[test]
    fn ein_gesendetes_zeichen_findet_seine_funktion_wieder() {
        let belegung = Belegung::auslieferung();
        let mut geprueft = 0usize;

        for funktion in belegung.funktionen() {
            if funktion.gehalten_von().is_some() {
                continue;
            }
            for kombination in funktion.tasten() {
                if kombination.taste().zeichen().is_none() {
                    continue;
                }
                let text = zeichen_des_ereignisses(*kombination);
                let druck = Tastendruck::aus_ereignis(
                    kombination.taste().code,
                    text.chars().next(),
                    rohe_flaggen(kombination.maske()).0 as u64,
                );
                let Nachschlag::Funktion(getroffen) = belegung.nachschlag(druck) else {
                    panic!("{kombination} findet als gesendetes Ereignis keine Funktion");
                };
                assert_eq!(
                    getroffen.kennung(),
                    funktion.kennung(),
                    "{kombination} findet als gesendetes Ereignis eine andere Funktion"
                );
                geprueft += 1;
            }
        }

        assert!(
            geprueft > 0,
            "die Auslieferungsbelegung fuehrt keine Kombination auf einem Buchstaben"
        );
    }
}
