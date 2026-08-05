//! Der Messbericht.
//!
//! Ein Bericht ohne seinen Bedingungskopf gilt laut Plan als nicht vorhanden.
//! Der Grund ist einfach: eine Zahl ohne die Bedingungen, unter denen sie
//! entstanden ist, laesst sich weder nachvollziehen noch widerlegen. Der Kopf
//! traegt deshalb Zeitpunkt, `hw.model`, `sw_vers`, Bildwiederholrate,
//! Cache-Zustand, Wiederholungszahl sowie Pfad und Startwert des Pruefordners.
//!
//! Ausgewiesen wird je Messgroesse das 95. Perzentil, wie C8 es verlangt, und
//! daneben Median und Minimum. Die drei Zahlen zusammen machen einen Ausreisser
//! sichtbar; das Perzentil allein taete es nicht.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::messen::{self, ANTEIL_IM_BILD_PROZENT, Gesamtergebnis, Gesamtlauf, Messreihe, Zusage};

/// Der Ordner, in dem die Berichte liegen.
pub const MESSUNGEN: &str = "messungen";

/// Die Bedingungen, unter denen gemessen wurde.
#[derive(Debug, Clone)]
pub struct Kopf {
    /// Der Zeitpunkt der Messung, in UTC.
    pub zeitpunkt: String,
    /// `sysctl -n hw.model`.
    pub hw_model: String,
    /// `sw_vers`, zu einer Zeile zusammengezogen.
    pub sw_vers: String,
    /// Die Bildwiederholrate des Hauptbildschirms.
    pub bildwiederholrate: String,
    /// Der Startwert des Pruefordners, aus seinem Steckbrief.
    pub startwert: String,
    /// Was der Steckbrief ueber die Eintragszahl sagt.
    pub eintraege_laut_steckbrief: String,
}

impl Kopf {
    /// Sammelt die Bedingungen vom laufenden Geraet ein.
    pub fn erheben(ordner: &Path) -> Self {
        let steckbrief = crate::fixture::steckbrief_lesen(ordner);
        Self {
            zeitpunkt: zeitstempel(SystemTime::now()),
            hw_model: befehl_ausgabe("/usr/sbin/sysctl", &["-n", "hw.model"]),
            sw_vers: betriebssystem(),
            bildwiederholrate: bildwiederholrate(),
            startwert: steckbrief
                .as_ref()
                .map(|brief| brief.startwert.to_string())
                .unwrap_or_else(unbekannt),
            eintraege_laut_steckbrief: steckbrief
                .as_ref()
                .map(|brief| brief.eintraege.to_string())
                .unwrap_or_else(unbekannt),
        }
    }
}

fn unbekannt() -> String {
    "unbekannt (kein Steckbrief neben dem Ordner)".to_owned()
}

/// Ob der Bericht aus einem Bau mit oder ohne Optimierung stammt.
///
/// Der Unterschied macht bei einem Verzeichnisleser leicht den Faktor fuenf
/// aus. Eine Zahl, die nicht sagt, aus welchem Bau sie stammt, ist gegen eine
/// Zusage nicht abnehmbar; deshalb steht die Bauart im Kopf.
pub fn bauart() -> &'static str {
    if cfg!(debug_assertions) {
        "Bau ohne Optimierung (debug) — nicht zur Abnahme einer Zusage geeignet"
    } else {
        "Bau mit Optimierung (release)"
    }
}

