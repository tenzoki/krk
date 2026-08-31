//! Die Git-Anbindung der Stufe A: KRK liest ein Repository und schreibt nicht
//! hinein.
//!
//! Vier Auskuenfte, und keine fuenfte: ob dieser Ordner in einem Repository
//! liegt, auf welchem Branch sein HEAD steht, welche Commits davor liegen und
//! welche Marke jeder Eintrag des angezeigten Ordners traegt. Sie stehen in
//! [`leser`]; die Woerter, die der Nutzer davon zu sehen bekommt, stehen in
//! [`texte`], und zwar dort und nirgends sonst.
//!
//! # Die Grenze der Stufe A: es wird gelesen und nicht geschrieben
//!
//! Kein Weg dieses Moduls ruft eine schreibende Funktion von `gix`, nimmt eine
//! Sperre auf ein Repository oder fasst eine Datei unter `.git` an. Der Posten,
//! an dem das etwas kostet, ist benannt und gemessen: `git` schreibt nach einem
//! Status den aufgefrischten Stat-Zwischenspeicher in den Index zurueck, damit
//! der naechste Lauf billiger wird, und `gix` bietet dasselbe ueber
//! `Outcome::write_changes` an. **Dieses Modul ruft es nicht**, und
//! `EntryStatus::NeedsUpdate` wird gelesen und verworfen. Ob die Stufe A das
//! Zurueckschreiben bekommen soll, ist eine offene Nutzerfrage
//! (`shared/decisions/260830-1006_*_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-oder-zahlt-sie-die-wiederholung.md`);
//! solange sie offen ist, zahlt jede Abfrage die Auffrischung erneut.
//!
//! # `None` heisst „unentschieden" und nie „nichts gefunden"
//!
//! Das ist die eine Regel, die dieses Modul durchgaengig traegt, und sie ist
//! aus dem Verzeichnisleser uebernommen: ein Fehlschlag, der ueber den **Zustand
//! des eigenen Prozesses** spricht, darf nicht als Aussage ueber ein fremdes
//! Repository ausgegeben werden. [`leser::Gitleser::kopf`],
//! [`leser::Gitleser::verlauf`] und [`leser::Gitleser::marken`] liefern deshalb
//! `Option`, und `None` heisst dort ueberall dasselbe: die Frage ist nicht
//! beantwortet. Ein Rufer, der `None` bekommt, meldet nichts und laesst die
//! Anzeige leer; er meldet **nicht** „keine Marken" oder „kein Verlauf".
//!
//! Der Gegenfall steht daneben und ist ebenso ausdruecklich:
//! [`leser::Oeffnung::KeinRepository`] ist eine **entschiedene** Antwort, und
//! [`leser::Oeffnung::Unentschieden`] ist es nicht. Wer die beiden zusammenzoege,
//! machte aus einem Deskriptormangel des eigenen Prozesses, aus einem
//! Rechteproblem oder aus einem defekten Repository die Auskunft „dieser Ordner
//! liegt in keinem Repository"; das ist derselbe Defekt, den der Durchlauf mit
//! `260815-0211` einmal getragen hat. Woran die beiden auseinandergehalten
//! werden, sagt `leser::entschiedene_verneinung`.
//!
//! # Der eine Weg herein: der Kanal
//!
//! Die Funktionen in [`leser`] sind **synchron** und sollen es bleiben: sie
//! kennen weder Faden noch Kanal. Wer sie ruft, steht dafuer ein, dass er nicht
//! auf dem Hauptfaden steht — und genau einer tut das, [`lauf::Gitlauf`].
//!
//! **Ein Abbruchkennzeichen kennt genau eine von ihnen**, und der Grund steht
//! bei ihr: [`leser::Gitleser::marken`] ist die einzige Auskunft, die lange
//! genug dauert, dass ein aufgegebener Lauf sie noch zu Ende laufen liesse. Ein
//! `&AtomicBool` ist dabei weder Faden noch Kanal, sondern dieselbe Form, die
//! [`crate::verzeichnis::durchlauf`] traegt; eine zweite daneben entstuende
//! sonst fuer dieselbe Sache.
//!
//! Kein Weg ausserhalb dieses Moduls fragt den Leser unmittelbar; was der
//! Git-Bereich und die Markenspalte zeigen, kommt ueber den Kanal (C7.1 der
//! Runde 23). Die Zaehlprobe
//! `keine_statusabfrage_steht_ausserhalb_des_gitmoduls` in
//! `crates/krk-core/tests/git.rs` haelt es fest.

