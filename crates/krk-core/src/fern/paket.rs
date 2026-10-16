//! Die Pakete von SFTP v3 als reine Kodierung und Zerlegung.
//!
//! Gesprochen wird die Fassung 3 des Entwurfs
//! `draft-ietf-secsh-filexfer-02`, die Fassung, die OpenSSH spricht und die
//! jeder verbreitete Server anbietet. Ein Paket ist eine Laenge von vier Bytes
//! in Netzordnung, dahinter ein Typbyte und der Rumpf; jede Anfrage ausser
//! `INIT` traegt nach dem Typbyte eine Kennung von vier Bytes, und die Antwort
//! nennt dieselbe Kennung. Ueber diese Kennung ordnet
//! [`super::gespraech`] die Antworten ihren Anfragen zu.
//!
//! **Dieses Modul liest und schreibt keinen Strom ausser in [`paket_lesen`],
//! und es kennt keinen Faden.** [`Paket::kodieren`] liefert die Bytes eines
//! ganzen Pakets samt Laenge, [`Paket::zerlegen`] nimmt den Rumpf ohne Laenge.
//! Beide Richtungen stehen fuer jeden Typ da, auch fuer die, die KRK nur in
//! einer Richtung braucht: so ist der Rundlauf je Paket pruefbar
//! (`jedes_paket_uebersteht_den_rundlauf` in `tests/fern.rs`), und ein
//! Fehler in der einen Richtung faellt an der anderen auf.
//!
//! # Die Grenze von 256 KiB
//!
//! [`paket_lesen`] liest die Laenge und weist jede ueber [`PAKETGRENZE`] ab,
//! **bevor** es Speicher fuer den Rumpf anlegt. Ohne die Grenze legte eine
//! verdorbene oder boeswillige Laenge bis zu 4 GiB an. Die Zahl ist die, die
//! `sftp-server` von OpenSSH selbst als groesstes Paket annimmt
//! (`SFTP_MAX_MSG_LENGTH`); KRK schickt hoechstens Bloecke zu 32 KiB und
//! bekommt hoechstens Antworten dieser Groesse oder Namenslisten von einigen
//! Kilobyte.
//!
//! # Namen sind Bytes
//!
//! Jede Zeichenkette des Protokolls ist eine Laenge und Bytes ohne Zusage
//! einer Kodierung. Pfade, Namen, Handles und Fehlertexte stehen deshalb hier
//! als `Vec<u8>`; ob ein Name UTF-8 ist und was mit einem anderen geschieht,
//! entscheidet der Leser eines Ordners und nicht die Kodierung.

use std::fmt;
use std::io::{self, Read};

/// Das groesste Paket, das [`paket_lesen`] annimmt, gemessen an der
/// Laengenangabe, also Typbyte und Rumpf.
pub const PAKETGRENZE: u32 = 256 * 1024;

/// Paare aus Name und Wert, wie sie `INIT`, `VERSION` und die erweiterten
/// Attribute tragen.
pub type Paare = Vec<(Vec<u8>, Vec<u8>)>;

/// Die Fassung des Protokolls, die KRK spricht.
pub const FASSUNG: u32 = 3;

// Die Typbytes aus Abschnitt 3 des Entwurfs.
const INIT: u8 = 1;
const VERSION: u8 = 2;
const OPEN: u8 = 3;
const CLOSE: u8 = 4;
const READ: u8 = 5;
const WRITE: u8 = 6;
const LSTAT: u8 = 7;
const SETSTAT: u8 = 9;
const OPENDIR: u8 = 11;
const READDIR: u8 = 12;
const REMOVE: u8 = 13;
const MKDIR: u8 = 14;
const RMDIR: u8 = 15;
const REALPATH: u8 = 16;
const STAT: u8 = 17;
const RENAME: u8 = 18;
const READLINK: u8 = 19;
const SYMLINK: u8 = 20;
const STATUS: u8 = 101;
const HANDLE: u8 = 102;
const DATA: u8 = 103;
const NAME: u8 = 104;
const ATTRS: u8 = 105;

