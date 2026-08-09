//! Der Inhalt der Belegungsansicht aus C3, ohne AppKit.
//!
//! **Die Ansicht fuehrt keine eigene Tabelle.** Dieses Modul haelt waehrend
//! einer offenen Belegungsansicht genau eine [`Belegung`] — die Arbeitskopie —
//! und reicht jede Frage an sie weiter: die Zeilen sind ihre Funktionen, die
//! Zuweisung geht ueber [`Belegung::zuweisen`], das Zuruecksetzen ueber
//! [`Belegung::zuruecksetzen`], und die Konfliktmeldung kommt woertlich aus
//! `krk_core::tasten::konflikt`. Was hier dazukommt, ist allein die
//! Anzeigeform und das Kennzeichen, ob sich etwas geaendert hat.
//!
//! ```text
//! Kommando::BelegungAnsehen ──> Belegungsmodell (Arbeitskopie der Belegung)
//!                                    │  zuweisen / zuruecksetzen
//!                                    ▼
//!            beim Verlassen: in_belegung ──> Belegung::sichern (keymap.toml)
//! ```
//!
//! # Eine Zeile je Funktion, gegliedert nach Funktionsbereich
//!
//! C3 verlangt: genau eine Zeile je Funktion, alle Kombinationen dieser
//! Funktion in dieser einen Zeile. Das ist hier keine Rechenleistung, sondern
//! die Gestalt der Belegung selbst: [`Belegung::funktionen`] fuehrt jede
//! Funktion genau einmal, mit allen ihren Kombinationen.
//!
//! Angezeigt werden die Funktionen nicht in der Reihenfolge der Datei,
//! sondern gruppiert nach [`Funktionsbereich`] (Nutzerauftrag vom 260806,
//! `issues/260806-1054_*_belegungsansicht-gruppiert-nach-funktionsbereich.md`):
//! vor den Funktionen eines Bereichs steht eine Ueberschriftszeile mit seinem
//! Namen. Die Zuordnung Funktion → Bereich steht an genau einer Stelle,
//! [`bereich`], und dort als vollstaendige Fallunterscheidung ueber
//! [`Kommando`] ohne Auffangzweig: ein neues Kommando uebersetzt nicht, bevor
//! es seinen Bereich genannt hat. Die wenigen Funktionen ohne Kommando (die
//! sechs Textbefehle des Menues) stehen daneben mit Namen; dass keine
//! vergessen ist, prueft
//! `jede_kennung_hat_einen_funktionsbereich` gegen die
//! Auslieferungsbelegung. Innerhalb eines Bereichs bleibt die Reihenfolge
//! der Datei erhalten — eine zweite Ordnung neben ihr entsteht nicht.
//!
//! # Die Beschriftung geht ueber die Tastentabelle
//!
//! Eine Kombination schreibt sich ueber ihre [`fmt::Display`]-Form
//! (`shift+cmd+k`), und die kennt allein die Namen aus
//! `krk_core::tasten::parser::TASTEN`. Die Anzeigeform [`anzeige`] setzt
//! darauf nur Grossbuchstaben an den Teilanfang: `Shift+Cmd+K`, `F3`. Eine
//! zweite Namensliste entsteht nicht, und "Fn+" kann an keiner Stelle
//! erscheinen, weil die Schreibweise fn nicht kennt (C3, S7).

use krk_core::tasten::{Belegung, Funktion, Kombination, Kommando, Tastendruck};

/// Die Funktionsbereiche der Belegungsansicht, in der Reihenfolge der
/// Anzeige.
///
/// Ein Bereich buendelt die Funktionen, die derselben Gegend der Anwendung
/// gelten; die Ansicht setzt vor seine Funktionen eine Ueberschriftszeile.
/// [`Wirkungsbereich`](krk_core::tasten::Wirkungsbereich) traegt diese
/// Gliederung nicht: er beantwortet, welcher Bereich den Fokus haben muss,
/// und wirft dabei Fenster-, Fokus- und Anwendungsbefehle in einen Topf
/// (`Ueberall`), den kein Nutzer als Ordnung wiedererkennt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Funktionsbereich {
    /// Bewegung, Navigation, Markierung, Sortierung und Sichtbarkeit in der
    /// Dateiliste (C2, C10).
    Dateilisting,
    /// Kopieren, Verschieben, Loeschen, Anlegen, Umbenennen, Abbrechen und
    /// das Terminal im angezeigten Ordner (C4, C11).
    Dateioperationen,
    /// Die vier Tabbefehle (C1, nach C6 auch fuer die Vorschau-Tabs).
    Tabs,
    /// Das Vorschaufenster: was es anzeigt, und wie man hinkommt (C2, C3,
    /// C6, C10).
    Vorschau,
    /// Die Lesezeichen- und Geraeteleiste und der Fokuswechsel zwischen ihr
    /// und dem Dateifenster (C5).
    LeisteUndFokus,
    /// Das Anwendungsfenster und seine Bereiche: wechseln, ein- und
    /// ausblenden, Breiten (C1, C7).
    Fenster,
    /// Die Anwendung als ganze: Belegungsansicht und Beenden (C3).
    Anwendung,
    /// Die sechs Textbefehle, die das Menue "Bearbeiten" zustellt (C2, und
    /// Rueckgaengig und Wiederholen aus der Editor-Runde).
    Textbefehle,
    /// Der eingebaute Editor: die beiden Einstiegswege, der Fokus, die beiden
    /// Ansichten, das Sichern, der Zeilensprung, Suchen und Ersetzen (C1 bis
    /// C6 der Editor-Runde).
    Editor,
}

