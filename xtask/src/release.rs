//! Das Auslieferungspaket: `cargo xtask release` (Schritt 23).
//!
//! Der Weg in acht Stationen, jede scheitert mit einer benennenden Meldung.
//! Dazwischen stehen drei Vorlaeufe: sie kosten nichts, laufen deshalb frueh,
//! und tragen einen Buchstaben statt einer Zahl, weil ihr Ergebnis erst einer
//! spaeteren Station dient. Die Reihenfolge unten ist die des Quelltextes in
//! [`ausfuehren`].
//!
//! 1. **Tag, Arbeitsbaum und `gh` pruefen:** HEAD traegt einen Tag
//!    `v<version>` mit der Zahl aus `[workspace.package]`, keine verfolgte
//!    Datei ist geaendert, und das GitHub-Kommandozeilenwerkzeug ist vorhanden
//!    und angemeldet. Die billigste Station des Weges und die, die am
//!    haeufigsten anschlaegt; sie steht ganz vorn, damit ein Abbruch dieser Art
//!    keinen Uebersetzungslauf kostet. Was sie fragt, steht bei
//!    [`auslieferungsstand_pruefen`], der Vergleich selbst bei
//!    [`stand_pruefen`].
//!
//!    **`gh` steht seit dem 260821 hier und nicht mehr allein am Kopf der
//!    achten Station.** Die Zusage des Specs lautet, dass eine fehlende
//!    Voraussetzung auffallen soll, solange noch nichts geschehen ist; am Kopf
//!    der achten Station war zu diesem Zeitpunkt bereits eine Einreichung bei
//!    Apple abgeschlossen (Durchsicht 260821-1346, B4). Das Vorziehen kostet
//!    nichts: die Frage geht an das Werkzeug und nicht an den Baum, `bundle`
//!    bekommt keine neue Vorbedingung, und `make check` keine Abhaengigkeit
//!    von `gh`. Die achte Station behaelt ihre eigene Pruefung, denn sie hat
//!    einen zweiten Rufer, vor dem keine Station steht.
//!
//!    **Sie liest, und sie liest jetzt gegen etwas Geschriebenes.** Den Tag
//!    setzt seit dem 260813 `cargo xtask version <zahl>`, der Halbschritt vor
//!    diesem hier; er setzt auch die Zahl und traegt sie ein. Dass die Station
//!    trotzdem bleibt, ist der Kern der Sache: sie laeuft im **neu
//!    uebersetzten** Werkzeug und vergleicht die eingebackene Zahl aus
//!    `env!("CARGO_PKG_VERSION")` mit dem Tag. Bliebe ein altes Werkzeug
//!    stehen, truege die `Info.plist` die alte Zahl, waehrend der Tag die neue
//!    nennte — und genau das faellt hier auf. Die Einzelheiten stehen im
//!    Modulkopf von `version`.
//! - *Vorlauf a:* `bundle::vorbereiten` liest die Buendelbeschreibung und
//!   liefert die Vorlage fuer Station 5.
//! 2. **AppKit-Grenze pruefen:** keine Nennung einer `objc2`-Kiste ausserhalb
//!    von `crates/krk-ui/src/appkit/`, weder als `use`-Zeile noch als
//!    ausgeschriebener Pfad, und das in jeder `.rs`-Datei unter `crates/`.
//!    Die Pruefung traegt die Grenzzusage aus dem Plan maschinell, weil
//!    `#![deny(unsafe_code)]` sie nur zur Haelfte erzwingt: ein grosser Teil
//!    der `objc2`-Bindungen ist als sicher deklariert und uebersetzt
//!    ausserhalb anstandslos. Wo sie endet und warum sie dort endet, steht bei
//!    `GRENZWURZEL` und bei `verletzt_grenze`. Defekte
//!    `260803-1530_*_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen`,
//!    `260806-1333_*_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten`
//!    und
//!    `260807-0800_*_die-appkit-grenzpruefung-kennt-nur-src-baeume-und-nur-die-woertliche-schreibweise`.
//! - *Vorlauf b:* die Identitaetssuche aus `sign` liefert die Identitaet fuer
//!   Station 6.
//! - *Vorlauf c:* die Zielpruefung ueber `rustup` ist die Voraussetzung von
//!   Station 3.
//! 3. **Beide Ziele uebersetzen:** dieselbe Uebersetzung wie `bundle`, einmal
//!    je Tripel aus `rust-toolchain.toml`.
//! 4. **`lipo`:** die beiden Binaerdateien zu einer universellen
//!    zusammenfuegen; `lipo -archs` muss danach beide Architekturen melden.
//! 5. **Montage:** dasselbe Buendel wie `bundle`, ueber `bundle::Vorlage` —
//!    ein zweiter Buendelbauer waere die zweite Wahrheit ueber die Struktur
//!    von `KRK.app`.
//! 6. **Signieren:** die Identitaetssuche aus `sign` mit Developer-ID statt
//!    Entwicklungsidentitaet, `codesign` mit `--options runtime`.
//! 7. **Beglaubigen:** `xcrun notarytool submit --wait` und
//!    `xcrun stapler staple`. Beides verlangt das vollstaendige Xcode, die
//!    Beglaubigung zusaetzlich ein Apple-Entwicklerkonto; fehlt eines von
//!    beidem, bricht allein diese Station ab, und das gebaute, signierte
//!    Buendel bleibt liegen. Der Plan nimmt den Schritt auch in diesem Fall
//!    ab, deshalb werden die Voraussetzungen der Beglaubigung erst hier
//!    geprueft und nicht, wie sonst ueblich, vor dem ersten
//!    Uebersetzungslauf.
//!
//!    **Sie steht seit dem 260820 in `beglaubigung.rs` und nicht mehr hier.**
//!    Sie hat einen zweiten Rufer bekommen, `cargo xtask beglaubigen`, der
//!    genau diese Station allein faehrt, wenn ein Lauf erst an ihr gescheitert
//!    ist und das fertige Buendel schon dasteht. Was dieser zweite Weg
//!    ausdruecklich nicht prueft, ist Station 1: der Modulkopf von
//!    `beglaubigung` schreibt aus, warum das sein Zweck und nicht sein Mangel
//!    ist.
//! 8. **Veroeffentlichen:** aus dem beglaubigten Buendel wird
//!    `target/KRK-<version>.zip`, HEAD und `refs/tags/v<version>` gehen zur
//!    Gegenseite, und an einer oeffentlichen Releaseseite haengt danach das
//!    Zip. Was sie vorher prueft, ist `gh`: vorhanden und angemeldet. Das ist
//!    die dritte aeussere Voraussetzung der Kette, neben dem vollstaendigen
//!    Xcode und dem Apple-Entwicklerkonto von Station 7. Auf diesem Weg hat
//!    Station 1 sie schon erfragt; die Station fragt trotzdem noch einmal,
//!    weil ihr zweiter Rufer keine Station vor sich hat. Fehlt sie, bricht
//!    allein diese Station ab, und das beglaubigte Buendel bleibt liegen.
//!
//!    **Was sie im Erfolgsfall schreibt, steht ausserhalb dieses Geraets:**
//!    HEAD und `refs/tags/v<version>` gehen zu `origin`, und die Releaseseite
//!    ist oeffentlich. Das ist die einzige Wirkung der ganzen Kette, die sich
//!    nicht zuruecknehmen laesst.
//!
//!    **Sie steht in `veroeffentlichung.rs` und hat wie Station 7 zwei
//!    Rufer**, diesen hier und `cargo xtask veroeffentlichen <zahl>`. Der
//!    Unterschied zwischen beiden ist eine einzige Frage: der eigenstaendige
//!    Weg prueft selbst, ob `v<zahl>` auf HEAD steht, weil vor ihm keine
//!    Station stand; von hier aus hat Station 1 dieselbe Wahrheit schon gegen
//!    die eingebackene Zahl geprueft. Ausgedrueckt ist der Unterschied als
//!    `veroeffentlichung::Tagfrage` und nicht als Wahrheitswert.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Abbruch;
use crate::beglaubigung;
use crate::bundle;
use crate::git;
use crate::sign;
use crate::veroeffentlichung;

/// Die beiden Ziel-Tripel der universellen Binaerdatei.
///
/// Dieselben zwei wie in `rust-toolchain.toml`; `rustup` haelt sie darueber
/// installiert.
const ZIELE: [&str; 2] = ["x86_64-apple-darwin", "aarch64-apple-darwin"];

