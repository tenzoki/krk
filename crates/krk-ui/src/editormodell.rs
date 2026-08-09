//! Das Modell des eingebauten Editors: welche Datei er haelt, ihren Stand, ob
//! der Stand von der Datei abweicht, welche Ansicht gewaehlt ist und was die
//! laufende Suche gefunden hat (C2 bis C6).
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use`-Zeile aus einer
//! Objective-C-Bindungskiste, wie in [`crate::fenstermodell`],
//! [`crate::vorschaumodell`] und den sechs uebrigen Modulen daneben. Die
//! Ansicht dazu ist `crate::appkit::editor` mit ihrer `NSTextView`; sie
//! rechnet nichts nach, was hier steht, und dieses Modul weiss nichts von ihr.
//!
//! Die vier Nachbarmodule schreiben denselben Satz mit dem Namen der
//! Bindungskiste darin. Hier steht er ohne, und das ist kein Stilbruch,
//! sondern die Abnahme der Schritte 15 und 16: beide messen die Grenze, indem
//! sie den Kistennamen in genau dieser Datei zaehlen, und erwarten null. Ein
//! Satz, der den Namen nennt, um seine Abwesenheit zu behaupten, faellt durch
//! dieselbe Messung wie eine Verwendung.
//!
//! # Was hier steht und was in `krk_core::text`
//!
//! ```text
//!  Pfad ──> Ladevorgang (Arbeitsfaden) ──> text::datei::oeffnen
//!                                                │
//!                     Abweisung <────────────────┤
//!                                                v
//!   ┌──────────────────── Editormodell ─────────────────────┐
//!   │ pfad, stand, abweichung, ansicht, typ, stempel        │
//!   │                                                       │
//!   │  stand ──> suche::alle ──> Suchlauf: Treffer, der     │
//!   │                            angesteuerte darunter      │
//!   │                                                       │
//!   │  stand ──> text::datei::sichern ──> Platte            │
//!   └───────────────────────────────────────────────────────┘
//! ```
//!
//! Gerechnet wird in `krk_core::text` und hier nicht noch einmal: der
//! Zeilenindex, die Suche, das Ersetzen, das Einlesen und die Sicherungsform
//! stehen dort. Dieses Modul **haelt** und ruft.
//!
//! Insbesondere stellt es die Zusage ueber den gehaltenen Stand nicht ein
//! zweites Mal her. Sie lautet: gueltiges UTF-8 ohne Bytefolgenmarke, `\n` als
//! einziges Zeilenende. `krk_core::text::datei` stellt sie beim Einlesen her,
//! und alles hier rechnet darauf. Wer Text von anderswo hereingibt, fuehrt ihn
//! durch `krk_core::text::datei::in_gehaltene_form`.
//!
//! # Die zwei Eingaenge fuer fremden Text
//!
//! Fremd heisst: nicht aus `krk_core::text::datei::einlesen`. Das Modell hat
//! genau zwei solche Eingaenge, und beide fuehren durch `in_gehaltene_form`:
//!
//! ```text
//!  ganzer Stand aus der Textflaeche ──> bearbeiten
//!                                          │
//!  Ersatztext aus dem Eingabefeld ──> ersetzung_vorbereiten
//!                                          │
//!                                          v
//!                                  in_gehaltene_form ──> stand
//! ```
//!
//! Der erste ist der groessere: eine `NSTextView` bewahrt eingefuegten Text
//! zeichengetreu auf, also bringt ein Einfuegen aus einem Windows-Projekt
//! `\r\n` mit. Der zweite ist der kleinere und war der einzige, den der
//! Modulkopf von `krk_core::text::datei` bis zum 260809 vorhersah.
//!
//! **Die drei Zuweisungen an [`Editormodell::stand`] sind nicht die
//! Eingaenge**, und daran haengt eine Messung. Wer statt der beiden Eingaenge
//! die Zuweisungen wandelte, braeche [`Editormodell::treffer_ersetzen`]:
//! `krk_core::text::suche::einen_ersetzen` liefert den naechsten Treffer als
//! Byteversatz **in den Stand, den es eben gebildet hat**. Eine Wandlung
//! danach verschoebe jeden Versatz dahinter, die Suche nach seiner Stelle in
//! der neu gebildeten Liste ginge leer aus, und der Durchgang bliebe stehen,
//! ohne dass jemand etwas meldete. Die Probe
//! `ein_ersatztext_mit_crlf_kommt_in_gehaltener_form_an` haelt genau diesen
//! Fall fest.
//!
//! # Ein Stand, und deshalb kann ein Ansichtswechsel nichts verlieren
//!
//! [`Editormodell::stand`] ist die **einzige** Zeichenkette dieses Modells.
//! [`Ansicht`] steht daneben und sagt allein, wie die Textflaeche denselben
//! Stand darstellt. Daraus folgt das zehnte Abnahmekriterium von C3, "beide
//! Ansichten arbeiten auf demselben Stand und nicht auf zwei Kopien", **ohne
//! eine Vorkehrung**: [`Editormodell::ansicht_umschalten`] fasst weder
//! [`Editormodell::stand`] noch die Abweichungsmarke an, und es gibt keinen
//! zweiten Textbestand, in den etwas verlorengehen koennte. Die Ansicht setzt
//! die Einfaerbung als voruebergehende Merkmale des Layoutverwalters und
//! nicht in den Textspeicher; das ist die zweite Haelfte derselben Zusage und
//! steht in `crate::appkit::editor`.
//!
//! Die Ansichtswahl bleibt ueber einen Dateiwechsel hinweg stehen. Wer eine
//! Markdown-Datei gerendert liest und danach eine Codedatei oeffnet, bekommt
//! deren Formatansicht und nicht die Rohansicht; C3 legt das so fest, und
//! [`Editormodell::oeffnen`] setzt die Ansicht deshalb nicht zurueck.
//!
//! # Der ungesicherte Stand ist eine Marke und kein Vergleich
//!
//! [`Editormodell::hat_ungesicherten_stand`] liest ein `bool`, das
//! [`Editormodell::bearbeiten`] setzt und das Oeffnen wie das gelungene
//! Sichern loeschen. Es ist **nicht** der Vergleich des Standes mit dem
//! Dateiinhalt.
//!
//! **Der Preis steht hier und wird nicht verschwiegen:** wer eine Aenderung
//! tippt und sie wieder zuruecknimmt, meldet weiterhin ungesicherten Stand und
//! bekommt an den vier Anlaessen aus C4 die Nachfrage. Der Gegenwert ist, dass
//! die Frage "haelt der Editor ungesicherten Stand" ein Blick auf ein `bool`
//! ist und kein Vergleich zweier Zeichenketten. Sie wird bei jedem Tastendruck
//! gestellt, weil die Anzeige aus dem zweiten Abnahmekriterium von C4 an ihr
//! haengt, und ein Vergleich haette bei einer Datei an der Grenze von 16 MB je
//! Tastendruck 16 MB zu lesen. Der Zustandsuebergang, den der Spec zeichnet,
//! lautet "tippen oder ersetzen" und nicht "der Stand weicht ab"; die Marke
//! bildet ihn genau ab.
//!
//! # Der Arbeitsfaden
//!
//! [`Editormodell::oeffnen`] kehrt sofort zurueck: das Lesen laeuft je Anfrage
//! auf einem eigenen Faden, der genau eine Meldung ueber einen
//! `sync_channel(1)` schickt und endet. Derselbe Zuschnitt wie `Ladevorgang`
//! in [`crate::vorschaumodell`], und aus demselben Grund keine
//! Generationspruefung: eine neue Anfrage laesst den alten Empfaenger fallen,
//! das `send` des ueberholten Fadens scheitert still. Der Editor haelt
//! hoechstens eine Datei, also hoechstens einen Ladevorgang; der Fall ist noch
//! einfacher als bei der Vorschau mit ihren Tabs.
//!
//! Bis die Meldung eintrifft, bleibt der bisherige Stand stehen. Eine
//! Abweisung laesst ihn ebenfalls stehen: der Editor wirft nichts weg, weil
//! eine andere Datei sich nicht oeffnen liess.
//!
//! # Was dieses Modul nicht tut
//!
//! Es **fragt nicht nach**. Die Nachfrage an den vier Anlaessen aus C4 ist ein
//! Blatt am Fenster, und das Blatt wohnt in `crate::appkit`. Dieses Modul
//! beantwortet allein, ob es etwas zu fragen gibt
//! ([`Editormodell::hat_ungesicherten_stand`]), und fuehrt aus, was die
//! Antwort verlangt. [`Editormodell::oeffnen`] auf ein Modell mit
//! ungesichertem Stand ersetzt die Datei ohne Rueckfrage; der Aufrufer hat vor
//! dem Ruf zu fragen.

// **Diese Zeile faellt mit Schritt 37 und nicht, wie hier bis zum 260809
// stand, mit Schritt 16.** Der Schritt 16 baut die Textflaeche und leiht sich
// daraus zwei Stuecke, `Editormodell::neu` und `Editormodell::stand`. Jedes
// andere haengt an einem Befehl, den es ausloest, und der Befehl kommt mit
// seinem eigenen Schritt: das Lesen auf dem Arbeitsfaden mit S24, das Sichern
// mit S25, die Abweichungsmarke mit S26, die beiden Ansichten mit S33, der
// Suchlauf mit S36 und das Ersetzen mit S37. Der letzte davon ist S37; **dann**
// ist die Zeile wegzunehmen.
//
// Gemessen am 260809 nach S16, mit entfernter Zeile: `cargo clippy --workspace
// --all-targets` meldet vierzehn Fundstellen toten Werts in dieser Datei, und
// der Arbeitsbereich stuende rot, weil `make lint` mit `-D warnings` faehrt.
// Tot ist auch dann nichts: die Pruefungen am Dateiende fassen jedes Stueck
// dieses Moduls an.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::SystemTime;

