//! Die Form der Geheimnisse im Klartext von `secrets.txt`, ihre Handlungen
//! und die Suche eines Eintrags nach seinem Namen.
//!
//! **Ein Geheimnis ist eine Notiz, deren Text mit bis zu vier Feldzeilen
//! beginnt** (Modulkopf von `heimordner/eintraege.rs`), und jede Handlung hier
//! rechnet ueber [`notizen`]:
//! dieselbe Zerlegung, dieselbe Byte-Treue, dasselbe Dateiende.
//!
//! # Die Leseregel
//!
//! Nach der Themenzeile wird eine **ununterbrochene Folge** von Zeilen als
//! Feldzeilen gelesen, solange jede mit einem der vier Schluessel
//! `protocol`, `address`, `account`, `password` (ohne Ruecksicht auf Gross-
//! und Kleinschreibung), einem Doppelpunkt und danach nichts oder einem
//! Leerzeichen beginnt **und** dieser Schluessel in der Folge noch nicht
//! stand. Die erste andere Zeile beendet die Folge, und alles ab ihr sind die
//! Notizen. Der Wert ist der Rest der Zeile nach dem einen Leerzeichen, ohne
//! Zeilenumbruch und ohne ein `\r` davor: eine von Hand mit CRLF geschriebene
//! Feldzeile traegt ihr `\r` nicht in den Wert. Name und Notizen bleiben roh,
//! wie bei einer Notiz.
//!
//! **Die Grundform schreibt alle vier Feldzeilen**, eine leere als
//! `protocol:` ohne Leerzeichen dahinter. Nach einer Grundform endet die
//! Folge deshalb in jedem Fall, und eine Notizzeile, die wie eine Feldzeile
//! aussieht, bleibt Notiz. In einem Eintrag des frueheren Formats wird eine
//! erste Textzeile `password: …` als Feld gelesen; verloren geht dabei nichts,
//! sie steht nur in der Spalte, die ihr Wortlaut nennt.
//!
//! # Was eine Handlung schreibt
//!
//! [`aendern`] schreibt den beruehrten Eintrag in der Grundform: die
//! Themenzeile allein bei geaendertem Namen neu, sonst roh; die vier
//! Feldzeilen immer; die Notizzeilen allein bei geaenderten Notizen neu, sonst
//! roh; der Trenner bleibt. Jeder andere Eintrag bleibt Byte fuer Byte.
//! [`hinzufuegen`] haengt einen Eintrag in der Grundform an, [`loeschen`] und
//! [`verschieben`] sind die der Notizen.
//!
//! # Das Passwort
//!
//! [`Passwort`] haelt den Wert, der bei einer Verbindung KRK verlaesst
//! (Entscheidung 11 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`):
//! privates Feld, kein `Display`, `Debug` verdeckt, getilgt in `Drop`, und
//! genau ein Weg heraus, [`Passwort::preisgeben`]. Das Schreiben in den
//! Klartext von `secrets.txt` geht nicht ueber diesen Weg, sondern ueber eine
//! Methode, die allein dieses Modul sieht: der Klartext verlaesst KRK nur
//! verschluesselt. [`Geheimnis::passwort`] zeigt dagegen den Klartext fuer
//! die Tabelle des Editors, wie die Rohansicht derselben Datei es auch tut
//! (Entscheidung 17); das ist Editorinhalt und faellt unter die Regel um
//! `Editormodell::haelt_geheimnisse` in `krk-ui`.
//!
//! `stelle` zaehlt jeweils ueber die Eintraege in der Reihenfolge der Datei
//! ohne den Vorspann, wie [`Notizen::bloecke`] und [`Neustand::auswahl`].

use std::fmt;

use zeroize::{Zeroize, Zeroizing};

use super::{
    Abweisung, Block, Neustand, Notizen, Richtung, THEMENMARKE, ist_themenzeile, notizen,
    text_und_trenner,
};