impl Funktionsbereich {
    /// Alle Bereiche, in der Reihenfolge der Anzeige.
    pub const ALLE: [Funktionsbereich; 9] = [
        Funktionsbereich::Dateilisting,
        Funktionsbereich::Dateioperationen,
        Funktionsbereich::Tabs,
        Funktionsbereich::Vorschau,
        Funktionsbereich::LeisteUndFokus,
        Funktionsbereich::Fenster,
        Funktionsbereich::Anwendung,
        Funktionsbereich::Textbefehle,
        Funktionsbereich::Editor,
    ];

    /// Die Ueberschrift des Bereichs in der Ansicht.
    pub const fn name(self) -> &'static str {
        match self {
            Funktionsbereich::Dateilisting => "Dateilisting",
            Funktionsbereich::Dateioperationen => "Dateioperationen",
            Funktionsbereich::Tabs => "Tabs",
            Funktionsbereich::Vorschau => "Vorschau",
            Funktionsbereich::LeisteUndFokus => "Leiste und Fokus",
            Funktionsbereich::Fenster => "Fenster",
            Funktionsbereich::Anwendung => "Anwendung",
            Funktionsbereich::Textbefehle => "Textbefehle",
            Funktionsbereich::Editor => "Editor",
        }
    }
}

/// Der Funktionsbereich einer Funktion, aus ihrer Kennung.
///
/// **Die eine Stelle der Zuordnung.** Fuer jede Funktion mit einem
/// [`Kommando`] antwortet die vollstaendige Fallunterscheidung in
/// [`bereich_des_kommandos`]; die Funktionen ohne Kommando stehen hier mit
/// Namen, und es sind genau die, die nie eines bekommen: die sechs vom Menue
/// zugestellten Textbefehle. `None` heisst: die Zuordnung kennt diese Kennung
/// nicht — das faengt die Pruefung `jede_kennung_hat_einen_funktionsbereich`,
/// bevor es eine Ansicht erreicht.
///
/// `bearbeiten` stand bis zur Editor-Runde hier unten, weil der F4-Eintrag
/// reserviert war und kein Kommando trug. Seit S5 traegt er
/// [`Kommando::Bearbeiten`], der Zweig darueber greift, und eine Zeile hier
/// behauptete eine zweite Wahrheit ueber denselben Namen.
pub fn bereich(kennung: &str) -> Option<Funktionsbereich> {
    if let Some(kommando) = Kommando::aus_kennung(kennung) {
        return Some(bereich_des_kommandos(kommando));
    }
    match kennung {
        "text_ausschneiden"
        | "text_kopieren"
        | "text_einfuegen"
        | "text_alles_auswaehlen"
        | "text_rueckgaengig"
        | "text_wiederholen" => Some(Funktionsbereich::Textbefehle),
        _ => None,
    }
}

