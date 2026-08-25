//! Abnahme der Umrechnung eines Zeitpunkts in buergerliche Ortszeit.
//!
//! Geprueft wird [`krk_core::verzeichnis::sys::ortszeit`], die sechste
//! Schnittstelle der Systemschicht, und die eine Zusage, die sie traegt: **der
//! Zonenversatz ist der, der zum uebergebenen Zeitpunkt galt, und nicht der von
//! jetzt**. Eine Datei vom Juli und eine vom Januar bekommen in Mitteleuropa
//! verschiedene Versaetze; wer einen einzigen Wert je Lauf holt, legt eine von
//! beiden um eine Stunde daneben, und genau das tut `ditto(1)` gemessenermassen.
//!
//! # Warum Kindprozesse
//!
//! Eine Zusicherung auf einen festen Kalenderwert haengt an der Zone des
//! Pruefgeraets. Der Kern liest die Zone beim **ersten** `localtime_r` im
//! Prozess aus der Umgebungsvariablen `TZ`; ein `set_var` mitten im Lauf kaeme
//! also zu spaet und waere daneben in einem Programm mit mehreren Faden
//! unzulaessig. Die Form, die dieser Baum dafuer kennt, ist die Kindprobe mit
//! gesetzter Umgebung: `tests/ablage.rs` startet dieselbe Pruefdatei mit einer
//! gesetzten Variablen noch einmal, `tests/verzeichnis.rs` tut dasselbe unter
//! `ulimit -n 64`. Diese Datei schreibt die Form ab und macht keine neue auf.
//!
//! Drei Proben, und jede beantwortet eine andere Frage. Unter `TZ=UTC` stehen
//! feste Zeitpunkte gegen feste Kalenderwerte. Unter `TZ=Europe/Berlin` stehen
//! **zwei** Zeitpunkte aus verschiedenen Halbjahren nebeneinander, und das ist
//! die einzige Probe, die den Sommerzeitfall ueberhaupt pruefen kann: ohne sie
//! waere die Zusage behauptet. Die dritte kennt keine Zone und belegt nur, dass
//! ueberhaupt ein Wert ankommt; sie laeuft in jeder Zone gruen.

use std::process::{Command, Output};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use krk_core::verzeichnis::sys::{Ortszeit, ortszeit};

// ---------------------------------------------------------------------------
// Die Zeitpunkte, gegen die gemessen wird
// ---------------------------------------------------------------------------

/// Der Nullpunkt: 1970-01-01 00:00:00 UTC.
const NULLPUNKT: i64 = 0;

/// 2026-01-01 12:00:00 UTC. In Mitteleuropa gilt hier die Normalzeit, +1.
const WINTERMITTAG: i64 = 1_767_268_800;

/// 2026-07-15 12:00:00 UTC. In Mitteleuropa gilt hier die Sommerzeit, +2.
///
/// **Beide Zeitpunkte liegen auf 12:00 UTC.** Dass die buergerliche Uhrzeit
/// trotzdem auseinandergeht, ist der ganze Gegenstand dieser Datei.
const SOMMERMITTAG: i64 = 1_784_116_800;

/// 2026-03-29 01:00:00 UTC, der Augenblick der Umstellung in Mitteleuropa.
///
/// Die buergerliche Uhr springt hier von 02:00 auf 03:00; die Stunde dazwischen
/// gibt es an diesem Tag nicht.
const UMSTELLUNG: i64 = 1_774_746_000;

/// 1980-01-01 00:00:00 UTC, der Nullpunkt des MS-DOS-Zeitfeldes im Zip-Format.
const ZIPNULLPUNKT: i64 = 315_532_800;

// ---------------------------------------------------------------------------
// Die Elternproben
// ---------------------------------------------------------------------------

/// Die Umgebungsvariable, die eine Kindprobe beauftragt. Ihr Wert ist die Zone,
/// die der Elternteil daneben in `TZ` gesetzt hat.
const AUFTRAG_ZONE: &str = "KRK_PROBE_ZEITZONE";

/// Startet dieselbe Testdatei noch einmal, mit gesetzter Zone, und laesst genau
/// eine Kindprobe laufen.
fn kindprobe_in_zone(name: &str, zone: &str) -> Output {
    let selbst = std::env::current_exe().expect("die Testdatei kennt ihren Pfad nicht");
    Command::new(selbst)
        .args(["--exact", "--ignored", "--nocapture", "--test-threads", "1"])
        .arg(name)
        .env("TZ", zone)
        .env(AUFTRAG_ZONE, zone)
        .output()
        .expect("die Kindprobe laesst sich nicht starten")
}

