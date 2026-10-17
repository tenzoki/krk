//! Archive als Orte: Zip, tar, tar.gz und gz lesen, ohne sie zu entpacken.
//!
//! ```text
//!  Hauptfaden: Archivformat::aus_namen (Endung), Archivzugang::neu
//!              -- kein Systemaufruf --
//!  Arbeitsfaden (fern::faden, mit &Fernfaden)
//!    │ Archivzugang::verzeichnis
//!    v
//!  zugang       stat, Kennung, Zwischenspeicher ──> gespeichertes Verzeichnis
//!    │ ohne_warten_oeffnen, fstat
//!    v
//!  zipform::verzeichnis_lesen │ verzeichnis::tar_lesen │ tar_gz_lesen │ gz_lesen
//!    │ je Eintrag: Innenpfad::aus_archivname
//!    v
//!  verzeichnis::Sammler::abschliessen ──> Inhaltsverzeichnis
//! ```
//!
//! - [`format`](mod@format) entscheidet am Namen, ob eine Datei ein Archiv ist und welches.
//! - [`innenpfad`] rechnet an Pfaden im Archiv und traegt die eine
//!   Namensregel fuer alle Formate.
//! - [`verzeichnis`] haelt das Inhaltsverzeichnis, die Regeln ueber alle
//!   Eintraege zugleich und die Leser fuer tar und gz.
//! - `zipform` haelt das Zip-Format: die Zeitfelder und Rechte, die
//!   `operation::zippen` schreibt und `operation::entpacken` liest, und den
//!   Leser des Zentralverzeichnisses, dazu die rohe Form der Saetze fuer den
//!   Umbau.
//! - [`zugang`] haelt eine Archivdatei samt Zwischenspeicher.
//! - [`lesen`] liest einen Eintrag fuer das Herauskopieren: Zip wahlfrei,
//!   tar und tar.gz ueber einen Zeiger, der nur vorwaerts laeuft (Schritt 4
//!   des Plans).
//! - [`umbau`] aendert ein Archiv: jede Konfliktfrage am Verzeichnis, dann das
//!   ganze Archiv neu (Schritt 5 des Plans).
//! - [`ersetzen`] ist der einzige Schreiber an einer Archivdatei: das neue
//!   Archiv unter einem Zwischennamen, die Kennung des alten geprueft, zuletzt
//!   ein `rename`.
//!
//! **Das Original wird nie zum Schreiben geoeffnet.** Ein Archiv wird allein
//! ueber [`umbau::umbauen`] geaendert, der das ganze Archiv neben das alte
//! schreibt und es zuletzt mit einem `rename` ersetzt (E12 und E13 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).

pub mod ersetzen;
pub mod format;
pub mod innenpfad;
pub mod lesen;
pub mod umbau;
pub mod verzeichnis;
pub(crate) mod zipform;
pub mod zugang;

pub use format::Archivformat;
pub use innenpfad::Innenpfad;
pub use lesen::{Archivleser, Lesefehler};
pub use verzeichnis::{
    Archiveintrag, Fundstelle, Inhaltsverzeichnis, Lesbarkeit, Verzeichnisfehler,
};
pub use zugang::{Archivzugang, Dateikennung};

/// Wie viele Eintraege ein Archiv hoechstens fuehren darf, damit KRK es
/// zeigt.
///
/// Ein Eintrag des Inhaltsverzeichnisses kostet um die 200 Bytes samt Name,
/// Platz in der Namenstafel und Kinderliste; gemessen ist das in der
/// ignorierten Probe `hunderttausend_zip_eintraege_messen` in
/// `tests/archiv.rs`. Eine Million Eintraege sind damit rund 200 MB, und
/// waehrend des Lesens eines Zip haelt die Kiste ihr eigenes
/// Zentralverzeichnis daneben. Ein Archiv darueber meldet
/// [`Verzeichnisfehler::ZuGross`] und wird nicht gezeigt.
pub const HOECHSTENS_EINTRAEGE: usize = 1_000_000;