/// Setzt den Bericht zusammen.
pub fn verfassen(reihe: &Messreihe, kopf: &Kopf) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "KRK — Messbericht der kopflosen Strecke (Schritt 3)");
    let _ = writeln!(text, "====================================================");
    let _ = writeln!(text);

    let _ = writeln!(text, "Bedingungen");
    let _ = writeln!(text, "-----------");
    let mut zeile = |name: &str, wert: &str| {
        let _ = writeln!(text, "{name:<22}{wert}");
    };
    zeile("Zeitpunkt", &kopf.zeitpunkt);
    zeile("hw.model", &kopf.hw_model);
    zeile("sw_vers", &kopf.sw_vers);
    zeile("Bildwiederholrate", &kopf.bildwiederholrate);
    zeile("Cache-Zustand", reihe.cache.beschreibung());
    zeile("Wiederholungen", &reihe.wiederholungen.to_string());
    zeile("Pruefordner", &reihe.ordner.display().to_string());
    zeile("Startwert", &kopf.startwert);
    zeile(
        "Eintraege je Lauf",
        &format!(
            "{} (laut Steckbrief: {})",
            reihe.eintraege, kopf.eintraege_laut_steckbrief
        ),
    );
    zeile(
        "Werkzeug",
        &format!(
            "krk-bench {}, Ziel {}-{}, {}",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::ARCH,
            std::env::consts::OS,
            bauart()
        ),
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Zahlen");
    let _ = writeln!(text, "------");
    let _ = writeln!(
        text,
        "{:<50}{:>16}{:>14}{:>14}",
        "Messgroesse", "95. Perzentil", "Median", "Minimum"
    );
    for groesse in &reihe.groessen {
        let _ = writeln!(
            text,
            "{:<50}{:>16}{:>14}{:>14}",
            format!("{} ({})", groesse.name, groesse.zusagen),
            spanne(groesse.perzentil95()),
            spanne(groesse.median()),
            spanne(groesse.minimum())
        );
    }
    let _ = writeln!(text);

    let _ = writeln!(text, "Einzelwerte");
    let _ = writeln!(text, "-----------");
    for groesse in &reihe.groessen {
        let _ = writeln!(text, "{}:", groesse.name);
        let werte: Vec<String> = groesse.werte.iter().copied().map(spanne).collect();
        for buendel in werte.chunks(5) {
            let _ = writeln!(text, "  {}", buendel.join("  "));
        }
    }
    let _ = writeln!(text);

    let _ = writeln!(text, "Lesart");
    let _ = writeln!(text, "------");
    let _ = writeln!(
        text,
        "Diese Strecke misst ohne Fenster und ohne AppKit. \"Lesen bis zum ersten"
    );
    let _ = writeln!(
        text,
        "Stapel\" ist der Anteil des Kerns an L2, nicht L2 selbst: was das Zeichnen"
    );
    let _ = writeln!(
        text,
        "dazulegt, misst erst der Messmodus der Anwendung (Schritt 21). L1, L4 bis"
    );
    let _ = writeln!(
        text,
        "L9 kommen auf dieser Strecke nicht vor. Das 95. Perzentil ist der Wert"
    );
    let _ = writeln!(
        text,
        "des naechsten Rangs, nicht interpoliert: bei zwanzig Laeufen der"
    );
    let _ = writeln!(text, "neunzehnte der sortierten Reihe.");
    text
}

/// Schreibt den Bericht in den Messungenordner und liefert seinen Pfad.
pub fn schreiben(ziel: &Path, reihe: &Messreihe, text: &str) -> io::Result<PathBuf> {
    if !ziel.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} gibt es nicht. Rufe das Werkzeug aus dem Projektwurzelverzeichnis \
                 auf oder nenne den Ordner ueber --ziel.",
                ziel.display()
            ),
        ));
    }

    let kennung = kurzstempel(SystemTime::now());
    let ordnername = reihe
        .ordner
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "ordner".to_owned());
    let zustand = match reihe.cache {
        crate::messen::Cache::Kalt => "kalt",
        crate::messen::Cache::Warm => "warm",
    };
    let pfad = ziel.join(format!("{kennung}-kopflos-{ordnername}-{zustand}.txt"));
    fs::write(&pfad, text)?;
    Ok(pfad)
}

// ---------------------------------------------------------------------------
// Der Abnahmebericht ueber alle zehn Zusagen (Schritt 21)
// ---------------------------------------------------------------------------

