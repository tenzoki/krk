#![deny(unsafe_code)]
//! Pruefordner-Erzeuger und kopflose Messstrecke.
//!
//! Zwei Unterbefehle:
//!
//! ```text
//! krk-bench fixture --eintraege N --seed S --out PFAD
//! krk-bench messen  --kopflos --ordner PFAD [--kalt] [--ziel PFAD]
//! ```
//!
//! `fixture` legt einen reproduzierbaren Pruefordner an, `messen` faehrt die
//! Messreihe darauf. Die Abnahme der Zeitzusagen aus C8 braucht **drei**
//! Pruefordner, die dieser Befehl einzeln erzeugt:
//!
//! ```text
//! krk-bench fixture --eintraege 10000  --seed 1 --out <pfad>/a
//! krk-bench fixture --eintraege 10000  --seed 2 --out <pfad>/b
//! krk-bench fixture --eintraege 100000 --seed 3 --out <pfad>/gross
//! ```
//!
//! Die beiden 10.000er-Ordner sind nicht zweimal dasselbe. Die Pruefsitzung
//! fuer L4 hat zwei Dateifenster; zeigten beide auf denselben Ordner, laege er
//! beim zweiten Lesevorgang schon im Cache des Systems, und der Kaltstart waere
//! zur Haelfte warm gemessen. Der verschiedene Startwert ist genau das, was das
//! verhindert.
//!
//! Ein eigener Mehrfachmodus entsteht nicht: ein Aufruf, ein Ordner.

mod bericht;
mod fixture;
mod messen;

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use messen::{Cache, Messreihe, WIEDERHOLUNGEN};

const HILFE: &str = "\
krk-bench — Pruefordner-Erzeuger und kopflose Messstrecke

  krk-bench fixture --eintraege N --seed S --out PFAD
      Legt einen flachen Pruefordner mit N Eintraegen an, reproduzierbar aus
      dem Startwert S. Der Zielordner muss fehlen oder leer sein. Neben dem
      Ordner entsteht ein Steckbrief, der Eintragszahl und Startwert festhaelt.

  krk-bench messen --kopflos --ordner PFAD [--kalt] [--ziel PFAD]
      Misst zwanzigmal das Lesen bis zum ersten Stapel und das vollstaendige
      Lesen samt Sortierung und schreibt einen Bericht nach messungen/.
      --kalt   leert vor jedem Lauf den Dateisystem-Cache ueber purge und
               bricht ab, wenn das nicht gelingt. Braucht sudo.
      --ziel   ein anderer Berichtsordner als messungen/.

  krk-bench --hilfe
";

fn main() -> ExitCode {
    let argumente: Vec<String> = std::env::args().skip(1).collect();
    match ausfuehren(&argumente) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Abbruch::Aufruf(meldung)) => {
            eprintln!("krk-bench: {meldung}\n\n{HILFE}");
            ExitCode::from(2)
        }
        Err(Abbruch::Lauf(meldung)) => {
            eprintln!("krk-bench: {meldung}");
            ExitCode::FAILURE
        }
    }
}

/// Warum ein Lauf geendet hat.
///
/// Die Trennung haelt zwei verschiedene Rueckgabewerte auseinander: ein
/// falscher Aufruf ist etwas anderes als eine gescheiterte Messung, und wer
/// die Strecke aus einem Skript heraus faehrt, will das unterscheiden koennen.
enum Abbruch {
    /// Die Befehlszeile stimmt nicht. Rueckgabewert 2.
    Aufruf(String),
    /// Der Lauf selbst ist gescheitert. Rueckgabewert 1.
    Lauf(String),
}

impl From<io::Error> for Abbruch {
    fn from(fehler: io::Error) -> Self {
        Abbruch::Lauf(fehler.to_string())
    }
}

fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let Some(befehl) = argumente.first() else {
        return Err(Abbruch::Aufruf("kein Unterbefehl genannt".to_owned()));
    };
    match befehl.as_str() {
        "fixture" => fixture_bauen(&argumente[1..]),
        "messen" => messen_fahren(&argumente[1..]),
        "--hilfe" | "--help" | "-h" | "hilfe" => {
            println!("{HILFE}");
            Ok(())
        }
        anderer => Err(Abbruch::Aufruf(format!(
            "unbekannter Unterbefehl {anderer:?}"
        ))),
    }
}

