//! Der Ordnerdialog von „Ort waehlen…“ (H3 des Spec
//! `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`).
//!
//! ```text
//! Kommando::OrtWaehlen ──> Anwendungsdelegierter::ort_waehlen
//!                               │
//!                               ▼
//!                zeigen(fenster, beginn, fertig) ── NSOpenPanel als Blatt
//!                               │
//!              fertig(Some(pfad) | None) auf dem Hauptfaden
//!                               │
//!                               ▼
//!              Anwendungsdelegierter::ort_uebernehmen
//! ```
//!
//! Dieses Modul zeigt den Dialog und liefert den gewaehlten Ordner, und sonst
//! nichts. Was mit dem Ordner geschieht — pruefen, schreiben, den Griff
//! ersetzen, die Tabs nachziehen —, entscheidet der Anwendungsdelegierte ueber
//! die Kernfunktionen aus `krk_core::heimordner::ort`.
//!
//! # Ein Blatt und kein eigenes modales Fenster
//!
//! **`runModal` faehrt eine eigene Ereignisschleife im modalen Modus**, und in
//! ihr stuenden die Zeitgeber der Sitzungssicherung, des Einzugs der
//! Dateilisten und der Vorschau still. Das Blatt kehrt sofort zurueck, und die
//! Laufschleife bleibt die der Anwendung. Als Blatt greift ausserdem die
//! bestehende Blattsperre: solange der Dialog steht, kommen genau die Befehle
//! durch, die die Probe `waehrend_eines_blattes_kommen_genau_diese_vier_durch`
//! zaehlt, und „Ort waehlen…“ gehoert nicht dazu.
//!
//! # Kein Blattgriff
//!
//! **Der Dialog legt keinen [`super::Blattgriff`] in den Schlitz**, anders als
//! jedes Blatt dieses Verzeichnisses, das auf `NSAlert` aufbaut. Der Griff ist
//! der Weg, auf dem `esc` ein Blatt von aussen schliesst; der Ordnerdialog
//! schliesst sich mit `esc` selbst, weil AppKit die Taste dem anhaengenden
//! Blatt gibt. Dass sie dort ankommt, auch wenn ein alter Griff eines
//! Eingabeblatts im Schlitz liegt, haelt die Regel aus Schritt 3.2 des Plans:
//! der erste Rang von `Anwendungsdelegierter::abbrechen` nimmt einen Griff nur,
//! wenn [`super::Blattgriff::steht`] ja sagt, und gibt die Taste sonst an
//! AppKit zurueck. Der Abschlussblock ruft trotzdem, wie jedes Blatt,
//! `blatt_geschlossen` beim Delegierten: dort wird nachgeholt, was waehrend
//! des Blattes liegengeblieben ist, etwa die Konfliktfrage eines laufenden
//! Vorgangs.
//!
//! # Was hier nicht nachgelesen ist
//!
//! `setResolvesAliases(true)` ist der Vorgabewert von AppKit und steht hier
//! ausdruecklich, damit ein Finder-Alias auf einen Ordner einen Ordner ergibt
//! und kein Alias-Dokument. **Ob AppKit einen gewaehlten symbolischen Verweis
//! dabei ebenfalls aufloest**, ist nicht nachgelesen und steht offen; zeigt
//! es sich, steht in `settings.toml` das Ziel statt des Wegs durch den
//! Verweis. Die Nutzerpruefung von Stufe 3 mit `~/Dropbox` klaert es.
//!
//! Die Bindung warnt, dass der Abschlussblock laufen kann, **waehrend das
//! Blatt noch am Fenster haengt**. `blatt_geschlossen` weckt den Hauptfaden
//! deshalb ueber die Hauptschlange und holt erst einen Durchgang spaeter nach.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSOpenPanel` steht seit macOS 10.0 zur Verfuegung, ebenso `openPanel`,
//! `setCanChooseDirectories:`, `setCanChooseFiles:`,
//! `setAllowsMultipleSelection:`, `setResolvesAliases:` und `URLs`
//! (`NSOpenPanel.h`), dazu aus `NSSavePanel` `setCanCreateDirectories:`,
//! `setPrompt:` und `setMessage:`. `setDirectoryURL:` steht seit 10.6, und
//! `beginSheetModalForWindow:completionHandler:` (`NSSavePanel.h`) ebenfalls
//! seit 10.6; der Abschlussblock aus `block2` ist dessen Argument und hat
//! keine eigene Untergrenze. `NSWindow` steht seit 10.0, `NSURL` mit
//! `fileURLWithPath:` und `path` ebenso, `NSString` auch; `NSModalResponse`
//! ist ein `typedef` auf `NSInteger` ohne Verfuegbarkeitsangabe, und
//! `NSModalResponseOK` steht seit 10.9 und ist eine
//! Uebersetzungszeitkonstante ohne eigenes Laufzeitsymbol. `MainThreadMarker`
//! ist ein Rust-Typ der Kiste und hat kein macOS-Alter.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine dieser Beruehrungen
//! ist nach macOS 15 hinzugekommen, und keine braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::path::{Path, PathBuf};

