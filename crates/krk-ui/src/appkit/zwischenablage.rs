//! Die Beruehrungen mit dem System, die C10 braucht, und die **eine** Huelle
//! um `NSPasteboard`.
//!
//! ```text
//! NSPasteboard ──> lesen ────────> krk_core::zwischenablage::deuten
//!              │                            │
//!              │                    Ziel::Web ──> im_browser_oeffnen ──> NSWorkspace
//!              └─> inhalt_lesen ──> crate::vorschaumodell (Text, Bild, Leer)
//! ```
//!
//! [`lesen`] traegt seit S13 den Sprung, [`inhalt_lesen`] seit S19 die
//! Vorschau der Zwischenablage; eine zweite Huelle um `NSPasteboard` entsteht
//! dabei nicht. Alles in einem Modul, weil es die eine Frage beantwortet: was
//! steht in der Zwischenablage, und wohin geht KRK damit. Denselben Zuschnitt
//! zieht
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
//! **KRK schreibt die Zwischenablage in keinem Fall.** In dieser Datei steht
//! kein Aufruf, der das koennte; `setString:forType:` und `writeObjects:`
//! kommen nicht vor. Cmd+C und Cmd+V bleiben ab Werk unbelegt (C3).

use objc2_app_kit::{
    NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypePNG, NSPasteboardTypeString,
    NSPasteboardTypeTIFF, NSWorkspace,
};
use objc2_foundation::{NSString, NSURL};

use crate::vorschaumodell::Zwischenablageinhalt;

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
pub fn inhalt_lesen() -> Zwischenablageinhalt {
    let ablage = NSPasteboard::generalPasteboard();
    for sorte in [unsafe { NSPasteboardTypePNG }, unsafe {
        NSPasteboardTypeTIFF
    }] {
        if let Some(daten) = ablage.dataForType(sorte) {
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
