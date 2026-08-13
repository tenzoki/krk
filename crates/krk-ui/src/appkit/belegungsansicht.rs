//! Die Belegungsansicht aus C3: jede Funktion mit ihrer Belegung, aenderbar.
//!
//! Ein Blatt am Hauptfenster, gebaut ueber [`super::blaetter::Blatt`] wie die
//! Dialoge aus C2 und C4. Darin eine Tabelle mit zwei Spalten — Funktion und
//! Belegung —, zwei Schaltflaechen fuer das Zuweisen und das Zuruecksetzen und
//! eine Meldungszeile fuer Bestaetigung, Konflikt und Abweisung. Der Inhalt
//! kommt vollstaendig aus [`crate::belegungsmodell`]; dieses Modul zeigt an
//! und haelt keine eigene Tabelle der Funktionen.
//!
//! Die Zeilen sind nach Funktionsbereichen gegliedert (Nutzerauftrag vom
//! 260806): vor den Funktionen eines Bereichs steht seine Ueberschrift als
//! Gruppenzeile der Tabelle. Welche Zeile eine Ueberschrift ist, weiss allein
//! das Modell ([`Belegungsmodell::ueberschrift`]); dieses Modul meldet es der
//! Tabelle ueber `tableView:isGroupRow:`, verweigert dafuer die Auswahl ueber
//! `tableView:shouldSelectRow:` — die Pfeiltasten ueberspringen eine solche
//! Zeile von selbst — und setzt die Anfangsauswahl auf die erste
//! Funktionszeile, weil die Zeile 0 seither eine Ueberschrift ist. Die
//! Bedienung aus S20 bleibt unveraendert.
//!
//! ```text
//! F1 ──> Kommando::BelegungAnsehen ──> zeigen(Blatt mit Tabelle)
//!                                          │ "Zuweisen" (Cmd+T): Aufnahme an
//!  Ereignisabgriff ──Faenger, 1. Station──> tastendruck_aufnehmen
//!                   └─────────2. Station──> suchzeichen_aufnehmen
//!                                           suchzeichen_wegnehmen
//!                                           zum_naechsten_treffer
//!                                          │ "Fertig" (Cmd+Eingabe) / esc
//!                                     verlassen ──> sichern, Menue, Abgriff
//! ```
//!
//! # Die Aufnahme laeuft ueber den einen Ereignisabgriff
//!
//! Die Zuweisung durch Druecken braucht den rohen Tastendruck, bevor die
//! Belegung ihn nachschlaegt. Diese Ansicht faengt ihn trotzdem nicht selbst:
//! sie haelt nur das Kennzeichen [`Belegungsquelle::nimmt_auf`], und der
//! Faenger im Ereignisabgriff fragt es ab und reicht den Druck an
//! [`Belegungsquelle::tastendruck_aufnehmen`]. Eine eigene
//! `keyDown:`-Behandlung bekaeme sonst genau die Ansicht, deren Zweck es ist,
//! dass es so etwas nicht gibt.
//!
//! # Die Suche laeuft ueber denselben Faenger, als seine zweite Station
//!
//! Seit der Runde 7 sucht jedes getippte Zeichen in dieser Ansicht (C1). Die
//! Rechnung dazu steht ohne AppKit in
//! [`Suchlage`](crate::belegungsmodell::Suchlage); dieses Modul haelt einen
//! Wert davon, gibt ihm die drei Ereignisse weiter und zeigt danach seine
//! Zielzeile und seine Meldung an. Auch dafuer gibt es keine eigene
//! `keyDown:`-Behandlung: der Faenger bekommt eine **zweite Station** hinter
//! der Aufnahme, und diese Reihenfolge **ist** der Vorrang aus C1.15. Eine
//! dritte Regel daneben entsteht nicht.
//!
//! **Die eingebaute Tippauswahl der `NSTableView` ist abgeschaltet**
//! (`setAllowsTypeSelect:` mit `false`, C1.11). Sie ist die zweite Suche mit
//! zweiten Regeln: sie vergleicht am Wortanfang, kennt keine Ruecktaste und
//! setzt sich nach einer Pause zurueck. Zwei Suchen in einer Ansicht waeren
//! zwei Wahrheiten darueber, was ein Treffer ist.
//!
//! # Die Tastatur bedient das Blatt ohne die Belegung
//!
//! Solange das Blatt steht, fuehrt der Ereignisabgriff nichts aus (allein der
//! Abbruch kommt durch) und reicht jeden Tastendruck an AppKit weiter. Die
//! Bedienung haengt deshalb an den Tastenentsprechungen des Blattes: die
//! Pfeile bewegen die Auswahl der Tabelle, und die drei Schaltflaechen liegen
//! auf den drei Kombinationen aus [`SCHALTFLAECHEN`]. **Keine davon ist ein
//! blosses Zeichen**, und das ist seit der Runde 7 die Bedingung dafuer, dass
//! die Suche jedes Zeichen bekommt: bis dahin lag "Zuweisen" auf der Leertaste
//! und "Fertig" auf der blossen Eingabetaste, und beide sind Eingaben der
//! Suche. Der Datensatz dazu ist
//! `shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`,
//! Moeglichkeit 1.
//!
//! `esc` geht als Befehl `abbrechen` durch den Abgriff und schliesst das Blatt
//! ueber denselben Griff wie jede andere Rueckfrage; waehrend der Aufnahme
//! faengt der Faenger es vorher ab und bricht nur die Aufnahme ab. **Eine
//! dritte Bedeutung "Suchtext loeschen" bekommt es nicht** (C1.13); dafuer ist
//! die Ruecktaste da. Sein Zeichen ist ein Steuerzeichen und faellt damit durch
//! die Aufnahmeregel der Suche, ohne dass die zweite Station es eigens
//! ausnehmen muesste.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`, `NSTextField`,
//! `NSButton`, `NSFont`, `NSWindow`, `NSIndexSet` und `NSString` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso die drei bedienten Protokolle
//! `NSTableViewDataSource`, `NSTableViewDelegate` und
//! `NSControlTextEditingDelegate`. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`).
//!
//! **Sechs Beruehrungen sind juenger als ihre Klasse, und alle liegen unter
//! dem Zielsystem:**
//!
//! - `allowsTypeSelect` seit 10.5 (`NSTableView.h:373`)
//! - `tableView:isGroupRow:` seit 10.5 (`NSTableView.h:685`)
//! - `tableView:viewForTableColumn:row:` seit 10.7 (`NSTableView.h:593`)
//! - `labelWithString:` und `wrappingLabelWithString:` seit 10.12
//!   (`NSTextField.h:93` und `:100`)
//! - `buttonWithTitle:target:action:` seit 10.12 (`NSButton.h:41`)
//! - `setUsesAutomaticRowHeights:` seit 10.13 (`NSTableView.h:574`) — die
//!   hoechste Untergrenze dieser Datei
//!
//! **`NSEventModifierFlags::Command` sieht juenger aus, als es ist.** Die
//! Schreibweise `NSEventModifierFlagCommand` traegt keine Angabe und steht
//! seit 10.0 (`NSEvent.h:172`); mit 10.12 abgekuendigt ist allein der alte
//! Name `NSCommandKeyMask` (`NSEvent.h:185`), den diese Datei nicht anspricht.
//! Ebenso ohne eigene Angabe und damit seit 10.0: `setKeyEquivalent:`,
//! `setKeyEquivalentModifierMask:`, `selectRowIndexes:byExtendingSelection:`,
//! `scrollRowToVisible:`, `systemFontOfSize:`, `boldSystemFontOfSize:` und
//! `smallSystemFontSize`.
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::{Cell, RefCell};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSControlTextEditingDelegate, NSEventModifierFlags,
    NSFont, NSScrollView, NSTableColumn, NSTableView, NSTableViewDataSource, NSTableViewDelegate,
    NSTextField, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString, ns_string,
};

