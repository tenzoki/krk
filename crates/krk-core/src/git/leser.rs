//! Der Gitleser: vier synchrone Auskuenfte ueber einem gehaltenen Repository.
//!
//! [`Gitleser::oeffnen`] findet das Repository und haelt es fest;
//! [`Gitleser::kopf`], [`Gitleser::verlauf`] und [`Gitleser::marken`] fragen es.
//! Alle drei sind **synchron** und kennen weder Faden noch Kanal: wer sie ruft,
//! ist dafuer verantwortlich, dass er nicht auf dem Hauptfaden steht. Der Lauf,
//! der das tut, ist `git/lauf.rs`.
//!
//! **Das Repository wird festgehalten und nicht je Frage neu geoeffnet**, und
//! das ist gemessen: das erste `index_or_empty()` auf einem Baum mit 100 000
//! Eintraegen kostet 36,7 ms, jedes weitere 3 bis 12 µs. `Repository::index()`
//! liest neu ein, sobald die Datei auf der Platte sich geaendert hat; ein
//! festgehaltenes Repository veraltet also nicht bei einem `git add` von aussen.
//!
//! # Ab welcher `gix`-Fassung die angesprochenen Wege stehen
//!
//! Alle gegen `gix` 0.87.1 geprueft, das die Wurzel-`Cargo.toml` auf `0.87`
//! festnagelt: `gix::discover`, `Repository::head`, `Repository::head_id`,
//! `Repository::rev_walk`, `Repository::find_commit`, `Repository::status`,
//! `Platform::into_iter`. Die Kiste fuehrt unter 1.0 keine Zusage ueber kleine
//! Fassungen; wer die Zahl in der `Cargo.toml` hebt, liest diese Liste nach.
//!
//! # Drei Voreinstellungen, die stehen bleiben, und warum
//!
//! **`bail_if_untrusted` bleibt auf `false`.** `gix` leitet die Vertrauensstufe
//! aus dem Eigentum am Pfad ab: gehoert das Verzeichnis dem laufenden Benutzer,
//! gilt die volle Stufe, sonst die reduzierte. In der reduzierten ueberliest
//! `gix` die empfindlichen Abschnitte der Konfiguration — darunter die Pfade zu
//! ausfuehrbaren Programmen, die ein Filtertreiber `filter.<name>.clean`
//! starten wuerde — statt den Zugriff zu verweigern. Fuer einen Dateimanager,
//! der beliebige Ordner betritt, fremde Wechselplatten und Heimatverzeichnisse
//! anderer Benutzer eingeschlossen, ist genau das die richtige Voreinstellung:
//! **ein fremdes Repository wird gelesen und nicht abgewiesen**, und seine
//! Konfiguration darf trotzdem nichts starten. Die Einstellung auf `true` zu
//! setzen hiesse, dem Nutzer in einem fremden Ordner eine Fehlermeldung statt
//! einer Auskunft zu zeigen (C6.7 der Runde 23).
//!
//! **`Platform::index_worktree_options_mut().thread_limit` wird nicht
//! gesetzt.** Ohne Deckel nimmt `gix` so viele Faeden, wie das Geraet Kerne hat,
//! und zwei Dateifenster koennen je einen Lauf halten; der Statuslauf kann dem
//! Zeichendurchgang damit Bilder wegnehmen. Ob das geschieht und woran die Zahl
//! zu messen waere, ist aus den Eingaben dieser Runde **nicht** entscheidbar:
//! die Wirkung haengt am Geraet, an der Zahl der Kerne und an der Groesse des
//! Baums, und keine dieser drei Groessen ist gemessen. Eine Zahl hier waere ein
//! Deckel, den niemand gemessen hat. Die Frage ist als Datensatz gefilt
//! (`circles/260830-1045-git-bereich-liest-status-branch-verlauf/decisions/260830-1317_*_wird-die-fadenzahl-von-gix-gedeckelt-und-woran-waere-die-zahl-zu-messen.md`);
//! **diese Zeile ist die Stelle, an der der Deckel spaeter steht**, und sie
//! steht hier namentlich, damit der Umbau eine Zeile bleibt und keine Bauform.
//!
//! **`Outcome::write_changes` wird nicht gerufen, und
//! `EntryStatus::NeedsUpdate` wird gelesen und verworfen.** Die Stufe A liest;
//! die Begruendung und der offene Datensatz stehen im Kopf von
//! [`crate::git`](super).
//!
//! # Ein Deskriptormangel entscheidet nichts
//!
//! `EMFILE` und `ENFILE` sprechen ueber die Deskriptortabelle des Prozesses und
//! nicht ueber den Pfad; jeder andere Fehler beim Oeffnen spricht ueber den
//! Pfad. Die Unterscheidung trifft [`crate::verzeichnis::sys::ist_deskriptormangel`]
//! und keine zweite Regel daneben; [`fehlerkette_meldet_deskriptormangel`]
//! sucht sie in der Fehlerkette von `gix`, weil die Kiste ihre `io::Error` in
//! eigene Fehler einwickelt.
//!
//! **Gemessen und nicht vermutet:** unter `ulimit -n 64` mit belegter
//! Deskriptortabelle scheitert `gix::discover` an einem echten Repository mit
//! „Could not obtain the current working directory", und die Fehlerkette traegt
//! `errno 24`. Wer diesen Fehlschlag als [`Oeffnung::KeinRepository`] ausgaebe,
//! machte aus einem Zustand des eigenen Prozesses die Auskunft „dieser Ordner
//! liegt in keinem Repository" — derselbe Defekt, den der Durchlauf mit
//! `260815-0211` einmal getragen hat. Er bekommt deshalb
//! [`Oeffnung::Unentschieden`], und die drei Fragefunktionen liefern in
//! derselben Lage `None`.

