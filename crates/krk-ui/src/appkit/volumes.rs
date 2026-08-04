//! Die Beobachtung der eingehaengten Datentraeger ueber `NSWorkspace` (C9).
//!
//! Ordnerinhalte und Datentraeger sind zwei Mechanismen und bekommen zwei
//! Module. [`super::fsevents`] beobachtet, was sich **in** einem Ordner
//! aendert; hier steht, wann ein Datentraeger kommt und geht. Die beiden
//! ueberschneiden sich nicht: FSEvents meldet keinen Auswurf, und `NSWorkspace`
//! meldet keine angelegte Datei.
//!
//! Beobachtet werden die drei Meldungen, die `### Frage 3` des Plans nennt:
//!
//! ```text
//! didMount      ──> Wechsel::Eingehaengt      ein Datentraeger ist da   (C5)
//! willUnmount   ──> Wechsel::WirdAusgeworfen  er geht gleich            (C9)
//! didUnmount    ──> Wechsel::Ausgeworfen      er ist weg                (C9)
//! ```
//!
//! **`willUnmount` und `didUnmount` sind beide noetig und keine Verdopplung.**
//! Der geordnete Auswurf ueber den Finder meldet zuerst `willUnmount`, und
//! genau dann muss KRK den Ordner verlassen: ein Dateifenster, das noch auf
//! dem Datentraeger steht, haelt ihn offen und laesst den Auswurf scheitern.
//! Ein abgezogenes Medium meldet allein `didUnmount`, weil niemand vorher
//! gefragt hat. Wer nur eine der beiden nimmt, verfehlt einen der beiden
//! Faelle.
//!
//! Was danach geschieht, entscheidet [`crate::auffrischung`] und nicht dieses
//! Modul: hierher gehoert die Beruehrung mit AppKit, dorthin die Frage, welches
//! Dateifenster betroffen ist.
//!
//! Mit S18 kommt die Aufzaehlung ueber `NSFileManager.mountedVolumeURLs…`
//! daneben, damit ein Modul die ganze Frage "welche Datentraeger gibt es
//! gerade" beantwortet.

use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSWorkspace, NSWorkspaceDidMountNotification, NSWorkspaceDidUnmountNotification,
    NSWorkspaceVolumeLocalizedNameKey, NSWorkspaceVolumeURLKey, NSWorkspaceWillUnmountNotification,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSString, NSURL,
};

/// Was mit einem Datentraeger geschehen ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wechsel {
    /// Er ist eingehaengt worden.
    Eingehaengt,
    /// Er wird gleich ausgeworfen.
    WirdAusgeworfen,
    /// Er ist ausgehaengt worden.
    Ausgeworfen,
}

/// Ein gemeldeter Datentraeger.
#[derive(Debug, Clone)]
pub struct Datentraeger {
    /// Was mit ihm geschehen ist.
    pub art: Wechsel,
    /// Sein Einhaengepunkt, gewoehnlich unterhalb von `/Volumes`.
    pub pfad: PathBuf,
    /// Sein Name in der Schreibweise, die der Nutzer im Finder sieht.
    ///
    /// Faellt auf den letzten Namensteil des Einhaengepunkts zurueck, wenn die
    /// Meldung keinen mitbringt. Eine Meldung ohne Namen waere fuer den Nutzer
    /// nicht zuzuordnen.
    pub name: String,
}

/// Was das Rueckrufziel der Datentraegerbeobachtung haelt.
pub struct WacheIvars {
    /// Die Senke, an die jede der drei Meldungen geht.
    senke: Box<dyn Fn(Datentraeger)>,
}

define_class!(
    /// Das Ziel, an das die drei Meldungen von `NSWorkspace` gehen.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = WacheIvars]
    struct Datentraegerziel;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Datentraegerziel {}

    impl Datentraegerziel {
        /// `NSWorkspaceDidMountNotification`.
        // SAFETY: Die Signatur ist die einer Meldungsannahme: ein Argument,
        // die Meldung.
        #[unsafe(method(datentraegerEingehaengt:))]
        fn eingehaengt(&self, meldung: &NSNotification) {
            self.weitergeben(Wechsel::Eingehaengt, meldung);
        }

        /// `NSWorkspaceWillUnmountNotification`.
        // SAFETY: Die Signatur ist die einer Meldungsannahme.
        #[unsafe(method(datentraegerWirdAusgeworfen:))]
        fn wird_ausgeworfen(&self, meldung: &NSNotification) {
            self.weitergeben(Wechsel::WirdAusgeworfen, meldung);
        }

        /// `NSWorkspaceDidUnmountNotification`.
        // SAFETY: Die Signatur ist die einer Meldungsannahme.
        #[unsafe(method(datentraegerAusgeworfen:))]
        fn ausgeworfen(&self, meldung: &NSNotification) {
            self.weitergeben(Wechsel::Ausgeworfen, meldung);
        }
    }
);

