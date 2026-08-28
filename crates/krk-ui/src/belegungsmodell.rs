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
//!
//!                              Suchlage (daneben, seit der Runde 7)
//!                                    │  liest funktionstext und tastentext
//!                                    ▼
//!                              zielzeile / meldung ──> die Ansicht
//! ```
//!
//! **Die Suche steht neben der Arbeitskopie und nicht in ihr.** Sie aendert
//! keine Belegung; sie waehlt eine Zeile aus. Eine Aufnahme laesst den Suchtext
//! unberuehrt und umgekehrt (C1.12), und zwei Werte nebeneinander sagen das
//! deutlicher als ein Feld in [`Belegungsmodell`].
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
//!
//! **Seit S2 ist diese Beschriftung auf jeder Tastaturbelegung wahr, und bis
//! dahin war sie es nicht.** Ein einbuchstabiger Name benennt seit S2 das
//! **Zeichen** und nicht mehr die Stelle
//! ([`Tastenkennung`](krk_core::tasten::Tastenkennung)), und der
//! Ereignisabgriff schlaegt Buchstaben ueber dasselbe Zeichen nach. Wo die
//! Ansicht `Cmd+Y` schreibt, ist damit die Taste mit der **Aufschrift** Y
//! gemeint — auf einer deutschen Tastatur die Stelle `kVK_ANSI_Z`, auf einer
//! amerikanischen `kVK_ANSI_Y`. Vor S2 zeigte dieselbe Zeile `Cmd+Y` und
//! wirkte auf der deutschen Tastatur unter der Aufschrift Z. Dieses Modul hat
//! dafuer keinen Zweig: es schreibt den Namen der Taste auf, und der Kern hat
//! den Namen auf die Aufschrift gelegt. `die_beschriftung_nennt_die_taste_auf_
//! einer_deutschen_tastatur` haelt es fest.

use krk_core::tasten::{Belegung, Funktion, Kombination, Kommando, Tastendruck};
use krk_core::verzeichnis::filter::traegt_ein_dateiname;

/// Die Funktionsbereiche der Belegungsansicht, in der Reihenfolge der
/// Anzeige.
///
/// Ein Bereich buendelt die Funktionen, die derselben Gegend der Anwendung
/// gelten; die Ansicht setzt vor seine Funktionen eine Ueberschriftszeile.
/// [`Wirkungsbereich`](krk_core::tasten::Wirkungsbereich) traegt diese
/// Gliederung nicht: er beantwortet, welcher Bereich den Fokus haben muss,
/// und wirft dabei Fenster-, Fokus- und Anwendungsbefehle in einen Topf
/// (`Ueberall`), den kein Nutzer als Ordnung wiedererkennt.
///
/// # Die Reihenfolge beschreibt seit der Runde 7 eine Mac-Menueleiste
///
/// Drei Abnehmer lesen diese Gliederung, und der dritte ist seit der Runde 7
/// das Hauptmenue ([`crate::menuemodell`]). Fuer eine Menueleiste sind zwei
/// Stellen nicht waehlbar: **Anwendung** muss vorn stehen, weil macOS den Titel
/// des ersten Obermenues durch den Namen aus der `Info.plist` ersetzt, und
/// **Fenster** gehoert nach Mac-Gewohnheit ans Ende. Bis dahin lag `Anwendung`
/// an siebter und `Fenster` an sechster Stelle.
///
/// **Die zwei anderen Abnehmer folgen dem Menue, statt eine zweite Ordnung zu
/// bekommen** — die Belegungsansicht und die Markdown-Ausgabe der Runde 3
/// zeigen ihre Abschnitte seither in dieser Folge. Der Nutzer sieht damit in
/// allen drei Oberflaechen dieselbe Gliederung. Der Datensatz dazu ist
/// `circles/260813-0100-…/decisions/260813-0159_*_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`,
/// Moeglichkeit 1; eine zweite Ordnung im Menuemodell waere die Verdopplung,
/// die der Doc-Kommentar von [`nach_bereichen`] ausschliesst.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Funktionsbereich {
    /// Die Anwendung als ganze: Belegungsansicht und Beenden (C3).
    ///
    /// Steht seit der Runde 7 vorn, weil macOS den Titel des ersten
    /// Obermenues ohnehin durch den Namen aus der `Info.plist` ersetzt.
    Anwendung,
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
    /// Der eingebaute Editor: die beiden Einstiegswege, der Fokus, die beiden
    /// Ansichten, das Sichern, der Zeilensprung, Suchen und Ersetzen (C1 bis
    /// C6 der Editor-Runde).
    Editor,
    /// Die sechs Textbefehle, die das Menue "Bearbeiten" zustellt (C2, und
    /// Rueckgaengig und Wiederholen aus der Editor-Runde).
    ///
    /// **Der Anzeigename lautet seit der Runde 7 „Bearbeiten" und nicht mehr
    /// „Textbefehle".** Der Grund steht an [`Funktionsbereich::name`]; der
    /// Bezeichner der Variante bleibt, weil er die sechs Funktionen benennt und
    /// nicht das Menue.
    Textbefehle,
    /// Das Anwendungsfenster und seine Bereiche: wechseln, ein- und
    /// ausblenden, Breiten (C1, C7).
    ///
    /// Steht seit der Runde 7 hinten, weil das Fenstermenue auf dem Mac das
    /// letzte ist.
    Fenster,
}

impl Funktionsbereich {
    /// Alle Bereiche, in der Reihenfolge der Anzeige.
    ///
    /// Dieselbe Folge wie die Aufzaehlung darueber, und dort steht auch, warum
    /// sie seit der Runde 7 eine Mac-Menueleiste beschreibt.
    pub const ALLE: [Funktionsbereich; 9] = [
        Funktionsbereich::Anwendung,
        Funktionsbereich::Dateilisting,
        Funktionsbereich::Dateioperationen,
        Funktionsbereich::Tabs,
        Funktionsbereich::Vorschau,
        Funktionsbereich::LeisteUndFokus,
        Funktionsbereich::Editor,
        Funktionsbereich::Textbefehle,
        Funktionsbereich::Fenster,
    ];

