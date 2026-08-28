//! Die eine Statuszeile am Fensterfuss.
//!
//! Sie ist der einzige Weg, auf dem KRK dem Nutzer eine laufende Meldung zeigt.
//! Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: der
//! Ordner ohne Leserecht meldet sich hier, und allein der fehlende
//! Tastenabgriff bricht mit einem Hinweisfenster ab, weil er die Anwendung als
//! ganze betrifft.
//!
//! Vorher lief beides ueber `eprintln!`. Eine ueber den Finder gestartete
//! Anwendung hat keine Standardfehlerausgabe, LaunchServices haengt sie ins
//! Leere; in der einzigen Betriebsart, die die Abnahme zulaesst, war die
//! Fehlerbehandlung damit still. C1 verlangt seit dem 260804-0830
//! ausdruecklich, dass KRK keine Meldung an den Nutzer ueber die
//! Standardfehlerausgabe gibt.
//!
//! Seit dem 260804-1832 traegt sie eine zweite Art von Meldung: den Stand einer
//! laufenden Dateioperation aus C4. Der Nutzer hat den Fortschritt aus dem
//! Blatt hierher verlegt
//! (`decisions/260804-1832_*_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`),
//! weil ein Blatt das Fenster sperrt, das C4 bedienbar zusagt, und auf dem
//! Referenzgeraet 354 bis 403 ms zum Aufgehen braucht, waehrend L8 200 ms
//! zusagt.
//!
//! **Die Art steht in der Signatur und nicht in einer zweiten Funktion.** Ein
//! Fortschritt ist kein Fehler und wird nicht rot; eine zweite Funktion neben
//! [`Statuszeile::zeigen`] waeren zwei Wahrheiten darueber, was in der Zeile
//! steht.
//!
//! Was diese Zeile **nicht** traegt: den Lesefortschritt und die Zahl der
//! Eintraege. C1 der Runde 1 sagt beides nicht zu; sie kaemen in einer
//! spaeteren Runde in dieselbe Zeile und nicht in eine zweite daneben.
//!
//! # Eine Zeile ueber die volle Fensterbreite, nicht zwei an zwei Fuessen
//!
//! Bis zur Runde 6 sass eine Zeile am Fuss **jedes** Dateifensters, und welche
//! von beiden eine Meldung bekam, sagte ihre Lage: die Meldung des linken
//! Dateifensters stand links. Seit der Runde 6 gibt es genau eine Zeile, und
//! sie liegt unter der Fensterzeile ueber die volle Breite, zwischen der
//! Bereichsleiste und den fuenf Bereichen. Der Nutzer hat den Preis dieser
//! Zusammenlegung am 260812-1105 vorgelegt bekommen und angenommen; er steht
//! ausgeschrieben bei [`zeile`] und bei [`zeilentext`].
//!
//! Die Zeile gehoert damit dem Fenster und keinem Bereich. Sie ist kein
//! sechster Wert von [`crate::kommandos::fokus::Fokus`] und kein sechster
//! Bereich der Fensterzeile, sondern deren Schwester unter der Inhaltsflaeche,
//! genau wie die Bereichsleiste; `ersthelferbereich` geht die fuenf Bereiche
//! der `NSSplitView` durch, und eine Zeile darin waere ein sechster Bereich
//! oder ein blinder Fleck.
//!
//! **Den Ersthelferrang nimmt sie nicht an** (C5.11), und getragen wird das
//! allein davon, was `labelWithString:` baut: der Kopf des Systems nennt es
//! "a non-wrapping, non-editable, non-selectable text field"
//! (`NSTextField.h:87-93`). Ein `setRefusesFirstResponder(true)` steht deshalb
//! nicht daneben; die Schalter der Bereichsleiste brauchen es, weil ein
//! `NSButton` den Rang von sich aus annimmt. **Ob das bei eingeschalteter
//! vollstaendiger Tastaturbedienung haelt, ist nicht gemessen.** Die
//! Abschlussnotiz der Runde 5 hat die Frage fuer die Schalter offen gelassen,
//! und sie steht fuer diese Zeile ebenso offen; C5.11 ist ein Kriterium am
//! laufenden Buendel und keine Zusage im Baum.
//!
//! **Es ist wieder eine Ansicht und nicht zwei, und C5.11 damit wieder so eng
//! wie in der Runde 5.** Schritt 11 der Runde 6 hatte das Feld als
//! Dokumentansicht in eine `NSScrollView` gesetzt und die Frage damit auf deren
//! `NSScroller` ausgeweitet, also auf Steuerelemente derselben Art wie die
//! Schalter der Bereichsleiste. Der Nutzer hat den Schritt am 260812
//! zurueckgenommen
//! (`decisions/260812-1809_*_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`);
//! ohne Bildlaufansicht gibt es keine `NSScroller`, und die Frage betrifft
//! wieder allein dieses eine Textfeld. **Beantwortet ist sie dadurch nicht** —
//! abzunehmen bleibt sie am laufenden Buendel, mit eingeschalteter
//! vollstaendiger Tastaturbedienung.
//!
//! # Wer die Zeile bekommt, wenn mehrere zugleich etwas zu sagen haben
//!
//! Die Auswahl steht in [`zeile`], einer Funktion ohne AppKit, damit sie
//! pruefbar ist. Die Lebensdauern der vier Quellen mit eigenem Feld stehen bei
//! ihren Feldern in `DateifensterQuelle`; hier steht allein die Rangfolge.
//!
//! **Die untersten Raenge haben kein Feld.** Der Filterstand aus C4 der
//! Runde 10 und der Markierungsstand aus C2 werden bei jedem Schreiben der
//! Zeile aus dem Ordnermodell des sichtbaren Tabs gerechnet, statt gesetzt und
//! geloescht zu werden. Beide tragen dieselbe Begruendung: ein Feld haette
//! eine zweite Loeschregel, und beide sind ein Zustand und kein Ereignis. Sie
//! steht bei `DateifensterQuelle::markierungsstand_text` und bei
//! [`filterstand_text`]. Der Seitenzaehler aus C4 der Runde 20 dazwischen wird
//! ebenso bei jedem Schreiben gefragt, beim Vorschaufenster statt beim
//! Ordnermodell; seinen Satz formt [`seitenzaehler_text`].
//!
//! **Zwei Herkuenfte, eine Rangfolge.** Die Dateifenster tragen die meisten
//! Raenge, und zwar jedes fuer sich: je Dateifenster-Rang treten zwei Bewerber
//! an. Der Seitenzaehler kommt aus dem Vorschaufenster, das zu keiner Seite
//! gehoert, und hat genau einen Bewerber. Wer welchen Rang traegt, sagt
//! [`Rang::herkunft`] und sonst nichts.
//!
//! **Ein ausgeblendeter Bereich bewirbt sich nicht.** Nur die Quellen der
//! sichtbaren Dateifenster treten an, und der Seitenzaehler nur bei sichtbarer
//! Vorschau; die Begruendung steht bei [`zeile`].
//!
//! # Wie eine Meldung lesbar wird, die breiter ist als das Fenster
//!
//! Die Zeile kuerzt am rechten Rand, und **genau dann, wenn sie kuerzt**, traegt
//! sie einen Kurzhinweis, der den ganzen Satz zeigt (`setToolTip:`). Der Anlass
//! ist der Inhalt dieser Runde: die Meldungen zur beschaedigten Ablagedatei
//! nennen zwei Pfade, und ein abgeschnittener Pfad sieht aus wie eine Auskunft
//! und ist keine, denn welche Datei gemeint ist, steht am Ende und nicht am
//! Anfang.
//!
//! **"Genau dann" ist eine Messung und keine Redewendung.** Ein Hinweis, der
//! den sichtbaren Text wiederholt, waere Rauschen, deshalb haengt er nicht am
//! Vorhandensein eines Textes, sondern an [`Statuszeile::abgeschnitten`]: dort
//! wird verglichen, was `sizeToFit` am Feld misst, mit der Breite, die das Feld
//! im Fenster hat.
//!
//! **Geblaettert wird dafuer nicht.** Schritt 11 der Runde 6 hatte das gebaut
//! und dabei zwei Kosten erzeugt, die der Nutzer am 260812 nicht angenommen hat
//! (`decisions/260812-1809_*_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`):
//! mit dem Zeiger ueber den achtzehn Punkten am Fensterfuss bewegte ein
//! Zweifingerstrich die Zeile und nicht die Liste darueber, und die
//! `NSScrollView` brachte `NSScroller` mit, also genau die Art Steuerelement,
//! fuer die C5.11 offen ist. Der Kurzhinweis kostet keine von beiden: keine
//! zweite Ansicht, keine Ereignisbehandlung, kein Griff nach der Geste. **Das
//! Mittel liegt im Baum** — [`super::bereichsleiste`] setzt es an den Schaltern
//! der Leiste, und dieser Zuschnitt ist von dort uebernommen.
//!
//! **Der Preis ist benannt und vom Nutzer angenommen: der Text ist nicht
//! markierbar und nicht kopierbar.** Der Hinweis erscheint erst nach einer
//! Verweildauer, die das System bestimmt, und verschwindet, sobald der Zeiger
//! weggeht. Wer eine Meldung weitergeben will, tippt sie ab; Kopierbarkeit war
//! in keiner Fassung von C5 zugesagt.
//!
//! **Gemessen wird beim Setzen des Textes, nicht beim Zeigen des Hinweises.**
//! Zieht der Nutzer das Fenster danach breiter oder schmaler, ohne dass eine
//! neue Meldung kommt, steht der Hinweis oder fehlt er nach der alten Breite;
//! die naechste Meldung zieht ihn nach
//! (`issues/260812-1854_*_der-kurzhinweis-der-statuszeile-veraltet-bei-einer-fensteraenderung.md`).
//! Der eine Ausloesepunkt einer Breitenaenderung waere `setFrameSize:` am Feld,
//! und ihn zu ueberschreiben verlangte eine eigene Klasse ueber `NSTextField`.
//! Die liesse sich nicht mehr ueber `labelWithString:` bauen — und genau dieser
//! Erzeuger ist die ganze Grundlage, auf der C5.11 heute ruht. Der Nachzug
//! kostete also die Zusage, die er begleiten soll.
//!
//! **Einzeilig bleibt sie.** `setMaximumNumberOfLines(1)` steht am Feld: eine
//! lange Meldung wird lang und nicht hoch, weil die Zeile [`HOEHE`] misst und
//! das Fenster darunter keinen Punkt frei hat.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField`, `NSColor`, `NSFont`, `NSView` und `NSString` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso `systemFontOfSize:`, `smallSystemFontSize`
//! und die Ausrichtung `NSTextAlignmentLeft`. Fuenf Beruehrungen sind juenger
//! als ihre Klasse: die drei Semantikfarben `labelColor`, `secondaryLabelColor`
//! und `systemRedColor` seit 10.10, `maximumNumberOfLines` seit 10.11 und
//! `labelWithString:` seit 10.12 — die hoechste Untergrenze dieser Datei.
//!
//! **Der Kurzhinweis und seine Messung heben sie nicht an.** `toolTip`
//! (`NSView.h:310`), `stringValue` (`NSControl.h:36`), `sizeToFit`
//! (`NSControl.h:44`) und `frame` (`NSView.h:129`) tragen im Kopf des Systems
//! keine eigene Angabe; ohne Angabe heisst 10.0. Alle Zahlen am SDK gelesen.
//! **Die Angaben zu `NSScrollView`, `NSClipView` und `NSBorderType` stehen hier
//! nicht mehr**, weil mit der Ruecknahme von Schritt 11 auch die Klassen wieder
//! abgegangen sind.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Die Autogroesse steht seit der Runde 6 nicht mehr hier**, sondern bei
//! [`super::fenster::fensterinhalt`], das die Zeile einhaengt und ihr Rahmen
//! und Maske in denselben zwei Zeilen gibt wie der Bereichsleiste daneben.
//! `setAutoresizingMask:` und `setFrame:` stehen seit 10.0 und tragen keine
//! eigene Angabe; `setAutoresizingMask:` ruft diese Datei nicht, und `setFrame:`
//! ruft sie beim Aufbau und um jede Messung herum, die sie unmittelbar danach
//! wieder zuruecknimmt.

use objc2::rc::Retained;
use objc2_app_kit::{NSColor, NSFont, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString, ns_string};

use krk_core::ablage::{Fensterseite, Sichtbarkeit};

use crate::fenstermodell::{Bereich, sichtbar_in};
use crate::kommandos::operationen::zahl;

