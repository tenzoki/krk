//! Der eine Zugang zu `git`.
//!
//! **Hier steht der einzige Prozessaufruf von `git` im ganzen Baum.** Die Probe
//! `xtask_ruft_git_an_genau_einer_stelle` in `release` haelt die Zahl auf eins.
//! Bis zum 260813 stand der Aufruf in `release`, weil `release` der einzige
//! Abnehmer war; seit `version` einen Stand eintraegt und taggt, sind es zwei,
//! und der Aufruf ist an die Stelle gewandert, die beide gemeinsam haben. Eine
//! zweite waere die zweite Wahrheit darueber, wie tief das Bauwerkzeug in den
//! Zustand des Arbeitsbaums schaut — und seit das Werkzeug auch schreibt, die
//! zweite Wahrheit darueber, was es schreiben darf.
//!
//! **Seit dem 260821 kommt keine nackte Wortliste mehr hier an.** [`rufen`]
//! nimmt einen [`Auftrag`], und `Auftrag` ist die vollstaendige Aufzaehlung
//! jedes Kommandos, das dieses Werkzeug an `git` reicht. Wer ein neues braucht,
//! kann es nicht danebenbauen: er bekommt es nur durch diese Tuer, und die Tuer
//! ist eine Variante. [`Auftrag::worte`] und [`Auftrag::wirkung`] sind beide
//! vollstaendige Fallunterscheidungen ohne Auffangzweig, also haelt der
//! Uebersetzer eine neue Variante an, bis sie ihre Woerter genannt und sich als
//! lesend oder schreibend eingeordnet hat — dieselbe Bauart, die dieses Projekt
//! fuer `Wirkungsbereich`, `Bereich` und `Fokus` fuehrt.
//!
//! **Bis dahin stand die Aufsicht daneben statt auf dem Weg.** Sie zaehlte drei
//! Bauer namentlich auf, und ein vierter — `git tag --list` — stand schon
//! daneben, ohne dass sie ihn las. Was eine Aufzaehlung von Namen nicht kann,
//! ist die Zusage tragen, dass sie vollstaendig ist. Sie steht deshalb jetzt in
//! [`aufsichtsbefund`], und [`rufen`] ruft sie vor jedem Prozessaufruf: gelesen
//! wird die Liste, die wirklich hinausgeht, und nicht die, an die jemand
//! gedacht hat.
//!
//! **Wie stark die Zusage danach ist, in drei Saetzen.** Der Uebersetzer haelt,
//! dass jedes Kommando eine Variante ist und dass jede Variante ihre Woerter und
//! ihre Wirkung nennt. Die Aufsicht auf dem Weg haelt, dass keine Liste — auch
//! die einer ungeprueften neuen Variante — einen fremden Unterbefehl, eine Marke
//! aus [`MARKEN`], eine `--force`-Form, eine kurze Gewaltmarke oder einen
//! erzwingenden Verweis mit `+` traegt. Was **nichts** haelt, ist ein zweiter
//! Prozessaufruf an [`rufen`] vorbei; das haelt weiterhin allein die Probe
//! `xtask_ruft_git_an_genau_einer_stelle`, und der Uebersetzer haelt es nicht.
//!
//! **Lesen und Schreiben stehen hier verschieden da, und das ist Absicht.** Die
//! Unterscheidung traegt [`Wirkung`], und sie ist nicht bloss Beschriftung: die
//! Aufsicht laesst zu einer lesenden Frage nur die lesenden Unterbefehle durch
//! und verlangt bei `tag` die Marke, die aus dem Anlegen eine Frage macht.
//! `git tag v0.2.0` als Frage getarnt kommt damit nicht hinaus.
//!
//! Die zwei kleinen Lesehilfen [`geaenderte_dateien`] und [`tag_steht`] stehen
//! ebenfalls hier und nicht bei ihren Abnehmern: `release` und `version`
//! stellen dieselben zwei Fragen, und zwei Auslegungen derselben Ausgabe waeren
//! zwei Antworten darauf, was ein sauberer Arbeitsbaum ist.

use std::path::Path;
use std::process::Command;

use crate::Abbruch;

/// Was ein Auftrag am Zustand aendert.
///
/// Jede Variante von [`Auftrag`] ordnet sich hier ein, und die Einordnung ist
/// nicht Beschriftung, sondern wird gelesen: [`aufsichtsbefund`] laesst zu
/// [`Wirkung::Liest`] nur [`LESENDE`] durch und zu [`Wirkung::Schreibt`] nur
/// [`SCHREIBENDE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Wirkung {
    /// Der Auftrag fragt und laesst den Baum, wie er ist.
    Liest,
    /// Der Auftrag aendert etwas — am Verzeichnis oder auf der Gegenseite.
    Schreibt,
}