use std::error::Error;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use gix::Repository;
use gix::bstr::{BStr, BString, ByteSlice};

use super::{Commit, Kopf, Marke};
use crate::verzeichnis::sys::ist_deskriptormangel;

/// Wie viele Zeichen ein Kurzhash traegt.
///
/// Sieben, wie `git log --oneline` sie schreibt, und **fest** statt ueber
/// `ObjectId::shorten()`: jene Laenge haengt vom Bestand des Repositorys ab und
/// aendert sich mit dem naechsten Commit. Eine Spaltenbreite, die mit dem
/// Bestand wandert, waere in der Verlaufsliste eine Unruhe ohne Gegenwert.
const KURZHASHLAENGE: usize = 7;

/// Die Antwort auf die Frage, ob dieser Ordner in einem Repository liegt.
///
/// Drei Werte, und der dritte ist der Grund fuer die Aufzaehlung: ein
/// `Option<Gitleser>` koennte „hier ist keines" und „ich konnte es nicht
/// feststellen" nicht auseinanderhalten, und genau diese zwei zusammenzuziehen
/// ist der Fehler, den der Kopf dieses Moduls beschreibt.
///
/// **Der Leser steht hinter einem `Box`**, und das ist gemessen und nicht
/// vorsorglich: ein `gix::Repository` traegt 1 240 Bytes, die beiden anderen
/// Werte tragen nichts, und ohne die Einschachtelung waere jede `Oeffnung` auf
/// dem Stapel so gross wie das Repository. Die eine Belegung je Ordnerwechsel
/// steht neben den 346 bis 900 µs, die `discover` im positiven Fall ohnehin
/// kostet.
#[derive(Debug)]
pub enum Oeffnung {
    /// Der Ordner liegt in einem Repository, und hier ist sein Leser.
    Offen(Box<Gitleser>),
    /// Der Ordner liegt bis zur Wurzel in keinem Repository. **Entschieden.**
    KeinRepository,
    /// Es liess sich nicht feststellen, weil dem Prozess die Deskriptoren
    /// ausgegangen sind. **Nicht entschieden**; der Rufer meldet nichts.
    Unentschieden,
}

/// Ein festgehaltenes Repository und die vier Auskuenfte darueber.
pub struct Gitleser {
    repo: Repository,
}