use block2::RcBlock;
use objc2_app_kit::{NSModalResponse, NSModalResponseOK, NSOpenPanel, NSWindow};
use objc2_foundation::{MainThreadMarker, NSString, NSURL};

/// Die Beschriftung der bestaetigenden Schaltflaeche.
const SCHALTFLAECHE: &str = "Wählen";

/// Die Zeile ueber der Ordnerliste.
const FRAGE: &str = "Wo soll der Notizordner liegen?";

/// Zeigt den Ordnerdialog als Blatt am Fenster.
///
/// Kehrt sofort zurueck. `fertig` laeuft genau einmal auf dem Hauptfaden: mit
/// dem gewaehlten Ordner, wenn der Nutzer bestaetigt hat, sonst mit `None`.
/// Anders als bei der Pfadeingabe laeuft es auch beim Abbruch, weil der
/// Delegierte in jedem Fall `blatt_geschlossen` rufen muss.
///
/// `beginn` ist der geltende Ort. **Er wird vorher nicht geprueft**: H3
/// verlangt, dass der Dialog dort beginnt, und fehlt der Ordner, beginnt
/// AppKit an einem Ort seiner Wahl.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    beginn: Option<&Path>,
    fertig: impl Fn(Option<PathBuf>) + 'static,
) {
    let dialog = NSOpenPanel::openPanel(mtm);
    dialog.setCanChooseDirectories(true);
    dialog.setCanChooseFiles(false);
    dialog.setAllowsMultipleSelection(false);
    dialog.setCanCreateDirectories(true);
    dialog.setResolvesAliases(true);
    if let Some(beginn) = beginn.and_then(Path::to_str) {
        let adresse = NSURL::fileURLWithPath(&NSString::from_str(beginn));
        dialog.setDirectoryURL(Some(&adresse));
    }
    dialog.setPrompt(Some(&NSString::from_str(SCHALTFLAECHE)));
    dialog.setMessage(Some(&NSString::from_str(FRAGE)));

    // Der Block haelt den Dialog, damit `URLs` nach der Antwort noch zu
    // fragen ist. Der Ring Dialog → Block → Dialog bricht, sobald AppKit den
    // Rueckruf nach der Antwort freigibt; dieselbe Bauart wie der Block in
    // `Blatt::zeigen_mit_wahl`.
    let gehalten = dialog.clone();
    let block = RcBlock::new(move |antwort: NSModalResponse| {
        let gewaehlt = if antwort == NSModalResponseOK {
            erster_pfad(&gehalten)
        } else {
            None
        };
        fertig(gewaehlt);
    });
    dialog.beginSheetModalForWindow_completionHandler(fenster, &block);
}

/// Der erste gewaehlte Ordner als Pfad; `None`, wenn der Dialog keinen oder
/// keinen Dateipfad liefert.
fn erster_pfad(dialog: &NSOpenPanel) -> Option<PathBuf> {
    dialog
        .URLs()
        .iter()
        .find_map(|adresse| adresse.path())
        .map(|pfad| PathBuf::from(pfad.to_string()))
}
