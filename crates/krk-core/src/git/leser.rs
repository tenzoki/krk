//! Der Gitleser: vier synchrone Auskuenfte ueber einem gehaltenen Repository.
//!
//! [`Gitleser::oeffnen`] findet das Repository und haelt es fest;
//! [`Gitleser::kopf`], [`Gitleser::verlauf`] und [`Gitleser::marken`] fragen es.
//! Alle drei sind **synchron** und kennen weder Faden noch Kanal: wer sie ruft,
//! ist dafuer verantwortlich, dass er nicht auf dem Hauptfaden steht. Der Lauf,
//! der das tut, ist `git/lauf.rs`. Die teuerste der drei nimmt daneben ein
//! Abbruchkennzeichen entgegen; ihr eigener Doc-Kommentar sagt, warum genau
//! sie.
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
//! `Repository::rev_walk`, `Repository::find_commit`,
//! `Repository::object_cache_size_if_unset`, `revision::walk::Platform::sorting`,
//! `Repository::index_worktree_status`, `Repository::tree_index_status`,
//! `Repository::pathspec`, `Repository::stat_options`,
//! `Repository::index_or_empty`, `Repository::head_tree_id_or_empty`,
//! `Repository::dirwalk_options`, `BuiltinSubmoduleStatus::new`, `CompareBlobs`,
//! `FastEq`, `VisitEntry`, `Stat::matches`, `Stat::from_fs`, `Stat::is_racy`,
//! `fs::Metadata::from_path_no_follow` und `State::timestamp`. Bis zum 261026
//! ging der Markenlauf ueber `Repository::status` und `Platform::into_iter`;
//! warum nicht mehr, sagt der Abschnitt „Eine Datei, die nur in der Cloud
//! liegt". Dazu die Aufzaehlungen, ueber die vollstaendig zerlegt wird:
//! `gix::discover::Error` und `gix::discover::upwards::Error` in
//! [`entschiedene_verneinung`], `Entry`, `EntryStatus` und `Change` aus
//! `gix-status` in `posten_deuten`: eine Variante mehr haelt dort den Bau an.
//! Die Kiste fuehrt unter 1.0 keine Zusage ueber kleine Fassungen; wer die Zahl
//! in der `Cargo.toml` hebt, liest diese Liste nach.
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
//! **`thread_limit` in den Optionen von `index_worktree_status` bleibt
//! `None`.** Ohne Deckel nimmt `gix` so viele Faeden, wie das Geraet Kerne hat,
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
//! **`Outcome::write_changes` wird nicht gerufen.** Das allein traegt die
//! Zusage der Stufe A; die Begruendung und der offene Datensatz stehen im Kopf
//! von [`crate::git`](super). Der Zweig fuer `EntryStatus::NeedsUpdate` in
//! `posten_deuten` traegt sie **nicht**: der Posten erreicht KRK seit dem
//! 261026 und bleibt dort ohne Marke, aber was ihn verwirft, ist allein, dass
//! nichts ihn zurueckschreibt. Der Weg ueber `index_worktree_status` liefert
//! ein `Outcome` ohne Schreibweg.
//!
//! # Eine Datei, die nur in der Cloud liegt
//!
//! **Der Markenlauf oeffnet keine ausgelagerte Datei** (`SF_DATALESS`,
//! [`crate::verzeichnis::ist_platzhalter`]); so verlangt es der Entscheid
//! `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`,
//! Moeglichkeit 1, fuer jeden Hintergrundleser. Bis zum 261026 tat er es: auf
//! einer Wegwerfkopie auf Google Drive stand ein Lauf ueber zehn Minuten in
//! `read_data::Stream::read_exact` und lud Datei fuer Datei herunter.
//!
//! **Die eine Stelle, an der `gix` den Inhalt einer verfolgten Datei liest**,
//! ist `compute_status` in `gix-status-0.34.1/src/index_as_worktree/function.rs`:
//! es stellt je Eintrag ein `lstat`, vergleicht mit `Stat::matches` und ruft
//! den Inhaltsvergleich `CompareBlobs::compare_blobs` allein, wenn die
//! stat-Angaben abweichen oder der Eintrag racy ist. Der Vergleich bekommt ein
//! `ReadData`, und erst dessen Aufruf oeffnet die Datei. `Platform::into_iter`
//! setzt den Vergleich fest auf `FastEq`, das bei gleicher Groesse liest und
//! hasht; [`Gitleser::marken_mit`] geht deshalb ueber die zwei oeffentlichen
//! Wege, aus denen jener sich zusammensetzt, und reicht `index_worktree_status`
//! einen eigenen Vergleich:
//!
//! ```text
//! gix: lstat, stat weicht ab oder racy
//!   -> Inhaltsvergleich: SF_DATALESS?
//!        nein -> FastEq wie bisher: Groesse, sonst lesen und hashen
//!        ja   -> Groesse weicht ab?
//!                  ja   -> Befund Geaendert, nichts gelesen        -> „M"
//!                  nein -> stat ohne ctime passt und nicht racy?
//!                            ja   -> kein Befund: unveraendert     -> keine Marke
//!                            nein -> Befund Ungelesen              -> „?"
//! ```
//!
//! Die Zweige sind disjunkt und vollstaendig. Die Groessenregel ist die von
//! `FastEq`, die stat-Pruefung ist `gix`s eigene `Stat::matches` mit den
//! Optionen des Repositorys und `trust_ctime: false`, dazu `Stat::is_racy`
//! gegen `State::timestamp`; eine zweite stat-Logik entsteht in KRK nicht.
//!
//! **Warum ctime bei einer ausgelagerten Datei nicht zaehlt**: gemessen am
//! 261026-1208 an drei ausgelagerten Dateien einer Wegwerfkopie auf Google
//! Drive (`stat -f` gegen `git ls-files --debug`) weicht nach dem Auslagern
//! allein ctime vom Index ab; mtime samt Nanosekunden, Groesse, Geraet, Inode,
//! `uid` und `gid` stimmen. Mit ctime saehe jede ausgelagerte Datei geaendert
//! aus. Der Nutzer hat entschieden, dass ctime fuer sie nicht zaehlt
//! (`261026-1208-zeigt-die-markenspalte-jede-ausgelagerte-datei-als-unbekannt-oder-nur-die-deren-stat-angaben-ausser-ctime-vom-index-abweichen.md`,
//! Moeglichkeit 2). Die Luecke, die das laesst, ist die von `git` mit
//! `core.trustctime=false`: eine Aenderung, die mtime, Groesse und Inode
//! unberuehrt laesst, sieht KRK an einer ausgelagerten Datei nicht. Fuer eine
//! lokale Datei bleibt ctime, wie das Repository es sagt.
//!
//! **Was dabei bleibt.** Der Provider kann eine Datei zwischen dem `lstat` des
//! Vergleichs und dem Oeffnen durch `FastEq` auslagern; dann wird gelesen und
//! heruntergeladen, einmal je solchem Wettlauf. Das ist hingenommen wie beim
//! Vorschaulesen im vorigen Paket. Der Verzeichnisdurchlauf liest daneben die
//! `.gitignore`-Dateien des Arbeitsbaums und ist darin nicht einstellbar.
//!
//! # Ein Fehlschlag beim Oeffnen entscheidet nur in einer Lage
//!
//! **Entschieden ist allein: bis zur Wurzel gesucht und nichts gefunden.**
//! Jeder andere Fehlschlag von `gix::discover` laesst die Frage offen, und
//! [`entschiedene_verneinung`] zerlegt sie dafuer ueber die Varianten der
//! beiden Fehleraufzaehlungen, vollstaendig und ohne Auffangzweig. Bis zum
//! 260831 stand es umgekehrt: der Auffangzweig war die Verneinung, und ein
//! defektes Repository, ein Rechteproblem oder ein `gitdir:`-Verweis ins Leere
//! kamen beim Nutzer als „Dieser Ordner liegt in keinem Git-Repository" an
//! (`260831-1444_*_jeder-fehlschlag-von-discover-ausser-dem-deskriptormangel-wird-als-kein-repository-ausgegeben.md`).
//!
//! **Zwei Lagen erreichen diese Unterscheidung nicht, weil `gix` sie vorher
//! zusammenzieht.** Ein `.git`-Verzeichnis, das sich nicht lesen laesst, und
//! ein `.git`, das als Datei einen `gitdir:`-Verweis ins Leere traegt, melden
//! beide `NoGitRepository`, und diese Variante traegt einen Pfad und keine
//! Ursache. Sie kommen deshalb weiter als entschiedene Verneinung beim Nutzer
//! an; der Datensatz ist
//! `260831-1652_*_gix-zieht-ein-unlesbares-git-verzeichnis-und-einen-toten-gitdir-verweis-selbst-zu-kein-repository-zusammen.md`,
//! und er traegt die Messung aller fuenf gemessenen Lagen.
//!
//! **Der Deskriptormangel braucht dafuer keine eigene Regel mehr, und das ist
//! kein Verzicht, sondern die Folge.** `EMFILE` und `ENFILE` sprechen ueber die
//! Deskriptortabelle des Prozesses und nicht ueber den Pfad; sie erreichen
//! `discover` als `io::Error`, und die drei Varianten, die einen solchen
//! ueberhaupt tragen koennen — `CurrentDir`, `CheckTrust` und
//! [`gix::discover::Error::Open`] —, sind samt und sonders unentschieden. Die
//! drei Varianten der entschiedenen Verneinung tragen dagegen nichts als Pfade.
//! Eine zweite Liste von Fehlernummern daneben liefe mit der ersten
//! auseinander; bis zum 260831 stand sie hier und wurde von der
//! Fallunterscheidung ueberholt.
//!
//! **Gemessen und nicht vermutet:** unter `ulimit -n 64` mit belegter
//! Deskriptortabelle scheitert `gix::discover` an einem echten Repository mit
//! „Could not obtain the current working directory", also an `CurrentDir`. Wer
//! diesen Fehlschlag als [`Oeffnung::KeinRepository`] ausgaebe, machte aus
//! einem Zustand des eigenen Prozesses die Auskunft „dieser Ordner liegt in
//! keinem Repository" — derselbe Defekt, den der Durchlauf mit `260815-0211`
//! einmal getragen hat. Er bekommt [`Oeffnung::Unentschieden`], und die drei
//! Fragefunktionen liefern in derselben Lage `None`; die Kindprobe
//! `kind_liest_unter_abgesenkter_deskriptorgrenze` misst es.

