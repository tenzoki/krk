//! Die eine Umsetzung einer [`Formatierung`] in die Merkmale einer
//! `NSTextView` (C3).
//!
//! ```text
//!   crate::hervorhebung ──> Formatierung ──> textmerkmale::anwenden ──> NSTextView
//!        (rein, ohne AppKit)   ^              │                         ├─ NSTextStorage
//!                              │              │                         └─ NSLayoutManager
//!                              │              └─ zuruecksetzen (dieselbe Datei)
//!                              │
//!   NSView::effectiveAppearance┴─ tafel_der_erscheinung ──> Tafel
//! ```
//!
//! # Ein Modul fuer zwei Verbraucher, und darum ist es eines
//!
//! Eine Ueberschrift sieht im Editor und in der Vorschau gleich aus: dieselbe
//! Stufenfolge, dieselbe feste Schrift im Quelltextblock, derselbe Einzug einer
//! Listenzeile, dieselben Farben aus derselben Tafel. Zwei Umsetzungen
//! nebeneinander waeren zwei Wahrheiten darueber, und sie liefen beim ersten
//! geaenderten Faktor auseinander, ohne dass ein Bau oder eine Probe es faenge.
//! Dieselbe Erwaegung laesst [`super::nummernspalte`] **eine** Klasse fuer beide
//! Textflaechen sein.
//!
//! **Beide rufen hier herein**, seit die Vorschau ihre Auszeichnungen traegt:
//! [`super::editor`] mit der Formatierung seiner Formatansicht,
//! [`super::vorschau`] mit der des gerenderten Markdown und mit den
//! eingefaerbten Stellen einer Quelltextdatei. Wer die Umsetzung hier
//! veraendert, aendert sie fuer beide Flaechen.
//!
//! **Was hier nicht wohnt.** Welche Stellen welche Auszeichnung tragen, rechnet
//! [`crate::hervorhebung`] ohne AppKit aus; diese Datei setzt das Ergebnis um
//! und rechnet es nicht nach. Und sie zeichnet nichts nach: dass die geaenderten
//! Zeilenkaesten eine neue Nummernspalte brauchen, sagt der Rueckgabewert von
//! [`anwenden`] dem Aufrufer, der seine Flaeche kennt.
//!
//! # Warum auch die Wahl der Farbtafel hier wohnt
//!
//! [`tafel_der_erscheinung`] beantwortet eine andere Frage als [`anwenden`]:
//! nicht, welches Merkmal eine Stelle traegt, sondern welche der beiden Tafeln
//! ueberhaupt gilt. Sie steht trotzdem in dieser Datei, und die Begruendung
//! sind die beiden Orte, an denen sie **nicht** stehen kann.
//!
//! - **Nicht in [`crate::hervorhebung`].** Die Antwort haengt am wirksamen
//!   Erscheinungsbild einer `NSView`, und jene Datei traegt keine Zeile
//!   AppKit; ihr Modulkopf sagt es zu, und S16 misst es, indem es die
//!   Kistennamen zaehlt. Sie nimmt die [`Tafel`] als Angabe herein und waehlt
//!   sie nicht aus.
//! - **Nicht privat in [`super::editor`].** Dort stand sie bis zum 260812, und
//!   solange der Editor der einzige Verbraucher war, war das der richtige Ort.
//!   Die Vorschau braucht dieselbe Antwort gleich zweimal — die Farbe eines
//!   Verweises im gerenderten Markdown kommt aus der Tafel, und die
//!   Einfaerbung des Quelltextes ebenso. Eine zweite Abfrage neben dieser
//!   waere die zweite Wahrheit darueber, was "dunkel" heisst.
//!
//! Diese Datei ist die AppKit-Seite derselben Naht: nebenan wird mit einer
//! Tafel gerechnet, hier steht, woher sie kommt und was aus dem Ergebnis wird.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextView` (`NSTextView.h:76`), `NSTextStorage` (`NSTextStorage.h:37`),
//! `NSFont` (`NSFont.h:24`), `NSColor` (`NSColor.h:77`),
//! `NSFontDescriptor` (`NSFontDescriptor.h:61`), `NSView` (`NSView.h:81`),
//! `NSArray` und
//! `NSMutableParagraphStyle` (`NSParagraphStyle.h:112`) stehen seit macOS 10.0
//! zur Verfuegung. `NSLayoutManager` nicht: die Klasse traegt im SDK
//! `macos(10.7)` (`NSLayoutManager.h:65`, am SDK gelesen). Sie steht in dieser
//! Aufzaehlung zwischen lauter 10.0ern, und die Reihe fortzuschreiben statt
//! nachzusehen hat in diesem Verzeichnis schon einmal die falsche Zahl
//! erzeugt. Das Buendel zielt auf 15.0 (`.cargo/config.toml`), keine von ihnen
//! ist nach macOS 15 hinzugekommen, und deshalb braucht keine der Beruehrungen
//! in dieser Datei eine Verfuegbarkeitspruefung zur Laufzeit.
//!
//! Zwei **Methoden** sind juenger als ihre Klasse und liegen beide weit unter
//! dem Zielsystem: `addTemporaryAttribute:value:forCharacterRange:` seit macOS
//! 10.5 (`NSLayoutManager.h:360`) und `colorWithSRGBRed:green:blue:alpha:` seit
//! macOS 10.7 (`NSColor.h:90`). Die uebrigen tragen im Kopf des Systems keine
//! eigene Angabe und stehen damit seit 10.0: `textStorage` und `layoutManager`
//! an `NSTextView` (`NSTextView.h:113`, `:111`),
//! `setTemporaryAttributes:forCharacterRange:` (`NSLayoutManager.h:353`),
//! `smallSystemFontSize` (`NSFont.h:76`), `systemFontOfSize:` (`:47`),
//! `boldSystemFontOfSize:` (`:48`), `userFixedPitchFontOfSize:` (`:41`),
//! `firstLineHeadIndent` und `headIndent` (`NSParagraphStyle.h:116`, `:117`),
//! die drei Stuecke der kursiven Schrift — `fontDescriptor` an `NSFont`
//! (`NSFont.h:87`), `fontDescriptorWithSymbolicTraits:`
//! (`NSFontDescriptor.h:92`) und `fontWithDescriptor:size:` (`NSFont.h:31`),
//! dazu der Wert `NSFontDescriptorTraitItalic` (`NSFontDescriptor.h:22`) —
//! sowie `beginEditing`, `endEditing` und `addAttributes:range:` an
//! `NSMutableAttributedString` (`NSAttributedString.h:85`, `:86`, `:76`); das
//! `removeAttribute:range:` daneben (`:77`) ist mit dem Grundabsatz vom 260929
//! aus dieser Datei gefallen. Die vier Merkmalsnamen
//! tragen `macos(10.0)` (`NSAttributedString.h:26`, `:27`, `:28`, `:34`),
//! `NSUnderlineStyleSingle` keine Angabe (`:64`). Alle Zahlen am SDK gelesen.
//!
//! **Der Grundabsatz (260929) liegt ebenso unter dem Zielsystem.**
//! `setTabStops:` und `setDefaultTabInterval:` an `NSMutableParagraphStyle`
//! tragen `macos(10.0)` (`NSParagraphStyle.h:127`, `:128`), die Leser
//! `tabStops` und `defaultTabInterval` der Probe an `NSParagraphStyle`
//! (`NSParagraphStyle.h:68`, macOS 10.0) ebenso (`:99`, `:100`), `headIndent`
//! ohne eigene Angabe;
//! `sizeWithAttributes:` aus der Kategorie `NSStringDrawing` traegt
//! `macos(10.0)` (`NSStringDrawing.h:39`), und die Klasse
//! `NSMutableAttributedString` (`NSAttributedString.h:63`) sowie `length` und
//! `string` (`:32`) tragen keine eigene Angabe. `ns_string!` ist ein Makro von
//! `objc2-foundation` und spricht keine Methode an. Die Probe baut ohne Flaeche
//! und beruehrt dafuer `NSTextContainer` mit `initWithSize:`, das juengste
//! Stueck dieses Abschnitts mit `macos(10.11)` (`NSTextContainer.h:25`), dazu
//! `addTextContainer:`, `ensureLayoutForTextContainer:`, `numberOfGlyphs` und
//! `locationForGlyphAtIndex:` an `NSLayoutManager` (`NSLayoutManager.h:91`,
//! `:173`, `:186`, `:260`), `addLayoutManager:` an `NSTextStorage`
//! (`NSTextStorage.h:45`) und `replaceCharactersInRange:withString:`
//! (`NSAttributedString.h:66`), alle ohne eigene Angabe. Die Vorgabe an der
//! Flaeche ([`grund_vorgeben`]) spricht `typingAttributes` und
//! `setTypingAttributes:` (`NSTextView.h:375`) sowie `defaultParagraphStyle`
//! (`NSTextView.h:392`) an, beide ohne eigene Angabe, und fuer die Kopie der
//! Anschlagsmerkmale `mutableCopy` und `setObject:forKey:` an
//! `NSMutableDictionary` (`NSDictionary.h:102`), ebenso ohne Angabe.
//!
//! **Die Wahl der Farbtafel ist die juengste Beruehrung dieser Datei und liegt
//! immer noch weit unter dem Zielsystem.** `NSAppearance` steht seit macOS 10.9
//! (`NSAppearance.h:19`), ebenso seine Eigenschaft `name` (`:22`), die
//! Eigenschaft `effectiveAppearance` des Protokolls `NSAppearanceCustomization`
//! (`:90`) und der Name `NSAppearanceNameAqua` (`:63`). Seit macOS 10.14 stehen
//! `bestMatchFromAppearancesWithNames:` (`:56`) und `NSAppearanceNameDarkAqua`
//! (`:64`) — die beiden juengsten Angaben im Kopf dieser Datei.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `NSRange` ist eine C-Struktur
//! (`NSRange.h:12`); die Klassen `NSString` (`NSString.h:103`), `NSNumber`
//! (`NSValue.h:42`) und `NSDictionary` (`NSDictionary.h:14`) tragen keine
//! eigene Angabe; die Aufzaehlung `NSFontDescriptorSymbolicTraits`
//! (`NSFontDescriptor.h:21`) schliesst mit blossem `};`, `NSUnderlineStyle`
//! (`NSAttributedString.h:62`) traegt `API_AVAILABLE(macos(10.0))`; die drei
//! Merkmalsschluessel `NSFontAttributeName` (`NSAttributedString.h:26`),
//! `NSForegroundColorAttributeName` (`:28`), `NSParagraphStyleAttributeName`
//! (`:27`) und `NSUnderlineStyleAttributeName` (`:34`) tragen `macos(10.0)`;
//! `NSStringDrawing`, `NSMutableAttributedString` und `ns_string` stehen im
//! Absatz zum Grundabsatz, `NSTextContainer`, `NSTextStorage`,
//! `NSLayoutManager` und `NSParagraphStyle` der Probe weiter oben;
//! `NSMutableCopying` ist das Protokoll hinter `mutableCopy`
//! (`NSObject.h:22`) und traegt wie `NSMutableDictionary`
//! (`NSDictionary.h:99`) keine eigene Angabe;
//! alle uebrigen tragen im SDK keine eigene Verfuegbarkeitsangabe und stehen
//! damit seit 10.0.