    /// Die Ueberschrift des Bereichs in der Ansicht.
    ///
    /// **[`Funktionsbereich::Textbefehle`] heisst hier „Bearbeiten", und das
    /// ist keine Schoenheitsfrage.** Der Name ist der, den die Mac-Gewohnheit
    /// fuer dieses Menue verlangt, und er ist genauer als der alte: die sechs
    /// Funktionen tragen saemtlich `gehalten_von = "menue"` und sind genau die
    /// Eintraege jenes Menues. Dieselbe Gliederung tragen die Belegungsansicht
    /// und die Markdown-Ausgabe, also heisst der Bereich dort ebenso.
    ///
    /// **Ob macOS seine eigenen Textzusaetze an den Menue*titel* haengt, ist
    /// ungemessen**, und der Name traegt diese Zusage deshalb nicht.
    /// `appkit::menue::systemzusaetze_unterdruecken` setzt nicht am Titel an,
    /// sondern traegt drei Namen in `NSUserDefaults` ein und wirkt damit
    /// unabhaengig davon, wie das Obermenue heisst; ihr eigener Doc-Kommentar
    /// nennt die Messung dazu. Hier stand bis zur Runde 7 das Gegenteil, und es
    /// berief sich als Beleg auf genau jene Funktion
    /// (`issues/260813-0540_*_ein-doc-kommentar-begruendet-bearbeiten-mit-einem-mechanismus-den-es-nicht-gibt.md`).
    /// Die offene Frage, ob umbenannt werden **darf**, ist eine andere und steht
    /// in `decisions/260813-0159_*_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`.
    pub const fn name(self) -> &'static str {
        match self {
            Funktionsbereich::Anwendung => "Anwendung",
            Funktionsbereich::Dateilisting => "Dateilisting",
            Funktionsbereich::Dateioperationen => "Dateioperationen",
            Funktionsbereich::Tabs => "Tabs",
            Funktionsbereich::Vorschau => "Vorschau",
            Funktionsbereich::LeisteUndFokus => "Leiste und Fokus",
            Funktionsbereich::Editor => "Editor",
            Funktionsbereich::Textbefehle => "Bearbeiten",
            Funktionsbereich::Fenster => "Fenster",
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
        // Der Ordnersprung aus C2 der Runde 6 steht neben dem Aufstieg und dem
        // Sprung aus der Zwischenablage: alle drei setzen den Ordner, den eine
        // Dateiliste zeigt. Dass seine Quelle aus einem anderen Bereich kommt,
        // macht keinen zweiten Ort auf — diese Gliederung fragt nach der
        // Gegend der Anwendung, und die ist die Dateiliste, die sich bewegt.
        | Kommando::OrdnerDerDatei
        // Das Angleichen aus C1 der Runde 13 steht aus demselben Grund hier
        // wie der Aufstieg und der Sprung aus der Zwischenablage: alle setzen
        // den Ordner, den eine Dateiliste zeigt. Dass es die **andere** Liste
        // ist, die sich bewegt, macht keinen zweiten Ort auf — diese
        // Gliederung fragt nach der Gegend der Anwendung, und die Dateiliste
        // ist eine, gleich wie viele es davon gibt.
        | Kommando::OrdnerAngleichen
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
        // Die drei Spaltenschalter stehen neben dem Ein- und Ausblenden der
        // versteckten Eintraege: beides bestimmt, was die Liste zeigt. Dass
        // sie beide Listen zugleich treffen, macht keinen zweiten Ort auf —
        // diese Gliederung fragt nach der Gegend der Anwendung, und die
        // Dateiliste ist eine, gleich wie viele es davon gibt.
        | Kommando::SpalteGroesseUmschalten
        | Kommando::SpalteDatumUmschalten
        | Kommando::SpalteTypUmschalten
        // Der Schalter "Deep" aus C5 der Filter-Runde steht mit hier: er
        // bestimmt, was die Liste zeigt, wie das Ein- und Ausblenden der
        // versteckten Eintraege und die drei Spaltenschalter darueber. Damit
        // steht er im Hauptmenue dort, wo der Nutzer ihn sucht, naemlich bei
        // den drei Schaltern, mit denen er in der Bereichsleiste eine Reihe
        // bildet.
        | Kommando::TiefeSucheUmschalten
        // Der Schalter "Content" aus C2 der Inhaltsfilter-Runde steht neben
        // "Deep": er bestimmt, was die Liste zeigt, und er bildet mit ihm in
        // der Bereichsleiste eine Reihe. Im Hauptmenue steht er damit dort, wo
        // der Nutzer ihn sucht, naemlich unmittelbar bei der tiefen Suche.
        | Kommando::InhaltssucheUmschalten
        | Kommando::ZwischenablageSpringen => Funktionsbereich::Dateilisting,
        // Die Dateioperationen aus C4 und der Terminal-Befehl aus C11, der
        // wie sie auf dem angezeigten Ordner arbeitet.
        //
        // Die drei Befehle der Runde 4 stehen aus derselben Regel hier: diese
        // Gliederung fragt nach der Gegend der Anwendung und nicht nach dem
        // Mechanismus, und ein Befehl, der den Ordner oder einen Eintrag an
        // etwas ausserhalb der Liste uebergibt, steht dort, wo der
        // Terminal-Befehl steht. `Dateilisting` traegt Bewegung, Markierung
        // und Sortierung; keiner der drei tut davon etwas.
        Kommando::Kopieren
        | Kommando::Verschieben
        | Kommando::InPapierkorb
        | Kommando::Abbrechen
        | Kommando::OrdnerAnlegen
        | Kommando::DateiAnlegen
        | Kommando::UmbenennenStapel
        | Kommando::Umbenennen
        | Kommando::TerminalOeffnen
        | Kommando::OrdnerpfadKopieren
        | Kommando::EintragspfadKopieren
        | Kommando::MitStandardprogrammOeffnen
        // Das Teilen aus C1 der Runde 6 faellt unter denselben Satz: es
        // uebergibt einen Eintrag an etwas ausserhalb der Liste, wie der
        // Terminal-Befehl den Ordner und der Oeffner den Eintrag. Dass seine
        // Quelle je nach Fokus aus der Vorschau oder dem Editor kommt, macht
        // keinen zweiten Ort auf — diese Gliederung fragt nach der Gegend der
        // Anwendung, und wer teilen will, sucht bei den Befehlen, die etwas
        // aus KRK herausgeben.
        | Kommando::Teilen => Funktionsbereich::Dateioperationen,
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
        // Vorschau kommt, findet unter "Vorschau" alle Befehle, die sie
        // angehen.
        //
        // Die drei Zoombefehle des PDF-Betrachters aus der Runde 20 stehen aus
        // demselben Satz hier: der Betrachter ist eine Ansicht des
        // Vorschaufensters, und die drei tragen als einzige
        // `Wirkungsbereich::Vorschau`.
        Kommando::VorschauUmschalten
        | Kommando::ZwischenablageAnsehen
        | Kommando::FokusVorschau
        | Kommando::VorschauVergroessern
        | Kommando::VorschauVerkleinern
        | Kommando::VorschauAusgangsgroesse => Funktionsbereich::Vorschau,
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
        | Kommando::ErstesFensterUmschalten
        | Kommando::ZweitesFensterUmschalten
        | Kommando::FensterEinblenden
        | Kommando::FensterSchliessen
        | Kommando::BereichVerbreitern
        | Kommando::BereichVerschmaelern => Funktionsbereich::Fenster,
        // Die weitere Instanz aus C3 der Runde 7 steht bei der Anwendung und
        // nicht beim Fenster: sie startet einen zweiten Prozess mit eigener
        // Menueleiste, und `Fenster` fuehrt die Bereiche **dieses** Fensters.
        // Diese Runde fuehrt keine zweiten Fenster ein.
        //
        // Der Notizzettel steht mit hier und bekommt **keinen** eigenen
        // Funktionsbereich: er waere ein Obermenue mit einem einzigen Eintrag,
        // und diese Gliederung fragt nach der Gegend der Anwendung. Der Zettel
        // haengt als Blatt am Hauptfenster und gehoert damit der Anwendung als
        // ganze, so wie die Belegungsansicht daneben.
        Kommando::BelegungAnsehen
        | Kommando::Beenden
        | Kommando::WeitereInstanz
        | Kommando::Notizzettel => Funktionsbereich::Anwendung,
        // Der eingebaute Editor, und `bearbeiten` steht mit darin.
        //
        // Es ist die einzige Stelle, an der diese Gliederung und
        // [`Kommando::wirkungsbereich`] auseinandergehen, und der Grund ist
        // derselbe wie beim Ein- und Ausblenden der Vorschau weiter oben: die
        // Gliederung fragt nach der **Gegend der Anwendung**, der
        // Wirkungsbereich nach dem Fokus, den ein Befehl braucht. F4 braucht
        // das Dateifenster, aber wer die Zeile sucht, sucht sie unter "Editor"
        // und nicht unter "Dateioperationen". Derselbe Satz ordnet den Rundweg
        // hierher und nicht zu "Vorschau": beide fuehren in den Editor, und der
        // Nutzer findet unter "Editor" alle Befehle, die ihn angehen. Fuer den
        // Rundweg traegt der Satz seit dem 260823 mehr als vorher, denn er
        // wirkt jetzt in drei Bereichen und keiner davon ist die Vorschau
        // allein.
        //
        // Das Ein- und Ausblenden des Editors steht hier und nicht unter
        // "Fenster", wo sein Gegenstueck fuer die beiden Dateifenster steht.
        // Derselbe Satz wie beim Ein- und Ausblenden der Vorschau weiter oben:
        // die Gliederung fragt nach der Gegend der Anwendung, und wer den
        // Editor sucht, sucht unter "Editor". Dass die beiden Dateifenster
        // unter "Fenster" stehen, ist kein Widerspruch, sondern dieselbe
        // Regel: ein Dateifenster ist keine eigene Gegend der Belegung, es
        // gibt keinen Abschnitt dafuer.
        Kommando::Bearbeiten
        | Kommando::EditorRundweg
        | Kommando::FokusEditor
        | Kommando::EditorSchliessen
        | Kommando::EditorUmschalten
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
        Some(tastenliste(self.funktion(stelle)?))
    }

