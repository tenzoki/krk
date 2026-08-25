//! Die Auswertung der vier Bausteine und der Einstieg in eine Zusammenfassung.
//!
//! [`zusammenfassen`] ist der eine Weg von einem ausgewaehlten Ordner zu dem,
//! was das Vorschaufenster dort zeigt: Ordner aufloesen, Profil erkennen, die
//! Zeilen des Profils in Dateireihenfolge rechnen. Was dabei gelesen werden
//! darf, zaehlt der [`Haushalt`] mit; welche Zahlen er haelt, steht als
//! Konstanten im Elternmodul und nirgends sonst.
//!
//! ```text
//! ausgewaehlter Ordner
//!   ├─ canonicalize ──────────> die Schranke aus C3.13
//!   ├─ ist es ein Verzeichnis? ─> sonst None            (C2.6)
//!   ├─ erkennung::erkennen ───> Profil          (Leselauf 1, auf Verlangen)
//!   └─ je Zeile ein Baustein ─> Wert oder Wert::Nicht
//! ```
//!
//! Die zweite Zeile des Ablaufs ist die Zusage aus C2.6, und sie steht hier,
//! weil sie hier zu halten ist: die Erkennung ueber ein Pfadmuster sieht allein
//! auf den Pfadtext, also traefe sie eine Datei genauso wie den Ordner daneben.
//! Ein Rufer, der die Frage vor dem Aufruf stellt, haelt die Zusage fuer sich
//! und nicht fuer den naechsten Rufer.
//!
//! # Ein Ort wird je Zusammenfassung hoechstens einmal gelesen
//!
//! [`Lauf`] merkt jede Lesung nach ihrem **aufgeloesten Ort** ([`Ort`]). Wer
//! denselben Ort ein zweites Mal braucht, bekommt die Eintraege der ersten
//! Lesung, und ein zweiter Leselauf faellt nicht an. Die Regel gilt ohne
//! Ausnahme: fuer den erkannten Ordner, den drei Rufer brauchen (der zweite
//! Erkennungsdurchgang, jeder Baustein ohne Ortsangabe und der Feldbaustein,
//! der seine Datei ueber ein Namensmuster sucht), genauso wie fuer einen
//! Unterordner, den so viele Rufer brauchen, wie ihn nennen, und genauso fuer
//! einen Platzhalterlauf, den zwei Zeilen mit derselben Ortsangabe sich teilen.
//!
//! **Gelesen wird traege**, naemlich erst, wenn der erste Rufer den Ort
//! braucht. Ein Profil, dessen Zeilen alle in Unterordnern arbeiten, liest den
//! erkannten Ordner ueberhaupt nicht.
//!
//! **Die Zahl der Leselaeufe bleibt aus dem Profil ablesbar**, und zwar als die
//! Zahl der **verschiedenen** genannten Orte. Das ist die genauere Groesse als
//! die Zahl der Zeilen mit Ortsangabe, denn sie zaehlt keine Arbeit doppelt,
//! die nur einmal geschieht. Aus dieser Bauart fallen die Zahlen aus C6.7: das
//! groesste mitgelieferte Profil, das des einzelnen Circles, nennt vier
//! verschiedene Orte (der erkannte Ordner, `planning`, `decisions`, `history`)
//! und kommt damit auf vier Leselaeufe und elf Oeffnungen (der
//! Circle-Datensatz, zehn Verlaufsdateien).
//!
//! # Ein Platzhalterlauf ist ein Ort und kostet einen Leselauf
//!
//! Traegt die Ortsangabe einen Platzhalter (`*`, `*/issues`), dann ist der Ort
//! kein einzelnes Verzeichnis mehr, sondern eine **Sammlung**: gelesen wird der
//! Ordner vor dem Platzhalter, aus ihm wird jeder Eintrag vom Typ
//! [`Typ::Ordner`] genommen, das Stueck hinter dem Platzhalter angehaengt und
//! dort gelesen. Die Eintraege aller Treffer liegen danach in **einem**
//! [`Lesestand`].
//!
//! **Die Einheit der Begrenzung wechselt damit**, und das ist der Zweck der
//! Form und nicht ihr Nebenwirkung. Die Zahl der Unterordner eines Ordners
//! waechst mit dem Bestand: eine Auskunft ueber alle Runden einer Werkbank
//! kostete je Runde ein Verzeichnis, und keine feste Zahl in
//! [`super::HOECHSTENS_LESELAEUFE`] traegt das dauerhaft. Begrenzt wird die
//! Sammlung deshalb durch [`HOECHSTENS_EINTRAEGE`] — eine Zahl, die vom Bestand
//! unabhaengig ist —, und gebucht wird **ein** Leselauf.
//!
//! Daraus folgt der Preis, den diese Runde bewusst zahlt: **ein Leselauf
//! oeffnet nicht mehr genau ein Verzeichnis.** Wie viele Systemaufrufe eine
//! Zusammenfassung kostet, steht nicht mehr im Profil, sondern erst am Profil
//! **und** am Bestand; ablesbar bleibt allein ihre Form, naemlich ein Lauf
//! ueber den Ordner vor dem Platzhalter und dann einer je Treffer. Genau
//! darum nimmt [`Ortsangabe`] hoechstens **einen** Platzhalter an.
//!
//! **Der Ordner vor dem Platzhalter ist selbst ein Ort** und geht durch
//! dieselbe Merkstelle. Nennt eine andere Zeile ihn ebenfalls, wird er trotzdem
//! nur einmal gelesen.
//!
//! **Was ein Platzhalterlauf ueber sich sagt**, wenn er unvollstaendig blieb:
//! `abgeschnitten` steht, sobald eine der Teillesungen abgeschnitten war oder
//! die Sammlung die Schranke erreicht hat. Die drei Regeln aus dem Abschnitt
//! darunter tragen damit unveraendert weiter; sie fragen den Lesestand und
//! nicht, aus wie vielen Verzeichnissen er stammt.
//!
//! **Bei der Dateioeffnung faellt die Wahl anders aus**, siehe
//! [`super::HOECHSTENS_OEFFNUNGEN`]: zwei Feldbausteine auf derselben Datei
//! oeffnen sie zweimal. Der Unterschied ist keine Unachtsamkeit. Ein Ort steht
//! als Ortsangabe im Profil und ist damit vor jeder Lesung bekannt; welche
//! Datei ein Baustein oeffnet, entscheidet erst sein Muster an den gelesenen
//! Eintraegen. Ein Merker darueber haenge am Inhalt des Ordners statt am
//! Profil, und die Zahl der Oeffnungen waere nicht mehr abzulesen.
//!
//! **Der Merker lebt genau so lange wie ein [`Lauf`]**, also fuer eine
//! Zusammenfassung. Zwei Zusammenfassungen desselben Ordners nacheinander lesen
//! zweimal; alles andere zeigte dem Nutzer einen Stand von vorhin.
//!
//! # Was eine unvollstaendige Lesung sagen darf
//!
//! Ein Leselauf, der bei [`HOECHSTENS_EINTRAEGE`] abbricht, liefert eine
//! Teilauskunft, und darauf gilt **eine** Regel, dreimal angewandt: *es wird
//! nur gesagt, was die Teillesung entscheidet.*
//!
//! - Die Zaehlung liefert [`Wert::UeberGrenze`] statt [`Wert::Zahl`]. Sie sagt
//!   zweierlei und sonst nichts: dass es **mindestens** so viele sind wie die
//!   getroffenen, und dass die Lesung abgebrochen wurde. *Mehr* als die
//!   getroffenen sagt sie nicht — ein weiterer Treffer hinter dem Abbruch ist
//!   moeglich und nicht gesichert. Warum die Zahl die der Treffer ist und
//!   nicht die der Grenze, steht bei [`Wert::UeberGrenze`].
//! - Das Vorhandensein liefert `ja`, wenn es einen Treffer gefunden hat, und
//!   den Platzhalter, wenn es keinen gefunden hat. Ein Nichtfund in einer
//!   Teilliste ist kein Nichtvorhandensein.
//! - Die juengsten N liefern den Platzhalter. Die juengsten zehn einer
//!   Teilliste sind nicht die juengsten zehn.
//!
//! Der Rueckgriff auf den Platzhalter statt auf eine negative Antwort ist
//! derselbe, den [`crate::verzeichnis::sys::ist_deskriptormangel`] seit der
//! Runde 10 im Durchlauf traegt: ein Mangel von aussen laesst den Auftrag
//! **unentschieden**, statt ihn negativ zu entscheiden.
//!
//! Derselbe Satz gilt fuer den erschoepften Haushalt: ein Baustein, dessen
//! Leselauf oder dessen Oeffnungen nicht mehr hineinpassen, liefert den
//! Platzhalter und keine halbe Antwort. Die juengsten N nehmen ihre Oeffnungen
//! deshalb **in einem Zug oder gar nicht**: eine Liste aus drei von zehn
//! Titeln stuende unter der Beschriftung „die juengsten zehn" und laese sich
//! als „es sind nur drei".
//!
//! # Was ein Name entscheidet und was eine Datei
//!
//! Zwei Bausteine sehen auf Namen, zwei lesen Dateien, und daran haengt der
//! Umgang mit Verknuepfungen:
//!
//! - **Zaehlung und Vorhandensein** sehen auf die Namen aller Eintraege, gleich
//!   welchen Typs. Eine Verknuepfung zaehlt mit, denn sie steht im Ordner.
//! - **Juengste N und Feld** lesen Dateien und nehmen dafuer allein Eintraege
//!   vom Typ [`Typ::Datei`]. Eine Verknuepfung wird uebergangen, aus demselben
//!   Grund, aus dem der Durchlauf nicht in sie absteigt: sie fuehrt aus dem
//!   Ordner heraus, den die Zusammenfassung beschreibt.
//!
//! **Dieselbe Naht traegt den Platzhalter**: die zwei, die auf Namen sehen,
//! nehmen ihn an, die zwei, die Dateien lesen, nicht. Wer eine Datei liest,
//! braucht ihren Pfad, und den traegt ein Lesestand nicht mehr, in dem die
//! Eintraege mehrerer Ordner zusammenliegen. Abgewiesen wird das beim Laden
//! (`datei::ortsangabe_ohne_platzhalter`); [`Lauf::in_einem_ordner`] ist die
//! zweite Haelfte derselben Aussage, an der Stelle, an der die Bauart sie
//! haelt.
//!
//! Die zweite Haelfte derselben Frage ist die Ortsangabe eines Bausteins
//! (C3.13). Sie wird beim Laden textlich geprueft ([`Ortsangabe::aus_angabe`])
//! und hier **aufgeloest** gegen den aufgeloesten erkannten Ordner gehalten;
//! erst das entscheidet ueber eine Verknuepfung im Weg, die im Text nicht
//! dasteht. Fuer einen Platzhalterlauf gilt sie an jedem Treffer einzeln: das
//! Stueck hinter dem Platzhalter wird aufgeloest und gegen die Wurzel gehalten
//! wie jede andere Ortsangabe.
//!
//! # Der Deskriptorhaushalt (C6.9)
//!
//! Zu keinem Zeitpunkt steht mehr als ein Verzeichnis- und ein Dateideskriptor
//! offen. [`crate::verzeichnis::leser::lesen_hoechstens`] oeffnet und schliesst
//! innerhalb seines Aufrufs, [`crate::text::datei::anlesen`] ebenso, und keine
//! Stelle hier haelt einen Ordner offen, waehrend sie eine Datei liest:
//! gelesen wird erst der Ordner, dann aus seinen Eintraegen eine Datei nach der
//! anderen. Wer daraus eine Liste offener Dateien macht, holt sich den Defekt
//! `260815-0211` in seiner naechsten Gestalt.
//!
//! **Der Platzhalterlauf aendert daran nichts**, obwohl er viele Verzeichnisse
//! liest: er haelt die *Eintraege* des Ordners vor dem Platzhalter und keinen
//! Deskriptor darauf, und die Treffer liest er einen nach dem anderen.
//! Dieselbe Wahl trifft `verzeichnis/durchlauf.rs`, das seine Unterordner als
//! **Pfad** auf einem Stapel vormerkt, statt den Leser der uebergeordneten
//! Ebene offen zu halten.

