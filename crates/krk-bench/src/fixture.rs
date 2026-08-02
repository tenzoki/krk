//! Der Pruefordner-Erzeuger.
//!
//! Ein Aufruf legt einen flachen Ordner aus gemischten Eintragsarten, Groessen
//! und Namen an. Derselbe Startwert liefert denselben Ordner, auf jedem Geraet
//! und zu jeder Zeit; ein anderer Startwert liefert einen anderen. Genau daran
//! haengt die Messstrecke: ohne Reproduzierbarkeit messen zwanzig
//! Wiederholungen zwanzig verschiedene Ordner, und die Zahl darunter ist nicht
//! verteidigbar.
//!
//! # Was "derselbe Ordner" hier heisst
//!
//! Reproduziert werden **Name, Art, Groesse, Verknuepfungsziel und
//! Aenderungsdatum** jedes Eintrags, dazu das Aenderungsdatum des Ordners
//! selbst. Das geht ueber die Zusage des Plans hinaus, der nur von Namens- und
//! Groessenlisten spricht, und es hat einen Grund: erst mit festen
//! Aenderungsdaten ist die Abnahme ueber `ls -la <ordner> | shasum` ueberhaupt
//! durchfuehrbar, denn `ls -la` druckt die Zeiten mit. Die Zeiten liegen
//! bewusst in der Vergangenheit (ab 2020), weil `ls` fuer alles, was aelter als
//! ein halbes Jahr ist, die Jahreszahl statt der Uhrzeit druckt; damit bleibt
//! die Ausgabe auch in einem Jahr noch dieselbe.
//!
//! # Duennbesetzte Dateien
//!
//! Ein Eintrag traegt seine Groesse als Zahl, nicht als Plattenplatz. Nur die
//! ersten [`ECHTE_BYTES`] Bytes werden wirklich geschrieben, der Rest entsteht
//! ueber `set_len` als Loch. Der Verzeichnisleser fragt
//! `ATTR_FILE_DATALENGTH` ab und sieht die volle Groesse; auf der Platte liegt
//! ein Bruchteil davon. Fuer eine Messstrecke, die Verzeichnismetadaten liest
//! und keine Dateiinhalte, aendert das am Messwert nichts, spart aber bei
//! 100.000 Eintraegen mehrere Gigabyte. **Wer diese Ordner spaeter fuer eine
//! Messung von Kopiervorgaengen (L8) benutzen will, muss das wissen:** dafuer
//! taugen sie nicht.

use std::fs::{self, File, FileTimes};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Wie viele Bytes je Datei wirklich geschrieben werden. Alles darueber ist ein
/// Loch.
pub const ECHTE_BYTES: u64 = 512;

/// Fruehester Zeitpunkt der erzeugten Aenderungsdaten: 2020-01-01T00:00:00Z.
const ZEITBASIS: u64 = 1_577_836_800;

/// Spanne, ueber die sich die Aenderungsdaten verteilen: rund vier Jahre.
const ZEITSPANNE: u64 = 4 * 365 * 24 * 60 * 60;

/// Wie viele Verknuepfungen ein `touch`-Aufruf auf einmal bekommt.
const TOUCH_BUENDEL: usize = 400;

/// Die Namensteile, aus denen sich die Eintragsnamen zusammensetzen. Die
/// unterschiedlichen Laengen sind Absicht: `getattrlistbulk` packt seine
/// Antwortsaetze dicht, und die Satzlaenge haengt an der Namenslaenge.
const STAEMME: [&str; 24] = [
    "notiz",
    "bericht",
    "auszug",
    "protokoll",
    "bild",
    "tabelle",
    "entwurf",
    "abschrift",
    "vermerk",
    "liste",
    "plan",
    "messung",
    "quelle",
    "sicherung",
    "vorlage",
    "anhang",
    "verzeichnis-der-vorgaenge",
    "x",
    "zusammenfassung-der-woche",
    "beleg",
    "karte",
    "skizze",
    "aufstellung",
    "uebersicht",
];

/// Die Endungen, aus denen sich die gemischten Dateitypen ergeben. Der leere
/// Eintrag steht fuer eine Datei ohne Endung.
const ENDUNGEN: [&str; 14] = [
    ".txt", ".md", ".rs", ".toml", ".png", ".jpg", ".pdf", ".zip", ".bin", ".csv", ".log",
    ".tar.gz", ".plist", "",
];

/// Die Groessenklassen: Anteil in Promille und die Spanne in Bytes.
///
/// Die Verteilung bildet einen gewachsenen Arbeitsordner nach: viele kleine
/// Dateien, wenige grosse, ein paar leere. Die Summe der Anteile ist 1.000.
const GROESSENKLASSEN: [(u32, u64, u64); 5] = [
    (100, 0, 1),                        // leer
    (350, 1, 512),                      // winzig
    (300, 512, 8 * 1024),               // klein
    (190, 8 * 1024, 256 * 1024),        // mittel
    (60, 256 * 1024, 64 * 1024 * 1024), // gross
];

