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
//! `issues/260803-2007_*_s14-bindet-fsevents-ohne-das-framework-coreservices-zu-verlinken.md`.
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
//! Der Rueckruf laeuft auf der Warteschlange, der der Strom zugeteilt ist. Das
//! ist die Hauptwarteschlange, die der Hauptfaden abarbeitet, und deshalb darf
//! die Senke die Oberflaeche anfassen.
//!
//! # Warteschlange statt Laufschleife
//!
//! Bis zum 260805 haengte der Strom ueber `FSEventStreamScheduleWithRunLoop` in
//! der Laufschleife des Hauptfadens. Der Kopf des Systems fuehrt diesen Aufruf
//! seit macOS 13 als abgeloest: `API_DEPRECATED("Use
//! FSEventStreamSetDispatchQueue instead.", macos(10.5, 13.0), ios(6.0,16.0))`
//! in `FSEvents.h`. Rust sieht Apples Vermerk an einer von Hand geschriebenen
//! Bindung nicht, der Uebersetzer warnte also nie; ein Aufruf, den Apple seit
//! drei Hauptversionen abgeloest fuehrt, ist trotzdem eine Zusage auf Zeit.
//! Gewechselt in
//! `issues/260804-1451_*_fseventstreamschedulewithrunloop-ist-seit-macos-13-als-veraltet-gekennzeichnet.md`.
//!
//! **Die Zuteilung kostet keine neue Bindung.** `dispatch_get_main_queue()` ist
//! im Kopf des Systems eine `static inline`-Funktion und keine Ausfuhr, der Weg
//! dorthin waere das Symbol `_dispatch_main_q`. Diesen Weg geht die Kiste
//! `dispatch2` bereits, und KRK fuehrt sie seit Schritt 16 fuer den Weckruf des
//! Vermittlerfadens; `DispatchQueue::main()` ist genau dieselbe Warteschlange.
//!
//! **Was der Wechsel am Verhalten aendert: nichts, das hier zaehlt.** Die
//! Hauptwarteschlange wird vom Hauptfaden abgearbeitet, der Rueckruf laeuft also
//! weiter dort. Die Laufschleifen-Form brauchte dafuer ausdruecklich die
//! gemeinsamen Modi, weil der gewoehnliche Modus ruht, solange der Nutzer
//! blaettert oder ein Menue offen haelt; eine Warteschlange kennt diese
//! Unterscheidung nicht und wird in beiden Faellen abgearbeitet. Die Ueberlegung
//! zu den Modi entfaellt damit, statt uebergangen zu werden.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! **Dieses Modul spricht keine Klasse an.** Es ist damit die eine Ausnahme
//! unter den Modulen dieses Verzeichnisses: die Gewohnheit, jede Untergrenze im
//! Modulkopf zu nennen, gilt trotzdem, sie zaehlt hier nur C-Funktionen und
//! CoreFoundation-Typen statt Objective-C-Klassen. Der Grund, aus dem `objc2`
//! hier nicht hilft, ist derselbe wie ueberall — es fuehrt keine
//! Verfuegbarkeitsangaben mit sich —, und an einer von Hand geschriebenen
//! Bindung sieht der Uebersetzer ohnehin keine.
//!
//! Die sechs gebundenen Funktionen stehen laut `FSEvents.h` im SDK seit macOS
//! 10.5: `FSEventStreamCreate`, `FSEventStreamStart`, `FSEventStreamStop`,
//! `FSEventStreamInvalidate` und `FSEventStreamRelease` tragen
//! `__OSX_AVAILABLE_STARTING(__MAC_10_5, …)`; allein
//! `FSEventStreamSetDispatchQueue` steht seit 10.6
//! (`__OSX_AVAILABLE_STARTING(__MAC_10_6, …)`). Die beiden benannten
//! Konstanten [`KENNZEICHEN_CF_TYPEN`] und [`SEIT_JETZT`] tragen im Kopf keine
//! eigene Angabe. Was der Kopf zum abgeloesten
//! `FSEventStreamScheduleWithRunLoop` vermerkt, steht oben unter
//! "Warteschlange statt Laufschleife" und wird hier nicht wiederholt.
//!
//! Die angesprochenen CoreFoundation-Typen sind `CFArray`, `CFString` und der
//! Ganzzahltyp `CFIndex`; sie und die dahinterliegenden Aufrufe
//! (`CFArrayCreate`, `CFArrayGetCount`, `CFArrayGetValueAtIndex`,
//! `CFStringCreateWithBytes`, `CFStringGetBytes`) tragen in den
//! CoreFoundation-Koepfen keine Verfuegbarkeitsangabe und stehen damit seit
//! macOS 10.0. `CFRetained` ist **kein** Systemtyp, sondern der zaehlende
//! Zeiger der Kiste `objc2-core-foundation`, und stellt keine
//! Verfuegbarkeitsfrage. Die Hauptwarteschlange kommt ueber
//! `DispatchQueue::main()` aus `dispatch2`, also ueber das Symbol
//! `_dispatch_main_q` von libdispatch (seit 10.6).
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); nichts davon ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit.

use std::ffi::c_void;
use std::path::PathBuf;

use dispatch2::DispatchQueue;
use objc2_core_foundation::{CFArray, CFIndex, CFRetained, CFString};

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

    /// `void FSEventStreamSetDispatchQueue(FSEventStreamRef, dispatch_queue_t)`
    ///
    /// Der Ersatz fuer `FSEventStreamScheduleWithRunLoop`, siehe Modulkopf.
    /// `dispatch_queue_t` ist ein Zeiger auf das Warteschlangenobjekt; die
    /// Referenz auf [`DispatchQueue`] ist genau das.
    fn FSEventStreamSetDispatchQueue(strom: FSEventStreamRef, warteschlange: &DispatchQueue);

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

        // SAFETY: `strom` ist der eben angelegte und noch keiner Warteschlange
        // zugeteilte Strom; `DispatchQueue::main()` liefert die
        // Hauptwarteschlange, die es fuer die Dauer des Prozesses gibt.
        unsafe { FSEventStreamSetDispatchQueue(strom, DispatchQueue::main()) };

        // SAFETY: `strom` ist zugeteilt und noch nicht gestartet.
        let gestartet = unsafe { FSEventStreamStart(strom) } != 0;
        if !gestartet {
            // SAFETY: `strom` ist angelegt und zugeteilt, aber nicht
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
        // SAFETY: `self.strom` ist ein gestarteter, zugeteilter Strom. Die
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