    /// Ob die Zeile den bereits kleingeschriebenen Suchtext traegt (C1.3 bis
    /// C1.6).
    ///
    /// Gesucht wird ueber die zwei Spalten, die auf dem Schirm stehen, und
    /// ueber keine dritte Groesse. Eine Bereichsueberschrift traegt nie einen
    /// Treffer, und das ist hier kein Zweig: [`Belegungsmodell::funktionstext`]
    /// und [`Belegungsmodell::tastentext`] antworten fuer sie `None`.
    ///
    /// `gesucht` kommt kleingeschrieben herein, damit die Umschreibung einmal
    /// je Suche laeuft und nicht einmal je Zeile.
    fn zeile_traegt(&self, stelle: usize, gesucht: &str) -> bool {
        [self.funktionstext(stelle), self.tastentext(stelle)]
            .into_iter()
            .flatten()
            .any(|text| text.to_lowercase().contains(gesucht))
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

/// Der Stand der Suche in der Belegungsansicht (C1 der Runde 7).
///
/// **Die Rechnung steht hier und nicht in der Ansicht**, weil sie ohne AppKit
/// auskommt und damit ohne Fenster pruefbar ist. Die Ansicht haelt einen Wert
/// dieser Art, gibt ihm die drei Ereignisse weiter, die der Faenger des
/// Ereignisabgriffs ihr zustellt, und liest danach [`Suchlage::zielzeile`] und
/// [`Suchlage::meldung`] ab.
///
/// ```text
///   Zeichen ──> zeichen_anhaengen ─┐
///   Ruecktaste ─> letztes_zeichen_weg ─┼─> Suchtext ──> Trefferzeilen
///   Eingabetaste ─> naechster_treffer ─┘        │            │
///                                          meldung()    zielzeile()
/// ```
///
/// # Gesucht wird ueber den Text, den die Ansicht zeigt
///
/// Ueber [`Belegungsmodell::funktionstext`] und
/// [`Belegungsmodell::tastentext`], also genau ueber die zwei Spalten auf dem
/// Schirm (C1.3). Die Kennung wird **nicht** durchsucht: sie steht nicht da,
/// und ein Treffer, den der Nutzer nicht sehen kann, ist keiner.
///
/// Verglichen wird als Teilzeichenfolge (C1.4) und ohne Ruecksicht auf Gross-
/// und Kleinschreibung (C1.5), wie im Filter der Dateiliste. Eine
/// Bereichsueberschrift kann kein Treffer sein, und dafuer braucht es keinen
/// Zweig: [`Belegungsmodell::funktionstext`] und
/// [`Belegungsmodell::tastentext`] antworten fuer eine Ueberschriftszeile
/// `None`, und eine Zeile ohne Text traegt keinen Treffer (C1.6).
///
/// # Der Suchtext lebt so lange wie die Ansicht
///
/// Keine Pause setzt ihn zurueck, und es gibt keinen Zeitgeber (C1.12). Der
/// Filter der Dateiliste haelt es seit der Runde 10 ebenso; die Sekundenregel
/// der abgeloesten Sprungmarke war der letzte Zeitgeber im Weg eines getippten
/// Zeichens und ist mit ihr gefallen.
///
/// # Bei leerem Suchtext geschieht nichts
///
/// Eingabetaste und Ruecktaste bleiben wirkungslos (C1.8, C1.17). Das ist eine
/// Regel und nicht zwei: ohne Suchtext gibt es kein naechstes Vorkommen und
/// kein letztes Zeichen. Beide Wege melden es ueber ihren Rueckgabewert, und
/// die Ansicht laesst dann ihre Meldungszeile stehen, statt eine
/// Zuweisungsmeldung mit einer leeren Suchmeldung zu ueberschreiben.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Suchlage {
    /// Was bisher getippt ist.
    suchtext: String,
    /// Die Zeilen der Ansicht, die den Suchtext tragen, aufsteigend. Die
    /// Sortierung ist die Voraussetzung von
    /// [`krk_core::text::suche::erster_ab_stelle`] und entsteht von selbst,
    /// weil die Rechnung die Zeilen von oben nach unten durchgeht.
    treffer: Vec<usize>,
    /// Der angesteuerte Treffer, als Stelle in [`Suchlage::treffer`].
    stelle: Option<usize>,
}

impl Suchlage {
    /// Eine Suche ohne Suchtext und ohne Treffer.
    pub fn neu() -> Self {
        Self::default()
    }

    /// Die Zeile, auf die die Auswahl gehoert, oder `None` ohne Treffer.
    ///
    /// Bei null Treffern bleibt die Auswahl stehen (C1.9); das ist die Folge
    /// davon, dass hier `None` steht, und keine eigene Regel in der Ansicht.
    pub fn zielzeile(&self) -> Option<usize> {
        self.treffer.get(self.stelle?).copied()
    }

    /// Haengt ein getipptes Zeichen an und sucht erneut (C1.1).
    ///
    /// Liefert, ob das Zeichen aufgenommen wurde. **Die Aufnahmeregel ist
    /// `krk_core::verzeichnis::filter::traegt_ein_dateiname`**, dieselbe, die
    /// der Filter der Dateiliste benutzt; eine zweite Zeichenregel
    /// entsteht nicht (C1.2). Sie weist Steuerzeichen ab, den privaten
    /// Bereich U+F700 bis U+F8FF, in dem AppKit die Funktions- und Pfeiltasten
    /// meldet, und den Schraegstrich.
    ///
    /// **Der Schraegstrich kostet diese Suche nichts.** Kein Tastenname der
    /// Tabelle in `krk_core::tasten::parser::TASTEN` traegt ihn, also kann
    /// keine Anzeigeform einer Kombination ihn zeigen, und keine Beschriftung
    /// in `resources/default-keymap.toml` fuehrt ihn. Gesucht wird ueber genau
    /// diese beiden Spalten, siehe `Belegungsmodell::zeile_traegt`.
    #[must_use]
    pub fn zeichen_anhaengen(&mut self, zeichen: char, modell: &Belegungsmodell) -> bool {
        if !traegt_ein_dateiname(zeichen) {
            return false;
        }
        self.suchtext.push(zeichen);
        self.nachrechnen(modell);
        true
    }

    /// Nimmt das letzte Zeichen weg und sucht erneut (C1.8).
    ///
    /// Liefert, ob etwas wegzunehmen war. Bei leerem Suchtext geschieht
    /// nichts.
    #[must_use]
    pub fn letztes_zeichen_weg(&mut self, modell: &Belegungsmodell) -> bool {
        if self.suchtext.pop().is_none() {
            return false;
        }
        self.nachrechnen(modell);
        true
    }

