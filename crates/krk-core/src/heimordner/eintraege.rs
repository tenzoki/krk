//! Die Form der Eintraege in `notes.txt` und `tasks.txt`, an einer Stelle.
//!
//! Beide Dateien stehen in einer Markdown-nahen Textform, die der Nutzer auch
//! ohne Sondereditor in jedem Textprogramm pflegen kann
//! (`260926-0007_*_in-welchem-format-stehen-notes-txt-und-tasks-txt.md`,
//! Moeglichkeit 1, mit den vier Regeln der Zweitlesung des Spec):
//!
//! - **Eine Notiz** beginnt mit einer Themenzeile `## <Thema>`
//!   ([`ist_themenzeile`]); alles bis zur naechsten Themenzeile ist ihr Text,
//!   auch Zeilen mit `#` und `###`.
//! - **Eine Aufgabe** ist eine Zeile `- [ ] <Text>`, erledigt `- [x] <Text>`.
//!   Gelesen wird grosszuegig ([`aufgabenzeile`]): `*` statt `-`, `[X]` statt
//!   `[x]` und eine Einrueckung davor gelten ebenso. Geschrieben wird eine
//!   Zeile allein dann neu, wenn eine Handlung sie beruehrt hat, und dann in der
//!   Grundform ([`aufgabe_in_grundform`]).
//! - **Eine fremde Zeile**, die keiner Eintragsform folgt, bleibt erhalten. In
//!   `tasks.txt` haengt sie an der Aufgabe ueber ihr und wandert mit ihr; in
//!   `notes.txt` ist alles nach einer Themenzeile Notiztext. Was vor dem ersten
//!   Eintrag steht, ist der **Vorspann** und bleibt oben stehen.
//!
//! # Roh gehalten, roh zusammengesetzt
//!
//! [`Aufgaben::lesen`] und [`Notizen::lesen`] zerlegen einen Stand in Vorspann
//! und Bloecke, und jeder [`Block`] haelt seine Zeilen **so, wie sie in der
//! Datei stehen**, samt Zeilenumbruch. `schreiben` setzt sie unveraendert
//! zusammen. Lesen und Schreiben ergeben deshalb dieselben Bytes, gleich in
//! welcher grosszuegigen Schreibweise die Datei steht; eine Rueckuebersetzung
//! ueber eine Grundform gibt es auf diesem Weg nicht, und sie koennte deshalb
//! auch nichts verfaelschen.
//!
//! **Der fehlende Schlussumbruch gehoert dem Dateiende und nicht dem Block.**
//! Endet die Datei ohne `\n`, traegt die letzte Zeile keinen; wandert ihr Block
//! bei einer Handlung nach oben, bekommt er einen, und der neue letzte Block
//! verliert seinen. Eine Handlung aendert damit das Dateiende nicht.
//!
//! Kodierung und Zeilenende sind die Zusage des Editors: UTF-8 ohne
//! Bytefolgenmarke, Zeilenende `\n` (`text/datei.rs`, Modulkopf). Ein `\r` vor
//! dem Umbruch ist hier kein Trenner, sondern ein Zeichen der Zeile.
//!
//! # Handlungen
//!
//! Eine Handlung ist eine reine Funktion vom alten Stand auf einen
//! [`Neustand`]: sie rechnet, der Editor wendet an. Die erste steht in
//! [`aufgaben`], weil die Probe zu den fremden Zeilen sie braucht; die
//! uebrigen folgen mit den Stufen, die sie bedienen.

/// Die Zeilenmarke einer Themenzeile.
const THEMENMARKE: &str = "## ";

/// Die Grundform einer offenen Aufgabe, bis vor den Text.
pub const GRUNDFORM_OFFEN: &str = "- [ ] ";

/// Die Grundform einer erledigten Aufgabe, bis vor den Text.
pub const GRUNDFORM_ERLEDIGT: &str = "- [x] ";

/// Ob die Zeile eine Notiz eroeffnet: sie beginnt mit `## `.
///
/// `## ` und nicht `# `, damit eine Zeile `# …` als Ueberschrift der ganzen
/// Datei frei bleibt und im Notiztext `#` und `###` stehen duerfen. Die Zeile
/// darf ihren Umbruch tragen; er aendert die Antwort nicht.
pub fn ist_themenzeile(zeile: &str) -> bool {
    zeile.starts_with(THEMENMARKE)
}

/// Eine als Aufgabe gelesene Zeile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aufgabenzeile<'a> {
    /// Ob das Kaestchen `[x]` oder `[X]` traegt.
    pub erledigt: bool,
    /// Der Text nach dem Kaestchen, ohne den einen Trenner davor und ohne
    /// Zeilenumbruch.
    pub text: &'a str,
}