/// Die Hoehe der Zeile in Punkten.
///
/// Eine Zeile in der kleinen Systemschrift mit etwas Luft darum.
pub const HOEHE: f64 = 18.0;

/// Der Abstand vom linken Rand, damit der Text nicht an der Trennlinie klebt.
pub const EINZUG: f64 = 6.0;

/// Was fuer eine Meldung gerade in der Zeile steht.
///
/// Zwei Werte, weil die Zeile seit dem 260804-1832 zwei Sorten traegt. Sie
/// unterscheiden sich allein in der Farbe: ein Fehler ist rot, damit ihn der
/// Nutzer neben einer leeren Liste nicht uebersieht, und ein Fortschritt ist es
/// nicht, weil er keiner ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// Ein Fehler oder ein Hinweis, den der Nutzer bemerken soll.
    Fehler,
    /// Der Stand einer laufenden Dateioperation (C4).
    Vorgang,
}

/// Die Raenge der Zeile, vom obersten zum untersten.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig: ein weiterer Rang
/// haelt den Bau an und erzwingt die Antwort darauf, wo er einzuordnen ist,
/// ob er ein Fehler ist und wer ihn traegt (C4.10 der Runde 10, C4.6 der
/// Runde 20). Dieselbe Bauart wie `Bereich` und `Fokus`. **Wie viele es sind,
/// sagt [`Rang::ALLE`] und kein Doc-Kommentar**: die Zahl ist mit der Runde 20
/// gestiegen, und jede Nennung hier waere mit ihr falsch geworden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rang {
    /// Was KRK auf den letzten Tastenbefehl zu sagen hat.
    Befehlsantwort,
    /// Der Stand einer laufenden Dateioperation (C4).
    Vorgangsanzeige,
    /// Ein Ereignis am Fenster, das niemand angefordert hat.
    Fenstermeldung,
    /// Der Zustand des sichtbaren Ordners.
    Tabmeldung,
    /// Der stehende Filtertext und was er von der Liste uebrig laesst (C4 der
    /// Runde 10).
    Filterstand,
    /// Die aktuelle Seite und die Seitenzahl des PDF, das die Vorschau zeigt
    /// (C4 der Runde 20).
    ///
    /// Der einzige Rang, den kein Dateifenster traegt; siehe
    /// [`Rang::herkunft`].
    Seitenzaehler,
    /// Was im sichtbaren Tab markiert ist (C2).
    Markierungsstand,
}

/// Wer einen Rang traegt: eines der zwei Dateifenster oder das Vorschaufenster.
///
/// Eine vollstaendige Fallunterscheidung wie [`Rang`] selbst. Sie ist der
/// Grund, warum der Seitenzaehler nicht in [`Quellen`] steht: `Quellen` ist,
/// was **ein Dateifenster** der Zeile anzubieten hat, und der Seitenzaehler
/// kommt aus einem Bereich, der zu keiner Seite gehoert. Ihn dort
/// einzutragen, waere eine Luege ueber seine Herkunft, die [`zeilentext`] beim
/// naechsten Seitenwechsel als "linkes Dateifenster: Seite 3 von 9"
/// aussprechen wuerde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunftsart {
    /// Den Rang traegt jedes Dateifenster fuer sich; es gibt zwei Bewerber.
    Dateifenster,
    /// Den Rang traegt allein das Vorschaufenster; es gibt einen Bewerber.
    Vorschau,
}

impl Rang {
    /// Alle Raenge, vom obersten zum untersten.
    ///
    /// **Die Reihenfolge ist die Rangfolge**, und [`zeile`] laeuft ohne eine
    /// zweite Vorschrift daneben ueber dieses Feld. Wer sie aendert, aendert
    /// die Auswahl der Zeile.
    ///
    /// **Der Filterstand steht ueber dem Markierungsstand** (C4.1). Die eine
    /// Zeile darunter traegt die ganze Wirkung der Nutzerfrage
    /// `decisions/260814-1552_*_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`;
    /// faellt sie anders aus als die Empfehlung, wandert diese Zeile und
    /// sonst nichts.
    ///
    /// **Der Seitenzaehler steht zwischen beiden** (A5 der Runde 20): ein
    /// stehender Filtertext ist eine Eingabe des Nutzers, die er sehen muss,
    /// und verdraengt den Zaehler (C4.5); die Markierung im Dateifenster ist
    /// eine Auskunft desselben Ranges wie die Seite, und die Seite ist das
    /// Juengere.
    pub const ALLE: [Rang; 7] = [
        Rang::Befehlsantwort,
        Rang::Vorgangsanzeige,
        Rang::Fenstermeldung,
        Rang::Tabmeldung,
        Rang::Filterstand,
        Rang::Seitenzaehler,
        Rang::Markierungsstand,
    ];

    /// Ob eine Meldung dieses Ranges ein Fehler ist.
    ///
    /// **Die Art faellt mit dem Rang und wird aus ihm gerechnet statt
    /// gesetzt.** Ein Fortschritt, eine Filterzahl, eine Seitenzahl und eine
    /// Markierungszahl sind keine Fehler, die drei uebrigen sind welche (C4.2
    /// der Runde 10, C4.6 der Runde 20). Ein zweites Feld, das jemand setzt,
    /// waere die Gelegenheit, eine Markierungszahl rot zu faerben.
    pub const fn art(self) -> Art {
        match self {
            Rang::Befehlsantwort => Art::Fehler,
            Rang::Vorgangsanzeige => Art::Vorgang,
            Rang::Fenstermeldung => Art::Fehler,
            Rang::Tabmeldung => Art::Fehler,
            Rang::Filterstand => Art::Vorgang,
            Rang::Seitenzaehler => Art::Vorgang,
            Rang::Markierungsstand => Art::Vorgang,
        }
    }

    /// Wer diesen Rang traegt.
    ///
    /// Die eine Stelle, an der ein Rang seiner Herkunft zugeordnet wird;
    /// [`zeile`] verzweigt darueber, und [`Quellen::text`] antwortet fuer
    /// jeden Rang der Vorschau mit `None`. Vollstaendig, ohne Auffangzweig.
    pub const fn herkunft(self) -> Herkunftsart {
        match self {
            Rang::Befehlsantwort
            | Rang::Vorgangsanzeige
            | Rang::Fenstermeldung
            | Rang::Tabmeldung
            | Rang::Filterstand
            | Rang::Markierungsstand => Herkunftsart::Dateifenster,
            Rang::Seitenzaehler => Herkunftsart::Vorschau,
        }
    }
}

/// Was ein Dateifenster der Zeile anzubieten hat.
///
/// Je Dateifenster-Rang eine Quelle. Die vier oberen haelt das Dateifenster in
/// je einem eigenen Feld mit je einer Loeschregel, die zwei unteren rechnet es
/// bei jeder Abfrage; `DateifensterQuelle::meldungsquellen` schreibt sie ab.
/// Der Seitenzaehler hat hier kein Feld, weil kein Dateifenster ihn traegt
/// ([`Rang::herkunft`]).
///
/// **Eigene Zeichenketten und keine Ausleihen.** Der Anwendungsdelegierte holt
/// beide Saetze nacheinander aus zwei `RefCell`-Feldern und ruft danach
/// AppKit; eine Ausleihe, die diesen Ruf ueberlebte, waere genau die Bauart,
/// die der Modulkopf von [`super::tabelle`] fuer das Tabmodell ausschliesst.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Quellen {
    /// Rang 1: was KRK auf den letzten Tastenbefehl zu sagen hat.
    pub befehlsantwort: Option<String>,
    /// Rang 2: der Stand einer laufenden Dateioperation.
    pub vorgangsanzeige: Option<String>,
    /// Rang 3: ein Ereignis am Fenster, das niemand angefordert hat.
    pub fenstermeldung: Option<String>,
    /// Rang 4: der Zustand des sichtbaren Ordners.
    pub tabmeldung: Option<String>,
    /// Rang 5: der stehende Filtertext und was er von der Liste uebrig laesst.
    pub filterstand: Option<String>,
    /// Rang 6: was im sichtbaren Tab markiert ist.
    pub markierungsstand: Option<String>,
}

impl Quellen {
    /// Was dieses Dateifenster auf diesem Rang zu sagen hat.
    ///
    /// Die eine Stelle, die einen Rang auf sein Feld abbildet, und eine
    /// vollstaendige Fallunterscheidung: ein weiterer Rang haelt hier den Bau
    /// an (C4.10 der Runde 10).
    fn text(&self, rang: Rang) -> Option<&str> {
        match rang {
            Rang::Befehlsantwort => self.befehlsantwort.as_deref(),
            Rang::Vorgangsanzeige => self.vorgangsanzeige.as_deref(),
            Rang::Fenstermeldung => self.fenstermeldung.as_deref(),
            Rang::Tabmeldung => self.tabmeldung.as_deref(),
            Rang::Filterstand => self.filterstand.as_deref(),
            // Kein Dateifenster traegt diesen Rang (`Rang::herkunft`); `zeile`
            // holt ihn beim Vorschaufenster und fragt diese Quellen dafuer gar
            // nicht erst. Ein Feld hier waere eine Luege ueber die Herkunft.
            Rang::Seitenzaehler => None,
            Rang::Markierungsstand => self.markierungsstand.as_deref(),
        }
    }
}

/// Was der fuenfte Rang aus dem sichtbaren Tab braucht.
///
/// Sechs Groessen, und keine davon wird hier gerechnet. Vier kommen aus dem
/// `Ordnermodell` und stehen dort schon; die zwei der Runde 11 kommen vom
/// [`crate::tabs::Tabinhalt`], der den Durchlauf haelt. **Dieser Rang rechnet
/// nichts nach, was Modell oder Tab ohnehin wissen**; eine eigene Rechnung
/// daneben waere eine zweite Wahrheit ueber denselben Zustand.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Filterstand {
    /// Wie viele Zeilen die Liste jetzt zeigt: `Ordnermodell::zeilenzahl`.
    ///
    /// **Entschiedene Zeilen und keine Treffer** (C4.6 der Runde 10). Wie viele Treffer
    /// unter einem Ordner liegen, weiss niemand in diesem Baum: der Durchlauf
    /// hoert je Ordner beim ersten Fund auf. Der Wert waechst waehrend eines
    /// Durchlaufs von selbst mit, weil er die Sichtreihenfolge zaehlt und
    /// keinen eigenen Zaehler fuehrt (C4.5 der Runde 10).
    pub gezeigt: usize,
    /// Wie viele Eintraege der angezeigte Ordner hat, ungefiltert.
    pub vorhanden: usize,
    /// Wie viele Markierungen der Filter gerade ausblendet.
    ///
    /// Null heisst: keine, und dann steht der Teil des Satzes, der sie nennt,
    /// nicht da (C4.4 der Runde 10).
    pub ausgeblendete_markierungen: usize,
    /// Ob der begonnene Lesevorgang seinen Bestand noch abloesen muss.
    ///
    /// Kommt aus `Ordnermodell::ersetzt_beim_naechsten_stapel`, der vorhandenen
    /// Frage nach genau diesem Zustand (C4.7 der Runde 10).
    pub ersetzt_beim_naechsten_stapel: bool,
    /// Ob gerade ein Durchlauf laeuft, der Dateiinhalte liest (C4.8 der
    /// Runde 11).
    ///
    /// Kommt aus `Tabinhalt::liest_inhalt` und traegt dessen beide
    /// Bedingungen: es laeuft ein Durchlauf, **und** der Inhaltsfilter wirkt.
    /// Ein reiner Namensdurchlauf ist damit falsch, und der Satz bei
    /// ausgeschaltetem "Content" zeichengleich mit dem der Runde 10.
    pub liest_inhalt: bool,
    /// Wie viele Dateien der Durchlauf wegen ihrer Groesse **nicht** gelesen
    /// hat.
    ///
    /// Null heisst: keine, und dann steht der Teil des Satzes, der sie nennt,
    /// nicht da. Kommt aus `Tabinhalt::zu_gross` und steht deshalb auch nach
    /// dem Ende des Laufs noch; bei einem kleinen Ordner ist der Lauf durch,
    /// bevor die Zeile das naechste Mal rechnet.
    pub zu_gross: u64,
}