use std::fs::Metadata;
use std::ops::ControlFlow;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};

use gix::Repository;
use gix::bstr::{BStr, BString, ByteSlice};
use gix::revision::walk::Sorting;
use gix::status::plumbing::index_as_worktree::traits::{CompareBlobs, FastEq, ReadData};
use gix::status::plumbing::index_as_worktree::{Change, EntryStatus};
use gix::status::plumbing::index_as_worktree_with_renames::{Entry, VisitEntry};
use gix::traverse::commit::simple::CommitTimeOrder;

use super::{Commit, KURZHASHLAENGE, Kopf, Marke};

/// Wie gross der Objektzwischenspeicher eines geoeffneten Repositorys ist.
///
/// **Gemessen und nicht gesetzt.** Der Lauf ueber den Verlauf schlaegt jeden
/// Commit zweimal nach — einmal die Sortierung, um an seine Zeit zu kommen,
/// einmal [`Gitleser::verlauf`] selbst, um seine sechs Felder zu lesen —, und
/// `gix` nennt den Zwischenspeicher fuer genau diesen Fall (`gix-0.87.1`,
/// `src/revision/walk.rs:50-53`).
///
/// **Die Messung**, an KRKs eigenem Repository mit 800 Commits, je Messung ein
/// frisch geoeffneter Leser, sieben Messungen je Stand, Profil `release`, drei
/// Laeufe: der erste Schwung von fuenfzig Commits kostet im Mittel
///
/// | Stand | erster Schwung | tiefer Schwung (750 uebersprungen) |
/// |---|---|---|
/// | Graphenordnung, ohne Zwischenspeicher (bis 260831) | 4,1 bis 4,9 ms | 2,81 bis 2,83 ms |
/// | nach Zeit sortiert, ohne Zwischenspeicher | 5,9 bis 6,2 ms | 4,6 bis 4,8 ms |
/// | nach Zeit sortiert, mit diesem Zwischenspeicher | 2,5 bis 3,2 ms | 2,9 bis 3,6 ms |
///
/// Die Sortierung allein kostet also rund 1,8 ms, und der Zwischenspeicher gibt
/// mehr zurueck, als sie nimmt.
///
/// Ein Viertel Megabyte, und dieselbe Messung bei 64 KiB lieferte dasselbe:
/// gebraucht wird wenig, weil der Zwischenspeicher allein die Spanne zwischen
/// den beiden Nachschlaegen ueberbruecken muss. Er lebt so lange wie der Leser,
/// also die Dauer eines Laufs.
///
/// `object_cache_size_if_unset` und nicht `object_cache_size`: ein Repository,
/// das `gitoxide.objects.cacheLimit` selbst setzt, behaelt seine Zahl.
const OBJEKTSPEICHER: usize = 256 * 1024;

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
    /// Es liess sich nicht feststellen. **Nicht entschieden**; der Rufer meldet
    /// nichts.
    ///
    /// Hierher faellt jeder Fehlschlag, der nicht sagt „bis zur Wurzel gesucht
    /// und nichts gefunden": der Deskriptormangel des eigenen Prozesses ebenso
    /// wie ein gefundenes Repository, das sich nicht oeffnen liess.
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
    /// **Die entschiedene Verneinung haengt an der Fehlervariante und nicht an
    /// einer Fehlernummer**; [`entschiedene_verneinung`] traegt die
    /// Fallunterscheidung, und der Modulkopf sagt, warum sie so faellt.
    #[must_use = "die Antwort sagt, ob ueberhaupt ein Repository dasteht"]
    pub fn oeffnen(ordner: &Path) -> Oeffnung {
        match gix::discover(ordner) {
            Ok(mut repo) => {
                repo.object_cache_size_if_unset(OBJEKTSPEICHER);
                Oeffnung::Offen(Box::new(Self { repo }))
            }
            Err(fehler) if entschiedene_verneinung(&fehler) => Oeffnung::KeinRepository,
            Err(_) => Oeffnung::Unentschieden,
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
    /// **Die Reihenfolge ist die der Commit-Zeit und nicht die des Graphen.**
    /// `rev_walk` laeuft ohne `sorting` breitenzuerst, also in der Reihenfolge
    /// des Commit-Graphen; wo zwei Zweige nebeneinander in der Warteschlange
    /// stehen, greift er abwechselnd zu, und die Liste steht dann nicht nach
    /// Zeit. Ein Nutzer, der eine Verlaufsliste liest, erwartet die zeitliche
    /// Ordnung, und `git log` liefert sie ab Werk.
    ///
    /// **Sortiert wird nach der Zeit des Committers, angezeigt wird die des
    /// Autors** ([`Commit::zeit`]) — dieselbe Trennung, die `git log` traegt.
    /// Sie faellt auseinander, wo ein Commit umgesetzt worden ist; die
    /// Reihenfolge folgt dann dem, was `git log` an derselben Stelle zeigt.
    ///
    /// **Und die Sortierung gilt jedem Schwung gleich**, weil jeder Schwung
    /// derselbe Lauf von HEAD aus ist. Was sie kostet, haengt am
    /// Objektzwischenspeicher und ist bei [`OBJEKTSPEICHER`] gemessen: mit ihm
    /// ist der sortierte Lauf schneller als der unsortierte ohne ihn.
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
        let lauf = self
            .repo
            .rev_walk([anfang])
            .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
            .all()
            .ok()?;

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
    /// **Eine Datei, die nur in der Cloud liegt, wird dabei nicht gelesen**;
    /// ob sie das tut, fragt [`crate::verzeichnis::ist_platzhalter`], und was
    /// die Marke dann sagt, steht im Modulkopf unter „Eine Datei, die nur in
    /// der Cloud liegt". Der Rumpf steht in [`Gitleser::marken_mit`], und diese
    /// Funktion reicht ihm allein die Frage des Betriebs.
    ///
    /// **Der Status ist ueber die Pfadmuster auf `ordner` beschraenkt** (E9,
    /// C7.7). Der Gewinn ist gemessen und gross, wo der
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
    ///
    /// # `abbruch` ist die eine Ausnahme von „der Leser kennt kein Kennzeichen"
    ///
    /// Diese Auskunft ist die einzige, die lange genug dauert, dass ein
    /// aufgegebener Lauf sie noch zu Ende laufen liesse: gemessen 12 bis 164 ms
    /// an einem Baum mit 100 000 Eintraegen, in einem groesseren mehr. Wer
    /// schnell durch Ordner geht, stapelte damit Statuslaeufe, deren Ergebnis
    /// niemand mehr annimmt, und jeder von ihnen nimmt so viele Faeden, wie das
    /// Geraet Kerne hat. Das Kennzeichen kommt deshalb bis hier herein, in der
    /// Form, die
    /// [`verzeichnis::durchlauf`](crate::verzeichnis::durchlauf) traegt: ein
    /// `&AtomicBool`, gelesen mit [`Ordering::Relaxed`] vor jeder Einheit, die
    /// dauern kann. Der Eintritt selbst ist beim Rufer geprueft, wie beim
    /// Durchlauf.
    ///
    /// **Die Einheit ist seit dem 261026 der naechste Eintrag des Index, und
    /// gefragt wird in `gix` selbst.** `marken_mit` reicht `abbruch` unveraendert
    /// an `Repository::index_worktree_status` weiter; `gix` liest es in jedem
    /// Arbeitsfaden vor jedem Eintrag und im Verzeichnisdurchlauf, und der
    /// Rueckruf des Vergleichs von Kopf-Baum und Index antwortet `Break`,
    /// sobald es gesetzt ist. Bis dahin las KRK es allein zwischen zwei Posten
    /// des Statusstroms von `Platform::into_iter`, und das griff nicht, solange
    /// `gix` an einer Datei hing: der Strom fragte ein eigenes Kennzeichen, das
    /// KRK nie setzte, und ein Erzeuger, der eine ausgelagerte Datei
    /// herunterlud, lieferte keinen Posten, nach dem KRK haette fragen koennen.
    /// Ein Messlauf auf einem Cloudlaufwerk stand so ueber zehn Minuten. Das
    /// Hashen einer grossen **lokalen** Datei bleibt innerhalb der Datei
    /// ununterbrochen; `gix` reicht seinem Hasher kein Kennzeichen hinein.
    ///
    /// Ein Abbruch liefert `None` und keine halbe Liste: die halbe waere von
    /// „diese Eintraege sind unveraendert" nicht zu unterscheiden.
    #[must_use = "die Marken sind der Befund, den das Ordnermodell nachtraegt"]
    pub fn marken(&self, ordner: &Path, abbruch: &AtomicBool) -> Option<Vec<(String, Marke)>> {
        self.marken_mit(ordner, abbruch, &|_, angaben| {
            crate::verzeichnis::ist_platzhalter(angaben)
        })
    }

    /// Dasselbe wie [`Gitleser::marken`], mit austauschbarer Platzhalterfrage.
    ///
    /// **Der Betrieb ruft [`Gitleser::marken`] und nie diese Funktion.** Die
    /// Frage ist allein fuer die Proben austauschbar: ein echter Platzhalter
    /// laesst sich ohne File-Provider-Domaene nicht herstellen, und die Proben
    /// in `tests/git.rs` reichen deshalb eine Frage herein, die nach dem
    /// Dateinamen antwortet. Oeffentlich ist sie, weil diese Proben ausserhalb
    /// der Kiste liegen.
    ///
    /// `ist_platzhalter` bekommt den absoluten Pfad einer verfolgten Datei und
    /// ihre Angaben aus `lstat`, und zwar allein fuer Dateien, deren
    /// stat-Angaben vom Index abweichen oder deren Eintrag racy ist; fuer alle
    /// anderen fragt `gix` den Vergleich gar nicht. Sie wird aus mehreren
    /// Arbeitsfaeden zugleich gerufen und ist deshalb `Sync`.
    #[must_use = "die Marken sind der Befund, den das Ordnermodell nachtraegt"]
    pub fn marken_mit(
        &self,
        ordner: &Path,
        abbruch: &AtomicBool,
        ist_platzhalter: &(dyn Fn(&Path, &Metadata) -> bool + Sync),
    ) -> Option<Vec<(String, Marke)>> {
        let Some(arbeitsbaum) = self.repo.workdir() else {
            return Some(Vec::new());
        };
        let praefix = repositorypfad(arbeitsbaum, ordner)?;

        let muster: Vec<BString> = if praefix.is_empty() {
            Vec::new()
        } else {
            vec![BString::from(praefix.clone())]
        };

        // Die Reihenfolge ist die des Zweigs ohne `parallel` in
        // `Platform::into_iter` (`gix-0.87.1/src/status/iter/mod.rs`): erst
        // Kopf-Baum gegen Index, dann Index gegen Arbeitskopie.
        let gehalten = self.repo.index_or_empty().ok()?;
        let index: &gix::index::State = &gehalten;
        let baum = self.repo.head_tree_id_or_empty().ok()?;
        let mut pfadmuster = self
            .repo
            .pathspec(
                false,
                &muster,
                true,
                index,
                gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
            )
            .ok()?;

        let mut gefunden: Vec<(String, Marke)> = Vec::new();
        self.repo
            .tree_index_status(
                &baum,
                index,
                Some(&mut pfadmuster),
                gix::status::tree_index::TrackRenames::AsConfigured,
                |aenderung, _, _| {
                    let aenderung = aenderung.into_owned();
                    let marke = indexaenderung_deuten(&aenderung);
                    if let Some(name) =
                        eintragsname(pfad_der_indexaenderung(&aenderung).as_ref(), &praefix)
                    {
                        eintragen(&mut gefunden, name, marke);
                    }
                    let weiter = if abbruch.load(Ordering::Relaxed) {
                        ControlFlow::Break(())
                    } else {
                        ControlFlow::Continue(())
                    };
                    Ok::<_, std::convert::Infallible>(weiter)
                },
            )
            .ok()?;
        if abbruch.load(Ordering::Relaxed) {
            return None;
        }

        let mut stat = self.repo.stat_options().ok()?;
        stat.trust_ctime = false;
        let vergleich = Inhaltsvergleich {
            index,
            arbeitsbaum,
            stat,
            ist_platzhalter,
        };
        let unterbau = gix::status::index_worktree::BuiltinSubmoduleStatus::new(
            self.repo.clone().into_sync(),
            gix::status::Submodule::Given {
                ignore: gix::submodule::config::Ignore::All,
                check_dirty: false,
            },
        )
        .ok()?;
        let durchlauf = self
            .repo
            .dirwalk_options()
            .ok()?
            .emit_untracked(gix::dir::walk::EmissionMode::Matching);
        let mut sammler = Sammler {
            praefix: &praefix,
            gefunden,
        };
        self.repo
            .index_worktree_status(
                index,
                &muster,
                &mut sammler,
                vergleich,
                unterbau,
                &mut gix::progress::Discard,
                abbruch,
                gix::status::index_worktree::Options {
                    sorting: None,
                    dirwalk_options: Some(durchlauf),
                    rewrites: None,
                    thread_limit: None,
                },
            )
            .ok()?;

        // `gix` darf nach einem Abbruch mit `Ok` und einer halben Liste
        // zurueckkehren; die halbe waere von „unveraendert" nicht zu
        // unterscheiden.
        if abbruch.load(Ordering::Relaxed) {
            return None;
        }
        Some(sammler.gefunden)
    }
}

/// Was der [`Inhaltsvergleich`] ueber eine Datei sagt, deren stat-Angaben vom
/// Index abweichen.
///
/// Kein Befund, also `None` aus dem Vergleich, heisst „unveraendert". Die zwei
/// Werte trennen, was KRK weiss, von dem, was es nicht weiss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Befund {
    /// Der Inhalt weicht ab: gelesen und verglichen bei einer lokalen Datei,
    /// an der Groesse erkannt bei einer ausgelagerten.
    Geaendert,
    /// Die Datei liegt nur in der Cloud, und ohne ihren Inhalt laesst sich
    /// nicht sagen, ob sie geaendert ist. Daraus wird [`Marke::Unbekannt`].
    Ungelesen,
}

