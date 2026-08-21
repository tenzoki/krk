//! Die Version setzen, eintragen und taggen: `cargo xtask version <zahl>`.
//!
//! Der erste der beiden Halbschritte des Auslieferungswegs; der zweite ist
//! `cargo xtask release`. Zusammen sind sie das eine Kommando mit dem einen
//! Argument, das `./release.sh 0.2.0` fuehrt:
//!
//! ```text
//! ./release.sh 0.2.0
//!   └─ make ausliefern VERSION=0.2.0
//!        ├─ cargo xtask version 0.2.0    ← dieses Modul: Zahl, Eintrag, Tag
//!        └─ cargo xtask release          ← release.rs: die sieben Stationen
//! ```
//!
//! **Warum es zwei Kommandos sind und nicht eines.** `xtask` liest die
//! Versionszahl ueber `env!("CARGO_PKG_VERSION")`, also beim **Uebersetzen**.
//! Ein Lauf, der die `Cargo.toml` aendert und danach im selben Prozess weiter
//! ausliefert, traegt bis zu seinem Ende die alte Zahl mit sich: die
//! `Info.plist` bekaeme sie eingesetzt, waehrend der Tag die neue nennt, und
//! Station 1 sagte dazu nichts, weil sie dieselbe alte Zahl vergleicht. Der
//! Prozess muss also enden, damit `cargo` `xtask` neu uebersetzt. Nachgemessen
//! am 260813: `cargo run` uebersetzt nach einer Aenderung an
//! `[workspace.package].version` neu und meldet es als `Compiling xtask
//! v<neue Zahl>`, und das Programm gibt danach die neue Zahl aus.
//!
//! Genau daraus wird die Trennung zur Pruefung: Station 1 von `release` laeuft
//! im **neu uebersetzten** Werkzeug und vergleicht die eingebackene Zahl mit
//! dem Tag. Bliebe ein altes Werkzeug stehen, faende sie den Tag `v0.2.0`
//! nicht und braeche ab. Der Umweg ueber zwei Prozesse ist damit nicht der
//! Preis der Loesung, sondern ihr Wachposten.
//!
//! **Die `Cargo.lock` gehoert zum selben Schritt.** Sie fuehrt die Zahl fuer
//! jedes der vier Mitglieder mit, und `cargo` schreibt sie beim naechsten Bau
//! von sich aus nach. Bliebe sie hier liegen, frischte der Bau von
//! `cargo xtask release` sie auf, und Station 1 saehe unmittelbar danach einen
//! geaenderten Arbeitsbaum und braeche ab — an einer Datei, die das Werkzeug
//! selbst erzeugt hat. Aufgefrischt wird sie nicht von Hand, sondern von
//! `cargo update --workspace --offline`: `--workspace` ruehrt keine fremde
//! Kiste an, `--offline` geht nicht ins Netz. Dass dieser Aufruf aus einem
//! laufenden `cargo run` heraus nicht an der Bausperre haengenbleibt, ist am
//! 260813 nachgemessen.
//!
//! **Was ein Abbruch hinterlaesst.** Alles, was ohne Schreiben zu pruefen ist,
//! wird vor dem ersten Schreiben geprueft: die Zahl, das Git-Verzeichnis, der
//! Arbeitsbaum und der Tag. Danach gibt es zwei Fenster. Scheitert die
//! `Cargo.lock` oder der Eintrag, schreibt [`zuruecknehmen`] beide Dateien auf
//! ihren vorigen Stand zurueck, und es bleibt nichts. Scheitert allein das
//! Setzen des Tags, **steht der Eintrag** und wird nicht zurueckgenommen: eine
//! Ruecknahme schriebe Geschichte um, und der Eintrag ist fuer sich richtig.
//! Die Meldung sagt dann, was steht und welcher Handgriff fehlt. Dasselbe gilt
//! fuer einen Abbruch der sieben Stationen danach: Eintrag und Tag bleiben
//! stehen, und ein zweiter Lauf desselben `./release.sh 0.2.0` faellt hier
//! durch, ohne etwas zu tun, und faehrt gleich weiter zu `release`.

use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

use crate::Abbruch;
use crate::bundle;
use crate::git;

/// Der Abschnitt der Wurzel-`Cargo.toml`, der die Versionszahl fuehrt.
///
/// **Die eine Stelle.** Jedes Mitglied erbt sie ueber `version.workspace =
/// true`, die `Info.plist` bekommt sie beim Buendeln eingesetzt, und `xtask`
/// liest sie beim Uebersetzen. Ein zweiter Ort ist ausgeschlossen, und deshalb
/// schreibt dieses Modul genau eine Zeile.
const ABSCHNITT: &str = "[workspace.package]";

/// Die beiden Dateien, die der Versionsschritt eintraegt.
///
/// Die zweite ist abgeleitet und nicht Quelle: `cargo` schreibt sie, dieses
/// Modul reicht sie nur mit ein. Warum sie mit muss, steht im Modulkopf.
const EINGETRAGENE: [&str; 2] = ["Cargo.toml", "Cargo.lock"];

