//! Die Zerlegung von Markdown fuer die Vorschau (C4 der Runde 6): aus dem
//! Quelltext einer Datei werden der Text, der zu sehen ist, und die Stellen,
//! die eine Auszeichnung tragen.
//!
//! **Keine Zeile AppKit.** Wie [`crate::hervorhebung`] daneben steht hier keine
//! `use`-Zeile aus einer Objective-C-Bindungskiste. Diese Datei rechnet, und
//! `crate::appkit::textmerkmale` setzt das Ergebnis in Merkmale um — dieselbe
//! Umsetzung, die der Editor benutzt, und keine zweite daneben. Deshalb ist
//! [`Gerendert::formatierung`] die vorhandene [`Formatierung`] und kein eigener
//! Typ: eine Ueberschrift sieht in der Vorschau aus wie im Editor.
//!
//! # Warum ein Zerleger und nicht die Kiste, die schon da ist
//!
//! Steht in der Wurzel-`Cargo.toml` an der Versionsangabe von
//! `pulldown-cmark`, mit den drei geprueften Mitteln und den vier erhobenen
//! Zahlen. Kurz: `syntect` ist mit 0,3 MB/s zu langsam fuer einen Text, der
//! sofort dastehen soll, und welche Zeichen es als Auszeichnung ausblenden
//! wuerde, entscheiden fremde Sprachdefinitionen.
//!
//! # Die Regel der Zerlegung
//!
//! Ein Durchgang ueber [`pulldown_cmark::OffsetIter`], der zu jedem Ereignis
//! auch den Quellbereich liefert:
//!
//! ```text
//!   Quelltext
//!       │  Parser::new_ext(.., Options::empty()).into_offset_iter()
//!       v
//!   ┌───────────────────────────────────────────────────────────────┐
//!   │ Ueberschrift   ──> Auszeichnung::Ueberschrift { stufe }       │
//!   │ Listenpunkt    ──> Merkzeichen + Listenzeile { tiefe }        │
//!   │ Zitatblock     ──> Auszeichnung::Listenzeile { tiefe }        │
//!   │ Quelltext      ──> Auszeichnung::FesteSchrift                 │
//!   │ Betonung       ──> Auszeichnung::Betonung                     │
//!   │ starke Bet.    ──> Auszeichnung::StarkeBetonung               │
//!   │ Verweis        ──> Einfaerbung (Farbe der Tafel, unterstr.)   │
//!   │ weicher/harter                                                │
//!   │   Umbruch      ──> "\\n"                                       │
//!   │ alles Uebrige  ──> der Quellbereich, woertlich                │
//!   └───────────────────────────────────────────────────────────────┘
//! ```
//!
//! **Die letzte Zeile ist die eine Auffangregel, und sie macht die
//! Fallunterscheidung ueber [`Tag`] total.** Was ausserhalb des Grundumfangs
//! liegt, erscheint als der Text, der dasteht, und wird bis zum Ende seines
//! Elements uebersprungen: eingebettetes HTML, Bilder und Trennlinien. Das
//! dritte Abnahmekriterium von C4 verlangt genau eine Regel dafuer und nicht
//! eine je Fall.
//!
//! **Eine Tabelle faellt nicht unter die Auffangregel, sondern kommt von
//! selbst richtig heraus.** Ohne das Merkmal `ENABLE_TABLES` sieht die Kiste in
//! den drei Zeilen einer Tabelle einen gewoehnlichen Absatz mit weichen
//! Umbruechen; die Zwischenraeume bleiben stehen, und das Quelltextraster aus
//! dem Datensatz vom 260812-1105 entsteht ohne Sonderregel.
//!
//! # Die Deckung: was aus der Quelle herausfaellt, und was nicht
//!
//! Die Auffangregel ueber [`Tag`] deckt die **Ereignisse** ab und damit noch
//! nicht die **Zeichen** der Datei. Bis zum 260812 fiel deshalb Quelltext
//! spurlos heraus, zu dem die Kiste gar kein Ereignis liefert — eine
//! Verweisdefinition `[ref]: https://…` wird beim Aufloesen verbraucht und
//! nicht gemeldet, und eine Datei aus lauter Definitionen zeigte eine leere
//! Flaeche (Defekt `260812-1805`, zwei gemessene Faelle).
//!
//! **Gedeckt wird deshalb ueber die Quellbereiche und nicht ueber die
//! Ereignisarten.** [`Zerlegung::gelesen`] ist der Stand, bis zu dem die Quelle
//! abgetragen ist, und daran haengen drei Saetze:
//!
//! 1. **Auf Dokumentebene**, also wenn kein Element offen ist, wird die Luecke
//!    vor dem naechsten Ereignis ausgegeben ([`Zerlegung::luecke_bis`]); am
//!    Ende des Durchgangs ebenso bis [`str::len`]. Eine Luecke, in der nur
//!    Leerraum steht, faellt weg: die Abstaende zwischen den Bloecken rechnet
//!    [`Zerlegung::absetzen`], und sie ein zweites Mal zu schreiben ergaebe
//!    Leerzeilen. **Der Einzug einer Zeile bleibt dort stehen**, denn hier
//!    wiederholt keine Umgebung etwas, und der Leerraum ist Inhalt — die
//!    Fortsetzungszeile einer mehrzeiligen Verweisdefinition behaelt ihn.
//! 2. **Innerhalb eines Elements, das Bloecke enthaelt** — Zitatblock, Liste,
//!    Listenpunkt, also [`Inhaltsart::Bloecke`] —, gilt derselbe Satz: zwischen
//!    zwei Kindern und hinter dem letzten kann Quelltext stehen, den die Kiste
//!    nicht meldet, und er wird ausgegeben. Beim naechsten Ereignis erledigt
//!    das [`Zerlegung::luecke_bis`], am Ende des Elements
//!    [`Zerlegung::schliessen`]. Was die Umgebung auf jeder Zeile wiederholt,
//!    faellt dabei weg: der Einzug eines Punktes und das `>` eines Zitats
//!    ([`ohne_umgebungszeichen`]).
//! 3. **Innerhalb eines Elements, das seine eigenen Zeichen traegt** — Absatz,
//!    Ueberschrift, Quelltextblock, Betonung, Verweis, also
//!    [`Inhaltsart::Zeichen`] —, sind die Luecken darin seine
//!    Auszeichnungszeichen und gehoeren weg: das `[` und das `][ref]` eines
//!    Verweises. Hat es **kein** Zeichen geliefert, gibt es beim Schliessen
//!    seinen Quellbereich woertlich heraus; so bleibt
//!    `[](https://example.com)` stehen, statt zu verschwinden.
//!
//! Die drei Saetze zaehlen keine Faelle auf: der erste fragt allein, ob
//! [`Zerlegung::offen`] leer ist, der zweite und der dritte allein nach
//! [`Offen::inhalt`], und der letzte Halbsatz allein danach, ob die Laenge
//! null ist. Eine kuenftige Fassung der Kiste, die ein Element anders meldet,
//! aendert daran nichts.
//!
//! **Wo die Deckung endet, und das ist genau eine Stelle:** der **Vorspann**
//! eines Elements aus Satz 2, also alles von seinem Anfang bis zu dem Byte,
//! das als erstes darin gelesen wird. Dort steht sein eigenes Merkzeichen —
//! `- `, `1. `, `> ` —, und das gehoert weg. Steht dort daneben Quelltext, den
//! die Kiste nicht meldet, so faellt er heraus: eine Verweisdefinition **vor**
//! dem ersten Absatz eines Punktes ist weg, eine dahinter steht da. Ein Punkt,
//! der kein einziges Zeichen geliefert hat, faellt nicht darunter — er gibt
//! nach Satz 3 seinen Quellbereich woertlich heraus, sein `- ` eingeschlossen,
//! und so kommt die Verweisdefinition darin ueberhaupt heraus.
//!
//! **Traegt jener Quellbereich nichts als das Merkzeichen selbst**, so ist
//! nichts herauszugeben, was nicht das Merkzeichen waere, und statt des rohen
//! `- ` wird der Wunsch eingeloest: ein leerer Punkt steht als `• ` da und
//! rueckt ein wie jeder andere ([`traegt_nur_sein_merkzeichen`], Defekt
//! `260812-2019`). Die Frage dahinter ist eine einzige und keine Aufzaehlung
//! von Faellen: steht hinter dem ersten Stueck der Quelle noch eines?
//!
//! # Die Abstaende zwischen den Bloecken
//!
//! Ein Block **verlangt** eine Zahl von Umbruechen vor und nach sich —
//! Absaetze, Ueberschriften, Zitate und Quelltextbloecke zwei, Listen und ihre
//! Punkte einen. Geschrieben wird nichts davon sofort: der Wunsch wird
//! aufgehoben und erst vor dem naechsten Zeichen eingeloest, und dabei
//! **aufgefuellt statt angehaengt**. So traegt ein Block, dessen Quelltext
//! schon mit einem Umbruch endet, hinterher keine Leerzeile zuviel, und zwei
//! Wuensche hintereinander werden nicht zu vier Zeilen.
//!
//! # Die Einrueckebenen einer Liste
//!
//! Ein Listenpunkt traegt sein Merkzeichen im Text — `• ` oder die Nummer
//! seiner geordneten Liste — und seine Tiefe in der Auszeichnung
//! ([`Auszeichnung::Listenzeile`]). Bis zum 260812 trug er einen festen Einzug
//! und sonst nichts; eine dreistufige Liste stand danach flach und ohne
//! Merkzeichen da, und eine geordnete war von einer ungeordneten nicht zu
//! unterscheiden (Defekt `260812-1805`).
//!
//! **Das Merkzeichen ist ein Wunsch und kein Text**, genau wie die Trennung
//! daneben: geschrieben wird es erst vor dem ersten Zeichen des Punktes
//! ([`Zerlegung::merkzeichen_einloesen`]). In einer **losen** Liste — einer
//! mit Leerzeilen zwischen den Punkten, und das ist jede, deren Punkte mehr
//! als eine Zeile tragen — schiebt die Kiste zwischen Punkt und Text einen
//! Absatz, und der verlangt zwei Umbrueche. Sofort geschrieben, stuende das
//! Merkzeichen danach allein auf seiner Zeile (Defekt `260812-1920`). Ein
//! Punkt, der ueberhaupt kein Zeichen liefert, loest seinen Wunsch nur dann
//! ein, wenn sein Quellbereich nichts als das Merkzeichen traegt; sonst tritt
//! der Quellbereich an seine Stelle, und darin steht das Merkzeichen der
//! Quelle schon.
//!
//! **Das Merkzeichen gehoert seinem Punkt und nicht seinem ersten Kind.** Was
//! innerhalb des Punktes offen steht, rueckt beim Einloesen dahinter nach
//! ([`Zerlegung::merkzeichen_einloesen`]); ohne diesen Nachzug deckte die
//! Auszeichnung des ersten Kindes das Merkzeichen mit ab, und `- **fett**`
//! setzte den Aufzaehlungspunkt fett (Defekt `260812-2019`). Der Nachzug ist
//! nicht derselbe wie der in [`Zerlegung::absetzen`]: ein Abstand gehoert
//! keinem der offenen Elemente, also ruecken dort **alle** nach, das
//! Merkzeichen gehoert seinem Punkt, also ruecken hier nur die **inneren**
//! nach.
//!
//! Die Tiefe zaehlt [`Zerlegung::tiefe`] aus den offenen Elementen und haelt
//! keinen zweiten Zaehler daneben: jedes Element, das einrueckt, traegt eine
//! [`Ebene`], und die Tiefe ist ihre Zahl. Ein Zitatblock zaehlt mit, weil er
//! genauso einrueckt; die Nummer einer geordneten Liste steht in der [`Ebene`]
//! der Liste und wird vom Punkt verbraucht.
//!
//! # Die Stellen sind UTF-16-Einheiten
//!
//! Wie in [`crate::hervorhebung`], und aus demselben Grund: ein `NSRange` zaehlt
//! UTF-16-Einheiten, und ein Umlaut oder ein Emoji verschoebe jede Stelle
//! dahinter. Gezaehlt wird **im Durchgang** und nicht in einem zweiten danach;
//! [`Formatierung::laenge`] ist der Endstand dieses Zaehlers und traegt damit
//! denselben Guertel gegen einen Programmabbruch, den der Editor schon hat.
//!
//! # Der Quellbezug: die zweite Auskunft desselben Durchgangs
//!
//! Neben dem Text entsteht im selben Durchgang der [`Quellbezug`]. Er sagt zu
//! jeder Stelle des gerenderten Textes, aus welchen Bytes der Quelle sie
//! stammt, und er ist die Grundlage dafuer, dass eine Auswahl in der Vorschau
//! den Quelltext mit seinen Auszeichnungszeichen in die Zwischenablage legt
//! (C2 der Runde 14).
//!
//! **Er entsteht hier und nicht in einem zweiten Durchgang**, und das ist eine
//! Frage der Entscheidbarkeit und keine der Bequemlichkeit: allein hier ist
//! bekannt, welche Bytes ein geschriebenes Zeichen hervorgebracht haben. Aus
//! dem fertigen Text zurueckzurechnen ginge nicht, denn `**` und `# ` stehen
//! dort nicht mehr.
//!
//! Die Abbildung ist eine **Kachelung** aus [`Abschnitt`]en mit zwei Zusagen:
//!
//! 1. Die Quellbereiche der Abschnitte reihen sich lueckenlos und
//!    ueberschneidungsfrei ueber `0..quelle.len()`.
//! 2. Die Textbereiche der Abschnitte reihen sich ebenso ueber
//!    `0..formatierung.laenge`.
//!
//! Beide fallen aus der Bauart heraus und nicht aus einer Nachpruefung: ein
//! Abschnitt entsteht genau dann, wenn einer der beiden Zaehler vorrueckt,
//! [`Zerlegung::kacheln`] ist die eine Stelle dafuer, und beide Zaehler ruecken
//! nur vorwaerts. Damit hat jede Stelle des Textes genau eine Antwort und jedes
//! Byte der Quelle genau einen Ort; ein Auffangzweig „keine Antwort" entsteht
//! nicht. Nachgemessen wird beides von der Kachelungsprobe im Pruefmodul, ueber
//! einen Satz von zehn Beispielen.
//!
//! **Jede Luecke bekommt deshalb einen Abschnitt, obwohl sie nicht dasteht.**
//! Fuer die Anzeige faellt sie weg, und der Absatz „Wo die Deckung endet" oben
//! gilt unveraendert; fuers Kopieren dreht sich das Vorzeichen,
//! denn `# `, `**`, `[`, `- ` und `> ` sind Bytes der Quelle und wollen einen
//! Ort. Sie bekommen ihn als Abschnitt mit leerem Textbereich, und die Anzeige
//! aendert sich davon nicht. Das gilt fuer den Vorspann eines Elements aus
//! Bloecken wie fuer die Auszeichnungszeichen innerhalb eines Elements aus
//! Zeichen: beide traegt [`Zerlegung::luecke_bis`] ab, ohne etwas zu
//! schreiben. Damit steht jedes geschriebene Stueck Zeichen fuer Zeichen an
//! seiner Quelle, statt auf die Auszeichnung davor aufzurunden.
//!
//! **Die Klammer** ([`Quellelement`]) ist die zweite Auskunft des Durchgangs:
//! sie sagt zu jedem Element, ob es an seinen **Raendern** Zeichen traegt, die
//! im Text nicht wiederkehren — vor dem ersten Ereignis in seinem Quellbereich
//! sein Vorspann, hinter dem letzten sein Nachspann. Was dazwischen verdeckt
//! bleibt, zaehlt nicht: eine Entitaet oder ein Escape mitten in einem Absatz
//! zerschneidet nichts, denn es steht ganz in dem Stueck, das eine Auswahl
//! ohnehin liefert. Ein Absatz mit einer starken Betonung darin traegt selbst
//! keine Klammer, eine Ueberschrift traegt ihr `# ` auch dann, wenn gleich ein
//! Kind darauf folgt, und genau daran haengt das Beispiel des bindenden
//! Datensatzes — siehe [`Zerlegung::ereignis_verbuchen`] und
//! [`klammer_der_raender`].
//!
//! **Beantwortet wird daraus genau eine Frage**, und [`Quellbezug::quelltext`]
//! ist der eine oeffentliche Zugang: zu dieser Auswahl gehoert dieser
//! Quelltext. Die Oberflaeche rechnet nichts. Der Rechenweg hat zwei Stufen —
//! die Huelle ueber die beruehrten Abschnitte, dann der Fixpunkt ueber die
//! Elemente mit Klammer —, und beide stehen dort ausgeschrieben.
//!
//! **Der eine Rufer sitzt in der Oberflaeche und heisst
//! `Vorschautext::auswahl_ablegen`** (`appkit/vorschau.rs`), die
//! Ueberschreibung, an der AppKit jede Auswahl aus der Vorschau ablegt. Die
//! Rechnung selbst bleibt davon unberuehrt: sie ist ohne AppKit pruefbar
//! (C2.5), und die Proben unten fahren sie.

use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::hervorhebung::{
    Auszeichnung, Auszeichnungsstelle, Darstellungsart, Einfaerbung, Farbe, Formatierung, Tafel,
    linkfarbe,
};

/// Ein gerendertes Markdown: der Text, der zu sehen ist, und seine Stellen.
///
/// **Die Formatierung ist die vorhandene aus [`crate::hervorhebung`]** und
/// keine eigene. Damit gibt es eine Umsetzung in AppKit-Merkmale und nicht
/// zwei; siehe den Modulkopf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gerendert {
    /// Der Text ohne die Auszeichnungszeichen, so wie er dasteht.
    pub text: String,
    /// Welche Stelle von [`Gerendert::text`] was traegt.
    pub formatierung: Formatierung,
    /// Woher jede Stelle von [`Gerendert::text`] stammt.
    ///
    /// **Geteilt statt kopiert.** `crate::vorschaumodell::Inhalt` wird bei
    /// jedem Neuzeichnen des aktiven Tabs geklont; ein zweiter Textspeicher bis
    /// zur Vorschaugrenze von 1 MB im Klon waere teuer, ein Zaehlerschritt ist
    /// es nicht. Es ist derselbe Griff, den die Bilddaten seit der Runde 1 tun.
    ///
    /// **[`Arc`] und nicht `Rc`**, weil der Arbeitsfaden `krk-vorschau` den
    /// Wert baut und durch einen Kanal schickt.
    ///
    /// **Ein Feld und kein Nachbarwert in der Aufzaehlung:** der Quellbezug
    /// gehoert zu genau diesem gerenderten Text, und als Feld kann er von ihm
    /// nicht getrennt werden. Daneben verlangt C2.13, dass die Abbildung auf
    /// der Seite des Textes liegt und nicht auf der der Einfaerbung.
    pub quellbezug: Arc<Quellbezug>,
}

