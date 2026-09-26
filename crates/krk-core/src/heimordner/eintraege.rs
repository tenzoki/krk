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
//! [`Neustand`]: sie rechnet, der Editor wendet an. Die fuenf an `tasks.txt`
//! stehen in [`aufgaben`]; die an `notes.txt` folgen mit der Stufe, die sie
//! bedient.
//!
//! **Eine Handlung schreibt allein die Zeile neu, die sie beruehrt**, und die
//! in der Grundform: Abhaken und Textaendern ersetzen die Kopfzeile ihres
//! Blocks, Hinzufuegen haengt eine an, Loeschen nimmt eine weg, Verschieben
//! schreibt gar keine Zeile neu. Jede andere Zeile bleibt Byte fuer Byte, und
//! fuer das Dateiende gilt die Regel aus dem vorigen Abschnitt.
//!
//! **Keine Handlung liefert einen Neustand, der nichts aendert.** `None` heisst
//! bei jeder: es gibt nichts zu tun, weil der Eintrag fehlt, am Rand steht oder
//! schon den verlangten Text traegt. Der Editor legt fuer `None` keinen Umbau
//! und damit keine Handlung auf den Rueckgaengigstapel. Eine unzulaessige
//! Eingabe ist davon getrennt eine [`Abweisung`].

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
        dateiende_angleichen(&mut text, stand);
        text
    }

    /// Der Stand mit ersetzter oder entfernter Kopfzeile des Blocks an
    /// `stelle`; jede andere Zeile bleibt, wie sie war.
    ///
    /// `Some(zeile)` ersetzt die Kopfzeile durch `zeile`, ohne Umbruch
    /// uebergeben; sie bekommt den Umbruch, den die alte trug, so dass das
    /// Dateiende bleibt. `None` entfernt die Kopfzeile, und ihr Anhang haengt
    /// danach am Block darueber oder am Vorspann.
    fn mit_kopfzeile(&self, stelle: usize, zeile: Option<&str>, stand: &str) -> String {
        let mut text: String = self.vorspann.concat();
        for (nummer, block) in self.bloecke.iter().enumerate() {
            if nummer != stelle {
                text.push_str(&block.zeilen.concat());
                continue;
            }
            if let Some(zeile) = zeile {
                text.push_str(zeile);
                if block.kopf().ends_with('\n') {
                    text.push('\n');
                }
            }
            text.push_str(&block.anhang().concat());
        }
        dateiende_angleichen(&mut text, stand);
        text
    }
}

/// Nimmt dem neuen Text den Schlussumbruch, wenn der alte Stand ohne endete.
///
/// Das ist die eine Stelle, an der die Regel „der fehlende Schlussumbruch
/// gehoert dem Dateiende" aus dem Modulkopf durchgesetzt wird. Einen fehlenden
/// Umbruch ergaenzt sie nie: endete der alte Stand mit einem, endet jede
/// Zusammensetzung aus seinen Zeilen ohnehin mit einem.
fn dateiende_angleichen(text: &mut String, stand: &str) {
    if !stand.ends_with('\n') && text.ends_with('\n') {
        let _ = text.pop();
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

/// Warum eine Handlung eine Eingabe nicht annimmt.
///
/// Eine Abweisung laesst den Stand, wie er ist; der Editor laesst die Zelle in
/// Bearbeitung und schreibt [`Abweisung::meldung`] in die Statuszeile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abweisung {
    /// Der Text einer Aufgabe traegt einen Zeilenumbruch. Eine Aufgabe ist eine
    /// Zeile; der Rest hinter dem Umbruch waere eine fremde Zeile oder eine
    /// zweite Aufgabe, und keins von beiden hat der Nutzer verlangt.
    UmbruchImAufgabentext,
}

impl Abweisung {
    /// Der Grund im Wortlaut der Statuszeile.
    pub fn meldung(self) -> &'static str {
        match self {
            Self::UmbruchImAufgabentext => {
                "Eine Aufgabe ist eine Zeile und trägt keinen Zeilenumbruch."
            }
        }
    }
}

/// Die Handlungen an `tasks.txt`.
///
/// `index` zaehlt jeweils ueber die Aufgaben ohne den Vorspann, wie
/// [`Aufgaben::bloecke`] und [`Neustand::auswahl`].
pub mod aufgaben {
    use super::{
        Abweisung, Aufgaben, Aufgabenzeile, Neustand, Richtung, aufgabe_in_grundform, aufgabenzeile,
    };

    /// Weist einen Aufgabentext ab, der keine Zeile ist.
    fn text_pruefen(text: &str) -> Result<(), Abweisung> {
        if text.contains('\n') {
            Err(Abweisung::UmbruchImAufgabentext)
        } else {
            Ok(())
        }
    }