use std::cell::{Cell, OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use regex::Regex;

use crate::text::datei::anlesen;
use crate::verzeichnis::leser::{self, Lesestand};
use crate::verzeichnis::{Eintrag, Typ};

use super::erkennung::erkennen;
use super::{
    Baustein, HOECHSTENS_BYTES, HOECHSTENS_EINTRAEGE, Haushalt, Ortsangabe, Profile, Wert,
    Zusammenfassung, Zusammenfassungszeile,
};

// ---------------------------------------------------------------------------
// Der Einstieg
// ---------------------------------------------------------------------------

/// Die Zusammenfassung eines ausgewaehlten Ordners, oder `None`.
///
/// `None` heisst: kein Profil greift, und die Vorschau zeigt die heutige
/// Metadatenanzeige (C2.5). Denselben Weg nimmt ein Ordner, der sich nicht
/// aufloesen laesst, denn ohne aufgeloesten Ordner gibt es keine Schranke, an
/// der C3.13 zu messen waere.
///
/// **Ein Eintrag, der aufgeloest kein Verzeichnis ist, bekommt nie eine
/// Zusammenfassung** (C2.6), auch dann nicht, wenn sein Pfad ein Pfadmuster
/// erfuellt. Die Frage wird hier entschieden und nicht beim Aufrufer: der
/// erste Erkennungsdurchgang sieht allein auf den Pfadtext und braucht keine
/// Eintraege, also traefe ein Profil mit Pfadmuster auch eine Datei. Eine
/// Zusage, die an einem Zweig des einen heutigen Rufers haengt, faellt mit dem
/// zweiten, und die Vorschau ist nicht der einzige denkbare.
///
/// **Der Pfad, den der Nutzer ausgewaehlt hat, und der aufgeloeste Pfad sind
/// zwei verschiedene Dinge, und jeder hat seine Aufgabe.** Das Pfadmuster der
/// Erkennung laeuft gegen den ausgewaehlten Pfad, denn der steht im
/// Fenstertitel und den meint der Nutzer, wenn er sein Muster schreibt; die
/// Kopfzeile der Zusammenfassung zeigt ihn aus demselben Grund. Gelesen und
/// verglichen wird dagegen am aufgeloesten Pfad, denn nur er beantwortet, ob
/// eine Ortsangabe im Ordner bleibt.
#[must_use = "die Zusammenfassung ist das Ergebnis des ganzen Lesens; wer sie \
              fallen laesst, hat den Ordner umsonst gelesen"]
pub fn zusammenfassen(profile: &Profile, ordner: &Path) -> Option<Zusammenfassung> {
    zusammenfassen_gezaehlt(profile, ordner).map(|(zusammenfassung, _)| zusammenfassung)
}

/// Wie [`zusammenfassen`], aber mit dem verbrauchten [`Haushalt`] daneben.
///
/// Der Einstieg ist an dieser Naht geteilt, damit die Zaehlproben zu C6 den
/// Haushalt eines Laufs auslesen koennen, statt eine zweite Zaehlstelle neben
/// die eine zu stellen, die es ohnehin gibt. **Gemessen wird damit derselbe
/// Lauf, den die Vorschau faehrt**, und nicht ein zweiter daneben:
/// [`zusammenfassen`] ist diese Funktion ohne ihre zweite Haelfte und hat
/// keinen eigenen Rumpf mehr.
///
/// Das ist die Antwort auf C6.8, die verlangt, dass die Zahlen aus C6.1 bis
/// C6.7 durch Proben belegt sind, die **Aufrufe** zaehlen und keine
/// Millisekunden. Eine Probe, die Leselaeufe und Oeffnungen selbst mitzaehlte,
/// zaehlte, was sie erwartet, und nicht, was geschieht.
///
/// **Die Anzeige ruft sie nicht.** Sie hat mit dem verbrauchten Haushalt
/// nichts zu tun; fuer sie ist [`zusammenfassen`] da, das die zweite Haelfte
/// des Paares fallen laesst.
#[must_use = "das Paar ist das Ergebnis des ganzen Lesens; wer es fallen laesst, hat \
              den Ordner umsonst gelesen"]
pub fn zusammenfassen_gezaehlt(
    profile: &Profile,
    ordner: &Path,
) -> Option<(Zusammenfassung, Haushalt)> {
    let wurzel = std::fs::canonicalize(ordner).ok()?;
    // C2.6, am aufgeloesten Pfad und nicht am ausgewaehlten: eine Verknuepfung
    // auf eine Datei ist eine Datei. Der Aufruf kostet einen Systemaufruf je
    // Zusammenfassung und keinen Leselauf und keine Oeffnung; der Haushalt aus
    // C6 zaehlt die zwei letzteren.
    if !std::fs::metadata(&wurzel).is_ok_and(|angaben| angaben.is_dir()) {
        return None;
    }
    let lauf = Lauf::neu(&wurzel);

    let profil = erkennen(profile, ordner, &|| lauf.eintraege())?;
    let zeilen = profil
        .zeilen()
        .iter()
        .map(|zeile| {
            let wert = match zeile.baustein() {
                Some(baustein) => lauf.rechnen(baustein),
                // Beim Laden abgewiesen: die Beschriftung bleibt, der Wert ist
                // der Platzhalter (C3.12).
                None => Wert::Nicht,
            };
            Zusammenfassungszeile::neu(zeile.beschriftung().to_owned(), wert)
        })
        .collect();

    let zusammenfassung = Zusammenfassung::neu(ordnername(ordner), ordner.to_path_buf(), zeilen);
    Some((zusammenfassung, lauf.haushalt.get()))
}

/// Der Name des Ordners fuer die Kopfzeile (Festlegung A6).
///
/// Die Wurzel des Dateisystems hat keinen letzten Namensbestandteil; dort
/// steht der Pfad selbst, so wie ihn die Metadatenanzeige zeigen wuerde.
fn ordnername(ordner: &Path) -> String {
    match ordner.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => ordner.to_string_lossy().into_owned(),
    }
}

