//! Die Systemschicht des Kerns: die fuenf Schnittstellen, die KRK braucht, und
//! die neun Funktionen, die sie binden.
//!
//! Dies ist das einzige Modul in `krk-core`, das die Regel `deny(unsafe_code)`
//! aus `lib.rs` oeffnet. Die Regel lautet dort `deny` und nicht `forbid`, damit
//! genau diese Oeffnung moeglich ist. Ein zweites Modul mit dieser Ausnahme
//! entsteht nicht; mit Schritt 15 sind `copyfile(3)` und `renamex_np(2)`
//! hierhergekommen statt daneben, mit dem Defekt `260809-1652` `fcntl(2)`.
//!
//! ```text
//! getattrlistbulk(2) ──> Schwungleser         ──> verzeichnis::leser
//! copyfile(3)        ──> datei_kopieren       ──> operation::kopieren
//!   copyfile_state_{alloc,free,set,get}
//! renamex_np(2)      ──> im_datentraeger_...  ──> operation::{verschieben,umbenennen}
//! fcntl(2)           ──> ohne_warten_oeffnen  ──> text::datei::lesen
//!                                             ├─> text::datei::bis_zur_grenze_lesen
//!                                             ├─> operation::zippen
//!                                             └─> operation::entpacken
//! flock(2)           ──> sperre_nehmen        ──> ablage::sperre
//!                        sperre_versuchen
//!                        sperre_abgeben
//! ```
//!
//! **Fuenf ist die Zahl der Schnittstellen und nicht die der Bindungen: es sind
//! fuenf Schnittstellen und neun gebundene Funktionen, denn `copyfile(3)`
//! braucht seine vier `copyfile_state_*`-Helfer.** Ohne sie liesse sich der
//! Fortschrittsrueckruf nicht setzen und die Zahl der kopierten Bytes nicht
//! abfragen; eine eigene Schnittstelle sind sie deshalb nicht, aber vier weitere
//! Aufrufe ueber die Sprachgrenze schon, und die Reichweite der Ausnahme oben
//! liest ein Leser hier nach. Die Zeile steht wortgleich in `lib.rs` und in
//! `verzeichnis/mod.rs`; die dritte Stelle hat der Defekt `260810-1017`
//! nachgezogen, die fuenfte Schnittstelle die Runde 7. Gebunden sind alle neun
//! in den vier `unsafe extern "C"`-Bloecken dieses Moduls, und gerufen sind sie
//! ebenfalls alle neun.
//!
//! **`flock(2)` ist die fuenfte, und sie ist die erste, die keine Datei liest
//! oder schreibt, sondern eine Absprache zwischen zwei Prozessen traegt.** An
//! ihr haengen die Schreibsperre und das Sitzungsrecht der Ablage, beide in
//! [`crate::ablage::sperre`]. Gewaehlt ist sie, weil der Kern eine
//! `flock`-Sperre beim Prozessende von sich aus freigibt, auch nach einem
//! Absturz; eine Marke im Dateisystem ueber `create_new` oder ueber
//! `renamex_np` mit `RENAME_EXCL` ueberlebte ihn und sperrte danach jede
//! weitere Instanz von KRK fuer immer aus.
//!
//! Der Name des Moduls ist damit weiter gedeckt: es ist die Systemschicht des
//! Kerns und nicht allein die des Lesers. Die Zeile zu `fcntl(2)` traegt die
//! ersten Aufrufer von ausserhalb `verzeichnis/`, und sie sind der Grund, aus
//! dem die Aussage nicht mehr nur behauptet ist. **Wie viele es sind, sagt
//! `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` und nicht diese
//! Zeile**; sie stand vom Defekt `260810-1247` bis zur Runde 17 auf zwei und ist
//! mit den zwei Archivwegen jener Runde falsch geworden. Die zwei aeltesten
//! liegen seit der Runde 11 in `text/datei.rs`: [`crate::text::datei::lesen`]
//! fuer den Editor und [`crate::text::datei::bis_zur_grenze_lesen`] fuer jeden,
//! der seine eigene Grenze mitbringt. Die zwei juengsten sind
//! [`crate::operation::zippen`], das jede Quelle in ein Archiv liest, und
//! [`crate::operation::entpacken`], das jedes Archiv oeffnet; beide aus dem
//! gleichen Grund wie die aelteren, naemlich einer benannten Roehre im Ordner,
//! die ein `File::open` bis in alle Ewigkeit anhielte. Die Vorschau und der
//! Inhaltsfilter der Dateiliste, beide in `krk-ui`, rufen die Huelle und nicht
//! diese Stelle; bis zur Runde 11 stand die Huelle in `krk-ui`s
//! `vorschaumodell.rs`, und der zweite Aufrufer lag damit ausserhalb der Kiste.
//! Warum die Zielpruefung trotzdem bei den
//! Aufrufern bleibt, steht bei [`ohne_warten_oeffnen`]. Das Modul liegt unter
//! `verzeichnis/`, weil es dort entstanden ist und ein Umzug jede Fundstelle
//! verschoebe, ohne eine Zeile besser zu machen.
//!
//! # Warum `getattrlistbulk` und nicht `readdir` plus `stat`
//!
//! Ein Verzeichniseintrag braucht Name, Groesse, Aenderungsdatum, Typ und die
//! Kennzeichnung als versteckt. `readdir` liefert davon Name und einen groben
//! Typ; jeder weitere Wert kostet einen eigenen Systemaufruf. Bei 100.000
//! Eintraegen waeren das 100.000 zusaetzliche Aufrufe. `getattrlistbulk`
//! liefert alle Attribute gebuendelt fuer viele Eintraege je Aufruf und ist
//! seit macOS 10.10 verfuegbar, also weit unter dem Mindest-Zielsystem 15.0.
//!
//! # Aufbau des Antwortpuffers
//!
//! Der Kern liefert je Eintrag einen Satz. Der Satz beginnt mit seiner eigenen
//! Laenge als `u32`, danach folgt `ATTR_CMN_RETURNED_ATTRS`, danach die
//! uebrigen angeforderten Attribute in aufsteigender Reihenfolge ihrer
//! Bitwerte aus `sys/attr.h`, dicht gepackt und ohne Ausrichtungsluecken.
//! Der Zerleger richtet sich nach dem zurueckgemeldeten Bitmuster, statt eine
//! feste Satzstruktur anzunehmen: ein Ordner bekommt keine Dateiattribute, und
//! sein Satz ist entsprechend kuerzer.
//!
//! Der Aufbau ist am Geraet nachgemessen und nicht nur nachgelesen. Ein
//! Zahlendreher an dieser Stelle faellt nicht auf, er liefert nur falsche
//! Werte; deshalb prueft `tests/verzeichnis.rs` jeden gelesenen Wert gegen
//! `std::fs`.
//!
//! Weil jeder Satz seine Laenge selbst traegt, kann ein Fehler beim Zerlegen
//! eines Satzes die Satzgrenzen nicht verschieben. Der Schaden bleibt auf
//! einen Eintrag begrenzt.
//!
//! `ATTR_CMN_ERROR` fordert dieses Modul nicht an. Ein Eintrag, zu dem der
//! Kern nichts liefern kann, kommt ohne Namen zurueck und wird uebergangen.
#![allow(unsafe_code)]