/// Setzt den Abnahmebericht ueber alle zehn Zusagen zusammen.
///
/// Der Bedingungskopf traegt die neun verlangten Angaben, darunter die
/// Bildwiederholrate als Zahl aus `NSScreen` und — als neunte Angabe seit dem
/// 260804-2318 — die Systemlast vor und nach dem Lauf, an der der
/// L4-Streuungsvergleich aus S22 pruefbar wird.
pub fn gesamt_verfassen(lauf: &Gesamtlauf, ergebnis: &Gesamtergebnis) -> String {
    let mut text = String::new();
    let _ = writeln!(
        text,
        "KRK — Abnahmebericht ueber die zehn Zusagen aus C8 (Schritt 21)"
    );
    let _ = writeln!(
        text,
        "==============================================================="
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Bedingungen");
    let _ = writeln!(text, "-----------");
    let mut zeile = |name: &str, wert: &str| {
        let _ = writeln!(text, "{name:<22}{wert}");
    };
    zeile("Zeitpunkt", &zeitstempel(SystemTime::now()));
    zeile(
        "hw.model",
        &befehl_ausgabe("/usr/sbin/sysctl", &["-n", "hw.model"]),
    );
    zeile("sw_vers", &betriebssystem());
    zeile(
        "Bildwiederholrate",
        &format!(
            "{} Hz, gelesen aus NSScreen.maximumFramesPerSecond am Bildschirm des \
             gemessenen Fensters; eine Bildlaenge sind damit {}",
            ergebnis.bildwiederholrate,
            spanne(ergebnis.bildlaenge)
        ),
    );
    zeile(
        "Cache-Zustand",
        "warm (purge braucht Rechte, die dieser Lauf nicht hat; L4 ist damit eine \
         Untergrenze der Kaltstart-Zusage, siehe Einschraenkungen)",
    );
    zeile(
        "Wiederholungen",
        &format!(
            "{} je Zusage und Runde, {} Runden",
            lauf.wiederholungen, lauf.runden
        ),
    );
    zeile("Pruefordner A", &messen::ordner_beschreiben(&lauf.ordner_a));
    zeile("Pruefordner B", &messen::ordner_beschreiben(&lauf.ordner_b));
    zeile(
        "Pruefordner 100k",
        &messen::ordner_beschreiben(&lauf.ordner100k),
    );
    zeile(
        "Unterordner L6",
        &messen::ordner_beschreiben(&ergebnis.unterordner),
    );
    zeile(
        "Kopierziel L8/L9",
        &format!(
            "{} (derselbe APFS-Datentraeger wie Pruefordner A, geprueft; \
             Klonweg, keine Durchsatzmessung)",
            lauf.kopierziel.display()
        ),
    );
    zeile("Pruefsitzung", PRUEFSITZUNG);
    zeile(
        "Systemlast",
        &format!(
            "vor dem Lauf {}, nach dem Lauf {} (sysctl vm.loadavg)",
            ergebnis.systemlast_vorher, ergebnis.systemlast_nachher
        ),
    );
    zeile("Gemessenes Buendel", &lauf.programm.display().to_string());
    zeile(
        "Werkzeug",
        &format!(
            "krk-bench {}, Ziel {}-{}, {}",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::ARCH,
            std::env::consts::OS,
            bauart()
        ),
    );
    let _ = writeln!(text);

    let _ = writeln!(text, "Zahlen");
    let _ = writeln!(text, "------");
    let _ = writeln!(
        text,
        "Zwei Abnahmemasse stehen nebeneinander, die Spalte \"Abnahme nach\" nennt je"
    );
    let _ = writeln!(
        text,
        "Zeile, welches gilt: das 95. Perzentil der Runde fuer die Dauerzusagen, der"
    );
    let _ = writeln!(
        text,
        "Anteil der Eingaben im naechsten Bild fuer L1 und L9. Das Perzentil steht je"
    );
    let _ = writeln!(
        text,
        "Runde einmal; Median, Minimum und Maximum laufen ueber alle Einzelwerte."
    );
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "{:<64}{:>13}{:>13}{:>12}{:>12}{:>12}{:>11}{:>20}   Urteil",
        "Gemessene Groesse",
        "p95 bestes",
        "p95 schlecht",
        "Median",
        "Minimum",
        "Maximum",
        "im Bild",
        "Abnahme nach"
    );
    for zusage in &ergebnis.zusagen {
        let _ = writeln!(
            text,
            "{:<64}{:>13}{:>13}{:>12}{:>12}{:>12}{:>11}{:>20}   {}",
            format!("{} — {}", zusage.kennung, zusage.was),
            spanne(zusage.bestes_perzentil()),
            spanne(zusage.schlechtestes_perzentil()),
            spanne(zusage.median()),
            spanne(zusage.minimum()),
            spanne(zusage.maximum()),
            match zusage.schlechtester_anteil() {
                Some(prozent) => format!("{prozent:.1} %"),
                None => "-".to_owned(),
            },
            zusage.mass.beschreibung(),
            messen::urteil(zusage)
        );
    }
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "Urteil: {}",
        if ergebnis.bestanden() {
            "alle zehn Zusagen halten ihr Mass in jeder Runde."
        } else {
            "MINDESTENS EINE ZUSAGE IST VERFEHLT; die Tabelle nennt welche. Eine \
             verfehlte Zusage fuehrt zu einem Entscheidungsdatensatz, nicht zu einer \
             stillschweigenden Lockerung (C8)."
        }
    );
    let _ = writeln!(text);

    let anteilszeilen: Vec<&Zusage> = ergebnis
        .zusagen
        .iter()
        .filter(|zusage| zusage.im_bild().is_some())
        .collect();
    if !anteilszeilen.is_empty() {
        let _ = writeln!(text, "Der Anteil im naechsten Bild, Runde fuer Runde");
        let _ = writeln!(text, "---------------------------------------------");
        let _ = writeln!(
            text,
            "Eine Eingabe erreicht ihr naechstes Bild, wenn ihre Spanne hoechstens eine"
        );
        let _ = writeln!(
            text,
            "Bildlaenge betraegt, hier {}. Gehalten heisst: in jeder Runde mindestens {} %.",
            spanne(ergebnis.bildlaenge),
            ANTEIL_IM_BILD_PROZENT
        );
        for zusage in anteilszeilen {
            let runden = zusage.im_bild().unwrap_or_default();
            let werte: Vec<String> = runden
                .into_iter()
                .map(|(erreicht, gesamt)| {
                    let prozent = if gesamt == 0 {
                        0.0
                    } else {
                        100.0 * erreicht as f64 / gesamt as f64
                    };
                    format!("{prozent:.1} % ({erreicht}/{gesamt})")
                })
                .collect();
            let _ = writeln!(text, "{:<8}{}", zusage.kennung, werte.join("  "));
        }
        let _ = writeln!(text);
    }

    let _ = writeln!(text, "Einzelwerte");
    let _ = writeln!(text, "-----------");
    for zusage in &ergebnis.zusagen {
        let _ = writeln!(text, "{} ({}):", zusage.kennung, zusage.was);
        for (nummer, runde) in zusage.runden.iter().enumerate() {
            let werte: Vec<String> = runde.iter().copied().map(spanne).collect();
            let _ = writeln!(text, "  Runde {}:", nummer + 1);
            for buendel in werte.chunks(5) {
                let _ = writeln!(text, "    {}", buendel.join("  "));
            }
        }
    }
    let _ = writeln!(text);
    text.push_str(GESAMT_LESART);
    text
}