/// Woher jede Stelle des gerenderten Textes stammt (C2 der Runde 14).
///
/// Der Aufbau und die beiden Zusagen der Kachelung stehen im Modulkopf unter
/// „Der Quellbezug: die zweite Auskunft desselben Durchgangs".
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Quellbezug {
    /// Die Quelle, aus der gerendert wurde.
    ///
    /// Sie wird nicht ein zweites Mal von der Platte gelesen, sondern ist die
    /// Eingabe des Durchgangs (C2.3).
    quelle: String,
    /// Die Kachelung.
    ///
    /// Ihre Quellbereiche decken `0..quelle.len()` lueckenlos und
    /// ueberschneidungsfrei, ihre Textbereiche `0..formatierung.laenge` ebenso
    /// (C2.6).
    abschnitte: Vec<Abschnitt>,
    /// Die Elemente, die der Durchgang geoeffnet hat, in der Reihenfolge des
    /// Oeffnens. Traeger der Klammerregel (C2.9).
    elemente: Vec<Quellelement>,
}

impl Quellbezug {
    /// Der Quelltext zu einer Auswahl im gerenderten Text (C2.2, C2.9).
    ///
    /// **Der eine oeffentliche Zugang zum Quellbezug.** Die Oberflaeche reicht
    /// die Grenzen ihrer Auswahl herein — ein `NSRange` zaehlt UTF-16-Einheiten
    /// und traegt damit schon die Koordinaten dieser Abbildung — und bekommt
    /// den Ausschnitt der Quelle heraus, den sie ablegt. Gerechnet wird hier
    /// und nicht dort.
    ///
    /// Der Rechenweg hat zwei Stufen und keine dritte: erst die Huelle ueber
    /// die beruehrten Abschnitte ([`Quellbezug::huelle_der_abschnitte`]), dann
    /// der Fixpunkt ueber die Elemente mit Klammer
    /// ([`Quellbezug::klammern_schliessen`]).
    ///
    /// **Warum die zweite Stufe an der Klammer haengt und nicht an jedem
    /// Element.** Der bindende Datensatz
    /// `shared/decisions/260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md`
    /// nennt drei Moeglichkeiten, und der Nutzer hat die zweite gewaehlt: eine
    /// beruehrte Auszeichnung faehrt ganz mit. Ohne die Bedingung „traegt eine
    /// Klammer" waere daraus die **nicht** gewaehlte Moeglichkeit 3, die
    /// blockweise: ein Absatz ist auch ein Element, und jede Auswahl darin
    /// blaehte sich auf ihn auf. Der Nutzer bekaeme den ganzen Absatz, wo er
    /// zwei Woerter markiert hat. Die Klammer trennt genau das: ein Element,
    /// dessen Quellbereich keine Bytes traegt, die der Text weglaesst, kann an
    /// seinen Raendern auch nichts zerschneiden.
    ///
    /// **Warum ein Abschnitt ohne Textzeichen im geschlossenen Auswahlintervall
    /// mitfaehrt.** Das ist keine Ausnahme, sondern die einzige Lesart, unter
    /// der ein solcher Abschnitt ueberhaupt erreichbar ist: sein Textbereich
    /// ist leer, und ein leerer Bereich schneidet kein halboffenes Intervall.
    /// Nach der halboffenen Lesart fiele jeder Abschnitt heraus, der Quelle
    /// ohne Anzeige traegt — das Merkzeichen am Dateianfang, der abschliessende
    /// Zeilenumbruch —, und C2.8 (die Auswahl ueber alles liefert die Datei
    /// vollstaendig) braeuchte eine Sonderregel. Was die Halbregel dabei
    /// hereinholt, gehoert entweder einem Element mit Klammer, das die zweite
    /// Stufe danach vollstaendig macht, oder es ist der Zeilenumbruch hinter
    /// einem Block, und der schadet in keiner Zwischenablage.
    #[must_use]
    pub fn quelltext(&self, auswahl: Range<usize>) -> &str {
        &self.quelle[self.ausschnitt(&auswahl)]
    }

    /// Die beiden Stufen der Klammerregel, hintereinander.
    ///
    /// Steht als eigene Methode neben [`Quellbezug::quelltext`], damit die
    /// Regel als Bereich zu pruefen ist und nicht nur als Zeichenfolge.
    ///
    /// **Eine verdrehte Auswahl wird zur leeren.** `NSRange` kann sie nicht
    /// liefern, aber die Rechnung soll auch dann eine Antwort geben und keine
    /// Panik: ein Bereich, dessen Ende vor seinem Anfang laege, wuerde beim
    /// Zugriff auf die Quelle abbrechen.
    fn ausschnitt(&self, auswahl: &Range<usize>) -> Range<usize> {
        let auswahl = auswahl.start..auswahl.end.max(auswahl.start);
        let Some(huelle) = self.huelle_der_abschnitte(&auswahl) else {
            return 0..0;
        };
        self.klammern_schliessen(huelle)
    }

    /// Erste Stufe: die Huelle ueber die Quellbereiche der beruehrten
    /// Abschnitte.
    ///
    /// `None`, wenn kein Abschnitt beitraegt — bei einer leeren Auswahl in
    /// einem Text ohne Abschnitt ohne Textzeichen an ihrer Stelle, und bei
    /// einer leeren Quelle.
    ///
    /// Eine Huelle und keine Vereinigung: die Abschnitte kacheln die Quelle
    /// lueckenlos, also liegt zwischen zwei beruehrten Abschnitten nur, was
    /// ohnehin dazwischen steht.
    fn huelle_der_abschnitte(&self, auswahl: &Range<usize>) -> Option<Range<usize>> {
        let mut huelle: Option<Range<usize>> = None;
        for abschnitt in &self.abschnitte {
            let Some(beitrag) = self.beitrag(abschnitt, auswahl) else {
                continue;
            };
            huelle = Some(match huelle {
                Some(bisher) => bisher.start.min(beitrag.start)..bisher.end.max(beitrag.end),
                None => beitrag,
            });
        }
        huelle
    }

    /// Was ein einzelner Abschnitt zur Huelle beitraegt.
    ///
    /// **Die Fallunterscheidung ueber [`Abschnittsart`] ist vollstaendig und
    /// ueberschneidungsfrei**, und jeder Zweig steht fuer eine der drei
    /// Antworten, die die Art gibt:
    ///
    /// - [`Abschnittsart::Woertlich`]: beide Seiten stehen Zeichen fuer Zeichen
    ///   aneinander, also rechnet sich die Auswahlgrenze genau auf ein Byte um
    ///   ([`byte_zur_stelle`], die eine Umrechnung im Modul, C2.7).
    /// - [`Abschnittsart::Ersetzt`]: die Quelle hat den Text hervorgebracht,
    ///   ohne ihm zu gleichen, also gibt es innen nichts umzurechnen und der
    ///   Abschnitt rundet auf seine Raender.
    /// - [`Abschnittsart::Erzeugt`]: KRK hat die Zeichen gesetzt, sein
    ///   Quellbereich ist leer, er traegt nichts bei.
    fn beitrag(&self, abschnitt: &Abschnitt, auswahl: &Range<usize>) -> Option<Range<usize>> {
        if !abschnitt.beruehrt(auswahl) {
            return None;
        }
        match abschnitt.art {
            Abschnittsart::Woertlich => {
                let stueck = &self.quelle[abschnitt.quelle.clone()];
                let von = auswahl
                    .start
                    .clamp(abschnitt.text.start, abschnitt.text.end)
                    - abschnitt.text.start;
                let bis = auswahl.end.clamp(abschnitt.text.start, abschnitt.text.end)
                    - abschnitt.text.start;
                let anfang = abschnitt.quelle.start + byte_zur_stelle(stueck, von);
                let ende = abschnitt.quelle.start + byte_zur_stelle(stueck, bis);
                Some(anfang..ende)
            }
            Abschnittsart::Ersetzt => Some(abschnitt.quelle.clone()),
            Abschnittsart::Erzeugt => None,
        }
    }

    /// Zweite Stufe: der Fixpunkt ueber die Elemente mit Klammer (C2.9).
    ///
    /// > Erweitere den Quellausschnitt so lange auf die Huelle mit dem ganzen
    /// > Quellbereich jedes Elements, das eine Klammer traegt, das der
    /// > Ausschnitt schneidet und das er nicht ganz enthaelt, bis er sich nicht
    /// > mehr aendert.
    ///
    /// **Das Verfahren endet**, und zwar aus zwei Gruenden zusammen: der
    /// Ausschnitt waechst allein — jeder Durchgang, der etwas aendert,
    /// vergroessert ihn um mindestens ein Byte —, und die Quelle ist endlich.
    /// Ein Element, das der Ausschnitt einmal ganz enthaelt, bleibt darin, denn
    /// der Ausschnitt schrumpft nie; jedes Element erweitert ihn also
    /// hoechstens einmal.
    ///
    /// **Ueber verschachtelte Elemente ist es dasselbe Verfahren und keine
    /// zweite Regel.** Wer drei Buchstaben in `**fett *und kursiv* zugleich**`
    /// markiert, erweitert im ersten Durchgang auf die innere Betonung und im
    /// zweiten auf die aeussere; niemand fragt dabei nach der Schachtelung.
    fn klammern_schliessen(&self, mut ausschnitt: Range<usize>) -> Range<usize> {
        loop {
            let mut gewachsen = false;
            for element in &self.elemente {
                if !element.klammer {
                    continue;
                }
                let bereich = &element.quelle;
                // Schneidet er ihn? Halboffen gefragt, denn beide Bereiche
                // zaehlen Bytes und keiner von beiden ist eine Stelle.
                if bereich.start >= ausschnitt.end || ausschnitt.start >= bereich.end {
                    continue;
                }
                if ausschnitt.start <= bereich.start && bereich.end <= ausschnitt.end {
                    continue;
                }
                ausschnitt = ausschnitt.start.min(bereich.start)..ausschnitt.end.max(bereich.end);
                gewachsen = true;
            }
            if !gewachsen {
                return ausschnitt;
            }
        }
    }
}

/// Eine Kachel: ein Stueck gerenderter Text und die Bytes, aus denen es kam.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
struct Abschnitt {
    /// Der Bereich im gerenderten Text, in UTF-16-Einheiten.
    ///
    /// Darf leer sein: dann traegt die Quelle Zeichen, die die Anzeige
    /// weglaesst — das schliessende `**` einer Betonung, der Vorspann eines
    /// Listenpunktes.
    text: Range<usize>,
    /// Der Bereich in der Quelle, in Bytes.
    ///
    /// Darf leer sein: dann hat KRK die Zeichen erzeugt, und der leere Bereich
    /// ist ihre Verankerung.
    quelle: Range<usize>,
    art: Abschnittsart,
}

impl Abschnitt {
    /// Ob dieser Abschnitt von einer Auswahl beruehrt wird.
    ///
    /// **Zwei Lesarten fuer zwei Gestalten desselben Bereichs**, und die Frage,
    /// die sie trennt, ist eine einzige: traegt der Abschnitt Zeichen?
    ///
    /// Ein Abschnitt **mit** Zeichen wird beruehrt, wenn sein Textbereich die
    /// Auswahl im gewoehnlichen Sinn schneidet, halboffen. So faehrt der
    /// Abschnitt hinter der Auswahl nicht mit, bloss weil er an ihrem Ende
    /// beginnt.
    ///
    /// Ein Abschnitt **ohne** Zeichen wird beruehrt, wenn seine Textstelle im
    /// **geschlossenen** Auswahlintervall liegt. Das ist keine angeflickte
    /// Ausnahme, sondern die einzige Lesart, unter der er ueberhaupt erreichbar
    /// ist: sein Textbereich ist leer und schneidet nichts. Warum das so sein
    /// muss und was es hereinholt, steht an [`Quellbezug::quelltext`].
    #[must_use]
    fn beruehrt(&self, auswahl: &Range<usize>) -> bool {
        if self.text.is_empty() {
            auswahl.start <= self.text.start && self.text.start <= auswahl.end
        } else {
            self.text.start < auswahl.end && auswahl.start < self.text.end
        }
    }
}

/// Rechnet eine Stelle im Text auf ihr Byte in der Quelle um (C2.7).
///
/// **Die eine Umrechnung zwischen UTF-16-Einheiten und Bytes im Modul.** Sie
/// steht hier, weil sie nur an einer Stelle etwas zu rechnen hat: innerhalb
/// eines Abschnitts der Art [`Abschnittsart::Woertlich`], wo `stueck` beide
/// Seiten zugleich ist — der geschriebene Text und der Ausschnitt der Quelle,
/// Zeichen fuer Zeichen dieselben. Die beiden anderen Arten rechnen nichts:
/// [`Abschnittsart::Ersetzt`] rundet auf die Raender, [`Abschnittsart::Erzeugt`]
/// traegt nichts bei.
///
/// Gezaehlt wird ueber die Zeichen und nicht ueber die Bytes, denn ein Umlaut
/// zaehlt zwei Bytes und eine UTF-16-Einheit, ein Emoji vier Bytes und zwei
/// Einheiten. Eine Stelle hinter dem Ende gibt die Laenge; eine Stelle mitten
/// in einem Ersatzpaar gibt das Byte hinter dem Zeichen, statt eine ungueltige
/// Zeichengrenze zu liefern, an der der Zugriff auf die Quelle abbraeche.
#[must_use]
fn byte_zur_stelle(stueck: &str, stelle: usize) -> usize {
    let mut einheiten = 0usize;
    for (byte, zeichen) in stueck.char_indices() {
        if einheiten >= stelle {
            return byte;
        }
        einheiten += zeichen.len_utf16();
    }
    stueck.len()
}

/// Welche Seite eines Abschnitts massgeblich ist.
///
/// Die Fallunterscheidung ist vollstaendig und ueberschneidungsfrei ueber die
/// eine Frage, welche Seite den Inhalt traegt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Abschnittsart {
    /// Text und Quellausschnitt sind Zeichen fuer Zeichen dieselben.
    ///
    /// Eine Auswahlgrenze darin rechnet sich genau auf ein Byte um.
    Woertlich,
    /// Die Quelle hat diesen Text hervorgebracht, ohne ihm zu gleichen — der
    /// leere Text eingeschlossen.
    ///
    /// Eine Auswahlgrenze darin rundet auf die Raender des Abschnitts.
    Ersetzt,
    /// KRK hat diese Zeichen gesetzt; die Quelle kennt sie nicht.
    ///
    /// Sie tragen zum Quellausschnitt nichts bei, und einen Auffangzweig „keine
    /// Antwort" gibt es deshalb nicht (C2.6).
    Erzeugt,
}

/// Ein Element der Quelle und die Frage, ob es Auszeichnungszeichen traegt.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
struct Quellelement {
    /// Sein Bereich in der Quelle, in Bytes.
    quelle: Range<usize>,
    /// Ob es an seinen Raendern Bytes traegt, die in seinem gerenderten
    /// Bereich nicht erscheinen.
    ///
    /// Wahr fuer Ueberschrift, Betonung, Verweis, Listenpunkt, Zitat,
    /// Quelltextblock und das Stueck fester Schrift in der Zeile; falsch fuer
    /// einen gewoehnlichen Absatz — auch fuer einen mit einer Entitaet oder
    /// einem Escape darin — und fuer eine Liste, deren Merkzeichen ihren
    /// Punkten gehoeren. Wie sie zustande kommt, steht an
    /// [`klammer_der_raender`].
    klammer: bool,
}

