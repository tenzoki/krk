//! Das Blatt fuer das Umbenennen im Stapel: Regeleingabe und Vorschau (C4).
//!
//! ```text
//!  ┌ Suchen nach:    [____________________] ┐
//!  │ Ersetzen durch: [____________________] │  vier Eingabefelder
//!  │ Nummer ab: [__]  Stellen: [__]         ┘
//!  │ 50 Einträge, davon 2 mit einem Hinweis    ← Zusammenfassung oder Regelfehler
//!  │ ┌───────────┬───────────┬────────────┐
//!  │ │ Bisher    │ Neu       │ Hinweis    │   ← Vorschau, eine Zeile je
//!  │ └───────────┴───────────┴────────────┘      markiertem Eintrag
//!  └ [Abbrechen]                [Umbenennen]
//! ```
//!
//! **Dieses Blatt rechnet nichts.** Regelmodell, Vorschau und
//! Kollisionspruefung stehen in [`krk_core::umbenennen`] und sind dort ohne
//! Fenster pruefbar; hier steht allein, was AppKit betrifft. Das Blatt liegt
//! trotzdem in `appkit/`, aus demselben Grund wie die uebrigen: die Vorschau ist
//! eine `NSTableView` mit eigener Datenquelle, und eine Datenquelle entsteht nur
//! ueber `define_class!` mit `#[unsafe(method(...))]`.
//!
//! # Der zweite, ausdrueckliche Befehl
//!
//! C4 verlangt, dass erst ein zweiter Befehl ausfuehrt, was die Vorschau zeigt.
//! Der erste ist der Tastenbefehl, der dieses Blatt oeffnet; der zweite ist die
//! Schaltflaeche "Umbenennen", auf der die Eingabetaste liegt. Solange das Blatt
//! steht, ist nichts umbenannt: die Vorschau rechnet auf Zeichenketten.
//!
//! **Die Eingabetaste liegt auf "Umbenennen" und nicht auf "Abbrechen".** Das
//! ist der Unterschied zur Rueckfrage vor dem endgueltigen Loeschen, wo C4 die
//! Vorbelegung ausdruecklich umdreht: ein Umbenennen loescht nichts, und der
//! Nutzer hat die Vorschau vor sich, waehrend er die Taste drueckt.
//!
//! # Bedienung ohne Maus
//!
//! Der Tabulator laeuft durch die vier Felder und die Vorschau und wieder
//! zurueck; der Ring steht ausdruecklich da ([`schluesselring_legen`]), weil
//! AppKit ihn sonst aus der Anordnung der Ansichten raet. In der Vorschau
//! blaettern die Pfeiltasten, sobald sie den Fokus hat. Die Eingabetaste
//! benennt um, die Escape-Taste bricht ab, und beide wirken auch aus einem
//! Eingabefeld heraus: dafuer sorgt der Eingabewaechter der Huelle.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBorderType, NSColor, NSControlTextEditingDelegate, NSScrollView,
    NSTableColumn, NSTableView, NSTableViewDataSource, NSTableViewDelegate, NSTableViewStyle,
    NSTextAlignment, NSTextField, NSUserInterfaceItemIdentification, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSInteger, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
    ns_string,
};

use krk_core::umbenennen::{Regel, Vorschau, vorschau};

use super::Blatt;

/// Die Breite der Beigabe in Punkten.
///
/// Sie bestimmt zugleich die Breite des Blattes: `NSAlert` waechst mit seiner
/// Beigabe. Am laufenden Buendel gemessen am 260804-2033: bei 520 Punkten
/// schnitt die Zusammenfassungszeile ab.
const BREITE: f64 = 580.0;

/// Die Breite der Beschriftungsspalte links.
const BESCHRIFTUNG: f64 = 120.0;

/// Die Hoehe einer Eingabezeile.
const ZEILENHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen zwei Eingabezeilen.
const ZEILENABSTAND: f64 = 6.0;

/// Die Hoehe der Zeile mit der Zusammenfassung.
const HINWEISHOEHE: f64 = 17.0;