    /// Geht auf das naechste Vorkommen; hinter dem letzten beim ersten weiter
    /// (C1.7).
    ///
    /// Liefert, ob es ein Vorkommen gibt. Ohne Suchtext und ohne Treffer
    /// geschieht nichts (C1.17).
    #[must_use]
    pub fn naechster_treffer(&mut self) -> bool {
        let Some(zeile) = self.zielzeile() else {
            return false;
        };
        self.stelle = krk_core::text::suche::naechster_stelle(&self.treffer, zeile);
        true
    }

    /// Der Satz fuer die Meldungszeile: Suchtext, Trefferzahl und Stelle
    /// darin (C1.9, C1.10).
    pub fn meldung(&self) -> String {
        if self.suchtext.is_empty() {
            return "Der Suchtext ist leer; jedes getippte Zeichen sucht.".to_owned();
        }
        match self.stelle {
            Some(stelle) => format!(
                "Suche »{}«: Treffer {} von {}.",
                self.suchtext,
                stelle + 1,
                self.treffer.len()
            ),
            None => format!("Suche »{}«: kein Treffer.", self.suchtext),
        }
    }

    /// Sucht den Suchtext neu und behaelt dabei die Stelle, so gut es geht.
    ///
    /// **Auch von aussen zu rufen, und zwar nach jeder Aenderung am Modell.**
    /// Gesucht wird ueber den Text, den die Ansicht zeigt, und eine Zuweisung
    /// oder ein Zuruecksetzen aendert die Spalte „Belegung". Eine Trefferliste,
    /// die danach stehen bliebe, zeigte auf Zeilen, die den Suchtext nicht mehr
    /// tragen. Die Meldungszeile bleibt dabei unberuehrt: nach einer Zuweisung
    /// steht dort deren Bestaetigung, bis das naechste Suchzeichen kommt
    /// (C1.10).
    ///
    /// **Gesucht wird ab der bisherigen Zielzeile und nicht wieder von oben.**
    /// Wer beim Tippen einen Suchtext verlaengert, will nicht bei jedem
    /// Buchstaben an den Listenanfang zurueckgeworfen werden;
    /// [`krk_core::text::suche::erster_ab_stelle`] zaehlt die Zeile unter der
    /// Auswahl deshalb mit und laeuft hinter der letzten um. Ohne bisherige
    /// Zielzeile faengt die Suche bei Zeile 0 an, und das ist der erste Treffer
    /// (C1.1).
    pub fn nachrechnen(&mut self, modell: &Belegungsmodell) {
        let ab = self.zielzeile().unwrap_or(0);
        self.treffer = if self.suchtext.is_empty() {
            Vec::new()
        } else {
            let gesucht = self.suchtext.to_lowercase();
            (0..modell.zeilen())
                .filter(|&stelle| modell.zeile_traegt(stelle, &gesucht))
                .collect()
        };
        self.stelle = krk_core::text::suche::erster_ab_stelle(&self.treffer, ab);
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
/// Auslieferungsbelegung ist jeder Bereich besetzt. Die Gruppierung selbst
/// steht seit der Runde 3 in [`nach_bereichen`] darunter, weil die
/// Tastenbelegung als Markdown dieselbe braucht; hier bleibt allein das
/// Umschreiben in Zeilen. Der laute Abbruch bei einer Funktion ohne Bereich ist
/// mitgewandert und steht dort.
fn gliederung(belegung: &Belegung) -> Vec<Zeile> {
    let gruppen = nach_bereichen(belegung);
    let mut zeilen = Vec::with_capacity(belegung.funktionen().len() + gruppen.len());
    for (gruppe, stellen) in gruppen {
        zeilen.push(Zeile::Ueberschrift(gruppe));
        zeilen.extend(stellen.into_iter().map(Zeile::Funktion));
    }
    zeilen
}

/// Die Funktionen einer Belegung nach Funktionsbereich, in der Reihenfolge von
/// [`Funktionsbereich::ALLE`], je Bereich die Stellen in
/// [`Belegung::funktionen`] in der Reihenfolge der Datei.
///
/// **Die eine Gliederung, zwei Abnehmer.** Die Belegungsansicht baut daraus
/// ihre Zeilen ([`gliederung`] darueber), die Tastenbelegung als Markdown ihre
/// Abschnitte ([`crate::belegungsausgabe::markdown`]). Eine zweite Gruppierung
/// entsteht nicht; die Directive der Runde 3 schliesst eine zweite Aufbereitung
/// ausdruecklich aus.
///
/// Ein Bereich ohne Funktion erscheint nicht in der Liste; in der
/// Auslieferungsbelegung ist jeder Bereich besetzt.
///
/// Eine Funktion ohne Bereich waere ein Programmierfehler — eine neue Funktion
/// ist erst vollstaendig, wenn [`bereich`] sie einordnet, und die Pruefung
/// `jede_kennung_hat_einen_funktionsbereich` haelt das fest. Sie still
/// auszulassen hiesse, eine Funktion aus der Ansicht **und** aus der Datei
/// verschwinden zu lassen, die beide vollstaendig sein sollen; deshalb bricht
/// der Bau hier laut ab.
pub fn nach_bereichen(belegung: &Belegung) -> Vec<(Funktionsbereich, Vec<usize>)> {
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

    let mut gruppen = Vec::with_capacity(Funktionsbereich::ALLE.len());
    for gruppe in Funktionsbereich::ALLE {
        let stellen: Vec<usize> = bereiche
            .iter()
            .enumerate()
            .filter(|(_, eingeordnet)| **eingeordnet == gruppe)
            .map(|(stelle, _)| stelle)
            .collect();
        if !stellen.is_empty() {
            gruppen.push((gruppe, stellen));
        }
    }
    gruppen
}

/// Alle Kombinationen einer Funktion in der Anzeigeform, durch Komma und
/// Leerzeichen getrennt.
///
/// **Die eine Schreibweise, zwei Abnehmer**, wie [`nach_bereichen`] darueber:
/// die Spalte "Belegung" der Bildschirmansicht ([`Belegungsmodell::tastentext`])
/// und die Spalte "Kombinationen" der Markdown-Datei. Die Ein-Zeilen-Regel aus
/// C3 der Runde 1 gilt damit in der Datei wie am Schirm, ohne dass sie zweimal
/// gebaut waere.
pub fn tastenliste(funktion: &Funktion) -> String {
    funktion
        .tasten()
        .iter()
        .map(anzeige)
        .collect::<Vec<String>>()
        .join(", ")
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
    /// je Funktion, und der Papierkorb ist seit dem Wegfall des endgueltigen
    /// Loeschens die eine Zeile des Loeschwegs und nicht mehr eine von zweien.
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

    /// Die zwei Stellen der Gliederung, die eine Mac-Menueleiste nicht frei
    /// waehlen darf.
    ///
    /// **Ohne diese Probe faellt eine falsche Reihenfolge niemandem auf.** Der
    /// Uebersetzer haelt sie nicht: [`Funktionsbereich::ALLE`] ist eine Liste
    /// und keine Fallunterscheidung, und eine umgestellte Zeile darin
    /// uebersetzt anstandslos. Was daran haengt, steht am Doc-Kommentar der
    /// Aufzaehlung: macOS ersetzt den Titel des **ersten** Obermenues durch den
    /// Namen aus der `Info.plist`, also muss dort der Anwendungsbereich stehen,
    /// und das Fenstermenue ist auf dem Mac das letzte.
    #[test]
    fn die_gliederung_beginnt_mit_der_anwendung_und_endet_mit_dem_fenster() {
        assert_eq!(
            Funktionsbereich::ALLE.first(),
            Some(&Funktionsbereich::Anwendung),
            "das erste Obermenue traegt nicht den Anwendungsbereich; macOS \
             ueberschreibt dessen Titel mit dem Namen aus der Info.plist"
        );
        assert_eq!(
            Funktionsbereich::ALLE.last(),
            Some(&Funktionsbereich::Fenster),
            "das Fenstermenue steht auf dem Mac am Ende der Leiste"
        );
    }

    /// Das Obermenue der sechs zugestellten Textbefehle heisst „Bearbeiten".
    ///
    /// **Der Name traegt eine Zusage und nicht bloss eine Aufschrift.** macOS
    /// haengt seine eigenen Textzusaetze — „Emoji & Symbols", „Start
    /// Dictation…", das Untermenue „AutoFill" — an ein Menue dieses Namens, und
    /// `appkit::menue::systemzusaetze_unterdruecken` setzt genau dort an.
    /// Heisst das Obermenue anders, ruht die Zusage aus C2.13 auf einer
    /// ungeprueften Annahme darueber, woran macOS seine Zusaetze festmacht.
    #[test]
    fn der_bereich_der_textbefehle_heisst_bearbeiten() {
        assert_eq!(
            Funktionsbereich::Textbefehle.name(),
            "Bearbeiten",
            "das Obermenue der Textbefehle heisst nicht mehr wie das, an das \
             macOS seine Zusaetze haengt"
        );
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
    ///
    /// Genommen wird `f6`, weil die Probe eine Kombination braucht, die einer
    /// **anderen** Funktion als der beschriebenen Zeile gehoert. Bis zum
    /// 260817 stand hier `f8` mit `endgueltig_loeschen` als der anderen
    /// Funktion; mit dem Wegfall des endgueltigen Loeschens waere das keine
    /// vergebene Kombination mehr gewesen.
    #[test]
    fn eine_vergebene_kombination_meldet_die_andere_funktion() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        // f6 gehoert dem Verschieben.
        let druck = Tastendruck::neu(code_von_pflicht("f6"), ModMaske::LEER);
        let Zuweisung::Abgelehnt(meldung) = modell.zuweisen(stelle, druck) else {
            panic!("f6 ist vergeben und darf nicht zugewiesen werden");
        };
        assert!(
            meldung.contains("verschieben"),
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

    /// Die Ansicht fuehrt genau die Befehle des Editors unter der Ueberschrift
    /// "Editor", jeden mit mindestens einer Kombination.
    ///
    /// Das achte Abnahmekriterium von C7 sagt zu, dass jeder neue Befehl der
    /// Editor-Runde in der Belegungsansicht **aufgefuehrt** ist. Die
    /// Vollstaendigkeit der Zuordnung prueft
    /// [`jede_kennung_hat_einen_funktionsbereich`](jede_kennung_hat_einen_funktionsbereich)
    /// bereits; hier steht die andere Haelfte, naemlich dass die benannten im
    /// **richtigen** Abschnitt landen und dass keine weitere Funktion sich
    /// dazwischenschiebt.
    ///
    /// **Der Name nennt die Zahl der Befehle nicht**, und die Liste ist die
    /// Zusage. Bis zum 260812 hiess die Probe
    /// `der_bereich_editor_fuehrt_die_zwoelf_befehle_der_runde`; mit
    /// `editor_umschalten` aus der Bereichsleisten-Runde sind es dreizehn, und
    /// eine Zahl im Namen bindet die Probe an die Groesse der Liste statt an
    /// ihre Zusage. Denselben Grund nennt
    /// `die_ab_werk_freien_kombinationen_kommen_nicht_vor` in
    /// `crates/krk-core/tests/belegung.rs`.
    ///
    /// Die Kennungen stehen ausgeschrieben und nicht aus
    /// [`bereich_des_kommandos`] abgeleitet: eine Ableitung pruefte die
    /// Zuordnung gegen sich selbst und liefe mit jedem Umzug stillschweigend
    /// mit.
    #[test]
    fn der_bereich_editor_fuehrt_genau_die_befehle_des_editors() {
        const EDITORBEFEHLE: [&str; 13] = [
            "bearbeiten",
            "editor_rundweg",
            "fokus_editor",
            "editor_schliessen",
            "editor_umschalten",
            "editor_ansicht_umschalten",
            "editor_sichern",
            "editor_zeile_springen",
            "editor_suchen",
            "editor_weitersuchen",
            "editor_rueckwaerts_suchen",
            "editor_ersetzen",
            "editor_alle_ersetzen",
        ];

        let belegung = Belegung::auslieferung();
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let gefuehrt = funktionen_unter(&modell, "Editor");

        for kennung in EDITORBEFEHLE {
            let funktion = belegung
                .funktion(kennung)
                .unwrap_or_else(|| panic!("die Auslieferungsbelegung kennt {kennung} nicht"));
            assert!(
                gefuehrt.iter().any(|name| name == funktion.name()),
                "{kennung} steht nicht unter der Ueberschrift Editor, sondern in {:?}",
                bereich(kennung)
            );
            assert!(
                !funktion.tasten().is_empty(),
                "{kennung} steht in der Ansicht, traegt aber keine Kombination"
            );
        }
        assert_eq!(
            gefuehrt.len(),
            EDITORBEFEHLE.len(),
            "unter Editor stehen andere Funktionen als die benannten: {gefuehrt:?}"
        );
    }

    /// Die beiden Umschalter der Bereichsleisten-Runde stehen in ihrem
    /// Funktionsbereich, und es sind zwei verschiedene.
    ///
    /// Das linke Dateifenster steht bei den Fensterbefehlen, neben dem
    /// rechten; der Editor bei seinen eigenen, neben dem Schliessen, von dem
    /// er sich unterscheidet. Beide Zuordnungen sind eine Wahl und keine
    /// Ableitung: `editor_umschalten` traegt
    /// [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
    /// koennte damit ebensogut unter "Fenster" stehen. Die Gliederung fragt
    /// nach der Gegend der Anwendung, nicht nach dem Fokus.
    #[test]
    fn die_beiden_neuen_umschalter_stehen_in_ihrem_bereich() {
        assert_eq!(
            bereich("erstes_fenster_umschalten"),
            Some(Funktionsbereich::Fenster),
            "das linke Dateifenster steht nicht bei den Fensterbefehlen"
        );
        assert_eq!(
            bereich("editor_umschalten"),
            Some(Funktionsbereich::Editor),
            "der Editorschalter steht nicht beim Editor"
        );
    }

    /// Die beiden anderen neuen Funktionen stehen unter
    /// [`Funktionsbereich::Textbefehle`] und nicht beim Editor, und der
    /// Abschnitt fuehrt damit sechs.
    ///
    /// Rueckgaengig und Wiederholen kommen aus dieser Runde, gehoeren aber
    /// nicht dem Editor: das Menue "Bearbeiten" stellt sie zu, und im Textfeld
    /// wirken sie genauso. Die Gliederung fragt nach der Gegend der Anwendung,
    /// und die ist hier dieselbe wie bei Ausschneiden, Kopieren und Einfuegen.
    ///
    /// Die Ueberschrift steht hier als [`Funktionsbereich::name`] und nicht als
    /// Zeichenkette: seit der Runde 7 lautet sie „Bearbeiten", und der Name
    /// dieser Probe benennt die Variante und nicht die Aufschrift.
    #[test]
    fn die_beiden_neuen_textbefehle_stehen_unter_textbefehle() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let gefuehrt = funktionen_unter(&modell, Funktionsbereich::Textbefehle.name());
        assert_eq!(
            gefuehrt.len(),
            6,
            "der Abschnitt Textbefehle fuehrt nicht sechs Funktionen: {gefuehrt:?}"
        );
        for kennung in ["text_rueckgaengig", "text_wiederholen"] {
            assert_eq!(
                bereich(kennung),
                Some(Funktionsbereich::Textbefehle),
                "{kennung} steht im falschen Abschnitt"
            );
        }
    }

    /// Der Ordnersprung aus C2 der Runde 6 steht unter "Dateilisting".
    ///
    /// Die Zuordnung ist eine Wahl und keine Ableitung: der Befehl traegt
    /// [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
    /// nimmt seine Quelle aus der Vorschau oder dem Editor, koennte nach dem
    /// Fokus also ebensogut dort stehen. Die Gliederung fragt nach der Gegend
    /// der Anwendung, und bewegt wird eine Dateiliste.
    #[test]
    fn der_ordnersprung_steht_unter_dateilisting() {
        assert_eq!(
            bereich("ordner_der_datei"),
            Some(Funktionsbereich::Dateilisting),
            "der Ordnersprung steht nicht beim Dateilisting"
        );
        let belegung = Belegung::auslieferung();
        let funktion = belegung
            .funktion("ordner_der_datei")
            .expect("die Auslieferungsbelegung kennt ordner_der_datei");
        assert!(
            !funktion.tasten().is_empty(),
            "der Ordnersprung traegt ab Werk keine Kombination und erschiene damit in keiner Zeile"
        );
    }

    /// Das Angleichen aus C1 der Runde 13 steht unter "Dateilisting".
    ///
    /// Anders als beim Ordnersprung darueber ist die Zuordnung hier keine
    /// Wahl gegen den Fokus: der Befehl traegt
    /// [`Wirkungsbereich::Dateifenster`](krk_core::tasten::Wirkungsbereich),
    /// weil seine Quelle der angezeigte Ordner eines Dateifensters ist. Die
    /// Probe haelt trotzdem beides fest, denn die Gliederung ist eine eigene
    /// Aussage und keine Ableitung aus dem Wirkungsbereich.
    ///
    /// Die zweite Behauptung ist die wichtigere: ohne Kombination ab Werk
    /// erschiene die Funktion in keiner Zeile der Belegungsansicht, und der
    /// Befehl waere gebaut und unerreichbar.
    #[test]
    fn das_ordnerangleichen_steht_unter_dateilisting() {
        assert_eq!(
            bereich("ordner_angleichen"),
            Some(Funktionsbereich::Dateilisting),
            "das Ordnerangleichen steht nicht beim Dateilisting"
        );
        let belegung = Belegung::auslieferung();
        let funktion = belegung
            .funktion("ordner_angleichen")
            .expect("die Auslieferungsbelegung kennt ordner_angleichen");
        assert!(
            !funktion.tasten().is_empty(),
            "das Ordnerangleichen traegt ab Werk keine Kombination und erschiene damit in keiner Zeile"
        );
    }

    /// Jede der dreizehn Kennungen, die S6 der Belegungsdatei hinzugefuegt
    /// hat, ist ueber die Ansicht umbelegbar.
    ///
    /// Die zweite Haelfte des achten Abnahmekriteriums von C7: aufgefuehrt zu
    /// sein genuegt nicht, die Zeile muss auch eine Zuweisung annehmen. Der
    /// Weg geht bewusst ueber [`Belegungsmodell::zuweisen`] und nicht direkt
    /// ueber `Belegung::zuweisen`, weil allein der erste die Zeilennummer der
    /// Ansicht auf die Funktion abbildet.
    ///
    /// Die beiden Textbefehle stehen mit in der Liste, obwohl das Menue sie
    /// zustellt und nicht der Ereignisabgriff: die Zusage von C3 gilt jeder
    /// Kombination, die in KRK etwas ausloest, gleich wer sie zustellt.
    ///
    /// **`editor_rundweg` steht seit dem 260823-0942 unter diesem Namen**; die
    /// Editor-Runde hat den Eintrag als `editor_aus_vorschau` angelegt. Die
    /// Kennung ist dieselbe Zeile der Belegungsdatei, der Name kommt aus dem
    /// Nutzerentscheid, der `cmd+e` zum Rundweg gemacht hat.
    #[test]
    fn jede_neue_kennung_der_editor_runde_ist_umbelegbar() {
        const NEUE_KENNUNGEN: [&str; 13] = [
            "editor_rundweg",
            "fokus_editor",
            "editor_schliessen",
            "editor_ansicht_umschalten",
            "editor_sichern",
            "editor_zeile_springen",
            "editor_suchen",
            "editor_weitersuchen",
            "editor_rueckwaerts_suchen",
            "editor_ersetzen",
            "editor_alle_ersetzen",
            "text_rueckgaengig",
            "text_wiederholen",
        ];

        for kennung in NEUE_KENNUNGEN {
            let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
            let stelle = zeile_von(kennung);
            let name = modell
                .name(stelle)
                .expect("die Zeile traegt eine Funktion")
                .to_owned();
            // F9 gehoert ab Werk keiner Funktion, weder einer vom Abgriff noch
            // einer vom Menue zugestellten.
            let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
            assert_eq!(
                modell.zuweisen(stelle, druck),
                Zuweisung::Zugewiesen {
                    funktion: name,
                    kombination: "F9".to_owned(),
                },
                "{kennung} nimmt keine Zuweisung an"
            );
            assert!(modell.geaendert());
            let tasten = modell.tastentext(stelle).expect("die Zeile hat Tasten");
            assert!(
                tasten.contains("F9"),
                "{kennung} zeigt F9 nicht an: {tasten}"
            );
        }
    }

    /// Die Beschriftung nennt die Taste, die der Nutzer druecken muss — auch
    /// auf einer deutschen Tastatur.
    ///
    /// **Diese Pruefung waere vor S2 gefallen.** Bis dahin schlug der
    /// Ereignisabgriff Buchstaben ueber den Tastencode nach, also ueber die
    /// Stelle; die Zeile `Cmd+Y` wirkte auf einer deutschen Tastatur unter der
    /// Aufschrift Z. Seit S2 ist der Name der Taste ihr Zeichen, und beide
    /// Richtungen stehen hier: was die Ansicht schreibt, und was ein Druck auf
    /// die so beschriftete Taste ergibt.
    #[test]
    fn die_beschriftung_nennt_die_taste_auf_einer_deutschen_tastatur() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let tasten = modell
            .tastentext(zeile_von("vorschau_umschalten"))
            .expect("die Zeile hat Tasten");
        assert!(
            tasten.contains("Cmd+Y"),
            "die Vorschau-Zeile schreibt nicht Cmd+Y, sondern {tasten}"
        );

        // Auf einer deutschen Tastatur traegt die Stelle kVK_ANSI_Z die
        // Aufschrift Y und meldet das Zeichen 'y'. Genau dieser Druck muss die
        // Kombination ergeben, die oben in der Zeile steht.
        let aufschrift_y = Tastendruck::aus_ereignis(code_von_pflicht("z"), Some('y'), roh::BEFEHL);
        let kombination =
            Kombination::aus_tastendruck(aufschrift_y).expect("die Taste hat einen Namen");
        assert_eq!(anzeige(&kombination), "Cmd+Y");

        // Und die Gegenprobe: die Stelle kVK_ANSI_Y traegt dort die Aufschrift
        // Z und darf die Vorschau nicht umschalten.
        let aufschrift_z = Tastendruck::aus_ereignis(code_von_pflicht("y"), Some('z'), roh::BEFEHL);
        let kombination =
            Kombination::aus_tastendruck(aufschrift_z).expect("die Taste hat einen Namen");
        assert_eq!(anzeige(&kombination), "Cmd+Z");

        // Die Regel, nicht der Einzelfall: keine Zeile der Ansicht beschriftet
        // eine ueber ihr Zeichen nachgeschlagene Taste mit etwas anderem als
        // ihrem Namen in der Schreibweise von anzeige(). Fuer Buchstaben und
        // Ziffern ist das der Grossbuchstabe des Zeichens; `plus` und `minus`
        // (Runde 20) heissen "Plus" und "Minus", weil ein nacktes `+` in
        // einer mit `+` gefuegten Anzeigeform nicht lesbar waere und eine
        // Uebersetzungsliste die zweite Namensliste waere, die der Plan der
        // Runde 3 ausschliesst.
        for funktion in Belegung::auslieferung().funktionen() {
            for kombination in funktion.tasten() {
                let taste = kombination.taste();
                if taste.zeichen().is_none() {
                    continue;
                }
                let beschriftet = anzeige(kombination);
                let erwartet = teilanfang_gross(taste.name);
                assert!(
                    beschriftet.ends_with(&format!("+{erwartet}")),
                    "{} beschriftet eine Zeichentaste falsch: {beschriftet}, erwartet {erwartet} am Ende",
                    funktion.kennung()
                );
            }
        }
    }

    /// Die Namen der Funktionen unter einer Bereichsueberschrift, in der
    /// Reihenfolge der Ansicht.
    ///
    /// Gelesen wird, wie der Nutzer liest: ab der Ueberschrift bis zur
    /// naechsten. Damit misst der Helfer die Gliederung und nicht die
    /// Zuordnung, aus der sie entsteht.
    fn funktionen_unter(modell: &Belegungsmodell, ueberschrift: &str) -> Vec<String> {
        let mut namen = Vec::new();
        let mut darin = false;
        for stelle in 0..modell.zeilen() {
            match modell.ueberschrift(stelle) {
                Some(gesehen) => {
                    if darin {
                        break;
                    }
                    darin = gesehen == ueberschrift;
                }
                None => {
                    if darin {
                        namen.push(
                            modell
                                .name(stelle)
                                .expect("eine Funktionszeile hat einen Namen")
                                .to_owned(),
                        );
                    }
                }
            }
        }
        assert!(
            !namen.is_empty(),
            "die Ansicht fuehrt keine Ueberschrift {ueberschrift} mit Funktionen"
        );
        namen
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

#[cfg(test)]
mod suchproben {
    use super::*;

    /// Ein Modell ueber der Auslieferungsbelegung, wie die Ansicht es zeigt.
    fn modell() -> Belegungsmodell {
        Belegungsmodell::neu(Belegung::auslieferung())
    }

    /// Tippt den Text Zeichen fuer Zeichen, wie der Faenger es tut.
    fn tippen(lage: &mut Suchlage, text: &str, modell: &Belegungsmodell) {
        for zeichen in text.chars() {
            assert!(
                lage.zeichen_anhaengen(zeichen, modell),
                "das Zeichen {zeichen:?} wurde nicht aufgenommen"
            );
        }
    }

    /// Das erste Zeichen sucht, und die Auswahl steht auf dem ersten Treffer
    /// (C1.1).
    #[test]
    fn das_erste_zeichen_springt_auf_den_ersten_treffer() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        assert_eq!(lage.zielzeile(), None, "ohne Suchtext keine Zielzeile");

        tippen(&mut lage, "t", &modell);
        let erste = (0..modell.zeilen())
            .find(|&stelle| modell.zeile_traegt(stelle, "t"))
            .expect("die Auslieferungsbelegung traegt irgendwo ein t");
        assert_eq!(lage.zielzeile(), Some(erste));
    }

    /// Gesucht wird als Teilzeichenfolge und ohne Ruecksicht auf Gross- und
    /// Kleinschreibung (C1.4, C1.5).
    ///
    /// „datum" steht in der Auslieferungsbelegung mitten im Wort, naemlich in
    /// „Spalte Änderungsdatum ein- und ausblenden". Ein Anfangsvergleich, wie
    /// ihn die bis zur Runde 10 bestehende Sprungmarke der Dateiliste zog,
    /// faende die Zeile nicht.
    ///
    /// Getroffen wird mehr als diese eine Zeile — „Nach Änderungsdatum
    /// sortieren" traegt das Wort ebenso —, und deshalb prueft der Test die
    /// ganze Trefferliste und nicht die erste Zielzeile.
    #[test]
    fn datum_findet_das_wort_in_der_wortmitte_und_in_jeder_schreibweise() {
        let modell = modell();
        let ziel = zeile_der_spalte_aenderungsdatum(&modell);

        let mut erwartet = None;
        for geschrieben in ["datum", "DATUM", "Datum"] {
            let mut lage = Suchlage::neu();
            tippen(&mut lage, geschrieben, &modell);
            let getroffen = trefferzeilen(&lage);
            assert!(
                getroffen.contains(&ziel),
                "{geschrieben:?} findet die Zeile in der Wortmitte nicht"
            );
            match &erwartet {
                None => erwartet = Some(getroffen),
                Some(erste) => assert_eq!(
                    &getroffen, erste,
                    "{geschrieben:?} findet andere Zeilen als »datum«"
                ),
            }
        }
    }

    /// Ein Suchtext mit Leerzeichen findet einen mehrwortigen Namen.
    ///
    /// Das ist der Fall, wegen dessen die Schaltflaeche „Zuweisen" von der
    /// Leertaste auf Cmd+T umgezogen ist: fast jeder Funktionsname besteht aus
    /// mehreren Woertern.
    #[test]
    fn ein_suchtext_mit_leerzeichen_findet_einen_mehrwortigen_namen() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "spalte änderungsdatum", &modell);
        assert_eq!(
            lage.zielzeile(),
            Some(zeile_der_spalte_aenderungsdatum(&modell))
        );
    }

    /// Die Zeilen, die der Suchtext trifft, aufsteigend.
    ///
    /// Gelesen aus derselben Rechnung, die die Suchlage benutzt; die Liste
    /// selbst gibt sie nach aussen nicht heraus, weil die Ansicht sie nicht
    /// braucht.
    fn trefferzeilen(lage: &Suchlage) -> Vec<usize> {
        lage.treffer.clone()
    }

    /// Die Kennung einer Funktion ist kein Treffer (C1.3).
    ///
    /// Sie steht nicht auf dem Schirm; gesucht wird ueber die zwei Spalten der
    /// Ansicht. `spalte_datum_umschalten` ist die Kennung derselben Funktion,
    /// die der Test darueber ueber ihren Namen findet.
    #[test]
    fn die_kennung_einer_funktion_ist_kein_treffer() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "spalte_datum", &modell);
        assert_eq!(lage.zielzeile(), None);
        assert!(lage.meldung().contains("kein Treffer"));
    }