/// Anteil der Unterordner in Promille.
const ANTEIL_ORDNER: u32 = 20;

/// Anteil der symbolischen Verknuepfungen in Promille.
const ANTEIL_VERKNUEPFUNG: u32 = 10;

// ---------------------------------------------------------------------------
// Der Zufallsgenerator
// ---------------------------------------------------------------------------

/// SplitMix64, der Zufallsgenerator hinter der Reproduzierbarkeit.
///
/// Selbst geschrieben und nicht aus einer Fremdbibliothek geholt, weil die
/// Zusage hier lautet: derselbe Startwert liefert dieselbe Liste, heute und in
/// zwei Jahren. Eine Fremdbibliothek darf ihren Zahlenstrom mit einer neuen
/// Hauptversion aendern, dieses Dutzend Zeilen nicht. Der Algorithmus stammt
/// aus Steele/Lea/Flood 2014 und ist unveraendert uebernommen.
#[derive(Debug, Clone)]
pub struct Zufall {
    zustand: u64,
}

impl Zufall {
    /// Ein Generator zum genannten Startwert.
    pub fn neu(startwert: u64) -> Self {
        Self { zustand: startwert }
    }

    /// Die naechste Zahl.
    pub fn naechste(&mut self) -> u64 {
        self.zustand = self.zustand.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.zustand;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Eine Zahl unterhalb der Grenze.
    ///
    /// Die Restklassenbildung bevorzugt die kleinen Reste minimal. Bei Grenzen
    /// bis 64 Mi gegenueber einem Wertebereich von 2^64 liegt die Abweichung
    /// unter 2^-38 und ist fuer die Zusammensetzung eines Pruefordners ohne
    /// Belang.
    pub fn unter(&mut self, grenze: u64) -> u64 {
        debug_assert!(grenze > 0, "die Grenze muss positiv sein");
        self.naechste() % grenze
    }

    /// Eine Zahl in den Grenzen `[von, bis)`.
    pub fn zwischen(&mut self, von: u64, bis: u64) -> u64 {
        debug_assert!(bis > von, "die obere Grenze muss ueber der unteren liegen");
        von + self.unter(bis - von)
    }
}

// ---------------------------------------------------------------------------
// Der Bauplan
// ---------------------------------------------------------------------------

/// Die Art eines geplanten Eintrags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// Eine gewoehnliche Datei.
    Datei,
    /// Ein leerer Unterordner. Der Pruefordner bleibt flach: in den
    /// Unterordnern liegt nichts.
    Ordner,
    /// Eine symbolische Verknuepfung auf einen anderen Eintrag desselben
    /// Ordners.
    Verknuepfung,
}

/// Ein geplanter Eintrag, noch bevor er im Dateisystem steht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bauplan {
    /// Der Name ohne Pfad.
    pub name: String,
    /// Datei, Ordner oder Verknuepfung.
    pub art: Art,
    /// Die Groesse in Bytes. Ordner und Verknuepfungen tragen 0.
    pub groesse: u64,
    /// Das Aenderungsdatum, das der Erzeuger setzt.
    pub geaendert: SystemTime,
    /// Bei einer Verknuepfung der Name des Ziels, sonst `None`.
    pub ziel: Option<String>,
}

/// Berechnet den vollstaendigen Bauplan eines Pruefordners.
///
/// Reine Rechnung, ohne Zugriff auf das Dateisystem. Das ist der Grund, aus dem
/// sich die Reproduzierbarkeit pruefen laesst, ohne zweimal 100.000 Dateien
/// anzulegen.
pub fn bauplan(anzahl: usize, startwert: u64) -> Vec<Bauplan> {
    let mut zufall = Zufall::neu(startwert);
    let mut plaene = Vec::with_capacity(anzahl);

    for nummer in 0..anzahl {
        let wurf = zufall.unter(1000) as u32;
        let art = if wurf < ANTEIL_ORDNER {
            Art::Ordner
        } else if wurf < ANTEIL_ORDNER + ANTEIL_VERKNUEPFUNG {
            Art::Verknuepfung
        } else {
            Art::Datei
        };

        let stamm = STAEMME[zufall.unter(STAEMME.len() as u64) as usize];
        let marke = zufall.naechste() & 0xFFFF_FFFF;
        // Die Zufallsmarke steht vor der laufenden Nummer, damit die
        // Namensordnung nicht mit der Anlegereihenfolge zusammenfaellt. Die
        // laufende Nummer am Ende macht den Namen einmalig, ohne dass der
        // Erzeuger eine Kollisionspruefung braucht.
        let name = match art {
            Art::Ordner => format!("{stamm}-{marke:08x}-{nummer:06}"),
            Art::Verknuepfung => format!("{stamm}-{marke:08x}-{nummer:06}.lnk"),
            Art::Datei => {
                let endung = ENDUNGEN[zufall.unter(ENDUNGEN.len() as u64) as usize];
                format!("{stamm}-{marke:08x}-{nummer:06}{endung}")
            }
        };

        let groesse = match art {
            Art::Datei => groesse_ziehen(&mut zufall),
            _ => 0,
        };

        let geaendert =
            UNIX_EPOCH + Duration::from_secs(zufall.zwischen(ZEITBASIS, ZEITBASIS + ZEITSPANNE));

        plaene.push(Bauplan {
            name,
            art,
            groesse,
            geaendert,
            ziel: None,
        });
    }

    ziele_zuweisen(&mut plaene, startwert);
    plaene
}