use std::collections::HashMap;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSAppearanceCustomization, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSColor, NSFont,
    NSFontAttributeName, NSFontDescriptorSymbolicTraits, NSForegroundColorAttributeName,
    NSMutableParagraphStyle, NSParagraphStyleAttributeName, NSStringDrawing, NSTextView,
    NSUnderlineStyle, NSUnderlineStyleAttributeName, NSView,
};
use objc2_foundation::{
    NSArray, NSDictionary, NSMutableAttributedString, NSMutableCopying, NSNumber, NSRange,
    NSString, ns_string,
};

use crate::editormodell::Ansicht;
use crate::hervorhebung::{Auszeichnung, Darstellungsart, Farbe, Formatierung, Tafel};

/// Um wie viele Punkte die Formatansicht ihre Grundschrift ueber die der
/// Rohansicht hebt (C3).
///
/// C3 verlangt fuer einfachen Text "eine lesbare Schriftgroesse" und der Plan
/// "eine gegenueber der Rohansicht lesbarere". Beides nennt keine Zahl, und
/// diese ist gewaehlt und nicht abgeleitet: zwei Punkte sind der kleinste
/// Schritt, den man nebeneinandergehalten sieht, und der groesste, der die Zahl
/// der Zeilen im Bild nicht spuerbar aendert.
///
/// **Code bekommt den Zuschlag nicht.** Quelltext wird in der Groesse gelesen,
/// in der er geschrieben wurde, und der sichtbare Unterschied zur Rohansicht ist
/// bei ihm die Einfaerbung und der Umbruch.
///
/// Ein Zuschlag **auf** die Grundlage und keine eigene Groesse: er wird auf das
/// addiert, was [`grundmerkmale`] als Grundlage nimmt, und geht deshalb mit,
/// wenn die sich aendert.
const LESEZUSCHLAG: f64 = 2.0;

/// Um welchen Faktor eine Markdown-Ueberschrift ihre Grundschrift ueberschreitet,
/// nach Stufen von 1 bis 6.
///
/// Absteigend, weil `#` mehr wiegt als `######`. Die Zahlen sind gewaehlt und
/// nicht abgeleitet; sie halten die sechste Stufe noch merklich ueber dem
/// Fliesstext, damit keine Ueberschrift aussieht wie keine.
const UEBERSCHRIFTSFAKTOREN: [f64; 6] = [1.7, 1.5, 1.3, 1.2, 1.1, 1.05];

/// Der Einzug einer Markdown-Listenzeile **je Ebene**, in Punkten (C3).
///
/// Er rueckt den ganzen Absatz ein, das Aufzaehlungszeichen eingeschlossen; das
/// Zeichen selbst bleibt stehen, wie der Datensatz vom 260808-0140 es verlangt.
const LISTENEINZUG: f64 = 20.0;

/// Ab welcher Verschachtelungstiefe der Einzug nicht weiter waechst.
///
/// Acht Ebenen sind 160 Punkte, und das ist die Mindestbreite eines Bereichs
/// der Fensterzeile: waechst der Einzug darueber hinaus, steht die Zeile ganz
/// ausserhalb der Vorschau. Eine Markdown-Datei kann beliebig tief
/// verschachteln, und eine Grenze ist deshalb keine Vorsicht, sondern die
/// Bedingung dafuer, dass die Zeile sichtbar bleibt.
const EINZUGSGRENZE: u8 = 8;

/// Wie viele Leerzeichenbreiten ein Tabulatorschritt misst (C3, C6).
///
/// **Vier und nicht acht**, weil vier die Vorgabe der gaengigen Editoren ist
/// und die Vorschau ein schmaler Bereich der Fensterzeile: acht Spalten je
/// Schritt schoeben eine zweifach eingerueckte Zeile um sechzehn Zeichen nach
/// rechts. Die Zahl ist gewaehlt und nicht abgeleitet; sie gilt fuer Editor und
/// Vorschau gleich, weil beide ihren Absatzstil aus [`grundabsatz`] nehmen.
const TABSPALTEN: f64 = 4.0;