/// Die Zeile als Aufgabe gelesen, oder `None`, wenn sie keine ist.
///
/// Grosszuegig: vor der Marke darf Einrueckung aus Leerzeichen und Tabulatoren
/// stehen, die Marke ist `-` oder `*`, danach folgt mindestens ein Leerzeichen
/// oder Tabulator, dann `[ ]`, `[x]` oder `[X]`. Nach dem Kaestchen endet die
/// Zeile, oder es folgt ein Leerzeichen und der Text. `- [ ]x` ist keine
/// Aufgabe, `- [ ]` eine mit leerem Text.
pub fn aufgabenzeile(zeile: &str) -> Option<Aufgabenzeile<'_>> {
    let zeile = zeile.strip_suffix('\n').unwrap_or(zeile);
    let rest = zeile.trim_start_matches([' ', '\t']);
    let rest = rest.strip_prefix(['-', '*'])?;
    let ohne_trenner = rest.trim_start_matches([' ', '\t']);
    if ohne_trenner.len() == rest.len() {
        return None;
    }
    let (erledigt, rest) = if let Some(rest) = ohne_trenner.strip_prefix("[ ]") {
        (false, rest)
    } else {
        let rest = ohne_trenner
            .strip_prefix("[x]")
            .or_else(|| ohne_trenner.strip_prefix("[X]"))?;
        (true, rest)
    };
    let text = if rest.is_empty() {
        rest
    } else {
        rest.strip_prefix(' ')?
    };
    Some(Aufgabenzeile { erledigt, text })
}

/// Eine Aufgabe in der Grundform, ohne Zeilenumbruch.
///
/// Das ist die Form, in der eine Handlung eine Zeile schreibt, die sie
/// beruehrt hat; jede andere Zeile bleibt roh.
pub fn aufgabe_in_grundform(erledigt: bool, text: &str) -> String {
    let marke = if erledigt {
        GRUNDFORM_ERLEDIGT
    } else {
        GRUNDFORM_OFFEN
    };
    format!("{marke}{text}")
}

/// Ein Eintrag mit seinen Zeilen, roh und samt Umbruch.
///
/// Die erste Zeile ist die Kopfzeile: die Aufgabenzeile oder die Themenzeile.
/// Die uebrigen sind in `tasks.txt` die fremden Zeilen, die an der Aufgabe
/// haengen, in `notes.txt` der Notiztext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'a> {
    zeilen: Vec<&'a str>,
}

impl<'a> Block<'a> {
    /// Die Kopfzeile, roh und samt Umbruch, wenn sie einen traegt.
    pub fn kopf(&self) -> &'a str {
        self.zeilen[0]
    }

    /// Die Zeilen nach der Kopfzeile, roh.
    pub fn anhang(&self) -> &[&'a str] {
        &self.zeilen[1..]
    }

    /// Alle Zeilen des Blocks, roh, die Kopfzeile zuerst.
    pub fn zeilen(&self) -> &[&'a str] {
        &self.zeilen
    }
}

/// Ein Stand in Vorspann und Bloecke zerlegt; die gemeinsame Mechanik von
/// [`Aufgaben`] und [`Notizen`], die sich allein in der Frage unterscheiden,
/// welche Zeile einen Block eroeffnet.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Zerlegung<'a> {
    vorspann: Vec<&'a str>,
    bloecke: Vec<Block<'a>>,
}

impl<'a> Zerlegung<'a> {
    fn lesen(stand: &'a str, eroeffnet: fn(&str) -> bool) -> Self {
        let mut vorspann = Vec::new();
        let mut bloecke: Vec<Block<'a>> = Vec::new();
        for zeile in stand.split_inclusive('\n') {
            if eroeffnet(zeile) {
                bloecke.push(Block {
                    zeilen: vec![zeile],
                });
            } else if let Some(letzter) = bloecke.last_mut() {
                letzter.zeilen.push(zeile);
            } else {
                vorspann.push(zeile);
            }
        }
        Self { vorspann, bloecke }
    }

    /// Die Zeilen unveraendert hintereinander.
    fn schreiben(&self) -> String {
        self.vorspann
            .iter()
            .chain(self.bloecke.iter().flat_map(|block| block.zeilen.iter()))
            .copied()
            .collect()
    }

    /// Die Bloecke in der genannten Reihenfolge, hinter dem Vorspann, mit dem
    /// Dateiende des urspruenglichen Standes.
    ///
    /// Jeder Block, der nicht zuletzt steht, endet auf `\n`; endete der alte
    /// Stand ohne, endet der neue ebenso ohne. Das ist die Regel „der fehlende
    /// Schlussumbruch gehoert dem Dateiende" aus dem Modulkopf.
    fn in_reihenfolge(&self, reihenfolge: &[usize], stand: &str) -> String {
        let mut text: String = self.vorspann.concat();
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        for &stelle in reihenfolge {
            text.push_str(&self.bloecke[stelle].zeilen.concat());
            if !text.ends_with('\n') {
                text.push('\n');
            }
        }
        if !stand.ends_with('\n') && text.ends_with('\n') {
            let _ = text.pop();
        }
        text
    }
}

