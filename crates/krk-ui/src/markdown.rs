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
//! # Die Deckung: kein Quellbyte faellt heraus
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
//! abgetragen ist, und daran haengen zwei Saetze, die zusammen jedes Byte
//! treffen:
//!
//! 1. **Auf Dokumentebene**, also wenn kein Element offen ist, wird die Luecke
//!    vor dem naechsten Ereignis woertlich ausgegeben ([`Zerlegung::luecke_bis`]);
//!    am Ende des Durchgangs ebenso bis [`str::len`]. Eine Luecke, in der nur
//!    Leerraum steht, faellt weg: die Abstaende zwischen den Bloecken rechnet
//!    [`Zerlegung::absetzen`], und sie ein zweites Mal zu schreiben ergaebe
//!    Leerzeilen.
//! 2. **Innerhalb eines Elements** deckt das Element seinen ganzen
//!    Quellbereich: hat es Zeichen geliefert, sind die Luecken darin seine
//!    Auszeichnungszeichen und gehoeren weg — das `[` und das `][ref]` eines
//!    Verweises. Hat es **kein** Zeichen geliefert, gibt es beim Schliessen
//!    seinen Quellbereich woertlich heraus ([`Zerlegung::schliessen`]); so
//!    bleibt `[](https://example.com)` stehen, statt zu verschwinden.
//!
//! Beides ist mechanisch und zaehlt keine Faelle auf: der erste Satz fragt
//! allein, ob [`Zerlegung::offen`] leer ist, der zweite allein, ob die Laenge
//! null ist. Eine kuenftige Fassung der Kiste, die ein Element anders meldet,
//! aendert daran nichts.
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