// ---------------------------------------------------------------------------
// Der Lauf: ein Ordner, ein Haushalt, die gemerkten Leselaeufe
// ---------------------------------------------------------------------------

/// Der aufgeloeste Ort, an dem ein Baustein arbeitet.
///
/// Er entsteht aus einer [`Ortsangabe`] in [`Lauf::ort_aufloesen`] und ist
/// zugleich der Schluessel, unter dem [`Lauf`] seine Lesungen merkt: zwei
/// Zeilen mit derselben Angabe treffen denselben Wert und teilen sich damit die
/// eine Lesung.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig. Sie steht dafuer,
/// dass ein Ort seit der Runde 18 nicht mehr dasselbe ist wie ein Verzeichnis.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Ort {
    /// Ein einzelnes Verzeichnis, aufgeloest und innerhalb der Wurzel.
    Einer(PathBuf),
    /// Jeder Unterordner von `ueber`, je um `hinter` verlaengert, zu einem
    /// Lesestand zusammengelegt. Ein leeres `hinter` heisst: die Unterordner
    /// selbst.
    Gestreut {
        /// Der aufgeloeste Ordner vor dem Platzhalter.
        ueber: PathBuf,
        /// Die Namensbestandteile hinter dem Platzhalter.
        hinter: Vec<String>,
    },
}