/// Was der Lauf zu tun hat, nachdem er Zahl und Tag verglichen hat.
#[derive(Debug, PartialEq, Eq)]
enum Vorhaben {
    /// Die Zahl steht, der Tag steht auf HEAD: ein zweiter Lauf desselben
    /// Kommandos. Es wird nichts geschrieben.
    NichtsZuTun,
    /// Die Zahl steht schon, der Tag fehlt noch.
    NurTaggen,
    /// Der gewoehnliche Fall: Zahl setzen, eintragen, taggen.
    SetzenEintragenTaggen,
}

/// Setzt die Versionszahl, traegt sie ein und setzt den Tag `v<zahl>`.
pub(crate) fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let [zahl] = argumente else {
        return Err(Abbruch::Aufruf(format!(
            "version nimmt genau ein Argument, die Versionszahl, und hat {} bekommen",
            argumente.len()
        )));
    };
    versionszahl_pruefen(zahl).map_err(Abbruch::Aufruf)?;
    let tagname = format!("v{zahl}");
    let wurzel = bundle::wurzel();

    // Erst fragen, dann schreiben. Die vier Vorpruefungen kosten nichts und
    // lassen den Baum, wie er ist.
    git::rufen(&wurzel, git::VERZEICHNIS).map_err(|fehler| {
        let grund = match fehler {
            Abbruch::Lauf(text) | Abbruch::Aufruf(text) => text,
        };
        Abbruch::Lauf(format!(
            "Die Auslieferung braucht ein Git-Verzeichnis, und in {} ist keines zu befragen: \
             {grund}\n\
             \n\
             Die Versionszahl wird eingetragen und getaggt; ohne Git-Verzeichnis ist weder das \
             eine noch das andere moeglich. Es wird nichts geschrieben.",
            wurzel.display()
        ))
    })?;

    let stand = git::rufen(&wurzel, git::STAND)?;
    let geaendert = git::geaenderte_dateien(&stand);
    if !geaendert.is_empty() {
        return Err(Abbruch::Lauf(arbeitsbaum_meldung(&geaendert, zahl)));
    }

    let auf_head = git::rufen(&wurzel, git::TAGS_AUF_HEAD)?;
    let vorhandene = git::rufen(&wurzel, &tagliste_argumente(&tagname))?;
    let tag_auf_head = git::tag_steht(&auf_head, &tagname);
    let tag_existiert = git::tag_steht(&vorhandene, &tagname);

    let pfad = wurzel.join(EINGETRAGENE[0]);
    let inhalt = fs::read_to_string(&pfad).map_err(|fehler| {
        Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", pfad.display()))
    })?;
    let (spanne, alt) = versionsfeld_finden(&inhalt).map_err(Abbruch::Lauf)?;
    let alt = alt.to_owned();

    match vorhaben_bestimmen(&alt, zahl, &tagname, tag_existiert, tag_auf_head)
        .map_err(Abbruch::Lauf)?
    {
        Vorhaben::NichtsZuTun => {
            println!(
                "Die Cargo.toml fuehrt die Version {zahl}, und HEAD traegt den Tag {tagname}. Es \
                 ist nichts zu tun."
            );
            Ok(())
        }
        Vorhaben::NurTaggen => {
            println!(
                "Die Cargo.toml fuehrt die Version {zahl} bereits; es wird nichts eingetragen."
            );
            taggen(&wurzel, &tagname, false)
        }
        Vorhaben::SetzenEintragenTaggen => {
            setzen_eintragen_taggen(&wurzel, &pfad, &inhalt, &spanne, zahl, &tagname)
        }
    }
}

/// Der gewoehnliche Fall: Zahl setzen, `Cargo.lock` auffrischen, eintragen,
/// taggen.
///
/// Ab der ersten Zeile ist etwas geschrieben, und ab hier gilt, was der
/// Modulkopf unter „Was ein Abbruch hinterlaesst" festhaelt.
fn setzen_eintragen_taggen(
    wurzel: &Path,
    pfad: &Path,
    inhalt: &str,
    spanne: &Range<usize>,
    neu: &str,
    tagname: &str,
) -> Result<(), Abbruch> {
    let alt = &inhalt[spanne.clone()];
    let sperrpfad = wurzel.join(EINGETRAGENE[1]);
    let sperrinhalt = fs::read_to_string(&sperrpfad).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} ist nicht lesbar: {fehler}\n\
             \n\
             Sie wird mit eingetragen, weil sie die Zahl fuer jedes Mitglied mitfuehrt. Es wird \
             nichts geschrieben.",
            sperrpfad.display()
        ))
    })?;
    let voriger: Vec<(PathBuf, String)> = vec![
        (pfad.to_path_buf(), inhalt.to_owned()),
        (sperrpfad.clone(), sperrinhalt),
    ];

    let mut gesetzt = String::with_capacity(inhalt.len() + neu.len());
    gesetzt.push_str(&inhalt[..spanne.start]);
    gesetzt.push_str(neu);
    gesetzt.push_str(&inhalt[spanne.end..]);
    fs::write(pfad, &gesetzt).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} ist nicht schreibbar: {fehler}. Es ist nichts geaendert.",
            pfad.display()
        ))
    })?;
    println!("Version {alt} → {neu} in {}.", pfad.display());

    if let Err(fehler) = sperrdatei_auffrischen(wurzel) {
        return Err(mit_ruecknahme(fehler, &voriger));
    }

    let meldung = eintragsmeldung(neu);
    if let Err(fehler) = git::rufen(wurzel, &eintrag_argumente(&meldung)) {
        return Err(mit_ruecknahme(fehler, &voriger));
    }
    println!("Eingetragen: {meldung}");

    // Ab hier steht der Eintrag. Er wird nicht zurueckgenommen; warum, steht im
    // Modulkopf.
    taggen(wurzel, tagname, true)
}

