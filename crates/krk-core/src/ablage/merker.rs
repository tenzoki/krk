//! `reported.toml`: fuer welche Fassung von KRK die Neuerungen an den von Hand
//! gepflegten Ablagedateien schon gemeldet sind, und seit dem 260926 auch, ob
//! die alten Zettel schon in `notes.txt` uebernommen sind.
//!
//! ```text
//! env!("CARGO_PKG_VERSION") ──> LAUFENDE_FASSUNG ──┐
//!                                                  ├──> meldung_steht_aus
//!   ~/Library/.../KRK/reported.toml ──laden────────┘
//! ```
//!
//! Die Zusage der Runde 24 lautet, die Startzeile aus [`super::neuerungen`]
//! erscheint **einmal je Fassung**. Entscheidbar ist das aus zwei Werten und
//! sonst nichts: der eingebackenen Nummer der laufenden Fassung und einem Wert,
//! der den Prozess ueberlebt. Der zweite gehoert deshalb auf die Platte, und
//! diese Datei traegt ihn.
//!
//! # Warum eine eigene Ablagedatei und kein Feld auf der Sitzung
//!
//! Der naechstliegende Ort waere ein oberstes Feld auf
//! [`Sitzung`](super::Sitzung) gewesen: `session.toml` wird ohnehin alle zwei
//! Sekunden geschrieben, und ein Feld dort haette keinen neuen Schreibweg
//! gebraucht. **Der Ausschlag gegen ihn war die zweite Instanz.**
//! [`Sitzungsschreiber::neu`](super::Sitzungsschreiber::neu) gibt ohne das
//! Sitzungsrecht `None` zurueck, und keine andere Stelle schreibt
//! `session.toml`; eine weitere Instanz — ein gebauter Befehl seit der Runde 7
//! — haette den Merker nie geschrieben und bei **jedem** Start gemeldet. Eine
//! Zusage, die fuer eine Instanz still ausfaellt, ist keine.
//!
//! Diese Datei geht dagegen ueber [`Zugang::sichern`], und den `Zugang` gibt es
//! nur unter der Schreibsperre, die jede Instanz nimmt. Die erste, die meldet,
//! schreibt den Merker; die zweite liest ihn und meldet nicht.
//!
//! Dazu kommt, dass der Merker kein Sitzungszustand ist: `session.toml` haelt,
//! was sich beim **Arbeiten** aendert, und was KRK dem Nutzer einmal gesagt
//! hat, aendert sich beim Einrichten. Die Wahl ist die des Nutzers vom 260910
//! (`circles/260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap/decisions/260910-0818_*_wo-merkt-sich-krk-fuer-welche-fassung-es-die-neuerungen-schon-gemeldet-hat.md`,
//! Moeglichkeit 2). Die offene Frage, ob `session.toml` ihrerseits eine
//! Fassungsangabe bekommt, bleibt davon unberuehrt: dort ginge es um die
//! Fassung, die die **Datei geschrieben** hat, hier um die, fuer die
//! **gemeldet** wurde, und die beiden Werte laufen auseinander, sobald eine
//! Fassung startet, nichts meldet und schreibt.
//!
//! # Die zweite Auskunft: die alten Zettel sind uebernommen
//!
//! [`Merker::zettel_uebernommen`] haelt fest, dass F2 die Zettel des
//! Notizblatts der Runde 9 nicht mehr uebernimmt. Die Uebernahme ist einmalig
//! beantwortet (`260926-0007_*_was-geschieht-mit-den-zwei-zetteln-des-bisherigen-notizblatts.md`),
//! und die zwei alten Dateien bleiben unangetastet liegen; an ihnen selbst
//! laesst sich also nicht ablesen, dass sie schon uebernommen sind, und am
//! Heimordner auch nicht, sobald der Nutzer ihn loescht. Der Wert gehoert
//! deshalb in eine Ablagedatei, und in **diese** aus denselben zwei Gruenden
//! wie die Fassung: `session.toml` schreibt eine Instanz ohne Sitzungsrecht
//! nie, und eine beschaedigte `session.toml` ersetzt der Auslieferungszustand;
//! dieser Merker geht dagegen unter der Schreibsperre ueber [`Zugang::sichern`],
//! die jede Instanz nimmt (`260926-1527_*_die-alten-zettel-werden-bei-jedem-neuen-anlegen-von-krkhome-erneut-uebernommen.md`).
//! Eine weitere Ablagedatei waere dieselbe Bauart mit einem Eintrag mehr in
//! [`Datei::ALLE`] und in jeder Prosastelle, die die Ablage zaehlt.
//!
//! **Die zwei Felder sind zwei Fragen, und keine schreibt die andere mit.**
//! [`vermerken`] und [`zettel_vermerken`] lesen den Merker und setzen allein
//! ihr eigenes Feld; ein Schreiber, der den ganzen Wert aus einem Feld baute,
//! loeschte bei jedem Fassungswechsel die Auskunft ueber die Zettel.
//!
//! # Was diese Datei nicht traegt
//!
//! Einen Schalter „nicht mehr melden" traegt sie nicht, und auch keine Liste
//! der gemeldeten Namen. Beides waere eine weitere Frage neben den zweien, die
//! der Merker beantwortet, und beide Antworten liessen sich aus ihm nicht
//! ablesen. Die Fassung ist eine Zeichenkette und keine zerlegte Fassungsnummer:
//! verglichen wird auf Gleichheit, und „welche ist neuer" fragt hier niemand.
//!
//! # Ein Merker, der nicht dasteht, heisst „noch nie gemeldet"
//!
//! Und ein leerer heisst dasselbe. Beides faellt mit dem
//! Auslieferungszustand von [`Merker`] zusammen, also mit der leeren
//! Zeichenkette, und die ist ungleich jeder Fassungsnummer — eine
//! Fallunterscheidung dafuer gibt es deshalb nicht. Verloren geht der Merker
//! auch dann, wenn ein Loeschwerkzeug den Ablageordner mitnimmt (der Fall vom
//! 17.08.); dann kommt die Meldung ein zweites Mal, und das ist der bekannte
//! und angenommene Preis. Die alten Zettel liegen in demselben Ablageordner
//! und gehen mit ihm; eine zweite Uebernahme gibt es dann nicht mehr.
//!
//! Fuer die Zettel heisst ein fehlender oder beschaedigter Merker „noch nicht
//! uebernommen", `false` ist der Auslieferungszustand, und `#[serde(default)]`
//! liest eine `reported.toml` aus der ersten Fassung mit `~/krkhome`, die das Feld noch nicht
//! kennt, ohne Ersetzung. Was F2 daraus fuer einen Nutzer macht, der mit
//! jener Fassung schon uebernommen hat, steht im Kopf von `heimordner/bereitstellen.rs`.