/// Die Architekturnamen, die `lipo -archs` danach melden muss.
///
/// **In der Reihenfolge von [`ZIELE`]:** [`lipo_name`] liest die beiden
/// Aufzaehlungen paarweise, und wer eine davon umsortiert, sortiert die andere
/// mit. Die Probe `die_beiden_ziele_tragen_die_namen_die_lipo_dafuer_meldet`
/// faengt es.
const ARCHITEKTUREN: [&str; 2] = ["x86_64", "arm64"];

/// Paare haben gleich viele Glieder; sonst waeren es keine.
const _: () = assert!(ZIELE.len() == ARCHITEKTUREN.len());

/// Der Name, unter dem `lipo` die Architektur `architektur` meldet.
///
/// Rust und `lipo` benennen dieselbe Architektur verschieden: `rustc` zielt auf
/// `aarch64-apple-darwin`, und `std::env::consts::ARCH` sagt entsprechend
/// `aarch64`, waehrend `lipo -info` und `lipo -archs` fuer dasselbe Programm
/// `arm64` schreiben. Wer eine Architektur nennt, damit der Leser die
/// Weitergabefaehigkeit mit `lipo` nachprueft, muss den Namen von `lipo`
/// nennen; sonst sucht der Leser in dessen Ausgabe ein Wort, das dort nie
/// steht. Am 260812 ist genau das geschehen
/// (`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`).
///
/// **Eine zweite Namensliste entsteht dabei nicht.** Beide Namen stehen schon
/// in dieser Datei und tragen dort je eine eigene Aufgabe: der Rust-Name als
/// Praefix des Ziel-Tripels in [`ZIELE`], der Name von `lipo` in
/// [`ARCHITEKTUREN`], wo er die Pruefbedingung des Zusammenfuegens ist. Die
/// Umrechnung liest die beiden paarweise und schreibt keinen Namen selbst aus.
///
/// **Einen unbekannten Namen reicht sie durch.** KRK zielt heute auf zwei
/// Architekturen; bekaeme diese Funktion eine dritte, waere eine geratene
/// Uebersetzung falsch und ein Weglassen ein Verschweigen. Ein durchgereichtes
/// `aarch64` ist eine schlechtere Auskunft als `arm64`, ein erfundener Name
/// waere eine unwahre.
#[must_use]
pub fn lipo_name(architektur: &str) -> &str {
    for (ziel, gemeldet) in ZIELE.into_iter().zip(ARCHITEKTUREN) {
        if ziel
            .strip_prefix(architektur)
            .is_some_and(|rest| rest.starts_with('-'))
        {
            return gemeldet;
        }
    }
    architektur
}

/// Die Wurzel der AppKit-Grenzpruefung: das Verzeichnis der Anwendungskisten.
///
/// **Warum das ganze Verzeichnis und nicht die drei `src`-Baeume.** Bis zum
/// 260807 stand hier eine Liste aus `crates/krk-ui/src`, `crates/krk-core/src`
/// und `crates/krk-bench/src`. Cargo uebersetzt je Kiste aber ausser `src/`
/// auch `tests/`, `benches/`, `examples/` und `build.rs`, und `krk-ui` fuehrt
/// fuenf `objc2`-Abhaengigkeiten: ein `crates/krk-ui/tests/…rs` mit einem
/// AppKit-Aufruf waere gruen durchgegangen, waehrend das Werkzeug meldete, es
/// gebe keinen
/// (`issues/260807-0800_*_die-appkit-grenzpruefung-kennt-nur-src-baeume-und-nur-die-woertliche-schreibweise.md`).
/// Diese Baeume hier aufzuzaehlen hiesse, Cargos Verzeichnisregeln ein zweites
/// Mal zu schreiben und mit der naechsten Fassung auseinanderlaufen zu lassen.
/// Ueber das Kistenverzeichnis zu gehen umfasst sie alle, ohne eine davon zu
/// kennen, und nimmt zugleich eine vierte Kiste mit, die noch niemand angelegt
/// hat — die Liste zu ergaenzen war die zweite Art, die Pruefung im
/// Vorbeigehen zu verlieren.
///
/// **Warum `xtask` nicht dazugehoert.** Die Grenze ist eine Zusage ueber die
/// Anwendung, und `xtask` uebersetzt nicht in `KRK.app` hinein: es baut das
/// Buendel, es sitzt nicht darin. Dazu nennt genau diese Datei `objc2`
/// zwangslaeufig, weil sie die Pruefung *ist* — ihre Proben schreiben die
/// gesuchten Zeilen woertlich aus. Ein Tor, das auf sich selbst anschlaegt,
/// waere kein Tor.
const GRENZWURZEL: &str = "crates";

/// Der eine Teilbaum unter [`GRENZWURZEL`], der eine `objc2`-Kiste nennen darf.
const AUSNAHME: &str = "crates/krk-ui/src/appkit";

/// Baut, signiert, beglaubigt und veroeffentlicht das Auslieferungspaket.
pub fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    if let Some(ueberzaehlig) = argumente.first() {
        return Err(Abbruch::Aufruf(format!(
            "release kennt {ueberzaehlig:?} nicht"
        )));
    }

    auslieferungsstand_pruefen(&bundle::wurzel())?;

    // Die aeussere Voraussetzung der achten Station, hier vorn erfragt. Sie
    // kostet nichts und laesst den Baum, wie er ist; stuende sie allein am Kopf
    // der achten, faende ein Lauf ohne `gh` das erst hinter einer
    // abgeschlossenen Einreichung bei Apple heraus. Die achte Station fragt
    // trotzdem noch einmal — sie hat einen zweiten Rufer, vor dem keine Station
    // steht.
    veroeffentlichung::gh_pruefen()?;

    let vorlage = bundle::vorbereiten()?;
    appkit_grenze_pruefen(&vorlage.wurzel)?;
    let identitaet = sign::bestimmen_fuer_release()?;
    if !identitaet.name.starts_with(sign::DEVELOPER_ID_PRAEFIX) {
        println!(
            "Hinweis: {:?} ist keine Developer-ID-Identitaet. Signiert wird trotzdem; die \
             Beglaubigung nimmt ein so signiertes Buendel nicht an.",
            identitaet.name
        );
    }
    ziele_pruefen()?;

    for ziel in ZIELE {
        bundle::uebersetzen(&vorlage.wurzel, &vorlage.binaername, Some(ziel))?;
    }
    let universell = zusammenfuegen(&vorlage)?;

    let buendel = vorlage.zusammensetzen(&universell)?;
    sign::signieren_gehaertet(&buendel, &identitaet)?;
    println!(
        "Universell gebaut und mit gehaerteter Laufzeitumgebung signiert: {}",
        buendel.display()
    );

    beglaubigung::beglaubigen(&buendel)?;
    println!("Beglaubigt und angeheftet: {}", buendel.display());

    // Station 8 nimmt die Zahl aus `env!("CARGO_PKG_VERSION")`, denn `release`
    // nimmt kein Argument; es ist dieselbe Zahl, gegen die Station 1 den Tag
    // gehalten hat. Deshalb fragt die Station hier nicht noch einmal danach.
    veroeffentlichung::veroeffentlichen(
        env!("CARGO_PKG_VERSION"),
        veroeffentlichung::Tagfrage::Erledigt,
    )
}

/// Station 1: HEAD traegt den passenden Tag, und der Arbeitsbaum entspricht
/// ihm.
///
/// Sie stellt `git` die drei Fragen [`git::Auftrag::Verzeichnis`],
/// [`git::Auftrag::TagsAufHead`] und [`git::Auftrag::Stand`] und reicht die
/// beiden Antworten an [`stand_pruefen`]
/// weiter. **Alle drei lesen.** Aus diesem Modul entsteht kein `git tag`, kein
/// `git commit` und kein Schreibzugriff; wer schreibt, ist `version`, und das
/// laeuft vor diesem Kommando und in einem eigenen Prozess.
///
/// Die Sollversion holt sie aus [`bundle::VERSION`], also aus derselben
/// Konstanten, die auch in die `Info.plist` wandert. Eine zweite Quelle der
/// Versionszahl entsteht nicht, und ein Zerteiler fuer die `Cargo.toml` auch
/// nicht.
fn auslieferungsstand_pruefen(wurzel: &Path) -> Result<(), Abbruch> {
    git::rufen(wurzel, &git::Auftrag::Verzeichnis).map_err(|fehler| {
        let grund = match fehler {
            Abbruch::Lauf(text) | Abbruch::Aufruf(text) => text,
        };
        Abbruch::Lauf(format!(
            "Die Auslieferung braucht ein Git-Verzeichnis, und in {} ist keines zu befragen: \
             {grund}\n\
             \n\
             Ausgeliefert wird ein eingetragener Stand, den ein Tag v{} benennt; ohne \
             Git-Verzeichnis steht weder das eine noch das andere fest. Es wird nicht \
             ersatzweise durchgebaut, und es entsteht kein Auslieferungspaket.",
            wurzel.display(),
            bundle::VERSION
        ))
    })?;

    let tags = git::rufen(wurzel, &git::Auftrag::TagsAufHead)?;
    let geaenderte = git::rufen(wurzel, &git::Auftrag::Stand)?;
    stand_pruefen(bundle::VERSION, &tags, &geaenderte).map_err(Abbruch::Lauf)?;

    println!(
        "Auslieferungsstand geprueft: HEAD traegt den Tag v{}, und keine verfolgte Datei ist \
         geaendert.",
        bundle::VERSION
    );
    Ok(())
}