/// Zerlegt eine Markdown-Quelle in den sichtbaren Text und seine Stellen (C4).
///
/// Laeuft auf dem Arbeitsfaden der Vorschau (`krk-vorschau`) und damit vor der
/// Endbedingung von L7. Gemessen kostet der Durchgang 19 bis 30 ms fuer
/// 1,05 MB; die Zahlen und ihre Erhebung stehen in der Wurzel-`Cargo.toml`.
///
/// Die Tafel entscheidet allein ueber die Farbe eines Verweises. Sie kommt von
/// aussen herein, weil dieses Modul keine Farbe kennt und die Wahl zwischen
/// Hell und Dunkel am Erscheinungsbild des Fensters haengt.
pub fn rendern(quelle: &str, tafel: Tafel) -> Gerendert {
    let mut zerlegung = Zerlegung::neu(quelle, linkfarbe(tafel));
    let mut ereignisse = Parser::new_ext(quelle, Options::empty()).into_offset_iter();
    while let Some((ereignis, bereich)) = ereignisse.next() {
        // Der erste Satz der Deckung, vor jedem Ereignis und ohne seine Art zu
        // kennen: was seit dem letzten Stand ungelesen blieb, steht da.
        zerlegung.luecke_bis(bereich.start);
        // Die Klammer, ebenfalls ohne die Art des Ereignisses zu kennen: jedes
        // Ereignis verbucht seinen Quellbereich beim umgebenden Element und
        // schiebt damit dessen Vorspann und Nachspann zusammen (C2.9, siehe
        // [`Zerlegung::ereignis_verbuchen`]).
        //
        // **Das Ende ist die eine Ausnahme**, und sie ist keine Aufzaehlung:
        // sein Bereich ist der des Elements, das sich gerade schliesst, und
        // nicht der eines Kindes darin. Verbuchte es sich selbst, haette kein
        // Element je einen Vorspann. Das `matches!` traegt hier seinen stillen
        // `_ => false` zu Recht — eine kuenftige Ereignisart der Kiste ist ein
        // Kind und will verbucht werden.
        if !matches!(ereignis, Event::End(_)) {
            zerlegung.ereignis_verbuchen(&bereich);
        }
        match ereignis {
            Event::Start(tag) => match behandlung(&tag) {
                Behandlung::Block { umbrueche, art } => {
                    zerlegung.trennen(umbrueche);
                    zerlegung.oeffnen(
                        bereich,
                        Abschluss::von(art),
                        umbrueche,
                        None,
                        Inhaltsart::Zeichen,
                    );
                }
                Behandlung::Zitat => zerlegung.zitat_oeffnen(bereich),
                Behandlung::Liste { erste } => {
                    zerlegung.trennen(PUNKTABSTAND);
                    zerlegung.oeffnen(
                        bereich,
                        Abschluss::Nichts,
                        PUNKTABSTAND,
                        Some(Ebene::Liste(erste)),
                        Inhaltsart::Bloecke,
                    );
                }
                Behandlung::Punkt => zerlegung.punkt_oeffnen(bereich),
                Behandlung::Stueck(art) => {
                    zerlegung.oeffnen(
                        bereich,
                        Abschluss::Auszeichnung(art),
                        0,
                        None,
                        Inhaltsart::Zeichen,
                    );
                }
                Behandlung::Verweis => {
                    zerlegung.oeffnen(bereich, Abschluss::Verweis, 0, None, Inhaltsart::Zeichen);
                }
                // Die eine Auffangregel: der Quelltext, woertlich, und das
                // Element danach uebersprungen. Ein Bild, das hier
                // hindurchgeht, wird damit auch nicht geladen — das sechste
                // Abnahmekriterium von C4.
                Behandlung::Woertlich => {
                    zerlegung.woertlich(bereich);
                    bis_zum_ende_ueberspringen(&mut ereignisse);
                }
            },
            Event::End(_) => zerlegung.schliessen(),
            Event::Text(inhalt) => zerlegung.schreiben(&inhalt, bereich.end),
            Event::Code(inhalt) => {
                let ende = bereich.end;
                zerlegung.oeffnen(
                    bereich,
                    Abschluss::Auszeichnung(Auszeichnung::FesteSchrift),
                    0,
                    None,
                    Inhaltsart::Zeichen,
                );
                zerlegung.schreiben(&inhalt, ende);
                zerlegung.schliessen();
            }
            // Die Zeile, an der das Quelltextraster einer Tabelle haengt: die
            // drei Zeilen einer Tabelle sind ein Absatz mit weichen
            // Umbruechen, und der Umbruch bleibt einer.
            Event::SoftBreak | Event::HardBreak => zerlegung.schreiben("\n", bereich.end),
            // Dieselbe Auffangregel fuer die Ereignisse, die kein Ende haben:
            // eine Trennlinie, eingebettetes HTML in der Zeile, alles Uebrige.
            Event::Rule
            | Event::Html(_)
            | Event::InlineHtml(_)
            | Event::FootnoteReference(_)
            | Event::InlineMath(_)
            | Event::DisplayMath(_)
            | Event::TaskListMarker(_) => zerlegung.woertlich(bereich),
        }
    }
    zerlegung.abschliessen()
}

/// Was aus einem Element wird.
///
/// **Die Fallunterscheidung ueber [`Tag`] ist total**, und der letzte Wert ist
/// der Grund: er faengt jedes Element auf, das der Grundumfang aus dem zweiten
/// Abnahmekriterium von C4 nicht nennt. Eine Aufzaehlung der Ausnahmen stuende
/// hier sonst neben der Aufzaehlung der Faelle und veraltete mit der naechsten
/// Fassung der Kiste.
enum Behandlung {
    /// Ein Block: er setzt sich von seinen Nachbarn ab und traegt hoechstens
    /// eine Auszeichnung ueber alle seine Zeilen.
    Block {
        /// Wie viele Umbrueche vor und nach ihm stehen.
        umbrueche: usize,
        /// Was seine Zeilen tragen; `None` fuer einen Block, der nur abtrennt.
        art: Option<Auszeichnung>,
    },
    /// Ein Zitatblock: er legt eine Einrueckebene an und rueckt sich selbst ein.
    ///
    /// Er bekommt den Einzug der Listen, weil sein Merkzeichen sonst spurlos
    /// verschwaende (C4, zweites Kriterium).
    Zitat,
    /// Eine Liste: sie legt eine Einrueckebene an und traegt selbst nichts.
    ///
    /// Der Einzug haengt am Punkt, denn er ist die Zeile, die eingerueckt wird.
    Liste {
        /// Die erste Nummer einer geordneten Liste; `None` bei einer
        /// ungeordneten.
        erste: Option<u64>,
    },
    /// Ein Listenpunkt: sein Merkzeichen und der Einzug seiner Ebene.
    Punkt,
    /// Ein Stueck in der Zeile, das eine Auszeichnung traegt.
    Stueck(Auszeichnung),
    /// Ein Verweis: Farbe und Unterstreichung statt einer Auszeichnung.
    ///
    /// **Keine Klickwirkung.** Welche Quellen eine Adresse setzen duerfen, ist
    /// die erste offene Frage des Web-Betrachter-Circles; sie hier nebenbei zu
    /// beantworten naehme jenem Circle seine Klaerungsrunde.
    Verweis,
    /// Die eine Auffangregel: der Quellbereich, woertlich.
    Woertlich,
}

/// Wie viele Umbrueche ein Absatz von seinen Nachbarn trennen.
///
/// Zwei, also eine Leerzeile. Sie ist der Unterschied zwischen einem
/// gerenderten Text und einer Wand.
const ABSATZABSTAND: usize = 2;

/// Wie viele Umbrueche einen Listenpunkt von seinem Vorgaenger trennen.
///
/// Einer: Punkte gehoeren zusammen, und eine Leerzeile dazwischen zerrisse die
/// Liste optisch in lauter Absaetze.
const PUNKTABSTAND: usize = 1;

/// Das Merkzeichen eines Punktes in einer ungeordneten Liste.
///
/// Mit dem Leerzeichen dahinter, weil es zum Zeichen gehoert und nicht zum
/// Text. Der Punkt `U+2022` ist derselbe, den jeder Betrachter setzt.
const AUFZAEHLUNGSZEICHEN: &str = "• ";

/// Was aus dem Element mit diesem Anfangszeichen wird.
fn behandlung(tag: &Tag<'_>) -> Behandlung {
    match tag {
        Tag::Paragraph => Behandlung::Block {
            umbrueche: ABSATZABSTAND,
            art: None,
        },
        Tag::Heading { level, .. } => Behandlung::Block {
            umbrueche: ABSATZABSTAND,
            art: Some(Auszeichnung::Ueberschrift {
                stufe: *level as u8,
            }),
        },
        Tag::BlockQuote(_) => Behandlung::Zitat,
        Tag::CodeBlock(_) => Behandlung::Block {
            umbrueche: ABSATZABSTAND,
            art: Some(Auszeichnung::FesteSchrift),
        },
        Tag::List(erste) => Behandlung::Liste { erste: *erste },
        Tag::Item => Behandlung::Punkt,
        Tag::Emphasis => Behandlung::Stueck(Auszeichnung::Betonung),
        Tag::Strong => Behandlung::Stueck(Auszeichnung::StarkeBetonung),
        Tag::Link { .. } => Behandlung::Verweis,
        _ => Behandlung::Woertlich,
    }
}

/// Verwirft die Ereignisse bis zum Ende des gerade begonnenen Elements.
///
/// Der zweite Halbsatz der Auffangregel. Der Quelltext des Elements ist bereits
/// geschrieben; alles, was die Kiste darin noch findet, stuende sonst ein
/// zweites Mal da. Gezaehlt wird die Tiefe, damit ein verschachteltes Element
/// nicht das falsche Ende trifft; die Kiste sagt zu, dass Anfang und Ende
/// ausgeglichen sind.
fn bis_zum_ende_ueberspringen(ereignisse: &mut pulldown_cmark::OffsetIter<'_>) {
    let mut tiefe = 1usize;
    for (ereignis, _) in ereignisse.by_ref() {
        match ereignis {
            Event::Start(_) => tiefe += 1,
            Event::End(_) => {
                tiefe -= 1;
                if tiefe == 0 {
                    return;
                }
            }
            _ => {}
        }
    }
}

/// Nimmt einer Luecke, was ihre Umgebung auf jeder ihrer Zeilen wiederholt.
///
/// Zeilenweise faellt vorn weg, was ein Zitatblock und ein Einzug dort
/// ohnehin stehen haben — `>` und Leerraum. Was uebrig bleibt, ist der
/// Quelltext, zu dem die Kiste kein Ereignis geliefert hat; bleibt nichts
/// uebrig, war die Luecke die Auszeichnung ihrer Umgebung und keine Zeile
/// Inhalt. Ohne diesen Griff stuende zwischen zwei Absaetzen eines Zitats das
/// nackte `>` seiner Leerzeile.
///
/// Das Merkzeichen einer Liste steht **nicht** in dieser Menge: `-` und `1.`
/// koennen der Anfang einer Zeile sein, die dasteht. Der Vorspann eines
/// Punktes faellt statt dessen ueber [`Zerlegung::luecke_bis`] weg.
///
/// **Gerufen wird nur innerhalb eines Elements**, also nur dort, wo es
/// ueberhaupt eine Umgebung gibt. Auf Dokumentebene wiederholt nichts sich,
/// und der Einzug einer Zeile ist dort Inhalt; [`Zerlegung::luecke_bis`]
/// bleibt deshalb beim blossen [`str::trim`], das nur an den beiden Enden der
/// ganzen Luecke schneidet.
fn ohne_umgebungszeichen(luecke: &str) -> String {
    let zeilen: Vec<&str> = luecke
        .lines()
        .map(|zeile| zeile.trim_start_matches([' ', '\t', '>']))
        .collect();
    zeilen.join("\n").trim().to_owned()
}

/// Ob ein Listenpunkt in seiner Quelle nichts traegt als sein Merkzeichen.
///
/// Der Quellbereich eines Punktes faengt bei seinem Merkzeichen an — `-`,
/// `*`, `+`, `1.`, `1)` —, und CommonMark laesst darauf Leerraum oder das
/// Zeilenende folgen. Das erste durch Leerraum abgetrennte Stueck ist deshalb
/// immer das Merkzeichen und nie etwas anderes; gefragt ist allein, **ob**
/// dahinter noch eines kommt.
///
/// Das ist eine Frage und keine Aufzaehlung der Merkzeichenformen: welche
/// Zeichen ein Merkzeichen ausmachen, muss hier niemand wissen, und eine
/// Form, die CommonMark spaeter hinzunimmt, aendert daran nichts.
fn traegt_nur_sein_merkzeichen(quelle: &str) -> bool {
    quelle.split_whitespace().nth(1).is_none()
}

/// Was beim Ende eines Elements einzutragen ist.
enum Abschluss {
    /// Nichts: der Block hat nur abgetrennt.
    Nichts,
    /// Eine Auszeichnung ueber den ganzen Bereich des Elements.
    Auszeichnung(Auszeichnung),
    /// Farbe und Unterstreichung eines Verweises.
    Verweis,
}

impl Abschluss {
    /// Der Abschluss zu der Auszeichnung, die ein Block traegt.
    fn von(art: Option<Auszeichnung>) -> Self {
        match art {
            Some(art) => Abschluss::Auszeichnung(art),
            None => Abschluss::Nichts,
        }
    }
}

/// Eine Einrueckebene: was die Tiefe einer Listenzeile ausmacht.
///
/// Die offenen Ebenen stehen in [`Zerlegung::offen`] und nicht in einem
/// zweiten Stapel daneben; ihre Zahl ist die Tiefe, und mit dem Element
/// verschwindet auch seine Ebene.
enum Ebene {
    /// Ein Zitatblock: er rueckt ein und zaehlt keine Nummern.
    Zitat,
    /// Eine Liste: `None` ungeordnet, `Some(n)` die naechste zu vergebende
    /// Nummer.
    Liste(Option<u64>),
}

/// Was ein Element enthaelt: andere Bloecke oder seine eigenen Zeichen.
///
/// Der Name traegt sein `-art`, weil `crate::vorschaumodell::Inhalt` in
/// derselben Kiste etwas anderes heisst — die Art dessen, was die Vorschau
/// anzeigt.
///
/// Daran haengt der zweite und der dritte Satz der Deckung, siehe Modulkopf.
/// Die Unterscheidung ist die von CommonMark zwischen einem Containerblock
/// und einem Blattblock, und sie ist die einzige, die traegt: gefragt ist
/// nicht, ob ein Element Zeichen geliefert hat, sondern ob zwischen seinen
/// Kindern ueberhaupt Quelltext stehen kann, den die Kiste nicht meldet.
enum Inhaltsart {
    /// Andere Bloecke: Zitatblock, Liste, Listenpunkt.
    ///
    /// Zwischen zwei Kindern und hinter dem letzten kann eine
    /// Verweisdefinition stehen; das Element deckt sie.
    Bloecke,
    /// Die eigenen Zeichen: Absatz, Ueberschrift, Quelltextblock und jedes
    /// Stueck in der Zeile.
    ///
    /// Sein Quellbereich gehoert ihm allein, und die Luecken darin sind seine
    /// Auszeichnungszeichen. Sie auszugeben truege das `[` und das `][ref]`
    /// eines Verweises wieder in den Text.
    Zeichen,
}

impl Inhaltsart {
    /// Ob ein Element dieser Art die Luecken in seinem Quellbereich deckt.
    ///
    /// **Ein `match` und kein `matches!`**, und darin liegt der ganze Zweck
    /// dieser Methode: ein `matches!` traegt einen stillen `_ => false`, eine
    /// dritte Variante liefe damit still als „nicht gedeckt" durch, statt den
    /// Bau anzuhalten. CommonMark kennt mit der Fussnotendefinition und der
    /// Definitionsliste weitere Containerbloecke; sie sind heute nur deshalb
    /// kein Fall, weil `Options::empty()` sie abschaltet. Wer eine Option
    /// einschaltet, soll vom Uebersetzer diese Stelle genannt bekommen — so,
    /// wie es [`Auszeichnung`] in [`crate::hervorhebung`] zusagt und wie
    /// `CLAUDE.md` es unter „Was man nicht sieht" fuer die gewachsenen
    /// Aufzaehlungen des Programms beschreibt.
    ///
    /// Sie ist zugleich die eine Lesestelle statt zweier: [`Zerlegung::luecke_bis`]
    /// und [`Zerlegung::schliessen`] fragen dieselbe Frage.
    fn deckt_luecken(&self) -> bool {
        match self {
            Inhaltsart::Bloecke => true,
            Inhaltsart::Zeichen => false,
        }
    }
}

/// Ein Element, dessen Ende noch aussteht.
struct Offen {
    /// Die Stelle in UTF-16-Einheiten, an der es begonnen hat.
    anfang: usize,
    /// Was beim Ende einzutragen ist.
    was: Abschluss,
    /// Wie viele Umbrueche nach ihm stehen.
    nach: usize,
    /// Sein Bereich in der Quelle, in Bytes.
    ///
    /// Der zweite Satz der Deckung haengt daran: ein Element ohne ein einziges
    /// Zeichen gibt genau diesen Bereich woertlich heraus.
    quelle: Range<usize>,
    /// Der wievielte geoeffnete Bereich dieser ist — sein Rang.
    ///
    /// Die Reihenfolge des Oeffnens laeuft von aussen nach innen und ist damit
    /// die Reihenfolge, in der [`Zerlegung::abschliessen`] bei gleichem Anfang
    /// und gleicher Laenge sortiert.
    ///
    /// **Zugleich sein Platz in [`Zerlegung::elemente`]**, und deshalb steht
    /// hier eine Zahl und nicht zwei: beide zaehlen dasselbe Ereignis, naemlich
    /// das Oeffnen eines Elements. Ein zweiter Zaehler daneben waere die zweite
    /// Wahrheit ueber dieselbe Sache und liefe beim ersten Element auseinander,
    /// das nur einer von beiden sieht — derselbe Grund, aus dem
    /// [`Zerlegung::tiefe`] zaehlt, statt mitzufuehren. [`Zerlegung::schliessen`]
    /// traegt darueber die Klammer in ihren Eintrag ein.
    rang: usize,
    /// Die Einrueckebene, die dieses Element anlegt.
    ebene: Option<Ebene>,
    /// Ob dieses Element andere Bloecke enthaelt oder seine eigenen Zeichen.
    inhalt: Inhaltsart,
    /// Das Merkzeichen eines Listenpunktes, solange es aussteht.
    ///
    /// Ein Wunsch wie [`Zerlegung::trennung`] und kein geschriebener Text;
    /// eingeloest wird er vor dem ersten Zeichen des Punktes. Wird der Punkt
    /// geschlossen, ohne dass ein Zeichen kam, faellt der Wunsch mit ihm weg
    /// — an seine Stelle tritt sein Quellbereich, in dem das Merkzeichen der
    /// Quelle schon steht. Traegt jener Bereich nichts als eben dieses
    /// Merkzeichen, so wird der Wunsch doch noch eingeloest; siehe
    /// [`Zerlegung::schliessen`].
    merkzeichen: Option<String>,
    /// Wo das erste Ereignis in seinem Quellbereich begonnen hat; `None`,
    /// solange keines kam.
    ///
    /// Davor liegt sein **Vorspann**: das `# ` einer Ueberschrift, die
    /// Sternchen einer Betonung, das `[` eines Verweises, das `- ` eines
    /// Punktes. Siehe [`Zerlegung::ereignis_verbuchen`].
    innen_ab: Option<usize>,
    /// Wo das letzte Ereignis in seinem Quellbereich geendet hat; anfangs sein
    /// eigener Anfang.
    ///
    /// Dahinter liegt sein **Nachspann**: die schliessenden Sternchen, das
    /// `](Ziel)` eines Verweises, der Zaun eines Quelltextblocks.
    innen_bis: usize,
}

