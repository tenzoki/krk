//! Die Bereichsleiste am Fensterfuss: neun Ankreuzfelder, sonst nichts.
//!
//! Fuenf Schalter fuer die Bereiche der Fensterzeile (C2 der
//! Bereichsleisten-Runde), drei fuer die schaltbaren Spalten der Dateilisten
//! (C3) und seit der Filter-Runde einer fuer die tiefe Suche (C2.1). Jeder
//! zeigt, ob sein Gegenstand steht, und schickt bei
//! einem Klick **ein Kommando** los; ausgefuehrt wird es dort, wo auch der
//! Tastenbefehl ausgefuehrt wird. Einen zweiten Weg an den Pruefungen vorbei
//! gibt es nicht, und das ist die Zusage aus C2.2.
//!
//! ```text
//!  Klick ──> Leistenquelle ──Kommando──> Anwendungsdelegierter
//!              Selbstkippung                 kommando_ausfuehren
//!              zuruecknehmen                 aufteilung_nachziehen
//!                                            bereichsleiste_nachziehen
//!            Schalterzustaende <──────────────────┘
//! ```
//!
//! # Der neunte Schalter ist eine Gruppe und keine dritte Sammlung
//!
//! Die ersten beiden Gruppen sind Reihungen ueber eine Aufzaehlung —
//! [`Bereich::ALLE`] und die schaltbaren Werte von [`Spalte::ALLE`] —, und
//! ihre Schalter nennen sich beim Klick ueber ihre `tag`. Der Schalter der
//! tiefen Suche ist ein einzelnes Feld: "Deep" ist weder ein Bereich noch
//! eine Spalte, sondern die Sucheinstellung des sichtbaren Tabs. Eine
//! Aufzaehlung mit einem Wert daneben waere eine Aufzaehlung zu viel, und
//! eine `tag` braucht er nicht, weil sein Selektor sein Kommando schon kennt.
//!
//! **Er ist auch die eine Groesse dieser Leiste, die nicht fensterweit
//! gilt.** Die acht anderen stehen im
//! [`Fenstermodell`](crate::fenstermodell::Fenstermodell),
//! dieser steht am `Ordnermodell` des sichtbaren Tabs im **aktiven**
//! Dateifenster. Daran haengen die drei Anlaesse, die
//! `Anwendungsdelegierter::bereichsleiste_nachziehen` seit der Filter-Runde
//! zusaetzlich hat; welche das sind, steht dort. Solange
//! `decisions/260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`
//! offen ist, faehrt der Bau auf "je Tab": faellt die Antwort auf "je
//! Fenster", wechselt allein die **Quelle** des dritten Arguments von
//! [`Bereichsleiste::zustaende_setzen`], und diese Datei bleibt, wie sie ist.
//!
//! # Die Leiste zeigt an und haelt keinen Stand
//!
//! Ein Ankreuzfeld kippt seinen Zustand selbst, **bevor** seine Aktion laeuft.
//! Was danach auf dem Schalter steht, ist deshalb nicht die Antwort des
//! Modells, sondern die Vermutung von AppKit. `Leistenquelle::geklickt` nimmt
//! diese eine fremde Schreibung sofort zurueck, noch bevor das Kommando
//! gemeldet ist; damit bleibt das Modell die einzige Quelle jedes angezeigten
//! Zustands. Was der Befehl daran aendert, schreibt der Anwendungsdelegierte
//! ueber [`Bereichsleiste::zustaende_setzen`], den einen Schreiber — **einmal,
//! auf jedem Weg**. Ein abgewiesener Klick braucht dafuer keinen eigenen
//! Anlass mehr: er hat nichts hinterlassen, das zurueckspringen muesste, und
//! der Schalter steht schon wieder auf dem Stand des Modells (C2.4).
//!
//! Gelesen wird aus der Leiste allein in `selbstkippung_zuruecknehmen`, und
//! dort nicht der angezeigte Stand, sondern nur, was der Klick gerade
//! umgestellt hat.
//!
//! # Kein Schalter nimmt den Ersthelferrang an
//!
//! Jeder Schalter traegt `setRefusesFirstResponder(true)` (C1.4). Das ist
//! keine Vorsicht, sondern die tragende Entscheidung dieses Moduls, und der
//! Plan begruendet sie unter `## Warum die Leiste keinen sechsten Fokuswert
//! bekommt`: `Anwendungsdelegierter::ersthelferbereich` laeuft ueber
//! [`Bereich::ALLE`] und faellt auf `Fokus::Dateifenster` zurueck, wenn der
//! Ersthelfer in keinem der fuenf Teilbaeume liegt. Die Leiste liegt in
//! keinem — sie ist keine Unteransicht der `NSSplitView`, sondern ihre
//! Schwester unter der Inhaltsflaeche. Naehme ein Schalter den Rang an,
//! antwortete `fokus()` weiter "Dateifenster", waehrend die Tasten beim
//! Schalter ankaemen: eine falsche Auskunft ueber den Fokus, und die ist
//! teurer als ein Absturz. `Fokus` bekommt deshalb keinen sechsten Wert,
//! sondern der Fall wird ausgeschlossen.
//!
//! # Warum Ankreuzfelder
//!
//! [`super::belegungsansicht`] baut ihre beiden Schaltflaechen ueber
//! `buttonWithTitle:target:action:`, also ueber dieselbe Familie bequemer
//! Erzeuger; ein Ankreuzfeld ist deren zweistufiges Geschwister und zeigt
//! seinen Zustand von sich aus. Ein `NSSegmentedControl` waere kompakter,
//! fuehrte aber einen im Baum unbekannten Bedienelementtyp ein und stellte die
//! neun Schalter ueber eine Segmentnummer statt ueber eine gehaltene Ansicht
//! je Bereich zu.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSButton`, seine Oberklasse `NSControl`, dazu `NSFont`,
//! `NSObject` und `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `initWithFrame:`, `setFrame:`, `addSubview:`, `setAutoresizingMask:`,
//! `sizeToFit`, `setFont:`, `setTag:`, `tag`, `setState:`, `state`,
//! `setToolTip:`, `refusesFirstResponder` (`NSControl.h:30`, ohne eigene
//! Angabe), `systemFontOfSize:` und `smallSystemFontSize`; die drei Werte von
//! `NSControlStateValue` (`NSCell.h:71-74`) tragen ebenfalls keine Angabe.
//! Zwei Beruehrungen sind juenger als ihre Klasse: `controlSize` seit 10.10
//! (`NSControl.h:32`) und `checkboxWithTitle:target:action:` seit 10.12
//! (`NSButton.h:59`) — die hoechste Untergrenze dieser Datei. Das Buendel
//! zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS 15
//! hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSButton, NSControlSize, NSControlStateValueOff, NSControlStateValueOn, NSFont, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use krk_core::ablage::{Sichtbarkeit, Spaltensichtbarkeit};
use krk_core::tasten::Kommando;