/// Die Kennzeichen von `OPEN`, Abschnitt 6.3 des Entwurfs.
pub mod oeffnen {
    /// Zum Lesen.
    pub const READ: u32 = 0x01;
    /// Zum Schreiben.
    pub const WRITE: u32 = 0x02;
    /// Jedes Schreiben haengt an.
    pub const APPEND: u32 = 0x04;
    /// Anlegen, wenn die Datei fehlt.
    pub const CREAT: u32 = 0x08;
    /// Auf Laenge null kuerzen.
    pub const TRUNC: u32 = 0x10;
    /// Mit `CREAT`: scheitern, wenn die Datei schon da ist.
    pub const EXCL: u32 = 0x20;
}

/// Die Statuscodes, Abschnitt 7 des Entwurfs.
pub mod status {
    /// Erfolg.
    pub const OK: u32 = 0;
    /// Ende der Datei oder des Ordners.
    pub const EOF: u32 = 1;
    /// Kein solcher Eintrag.
    pub const NO_SUCH_FILE: u32 = 2;
    /// Keine Berechtigung.
    pub const PERMISSION_DENIED: u32 = 3;
    /// Allgemeiner Fehlschlag, auch fuer "das Ziel ist schon da".
    pub const FAILURE: u32 = 4;
    /// Ein Paket, das der Server nicht versteht.
    pub const BAD_MESSAGE: u32 = 5;
    /// Keine Verbindung.
    pub const NO_CONNECTION: u32 = 6;
    /// Die Verbindung ist verloren.
    pub const CONNECTION_LOST: u32 = 7;
    /// Der Server kennt die Anfrage nicht.
    pub const OP_UNSUPPORTED: u32 = 8;
}

// Die Kennzeichen der Attribute, Abschnitt 5 des Entwurfs.
const ATTR_SIZE: u32 = 0x0000_0001;
const ATTR_UIDGID: u32 = 0x0000_0002;
const ATTR_PERMISSIONS: u32 = 0x0000_0004;
const ATTR_ACMODTIME: u32 = 0x0000_0008;
const ATTR_EXTENDED: u32 = 0x8000_0000;

/// Der Typ aus den Rechtebits, `S_IFMT` aus `<sys/stat.h>`.
pub const S_IFMT: u32 = 0o170_000;
/// Ein Ordner.
pub const S_IFDIR: u32 = 0o040_000;
/// Eine Verknuepfung.
pub const S_IFLNK: u32 = 0o120_000;

/// Die Attribute eines Eintrags. Jedes Feld ist optional, denn der Server
/// schickt nur, was er kennt, und eine Anfrage setzt nur, was sie aendern will.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Attribute {
    /// Die Groesse in Bytes.
    pub groesse: Option<u64>,
    /// Besitzer und Gruppe als Nummern.
    pub besitz: Option<(u32, u32)>,
    /// Die Rechte samt Typbits (`S_IFMT`).
    pub rechte: Option<u32>,
    /// Zugriffs- und Aenderungszeit in Sekunden seit der Unix-Epoche.
    pub zeiten: Option<(u32, u32)>,
    /// Erweiterte Angaben als Paare aus Name und Wert.
    pub erweitert: Paare,
}

impl Attribute {
    /// Die Aenderungszeit, wenn der Server sie geschickt hat.
    #[must_use]
    pub fn geaendert(&self) -> Option<u32> {
        self.zeiten.map(|(_, geaendert)| geaendert)
    }

    /// Ob die Typbits einen Ordner nennen. Ohne Rechteangabe: nein.
    #[must_use]
    pub fn ist_ordner(&self) -> bool {
        self.rechte.is_some_and(|rechte| rechte & S_IFMT == S_IFDIR)
    }

    /// Ob die Typbits eine Verknuepfung nennen. Ohne Rechteangabe: nein.
    #[must_use]
    pub fn ist_verknuepfung(&self) -> bool {
        self.rechte.is_some_and(|rechte| rechte & S_IFMT == S_IFLNK)
    }
}

/// Ein Name aus einer Antwort `NAME`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    /// Der Name, bei `READDIR` ohne Ordner davor.
    pub name: Vec<u8>,
    /// Die Zeile im Stil von `ls -l`, die der Server mitschickt. KRK liest sie
    /// nicht; sie hat kein festes Format.
    pub lang: Vec<u8>,
    /// Die Attribute des Eintrags.
    pub attribute: Attribute,
}

