//! Die Beruehrungen mit dem System, die C10 der Runde 1 und die beiden
//! Pfadkopierer aus C1 und C2 der Runde 4 brauchen, und die **eine** Huelle um
//! `NSPasteboard`.
//!
//! ```text
//! NSPasteboard ──> lesen ────────> krk_core::zwischenablage::deuten
//!              │                            │
//!              │                    Ziel::Web ──> im_browser_oeffnen ──> NSWorkspace
//!              ├─> inhalt_lesen ──> crate::vorschaumodell (Text, Bild, Leer)
//!              │
//!              └<── text_schreiben <── die beiden Pfadkopierer (C1, C2)
//! ```
//!
//! [`lesen`] traegt seit S13 den Sprung, [`inhalt_lesen`] seit S19 die
//! Vorschau der Zwischenablage, [`text_schreiben`] seit dem 260811 die
//! Gegenrichtung; eine zweite Huelle um `NSPasteboard` entsteht dabei nicht.
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
//! (`shift+cmd+c`) ab. Cmd+C und Cmd+V bleiben ab Werk unbelegt, wie es C3 der
//! Runde 1 zugesagt hat; die beiden Kopierer liegen daneben.
//!
//! **Geschrieben wird eine einzige Sorte, `NSPasteboardTypeString`.** Kein
//! Dateiverweis und kein `writeObjects:` — die Nutzerantwort vom 260811-1610
//! (`decisions/260811-1552_*_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md`).
//! Der Grund steht in einem Satz: ein Cmd+V, das im Finder eine Datei ablegt
//! und in einem Textfeld einen Pfad schreibt, waeren zwei Bedeutungen desselben
//! Befehls, und die zerstoererische von beiden sieht der Nutzer erst, nachdem
//! sie eingetreten ist. Eine Sorte heisst: eine Bedeutung. Sie ist zugleich die
//! Sorte, die [`lesen`] als zweite abfragt, und der Sprung aus der
//! Zwischenablage nimmt damit genau den Text, den KRK abgelegt hat.
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
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSPasteboard`, `NSWorkspace`, `NSString`, `NSURL` und `NSData` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso `setString:forType:` und `openURL:`;
//! `clearContents` und die vier abgefragten Sortenkonstanten
//! (`NSPasteboardTypeString`, `NSPasteboardTypeFileURL`, `NSPasteboardTypePNG`,
//! `NSPasteboardTypeTIFF`) seit 10.6. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine Verfuegbarkeitspruefung
//! zur Laufzeit. `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der
//! Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist die
//! Gegenmassnahme.

use objc2_app_kit::{
    NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypePNG, NSPasteboardTypeString,
    NSPasteboardTypeTIFF, NSWorkspace,
};
use objc2_foundation::{NSString, NSURL};

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

/// Legt einen Text als einzige Sorte in die Zwischenablage (C1, C2).
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
/// Warum allein `NSPasteboardTypeString` geschrieben wird und warum diese
/// Funktion keine Probe traegt, steht im Modulkopf.
pub fn text_schreiben(text: &str) -> bool {
    let ablage = NSPasteboard::generalPasteboard();
    ablage.clearContents();
    ablage.setString_forType(&NSString::from_str(text), unsafe { NSPasteboardTypeString })
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