/// Der Inhaltsvergleich, den KRK `gix` statt `FastEq` reicht.
///
/// `gix` ruft ihn allein fuer Eintraege, deren stat-Angaben abweichen oder die
/// racy sind (`compute_status` in `gix-status-0.34.1`), und er ist die eine
/// Stelle, an der `gix` sonst den Inhalt lesen wuerde. Fuer eine lokale Datei
/// tut er Byte fuer Byte dasselbe wie `FastEq`; fuer eine ausgelagerte faellt
/// er das Urteil aus den stat-Angaben allein und ruehrt `ReadData` nicht an.
/// Das Bild dazu steht im Modulkopf.
#[derive(Clone)]
struct Inhaltsvergleich<'a> {
    /// Der Index, aus dem `gix` die Eintraege nimmt; er liefert den Pfad eines
    /// Eintrags und den Zeitstempel fuer die Racy-Pruefung.
    index: &'a gix::index::State,
    /// Der Arbeitsbaum, unter dem der Pfad eines Eintrags liegt.
    arbeitsbaum: &'a Path,
    /// Die stat-Optionen des Repositorys, mit `trust_ctime` auf `false`: das
    /// Auslagern hebt ctime, ohne den Inhalt zu aendern.
    stat: gix::index::entry::stat::Options,
    /// Ob die Datei nur in der Cloud liegt.
    ist_platzhalter: &'a (dyn Fn(&Path, &Metadata) -> bool + Sync),
}

