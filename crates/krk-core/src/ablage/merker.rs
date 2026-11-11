//! `reported.toml`: fuer welche Fassung von KRK die Neuerungen an den von Hand
//! gepflegten Ablagedateien schon gemeldet sind. Vom 260926 bis zum 261111
//! hielt sie daneben, ob die alten Zettel schon in `notes.txt` uebernommen
//! sind; das Feld dazu steht weiter in der Datei und wird nicht mehr gesetzt.
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
//! # Das zweite Feld: `zettel_uebernommen` bleibt stehen und wird nicht mehr gesetzt
//!
//! [`Merker::zettel_uebernommen`] hielt vom 260926 bis zum 261111 fest, dass
//! F2 die Zettel `note-1.txt` und `note-2.txt` des Notizblatts der Runde 9
//! schon nach `notes.txt` uebernommen hat
//! (`260926-0007_*_was-geschieht-mit-den-zwei-zetteln-des-bisherigen-notizblatts.md`,
//! `260926-1527_*_die-alten-zettel-werden-bei-jedem-neuen-anlegen-von-krkhome-erneut-uebernommen.md`).
//! Mit `notes.txt` ist die Uebernahme gefallen: diese Fassung liest die Zettel
//! nicht, fragt das Feld nicht und setzt es nicht.
//!
//! **Das Feld bleibt trotzdem in der Struktur** (E7 des Plans
//! `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`).
//! [`vermerken`] liest den Merker und schreibt die ganze Struktur zurueck;
//! fiele das Feld, loeschte der naechste Start mit einer neuen Fassung einen
//! gesetzten Wert aus `reported.toml`, und eine aeltere Fassung von KRK, die
//! der Nutzer danach wieder startet, uebernaehme die Zettel in ein neu
//! angelegtes `~/krkhome` ein zweites Mal. Ein gesetzter Wert ueberlebt
//! deshalb jedes [`vermerken`]; die Probe
//! `ein_gesetzter_zettelmerker_ueberlebt_das_vermerken_der_fassung` in
//! `crates/krk-core/tests/ablage.rs` haelt das.
//!
//! # Was diese Datei nicht traegt
//!
//! Einen Schalter „nicht mehr melden" traegt sie nicht, und auch keine Liste
//! der gemeldeten Namen. Beides waere eine weitere Frage neben der einen, die
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
//! und angenommene Preis.
//!
//! `#[serde(default)]` liest eine `reported.toml`, die das Feld
//! `zettel_uebernommen` nicht kennt, ohne Ersetzung; es steht dann auf `false`.

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
    /// Ob eine Fassung bis zum 261111 die alten Zettel `note-1.txt` und
    /// `note-2.txt` schon nach `notes.txt` uebernommen hat.
    ///
    /// **Diese Fassung fragt das Feld nicht und setzt es nicht; es bleibt
    /// allein stehen, damit [`vermerken`] einen gesetzten Wert zurueckschreibt**
    /// (E7 des Plans
    /// `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`).
    /// Ohne das Feld loeschte der naechste Fassungswechsel den Wert, und eine
    /// aeltere Fassung von KRK uebernaehme die Zettel in ein neu angelegtes
    /// `~/krkhome` ein zweites Mal. Wer das Feld beim Aufraeumen streicht,
    /// weil es keinen Leser hat, baut genau das ein; der Modulkopf sagt es
    /// ausfuehrlich.
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
/// zweiter daneben. **Das Feld der Zettel bleibt, wie es gelesen wurde**: es
/// wird gelesen und mit der ganzen Struktur zurueckgeschrieben, obwohl diese
/// Fassung es nie setzt; der Modulkopf sagt, warum.
pub fn vermerken(zugang: &Zugang<'_>, fassung: &str) -> io::Result<()> {
    let mut merker = laden(zugang).wert;
    merker.gemeldete_fassung = fassung.to_owned();
    zugang.sichern(Datei::Merker, &merker)
}
