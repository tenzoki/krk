//! Die eine Huelle um den Papierkorb des Systems: **Raeumen und Vorpruefung**.
//!
//! Zwei Stuecke, ein Gegenstand. [`Systempapierkorb`] raeumt einen Eintrag in
//! den Papierkorb und ist die Implementierung der Schnittstelle
//! [`krk_core::operation::Papierkorb`]; [`fuehrt_einen_papierkorb`] fragt
//! vorher, ob das Ziel ueberhaupt einen fuehrt (C4). Beide liegen hier, weil
//! beide AppKit rufen, und sie liegen **zusammen** und nicht in zwei Modulen,
//! weil sie dieselbe Stelle fragen: `NSFileManager.defaultManager()`. Genau
//! darauf beruht die Entscheidbarkeit der Vorpruefung — sie fragt dieselbe
//! Instanz, die `trashItemAtURL:` gleich beantworten wird, nach demselben
//! Datentraeger.
//!
//! Die Schnittstelle des ersten Stuecks steht im Kern und kennt AppKit nicht;
//! der Aufrufer des zweiten steht in dieser Kiste:
//!
//! ```text
//!   krk-core::operation::loeschen        hier
//!   ─────────────────────────────        ────
//!   trait Papierkorb            <──────  impl Papierkorb for Systempapierkorb
//!        ^                                       │
//!        └── die Operationsmaschine ruft         └─> NSFileManager
//!
//!   krk-ui::appkit::anwendung             hier
//!   ─────────────────────────             ────
//!   loeschen_nach_rueckfrage    ──────>  fuehrt_einen_papierkorb
//!                                                │
//!                                                └─> NSFileManager
//! ```
//!
//! Das obere Bild ist die eine Abhaengigkeitsumkehr des Entwurfs: der **Aufruf**
//! laeuft von unten nach oben, die **Uebersetzungsabhaengigkeit** weiterhin von
//! oben nach unten. `krk-core` nennt keine `objc2`-Kiste. Das untere Bild ist
//! der gewoehnliche Weg und braucht keine Umkehr, weil sein Aufrufer schon in
//! dieser Kiste sitzt.
//!
//! Was ueber die Grenze geht, sind gewoehnliche Rust-Werte: ein [`Path`] hinein,
//! ein [`PathBuf`], ein [`io::Error`] oder ein [`Befund`] heraus. Kein `NSURL`,
//! kein `NSError`.
//!
//! # Warum die Vorpruefung keine Methode der Schnittstelle ist
//!
//! Weil sie zu einem Zeitpunkt gefragt wird, an dem es noch keinen Auftrag gibt.
//! Ihr Aufrufer ist der Kommandoweg in [`crate::appkit::anwendung`], und zwar
//! **vor** der Rueckfrage; die Operationsmaschine im Kern erreicht diesen Punkt
//! nie, denn wenn sie laeuft, ist die Entscheidung zu loeschen gefallen. Eine
//! Methode an `trait Papierkorb` haette die Frage also an die falsche Stelle
//! gelegt und dort einen Aufrufer gebraucht, den es nicht gibt.
//!
//! # Auf welcher Polaritaet der Befund liegt
//!
//! Auf der **zweiten**: bei der Frage nach dem Papierkorb ist [`Befund::Ja`] die
//! **Erlaubnis** und nicht der Warngrund, und [`Befund::Unentschieden`] gehoert
//! deshalb zu [`Befund::Nein`]. [`Befund::ist_warnwuerdig`] ist hier folglich das
//! falsche Werkzeug — es fasst `Ja` und `Unentschieden` zusammen, und wer es an
//! diesen Rueckgabewert haelt, macht aus „wir wissen nichts" die Erlaubnis zu
//! loeschen. Der Aufrufer prueft auf [`Befund::Ja`] selbst. Die beiden
//! Polaritaeten und der Grund fuer die Unterscheidung stehen im Modulkopf von
//! [`krk_core::verzeichnis::Befund`] auseinandergehalten; hier steht nur, welche
//! von beiden gilt, damit der naechste Leser sie nicht neu findet.
//!
//! # Warum die Vorpruefung nicht anlegen laesst
//!
//! `create:` steht auf `false`. Die Frage ist, ob das Ziel einen Papierkorb
//! **fuehrt**, und eine Pruefung, die ihn im Zweifel anlegt, beantwortet eine
//! andere: sie veraendert das Ziel, ueber das sie gleich urteilt, und meldete
//! danach ein `Ja` ueber einen Datentraeger, der einen Augenblick vorher keinen
//! Papierkorb hatte. Der Nutzer bekaeme dann eine Rueckfrage statt der Meldung,
//! dass hier nicht geloescht wird, und sein Eintrag laege in einem Papierkorb,
//! den KRK ihm ungefragt eingerichtet hat.
//!
//! # Warum der Papierkorb keinen eigenen Faden braucht
//!
//! `NSFileManager` ist von jedem Faden aus zu rufen, und die
//! Operationsmaschine ruft ihn von ihrem Arbeitsfaden. Der Hauptfaden bleibt
//! damit auch beim Loeschen frei, was L9 verlangt. Eine Ruecknahme fuehrt KRK
//! nicht selbst: der Rueckweg ist der Papierkorb des Systems (C4).
//!
//! **Fuer die Vorpruefung gilt dieser Satz nicht, und das ist Absicht.** Sie
//! laeuft auf dem Hauptfaden, weil ihr Ausgang entscheidet, ob die Rueckfrage
//! ueberhaupt erscheint; ein Nebenfaden brauchte einen Rueckweg und machte aus
//! einer Entscheidung vor dem Blatt eine nach dem Blatt. Es ist ein Aufruf, und
//! er faellt in die Spanne zwischen Tastendruck und Blatt, in der der Nutzer
//! ohnehin auf das Blatt wartet. Keine der zehn Zusagen aus C8 vermisst diese
//! Spanne — L9 misst den Tastendruck **waehrend einer laufenden Kopie**. Der
//! Rest ist benannt und nicht weggerechnet: haengt der Datentraeger unter dem
//! angezeigten Ordner, verzoegert sich das Blatt um die Antwort des Systems.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSFileManager`, `NSString`, `NSURL` und `NSError` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `defaultManager`, `fileURLWithPath:`, `NSURL.path` und
//! `localizedDescription`. Drei Beruehrungen sind juenger als ihre Klasse:
//! `trashItemAtURL:resultingItemURL:error:` steht seit 10.8 (`NSFileManager.h`),
//! `URLForDirectory:inDomain:appropriateForURL:create:error:` seit 10.6
//! (`NSFileManager.h:127`) und `NSTrashDirectory` seit 10.8
//! (`NSPathUtilities.h:88`). Drei tragen im Kopf keine Angabe und stehen damit
//! seit 10.0: die Aufzaehlungen `NSSearchPathDirectory` (`NSPathUtilities.h:61`)
//! und `NSSearchPathDomainMask` (`NSPathUtilities.h:92`) sowie der Wert
//! `NSUserDomainMask` (`NSPathUtilities.h:93`) — er ist der einzige der fuenf
//! Werte seiner Aufzaehlung ohne `API_AVAILABLE`, und das ist keine Auslassung,
//! sondern die Angabe „von Anfang an". Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::io;
use std::path::{Path, PathBuf};