/// Vergleicht Version, Tags und Arbeitsbaum; die reine Haelfte von Station 1.
///
/// Drei Zeichenketten hinein, `Ok(())` im gruenen Fall, sonst die fertige
/// Abbruchmeldung. Kein Prozessaufruf, kein Dateizugriff, kein
/// Git-Verzeichnis: der gruene Fall wird an dieser Funktion abgenommen und
/// nicht an einem Lauf, der einen gesetzten Tag voraussetzt.
///
/// `tags_auf_head` ist die Ausgabe von `git tag --points-at HEAD`, ein Name je
/// Zeile; einer davon muss passen, und mehrere stoeren nicht. `geaenderte` ist
/// die Ausgabe von `git status --porcelain --untracked-files=no`, eine Datei
/// je Zeile.
///
/// **Zum `#[must_use]` dieses Vorhabens:** der Rueckgabetyp ist `Result`, und
/// `Result` traegt das Attribut schon in der Standardbibliothek. Ein stilles
/// Fallenlassen haelt den Bau unter `-D warnings` an, die Zusage steht also
/// strukturell; ein zweites Attribut daneben waere Rauschen.
fn stand_pruefen(version: &str, tags_auf_head: &str, geaenderte: &str) -> Result<(), String> {
    let erwartet = format!("v{version}");
    let tag_steht = git::tag_steht(tags_auf_head, &erwartet);
    let abweichungen = git::geaenderte_dateien(geaenderte);

    if tag_steht && abweichungen.is_empty() {
        return Ok(());
    }

    let mut befunde = Vec::new();
    if !tag_steht {
        befunde.push(format!(
            "Auf HEAD steht kein Tag {erwartet}. Die Cargo.toml fuehrt die Version {version}, \
             und eine Auslieferung traegt einen Tag, der genau diese Zahl benennt; sonst nennt \
             die Zahl im Buendel keinen Stand, den jemand wiederfindet. Zahl, Eintrag und Tag \
             setzt der Halbschritt davor, und der ganze Weg steht in einem Kommando:\n\
             \x20      ./release.sh {version}\n\
             \x20      cargo xtask version {version}   (nur der Halbschritt)"
        ));
    }
    if !abweichungen.is_empty() {
        let aufzaehlung: Vec<String> = abweichungen
            .iter()
            .map(|zeile| format!("\x20      {zeile}"))
            .collect();
        let zahlwort = if abweichungen.len() == 1 {
            "1 verfolgte Datei ist".to_owned()
        } else {
            format!("{} verfolgte Dateien sind", abweichungen.len())
        };
        befunde.push(format!(
            "Der Arbeitsbaum weicht vom eingetragenen Stand ab; {zahlwort} geaendert:\n\
             \n\
             {}\n\
             \n\
             Ein Buendel aus diesem Baum traegt die Version {version} und ist nicht aus dem \
             Stand gebaut, den {erwartet} benennt. Abhilfe: die Aenderungen eintragen oder \
             wegstellen:\n\
             \x20      git commit -a\n\
             \x20      git stash",
            aufzaehlung.join("\n")
        ));
    }

    Err(format!(
        "Der Auslieferungsstand ist nicht gedeckt:\n\
         \n\
         {}\n\
         \n\
         Unbeachtete Dateien zaehlen nicht mit, und `cargo xtask bundle` baut weiterhin \
         jederzeit ohne Tag. Es entsteht kein Auslieferungspaket.",
        befunde.join("\n\n")
    ))
}

/// Prueft, dass ausserhalb von `crates/krk-ui/src/appkit/` keine `objc2`-Kiste
/// genannt wird.
///
/// Dieselbe Vorschrift wie im Abnahmekriterium von Schritt 23, und sie besteht
/// aus zwei Suchen ueber jede `.rs`-Datei unter [`GRENZWURZEL`] ausserhalb von
/// [`AUSNAHME`]: die `use`-Zeile aus `ist_objc2_use` und der ausgeschriebene
/// Pfad aus `nennt_objc2_pfad`. Eine `objc2`-Bindung kommt ohne eines von
/// beidem nicht zustande, gleich ob die Kiste sie als `pub fn` oder als
/// `pub unsafe fn` fuehrt; zusammen fangen die zwei Suchen beide Haelften der
/// Grenze.
fn appkit_grenze_pruefen(wurzel: &Path) -> Result<(), Abbruch> {
    let mut verstoesse = Vec::new();
    dateien_pruefen(
        &wurzel.join(GRENZWURZEL),
        &wurzel.join(AUSNAHME),
        &mut verstoesse,
    )?;
    if !verstoesse.is_empty() {
        verstoesse.sort();
        let aufzaehlung: Vec<String> = verstoesse
            .iter()
            .map(|pfad| format!("\x20      {}", pfad.display()))
            .collect();
        return Err(Abbruch::Lauf(format!(
            "Die AppKit-Grenze ist verletzt: eine `objc2`-Kiste ist ausserhalb von \
             crates/krk-ui/src/appkit/ genannt, als `use`-Zeile oder als ausgeschriebener \
             Pfad, in\n\
             \n\
             {}\n\
             \n\
             Jeder AppKit-Aufruf liegt hinter einer sicheren Huelle unter \
             crates/krk-ui/src/appkit/; der Aufruf gehoert dorthin verschoben. Es entsteht \
             kein Auslieferungspaket.",
            aufzaehlung.join("\n")
        )));
    }
    println!(
        "AppKit-Grenze geprueft: keine `objc2`-Kiste ausserhalb von \
         crates/krk-ui/src/appkit/, weder als `use`-Zeile noch als ausgeschriebener Pfad."
    );
    Ok(())
}

/// Geht die `.rs`-Dateien unter `ordner` durch und sammelt die Verstoesse.
///
/// Der Teilbaum `ausgenommen` wird nicht betreten: dort, und nur dort, ist
/// eine `objc2`-Kiste erlaubt.
fn dateien_pruefen(
    ordner: &Path,
    ausgenommen: &Path,
    verstoesse: &mut Vec<PathBuf>,
) -> Result<(), Abbruch> {
    let eintraege = fs::read_dir(ordner).map_err(|fehler| {
        Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", ordner.display()))
    })?;
    for eintrag in eintraege {
        let eintrag = eintrag.map_err(|fehler| {
            Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", ordner.display()))
        })?;
        let pfad = eintrag.path();
        if pfad == ausgenommen {
            continue;
        }
        if pfad.is_dir() {
            dateien_pruefen(&pfad, ausgenommen, verstoesse)?;
            continue;
        }
        if pfad.extension().is_none_or(|endung| endung != "rs") {
            continue;
        }
        let inhalt = fs::read_to_string(&pfad).map_err(|fehler| {
            Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", pfad.display()))
        })?;
        if inhalt.lines().any(verletzt_grenze) {
            verstoesse.push(pfad);
        }
    }
    Ok(())
}