/// Die Hoehe der Vorschautabelle.
///
/// Rund acht Zeilen. Mehr machte das Blatt hoeher als das Fenster darunter;
/// wer weiter sehen will, blaettert.
const VORSCHAUHOEHE: f64 = 200.0;

/// Die Hoehe einer Zeile der Vorschau.
const VORSCHAUZEILE: f64 = 18.0;

/// Der Abstand zwischen den Bloecken der Beigabe.
const BLOCKABSTAND: f64 = 8.0;

/// Die Breite des Feldes fuer den Startwert der Nummerierung.
const NUMMERBREITE: f64 = 70.0;

/// Die Breite der Beschriftung "Stellen:".
const STELLENBESCHRIFTUNG: f64 = 60.0;

/// Die Breite des Feldes fuer die Stellenzahl.
const STELLENBREITE: f64 = 50.0;

/// Die drei Spalten der Vorschau.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spalte {
    /// Wie der Eintrag heisst.
    Alt,
    /// Wie er nach der Regel hiesse.
    Neu,
    /// Warum er so nicht heissen kann.
    Grund,
}

impl Spalte {
    /// Alle drei, in der Reihenfolge, in der sie stehen.
    const ALLE: [Spalte; 3] = [Spalte::Alt, Spalte::Neu, Spalte::Grund];

    /// Die Kennung, unter der AppKit die Spalte fuehrt.
    fn kennung(self) -> &'static NSString {
        match self {
            Spalte::Alt => ns_string!("alt"),
            Spalte::Neu => ns_string!("neu"),
            Spalte::Grund => ns_string!("grund"),
        }
    }

    /// Die Ueberschrift, die der Nutzer liest.
    fn titel(self) -> &'static NSString {
        match self {
            Spalte::Alt => ns_string!("Bisher"),
            Spalte::Neu => ns_string!("Neu"),
            Spalte::Grund => ns_string!("Hinweis"),
        }
    }

    /// Die Breite in Punkten.
    ///
    /// Die Spalte mit dem Grund ist die breiteste, und das ist Absicht: ein
    /// abgeschnittener Grund nennt den Grund nicht. Die beiden Namensspalten
    /// vertragen den Schnitt, weil derselbe Name daneben in der Dateiliste
    /// steht.
    fn breite(self) -> f64 {
        match self {
            Spalte::Alt | Spalte::Neu => 155.0,
            Spalte::Grund => 240.0,
        }
    }

    /// Die Spalte zu einer Kennung.
    fn aus_kennung(kennung: &NSString) -> Option<Spalte> {
        Spalte::ALLE
            .into_iter()
            .find(|spalte| kennung == spalte.kennung())
    }
}

/// Was die Datenquelle der Vorschau haelt.
pub struct VorschauIvars {
    /// Die Tabelle, der die Quelle Aenderungen meldet.
    ///
    /// `NSTableView` haelt Datenquelle und Delegierten schwach; die starke
    /// Richtung laeuft deshalb von hier nach dort.
    tabelle: Retained<NSTableView>,
    /// Die Zeile ueber der Vorschau: die Zusammenfassung oder der Grund, aus
    /// dem sich aus den Feldern keine Regel bauen laesst.
    hinweis: Retained<NSTextField>,
    /// Die vier Eingabefelder, aus denen die Regel entsteht.
    felder: Regelfelder,
    /// Die markierten Eintraege in Sichtreihenfolge.
    ///
    /// Sie bestimmen zugleich die Reihenfolge der fortlaufenden Nummer.
    markierte: Vec<String>,
    /// Alle Namen des Ordners, auch die ausgeblendeten.
    bestand: Vec<String>,
    /// Was gerade in der Vorschau steht.
    stand: RefCell<Vorschau>,
}

/// Die vier Eingabefelder eines Regelblattes.
struct Regelfelder {
    suchen: Retained<NSTextField>,
    ersetzen: Retained<NSTextField>,
    nummer_ab: Retained<NSTextField>,
    stellen: Retained<NSTextField>,
}