use crate::fenstermodell::{Bereich, sichtbar_in, spalte_sichtbar_in};
use crate::spalten::Spalte;

use super::statuszeile;

/// Die Hoehe der Leiste in Punkten.
///
/// **Dieselbe Zahl wie eine Statuszeile, und deshalb dieselbe Konstante.**
/// Beide tragen eine Zeile in der kleinen Systemschrift mit etwas Luft darum;
/// eine zweite 18 daneben waere eine zweite Wahrheit ueber dieselbe Sache und
/// liefe beim ersten Nachjustieren auseinander. Kriterium C1.1 nennt die
/// Gleichheit ausdruecklich.
pub const HOEHE: f64 = statuszeile::HOEHE;

/// Der Abstand vom linken Fensterrand, wie ihn die Statuszeile haelt.
const EINZUG: f64 = statuszeile::EINZUG;

/// Der waagerechte Abstand zwischen zwei Schaltern derselben Gruppe.
const ABSTAND: f64 = 10.0;

/// Der Abstand zwischen den Bereichs- und den Spaltenschaltern.
///
/// Groesser als [`ABSTAND`], weil die beiden Gruppen verschiedene Dinge
/// schalten: die fuenf die Flaechen des Fensters, die drei die Spalten in zwei
/// von ihnen.
const GRUPPENABSTAND: f64 = 24.0;

/// Die Breite, mit der die Leiste aufgebaut wird.
///
/// Sie waechst mit dem Fenster; die Zahl hier gilt nur bis zum ersten
/// Auslegen. Dieselbe Rolle wie `AUFBAUGROESSE` in [`super::aufteilung`].
const AUFBAUBREITE: f64 = 1280.0;

/// Welches Kommando der Schalter dieses Bereichs schickt.
///
/// **Die eine Haelfte der Aufbautabelle**, und eine vollstaendige
/// Fallunterscheidung ohne Auffangzweig: ein sechster Bereich haelt hier den
/// Bau an und erzwingt die Antwort darauf, womit sein Schalter zu bedienen
/// waere. Ein Auffangzweig gaebe ihm still den Schalter eines anderen.
///
/// Alle fuenf tragen `Wirkungsbereich::Ueberall` (C2.6); die Probe
/// `jeder_schalter_wirkt_aus_jedem_fokus` haelt es fest.
const fn kommando_des_bereichs(bereich: Bereich) -> Kommando {
    match bereich {
        Bereich::Lesezeichen => Kommando::LeisteUmschalten,
        Bereich::Links => Kommando::ErstesFensterUmschalten,
        Bereich::Rechts => Kommando::ZweitesFensterUmschalten,
        Bereich::Vorschau => Kommando::VorschauUmschalten,
        Bereich::Editor => Kommando::EditorUmschalten,
    }
}