/// Zieht eine Groesse aus den gewichteten Klassen.
fn groesse_ziehen(zufall: &mut Zufall) -> u64 {
    let wurf = zufall.unter(1000) as u32;
    let mut schwelle = 0;
    for (anteil, von, bis) in GROESSENKLASSEN {
        schwelle += anteil;
        if wurf < schwelle {
            return if bis <= von + 1 {
                von
            } else {
                zufall.zwischen(von, bis)
            };
        }
    }
    // Nur erreichbar, wenn die Anteile nicht auf 1.000 aufgehen. Der Test
    // `die_groessenklassen_gehen_auf_tausend_auf` haelt das fest.
    unreachable!("die Groessenklassen decken den Wurf nicht ab");
}

/// Weist jeder Verknuepfung ein Ziel aus demselben Ordner zu.
///
/// Zweiter Durchgang, weil eine Verknuepfung auch auf einen Eintrag zeigen
/// koennen soll, der erst spaeter im Bauplan steht. Der eigene Startwert haelt
/// diesen Durchgang vom ersten getrennt: waere es derselbe Generator, wuerde
/// jede Aenderung an der Zahl der Verknuepfungen auch alle Namen verschieben.
fn ziele_zuweisen(plaene: &mut [Bauplan], startwert: u64) {
    let dateien: Vec<String> = plaene
        .iter()
        .filter(|plan| plan.art == Art::Datei)
        .map(|plan| plan.name.clone())
        .collect();
    if dateien.is_empty() {
        // Ein Ordner ohne eine einzige Datei kommt nur bei sehr kleinen
        // Eintragszahlen vor. Dann zeigt die Verknuepfung auf einen Namen, den
        // es nicht gibt; der Leser folgt ihr ohnehin nicht.
        for plan in plaene
            .iter_mut()
            .filter(|plan| plan.art == Art::Verknuepfung)
        {
            plan.ziel = Some("kein-ziel".to_owned());
        }
        return;
    }

    let mut zufall = Zufall::neu(startwert ^ 0x5A5A_5A5A_5A5A_5A5A);
    for plan in plaene
        .iter_mut()
        .filter(|plan| plan.art == Art::Verknuepfung)
    {
        let treffer = zufall.unter(dateien.len() as u64) as usize;
        plan.ziel = Some(dateien[treffer].clone());
    }
}

// ---------------------------------------------------------------------------
// Das Anlegen
// ---------------------------------------------------------------------------

/// Was ein Lauf des Erzeugers hinterlassen hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erzeugt {
    /// Der angelegte Ordner.
    pub ordner: PathBuf,
    /// Der danebengelegte Steckbrief.
    pub steckbrief: PathBuf,
    /// Wie viele Eintraege der Ordner traegt.
    pub eintraege: usize,
    /// Davon Dateien.
    pub dateien: usize,
    /// Davon Unterordner.
    pub ordnerzahl: usize,
    /// Davon symbolische Verknuepfungen.
    pub verknuepfungen: usize,
    /// Die Summe der genannten Groessen. Auf der Platte liegt wegen der
    /// duennen Besetzung weniger.
    pub summe_groessen: u64,
}