/// Der Funktionsbereich jedes Kommandos, ohne Auffangzweig.
///
/// Der Uebersetzer erzwingt die Vollstaendigkeit: ein neues Kommando
/// uebersetzt nicht, bevor es hier seinen Bereich genannt hat — dasselbe
/// Muster wie [`Kommando::wirkungsbereich`] im Kern.
const fn bereich_des_kommandos(kommando: Kommando) -> Funktionsbereich {
    match kommando {
        // Die Dateiliste: Bewegung, Navigation, Markierung, Sortierung,
        // Sichtbarkeit und der Sprung zum Inhalt der Zwischenablage, der
        // dieselbe Handlung ist wie die Pfadeingabe mit vorausgefuelltem
        // Wert (C2, C10).
        Kommando::AuswahlHoch
        | Kommando::AuswahlRunter
        | Kommando::SeiteHoch
        | Kommando::SeiteRunter
        | Kommando::Listenanfang
        | Kommando::Listenende
        | Kommando::Oeffnen
        | Kommando::OrdnerAufwaerts
        | Kommando::Pfadeingabe
        | Kommando::MarkierungUmschalten
        | Kommando::AlleMarkieren
        | Kommando::MarkierungAufheben
        | Kommando::MarkierungUmkehren
        | Kommando::SortierungName
        | Kommando::SortierungGroesse
        | Kommando::SortierungDatum
        | Kommando::SortierungTyp
        | Kommando::SortierrichtungUmkehren
        | Kommando::VersteckteUmschalten
        | Kommando::ZwischenablageSpringen => Funktionsbereich::Dateilisting,
        // Die Dateioperationen aus C4 und der Terminal-Befehl aus C11, der
        // wie sie auf dem angezeigten Ordner arbeitet.
        Kommando::Kopieren
        | Kommando::Verschieben
        | Kommando::InPapierkorb
        | Kommando::EndgueltigLoeschen
        | Kommando::Abbrechen
        | Kommando::OrdnerAnlegen
        | Kommando::DateiAnlegen
        | Kommando::UmbenennenStapel
        | Kommando::Umbenennen
        | Kommando::TerminalOeffnen => Funktionsbereich::Dateioperationen,
        Kommando::TabNeu
        | Kommando::TabSchliessen
        | Kommando::TabNaechster
        | Kommando::TabVoriger => Funktionsbereich::Tabs,
        // Das Ein- und Ausblenden der Vorschau steht bei ihr und nicht bei
        // den Fensterbefehlen: wer die Vorschau sucht, sucht unter Vorschau,
        // und "Zwischenablage ansehen" zeigt in dasselbe Fenster (C3, C10).
        //
        // Derselbe Satz ordnet den Fokusbefehl aus C2/C6 hierher und nicht zu
        // "Leiste und Fokus": diese Gliederung fragt nach der Gegend der
        // Anwendung und nicht nach dem Mechanismus, sonst stuende auch
        // `leiste_umschalten` unter "Fenster". Wer wissen will, wie er in die
        // Vorschau kommt, findet unter "Vorschau" alle drei Befehle, die sie
        // angehen.
        Kommando::VorschauUmschalten
        | Kommando::ZwischenablageAnsehen
        | Kommando::FokusVorschau => Funktionsbereich::Vorschau,
        // Die Leiste aus C5 samt ihrem Ein- und Ausblenden aus C7 und den
        // beiden Fokusbefehlen, die zwischen ihr und dem Dateifenster
        // wechseln.
        Kommando::LesezeichenAnlegen
        | Kommando::LesezeichenUmbenennen
        | Kommando::LesezeichenLoeschen
        | Kommando::LesezeichenHoch
        | Kommando::LesezeichenRunter
        | Kommando::FokusLeiste
        | Kommando::FokusDateifenster
        | Kommando::LeisteUmschalten => Funktionsbereich::LeisteUndFokus,
        // Das Anwendungsfenster und seine Bereiche (C1, C7).
        Kommando::FensterWechseln
        | Kommando::ZweitesFensterUmschalten
        | Kommando::FensterEinblenden
        | Kommando::FensterSchliessen
        | Kommando::BereichVerbreitern
        | Kommando::BereichVerschmaelern => Funktionsbereich::Fenster,
        Kommando::BelegungAnsehen | Kommando::Beenden => Funktionsbereich::Anwendung,
        // Der eingebaute Editor, und `bearbeiten` steht mit darin.
        //
        // Es ist die einzige Stelle, an der diese Gliederung und
        // [`Kommando::wirkungsbereich`] auseinandergehen, und der Grund ist
        // derselbe wie beim Ein- und Ausblenden der Vorschau weiter oben: die
        // Gliederung fragt nach der **Gegend der Anwendung**, der
        // Wirkungsbereich nach dem Fokus, den ein Befehl braucht. F4 braucht
        // das Dateifenster, aber wer die Zeile sucht, sucht sie unter "Editor"
        // und nicht unter "Dateioperationen". Derselbe Satz ordnet den
        // Uebergang aus der Vorschau hierher und nicht zu "Vorschau": beide
        // sind Einstiegswege in den Editor, und der Nutzer findet unter
        // "Editor" alle Befehle, die ihn angehen.
        Kommando::Bearbeiten
        | Kommando::EditorAusVorschau
        | Kommando::FokusEditor
        | Kommando::EditorSchliessen
        | Kommando::EditorAnsichtUmschalten
        | Kommando::EditorSichern
        | Kommando::EditorZeileSpringen
        | Kommando::EditorSuchen
        | Kommando::EditorWeitersuchen
        | Kommando::EditorRueckwaertsSuchen
        | Kommando::EditorErsetzen
        | Kommando::EditorAlleErsetzen => Funktionsbereich::Editor,
    }
}

/// Eine Zeile der Ansicht: eine Bereichsueberschrift oder eine Funktion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zeile {
    /// Die Ueberschriftszeile vor den Funktionen eines Bereichs.
    Ueberschrift(Funktionsbereich),
    /// Eine Funktion, als Stelle in [`Belegung::funktionen`].
    Funktion(usize),
}