impl Regelfelder {
    /// Die Regel, die gerade in den Feldern steht.
    fn regel(&self) -> Result<Regel, krk_core::umbenennen::Regelfehler> {
        Regel::aus_eingabe(
            &self.suchen.stringValue().to_string(),
            &self.ersetzen.stringValue().to_string(),
            &self.nummer_ab.stringValue().to_string(),
            &self.stellen.stringValue().to_string(),
        )
    }
}

define_class!(
    /// Die Datenquelle der Vorschautabelle.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = VorschauIvars]
    pub struct Vorschauquelle;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Vorschauquelle {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for Vorschauquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            self.ivars().stand.borrow().zeilen().len() as NSInteger
        }
    }

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und hat nur wahlfreie Methoden.
    unsafe impl NSControlTextEditingDelegate for Vorschauquelle {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for Vorschauquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method_id(tableView:viewForTableColumn:row:))]
        fn ansicht_fuer_zelle(
            &self,
            tabelle: &NSTableView,
            spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            // Der Rumpf steht in `zellenansicht`, weil `define_class!` den
            // Rueckgabetyp umschreibt und der Fragezeichenoperator hier
            // deshalb nicht greift.
            self.zellenansicht(tabelle, spalte, zeile)
        }
    }
);

impl Vorschauquelle {
    /// Eine Quelle fuer diese Eintraege, mit noch leerer Vorschau.
    fn neu(
        mtm: MainThreadMarker,
        tabelle: Retained<NSTableView>,
        hinweis: Retained<NSTextField>,
        felder: Regelfelder,
        markierte: Vec<String>,
        bestand: Vec<String>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(VorschauIvars {
            tabelle,
            hinweis,
            felder,
            markierte,
            bestand,
            stand: RefCell::new(Vorschau::default()),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Rechnet die Vorschau aus den Feldern neu und zeichnet sie.
    ///
    /// **Die eine Stelle, an der die Vorschau entsteht.** Sie laeuft beim
    /// Aufbau des Blattes und danach bei jedem getippten Zeichen; ein zweiter
    /// Rechenweg entsteht nicht. Ein unlesbarer Startwert leert die Vorschau
    /// und schreibt seinen Grund in die Hinweiszeile, statt still eine Regel
    /// ohne Nummerierung anzuwenden.
    fn neu_rechnen(&self) {
        let ivars = self.ivars();
        let (stand, hinweis) = match ivars.felder.regel() {
            Ok(regel) => {
                let stand = vorschau(&regel, &ivars.markierte, &ivars.bestand);
                let text = zusammenfassung(&stand);
                (stand, text)
            }
            Err(fehler) => (Vorschau::default(), fehler.to_string()),
        };
        *ivars.stand.borrow_mut() = stand;
        ivars.hinweis.setStringValue(&NSString::from_str(&hinweis));
        ivars.tabelle.reloadData();
    }

    /// Die Vorschau, wie sie gerade auf dem Schirm steht.
    ///
    /// Der Wert, den das Blatt bei "Umbenennen" zurueckgibt. Ausgefuehrt wird
    /// genau das, was der Nutzer gesehen hat: es gibt keinen zweiten
    /// Rechendurchgang zwischen dem Tastendruck und dem Dateisystem.
    fn ergebnis(&self) -> Vorschau {
        self.ivars().stand.borrow().clone()
    }

    /// Die beschriftete Ansicht fuer eine Zelle.
    fn zellenansicht(
        &self,
        tabelle: &NSTableView,
        spalte: Option<&NSTableColumn>,
        zeile: NSInteger,
    ) -> Option<Retained<NSView>> {
        let spalte = Spalte::aus_kennung(&spalte?.identifier())?;
        let zeile = usize::try_from(zeile).ok()?;
        let stand = self.ivars().stand.borrow();
        let eintrag = stand.zeilen().get(zeile)?;

        let text = match spalte {
            Spalte::Alt => eintrag.alt.clone(),
            Spalte::Neu => eintrag.neu.clone(),
            Spalte::Grund => eintrag
                .kollision
                .map_or_else(String::new, |kollision| kollision.grund().to_owned()),
        };
        let feld = self.feld(tabelle, spalte);
        feld.setStringValue(&NSString::from_str(&text));
        // Rot faerbt allein, was zurueckgehalten wird, und der Grund steht
        // daneben in Worten: eine Markierung, die nur an der Farbe zu erkennen
        // waere, ist als Defekt festgehalten. Die Farbe wird in **jedem**
        // Durchgang gesetzt, weil die Zellenansichten wiederverwendet sind.
        let auffaellig = eintrag.kollision.is_some() && spalte != Spalte::Alt;
        let farbe = if auffaellig {
            NSColor::systemRedColor()
        } else {
            NSColor::labelColor()
        };
        feld.setTextColor(Some(&farbe));
        Some(Retained::into_super(Retained::into_super(feld)))
    }

    /// Holt eine Zellenansicht aus dem Vorrat der Tabelle oder baut eine neue.
    fn feld(&self, tabelle: &NSTableView, spalte: Spalte) -> Retained<NSTextField> {
        let kennung = spalte.kennung();
        // SAFETY: `self` ist der Eigentuemer, den AppKit an eine neu geladene
        // Ansicht weiterreicht; die Kennung ist eine gueltige Zeichenkette.
        let vorrat = unsafe { tabelle.makeViewWithIdentifier_owner(kennung, Some(self)) };
        if let Some(gebraucht) = vorrat.and_then(|ansicht| ansicht.downcast::<NSTextField>().ok()) {
            return gebraucht;
        }
        let feld = NSTextField::labelWithString(ns_string!(""), self.mtm());
        feld.setIdentifier(Some(kennung));
        feld.setMaximumNumberOfLines(1);
        feld
    }
}

/// Zeigt das Blatt fuer das Umbenennen im Stapel und liefert die Vorschau, die
/// der Nutzer bestaetigt hat.
///
/// `markierte` sind die Namen der markierten Eintraege in Sichtreihenfolge,
/// `bestand` alle Namen des Ordners, auch die ausgeblendeten. `fertig` laeuft
/// auf dem Hauptfaden und nur dann, wenn der Nutzer "Umbenennen" gewaehlt hat;
/// beim Abbruch laeuft es gar nicht.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    markierte: Vec<String>,
    bestand: Vec<String>,
    fertig: impl Fn(Vorschau) + 'static,
) {
    let (beigabe, tabelle, hinweis, felder) = beigabe_bauen(mtm);

    let mut blatt = Blatt::neu(mtm, &frage(markierte.len()), "Umbenennen");
    blatt.erlaeuterung_setzen(
        "Die Vorschau zeigt, was der Befehl täte. Umbenannt wird erst mit Return; \
         Esc bricht ab. Einträge mit einem Hinweis bleiben stehen.",
    );
    for feld in [
        &felder.suchen,
        &felder.ersetzen,
        &felder.nummer_ab,
        &felder.stellen,
    ] {
        blatt.waechter_anhaengen(mtm, feld);
    }
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&felder.suchen);
    schluesselring_legen(&felder, &tabelle);

    let quelle = Vorschauquelle::neu(mtm, tabelle.clone(), hinweis, felder, markierte, bestand);
    // SAFETY: Die Quelle beantwortet beide Protokolle, die sie oben
    // implementiert. Die Tabelle haelt Datenquelle und Delegierten schwach
    // (`objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`, "This is a
    // weak property"); stark gehalten wird die Quelle vom Abschlussblock
    // unten, und der lebt, solange das Blatt steht.
    unsafe {
        tabelle.setDataSource(Some(ProtocolObject::from_ref(&*quelle)));
        tabelle.setDelegate(Some(ProtocolObject::from_ref(&*quelle)));
    }
    quelle.neu_rechnen();

    // Die Vorschau rechnet mit jedem getippten Zeichen neu. Der Rueckruf haelt
    // die Quelle **schwach**, sonst schloesse sich der Ring Quelle → Blatt →
    // Waechter → Rueckruf → Quelle.
    let schwach = objc2::rc::Weak::from_retained(&quelle);
    blatt.textaenderung_melden(Box::new(move || {
        if let Some(quelle) = schwach.load() {
            quelle.neu_rechnen();
        }
    }));

    blatt.zeigen(fenster, move |bestaetigt| {
        if bestaetigt {
            fertig(quelle.ergebnis());
        }
    });
}

/// Die Frage in der Kopfzeile des Blattes.
fn frage(eintraege: usize) -> String {
    match eintraege {
        1 => "Einen Eintrag umbenennen".to_owned(),
        zahl => format!("{zahl} Einträge im Stapel umbenennen"),
    }
}

/// Die Zeile ueber der Vorschau, wenn die Regel lesbar ist.
fn zusammenfassung(stand: &Vorschau) -> String {
    let zeilen = stand.zeilen().len();
    let kollisionen = stand.kollisionen();
    let umzubenennen = stand.auszufuehren().count();
    if kollisionen == 0 {
        return format!("{zeilen} Einträge, davon {umzubenennen} mit neuem Namen");
    }
    format!("{zeilen} Einträge: {umzubenennen} werden umbenannt, {kollisionen} bleiben stehen")
}

/// Baut die Beigabe: vier Eingabefelder, die Hinweiszeile und die Vorschau.
///
/// Die Ansichten bekommen feste Rahmen und keine Auslegeregeln. Die Beigabe
/// eines `NSAlert` waechst nicht mit dem Fenster; eine Anordnung, die sich
/// anpasste, haette hier nichts, woran sie sich anpassen koennte.
fn beigabe_bauen(
    mtm: MainThreadMarker,
) -> (
    Retained<NSView>,
    Retained<NSTableView>,
    Retained<NSTextField>,
    Regelfelder,
) {
    let hoehe = VORSCHAUHOEHE
        + BLOCKABSTAND
        + HINWEISHOEHE
        + BLOCKABSTAND
        + 3.0f64.mul_add(ZEILENHOEHE, 2.0 * ZEILENABSTAND);
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, hoehe)),
    );

    // Von unten nach oben, weil AppKit von unten nach oben misst.
    let (vorschau, tabelle) = vorschautabelle(mtm);
    beigabe.addSubview(&vorschau);

    let hinweis = NSTextField::labelWithString(ns_string!(""), mtm);
    hinweis.setFrame(NSRect::new(
        NSPoint::new(0.0, VORSCHAUHOEHE + BLOCKABSTAND),
        NSSize::new(BREITE, HINWEISHOEHE),
    ));
    hinweis.setMaximumNumberOfLines(1);
    beigabe.addSubview(&hinweis);

    let unterkante = VORSCHAUHOEHE + BLOCKABSTAND + HINWEISHOEHE + BLOCKABSTAND;
    let nummernzeile = unterkante;
    let ersetzenzeile = unterkante + ZEILENHOEHE + ZEILENABSTAND;
    let suchenzeile = unterkante + 2.0 * (ZEILENHOEHE + ZEILENABSTAND);

    let suchen = eingabezeile(
        mtm,
        &beigabe,
        "Suchen nach:",
        suchenzeile,
        BREITE - BESCHRIFTUNG,
    );
    let ersetzen = eingabezeile(
        mtm,
        &beigabe,
        "Ersetzen durch:",
        ersetzenzeile,
        BREITE - BESCHRIFTUNG,
    );
    let nummer_ab = eingabezeile(mtm, &beigabe, "Nummer ab:", nummernzeile, NUMMERBREITE);

    // Die Stellenzahl steht in derselben Zeile wie der Startwert: die beiden
    // gehoeren zusammen, und eine eigene Zeile machte das Blatt hoeher, ohne
    // etwas zu erklaeren.
    let stellenbeschriftung = NSTextField::labelWithString(ns_string!("Stellen:"), mtm);
    stellenbeschriftung.setFrame(NSRect::new(
        NSPoint::new(
            BESCHRIFTUNG + NUMMERBREITE + BLOCKABSTAND * 2.0,
            nummernzeile + 3.0,
        ),
        NSSize::new(STELLENBESCHRIFTUNG, HINWEISHOEHE),
    ));
    stellenbeschriftung.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&stellenbeschriftung);

    let stellen = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(
            NSPoint::new(
                BESCHRIFTUNG
                    + NUMMERBREITE
                    + BLOCKABSTAND * 2.0
                    + STELLENBESCHRIFTUNG
                    + BLOCKABSTAND,
                nummernzeile,
            ),
            NSSize::new(STELLENBREITE, ZEILENHOEHE),
        ),
    );
    beigabe.addSubview(&stellen);

    let felder = Regelfelder {
        suchen,
        ersetzen,
        nummer_ab,
        stellen,
    };
    (beigabe, tabelle, hinweis, felder)
}