impl CompareBlobs for Inhaltsvergleich<'_> {
    type Output = Befund;

    fn compare_blobs<'a, 'b>(
        &mut self,
        entry: &gix::index::Entry,
        worktree_blob_size: u64,
        data: impl ReadData<'a>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Befund>, gix::status::plumbing::index_as_worktree::Error> {
        let pfad = self
            .arbeitsbaum
            .join(gix::path::from_bstr(entry.path(self.index)));
        // Ein zweites `lstat` je Eintrag, und nur fuer Eintraege, deren stat
        // abweicht. `lstat` laedt nichts herunter.
        let Ok(angaben) = std::fs::symlink_metadata(&pfad) else {
            return Ok(Some(Befund::Ungelesen));
        };
        if !(self.ist_platzhalter)(&pfad, &angaben) {
            return FastEq
                .compare_blobs(entry, worktree_blob_size, data, buf)
                .map(|befund| befund.map(|()| Befund::Geaendert));
        }
        Ok(ausgelagert_deuten(
            entry,
            worktree_blob_size,
            &pfad,
            self.stat,
            self.index,
        ))
    }
}

/// Das Urteil ueber eine ausgelagerte Datei, aus ihren stat-Angaben allein.
///
/// Drei Ausgaenge in dieser Reihenfolge (Entscheid E-Stat, Moeglichkeit 2):
/// weicht die Groesse ab, ist sie geaendert, nach derselben Regel, die `FastEq`
/// ohne zu lesen anwendet; passen die stat-Angaben ohne ctime und ist der
/// Eintrag nicht racy, gilt sie als unveraendert; sonst ist sie ungelesen.
/// Die Pruefung nimmt `gix`s eigene `Stat::matches` und `Stat::is_racy`, damit
/// keine zweite stat-Logik neben der Kiste entsteht. Dafuer holt sie die
/// Angaben ein drittes Mal ueber `lstat`, in der Form, die `gix` liest; das
/// kostet einen Aufruf je ausgelagerter Datei und laedt nichts.
fn ausgelagert_deuten(
    eintrag: &gix::index::Entry,
    groesse: u64,
    pfad: &Path,
    optionen: gix::index::entry::stat::Options,
    index: &gix::index::State,
) -> Option<Befund> {
    if u64::from(eintrag.stat.size) != groesse
        && (eintrag.id.is_empty_blob() || eintrag.stat.size != 0)
    {
        return Some(Befund::Geaendert);
    }
    let Ok(angaben) = gix::index::fs::Metadata::from_path_no_follow(pfad) else {
        return Some(Befund::Ungelesen);
    };
    let Ok(jetzt) = gix::index::entry::Stat::from_fs(&angaben) else {
        return Some(Befund::Ungelesen);
    };
    if jetzt.matches(&eintrag.stat, optionen) && !jetzt.is_racy(index.timestamp(), optionen) {
        None
    } else {
        Some(Befund::Ungelesen)
    }
}

