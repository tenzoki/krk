//! Ordner auf einem Server, gesprochen ueber SFTP v3.
//!
//! KRK spricht das Protokoll selbst, ueber die Pipes eines Kindprozesses;
//! Verschluesselung, Rechnerschluessel, Schluessel, Agent und `~/.ssh/config`
//! bleiben bei `/usr/bin/ssh`, das mit `-s … sftp` allein den Kanal traegt
//! (Moeglichkeit 2 der Analyse
//! `261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md`, Plan
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`). Keine Kiste
//! kommt dafuer in den Baum: Prozess und Pipes gehen ueber `std::process`.
//!
//! ```text
//!  Arbeitsfaden (faden::faden, mit &Fernfaden)
//!    │ Gespraech::realpath / opendir / readdir / open / read / write / …
//!    v
//!  gespraech    Tabelle der Kennungen, Mutex um die Eingabe
//!    │ paket::Paket::kodieren                ^ paket::paket_lesen, zerlegen
//!    v                                       │
//!  Kindprozess: /usr/bin/ssh -s … sftp  ──>  Gespraechsfaden krk-sftp-<name>
//!               (in den Proben: /usr/libexec/sftp-server)
//! ```
//!
//! - [`paket`] kodiert und zerlegt die Pakete und kennt weder Strom noch
//!   Faden.
//! - [`gespraech`] fuehrt ein Gespraech: Kennungen, offene Anfragen zugleich,
//!   Trennung.
//! - [`faden`](mod@faden) haelt jede Anfrage vom Hauptfaden fern, beim Uebersetzen.
//! - [`pfad`] rechnet an Pfaden auf dem Server, rein lexikalisch.
//! - [`adresse`] zerlegt die Adresse einer Verbindung in Rechner, Port, Konto
//!   und Startordner.
//! - [`kanal`] rechnet den Aufruf von `ssh` aus und startet ihn.
//! - [`rechnerschluessel`] fragt vor dem Verbinden, ob `ssh` den Schluessel
//!   des Rechners kennt.
//! - [`zugang`] haelt ein Gespraech je Server und baut es neu auf.
//!
//! **Die Proben brauchen kein Netz.** `tests/fern.rs` startet
//! `/usr/libexec/sftp-server` unmittelbar als Kindprozess und spricht mit ihm
//! ueber dieselben Pipes, ueber die im Betrieb `ssh` die Bytes des Servers
//! durchreicht; derselbe Code, nur ohne `ssh` davor. Was das nicht prueft,
//! ist der Server des Nutzers: ob er `RENAME` nach dem Entwurf beantwortet,
//! sagt allein der Test gegen ihn.

pub mod adresse;
pub mod faden;
pub mod gespraech;
pub mod kanal;
pub mod paket;
pub mod pfad;
pub mod rechnerschluessel;
pub mod zugang;

pub use adresse::{Adressfehler, Fernziel, Serverkennung, Startpfad, fernziel};

pub use faden::{Fernfaden, faden};
pub use gespraech::{
    Ausstehend, BLOCK, Fernfehler, Gespraech, Handle, OFFENE_LESUNGEN, OFFENE_SCHREIBUNGEN,
    Trennung, Trennungsgrund, Zustand,
};
pub use kanal::{Anmeldung, Kanalplan, ssh_plan};
pub use pfad::{Fernpfad, ist_einzelner_name};
pub use zugang::{Fernzugang, Herkunft, Startordner, Zugangsfehler, Zugangsregister};