/// Der Absatzstil jeder Stelle ohne eigenen Einzug: keine festen Tabstopps,
/// dafuer ein Schritt von [`TABSPALTEN`] Leerzeichenbreiten der Grundschrift.
///
/// # Warum es ihn gibt (Defekt 260929, Tabulatoren in der Vorschau)
///
/// Ohne eigenen Absatzstil gilt der des Systems, und der traegt zwoelf
/// Tabstopps im Abstand von **28 Punkt** (`NSParagraphStyle.h:99`). Das ist
/// kein Vielfaches der Spaltenbreite der festen Schrift: bei der kleinen
/// Systemschriftgroesse misst ein Zeichen rund 6,6 Punkt, und ein Wort aus vier
/// Zeichen endet 1,5 Punkt vor dem ersten Stopp. `Wort\tWort` stand damit als
/// `WortWort` da (gemessen: das zweite `W` bei 33,0, das Tabzeichen bei 31,5),
/// in Vorschau und Editor gleich, weil beide ueber [`zuruecksetzen`] gehen.
///
/// **Ein Schritt in Spalten und keine Stopps in Punkt.** Liegen die Stopps auf
/// Vielfachen der Spaltenbreite, springt ein Tabulator wie im Terminal auf die
/// naechste Spalte, die durch [`TABSPALTEN`] teilbar ist, und laesst mindestens
/// eine Spalte Zwischenraum. Die leere Liste der Stopps ist dabei tragend: nur
/// jenseits des letzten Stopps gilt `defaultTabInterval`
/// (`NSParagraphStyle.h:100`), und ohne Stopp ist das von Anfang an.
///
/// Gemessen wird die Breite an einem Leerzeichen der uebergebenen Schrift. In
/// der festen Schrift ist das genau eine Spalte; in der Systemschrift der
/// Formatansicht gibt es keine Spalten, und der Schritt ist dort vier
/// Leerzeichen breit, ohne einen Mindestabstand zusagen zu koennen.
fn grundabsatz(schrift: &NSFont) -> Retained<NSMutableParagraphStyle> {
    let stil = NSMutableParagraphStyle::new();
    stil.setTabStops(Some(&NSArray::new()));
    stil.setDefaultTabInterval(TABSPALTEN * leerzeichenbreite(schrift));
    stil
}

/// Die Breite eines Leerzeichens in dieser Schrift, in Punkt.
fn leerzeichenbreite(schrift: &NSFont) -> f64 {
    let merkmale = schriftmerkmal(schrift);
    // SAFETY: Das Verzeichnis traegt allein die Schrift und ist damit ein
    // gueltiges Merkmalsverzeichnis einer Zeichenkette.
    unsafe { ns_string!(" ").sizeWithAttributes(Some(&merkmale)) }.width
}

