//! Der Laeufer des Pruefziels `hauptfaden`: er faehrt die Proben, die eine
//! AppKit-Ansicht bauen oder einen `MainThreadMarker` brauchen, auf dem
//! Hauptfaden.
//!
//! **Warum ein eigener Laeufer.** `libtest` legt jede Probe auf einen eigenen
//! Faden, auch mit `--test-threads=1`, und `MainThreadMarker::new()` antwortet
//! dort `None`. AppKit bindet `NSView` an den Hauptfaden, und seit dem SDK 26
//! setzt es die Bindung durch: Proben, die den Faden nur behaupteten, brechen
//! mit `NSInternalInconsistencyException` ab. Das Pruefziel `hauptfaden`
//! (`crates/krk-ui/Cargo.toml`, `harness = false`) haelt sein `main` selbst,
//! also laeuft dieser Laeufer auf dem Hauptfaden, und er **fragt** ihn, statt
//! ihn vorauszusetzen. Entschieden im Datensatz
//! `261101-1436-wie-kommen-die-appkit-proben-auf-den-hauptfaden.md`
//! (Moeglichkeit 1).
//!
//! **Wie eine Probe hierher kommt.** Sie bleibt im Pruefmodul ihrer Datei und
//! erreicht dort alles Private, ist aber kein `#[test]`, sondern eine Funktion
//! `fn(MainThreadMarker)`. Das Pruefmodul fuehrt sie in seiner Liste
//! `HAUPTFADENPROBEN`, gebaut mit [`hauptfadenprobe`], und reicht die Liste mit
//! `pub(super) use tests::HAUPTFADENPROBEN;` vor `#[cfg(test)]` und
//! `mod tests {` herauf; [`alle`] haengt die Listen aneinander. Im Bau von
//! `libtest` ist eine Probenfunktion, die in keiner Liste steht, toter Code,
//! und `make lint` faehrt mit `-D warnings`: das Fehlen einer Eintragung ist
//! laut.
//!
//! **Der Ablauf hat drei Teile, und jeder der ersten zwei haelt vor der ersten
//! Probe an.** Erstens der Hauptfaden ueber `MainThreadMarker::new()`. Zweitens
//! die [`WERKSVORGABEN`], festgelegt im fluechtigen Argumentbereich und
//! zurueckgelesen. Drittens die gewaehlten Proben, nacheinander, jede in
//! `catch_unwind` und unter einer [`FRIST_JE_PROBE`], die ein Wachfaden haelt.
//! Ausgegeben wird in der Form von `libtest`, und die Befehlszeile versteht
//! dieselben Woerter, die `cargo test` an ein Ziel reicht (`auswerten`).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSUserDefaults` (`NSUserDefaults.h:144`), `NSDictionary`
//! (`NSDictionary.h:97`), `NSMutableDictionary` (`NSDictionary.h:606`), `NSNumber` (`NSValue.h:181`) und `NSString` tragen
//! im SDK keine eigene Verfuegbarkeitsangabe und stehen seit macOS 10.0; das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`). Ebenso ohne Angabe sind die
//! angesprochenen Methoden `standardUserDefaults`, `objectForKey:`
//! (`NSUserDefaults.h:179`), `boolForKey:` (`:252`),
//! `volatileDomainForName:` (`:329`), `setVolatileDomain:forName:` (`:335`)
//! und `removeVolatileDomainForName:` (`:339`), und die Konstante
//! `NSArgumentDomain` (`:15`). `NSMutableCopying` ist das Protokoll hinter
//! `mutableCopy` (`NSObject.h:43`) und traegt keine Angabe; `description`, das
//! ueber `msg_send!` am Vorgabewert gefragt wird, steht am Protokoll `NSObject`
//! ohne Angabe. `MainThreadMarker` ist ein Rust-Typ der Kiste und hat kein
//! macOS-Alter. Keine Beruehrung dieser Datei liegt ueber macOS 15.

use std::collections::BTreeSet;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_foundation::{
    MainThreadMarker, NSArgumentDomain, NSDictionary, NSMutableCopying, NSMutableDictionary,
    NSNumber, NSString, NSUserDefaults,
};