/// Legt den Pruefordner an.
///
/// Der Zielordner darf fehlen oder leer sein. Ein Ordner mit Inhalt wird
/// **nicht** ueberschrieben: ein halb ueberschriebener Pruefordner waere
/// weder der alte noch der neue, und eine Messung darauf waere wertlos.
pub fn erzeugen(ziel: &Path, anzahl: usize, startwert: u64) -> io::Result<Erzeugt> {
    pruefen_dass_leer(ziel)?;
    fs::create_dir_all(ziel)?;

    let plaene = bauplan(anzahl, startwert);
    let mut verknuepfungen: Vec<PathBuf> = Vec::new();
    let mut erzeugt = Erzeugt {
        ordner: ziel.to_path_buf(),
        steckbrief: steckbriefpfad(ziel)?,
        eintraege: plaene.len(),
        dateien: 0,
        ordnerzahl: 0,
        verknuepfungen: 0,
        summe_groessen: 0,
    };

    for plan in &plaene {
        let pfad = ziel.join(&plan.name);
        match plan.art {
            Art::Datei => {
                datei_anlegen(&pfad, plan.groesse, plan.geaendert)?;
                erzeugt.dateien += 1;
                erzeugt.summe_groessen += plan.groesse;
            }
            Art::Ordner => {
                fs::create_dir(&pfad)?;
                zeit_setzen(&pfad, plan.geaendert)?;
                erzeugt.ordnerzahl += 1;
            }
            Art::Verknuepfung => {
                let zielname = plan.ziel.as_deref().unwrap_or("kein-ziel");
                std::os::unix::fs::symlink(zielname, &pfad)?;
                verknuepfungen.push(pfad);
                erzeugt.verknuepfungen += 1;
            }
        }
    }

    verknuepfungszeiten_setzen(ziel, &verknuepfungen, startwert)?;

    // Der Ordner selbst zuletzt: jeder angelegte Eintrag hat sein
    // Aenderungsdatum bis hierher fortgeschrieben.
    zeit_setzen(ziel, UNIX_EPOCH + Duration::from_secs(ZEITBASIS))?;

    steckbrief_schreiben(&erzeugt, startwert)?;
    Ok(erzeugt)
}

/// Bricht ab, wenn im Zielordner schon etwas liegt.
fn pruefen_dass_leer(ziel: &Path) -> io::Result<()> {
    match fs::read_dir(ziel) {
        Ok(mut eintraege) => {
            if eintraege.next().is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "{} ist nicht leer. Der Erzeuger ueberschreibt nichts; \
                         raeume den Ordner ab oder waehle einen anderen Pfad.",
                        ziel.display()
                    ),
                ));
            }
            Ok(())
        }
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(fehler) => Err(fehler),
    }
}

/// Legt eine Datei mit der genannten Groesse und dem genannten Datum an.
fn datei_anlegen(pfad: &Path, groesse: u64, geaendert: SystemTime) -> io::Result<()> {
    let datei = File::create(pfad)?;
    let echte = groesse.min(ECHTE_BYTES);
    if echte > 0 {
        let mut schreiber = &datei;
        // Ein wiederkehrendes Muster statt Nullbytes, damit der Kopf der Datei
        // nicht selbst wie ein Loch aussieht.
        let muster: Vec<u8> = (0..echte)
            .map(|stelle| b'a' + (stelle % 26) as u8)
            .collect();
        schreiber.write_all(&muster)?;
    }
    if groesse > echte {
        datei.set_len(groesse)?;
    }
    datei.set_times(zeiten(geaendert))?;
    Ok(())
}

/// Setzt das Aenderungsdatum eines Ordners.
fn zeit_setzen(pfad: &Path, geaendert: SystemTime) -> io::Result<()> {
    let griff = File::open(pfad)?;
    griff.set_times(zeiten(geaendert))
}

fn zeiten(geaendert: SystemTime) -> FileTimes {
    FileTimes::new()
        .set_modified(geaendert)
        .set_accessed(geaendert)
}

/// Setzt das Aenderungsdatum aller Verknuepfungen auf einen festen Zeitpunkt.
///
/// `File::set_times` folgt der Verknuepfung und traefe das Ziel statt die
/// Verknuepfung selbst. Den Unterschied kennt auf dieser Ebene nur `lutimes`,
/// und das hiesse eine Bindung an `libc` samt `unsafe` in einem Werkzeug, das
/// sonst ohne beides auskommt. Stattdessen uebernimmt `touch -h -r` die Zeit
/// einer Marke, die vorher gesetzt wird.
///
/// Alle Verknuepfungen eines Pruefordners tragen deshalb **dasselbe** Datum,
/// waehrend die Dateien gestreute Daten tragen. Fuer eine Messstrecke, die
/// Verzeichnismetadaten liest, ist das ohne Belang; fuer die Sortierprobe des
/// Kerns ist es keine Grundlage, die liegt in `krk-core/tests/verzeichnis.rs`.
fn verknuepfungszeiten_setzen(
    ziel: &Path,
    verknuepfungen: &[PathBuf],
    startwert: u64,
) -> io::Result<()> {
    if verknuepfungen.is_empty() {
        return Ok(());
    }

    let marke = markenpfad(ziel)?;
    let mut zufall = Zufall::neu(startwert ^ 0x3C3C_3C3C_3C3C_3C3C);
    let zeitpunkt =
        UNIX_EPOCH + Duration::from_secs(zufall.zwischen(ZEITBASIS, ZEITBASIS + ZEITSPANNE));
    let datei = File::create(&marke)?;
    datei.set_times(zeiten(zeitpunkt))?;
    drop(datei);

    let ergebnis = touch_laufen_lassen(&marke, verknuepfungen);
    let _ = fs::remove_file(&marke);
    ergebnis
}

