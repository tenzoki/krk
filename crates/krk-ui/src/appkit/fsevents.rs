//! Die Beobachtung der sichtbaren Ordner ueber FSEvents (C9).
//!
//! FSEvents ist eine C-Schnittstelle und keine Objective-C-Klasse. Sie wird
//! hier von Hand gebunden, in derselben Form wie `getattrlistbulk` in
//! `krk-core/src/verzeichnis/sys.rs`: ein `unsafe extern "C"`-Block mit den
//! Signaturen, die der Kopf des Systems nennt, und darueber eine Huelle, die
//! nur gewoehnliche Rust-Werte herausgibt.
//!
//! **Der Vergleich mit `getattrlistbulk` traegt nicht bis zum Binder, und
//! deshalb steht `#[link]` an dem Block.** `getattrlistbulk` liegt in
//! `libSystem`, das jedes Rust-Programm auf macOS ohnehin bekommt;
//! `FSEventStreamCreate` liegt in `CoreServices`, und keine Kiste dieses
//! Workspace nennt dieses Framework. Ein Rust-Programm, das allein diesen
//! `extern`-Block enthaelt, bindet ohne das Attribut nicht: `ld` meldet
//! `Undefined symbols: _FSEventStreamCreate`. Am 260804 als eigenes
//! Probeprogramm nachgestellt.
//!
//! **KRK bindet trotzdem auch ohne das Attribut, und das ist kein Grund, es
//! wegzulassen.** `AppKit` reexportiert `ApplicationServices`, und das
//! reexportiert `CoreServices`; der Binder findet das Symbol ueber diese Kette
//! und traegt `CoreServices` von sich aus in die Ladebefehle ein. Damit haengt
//! die Aufloesung an einer Zusage, die AppKit gibt und nicht KRK. Das Attribut
//! macht die Abhaengigkeit ausdruecklich, es steht bei der Bindung, die sie
//! braucht, und es bleibt richtig, wenn die Kette sich aendert. Nachgewiesen
//! und entschieden in
//! `issues/260803-2007_c_s14-bindet-fsevents-ohne-das-framework-coreservices-zu-verlinken.md`.
//!
//! # Was beobachtet wird
//!
//! Die Ordner, die gerade in einem der beiden Dateifenster stehen, hoechstens
//! zwei. Welche das sind, weiss allein das Fenstermodell; der Strom wird bei
//! jeder Navigation neu aufgesetzt, weil ein FSEventStream seine Pfadliste
//! nach dem Anlegen nicht mehr aendert.
//!
//! Die Sammelverzoegerung betraegt 300 ms: FSEvents wartet nach der ersten
//! Aenderung diese Spanne ab und meldet dann alles zusammen. Damit liegt eine
//! im Terminal angelegte Datei deutlich innerhalb der Sekunde im Fenster, die
//! das Abnahmekriterium von S14 zusagt, und ein `cp` ueber tausend Dateien
//! loest trotzdem nur einen Lesevorgang aus.
//!
//! `kFSEventStreamCreateFlagFileEvents` ist ausdruecklich **nicht** gesetzt.
//! Die Aufloesung auf Verzeichnisebene genuegt, weil KRK ohnehin den ganzen
//! Ordner neu liest; mit dem Kennzeichen kaeme je geaenderter Datei eine
//! Meldung, und aus tausend Meldungen entstuende derselbe eine Lesevorgang.
//!
//! # Wer den Rueckruf traegt
//!
//! FSEvents kennt keine Rust-Abschluesse. Der Weg dorthin ist der
//! `info`-Zeiger des Kontexts: [`Dateisystemwache`] haelt die Senke in einer
//! `Box` und gibt ihre Adresse als `info` mit. Der Strom haelt die Adresse,
//! die Wache haelt die `Box`; faellt die Wache, wird der Strom vorher
//! angehalten und freigegeben, und danach faellt die `Box`. Ein Rueckruf auf
//! eine gefallene Senke gibt es damit nicht.
//!
//! Der Rueckruf laeuft auf dem Faden, dessen Laufschleife den Strom
//! eingeplant hat. Das ist der Hauptfaden, und deshalb darf die Senke die
//! Oberflaeche anfassen.

use std::ffi::c_void;
use std::path::PathBuf;

use objc2_core_foundation::{
    CFArray, CFIndex, CFRetained, CFRunLoop, CFRunLoopMode, CFString, kCFRunLoopCommonModes,
};

/// Die Spanne, die FSEvents Aenderungen sammelt, bevor es meldet.
///
/// 300 ms, wie `### Frage 3` des Plans sie festlegt.
const SAMMELVERZOEGERUNG: f64 = 0.3;

/// `kFSEventStreamCreateFlagUseCFTypes`: die Pfade kommen als `CFArray` von
/// `CFString` und nicht als `char**`.
///
/// Das ist das einzige gesetzte Kennzeichen. `FileEvents` bleibt aus, siehe
/// Modulkopf.
const KENNZEICHEN_CF_TYPEN: u32 = 0x0000_0001;