/// Jedes Kommando, das dieses Werkzeug an `git` reicht.
///
/// **Die Aufzaehlung ist die Aufsicht.** [`rufen`] nimmt nichts anderes
/// entgegen, also gibt es keine Liste, die an dieser Aufzaehlung vorbei bei
/// `git` ankommt. Ein achter Auftrag ist eine achte Variante, und beide
/// Fallunterscheidungen darunter haben keinen Auffangzweig: der Bau steht, bis
/// die neue Variante ihre Woerter genannt und sich eingeordnet hat.
///
/// **Die Bauer standen bis zum 260821 bei ihren Abnehmern**, `git tag` und
/// `git commit` in `version`, `git push` in `veroeffentlichung`. Sie stehen
/// jetzt hier, weil die Aufsicht hier steht; was bei den Abnehmern bleibt, ist
/// die Entscheidung, den Auftrag zu erteilen, und deren Begruendung. Die
/// Begruendung der einzelnen Woerter steht an der Variante, die sie traegt.
pub(crate) enum Auftrag<'a> {
    /// `git rev-parse --git-dir`: liegt ueberhaupt ein Git-Verzeichnis vor?
    ///
    /// Steht getrennt, damit die Antwort darauf nicht am Wortlaut einer
    /// Fehlermeldung der anderen Fragen haengt.
    Verzeichnis,
    /// `git tag --points-at HEAD`: welche Tags stehen auf HEAD?
    ///
    /// `--points-at` fragt allein; `git tag` legt erst dann einen Tag an, wenn
    /// ein Name dabeisteht. Annotierte und leichte Tags stehen in dieser
    /// Ausgabe gleich, und das ist die Zusage aus C3.3: gefragt ist, welcher
    /// Name auf HEAD steht, und nicht, wie er entstanden ist.
    TagsAufHead,
    /// `git status --porcelain --untracked-files=no`: welche verfolgten Dateien
    /// sind geaendert?
    ///
    /// `--porcelain` meldet vorgemerkte und nicht vorgemerkte Aenderungen in
    /// derselben Form und fuehrt geloeschte verfolgte Dateien mit;
    /// `--untracked-files=no` haelt unbeachtete Dateien draussen, wie es der
    /// Entscheid vom 260813-1010 verlangt.
    ///
    /// **Ohne Pfadfilter, und das ist eine Festlegung.** Gezaehlt wird das
    /// ganze Verzeichnis. Eine Liste der bauwirksamen Ordner muesste jemand
    /// pflegen, und sie zu ergaenzen zu vergessen ist die zweite Art, eine
    /// Pruefung im Vorbeigehen zu verlieren — dieselbe Erwaegung, die schon bei
    /// `release::GRENZWURZEL` steht. Was der Verzicht kostet, steht in
    /// `shared/issues/260813-1515_*_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md`;
    /// beide Abnehmer zaehlen die betroffenen Dateien deshalb nicht nur, sie
    /// nennen sie beim Namen.
    Stand,
    /// `git tag --list <name>`: steht dieser Tag irgendwo im Verzeichnis?
    ///
    /// `--list` ist das, was den Aufruf zu einer Frage macht. Ohne die Marke
    /// legte derselbe Aufruf den Tag an, und die Aufsicht laesst ihn deshalb
    /// als [`Wirkung::Liest`] nur mit ihr durch.
    Tagliste(&'a str),
    /// `git tag <name>`: der leichte Tag auf HEAD.
    ///
    /// Leicht und nicht annotiert, wie `v0.1.0` vom 260813, den der Nutzer von
    /// Hand gesetzt hat. Die Frage nach den Tags auf HEAD unterscheidet die
    /// beiden Arten nicht; zwei Arten nebeneinander waeren trotzdem zwei
    /// Schreibweisen fuer dieselbe Sache.
    ///
    /// **Ohne `-f`.** Ein bestehender Tag laesst diesen Aufruf scheitern, und
    /// das ist die Absicht: `version::vorhaben_bestimmen` hat den Fall vorher
    /// entschieden.
    TagSetzen(&'a str),
    /// `git commit --only -m <meldung> -- <dateien>`: der Eintrag einer
    /// benannten Aenderung.
    ///
    /// **`--only` mit Pfaden und kein `git add`.** Der Eintrag entsteht aus dem
    /// Stand der genannten Dateien im Arbeitsbaum, ohne die Vormerkung
    /// anzufassen. Das hat zwei Wirkungen: ein gescheiterter Eintrag laesst
    /// nichts Vorgemerktes zurueck, das jemand wegraeumen muesste, und der Lauf
    /// greift nicht auf die gemeinsame Vormerkung zu, an der in diesem Projekt
    /// auch Agenten arbeiten.
    ///
    /// **Die Dateien kommen herein und stehen nicht hier.** Welche zwei der
    /// Versionsschritt eintraegt, ist seine Sache und steht als
    /// `version::EINGETRAGENE` dort; hierher gehoert allein die Form des
    /// Kommandos.
    Eintrag {
        /// Die Eintragsmeldung, ein Wort hinter `-m`.
        meldung: &'a str,
        /// Die Pfade hinter dem Trenner `--`.
        dateien: &'a [&'a str],
    },
    /// `git push origin HEAD <verweis>`: das Schieben zur Gegenseite.
    ///
    /// Genau vier Woerter. **Geschoben wird `HEAD` und nicht der Zweigname**,
    /// damit keine vierte lesende Frage nach `git` noetig wird; `HEAD` als
    /// Quellreferenz schreibt auf der Gegenseite in den Zweig gleichen Namens.
    ///
    /// **Es ist ein Auftrag und nicht zwei.** Zwei haetten einen
    /// Zwischenzustand, in dem der Zweig oben steht und der Tag nicht; und eine
    /// Liste, die beide Referenzen traegt, ist an einer Stelle nachzusehen
    /// statt an zweien.
    ///
    /// Der Verweis kommt fertig herein und wird nicht hier gefuegt: gefuegt
    /// wird in `veroeffentlichung::tagverweis`, bei dem, der schiebt.
    Schub {
        /// Der vollstaendige Verweis auf den Tag, `refs/tags/<name>`.
        verweis: &'a str,
    },
}