/// Was eine einzelne Zusammenfassung ueber ihren Lauf mitfuehrt.
///
/// Die drei Felder mit Innenveraenderlichkeit sind kein Zufall der Bequemlich-
/// keit: [`erkennen`] nimmt die Eintraege als Abschluss ueber `&self`
/// entgegen, damit der Leselauf erst beim ersten Profil mit Kennzeichendatei
/// anfaellt. Ein `&mut self` waere durch diesen Abschluss nicht zu reichen.
struct Lauf<'w> {
    /// Der aufgeloeste erkannte Ordner. Er ist zugleich die Schranke, innerhalb
    /// derer jede Ortsangabe bleiben muss (C3.13).
    wurzel: &'w Path,
    /// Was dieser Lauf schon verbraucht hat.
    haushalt: Cell<Haushalt>,
    /// Jeder Ort, den dieser Lauf gelesen hat, nach aufgeloestem [`Ort`], jeder
    /// beim ersten Bedarf angefordert. `None` an einem Eintrag heisst „steht
    /// nicht zur Verfuegung": der Haushalt war erschoepft oder das Lesen ist
    /// gescheitert. Auch das wird gemerkt, denn ein zweiter Versuch am selben
    /// Ort scheiterte genauso und kostete einen weiteren Leselauf.
    ///
    /// Eine Liste und keine Abbildung: es sind hoechstens so viele Eintraege,
    /// wie [`super::HOECHSTENS_LESELAEUFE`] zulaesst, und bei zwoelf davon ist
    /// das Durchgehen der guenstigere Weg als das Streuen.
    ///
    /// Der [`Lesestand`] steht unter [`Rc`], weil ihn zwei Stellen brauchen:
    /// diese Liste haelt ihn, und ein Rufer bekommt ihn geliehen. Ohne den
    /// gemeinsamen Besitz muesste der Rufer die Liste ausgeliehen halten,
    /// waehrend er rechnet, und ein spaeterer Baustein, der dabei einen
    /// zweiten Ort braucht, liefe in einen Ausleihfehler zur Laufzeit. Ein
    /// Platzhalterlauf braucht ihn dazu **waehrend** seiner eigenen Lesung: er
    /// fragt diese Liste nach dem Ordner vor dem Platzhalter, waehrend er
    /// gerade selbst darin aufgenommen wird.
    staende: RefCell<Vec<(Ort, Option<Rc<Lesestand>>)>>,
    /// Derselbe Stand fuer den erkannten Ordner, ein zweites Mal gehalten.
    ///
    /// **Keine zweite Lesung**, sondern ein geliehener Handgriff auf den
    /// Eintrag aus `staende`: [`erkennen`] erwartet die Eintraege als
    /// Ausschnitt, der so lange lebt wie der Lauf, und den gibt eine
    /// [`RefCell`] nicht heraus. Eine [`OnceCell`] gibt ihn.
    wurzelstand: OnceCell<Option<Rc<Lesestand>>>,
}

impl<'w> Lauf<'w> {
    fn neu(wurzel: &'w Path) -> Self {
        Self {
            wurzel,
            haushalt: Cell::new(Haushalt::neu()),
            staende: RefCell::new(Vec::new()),
            wurzelstand: OnceCell::new(),
        }
    }

