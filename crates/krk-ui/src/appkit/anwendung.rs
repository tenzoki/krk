//! Der Start: `NSApplication`, der Anwendungsdelegierte, das erste Fenster.
//!
//! KRK laeuft als gewoehnliche Anwendung im Vordergrund
//! (`NSApplicationActivationPolicy::Regular`), auch wenn `cargo run` sie ohne
//! Buendel startet. Fuer die Abnahme zaehlt trotzdem allein der Start ueber
//! `target/KRK.app`: nur ein signiertes Buendel loest die Rueckfragen von TCC
//! aus, und ein nacktes Binaerprogramm erbt stattdessen die Freigaben des
//! Terminals.
//!
//! # Der Messmodus haengt an derselben Stelle wie der Tastenabgriff
//!
//! Ist `--messmodus` gesetzt, richtet [`Anwendungsdelegierter::oberflaeche_aufbauen`]
//! nach dem Tastenabgriff zwei weitere Dinge ein: den Bildtakt aus
//! [`super::bildtakt`], der jede Bildgrenze meldet, und einen Ausloesetakt, der
//! den naechsten Messschritt anstoesst. Beide reichen ausschliesslich
//! gewoehnliche Rust-Werte an [`crate::messmodus`] weiter — die Zeitpunkte der
//! Bildgrenzen und drei Zahlen ueber den Zustand der Liste.
//!
//! ```text
//!  Ausloesetakt (97 ms) ──> messmodus::naechster_schritt ──> Anweisung
//!                                                             │
//!            ordner_lesen / pfeil_ab_senden  <────────────────┘
//!
//!  Bildtakt (CADisplayLink) ──> messmodus::bildgrenze(Zeitpunkt, Zustand)
//! ```

use std::cell::OnceCell;
use std::path::PathBuf;
use std::rc::Rc;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes,
    NSTimer,
};

use crate::messmodus::{Anweisung, Aufgabe, Messlauf, Zustand};

use super::bildtakt::{self, Zeichenende};
use super::ereignisse::{self, Tastenabgriff};
use super::fenster::{self, FensterDelegierter};
use super::menue;
use super::tabelle::Dateifenster;

/// Der Rueckgabewert, mit dem ein Messlauf ohne Bildschirm endet.
const OHNE_BILDSCHIRM: i32 = 3;

/// Was der Anwendungsdelegierte haelt.
///
/// Die Zellen tragen Objekte, die AppKit nur schwach referenziert oder gar
/// nicht kennt. Faellt eines von ihnen, faellt das Fenster mit; faellt der
/// Tastenabgriff, meldet er sich bei AppKit ab, und faellt der Bildtakt, gibt
/// er den `CADisplayLink` frei.
pub struct AnwendungsIvars {
    /// Ob der Protokollmodus `--tasten-protokoll` laeuft.
    tasten_protokoll: bool,
    /// Die Aufgabe des Messmodus, falls einer laeuft.
    messaufgabe: Option<Aufgabe>,
    fenster: OnceCell<Retained<NSWindow>>,
    fenster_delegierter: OnceCell<Retained<FensterDelegierter>>,
    dateifenster: OnceCell<Dateifenster>,
    tastenabgriff: OnceCell<Tastenabgriff>,
    /// Der Ablauf der Messung. Der Bildtakt haelt eine zweite Referenz.
    messlauf: OnceCell<Rc<std::cell::RefCell<Messlauf>>>,
    zeichenende: OnceCell<Zeichenende>,
    /// Der Zeitgeber, der den naechsten Messschritt anstoesst.
    ausloesetakt: OnceCell<Retained<NSTimer>>,
}

define_class!(
    /// Der Anwendungsdelegierte.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = AnwendungsIvars]
    pub struct Anwendungsdelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Anwendungsdelegierter {}

    impl Anwendungsdelegierter {
        /// Der Rueckruf des Ausloesetakts.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(messSchritt:))]
        fn mess_schritt(&self, _zeitgeber: &NSTimer) {
            self.messen_weiter();
        }
    }

    // SAFETY: `NSApplicationDelegate` stellt keine Bedingungen.
    unsafe impl NSApplicationDelegate for Anwendungsdelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn start_abgeschlossen(&self, _meldung: &NSNotification) {
            self.oberflaeche_aufbauen();
        }
    }
);