/// Die Schluessel der vier Feldzeilen in ihrer Reihenfolge in der
/// Grundform; die Stelle eines Schluessels ist die Stelle seines Werts in
/// [`Teile::felder`].
const FELDSCHLUESSEL: [&str; 4] = ["protocol", "address", "account", "password"];

/// Eine Spalte der Geheimnistabelle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feld {
    /// Der Name, die Themenzeile `## <Name>`; der Schluessel, unter dem
    /// ein Lesezeichen den Eintrag findet.
    Name,
    /// Die Feldzeile `protocol:`.
    Protokoll,
    /// Die Feldzeile `address:`.
    Adresse,
    /// Die Feldzeile `account:`.
    Konto,
    /// Die Feldzeile `password:`.
    Passwort,
    /// Alles nach den Feldzeilen, mit Absaetzen.
    Notizen,
}

impl Feld {
    /// Jedes Feld, in der Reihenfolge der Spalten.
    pub const ALLE: [Feld; 6] = [
        Feld::Name,
        Feld::Protokoll,
        Feld::Adresse,
        Feld::Konto,
        Feld::Passwort,
        Feld::Notizen,
    ];

    /// Die Stelle der Feldzeile in [`FELDSCHLUESSEL`], oder `None` fuer
    /// Name und Notizen, die keine Feldzeile sind.
    fn feldzeile(self) -> Option<usize> {
        match self {
            Feld::Protokoll => Some(0),
            Feld::Adresse => Some(1),
            Feld::Konto => Some(2),
            Feld::Passwort => Some(3),
            Feld::Name | Feld::Notizen => None,
        }
    }
}

/// Ein Passwort aus `secrets.txt` oder aus einer Eingabe.
///
/// **Der Typ haelt den einen Weg, auf dem ein Passwort KRK verlaesst**: das
/// Feld ist privat, es gibt kein `Display`, `Debug` zeigt `Passwort(****)`,
/// `Drop` tilgt den Text mit `zeroize`, und herausgegeben wird er allein
/// ueber [`Passwort::preisgeben`]. Kopien aus `Clone` werden ebenso getilgt.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Passwort(String);

impl Passwort {
    /// Nimmt den Text an, etwa aus dem Passwortfeld eines Blattes.
    #[must_use]
    pub fn neu(text: String) -> Passwort {
        Passwort(text)
    }

    /// Ob das Passwort leer ist; verraet sonst nichts.
    #[must_use]
    pub fn ist_leer(&self) -> bool {
        self.0.is_empty()
    }

    /// Gibt den Text heraus, als Wert, der sich beim Fallen tilgt.
    ///
    /// **Der eine Weg, auf dem ein Passwort KRK verlaesst.** Seine Rufer
    /// haelt eine Probe am Quelltext; gerufen wird er allein dort, wo der
    /// Wert unmittelbar an das System geht, das sich damit anmeldet.
    #[must_use = "der herausgegebene Text ist die ganze Wirkung"]
    pub fn preisgeben(mut self) -> Zeroizing<String> {
        Zeroizing::new(std::mem::take(&mut self.0))
    }

    /// Der Text fuer das Schreiben in den Klartext von `secrets.txt`;
    /// allein in diesem Modul sichtbar.
    fn als_text(&self) -> &str {
        &self.0
    }
}

impl Drop for Passwort {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for Passwort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Passwort(****)")
    }
}

/// Ein Eintrag, wie die Tabelle ihn zeigt.
///
/// `Debug` zeigt das Passwort nicht.
#[derive(Clone, PartialEq, Eq)]
pub struct Geheimnis<'a> {
    /// Der Name, ohne `## ` davor und ohne Zeilenumbruch, roh.
    pub name: &'a str,
    /// Der Wert von `protocol:`, leer ohne Feldzeile.
    pub protokoll: &'a str,
    /// Der Wert von `address:`.
    pub adresse: &'a str,
    /// Der Wert von `account:`.
    pub konto: &'a str,
    /// Der Wert von `password:`, im Klartext fuer die Tabelle.
    pub passwort: &'a str,
    /// Die Notizen, Zeilen durch `\n` getrennt, ohne Trenner und ohne
    /// Schlussumbruch, wie [`super::Notiz::text`].
    pub notizen: String,
}