use krk_core::text::{Abweisung, Treffer, datei, suche};

/// Welche der beiden Ansichten aus C3 die Textflaeche zeigt.
///
/// **Zwei Werte und kein dritter.** Der Spec kennt die Rohansicht und die
/// Formatansicht; dass die Formatansicht je nach [`Dateityp`] verschieden
/// aussieht, macht sie nicht zu mehreren Ansichten, sondern zu einer mit einer
/// Fallunterscheidung darin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ansicht {
    /// Die Zeichen der Datei ohne Umbruch, ohne Einfaerbung, ohne Ausblendung.
    Roh,
    /// Je Dateityp besetzt: Umbruch und lesbarere Schrift fuer alles, dazu die
    /// Einfaerbung der Syntaxkiste und bei Markdown die Ueberschriften.
    ///
    /// **Die Vorgabe**, weil sie die Ansicht ist, die C3 je Dateityp
    /// beschreibt; die Rohansicht ist der ausdrueckliche Schritt von ihr weg.
    #[default]
    Format,
}

impl Ansicht {
    /// Die jeweils andere Ansicht.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig.
    pub fn andere(self) -> Self {
        match self {
            Ansicht::Roh => Ansicht::Format,
            Ansicht::Format => Ansicht::Roh,
        }
    }
}

/// Was die Formatansicht ueber die gehaltene Datei **aus ihrem Pfad** weiss.
///
/// # Warum hier zwei Werte stehen und nicht die drei aus C3
///
/// C3 nennt drei Besetzungen der Formatansicht: Markdown gerendert, Code mit
/// Syntaxhervorhebung, einfacher Text mit Umbruch. Die Grenze zwischen den
/// beiden letzten ist aus dem Pfad allein **nicht zu ziehen**: "Code" heisst
/// nach dem sechsten Abnahmekriterium von C3 genau "die eingebundene Kiste
/// kennt eine Sprache dafuer", und eine Datei in einer Sprache, die sie nicht
/// kennt, faellt auf die Textdarstellung zurueck. Wer diese Frage hier
/// beantwortete, muesste die Sprachliste der Kiste ein zweites Mal fuehren und
/// waere ab der ersten Fassung, die eine Sprache nachreicht, falsch.
///
/// Gestellt wird deshalb allein die Frage, die aus dem Pfad zu beantworten ist
/// und die die Ansicht braucht, bevor sie die Kiste fragt: **verlangt diese
/// Datei die Markdown-Zutaten?** Ueber Code gegen einfachen Text entscheidet
/// `crate::appkit::editor` beim Darstellen, indem es die Kiste nach dem Pfad
/// fragt und ihre Antwort nimmt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Dateityp {
    /// Markdown: die Formatansicht setzt Ueberschriften groesser, rueckt
    /// Listen ein und unterstreicht Links, zusaetzlich zur Einfaerbung.
    Markdown,
    /// Alles Uebrige, einschliesslich "der Editor haelt keine Datei".
    #[default]
    Sonstiges,
}

/// Die Endungen, die als Markdown gelten.
///
/// Verglichen ohne Ruecksicht auf Gross- und Kleinschreibung, wie
/// `BILDENDUNGEN` in [`crate::vorschaumodell`]. Die Liste nennt die Endungen,
/// die auf einem Mac ueblich sind; eine Endung, die fehlt, bekommt die
/// gewoehnliche Formatansicht und keinen Fehler.
const MARKDOWNENDUNGEN: [&str; 4] = ["md", "markdown", "mdown", "mkd"];

impl Dateityp {
    /// Was der Pfad ueber die Datei sagt.
    pub fn von_pfad(pfad: &Path) -> Self {
        let endung = pfad
            .extension()
            .map(|endung| endung.to_string_lossy().to_ascii_lowercase());
        match endung {
            Some(endung) if MARKDOWNENDUNGEN.contains(&endung.as_str()) => Dateityp::Markdown,
            _ => Dateityp::Sonstiges,
        }
    }
}

/// Der Zustand der Datei auf der Platte beim Oeffnen oder Sichern (C4).
///
/// Aenderungszeit und Groesse zusammen, weil eine Aenderung, die die Groesse
/// nicht bewegt, haeufig ist und eine, die die Zeit nicht bewegt, selten. Die
/// beiden Angaben sind die, die ein `stat(2)` ohnehin liefert; ein Pruefwert
/// ueber den Inhalt braeuchte einen Lesevorgang und damit genau das, was das
/// elfte Abnahmekriterium von C6 der Leiste verbietet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stempel {
    /// Der Zeitpunkt der letzten Aenderung.
    pub geaendert: SystemTime,
    /// Die Groesse in Bytes.
    pub groesse: u64,
}

impl Stempel {
    /// Der Stempel des genannten Pfades; `None`, wenn er sich nicht erheben
    /// laesst.
    ///
    /// `metadata` und nicht `symlink_metadata`, damit eine Verknuepfung nach
    /// dem behandelt wird, worauf sie zeigt. Dieselbe Wahl wie in
    /// `krk_core::text::datei::oeffnen`, und sie muss dieselbe sein: sonst
    /// verglichen Oeffnen und Stempel zwei verschiedene Dateien.
    pub fn von_pfad(pfad: &Path) -> Option<Self> {
        let roh = std::fs::metadata(pfad).ok()?;
        Some(Self {
            geaendert: roh.modified().ok()?,
            groesse: roh.len(),
        })
    }
}

/// Der laufende Suchlauf im gehaltenen Stand (C5).
///
/// Die Trefferliste gehoert zu **dem** Stand, aus dem sie gebildet wurde. Jede
/// Aenderung des Standes macht ihre Versaetze ungueltig, und ein ungueltiger
/// Versatz ist in Rust kein falsches Ergebnis, sondern eine Panik. Deshalb
/// gibt es genau zwei Wege, auf denen ein Suchlauf eine Aenderung ueberlebt,
/// und beide bilden ihn neu: [`Editormodell::treffer_ersetzen`] und
/// [`Editormodell::alle_treffer_ersetzen`]. Jede andere Aenderung
/// ([`Editormodell::bearbeiten`]) beendet ihn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suchlauf {
    gesucht: String,
    treffer: Vec<Treffer>,
    /// Die Stelle des angesteuerten Treffers in [`Suchlauf::treffer`]; `None`,
    /// wenn es keinen Treffer gibt oder der Durchgang eines Ersetzens zu Ende
    /// ist.
    angesteuert: Option<usize>,
}

impl Suchlauf {
    /// Wonach gesucht wird.
    pub fn gesucht(&self) -> &str {
        &self.gesucht
    }

    /// Alle Treffer im gehaltenen Stand, in Textreihenfolge.
    pub fn treffer(&self) -> &[Treffer] {
        &self.treffer
    }

    /// Wie viele Treffer die Datei enthaelt (C5).
    pub fn zahl(&self) -> usize {
        self.treffer.len()
    }

    /// Der angesteuerte Treffer.
    pub fn angesteuert(&self) -> Option<Treffer> {
        self.angesteuert.map(|stelle| self.treffer[stelle])
    }

    /// Der wievielte Treffer angesteuert ist, ab 1 gezaehlt (C5).
    ///
    /// Ab 1, weil die Zahl der Nutzer liest; die Versaetze daneben zaehlen
    /// Bytes ab 0. Dieselbe Trennung wie bei den Zeilennummern in
    /// `krk_core::text`.
    pub fn nummer(&self) -> Option<usize> {
        self.angesteuert.map(|stelle| stelle + 1)
    }

    /// Der Satz fuer die Statuszeile, der beide Zahlen aus C5 nennt.
    ///
    /// Die Fallunterscheidung ist vollstaendig: entweder es gibt einen
    /// angesteuerten Treffer, dann steht seine Nummer und die Gesamtzahl da,
    /// oder es gibt keinen, dann steht der Suchtext da und der Nutzer weiss,
    /// wonach vergeblich gesucht wurde.
    pub fn meldung(&self) -> String {
        match self.nummer() {
            Some(nummer) => format!("Treffer {nummer} von {}", self.zahl()),
            None if self.treffer.is_empty() => {
                format!("Kein Treffer für „{}“", self.gesucht)
            }
            None => format!("Kein weiterer Treffer für „{}“", self.gesucht),
        }
    }
}

/// Was der Arbeitsfaden geliefert hat.
#[derive(Debug)]
struct Geladen {
    ergebnis: Result<String, Abweisung>,
    /// Der Stempel, **vor** dem Lesen erhoben; siehe [`Ladevorgang::starten`].
    stempel: Option<Stempel>,
}

/// Ein laufendes Laden einer Datei in den Editor.
///
/// Faellt der Vorgang, faellt sein Empfaenger, und das `send` des Fadens
/// scheitert still; siehe den Modulkopf.
#[derive(Debug)]
pub struct Ladevorgang {
    empfaenger: Receiver<Geladen>,
    pfad: PathBuf,
}