/// Eine Anfrage mit Kennung, also jede ausser `INIT`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Anfrage {
    /// `OPEN`: eine Datei oeffnen, mit den Kennzeichen aus [`oeffnen`].
    Open {
        /// Der Pfad.
        pfad: Vec<u8>,
        /// Die Kennzeichen.
        kennzeichen: u32,
        /// Die Attribute einer neu angelegten Datei.
        attribute: Attribute,
    },
    /// `CLOSE`: ein Handle schliessen.
    Close {
        /// Das Handle.
        handle: Vec<u8>,
    },
    /// `READ`: hoechstens `laenge` Bytes ab `versatz`.
    Read {
        /// Das Handle.
        handle: Vec<u8>,
        /// Der Versatz in der Datei.
        versatz: u64,
        /// Wie viele Bytes hoechstens.
        laenge: u32,
    },
    /// `WRITE`: Bytes ab `versatz`.
    Write {
        /// Das Handle.
        handle: Vec<u8>,
        /// Der Versatz in der Datei.
        versatz: u64,
        /// Die Bytes.
        daten: Vec<u8>,
    },
    /// `LSTAT`: die Attribute, ohne einer Verknuepfung zu folgen.
    Lstat {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `SETSTAT`: Attribute setzen.
    Setstat {
        /// Der Pfad.
        pfad: Vec<u8>,
        /// Die zu setzenden Attribute.
        attribute: Attribute,
    },
    /// `OPENDIR`: einen Ordner zum Lesen oeffnen.
    Opendir {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `READDIR`: die naechsten Namen eines Ordners.
    Readdir {
        /// Das Handle.
        handle: Vec<u8>,
    },
    /// `REMOVE`: eine Datei entfernen.
    Remove {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `MKDIR`: einen Ordner anlegen.
    Mkdir {
        /// Der Pfad.
        pfad: Vec<u8>,
        /// Die Attribute des neuen Ordners.
        attribute: Attribute,
    },
    /// `RMDIR`: einen leeren Ordner entfernen.
    Rmdir {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `REALPATH`: einen Pfad auf dem Server aufloesen.
    Realpath {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `STAT`: die Attribute, einer Verknuepfung folgend.
    Stat {
        /// Der Pfad.
        pfad: Vec<u8>,
    },
    /// `RENAME`: umbenennen; nach dem Entwurf scheitert es an einem
    /// vorhandenen Ziel.
    Rename {
        /// Der alte Pfad.
        alt: Vec<u8>,
        /// Der neue Pfad.
        neu: Vec<u8>,
    },
    /// `READLINK`: das Ziel einer Verknuepfung.
    Readlink {
        /// Der Pfad der Verknuepfung.
        pfad: Vec<u8>,
    },
    /// `SYMLINK`: eine Verknuepfung anlegen. Die zwei Zeichenketten stehen
    /// hier in der Reihenfolge des Pakets und nicht nach ihrer Bedeutung, denn
    /// OpenSSH vertauscht sie gegen den Entwurf; welche Folge gilt, entscheidet
    /// `Gespraech::symlink`.
    Symlink {
        /// Die erste Zeichenkette des Pakets.
        erste: Vec<u8>,
        /// Die zweite Zeichenkette des Pakets.
        zweite: Vec<u8>,
    },
}

/// Eine Antwort mit Kennung, also jede ausser `VERSION`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Antwort {
    /// `STATUS`: Erfolg oder Fehlschlag.
    Status {
        /// Der Code aus [`status`].
        code: u32,
        /// Der Text des Servers, ohne Zusage einer Kodierung.
        meldung: Vec<u8>,
        /// Die Sprachkennung des Texts.
        sprache: Vec<u8>,
    },
    /// `HANDLE`: ein offenes Handle.
    Handle(Vec<u8>),
    /// `DATA`: gelesene Bytes.
    Data(Vec<u8>),
    /// `NAME`: eine Liste von Namen.
    Name(Vec<Name>),
    /// `ATTRS`: Attribute.
    Attrs(Attribute),
}

/// Ein ganzes Paket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Paket {
    /// `INIT`, die erste Nachricht des Clients.
    Init {
        /// Die Fassung, die der Client spricht.
        fassung: u32,
        /// Erweiterungen; KRK schickt keine.
        erweiterungen: Paare,
    },
    /// `VERSION`, die Antwort des Servers auf `INIT`.
    Version {
        /// Die Fassung, die der Server spricht.
        fassung: u32,
        /// Die Erweiterungen, die der Server anbietet, als Name und Wert.
        erweiterungen: Paare,
    },
    /// Eine Anfrage mit ihrer Kennung.
    Anfrage {
        /// Die Kennung, die die Antwort wiederholt.
        kennung: u32,
        /// Die Anfrage.
        anfrage: Anfrage,
    },
    /// Eine Antwort mit der Kennung ihrer Anfrage.
    Antwort {
        /// Die Kennung der Anfrage.
        kennung: u32,
        /// Die Antwort.
        antwort: Antwort,
    },
}

/// Warum ein Paket nicht gelesen oder zerlegt werden konnte.
#[derive(Debug)]
pub enum Paketfehler {
    /// Die Laengenangabe liegt ueber [`PAKETGRENZE`].
    ZuLang(u32),
    /// Die Laengenangabe ist null; es fehlt sogar das Typbyte.
    Leer,
    /// Der Rumpf endet, bevor ein Feld vollstaendig ist.
    ZuKurz,
    /// Nach dem letzten Feld stehen noch Bytes.
    Rest(usize),
    /// Ein Typbyte, das dieses Modul nicht kennt.
    UnbekannterTyp(u8),
    /// Der Strom endet mitten in einem Paket.
    Abgerissen,
    /// Der Strom meldet einen Fehler.
    Lesefehler(io::Error),
}

impl fmt::Display for Paketfehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Paketfehler::ZuLang(laenge) => {
                write!(f, "Paket mit {laenge} Bytes ueber der Grenze {PAKETGRENZE}")
            }
            Paketfehler::Leer => write!(f, "Paket ohne Typbyte"),
            Paketfehler::ZuKurz => write!(f, "Paket endet mitten in einem Feld"),
            Paketfehler::Rest(bytes) => write!(f, "{bytes} Bytes nach dem letzten Feld"),
            Paketfehler::UnbekannterTyp(typ) => write!(f, "unbekannter Pakettyp {typ}"),
            Paketfehler::Abgerissen => write!(f, "Strom endet mitten in einem Paket"),
            Paketfehler::Lesefehler(fehler) => write!(f, "Lesefehler: {fehler}"),
        }
    }
}