use std::borrow::Cow;
use std::ffi::{CString, c_char, c_int, c_uint, c_void};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::eintrag::Typ;

/// Groesse des Antwortpuffers. Apples Beispiel zu `getattrlistbulk` verwendet
/// dieselbe Groesse. Sie fasst je nach Namenslaenge etwa 2.000 bis 4.000
/// Eintraege und haelt die Zahl der Systemaufrufe damit klein.
const PUFFERGROESSE: usize = 256 * 1024;

/// `ATTR_BIT_MAP_COUNT` aus `sys/attr.h`: die Zahl der Attributgruppen.
const ATTR_BIT_MAP_COUNT: u16 = 5;

const ATTR_CMN_NAME: u32 = 0x0000_0001;
const ATTR_CMN_OBJTYPE: u32 = 0x0000_0008;
const ATTR_CMN_MODTIME: u32 = 0x0000_0400;
const ATTR_CMN_FLAGS: u32 = 0x0004_0000;
const ATTR_CMN_RETURNED_ATTRS: u32 = 0x8000_0000;

const ATTR_FILE_DATALENGTH: u32 = 0x0000_0200;

/// `UF_HIDDEN` aus `sys/stat.h`: das Dateisystem kennzeichnet den Eintrag als
/// versteckt, unabhaengig von einem fuehrenden Punkt im Namen.
const UF_HIDDEN: u32 = 0x0000_8000;

/// `fsobj_type_t` aus `sys/vnode.h`, soweit hier gebraucht.
const VDIR: u32 = 2;
const VLNK: u32 = 5;

/// `struct attrlist` aus `sys/attr.h`.
#[repr(C)]
#[derive(Clone, Copy)]
struct Attrlist {
    bitmapcount: u16,
    reserved: u16,
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

unsafe extern "C" {
    /// `int getattrlistbulk(int, struct attrlist *, void *, size_t, uint64_t)`
    ///
    /// Liefert die Zahl der Eintraege im Puffer, 0 am Ende des Verzeichnisses
    /// und -1 im Fehlerfall.
    fn getattrlistbulk(
        dirfd: c_int,
        attrlist: *mut Attrlist,
        attr_buf: *mut c_void,
        attr_buf_size: usize,
        options: u64,
    ) -> c_int;
}

/// Ein Eintrag, wie ihn der Kern geliefert hat, noch ohne Sortierschluessel.
///
/// Der Name leiht sich aus dem Antwortpuffer und lebt nur bis zum naechsten
/// Schwung. Wer ihn behalten will, baut daraus einen [`super::Eintrag`].
#[derive(Debug)]
pub struct RohEintrag<'a> {
    /// Der Name ohne Pfad.
    pub name: Cow<'a, str>,
    /// Ordner, Datei oder symbolische Verknuepfung.
    pub typ: Typ,
    /// Die Groesse der Daten in Bytes.
    pub groesse: u64,
    /// Der Zeitpunkt der letzten Aenderung.
    pub geaendert: SystemTime,
    /// Das Kennzeichen `UF_HIDDEN` des Dateisystems.
    pub systemseitig_versteckt: bool,
}

/// Liest ein Verzeichnis schwungweise ueber `getattrlistbulk(2)`.
///
/// Ein Schwung ist so viel, wie in den Antwortpuffer passt. Die Stapel zu 1.024
/// Eintraegen, die an den Hauptfaden gehen, schneidet der [`super::leser`]
/// daraus zu; die beiden Groessen haben nichts miteinander zu tun.
pub struct Schwungleser {
    verzeichnis: File,
    puffer: Vec<u8>,
    attrliste: Attrlist,
}

impl Schwungleser {
    /// Oeffnet das Verzeichnis.
    ///
    /// Schlaegt fehl, wenn der Pfad nicht existiert, nicht lesbar ist oder kein
    /// Verzeichnis benennt.
    pub fn oeffnen(pfad: &Path) -> io::Result<Self> {
        let verzeichnis = File::open(pfad)?;
        if !verzeichnis.metadata()?.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("{} ist kein Verzeichnis", pfad.display()),
            ));
        }
        Ok(Self {
            verzeichnis,
            puffer: vec![0u8; PUFFERGROESSE],
            attrliste: Attrlist {
                bitmapcount: ATTR_BIT_MAP_COUNT,
                reserved: 0,
                commonattr: ATTR_CMN_RETURNED_ATTRS
                    | ATTR_CMN_NAME
                    | ATTR_CMN_OBJTYPE
                    | ATTR_CMN_MODTIME
                    | ATTR_CMN_FLAGS,
                volattr: 0,
                dirattr: 0,
                fileattr: ATTR_FILE_DATALENGTH,
                forkattr: 0,
            },
        })
    }

    /// Holt den naechsten Schwung und ruft `aufnehmen` fuer jeden Eintrag, den
    /// der Zerleger versteht.
    ///
    /// Der Rueckgabewert ist die Zahl der Eintraege, die der Kern geliefert
    /// hat, nicht die Zahl der aufgenommenen. Nur so bleibt `Ok(0)` eindeutig
    /// das Ende des Verzeichnisses. `.` und `..` liefert `getattrlistbulk`
    /// nicht.
    pub fn naechster_schwung<F>(&mut self, mut aufnehmen: F) -> io::Result<usize>
    where
        F: FnMut(RohEintrag<'_>),
    {
        let fd = self.verzeichnis.as_raw_fd();
        // SICHERHEIT: `fd` ist ein offener Deskriptor auf ein Verzeichnis und
        // lebt so lange wie `self`. `attrliste` ist eine gueltige
        // `struct attrlist` mit `bitmapcount = ATTR_BIT_MAP_COUNT`. Puffer und
        // Laengenangabe stammen aus demselben `Vec`, sind also konsistent. Der
        // Kern schreibt hoechstens `attr_buf_size` Bytes.
        let geliefert = unsafe {
            getattrlistbulk(
                fd,
                &raw mut self.attrliste,
                self.puffer.as_mut_ptr().cast::<c_void>(),
                self.puffer.len(),
                0,
            )
        };
        if geliefert < 0 {
            return Err(io::Error::last_os_error());
        }
        let geliefert = geliefert as usize;
        if geliefert == 0 {
            return Ok(0);
        }

        let mut versatz = 0usize;
        for _ in 0..geliefert {
            let Some(laenge) = lies_u32(&self.puffer, versatz) else {
                break;
            };
            let laenge = laenge as usize;
            let Some(ende) = versatz.checked_add(laenge) else {
                break;
            };
            if laenge < 4 || ende > self.puffer.len() {
                break;
            }
            if let Some(eintrag) = satz_zerlegen(&self.puffer[versatz..ende]) {
                aufnehmen(eintrag);
            }
            versatz = ende;
        }
        Ok(geliefert)
    }
}

/// `ENFILE` aus `sys/errno.h`: die systemweite Deskriptortabelle ist voll.
const ENFILE: i32 = 23;

/// `EMFILE` aus `sys/errno.h`: dieser Prozess hat keinen Deskriptor mehr frei.
const EMFILE: i32 = 24;