/// Traegt eine fertige Formatierung in eine Textflaeche und meldet, ob sie
/// gesetzt hat (C3).
///
/// **Zwei Listen und zwei Orte**, und der Grund steht im Modulkopf von
/// [`crate::hervorhebung`]: der Layoutverwalter beachtet als voruebergehendes
/// Merkmal allein, was die Auslegung nicht aendert. Farbe und
/// Unterstreichung gehen deshalb dorthin, Schriftgroesse, Schriftschnitt,
/// feste Schrift und Einzug in den Textspeicher. In die **Datei** geraet
/// weder das eine noch das andere: gesichert wird
/// [`Editormodell::stand`](crate::editormodell::Editormodell::stand), und der
/// kommt aus den Zeichen der Flaeche und nicht aus ihren Merkmalen.
///
/// **Der Guertel vorweg.** Stimmt die Laenge nicht mehr, gehoert die
/// Lieferung zu einem anderen Stand, und jeder Bereich dahinter waere ein
/// Programmabbruch statt eines falschen Bildes. Erreichbar ist der Fall im
/// Editor nicht, weil ein ueberholtes Ergebnis schon beim Einziehen der
/// Einfaerbung fallengelassen wird; er steht hier, weil der Preis eines Irrtums
/// an dieser Stelle das Programm ist.
///
/// **Erst zuruecknehmen, dann setzen** (Defekt 260810-1245). Bis zum
/// 260810-1245 fing diese Rechnung bei `addAttributes:range:` an und nahm
/// nichts heraus; eine Auszeichnung, die die neue Formatierung nicht mehr
/// fuehrt, blieb damit stehen. Zurueckgenommen wird ueber [`zuruecksetzen`],
/// der einen Stelle, die das tut — und die deckt die voruebergehenden Merkmale
/// mit ab, weshalb hier kein zweites Leeren daneben steht.
///
/// **Der Rueckgabewert laesst sich nicht still fallenlassen.** `false` heisst,
/// dass die Flaeche unberuehrt geblieben ist; wer das nicht unterscheidet, zieht
/// eine Nummernspalte nach, die nichts zu zeichnen bekommen hat, oder haelt eine
/// abgewiesene Lieferung fuer gesetzt. Das `#[must_use]` macht daraus einen
/// Uebersetzerfehler, wie die Regel dieses Projekts es seit dem 260811-2140
/// verlangt. Wer die Auskunft wirklich nicht braucht, schreibt `let _ =` davor
/// und sagt damit genau das.
#[must_use = "wurden Merkmale gesetzt, sind die Zeilenkaesten neu und die Nummernspalte nachzuziehen"]
pub fn anwenden(
    text: &NSTextView,
    formatierung: &Formatierung,
    art: Darstellungsart,
    ansicht: Ansicht,
) -> bool {
    // SAFETY: Speicher und Verwalter bringt die Flaeche selbst mit.
    let (speicher, verwalter) = unsafe { (text.textStorage(), text.layoutManager()) };
    let (Some(speicher), Some(verwalter)) = (speicher, verwalter) else {
        return false;
    };
    if speicher.length() != formatierung.laenge {
        return false;
    }
    zuruecksetzen(text, ansicht, art);

    // Die Merkmale des Textspeichers: was auf die Auslegung wirkt.
    //
    // **Die Groesse kommt aus derselben Stelle wie die Grundschrift** und wird
    // hier nicht ein zweites Mal gerechnet. Bis zum 260907 stand hier
    // `systemFontSize() + LESEZUSCHLAG` ausgeschrieben, also die Formatansicht
    // ohne Ruecksicht auf die uebergebene Ansicht; eine Ueberschrift setzte
    // damit auf einer anderen Grundlage auf als der Text, ueber dem sie steht,
    // sobald jemand die Grundlage aendert.
    let grundgroesse = grundgroesse(ansicht, art);
    speicher.beginEditing();
    for stelle in &formatierung.auszeichnungen {
        let bereich = NSRange::new(stelle.anfang, stelle.laenge);
        let merkmale = match stelle.art {
            Auszeichnung::Ueberschrift { stufe } => {
                let faktor = UEBERSCHRIFTSFAKTOREN[usize::from(stufe.clamp(1, 6)) - 1];
                schriftmerkmal(&NSFont::boldSystemFontOfSize(grundgroesse * faktor))
            }
            Auszeichnung::FesteSchrift => schriftmerkmal(&feste_schrift(grundgroesse)),
            Auszeichnung::Listenzeile { tiefe } => {
                einzugsmerkmal(tiefe, &grundschrift(ansicht, art))
            }
            Auszeichnung::Betonung => schriftmerkmal(&kursive_schrift(grundgroesse)),
            Auszeichnung::StarkeBetonung => {
                schriftmerkmal(&NSFont::boldSystemFontOfSize(grundgroesse))
            }
        };
        // SAFETY: Der Bereich liegt im Text, und das ist die ganze
        // Bedingung: die Laenge ist oben geprueft, und jede Stelle der
        // Formatierung liegt nach dem Modulkopf von `crate::hervorhebung`
        // innerhalb dieser Laenge.
        //
        // **Aufsteigend und ueberschneidungsfrei sind die Auszeichnungen
        // nicht**, anders als bis zum 260810 hier stand: eine Listenzeile
        // wird nach den Stuecken ihrer Zeile angehaengt und beginnt vor
        // ihnen. In `- Punkt mit `Code`` liefert die Formatierung
        // `FesteSchrift` bei 12 und danach `Listenzeile` bei 0 (gemessen).
        //
        // **Ueberschneidungen gleichen Merkmalsnamens kommen vor, und was
        // dann gilt, entscheidet allein die Reihenfolge dieser Schleife.**
        // Bis zum 260812 stand hier, `Ueberschrift` und `FesteSchrift`
        // ueberlappten einander nie und die uebrigen setzten verschiedene
        // Namen. Beides gilt nicht mehr: `crate::markdown` ist ein zweiter
        // Erzeuger von `Formatierung`, vier der fuenf Auszeichnungen setzen
        // `NSFontAttributeName` — `Ueberschrift`, `FesteSchrift`, `Betonung`,
        // `StarkeBetonung` —, und verschachtelte Listenzeilen setzen
        // einander ueberlappend denselben Absatzstil. `addAttributes:`
        // ersetzt bei gleichem Namen, statt zusammenzulegen.
        //
        // Getragen wird das von der Sortierung in
        // `crate::markdown::Zerlegung::abschliessen`: aussen vor innen, bei
        // gleichem Bereich das zuerst geoeffnete zuerst. Das innere Stueck
        // kommt damit zuletzt und gewinnt — der Quelltext in einer
        // Ueberschrift bekommt seine feste Schrift, der tiefere Listenpunkt
        // seinen groesseren Einzug.
        //
        // **Der Quelltext in einer Ueberschrift faellt dabei auf die
        // Grundgroesse**, und das gehoert zum vorigen Satz dazu: er bekommt
        // seine feste Schrift **und verliert die Groesse der Ueberschrift**,
        // weil `feste_schrift(grundgroesse)` eine ganz neue Schrift setzt. Bei
        // Stufe 1 sind das 41 Prozent Hoehe gegenueber den Nachbarn in
        // derselben Zeile. Dasselbe trifft jede Betonung in einer
        // Ueberschrift, also `## Ein **fetter** Teil` und `## *kursiver* Teil`
        // (`issues/260812-1920_*_eine-auszeichnung-in-einer-ueberschrift-verliert-deren-schriftgroesse.md`).
        //
        // **Was diese Reihenfolge nicht kann, ist zusammenlegen.** Wo zwei
        // schriftsetzende Auszeichnungen einander enthalten, geht die
        // aeussere fuer den ueberlappten Bereich verloren, statt sich mit der
        // inneren zu verbinden: in `*kursiv **fett** wieder kursiv*` ist
        // "fett" fett und nicht mehr kursiv (gemessen). Fett **und** kursiv
        // brauchte einen Schriftzustand je Stelle statt eines Ersetzens; der
        // Datensatz dazu ist
        // `issues/260812-1805_*_der-ueberschneidungssatz-in-textmerkmale-anwenden-gilt-seit-markdown-rs-nicht-mehr.md`.
        //
        // **Ein blosses Zusammenlegen der Schnitte behebt den Groessenverlust
        // nicht.** `NSFontDescriptor`-Merkmale und `applyFontTraits:range:`
        // legen Schnitte zusammen und keine Groessen; der Zustand je Stelle
        // muesste die Groesse mitfuehren.
        unsafe { speicher.addAttributes_range(&merkmale, bereich) };
    }
    speicher.endEditing();

    // Die voruebergehenden Merkmale: was die Auslegung nicht anfasst.
    let strich = NSNumber::numberWithInteger(NSUnderlineStyle::Single.0);
    let mut farben: HashMap<Farbe, Retained<NSColor>> = HashMap::new();
    // SAFETY: Dieselbe Pruefung deckt beide Schleifen; der Verwalter gehoert
    // dieser Flaeche. Geleert ist die Liste schon: das tut `zuruecksetzen`
    // weiter oben, und ein zweites Leeren hier waere die zweite Stelle mit einer
    // Meinung darueber, was zurueckzunehmen ist.
    unsafe {
        for stueck in &formatierung.einfaerbungen {
            let bereich = NSRange::new(stueck.anfang, stueck.laenge);
            let farbe = farben
                .entry(stueck.farbe)
                .or_insert_with(|| nsfarbe(stueck.farbe));
            verwalter.addTemporaryAttribute_value_forCharacterRange(
                NSForegroundColorAttributeName,
                farbe,
                bereich,
            );
            if stueck.unterstrichen {
                verwalter.addTemporaryAttribute_value_forCharacterRange(
                    NSUnderlineStyleAttributeName,
                    &strich,
                    bereich,
                );
            }
        }
    }

    true
}

/// Nimmt jede gesetzte Auszeichnung wieder heraus und stellt die Grundschrift
/// ueber den ganzen Text her.
///
/// **Beide Listen**, denn beide werden gesetzt: die voruebergehenden Merkmale
/// im Layoutverwalter und Schrift wie Absatzeinzug im Textspeicher.
///
/// # Warum die Schrift hier steht und nicht dem `setFont:` ueberlassen bleibt
///
/// Bis zum 260810-1245 stand hier allein der Absatzeinzug, mit der Begruendung,
/// `setFont:` und `setTextColor:` an der Flaeche ueberschrieben den ganzen
/// Speicher ohnehin. Der Satz stimmt, gilt aber nur fuer die vier Anlaesse, aus
/// denen der Editor seine Darstellung neu setzt — Aufbau, gelungenes Oeffnen,
/// Schliessen, Ansichtswechsel — und **nicht** fuer den fuenften, das Tippen.
/// Dort geht der Weg vom `textDidChange:` ueber die angeforderte Einfaerbung
/// nach [`anwenden`], und der setzte Merkmale, ohne je eines herauszunehmen: wer
/// in der Formatansicht das `#` einer Markdown-Ueberschrift loeschte, sah die
/// Zeile weiter gross und fett, bis er die Ansicht umschaltete oder die Datei
/// neu oeffnete. Dasselbe fuer den Einzug einer entfernten Listenzeile und die
/// feste Schrift eines entfernten Zauns
/// (`issues/260810-1245_*_die-formatansicht-nimmt-gesetzte-merkmale-des-textspeichers-nie-wieder-heraus.md`).
///
/// **Deshalb ist dies die eine Stelle, die zuruecknimmt**, und [`anwenden`]
/// ruft sie, statt eine zweite halbe Ruecknahme daneben zu tragen. Die Wirkung,
/// die das Setzen der Merkmale haben soll, ist **setzen** und nicht hinzufuegen:
/// nach dem Ruf traegt der Textspeicher genau die Merkmale der uebergebenen
/// Formatierung.
///
/// Was hier **nicht** steht, ist die Farbe. Sie ist ein voruebergehendes
/// Merkmal des Layoutverwalters, und die werden vollstaendig geleert; der
/// Textspeicher traegt keine.
pub fn zuruecksetzen(text: &NSTextView, ansicht: Ansicht, art: Darstellungsart) {
    // SAFETY: Speicher und Verwalter bringt die Flaeche selbst mit und wird
    // hier nur beschrieben; die Bereiche decken genau den vorhandenen Text.
    unsafe {
        if let Some(speicher) = text.textStorage() {
            grund_legen(&speicher, ansicht, art);
            let ganz = NSRange::new(0, speicher.length());
            if let Some(verwalter) = text.layoutManager() {
                let leer: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
                verwalter.setTemporaryAttributes_forCharacterRange(&leer, ganz);
            }
        }
    }
}