fn touch_laufen_lassen(marke: &Path, verknuepfungen: &[PathBuf]) -> io::Result<()> {
    for buendel in verknuepfungen.chunks(TOUCH_BUENDEL) {
        let ausgang = Command::new("/usr/bin/touch")
            .arg("-h")
            .arg("-r")
            .arg(marke)
            .args(buendel)
            .status()?;
        if !ausgang.success() {
            return Err(io::Error::other(format!(
                "/usr/bin/touch -h -r ist mit {ausgang} gescheitert; \
                 die Verknuepfungen tragen kein festes Aenderungsdatum"
            )));
        }
    }
    Ok(())
}

/// Der Pfad der Zeitmarke: neben dem Pruefordner, nicht darin.
///
/// Nichts darf in den Pruefordner geraten, was nicht im Bauplan steht. Die
/// Eintragszahl ist eine Zusage, und eine Hilfsdatei mehr macht aus 10.000
/// Eintraegen 10.001.
fn markenpfad(ziel: &Path) -> io::Result<PathBuf> {
    nebenpfad(ziel, "zeitmarke")
}

// ---------------------------------------------------------------------------
// Der Steckbrief
// ---------------------------------------------------------------------------

/// Was neben einem Pruefordner ueber ihn festgehalten ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Steckbrief {
    /// Die Zahl der Eintraege.
    pub eintraege: usize,
    /// Der Startwert, aus dem der Ordner entstanden ist.
    pub startwert: u64,
}

/// Der Pfad des Steckbriefs zu einem Pruefordner.
pub fn steckbriefpfad(ordner: &Path) -> io::Result<PathBuf> {
    nebenpfad(ordner, "pruefordner.toml")
}

/// Baut einen Pfad neben dem Pruefordner, mit der genannten Endung.
fn nebenpfad(ordner: &Path, endung: &str) -> io::Result<PathBuf> {
    let name = ordner.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} hat keinen Namen, neben den sich etwas legen liesse",
                ordner.display()
            ),
        )
    })?;
    let mut dateiname = name.to_os_string();
    dateiname.push(format!(".{endung}"));
    Ok(ordner.with_file_name(dateiname))
}

/// Schreibt den Steckbrief neben den Pruefordner.
///
/// Er ist die Antwort auf eine Frage, die der Berichtskopf der Messstrecke
/// stellt: zu welchem Startwert gehoert dieser Ordner? Der Unterbefehl
/// `messen` bekommt nur einen Pfad, und ein Pfad traegt den Startwert nicht.
/// Der Steckbrief liegt **neben** dem Ordner und nicht darin, damit die
/// Eintragszahl stimmt.
fn steckbrief_schreiben(erzeugt: &Erzeugt, startwert: u64) -> io::Result<()> {
    let inhalt = format!(
        "# Steckbrief eines KRK-Pruefordners, geschrieben von krk-bench {version}.\n\
         # Der Ordner ist aus Eintragszahl und Startwert reproduzierbar:\n\
         #   cargo run -p krk-bench -- fixture --eintraege {eintraege} --seed {startwert} --out {ordner}\n\
         ordner = \"{ordner}\"\n\
         eintraege = {eintraege}\n\
         startwert = {startwert}\n\
         dateien = {dateien}\n\
         unterordner = {unterordner}\n\
         verknuepfungen = {verknuepfungen}\n\
         summe_groessen = {summe}\n",
        version = env!("CARGO_PKG_VERSION"),
        ordner = erzeugt.ordner.display(),
        eintraege = erzeugt.eintraege,
        startwert = startwert,
        dateien = erzeugt.dateien,
        unterordner = erzeugt.ordnerzahl,
        verknuepfungen = erzeugt.verknuepfungen,
        summe = erzeugt.summe_groessen,
    );
    fs::write(&erzeugt.steckbrief, inhalt)
}