/// Ob ein Fehlschlag beim Oeffnen **ueber den Prozess** spricht und nicht ueber
/// den Pfad.
///
/// `ENOENT`, `EACCES` und `ENOTDIR` sind Aussagen ueber den Pfad: er ist nicht
/// da, er gehoert einem anderen, er ist keine Ordnung. Wer sie bekommt, weiss
/// etwas ueber den Pfad und darf danach entscheiden. [`EMFILE`] und [`ENFILE`]
/// sagen dagegen nichts ueber den Pfad, sondern ueber die Deskriptortabelle des
/// Prozesses oder des Systems; derselbe Aufruf auf denselben Pfad kann eine
/// Sekunde spaeter gelingen.
///
/// **Wer beide Sorten zusammenzieht, macht aus einem Zustand seines eigenen
/// Prozesses eine Aussage ueber eine fremde Ordnung.** Genau das hat der
/// Durchlauf getan, bis der Defekt `260815-0211` es nachgestellt hat: unter
/// einer knappen Grenze fiel ein Ordner mit einem Treffer darunter still aus
/// der Liste. Die Aufrufer, die die Unterscheidung brauchen, sind zwei:
/// [`crate::verzeichnis::durchlauf`], der einen Ordner sonst still aus der
/// Liste fallen liesse, und seit der Runde 11
/// [`crate::text::datei::bis_zur_grenze_lesen`], das den Mangel als eigenes
/// [`Lesehindernis`](crate::text::datei::Lesehindernis) herausgibt, statt ihn
/// unter die uebrigen Lesefehler zu ziehen. Der Verzeichnisleser braucht sie
/// nicht, weil ein Lesevorgang seinen einen Ordner ohnehin abbricht und den
/// Fehler meldet.
///
/// Eine dritte Sorte gibt es nicht, und das ist keine Verkuerzung: gefragt ist
/// allein, ob der Fehlschlag am Vorrat an Deskriptoren liegt, und darauf gibt
/// es zwei Antworten.
#[must_use]
pub fn ist_deskriptormangel(fehler: &io::Error) -> bool {
    matches!(fehler.raw_os_error(), Some(EMFILE | ENFILE))
}

/// Zerlegt einen einzelnen Satz aus dem Antwortpuffer.
///
/// Liefert `None`, wenn der Satz keinen Namen traegt. Das trifft die Eintraege,
/// zu denen der Kern einen Fehler gemeldet hat; sie werden uebergangen, statt
/// den ganzen Schwung scheitern zu lassen.
fn satz_zerlegen(satz: &[u8]) -> Option<RohEintrag<'_>> {
    // Das Laengenfeld ist bereits ausgewertet.
    let mut stelle = 4usize;

    // `attribute_set_t`: fuenf Gruppen zu je vier Bytes.
    let gemeinsam = lies_u32(satz, stelle)?;
    let dateiseitig = lies_u32(satz, stelle + 12)?;
    stelle += 20;

    let name = if gemeinsam & ATTR_CMN_NAME != 0 {
        // `attrreference_t`: Versatz relativ zur Referenz selbst, dann Laenge
        // einschliesslich des abschliessenden Nullbytes.
        let daten_versatz = lies_i32(satz, stelle)?;
        let daten_laenge = lies_u32(satz, stelle + 4)? as usize;
        let beginn = stelle.checked_add_signed(daten_versatz as isize)?;
        let roh = satz.get(beginn..beginn.checked_add(daten_laenge)?)?;
        let ohne_null = roh.strip_suffix(&[0u8]).unwrap_or(roh);
        stelle += 8;
        String::from_utf8_lossy(ohne_null)
    } else {
        return None;
    };

    let typ = if gemeinsam & ATTR_CMN_OBJTYPE != 0 {
        let roh = lies_u32(satz, stelle)?;
        stelle += 4;
        typ_aus_objtype(roh)
    } else {
        Typ::Datei
    };

    let geaendert = if gemeinsam & ATTR_CMN_MODTIME != 0 {
        // `struct timespec`: zwei 64-Bit-Werte.
        let sekunden = lies_i64(satz, stelle)?;
        let nanosekunden = lies_i64(satz, stelle + 8)?;
        stelle += 16;
        zu_systemzeit(sekunden, nanosekunden)
    } else {
        UNIX_EPOCH
    };

    let kennzeichen = if gemeinsam & ATTR_CMN_FLAGS != 0 {
        let roh = lies_u32(satz, stelle)?;
        stelle += 4;
        roh
    } else {
        0
    };

    let groesse = if dateiseitig & ATTR_FILE_DATALENGTH != 0 {
        let roh = lies_i64(satz, stelle)?;
        stelle += 8;
        roh.max(0) as u64
    } else {
        0
    };

    debug_assert!(stelle <= satz.len(), "Zerleger ist ueber den Satz hinaus");

    Some(RohEintrag {
        name,
        typ,
        groesse,
        geaendert,
        systemseitig_versteckt: kennzeichen & UF_HIDDEN != 0,
    })
}

fn typ_aus_objtype(roh: u32) -> Typ {
    match roh {
        VDIR => Typ::Ordner,
        VLNK => Typ::Verknuepfung,
        _ => Typ::Datei,
    }
}

/// Rechnet eine `struct timespec` in eine [`SystemTime`] um.
///
/// Zeitpunkte vor 1970 sind moeglich (negative Sekunden). Laeuft die Rechnung
/// ueber, liefert die Funktion den Nullpunkt, statt in Panik zu geraten.
fn zu_systemzeit(sekunden: i64, nanosekunden: i64) -> SystemTime {
    let nanosekunden = nanosekunden.clamp(0, 999_999_999) as u32;
    if sekunden >= 0 {
        UNIX_EPOCH
            .checked_add(Duration::new(sekunden as u64, nanosekunden))
            .unwrap_or(UNIX_EPOCH)
    } else {
        UNIX_EPOCH
            .checked_sub(Duration::new(sekunden.unsigned_abs(), 0))
            .and_then(|zeit| zeit.checked_add(Duration::new(0, nanosekunden)))
            .unwrap_or(UNIX_EPOCH)
    }
}

fn lies_u32(satz: &[u8], stelle: usize) -> Option<u32> {
    let stueck = satz.get(stelle..stelle.checked_add(4)?)?;
    Some(u32::from_ne_bytes(stueck.try_into().ok()?))
}

fn lies_i32(satz: &[u8], stelle: usize) -> Option<i32> {
    lies_u32(satz, stelle).map(|wert| wert as i32)
}

fn lies_i64(satz: &[u8], stelle: usize) -> Option<i64> {
    let stueck = satz.get(stelle..stelle.checked_add(8)?)?;
    Some(i64::from_ne_bytes(stueck.try_into().ok()?))
}

// ---------------------------------------------------------------------------
// copyfile(3): eine Datei uebertragen, mit Fortschritt und Abbruch
// ---------------------------------------------------------------------------

/// `COPYFILE_ALL` aus `copyfile.h`: Daten, Rechte, Zeiten, ACL und erweiterte
/// Attribute. Das ist die Zusage aus C4, dass eine Kopie mit allem ankommt,
/// was der Finder auch mitnimmt.
const COPYFILE_ALL: u32 = 0x0000_000F;