use krk_core::tasten::{Tastendruck, code_von_pflicht};

use crate::belegungsmodell::{Belegungsmodell, Suchlage, Zuweisung};

use super::blaetter::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Die Hoehe einer Zeile in Punkten, wie in der Dateiliste und der Leiste.
const ZEILENHOEHE: f64 = 20.0;

/// Die Breite der Beigabe in Punkten.
const BREITE: f64 = 560.0;

/// Die Hoehe der Tabelle in Punkten. Sie fasst fuenfzehn Zeilen zu je
/// [`ZEILENHOEHE`]; die Belegung ist mit ihren Funktionen und den neun
/// Bereichsueberschriften laenger und braucht deshalb einen Rollbalken. Die
/// Zahl hier bestimmt nur, wie viele Zeilen ohne Rollen sichtbar sind, nicht
/// wie viele es gibt.
///
/// Die Zahl der Funktionen stand hier bis zum 260807 als feste Zahl und lief
/// mit dem ersten Nachtrag an `resources/default-keymap.toml` auseinander
/// (`issues/260807-1015_*_der-kommentar-zur-tabellenhoehe-nennt-57-funktionen-und-die-belegung-fuehrt-58.md`).
/// Sie steht nicht wieder hier, weil die Konstante nie an ihr hing.
const TABELLENHOEHE: f64 = 300.0;

/// Der Tastencode der Escape-Taste, aus der einen Tastentabelle des Kerns.
const CODE_ESC: u16 = code_von_pflicht("esc");

/// Eine Schaltflaeche der Ansicht mit ihrer Tastenentsprechung.
///
/// Die Angaben stehen als Werte da und nicht als Satz, damit die
/// Erlaeuterungszeile und die gesetzte Taste nicht auseinanderlaufen koennen.
#[derive(Debug, Clone, Copy)]
struct Schaltflaechentaste {
    /// Die Beschriftung, wie sie auf der Schaltflaeche steht.
    titel: &'static str,
    /// Ob die Befehlstaste dazugehoert. Fuer alle drei wahr, siehe
    /// [`SCHALTFLAECHEN`].
    mit_befehl: bool,
    /// Die Kombination in der Anzeigeform, fuer die Erlaeuterungszeile.
    anzeige: &'static str,
}

