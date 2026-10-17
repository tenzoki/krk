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
//!   Leser des Zentralverzeichnisses.
//! - [`zugang`] haelt eine Archivdatei samt Zwischenspeicher.
//!
//! **Geschrieben wird hier nichts.** Das Lesen ist der erste Schritt des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`; ein Archiv
//! wird dort spaeter allein ueber einen Umbau geaendert, der das ganze
//! Archiv neben das alte schreibt und es zuletzt mit einem `rename`
//! ersetzt.

pub mod format;
pub mod innenpfad;
pub mod verzeichnis;
pub(crate) mod zipform;
pub mod zugang;

pub use format::Archivformat;
pub use innenpfad::Innenpfad;
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