/// Haelt fest, dass das Kind durchgelaufen ist, und gibt sonst aus, was es sah.
#[track_caller]
fn kind_ist_durchgelaufen(ergebnis: &Output, zone: &str) {
    assert!(
        ergebnis.status.success(),
        "die Kindprobe unter TZ={zone} ist gescheitert\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

/// Feste Zeitpunkte gegen feste Kalenderwerte, in einer Zone ohne Versatz.
///
/// Ohne diese Probe traege die naechste allein die Aussage „die zwei
/// unterscheiden sich"; hier steht, dass die Zerlegung als solche stimmt.
#[test]
fn unter_tz_utc_stehen_feste_zeitpunkte_auf_festen_kalenderwerten() {
    let ergebnis = kindprobe_in_zone("kind_rechnet_in_utc", "UTC");
    kind_ist_durchgelaufen(&ergebnis, "UTC");
}

/// Der Versatz haengt am Zeitpunkt und nicht am Lauf.
///
/// **Die eine Probe, die den Sommerzeitfall misst statt ihn anzunehmen.** Sie
/// rechnet in **einem** Lauf zwei Zeitpunkte aus verschiedenen Halbjahren um;
/// ein Versatz je Lauf koennte sie nicht bestehen.
#[test]
fn unter_europe_berlin_haengt_der_versatz_am_zeitpunkt_und_nicht_am_lauf() {
    let ergebnis = kindprobe_in_zone("kind_rechnet_in_berlin", "Europe/Berlin");
    kind_ist_durchgelaufen(&ergebnis, "Europe/Berlin");
}

/// Ein Wert kommt an, gleich in welcher Zone das Pruefgeraet steht.
///
/// Der Rundweg ohne feste Kalenderwerte: er laeuft im Elternprozess und damit
/// in der Zone des Geraets, und er belegt, dass [`ortszeit`] ueberhaupt
/// antwortet und was sie antwortet innerhalb des Kalenders liegt.
#[test]
fn eine_ortszeit_kommt_in_jeder_zone_an() {
    for sekunden in [NULLPUNKT, ZIPNULLPUNKT, WINTERMITTAG, SOMMERMITTAG] {
        let zerlegt = ortszeit(zeitpunkt(sekunden))
            .unwrap_or_else(|| panic!("{sekunden} laesst sich nicht in Ortszeit umrechnen"));

        assert!(
            (1969..=2027).contains(&zerlegt.jahr),
            "{sekunden} liegt im Jahr {}, und das kann keine Zone erklaeren",
            zerlegt.jahr
        );
        assert!((1..=12).contains(&zerlegt.monat), "Monat {}", zerlegt.monat);
        assert!((1..=31).contains(&zerlegt.tag), "Tag {}", zerlegt.tag);
        assert!(zerlegt.stunde <= 23, "Stunde {}", zerlegt.stunde);
        assert!(zerlegt.minute <= 59, "Minute {}", zerlegt.minute);
        assert!(zerlegt.sekunde <= 60, "Sekunde {}", zerlegt.sekunde);
    }

    // Der Abstand zweier Zeitpunkte kommt auf der buergerlichen Uhr an. Keine
    // Zone der Welt verschiebt eine Minute innerhalb einer Stunde.
    let frueh = ortszeit(zeitpunkt(WINTERMITTAG)).expect("kein Wert");
    let spaet = ortszeit(zeitpunkt(WINTERMITTAG + 60)).expect("kein Wert");
    assert_eq!(frueh.stunde, spaet.stunde);
    assert_eq!(spaet.minute, frueh.minute + 1);
}

// ---------------------------------------------------------------------------
// Die Kindproben
// ---------------------------------------------------------------------------

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_PROBE_ZEITZONE mit gesetztem TZ gestartet"]
fn kind_rechnet_in_utc() {
    let Some(zone) = beauftragte_zone() else {
        return;
    };
    assert_eq!(zone, "UTC");

    gleicht(NULLPUNKT, wie(1970, 1, 1, 0, 0, 0));
    gleicht(WINTERMITTAG, wie(2026, 1, 1, 12, 0, 0));
    gleicht(SOMMERMITTAG, wie(2026, 7, 15, 12, 0, 0));
    gleicht(UMSTELLUNG, wie(2026, 3, 29, 1, 0, 0));
    gleicht(ZIPNULLPUNKT, wie(1980, 1, 1, 0, 0, 0));

    // Eine Sekunde vor dem Nullpunkt liegt im Jahr davor, nicht im Jahr danach.
    gleicht(-1, wie(1969, 12, 31, 23, 59, 59));

    // Und die Gegenprobe zur Zone, die die naechste Kindprobe faehrt: ohne
    // Versatz stehen die zwei Mittage auf derselben Stunde.
    let winter = zerlegt(WINTERMITTAG);
    let sommer = zerlegt(SOMMERMITTAG);
    assert_eq!(
        winter.stunde, sommer.stunde,
        "in UTC gibt es keine Sommerzeit, und die zwei Mittage duerfen nicht auseinandergehen"
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_PROBE_ZEITZONE mit gesetztem TZ gestartet"]
fn kind_rechnet_in_berlin() {
    let Some(zone) = beauftragte_zone() else {
        return;
    };
    assert_eq!(zone, "Europe/Berlin");

    // Zwei Zeitpunkte, beide 12:00 UTC, ein Lauf, zwei Versaetze. Scheitert
    // eine dieser zwei Zusicherungen, rechnet die Umrechnung mit einem Versatz
    // je Lauf statt mit einem je Zeitpunkt -- das ist der Fehler, den `ditto`
    // macht. Scheitern **beide**, fehlt eher die Zonendatenbank unter
    // `/usr/share/zoneinfo`, und der Kern ist auf UTC zurueckgefallen.
    gleicht(WINTERMITTAG, wie(2026, 1, 1, 13, 0, 0));
    gleicht(SOMMERMITTAG, wie(2026, 7, 15, 14, 0, 0));

    let winter = zerlegt(WINTERMITTAG);
    let sommer = zerlegt(SOMMERMITTAG);
    assert_eq!(
        sommer.stunde,
        winter.stunde + 1,
        "beide Zeitpunkte liegen auf 12:00 UTC; die Sommerzeit muss genau eine Stunde ausmachen"
    );

    // Der Nullpunkt liegt in Mitteleuropa schon im Jahr 1970, aber um eine
    // Stunde versetzt. Die Sekunde davor faellt nicht ins Vorjahr.
    gleicht(NULLPUNKT, wie(1970, 1, 1, 1, 0, 0));
    gleicht(-1, wie(1970, 1, 1, 0, 59, 59));

    // Der Augenblick der Umstellung: die buergerliche Uhr springt von 02:00 auf
    // 03:00, die Stunde dazwischen gibt es an diesem Tag nicht.
    gleicht(UMSTELLUNG, wie(2026, 3, 29, 3, 0, 0));
    gleicht(UMSTELLUNG - 1, wie(2026, 3, 29, 1, 59, 59));

    gleicht(ZIPNULLPUNKT, wie(1980, 1, 1, 1, 0, 0));
}

// ---------------------------------------------------------------------------
// Die Helfer
// ---------------------------------------------------------------------------

/// Die Zone, die der Elternteil beauftragt hat, oder `None` fuer den Lauf ohne
/// Auftrag.
///
/// Die zweite Zusicherung ist der Grund, aus dem der Auftrag ueberhaupt eine
/// eigene Variable bekommt und nicht `TZ` allein genuegt: sie faengt den Lauf
/// ab, in dem der Auftrag steht und die Zone selbst nicht angekommen ist.
fn beauftragte_zone() -> Option<String> {
    let zone = std::env::var("KRK_PROBE_ZEITZONE").ok()?;
    assert_eq!(
        std::env::var("TZ").ok().as_deref(),
        Some(zone.as_str()),
        "die Kindprobe ist beauftragt, aber TZ traegt die Zone nicht"
    );
    Some(zone)
}

/// Macht aus einer Zahl von Sekunden seit 1970 einen [`SystemTime`], in beide
/// Richtungen.
fn zeitpunkt(sekunden: i64) -> SystemTime {
    let spanne = Duration::from_secs(sekunden.unsigned_abs());
    if sekunden >= 0 {
        UNIX_EPOCH + spanne
    } else {
        UNIX_EPOCH - spanne
    }
}

/// Die Zerlegung eines Zeitpunkts, mit einer Meldung statt eines nackten
/// `unwrap`.
#[track_caller]
fn zerlegt(sekunden: i64) -> Ortszeit {
    ortszeit(zeitpunkt(sekunden))
        .unwrap_or_else(|| panic!("{sekunden} laesst sich nicht in Ortszeit umrechnen"))
}

/// Ein erwarteter Kalenderwert, in der Reihenfolge, in der man ihn spricht.
fn wie(jahr: i32, monat: u8, tag: u8, stunde: u8, minute: u8, sekunde: u8) -> Ortszeit {
    Ortszeit {
        jahr,
        monat,
        tag,
        stunde,
        minute,
        sekunde,
    }
}

/// Haelt einen Zeitpunkt gegen seinen Kalenderwert.
#[track_caller]
fn gleicht(sekunden: i64, erwartet: Ortszeit) {
    assert_eq!(
        zerlegt(sekunden),
        erwartet,
        "die Sekunde {sekunden} seit 1970 steht auf dem falschen Kalenderwert"
    );
}