/// `COPYFILE_CLONE`: auf demselben APFS-Datentraeger einen Klon anlegen statt
/// die Bytes zu kopieren. Ein bester Versuch; wo Klonen nicht geht, kopiert
/// `copyfile(3)` von selbst die Bytes.
///
/// Das Kennzeichen schliesst `COPYFILE_EXCL` ein: ein vorhandenes Ziel laesst
/// den Aufruf scheitern. Genau das ist gewollt, denn ueber ein vorhandenes Ziel
/// entscheidet die Konfliktregel und nicht `copyfile(3)`.
const COPYFILE_CLONE: u32 = 0x0100_0000;

const COPYFILE_STATE_STATUS_CB: u32 = 6;
const COPYFILE_STATE_STATUS_CTX: u32 = 7;
const COPYFILE_STATE_COPIED: u32 = 8;
const COPYFILE_STATE_WAS_CLONED: u32 = 10;

/// `what`-Werte des Statusrueckrufs, soweit hier gebraucht.
const COPYFILE_COPY_DATA: c_int = 4;

/// `stage`-Werte des Statusrueckrufs.
const COPYFILE_FINISH: c_int = 2;
const COPYFILE_PROGRESS: c_int = 4;

/// Rueckgabewerte des Statusrueckrufs.
const COPYFILE_CONTINUE: c_int = 0;
const COPYFILE_QUIT: c_int = 2;

/// `ECANCELED` aus `sys/errno.h`. `copyfile(3)` meldet damit, dass der eigene
/// Rueckruf `COPYFILE_QUIT` geliefert hat.
const ECANCELED: i32 = 89;

/// `EXDEV` aus `sys/errno.h`. `rename(2)` meldet damit, dass Quelle und Ziel
/// auf verschiedenen Datentraegern liegen.
pub const EXDEV: i32 = 18;

/// `RENAME_EXCL` aus `stdio.h`: scheitert mit `EEXIST`, statt ein vorhandenes
/// Ziel zu ueberschreiben.
const RENAME_EXCL: c_uint = 0x0000_0004;

unsafe extern "C" {
    /// `int copyfile(const char *, const char *, copyfile_state_t, copyfile_flags_t)`
    fn copyfile(
        von: *const c_char,
        nach: *const c_char,
        zustand: *mut c_void,
        kennzeichen: u32,
    ) -> c_int;

    /// `copyfile_state_t copyfile_state_alloc(void)`
    fn copyfile_state_alloc() -> *mut c_void;

    /// `int copyfile_state_free(copyfile_state_t)`
    fn copyfile_state_free(zustand: *mut c_void) -> c_int;

    /// `int copyfile_state_set(copyfile_state_t, uint32_t, const void *)`
    fn copyfile_state_set(zustand: *mut c_void, kennzeichen: u32, wert: *const c_void) -> c_int;

    /// `int copyfile_state_get(copyfile_state_t, uint32_t, void *)`
    fn copyfile_state_get(zustand: *mut c_void, kennzeichen: u32, ziel: *mut c_void) -> c_int;

    /// `int renamex_np(const char *, const char *, unsigned int)`
    fn renamex_np(von: *const c_char, nach: *const c_char, kennzeichen: c_uint) -> c_int;
}

/// Der Typ des Statusrueckrufs, `copyfile_callback_t` aus `copyfile.h`.
type Statusrueckruf = extern "C" fn(
    was: c_int,
    stufe: c_int,
    zustand: *mut c_void,
    quelle: *const c_char,
    ziel: *const c_char,
    kontext: *mut c_void,
) -> c_int;

/// Was der Fortschrittsrueckruf dem laufenden Kopiervorgang antwortet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weiter {
    /// Weitermachen.
    Weitermachen,
    /// Abbrechen. `copyfile(3)` bricht mitten in der Datei ab.
    Abbrechen,
}

/// Wie eine einzelne Datei uebertragen wird.
///
/// Die Oberflaeche waehlt immer [`Uebertragungsart::KlonenWennMoeglich`]. Die
/// zweite Variante ist keine Einstellung fuer den Nutzer, sondern die
/// Beschreibung dessen, was `copyfile(3)` ohnehin tut, sobald Klonen nicht
/// geht: ueber Datentraegergrenzen hinweg, auf einem Dateisystem ohne
/// Klonunterstuetzung und auf einem Netzlaufwerk werden die Bytes kopiert.
/// Benennbar ist sie, weil dieser Weg der einzige ist, auf dem sich ein
/// Abbruch **innerhalb** einer Datei ueberhaupt beobachten laesst: ein Klon ist
/// fertig, bevor ein Abbruch ihn erreichen koennte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Uebertragungsart {
    /// `COPYFILE_ALL | COPYFILE_CLONE`.
    #[default]
    KlonenWennMoeglich,
    /// `COPYFILE_ALL`. Immer die Bytes.
    ImmerBytes,
}

/// Was ein Kopiervorgang hinterlassen hat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kopierergebnis {
    /// Wie viele Bytes uebertragen wurden. Nach einem Klon 0, weil keine Bytes
    /// geflossen sind.
    pub bytes: u64,
    /// Wahr, wenn der Rueckruf abgebrochen hat.
    pub abgebrochen: bool,
    /// Wahr, wenn das Dateisystem geklont hat, statt zu kopieren.
    pub geklont: bool,
}

/// Was der Statusrueckruf mit sich fuehrt.
struct Kopierkontext<'a> {
    bytes: u64,
    melden: &'a mut dyn FnMut(u64) -> Weiter,
}