/// Ob eine Zeile die AppKit-Grenze verletzt.
///
/// Zwei Formen nennen eine `objc2`-Kiste: die `use`-Zeile und der
/// ausgeschriebene Pfad. Die zweite kam bis zum 260806 durch
/// (`issues/260806-1333_*_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md`);
/// `objc2::rc::Weak::from_retained(&x)` ist gueltiges Rust ohne jede
/// `use`-Zeile und steht heute mehrfach in `appkit/anwendung.rs`.
///
/// **Was als Kommentar gilt, und warum die Regel so grob ist.** Eine Zeile,
/// deren erstes nicht-leeres Zeichen ein `/` ist, wird nicht gelesen. Das ist
/// die ganze Kommentarbehandlung — kein Zustandsautomat fuer `//` und
/// `/* */`, wie der Defekt ihn erwogen hat. Drei Gruende. Erstens treffen die
/// dreizehn Kommentarzeilen des Baums, die `objc2` nennen und auf denen die
/// Pruefung nicht anschlagen darf, allesamt diese Form: sie stehen als `//!`
/// oder `//` in Spalte 1 beziehungsweise nach der Einrueckung. Zweitens gibt
/// es im ganzen Verzeichnis `crates/` keinen einzigen Blockkommentar, gemessen
/// am 260807; ein Automat dafuer waere Code gegen einen Fall, den es nicht
/// gibt, und die Maxime des Vorhabens ist "supersimpel". Drittens faellt die
/// verbleibende Luecke — ein nachgestellter Kommentar hinter Code, der
/// `objc2::` nennt — zur sicheren Seite: sie meldet einen Verstoss zu viel,
/// nicht einen zu wenig, und ein Umformulieren des Kommentars raeumt sie aus.
/// Ein halber Rust-Zerteiler in einem Bauwerkzeug koennte umgekehrt scheitern,
/// und dann schweigt das Tor.
///
/// # Wo die Pruefung endet, und warum sie dort endet
///
/// Sie liest Rust-Quelltext und sucht darin die Zeichenfolge `objc2`. Wer die
/// Kiste unter einem anderen Namen einbindet, kommt deshalb durch, und das ist
/// eine Festlegung, kein Versehen. Zwei Formen tun das:
///
/// - Ein Umbenennen in der `Cargo.toml`:
///   `appkit = { package = "objc2-app-kit", … }`. Danach ist
///   `use appkit::NSView;` gueltiges Rust in jeder Datei von `krk-ui`.
/// - `extern crate objc2 as ak;` innerhalb einer Datei — die Altform aus
///   Edition 2015, die in Edition 2024 nichts mehr leistet, was `use` nicht
///   auch leistet. `use objc2_app_kit as ak;` faellt bereits durch
///   `ist_objc2_use`.
///
/// Beide Formen schlaegt die Pruefung nicht, weil sie kein Vertippen sind und
/// kein Abdriften. Sie fangen soll sie den AppKit-Aufruf, der aus der Huelle
/// unter `appkit/` herauswandert, weil jemand ihn an der naechstbesten Stelle
/// brauchte; das ist die Bewegung, die der Plan verbietet und die im Alltag
/// vorkommt. Ein Umbenennen der Abhaengigkeit ist dagegen ein eigener,
/// sichtbarer Eingriff in `Cargo.toml`, wo jede der `objc2`-Kisten heute unter
/// ihrem eigenen Namen und mit einer eigenen Begruendung steht. Es dafuer
/// einzurichten hiesse, dem Werkzeug ein zweites Dateiformat und eine zweite
/// Grammatik beizubringen, gegen einen Zug, den niemand versehentlich macht —
/// genau die Sammlung von Sonderfaellen, die "supersimpel" ausschliesst.
/// Nachgesehen am 260807: keine `Cargo.toml` des Workspace benennt eine Kiste
/// um, und keine Datei unter `crates/` schreibt `extern crate`.
fn verletzt_grenze(zeile: &str) -> bool {
    let inhalt = zeile.trim_start();
    if inhalt.starts_with('/') {
        return false;
    }
    ist_objc2_use(inhalt) || nennt_objc2_pfad(inhalt)
}

/// Ob die Zeile einen ausgeschriebenen Pfad in eine `objc2`-Kiste nennt.
///
/// Gesucht ist ein Bezeichner, der mit `objc2` beginnt und auf den unmittelbar
/// `::` folgt: `objc2::rc::Weak`, `objc2_app_kit::NSView`,
/// `<objc2_foundation::NSString>::from_str`. Vor dem `objc2` muss ein
/// Zeichen stehen, das kein Bezeichnerzeichen ist, sonst traefe die Suche auch
/// `meinobjc2::x`, also einen fremden Namen, der nur so endet.
///
/// Die Zeile wird nicht auf Kommentare geprueft; das erledigt
/// `verletzt_grenze` vorher.
fn nennt_objc2_pfad(zeile: &str) -> bool {
    let bytes = zeile.as_bytes();
    let mut ab = 0;
    while let Some(stelle) = zeile[ab..].find("objc2") {
        let anfang = ab + stelle;
        ab = anfang + "objc2".len();
        if anfang > 0 && ist_bezeichnerzeichen(bytes[anfang - 1]) {
            continue;
        }
        let mut ende = ab;
        while ende < bytes.len() && ist_bezeichnerzeichen(bytes[ende]) {
            ende += 1;
        }
        if zeile[ende..].starts_with("::") {
            return true;
        }
    }
    false
}

/// Ob das Byte in einem Rust-Bezeichner stehen darf.
///
/// Nur die ASCII-Haelfte: ein Bezeichner darf zwar auch Unicode tragen, aber
/// keine Kiste des Vorhabens tut das, und ein Fortsetzungsbyte einer deutschen
/// Umlaut-Kommentarzeile gilt so als Grenze statt als Bezeichnerzeichen.
fn ist_bezeichnerzeichen(zeichen: u8) -> bool {
    zeichen.is_ascii_alphanumeric() || zeichen == b'_'
}

/// Ob eine Zeile eine `use objc2`-Zeile ist.
///
/// Gelesen wird: Einrueckung, eine mitgeschriebene Sichtbarkeit, `use`, ein
/// Trenner, ein moegliches fuehrendes `::`, dann ein Pfad, der mit `objc2`
/// beginnt. Ein Modulkommentar wie "In dieser Datei steht keine `use
/// objc2`-Zeile" beginnt nach der Einrueckung mit `//` und faellt durch —
/// genau die sechs Treffer, die eine unverankerte Suche gefunden haette.
///
/// **Zwei Schreibweisen, die bis zum 260806 durchkamen.** Die Vorgaengerin
/// verlangte `use` unmittelbar nach der Einrueckung und `objc2` unmittelbar
/// nach dem Zwischenraum. `pub use objc2_app_kit::NSView;` beginnt aber mit
/// `pub`, und `use ::objc2::rc::Retained;` schiebt `::` dazwischen; beide sind
/// gueltiges Rust, und ein Reexport der ersten Sorte haette jedem weiteren
/// Verbraucher die eigene `use objc2`-Zeile erspart. Einen Verstoss gab es
/// nicht, die Luecke war trotzdem da
/// (`issues/260806-0834_*_die-appkit-grenzpruefung-uebersieht-pub-use-und-use-mit-fuehrendem-doppelpunkt.md`).
fn ist_objc2_use(zeile: &str) -> bool {
    let ohne_sichtbarkeit = sichtbarkeit_abstreifen(zeile.trim_start());
    let Some(nach_use) = ohne_sichtbarkeit.strip_prefix("use") else {
        return false;
    };
    let getrimmt = nach_use.trim_start();
    // Nach `use` steht ein Trenner: Zwischenraum oder das fuehrende `::`.
    // Ohne beides ist es ein Bezeichner wie `useobjc2`.
    let pfad = match getrimmt.strip_prefix("::") {
        Some(rest) => rest.trim_start(),
        None if getrimmt.len() < nach_use.len() => getrimmt,
        None => return false,
    };
    pfad.starts_with("objc2")
}

/// Streift ein mitgeschriebenes Sichtbarkeitspraefix ab.
///
/// `pub`, `pub(crate)`, `pub(super)`, `pub(in ::eine::stelle)` — alles, was
/// vor `use` stehen darf. Steht keines da oder faengt das Wort nur mit `pub`
/// an (`public_use`), kommt die Zeile unveraendert zurueck.
fn sichtbarkeit_abstreifen(zeile: &str) -> &str {
    let Some(nach_pub) = zeile.strip_prefix("pub") else {
        return zeile;
    };
    if let Some(offen) = nach_pub.strip_prefix('(') {
        // Die erste schliessende Klammer ist die zugehoerige: der Inhalt einer
        // Sichtbarkeitsangabe traegt selbst keine Klammern.
        return match offen.find(')') {
            Some(stelle) => offen[stelle + 1..].trim_start(),
            None => zeile,
        };
    }
    let getrimmt = nach_pub.trim_start();
    if getrimmt.len() < nach_pub.len() {
        getrimmt
    } else {
        zeile
    }
}