/// Nimmt die Eintraege von Index gegen Arbeitskopie entgegen und traegt sie
/// unter dem Namen im angezeigten Ordner ein.
struct Sammler<'p> {
    /// Der angezeigte Ordner, repositoryrelativ; siehe [`eintragsname`].
    praefix: &'p [u8],
    /// Die Marken, wie [`eintragen`] sie fuehrt; der Vergleich von Kopf-Baum
    /// und Index hat sie schon begonnen.
    gefunden: Vec<(String, Marke)>,
}

impl<'i> VisitEntry<'i> for Sammler<'_> {
    type ContentChange = Befund;
    type SubmoduleStatus = gix::submodule::Status;

    fn visit_entry(&mut self, eintrag: Arbeitskopieposten<'i>) {
        let Some((pfad, marke)) = posten_deuten(&eintrag) else {
            return;
        };
        let Some(name) = eintragsname(pfad, self.praefix) else {
            return;
        };
        eintragen(&mut self.gefunden, name, marke);
    }
}

/// Ein Eintrag aus dem Vergleich von Index und Arbeitskopie, wie
/// [`Sammler`] ihn bekommt.
type Arbeitskopieposten<'i> = Entry<'i, Befund, gix::submodule::Status>;

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

/// Welche Marke eine Aenderung zwischen Kopf-Baum und Index traegt.
///
/// Diese Haelfte liefert das **Vorgemerkte** und die Umbenennung im Index; die
/// Fallunterscheidung ist vollstaendig und ohne Auffangzweig.
fn indexaenderung_deuten(aenderung: &gix::diff::index::Change) -> Marke {
    match aenderung {
        gix::diff::index::Change::Rewrite { .. } => Marke::Umbenannt,
        gix::diff::index::Change::Addition { .. }
        | gix::diff::index::Change::Deletion { .. }
        | gix::diff::index::Change::Modification { .. } => Marke::Vorgemerkt,
    }
}