/// Liest ein Paket aus dem Strom und liefert seinen Rumpf ohne Laenge, oder
/// `None`, wenn der Strom an einer Paketgrenze sauber endet.
///
/// Die Laenge wird gegen [`PAKETGRENZE`] gehalten, **bevor** Speicher fuer
/// den Rumpf entsteht; eine zu grosse Angabe kostet also nichts als vier
/// gelesene Bytes.
pub fn paket_lesen(strom: &mut impl Read) -> Result<Option<Vec<u8>>, Paketfehler> {
    let mut kopf = [0u8; 4];
    let mut gelesen = 0;
    while gelesen < kopf.len() {
        match strom.read(&mut kopf[gelesen..]) {
            Ok(0) if gelesen == 0 => return Ok(None),
            Ok(0) => return Err(Paketfehler::Abgerissen),
            Ok(n) => gelesen += n,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => {}
            Err(fehler) => return Err(Paketfehler::Lesefehler(fehler)),
        }
    }
    let laenge = u32::from_be_bytes(kopf);
    if laenge > PAKETGRENZE {
        return Err(Paketfehler::ZuLang(laenge));
    }
    if laenge == 0 {
        return Err(Paketfehler::Leer);
    }
    let mut rumpf = vec![0u8; laenge as usize];
    strom.read_exact(&mut rumpf).map_err(|fehler| {
        if fehler.kind() == io::ErrorKind::UnexpectedEof {
            Paketfehler::Abgerissen
        } else {
            Paketfehler::Lesefehler(fehler)
        }
    })?;
    Ok(Some(rumpf))
}

impl Paket {
    /// Die Bytes des ganzen Pakets, Laenge eingeschlossen.
    #[must_use]
    pub fn kodieren(&self) -> Vec<u8> {
        let mut s = Schreiber {
            bytes: vec![0, 0, 0, 0],
        };
        match self {
            Paket::Init {
                fassung,
                erweiterungen,
            } => {
                s.u8(INIT);
                s.u32(*fassung);
                s.paare(erweiterungen);
            }
            Paket::Version {
                fassung,
                erweiterungen,
            } => {
                s.u8(VERSION);
                s.u32(*fassung);
                s.paare(erweiterungen);
            }
            Paket::Anfrage { kennung, anfrage } => anfrage_kodieren(&mut s, *kennung, anfrage),
            Paket::Antwort { kennung, antwort } => antwort_kodieren(&mut s, *kennung, antwort),
        }
        let laenge = u32::try_from(s.bytes.len() - 4).unwrap_or(u32::MAX);
        s.bytes[..4].copy_from_slice(&laenge.to_be_bytes());
        s.bytes
    }

