//! Die Beruehrungen mit dem System, die C10 der Runde 1 und die beiden
//! Pfadkopierer aus C1 und C2 der Runde 4 brauchen, und die **eine** Huelle um
//! `NSPasteboard`.
//!
//! ```text
//! die Zwischenablage des Nutzers
//! NSPasteboard::generalPasteboard
//!              │
//!              ├─> lesen ────────> krk_core::zwischenablage::deuten
//!              │                            │
//!              │                    Ziel::Web ──> im_browser_oeffnen ──> NSWorkspace
//!              ├─> inhalt_lesen ──> crate::vorschaumodell (Text, Bild, Leer)
//!              │
//!              ├<── text_schreiben <── die beiden Pfadkopierer (C1, C2)
//!              │          │
//!              │          └<── text_auf_ablage_schreiben (jede Ablage)
//!              │
//!              └<── dateiverweise_schreiben <── cmd+c und cmd+x im
//!                         │                    Dateifenster (Runde 22)
//!                         └<── dateiverweise_auf_ablage_schreiben (jede Ablage)
//!
//! die Ablage eines Ziehvorgangs
//! NSDraggingInfo::draggingPasteboard
//!              │
//!              └─> dateiverweise ──> Vec<PathBuf> (C4 und C7 der Runde 13)
//! ```
//!
//! [`lesen`] traegt seit S13 den Sprung, [`inhalt_lesen`] seit S19 die
//! Vorschau der Zwischenablage, [`text_schreiben`] seit dem 260811 die
//! Gegenrichtung, [`dateiverweise`] seit der Runde 13 den Abwurf, und
//! [`text_auf_ablage_schreiben`] seit der Runde 14 das Schreiben in eine
//! beliebige Ablage und [`dateiverweise_schreiben`] mit
//! [`dateiverweise_auf_ablage_schreiben`] seit der Runde 22 den zweiten
//! Ausgang, der Dateiverweise und Namen ablegt; eine zweite Huelle um
//! `NSPasteboard` entsteht dabei nicht.
//! Alles in einem Modul, weil es die eine Frage beantwortet: was steht in der
//! Zwischenablage, und wohin geht KRK damit. Die Frage ist mit der Runde 4 um
//! eine Richtung breiter geworden und geblieben, was sie war: eine. Denselben
//! Zuschnitt zieht
//! `appkit/volumes.rs`, wo die `NSWorkspace`-Beobachtung und die Aufzaehlung
//! ueber `NSFileManager` zusammen die eine Frage nach den Datentraegern
//! beantworten.
//!
//! **Die Deutung steht nicht hier.** Aus einer Zeichenkette einen Pfad, eine
//! Adresse oder nichts Verwertbares zu machen ist reines Rust und liegt in
//! `krk_core::zwischenablage`, wo es ohne Fenster pruefbar ist.
//!
//! # Zwei Sorten, eine Rangfolge
//!
//! Der Nutzer hat am 260804 entschieden, dass die Auswertung **Text und
//! Dateiverweis** liest
//! (`decisions/260804-0830_*_was-die-zwischenablage-auswertung-liest.md`).
//! [`lesen`] fragt deshalb zwei Sorten ab, in dieser Reihenfolge:
//!
//! 1. `NSPasteboardTypeFileURL`, den Dateiverweis. Wer im Finder eine Datei mit
//!    Cmd+C kopiert, legt ihn ab und daneben nur den blossen Namen als Text.
//! 2. `NSPasteboardTypeString`, den Text. Ein von Hand kopierter Pfad und ein
//!    aus einer Adresszeile kopierter Link liegen hier.
//!
//! Die Rangfolge ist keine Fallunterscheidung mit eigenem Rueckfallweg, sondern
//! die genauere Sorte vor der ungenaueren: ein Dateiverweis **ist** bereits ein
//! Pfad und braucht keine Deutung, ein Text muss erst als Pfad erkannt werden.
//! Beide muenden in dieselbe Auswertung, denn ein Dateiverweis kommt als
//! `file:`-Zeichenkette an.
//!
//! # Seit dem 260811 ist die Zwischenablage auch Ziel
//!
//! **Bis zur Runde 4 sagte dieser Kopf zu, KRK schreibe die Zwischenablage in
//! keinem Fall.** Die Zusage ist gebrochen und durch die Lage ersetzt, die C1
//! und C2 jener Runde bestellt haben: [`text_schreiben`] legt den Pfad des
//! angezeigten Ordners (`opt+cmd+c`) und die Pfade der betroffenen Eintraege
//! (`shift+cmd+c`) ab. **Cmd+C und Cmd+V tragen keinen der beiden.** Sie
//! tragen seit dem 260805 die Textbefehle des Menues „Bearbeiten", und genau
//! das hielt sie bis zur Runde 22 fuer eine Dateizwischenablage frei; der
//! Kopf von `resources/default-keymap.toml` schreibt den Wechsel aus, und die
//! Reservierung aus C3 der Runde 1 ist damit eingeloest und nicht gebrochen.
//! Seit der Runde 22 ist die `copy:`- und die `cut:`-Haelfte dieses
//! Einhaengepunkts besetzt (der Abschnitt unten), die `paste:`-Haelfte nicht.
//!
//! **Die zwei Pfadkopierer schreiben allein Text, `NSPasteboardTypeString`.**
//! Kein Dateiverweis und kein `writeObjects:` fuer sie — die Nutzerantwort vom
//! 260811-1610
//! (`decisions/260811-1552_*_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md`).
//! Der Grund steht in einem Satz: ein Cmd+V, das im Finder eine Datei ablegt
//! und in einem Textfeld einen Pfad schreibt, waeren zwei Bedeutungen desselben
//! Befehls, und die zerstoererische von beiden sieht der Nutzer erst, nachdem
//! sie eingetreten ist. Eine Sorte heisst: eine Bedeutung. Sie ist zugleich die
//! Sorte, die [`lesen`] als zweite abfragt, und der Sprung aus der
//! Zwischenablage nimmt damit genau den Text, den KRK abgelegt hat. **Der
//! Entscheid gilt fort**, auch nachdem die Huelle mit der Runde 22 Verweise
//! schreiben kann: der Name der zwei Befehle verspricht einen Pfad, und ein
//! Pfad ist Text.
//!
//! **[`text_schreiben`] traegt keine Probe, und das ist Absicht.**
//! `generalPasteboard` ist die Zwischenablage des angemeldeten Nutzers; eine
//! Probe, die sie beschriebe, wuerfe bei jedem `make check` weg, was der
//! Entwickler gerade kopiert hat. Geprueft wird stattdessen, was ohne AppKit
//! pruefbar ist: die Form der Pfade, der zusammengesetzte Text und die
//! Meldungen, alle in [`crate::kommandos::operationen`]. Dass
//! `setString:forType:` den Text wirklich ablegt, sieht der Nutzer am gebauten
//! Buendel mit einem Einfuegen.
//!
//! # Seit der Runde 13 vier Fragen an zwei Ablagen
//!
//! `NSPasteboard` ist nicht nur die Zwischenablage des angemeldeten Nutzers.
//! Jeder Ziehvorgang bringt seine eigene mit, und `draggingPasteboard` reicht
//! sie herein. [`dateiverweise`] beantwortet die vierte Frage dieses Moduls an
//! genau dieser zweiten Ablage: **welche Dateien auf dem Datentraeger traegt
//! sie?** Das Modul bleibt damit die eine Huelle. Der Gegenstand ist breiter
//! geworden, die Klasse dahinter ist dieselbe geblieben, und eine zweite
//! Huelle neben dieser waere der Fehler, den der Kopf oben ausdruecklich
//! vermeidet.
//!
//! **[`lesen`] bekommt dafuer keinen Parameter, und das ist die Absicht.** Es
//! beantwortet eine Frage an einen Gegenstand — was steht in der
//! Zwischenablage des Nutzers, als eine Zeichenkette — und sein einziger
//! Gegenstand steht deshalb im Rumpf und nicht in der Signatur. Ein Parameter
//! machte aus einer Funktion mit einer Bedeutung eine mit zweien, und jeder
//! seiner Aufrufer muesste denselben Wert einsetzen, damit sich nichts
//! aendert. [`dateiverweise`] nimmt seine Ablage entgegen, weil sie ihm von
//! AppKit gereicht wird und er sie nicht beschaffen kann.
//!
//! **`stringForType:` traegt hier nicht.** Es liefert **eine** Zeichenkette je
//! Sorte, und genau das ist der Zuschnitt von [`lesen`]: eine Datei, ein Pfad.
//! Ein Abwurf hat mehrere Eintraege, und mehrere Dateiverweise als eine
//! Zeichenkette zurueckzubekommen hiesse, sie hinterher wieder auseinander zu
//! schneiden — an einem Trennzeichen, das in einem Dateinamen vorkommen darf.
//! `readObjectsForClasses:options:` gibt stattdessen je Eintrag einen `NSURL`
//! her, und die Zerlegung entsteht gar nicht erst.
//!
//! **Eine leere Antwort ist kein Fehler.** Sie heisst „diese Ablage traegt
//! keine Datei auf dem Datentraeger" und ist genau die Auskunft, die C7 der
//! Runde 13 braucht, um eine Zusagedatei abzuweisen, ohne je eine einzuordnen:
//! KRK misst, was ihm gereicht wird, statt der abgebenden Anwendung eine
//! Diagnose zu stellen. Ein `NSURL` ohne `path` faellt aus demselben Grund
//! still weg — er benennt keine Stelle im Dateisystem, und mehr will diese
//! Funktion von ihm nicht wissen.
//!
//! # Seit der Runde 14 nimmt das Schreiben eine fremde Ablage entgegen
//!
//! **Bis zur Runde 14 kannte diese Huelle beim Schreiben nur eine Ablage**:
//! [`text_schreiben`] griff `generalPasteboard` selbst. Das genuegt fuer die
//! beiden Pfadkopierer, nicht aber fuer einen Ausgabeweg, der seine Ablage von
//! AppKit gereicht bekommt, so wie [`dateiverweise`] sie beim Lesen schon
//! gereicht bekommt. [`text_auf_ablage_schreiben`] nimmt deshalb die Ablage als
//! Parameter entgegen und traegt den Rumpf, den [`text_schreiben`] bis dahin
//! trug; [`text_schreiben`] reicht ihr seinerseits `generalPasteboard` hinein
//! und aendert sein Verhalten dadurch nicht. Es ist derselbe Griff wie bei
//! [`dateiverweise`]: die Huelle beantwortet die Frage nach der Zwischenablage,
//! und welche es ist, entscheidet der Rufer.
//!
//! # Seit der Runde 22 schreibt die Huelle zwei Sorten
//!
//! `cmd+c` und `cmd+x` im Dateifenster versprechen nicht einen Pfad, sondern
//! die Datei, und die Sorte, die das einloest, ist der Dateiverweis:
//! [`dateiverweise_auf_ablage_schreiben`] legt je Eintrag ein Datei-`NSURL`
//! ueber `writeObjects:` ab und daneben die Namen als Text, einer je Zeile,
//! wie der Finder es tut (A3 der Runde 22). Ein Einfuegen im Finder legt
//! damit die Dateien ab, ein Einfuegen in ein Textfeld die Namen; das sind
//! zwei Sorten fuer zwei Arten von Ziel und nicht zwei Bedeutungen eines
//! Befehls, denn das Ziel waehlt die Sorte, die es lesen kann. Den Text der
//! Pfadkopierer beruehrt das nicht (der Abschnitt darueber).
//!
//! **Erst `writeObjects:`, dann `setString:forType:`, und das ist keine
//! Reihenfolge nach Belieben.** `setString:forType:` setzt seine Sorte am
//! **ersten** Ablageeintrag und legt keinen neuen an (`NSPasteboard.h`, „on
//! the first pasteboard item"); `writeObjects:` legt je `NSURL` einen Eintrag
//! an. Kaeme der Text zuerst, laege er auf einem Eintrag ohne Verweis, und die
//! Verweise folgten als zweiter bis n-ter. So liegen Verweis und Namen auf dem
//! ersten Eintrag beieinander, und [`lesen`], das `stringForType:` und damit
//! ebenfalls den ersten Eintrag fragt, findet dort den Dateiverweis vor dem
//! Text: der Sprung aus der Zwischenablage geht zum ersten kopierten Eintrag
//! (C2.5 der Runde 22).
//!
//! **Die Namen kommen fertig herein, als eine Zeichenkette.** Die Huelle
//! deutet nicht (der Kopf oben, „Die Deutung steht nicht hier"), und sie
//! setzt auch keinen Text zusammen; das tut `namenszeilen` in
//! [`crate::kommandos::operationen`], wo die Statuszeile denselben Namen
//! bezieht. So ist der Name in der Meldung derselbe wie der in der Ablage.
//!
//! **Welche Sorten ein Datei-`NSURL` neben dem Verweis von sich aus ablegt,
//! ist am Buendel zu messen und nicht hier entschieden.** Die Probe
//! `der_zweite_ausgang_legt_verweise_und_namen_ab` liest genau
//! `NSPasteboardTypeString` zurueck und wird rot, sobald eine mitgeschriebene
//! Sorte die Namenszeilen verdraengt; die Sortenliste selbst nennt der
//! Abnahmelauf der Runde 22.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSPasteboard`, `NSWorkspace`, `NSString`, `NSURL`, `NSNumber`, `NSArray`,
//! `NSDictionary` und `NSData` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `setString:forType:`, `openURL:`, `path`, `pasteboardWithName:`
//! (`NSPasteboard.h:160`) und `fileURLWithPath:` (`NSURL.h:52`, ohne
//! `API_AVAILABLE`, am SDK 15 nachgelesen; die Fassung mit `isDirectory:`
//! daneben steht seit 10.5 und wird nicht gerufen). Seit 10.6 stehen
//! `clearContents`, `writeObjects:`, das Protokoll `NSPasteboardWriting`, das
//! `NSURL` erfuellt (`NSPasteboard.h:386`, `:379`),
//! `readObjectsForClasses:options:` (`NSPasteboard.h:190`), der
//! Vorgabeschluessel `NSPasteboardURLReadingFileURLsOnlyKey`
//! (`NSPasteboard.h:146`) und drei der vier abgefragten Sortenkonstanten:
//! `NSPasteboardTypeString`, `NSPasteboardTypePNG` und `NSPasteboardTypeTIFF`
//! (`NSPasteboard.h:24`, `:27`, `:26`).
//!
//! **`NSPasteboardTypeFileURL` steht seit 10.13** (`NSPasteboard.h:39`) und
//! nicht seit 10.6, wie diese Stelle bis zur Runde 13 sagte. Die Zahl ist am
//! SDK nachgelesen und berichtigt; an der Untergrenze des Buendels aendert sie
//! nichts, weil 10.13 weit darunter liegt.
//!
//! Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine Verfuegbarkeitspruefung
//! zur Laufzeit. `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der
//! Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist die
//! Gegenmassnahme.

