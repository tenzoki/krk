//! Der Pruefordner der Proben dieser Kiste: **die** eine Fassung.
//!
//! Ein Ordner unter dem Temporaerverzeichnis, der Zweck, Prozesskennung und
//! Laufnummer im Namen traegt und sich in `Drop` wieder abraeumt. Prozesskennung
//! und Laufnummer sind kein Zierrat: ohne sie treffen sich zwei gleichzeitige
//! Testlaeufe in demselben Ordner, und ein Fehlschlag daraus benennt nichts, was
//! am Code falsch waere. Zwei Defekte dazu stehen im Baum
//! (`260807-0800` fuer das Leistenmodell, `260810-1256` fuer die Vorschau).
//!
//! Bis zum 260810 stand diese halbe Seite viermal in dieser einen Kiste, in den
//! Probenmodulen von `vorschaumodell`, `editormodell`, `leistenmodell` und
//! `kommandos::pfadeingabe`. Der Defekt dazu ist
//! `issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.
//!
//! Der Ordner liegt unter `std::env::temp_dir()` und ausdruecklich **nicht**
//! unter `~/Library/Caches/krk-messplatz`: der Messplatz gehoert der
//! Messstrecke, nicht den Proben.
//!
//! # Zwei Erzeuger, weil zwei Proben den Ordner fehlend brauchen
//!
//! [`Pruefordner::neu`] legt den Ordner an, [`Pruefordner::nur_name`] liefert
//! nur einen freien Namen. Die Proben der Gueltigkeitspruefung im
//! `leistenmodell` brauchen denselben Pfad einmal vorhanden und einmal fehlend
//! und schalten mit [`Pruefordner::anlegen`] und [`Pruefordner::loeschen`]
//! zwischen beiden Lagen um.
//!
//! # Der Zaehler steht hier und nicht je Probenmodul
//!
//! Alle Einheitsproben von `krk-ui` uebersetzen in **ein** Probenprogramm, also
//! in einen Prozess. Vier Zaehler in vier Modulen konnten deshalb zweimal
//! dieselbe Laufnummer liefern, und zwei Proben mit demselben Zweck in
//! verschiedenen Modulen haetten denselben Ordner getroffen. Ein Zaehler fuer
//! die Kiste schliesst das aus.
//!
//! Diese Fassung ist nicht mit der in `krk-core/tests/gemeinsam/mod.rs` oder der
//! in `krk-bench/src/wegwerfordner.rs` zusammenlegbar: ein Testziel und ein
//! Binaerziel sind je eine eigene Kiste, und `krk-ui` hat kein
//! Bibliotheksziel, das eine gemeinsame Fassung tragen koennte.
//!
//! # Das Abraeumen ist zweistufig, wie im Kern
//!
//! `CLAUDE.md` sagt, es gebe drei Fassungen, eine je Kiste, und dass sie
//! dasselbe **tun**, stand bis zum 260908 nirgends und war auch nicht so: der
//! Kern raeumte ueber [`abraeumen`] in zwei Stufen ab, diese Kiste ueber ein
//! blankes `remove_dir_all` (Defekt `260826-1442`). Zwei Proben dieser Kiste
//! setzen `0o000` — `kommandos::pfadeingabe` auf einen Unterordner,
//! `leistenmodell` auf eine Datei —, und an einem Unterordner mit `0o000`
//! scheitert der einstufige Weg. Die eine Probe hat deshalb von Hand
//! aufgeraeumt, „bevor die Probe fehlschlagen kann"; genau diese Handarbeit
//! macht der zweistufige Weg entbehrlich, und sie steht dort seither als das
//! da, was sie ist: eine Vorsichtsmassnahme, die den Fall gar nicht mehr
//! erreicht.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Die Laufnummer, die zwei Ordner desselben Zwecks im selben Prozess trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
#[must_use = "fallengelassen raeumt Drop den Ordner sofort wieder ab; er ist zu halten, solange die Probe laeuft"]
pub struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    /// Ein frisch angelegter, leerer Ordner.
    pub fn neu(zweck: &str) -> Self {
        let ordner = Self::nur_name(zweck);
        ordner.anlegen();
        ordner
    }

    /// Ein Name, unter dem noch nichts liegt. Angelegt wird der Ordner nicht.
    pub fn nur_name(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let pfad = std::env::temp_dir().join(format!(
            "krk-ui-probe-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        abraeumen(&pfad);
        Self { pfad }
    }

    /// Der Ordner selbst.
    pub fn pfad(&self) -> &Path {
        &self.pfad
    }

    /// Ein Pfad im Ordner, ohne dass dort etwas angelegt wird.
    pub fn unter(&self, name: &str) -> PathBuf {
        self.pfad.join(name)
    }

    pub fn anlegen(&self) {
        std::fs::create_dir_all(&self.pfad).expect("der Pruefordner laesst sich nicht anlegen");
    }

    pub fn loeschen(&self) {
        std::fs::remove_dir_all(&self.pfad).expect("der Pruefordner laesst sich nicht loeschen");
    }

    /// Legt einen Unterordner an und liefert seinen Pfad.
    ///
    /// **Die Schwesterfassung in `krk-core/tests/gemeinsam/mod.rs` fuehrt das
    /// schon**; hier fehlte es bis zum 260810, weil vor dem `Planordner` der
    /// Messstrecke keine Probe dieser Kiste einen Unterordner ueber den
    /// Pruefordner angelegt hat
    /// (`issues/260810-1430_*_planordner-in-messmodus-ist-die-dreizehnte-fassung-und-kann-jetzt-auf-die-gemeinsame-aufsetzen.md`).
    pub fn ordner(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        std::fs::create_dir_all(&pfad).expect("der Unterordner laesst sich nicht anlegen");
        pfad
    }

    /// Legt eine Datei an und liefert ihren Pfad.
    ///
    /// Der Inhalt ist `impl AsRef<[u8]>` und nicht `&str`, weil die Proben der
    /// Vorschau Bytefolgen schreiben, die in Rust-Quelltext als Zeichenkette
    /// nicht mehr das waeren, was auf der Platte stehen soll.
    pub fn datei(&self, name: &str, inhalt: impl AsRef<[u8]>) -> PathBuf {
        let pfad = self.unter(name);
        std::fs::write(&pfad, inhalt).expect("die Pruefdatei laesst sich nicht schreiben");
        pfad
    }

    /// Legt eine benannte Roehre an und liefert ihren Pfad.
    ///
    /// Angelegt wird sie ueber `mkfifo(1)` und nicht ueber einen Fremdaufruf:
    /// `krk-ui` traegt `#![deny(unsafe_code)]`, KRK legt keine Roehren an, und
    /// ein Werkzeug des Systems zu rufen ist der kleinere Eingriff. Dieselbe
    /// Begruendung wie in `krk-core/tests/gemeinsam/mod.rs`.
    pub fn roehre(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        let stand = std::process::Command::new("/usr/bin/mkfifo")
            .arg(&pfad)
            .status()
            .expect("mkfifo laesst sich nicht starten");
        assert!(stand.success(), "mkfifo ist gescheitert: {stand:?}");
        pfad
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        abraeumen(&self.pfad);
    }
}

/// Raeumt einen Baum ab, notfalls gegen entzogene Rechte.
///
/// Zwei Stufen, und die Reihenfolge ist der Grund fuer beide: `remove_dir_all`
/// raeumt einen Ordner mit vielen Eintraegen in einem Zug ab, kommt aber an
/// einem Eintrag mit `0o000` nicht vorbei. [`entsperren_und_loeschen`] steigt
/// dafuer Eintrag fuer Eintrag hinab und dreht jedem die Rechte zurueck; das
/// kostet einen Aufruf je Eintrag und laeuft deshalb nur, wenn der schnelle Weg
/// gescheitert ist.
///
/// **Wortgleich mit `abraeumen` in `krk-core/tests/gemeinsam/mod.rs`**, und
/// zusammenlegen laesst es sich nicht: der Modulkopf sagt, warum.
fn abraeumen(pfad: &Path) {
    if std::fs::remove_dir_all(pfad).is_ok() {
        return;
    }
    let _ = entsperren_und_loeschen(pfad);
}

/// Raeumt einen Baum ab und gibt vorher jedem Eintrag wieder Rechte.
///
/// Eine Verknuepfung bekommt keine neuen Rechte: `set_permissions` folgt ihr
/// und aendere sonst die Rechte ihres Ziels, das ausserhalb des Pruefordners
/// liegen kann.
fn entsperren_und_loeschen(pfad: &Path) -> std::io::Result<()> {
    if let Ok(angaben) = std::fs::symlink_metadata(pfad) {
        if !angaben.is_symlink() {
            let _ = std::fs::set_permissions(pfad, std::fs::Permissions::from_mode(0o755));
        }
        if angaben.is_dir()
            && let Ok(eintraege) = std::fs::read_dir(pfad)
        {
            for eintrag in eintraege.flatten() {
                let _ = entsperren_und_loeschen(&eintrag.path());
            }
        }
    }
    match std::fs::symlink_metadata(pfad) {
        Ok(angaben) if angaben.is_dir() => std::fs::remove_dir(pfad),
        Ok(_) => std::fs::remove_file(pfad),
        Err(fehler) => Err(fehler),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die zweite Stufe, an dem Fall gefahren, den die erste nicht schafft.
    ///
    /// Der Pruefordner haelt seinen Pfad, nicht der Ordner den Pruefordner:
    /// nach dem `drop` ist die Frage, ob unter dem gemerkten Pfad noch etwas
    /// steht. Unter `root` greift die Rechtesperre nicht, dann raeumt schon die
    /// erste Stufe ab und die Probe misst nichts — sie ist trotzdem gruen, und
    /// die Zusage ist dieselbe.
    #[test]
    fn ein_unterordner_ohne_rechte_haelt_das_abraeumen_nicht_auf() {
        let ordner = Pruefordner::neu("abraeumen-gesperrt");
        let pfad = ordner.pfad().to_path_buf();
        let gesperrt = ordner.ordner("gesperrt");
        std::fs::write(gesperrt.join("darin.txt"), b"x").expect("die Datei laesst sich schreiben");
        std::fs::set_permissions(&gesperrt, std::fs::Permissions::from_mode(0o000))
            .expect("die Rechte lassen sich entziehen");

        drop(ordner);

        assert!(
            !pfad.exists(),
            "{} steht noch: das Abraeumen ist an dem 0o000 haengengeblieben",
            pfad.display()
        );
    }
}