/// Liest den Steckbrief neben einem Pruefordner, falls es einen gibt.
///
/// Liefert `None`, wenn keiner daliegt. Das ist kein Fehler: die Messstrecke
/// muss auch auf einen Ordner zeigen koennen, den sie nicht selbst erzeugt hat.
/// Der Berichtskopf schreibt den Startwert dann als unbekannt aus, statt einen
/// zu erfinden.
pub fn steckbrief_lesen(ordner: &Path) -> Option<Steckbrief> {
    let pfad = steckbriefpfad(ordner).ok()?;
    let inhalt = fs::read_to_string(pfad).ok()?;
    let mut eintraege = None;
    let mut startwert = None;
    for zeile in inhalt.lines() {
        let Some((schluessel, wert)) = zeile.split_once('=') else {
            continue;
        };
        let schluessel = schluessel.trim();
        let wert = wert.trim().trim_matches('"');
        match schluessel {
            "eintraege" => eintraege = wert.parse().ok(),
            "startwert" => startwert = wert.parse().ok(),
            _ => {}
        }
    }
    Some(Steckbrief {
        eintraege: eintraege?,
        startwert: startwert?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicU64, Ordering};

    static ZAEHLER: AtomicU64 = AtomicU64::new(0);

    /// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
    struct Wegwerfordner {
        pfad: PathBuf,
    }

    impl Wegwerfordner {
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let mut pfad = std::env::temp_dir();
            pfad.push(format!(
                "krk-bench-test-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&pfad);
            Self { pfad }
        }

        fn pfad(&self) -> &Path {
            &self.pfad
        }
    }

    impl Drop for Wegwerfordner {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.pfad);
            if let Ok(steckbrief) = steckbriefpfad(&self.pfad) {
                let _ = fs::remove_file(steckbrief);
            }
        }
    }

    #[test]
    fn die_groessenklassen_gehen_auf_tausend_auf() {
        let summe: u32 = GROESSENKLASSEN.iter().map(|(anteil, _, _)| anteil).sum();
        assert_eq!(
            summe, 1000,
            "die Groessenklassen decken nicht jeden Wurf ab"
        );
        const { assert!(ANTEIL_ORDNER + ANTEIL_VERKNUEPFUNG < 1000) };
    }

    #[test]
    fn derselbe_startwert_liefert_denselben_bauplan() {
        assert_eq!(bauplan(2_000, 1), bauplan(2_000, 1));
        assert_eq!(bauplan(2_000, 3), bauplan(2_000, 3));
    }

    #[test]
    fn startwert_zwei_liefert_eine_andere_liste_als_startwert_eins() {
        let a = bauplan(10_000, 1);
        let b = bauplan(10_000, 2);
        assert_eq!(a.len(), b.len());
        assert_ne!(a, b, "die beiden Pruefordner waeren nicht unterscheidbar");

        let namen_a: HashSet<&str> = a.iter().map(|plan| plan.name.as_str()).collect();
        let namen_b: HashSet<&str> = b.iter().map(|plan| plan.name.as_str()).collect();
        let gemeinsam = namen_a.intersection(&namen_b).count();
        assert!(
            gemeinsam * 100 < a.len(),
            "die Namenslisten ueberschneiden sich in {gemeinsam} von {} Namen; \
             ein gemeinsamer Ordner laege beim zweiten Lesevorgang im Cache",
            a.len()
        );
    }

    #[test]
    fn der_bauplan_mischt_die_drei_arten_und_die_groessen() {
        let plaene = bauplan(10_000, 1);
        let zahl = |gesucht: Art| plaene.iter().filter(|plan| plan.art == gesucht).count();
        assert!(zahl(Art::Ordner) > 0, "kein Unterordner im Bauplan");
        assert!(zahl(Art::Verknuepfung) > 0, "keine Verknuepfung im Bauplan");
        assert!(zahl(Art::Datei) > 9_000, "zu wenige Dateien im Bauplan");

        let groessen: HashSet<u64> = plaene.iter().map(|plan| plan.groesse).collect();
        assert!(groessen.len() > 1_000, "die Groessen streuen zu wenig");
        assert!(
            plaene.iter().any(|plan| plan.groesse == 0),
            "keine leere Datei"
        );
        assert!(
            plaene.iter().any(|plan| plan.groesse > 1024 * 1024),
            "keine grosse Datei"
        );

        let endungen: HashSet<&str> = plaene
            .iter()
            .filter(|plan| plan.art == Art::Datei)
            .map(|plan| {
                plan.name
                    .rsplit_once('.')
                    .map(|(_, endung)| endung)
                    .unwrap_or("")
            })
            .collect();
        assert!(endungen.len() > 5, "zu wenige verschiedene Dateitypen");
    }

    #[test]
    fn namen_sind_einmalig() {
        let plaene = bauplan(20_000, 7);
        let einmalig: HashSet<&str> = plaene.iter().map(|plan| plan.name.as_str()).collect();
        assert_eq!(
            einmalig.len(),
            plaene.len(),
            "zwei Eintraege heissen gleich"
        );
    }

    #[test]
    fn jede_verknuepfung_zeigt_auf_einen_eintrag_desselben_ordners() {
        let plaene = bauplan(5_000, 4);
        let namen: HashSet<&str> = plaene.iter().map(|plan| plan.name.as_str()).collect();
        let mut geprueft = 0;
        for plan in plaene.iter().filter(|plan| plan.art == Art::Verknuepfung) {
            let ziel = plan.ziel.as_deref().expect("Verknuepfung ohne Ziel");
            assert!(namen.contains(ziel), "{ziel} gibt es im Ordner nicht");
            geprueft += 1;
        }
        assert!(geprueft > 0, "die Probe hat keine Verknuepfung gesehen");
    }

    /// Die Kernprobe: der erzeugte Ordner wird nicht geglaubt, sondern
    /// nachgemessen — Eintrag fuer Eintrag gegen `std::fs` und zusaetzlich
    /// gegen den Verzeichnisleser aus `krk-core`.
    #[test]
    fn der_erzeugte_ordner_traegt_genau_das_was_der_bauplan_sagt() {
        let ordner = Wegwerfordner::neu("querprobe");
        let erzeugt = erzeugen(ordner.pfad(), 600, 1).expect("Erzeugen gescheitert");
        let plaene = bauplan(600, 1);

        assert_eq!(erzeugt.eintraege, 600);
        assert_eq!(
            erzeugt.dateien + erzeugt.ordnerzahl + erzeugt.verknuepfungen,
            600,
            "die Zaehlung des Erzeugers geht nicht auf"
        );

        let im_dateisystem = fs::read_dir(ordner.pfad())
            .expect("Pruefordner laesst sich nicht lesen")
            .count();
        assert_eq!(
            im_dateisystem, 600,
            "im Ordner liegen {im_dateisystem} Eintraege statt 600"
        );

        for plan in &plaene {
            let pfad = ordner.pfad().join(&plan.name);
            let angaben = fs::symlink_metadata(&pfad)
                .unwrap_or_else(|_| panic!("{} gibt es nicht", plan.name));

            match plan.art {
                Art::Datei => {
                    assert!(angaben.is_file(), "{} ist keine Datei", plan.name);
                    assert_eq!(
                        angaben.len(),
                        plan.groesse,
                        "{} traegt die falsche Groesse",
                        plan.name
                    );
                }
                Art::Ordner => assert!(angaben.is_dir(), "{} ist kein Ordner", plan.name),
                Art::Verknuepfung => {
                    assert!(angaben.is_symlink(), "{} ist keine Verknuepfung", plan.name);
                    let ziel = fs::read_link(&pfad).expect("Ziel nicht lesbar");
                    assert_eq!(
                        ziel.to_string_lossy(),
                        plan.ziel.clone().unwrap_or_default(),
                        "{} zeigt woandershin",
                        plan.name
                    );
                }
            }

            if plan.art != Art::Verknuepfung {
                let gesetzt = angaben.modified().expect("kein Aenderungsdatum");
                assert_eq!(
                    gesetzt, plan.geaendert,
                    "{} traegt das falsche Aenderungsdatum",
                    plan.name
                );
            }
        }

        // Und dasselbe noch einmal durch die Augen des Lesers, der spaeter
        // gemessen wird: was der Erzeuger hinlegt, muss krk-core auch sehen.
        let gelesen = krk_core::verzeichnis::lesen(ordner.pfad()).expect("Lesen gescheitert");
        assert_eq!(
            gelesen.len(),
            600,
            "der Leser sieht eine andere Eintragszahl"
        );

        let mut nach_namen: Vec<(&str, u64)> = gelesen
            .iter()
            .filter(|eintrag| eintrag.typ == krk_core::verzeichnis::Typ::Datei)
            .map(|eintrag| (eintrag.name.as_str(), eintrag.groesse))
            .collect();
        nach_namen.sort_unstable();
        let mut erwartet: Vec<(&str, u64)> = plaene
            .iter()
            .filter(|plan| plan.art == Art::Datei)
            .map(|plan| (plan.name.as_str(), plan.groesse))
            .collect();
        erwartet.sort_unstable();
        assert_eq!(
            nach_namen, erwartet,
            "der Leser meldet andere Namen oder Groessen als der Bauplan"
        );
    }

    #[test]
    fn zwei_ordner_aus_demselben_startwert_sind_nicht_zu_unterscheiden() {
        let eins = Wegwerfordner::neu("gleich-a");
        let zwei = Wegwerfordner::neu("gleich-b");
        erzeugen(eins.pfad(), 400, 1).expect("Erzeugen gescheitert");
        erzeugen(zwei.pfad(), 400, 1).expect("Erzeugen gescheitert");

        assert_eq!(auflistung(eins.pfad()), auflistung(zwei.pfad()));
    }

    #[test]
    fn zwei_ordner_aus_verschiedenen_startwerten_sind_zu_unterscheiden() {
        let eins = Wegwerfordner::neu("verschieden-a");
        let zwei = Wegwerfordner::neu("verschieden-b");
        erzeugen(eins.pfad(), 400, 1).expect("Erzeugen gescheitert");
        erzeugen(zwei.pfad(), 400, 2).expect("Erzeugen gescheitert");

        assert_ne!(auflistung(eins.pfad()), auflistung(zwei.pfad()));
    }

    /// Namen, Groessen und Aenderungsdaten, wie das Dateisystem sie meldet.
    fn auflistung(ordner: &Path) -> Vec<(String, u64, SystemTime)> {
        let mut zeilen: Vec<(String, u64, SystemTime)> = fs::read_dir(ordner)
            .expect("Ordner nicht lesbar")
            .map(|eintrag| {
                let eintrag = eintrag.expect("Eintrag nicht lesbar");
                let angaben = eintrag.metadata().expect("keine Angaben");
                (
                    eintrag.file_name().to_string_lossy().into_owned(),
                    angaben.len(),
                    angaben.modified().expect("kein Datum"),
                )
            })
            .collect();
        zeilen.sort();
        zeilen
    }

    #[test]
    fn der_ordner_selbst_traegt_ein_festes_aenderungsdatum() {
        let ordner = Wegwerfordner::neu("ordnerdatum");
        erzeugen(ordner.pfad(), 50, 9).expect("Erzeugen gescheitert");
        let angaben = fs::metadata(ordner.pfad()).expect("keine Angaben");
        assert_eq!(
            angaben.modified().expect("kein Datum"),
            UNIX_EPOCH + Duration::from_secs(ZEITBASIS),
            "ohne festes Ordnerdatum ist `ls -la` nicht vergleichbar"
        );
    }

    #[test]
    fn alle_verknuepfungen_tragen_dasselbe_feste_datum() {
        let ordner = Wegwerfordner::neu("verknuepfungsdatum");
        erzeugen(ordner.pfad(), 1_000, 5).expect("Erzeugen gescheitert");

        let daten: HashSet<SystemTime> = fs::read_dir(ordner.pfad())
            .expect("Ordner nicht lesbar")
            .filter_map(|eintrag| {
                let eintrag = eintrag.ok()?;
                let angaben = fs::symlink_metadata(eintrag.path()).ok()?;
                angaben.is_symlink().then(|| angaben.modified().ok())?
            })
            .collect();
        assert_eq!(daten.len(), 1, "die Verknuepfungen tragen gestreute Daten");
        let einziges = *daten.iter().next().expect("keine Verknuepfung gefunden");
        assert!(
            einziges < UNIX_EPOCH + Duration::from_secs(ZEITBASIS + ZEITSPANNE),
            "das Datum liegt ausserhalb der geplanten Spanne"
        );
    }

    #[test]
    fn ein_ordner_mit_inhalt_wird_nicht_ueberschrieben() {
        let ordner = Wegwerfordner::neu("nicht-ueberschreiben");
        erzeugen(ordner.pfad(), 20, 1).expect("Erzeugen gescheitert");
        let fehler = erzeugen(ordner.pfad(), 20, 1).expect_err("das haette scheitern muessen");
        assert_eq!(fehler.kind(), io::ErrorKind::AlreadyExists);
    }

    #[test]
    fn der_steckbrief_liegt_neben_dem_ordner_und_nennt_den_startwert() {
        let ordner = Wegwerfordner::neu("steckbrief");
        let erzeugt = erzeugen(ordner.pfad(), 120, 42).expect("Erzeugen gescheitert");

        assert_eq!(erzeugt.steckbrief.parent(), ordner.pfad().parent());
        assert!(erzeugt.steckbrief.exists(), "kein Steckbrief geschrieben");

        let gelesen = steckbrief_lesen(ordner.pfad()).expect("Steckbrief nicht lesbar");
        assert_eq!(gelesen.startwert, 42);
        assert_eq!(gelesen.eintraege, 120);

        // Und er faelscht die Eintragszahl nicht, weil er ausserhalb liegt.
        assert_eq!(
            fs::read_dir(ordner.pfad()).expect("nicht lesbar").count(),
            120
        );
    }

    #[test]
    fn ohne_steckbrief_gibt_es_keinen_erfundenen_startwert() {
        let ordner = Wegwerfordner::neu("ohne-steckbrief");
        fs::create_dir_all(ordner.pfad()).expect("Ordner nicht anlegbar");
        assert_eq!(steckbrief_lesen(ordner.pfad()), None);
    }
}