/// Ob ein Element an seinen Raendern Zeichen traegt, die der Text weglaesst —
/// seine Klammer (C2.9).
///
/// Gefragt sind der **Vorspann** vor dem ersten Ereignis in seinem
/// Quellbereich und der **Nachspann** hinter dem letzten. Was dazwischen
/// verdeckt bleibt, geht die Klammer nichts an: es steht ganz in dem Stueck,
/// das eine Auswahl ohnehin liefert, und kann an keinem Rand zerschnitten
/// werden. Wie die beiden Staende entstehen, steht an
/// [`Zerlegung::ereignis_verbuchen`].
///
/// **Leerraum ist keine Auszeichnung.** Der Quellbereich eines Absatzes endet
/// hinter seinem Zeilenumbruch, und der steht im Text nicht mehr; ohne diesen
/// Halbsatz truege jeder Absatz eine Klammer, und daraus waere die vom Nutzer
/// nicht gewaehlte Moeglichkeit 3. Ein Merkzeichen `- ` oder ein `> ` bleibt
/// uebrig, wenn man den Leerraum abzieht, ein Zeilenumbruch nicht.
///
/// **Ein Element ohne ein einziges Ereignis darin ist kein Sonderfall.** Es
/// hat kein Inneres, also faellt sein ganzer Quellbereich in beide Spannen —
/// [`Offen::innen_ab`] steht auf `None` und liest sich als sein Ende,
/// [`Offen::innen_bis`] auf seinem Anfang. So bekommt ein Stueck fester
/// Schrift `` `code` `` seine Haken, ohne dass jemand es aufzaehlen muesste.
#[must_use]
fn klammer_der_raender(quelle: &str, eintrag: &Offen) -> bool {
    let vorspann = eintrag.quelle.start..eintrag.innen_ab.unwrap_or(eintrag.quelle.end);
    let nachspann = eintrag.innen_bis..eintrag.quelle.end;
    !quelle[vorspann].trim().is_empty() || !quelle[nachspann].trim().is_empty()
}

/// Der Ausgabetext im Aufbau, mit seinen Stellen und den offenen Elementen.
struct Zerlegung<'q> {
    /// Die Quelle, aus der gerendert wird.
    ///
    /// Sie steht hier, weil beide Saetze der Deckung sie brauchen und keiner
    /// von beiden in [`rendern`] steht.
    quelle: &'q str,
    /// Bis wohin die Quelle abgetragen ist, in Bytes.
    gelesen: usize,
    /// Die Farbe eines Verweises; `None`, wenn die Tafel keine hergibt.
    linkfarbe: Option<Farbe>,
    text: String,
    /// Die Laenge von [`Zerlegung::text`] in UTF-16-Einheiten, im Durchgang
    /// mitgezaehlt.
    stelle: usize,
    /// Wie viele Umbrueche vor dem naechsten Zeichen stehen sollen.
    ///
    /// Ein Wunsch und kein geschriebener Text; eingeloest wird er in
    /// [`Zerlegung::absetzen`], und dort wird aufgefuellt statt angehaengt.
    trennung: usize,
    offen: Vec<Offen>,
    einfaerbungen: Vec<Einfaerbung>,
    /// Die Auszeichnungen mit dem Rang ihres Elements ([`Offen::rang`]).
    auszeichnungen: Vec<(usize, Auszeichnungsstelle)>,
    /// Die Kachelung im Aufbau; sie waechst allein in [`Zerlegung::kacheln`].
    abschnitte: Vec<Abschnitt>,
    /// Die Elemente in der Reihenfolge des Oeffnens.
    ///
    /// Angelegt wird ein Eintrag in [`Zerlegung::oeffnen`], seine Klammer
    /// eingetragen in [`Zerlegung::schliessen`].
    elemente: Vec<Quellelement>,
}

impl<'q> Zerlegung<'q> {
    fn neu(quelle: &'q str, linkfarbe: Option<Farbe>) -> Self {
        Self {
            quelle,
            gelesen: 0,
            linkfarbe,
            text: String::new(),
            stelle: 0,
            trennung: 0,
            offen: Vec::new(),
            einfaerbungen: Vec::new(),
            auszeichnungen: Vec::new(),
            abschnitte: Vec::new(),
            elemente: Vec::new(),
        }
    }

    /// Legt einen Abschnitt an und traegt die Quelle bis `bis` ab.
    ///
    /// **Die eine Stelle, an der die Kachelung waechst**, und damit die eine
    /// Stelle, an der [`Zerlegung::gelesen`] vorrueckt. Der Textbereich reicht
    /// von `von` bis zur jetzigen Stelle, der Quellbereich vom bisherigen
    /// Lesestand bis `bis`. Weil beide Zaehler nur vorwaerts laufen, reihen
    /// sich die Abschnitte auf beiden Seiten lueckenlos und
    /// ueberschneidungsfrei — die zwei Zusagen aus dem Modulkopf fallen daraus
    /// heraus, statt nachtraeglich geprueft zu werden.
    ///
    /// **Ein Rueckschritt in der Quelle wird zum leeren Bereich und nicht zu
    /// einer Ueberschneidung.** Ein `bis` hinter dem Lesestand kommt vor, wenn
    /// ein Element seinen Bereich schon abgetragen hat und ein spaeteres
    /// Ereignis darin noch einmal davor zeigt.
    fn kacheln(&mut self, von: usize, bis: usize, art: Abschnittsart) {
        let quelle = self.gelesen..bis.max(self.gelesen);
        self.gelesen = quelle.end;
        self.abschnitte.push(Abschnitt {
            text: von..self.stelle,
            quelle,
            art,
        });
    }

    /// Haelt am innersten offenen Element fest, wie weit sein Inneres reicht —
    /// die eine Quelle seiner Klammer (C2.9).
    ///
    /// **Verbucht wird der Quellbereich eines Ereignisses beim Element, das es
    /// umschliesst**, und daraus wachsen dessen [`Offen::innen_ab`] und
    /// [`Offen::innen_bis`] zusammen. Was davor und dahinter uebrig bleibt,
    /// sind sein Vorspann und sein Nachspann, und allein die beiden entscheiden
    /// beim Schliessen ueber die Klammer ([`klammer_der_raender`]).
    ///
    /// **Warum die Raender und nicht die verdeckten Bytes im Inneren.** Der
    /// bindende Datensatz `shared/decisions/260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md`
    /// laesst eine beruehrte Auszeichnung ganz mitfahren, weil eine Auswahl
    /// sie sonst unbalanciert zerschnitte. Zerschneiden kann eine Auswahl aber
    /// nur, was an den **Raendern** eines Elements steht. Eine Entitaet
    /// `&amp;` oder ein Escape `\*` mitten in einem Absatz steht ganz in dem
    /// Stueck, das die Auswahl ohnehin liefert; naehme es dem Absatz eine
    /// Klammer ab, blaehte sich jede Auswahl darin auf ihn auf — die vom
    /// Nutzer nicht gewaehlte Moeglichkeit 3 (Defekt `260820-0728`).
    ///
    /// **Warum das Ereignis und nicht das geschriebene Zeichen.** Innerhalb
    /// eines Elements aus Zeichen faellt eine Luecke aus der Anzeige, und was
    /// dort geschrieben wird, laesst sich der Quelle nicht mehr Byte fuer Byte
    /// zuordnen. Der Quellbereich eines Ereignisses liegt dagegen fest und
    /// gehoert dem Kind: in `# **Titel** und mehr` beginnt das erste Ereignis
    /// der Ueberschrift bei der Betonung, also bleibt ihr das `# ` als
    /// Vorspann — auch wenn das erste geschriebene Zeichen tief im Kind sitzt
    /// (Defekt `260820-0731`).
    ///
    /// **Beschnitten wird auf den Quellbereich des Elements**, denn ein
    /// Ereignis der Kiste muss nicht in ihm liegen; ein Bereich, der ganz
    /// draussen liegt, verbucht nichts.
    fn ereignis_verbuchen(&mut self, bereich: &Range<usize>) {
        let Some(innerstes) = self.offen.last_mut() else {
            return;
        };
        let anfang = bereich.start.max(innerstes.quelle.start);
        let ende = bereich.end.min(innerstes.quelle.end);
        if anfang >= ende {
            return;
        }
        if innerstes.innen_ab.is_none() {
            innerstes.innen_ab = Some(anfang);
        }
        innerstes.innen_bis = innerstes.innen_bis.max(ende);
    }

    /// Merkt vor, dass hier so viele Umbrueche stehen sollen.
    ///
    /// Am Textanfang geschieht nichts: dort ist nichts abzutrennen.
    fn trennen(&mut self, umbrueche: usize) {
        if self.text.is_empty() {
            return;
        }
        self.trennung = self.trennung.max(umbrueche);
    }

    /// Loest den vorgemerkten Abstand ein, indem er **auffuellt**.
    ///
    /// Steht am Ende des Textes schon ein Umbruch, wird nur die Differenz
    /// geschrieben. Ohne das Auffuellen truege ein Quelltextblock, dessen
    /// letzte Zeile mit einem Umbruch endet, hinterher eine Leerzeile zuviel.
    ///
    /// **Ein Element, das noch kein Zeichen bekommen hat, rueckt mit.** Sonst
    /// truege sein Bereich die Umbrueche des vorigen Absatzes, und ein
    /// Absatzmerkmal — der Einzug einer Listenzeile — schluege auf jenen durch.
    fn absetzen(&mut self) {
        if self.trennung == 0 {
            return;
        }
        let vorhanden = self.text.chars().rev().take_while(|z| *z == '\n').count();
        let fehlend = self.trennung.saturating_sub(vorhanden);
        self.trennung = 0;
        if fehlend == 0 {
            return;
        }
        let vorher = self.stelle;
        self.erzeugen(&"\n".repeat(fehlend));
        for eintrag in &mut self.offen {
            if eintrag.anfang == vorher {
                eintrag.anfang = self.stelle;
            }
        }
    }

    /// Schreibt Zeichen, die KRK selbst setzt, und verankert sie in der Quelle.
    ///
    /// **Zwei Namen fuer zwei Domaenen**, statt den Quellstand als `Option` zu
    /// fuehren und im Rumpf eine Fallunterscheidung zu treffen:
    /// [`Zerlegung::schreiben`] traegt Text, den die Quelle hervorgebracht hat,
    /// diese Methode Text, den sie nicht kennt — den Abstand zwischen zwei
    /// Bloecken und das Merkzeichen eines Punktes.
    ///
    /// Der Quellbereich des Abschnitts bleibt leer und liegt am Lesestand; zum
    /// Quellausschnitt einer Auswahl traegt er nichts bei.
    fn erzeugen(&mut self, stueck: &str) {
        if stueck.is_empty() {
            return;
        }
        let von = self.stelle;
        self.text.push_str(stueck);
        self.stelle += stueck.encode_utf16().count();
        self.kacheln(von, self.gelesen, Abschnittsart::Erzeugt);
    }

    /// Schreibt ein Stueck Text und zaehlt seine UTF-16-Einheiten mit.
    ///
    /// **Erst der Abstand, dann das Merkzeichen, dann der Text.** Nur in
    /// dieser Reihenfolge steht das Merkzeichen vor dem Zeichen, zu dem es
    /// gehoert, und innerhalb des Bereichs seiner Listenzeile.
    ///
    /// **`bis` ist der Quellstand hinter dem Stueck**, und daraus entsteht sein
    /// Abschnitt: er reicht vom Lesestand bis dorthin. Seine Art ist
    /// [`Abschnittsart::Woertlich`], wenn jene Bytes genau das geschriebene
    /// Stueck sind, und sonst [`Abschnittsart::Ersetzt`] — dann liegt
    /// Auszeichnung dazwischen, die der Text weglaesst. Der Parameter steht
    /// hier und nicht als eigener Aufruf daneben, weil die Herkunft eines
    /// Zeichens an derselben Stelle bekannt sein muss, an der es geschrieben
    /// wird.
    ///
    /// **Ein leeres Stueck traegt die Quelle trotzdem ab.** Sonst risse die
    /// Kachelung auf der Quellseite ein Loch.
    fn schreiben(&mut self, stueck: &str, bis: usize) {
        if stueck.is_empty() {
            self.gelesen_bis(bis);
            return;
        }
        self.absetzen();
        self.merkzeichen_einloesen();
        let gelesen = self.gelesen;
        let von = self.stelle;
        self.text.push_str(stueck);
        self.stelle += stueck.encode_utf16().count();
        let art = if bis > gelesen && self.quelle[gelesen..bis] == *stueck {
            Abschnittsart::Woertlich
        } else {
            Abschnittsart::Ersetzt
        };
        self.kacheln(von, bis, art);
    }

    /// Loest die vorgemerkten Merkzeichen der offenen Punkte ein.
    ///
    /// Von aussen nach innen, denn so stehen sie in der Quelle: `- - tief`
    /// traegt zwei. Ein Punkt, der zwischendurch geschlossen wird, ohne dass
    /// ein Zeichen kam, nimmt seinen Wunsch mit — siehe
    /// [`Zerlegung::schliessen`].
    ///
    /// **Was innerhalb eines Punktes offen steht, rueckt hinter sein
    /// Merkzeichen nach.** Ein Kind, das als erstes im Punkt geoeffnet wurde,
    /// hat seinen Anfang vor dem noch uneingeloesten Wunsch bekommen; ohne den
    /// Nachzug deckte sein Bereich das Merkzeichen mit ab, und `- **fett**`
    /// setzte den Aufzaehlungspunkt fett, kursiv, fest, ueberschriftsgross oder
    /// eingefaerbt (Defekt `260812-2019`).
    ///
    /// **Nur die inneren ruecken nach, und darin unterscheidet sich dieser
    /// Nachzug von dem in [`Zerlegung::absetzen`].** Der Punkt selbst soll sein
    /// Merkzeichen mitnehmen — der Einzug der Listenzeile gilt ihm mit —, und
    /// alles ausserhalb von ihm hat es ohnehin schon. Ein Abstand dagegen
    /// gehoert keinem der offenen Elemente, also ruecken dort alle nach.
    fn merkzeichen_einloesen(&mut self) {
        for stufe in 0..self.offen.len() {
            let Some(merkzeichen) = self.offen[stufe].merkzeichen.take() else {
                continue;
            };
            let vorher = self.stelle;
            self.erzeugen(&merkzeichen);
            for eintrag in &mut self.offen[stufe + 1..] {
                if eintrag.anfang == vorher {
                    eintrag.anfang = self.stelle;
                }
            }
        }
    }

    /// Traegt die Quelle bis hierhin als gelesen ab.
    ///
    /// **Was dabei abgetragen wird, bekommt einen Abschnitt mit leerem
    /// Textbereich.** Es sind Bytes, die kein Zeichen hervorgebracht haben: das
    /// schliessende `**` einer Betonung, das `](Ziel)` eines Verweises, der
    /// Zeilenumbruch hinter einem Absatz. Ohne sie haette die Kachelung Loecher
    /// auf der Quellseite, und C2.8 — die Auswahl ueber alles liefert die
    /// Quelle vollstaendig — fiele an den Raendern der Datei aus.
    fn gelesen_bis(&mut self, bis: usize) {
        if bis <= self.gelesen {
            return;
        }
        let von = self.stelle;
        self.kacheln(von, bis, Abschnittsart::Ersetzt);
    }

    /// Schreibt einen Quellbereich woertlich und traegt ihn ab.
    ///
    /// Die Auffangregel und die Deckung treffen sich hier: was woertlich
    /// dasteht, ist gelesen.
    fn woertlich(&mut self, bereich: Range<usize>) {
        let quelle = self.quelle;
        self.schreiben(&quelle[bereich.clone()], bereich.end);
    }