impl Geheimnis<'_> {
    /// Der Wert der Spalte `feld`, wie die Zelle ihn zeigt.
    #[must_use]
    pub fn wert(&self, feld: Feld) -> &str {
        match feld {
            Feld::Name => self.name,
            Feld::Protokoll => self.protokoll,
            Feld::Adresse => self.adresse,
            Feld::Konto => self.konto,
            Feld::Passwort => self.passwort,
            Feld::Notizen => &self.notizen,
        }
    }
}

impl fmt::Debug for Geheimnis<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Geheimnis")
            .field("name", &self.name)
            .field("protokoll", &self.protokoll)
            .field("adresse", &self.adresse)
            .field("konto", &self.konto)
            .field("passwort", &"****")
            .field("notizen", &self.notizen)
            .finish()
    }
}

/// Die Felder eines neuen Eintrags, fuer [`hinzufuegen`].
///
/// `Debug` zeigt das Passwort nicht, weil [`Passwort`] es verbirgt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Geheimnisentwurf {
    /// Der Name.
    pub name: String,
    /// Das Protokoll als Text.
    pub protokoll: String,
    /// Die Adresse.
    pub adresse: String,
    /// Das Konto.
    pub konto: String,
    /// Das Passwort.
    pub passwort: Passwort,
    /// Die Notizen.
    pub notizen: String,
}

/// Was eine Verbindung aus einem Eintrag braucht: Name, Protokoll,
/// Adresse, Konto und Passwort, ohne die Notizen.
///
/// `Debug` zeigt das Passwort nicht, weil [`Passwort`] es verbirgt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verbindungsdaten {
    /// Der Name, wie er in der Datei steht, roh.
    pub name: String,
    /// Das Protokoll als Text; ob KRK es kennt, sagt
    /// `heimordner::verbindung`.
    pub protokoll: String,
    /// Die Adresse.
    pub adresse: String,
    /// Das Konto.
    pub konto: String,
    /// Das Passwort.
    pub passwort: Passwort,
}

/// Warum [`finden`] keinen Eintrag liefert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suchfehler {
    /// Kein Eintrag traegt den Namen.
    Keiner,
    /// So viele Eintraege tragen ihn, mehr als einer; verbunden wird
    /// dann nicht, statt zu raten.
    Mehrdeutig(usize),
}

/// Der Klartext von `secrets.txt`, zerlegt in Vorspann und Eintraege.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Geheimnisse<'a> {
    notizen: Notizen<'a>,
}

impl<'a> Geheimnisse<'a> {
    /// Zerlegt einen Stand mit [`Notizen::lesen`]; jede Themenzeile
    /// eroeffnet einen Eintrag.
    #[must_use]
    pub fn lesen(stand: &'a str) -> Self {
        Self {
            notizen: Notizen::lesen(stand),
        }
    }

    /// Die Zahl der Eintraege.
    #[must_use]
    pub fn anzahl(&self) -> usize {
        self.notizen.bloecke().len()
    }

    /// Der Stand, aus dem gelesen wurde, Byte fuer Byte.
    #[must_use]
    pub fn schreiben(&self) -> String {
        self.notizen.schreiben()
    }

    /// Der Eintrag an `stelle` mit seinen sechs Werten, oder `None`, wenn
    /// es ihn nicht gibt.
    #[must_use]
    pub fn geheimnis(&self, stelle: usize) -> Option<Geheimnis<'a>> {
        let block = self.notizen.bloecke().get(stelle)?;
        Some(Teile::aus(block)?.geheimnis(block))
    }

    /// Alle Eintraege in der Reihenfolge der Datei.
    fn alle(&self) -> impl Iterator<Item = Geheimnis<'a>> + '_ {
        (0..self.anzahl()).filter_map(|stelle| self.geheimnis(stelle))
    }
}