/// Schreibt den Abnahmebericht in den Messungenordner und liefert seinen Pfad.
pub fn gesamt_schreiben(ziel: &Path, text: &str) -> io::Result<PathBuf> {
    if !ziel.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} gibt es nicht. Rufe das Werkzeug aus dem Projektwurzelverzeichnis \
                 auf oder nenne den Ordner ueber --ziel.",
                ziel.display()
            ),
        ));
    }
    let pfad = ziel.join(format!(
        "{}-alle-zusagen.txt",
        kurzstempel(SystemTime::now())
    ));
    fs::write(&pfad, text)?;
    Ok(pfad)
}

/// Die Pruefsitzung aus C8, wie der Kopf sie beschreibt.
const PRUEFSITZUNG: &str = "zwei Dateifenster mit je zwei Tabs: links Pruefordner A \
sichtbar und B dahinter, rechts umgekehrt; Auswahl in beiden sichtbaren Tabs auf dem \
ersten Eintrag, Lesezeichenleiste und Vorschau eingeblendet, Breiten im \
Auslieferungszustand. Hergestellt ueber session.toml, in derselben Serialisierung, \
die die Anwendung beim Beenden schreibt";

/// Was der Bericht ueber seine eigenen Zahlen sagen muss.
const GESAMT_LESART: &str = "\
Lesart und Einschraenkungen
---------------------------
**L1 ist die Spanne vom Zeitstempel des Tastenereignisses bis zum Ende des
Zeichendurchgangs, der die Aenderung traegt — keine Bildschirmmessung.** Eine
Bildgrenze ist der Zeitpunkt, an dem das System sein naechstes Bild
vorbereitet, nicht der, an dem ein Pixel leuchtet; aus dem eigenen Prozess
heraus ist der zweite nicht feststellbar. Dasselbe Ende gilt fuer L5, L6, L7,
L8 und L9: ein Messweg, kein zweiter daneben.

L1, L5, L6, L7, L8 und L9 sind auf der Pruefsitzung im laufenden Buendel
gemessen, mit synthetischen Tastenereignissen ueber die eigene
Ereignisschlange (postEvent:atStart:), die denselben Weg gehen wie ein
koerperlicher Druck. Der Ausloeser haengt an einem Zeitgeber von 97 ms, keinem
Vielfachen der Bildlaenge, damit der Druckzeitpunkt ueber die Wiederholungen
durch das Bild wandert.