    /// Die Eintraege eines Ortes, je Lauf hoechstens einmal gelesen.
    ///
    /// Die eine Stelle, an der ein Leselauf ueberhaupt angefordert wird. Wer
    /// den Ort schon gelesen hat, bekommt den gemerkten Stand; wer der erste
    /// ist, bezahlt ihn. Der Ort kommt **aufgeloest** herein, sonst waeren zwei
    /// Schreibweisen desselben Ortes zwei Eintraege.
    fn stand_am(&self, ort: &Ort) -> Option<Rc<Lesestand>> {
        let gemerkt = self
            .staende
            .borrow()
            .iter()
            .find(|(gelesen, _)| gelesen == ort)
            .map(|(_, stand)| stand.clone());
        if let Some(stand) = gemerkt {
            return stand;
        }
        let stand = self.lesen(ort).map(Rc::new);
        self.staende.borrow_mut().push((ort.clone(), stand.clone()));
        stand
    }

    /// Die Eintraege des erkannten Ordners, hoechstens einmal gelesen.
    fn stand(&self) -> Option<&Lesestand> {
        self.wurzelstand
            .get_or_init(|| self.stand_am(&Ort::Einer(self.wurzel.to_path_buf())))
            .as_deref()
    }

    /// Dieselben Eintraege als Ausschnitt, so wie [`erkennen`] sie erwartet.
    fn eintraege(&self) -> Option<&[Eintrag]> {
        self.stand().map(|stand| stand.eintraege.as_slice())
    }

    /// Liest einen Ort auf Kosten des Haushalts.
    ///
    /// Gebucht wird der **Versuch**: der Systemaufruf faellt an, ob er gelingt
    /// oder nicht, und der Haushalt begrenzt die Arbeit und nicht den Erfolg.
    /// Gebucht wird **ein** Leselauf, auch fuer eine Sammlung ueber viele
    /// Verzeichnisse; der Modulkopf schreibt aus, warum die Einheit dort
    /// wechselt.
    ///
    /// Die Fallunterscheidung ueber die zwei Ortsformen ist vollstaendig und
    /// hat keinen Auffangzweig.
    fn lesen(&self, ort: &Ort) -> Option<Lesestand> {
        if !self.buchen(Haushalt::leselauf_nehmen) {
            return None;
        }
        match ort {
            Ort::Einer(pfad) => leser::lesen_hoechstens(pfad, HOECHSTENS_EINTRAEGE).ok(),
            Ort::Gestreut { ueber, hinter } => self.gestreut_lesen(ueber, hinter),
        }
    }

    /// Legt die Eintraege aller getroffenen Ordner zu einem Lesestand zusammen.
    ///
    /// Der Ordner vor dem Platzhalter geht durch [`Lauf::stand_am`] wie jeder
    /// andere Ort und wird deshalb hoechstens einmal gelesen, auch wenn eine
    /// andere Zeile ihn ebenfalls nennt.
    ///
    /// **Gegriffen wird allein, was vom Typ [`Typ::Ordner`] ist.** Eine
    /// Verknuepfung wird damit uebergangen, ohne dass es dafuer eine eigene
    /// Pruefung braeuchte, und C3.13 haelt an dieser Stelle durch Bauart: ein
    /// wirklicher Unterordner eines Ordners innerhalb der Schranke liegt
    /// innerhalb der Schranke. Das Stueck **hinter** dem Platzhalter kann
    /// dagegen sehr wohl hinausfuehren und geht deshalb durch dieselbe
    /// Aufloesung wie jede andere Ortsangabe.
    ///
    /// Ein Ordner, den es hinter dem Platzhalter nicht gibt, wird uebergangen
    /// und nicht zum Abbruch: eine Runde ohne Defektspeicher ist eine Runde
    /// ohne Defekte und keine Auskunft, die fehlt.
    fn gestreut_lesen(&self, ueber: &Path, hinter: &[String]) -> Option<Lesestand> {
        let elternstand = self.stand_am(&Ort::Einer(ueber.to_path_buf()))?;
        let mut eintraege = Vec::new();
        // Eine abgeschnittene Elternlesung heisst, dass Unterordner fehlen, und
        // damit ist auch die Sammlung darueber unvollstaendig.
        let mut abgeschnitten = elternstand.abgeschnitten;
        for treffer in elternstand
            .eintraege
            .iter()
            .filter(|eintrag| eintrag.typ == Typ::Ordner)
        {
            let offen = HOECHSTENS_EINTRAEGE - eintraege.len();
            if offen == 0 {
                abgeschnitten = true;
                break;
            }
            let mut pfad = ueber.join(&treffer.name);
            for teil in hinter {
                pfad.push(teil);
            }
            let Some(pfad) = self.innerhalb(&pfad) else {
                continue;
            };
            let Ok(teilstand) = leser::lesen_hoechstens(&pfad, offen) else {
                continue;
            };
            abgeschnitten |= teilstand.abgeschnitten;
            eintraege.extend(teilstand.eintraege);
        }
        Some(Lesestand {
            eintraege,
            abgeschnitten,
        })
    }

    /// Bucht etwas im Haushalt und sagt, ob es noch hineinpasste.
    ///
    /// Die eine Stelle, an der der Haushalt aus seiner Zelle geholt, veraendert
    /// und zurueckgelegt wird. Ohne sie stuende dieses Dreierpaar an jedem
    /// Rufer, und ein vergessenes Zuruecklegen bliebe still.
    fn buchen(&self, nehmen: impl FnOnce(&mut Haushalt) -> bool) -> bool {
        let mut haushalt = self.haushalt.get();
        let gelungen = nehmen(&mut haushalt);
        self.haushalt.set(haushalt);
        gelungen
    }