/// Von Hand und nicht abgeleitet: `gix::Repository` traegt seinen ganzen
/// Zwischenspeicher mit sich, und eine abgeleitete Ausgabe waere seitenlang.
/// Was einen Leser unterscheidet, ist sein Arbeitsbaum.
impl std::fmt::Debug for Gitleser {
    fn fmt(&self, ausgabe: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ausgabe
            .debug_struct("Gitleser")
            .field("arbeitsbaum", &self.repo.workdir())
            .finish()
    }
}

impl Gitleser {
    /// Sucht von `ordner` aufwaerts nach einem Repository und haelt es fest.
    ///
    /// Der Weg ist `gix::discover` und nicht `gix::open`: ein Ordner **in**
    /// einem Repository ist nicht dasselbe wie ein Repository, und KRK zeigt
    /// weit oefter einen Unterordner als eine Wurzel (C3.10). Der negative Fall
    /// ist gemessen billig — 21 bis 82 µs an einem Pfad ohne `.git`, also unter
    /// einem Zweihundertstel eines Bildes —, und deshalb darf ein Ordnerwechsel
    /// die Frage synchron stellen (C6.5).
    ///
    /// Die Kosten steigen mit der Zahl der Ebenen bis zum Wurzelverzeichnis,
    /// weil `discover` je Ebene nach `.git` sieht.
    #[must_use = "die Antwort sagt, ob ueberhaupt ein Repository dasteht"]
    pub fn oeffnen(ordner: &Path) -> Oeffnung {
        match gix::discover(ordner) {
            Ok(repo) => Oeffnung::Offen(Box::new(Self { repo })),
            Err(fehler) if fehlerkette_meldet_deskriptormangel(&fehler) => Oeffnung::Unentschieden,
            Err(_) => Oeffnung::KeinRepository,
        }
    }

    /// Worauf HEAD steht.
    ///
    /// Drei Lagen, in dieser Reihenfolge gefragt, und die Reihenfolge ist
    /// tragend: **ungeboren zuerst**, weil ein ungeborener HEAD einen
    /// Branchnamen traegt und trotzdem keinen Commit hat. Danach abgeloest,
    /// zuletzt der Branch.
    ///
    /// `None` heisst unentschieden, nicht „kein Repository":
    /// [`Kopf::KeinRepository`] gehoert zu [`Oeffnung::KeinRepository`] und
    /// entsteht beim Rufer.
    #[must_use = "der Kopf ist die obere Zeile des Git-Bereichs"]
    pub fn kopf(&self) -> Option<Kopf> {
        let kopf = self.repo.head().ok()?;
        if kopf.is_unborn() {
            return Some(Kopf::OhneCommit(branchname(&kopf)?));
        }
        if kopf.is_detached() {
            let id = self.repo.head_id().ok()?;
            return Some(Kopf::Abgeloest(kurzhash(&id)));
        }
        Some(Kopf::Branch(branchname(&kopf)?))
    }