/// Die drei Schaltflaechen der Belegungsansicht mit ihren Tasten (C1.16).
///
/// **Eine Quelle, zwei Abnehmer.** [`zeigen`] setzt daraus die
/// Tastenentsprechungen, [`erlaeuterung`] schreibt daraus den Satz unter der
/// Ueberschrift des Blattes. Ein Satz, der eine andere Taste nennt als die
/// Schaltflaeche traegt, kann so nicht entstehen.
///
/// **Jede der drei traegt die Befehlstaste, und keine ist ein blosses
/// Zeichen.** Seit der Runde 7 geht jedes getippte Zeichen in die Suche; eine
/// Schaltflaeche auf der Leertaste oder auf der blossen Eingabetaste naehme ihr
/// die Eingabe wieder weg, und ein mehrwortiger Funktionsname waere nicht mehr
/// zu suchen. Bis dahin lag "Zuweisen" auf der Leertaste und "Fertig" auf der
/// Eingabetaste. Die Probe `keine_schaltflaeche_liegt_auf_einem_blossen_zeichen`
/// haelt es fest.
///
/// "Fertig" gehoert dem Blatt und nicht dieser Beigabe; seine Taste kommt
/// deshalb ueber [`Taste::EingabeMitBefehl`] und nicht ueber
/// `setKeyEquivalent`. Der Eintrag steht hier trotzdem, weil die
/// Erlaeuterungszeile alle drei nennen muss.
const SCHALTFLAECHEN: [Schaltflaechentaste; 3] = [
    Schaltflaechentaste {
        titel: "Zuweisen",
        mit_befehl: true,
        anzeige: "Cmd+T",
    },
    Schaltflaechentaste {
        titel: "Auslieferungszustand",
        mit_befehl: true,
        anzeige: "Cmd+R",
    },
    Schaltflaechentaste {
        titel: "Fertig",
        mit_befehl: true,
        anzeige: "Cmd+Eingabe",
    },
];

/// Die Schaltflaeche "Zuweisen", als Stelle in [`SCHALTFLAECHEN`].
const ZUWEISEN: usize = 0;
/// Die Schaltflaeche "Auslieferungszustand".
const ZURUECKSETZEN: usize = 1;
/// Die Schaltflaeche "Fertig".
const FERTIG: usize = 2;

/// Der Satz unter der Ueberschrift des Blattes (C1.16).
///
/// Er nennt die drei Kombinationen aus [`SCHALTFLAECHEN`] und die Suche. Weil
/// er sie von dort liest, kann er keine Taste nennen, die eine Schaltflaeche
/// nicht traegt.
fn erlaeuterung() -> String {
    format!(
        "Jedes getippte Zeichen sucht in beiden Spalten und springt auf den ersten \
         Treffer; die Eingabetaste geht zum nächsten, die Rücktaste kürzt den \
         Suchtext. Pfeiltasten wählen die Funktion. {} ({}) nimmt die nächste \
         gedrückte Kombination auf; esc bricht die Aufnahme ab. {} ({}) setzt \
         alles zurück. {} ({}) oder esc verlässt die Ansicht und sichert die \
         Änderungen.",
        SCHALTFLAECHEN[ZUWEISEN].titel,
        SCHALTFLAECHEN[ZUWEISEN].anzeige,
        SCHALTFLAECHEN[ZURUECKSETZEN].titel,
        SCHALTFLAECHEN[ZURUECKSETZEN].anzeige,
        SCHALTFLAECHEN[FERTIG].titel,
        SCHALTFLAECHEN[FERTIG].anzeige,
    )
}

/// Was die Datenquelle der Belegungsansicht haelt.
pub struct BelegungsansichtIvars {
    /// Die Tabelle, der die Quelle Aenderungen meldet.
    ///
    /// `NSTableView` haelt Datenquelle und Delegierten nur schwach; die starke
    /// Richtung laeuft von hier nach dort.
    tabelle: Retained<NSTableView>,
    /// Die Meldungszeile unter den Schaltflaechen.
    meldung: Retained<NSTextField>,
    /// Die Arbeitskopie der Belegung.
    modell: RefCell<Belegungsmodell>,
    /// Wahr, solange der naechste Tastendruck eine Zuweisung ist.
    nimmt_auf: Cell<bool>,
    /// Der Stand der Suche (C1).
    ///
    /// Sie steht **neben** der Arbeitskopie und nicht in ihr: eine Suche
    /// aendert keine Belegung, sie waehlt eine Zeile aus. Sie lebt so lange wie
    /// die Ansicht, und eine Aufnahme laesst sie unberuehrt (C1.12).
    suche: RefCell<Suchlage>,
}