/// Eine Probe, die auf dem Hauptfaden laeuft.
///
/// `pfad` traegt den vollen Modulpfad samt Kistennamen, so wie [`hauptfadenprobe`]
/// ihn aus `module_path!()` bildet; [`Probe::name`] nimmt den Kistennamen weg
/// und liefert den Pfad, unter dem die Probe in der Reihe von `libtest` stand.
/// Wer nach dem alten Namen filtert, findet sie also wieder.
pub(crate) struct Probe {
    pub(crate) pfad: &'static str,
    /// Der Grund aus dem frueheren `#[ignore]`, oder `None`.
    pub(crate) ignoriert: Option<&'static str>,
    pub(crate) lauf: fn(MainThreadMarker),
}

impl Probe {
    /// Der Name ohne Kistennamen, etwa `appkit::editor::tests::…`.
    pub(crate) fn name(&self) -> &'static str {
        self.pfad
            .split_once("::")
            .map_or(self.pfad, |(_kiste, rest)| rest)
    }
}

/// Baut eine [`Probe`] aus dem Namen ihrer Funktion.
///
/// Name und Funktion kommen aus demselben Bezeichner, also kann der Name, unter
/// dem die Probe laeuft, nicht von der Funktion abweichen, die sie faehrt.
/// `module_path!()` steht in der Ausdehnung und nennt das Modul des Aufrufers.
macro_rules! hauptfadenprobe {
    ($lauf:ident) => {
        $crate::appkit::hauptfaden::Probe {
            pfad: concat!(module_path!(), "::", stringify!($lauf)),
            ignoriert: None,
            lauf: $lauf,
        }
    };
    ($lauf:ident, ignoriert = $grund:literal) => {
        $crate::appkit::hauptfaden::Probe {
            pfad: concat!(module_path!(), "::", stringify!($lauf)),
            ignoriert: Some($grund),
            lauf: $lauf,
        }
    };
}
pub(crate) use hauptfadenprobe;

/// Jede Probe des Pruefziels, in der Reihenfolge der Listen.
fn alle() -> Vec<&'static Probe> {
    [
        super::editor::HAUPTFADENPROBEN,
        super::tabelle::HAUPTFADENPROBEN,
        super::blaetter::HAUPTFADENPROBEN,
        super::blaetter::namenseingabe::HAUPTFADENPROBEN,
        super::nebenfaden::HAUPTFADENPROBEN,
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Die Frist, in der eine Probe enden muss.
///
/// Ueberschreitet eine sie, schreibt der Wachfaden ihren Namen auf die
/// Standardfehlerausgabe und beendet den Prozess mit 101: ein Haengen auf dem
/// Hauptfaden wird ein benannter Fehlschlag, und `make check` steht nicht
/// still. Die Proben brauchen Bruchteile einer Sekunde; die Frist ist so weit
/// gewaehlt, dass ein beschaeftigtes Geraet sie nicht reisst. Fuer Proben, die
/// nur ueber `--ignored` laufen (Messungen), gilt sie nicht.
const FRIST_JE_PROBE: Duration = Duration::from_secs(60);

/// Der Rueckgabewert eines roten Laufs, derselbe wie bei `libtest`.
const ROT: i32 = 101;

/// Die Werksvorgaben, die der Laeufer vor der ersten Probe festlegt.
///
/// Zwei Proben in `super::editor` vergleichen KRKs Flaechen mit einer frischen
/// `NSTextView` und setzen voraus, dass die frische die Werkseinstellung
/// traegt. Die frische liest die Automatiken aber aus den Vorgaben des
/// Nutzers, und wer sie global abgeschaltet hat, haette einen Zeugen, der
/// schon aus steht und ueber KRKs Abschaltung nichts belegt. Der Laeufer legt
/// die Werte deshalb im **fluechtigen Argumentbereich** (`NSArgumentDomain`)
/// fest, der in der Suchreihe vor dem globalen Bereich steht. Er gilt allein fuer
/// den laufenden Prozess, und nichts davon geht auf die Platte: die
/// Einstellungen des Nutzers bleiben unberuehrt, und die Proben messen auf
/// jedem Geraet gegen denselben Zeugen.
const WERKSVORGABEN: &[(&str, &str)] = &[
    (
        "NSAutomaticQuoteSubstitutionEnabled",
        "ab Werk ersetzt eine frische Flaeche gerade Anfuehrungszeichen durch typografische",
    ),
    (
        "NSAutomaticDashSubstitutionEnabled",
        "ab Werk ersetzt eine frische Flaeche zwei Bindestriche durch einen Gedankenstrich",
    ),
    (
        "NSAutomaticSpellingCorrectionEnabled",
        "ab Werk korrigiert eine frische Flaeche die Rechtschreibung beim Tippen",
    ),
];

/// Die Marken von `libtest`, die ihren Wert im folgenden Wort tragen.
///
/// Der Laeufer beachtet keine davon, muss aber ihr Folgewort mitnehmen, sonst
/// waehlte `--test-threads 1` alle Proben mit einer `1` im Namen.
const MARKEN_MIT_WERT: &[&str] = &[
    "--test-threads",
    "--format",
    "--color",
    "--logfile",
    "--shuffle-seed",
    "-Z",
];

/// Was mit den Proben geschieht, die ein `#[ignore]` getragen haetten.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ignorierte {
    /// Ohne Marke: sie werden als `ignored` gemeldet und nicht gefahren.
    Auslassen,
    /// `--ignored`: allein sie werden gefahren.
    Allein,
    /// `--include-ignored`: sie werden mit den uebrigen gefahren.
    Dazu,
}

