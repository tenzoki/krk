//! Wie ein eingehaengter Datentraeger ausgeworfen wird (261016).
//!
//! ```text
//!  Einhaengepunkt ──> NSURL ──> unmountVolumeAtURL:options:completionHandler:
//!                                         │  (irgendein Faden des Systems)
//!                                         v
//!                         Ergebnis ──> DispatchQueue::main() ──> fertig(mtm, ergebnis)
//! ```
//!
//! Die eine Frage dieses Moduls: **wie kommt ein Auswurf beim System an, ohne
//! den Hauptfaden aufzuhalten, und wie kommt die Antwort zurueck.** Der
//! Zuschnitt ist der von [`super::finder`] und [`super::volumes`]: ein Modul je
//! Frage, eine sichere Huelle je Aufruf, und was die Huelle verlaesst, ist ein
//! gewoehnlicher Rust-Wert. **Welcher Eintrag ein Datentraeger ist, steht
//! nicht hier**, sondern in `crate::kommandos::kontextmenue::auswurfbezug`;
//! was danach auf dem Schirm geschieht, beim Anwendungsdelegierten.
//!
//! # Warum `NSFileManager` und kein eigener Faden
//!
//! `unmountVolumeAtURL:options:completionHandler:` ist der Weg, den Apple fuer
//! genau diese Handlung vorsieht, und er ist von sich aus asynchron: der Aufruf
//! kehrt sofort zurueck, das System haengt aus und ruft den Abschlussblock auf
//! einem Faden seiner Wahl. Ein eigener Arbeitsfaden mit `diskutil` oder
//! `unmount(2)` daneben waere eine zweite Maschine fuer dieselbe Frage und
//! kaeme ohne die Rueckfrage der Diskarbitration aus, die andere Programme
//! (den Finder eingeschlossen) vor dem Aushaengen hoeren.
//!
//! **Gewaehlt sind zwei Optionen.**
//! `NSFileManagerUnmountAllPartitionsAndEjectDisk` wirft das ganze Medium aus
//! und haengt alle seine Partitionen aus, wie der Finder es tut: ein DMG wird
//! dabei auch abgemeldet, ein USB-Laufwerk mit zwei Partitionen verschwindet
//! ganz. `NSFileManagerUnmountWithoutUI` laesst das System keinen eigenen Dialog
//! zeigen, denn die Antwort gehoert in KRKs Statuszeile; zwei Meldungen fuer
//! einen Fehlschlag waeren eine zu viel.
//!
//! # Was der Abschlussblock tut und was nicht
//!
//! Er laeuft auf einem fremden Faden und darf deshalb nichts beruehren, was dem
//! Hauptfaden gehoert. Er tut zweierlei: er liest den Wortlaut des Fehlers
//! (`NSError` ist `Send` und `Sync`, `localizedDescription` von jedem Faden aus
//! lesbar) und reicht das Ergebnis als `Result<(), String>` ueber
//! `DispatchQueue::main()` an den Hauptfaden weiter — derselbe Weg, den der
//! Weckruf des Vorgangs in `super::anwendung` nimmt. Dort erst laeuft `fertig`,
//! mit einem [`MainThreadMarker`], und erst dort darf der Rufer die
//! Oberflaeche anfassen.
//!
//! **`fertig` laeuft genau einmal.** Der Block hat die Bauform `Fn`, weil
//! `block2` es so verlangt; das System ruft ihn einmal, und die Huelle haelt
//! `fertig` in einem `Mutex<Option<_>>`, aus dem der erste Aufruf es nimmt. Ein
//! zweiter faende nichts mehr und taete nichts.
//!
//! # Diese Huelle traegt keine Probe, und das ist Absicht
//!
//! Eine Probe, die sie ausloeste, braeuchte einen eingehaengten Datentraeger,
//! den sie auswerfen darf, und haengte bei jedem `make check` ein Medium des
//! Nutzers aus. Geprueft wird stattdessen, was ohne System pruefbar ist: die
//! Regel `auswurfbezug` und die Saetze des Aufrufers.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSFileManager`, `NSURL`, `NSString` und `NSError` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `defaultManager` und `localizedDescription`.
//! `fileURLWithPath:isDirectory:` (`NSURL.h`) steht seit 10.5.
//! **`unmountVolumeAtURL:options:completionHandler:` ist die juengste
//! Beruehrung dieser Datei und steht seit macOS 10.11** (`NSFileManager.h`,
//! `API_AVAILABLE(macos(10.11))` an der Methode), ebenso die Optionsmenge
//! `NSFileManagerUnmountOptions` mit `NSFileManagerUnmountAllPartitionsAndEjectDisk`
//! und `NSFileManagerUnmountWithoutUI`. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist ein Rust-Typ der
//! Kiste und hat kein macOS-Alter; `RcBlock` aus `block2` und `DispatchQueue`
//! aus `dispatch2` sind Rust-Huellen um Bloecke und `libdispatch`, beide seit
//! 10.6 im System.

