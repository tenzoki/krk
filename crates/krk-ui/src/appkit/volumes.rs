//! Die eine Stelle, die das System nach **Datentraegern** fragt: Beobachtung,
//! Aufzaehlung und die Einordnung eines einzelnen Ordners (C9 der Runde 1, C3
//! dieser Runde).
//!
//! Drei Fragen, ein Gegenstand, und die dritte ist die juengste:
//!
//! ```text
//!  1. was hat sich geaendert?          Datentraegerwache ──> NSWorkspace
//!  2. welche gibt es gerade?           eingehaengte      ──> NSFileManager
//!  3. liegt der Ordner auf einem
//!     Netzlaufwerk?                    liegt_auf_netzlaufwerk ──> NSURL
//! ```
//!
//! Sie liegen zusammen, weil sie dieselbe Sache befragen, und die dritte kommt
//! ausdruecklich **hierher** und nicht in ein eigenes Modul: die Abfrage von
//! Ressourcenwerten eines Datentraegers ueber `resourceValuesForKeys:error:`
//! steht in [`eingehaengte`] schon, und eine zweite Stelle daneben waere der
//! Doppelbau, den dieses Modul seit S18 vermeidet.
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
//! Seit S18 steht die Aufzaehlung ueber `NSFileManager.mountedVolumeURLs…`
//! daneben, damit ein Modul die ganze Frage "welche Datentraeger gibt es
//! gerade" beantwortet: [`eingehaengte`]. Beobachtung und Aufzaehlung sind
//! zwei Haelften derselben Frage, und sie auf zwei Module zu verteilen hiesse,
//! dass die eine Haelfte sich aendert, ohne dass die andere es merkt.
//!
//! **Was dieses Modul nicht tut, ist die Liste der Leiste bauen.** Sie enthaelt
//! neben den Datentraegern das Benutzerverzeichnis, und das ist kein
//! Datentraeger und kommt aus `krk_core::ablage::pfade`. Zusammengesetzt wird
//! beides in [`crate::leistenmodell`], das auch die Reihenfolge fuehrt.
//!
//! # Die dritte Frage, und warum sie nach dem Netzlaufwerk fragt
//!
//! [`liegt_auf_netzlaufwerk`] beantwortet den dritten der sechs Ausloeser aus
//! C3: „der Datentraeger des Ordners ist kein lokaler". **Die Funktion heisst
//! nach dem Ausloeser und liefert dessen Antwort** und nicht die des
//! Ressourcenwerts, den sie dafuer abfragt:
//!
//! ```text
//!  liegt_auf_netzlaufwerk(ordner)  ──>  Ja            kein lokaler       LAUT
//!                                  ──>  Nein          ein lokaler        ruhig
//!                                  ──>  Unentschieden KRK weiss nichts   LAUT
//! ```
//!
//! Damit liegt die Antwort auf der **ersten** Polaritaet aus dem Modulkopf von
//! [`krk_core::verzeichnis::Loeschzielbefund`]: `Ja` ist der Warngrund, und
//! `Unentschieden` gehoert zu ihm. [`Loeschzielbefund::ist_warnwuerdig`] waere
//! fuer diesen Wert folglich eine **zulaessige** Frage — anders als bei
//! [`super::papierkorb::fuehrt_einen_papierkorb`], das auf der zweiten
//! Polaritaet liegt und bei dem `Ja` die Erlaubnis ist. Zulaessig heisst nicht
//! gebraucht: wer den Befund verbraucht, muss den **Grund** benennen, und dafuer
//! braucht er `Ja` und `Unentschieden` getrennt. Der Absatz zur Zaehlprobe
//! weiter unten sagt, warum die Frage auch in dieser Datei nicht steht.
//!
//! **Der Ressourcenwert antwortet umgekehrt, und die Umkehrung geschieht genau
//! einmal: hier, im Rumpf, neben dieser Erklaerung.** Gefragt wird
//! `NSURLVolumeIsLocalKey`, und ein `true` von dort heisst „lokal", also
//! harmlos; [`liegt_auf_netzlaufwerk`] gibt darauf `Nein` zurueck.
//!
//! Bis zum 260817 hiess die Funktion `ist_lokal` und lieferte die Antwort des
//! Ressourcenwerts. Sie fuellte damit ein Feld — `Loeschziel::netzlaufwerk` aus
//! dem zehnten Schritt dieser Runde —, das die **umgekehrte** Polaritaet
//! traegt, und beide Seiten trugen denselben Typ:
//! `netzlaufwerk: volumes::ist_lokal(&ordner)` uebersetzte, bestand jede Probe
//! und vertauschte lokal und fern. `Unentschieden` ist ein Fixpunkt der
//! Umkehrung, also blieb die Zusage „Unentschieden gilt als laut" dabei
//! sichtbar erfuellt, waehrend der **genannte Grund** in den beiden
//! entschiedenen Faellen falsch war. Der Nutzer hat am 260817-1640 den Weg 1
//! aus
//! `issues/260817-1623_*_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`
//! gewaehlt: Name und Rueckgabewert folgen dem Ausloeser, die Umkehrung steht
//! einmal im Rumpf. Verworfen sind ein `Loeschzielbefund::umgekehrt()`, zwei
//! Typen je Polaritaet und die Umkehrung von Hand im Aufrufer.
//!
//! **Was die Umbenennung nicht leistet.** Sie macht die Vertauschung nicht
//! unuebersetzbar, sie nimmt ihr den Anlass. Wer zwei Fragen entgegengesetzter
//! Polaritaet in demselben Typ fuehrt, kann sie weiter verwechseln; dagegen
//! stuende allein der zweite Weg aus
//! `issues/260817-1419_*_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`,
//! zwei Typen fuer zwei Fragen, und der ist unberuehrt.
//!
//! **Die Zaehlprobe `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` bleibt
//! stehen, und ihr Gegenstand hat gewechselt.** Bis zur Umbenennung hielt sie
//! einen Fehler ab: [`Loeschzielbefund::ist_warnwuerdig`] an einem Wert der
//! zweiten Polaritaet. Diesen Fehler gibt es hier nicht mehr, denn die Frage
//! ist jetzt die richtige. Was die Zaehlung seitdem festhaelt, ist eine
//! Modulgrenze: dieses Modul **beantwortet** den Ausloeser und **beurteilt**
//! ihn nicht. Ob die Rueckfrage laut wird, entscheidet
//! `crate::kommandos::loeschwarnung::warngruende` an einer Stelle fuer alle
//! sechs Ausloeser; eine Datei, die ihren eigenen Befund schon hier nach seiner
//! Warnwuerdigkeit fragt, legt die Rangfolge aus C3 ein zweites Mal an.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace`, `NSNotificationCenter`, `NSNotification`, `NSFileManager`,
//! `NSArray`, `NSDictionary`, `NSString`, `NSNumber` und `NSURL` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso die drei beobachteten Meldungsnamen
//! (`NSWorkspaceDidMountNotification`, `NSWorkspaceWillUnmountNotification`,
//! `NSWorkspaceDidUnmountNotification`) und die Beruehrungen `sharedWorkspace`,
//! `notificationCenter`, `addObserver:selector:name:object:`, `removeObserver:`,
//! `userInfo`, `objectForKey:`, `fileURLWithPath:` (`NSURL.h:52`, die Form ohne
//! weitere Argumente), `boolValue` (`NSValue.h:73`) und `NSURL.path`. Sieben
//! Beruehrungen sind juenger: `NSWorkspaceVolumeURLKey`,
//! `NSWorkspaceVolumeLocalizedNameKey`,
//! `mountedVolumeURLsIncludingResourceValuesForKeys:options:`,
//! `resourceValuesForKeys:error:` (`NSURL.h:183`) und die Aufzaehlung
//! `NSVolumeEnumerationOptions` stehen seit 10.6,
//! `NSURLVolumeLocalizedNameKey` (`NSURL.h:344`) und `NSURLVolumeIsLocalKey`
//! (`NSURL.h:338`), beide `API_AVAILABLE(macos(10.7), …)`, seit 10.7. Jeder
//! Name traegt seine eigene Zeile: eine Zeilenangabe, die fuer ein Paar gilt,
//! stimmt beim Nachlesen fuer hoechstens einen der beiden. Ein Typname traegt
//! im Kopf keine Angabe und steht damit seit 10.0: `NSURLResourceKey`
//! (`NSURL.h:17`), der Schluesseltyp der Ressourcenwerte, ein `typedef` auf
//! `NSString` und keine eigene Klasse. Alle Zeilenangaben sind am 260817 in
//! `$(xcrun --show-sdk-path)/System/Library/Frameworks/Foundation.framework/Headers/`
//! nachgelesen und nicht uebernommen. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine Verfuegbarkeitspruefung
//! zur Laufzeit. `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der
//! Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist die
//! Gegenmassnahme.