**L2, L3 und L10 stammen aus der kopflosen Strecke aus Schritt 3** und sind
mit diesem Bericht zusammengefuehrt. Fuer L2 und die erste Bildschirmseite von
L10 ist das der Anteil des Kerns, ohne das Zeichnen; was das Zeichnen dazulegt,
hat die Fruehmessung aus Schritt 8 am Durchstich gemessen. L3 und das
vollstaendige Lesen von L10 enden im Kern, ihre Zahlen sind vollstaendig.

**L4 ist warm gemessen, C8 sagt Kaltstart.** purge braucht Rechte, die dieser
Lauf nicht hat; die Zahl ist eine Untergrenze der Zusage. Gemessen ist der
Prozessstart bis zur bedienbaren Pruefsitzung: beide sichtbaren Tabs zeigen
ihre erste Bildschirmseite, die Tastatur reagiert. Die Sitzung kommt aus
session.toml, geschrieben vom Sitzungslauf davor.

**L5 ist in beiden Faellen mit bereits gelesenen Zielordnern gemessen** (ein
ungemessener Wechsel davor waermt sie): das ist der Regelfall, den C8 nennt,
weil KRK nach dem Erreichen der bedienbaren Oberflaeche weiterliest. Ende ist
die bedienbare erste Bildschirmseite des Ziels; das vollstaendige Lesen fiele
unter L2, L3 beziehungsweise L10.

**L8 misst den Weg wie L1, L5, L6 und L7**: vom F5-Ereignis bis zum Ende des
Zeichendurchgangs, mit dem die Vorgangsanzeige in der Statuszeile steht. Die
Kopie laeuft auf Pruefordner A mit allen Eintraegen markiert, auf demselben
APFS-Datentraeger; nach der L9-Eingabe wird sie abgebrochen und das
Kopierziel geleert, damit jede Wiederholung dasselbe misst. L9 zaehlt wie L1
den Anteil der Eingaben im naechsten Bild, waehrend die Kopie laeuft.

Das 95. Perzentil ist der Wert des naechsten Rangs, nicht interpoliert: bei
zwanzig Laeufen der neunzehnte der sortierten Reihe. Eine Zusage gilt nur als
gehalten, wenn sie es in jeder Runde tut.
";

// ---------------------------------------------------------------------------
// Angaben des Geraets
// ---------------------------------------------------------------------------

pub fn befehl_ausgabe(programm: &str, argumente: &[&str]) -> String {
    match Command::new(programm).args(argumente).output() {
        Ok(ausgabe) if ausgabe.status.success() => {
            String::from_utf8_lossy(&ausgabe.stdout).trim().to_owned()
        }
        Ok(ausgabe) => format!("nicht ermittelt ({programm} endete mit {})", ausgabe.status),
        Err(fehler) => format!("nicht ermittelt ({programm}: {fehler})"),
    }
}

/// `sw_vers` zu einer Zeile zusammengezogen.
pub fn betriebssystem() -> String {
    let roh = befehl_ausgabe("/usr/bin/sw_vers", &[]);
    let teile: Vec<&str> = roh
        .lines()
        .filter_map(|zeile| zeile.split_once(':').map(|(_, wert)| wert.trim()))
        .collect();
    if teile.is_empty() {
        roh
    } else {
        teile.join(" ")
    }
}