impl<'a> Auftrag<'a> {
    /// Die Woerter hinter `git`, in ihrer Reihenfolge.
    ///
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine neue Variante
    /// haelt hier den Bau an.
    #[must_use]
    pub(crate) fn worte(&self) -> Vec<&'a str> {
        match self {
            Auftrag::Verzeichnis => vec!["rev-parse", "--git-dir"],
            Auftrag::TagsAufHead => vec!["tag", "--points-at", "HEAD"],
            Auftrag::Stand => vec!["status", "--porcelain", "--untracked-files=no"],
            Auftrag::Tagliste(name) => vec!["tag", "--list", name],
            Auftrag::TagSetzen(name) => vec!["tag", name],
            Auftrag::Eintrag { meldung, dateien } => {
                let mut worte = vec!["commit", "--only", "-m", meldung, "--"];
                worte.extend_from_slice(dateien);
                worte
            }
            Auftrag::Schub { verweis } => vec!["push", "origin", "HEAD", verweis],
        }
    }

    /// Ob dieser Auftrag liest oder schreibt.
    ///
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine neue Variante
    /// haelt auch hier den Bau an und muss sich einordnen. Die Einordnung wird
    /// gelesen, siehe [`aufsichtsbefund`].
    #[must_use]
    pub(crate) fn wirkung(&self) -> Wirkung {
        match self {
            Auftrag::Verzeichnis | Auftrag::TagsAufHead | Auftrag::Stand | Auftrag::Tagliste(_) => {
                Wirkung::Liest
            }
            Auftrag::TagSetzen(_) | Auftrag::Eintrag { .. } | Auftrag::Schub { .. } => {
                Wirkung::Schreibt
            }
        }
    }
}

/// Die Unterbefehle, die eine lesende Frage tragen darf.
///
/// Eine Erlaubnisliste und keine Verbotsliste: sie faellt zur sicheren Seite.
/// Ein Unterbefehl, an den niemand gedacht hat, steht nicht darin und kommt
/// deshalb nicht durch, statt durchzukommen, weil niemand ihn verboten hat.
const LESENDE: [&str; 3] = ["rev-parse", "tag", "status"];