    /// Haengt eine offene Aufgabe mit `text` ans Ende der Datei; sie ist danach
    /// gewaehlt.
    ///
    /// Endet die Datei ohne Umbruch, bekommt ihre letzte Zeile einen, und die
    /// neue Aufgabe endet ohne: das Dateiende bleibt (Modulkopf). Eine leere
    /// Datei hat kein Dateiende zu wahren und bekommt die Zeile samt Umbruch,
    /// wie jede Zeile, die ein Textprogramm schreibt. Ein leerer Text ist
    /// zulaessig; der Editor legt so eine Aufgabe an, deren Text der Nutzer
    /// danach tippt.
    #[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts hinzugefuegt"]
    pub fn hinzufuegen(stand: &str, text: &str) -> Result<Neustand, Abweisung> {
        text_pruefen(text)?;
        let anzahl = Aufgaben::lesen(stand).bloecke().len();
        let mut neu = String::with_capacity(stand.len() + text.len() + 8);
        neu.push_str(stand);
        let schluss = stand.is_empty() || stand.ends_with('\n');
        if !schluss {
            neu.push('\n');
        }
        neu.push_str(&aufgabe_in_grundform(false, text));
        if schluss {
            neu.push('\n');
        }
        Ok(Neustand {
            text: neu,
            auswahl: Some(anzahl),
        })
    }

    /// Ersetzt den Text der Aufgabe an `index` und laesst ihren Erledigt-Zustand.
    ///
    /// Die Aufgabenzeile wird in der Grundform geschrieben, ihre fremden Zeilen
    /// und jede andere Zeile bleiben. `Ok(None)`, wenn es die Aufgabe nicht
    /// gibt oder sie den Text schon traegt: eine Zelle, die ohne Aenderung
    /// verlassen wird, schreibt ihre Zeile deshalb nicht in die Grundform um.
    #[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts geaendert"]
    pub fn text_aendern(
        stand: &str,
        index: usize,
        text: &str,
    ) -> Result<Option<Neustand>, Abweisung> {
        text_pruefen(text)?;
        let aufgaben = Aufgaben::lesen(stand);
        let Some(alt) = gelesen(&aufgaben, index) else {
            return Ok(None);
        };
        if alt.text == text {
            return Ok(None);
        }
        Ok(Some(Neustand {
            text: aufgaben.zerlegung.mit_kopfzeile(
                index,
                Some(&aufgabe_in_grundform(alt.erledigt, text)),
                stand,
            ),
            auswahl: Some(index),
        }))
    }

    /// Hakt die Aufgabe an `index` ab oder oeffnet sie wieder.
    ///
    /// Geschrieben wird allein ihre Zeile, in der Grundform; die Aufgabe bleibt
    /// an ihrer Stelle. `None`, wenn es die Aufgabe nicht gibt.
    #[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts abgehakt"]
    pub fn abhaken(stand: &str, index: usize) -> Option<Neustand> {
        let aufgaben = Aufgaben::lesen(stand);
        let alt = gelesen(&aufgaben, index)?;
        Some(Neustand {
            text: aufgaben.zerlegung.mit_kopfzeile(
                index,
                Some(&aufgabe_in_grundform(!alt.erledigt, alt.text)),
                stand,
            ),
            auswahl: Some(index),
        })
    }

    /// Entfernt die Aufgabenzeile an `index`.
    ///
    /// **Allein die Aufgabenzeile**: ihre fremden Zeilen bleiben stehen und
    /// haengen danach an der Aufgabe darueber oder, war es die erste, am
    /// Vorspann. Was der Nutzer von Hand geschrieben hat, verschwindet so nicht
    /// mit einer Handlung, die ihm nur die Aufgabe zeigt. Gewaehlt ist danach
    /// die Aufgabe, die an die Stelle nachrueckt, sonst die davor; `None` als
    /// Auswahl, wenn keine bleibt. `None` insgesamt, wenn es die Aufgabe nicht
    /// gibt.
    #[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts geloescht"]
    pub fn loeschen(stand: &str, index: usize) -> Option<Neustand> {
        let aufgaben = Aufgaben::lesen(stand);
        let anzahl = aufgaben.bloecke().len();
        if index >= anzahl {
            return None;
        }
        let auswahl = if index + 1 < anzahl {
            Some(index)
        } else {
            index.checked_sub(1)
        };
        Some(Neustand {
            text: aufgaben.zerlegung.mit_kopfzeile(index, None, stand),
            auswahl,
        })
    }

    /// Die Aufgabe an `index`, als Aufgabenzeile gelesen.
    ///
    /// `None` allein, wenn es die Aufgabe nicht gibt: jede Kopfzeile eines
    /// Aufgabenblocks ist nach [`Aufgaben::lesen`] eine Aufgabenzeile, weil die
    /// Zerlegung dieselbe Frage stellt.
    fn gelesen<'a>(aufgaben: &Aufgaben<'a>, index: usize) -> Option<Aufgabenzeile<'a>> {
        aufgabenzeile(aufgaben.bloecke().get(index)?.kopf())
    }

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