use objc2_foundation::{
    NSFileManager, NSSearchPathDirectory, NSSearchPathDomainMask, NSString, NSURL,
};

use krk_core::operation::Papierkorb;
use krk_core::verzeichnis::Befund;

/// Der Papierkorb des Systems.
///
/// Eingehaengt wird er seit S16 in
/// [`crate::appkit::anwendung`], wo jeder Auftrag an die Operationsmaschine
/// entsteht. Bis dahin hatte die Schnittstelle im Kern im laufenden Programm
/// keine Implementierung.
#[derive(Debug, Clone, Copy, Default)]
pub struct Systempapierkorb;

impl Papierkorb for Systempapierkorb {
    fn in_den_papierkorb(&self, pfad: &Path) -> io::Result<PathBuf> {
        let Some(text) = pfad.to_str() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} ist kein gueltiger UTF-8-Pfad", pfad.display()),
            ));
        };
        let url = NSURL::fileURLWithPath(&NSString::from_str(text));

        let mut neuer_ort: Option<objc2::rc::Retained<NSURL>> = None;
        NSFileManager::defaultManager()
            .trashItemAtURL_resultingItemURL_error(&url, Some(&mut neuer_ort))
            .map_err(|fehler| io::Error::other(fehler.localizedDescription().to_string()))?;

        // Das System nennt den neuen Ort; nennt es keinen, ist der Eintrag
        // trotzdem im Papierkorb. Dann bleibt der alte Pfad die einzige
        // Auskunft, die wir haben, und die ist besser als ein Fehler ueber eine
        // Loeschung, die geklappt hat.
        Ok(neuer_ort.and_then(|ort| ort.path()).map_or_else(
            || pfad.to_path_buf(),
            |pfad| PathBuf::from(pfad.to_string()),
        ))
    }
}