/// Welches Kommando der Schalter dieser Spalte schickt, falls sie einen hat.
///
/// **Die andere Haelfte der Aufbautabelle, und zugleich die eine Stelle, die
/// sagt, welche Spalten schaltbar sind.** `None` fuer die Namensspalte: eine
/// Dateiliste ohne sie zeigt nichts, was den Eintrag benennt (C3.1). Auch das
/// eine vollstaendige Fallunterscheidung ohne Auffangzweig — eine fuenfte
/// Spalte haelt den Bau an, statt still ohne Schalter zu bleiben.
const fn kommando_der_spalte(spalte: Spalte) -> Option<Kommando> {
    match spalte {
        Spalte::Name => None,
        Spalte::Groesse => Some(Kommando::SpalteGroesseUmschalten),
        Spalte::Geaendert => Some(Kommando::SpalteDatumUmschalten),
        Spalte::Typ => Some(Kommando::SpalteTypUmschalten),
    }
}

/// Das Kommando, das der Schalter der tiefen Suche schickt (C2.3).
///
/// **Eine Konstante und keine Aufbautabelle.** Die beiden Gruppen darueber
/// rechnen ihr Kommando aus einer Aufzaehlung aus, weil es mehrere sind; hier
/// ist es eines, und eine Aufzaehlung mit einem Wert waere eine Aufzaehlung zu
/// viel. Der Selektor `tiefeGedrueckt:` und die Probe lesen dieselbe
/// Konstante; eine Aufstellung daneben pruefte sich selbst.
const KOMMANDO_DER_TIEFE: Kommando = Kommando::TiefeSucheUmschalten;

/// Die Aufschrift des Schalters der tiefen Suche (C2.1).
///
/// **Englisch und nicht uebersetzt**, waehrend die acht anderen deutsch
/// beschriftet sind. Das ist ein Nutzerentscheid ueber die Anzeige und keiner
/// ueber die Kennung: die heisst `tiefe_suche_umschalten` und folgt der
/// Schreibweise der uebrigen Befehle. Die Konstante steht hier, damit
/// [`Bereichsleiste::bauen`] und die Probe dieselbe Zeichenkette lesen.
const AUFSCHRIFT_DER_TIEFE: &str = "Deep";

/// Die Stelle, an der der Schalter dieser Spalte in
/// [`Bereichsleiste::spaltenschalter`] steht.
///
/// **Abgeleitet und nicht hingeschrieben.** Die Reihenfolge der drei Schalter
/// ist die Reihenfolge der schaltbaren Werte in [`Spalte::ALLE`], und diese
/// Funktion rechnet sie aus derselben Quelle aus, aus der [`Bereichsleiste::bauen`]
/// sie baut. Eine Tabelle mit drei festen Zahlen daneben waere eine zweite
/// Aufzaehlung und liefe beim naechsten Nachtrag auseinander.
fn spaltenfach(gesucht: Spalte) -> Option<usize> {
    Spalte::ALLE
        .into_iter()
        .filter(|spalte| kommando_der_spalte(*spalte).is_some())
        .position(|spalte| spalte == gesucht)
}

/// Was ein Klick auf einen Schalter dem Anwendungsdelegierten meldet.
///
/// Ein eigener Name statt des ausgeschriebenen Typs, wie bei
/// [`super::editor::Ausgangsmelder`] und den drei Meldern in
/// [`super::tabelle`]: der Typ steht an zwei Stellen dieser Datei und in der
/// Signatur, die der Anwendungsdelegierte bedient.
///
/// **Ein Kommando und kein Schaltbefehl.** Die Leiste sagt nicht, was zu tun
/// ist, sondern welchen Befehl der Nutzer ausgeloest hat; ausgefuehrt wird er
/// auf demselben Weg wie der Tastendruck (C2.2).
pub type Kommandomelder = Box<dyn Fn(Kommando)>;

/// Was das Rueckrufziel der Leiste haelt.
pub struct LeistenquellenIvars {
    /// Der Melder, den [`Bereichsleiste::melder_setzen`] eintraegt.
    ///
    /// Er haelt den Anwendungsdelegierten **schwach**, wie die fuenf anderen
    /// Melder dieses Projekts; sonst schloesse sich der Ring Delegierter →
    /// Leiste → Quelle → Rueckruf → Delegierter. `None` heisst: der Aufbau ist
    /// noch nicht so weit, und dann gibt es auch nichts auszufuehren.
    melden: RefCell<Option<Kommandomelder>>,
}