/// Legt Grundschrift und [`grundabsatz`] ueber den ganzen Text; der Teil von
/// [`zuruecksetzen`], der den Textspeicher beschreibt.
///
/// **Der Absatzstil wird nicht entfernt, sondern ersetzt.** Bis zum 260929
/// stand hier `removeAttribute:` fuer den Absatzstil, und damit galt der des
/// Systems mit seinen 28-Punkt-Stopps (siehe [`grundabsatz`]). `addAttributes:`
/// ersetzt bei gleichem Namen, also faellt der Einzug einer vorigen Listenzeile
/// weiterhin mit.
///
/// Eine eigene Funktion und nicht der Rumpf von [`zuruecksetzen`], weil sie
/// allein den Speicher braucht: die Probe
/// `ein_tabulator_laesst_mindestens_eine_spalte_zwischenraum` faehrt sie an
/// einem Speicher ohne Flaeche, denn schon das Beschreiben einer nackten
/// `NSTextView` endet unter `libtest` mit `SIGSEGV` (Modulkopf der Proben in
/// [`super::editor`]).
fn grund_legen(speicher: &NSMutableAttributedString, ansicht: Ansicht, art: Darstellungsart) {
    let grundmerkmal = grundmerkmale_des_textes(&grundschrift(ansicht, art));
    let ganz = NSRange::new(0, speicher.length());
    // SAFETY: Der Bereich deckt genau den vorhandenen Text, und das
    // Verzeichnis traegt eine Schrift und einen Absatzstil unter ihren Namen.
    unsafe { speicher.addAttributes_range(&grundmerkmal, ganz) };
}

/// Setzt Grundschrift und [`grundabsatz`] als Vorgabe einer bearbeitbaren
/// Flaeche, also auch fuer den naechsten Anschlag (C3).
///
/// # Warum neben [`grund_legen`] (Defekt 260929-1141, Quicknote)
///
/// [`grund_legen`] beschreibt allein Text, der schon im Speicher steht. Ist
/// der Speicher leer, setzt es nichts, und getippter Text erbt die
/// `typingAttributes` der Flaeche, die ohne diese Funktion den Absatzstil des
/// Systems mit seinen 28-Punkt-Stopps tragen. Das traf die Quicknote immer und
/// eine leer geoeffnete Datei in der Rohansicht des Editors; in der
/// Formatansicht holte die Einfaerbung es zufaellig nach.
///
/// **Die eine Stelle fuer beide Flaechen**: der Editor ruft sie beim Bau und
/// bei jedem Nachziehen der Darstellung, die Quicknote beim Bau. Sie setzt die
/// Schrift mit `setFont:`, damit keine Flaeche die Schrift ohne den Absatz
/// bekommt, dazu `defaultParagraphStyle` und den Absatzstil der
/// `typingAttributes`; die uebrigen Anschlagsmerkmale (Farbe) bleiben, siehe
/// [`anschlagsmerkmale`]. Die Vorschau ist nicht bearbeitbar und braucht sie
/// nicht. Die Probe `beide_bearbeitbaren_flaechen_nehmen_die_vorgabe_von_hier`
/// haelt, dass Editor und Quicknote hier hereinrufen und keine von beiden die
/// Schrift daran vorbei setzt.
pub fn grund_vorgeben(text: &NSTextView, ansicht: Ansicht, art: Darstellungsart) {
    let schrift = grundschrift(ansicht, art);
    text.setFont(Some(&schrift));
    let stil = grundabsatz(&schrift);
    text.setDefaultParagraphStyle(Some(&stil));
    let merkmale = anschlagsmerkmale(&text.typingAttributes(), &stil);
    // SAFETY: Das Verzeichnis ist das der Flaeche, um einen Absatzstil unter
    // dessen Namen ergaenzt, und damit ein gueltiges Merkmalsverzeichnis.
    unsafe { text.setTypingAttributes(&merkmale) };
}

/// Die Anschlagsmerkmale einer Flaeche mit dem uebergebenen Absatzstil statt
/// ihres bisherigen; alle uebrigen Merkmale bleiben, wie sie waren.
///
/// Eine eigene Funktion, weil sie ohne `NSTextView` pruefbar ist (Modulkopf
/// der Proben in [`super::editor`]).
fn anschlagsmerkmale(
    vorhandene: &NSDictionary<NSString, AnyObject>,
    stil: &NSMutableParagraphStyle,
) -> Retained<NSDictionary<NSString, AnyObject>> {
    let merkmale = vorhandene.mutableCopy();
    // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname des Absatzstils. Es
    // wird gelesen und nicht geschrieben.
    merkmale.insert(unsafe { NSParagraphStyleAttributeName }, stil);
    Retained::into_super(merkmale)
}

/// Die Grundschrift einer Ansicht: die Schrift, in der jede Stelle steht, die
/// keine eigene Auszeichnung traegt (C3).
///
/// **Eine Regel und keine drei.** Fest geschrieben wird, was Zeichen fuer Zeichen
/// gelesen wird: die Rohansicht immer, und die Formatansicht bei Code. Alles
/// Uebrige — einfacher Text und Markdown — bekommt die Systemschrift mit dem
/// [`LESEZUSCHLAG`]. Das ist die "lesbare Schriftgroesse", die C3 fuer einfachen
/// Text zusagt, und zugleich die Grundschrift, ueber der die
/// Markdown-Ueberschriften ihre Stufen haben.
///
/// Welche Groesse dabei die Grundlage ist, sagt [`grundmerkmale`], und diese
/// Funktion rechnet sie nicht nach.
///
/// **Sie steht hier und nicht bei ihren Aufrufern.** Editor und Quicknote setzen
/// sie ueber [`grund_vorgeben`] an der Flaeche und damit auch fuer den naechsten
/// Anschlag — der Editor beim Bau der Flaeche und bei jedem Nachziehen der
/// Darstellung —, die Vorschau
/// ebenso beim Bau ihrer Textanzeige, und [`zuruecksetzen`] setzt sie als
/// Merkmal ueber den ganzen Textspeicher, um eine weggefallene Auszeichnung
/// zurueckzunehmen. Zwei Rechnungen daneben waeren die erste Gelegenheit, dass
/// eine geloeschte Ueberschrift in einer anderen Schrift landete als der, in der
/// ihre Zeile getippt wird.
#[must_use]
pub fn grundschrift(ansicht: Ansicht, art: Darstellungsart) -> Retained<NSFont> {
    let (fest, groesse) = grundmerkmale(ansicht, art);
    if fest {
        feste_schrift(groesse)
    } else {
        NSFont::systemFontOfSize(groesse)
    }
}

/// Die Groesse der Grundschrift dieser Ansicht, ohne die Schrift selbst.
///
/// Fuer [`anwenden`], das die Groesse braucht und die Schrift nicht: die
/// Ueberschriftsstufen und die beiden Betonungen rechnen ueber ihr, und der
/// Quelltextblock setzt seine feste Schrift in ihr. Sie fragt [`grundmerkmale`]
/// und rechnet nichts nach.
fn grundgroesse(ansicht: Ansicht, art: Darstellungsart) -> f64 {
    grundmerkmale(ansicht, art).1
}