/// Der Inhalt von `tasks.txt`, zerlegt in Vorspann und Aufgaben.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aufgaben<'a> {
    zerlegung: Zerlegung<'a>,
}

impl<'a> Aufgaben<'a> {
    /// Zerlegt einen Stand: jede Aufgabenzeile eroeffnet einen Block, jede
    /// andere haengt am Block darueber oder, vor der ersten Aufgabe, am
    /// Vorspann.
    pub fn lesen(stand: &'a str) -> Self {
        Self {
            zerlegung: Zerlegung::lesen(stand, |zeile| aufgabenzeile(zeile).is_some()),
        }
    }

    /// Die Zeilen vor der ersten Aufgabe, roh.
    pub fn vorspann(&self) -> &[&'a str] {
        &self.zerlegung.vorspann
    }

    /// Die Aufgaben in der Reihenfolge der Datei.
    pub fn bloecke(&self) -> &[Block<'a>] {
        &self.zerlegung.bloecke
    }

    /// Der Stand, aus dem gelesen wurde, Byte fuer Byte.
    pub fn schreiben(&self) -> String {
        self.zerlegung.schreiben()
    }
}

/// Der Inhalt von `notes.txt`, zerlegt in Vorspann und Notizen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notizen<'a> {
    zerlegung: Zerlegung<'a>,
}

impl<'a> Notizen<'a> {
    /// Zerlegt einen Stand: jede Themenzeile eroeffnet einen Block, alles
    /// danach bis zur naechsten ist Notiztext, alles davor Vorspann.
    pub fn lesen(stand: &'a str) -> Self {
        Self {
            zerlegung: Zerlegung::lesen(stand, ist_themenzeile),
        }
    }

    /// Die Zeilen vor der ersten Themenzeile, roh.
    pub fn vorspann(&self) -> &[&'a str] {
        &self.zerlegung.vorspann
    }

    /// Die Notizen in der Reihenfolge der Datei.
    pub fn bloecke(&self) -> &[Block<'a>] {
        &self.zerlegung.bloecke
    }

    /// Der Stand, aus dem gelesen wurde, Byte fuer Byte.
    pub fn schreiben(&self) -> String {
        self.zerlegung.schreiben()
    }
}

/// Das Ergebnis einer Handlung: der neue Text und der Eintrag, der danach
/// gewaehlt sein soll.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "ein Neustand ist die ganze Wirkung einer Handlung; fallengelassen ist sie nie geschehen"]
pub struct Neustand {
    /// Der ganze neue Inhalt der Datei.
    pub text: String,
    /// Die Stelle des Eintrags, der nach der Handlung gewaehlt ist, gezaehlt
    /// ueber die Bloecke ohne den Vorspann.
    pub auswahl: Option<usize>,
}

/// Wohin ein Eintrag wandert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Richtung {
    /// Eine Stelle nach oben, zum Dateianfang hin.
    Hoch,
    /// Eine Stelle nach unten, zum Dateiende hin.
    Runter,
}

/// Die Handlungen an `tasks.txt`.
pub mod aufgaben {
    use super::{Aufgaben, Neustand, Richtung};

    /// Tauscht die Aufgabe an `index` mit ihrer Nachbarin in `richtung`.
    ///
    /// **Ein bewegter Block behaelt seine Bytes**, samt der fremden Zeilen, die
    /// an ihm haengen; nur die Reihenfolge aendert sich, und der Vorspann bleibt
    /// oben. Die einzige Ausnahme ist der Schlussumbruch, der dem Dateiende
    /// gehoert (Modulkopf). `None`, wenn es die Aufgabe nicht gibt oder sie in
    /// dieser Richtung schon am Rand steht.
    #[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts verschoben"]
    pub fn verschieben(stand: &str, index: usize, richtung: Richtung) -> Option<Neustand> {
        let aufgaben = Aufgaben::lesen(stand);
        let anzahl = aufgaben.bloecke().len();
        if index >= anzahl {
            return None;
        }
        let ziel = match richtung {
            Richtung::Hoch => index.checked_sub(1)?,
            Richtung::Runter if index + 1 < anzahl => index + 1,
            Richtung::Runter => return None,
        };
        let mut reihenfolge: Vec<usize> = (0..anzahl).collect();
        reihenfolge.swap(index, ziel);
        Some(Neustand {
            text: aufgaben.zerlegung.in_reihenfolge(&reihenfolge, stand),
            auswahl: Some(ziel),
        })
    }
}