define_class!(
    /// Das Ziel, das der Klick auf einen der neun Schalter anspricht.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = LeistenquellenIvars]
    struct Leistenquelle;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Leistenquelle {}

    impl Leistenquelle {
        /// Der Nutzer hat einen der fuenf Bereichsschalter angeklickt.
        ///
        /// **Der Absender nennt sich ueber seine `tag`**, und die ist die
        /// Stelle des Bereichs in [`Bereich::ALLE`]. Damit entsteht keine
        /// zweite Aufzaehlung neben der bestehenden: die Nummer kommt aus ihr
        /// und wird in ihr wieder nachgeschlagen.
        // SAFETY: Die Signatur passt zu der, die `NSControl` aufruft: ein
        // Argument, der Absender.
        #[unsafe(method(bereichGedrueckt:))]
        fn bereich_gedrueckt(&self, absender: &NSButton) {
            let kommando = stelle_des_absenders(absender)
                .and_then(|stelle| Bereich::ALLE.get(stelle).copied())
                .map(kommando_des_bereichs);
            self.geklickt(absender, kommando);
        }

        /// Der Nutzer hat einen der drei Spaltenschalter angeklickt.
        ///
        /// Die `tag` ist hier die Stelle in [`Spalte::ALLE`], aus demselben
        /// Grund wie oben. Eine Spalte ohne Kommando kann hier nicht
        /// eintreffen — sie hat keinen Schalter —, und der Zweig meldet
        /// deshalb nichts, statt zu raten.
        // SAFETY: Die Signatur passt zu der, die `NSControl` aufruft: ein
        // Argument, der Absender.
        #[unsafe(method(spalteGedrueckt:))]
        fn spalte_gedrueckt(&self, absender: &NSButton) {
            let kommando = stelle_des_absenders(absender)
                .and_then(|stelle| Spalte::ALLE.get(stelle).copied())
                .and_then(kommando_der_spalte);
            self.geklickt(absender, kommando);
        }

        /// Der Nutzer hat den Schalter der tiefen Suche angeklickt (C2.3).
        ///
        /// **Ohne Umweg ueber die `tag`**, und das ist der Unterschied zu den
        /// beiden Zweigen darueber: der Absender ist der einzige Schalter
        /// dieses Selektors, also nennt der Selektor selbst schon das
        /// Kommando. Es gibt hier deshalb auch keinen Fall "kein Kommando";
        /// [`Leistenquelle::geklickt`] nimmt trotzdem denselben Weg, damit die
        /// Selbstkippung an genau einer Stelle zurueckgenommen wird.
        // SAFETY: Die Signatur passt zu der, die `NSControl` aufruft: ein
        // Argument, der Absender.
        #[unsafe(method(tiefeGedrueckt:))]
        fn tiefe_gedrueckt(&self, absender: &NSButton) {
            self.geklickt(absender, Some(KOMMANDO_DER_TIEFE));
        }
    }
);

impl Leistenquelle {
    /// Eine Quelle ohne Melder; ihn traegt der Aufbau der Oberflaeche nach.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(LeistenquellenIvars {
            melden: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Was bei jedem Klick geschieht, gleich auf welchen der neun Schalter und
    /// gleich, ob daraus ein Kommando wird.
    ///
    /// **Zuerst die Selbstkippung zuruecknehmen, dann melden.** Ein
    /// Ankreuzfeld kippt seinen Zustand selbst, bevor seine Aktion laeuft; das
    /// ist der einzige Schreibzugriff auf einen Schalterzustand, der nicht aus
    /// dem Modell kommt, und er wird hier sofort wieder abgetragen. Danach
    /// zeigt die Leiste wieder genau den Stand des Modells, und was der Befehl
    /// daran aendert, schreibt der eine Schreiber
    /// `Anwendungsdelegierter::bereichsleiste_nachziehen` — einmal, auf jedem
    /// Weg.
    ///
    /// **Deshalb braucht der abgewiesene Klick keinen eigenen Nachzug** (C2.4):
    /// er hat nach dieser Zeile nichts mehr hinterlassen, das zurueckspringen
    /// muesste. Bis zum 260812 zog der Melder des Delegierten stattdessen
    /// unbedingt nach, und nach einem **angenommenen** Klick liefen deshalb
    /// zwei Nachzuege hintereinander
    /// (`issues/260812-0727_*_der-nachzug-der-bereichsleiste-laeuft-nach-einem-angenommenen-klick-zweimal.md`).
    /// Die Alternative, den Nachzug an das Ergebnis des Befehls zu binden,
    /// waere eine Fallunterscheidung "war der Klick angenommen?" gewesen; hier
    /// entfaellt die Frage, statt beantwortet zu werden.
    ///
    /// **Ein Kommando ohne Absender gibt es hier nicht**: die Kippung wird auch
    /// dann zurueckgenommen, wenn die `tag` zu keinem Schalter dieser Leiste
    /// gehoert und nichts zu melden ist.
    fn geklickt(&self, absender: &NSButton, kommando: Option<Kommando>) {
        selbstkippung_zuruecknehmen(absender);
        if let Some(kommando) = kommando {
            self.melden(kommando);
        }
    }

    /// Gibt das Kommando weiter, falls jemand zuhoert.
    ///
    /// Die Ausleihe ist lesend und steht waehrend des Rufs, wie bei
    /// `Hauptfenster::melden`. Der einzige schreibende Zugriff auf dieselbe
    /// Zelle ist der Aufbau.
    fn melden(&self, kommando: Kommando) {
        let melden = self.ivars().melden.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(kommando);
        }
    }
}