pub mod lauf;
pub mod leser;
pub mod texte;

use std::time::SystemTime;

/// Der Objektname eines Commits, weitergereicht aus `gix`.
///
/// Er steht hier als Wiederausfuhr, damit `krk-ui` den Verlauf halten kann,
/// ohne `gix` selbst als Abhaengigkeit zu fuehren: die Oberflaeche merkt sich
/// den letzten angezeigten Commit, um beim Nachladen dort weiterzumachen, und
/// braucht dafuer den Namen und sonst nichts aus der Kiste.
pub use gix::ObjectId;

/// Die Marke, die ein Eintrag der Dateiliste traegt.
///
/// Fuenf Zustaende, und ein sechster fuer „unveraendert" entsteht nicht: ein
/// Eintrag ohne Befund traegt eine leere Zelle (A11 der Runde 23). Die
/// Aufzaehlung ist damit die Liste dessen, was ueberhaupt eine Zelle fuellt.
///
/// **Die Reihenfolge der Varianten ist nicht die Rangfolge.** Sie ist die
/// Reihenfolge, in der die Zusammenfassung ihre Zahlen nennt; welche Marke
/// gewinnt, wenn zwei auf denselben Eintrag zutreffen, sagt [`Marke::rang`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Marke {
    /// Der Eintrag ist gegenueber dem Index geaendert und nicht vorgemerkt.
    Geaendert,
    /// Der Eintrag ist gegenueber dem Kopf-Baum vorgemerkt.
    Vorgemerkt,
    /// Der Eintrag ist neu und wird nicht verfolgt.
    Neu,
    /// Der Eintrag steht in einem Konflikt.
    Konflikt,
    /// Der Eintrag ist umbenannt oder kopiert worden.
    Umbenannt,
}

impl Marke {
    /// Alle fuenf Werte, in der Reihenfolge der Aufzaehlung.
    ///
    /// **Der Uebersetzer haelt diese Liste nicht.** Die Feldbreite `[Marke; 5]`
    /// zwingt zu fuenf Eintraegen und sagt nichts darueber, welche fuenf; eine
    /// sechste Variante uebersetzt hier vorbei. Gehalten wird sie deshalb von
    /// der Probe `jede_marke_steht_genau_einmal_in_alle` in
    /// `crates/krk-core/tests/git.rs`, die die Varianten aus dem Quelltext der
    /// Aufzaehlung liest und beide Mengen gegeneinander haelt — dieselbe
    /// Bauform, mit der `Kommando::KENNUNGEN` gehalten wird.
    pub const ALLE: [Marke; 5] = [
        Marke::Geaendert,
        Marke::Vorgemerkt,
        Marke::Neu,
        Marke::Konflikt,
        Marke::Umbenannt,
    ];

    /// Der eine Buchstabe der Markenspalte (E11 der Runde 23).
    ///
    /// Schmal und ohne Farbe lesbar; kein ausgeschriebenes Wort, kein farbiger
    /// Punkt als alleiniges Merkmal. Die Buchstaben folgen den deutschen
    /// Woertern und nicht `git status`: `S` steht fuer „vorgemerkt" und nicht
    /// fuer „staged", `K` fuer „Konflikt", `U` fuer „umbenannt".
    #[must_use]
    pub const fn buchstabe(self) -> char {
        match self {
            Marke::Geaendert => 'M',
            Marke::Vorgemerkt => 'S',
            Marke::Neu => 'N',
            Marke::Konflikt => 'K',
            Marke::Umbenannt => 'U',
        }
    }