/// Eine beschriftete Eingabezeile, in die Beigabe gehaengt.
fn eingabezeile(
    mtm: MainThreadMarker,
    beigabe: &NSView,
    beschriftung: &str,
    unterkante: f64,
    feldbreite: f64,
) -> Retained<NSTextField> {
    let text = NSTextField::labelWithString(&NSString::from_str(beschriftung), mtm);
    text.setFrame(NSRect::new(
        NSPoint::new(0.0, unterkante + 3.0),
        NSSize::new(BESCHRIFTUNG - BLOCKABSTAND, HINWEISHOEHE),
    ));
    text.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&text);

    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(
            NSPoint::new(BESCHRIFTUNG, unterkante),
            NSSize::new(feldbreite, ZEILENHOEHE),
        ),
    );
    beigabe.addSubview(&feld);
    feld
}

/// Baut die Vorschautabelle samt ihrer Bildlaufansicht.
fn vorschautabelle(mtm: MainThreadMarker) -> (Retained<NSScrollView>, Retained<NSTableView>) {
    let rahmen = NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, VORSCHAUHOEHE));
    let tabelle = NSTableView::initWithFrame(NSTableView::alloc(mtm), rahmen);
    tabelle.setRowHeight(VORSCHAUZEILE);
    tabelle.setUsesAutomaticRowHeights(false);
    tabelle.setUsesAlternatingRowBackgroundColors(true);
    tabelle.setStyle(NSTableViewStyle::FullWidth);
    for spalte in Spalte::ALLE {
        let kopf = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), spalte.kennung());
        kopf.setTitle(spalte.titel());
        kopf.setWidth(spalte.breite());
        tabelle.addTableColumn(&kopf);
    }

    let sicht = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    sicht.setHasVerticalScroller(true);
    sicht.setAutohidesScrollers(true);
    sicht.setBorderType(NSBorderType::BezelBorder);
    sicht.setDocumentView(Some(&tabelle));
    sicht.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    (sicht, tabelle)
}