    /// Die naechsten `zahl` Commits, nachdem `bereits` viele uebergangen sind.
    ///
    /// **Jeder Schwung laeuft von HEAD los und ueberspringt, was schon
    /// dasteht**, so wie `git log --skip`. Der Nachschlag aus E12 setzt damit
    /// dort an, wo die Liste aufgehoert hat, ohne eine Zeile zu doppeln — und
    /// ohne einen Nebenzweig zu verlieren.
    ///
    /// **Die Zahl und nicht der letzte Commit, und der Grund ist tragend.** Ein
    /// Lauf, der beim zuletzt angezeigten Commit ansetzte, lieferte allein
    /// dessen **Vorfahren**; der letzte Eintrag eines Schwungs beherrscht den
    /// Graphen aber nicht. Wo mehrere Zweige nebeneinander in der
    /// Warteschlange stehen, faellt jeder Commit dauerhaft heraus, der beim
    /// Schwungende darin stand und kein Vorfahre des letzten angezeigten ist.
    /// Ein Lauf, der immer bei HEAD beginnt, kann das nicht: `rev_walk` gibt
    /// jeden erreichbaren Commit genau einmal aus, also zerlegen die Schwuenge
    /// ihn in Stuecke statt ihn zu beschneiden. Gemessen an einem
    /// Pruefrepository mit zwei Zweigen von je dreissig Commits und einer
    /// Zusammenfuehrung darueber: der Ansatz am letzten Commit sah 56 von 62.
    ///
    /// **Und der Preis ist die Wiederholung der uebersprungenen Schritte**, die
    /// der Lauf am Graphen und nicht am Objektspeicher zahlt: gelesen und
    /// zerlegt werden allein die `zahl` genommenen Commits.
    ///
    /// **Dieselbe Bauform traegt eine Sortierung, sobald eine gewaehlt ist**
    /// (`260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md`):
    /// ein `.sorting(…)` am Lauf gilt dann jedem Schwung gleich, weil jeder
    /// Schwung derselbe Lauf von HEAD aus ist.
    ///
    /// **Woran der Rufer erkennt, dass nichts mehr folgt** (C4.3): die Liste
    /// ist kuerzer als `zahl`. Ein eigenes Kennzeichen daneben waere eine
    /// zweite Quelle fuer dieselbe Auskunft; die Laenge sagt es schon.
    ///
    /// Ein Repository ohne Commit liefert `Some(Vec::new())` und nicht `None`:
    /// „es gibt keine Commits" ist eine **entschiedene** Antwort (A7).
    ///
    /// Der Verlauf ist repositoryweit und nicht auf einen Ordner beschraenkt
    /// (A4): eine Beschraenkung verlangte einen Vergleich je Commit.
    #[must_use = "der Verlauf ist die Liste in der Mitte des Git-Bereichs"]
    pub fn verlauf(&self, bereits: usize, zahl: usize) -> Option<Vec<Commit>> {
        let kopf = self.repo.head().ok()?;
        if kopf.is_unborn() {
            return Some(Vec::new());
        }
        let anfang = self.repo.head_id().ok()?.detach();
        let lauf = self.repo.rev_walk([anfang]).all().ok()?;

        let mut gefunden = Vec::with_capacity(zahl);
        for stand in lauf.skip(bereits).take(zahl) {
            let stand = stand.ok()?;
            let commit = self.repo.find_commit(stand.id).ok()?;
            gefunden.push(commit_lesen(&commit)?);
        }
        Some(gefunden)
    }