use std::path::{Path, PathBuf};

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSWorkspace, NSWorkspaceDidMountNotification, NSWorkspaceDidUnmountNotification,
    NSWorkspaceVolumeLocalizedNameKey, NSWorkspaceVolumeURLKey, NSWorkspaceWillUnmountNotification,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSFileManager, NSNotification, NSNumber, NSObject, NSObjectProtocol,
    NSString, NSURL, NSURLVolumeIsLocalKey, NSURLVolumeLocalizedNameKey,
    NSVolumeEnumerationOptions,
};

use krk_core::verzeichnis::Loeschzielbefund;

use crate::leistenmodell::Ort;

/// Alle gerade eingehaengten Datentraeger, in der Reihenfolge des Systems (C5).
///
/// Sie fuellen zusammen mit dem Benutzerverzeichnis den unteren Teil der
/// Leiste. Aufgezaehlt wird mit `SkipHiddenVolumes`, also ohne die
/// Systemdatentraeger, die auch der Finder nicht zeigt; ein Nutzer, der
/// `/System/Volumes/VM` in seiner Leiste fuende, haette dort einen Eintrag, den
/// er nie gewollt hat.
///
/// Der Name ist der, den der Finder zeigt (`NSURLVolumeLocalizedNameKey`), und
/// faellt auf den letzten Namensteil des Einhaengepunkts zurueck. Dieselbe
/// Ruecknahme wie in [`Datentraegerziel::weitergeben`]: ein Eintrag ohne Namen
/// waere in der Leiste nicht zuzuordnen.
///
/// Ein Datentraeger ohne lesbaren Pfad wird uebergangen und nicht geraten,
/// ebenfalls wie dort.
pub fn eingehaengte() -> Vec<Ort> {
    // SAFETY: Ein Fremdsymbol von Foundation, der Schluesselname des
    // Datentraegernamens. Es wird gelesen und nicht geschrieben.
    let schluessel_name = unsafe { NSURLVolumeLocalizedNameKey };
    let schluessel = NSArray::from_slice(&[schluessel_name]);
    let verwalter = NSFileManager::defaultManager();
    let Some(orte) = verwalter.mountedVolumeURLsIncludingResourceValuesForKeys_options(
        Some(&schluessel),
        NSVolumeEnumerationOptions::SkipHiddenVolumes,
    ) else {
        return Vec::new();
    };

    orte.iter()
        .filter_map(|url| {
            let pfad = PathBuf::from(url.path()?.to_string());
            let name = url
                .resourceValuesForKeys_error(&schluessel)
                .ok()
                .and_then(|werte| werte.objectForKey(schluessel_name))
                .and_then(|wert| wert.downcast::<NSString>().ok())
                .map(|text| text.to_string())
                .unwrap_or_else(|| namensteil(&pfad));
            Some(Ort::neu(name, pfad))
        })
        .collect()
}