define_class!(
    /// Datenquelle, Delegierter der Tabelle und Ziel der Schaltflaechen in
    /// einem Objekt, wie bei der Leiste.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = BelegungsansichtIvars]
    pub struct Belegungsquelle;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Belegungsquelle {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for Belegungsquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            self.ivars().modell.borrow().zeilen() as NSInteger
        }
    }

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und hat nur wahlfreie Methoden; die Ansicht
    // bearbeitet keinen Text.
    unsafe impl NSControlTextEditingDelegate for Belegungsquelle {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for Belegungsquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method_id(tableView:viewForTableColumn:row:))]
        fn ansicht_fuer_zelle(
            &self,
            _tabelle: &NSTableView,
            spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            self.zellenansicht(spalte, zeile)
        }

        /// Eine Bereichsueberschrift ist eine Gruppenzeile: AppKit zeichnet
        /// sie ueber die ganze Breite und fragt ihre Ansicht ohne Spalte an.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:isGroupRow:))]
        fn ist_gruppenzeile(&self, _tabelle: &NSTableView, zeile: NSInteger) -> bool {
            self.ist_ueberschrift(zeile)
        }

        /// Eine Bereichsueberschrift ist nicht auswaehlbar; die Pfeiltasten
        /// ueberspringen sie damit von selbst, und keine Zuweisung kann ihr
        /// gelten.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:shouldSelectRow:))]
        fn darf_zeile_auswaehlen(&self, _tabelle: &NSTableView, zeile: NSInteger) -> bool {
            !self.ist_ueberschrift(zeile)
        }
    }

    impl Belegungsquelle {
        /// Die Schaltflaeche "Zuweisen": der naechste Tastendruck gehoert der
        /// ausgewaehlten Funktion.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Aktion: ein
        // Argument, der Absender.
        #[unsafe(method(zuweisenGedrueckt:))]
        fn zuweisen_gedrueckt(&self, _absender: Option<&AnyObject>) {
            let Some(zeile) = self.gewaehlte_zeile() else {
                self.melden("Erst eine Funktion auswählen, dann Zuweisen drücken.");
                return;
            };
            let name = self
                .ivars()
                .modell
                .borrow()
                .name(zeile)
                .map(str::to_owned)
                .unwrap_or_default();
            self.ivars().nimmt_auf.set(true);
            self.melden(&format!(
                "Jetzt die gewünschte Kombination für »{name}« drücken; esc bricht ab."
            ));
        }

        /// Die Schaltflaeche "Zuruecksetzen": die Arbeitskopie zurueck auf den
        /// Auslieferungszustand (C3).
        // SAFETY: Die Signatur ist die einer gewoehnlichen Aktion: ein
        // Argument, der Absender.
        #[unsafe(method(zuruecksetzenGedrueckt:))]
        fn zuruecksetzen_gedrueckt(&self, _absender: Option<&AnyObject>) {
            self.ivars().nimmt_auf.set(false);
            self.ivars().modell.borrow_mut().zuruecksetzen();
            self.nachziehen();
            self.melden("Die Belegung ist auf den Auslieferungszustand zurückgesetzt.");
        }
    }
);