/// Was aus dem Versuch geworden ist, der ausgewaehlten Funktion die gedrueckte
/// Kombination zu geben.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zuweisung {
    /// Die Funktion traegt die Kombination jetzt. Beide Namen stehen in der
    /// Anzeigeform fuer die Bestaetigung.
    Zugewiesen {
        /// Die Beschriftung der Funktion.
        funktion: String,
        /// Die Kombination in der Anzeigeform.
        kombination: String,
    },
    /// Die gedrueckte Taste hat in der Schreibweise keinen Namen (Satzzeichen,
    /// Zehnerblock) und liesse sich nicht wieder aus `keymap.toml` einlesen.
    OhneNamen,
    /// Die Belegung hat die Zuweisung abgewiesen; der Text nennt den Grund
    /// und kommt woertlich aus dem Kern, samt der anderen Funktion bei einem
    /// Konflikt.
    Abgelehnt(String),
}

/// Die Arbeitskopie der Belegung, solange die Ansicht offen ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Belegungsmodell {
    belegung: Belegung,
    /// Die Zeilen der Ansicht: je Bereich eine Ueberschrift, darunter seine
    /// Funktionen. Gebaut aus der Belegung in [`gliederung`]; neu gebaut nur
    /// beim Zuruecksetzen, denn eine Zuweisung aendert keine Zeile, nur ihren
    /// Inhalt.
    zeilen: Vec<Zeile>,
    /// Ob eine Zuweisung oder ein Zuruecksetzen gelungen ist. Nur dann wird
    /// beim Verlassen gesichert; eine unveraenderte Ansicht schreibt nichts.
    geaendert: bool,
}

impl Belegungsmodell {
    /// Ein Modell ueber der uebergebenen Belegung, ohne Aenderung.
    pub fn neu(belegung: Belegung) -> Self {
        Self {
            zeilen: gliederung(&belegung),
            belegung,
            geaendert: false,
        }
    }

    /// Wie viele Zeilen die Ansicht fuehrt: eine je Funktion, dazu die
    /// Ueberschrift vor jedem Bereich.
    pub fn zeilen(&self) -> usize {
        self.zeilen.len()
    }