/// Der fuenfte Rang der Statuszeile: der stehende Filtertext und was er von der
/// Liste uebrig laesst (C4 der Runde 10).
///
/// `None` heisst: dieser Rang meldet nichts. Zwei Wege fuehren dorthin, und
/// **beide stehen hier und nicht beim Aufrufer**, damit sie ohne Fenster
/// pruefbar sind. Steht kein Filtertext, ist nichts zu melden, und die Zeile
/// verhaelt sich wie vor jener Runde (C4.8 der Runde 10). Und solange ein
/// begonnener Lesevorgang noch nichts geliefert hat, stehen noch die Zeilen des
/// vorigen Ordners; eine Zahl daraus waere eine Auskunft ueber einen Ordner,
/// den der Nutzer schon verlassen hat (C4.7 der Runde 10).
///
/// **Der Satz hat einen Kern und drei Zusaetze, und jeder Zusatz steht nur
/// unter seiner Bedingung.** Der Kern nennt den Filtertext, die Zahl der
/// gezeigten Zeilen und die Zahl der Eintraege des angezeigten Ordners (C4.3
/// der Runde 10). Danach folgen, in dieser Reihenfolge:
///
/// ```text
/// Filter „notiz“: 38 von 4.812 angezeigt        Kern, immer
/// , Inhalt wird gelesen                         solange ein Inhaltsdurchlauf laeuft
/// , 12 Dateien zu groß                          wenn der Lauf Dateien uebergangen hat
/// , 3 Markierungen ausgeblendet                 wenn der Filter Markierungen verdeckt
/// ```
///
/// **Die Reihenfolge ist entschieden und nicht beliebig**
/// (`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_*_in-welcher-reihenfolge-stehen-die-satzteile-des-filterstands-und-was-faellt-im-schmalen-fenster-weg.md`,
/// Moeglichkeit 1). Der Lesehinweis steht unmittelbar hinter der Zahl, die er
/// einschraenkt: "38 von 4.812" ist waehrend eines Laufs eine Momentaufnahme,
/// und die beiden zu trennen hiesse, die Einschraenkung von der Aussage zu
/// loesen. Gekuerzt wird von hinten, also faellt im schmalen Fenster der
/// Markierungshinweis zuerst und der Lesehinweis zuletzt.
///
/// **Der Satz der Runde 10 aendert sich dadurch an keiner Stelle.** Beide
/// neuen Teile entstehen allein bei gesetztem "Content", und ohne sie steht
/// der Markierungshinweis wie bisher direkt hinter dem Kern.
///
/// **Gekuerzt wird von AppKit und nicht hier.** Eine zweite, kurze Fassung
/// jedes Satzteils entsteht nicht: die Zeile kuerzt am rechten Rand, und
/// `Statuszeile::kurzhinweis_nachziehen` haengt genau dann den vollen Satz
/// als Kurzhinweis an. Diese Funktion bleibt rein und ohne Fenster pruefbar;
/// eine Messung der Breite zoege das Fenster in sie hinein.
///
/// **Der Groessenhinweis ist mitentschieden und nicht optional**
/// (`shared/decisions/260816-1310_*_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md`).
/// Er ist der Rest der Antwort zur 1-MB-Grenze: ohne ihn haelt der Nutzer eine
/// nicht gefundene grosse Datei fuer nicht vorhanden, und genau diese
/// Verwechslung war der einzige ernsthafte Einwand gegen die Grenze.
///
/// **Kein siebter Rang und keine neue Farbregel** (C4.9, C4.10 der Runde 11).
/// Es bleibt bei einer Statuszeile, der Filterstand bleibt ein Rang, und
/// [`Rang::art`] rechnet fuer ihn weiter [`Art::Vorgang`]: ein Lesefortschritt
/// ist kein Fehler und wird nicht rot.
///
/// **Er ist die Gegenleistung dafuer, dass die Markierungsregel unter dem
/// Filter unveraendert bleibt.** Der Nutzer hat am 260814-1610 entschieden,
/// dass eine ausgeblendete Markierung fortbesteht und nicht wirkt
/// (`decisions/260814-1552_*_was-geschieht-mit-einer-markierung-die-der-filter-ausblendet.md`);
/// ohne diesen Satzteil muesste er erraten, dass es sie ueberhaupt gibt.
///
/// **Jede Zahl geht durch [`zahl`]** und traegt damit dieselben
/// Tausenderpunkte wie ein laufender Vorgang und der Markierungsstand daneben.
/// Ein zweites Zahlenformat entsteht nicht. Der Groessenhinweis hat dafuer
/// einen eigenen Singularzweig, genau wie der Markierungshinweis unter ihm.
///
/// **Sie steht hier und nicht in [`crate::kommandos`]**, wo
/// `auswahl::markierungsstand_text` fuer den Rang darunter steht. Jene braucht
/// einen Baustein aus `operationen` und gehoert zu C2 der Runde 1; diese
/// gehoert zu keiner Faehigkeit ausser der Zeile selbst und ist bei dem Rang,
/// den sie fuellt, besser aufgehoben als ohne ihn. AppKit ruft sie so wenig wie
/// jene, und beide sind ohne Fenster pruefbar.
pub fn filterstand_text(filtertext: &str, stand: Filterstand) -> Option<String> {
    // Dieselbe Frage wie `Ordnermodell::filter_steht`, an demselben Wert
    // gestellt: der Aufrufer reicht den Filtertext herein, statt die Antwort
    // getrennt mitzubringen.
    if filtertext.is_empty() || stand.ersetzt_beim_naechsten_stapel {
        return None;
    }
    let liest = if stand.liest_inhalt {
        ", Inhalt wird gelesen"
    } else {
        ""
    };
    // `zu_gross` zaehlt Dateien und kommt als `u64` vom Tab, weil der
    // `Durchlauf` es so fuehrt. `zahl` nimmt ein `usize`; auf dem Bauziel sind
    // beide gleich breit, und die Saettigung ist trotzdem ehrlicher als ein
    // Abschneiden.
    let zu_gross = match stand.zu_gross {
        0 => String::new(),
        1 => ", eine Datei zu groß".to_owned(),
        mehrere => format!(
            ", {} Dateien zu groß",
            zahl(usize::try_from(mehrere).unwrap_or(usize::MAX))
        ),
    };
    let ausgeblendet = match stand.ausgeblendete_markierungen {
        0 => String::new(),
        1 => ", eine Markierung ausgeblendet".to_owned(),
        mehrere => format!(", {} Markierungen ausgeblendet", zahl(mehrere)),
    };
    Some(format!(
        "Filter \u{201e}{filtertext}\u{201c}: {} von {} angezeigt{liest}{zu_gross}{ausgeblendet}",
        zahl(stand.gezeigt),
        zahl(stand.vorhanden)
    ))
}

/// Der Satz des Seitenzaehlers: "Seite N von M" (C4.1 der Runde 20).
///
/// **Jede Zahl geht durch [`zahl`]** und traegt damit dieselben
/// Tausenderpunkte wie ein laufender Vorgang, der Filterstand und der
/// Markierungsstand. Ein zweites Zahlenformat entsteht nicht.
///
/// **Sie steht hier und nicht bei der Vorschau**, aus demselben Grund wie
/// [`filterstand_text`] darueber: der Satz gehoert zu keiner Faehigkeit ausser
/// der Zeile selbst und ist bei dem Rang, den er fuellt, besser aufgehoben als
/// ohne ihn. `Vorschaufenster::seitenzaehler` ruft sie und rechnet nichts
/// selbst; welche Seite die aktuelle ist, sagt PDFKit (C4.3), und wie sie
/// heisst, sagt diese Zeile.
pub fn seitenzaehler_text(aktuell: usize, gesamt: usize) -> String {
    format!("Seite {} von {}", zahl(aktuell), zahl(gesamt))
}

/// Woher eine Meldung kommt: aus einem der zwei Dateifenster oder aus dem
/// Vorschaufenster.
///
/// Die Herkunft einer **Meldung** und nicht eines Rangs: [`Herkunftsart`]
/// sagt, wer einen Rang tragen kann, dieser Wert sagt, wer ihn gerade
/// gewonnen hat, und beim Dateifenster gehoert die Seite dazu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunft {
    /// Das genannte Dateifenster.
    Dateifenster(Fensterseite),
    /// Das Vorschaufenster, das zu keiner Seite gehoert.
    Vorschau,
}

/// Die eine Aussage, die von allen Bewerbern jetzt in der Zeile steht.
///
/// Sie traegt ihre Herkunft mit: [`zeilentext`] braucht sie, um zu
/// entscheiden, ob der Satz ein Dateifenster nennen muss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meldung<'a> {
    /// Der Bereich, aus dem die Meldung kommt.
    pub herkunft: Herkunft,
    /// Der Rang, auf dem sie gewonnen hat.
    pub rang: Rang,
    /// Der Text, wie ihn die Quelle gesetzt hat, ohne Zusatz.
    pub text: &'a str,
    /// Ob sie rot erscheint; gerechnet aus [`Rang::art`].
    pub art: Art,
}

