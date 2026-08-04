//! Der Start: `NSApplication`, der Anwendungsdelegierte, das eine Fenster.
//!
//! KRK laeuft als gewoehnliche Anwendung im Vordergrund
//! (`NSApplicationActivationPolicy::Regular`), auch wenn `cargo run` sie ohne
//! Buendel startet. Fuer die Abnahme zaehlt trotzdem allein der Start ueber
//! `target/KRK.app`: nur ein signiertes Buendel loest die Rueckfragen von TCC
//! aus, und ein nacktes Binaerprogramm erbt stattdessen die Freigaben des
//! Terminals.
//!
//! # Was der Delegierte haelt
//!
//! ```text
//! Anwendungsdelegierter
//!   ├─ Fenstermodell        aktives Dateifenster, Sichtbarkeit, Breiten
//!   ├─ Aufteilung           die NSSplitView mit ihren vier Bereichen
//!   ├─ Dateifenster × 2     Tableiste, Dateiliste, Statuszeile, Tabs
//!   ├─ NSWindow             genau eines, siehe unten
//!   ├─ Tastenabgriff        der eine Eintrittspunkt fuer Tastendruecke
//!   ├─ Dateisystemwache     FSEvents auf den sichtbaren Ordnern (C9)
//!   ├─ Datentraegerwache    NSWorkspace auf Einhaengen und Auswerfen (C9)
//!   └─ Sitzungsschreiber    gebuendelt, hoechstens alle zwei Sekunden
//! ```
//!
//! Die beiden Wachen stehen hier aus demselben Grund wie der Tastenabgriff:
//! ohne Halter meldet sich ein Beobachter beim Fallenlassen sofort wieder ab.
//!
//! # Der Weg einer fremden Aenderung
//!
//! ```text
//!  Dateisystemwache ──> auffrischung::ordner_neu_lesen ──> Dateifenster::neu_lesen
//!  Datentraegerwache ─> auffrischung::datentraeger_verloren ─> wechseln + melden
//!
//!  jede Navigation ───> Dateisystemwache neu aufsetzen
//! ```
//!
//! Der Anwendungsdelegierte setzt beides zusammen: er ist die einzige Stelle,
//! die beide Dateifenster **und** das Fenstermodell haelt, und damit die
//! einzige, die die Frage "welche Ordner stehen gerade auf dem Schirm"
//! beantworten kann. Die Antwort selbst rechnet [`crate::auffrischung`]; hier
//! steht nur die Zuleitung.
//!
//! **KRK haelt in dieser Runde genau ein Anwendungsfenster.** Die beiden
//! Dateifenster aus C1 sind Bereiche darin und keine zwei Fenster des Systems.
//! Der Nutzer hat das am 260804-0830 mit Moeglichkeit 2 aus
//! `decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
//! festgelegt: das Fenster ueberlebt sein Schliessen, und zwei Wege holen es
//! zurueck, der Menueeintrag "Fenster einblenden" auf Cmd+N und der Klick auf
//! das Dock-Symbol ueber `applicationShouldHandleReopen:`. Ein laufendes KRK
//! ohne Fenster und ohne Rueckweg gibt es damit nicht mehr.
//!
//! # Der Weg eines Tastendrucks
//!
//! Der Ereignisabgriff kennt kein Dateifenster; er liefert eine [`Eingabe`] an
//! [`Anwendungsdelegierter::eingabe_ausfuehren`]. Der teilt auf: was das
//! Fenster als ganzes betrifft, bleibt hier, alles uebrige geht an die
//! Datenquelle des **aktiven** Dateifensters. Eine zweite Stelle, die
//! entscheidet, wohin ein Tastendruck geht, entsteht nicht.
//!
//! Zwei Sorten von Eingabe kommen an. Ein [`Kommando`] ist eine nachgeschlagene
//! Funktion; ein Zeichen gehoert der Sprungmarke aus C2 und damit immer dem
//! aktiven Dateifenster, weil sie die Liste durchsucht, die vor dem Nutzer
//! steht.
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
//!
//! **Ein Messlauf ruehrt die Sitzung des Nutzers nicht an.** Er laedt
//! `session.toml` nicht und schreibt sie nicht, und allein das linke
//! Dateifenster liest den Pruefordner. Beides haelt gemessen, was Schritt 8
//! gemessen hat: eine wiederhergestellte Sitzung braechte fremde Ordner in die
//! Messung, und ein zweiter Lesevorgang auf denselben Pruefordner machte den
//! Kaltstart zur Haelfte warm.

use std::cell::{Cell, OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes,
    NSTimer,
};

use krk_core::ablage::sitzung::Sitzungsschreiber;
use krk_core::ablage::{Ablage, Datei, Fensterseite, Sitzung, pfade};
use krk_core::operation::{
    self, Art, Auftrag, Bericht, Konfliktantwort, Konfliktentscheid, Lauf, Meldung, freier_name,
};
use krk_core::tasten::Kommando;
use krk_core::tasten::belegung;

use crate::auffrischung::{self, Dateifenstersicht};
use crate::fenstermodell::{BREITENSCHRITT, Bereich, Fenstermodell};
use crate::kommandos::operationen::{self, Fokus, Konfliktfrage, Vorgangszustand};
use crate::messmodus::{Anweisung, Aufgabe, Messlauf, Zustand};
use crate::tabs::Tabliste;

use super::aufteilung::Aufteilung;
use super::bildtakt::{self, Zeichenende};
use super::blaetter::fortschritt::Fortschrittsblatt;
use super::blaetter::{Blattgriff, konflikt, loeschbestaetigung, uebersprungen};
use super::ereignisse::{self, Eingabe, Tastenabgriff};
use super::fenster::{self, FensterDelegierter};
use super::fsevents::Dateisystemwache;
use super::menue;
use super::papierkorb::Systempapierkorb;
use super::tabelle::Dateifenster;
use super::volumes::{Datentraeger, Datentraegerwache, Wechsel};

/// Der Rueckgabewert, mit dem ein Messlauf ohne Bildschirm endet.
const OHNE_BILDSCHIRM: i32 = 3;