/// Der letzte Namensteil eines Pfades, oder der ganze Pfad.
///
/// Der Rueckfall fuer einen Datentraeger ohne Namen. `/` hat keinen
/// Namensteil, und dort ist der Pfad selbst die beste Auskunft.
fn namensteil(pfad: &Path) -> String {
    pfad.file_name()
        .map(|teil| teil.to_string_lossy().into_owned())
        .unwrap_or_else(|| pfad.display().to_string())
}

/// Ob der Ordner auf einem Netzlaufwerk liegt (C3, Ausloeser 3).
///
/// Gefragt wird der Ressourcenwert `NSURLVolumeIsLocalKey` am `NSURL` des
/// Ordners. Das System beantwortet ihn aus dem Einhaengepunkt, unter dem der
/// Pfad liegt; ein `NSNumber` mit einem Wahrheitswert kommt zurueck, und der
/// Kommentar der Kopfdatei nennt ihn „true if the volume is stored on a local
/// device".
///
/// **Der Rueckgabewert ist die Antwort des Ausloesers und nicht die des
/// Ressourcenwerts.** Die eine Umkehrung dazwischen steht im Rumpf, wenige
/// Zeilen unter dieser Erklaerung; der Modulkopf sagt, warum sie genau hier
/// geschieht und genau einmal.
///
/// **Die drei Ausgaenge, und welcher warnt:**
///
/// - [`Loeschzielbefund::Ja`] — der Datentraeger ist **kein** lokaler. **Das
///   ist der Warngrund**, denn der Ausloeser aus C3 lautet „der Datentraeger
///   des Ordners ist kein lokaler".
/// - [`Loeschzielbefund::Nein`] — der Datentraeger ist ein lokaler. Das ist die
///   **harmlose** Auskunft, und die Rueckfrage bleibt an diesem Ausloeser ruhig.
/// - [`Loeschzielbefund::Unentschieden`] — der Pfad ist kein gueltiges UTF-8,
///   das System nennt einen Fehler, oder es liefert den Schluessel ohne Wert
///   beziehungsweise mit einem Wert, der kein `NSNumber` ist. Das ist keine
///   Aussage ueber den Datentraeger, sondern eine ueber KRKs Kenntnis von ihm,
///   und **auch dieser Ausgang warnt**.
///
/// Ein fehlender Wert wird ausdruecklich **nicht** als „lokal" gelesen. Ein
/// Vorgabewert „lokal, wenn nichts dagegen spricht" waere der bequeme Weg und
/// stellte die Warnung genau dort still, wo KRK am wenigsten ueber das Ziel
/// weiss; die Zusage „Unentschieden gilt als laut" waere damit an dieser
/// Pruefung aufgegeben.
///
/// **Auf der ersten Polaritaet**, wie das Feld `Loeschziel::netzlaufwerk`, das
/// die Antwort aufnimmt: `Ja` warnt, `Unentschieden` gehoert zu ihm, und
/// [`Loeschzielbefund::ist_warnwuerdig`] waere damit fuer diesen Wert eine
/// zulaessige Frage. **Gestellt wird sie weder hier noch dort, wo die Rangfolge
/// aus C3 steht**: `crate::kommandos::loeschwarnung::warngruende` schreibt alle
/// drei Antworten aus, weil `Ja` auf den Wortlaut „von einem Netzlaufwerk"
/// fuehrt und `Unentschieden` auf „von einem Ziel unbekannter Einordnung",
/// und eine zusammenfassende Frage wuesste den Unterschied nicht mehr. Der
/// Modulkopf sagt es im Einzelnen.
///
/// Der Ordner kommt **aufgeloest** herein, wie bei
/// [`super::papierkorb::fuehrt_einen_papierkorb`] und aus demselben Grund: eine
/// Verknuepfung meldete sonst den Datentraeger ihres eigenen Ortes statt den
/// ihres Ziels. Diese Funktion ruft weder `canonicalize` noch sonst etwas am
/// Dateisystem; ein Pfad, der sich nicht aufloesen laesst, zaehlt beim Aufrufer
/// als [`Loeschzielbefund::Unentschieden`].
#[must_use = "der Befund entscheidet, ob die Rueckfrage das Netzlaufwerk nennt; fallengelassen bleibt sie darueber still"]
pub fn liegt_auf_netzlaufwerk(pfad: &Path) -> Loeschzielbefund {
    let Some(text) = pfad.to_str() else {
        return Loeschzielbefund::Unentschieden;
    };
    let url = NSURL::fileURLWithPath(&NSString::from_str(text));
    // SAFETY: Ein Fremdsymbol von Foundation, der Schluesselname der
    // Ortsangabe des Datentraegers. Es wird gelesen und nicht geschrieben,
    // ebenso wie der Namensschluessel in `eingehaengte`.
    let schluessel_lokal = unsafe { NSURLVolumeIsLocalKey };
    let schluessel = NSArray::from_slice(&[schluessel_lokal]);

    let Some(wert) = url
        .resourceValuesForKeys_error(&schluessel)
        .ok()
        .and_then(|werte| werte.objectForKey(schluessel_lokal))
        .and_then(|wert| wert.downcast::<NSNumber>().ok())
    else {
        return Loeschzielbefund::Unentschieden;
    };

    // **Die eine Umkehrung.** `NSURLVolumeIsLocalKey` antwortet „lokal", der
    // Ausloeser aus C3 fragt „kein lokaler". Sie steht hier und nicht beim
    // Aufrufer, damit Name, Rueckgabewert und das Feld, das ihn aufnimmt, in
    // dieselbe Richtung zeigen; der Modulkopf schreibt aus, was die frueher
    // gegenlaeufige Form gekostet haette.
    if wert.boolValue() {
        Loeschzielbefund::Nein
    } else {
        Loeschzielbefund::Ja
    }
}

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
            .unwrap_or_else(|| namensteil(&pfad));
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