/// Die eine Fallunterscheidung ueber Schriftart und Groesse einer Ansicht.
///
/// **Die Grundlage ist die kleine Systemschriftgroesse**, und zwar fuer beide
/// Textflaechen. Bis zum 260907 war es die gewoehnliche
/// (`NSFont::systemFontSize`); der Nutzer hat an jenem Tag entschieden, Editor
/// und Vorschau gemeinsam auf die kleine zu ziehen, statt der Vorschau die
/// Groesse zurueckzugeben, die sie bis zum Zusammenlegen der Schriftwahl allein
/// trug (`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-oder-waechst-sie-auf-die-des-editors.md`,
/// Moeglichkeit 3). **Damit faellt auch die Groesse, die der Editor seit der
/// Runde 2 trug**, und das ist mitentschieden und kein Nebenschaden: der Gewinn
/// ist mehr Text auf einmal in beiden Flaechen, der Preis eine kleinere Schrift
/// im Editor.
///
/// **Die Zahl steht an dieser einen Stelle**, und [`LESEZUSCHLAG`] ist ein
/// Zuschlag auf sie und keine zweite Zahl daneben. Wer die Grundlage aendert,
/// aendert sie hier, und Rohansicht wie Formatansicht gehen mit.
fn grundmerkmale(ansicht: Ansicht, art: Darstellungsart) -> (bool, f64) {
    let grundlage = NSFont::smallSystemFontSize();
    match (ansicht, art) {
        (Ansicht::Roh, _) | (Ansicht::Format, Darstellungsart::Code) => (true, grundlage),
        (Ansicht::Format, Darstellungsart::EinfacherText | Darstellungsart::Markdown) => {
            (false, grundlage + LESEZUSCHLAG)
        }
    }
}

/// Welche Farbtafel zum wirksamen Erscheinungsbild dieser Ansicht passt (S34).
///
/// **Die eine Zuordnung**, und sie ist eine Zeile und keine Tabelle:
/// `bestMatchFromAppearancesWithNames:` ist die Stelle, die AppKit fuer diese
/// Frage vorsieht, und sie beantwortet auch die Erscheinungsbilder mit erhoehtem
/// Kontrast, indem sie sie auf eines der beiden genannten abbildet.
///
/// Alles, was nicht das dunkle Erscheinungsbild ist, bekommt die helle Tafel.
/// Die Fallunterscheidung ist damit trennscharf und vollstaendig, ohne dass KRK
/// eine Liste der Erscheinungsbilder fuehrte, die das System kennt.
///
/// **Zwei Aufrufer, und warum die Frage hier steht und nicht bei ihnen**, sagt
/// der Abschnitt "Warum auch die Wahl der Farbtafel hier wohnt" im Modulkopf.
/// Gefragt wird nach der **Ansicht** und nicht nach der Anwendung: das
/// Erscheinungsbild ist eine Eigenschaft der Ansichtenkette, und ein Fenster
/// kann eines tragen, das von dem der Anwendung abweicht.
#[must_use]
pub fn tafel_der_erscheinung(sicht: &NSView) -> Tafel {
    // SAFETY: Zwei Fremdsymbole von AppKit, die Namen der beiden
    // Erscheinungsbilder. Sie werden gelesen und nicht geschrieben.
    let (hell, dunkel) = unsafe { (NSAppearanceNameAqua, NSAppearanceNameDarkAqua) };
    let namen = NSArray::from_slice(&[hell, dunkel]);
    match sicht
        .effectiveAppearance()
        .bestMatchFromAppearancesWithNames(&namen)
    {
        Some(name) if *name == *dunkel => Tafel::Dunkel,
        _ => Tafel::Hell,
    }
}

/// Die feste Schreibmaschinenschrift des Nutzers, hilfsweise die Systemschrift.
///
/// Dieselbe Wahl und derselbe Rueckfall wie in [`super::nummernspalte`]. Ein
/// System ohne feste Schrift gibt es nicht; der Rueckfall steht da, weil die
/// Schnittstelle ihn zulaesst und ein Editor ohne Schrift keine Antwort ist.
fn feste_schrift(groesse: f64) -> Retained<NSFont> {
    NSFont::userFixedPitchFontOfSize(groesse).unwrap_or_else(|| NSFont::systemFontOfSize(groesse))
}

/// Die kursive Systemschrift, hilfsweise die aufrechte (C4 der Runde 6).
///
/// **Ueber die Beschreibung der Schrift und nicht ueber `NSFontManager`.** Der
/// Verwalter ist die Maschinerie hinter dem Schriftfenster; er baut beim ersten
/// Zugriff einen gemeinsamen Zustand auf, den KRK nirgends sonst braucht.
/// `NSFontDescriptor` beantwortet dieselbe Frage ohne diesen Anhang: er nimmt
/// die Beschreibung der Grundschrift, setzt das Merkmal `TraitItalic` und laesst
/// das System die passende Schnittfassung suchen.
///
/// **Der Rueckfall ist die aufrechte Schrift und kein Fehler.** Findet das
/// System keine kursive Fassung, ist eine aufrechte Betonung die schlechtere
/// Anzeige und eine fehlende Zeile die schlechteste; dieselbe Erwaegung wie bei
/// [`feste_schrift`] daneben.
fn kursive_schrift(groesse: f64) -> Retained<NSFont> {
    let aufrecht = NSFont::systemFontOfSize(groesse);
    let beschreibung = aufrecht
        .fontDescriptor()
        .fontDescriptorWithSymbolicTraits(NSFontDescriptorSymbolicTraits::TraitItalic);
    NSFont::fontWithDescriptor_size(&beschreibung, groesse).unwrap_or(aufrecht)
}

/// Ein Merkmalsverzeichnis mit genau einer Schrift darin.
fn schriftmerkmal(schrift: &NSFont) -> Retained<NSDictionary<NSString, AnyObject>> {
    // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname der Schrift. Es wird
    // gelesen und nicht geschrieben.
    let schluessel = unsafe { [NSFontAttributeName] };
    let werte: [&AnyObject; 1] = [schrift];
    NSDictionary::from_slices(&schluessel, &werte)
}

/// Die Merkmale, die [`zuruecksetzen`] ueber den ganzen Text legt: die
/// Grundschrift und der [`grundabsatz`] zu ihr.
fn grundmerkmale_des_textes(schrift: &NSFont) -> Retained<NSDictionary<NSString, AnyObject>> {
    let stil = grundabsatz(schrift);
    // SAFETY: Zwei Fremdsymbole von AppKit, die Merkmalsnamen von Schrift und
    // Absatzstil. Sie werden gelesen und nicht geschrieben.
    let schluessel = unsafe { [NSFontAttributeName, NSParagraphStyleAttributeName] };
    let werte: [&AnyObject; 2] = [schrift, &stil];
    NSDictionary::from_slices(&schluessel, &werte)
}

/// Ein Merkmalsverzeichnis mit dem Einzug einer Listenzeile darin (C3).
///
/// **Der Einzug waechst mit der Tiefe**, gedeckelt bei [`EINZUGSGRENZE`]. Bis
/// zum 260812 war er fest, und damit stand eine dreistufige Liste flach da
/// (Defekt `260812-1805`).
///
/// **Auf dem [`grundabsatz`] und nicht auf einem leeren Stil**, denn der
/// Absatzstil ersetzt den des Grundes ganz: ohne ihn fielen die Tabulatoren
/// einer Listenzeile auf die 28 Punkt des Systems zurueck.
fn einzugsmerkmal(tiefe: u8, schrift: &NSFont) -> Retained<NSDictionary<NSString, AnyObject>> {
    let einzug = LISTENEINZUG * f64::from(tiefe.clamp(1, EINZUGSGRENZE));
    let stil = grundabsatz(schrift);
    // Beide, damit die erste Zeile mit dem Aufzaehlungszeichen genauso weit
    // einrueckt wie ihre Fortsetzung nach einem Umbruch; sonst haengt das
    // Zeichen als einziges am linken Rand.
    stil.setFirstLineHeadIndent(einzug);
    stil.setHeadIndent(einzug);
    // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname des Absatzstils.
    let schluessel = unsafe { [NSParagraphStyleAttributeName] };
    let werte: [&AnyObject; 1] = [&stil];
    NSDictionary::from_slices(&schluessel, &werte)
}

