//! Die eine Umsetzung einer [`Formatierung`] in die Merkmale einer
//! `NSTextView` (C3).
//!
//! ```text
//!   crate::hervorhebung ──> Formatierung ──> textmerkmale::anwenden ──> NSTextView
//!        (rein, ohne AppKit)                  │                         ├─ NSTextStorage
//!                                             │                         └─ NSLayoutManager
//!                                             └─ zuruecksetzen (dieselbe Datei)
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
//! **Heute ruft allein [`super::editor`] hier herein.** Die Vorschau bekommt
//! ihre Auszeichnungen mit dem Schritt, der das gerenderte Markdown in ihre
//! Textflaeche traegt; bis dahin ist der zweite Verbraucher der Grund fuer den
//! Schnitt und noch nicht sein Nutzer. Wer die Umsetzung hier veraendert,
//! aendert sie deshalb schon jetzt fuer beide Flaechen.
//!
//! **Was hier nicht wohnt.** Welche Stellen welche Auszeichnung tragen, rechnet
//! [`crate::hervorhebung`] ohne AppKit aus; diese Datei setzt das Ergebnis um
//! und rechnet es nicht nach. Und sie zeichnet nichts nach: dass die geaenderten
//! Zeilenkaesten eine neue Nummernspalte brauchen, sagt der Rueckgabewert von
//! [`anwenden`] dem Aufrufer, der seine Flaeche kennt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextView` (`NSTextView.h:76`), `NSTextStorage` (`NSTextStorage.h:37`),
//! `NSFont` (`NSFont.h:24`), `NSColor` (`NSColor.h:77`),
//! `NSFontDescriptor` (`NSFontDescriptor.h:61`) und
//! `NSMutableParagraphStyle` (`NSParagraphStyle.h:112`) stehen seit macOS 10.0
//! zur Verfuegung. `NSLayoutManager` traegt im SDK `macos(10.7)`
//! (`NSLayoutManager.h:65`) und nicht die 10.0, die zwei andere Modulkoepfe
//! dieses Verzeichnisses fuer dieselbe Klasse nennen; die Zahl hier ist am SDK
//! gelesen. Das Buendel zielt auf 15.0 (`.cargo/config.toml`), keine von ihnen
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
//! `systemFontSize` (`NSFont.h:75`), `systemFontOfSize:` (`:47`),
//! `boldSystemFontOfSize:` (`:48`), `userFixedPitchFontOfSize:` (`:41`),
//! `firstLineHeadIndent` und `headIndent` (`NSParagraphStyle.h:116`, `:117`),
//! die drei Stuecke der kursiven Schrift — `fontDescriptor` an `NSFont`
//! (`NSFont.h:87`), `fontDescriptorWithSymbolicTraits:`
//! (`NSFontDescriptor.h:92`) und `fontWithDescriptor:size:` (`NSFont.h:31`),
//! dazu der Wert `NSFontDescriptorTraitItalic` (`NSFontDescriptor.h:22`) —
//! sowie `beginEditing`, `endEditing`, `addAttributes:range:` und
//! `removeAttribute:range:` an `NSMutableAttributedString`
//! (`NSAttributedString.h:85`, `:86`, `:76`, `:77`). Die vier Merkmalsnamen
//! tragen `macos(10.0)` (`NSAttributedString.h:26`, `:27`, `:28`, `:34`),
//! `NSUnderlineStyleSingle` keine Angabe (`:64`). Alle Zahlen am SDK gelesen.

use std::collections::HashMap;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSColor, NSFont, NSFontAttributeName, NSFontDescriptorSymbolicTraits,
    NSForegroundColorAttributeName, NSMutableParagraphStyle, NSParagraphStyleAttributeName,
    NSTextView, NSUnderlineStyle, NSUnderlineStyleAttributeName,
};
use objc2_foundation::{NSDictionary, NSNumber, NSRange, NSString};

use crate::editormodell::Ansicht;
use crate::hervorhebung::{Auszeichnung, Darstellungsart, Farbe, Formatierung};

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
const LESEZUSCHLAG: f64 = 2.0;

/// Um welchen Faktor eine Markdown-Ueberschrift ihre Grundschrift ueberschreitet,
/// nach Stufen von 1 bis 6.
///
/// Absteigend, weil `#` mehr wiegt als `######`. Die Zahlen sind gewaehlt und
/// nicht abgeleitet; sie halten die sechste Stufe noch merklich ueber dem
/// Fliesstext, damit keine Ueberschrift aussieht wie keine.
const UEBERSCHRIFTSFAKTOREN: [f64; 6] = [1.7, 1.5, 1.3, 1.2, 1.1, 1.05];