    /// Die Marken der Eintraege von `ordner`, ueber ihren Namen.
    ///
    /// **Der Status ist ueber die Pfadmuster von `into_iter` auf `ordner`
    /// beschraenkt** (E9, C7.7). Der Gewinn ist gemessen und gross, wo der
    /// Ordner klein ist: 12 ms statt 220 ms fuer einen Unterordner mit 500
    /// Eintraegen in einem Repository mit 100 000. Er verschwindet, wo der
    /// Ordner selbst der ganze Baum ist.
    ///
    /// **Der Name ist der des Eintrags im angezeigten Ordner und nicht der
    /// repositoryrelative Pfad.** Ein Befund tief im Unterbaum faellt damit auf
    /// den Ordner, ueber den er zu erreichen ist; anders traege ein Ordner nie
    /// eine Marke, und die Zusammenfassung zaehlte Namen, die in der Liste gar
    /// nicht stehen. Treffen dabei mehrere Marken auf denselben Eintrag,
    /// gewinnt die mit dem hoeheren [`Marke::rang`].
    ///
    /// **`status.showUntrackedFiles` wird ausdruecklich uebergangen.** `gix`
    /// respektiert den Schluessel, und ein Repository, das ihn auf `no` setzt,
    /// lieferte KRK keine einzige Marke `N`. Was die Spalte zeigt, soll nicht
    /// von der Konfiguration eines fremden Baums abhaengen; deshalb steht die
    /// Form hier fest.
    ///
    /// **Und zwar je Datei und nicht zusammengefasst, obwohl das mehr kostet.**
    /// Die zusammengefasste Form — die Voreinstellung von `gix` — zieht einen
    /// ganz unverfolgten Ordner zu **einem** Eintrag zusammen, und dieser eine
    /// Eintrag ist dann der angezeigte Ordner selbst: unter ihm bleibt kein
    /// Pfadteil mehr uebrig, dem eine Marke gaelte, und kein Eintrag der Liste
    /// bekaeme eine. Gemessen an einem Pruefrepository, dessen Unterordner
    /// vollstaendig unverfolgt ist: die zusammengefasste Form liefert eine leere
    /// Markenliste. Was die Wahl kostet, steht dazu: 10 000 unverfolgte Dateien
    /// kosten je Datei gemessen 18,8 ms; der Lauf ist nebenlaeufig, also kostet
    /// er keine Bildzeit.
    ///
    /// `None` heisst unentschieden. Jeder Fehlschlag fuehrt dorthin, und das
    /// ist keine Verkuerzung: ein halb gelesener Status ist von „diese
    /// Eintraege sind unveraendert" nicht zu unterscheiden, und die zweite
    /// Auskunft waere falsch.
    ///
    /// Ein Repository ohne Arbeitsbaum liefert `Some(Vec::new())`: dort gibt es
    /// keine Arbeitskopie, deren Zustand eine Marke haette, und das ist eine
    /// entschiedene Antwort.
    #[must_use = "die Marken sind der Befund, den das Ordnermodell nachtraegt"]
    pub fn marken(&self, ordner: &Path) -> Option<Vec<(String, Marke)>> {
        let Some(arbeitsbaum) = self.repo.workdir() else {
            return Some(Vec::new());
        };
        let praefix = repositorypfad(arbeitsbaum, ordner)?;

        let muster: Vec<BString> = if praefix.is_empty() {
            Vec::new()
        } else {
            vec![BString::from(praefix.clone())]
        };

        let strom = self
            .repo
            .status(gix::progress::Discard)
            .ok()?
            .untracked_files(gix::status::UntrackedFiles::Files)
            .index_worktree_submodules(gix::status::Submodule::Given {
                ignore: gix::submodule::config::Ignore::All,
                check_dirty: false,
            })
            .into_iter(muster)
            .ok()?;

        let mut gefunden: Vec<(String, Marke)> = Vec::new();
        for posten in strom {
            let posten = posten.ok()?;
            let Some((pfad, marke)) = posten_deuten(&posten) else {
                continue;
            };
            let Some(name) = eintragsname(pfad.as_ref(), &praefix) else {
                continue;
            };
            eintragen(&mut gefunden, name, marke);
        }
        Some(gefunden)
    }
}

/// Traegt `name` mit `marke` ein, ohne eine hoeherrangige Marke zu verdraengen.
///
/// Eine flache Liste und keine `HashMap`: ein Ordner traegt so viele Eintraege,
/// wie er Zeilen hat, und die Zahl der **markierten** darunter ist klein. Der
/// Rufer bekommt am Ende eine Liste in der Reihenfolge des ersten Auftretens.
fn eintragen(gefunden: &mut Vec<(String, Marke)>, name: String, marke: Marke) {
    match gefunden.iter_mut().find(|(steht, _)| *steht == name) {
        Some((_, bisher)) => {
            if marke.rang() > bisher.rang() {
                *bisher = marke;
            }
        }
        None => gefunden.push((name, marke)),
    }
}

/// Der Pfad von `ordner` relativ zum Arbeitsbaum, als Bytes mit `/` als
/// Trenner; leer, wenn `ordner` der Arbeitsbaum selbst ist.
///
/// Beide Seiten werden aufgeloest, bevor sie verglichen werden: auf macOS ist
/// `/tmp` eine Verknuepfung auf `/private/tmp`, und ohne die Aufloesung
/// scheiterte der Vergleich an jedem Pruefordner. `None` heisst unentschieden —
/// das Aufloesen kann selbst an einem Deskriptormangel scheitern.
fn repositorypfad(arbeitsbaum: &Path, ordner: &Path) -> Option<Vec<u8>> {
    let wurzel: PathBuf = arbeitsbaum.canonicalize().ok()?;
    let hier: PathBuf = ordner.canonicalize().ok()?;
    let relativ = hier.strip_prefix(&wurzel).ok()?;
    Some(relativ.as_os_str().as_bytes().to_vec())
}