/// Welche Marke ein Eintrag aus dem Vergleich von Index und Arbeitskopie
/// traegt, und zu welchem Pfad.
///
/// Diese Haelfte liefert das **Geaenderte**, das **Unbekannte**, das **Neue**
/// und den **Konflikt**, dazu die Umbenennung aus dem Verzeichnisdurchlauf. Die
/// Fallunterscheidung ist vollstaendig ueber `Entry`, `EntryStatus`, `Change`
/// und [`Befund`] und hat keinen Auffangzweig.
///
/// **[`Marke::Unbekannt`] steht genau dort, wo der Inhalt ungelesen blieb und
/// sonst nichts bekannt ist.** Eine geaenderte Ausfuehrbarkeit ist eine
/// bekannte Aenderung, auch wenn der Inhalt ungelesen blieb, und bleibt „M".
///
/// `None` heisst „dieser Eintrag traegt keine Marke". Drei Faelle fallen
/// darunter, und jeder aus einem eigenen Grund:
///
/// - **`EntryStatus::NeedsUpdate`**: der Eintrag hat sich nicht geaendert, nur
///   sein Stat-Zwischenspeicher waere aufzufrischen. **Seit dem 261026 erreicht
///   der Posten KRK**: bis dahin fing ihn der Statusiterator von `gix` ab
///   (`Iter::maybe_keep_index_change` in `gix-0.87.1/src/status/iter/mod.rs`,
///   gemessen am 260831 in
///   `circles/260830-1045-git-bereich-liest-status-branch-verlauf/issues/260831-0855_*_der-zweig-fuer-needsupdate-in-posten-deuten-ist-unerreichbar-gix-faengt-den-posten-vorher-ab.md`),
///   und der Weg ueber `Repository::index_worktree_status` kennt diesen Filter
///   nicht. Er bleibt ohne Marke, und zurueckgeschrieben wird weiter nichts:
///   der Weg liefert ein anderes `Outcome`, an dem es `write_changes` gar
///   nicht gibt (E8). Eine ausgelagerte Datei, deren stat ohne ctime passt,
///   kommt ebenfalls hier an.
/// - **`Status::Ignored` und `Status::Pruned`** aus dem Verzeichnisdurchlauf:
///   ein ignorierter Eintrag ist kein neuer.
/// - **`Status::Tracked`**: der Durchlauf hat einen Eintrag gefunden, den der
///   Index schon fuehrt; seine Aenderung meldet der Vergleich der Eintraege.
fn posten_deuten<'p>(posten: &'p Arbeitskopieposten<'_>) -> Option<(&'p BStr, Marke)> {
    match posten {
        Entry::Modification {
            rela_path, status, ..
        } => {
            let marke = match status {
                EntryStatus::Conflict { .. } => Marke::Konflikt,
                EntryStatus::Change(aenderung) => aenderung_deuten(aenderung),
                EntryStatus::IntentToAdd => Marke::Vorgemerkt,
                EntryStatus::NeedsUpdate(_) => return None,
            };
            Some((rela_path, marke))
        }
        Entry::DirectoryContents { entry, .. } => match entry.status {
            gix::dir::entry::Status::Untracked => Some((entry.rela_path.as_ref(), Marke::Neu)),
            gix::dir::entry::Status::Ignored(_)
            | gix::dir::entry::Status::Pruned
            | gix::dir::entry::Status::Tracked => None,
        },
        Entry::Rewrite { dirwalk_entry, .. } => {
            Some((dirwalk_entry.rela_path.as_ref(), Marke::Umbenannt))
        }
    }
}