/// Prueft, dass beide Ziel-Tripel installiert sind.
///
/// Ein fehlendes Tripel soll mit seinem Namen und dem Kommando dagegen
/// abbrechen, nicht erst mitten im zweiten Uebersetzungslauf. Laeuft `rustup`
/// selbst nicht (etwa bei einer Werkzeugkette ohne `rustup`), faellt die
/// Vorpruefung aus, und der Uebersetzungslauf meldet ein fehlendes Ziel
/// selbst.
fn ziele_pruefen() -> Result<(), Abbruch> {
    let Ok(ausgabe) = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
    else {
        println!(
            "Hinweis: rustup laesst sich nicht starten, die Zielpruefung entfaellt. Ein \
             fehlendes Ziel meldet der Uebersetzungslauf selbst."
        );
        return Ok(());
    };
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "rustup target list --installed ist gescheitert ({}): {}",
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    let installiert = String::from_utf8_lossy(&ausgabe.stdout).into_owned();
    for ziel in ZIELE {
        if !installiert.lines().any(|zeile| zeile.trim() == ziel) {
            return Err(Abbruch::Lauf(format!(
                "Das Ziel {ziel} ist nicht installiert; die universelle Binaerdatei braucht \
                 beide Ziele aus rust-toolchain.toml. Abhilfe: rustup target add {ziel}"
            )));
        }
    }
    Ok(())
}