// ---------------------------------------------------------------------------
// fixture
// ---------------------------------------------------------------------------

fn fixture_bauen(argumente: &[String]) -> Result<(), Abbruch> {
    let mut eintraege: Option<usize> = None;
    let mut startwert: Option<u64> = None;
    let mut ziel: Option<PathBuf> = None;

    let mut rest = argumente.iter();
    while let Some(marke) = rest.next() {
        match marke.as_str() {
            "--eintraege" => eintraege = Some(zahl(&mut rest, "--eintraege")?),
            "--seed" => startwert = Some(zahl(&mut rest, "--seed")?),
            "--out" => ziel = Some(PathBuf::from(wert(&mut rest, "--out")?)),
            anderes => {
                return Err(Abbruch::Aufruf(format!("fixture kennt {anderes:?} nicht")));
            }
        }
    }

    let eintraege = eintraege.ok_or_else(|| Abbruch::Aufruf("--eintraege fehlt".to_owned()))?;
    let startwert = startwert.ok_or_else(|| Abbruch::Aufruf("--seed fehlt".to_owned()))?;
    let ziel = ziel.ok_or_else(|| Abbruch::Aufruf("--out fehlt".to_owned()))?;
    if eintraege == 0 {
        return Err(Abbruch::Aufruf(
            "ein Pruefordner ohne Eintraege taugt zu nichts".to_owned(),
        ));
    }

    let erzeugt = fixture::erzeugen(&ziel, eintraege, startwert)?;
    println!(
        "Pruefordner {} angelegt: {} Eintraege ({} Dateien, {} Unterordner, {} Verknuepfungen), \
         Startwert {startwert}.",
        erzeugt.ordner.display(),
        erzeugt.eintraege,
        erzeugt.dateien,
        erzeugt.ordnerzahl,
        erzeugt.verknuepfungen
    );
    println!(
        "Summe der genannten Groessen: {} Bytes. Auf der Platte liegt weniger, weil alles \
         ueber {} Bytes je Datei als Loch entsteht.",
        erzeugt.summe_groessen,
        fixture::ECHTE_BYTES
    );
    println!("Steckbrief: {}", erzeugt.steckbrief.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// messen
// ---------------------------------------------------------------------------

fn messen_fahren(argumente: &[String]) -> Result<(), Abbruch> {
    let mut ordner: Option<PathBuf> = None;
    let mut kopflos = false;
    let mut cache = Cache::Warm;
    let mut ziel = PathBuf::from(bericht::MESSUNGEN);

    let mut rest = argumente.iter();
    while let Some(marke) = rest.next() {
        match marke.as_str() {
            "--kopflos" => kopflos = true,
            "--kalt" => cache = Cache::Kalt,
            "--ordner" => ordner = Some(PathBuf::from(wert(&mut rest, "--ordner")?)),
            "--ziel" => ziel = PathBuf::from(wert(&mut rest, "--ziel")?),
            anderes => {
                return Err(Abbruch::Aufruf(format!("messen kennt {anderes:?} nicht")));
            }
        }
    }

    if !kopflos {
        // Kein stillschweigendes Ausweichen auf die einzige Strecke, die es
        // gibt: die Messung an der laufenden Anwendung kommt mit Schritt 21,
        // und bis dahin soll ein Aufruf ohne --kopflos nicht so aussehen, als
        // haette er sie gefahren.
        return Err(Abbruch::Aufruf(
            "--kopflos fehlt. Eine andere Strecke gibt es noch nicht; die Messung an der \
             laufenden Anwendung entsteht mit Schritt 21 des Plans."
                .to_owned(),
        ));
    }
    let ordner = ordner.ok_or_else(|| Abbruch::Aufruf("--ordner fehlt".to_owned()))?;
    if !ordner.is_dir() {
        return Err(Abbruch::Lauf(format!(
            "{} ist kein Verzeichnis",
            ordner.display()
        )));
    }

    let reihe = Messreihe::fahren(&ordner, cache, WIEDERHOLUNGEN)?;
    let kopf = bericht::Kopf::erheben(&ordner);
    let text = bericht::verfassen(&reihe, &kopf);
    let geschrieben = schreiben_und_melden(&ziel, &reihe, &text)?;
    print!("{text}");
    println!("Bericht: {}", geschrieben.display());
    Ok(())
}

fn schreiben_und_melden(ziel: &Path, reihe: &Messreihe, text: &str) -> Result<PathBuf, Abbruch> {
    bericht::schreiben(ziel, reihe, text).map_err(Abbruch::from)
}

// ---------------------------------------------------------------------------
// Befehlszeile
// ---------------------------------------------------------------------------

fn wert<'a>(
    rest: &mut impl Iterator<Item = &'a String>,
    marke: &str,
) -> Result<&'a String, Abbruch> {
    rest.next()
        .ok_or_else(|| Abbruch::Aufruf(format!("{marke} braucht einen Wert")))
}