/// Setzt den Tag auf HEAD.
///
/// `eintrag_steht` entscheidet allein ueber den Wortlaut des Abbruchs: ist der
/// Eintrag gerade entstanden, muss die Meldung sagen, dass er stehenbleibt.
fn taggen(wurzel: &Path, tagname: &str, eintrag_steht: bool) -> Result<(), Abbruch> {
    match git::rufen(wurzel, &tag_argumente(tagname)) {
        Ok(_) => {
            println!("Tag {tagname} steht auf HEAD.");
            Ok(())
        }
        Err(fehler) => {
            let grund = match fehler {
                Abbruch::Lauf(text) | Abbruch::Aufruf(text) => text,
            };
            let eintrag = if eintrag_steht {
                "Der Eintrag mit der neuen Zahl steht und wird nicht zurueckgenommen: er ist fuer \
                 sich richtig, und eine Ruecknahme schriebe Geschichte um. "
            } else {
                ""
            };
            Err(Abbruch::Lauf(format!(
                "Der Tag {tagname} liess sich nicht setzen: {grund}\n\
                 \n\
                 {eintrag}Was fehlt, ist der Tag. Derselbe Aufruf noch einmal holt ihn nach und \
                 traegt nichts zweites ein.\n\
                 \n\
                 Es entsteht kein Auslieferungspaket."
            )))
        }
    }
}

/// Schreibt die vorigen Staende zurueck und haengt das Ergebnis an die Meldung.
///
/// Der urspruengliche Abbruch bleibt die Ursache; die Ruecknahme ist eine
/// Nachricht darueber, was der Baum jetzt traegt. Misslingt sie, sagt die
/// Meldung das und nennt den Handgriff — verschwiegen wird sie nicht.
fn mit_ruecknahme(fehler: Abbruch, voriger: &[(PathBuf, String)]) -> Abbruch {
    let grund = match fehler {
        Abbruch::Lauf(text) | Abbruch::Aufruf(text) => text,
    };
    Abbruch::Lauf(format!("{grund}\n\n{}", zuruecknehmen(voriger)))
}

/// Schreibt die gemerkten Staende zurueck und meldet, ob es gelungen ist.
fn zuruecknehmen(voriger: &[(PathBuf, String)]) -> String {
    let misslungen: Vec<String> = voriger
        .iter()
        .filter(|(pfad, inhalt)| fs::write(pfad, inhalt).is_err())
        .map(|(pfad, _)| pfad.display().to_string())
        .collect();
    if misslungen.is_empty() {
        "Zurueckgenommen: Cargo.toml und Cargo.lock stehen wieder auf ihrem vorigen Stand, und es \
         ist weder ein Eintrag noch ein Tag entstanden."
            .to_owned()
    } else {
        format!(
            "Achtung: diese Dateien liessen sich nicht zurueckschreiben und stehen auf einem \
             halben Stand:\n\
             \x20      {}\n\
             \n\
             Der Baum ist mit `git checkout -- Cargo.toml Cargo.lock` wieder herzustellen. Ein \
             Eintrag oder Tag ist nicht entstanden.",
            misslungen.join("\n\x20      ")
        )
    }
}

/// Frischt die `Cargo.lock` auf die neue Zahl auf.
///
/// Warum sie ueberhaupt mitmuss und warum es `cargo` tut und nicht dieses
/// Modul, steht im Modulkopf.
fn sperrdatei_auffrischen(wurzel: &Path) -> Result<(), Abbruch> {
    let cargo = bundle::cargo();
    let argumente = ["update", "--workspace", "--offline"];
    let status = std::process::Command::new(&cargo)
        .current_dir(wurzel)
        .args(argumente)
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("{cargo} laesst sich nicht starten: {fehler}")))?;
    if !status.success() {
        return Err(Abbruch::Lauf(format!(
            "cargo {} ist gescheitert ({status}). Ohne aufgefrischte Cargo.lock traegt der \
             naechste Bau sie nach, und Station 1 von `cargo xtask release` faende einen \
             geaenderten Arbeitsbaum.",
            argumente.join(" ")
        )));
    }
    Ok(())
}

