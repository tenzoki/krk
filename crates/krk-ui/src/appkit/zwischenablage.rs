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
//!              └<── text_schreiben <── die beiden Pfadkopierer (C1, C2)
//!
//! die Ablage eines Ziehvorgangs
//! NSDraggingInfo::draggingPasteboard
//!              │
//!              └─> dateiverweise ──> Vec<PathBuf> (C4 und C7 der Runde 13)
//! ```
//!
//! [`lesen`] traegt seit S13 den Sprung, [`inhalt_lesen`] seit S19 die
//! Vorschau der Zwischenablage, [`text_schreiben`] seit dem 260811 die
//! Gegenrichtung, [`dateiverweise`] seit der Runde 13 den Abwurf; eine zweite
//! Huelle um `NSPasteboard` entsteht dabei nicht.
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
//! tragen seit dem 260805 die Textbefehle des Menues „Bearbeiten" und sonst
//! nichts, und genau das haelt sie fuer eine Dateizwischenablage einer
//! spaeteren Runde frei; der Kopf von `resources/default-keymap.toml` schreibt
//! den Wechsel aus, und die Reservierung aus C3 der Runde 1 ist damit
//! eingeloest und nicht gebrochen.
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
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSPasteboard`, `NSWorkspace`, `NSString`, `NSURL`, `NSNumber`, `NSArray`,
//! `NSDictionary` und `NSData` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `setString:forType:`, `openURL:`, `path` und `pasteboardWithName:`
//! (`NSPasteboard.h:160`). Seit 10.6 stehen `clearContents`, `writeObjects:`,
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
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypePNG, NSPasteboardTypeString,
    NSPasteboardTypeTIFF, NSPasteboardURLReadingFileURLsOnlyKey, NSWorkspace,
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
    use objc2::runtime::ProtocolObject;
    use objc2_app_kit::NSPasteboardWriting;

    use super::*;
    use crate::pruefordner::Pruefordner;

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
    /// bei zwei, und `clearContents` macht den Anfangszustand jedes Laufs
    /// gleich.
    fn probenablage(zweck: &str) -> objc2::rc::Retained<NSPasteboard> {
        let ablage = NSPasteboard::pasteboardWithName(&NSString::from_str(&format!(
            "com.krk.probe.{zweck}"
        )));
        let _ = ablage.clearContents();
        ablage
    }

    /// Legt die Pfade als Datei-`NSURL` in die Ablage.
    fn dateien_ablegen(ablage: &NSPasteboard, pfade: &[&std::path::Path]) {
        let urls: Vec<_> = pfade
            .iter()
            .map(|pfad| NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy())))
            .collect();
        let schreiber: Vec<&ProtocolObject<dyn NSPasteboardWriting>> = urls
            .iter()
            .map(|url| ProtocolObject::from_ref(&**url))
            .collect();
        assert!(
            ablage.writeObjects(&NSArray::from_slice(&schreiber)),
            "die Probenablage nimmt die Dateiverweise an"
        );
    }

    #[test]
    fn zwei_dateiverweise_kommen_als_zwei_pfade_zurueck() {
        let ordner = Pruefordner::neu("dateiverweise");
        let erste = ordner.datei("erste.txt", b"eins");
        let zweite = ordner.datei("zweite.txt", b"zwei");

        let ablage = probenablage("dateiverweise");
        dateien_ablegen(&ablage, &[&erste, &zweite]);

        assert_eq!(
            dateiverweise(&ablage),
            vec![erste, zweite],
            "C4: jeder gezogene Eintrag kommt mit seinem Pfad zurueck, in der Reihenfolge der Ablage"
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
}