/// Die Bildwiederholrate des Hauptbildschirms.
///
/// Der Plan verlangt sie im Kopf jeder Messdatei. Auf dieser Strecke wird
/// nichts gezeichnet, sie ist hier also eine Angabe zum Geraet und keine
/// Messbedingung; ihre Rolle bekommt sie mit L1 und L5 in Schritt 21. Sie wird
/// mitgeschrieben, damit sich die Berichte beider Strecken vergleichen lassen.
///
/// **Nicht jedes Geraet meldet sie.** Auf dem Referenzgeraet `MacBookPro15,1`
/// fuehrt `system_profiler` zum eingebauten Bildschirm keine Zeile
/// `Refresh Rate`. Dann steht die Aufloesung im Kopf und dazu der Satz, dass
/// die Rate nicht gemeldet wurde — eine erfundene 60 kaeme nicht in Frage,
/// auch wenn C8 sie fuer dieses Geraet nennt.
fn bildwiederholrate() -> String {
    let ausgabe = befehl_ausgabe("/usr/sbin/system_profiler", &["SPDisplaysDataType"]);
    let mut aufloesung = None;
    for zeile in ausgabe.lines() {
        let Some((name, wert)) = zeile.trim().split_once(':') else {
            continue;
        };
        let wert = wert.trim();
        if wert.is_empty() {
            continue;
        }
        if name.contains("Refresh") {
            return wert.to_owned();
        }
        if name.trim() == "Resolution" && aufloesung.is_none() {
            aufloesung = Some(wert.to_owned());
        }
    }
    match aufloesung {
        Some(aufloesung) => format!(
            "von system_profiler nicht gemeldet; Hauptbildschirm {aufloesung} \
             (kopflose Strecke zeichnet nicht)"
        ),
        None => "nicht ermittelt (kopflose Strecke zeichnet nicht)".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Zeit
// ---------------------------------------------------------------------------

/// Ein Zeitpunkt als `JJJJ-MM-TTTHH:MM:SSZ`, in UTC.
pub fn zeitstempel(zeitpunkt: SystemTime) -> String {
    let (jahr, monat, tag, stunde, minute, sekunde) = zerlegen(zeitpunkt);
    format!("{jahr:04}-{monat:02}-{tag:02}T{stunde:02}:{minute:02}:{sekunde:02}Z")
}

/// Ein Zeitpunkt als `JJMMTT-HHMM`, wie ihn die Dateinamen des Projekts fuehren.
pub fn kurzstempel(zeitpunkt: SystemTime) -> String {
    let (jahr, monat, tag, stunde, minute, _) = zerlegen(zeitpunkt);
    format!("{:02}{monat:02}{tag:02}-{stunde:02}{minute:02}", jahr % 100)
}

/// Zerlegt einen Zeitpunkt in seine UTC-Bestandteile.
///
/// Von Hand gerechnet und nicht aus einer Fremdbibliothek geholt: eine
/// Zeitbibliothek waere die einzige Abhaengigkeit dieses Werkzeugs ausser
/// `krk-core`, und sie truege eine Zeitzonendatenbank mit sich, von der hier
/// nichts gebraucht wird. Das Verfahren stammt aus Howard Hinnants
/// `civil_from_days` und rechnet den Kalender ueber Vierhundertjahreszyklen.
fn zerlegen(zeitpunkt: SystemTime) -> (i64, u32, u32, u32, u32, u32) {
    let sekunden = zeitpunkt
        .duration_since(UNIX_EPOCH)
        .map(|spanne| spanne.as_secs() as i64)
        .unwrap_or(0);
    let tage = sekunden.div_euclid(86_400);
    let rest = sekunden.rem_euclid(86_400);

    // Der Kalender wird auf den 1. Maerz 0000 verschoben, damit der Schalttag
    // ans Jahresende faellt und keine Fallunterscheidung braucht.
    let verschoben = tage + 719_468;
    let zyklus = verschoben.div_euclid(146_097);
    let tag_im_zyklus = verschoben.rem_euclid(146_097);
    let jahr_im_zyklus = (tag_im_zyklus - tag_im_zyklus / 1_460 + tag_im_zyklus / 36_524
        - tag_im_zyklus / 146_096)
        / 365;
    let tag_im_jahr =
        tag_im_zyklus - (365 * jahr_im_zyklus + jahr_im_zyklus / 4 - jahr_im_zyklus / 100);
    let monat_verschoben = (5 * tag_im_jahr + 2) / 153;
    let tag = (tag_im_jahr - (153 * monat_verschoben + 2) / 5 + 1) as u32;
    let monat = if monat_verschoben < 10 {
        monat_verschoben + 3
    } else {
        monat_verschoben - 9
    } as u32;
    let jahr = 400 * zyklus + jahr_im_zyklus + i64::from(monat <= 2);

    (
        jahr,
        monat,
        tag,
        (rest / 3_600) as u32,
        ((rest % 3_600) / 60) as u32,
        (rest % 60) as u32,
    )
}

/// Eine Zeitspanne, lesbar und mit gleichbleibender Genauigkeit.
pub fn spanne(dauer: Duration) -> String {
    format!("{:.3} ms", dauer.as_secs_f64() * 1_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messen::{Cache, Messgroesse};

    fn probe_reihe() -> Messreihe {
        let werte: Vec<Duration> = (1..=20).map(Duration::from_millis).collect();
        Messreihe {
            ordner: PathBuf::from("/tmp/pruefordner-a"),
            cache: Cache::Warm,
            wiederholungen: 20,
            eintraege: 10_000,
            groessen: vec![
                Messgroesse {
                    name: "Lesen bis zum ersten Stapel",
                    zusagen: "Anteil an L2",
                    werte: werte.clone(),
                },
                Messgroesse {
                    name: "Vollstaendiges Lesen samt Sortierung",
                    zusagen: "L3, L10",
                    werte,
                },
            ],
        }
    }

    fn probe_kopf() -> Kopf {
        Kopf {
            zeitpunkt: "2026-08-02T18:00:00Z".to_owned(),
            hw_model: "MacBookPro15,1".to_owned(),
            sw_vers: "macOS 15.7.7 24G720".to_owned(),
            bildwiederholrate: "60 Hz".to_owned(),
            startwert: "1".to_owned(),
            eintraege_laut_steckbrief: "10000".to_owned(),
        }
    }

    #[test]
    fn der_kopf_traegt_die_sechs_verlangten_angaben() {
        let text = verfassen(&probe_reihe(), &probe_kopf());
        for angabe in [
            "Zeitpunkt",
            "2026-08-02T18:00:00Z",
            "hw.model",
            "MacBookPro15,1",
            "sw_vers",
            "macOS 15.7.7",
            "Cache-Zustand",
            "warm",
            "Wiederholungen",
            "Startwert",
        ] {
            assert!(text.contains(angabe), "im Kopf fehlt {angabe}:\n{text}");
        }
        assert!(text.contains("\nWiederholungen        20\n"), "\n{text}");
    }

    #[test]
    fn der_zahlenteil_nennt_je_messgroesse_das_perzentil() {
        let text = verfassen(&probe_reihe(), &probe_kopf());
        assert!(text.contains("95. Perzentil"));
        assert!(text.contains("Median"));
        assert!(text.contains("Minimum"));
        for name in [
            "Lesen bis zum ersten Stapel",
            "Vollstaendiges Lesen samt Sortierung",
        ] {
            assert!(text.contains(name), "es fehlt die Messgroesse {name}");
        }
        // ceil(0,95 * 20) = 19: der neunzehnte Wert der Reihe 1..20 ms.
        assert!(
            text.contains("19.000 ms"),
            "das 95. Perzentil steht nicht im Bericht:\n{text}"
        );
        assert!(text.contains("1.000 ms"), "das Minimum fehlt");
    }

    #[test]
    fn der_bericht_sagt_was_er_nicht_misst() {
        let text = verfassen(&probe_reihe(), &probe_kopf());
        assert!(
            text.contains("Anteil des Kerns an L2"),
            "der Bericht gibt eine Teilmessung als L2 aus"
        );
    }

    #[test]
    fn der_zeitstempel_rechnet_bekannte_zeitpunkte_um() {
        let bei = |sekunden: u64| zeitstempel(UNIX_EPOCH + Duration::from_secs(sekunden));
        assert_eq!(bei(0), "1970-01-01T00:00:00Z");
        assert_eq!(bei(1), "1970-01-01T00:00:01Z");
        // Der Schalttag im Jahr 2000, das trotz der Hunderterregel ein
        // Schaltjahr ist.
        assert_eq!(bei(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(bei(951_868_800), "2000-03-01T00:00:00Z");
        // 1900 war keines; der 2000er-Zyklus muss trotzdem stimmen.
        assert_eq!(bei(1_583_020_800), "2020-03-01T00:00:00Z");
        assert_eq!(bei(1_754_150_400), "2025-08-02T16:00:00Z");
        assert_eq!(bei(2_147_483_647), "2038-01-19T03:14:07Z");
        // Der Zeitpunkt, ab dem der Pruefordner-Erzeuger seine Daten vergibt.
        assert_eq!(bei(1_577_836_800), "2020-01-01T00:00:00Z");
    }

    #[test]
    fn der_kurzstempel_passt_zur_namensform_des_projekts() {
        let stempel = kurzstempel(UNIX_EPOCH + Duration::from_secs(1_754_150_400));
        assert_eq!(stempel, "250802-1600");
    }

    #[test]
    fn eine_spanne_wird_in_millisekunden_ausgewiesen() {
        assert_eq!(spanne(Duration::from_millis(12)), "12.000 ms");
        assert_eq!(spanne(Duration::from_micros(1_234)), "1.234 ms");
        assert_eq!(spanne(Duration::ZERO), "0.000 ms");
    }

    #[test]
    fn der_abnahmebericht_traegt_alle_zehn_zusagen_und_den_vollen_kopf() {
        use crate::messen::{Abnahmemass, Gesamtergebnis, Gesamtlauf, Zusage};

        let bildlaenge = Duration::from_secs_f64(1.0 / 60.0);
        let runde: Vec<Duration> = vec![Duration::from_millis(10); 20];
        let zusage = |kennung: &'static str, was: &'static str, mass: Abnahmemass| Zusage {
            kennung,
            was,
            mass,
            runden: vec![runde.clone()],
        };
        let lauf = Gesamtlauf {
            programm: PathBuf::from("/egal/KRK.app/Contents/MacOS/krk"),
            ordner_a: PathBuf::from("/tmp/a"),
            ordner_b: PathBuf::from("/tmp/b"),
            ordner100k: PathBuf::from("/tmp/gross"),
            kopierziel: PathBuf::from("/tmp/ziel"),
            wiederholungen: 20,
            runden: 1,
        };
        let p = |ms: u64| Abnahmemass::Perzentil(Duration::from_millis(ms));
        let ergebnis = Gesamtergebnis {
            bildwiederholrate: 60,
            bildlaenge,
            unterordner: PathBuf::from("/tmp/a-l6"),
            systemlast_vorher: "{ 1.0 1.0 1.0 }".to_owned(),
            systemlast_nachher: "{ 1.2 1.1 1.0 }".to_owned(),
            zusagen: vec![
                zusage(
                    "L1",
                    "Tastendruck",
                    Abnahmemass::AnteilImBild { bildlaenge },
                ),
                zusage("L2", "erste Seite", p(100)),
                zusage("L3", "vollstaendig", p(400)),
                zusage("L4", "Start", p(1000)),
                zusage("L5", "Tabwechsel", p(50)),
                zusage("L5", "Fensterwechsel", p(50)),
                zusage("L6", "Unterordner", p(100)),
                zusage("L7", "Vorschau", p(100)),
                zusage("L8", "Fortschritt", p(200)),
                zusage("L9", "Kopie", Abnahmemass::AnteilImBild { bildlaenge }),
                zusage("L10", "erste Seite", p(100)),
                zusage("L10", "vollstaendig", p(4000)),
            ],
        };
        let text = gesamt_verfassen(&lauf, &ergebnis);

        // Der Kopf: die drei Pruefordner, das Kopierziel, die Pruefsitzung,
        // die Bildwiederholrate als Zahl und die Systemlast als neunte Angabe.
        for angabe in [
            "Pruefordner A",
            "Pruefordner B",
            "Pruefordner 100k",
            "Unterordner L6",
            "Kopierziel L8/L9",
            "Pruefsitzung",
            "60 Hz, gelesen aus NSScreen.maximumFramesPerSecond",
            "Systemlast",
            "vor dem Lauf { 1.0 1.0 1.0 }",
        ] {
            assert!(text.contains(angabe), "im Kopf fehlt {angabe}:\n{text}");
        }
        // Alle zehn Kennungen stehen in der Tabelle.
        for kennung in [
            "L1 —", "L2 —", "L3 —", "L4 —", "L5 —", "L6 —", "L7 —", "L8 —", "L9 —", "L10 —",
        ] {
            assert!(text.contains(kennung), "es fehlt die Zusage {kennung}");
        }
        // L8 wird gegen 200 ms Perzentil abgenommen, L1 als Spanne bis zum
        // Ende des Zeichendurchgangs gekennzeichnet.
        assert!(text.contains("p95 <= 200 ms"));
        assert!(text.contains("Ende des\nZeichendurchgangs") || text.contains("Zeichendurchgang"));
        assert!(ergebnis.bestanden());
    }

    #[test]
    fn ohne_messungenordner_wird_kein_bericht_geschrieben() {
        let fehlt = std::env::temp_dir().join("krk-bench-gibt-es-nicht");
        let _ = fs::remove_dir_all(&fehlt);
        let fehler =
            schreiben(&fehlt, &probe_reihe(), "egal").expect_err("haette scheitern muessen");
        assert_eq!(fehler.kind(), io::ErrorKind::NotFound);
    }
}