    /// Zerlegt den Rumpf eines Pakets, also alles hinter der Laenge.
    pub fn zerlegen(rumpf: &[u8]) -> Result<Paket, Paketfehler> {
        let mut l = Leser { bytes: rumpf };
        let typ = l.u8()?;
        let paket = match typ {
            INIT => Paket::Init {
                fassung: l.u32()?,
                erweiterungen: l.paare()?,
            },
            VERSION => Paket::Version {
                fassung: l.u32()?,
                erweiterungen: l.paare()?,
            },
            STATUS | HANDLE | DATA | NAME | ATTRS => {
                let kennung = l.u32()?;
                let antwort = antwort_zerlegen(&mut l, typ)?;
                Paket::Antwort { kennung, antwort }
            }
            _ => {
                let kennung = l.u32()?;
                let anfrage = anfrage_zerlegen(&mut l, typ)?;
                Paket::Anfrage { kennung, anfrage }
            }
        };
        if !l.bytes.is_empty() {
            return Err(Paketfehler::Rest(l.bytes.len()));
        }
        Ok(paket)
    }
}

fn anfrage_kodieren(s: &mut Schreiber, kennung: u32, anfrage: &Anfrage) {
    match anfrage {
        Anfrage::Open {
            pfad,
            kennzeichen,
            attribute,
        } => {
            s.kopf(OPEN, kennung);
            s.text(pfad);
            s.u32(*kennzeichen);
            s.attribute(attribute);
        }
        Anfrage::Close { handle } => {
            s.kopf(CLOSE, kennung);
            s.text(handle);
        }
        Anfrage::Read {
            handle,
            versatz,
            laenge,
        } => {
            s.kopf(READ, kennung);
            s.text(handle);
            s.u64(*versatz);
            s.u32(*laenge);
        }
        Anfrage::Write {
            handle,
            versatz,
            daten,
        } => {
            s.kopf(WRITE, kennung);
            s.text(handle);
            s.u64(*versatz);
            s.text(daten);
        }
        Anfrage::Lstat { pfad } => {
            s.kopf(LSTAT, kennung);
            s.text(pfad);
        }
        Anfrage::Setstat { pfad, attribute } => {
            s.kopf(SETSTAT, kennung);
            s.text(pfad);
            s.attribute(attribute);
        }
        Anfrage::Opendir { pfad } => {
            s.kopf(OPENDIR, kennung);
            s.text(pfad);
        }
        Anfrage::Readdir { handle } => {
            s.kopf(READDIR, kennung);
            s.text(handle);
        }
        Anfrage::Remove { pfad } => {
            s.kopf(REMOVE, kennung);
            s.text(pfad);
        }
        Anfrage::Mkdir { pfad, attribute } => {
            s.kopf(MKDIR, kennung);
            s.text(pfad);
            s.attribute(attribute);
        }
        Anfrage::Rmdir { pfad } => {
            s.kopf(RMDIR, kennung);
            s.text(pfad);
        }
        Anfrage::Realpath { pfad } => {
            s.kopf(REALPATH, kennung);
            s.text(pfad);
        }
        Anfrage::Stat { pfad } => {
            s.kopf(STAT, kennung);
            s.text(pfad);
        }
        Anfrage::Rename { alt, neu } => {
            s.kopf(RENAME, kennung);
            s.text(alt);
            s.text(neu);
        }
        Anfrage::Readlink { pfad } => {
            s.kopf(READLINK, kennung);
            s.text(pfad);
        }
        Anfrage::Symlink { erste, zweite } => {
            s.kopf(SYMLINK, kennung);
            s.text(erste);
            s.text(zweite);
        }
    }
}

