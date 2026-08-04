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
//!   └─ Sitzungsschreiber    gebuendelt, hoechstens alle zwei Sekunden
//! ```
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
//! Der Ereignisabgriff kennt kein Dateifenster; er liefert ein [`Kommando`] an
//! [`Anwendungsdelegierter::kommando_ausfuehren`]. Der teilt auf: was das
//! Fenster als ganzes betrifft, bleibt hier, alles uebrige geht an die
//! Datenquelle des **aktiven** Dateifensters. Eine zweite Stelle, die
//! entscheidet, wohin ein Tastendruck geht, entsteht nicht.
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
use std::rc::Rc;
use std::time::Instant;

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
use krk_core::ablage::{Ablage, Datei, Fensterseite, Sitzung};
use krk_core::tasten::Kommando;
use krk_core::tasten::belegung;

use crate::fenstermodell::{BREITENSCHRITT, Bereich, Fenstermodell};
use crate::messmodus::{Anweisung, Aufgabe, Messlauf, Zustand};
use crate::tabs::Tabliste;

use super::aufteilung::Aufteilung;
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
    /// Das aktive Dateifenster, die Sichtbarkeit und die Breiten.
    modell: RefCell<Fenstermodell>,
    fenster: OnceCell<Retained<NSWindow>>,
    fenster_delegierter: OnceCell<Retained<FensterDelegierter>>,
    aufteilung: OnceCell<Aufteilung>,
    /// Die beiden Dateifenster, links zuerst.
    dateifenster: OnceCell<[Dateifenster; 2]>,
    tastenabgriff: OnceCell<Tastenabgriff>,
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
            sitzungsschreiber: RefCell::new(None),
            schreibfehler_gemeldet: Cell::new(false),
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
        }

        self.aufteilung_nachziehen();
        self.tastenabgriff_einrichten(&mut meldungen);
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
        let abgriff =
            Tastenabgriff::einrichten(belegung, self.ivars().tasten_protokoll, move |kommando| {
                match schwach.load() {
                    Some(selbst) => selbst.kommando_ausfuehren(kommando),
                    None => false,
                }
            });
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
                .ordner_lesen(&pfad);
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

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Liefert, ob es ausgefuehrt wurde; nur dann schluckt der Abgriff das
    /// Ereignis.
    fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        if self.ivars().dateifenster.get().is_none() {
            return false;
        }
        let ausgefuehrt = match kommando {
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
        self.ivars().modell.borrow_mut().umschalten(bereich)
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