use std::io;

use serde::{Deserialize, Serialize};

use super::{Datei, Geladen, Zugang};

/// Die Fassung, die gerade laeuft.
///
/// **Sie kommt aus dem Bauwerkzeug und nicht aus dem Buendel.** `Cargo.toml`
/// des Workspace haelt die Zahl an einer Stelle, jedes Mitglied erbt sie ueber
/// `version.workspace = true`, und `env!` legt sie beim Uebersetzen hier ab.
/// Ein Weg ueber die `Info.plist` des Buendels braeuchte einen Zugriff auf das
/// Buendel und lieferte im Prueflauf gar nichts.
pub const LAUFENDE_FASSUNG: &str = env!("CARGO_PKG_VERSION");

/// Was KRK dem Nutzer ueber seine Ablagedateien schon gesagt hat.
///
/// **`#[serde(default)]` und bewusst kein `deny_unknown_fields`**, aus dem
/// Grund, den der Modulkopf von [`super`] fuer `session.toml` ausschreibt: eine
/// spaetere Fassung von KRK darf dieser Datei ein Feld hinzufuegen, ohne dass
/// eine fruehere sie deshalb als beschaedigt liest.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Merker {
    /// Die Fassung, fuer die zuletzt gemeldet wurde.
    ///
    /// Leer heisst „noch nie gemeldet"; siehe den Modulkopf.
    pub gemeldete_fassung: String,
    /// Ob F2 die alten Zettel schon in `notes.txt` uebernommen hat, oder am
    /// Vorgabeort eine `notes.txt` vorgefunden hat, die an ihre Stelle tritt.
    ///
    /// `false` heisst „noch nicht"; siehe den Modulkopf.
    pub zettel_uebernommen: bool,
}

impl Merker {
    /// Ob fuer diese Fassung noch zu melden ist.
    ///
    /// **Eine Ungleichheit und kein Groessenvergleich.** Wer eine aeltere
    /// Fassung ueber eine neuere installiert, soll ihre Meldung ebenso
    /// bekommen: was seine Dateien fuehren, hat sich gegenueber **dieser**
    /// Fassung geaendert, gleich in welche Richtung die Zahl gelaufen ist.
    ///
    /// Die drei Faelle der Abnahme fallen damit auf eine Zeile: ein fehlender
    /// Merker liefert den Auslieferungszustand und also die leere Zeichenkette,
    /// ein leerer traegt sie selbst, und beide sind ungleich jeder
    /// Fassungsnummer.
    #[must_use]
    pub fn meldung_steht_aus(&self, fassung: &str) -> bool {
        self.gemeldete_fassung != fassung
    }
}

/// Liest den Merker.
///
/// Scheitert nie, wie [`Zugang::laden`] darunter. Die
/// [`Ersetzung`](super::Ersetzung) reicht der Ladeweg weiter und
/// verschluckt sie nicht: diese Datei hat keinen zweiten Leser, der sie
/// meldete, und eine beschaedigte Ablagedatei ist auch dann eine Meldung wert,
/// wenn der Schaden nur eine wiederholte Startzeile kostet.
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Merker> {
    zugang.laden(Datei::Merker)
}

/// Vermerkt, dass fuer diese Fassung gemeldet ist.
///
/// Geschrieben wird ueber [`Zugang::sichern`], also unter der Schreibsperre und
/// atomar — derselbe Weg wie fuer `session.toml` und `bookmarks.toml` und kein
/// zweiter daneben. **Das Feld der Zettel bleibt, wie es gelesen wurde**; der
/// Modulkopf sagt, warum.
pub fn vermerken(zugang: &Zugang<'_>, fassung: &str) -> io::Result<()> {
    let mut merker = laden(zugang).wert;
    merker.gemeldete_fassung = fassung.to_owned();
    zugang.sichern(Datei::Merker, &merker)
}

/// Vermerkt, dass die alten Zettel uebernommen sind; die gemeldete Fassung
/// bleibt, wie sie gelesen wurde.
///
/// Derselbe Schreibweg wie [`vermerken`]. Gerufen wird er allein aus
/// `heimordner::bereitstellen`, im selben Durchgang, der `notes.txt` anlegt.
pub fn zettel_vermerken(zugang: &Zugang<'_>) -> io::Result<()> {
    let mut merker = laden(zugang).wert;
    merker.zettel_uebernommen = true;
    zugang.sichern(Datei::Merker, &merker)
}