/// Die ausgewertete Befehlszeile.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Auswahl {
    filter: Vec<String>,
    genau: bool,
    auslassen: Vec<String>,
    ignorierte: Ignorierte,
    nur_auflisten: bool,
}

impl Auswahl {
    /// Ob die Probe dieses Namens unter die Auswahl faellt, ohne Ruecksicht
    /// auf `#[ignore]`.
    ///
    /// Wie bei `libtest`: freie Woerter waehlen als Teiltext oder mit
    /// `--exact` als ganzer Name, und irgendeines genuegt; `--skip` laesst aus,
    /// mit `--exact` ebenso als ganzer Name.
    fn trifft(&self, name: &str) -> bool {
        let passt = |muster: &String| {
            if self.genau {
                name == muster
            } else {
                name.contains(muster.as_str())
            }
        };
        (self.filter.is_empty() || self.filter.iter().any(passt))
            && !self.auslassen.iter().any(passt)
    }

    /// Ob die Probe gewaehlt ist, mit Ruecksicht auf `--ignored`.
    fn waehlt(&self, probe: &Probe) -> bool {
        self.trifft(probe.name())
            && (self.ignorierte != Ignorierte::Allein || probe.ignoriert.is_some())
    }
}

/// Wertet die Befehlszeile aus, wie `cargo test` sie an ein Ziel ohne Harness
/// reicht.
///
/// Jede Marke, die hier nicht steht, wird uebergangen und nicht bemaengelt;
/// die mit Wert im Folgewort stehen in [`MARKEN_MIT_WERT`].
fn auswerten(argumente: impl IntoIterator<Item = String>) -> Auswahl {
    let mut auswahl = Auswahl {
        filter: Vec::new(),
        genau: false,
        auslassen: Vec::new(),
        ignorierte: Ignorierte::Auslassen,
        nur_auflisten: false,
    };
    let mut woerter = argumente.into_iter();
    while let Some(wort) = woerter.next() {
        match wort.as_str() {
            "--exact" => auswahl.genau = true,
            "--ignored" => auswahl.ignorierte = Ignorierte::Allein,
            "--include-ignored" => auswahl.ignorierte = Ignorierte::Dazu,
            "--list" => auswahl.nur_auflisten = true,
            "--skip" => auswahl.auslassen.extend(woerter.next()),
            marke if marke.starts_with("--skip=") => {
                auswahl.auslassen.push(marke["--skip=".len()..].to_owned());
            }
            marke if MARKEN_MIT_WERT.contains(&marke) => {
                let _ = woerter.next();
            }
            marke if marke.starts_with('-') => {}
            _ => auswahl.filter.push(wort),
        }
    }
    auswahl
}

/// Der Ausgang eines Laufs, vergleichbar, damit eine Probe ihn pruefen kann.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ausgang {
    /// Alle gefahrenen Proben sind gruen.
    Gruen { gefahren: usize },
    /// Mindestens eine Probe ist rot.
    Rot { gefahren: usize, gescheitert: usize },
    /// `--list`: aufgelistet, nichts gefahren.
    Aufgelistet { anzahl: usize },
    /// `MainThreadMarker::new()` hat `None` geliefert; keine Probe ist gefahren.
    NichtAufDemHauptfaden,
    /// Der Wachfaden laesst sich nicht starten; keine Probe ist gefahren, weil
    /// keine ihre Frist haette.
    OhneWache,
    /// Eine Werksvorgabe liest nach dem Festlegen nicht zurueck; keine Probe ist
    /// gefahren.
    VorgabeHaeltNicht {
        schluessel: &'static str,
        gelesen: String,
    },
}