impl Anwendungsdelegierter {
    /// Einen Anwendungsdelegierten ohne Oberflaeche.
    fn neu(
        mtm: MainThreadMarker,
        tasten_protokoll: bool,
        messaufgabe: Option<Aufgabe>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AnwendungsIvars {
            tasten_protokoll,
            messaufgabe,
            fenster: OnceCell::new(),
            fenster_delegierter: OnceCell::new(),
            dateifenster: OnceCell::new(),
            tastenabgriff: OnceCell::new(),
            messlauf: OnceCell::new(),
            zeichenende: OnceCell::new(),
            ausloesetakt: OnceCell::new(),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Baut Fenster und Dateifenster und liest den Startordner.
    fn oberflaeche_aufbauen(&self) {
        let mtm = self.mtm();

        let dateifenster = Dateifenster::bauen(mtm);
        let fenster_delegierter = FensterDelegierter::neu(mtm, dateifenster.quelle().retain());
        let fenster = fenster::hauptfenster(mtm, dateifenster.sicht(), &fenster_delegierter);

        // Erst festhalten, dann anzeigen: das Fenster haelt seinen Delegierten
        // schwach, die Tabelle haelt Datenquelle und Delegierten schwach.
        let ivars = self.ivars();
        let _ = ivars.dateifenster.set(dateifenster);
        let _ = ivars.fenster_delegierter.set(fenster_delegierter);
        let _ = ivars.fenster.set(fenster);

        if let Some(dateifenster) = ivars.dateifenster.get() {
            let abgriff =
                Tastenabgriff::einrichten(dateifenster.quelle().retain(), ivars.tasten_protokoll);
            match abgriff {
                Some(abgriff) => {
                    let _ = ivars.tastenabgriff.set(abgriff);
                }
                // Ohne Abgriff bewegt keine Taste mehr die Auswahl. Das still
                // hinzunehmen hiesse, eine Anwendung auszuliefern, deren erste
                // Maxime die Tastatursteuerung ist und die keine hat.
                None => eprintln!(
                    "krk: der Tastenabgriff liess sich nicht einrichten, die Tastatursteuerung bleibt aus"
                ),
            }
            dateifenster.quelle().ordner_lesen(&startordner(ivars));
        }
        if let Some(fenster) = ivars.fenster.get() {
            fenster.makeKeyAndOrderFront(None);
        }
        self.messmodus_einrichten();
    }

    /// Haengt Bildtakt und Ausloesetakt ein, wenn ein Messlauf ansteht.
    fn messmodus_einrichten(&self) {
        let ivars = self.ivars();
        let Some(aufgabe) = ivars.messaufgabe.clone() else {
            return;
        };
        let (Some(fenster), Some(dateifenster)) = (ivars.fenster.get(), ivars.dateifenster.get())
        else {
            return;
        };

        // Die Rate zuerst, und ohne sie kein Messlauf. Die Regel steht in S21
        // des Plans ausgeschrieben: ein Fenster auf keinem Bildschirm heisst
        // Abbruch, nicht Ausweichen auf den Hauptbildschirm.
        let Some(hertz) = bildtakt::bildwiederholrate(fenster) else {
            eprintln!("krk: {}", crate::messmodus::OHNE_BILDSCHIRM);
            std::process::exit(OHNE_BILDSCHIRM);
        };

        let mut lauf = Messlauf::neu(aufgabe);
        lauf.rate_setzen(hertz);
        let lauf = Rc::new(std::cell::RefCell::new(lauf));
        let _ = ivars.messlauf.set(Rc::clone(&lauf));

        let quelle = dateifenster.quelle().retain();
        let takt = Zeichenende::einrichten(self.mtm(), dateifenster.sicht(), move |jetzt| {
            let zustand = Zustand {
                zeilen: quelle.zeilen(),
                liest: quelle.liest_noch(),
                auswahl: quelle.auswahlzeile(),
            };
            if lauf.borrow_mut().bildgrenze(jetzt, zustand) {
                std::process::exit(0);
            }
        });
        let _ = ivars.zeichenende.set(takt);

        // SAFETY: `self` ist das Ziel und beantwortet `messSchritt:` mit der
        // erwarteten Signatur. Der Zeitgeber wird unten in die Laufschleife
        // gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol von Foundation.
        let zeitgeber = unsafe {
            let zeitgeber = NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                crate::messmodus::AUSLOESETAKT,
                self,
                sel!(messSchritt:),
                None,
                true,
            );
            NSRunLoop::currentRunLoop().addTimer_forMode(&zeitgeber, NSRunLoopCommonModes);
            zeitgeber
        };
        let _ = ivars.ausloesetakt.set(zeitgeber);
    }

    /// Ein Takt des Ausloesers: den naechsten Messschritt holen und ausfuehren.
    fn messen_weiter(&self) {
        let ivars = self.ivars();
        let (Some(lauf), Some(dateifenster), Some(fenster)) = (
            ivars.messlauf.get(),
            ivars.dateifenster.get(),
            ivars.fenster.get(),
        ) else {
            return;
        };
        let quelle = dateifenster.quelle();
        let zustand = Zustand {
            zeilen: quelle.zeilen(),
            liest: quelle.liest_noch(),
            auswahl: quelle.auswahlzeile(),
        };

        // Die Ausleihe endet vor dem AppKit-Aufruf: der Bildtakt greift auf
        // denselben `RefCell` zu, und ein Zeichendurchgang mitten in einer
        // gehaltenen Ausleihe waere der doppelte Zugriff.
        let anweisung = lauf.borrow_mut().naechster_schritt(zustand);
        match anweisung {
            Anweisung::Warten => {}
            Anweisung::Lesen(pfad) => quelle.ordner_lesen(&pfad),
            Anweisung::Taste => ereignisse::pfeil_ab_senden(self.mtm(), fenster),
            Anweisung::Fertig => {
                lauf.borrow().ausgeben();
                std::process::exit(0);
            }
            Anweisung::Abbruch(grund) => {
                eprintln!("krk: {grund}. Es wird keine Zahl ausgegeben.");
                std::process::exit(4);
            }
        }
    }
}

/// Startet die Anwendung. Kehrt zurueck, wenn sie beendet ist.
///
/// `tasten_protokoll` schaltet den Modus `--tasten-protokoll` aus der
/// Befehlszeile durch bis zum Ereignisabgriff, `messaufgabe` den Modus
/// `--messmodus` bis zum Aufbau der Oberflaeche.
pub fn starten(tasten_protokoll: bool, messaufgabe: Option<Aufgabe>) {
    let mtm = MainThreadMarker::new()
        .expect("die Oberflaeche von KRK laeuft ausschliesslich auf dem Hauptfaden");

    let anwendung = NSApplication::sharedApplication(mtm);
    anwendung.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    anwendung.setMainMenu(Some(&menue::hauptmenue(mtm)));

    // Der Delegierte bleibt bis zum Ende von `starten` am Leben, weil
    // `NSApplication` ihn nur schwach haelt.
    let delegierter = Anwendungsdelegierter::neu(mtm, tasten_protokoll, messaufgabe);
    anwendung.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));

    anwendung.run();
}

/// Der Ordner, den KRK beim Start zeigt.
///
/// Im Messmodus ist es der Pruefordner, sonst das Benutzerverzeichnis.
fn startordner(ivars: &AnwendungsIvars) -> PathBuf {
    match &ivars.messaufgabe {
        Some(aufgabe) => aufgabe.startordner().to_path_buf(),
        None => heimatverzeichnis(),
    }
}

/// Das Benutzerverzeichnis.
///
/// `$HOME` ist auf einem Mac gesetzt, solange die Anwendung fuer einen
/// angemeldeten Nutzer laeuft. Fehlt die Variable trotzdem, ist das
/// Wurzelverzeichnis der einzige Ordner, den es mit Sicherheit gibt.
fn heimatverzeichnis() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}