/// Die Stelle, die ein Schalter ueber seine `tag` nennt.
///
/// Eine negative `tag` gibt es in dieser Leiste nicht; sie kaeme allein von
/// einem Absender, den dieses Modul nicht gebaut hat.
fn stelle_des_absenders(absender: &NSButton) -> Option<usize> {
    usize::try_from(absender.tag()).ok()
}

/// Die aufgebaute Bereichsleiste.
///
/// `NSControl` haelt sein Ziel nur schwach; die Leiste haelt die Quelle
/// deshalb selbst fest, wie es [`super::tableiste`] mit ihrem Ziel tut. Der
/// Ring, den die Gegenrichtung aufspannte, entsteht nicht: die Quelle haelt
/// allein den Melder und keine Ansicht.
pub struct Bereichsleiste {
    sicht: Retained<NSView>,
    /// Die Schalter der fuenf Bereiche, in der Reihenfolge von
    /// [`Bereich::ALLE`].
    ///
    /// Die Feldbreite steht in der Typangabe: ein sechster Bereich haelt hier
    /// den Bau an, dasselbe Muster wie `Aufteilung::rahmen`.
    bereichsschalter: [Retained<NSButton>; 5],
    /// Die Schalter der drei schaltbaren Spalten, in der Reihenfolge, die
    /// [`spaltenfach`] nennt.
    spaltenschalter: [Retained<NSButton>; 3],
    /// Der Schalter der tiefen Suche (C2.1).
    ///
    /// **Ein einzelnes Feld und keine dritte Reihung.** Die Begruendung steht
    /// im Modulkopf unter `# Der neunte Schalter ist eine Gruppe und keine
    /// dritte Sammlung`.
    tiefenschalter: Retained<NSButton>,
    quelle: Retained<Leistenquelle>,
}

impl Bereichsleiste {
    /// Baut die Leiste mit ihren neun Schaltern.
    ///
    /// Die Schalter stehen nebeneinander von links nach rechts, jeder so
    /// breit, wie seine Aufschrift ihn macht. Ihre Zustaende sind nach dem Bau
    /// noch nicht gesetzt; das tut [`Self::zustaende_setzen`], sobald das
    /// Modell steht.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
        let sicht = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, HOEHE)),
        );
        let quelle = Leistenquelle::neu(mtm);

        let mut links = EINZUG;
        let bereichsschalter = Bereich::ALLE.map(|bereich| {
            let schalter = schalter_bauen(
                mtm,
                &quelle,
                sel!(bereichGedrueckt:),
                Some(bereich.index()),
                bereich.beschriftung(),
                &format!("{} ein- und ausblenden", bereich.langname()),
            );
            einhaengen(&sicht, &schalter, &mut links);
            schalter
        });

        links += GRUPPENABSTAND - ABSTAND;
        let mut spaltenschalter = Vec::with_capacity(3);
        for (stelle, spalte) in Spalte::ALLE.into_iter().enumerate() {
            // Die Namensspalte traegt keinen Schalter (C3.1); welche Spalten
            // einen tragen, sagt allein `kommando_der_spalte`.
            if kommando_der_spalte(spalte).is_none() {
                continue;
            }
            let schalter = schalter_bauen(
                mtm,
                &quelle,
                sel!(spalteGedrueckt:),
                Some(stelle),
                spalte.beschriftung(),
                &format!(
                    "Spalte »{}« in beiden Dateilisten ein- und ausblenden",
                    spalte.beschriftung()
                ),
            );
            einhaengen(&sicht, &schalter, &mut links);
            spaltenschalter.push(schalter);
        }
        // Die Laenge ist keine Annahme ueber die Zukunft: sie folgt aus
        // `kommando_der_spalte`, und die Probe `genau_drei_spalten_sind_schaltbar`
        // haelt beide aneinander.
        let spaltenschalter: [Retained<NSButton>; 3] = spaltenschalter
            .try_into()
            .expect("genau drei Spalten tragen ein Kommando");

        // Die dritte Gruppe, und deshalb noch einmal [`GRUPPENABSTAND`]: der
        // Schalter der tiefen Suche schaltet weder eine Flaeche des Fensters
        // noch eine Spalte, sondern die Sucheinstellung des sichtbaren Tabs.
        // Er steht rechts von `Typ`, weil die Reihenfolge der Gruppen die
        // Reihenfolge ihres Nachtrags ist (C2.1).
        links += GRUPPENABSTAND - ABSTAND;
        let tiefenschalter = schalter_bauen(
            mtm,
            &quelle,
            sel!(tiefeGedrueckt:),
            None,
            AUFSCHRIFT_DER_TIEFE,
            "Den stehenden Filter auf den Unterbaum ausdehnen",
        );
        einhaengen(&sicht, &tiefenschalter, &mut links);

        Self {
            sicht,
            bereichsschalter,
            spaltenschalter,
            tiefenschalter,
            quelle,
        }
    }

    /// Die Ansicht, die in die Inhaltsflaeche des Fensters gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.sicht
    }

    /// Traegt den Melder ein, der jeden Klick als Kommando weitergibt.
    ///
    /// Gerufen vom Aufbau der Oberflaeche, mit einem Rueckruf, der den
    /// Anwendungsdelegierten schwach haelt. Derselbe Zuschnitt wie
    /// `Hauptfenster::melder_setzen` und die uebrigen Melder dieses Projekts.
    pub fn melder_setzen(&self, melden: Kommandomelder) {
        *self.quelle.ivars().melden.borrow_mut() = Some(melden);
    }

    /// Schreibt die neun Schalterzustaende aus dem Modell.
    ///
    /// **Der eine Schreiber, und er schreibt nichts als Zustaende.** Er ruft
    /// weder `anwenden` noch `setHidden` und fasst den Ersthelfer nicht an,
    /// aus demselben Grund, aus dem `fokusanzeige_nachziehen` es nicht tut:
    /// eine ausgeblendete Ansicht, die den Rang haelt, laesst AppKit ihn neu
    /// vergeben und die Meldung ein zweites Mal ausloesen.
    ///
    /// **Immer alle neun und nie nur der geaenderte.** Ein Befehl kann zwei
    /// Schalter bewegen (Vorschau und Editor teilen sich die Flaeche, C2.3),
    /// und der erste Ruf ueberhaupt schreibt die ganze geladene Sitzung in die
    /// Leiste, ohne dass jemand geklickt haette. Die Frage "welcher hat sich
    /// geaendert" waere hier also eine Fallunterscheidung, die die Antwort
    /// schon voraussetzt.
    ///
    /// **`tief` kommt aus einer anderen Quelle als die beiden anderen
    /// Argumente**, und die Leiste erfaehrt davon nichts: sie bekommt drei
    /// Werte und schreibt drei Gruppen. Wo `tief` herkommt, entscheidet allein
    /// `Anwendungsdelegierter::bereichsleiste_nachziehen`; das ist die Naht,
    /// an der eine Antwort "je Fenster" auf
    /// `decisions/260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`
    /// anfiele, ohne diese Datei zu beruehren.
    pub fn zustaende_setzen(
        &self,
        sichtbar: &Sichtbarkeit,
        spalten: &Spaltensichtbarkeit,
        tief: bool,
    ) {
        for bereich in Bereich::ALLE {
            zustand_setzen(
                &self.bereichsschalter[bereich.index()],
                sichtbar_in(sichtbar, bereich),
            );
        }
        for spalte in Spalte::ALLE {
            let Some(fach) = spaltenfach(spalte) else {
                continue;
            };
            zustand_setzen(
                &self.spaltenschalter[fach],
                spalte_sichtbar_in(spalten, spalte),
            );
        }
        zustand_setzen(&self.tiefenschalter, tief);
    }
}