/// Ob der Datentraeger unter diesem Ordner einen Papierkorb fuehrt (C4).
///
/// Gefragt wird `NSFileManager.defaultManager()` nach dem Papierkorb des
/// Benutzers, der fuer diesen Ordner zustaendig ist. Der Aufruf scheitert genau
/// dann, wenn es dort keinen gibt — er sagt keinen kuenftigen Systemaufruf
/// voraus, sondern laesst die Stelle antworten, die `trashItemAtURL:` gleich
/// beantworten wird. Warum `create:` dabei auf `false` steht, sagt der
/// Modulkopf.
///
/// Drei Ausgaenge:
///
/// - [`Befund::Ja`] — das System nennt einen Papierkorb. **Das ist die
///   Erlaubnis**, nicht ein Warngrund; die Polaritaet steht im Modulkopf.
/// - [`Befund::Nein`] — das System nennt einen Fehler. Es gibt dort keinen
///   Papierkorb, und es wird nicht geloescht.
/// - [`Befund::Unentschieden`] — der Pfad ist kein gueltiges UTF-8 und laesst
///   sich nicht in ein `NSString` uebersetzen. Das ist keine Aussage ueber das
///   Ziel, sondern eine ueber KRKs Kenntnis von ihm, und der Aufrufer loescht
///   auch dann nicht.
///
/// Der Ordner kommt **aufgeloest** herein. Diese Funktion ruft weder
/// `canonicalize` noch sonst etwas am Dateisystem: eine Verknuepfung wuerde
/// sonst den Papierkorb ihres eigenen Ortes melden statt den ihres Ziels. Wer
/// sie aufloest, ist ihr Aufrufer, und ein Pfad, der sich nicht aufloesen laesst,
/// zaehlt dort ebenfalls als [`Befund::Unentschieden`].
///
/// **Ein Rest bleibt und ist benannt:** ein einzelner Eintrag kann trotz
/// bestandener Pruefung scheitern, etwa weil unter dem angezeigten Ordner ein
/// Einhaengepunkt eines anderen Datentraegers liegt. Dieser Rest wird
/// nachtraeglich am Ergebnis des einzelnen Eintrags entschieden, und sein
/// Ausgang ist „uebersprungen mit Grund" und nie „endgueltig geloescht".
#[must_use = "der Befund ist die Erlaubnis zu loeschen; fallengelassen loescht der Aufrufer auf einem Ziel, das keinen Papierkorb fuehrt"]
pub fn fuehrt_einen_papierkorb(ordner: &Path) -> Befund {
    let Some(text) = ordner.to_str() else {
        return Befund::Unentschieden;
    };
    let url = NSURL::fileURLWithPath(&NSString::from_str(text));

    match NSFileManager::defaultManager().URLForDirectory_inDomain_appropriateForURL_create_error(
        NSSearchPathDirectory::TrashDirectory,
        NSSearchPathDomainMask::UserDomainMask,
        Some(&url),
        false,
    ) {
        Ok(_) => Befund::Ja,
        Err(_) => Befund::Nein,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Das Benutzerverzeichnis fuehrt einen Papierkorb.
    ///
    /// Der positive Ausgang an einem echten Ziel, und die Probe braucht kein
    /// Fenster und keinen Hauptfaden: `NSFileManager` ist von jedem Faden aus zu
    /// rufen, und die Aussage darueber steht im Modulkopf.
    #[test]
    fn das_benutzerverzeichnis_fuehrt_einen_papierkorb() {
        let Some(zuhause) = krk_core::ablage::pfade::benutzerverzeichnis() else {
            panic!(
                "das System nennt kein Benutzerverzeichnis, und ohne eines misst diese Probe nichts"
            );
        };
        assert_eq!(
            fuehrt_einen_papierkorb(&zuhause),
            Befund::Ja,
            "das Benutzerverzeichnis {} fuehrt keinen Papierkorb",
            zuhause.display()
        );
    }

    /// `/dev` fuehrt keinen Papierkorb.
    ///
    /// **Der negative Ausgang, ohne dass eine Probe einen Datentraeger
    /// einhaengen muesste.** `/dev` ist auf jedem macOS ein eigener
    /// Einhaengepunkt mit einem Dateisystem fuer Geraetedateien, und es kann
    /// keinen Papierkorb fuehren; kein Recht und kein Aufbau ist dafuer noetig.
    /// Ohne diese Probe waere die Funktion mit einem festen [`Befund::Ja`]
    /// gruen, und die Zusage von C4 haette keinen Beleg.
    ///
    /// Gewaehlt ist ein Ort, der die Antwort aus seinem Wesen bezieht, und nicht
    /// ein fehlender Pfad: ein fehlender Pfad liefert dieselbe Antwort, sagt
    /// damit aber nichts ueber den Papierkorb eines Datentraegers, und er kommt
    /// hier ohnehin nicht an — den loest der Aufrufer vorher auf und zaehlt sein
    /// Scheitern als [`Befund::Unentschieden`].
    #[test]
    fn ein_datentraeger_ohne_papierkorb_wird_erkannt() {
        assert_eq!(
            fuehrt_einen_papierkorb(Path::new("/dev")),
            Befund::Nein,
            "/dev fuehrt angeblich einen Papierkorb, also unterscheidet die Pruefung nicht"
        );
    }

    /// Ein Pfad ohne gueltiges UTF-8 bleibt unentschieden und wird nicht zum
    /// `Nein`.
    ///
    /// Der Unterschied traegt: `Nein` waere eine Aussage ueber den Datentraeger,
    /// und die Funktion hat ihn nie gefragt. Beide Ausgaenge halten den Aufrufer
    /// vom Loeschen ab, aber nur einer von beiden nennt in der Statuszeile den
    /// richtigen Grund.
    ///
    /// Das Byte `0xff` ist in keiner UTF-8-Folge zulaessig; der Ordner wird nicht
    /// angelegt, denn die Funktion greift nicht auf das Dateisystem zu.
    #[test]
    fn ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let krumm = PathBuf::from(OsStr::from_bytes(b"/tmp/krk-papierkorb-\xffkrumm"));
        assert!(
            krumm.to_str().is_none(),
            "der Pfad der Probe ist gueltiges UTF-8 und misst damit nicht, was sie messen soll"
        );
        assert_eq!(
            fuehrt_einen_papierkorb(&krumm),
            Befund::Unentschieden,
            "ein Pfad ohne gueltiges UTF-8 liefert nicht den unentschiedenen Befund"
        );
    }
}