/// Ein Eintrag in seine Teile zerlegt, ohne die Notizen schon
/// zusammenzusetzen.
struct Teile<'a> {
    /// Der Name, roh.
    name: &'a str,
    /// Die Werte der vier Feldzeilen, leer fuer eine fehlende.
    felder: [&'a str; 4],
    /// Wie viele Zeilen nach der Themenzeile Feldzeilen sind.
    feldzeilen: usize,
}

impl<'a> Teile<'a> {
    /// Zerlegt den Block nach der Leseregel des Modulkopfs; `None`, wenn
    /// seine Kopfzeile keine Themenzeile ist, was nach
    /// [`Notizen::lesen`] nicht vorkommt.
    fn aus(block: &Block<'a>) -> Option<Self> {
        let kopf = block.kopf();
        let name = kopf.strip_suffix('\n').unwrap_or(kopf);
        let name = name.strip_prefix(THEMENMARKE)?;
        let (textzeilen, _) = text_und_trenner(block.anhang());
        let mut felder = [""; 4];
        let mut gesehen = [false; 4];
        let mut feldzeilen = 0;
        for zeile in textzeilen {
            match feldzeile(zeile) {
                Some((stelle, wert)) if !gesehen[stelle] => {
                    gesehen[stelle] = true;
                    felder[stelle] = wert;
                    feldzeilen += 1;
                }
                _ => break,
            }
        }
        Some(Self {
            name,
            felder,
            feldzeilen,
        })
    }

    /// Die Notizzeilen des Blocks, roh: der Text nach den Feldzeilen ohne
    /// den Trenner.
    fn notizzeilen<'b>(&self, block: &'b Block<'a>) -> &'b [&'a str] {
        let (textzeilen, _) = text_und_trenner(block.anhang());
        &textzeilen[self.feldzeilen..]
    }