use std::path::Path;
use std::sync::Mutex;

use block2::RcBlock;
use dispatch2::DispatchQueue;
use objc2_foundation::{
    MainThreadMarker, NSError, NSFileManager, NSFileManagerUnmountOptions, NSString, NSURL,
};

/// Was das System auf einen Auswurf geantwortet hat: nichts, oder den
/// Wortlaut seines Fehlers.
pub type Auswurfergebnis = Result<(), String>;

/// Wirft den Datentraeger am genannten Einhaengepunkt aus und ruft danach
/// `fertig` auf dem Hauptfaden (261016).
///
/// **Kehrt sofort zurueck.** Auf dem Hauptfaden steht damit allein der Aufruf
/// an `NSFileManager`; das Aushaengen und die Rueckfrage an andere Programme
/// geschehen im System. `fileURLWithPath:isDirectory:` fragt das Dateisystem
/// nicht, anders als die Form ohne `isDirectory:`, die nachsieht, ob der Pfad
/// ein Ordner ist — ein Einhaengepunkt ist immer einer.
///
/// Ein Pfad ohne gueltiges UTF-8 kann kein `NSString` werden; dann antwortet
/// `fertig` sofort mit einem Fehler und das System wird nicht gefragt. Der
/// Wortlaut ist in diesem Fall der Pfad selbst, weil es keinen Fehler des
/// Systems gibt, den man nennen koennte.
pub fn auswerfen(
    einhaengepunkt: &Path,
    fertig: impl FnOnce(MainThreadMarker, Auswurfergebnis) + Send + 'static,
) {
    let Some(text) = einhaengepunkt.to_str() else {
        let pfad = einhaengepunkt.display().to_string();
        auf_dem_hauptfaden(fertig, Err(pfad));
        return;
    };
    let url = NSURL::fileURLWithPath_isDirectory(&NSString::from_str(text), true);
    let halter = Mutex::new(Some(fertig));
    let block = RcBlock::new(move |fehler: *mut NSError| {
        // SAFETY: Das System reicht entweder `nil` oder einen gueltigen
        // `NSError`, der fuer die Dauer des Blocks lebt (die Signatur im
        // SDK-Kopf ist `void (^)(NSError * _Nullable errorOrNil)`).
        let ergebnis = match unsafe { fehler.as_ref() } {
            None => Ok(()),
            Some(fehler) => Err(fehler.localizedDescription().to_string()),
        };
        let genommen = match halter.lock() {
            Ok(mut fertig) => fertig.take(),
            // Ein vergifteter Halter heisst, dass ein frueherer Aufruf mitten
            // im Nehmen abgebrochen ist; der Wert darin ist trotzdem der
            // richtige, und ihn liegen zu lassen hiesse, die Antwort zu
            // verschlucken.
            Err(vergiftet) => vergiftet.into_inner().take(),
        };
        if let Some(fertig) = genommen {
            auf_dem_hauptfaden(fertig, ergebnis);
        }
    });
    let optionen = NSFileManagerUnmountOptions::AllPartitionsAndEjectDisk
        | NSFileManagerUnmountOptions::WithoutUI;
    // SAFETY: Die Bindung verlangt einen Block, der von einem fremden Faden
    // aus gerufen werden darf. Er haelt allein einen `Mutex` um `fertig`, und
    // `fertig` ist `Send`; das Ergebnis, das er baut, ist ein `String`. Nichts
    // darin gehoert dem Hauptfaden.
    unsafe {
        NSFileManager::defaultManager()
            .unmountVolumeAtURL_options_completionHandler(&url, optionen, &block);
    }
}

/// Ruft `fertig` mit dem Ergebnis auf dem Hauptfaden.
///
/// Der Weg ueber die Hauptschlange und nicht ein unmittelbarer Aufruf, auch
/// wenn der Rufer schon auf dem Hauptfaden steht: so kommt die Antwort auf
/// beiden Wegen in derselben Lage an, nach dem Ende des Rumpfes, der den
/// Auswurf angestossen hat.
fn auf_dem_hauptfaden(
    fertig: impl FnOnce(MainThreadMarker, Auswurfergebnis) + Send + 'static,
    ergebnis: Auswurfergebnis,
) {
    DispatchQueue::main().exec_async(move || {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Ein Abbruch waere hier trotzdem falsch, weil er die
            // Anwendung um eine Statusmeldung braechte und nicht um mehr.
            return;
        };
        fertig(mtm, ergebnis);
    });
}
