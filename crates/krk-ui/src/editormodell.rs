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
//!   │  stand ──> Stempel gleich? ──> sichern ──> Platte     │
//!   │            sonst: nicht geschrieben                   │
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
//! **Der groessere Eingang sagt, ob er gewandelt hat**, weil die `NSTextView`
//! hinter ihm einen eigenen Textbestand fuehrt und den nicht selbst nachzieht.
//! [`Editormodell::bearbeiten`] liefert deshalb ein `bool`: wandelte es, laufen
//! Stand und Flaeche um die gewandelten Zeichen auseinander, und die Ansicht
//! hat die Flaeche auf den Stand zu bringen. Der Ersatztext braucht das nicht,
//! weil hinter ihm kein Bestand steht, der stehen bliebe. Der Defekt dazu ist
//! `issues/260810-0215_*_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`.
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
//! zweiten Textbestand, in den etwas verlorengehen koennte.
//!
//! Die zweite Haelfte derselben Zusage steht in `crate::appkit::editor` und
//! lautet: **kein Merkmal der Flaeche kann beim Sichern in die Datei geraten.**
//! Sie haengt nicht daran, in welchem der beiden Speicher ein Merkmal liegt,
//! sondern daran, dass [`Editormodell::sichern`] allein [`Editormodell::stand`]
//! schreibt und der aus den **Zeichen** der Flaeche kommt. Warum ein Teil der
//! Auszeichnung im Textspeicher liegen muss und nicht im Layoutverwalter, steht
//! im Modulkopf von `crate::hervorhebung`.
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
//! # Die fremde Aenderung: eine Frage, zwei Momente
//!
//! [`Editormodell::fremd_geaendert`] vergleicht den gemerkten Stempel gegen den
//! der Platte. Zwei Stellen stellen die Frage, und beide dieselbe:
//!
//! ```text
//!  FSEvents meldet den Ordner ──> fremdaenderung_melden ──> Satz, einmal
//!  cmd+s                      ──> sichern               ──> nicht geschrieben
//! ```
//!
//! Das ist **kein zweiter Mechanismus**, sondern eine Frage an zwei Momenten;
//! das neunte Abnahmekriterium von C4 verlangt beides. Der erste meldet, der
//! zweite verhindert das Ueberschreiben. Warum der erste sich merkt, dass er
//! gemeldet hat, steht an [`Editormodell::fremdaenderung_melden`]; **ob** er
//! ueberhaupt gefragt wird, entscheidet [`crate::auffrischung`] und nicht dieses
//! Modul.
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
//! **Es gibt seit S24 genau einen Leseweg, und er laeuft ueber den Faden.**
//! Bis dahin stand `jetzt_oeffnen` daneben und las auf dem rufenden Faden, als
//! benannter Zwischenstand, solange niemand die Antwort des Fadens abholte;
//! der Takt dafuer steht jetzt in `crate::appkit::editor`. Zwei Lesewege waeren
//! zwei Wahrheiten darueber, wann der Hauptfaden anhaelt, und der zweite ist
//! deshalb mit seinem Zwischenstand gefallen. [`Editormodell::uebernehmen`]
//! bleibt die eine Stelle, an der eine gelesene Datei zum Stand wird.
//!
//! Beide Rueckgaben des Weges sagen dasselbe: [`Editormodell::oeffnen`] und
//! [`Editormodell::einziehen`] liefern `Some(...)`, wenn ein Ausgang zu
//! behandeln ist, und `None`, wenn der Aufrufer nichts zu tun hat und auf den
//! Faden wartet.
//!
//! # Was dieses Modul nicht tut
//!
//! Es **fragt nicht nach**. Die Nachfrage an den vier Anlaessen aus C4 ist ein
//! Blatt am Fenster, und das Blatt wohnt in `crate::appkit`. Dieses Modul
//! beantwortet allein, ob es etwas zu fragen gibt
//! ([`Editormodell::hat_ungesicherten_stand`]), und fuehrt aus, was die
//! Antwort verlangt.
//!
//! # Die gelesene Datei wird zurueckgehalten, statt den Stand zu ueberschreiben
//!
//! Einer der vier Anlaesse gehoert diesem Modul trotzdem, und zwar nicht, weil
//! es fragte, sondern weil allein hier die Reihenfolge einzuhalten ist, die das
//! elfte Abnahmekriterium von C2 verlangt: **erst die Pruefung, dann die
//! Nachfrage.** Gelesen und geprueft wird seit S24 auf dem Arbeitsfaden, und wer
//! vor [`Editormodell::oeffnen`] fragte, fragte vor der Pruefung — der Nutzer
//! bekaeme die Nachfrage auch fuer einen Ordner, den der Editor ohnehin abweist.
//!
//! ```text
//!  oeffnen ──> Arbeitsfaden ──> einziehen ──┬─ abgewiesen ─────> Abgewiesen
//!                                           ├─ kein ungesicherter Stand
//!                                           │                 ──> Geoeffnet
//!                                           └─ ungesicherter Stand
//!                                                             ──> Zurueckgehalten
//!                                                                      │
//!            zurueckgehaltenes_uebernehmen  <── Antwort des Nutzers ────┤
//!            zurueckgehaltenes_fallenlassen <───────────────────────────┘
//! ```
//!
//! Die gelesene Datei wartet dann in [`Editormodell`], und der gehaltene Stand
//! steht unangetastet da, bis der Aufrufer die Antwort des Nutzers bringt. Das
//! ist **kein zweiter Stand des Editors**: was der Editor haelt, sagt weiterhin
//! [`Editormodell::stand`] allein. Der zurueckgehaltene Wert ist ein noch nicht
//! angenommener Eingang, und er hat genau zwei Ausgaenge, von denen jeder ihn
//! aufbraucht.

