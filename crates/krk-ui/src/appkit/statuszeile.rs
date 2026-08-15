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
//! **Die beiden untersten Raenge haben kein Feld.** Der Filterstand aus C4
//! der Runde 10 und der Markierungsstand aus C2 werden bei jedem Schreiben der
//! Zeile aus dem Ordnermodell des sichtbaren Tabs gerechnet, statt gesetzt und
//! geloescht zu werden. Beide tragen dieselbe Begruendung: ein Feld haette
//! eine zweite Loeschregel, und beide sind ein Zustand und kein Ereignis. Sie
//! steht bei `DateifensterQuelle::markierungsstand_text` und bei
//! [`filterstand_text`].
//!
//! **Ein ausgeblendetes Dateifenster bewirbt sich nicht.** Es sind zwoelf
//! Quellen zweier Dateifenster, aber nur die des sichtbaren treten an; die
//! Begruendung steht bei [`zeile`].
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

/// Die sechs Raenge der Zeile, vom obersten zum untersten.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig: ein siebter Rang
/// haelt den Bau an und erzwingt die Antwort darauf, wo er einzuordnen ist und
/// ob er ein Fehler ist (C4.10). Dieselbe Bauart wie `Bereich` und `Fokus`.
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
    /// Was im sichtbaren Tab markiert ist (C2).
    Markierungsstand,
}

impl Rang {
    /// Alle sechs, vom obersten zum untersten.
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
    pub const ALLE: [Rang; 6] = [
        Rang::Befehlsantwort,
        Rang::Vorgangsanzeige,
        Rang::Fenstermeldung,
        Rang::Tabmeldung,
        Rang::Filterstand,
        Rang::Markierungsstand,
    ];

    /// Ob eine Meldung dieses Ranges ein Fehler ist.
    ///
    /// **Die Art faellt mit dem Rang und wird aus ihm gerechnet statt
    /// gesetzt.** Ein Fortschritt, eine Filterzahl und eine Markierungszahl
    /// sind keine Fehler, die drei uebrigen sind welche (C4.2). Ein zweites
    /// Feld, das jemand setzt, waere die Gelegenheit, eine Markierungszahl rot
    /// zu faerben.
    pub const fn art(self) -> Art {
        match self {
            Rang::Befehlsantwort => Art::Fehler,
            Rang::Vorgangsanzeige => Art::Vorgang,
            Rang::Fenstermeldung => Art::Fehler,
            Rang::Tabmeldung => Art::Fehler,
            Rang::Filterstand => Art::Vorgang,
            Rang::Markierungsstand => Art::Vorgang,
        }
    }
}

/// Was ein Dateifenster der Zeile anzubieten hat.
///
/// Sechs Quellen, je Rang eine. Die vier oberen haelt das Dateifenster in je
/// einem eigenen Feld mit je einer Loeschregel, die zwei unteren rechnet es bei
/// jeder Abfrage; `DateifensterQuelle::meldungsquellen` schreibt sie ab.
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
    /// vollstaendige Fallunterscheidung: ein siebter Rang haelt hier den Bau
    /// an (C4.10).
    fn text(&self, rang: Rang) -> Option<&str> {
        match rang {
            Rang::Befehlsantwort => self.befehlsantwort.as_deref(),
            Rang::Vorgangsanzeige => self.vorgangsanzeige.as_deref(),
            Rang::Fenstermeldung => self.fenstermeldung.as_deref(),
            Rang::Tabmeldung => self.tabmeldung.as_deref(),
            Rang::Filterstand => self.filterstand.as_deref(),
            Rang::Markierungsstand => self.markierungsstand.as_deref(),
        }
    }
}

