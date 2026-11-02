//! Der eine Fadenweg, auf dem eine Frage an die Platte vom Hauptfaden
//! genommen wird und ihre Antwort ueber die Hauptschlange zurueckkommt.
//!
//! ```text
//! Hauptfaden ──> fragen_lassen ──> benannter Faden: arbeit()
//!                                          │
//!   fertig(mtm, ergebnis) <── DispatchQueue::main() <──┘
//! ```
//!
//! # Woher der Weg kommt
//!
//! Bis zum 261102 stand er privat in [`super::terminal`] und hiess dort
//! `auf_dem_faden`; `ctrl+o`, „Im Finder oeffnen" und „Im Terminal
//! bearbeiten" gingen ueber ihn, alle drei auf dem Faden `krk-terminal`.
//! Gehoben ist er, weil weitere Menue- und Tastenwege dieselbe Frage vom
//! Hauptfaden nehmen sollen (Plan
//! `261102-1314-plan-dateisystemfragen-vom-hauptfaden-nehmen.md`, Schritt 1),
//! und ein zweiter Mechanismus daneben waere der Doppelbau, den der Zuschnitt
//! von [`super::terminal`] schon einmal vermieden hat. **Neu ist allein, dass
//! der Rufer den Fadennamen angibt**; Halter, Rueckweg und das genau einmalige
//! `fertig` sind dieselben wie vorher.
//!
//! # Was der Rufer zusagt und was dieser Weg zusagt
//!
//! `arbeit` laeuft auf dem neuen Faden und bekommt ihre Eingaben als
//! gewoehnliche Rust-Werte mit, denn ein `Retained` geht nicht ueber die
//! Fadengrenze. `fertig` bekommt auf dem Hauptfaden den [`MainThreadMarker`],
//! damit der Rufer seinen Delegierten dort sucht, wo er steht. **`fertig` wird
//! in jedem Fall genau einmal gerufen**, auch wenn schon der Fadenstart
//! scheitert, und dann mit dem Ergebnis, das `startfehler` aus dem Fehler
//! macht; ausgenommen ist allein eine Hauptschlange, die nie wieder laeuft.
//! Das Muster ist das von [`super::auswurf`].
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Diese Datei spricht keine Objective-C-Klasse an. `MainThreadMarker` aus
//! `objc2_foundation` ist ein Rust-Typ der Kiste und hat kein macOS-Alter;
//! `DispatchQueue` aus `dispatch2` ist eine Rust-Huelle um `libdispatch`, seit
//! macOS 10.6 im System. Das Buendel zielt auf 15.0 (`.cargo/config.toml`);
//! keines von beiden hebt die Untergrenze an, und keine Beruehrung in dieser
//! Datei braucht eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt
//! keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::sync::{Arc, Mutex};

use dispatch2::DispatchQueue;
use objc2_foundation::MainThreadMarker;

/// `arbeit` laeuft auf einem eigenen Faden namens `fadenname`, ihr Ergebnis
/// geht ueber `DispatchQueue::main()` an `fertig`.
///
/// Scheitert der Fadenstart, macht `startfehler` aus dem Fehler das Ergebnis,
/// und `fertig` bekommt es ebenso ueber die Hauptschlange. **Auf dem rufenden
/// Faden laeuft nichts von `arbeit`**; dort stehen allein der Fadenstart und,
/// scheitert er, `startfehler`.
pub(crate) fn fragen_lassen<T: Send + 'static>(
    fadenname: &'static str,
    arbeit: impl FnOnce() -> T + Send + 'static,
    startfehler: impl FnOnce(std::io::Error) -> T,
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
) {
    // `Builder::spawn` nimmt den Abschluss und gibt ihn bei einem Fehlschlag
    // nicht zurueck; der Halter laesst `fertig` deshalb auch dann erreichbar,
    // wenn kein Faden entsteht.
    let halter = Arc::new(Mutex::new(Some(fertig)));
    let auf_dem_faden = Arc::clone(&halter);
    let gestartet = std::thread::Builder::new()
        .name(fadenname.to_owned())
        .spawn(move || {
            let ergebnis = arbeit();
            if let Some(fertig) = nehmen(&auf_dem_faden) {
                auf_dem_hauptfaden(fertig, ergebnis);
            }
        });
    if let Err(fehler) = gestartet
        && let Some(fertig) = nehmen(&halter)
    {
        auf_dem_hauptfaden(fertig, startfehler(fehler));
    }
}

/// Nimmt `fertig` aus dem Halter, auch aus einem vergifteten: der Wert darin
/// ist trotzdem der richtige, und ihn liegen zu lassen hiesse, die Antwort zu
/// verschlucken (dieselbe Ueberlegung wie in [`super::auswurf`]).
fn nehmen<F>(halter: &Mutex<Option<F>>) -> Option<F> {
    match halter.lock() {
        Ok(mut fertig) => fertig.take(),
        Err(vergiftet) => vergiftet.into_inner().take(),
    }
}

/// Reicht das Ergebnis an den Hauptfaden weiter.
fn auf_dem_hauptfaden<T: Send + 'static>(
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
    ergebnis: T,
) {
    DispatchQueue::main().exec_async(move || {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Was das Ergebnis haelt, bliebe hier ungenutzt; beim
            // Skript aus `super::terminal` ist das eine Datei mit `0700` im
            // privaten `$TMPDIR`, und das ist der Rest, den E2 des Plans
            // `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`
            // hinnimmt.
            return;
        };
        fertig(mtm, ergebnis);
    });
}