/// `kFSEventStreamEventIdSinceNow`: gemeldet wird, was ab jetzt geschieht.
const SEIT_JETZT: u64 = u64::MAX;

/// Ein Beobachtungsstrom, wie FSEvents ihn fuehrt. Sein Inneres ist verdeckt.
#[repr(C)]
struct FSEventStream {
    _verdeckt: [u8; 0],
}

/// `FSEventStreamRef` aus `FSEvents.h`.
type FSEventStreamRef = *mut FSEventStream;

/// `ConstFSEventStreamRef` aus `FSEvents.h`.
type ConstFSEventStreamRef = *const FSEventStream;

/// `struct FSEventStreamContext` aus `FSEvents.h`.
///
/// `version` traegt heute allein die Null; die drei Rueckrufe bleiben leer,
/// weil `info` auf eine Rust-`Box` zeigt und nicht auf ein CoreFoundation-
/// Objekt, das sich zaehlen liesse. Die Lebensdauer traegt [`Dateisystemwache`].
#[repr(C)]
struct FSEventStreamContext {
    version: CFIndex,
    info: *mut c_void,
    retain: Option<unsafe extern "C" fn(*const c_void) -> *const c_void>,
    release: Option<unsafe extern "C" fn(*const c_void)>,
    beschreiben: Option<unsafe extern "C" fn(*const c_void) -> *const c_void>,
}

/// `FSEventStreamCallback` aus `FSEvents.h`.
type FSEventStreamCallback = unsafe extern "C" fn(
    strom: ConstFSEventStreamRef,
    info: *mut c_void,
    zahl: usize,
    pfade: *mut c_void,
    kennzeichen: *const u32,
    nummern: *const u64,
);

// Das Attribut, um das es im Defekt 260803-2007 geht. Warum es steht, obwohl
// KRK auch ohne bindet, sagt der Modulkopf.
#[link(name = "CoreServices", kind = "framework")]
unsafe extern "C" {
    /// `FSEventStreamRef FSEventStreamCreate(CFAllocatorRef, FSEventStreamCallback,
    /// FSEventStreamContext *, CFArrayRef, FSEventStreamEventId, CFTimeInterval,
    /// FSEventStreamCreateFlags)`
    ///
    /// Liefert einen Nullzeiger, wenn sich der Strom nicht anlegen liess.
    fn FSEventStreamCreate(
        zuteiler: *const c_void,
        rueckruf: FSEventStreamCallback,
        kontext: *mut FSEventStreamContext,
        pfade: &CFArray<CFString>,
        seit: u64,
        verzoegerung: f64,
        kennzeichen: u32,
    ) -> FSEventStreamRef;

    /// `void FSEventStreamScheduleWithRunLoop(FSEventStreamRef, CFRunLoopRef, CFStringRef)`
    fn FSEventStreamScheduleWithRunLoop(
        strom: FSEventStreamRef,
        schleife: &CFRunLoop,
        modus: &CFRunLoopMode,
    );

    /// `Boolean FSEventStreamStart(FSEventStreamRef)`
    ///
    /// `Boolean` ist in CoreFoundation ein `unsigned char`; deshalb `u8` und
    /// nicht `bool`.
    fn FSEventStreamStart(strom: FSEventStreamRef) -> u8;

    /// `void FSEventStreamStop(FSEventStreamRef)`
    fn FSEventStreamStop(strom: FSEventStreamRef);

    /// `void FSEventStreamInvalidate(FSEventStreamRef)`
    fn FSEventStreamInvalidate(strom: FSEventStreamRef);

    /// `void FSEventStreamRelease(FSEventStreamRef)`
    fn FSEventStreamRelease(strom: FSEventStreamRef);
}

/// Die Senke, an die jeder Schwung gemeldeter Ordner geht.
///
/// Zweifach eingepackt, damit der `info`-Zeiger ein gewoehnlicher Zeiger ist
/// und kein fetter: `dyn Fn` allein waere unsized.
type Senke = Box<dyn Fn(&[PathBuf])>;

/// Der Rueckruf, den FSEvents aufruft.
///
/// # Safety
///
/// Wird allein von FSEvents gerufen, mit dem `info`-Zeiger, den
/// [`Dateisystemwache::einrichten`] gesetzt hat, und mit `pfade` als
/// `CFArrayRef` von `CFStringRef`, weil [`KENNZEICHEN_CF_TYPEN`] gesetzt ist.
unsafe extern "C" fn gemeldet(
    _strom: ConstFSEventStreamRef,
    info: *mut c_void,
    zahl: usize,
    pfade: *mut c_void,
    _kennzeichen: *const u32,
    _nummern: *const u64,
) {
    if info.is_null() || pfade.is_null() || zahl == 0 {
        return;
    }
    // SAFETY: `info` ist die Adresse der `Senke`, die die Wache haelt; sie
    // lebt laenger als der Strom, weil die Wache den Strom vor der Senke
    // freigibt.
    let senke = unsafe { &*(info as *const Senke) };
    // SAFETY: Mit `kFSEventStreamCreateFlagUseCFTypes` reicht FSEvents hier
    // ein `CFArrayRef` von `CFStringRef` herein, das fuer die Dauer des
    // Aufrufs lebt.
    let liste = unsafe { &*(pfade as *const CFArray<CFString>) };
    let ordner: Vec<PathBuf> = (0..liste.len())
        .filter_map(|stelle| liste.get(stelle))
        .map(|pfad| PathBuf::from(pfad.to_string()))
        .collect();
    if !ordner.is_empty() {
        senke(&ordner);
    }
}