/// Die Unterbefehle, die ein schreibender Auftrag tragen darf.
///
/// Ebenfalls eine Erlaubnisliste. `reset`, `clean`, `checkout`, `restore` und
/// `stash` stehen nicht darin und kommen deshalb nicht vor, ohne dass sie
/// jemand einzeln verbieten muesste.
const SCHREIBENDE: [&str; 3] = ["tag", "commit", "push"];

/// Die Marken, die einem Kommando Reichweite oder Gewalt hinzufuegen.
///
/// Verglichen wird das ganze Wort. Die `--force`-Familie steht daneben und
/// nicht hier, weil sie Formen mit Anhang hat — `--force-with-lease`,
/// `--force-if-includes`, `--force-with-lease=<verweis>` —, die ein Vergleich
/// auf Gleichheit nicht faengt; sie wird deshalb am Wortanfang geprueft.
const MARKEN: [&str; 7] = [
    "--tags",
    "--follow-tags",
    "--all",
    "--mirror",
    "--delete",
    "--prune",
    "--amend",
];

/// Der Anfang, an dem jede Form von `--force` erkannt wird.
const GEWALTANFANG: &str = "--force";

/// Weitere lange Marken, die eine Pruefung uebergehen.
const UEBERGEHENDE: [&str; 1] = ["--no-verify"];

/// Die Buchstaben, die in einer kurzen Marke Gewalt oder Reichweite bedeuten.
///
/// Gemeint sind die Gruppen mit einem Strich: `-f`, `-d`, `-a` — und `-fd`,
/// denn `git` nimmt kurze Marken auch zusammengezogen entgegen. Geprueft wird
/// deshalb Buchstabe fuer Buchstabe und nicht das Wort als Ganzes; ein
/// Vergleich auf Gleichheit liesse `-fd` durch. `-m` bleibt zulaessig, weil `m`
/// nicht darunter steht.
const GEWALTBUCHSTABEN: [char; 3] = ['f', 'd', 'a'];

/// Die Aufsicht ueber ein Kommando: was daran nicht hinausgehen darf.
///
/// **Sie steht auf dem Weg und nicht daneben.** [`rufen`] ruft sie vor jedem
/// Prozessaufruf mit der Liste, die wirklich hinausgeht. Bis zum 260821 stand
/// an ihrer Stelle eine Probe, die drei Bauer namentlich aufzaehlte; eine
/// Aufzaehlung von Namen kann aber nicht zusagen, dass sie vollstaendig ist,
/// und sie war es auch nicht. Hier kommt nichts vorbei.
///
/// Vier Fragen, in dieser Reihenfolge:
///
/// 1. Es steht ueberhaupt ein Unterbefehl da.
/// 2. Der Unterbefehl steht in der Erlaubnisliste, die zur [`Wirkung`] gehoert.
///    Damit ist `push` an einem lesenden Auftrag ausgeschlossen und `reset` an
///    jedem.
/// 3. Ein lesendes `tag` traegt die Marke, die aus dem Anlegen eine Frage
///    macht. `git tag v0.2.0` als Frage getarnt kommt so nicht hinaus.
/// 4. Kein Wort hinter dem Unterbefehl traegt Gewalt: keine Marke aus
///    [`MARKEN`], keine `--force`-Form, keine kurze Gruppe mit einem Buchstaben
///    aus [`GEWALTBUCHSTABEN`], keine Marke aus [`UEBERGEHENDE`] und kein
///    Verweis mit fuehrendem `+`, der ohne jede Marke erzwingt.
///
/// **Was sie nicht kann.** Der vierte Punkt ist eine Verbotsliste und damit
/// nie beweisbar vollstaendig; die Punkte 2 und 3 sind Erlaubnislisten und
/// fallen zur sicheren Seite. Ein Auftrag, der mit erlaubtem Unterbefehl und
/// ohne Marke etwas Unerwuenschtes tut, kommt durch — was ihn haelt, ist die
/// Aufzaehlung [`Auftrag`] und der Blick dessen, der eine Variante hinzufuegt.
fn aufsichtsbefund(wirkung: Wirkung, worte: &[&str]) -> Option<String> {
    let Some(unterbefehl) = worte.first() else {
        return Some("es steht gar kein Unterbefehl da".to_owned());
    };
    let erlaubte: &[&str] = match wirkung {
        Wirkung::Liest => &LESENDE,
        Wirkung::Schreibt => &SCHREIBENDE,
    };
    if !erlaubte.contains(unterbefehl) {
        return Some(format!(
            "der Unterbefehl {unterbefehl} steht nicht in der Erlaubnisliste {erlaubte:?}"
        ));
    }
    if wirkung == Wirkung::Liest
        && *unterbefehl == "tag"
        && !worte[1..].contains(&"--points-at")
        && !worte[1..].contains(&"--list")
    {
        return Some(
            "git tag ohne --points-at und ohne --list legt einen Tag an, statt zu fragen"
                .to_owned(),
        );
    }
    for wort in &worte[1..] {
        if let Some(befund) = gewaltbefund(wort) {
            return Some(befund);
        }
    }
    None
}