    /// Loest einen Pfad auf und haelt ihn gegen die Wurzel (C3.13, zweite
    /// Haelfte).
    ///
    /// `None` heisst: es gibt ihn nicht, oder er liegt aufgeloest ausserhalb
    /// des erkannten Ordners. Eine Zusammenfassung liest nie ausserhalb des
    /// Ordners, den sie beschreibt. Die eine Stelle, an der diese Schranke
    /// gemessen wird; die zwei Rufer sind die Ortsangabe eines Bausteins und
    /// jeder einzelne Treffer eines Platzhalterlaufs.
    fn innerhalb(&self, pfad: &Path) -> Option<PathBuf> {
        let aufgeloest = std::fs::canonicalize(pfad).ok()?;
        aufgeloest.starts_with(self.wurzel).then_some(aufgeloest)
    }

    /// Der Ordner, in dem ein Baustein arbeitet, aufgeloest und geprueft.
    ///
    /// Die Wurzel selbst ist schon aufgeloest und braucht die Pruefung nicht.
    fn zielordner(&self, teile: &[String]) -> Option<PathBuf> {
        if teile.is_empty() {
            return Some(self.wurzel.to_path_buf());
        }
        let mut pfad = self.wurzel.to_path_buf();
        for teil in teile {
            pfad.push(teil);
        }
        self.innerhalb(&pfad)
    }

    /// Der Ort eines Bausteins, aufgeloest: ein Ordner oder eine Sammlung.
    fn ort_aufloesen(&self, ort: &Ortsangabe) -> Option<Ort> {
        let basis = self.zielordner(ort.teile())?;
        match ort.hinter_dem_platzhalter() {
            None => Some(Ort::Einer(basis)),
            Some(hinter) => Some(Ort::Gestreut {
                ueber: basis,
                hinter: hinter.to_vec(),
            }),
        }
    }

    /// Fuehrt eine Rechnung am Ort eines Bausteins aus.
    ///
    /// Sie kostet **hoechstens** einen Leselauf (C6.1) und keinen, wenn dieser
    /// Lauf den Ort schon gelesen hat — ob als erkannten Ordner, als Ortsangabe
    /// einer frueheren Zeile oder als denselben Platzhalterlauf, denn alle Wege
    /// gehen durch [`Lauf::stand_am`] und alle finden ihn unter demselben
    /// aufgeloesten [`Ort`].
    ///
    /// **Der Rechner bekommt keinen Pfad**, und das ist der Unterschied zu
    /// [`Lauf::in_einem_ordner`]: eine Sammlung ueber mehrere Verzeichnisse hat
    /// keinen. Wer einen braucht, nimmt jene Stelle.
    fn am_ort<T>(&self, ort: &Ortsangabe, rechnen: impl FnOnce(&Lesestand) -> T) -> Option<T> {
        let aufgeloest = self.ort_aufloesen(ort)?;
        let stand = self.stand_am(&aufgeloest)?;
        Some(rechnen(&stand))
    }

    /// Dasselbe fuer die zwei Bausteine, die **einen** Ordner brauchen.
    ///
    /// Wer eine Datei liest, braucht ihren Pfad, und den gibt es nur, wenn alle
    /// Eintraege aus demselben Verzeichnis stammen. Eine Ortsangabe mit
    /// Platzhalter wird deshalb schon beim Laden abgewiesen und erreicht diese
    /// Stelle nicht; kommt sie doch hier an, liefert der Baustein seinen
    /// Platzhalterwert, statt dass hier ein Pfad erfunden wuerde, unter dem
    /// keine der Dateien liegt.
    fn in_einem_ordner<T>(
        &self,
        ort: &Ortsangabe,
        rechnen: impl FnOnce(&Path, &Lesestand) -> T,
    ) -> Option<T> {
        let aufgeloest = self.ort_aufloesen(ort)?;
        let Ort::Einer(pfad) = &aufgeloest else {
            return None;
        };
        let stand = self.stand_am(&aufgeloest)?;
        Some(rechnen(pfad, &stand))
    }

    /// Rechnet einen einzelnen Baustein.
    ///
    /// Die Fallunterscheidung ueber die vier Bausteine ist vollstaendig und hat
    /// keinen Auffangzweig; ein fuenfter Baustein haelt den Bau hier an.
    #[must_use = "der Wert ist die Antwort dieser Profilzeile"]
    fn rechnen(&self, baustein: &Baustein) -> Wert {
        match baustein {
            Baustein::Zaehlung { ort, muster } => self
                .am_ort(ort, |stand| zaehlen(stand, muster.as_ref()))
                .unwrap_or(Wert::Nicht),
            Baustein::Juengste {
                ort,
                muster,
                anzahl,
            } => self
                .in_einem_ordner(ort, |pfad, stand| {
                    self.juengste(pfad, stand, muster.as_ref(), *anzahl)
                })
                .unwrap_or(Wert::Nicht),
            Baustein::Feld {
                ort,
                datei,
                feldmuster,
            } => self
                .in_einem_ordner(ort, |pfad, stand| self.feld(pfad, stand, datei, feldmuster))
                .unwrap_or(Wert::Nicht),
            Baustein::Vorhandensein { ort, muster } => self
                .am_ort(ort, |stand| vorhandensein(stand, muster))
                .unwrap_or(Wert::Nicht),
        }
    }