/// Baut ein Ankreuzfeld, das seine Stelle ueber die `tag` nennt.
///
/// **`None` heisst: dieser Schalter muss sich nicht nennen.** Ein Selektor,
/// den mehrere Schalter teilen, braucht die `tag`, um den Absender einer
/// Aufzaehlung zuzuordnen; der Schalter der tiefen Suche ist der einzige
/// seines Selektors, und eine `tag` an ihm waere eine Nummer, die niemand
/// liest — und die ein Leser fuer die Stelle in einer Aufzaehlung hielte.
fn schalter_bauen(
    mtm: MainThreadMarker,
    quelle: &Leistenquelle,
    aktion: Sel,
    stelle: Option<usize>,
    aufschrift: &str,
    hinweis: &str,
) -> Retained<NSButton> {
    // SAFETY: `quelle` ist von der Klasse, die `bereichGedrueckt:` und
    // `spalteGedrueckt:` mit der erwarteten Signatur beantwortet, und `sel!`
    // beim Aufrufer liefert einen gueltigen Selektor. Ueber die Lebensdauer
    // verlangt die Bindung nichts: `NSControl.target` ist eine schwache
    // Eigenschaft ("This value is weak in apps built with ARC",
    // `objc2-app-kit-0.3.2/src/generated/NSControl.rs:91-99`), und
    // `Bereichsleiste` haelt die Quelle darum selbst fest.
    let schalter = unsafe {
        NSButton::checkboxWithTitle_target_action(
            &NSString::from_str(aufschrift),
            Some(quelle as &AnyObject),
            Some(aktion),
            mtm,
        )
    };
    // **Erst die Groesse, dann die Schrift.** `setControlSize:` schreibt die
    // Masse der Zelle neu und zieht dabei die Schrift nach; danach gesetzt
    // bliebe die kleine Systemschrift stehen, davor gesetzt waere sie wieder
    // ueberschrieben.
    schalter.setControlSize(NSControlSize::Small);
    schalter.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    schalter.setToolTip(Some(&NSString::from_str(hinweis)));
    // **Kein Schalter nimmt den Ersthelferrang an** (C1.4). Die Begruendung
    // steht im Modulkopf; sie ist der Grund, aus dem `Fokus` keinen sechsten
    // Wert bekommt.
    schalter.setRefusesFirstResponder(true);
    if let Some(stelle) = stelle {
        schalter.setTag(stelle as isize);
    }
    schalter
}