/// Ob ein einzelnes Wort hinter dem Unterbefehl Gewalt oder Reichweite traegt.
///
/// Die innerste Haelfte der Aufsicht, getrennt, weil ihre vier Faelle sich
/// einzeln nachsehen lassen.
fn gewaltbefund(wort: &str) -> Option<String> {
    if MARKEN.contains(&wort) {
        return Some(format!("die Marke {wort} erweitert die Reichweite"));
    }
    if UEBERGEHENDE.contains(&wort) {
        return Some(format!("die Marke {wort} uebergeht eine Pruefung"));
    }
    if wort.starts_with(GEWALTANFANG) {
        return Some(format!("die Marke {wort} erzwingt"));
    }
    if let Some(buchstaben) = kurze_marke(wort)
        && let Some(gewalt) = buchstaben
            .chars()
            .find(|zeichen| GEWALTBUCHSTABEN.contains(zeichen))
    {
        return Some(format!(
            "die kurze Marke {wort} traegt -{gewalt} und erzwingt oder erweitert"
        ));
    }
    if wort.starts_with('+') {
        return Some(format!(
            "der Verweis {wort} erzwingt mit seinem fuehrenden + und ohne jede Marke"
        ));
    }
    None
}

/// Die Buchstaben einer kurzen Markengruppe, oder `None`.
///
/// Eine kurze Gruppe ist ein Strich und danach nur Buchstaben: `-f`, `-fd`,
/// `-m`. Der Trenner `--`, jede lange Marke und jeder Pfad fallen heraus.
fn kurze_marke(wort: &str) -> Option<&str> {
    let ohne_strich = wort.strip_prefix('-')?;
    if ohne_strich.is_empty()
        || !ohne_strich
            .chars()
            .all(|zeichen| zeichen.is_ascii_alphabetic())
    {
        return None;
    }
    Some(ohne_strich)
}

/// Die Meldung, wenn die Aufsicht ein Kommando anhaelt.
///
/// Sie schreibt die ganze Liste aus, denn wer sie liest, sucht die Stelle, an
/// der sie gebaut wird, und nicht die Aufsicht.
#[must_use]
fn aufsichtsmeldung(worte: &[&str], befund: &str) -> String {
    format!(
        "Die Aufsicht haelt das Kommando `git {}` an: {befund}.\n\
         \n\
         Dieses Werkzeug reicht an git nur Kommandos, die als Variante von git::Auftrag \
         dastehen und die Aufsicht in git.rs bestehen. Wer ein weitergehendes braucht, \
         entscheidet es dort und nicht im Vorbeigehen.\n\
         \n\
         Es wird nichts ausgefuehrt.",
        worte.join(" ")
    )
}

/// Ruft `git` im Projektverzeichnis und liefert seine Standardausgabe.
///
/// Nach dem Muster von `security_fragen` in `sign`: absoluter Pfad, weil der
/// Baum jedes Systemwerkzeug so ruft, `.current_dir` auf die Projektwurzel,
/// weil die Antwort sonst am Arbeitsverzeichnis des Aufrufers haengt.
/// Startfehler und ein Rueckgabewert ungleich null werden beide zum
/// Laufabbruch.
///
/// **Vor dem Prozessaufruf steht die Aufsicht.** Sie liest die Woerter des
/// Auftrags, nicht seinen Namen; ein Auftrag, den niemand nachgesehen hat,
/// kommt damit trotzdem nicht als Gewalt hinaus.
pub(crate) fn rufen(wurzel: &Path, auftrag: &Auftrag<'_>) -> Result<String, Abbruch> {
    let worte = auftrag.worte();
    if let Some(befund) = aufsichtsbefund(auftrag.wirkung(), &worte) {
        return Err(Abbruch::Lauf(aufsichtsmeldung(&worte, &befund)));
    }
    let ausgabe = Command::new("/usr/bin/git")
        .args(&worte)
        .current_dir(wurzel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("git laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "git {} ist gescheitert ({}): {}",
            worte.join(" "),
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&ausgabe.stdout).into_owned())
}