/// Der Einzug einer Markdown-Listenzeile in Punkten (C3).
///
/// Er rueckt den ganzen Absatz ein, das Aufzaehlungszeichen eingeschlossen; das
/// Zeichen selbst bleibt stehen, wie der Datensatz vom 260808-0140 es verlangt.
const LISTENEINZUG: f64 = 20.0;

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
    let grundgroesse = NSFont::systemFontSize() + LESEZUSCHLAG;
    speicher.beginEditing();
    for stelle in &formatierung.auszeichnungen {
        let bereich = NSRange::new(stelle.anfang, stelle.laenge);
        let merkmale = match stelle.art {
            Auszeichnung::Ueberschrift { stufe } => {
                let faktor = UEBERSCHRIFTSFAKTOREN[usize::from(stufe.clamp(1, 6)) - 1];
                schriftmerkmal(&NSFont::boldSystemFontOfSize(grundgroesse * faktor))
            }
            Auszeichnung::FesteSchrift => schriftmerkmal(&feste_schrift(grundgroesse)),
            Auszeichnung::Listenzeile => einzugsmerkmal(),
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
        // Fuer `addAttributes:range:` ist das ohne Belang, und zwar aus
        // einem Grund und nicht aus Glueck: die beiden ueberlappenden
        // Auszeichnungen setzen verschiedene Merkmalsnamen — Schrift gegen
        // Absatzstil —, und `addAttributes:` legt zusammen, statt zu
        // ersetzen. `Ueberschrift` und `FesteSchrift` setzen beide die
        // Schrift und ueberlappen einander deshalb nie: die
        // Fallunterscheidung in `crate::hervorhebung` fragt die
        // Ueberschriftsstufe zuerst und die feste Schrift nur sonst.
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
    let grundmerkmal = schriftmerkmal(&grundschrift(ansicht, art));
    // SAFETY: Speicher und Verwalter bringt die Flaeche selbst mit und wird
    // hier nur beschrieben; die Bereiche decken genau den vorhandenen Text.
    unsafe {
        if let Some(speicher) = text.textStorage() {
            let ganz = NSRange::new(0, speicher.length());
            speicher.removeAttribute_range(NSParagraphStyleAttributeName, ganz);
            speicher.addAttributes_range(&grundmerkmal, ganz);
            if let Some(verwalter) = text.layoutManager() {
                let leer: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
                verwalter.setTemporaryAttributes_forCharacterRange(&leer, ganz);
            }
        }
    }
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
/// **Sie steht hier und nicht bei ihren beiden Aufrufern.** Der Editor setzt sie
/// mit `setFont:` an der Flaeche und damit auch fuer den naechsten Anschlag;
/// [`zuruecksetzen`] setzt sie als Merkmal ueber den ganzen Textspeicher, um eine
/// weggefallene Auszeichnung zurueckzunehmen. Zwei Rechnungen daneben waeren die
/// erste Gelegenheit, dass eine geloeschte Ueberschrift in einer anderen Schrift
/// landete als der, in der ihre Zeile getippt wird.
pub fn grundschrift(ansicht: Ansicht, art: Darstellungsart) -> Retained<NSFont> {
    let (fest, groesse) = match (ansicht, art) {
        (Ansicht::Roh, _) | (Ansicht::Format, Darstellungsart::Code) => {
            (true, NSFont::systemFontSize())
        }
        (Ansicht::Format, Darstellungsart::EinfacherText | Darstellungsart::Markdown) => {
            (false, NSFont::systemFontSize() + LESEZUSCHLAG)
        }
    };
    if fest {
        feste_schrift(groesse)
    } else {
        NSFont::systemFontOfSize(groesse)
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
/// [`NSFontDescriptor`] beantwortet dieselbe Frage ohne diesen Anhang: er nimmt
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

/// Ein Merkmalsverzeichnis mit dem Einzug einer Listenzeile darin (C3).
fn einzugsmerkmal() -> Retained<NSDictionary<NSString, AnyObject>> {
    let stil = NSMutableParagraphStyle::new();
    // Beide, damit die erste Zeile mit dem Aufzaehlungszeichen genauso weit
    // einrueckt wie ihre Fortsetzung nach einem Umbruch; sonst haengt das
    // Zeichen als einziges am linken Rand.
    stil.setFirstLineHeadIndent(LISTENEINZUG);
    stil.setHeadIndent(LISTENEINZUG);
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