/// Eine Farbe der Tafel als `NSColor`.
///
/// Im sRGB-Farbraum, weil die Tafeln ihre Werte darin angeben. Ohne Angabe des
/// Farbraums nimmt AppKit den kalibrierten, und dieselbe Zahl saehe dann anders
/// aus als in jedem anderen Programm, das dieselbe Tafel zeigt.
fn nsfarbe(farbe: Farbe) -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(
        f64::from(farbe.rot) / 255.0,
        f64::from(farbe.gruen) / 255.0,
        f64::from(farbe.blau) / 255.0,
        1.0,
    )
}

#[cfg(test)]
mod proben {
    use objc2::AnyThread;
    use objc2_app_kit::{NSLayoutManager, NSParagraphStyle, NSTextContainer, NSTextStorage};
    use objc2_foundation::NSSize;

    use super::*;

    /// Legt den Grund ueber einen Speicher ohne Flaeche und liefert ihn mit
    /// der waagrechten Lage jedes Zeichens.
    ///
    /// **Ohne `NSTextView`**: schon das Beschreiben einer nackten Flaeche endet
    /// unter `libtest` mit `SIGSEGV` (Modulkopf der Proben in
    /// [`super::super::editor`]). Speicher, Layoutverwalter und Behaelter
    /// sind dieselben drei Glieder, die eine Flaeche in sich traegt, und
    /// [`grund_legen`] ist genau der Teil von [`zuruecksetzen`], der den
    /// Speicher beschreibt.
    fn gelegt(
        text: &str,
        ansicht: Ansicht,
        art: Darstellungsart,
    ) -> (Retained<NSTextStorage>, Vec<f64>) {
        let speicher = NSTextStorage::new();
        let verwalter = NSLayoutManager::new();
        let behaelter = NSTextContainer::initWithSize(
            NSTextContainer::alloc(),
            NSSize::new(10_000.0, 10_000.0),
        );
        verwalter.addTextContainer(&behaelter);
        speicher.addLayoutManager(&verwalter);
        speicher.replaceCharactersInRange_withString(NSRange::new(0, 0), &NSString::from_str(text));
        grund_legen(&speicher, ansicht, art);
        verwalter.ensureLayoutForTextContainer(&behaelter);
        let lagen = (0..verwalter.numberOfGlyphs())
            .map(|stelle| verwalter.locationForGlyphAtIndex(stelle).x)
            .collect();
        (speicher, lagen)
    }

    /// Ein Tabulator laesst in der festen Schrift mindestens eine Spalte
    /// Zwischenraum und springt auf die naechste durch [`TABSPALTEN`] teilbare
    /// Spalte (Defekt 260929, „zwei durch Tab getrennte Woerter erscheinen
    /// ohne Zwischenraum“).
    ///
    /// **Was sie faengt:** den Rueckfall auf den Absatzstil des Systems. Dessen
    /// Stopps liegen alle 28 Punkt, und `Wort\tWort` stand damit bei 1,5 Punkt
    /// Abstand als ein Wort da; mit `removeAttribute:` statt des
    /// [`grundabsatz`] in [`grund_legen`] wird sie rot.
    ///
    /// Die Zeichen selbst bleiben, was sie waren: der Grund ist ein Merkmal und
    /// ersetzt kein `\t` durch Leerzeichen. Daran haengt, dass das Kopieren aus
    /// Vorschau und Editor weiter den Tabulator liefert.
    #[test]
    fn ein_tabulator_laesst_mindestens_eine_spalte_zwischenraum() {
        let spalte = leerzeichenbreite(&grundschrift(Ansicht::Roh, Darstellungsart::EinfacherText));
        assert!(spalte > 0.0, "die feste Schrift misst kein Leerzeichen");
        // (Text, Stelle des Zeichens hinter dem Tabulator, erwartete Spalte)
        let faelle = [
            ("a\tb", 2, 4.0),
            ("abc\tb", 4, 4.0),
            ("Wort\tWort", 5, 8.0),
            ("abcde\tx", 6, 8.0),
            ("\t\tx", 2, 8.0),
        ];
        for (text, hinter, spalte_erwartet) in faelle {
            let (speicher, lagen) = gelegt(text, Ansicht::Roh, Darstellungsart::EinfacherText);
            assert_eq!(
                speicher.string().to_string(),
                text,
                "{text:?}: der Grund hat die Zeichen veraendert"
            );
            let spalten = (lagen[hinter] - lagen[0]) / spalte;
            assert!(
                (spalten - spalte_erwartet).abs() < 0.01,
                "{text:?}: das Zeichen hinter dem Tabulator steht in Spalte {spalten:.2} statt \
                 {spalte_erwartet}"
            );
            assert!(
                lagen[hinter] - lagen[hinter - 1] >= spalte * 0.99,
                "{text:?}: der Tabulator ist schmaler als eine Spalte und laesst keinen \
                 Zwischenraum"
            );
        }
    }

    /// Der Einzug einer Listenzeile traegt denselben Tabulatorschritt wie der
    /// Grund und faellt nicht auf die 28 Punkt des Systems zurueck.
    ///
    /// `addAttributes:` ersetzt den Absatzstil des Grundes ganz; baute
    /// [`einzugsmerkmal`] auf einem leeren Stil auf, verloere jede Listenzeile
    /// der Formatansicht die Tabulatoren wieder.
    #[test]
    fn der_listeneinzug_behaelt_den_tabulatorschritt() {
        let schrift = grundschrift(Ansicht::Format, Darstellungsart::Markdown);
        let schritt = TABSPALTEN * leerzeichenbreite(&schrift);
        let merkmale = einzugsmerkmal(2, &schrift);
        // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname des Absatzstils.
        let stil = unsafe { merkmale.objectForKey(NSParagraphStyleAttributeName) }
            .expect("der Einzug setzt einen Absatzstil")
            .downcast::<NSParagraphStyle>()
            .expect("unter dem Namen des Absatzstils steht ein Absatzstil");
        assert_eq!(stil.tabStops().count(), 0, "der Einzug fuehrt feste Stopps");
        assert!(
            (stil.defaultTabInterval() - schritt).abs() < f64::EPSILON,
            "der Einzug traegt den Schritt {} statt {schritt}",
            stil.defaultTabInterval()
        );
        assert!(
            (stil.headIndent() - 2.0 * LISTENEINZUG).abs() < f64::EPSILON,
            "der Einzug selbst ist verloren gegangen"
        );
    }