impl Ausgang {
    /// Ob `cargo test` den Lauf als gruen lesen soll.
    pub(crate) fn ist_gruen(&self) -> bool {
        matches!(self, Ausgang::Gruen { .. } | Ausgang::Aufgelistet { .. })
    }
}

/// Faehrt die gewaehlten Proben auf dem Hauptfaden.
pub(crate) fn laufen(argumente: impl IntoIterator<Item = String>) -> Ausgang {
    let auswahl = auswerten(argumente);
    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!(
            "hauptfaden: dieser Laeufer steht nicht auf dem Hauptfaden; keine Probe ist \
             gefahren. Er gehoert in das Pruefziel ohne Harness und nicht unter libtest."
        );
        return Ausgang::NichtAufDemHauptfaden;
    };
    let proben = alle();
    let gewaehlt: Vec<&Probe> = proben
        .iter()
        .copied()
        .filter(|probe| auswahl.waehlt(probe))
        .collect();

    if auswahl.nur_auflisten {
        for probe in &gewaehlt {
            println!("{}: test", probe.name());
        }
        println!("\n{} tests, 0 benchmarks", gewaehlt.len());
        return Ausgang::Aufgelistet {
            anzahl: gewaehlt.len(),
        };
    }

    if let Err(ausgang) = werksvorgaben_festlegen() {
        return ausgang;
    }

    let ausgefiltert = proben.len() - gewaehlt.len();
    let wache = match wachfaden_starten(FRIST_JE_PROBE) {
        Ok(wache) => wache,
        Err(fehler) => {
            eprintln!(
                "hauptfaden: der Wachfaden laesst sich nicht starten ({fehler}); ohne ihn \
                 haelt keine Probe ihre Frist, also ist keine gefahren"
            );
            return Ausgang::OhneWache;
        }
    };
    let beginn = Instant::now();
    let mut gruen = 0;
    let mut ignoriert = 0;
    let mut gescheitert: Vec<&str> = Vec::new();
    println!("\nrunning {} tests", gewaehlt.len());
    for probe in gewaehlt {
        let name = probe.name();
        if let Some(grund) = probe.ignoriert
            && auswahl.ignorierte == Ignorierte::Auslassen
        {
            println!("test {name} ... ignored, {grund}");
            ignoriert += 1;
            continue;
        }
        probe_ansagen(&mut std::io::stdout(), name)
            .expect("die Standardausgabe nimmt die Ansage einer Probe");
        let mit_frist = probe.ignoriert.is_none();
        if mit_frist {
            wache
                .send(Wachruf::Beginn(name))
                .expect("der Wachfaden lebt, solange der Laeufer laeuft");
        }
        let ergebnis = catch_unwind(AssertUnwindSafe(|| (probe.lauf)(mtm)));
        if mit_frist {
            wache
                .send(Wachruf::Ende)
                .expect("der Wachfaden lebt, solange der Laeufer laeuft");
        }
        if ergebnis.is_ok() {
            println!("ok");
            gruen += 1;
        } else {
            println!("FAILED");
            gescheitert.push(name);
        }
    }

    let gefahren = gruen + gescheitert.len();
    if !gescheitert.is_empty() {
        println!("\nfailures:");
        for name in &gescheitert {
            println!("    {name}");
        }
    }
    println!(
        "\ntest result: {}. {gruen} passed; {} failed; {ignoriert} ignored; 0 measured; \
         {ausgefiltert} filtered out; finished in {:.2}s\n",
        if gescheitert.is_empty() {
            "ok"
        } else {
            "FAILED"
        },
        gescheitert.len(),
        beginn.elapsed().as_secs_f64()
    );
    if gescheitert.is_empty() {
        Ausgang::Gruen { gefahren }
    } else {
        Ausgang::Rot {
            gefahren,
            gescheitert: gescheitert.len(),
        }
    }
}