impl Belegungsquelle {
    /// Eine Datenquelle ueber der uebergebenen Arbeitskopie.
    fn neu(
        mtm: MainThreadMarker,
        tabelle: Retained<NSTableView>,
        meldung: Retained<NSTextField>,
        modell: Belegungsmodell,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(BelegungsansichtIvars {
            tabelle,
            meldung,
            modell: RefCell::new(modell),
            nimmt_auf: Cell::new(false),
            suche: RefCell::new(Suchlage::neu()),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Ob der naechste Tastendruck eine Zuweisung ist.
    ///
    /// Der Faenger des Ereignisabgriffs fragt das vor jedem Nachschlag ab.
    pub fn nimmt_auf(&self) -> bool {
        self.ivars().nimmt_auf.get()
    }

    /// Nimmt den gedrueckten Tastendruck als Zuweisung entgegen.
    ///
    /// Die Aufnahme endet mit diesem Druck, gleich wie er ausgeht: wer sich
    /// vertippt hat, drueckt Zuweisen erneut. Ein nacktes `esc` bricht ab,
    /// ohne etwas zu aendern; mit Zusatztaste ist es eine gewoehnliche
    /// Kombination und zuweisbar.
    pub fn tastendruck_aufnehmen(&self, druck: Tastendruck) {
        self.ivars().nimmt_auf.set(false);
        if druck.code == CODE_ESC && druck.maske.ist_leer() {
            self.melden("Die Aufnahme ist abgebrochen; die Belegung ist unverändert.");
            return;
        }
        let Some(zeile) = self.gewaehlte_zeile() else {
            self.melden("Es ist keine Funktion ausgewählt; die Belegung ist unverändert.");
            return;
        };
        let ergebnis = self.ivars().modell.borrow_mut().zuweisen(zeile, druck);
        match ergebnis {
            Zuweisung::Zugewiesen {
                funktion,
                kombination,
            } => {
                self.nachziehen();
                self.melden(&format!("»{funktion}« liegt jetzt auf {kombination}."));
            }
            // Satzzeichen und Zehnerblock: die Schreibweise kennt keinen
            // Namen, also entstuende eine Zeile in keymap.toml, die niemand
            // wieder einlesen kann. Die Ansicht sagt das, statt sie zu
            // schreiben (C3, S11b).
            Zuweisung::OhneNamen => self.melden(
                "Diese Taste hat in der Kombinationsschreibweise keinen Namen und lässt \
                 sich nicht ablegen; die Belegung ist unverändert.",
            ),
            // Der Konflikt samt Namen der anderen Funktion, woertlich aus dem
            // Kern (C3).
            Zuweisung::Abgelehnt(grund) => self.melden(&grund),
        }
    }

    /// Nimmt ein getipptes Zeichen in den Suchtext auf (C1.1).
    ///
    /// **Liefert, ob das Zeichen der Suche gehoerte**, und nur dann verbraucht
    /// die zweite Station des Faengers das Ereignis. Die Aufnahmeregel steht in
    /// [`Suchlage::zeichen_anhaengen`] und nirgends daneben; sie weist
    /// Steuerzeichen und den privaten Bereich ab, in dem AppKit die Funktions-
    /// und Pfeiltasten meldet. Damit laeuft `esc` weiter und verlaesst die
    /// Ansicht (C1.13), ohne dass der Faenger es eigens ausnehmen muesste.
    #[must_use]
    pub fn suchzeichen_aufnehmen(&self, zeichen: char) -> bool {
        let genommen = {
            let modell = self.ivars().modell.borrow();
            self.ivars()
                .suche
                .borrow_mut()
                .zeichen_anhaengen(zeichen, &modell)
        };
        if genommen {
            self.suche_zeigen();
        }
        genommen
    }

    /// Nimmt das letzte Zeichen des Suchtextes weg und sucht erneut (C1.8).
    ///
    /// Bei leerem Suchtext geschieht nichts, und zwar auch an der
    /// Meldungszeile nicht: eine Zuweisungsmeldung soll dort stehen bleiben,
    /// statt von einer leeren Suchmeldung ueberschrieben zu werden (C1.10,
    /// C1.17).
    pub fn suchzeichen_wegnehmen(&self) {
        let geaendert = {
            let modell = self.ivars().modell.borrow();
            self.ivars().suche.borrow_mut().letztes_zeichen_weg(&modell)
        };
        if geaendert {
            self.suche_zeigen();
        }
    }

    /// Geht auf das naechste Vorkommen des Suchtextes (C1.7).
    ///
    /// Ohne Treffer geschieht nichts; siehe [`Self::suchzeichen_wegnehmen`] zum
    /// Grund, aus dem dann auch die Meldungszeile stehen bleibt.
    pub fn zum_naechsten_treffer(&self) {
        if self.ivars().suche.borrow_mut().naechster_treffer() {
            self.suche_zeigen();
        }
    }

    /// Zieht Auswahl und Meldungszeile auf den Stand der Suche nach.
    ///
    /// Ohne Treffer bleibt die Auswahl stehen, und allein die Meldungszeile
    /// sagt es (C1.9). Die Zeile ist die vorhandene des Blattes; eine zweite
    /// Meldeflaeche entsteht nicht (C1.10).
    fn suche_zeigen(&self) {
        let (zielzeile, meldung) = {
            let suche = self.ivars().suche.borrow();
            (suche.zielzeile(), suche.meldung())
        };
        if let Some(zeile) = zielzeile {
            self.auswahl_setzen(zeile);
        }
        self.melden(&meldung);
    }

    /// Gibt die Arbeitskopie ab, fuer das Sichern beim Verlassen.
    ///
    /// Zurueck bleibt der Auslieferungszustand ohne Aenderungskennzeichen; er
    /// wird nie mehr angezeigt, weil das Blatt beim Aufruf schon geschlossen
    /// ist.
    pub fn modell_abgeben(&self) -> Belegungsmodell {
        self.ivars().modell.take()
    }

    /// Die ausgewaehlte Zeile der Tabelle.
    fn gewaehlte_zeile(&self) -> Option<usize> {
        usize::try_from(self.ivars().tabelle.selectedRow()).ok()
    }

    /// Ob diese Zeile eine Bereichsueberschrift ist.
    fn ist_ueberschrift(&self, zeile: NSInteger) -> bool {
        usize::try_from(zeile)
            .ok()
            .is_some_and(|stelle| self.ivars().modell.borrow().ueberschrift(stelle).is_some())
    }

    /// Schreibt den Stand des Modells in die Tabelle, ohne die Auswahl zu
    /// verlieren.
    ///
    /// `reloadData` nimmt der `NSTableView` ihre Auswahl; dieselbe Vorkehrung
    /// wie in der Leiste.
    ///
    /// **Wiederhergestellt wird nur eine waehlbare Zeile.**
    /// `selectRowIndexes:byExtendingSelection:` fragt
    /// `tableView:shouldSelectRow:` **nicht**; die Sperre fuer
    /// Ueberschriftszeilen weiter oben greift allein fuer Maus und Tastatur.
    /// Nach dem Zuruecksetzen aus C3 baut das Modell seine Zeilenliste neu,
    /// und dass die Ueberschriften danach an denselben Stellen stehen, haengt
    /// an einer Zusage aus einer anderen Kiste: das Einlesen einer Belegung
    /// weist unbekannte Kennungen ab und ergaenzt fehlende, also ist der
    /// Funktionsbestand immer der der Auslieferung. Der Umweg ueber
    /// [`Belegungsmodell::waehlbare_zeile`] macht die Absicherung an der
    /// Aufrufstelle selbst fest, damit die Auswahl nicht auf einer
    /// Ueberschrift landen kann und "Zuweisen" keine Aufforderung mit leerem
    /// Funktionsnamen stellt.
    /// **Die Suche wird dabei neu gerechnet, und die Meldungszeile nicht
    /// angefasst.** Eine Zuweisung und ein Zuruecksetzen aendern die Spalte
    /// "Belegung", ueber die mitgesucht wird; eine stehen gebliebene
    /// Trefferliste zeigte danach auf Zeilen, die den Suchtext nicht mehr
    /// tragen. Die Meldung gehoert in dieser Spanne der Zuweisung und wird erst
    /// vom naechsten Suchzeichen abgeloest (C1.10).
    fn nachziehen(&self) {
        let auswahl = self
            .gewaehlte_zeile()
            .and_then(|zeile| self.ivars().modell.borrow().waehlbare_zeile(zeile));
        self.ivars().tabelle.reloadData();
        if let Some(zeile) = auswahl {
            self.auswahl_setzen(zeile);
        }
        let modell = self.ivars().modell.borrow();
        self.ivars().suche.borrow_mut().nachrechnen(&modell);
    }

    /// Setzt die Auswahl auf diese Zeile und rollt sie ins Bild.
    ///
    /// Die eine Stelle, die das tut: die Suche braucht sie ohne `reloadData`,
    /// [`Self::nachziehen`] mit. Zwei Fassungen daneben liefen bei der naechsten
    /// Aenderung an der Auswahl auseinander.
    fn auswahl_setzen(&self, zeile: usize) {
        let stelle = NSIndexSet::indexSetWithIndex(zeile);
        self.ivars()
            .tabelle
            .selectRowIndexes_byExtendingSelection(&stelle, false);
        self.ivars().tabelle.scrollRowToVisible(zeile as NSInteger);
    }

    /// Setzt die Meldungszeile.
    fn melden(&self, text: &str) {
        self.ivars()
            .meldung
            .setStringValue(&NSString::from_str(text));
    }

    /// Die beschriftete Ansicht fuer eine Zelle.
    ///
    /// Eine Gruppenzeile fragt AppKit ohne Spalte an; sie bekommt die
    /// Bereichsueberschrift des Modells, fett gesetzt. Jede andere Zelle
    /// traegt den Text ihrer Spalte wie zuvor.
    fn zellenansicht(
        &self,
        spalte: Option<&NSTableColumn>,
        zeile: NSInteger,
    ) -> Option<Retained<NSView>> {
        let mtm = self.mtm();
        let stelle = usize::try_from(zeile).ok()?;
        let (text, ist_ueberschrift) = {
            let modell = self.ivars().modell.borrow();
            match modell.ueberschrift(stelle) {
                Some(titel) => (titel.to_owned(), true),
                None => {
                    let kennung = spalte?.identifier();
                    let text = if &*kennung == ns_string!("funktion") {
                        modell.funktionstext(stelle)?
                    } else {
                        modell.tastentext(stelle)?
                    };
                    (text, false)
                }
            }
        };

        let beschriftung = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        let schrift = if ist_ueberschrift {
            NSFont::boldSystemFontOfSize(NSFont::smallSystemFontSize())
        } else {
            NSFont::systemFontOfSize(NSFont::smallSystemFontSize())
        };
        beschriftung.setFont(Some(&schrift));
        beschriftung.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(0.0, ZEILENHOEHE)));
        beschriftung.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);

        let zelle = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(0.0, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        zelle.addSubview(&beschriftung);
        Some(zelle)
    }
}

/// Legt die Tastenentsprechung einer Schaltflaeche aus ihrem Eintrag in
/// [`SCHALTFLAECHEN`] fest.
///
/// Die Zusatztaste kommt aus dem Eintrag und nicht aus der Aufrufstelle, damit
/// die Zusage aus C1.16 an einem Wert haengt, den die Probe lesen kann, und
/// nicht an zwei Zeilen, die einzeln zu vergessen waeren.
fn taste_setzen(knopf: &NSButton, zeichen: &NSString, angabe: Schaltflaechentaste) {
    knopf.setKeyEquivalent(zeichen);
    if angabe.mit_befehl {
        knopf.setKeyEquivalentModifierMask(NSEventModifierFlags::Command);
    }
}

/// Zeigt die Belegungsansicht als Blatt am Fenster.
///
/// `verlassen` laeuft genau einmal, sobald das Blatt zu ist — ueber "Fertig"
/// wie ueber den Abbruchbefehl. Der Aufrufer holt sich dann die Arbeitskopie
/// ueber [`Belegungsquelle::modell_abgeben`] und sichert sie. Der
/// [`Blattgriff`] gehoert in `offenes_blatt`, damit `esc` das Blatt wie jede
/// Rueckfrage schliesst.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    modell: Belegungsmodell,
    verlassen: impl Fn() + 'static,
) -> (Retained<Belegungsquelle>, Blattgriff) {
    let rahmen = NSRect::new(NSPoint::ZERO, NSSize::ZERO);
    let tabelle = NSTableView::initWithFrame(NSTableView::alloc(mtm), rahmen);
    tabelle.setRowHeight(ZEILENHOEHE);
    tabelle.setUsesAutomaticRowHeights(false);
    tabelle.setAllowsMultipleSelection(false);
    // Die eingebaute Tippauswahl aus, damit die Ansicht genau eine Suche fuehrt
    // (C1.11). Siehe den Modulkopf: sie vergleicht am Wortanfang, kennt keine
    // Ruecktaste und setzt sich nach einer Pause zurueck, also drei Regeln
    // gegen die der Suche aus C1.
    tabelle.setAllowsTypeSelect(false);
    // Ohne Auswahl gaebe es keine Funktion, der die Zuweisung gelten koennte;
    // ausgewaehlt ist deshalb von Anfang an eine Zeile — unten ausdruecklich
    // die erste Funktionszeile, denn die Zeile 0 ist eine Ueberschrift.
    tabelle.setAllowsEmptySelection(false);

    for (kennung, titel, breite) in [
        (ns_string!("funktion"), ns_string!("Funktion"), 300.0),
        (ns_string!("belegung"), ns_string!("Belegung"), 220.0),
    ] {
        let spalte = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), kennung);
        spalte.setTitle(titel);
        spalte.setWidth(breite);
        tabelle.addTableColumn(&spalte);
    }

    let bildlauf = NSScrollView::initWithFrame(
        NSScrollView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 84.0), NSSize::new(BREITE, TABELLENHOEHE)),
    );
    bildlauf.setHasVerticalScroller(true);
    bildlauf.setAutohidesScrollers(true);
    bildlauf.setDocumentView(Some(&tabelle));

    // Die Meldungszeile: umbrechend, damit auch die Konfliktmeldung mit zwei
    // Funktionsnamen vollstaendig lesbar ist.
    let meldung = NSTextField::wrappingLabelWithString(ns_string!(""), mtm);
    meldung.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    meldung.setSelectable(false);
    meldung.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, 40.0)));

    let erste_funktionszeile = modell.erste_funktionszeile();
    let quelle = Belegungsquelle::neu(mtm, tabelle.clone(), meldung.clone(), modell);
    // SAFETY: Die Quelle beantwortet beide Protokolle, die sie oben
    // implementiert. Ueber die Lebensdauer verlangt die Bindung nichts;
    // `dataSource` und `delegate` sind nullende schwache Eigenschaften, und
    // der Anwendungsdelegierte haelt die Quelle, solange das Blatt steht.
    unsafe {
        tabelle.setDataSource(Some(ProtocolObject::from_ref(&*quelle)));
        tabelle.setDelegate(Some(ProtocolObject::from_ref(&*quelle)));
    }
    tabelle.reloadData();
    // Die Anfangsauswahl auf der ersten Funktionszeile: die Zeile 0 ist eine
    // Bereichsueberschrift und nicht auswaehlbar.
    if let Some(zeile) = erste_funktionszeile {
        let stelle = NSIndexSet::indexSetWithIndex(zeile);
        tabelle.selectRowIndexes_byExtendingSelection(&stelle, false);
    }

    // Die beiden Schaltflaechen. `NSControl` haelt sein Ziel schwach; die
    // Quelle lebt beim Anwendungsdelegierten, solange das Blatt steht.
    // SAFETY: `quelle` beantwortet die beiden Selektoren mit der ueblichen
    // Aktionssignatur, und `sel!` liefert gueltige Selektoren.
    let zuweisen = unsafe {
        NSButton::buttonWithTitle_target_action(
            &NSString::from_str(SCHALTFLAECHEN[ZUWEISEN].titel),
            Some(&*quelle),
            Some(sel!(zuweisenGedrueckt:)),
            mtm,
        )
    };
    // Cmd+T statt der Leertaste: die Leertaste ist ein Zeichen und gehoert
    // seit der Runde 7 der Suche. Siehe `SCHALTFLAECHEN`.
    taste_setzen(&zuweisen, ns_string!("t"), SCHALTFLAECHEN[ZUWEISEN]);
    zuweisen.setFrame(NSRect::new(
        NSPoint::new(0.0, 46.0),
        NSSize::new(160.0, 30.0),
    ));
    // SAFETY: wie bei `zuweisen`.
    let zuruecksetzen = unsafe {
        NSButton::buttonWithTitle_target_action(
            &NSString::from_str(SCHALTFLAECHEN[ZURUECKSETZEN].titel),
            Some(&*quelle),
            Some(sel!(zuruecksetzenGedrueckt:)),
            mtm,
        )
    };
    taste_setzen(
        &zuruecksetzen,
        ns_string!("r"),
        SCHALTFLAECHEN[ZURUECKSETZEN],
    );
    zuruecksetzen.setFrame(NSRect::new(
        NSPoint::new(170.0, 46.0),
        NSSize::new(220.0, 30.0),
    ));

    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, 84.0 + TABELLENHOEHE)),
    );
    beigabe.addSubview(&bildlauf);
    beigabe.addSubview(&zuweisen);
    beigabe.addSubview(&zuruecksetzen);
    beigabe.addSubview(&meldung);

    // "Fertig" auf Cmd+Eingabe statt auf der blossen Eingabetaste: die
    // Eingabetaste geht seit der Runde 7 zum naechsten Treffer der Suche.
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        "Tastaturbelegung",
        &[Schaltflaeche::neu(
            SCHALTFLAECHEN[FERTIG].titel,
            Taste::EingabeMitBefehl,
        )],
    );
    blatt.erlaeuterung_setzen(&erlaeuterung());
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&tabelle);

    let griff = blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| verlassen());
    (quelle, griff)
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::quelldateien;

    use super::*;

    /// Die Ansicht fuehrt genau eine Suche: die eingebaute Tippauswahl ist
    /// abgeschaltet (C1.11).
    ///
    /// **Gezaehlt wird der gesetzte Schalter im Quelltext**, wie in
    /// [`crate::quellbaum`] beschrieben. Eine Pruefung am Wert der Eigenschaft
    /// braeuchte eine gebaute `NSTableView` und damit den Hauptfaden, den
    /// `libtest` nicht hergibt; der offene Zustand aus `issues/260810-1001`
    /// soll durch diese Runde nicht wachsen. Was die Probe nicht sagt: ob
    /// AppKit den Schalter befolgt. Das entscheidet der Lauf am Buendel.
    ///
    /// Zwei Nadeln: das Abschalten steht genau einmal, und die Eigenschaft wird
    /// im ganzen Baum in keiner anderen Datei angefasst — ein zweiter Aufruf
    /// anderswo koennte sie wieder einschalten.
    #[test]
    fn die_eingebaute_tippauswahl_ist_abgeschaltet() {
        let abschalten = concat!("setAllowsType", "Select(false)");
        let eigenschaft = concat!("AllowsType", "Select");
        let dateien = quelldateien();

        let gesetzt: usize = dateien
            .iter()
            .map(|(_, inhalt)| inhalt.matches(abschalten).count())
            .sum();
        assert_eq!(
            gesetzt, 1,
            "die Tippauswahl wird nicht genau einmal abgeschaltet"
        );

        let angefasst: Vec<String> = dateien
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains(eigenschaft))
            .map(|(name, _)| name)
            .collect();
        assert_eq!(
            angefasst,
            vec!["krk-ui/src/appkit/belegungsansicht.rs".to_owned()],
            "die Tippauswahl wird ausserhalb dieser Datei angefasst"
        );
    }

    /// Keine der drei Schaltflaechen liegt auf einem blossen Zeichen (C1.16).
    ///
    /// Das ist die Bedingung dafuer, dass die Suche jedes getippte Zeichen
    /// bekommt: bis zur Runde 7 lag "Zuweisen" auf der Leertaste und "Fertig"
    /// auf der Eingabetaste, und beide sind Eingaben der Suche. Gelesen wird am
    /// Wert und nicht an einer Zeichenkette im Pruefcode.
    #[test]
    fn keine_schaltflaeche_liegt_auf_einem_blossen_zeichen() {
        for angabe in SCHALTFLAECHEN {
            assert!(
                angabe.mit_befehl,
                "»{}« traegt keine Zusatztaste und naehme der Suche ein Zeichen weg",
                angabe.titel
            );
        }
    }

    /// Die Erlaeuterungszeile nennt alle drei Kuerzel und die Suche (C1.16).
    ///
    /// Sie liest sie aus [`SCHALTFLAECHEN`], also aus derselben Quelle, aus der
    /// [`zeigen`] die Tastenentsprechungen setzt; ein Satz, der eine andere
    /// Taste nennt als die Schaltflaeche traegt, kann damit nicht entstehen.
    /// Die Probe haelt fest, dass er sie auch wirklich alle nennt.
    #[test]
    fn die_erlaeuterung_nennt_die_drei_kuerzel_und_die_suche() {
        let satz = erlaeuterung();
        for angabe in SCHALTFLAECHEN {
            assert!(
                satz.contains(angabe.titel),
                "die Erlaeuterung nennt »{}« nicht: {satz}",
                angabe.titel
            );
            assert!(
                satz.contains(angabe.anzeige),
                "die Erlaeuterung nennt {} nicht: {satz}",
                angabe.anzeige
            );
        }
        for genannt in ["Zeichen sucht", "Eingabetaste", "Rücktaste"] {
            assert!(
                satz.contains(genannt),
                "die Erlaeuterung nennt die Suche nicht ({genannt}): {satz}"
            );
        }
    }
}
