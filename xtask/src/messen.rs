//! `cargo xtask messen` — der eine Einstiegspunkt fuer beide Messstrecken.
//!
//! ```text
//! cargo xtask messen --alle --ordner-a P --ordner-b P --ordner100k P --kopierziel P
//! cargo xtask messen --kopflos --ordner P [--kalt] [--ziel P]
//! ```
//!
//! `--alle` baut zuerst das signierte Buendel ueber denselben Weg wie
//! `cargo xtask bundle` und faehrt dann den Abnahmelauf aus Schritt 21 in
//! `krk-bench`: alle zehn Zusagen L1 bis L10 in einem Bericht. `--kopflos`
//! reicht unveraendert an die kopflose Strecke aus Schritt 3 durch.
//!
//! Gerechnet und berichtet wird ausschliesslich in `crates/krk-bench`; dieses
//! Modul baut das Buendel und ruft das Werkzeug. Eine zweite Auswertung
//! neben der von `krk-bench` entsteht nicht.

use std::process::Command;

use crate::Abbruch;
use crate::bundle;

/// Fuehrt `cargo xtask messen` aus.
pub fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let alle = argumente.iter().any(|marke| marke == "--alle");
    let kopflos = argumente.iter().any(|marke| marke == "--kopflos");
    match (alle, kopflos) {
        (true, false) => alle_fahren(argumente),
        (false, true) => kopflos_fahren(argumente),
        (true, true) => Err(Abbruch::Aufruf(
            "--alle und --kopflos schliessen einander aus".to_owned(),
        )),
        (false, false) => Err(Abbruch::Aufruf(
            "messen braucht eine Strecke: --alle (alle zehn Zusagen am Buendel) oder \
             --kopflos (die Strecke aus Schritt 3)"
                .to_owned(),
        )),
    }
}

/// Der Abnahmelauf aus Schritt 21: Buendel bauen, dann `krk-bench alle`.
fn alle_fahren(argumente: &[String]) -> Result<(), Abbruch> {
    // Der Binaerpfad kommt aus dem Bau und wird hier nicht zusammengesetzt;
    // sein letzter Namensteil steht in `CFBundleExecutable`. Siehe
    // [`bundle::Gebaut`].
    let gebaut = bundle::bauen()?;

    let mut befehl: Vec<String> = vec![
        "alle".to_owned(),
        "--buendel".to_owned(),
        gebaut.binaer.display().to_string(),
    ];
    befehl.extend(argumente.iter().filter(|marke| *marke != "--alle").cloned());
    krk_bench(&befehl)
}

/// Die kopflose Strecke: unveraendert an `krk-bench messen` durchreichen.
fn kopflos_fahren(argumente: &[String]) -> Result<(), Abbruch> {
    let mut befehl: Vec<String> = vec!["messen".to_owned()];
    befehl.extend(argumente.iter().cloned());
    krk_bench(&befehl)
}

/// Ruft `krk-bench` im Profil release, ueber das cargo, das auch diesen
/// Prozess gestartet hat.
///
/// Release und nicht debug, weil eine Zahl aus einem Bau ohne Optimierung
/// gegen keine Zusage abnehmbar ist; der Berichtskopf von `krk-bench` weist
/// die Bauart zusaetzlich selbst aus.
fn krk_bench(argumente: &[String]) -> Result<(), Abbruch> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let status = Command::new(&cargo)
        .args(["run", "--release", "--package", "krk-bench", "--"])
        .args(argumente)
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("{cargo} laesst sich nicht starten: {fehler}")))?;
    if status.success() {
        Ok(())
    } else {
        // Die Meldung hat krk-bench selbst geschrieben; sie hier zu
        // wiederholen waere dieselbe Auskunft ein zweites Mal.
        Err(Abbruch::Lauf(format!(
            "krk-bench endete mit {status}; die Meldung steht darueber."
        )))
    }
}