    /// Der erste und der zweite Satz der Deckung: gibt heraus, was bis hierhin
    /// ungelesen blieb.
    ///
    /// **Auf Dokumentebene immer, innerhalb eines Elements nur, wenn es
    /// Bloecke enthaelt** ([`Inhaltsart::Bloecke`]). In einem Element, das seine
    /// eigenen Zeichen traegt, sind die Luecken zwischen den Ereignissen seine
    /// Auszeichnungszeichen, und die gehoeren weg; gedeckt ist es dort ueber
    /// seinen eigenen Quellbereich, siehe [`Zerlegung::schliessen`].
    ///
    /// **Der Vorspann eines Elements faellt weg.** Solange darin noch kein
    /// Byte gelesen wurde, steht dort sein eigenes Merkzeichen — `- `, `1. `,
    /// `> ` —, und das ist Auszeichnung. Hier endet die Deckung, und der
    /// Modulkopf sagt es an derselben Stelle.
    ///
    /// **Fuer den Quellbezug dreht sich an beiden Stellen das Vorzeichen.**
    /// Was aus der Anzeige faellt, faellt nicht aus der Kachelung: die Luecke
    /// bekommt ihren eigenen Abschnitt mit leerem Textbereich, und der
    /// Lesestand rueckt darueber hinweg. Die Anzeige aendert sich davon nicht,
    /// wohl aber die Genauigkeit der Abbildung — das Stueck dahinter steht
    /// danach Zeichen fuer Zeichen an seiner Quelle
    /// ([`Abschnittsart::Woertlich`]) und rundet nicht mehr auf die
    /// Auszeichnungszeichen davor auf.
    ///
    /// **Leerraum faellt weg**, und mit ihm, was die Umgebung auf jeder Zeile
    /// wiederholt ([`ohne_umgebungszeichen`]). Die Abstaende zwischen den
    /// Bloecken rechnet [`Zerlegung::absetzen`] aus dem Wunsch des Blocks; der
    /// Leerraum der Quelle daneben ergaebe Leerzeilen. Was uebrig bleibt, ist
    /// ein Block und wird wie einer abgesetzt.
    fn luecke_bis(&mut self, bis: usize) {
        if self.gelesen >= bis {
            return;
        }
        if let Some((deckt, anfang)) = self
            .offen
            .last()
            .map(|eintrag| (eintrag.inhalt.deckt_luecken(), eintrag.quelle.start))
        {
            // Zwei Luecken fallen aus der Anzeige, und beide werden fuer den
            // Quellbezug trotzdem abgetragen und bekommen ihre Kachel.
            //
            // Die erste: in einem Element aus Zeichen sind die Luecken seine
            // Auszeichnungszeichen — das `# `, die Sternchen, das `[`.
            //
            // Die zweite: der Vorspann eines Elements aus Bloecken. Darin ist
            // noch kein Byte gelesen, also steht hier sein eigenes
            // Merkzeichen. Hier endet die Deckung, und der Modulkopf sagt es
            // an derselben Stelle.
            if !deckt || self.gelesen <= anfang {
                self.gelesen_bis(bis);
                return;
            }
        }
        let quelle = self.quelle;
        let luecke = &quelle[self.gelesen..bis];
        // Satz 1 gegen Satz 2, und keine dritte Frage: gefragt ist dasselbe
        // `self.offen.is_empty()`, das die beiden Saetze ohnehin trennt. Auf
        // Dokumentebene wiederholt keine Umgebung etwas, also ist der Einzug
        // dort Inhalt und bleibt stehen (Defekt `260812-2019`).
        let uebergangen: Cow<'_, str> = if self.offen.is_empty() {
            Cow::Borrowed(luecke.trim())
        } else {
            Cow::Owned(ohne_umgebungszeichen(luecke))
        };
        if uebergangen.is_empty() {
            self.gelesen_bis(bis);
            return;
        }
        self.trennen(ABSATZABSTAND);
        self.schreiben(&uebergangen, bis);
        self.trennen(ABSATZABSTAND);
    }

    /// Beginnt ein Element an der aktuellen Stelle.
    fn oeffnen(
        &mut self,
        quelle: Range<usize>,
        was: Abschluss,
        nach: usize,
        ebene: Option<Ebene>,
        inhalt: Inhaltsart,
    ) {
        self.absetzen();
        let rang = self.elemente.len();
        let anfang_der_quelle = quelle.start;
        self.elemente.push(Quellelement {
            quelle: quelle.clone(),
            klammer: false,
        });
        self.offen.push(Offen {
            anfang: self.stelle,
            was,
            nach,
            quelle,
            rang,
            ebene,
            inhalt,
            merkzeichen: None,
            innen_ab: None,
            innen_bis: anfang_der_quelle,
        });
    }

    /// Wie viele Einrueckebenen gerade offen stehen.
    ///
    /// Gezaehlt und nicht mitgefuehrt: ein zweiter Zaehler neben
    /// [`Zerlegung::offen`] waere die zweite Wahrheit ueber dieselbe Sache und
    /// liefe beim ersten uebersprungenen Element auseinander.
    fn tiefe(&self) -> u8 {
        let ebenen = self.offen.iter().filter(|e| e.ebene.is_some()).count();
        u8::try_from(ebenen).unwrap_or(u8::MAX)
    }

    /// Nimmt das Merkzeichen des naechsten Punktes und zaehlt die Nummer weiter.
    ///
    /// Die Nummer steht in der [`Ebene`] der umgebenden Liste; eine
    /// ungeordnete Liste gibt das [`AUFZAEHLUNGSZEICHEN`]. Steht keine Liste
    /// offen — die Kiste liefert einen Punkt nur innerhalb einer —, gilt
    /// dasselbe Zeichen.
    fn merkzeichen(&mut self) -> String {
        for eintrag in self.offen.iter_mut().rev() {
            if let Some(Ebene::Liste(nummer)) = &mut eintrag.ebene {
                let Some(zahl) = *nummer else {
                    return AUFZAEHLUNGSZEICHEN.to_owned();
                };
                *nummer = Some(zahl.saturating_add(1));
                return format!("{zahl}. ");
            }
        }
        AUFZAEHLUNGSZEICHEN.to_owned()
    }

    /// Beginnt einen Listenpunkt: Merkzeichen vorgemerkt, Tiefe in der
    /// Auszeichnung.
    ///
    /// Das Merkzeichen steht **innerhalb** des Bereichs der Listenzeile, denn
    /// der Einzug soll es mitnehmen; genau das sagt der Kommentar an
    /// `einzugsmerkmal` in `crate::appkit::textmerkmale` zu.
    ///
    /// **Geschrieben wird es hier nicht.** Es wird am Punkt vorgemerkt und
    /// vor seinem ersten Zeichen eingeloest; sonst draengte sich in einer
    /// losen Liste der Absatz des Punktes mit zwei Umbruechen dazwischen, und
    /// das Merkzeichen stuende allein auf seiner Zeile (Defekt `260812-1920`).
    fn punkt_oeffnen(&mut self, quelle: Range<usize>) {
        let tiefe = self.tiefe();
        let merkzeichen = self.merkzeichen();
        self.trennen(PUNKTABSTAND);
        self.oeffnen(
            quelle,
            Abschluss::Auszeichnung(Auszeichnung::Listenzeile { tiefe }),
            PUNKTABSTAND,
            None,
            Inhaltsart::Bloecke,
        );
        if let Some(punkt) = self.offen.last_mut() {
            punkt.merkzeichen = Some(merkzeichen);
        }
    }

    /// Beginnt einen Zitatblock: er legt seine Ebene an und rueckt sich selbst
    /// um sie ein.
    fn zitat_oeffnen(&mut self, quelle: Range<usize>) {
        self.trennen(ABSATZABSTAND);
        let tiefe = self.tiefe().saturating_add(1);
        self.oeffnen(
            quelle,
            Abschluss::Auszeichnung(Auszeichnung::Listenzeile { tiefe }),
            ABSATZABSTAND,
            Some(Ebene::Zitat),
            Inhaltsart::Bloecke,
        );
    }

    /// Beendet das zuletzt begonnene Element und traegt seine Stelle ein.
    ///
    /// **Der zweite Satz der Deckung steht hier.** Ein Element ohne ein
    /// einziges Zeichen traegt keine Stelle ein — ein Bereich der Laenge null
    /// saehe man nicht und stuende doch in der Liste — und gibt statt dessen
    /// seinen Quellbereich woertlich heraus. Sonst verschwaende er spurlos:
    /// `[](https://example.com)` liefert ein Ereignis, aber kein Zeichen.
    /// Doppelt geschrieben wird dabei nichts, denn haette ein Kind etwas
    /// geliefert, waere die Laenge nicht null. Ein Punkt, dessen Merkzeichen
    /// noch aussteht, nimmt es dabei mit: sein Quellbereich traegt das
    /// Merkzeichen der Quelle schon, und `• - [ref]: …` stuende doppelt da.
    ///
    /// **Das gilt fuer den Punkt, der sich hier schliesst, und nicht fuer
    /// seine aeusseren.** Deren Merkzeichen stehen weiter aus und werden vom
    /// woertlichen Quelltext ueber [`Zerlegung::schreiben`] eingeloest, denn
    /// sie stehen **vor** dem ausgegebenen Bereich und nicht darin. Bei
    /// `- - [ref]: …` kommt deshalb `• - [ref]: …` heraus: das `• ` ist das
    /// Merkzeichen des aeusseren Punktes, das `- ` das des inneren, das mit
    /// dessen Quellbereich mitkommt. Beide stehen genau einmal da, und keines
    /// fehlt — gemessen von
    /// `ein_innerer_punkt_ohne_zeichen_steht_neben_dem_merkzeichen_des_aeusseren`
    /// im Pruefmodul (Defekt `260812-2019`).
    ///
    /// **Ein Punkt, dessen Quellbereich nichts traegt als sein Merkzeichen,
    /// loest statt dessen seinen Wunsch ein.** Woertlich herauszugeben ist
    /// dort nichts, was nicht das Merkzeichen selbst waere; das rohe `- `
    /// stuende neben dem `• ` seiner Nachbarn, und ohne Auszeichnung rueckte
    /// die Zeile auch nicht ein ([`traegt_nur_sein_merkzeichen`]).
    ///
    /// **Die zweite Haelfte des zweiten Satzes steht ebenfalls hier.** Was
    /// zwischen dem letzten Kind und dem Ende eines Elements aus Bloecken
    /// ungelesen blieb, gibt es heraus, bevor es sich schliesst. Beim
    /// Endereignis greift [`Zerlegung::luecke_bis`] dafuer nicht: dessen
    /// Quellbereich beginnt am Anfang des Elements und nicht an dieser Luecke.
    ///
    /// **Abgetragen wird, solange das Element noch offen steht, und erst danach
    /// wird es abgeraeumt.** Sein Merkzeichen muss vorher weg (siehe oben),
    /// und daran kommt nur heran, wer seinen Eintrag noch vorfindet; die
    /// aeusseren Punkte dagegen sollen ihres behalten, damit der woertliche
    /// Quelltext es einloesen kann. Die Klammer haengt daran nicht mehr: sie
    /// entsteht aus den Raendern des Elements ([`klammer_der_raender`]) und
    /// nicht aus der Art eines Abschnitts, der zufaellig gerade entsteht.
    fn schliessen(&mut self) {
        if let Some(ende) = self
            .offen
            .last()
            .and_then(|eintrag| eintrag.inhalt.deckt_luecken().then_some(eintrag.quelle.end))
        {
            self.luecke_bis(ende);
        }
        let quelle = self.quelle;
        let nur_das_merkzeichen = self.offen.last().is_some_and(|eintrag| {
            eintrag.merkzeichen.is_some()
                && self.stelle == eintrag.anfang
                && traegt_nur_sein_merkzeichen(&quelle[eintrag.quelle.clone()])
        });
        if nur_das_merkzeichen {
            self.absetzen();
            self.merkzeichen_einloesen();
        }
        // **Abgetragen wird, solange das Element noch offen steht**, und erst
        // danach wird es abgeraeumt: allein hier ist sein Eintrag noch da, und
        // sein Merkzeichen muss aus ihm heraus, bevor der woertliche Quelltext
        // es ein zweites Mal einloest.
        let Some(letztes) = self.offen.last_mut() else {
            return;
        };
        let laenge = self.stelle - letztes.anfang;
        let bereich = letztes.quelle.clone();
        // Das Merkzeichen faellt mit seinem Punkt weg, bevor der woertliche
        // Quelltext es einloesen koennte; in jenem Quelltext steht es schon.
        // Sonst stuende `• - [ref]: …` doppelt da. Bei einer Laenge groesser
        // null ist es ohnehin schon eingeloest.
        letztes.merkzeichen = None;
        if laenge > 0 {
            self.gelesen_bis(bereich.end);
        } else {
            self.woertlich(bereich);
        }
        // Dieselbe Frage wie eben und dieselbe Antwort: steht nichts offen, ist
        // nichts zu schliessen. Der Zweig ist nicht erreichbar, denn zwischen
        // den beiden Abfragen raeumt niemand ab — [`Zerlegung::woertlich`] und
        // [`Zerlegung::gelesen_bis`] schreiben und lesen, sie schliessen nicht.
        // Er steht statt eines `expect`, das den Durchgang abbraeche.
        let Some(eintrag) = self.offen.pop() else {
            return;
        };
        self.elemente[eintrag.rang].klammer = klammer_der_raender(quelle, &eintrag);
        if laenge > 0 {
            match eintrag.was {
                Abschluss::Nichts => {}
                Abschluss::Auszeichnung(art) => self.auszeichnungen.push((
                    eintrag.rang,
                    Auszeichnungsstelle {
                        anfang: eintrag.anfang,
                        laenge,
                        art,
                    },
                )),
                Abschluss::Verweis => {
                    if let Some(farbe) = self.linkfarbe {
                        self.einfaerbungen.push(Einfaerbung {
                            anfang: eintrag.anfang,
                            laenge,
                            farbe,
                            unterstrichen: true,
                        });
                    }
                }
            }
        }
        self.trennen(eintrag.nach);
    }

    /// Gibt den fertigen Text mit seiner Formatierung heraus.
    ///
    /// **Sortiert wird nach Anfang, bei gleichem Anfang das laengere zuerst,
    /// bei gleicher Laenge das zuerst geoeffnete.** Damit steht das aeussere
    /// Element vor dem inneren, und weil `addAttributes:range:` in dieser
    /// Reihenfolge setzt, gewinnt innen das innere: der Quelltext in einer
    /// Ueberschrift bekommt seine feste Schrift und nicht die der Ueberschrift,
    /// und der tiefere von zwei Listenpunkten seinen groesseren Einzug.
    ///
    /// **Der dritte Schluessel ist nicht schmueckendes Beiwerk.** Ohne ihn
    /// entschiede bei gleichem Anfang **und** gleicher Laenge die stabile
    /// Sortierung, also die Reihenfolge der Endereignisse, und die laeuft von
    /// innen nach aussen — genau verkehrt. Gemessen an `` **`code`** ``, wo
    /// starke Betonung und feste Schrift denselben Bereich decken (Defekt
    /// `260812-1805` zum Ueberschneidungssatz). Mit dem Rang ist die Ordnung
    /// total und haengt an nichts, was die Kiste sonst noch tut.
    fn abschliessen(mut self) -> Gerendert {
        self.luecke_bis(self.quelle.len());
        self.auszeichnungen.sort_by(|(rang_a, a), (rang_b, b)| {
            a.anfang
                .cmp(&b.anfang)
                .then(b.laenge.cmp(&a.laenge))
                .then(rang_a.cmp(rang_b))
        });
        self.einfaerbungen.sort_by_key(|stueck| stueck.anfang);
        Gerendert {
            formatierung: Formatierung {
                art: Darstellungsart::Markdown,
                laenge: self.stelle,
                einfaerbungen: self.einfaerbungen,
                auszeichnungen: self
                    .auszeichnungen
                    .into_iter()
                    .map(|(_, stelle)| stelle)
                    .collect(),
            },
            text: self.text,
            quellbezug: Arc::new(Quellbezug {
                quelle: self.quelle.to_owned(),
                abschnitte: self.abschnitte,
                elemente: self.elemente,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rendert mit der hellen Tafel; die Farbe spielt in den meisten Proben
    /// keine Rolle, die Tafel muss aber genannt werden.
    fn gerendert(quelle: &str) -> Gerendert {
        rendern(quelle, Tafel::Hell)
    }

    /// Die Stellen einer Auszeichnung, nach ihrer Art.
    fn stellen(gerendert: &Gerendert, gesucht: Auszeichnung) -> Vec<(usize, usize)> {
        gerendert
            .formatierung
            .auszeichnungen
            .iter()
            .filter(|stelle| stelle.art == gesucht)
            .map(|stelle| (stelle.anfang, stelle.laenge))
            .collect()
    }

    /// Das Stueck des Textes, das eine Stelle bezeichnet — in UTF-16 gerechnet,
    /// also genau so, wie AppKit den Bereich liest.
    fn stueck(text: &str, anfang: usize, laenge: usize) -> String {
        let einheiten: Vec<u16> = text.encode_utf16().collect();
        String::from_utf16(&einheiten[anfang..anfang + laenge]).expect("gueltiges UTF-16")
    }

    #[test]
    fn eine_ueberschrift_verliert_ihre_doppelkreuze() {
        let ergebnis = gerendert("# Titel\n\nText\n");
        assert_eq!(ergebnis.text, "Titel\n\nText");
        let ueberschriften = stellen(&ergebnis, Auszeichnung::Ueberschrift { stufe: 1 });
        assert_eq!(ueberschriften, vec![(0, 5)]);
        assert_eq!(stueck(&ergebnis.text, 0, 5), "Titel");
    }

    #[test]
    fn alle_sechs_stufen_kommen_mit_ihrer_zahl_an() {
        for stufe in 1..=6u8 {
            let quelle = format!("{} Stufe\n", "#".repeat(usize::from(stufe)));
            let ergebnis = gerendert(&quelle);
            assert_eq!(ergebnis.text, "Stufe");
            assert_eq!(
                stellen(&ergebnis, Auszeichnung::Ueberschrift { stufe }),
                vec![(0, 5)],
                "Stufe {stufe}"
            );
        }
    }

    #[test]
    fn betonung_und_starke_betonung_verlieren_ihre_sternchen() {
        let ergebnis = gerendert("Ein *kursiv* und ein **fett**.\n");
        assert_eq!(ergebnis.text, "Ein kursiv und ein fett.");
        let betont = stellen(&ergebnis, Auszeichnung::Betonung);
        assert_eq!(betont.len(), 1);
        assert_eq!(stueck(&ergebnis.text, betont[0].0, betont[0].1), "kursiv");
        let stark = stellen(&ergebnis, Auszeichnung::StarkeBetonung);
        assert_eq!(stark.len(), 1);
        assert_eq!(stueck(&ergebnis.text, stark[0].0, stark[0].1), "fett");
    }

    #[test]
    fn ein_quelltextblock_verliert_seine_zaeune_und_traegt_feste_schrift() {
        let ergebnis = gerendert("```rust\nfn main() {}\n```\n");
        assert_eq!(ergebnis.text, "fn main() {}\n");
        let fest = stellen(&ergebnis, Auszeichnung::FesteSchrift);
        assert_eq!(fest.len(), 1);
        assert_eq!(fest[0].0, 0);
    }

    #[test]
    fn quelltext_in_der_zeile_verliert_seine_haken() {
        let ergebnis = gerendert("Ein `Aufruf` darin.\n");
        assert_eq!(ergebnis.text, "Ein Aufruf darin.");
        let fest = stellen(&ergebnis, Auszeichnung::FesteSchrift);
        assert_eq!(fest.len(), 1);
        assert_eq!(stueck(&ergebnis.text, fest[0].0, fest[0].1), "Aufruf");
    }

    /// Das erste Abnahmekriterium von C4: `[Text](Ziel)` steht als
    /// eingefaerbter Verweis ohne Klammern, die Adresse verschwindet.
    #[test]
    fn ein_verweis_behaelt_seinen_text_und_verliert_seine_adresse() {
        let ergebnis = gerendert("Siehe [die Seite](https://example.com) dort.\n");
        assert_eq!(ergebnis.text, "Siehe die Seite dort.");
        assert!(
            !ergebnis.text.contains("example.com"),
            "die Adresse gehoert nicht in den Text"
        );
        assert_eq!(ergebnis.formatierung.einfaerbungen.len(), 1);
        let verweis = ergebnis.formatierung.einfaerbungen[0];
        assert!(verweis.unterstrichen, "C4: Farbe und Unterstreichung");
        assert_eq!(
            stueck(&ergebnis.text, verweis.anfang, verweis.laenge),
            "die Seite"
        );
    }

    /// Die Farbe kommt aus der Tafel und nicht aus diesem Modul.
    ///
    /// Dass sie nicht die Farbe des Fliesstextes ist, haelt
    /// `crate::hervorhebung::tests::die_tafel_faerbt_einen_verweis` fest; hier
    /// steht die andere Haelfte, naemlich dass das Rendern genau diesen
    /// Nachschlag nimmt und keine eigene Farbe setzt.
    #[test]
    fn der_verweis_traegt_die_farbe_seiner_tafel() {
        let quelle = "[Ziel](https://example.com)\n";
        for tafel in [Tafel::Hell, Tafel::Dunkel] {
            let einfaerbungen = rendern(quelle, tafel).formatierung.einfaerbungen;
            assert_eq!(einfaerbungen.len(), 1, "{tafel:?}");
            assert_eq!(
                einfaerbungen[0].farbe,
                linkfarbe(tafel).expect("beide Tafeln stehen im Vorgabesatz"),
                "{tafel:?}"
            );
        }
    }

    #[test]
    fn ein_listenpunkt_traegt_den_einzug_und_behaelt_sein_zeichen() {
        let ergebnis = gerendert("- eins\n- zwei\n");
        assert_eq!(ergebnis.text, "• eins\n• zwei");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(punkte.len(), 2, "je Punkt eine Zeile");
        assert_eq!(
            stueck(&ergebnis.text, punkte[0].0, punkte[0].1),
            "• eins",
            "das Merkzeichen liegt im Bereich der Zeile und rueckt mit ein"
        );
        assert_eq!(stueck(&ergebnis.text, punkte[1].0, punkte[1].1), "• zwei");
    }

    /// Defekt `260812-1805`, erster gemessener Fall: eine geordnete Liste war
    /// von einer ungeordneten nicht zu unterscheiden.
    #[test]
    fn eine_geordnete_liste_behaelt_ihre_nummern() {
        let ergebnis = gerendert("1. eins\n2. zwei\n3. drei\n");
        assert_eq!(ergebnis.text, "1. eins\n2. zwei\n3. drei");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(punkte.len(), 3);
        assert_eq!(stueck(&ergebnis.text, punkte[2].0, punkte[2].1), "3. drei");
    }

    /// Die Nummer kommt aus der Liste und ist kein Zaehler bei eins: eine
    /// Liste, die bei 3 anfaengt, faengt bei 3 an.
    #[test]
    fn eine_geordnete_liste_beginnt_bei_ihrer_eigenen_zahl() {
        assert_eq!(gerendert("3. drei\n4. vier\n").text, "3. drei\n4. vier");
    }

    /// Defekt `260812-1805`, zweiter gemessener Fall: eine dreistufige Liste
    /// stand flach da, alle vier Bereiche mit demselben Einzug.
    #[test]
    fn eine_verschachtelte_liste_traegt_ihre_tiefe() {
        let ergebnis = gerendert("- eins\n- zwei\n  - drunter\n    - noch tiefer\n");
        assert_eq!(ergebnis.text, "• eins\n• zwei\n• drunter\n• noch tiefer");
        let tiefen: Vec<u8> = ergebnis
            .formatierung
            .auszeichnungen
            .iter()
            .filter_map(|stelle| match stelle.art {
                Auszeichnung::Listenzeile { tiefe } => Some(tiefe),
                _ => None,
            })
            .collect();
        assert_eq!(
            tiefen,
            vec![1, 1, 2, 3],
            "die vier Punkte liegen auf drei Ebenen"
        );
    }

    /// Defekt `260812-1920`, erster gemessener Fall: in einer **losen** Liste
    /// stand das Merkzeichen allein auf seiner Zeile.
    ///
    /// Lose heisst: eine Leerzeile zwischen den Punkten. Die Kiste schiebt
    /// dann zwischen Punkt und Text einen Absatz, und der verlangt zwei
    /// Umbrueche; das sofort geschriebene Merkzeichen bekam sie zwischen sich
    /// und seinen Text (`"• \n\neins\n\n• \n\nzwei"`). Die Leerzeile
    /// zwischen den Punkten bleibt, denn die Quelle traegt sie.
    #[test]
    fn eine_lose_liste_haelt_ihr_merkzeichen_bei_seinem_text() {
        let ergebnis = gerendert("- eins\n\n- zwei\n");
        assert_eq!(ergebnis.text, "• eins\n\n• zwei");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(punkte.len(), 2, "je Punkt eine Zeile");
        assert_eq!(
            stueck(&ergebnis.text, punkte[0].0, punkte[0].1),
            "• eins",
            "der Bereich der Zeile faengt beim Merkzeichen an und hoert beim Text auf"
        );
        assert_eq!(stueck(&ergebnis.text, punkte[1].0, punkte[1].1), "• zwei");
    }

    /// Dasselbe fuer die Nummer einer geordneten Liste: sie ist derselbe
    /// Wunsch und wird an derselben Stelle eingeloest.
    #[test]
    fn eine_lose_geordnete_liste_haelt_ihre_nummer_bei_ihrem_text() {
        assert_eq!(gerendert("1. eins\n\n2. zwei\n").text, "1. eins\n\n2. zwei");
    }

    /// Derselbe Defekt bei jedem Punkt mit mehr als einem Block, zweiter
    /// gemessener Fall: ein Zitat unter dem Absatz des Punktes.
    ///
    /// Der Absatz des Punktes ist auch hier der, der sich dazwischendraengte
    /// (`"• \n\nPunkt\n\nZitat"`).
    #[test]
    fn ein_punkt_aus_zwei_bloecken_haelt_sein_merkzeichen() {
        let ergebnis = gerendert("- Punkt\n\n  > Zitat\n");
        assert_eq!(ergebnis.text, "• Punkt\n\nZitat");
    }

    /// Zwei Merkzeichen uebereinander bleiben zwei: `- - tief` traegt beide,
    /// und beide werden vor demselben Zeichen eingeloest, von aussen nach
    /// innen.
    #[test]
    fn zwei_punkte_uebereinander_tragen_beide_ihr_merkzeichen() {
        assert_eq!(gerendert("- - tief\n").text, "• • tief");
    }

    /// Defekt `260812-2019`, der Hauptfall: beginnt ein Punkt unmittelbar mit
    /// einer Auszeichnung, so deckte deren Bereich das Merkzeichen mit ab.
    ///
    /// **Diese Probe misst den Bereich und nicht den Ausgabetext.** Der Text
    /// war die ganze Zeit richtig — `"• fett"` —, und genau deshalb hat ihn
    /// keine der vorhandenen Proben gefangen: in AppKit setzt
    /// `crate::appkit::textmerkmale` ueber diesen Bereich eine Schrift, und
    /// der Aufzaehlungspunkt wurde fett, kursiv, festbreit oder
    /// ueberschriftsgross mitgesetzt. Fuenf Auszeichnungsarten, jede als
    /// erstes Kind eines Punktes.
    #[test]
    fn eine_auszeichnung_am_anfang_eines_punktes_deckt_das_merkzeichen_nicht() {
        let faelle: [(&str, Auszeichnung, &str, &str); 5] = [
            (
                "- **fett**\n",
                Auszeichnung::StarkeBetonung,
                "fett",
                "• fett",
            ),
            ("- *kursiv*\n", Auszeichnung::Betonung, "kursiv", "• kursiv"),
            ("- `code`\n", Auszeichnung::FesteSchrift, "code", "• code"),
            (
                "- # Titel\n",
                Auszeichnung::Ueberschrift { stufe: 1 },
                "Titel",
                "• Titel",
            ),
            (
                "- ```\n  code\n  ```\n",
                Auszeichnung::FesteSchrift,
                "code\n",
                "• code\n",
            ),
        ];
        for (quelle, art, erwartet, zeile) in faelle {
            let ergebnis = gerendert(quelle);
            let gefunden = stellen(&ergebnis, art);
            assert_eq!(gefunden.len(), 1, "{quelle:?}");
            assert_eq!(
                stueck(&ergebnis.text, gefunden[0].0, gefunden[0].1),
                erwartet,
                "das Merkzeichen gehoert nicht in den Bereich von {art:?} ({quelle:?})"
            );
            let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
            assert_eq!(punkte.len(), 1, "{quelle:?}");
            assert_eq!(
                stueck(&ergebnis.text, punkte[0].0, punkte[0].1),
                zeile,
                "die Listenzeile nimmt das Merkzeichen dagegen mit ({quelle:?})"
            );
        }
    }

    /// Derselbe Defekt fuer den Verweis, der als einziger keine
    /// [`Auszeichnung`] traegt, sondern Farbe und Unterstreichung.
    ///
    /// Die Wirkung war hier die sichtbarste: der Aufzaehlungspunkt bekam die
    /// Verweisfarbe und wurde unterstrichen.
    #[test]
    fn ein_verweis_am_anfang_eines_punktes_faerbt_das_merkzeichen_nicht() {
        let ergebnis = gerendert("- [Link](https://example.com)\n");
        assert_eq!(ergebnis.text, "• Link");
        assert_eq!(ergebnis.formatierung.einfaerbungen.len(), 1);
        let verweis = ergebnis.formatierung.einfaerbungen[0];
        assert_eq!(
            stueck(&ergebnis.text, verweis.anfang, verweis.laenge),
            "Link",
            "das Merkzeichen wird nicht eingefaerbt und nicht unterstrichen"
        );
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(stueck(&ergebnis.text, punkte[0].0, punkte[0].1), "• Link");
    }

    /// Dasselbe fuer die Nummer einer geordneten Liste: sie ist derselbe
    /// Wunsch und liegt genauso ausserhalb.
    #[test]
    fn eine_nummer_am_anfang_eines_punktes_wird_nicht_mit_ausgezeichnet() {
        let ergebnis = gerendert("1. **fett**\n");
        assert_eq!(ergebnis.text, "1. fett");
        let stark = stellen(&ergebnis, Auszeichnung::StarkeBetonung);
        assert_eq!(stueck(&ergebnis.text, stark[0].0, stark[0].1), "fett");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(stueck(&ergebnis.text, punkte[0].0, punkte[0].1), "1. fett");
    }

    /// Zwei Merkzeichen uebereinander, und jedes liegt genau in einem Bereich
    /// mehr als das naechstinnere.
    ///
    /// Der aeussere Punkt nimmt beide mit, der innere nur seines, die starke
    /// Betonung keines. Das ist die Staffelung, die der Nachzug in
    /// [`Zerlegung::merkzeichen_einloesen`] herstellt: er rueckt beim Einloesen
    /// eines Merkzeichens nur die **inneren** Eintraege nach.
    #[test]
    fn zwei_merkzeichen_liegen_gestaffelt_ausserhalb_der_auszeichnung() {
        let ergebnis = gerendert("- - **fett**\n");
        assert_eq!(ergebnis.text, "• • fett");
        let stark = stellen(&ergebnis, Auszeichnung::StarkeBetonung);
        assert_eq!(stueck(&ergebnis.text, stark[0].0, stark[0].1), "fett");
        let aussen = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(stueck(&ergebnis.text, aussen[0].0, aussen[0].1), "• • fett");
        let innen = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 2 });
        assert_eq!(stueck(&ergebnis.text, innen[0].0, innen[0].1), "• fett");
    }

    /// Derselbe Bereich in einer **losen** Liste, also mit dem Absatz
    /// dazwischen, den die Kiste dort einschiebt.
    ///
    /// Die lose Liste ist die Form, die Turn 3 uebersehen hat, die
    /// Auszeichnung am Anfang die, die Turn 4 uebersehen hat. Beide zusammen
    /// stehen hier.
    #[test]
    fn eine_lose_liste_haelt_ihre_auszeichnung_hinter_dem_merkzeichen() {
        let ergebnis = gerendert("- eins\n\n- **fett**\n");
        assert_eq!(ergebnis.text, "• eins\n\n• fett");
        let stark = stellen(&ergebnis, Auszeichnung::StarkeBetonung);
        assert_eq!(stark.len(), 1);
        assert_eq!(stueck(&ergebnis.text, stark[0].0, stark[0].1), "fett");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(punkte.len(), 2);
        assert_eq!(stueck(&ergebnis.text, punkte[1].0, punkte[1].1), "• fett");
    }

    /// Der Gurt um die ganze Klasse: **kein Merkzeichen liegt im Bereich
    /// eines Stueckes.**
    ///
    /// Die Proben darueber messen je einen Fall. Diese misst die Regel: ueber
    /// eine Reihe von Quellen darf kein Bereich, der kein Absatzmerkmal ist,
    /// mit einem Merkzeichen beginnen. Nur [`Auszeichnung::Listenzeile`] darf
    /// es — sie ist das Absatzmerkmal, dem der Einzug gilt. Waere diese Probe
    /// zur Zeit von `c35f8b1` dagewesen, haette sie den Defekt gefangen, ohne
    /// dass jemand den einzelnen Fall haette nennen muessen.
    #[test]
    fn kein_merkzeichen_liegt_im_bereich_eines_stueckes() {
        let quellen = [
            "- **fett**\n",
            "- *kursiv*\n",
            "- `code`\n",
            "- [Link](https://example.com)\n",
            "- # Titel\n",
            "- ```\n  code\n  ```\n",
            "1. **fett**\n",
            "- - **fett**\n",
            "- eins\n\n- **fett**\n",
            "> - **fett im Zitat**\n",
            "- **fett**\n- *kursiv*\n",
            "3. [Link](https://example.com)\n",
            "- - - `tief`\n",
        ];
        for quelle in quellen {
            let ergebnis = gerendert(quelle);
            for stelle in &ergebnis.formatierung.auszeichnungen {
                if matches!(stelle.art, Auszeichnung::Listenzeile { .. }) {
                    continue;
                }
                let text = stueck(&ergebnis.text, stelle.anfang, stelle.laenge);
                assert!(
                    !beginnt_mit_merkzeichen(&text),
                    "{:?} deckt in {quelle:?} ein Merkzeichen mit: {text:?}",
                    stelle.art
                );
            }
            for farbig in &ergebnis.formatierung.einfaerbungen {
                let text = stueck(&ergebnis.text, farbig.anfang, farbig.laenge);
                assert!(
                    !beginnt_mit_merkzeichen(&text),
                    "eine Einfaerbung deckt in {quelle:?} ein Merkzeichen mit: {text:?}"
                );
            }
        }
    }

    /// Ob ein Stueck Text mit einem gerenderten Merkzeichen anfaengt.
    ///
    /// Das [`AUFZAEHLUNGSZEICHEN`] oder eine Nummer mit Punkt und Leerzeichen
    /// — die beiden Formen, die [`Zerlegung::merkzeichen`] herausgibt.
    fn beginnt_mit_merkzeichen(text: &str) -> bool {
        if text.starts_with(AUFZAEHLUNGSZEICHEN) {
            return true;
        }
        let ziffern = text.trim_start_matches(|zeichen: char| zeichen.is_ascii_digit());
        ziffern.len() < text.len() && ziffern.starts_with(". ")
    }

    /// Defekt `260812-2019`: ein Punkt ohne jeden Inhalt zeigte sein rohes
    /// `- ` samt Zeilenumbruch und rueckte nicht ein.
    ///
    /// Sein Quellbereich traegt nichts als das Merkzeichen, also ist woertlich
    /// nichts herauszugeben; statt dessen wird der Wunsch eingeloest
    /// ([`traegt_nur_sein_merkzeichen`]).
    #[test]
    fn ein_punkt_ohne_jeden_inhalt_zeigt_sein_gerendertes_merkzeichen() {
        for quelle in ["- \n", "-\n", "*\n", "+\n"] {
            let ergebnis = gerendert(quelle);
            assert_eq!(ergebnis.text, "• ", "{quelle:?}");
            let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
            assert_eq!(punkte.len(), 1, "der leere Punkt rueckt ein ({quelle:?})");
            assert_eq!(stueck(&ergebnis.text, punkte[0].0, punkte[0].1), "• ");
        }
        assert_eq!(gerendert("1.\n").text, "1. ", "auch die Nummer");
    }

    /// Dieselbe Liste zeigte zwei verschiedene Merkzeichen nebeneinander, je
    /// nachdem ob der Punkt Text trug.
    #[test]
    fn ein_leerer_punkt_traegt_dasselbe_merkzeichen_wie_seine_nachbarn() {
        let ergebnis = gerendert("- eins\n- \n");
        assert_eq!(ergebnis.text, "• eins\n• ");
        assert_eq!(
            stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 }).len(),
            2,
            "beide Punkte rueckten ein"
        );
        assert_eq!(gerendert("- \n- zwei\n").text, "• \n• zwei");
        assert_eq!(
            gerendert("- \n\nAbsatz\n").text,
            "• \n\nAbsatz",
            "der Zeilenumbruch der Quelle kommt nicht mit: der Abstand ist der des Blocks"
        );
    }

    /// Zwei leere Punkte uebereinander tragen beide ihr Merkzeichen, und die
    /// Staffelung der Bereiche bleibt dieselbe wie bei einem Punkt mit Text.
    #[test]
    fn zwei_leere_punkte_uebereinander_tragen_beide_ihr_merkzeichen() {
        let ergebnis = gerendert("- -\n");
        assert_eq!(ergebnis.text, "• • ");
        let aussen = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(stueck(&ergebnis.text, aussen[0].0, aussen[0].1), "• • ");
        let innen = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 2 });
        assert_eq!(stueck(&ergebnis.text, innen[0].0, innen[0].1), "• ");
    }

    /// Die Grenze zwischen dem leeren Punkt und dem woertlichen Zweig, von der
    /// anderen Seite gemessen.
    ///
    /// `ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen`
    /// haelt fest, dass ein Punkt mit Verweisdefinition woertlich dasteht;
    /// hier steht, woran das haengt — der Quellbereich traegt **mehr** als
    /// das Merkzeichen. Ein Punkt, dessen Merkzeichen von einem Leerzeichen
    /// mehr gefolgt wird, faellt weiterhin auf die andere Seite.
    #[test]
    fn der_leere_punkt_und_der_woertliche_zweig_trennen_sich_am_inhalt() {
        assert!(traegt_nur_sein_merkzeichen("- \n"));
        assert!(traegt_nur_sein_merkzeichen("-\n"));
        assert!(traegt_nur_sein_merkzeichen("1.   \n"));
        assert!(!traegt_nur_sein_merkzeichen("- [ref]: http://a.example\n"));
        assert!(!traegt_nur_sein_merkzeichen("- Text\n"));
        assert_eq!(
            gerendert("- [ref]: http://a.example\n").text,
            "- [ref]: http://a.example\n",
            "die Gegenseite bleibt, wie der Datensatz 260812-1920 sie entschieden hat"
        );
    }

    /// Defekt `260812-2019`: das Merkzeichen eines **aeusseren** Punktes wird
    /// vom woertlichen Quelltext eines inneren eingeloest.
    ///
    /// Der Doc-Kommentar an [`Zerlegung::schliessen`] sagte zu, `• - [ref]: …`
    /// entstehe nicht. Es entsteht, und es ist richtig so: das `• ` ist das
    /// Merkzeichen des **aeusseren** Punktes, dessen `- ` in der Quelle vor
    /// dem ausgegebenen Bereich steht und damit nirgends sonst herauskaeme;
    /// das `- ` daneben ist das des inneren und kommt mit dessen Quellbereich.
    /// Beide stehen genau einmal da. Die Zusage ist berichtigt, und diese
    /// Probe schreibt die Ausgabe fest.
    #[test]
    fn ein_innerer_punkt_ohne_zeichen_steht_neben_dem_merkzeichen_des_aeusseren() {
        assert_eq!(
            gerendert("- - [ZIEL]: http://z.example\n").text,
            "• - [ZIEL]: http://z.example\n"
        );
    }

    /// Defekt `260812-2019`: [`ohne_umgebungszeichen`] lief auch auf
    /// Dokumentebene und nahm dort einen Einzug weg, der Inhalt ist.
    ///
    /// Dort wiederholt keine Umgebung etwas. Die Fortsetzungszeile einer
    /// mehrzeiligen Verweisdefinition behaelt deshalb ihren Einzug.
    #[test]
    fn auf_dokumentebene_bleibt_der_einzug_einer_zeile_stehen() {
        assert_eq!(
            gerendert("[ZIEL]: http://z.example\n      \"Titel\"\n").text,
            "[ZIEL]: http://z.example\n      \"Titel\""
        );
    }

    /// Ein Zitatblock zaehlt als Einrueckebene: der Punkt darin liegt eine
    /// Ebene tiefer als das Zitat.
    ///
    /// Beide Bereiche sind hier **gleich lang**, und damit haengt die Antwort
    /// allein am dritten Sortierschluessel: das zuerst geoeffnete Zitat steht
    /// vorn, der Punkt setzt danach und gewinnt.
    #[test]
    fn ein_punkt_im_zitat_liegt_eine_ebene_tiefer() {
        let ergebnis = gerendert("> - Punkt im Zitat\n");
        assert_eq!(ergebnis.text, "• Punkt im Zitat");
        let arten: Vec<Auszeichnung> = ergebnis
            .formatierung
            .auszeichnungen
            .iter()
            .map(|stelle| stelle.art)
            .collect();
        assert_eq!(
            arten,
            vec![
                Auszeichnung::Listenzeile { tiefe: 1 },
                Auszeichnung::Listenzeile { tiefe: 2 }
            ]
        );
    }

    /// Das zweite Abnahmekriterium von C4: der Zitatblock bekommt den Einzug
    /// der Listen, weil sein Merkzeichen sonst spurlos verschwaende.
    #[test]
    fn ein_zitatblock_bekommt_den_einzug_der_listen() {
        let ergebnis = gerendert("> Zitat\n");
        assert_eq!(ergebnis.text, "Zitat");
        assert!(
            !stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 }).is_empty(),
            "ohne Einzug bliebe vom Zitat nichts uebrig"
        );
    }

    /// Das vierte Abnahmekriterium von C4: eine Tabelle steht Zeile fuer Zeile
    /// mit ihren Zwischenraeumen da.
    #[test]
    fn eine_tabelle_bleibt_ein_quelltextraster() {
        let quelle = "| Spalte A | Spalte B |\n|----------|----------|\n| 1        | 2        |\n";
        let ergebnis = gerendert(quelle);
        assert_eq!(
            ergebnis.text,
            "| Spalte A | Spalte B |\n|----------|----------|\n| 1        | 2        |",
            "die Zeilen bleiben Zeilen und die Zwischenraeume stehen"
        );
    }

    /// Das dritte Abnahmekriterium von C4, erster Fall: ein Bild erscheint als
    /// sein Quelltext, und geladen wird es nicht.
    #[test]
    fn ein_bild_erscheint_als_sein_quelltext() {
        let ergebnis = gerendert("Davor ![Alt](bild.png) danach.\n");
        assert_eq!(ergebnis.text, "Davor ![Alt](bild.png) danach.");
    }

    /// Zweiter Fall: eingebettetes HTML, als Block und in der Zeile.
    #[test]
    fn eingebettetes_html_erscheint_als_sein_quelltext() {
        let block = gerendert("<div class=\"x\">\ndrin\n</div>\n");
        assert_eq!(block.text, "<div class=\"x\">\ndrin\n</div>\n");

        let zeile = gerendert("Text mit <span>drin</span> darin.\n");
        assert_eq!(zeile.text, "Text mit <span>drin</span> darin.");
    }

    /// Dritter Fall: eine Trennlinie erscheint als ihre Zeichen.
    #[test]
    fn eine_trennlinie_erscheint_als_ihre_zeichen() {
        let ergebnis = gerendert("davor\n\n---\n\ndanach\n");
        assert_eq!(ergebnis.text, "davor\n\n---\n\ndanach");
    }

    /// Vierter Fall, und der erste, den nicht die Auffangregel ueber [`Tag`]
    /// traegt: eine Verweisdefinition erzeugt **kein einziges Ereignis**.
    ///
    /// Die Kiste verbraucht sie beim Aufloesen des Verweises und meldet sie
    /// nicht; bis zum 260812 verschwand sie damit spurlos aus der Anzeige
    /// (Defekt `260812-1805`, gemessen). Gedeckt ist sie jetzt ueber die
    /// Luecke zwischen den Quellbereichen, siehe Modulkopf.
    #[test]
    fn eine_verweisdefinition_bleibt_als_ihr_quelltext_stehen() {
        let ergebnis = gerendert(
            "Text davor.\n\nSiehe [den Text][ref] hier.\n\n[ref]: https://example.com \"Titel\"\n",
        );
        assert_eq!(
            ergebnis.text,
            "Text davor.\n\nSiehe den Text hier.\n\n[ref]: https://example.com \"Titel\""
        );
    }

    /// Der schaerfste Fall derselben Luecke: eine Datei, die nur aus
    /// Definitionen besteht, zeigte eine leere Flaeche.
    #[test]
    fn eine_datei_aus_lauter_verweisdefinitionen_bleibt_sichtbar() {
        let ergebnis = gerendert("[ref]: https://example.com\n[zwei]: https://b.example\n");
        assert_eq!(
            ergebnis.text, "[ref]: https://example.com\n[zwei]: https://b.example",
            "eine Datei mit Inhalt darf keine leere Flaeche zeigen"
        );
    }

    /// Die andere Haelfte desselben Defekts: ein Ereignis **ohne Zeichen**.
    ///
    /// Ein Verweis ohne Text schrieb nichts, und weil die Laenge null war,
    /// trug auch das Schliessen nichts ein; die 23 Zeichen waren weg. Jetzt
    /// gibt ein Element ohne ein einziges Zeichen seinen Quellbereich heraus.
    #[test]
    fn ein_verweis_ohne_text_erscheint_als_sein_quelltext() {
        let ergebnis = gerendert("Siehe [](https://example.com) dort.\n");
        assert_eq!(ergebnis.text, "Siehe [](https://example.com) dort.");
    }

    /// Die Kehrseite der Deckung: die Auszeichnungszeichen eines Elements, das
    /// Zeichen geliefert hat, bleiben verschwunden.
    ///
    /// Ohne diese Grenze truege die Luecken-Regel das `[` und das `][ref]`
    /// eines Verweises in Kurzform wieder in den Text und machte aus der
    /// Behebung eine Verschlimmerung.
    #[test]
    fn die_zeichen_eines_gerenderten_elements_bleiben_weg() {
        let ergebnis = gerendert("Siehe [den Text][ref] hier.\n\n[ref]: https://example.com\n");
        assert!(
            ergebnis.text.starts_with("Siehe den Text hier."),
            "die Klammern des Verweises gehoeren nicht in den Text: {:?}",
            ergebnis.text
        );
    }

    /// Defekt `260812-1920`, dritter gemessener Fall des Merkzeichen-Defekts
    /// und zugleich die Luecke, die der zweite Satz der Deckung schliesst:
    /// eine Verweisdefinition **hinter** dem Absatz eines Punktes.
    ///
    /// Sie steht in keinem Ereignis, und beim Endereignis des Punktes greift
    /// [`Zerlegung::luecke_bis`] nicht — dessen Quellbereich beginnt am Anfang
    /// des Punktes. Herausgegeben wird sie deshalb beim Schliessen.
    #[test]
    fn eine_verweisdefinition_hinter_dem_absatz_eines_punktes_bleibt_stehen() {
        let ergebnis = gerendert("- Punkt\n\n  [ref]: http://a.example\n");
        assert_eq!(ergebnis.text, "• Punkt\n\n[ref]: http://a.example");
    }

    /// Defekt `260812-1920`, erster gemessener Fall der zu engen
    /// Deckungszusage: ein Punkt, der nichts als eine Verweisdefinition
    /// enthaelt, gab nur sein Merkzeichen her (`"• "`).
    ///
    /// Er liefert kein einziges Zeichen, also tritt nach dem dritten Satz der
    /// Deckung sein Quellbereich an seine Stelle — mit dem `- ` der Quelle
    /// und ohne das Merkzeichen, das nie geschrieben wurde. Ein doppeltes
    /// `• - ` waere das Ergebnis, wenn der Wunsch den Punkt ueberlebte.
    #[test]
    fn ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen() {
        let ergebnis = gerendert("- [ref]: http://a.example\n");
        assert_eq!(ergebnis.text, "- [ref]: http://a.example\n");
    }

    /// Zweiter gemessener Fall derselben Zusage: eine Verweisdefinition am
    /// Ende eines Zitatblocks.
    ///
    /// Das `>` jeder Zeile faellt weg, denn es steht auf jeder Zeile des
    /// Zitats und ist seine Auszeichnung.
    #[test]
    fn eine_verweisdefinition_am_ende_eines_zitats_bleibt_stehen() {
        let ergebnis = gerendert("> Zitat\n>\n> [ref]: http://a.example\n");
        assert_eq!(ergebnis.text, "Zitat\n\n[ref]: http://a.example");
    }

    /// Die Gegenprobe dazu: die Zeichen des Zitats selbst kommen nicht in den
    /// Text.
    ///
    /// Zwischen zwei Absaetzen eines Zitats steht in der Quelle eine Zeile aus
    /// einem einzigen `>`. Sie ist die Luecke, die der zweite Satz der Deckung
    /// jetzt sieht; ohne [`ohne_umgebungszeichen`] stuende sie als nacktes
    /// `>` zwischen den Absaetzen.
    #[test]
    fn ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text() {
        assert_eq!(gerendert("> eins\n>\n> zwei\n").text, "eins\n\nzwei");
    }

    /// **Hier endet die Deckung, und sie endet mit Absicht:** der Vorspann
    /// eines Elements, also alles vor dem ersten Byte, das darin gelesen wird.
    ///
    /// Dort steht sein Merkzeichen — `- `, `> ` —, und das gehoert weg. Eine
    /// Verweisdefinition, die sich dorthin verirrt, faellt mit heraus. Diese
    /// Probe haelt die Grenze fest, damit der naechste Leser sie gemessen
    /// vorfindet und nicht nachrechnen muss; der Modulkopf sagt dieselbe
    /// Grenze in Worten.
    #[test]
    fn im_vorspann_eines_elements_endet_die_deckung() {
        assert_eq!(
            gerendert("- [ref]: http://a.example\n\n  Text\n").text,
            "• Text",
            "vor dem ersten Absatz des Punktes steht sein Merkzeichen, und dort wird nicht gedeckt"
        );
        assert_eq!(
            gerendert("> [ref]: http://a.example\n>\n> Zitat\n").text,
            "Zitat",
            "dasselbe im Zitatblock"
        );
    }

    /// Der Guertel: jede Stelle liegt innerhalb der gemeldeten Laenge, und die
    /// Laenge ist die UTF-16-Laenge des Ausgabetextes. Ohne diese Zusage waere
    /// ein Bereich in AppKit ein Programmabbruch.
    #[test]
    fn jede_stelle_liegt_innerhalb_der_laenge() {
        let quelle = "# Titel mit `Code`\n\n- ein *Punkt* mit [Verweis](https://example.com)\n\n\
                      > Zitat\n\n```sh\necho hallo\n```\n\n![Bild](b.png)\n\n---\n";
        let ergebnis = gerendert(quelle);
        assert_eq!(
            ergebnis.formatierung.laenge,
            ergebnis.text.encode_utf16().count(),
            "die Laenge ist die UTF-16-Laenge des Ausgabetextes"
        );
        for stelle in &ergebnis.formatierung.auszeichnungen {
            assert!(
                stelle.anfang + stelle.laenge <= ergebnis.formatierung.laenge,
                "Auszeichnung {stelle:?} liegt ausserhalb"
            );
        }
        for stueck in &ergebnis.formatierung.einfaerbungen {
            assert!(
                stueck.anfang + stueck.laenge <= ergebnis.formatierung.laenge,
                "Einfaerbung {stueck:?} liegt ausserhalb"
            );
        }
    }

    /// Die Stellen sind UTF-16-Einheiten und keine Bytes und keine Zeichen.
    ///
    /// Der Text traegt beides, was auseinanderlaeuft: Umlaute (zwei Bytes, eine
    /// UTF-16-Einheit) und ein Emoji (vier Bytes, **zwei** UTF-16-Einheiten).
    /// Wer in Bytes oder in `chars` zaehlte, bekaeme hier drei verschiedene
    /// Zahlen.
    #[test]
    fn die_stellen_sind_utf16_einheiten() {
        let quelle = "Grüße 😀 an *dich*.\n";
        let ergebnis = gerendert(quelle);
        assert_eq!(ergebnis.text, "Grüße 😀 an dich.");

        let betont = stellen(&ergebnis, Auszeichnung::Betonung);
        assert_eq!(betont.len(), 1);
        let (anfang, laenge) = betont[0];
        assert_eq!(stueck(&ergebnis.text, anfang, laenge), "dich");

        // "Grüße 😀 an " sind 12 UTF-16-Einheiten, aber 16 Bytes und 11
        // Zeichen. Genau diese Zahl muss ankommen.
        assert_eq!(anfang, 12);
        assert_eq!(laenge, 4);
        assert_eq!("Grüße 😀 an ".len(), 16, "in Bytes waeren es 16");
        assert_eq!(
            "Grüße 😀 an ".chars().count(),
            11,
            "in Zeichen waeren es 11"
        );
    }

    /// Ein Absatz haengt nicht am naechsten, und ein Absatzmerkmal greift nicht
    /// auf den vorigen ueber: der Bereich eines Blocks beginnt hinter den
    /// Umbruechen, die ihn abtrennen.
    #[test]
    fn ein_block_beginnt_hinter_seiner_trennung() {
        let ergebnis = gerendert("Absatz davor.\n\n- Punkt\n");
        assert_eq!(ergebnis.text, "Absatz davor.\n\n• Punkt");
        let punkte = stellen(&ergebnis, Auszeichnung::Listenzeile { tiefe: 1 });
        assert_eq!(punkte.len(), 1);
        assert_eq!(
            stueck(&ergebnis.text, punkte[0].0, punkte[0].1),
            "• Punkt",
            "der Einzug darf den Absatz davor nicht mitnehmen"
        );
    }

    /// Eine verschachtelte Liste bleibt lesbar: der Unterpunkt faengt eine
    /// eigene Zeile an und klebt nicht am Text seines Punktes.
    #[test]
    fn eine_verschachtelte_liste_haengt_nicht_aneinander() {
        let ergebnis = gerendert("- eins\n- zwei\n  - drunter\n");
        assert_eq!(ergebnis.text, "• eins\n• zwei\n• drunter");
    }

    /// Leerer Text ergibt leeren Text und keine Stelle.
    #[test]
    fn eine_leere_quelle_ergibt_nichts() {
        let ergebnis = gerendert("");
        assert_eq!(ergebnis.text, "");
        assert_eq!(ergebnis.formatierung.laenge, 0);
        assert!(ergebnis.formatierung.auszeichnungen.is_empty());
        assert!(ergebnis.formatierung.einfaerbungen.is_empty());
    }

    /// Die Auszeichnungen stehen in Textreihenfolge, und bei gleichem Anfang
    /// das aeussere zuerst.
    ///
    /// Daran haengt eine sichtbare Aussage: `crate::appkit::textmerkmale` setzt
    /// sie in dieser Reihenfolge, und Ueberschrift und feste Schrift setzen
    /// beide die Schrift. Stuende das innere Stueck zuerst, truege der
    /// Quelltext in einer Ueberschrift die Schrift der Ueberschrift.
    #[test]
    fn die_auszeichnungen_stehen_von_aussen_nach_innen() {
        let ergebnis = gerendert("# `Code` im Titel\n");
        assert_eq!(ergebnis.text, "Code im Titel");
        let arten: Vec<Auszeichnung> = ergebnis
            .formatierung
            .auszeichnungen
            .iter()
            .map(|stelle| stelle.art)
            .collect();
        assert_eq!(
            arten,
            vec![
                Auszeichnung::Ueberschrift { stufe: 1 },
                Auszeichnung::FesteSchrift
            ],
            "die Ueberschrift umschliesst den Quelltext und steht deshalb vor ihm"
        );
    }

    /// Bei gleichem Anfang **und** gleicher Laenge entscheidet der Rang und
    /// nicht die stabile Sortierung.
    ///
    /// Gemessen an `` **`code`** `` (Defekt `260812-1805` zum
    /// Ueberschneidungssatz): dort deckten starke Betonung und feste Schrift
    /// denselben Bereich, und die Reihenfolge der Endereignisse stellte das
    /// innere Stueck nach vorn — genau verkehrt herum.
    #[test]
    fn bei_gleichem_bereich_steht_das_zuerst_geoeffnete_vorn() {
        let ergebnis = gerendert("**`code`**");
        assert_eq!(ergebnis.text, "code");
        let arten: Vec<Auszeichnung> = ergebnis
            .formatierung
            .auszeichnungen
            .iter()
            .map(|stelle| stelle.art)
            .collect();
        assert_eq!(
            arten,
            vec![Auszeichnung::StarkeBetonung, Auszeichnung::FesteSchrift],
            "die starke Betonung umschliesst den Quelltext und steht deshalb vor ihm"
        );
    }

    /// Die Formatierung sagt selbst, dass sie aus Markdown stammt; daran
    /// haengt in `crate::appkit::textmerkmale` die Grundschrift.
    #[test]
    fn die_formatierung_nennt_ihre_darstellungsart() {
        assert_eq!(
            gerendert("Text\n").formatierung.art,
            Darstellungsart::Markdown
        );
    }

    // ── Der Quellbezug (C2 der Runde 14) ─────────────────────────────────────

    /// Der Satz von Beispielen, an dem die Kachelung nachgemessen wird.
    ///
    /// Zehn Faelle, und jeder steht fuer einen Weg, auf dem Quelltext in den
    /// Durchgang kommt: der gewoehnliche Absatz, die Ueberschrift, die starke
    /// Betonung und der Verweis, die Liste ueber zwei Ebenen, der Zitatblock,
    /// der Quelltextblock, die Verweisdefinition, das Stueck in fester Schrift
    /// und ein Text, dessen Zeichen mehr als ein Byte und mehr als eine
    /// UTF-16-Einheit brauchen.
    const KACHELBEISPIELE: [&str; 10] = [
        "Ein gewoehnlicher Absatz. Und noch ein Satz dahinter.\n",
        "# Titel\n\nDarunter steht ein Absatz.\n",
        "Ein **fetter** Text mit [Verweis](https://example.com) darin.\n",
        "- eins\n  - tief\n- zwei\n",
        "> Ein Zitat ueber zwei\n> Zeilen.\n",
        "```rust\nlet x = 1;\n```\n",
        "[ref]: https://example.com\n\nText [x][ref].\n",
        "Ein `code` und *kursiv* im selben Absatz.\n",
        "Grüße 😀 an *dich*.\n",
        "- Punkt am Dateianfang\n\nund ein Absatz mit Umbruch am Ende.\n",
    ];

    /// Misst die beiden Zusagen der Kachelung an einer Quelle nach.
    ///
    /// Erstens reihen sich die Quellbereiche lueckenlos und
    /// ueberschneidungsfrei ueber `0..quelle.len()`, zweitens die Textbereiche
    /// ebenso ueber `0..formatierung.laenge`. Dazu die Zusage, die
    /// [`Abschnittsart::Woertlich`] gibt: dort stehen beide Seiten Zeichen fuer
    /// Zeichen aneinander, und daran haengt in Schritt 2 die Umrechnung einer
    /// Auswahlgrenze auf ein Byte.
    fn kachelung_pruefen(quelle: &str) -> Gerendert {
        let ergebnis = gerendert(quelle);
        assert_eq!(
            ergebnis.formatierung.laenge,
            ergebnis.text.encode_utf16().count(),
            "der mitgezaehlte Endstand ist die Laenge des Textes: {quelle:?}"
        );
        let mut quellstand = 0usize;
        let mut textstand = 0usize;
        for abschnitt in &ergebnis.quellbezug.abschnitte {
            assert_eq!(
                abschnitt.quelle.start, quellstand,
                "die Quellseite reiht sich lueckenlos: {quelle:?}, {abschnitt:?}"
            );
            assert!(
                abschnitt.quelle.end >= abschnitt.quelle.start,
                "kein Abschnitt laeuft rueckwaerts: {quelle:?}, {abschnitt:?}"
            );
            assert_eq!(
                abschnitt.text.start, textstand,
                "die Textseite reiht sich lueckenlos: {quelle:?}, {abschnitt:?}"
            );
            assert!(
                abschnitt.text.end >= abschnitt.text.start,
                "kein Abschnitt laeuft rueckwaerts: {quelle:?}, {abschnitt:?}"
            );
            quellstand = abschnitt.quelle.end;
            textstand = abschnitt.text.end;
            match abschnitt.art {
                Abschnittsart::Woertlich => assert_eq!(
                    stueck(
                        &ergebnis.text,
                        abschnitt.text.start,
                        abschnitt.text.end - abschnitt.text.start
                    ),
                    quelle[abschnitt.quelle.clone()],
                    "ein woertlicher Abschnitt haelt beide Seiten aneinander: {quelle:?}"
                ),
                Abschnittsart::Ersetzt | Abschnittsart::Erzeugt => {}
            }
        }
        assert_eq!(
            quellstand,
            quelle.len(),
            "die Quellseite deckt bis zum letzten Byte: {quelle:?}"
        );
        assert_eq!(
            textstand, ergebnis.formatierung.laenge,
            "die Textseite deckt bis zur letzten Stelle: {quelle:?}"
        );
        ergebnis
    }

    /// C2.6: die Kachelung deckt beide Seiten vollstaendig.
    ///
    /// Diese Probe ist der Beweis der Totalitaet und keine Aufzaehlung von
    /// Faellen: sie faengt jeden Ereignisfall, der Quelltext abtraegt, ohne
    /// einen Abschnitt anzulegen.
    #[test]
    fn die_kachelung_deckt_quelle_und_text_lueckenlos() {
        for quelle in KACHELBEISPIELE {
            let _ = kachelung_pruefen(quelle);
        }
    }

    /// C2.7: die Umrechnung trifft an beiden Enden eines Abschnitts.
    ///
    /// „Grüße 😀 an " sind 12 UTF-16-Einheiten, aber 16 Bytes: der Umlaut und
    /// das scharfe s brauchen je zwei Bytes, das Emoji vier Bytes und zwei
    /// UTF-16-Einheiten. Ein Abschnitt, der die beiden Zaehler verwechselt,
    /// steht hier schief.
    #[test]
    fn umlaute_und_ein_emoji_treffen_beide_enden_eines_abschnitts() {
        let quelle = "Grüße 😀 an *dich*.\n";
        let ergebnis = kachelung_pruefen(quelle);
        let abschnitte = &ergebnis.quellbezug.abschnitte;
        assert_eq!(ergebnis.text, "Grüße 😀 an dich.");
        // Das erste Stueck: woertlich, und beide Zaehler stehen verschieden.
        assert_eq!(abschnitte[0].art, Abschnittsart::Woertlich);
        assert_eq!(abschnitte[0].quelle, 0..16, "in Bytes gerechnet");
        assert_eq!(abschnitte[0].text, 0..12, "in UTF-16-Einheiten gerechnet");
        // Das letzte Stueck mit Zeichen: der Punkt hinter der Betonung. Seine
        // Textstelle haengt an derselben Umrechnung, nur vom anderen Ende her.
        let letzter = abschnitte
            .iter()
            .rfind(|abschnitt| !abschnitt.text.is_empty())
            .expect("ein Abschnitt mit Zeichen");
        assert_eq!(letzter.art, Abschnittsart::Woertlich);
        assert_eq!(letzter.quelle, 22..23, "das Byte des Punktes");
        assert_eq!(letzter.text, 16..17, "die UTF-16-Stelle des Punktes");
    }

    /// C2.9: wer eine Klammer traegt und wer nicht.
    ///
    /// Die Elemente stehen in der Reihenfolge des Oeffnens, also von aussen
    /// nach innen. Die vierte Zeile ist die tragende: der Absatz des Beispiels
    /// aus dem bindenden Datensatz traegt **keine** Klammer, obwohl seine
    /// Kinder welche tragen. Truege er eine, blaehte jede Auswahl darin sich
    /// auf den ganzen Absatz auf — die vom Nutzer nicht gewaehlte
    /// Moeglichkeit 3.
    #[test]
    fn ueberschrift_betonung_verweis_und_punkt_tragen_eine_klammer_ein_absatz_nicht() {
        assert_eq!(
            klammern("# Titel\n\nEin Absatz.\n"),
            vec![(0..8, true), (9..21, false)],
            "die Ueberschrift traegt ihr `# `, der Absatz nichts"
        );
        assert_eq!(
            klammern("Ein **fetter** Text mit [Verweis](https://example.com) darin.\n"),
            vec![(0..62, false), (4..14, true), (24..54, true)],
            "der Absatz des bindenden Datensatzes traegt keine Klammer, seine Kinder schon"
        );
        assert_eq!(
            klammern("- Punkt\n"),
            vec![(0..8, false), (0..8, true)],
            "das Merkzeichen gehoert dem Punkt und nicht der Liste um ihn herum"
        );
        assert_eq!(
            klammern("> Zitat\n"),
            vec![(0..8, true), (2..8, false)],
            "das `> ` gehoert dem Zitat und nicht dem Absatz darin"
        );
    }

    /// Die Auswahl, die im gerenderten Text auf `gesucht` zeigt, in
    /// UTF-16-Einheiten.
    ///
    /// Zaehlt und rechnet nicht um: die Rechnung, die C2.7 an eine Stelle
    /// bindet, steht in [`byte_zur_stelle`] und bildet eine Textstelle auf ein
    /// Byte der **Quelle** ab. Hier wird allein die Stelle gesucht, an der der
    /// Nutzer die Maus aufgesetzt haette.
    fn auswahl(text: &str, gesucht: &str) -> Range<usize> {
        let byte = text
            .find(gesucht)
            .expect("die gesuchte Stelle steht im Text");
        let anfang = text[..byte].encode_utf16().count();
        anfang..anfang + gesucht.encode_utf16().count()
    }

    /// Der Quelltext zu der Auswahl, die im gerenderten Text auf `gesucht`
    /// zeigt.
    fn kopiert(quelle: &str, gesucht: &str) -> String {
        let ergebnis = kachelung_pruefen(quelle);
        let stelle = auswahl(&ergebnis.text, gesucht);
        ergebnis.quellbezug.quelltext(stelle).to_owned()
    }

    /// C2.2 und C2.9: das Beispiel des bindenden Datensatzes, woertlich.
    ///
    /// `shared/decisions/260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md`
    /// nennt genau diese Quelle, genau diese Auswahl und genau diese Erwartung.
    /// Eine zeichenweise Abbildung — die Moeglichkeit 1 jenes Datensatzes —
    /// lieferte hier `fetter** Text mit [Verweis`, also eine offene Betonung
    /// und einen Verweis ohne Adresse.
    #[test]
    fn das_beispiel_des_datensatzes_liefert_wohlgeformtes_markdown() {
        assert_eq!(
            kopiert(
                "Ein **fetter** Text mit [Verweis](https://example.com) darin.\n",
                "fetter Text mit Verweis"
            ),
            "**fetter** Text mit [Verweis](https://example.com)"
        );
    }

    /// C2.9: eine Auswahl innerhalb einer Ueberschrift liefert ihr Doppelkreuz.
    ///
    /// Der Quellbereich einer Ueberschrift reicht bis hinter ihren
    /// Zeilenumbruch, also faehrt der mit; das Doppelkreuz ist der Punkt, und
    /// ein Umbruch am Ende schadet in keiner Zwischenablage.
    #[test]
    fn eine_auswahl_in_einer_ueberschrift_liefert_ihr_doppelkreuz() {
        assert_eq!(kopiert("# Überschrift\n", "berschr"), "# Überschrift\n");
    }

    /// C2.9: eine Auswahl im Text eines Verweises liefert seine Adresse mit.
    #[test]
    fn eine_auswahl_im_text_eines_verweises_liefert_die_ganze_adresse() {
        assert_eq!(
            kopiert("Ein [Verweis](https://example.com) im Satz.\n", "erwei"),
            "[Verweis](https://example.com)"
        );
    }

    /// **Die Probe, die Moeglichkeit 3 ausschliesst.**
    ///
    /// Eine Auswahl mitten in einem langen Absatz liefert die markierten
    /// Zeichen und nicht den Absatz. Der Absatz traegt keine Klammer, also
    /// erweitert die zweite Stufe nichts; truege er eine — zaehlte
    /// [`klammer_der_raender`] etwa die verdeckten Bytes in seinem Inneren
    /// statt der Zeichen an seinen Raendern —, stuende hier der ganze Satz.
    #[test]
    fn eine_auswahl_in_einem_langen_absatz_liefert_nicht_den_absatz() {
        assert_eq!(
            kopiert(
                "Ein recht langer Absatz mit vielen Woertern, aus dem nur zwei \
                 Woerter markiert sind und sonst nichts.\n",
                "zwei Woerter"
            ),
            "zwei Woerter"
        );
    }

    /// C2.8: die Auswahl ueber alles liefert die Quelle byteweise vollstaendig.
    ///
    /// Gemessen an einer Datei, die mit einem Listenpunkt beginnt und mit einem
    /// Zeilenumbruch endet — die beiden Raender, an denen die Kachelung sonst
    /// ausfiele. Das `- ` steht vor dem ersten Quellzeichen des gerenderten
    /// Textes, der Umbruch hinter dem letzten; beide kommen ueber die
    /// Halbregel fuer Abschnitte ohne Textzeichen mit und brauchen dafuer keine
    /// Sonderregel.
    #[test]
    fn die_auswahl_ueber_alles_liefert_die_quelle_vollstaendig() {
        for quelle in KACHELBEISPIELE {
            let ergebnis = kachelung_pruefen(quelle);
            assert_eq!(
                ergebnis
                    .quellbezug
                    .quelltext(0..ergebnis.formatierung.laenge),
                quelle,
                "die Auswahl ueber alles liefert die Datei: {quelle:?}"
            );
        }
    }

    /// C2.9: ueber verschachtelte Elemente ist es dasselbe Verfahren.
    ///
    /// Drei Buchstaben mitten in der inneren Betonung erweitern im ersten
    /// Durchgang auf sie und im zweiten auf die aeussere. Eine zweite Regel
    /// fuer die Schachtelung gibt es nicht.
    #[test]
    fn eine_auswahl_im_verschachtelten_element_liefert_das_aeussere_ganz() {
        assert_eq!(
            kopiert("**fett *und kursiv* zugleich**\n", "und"),
            "**fett *und kursiv* zugleich**"
        );
    }

    /// C2.7: die Auswahlgrenzen treffen die Bytegrenzen.
    ///
    /// „Grü" sind drei UTF-16-Einheiten und vier Bytes, das Emoji zwei
    /// Einheiten und vier Bytes. Wer die beiden Zaehler verwechselt, schneidet
    /// hier mitten in ein Zeichen und bricht beim Zugriff auf die Quelle ab.
    #[test]
    fn eine_auswahl_zwischen_umlauten_und_einem_emoji_trifft_die_bytegrenzen() {
        assert_eq!(kopiert("Grüße 😀 an *dich*.\n", "ße 😀"), "ße 😀");
        assert_eq!(kopiert("Grüße 😀 an *dich*.\n", "dich"), "*dich*");
    }

    /// C2.9: der Quelltextblock traegt seine Zaeune, das Stueck in der Zeile
    /// seine Haken.
    ///
    /// Die beiden Faelle, die die Probe darueber nicht nennt, und der zweite
    /// ist der einzige Weg in den Zweig „kein Ereignis darin" von
    /// [`klammer_der_raender`]: ein Stueck fester Schrift in der Zeile kommt
    /// als ein einziges Ereignis herein, hat also kein Inneres, und sein
    /// ganzer Quellbereich faellt in beide Spannen.
    #[test]
    fn der_quelltextblock_und_das_stueck_in_der_zeile_tragen_ihre_zeichen() {
        assert_eq!(
            klammern("```rust\nlet x = 1;\n```\n"),
            vec![(0..22, true)],
            "die Zaeune stehen vor und hinter dem Text des Blocks"
        );
        assert_eq!(
            klammern("Ein `code` im Satz.\n"),
            vec![(0..20, false), (4..10, true)],
            "die Haken gehoeren dem Stueck und nicht dem Absatz um es herum"
        );
    }

    /// C2.9: eine Entitaet, ein Escape und ein harter Umbruch mitten in einem
    /// Absatz geben ihm keine Klammer (Defekt `260820-0728`).
    ///
    /// Sie stehen **nicht** an seinen Raendern, also zerschneidet eine Auswahl
    /// sie nicht, und der Absatz bleibt ohne Klammer. Truege er eine, blaehte
    /// sich jede Auswahl darin auf ihn auf — die vom Nutzer nicht gewaehlte
    /// Moeglichkeit 3, und zwar bei jedem `&amp;`, jedem `\*` und jedem harten
    /// Umbruch mit Backslash.
    #[test]
    fn eine_entitaet_oder_ein_escape_im_absatz_blaeht_die_auswahl_nicht_auf() {
        assert_eq!(
            kopiert("Ein &amp; hier im Absatz mit vielen Woertern.\n", "vielen"),
            "vielen",
            "die Entitaet steht mitten im Absatz und zerschneidet nichts"
        );
        assert_eq!(
            kopiert("Ein \\* Stern im Absatz mit vielen Woertern.\n", "vielen"),
            "vielen",
            "das Escape steht mitten im Absatz und zerschneidet nichts"
        );
        assert_eq!(
            kopiert("Zeile eins\\\nund vielen Woertern dahinter.\n", "vielen"),
            "vielen",
            "der harte Umbruch mit Backslash steht mitten im Absatz"
        );
        assert_eq!(
            klammern("Ein &amp; hier im Absatz.\n"),
            vec![(0..26, false)],
            "der Absatz traegt an seinen Raendern nichts als seinen Umbruch"
        );
    }

    /// C2.2 und C2.9: eine Ueberschrift, die mit einem Kind beginnt, behaelt
    /// ihr Doppelkreuz (Defekt `260820-0731`).
    ///
    /// Ihr Vorspann ist das `# `, und er gehoert ihr, gleich ob dahinter erst
    /// eine Betonung, ein Stueck fester Schrift oder ein Verweis kommt. Ohne
    /// diese Zusage kaeme eine Auswahl in ihrem Schwanz als gewoehnlicher
    /// Absatz in der Zieldatei an.
    #[test]
    fn eine_ueberschrift_mit_einem_kind_am_anfang_behaelt_ihr_doppelkreuz() {
        assert_eq!(
            kopiert("# **Titel** und noch ein Stueck Text\n", "noch ein"),
            "# **Titel** und noch ein Stueck Text\n",
            "die Betonung am Anfang nimmt der Ueberschrift ihr `# ` nicht"
        );
        assert_eq!(
            kopiert("## `code` und noch ein Stueck Text\n", "noch ein"),
            "## `code` und noch ein Stueck Text\n",
            "das Stueck fester Schrift am Anfang ebenso wenig"
        );
        assert_eq!(
            kopiert("# [V](https://e.com) und noch ein Stueck\n", "noch ein"),
            "# [V](https://e.com) und noch ein Stueck\n",
            "und der Verweis am Anfang ebenso wenig"
        );
        assert_eq!(
            klammern("# **Titel** hier\n"),
            vec![(0..17, true), (2..11, true)],
            "die Ueberschrift traegt ihr `# `, die Betonung ihre Sternchen"
        );
    }

    /// Die Elemente einer Quelle mit ihrer Klammer, in der Reihenfolge des
    /// Oeffnens, also von aussen nach innen.
    fn klammern(quelle: &str) -> Vec<(Range<usize>, bool)> {
        kachelung_pruefen(quelle)
            .quellbezug
            .elemente
            .iter()
            .map(|element| (element.quelle.clone(), element.klammer))
            .collect()
    }
}