/// Die sechs Faelle aus Zahl und Tag, drei davon ein Abbruch.
///
/// | `Cargo.toml` fuehrt | Tag `v<neu>` | Vorhaben |
/// |---|---|---|
/// | die neue Zahl | steht nicht | nur taggen |
/// | die neue Zahl | steht auf HEAD | nichts zu tun |
/// | die neue Zahl | steht anderswo | Abbruch |
/// | eine andere Zahl | steht nicht | setzen, eintragen, taggen |
/// | eine andere Zahl | steht auf HEAD | Abbruch |
/// | eine andere Zahl | steht anderswo | Abbruch |
///
/// Die Aufteilung ist ueberschneidungsfrei und vollstaendig. Die beiden
/// Wahrheitswerte spannen vier Paare auf, aber nur drei Lagen: ein Tag auf
/// HEAD, den es nicht gibt, kommt nicht vor. Das vierte Paar faellt deshalb
/// nicht durch, sondern mit `tag_existiert == false` in dieselbe Zeile wie
/// „steht nicht" — eine Lage ohne Aufrufer, die trotzdem eine Antwort bekommt.
///
/// **Der dritte Abbruchgrund ist der unscheinbarste und der wichtigste.** Steht
/// der Tag schon auf HEAD, waehrend die `Cargo.toml` eine andere Zahl fuehrt,
/// dann schoebe ein Eintrag HEAD um einen Schritt weiter und liesse den Tag auf
/// dem Commit davor zurueck. Der Tag benennte danach einen Stand, dessen
/// `Cargo.toml` die alte Zahl fuehrt, und das Buendel truege eine Zahl, die
/// kein Tag nennt.
fn vorhaben_bestimmen(
    alt: &str,
    neu: &str,
    tagname: &str,
    tag_existiert: bool,
    tag_auf_head: bool,
) -> Result<Vorhaben, String> {
    if tag_existiert && !tag_auf_head {
        return Err(format!(
            "Der Tag {tagname} steht schon, aber nicht auf HEAD.\n\
             \n\
             Ein Tag benennt genau einen Stand, und dieser Name ist vergeben. Er wird nicht \
             verschoben: der Stand, den er heute benennt, waere danach unauffindbar. Wer ihn \
             gemeint hat, sieht ihn sich an; wer eine neue Auslieferung meint, nimmt die \
             naechste Zahl:\n\
             \x20      git show {tagname}\n\
             \n\
             Es ist nichts geschrieben worden."
        ));
    }
    if !tag_existiert {
        return Ok(if alt == neu {
            Vorhaben::NurTaggen
        } else {
            Vorhaben::SetzenEintragenTaggen
        });
    }
    if alt == neu {
        return Ok(Vorhaben::NichtsZuTun);
    }
    Err(format!(
        "Der Tag {tagname} steht auf HEAD, aber die Cargo.toml fuehrt die Version {alt}.\n\
         \n\
         Ein Eintrag mit der Zahl {neu} schoebe HEAD einen Schritt weiter und liesse {tagname} \
         auf dem Commit davor zurueck; der Tag benennte danach einen Stand mit der Zahl {alt}. \
         Zu klaeren ist, was gilt: der Tag oder die Zahl.\n\
         \n\
         Es ist nichts geschrieben worden."
    ))
}

/// Nennt die geaenderten verfolgten Dateien und sagt, warum keine ausreicht.
///
/// **Die Aufzaehlung ist der Punkt.** „Der Arbeitsbaum ist geaendert" laesst
/// den Leser suchen; vier Dateien beim Namen sagen ihm in einer Zeile, ob er
/// vergessene Arbeit vor sich hat oder den bekannten Befund aus
/// `shared/issues/260813-1515_*`, den kein Agentenlauf loslaesst.
fn arbeitsbaum_meldung(geaendert: &[&str], zahl: &str) -> String {
    let aufzaehlung: Vec<String> = geaendert
        .iter()
        .map(|zeile| format!("\x20      {zeile}"))
        .collect();
    let zahlwort = if geaendert.len() == 1 {
        "1 verfolgte Datei ist".to_owned()
    } else {
        format!("{} verfolgte Dateien sind", geaendert.len())
    };
    format!(
        "Der Arbeitsbaum weicht vom eingetragenen Stand ab; {zahlwort} geaendert:\n\
         \n\
         {}\n\
         \n\
         Ein Eintrag mit der Zahl {zahl} naehme sie alle mit, und der Tag benennte danach einen \
         Stand, den so niemand gemeint hat. Unbeachtete Dateien zaehlen nicht mit.\n\
         \n\
         Abhilfe: die Aenderungen eintragen oder wegstellen:\n\
         \x20      git commit -a\n\
         \x20      git stash\n\
         \n\
         Stehen darunter allein Dateien unter fusion-workbench/, die jeder Agentenlauf neu \
         schreibt, ist es der bekannte Befund\n\
         \x20      shared/issues/260813-1515_*_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md\n\
         \n\
         Es ist nichts geschrieben worden.",
        aufzaehlung.join("\n")
    )
}