/// Was von allen Bewerbern jetzt in der Zeile steht: zwei je
/// Dateifenster-Rang und einer fuer den Vorschau-Rang.
///
/// **Die eine Regel, und kein Sonderfall je Meldungsart.** Die Zeile traegt
/// einen Text. Steht mehr als eine Aussage, gewinnt die, die dem letzten Tun
/// des Nutzers am naechsten ist:
///
/// ```text
/// 1  Befehlsantwort    was KRK auf einen Tastenbefehl zu sagen hat
/// 2  Vorgangsanzeige   der Stand einer laufenden Operation
/// 3  Fenstermeldung    ein Ereignis am Fenster, das niemand angefordert hat
/// 4  Tabmeldung        der Zustand des sichtbaren Ordners
/// 5  Filterstand       der stehende Filtertext und was er uebrig laesst
/// 6  Seitenzaehler     die Seite des PDF, das die Vorschau zeigt
/// 7  Markierungsstand  was im sichtbaren Tab markiert ist
/// ```
///
/// **Der Seitenzaehler steht unter dem Filterstand und ueber dem
/// Markierungsstand** (A5 der Runde 20). Ein stehender Filtertext verdraengt
/// ihn und laesst ihn nach dem Fallen zurueck (C4.5); Vorgangsanzeige,
/// Befehlsantwort und Fenstermeldung stehen ueber ihm wie ueber jeder anderen
/// Auskunft (C4.6). Er ist der eine Rang, den kein Dateifenster traegt: seinen
/// Text reicht der Aufrufer als `vorschau` herein, geholt beim
/// Vorschaufenster, und `None` heisst, dass es kein PDF zeigt (C4.4).
///
/// **Der Filterstand steht ueber dem Markierungsstand und unter der
/// Tabmeldung** (C4.1). Beide unteren beschreiben einen Zustand des sichtbaren
/// Tabs, und bei stehendem Filter treten sie gegeneinander an; der Nutzer
/// filtert und markiert im selben Augenblick. Vorn steht die Auskunft, ohne die
/// er das Fehlen eines Eintrags fuer einen Defekt haelt, und nicht die Zahl,
/// die er durch Hinsehen abschaetzen kann. **Verloren ist der Markierungsstand
/// dabei nicht**: der Filterstand nennt selbst, wie viele Markierungen der
/// Filter gerade ausblendet, und ohne Filtertext meldet er nichts, sodass die
/// Zeile sich verhaelt wie vor dieser Runde (C4.4, C4.8).
///
/// **Der Markierungsstand steht unter der Tabmeldung und nicht neben ihr.**
/// Beide beschreiben einen Zustand des sichtbaren Tabs, aber mit
/// verschiedenen Lebensdauern: die Tabmeldung traegt einen Ordner, der sich
/// nicht lesen liess, und muss stehen bleiben, waehrend der Nutzer markiert
/// und die Markierung wieder aufhebt. Beide in ein Feld zu legen gaebe diesem
/// Feld zwei Loeschregeln, und das ist der Sonderfall, den diese Datei fuer
/// Befehlsantwort und Fenstermeldung schon einmal ausgeschlossen hat. Unter
/// der Tabmeldung steht er, weil ein nicht lesbarer Ordner ein Fehler ist und
/// eine Markierungszahl keiner; er ist der Ruhezustand der Zeile, und ein
/// Ruhezustand ist der unterste Rang.
///
/// Das ist dieselbe Ordnung, die S14 zwischen Fenster- und Tabmeldung gezogen
/// hat ("ein Ereignis ist neuer als ein Zustand"), zu Ende gefuehrt: eine
/// laufende Operation ist neuer als ein Ereignis, und die Antwort auf einen
/// Tastendruck, den der Nutzer eben gemacht hat, ist neuer als beides. S16b
/// hatte sie mit drei Raengen gebaut und die Befehlsantwort in die
/// Fenstermeldung gelegt; dort verschwand die Meldung "es laeuft bereits eine
/// Operation" hinter dem Fortschritt desselben Dateifensters
/// (`issues/260804-1915_*_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`).
///
/// **Verdraengt wird nichts geloescht.** Jede der acht Quellen mit eigenem Feld
/// haelt ihren Text dort, und jedes Feld hat genau eine Loeschregel; die
/// gerechneten Raenge koennen gar nicht veralten. Eine verdraengte Aussage
/// erscheint, sobald alles ueber ihr gefallen ist: die Auswurfmeldung, die
/// waehrend einer Kopie eintrifft, steht auf Rang 3, wartet die Kopie und deren
/// Abschlusstext (Rang 1) ab und ist mit dem naechsten Tastenbefehl in der
/// Zeile. Ein Zeitgeber ist dafuer nicht noetig, weil jede Lebensdauer an einem
/// Ereignis haengt und an keiner Uhr.
///
/// # Die zweite Stelle der Ordnung: erst der Rang, dann die aktive Seite
///
/// Seit der Runde 6 gibt es eine Zeile fuer zwei Dateifenster, und damit
/// doppelt so viele Bewerber wie Raenge. **Die Ordnung ist zweistellig: zuerst entscheidet der
/// Rang, und erst bei gleichem Rang die aktive Seite.** Eine Fenstermeldung des
/// inaktiven Dateifensters steht damit ueber einer Markierungszahl des aktiven,
/// und zwei laufende Vorgaenge entscheidet die aktive Seite.
///
/// Sie ist ueber alle Bewerber vollstaendig und ueberschneidungsfrei,
/// und nicht aus Sorgfalt, sondern der Bauart nach: zwei Bewerber desselben
/// Ranges gehoeren immer verschiedenen Seiten, also entscheidet die zweite
/// Stelle jeden Gleichstand der ersten, und der Vorschau-Rang hat nur einen
/// Bewerber und damit keinen Gleichstand. **Die Ordnung steht deshalb in der
/// Schleifenreihenfolge und nicht in einer Vergleichsfunktion** — aussen die
/// Raenge aus [`Rang::ALLE`], innen die aktive Seite vor der anderen.
///
/// **Der Preis ist benannt und vom Nutzer am 260812-1105 angenommen: laufen in
/// beiden Dateifenstern zugleich Vorgaenge, ist nur der des aktiven zu sehen.**
/// Das ist neu gegenueber zwei Zeilen, an denen jeder Fortschritt seinen
/// eigenen Platz hatte. Verloren ist der zweite nicht — sein Feld steht, und er
/// erscheint, sobald der erste faellt —, aber waehrend beide laufen, zeigt die
/// Zeile einen von beiden. Die Gegenrechnung dazu ist die Zuordnung im Satz:
/// welches Dateifenster gemeint ist, sagt seit dieser Runde [`zeilentext`],
/// nicht mehr die Lage der Zeile.
///
/// # Ein ausgeblendetes Dateifenster bewirbt sich nicht
///
/// Angetreten wird nur mit den Quellen der **sichtbaren** Dateifenster. Bis zum
/// 260812 traten beide an, und dann konnte in der Zeile dauerhaft eine Meldung
/// ueber einen Bereich stehen, den der Nutzer nicht sieht — mit dem Zusatz
/// "rechtes Dateifenster: …" davor, obwohl nur eines dasteht
/// (`issues/260812-1805_*_die-eine-statuszeile-zeigt-meldungen-eines-ausgeblendeten-dateifensters.md`).
/// Der zweite Satz von C5.8 sagt das Gegenteil zu.
///
/// **Bis zur Runde 6 trug die Lage der Zeile diese Zusage.** Jede Zeile sass am
/// Fuss ihres Dateifensters und ging mit ihm; die Zusammenlegung hat die
/// Kopplung geloest, und diese Bedingung nimmt sie wieder auf.
///
/// **Verloren geht dabei nichts** (C5.7). Die vier Felder der ausgeblendeten
/// Seite bleiben unangetastet stehen, und ihre Meldung ist wieder da, sobald
/// der Nutzer den Bereich einblendet.
///
/// **Die Sichtbarkeit steht in der Signatur und nicht beim Aufrufer.** Eine
/// Bedingung, die `Anwendungsdelegierter::statuszeile_nachziehen` vor dem
/// Aufruf zoege, waere von keiner Probe dieser Datei zu erreichen; hier ist die
/// Regel ohne Fenster pruefbar. Welches Feld von [`Sichtbarkeit`] zu einer
/// [`Fensterseite`] gehoert, sagt [`sichtbar_in`] ueber [`Bereich::von_seite`] —
/// eine zweite Zuordnung daneben waere genau die, die
/// [`sichtbar_in`] als die eine abgeloest hat.
///
/// **Dieselbe Regel gilt der Vorschau** (Runde 20): eine ausgeblendete
/// Vorschau bewirbt sich nicht, gefragt mit derselben Funktion und
/// [`Bereich::Vorschau`]. Ein Seitenzaehler ueber einen Bereich, den der
/// Nutzer nicht sieht, waere derselbe Defekt wie der vom 260812-1805, nur
/// ohne Namenszusatz.
///
/// **Das aktive Dateifenster ist immer sichtbar**, und deshalb liefert diese
/// Funktion nie eine Meldung, der [`zeilentext`] einen Zusatz voranstellen
/// muesste, waehrend nur ein Dateifenster dasteht. Hergestellt wird die Zusage
/// im Modell, an beiden Wegen dorthin:
/// [`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)
/// gibt die Aktivitaet ab, wenn das aktive ausgeblendet wird, und
/// [`Fenstermodell::aus_sitzung`](crate::fenstermodell::Fenstermodell::aus_sitzung)
/// zieht sie nach, wenn eine von Hand geschriebene `session.toml` sie auf ein
/// ausgeblendetes zeigen laesst.
pub fn zeile<'a>(
    links: &'a Quellen,
    rechts: &'a Quellen,
    aktiv: Fensterseite,
    sichtbar: &Sichtbarkeit,
    vorschau: Option<&'a str>,
) -> Option<Meldung<'a>> {
    let quellen = |seite: Fensterseite| match seite {
        Fensterseite::Links => links,
        Fensterseite::Rechts => rechts,
    };
    for rang in Rang::ALLE {
        match rang.herkunft() {
            Herkunftsart::Dateifenster => {
                // Die aktive Seite zuerst: die zweite Stelle der Ordnung.
                for seite in [aktiv, aktiv.andere()] {
                    // Wer nicht dasteht, sagt nichts. Die Begruendung steht
                    // im Kopf dieser Funktion.
                    if !sichtbar_in(sichtbar, Bereich::von_seite(seite)) {
                        continue;
                    }
                    if let Some(text) = quellen(seite).text(rang) {
                        return Some(Meldung {
                            herkunft: Herkunft::Dateifenster(seite),
                            rang,
                            text,
                            art: rang.art(),
                        });
                    }
                }
            }
            Herkunftsart::Vorschau => {
                // Ein Bewerber, keine zweite Stelle; und eine ausgeblendete
                // Vorschau bewirbt sich so wenig wie ein ausgeblendetes
                // Dateifenster.
                if !sichtbar_in(sichtbar, Bereich::Vorschau) {
                    continue;
                }
                if let Some(text) = vorschau {
                    return Some(Meldung {
                        herkunft: Herkunft::Vorschau,
                        rang,
                        text,
                        art: rang.art(),
                    });
                }
            }
        }
    }
    None
}

/// Der Satz, wie er in der Zeile steht: mit dem Namen des Dateifensters, wenn
/// die Meldung nicht vom aktiven kommt.
///
/// **Die Zuordnung ist seit der Runde 6 sprachlich und nicht mehr raeumlich.**
/// Bis dahin sagte die Lage der Zeile, zu welchem Dateifenster eine Meldung
/// gehoert; mit einer Zeile ueber die volle Breite sagt es der Satz. Der Nutzer
/// hat diesen Tausch am 260812-1105 angenommen.
///
/// **Genannt wird die Seite genau dann, wenn sie nicht die aktive ist**, und
/// das ist eine Regel und keine zwei. Die Meldung des Dateifensters, mit dem
/// der Nutzer gerade arbeitet, ist der Normalfall und traegt keinen Zusatz;
/// jede andere sagt, woher sie kommt.
///
/// **Der Fall "es steht nur ein Dateifenster" braucht hier keinen eigenen
/// Zweig, aber er wird auch nicht hier eingeloest**, sondern in [`zeile`]: dort
/// bewirbt sich ein ausgeblendetes Dateifenster nicht, und das aktive ist immer
/// sichtbar. Zusammen heisst das, dass diese Funktion nie eine Meldung von
/// einer Seite bekommt, die der Nutzer nicht sieht. Bis zum 260812 stand hier
/// die Behauptung, der Fall folge schon aus der einen Bedingung; er folgte
/// nicht
/// (`issues/260812-1805_*_die-eine-statuszeile-zeigt-meldungen-eines-ausgeblendeten-dateifensters.md`).
///
/// **Eine Meldung aus der Vorschau traegt nie einen Zusatz** (Runde 20). Die
/// Regel bleibt eine: genannt wird ein Dateifenster, das nicht das aktive
/// ist, und die Vorschau ist keines. "Seite 3 von 9" steht deshalb ohne
/// Vorsatz, gleich welches Dateifenster aktiv ist; einen Namen fuer den
/// Bereich gibt es hier nicht, weil es nur eine Vorschau gibt und der Satz
/// selbst sagt, wovon er spricht.
///
/// Die beiden Namen stehen hier und nicht im Kern: es sind Anzeigetexte, und
/// [`Fensterseite`] ist ein Wert der Ablage, der von Anzeige nichts weiss.
pub fn zeilentext(meldung: &Meldung<'_>, aktiv: Fensterseite) -> String {
    match meldung.herkunft {
        Herkunft::Dateifenster(seite) if seite != aktiv => {
            format!("{}: {}", seitenname(seite), meldung.text)
        }
        Herkunft::Dateifenster(_) | Herkunft::Vorschau => meldung.text.to_owned(),
    }
}

/// Wie ein Dateifenster im Satz heisst.
///
/// Eine vollstaendige Fallunterscheidung ueber [`Fensterseite`]; ein dritter
/// Wert haelt den Bau an.
const fn seitenname(seite: Fensterseite) -> &'static str {
    match seite {
        Fensterseite::Links => "linkes Dateifenster",
        Fensterseite::Rechts => "rechtes Dateifenster",
    }
}

/// Die eine Textzeile am Fensterfuss.
///
/// **Eine Ansicht und nicht zwei.** Schritt 11 der Runde 6 hatte das Feld in
/// eine `NSScrollView` gesetzt; der Nutzerentscheid vom 260812 hat den Schritt
/// zurueckgenommen. Wie eine Meldung lesbar wird, die breiter ist als das
/// Fenster, steht im Modulkopf.
pub struct Statuszeile {
    feld: Retained<NSTextField>,
}