/// Haengt einen Schalter an der laufenden Stelle ein und rueckt sie weiter.
///
/// Senkrecht mittig in der 18 Punkte hohen Leiste. Keine Autogroesse: die
/// Schalter haengen am linken Rand, und was rechts frei wird, bleibt frei.
fn einhaengen(sicht: &NSView, schalter: &NSButton, links: &mut f64) {
    schalter.sizeToFit();
    let groesse = schalter.frame().size;
    schalter.setFrame(NSRect::new(
        NSPoint::new(*links, (HOEHE - groesse.height) / 2.0),
        groesse,
    ));
    sicht.addSubview(schalter);
    *links += groesse.width + ABSTAND;
}

/// Nimmt zurueck, was das Ankreuzfeld beim Klick an sich selbst geschrieben
/// hat.
///
/// Der Zustand, den `state` jetzt meldet, ist die Vermutung von AppKit und
/// nicht die Antwort des Modells; das Gegenteil davon ist der Zustand von vor
/// dem Klick. Gelesen wird dabei ausnahmsweise aus der Leiste, und nur diese
/// eine Frage: was hat der Klick gerade umgestellt. Der angezeigte Stand
/// selbst kommt weiterhin allein aus dem Modell.
///
/// **Sichtbar wird die Zwischenstellung nicht.** Aktion und Nachzug laufen im
/// selben Durchgang der Ereignisschlange, gezeichnet wird danach; auf dem
/// Schirm steht nur das Ergebnis.
fn selbstkippung_zuruecknehmen(schalter: &NSButton) {
    zustand_setzen(schalter, schalter.state() != NSControlStateValueOn);
}

/// Setzt einen Schalter auf an oder aus.
fn zustand_setzen(schalter: &NSButton, an: bool) {
    let zustand = if an {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    };
    schalter.setState(zustand);
}

#[cfg(test)]
mod tests {
    use super::*;

    use krk_core::tasten::Wirkungsbereich;

    use crate::kommandos::fokus::Fokus;

    /// Alle neun Schalter dieser Leiste, mit dem Kommando, das sie schicken,
    /// in der Reihenfolge, in der [`Bereichsleiste::bauen`] sie einhaengt.
    ///
    /// Die Proben lesen dieselben Quellen, aus denen [`Bereichsleiste::bauen`]
    /// die Schalter baut: die beiden Aufbautabellen und, fuer den neunten,
    /// [`KOMMANDO_DER_TIEFE`] samt [`AUFSCHRIFT_DER_TIEFE`]. Eine Aufstellung
    /// daneben pruefte sich selbst.
    fn alle_schalter() -> Vec<(String, Kommando)> {
        let bereiche = Bereich::ALLE.into_iter().map(|bereich| {
            (
                bereich.beschriftung().to_owned(),
                kommando_des_bereichs(bereich),
            )
        });
        let spalten = Spalte::ALLE.into_iter().filter_map(|spalte| {
            kommando_der_spalte(spalte).map(|kommando| (spalte.beschriftung().to_owned(), kommando))
        });
        let tiefe = std::iter::once((AUFSCHRIFT_DER_TIEFE.to_owned(), KOMMANDO_DER_TIEFE));
        bereiche.chain(spalten).chain(tiefe).collect()
    }

    /// C2.1 der Filter-Runde, C2.1 und C3.1 der Bereichsleisten-Runde: fuenf
    /// plus drei plus einer, und keiner doppelt.
    #[test]
    fn die_leiste_traegt_neun_schalter() {
        let schalter = alle_schalter();
        assert_eq!(
            schalter.len(),
            9,
            "fuenf Bereiche, drei Spalten und die tiefe Suche"
        );
    }

    /// C2.1: der neunte Schalter traegt die englische Aufschrift `Deep` und
    /// steht rechts neben `Typ`.
    ///
    /// Die **Lage auf dem Schirm** prueft diese Probe nicht — sie liest die
    /// Reihenfolge, in der [`Bereichsleiste::bauen`] einhaengt, und die ist
    /// dieselbe Reihenfolge von links nach rechts. Dass die Leiste dabei ihre
    /// 18 Punkte behaelt, ist am laufenden Buendel zu sehen und hier nicht.
    #[test]
    fn der_neunte_schalter_heisst_deep_und_steht_rechts_von_typ() {
        assert_eq!(
            AUFSCHRIFT_DER_TIEFE, "Deep",
            "die Aufschrift ist englisch und wird nicht uebersetzt"
        );
        let schalter = alle_schalter();
        let namen: Vec<&str> = schalter.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(
            namen.last().copied(),
            Some(AUFSCHRIFT_DER_TIEFE),
            "der Schalter der tiefen Suche steht ganz rechts"
        );
        assert_eq!(
            namen.get(namen.len() - 2).copied(),
            Some(Spalte::Typ.beschriftung()),
            "links von ihm steht der Schalter der Spalte »Typ«"
        );
    }