    /// Welche Marke gewinnt, wenn zwei auf denselben Eintrag zutreffen.
    ///
    /// Die Zelle traegt einen Buchstaben und nicht zwei, und ein Eintrag kann
    /// mehrere Zustaende zugleich haben: eine vorgemerkte Datei, die danach
    /// weiter bearbeitet wurde, ist vorgemerkt **und** geaendert. Ein Ordner
    /// erbt daneben die Marken seines ganzen Unterbaums, und dort treffen
    /// regelmaessig mehrere zusammen.
    ///
    /// Die Rangfolge beantwortet eine Frage, und zwar immer dieselbe: **was ist
    /// an dieser Stelle noch zu tun?** Ein Konflikt will aufgeloest werden und
    /// steht deshalb oben. Eine Umbenennung ist die Auskunft, die ein Paar aus
    /// `N` und einer verschwundenen Zeile gerade verbirgt. Eine unvorgemerkte
    /// Aenderung ist noch vorzumerken, eine vorgemerkte noch zu committen. Neu
    /// ist der Grundfall und steht unten — und das ist keine Geschmacksfrage:
    /// das Ziel einer erkannten Umbenennung erscheint im Verzeichnisdurchlauf
    /// als unverfolgter Eintrag, und stuende `Neu` oben, verschwaende die
    /// Umbenennung wieder.
    #[must_use]
    pub const fn rang(self) -> u8 {
        match self {
            Marke::Konflikt => 4,
            Marke::Umbenannt => 3,
            Marke::Geaendert => 2,
            Marke::Vorgemerkt => 1,
            Marke::Neu => 0,
        }
    }
}

/// Worauf HEAD steht.
///
/// Vier Werte, vollstaendig und ohne Auffangzweig. Die Trennung, an der es
/// haengt, ist die zwischen [`Kopf::Branch`] und [`Kopf::OhneCommit`]: ein
/// frisch angelegtes Repository hat einen **ungeborenen** HEAD, und dort
/// liefert `head_name()` den Namen, waehrend `head_id()` mit `Unborn`
/// scheitert. Wer den Verlauf holt, ohne diesen Fall zu trennen, bekommt einen
/// Fehler statt einer leeren Liste; die Pruefhuelle der Machbarkeitsanalyse ist
/// genau daran gescheitert, bevor die Trennung eingebaut war.
///
/// **[`Kopf::KeinRepository`] entsteht nicht in [`leser::Gitleser::kopf`]**,
/// sondern beim Rufer: es ist die Anzeige, die zu
/// [`leser::Oeffnung::KeinRepository`] gehoert. Der Wert steht hier und nicht
/// beim Leser, weil `Kopf` der Zustand der Anzeige ist und nicht die Antwort
/// eines geoeffneten Repositorys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kopf {
    /// HEAD steht auf einem Branch dieses Namens, und der Branch hat einen
    /// Commit.
    Branch(String),
    /// HEAD ist abgeloest und steht unmittelbar auf einem Commit; der Wert ist
    /// dessen Kurzhash. `gix` beantwortet nicht, welcher Branch diesen Commit
    /// enthaelt, und `git` tut es an dieser Stelle auch nicht; KRK behauptet
    /// deshalb keinen Branchnamen, den es nicht hat (A6).
    Abgeloest(String),
    /// HEAD steht auf einem Branch dieses Namens, und das Repository hat noch
    /// keinen Commit (A7).
    OhneCommit(String),
    /// Der angezeigte Ordner liegt bis zur Wurzel in keinem Repository (E5).
    KeinRepository,
}

/// Ein Commit, so weit ihn der Git-Bereich zeigt.
///
/// Sechs Felder und keine Liste geaenderter Dateien: `gix-diff` je Commit zu
/// befragen ist der Gegenstand von E13 ausdruecklich **nicht**.
/// [`Commit::kurzbeschreibung`] steht in der Verlaufszeile,
/// [`Commit::nachricht`] in der Flaeche darunter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// Der volle Objektname. Er ist zugleich die Marke, an der das Nachladen
    /// des Verlaufs wieder ansetzt.
    pub id: ObjectId,
    /// Die erste Zeile der Nachricht.
    pub kurzbeschreibung: String,
    /// Die vollstaendige Nachricht, so wie sie im Objekt steht.
    pub nachricht: String,
    /// Der Name des Autors.
    pub autor: String,
    /// Die E-Mail-Adresse des Autors.
    pub email: String,
    /// Der Zeitpunkt des Autors, nicht der des Committers.
    pub zeit: SystemTime,
}