impl Statuszeile {
    /// Baut eine leere Statuszeile.
    ///
    /// Sie entsteht ohne Breite und bekommt ihre erste beim Einhaengen;
    /// [`super::fenster::fensterinhalt`] setzt Rahmen und Autogroesse in
    /// denselben zwei Zeilen wie fuer die Bereichsleiste daneben. Bis zur
    /// Runde 6 setzte diese Methode die Maske selbst, weil sie die Zeile an den
    /// Fuss eines Dateifensters band; jetzt haengt die Zeile am Fenster, und
    /// die Wahl gehoert dorthin, wo eingehaengt wird.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
        let feld = NSTextField::labelWithString(ns_string!(""), mtm);
        feld.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(0.0, HOEHE)));
        feld.setFont(Some(&NSFont::systemFontOfSize(
            NSFont::smallSystemFontSize(),
        )));
        feld.setTextColor(Some(&NSColor::secondaryLabelColor()));
        feld.setAlignment(NSTextAlignment::Left);
        feld.setMaximumNumberOfLines(1);
        Self { feld }
    }

    /// Die Ansicht, die in die Inhaltsflaeche des Fensters gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.feld
    }

    /// Zeigt eine Meldung der genannten Art an, oder leert die Zeile bei
    /// `None`.
    ///
    /// Ein Fehler faerbt die Zeile rot: ein Ordner ohne Leserecht ist ein
    /// Fehler und kein Hinweis, und eine graue Zeile am Fuss uebersieht der
    /// Nutzer neben einer leeren Liste genauso, wie er die
    /// Standardfehlerausgabe uebersehen hat. Ein Fortschritt bekommt die
    /// gewoehnliche Textfarbe; auffindbar ist der Abbruch bei ihm nicht ueber
    /// die Farbe, sondern weil die Zeile ihn benennt ("Esc bricht ab").
    ///
    /// **Jeder neue Text zieht den Kurzhinweis nach**, und der Ruf steht hinter
    /// beiden Zweigen und nicht in ihnen: eine geleerte Zeile hat nichts
    /// abzuschneiden, also nimmt derselbe Nachzug den Hinweis dort weg, ohne
    /// dass er einen eigenen Zweig braucht.
    pub fn zeigen(&self, meldung: Option<(&str, Art)>) {
        match meldung {
            Some((text, art)) => {
                self.feld.setStringValue(&NSString::from_str(text));
                let farbe = match art {
                    Art::Fehler => NSColor::systemRedColor(),
                    Art::Vorgang => NSColor::labelColor(),
                };
                self.feld.setTextColor(Some(&farbe));
            }
            None => {
                self.feld.setStringValue(ns_string!(""));
                self.feld
                    .setTextColor(Some(&NSColor::secondaryLabelColor()));
            }
        }
        self.kurzhinweis_nachziehen();
    }

    /// Setzt den Kurzhinweis, wenn der Text nicht ganz in die Zeile passt, und
    /// nimmt ihn sonst wieder weg.
    ///
    /// **Der Hinweis kommt aus dem Feld und nicht aus dem Aufrufer.** Was er
    /// zeigt, ist damit der Bauart nach genau das, was in der Zeile steht,
    /// statt ein zweites Argument, das damit uebereinstimmen muesste.
    ///
    /// **Der Zweig `None` ist keine Zierde.** Ohne ihn bliebe der Hinweis einer
    /// laengeren Vorgaengermeldung stehen und zeigte beim Verweilen einen Satz,
    /// den die Zeile nicht mehr traegt.
    fn kurzhinweis_nachziehen(&self) {
        let voll = self.feld.stringValue();
        self.feld.setToolTip(self.abgeschnitten().then_some(&*voll));
    }

    /// Ob der Text breiter ist als die Zeile, die ihn zeigen soll.
    ///
    /// **Die Messung aus Schritt 11, weitergenutzt statt neu geschrieben.**
    /// Dort brachte `sizeToFit` die Dokumentansicht auf die Breite ihres
    /// Textes; hier fragt derselbe Ruf nur noch, wie breit der Text **waere**.
    /// Der Umweg ueber den Rahmen gehoert dazu: `sizeToFit` ist die eine
    /// Stelle, an der AppKit diese Breite herausgibt, und sie arbeitet
    /// schreibend. Der Rahmen von vorher kommt deshalb unmittelbar danach
    /// zurueck, und zwar ganz — `sizeToFit` schreibt auch die Hoehe, und die
    /// Schrifthoehe der kleinen Systemschrift liegt unter [`HOEHE`].
    ///
    /// **Verglichen wird gegen die Breite, die das Feld im Fenster hat**, und
    /// die steht in seinem Rahmen: [`super::fenster::fensterinhalt`] setzt sie
    /// beim Aufbau, die Autogroesse zieht sie bei jeder Fensteraenderung nach.
    /// Beide Zahlen kommen aus derselben Zelle, also entscheidet der Vergleich
    /// dieselbe Frage, die AppKit beim Kuerzen entscheidet.
    fn abgeschnitten(&self) -> bool {
        let rahmen = self.feld.frame();
        self.feld.sizeToFit();
        let textbreite = self.feld.frame().size.width;
        self.feld.setFrame(rahmen);
        textbreite > rahmen.size.width
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Art, Bereich, Filterstand, Herkunft, Herkunftsart, Meldung, Quellen, Rang,
        filterstand_text, seitenzaehler_text, sichtbar_in, zeile, zeilentext,
    };
    use krk_core::ablage::{Fensterseite, Sichtbarkeit};

    /// Beide Dateifenster stehen: die Lage, in der die Zeile fast immer
    /// schreibt, und der Auslieferungszustand der Ablage.
    fn beide() -> Sichtbarkeit {
        Sichtbarkeit::default()
    }

    /// Nur das genannte Dateifenster steht, das andere ist ausgeblendet.
    ///
    /// **Die beiden Zusicherungen darunter binden den Helfer an die eine
    /// Zuordnung**, ueber die auch [`zeile`] fragt. Ohne sie schriebe die Probe
    /// die Zuordnung von einer [`Fensterseite`] auf ihr Feld ein zweites Mal
    /// auf und koennte gegen eine falsche pruefen.
    fn steht_nur(seite: Fensterseite) -> Sichtbarkeit {
        let mut sichtbar = Sichtbarkeit::default();
        match seite.andere() {
            Fensterseite::Links => sichtbar.erstes_dateifenster = false,
            Fensterseite::Rechts => sichtbar.zweites_dateifenster = false,
        }
        assert!(sichtbar_in(&sichtbar, Bereich::von_seite(seite)));
        assert!(!sichtbar_in(&sichtbar, Bereich::von_seite(seite.andere())));
        sichtbar
    }

    /// Ein Quellensatz mit genau einem gesetzten Rang.
    fn nur(rang: Rang, text: &str) -> Quellen {
        let mut quellen = Quellen::default();
        let feld = match rang {
            Rang::Befehlsantwort => &mut quellen.befehlsantwort,
            Rang::Vorgangsanzeige => &mut quellen.vorgangsanzeige,
            Rang::Fenstermeldung => &mut quellen.fenstermeldung,
            Rang::Tabmeldung => &mut quellen.tabmeldung,
            Rang::Filterstand => &mut quellen.filterstand,
            Rang::Seitenzaehler => {
                panic!(
                    "kein Dateifenster traegt den Seitenzaehler; die Probe fragt `dateifenster_raenge`"
                )
            }
            Rang::Markierungsstand => &mut quellen.markierungsstand,
        };
        *feld = Some(text.to_owned());
        quellen
    }

    /// Die Raenge, die ein Dateifenster traegt, in der Rangfolge.
    ///
    /// Aus [`Rang::herkunft`] gelesen und nicht danebengeschrieben: die Proben
    /// ueber "jeden Rang eines Dateifensters" laufen hierueber, damit sie den
    /// Vorschau-Rang nicht in ein Dateifenster schreiben und nicht wissen
    /// muessen, welcher es ist.
    fn dateifenster_raenge() -> impl Iterator<Item = Rang> {
        Rang::ALLE
            .into_iter()
            .filter(|rang| rang.herkunft() == Herkunftsart::Dateifenster)
    }

    /// Die alte Fassung von `zeile`: die vier Felder und der Markierungsstand
    /// eines Dateifensters, das zugleich das aktive ist. Die acht Proben der
    /// Runde 1 treffen darueber dieselben Aussagen wie vorher.
    ///
    /// **Der Filterstand aus C4 der Runde 10 steht nicht in der Liste**, und
    /// das ist Absicht: er hat seine eigenen Proben weiter unten, und diese
    /// Helferliste ist die der Runde 1. Ein sechster Parameter haette jede
    /// ihrer acht Proben angefasst, ohne an einer davon etwas zu pruefen.
    fn allein(
        befehlsantwort: Option<&str>,
        vorgangsanzeige: Option<&str>,
        fenstermeldung: Option<&str>,
        tabmeldung: Option<&str>,
        markierungsstand: Option<&str>,
    ) -> Option<(String, Art)> {
        let links = Quellen {
            befehlsantwort: befehlsantwort.map(str::to_owned),
            vorgangsanzeige: vorgangsanzeige.map(str::to_owned),
            fenstermeldung: fenstermeldung.map(str::to_owned),
            tabmeldung: tabmeldung.map(str::to_owned),
            filterstand: None,
            markierungsstand: markierungsstand.map(str::to_owned),
        };
        let rechts = Quellen::default();
        zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .map(|meldung| (meldung.text.to_owned(), meldung.art))
    }

    /// Wie `allein`, nur mit dem Satz, wie er wirklich in der Zeile steht.
    fn text_von(links: &Quellen, rechts: &Quellen, aktiv: Fensterseite) -> Option<String> {
        text_von_bei(links, rechts, aktiv, &beide())
    }

    /// Wie `text_von`, nur fuer eine Lage, in der nicht beide Dateifenster
    /// stehen.
    fn text_von_bei(
        links: &Quellen,
        rechts: &Quellen,
        aktiv: Fensterseite,
        sichtbar: &Sichtbarkeit,
    ) -> Option<String> {
        zeile(links, rechts, aktiv, sichtbar, None).map(|meldung| zeilentext(&meldung, aktiv))
    }

    fn erwartet(meldung: Option<(String, Art)>, text: &str, art: Art) {
        assert_eq!(meldung, Some((text.to_owned(), art)));
    }

    // ------------------------------------------------------------------
    // Die acht Proben der Runde 1, in der neuen Form
    // ------------------------------------------------------------------

    #[test]
    fn ohne_jede_quelle_bleibt_die_zeile_leer() {
        assert_eq!(allein(None, None, None, None, None), None);
        assert_eq!(
            zeile(
                &Quellen::default(),
                &Quellen::default(),
                Fensterseite::Rechts,
                &beide(),
                None
            ),
            None
        );
    }

    #[test]
    fn jede_quelle_steht_fuer_sich_allein_in_der_zeile() {
        erwartet(
            allein(Some("Antwort"), None, None, None, None),
            "Antwort",
            Art::Fehler,
        );
        erwartet(
            allein(None, Some("Vorgang"), None, None, None),
            "Vorgang",
            Art::Vorgang,
        );
        erwartet(
            allein(None, None, Some("Ereignis"), None, None),
            "Ereignis",
            Art::Fehler,
        );
        erwartet(
            allein(None, None, None, Some("Zustand"), None),
            "Zustand",
            Art::Fehler,
        );
    }

    /// Der Defekt vom 260804-1915: der zweite F5 meldete sich in dem
    /// Dateifenster, in dem der Fortschritt stand, und war dort unsichtbar.
    #[test]
    fn die_befehlsantwort_steht_ueber_dem_laufenden_vorgang() {
        erwartet(
            allein(
                Some("es läuft bereits eine Operation: Kopieren"),
                Some("Kopieren: 8.189 Einträge …"),
                None,
                None,
                None,
            ),
            "es läuft bereits eine Operation: Kopieren",
            Art::Fehler,
        );
    }

    #[test]
    fn der_laufende_vorgang_steht_ueber_ereignis_und_zustand() {
        erwartet(
            allein(
                None,
                Some("Kopieren: 8.189 Einträge …"),
                Some("Datenträger ausgeworfen"),
                Some("Ordner nicht lesbar"),
                Some("12 markiert, davon 3 Ordner, 4,2 MB"),
            ),
            "Kopieren: 8.189 Einträge …",
            Art::Vorgang,
        );
    }

    #[test]
    fn das_ereignis_am_fenster_steht_ueber_dem_zustand_des_ordners() {
        erwartet(
            allein(
                None,
                None,
                Some("Datenträger ausgeworfen"),
                Some("Ordner nicht lesbar"),
                Some("12 markiert, davon 3 Ordner, 4,2 MB"),
            ),
            "Datenträger ausgeworfen",
            Art::Fehler,
        );
    }

    /// Der Defekt vom 260804-1915: der Abschlusstext ueberschrieb die waehrend
    /// der Kopie eingetroffene Auswurfmeldung. Er verdeckt sie jetzt, und sie
    /// steht wieder da, sobald er mit dem naechsten Befehl faellt.
    #[test]
    fn die_verdraengte_auswurfmeldung_erscheint_nach_dem_abschlusstext() {
        let auswurf = "Datenträger „Sicherung“ wurde ausgeworfen";
        let abschluss = "Kopieren abgebrochen: 9.175 Einträge übertragen";
        // Waehrend der Kopie: der Fortschritt gewinnt, die Auswurfmeldung
        // bleibt in ihrem Feld stehen.
        erwartet(
            allein(
                None,
                Some("Kopieren: 9.131 Einträge …"),
                Some(auswurf),
                None,
                None,
            ),
            "Kopieren: 9.131 Einträge …",
            Art::Vorgang,
        );
        // Unmittelbar nach dem Bericht: der Abschlusstext ist die Antwort auf
        // den Befehl und steht oben.
        erwartet(
            allein(Some(abschluss), None, Some(auswurf), None, None),
            abschluss,
            Art::Fehler,
        );
        // Der naechste Tastenbefehl raeumt die Antwort weg; jetzt ist die
        // Auswurfmeldung an der Reihe, statt verloren zu sein.
        erwartet(
            allein(None, None, Some(auswurf), None, None),
            auswurf,
            Art::Fehler,
        );
    }

    /// Der Markierungsstand aus S16c: er steht unter der Tabmeldung.
    #[test]
    fn der_markierungsstand_steht_hinter_der_tabmeldung() {
        let markiert = "12 markiert, davon 3 Ordner, 4,2 MB";
        assert_eq!(
            allein(
                None,
                None,
                None,
                Some("Ordner nicht lesbar"),
                Some(markiert)
            ),
            Some(("Ordner nicht lesbar".to_owned(), Art::Fehler)),
            "ein nicht lesbarer Ordner ist wichtiger als eine Markierungszahl"
        );
        assert_eq!(
            allein(None, None, None, None, Some(markiert)),
            Some((markiert.to_owned(), Art::Vorgang)),
            "ohne Tabmeldung steht der Markierungsstand in der Zeile"
        );
    }

    /// Eine Markierungszahl ist kein Fehler und wird deshalb nicht rot.
    #[test]
    fn der_markierungsstand_gilt_nicht_als_fehler() {
        let (_, art) = allein(
            None,
            None,
            None,
            None,
            Some("3 markiert, davon 0 Ordner, 6 KB"),
        )
        .expect("der Markierungsstand steht als einzige Quelle in der Zeile");
        assert_eq!(art, Art::Vorgang);
        assert_ne!(art, Art::Fehler);
    }

    // ------------------------------------------------------------------
    // Der fuenfte Rang: der Filterstand (C4, Runde 10)
    // ------------------------------------------------------------------

    /// Die Stelle eines Ranges in der Rangfolge, aus [`Rang::ALLE`] gelesen
    /// und nicht danebengeschrieben.
    fn stelle(gesucht: Rang) -> usize {
        Rang::ALLE
            .iter()
            .position(|rang| *rang == gesucht)
            .expect("jeder Rang steht in der Rangfolge")
    }

    /// Ein Filterstand mit drei Zahlen, ohne ausstehenden Ersatz und ohne
    /// Inhaltsdurchlauf.
    ///
    /// Der Stand der Runde 10: kein Lesehinweis, keine zu grosse Datei. Die
    /// Proben jener Runde geben ihn unveraendert weiter und pruefen damit
    /// zugleich, dass ihr Satz zeichengleich geblieben ist.
    fn stand(gezeigt: usize, vorhanden: usize, ausgeblendet: usize) -> Filterstand {
        Filterstand {
            gezeigt,
            vorhanden,
            ausgeblendete_markierungen: ausgeblendet,
            ersetzt_beim_naechsten_stapel: false,
            liest_inhalt: false,
            zu_gross: 0,
        }
    }

    /// C4.1: er steht ueber dem Markierungsstand und unter der Tabmeldung,
    /// und zwar in der Rangfolge selbst und in der Auswahl, die daraus faellt.
    #[test]
    fn der_filterstand_steht_zwischen_tabmeldung_und_markierungsstand() {
        assert_eq!(Rang::ALLE.len(), 7);
        assert!(stelle(Rang::Tabmeldung) < stelle(Rang::Filterstand));
        assert!(stelle(Rang::Filterstand) < stelle(Rang::Seitenzaehler));
        assert!(stelle(Rang::Seitenzaehler) < stelle(Rang::Markierungsstand));

        let leer = Quellen::default();
        let mut quellen = nur(Rang::Filterstand, "Filter „rs“: 12 von 340 angezeigt");
        quellen.markierungsstand = Some("12 markiert, davon 3 Ordner, 4,2 MB".to_owned());
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide(), None)
            .expect("zwei Raenge melden etwas");
        assert_eq!(
            meldung.rang,
            Rang::Filterstand,
            "eine verkuerzte Liste wiegt schwerer als die Markierungszahl"
        );

        quellen.tabmeldung = Some("Ordner nicht lesbar".to_owned());
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide(), None)
            .expect("drei Raenge melden etwas");
        assert_eq!(
            meldung.rang,
            Rang::Tabmeldung,
            "ein nicht lesbarer Ordner ist wichtiger als eine Filterzahl"
        );
    }

    /// C4.2: eine Filterzahl ist kein Fehler und wird nicht rot.
    #[test]
    fn der_filterstand_gilt_nicht_als_fehler() {
        assert_eq!(Rang::Filterstand.art(), Art::Vorgang);
        assert_ne!(Rang::Filterstand.art(), Art::Fehler);
        let quellen = nur(Rang::Filterstand, "Filter „rs“: 12 von 340 angezeigt");
        let leer = Quellen::default();
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide(), None)
            .expect("der Filterstand steht als einzige Quelle in der Zeile");
        assert_eq!(meldung.art, Art::Vorgang);
    }

    /// C4.3: der Satz nennt den Filtertext, die gezeigten und die vorhandenen
    /// Eintraege, und die Zahlen tragen dieselben Tausenderpunkte wie ein
    /// laufender Vorgang.
    #[test]
    fn der_satz_nennt_filtertext_gezeigte_und_vorhandene() {
        assert_eq!(
            filterstand_text("rs", stand(38, 4_812, 0)).as_deref(),
            Some("Filter „rs“: 38 von 4.812 angezeigt")
        );
    }

    /// C4.4: die ausgeblendeten Markierungen stehen daneben, und ohne sie
    /// steht dieser Teil nicht da.
    #[test]
    fn ausgeblendete_markierungen_stehen_daneben_und_sonst_nicht() {
        assert_eq!(
            filterstand_text("rs", stand(38, 412, 0)).as_deref(),
            Some("Filter „rs“: 38 von 412 angezeigt"),
            "ohne ausgeblendete Markierung steht der vierte Teil nicht da"
        );
        assert_eq!(
            filterstand_text("rs", stand(38, 412, 1)).as_deref(),
            Some("Filter „rs“: 38 von 412 angezeigt, eine Markierung ausgeblendet")
        );
        assert_eq!(
            filterstand_text("rs", stand(38, 412, 2_500)).as_deref(),
            Some("Filter „rs“: 38 von 412 angezeigt, 2.500 Markierungen ausgeblendet")
        );
    }

    /// C4.5 und C4.6: die linke Zahl ist die der gezeigten Zeilen und waechst
    /// mit ihnen, die rechte steht. Gezaehlt werden entschiedene Zeilen; das
    /// Wort "Treffer" kommt in der Zeile nicht vor, denn eine Trefferzahl gibt
    /// es nicht.
    #[test]
    fn die_linke_zahl_waechst_und_zaehlt_zeilen_und_keine_treffer() {
        let mut vorher = 0;
        for gezeigt in [3_usize, 17, 240] {
            let satz = filterstand_text("rs", stand(gezeigt, 412, 0))
                .expect("bei stehendem Filtertext meldet der Rang etwas");
            assert_eq!(satz, format!("Filter „rs“: {gezeigt} von 412 angezeigt"));
            assert!(!satz.contains("Treffer"));
            assert!(gezeigt > vorher);
            vorher = gezeigt;
        }
    }

    /// C4.7: solange der begonnene Lesevorgang seinen Bestand noch abloesen
    /// muss, nennt der Rang keine Zahl aus dem vorigen Ordner.
    #[test]
    fn waehrend_der_ersatz_aussteht_nennt_der_rang_nichts() {
        let mut ausstehend = stand(38, 412, 0);
        ausstehend.ersetzt_beim_naechsten_stapel = true;
        assert_eq!(filterstand_text("rs", ausstehend), None);
    }

    /// C4.8: ohne Filtertext meldet der Rang nichts, und die Zeile verhaelt
    /// sich wie vor dieser Runde.
    #[test]
    fn ohne_filtertext_meldet_der_rang_nichts() {
        assert_eq!(filterstand_text("", stand(412, 412, 0)), None);
        assert_eq!(filterstand_text("", stand(412, 412, 7)), None);
        let quellen = Quellen {
            filterstand: filterstand_text("", stand(412, 412, 0)),
            markierungsstand: Some("12 markiert, davon 3 Ordner, 4,2 MB".to_owned()),
            ..Quellen::default()
        };
        let leer = Quellen::default();
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide(), None)
            .expect("der Markierungsstand steht in der Zeile");
        assert_eq!(meldung.rang, Rang::Markierungsstand);
    }

    // ------------------------------------------------------------------
    // Die zwei Satzteile des Inhaltsfilters (C4.8 bis C4.10, Runde 11)
    // ------------------------------------------------------------------

    /// Der Kern des Satzes, wie ihn alle Proben dieses Abschnitts erwarten.
    const KERN: &str = "Filter \u{201e}notiz\u{201c}: 38 von 4.812 angezeigt";

    /// Ein Filterstand mit allen sechs Groessen.
    fn voll(liest_inhalt: bool, zu_gross: u64, ausgeblendet: usize) -> Filterstand {
        Filterstand {
            liest_inhalt,
            zu_gross,
            ..stand(38, 4_812, ausgeblendet)
        }
    }

    /// Der Satz zu einem Stand, der bei stehendem Filtertext immer einen hat.
    fn satz(stand: Filterstand) -> String {
        filterstand_text("notiz", stand).expect("bei stehendem Filtertext meldet der Rang etwas")
    }

    /// C4.8: jede der acht Kombinationen der drei Zusaetze, in der
    /// entschiedenen Reihenfolge Kern, Lesehinweis, Groessenhinweis,
    /// Markierungshinweis.
    ///
    /// Die erwarteten Saetze stehen ausgeschrieben da und werden nicht aus
    /// denselben Bausteinen zusammengesetzt wie [`filterstand_text`]; eine
    /// Probe, die die Regel nachbaut, prueft sie nicht.
    #[test]
    fn jede_kombination_der_vier_satzteile_steht_in_der_festgelegten_reihenfolge() {
        let faelle: [(bool, u64, usize, String); 8] = [
            (false, 0, 0, KERN.to_owned()),
            (false, 0, 3, format!("{KERN}, 3 Markierungen ausgeblendet")),
            (false, 12, 0, format!("{KERN}, 12 Dateien zu groß")),
            (
                false,
                12,
                3,
                format!("{KERN}, 12 Dateien zu groß, 3 Markierungen ausgeblendet"),
            ),
            (true, 0, 0, format!("{KERN}, Inhalt wird gelesen")),
            (
                true,
                0,
                3,
                format!("{KERN}, Inhalt wird gelesen, 3 Markierungen ausgeblendet"),
            ),
            (
                true,
                12,
                0,
                format!("{KERN}, Inhalt wird gelesen, 12 Dateien zu groß"),
            ),
            (
                true,
                12,
                3,
                format!(
                    "{KERN}, Inhalt wird gelesen, 12 Dateien zu groß, 3 Markierungen ausgeblendet"
                ),
            ),
        ];
        for (liest_inhalt, zu_gross, ausgeblendet, erwartet) in faelle {
            assert_eq!(
                satz(voll(liest_inhalt, zu_gross, ausgeblendet)),
                erwartet,
                "liest_inhalt={liest_inhalt}, zu_gross={zu_gross}, ausgeblendet={ausgeblendet}"
            );
        }
    }

    /// Null zu grosse Dateien heisst: der Groessenhinweis steht nicht da.
    ///
    /// Der Fall steht ausdruecklich fuer sich, weil er der haeufigste ist:
    /// jeder Lauf ohne eine Datei ueber der Grenze faellt in ihn, und ein
    /// Satzteil "0 Dateien zu groß" waere eine Auskunft ueber nichts.
    #[test]
    fn ohne_zu_grosse_datei_steht_der_groessenhinweis_nicht_da() {
        for eingabe in [voll(false, 0, 0), voll(true, 0, 0), voll(true, 0, 3)] {
            let gesetzt = satz(eingabe);
            assert!(
                !gesetzt.contains("zu groß"),
                "kein Groessenhinweis bei null zu grossen Dateien: {gesetzt}"
            );
            assert!(
                !gesetzt.contains("Datei"),
                "auch kein Rest des Hinweises: {gesetzt}"
            );
        }
        assert_eq!(
            satz(voll(true, 0, 3)),
            format!("{KERN}, Inhalt wird gelesen, 3 Markierungen ausgeblendet")
        );
    }

    /// Der Groessenhinweis hat einen Singularzweig, genau wie der
    /// Markierungshinweis unter ihm, und seine Zahl traegt den Tausenderpunkt.
    #[test]
    fn der_groessenhinweis_trennt_eine_datei_von_mehreren() {
        assert_eq!(
            satz(voll(false, 1, 0)),
            format!("{KERN}, eine Datei zu groß")
        );
        assert_eq!(
            satz(voll(false, 2, 0)),
            format!("{KERN}, 2 Dateien zu groß")
        );
        assert_eq!(
            satz(voll(false, 2_500, 0)),
            format!("{KERN}, 2.500 Dateien zu groß")
        );
    }

    /// Ohne die beiden neuen Teile ist der Satz zeichengleich mit dem der
    /// Runde 10.
    ///
    /// Das ist die Bedingung der Bauentscheidung: die Umstellung der
    /// Reihenfolge wirkt genau dann, wenn einer der neuen Teile dasteht, und
    /// die entstehen nur bei gesetztem "Content".
    #[test]
    fn ohne_inhaltsdurchlauf_ist_der_satz_der_der_runde_zehn() {
        assert_eq!(
            satz(voll(false, 0, 0)),
            "Filter \u{201e}notiz\u{201c}: 38 von 4.812 angezeigt"
        );
        assert_eq!(
            satz(voll(false, 0, 1)),
            "Filter \u{201e}notiz\u{201c}: 38 von 4.812 angezeigt, eine Markierung ausgeblendet"
        );
        assert_eq!(
            satz(voll(false, 0, 2_500)),
            "Filter \u{201e}notiz\u{201c}: 38 von 4.812 angezeigt, 2.500 Markierungen ausgeblendet"
        );
    }

    /// C4.9 und C4.10: der volle Satz bleibt ein Rang von sechs und ist kein
    /// Fehler.
    ///
    /// Die beiden Zusaetze aendern weder die Rangfolge noch die Farbe: ein
    /// Lesefortschritt wird nicht rot.
    #[test]
    fn der_volle_satz_bleibt_ein_rang_und_kein_fehler() {
        assert_eq!(Rang::ALLE.len(), 7, "sieben Raenge seit der Runde 20");
        let quellen = Quellen {
            filterstand: filterstand_text("notiz", voll(true, 12, 3)),
            ..Quellen::default()
        };
        let leer = Quellen::default();
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide(), None)
            .expect("der Filterstand steht als einzige Quelle in der Zeile");
        assert_eq!(meldung.rang, Rang::Filterstand);
        assert_eq!(meldung.art, Art::Vorgang);
        assert_ne!(meldung.art, Art::Fehler);
        assert!(meldung.text.contains("Inhalt wird gelesen"));
    }

    /// Die beiden Abbruchgruende der Runde 10 gehen den neuen Teilen vor:
    /// ohne Filtertext und waehrend eines ausstehenden Ersatzes meldet der
    /// Rang nichts, gleich was der Durchlauf treibt.
    #[test]
    fn die_neuen_teile_heben_die_beiden_abbruchgruende_nicht_auf() {
        assert_eq!(filterstand_text("", voll(true, 12, 3)), None);
        let mut ausstehend = voll(true, 12, 3);
        ausstehend.ersetzt_beim_naechsten_stapel = true;
        assert_eq!(filterstand_text("notiz", ausstehend), None);
    }

    /// C4.10 der Runde 10: die Rangfolge traegt lauter verschiedene Werte, und
    /// jeder Dateifenster-Rang hat sein Feld in [`Quellen`]. Beide
    /// Fallunterscheidungen sind damit ueber dieselben Werte vollstaendig; ein
    /// weiterer Rang haelt den Bau an, statt still in einen Auffangzweig zu
    /// fallen. Der eine Vorschau-Rang hat **kein** Feld, und das ist die
    /// Aussage von [`Rang::herkunft`].
    #[test]
    fn jeder_dateifenster_rang_hat_genau_ein_feld_und_der_vorschau_rang_keines() {
        for (stelle_im_feld, rang) in Rang::ALLE.iter().enumerate() {
            assert_eq!(stelle(*rang), stelle_im_feld, "kein Rang steht doppelt");
            if rang.herkunft() == Herkunftsart::Vorschau {
                assert_eq!(*rang, Rang::Seitenzaehler, "der eine Vorschau-Rang");
                for quellen in [Quellen::default(), nur(Rang::Filterstand, "Text")] {
                    assert_eq!(quellen.text(*rang), None, "kein Dateifenster traegt ihn");
                }
                continue;
            }
            let quellen = nur(*rang, "Text");
            let gesetzt = Rang::ALLE
                .iter()
                .filter(|anderer| quellen.text(**anderer).is_some())
                .count();
            assert_eq!(gesetzt, 1, "ein Rang, ein Feld");
            assert_eq!(quellen.text(*rang), Some("Text"));
        }
    }

    // ------------------------------------------------------------------
    // Die zweite Stelle der Ordnung (C5.6, Runde 6)
    // ------------------------------------------------------------------

    /// Erste der vier Stellen, an denen sich die Ordnung entscheidet: gleicher
    /// Rang auf beiden Seiten. Dann gewinnt die aktive.
    #[test]
    fn bei_gleichem_rang_gewinnt_die_aktive_seite() {
        let links = nur(Rang::Vorgangsanzeige, "Kopieren links");
        let rechts = nur(Rang::Vorgangsanzeige, "Kopieren rechts");
        for (aktiv, text) in [
            (Fensterseite::Links, "Kopieren links"),
            (Fensterseite::Rechts, "Kopieren rechts"),
        ] {
            let meldung =
                zeile(&links, &rechts, aktiv, &beide(), None).expect("beide Seiten melden etwas");
            assert_eq!(meldung.herkunft, Herkunft::Dateifenster(aktiv));
            assert_eq!(meldung.text, text);
        }
    }

    /// Zweite Stelle: der hoehere Rang des **inaktiven** Dateifensters gewinnt
    /// gegen den niedrigeren des aktiven. Das Kriterium nennt genau dieses
    /// Paar.
    #[test]
    fn der_hoehere_rang_der_inaktiven_seite_schlaegt_den_niedrigeren_der_aktiven() {
        let links = nur(
            Rang::Markierungsstand,
            "12 markiert, davon 3 Ordner, 4,2 MB",
        );
        let rechts = nur(Rang::Fenstermeldung, "Datenträger ausgeworfen");
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .expect("beide Seiten melden etwas");
        assert_eq!(
            meldung.herkunft,
            Herkunft::Dateifenster(Fensterseite::Rechts)
        );
        assert_eq!(meldung.rang, Rang::Fenstermeldung);
    }

    /// Dritte Stelle: meldet nur das inaktive Dateifenster, steht seine
    /// Meldung in der Zeile. Eine leere Zeile neben einer ungelesenen Meldung
    /// waere der Verlust, den C5.7 ausschliesst.
    #[test]
    fn meldet_nur_die_inaktive_seite_steht_ihre_meldung_in_der_zeile() {
        let links = Quellen::default();
        let rechts = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .expect("die inaktive Seite meldet etwas");
        assert_eq!(
            meldung.herkunft,
            Herkunft::Dateifenster(Fensterseite::Rechts)
        );
        assert_eq!(meldung.text, "Ordner nicht lesbar");
    }

    /// Vierte Stelle: schweigen beide, bleibt die Zeile leer.
    #[test]
    fn schweigen_beide_seiten_bleibt_die_zeile_leer() {
        for aktiv in Fensterseite::ALLE {
            assert_eq!(
                zeile(
                    &Quellen::default(),
                    &Quellen::default(),
                    aktiv,
                    &beide(),
                    None
                ),
                None
            );
        }
    }

    /// Die Ordnung ist ueber alle zwoelf Bewerber vollstaendig und
    /// ueberschneidungsfrei: melden beide Seiten auf jedem der sechs Raenge,
    /// gewinnt genau eine Aussage, und es ist die des obersten Ranges der
    /// aktiven Seite.
    #[test]
    fn ueber_alle_zwoelf_bewerber_gewinnt_genau_eine_aussage() {
        let voll = |kennung: &str| Quellen {
            befehlsantwort: Some(format!("Antwort {kennung}")),
            vorgangsanzeige: Some(format!("Vorgang {kennung}")),
            fenstermeldung: Some(format!("Ereignis {kennung}")),
            tabmeldung: Some(format!("Zustand {kennung}")),
            filterstand: Some(format!("Filter {kennung}")),
            markierungsstand: Some(format!("Markierung {kennung}")),
        };
        let links = voll("links");
        let rechts = voll("rechts");
        for (aktiv, text) in [
            (Fensterseite::Links, "Antwort links"),
            (Fensterseite::Rechts, "Antwort rechts"),
        ] {
            let meldung = zeile(&links, &rechts, aktiv, &beide(), None)
                .expect("zwoelf Bewerber, einer gewinnt");
            assert_eq!(meldung.rang, Rang::Befehlsantwort);
            assert_eq!(meldung.herkunft, Herkunft::Dateifenster(aktiv));
            assert_eq!(meldung.text, text);
        }
    }

    /// **Verdraengt wird nichts geloescht** (C5.7), jetzt ueber beide
    /// Dateifenster: faellt die ueberlegene Meldung der aktiven Seite, kommt
    /// die der inaktiven zum Vorschein, ohne dass jemand sie neu gesetzt
    /// haette.
    #[test]
    fn die_verdraengte_meldung_der_inaktiven_seite_erscheint_danach() {
        let mut links = nur(Rang::Befehlsantwort, "es ist nichts ausgewählt");
        let rechts = nur(Rang::Fenstermeldung, "Datenträger ausgeworfen");
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .expect("die Antwort gewinnt");
        assert_eq!(
            meldung.herkunft,
            Herkunft::Dateifenster(Fensterseite::Links)
        );
        // Der naechste Tastenbefehl raeumt die Befehlsantwort weg. Am
        // Quellensatz der rechten Seite hat sich nichts geaendert.
        links.befehlsantwort = None;
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .expect("die Auswurfmeldung steht noch");
        assert_eq!(
            meldung.herkunft,
            Herkunft::Dateifenster(Fensterseite::Rechts)
        );
        assert_eq!(meldung.text, "Datenträger ausgeworfen");
    }

    /// Jeder der sechs Raenge traegt seine Art, und zwar dieselbe auf beiden
    /// Seiten: die Herkunft faerbt nichts.
    #[test]
    fn die_art_haengt_am_rang_und_nicht_an_der_seite() {
        for rang in dateifenster_raenge() {
            for aktiv in Fensterseite::ALLE {
                let quellen = nur(rang, "Text");
                let leer = Quellen::default();
                let links = zeile(&quellen, &leer, aktiv, &beide(), None)
                    .expect("die linke Seite meldet etwas");
                let rechts = zeile(&leer, &quellen, aktiv, &beide(), None)
                    .expect("die rechte Seite meldet etwas");
                assert_eq!(links.art, rang.art());
                assert_eq!(rechts.art, rang.art());
            }
        }
    }

    // ------------------------------------------------------------------
    // Der Namenszusatz (C5.8, Runde 6)
    // ------------------------------------------------------------------

    /// Die Meldung des aktiven Dateifensters traegt keinen Zusatz, die des
    /// anderen nennt es.
    #[test]
    fn den_namenszusatz_traegt_genau_die_inaktive_seite() {
        let links = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        let leer = Quellen::default();
        assert_eq!(
            text_von(&links, &leer, Fensterseite::Links).as_deref(),
            Some("Ordner nicht lesbar"),
            "die Meldung des aktiven Dateifensters steht ohne Zusatz"
        );
        assert_eq!(
            text_von(&links, &leer, Fensterseite::Rechts).as_deref(),
            Some("linkes Dateifenster: Ordner nicht lesbar"),
            "die Meldung des inaktiven Dateifensters nennt es"
        );
        let rechts = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        assert_eq!(
            text_von(&leer, &rechts, Fensterseite::Links).as_deref(),
            Some("rechtes Dateifenster: Ordner nicht lesbar")
        );
    }

    /// Der Zusatz haengt an der Seite und nicht am Rang: er steht auf jedem
    /// der fuenf.
    #[test]
    fn der_namenszusatz_gilt_auf_jedem_rang() {
        for rang in dateifenster_raenge() {
            let rechts = nur(rang, "Text");
            assert_eq!(
                text_von(&Quellen::default(), &rechts, Fensterseite::Links).as_deref(),
                Some("rechtes Dateifenster: Text")
            );
            assert_eq!(
                text_von(&Quellen::default(), &rechts, Fensterseite::Rechts).as_deref(),
                Some("Text")
            );
        }
    }

    /// Steht nur ein Dateifenster, traegt kein Satz einen Zusatz — auch dann
    /// nicht, wenn das ausgeblendete etwas zu sagen haette, und selbst dann
    /// nicht, wenn es den hoeheren Rang haelt.
    ///
    /// **Die Probe setzt die Voraussetzung nicht mehr, sondern misst sie.** Bis
    /// zum 260812 uebergab sie fuer das ausgeblendete Dateifenster
    /// `Quellen::default()` und begruendete das im Doc-Kommentar mit "es meldet
    /// nichts". Genau diese Voraussetzung stellte
    /// `Anwendungsdelegierter::statuszeile_nachziehen` nicht her, und die Probe
    /// waere gruen geblieben, wenn jemand den Fall behoebe oder verschlimmerte
    /// (`issues/260812-1805_*_die-eine-statuszeile-zeigt-meldungen-eines-ausgeblendeten-dateifensters.md`).
    #[test]
    fn steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz() {
        for aktiv in Fensterseite::ALLE {
            let sichtbares = nur(Rang::Markierungsstand, "3 markiert, davon 0 Ordner, 6 KB");
            let ausgeblendetes = nur(Rang::Fenstermeldung, "Datenträger ausgeworfen");
            let (links, rechts) = match aktiv {
                Fensterseite::Links => (sichtbares, ausgeblendetes),
                Fensterseite::Rechts => (ausgeblendetes, sichtbares),
            };
            assert_eq!(
                text_von_bei(&links, &rechts, aktiv, &steht_nur(aktiv)).as_deref(),
                Some("3 markiert, davon 0 Ordner, 6 KB"),
                "das ausgeblendete Dateifenster haelt den hoeheren Rang und bleibt trotzdem stumm"
            );
        }
    }

    // ------------------------------------------------------------------
    // Ein ausgeblendetes Dateifenster bewirbt sich nicht (C5.8, Runde 6)
    // ------------------------------------------------------------------

    /// Der Weg aus dem Defekt vom 260812-1805, Schritt fuer Schritt: im rechten
    /// Dateifenster steht ein Ordner, der sich nicht lesen laesst, und der
    /// Nutzer blendet das rechte aus. Danach ist die Zeile leer statt dauerhaft
    /// rot.
    #[test]
    fn die_meldung_eines_ausgeblendeten_dateifensters_steht_nicht_in_der_zeile() {
        let links = Quellen::default();
        let rechts = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        assert_eq!(
            zeile(
                &links,
                &rechts,
                Fensterseite::Links,
                &steht_nur(Fensterseite::Links),
                None
            ),
            None,
            "das ausgeblendete rechte Dateifenster bewirbt sich nicht"
        );
    }

    /// **Verdraengt wird nichts geloescht** (C5.7), und ausgeblendet ebenso
    /// wenig: derselbe Quellensatz steht wieder in der Zeile, sobald der
    /// Bereich zurueckkommt. Niemand muss die Meldung neu setzen.
    #[test]
    fn die_meldung_kommt_mit_dem_eingeblendeten_dateifenster_zurueck() {
        let links = Quellen::default();
        let rechts = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide(), None)
            .expect("mit beiden Dateifenstern steht die Meldung wieder da");
        assert_eq!(
            meldung.herkunft,
            Herkunft::Dateifenster(Fensterseite::Rechts)
        );
        assert_eq!(meldung.text, "Ordner nicht lesbar");
    }

    /// Die Bedingung haengt an der Seite und nicht am Rang: sie gilt auf allen
    /// fuenf und in beide Richtungen.
    #[test]
    fn auf_jedem_rang_bewirbt_sich_allein_das_sichtbare_dateifenster() {
        for rang in dateifenster_raenge() {
            for sichtbare in Fensterseite::ALLE {
                let quellen = nur(rang, "Text");
                let leer = Quellen::default();
                // Der Satz gehoert jeweils dem ausgeblendeten Dateifenster.
                let (links, rechts) = match sichtbare {
                    Fensterseite::Links => (leer, quellen),
                    Fensterseite::Rechts => (quellen, leer),
                };
                assert_eq!(
                    zeile(&links, &rechts, sichtbare, &steht_nur(sichtbare), None),
                    None,
                    "{rang:?} des ausgeblendeten Dateifensters steht in der Zeile"
                );
            }
        }
    }

    /// [`zeilentext`] liest allein die Seite und laesst den Text unangetastet;
    /// der Zusatz kommt davor und ersetzt nichts.
    #[test]
    fn der_zusatz_steht_vor_dem_unveraenderten_text() {
        let meldung = Meldung {
            herkunft: Herkunft::Dateifenster(Fensterseite::Rechts),
            rang: Rang::Befehlsantwort,
            text: "die Zwischenablage ist leer",
            art: Art::Fehler,
        };
        let satz = zeilentext(&meldung, Fensterseite::Links);
        assert!(satz.ends_with("die Zwischenablage ist leer"), "{satz}");
        assert!(satz.starts_with("rechtes Dateifenster"), "{satz}");
    }

    // ------------------------------------------------------------------
    // Der siebte Rang: der Seitenzaehler der Vorschau (C4, Runde 20)
    // ------------------------------------------------------------------

    /// Eine Vorschau, die ein PDF zeigt, ohne dass ein Dateifenster etwas
    /// zu sagen hat.
    const SEITE: &str = "Seite 3 von 9";

    /// Der Seitenzaehler steht zwischen Filterstand und Markierungsstand, in
    /// der Rangfolge selbst und in der Auswahl, die daraus faellt; und er hat
    /// die Herkunft, die ihn aus den [`Quellen`] heraushaelt.
    #[test]
    fn der_seitenzaehler_steht_zwischen_filterstand_und_markierungsstand() {
        assert_eq!(Rang::ALLE.len(), 7);
        assert!(stelle(Rang::Filterstand) < stelle(Rang::Seitenzaehler));
        assert!(stelle(Rang::Seitenzaehler) < stelle(Rang::Markierungsstand));
        assert_eq!(Rang::Seitenzaehler.herkunft(), Herkunftsart::Vorschau);
        assert_eq!(dateifenster_raenge().count(), Rang::ALLE.len() - 1);

        let leer = Quellen::default();
        let meldung = zeile(&leer, &leer, Fensterseite::Links, &beide(), Some(SEITE))
            .expect("die Vorschau meldet etwas");
        assert_eq!(meldung.rang, Rang::Seitenzaehler);
        assert_eq!(meldung.herkunft, Herkunft::Vorschau);
        assert_eq!(meldung.text, SEITE);
    }

    /// C4.5: ein stehender Filtertext verdraengt den Zaehler, und faellt der
    /// Filter, ist er wieder da — verdraengt wird nichts geloescht.
    #[test]
    fn ein_stehender_filtertext_verdraengt_den_seitenzaehler_und_gibt_ihn_zurueck() {
        let leer = Quellen::default();
        let mit_filter = nur(Rang::Filterstand, "Filter „rs“: 12 von 340 angezeigt");
        let meldung = zeile(
            &mit_filter,
            &leer,
            Fensterseite::Links,
            &beide(),
            Some(SEITE),
        )
        .expect("zwei Raenge melden etwas");
        assert_eq!(meldung.rang, Rang::Filterstand);

        let meldung = zeile(&leer, &leer, Fensterseite::Links, &beide(), Some(SEITE))
            .expect("die Vorschau meldet noch");
        assert_eq!(meldung.rang, Rang::Seitenzaehler);
        assert_eq!(meldung.text, SEITE);
    }

    /// Der Zaehler steht ueber dem Markierungsstand: beide sind Auskuenfte
    /// desselben Ranges, und die Seite ist die juengere (A5).
    #[test]
    fn der_seitenzaehler_steht_ueber_dem_markierungsstand() {
        let leer = Quellen::default();
        let markiert = nur(
            Rang::Markierungsstand,
            "12 markiert, davon 3 Ordner, 4,2 MB",
        );
        let meldung = zeile(&markiert, &leer, Fensterseite::Links, &beide(), Some(SEITE))
            .expect("zwei Raenge melden etwas");
        assert_eq!(meldung.rang, Rang::Seitenzaehler);
    }

    /// C4.6: Vorgangsanzeige, Befehlsantwort und Fenstermeldung stehen ueber
    /// dem Zaehler, und zwar auch aus dem inaktiven Dateifenster, weil der
    /// Rang vor der Seite entscheidet; und der Zaehler ist kein Fehler.
    #[test]
    fn vorgang_befehlsantwort_und_fenstermeldung_stehen_ueber_dem_seitenzaehler() {
        assert_eq!(Rang::Seitenzaehler.art(), Art::Vorgang);
        let leer = Quellen::default();
        for hoeherer in [
            Rang::Befehlsantwort,
            Rang::Vorgangsanzeige,
            Rang::Fenstermeldung,
            Rang::Tabmeldung,
        ] {
            let rechts = nur(hoeherer, "Text");
            let meldung = zeile(&leer, &rechts, Fensterseite::Links, &beide(), Some(SEITE))
                .expect("zwei Raenge melden etwas");
            assert_eq!(
                meldung.rang, hoeherer,
                "{hoeherer:?} steht ueber dem Zaehler"
            );
            assert_eq!(
                meldung.herkunft,
                Herkunft::Dateifenster(Fensterseite::Rechts)
            );
        }
        let meldung = zeile(&leer, &leer, Fensterseite::Links, &beide(), Some(SEITE))
            .expect("die Vorschau meldet etwas");
        assert_eq!(meldung.art, Art::Vorgang);
    }

    /// Der Zaehler traegt keinen Seitennamen, gleich welches Dateifenster
    /// aktiv ist: die Vorschau gehoert zu keiner Seite.
    #[test]
    fn der_seitenzaehler_traegt_keinen_seitennamen() {
        let leer = Quellen::default();
        for aktiv in Fensterseite::ALLE {
            let meldung = zeile(&leer, &leer, aktiv, &beide(), Some(SEITE))
                .expect("die Vorschau meldet etwas");
            assert_eq!(zeilentext(&meldung, aktiv), SEITE, "{aktiv:?} aktiv");
        }
    }

    /// Bei ausgeblendeter Vorschau bewirbt sich der Zaehler nicht, mit
    /// derselben Frage wie fuer ein ausgeblendetes Dateifenster; und er kommt
    /// mit der Vorschau zurueck.
    #[test]
    fn bei_ausgeblendeter_vorschau_bewirbt_sich_der_seitenzaehler_nicht() {
        let mut ohne_vorschau = beide();
        ohne_vorschau.vorschau = false;
        assert!(!sichtbar_in(&ohne_vorschau, Bereich::Vorschau));
        let leer = Quellen::default();
        assert_eq!(
            zeile(
                &leer,
                &leer,
                Fensterseite::Links,
                &ohne_vorschau,
                Some(SEITE)
            ),
            None,
            "die ausgeblendete Vorschau bewirbt sich nicht"
        );
        let markiert = nur(Rang::Markierungsstand, "3 markiert, davon 0 Ordner, 6 KB");
        let meldung = zeile(
            &markiert,
            &leer,
            Fensterseite::Links,
            &ohne_vorschau,
            Some(SEITE),
        )
        .expect("das Dateifenster meldet etwas");
        assert_eq!(meldung.rang, Rang::Markierungsstand);
        let meldung = zeile(&markiert, &leer, Fensterseite::Links, &beide(), Some(SEITE))
            .expect("mit der Vorschau steht der Zaehler wieder da");
        assert_eq!(meldung.rang, Rang::Seitenzaehler);
    }

    /// C4.4: ohne PDF meldet die Vorschau nichts, und die Zeile zeigt, was sie
    /// vor der Runde 20 zeigte.
    #[test]
    fn ohne_pdf_meldet_der_vorschau_rang_nichts() {
        let leer = Quellen::default();
        assert_eq!(
            zeile(&leer, &leer, Fensterseite::Links, &beide(), None),
            None
        );
        let markiert = nur(Rang::Markierungsstand, "3 markiert, davon 0 Ordner, 6 KB");
        let meldung = zeile(&markiert, &leer, Fensterseite::Links, &beide(), None)
            .expect("das Dateifenster meldet etwas");
        assert_eq!(meldung.rang, Rang::Markierungsstand);
    }

    /// C4.1: der Satz lautet "Seite N von M", und die Zahlen tragen dieselben
    /// Tausenderpunkte wie jede andere Zahl der Zeile.
    #[test]
    fn der_seitenzaehler_satz_nennt_seite_und_seitenzahl_mit_tausenderpunkten() {
        assert_eq!(seitenzaehler_text(1, 9), "Seite 1 von 9");
        assert_eq!(seitenzaehler_text(1200, 3400), "Seite 1.200 von 3.400");
    }
}