fn anfrage_zerlegen(l: &mut Leser<'_>, typ: u8) -> Result<Anfrage, Paketfehler> {
    Ok(match typ {
        OPEN => Anfrage::Open {
            pfad: l.text()?,
            kennzeichen: l.u32()?,
            attribute: l.attribute()?,
        },
        CLOSE => Anfrage::Close { handle: l.text()? },
        READ => Anfrage::Read {
            handle: l.text()?,
            versatz: l.u64()?,
            laenge: l.u32()?,
        },
        WRITE => Anfrage::Write {
            handle: l.text()?,
            versatz: l.u64()?,
            daten: l.text()?,
        },
        LSTAT => Anfrage::Lstat { pfad: l.text()? },
        SETSTAT => Anfrage::Setstat {
            pfad: l.text()?,
            attribute: l.attribute()?,
        },
        OPENDIR => Anfrage::Opendir { pfad: l.text()? },
        READDIR => Anfrage::Readdir { handle: l.text()? },
        REMOVE => Anfrage::Remove { pfad: l.text()? },
        MKDIR => Anfrage::Mkdir {
            pfad: l.text()?,
            attribute: l.attribute()?,
        },
        RMDIR => Anfrage::Rmdir { pfad: l.text()? },
        REALPATH => Anfrage::Realpath { pfad: l.text()? },
        STAT => Anfrage::Stat { pfad: l.text()? },
        RENAME => Anfrage::Rename {
            alt: l.text()?,
            neu: l.text()?,
        },
        READLINK => Anfrage::Readlink { pfad: l.text()? },
        SYMLINK => Anfrage::Symlink {
            erste: l.text()?,
            zweite: l.text()?,
        },
        andere => return Err(Paketfehler::UnbekannterTyp(andere)),
    })
}

fn antwort_kodieren(s: &mut Schreiber, kennung: u32, antwort: &Antwort) {
    match antwort {
        Antwort::Status {
            code,
            meldung,
            sprache,
        } => {
            s.kopf(STATUS, kennung);
            s.u32(*code);
            s.text(meldung);
            s.text(sprache);
        }
        Antwort::Handle(handle) => {
            s.kopf(HANDLE, kennung);
            s.text(handle);
        }
        Antwort::Data(daten) => {
            s.kopf(DATA, kennung);
            s.text(daten);
        }
        Antwort::Name(namen) => {
            s.kopf(NAME, kennung);
            s.u32(u32::try_from(namen.len()).unwrap_or(u32::MAX));
            for name in namen {
                s.text(&name.name);
                s.text(&name.lang);
                s.attribute(&name.attribute);
            }
        }
        Antwort::Attrs(attribute) => {
            s.kopf(ATTRS, kennung);
            s.attribute(attribute);
        }
    }
}

fn antwort_zerlegen(l: &mut Leser<'_>, typ: u8) -> Result<Antwort, Paketfehler> {
    Ok(match typ {
        STATUS => {
            let code = l.u32()?;
            // Aeltere Server schicken in Fassung 3 nur den Code; Text und
            // Sprache fehlen dann, und das ist kein Fehler.
            let (meldung, sprache) = if l.bytes.is_empty() {
                (Vec::new(), Vec::new())
            } else {
                (l.text()?, l.text()?)
            };
            Antwort::Status {
                code,
                meldung,
                sprache,
            }
        }
        HANDLE => Antwort::Handle(l.text()?),
        DATA => Antwort::Data(l.text()?),
        NAME => {
            let zahl = l.u32()?;
            // Jeder Name braucht mindestens zwoelf Bytes (zwei leere
            // Zeichenketten und die Kennzeichen); eine groessere Zahl kann der
            // Rumpf nicht tragen, und sie legte sonst vorab Speicher an.
            if zahl as usize > l.bytes.len() / 12 {
                return Err(Paketfehler::ZuKurz);
            }
            let mut namen = Vec::with_capacity(zahl as usize);
            for _ in 0..zahl {
                namen.push(Name {
                    name: l.text()?,
                    lang: l.text()?,
                    attribute: l.attribute()?,
                });
            }
            Antwort::Name(namen)
        }
        ATTRS => Antwort::Attrs(l.attribute()?),
        andere => return Err(Paketfehler::UnbekannterTyp(andere)),
    })
}

/// Schreibt Felder in Netzordnung an einen Puffer.
struct Schreiber {
    bytes: Vec<u8>,
}

impl Schreiber {
    fn u8(&mut self, wert: u8) {
        self.bytes.push(wert);
    }

    fn u32(&mut self, wert: u32) {
        self.bytes.extend_from_slice(&wert.to_be_bytes());
    }