/// Die geaenderten verfolgten Dateien aus der Ausgabe von [`Auftrag::Stand`].
///
/// Eine Datei je Zeile, samt der zweistelligen Zustandsspalte, die
/// `--porcelain` voranstellt. Leerzeilen fallen weg; der Rest bleibt so
/// stehen, wie `git` ihn schreibt, damit die Meldung die Datei so nennt, wie
/// der Nutzer sie im naechsten `git status` wiederfindet.
pub(crate) fn geaenderte_dateien(ausgabe: &str) -> Vec<&str> {
    ausgabe
        .lines()
        .map(str::trim_end)
        .filter(|zeile| !zeile.trim().is_empty())
        .collect()
}

/// Steht `erwartet` in der Ausgabe von [`Auftrag::TagsAufHead`]?
///
/// Verglichen wird die ganze Zeile und nicht ihr Anfang: sonst deckte
/// `v0.1.0-rc1` die Auslieferung von `0.1.0`.
pub(crate) fn tag_steht(tags_auf_head: &str, erwartet: &str) -> bool {
    tags_auf_head.lines().any(|zeile| zeile.trim() == erwartet)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die zwei Dateien, die der Eintrag des Versionsschritts traegt.
    const BEISPIELDATEIEN: [&str; 2] = ["Cargo.toml", "Cargo.lock"];

    /// Jeder Auftrag einmal, mit Beispielwerten.
    ///
    /// **Diese Liste ist nicht die Zusage, sondern ihre Vorwegnahme.** Was
    /// haelt, dass kein Auftrag Gewalt traegt, ist [`aufsichtsbefund`] auf dem
    /// Weg selbst; diese Liste laesst den Ausfall bei `cargo test` geschehen
    /// statt beim Auslieferungslauf. Bleibt eine neue Variante hier stehen,
    /// faengt die Aufsicht sie trotzdem — nur eben spaeter.
    fn beispiele() -> Vec<Auftrag<'static>> {
        vec![
            Auftrag::Verzeichnis,
            Auftrag::TagsAufHead,
            Auftrag::Stand,
            Auftrag::Tagliste("v0.2.0"),
            Auftrag::TagSetzen("v0.2.0"),
            Auftrag::Eintrag {
                meldung: "chore(release): die Version steht auf 0.2.0",
                dateien: &BEISPIELDATEIEN,
            },
            Auftrag::Schub {
                verweis: "refs/tags/v0.2.0",
            },
        ]
    }

    /// Kein Auftrag dieses Werkzeugs traegt Gewalt.
    #[test]
    fn die_aufsicht_laesst_jeden_auftrag_durch() {
        for auftrag in beispiele() {
            let worte = auftrag.worte();
            assert_eq!(
                aufsichtsbefund(auftrag.wirkung(), &worte),
                None,
                "{worte:?} kommt nicht durch"
            );
        }
    }

    /// Die drei Fragen des Auslieferungswegs lesen, jede einzeln nachgesehen.
    ///
    /// **Der Name bleibt, und der Gegenstand auch.** Das Abnahmekriterium C3.7
    /// der Runde „Artefakt und Release" nennt diese Probe beim Namen und
    /// verlangt, dass sie unveraendert gruen laeuft; sie steht deshalb weiter
    /// da, obwohl [`aufsichtsbefund`] dieselbe Frage seit dem 260821 auf dem
    /// Weg selbst stellt. Was sich geaendert hat, ist die Form der drei Fragen:
    /// sie sind Varianten von [`Auftrag`] statt dreier Konstanten.
    ///
    /// `tag` steht in [`SCHREIBENDE`] und trotzdem in [`LESENDE`], weil es
    /// beides kann: mit `--points-at` fragt es, mit einem Namen legt es an.
    /// Genau diese Unterscheidung prueft der zweite Teil.
    #[test]
    fn keine_der_drei_fragen_schreibt() {
        for frage in [Auftrag::Verzeichnis, Auftrag::TagsAufHead, Auftrag::Stand] {
            assert_eq!(frage.wirkung(), Wirkung::Liest);
            let worte = frage.worte();
            assert_eq!(aufsichtsbefund(Wirkung::Liest, &worte), None, "{worte:?}");
            assert!(LESENDE.contains(&worte[0]), "{worte:?}");
            if worte[0] == "tag" {
                assert!(
                    worte.contains(&"--points-at"),
                    "git tag ohne --points-at legt einen Tag an: {worte:?}"
                );
                assert!(
                    worte
                        .iter()
                        .all(|wort| wort.starts_with("--") || *wort == "tag" || *wort == "HEAD"),
                    "ein Name hinter git tag legt ihn an: {worte:?}"
                );
            }
        }
    }

    /// Die sieben Auftraege, Wort fuer Wort.
    ///
    /// Bis zum 260821 stand diese Nachschau in `version::tests` und las drei
    /// Bauer namentlich; sie steht jetzt dort, wo die Bauer stehen, und liest
    /// alle sieben.
    #[test]
    fn die_auftraege_stehen_wort_fuer_wort() {
        assert_eq!(Auftrag::Verzeichnis.worte(), ["rev-parse", "--git-dir"]);
        assert_eq!(Auftrag::TagsAufHead.worte(), ["tag", "--points-at", "HEAD"]);
        assert_eq!(
            Auftrag::Stand.worte(),
            ["status", "--porcelain", "--untracked-files=no"]
        );
        assert_eq!(
            Auftrag::Tagliste("v0.2.0").worte(),
            ["tag", "--list", "v0.2.0"]
        );
        assert_eq!(Auftrag::TagSetzen("v0.2.0").worte(), ["tag", "v0.2.0"]);
        assert_eq!(
            Auftrag::Eintrag {
                meldung: "eine Meldung",
                dateien: &BEISPIELDATEIEN,
            }
            .worte(),
            [
                "commit",
                "--only",
                "-m",
                "eine Meldung",
                "--",
                "Cargo.toml",
                "Cargo.lock"
            ]
        );
        assert_eq!(
            Auftrag::Schub {
                verweis: "refs/tags/v0.2.0"
            }
            .worte(),
            ["push", "origin", "HEAD", "refs/tags/v0.2.0"]
        );
    }

    /// Genau drei Auftraege schreiben, und `push` ist einer davon.
    ///
    /// Die Teilung ist trennscharf und ohne Ausnahmeliste: `push` ist an der
    /// einen Stelle erlaubt, weil dort ein Auftrag `push` **ist**, und an jeder
    /// lesenden Frage ausgeschlossen, weil `push` nicht in [`LESENDE`] steht.
    #[test]
    fn lesen_und_schreiben_sind_getrennt() {
        let schreibende: Vec<Vec<&str>> = beispiele()
            .iter()
            .filter(|auftrag| auftrag.wirkung() == Wirkung::Schreibt)
            .map(Auftrag::worte)
            .collect();
        assert_eq!(schreibende.len(), 3, "{schreibende:?}");
        for worte in &schreibende {
            assert!(SCHREIBENDE.contains(&worte[0]), "{worte:?}");
        }
        for auftrag in beispiele() {
            if auftrag.wirkung() == Wirkung::Liest {
                assert!(!SCHREIBENDE.contains(&auftrag.worte()[0]) || auftrag.worte()[0] == "tag");
            }
        }
    }

    /// Die drei Lucken, die die alte Markenliste hatte.
    ///
    /// Keine dieser Listen baut heute jemand — genau darum geht es. Die
    /// Aufsicht ist fuer die Aenderung von morgen gebaut, und diese Probe
    /// stellt sie ihr.
    #[test]
    fn die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen() {
        for gewalt in [
            vec!["push", "origin", "-d", "refs/tags/v0.2.0"],
            vec!["tag", "-d", "v0.2.0"],
            vec!["push", "origin", "-fd", "HEAD"],
            vec!["push", "--force-with-lease", "origin", "HEAD"],
            vec![
                "push",
                "--force-with-lease=refs/heads/main",
                "origin",
                "HEAD",
            ],
            vec!["push", "--force-if-includes", "origin", "HEAD"],
            vec!["push", "--prune", "origin", "refs/tags/*"],
            vec!["push", "--follow-tags", "origin", "HEAD"],
        ] {
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &gewalt).is_some(),
                "{gewalt:?} kommt durch"
            );
        }
    }

    /// Ein Verweis mit fuehrendem `+` erzwingt ohne jede Marke.
    #[test]
    fn die_aufsicht_faengt_den_erzwingenden_verweis() {
        assert!(aufsichtsbefund(Wirkung::Schreibt, &["push", "origin", "+HEAD"]).is_some());
        assert!(
            aufsichtsbefund(
                Wirkung::Schreibt,
                &["push", "origin", "+refs/tags/v0.2.0:refs/tags/v0.2.0"]
            )
            .is_some()
        );
    }

    /// Ein fremder Unterbefehl kommt nicht durch, ohne dass ihn jemand einzeln
    /// verboten haette.
    ///
    /// Das ist der Unterschied zwischen einer Erlaubnisliste und einer
    /// Verbotsliste: die vier hier stehen nirgends als verboten da.
    #[test]
    fn die_aufsicht_faengt_jeden_fremden_unterbefehl() {
        for fremd in [
            vec!["reset", "--hard"],
            vec!["clean", "-x"],
            vec!["checkout", "."],
            vec!["stash"],
            vec!["update-ref", "refs/heads/main", "HEAD"],
        ] {
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &fremd).is_some(),
                "{fremd:?} kommt durch"
            );
            assert!(
                aufsichtsbefund(Wirkung::Liest, &fremd).is_some(),
                "{fremd:?} kommt als Frage durch"
            );
        }
        assert!(aufsichtsbefund(Wirkung::Liest, &[]).is_some());
    }

    /// Eine lesende Frage darf nicht schreiben.
    ///
    /// `git tag <name>` legt an; als [`Wirkung::Liest`] eingeordnet kommt es
    /// nicht hinaus, und `push` an einer Frage ebenso wenig.
    #[test]
    fn eine_lesende_frage_legt_keinen_tag_an() {
        assert!(aufsichtsbefund(Wirkung::Liest, &["tag", "v0.2.0"]).is_some());
        assert!(aufsichtsbefund(Wirkung::Liest, &["tag", "--list", "v0.2.0"]).is_none());
        assert!(aufsichtsbefund(Wirkung::Liest, &["tag", "--points-at", "HEAD"]).is_none());
        assert!(aufsichtsbefund(Wirkung::Liest, &["push", "origin", "HEAD"]).is_some());
    }

    /// `-m` bleibt zulaessig: `m` ist kein Gewaltbuchstabe.
    #[test]
    fn die_kurze_marke_des_eintrags_bleibt_zulaessig() {
        assert_eq!(kurze_marke("-m"), Some("m"));
        assert_eq!(kurze_marke("--"), None);
        assert_eq!(kurze_marke("--only"), None);
        assert_eq!(kurze_marke("Cargo.toml"), None);
        assert_eq!(gewaltbefund("-m"), None);
        assert!(gewaltbefund("-f").is_some());
        assert!(gewaltbefund("-df").is_some());
    }

    /// Unbeachtete Dateien bleiben aussen vor, und das haengt an der Marke.
    #[test]
    fn die_standabfrage_laesst_unbeachtete_dateien_aussen_vor() {
        let stand = Auftrag::Stand.worte();
        assert!(stand.contains(&"--untracked-files=no"), "{stand:?}");
        // Kein Pfadfilter: die Abfrage traegt kein `--` und keinen Pfad.
        assert!(
            !stand.contains(&"--"),
            "ein Pfadfilter waere eine Liste, die jemand pflegen muss: {stand:?}"
        );
    }

    #[test]
    fn leerzeilen_zaehlen_nicht_als_geaenderte_datei() {
        assert!(geaenderte_dateien("").is_empty());
        assert!(geaenderte_dateien("\n\n").is_empty());
        assert_eq!(
            geaenderte_dateien("M  Cargo.toml\n M README.md\n"),
            vec!["M  Cargo.toml", " M README.md"]
        );
    }

    #[test]
    fn ein_tag_gilt_nur_bei_ganzer_uebereinstimmung() {
        assert!(tag_steht("v0.1.0\n", "v0.1.0"));
        assert!(tag_steht("anderer\nv0.1.0\nnoch-einer\n", "v0.1.0"));
        assert!(!tag_steht("v0.1.0-rc1\nv0.1.10\n", "v0.1.0"));
        assert!(!tag_steht("", "v0.1.0"));
    }

    /// Die Meldung nennt das Kommando und den Befund.
    #[test]
    fn die_aufsichtsmeldung_nennt_kommando_und_befund() {
        let meldung =
            aufsichtsmeldung(&["push", "origin", "--force"], "die Marke --force erzwingt");
        assert!(meldung.contains("git push origin --force"), "{meldung}");
        assert!(meldung.contains("--force erzwingt"), "{meldung}");
        assert!(meldung.contains("Es wird nichts ausgefuehrt."), "{meldung}");
    }
}