/// Was der fuenfte Rang aus dem Modell des sichtbaren Tabs braucht.
///
/// Vier Groessen, alle aus demselben `Ordnermodell` und alle dort schon
/// vorhanden. **Dieser Rang rechnet nichts nach, was das Modell ohnehin
/// weiss**; eine eigene Rechnung daneben waere eine zweite Wahrheit ueber
/// denselben Zustand.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Filterstand {
    /// Wie viele Zeilen die Liste jetzt zeigt: `Ordnermodell::zeilenzahl`.
    ///
    /// **Entschiedene Zeilen und keine Treffer** (C4.6). Wie viele Treffer
    /// unter einem Ordner liegen, weiss niemand in diesem Baum: der Durchlauf
    /// hoert je Ordner beim ersten Fund auf. Der Wert waechst waehrend eines
    /// Durchlaufs von selbst mit, weil er die Sichtreihenfolge zaehlt und
    /// keinen eigenen Zaehler fuehrt (C4.5).
    pub gezeigt: usize,
    /// Wie viele Eintraege der angezeigte Ordner hat, ungefiltert.
    pub vorhanden: usize,
    /// Wie viele Markierungen der Filter gerade ausblendet.
    ///
    /// Null heisst: keine, und dann steht der Teil des Satzes, der sie nennt,
    /// nicht da (C4.4).
    pub ausgeblendete_markierungen: usize,
    /// Ob der begonnene Lesevorgang seinen Bestand noch abloesen muss.
    ///
    /// Kommt aus `Ordnermodell::ersetzt_beim_naechsten_stapel`, der vorhandenen
    /// Frage nach genau diesem Zustand (C4.7).
    pub ersetzt_beim_naechsten_stapel: bool,
}

/// Der fuenfte Rang der Statuszeile: der stehende Filtertext und was er von der
/// Liste uebrig laesst (C4 der Runde 10).
///
/// `None` heisst: dieser Rang meldet nichts. Zwei Wege fuehren dorthin, und
/// **beide stehen hier und nicht beim Aufrufer**, damit sie ohne Fenster
/// pruefbar sind. Steht kein Filtertext, ist nichts zu melden, und die Zeile
/// verhaelt sich wie vor dieser Runde (C4.8). Und solange ein begonnener
/// Lesevorgang noch nichts geliefert hat, stehen noch die Zeilen des vorigen
/// Ordners; eine Zahl daraus waere eine Auskunft ueber einen Ordner, den der
/// Nutzer schon verlassen hat (C4.7).
///
/// **Der Satz nennt drei Dinge und manchmal ein viertes** (C4.3, C4.4): den
/// Filtertext, die Zahl der gezeigten Zeilen, die Zahl der Eintraege des
/// angezeigten Ordners, und die Zahl der Markierungen, die der Filter gerade
/// ausblendet. Der vierte Teil steht nur da, wenn es solche Markierungen gibt.
///
/// **Er ist die Gegenleistung dafuer, dass die Markierungsregel unter dem
/// Filter unveraendert bleibt.** Der Nutzer hat am 260814-1610 entschieden,
/// dass eine ausgeblendete Markierung fortbesteht und nicht wirkt
/// (`decisions/260814-1552_*_was-geschieht-mit-einer-markierung-die-der-filter-ausblendet.md`);
/// ohne diesen Satzteil muesste er erraten, dass es sie ueberhaupt gibt.
///
/// **Beide Zahlen gehen durch [`zahl`]** und tragen damit dieselben
/// Tausenderpunkte wie ein laufender Vorgang und der Markierungsstand daneben.
/// Ein zweites Zahlenformat entsteht nicht.
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
    let ausgeblendet = match stand.ausgeblendete_markierungen {
        0 => String::new(),
        1 => ", eine Markierung ausgeblendet".to_owned(),
        mehrere => format!(", {} Markierungen ausgeblendet", zahl(mehrere)),
    };
    Some(format!(
        "Filter \u{201e}{filtertext}\u{201c}: {} von {} angezeigt{ausgeblendet}",
        zahl(stand.gezeigt),
        zahl(stand.vorhanden)
    ))
}

/// Die eine Aussage, die von zwoelf moeglichen jetzt in der Zeile steht.
///
/// Sie traegt ihre Herkunft mit: [`zeilentext`] braucht die Seite, um zu
/// entscheiden, ob der Satz sie nennen muss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meldung<'a> {
    /// Das Dateifenster, aus dem die Meldung kommt.
    pub seite: Fensterseite,
    /// Der Rang, auf dem sie gewonnen hat.
    pub rang: Rang,
    /// Der Text, wie ihn die Quelle gesetzt hat, ohne Zusatz.
    pub text: &'a str,
    /// Ob sie rot erscheint; gerechnet aus [`Rang::art`].
    pub art: Art,
}