/// Der Name des Eintrags im angezeigten Ordner, zu dem `rela_path` gehoert.
///
/// `rela_path` ist repositoryrelativ und traegt `/` als Trenner. `praefix` ist
/// der Ordner, ebenso repositoryrelativ, oder leer fuer die Wurzel. Uebrig
/// bleibt der erste Pfadteil darunter.
///
/// `None` heisst, dass der Pfad nicht unter dem Ordner liegt. Das kommt vor,
/// weil ein Pfadmuster wie `crates` auch `crates-alt` praefixweise treffen
/// kann; ein Eintrag, den der Ordner nicht traegt, wird uebergangen.
fn eintragsname(rela_path: &BStr, praefix: &[u8]) -> Option<String> {
    let rest = if praefix.is_empty() {
        rela_path.as_bytes()
    } else {
        let bytes = rela_path.as_bytes();
        if bytes.len() <= praefix.len() || !bytes.starts_with(praefix) {
            return None;
        }
        if bytes[praefix.len()] != b'/' {
            return None;
        }
        &bytes[praefix.len() + 1..]
    };
    let name = match rest.iter().position(|zeichen| *zeichen == b'/') {
        Some(stelle) => &rest[..stelle],
        None => rest,
    };
    if name.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(name).into_owned())
}

/// Welche Marke ein Posten des Statusstroms traegt, und zu welchem Pfad.
///
/// Die Fallunterscheidung ist vollstaendig ueber beide Haelften des Stroms:
/// `TreeIndex` vergleicht den Kopf-Baum mit dem Index und liefert damit das
/// **Vorgemerkte**, `IndexWorktree` vergleicht den Index mit der Arbeitskopie
/// und liefert das **Geaenderte**, das **Neue** und den **Konflikt**. Die
/// Umbenennung kommt aus beiden.
///
/// `None` heisst „dieser Posten traegt keine Marke". Drei Faelle fallen
/// darunter, und jeder aus einem eigenen Grund:
///
/// - **`EntryStatus::NeedsUpdate`**: der Eintrag hat sich nicht geaendert, nur
///   sein Stat-Zwischenspeicher waere aufzufrischen. Er wird gelesen und
///   verworfen; die Stufe A schreibt nicht (E8).
/// - **`Status::Ignored` und `Status::Pruned`** aus dem Verzeichnisdurchlauf:
///   ein ignorierter Eintrag ist kein neuer.
/// - **`Status::Tracked`**: der Durchlauf hat einen Eintrag gefunden, den der
///   Index schon fuehrt; seine Aenderung meldet die andere Haelfte.
fn posten_deuten(posten: &gix::status::Item) -> Option<(BString, Marke)> {
    use gix::status::index_worktree::Item as Arbeitskopie;
    use gix::status::plumbing::index_as_worktree::EntryStatus;

    match posten {
        gix::status::Item::TreeIndex(aenderung) => {
            let marke = match aenderung {
                gix::diff::index::Change::Rewrite { .. } => Marke::Umbenannt,
                gix::diff::index::Change::Addition { .. }
                | gix::diff::index::Change::Deletion { .. }
                | gix::diff::index::Change::Modification { .. } => Marke::Vorgemerkt,
            };
            Some((pfad_der_indexaenderung(aenderung), marke))
        }
        gix::status::Item::IndexWorktree(Arbeitskopie::Modification {
            rela_path, status, ..
        }) => {
            let marke = match status {
                EntryStatus::Conflict { .. } => Marke::Konflikt,
                EntryStatus::Change(_) => Marke::Geaendert,
                EntryStatus::IntentToAdd => Marke::Vorgemerkt,
                EntryStatus::NeedsUpdate(_) => return None,
            };
            Some((rela_path.clone(), marke))
        }
        gix::status::Item::IndexWorktree(Arbeitskopie::DirectoryContents { entry, .. }) => {
            match entry.status {
                gix::dir::entry::Status::Untracked => Some((entry.rela_path.clone(), Marke::Neu)),
                gix::dir::entry::Status::Ignored(_)
                | gix::dir::entry::Status::Pruned
                | gix::dir::entry::Status::Tracked => None,
            }
        }
        gix::status::Item::IndexWorktree(Arbeitskopie::Rewrite { dirwalk_entry, .. }) => {
            Some((dirwalk_entry.rela_path.clone(), Marke::Umbenannt))
        }
    }
}

