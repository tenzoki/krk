//! Das Ersetzen einer Archivdatei: der einzige Schreiber an einer Archivdatei.
//!
//! ```text
//!  umbau::umbauen
//!    │ ersetzen(archiv, kennung, rechte, schreiben)
//!    v
//!  1. Zwischenname im Ordner des Archivs, create_new, Rechte des Archivs
//!  2. schreiben(&mut datei)        <- das ganze neue Archiv
//!     auf_die_platte_bringen
//!  3. abgebrochen?                 ja: Zwischenname weg, Abgebrochen
//!     stat(archiv) == kennung?     nein: Zwischenname weg, VonAussenVeraendert
//!  4. rename(Zwischenname, archiv)
//! ```
//!
//! **KRK oeffnet das Original nie zum Schreiben** (E12 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Jede
//! Aenderung entsteht als vollstaendiges neues Archiv unter einem Zwischennamen
//! im selben Ordner, also auf demselben Dateisystem, und erst danach ersetzt
//! ein einziges `rename(2)` das Original. Vor diesem `rename` ist das Original
//! unberuehrt, danach ist das neue vollstaendig. Ein Abbruch, ein Fehler und
//! ein voller Datentraeger in den Schritten 1 bis 3 raeumen den Zwischennamen
//! weg und lassen das Original Byte fuer Byte stehen; das haelt der
//! [`Zwischendatei`]-Waechter, dessen `Drop` den Namen entfernt, solange das
//! `rename` nicht gelungen ist.
//!
//! **Geprueft wird eine entscheidbare Frage und nicht eine unentscheidbare.**
//! Ob ein anderes Programm das Archiv zwischen Pruefung und `rename` beschreibt,
//! laesst sich ohne Sperre nicht feststellen, und macOS kennt keine
//! verbindliche. Gefragt wird deshalb, ob die Datei unter dem Namen
//! unmittelbar vor dem `rename` dieselbe ist wie beim Beginn des Umbaus:
//! Geraet, Inode, Groesse und Aenderungszeit in Nanosekunden
//! ([`Dateikennung`]). Was zwischen dieser Frage und dem `rename` in das alte
//! Archiv geschrieben wird, landet im alten Inode und ist danach verloren; das
//! Fenster ist einen Systemaufruf breit und bleibt eine genannte Grenze.
//!
//! # Was das Ersetzen nicht erhaelt
//!
//! Das neue Archiv ist eine neue Datei. Harte Verweise auf das alte zeigen
//! danach auf das alte, und die erweiterten Attribute samt Finder-Etiketten,
//! die Zugriffslisten und das Erstellungsdatum der Archivdatei gehen verloren.
//! Die neun Rechtebits gehen mit, und zwar **bevor** der Inhalt in den
//! Zwischennamen fliesst, aus dem Grund, den `ablage::atomar` fuer dieselbe
//! Reihenfolge nennt; die Sonderbits bleiben aus dem Grund von
//! `ablage::atomar::RECHTEMASKE` draussen. Ein Absturz von KRK mitten im
//! Umbau hinterlaesst den Zwischennamen mit fuehrendem Punkt, und KRK raeumt
//! ihn nicht auf. Geschrieben wird nur, wenn KRK im Ordner des Archivs
//! schreiben darf; das Schreibrecht an der Datei allein genuegt nicht, und
//! fehlt es, schreibt der Umbau gar nicht erst (`Umbaufehler::Schreibgeschuetzt`
//! in `umbau`).
//!
//! # Der letzte Augenblick fuer einen Abbruch
//!
//! **Gefragt wird der Abbruch ein letztes Mal unmittelbar vor dem `rename`**,
//! nachdem das neue Archiv auf der Platte steht. Bis dahin laesst er das
//! Original unberuehrt; was danach kommt, haelt nichts mehr auf, und der
//! Rufer meldet den Vorgang dann als fertig und den Abbruch als zu spaet
//! gekommen (`umbau::abschluss_nach_ersetzen`). So gibt es zu einem Abbruch
//! genau zwei Ausgaenge: abgebrochen und das Archiv unveraendert, oder fertig
//! und das Archiv ersetzt.
//!
//! Dass hier und nirgends sonst unter `archiv/` eine Datei zum Schreiben
//! geoeffnet, umbenannt oder entfernt wird, haelt die Probe
//! `archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei` in
//! `tests/archiv_umbau.rs`.

use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::ablage::atomar::RECHTEMASKE;
use crate::verzeichnis::sys::auf_die_platte_bringen;

use super::umbau::Umbaufehler;
use super::zugang::Dateikennung;

/// Der Vorsatz jedes Zwischennamens. Der fuehrende Punkt verbirgt ihn im
/// Finder; KRK zeigt ihn als versteckten Eintrag, wenn ein Absturz ihn
/// zurueckgelassen hat.
pub const ZWISCHENVORSATZ: &str = ".krk-umbau-";

/// Wie lang ein Dateiname auf den Dateisystemen von macOS hoechstens ist.
const NAMENSGRENZE: usize = 255;

/// Zaehlt die Zwischennamen dieses Prozesses, damit zwei Umbauten zugleich
/// nie denselben Namen waehlen.
static LAUFNUMMER: AtomicU64 = AtomicU64::new(0);