/// Prueft die Zahl aus der Befehlszeile: genau drei Zahlenteile mit Punkten.
///
/// **Warum so streng.** Die Zahl wird in eine Datei geschrieben, eingetragen
/// und zum Tagnamen; ein Tippfehler traegt sich fort und ist danach nur noch
/// von Hand aus der Geschichte zu holen. Die drei Stufen aus `README.md`
/// kennen genau drei Zahlen, also nimmt diese Pruefung genau drei. Ein Anhang
/// wie `-rc1` ist damit ausgeschlossen: ihn zuzulassen waere eine Zusage ueber
/// Vorabstaende, die dieses Projekt nicht gibt, und wer sie geben will, aendert
/// diese Funktion und weiss dann, was er tut.
///
/// `pub(crate)` seit dem 260820: `beglaubigen` nimmt dieselbe Zahl entgegen
/// und stellt an sie dieselbe Anforderung. Eine zweite Pruefung daneben waere
/// eine zweite Vorschrift darueber, wie eine Versionszahl dieses Projekts
/// aussieht.
pub(crate) fn versionszahl_pruefen(roh: &str) -> Result<(), String> {
    if let Some(ohne) = roh.strip_prefix('v') {
        return Err(format!(
            "die Versionszahl steht ohne `v`: {ohne} statt {roh}. Das `v` traegt allein der Tag, \
             und den setzt das Werkzeug"
        ));
    }
    let teile: Vec<&str> = roh.split('.').collect();
    if teile.len() != 3 {
        return Err(format!(
            "die Versionszahl hat drei Teile, Haupt.Neben.Behebung, und {roh:?} hat {}. Was wann \
             steigt, steht in README.md unter „Versionsstufen\"",
            teile.len()
        ));
    }
    for teil in teile {
        if teil.is_empty() || !teil.bytes().all(|zeichen| zeichen.is_ascii_digit()) {
            return Err(format!(
                "jeder Teil der Versionszahl ist eine Zahl, und {teil:?} in {roh:?} ist keine"
            ));
        }
        if teil.len() > 1 && teil.starts_with('0') {
            return Err(format!(
                "eine fuehrende Null macht zwei Schreibweisen fuer dieselbe Zahl: {teil:?} in \
                 {roh:?}"
            ));
        }
    }
    Ok(())
}

/// Findet die Versionszeile unter [`ABSCHNITT`] und liefert Spanne und Wert.
///
/// **Warum von Hand und nicht mit einem TOML-Zerleger.** Die Wurzel-`Cargo.toml`
/// besteht zum groesseren Teil aus Begruendungen: jede fremde Kiste traegt dort
/// ihren Grund, und ein Zerleger schriebe die Datei beim Ausgeben neu und
/// naehme jeden dieser Kommentare mit. Getauscht wird deshalb der Inhalt
/// zwischen zwei Anfuehrungszeichen und sonst kein Byte. `xtask` fuehrt aus
/// demselben Grund keine Abhaengigkeit.
///
/// Gefunden werden muss **genau eine** Zeile. Keine ist ein Abbruch, zwei
/// ebenso: bei zweien waere nicht entscheidbar, welche die Zahl fuehrt, die
/// `env!` spaeter liest.
fn versionsfeld_finden(inhalt: &str) -> Result<(Range<usize>, &str), String> {
    let mut versatz = 0usize;
    let mut im_abschnitt = false;
    let mut gefunden: Option<(Range<usize>, &str)> = None;
    for zeile in inhalt.split_inclusive('\n') {
        let rumpf = zeile.trim_end_matches(['\n', '\r']);
        let ohne_einzug = rumpf.trim_start();
        let einzug = rumpf.len() - ohne_einzug.len();
        if ohne_einzug.starts_with('[') {
            im_abschnitt = ohne_einzug.trim_end() == ABSCHNITT;
        } else if im_abschnitt
            && !ohne_einzug.starts_with('#')
            && let Some(spanne) = wertspanne(ohne_einzug)
        {
            let start = versatz + einzug + spanne.start;
            let ende = versatz + einzug + spanne.end;
            if gefunden.is_some() {
                return Err(format!(
                    "die Cargo.toml fuehrt unter {ABSCHNITT} zwei Zeilen `version = …`. \
                         Welche von beiden `env!(\"CARGO_PKG_VERSION\")` liest, ist damit nicht \
                         entschieden; es wird keine gesetzt."
                ));
            }
            gefunden = Some((start..ende, &inhalt[start..ende]));
        }
        versatz += zeile.len();
    }
    gefunden.ok_or_else(|| {
        format!(
            "die Cargo.toml fuehrt unter {ABSCHNITT} keine Zeile `version = \"…\"`. Genau dort \
             wohnt die Zahl, und ein zweiter Ort ist ausgeschlossen; es wird nichts gesetzt."
        )
    })
}

/// Die Spanne des Werts in einer Zeile `version = "…"`, bezogen auf die Zeile.
///
/// Steht getrennt, weil sie die einzige Stelle mit Byteversaetzen ist und ihre
/// Faelle sich einzeln nachsehen lassen.
fn wertspanne(zeile: &str) -> Option<Range<usize>> {
    let nach_namen = zeile.strip_prefix("version")?;
    // `versionsnummer = …` faengt genauso an und ist etwas anderes.
    if !nach_namen.starts_with(|zeichen: char| zeichen.is_whitespace() || zeichen == '=') {
        return None;
    }
    let nach_gleich = nach_namen.trim_start().strip_prefix('=')?;
    let nach_anfuehrung = nach_gleich.trim_start().strip_prefix('"')?;
    let laenge = nach_anfuehrung.find('"')?;
    let start = zeile.len() - nach_anfuehrung.len();
    Some(start..start + laenge)
}

/// Die Meldung des Eintrags.
///
/// Deutsch hinter dem englischen Typ, wie jeder Eintrag dieses Baums.
fn eintragsmeldung(neu: &str) -> String {
    format!("chore(release): die Version steht auf {neu}")
}

/// `git tag --list <name>`: steht dieser Tag irgendwo im Verzeichnis?
///
/// `--list` ist das, was den Aufruf zu einer Frage macht. Ohne die Marke legte
/// derselbe Aufruf den Tag an, und die Probe `die_tagliste_fragt_nur` sieht
/// deshalb nach ihr.
fn tagliste_argumente(tagname: &str) -> Vec<&str> {
    vec!["tag", "--list", tagname]
}

