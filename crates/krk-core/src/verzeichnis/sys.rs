//! Die Systemschicht des Verzeichnislesers: die Bindung an `getattrlistbulk(2)`.
//!
//! Dies ist das einzige Modul in `krk-core`, das die Regel `deny(unsafe_code)`
//! aus `lib.rs` oeffnet. Die Regel lautet dort `deny` und nicht `forbid`, damit
//! genau diese Oeffnung moeglich ist. Spaeter kommt die Bindung an `copyfile(3)`
//! hierher dazu (Schritt 15 des Plans); ein zweites Modul mit dieser Ausnahme
//! entsteht nicht.
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
use std::ffi::{c_int, c_void};
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attrlist_hat_die_groesse_aus_sys_attr_h() {
        assert_eq!(size_of::<Attrlist>(), 24);
        assert_eq!(align_of::<Attrlist>(), 4);
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
}
