//! Die Belegungsansicht aus C3: jede Funktion mit ihrer Belegung, aenderbar.
//!
//! Ein Blatt am Hauptfenster, gebaut ueber [`super::blaetter::Blatt`] wie die
//! Dialoge aus C2 und C4. Darin eine Tabelle mit zwei Spalten — Funktion und
//! Belegung —, zwei Schaltflaechen fuer das Zuweisen und das Zuruecksetzen und
//! eine Meldungszeile fuer Bestaetigung, Konflikt und Abweisung. Der Inhalt
//! kommt vollstaendig aus [`crate::belegungsmodell`]; dieses Modul zeigt an
//! und haelt keine eigene Tabelle der Funktionen.
//!
//! ```text
//! F1 ──> Kommando::BelegungAnsehen ──> zeigen(Blatt mit Tabelle)
//!                                          │ "Zuweisen": Aufnahme an
//!  Ereignisabgriff ──Faenger──> tastendruck_aufnehmen ──> Belegungsmodell
//!                                          │ "Fertig" / esc
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
//! # Die Tastatur bedient das Blatt ohne die Belegung
//!
//! Solange das Blatt steht, fuehrt der Ereignisabgriff nichts aus (allein der
//! Abbruch kommt durch) und reicht jeden Tastendruck an AppKit weiter. Die
//! Bedienung haengt deshalb an den Tastenentsprechungen des Blattes: die
//! Pfeile bewegen die Auswahl der Tabelle, die Leertaste liegt auf "Zuweisen",
//! Cmd+R auf "Zuruecksetzen", die Eingabetaste auf "Fertig". `esc` geht als
//! Befehl `abbrechen` durch den Abgriff und schliesst das Blatt ueber
//! denselben Griff wie jede andere Rueckfrage; waehrend der Aufnahme faengt
//! der Faenger es vorher ab und bricht nur die Aufnahme ab.

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

use crate::belegungsmodell::{Belegungsmodell, Zuweisung};

use super::blaetter::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Die Hoehe einer Zeile in Punkten, wie in der Dateiliste und der Leiste.
const ZEILENHOEHE: f64 = 20.0;

/// Die Breite der Beigabe in Punkten.
const BREITE: f64 = 560.0;

/// Die Hoehe der Tabelle in Punkten. 57 Funktionen brauchen einen Rollbalken;
/// die Zahl hier bestimmt nur, wie viele Zeilen ohne Rollen sichtbar sind.
const TABELLENHOEHE: f64 = 300.0;

/// Der Tastencode der Escape-Taste, aus der einen Tastentabelle des Kerns.
const CODE_ESC: u16 = code_von_pflicht("esc");

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

    /// Schreibt den Stand des Modells in die Tabelle, ohne die Auswahl zu
    /// verlieren.
    ///
    /// `reloadData` nimmt der `NSTableView` ihre Auswahl; dieselbe Vorkehrung
    /// wie in der Leiste.
    fn nachziehen(&self) {
        let auswahl = self.gewaehlte_zeile();
        self.ivars().tabelle.reloadData();
        if let Some(zeile) = auswahl {
            let stelle = NSIndexSet::indexSetWithIndex(zeile);
            self.ivars()
                .tabelle
                .selectRowIndexes_byExtendingSelection(&stelle, false);
            self.ivars().tabelle.scrollRowToVisible(zeile as NSInteger);
        }
    }

    /// Setzt die Meldungszeile.
    fn melden(&self, text: &str) {
        self.ivars()
            .meldung
            .setStringValue(&NSString::from_str(text));
    }

    /// Die beschriftete Ansicht fuer eine Zelle.
    fn zellenansicht(
        &self,
        spalte: Option<&NSTableColumn>,
        zeile: NSInteger,
    ) -> Option<Retained<NSView>> {
        let mtm = self.mtm();
        let stelle = usize::try_from(zeile).ok()?;
        let kennung = spalte?.identifier();
        let text = {
            let modell = self.ivars().modell.borrow();
            if &*kennung == ns_string!("funktion") {
                modell.funktionstext(stelle)?
            } else {
                modell.tastentext(stelle)?
            }
        };

        let beschriftung = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        beschriftung.setFont(Some(&NSFont::systemFontOfSize(
            NSFont::smallSystemFontSize(),
        )));
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
    // Ohne Auswahl gaebe es keine Funktion, der die Zuweisung gelten koennte;
    // die erste Zeile ist deshalb von Anfang an ausgewaehlt.
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

    // Die beiden Schaltflaechen. `NSControl` haelt sein Ziel schwach; die
    // Quelle lebt beim Anwendungsdelegierten, solange das Blatt steht.
    // SAFETY: `quelle` beantwortet die beiden Selektoren mit der ueblichen
    // Aktionssignatur, und `sel!` liefert gueltige Selektoren.
    let zuweisen = unsafe {
        NSButton::buttonWithTitle_target_action(
            ns_string!("Zuweisen"),
            Some(&*quelle),
            Some(sel!(zuweisenGedrueckt:)),
            mtm,
        )
    };
    zuweisen.setKeyEquivalent(ns_string!(" "));
    zuweisen.setFrame(NSRect::new(
        NSPoint::new(0.0, 46.0),
        NSSize::new(160.0, 30.0),
    ));
    // SAFETY: wie bei `zuweisen`.
    let zuruecksetzen = unsafe {
        NSButton::buttonWithTitle_target_action(
            ns_string!("Auslieferungszustand"),
            Some(&*quelle),
            Some(sel!(zuruecksetzenGedrueckt:)),
            mtm,
        )
    };
    zuruecksetzen.setKeyEquivalent(ns_string!("r"));
    zuruecksetzen.setKeyEquivalentModifierMask(NSEventModifierFlags::Command);
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

    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        "Tastaturbelegung",
        &[Schaltflaeche::neu("Fertig", Taste::Eingabe)],
    );
    blatt.erlaeuterung_setzen(
        "Pfeiltasten wählen die Funktion. Zuweisen (Leertaste) nimmt die nächste \
         gedrückte Kombination auf; esc bricht die Aufnahme ab. \
         Auslieferungszustand (Cmd+R) setzt alles zurück. Fertig (Eingabetaste) \
         oder esc verlässt die Ansicht und sichert die Änderungen.",
    );
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&tabelle);

    let griff = blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| verlassen());
    (quelle, griff)
}