    /// Ein Steuerzeichen und ein Zeichen aus dem privaten Bereich werden
    /// abgewiesen (C1.2).
    ///
    /// Die Regel dafuer ist die eine Zeichenregel des Filters, und eine zweite
    /// entsteht nicht. U+F701 ist das Zeichen, das AppKit dem Pfeil ab
    /// beilegt.
    #[test]
    fn steuerzeichen_und_funktionstasten_gehen_nicht_in_den_suchtext() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        for abgewiesen in ['\u{1B}', '\r', '\u{7F}', '\u{F701}', '\u{F8FF}'] {
            assert!(
                !lage.zeichen_anhaengen(abgewiesen, &modell),
                "{abgewiesen:?} haette nicht aufgenommen werden duerfen"
            );
        }
        assert_eq!(lage.suchtext, "");
    }

    /// Hinter dem letzten Treffer geht es beim ersten weiter (C1.7).
    #[test]
    fn hinter_dem_letzten_treffer_geht_es_beim_ersten_weiter() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "vorschau", &modell);

        let erste = lage.zielzeile().expect("»vorschau« hat Treffer");
        let mut gesehen = vec![erste];
        loop {
            assert!(lage.naechster_treffer(), "die Suche hat Treffer");
            let zeile = lage.zielzeile().expect("ein angesteuerter Treffer");
            if zeile == erste {
                break;
            }
            assert!(
                gesehen.len() < modell.zeilen(),
                "der Ring laeuft nicht auf den ersten Treffer zurueck"
            );
            gesehen.push(zeile);
        }
        assert!(
            gesehen.len() > 1,
            "»vorschau« traegt nur einen Treffer; der Umlauf ist damit nicht gemessen"
        );
    }

    /// Eine Bereichsueberschrift ist nie ein Treffer (C1.6).
    ///
    /// „Vorschau" ist zugleich der Name eines Funktionsbereichs und steht
    /// deshalb als Ueberschriftszeile in der Ansicht. Getroffen werden trotzdem
    /// allein Funktionszeilen.
    #[test]
    fn eine_bereichsueberschrift_ist_nie_ein_treffer() {
        let modell = modell();
        let ueberschrift = (0..modell.zeilen())
            .find(|&stelle| modell.ueberschrift(stelle) == Some("Vorschau"))
            .expect("die Gliederung fuehrt einen Bereich »Vorschau«");

        let mut lage = Suchlage::neu();
        tippen(&mut lage, "vorschau", &modell);

        let erste = lage.zielzeile().expect("»vorschau« hat Treffer");
        let mut zeile = erste;
        loop {
            assert_ne!(zeile, ueberschrift, "die Ueberschrift ist ein Treffer");
            assert!(
                modell.ueberschrift(zeile).is_none(),
                "die Zeile {zeile} ist eine Ueberschrift und ein Treffer"
            );
            assert!(lage.naechster_treffer());
            zeile = lage.zielzeile().expect("ein angesteuerter Treffer");
            if zeile == erste {
                break;
            }
        }
    }

    /// Die Ruecktaste nimmt das letzte Zeichen weg und sucht erneut (C1.8).
    #[test]
    fn die_ruecktaste_kuerzt_den_suchtext_und_sucht_erneut() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "datum", &modell);
        assert!(lage.zielzeile().is_some());

        assert!(lage.letztes_zeichen_weg(&modell));
        assert_eq!(lage.suchtext, "datu");

        for _ in 0..4 {
            assert!(lage.letztes_zeichen_weg(&modell));
        }
        assert_eq!(lage.suchtext, "");
        assert_eq!(lage.zielzeile(), None, "ohne Suchtext gibt es kein Ziel");
    }

    /// Bei leerem Suchtext bleiben Eingabetaste und Ruecktaste wirkungslos
    /// (C1.8, C1.17).
    ///
    /// Beide melden es ueber ihren Rueckgabewert, damit die Ansicht ihre
    /// Meldungszeile stehen laesst, statt eine Zuweisungsmeldung zu
    /// ueberschreiben.
    #[test]
    fn bei_leerem_suchtext_bleiben_eingabetaste_und_ruecktaste_wirkungslos() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        assert!(!lage.naechster_treffer());
        assert!(!lage.letztes_zeichen_weg(&modell));
        assert_eq!(lage, Suchlage::neu(), "die leere Suche hat sich geaendert");
    }

    /// Ein Suchtext ohne Treffer laesst die Auswahl stehen und sagt es (C1.9).
    #[test]
    fn ohne_treffer_bleibt_die_auswahl_stehen_und_die_meldung_sagt_es() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "datum", &modell);
        let vorher = lage.zielzeile();
        assert!(vorher.is_some());

        tippen(&mut lage, "xyz", &modell);
        assert_eq!(lage.zielzeile(), None, "ohne Treffer keine Zielzeile");
        assert!(lage.meldung().contains("kein Treffer"));
        assert!(
            !lage.naechster_treffer(),
            "ohne Treffer geht es nicht weiter"
        );
    }

    /// Die Meldungszeile nennt Suchtext, Trefferzahl und Stelle darin (C1.10).
    #[test]
    fn die_meldung_nennt_suchtext_trefferzahl_und_stelle() {
        let modell = modell();
        let mut lage = Suchlage::neu();
        tippen(&mut lage, "vorschau", &modell);

        let meldung = lage.meldung();
        assert!(meldung.contains("vorschau"), "{meldung}");
        assert!(meldung.contains("Treffer 1 von "), "{meldung}");

        assert!(lage.naechster_treffer());
        assert!(
            lage.meldung().contains("Treffer 2 von "),
            "{}",
            lage.meldung()
        );
    }

    /// Der Suchtext hat keinen Zeitgeber (C1.12).
    ///
    /// **Gezaehlt werden Erklaerungen im Quelltext**, wie in
    /// [`crate::quellbaum`] beschrieben: an keinem Rueckgabewert ist abzulesen,
    /// dass es keine Uhr gibt. Diese Datei fuehrt keinen Zeitpunkt der
    /// Standardbibliothek; ein Zeitgeber, der den Suchtext nach einer Pause
    /// zuruecksetzte, waere hier zu sehen. Die Sekundenregel der Sprungmarke
    /// aus C2 der Runde 1 war die eine Stelle, die es anders hielt; sie ist mit
    /// der Sprungmarke in der Runde 10 gefallen, und
    /// `krk_core::verzeichnis::filter` fuehrt keine Zeitmessung mehr.
    ///
    /// **Die zwei Nadeln stehen zusammengesetzt da**, wie bei jeder Zaehlprobe
    /// dieses Baums: als ein Stueck geschrieben faende jede sich in dieser
    /// Datei selbst und liesse die Probe fehlschlagen, ohne dass eine Uhr da
    /// waere.
    #[test]
    fn die_suche_fuehrt_keinen_zeitgeber() {
        let uhr = concat!("Inst", "ant");
        let dauer = concat!("Dura", "tion");
        let (_, inhalt) = crate::quellbaum::quelldateien()
            .into_iter()
            .find(|(name, _)| name == "krk-ui/src/belegungsmodell.rs")
            .expect("der Quellbaum fuehrt belegungsmodell.rs");
        assert!(!inhalt.contains(uhr), "die Suche fuehrt eine Uhr");
        assert!(!inhalt.contains(dauer), "die Suche fuehrt eine Zeitspanne");
    }

    /// Die Zeile der Funktion, deren Name „datum" mitten im Wort traegt.
    ///
    /// Gesucht wird sie ueber ihre Kennung, damit der Test nicht an der
    /// Schreibweise des Namens haengt.
    fn zeile_der_spalte_aenderungsdatum(modell: &Belegungsmodell) -> usize {
        let belegung = Belegung::auslieferung();
        let name = belegung
            .funktion("spalte_datum_umschalten")
            .expect("die Auslieferungsbelegung kennt spalte_datum_umschalten")
            .name()
            .to_owned();
        assert!(
            name.to_lowercase().contains("datum"),
            "der Name {name:?} traegt kein »datum« mehr; der Test misst nichts"
        );
        (0..modell.zeilen())
            .find(|&stelle| modell.name(stelle) == Some(name.as_str()))
            .expect("die Funktion steht in der Gliederung")
    }
}