/// Legt die [`WERKSVORGABEN`] im fluechtigen Argumentbereich fest und liest
/// sie ueber `standardUserDefaults` zurueck.
///
/// Der vorhandene Bereich wird gelesen, ergaenzt, entfernt und neu gesetzt,
/// denn `setVolatileDomain:forName:` wirft, wenn der Name schon steht; was die
/// Befehlszeile dort eingetragen hat, bleibt also stehen. Geschrieben wird
/// nichts, auch nicht in die Domaene des Pruefprogramms. Weicht ein
/// zurueckgelesener Wert ab, faehrt der Laeufer keine Probe.
fn werksvorgaben_festlegen() -> Result<(), Ausgang> {
    let vorgaben = NSUserDefaults::standardUserDefaults();
    // SAFETY: Ein Fremdsymbol von Foundation, der Name des Argumentbereichs. Es
    // wird gelesen und nicht geschrieben.
    let name: &NSString = unsafe { NSArgumentDomain };
    // Gefragt ueber `msg_send!` und nicht ueber die Bindung der Kiste: die
    // fuehrt beide Antworten als nie leer, und am 261101 liefert
    // `volatileDomainNames` in diesem Pruefziel `nil` (gemessen; die Bindung
    // bricht dann ab). Ein fehlender Bereich ist hier kein Fehler, sondern der
    // Fall, in dem es nichts zu ergaenzen gibt.
    // SAFETY: `volatileDomainForName:` nimmt ein `NSString` und gibt ein
    // `NSDictionary` mit Zeichenketten als Schluesseln zurueck, oder `nil`.
    let vorhanden: Option<Retained<NSDictionary<NSString, AnyObject>>> =
        unsafe { msg_send![&*vorgaben, volatileDomainForName: name] };
    let bereich: Retained<NSMutableDictionary<NSString, AnyObject>> = match vorhanden {
        Some(vorhanden) => {
            let ergaenzt = vorhanden.mutableCopy();
            vorgaben.removeVolatileDomainForName(name);
            ergaenzt
        }
        None => NSMutableDictionary::new(),
    };
    let ja = NSNumber::new_bool(true);
    for (schluessel, _grund) in WERKSVORGABEN {
        bereich.insert(&*NSString::from_str(schluessel), ja.as_ref() as &AnyObject);
    }
    // SAFETY: Das Verzeichnis traegt Zeichenketten als Schluessel und
    // `NSNumber` oder die Werte des vorigen Argumentbereichs als Werte, also
    // genau die Typen, die ein Vorgabenbereich fuehrt; der Name steht nach dem
    // Entfernen nicht mehr.
    unsafe { vorgaben.setVolatileDomain_forName(&bereich, name) };

    for (schluessel, _grund) in WERKSVORGABEN {
        let schluessel_text = NSString::from_str(schluessel);
        let gelesen = vorgaben
            .objectForKey(&schluessel_text)
            .map_or_else(|| "nichts".to_owned(), |wert| beschreibung(&wert));
        if !vorgaben.boolForKey(&schluessel_text) {
            eprintln!(
                "hauptfaden: die Werksvorgabe {schluessel} liest nach dem Festlegen als \
                 {gelesen} zurueck und nicht als 1; keine Probe ist gefahren"
            );
            return Err(Ausgang::VorgabeHaeltNicht {
                schluessel,
                gelesen,
            });
        }
        println!("werksvorgabe {schluessel} = {gelesen} (Argumentbereich)");
    }
    Ok(())
}

/// Die Beschreibung eines Vorgabewerts, wie Foundation sie liefert.
fn beschreibung(wert: &AnyObject) -> String {
    // SAFETY: `description` steht an `NSObject`, gibt ein `NSString` zurueck und
    // nimmt nichts; jeder Wert eines Vorgabenbereichs ist ein `NSObject`.
    let text: Retained<NSString> = unsafe { msg_send![wert, description] };
    text.to_string()
}

/// Kuendigt eine Probe an, bevor sie laeuft: `test <name> ... ` ohne
/// Zeilenende, gespuelt, und das Ergebnis folgt auf derselben Zeile, wie
/// `libtest` es tut.
///
/// **Warum vorher.** `catch_unwind` faengt allein eine Panik in Rust. Eine
/// Ausnahme von Objective-C, ein `SIGSEGV` an einer `NSTextView` oder eine
/// Panik ueber eine `extern "C"`-Grenze beenden den Prozess, und stuende der
/// Name erst nach der Probe da, naennte die letzte Zeile die vorige Probe mit
/// `ok`. Gespuelt wird, weil die Standardausgabe an einer Roehre
/// blockgepuffert ist und ein Abbruch den Puffer nicht mehr schreibt.
fn probe_ansagen(aus: &mut impl Write, name: &str) -> std::io::Result<()> {
    write!(aus, "test {name} ... ")?;
    aus.flush()
}