impl Ladevorgang {
    /// Startet den Arbeitsfaden fuer den genannten Pfad.
    ///
    /// **Der Stempel wird vor dem Lesen erhoben und nicht danach.** Die
    /// Reihenfolge ist die vorsichtige von zweien: aendert sich die Datei
    /// waehrend des Lesens, ist der Stempel danach aelter als der gelesene
    /// Inhalt, und C4 meldet eine Aenderung von aussen, die keine war. Umgekehrt
    /// waere der Stempel neuer als der Inhalt, und die Aenderung von aussen
    /// bliebe unbemerkt, bis das naechste Sichern sie ueberschreibt. Die Zusage
    /// von C4 lautet, fremde Aenderungen nicht ohne Zutun zu ueberschreiben;
    /// eine ueberfluessige Meldung haelt sie ein, ein Ueberschreiben nicht.
    fn starten(pfad: PathBuf) -> Self {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        let fuer_faden = pfad.clone();
        let ergebnis = thread::Builder::new()
            .name("krk-editor".to_owned())
            .spawn(move || {
                let stempel = Stempel::von_pfad(&fuer_faden);
                let _ = SyncSender::send(
                    &sender,
                    Geladen {
                        ergebnis: datei::oeffnen(&fuer_faden),
                        stempel,
                    },
                );
            });
        if let Err(fehler) = ergebnis {
            // Ohne Faden kommt nie eine Meldung; der Kanal ist zu diesem
            // Zeitpunkt schon wieder ohne Sender, und `einziehen` raeumt den
            // Vorgang beim naechsten Takt ab. Der Hinweis hier ist die einzige
            // Spur, die der Fall hinterlaesst. Derselbe Zuschnitt und derselbe
            // Grund wie in `vorschaumodell`.
            eprintln!("krk: der Editor-Arbeitsfaden liess sich nicht starten: {fehler}");
        }
        Self { empfaenger, pfad }
    }
}

/// Wie ein Ladevorgang ausgegangen ist.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig.** Entweder der Editor
/// haelt danach eine neue Datei, oder er hielt sie schon und nichts hat sich
/// bewegt, oder er haelt weiter, was er vorher hielt, und der Nutzer bekommt
/// den Grund. Ein vierter Ausgang, bei dem der Editor nichts mehr haelt,
/// entsteht nicht: eine gescheiterte Anfrage wirft nichts weg.
///
/// **Der mittlere Wert ist seit dem 260809 dabei** und trennt zwei Ausgaenge,
/// die bis dahin beide `Geoeffnet` hiessen. Der Unterschied ist nicht
/// buchhalterisch: die Ansicht traegt den Stand allein bei [`Self::Geoeffnet`]
/// in die Textflaeche, und bei [`Self::SchonOffen`] gerade **nicht**. Wer die
/// beiden zusammenzoege, ueberschriebe die Textflaeche mit einem frisch
/// gelesenen Plattenstand und naehme dem Nutzer, was er getippt hat; genau das
/// tat F4 bis zum 260809
/// (`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ladeausgang {
    /// Die Datei steht; die Ansicht traegt den Stand in die Textflaeche.
    Geoeffnet,
    /// Der Editor hielt genau diese Datei schon. Es wurde nicht gelesen, nichts
    /// am Modell hat sich bewegt, und die Textflaeche bleibt unberuehrt.
    ///
    /// Der Aufrufer holt den Editor hervor und setzt den Fokus hinein, wie bei
    /// [`Self::Geoeffnet`]; das ist der Teil des Befehls, der noch etwas zu tun
    /// hat.
    SchonOffen,
    /// Der Grund gehoert in die Statuszeile aus C1. Der bisherige Stand bleibt.
    Abgewiesen(Abweisung),
}

/// Wie ein Sichern ausgegangen ist (C4).
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// Das gescheiterte Sichern ist ein eigener Wert und kein Nichts, weil das
/// zehnte Abnahmekriterium von C4 zwei Sachen zugleich verlangt: den Grund in
/// der Statuszeile und einen Stand, der stehen bleibt. Wer beides in "es hat
/// nicht geklappt" zusammenzoege, koennte das erste nicht liefern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sicherungsausgang {
    /// Geschrieben. Der Editor meldet danach keinen ungesicherten Stand mehr.
    Gesichert,
    /// Der Grund gehoert in die Statuszeile; der Stand bleibt unveraendert
    /// stehen, und ein Anlass, der auf dieses Sichern gewartet hat, unterbleibt.
    Gescheitert(String),
    /// Der Editor haelt keine Datei; es gibt nichts zu sichern.
    NichtsGehalten,
}

/// Was der Editor ueber die geoeffnete Datei weiss (C2 bis C6).
#[derive(Debug, Default)]
pub struct Editormodell {
    /// Die gehaltene Datei; `None`, solange keine gehalten wird.
    pfad: Option<PathBuf>,
    /// Der gehaltene Stand. Leer, solange keine Datei gehalten wird.
    stand: String,
    /// Ob seit dem Oeffnen oder dem letzten gelungenen Sichern etwas
    /// bearbeitet wurde; siehe den Modulkopf.
    abweichung: bool,
    /// Roh oder Format. Bleibt ueber einen Dateiwechsel hinweg stehen (C3).
    ansicht: Ansicht,
    /// Was der Pfad ueber die Datei sagt (C3).
    typ: Dateityp,
    /// Der laufende Suchlauf; `None`, solange keiner laeuft (C5).
    suchlauf: Option<Suchlauf>,
    /// Der Zustand der Datei beim Oeffnen oder beim letzten Sichern (C4).
    stempel: Option<Stempel>,
    /// Das laufende Laden, falls eines laeuft (C2).
    ladevorgang: Option<Ladevorgang>,
}

impl Editormodell {
    /// Ein Editor, der keine Datei haelt.
    pub fn neu() -> Self {
        Self::default()
    }

    /// Die gehaltene Datei; `None`, solange keine gehalten wird.
    pub fn pfad(&self) -> Option<&Path> {
        self.pfad.as_deref()
    }

    /// Ob der Editor eine Datei haelt.
    pub fn haelt_datei(&self) -> bool {
        self.pfad.is_some()
    }

    /// Ob der Editor genau diese Datei schon haelt (C2).
    ///
    /// **Die eine Stelle, an der "dieselbe Datei" beantwortet wird**, und die
    /// Bedingung, unter der [`Self::jetzt_oeffnen`] nicht liest. Verglichen
    /// wird der Pfad, wie er hereingereicht wurde, und nicht ein aufgeloester:
    /// beide Seiten stammen aus derselben Quelle, naemlich der Auswahl des
    /// Dateifensters, und ein `canonicalize` daneben kostete einen Zugriff auf
    /// die Platte fuer eine Frage, die der Vergleich schon beantwortet. Geht
    /// der Vergleich einmal daneben, liest der Editor neu — der Fehler faellt
    /// also auf die Seite des bisherigen Verhaltens und nicht auf die eines
    /// falsch stehengelassenen Standes.
    pub fn haelt_bereits(&self, pfad: &Path) -> bool {
        self.pfad.as_deref() == Some(pfad)
    }

    /// Der gehaltene Stand.
    pub fn stand(&self) -> &str {
        &self.stand
    }

    /// Ob der Editor Aenderungen haelt, die nicht in der Datei stehen (C4).
    ///
    /// Das ist die Frage, die an den vier Anlaessen aus C4 gestellt wird und an
    /// der die Anzeige aus dem zweiten Abnahmekriterium haengt. Warum sie eine
    /// Marke liest und keinen Vergleich fuehrt, steht im Modulkopf.
    pub fn hat_ungesicherten_stand(&self) -> bool {
        self.abweichung
    }

    /// Welche Ansicht gewaehlt ist (C3).
    pub fn ansicht(&self) -> Ansicht {
        self.ansicht
    }

    /// Was der Pfad ueber die gehaltene Datei sagt (C3).
    pub fn typ(&self) -> Dateityp {
        self.typ
    }

    /// Der Stempel der Datei beim Oeffnen oder beim letzten Sichern (C4).
    pub fn stempel(&self) -> Option<Stempel> {
        self.stempel
    }

    /// Der laufende Suchlauf (C5).
    pub fn suchlauf(&self) -> Option<&Suchlauf> {
        self.suchlauf.as_ref()
    }

    /// Wechselt zwischen Rohansicht und Formatansicht und liefert die neue (C3).
    ///
    /// **Fasst den Stand nicht an**, und das ist der ganze Punkt: ein
    /// Ansichtswechsel kann keine ungesicherte Aenderung verlieren, weil er
    /// nichts anfasst, worin eine stecken koennte. Weder [`Self::stand`] noch
    /// die Abweichungsmarke noch der Suchlauf aendern sich.
    pub fn ansicht_umschalten(&mut self) -> Ansicht {
        self.ansicht = self.ansicht.andere();
        self.ansicht
    }