    /// B2: die N Eintraege mit dem juengsten Aenderungsdatum, je mit Titel.
    #[must_use = "der Wert ist die Antwort dieser Profilzeile"]
    fn juengste(
        &self,
        ordner: &Path,
        stand: &Lesestand,
        muster: Option<&Regex>,
        anzahl: u8,
    ) -> Wert {
        if stand.abgeschnitten {
            return Wert::Nicht;
        }
        let mut kandidaten: Vec<&Eintrag> = stand
            .eintraege
            .iter()
            .filter(|eintrag| eintrag.typ == Typ::Datei)
            .filter(|eintrag| muster.is_none_or(|muster| muster.is_match(&eintrag.name)))
            .collect();
        // Absteigend nach Aenderungsdatum, bei gleichem Zeitpunkt aufsteigend
        // nach Namen. Der zweite Schluessel steht allein fuer die Bestimmtheit
        // der Reihenfolge da und nicht fuer eine Anzeige, deshalb genuegt der
        // Byte-Vergleich und es braucht den Kollationsschluessel nicht.
        kandidaten.sort_by(|links, rechts| {
            rechts
                .geaendert
                .cmp(&links.geaendert)
                .then_with(|| links.name.cmp(&rechts.name))
        });
        kandidaten.truncate(usize::from(anzahl));
        if kandidaten.is_empty() {
            return Wert::Nicht;
        }
        // In einem Zug oder gar nicht, siehe den Modulkopf.
        let wie_viele = u32::try_from(kandidaten.len()).unwrap_or(u32::MAX);
        if !self.buchen(|haushalt| haushalt.oeffnungen_nehmen(wie_viele)) {
            return Wert::Nicht;
        }
        Wert::Titel(
            kandidaten
                .iter()
                .map(|eintrag| titel(&ordner.join(&eintrag.name), &eintrag.name))
                .collect(),
        )
    }

    /// B3: die erste Fanggruppe des ersten Treffers im Inhalt einer Datei.
    ///
    /// **Die erste passende Datei ist die erste in der Lesereihenfolge**, und
    /// die gibt das Dateisystem vor. Ein Muster, auf das mehrere Eintraege
    /// passen, hat damit keine zugesagte Wahl unter ihnen; die mitgelieferten
    /// Profile verankern ihre Dateimuster deshalb an beiden Enden.
    #[must_use = "der Wert ist die Antwort dieser Profilzeile; wer ihn fallen laesst, hat \
                  eine Datei umsonst geoeffnet"]
    fn feld(&self, ordner: &Path, stand: &Lesestand, datei: &Regex, feldmuster: &Regex) -> Wert {
        let Some(eintrag) = stand
            .eintraege
            .iter()
            .find(|eintrag| eintrag.typ == Typ::Datei && datei.is_match(&eintrag.name))
        else {
            return Wert::Nicht;
        };
        if !self.buchen(|haushalt| haushalt.oeffnungen_nehmen(1)) {
            return Wert::Nicht;
        }
        let Some(text) = angelesener_text(&ordner.join(&eintrag.name)) else {
            return Wert::Nicht;
        };
        match feldmuster
            .captures(&text)
            .and_then(|treffer| treffer.get(1))
        {
            Some(gruppe) => Wert::Text(gruppe.as_str().to_owned()),
            None => Wert::Nicht,
        }
    }
}

// ---------------------------------------------------------------------------
// Die zwei Bausteine, die allein auf Namen sehen
// ---------------------------------------------------------------------------

/// B1: die Zahl der Eintraege, deren Name das Muster erfuellt.
///
/// Ohne Muster zaehlt sie alle. Sie laeuft flach ueber eine Ebene und nicht
/// ueber den Unterbaum (Festlegung A2, C3.2).
///
/// Eine Zahl **0** ist eine Antwort und kein Fehlschlag: der Ordner steht da
/// und ist gelesen, es trifft nur nichts darin. Der Platzhalter bleibt dem
/// Fall vorbehalten, in dem der Ordner selbst nicht zu lesen war.
#[must_use = "der Wert ist die Antwort dieser Profilzeile"]
fn zaehlen(stand: &Lesestand, muster: Option<&Regex>) -> Wert {
    let gezaehlt = stand
        .eintraege
        .iter()
        .filter(|eintrag| muster.is_none_or(|muster| muster.is_match(&eintrag.name)))
        .count() as u64;
    if stand.abgeschnitten {
        Wert::UeberGrenze(gezaehlt)
    } else {
        Wert::Zahl(gezaehlt)
    }
}

/// B4: ob ein Eintrag das Muster erfuellt.
///
/// Ein Treffer entscheidet auch in einer Teilliste; ein Nichtfund darin nicht.
#[must_use = "der Wert ist die Antwort dieser Profilzeile"]
fn vorhandensein(stand: &Lesestand, muster: &Regex) -> Wert {
    if stand
        .eintraege
        .iter()
        .any(|eintrag| muster.is_match(&eintrag.name))
    {
        return Wert::Vorhanden(true);
    }
    if stand.abgeschnitten {
        return Wert::Nicht;
    }
    Wert::Vorhanden(false)
}

// ---------------------------------------------------------------------------
// Aus Bytes ein Titel und ein Feld
// ---------------------------------------------------------------------------

/// Der Titel einer Datei, ersatzweise ihr Name.
///
/// Der Titel ist die **erste nicht leere Zeile**, ein fuehrendes `#` und die
/// Leerzeichen dahinter fallen weg (Nutzerentscheid vom 260824-0610). Die
/// Ueberschriftenzeile allein genuegte nicht: das Dateiformat der
/// Defektdatensaetze schreibt eine nackte Titelzeile ohne `#` vor, und in der
/// Werkbank, an der die Runde gemessen hat, traegt keiner von 82 Datensaetzen
/// im gemeinsamen Speicher eine Markdown-Ueberschrift.
///
/// Auf den Dateinamen faellt die Regel in drei Lagen zurueck, und in allen
/// dreien ist er die einzige Auskunft, die es gibt: die Datei ist leer, sie
/// laesst sich nicht lesen, oder sie ist kein Text.
fn titel(pfad: &Path, name: &str) -> String {
    angelesener_text(pfad)
        .as_deref()
        .and_then(titelzeile)
        .unwrap_or_else(|| name.to_owned())
}