    /// Die Bereichsueberschrift an dieser Stelle, falls dort eine steht.
    ///
    /// `None` heisst: die Zeile ist eine Funktion. Die Ansicht fragt das fuer
    /// die Gruppenzeilen der Tabelle ab, und eine Ueberschriftszeile ist
    /// nicht auswaehlbar und nimmt keine Zuweisung an.
    pub fn ueberschrift(&self, stelle: usize) -> Option<&'static str> {
        match self.zeilen.get(stelle)? {
            Zeile::Ueberschrift(bereich) => Some(bereich.name()),
            Zeile::Funktion(_) => None,
        }
    }

    /// Die erste Zeile, die eine Funktion traegt, fuer die Auswahl beim
    /// Oeffnen: die Zeile 0 ist seit der Gliederung eine Ueberschrift.
    pub fn erste_funktionszeile(&self) -> Option<usize> {
        self.zeilen
            .iter()
            .position(|zeile| matches!(zeile, Zeile::Funktion(_)))
    }

    /// Die naechste waehlbare Zeile zu einer wiederherzustellenden Stelle.
    ///
    /// Die Stelle selbst, falls sie in der Liste liegt und eine Funktion
    /// traegt; sonst die erste Funktionszeile. `None` heisst, dass es keine
    /// Funktion gibt, was in einer geladenen Belegung nicht vorkommt.
    ///
    /// Reines Rust und deshalb hier und nicht in der Ansicht: die Entscheidung
    /// ist ohne Fenster pruefbar. Wozu es sie gibt, steht an der einen
    /// Aufrufstelle in `appkit/belegungsansicht.rs` — der programmatische Weg,
    /// eine Zeile auszuwaehlen, fragt die Sperre fuer Ueberschriften nicht.
    pub fn waehlbare_zeile(&self, stelle: usize) -> Option<usize> {
        if stelle < self.zeilen.len() && self.ueberschrift(stelle).is_none() {
            return Some(stelle);
        }
        self.erste_funktionszeile()
    }

    /// Die Funktion hinter dieser Zeile, falls die Zeile eine traegt.
    fn funktion(&self, stelle: usize) -> Option<&Funktion> {
        match self.zeilen.get(stelle)? {
            Zeile::Funktion(funktionsstelle) => self.belegung.funktionen().get(*funktionsstelle),
            Zeile::Ueberschrift(_) => None,
        }
    }

    /// Die Beschriftung der Funktion an dieser Stelle, fuer die Spalte
    /// "Funktion".
    ///
    /// Eine reservierte Funktion traegt den Vorbehalt im Text, wie C3 es fuer
    /// den F4-Eintrag verlangte; eine vom Hauptmenue zugestellte den
    /// Zusteller, damit die beiden Cmd+A-Zeilen unterscheidbar sind.
    ///
    /// Die Auslieferungsbelegung fuehrt seit der Editor-Runde keine
    /// reservierte Funktion mehr — `bearbeiten` traegt seit S6 die Taste F4.
    /// Der Zweig bleibt trotzdem stehen: `reserviert_fuer` ist ein Feld der
    /// Belegungsdatei, und eine `keymap.toml` aus einer aelteren Fassung kann
    /// es weiterhin tragen.
    pub fn funktionstext(&self, stelle: usize) -> Option<String> {
        let funktion = self.funktion(stelle)?;
        let mut text = funktion.name().to_owned();
        if let Some(wofuer) = funktion.reserviert_fuer() {
            let wofuer = match wofuer {
                "editor" => "den Editor",
                andere => andere,
            };
            text.push_str(&format!(" (reserviert für {wofuer})"));
        }
        if let Some(zusteller) = funktion.gehalten_von() {
            let zusteller = match zusteller {
                "menue" => "Kürzel des Menüs",
                andere => andere,
            };
            text.push_str(&format!(" ({zusteller})"));
        }
        Some(text)
    }

    /// Alle Kombinationen der Funktion an dieser Stelle, in der Anzeigeform,
    /// fuer die Spalte "Belegung".
    pub fn tastentext(&self, stelle: usize) -> Option<String> {
        let funktion = self.funktion(stelle)?;
        Some(
            funktion
                .tasten()
                .iter()
                .map(anzeige)
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    /// Der blosse Name der Funktion an dieser Stelle, fuer die Aufforderung
    /// waehrend der Aufnahme.
    pub fn name(&self, stelle: usize) -> Option<&str> {
        self.funktion(stelle).map(|funktion| funktion.name())
    }

    /// Gibt der Funktion an dieser Stelle die gedrueckte Kombination.
    ///
    /// Der Tastendruck kommt normalisiert herein; zwei Druecke, die sich
    /// allein durch gehaltenes fn unterscheiden, sind hier schon derselbe
    /// (S7), und eine fn-Kombination ist deshalb nicht anlegbar.
    pub fn zuweisen(&mut self, stelle: usize, druck: Tastendruck) -> Zuweisung {
        let Some(kombination) = Kombination::aus_tastendruck(druck) else {
            return Zuweisung::OhneNamen;
        };
        let Some(funktion) = self.funktion(stelle) else {
            return Zuweisung::Abgelehnt("es ist keine Funktion ausgewählt".to_owned());
        };
        let kennung = funktion.kennung().to_owned();
        let name = funktion.name().to_owned();
        match self.belegung.zuweisen(&kennung, kombination) {
            Ok(()) => {
                self.geaendert = true;
                Zuweisung::Zugewiesen {
                    funktion: name,
                    kombination: anzeige(&kombination),
                }
            }
            Err(fehler) => Zuweisung::Abgelehnt(fehler.to_string()),
        }
    }

    /// Setzt die Arbeitskopie auf den Auslieferungszustand zurueck (C3).
    ///
    /// Die Zeilen werden neu gebaut: eine Belegung des Nutzers darf ihre
    /// Funktionen anders anordnen als die Auslieferung, und innerhalb eines
    /// Bereichs folgt die Anzeige dieser Ordnung.
    pub fn zuruecksetzen(&mut self) {
        self.belegung.zuruecksetzen();
        self.zeilen = gliederung(&self.belegung);
        self.geaendert = true;
    }

    /// Ob beim Verlassen zu sichern ist.
    pub fn geaendert(&self) -> bool {
        self.geaendert
    }

    /// Die Arbeitskopie, fuer das Sichern und den weiteren Betrieb.
    pub fn in_belegung(self) -> Belegung {
        self.belegung
    }
}

impl Default for Belegungsmodell {
    /// Die Auslieferungsbelegung, unveraendert.
    ///
    /// Der Wert, den `RefCell::take` beim Abschliessen der Ansicht
    /// zuruecklaesst; er wird nie angezeigt und nie gesichert.
    fn default() -> Self {
        Self::neu(Belegung::auslieferung())
    }
}

/// Die Zeilen der Ansicht ueber einer Belegung: je Bereich eine Ueberschrift,
/// darunter seine Funktionen in der Reihenfolge der Datei.
///
/// Ein Bereich ohne Funktion bekommt keine Ueberschrift; in der
/// Auslieferungsbelegung ist jeder Bereich besetzt. Eine Funktion ohne
/// Bereich waere ein Programmierfehler — eine neue Funktion ist erst
/// vollstaendig, wenn [`bereich`] sie einordnet, und die Pruefung
/// `jede_kennung_hat_einen_funktionsbereich` haelt das fest. Sie still
/// auszulassen hiesse, eine Funktion aus der Ansicht verschwinden zu lassen,
/// die C3 vollstaendig verlangt; deshalb bricht der Bau hier laut ab.
fn gliederung(belegung: &Belegung) -> Vec<Zeile> {
    let bereiche: Vec<Funktionsbereich> = belegung
        .funktionen()
        .iter()
        .map(|funktion| {
            bereich(funktion.kennung()).unwrap_or_else(|| {
                panic!(
                    "die Funktion {} hat keinen Funktionsbereich; \
                     die Zuordnung steht in belegungsmodell::bereich",
                    funktion.kennung()
                )
            })
        })
        .collect();

    let mut zeilen = Vec::with_capacity(bereiche.len() + Funktionsbereich::ALLE.len());
    for gruppe in Funktionsbereich::ALLE {
        let mut mit_ueberschrift = false;
        for (stelle, eingeordnet) in bereiche.iter().enumerate() {
            if *eingeordnet == gruppe {
                if !mit_ueberschrift {
                    zeilen.push(Zeile::Ueberschrift(gruppe));
                    mit_ueberschrift = true;
                }
                zeilen.push(Zeile::Funktion(stelle));
            }
        }
    }
    zeilen
}

/// Die Anzeigeform einer Kombination: die Schreibweise mit grossem
/// Teilanfang.
///
/// `shift+cmd+k` wird zu `Shift+Cmd+K`, `f3` zu `F3`. Mehr geschieht nicht:
/// die Namen kommen aus der einen Tastentabelle des Kerns, und eine
/// Uebersetzungsliste daneben waere die zweite Namensliste, die der Plan
/// ausschliesst.
pub fn anzeige(kombination: &Kombination) -> String {
    kombination
        .to_string()
        .split('+')
        .map(teilanfang_gross)
        .collect::<Vec<String>>()
        .join("+")
}

/// Der Teil mit grossem ersten Buchstaben.
fn teilanfang_gross(teil: &str) -> String {
    let mut zeichen = teil.chars();
    match zeichen.next() {
        Some(erstes) => erstes.to_ascii_uppercase().to_string() + zeichen.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::normalisierung::{ModMaske, roh};
    use krk_core::tasten::{Kommando, code_von_pflicht};

    use super::*;

    /// Das Modell fuehrt jede Funktion der Belegung genau einmal: eine Zeile
    /// je Funktion, und Papierkorb und endgueltiges Loeschen sind zwei davon.
    /// Dazu kommen allein die Bereichsueberschriften.
    #[test]
    fn eine_zeile_je_funktion() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let ueberschriften = (0..modell.zeilen())
            .filter(|&stelle| modell.ueberschrift(stelle).is_some())
            .count();
        assert_eq!(
            modell.zeilen(),
            Belegung::auslieferung().funktionen().len() + ueberschriften,
            "neben Funktionen und Ueberschriften gibt es keine Zeile"
        );

        let namen: Vec<String> = (0..modell.zeilen())
            .filter(|&stelle| modell.ueberschrift(stelle).is_none())
            .map(|stelle| {
                modell
                    .funktionstext(stelle)
                    .expect("jede Funktionszeile hat einen Text")
            })
            .collect();
        assert_eq!(
            namen.len(),
            Belegung::auslieferung().funktionen().len(),
            "eine Funktion fehlt in den Zeilen"
        );
        let mut sortiert = namen.clone();
        sortiert.sort();
        sortiert.dedup();
        assert_eq!(namen.len(), sortiert.len(), "eine Funktion steht zweimal");

        // Auch die blossen Namen sind eindeutig; daran haengt neben der
        // Verstaendlichkeit der Meldungen der Helfer `zeile_von`.
        let mut blosse: Vec<&str> = (0..modell.zeilen())
            .filter_map(|stelle| modell.name(stelle))
            .collect();
        blosse.sort_unstable();
        blosse.dedup();
        assert_eq!(
            blosse.len(),
            namen.len(),
            "zwei Funktionen teilen den Namen"
        );

        let belegung = Belegung::auslieferung();
        assert!(belegung.funktion("in_papierkorb").is_some());
        assert!(belegung.funktion("endgueltig_loeschen").is_some());
    }

    /// Jede Kennung der Auslieferungsbelegung hat einen Funktionsbereich.
    ///
    /// Die Haelfte der Zuordnung, die der Uebersetzer nicht erzwingen kann:
    /// eine neue Funktion ohne Kommando (reserviert oder zugestellt) faellt
    /// hier auf, bevor [`gliederung`] am lebenden Blatt abbricht.
    #[test]
    fn jede_kennung_hat_einen_funktionsbereich() {
        for funktion in Belegung::auslieferung().funktionen() {
            assert!(
                bereich(funktion.kennung()).is_some(),
                "die Funktion {} hat keinen Funktionsbereich",
                funktion.kennung()
            );
        }
    }

    /// Die Zeilen sind nach Funktionsbereichen gegliedert: die erste Zeile
    /// ist eine Ueberschrift, die Ueberschriften folgen der Reihenfolge von
    /// [`Funktionsbereich::ALLE`], und jede kommt hoechstens einmal vor.
    #[test]
    fn die_zeilen_sind_nach_bereichen_gegliedert() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        assert!(
            modell.ueberschrift(0).is_some(),
            "vor der ersten Funktion steht keine Ueberschrift"
        );
        assert_eq!(modell.erste_funktionszeile(), Some(1));

        let gesehen: Vec<&'static str> = (0..modell.zeilen())
            .filter_map(|stelle| modell.ueberschrift(stelle))
            .collect();
        let erwartet: Vec<&'static str> = Funktionsbereich::ALLE
            .iter()
            .map(|bereich| bereich.name())
            .filter(|name| gesehen.contains(name))
            .collect();
        assert_eq!(
            gesehen, erwartet,
            "die Ueberschriften folgen nicht der Bereichsreihenfolge oder eine steht doppelt"
        );
        // In der Auslieferungsbelegung ist jeder Bereich besetzt.
        assert_eq!(gesehen.len(), Funktionsbereich::ALLE.len());
    }

    /// Eine Ueberschriftszeile nimmt keine Zuweisung an.
    #[test]
    fn eine_ueberschrift_nimmt_keine_zuweisung_an() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
        assert_eq!(
            modell.zuweisen(0, druck),
            Zuweisung::Abgelehnt("es ist keine Funktion ausgewählt".to_owned())
        );
        assert!(!modell.geaendert());
    }

    /// Die Auswahlwiederherstellung nach `reloadData` weicht von jeder Zeile
    /// aus, die nicht waehlbar ist: von einer Ueberschrift und von einer
    /// Stelle hinter dem Ende der Liste.
    #[test]
    fn die_auswahlwiederherstellung_meidet_ueberschriften_und_das_listenende() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let erste = modell
            .erste_funktionszeile()
            .expect("die Auslieferung hat Funktionen");
        assert_eq!(
            modell.waehlbare_zeile(0),
            Some(erste),
            "die Zeile 0 ist eine Ueberschrift"
        );
        assert_eq!(
            modell.waehlbare_zeile(erste),
            Some(erste),
            "eine Funktionszeile bleibt, wo sie ist"
        );
        assert_eq!(
            modell.waehlbare_zeile(modell.zeilen()),
            Some(erste),
            "eine Stelle hinter dem Ende weicht auf die erste Funktion aus"
        );
        for stelle in 0..modell.zeilen() {
            let gewaehlt = modell.waehlbare_zeile(stelle).expect("es gibt Funktionen");
            assert!(
                modell.ueberschrift(gewaehlt).is_none(),
                "die Wiederherstellung landete auf der Ueberschrift {gewaehlt}"
            );
        }
    }

    /// Kein Zeilentext der Ansicht schreibt "Fn+" vor eine Kombination, und
    /// die Funktionstasten erscheinen als F1 bis F12 (C3). Das gilt auch fuer
    /// die Bereichsueberschriften.
    #[test]
    fn keine_zeile_traegt_fn_und_die_funktionstasten_heissen_f1_bis_f12() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let mut gross_f_gesehen = false;
        for stelle in 0..modell.zeilen() {
            if let Some(ueberschrift) = modell.ueberschrift(stelle) {
                assert!(!ueberschrift.contains("Fn+"), "{ueberschrift} traegt Fn+");
                assert!(!ueberschrift.contains("fn+"), "{ueberschrift} traegt fn+");
                continue;
            }
            let tasten = modell
                .tastentext(stelle)
                .expect("jede Funktionszeile hat Tasten");
            let funktion = modell.funktionstext(stelle).expect("und einen Text");
            for text in [&tasten, &funktion] {
                assert!(!text.contains("Fn+"), "{text} traegt Fn+");
                assert!(!text.contains("fn+"), "{text} traegt fn+");
            }
            // Kein kleines f vor einer Ziffer: f3 erscheint als F3.
            assert!(
                !tasten.split(['+', ',', ' ']).any(|teil| {
                    teil.starts_with('f')
                        && teil[1..].chars().all(|z| z.is_ascii_digit())
                        && !teil[1..].is_empty()
                }),
                "{tasten} schreibt eine Funktionstaste klein"
            );
            if tasten.contains('F') {
                gross_f_gesehen = true;
            }
        }
        assert!(gross_f_gesehen, "keine Zeile zeigt eine Funktionstaste");
    }

    /// Der F4-Eintrag traegt seine Taste und steht sichtbar im Bereich
    /// "Editor": die naechste Ueberschrift ueber ihm traegt diesen Namen.
    ///
    /// **Die Vorgaengerin dieser Pruefung hielt den Vorbehalt fest.** Bis zur
    /// Editor-Runde stand `bearbeiten` mit `reserviert_fuer = "editor"` und
    /// ohne Kombination in der Auslieferungsbelegung, und die Zeile las sich
    /// als "Bearbeiten (reserviert für den Editor)". Seit S5 und S6 traegt die
    /// Funktion `f4` und [`Kommando::Bearbeiten`]; der Vorbehalt ist
    /// eingeloest und nicht gebrochen, und die Zusage wandert entsprechend
    /// mit: der Eintrag ist erreichbar und steht weiter unter "Editor".
    #[test]
    fn der_f4_eintrag_traegt_seine_taste_und_steht_im_bereich_editor() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("bearbeiten");
        let text = modell
            .funktionstext(stelle)
            .expect("die Zeile hat einen Text");
        assert!(
            !text.contains("reserviert"),
            "{text} nennt einen Vorbehalt, den es nicht mehr gibt"
        );
        assert_eq!(modell.tastentext(stelle).as_deref(), Some("F4"));
        assert_eq!(
            Kommando::aus_kennung("bearbeiten"),
            Some(Kommando::Bearbeiten),
            "die Kennung aus der Belegungsdatei fuehrt nicht zum Kommando"
        );

        let ueberschrift = (0..stelle)
            .rev()
            .find_map(|davor| modell.ueberschrift(davor))
            .expect("ueber dem F4-Eintrag steht eine Ueberschrift");
        assert_eq!(ueberschrift, "Editor");
    }

    /// Die Zuweisung durch Druecken: eine freie Kombination landet in der
    /// Zeile ihrer Funktion.
    #[test]
    fn eine_freie_kombination_wird_zugewiesen() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
        assert_eq!(
            modell.zuweisen(stelle, druck),
            Zuweisung::Zugewiesen {
                funktion: "In das andere Fenster kopieren".to_owned(),
                kombination: "F9".to_owned(),
            }
        );
        assert!(modell.geaendert());
        let tasten = modell.tastentext(stelle).expect("die Zeile hat Tasten");
        assert!(tasten.contains("F9"), "{tasten} traegt F9 nicht");
    }

    /// Eine vergebene Kombination wird abgewiesen, und die Meldung nennt die
    /// andere Funktion — sie kommt woertlich aus dem Kern (C3).
    #[test]
    fn eine_vergebene_kombination_meldet_die_andere_funktion() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        // f8 gehoert dem endgueltigen Loeschen.
        let druck = Tastendruck::neu(code_von_pflicht("f8"), ModMaske::LEER);
        let Zuweisung::Abgelehnt(meldung) = modell.zuweisen(stelle, druck) else {
            panic!("f8 ist vergeben und darf nicht zugewiesen werden");
        };
        assert!(
            meldung.contains("endgueltig_loeschen"),
            "{meldung} nennt die andere Funktion nicht"
        );
        assert!(!modell.geaendert());
    }

    /// Eine Taste ohne Namen in der Schreibweise ergibt keine Zeile, sondern
    /// die Auskunft [`Zuweisung::OhneNamen`] (C3, S11b).
    #[test]
    fn eine_taste_ohne_namen_wird_gemeldet_statt_geschrieben() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        // Tastencode 10 traegt auf einer deutschen Tastatur die Taste links
        // neben der 1; die Schreibweise kennt keinen Namen dafuer.
        let druck = Tastendruck::neu(10, ModMaske::LEER);
        assert_eq!(modell.zuweisen(0, druck), Zuweisung::OhneNamen);
        assert!(!modell.geaendert());
    }

    /// Zwei Druecke, die sich allein durch gehaltenes fn unterscheiden, sind
    /// dieselbe Kombination: eine fn-Belegung ist nicht anlegbar (C3, S7).
    #[test]
    fn fn_unterscheidet_keine_kombination() {
        let mit_fn = Tastendruck::aus_ereignis(code_von_pflicht("f9"), None, roh::FUNKTION);
        let ohne = Tastendruck::aus_ereignis(code_von_pflicht("f9"), None, 0);
        assert_eq!(mit_fn, ohne);
        assert_eq!(
            Kombination::aus_tastendruck(mit_fn),
            Kombination::aus_tastendruck(ohne)
        );
    }

    /// Das Zuruecksetzen stellt die Auslieferungsbelegung wieder her (C3).
    #[test]
    fn das_zuruecksetzen_stellt_die_auslieferung_wieder_her() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
        assert!(matches!(
            modell.zuweisen(stelle, druck),
            Zuweisung::Zugewiesen { .. }
        ));
        modell.zuruecksetzen();
        assert!(modell.geaendert());
        assert_eq!(modell.in_belegung(), Belegung::auslieferung());
    }

    /// Die Kennung der Ansicht fuehrt seit diesem Schritt zu einem Kommando.
    #[test]
    fn belegung_ansehen_ist_ein_kommando() {
        assert_eq!(
            Kommando::aus_kennung("belegung_ansehen"),
            Some(Kommando::BelegungAnsehen)
        );
    }

    /// Die Zeile der genannten Funktion im gegliederten Modell ueber der
    /// Auslieferungsbelegung.
    ///
    /// Der Weg geht ueber den Namen, weil das Modell nach aussen nur die
    /// Anzeigeform kennt; die Namen der Auslieferungsbelegung sind eindeutig,
    /// das haelt `eine_zeile_je_funktion` fest.
    fn zeile_von(kennung: &str) -> usize {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let belegung = Belegung::auslieferung();
        let name = belegung
            .funktion(kennung)
            .unwrap_or_else(|| panic!("die Auslieferungsbelegung kennt {kennung} nicht"))
            .name()
            .to_owned();
        (0..modell.zeilen())
            .find(|&stelle| modell.name(stelle) == Some(name.as_str()))
            .unwrap_or_else(|| panic!("keine Zeile traegt die Funktion {kennung}"))
    }
}