/// Ein laufender Dateivorgang, aus der Sicht des Hauptfadens (C4).
///
/// Es gibt hoechstens einen. Solange er laeuft, steht sein Blatt am Fenster und
/// nimmt jeden Tastenbefehl ausser dem Abbruch entgegen; ein zweiter Vorgang
/// daneben waere ein zweites Blatt an demselben Fenster, und AppKit stellte es
/// ohnehin hinten an.
struct Vorgang {
    /// Was geschieht. Traegt die Ueberschrift und die Abschlussmeldung.
    art: Art,
    /// Der Ordner, aus dem die Eintraege stammen.
    quellordner: PathBuf,
    /// Wie viele Positionen der Nutzer ausgewaehlt hatte.
    positionen: usize,
    /// Wann der Vorgang begonnen hat. Der Verzug misst ab hier.
    begonnen: Instant,
    /// Der Zustand, den der Vermittlerfaden fuellt.
    zustand: Arc<Vorgangszustand>,
    /// Das Fortschrittsblatt, sobald es steht.
    blatt: RefCell<Option<Fortschrittsblatt>>,
    /// Ob gerade eine Konfliktfrage auf dem Schirm steht.
    ///
    /// Solange sie steht, geht kein Fortschrittsblatt auf: AppKit stellte das
    /// zweite Blatt hinter das erste, und der Arbeitsfaden wartete auf eine
    /// Antwort, die niemand geben kann.
    konflikt_steht: Cell<bool>,
}

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
    /// Das aktive Dateifenster, die Sichtbarkeit und die Breiten.
    modell: RefCell<Fenstermodell>,
    fenster: OnceCell<Retained<NSWindow>>,
    fenster_delegierter: OnceCell<Retained<FensterDelegierter>>,
    aufteilung: OnceCell<Aufteilung>,
    /// Die beiden Dateifenster, links zuerst.
    dateifenster: OnceCell<[Dateifenster; 2]>,
    tastenabgriff: OnceCell<Tastenabgriff>,
    /// Die Beobachtung der sichtbaren Ordner (C9).
    ///
    /// Veraenderlich und nicht einmalig wie die uebrigen Halter: ein
    /// `FSEventStream` aendert seine Pfadliste nach dem Anlegen nicht mehr,
    /// also wird bei jeder Navigation ein neuer eingerichtet und der alte
    /// fallen gelassen. Leer, solange kein Ordner feststeht, und dann, wenn
    /// sich der Strom nicht einrichten liess.
    dateisystemwache: RefCell<Option<Dateisystemwache>>,
    /// Die Beobachtung der Datentraeger (C9). Sie steht fuer die ganze
    /// Laufzeit, weil sie an keinem Pfad haengt.
    datentraegerwache: OnceCell<Datentraegerwache>,
    /// Der gebuendelte Schreiber fuer `session.toml`.
    ///
    /// Leer im Messmodus und dann, wenn sich der Ablageordner nicht oeffnen
    /// liess. Im zweiten Fall steht die Meldung dazu in der Statuszeile.
    sitzungsschreiber: RefCell<Option<Sitzungsschreiber>>,
    /// Ob eine Meldung ueber einen gescheiterten Schreibvorgang schon steht.
    ///
    /// Ohne dieses Kennzeichen ueberschriebe ein dauerhaft scheiternder
    /// Schreibvorgang alle zwei Sekunden jede andere Meldung.
    schreibfehler_gemeldet: Cell<bool>,
    /// Die laufende Dateioperation aus C4, falls eine laeuft.
    vorgang: RefCell<Option<Vorgang>>,
    /// Ein Blatt, das auf eine Antwort des Nutzers wartet: die Konfliktfrage,
    /// die Rueckfrage vor dem endgueltigen Loeschen oder die Abschlussliste.
    ///
    /// Es steht hier, damit die Escape-Taste es schliessen kann. Ein `NSButton`
    /// traegt genau eine Tastenentsprechung, und die Eingabetaste liegt in der
    /// Rueckfrage auf "Abbrechen"; der zweite Weg zum Abbruch laeuft deshalb
    /// ueber den Befehl `abbrechen` aus `resources/default-keymap.toml`.
    offenes_blatt: RefCell<Option<Blattgriff>>,
    /// Der Ablauf der Messung. Der Bildtakt haelt eine zweite Referenz.
    messlauf: OnceCell<Rc<RefCell<Messlauf>>>,
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

        /// Der Menueeintrag "Fenster einblenden" (C7).
        ///
        /// Er erreicht den Delegierten ueber die Antwortkette, an deren Ende
        /// `NSApplication` seinen Delegierten fragt. Genau deshalb traegt der
        /// Eintrag kein festes Ziel: er bleibt damit auch dann bedienbar, wenn
        /// kein Fenster offen ist, und das ist der Fall, fuer den es ihn gibt.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(fensterEinblenden:))]
        fn fenster_einblenden(&self, _absender: Option<&AnyObject>) {
            self.fenster_zeigen();
        }
    }

    // SAFETY: `NSApplicationDelegate` stellt keine Bedingungen.
    unsafe impl NSApplicationDelegate for Anwendungsdelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn start_abgeschlossen(&self, _meldung: &NSNotification) {
            self.oberflaeche_aufbauen();
        }

        /// Der Klick auf das Dock-Symbol (C7).
        ///
        /// Der zweite der beiden Wege zurueck zum geschlossenen Fenster. Er
        /// liefert `false`, weil KRK das Fenster selbst nach vorn holt und
        /// AppKit nichts weiter tun soll.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationShouldHandleReopen:hasVisibleWindows:))]
        fn wieder_geoeffnet(&self, _absender: &NSApplication, sichtbare_fenster: bool) -> bool {
            if !sichtbare_fenster {
                self.fenster_zeigen();
            }
            false
        }

        /// KRK wird beendet: den letzten Sitzungsstand schreiben.
        ///
        /// Der eine Schreibvorgang ohne Ruecksicht auf den Takt, den
        /// `### Frage 4` des Plans neben der Buendelung zusagt.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationWillTerminate:))]
        fn wird_beendet(&self, _meldung: &NSNotification) {
            self.sitzung_vormerken();
            let sitzung = self.sitzung_bauen();
            let mut schreiber = self.ivars().sitzungsschreiber.borrow_mut();
            if let Some(schreiber) = schreiber.as_mut() {
                let jetzt = Instant::now();
                let _ = schreiber.vormerken(sitzung, jetzt);
                let _ = schreiber.beenden(jetzt);
            }
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
            modell: RefCell::new(Fenstermodell::aus_sitzung(&Sitzung::default())),
            fenster: OnceCell::new(),
            fenster_delegierter: OnceCell::new(),
            aufteilung: OnceCell::new(),
            dateifenster: OnceCell::new(),
            tastenabgriff: OnceCell::new(),
            dateisystemwache: RefCell::new(None),
            datentraegerwache: OnceCell::new(),
            sitzungsschreiber: RefCell::new(None),
            schreibfehler_gemeldet: Cell::new(false),
            vorgang: RefCell::new(None),
            offenes_blatt: RefCell::new(None),
            messlauf: OnceCell::new(),
            zeichenende: OnceCell::new(),
            ausloesetakt: OnceCell::new(),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Baut die vier Bereiche, stellt die Sitzung her und liest die Ordner.
    fn oberflaeche_aufbauen(&self) {
        let mtm = self.mtm();
        let ivars = self.ivars();

        let (sitzung, mut meldungen) = self.sitzung_laden();
        *ivars.modell.borrow_mut() = Fenstermodell::aus_sitzung(&sitzung);

        let dateifenster = [
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[0])),
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[1])),
        ];
        let aufteilung = Aufteilung::bauen(mtm, [&dateifenster[0], &dateifenster[1]]);
        let fenster_delegierter = FensterDelegierter::neu(
            mtm,
            [
                dateifenster[0].quelle().retain(),
                dateifenster[1].quelle().retain(),
            ],
        );
        let fenster = fenster::hauptfenster(mtm, aufteilung.sicht(), &fenster_delegierter);

        // Erst festhalten, dann anzeigen: das Fenster haelt seinen Delegierten
        // schwach, die Tabelle haelt Datenquelle und Delegierten schwach.
        let _ = ivars.dateifenster.set(dateifenster);
        let _ = ivars.aufteilung.set(aufteilung);
        let _ = ivars.fenster_delegierter.set(fenster_delegierter);
        let _ = ivars.fenster.set(fenster);

        // Ein Klick in eine der beiden Listen macht sie zur aktiven. Der
        // Rueckruf haelt den Delegierten **schwach**, sonst schloesse sich der
        // Ring Delegierter → Dateifenster → Quelle → Rueckruf → Delegierter.
        for seite in Fensterseite::ALLE {
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .aktivierung_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.aktives_setzen(seite);
                    }
                }));
            // Jede Navigation setzt die Dateisystembeobachtung neu auf. Auch
            // dieser Rueckruf haelt den Delegierten **schwach**, aus demselben
            // Grund wie der darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .ordnerwechsel_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.dateisystemwache_nachziehen();
                    }
                }));
        }

        self.aufteilung_nachziehen();
        self.tastenabgriff_einrichten(&mut meldungen);
        self.datentraegerwache_einrichten();
        self.lesevorgaenge_starten();
        if let Some(fenster) = ivars.fenster.get() {
            fenster.makeKeyAndOrderFront(None);
        }
        for meldung in meldungen {
            self.dateifenster(Fensterseite::Links)
                .quelle()
                .meldung_zeigen(&meldung);
        }
        self.messmodus_einrichten();
    }

    /// Laedt die Sitzung und den Ablageordner, oder liefert den
    /// Auslieferungszustand.
    ///
    /// Im Messmodus wird nichts geladen und nichts geschrieben, siehe den
    /// Modulkopf.
    fn sitzung_laden(&self) -> (Sitzung, Vec<String>) {
        let ivars = self.ivars();
        if ivars.messaufgabe.is_some() {
            return (Sitzung::default(), Vec::new());
        }
        let mut meldungen = Vec::new();
        let ablage = match Ablage::im_benutzerverzeichnis() {
            Ok(ablage) => ablage,
            Err(fehler) => {
                meldungen.push(format!(
                    "der Ablageordner liess sich nicht oeffnen, die Sitzung wird nicht gesichert: {fehler}"
                ));
                return (Sitzung::default(), meldungen);
            }
        };
        *ivars.sitzungsschreiber.borrow_mut() = Some(ablage.sitzungsschreiber());
        let (sitzung, meldung) = ablage.laden::<Sitzung>(Datei::Sitzung).mit_meldung();
        meldungen.extend(meldung);
        (sitzung, meldungen)
    }

    /// Richtet den einen Eintrittspunkt fuer Tastendruecke ein.
    fn tastenabgriff_einrichten(&self, meldungen: &mut Vec<String>) {
        let (belegung, meldung) = belegung::fuer_den_betrieb();
        meldungen.extend(meldung);

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let abgriff = Tastenabgriff::einrichten(
            self.mtm(),
            belegung,
            self.ivars().tasten_protokoll,
            move |eingabe| match schwach.load() {
                Some(selbst) => selbst.eingabe_ausfuehren(eingabe),
                None => false,
            },
        );
        match abgriff {
            Some(abgriff) => {
                let _ = self.ivars().tastenabgriff.set(abgriff);
            }
            // Ohne Abgriff bewegt keine Taste mehr die Auswahl. Das still
            // hinzunehmen hiesse, eine Anwendung auszuliefern, deren erste
            // Maxime die Tastatursteuerung ist und die keine hat. Der Abbruch
            // mit Hinweisfenster ist Schritt 6b und steht noch aus.
            None => eprintln!(
                "krk: der Tastenabgriff liess sich nicht einrichten, die Tastatursteuerung bleibt aus"
            ),
        }
    }

    // ------------------------------------------------------------------
    // Dateisystem und Datentraeger (C9)
    // ------------------------------------------------------------------

    /// Setzt die Beobachtung der sichtbaren Ordner neu auf (C9).
    ///
    /// Gerufen nach jeder Navigation und nach jedem Ein- oder Ausblenden des
    /// zweiten Dateifensters. Der alte Strom faellt dabei; ein
    /// `FSEventStream` aendert seine Pfadliste nach dem Anlegen nicht mehr,
    /// und einen zweiten Strom danebenzustellen hiesse, denselben Ordner
    /// doppelt zu beobachten.
    ///
    /// **Im Messmodus geschieht nichts.** Ein Messlauf misst die Zusagen aus
    /// C8 auf einem Pruefordner, den niemand nebenher aendert; ein Strom
    /// darauf brachte Arbeit in die Messung, die im Betrieb an anderer Stelle
    /// anfiele. Dieselbe Haltung wie bei der Sitzung, die ein Messlauf weder
    /// laedt noch schreibt.
    fn dateisystemwache_nachziehen(&self) {
        if self.ivars().messaufgabe.is_some() {
            return;
        }
        let ordner = auffrischung::sichtbare_ordner(self);
        // Erst den alten Strom fallen lassen, dann den neuen anlegen: sonst
        // beobachteten beide gleichzeitig dieselben Pfade.
        *self.ivars().dateisystemwache.borrow_mut() = None;

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let wache = Dateisystemwache::einrichten(&ordner, move |gemeldet| {
            let Some(selbst) = schwach.load() else {
                return;
            };
            for pfad in gemeldet {
                auffrischung::ordner_neu_lesen(&*selbst, pfad);
            }
        });
        if wache.is_none() && !ordner.is_empty() {
            // Ohne Strom zeigt KRK fremde Aenderungen nicht mehr an. Das still
            // hinzunehmen waere die Sorte Fehler, die erst dem Nutzer auffaellt.
            self.dateifenster(self.ivars().modell.borrow().aktiv())
                .quelle()
                .meldung_zeigen(
                    "die Ordner lassen sich nicht beobachten; fremde Aenderungen erscheinen erst nach einem Ordnerwechsel",
                );
        }
        *self.ivars().dateisystemwache.borrow_mut() = wache;
    }

    /// Richtet die Beobachtung der Datentraeger ein (C9).
    ///
    /// Sie haengt an keinem Pfad und wird deshalb genau einmal eingerichtet.
    /// Im Messmodus unterbleibt sie, aus demselben Grund wie die
    /// Dateisystembeobachtung.
    fn datentraegerwache_einrichten(&self) {
        if self.ivars().messaufgabe.is_some() {
            return;
        }
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let wache = Datentraegerwache::einrichten(self.mtm(), move |gemeldet| {
            if let Some(selbst) = schwach.load() {
                selbst.datentraeger_gewechselt(gemeldet);
            }
        });
        let _ = self.ivars().datentraegerwache.set(wache);
    }

    /// Ein Datentraeger ist gekommen oder gegangen (C9).
    fn datentraeger_gewechselt(&self, gemeldet: Datentraeger) {
        match gemeldet.art {
            // C5 baut daraus die Geraeteleiste; das ist S18. Bis dahin gibt es
            // in dieser Runde nichts zu tun: kein Dateifenster zeigt einen
            // Ordner, den es vorher nicht gab.
            Wechsel::Eingehaengt => {}
            // Beide Richtungen enden hier. `willUnmount` ist der geordnete
            // Auswurf und der Zeitpunkt, zu dem KRK den Ordner freigeben muss,
            // damit der Auswurf nicht an ihm scheitert; `didUnmount` faengt das
            // abgezogene Medium ab, das niemand vorher angekuendigt hat. Ein
            // zweites Mal richtet der Aufruf nichts an: nach dem ersten steht
            // kein Dateifenster mehr auf dem Datentraeger.
            Wechsel::WirdAusgeworfen | Wechsel::Ausgeworfen => {
                let ausweichziel = benutzerverzeichnis();
                auffrischung::datentraeger_verloren(
                    self,
                    &gemeldet.pfad,
                    &gemeldet.name,
                    &ausweichziel,
                );
            }
        }
    }

    /// Startet die Lesevorgaenge in der Reihenfolge, die das Modell vorgibt.
    ///
    /// Zuerst der sichtbare Tab jedes sichtbaren Dateifensters. Die verdeckten
    /// folgen, sobald der sichtbare bedienbar ist; das loest der Einzugstakt
    /// des jeweiligen Dateifensters aus, siehe [`crate::tabs`].
    fn lesevorgaenge_starten(&self) {
        if let Some(aufgabe) = &self.ivars().messaufgabe {
            // Ein Messlauf liest allein den Pruefordner, und allein links.
            let pfad = aufgabe.startordner().to_path_buf();
            self.dateifenster(Fensterseite::Links)
                .quelle()
                .ordner_lesen(&pfad, None);
            return;
        }
        let uebersicht = [
            self.dateifenster(Fensterseite::Links).quelle().uebersicht(),
            self.dateifenster(Fensterseite::Rechts)
                .quelle()
                .uebersicht(),
        ];
        let reihenfolge = self.ivars().modell.borrow().lesereihenfolge(uebersicht);
        for (seite, stelle) in reihenfolge {
            if stelle == uebersicht[seite.index()].sichtbar {
                self.dateifenster(seite).quelle().sichtbaren_lesen();
            }
        }
    }

    /// Eines der beiden Dateifenster.
    fn dateifenster(&self, seite: Fensterseite) -> &Dateifenster {
        &self
            .ivars()
            .dateifenster
            .get()
            .expect("die Dateifenster stehen seit `oberflaeche_aufbauen`")[seite.index()]
    }

    // ------------------------------------------------------------------
    // Kommandos
    // ------------------------------------------------------------------

    /// Fuehrt aus, was der Ereignisabgriff geliefert hat.
    ///
    /// Die eine Stelle, die entscheidet, wohin ein Tastendruck geht. Ein
    /// getipptes Zeichen gehoert immer dem aktiven Dateifenster, weil die
    /// Sprungmarke aus C2 die Liste durchsucht, die vor dem Nutzer steht.
    fn eingabe_ausfuehren(&self, eingabe: Eingabe) -> bool {
        if self.ivars().dateifenster.get().is_none() {
            return false;
        }
        match eingabe {
            Eingabe::Kommando(kommando) => self.kommando_ausfuehren(kommando),
            Eingabe::Zeichen(zeichen) => {
                // Ein getipptes Zeichen gehoert dem Blatt, solange eines steht:
                // die Sprungmarke durchsucht eine Liste, die der Nutzer gerade
                // nicht bedient.
                if self.blatt_steht() {
                    return false;
                }
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .sprungmarke_tippen(zeichen)
            }
        }
    }

    /// Ob am Hauptfenster gerade ein Blatt steht.
    ///
    /// Die eine Abfrage dafuer. Sie deckt jedes Blatt ab, auch die Pfadeingabe
    /// aus C2 und die kommenden aus S17, und nicht nur die vier aus diesem
    /// Schritt.
    fn blatt_steht(&self) -> bool {
        self.ivars()
            .fenster
            .get()
            .and_then(|fenster| fenster.attachedSheet())
            .is_some()
    }

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Liefert, ob es ausgefuehrt wurde; nur dann schluckt der Abgriff das
    /// Ereignis.
    fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        // Solange ein Blatt steht oder eine Dateioperation laeuft, kommt allein
        // der Abbruch durch. Alles uebrige geht unveraendert an AppKit weiter,
        // damit das Blatt seine eigene Tastaturbedienung behaelt.
        if (self.blatt_steht() || self.ivars().vorgang.borrow().is_some())
            && !operationen::waehrend_blatt_erlaubt(kommando)
        {
            return false;
        }

        let ausgefuehrt = match kommando {
            Kommando::Kopieren => self.uebertragen(kommando),
            Kommando::Verschieben => self.uebertragen(kommando),
            Kommando::InPapierkorb => self.in_den_papierkorb(),
            Kommando::EndgueltigLoeschen => self.endgueltig_loeschen(),
            Kommando::Abbrechen => self.abbrechen(),
            Kommando::FensterWechseln => self.ivars().modell.borrow_mut().fenster_wechseln(),
            Kommando::LeisteUmschalten => self.bereich_umschalten(Bereich::Lesezeichen),
            Kommando::ZweitesFensterUmschalten => self.bereich_umschalten(Bereich::Rechts),
            Kommando::VorschauUmschalten => self.bereich_umschalten(Bereich::Vorschau),
            Kommando::FensterEinblenden => {
                self.fenster_zeigen();
                true
            }
            Kommando::BereichVerbreitern => self.breite_aendern(BREITENSCHRITT),
            Kommando::BereichVerschmaelern => self.breite_aendern(-BREITENSCHRITT),
            // Alles uebrige gehoert dem aktiven Dateifenster.
            andere => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .kommando_ausfuehren(andere)
            }
        };
        if ausgefuehrt {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
        ausgefuehrt
    }

    /// Blendet einen Bereich aus oder wieder ein (C7).
    fn bereich_umschalten(&self, bereich: Bereich) -> bool {
        let umgeschaltet = self.ivars().modell.borrow_mut().umschalten(bereich);
        // Mit dem zweiten Dateifenster kommt und geht ein beobachteter Ordner.
        // Die beiden Randbereiche zeigen keinen.
        if umgeschaltet && bereich == Bereich::Rechts {
            self.dateisystemwache_nachziehen();
        }
        umgeschaltet
    }

    /// Aendert die Breite des aktiven Dateifensters um einen Schritt (C7).
    ///
    /// Der "aktive Bereich" der beiden Kuerzel ist das aktive Dateifenster.
    /// Die Lesezeichenleiste und die Vorschau bekommen ihre Breite mit der
    /// Maus; ihnen ein eigenes Kuerzelpaar zu geben, hiesse vier Befehle fuer
    /// eine Sache, und C7 verlangt sie nicht.
    fn breite_aendern(&self, betrag: f64) -> bool {
        // Zuerst nachlesen, was wirklich auf dem Schirm steht: der Nutzer kann
        // die Trennlinie zwischendurch mit der Maus verschoben haben, und ein
        // Schritt auf eine ueberholte Zahl spraenge zurueck.
        if let Some(aufteilung) = self.ivars().aufteilung.get() {
            self.ivars()
                .modell
                .borrow_mut()
                .breiten_uebernehmen(aufteilung.gemessene_breiten());
        }
        let mut modell = self.ivars().modell.borrow_mut();
        let bereich = Bereich::von_seite(modell.aktiv());
        modell.breite_aendern(bereich, betrag);
        true
    }

    /// Macht das genannte Dateifenster zum aktiven.
    fn aktives_setzen(&self, seite: Fensterseite) {
        if self.ivars().modell.borrow_mut().aktiv_setzen(seite) {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
    }

    /// Holt das Fenster nach vorn (C7).
    ///
    /// Es wird nicht angelegt: `setReleasedWhenClosed(false)` haelt es ueber
    /// sein Schliessen hinweg am Leben, und der Delegierte haelt es weiter.
    fn fenster_zeigen(&self) {
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        fenster.makeKeyAndOrderFront(None);
        NSApplication::sharedApplication(self.mtm()).activate();
    }

    /// Schreibt Sichtbarkeit, Breiten und die Markierung des aktiven
    /// Dateifensters in die Ansicht.
    fn aufteilung_nachziehen(&self) {
        let Some(aufteilung) = self.ivars().aufteilung.get() else {
            return;
        };
        let (breiten, sichtbar, aktiv) = {
            let modell = self.ivars().modell.borrow();
            (modell.breiten(), modell.sichtbarkeit(), modell.aktiv())
        };
        aufteilung.anwenden(&breiten, &sichtbar);
        aufteilung.aktives_markieren(aktiv);
    }

    // ------------------------------------------------------------------
    // Dateioperationen (C4)
    // ------------------------------------------------------------------

    /// Kopieren oder Verschieben in den Ordner des anderen Dateifensters (C4).
    fn uebertragen(&self, kommando: Kommando) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let ziel = self
            .dateifenster(aktiv.andere())
            .quelle()
            .angezeigter_ordner();
        let art = match kommando {
            Kommando::Verschieben => Art::Verschieben { ziel },
            // Der Aufrufer schickt nur diese beiden; ein drittes Kommando hier
            // waere ein Fehler im Zweig darueber und nicht in dieser Zeile.
            _ => Art::Kopieren { ziel },
        };
        self.auftrag_stellen(art)
    }

    /// Die Auswahl in den Papierkorb des Systems raeumen (C4, Taste Delete).
    ///
    /// Sofort und ohne Rueckfrage: der Rueckweg ist der Papierkorb des Systems,
    /// und einen eigenen Rueckgaengig-Speicher fuehrt KRK nicht
    /// (`shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`).
    fn in_den_papierkorb(&self) -> bool {
        if !operationen::loeschtaste_wirkt(self.fokus()) {
            return false;
        }
        self.auftrag_stellen(Art::InDenPapierkorb)
    }

    /// Die Auswahl endgueltig loeschen, nach genau einer Rueckfrage (C4, F8).
    fn endgueltig_loeschen(&self) -> bool {
        if !operationen::loeschtaste_wirkt(self.fokus()) {
            return false;
        }
        let aktiv = self.ivars().modell.borrow().aktiv();
        let auswahl = self.dateifenster(aktiv).quelle().betroffene_eintraege();
        if auswahl.ist_leer() {
            self.melden(aktiv, "es ist nichts ausgewählt");
            return true;
        }
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };

        let (frage, erlaeuterung) = operationen::loeschfrage(&auswahl);
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = loeschbestaetigung::zeigen(
            self.mtm(),
            fenster,
            &frage,
            &erlaeuterung,
            move |bestaetigt| {
                let Some(selbst) = schwach.load() else {
                    return;
                };
                *selbst.ivars().offenes_blatt.borrow_mut() = None;
                if bestaetigt {
                    selbst.auftrag_stellen(Art::EndgueltigLoeschen);
                }
            },
        );
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
        true
    }

    /// Der Abbruchbefehl (C4).
    ///
    /// Er bedient zwei Faelle, und die Reihenfolge ist bindend: ein offenes
    /// Blatt zuerst, weil die Konfliktfrage waehrend eines laufenden Vorgangs
    /// steht und der Abbruch dann ihr gilt.
    fn abbrechen(&self) -> bool {
        let blatt = self.ivars().offenes_blatt.borrow_mut().take();
        if let Some(blatt) = blatt {
            blatt.abbrechen();
            return true;
        }
        let vorgang = self.ivars().vorgang.borrow();
        let Some(vorgang) = vorgang.as_ref() else {
            return false;
        };
        vorgang.zustand.abbrechen();
        if let Some(blatt) = vorgang.blatt.borrow().as_ref() {
            blatt.stand_setzen("Abbruch angefordert, der Vorgang endet gleich …");
        }
        true
    }

    /// Wo der Eingabefokus steht, soweit es die Loeschtasten angeht (C4).
    ///
    /// **Eine Frage, eine Antwort.** Steht ein Blatt am Fenster, ist dessen
    /// Panel das Schluesselfenster und nicht das Hauptfenster; steht die
    /// Schreibmarke in einem Textfeld, hat der Ereignisabgriff den Tastendruck
    /// ohnehin schon weitergereicht und dieses Kommando gar nicht erst
    /// erzeugt. Im Hauptfenster gibt es in dieser Runde nur die beiden
    /// Dateilisten; die Lesezeichenleiste aus C5 kommt mit S18, und mit ihr
    /// bekommt diese Abfrage einen dritten Fall.
    fn fokus(&self) -> Fokus {
        let (Some(schluessel), Some(haupt)) = (
            NSApplication::sharedApplication(self.mtm()).keyWindow(),
            self.ivars().fenster.get(),
        ) else {
            return Fokus::Anderswo;
        };
        if schluessel.isEqual(Some(haupt)) {
            Fokus::Dateifenster
        } else {
            Fokus::Anderswo
        }
    }

    /// Baut den Auftrag aus der Auswahl des aktiven Dateifensters und startet
    /// ihn.
    ///
    /// Liefert `true`, auch wenn nichts ausgewaehlt war: der Tastendruck ist
    /// dann verbraucht, und die Statuszeile sagt warum. Ihn weiterzureichen
    /// hiesse, dass F5 auf leerer Auswahl in der Menueleiste landet.
    fn auftrag_stellen(&self, art: Art) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let quelle = self.dateifenster(aktiv).quelle();
        let auswahl = quelle.betroffene_eintraege();
        if auswahl.ist_leer() {
            self.melden(aktiv, "es ist nichts ausgewählt");
            return true;
        }
        let quellordner = quelle.angezeigter_ordner();
        if art.eq(&Art::Kopieren {
            ziel: quellordner.clone(),
        }) || art.eq(&Art::Verschieben {
            ziel: quellordner.clone(),
        }) {
            self.melden(aktiv, "Quelle und Ziel sind derselbe Ordner");
            return true;
        }

        let positionen = auswahl.zahl();
        let auftrag = Auftrag {
            quellen: auswahl.pfade,
            art: art.clone(),
            konfliktregel: Default::default(),
            uebertragung: Default::default(),
        };
        // Hier bekommt die Schnittstelle aus `operation/loeschen.rs` ihre
        // Implementierung: bis zu diesem Aufruf hatte sie im laufenden Programm
        // keine.
        let lauf = operation::starten(auftrag, Arc::new(Systempapierkorb));

        let zustand = Arc::new(Vorgangszustand::neu());
        let fuer_faden = Arc::clone(&zustand);
        let gestartet = thread::Builder::new()
            .name("krk-vermittler".to_owned())
            .spawn(move || vermitteln(lauf, &fuer_faden));
        if let Err(fehler) = gestartet {
            // Der Lauf ist mit `gestartet` gefallen und damit abgebrochen; er
            // hat noch nichts angefasst.
            self.melden(
                aktiv,
                &format!("die Operation liess sich nicht starten: {fehler}"),
            );
            return true;
        }

        *self.ivars().vorgang.borrow_mut() = Some(Vorgang {
            art,
            quellordner,
            positionen,
            begonnen: Instant::now(),
            zustand,
            blatt: RefCell::new(None),
            konflikt_steht: Cell::new(false),
        });
        true
    }

    /// Der Weckruf des Vermittlerfadens, auf dem Hauptfaden angekommen.
    ///
    /// Der Weg dorthin geht ueber die Hauptschlange und den Anwendungsdelegierten
    /// von `NSApplication`, damit der Weckruf selbst nichts festhalten muss, was
    /// dem Hauptfaden gehoert.
    fn vorgang_einziehen(mtm: MainThreadMarker) {
        let Some(delegierter) = NSApplication::sharedApplication(mtm).delegate() else {
            return;
        };
        let Ok(selbst) = delegierter.downcast::<Anwendungsdelegierter>() else {
            return;
        };
        selbst.vorgang_zeichnen();
    }

    /// Zeichnet den Stand des laufenden Vorgangs.
    ///
    /// **Die Reihenfolge ist bindend** und im Modulkopf von
    /// [`crate::kommandos::operationen`] begruendet: erst `gezeichnet`, dann
    /// den Stand lesen, dann zeichnen. Umgekehrt fiele eine Meldung, die
    /// waehrend des Zeichnens eintrifft, zwischen die beiden Schritte.
    fn vorgang_zeichnen(&self) {
        // Die Ausleihe endet vor jedem AppKit-Aufruf: ein Blatt ruft zurueck,
        // und der Rueckruf will denselben `RefCell`.
        let Some((zustand, art, positionen, begonnen, konflikt_steht)) = ({
            let vorgang = self.ivars().vorgang.borrow();
            vorgang.as_ref().map(|vorgang| {
                (
                    Arc::clone(&vorgang.zustand),
                    vorgang.art.clone(),
                    vorgang.positionen,
                    vorgang.begonnen,
                    vorgang.konflikt_steht.get(),
                )
            })
        }) else {
            return;
        };

        zustand.buendelung.gezeichnet();
        let (fortschritt, konflikt, bericht) = zustand.aendern(|stand| {
            (
                stand.fortschritt.clone(),
                stand.konflikt.take(),
                stand.bericht.take(),
            )
        });

        if let Some(bericht) = bericht {
            self.vorgang_beenden(&bericht);
            return;
        }
        if let Some(konflikt) = konflikt {
            self.konflikt_fragen(konflikt);
            return;
        }
        if konflikt_steht {
            return;
        }
        if !operationen::blatt_faellig(begonnen, Instant::now()) {
            return;
        }
        self.fortschritt_zeigen(
            operationen::ueberschrift(&art),
            &operationen::standtext(fortschritt.as_ref(), positionen),
        );
    }

    /// Zeigt das Fortschrittsblatt oder schreibt den neuen Stand hinein.
    fn fortschritt_zeigen(&self, ueberschrift: &str, stand: &str) {
        let steht = {
            let vorgang = self.ivars().vorgang.borrow();
            let Some(vorgang) = vorgang.as_ref() else {
                return;
            };
            let blatt = vorgang.blatt.borrow();
            match blatt.as_ref() {
                Some(blatt) => {
                    blatt.stand_setzen(stand);
                    true
                }
                None => false,
            }
        };
        if steht {
            return;
        }

        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let blatt = super::blaetter::fortschritt::zeigen(
            self.mtm(),
            fenster,
            ueberschrift,
            stand,
            move || {
                if let Some(selbst) = schwach.load() {
                    selbst.abbrechen();
                }
            },
        );
        let vorgang = self.ivars().vorgang.borrow();
        if let Some(vorgang) = vorgang.as_ref() {
            *vorgang.blatt.borrow_mut() = Some(blatt);
        }
    }

    /// Stellt die Konfliktfrage aus C4 und schickt die Antwort zurueck.
    ///
    /// Das Fortschrittsblatt weicht dafuer: an einem Fenster steht genau ein
    /// Blatt, und AppKit stellte das zweite hinter das erste. Es geht mit der
    /// naechsten Meldung von selbst wieder auf.
    fn konflikt_fragen(&self, frage: Konfliktfrage) {
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        {
            let vorgang = self.ivars().vorgang.borrow();
            let Some(vorgang) = vorgang.as_ref() else {
                return;
            };
            vorgang.konflikt_steht.set(true);
            if let Some(blatt) = vorgang.blatt.borrow_mut().take() {
                blatt.schliessen();
            }
        }

        let vorschlag = freier_name(&frage.ziel);
        let antwortweg = frage.antwort.clone();
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = konflikt::zeigen(
            self.mtm(),
            fenster,
            &frage.quelle,
            &frage.ziel,
            &vorschlag,
            move |entscheid| {
                // Ein leerer Name waere kein Name; dann bleibt der Eintrag
                // stehen, statt unter einem Namen zu landen, den niemand
                // getippt hat. Die Pruefung im Kern faenge ihn ebenfalls ab und
                // meldete ihn als uebersprungen; hier ist sie naeher am Nutzer.
                let entscheid = match &entscheid.antwort {
                    Konfliktantwort::UmbenennenIn(name) if name.is_empty() => Konfliktentscheid {
                        antwort: Konfliktantwort::Ueberspringen,
                        fuer_alle_weiteren: false,
                    },
                    _ => entscheid,
                };
                let _ = antwortweg.send(entscheid);
                if let Some(selbst) = schwach.load() {
                    *selbst.ivars().offenes_blatt.borrow_mut() = None;
                    let vorgang = selbst.ivars().vorgang.borrow();
                    if let Some(vorgang) = vorgang.as_ref() {
                        vorgang.konflikt_steht.set(false);
                    }
                }
            },
        );
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
    }

    /// Schliesst den Vorgang ab: Blatt weg, Meldung, Auffrischung, Liste.
    fn vorgang_beenden(&self, bericht: &Bericht) {
        let Some(vorgang) = self.ivars().vorgang.borrow_mut().take() else {
            return;
        };
        if let Some(blatt) = vorgang.blatt.borrow_mut().take() {
            blatt.schliessen();
        }

        let aktiv = self.ivars().modell.borrow().aktiv();
        self.melden(
            aktiv,
            &operationen::abschlusstext(&vorgang.art, bericht, vorgang.positionen),
        );

        // **Der eine Auffrischungspfad.** Der gemeldete Abschluss einer
        // Dateioperation ist der zweite Ausloeser von `ordner_neu_lesen`, den
        // S14 angelegt und `### Frage 3` zugesagt hat. Ein eigener Weg fuer die
        // selbst verursachte Aenderung entsteht nicht.
        auffrischung::ordner_neu_lesen(self, &vorgang.quellordner);
        if let Art::Kopieren { ziel } | Art::Verschieben { ziel } = &vorgang.art {
            auffrischung::ordner_neu_lesen(self, ziel);
        }

        let Some((frage, liste)) = operationen::uebersprungenliste(&bericht.uebersprungen) else {
            return;
        };
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = uebersprungen::zeigen(self.mtm(), fenster, &frage, &liste, move || {
            if let Some(selbst) = schwach.load() {
                *selbst.ivars().offenes_blatt.borrow_mut() = None;
            }
        });
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
    }

    /// Stellt einen Text in die Statuszeile des genannten Dateifensters.
    fn melden(&self, seite: Fensterseite, text: &str) {
        self.dateifenster(seite).quelle().meldung_zeigen(text);
    }

    // ------------------------------------------------------------------
    // Sitzung
    // ------------------------------------------------------------------

    /// Der Sitzungszustand, wie er auf die Platte gehoert.
    fn sitzung_bauen(&self) -> Sitzung {
        if let Some(aufteilung) = self.ivars().aufteilung.get() {
            self.ivars()
                .modell
                .borrow_mut()
                .breiten_uebernehmen(aufteilung.gemessene_breiten());
        }
        let fenster = [
            self.dateifenster(Fensterseite::Links).quelle().zustand(),
            self.dateifenster(Fensterseite::Rechts).quelle().zustand(),
        ];
        self.ivars().modell.borrow().sitzung(fenster)
    }

    /// Merkt den Sitzungszustand vor; geschrieben wird gebuendelt.
    ///
    /// Hoechstens alle zwei Sekunden, wie `### Frage 4` es vorschreibt. Ein
    /// liegengebliebener Stand geht spaetestens beim Beenden auf die Platte.
    fn sitzung_vormerken(&self) {
        if self.ivars().sitzungsschreiber.borrow().is_none() {
            return;
        }
        let sitzung = self.sitzung_bauen();
        let ergebnis = {
            let mut schreiber = self.ivars().sitzungsschreiber.borrow_mut();
            let schreiber = schreiber
                .as_mut()
                .expect("oben geprueft, und dazwischen laeuft nichts");
            schreiber.vormerken(sitzung, Instant::now())
        };
        if let Err(fehler) = ergebnis
            && !self.ivars().schreibfehler_gemeldet.replace(true)
        {
            let meldung = format!("die Sitzung liess sich nicht sichern: {fehler}");
            self.dateifenster(Fensterseite::Links)
                .quelle()
                .meldung_zeigen(&meldung);
        }
    }

    // ------------------------------------------------------------------
    // Messmodus
    // ------------------------------------------------------------------

    /// Haengt Bildtakt und Ausloesetakt ein, wenn ein Messlauf ansteht.
    fn messmodus_einrichten(&self) {
        let ivars = self.ivars();
        let Some(aufgabe) = ivars.messaufgabe.clone() else {
            return;
        };
        let Some(fenster) = ivars.fenster.get() else {
            return;
        };
        let dateifenster = self.dateifenster(Fensterseite::Links);

        // Die Rate zuerst, und ohne sie kein Messlauf. Die Regel steht in S21
        // des Plans ausgeschrieben: ein Fenster auf keinem Bildschirm heisst
        // Abbruch, nicht Ausweichen auf den Hauptbildschirm.
        let Some(hertz) = bildtakt::bildwiederholrate(fenster) else {
            eprintln!("krk: {}", crate::messmodus::OHNE_BILDSCHIRM);
            std::process::exit(OHNE_BILDSCHIRM);
        };

        let mut lauf = Messlauf::neu(aufgabe);
        lauf.rate_setzen(hertz);
        let lauf = Rc::new(RefCell::new(lauf));
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
        let (Some(lauf), Some(fenster)) = (ivars.messlauf.get(), ivars.fenster.get()) else {
            return;
        };
        let quelle = self.dateifenster(Fensterseite::Links).quelle();
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
            Anweisung::Lesen(pfad) => quelle.ordner_lesen(&pfad, None),
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

/// Was der Auffrischungspfad aus C9 von den beiden Dateifenstern braucht.
///
/// Jede Methode ist eine Zeile: der Delegierte ist die einzige Stelle, die
/// beide Dateifenster und das Fenstermodell haelt, und deshalb die einzige,
/// die die Fragen beantworten kann. Die Rechnung darauf steht in
/// [`crate::auffrischung`] und ist ohne Fenster pruefbar.
impl Dateifenstersicht for Anwendungsdelegierter {
    fn ordner(&self, seite: Fensterseite) -> PathBuf {
        self.dateifenster(seite).quelle().angezeigter_ordner()
    }

    fn sichtbar(&self, seite: Fensterseite) -> bool {
        self.ivars()
            .modell
            .borrow()
            .sichtbar(Bereich::von_seite(seite))
    }

    fn neu_lesen(&self, seite: Fensterseite) {
        self.dateifenster(seite).quelle().neu_lesen();
    }

    fn wechseln(&self, seite: Fensterseite, ziel: &Path) {
        self.dateifenster(seite).quelle().ordner_lesen(ziel, None);
    }

    fn melden(&self, seite: Fensterseite, text: &str) {
        self.dateifenster(seite).quelle().meldung_zeigen(text);
    }
}

/// Der Vermittlerfaden zwischen der Operationsmaschine und dem Hauptfaden.
///
/// **Er ist kein Takt.** Er schlaeft in `recv`, solange nichts zu melden ist,
/// und zieht dabei keinen Strom; geweckt wird er von der Meldung selbst, und er
/// weckt seinerseits den Hauptfaden. Damit haelt die Wahl des Nutzers vom
/// 260804, die Buendelung ohne Zeitgeber zu bauen
/// (`issues/260803-2007_o_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`,
/// Weg 3).
///
/// **Warum es ihn ueberhaupt gibt.** Der Empfaenger des Meldekanals darf nicht
/// zwischen Faeden geteilt werden, und der Hauptfaden darf in `recv` nicht
/// warten: das waere die Dateisystem-Arbeit auf dem Hauptfaden, die
/// `### Frage 6` ausschliesst, und L9 fiele mit ihr. Ein Faden, der wartet, ist
/// der Preis dafuer.
///
/// **Der Abbruchwunsch laeuft ueber diesen Faden zurueck**, weil der
/// [`Lauf`] hier liegt. Er wird nach jeder Meldung geprueft; die Spanne bis zum
/// Greifen ist damit die bis zur naechsten Meldung, also hoechstens der
/// Meldeabstand von 8 ms, solange eine Datei uebertragen wird.
fn vermitteln(lauf: Lauf, zustand: &Arc<Vorgangszustand>) {
    while let Ok(meldung) = lauf.meldungen().recv() {
        if zustand.abgebrochen() && !lauf.ist_abgebrochen() {
            lauf.abbrechen();
        }
        let fertig = matches!(meldung, Meldung::Fertig(_));
        zustand.aendern(|stand| match meldung {
            Meldung::Fortschritt(fortschritt) => stand.fortschritt = Some(fortschritt),
            Meldung::Uebersprungen(eintrag) => stand.uebersprungen.push(eintrag),
            Meldung::Konflikt {
                quelle,
                ziel,
                antwort,
            } => {
                stand.konflikt = Some(Konfliktfrage {
                    quelle,
                    ziel,
                    antwort,
                });
            }
            Meldung::Fertig(bericht) => stand.bericht = Some(bericht),
        });
        // Auch der Abschluss und die Konfliktfrage gehen durch die Buendelung.
        // Verworfen wird dabei allein der **Weckruf**, nicht die Meldung: steht
        // schon einer aus, hat der Hauptfaden noch nicht gelesen und findet
        // beides beim naechsten Durchgang vor.
        if zustand.buendelung.melden() {
            hauptfaden_wecken();
        }
        if fertig {
            break;
        }
    }
    lauf.warten();
}

/// Weckt den Hauptfaden, damit er den Stand des Vorgangs zeichnet.
///
/// Der Block haelt nichts fest, was dem Hauptfaden gehoert: er sucht den
/// Anwendungsdelegierten dort, wo er ohnehin steht. Damit braucht der Weckruf
/// keine Verrenkung, um einen `Retained` ueber die Fadengrenze zu tragen.
fn hauptfaden_wecken() {
    DispatchQueue::main().exec_async(|| {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Ein Abbruch waere hier trotzdem falsch, weil er eine
            // laufende Kopie um ihre Anzeige braechte und nicht um mehr.
            return;
        };
        Anwendungsdelegierter::vorgang_einziehen(mtm);
    });
}

/// Der Ordner, auf den ein Dateifenster ausweicht, wenn sein Datentraeger
/// verschwindet (C9).
///
/// Das Benutzerverzeichnis, und ohne eines die Wurzel. Derselbe Rueckfall wie
/// beim Standardordner eines Tabs in `krk-core`: ein Dateifenster muss einen
/// Ordner zeigen, und `/` gibt es immer.
fn benutzerverzeichnis() -> PathBuf {
    pfade::benutzerverzeichnis().unwrap_or_else(|| PathBuf::from("/"))
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