/// Die erste Zeile, aus der nach dem Abraeumen noch etwas uebrig bleibt.
fn titelzeile(text: &str) -> Option<String> {
    text.lines().find_map(|zeile| {
        let gekuerzt = zeile.trim().trim_start_matches('#').trim();
        (!gekuerzt.is_empty()).then(|| gekuerzt.to_owned())
    })
}

/// Die ersten [`HOECHSTENS_BYTES`] Bytes einer Datei als Text (C6.6).
///
/// `None` heisst: nicht lesbar oder kein Text.
fn angelesener_text(pfad: &Path) -> Option<String> {
    let bytes = anlesen(pfad, HOECHSTENS_BYTES).ok()?;
    lesbarer_anfang(&bytes).map(str::to_owned)
}

/// Der als UTF-8 lesbare Anfang einer angelesenen Bytefolge.
///
/// **Ein unvollstaendiges Zeichen am Ende ist die Naht des Deckels und kein
/// Befund ueber die Datei.** [`anlesen`] schneidet nach einer Zahl von Bytes
/// ab, nicht nach Zeichen; faellt der Schnitt mitten in ein mehrbytiges
/// Zeichen, waere die ganze Datei nach [`String::from_utf8`] „kein Text",
/// obwohl ihre erste Zeile tadellos dasteht. Ein ungueltiges Byte **mitten**
/// im Gelesenen bleibt dagegen der Befund, der es ist, und liefert `None`.
///
/// Die Unterscheidung steht hier und nicht bei
/// [`crate::verzeichnis::inhalt::traegt_der_inhalt`]: jene Stelle liest ueber
/// `bis_zur_grenze_lesen`, das eine zu grosse Datei abweist statt sie
/// abzuschneiden, und kennt die Naht deshalb nicht.
fn lesbarer_anfang(bytes: &[u8]) -> Option<&str> {
    match std::str::from_utf8(bytes) {
        Ok(text) => Some(text),
        Err(fehler) if fehler.error_len().is_none() => {
            std::str::from_utf8(&bytes[..fehler.valid_up_to()]).ok()
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leseprofil::HOECHSTENS_OEFFNUNGEN;

    #[test]
    fn der_titel_ist_die_erste_nicht_leere_zeile_ohne_ihr_doppelkreuz() {
        assert_eq!(
            titelzeile("# Eine Ueberschrift\n\nAbsatz\n").as_deref(),
            Some("Eine Ueberschrift")
        );
        assert_eq!(
            titelzeile("\n\n260823-1445_o_ein Defekt ohne Doppelkreuz\n").as_deref(),
            Some("260823-1445_o_ein Defekt ohne Doppelkreuz"),
            "das Defektformat schreibt eine nackte Titelzeile vor"
        );
        assert_eq!(
            titelzeile("###   Drei Kreuze\n").as_deref(),
            Some("Drei Kreuze")
        );
        assert_eq!(
            titelzeile("#\n\n# Erst hier\n").as_deref(),
            Some("Erst hier"),
            "eine Zeile, von der nichts uebrig bleibt, ist keine Titelzeile"
        );
        assert_eq!(titelzeile("").as_deref(), None);
        assert_eq!(titelzeile("\n   \n\t\n").as_deref(), None);
    }

    #[test]
    fn ein_abgeschnittenes_zeichen_am_ende_nimmt_der_datei_nicht_ihren_text() {
        let ganz = "Titel Ü".as_bytes();
        assert_eq!(
            ganz.len(),
            8,
            "sechs ASCII-Bytes und zwei fuer das Umlaut-U"
        );
        assert_eq!(lesbarer_anfang(ganz), Some("Titel Ü"));

        // Der Deckel faellt zwischen die zwei Bytes des „Ü": das erste steht
        // noch da, das zweite fehlt.
        let naht = &ganz[..7];
        assert!(
            std::str::from_utf8(naht).is_err(),
            "sonst traegt schon der erste Zweig die Antwort und der zweite bleibt ungemessen"
        );
        assert_eq!(
            lesbarer_anfang(naht),
            Some("Titel "),
            "die Naht des Deckels nimmt der Datei ihren Text"
        );

        // Der Schnitt hinter einem ganzen Zeichen geht durch den ersten Zweig.
        assert_eq!(
            lesbarer_anfang(&ganz[..6]),
            Some("Titel "),
            "der Schnitt hinter einem ganzen Zeichen ist kein Sonderfall"
        );

        // Ein ungueltiges Byte mitten im Gelesenen bleibt ein Befund.
        assert_eq!(lesbarer_anfang(&[b'a', 0xff, b'b']), None);
        assert_eq!(
            lesbarer_anfang(&[b'T', 0xc3, 0x9c, 0xff]),
            None,
            "ein ungueltiges Byte am Ende ist keine Naht, sondern ein Befund"
        );
    }

    #[test]
    fn der_haushalt_deckelt_die_oeffnungen_und_nimmt_sie_ganz_oder_gar_nicht() {
        let mut haushalt = Haushalt::neu();
        assert!(haushalt.oeffnungen_nehmen(HOECHSTENS_OEFFNUNGEN - 1));
        assert!(
            !haushalt.oeffnungen_nehmen(2),
            "zwei passen nicht mehr, also wird keine genommen"
        );
        assert_eq!(haushalt.oeffnungen(), HOECHSTENS_OEFFNUNGEN - 1);
        assert!(haushalt.oeffnungen_nehmen(1));
        assert_eq!(haushalt.oeffnungen(), HOECHSTENS_OEFFNUNGEN);
    }
}