/// `git tag <name>`: der leichte Tag auf HEAD.
///
/// Leicht und nicht annotiert, wie `v0.1.0` vom 260813, den der Nutzer von
/// Hand gesetzt hat. Station 1 von `release` fragt `--points-at` und
/// unterscheidet die beiden Arten nicht; zwei Arten nebeneinander waeren
/// trotzdem zwei Schreibweisen fuer dieselbe Sache.
///
/// **Ohne `-f`.** Ein bestehender Tag laesst diesen Aufruf scheitern, und das
/// ist die Absicht: [`vorhaben_bestimmen`] hat den Fall vorher entschieden.
fn tag_argumente(tagname: &str) -> Vec<&str> {
    vec!["tag", tagname]
}

/// `git commit --only -- Cargo.toml Cargo.lock`: der Eintrag der einen
/// Aenderung.
///
/// **`--only` mit Pfaden und kein `git add`.** Der Eintrag entsteht aus dem
/// Stand dieser beiden Dateien im Arbeitsbaum, ohne die Vormerkung anzufassen.
/// Das hat zwei Wirkungen: ein gescheiterter Eintrag laesst nichts Vorgemerktes
/// zurueck, das jemand wegraeumen muesste, und der Lauf greift nicht auf die
/// gemeinsame Vormerkung zu, an der in diesem Projekt auch Agenten arbeiten.
fn eintrag_argumente(meldung: &str) -> Vec<&str> {
    vec![
        "commit",
        "--only",
        "-m",
        meldung,
        "--",
        EINGETRAGENE[0],
        EINGETRAGENE[1],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Der dritte Bauer der Argumentlisten steht dort, wo das Schieben
    // hingehoert; die Aufsicht ueber alle drei steht hier. Warum, sagt der
    // Pruefkommentar von `die_schreibenden_kommandos_tragen_keine_gewalt`.
    use crate::veroeffentlichung;

    /// Die Wurzel-Cargo.toml in klein: Abschnitte drumherum, Kommentare drin,
    /// und in den anderen Abschnitten Zeilen, die genauso anfangen.
    const MUSTER: &str = "\
[workspace]
resolver = \"3\"
members = [\"crates/krk-core\"]

# Gemeinsame Paketangaben.
[workspace.package]
version = \"0.1.0\"
edition = \"2024\"
rust-version = \"1.97.1\"

[workspace.dependencies]
serde = { version = \"1\", features = [\"derive\"] }
toml = \"1\"
";

    #[test]
    fn die_versionszeile_wird_unter_workspace_package_gefunden() {
        let (spanne, wert) = versionsfeld_finden(MUSTER).expect("das Muster fuehrt eine");
        assert_eq!(wert, "0.1.0");
        assert_eq!(&MUSTER[spanne], "0.1.0");
    }

    /// Getauscht wird der Wert und sonst kein Byte — Kommentare, Reihenfolge
    /// und die `rust-version` daneben bleiben stehen.
    #[test]
    fn das_setzen_ruehrt_nur_den_wert_an() {
        let (spanne, _) = versionsfeld_finden(MUSTER).expect("das Muster fuehrt eine");
        let gesetzt = format!(
            "{}{}{}",
            &MUSTER[..spanne.start],
            "0.2.0",
            &MUSTER[spanne.end..]
        );
        assert_eq!(
            gesetzt,
            MUSTER.replace("version = \"0.1.0\"", "version = \"0.2.0\"")
        );
        assert!(gesetzt.contains("rust-version = \"1.97.1\""));
        assert!(gesetzt.contains("# Gemeinsame Paketangaben."));
        assert!(gesetzt.contains("serde = { version = \"1\""));
    }

    #[test]
    fn eine_version_in_einem_anderen_abschnitt_zaehlt_nicht() {
        let ohne = "[workspace]\nversion = \"9.9.9\"\n\n[workspace.dependencies]\ntoml = \"1\"\n";
        assert!(versionsfeld_finden(ohne).is_err());
    }

    #[test]
    fn ohne_abschnitt_wird_nichts_gesetzt() {
        let meldung = versionsfeld_finden("[package]\nname = \"krk\"\n")
            .expect_err("ohne den Abschnitt gibt es keine Zahl");
        assert!(meldung.contains("[workspace.package]"), "{meldung}");
    }

    #[test]
    fn zwei_versionszeilen_sind_ein_abbruch() {
        let doppelt = "[workspace.package]\nversion = \"0.1.0\"\nversion = \"0.2.0\"\n";
        let meldung = versionsfeld_finden(doppelt).expect_err("zwei sind nicht entscheidbar");
        assert!(meldung.contains("zwei Zeilen"), "{meldung}");
    }

    #[test]
    fn ein_kommentar_mit_einer_versionszeile_zaehlt_nicht() {
        let mit = "[workspace.package]\n# version = \"9.9.9\"\nversion = \"0.1.0\"\n";
        let (_, wert) = versionsfeld_finden(mit).expect("die echte Zeile steht darunter");
        assert_eq!(wert, "0.1.0");
    }

    #[test]
    fn ein_feld_das_nur_so_anfaengt_zaehlt_nicht() {
        assert!(wertspanne("versionsnummer = \"9\"").is_none());
        assert!(wertspanne("rust-version = \"1.97.1\"").is_none());
        assert!(wertspanne("version = \"0.1.0\"").is_some());
        assert!(wertspanne("version=\"0.1.0\"").is_some());
    }

    #[test]
    fn drei_zahlen_gehen_durch() {
        assert!(versionszahl_pruefen("0.1.0").is_ok());
        assert!(versionszahl_pruefen("1.2.3").is_ok());
        assert!(versionszahl_pruefen("10.0.12").is_ok());
    }

    #[test]
    fn ein_fuehrendes_v_wird_benannt_und_nicht_stillschweigend_abgestreift() {
        let meldung = versionszahl_pruefen("v0.2.0").expect_err("das v traegt der Tag");
        assert!(meldung.contains("ohne `v`"), "{meldung}");
    }

    #[test]
    fn zwei_oder_vier_teile_sind_keine_versionszahl() {
        assert!(versionszahl_pruefen("0.2").is_err());
        assert!(versionszahl_pruefen("0.2.0.1").is_err());
        assert!(versionszahl_pruefen("").is_err());
    }

    #[test]
    fn ein_anhang_wie_rc1_ist_ausgeschlossen() {
        assert!(versionszahl_pruefen("0.2.0-rc1").is_err());
    }

    #[test]
    fn eine_fuehrende_null_ist_zwei_schreibweisen_fuer_eine_zahl() {
        assert!(versionszahl_pruefen("0.02.0").is_err());
        assert!(versionszahl_pruefen("0.0.0").is_ok());
    }

    #[test]
    fn ohne_tag_wird_gesetzt_und_getaggt() {
        assert_eq!(
            vorhaben_bestimmen("0.1.0", "0.2.0", "v0.2.0", false, false),
            Ok(Vorhaben::SetzenEintragenTaggen)
        );
    }

    #[test]
    fn steht_die_zahl_schon_wird_nur_getaggt() {
        assert_eq!(
            vorhaben_bestimmen("0.2.0", "0.2.0", "v0.2.0", false, false),
            Ok(Vorhaben::NurTaggen)
        );
    }

    /// Der zweite Lauf desselben Kommandos tut nichts und scheitert nicht.
    /// Genau das macht `./release.sh 0.2.0` nach einem Abbruch der sieben
    /// Stationen wiederholbar.
    #[test]
    fn zahl_und_tag_stehen_schon_dann_ist_nichts_zu_tun() {
        assert_eq!(
            vorhaben_bestimmen("0.2.0", "0.2.0", "v0.2.0", true, true),
            Ok(Vorhaben::NichtsZuTun)
        );
    }

    #[test]
    fn ein_vergebener_tag_anderswo_haelt_den_lauf_an() {
        let meldung = vorhaben_bestimmen("0.1.0", "0.2.0", "v0.2.0", true, false)
            .expect_err("der Name ist vergeben");
        assert!(meldung.contains("nicht auf HEAD"), "{meldung}");
        assert!(meldung.contains("git show v0.2.0"), "{meldung}");
        assert!(
            meldung.contains("Es ist nichts geschrieben worden."),
            "{meldung}"
        );
    }

    /// Der Fall, der ohne Pruefung den Tag um einen Commit zuruecklassen
    /// wuerde.
    #[test]
    fn ein_tag_auf_head_bei_alter_zahl_haelt_den_lauf_an() {
        let meldung = vorhaben_bestimmen("0.1.0", "0.2.0", "v0.2.0", true, true)
            .expect_err("der Eintrag schoebe HEAD unter dem Tag weg");
        assert!(meldung.contains("fuehrt die Version 0.1.0"), "{meldung}");
        assert!(
            meldung.contains("Es ist nichts geschrieben worden."),
            "{meldung}"
        );
    }

    /// Kein Weg vorbei an einem vergebenen Namen: keine der drei Meldungen
    /// nennt eine Marke, die den Tag verschoebe.
    #[test]
    fn keine_meldung_bietet_gewalt_an() {
        for lage in [(true, false), (true, true)] {
            let meldung = vorhaben_bestimmen("0.1.0", "0.2.0", "v0.2.0", lage.0, lage.1)
                .expect_err("beide Lagen halten an");
            assert!(!meldung.contains("--force"), "{meldung}");
            assert!(!meldung.contains("-f "), "{meldung}");
        }
    }

    #[test]
    fn die_meldung_zum_arbeitsbaum_nennt_jede_datei_beim_namen() {
        let geaendert = vec![
            " M fusion-workbench/monitor",
            " M fusion-workbench/.fusion-setup",
            " M fusion-workbench/.guard-state/churn.json",
            " M fusion-workbench/orchestrator-live.md",
        ];
        let meldung = arbeitsbaum_meldung(&geaendert, "0.2.0");
        for datei in &geaendert {
            assert!(meldung.contains(datei.trim()), "{meldung}");
        }
        assert!(meldung.contains("4 verfolgte Dateien sind"), "{meldung}");
        assert!(meldung.contains("260813-1515"), "{meldung}");
        assert!(
            meldung.contains("Es ist nichts geschrieben worden."),
            "{meldung}"
        );
    }

    #[test]
    fn eine_einzelne_datei_wird_im_singular_gezaehlt() {
        let meldung = arbeitsbaum_meldung(&[" M Cargo.toml"], "0.2.0");
        assert!(meldung.contains("1 verfolgte Datei ist"), "{meldung}");
    }

    /// Die Frage nach einem Tag legt keinen an, und daran haengt die Marke.
    #[test]
    fn die_tagliste_fragt_nur() {
        let argumente = tagliste_argumente("v0.2.0");
        assert_eq!(argumente, vec!["tag", "--list", "v0.2.0"]);
        assert!(argumente.contains(&"--list"), "{argumente:?}");
    }

    /// Die drei schreibenden Kommandos, Wort fuer Wort. Sie koennen keine
    /// Konstanten sein, weil jedes einen Wert aus der Befehlszeile traegt;
    /// nachgesehen werden sie trotzdem.
    ///
    /// **Seit dem 260821 deckt diese Aufsicht drei Kommandos und nicht mehr
    /// zwei.** Das dritte ist das Schieben, und sein Bauer steht nicht hier,
    /// sondern in `veroeffentlichung`, wo das Schieben hingehoert. Die Aufsicht
    /// ist trotzdem nicht mitgewandert und auch nicht verdoppelt worden: sie
    /// liest die Listen, die bei `git::rufen` landen, und es gibt einen solchen
    /// Aufruf im ganzen Baum. Eine zweite Aufsicht daneben waere eine zweite
    /// Antwort darauf, was ein schreibendes Kommando dieses Werkzeugs tragen
    /// darf.
    ///
    /// **Wie die Zusage seither lautet.** Bis dahin sagte sie, dass keines der
    /// Kommandos eine Marke aus einer Liste traegt, und `push` stand in jener
    /// Liste neben `--force`. Das geht nicht weiter, sobald ein Kommando `push`
    /// **ist**. Sie steht deshalb in zwei Haelften, und die Teilung ist
    /// trennscharf und ohne Ausnahmeliste:
    ///
    /// - **Das erste Wort ist der Unterbefehl und wird auf Gleichheit
    ///   geprueft**, je Kommando einzeln: `tag`, `commit`, `push`. Damit ist
    ///   gesagt, dass jedes genau eine Sache tut — und `push` ist an der einen
    ///   Stelle erlaubt und an den zwei anderen ausgeschlossen, ohne dass
    ///   irgendwo eine Ausnahme stuende. `add` faellt aus der Markenliste
    ///   heraus, weil diese Haelfte es abdeckt.
    /// - **Die Woerter danach tragen keine Marke, die Reichweite oder Gewalt
    ///   hinzufuegt.** Die sechs des Schiebens und die drei, die schon
    ///   dastanden.
    #[test]
    fn die_schreibenden_kommandos_tragen_keine_gewalt() {
        let tag = tag_argumente("v0.2.0");
        assert_eq!(tag, vec!["tag", "v0.2.0"]);

        let eintrag = eintrag_argumente("chore(release): die Version steht auf 0.2.0");
        assert!(
            eintrag.starts_with(&["commit", "--only", "-m"]),
            "{eintrag:?}"
        );
        assert!(
            eintrag.ends_with(&["--", "Cargo.toml", "Cargo.lock"]),
            "{eintrag:?}"
        );

        let verweis = veroeffentlichung::tagverweis("v0.2.0");
        let schub = veroeffentlichung::schiebe_argumente(&verweis);
        assert_eq!(schub, vec!["push", "origin", "HEAD", "refs/tags/v0.2.0"]);

        // Erste Haelfte: das erste Wort, auf Gleichheit.
        for (kommando, unterbefehl) in [(&tag, "tag"), (&eintrag, "commit"), (&schub, "push")] {
            assert_eq!(kommando.first(), Some(&unterbefehl), "{kommando:?}");
        }

        // Zweite Haelfte: keine Marke, die Reichweite oder Gewalt hinzufuegt.
        const MARKEN: [&str; 9] = [
            "--force",
            "-f",
            "--tags",
            "--all",
            "--mirror",
            "--delete",
            "--amend",
            "--no-verify",
            "-a",
        ];
        for kommando in [&tag, &eintrag, &schub] {
            for marke in MARKEN {
                assert!(
                    !kommando[1..].contains(&marke),
                    "{kommando:?} traegt {marke}"
                );
            }
        }
    }

    #[test]
    fn die_eintragsmeldung_nennt_typ_und_zahl() {
        let meldung = eintragsmeldung("0.2.0");
        assert!(meldung.starts_with("chore(release): "), "{meldung}");
        assert!(meldung.contains("0.2.0"), "{meldung}");
    }

    /// Die Zahl wird an genau einer Stelle gesetzt: die `Cargo.lock` daneben
    /// ist abgeleitet, und `cargo` schreibt sie.
    #[test]
    fn eingetragen_werden_die_manifestdatei_und_ihre_ableitung() {
        assert_eq!(EINGETRAGENE, ["Cargo.toml", "Cargo.lock"]);
    }

    #[test]
    fn version_nimmt_genau_ein_argument() {
        assert!(matches!(ausfuehren(&[]), Err(Abbruch::Aufruf(_))));
        assert!(matches!(
            ausfuehren(&["0.2.0".to_owned(), "0.3.0".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
        // Eine falsche Zahl ist ebenfalls ein Aufruffehler und kein Lauffehler:
        // sie steht in der Befehlszeile.
        assert!(matches!(
            ausfuehren(&["0.2".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
    }
}