// **Die Zeile `#![allow(dead_code)]`, die hier bis zum 260810 stand, ist mit
// S37 gefallen**, wie sie es angekuendigt hatte. Sie deckte vierzehn
// Fundstellen ab, solange die Befehle des Editors fehlten; mit dem Lesen auf
// dem Arbeitsfaden (S24), dem Sichern (S25), der Abweichungsmarke (S26), den
// beiden Ansichten (S33), dem Suchlauf (S36) und dem Ersetzen (S37) haben zehn
// davon ihren Aufrufer bekommen.
//
// **Vier haben ihn nicht**, und sie tragen die Ausnahme seither einzeln, mit
// dem Grund daran. Das ist der Unterschied, um den es geht: eine Zeile am
// Dateikopf verbirgt jede kuenftige tote Stelle mit, vier Zeilen an vier
// Stellen nennen genau die vier. Gemessen am 260810 nach S37, ohne die
// Ausnahmen: `cargo clippy --workspace --all-targets` meldet `Suchlauf::treffer`,
// `Editormodell::stempel`, `Editormodell::haelt_zurueck` und
// `Editormodell::suche_beenden`, und der Arbeitsbereich stuende rot, weil
// `make lint` mit `-D warnings` faehrt. Tot ist auch dann nichts: die Pruefungen
// am Dateiende fassen jedes Stueck dieses Moduls an.
//
// Von den vieren nennt allein `stempel` einen Schritt, der ihn ruft, naemlich
// S31. Die drei uebrigen nennt kein Schritt des Plans; sie stehen als
// `issues/260810-0212_o_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`.
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
/// seit S33 `crate::hervorhebung::art` beim Darstellen, indem es die Kiste nach
/// dem Pfad fragt und ihre Antwort nimmt.
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
    // **Ohne Aufrufer, und der Plan nennt keinen.** Die Oberflaeche kommt mit
    // `zahl`, `nummer`, `angesteuert` und `meldung` aus; wer die ganze Liste
    // braeuchte, waere jemand, der die Treffer alle zugleich zeichnete, und das
    // sagt C5 nicht zu. Gefuehrt als
    // `issues/260810-0212_*_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`.
    #[allow(dead_code)]
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
/// **Vier Werte, ueberschneidungsfrei und vollstaendig.** Entweder der Editor
/// haelt danach eine neue Datei, oder er hielt sie schon und nichts hat sich
/// bewegt, oder die gelesene Datei wartet auf die Nachfrage aus C4, oder er
/// haelt weiter, was er vorher hielt, und der Nutzer bekommt den Grund. Ein
/// fuenfter Ausgang, bei dem der Editor nichts mehr haelt, entsteht nicht: eine
/// gescheiterte Anfrage wirft nichts weg.
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
    /// Die Datei ist gelesen und geprueft, wird aber zurueckgehalten: der Editor
    /// haelt ungesicherten Stand, und die Nachfrage aus C4 steht davor (C2, C4).
    ///
    /// **Der Ausgang bewegt nichts.** Der gehaltene Stand, die Abweichungsmarke,
    /// der Pfad und der Stempel stehen unveraendert da; die Textflaeche wird
    /// nicht beschrieben. Der Aufrufer hat genau eines zu tun: zu fragen, und
    /// die Antwort ueber [`Editormodell::zurueckgehaltenes_uebernehmen`] oder
    /// [`Editormodell::zurueckgehaltenes_fallenlassen`] zurueckzubringen.
    ///
    /// Ein Wert und kein `bool` am Ausgang `Geoeffnet`: die drei Ausgaenge
    /// verlangen drei verschiedene Handlungen der Ansicht, und ein Kennzeichen
    /// daneben liesse die Fallunterscheidung unvollstaendig, die dieses
    /// Programm an jeder solchen Stelle erzwingt.
    Zurueckgehalten,
    /// Der Grund gehoert in die Statuszeile aus C1. Der bisherige Stand bleibt.
    Abgewiesen(Abweisung),
}