    /// Die Anschlagsmerkmale tragen nach [`anschlagsmerkmale`] den
    /// [`grundabsatz`] und behalten jedes andere Merkmal (Defekt 260929-1141,
    /// Quicknote und leer geoeffnete Datei).
    ///
    /// Ohne `NSTextView`, aus demselben Grund wie [`gelegt`]; dass beide
    /// Flaechen ueberhaupt hier hereinrufen, haelt die Probe darunter.
    #[test]
    fn die_anschlagsmerkmale_tragen_den_grundabsatz_und_behalten_den_rest() {
        let schrift = grundschrift(Ansicht::Roh, Darstellungsart::EinfacherText);
        let schritt = TABSPALTEN * leerzeichenbreite(&schrift);
        let vorhandene = schriftmerkmal(&schrift);
        let merkmale = anschlagsmerkmale(&vorhandene, &grundabsatz(&schrift));
        // SAFETY: Zwei Fremdsymbole von AppKit, Merkmalsnamen; nur gelesen.
        let (absatzname, schriftname) =
            unsafe { (NSParagraphStyleAttributeName, NSFontAttributeName) };
        let stil = merkmale
            .objectForKey(absatzname)
            .expect("die Anschlagsmerkmale tragen keinen Absatzstil")
            .downcast::<NSParagraphStyle>()
            .expect("unter dem Namen des Absatzstils steht ein Absatzstil");
        assert_eq!(
            stil.tabStops().count(),
            0,
            "der Anschlag fuehrt feste Stopps"
        );
        assert!(
            (stil.defaultTabInterval() - schritt).abs() < f64::EPSILON,
            "der Anschlag traegt den Schritt {} statt {schritt}",
            stil.defaultTabInterval()
        );
        assert!(
            merkmale.objectForKey(schriftname).is_some(),
            "die Schrift des Anschlags ist verloren gegangen"
        );
        assert_eq!(
            merkmale.count(),
            2,
            "die Vorgabe hat mehr als den Absatz geaendert"
        );
    }

    /// Editor und Quicknote nehmen Schrift und Grundabsatz aus
    /// [`grund_vorgeben`] und setzen die Schrift nirgends daran vorbei.
    ///
    /// Eine Quelltextprobe, weil die Wirkung an einer `NSTextView` haengt, die
    /// unter `libtest` nicht zu bauen ist. **Was sie faengt:** einen
    /// zurueckkehrenden `setFont:` mit der Grundschrift an einer der beiden
    /// Flaechen, der die Schrift ohne den Absatz setzte. **Was sie nicht
    /// sieht:** eine dritte bearbeitbare Flaeche, die weder das eine noch das
    /// andere tut.
    #[test]
    fn beide_bearbeitbaren_flaechen_nehmen_die_vorgabe_von_hier() {
        let flaechen = [
            ("editor.rs", include_str!("editor.rs"), 2),
            ("quicknote.rs", include_str!("quicknote.rs"), 1),
        ];
        for (name, quelle, erwartet) in flaechen {
            let ohne_proben = quelle
                .split("\n#[cfg(test)]\nmod ")
                .next()
                .unwrap_or(quelle);
            let rufe = ohne_proben
                .matches("textmerkmale::grund_vorgeben(&")
                .count();
            assert_eq!(
                rufe, erwartet,
                "{name}: {rufe} Rufe von grund_vorgeben statt {erwartet}"
            );
            assert!(
                !ohne_proben.contains("setFont(Some(&textmerkmale::grundschrift"),
                "{name}: setzt die Grundschrift an grund_vorgeben vorbei"
            );
        }
    }

    /// Die sechs Eingabepaare von [`grundmerkmale`], Zeile fuer Zeile.
    ///
    /// **Ohne AppKit-Objekt und ohne Hauptfaden.** `smallSystemFontSize` ist
    /// eine Klassenangabe von `NSFont` und baut nichts; die Probe fasst keine
    /// Ansicht an und braucht deshalb weder Fenster noch
    /// `MainThreadMarker::new_unchecked`. Gemessen wird die
    /// Fallunterscheidung und nicht die Zahl: welche Grundlage die kleine
    /// Systemschriftgroesse ist, entscheidet das System.
    ///
    /// **Was sie haelt:** die Zusage des Doc-Kommentars, dass es *eine* Regel
    /// ist und nicht drei — die Rohansicht traegt in jeder Darstellungsart
    /// feste Schrift in der Grundlage, die Formatansicht nur bei Code, und die
    /// beiden lesbaren Arten bekommen den Zuschlag. Sie faengt damit den
    /// Wiedereinzug einer zweiten Groessenrechnung, den `anwenden` bis zum
    /// 260907 trug.
    #[test]
    fn die_tafel_der_grundmerkmale() {
        let grundlage = NSFont::smallSystemFontSize();
        let faelle = [
            (
                Ansicht::Roh,
                Darstellungsart::EinfacherText,
                true,
                grundlage,
            ),
            (Ansicht::Roh, Darstellungsart::Code, true, grundlage),
            (Ansicht::Roh, Darstellungsart::Markdown, true, grundlage),
            (Ansicht::Format, Darstellungsart::Code, true, grundlage),
            (
                Ansicht::Format,
                Darstellungsart::EinfacherText,
                false,
                grundlage + LESEZUSCHLAG,
            ),
            (
                Ansicht::Format,
                Darstellungsart::Markdown,
                false,
                grundlage + LESEZUSCHLAG,
            ),
        ];

        for (ansicht, art, fest_erwartet, groesse_erwartet) in faelle {
            let (fest, groesse) = grundmerkmale(ansicht, art);
            assert_eq!(
                fest, fest_erwartet,
                "{ansicht:?}/{art:?}: die Schriftart der Grundschrift stimmt nicht"
            );
            assert!(
                (groesse - groesse_erwartet).abs() < f64::EPSILON,
                "{ansicht:?}/{art:?}: die Groesse ist {groesse} statt {groesse_erwartet}"
            );
        }
    }

    /// [`grundgroesse`] rechnet nichts nach, sondern liefert die zweite Haelfte
    /// von [`grundmerkmale`].
    ///
    /// Die Zusage des Doc-Kommentars, und sie ist die, an der `anwenden` haengt:
    /// eine Ueberschrift setzt auf derselben Grundlage auf wie der Text, ueber
    /// dem sie steht.
    #[test]
    fn die_grundgroesse_ist_die_zweite_haelfte_der_grundmerkmale() {
        for ansicht in [Ansicht::Roh, Ansicht::Format] {
            for art in [
                Darstellungsart::EinfacherText,
                Darstellungsart::Code,
                Darstellungsart::Markdown,
            ] {
                assert!(
                    (grundgroesse(ansicht, art) - grundmerkmale(ansicht, art).1).abs()
                        < f64::EPSILON,
                    "{ansicht:?}/{art:?}: die Groesse kommt aus einer zweiten Rechnung"
                );
            }
        }
    }

    /// Die sechs Ueberschriftsfaktoren fallen streng.
    ///
    /// „Absteigend, weil `#` mehr wiegt als `######`" steht als Begruendung an
    /// [`UEBERSCHRIFTSFAKTOREN`]; hier steht sie als Zusage. Die untere Schranke
    /// gehoert dazu: ein Faktor unter eins machte eine Ueberschrift kleiner als
    /// den Text darunter.
    #[test]
    fn die_ueberschriftsfaktoren_fallen_streng_und_bleiben_ueber_eins() {
        for paar in UEBERSCHRIFTSFAKTOREN.windows(2) {
            assert!(
                paar[0] > paar[1],
                "die Faktoren fallen nicht streng: {} steht vor {}",
                paar[0],
                paar[1]
            );
        }
        assert!(
            UEBERSCHRIFTSFAKTOREN.iter().all(|faktor| *faktor > 1.0),
            "ein Faktor liegt bei eins oder darunter; die Ueberschrift waere nicht groesser \
             als ihr Text"
        );
    }

    /// Die Liste traegt genau eine Zahl je Ueberschriftsstufe, die Markdown
    /// kennt.
    ///
    /// `anwenden` greift mit `stufe.clamp(1, 6) - 1` hinein; eine kuerzere Liste
    /// waere ein Zugriff hinter das Feld, eine laengere eine Zahl, die niemand
    /// liest.
    #[test]
    fn die_liste_traegt_eine_zahl_je_stufe() {
        assert_eq!(
            UEBERSCHRIFTSFAKTOREN.len(),
            6,
            "Markdown kennt sechs Ueberschriftsstufen, und `anwenden` greift auf `stufe - 1` zu"
        );
    }
}