use std::path::PathBuf;

use objc2::ClassType;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_app_kit::{
    NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypePNG, NSPasteboardTypeString,
    NSPasteboardTypeTIFF, NSPasteboardURLReadingFileURLsOnlyKey, NSPasteboardWriting, NSWorkspace,
};
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSString, NSURL};

use crate::vorschaumodell::{BILDGRENZE, Zwischenablageinhalt};

/// Was in der Zwischenablage steht, als eine Zeichenkette.
///
/// `None`, wenn sie weder einen Dateiverweis noch Text traegt, etwa nach dem
/// Kopieren eines Bildes. Der Aufrufer meldet das als nicht verwertbar; die
/// Anzeige eines Bildes ist die andere Funktion aus C10 und gehoert zu Schritt
/// 19.
pub fn lesen() -> Option<String> {
    let ablage = NSPasteboard::generalPasteboard();
    for sorte in [unsafe { NSPasteboardTypeFileURL }, unsafe {
        NSPasteboardTypeString
    }] {
        if let Some(inhalt) = ablage.stringForType(sorte) {
            let text = inhalt.to_string();
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Was in der Zwischenablage steht, fuer die Vorschau aus C10 gedeutet.
///
/// Die Dreiteilung der Anzeige: ein Bild als Bild, Text als Text, und eine
/// leere Zwischenablage als ausdrueckliche Meldung. Das Bild kommt **vor** dem
/// Text, weil ein kopiertes Bild oft eine Textbeschreibung neben sich traegt,
/// etwa die Adresse, von der es stammt; wer ein Bild kopiert, will das Bild
/// sehen. Ein im Finder kopierter Eintrag legt kein Bild ab und erscheint
/// deshalb als sein `file:`-Verweis, also als Text.
///
/// Gefragt werden PNG und TIFF, die beiden Bildsorten, die `NSPasteboard`
/// selbst fuehrt; jede bildgebende Anwendung legt mindestens eine davon ab,
/// und `NSImage` liest beide.
///
/// **Die Bildgrenze aus C6 gilt hier genauso wie im Dateiweg.** Sie steht
/// **vor** `to_vec()`, aus demselben Grund, aus dem sie in
/// [`crate::vorschaumodell`] vor `std::fs::read` steht: danach laege das Bild
/// schon im Arbeitsspeicher, und genau das verhindert die Grenze. Ein in einem
/// Bildbearbeitungsprogramm kopiertes TIFF liegt ohne Weiteres ueber 100 MB.
/// Gefragt wird die Laenge des `NSData`, nicht seine Bytes; die Daten bleiben
/// dabei im Pasteboard-Server liegen, wo sie ohnehin schon stehen. Die Zahl
/// selbst kommt aus [`BILDGRENZE`], eine zweite entsteht nicht.
pub fn inhalt_lesen() -> Zwischenablageinhalt {
    let ablage = NSPasteboard::generalPasteboard();
    for sorte in [unsafe { NSPasteboardTypePNG }, unsafe {
        NSPasteboardTypeTIFF
    }] {
        if let Some(daten) = ablage.dataForType(sorte) {
            let laenge = daten.len() as u64;
            if laenge > BILDGRENZE {
                return Zwischenablageinhalt::BildZuGross(laenge);
            }
            let bytes = daten.to_vec();
            if !bytes.is_empty() {
                return Zwischenablageinhalt::Bild(bytes);
            }
        }
    }
    match lesen() {
        Some(text) => Zwischenablageinhalt::Text(text),
        None => Zwischenablageinhalt::Leer,
    }
}

/// Legt einen Text als einzige Sorte in eine beliebige Ablage (C1, C2 der
/// Runde 4; C2.10 der Runde 14).
///
/// Liefert, ob das System ihn angenommen hat. Der Aufrufer meldet das in der
/// Statuszeile; wortlos nichts zu tun ist in keinem der beiden Faelle zulaessig.
///
/// **`clearContents` ist keine Vorsichtsmassnahme, sondern Bedingung.** Ohne
/// den Aufruf nimmt `setString:forType:` den Text nicht an, weil die Ablage
/// noch dem vorigen Besitzer gehoert. Er ist zugleich die Zusage aus C1, dass
/// ein zweiter Aufruf den Inhalt **ersetzt** und nichts anhaengt: das Leeren
/// nimmt jede Sorte weg, die vorher darin stand, auch den Dateiverweis eines im
/// Finder kopierten Eintrags.
///
/// Warum allein `NSPasteboardTypeString` geschrieben wird, steht im
/// Modulkopf. Die Ablage kommt herein und wird nicht beschafft, wie bei
/// [`dateiverweise`]: welche es ist, entscheidet der Rufer, nicht diese
/// Huelle.
#[must_use]
pub fn text_auf_ablage_schreiben(ablage: &NSPasteboard, text: &str) -> bool {
    ablage.clearContents();
    ablage.setString_forType(&NSString::from_str(text), unsafe { NSPasteboardTypeString })
}

/// Legt einen Text als einzige Sorte in die Zwischenablage des Nutzers (C1, C2).
///
/// Reicht `NSPasteboard::generalPasteboard()` an [`text_auf_ablage_schreiben`]
/// hinein; das Verhalten fuer die beiden Pfadkopierer aus C1 und C2 der Runde 4
/// bleibt dabei unveraendert. Warum diese Funktion keine Probe traegt, steht im
/// Modulkopf.
pub fn text_schreiben(text: &str) -> bool {
    text_auf_ablage_schreiben(&NSPasteboard::generalPasteboard(), text)
}

/// Legt Dateiverweise und daneben ihre Namen als Text in eine beliebige
/// Ablage (C1, C2 der Runde 22).
///
/// Liefert, ob die Ablage beides angenommen hat. Der Aufrufer meldet das in
/// der Statuszeile; wortlos nichts zu tun ist in keinem Fall zulaessig, und
/// deshalb traegt der Wert `#[must_use]`.
///
/// **`clearContents` ist Bedingung und keine Vorsichtsmassnahme**, wie bei
/// [`text_auf_ablage_schreiben`]: ohne den Aufruf gehoert die Ablage noch dem
/// vorigen Besitzer, und es ist zugleich die Zusage, dass ein zweites Ablegen
/// das erste **ersetzt** und nichts anhaengt. `writeObjects:` allein haengte
/// an: es legt je Objekt einen weiteren Eintrag hinter die bestehenden.
///
/// **Die Namen kommen fertig herein und werden nicht aus den Pfaden
/// gebildet**, weil die Huelle nicht deutet; `namenszeilen` in
/// [`crate::kommandos::operationen`] baut sie, und die Statuszeile bezieht
/// ihren Namen aus derselben Quelle. Warum `setString:forType:` **nach**
/// `writeObjects:` steht und damit am ersten Eintrag landet, steht im
/// Modulkopf unter „Seit der Runde 22 schreibt die Huelle zwei Sorten".
///
/// **`fileURLWithPath:` kostet je Eintrag ein `stat(2)` und kein Oeffnen.**
/// Der Erzeuger fragt das Dateisystem, ob der Pfad ein Verzeichnis ist, um
/// den abschliessenden Schraegstrich zu setzen; eine Datei wird dabei nicht
/// geoeffnet, und eine Verknuepfung nicht aufgeloest: der Verweis nennt den
/// Pfad der Verknuepfung (A7 der Runde 22). Die Schreibseite ist fuer grosse
/// Mengen ungemessen; die Leseseite in [`dateiverweise`] nennt ihre Zahlen.
///
/// **Die leere Menge entscheidet der Rufer und nicht die Huelle.** Mit leeren
/// `pfade` legte `writeObjects:` keinen Eintrag an und `setString:forType:`
/// einen allein mit Text; das waere eine Textablage unter dem Namen einer
/// Dateiablage. Der Rufer in `appkit/tabelle.rs` meldet die leere Menge
/// vorher (C1.7 der Runde 22) und erreicht diese Funktion damit nicht.
#[must_use]
pub fn dateiverweise_auf_ablage_schreiben(
    ablage: &NSPasteboard,
    pfade: &[PathBuf],
    namen: &str,
) -> bool {
    ablage.clearContents();

    let verweise: Vec<Retained<NSURL>> = pfade
        .iter()
        .map(|pfad| NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy())))
        .collect();
    // `NSURL` erfuellt `NSPasteboardWriting` (`NSPasteboard.h:379`); die
    // Umwandlung steht hier und nicht beim Aufrufer, damit nichts anderes
    // hereinkommt.
    let schreiber: Vec<&ProtocolObject<dyn NSPasteboardWriting>> = verweise
        .iter()
        .map(|url| ProtocolObject::from_ref(&**url))
        .collect();
    if !ablage.writeObjects(&NSArray::from_slice(&schreiber)) {
        return false;
    }

    ablage.setString_forType(&NSString::from_str(namen), unsafe {
        NSPasteboardTypeString
    })
}

/// Legt Dateiverweise und ihre Namen in die Zwischenablage des Nutzers
/// (`cmd+c` und `cmd+x` im Dateifenster, Runde 22).
///
/// Reicht `NSPasteboard::generalPasteboard()` an
/// [`dateiverweise_auf_ablage_schreiben`] hinein, nach dem Muster von
/// [`text_schreiben`], und traegt aus demselben Grund keine Probe: eine Probe
/// an der Ablage des angemeldeten Nutzers wuerfe bei jedem `make check` weg,
/// was der Entwickler gerade kopiert hat. Der Modulkopf schreibt es aus.
#[must_use]
pub fn dateiverweise_schreiben(pfade: &[PathBuf], namen: &str) -> bool {
    dateiverweise_auf_ablage_schreiben(&NSPasteboard::generalPasteboard(), pfade, namen)
}

/// Uebergibt eine Web-Adresse an den Systembrowser (C10).
///
/// Liefert, ob das System sie angenommen hat. KRK zeigt selbst keinen
/// Web-Inhalt an, haelt keinen Verlauf und traegt keine Ansicht dafuer; damit
/// bleibt es innerhalb der Grenze des Circles, die einen integrierten Browser
/// ausschliesst.
///
/// **Nur `http:` und `https:` erreichen diesen Aufruf.** Die Grenze zieht die
/// Deutung im Kern, und der Grund ist C9: gaebe KRK ein `smb:` oder `ftp:` an
/// das System, baute es ueber einen Umweg die Serververbindung auf, die C9
/// ausschliesst.
pub fn im_browser_oeffnen(adresse: &str) -> bool {
    let Some(url) = NSURL::URLWithString(&NSString::from_str(adresse)) else {
        return false;
    };
    NSWorkspace::sharedWorkspace().openURL(&url)
}

/// Welche Dateien auf dem Datentraeger die Ablage eines Ziehvorgangs traegt
/// (C4, C7 der Runde 13).
///
/// Die Ablage kommt herein und wird nicht beschafft: `draggingPasteboard`
/// reicht sie, und sie ist nicht die Zwischenablage des Nutzers. Warum
/// [`lesen`] dafuer keinen Parameter bekommt und warum `stringForType:` hier
/// nicht traegt, steht im Modulkopf.
///
/// **Gefragt wird nach `NSURL`, und der Vorgabeschluessel
/// `NSPasteboardURLReadingFileURLsOnlyKey` schneidet die Antwort auf
/// Dateiverweise zu.** Ohne ihn kaeme auch eine aus einer Adresszeile gezogene
/// Web-Adresse als `NSURL` zurueck, und KRK nennte sie einen Abwurf. Der
/// Schluessel ist der eine Ort, an dem diese Grenze steht.
///
/// **Der Aufruf kostet, und er kostet linear in der Zahl der gezogenen
/// Eintraege.** `readObjectsForClasses:options:` baut je Eintrag ein `NSURL`
/// ueber den Ablageserver, und darauf folgt je Eintrag ein `PathBuf`. Am
/// 260819 auf dem Referenzgeraet gemessen, im Profil `release` und je
/// Durchgang: 1 Eintrag 0,13 ms, 10 Eintraege 0,65 ms, 100 Eintraege 6,0 ms,
/// 1.000 Eintraege 155 ms, 5.000 Eintraege 585 ms. Wer diese Funktion in einen
/// Weg legt, der bei jeder Zeigerbewegung laeuft, legt diese Zahlen in jedes
/// Bild; `DateifensterQuelle::abwurf_pruefen` tut das seit dem 260819 nicht
/// mehr, sondern merkt sich die Antwort je Ziehsitzung.
///
/// **Ein leerer Vektor ist kein Fehler**, sondern die Antwort „diese Ablage
/// traegt keine Datei auf dem Datentraeger". Genau diese Antwort weist die
/// Zusagedatei aus C7 ab, ohne dass KRK eine solche je einordnen muesste: es
/// misst, was ihm gereicht wird, statt der abgebenden Anwendung eine Diagnose
/// zu stellen. Ein `NSURL` ohne `path` faellt aus demselben Grund still weg.
pub fn dateiverweise(ablage: &NSPasteboard) -> Vec<PathBuf> {
    let klassen = NSArray::from_slice(&[NSURL::class()]);
    let ja = NSNumber::new_bool(true);
    // SAFETY: Ein Fremdsymbol von AppKit, der Name des Vorgabeschluessels. Er
    // wird gelesen und nicht geschrieben, wie die vier Sortenkonstanten oben.
    let nur_dateien = unsafe { NSPasteboardURLReadingFileURLsOnlyKey };
    let vorgaben = NSDictionary::from_slices(&[nur_dateien], &[ja.as_ref() as &AnyObject]);

    // SAFETY: Die beiden Anforderungen der Bindung sind erfuellt. Die
    // Klassenliste traegt allein `NSURL`, eine Klasse, die
    // `NSPasteboardReading` umsetzt (`NSPasteboard.h:409`); die Vorgabenliste
    // traegt allein `NSPasteboardURLReadingFileURLsOnlyKey`, und dessen Wert
    // ist laut `NSPasteboard.h:144` ein `NSNumber` mit einem Wahrheitswert.
    let Some(eintraege) =
        (unsafe { ablage.readObjectsForClasses_options(&klassen, Some(&vorgaben)) })
    else {
        return Vec::new();
    };

    eintraege
        .iter()
        .filter_map(|eintrag| eintrag.downcast::<NSURL>().ok())
        .filter_map(|url| url.path())
        .map(|pfad| PathBuf::from(pfad.to_string()))
        .collect()
}

#[cfg(test)]
mod proben {
    use super::*;
    use crate::pruefordner::Pruefordner;
    use crate::quellbaum::quelldateien;

    /// Eine Ablage, die niemandem sonst gehoert.
    ///
    /// **`generalPasteboard` wird hier nicht angefasst**, aus demselben Grund,
    /// aus dem [`text_schreiben`] keine Probe traegt: sie wuerfe bei jedem
    /// `make check` weg, was der Entwickler gerade kopiert hat. Der Modulkopf
    /// schreibt es aus. Jede Probe legt sich stattdessen ueber
    /// `pasteboardWithName:` eine eigene an und leert sie als Erstes.
    ///
    /// **Der Name ist fest und nicht eindeutig**, obwohl
    /// `pasteboardWithUniqueName` daneben stuende. `objc2-app-kit 0.3.2` bindet
    /// `releaseGlobally` nicht, und eine eindeutig benannte Ablage bliebe damit
    /// beim Pasteboard-Server stehen, ohne dass diese Probe sie wieder abgeben
    /// koennte — je Lauf eine weitere. Ein fester Name je Probe haelt die Zahl
    /// bei der Zahl der Proben, die eine Ablage brauchen, und `clearContents`
    /// macht den Anfangszustand jedes Laufs gleich.
    fn probenablage(zweck: &str) -> objc2::rc::Retained<NSPasteboard> {
        let ablage = NSPasteboard::pasteboardWithName(&NSString::from_str(&format!(
            "com.krk.probe.{zweck}"
        )));
        let _ = ablage.clearContents();
        ablage
    }

    /// Legt die Pfade ueber den zweiten Ausgang der Huelle in die Ablage.
    ///
    /// Bis zur Runde 22 stand hier `dateien_ablegen`, ein eigener Schreiber
    /// ueber `writeObjects:` allein fuer die Proben; seit die Huelle selbst
    /// Verweise schreibt, ist er ein zweiter Schreiber neben ihr und faellt.
    fn dateien_ablegen(ablage: &NSPasteboard, pfade: &[PathBuf], namen: &str) {
        assert!(
            dateiverweise_auf_ablage_schreiben(ablage, pfade, namen),
            "die Probenablage nimmt die Dateiverweise und die Namen an"
        );
    }

    fn zeichenkette(ablage: &NSPasteboard) -> Option<String> {
        ablage
            .stringForType(unsafe { NSPasteboardTypeString })
            .map(|text| text.to_string())
    }

    #[test]
    fn zwei_dateiverweise_kommen_als_zwei_pfade_zurueck() {
        let ordner = Pruefordner::neu("dateiverweise");
        let erste = ordner.datei("erste.txt", b"eins");
        let zweite = ordner.datei("zweite.txt", b"zwei");

        let ablage = probenablage("dateiverweise");
        dateien_ablegen(&ablage, &[erste.clone(), zweite.clone()], "");

        assert_eq!(
            dateiverweise(&ablage),
            vec![erste, zweite],
            "C4: jeder gezogene Eintrag kommt mit seinem Pfad zurueck, in der Reihenfolge der Ablage"
        );
    }

    #[test]
    fn der_zweite_ausgang_legt_verweise_und_namen_ab() {
        let ordner = Pruefordner::neu("zweiter-ausgang");
        let erste = ordner.datei("erste.txt", b"eins");
        let zweite = ordner.datei("zweite.txt", b"zwei");

        let ablage = probenablage("zweiter-ausgang");
        dateien_ablegen(
            &ablage,
            &[erste.clone(), zweite.clone()],
            "erste.txt\nzweite.txt",
        );

        assert_eq!(
            dateiverweise(&ablage),
            vec![erste, zweite],
            "C1.4: die Verweise kommen in der Reihenfolge der Pfade zurueck"
        );
        assert_eq!(
            zeichenkette(&ablage).as_deref(),
            Some("erste.txt\nzweite.txt"),
            "C2.7: die Namen stehen als Text daneben, einer je Zeile, und keine \
             vom `NSURL` mitgeschriebene Sorte verdraengt sie"
        );
    }

    #[test]
    fn eine_verknuepfung_wird_als_verknuepfung_abgelegt() {
        let ordner = Pruefordner::neu("verknuepfung");
        let ziel = ordner.datei("ziel.txt", b"ziel");
        let verknuepfung = ordner.unter("verknuepfung.txt");
        std::os::unix::fs::symlink(&ziel, &verknuepfung)
            .expect("die Verknuepfung im Pruefordner laesst sich anlegen");

        let ablage = probenablage("verknuepfung");
        dateien_ablegen(
            &ablage,
            std::slice::from_ref(&verknuepfung),
            "verknuepfung.txt",
        );

        assert_eq!(
            dateiverweise(&ablage),
            vec![verknuepfung],
            "C1.9: der Verweis nennt die Verknuepfung und nicht ihr Ziel"
        );
    }

    #[test]
    fn ein_zweites_ablegen_ersetzt_das_erste() {
        let ordner = Pruefordner::neu("ersetzen");
        let alte = ordner.datei("alte.txt", b"alt");
        let neue = ordner.datei("neue.txt", b"neu");

        let ablage = probenablage("ersetzen");
        dateien_ablegen(&ablage, &[alte], "alte.txt");
        dateien_ablegen(&ablage, std::slice::from_ref(&neue), "neue.txt");

        assert_eq!(
            dateiverweise(&ablage),
            vec![neue],
            "das zweite Ablegen ersetzt die Verweise des ersten und haengt nichts an"
        );
        assert_eq!(
            zeichenkette(&ablage).as_deref(),
            Some("neue.txt"),
            "das zweite Ablegen ersetzt auch die Namen des ersten"
        );
    }

    #[test]
    fn eine_leere_ablage_liefert_einen_leeren_vektor() {
        let ablage = probenablage("leer");

        assert!(
            dateiverweise(&ablage).is_empty(),
            "C7: keine Datei auf dem Datentraeger ist eine Antwort und kein Fehler"
        );
    }

    #[test]
    fn text_auf_ablage_schreiben_legt_den_text_in_die_gereichte_ablage() {
        let ablage = probenablage("text-schreiben");

        assert!(
            text_auf_ablage_schreiben(&ablage, "geschriebener Text"),
            "C2.10: die gereichte Ablage nimmt den Text an"
        );

        let zurueckgelesen = ablage
            .stringForType(unsafe { NSPasteboardTypeString })
            .expect("die Probenablage traegt die Sorte, die eben geschrieben wurde")
            .to_string();
        assert_eq!(
            zurueckgelesen, "geschriebener Text",
            "der geschriebene Text kommt unveraendert zurueck"
        );
    }

    /// Die Huelle um `NSPasteboard` steht in genau einer Datei (C2.10 und die
    /// erste Haelfte von C4.7 der Runde 14).
    ///
    /// **Die aelteste Zusage dieses Moduls, zum ersten Mal gezaehlt.** Der
    /// Modulkopf traegt sie seit der Runde 1 — es gibt genau eine Huelle um die
    /// Zwischenablage —, und bis zur Runde 14 hat nichts sie nachgemessen. Der
    /// Anlass ist die Runde selbst: sie legt mit
    /// [`Vorschautext::auswahl_ablegen`](super::super::vorschau) einen zweiten
    /// Rufer aus einer ganz anderen Ecke des Programms an, und der
    /// naechstliegende und falsche Weg dorthin waere gewesen, dort selbst zu
    /// schreiben.
    ///
    /// **Drei Nadeln, weil die Huelle seit der Runde 22 drei Griffe hat.**
    /// Der erste ist das Schreiben von Text in eine Ablage, der zweite der
    /// Griff nach der Ablage des angemeldeten Nutzers, der dritte seit der
    /// Runde 22 das Schreiben von Objekten ueber `writeObjects:`. Sie koennten
    /// einzeln abwandern: wer anderswo in eine gereichte Ablage schriebe,
    /// umginge den ersten oder den dritten; wer sich anderswo die allgemeine
    /// Ablage holte, den zweiten. Bis zur Runde 22 stand der dritte Griff
    /// allein im Pruefmodul dieser Datei und war deshalb keine Nadel. Der
    /// Modulkopf begruendet, warum gerade der zweite Griff hier bleiben muss.
    ///
    /// **Erwartet wird die Datei und nicht die Zahl der Fundstellen.** Die
    /// Zusage ist eine ueber den Ort und keine ueber eine Menge: dass der Griff
    /// nach der allgemeinen Ablage in dieser Datei heute an drei Zeilen steht
    /// und morgen an zwei, ist keine Aenderung, gegen die diese Probe stehen
    /// soll. Der Plan der Runde hat an zwei anderen Stellen eine Zahl
    /// vorweggenommen, die am Baum nicht zutraf
    /// (`issues/260820-0646_*_der-plan-schreibt-zaehlerwartungen-ohne-sie-gegen-den-baum-zu-halten-dreimal-in-einer-runde.md`);
    /// hier ist die Zahl von vornherein nicht die Zusage.
    ///
    /// # Was diese Nadeln nicht sehen
    ///
    /// **Sie zaehlen Codezeilen.** Ein Aufruf, der zwischen dem Namen und
    /// seinem Argument umbricht, entgeht ihnen, und ob dieselbe Sache anderswo
    /// ueber `writeObjects:` oder `setData:forType:` noch einmal abgelegt wird,
    /// entscheidet keine Suche im Quelltext. Der Kopf von [`crate::quellbaum`]
    /// sagt, was daraus folgt.
    ///
    /// **Die zweite Haelfte von C4.7 misst diese Probe nicht.** Die drei
    /// Pruefordner-Fassungen zaehlt seit der Runde 1
    /// `genau_drei_pruefordner_fassungen_stehen_im_baum` in
    /// `krk-core/tests/baum.rs`; eine zweite Zaehlung daneben waere der
    /// Doppelbau, gegen den beide stehen.
    #[test]
    fn die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei() {
        // Beide Nadeln stehen zusammengesetzt da: die Probe liegt in dem Baum,
        // den sie liest, und als ein Stueck geschrieben faende jede sich selbst.
        let schreiben = concat!("setString", "_forType");
        let allgemeine_ablage = concat!("general", "Pasteboard");
        let objekte_schreiben = concat!("write", "Objects");
        let huelle = "krk-ui/src/appkit/zwischenablage.rs";

        let dateien = quelldateien();
        let traeger = |nadel: &str| -> Vec<String> {
            dateien
                .iter()
                .filter(|(_, inhalt)| {
                    inhalt
                        .lines()
                        .filter(|zeile| !zeile.trim_start().starts_with("//"))
                        .any(|zeile| zeile.contains(nadel))
                })
                .map(|(name, _)| name.clone())
                .collect()
        };

        assert_eq!(
            traeger(schreiben),
            vec![huelle.to_owned()],
            "`{schreiben}` steht nicht allein in der einen Huelle um die \
             Zwischenablage; ein zweiter Schreiber daneben waere eine zweite \
             Meinung darueber, wie ein Text abgelegt wird"
        );
        assert_eq!(
            traeger(allgemeine_ablage),
            vec![huelle.to_owned()],
            "`{allgemeine_ablage}` steht nicht allein in der einen Huelle um \
             die Zwischenablage; wer sich die Ablage des Nutzers anderswo holt, \
             umgeht die Huelle an ihrer zweiten Haelfte"
        );
        assert_eq!(
            traeger(objekte_schreiben),
            vec![huelle.to_owned()],
            "`{objekte_schreiben}` steht nicht allein in der einen Huelle um die \
             Zwischenablage; ein zweiter Schreiber von Dateiverweisen daneben \
             waere eine zweite Meinung darueber, wie ein Verweis abgelegt wird"
        );
    }
}