use std::ops::Range;

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
        match ereignis {
            Event::Start(tag) => match behandlung(&tag) {
                Behandlung::Block { umbrueche, art } => {
                    zerlegung.trennen(umbrueche);
                    zerlegung.oeffnen(bereich, Abschluss::von(art), umbrueche, None);
                }
                Behandlung::Zitat => zerlegung.zitat_oeffnen(bereich),
                Behandlung::Liste { erste } => {
                    zerlegung.trennen(PUNKTABSTAND);
                    zerlegung.oeffnen(
                        bereich,
                        Abschluss::Nichts,
                        PUNKTABSTAND,
                        Some(Ebene::Liste(erste)),
                    );
                }
                Behandlung::Punkt => zerlegung.punkt_oeffnen(bereich),
                Behandlung::Stueck(art) => {
                    zerlegung.oeffnen(bereich, Abschluss::Auszeichnung(art), 0, None);
                }
                Behandlung::Verweis => zerlegung.oeffnen(bereich, Abschluss::Verweis, 0, None),
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
            Event::Text(inhalt) => {
                zerlegung.schreiben(&inhalt);
                zerlegung.gelesen_bis(bereich.end);
            }
            Event::Code(inhalt) => {
                zerlegung.oeffnen(
                    bereich,
                    Abschluss::Auszeichnung(Auszeichnung::FesteSchrift),
                    0,
                    None,
                );
                zerlegung.schreiben(&inhalt);
                zerlegung.schliessen();
            }
            // Die Zeile, an der das Quelltextraster einer Tabelle haengt: die
            // drei Zeilen einer Tabelle sind ein Absatz mit weichen
            // Umbruechen, und der Umbruch bleibt einer.
            Event::SoftBreak | Event::HardBreak => {
                zerlegung.schreiben("\n");
                zerlegung.gelesen_bis(bereich.end);
            }
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
    /// Der wievielte geoeffnete Bereich dieser ist.
    ///
    /// Die Reihenfolge des Oeffnens laeuft von aussen nach innen und ist damit
    /// die Reihenfolge, in der [`Zerlegung::abschliessen`] bei gleichem Anfang
    /// und gleicher Laenge sortiert.
    rang: usize,
    /// Die Einrueckebene, die dieses Element anlegt.
    ebene: Option<Ebene>,
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
    /// Wie viele Bereiche bisher geoeffnet wurden; die Quelle der Raenge.
    raenge: usize,
    einfaerbungen: Vec<Einfaerbung>,
    /// Die Auszeichnungen mit dem Rang ihres Elements.
    auszeichnungen: Vec<(usize, Auszeichnungsstelle)>,
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
            raenge: 0,
            einfaerbungen: Vec::new(),
            auszeichnungen: Vec::new(),
        }
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
        for _ in 0..fehlend {
            self.text.push('\n');
            self.stelle += 1;
        }
        for eintrag in &mut self.offen {
            if eintrag.anfang == vorher {
                eintrag.anfang = self.stelle;
            }
        }
    }

    /// Schreibt ein Stueck Text und zaehlt seine UTF-16-Einheiten mit.
    fn schreiben(&mut self, stueck: &str) {
        if stueck.is_empty() {
            return;
        }
        self.absetzen();
        self.text.push_str(stueck);
        self.stelle += stueck.encode_utf16().count();
    }

    /// Traegt die Quelle bis hierhin als gelesen ab.
    fn gelesen_bis(&mut self, bis: usize) {
        self.gelesen = self.gelesen.max(bis);
    }

    /// Schreibt einen Quellbereich woertlich und traegt ihn ab.
    ///
    /// Die Auffangregel und die Deckung treffen sich hier: was woertlich
    /// dasteht, ist gelesen.
    fn woertlich(&mut self, bereich: Range<usize>) {
        let quelle = self.quelle;
        self.schreiben(&quelle[bereich.clone()]);
        self.gelesen_bis(bereich.end);
    }

    /// Der erste Satz der Deckung: gibt heraus, was auf Dokumentebene bis
    /// hierhin ungelesen blieb.
    ///
    /// **Nur wenn kein Element offen ist.** Innerhalb eines Elements sind die
    /// Luecken zwischen den Ereignissen seine Auszeichnungszeichen, und die
    /// gehoeren weg; gedeckt ist das Element dort ueber seinen eigenen
    /// Quellbereich, siehe [`Zerlegung::schliessen`].
    ///
    /// **Leerraum faellt weg.** Die Abstaende zwischen den Bloecken rechnet
    /// [`Zerlegung::absetzen`] aus dem Wunsch des Blocks; der Leerraum der
    /// Quelle daneben ergaebe Leerzeilen. Was uebrig bleibt, ist ein Block und
    /// wird wie einer abgesetzt.
    fn luecke_bis(&mut self, bis: usize) {
        if !self.offen.is_empty() || self.gelesen >= bis {
            return;
        }
        let quelle = self.quelle;
        let uebergangen = quelle[self.gelesen..bis].trim();
        self.gelesen = bis;
        if uebergangen.is_empty() {
            return;
        }
        self.trennen(ABSATZABSTAND);
        self.schreiben(uebergangen);
        self.trennen(ABSATZABSTAND);
    }

    /// Beginnt ein Element an der aktuellen Stelle.
    fn oeffnen(&mut self, quelle: Range<usize>, was: Abschluss, nach: usize, ebene: Option<Ebene>) {
        self.absetzen();
        self.raenge += 1;
        self.offen.push(Offen {
            anfang: self.stelle,
            was,
            nach,
            quelle,
            rang: self.raenge,
            ebene,
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

    /// Beginnt einen Listenpunkt: Merkzeichen im Text, Tiefe in der
    /// Auszeichnung.
    ///
    /// Das Merkzeichen steht **innerhalb** des Bereichs der Listenzeile, denn
    /// der Einzug soll es mitnehmen; genau das sagt der Kommentar an
    /// `einzugsmerkmal` in `crate::appkit::textmerkmale` zu.
    fn punkt_oeffnen(&mut self, quelle: Range<usize>) {
        let tiefe = self.tiefe();
        let merkzeichen = self.merkzeichen();
        self.trennen(PUNKTABSTAND);
        self.oeffnen(
            quelle,
            Abschluss::Auszeichnung(Auszeichnung::Listenzeile { tiefe }),
            PUNKTABSTAND,
            None,
        );
        self.schreiben(&merkzeichen);
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
    /// geliefert, waere die Laenge nicht null.
    fn schliessen(&mut self) {
        let Some(eintrag) = self.offen.pop() else {
            return;
        };
        let laenge = self.stelle - eintrag.anfang;
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
            self.gelesen_bis(eintrag.quelle.end);
        } else {
            self.woertlich(eintrag.quelle);
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
}