/// Ersetzt `archiv` durch das, was `schreiben` in eine neue Datei schreibt.
///
/// `archiv` ist der Pfad der Archivdatei selbst, eine Verknuepfung ist vorher
/// aufgeloest; `kennung` ist die Kennung des Archivs beim Beginn des Umbaus,
/// genommen am Deskriptor, aus dem der Umbau das alte Archiv liest; `rechte`
/// sind dessen Rechte. `schreiben` bekommt die frisch angelegte, leere Datei
/// unter dem Zwischennamen, zum Lesen und Schreiben geoeffnet, und schreibt
/// darin das ganze neue Archiv.
///
/// # Errors
///
/// Jeder Fehler laesst das Original unberuehrt und den Zwischennamen
/// entfernt: der Fehler von `schreiben`, [`Umbaufehler::Schreiben`] fuer das
/// Anlegen, das Synchronisieren und das `rename`, und
/// [`Umbaufehler::VonAussenVeraendert`], wenn die Kennung abweicht, und
/// [`Umbaufehler::Abgebrochen`], wenn `abbruch` vor dem `rename` steht
/// (Modulkopf).
pub(crate) fn ersetzen(
    archiv: &Path,
    kennung: Dateikennung,
    rechte: u32,
    abbruch: &AtomicBool,
    schreiben: impl FnOnce(&mut File) -> Result<(), Umbaufehler>,
) -> Result<(), Umbaufehler> {
    let mut zwischen = Zwischendatei::anlegen(archiv, rechte)?;
    schreiben(zwischen.datei())?;
    auf_die_platte_bringen(zwischen.datei()).map_err(Umbaufehler::Schreiben)?;
    if abbruch.load(Ordering::Relaxed) {
        return Err(Umbaufehler::Abgebrochen);
    }

    let jetzt = fs::metadata(archiv).map_err(Umbaufehler::Schreiben)?;
    if Dateikennung::aus_angaben(&jetzt) != kennung {
        return Err(Umbaufehler::VonAussenVeraendert);
    }
    zwischen.umbenennen(archiv)?;

    // Der Ordner haelt jetzt den neuen Namen; ihn auf die Platte zu bringen
    // ist die Zugabe fuer einen Stromausfall in den Sekunden danach. Scheitert
    // es, ist das Archiv trotzdem ersetzt, und eine Meldung waere falsch.
    if let Some(ordner) = archiv.parent()
        && let Ok(ordner) = File::open(ordner)
    {
        let _ = auf_die_platte_bringen(&ordner);
    }
    Ok(())
}

/// Der Zwischenname zu `archiv`: im selben Ordner, mit Vorsatz, Prozess und
/// Laufnummer, und hoechstens so lang, wie ein Name sein darf.
fn zwischenpfad(archiv: &Path) -> PathBuf {
    let name = archiv
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let vorsatz = format!(
        "{ZWISCHENVORSATZ}{}-{}-",
        std::process::id(),
        LAUFNUMMER.fetch_add(1, Ordering::Relaxed)
    );
    let mut rest = NAMENSGRENZE.saturating_sub(vorsatz.len()).min(name.len());
    while !name.is_char_boundary(rest) {
        rest -= 1;
    }
    archiv.with_file_name(format!("{vorsatz}{}", &name[..rest]))
}

/// Eine Datei unter einem Zwischennamen, die verschwindet, solange sie nicht
/// an ihren Platz umbenannt ist.
struct Zwischendatei {
    pfad: PathBuf,
    datei: Option<File>,
    umbenannt: bool,
}

impl Zwischendatei {
    /// Legt den Zwischennamen ausschliessend an und gibt ihm die Rechte des
    /// Archivs, bevor ein Byte hineingeht.
    fn anlegen(archiv: &Path, rechte: u32) -> Result<Self, Umbaufehler> {
        let pfad = zwischenpfad(archiv);
        let rechte = rechte & RECHTEMASKE;
        let datei = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(rechte)
            .open(&pfad)
            .map_err(Umbaufehler::Schreiben)?;
        let zwischen = Zwischendatei {
            pfad,
            datei: Some(datei),
            umbenannt: false,
        };
        // `mode` wirkt unter der umask; `fchmod` am Deskriptor setzt die Bits
        // genau.
        if let Some(datei) = &zwischen.datei {
            datei
                .set_permissions(fs::Permissions::from_mode(rechte))
                .map_err(Umbaufehler::Schreiben)?;
        }
        Ok(zwischen)
    }

    fn datei(&mut self) -> &mut File {
        self.datei
            .as_mut()
            .expect("die Datei bleibt offen bis zum Umbenennen")
    }

    /// Schliesst die Datei und benennt sie an die Stelle des Archivs um.
    fn umbenennen(&mut self, archiv: &Path) -> Result<(), Umbaufehler> {
        drop(self.datei.take());
        fs::rename(&self.pfad, archiv).map_err(Umbaufehler::Schreiben)?;
        self.umbenannt = true;
        Ok(())
    }
}

impl Drop for Zwischendatei {
    fn drop(&mut self) {
        drop(self.datei.take());
        if !self.umbenannt {
            // Ein Name, der sich nicht entfernen laesst, bleibt mit fuehrendem
            // Punkt liegen; das Original ist davon nicht beruehrt.
            let _ = fs::remove_file(&self.pfad);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Die Proben an der Platte stehen im Pruefmodul von `umbau.rs`: ein
    // Pruefmodul mit `temp_dir()` und `remove_dir_all` neben dem `Drop` von
    // `Zwischendatei` hielte `genau_drei_pruefordner_fassungen_stehen_im_baum`
    // fuer eine vierte Pruefordner-Fassung.

    /// Der Zwischenname bleibt auch bei einem langen Namen ein gueltiger Name.
    #[test]
    fn der_zwischenname_bleibt_unter_der_namensgrenze() {
        let lang = "ä".repeat(127);
        let pfad = zwischenpfad(&Path::new("/tmp").join(format!("{lang}.zip")));
        let name = pfad.file_name().expect("Name").to_str().expect("UTF-8");
        assert!(name.len() <= NAMENSGRENZE, "{} Bytes", name.len());
        assert!(name.starts_with(ZWISCHENVORSATZ));
    }
}