/// Ein laufender Beobachtungsstrom.
///
/// Er beobachtet, solange dieser Wert lebt. Wer ihn fallen laesst, nimmt die
/// Beobachtung damit zurueck; dieselbe Form wie beim Tastenabgriff aus S7 und
/// beim [`Zeichenende`](super::bildtakt::Zeichenende) aus S8, die sich beide in
/// ihrem `Drop` abmelden. Ohne Halter meldete sich der Strom beim
/// Fallenlassen sofort wieder ab.
pub struct Dateisystemwache {
    strom: FSEventStreamRef,
    /// Die Senke bleibt hier, weil der Strom nur ihre Adresse haelt.
    ///
    /// Die `Box` liegt auf dem Haufen; sie mit der Wache zu verschieben
    /// aendert die Adresse nicht.
    _senke: Box<Senke>,
}

impl Dateisystemwache {
    /// Beobachtet die genannten Ordner und meldet jede Aenderung an `senke`.
    ///
    /// Liefert `None` fuer eine leere Pfadliste und dann, wenn FSEvents den
    /// Strom nicht anlegt oder nicht startet. Der Aufrufer meldet das; still
    /// ohne Beobachtung weiterzulaufen hiesse, eine Anwendung auszuliefern,
    /// die fremde Aenderungen nicht anzeigt, ohne dass jemand es erfaehrt.
    pub fn einrichten(ordner: &[PathBuf], senke: impl Fn(&[PathBuf]) + 'static) -> Option<Self> {
        if ordner.is_empty() {
            return None;
        }
        let pfade: Vec<CFRetained<CFString>> = ordner
            .iter()
            .map(|pfad| CFString::from_str(&pfad.to_string_lossy()))
            .collect();
        let liste = CFArray::from_retained_objects(&pfade);

        let senke: Box<Senke> = Box::new(Box::new(senke));
        let mut kontext = FSEventStreamContext {
            version: 0,
            info: (&raw const *senke) as *mut c_void,
            retain: None,
            release: None,
            beschreiben: None,
        };

        // SAFETY: `liste` ist ein CFArray von CFString und lebt ueber den
        // Aufruf hinweg; `kontext` wird von FSEventStreamCreate ausgelesen und
        // nicht festgehalten; `gemeldet` hat die Signatur, die der Kopf des
        // Systems fuer `FSEventStreamCallback` nennt.
        let strom = unsafe {
            FSEventStreamCreate(
                std::ptr::null(),
                gemeldet,
                &raw mut kontext,
                &liste,
                SEIT_JETZT,
                SAMMELVERZOEGERUNG,
                KENNZEICHEN_CF_TYPEN,
            )
        };
        if strom.is_null() {
            return None;
        }

        let schleife = CFRunLoop::current()?;
        // Die gemeinsamen Modi und nicht der gewoehnliche, aus demselben Grund
        // wie beim Einzugstakt der Dateiliste: der gewoehnliche Modus ruht,
        // solange der Nutzer blaettert oder ein Menue offen haelt.
        // SAFETY: Ein Fremdsymbol von CoreFoundation, dieselbe Art Zugriff wie
        // auf `NSRunLoopCommonModes` beim Einzugstakt der Dateiliste.
        let modus = unsafe { kCFRunLoopCommonModes }?;
        // SAFETY: `strom` ist der eben angelegte und noch nicht eingeplante
        // Strom; `schleife` ist die Laufschleife dieses Fadens.
        unsafe { FSEventStreamScheduleWithRunLoop(strom, &schleife, modus) };

        // SAFETY: `strom` ist eingeplant und noch nicht gestartet.
        let gestartet = unsafe { FSEventStreamStart(strom) } != 0;
        if !gestartet {
            // SAFETY: `strom` ist angelegt und eingeplant, aber nicht
            // gestartet; genau dafuer sind die beiden Aufrufe da.
            unsafe {
                FSEventStreamInvalidate(strom);
                FSEventStreamRelease(strom);
            }
            return None;
        }

        Some(Self {
            strom,
            _senke: senke,
        })
    }
}

impl Drop for Dateisystemwache {
    fn drop(&mut self) {
        // SAFETY: `self.strom` ist ein gestarteter, eingeplanter Strom. Die
        // Reihenfolge Anhalten, Ungueltigmachen, Freigeben ist die, die
        // `FSEvents.h` im Abschnitt "Lifecycle" vorschreibt. Danach ruft
        // FSEvents `gemeldet` nicht mehr, und erst deshalb darf die Senke
        // unmittelbar hinterher fallen.
        unsafe {
            FSEventStreamStop(self.strom);
            FSEventStreamInvalidate(self.strom);
            FSEventStreamRelease(self.strom);
        }
    }
}