    /// C2.2: die Leiste bekommt einen neunten Schalter und [`Fokus`] keinen
    /// sechsten Wert.
    ///
    /// **Der Ersthelferrang ist die tragende Entscheidung dieses Moduls**, und
    /// ihr Grund steht im Modulkopf unter `# Kein Schalter nimmt den
    /// Ersthelferrang an`: `Anwendungsdelegierter::ersthelferbereich` laeuft
    /// ueber [`Bereich::ALLE`], und die Leiste liegt in keinem der fuenf
    /// Teilbaeume. Der neunte Schalter geht dafuer durch dieselbe eine
    /// Bauzeile wie die acht vorhandenen — [`schalter_bauen`] setzt
    /// `setRefusesFirstResponder(true)` fuer jeden —, und dass er sie damit
    /// wirklich traegt, ist am laufenden Buendel zu sehen und hier nicht.
    #[test]
    fn der_neunte_schalter_gibt_fokus_keinen_sechsten_wert() {
        assert_eq!(
            Fokus::ALLE.len(),
            5,
            "die Leiste ist kein fokussierbarer Bereich"
        );
    }

    /// Die Zusage der Feldbreite `[Retained<NSButton>; 3]`: genau drei Spalten
    /// tragen ein Kommando, und die Namensspalte ist nicht darunter (C3.1).
    ///
    /// Ohne diese Probe haenge das `expect` in [`Bereichsleiste::bauen`] an
    /// einer Annahme statt an einer geprueften Aussage.
    #[test]
    fn genau_drei_spalten_sind_schaltbar() {
        let schaltbar: Vec<Spalte> = Spalte::ALLE
            .into_iter()
            .filter(|spalte| kommando_der_spalte(*spalte).is_some())
            .collect();
        assert_eq!(
            schaltbar,
            vec![Spalte::Groesse, Spalte::Geaendert, Spalte::Typ]
        );
        assert_eq!(kommando_der_spalte(Spalte::Name), None);
    }

    /// Die Aufbautabelle nennt fuer jeden Schalter **genau ein** Kommando, und
    /// kein Kommando zweimal.
    ///
    /// Zwei Schalter auf demselben Kommando hiessen: einer von beiden schaltet
    /// etwas anderes, als seine Aufschrift sagt.
    #[test]
    fn jeder_schalter_nennt_genau_ein_eigenes_kommando() {
        let schalter = alle_schalter();
        for (stelle, (name, kommando)) in schalter.iter().enumerate() {
            for (anderer_name, anderes) in schalter.iter().skip(stelle + 1) {
                assert_ne!(
                    kommando, anderes,
                    "»{name}« und »{anderer_name}« schicken dasselbe Kommando"
                );
            }
        }
    }

    /// C2.6 der Bereichsleisten-Runde und C2.4 der Filter-Runde: die Schalter
    /// wirken aus jedem Fokus.
    ///
    /// Ein Kommando mit einem engeren Wirkungsbereich waere genau dann
    /// abgewiesen, wenn der Nutzer es am dringendsten braucht — der Klick mit
    /// der Schreibmarke im Editor, der den Editor wieder loswerden will.
    ///
    /// **Fuer den Schalter der tiefen Suche ist das zugleich die halbe
    /// Zusage aus C2.4**: ueber die Zulaessigkeit entscheidet der
    /// Wirkungsbereich und nicht, ob der Befehl etwas findet. Dass er auch bei
    /// leerem Filtertext durchkommt und dann nichts an der Liste aendert, haelt
    /// `ohne_filtertext_aendert_die_tiefe_suche_nichts` in
    /// `crates/krk-core/tests/verzeichnis.rs`.
    #[test]
    fn jeder_schalter_wirkt_aus_jedem_fokus() {
        for (name, kommando) in alle_schalter() {
            assert_eq!(
                kommando.wirkungsbereich(),
                Wirkungsbereich::Ueberall,
                "der Schalter »{name}« wirkt nicht aus jedem Fokus"
            );
        }
    }

    /// Die Stelle eines Spaltenschalters folgt der Reihenfolge in
    /// [`Spalte::ALLE`] und laesst kein Fach frei.
    #[test]
    fn jede_schaltbare_spalte_hat_ihr_eigenes_fach() {
        assert_eq!(spaltenfach(Spalte::Name), None);
        assert_eq!(spaltenfach(Spalte::Groesse), Some(0));
        assert_eq!(spaltenfach(Spalte::Geaendert), Some(1));
        assert_eq!(spaltenfach(Spalte::Typ), Some(2));
    }
}