/// Die Marke einer Aenderung in der Arbeitskopie.
fn aenderung_deuten(aenderung: &Change<Befund, gix::submodule::Status>) -> Marke {
    match aenderung {
        Change::Modification {
            executable_bit_changed: false,
            content_change,
            ..
        } => match content_change {
            Some(Befund::Ungelesen) => Marke::Unbekannt,
            Some(Befund::Geaendert) | None => Marke::Geaendert,
        },
        Change::Modification {
            executable_bit_changed: true,
            ..
        }
        | Change::Removed
        | Change::Type { .. }
        | Change::SubmoduleModification(_) => Marke::Geaendert,
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

/// Ob dieser Fehlschlag von `gix::discover` die **entschiedene** Auskunft
/// „dieser Ordner liegt in keinem Repository" traegt.
///
/// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig: eine
/// Variante, die `gix` hinzufuegt, haelt den Bau an und erzwingt eine bewusste
/// Einordnung, statt still in die Verneinung zu fallen. Die Wurzel-`Cargo.toml`
/// nagelt die Kiste dafuer auf `0.87` fest.
///
/// **Wahr wird genau eine Lage: bis zur Wurzel gesucht und nichts gefunden.**
/// Die drei Varianten, die das sagen, unterscheiden sich allein darin, wo die
/// Suche geendet hat — an der Wurzel, an einer Deckelung, an der
/// Dateisystemgrenze —, und alle drei heissen dasselbe.
///
/// Alles andere ist **nicht entschieden**, und jeder Fall aus eigenem Grund:
///
/// - `CurrentDir` spricht ueber den laufenden Prozess und nicht ueber den Pfad.
/// - `InvalidInput` und `InaccessibleDirectory` heissen, dass gar nicht erst
///   gesucht werden konnte.
/// - `NoMatchingCeilingDir` heisst, dass die Suche mit einer wirkungslosen
///   Deckelung gelaufen ist.
/// - `NoTrustedGitRepository` heisst, dass eines **gefunden** und wegen seiner
///   Vertrauensstufe verworfen wurde, und `CheckTrust`, dass die Stufe nicht zu
///   ermitteln war.
/// - [`gix::discover::Error::Open`] heisst, dass eines gefunden wurde und sich
///   nicht oeffnen liess: eine unlesbare Konfiguration, eine unbekannte
///   Erweiterung, ein `gitdir:`-Verweis ins Leere.
fn entschiedene_verneinung(fehler: &gix::discover::Error) -> bool {
    use gix::discover::Error as Fehlschlag;
    use gix::discover::upwards::Error as Suche;

    match fehler {
        Fehlschlag::Discover(
            Suche::NoGitRepository { .. }
            | Suche::NoGitRepositoryWithinCeiling { .. }
            | Suche::NoGitRepositoryWithinFs { .. },
        ) => true,
        Fehlschlag::Discover(
            Suche::CurrentDir(_)
            | Suche::InvalidInput { .. }
            | Suche::InaccessibleDirectory { .. }
            | Suche::NoMatchingCeilingDir
            | Suche::NoTrustedGitRepository { .. }
            | Suche::CheckTrust { .. },
        ) => false,
        Fehlschlag::Open(_) => false,
    }
}