fn zahl<'a, T: std::str::FromStr>(
    rest: &mut impl Iterator<Item = &'a String>,
    marke: &str,
) -> Result<T, Abbruch> {
    let roh = wert(rest, marke)?;
    roh.parse()
        .map_err(|_| Abbruch::Aufruf(format!("{marke} braucht eine Zahl, nicht {roh:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worte(zeile: &[&str]) -> Vec<String> {
        zeile.iter().map(|wort| (*wort).to_owned()).collect()
    }

    fn ist_aufruffehler(ergebnis: Result<(), Abbruch>) -> bool {
        matches!(ergebnis, Err(Abbruch::Aufruf(_)))
    }

    #[test]
    fn ohne_unterbefehl_ist_der_aufruf_falsch() {
        assert!(ist_aufruffehler(ausfuehren(&[])));
    }

    #[test]
    fn ein_unbekannter_unterbefehl_ist_ein_aufruffehler() {
        assert!(ist_aufruffehler(ausfuehren(&worte(&["messsen"]))));
    }

    #[test]
    fn fixture_verlangt_alle_drei_angaben() {
        assert!(ist_aufruffehler(fixture_bauen(&worte(&[
            "--eintraege",
            "10",
            "--seed",
            "1"
        ]))));
        assert!(ist_aufruffehler(fixture_bauen(&worte(&[
            "--eintraege",
            "10",
            "--out",
            "/tmp/krk-egal"
        ]))));
        assert!(ist_aufruffehler(fixture_bauen(&worte(&[
            "--seed",
            "1",
            "--out",
            "/tmp/krk-egal"
        ]))));
    }

    #[test]
    fn fixture_nimmt_keine_null_eintraege() {
        assert!(ist_aufruffehler(fixture_bauen(&worte(&[
            "--eintraege",
            "0",
            "--seed",
            "1",
            "--out",
            "/tmp/krk-egal"
        ]))));
    }

    #[test]
    fn fixture_verlangt_zahlen_wo_zahlen_hingehoeren() {
        assert!(ist_aufruffehler(fixture_bauen(&worte(&[
            "--eintraege",
            "viele",
            "--seed",
            "1",
            "--out",
            "/tmp/krk-egal"
        ]))));
    }

    #[test]
    fn messen_ohne_kopflos_faehrt_nicht_los() {
        assert!(ist_aufruffehler(messen_fahren(&worte(&[
            "--ordner", "/tmp"
        ]))));
    }

    #[test]
    fn messen_verlangt_einen_ordner() {
        assert!(ist_aufruffehler(messen_fahren(&worte(&["--kopflos"]))));
    }

    #[test]
    fn messen_auf_einem_nichtverzeichnis_ist_ein_laufzeitfehler() {
        let ergebnis = messen_fahren(&worte(&["--kopflos", "--ordner", "/etc/hosts"]));
        assert!(matches!(ergebnis, Err(Abbruch::Lauf(_))));
    }
}