/// Was von den zwoelf Quellen jetzt in der Zeile steht.
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
/// 6  Markierungsstand  was im sichtbaren Tab markiert ist
/// ```
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
/// haelt ihren Text dort, und jedes Feld hat genau eine Loeschregel; die vier
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
/// Sie ist ueber alle zwoelf Bewerber vollstaendig und ueberschneidungsfrei,
/// und nicht aus Sorgfalt, sondern der Bauart nach: zwei Bewerber desselben
/// Ranges gehoeren immer verschiedenen Seiten, also entscheidet die zweite
/// Stelle jeden Gleichstand der ersten. **Die Ordnung steht deshalb in der
/// Schleifenreihenfolge und nicht in einer Vergleichsfunktion** — aussen die
/// sechs Raenge aus [`Rang::ALLE`], innen die aktive Seite vor der anderen.
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
) -> Option<Meldung<'a>> {
    let quellen = |seite: Fensterseite| match seite {
        Fensterseite::Links => links,
        Fensterseite::Rechts => rechts,
    };
    for rang in Rang::ALLE {
        // Die aktive Seite zuerst: die zweite Stelle der Ordnung.
        for seite in [aktiv, aktiv.andere()] {
            // Wer nicht dasteht, sagt nichts. Die Begruendung steht im Kopf
            // dieser Funktion.
            if !sichtbar_in(sichtbar, Bereich::von_seite(seite)) {
                continue;
            }
            if let Some(text) = quellen(seite).text(rang) {
                return Some(Meldung {
                    seite,
                    rang,
                    text,
                    art: rang.art(),
                });
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
/// Die beiden Namen stehen hier und nicht im Kern: es sind Anzeigetexte, und
/// [`Fensterseite`] ist ein Wert der Ablage, der von Anzeige nichts weiss.
pub fn zeilentext(meldung: &Meldung<'_>, aktiv: Fensterseite) -> String {
    if meldung.seite == aktiv {
        return meldung.text.to_owned();
    }
    format!("{}: {}", seitenname(meldung.seite), meldung.text)
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
        Art, Bereich, Filterstand, Meldung, Quellen, Rang, filterstand_text, sichtbar_in, zeile,
        zeilentext,
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
            Rang::Markierungsstand => &mut quellen.markierungsstand,
        };
        *feld = Some(text.to_owned());
        quellen
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
        zeile(&links, &rechts, Fensterseite::Links, &beide())
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
        zeile(links, rechts, aktiv, sichtbar).map(|meldung| zeilentext(&meldung, aktiv))
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
                &beide()
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

    /// Ein Filterstand mit drei Zahlen und ohne ausstehenden Ersatz.
    fn stand(gezeigt: usize, vorhanden: usize, ausgeblendet: usize) -> Filterstand {
        Filterstand {
            gezeigt,
            vorhanden,
            ausgeblendete_markierungen: ausgeblendet,
            ersetzt_beim_naechsten_stapel: false,
        }
    }

    /// C4.1: er steht ueber dem Markierungsstand und unter der Tabmeldung,
    /// und zwar in der Rangfolge selbst und in der Auswahl, die daraus faellt.
    #[test]
    fn der_filterstand_steht_zwischen_tabmeldung_und_markierungsstand() {
        assert_eq!(Rang::ALLE.len(), 6);
        assert!(stelle(Rang::Tabmeldung) < stelle(Rang::Filterstand));
        assert!(stelle(Rang::Filterstand) < stelle(Rang::Markierungsstand));

        let leer = Quellen::default();
        let mut quellen = nur(Rang::Filterstand, "Filter „rs“: 12 von 340 angezeigt");
        quellen.markierungsstand = Some("12 markiert, davon 3 Ordner, 4,2 MB".to_owned());
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide())
            .expect("zwei Raenge melden etwas");
        assert_eq!(
            meldung.rang,
            Rang::Filterstand,
            "eine verkuerzte Liste wiegt schwerer als die Markierungszahl"
        );

        quellen.tabmeldung = Some("Ordner nicht lesbar".to_owned());
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide())
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
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide())
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
        let meldung = zeile(&quellen, &leer, Fensterseite::Links, &beide())
            .expect("der Markierungsstand steht in der Zeile");
        assert_eq!(meldung.rang, Rang::Markierungsstand);
    }

    /// C4.10: die Rangfolge traegt sechs verschiedene Werte, und jeder von
    /// ihnen hat sein Feld in [`Quellen`]. Beide Fallunterscheidungen sind
    /// damit ueber dieselben sechs Werte vollstaendig; ein siebter Rang haelt
    /// den Bau an, statt still in einen Auffangzweig zu fallen.
    #[test]
    fn jeder_der_sechs_raenge_hat_genau_ein_feld() {
        for (stelle_im_feld, rang) in Rang::ALLE.iter().enumerate() {
            assert_eq!(stelle(*rang), stelle_im_feld, "kein Rang steht doppelt");
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
                zeile(&links, &rechts, aktiv, &beide()).expect("beide Seiten melden etwas");
            assert_eq!(meldung.seite, aktiv);
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
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide())
            .expect("beide Seiten melden etwas");
        assert_eq!(meldung.seite, Fensterseite::Rechts);
        assert_eq!(meldung.rang, Rang::Fenstermeldung);
    }

    /// Dritte Stelle: meldet nur das inaktive Dateifenster, steht seine
    /// Meldung in der Zeile. Eine leere Zeile neben einer ungelesenen Meldung
    /// waere der Verlust, den C5.7 ausschliesst.
    #[test]
    fn meldet_nur_die_inaktive_seite_steht_ihre_meldung_in_der_zeile() {
        let links = Quellen::default();
        let rechts = nur(Rang::Tabmeldung, "Ordner nicht lesbar");
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide())
            .expect("die inaktive Seite meldet etwas");
        assert_eq!(meldung.seite, Fensterseite::Rechts);
        assert_eq!(meldung.text, "Ordner nicht lesbar");
    }

    /// Vierte Stelle: schweigen beide, bleibt die Zeile leer.
    #[test]
    fn schweigen_beide_seiten_bleibt_die_zeile_leer() {
        for aktiv in Fensterseite::ALLE {
            assert_eq!(
                zeile(&Quellen::default(), &Quellen::default(), aktiv, &beide()),
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
            let meldung =
                zeile(&links, &rechts, aktiv, &beide()).expect("zwoelf Bewerber, einer gewinnt");
            assert_eq!(meldung.rang, Rang::Befehlsantwort);
            assert_eq!(meldung.seite, aktiv);
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
        let meldung =
            zeile(&links, &rechts, Fensterseite::Links, &beide()).expect("die Antwort gewinnt");
        assert_eq!(meldung.seite, Fensterseite::Links);
        // Der naechste Tastenbefehl raeumt die Befehlsantwort weg. Am
        // Quellensatz der rechten Seite hat sich nichts geaendert.
        links.befehlsantwort = None;
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide())
            .expect("die Auswurfmeldung steht noch");
        assert_eq!(meldung.seite, Fensterseite::Rechts);
        assert_eq!(meldung.text, "Datenträger ausgeworfen");
    }

    /// Jeder der sechs Raenge traegt seine Art, und zwar dieselbe auf beiden
    /// Seiten: die Herkunft faerbt nichts.
    #[test]
    fn die_art_haengt_am_rang_und_nicht_an_der_seite() {
        for rang in Rang::ALLE {
            for aktiv in Fensterseite::ALLE {
                let quellen = nur(rang, "Text");
                let leer = Quellen::default();
                let links =
                    zeile(&quellen, &leer, aktiv, &beide()).expect("die linke Seite meldet etwas");
                let rechts =
                    zeile(&leer, &quellen, aktiv, &beide()).expect("die rechte Seite meldet etwas");
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
        for rang in Rang::ALLE {
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
                &steht_nur(Fensterseite::Links)
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
        let meldung = zeile(&links, &rechts, Fensterseite::Links, &beide())
            .expect("mit beiden Dateifenstern steht die Meldung wieder da");
        assert_eq!(meldung.seite, Fensterseite::Rechts);
        assert_eq!(meldung.text, "Ordner nicht lesbar");
    }

    /// Die Bedingung haengt an der Seite und nicht am Rang: sie gilt auf allen
    /// fuenf und in beide Richtungen.
    #[test]
    fn auf_jedem_rang_bewirbt_sich_allein_das_sichtbare_dateifenster() {
        for rang in Rang::ALLE {
            for sichtbare in Fensterseite::ALLE {
                let quellen = nur(rang, "Text");
                let leer = Quellen::default();
                // Der Satz gehoert jeweils dem ausgeblendeten Dateifenster.
                let (links, rechts) = match sichtbare {
                    Fensterseite::Links => (leer, quellen),
                    Fensterseite::Rechts => (quellen, leer),
                };
                assert_eq!(
                    zeile(&links, &rechts, sichtbare, &steht_nur(sichtbare)),
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
            seite: Fensterseite::Rechts,
            rang: Rang::Befehlsantwort,
            text: "die Zwischenablage ist leer",
            art: Art::Fehler,
        };
        let satz = zeilentext(&meldung, Fensterseite::Links);
        assert!(satz.ends_with("die Zwischenablage ist leer"), "{satz}");
        assert!(satz.starts_with("rechtes Dateifenster"), "{satz}");
    }
}