    /// Nimmt die genannte Datei auf (C2).
    ///
    /// Kehrt sofort zurueck; gelesen wird auf dem Arbeitsfaden aus dem
    /// Modulkopf, und geprueft wird dort von
    /// `krk_core::text::datei::oeffnen`, der einen Stelle, die entscheidet, ob
    /// der Editor eine Datei ueberhaupt oeffnet. Bis die Meldung eintrifft,
    /// haelt der Editor unveraendert, was er vorher hielt.
    ///
    /// **Fragt nicht nach.** Steht ungesicherter Stand offen, ist das einer der
    /// vier Anlaesse aus C4, und die Nachfrage gehoert vor diesen Ruf; siehe
    /// den Modulkopf.
    ///
    /// **Diese Funktion traegt die Abkuerzung aus [`Self::haelt_bereits`]
    /// noch nicht, und wer sie in Betrieb nimmt, hat sie mitzunehmen.** Sie hat
    /// heute keinen Aufrufer ausser den Pruefungen; der Weg des Nutzers laeuft
    /// ueber [`Self::jetzt_oeffnen`], und dort steht die Abkuerzung. Mit S24
    /// wechselt der Aufrufer auf diese Funktion, und ohne die Abkuerzung kaeme
    /// der Verlust aus
    /// `issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`
    /// stumm zurueck: der Faden laese die Datei neu, [`Self::uebernehmen`]
    /// setzte den Plattenstand ein, und die Ansicht schriebe ihn ueber das
    /// Getippte. Der Ausgang dafuer steht bereit und heisst
    /// [`Ladeausgang::SchonOffen`]; er ist hier nicht vorweggebaut, weil diese
    /// Funktion keinen Ausgang meldet und S24 ihr erst einen gibt.
    pub fn oeffnen(&mut self, pfad: &Path) {
        self.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf()));
    }

    /// Nimmt die genannte Datei auf dem rufenden Faden auf (C2).
    ///
    /// **Der Zwischenstand, bis das Lesen auf dem Arbeitsfaden in Betrieb
    /// geht.** [`Self::oeffnen`] startet den Faden aus dem Modulkopf, aber die
    /// Antwort holt erst ein Takt auf dem Hauptfaden ab, und den baut ein
    /// spaeterer Schritt; bis dahin faende [`Self::einziehen`] niemand, der ihn
    /// ruft. Hier wird deshalb gelesen, wo der Aufrufer steht. Der Preis steht
    /// im Spec unter `## Verhaeltnis zu den zehn Zeitzusagen`: solange der
    /// Editor eine grosse Datei einliest, haelt der Hauptfaden an, und die
    /// Zusage, dass die Dateifenster dabei bedienbar bleiben, gilt noch nicht.
    ///
    /// Geprueft wird in `krk_core::text::datei::oeffnen`, derselben einen
    /// Stelle wie auf dem Arbeitsfaden, und der Stempel wird **vor** dem Lesen
    /// erhoben, aus dem Grund, der an [`Ladevorgang::starten`] steht. Der
    /// Uebergang in den gehaltenen Stand geht durch [`Self::uebernehmen`] und
    /// ist damit derselbe wie dort.
    ///
    /// **Fragt nicht nach**, wie [`Self::oeffnen`]; siehe den Modulkopf.
    ///
    /// # Die Datei, die der Editor schon haelt, wird nicht neu gelesen
    ///
    /// Haelt der Editor genau diesen Pfad, kehrt die Funktion mit
    /// [`Ladeausgang::SchonOffen`] zurueck, **bevor** sie liest, und ruehrt
    /// nichts an. Ohne diese Zeile ist ein zweites F4 auf dieselbe Datei ein
    /// vollwertiges Oeffnen: [`Self::uebernehmen`] setzt den Plattenstand ein,
    /// loescht die Abweichungsmarke, und die Ansicht schreibt den Plattenstand
    /// ueber das, was der Nutzer getippt hat. Genau diesen Weg ging der Nutzer
    /// am 260809, weil die Vorschau den Editor nach C1 verdraengt und F4 der
    /// einzige Befehl ist, der ihn mit seiner Datei zurueckholt
    /// (`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`).
    ///
    /// **Der Preis steht hier und wird nicht verschwiegen:** F4 auf die schon
    /// gehaltene Datei liest sie damit auch dann nicht neu, wenn sie sich von
    /// aussen geaendert hat. Ein Befehl zum Neulesen gibt es nicht, und C2 sagt
    /// keinen zu; die Aenderung von aussen traegt S31, und die Frage, was mit
    /// einem ungesicherten Stand dabei geschieht, gehoert der Nachfrage aus C4
    /// (S27, S28). Solange es kein Sichern gibt, ist der ungesicherte Stand das
    /// einzige Stueck Arbeit im Programm, das sich nicht wiederherstellen
    /// laesst; der Plattenstand laesst sich jederzeit wieder lesen.
    pub fn jetzt_oeffnen(&mut self, pfad: &Path) -> Ladeausgang {
        if self.haelt_bereits(pfad) {
            return Ladeausgang::SchonOffen;
        }
        let stempel = Stempel::von_pfad(pfad);
        let geladen = Geladen {
            ergebnis: datei::oeffnen(pfad),
            stempel,
        };
        self.uebernehmen(pfad.to_path_buf(), geladen)
    }

    /// Uebernimmt, was ein Lesevorgang geliefert hat.
    ///
    /// **Die eine Stelle, an der eine gelesene Datei zum Stand des Editors
    /// wird.** Zwei Wege fuehren hierher, [`Self::einziehen`] vom Arbeitsfaden
    /// und [`Self::jetzt_oeffnen`] vom rufenden; zwei Uebergaenge nebeneinander
    /// waeren zwei Wahrheiten darueber, was ein geoeffneter Editor haelt, und
    /// der Umstieg auf den Arbeitsfaden wechselt so nur den Aufrufer.
    ///
    /// Bei Erfolg steht danach die neue Datei mit ihrem Stand, ihrem Typ, ihrem
    /// Stempel und ohne Abweichung; ein Suchlauf ueber den alten Stand ist
    /// beendet, weil seine Versaetze in den neuen nicht mehr passen.
    fn uebernehmen(&mut self, pfad: PathBuf, geladen: Geladen) -> Ladeausgang {
        match geladen.ergebnis {
            Ok(stand) => {
                self.typ = Dateityp::von_pfad(&pfad);
                self.pfad = Some(pfad);
                self.stand = stand;
                self.abweichung = false;
                self.stempel = geladen.stempel;
                self.suchlauf = None;
                Ladeausgang::Geoeffnet
            }
            // Der bisherige Stand bleibt vollstaendig stehen: der Editor wirft
            // nichts weg, weil eine andere Datei sich nicht oeffnen liess.
            Err(abweisung) => Ladeausgang::Abgewiesen(abweisung),
        }
    }

    /// Ob ein Ladevorgang laeuft.
    pub fn laedt_noch(&self) -> bool {
        self.ladevorgang.is_some()
    }

    /// Holt die wartende Meldung des Arbeitsfadens ab.
    ///
    /// Liefert `None`, solange keine da ist oder gar kein Vorgang laeuft; nur
    /// bei `Some` hat die Ansicht etwas zu tun.
    ///
    /// Bei [`Ladeausgang::Geoeffnet`] steht danach die neue Datei mit ihrem
    /// Stand, ihrem Typ, ihrem Stempel und ohne Abweichung; ein Suchlauf ueber
    /// den alten Stand ist beendet, weil seine Versaetze in den neuen nicht
    /// mehr passen.
    pub fn einziehen(&mut self) -> Option<Ladeausgang> {
        let vorgang = self.ladevorgang.as_ref()?;
        let geladener_pfad = vorgang.pfad.clone();
        match vorgang.empfaenger.try_recv() {
            Ok(geladen) => {
                self.ladevorgang = None;
                Some(self.uebernehmen(geladener_pfad, geladen))
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            // Der Faden ist ohne Meldung gefallen; darauf zu warten hat keinen
            // Sinn mehr. Derselbe Zweig und derselbe Grund wie in
            // `vorschaumodell::Vorschaumodell::einziehen`.
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.ladevorgang = None;
                None
            }
        }
    }

    /// Nimmt den bearbeiteten Stand aus der Textflaeche entgegen (C4).
    ///
    /// Setzt die Abweichungsmarke, immer und ohne Vergleich; der Grund und der
    /// Preis stehen im Modulkopf.
    ///
    /// **Beendet einen laufenden Suchlauf.** Seine Versaetze zeigen in den
    /// Stand, aus dem sie gebildet wurden; im neuen koennen sie mitten in einer
    /// Mehrbytefolge oder hinter dem Ende liegen, und beides endet in Rust in
    /// einer Panik. Die beiden Ersetzungswege bilden die Liste stattdessen neu
    /// und sind deshalb die einzigen, auf denen ein Suchlauf eine Aenderung
    /// ueberlebt.
    ///
    /// **Der groessere der beiden Eingaenge fuer fremden Text.** Der Stand
    /// kommt aus einer `NSTextView`, die eingefuegten Text zeichengetreu
    /// aufbewahrt, also mitsamt einem `\r\n`, das aus einer Windows-Quelle
    /// hineinkopiert wurde. Gewandelt wird ueber
    /// `krk_core::text::datei::in_gehaltene_form`, die eine Stelle des
    /// Programms, die das tut; siehe den Modulkopf. Ein Stand, der die Form
    /// schon hat, kommt ohne eine einzige Kopie zurueck und kostet einen
    /// Durchlauf.
    pub fn bearbeiten(&mut self, neuer_stand: String) {
        self.stand = datei::in_gehaltene_form(neuer_stand);
        self.abweichung = true;
        self.suchlauf = None;
    }

    /// Schreibt den Stand in die gehaltene Datei (C4).
    ///
    /// Geschrieben wird ueber `krk_core::text::datei::sichern`, die eine
    /// Stelle, die die Sicherungsform herstellt: Unix-Zeilenenden, ein
    /// abschliessender Umbruch, keine Bytefolgenmarke. Dieses Modul stellt
    /// nichts davon selbst her.
    ///
    /// Nach einem gelungenen Sichern meldet der Editor keinen ungesicherten
    /// Stand mehr, und der Stempel steht auf der eben geschriebenen Datei;
    /// damit gilt sie nicht als von aussen geaendert. Nach einem gescheiterten
    /// bleibt beides, wie es war.
    pub fn sichern(&mut self) -> Sicherungsausgang {
        let Some(pfad) = self.pfad.as_ref() else {
            return Sicherungsausgang::NichtsGehalten;
        };
        match datei::sichern(pfad, &self.stand) {
            Ok(()) => {
                self.abweichung = false;
                self.stempel = Stempel::von_pfad(pfad);
                Sicherungsausgang::Gesichert
            }
            Err(fehler) => Sicherungsausgang::Gescheitert(format!(
                "{} ließ sich nicht sichern: {fehler}",
                pfad.display()
            )),
        }
    }

    /// Gibt die gehaltene Datei auf (C1, C4).
    ///
    /// Ein ungesicherter Stand faellt dabei. Die Nachfrage davor gehoert dem
    /// Aufrufer; siehe den Modulkopf.
    pub fn schliessen(&mut self) {
        self.pfad = None;
        self.stand.clear();
        self.abweichung = false;
        self.typ = Dateityp::default();
        self.suchlauf = None;
        self.stempel = None;
        self.ladevorgang = None;
    }

    /// Ob die gehaltene Datei sich seit dem Oeffnen oder Sichern geaendert hat
    /// (C4).
    ///
    /// Kostet einen `stat(2)` und liest die Datei nicht. Eine Datei, die
    /// verschwunden oder unlesbar geworden ist, gilt als geaendert: auch das
    /// ist eine Aenderung von aussen, ueber die C4 den Nutzer nicht im Unklaren
    /// lassen will. Haelt der Editor keine Datei, ist die Antwort `false`.
    pub fn fremd_geaendert(&self) -> bool {
        let (Some(pfad), Some(gemerkt)) = (self.pfad.as_ref(), self.stempel) else {
            return false;
        };
        Stempel::von_pfad(pfad) != Some(gemerkt)
    }

    /// Beginnt eine Suche im gehaltenen Stand und steuert den ersten Treffer an
    /// (C5).
    ///
    /// `ab_versatz` ist die Stelle der Schreibmarke. Angesteuert wird der erste
    /// Treffer, der dort oder dahinter beginnt, und hinter dem letzten laeuft
    /// die Suche um; die Regel dafuer steht in `krk_core::text::suche` und wird
    /// hier nicht nachgebaut. Ein leerer Suchtext liefert keinen Treffer.
    ///
    /// Gesucht wird ueber den **gehaltenen Stand** und nicht ueber die Datei
    /// auf der Platte; das neunte Abnahmekriterium von C5 verlangt es, und es
    /// faellt von selbst an, weil `suche::alle` einen Pfad gar nicht
    /// entgegennehmen kann.
    pub fn suche_starten(&mut self, gesucht: &str, ab_versatz: usize) -> Option<Treffer> {
        let treffer = suche::alle(&self.stand, gesucht);
        let angesteuert = suche::erster_ab(&treffer, ab_versatz);
        self.suchlauf = Some(Suchlauf {
            gesucht: gesucht.to_owned(),
            treffer,
            angesteuert,
        });
        self.suchlauf.as_ref().and_then(Suchlauf::angesteuert)
    }

    /// Steuert den naechsten Treffer an und laeuft hinter dem letzten um (C5).
    ///
    /// Ohne laufenden Suchlauf und ohne Treffer `None`; die Schreibmarke bleibt
    /// dann stehen, wie das fuenfte Abnahmekriterium von C5 es verlangt.
    pub fn weitersuchen(&mut self) -> Option<Treffer> {
        self.weiter_mit(suche::naechster)
    }

    /// Steuert den vorigen Treffer an und laeuft vor dem ersten um (C5).
    pub fn rueckwaerts_suchen(&mut self) -> Option<Treffer> {
        self.weiter_mit(suche::voriger)
    }

    /// Die gemeinsame Haelfte von [`Self::weitersuchen`] und
    /// [`Self::rueckwaerts_suchen`].
    ///
    /// Beide unterscheiden sich allein in der Auswahlfunktion aus
    /// `krk_core::text::suche`; der Umlauf steckt dort und nicht hier.
    ///
    /// Steht noch kein Treffer an, weil ein Ersetzen den Durchgang beendet hat,
    /// wird vom Textanfang aus weitergegangen. Das ist die einzige Stelle, an
    /// der dieses Modul einen Versatz waehlt, und sie waehlt den einzigen, der
    /// in jedem Stand gueltig ist.
    fn weiter_mit(&mut self, auswahl: fn(&[Treffer], usize) -> Option<usize>) -> Option<Treffer> {
        let lauf = self.suchlauf.as_mut()?;
        let versatz = lauf
            .angesteuert
            .map_or(0, |stelle| lauf.treffer[stelle].anfang);
        lauf.angesteuert = auswahl(&lauf.treffer, versatz);
        lauf.angesteuert()
    }

    /// Beendet den Suchlauf (C5).
    pub fn suche_beenden(&mut self) {
        self.suchlauf = None;
    }

    /// Was beide Ersetzungswege brauchen, bevor sie `krk_core::text::suche`
    /// rufen: den Suchtext des laufenden Suchlaufs und den Ersatztext in der
    /// gehaltenen Form.
    ///
    /// **Der kleinere der beiden Eingaenge fuer fremden Text**, und die eine
    /// Stelle, an der ein Ersatztext ihn nimmt. Er kommt aus einem
    /// Eingabefeld und traegt ein `\r`, wenn er dort hineinkopiert wurde;
    /// gewandelt wird ueber `krk_core::text::datei::in_gehaltene_form` und
    /// nicht mit einer eigenen Wandlung daneben.
    ///
    /// **Vor dem Ersetzen und nicht danach**, denn `suche::einen_ersetzen`
    /// nennt den naechsten Treffer als Byteversatz in den Stand, den es
    /// gebildet hat. Der Grund im Einzelnen steht im Modulkopf.
    ///
    /// `None` heisst: es laeuft keine Suche, und dann ist nichts zu ersetzen.
    fn ersetzung_vorbereiten(&self, ersatz: &str) -> Option<(String, String)> {
        let lauf = self.suchlauf.as_ref()?;
        Some((
            lauf.gesucht.clone(),
            datei::in_gehaltene_form(ersatz.to_owned()),
        ))
    }

    /// Ersetzt den angesteuerten Treffer und steuert den naechsten an (C5).
    ///
    /// Liefert den naechsten Treffer im **neuen** Stand, oder `None`, wenn der
    /// Durchgang zu Ende ist. Das Ersetzen ist eine ungesicherte Aenderung im
    /// Sinne von C4 und schreibt nichts in die Datei; das achte
    /// Abnahmekriterium von C5 verlangt beides.
    ///
    /// Ohne laufenden Suchlauf und ohne angesteuerten Treffer geschieht nichts.
    pub fn treffer_ersetzen(&mut self, ersatz: &str) -> Option<Treffer> {
        let angesteuert = self.suchlauf.as_ref()?.angesteuert()?;
        let (gesucht, ersatz) = self.ersetzung_vorbereiten(ersatz)?;

        let ersetzung = suche::einen_ersetzen(&self.stand, &gesucht, &ersatz, angesteuert);
        self.stand = ersetzung.stand;
        self.abweichung = true;

        // Die Trefferliste wird im neuen Stand neu gebildet, statt die alte
        // fortzuschreiben: der Ersatztext kann den Suchtext enthalten, und dann
        // stimmt weder die Zahl noch die Lage. `einen_ersetzen` nennt den
        // naechsten Treffer; seine Stelle in der neuen Liste ist die, die er
        // dort hat.
        let treffer = suche::alle(&self.stand, &gesucht);
        let angesteuert = ersetzung
            .naechster
            .and_then(|naechster| treffer.iter().position(|kandidat| *kandidat == naechster));
        self.suchlauf = Some(Suchlauf {
            gesucht,
            treffer,
            angesteuert,
        });
        self.suchlauf.as_ref().and_then(Suchlauf::angesteuert)
    }

    /// Ersetzt alle Treffer in einem Zug und nennt ihre Zahl (C5).
    ///
    /// Danach steht kein Treffer mehr an; die Trefferliste ist die des neuen
    /// Standes und in aller Regel leer. Sie wird trotzdem gebildet, weil der
    /// Ersatztext den Suchtext enthalten kann und die Zahl in der Statuszeile
    /// dann nicht die Zahl der verbliebenen Treffer waere.
    ///
    /// Ohne laufenden Suchlauf geschieht nichts, und die Zahl ist 0.
    pub fn alle_treffer_ersetzen(&mut self, ersatz: &str) -> usize {
        let Some((gesucht, ersatz)) = self.ersetzung_vorbereiten(ersatz) else {
            return 0;
        };

        let ersetzung = suche::alle_ersetzen(&self.stand, &gesucht, &ersatz);
        if ersetzung.zahl == 0 {
            return 0;
        }
        self.stand = ersetzung.stand;
        self.abweichung = true;
        self.suchlauf = Some(Suchlauf {
            treffer: suche::alle(&self.stand, &gesucht),
            gesucht,
            angesteuert: None,
        });
        ersetzung.zahl
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static ZAEHLER: AtomicU64 = AtomicU64::new(0);

    /// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
    ///
    /// Dieselbe Form wie `Pruefordner` in [`crate::leistenmodell`],
    /// `krk-core/tests/verzeichnis.rs` und `krk-bench/src/fixture.rs`: Zweck,
    /// Prozesskennung und Laufnummer im Namen, und das Abraeumen in `Drop`.
    struct Pruefordner {
        pfad: PathBuf,
    }

    impl Pruefordner {
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let pfad = std::env::temp_dir().join(format!(
                "krk-editor-test-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&pfad);
            std::fs::create_dir_all(&pfad).expect("der Pruefordner laesst sich nicht anlegen");
            Self { pfad }
        }

        /// Legt eine Datei mit dem genannten Inhalt an und liefert ihren Pfad.
        fn datei(&self, name: &str, inhalt: &str) -> PathBuf {
            let pfad = self.pfad.join(name);
            std::fs::write(&pfad, inhalt).expect("die Pruefdatei laesst sich nicht schreiben");
            pfad
        }
    }

    impl Drop for Pruefordner {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.pfad);
        }
    }

    /// Wartet, bis der Arbeitsfaden geliefert hat.
    ///
    /// Die Schranke ist grosszuegig und dient allein dazu, dass ein Fehlschlag
    /// als Fehlschlag endet und nicht als haengende Probe.
    fn abwarten(modell: &mut Editormodell) -> Ladeausgang {
        for _ in 0..2000 {
            if let Some(ausgang) = modell.einziehen() {
                return ausgang;
            }
            thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("der Editor-Arbeitsfaden hat innerhalb von zwei Sekunden nichts geliefert");
    }

    fn geoeffnet(pfad: &Path) -> Editormodell {
        let mut modell = Editormodell::neu();
        modell.oeffnen(pfad);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        modell
    }

    #[test]
    fn ein_neuer_editor_haelt_nichts() {
        let modell = Editormodell::neu();
        assert!(!modell.haelt_datei());
        assert_eq!(modell.pfad(), None);
        assert_eq!(modell.stand(), "");
        assert!(!modell.hat_ungesicherten_stand());
        assert!(!modell.laedt_noch());
        assert_eq!(modell.ansicht(), Ansicht::Format);
    }

    /// Das erste Abnahmekriterium des Schrittes, in einem Zug: frisch geoeffnet
    /// keine Abweichung, nach einer Aenderung eine, nach dem Sichern wieder
    /// keine.
    #[test]
    fn die_abweichung_kommt_mit_der_aenderung_und_geht_mit_dem_sichern() {
        let ordner = Pruefordner::neu("abweichung");
        let pfad = ordner.datei("stand.txt", "erste Zeile\n");
        let mut modell = geoeffnet(&pfad);

        assert!(
            !modell.hat_ungesicherten_stand(),
            "frisch geoeffnet weicht nichts ab"
        );
        assert_eq!(modell.stand(), "erste Zeile\n");

        modell.bearbeiten("erste Zeile\nzweite Zeile\n".to_owned());
        assert!(modell.hat_ungesicherten_stand());

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert);
        assert!(
            !modell.hat_ungesicherten_stand(),
            "C4: nach dem Sichern meldet der Editor keine ungesicherten Aenderungen mehr"
        );
        assert_eq!(
            std::fs::read_to_string(&pfad).expect("die Datei ist nach dem Sichern lesbar"),
            "erste Zeile\nzweite Zeile\n"
        );
    }

    /// Das zweite Abnahmekriterium des Schrittes: ein zweiter Ladevorgang laesst
    /// den ersten verfallen.
    ///
    /// Die Probe haengt nicht an einer Wettlage. Der erste Empfaenger faellt in
    /// dem Augenblick, in dem `oeffnen` den zweiten Vorgang einsetzt; danach
    /// **kann** die Meldung des ersten Fadens nicht mehr ankommen, gleichgueltig
    /// wie schnell er war. Geprueft wird die Folge davon: der Stand ist der der
    /// zweiten Datei, und die erste hat ihn zu keinem Zeitpunkt beruehrt.
    #[test]
    fn ein_zweiter_ladevorgang_laesst_den_ersten_verfallen() {
        let ordner = Pruefordner::neu("zwei-ladevorgaenge");
        let erste = ordner.datei("erste.txt", "Inhalt der ersten Datei\n");
        let zweite = ordner.datei("zweite.txt", "Inhalt der zweiten Datei\n");

        let mut modell = Editormodell::neu();
        modell.oeffnen(&erste);
        modell.oeffnen(&zweite);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);

        assert_eq!(modell.pfad(), Some(zweite.as_path()));
        assert_eq!(modell.stand(), "Inhalt der zweiten Datei\n");
        assert!(
            !modell.laedt_noch(),
            "es steht kein zweiter Vorgang mehr aus"
        );
        assert_eq!(
            modell.einziehen(),
            None,
            "die Meldung des ersten Fadens kommt nicht nach"
        );
    }

    /// Das zehnte Abnahmekriterium von C3: der Wechsel zwischen den Ansichten
    /// verliert keine ungesicherte Aenderung.
    ///
    /// Geprueft wird an dem, was den Verlust ausmachen wuerde: Stand,
    /// Abweichungsmarke und gehaltene Datei vor und nach zwei Wechseln. Dass es
    /// keine zweite Kopie gibt, in die etwas verlorengehen koennte, ist eine
    /// Eigenschaft des Typs — `Editormodell` traegt genau ein `String`-Feld.
    #[test]
    fn ein_ansichtswechsel_verliert_keinen_ungesicherten_stand() {
        let ordner = Pruefordner::neu("ansichtswechsel");
        let pfad = ordner.datei("stand.md", "# Ueberschrift\n");
        let mut modell = geoeffnet(&pfad);

        modell.bearbeiten("# Ueberschrift\n\nein ungesicherter Absatz\n".to_owned());
        let vorher = modell.stand().to_owned();

        assert_eq!(modell.ansicht_umschalten(), Ansicht::Roh);
        assert_eq!(modell.stand(), vorher, "der Stand steht in der Rohansicht");
        assert!(
            modell.hat_ungesicherten_stand(),
            "die Abweichung ueberlebt den Wechsel"
        );

        assert_eq!(modell.ansicht_umschalten(), Ansicht::Format);
        assert_eq!(
            modell.stand(),
            vorher,
            "und steht nach der Rueckkehr unveraendert da"
        );
        assert!(modell.hat_ungesicherten_stand());
        assert_eq!(modell.pfad(), Some(pfad.as_path()));
    }

    /// C3: die Ansichtswahl ueberlebt einen Dateiwechsel.
    #[test]
    fn die_ansichtswahl_bleibt_ueber_einen_dateiwechsel_stehen() {
        let ordner = Pruefordner::neu("ansicht-bleibt");
        let erste = ordner.datei("erste.md", "# eins\n");
        let zweite = ordner.datei("zweite.rs", "fn zwei() {}\n");

        let mut modell = geoeffnet(&erste);
        assert_eq!(modell.typ(), Dateityp::Markdown);
        modell.ansicht_umschalten();
        assert_eq!(modell.ansicht(), Ansicht::Roh);

        modell.oeffnen(&zweite);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(
            modell.ansicht(),
            Ansicht::Roh,
            "C3: die Wahl gehoert nicht der Datei"
        );
        assert_eq!(modell.typ(), Dateityp::Sonstiges);
    }

    /// C2: eine abgewiesene Datei laesst den gehaltenen Stand stehen.
    #[test]
    fn eine_abgewiesene_datei_wirft_den_gehaltenen_stand_nicht_weg() {
        let ordner = Pruefordner::neu("abweisung");
        let gute = ordner.datei("gut.txt", "guter Inhalt\n");
        let mut modell = geoeffnet(&gute);
        modell.bearbeiten("guter Inhalt, bearbeitet\n".to_owned());

        // Ein Ordner ist der Fall, den die Pruefung namentlich abweist.
        modell.oeffnen(&ordner.pfad);
        let ausgang = abwarten(&mut modell);
        assert!(
            matches!(ausgang, Ladeausgang::Abgewiesen(_)),
            "ein Ordner laesst sich nicht im Editor oeffnen, {ausgang:?}"
        );
        assert_eq!(modell.pfad(), Some(gute.as_path()));
        assert_eq!(modell.stand(), "guter Inhalt, bearbeitet\n");
        assert!(modell.hat_ungesicherten_stand());
    }

    /// C2: die Pruefung steht vor dem Aufnehmen, und der Editor nimmt eine
    /// Datei ueber der Grenze nicht auf.
    ///
    /// Die Reihenfolge aus dem elften Abnahmekriterium von C2, auf dem Weg, den
    /// F4 seit S22 geht. Die Pruefdatei bekommt ihre Groesse ueber `set_len`
    /// und nicht ueber 16 MB geschriebener Bytes: entschieden wird an der
    /// Groesse aus `stat(2)`, und genau die steht danach da. Dass die Datei
    /// dabei gar nicht erst gelesen wird, ist der Punkt des sechsten
    /// Abnahmekriteriums.
    #[test]
    fn eine_datei_ueber_der_grenze_wird_gestellt_und_nicht_aufgenommen() {
        let ordner = Pruefordner::neu("zu-gross");
        let gute = ordner.datei("gut.txt", "guter Inhalt\n");
        let mut modell = Editormodell::neu();
        assert_eq!(modell.jetzt_oeffnen(&gute), Ladeausgang::Geoeffnet);

        let zu_gross = ordner.pfad.join("zu-gross.txt");
        std::fs::File::create(&zu_gross)
            .expect("die Pruefdatei laesst sich nicht anlegen")
            .set_len(datei::EDITORGRENZE + 1)
            .expect("die Pruefdatei laesst sich nicht auf Groesse bringen");

        let ausgang = modell.jetzt_oeffnen(&zu_gross);
        assert!(
            matches!(ausgang, Ladeausgang::Abgewiesen(Abweisung::ZuGross { .. })),
            "eine Datei ueber der Grenze wurde nicht als zu gross abgewiesen: {ausgang:?}"
        );
        assert_eq!(
            modell.pfad(),
            Some(gute.as_path()),
            "der Editor hat die abgewiesene Datei aufgenommen"
        );
        assert_eq!(modell.stand(), "guter Inhalt\n");
    }

    /// Der Verlust vom 260809-2029, an der Stelle nachgestellt, an der er
    /// entsteht.
    ///
    /// Der Weg des Nutzers war: F4 auf eine Datei, tippen, die Vorschau
    /// einblenden — was den Editor nach C1 verdraengt —, und F4 auf dieselbe
    /// Datei, um ihn zurueckzuholen. Das zweite F4 war bis zum 260809 ein
    /// vollwertiges Oeffnen und las die Datei neu; danach stand der
    /// Plattenstand im Modell, die Abweichungsmarke war geloescht, und
    /// `Editorbereich::stand_einsetzen` schrieb den Plattenstand ueber das
    /// Getippte.
    ///
    /// Nachgestellt wird der zweite Ruf und nicht der Weg dorthin: die
    /// Sichtbarkeit der Bereiche ist an dem Verlust unbeteiligt (sie setzt
    /// `hidden` und faellt keinen Stand), und die Textflaeche braucht ein
    /// Fenster. Was hier faellt, ist der Stand des Modells — und er ist es, den
    /// die Ansicht in die Flaeche traegt.
    #[test]
    fn ein_zweites_oeffnen_derselben_datei_wirft_den_bearbeiteten_stand_nicht_weg() {
        let ordner = Pruefordner::neu("zweimal-dieselbe");
        let pfad = ordner.datei("stand.txt", "auf der Platte\n");
        let mut modell = geoeffnet(&pfad);

        modell.bearbeiten("auf der Platte\nund ungesichert getippt\n".to_owned());
        let stempel_vorher = modell.stempel();

        let ausgang = modell.jetzt_oeffnen(&pfad);

        // Zuerst der Verlust selbst, damit ein Rueckfall ihn und nicht eine
        // Nebensache meldet.
        assert_eq!(
            modell.stand(),
            "auf der Platte\nund ungesichert getippt\n",
            "260809-2029: das zweite F4 wirft den ungesicherten Stand nicht weg"
        );
        assert_eq!(
            ausgang,
            Ladeausgang::SchonOffen,
            "die schon gehaltene Datei wird nicht ein zweites Mal gelesen"
        );
        assert!(
            modell.hat_ungesicherten_stand(),
            "die Abweichungsmarke ueberlebt den zweiten Ruf"
        );
        assert_eq!(modell.pfad(), Some(pfad.as_path()));
        assert_eq!(
            modell.stempel(),
            stempel_vorher,
            "ohne Lesevorgang bewegt sich auch der Stempel nicht"
        );
        assert!(!modell.laedt_noch(), "es wurde kein Ladevorgang gestartet");
    }

    /// Die Abkuerzung greift fuer diese eine Datei und nicht fuer die naechste.
    ///
    /// Der Wechsel auf eine **andere** Datei bleibt der zweite Anlass aus C4
    /// und faellt weiterhin ohne Rueckfrage; die Nachfrage baut S28. Die Probe
    /// haelt fest, dass die Abkuerzung ihn nicht stillschweigend mitnimmt.
    #[test]
    fn eine_andere_datei_wird_weiterhin_gelesen() {
        let ordner = Pruefordner::neu("andere-datei");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);

        assert!(modell.haelt_bereits(&erste));
        assert!(!modell.haelt_bereits(&zweite));

        assert_eq!(modell.jetzt_oeffnen(&zweite), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), "zweite\n");
        assert_eq!(modell.pfad(), Some(zweite.as_path()));
    }

    /// Beide Lesewege hinterlassen denselben Stand.
    ///
    /// Der Arbeitsfaden und der rufende Faden gehen durch dieselbe
    /// [`Editormodell::uebernehmen`]; die Probe haelt fest, dass der Umstieg
    /// auf den Arbeitsfaden nur den Aufrufer wechselt und nicht das Ergebnis.
    #[test]
    fn der_sofortige_weg_und_der_arbeitsfaden_hinterlassen_denselben_stand() {
        let ordner = Pruefordner::neu("zwei-wege");
        let pfad = ordner.datei("stand.txt", "eine Zeile\n");

        let ueber_den_faden = geoeffnet(&pfad);
        let mut sofort = Editormodell::neu();
        assert_eq!(sofort.jetzt_oeffnen(&pfad), Ladeausgang::Geoeffnet);

        assert_eq!(sofort.pfad(), ueber_den_faden.pfad());
        assert_eq!(sofort.stand(), ueber_den_faden.stand());
        assert_eq!(sofort.typ(), ueber_den_faden.typ());
        assert_eq!(sofort.stempel(), ueber_den_faden.stempel());
        assert!(!sofort.hat_ungesicherten_stand());
        assert!(!sofort.laedt_noch());
    }

    /// C4: ein gescheitertes Sichern nennt den Grund und wirft den Stand nicht
    /// weg.
    #[test]
    fn ein_gescheitertes_sichern_laesst_den_stand_stehen() {
        let ordner = Pruefordner::neu("sichern-scheitert");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        modell.bearbeiten("neuer Inhalt\n".to_owned());

        // Der Ordner ist nach dem Oeffnen fort; das Schreiben kann nicht
        // gelingen, und der Pfad ist derselbe geblieben.
        std::fs::remove_dir_all(&ordner.pfad).expect("der Pruefordner laesst sich raeumen");

        let ausgang = modell.sichern();
        match ausgang {
            Sicherungsausgang::Gescheitert(grund) => assert!(
                grund.contains("ließ sich nicht sichern"),
                "der Grund gehoert in die Statuszeile: {grund}"
            ),
            sonst => panic!("das Sichern haette scheitern muessen, {sonst:?}"),
        }
        assert_eq!(modell.stand(), "neuer Inhalt\n");
        assert!(
            modell.hat_ungesicherten_stand(),
            "C4: der Stand wird nicht weggeworfen"
        );
    }

    #[test]
    fn ein_editor_ohne_datei_hat_nichts_zu_sichern() {
        let mut modell = Editormodell::neu();
        assert_eq!(modell.sichern(), Sicherungsausgang::NichtsGehalten);
    }

    /// C4: der Stempel steht nach dem Oeffnen und nach dem Sichern auf der
    /// Datei, wie sie auf der Platte liegt.
    #[test]
    fn der_stempel_kennt_eine_aenderung_von_aussen() {
        let ordner = Pruefordner::neu("stempel");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        assert!(modell.stempel().is_some());
        assert!(!modell.fremd_geaendert());

        std::fs::write(&pfad, "von aussen geaendert\n").expect("die Datei laesst sich schreiben");
        assert!(
            modell.fremd_geaendert(),
            "C4: eine Aenderung von aussen wird bemerkt"
        );

        // Das eigene Sichern ist keine Aenderung von aussen: es zieht den
        // Stempel mit.
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert);
        assert!(!modell.fremd_geaendert());
    }

    #[test]
    fn eine_verschwundene_datei_gilt_als_geaendert() {
        let ordner = Pruefordner::neu("stempel-fort");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let modell = geoeffnet(&pfad);
        std::fs::remove_file(&pfad).expect("die Datei laesst sich loeschen");
        assert!(modell.fremd_geaendert());
    }

    #[test]
    fn das_schliessen_gibt_die_datei_auf() {
        let ordner = Pruefordner::neu("schliessen");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        modell.bearbeiten("bearbeitet\n".to_owned());

        modell.schliessen();
        assert!(!modell.haelt_datei());
        assert_eq!(modell.stand(), "");
        assert!(!modell.hat_ungesicherten_stand());
        assert!(modell.suchlauf().is_none());
    }

    /// C5: die Suche zaehlt und laeuft um, und sie geht ueber den gehaltenen
    /// Stand und nicht ueber die Datei.
    #[test]
    fn die_suche_zaehlt_und_laeuft_um() {
        let ordner = Pruefordner::neu("suche");
        let pfad = ordner.datei("stand.txt", "eins zwei eins\n");
        let mut modell = geoeffnet(&pfad);
        // Was der Nutzer eben getippt und noch nicht gesichert hat, wird
        // gefunden: der dritte Treffer steht nur im gehaltenen Stand.
        modell.bearbeiten("eins zwei eins drei eins\n".to_owned());

        let erster = modell
            .suche_starten("eins", 0)
            .expect("drei Treffer stehen im Stand");
        assert_eq!(erster.anfang, 0);
        let lauf = modell.suchlauf().expect("der Suchlauf steht");
        assert_eq!(lauf.gesucht(), "eins");
        assert_eq!(lauf.zahl(), 3);
        assert_eq!(
            lauf.treffer().iter().map(|t| t.anfang).collect::<Vec<_>>(),
            vec![0, 10, 20]
        );
        assert_eq!(lauf.nummer(), Some(1));
        assert_eq!(lauf.meldung(), "Treffer 1 von 3");

        assert_eq!(modell.weitersuchen().map(|t| t.anfang), Some(10));
        assert_eq!(modell.weitersuchen().map(|t| t.anfang), Some(20));
        assert_eq!(
            modell.weitersuchen().map(|t| t.anfang),
            Some(0),
            "C5: hinter dem letzten geht es beim ersten weiter"
        );
        assert_eq!(
            modell.rueckwaerts_suchen().map(|t| t.anfang),
            Some(20),
            "C5: vor dem ersten geht es beim letzten weiter"
        );

        modell.suche_beenden();
        assert!(modell.suchlauf().is_none());
        assert_eq!(modell.weitersuchen(), None);
    }

    #[test]
    fn eine_suche_ohne_treffer_meldet_das_und_steuert_nichts_an() {
        let ordner = Pruefordner::neu("suche-leer");
        let pfad = ordner.datei("stand.txt", "eins zwei\n");
        let mut modell = geoeffnet(&pfad);

        assert_eq!(modell.suche_starten("drei", 0), None);
        let lauf = modell
            .suchlauf()
            .expect("der Suchlauf steht auch ohne Treffer");
        assert_eq!(lauf.zahl(), 0);
        assert_eq!(lauf.nummer(), None);
        assert_eq!(lauf.meldung(), "Kein Treffer für „drei“");
    }

    /// C5: ein Ersetzen ist eine ungesicherte Aenderung und schreibt nichts.
    #[test]
    fn das_ersetzen_aendert_den_stand_und_nicht_die_datei() {
        let ordner = Pruefordner::neu("ersetzen");
        let pfad = ordner.datei("stand.txt", "eins zwei eins\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("eins", 0);
        let naechster = modell.treffer_ersetzen("drei");
        assert_eq!(modell.stand(), "drei zwei eins\n");
        assert_eq!(
            naechster.map(|t| t.anfang),
            Some(10),
            "der naechste Treffer steht im neuen Stand"
        );
        assert!(modell.hat_ungesicherten_stand());
        assert_eq!(
            std::fs::read_to_string(&pfad).expect("die Datei ist lesbar"),
            "eins zwei eins\n",
            "C5: das Ersetzen schreibt nicht von sich aus in die Datei"
        );
    }

    /// Der Ersatztext enthaelt den Suchtext: die neu gebildete Trefferliste
    /// zaehlt richtig, und der Durchgang laeuft nicht in das eben Eingesetzte
    /// zurueck.
    #[test]
    fn das_ersetzen_bildet_die_trefferliste_im_neuen_stand() {
        let ordner = Pruefordner::neu("ersetzen-selbstbezug");
        let pfad = ordner.datei("stand.txt", "foo bar foo\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("foo", 0);
        modell.treffer_ersetzen("foofoo");
        assert_eq!(modell.stand(), "foofoo bar foo\n");
        let lauf = modell.suchlauf().expect("der Suchlauf steht");
        assert_eq!(lauf.zahl(), 3, "zwei aus dem Ersatz und der unberuehrte");
        assert_eq!(
            lauf.angesteuert().map(|t| t.anfang),
            Some(11),
            "angesteuert ist der unberuehrte und nicht das eben Eingesetzte"
        );
    }

    #[test]
    fn das_sammelersetzen_nennt_die_zahl() {
        let ordner = Pruefordner::neu("alle-ersetzen");
        let pfad = ordner.datei("stand.txt", "a b a b a\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("a", 0);
        assert_eq!(modell.alle_treffer_ersetzen("x"), 3);
        assert_eq!(modell.stand(), "x b x b x\n");
        assert!(modell.hat_ungesicherten_stand());
        let lauf = modell.suchlauf().expect("der Suchlauf steht");
        assert_eq!(lauf.zahl(), 0);
        assert_eq!(lauf.meldung(), "Kein Treffer für „a“");
    }

    /// Die Entscheidung des Nutzers vom 260808-0043, an dem Ende gemessen, an
    /// dem sie zaehlt: KRK schreibt beim Sichern immer Unix-Zeilenenden.
    ///
    /// Eine `NSTextView` bewahrt eingefuegten Text zeichengetreu auf, also
    /// steht nach einem Einfuegen aus einer Windows-Quelle ein `\r\n` in dem
    /// Stand, den sie zurueckgibt. Geprueft wird nicht die Wandlung selbst,
    /// sondern die Datei auf der Platte: `sicherungsform` wandelt bewusst
    /// keine Zeilenenden, also faende ein `\r\n` von hier aus jeden Weg
    /// hinaus.
    #[test]
    fn ein_eingefuegtes_crlf_landet_nicht_auf_der_platte() {
        let ordner = Pruefordner::neu("crlf-sichern");
        let pfad = ordner.datei("stand.txt", "erste Zeile\n");
        let mut modell = geoeffnet(&pfad);

        modell.bearbeiten("aus Windows\r\neingefügt\r\nletzte".to_owned());
        assert_eq!(
            modell.stand(),
            "aus Windows\neingefügt\nletzte",
            "der gehaltene Stand traegt `\\n` als einziges Zeilenende"
        );

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert);
        let auf_der_platte =
            std::fs::read_to_string(&pfad).expect("die Datei ist nach dem Sichern lesbar");
        assert!(
            !auf_der_platte.contains('\r'),
            "260808-0043: beim Sichern gehen Unix-Zeilenenden hinaus, {auf_der_platte:?}"
        );
        assert_eq!(auf_der_platte, "aus Windows\neingefügt\nletzte\n");
    }

    /// Der Ersatztext wird **vor** dem Ersetzen gewandelt, nicht der Stand
    /// danach.
    ///
    /// Die beiden Zusicherungen unten trennen die richtige Reihenfolge von der
    /// falschen. Der gewandelte Ersatztext ist drei Bytes lang, der
    /// ungewandelte vier; der naechste Treffer steht deshalb auf 9, wenn vorher
    /// gewandelt wurde, und auf 10, wenn `einen_ersetzen` den rohen Text bekam.
    /// Im zweiten Fall faende die neu gebildete Trefferliste die 10 nicht, und
    /// `treffer_ersetzen` lieferte `None` — der Durchgang bliebe kommentarlos
    /// stehen.
    #[test]
    fn ein_ersatztext_mit_crlf_kommt_in_gehaltener_form_an() {
        let ordner = Pruefordner::neu("crlf-ersatz");
        let pfad = ordner.datei("stand.txt", "eins zwei eins\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("eins", 0);
        let naechster = modell.treffer_ersetzen("A\r\nB");
        assert_eq!(modell.stand(), "A\nB zwei eins\n");
        assert_eq!(
            naechster.map(|t| t.anfang),
            Some(9),
            "der Durchgang steuert den unberuehrten Treffer im gewandelten Stand an"
        );
    }

    #[test]
    fn das_sammelersetzen_wandelt_seinen_ersatztext_ebenfalls() {
        let ordner = Pruefordner::neu("crlf-sammelersatz");
        let pfad = ordner.datei("stand.txt", "a b a\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("a", 0);
        assert_eq!(modell.alle_treffer_ersetzen("x\r\ny"), 2);
        assert_eq!(modell.stand(), "x\ny b x\ny\n");
    }

    /// Ein Suchlauf ueberlebt keine Bearbeitung von aussen: seine Versaetze
    /// zeigen in den alten Stand.
    #[test]
    fn eine_bearbeitung_beendet_den_suchlauf() {
        let ordner = Pruefordner::neu("suchlauf-faellt");
        let pfad = ordner.datei("stand.txt", "eins zwei eins\n");
        let mut modell = geoeffnet(&pfad);

        modell.suche_starten("eins", 0);
        assert!(modell.suchlauf().is_some());
        modell.bearbeiten("kurz\n".to_owned());
        assert!(modell.suchlauf().is_none());
        assert_eq!(modell.weitersuchen(), None);
    }

    #[test]
    fn der_dateityp_kommt_aus_der_endung() {
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/lies.md")),
            Dateityp::Markdown
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/LIES.MARKDOWN")),
            Dateityp::Markdown,
            "die Endung wird ohne Ruecksicht auf Gross- und Kleinschreibung verglichen"
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/quelle.rs")),
            Dateityp::Sonstiges
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/Makefile")),
            Dateityp::Sonstiges,
            "ohne Endung ist die Frage nach Markdown mit Nein beantwortet"
        );
    }

    #[test]
    fn die_beiden_ansichten_sind_die_jeweils_andere() {
        assert_eq!(Ansicht::Roh.andere(), Ansicht::Format);
        assert_eq!(Ansicht::Format.andere(), Ansicht::Roh);
    }
}