/// Was der Laeufer dem Wachfaden meldet.
enum Wachruf {
    Beginn(&'static str),
    Ende,
}

/// Startet den Wachfaden, der jede Probe unter `frist` haelt.
///
/// Auf `Beginn` wartet er hoechstens `frist` auf `Ende`; kommt es nicht,
/// schreibt er den Namen und beendet den Prozess. Faellt der Sender, endet er.
/// Laesst er sich nicht starten, bekommt der Rufer den Fehler und faehrt keine
/// Probe.
fn wachfaden_starten(frist: Duration) -> std::io::Result<Sender<Wachruf>> {
    let (sender, empfaenger) = mpsc::channel::<Wachruf>();
    std::thread::Builder::new()
        .name("krk-probenwache".to_owned())
        .spawn(move || {
            while let Ok(ruf) = empfaenger.recv() {
                let Wachruf::Beginn(name) = ruf else {
                    continue;
                };
                match empfaenger.recv_timeout(frist) {
                    Ok(_) => {}
                    Err(RecvTimeoutError::Timeout) => {
                        eprintln!(
                            "hauptfaden: {name} ist nach {} s nicht zu Ende; der Lauf bricht ab",
                            frist.as_secs()
                        );
                        std::process::exit(ROT);
                    }
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
        })
        .map(|_griff| sender)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Eine Standardausgabe zum Mitlesen: was geschrieben ist und wie viel
    /// davon beim letzten Spuelen stand.
    #[derive(Default)]
    struct Mitschrift {
        geschrieben: Vec<u8>,
        gespuelt_bis: Option<usize>,
    }

    impl Write for Mitschrift {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.geschrieben.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.gespuelt_bis = Some(self.geschrieben.len());
            Ok(())
        }
    }

    /// Die Ansage ist `test <name> ... ` ohne Zeilenende, und sie ist ganz
    /// gespuelt, bevor `laufen` die Probe ruft. Dass `laufen` sie vor
    /// `probe.lauf` ruft, sieht diese Probe nicht; das sagt der Rumpf.
    #[test]
    fn die_ansage_steht_ohne_zeilenende_und_ist_gespuelt() {
        let mut aus = Mitschrift::default();
        probe_ansagen(&mut aus, "appkit::editor::tests::die_sammeltuer")
            .expect("eine Mitschrift scheitert nicht");
        assert_eq!(
            aus.geschrieben,
            b"test appkit::editor::tests::die_sammeltuer ... "
        );
        assert_eq!(aus.gespuelt_bis, Some(aus.geschrieben.len()));
    }

    fn woerter(zeile: &str) -> Vec<String> {
        zeile.split_whitespace().map(str::to_owned).collect()
    }

    fn auswahl(
        filter: &[&str],
        genau: bool,
        auslassen: &[&str],
        ignorierte: Ignorierte,
        nur_auflisten: bool,
    ) -> Auswahl {
        Auswahl {
            filter: filter.iter().map(|wort| (*wort).to_owned()).collect(),
            genau,
            auslassen: auslassen.iter().map(|wort| (*wort).to_owned()).collect(),
            ignorierte,
            nur_auflisten,
        }
    }

    /// Die Befehlszeile als Tafel: Woerter hinein, Auswahl heraus.
    #[test]
    fn die_befehlszeile_waehlt_wie_libtest() {
        use Ignorierte::{Allein, Auslassen, Dazu};
        let tafel = [
            ("", auswahl(&[], false, &[], Auslassen, false)),
            (
                "die_sammeltuer",
                auswahl(&["die_sammeltuer"], false, &[], Auslassen, false),
            ),
            (
                "a b --exact",
                auswahl(&["a", "b"], true, &[], Auslassen, false),
            ),
            (
                "--skip x --skip=y z",
                auswahl(&["z"], false, &["x", "y"], Auslassen, false),
            ),
            ("--ignored", auswahl(&[], false, &[], Allein, false)),
            ("--include-ignored", auswahl(&[], false, &[], Dazu, false)),
            ("--list", auswahl(&[], false, &[], Auslassen, true)),
            (
                "--test-threads 1 --format pretty --color never --logfile f --shuffle-seed 7 \
                 -Z unstable-options a",
                auswahl(&["a"], false, &[], Auslassen, false),
            ),
            (
                "--nocapture -q --test-threads=2 a",
                auswahl(&["a"], false, &[], Auslassen, false),
            ),
        ];
        for (zeile, erwartet) in tafel {
            assert_eq!(auswerten(woerter(zeile)), erwartet, "{zeile}");
        }
    }

    /// Die Auswahl trifft als Teiltext, mit `--exact` als ganzer Name, und
    /// `--skip` laesst aus.
    #[test]
    fn die_auswahl_trifft_teiltext_ganzen_namen_und_laesst_aus() {
        let name = "appkit::editor::tests::die_sammeltuer_ist_eine_sicht_auf_dieselben_bits";
        assert!(auswerten(woerter("")).trifft(name));
        assert!(auswerten(woerter("sammeltuer")).trifft(name));
        assert!(auswerten(woerter("fremd sammeltuer")).trifft(name));
        assert!(!auswerten(woerter("fremd")).trifft(name));
        assert!(!auswerten(woerter("sammeltuer --exact")).trifft(name));
        assert!(auswerten(woerter(&format!("{name} --exact"))).trifft(name));
        assert!(!auswerten(woerter("--skip appkit::editor::tests::")).trifft(name));
        assert!(!auswerten(woerter("--skip sammeltuer --exact")).trifft("sammeltuer"));
    }

    /// Gruen liest `cargo test` allein einen Lauf ohne Fehlschlag und das
    /// Auflisten; jede Weigerung vor der ersten Probe ist rot.
    #[test]
    fn gruen_ist_allein_ein_lauf_ohne_fehlschlag_und_das_auflisten() {
        assert!(Ausgang::Gruen { gefahren: 4 }.ist_gruen());
        assert!(Ausgang::Aufgelistet { anzahl: 4 }.ist_gruen());
        assert!(
            !Ausgang::Rot {
                gefahren: 4,
                gescheitert: 1
            }
            .ist_gruen()
        );
        assert!(!Ausgang::NichtAufDemHauptfaden.ist_gruen());
        assert!(!Ausgang::OhneWache.ist_gruen());
        assert!(
            !Ausgang::VorgabeHaeltNicht {
                schluessel: WERKSVORGABEN[0].0,
                gelesen: "0".to_owned()
            }
            .ist_gruen()
        );
    }

    /// Unter `libtest` steht keine Probe auf dem Hauptfaden, und der Laeufer
    /// verweigert dort, bevor er eine Probe faehrt oder eine Vorgabe setzt.
    #[test]
    fn der_laeufer_verweigert_ausserhalb_des_hauptfadens() {
        assert!(
            MainThreadMarker::new().is_none(),
            "die Voraussetzung: libtest faehrt diese Probe nicht auf dem Hauptfaden"
        );
        assert_eq!(laufen(Vec::<String>::new()), Ausgang::NichtAufDemHauptfaden);
    }

    /// Jede Hauptfadenprobe traegt einen eigenen Namen, und jeder steht unter
    /// `appkit::` in einem Pruefmodul.
    #[test]
    fn die_hauptfadenproben_tragen_eindeutige_namen_unter_appkit() {
        let proben = alle();
        assert!(!proben.is_empty(), "das Pruefziel faehrt keine Probe");
        let namen: BTreeSet<&str> = proben.iter().map(|probe| probe.name()).collect();
        assert_eq!(
            namen.len(),
            proben.len(),
            "zwei Proben tragen denselben Namen"
        );
        for name in namen {
            assert!(
                name.starts_with("appkit::") && name.contains("::tests::"),
                "{name} steht nicht in einem Pruefmodul unter appkit"
            );
        }
    }

    /// Keine Codezeile unter `krk-ui/src` behauptet den Hauptfaden mit
    /// `MainThreadMarker::new_unchecked`.
    ///
    /// Eine Probe, die AppKit baut, bekommt ihren Marker vom Laeufer auf dem
    /// Hauptfaden; eine, die wieder als `#[test]` geschrieben wird, erreicht
    /// AppKit nur ueber diese Behauptung und macht diese Probe rot. Gelesen
    /// werden die Codezeilen ([`crate::quellbaum::codezeilen`]), also nennt ein
    /// Kommentar den Namen ungestraft. Die Nadel ist zusammengesetzt, damit die
    /// Probe sich nicht selbst findet.
    ///
    /// **Was sie nicht sieht:** einen Marker, den eine Probe auf anderem Weg
    /// erfindet, etwa ueber `transmute` oder `MainThreadMarker::from` an einem
    /// Wert, der selbst nur behauptet ist, und eine Zeile, die den Aufruf hinter
    /// einen Kommentar auf derselben Zeile stellt. Sie liest allein `krk-ui`; die
    /// uebrigen Kisten duerfen `objc2` nicht kennen.
    #[test]
    fn kein_pruefcode_in_krk_ui_behauptet_den_hauptfaden() {
        let nadel = concat!("new_", "unchecked");
        let dateien = crate::quellbaum::quelldateien();
        let gelesen: Vec<&(String, String)> = dateien
            .iter()
            .filter(|(name, _)| name.starts_with("krk-ui/src/"))
            .collect();
        assert!(
            gelesen
                .iter()
                .any(|(name, _)| name == "krk-ui/src/appkit/hauptfaden.rs"),
            "die Lesart findet die Dateien von krk-ui nicht"
        );
        let fundstellen: Vec<String> = gelesen
            .iter()
            .flat_map(|(name, inhalt)| {
                crate::quellbaum::codezeilen(inhalt)
                    .filter(|zeile| zeile.contains(nadel))
                    .map(move |zeile| format!("{name}: {}", zeile.trim()))
            })
            .collect();
        assert!(
            fundstellen.is_empty(),
            "diese Zeilen behaupten den Hauptfaden, statt den Marker vom Laeufer des \
             Pruefziels hauptfaden zu nehmen:\n{}",
            fundstellen.join("\n")
        );
    }

    /// Die `mod`-Zeilen einer Kistenwurzel samt der Attributzeile unmittelbar
    /// davor, in ihrer Reihenfolge.
    fn mod_zeilen(inhalt: &str) -> Vec<(Option<&str>, &str)> {
        let zeilen: Vec<&str> = crate::quellbaum::codezeilen(inhalt).collect();
        zeilen
            .iter()
            .enumerate()
            .filter(|(_, zeile)| zeile.starts_with("mod ") || zeile.starts_with("pub mod "))
            .map(|(nummer, zeile)| {
                let davor = nummer
                    .checked_sub(1)
                    .map(|vorige| zeilen[vorige])
                    .filter(|vorige| vorige.starts_with("#["));
                (davor, *zeile)
            })
            .collect()
    }

    /// Die zweite Kistenwurzel fuehrt dieselben `mod`-Zeilen wie `main.rs`,
    /// samt Attributzeile und in derselben Reihenfolge.
    ///
    /// Ein neues Modul in `main.rs`, das hier fehlt, liesse seine Proben aus
    /// dem Pruefziel fallen, ohne dass der Bau es immer merkte; ein Modul, das
    /// ein anderes braucht, ist ausserdem ein Uebersetzungsfehler. Die
    /// Doc-Kommentare an den Modulen stehen allein in `main.rs` und werden
    /// nicht verglichen.
    #[test]
    fn die_zweite_kistenwurzel_fuehrt_die_mod_zeilen_von_main() {
        let dateien = crate::quellbaum::quelldateien();
        let inhalt = |gesucht: &str| {
            dateien
                .iter()
                .find(|(name, _)| name == gesucht)
                .map(|(_, inhalt)| inhalt.as_str())
                .unwrap_or_else(|| panic!("{gesucht} steht nicht im Baum"))
        };
        let haupt = mod_zeilen(inhalt("krk-ui/src/main.rs"));
        let zweite = mod_zeilen(inhalt("krk-ui/src/hauptfaden.rs"));
        assert!(
            haupt.iter().any(|(_, zeile)| *zeile == "mod appkit;"),
            "die Lesart findet in main.rs die mod-Zeilen nicht"
        );
        assert_eq!(
            zweite, haupt,
            "die mod-Zeilen von src/hauptfaden.rs weichen von src/main.rs ab"
        );
    }
}