    fn u64(&mut self, wert: u64) {
        self.bytes.extend_from_slice(&wert.to_be_bytes());
    }

    fn text(&mut self, text: &[u8]) {
        self.u32(u32::try_from(text.len()).unwrap_or(u32::MAX));
        self.bytes.extend_from_slice(text);
    }

    fn kopf(&mut self, typ: u8, kennung: u32) {
        self.u8(typ);
        self.u32(kennung);
    }

    fn paare(&mut self, paare: &[(Vec<u8>, Vec<u8>)]) {
        for (name, wert) in paare {
            self.text(name);
            self.text(wert);
        }
    }

    fn attribute(&mut self, a: &Attribute) {
        let mut kennzeichen = 0;
        if a.groesse.is_some() {
            kennzeichen |= ATTR_SIZE;
        }
        if a.besitz.is_some() {
            kennzeichen |= ATTR_UIDGID;
        }
        if a.rechte.is_some() {
            kennzeichen |= ATTR_PERMISSIONS;
        }
        if a.zeiten.is_some() {
            kennzeichen |= ATTR_ACMODTIME;
        }
        if !a.erweitert.is_empty() {
            kennzeichen |= ATTR_EXTENDED;
        }
        self.u32(kennzeichen);
        if let Some(groesse) = a.groesse {
            self.u64(groesse);
        }
        if let Some((uid, gid)) = a.besitz {
            self.u32(uid);
            self.u32(gid);
        }
        if let Some(rechte) = a.rechte {
            self.u32(rechte);
        }
        if let Some((zugriff, geaendert)) = a.zeiten {
            self.u32(zugriff);
            self.u32(geaendert);
        }
        if !a.erweitert.is_empty() {
            self.u32(u32::try_from(a.erweitert.len()).unwrap_or(u32::MAX));
            self.paare(&a.erweitert);
        }
    }
}

/// Liest Felder in Netzordnung aus einem Rumpf.
struct Leser<'a> {
    bytes: &'a [u8],
}

impl Leser<'_> {
    fn nimm<const N: usize>(&mut self) -> Result<[u8; N], Paketfehler> {
        let (kopf, rest) = self
            .bytes
            .split_first_chunk::<N>()
            .ok_or(Paketfehler::ZuKurz)?;
        self.bytes = rest;
        Ok(*kopf)
    }

    fn u8(&mut self) -> Result<u8, Paketfehler> {
        Ok(self.nimm::<1>()?[0])
    }

    fn u32(&mut self) -> Result<u32, Paketfehler> {
        Ok(u32::from_be_bytes(self.nimm()?))
    }

    fn u64(&mut self) -> Result<u64, Paketfehler> {
        Ok(u64::from_be_bytes(self.nimm()?))
    }

    fn text(&mut self) -> Result<Vec<u8>, Paketfehler> {
        let laenge = self.u32()? as usize;
        if laenge > self.bytes.len() {
            return Err(Paketfehler::ZuKurz);
        }
        let (text, rest) = self.bytes.split_at(laenge);
        self.bytes = rest;
        Ok(text.to_vec())
    }

    /// Paare aus Name und Wert bis zum Ende des Rumpfes, wie sie `INIT` und
    /// `VERSION` tragen.
    fn paare(&mut self) -> Result<Paare, Paketfehler> {
        let mut paare = Vec::new();
        while !self.bytes.is_empty() {
            paare.push((self.text()?, self.text()?));
        }
        Ok(paare)
    }

    fn attribute(&mut self) -> Result<Attribute, Paketfehler> {
        let kennzeichen = self.u32()?;
        let mut a = Attribute::default();
        if kennzeichen & ATTR_SIZE != 0 {
            a.groesse = Some(self.u64()?);
        }
        if kennzeichen & ATTR_UIDGID != 0 {
            a.besitz = Some((self.u32()?, self.u32()?));
        }
        if kennzeichen & ATTR_PERMISSIONS != 0 {
            a.rechte = Some(self.u32()?);
        }
        if kennzeichen & ATTR_ACMODTIME != 0 {
            a.zeiten = Some((self.u32()?, self.u32()?));
        }
        if kennzeichen & ATTR_EXTENDED != 0 {
            let zahl = self.u32()?;
            for _ in 0..zahl {
                a.erweitert.push((self.text()?, self.text()?));
            }
        }
        Ok(a)
    }
}