/// Der Pfad, an dem eine Aenderung zwischen Kopf-Baum und Index steht.
///
/// Bei einer Umbenennung ist es das **Ziel** und nicht die Quelle: die Quelle
/// steht nach der Umbenennung nicht mehr in der Dateiliste, das Ziel schon.
fn pfad_der_indexaenderung(aenderung: &gix::diff::index::Change) -> BString {
    match aenderung {
        gix::diff::index::Change::Addition { location, .. }
        | gix::diff::index::Change::Deletion { location, .. }
        | gix::diff::index::Change::Modification { location, .. }
        | gix::diff::index::Change::Rewrite { location, .. } => location.as_ref().to_owned(),
    }
}

/// Der Branchname ohne den Vorsatz `refs/heads/`.
fn branchname(kopf: &gix::Head<'_>) -> Option<String> {
    Some(kopf.referent_name()?.shorten().to_string())
}

/// Die ersten [`KURZHASHLAENGE`] Zeichen eines Objektnamens.
fn kurzhash(id: &gix::Id<'_>) -> String {
    id.to_hex_with_len(KURZHASHLAENGE).to_string()
}

/// Ein `gix`-Commit als [`Commit`] dieses Moduls.
///
/// `None` heisst, dass eines der sechs Felder nicht zu lesen war; ein Commit,
/// von dem die Haelfte fehlte, waere eine Zeile, die nichts sagt.
fn commit_lesen(commit: &gix::Commit<'_>) -> Option<Commit> {
    let nachricht = commit.message_raw().ok()?.to_string();
    let kurzbeschreibung = commit.message().ok()?.summary().to_string();
    let autor = commit.author().ok()?;
    let zeit = systemzeit(autor.time().ok()?)?;
    Some(Commit {
        id: commit.id().detach(),
        kurzbeschreibung,
        nachricht,
        autor: autor.name.to_string(),
        email: autor.email.to_string(),
        zeit,
    })
}

/// Ein Zeitpunkt aus `gix` als [`SystemTime`].
///
/// `gix::date::Time::seconds` zaehlt ab dem Nullpunkt von 1970 und darf negativ
/// sein; die Kiste laesst ausdruecklich Daten davor zu. Die Zeitzone bleibt
/// aussen vor: `SystemTime` traegt keine, und die buergerliche Ortszeit rechnet
/// erst die Anzeige aus.
fn systemzeit(zeit: gix::date::Time) -> Option<SystemTime> {
    let sekunden = zeit.seconds;
    if sekunden >= 0 {
        SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(u64::try_from(sekunden).ok()?))
    } else {
        SystemTime::UNIX_EPOCH.checked_sub(Duration::from_secs(sekunden.unsigned_abs()))
    }
}

/// Ob irgendein Glied der Fehlerkette einen Deskriptormangel meldet.
///
/// `gix` wickelt seine `io::Error` in eigene Fehler ein, oft zwei Lagen tief;
/// die Kette wird deshalb ueber [`Error::source`] abgelaufen und jedes Glied
/// auf einen `io::Error` geprueft. Was ein Mangel ist, entscheidet
/// [`ist_deskriptormangel`] und nicht diese Funktion: eine zweite Liste von
/// Fehlernummern daneben liefe mit der ersten auseinander.
#[must_use]
pub fn fehlerkette_meldet_deskriptormangel(fehler: &(dyn Error + 'static)) -> bool {
    let mut glied = Some(fehler);
    while let Some(dieses) = glied {
        if let Some(io) = dieses.downcast_ref::<std::io::Error>()
            && ist_deskriptormangel(io)
        {
            return true;
        }
        glied = dieses.source();
    }
    false
}