impl Datentraegerziel {
    /// Ein Ziel, das jede der drei Meldungen an die genannte Senke reicht.
    fn neu(mtm: MainThreadMarker, senke: Box<dyn Fn(Datentraeger)>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(WacheIvars { senke });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Liest Einhaengepunkt und Namen aus der Meldung und gibt sie weiter.
    ///
    /// Eine Meldung ohne Einhaengepunkt wird uebergangen: ohne Pfad laesst sich
    /// kein Dateifenster zuordnen, und ein geratener Pfad waere schlimmer als
    /// keiner.
    fn weitergeben(&self, art: Wechsel, meldung: &NSNotification) {
        let Some(angaben) = meldung.userInfo() else {
            return;
        };
        // SAFETY: Zwei Fremdsymbole von AppKit, die Schluesselnamen der
        // Datentraegermeldungen. Sie werden gelesen und nicht geschrieben.
        let (schluessel_ort, schluessel_name) =
            unsafe { (NSWorkspaceVolumeURLKey, NSWorkspaceVolumeLocalizedNameKey) };
        let Some(ort) = angaben
            .objectForKey(schluessel_ort)
            .and_then(|wert| wert.downcast::<NSURL>().ok())
            .and_then(|url| url.path())
        else {
            return;
        };
        let pfad = PathBuf::from(ort.to_string());
        let name = angaben
            .objectForKey(schluessel_name)
            .and_then(|wert| wert.downcast::<NSString>().ok())
            .map(|text| text.to_string())
            .unwrap_or_else(|| {
                pfad.file_name()
                    .map(|teil| teil.to_string_lossy().into_owned())
                    .unwrap_or_else(|| pfad.display().to_string())
            });
        (self.ivars().senke)(Datentraeger { art, pfad, name });
    }
}

/// Eine laufende Datentraegerbeobachtung.
///
/// Sie beobachtet, solange dieser Wert lebt. Ohne Halter meldete sich das Ziel
/// beim Fallenlassen sofort wieder ab; dieselbe Form wie beim Tastenabgriff
/// aus S7, beim [`Zeichenende`](super::bildtakt::Zeichenende) aus S8 und bei
/// der [`Dateisystemwache`](super::fsevents::Dateisystemwache) nebenan.
pub struct Datentraegerwache {
    ziel: Retained<Datentraegerziel>,
}

impl Datentraegerwache {
    /// Meldet jedes Einhaengen und Auswerfen an `senke`.
    pub fn einrichten(mtm: MainThreadMarker, senke: impl Fn(Datentraeger) + 'static) -> Self {
        let ziel = Datentraegerziel::neu(mtm, Box::new(senke));
        let zentrale = NSWorkspace::sharedWorkspace().notificationCenter();
        for (name, wahl) in [
            (
                unsafe { NSWorkspaceDidMountNotification },
                sel!(datentraegerEingehaengt:),
            ),
            (
                unsafe { NSWorkspaceWillUnmountNotification },
                sel!(datentraegerWirdAusgeworfen:),
            ),
            (
                unsafe { NSWorkspaceDidUnmountNotification },
                sel!(datentraegerAusgeworfen:),
            ),
        ] {
            // SAFETY: `ziel` ist von der Klasse, die die drei Selektoren
            // beantwortet, und jede der drei Methoden hat die Signatur einer
            // Meldungsannahme. Der Beobachter wird in `Drop` wieder
            // abgemeldet, also ueberlebt er die Zentrale nicht.
            unsafe { zentrale.addObserver_selector_name_object(&ziel, wahl, Some(name), None) };
        }
        Self { ziel }
    }
}

impl Drop for Datentraegerwache {
    fn drop(&mut self) {
        let zentrale = NSWorkspace::sharedWorkspace().notificationCenter();
        // SAFETY: `self.ziel` ist der Beobachter, den `einrichten` fuer alle
        // drei Meldungen angemeldet hat. Die Form ohne Namen nimmt ihn fuer
        // alle drei zugleich wieder heraus.
        unsafe { zentrale.removeObserver(&self.ziel) };
    }
}