/// Eine gelesene Datei, die auf die Antwort der Nachfrage aus C4 wartet.
///
/// Sie haelt den Pfad und die Lieferung des Arbeitsfadens zusammen, weil
/// [`Editormodell::uebernehmen`] beide braucht und weil zwei Felder nebeneinander
/// zwei Wahrheiten darueber waeren, ob etwas wartet.
#[derive(Debug)]
struct Zurueckgehalten {
    /// Der Pfad, fuer den der Faden gelesen hat.
    pfad: PathBuf,
    /// Was er geliefert hat. Immer ein `Ok`: eine Abweisung wird nie
    /// zurueckgehalten, sondern sofort gemeldet.
    geladen: Geladen,
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
    ///
    /// **Der Pfad steht dabei**, weil die Meldung an den Nutzer ihn nennt, wie
    /// jede andere Meldung des Editors. Ihn beim Aufrufer ein zweites Mal zu
    /// erfragen hiesse, ein Modell zu befragen, das die Frage eben beantwortet
    /// hat, als es schrieb — und es hiesse, mit einem `Option` umzugehen, das
    /// an dieser Stelle nie leer ist, weil ein leeres
    /// [`Self::NichtsGehalten`] heisst.
    Gesichert(PathBuf),
    /// Der Grund gehoert in die Statuszeile; der Stand bleibt unveraendert
    /// stehen, und ein Anlass, der auf dieses Sichern gewartet hat, unterbleibt.
    ///
    /// **Zwei Anlaesse fuehren hierher, und beide sagen dasselbe zu**: das
    /// Schreiben ist gescheitert, und ein Schreiben, das unterblieben ist, weil
    /// die Datei sich von aussen geaendert hat. Verschieden ist allein der Satz
    /// darin. Sie zu trennen braechte dem Aufrufer nichts: er hat in beiden
    /// Faellen dasselbe zu tun, naemlich den Grund zu zeigen und den Anlass
    /// unterbleiben zu lassen.
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
    /// Eine gelesene Datei, die auf die Antwort der Nachfrage aus C4 wartet.
    ///
    /// `None` ist der gewoehnliche Zustand; belegt ist das Feld allein zwischen
    /// [`Ladeausgang::Zurueckgehalten`] und der Antwort des Nutzers. Der Grund
    /// steht im Modulkopf.
    zurueckgehalten: Option<Zurueckgehalten>,
    /// Ob die laufende fremde Aenderung dem Nutzer schon gemeldet wurde (C4).
    ///
    /// Es beantwortet **nicht**, ob die Datei sich geaendert hat — das tut
    /// [`Editormodell::fremd_geaendert`] mit einem `stat(2)` —, sondern allein,
    /// ob dieselbe Aenderung schon einen Satz in der Statuszeile hatte. Ohne das
    /// Feld truege jede weitere Meldung des Ordners denselben Satz noch einmal,
    /// und ein fremdes Programm, das im Sekundentakt schreibt, verdraengte jede
    /// andere Meldung. Gesetzt und geloescht wird es allein in
    /// [`Editormodell::fremdaenderung_melden`]; siehe den Grund dort.
    fremd_gemeldet: bool,
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
    /// Bedingung, unter der [`Self::oeffnen`] keinen Faden startet. Verglichen
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
    // **S31 hat diese Zeile nicht abgeloest, obwohl sie es angekuendigt hatte,
    // und das ist die richtige Auflegung.** Angekuendigt war, das Melden einer
    // fremden Aenderung vergleiche hier den Stempel. Gebaut ist es ueber
    // `fremd_geaendert`, weil S25 dieselbe Frage schon so stellt und der
    // Vergleich damit an einer Stelle steht statt an zweien; der Stempel selbst
    // geht dafuer nicht nach aussen. Ohne Aufrufer bleiben damit vier Stuecke
    // statt drei, und der Defekt
    // `issues/260810-0212_*_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`
    // fuehrt sie; die Pruefungen am Dateiende fassen jedes von ihnen an.
    #[allow(dead_code)]
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
    /// `None` heisst: es laeuft ein Ladevorgang, und sein Ausgang kommt aus
    /// [`Self::einziehen`]. `Some(...)` heisst: der Ausgang steht schon fest,
    /// nichts laedt, und der Aufrufer hat ihn jetzt zu behandeln.
    ///
    /// **Fragt nicht nach, haelt aber zurueck.** Steht ungesicherter Stand
    /// offen, ist das einer der vier Anlaesse aus C4. Die Nachfrage gehoert
    /// nicht vor diesen Ruf, sondern hinter die Pruefung, die auf dem
    /// Arbeitsfaden laeuft: [`Self::einziehen`] liefert dann
    /// [`Ladeausgang::Zurueckgehalten`], und der Aufrufer fragt. Der Grund und
    /// das Bild dazu stehen im Modulkopf.
    ///
    /// # Die Datei, die der Editor schon haelt, wird nicht neu gelesen
    ///
    /// Haelt der Editor genau diesen Pfad, kehrt die Funktion mit
    /// [`Ladeausgang::SchonOffen`] zurueck, **bevor** sie einen Faden startet,
    /// und ruehrt nichts an. Ohne diese Zeile ist ein zweites F4 auf dieselbe
    /// Datei ein vollwertiges Oeffnen: der Faden laese die Datei neu,
    /// [`Self::uebernehmen`] setzte den Plattenstand ein, loeschte die
    /// Abweichungsmarke, und die Ansicht schriebe den Plattenstand ueber das,
    /// was der Nutzer getippt hat. Genau diesen Weg ging der Nutzer am 260809,
    /// weil die Vorschau den Editor nach C1 verdraengt und F4 der einzige
    /// Befehl ist, der ihn mit seiner Datei zurueckholt
    /// (`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`).
    /// Die Abkuerzung stand bis S24 in `jetzt_oeffnen` und ist mit dem Umstieg
    /// auf den Faden hierher gewandert, wie der Doc-Kommentar dort verlangte.
    ///
    /// **Der Preis steht hier und wird nicht verschwiegen:** F4 auf die schon
    /// gehaltene Datei liest sie damit auch dann nicht neu, wenn sie sich von
    /// aussen geaendert hat. Ein Befehl zum Neulesen gibt es nicht, und C2 sagt
    /// keinen zu; die Aenderung von aussen traegt S31. Die Nachfrage aus C4
    /// greift auf dieser Abkuerzung nicht, und sie soll es nicht: es wird
    /// nichts gelesen und nichts ersetzt, also ist auch nichts zu verlieren.
    pub fn oeffnen(&mut self, pfad: &Path) -> Option<Ladeausgang> {
        if self.haelt_bereits(pfad) {
            return Some(Ladeausgang::SchonOffen);
        }
        self.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf()));
        None
    }

    /// Uebernimmt, was ein Lesevorgang geliefert hat.
    ///
    /// **Die eine Stelle, an der eine gelesene Datei zum Stand des Editors
    /// wird.** Ein Weg fuehrt hierher, [`Self::einziehen`] vom Arbeitsfaden;
    /// bis S24 war es daneben `jetzt_oeffnen` vom rufenden Faden. Zwei
    /// Uebergaenge nebeneinander waeren zwei Wahrheiten darueber, was ein
    /// geoeffneter Editor haelt, und weil beide durch diese Funktion gingen,
    /// hat der Wegfall des zweiten Lesewegs am Ergebnis nichts geaendert.
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

    /// Nimmt die Lieferung auf oder haelt sie fuer die Nachfrage aus C4
    /// zurueck.
    ///
    /// **Die eine Stelle, an der die Reihenfolge aus dem elften
    /// Abnahmekriterium von C2 haengt: erst die Pruefung, dann die Nachfrage.**
    /// Sie steht hier und nicht bei den Einstiegen, und das ist der ganze
    /// Gewinn: F4, der Uebergang aus der Vorschau und der Sprung auf eine
    /// Textmarke aus C6 erben die Regel, ohne sie zu kennen. Drei Abfragen bei
    /// drei Aufrufern waeren drei Wahrheiten darueber, wann gefragt wird, und
    /// die erste Abweichung zwischen ihnen faende keine Pruefung.
    ///
    /// Zwei Faelle gehen unverzueglich durch:
    ///
    /// - **Eine Abweisung**, weil sie nichts anfasst. Eine Nachfrage ueber eine
    ///   Datei, die der Editor gar nicht nimmt, kostete den Nutzer eine Antwort
    ///   ohne Gegenstand; genau das verbietet das elfte Abnahmekriterium.
    /// - **Ein Editor ohne ungesicherten Stand**, weil dann nichts zu verlieren
    ///   ist und es nichts zu fragen gibt.
    fn uebernehmen_oder_zurueckhalten(&mut self, pfad: PathBuf, geladen: Geladen) -> Ladeausgang {
        if geladen.ergebnis.is_err() || !self.abweichung {
            return self.uebernehmen(pfad, geladen);
        }
        self.zurueckgehalten = Some(Zurueckgehalten { pfad, geladen });
        Ladeausgang::Zurueckgehalten
    }

    /// Nimmt die zurueckgehaltene Datei jetzt auf (C4).
    ///
    /// Der Weg der Antworten "sichern" und "verwerfen": in beiden Faellen
    /// nimmt der Editor die neue Datei, und der bisherige Stand faellt — beim
    /// Sichern, nachdem er in seiner Datei steht, beim Verwerfen ohne das.
    ///
    /// Die Uebernahme geht durch [`Self::uebernehmen`] wie jede andere; es gibt
    /// keinen zweiten Uebergang in den gehaltenen Stand. `None` heisst: es
    /// wartete nichts, und dann ist auch nichts zu tun.
    pub fn zurueckgehaltenes_uebernehmen(&mut self) -> Option<Ladeausgang> {
        let wartend = self.zurueckgehalten.take()?;
        Some(self.uebernehmen(wartend.pfad, wartend.geladen))
    }

    /// Laesst die zurueckgehaltene Datei fallen (C4).
    ///
    /// Der Weg der Antwort "abbrechen" und der eines gescheiterten Sicherns:
    /// der Anlass unterbleibt, der gehaltene Stand bleibt mit seiner
    /// Abweichungsmarke stehen, und die gelesene Datei wird nicht gebraucht.
    pub fn zurueckgehaltenes_fallenlassen(&mut self) {
        self.zurueckgehalten = None;
    }

    /// Ob eine gelesene Datei auf die Antwort der Nachfrage wartet (C4).
    // **Ohne Aufrufer, und der Plan nennt keinen.** Die Oberflaeche braucht die
    // Frage nicht: sie erfaehrt das Zurueckhalten als `Ladeausgang` und
    // beantwortet es im Rueckruf des Blattes, ohne zwischendurch nachzusehen.
    // Gefuehrt im selben Defekt wie `Suchlauf::treffer`.
    #[allow(dead_code)]
    pub fn haelt_zurueck(&self) -> bool {
        self.zurueckgehalten.is_some()
    }

    /// Ob ein Ladevorgang laeuft.
    pub fn laedt_noch(&self) -> bool {
        self.ladevorgang.is_some()
    }

    /// Holt die wartende Meldung des Arbeitsfadens ab.
    ///
    /// Liefert `None`, solange keine da ist oder gar kein Vorgang laeuft; nur
    /// bei `Some` hat die Ansicht etwas zu tun. Gerufen wird sie vom Einzugstakt
    /// in `crate::appkit::editor`, im Takt von 1/60 s, solange
    /// [`Self::laedt_noch`] wahr ist.
    ///
    /// Bei [`Ladeausgang::Geoeffnet`] steht danach die neue Datei mit ihrem
    /// Stand, ihrem Typ, ihrem Stempel und ohne Abweichung; ein Suchlauf ueber
    /// den alten Stand ist beendet, weil seine Versaetze in den neuen nicht
    /// mehr passen. Bei [`Ladeausgang::Zurueckgehalten`] hat sich dagegen
    /// nichts bewegt, und der Aufrufer hat zu fragen; siehe
    /// [`Self::uebernehmen_oder_zurueckhalten`].
    pub fn einziehen(&mut self) -> Option<Ladeausgang> {
        let vorgang = self.ladevorgang.as_ref()?;
        let geladener_pfad = vorgang.pfad.clone();
        match vorgang.empfaenger.try_recv() {
            Ok(geladen) => {
                self.ladevorgang = None;
                Some(self.uebernehmen_oder_zurueckhalten(geladener_pfad, geladen))
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
    /// **Der Aufrufer ist seit S26 der Delegierte `textDidChange:`** in
    /// `crate::appkit::editor`, die eine Stelle, die AppKit fuer diese Meldung
    /// vorsieht. Bis dahin hatte diese Funktion keinen, und was der Nutzer
    /// tippte, stand allein in der `NSTextView`
    /// (`issues/260809-2148_*_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`).
    ///
    /// Setzt die Abweichungsmarke, immer und ohne Vergleich; der Grund und der
    /// Preis stehen im Modulkopf.
    ///
    /// **Der ganze Stand kommt herein und nicht die geaenderte Stelle**, und
    /// das kostet je Tastendruck einen Durchlauf ueber die Datei. Die Wahl ist
    /// die des Modulkopfs — ein Eingang, eine Wandlungsstelle — und der Ausweg
    /// ist benannt und nicht zu suchen: `NSTextStorage` meldet den geaenderten
    /// Bereich mit, und ein Stand, der sich daran fortschreibt, kostete die
    /// geaenderte Stelle. `speculation:` ungemessen, wie beim Zeilenindex aus
    /// S46, der dieselbe Frage stellt;
    /// `issues/260809-2322_*_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`
    /// fuehrt sie.
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
    ///
    /// # Der Rueckgabewert sagt, ob die Flaeche nachzuziehen ist
    ///
    /// `true` heisst: der hereingegebene Text war **nicht** in gehaltener Form,
    /// und der Stand traegt jetzt andere Zeichen als der Textbestand, aus dem er
    /// kam. Wer diesen Bestand fuehrt, hat ihn danach auf den Stand zu bringen;
    /// tut er es nicht, zeigt dieselbe Stelle in den beiden Texten von der
    /// Wandlung an auf Verschiedenes, und die Umrechnung zwischen den beiden
    /// Koordinaten in `crate::appkit::koordinaten` rechnet gegen den falschen
    /// Text (`260810-0215`).
    ///
    /// **Der Wert wird nicht aus einem Vergleich der beiden Zeichenketten
    /// gewonnen**, der eine Kopie des ganzen Standes voraussetzte, sondern aus
    /// `krk_core::text::datei::ist_in_gehaltener_form` — derselben Bedingung,
    /// an der die Wandlung ihren kurzen Weg nimmt. Sie kostet einen zweiten
    /// Durchlauf ueber den Text neben dem der Wandlung; gemessen an dem
    /// Umschreiben aus UTF-16, das jedem Ruf hierher vorausgeht, ist er nicht zu
    /// bemerken.
    pub fn bearbeiten(&mut self, neuer_stand: String) -> bool {
        let war_gehalten = datei::ist_in_gehaltener_form(&neuer_stand);
        self.stand = datei::in_gehaltene_form(neuer_stand);
        self.abweichung = true;
        self.suchlauf = None;
        !war_gehalten
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
    ///
    /// # Der Stempel wird vor dem Schreiben geprueft
    ///
    /// Hat die Datei sich seit dem Oeffnen oder dem letzten Sichern von aussen
    /// geaendert, unterbleibt das Schreiben, und der Grund geht in die
    /// Statuszeile. Das ist die eine Haelfte des neunten Abnahmekriteriums von
    /// C4, die ohne Weiteres zuverlaessig ist: sie fragt in dem Augenblick, in
    /// dem es darauf ankommt, naemlich unmittelbar vor dem Ueberschreiben. Die
    /// andere Haelfte, das Melden im laufenden Betrieb, kommt mit S31.
    ///
    /// **Gefragt wird ueber [`Self::fremd_geaendert`] und nicht mit einer
    /// zweiten, enger geschnittenen Frage daneben.** Damit gilt eine
    /// verschwundene oder unlesbar gewordene Datei ebenfalls als geaendert, und
    /// **das ist der Preis, der hier steht und nicht verschwiegen wird:** wem
    /// die geoeffnete Datei unter der Hand weggeraeumt wird, der bekommt sie
    /// aus dem Editor heraus nicht wieder geschrieben, solange die Wahl aus dem
    /// Zustandsbild des Specs (`Fremd` mit seinen zwei Ausgaengen) nicht
    /// gebaut ist; sein Stand bleibt dabei vollstaendig stehen. Eine zweite
    /// Frage, die das Verschwinden vom Aendern trennte, waere ein Sonderfall
    /// mit eigener Regel an einer Stelle, die genau eine Frage zu stellen hat.
    ///
    /// **Ein Wettlauf bleibt und ist nicht zu schliessen.** Zwischen der Frage
    /// und dem `rename` in `crate::ablage::atomar` liegt eine Spanne, in der
    /// ein fremder Schreiber zuschlagen kann. Diese Pruefung macht das Fenster
    /// klein; zu schliessen waere es allein mit einer Sperre auf der Datei, und
    /// die sagt weder C4 noch der Spec zu.
    pub fn sichern(&mut self) -> Sicherungsausgang {
        let Some(pfad) = self.pfad.as_ref() else {
            return Sicherungsausgang::NichtsGehalten;
        };
        if self.fremd_geaendert() {
            return Sicherungsausgang::Gescheitert(format!(
                "{} hat sich außerhalb von KRK geändert und wird nicht überschrieben",
                pfad.display()
            ));
        }
        match datei::sichern(pfad, &self.stand) {
            Ok(()) => {
                self.abweichung = false;
                self.stempel = Stempel::von_pfad(pfad);
                Sicherungsausgang::Gesichert(pfad.clone())
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
    ///
    /// **Ein laufendes und ein zurueckgehaltenes Laden fallen mit.** Der Editor
    /// gibt hier alles auf, was er ueber eine Datei weiss, und eine Lieferung,
    /// die danach noch eintraefe oder wartete, gehoerte zu einer Datei, die
    /// niemand mehr will.
    pub fn schliessen(&mut self) {
        self.pfad = None;
        self.stand.clear();
        self.abweichung = false;
        self.typ = Dateityp::default();
        self.suchlauf = None;
        self.stempel = None;
        self.ladevorgang = None;
        self.zurueckgehalten = None;
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

    /// Der Satz ueber eine fremde Aenderung, einmal je Aenderung (C4).
    ///
    /// **Der erste der beiden Momente aus dem neunten Abnahmekriterium von C4.**
    /// Der zweite ist [`Self::sichern`], das unmittelbar vor dem Ueberschreiben
    /// dieselbe Frage stellt. Es ist derselbe Vergleich an zwei Stellen und kein
    /// zweiter Mechanismus: dieser meldet, jener verhindert das Ueberschreiben.
    /// Gefragt wird deshalb auch hier ueber [`Self::fremd_geaendert`] und nicht
    /// mit einer zweiten, enger geschnittenen Frage daneben; damit gilt eine
    /// verschwundene Datei ebenfalls als geaendert, wie dort.
    ///
    /// **Gerufen wird sie, wenn die Dateisystemwache den Ordner der gehaltenen
    /// Datei meldet.** Ob die Meldung den Ordner ueberhaupt betrifft, hat
    /// [`crate::auffrischung::betrifft_editordatei`] vorher entschieden; diese
    /// Funktion stellt keine zweite Vorbedingung daneben und kostet einen
    /// `stat(2)`.
    ///
    /// # Warum sie sich merkt, dass sie gemeldet hat
    ///
    /// Ein fremdes Programm, das eine Protokolldatei fortschreibt, laesst
    /// FSEvents im Sekundentakt melden. Ohne Gedaechtnis stuende derselbe Satz
    /// bei jeder Meldung neu in der Zeile und verdraengte alles andere. Gemeldet
    /// wird deshalb der **Uebergang**: das erste Mal, seit die Datei abweicht.
    ///
    /// Die Marke loescht sich selbst, sobald der Vergleich wieder aufgeht — und
    /// er geht bei jedem Weg auf, der den Stempel neu setzt: nach einem Sichern,
    /// nach dem Aufnehmen einer Datei, nach dem Schliessen. Deshalb steht an
    /// keiner dieser drei Stellen eine Zeile dafuer.
    ///
    /// **Der Preis steht hier und wird nicht verschwiegen:** aendert ein fremdes
    /// Programm die Datei ein zweites Mal, ohne dass KRK dazwischen gesichert
    /// oder neu geoeffnet hat, kommt kein zweiter Satz. Das ist richtig herum
    /// falsch: die Aussage "die Datei auf der Platte weicht ab" gilt weiter, und
    /// das Sichern haelt sie ohnehin zurueck.
    pub fn fremdaenderung_melden(&mut self) -> Option<String> {
        if !self.fremd_geaendert() {
            self.fremd_gemeldet = false;
            return None;
        }
        if std::mem::replace(&mut self.fremd_gemeldet, true) {
            return None;
        }
        let pfad = self
            .pfad
            .as_ref()
            .expect("ohne gehaltene Datei meldet `fremd_geaendert` nichts");
        Some(format!(
            "{} hat sich außerhalb von KRK geändert",
            pfad.display()
        ))
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
    // **Ohne Aufrufer, und der Plan nennt keinen.** Der Spec sagt keinen Befehl
    // zu, der eine Suche beendet; sie endet von selbst, wenn der Nutzer tippt
    // (`bearbeiten`), eine andere Datei kommt (`uebernehmen`) oder der Editor
    // schliesst (`schliessen`), und jede dieser drei Stellen setzt das Feld
    // unmittelbar. Gefuehrt im selben Defekt wie `Suchlauf::treffer`.
    #[allow(dead_code)]
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
        assert_eq!(
            modell.oeffnen(pfad),
            None,
            "eine neue Datei wird auf dem Arbeitsfaden gelesen"
        );
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

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
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
        assert_eq!(modell.oeffnen(&erste), None);
        assert_eq!(modell.oeffnen(&zweite), None);
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

        assert_eq!(modell.oeffnen(&zweite), None);
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
        assert_eq!(modell.oeffnen(&ordner.pfad), None);
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
        let mut modell = geoeffnet(&gute);

        let zu_gross = ordner.pfad.join("zu-gross.txt");
        std::fs::File::create(&zu_gross)
            .expect("die Pruefdatei laesst sich nicht anlegen")
            .set_len(datei::EDITORGRENZE + 1)
            .expect("die Pruefdatei laesst sich nicht auf Groesse bringen");

        assert_eq!(modell.oeffnen(&zu_gross), None);
        let ausgang = abwarten(&mut modell);
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

        let ausgang = modell.oeffnen(&pfad);

        // Zuerst der Verlust selbst, damit ein Rueckfall ihn und nicht eine
        // Nebensache meldet.
        assert_eq!(
            modell.stand(),
            "auf der Platte\nund ungesichert getippt\n",
            "260809-2029: das zweite F4 wirft den ungesicherten Stand nicht weg"
        );
        assert_eq!(
            ausgang,
            Some(Ladeausgang::SchonOffen),
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
    /// Ohne ungesicherten Stand ist der Wechsel auf eine **andere** Datei ein
    /// gewoehnliches Oeffnen: es gibt nichts zu verlieren und deshalb nichts zu
    /// fragen. Die Probe haelt fest, dass die Abkuerzung fuer die gehaltene
    /// Datei den Wechsel nicht stillschweigend mitnimmt.
    #[test]
    fn eine_andere_datei_wird_weiterhin_gelesen() {
        let ordner = Pruefordner::neu("andere-datei");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);

        assert!(modell.haelt_bereits(&erste));
        assert!(!modell.haelt_bereits(&zweite));

        assert_eq!(
            modell.oeffnen(&zweite),
            None,
            "die andere Datei geht auf den Arbeitsfaden"
        );
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), "zweite\n");
        assert_eq!(modell.pfad(), Some(zweite.as_path()));
        assert!(!modell.haelt_zurueck());
    }

    /// Das fuenfte Abnahmekriterium von C4: der Wechsel auf eine andere Datei
    /// wirft den ungesicherten Stand nicht mehr ohne Nachfrage weg.
    ///
    /// **Die Probe hat mit S28 ihre Aussage gewechselt.** Bis dahin hielt sie
    /// fest, dass F4 auf eine andere Datei den getippten Stand kommentarlos
    /// ersetzt; die Nachfrage stand als Schritt aus. Jetzt haelt das Modell die
    /// gelesene Datei zurueck, und der gehaltene Stand steht vollstaendig da,
    /// bis die Antwort des Nutzers kommt.
    #[test]
    fn ein_wechsel_mit_ungesichertem_stand_haelt_die_gelesene_datei_zurueck() {
        let ordner = Pruefordner::neu("zurueckhalten");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);
        modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        assert!(modell.haelt_zurueck(), "die gelesene Datei wartet");
        assert_eq!(
            modell.pfad(),
            Some(erste.as_path()),
            "der Editor haelt weiter die erste Datei"
        );
        assert_eq!(modell.stand(), "erste, bearbeitet\n");
        assert!(
            modell.hat_ungesicherten_stand(),
            "die Abweichungsmarke steht, solange gefragt wird"
        );
    }

    /// C4: "sichern" und "verwerfen" nehmen die zurueckgehaltene Datei auf.
    ///
    /// Geprueft wird der Weg beider Antworten, denn er ist derselbe: sie
    /// unterscheiden sich allein darin, ob der Aufrufer vorher gesichert hat.
    #[test]
    fn das_zurueckgehaltene_wird_auf_antwort_aufgenommen() {
        let ordner = Pruefordner::neu("zurueckgehalten-uebernehmen");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);
        modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            Some(Ladeausgang::Geoeffnet),
            "die Uebernahme geht denselben Weg wie jedes Oeffnen"
        );
        assert_eq!(modell.pfad(), Some(zweite.as_path()));
        assert_eq!(modell.stand(), "zweite\n");
        assert!(
            !modell.hat_ungesicherten_stand(),
            "die neue Datei kommt ohne Abweichung herein"
        );
        assert!(!modell.haelt_zurueck(), "es wartet nichts mehr");
        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            None,
            "ein zweiter Ruf findet nichts und tut nichts"
        );
    }

    /// C4: "abbrechen" laesst die gelesene Datei fallen und den Stand stehen.
    #[test]
    fn ein_abgebrochener_wechsel_laesst_den_stand_vollstaendig_stehen() {
        let ordner = Pruefordner::neu("zurueckgehalten-fallenlassen");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);
        modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        modell.zurueckgehaltenes_fallenlassen();
        assert!(!modell.haelt_zurueck());
        assert_eq!(modell.pfad(), Some(erste.as_path()));
        assert_eq!(modell.stand(), "erste, bearbeitet\n");
        assert!(modell.hat_ungesicherten_stand());
    }

    /// Das elfte Abnahmekriterium von C2: die Pruefung steht vor der Nachfrage.
    ///
    /// Eine Datei, die der Editor ohnehin abweist, wird nicht zurueckgehalten
    /// und kostet den Nutzer deshalb keine Rueckfrage — auch dann nicht, wenn
    /// er ungesicherten Stand haelt. Das ist der Fall, an dem die Reihenfolge
    /// haengt, und er ist der Grund, aus dem das Zurueckhalten im Modell steht
    /// und nicht bei den beiden Einstiegen.
    #[test]
    fn eine_abgewiesene_datei_wird_nicht_zurueckgehalten() {
        let ordner = Pruefordner::neu("abweisung-ohne-nachfrage");
        let gute = ordner.datei("gut.txt", "guter Inhalt\n");
        let mut modell = geoeffnet(&gute);
        modell.bearbeiten("guter Inhalt, bearbeitet\n".to_owned());

        // Ein Ordner ist der Fall, den die Pruefung namentlich abweist.
        assert_eq!(modell.oeffnen(&ordner.pfad), None);
        let ausgang = abwarten(&mut modell);
        assert!(
            matches!(ausgang, Ladeausgang::Abgewiesen(_)),
            "eine Abweisung geht unverzueglich durch, {ausgang:?}"
        );
        assert!(
            !modell.haelt_zurueck(),
            "eine abgewiesene Datei wartet auf keine Antwort"
        );
        assert!(modell.hat_ungesicherten_stand());
    }

    /// C1, C4: das Schliessen gibt auch eine wartende Datei auf.
    #[test]
    fn das_schliessen_laesst_die_zurueckgehaltene_datei_fallen() {
        let ordner = Pruefordner::neu("schliessen-zurueckgehalten");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);
        modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        modell.schliessen();
        assert!(!modell.haelt_zurueck());
        assert!(!modell.haelt_datei());
        assert_eq!(modell.stand(), "");
        assert!(!modell.hat_ungesicherten_stand());
    }

    /// C4: ein gescheitertes Schreiben nennt den Grund und wirft den Stand
    /// nicht weg.
    ///
    /// Der Fehlschlag wird an dem Ort erzeugt, an dem er beim Nutzer entsteht:
    /// im **Ordner**, nicht an der Datei. `krk_core::ablage::atomar` schreibt
    /// erst eine Nachbardatei und benennt sie dann um; ein `rename` gelingt
    /// auch auf eine schreibgeschuetzte Datei, solange der Ordner darum
    /// beschreibbar ist. Die Rechte werden unmittelbar nach dem Ruf
    /// zurueckgesetzt, damit der Pruefordner sich in `Drop` abraeumen kann.
    #[test]
    fn ein_gescheitertes_schreiben_laesst_den_stand_stehen() {
        use std::os::unix::fs::PermissionsExt;

        let ordner = Pruefordner::neu("sichern-scheitert");
        let unterordner = ordner.pfad.join("gesperrt");
        std::fs::create_dir(&unterordner).expect("der Unterordner laesst sich anlegen");
        let pfad = unterordner.join("stand.txt");
        std::fs::write(&pfad, "Inhalt\n").expect("die Pruefdatei laesst sich schreiben");

        let mut modell = geoeffnet(&pfad);
        modell.bearbeiten("neuer Inhalt\n".to_owned());

        std::fs::set_permissions(&unterordner, std::fs::Permissions::from_mode(0o500))
            .expect("die Rechte lassen sich setzen");
        let ausgang = modell.sichern();
        std::fs::set_permissions(&unterordner, std::fs::Permissions::from_mode(0o700))
            .expect("die Rechte lassen sich zuruecksetzen");

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
        assert_eq!(
            std::fs::read_to_string(&pfad).expect("die Datei ist lesbar"),
            "Inhalt\n",
            "ein gescheitertes Schreiben laesst die Datei, wie sie war"
        );
    }

    /// Das neunte Abnahmekriterium von C4, an der Stelle, an der der Schaden
    /// entstuende: eine von aussen geaenderte Datei wird nicht ueberschrieben.
    #[test]
    fn eine_von_aussen_geaenderte_datei_wird_nicht_ueberschrieben() {
        let ordner = Pruefordner::neu("sichern-fremd");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        modell.bearbeiten("im Editor getippt\n".to_owned());

        std::fs::write(&pfad, "von jemand anderem geschrieben\n")
            .expect("die Datei laesst sich von aussen schreiben");

        match modell.sichern() {
            Sicherungsausgang::Gescheitert(grund) => assert!(
                grund.contains("außerhalb von KRK"),
                "der Grund nennt die fremde Änderung: {grund}"
            ),
            sonst => panic!("die fremde Änderung haette das Schreiben anhalten muessen, {sonst:?}"),
        }
        assert_eq!(
            std::fs::read_to_string(&pfad).expect("die Datei ist lesbar"),
            "von jemand anderem geschrieben\n",
            "C4: die fremde Änderung wird nicht ohne Zutun des Nutzers ueberschrieben"
        );
        assert_eq!(modell.stand(), "im Editor getippt\n");
        assert!(
            modell.hat_ungesicherten_stand(),
            "der eigene Stand bleibt vollstaendig stehen"
        );
    }

    /// Eine verschwundene Datei geht denselben Weg wie eine geaenderte, und der
    /// Preis dafuer steht am Doc-Kommentar von [`Editormodell::sichern`].
    #[test]
    fn eine_verschwundene_datei_wird_nicht_neu_geschrieben() {
        let ordner = Pruefordner::neu("sichern-fort");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        modell.bearbeiten("im Editor getippt\n".to_owned());

        std::fs::remove_file(&pfad).expect("die Datei laesst sich loeschen");

        assert!(
            matches!(modell.sichern(), Sicherungsausgang::Gescheitert(_)),
            "eine verschwundene Datei gilt als von aussen geaendert"
        );
        assert!(!pfad.exists(), "geschrieben wurde nichts");
        assert!(modell.hat_ungesicherten_stand());
    }

    #[test]
    fn ein_editor_ohne_datei_hat_nichts_zu_sichern() {
        let mut modell = Editormodell::neu();
        assert_eq!(modell.sichern(), Sicherungsausgang::NichtsGehalten);
    }

    /// C4: der Stempel steht nach dem Oeffnen und nach dem Sichern auf der
    /// Datei, wie sie auf der Platte liegt.
    ///
    /// Die Reihenfolge ist seit S25 die umgekehrte: das Sichern kommt vor der
    /// fremden Aenderung, weil es nach ihr gar nicht mehr schreibt. Geprueft
    /// wird dieselbe Zusage — das eigene Sichern ist keine Aenderung von aussen
    /// und zieht den Stempel mit.
    #[test]
    fn der_stempel_kennt_eine_aenderung_von_aussen() {
        let ordner = Pruefordner::neu("stempel");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        assert!(modell.stempel().is_some());
        assert!(!modell.fremd_geaendert());

        modell.bearbeiten("im Editor geändert\n".to_owned());
        assert!(
            !modell.fremd_geaendert(),
            "die eigene Bearbeitung ruehrt die Datei nicht an"
        );
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        assert!(
            !modell.fremd_geaendert(),
            "das eigene Sichern zieht den Stempel mit"
        );

        std::fs::write(&pfad, "von aussen geaendert\n").expect("die Datei laesst sich schreiben");
        assert!(
            modell.fremd_geaendert(),
            "C4: eine Aenderung von aussen wird bemerkt"
        );
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

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let auf_der_platte =
            std::fs::read_to_string(&pfad).expect("die Datei ist nach dem Sichern lesbar");
        assert!(
            !auf_der_platte.contains('\r'),
            "260808-0043: beim Sichern gehen Unix-Zeilenenden hinaus, {auf_der_platte:?}"
        );
        assert_eq!(auf_der_platte, "aus Windows\neingefügt\nletzte\n");
    }

    /// Der Defekt 260810-0215: das Bearbeiten sagt, ob es gewandelt hat.
    ///
    /// Die `NSTextView` hinter diesem Eingang fuehrt einen eigenen Textbestand
    /// und zieht ihn nicht selbst nach. Ohne die Meldung hier bliebe ihr `\r\n`
    /// stehen, waehrend der Stand ein `\n` traegt, und jede Stelle hinter der
    /// eingefuegten zeigte in den beiden Texten auf Verschiedenes.
    #[test]
    fn ein_eingefuegtes_crlf_meldet_sich_und_ein_gewoehnlicher_anschlag_nicht() {
        let ordner = Pruefordner::neu("crlf-meldung");
        let pfad = ordner.datei("stand.txt", "erste Zeile\n");
        let mut modell = geoeffnet(&pfad);

        assert!(
            !modell.bearbeiten("erste Zeile\nzweite Zeile\n".to_owned()),
            "ein gewoehnlicher Anschlag laesst die Flaeche in Ruhe"
        );
        assert!(
            modell.bearbeiten("aus Windows\r\neingefügt\n".to_owned()),
            "ein eingefuegtes CRLF verlangt, die Flaeche nachzuziehen"
        );
        assert_eq!(modell.stand(), "aus Windows\neingefügt\n");
        let gehalten = modell.stand().to_owned();
        assert!(
            !modell.bearbeiten(gehalten),
            "der gewandelte Stand meldet sich nicht ein zweites Mal"
        );

        // Die fuehrende Bytefolgenmarke faellt unter dieselbe Meldung: sie
        // verkuerzt den Stand gegenueber der Flaeche genauso.
        assert!(modell.bearbeiten("\u{feff}mit Marke\n".to_owned()));
        assert_eq!(modell.stand(), "mit Marke\n");
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

    /// Der erste Moment aus dem neunten Abnahmekriterium von C4: eine fremde
    /// Aenderung meldet sich, und zwar einmal.
    ///
    /// Die Aenderung wird ueber `set_len` und ein Neuschreiben erzeugt, damit
    /// sich die Groesse **und** die Aenderungszeit bewegen; auf einem
    /// Dateisystem mit grober Zeitaufloesung traegt sonst allein die Groesse den
    /// Unterschied, und die Probe haenge an ihr.
    #[test]
    fn eine_fremde_aenderung_meldet_sich_und_meldet_sich_nur_einmal() {
        let ordner = Pruefordner::neu("fremd-geaendert");
        let pfad = ordner.datei("stand.txt", "der eigene Stand\n");
        let mut modell = geoeffnet(&pfad);

        assert_eq!(
            modell.fremdaenderung_melden(),
            None,
            "eine unveraenderte Datei meldet nichts"
        );

        std::fs::write(&pfad, "von einem fremden Programm geschrieben\n")
            .expect("die Pruefdatei laesst sich nicht neu schreiben");

        let satz = modell
            .fremdaenderung_melden()
            .expect("die fremde Aenderung wurde nicht gemeldet");
        assert!(
            satz.contains("stand.txt") && satz.contains("außerhalb von KRK"),
            "der Satz nennt die Datei und den Grund nicht: {satz}"
        );
        assert_eq!(
            modell.fremdaenderung_melden(),
            None,
            "dieselbe Aenderung meldet sich kein zweites Mal"
        );

        // **Der Preis, den der Doc-Kommentar nennt**, hier festgehalten: eine
        // zweite fremde Aenderung meldet sich nicht, solange der Stempel nicht
        // neu gesetzt wurde. Die Aussage "die Datei auf der Platte weicht ab"
        // gilt weiter, und `sichern` haelt das Ueberschreiben ohnehin zurueck.
        std::fs::write(&pfad, "und noch einmal von aussen\n")
            .expect("die Pruefdatei laesst sich nicht neu schreiben");
        assert_eq!(modell.fremdaenderung_melden(), None);

        // Das Sichern ist der Weg **nicht** zurueck: es unterbleibt, solange die
        // Datei abweicht (S25). Zurueck fuehrt allein ein neuer Stempel, und den
        // setzt das Aufnehmen einer Datei.
        assert!(matches!(
            modell.sichern(),
            Sicherungsausgang::Gescheitert(_)
        ));

        modell.schliessen();
        assert_eq!(modell.oeffnen(&pfad), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(
            modell.fremdaenderung_melden(),
            None,
            "die eben gelesene Datei weicht nicht ab"
        );

        std::fs::write(&pfad, "ein drittes Mal von aussen\n")
            .expect("die Pruefdatei laesst sich nicht neu schreiben");
        assert!(
            modell.fremdaenderung_melden().is_some(),
            "nach einem neuen Stempel meldet sich die naechste fremde Aenderung wieder"
        );
    }

    /// Ohne gehaltene Datei gibt es keine fremde Aenderung.
    #[test]
    fn ein_editor_ohne_datei_meldet_keine_fremde_aenderung() {
        let mut modell = Editormodell::neu();
        assert_eq!(modell.fremdaenderung_melden(), None);
    }
}