/// Fuegt die beiden uebersetzten Binaerdateien mit `lipo` zusammen.
///
/// Ergebnis ist `target/universal/<binaername>`; `lipo -archs` prueft es
/// sofort gegen beide Architekturen, damit ein halbes Ergebnis nicht erst am
/// ausgelieferten Buendel auffaellt.
fn zusammenfuegen(vorlage: &bundle::Vorlage) -> Result<PathBuf, Abbruch> {
    let ordner = vorlage.wurzel.join("target").join("universal");
    fs::create_dir_all(&ordner).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} laesst sich nicht anlegen: {fehler}",
            ordner.display()
        ))
    })?;
    let ausgabe_pfad = ordner.join(&vorlage.binaername);

    let mut kommando = Command::new("/usr/bin/lipo");
    kommando.arg("-create");
    for ziel in ZIELE {
        kommando.arg(bundle::zielpfad(
            &vorlage.wurzel,
            Some(ziel),
            &vorlage.binaername,
        ));
    }
    kommando.arg("-output").arg(&ausgabe_pfad);
    let ausgabe = kommando
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("lipo laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "lipo -create ist gescheitert ({}): {}",
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }

    let archs = Command::new("/usr/bin/lipo")
        .arg("-archs")
        .arg(&ausgabe_pfad)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("lipo laesst sich nicht starten: {fehler}")))?;
    if !archs.status.success() {
        return Err(Abbruch::Lauf(format!(
            "lipo -archs ist gescheitert ({}): {}",
            archs.status,
            String::from_utf8_lossy(&archs.stderr).trim()
        )));
    }
    let gemeldet = String::from_utf8_lossy(&archs.stdout).into_owned();
    for architektur in ARCHITEKTUREN {
        if !gemeldet.split_whitespace().any(|wort| wort == architektur) {
            return Err(Abbruch::Lauf(format!(
                "lipo -archs meldet {:?} statt beider Architekturen {}; die Binaerdatei ist \
                 nicht universell.",
                gemeldet.trim(),
                ARCHITEKTUREN.join(" ")
            )));
        }
    }
    println!("lipo -archs: {}", gemeldet.trim());
    Ok(ausgabe_pfad)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die zwei Ziele dieses Projekts unter dem Namen ihres Pruefwerkzeugs.
    ///
    /// Sie haelt zugleich die Paarung der beiden Aufzaehlungen: wer eine davon
    /// umsortiert und die andere stehen laesst, meldet hier `x86_64` fuer
    /// `aarch64`.
    #[test]
    fn die_beiden_ziele_tragen_die_namen_die_lipo_dafuer_meldet() {
        assert_eq!(lipo_name("aarch64"), "arm64");
        assert_eq!(lipo_name("x86_64"), "x86_64");
    }

    /// Die Umrechnung deckt jedes gebaute Ziel ab.
    ///
    /// Sie liest die Rust-Namen aus [`ZIELE`] statt sie aufzuschreiben; ein
    /// drittes Ziel liefe hier von selbst mit und muesste einen Namen aus
    /// [`ARCHITEKTUREN`] bekommen, statt still durchgereicht zu werden.
    #[test]
    fn jedes_ziel_tripel_bekommt_einen_namen_aus_den_architekturen() {
        for ziel in ZIELE {
            let rust_name = ziel.split('-').next().unwrap();
            assert!(
                ARCHITEKTUREN.contains(&lipo_name(rust_name)),
                "{ziel} wird nicht uebersetzt"
            );
        }
    }

    /// Ein unbekannter Name wird durchgereicht und nicht erfunden.
    ///
    /// Vier Faelle, die keine Uebersetzung haben: eine fremde Architektur, ein
    /// Name, der schon der von `lipo` ist, das ganze Tripel statt seines
    /// Praefixes und die leere Zeichenkette. Keiner darf verschwinden, und
    /// keiner darf zu `arm64` oder `x86_64` werden.
    #[test]
    fn ein_unbekannter_name_wird_durchgereicht_und_nicht_erfunden() {
        assert_eq!(lipo_name("riscv64"), "riscv64");
        assert_eq!(lipo_name("arm64"), "arm64");
        assert_eq!(lipo_name("aarch64-apple-darwin"), "aarch64-apple-darwin");
        assert_eq!(lipo_name(""), "");
    }

    #[test]
    fn eine_use_zeile_aus_einer_objc2_kiste_ist_ein_verstoss() {
        assert!(ist_objc2_use("use objc2::rc::Retained;"));
        assert!(ist_objc2_use("use objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("    use objc2_foundation::NSString;"));
        assert!(ist_objc2_use("\tuse  objc2::MainThreadMarker;"));
    }

    #[test]
    fn ein_modulkommentar_ueber_die_grenze_ist_kein_verstoss() {
        // Genau die sechs Treffer, die die unverankerte Suche am 260805-0000
        // gefunden haette: Kommentare der Form "keine `use objc2`-Zeile".
        assert!(!ist_objc2_use(
            "// In dieser Datei steht keine `use objc2`-Zeile."
        ));
        assert!(!ist_objc2_use(
            "//! In dieser Datei steht keine `use objc2`-Zeile."
        ));
    }

    #[test]
    fn andere_use_zeilen_sind_kein_verstoss() {
        assert!(!ist_objc2_use("use std::path::PathBuf;"));
        assert!(!ist_objc2_use("use crate::appkit;"));
        assert!(!ist_objc2_use("useobjc2::x;"));
        assert!(!ist_objc2_use("user objc2"));
        assert!(!ist_objc2_use("use"));
        assert!(!ist_objc2_use("pub use crate::appkit;"));
        assert!(!ist_objc2_use("pub(crate) use std::fmt;"));
        // Ein Bezeichner, der mit `pub` anfaengt, ist keine Sichtbarkeit.
        assert!(!ist_objc2_use("public_use objc2::x;"));
    }

    /// Die beiden Schreibweisen, die bis zum 260806 durchkamen.
    #[test]
    fn sichtbarkeit_und_fuehrendes_doppelkolon_kommen_nicht_durch() {
        assert!(ist_objc2_use("pub use objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("pub(crate) use objc2::rc::Retained;"));
        assert!(ist_objc2_use("pub(super) use objc2_foundation::NSString;"));
        assert!(ist_objc2_use("pub(in crate::appkit) use objc2::sel;"));
        assert!(ist_objc2_use("use ::objc2::rc::Retained;"));
        assert!(ist_objc2_use("use::objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("    pub use ::objc2::MainThreadMarker;"));
    }

    /// Die erste der beiden Luecken vom 260806-1333: der ausgeschriebene Pfad.
    #[test]
    fn ein_ausgeschriebener_objc2_pfad_ist_ein_verstoss() {
        // Woertlich aus crates/krk-ui/src/appkit/anwendung.rs:575. Innerhalb
        // von appkit/ ist die Zeile erlaubt, ausserhalb ist sie der Verstoss,
        // den die Vorgaengerin nicht sah.
        assert!(verletzt_grenze(
            "            let schwach = objc2::rc::Weak::from_retained(&self.retain());"
        ));
        assert!(verletzt_grenze("    objc2_app_kit::NSView::alloc(mtm);"));
        assert!(verletzt_grenze(
            "    let text = <objc2_foundation::NSString>::from_str(\"x\");"
        ));
        assert!(verletzt_grenze(
            "    fn sicht(&self) -> objc2::rc::Retained<NSView> {"
        ));
        // Die `use`-Zeile bleibt ein Verstoss, jetzt ueber dieselbe Frage.
        assert!(verletzt_grenze("use objc2::rc::Retained;"));
        assert!(verletzt_grenze("pub use objc2_app_kit::NSView;"));
        // Ohne `::` ist `objc2` nur ein Wort; die `use`-Zeile faengt es.
        assert!(verletzt_grenze("use objc2_app_kit as ak;"));
    }

    /// Die dreizehn Kommentarzeilen, die es heute im Baum gibt — woertlich.
    ///
    /// Zehn unter `crates/krk-ui/src` ausserhalb von `appkit/`, zwei unter
    /// `crates/krk-core/src`, eine unter `crates/krk-core/tests`. Schlaegt die
    /// Pruefung auf einer davon an, ist der Bau sofort rot, ohne dass die
    /// Grenze verletzt waere. Die dreizehnte kam am 260807 dazu, nicht weil
    /// jemand sie geschrieben haette, sondern weil die Pruefung seither auch
    /// `tests/` liest.
    #[test]
    fn die_kommentarzeilen_des_baums_sind_kein_verstoss() {
        for zeile in [
            "//! **Keine Zeile AppKit.** In diesem Verzeichnis steht keine `use objc2`-Zeile,",
            "//! keines von ihnen nennt eine `objc2`-Kiste. `messmodus` haelt den Ablauf der",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, und",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, wie",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile und",
            "//! hier keine `use objc2`-Zeile. Wo der Fokus steht, liest",
            "//! keine `use objc2`-Zeile**, und das ist nachpruefbar, nicht nur gemeint.",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die",
            "//! hier keine `use objc2`-Zeile. Die Ansichten dazu sind die vier Blaetter unter",
            "//! Diese Datei ist reines Rust und nennt keine `objc2`-Kiste. Sie bekommt eine",
            "//! weiterhin von oben nach unten: `krk-core` nennt keine `objc2`-Kiste. Ein",
            "    // nachgesehen am 260803 und fuer die acht Nachtraege am 260804. `objc2`",
        ] {
            assert!(!verletzt_grenze(zeile), "schlaegt an auf: {zeile}");
        }
        // Und ein Kommentar, der den Pfad ausschreibt: heute steht er so
        // nirgends, morgen kann er es. Die Kommentarregel faengt ihn.
        assert!(!verletzt_grenze(
            "//! Die Huelle um `objc2::rc::Retained` liegt unter `appkit/`."
        ));
        assert!(!verletzt_grenze(
            "    /// Reicht `objc2_app_kit::NSView` nach draussen."
        ));
    }

    #[test]
    fn zeilen_ohne_objc2_sind_kein_verstoss() {
        assert!(!verletzt_grenze("use std::path::PathBuf;"));
        assert!(!verletzt_grenze("    let x = std::mem::take(&mut y);"));
        assert!(!verletzt_grenze(""));
        // Ein fremder Name, der nur auf `objc2` endet.
        assert!(!verletzt_grenze("    meinobjc2::rufen();"));
        // `objc2` ohne folgendes `::` und ohne `use` ist nur ein Wort.
        assert!(!verletzt_grenze("    let name = \"objc2\";"));
    }

    /// Die Pruefung am echten Baum, nicht nur an erfundenen Zeilen.
    ///
    /// Sie haengt sonst allein an `cargo xtask release`, und das verlangt eine
    /// Signaturidentitaet und zwei Uebersetzungslaeufe. So laeuft dieselbe
    /// Pruefung bei jedem `make check` mit und meldet einen Verstoss am Tag,
    /// an dem er entsteht, statt am Tag der Auslieferung.
    #[test]
    fn die_grenzpruefung_laeuft_am_baum_gruen() {
        appkit_grenze_pruefen(&bundle::wurzel()).expect("die AppKit-Grenze haelt");
    }

    /// Die Pruefung reicht ueber `src/` hinaus und haelt vor `appkit/` an.
    ///
    /// Gebaut wird ein Wegwerf-Workspace mit drei Dateien: eine unter
    /// `crates/krk-ui/src/appkit/` (erlaubt), eine unter `crates/krk-ui/tests/`
    /// und eine unter `crates/krk-ui/` selbst (beide verboten). Bis zum 260807
    /// sah die Pruefung allein `<kiste>/src` und liess die zweite und die
    /// dritte durch, waehrend sie meldete, es gebe keinen Verstoss
    /// (`issues/260807-0800_*_die-appkit-grenzpruefung-kennt-nur-src-baeume-und-nur-die-woertliche-schreibweise.md`).
    #[test]
    fn die_pruefung_liest_jeden_baum_der_kiste_und_nicht_nur_src() {
        let wurzel = wegwerfwurzel("grenzbaeume");
        let ui = wurzel.pfad().join("crates/krk-ui");
        schreiben(&ui.join("src/appkit/huelle.rs"), "use objc2::rc::Retained;");
        schreiben(&ui.join("src/modell.rs"), "let x = 1;");
        schreiben(&ui.join("tests/probe.rs"), "use objc2_app_kit::NSView;");
        schreiben(&ui.join("build.rs"), "    objc2::rc::Weak::neu();");

        let fehler = appkit_grenze_pruefen(wurzel.pfad())
            .expect_err("zwei Dateien ausserhalb von appkit/ nennen eine objc2-Kiste");
        let Abbruch::Lauf(meldung) = fehler else {
            panic!("die Grenzverletzung ist ein Laufabbruch");
        };
        assert!(meldung.contains("tests/probe.rs"), "{meldung}");
        assert!(meldung.contains("build.rs"), "{meldung}");
        assert!(
            !meldung.contains("huelle.rs"),
            "appkit/ ist die Ausnahme: {meldung}"
        );
    }

    /// Ein Wegwerf-Wurzelordner, wie ihn die Proben des Kerns benutzen.
    struct Wegwerfwurzel {
        pfad: PathBuf,
    }

    impl Wegwerfwurzel {
        fn pfad(&self) -> &Path {
            &self.pfad
        }
    }

    impl Drop for Wegwerfwurzel {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.pfad);
        }
    }

    fn wegwerfwurzel(zweck: &str) -> Wegwerfwurzel {
        let laufnummer = ZAEHLER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pfad = std::env::temp_dir().join(format!(
            "krk-xtask-test-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        Wegwerfwurzel { pfad }
    }

    static ZAEHLER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn schreiben(pfad: &Path, inhalt: &str) {
        fs::create_dir_all(pfad.parent().expect("die Datei hat einen Ordner"))
            .expect("der Ordner laesst sich nicht anlegen");
        fs::write(pfad, inhalt).expect("die Datei laesst sich nicht schreiben");
    }

    /// Die Ausgabe von `git tag --points-at HEAD` auf einem getaggten Stand.
    const TAG_PASST: &str = "v0.1.0\n";

    /// Dieselbe Abfrage auf einem Stand ohne jeden Tag: eine leere Ausgabe.
    /// So sieht der Baum am 260813 aus, denn er traegt keinen einzigen Tag.
    const TAG_FEHLT: &str = "";

    /// Mehrere Tags auf demselben Commit, der passende in der Mitte. `git tag`
    /// gibt sie sortiert und je Zeile aus.
    const TAG_UNTER_MEHREREN: &str = "release-2026-08\nv0.1.0\nvorletzter-stand\n";

    /// Ein Tag, der nur so anfaengt wie der gesuchte. Er darf nicht passen,
    /// sonst deckte `v0.1.0-rc1` die Auslieferung von `0.1.0`.
    const TAG_AEHNLICH: &str = "v0.1.0-rc1\nv0.1.10\n";

    /// Ein sauberer Arbeitsbaum: `git status --porcelain` schweigt.
    const BAUM_SAUBER: &str = "";

    /// Zwei geaenderte verfolgte Dateien, eine vorgemerkt und eine nicht.
    /// `--porcelain` schreibt beide in dieselbe Form.
    const BAUM_GEAENDERT: &str = "M  xtask/src/release.rs\n M README.md\n";

    /// Eine geloeschte verfolgte Datei. Sie zaehlt mit, denn ein Buendel aus
    /// einem Baum ohne sie ist nicht der getaggte Stand.
    const BAUM_GELOESCHT: &str = " D crates/krk-ui/src/fenstertitel.rs\n";

    #[test]
    fn ein_getaggter_und_sauberer_stand_geht_durch() {
        assert!(stand_pruefen("0.1.0", TAG_PASST, BAUM_SAUBER).is_ok());
    }

    #[test]
    fn unter_mehreren_tags_genuegt_der_passende() {
        assert!(stand_pruefen("0.1.0", TAG_UNTER_MEHREREN, BAUM_SAUBER).is_ok());
    }

    #[test]
    fn ein_fehlender_tag_haelt_die_auslieferung_an() {
        let meldung =
            stand_pruefen("0.1.0", TAG_FEHLT, BAUM_SAUBER).expect_err("ohne Tag kein Paket");
        assert!(meldung.contains("kein Tag v0.1.0"), "{meldung}");
    }

    /// Ein Tag, der mit dem gesuchten Namen nur anfaengt, deckt ihn nicht.
    #[test]
    fn ein_aehnlicher_tag_deckt_die_version_nicht() {
        assert!(stand_pruefen("0.1.0", TAG_AEHNLICH, BAUM_SAUBER).is_err());
    }

    #[test]
    fn ein_geaenderter_baum_haelt_die_auslieferung_an() {
        let meldung = stand_pruefen("0.1.0", TAG_PASST, BAUM_GEAENDERT)
            .expect_err("ein geaenderter Baum ist nicht der getaggte Stand");
        assert!(
            meldung.contains("weicht vom eingetragenen Stand ab"),
            "{meldung}"
        );
        assert!(meldung.contains("xtask/src/release.rs"), "{meldung}");
        assert!(meldung.contains("README.md"), "{meldung}");
        assert!(meldung.contains("2 verfolgte Dateien sind"), "{meldung}");
    }

    #[test]
    fn eine_geloeschte_verfolgte_datei_zaehlt_mit() {
        let meldung = stand_pruefen("0.1.0", TAG_PASST, BAUM_GELOESCHT)
            .expect_err("eine geloeschte verfolgte Datei ist eine Abweichung");
        assert!(meldung.contains("fenstertitel.rs"), "{meldung}");
        assert!(meldung.contains("1 verfolgte Datei ist"), "{meldung}");
    }

    /// Treffen beide Befunde zu, nennt **eine** Meldung beide. Der Nutzer
    /// raeumt nicht erst den Baum auf, um danach vom fehlenden Tag zu
    /// erfahren.
    #[test]
    fn beide_befunde_stehen_in_einer_meldung() {
        let meldung = stand_pruefen("0.1.0", TAG_FEHLT, BAUM_GEAENDERT)
            .expect_err("beide Bedingungen sind verletzt");
        assert!(meldung.contains("kein Tag v0.1.0"), "{meldung}");
        assert!(
            meldung.contains("weicht vom eingetragenen Stand ab"),
            "{meldung}"
        );
    }

    /// Ohne Git-Verzeichnis bricht Station 1 ab und baut nicht ersatzweise
    /// durch (C3.11).
    ///
    /// Der Wegwerfordner liegt im Temporaerverzeichnis und damit ausserhalb
    /// jedes Arbeitsbaums; `git rev-parse --git-dir` scheitert dort. Das ist
    /// die eine Probe dieser Datei, die `git` wirklich startet — die drei
    /// Vergleichsfaelle darueber brauchen weder Prozess noch Verzeichnis.
    #[test]
    fn ohne_git_verzeichnis_bricht_station_eins_ab() {
        let wurzel = wegwerfwurzel("ohne-git");
        fs::create_dir_all(wurzel.pfad()).expect("der Ordner laesst sich anlegen");

        let fehler = auslieferungsstand_pruefen(wurzel.pfad())
            .expect_err("im Temporaerverzeichnis liegt kein Git-Verzeichnis");
        let Abbruch::Lauf(meldung) = fehler else {
            panic!("das fehlende Git-Verzeichnis ist ein Laufabbruch");
        };
        assert!(meldung.contains("braucht ein Git-Verzeichnis"), "{meldung}");
        assert!(
            meldung.contains("Es wird nicht ersatzweise durchgebaut"),
            "{meldung}"
        );
    }

    /// Die drei Bestandteile aus C3.8, und was die Meldung nicht nennt.
    #[test]
    fn die_meldung_nennt_bedingung_version_und_abhilfe() {
        let meldung = stand_pruefen("1.2.3", TAG_FEHLT, BAUM_GEAENDERT)
            .expect_err("beide Bedingungen sind verletzt");
        // Die verletzte Bedingung.
        assert!(meldung.contains("kein Tag v1.2.3"), "{meldung}");
        assert!(
            meldung.contains("weicht vom eingetragenen Stand ab"),
            "{meldung}"
        );
        // Die Version aus der Cargo.toml.
        assert!(meldung.contains("Version 1.2.3"), "{meldung}");
        // Die Abhilfe, als kopierbares Kommando. Seit dem 260813 ist es der
        // Auslieferungsweg selbst und nicht mehr ein `git tag` von Hand.
        assert!(meldung.contains("./release.sh 1.2.3"), "{meldung}");
        assert!(meldung.contains("cargo xtask version 1.2.3"), "{meldung}");
        assert!(meldung.contains("git commit -a"), "{meldung}");
        // Kein Weg vorbei: weder Gewalt noch eine Marke zum Ueberspringen.
        assert!(!meldung.contains("--force"), "{meldung}");
        assert!(!meldung.contains("-f "), "{meldung}");
        assert!(!meldung.contains("--no-verify"), "{meldung}");
        assert!(
            meldung.contains("Es entsteht kein Auslieferungspaket."),
            "{meldung}"
        );
    }

    /// Genau ein Aufruf von `git` im ganzen Baum (C3.13).
    ///
    /// Die Nadel steht als `concat!`, weil die Probe in derselben Datei liegt,
    /// die sie liest: ausgeschrieben zaehlte sie sich selbst mit.
    ///
    /// Seit dem 260813 steht der Aufruf in `git.rs` und nicht mehr hier: das
    /// Werkzeug hat einen zweiten Abnehmer bekommen, `version`, und der
    /// schreibt. Die Zahl bleibt eins, und die Probe bleibt hier, weil die
    /// Zusage aus dem Abnahmekriterium dieser Station stammt.
    #[test]
    fn xtask_ruft_git_an_genau_einer_stelle() {
        let nadel = concat!("Command", "::new(\"/usr/bin/git\")");
        let nackt = concat!("Command", "::new(\"git\")");
        let mut stellen = Vec::new();
        for datei in rust_dateien(&bundle::wurzel()) {
            let inhalt = fs::read_to_string(&datei).expect("die Datei ist lesbar");
            let zahl = inhalt.matches(nadel).count() + inhalt.matches(nackt).count();
            for _ in 0..zahl {
                stellen.push(datei.clone());
            }
        }
        assert_eq!(stellen.len(), 1, "git wird gerufen in {stellen:?}");
        assert!(stellen[0].ends_with("xtask/src/git.rs"), "{:?}", stellen[0]);
    }

    /// Die Tag-Pruefung haengt allein an `release` (C3.12).
    ///
    /// `cargo xtask bundle` fragt weder nach einem Tag noch nach dem
    /// Arbeitsbaum, und `make check` bekommt keine neue Vorbedingung: die
    /// sieben Ziele des `Makefile`, die an `bundle` haengen, laufen
    /// unveraendert.
    #[test]
    fn allein_release_fragt_nach_tag_und_arbeitsbaum() {
        let nadel = concat!("auslieferungsstand_", "pruefen");
        let mut dateien = Vec::new();
        for datei in rust_dateien(&bundle::wurzel()) {
            let inhalt = fs::read_to_string(&datei).expect("die Datei ist lesbar");
            if inhalt.contains(nadel) {
                dateien.push(datei);
            }
        }
        assert_eq!(dateien.len(), 1, "die Station steht in {dateien:?}");
        assert!(
            dateien[0].ends_with("xtask/src/release.rs"),
            "{:?}",
            dateien[0]
        );

        let bundle_quelle = fs::read_to_string(bundle::wurzel().join("xtask/src/bundle.rs"))
            .expect("bundle.rs ist lesbar");
        assert!(
            !bundle_quelle.contains(concat!("/usr/bin/", "git")),
            "bundle.rs ruft git"
        );
        assert!(!bundle_quelle.contains(nadel), "bundle.rs prueft den Stand");
    }

    /// Station 1 steht vor dem ersten Uebersetzungslauf (C3.9).
    ///
    /// **Was diese Probe nicht sieht:** sie liest die Reihenfolge des Textes
    /// im Rumpf von [`ausfuehren`] und nicht den Ablauf. Was sie haelt, ist
    /// die eine Zusage, dass kein Abbruch dieser Art einen Uebersetzungslauf
    /// kostet.
    #[test]
    fn die_standpruefung_steht_vor_der_ersten_uebersetzung() {
        let quelle = include_str!("release.rs");
        let anfang = quelle
            .find(concat!("pub fn ", "ausfuehren("))
            .expect("release.rs fuehrt ausfuehren");
        let rumpf = &quelle[anfang..];
        let ende = rumpf.find("\n}\n").expect("ausfuehren hat ein Ende");
        let rumpf = &rumpf[..ende];

        let pruefung = rumpf
            .find(concat!("auslieferungsstand_", "pruefen(&"))
            .expect("ausfuehren ruft Station 1");
        let uebersetzung = rumpf
            .find("bundle::uebersetzen(")
            .expect("ausfuehren uebersetzt");
        assert!(
            pruefung < uebersetzung,
            "Station 1 steht hinter dem ersten Uebersetzungslauf"
        );
    }

    /// Die aeussere Voraussetzung steht vor dem ersten Uebersetzungslauf.
    ///
    /// Dieselbe Zusage wie bei
    /// [`die_standpruefung_steht_vor_der_ersten_uebersetzung`], fuer die dritte
    /// aeussere Voraussetzung der Kette: fehlt `gh`, faellt es auf, bevor
    /// irgendetwas geschehen ist — und nicht erst hinter einer abgeschlossenen
    /// Einreichung bei Apple (Durchsicht 260821-1346, B4).
    ///
    /// **Was diese Probe nicht sieht:** dieselbe Grenze wie dort — sie liest
    /// die Reihenfolge des Textes und nicht den Ablauf.
    #[test]
    fn die_aeussere_voraussetzung_steht_vor_der_ersten_uebersetzung() {
        let quelle = include_str!("release.rs");
        let anfang = quelle
            .find(concat!("pub fn ", "ausfuehren("))
            .expect("release.rs fuehrt ausfuehren");
        let rumpf = &quelle[anfang..];
        let ende = rumpf.find("\n}\n").expect("ausfuehren hat ein Ende");
        let rumpf = &rumpf[..ende];

        let pruefung = rumpf
            .find(concat!("veroeffentlichung::gh_", "pruefen()"))
            .expect("ausfuehren fragt nach gh");
        let uebersetzung = rumpf
            .find("bundle::uebersetzen(")
            .expect("ausfuehren uebersetzt");
        assert!(
            pruefung < uebersetzung,
            "die Frage nach gh steht hinter dem ersten Uebersetzungslauf"
        );
    }

    /// Die achte Station steht hinter der Beglaubigung (C1.4).
    ///
    /// Die Reihenfolge ist keine Bequemlichkeit: veroeffentlicht wird ein
    /// Buendel mit angeheftetem Ticket, und angeheftet wird es in Station 7.
    /// Stuende die achte davor, truege das Zip den Nachweis nicht — und die
    /// Ticketpruefung jener Station liesse den Lauf jedesmal scheitern.
    ///
    /// **Was diese Probe nicht sieht:** dieselbe Grenze wie bei
    /// [`die_standpruefung_steht_vor_der_ersten_uebersetzung`] — sie liest die
    /// Reihenfolge des Textes und nicht den Ablauf.
    #[test]
    fn die_achte_station_steht_hinter_der_beglaubigung() {
        let quelle = include_str!("release.rs");
        let anfang = quelle
            .find(concat!("pub fn ", "ausfuehren("))
            .expect("release.rs fuehrt ausfuehren");
        let rumpf = &quelle[anfang..];
        let ende = rumpf.find("\n}\n").expect("ausfuehren hat ein Ende");
        let rumpf = &rumpf[..ende];

        let heften = rumpf
            .find(concat!("beglaubigung", "::beglaubigen(&"))
            .expect("ausfuehren faehrt Station 7");
        let veroeffentlichen = rumpf
            .find(concat!("veroeffentlichung", "::veroeffentlichen("))
            .expect("ausfuehren faehrt Station 8");
        assert!(
            heften < veroeffentlichen,
            "Station 8 steht vor der Beglaubigung"
        );
    }

    /// Die Hilfezeile des `Makefile` nennt das Schieben.
    ///
    /// **Was `make help` ausgibt, ist das Letzte, was der Nutzer vor dem Tippen
    /// liest.** `make release` schiebt seit dem 260821 HEAD und einen Tag zu
    /// `origin`; die `##`-Zeile sagte es bis zur Durchsicht 260821-1346 nicht.
    /// Die Zaehlprobe darunter konnte die Stelle nicht fangen, denn eine
    /// Zaehlprobe faengt, was falsch **dasteht**, nie, was fehlt. Diese hier
    /// fragt nach dem, was dastehen muss.
    #[test]
    fn die_hilfezeile_des_makefiles_nennt_das_schieben() {
        let makefile =
            fs::read_to_string(bundle::wurzel().join("Makefile")).expect("das Makefile ist lesbar");
        let zeile = makefile
            .lines()
            .find(|zeile| zeile.starts_with("release: ##"))
            .expect("das Makefile fuehrt ein Ziel release mit Hilfezeile");
        assert!(zeile.contains("origin"), "{zeile}");
        assert!(zeile.contains("schieben"), "{zeile}");
    }

    /// Alle `.rs`-Dateien des Baums, ohne `target/` und ohne das
    /// Git-Verzeichnis.
    fn rust_dateien(wurzel: &Path) -> Vec<PathBuf> {
        let mut gefunden = Vec::new();
        sammeln(wurzel, &mut gefunden);
        gefunden.sort();
        gefunden
    }

    /// Der Quellbaum nennt die alte Stationszahl nicht mehr (C6.3).
    ///
    /// Gelesen werden `README.md`, das `Makefile` und jede `.rs`-Datei unter
    /// `xtask/`, also die drei Orte, an denen der Weg beschrieben wird. Gesucht
    /// ist die Wendung aus der Zahl vor der achten Station und dem Wort
    /// `Stationen`; sie stand am 260821 an sieben Stellen und steht seitdem an
    /// keiner.
    ///
    /// **Die Werkbank bleibt draussen, und das ist eine Festlegung.** Unter
    /// `fusion-workbench/` liegen Aufzeichnungen eines vergangenen Standes, und
    /// die behalten nach der Ortsregel aus `CLAUDE.md` ihren damaligen
    /// Wortlaut. Das Abnahmekriterium selbst enthaelt ueberdies die
    /// Zeichenfolge, die es verbietet, ist also woertlich genommen nicht
    /// erfuellbar; der Befund ist gefilt
    /// (`shared/issues/260821-1221_*_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md`),
    /// und der Plan begrenzt die Zusage deshalb auf den Quellbaum.
    ///
    /// Die Nadel steht als `concat!`, und keine Meldung dieser Probe schreibt
    /// sie aus: die Probe liegt in einer der Dateien, die sie liest, und
    /// ausgeschrieben zaehlte sie sich selbst mit.
    #[test]
    fn der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr() {
        let nadel = concat!("sieben ", "Stationen");
        let wurzel = bundle::wurzel();
        let mut zu_lesen = vec![wurzel.join("README.md"), wurzel.join("Makefile")];
        zu_lesen.extend(rust_dateien(&wurzel.join("xtask")));

        let mut stellen = Vec::new();
        for datei in zu_lesen {
            let inhalt = fs::read_to_string(&datei).expect("die Datei ist lesbar");
            if inhalt.contains(nadel) {
                stellen.push(datei);
            }
        }
        assert!(
            stellen.is_empty(),
            "die alte Zahl steht noch in {stellen:?}"
        );
    }

    fn sammeln(ordner: &Path, gefunden: &mut Vec<PathBuf>) {
        let Ok(eintraege) = fs::read_dir(ordner) else {
            return;
        };
        for eintrag in eintraege.flatten() {
            let pfad = eintrag.path();
            let name = eintrag.file_name();
            if pfad.is_dir() {
                if name == "target" || name == ".git" {
                    continue;
                }
                sammeln(&pfad, gefunden);
                continue;
            }
            if pfad.extension().is_some_and(|endung| endung == "rs") {
                gefunden.push(pfad);
            }
        }
    }

    #[test]
    fn release_nimmt_keine_weiteren_marken() {
        let argumente = vec!["--adhoc".to_owned()];
        assert!(matches!(ausfuehren(&argumente), Err(Abbruch::Aufruf(_))));
    }
}