/// Legt den Ring, den der Tabulator abgeht (C2, C4).
///
/// Er steht ausdruecklich da und wird nicht AppKit ueberlassen: die Anordnung
/// der Ansichten in der Beigabe ergibt eine andere Reihenfolge als die, in der
/// der Nutzer die Felder ausfuellt, und die Vorschau kaeme darin gar nicht vor.
/// Ohne den Ring waere das Blaettern durch die Vorschau ohne Maus nicht
/// moeglich, und genau das sagt C4 zu.
fn schluesselring_legen(felder: &Regelfelder, tabelle: &NSTableView) {
    // SAFETY: `setNextKeyView:` verlangt vom Nachfolger allein, dass er eine
    // Ansicht ist und lebt. Alle fuenf haengen in der Beigabe beziehungsweise
    // in ihrer Bildlaufansicht und leben, solange das Blatt steht; die Kette
    // ist geschlossen und verweist auf keine Ansicht ausserhalb.
    unsafe {
        felder.suchen.setNextKeyView(Some(&felder.ersetzen));
        felder.ersetzen.setNextKeyView(Some(&felder.nummer_ab));
        felder.nummer_ab.setNextKeyView(Some(&felder.stellen));
        felder.stellen.setNextKeyView(Some(tabelle));
        tabelle.setNextKeyView(Some(&felder.suchen));
    }
}