    fn geheimnis(&self, block: &Block<'a>) -> Geheimnis<'a> {
        let mut notizen = self.notizzeilen(block).concat();
        if notizen.ends_with('\n') {
            let _ = notizen.pop();
        }
        Geheimnis {
            name: self.name,
            protokoll: self.felder[0],
            adresse: self.felder[1],
            konto: self.felder[2],
            passwort: self.felder[3],
            notizen,
        }
    }
}

/// Die Zeile als Feldzeile gelesen: die Stelle ihres Schluessels in
/// [`FELDSCHLUESSEL`] und ihr Wert, oder `None`, wenn sie keine ist.
fn feldzeile(zeile: &str) -> Option<(usize, &str)> {
    let zeile = zeile.strip_suffix('\n').unwrap_or(zeile);
    let zeile = zeile.strip_suffix('\r').unwrap_or(zeile);
    let (schluessel, rest) = zeile.split_once(':')?;
    let stelle = FELDSCHLUESSEL
        .iter()
        .position(|bekannt| bekannt.eq_ignore_ascii_case(schluessel))?;
    let wert = if rest.is_empty() {
        rest
    } else {
        rest.strip_prefix(' ')?
    };
    Some((stelle, wert))
}

/// Die vier Feldzeilen in der Grundform, jede samt Umbruch.
fn feldzeilen_in_grundform(felder: [&str; 4]) -> String {
    let mut zeilen = String::with_capacity(grundform_laenge(felder));
    feldzeilen_anhaengen(&mut zeilen, felder);
    zeilen
}

/// Wie viele Bytes die vier Feldzeilen in der Grundform hoechstens tragen;
/// damit waechst eine Zeichenkette, die sie aufnimmt, nicht unterwegs um und
/// laesst keine Abschrift im freigegebenen Speicher zurueck.
fn grundform_laenge(felder: [&str; 4]) -> usize {
    FELDSCHLUESSEL
        .iter()
        .zip(felder)
        .map(|(schluessel, wert)| schluessel.len() + wert.len() + 3)
        .sum()
}

/// Haengt die vier Feldzeilen in der Grundform an `zeilen` an.
fn feldzeilen_anhaengen(zeilen: &mut String, felder: [&str; 4]) {
    for (schluessel, wert) in FELDSCHLUESSEL.iter().zip(felder) {
        zeilen.push_str(schluessel);
        zeilen.push(':');
        if !wert.is_empty() {
            zeilen.push(' ');
            zeilen.push_str(wert);
        }
        zeilen.push('\n');
    }
}

/// Weist einen Wert eines einzeiligen Feldes mit Zeilenumbruch ab.
fn einzeilig_pruefen(wert: &str) -> Result<(), Abweisung> {
    if wert.contains(['\n', '\r']) {
        Err(Abweisung::UmbruchImGeheimnisfeld)
    } else {
        Ok(())
    }
}

/// Weist Notizen mit einer Zeile ab, die einen Eintrag eroeffnete.
fn notizen_pruefen(notizen: &str) -> Result<(), Abweisung> {
    if notizen.split('\n').any(ist_themenzeile) {
        Err(Abweisung::KopfzeileInGeheimnisnotizen)
    } else {
        Ok(())
    }
}

/// Haengt einen Eintrag in der Grundform ans Ende des Klartexts; er ist
/// danach gewaehlt. Das Dateiende bleibt wie bei
/// [`notizen::hinzufuegen`], dessen Rumpf das hier ist.
///
/// Ob der Name frei ist, fragt diese Handlung nicht; das fragt der Rufer
/// mit [`name_frei`], denn der Editor legt mit „Eintrag hinzufuegen“ einen
/// leeren Eintrag an, und dessen leerer Name darf mehrfach stehen.
#[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts hinzugefuegt"]
pub fn hinzufuegen(stand: &str, entwurf: &Geheimnisentwurf) -> Result<Neustand, Abweisung> {
    for wert in [
        entwurf.name.as_str(),
        &entwurf.protokoll,
        &entwurf.adresse,
        &entwurf.konto,
        entwurf.passwort.als_text(),
    ] {
        einzeilig_pruefen(wert)?;
    }
    let notizen = entwurf.notizen.trim_end_matches('\n');
    notizen_pruefen(notizen)?;
    // Der Text traegt das neue Passwort: er tilgt sich beim Fallen und ist
    // gleich so gross angelegt, dass er unterwegs nicht umzieht.
    let felder = [
        entwurf.protokoll.as_str(),
        &entwurf.adresse,
        &entwurf.konto,
        entwurf.passwort.als_text(),
    ];
    let mut text = Zeroizing::new(String::with_capacity(
        grundform_laenge(felder) + notizen.len(),
    ));
    feldzeilen_anhaengen(&mut text, felder);
    text.push_str(notizen);
    if text.ends_with('\n') {
        let _ = text.pop();
    }
    Ok(notizen::angehaengt(stand, &entwurf.name, &text))
}

/// Ersetzt den Wert in der Spalte `feld` des Eintrags an `stelle` und
/// schreibt den Eintrag in der Grundform (Kopf dieses Moduls).
///
/// `Ok(None)`, wenn es den Eintrag nicht gibt oder er den Wert schon
/// traegt; das wird **vor** der Pruefung gefragt, so dass eine unveraendert
/// verlassene Zelle mit dem rohen `\r` eines Namens aus einer CRLF-Datei
/// keine Abweisung ergibt. Ein geaenderter Wert eines einzeiligen Feldes
/// mit Zeilenumbruch ist [`Abweisung::UmbruchImGeheimnisfeld`], geaenderte
/// Notizen mit einer Zeile `## …` sind
/// [`Abweisung::KopfzeileInGeheimnisnotizen`]. Notizen werden ohne ihre
/// Schlussumbrueche genommen, wie bei [`notizen::aendern`].
#[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts geaendert"]
pub fn aendern(
    stand: &str,
    stelle: usize,
    feld: Feld,
    wert: &str,
) -> Result<Option<Neustand>, Abweisung> {
    let geheimnisse = Geheimnisse::lesen(stand);
    let Some(block) = geheimnisse.notizen.bloecke().get(stelle) else {
        return Ok(None);
    };
    let Some(teile) = Teile::aus(block) else {
        return Ok(None);
    };
    let wert = match feld {
        Feld::Notizen => wert.trim_end_matches('\n'),
        _ => wert,
    };
    if teile.geheimnis(block).wert(feld) == wert {
        return Ok(None);
    }
    match feld {
        Feld::Notizen => notizen_pruefen(wert)?,
        _ => einzeilig_pruefen(wert)?,
    }

    let mut ersatz = if feld == Feld::Name {
        notizen::themenzeile(wert)
    } else {
        let mut kopf = block.kopf().to_owned();
        if !kopf.ends_with('\n') {
            kopf.push('\n');
        }
        kopf
    };
    let mut felder = teile.felder;
    if let Some(stelle) = feld.feldzeile() {
        felder[stelle] = wert;
    }
    ersatz.push_str(&feldzeilen_in_grundform(felder));
    if feld == Feld::Notizen {
        ersatz.push_str(&notizen::textzeilen(wert));
    } else {
        let alte = teile.notizzeilen(block).concat();
        ersatz.push_str(&alte);
        if !alte.is_empty() && !ersatz.ends_with('\n') {
            ersatz.push('\n');
        }
    }
    let (_, trenner) = text_und_trenner(block.anhang());
    ersatz.push_str(&trenner.concat());

    Ok(Some(Neustand {
        text: geheimnisse
            .notizen
            .zerlegung
            .mit_block(stelle, Some(&ersatz), stand),
        auswahl: Some(stelle),
    }))
}

/// Entfernt den Eintrag an `stelle` ganz, wie [`notizen::loeschen`] eine
/// Notiz.
#[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts geloescht"]
pub fn loeschen(stand: &str, stelle: usize) -> Option<Neustand> {
    notizen::loeschen(stand, stelle)
}

/// Tauscht den Eintrag an `stelle` mit seinem Nachbarn, wie
/// [`notizen::verschieben`] eine Notiz.
#[must_use = "der Neustand ist die ganze Wirkung; fallengelassen ist nichts verschoben"]
pub fn verschieben(stand: &str, stelle: usize, richtung: Richtung) -> Option<Neustand> {
    notizen::verschieben(stand, stelle, richtung)
}

/// Der Eintrag mit dem Namen `name`, verglichen ohne Leerraum an den
/// Enden beider Seiten.
///
/// **Der Name ist der Schluessel und muss eindeutig sein**: tragen ihn
/// mehrere Eintraege, ist die Antwort [`Suchfehler::Mehrdeutig`] mit ihrer
/// Zahl, traegt ihn keiner, [`Suchfehler::Keiner`]. Geraten wird nie.
pub fn finden(stand: &str, name: &str) -> Result<Verbindungsdaten, Suchfehler> {
    let name = name.trim();
    let geheimnisse = Geheimnisse::lesen(stand);
    let mut treffer = geheimnisse
        .alle()
        .filter(|geheimnis| geheimnis.name.trim() == name);
    let Some(erster) = treffer.next() else {
        return Err(Suchfehler::Keiner);
    };
    let weitere = treffer.count();
    if weitere > 0 {
        return Err(Suchfehler::Mehrdeutig(weitere + 1));
    }
    Ok(Verbindungsdaten {
        name: erster.name.to_owned(),
        protokoll: erster.protokoll.to_owned(),
        adresse: erster.adresse.to_owned(),
        konto: erster.konto.to_owned(),
        passwort: Passwort::neu(erster.passwort.to_owned()),
    })
}

/// Ob kein Eintrag den Namen `name` traegt, verglichen wie in [`finden`].
#[must_use]
pub fn name_frei(stand: &str, name: &str) -> bool {
    let name = name.trim();
    !Geheimnisse::lesen(stand)
        .alle()
        .any(|geheimnis| geheimnis.name.trim() == name)
}