/// Der Einhaengepunkt der Automatik fuer `/home`, der Ort des warnenden
/// Ausgangs von [`liegt_auf_netzlaufwerk`].
///
/// Er steht als Konstante neben den Proben und nicht in ihnen, weil zwei von
/// ihnen ihn brauchen: die eine prueft die Vorbedingung, die andere den Befund.
#[cfg(test)]
const AUTOMATIK_HOME: &str = "/System/Volumes/Data/home";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quellbaum::{aufrufstellen, quelldateien};

    /// Das Benutzerverzeichnis liegt nicht auf einem Netzlaufwerk.
    ///
    /// Der ruhige Ausgang an einem echten Ort, und die Probe braucht kein
    /// Fenster und keinen Hauptfaden: die Abfrage eines Ressourcenwerts an einem
    /// `NSURL` ist von jedem Faden aus zu stellen, ebenso wie die Vorpruefung in
    /// [`super::super::papierkorb`].
    ///
    /// **Die Erwartung ist seit dem 260817 [`Loeschzielbefund::Nein`]** und
    /// nicht mehr `Ja`: der Rueckgabewert traegt jetzt die Antwort des
    /// Ausloesers, und „liegt auf einem Netzlaufwerk" ist am
    /// Benutzerverzeichnis eines Mac mit eingebautem Datenband zu verneinen.
    #[test]
    fn das_benutzerverzeichnis_liegt_nicht_auf_einem_netzlaufwerk() {
        let Some(zuhause) = krk_core::ablage::pfade::benutzerverzeichnis() else {
            panic!(
                "das System nennt kein Benutzerverzeichnis, und ohne eines misst diese Probe nichts"
            );
        };
        assert_eq!(
            liegt_auf_netzlaufwerk(&zuhause),
            Loeschzielbefund::Nein,
            "das Benutzerverzeichnis {} liegt angeblich auf einem Netzlaufwerk",
            zuhause.display()
        );
    }

    /// Der Einhaengepunkt der `/home`-Automatik gilt als Netzlaufwerk.
    ///
    /// **Der warnende Ausgang, ohne dass eine Probe ein Netzlaufwerk einhaengen
    /// muesste.** macOS haengt seit der Trennung von System- und Datenband unter
    /// [`AUTOMATIK_HOME`] die Automatik `auto_home` ein; sie ist ein
    /// `autofs`-Einhaengepunkt, `/sbin/mount` fuehrt ihn ohne das Merkmal
    /// `local`, und `NSURLVolumeIsLocalKey` antwortet dort `false`. Ohne diese
    /// Probe waere [`liegt_auf_netzlaufwerk`] mit einem festen
    /// [`Loeschzielbefund::Nein`] gruen, und der dritte Ausloeser aus C3 haette
    /// keinen Beleg — genau die Lage, die bei der Frage nach dem Papierkorb
    /// `/dev` aufgeloest hat.
    ///
    /// **Der Wortlaut des Ausloesers ist „Netzlaufwerk", gemessen wird „kein
    /// lokaler Datentraeger".** Eine Automatik von `autofs` ist kein
    /// Netzlaufwerk im engeren Sinn, und trotzdem ist `Ja` hier die richtige
    /// Antwort: der Ausloeser aus C3 hangt am fehlenden Merkmal `local` und
    /// nicht an einem Protokoll. Der Spec waehlt „Netzlaufwerk" als den
    /// Wortlaut, den der Nutzer versteht.
    ///
    /// **Es ist der Zielpfad und nicht `/home`.** Am 260817 gemessen antwortet
    /// `/home` mit `true` und [`AUTOMATIK_HOME`] mit `false`, obwohl das erste
    /// eine Festverknuepfung auf das zweite ist. Woran das haengt, ist hier nicht
    /// geklaert und wird nicht geraten; die Messung steht, und wer den Pfad
    /// „vereinfacht", macht die Probe still gruen.
    ///
    /// # Warum die Vorbedingung mitgeprueft wird
    ///
    /// Ein Nutzer kann `/etc/auto_master` aendern und die `/home`-Automatik
    /// abschalten. Dann bleibt unter [`AUTOMATIK_HOME`] ein gewoehnlicher, leerer
    /// Ordner des Datenbands stehen, und der ist lokal: die Probe wuerde rot,
    /// ohne dass an [`liegt_auf_netzlaufwerk`] etwas falsch waere. Geprueft wird deshalb
    /// zuerst, dass dort ueberhaupt ein eigener Einhaengepunkt steht, und zwar an
    /// der Geraetekennung aus `stat(2)`: ein Einhaengepunkt traegt eine andere als
    /// sein uebergeordneter Ordner. Diese Vorpruefung braucht kein AppKit und
    /// nicht die Funktion, die sie sichern soll.
    ///
    /// Fehlt der Einhaengepunkt, **haelt die Probe an statt sich zu ueberspringen**.
    /// Ein stiller Sprung liesse den einzigen negativen Beleg dieser Datei
    /// verschwinden, ohne dass es jemandem auffiele; ein Anhalten nennt den
    /// Grund und die Stelle.
    #[test]
    fn ein_nicht_lokaler_datentraeger_wird_erkannt() {
        use std::os::unix::fs::MetadataExt;

        let einhaengepunkt = Path::new(AUTOMATIK_HOME);
        let eigen = std::fs::metadata(einhaengepunkt)
            .unwrap_or_else(|fehler| panic!("{AUTOMATIK_HOME} ist nicht lesbar: {fehler}"));
        let darueber = std::fs::metadata(
            einhaengepunkt
                .parent()
                .expect("der Pfad der Automatik hat einen uebergeordneten Ordner"),
        )
        .expect("der Ordner ueber der Automatik ist nicht lesbar");
        assert_ne!(
            eigen.dev(),
            darueber.dev(),
            "unter {AUTOMATIK_HOME} steht kein eigener Einhaengepunkt, \
             also misst diese Probe den warnenden Ausgang der Pruefung nicht; \
             ist die /home-Automatik in /etc/auto_master abgeschaltet?"
        );

        assert_eq!(
            liegt_auf_netzlaufwerk(einhaengepunkt),
            Loeschzielbefund::Ja,
            "{AUTOMATIK_HOME} gilt als lokal, also unterscheidet die Pruefung nicht"
        );
    }

    /// Ein Pfad, den es nicht gibt, bleibt unentschieden und wird nicht zum
    /// `Nein`.
    ///
    /// Der Zweig, in dem das System einen Fehler nennt. Er ist von
    /// [`super::super::papierkorb::fuehrt_einen_papierkorb`] zu unterscheiden:
    /// dort heisst ein Fehler [`Loeschzielbefund::Nein`], denn dort **ist** der
    /// Fehler die Antwort. Hier sagt er nichts ueber den Datentraeger, und ein
    /// `Nein` erklaerte ein Ziel fuer harmlos, das niemand gesehen hat.
    ///
    /// Im laufenden Programm kommt dieser Fall nicht an: der Aufrufer loest den
    /// Ordner vorher auf und zaehlt sein Scheitern selbst als unentschieden. Die
    /// Probe haelt den Zweig trotzdem fest, weil er der einzige ist, in dem ein
    /// bequemer Vorgabewert die Warnung stillstellen wuerde.
    #[test]
    fn ein_fehlender_pfad_bleibt_unentschieden() {
        let fehlt = Path::new("/nicht-da-krk-volumes/und-auch-das-nicht");
        assert!(
            !fehlt.exists(),
            "der Pfad der Probe steht im Dateisystem und misst damit nicht, was sie messen soll"
        );
        assert_eq!(
            liegt_auf_netzlaufwerk(fehlt),
            Loeschzielbefund::Unentschieden,
            "ein fehlender Pfad liefert nicht den unentschiedenen Befund"
        );
    }

    /// Ein Pfad ohne gueltiges UTF-8 bleibt unentschieden.
    ///
    /// Derselbe Fall wie in [`super::super::papierkorb`] und aus demselben
    /// Grund: `NSString` nimmt nur gueltiges UTF-8, und die Uebersetzung
    /// scheitert, bevor das System etwas gefragt worden ist.
    ///
    /// Das Byte `0xff` ist in keiner UTF-8-Folge zulaessig; der Ordner wird
    /// nicht angelegt, denn die Funktion greift vor dieser Pruefung nicht auf das
    /// Dateisystem zu.
    #[test]
    fn ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let krumm = PathBuf::from(OsStr::from_bytes(b"/tmp/krk-volumes-\xffkrumm"));
        assert!(
            krumm.to_str().is_none(),
            "der Pfad der Probe ist gueltiges UTF-8 und misst damit nicht, was sie messen soll"
        );
        assert_eq!(
            liegt_auf_netzlaufwerk(&krumm),
            Loeschzielbefund::Unentschieden,
            "ein Pfad ohne gueltiges UTF-8 liefert nicht den unentschiedenen Befund"
        );
    }

    /// In dieser Datei wird nach der Warnwuerdigkeit nicht gefragt.
    ///
    /// **Die Zaehlung bleibt, ihr Gegenstand hat am 260817 gewechselt.** Bis zur
    /// Umbenennung von `ist_lokal` hielt sie einen Fehler ab: der
    /// Rueckgabewert lag auf der zweiten Polaritaet, und
    /// [`Loeschzielbefund::ist_warnwuerdig`] haette aus einem lokalen
    /// Datentraeger einen Warngrund und aus einem Netzlaufwerk eine harmlose
    /// Auskunft gemacht. Diesen Fehler gibt es hier nicht mehr:
    /// [`liegt_auf_netzlaufwerk`] liefert die Antwort des Ausloesers, und die
    /// Warnwuerdigkeit ist fuer diesen Wert die **richtige** Frage.
    ///
    /// **Sie faellt trotzdem nicht, weil sie eine zweite, unveraenderte Zusage
    /// traegt: dieses Modul beantwortet den Ausloeser und beurteilt ihn nicht.**
    /// Ob die Rueckfrage laut wird und welcher der sechs Gruende sie benennt,
    /// entscheidet `crate::kommandos::loeschwarnung::warngruende` an einer
    /// Stelle. Eine Datei, die ihren eigenen Befund schon hier nach seiner
    /// Warnwuerdigkeit fragt, legt die Rangfolge aus C3 ein zweites Mal an, und
    /// die zweite Lage laeuft von der ersten weg, ohne dass eine Uebersetzung
    /// etwas dazu sagt. Rot wird die Probe, wenn dieses Modul anfaengt zu
    /// urteilen; die richtige Antwort darauf ist die Frage, warum es das tut.
    ///
    /// **Die Umkehrung waere die falsche Bewegung.** Eine Zusicherung, dass hier
    /// **doch** nach der Warnwuerdigkeit gefragt wird, machte die Modulgrenze zur
    /// Pflicht, sie zu ueberschreiten.
    ///
    /// Die Richtung stammt aus
    /// `issues/260817-1419_*_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`,
    /// erster Weg. **Der Befund ist damit nicht geschlossen:** er verlangt
    /// dieselbe Zaehlung auch in `appkit/papierkorb.rs`, und sein zweiter,
    /// staerkerer Weg — zwei Typen fuer zwei Fragen — bleibt unberuehrt.
    ///
    /// Was eine Zaehlung im Quelltext leistet und was nicht, steht im Modulkopf
    /// von [`crate::quellbaum`]. Die Nadel steht zusammengesetzt da, weil die
    /// Probe in dem Baum liegt, den sie liest.
    #[test]
    fn hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt() {
        let zuhause = "krk-ui/src/appkit/volumes.rs";
        let name = concat!("ist_warn", "wuerdig");
        let dateien = quelldateien();
        let Some((_, inhalt)) = dateien.iter().find(|(datei, _)| datei == zuhause) else {
            panic!("{zuhause} steht nicht im gelesenen Quellbaum; die Zaehlung misst nichts");
        };
        assert_eq!(
            aufrufstellen(inhalt, name),
            0,
            "diese Datei fragt nach der Warnwuerdigkeit, und ihr Befund traegt den Warngrund \
             auf der anderen Antwort"
        );
    }
}