/// Der Statusrueckruf, den `copyfile(3)` waehrend der Uebertragung ruft.
///
/// Er darf nicht in Panik geraten: ein Abwickeln ueber die C-Grenze hinweg
/// waere undefiniert. Deshalb steht hier nichts, was scheitern koennte, und der
/// Rueckruf des Aufrufers bekommt allein eine Zahl zu sehen.
extern "C" fn statusrueckruf(
    was: c_int,
    stufe: c_int,
    zustand: *mut c_void,
    _quelle: *const c_char,
    _ziel: *const c_char,
    kontext: *mut c_void,
) -> c_int {
    if kontext.is_null() {
        return COPYFILE_CONTINUE;
    }
    // SICHERHEIT: `kontext` ist der Zeiger, den `datei_kopieren` ueber
    // `COPYFILE_STATE_STATUS_CTX` gesetzt hat. Er zeigt auf einen
    // `Kopierkontext`, der die ganze Laufzeit von `copyfile(3)` ueberlebt, weil
    // er als lokale Bindung in `datei_kopieren` steht. `copyfile(3)` ruft den
    // Rueckruf auf demselben Faden und niemals nebenlaeufig, es gibt also
    // keinen zweiten `&mut` auf denselben Wert.
    let kontext = unsafe { &mut *kontext.cast::<Kopierkontext<'_>>() };

    if was == COPYFILE_COPY_DATA && (stufe == COPYFILE_PROGRESS || stufe == COPYFILE_FINISH) {
        let mut kopiert: i64 = 0;
        // SICHERHEIT: `zustand` ist der Zustand, den `copyfile(3)` uns selbst
        // hereinreicht. `COPYFILE_STATE_COPIED` schreibt genau einen `off_t`,
        // und `kopiert` ist ein `i64` derselben Groesse.
        let gelesen = unsafe {
            copyfile_state_get(zustand, COPYFILE_STATE_COPIED, (&raw mut kopiert).cast())
        };
        if gelesen == 0 {
            kontext.bytes = kopiert.max(0) as u64;
        }
    }

    match (kontext.melden)(kontext.bytes) {
        Weiter::Weitermachen => COPYFILE_CONTINUE,
        Weiter::Abbrechen => COPYFILE_QUIT,
    }
}

/// Kopiert eine einzelne Datei ueber `copyfile(3)`.
///
/// `melden` bekommt waehrend der Uebertragung die Zahl der bisher kopierten
/// Bytes und entscheidet mit seinem Rueckgabewert, ob weitergemacht wird. Bei
/// einem Klon wird `melden` nicht gerufen: er ist fertig, bevor es etwas zu
/// melden gaebe.
///
/// Ein vorhandenes Ziel laesst den Aufruf scheitern. Ueber ein vorhandenes Ziel
/// entscheidet die Konfliktregel, nicht diese Funktion.
pub fn datei_kopieren(
    quelle: &Path,
    ziel: &Path,
    art: Uebertragungsart,
    melden: &mut dyn FnMut(u64) -> Weiter,
) -> io::Result<Kopierergebnis> {
    let quelle_c = als_c_pfad(quelle)?;
    let ziel_c = als_c_pfad(ziel)?;
    let mut kontext = Kopierkontext { bytes: 0, melden };

    // SICHERHEIT: `copyfile_state_alloc` nimmt keine Argumente und liefert
    // entweder einen gueltigen Zustand oder einen Nullzeiger.
    let zustand = unsafe { copyfile_state_alloc() };
    if zustand.is_null() {
        return Err(io::Error::last_os_error());
    }

    let ergebnis = mit_zustand_kopieren(zustand, &quelle_c, &ziel_c, art, &mut kontext);

    // SICHERHEIT: `zustand` stammt aus `copyfile_state_alloc`, ist nicht null
    // und wird hier genau einmal freigegeben.
    unsafe { copyfile_state_free(zustand) };
    ergebnis
}

/// Der Kern von [`datei_kopieren`], damit der Zustand auf jedem Weg
/// freigegeben wird.
fn mit_zustand_kopieren(
    zustand: *mut c_void,
    quelle: &CString,
    ziel: &CString,
    art: Uebertragungsart,
    kontext: &mut Kopierkontext<'_>,
) -> io::Result<Kopierergebnis> {
    let rueckruf: Statusrueckruf = statusrueckruf;
    // SICHERHEIT: beide Aufrufe setzen einen Zeigerwert in den eigenen Zustand.
    // `COPYFILE_STATE_STATUS_CB` und `COPYFILE_STATE_STATUS_CTX` uebernehmen
    // den Zeiger unveraendert, sie lesen ihn nicht. Der Rueckruf ist eine
    // `extern "C"`-Funktion mit der Signatur aus `copyfile.h`, und `kontext`
    // lebt laenger als der Aufruf von `copyfile(3)` weiter unten.
    unsafe {
        if copyfile_state_set(
            zustand,
            COPYFILE_STATE_STATUS_CB,
            (rueckruf as *const ()).cast::<c_void>(),
        ) != 0
        {
            return Err(io::Error::last_os_error());
        }
        if copyfile_state_set(
            zustand,
            COPYFILE_STATE_STATUS_CTX,
            std::ptr::from_mut(kontext).cast::<c_void>(),
        ) != 0
        {
            return Err(io::Error::last_os_error());
        }
    }

    let kennzeichen = match art {
        Uebertragungsart::KlonenWennMoeglich => COPYFILE_ALL | COPYFILE_CLONE,
        Uebertragungsart::ImmerBytes => COPYFILE_ALL,
    };

    // SICHERHEIT: beide Pfade sind nullterminiert und leben bis zum Ende des
    // Aufrufs. `zustand` ist gueltig und traegt Rueckruf und Kontext.
    let rueck = unsafe { copyfile(quelle.as_ptr(), ziel.as_ptr(), zustand, kennzeichen) };

    if rueck != 0 {
        let fehler = io::Error::last_os_error();
        if fehler.raw_os_error() == Some(ECANCELED) {
            return Ok(Kopierergebnis {
                bytes: kontext.bytes,
                abgebrochen: true,
                geklont: false,
            });
        }
        return Err(fehler);
    }

    Ok(Kopierergebnis {
        bytes: kontext.bytes,
        abgebrochen: false,
        geklont: wurde_geklont(zustand),
    })
}

/// Fragt den Zustand, ob das Dateisystem geklont statt kopiert hat.
///
/// Der Puffer ist grosszuegiger als das eine Byte, das `copyfile(3)` fuer
/// seinen `bool` schreibt. Ein zu kleiner Puffer waere ein Ueberlauf, ein zu
/// grosser kostet sieben Bytes auf dem Stapel.
fn wurde_geklont(zustand: *mut c_void) -> bool {
    let mut antwort = [0u8; 8];
    // SICHERHEIT: `zustand` ist gueltig, und `COPYFILE_STATE_WAS_CLONED`
    // schreibt einen `bool`, also ein Byte, in den uebergebenen Puffer.
    let gelesen = unsafe {
        copyfile_state_get(
            zustand,
            COPYFILE_STATE_WAS_CLONED,
            antwort.as_mut_ptr().cast::<c_void>(),
        )
    };
    gelesen == 0 && antwort[0] != 0
}

// ---------------------------------------------------------------------------
// renamex_np(2): verschieben innerhalb eines Datentraegers
// ---------------------------------------------------------------------------

/// Verschiebt oder benennt um, innerhalb eines Datentraegers, ueber
/// `rename(2)`.
///
/// Der Aufruf geht ueber `renamex_np(2)`, weil allein diese Fassung
/// `RENAME_EXCL` kennt: mit `nur_wenn_frei` scheitert sie mit `EEXIST`, statt
/// ein vorhandenes Ziel zu ueberschreiben. `std::fs::rename` kann das nicht
/// ausdruecken, und ein Blick auf das Ziel vorweg waere ein Zeitfenster, in dem
/// jemand anderes die Datei anlegt. Mit `nur_wenn_frei = false` ist der Aufruf
/// das gewoehnliche `rename(2)`.
///
/// Liegen Quelle und Ziel auf verschiedenen Datentraegern, meldet der Kern
/// [`EXDEV`]; der Aufrufer faellt dann auf Kopieren mit anschliessendem
/// Loeschen zurueck. Die Zahl der Systemaufrufe haengt nicht an der Groesse der
/// Datei: es ist genau einer, ob die Datei ein Byte oder ein Gigabyte traegt.
pub fn im_datentraeger_verschieben(alt: &Path, neu: &Path, nur_wenn_frei: bool) -> io::Result<()> {
    let alt_c = als_c_pfad(alt)?;
    let neu_c = als_c_pfad(neu)?;
    let kennzeichen = if nur_wenn_frei { RENAME_EXCL } else { 0 };
    // SICHERHEIT: beide Pfade sind nullterminiert und leben bis zum Ende des
    // Aufrufs. `renamex_np` nimmt nur diese drei Werte entgegen.
    let rueck = unsafe { renamex_np(alt_c.as_ptr(), neu_c.as_ptr(), kennzeichen) };
    if rueck != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// fcntl(2): oeffnen, ohne auf einen Schreiber zu warten
// ---------------------------------------------------------------------------

/// `O_NONBLOCK` aus `sys/fcntl.h`: das Oeffnen wartet nicht.
const O_NONBLOCK: c_int = 0x0000_0004;

/// `F_GETFL` aus `sys/fcntl.h`: die Kennzeichen des Deskriptors lesen.
const F_GETFL: c_int = 3;

/// `F_SETFL` aus `sys/fcntl.h`: die Kennzeichen des Deskriptors setzen.
const F_SETFL: c_int = 4;

unsafe extern "C" {
    /// `int fcntl(int fildes, int cmd, ...)`
    ///
    /// **Variadisch deklariert, so wie der Header es tut**, und das ist keine
    /// Formsache: auf arm64 uebergibt Apples ABI variadische Argumente auf dem
    /// Stapel, feste dagegen in Registern. Eine Deklaration mit drei festen
    /// Argumenten waere derselbe Aufruf mit dem falschen Argumentweg, und der
    /// Uebersetzer hat keine Gelegenheit, das zu bemerken.
    fn fcntl(fd: c_int, befehl: c_int, ...) -> c_int;
}

/// Oeffnet einen Pfad zum Lesen, **ohne bei einer benannten Roehre zu warten**.
///
/// `File::open` auf eine benannte Roehre ohne Schreiber haengt, bis jemand
/// hineinschreibt. Wer deshalb erst am **Namen** prueft, ob der Pfad eine
/// gewoehnliche Datei ist, und danach denselben Namen oeffnet, hat zwischen
/// beiden Aufrufen ein Fenster: zeigt der Pfad in dieser Spanne auf eine Roehre,
/// haengt das Oeffnen doch, und der Aufrufer bekommt keine Meldung, sondern
/// nichts.
///
/// Diese Funktion dreht die Reihenfolge um. Sie oeffnet zuerst, und zwar mit
/// `O_NONBLOCK`, sodass der Aufruf auch an einer Roehre zurueckkommt; der
/// Aufrufer fragt danach `fstat` am **Deskriptor** und entscheidet damit ueber
/// genau das Ding, das er in der Hand haelt. Das Fenster verschwindet nicht,
/// weil es bewacht wird, sondern weil nur noch **ein** Aufruf den Namen
/// anfasst.
///
/// # `O_NONBLOCK` wird noch hier abgeschaltet und nicht spaeter
///
/// Die Gefahr steht allein am `open`; ein `fcntl(F_SETFL)` haelt nichts an, auch
/// nicht an einer Roehre ohne Schreiber. Abgeschaltet gehoert das Kennzeichen,
/// weil POSIX die Wirkung von `O_NONBLOCK` auf gewoehnliche Dateien offen
/// laesst: `speculation:` auf einem Netzlaufwerk koennte ein Lesen sonst mit
/// `EAGAIN` scheitern, und ein Dateimanager sieht Netzlaufwerke. Ungemessen ist,
/// ob macOS auf einer SMB- oder FUSE-Flaeche wirklich `EAGAIN` liefert; das
/// Abschalten kostet einen Systemaufruf und macht die Frage gegenstandslos,
/// statt sie zu beantworten.
///
/// # Was der Deskriptor traegt
///
/// Geoeffnet wird **ohne** `O_NOFOLLOW`, also so wie `File::open`: bei einer
/// Verknuepfung steht am Deskriptor ihr Ziel, und `metadata()` darauf sagt
/// dasselbe wie `std::fs::metadata` auf den Pfad. Ein Ordner laesst sich auf
/// macOS zum Lesen oeffnen und faellt deshalb nicht schon hier heraus, sondern
/// erst am `fstat` des Aufrufers; das ist gewollt, denn was ein gueltiges Ziel
/// ist, entscheidet der Aufrufer und nicht diese Stelle.
///
/// # Mehrere Aufrufer, und die Zielpruefung bleibt bei jedem von ihnen
///
/// Gerufen wird die Funktion von [`crate::text::datei::lesen`], dem Eingang des
/// Editors, von [`crate::text::datei::bis_zur_grenze_lesen`], der Huelle mit der
/// uebergebenen Grenze, und seit der Runde 17 von den zwei Archivwegen
/// [`crate::operation::zippen`] und [`crate::operation::entpacken`]. **Alle
/// liegen in `krk-core`**; bis zur Runde 11 stand die zweite als private Fassung
/// in `krk-ui`s `vorschaumodell.rs` und hatte deshalb keinen Doku-Verweis.
/// Aufrufer der Huelle ist heute die Vorschau, mit ihren zwei Grenzen.
///
/// **Gemeinsam ist den zwei Textwegen der Ablauf**: hier oeffnen, `fstat` am
/// Deskriptor fragen, alles abweisen, was `is_file()` nicht bejaht, die Groesse
/// gegen eine Grenze halten, erst danach lesen. **Verschieden sind die Antwort
/// und die Grenze.** Der Editor weist mit [`crate::text::Abweisung::KeinGueltigesZiel`]
/// ab und nennt dem Nutzer den Grund; die Vorschau faellt auf ihre
/// Metadatenanzeige zurueck, die Groesse, Rechte und Datum zeigt. Der Editor
/// haelt dabei `EDITORGRENZE`, die Vorschau `TEXTGRENZE` oder `BILDGRENZE`, je
/// Endung des Pfades.
///
/// **Die zwei Archivwege fragen ihrerseits anders, und sie sind der beste Beleg
/// dafuer, dass die Frage hier nicht hingehoert.** Das Packen fragt `metadata()`
/// am Deskriptor wie die zwei Textwege und laesst alles aus, was `is_file()`
/// nicht bejaht — die Antwort ist aber weder eine Abweisung noch ein Rueckfall,
/// sondern eine Zeile in der Abschlussliste, und eine Groessengrenze kennt es
/// gar nicht. (Bis zum 260825 stand hier, das Packen kenne den Typ seiner Quelle
/// schon vor dem Oeffnen. Das war falsch: `typ_und_groesse` legt jeden Eintrag,
/// der weder Ordner noch Verknuepfung ist, in `Typ::Datei`, und dieses Fach
/// traegt auch Roehren, Geraete und Sockel. Der Defekt ist `260825-0942`.) Das
/// Entpacken fragt gar nicht nach dem Typ, sondern reicht den Deskriptor an
/// `ZipArchive::new` weiter und laesst die Kiste antworten; ein Ordner scheitert
/// dort an `EISDIR` und kommt mit ihrem Wortlaut in die Abschlussliste. Keine
/// der vier Antworten kennt diese Huelle.
///
/// **Ein weiterer Aufrufer verschiebt die Pruefung damit nicht hierher, sondern
/// begruendet sie jedes Mal besser.** Eine Typpruefung in dieser Huelle muesste
/// eine der Antworten waehlen, und keine der Grenzen kennt sie. Sie oeffnet und
/// nimmt `O_NONBLOCK` wieder ab; was ein gueltiges Ziel ist, bleibt die Frage
/// des Aufrufers. Wer sie auf den Editorfall zuschneidet, nimmt der Vorschau den
/// Schutz, den sie seit dem 260810 hat.
///
/// Der Defekt, der die Funktion verlangt hat, ist `260809-1652`; der zweite
/// Aufrufer ist mit `260810-1247` dazugekommen, der dritte und der vierte mit
/// der Runde 17.
pub fn ohne_warten_oeffnen(pfad: &Path) -> io::Result<File> {
    let datei = OpenOptions::new()
        .read(true)
        .custom_flags(O_NONBLOCK)
        .open(pfad)?;
    blockierend_stellen(&datei)?;
    Ok(datei)
}

/// Nimmt `O_NONBLOCK` von einem offenen Deskriptor wieder ab.
///
/// Gelesen und geschrieben werden die Kennzeichen und nicht ein einzelnes Bit:
/// `F_SETFL` ersetzt den ganzen Satz, und wer die anderen Kennzeichen nicht
/// vorher liest, loescht sie mit.
fn blockierend_stellen(datei: &File) -> io::Result<()> {
    let fd = datei.as_raw_fd();
    // SICHERHEIT: `fd` gehoert dem `File` des Aufrufers und ist offen, solange
    // dessen Bindung lebt. `F_GETFL` liest allein die Kennzeichen des
    // Deskriptors, fasst keinen Speicher an und nimmt kein weiteres Argument;
    // die 0 steht da, weil der Aufruf variadisch ist.
    let kennzeichen = unsafe { fcntl(fd, F_GETFL, 0) };
    if kennzeichen < 0 {
        return Err(io::Error::last_os_error());
    }
    if kennzeichen & O_NONBLOCK == 0 {
        return Ok(());
    }
    // SICHERHEIT: derselbe offene Deskriptor. `F_SETFL` nimmt genau ein `int`
    // mit den neuen Kennzeichen und fasst ebenfalls keinen Speicher an.
    if unsafe { fcntl(fd, F_SETFL, kennzeichen & !O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// flock(2): den Ablageordner gegen einen zweiten Prozess halten
// ---------------------------------------------------------------------------

/// `LOCK_EX` aus `sys/file.h`: eine Sperre mit genau einem Halter.
const LOCK_EX: c_int = 2;

/// `LOCK_NB` aus `sys/file.h`: nicht warten, sondern sofort zurueckkehren.
const LOCK_NB: c_int = 4;

/// `LOCK_UN` aus `sys/file.h`: die gehaltene Sperre abgeben.
const LOCK_UN: c_int = 8;

/// `EWOULDBLOCK` aus `sys/errno.h`, auf macOS derselbe Wert wie `EAGAIN`.
///
/// Der eine erwartete Fehlschlag von `flock` mit [`LOCK_NB`]: die Sperre haelt
/// ein anderer. Er ist kein Fehler, sondern eine Antwort, und
/// [`sperre_versuchen`] gibt ihm mit [`Sperrversuch::Belegt`] einen Namen.
const EWOULDBLOCK: i32 = 35;

unsafe extern "C" {
    /// `int flock(int fd, int operation)`
    ///
    /// **Nicht variadisch, anders als [`fcntl`] darueber**, und das ist keine
    /// Nachlaessigkeit, sondern der Header: `flock` nimmt genau zwei feste
    /// Argumente. Die Ueberlegung ist dieselbe wie dort — die Deklaration bildet
    /// den Header ab und nicht die Bequemlichkeit des Aufrufers, denn auf arm64
    /// laufen feste und variadische Argumente ueber verschiedene Wege.
    fn flock(fd: c_int, operation: c_int) -> c_int;
}

/// Was ein Sperrversuch ohne Warten ergeben hat.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig, und der Grund ist
/// derselbe wie ueberall in diesem Baum: eine dritte Antwort haelt den Bau an,
/// statt sich in einen bestehenden Zweig zu mogeln.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "eine nicht genommene Sperre schuetzt nichts"]
pub enum Sperrversuch {
    /// Die Sperre gehoert jetzt diesem Deskriptor.
    Genommen,
    /// Ein anderer Halter hat sie; es ist nichts genommen worden.
    Belegt,
}

/// Nimmt die Sperre auf diesen Deskriptor und **wartet**, bis sie frei ist.
///
/// Der Weg der Schreibsperre: ein Durchgang aus Lesen, Aendern und Schreiben
/// soll nicht abgewiesen werden, sondern warten, bis er an der Reihe ist.
///
/// Die Sperre haengt an der **offenen Datei** und nicht am Prozess. Zwei
/// Deskriptoren desselben Prozesses auf dieselbe Datei schliessen einander
/// deshalb genauso aus wie zwei Prozesse; zweimal derselbe Deskriptor dagegen
/// wartet nicht, sondern gibt die Sperre einfach erneut aus. Was daraus fuer die
/// Ablage folgt, steht bei [`crate::ablage::sperre`].
pub fn sperre_nehmen(datei: &File) -> io::Result<()> {
    flock_rufen(datei, LOCK_EX)
}

/// Versucht die Sperre, **ohne zu warten**.
///
/// Der Weg des Sitzungsrechts: wer es nicht bekommt, arbeitet ohne es weiter
/// und darf darueber nicht haengenbleiben. Der erwartete Fehlschlag
/// [`EWOULDBLOCK`] kommt als [`Sperrversuch::Belegt`] zurueck und nicht als
/// Fehler; jeder andere Fehlschlag bleibt einer.
pub fn sperre_versuchen(datei: &File) -> io::Result<Sperrversuch> {
    match flock_rufen(datei, LOCK_EX | LOCK_NB) {
        Ok(()) => Ok(Sperrversuch::Genommen),
        Err(fehler) if fehler.raw_os_error() == Some(EWOULDBLOCK) => Ok(Sperrversuch::Belegt),
        Err(fehler) => Err(fehler),
    }
}

/// Gibt die Sperre auf diesem Deskriptor wieder ab.
///
/// **Der Kern gibt sie auch ohne diesen Aufruf ab**, naemlich sobald der letzte
/// Deskriptor auf die offene Datei geschlossen wird, und das gilt fuer den
/// Absturz genauso wie fuer das gewoehnliche Ende. Diese Funktion braucht es
/// trotzdem: die Schreibsperre lebt kuerzer als ihr Deskriptor, denn die Ablage
/// haelt die Sperrdatei offen und nimmt die Sperre je Durchgang.
pub fn sperre_abgeben(datei: &File) -> io::Result<()> {
    flock_rufen(datei, LOCK_UN)
}

/// Der eine Aufruf ueber die Sprachgrenze, den sich die drei Huellen teilen.
fn flock_rufen(datei: &File, operation: c_int) -> io::Result<()> {
    // SICHERHEIT: `fd` gehoert dem `File` des Aufrufers und ist offen, solange
    // dessen Bindung lebt. `flock` nimmt genau zwei `int` entgegen, fasst
    // keinen Speicher an und gibt einen `int` zurueck.
    if unsafe { flock(datei.as_raw_fd(), operation) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Macht aus einem Pfad eine nullterminierte Zeichenkette fuer die C-Aufrufe.
///
/// Ein Pfad mit einem Nullbyte darin kann im Dateisystem nicht vorkommen; er
/// kommt hier als Fehler zurueck, statt abgeschnitten weitergereicht zu werden.
fn als_c_pfad(pfad: &Path) -> io::Result<CString> {
    CString::new(pfad.as_os_str().as_bytes()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} enthaelt ein Nullbyte", pfad.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attrlist_hat_die_groesse_aus_sys_attr_h() {
        assert_eq!(size_of::<Attrlist>(), 24);
        assert_eq!(align_of::<Attrlist>(), 4);
    }

    #[test]
    fn ein_pfad_mit_nullbyte_kommt_als_fehler_zurueck() {
        let fehler = als_c_pfad(Path::new("kaputt\0name")).expect_err("das darf nicht durchgehen");
        assert_eq!(fehler.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn zeit_vor_dem_nullpunkt_geraet_nicht_in_panik() {
        let zeit = zu_systemzeit(-1, 500);
        assert!(zeit < UNIX_EPOCH);
    }

    #[test]
    fn kurzer_satz_liefert_keinen_eintrag() {
        assert!(satz_zerlegen(&[0u8; 8]).is_none());
    }

    /// Zwei Deskriptoren desselben Prozesses auf dieselbe Datei schliessen
    /// einander aus.
    ///
    /// Die Zusage, auf der beide Sperren der Ablage ruhen: `flock` haengt an der
    /// **offenen Datei** und nicht am Prozess. Waere es umgekehrt, saehe eine
    /// zweite Instanz von KRK die Sperre der ersten zwar, ein Fehlgriff im
    /// eigenen Prozess dagegen nicht, und die Probe dazu waere nicht zu
    /// schreiben, ohne einen zweiten Prozess zu starten.
    ///
    /// Geprueft wird mit [`sperre_versuchen`], weil ein wartendes
    /// [`sperre_nehmen`] die Probe genau hier haengen liesse.
    #[test]
    fn ein_zweiter_deskriptor_auf_dieselbe_datei_bekommt_die_sperre_nicht() {
        let pfad = std::env::temp_dir().join(format!("krk-sys-flock-{}", std::process::id()));
        let erster = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&pfad)
            .expect("die Sperrdatei laesst sich nicht anlegen");
        let zweiter = OpenOptions::new()
            .write(true)
            .open(&pfad)
            .expect("die Sperrdatei laesst sich nicht ein zweites Mal oeffnen");

        assert_eq!(
            sperre_versuchen(&erster).expect("der erste Versuch ist gescheitert"),
            Sperrversuch::Genommen
        );
        assert_eq!(
            sperre_versuchen(&zweiter).expect("der zweite Versuch ist gescheitert"),
            Sperrversuch::Belegt,
            "der zweite Deskriptor hat die Sperre trotz des ersten bekommen"
        );

        sperre_abgeben(&erster).expect("das Abgeben ist gescheitert");
        assert_eq!(
            sperre_versuchen(&zweiter).expect("der dritte Versuch ist gescheitert"),
            Sperrversuch::Genommen,
            "nach dem Abgeben ist die Sperre nicht frei geworden"
        );

        drop(zweiter);
        drop(erster);
        let _ = std::fs::remove_file(&pfad);
    }

    /// Nur `EMFILE` und `ENFILE` sprechen ueber den Prozess, jeder andere
    /// Oeffnungsfehler spricht ueber den Pfad.
    ///
    /// Die Trennung traegt C3.15: [`crate::verzeichnis::durchlauf`] laesst
    /// einen Ordner unentschieden, wenn sie ja sagt, und entscheidet ihn
    /// negativ, wenn sie nein sagt. Ein zu weites Ja hielte den Durchlauf bei
    /// einem gewoehnlichen Rechtefehler an; ein zu enges Nein brachte den
    /// Defekt `260815-0211` zurueck.
    ///
    /// Der letzte Fall ist der, den ein Blick auf die Zahlen uebersieht:
    /// [`Schwungleser::oeffnen`] baut den Fehler „kein Verzeichnis" selbst und
    /// gibt ihm gar keine Betriebssystemnummer. `raw_os_error()` liefert dann
    /// `None`, und das darf nicht als Mangel durchgehen.
    #[test]
    fn nur_emfile_und_enfile_gelten_als_deskriptormangel() {
        for kennung in [EMFILE, ENFILE] {
            assert!(
                ist_deskriptormangel(&io::Error::from_raw_os_error(kennung)),
                "errno {kennung} spricht ueber den Vorrat an Deskriptoren und wird nicht erkannt"
            );
        }

        // Die Oeffnungsfehler, die etwas ueber den Pfad sagen, dazu `EINTR` als
        // einer, der weder ueber den Pfad noch ueber den Vorrat spricht.
        for (kennung, name) in [
            (2, "ENOENT"),
            (4, "EINTR"),
            (13, "EACCES"),
            (20, "ENOTDIR"),
            (62, "ELOOP"),
            (63, "ENAMETOOLONG"),
        ] {
            assert!(
                !ist_deskriptormangel(&io::Error::from_raw_os_error(kennung)),
                "{name} spricht nicht ueber den Vorrat an Deskriptoren und gilt trotzdem als Mangel"
            );
        }

        let ohne_nummer = io::Error::new(io::ErrorKind::NotADirectory, "kein Verzeichnis");
        assert!(
            !ist_deskriptormangel(&ohne_nummer),
            "ein selbst gebauter Fehler ohne Betriebssystemnummer gilt als Deskriptormangel"
        );
    }

    /// Der Deskriptor kommt blockierend zurueck, nicht nichtblockierend.
    ///
    /// Das ist die eine Zusage von [`ohne_warten_oeffnen`], die der Aufrufer
    /// nicht selbst nachsehen kann: `O_NONBLOCK` dient dem Oeffnen und darf das
    /// Lesen nicht erreichen. Gefragt wird der Deskriptor selbst und nicht der
    /// Quelltext daneben.
    #[test]
    fn ein_geoeffneter_deskriptor_traegt_o_nonblock_nicht_mehr() {
        let pfad = std::env::temp_dir().join(format!("krk-sys-nonblock-{}", std::process::id()));
        std::fs::write(&pfad, b"eins\n").expect("Pruefdatei laesst sich nicht schreiben");

        let datei = ohne_warten_oeffnen(&pfad).expect("die Pruefdatei gehoert geoeffnet");
        // SICHERHEIT: `datei` lebt bis zum Ende der Probe, der Deskriptor ist
        // also offen. `F_GETFL` liest nur.
        let kennzeichen = unsafe { fcntl(datei.as_raw_fd(), F_GETFL, 0) };
        let _ = std::fs::remove_file(&pfad);

        assert!(kennzeichen >= 0, "F_GETFL ist gescheitert");
        assert_eq!(
            kennzeichen & O_NONBLOCK,
            0,
            "der Deskriptor traegt O_NONBLOCK noch, das Lesen koennte mit EAGAIN scheitern"
        );
    }
}
