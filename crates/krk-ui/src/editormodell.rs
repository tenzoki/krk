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
//! bekommt an den drei Anlaessen aus C4 die Nachfrage. Der Gegenwert ist, dass
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
//! das achte Abnahmekriterium von C4 verlangt beides. Der erste meldet, der
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
//! **Hoechstens ein Lesen ist offen, und es ist das zuletzt begonnene.** Der
//! Satz gilt an beiden Ausgaengen von [`Editormodell::oeffnen`]: die Abkuerzung
//! fuer die schon gehaltene Datei gibt ein laufendes Lesen auf, statt es
//! stehenzulassen. Bis zum 260810 tat sie es nicht, und dann gehoerten zwei
//! Ladeausgaenge zu einer Folge von Oeffnungen; der Grund im Einzelnen steht
//! dort.
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
//! # `.secrets.txt`: gelesen mit PIN, gesichert als Chiffrat
//!
//! Seit Schritt 5.4a der krkhome-Arbeit (C7) haelt das Modell neben dem Stand
//! einen [`Schutz`], und er entscheidet, wie der Stand auf die Platte geht:
//!
//! ```text
//!  oeffnen(pfad, pin) ──> .secrets.txt? ──nein, ohne PIN──> Faden: datei::oeffnen
//!                              │                             Schutz::Klartext
//!                              ├──ja, ohne PIN──> Abgewiesen, kein Faden
//!                              └──ja, mit PIN───> Faden: leer? neuer_schluessel
//!                                                      : tresor::oeffnen
//!                                                 Schutz::Verschluesselt
//!
//!  sichern ──> Stempel gleich? ──> Klartext:       .secrets.txt? nicht schreiben
//!                                                  sonst datei::sichern
//!                                  Verschluesselt: Chiffrat::verschliessen
//!                                                  ──> Chiffrat::schreiben
//! ```
//!
//! Abgeleitet wird allein auf dem Arbeitsfaden, beim Oeffnen, und nie beim
//! Sichern. Die Proben `kein_weg_schreibt_klartext_nach_secrets_txt` und
//! `die_sperre_fragt_die_sonderdatei_vor_dem_lesen` halten die Wege am
//! Quelltext; die Faelle, in denen sich die Erkennung zwischen Oeffnen und
//! Sichern aendert, stehen an [`Editormodell::uebernehmen`] und
//! [`Editormodell::sichern`]. **Nicht erkannt wird `.secrets.txt` unter einer
//! dritten Schreibweise** (ein zweiter Verweis anderswo): die Erkennung kennt
//! zwei Pfadformen, wie der Nutzer es gewaehlt hat
//! (`260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md`),
//! und unter einer dritten ist die Datei fuer den Editor eine gewoehnliche.
//!
//! **„PIN ändern" (Schritt 5.5) ist der zweite Ort einer Ableitung**, und
//! auch er leitet nicht auf dem rufenden Faden ab:
//!
//! ```text
//!  pin_aendern(alte, neue) ──> alte == gehaltene? ──> Faden „krk-pin“: neuer_schluessel
//!  pinwechsel_einziehen   ──> Stempel gleich? ──> Platte lesen ──> oeffnen_mit(gehaltener)
//!                             ──> Chiffrat::verschliessen(neuer) ──> Chiffrat::schreiben
//!                             ──> Stempel neu, Schutz mit neuem Schluessel und neuer PIN
//! ```
//!
//! Umgeschluesselt wird der Stand der Platte und nicht der des Editors; was
//! ungesichert ist, bleibt ungesichert. Der Klartext der Platte lebt dabei
//! allein im Speicher.
//!
//! # Was dieses Modul nicht tut
//!
//! Es **fragt nicht nach**. Die Nachfrage an den drei Anlaessen aus C4 ist ein
//! Blatt am Fenster, und das Blatt wohnt in `crate::appkit`. Dieses Modul
//! beantwortet allein, ob es etwas zu fragen gibt
//! ([`Editormodell::hat_ungesicherten_stand`]), und fuehrt aus, was die
//! Antwort verlangt.
//!
//! # Die gelesene Datei wird zurueckgehalten, statt den Stand zu ueberschreiben
//!
//! Einer der drei Anlaesse gehoert diesem Modul trotzdem, und zwar nicht, weil
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

// **Dieses Modul traegt keine Ausnahme von der Totpruefung, weder am Dateikopf
// noch an einem einzelnen Stueck.** Die Zeile `#![allow(dead_code)]` stand hier
// bis zum 260810 und deckte vierzehn Fundstellen ab, solange die Befehle des
// Editors fehlten; mit dem Lesen auf dem Arbeitsfaden (S24), dem Sichern (S25),
// der Abweichungsmarke (S26), den beiden Ansichten (S33), dem Suchlauf (S36) und
// dem Ersetzen (S37) haben zehn davon ihren Aufrufer bekommen. Sie ist mit S37
// gefallen, wie sie es angekuendigt hatte.
//
// **Die vier ohne Aufrufer sind danach gefallen, und zwar sie selbst und nicht
// ihre Ausnahme.** Sie trugen bis zum 260810 je ein eigenes
// `#[allow(dead_code)]` mit dem Grund daran — eine Ausnahme ohne Ablaufdatum,
// weil kein Schritt des Plans einen Aufrufer nannte. Gefragt war nicht, wie sie
// am Leben zu halten sind, sondern ob sie gebraucht werden, und die Antwort war
// bei jedem der vier dieselbe:
//
// - `Suchlauf::treffer` gab die ganze Trefferliste heraus. Die Oberflaeche kommt
//   mit `zahl`, `nummer`, `angesteuert` und `meldung` aus; die ganze Liste
//   braeuchte, wer alle Treffer zugleich zeichnete, und das sagt C5 nicht zu.
// - `Editormodell::stempel` gab den Stempel nach aussen. Gefragt wird ueber
//   `fremd_geaendert` (S25, S31), damit der Vergleich an einer Stelle steht statt
//   an zweien; damit muss der Stempel das Modell nicht verlassen.
// - `Editormodell::haelt_zurueck` fragte, ob eine gelesene Datei auf die
//   Nachfrage aus C4 wartet. Die Oberflaeche erfaehrt das als `Ladeausgang` und
//   beantwortet es im Rueckruf des Blattes, ohne zwischendurch nachzusehen.
// - `Editormodell::suche_beenden` beendete den Suchlauf. Der Spec sagt keinen
//   Befehl zu, der das tut; die Suche endet von selbst beim Tippen
//   (`bearbeiten`), beim Dateiwechsel (`uebernehmen`) und beim Schliessen
//   (`schliessen`), und jede dieser drei Stellen setzt das Feld unmittelbar.
//
// Jedes der vier hatte allein Pruefungen am Dateiende als Verwendung, und jede
// dieser Pruefungen hat ihre Aussage entweder in der Zeile daneben schon
// gestanden oder sie ueber das Verhalten statt ueber ein Feld gestellt; im
// Einzelnen steht das am jeweiligen `#[test]`. Der Datensatz ist
// `issues/260810-0212_*_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`
// samt seinem Nachtrag, der aus den drei des Titels vier macht.
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::SystemTime;

use krk_core::ablage::atomar;
use krk_core::heimordner::tresor::{self, Pin, Schluessel, Tresorfehler};
use krk_core::heimordner::{Heimordner, Sonderdatei};
use krk_core::text::datei::Lesehindernis;
use krk_core::text::{Abweisung, Treffer, datei, suche};

use crate::heimgriff::{self, Heimgriff};

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
    #[must_use]
    pub fn andere(self) -> Self {
        match self {
            Ansicht::Roh => Ansicht::Format,
            Ansicht::Format => Ansicht::Roh,
        }
    }
}

/// Was die Formatansicht ueber die gehaltene Datei **aus ihrem Pfad** weiss.
///
/// # Die Eintragsdateien in `~/krkhome/`
///
/// `notes.txt` und `tasks.txt` im erkannten Heimordner sind ein eigener Wert,
/// [`Dateityp::Eintraege`], und **keine zweite Endungsregel**: sie tragen die
/// Endung `.txt` und waeren nach der Endung `Sonstiges`. Erkannt werden sie
/// allein ueber [`Heimordner::sonderdatei`], die eine Stelle der Erkennung,
/// und die stellt keinen Systemaufruf; `von_pfad` bleibt damit so billig wie
/// vorher und kostet die Vorschau vor der Endbedingung von L7 nichts. Jede
/// vollstaendige Fallunterscheidung ueber diesen Typ haelt den Bau an, bis sie
/// die neuen Dateien eingeordnet hat (Plan des Arbeitspakets
/// `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`, Schritt 2.1).
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
    /// Eine der Eintragsdateien im erkannten `~/krkhome/`.
    ///
    /// Die Vorschau rendert `notes.txt` und `tasks.txt` als Markdown mit
    /// Aufgabenkaestchen (`crate::markdown::Lesart::Eintragsdatei`); die
    /// Formatansicht des Editors zeigt sie als Tabelle. `.secrets.txt` ist
    /// seit Schritt 5.2 ebenfalls ein Wert dieser Art, und Vorschau und Editor
    /// zweigen fuer sie ab, bevor sie etwas lesen (5.3, 5.4a).
    Eintraege(Sonderdatei),
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
    ///
    /// **Erst die Erkennung, dann die Endung.** `heim` ist die Abschrift des
    /// einen geteilten Wertes ([`crate::heimgriff`]); `None` heisst, es gibt
    /// keinen Heimordner, und dann entscheidet allein die Endung. Weder die
    /// Erkennung noch die Endung beruehren die Platte.
    #[must_use]
    pub fn von_pfad(pfad: &Path, heim: Option<&Heimordner>) -> Self {
        if let Some(sonderdatei) = heim.and_then(|heim| heim.sonderdatei(pfad)) {
            return Dateityp::Eintraege(sonderdatei);
        }
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
    #[must_use]
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
    /// Die Stelle des angesteuerten Treffers im Feld `treffer` darueber; `None`,
    /// wenn es keinen Treffer gibt oder der Durchgang eines Ersetzens zu Ende
    /// ist.
    angesteuert: Option<usize>,
}

impl Suchlauf {
    /// Wonach gesucht wird.
    #[must_use]
    pub fn gesucht(&self) -> &str {
        &self.gesucht
    }

    /// Wie viele Treffer die Datei enthaelt (C5).
    #[must_use]
    pub fn zahl(&self) -> usize {
        self.treffer.len()
    }

    /// Der angesteuerte Treffer.
    #[must_use]
    pub fn angesteuert(&self) -> Option<Treffer> {
        self.angesteuert.map(|stelle| self.treffer[stelle])
    }

    /// Der wievielte Treffer angesteuert ist, ab 1 gezaehlt (C5).
    ///
    /// Ab 1, weil die Zahl der Nutzer liest; die Versaetze daneben zaehlen
    /// Bytes ab 0. Dieselbe Trennung wie bei den Zeilennummern in
    /// `krk_core::text`.
    #[must_use]
    pub fn nummer(&self) -> Option<usize> {
        self.angesteuert.map(|stelle| stelle + 1)
    }

    /// Der Satz fuer die Statuszeile, der beide Zahlen aus C5 nennt.
    ///
    /// Die Fallunterscheidung ist vollstaendig: entweder es gibt einen
    /// angesteuerten Treffer, dann steht seine Nummer und die Gesamtzahl da,
    /// oder es gibt keinen, dann steht der Suchtext da und der Nutzer weiss,
    /// wonach vergeblich gesucht wurde.
    #[must_use]
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
    ergebnis: Result<Gelesen, Abweisung>,
    /// Der Stempel, **vor** dem Lesen erhoben; siehe [`Ladevorgang::starten`].
    stempel: Option<Stempel>,
}

/// Eine gelesene Datei: der Stand und der Schutz, unter dem er gesichert wird.
///
/// **Beide reisen zusammen und nur im Erfolgsfall**, damit ein Schluessel nie
/// ohne den Stand ankommt, zu dem er gehoert, und nie an einer Abweisung
/// haengt.
#[derive(Debug)]
struct Gelesen {
    stand: String,
    schutz: Schutz,
}

/// Wie der gehaltene Stand auf die Platte geht (Schritt 5.4a des Plans der
/// krkhome-Arbeit, C7).
///
/// **Der Schutz entsteht allein beim Lesen und reist mit dem Stand**
/// ([`Gelesen`]); [`Editormodell::uebernehmen`] setzt ihn zusammen mit Pfad und
/// Stand, und [`Editormodell::schliessen`] wirft ihn fort. Einen Setzer daneben
/// gibt es nicht, also kann der Editor keinen Stand halten, dessen Schutz aus
/// einer anderen Datei stammt.
///
/// Getilgt wird beim Fortwerfen nichts: der Spec sagt kein Tilgen des
/// Speichers zu, und die Textflaeche haelt den Klartext ohnehin.
#[derive(Debug, Default)]
enum Schutz {
    /// Jede Datei ausser `.secrets.txt`: gesichert wird der Stand in seiner
    /// Sicherungsform.
    #[default]
    Klartext,
    /// `.secrets.txt` im erkannten Heimordner: gesichert wird allein das
    /// Chiffrat aus [`Chiffrat::verschliessen`], mit dem gehaltenen Schluessel
    /// und ohne neue Ableitung.
    Verschluesselt {
        /// Der Schluessel aus dem Oeffnen oder, bei einer leeren Datei, aus
        /// der neu festgelegten PIN; jede Sicherung verschliesst mit ihm.
        schluessel: Schluessel,
        /// Die PIN, allein fuer den Vergleich der alten PIN beim Aendern
        /// ([`Editormodell::pin_aendern`], Schritt 5.5). Nach dem
        /// Bedrohungsmodell unerheblich, weil der Speicher den Klartext
        /// ohnehin haelt.
        pin: Pin,
        /// Ob auf der Platte ein Kopf mit dieser PIN steht: gesetzt, wenn die
        /// Datei nicht leer gelesen wurde oder ein Sichern gelungen ist.
        ///
        /// **Eine Buchfuehrung und keine Frage an die Platte**, weil
        /// [`Editormodell::pin_aenderbar`] bei jedem Tastendruck und jeder
        /// Menueausgrauung gefragt wird und dort kein Lesen auf dem Hauptfaden
        /// stehen soll. Aendert jemand die Datei von aussen, geht der Wert
        /// daneben; der Befehl „PIN ändern" aus Schritt 5.5 prueft dann ueber
        /// `fremd_geaendert` wie jedes Sichern und weist ab. Nach einem
        /// gelungenen Aendern steht er wieder, denn der neue Kopf ist
        /// geschrieben.
        kopf_auf_platte: bool,
    },
}

/// Was der Arbeitsfaden lesen soll.
///
/// **Entschieden wird in [`Editormodell::oeffnen`] und nicht auf dem Faden**:
/// ob ein Pfad `.secrets.txt` im erkannten Heimordner ist, fragt das Modell,
/// bevor es einen Faden startet, und `.secrets.txt` ohne PIN bekommt gar
/// keinen.
#[derive(Debug, Clone, Copy)]
enum Leseauftrag {
    /// Eine gewoehnliche Textdatei, ueber `krk_core::text::datei::oeffnen`.
    Text,
    /// `.secrets.txt` mit der eingegebenen PIN; `datei::oeffnen` wird dafuer
    /// nie erreicht.
    Geheimnisse(Pin),
}

/// Die Bytes, die von einem verschluesselten Stand auf die Platte gehen.
///
/// **Ein eigener Typ, damit der Schreibweg fuer `.secrets.txt` keinen
/// Klartext annehmen kann**: [`Chiffrat::schreiben`] nimmt allein diesen Wert,
/// und er entsteht an genau einer Stelle, in [`Chiffrat::verschliessen`], aus
/// `tresor::verschliessen`. Das Feld ist privat; die Probe
/// `kein_weg_schreibt_klartext_nach_secrets_txt` haelt die eine Baustelle.
#[derive(Debug)]
struct Chiffrat(Vec<u8>);

impl Chiffrat {
    /// Der Stand in seiner Sicherungsform, verschlossen mit dem gehaltenen
    /// Schluessel: frische Nonce, Salz und Parameter aus dem Schluessel,
    /// **keine Ableitung**. Die Sicherungsform zuerst, damit der Rundlauf
    /// derselbe ist wie bei `notes.txt` (ein Stand ohne Schlussumbruch kommt
    /// mit einem zurueck).
    fn verschliessen(stand: &str, schluessel: &Schluessel) -> Result<Chiffrat, Tresorfehler> {
        let form = datei::sicherungsform(stand);
        let bytes = tresor::verschliessen(form.as_bytes(), schluessel)?;
        Ok(Chiffrat(bytes))
    }

    /// Der Schreibweg des Alltags: atomar, erst in die Nachbardatei, dann
    /// `rename`. Die Nachbardatei traegt damit zu keinem Zeitpunkt Klartext.
    fn schreiben(&self, ziel: &Path) -> io::Result<()> {
        atomar::schreiben(ziel, &mut self.0.as_slice())
    }
}

/// Ein laufender Wechsel der PIN: die Ableitung des neuen Schluessels auf
/// einem benannten Faden (Schritt 5.5 des Plans der krkhome-Arbeit).
///
/// **Derselbe Zuschnitt wie [`Ladevorgang`]**: ein Faden je Wechsel, genau
/// eine Meldung ueber einen `sync_channel(1)`, abgeholt vom selben
/// Einzugstakt ueber [`Editormodell::pinwechsel_einziehen`]. Auf dem Faden
/// laeuft allein die Ableitung, [`tresor::neuer_schluessel`] mit frischem Salz
/// und den Parametern des Codes; gelesen, entschluesselt und geschrieben wird
/// danach auf dem rufenden Faden, damit der Stand, der umgeschluesselt wird,
/// der ist, der im Augenblick des Schreibens auf der Platte steht, und nicht
/// der vor einer halben Sekunde.
#[derive(Debug)]
struct Pinwechsel {
    empfaenger: Receiver<Result<Schluessel, Tresorfehler>>,
    /// Die Datei, deren PIN geaendert wird.
    pfad: PathBuf,
    /// Die neue PIN; sie ersetzt die gehaltene, sobald der neue Kopf steht.
    neue: Pin,
}

impl Pinwechsel {
    /// Startet die Ableitung fuer die neue PIN.
    ///
    /// **Das Startergebnis wird nicht fallengelassen**: ohne Faden kaeme nie
    /// eine Meldung, und der Nutzer bekommt den Grund, statt auf eine Antwort
    /// zu warten, die nicht kommt (`kein_fadenstart_im_baum_wirft_seinen_rueckgabewert_weg`).
    fn starten(pfad: PathBuf, neue: Pin) -> Result<Self, String> {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        thread::Builder::new()
            .name("krk-pin".to_owned())
            .spawn(move || {
                let _ = SyncSender::send(&sender, tresor::neuer_schluessel(&neue));
            })
            .map_err(|fehler| {
                format!("die neue PIN lässt sich nicht ableiten: {fehler}; {PIN_BLEIBT}")
            })?;
        Ok(Self {
            empfaenger,
            pfad,
            neue,
        })
    }
}

/// Wie ein Wechsel der PIN ausgegangen ist (Schritt 5.5).
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "der Ausgang ist die einzige Antwort auf „PIN ändern“; fallengelassen erfaehrt der Nutzer nicht, ob die neue PIN gilt"]
pub enum Pinwechselausgang {
    /// Der neue Kopf steht auf der Platte, und ab jetzt oeffnet allein die neue
    /// PIN die Datei.
    Geaendert(PathBuf),
    /// Nichts ist geschrieben, und die alte PIN gilt weiter; der Satz sagt,
    /// warum.
    Gescheitert(String),
}

/// Der Satzschluss jeder Abweisung von „PIN ändern".
const PIN_BLEIBT: &str = "die PIN bleibt, wie sie war";

/// Die Grenze fuer das Lesen einer `.secrets.txt`: die Editorgrenze fuer den
/// Klartext, dazu der Kopf und die 16 Byte Pruefwert von Poly1305 (Kopftabelle
/// im Modulkopf von `krk_core::heimordner::tresor`). Mehr kann ein Chiffrat
/// nicht tragen, dessen Klartext der Editor annimmt.
const GEHEIMNISGRENZE: u64 = datei::EDITORGRENZE + tresor::KOPFLAENGE as u64 + 16;

/// Der Satz der Statuszeile, wenn `cmd+d` im Editor an `.secrets.txt` eine
/// Textmarke anlegen soll; die Regel steht an
/// [`Editormodell::textmarke_verweigert`].
pub const KEINE_TEXTMARKE_IN_GEHEIMNISSEN: &str = "für .secrets.txt legt KRK keine Textmarke an: \
     sie schriebe eine Zeile der Geheimnisse im Klartext in die Lesezeichen";

/// Der Grund, aus dem `.secrets.txt` ohne PIN nicht geoeffnet wird.
const OHNE_PIN: &str = "sie ist verschlüsselt und öffnet sich allein mit der PIN";

/// Eine Abweisung fuer `.secrets.txt`, im Wortlaut des Editors: der Pfad,
/// "laesst sich nicht im Editor oeffnen:" und der Grund.
///
/// **`KeinGueltigesZiel` und kein neuer Wert**, weil der Satz passt und die
/// Behandlung dieselbe ist: der bisherige Stand bleibt, der Grund geht in die
/// Statuszeile. Fuer eine falsche PIN und ein veraendertes Byte ist der Grund
/// derselbe, `Oeffnungsfehler::meldung`.
fn gesperrt(pfad: &Path, grund: String) -> Abweisung {
    Abweisung::KeinGueltigesZiel {
        pfad: pfad.to_path_buf(),
        grund,
        mangel: false,
    }
}

/// Liest eine gewoehnliche Textdatei; die eine Stelle, die dafuer
/// `krk_core::text::datei::oeffnen` ruft.
fn text_lesen(pfad: &Path) -> Result<Gelesen, Abweisung> {
    datei::oeffnen(pfad).map(|stand| Gelesen {
        stand,
        schutz: Schutz::Klartext,
    })
}

/// Liest `.secrets.txt` mit der PIN und leitet dabei ab; laeuft auf dem
/// Arbeitsfaden, weil die Ableitung rund eine halbe Sekunde kostet.
///
/// **Die Leere wird an den gelesenen Bytes entschieden und nicht an einem
/// `stat(2)` davor**: null Bytes heisst neue Datei, und dann entsteht der
/// Schluessel aus der eben festgelegten PIN mit frischem Salz. Geschrieben
/// wird hier nichts; eine leere Datei bleibt leer, bis gesichert wird.
///
/// Jeder Fehler ist eine Abweisung mit genau einem Grund: fuer eine falsche PIN
/// und eine veraenderte Datei derselbe, fuer einen beschaedigten Kopf der
/// Schaden, fuer ein Versagen des Systems dessen Satz.
fn geheimnisse_lesen(pfad: &Path, pin: &Pin) -> Result<Gelesen, Abweisung> {
    let bytes = datei::bis_zur_grenze_lesen(pfad, GEHEIMNISGRENZE).map_err(|hindernis| {
        let (grund, mangel) = match hindernis {
            Lesehindernis::ZuGross => ("sie ist zu groß für den Editor", false),
            Lesehindernis::KeineDatei => ("das ist keine gewöhnliche Datei", false),
            Lesehindernis::Deskriptormangel => ("KRK hat keinen freien Dateizugriff mehr", true),
            Lesehindernis::Fehler => ("sie lässt sich nicht lesen", false),
        };
        Abweisung::KeinGueltigesZiel {
            pfad: pfad.to_path_buf(),
            grund: grund.to_owned(),
            mangel,
        }
    })?;
    let kopf_auf_platte = !bytes.is_empty();
    let (klartext, schluessel) = if bytes.is_empty() {
        let schluessel =
            tresor::neuer_schluessel(pin).map_err(|fehler| gesperrt(pfad, fehler.meldung()))?;
        (Vec::new(), schluessel)
    } else {
        let geoeffnet =
            tresor::oeffnen(&bytes, pin).map_err(|fehler| gesperrt(pfad, fehler.meldung()))?;
        (geoeffnet.klartext, geoeffnet.schluessel)
    };
    let stand = datei::einlesen(klartext).ok_or_else(|| Abweisung::NichtAlsTextLesbar {
        pfad: pfad.to_path_buf(),
    })?;
    Ok(Gelesen {
        stand,
        schutz: Schutz::Verschluesselt {
            schluessel,
            pin: *pin,
            kopf_auf_platte,
        },
    })
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
    fn starten(pfad: PathBuf, auftrag: Leseauftrag) -> Self {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        let fuer_faden = pfad.clone();
        let ergebnis = thread::Builder::new()
            .name("krk-editor".to_owned())
            .spawn(move || {
                let stempel = Stempel::von_pfad(&fuer_faden);
                let ergebnis = match auftrag {
                    Leseauftrag::Text => text_lesen(&fuer_faden),
                    Leseauftrag::Geheimnisse(pin) => geheimnisse_lesen(&fuer_faden, &pin),
                };
                let _ = SyncSender::send(&sender, Geladen { ergebnis, stempel });
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
/// **Vier Werte aus dem Modell, ueberschneidungsfrei und vollstaendig**, dazu
/// [`Self::ZelleAbgewiesen`] und [`Self::PinVerlangt`], die allein die Ansicht
/// erzeugt. Entweder der Editor
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
#[must_use = "der Ausgang sagt, ob die Datei angenommen wurde; fallengelassen bleibt eine Abweisung stumm"]
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
    /// Eine Zelle der Eintragstabelle liess sich nicht uebernehmen, und das
    /// Oeffnen unterbleibt, bevor gelesen wird (Schritt 3.2b des Plans der
    /// krkhome-Arbeit). Der Satz sagt, warum.
    ///
    /// **Dieses Modell erzeugt den Wert nie**; er entsteht in
    /// `Editorbereich::datei_oeffnen`, das vor dem Oeffnen die laufende Zelle
    /// uebernimmt, und geht durch dieselbe Senke wie jeder Ausgang, damit es
    /// eine Behandlung gibt und nicht zwei. Ein eigener Wert und nicht
    /// `Abgewiesen`, weil jene Abweisung eine Datei nennt und diese keine.
    ZelleAbgewiesen(String),
    /// Die Datei ist `.secrets.txt` im erkannten Heimordner, und bevor gelesen
    /// wird, gehoert die PIN erfragt (Schritt 5.4b der krkhome-Arbeit, C7.10).
    ///
    /// **Dieses Modell erzeugt den Wert nie**, wie [`Self::ZelleAbgewiesen`]:
    /// er entsteht in `Editorbereich::datei_oeffnen` und geht durch dieselbe
    /// Senke, damit jeder Weg in den Editor — F4, `cmd+e` aus Liste und
    /// Vorschau, der Sprung auf eine Textmarke — an derselben Stelle beim Blatt
    /// ankommt. Nichts ist gelesen, nichts hat sich bewegt; der Empfaenger
    /// zeigt das Blatt in der genannten Form und reicht die PIN an
    /// `Editorbereich::geheimnisse_oeffnen` zurueck.
    PinVerlangt {
        /// Die Datei, fuer die gefragt wird.
        pfad: PathBuf,
        /// Festlegen oder eingeben, nach der Groesse der Datei.
        form: Pinform,
    },
}

/// In welcher Form das PIN-Blatt fragt (C7).
///
/// **Die ersten zwei entscheidet die Groesse der Datei**, wie der Spec es
/// sagt: null Bytes heisst, es gibt noch keine PIN, und der Nutzer legt eine
/// fest, zweimal einzugeben; jede andere Groesse heisst, er gibt die PIN ein.
/// Das Modell nimmt in beiden Faellen dieselbe PIN und entscheidet beim Lesen
/// noch einmal an den gelesenen Bytes; weicht die Platte in der Spanne
/// dazwischen ab, gilt das Lesen.
///
/// **Die dritte, [`Self::Aendern`], entscheidet der Befehl** „PIN ändern"
/// (Schritt 5.5) und nie die Groesse: [`Self::nach_groesse`] liefert sie nicht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pinform {
    /// Die Datei ist leer: eine neue PIN festlegen, zweimal eingegeben.
    Festlegen,
    /// Die Datei traegt einen Kopf: die PIN eingeben.
    Eingeben,
    /// Die offene Datei traegt einen Kopf, und ihre PIN wird geaendert: die
    /// alte einmal, die neue zweimal (Schritt 5.5).
    Aendern,
}

impl Pinform {
    /// Die Form zur Groesse der Datei; `None` heisst, die Groesse liess sich
    /// nicht erheben.
    ///
    /// **Ohne Groesse wird eingegeben und nicht festgelegt.** Eine Datei, die
    /// sich nicht befragen laesst, wird auch das Lesen danach abweisen, und
    /// dann mit seinem eigenen Grund; ein Festlegen dagegen verspraeche eine
    /// neue Datei, die es womoeglich nicht ist.
    #[must_use]
    pub fn nach_groesse(groesse: Option<u64>) -> Self {
        match groesse {
            Some(0) => Pinform::Festlegen,
            Some(_) | None => Pinform::Eingeben,
        }
    }
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
/// neunte Abnahmekriterium von C4 zwei Sachen zugleich verlangt: den Grund in
/// der Statuszeile und einen Stand, der stehen bleibt. Wer beides in "es hat
/// nicht geklappt" zusammenzoege, koennte das erste nicht liefern.
///
/// **`#[must_use]`, und der Grund steht in der Zusage darueber.** Der Wert
/// traegt bei [`Self::Gescheitert`] den einzigen Satz, den der Nutzer ueber
/// einen misslungenen Schreibvorgang je zu sehen bekommt; wer ihn fallen laesst,
/// hinterlaesst genau den stillen Fehlschlag, den die Zusage ausschliesst — eine
/// Datei, die ungesichert weiterlaeuft, ohne dass ein Wort darueber faellt. Bis
/// zum 260904 hielt diese Zusage allein die Prosa dieses Kommentars, und ein
/// nackter Ruf an [`Editormodell::sichern`] uebersetzte gruen
/// (`shared/issues/260904-1827_*_sichern-auf-einem-netzlaufwerk-schlaegt-still-fehl-*`).
/// Wer den Wert bewusst nicht braucht, schreibt `let _ =` davor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "der Grund eines gescheiterten Sicherns gehoert in die Statuszeile"]
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
    /// Eine Zelle der Eintragstabelle liess sich nicht uebernehmen; es wurde
    /// nicht geschrieben, und ein Anlass, der auf dieses Sichern gewartet hat,
    /// unterbleibt (Schritt 3.2b des Plans der krkhome-Arbeit).
    ///
    /// **Dieses Modell erzeugt den Wert nie**: die Zelle kennt allein
    /// `Editorbereich::sichern`, das sie vor dem Schreiben uebernimmt. Er steht
    /// neben [`Self::Gescheitert`] und nicht darin, damit der vollstaendige
    /// `match` beim Anwendungsdelegierten ihn eigens einordnen muss: ein
    /// Sichern, das nie angefangen hat, ist kein gescheitertes Schreiben.
    ZelleAbgewiesen(String),
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
    /// Wie der Stand auf die Platte geht: im Klartext oder verschluesselt
    /// (C7). Gesetzt allein zusammen mit dem Stand; siehe [`Schutz`].
    schutz: Schutz,
    /// Der geteilte Wert der Erkennung von `~/krkhome/`, aus dem
    /// [`Self::typ`] beim Uebernehmen einer gelesenen Datei entsteht.
    ///
    /// Ein Griff und keine Abschrift, damit ein F2, das die aufgeloeste Form
    /// erneuert, auch hier gilt; siehe [`crate::heimgriff`].
    heim: Heimgriff,
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
    /// Der laufende Wechsel der PIN, falls einer laeuft (Schritt 5.5).
    ///
    /// **Er ueberlebt einen Dateiwechsel und das Schliessen**, und zwar mit
    /// Absicht: der Nutzer hat ihn verlangt, und sein Ausgang gehoert in die
    /// Statuszeile, auch wenn es nur der Satz ist, dass die Datei nicht mehr
    /// offen ist und die PIN bleibt. [`Editormodell::pinwechsel_einziehen`]
    /// fragt deshalb im Augenblick des Schreibens, ob der Editor die Datei
    /// noch haelt.
    pinwechsel: Option<Pinwechsel>,
}

impl Editormodell {
    /// Ein Editor, der keine Datei haelt.
    ///
    /// `heim` ist der Griff, den der Anwendungsdelegierte einmal baut und an
    /// jeden Frager der Erkennung reicht.
    #[must_use]
    pub fn neu(heim: Heimgriff) -> Self {
        Self {
            heim,
            ..Self::default()
        }
    }

    /// Die gehaltene Datei; `None`, solange keine gehalten wird.
    #[must_use]
    pub fn pfad(&self) -> Option<&Path> {
        self.pfad.as_deref()
    }

    /// Ob der Editor eine Datei haelt.
    #[must_use]
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
    #[must_use]
    pub fn haelt_bereits(&self, pfad: &Path) -> bool {
        self.pfad.as_deref() == Some(pfad)
    }

    /// Der gehaltene Stand.
    #[must_use]
    pub fn stand(&self) -> &str {
        &self.stand
    }

    /// Ob der Editor Aenderungen haelt, die nicht in der Datei stehen (C4).
    ///
    /// Das ist die Frage, die an den drei Anlaessen aus C4 gestellt wird und an
    /// der die Anzeige aus dem zweiten Abnahmekriterium haengt. Warum sie eine
    /// Marke liest und keinen Vergleich fuehrt, steht im Modulkopf.
    #[must_use]
    pub fn hat_ungesicherten_stand(&self) -> bool {
        self.abweichung
    }

    /// Ob sich die PIN der gehaltenen Datei aendern laesst: der Editor haelt
    /// `.secrets.txt` entsperrt, und auf der Platte steht ihr Kopf (Schritt
    /// 5.4b der krkhome-Arbeit, gelesen ab 5.5).
    ///
    /// Eine leere Datei, deren PIN eben festgelegt und noch nie gesichert
    /// wurde, antwortet nein: ihre PIN steht in keinem Kopf, und das naechste
    /// Oeffnen fragt ohnehin nach einer neuen. Gefragt wird die Buchfuehrung im
    /// [`Schutz`] und nicht die Platte; der Grund steht dort.
    #[must_use]
    pub fn pin_aenderbar(&self) -> bool {
        matches!(
            self.schutz,
            Schutz::Verschluesselt {
                kopf_auf_platte: true,
                ..
            }
        )
    }

    /// Warum an der gehaltenen Datei keine Textmarke entsteht, oder `None`,
    /// wenn sie entstehen darf (C7.8 der krkhome-Arbeit).
    ///
    /// **Eine Textmarke traegt ihre Zeile im Klartext**, als `zeileninhalt` in
    /// `bookmarks.toml`, und haelt der Editor `.secrets.txt`, ist diese Zeile
    /// ein Geheimnis. Die Antwort ist deshalb fuer die Geheimnisse immer der
    /// eine Satz [`KEINE_TEXTMARKE_IN_GEHEIMNISSEN`]
    /// (`issues/260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md`).
    ///
    /// Ob es eine Geheimnisdatei ist, sagt [`Self::haelt_geheimnisse`].
    #[must_use]
    pub fn textmarke_verweigert(&self) -> Option<&'static str> {
        self.haelt_geheimnisse()
            .then_some(KEINE_TEXTMARKE_IN_GEHEIMNISSEN)
    }

    /// Ob der Editor `.secrets.txt` haelt (C7 der krkhome-Arbeit).
    ///
    /// **Die eine Antwort fuer jeden Weg, auf dem Inhalt der Geheimnisse aus
    /// KRK hinausgelangen koennte**: die Textmarke
    /// ([`Self::textmarke_verweigert`]) und das Tastenprotokoll, das waehrend
    /// dessen verdeckt schreibt (`Anwendungsdelegierter::tasten_verdeckt`).
    ///
    /// Gefragt werden zwei Dinge, und jedes allein genuegt: der [`Schutz`],
    /// der einen Schluessel haelt, und die Erkennung des Pfads. Der Schutz
    /// antwortet auch dann, wenn ein F2 die aufgeloeste Form des Heimordners
    /// inzwischen anders fuehrt; die Erkennung antwortet auch fuer eine
    /// leere `.secrets.txt`, deren Stand noch gar nichts traegt.
    #[must_use]
    pub fn haelt_geheimnisse(&self) -> bool {
        matches!(self.schutz, Schutz::Verschluesselt { .. })
            || self
                .pfad
                .as_deref()
                .is_some_and(|pfad| self.ist_geheimnisdatei(pfad))
    }

    /// Welche Ansicht gewaehlt ist (C3).
    #[must_use]
    pub fn ansicht(&self) -> Ansicht {
        self.ansicht
    }

    /// Was der Pfad ueber die gehaltene Datei sagt (C3).
    #[must_use]
    pub fn typ(&self) -> Dateityp {
        self.typ
    }

    /// Der laufende Suchlauf (C5).
    #[must_use]
    pub fn suchlauf(&self) -> Option<&Suchlauf> {
        self.suchlauf.as_ref()
    }

    /// Wechselt zwischen Rohansicht und Formatansicht und liefert die neue (C3).
    ///
    /// **Fasst den Stand nicht an**, und das ist der ganze Punkt: ein
    /// Ansichtswechsel kann keine ungesicherte Aenderung verlieren, weil er
    /// nichts anfasst, worin eine stecken koennte. Weder [`Self::stand`] noch
    /// die Abweichungsmarke noch der Suchlauf aendern sich.
    #[must_use]
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
    /// offen, ist das einer der drei Anlaesse aus C4. Die Nachfrage gehoert
    /// nicht vor diesen Ruf, sondern hinter die Pruefung, die auf dem
    /// Arbeitsfaden laeuft: [`Self::einziehen`] liefert dann
    /// [`Ladeausgang::Zurueckgehalten`], und der Aufrufer fragt. Der Grund und
    /// das Bild dazu stehen im Modulkopf.
    ///
    /// # Die Datei, die der Editor schon haelt, wird nicht neu gelesen
    ///
    /// Haelt der Editor genau diesen Pfad, kehrt die Funktion mit
    /// [`Ladeausgang::SchonOffen`] zurueck, **bevor** sie einen Faden startet,
    /// und fasst am gehaltenen Stand nichts an. Ohne diese Zeile ist ein zweites F4 auf dieselbe
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
    /// **Ein laufendes Lesen gibt die Abkuerzung trotzdem auf, und das ist das
    /// eine, was sie anfasst.** Der Nutzer hat die gehaltene Datei verlangt,
    /// also gehoert das Lesen der anderen niemandem mehr. Aufgegeben wird es
    /// ueber denselben Mechanismus, den der gewoehnliche Weg eine Zeile weiter
    /// unten in Anspruch nimmt: der Vorgang faellt, sein Empfaenger mit ihm, und
    /// das `send` des ueberholten Fadens scheitert still.
    ///
    /// Damit gilt fuer beide Ausgaenge dieser Funktion derselbe Satz:
    /// **hoechstens ein Lesen ist offen, und es ist das zuletzt begonnene.**
    /// Gemeint ist das Lesen und nicht die gelesene Datei, die auf die Nachfrage
    /// aus C4 wartet: die ist fertig gelesen, und ihr Ausgang steht beim
    /// Aufrufer statt bei einem Faden. Bis zum 260810 galt der Satz nur fuer den
    /// gewoehnlichen Weg, und die Abkuerzung war der eine Fall, in dem **zwei**
    /// Ladeausgaenge zu einer Folge von Oeffnungen gehoerten: `SchonOffen` kam
    /// unverzueglich, und danach lieferte der stehengelassene Faden noch einmal
    /// fuer die andere Datei. Der letzte Befehl des Nutzers war damit still
    /// ueberschrieben; verloren ging dabei kein Text, weil ein ungesicherter
    /// Stand die andere Datei ueber [`Ladeausgang::Zurueckgehalten`] durch die
    /// Nachfrage aus C4 gefuehrt haette, wohl aber die Wirkung des Befehls. Der
    /// Datensatz ist
    /// `issues/260810-1029_*_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`.
    ///
    /// **Der Preis steht hier und wird nicht verschwiegen:** F4 auf die schon
    /// gehaltene Datei liest sie damit auch dann nicht neu, wenn sie sich von
    /// aussen geaendert hat. Ein Befehl zum Neulesen gibt es nicht, und C2 sagt
    /// keinen zu; die Aenderung von aussen traegt S31. Die Nachfrage aus C4
    /// greift auf dieser Abkuerzung nicht, und sie soll es nicht: es wird
    /// nichts gelesen und nichts ersetzt, also ist auch nichts zu verlieren.
    ///
    /// # `.secrets.txt` oeffnet sich allein mit der PIN
    ///
    /// **Die Sperre sitzt hier und nicht bei einem Einstieg**, damit F4, der
    /// Uebergang aus der Vorschau, `cmd+e` und jeder kuenftige Weg sie erben,
    /// ohne sie zu kennen (Schritt 5.4a des Plans der krkhome-Arbeit). Gefragt
    /// wird die Erkennung ([`Self::ist_geheimnisdatei`]), **bevor** ein Faden
    /// startet: ohne `pin` kommt `.secrets.txt` sofort als
    /// [`Ladeausgang::Abgewiesen`] zurueck, ohne dass ein Byte gelesen wurde,
    /// und `krk_core::text::datei::oeffnen` wird fuer sie nie erreicht. Mit
    /// `pin` liest und leitet der Faden ab ([`geheimnisse_lesen`]). Eine `pin`
    /// fuer eine andere Datei weist ab, statt sie zu uebergehen: wer eine PIN
    /// mitgibt, meint eine verschluesselte Datei, und ein stilles Oeffnen im
    /// Klartext waere die falsche Seite des Irrtums.
    ///
    /// **Die gehaltene Datei bleibt davon unberuehrt**: haelt der Editor
    /// `.secrets.txt` schon, gilt die PIN, solange die Datei offen ist, und die
    /// Abkuerzung darueber greift vor der Sperre.
    #[must_use]
    pub fn oeffnen(&mut self, pfad: &Path, pin: Option<Pin>) -> Option<Ladeausgang> {
        if self.haelt_bereits(pfad) {
            // Der Nutzer hat die gehaltene Datei verlangt; ein Lesen, das noch
            // laeuft, gehoert damit niemandem mehr. Ohne diese Zeile gehoerten
            // zwei Ladeausgaenge zu einer Folge von Oeffnungen; siehe oben.
            self.ladevorgang = None;
            return Some(Ladeausgang::SchonOffen);
        }
        let auftrag = match (self.ist_geheimnisdatei(pfad), pin) {
            (false, None) => Leseauftrag::Text,
            (true, Some(pin)) => Leseauftrag::Geheimnisse(pin),
            (true, None) => {
                // Auch hier gehoert ein laufendes Lesen niemandem mehr: der
                // letzte Befehl des Nutzers galt dieser Datei.
                self.ladevorgang = None;
                return Some(Ladeausgang::Abgewiesen(gesperrt(pfad, OHNE_PIN.to_owned())));
            }
            (false, Some(_)) => {
                self.ladevorgang = None;
                return Some(Ladeausgang::Abgewiesen(gesperrt(
                    pfad,
                    "sie ist keine verschlüsselte Datei im Notizordner".to_owned(),
                )));
            }
        };
        self.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf(), auftrag));
        None
    }

    /// Gibt ein laufendes Lesen auf, ohne etwas anderes anzufassen.
    ///
    /// Der Weg von `Editorbereich::datei_oeffnen`, wenn es fuer
    /// `.secrets.txt` erst nach der PIN fragt: der letzte Befehl des Nutzers
    /// gilt dieser Datei, also gehoert ein Lesen, das noch fuer eine andere
    /// laeuft, niemandem mehr. Derselbe Satz wie an [`Self::oeffnen`]:
    /// hoechstens ein Lesen ist offen, und es ist das zuletzt begonnene.
    pub fn laden_aufgeben(&mut self) {
        self.ladevorgang = None;
    }

    /// Ob der Pfad `.secrets.txt` im erkannten Heimordner ist.
    ///
    /// Die eine Frage, an der Sperre und Sicherungsweg haengen; gestellt ueber
    /// [`Heimordner::sonderdatei`], die eine Stelle der Erkennung, ohne
    /// Systemaufruf.
    fn ist_geheimnisdatei(&self, pfad: &Path) -> bool {
        heimgriff::lesen(&self.heim)
            .and_then(|heim| heim.sonderdatei(pfad))
            .is_some_and(|sonderdatei| sonderdatei == Sonderdatei::Geheimnisse)
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
    ///
    /// **Eine im Klartext gelesene `.secrets.txt` wird nicht aufgenommen.** Die
    /// Sperre in [`Self::oeffnen`] fragt die Erkennung beim Start des Lesens;
    /// ein F2 dazwischen kann die aufgeloeste Form erneuern, und dann erkennt
    /// die Frage hier, was sie dort noch nicht erkannte. Der Editor hielte
    /// sonst `.secrets.txt` ohne Schluessel, und abgewiesen wird deshalb, bevor
    /// irgendein Feld sich bewegt.
    fn uebernehmen(&mut self, pfad: PathBuf, geladen: Geladen) -> Ladeausgang {
        match geladen.ergebnis {
            Ok(Gelesen { stand, schutz }) => {
                if matches!(schutz, Schutz::Klartext) && self.ist_geheimnisdatei(&pfad) {
                    return Ladeausgang::Abgewiesen(gesperrt(&pfad, OHNE_PIN.to_owned()));
                }
                self.typ = Dateityp::von_pfad(&pfad, heimgriff::lesen(&self.heim).as_ref());
                self.pfad = Some(pfad);
                self.stand = stand;
                self.schutz = schutz;
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
    #[must_use]
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

    /// Ob ein Ladevorgang laeuft.
    #[must_use]
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
    #[must_use]
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
    ///
    /// **Der Wert laesst sich nicht still fallenlassen**, und das ist eine
    /// Erzwingung und keine Bitte. Er ist die **einzige** Meldung, dass Stand
    /// und Textbestand auseinanderliefen; es gibt keine zweite Stelle, an der
    /// ein Vergessen auffiele. Genau diese Lage war der Defekt `260810-0215`,
    /// und der Bau war dabei gruen. Das `#[must_use]` macht daraus einen
    /// Uebersetzerfehler, wie die vollstaendigen Fallunterscheidungen dieses
    /// Programms es an anderen Stellen tun. Wer die Meldung wirklich nicht
    /// braucht — die Pruefungen am Dateiende, die den Stand danach selbst
    /// ansehen —, schreibt `let _ =` davor und sagt damit ausdruecklich, dass er
    /// sie nicht braucht. Der Datensatz ist
    /// `issues/260810-0423_*_der-rueckgabewert-von-bearbeiten-laesst-sich-still-fallenlassen.md`.
    #[must_use = "wandelte das Bearbeiten, ist die Textflaeche nachzuziehen"]
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
    /// Statuszeile. Das ist die eine Haelfte des achten Abnahmekriteriums von
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
    ///
    /// # `.secrets.txt` geht allein als Chiffrat auf die Platte
    ///
    /// Der [`Schutz`] entscheidet und nicht der Pfad: haelt der Editor einen
    /// Schluessel, geht der Stand in seiner Sicherungsform durch
    /// [`Chiffrat::verschliessen`], mit frischer Nonce und **ohne neue
    /// Ableitung**, und die Bytes gehen ueber [`Chiffrat::schreiben`] an
    /// `ablage::atomar::schreiben`. Die Stempelpruefung davor ist dieselbe wie
    /// bei jeder Datei.
    ///
    /// **Der Klartextweg fragt die Erkennung ein zweites Mal**, unmittelbar vor
    /// dem Schreiben: haelt der Editor keinen Schluessel und ist der Pfad
    /// trotzdem `.secrets.txt`, wird nicht geschrieben. Erreichbar ist das nur,
    /// wenn die Erkennung sich nach dem Oeffnen geaendert hat; die Frage kostet
    /// einen Textvergleich und schliesst den Weg, auf dem Klartext in die Datei
    /// kaeme.
    #[must_use = "der Ausgang traegt den Grund eines gescheiterten Sicherns; fallengelassen glaubt der Nutzer, die Datei stehe auf der Platte"]
    pub fn sichern(&mut self) -> Sicherungsausgang {
        self.sichern_ueber(|ziel, chiffrat| chiffrat.schreiben(ziel))
    }

    /// [`Self::sichern`] mit einem hereingereichten Schreibweg fuer das
    /// Chiffrat.
    ///
    /// **Die Naht gibt es allein fuer die Probe**, die die Bytes am Schreibweg
    /// abfaengt und das Bild der Nachbardatei vor dem `rename` liest; der
    /// Alltag reicht [`Chiffrat::schreiben`] herein. Hereingereicht wird ein
    /// Weg fuer ein [`Chiffrat`] und keiner fuer Bytes, damit auch die Naht
    /// keinen Klartext annehmen kann.
    fn sichern_ueber(
        &mut self,
        chiffrat_schreiben: impl FnOnce(&Path, &Chiffrat) -> io::Result<()>,
    ) -> Sicherungsausgang {
        let Some(pfad) = self.pfad.clone() else {
            return Sicherungsausgang::NichtsGehalten;
        };
        if self.fremd_geaendert() {
            return Sicherungsausgang::Gescheitert(format!(
                "{} hat sich außerhalb von KRK geändert und wird nicht überschrieben",
                pfad.display()
            ));
        }
        let geschrieben = match &self.schutz {
            Schutz::Klartext => {
                if self.ist_geheimnisdatei(&pfad) {
                    return Sicherungsausgang::Gescheitert(format!(
                        "{} ist verschlüsselt und wird nicht im Klartext geschrieben",
                        pfad.display()
                    ));
                }
                datei::sichern(&pfad, &self.stand)
            }
            Schutz::Verschluesselt { schluessel, .. } => {
                match Chiffrat::verschliessen(&self.stand, schluessel) {
                    Ok(chiffrat) => chiffrat_schreiben(&pfad, &chiffrat),
                    Err(fehler) => {
                        return Sicherungsausgang::Gescheitert(format!(
                            "{} ließ sich nicht sichern: {}",
                            pfad.display(),
                            fehler.meldung()
                        ));
                    }
                }
            }
        };
        match geschrieben {
            Ok(()) => {
                self.abweichung = false;
                self.stempel = Stempel::von_pfad(&pfad);
                // Eine leere `.secrets.txt` traegt nach dem ersten Sichern
                // ihren Kopf; ab jetzt laesst sich ihre PIN aendern.
                if let Schutz::Verschluesselt {
                    kopf_auf_platte, ..
                } = &mut self.schutz
                {
                    *kopf_auf_platte = true;
                }
                Sicherungsausgang::Gesichert(pfad)
            }
            Err(fehler) => Sicherungsausgang::Gescheitert(format!(
                "{} ließ sich nicht sichern: {fehler}",
                pfad.display()
            )),
        }
    }

    /// Ob sich die PIN jetzt aendern liesse, ohne die alte zu kennen; `Err`
    /// traegt den Satz, warum nicht (Schritt 5.5).
    ///
    /// **Die Frage vor dem Blatt**: `Editorbereich::pin_aendern` stellt sie,
    /// bevor der Nutzer drei PINs tippt, und [`Self::pin_aendern`] stellt sie
    /// noch einmal, bevor es die alte vergleicht. Gefragt wird die
    /// Buchfuehrung [`Self::pin_aenderbar`], ein laufender Wechsel und die
    /// fremde Aenderung ueber [`Self::fremd_geaendert`], dieselbe Frage wie
    /// vor jedem Sichern.
    pub fn pin_aenderung_pruefen(&self) -> Result<(), String> {
        let Some(pfad) = self.pfad.as_deref() else {
            return Err(format!("der Editor hält keine Datei; {PIN_BLEIBT}"));
        };
        if !self.pin_aenderbar() {
            return Err(format!(
                "{} trägt noch keine gesicherte PIN; erst sichern, dann ändern",
                pfad.display()
            ));
        }
        if self.pinwechsel.is_some() {
            return Err("die PIN wird schon geändert".to_owned());
        }
        if self.fremd_geaendert() {
            return Err(format!(
                "{} hat sich außerhalb von KRK geändert; {PIN_BLEIBT}",
                pfad.display()
            ));
        }
        Ok(())
    }

    /// Beginnt den Wechsel der PIN der gehaltenen `.secrets.txt` (C7.15,
    /// Schritt 5.5 des Plans der krkhome-Arbeit).
    ///
    /// **Die alte PIN wird gegen die gehaltene verglichen**, bevor irgendetwas
    /// anlaeuft; eine falsche weist ab, und nichts ist geschehen. Dann startet
    /// die Ableitung des neuen Schluessels auf einem benannten Faden
    /// ([`Pinwechsel`]), mit frischem Salz und den Parametern des Codes, also
    /// auch mit angehobenen
    /// (`260926-0050_*_zieht-jede-sicherung-von-secrets-txt-ein-neues-salz-wenn-das-eine-halbe-sekunde-je-cmd-s-kostet.md`,
    /// Moeglichkeit 3). Den Ausgang holt [`Self::pinwechsel_einziehen`] ab.
    ///
    /// **Der Stand des Editors bleibt unberuehrt**: ungesicherte Aenderungen
    /// bleiben ungesichert, denn umgeschluesselt wird der Stand auf der Platte
    /// und nicht der im Editor.
    pub fn pin_aendern(&mut self, alte: Pin, neue: Pin) -> Result<(), String> {
        self.pin_aenderung_pruefen()?;
        let Some(pfad) = self.pfad.clone() else {
            return Err(format!("der Editor hält keine Datei; {PIN_BLEIBT}"));
        };
        let Schutz::Verschluesselt { pin, .. } = &self.schutz else {
            return Err(format!(
                "{} ist nicht verschlüsselt; {PIN_BLEIBT}",
                pfad.display()
            ));
        };
        if *pin != alte {
            return Err(format!("die alte PIN stimmt nicht; {PIN_BLEIBT}"));
        }
        self.pinwechsel = Some(Pinwechsel::starten(pfad, neue)?);
        Ok(())
    }

    /// Ob ein Wechsel der PIN laeuft.
    #[must_use]
    pub fn pin_wechselt(&self) -> bool {
        self.pinwechsel.is_some()
    }

    /// Holt den Ausgang eines Wechsels der PIN ab und schreibt den neuen Kopf
    /// (Schritt 5.5).
    ///
    /// `None`, solange die Ableitung laeuft oder kein Wechsel laeuft. Mit dem
    /// neuen Schluessel geschieht dann, in dieser Reihenfolge und auf dem
    /// rufenden Faden: die Frage nach der fremden Aenderung wie vor jedem
    /// Sichern, das Lesen der Datei, das Entschluesseln mit dem **gehaltenen**
    /// Schluessel ohne Ableitung (`tresor::oeffnen_mit`), das Verschliessen mit
    /// dem neuen ueber [`Chiffrat::verschliessen`] und das Schreiben ueber
    /// [`Chiffrat::schreiben`], also `ablage::atomar::schreiben`. **Zu keinem
    /// Zeitpunkt geht Klartext auf die Platte**: der entschluesselte Stand lebt
    /// allein im Speicher, und der Schreibweg nimmt allein ein [`Chiffrat`].
    ///
    /// Danach steht der Stempel neu, damit das naechste Sichern keine fremde
    /// Aenderung meldet, und der Schutz traegt den neuen Schluessel, die neue
    /// PIN und `kopf_auf_platte`. Scheitert ein Schritt, bleibt alles, wie es
    /// war, Platte und Schutz.
    #[must_use]
    pub fn pinwechsel_einziehen(&mut self) -> Option<Pinwechselausgang> {
        self.pinwechsel_einziehen_ueber(|ziel, chiffrat| chiffrat.schreiben(ziel))
    }

    /// [`Self::pinwechsel_einziehen`] mit einem hereingereichten Schreibweg
    /// fuer das Chiffrat; dieselbe Naht und derselbe Grund wie an
    /// [`Self::sichern_ueber`].
    fn pinwechsel_einziehen_ueber(
        &mut self,
        chiffrat_schreiben: impl FnOnce(&Path, &Chiffrat) -> io::Result<()>,
    ) -> Option<Pinwechselausgang> {
        let geliefert = match self.pinwechsel.as_ref()?.empfaenger.try_recv() {
            Ok(geliefert) => geliefert,
            Err(std::sync::mpsc::TryRecvError::Empty) => return None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.pinwechsel = None;
                return Some(Pinwechselausgang::Gescheitert(format!(
                    "die neue PIN ließ sich nicht ableiten; {PIN_BLEIBT}"
                )));
            }
        };
        let Pinwechsel { pfad, neue, .. } = self.pinwechsel.take()?;
        let neuer = match geliefert {
            Ok(neuer) => neuer,
            Err(fehler) => {
                return Some(Pinwechselausgang::Gescheitert(format!(
                    "{}; {PIN_BLEIBT}",
                    fehler.meldung()
                )));
            }
        };
        Some(
            match self.umschluesseln(&pfad, &neuer, chiffrat_schreiben) {
                Ok(()) => {
                    self.stempel = Stempel::von_pfad(&pfad);
                    self.schutz = Schutz::Verschluesselt {
                        schluessel: neuer,
                        pin: neue,
                        kopf_auf_platte: true,
                    };
                    Pinwechselausgang::Geaendert(pfad)
                }
                Err(grund) => Pinwechselausgang::Gescheitert(grund),
            },
        )
    }

    /// Der Stand auf der Platte, mit dem gehaltenen Schluessel entschluesselt
    /// und mit dem neuen verschlossen geschrieben; fasst das Modell nicht an.
    fn umschluesseln(
        &self,
        pfad: &Path,
        neuer: &Schluessel,
        chiffrat_schreiben: impl FnOnce(&Path, &Chiffrat) -> io::Result<()>,
    ) -> Result<(), String> {
        let name = pfad.display();
        if self.pfad.as_deref() != Some(pfad) {
            return Err(format!("{name} ist nicht mehr offen; {PIN_BLEIBT}"));
        }
        let Schutz::Verschluesselt {
            schluessel: gehalten,
            ..
        } = &self.schutz
        else {
            return Err(format!("{name} ist nicht mehr entsperrt; {PIN_BLEIBT}"));
        };
        if self.fremd_geaendert() {
            return Err(format!(
                "{name} hat sich außerhalb von KRK geändert; {PIN_BLEIBT}"
            ));
        }
        let bytes = datei::bis_zur_grenze_lesen(pfad, GEHEIMNISGRENZE)
            .map_err(|_| format!("{name} lässt sich nicht lesen; {PIN_BLEIBT}"))?;
        let klartext = tresor::oeffnen_mit(&bytes, gehalten)
            .map_err(|fehler| format!("{name}: {}; {PIN_BLEIBT}", fehler.meldung()))?;
        let stand = datei::einlesen(klartext)
            .ok_or_else(|| format!("{name} ist nicht als Text lesbar; {PIN_BLEIBT}"))?;
        let chiffrat = Chiffrat::verschliessen(&stand, neuer)
            .map_err(|fehler| format!("{name}: {}; {PIN_BLEIBT}", fehler.meldung()))?;
        chiffrat_schreiben(pfad, &chiffrat)
            .map_err(|fehler| format!("{name} ließ sich nicht schreiben: {fehler}; {PIN_BLEIBT}"))
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
    ///
    /// **Der Schutz faellt mit**, samt Schluessel und PIN: die PIN gilt,
    /// solange die Datei offen ist, und nicht laenger (C7). Getilgt wird der
    /// Speicher dabei nicht; siehe [`Schutz`].
    pub fn schliessen(&mut self) {
        self.pfad = None;
        self.stand.clear();
        self.schutz = Schutz::Klartext;
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
    ///
    /// # Eine gehaltene Datei ohne Stempel gilt als geaendert
    ///
    /// `pfad = Some` mit `stempel = None` ist kein "der Editor haelt keine
    /// Datei", sondern "der Vergleich hat seine Bezugsgroesse verloren", und es
    /// gibt zwei Wege dorthin: [`Self::sichern`] setzt den Stempel nach dem
    /// Schreiben ueber [`Stempel::von_pfad`] neu, und [`Self::uebernehmen`]
    /// nimmt den vom Lesevorgang erhobenen; beide liefern `None`, wenn
    /// `metadata` oder `modified` scheitert. Die Antwort `false` schaltete die
    /// Zusage aus C4 fuer diese Datei bis zum naechsten Oeffnen **stumm** ab:
    /// [`Self::fremdaenderung_melden`] schwiege, und [`Self::sichern`]
    /// ueberschriebe jede fremde Aenderung ohne Rueckhalt. Deshalb faellt die
    /// Antwort hier auf `true`, dieselbe vorsichtige Wahl wie bei der
    /// verschwundenen Datei einen Absatz darueber.
    ///
    /// **Der Preis steht hier und wird nicht verschwiegen:** in dieser Lage
    /// meldet der Editor die fremde Aenderung und sichert nicht mehr, bis der
    /// Nutzer die Datei neu oeffnet. Das ist die teurere Seite des Irrtums und
    /// die richtige: die andere schreibt ueber fremde Arbeit.
    #[must_use]
    pub fn fremd_geaendert(&self) -> bool {
        let Some(pfad) = self.pfad.as_ref() else {
            return false;
        };
        let Some(gemerkt) = self.stempel else {
            return true;
        };
        Stempel::von_pfad(pfad) != Some(gemerkt)
    }

    /// Der Satz ueber eine fremde Aenderung, einmal je Aenderung (C4).
    ///
    /// **Der erste der beiden Momente aus dem achten Abnahmekriterium von C4.**
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn weitersuchen(&mut self) -> Option<Treffer> {
        self.weiter_mit(suche::naechster)
    }

    /// Steuert den vorigen Treffer an und laeuft vor dem ersten um (C5).
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    use super::*;
    use crate::pruefordner::Pruefordner;

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
        let mut modell = Editormodell::neu(Heimgriff::default());
        assert_eq!(
            modell.oeffnen(pfad, None),
            None,
            "eine neue Datei wird auf dem Arbeitsfaden gelesen"
        );
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        modell
    }

    #[test]
    fn ein_neuer_editor_haelt_nichts() {
        let modell = Editormodell::neu(Heimgriff::default());
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

        let _ = modell.bearbeiten("erste Zeile\nzweite Zeile\n".to_owned());
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

    /// C6.7 an der Haelfte ohne Fenster: der Text einer Aufgabenzelle, ueber
    /// den Kern zu einem Neustand gerechnet und auf dem Umbauweg ins Modell
    /// gebracht (`Editormodell::bearbeiten`, wie `umbau_anwenden` es ruft),
    /// steht nach dem naechsten Sichern in der Datei, und jede andere Zeile
    /// bleibt Byte fuer Byte. Die andere Haelfte — dass `sichern` die laufende
    /// Zelle zuerst uebernimmt — haelt die Quelltextprobe
    /// `die_zellenuebernahme_hat_genau_diese_rufer` in `appkit/editor.rs`.
    #[test]
    fn ein_uebernommener_zellentext_steht_nach_dem_sichern_in_der_datei() {
        use krk_core::heimordner::eintraege::aufgaben;
        let ordner = Pruefordner::neu("zellentext");
        let vorher = "# Aufgaben\n- [ ] Brot\n  fremde Zeile\n* [X] Steuer\n";
        let pfad = ordner.datei("tasks.txt", vorher);
        let mut modell = geoeffnet(&pfad);

        let neustand = aufgaben::text_aendern(modell.stand(), 0, "Brot und Butter")
            .expect("ein Text ohne Umbruch")
            .expect("die Aufgabe steht, und der Text ist neu");
        let _ = modell.bearbeiten(neustand.text);
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        assert_eq!(
            std::fs::read_to_string(&pfad).expect("die Datei ist nach dem Sichern lesbar"),
            "# Aufgaben\n- [ ] Brot und Butter\n  fremde Zeile\n* [X] Steuer\n"
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

        let mut modell = Editormodell::neu(Heimgriff::default());
        assert_eq!(modell.oeffnen(&erste, None), None);
        assert_eq!(modell.oeffnen(&zweite, None), None);
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

        let _ = modell.bearbeiten("# Ueberschrift\n\nein ungesicherter Absatz\n".to_owned());
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
        let _ = modell.ansicht_umschalten();
        assert_eq!(modell.ansicht(), Ansicht::Roh);

        assert_eq!(modell.oeffnen(&zweite, None), None);
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
        let _ = modell.bearbeiten("guter Inhalt, bearbeitet\n".to_owned());

        // Ein Ordner ist der Fall, den die Pruefung namentlich abweist.
        assert_eq!(modell.oeffnen(ordner.pfad(), None), None);
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

        let zu_gross = ordner.unter("zu-gross.txt");
        std::fs::File::create(&zu_gross)
            .expect("die Pruefdatei laesst sich nicht anlegen")
            .set_len(datei::EDITORGRENZE + 1)
            .expect("die Pruefdatei laesst sich nicht auf Groesse bringen");

        assert_eq!(modell.oeffnen(&zu_gross, None), None);
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

        let _ = modell.bearbeiten("auf der Platte\nund ungesichert getippt\n".to_owned());

        let ausgang = modell.oeffnen(&pfad, None);

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
        // Dass der Stempel dabei stehenbleibt, sagt die Zeile darunter mit: er
        // bewegt sich allein in `uebernehmen` und in `sichern`, und ohne
        // gestarteten Ladevorgang ist keines von beiden gelaufen.
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
            modell.oeffnen(&zweite, None),
            None,
            "die andere Datei geht auf den Arbeitsfaden"
        );
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), "zweite\n");
        assert_eq!(modell.pfad(), Some(zweite.as_path()));
        // Dass nichts zurueckgehalten wurde, steht im Ausgang oben: die vier
        // Werte von `Ladeausgang` sind ueberschneidungsfrei, und `Geoeffnet` ist
        // nicht `Zurueckgehalten`.
    }

    /// Warum das Sitzungsschreiben aus C7 am Ladeausgang haengt und nicht am
    /// Befehl.
    ///
    /// **Die Zeitspanne, in der `pfad()` noch die vorige Datei nennt, ist die
    /// Ursache des Defekts vom 260810-0240.** F4 merkt die Sitzung vor, sobald
    /// der Befehl gelaufen ist; gelesen wird da noch, und mitgeschrieben wuerde
    /// die vorige Datei. Diese Probe haelt die Spanne fest, damit sie auffaellt,
    /// wenn jemand das Lesen wieder auf den Hauptfaden zieht: dann liefert
    /// `oeffnen` sofort einen Ausgang, und `laedt_noch` ist hier falsch.
    #[test]
    fn der_gehaltene_pfad_wechselt_erst_mit_dem_eingezogenen_ausgang() {
        let ordner = Pruefordner::neu("pfadwechsel");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);

        assert_eq!(modell.oeffnen(&zweite, None), None);
        assert!(modell.laedt_noch(), "der Arbeitsfaden liest noch");
        assert_eq!(
            modell.pfad(),
            Some(erste.as_path()),
            "waehrend des Lesens nennt der Editor unveraendert die vorige Datei"
        );

        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(
            modell.pfad(),
            Some(zweite.as_path()),
            "erst der eingezogene Ausgang traegt die neue Datei"
        );
    }

    /// Der Defekt vom 260810-1029: die Abkuerzung gibt das laufende Lesen auf.
    ///
    /// Der Ablauf ist der des Datensatzes und liegt in der Spanne, die die Probe
    /// darueber festhaelt. Der Editor haelt die eine Datei, der Nutzer oeffnet
    /// die andere, und waehrend die gelesen wird, holt er mit F4 die gehaltene
    /// zurueck — der Weg, den `260809-2029` als den namentlich gegangenen
    /// festhaelt, weil die Vorschau den Editor nach C1 verdraengt. Bis zum
    /// 260810 kamen darauf **zwei** Ladeausgaenge: `SchonOffen` unverzueglich,
    /// und danach `Geoeffnet` fuer die andere Datei, sobald der stehengelassene
    /// Faden lieferte. Der Editor hielt am Ende die Datei, die der Nutzer
    /// zuletzt gerade nicht verlangt hatte.
    ///
    /// **Die Probe haengt nicht an einer Wettlage**, aus demselben Grund wie
    /// `ein_zweiter_ladevorgang_laesst_den_ersten_verfallen`: der Empfaenger
    /// faellt in dem Augenblick, in dem `oeffnen` den Vorgang aufgibt, und
    /// danach **kann** die Meldung nicht mehr ankommen, gleichgueltig wie
    /// schnell der Faden war. Die Schleife am Ende wartet trotzdem eine Spanne
    /// ab, in der der Faden bei diesen Dateigroessen laengst geliefert haette:
    /// die Zusage lautet "genau ein Ausgang", und ein zweiter faellt nur auf,
    /// wenn jemand auf ihn wartet.
    #[test]
    fn die_abkuerzung_fuer_die_gehaltene_datei_bricht_das_laufende_lesen_ab() {
        let ordner = Pruefordner::neu("abkuerzung-bricht-ab");
        let gehalten = ordner.datei("gehalten.txt", "Inhalt der gehaltenen Datei\n");
        let andere = ordner.datei("andere.txt", "Inhalt der anderen Datei\n");
        let mut modell = geoeffnet(&gehalten);

        assert_eq!(
            modell.oeffnen(&andere, None),
            None,
            "die andere Datei geht auf den Arbeitsfaden"
        );
        assert!(modell.laedt_noch(), "der Arbeitsfaden liest noch");

        assert_eq!(
            modell.oeffnen(&gehalten, None),
            Some(Ladeausgang::SchonOffen),
            "waehrend des Lesens nennt der Editor unveraendert die gehaltene Datei, \
             und die Abkuerzung greift"
        );
        assert!(
            !modell.laedt_noch(),
            "260810-1029: das Lesen der anderen Datei gehoert nach diesem Befehl niemandem mehr"
        );

        for _ in 0..300 {
            assert_eq!(
                modell.einziehen(),
                None,
                "260810-1029: auf `SchonOffen` folgt kein zweiter Ladeausgang"
            );
            thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(
            modell.pfad(),
            Some(gehalten.as_path()),
            "der Editor haelt die Datei, die der letzte Befehl verlangt hat"
        );
        assert_eq!(modell.stand(), "Inhalt der gehaltenen Datei\n");
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
        let _ = modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        // Dass die gelesene Datei wartet, sagt der Ausgang darueber; was hier
        // folgt, ist die zweite Haelfte der Zusage, naemlich dass er dabei nichts
        // bewegt hat.
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
        let _ = modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite, None), None);
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
        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            None,
            "es wartet nichts mehr: ein zweiter Ruf findet nichts und tut nichts"
        );
    }

    /// C4: "abbrechen" laesst die gelesene Datei fallen und den Stand stehen.
    #[test]
    fn ein_abgebrochener_wechsel_laesst_den_stand_vollstaendig_stehen() {
        let ordner = Pruefordner::neu("zurueckgehalten-fallenlassen");
        let erste = ordner.datei("erste.txt", "erste\n");
        let zweite = ordner.datei("zweite.txt", "zweite\n");
        let mut modell = geoeffnet(&erste);
        let _ = modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        modell.zurueckgehaltenes_fallenlassen();
        // Gefragt wird ueber den Weg, der die wartende Datei aufnehmen wuerde,
        // und nicht ueber ein Feld: findet er nichts, wartet nichts.
        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            None,
            "die gelesene Datei ist gefallen"
        );
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
        let _ = modell.bearbeiten("guter Inhalt, bearbeitet\n".to_owned());

        // Ein Ordner ist der Fall, den die Pruefung namentlich abweist.
        assert_eq!(modell.oeffnen(ordner.pfad(), None), None);
        let ausgang = abwarten(&mut modell);
        assert!(
            matches!(ausgang, Ladeausgang::Abgewiesen(_)),
            "eine Abweisung geht unverzueglich durch, {ausgang:?}"
        );
        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            None,
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
        let _ = modell.bearbeiten("erste, bearbeitet\n".to_owned());

        assert_eq!(modell.oeffnen(&zweite, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Zurueckgehalten);

        modell.schliessen();
        assert_eq!(
            modell.zurueckgehaltenes_uebernehmen(),
            None,
            "die wartende Datei ist mit dem Schliessen gefallen"
        );
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
        let unterordner = ordner.unter("gesperrt");
        std::fs::create_dir(&unterordner).expect("der Unterordner laesst sich anlegen");
        let pfad = unterordner.join("stand.txt");
        std::fs::write(&pfad, "Inhalt\n").expect("die Pruefdatei laesst sich schreiben");

        let mut modell = geoeffnet(&pfad);
        let _ = modell.bearbeiten("neuer Inhalt\n".to_owned());

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

    /// Das achte Abnahmekriterium von C4, an der Stelle, an der der Schaden
    /// entstuende: eine von aussen geaenderte Datei wird nicht ueberschrieben.
    #[test]
    fn eine_von_aussen_geaenderte_datei_wird_nicht_ueberschrieben() {
        let ordner = Pruefordner::neu("sichern-fremd");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        let _ = modell.bearbeiten("im Editor getippt\n".to_owned());

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
        let _ = modell.bearbeiten("im Editor getippt\n".to_owned());

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
        let mut modell = Editormodell::neu(Heimgriff::default());
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
        // Dass ueberhaupt ein Stempel erhoben wurde, sagt **diese** Zusicherung:
        // ohne gemerkten Stempel antwortet `fremd_geaendert` seit dem 260908
        // `true`, und die Probe bliebe an jeder folgenden Zeile haengen.
        assert!(!modell.fremd_geaendert());

        let _ = modell.bearbeiten("im Editor geändert\n".to_owned());
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

    /// Der zweite Weg zu `stempel = None` bei gehaltener Datei: nicht das
    /// Schliessen, sondern ein gescheitertes `Stempel::von_pfad` nach dem
    /// Sichern oder vor dem Lesen. Der Editor haelt danach eine Datei und hat
    /// seine Bezugsgroesse verloren; C4 gilt trotzdem weiter.
    #[test]
    fn eine_gehaltene_datei_ohne_stempel_gilt_als_geaendert() {
        let ordner = Pruefordner::neu("stempel-fehlt");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        assert!(
            !modell.fremd_geaendert(),
            "mit Stempel ist nichts geaendert"
        );

        // Genau der Zustand, den ein gescheitertes `metadata` hinterlaesst.
        modell.stempel = None;
        assert!(
            modell.fremd_geaendert(),
            "ohne Stempel bleibt die Zusage aus C4 stehen, statt stumm abzuschalten"
        );
        assert!(
            modell.fremdaenderung_melden().is_some(),
            "und der erste der beiden Momente meldet"
        );
        let Sicherungsausgang::Gescheitert(_) = modell.sichern() else {
            panic!("der zweite Moment ueberschreibt nicht ungeprueft");
        };
    }

    #[test]
    fn das_schliessen_gibt_die_datei_auf() {
        let ordner = Pruefordner::neu("schliessen");
        let pfad = ordner.datei("stand.txt", "Inhalt\n");
        let mut modell = geoeffnet(&pfad);
        let _ = modell.bearbeiten("bearbeitet\n".to_owned());

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
        let _ = modell.bearbeiten("eins zwei eins drei eins\n".to_owned());

        let erster = modell
            .suche_starten("eins", 0)
            .expect("drei Treffer stehen im Stand");
        assert_eq!(erster.anfang, 0);
        let lauf = modell.suchlauf().expect("der Suchlauf steht");
        assert_eq!(lauf.gesucht(), "eins");
        assert_eq!(lauf.zahl(), 3);
        assert_eq!(lauf.nummer(), Some(1));
        assert_eq!(lauf.meldung(), "Treffer 1 von 3");

        // Wo die drei Treffer stehen, sagen die drei Zeilen hier und nicht die
        // Liste im Suchlauf: sie nennen dieselben drei Versaetze in derselben
        // Reihenfolge und dazu den Umlauf, den die Liste allein nicht zeigt.
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
        // Dass ein beendeter Suchlauf nichts mehr ansteuert, steht in
        // `eine_bearbeitung_beendet_den_suchlauf` weiter unten — dort endet er
        // auf einem Weg, den das Programm wirklich geht.
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

        let _ = modell.suche_starten("eins", 0);
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

        let _ = modell.suche_starten("foo", 0);
        let _ = modell.treffer_ersetzen("foofoo");
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

        let _ = modell.suche_starten("a", 0);
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

        let _ = modell.bearbeiten("aus Windows\r\neingefügt\r\nletzte".to_owned());
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

        let _ = modell.suche_starten("eins", 0);
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

        let _ = modell.suche_starten("a", 0);
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

        let _ = modell.suche_starten("eins", 0);
        assert!(modell.suchlauf().is_some());
        let _ = modell.bearbeiten("kurz\n".to_owned());
        assert!(modell.suchlauf().is_none());
        assert_eq!(modell.weitersuchen(), None);
    }

    #[test]
    fn der_dateityp_kommt_aus_der_endung() {
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/lies.md"), None),
            Dateityp::Markdown
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/LIES.MARKDOWN"), None),
            Dateityp::Markdown,
            "die Endung wird ohne Ruecksicht auf Gross- und Kleinschreibung verglichen"
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/quelle.rs"), None),
            Dateityp::Sonstiges
        );
        assert_eq!(
            Dateityp::von_pfad(Path::new("/a/b/Makefile"), None),
            Dateityp::Sonstiges,
            "ohne Endung ist die Frage nach Markdown mit Nein beantwortet"
        );
    }

    /// Ein Pruef-krkhome: `zuhause/krkhome` als Verweis auf `ziel`, mit
    /// `notes.txt` und `tasks.txt` darin, und der Heimordner dazu.
    fn pruef_krkhome(ordner: &Pruefordner) -> (Heimordner, PathBuf, PathBuf) {
        let zuhause = ordner.ordner("zuhause");
        let ziel = ordner.ordner("ziel");
        std::os::unix::fs::symlink(&ziel, zuhause.join(krk_core::heimordner::ORDNERNAME))
            .expect("der Verweis laesst sich anlegen");
        std::fs::write(ziel.join("notes.txt"), "## Thema\nText\n").expect("notes.txt");
        std::fs::write(ziel.join("tasks.txt"), "- [ ] a\n").expect("tasks.txt");
        let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
        let geschrieben = heim.geschrieben().to_path_buf();
        (heim, geschrieben, ziel)
    }

    /// C4.1 fuer den Dateityp: die drei Dateien im erkannten Ordner sind
    /// Eintragsdateien, ueber die geschriebene und ueber die aufgeloeste Form,
    /// und eine gleichnamige Datei anderswo bleibt, was ihre Endung sagt.
    #[test]
    fn die_eintragsdateien_erkennt_der_dateityp_ueber_den_heimordner() {
        let ordner = Pruefordner::neu("dateityp-eintraege");
        let (heim, geschrieben, ziel) = pruef_krkhome(&ordner);
        for basis in [&geschrieben, &ziel] {
            assert_eq!(
                Dateityp::von_pfad(&basis.join("notes.txt"), Some(&heim)),
                Dateityp::Eintraege(Sonderdatei::Notizen),
                "{}",
                basis.display()
            );
            assert_eq!(
                Dateityp::von_pfad(&basis.join("tasks.txt"), Some(&heim)),
                Dateityp::Eintraege(Sonderdatei::Aufgaben),
                "{}",
                basis.display()
            );
            assert_eq!(
                Dateityp::von_pfad(&basis.join(".secrets.txt"), Some(&heim)),
                Dateityp::Eintraege(Sonderdatei::Geheimnisse),
                "{}",
                basis.display()
            );
        }
        let daneben = ordner.datei("notes.txt", "## Thema\n");
        assert_eq!(
            Dateityp::von_pfad(&daneben, Some(&heim)),
            Dateityp::Sonstiges
        );
        assert_eq!(
            Dateityp::von_pfad(&geschrieben.join("notes.txt"), None),
            Dateityp::Sonstiges,
            "ohne Heimordner entscheidet die Endung"
        );
        assert_eq!(
            Dateityp::von_pfad(&geschrieben.join("lies.md"), Some(&heim)),
            Dateityp::Markdown,
            "eine andere Datei im Heimordner bleibt bei ihrer Endung"
        );
    }

    /// Der Editor fragt beim Uebernehmen den geteilten Griff, und ein
    /// Ersetzen darin gilt fuer das naechste Oeffnen.
    #[test]
    fn der_editor_liest_den_dateityp_aus_dem_geteilten_griff() {
        let ordner = Pruefordner::neu("editor-eintraege");
        let (heim, geschrieben, _) = pruef_krkhome(&ordner);
        let griff = Heimgriff::default();
        let mut modell = Editormodell::neu(std::rc::Rc::clone(&griff));
        let notizen = geschrieben.join("notes.txt");

        assert_eq!(modell.oeffnen(&notizen, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(
            modell.typ(),
            Dateityp::Sonstiges,
            "noch kein Heimordner im Griff"
        );

        heimgriff::ersetzen(&griff, heim);
        let aufgaben = geschrieben.join("tasks.txt");
        assert_eq!(modell.oeffnen(&aufgaben, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.typ(), Dateityp::Eintraege(Sonderdatei::Aufgaben));
        assert_eq!(
            crate::hervorhebung::art(modell.pfad(), modell.typ()),
            crate::hervorhebung::Darstellungsart::Markdown,
            "die Formatansicht zeigt die Eintragsdatei bis zur Tabellenform als Markdown"
        );
    }

    #[test]
    fn die_beiden_ansichten_sind_die_jeweils_andere() {
        assert_eq!(Ansicht::Roh.andere(), Ansicht::Format);
        assert_eq!(Ansicht::Format.andere(), Ansicht::Roh);
    }

    /// Der erste Moment aus dem achten Abnahmekriterium von C4: eine fremde
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
        assert_eq!(modell.oeffnen(&pfad, None), None);
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
        let mut modell = Editormodell::neu(Heimgriff::default());
        assert_eq!(modell.fremdaenderung_melden(), None);
    }

    // ---------------------------------------------------------------------
    // `.secrets.txt`: Schutz, Laden mit PIN, verschluesseltes Sichern
    // (Schritt 5.4a des Plans der krkhome-Arbeit)
    // ---------------------------------------------------------------------

    /// Der bekannte Eintrag, der nie im Klartext auf der Platte stehen darf.
    const GEHEIM: &str = "Tresorwort-4711";

    fn pin(ziffern: &str) -> Pin {
        Pin::aus_eingabe(ziffern).expect("vier Ziffern")
    }

    /// Wie [`abwarten`], mit einer Schranke fuer die Ableitung: mit den
    /// Parametern des Codes kostet sie rund eine halbe Sekunde, unter der
    /// Last der uebrigen Proben mehr.
    fn abwarten_mit_ableitung(modell: &mut Editormodell) -> Ladeausgang {
        for _ in 0..30_000 {
            if let Some(ausgang) = modell.einziehen() {
                return ausgang;
            }
            thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("der Editor-Arbeitsfaden hat innerhalb von dreissig Sekunden nichts geliefert");
    }

    /// Ein Pruef-krkhome samt Modell, das es kennt, und dem Pfad von
    /// `.secrets.txt` in der geschriebenen Form.
    fn geheimnis_modell(ordner: &Pruefordner) -> (Editormodell, PathBuf, PathBuf) {
        let (heim, geschrieben, ziel) = pruef_krkhome(ordner);
        let griff = Heimgriff::default();
        heimgriff::ersetzen(&griff, heim);
        (
            Editormodell::neu(griff),
            geschrieben.join(".secrets.txt"),
            ziel,
        )
    }

    /// Schreibt eine `.secrets.txt` mit kleinen Parametern, damit die Proben
    /// nicht je Oeffnen eine halbe Sekunde warten. Geoeffnet wird sie mit den
    /// Parametern aus ihrem Kopf.
    fn verschlossen_ablegen(pfad: &Path, klartext: &str, ziffern: &str) {
        let klein = tresor::Parameter::neu(64, 1, 1).expect("gueltige Parameter");
        let schluessel =
            tresor::schluessel_ableiten(&pin(ziffern), &[5; tresor::SALZLAENGE], klein)
                .expect("Ableitung");
        let bytes =
            tresor::verschliessen(klartext.as_bytes(), &schluessel).expect("Verschluesseln");
        std::fs::write(pfad, bytes).expect("die Pruefdatei laesst sich schreiben");
    }

    fn enthaelt(heuhaufen: &[u8], nadel: &str) -> bool {
        heuhaufen
            .windows(nadel.len())
            .any(|fenster| fenster == nadel.as_bytes())
    }

    fn meldung_von(ausgang: &Ladeausgang) -> String {
        match ausgang {
            Ladeausgang::Abgewiesen(abweisung) => abweisung.meldung(),
            anderer => panic!("erwartet war eine Abweisung, gekommen ist {anderer:?}"),
        }
    }

    /// C7.10 im Modell: ohne PIN weist das Modell `.secrets.txt` ab, **bevor
    /// ein Faden startet**, ueber beide Formen des Heimordners. Die Datei
    /// traegt Modus `000`: gaebe es ein Lesen, scheiterte es mit einem anderen
    /// Satz. Das ist der Weg von F4 und `cmd+e`, die beide hierher fuehren.
    #[test]
    fn ohne_pin_weist_das_modell_secrets_txt_ab_ohne_zu_lesen() {
        use std::os::unix::fs::PermissionsExt;
        let ordner = Pruefordner::neu("geheim-ohne-pin");
        let (mut modell, geschrieben, ziel) = geheimnis_modell(&ordner);
        verschlossen_ablegen(&geschrieben, GEHEIM, "0417");
        std::fs::set_permissions(&geschrieben, std::fs::Permissions::from_mode(0o000))
            .expect("Modus 000");

        for pfad in [&geschrieben, &ziel.join(".secrets.txt")] {
            let ausgang = modell
                .oeffnen(pfad, None)
                .expect("der Ausgang steht sofort fest");
            let satz = meldung_von(&ausgang);
            assert!(
                satz.contains("öffnet sich allein mit der PIN"),
                "{}: {satz}",
                pfad.display()
            );
            assert!(!modell.laedt_noch(), "es wurde kein Faden gestartet");
            assert!(!modell.haelt_datei());
            assert_eq!(modell.stand(), "");
        }
        std::fs::set_permissions(&geschrieben, std::fs::Permissions::from_mode(0o600))
            .expect("Modus zurueck");
    }

    /// Mit der richtigen PIN kommen die Eintraege, als Eintragsdatei der
    /// Geheimnisse; ein zweites Oeffnen derselben Datei ohne PIN ist die
    /// Abkuerzung und liest nicht, denn die PIN gilt, solange die Datei offen
    /// ist.
    #[test]
    fn mit_der_richtigen_pin_kommen_die_eintraege() {
        let ordner = Pruefordner::neu("geheim-richtig");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        let klartext = format!("## Konto\n{GEHEIM}\n");
        verschlossen_ablegen(&pfad, &klartext, "0417");

        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), klartext);
        assert_eq!(modell.typ(), Dateityp::Eintraege(Sonderdatei::Geheimnisse));
        assert!(!modell.hat_ungesicherten_stand());
        assert!(matches!(modell.schutz, Schutz::Verschluesselt { .. }));

        assert_eq!(modell.oeffnen(&pfad, None), Some(Ladeausgang::SchonOffen));
        assert_eq!(modell.stand(), klartext);
    }

    /// C7.8: haelt der Editor `.secrets.txt`, verweigert er die Textmarke,
    /// deren `zeileninhalt` eine Zeile der Geheimnisse im Klartext in die
    /// Lesezeichendatei truege; ueber beide Formen des Heimordners, mit
    /// gesichertem Kopf und als leere Datei mit eben festgelegter PIN
    /// (`issues/260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md`).
    #[test]
    fn an_secrets_txt_entsteht_keine_textmarke() {
        let ordner = Pruefordner::neu("geheim-textmarke");
        let (mut modell, geschrieben, ziel) = geheimnis_modell(&ordner);
        assert_eq!(modell.textmarke_verweigert(), None, "ohne Datei");

        verschlossen_ablegen(&geschrieben, &format!("## Konto\n{GEHEIM}\n"), "0417");
        for pfad in [geschrieben.clone(), ziel.join(".secrets.txt")] {
            let mut frisch = Editormodell::neu(modell.heim.clone());
            assert_eq!(frisch.oeffnen(&pfad, Some(pin("0417"))), None);
            assert_eq!(abwarten_mit_ableitung(&mut frisch), Ladeausgang::Geoeffnet);
            assert_eq!(
                frisch.textmarke_verweigert(),
                Some(KEINE_TEXTMARKE_IN_GEHEIMNISSEN),
                "{}",
                pfad.display()
            );
        }

        std::fs::write(&geschrieben, b"").expect("leere .secrets.txt");
        assert_eq!(modell.oeffnen(&geschrieben, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(
            modell.textmarke_verweigert(),
            Some(KEINE_TEXTMARKE_IN_GEHEIMNISSEN),
            "leere Datei"
        );
    }

    /// Die Gegenprobe: jede andere Datei bekommt ihre Textmarke, auch
    /// `notes.txt` und `tasks.txt` im erkannten Ordner und eine `.secrets.txt`
    /// ausserhalb davon, die eine gewoehnliche Textdatei ist.
    #[test]
    fn jede_andere_datei_bekommt_ihre_textmarke() {
        let ordner = Pruefordner::neu("geheim-textmarke-gegen");
        let (mut modell, geschrieben, _) = geheimnis_modell(&ordner);
        let anderswo = ordner.ordner("anderswo");
        let daneben = anderswo.join(".secrets.txt");
        std::fs::write(&daneben, "kein Geheimnis\n").expect(".secrets.txt anderswo");
        let heim = geschrieben.parent().expect("der Heimordner").to_path_buf();
        for pfad in [heim.join("notes.txt"), heim.join("tasks.txt"), daneben] {
            assert_eq!(modell.oeffnen(&pfad, None), None, "{}", pfad.display());
            assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
            assert_eq!(modell.textmarke_verweigert(), None, "{}", pfad.display());
        }
    }

    /// Der Satz der Verweigerung nennt die Datei und traegt Umlaute nach der
    /// Schreibregel des Projekts, denn der Nutzer liest ihn in der Statuszeile.
    #[test]
    fn der_satz_der_verweigerten_textmarke_traegt_umlaute() {
        assert!(KEINE_TEXTMARKE_IN_GEHEIMNISSEN.contains(".secrets.txt"));
        assert!(KEINE_TEXTMARKE_IN_GEHEIMNISSEN.contains("für"));
        assert!(KEINE_TEXTMARKE_IN_GEHEIMNISSEN.contains("Klartext"));
        assert!(!KEINE_TEXTMARKE_IN_GEHEIMNISSEN.contains("fuer"));
    }

    /// Eine falsche PIN und ein veraendertes Byte geben dieselbe eine Meldung,
    /// keinen Text, und die Datei bleibt Byte fuer Byte, wie sie war. Ein
    /// beschaedigter Kopf nennt den Schaden. Der vorher gehaltene Stand bleibt
    /// stehen.
    #[test]
    fn falsche_pin_und_veraenderte_datei_geben_eine_meldung_und_lassen_die_datei_stehen() {
        let ordner = Pruefordner::neu("geheim-falsch");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        let vorher = ordner.datei("vorher.txt", "der bisherige Stand\n");
        assert_eq!(modell.oeffnen(&vorher, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);

        verschlossen_ablegen(&pfad, GEHEIM, "0417");
        let original = std::fs::read(&pfad).expect("lesen");

        assert_eq!(modell.oeffnen(&pfad, Some(pin("1234"))), None);
        let falsch = meldung_von(&abwarten_mit_ableitung(&mut modell));
        assert!(
            falsch.contains("PIN falsch oder Datei verändert"),
            "{falsch}"
        );
        assert!(!falsch.contains(GEHEIM));
        assert_eq!(std::fs::read(&pfad).expect("lesen"), original);
        assert_eq!(modell.pfad(), Some(vorher.as_path()));
        assert_eq!(modell.stand(), "der bisherige Stand\n");

        let mut veraendert = original.clone();
        let letztes = veraendert.len() - 1;
        veraendert[letztes] ^= 0x01;
        std::fs::write(&pfad, &veraendert).expect("schreiben");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        let geaendert = meldung_von(&abwarten_mit_ableitung(&mut modell));
        assert_eq!(geaendert, falsch, "eine Meldung fuer beide Faelle");
        assert_eq!(std::fs::read(&pfad).expect("lesen"), veraendert);

        std::fs::write(&pfad, "kein Kopf").expect("schreiben");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        let kopf = meldung_von(&abwarten_mit_ableitung(&mut modell));
        assert!(kopf.contains("Kopf der Datei ist beschädigt"), "{kopf}");
        assert_eq!(std::fs::read(&pfad).expect("lesen"), b"kein Kopf");
        assert_eq!(modell.pfad(), Some(vorher.as_path()));
    }

    /// C7.8 im Sicherungsteil: nach `sichern` steht der Klartext weder in der
    /// Datei noch im Bild der Nachbardatei vor dem `rename`, das die Probe am
    /// Schreibweg abfaengt. Ein Stand ohne Schlussumbruch kommt nach Sichern
    /// und Oeffnen mit einem zurueck, wie bei `notes.txt`. Das Salz bleibt das
    /// der geoeffneten Datei.
    #[test]
    fn nach_dem_sichern_steht_kein_klartext_auf_der_platte() {
        let ordner = Pruefordner::neu("geheim-sichern");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        verschlossen_ablegen(&pfad, "## Konto\nalt\n", "0417");
        let salz_vorher = *tresor::Kopf::lesen(&std::fs::read(&pfad).expect("lesen"))
            .expect("Kopf")
            .salz();

        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        let _ = modell.bearbeiten(format!("## Konto\n{GEHEIM}"));

        let mut abbild = None;
        let ausgang = modell.sichern_ueber(|ziel, chiffrat| {
            let nachbar = atomar::vorbereiten(ziel, &mut chiffrat.0.as_slice())?;
            abbild = Some(std::fs::read(nachbar.nachbarpfad())?);
            nachbar.umbenennen()
        });
        assert_eq!(ausgang, Sicherungsausgang::Gesichert(pfad.clone()));
        let abbild = abbild.expect("der Schreibweg wurde gerufen");
        assert!(
            !enthaelt(&abbild, GEHEIM),
            "die Nachbardatei traegt Klartext"
        );
        assert!(
            !enthaelt(&abbild, "Konto"),
            "die Nachbardatei traegt Klartext"
        );

        let platte = std::fs::read(&pfad).expect("lesen");
        assert_eq!(platte, abbild);
        assert!(platte.starts_with(&tresor::KENNUNG));
        assert!(!enthaelt(&platte, GEHEIM), "die Datei traegt Klartext");
        let kopf = tresor::Kopf::lesen(&platte).expect("Kopf");
        assert_eq!(*kopf.salz(), salz_vorher, "Sichern zieht kein neues Salz");
        assert!(!modell.hat_ungesicherten_stand());

        modell.schliessen();
        assert!(matches!(modell.schutz, Schutz::Klartext));
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), format!("## Konto\n{GEHEIM}\n"));
    }

    /// Der Alltagsweg `sichern` schreibt ebenso allein Chiffrat, und zwei
    /// Sicherungen ohne Aenderung ergeben verschiedene Bytes mit demselben
    /// Salz: frische Nonce, keine neue Ableitung.
    #[test]
    fn zwei_sicherungen_ergeben_verschiedene_bytes_mit_demselben_salz() {
        let ordner = Pruefordner::neu("geheim-zweimal");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        verschlossen_ablegen(&pfad, &format!("{GEHEIM}\n"), "0417");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let erste = std::fs::read(&pfad).expect("lesen");
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let zweite = std::fs::read(&pfad).expect("lesen");

        assert_ne!(erste, zweite);
        assert!(!enthaelt(&erste, GEHEIM) && !enthaelt(&zweite, GEHEIM));
        assert_eq!(
            tresor::Kopf::lesen(&erste).expect("Kopf").salz(),
            tresor::Kopf::lesen(&zweite).expect("Kopf").salz()
        );
    }

    /// Eine ausserhalb geaenderte `.secrets.txt` weist `sichern` ab wie jede
    /// Datei, und die fremden Bytes bleiben stehen.
    #[test]
    fn eine_fremd_geaenderte_secrets_txt_wird_nicht_ueberschrieben() {
        let ordner = Pruefordner::neu("geheim-fremd");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        verschlossen_ablegen(&pfad, "## A\nx\n", "0417");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        let _ = modell.bearbeiten(format!("## A\n{GEHEIM}\n"));

        std::fs::write(&pfad, b"von aussen, deutlich laenger als vorher").expect("schreiben");
        assert!(matches!(
            modell.sichern(),
            Sicherungsausgang::Gescheitert(_)
        ));
        assert_eq!(
            std::fs::read(&pfad).expect("lesen"),
            b"von aussen, deutlich laenger als vorher"
        );
    }

    /// Die neue Datei: null Bytes mit PIN oeffnen leitet einen neuen Schluessel
    /// ab und schreibt nichts, die Datei bleibt null Bytes. Nach dem ersten
    /// Sichern oeffnet sie allein diese PIN, und ein leerer Stand gibt ein
    /// Chiffrat und keine leere Datei. Dieser Weg leitet zweimal mit den
    /// Parametern des Codes ab.
    #[test]
    fn eine_neue_datei_bleibt_leer_bis_zum_sichern_und_oeffnet_dann_allein_mit_ihrer_pin() {
        let ordner = Pruefordner::neu("geheim-neu");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        std::fs::write(&pfad, b"").expect("leere Datei");

        assert_eq!(modell.oeffnen(&pfad, Some(pin("2468"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        assert_eq!(modell.stand(), "");
        let Schutz::Verschluesselt { schluessel, .. } = &modell.schutz else {
            panic!("die neue Datei haelt keinen Schluessel");
        };
        assert_eq!(schluessel.parameter(), tresor::Parameter::DES_CODES);
        modell.schliessen();
        assert_eq!(
            std::fs::metadata(&pfad).expect("stat").len(),
            0,
            "Oeffnen und Schliessen ohne Sichern schreiben nichts"
        );

        assert_eq!(modell.oeffnen(&pfad, Some(pin("2468"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        let _ = modell.bearbeiten(format!("## Neu\n{GEHEIM}\n"));
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let platte = std::fs::read(&pfad).expect("lesen");
        assert!(!enthaelt(&platte, GEHEIM));
        let geoeffnet = tresor::oeffnen(&platte, &pin("2468")).expect("die eigene PIN oeffnet");
        assert_eq!(
            geoeffnet.klartext,
            format!("## Neu\n{GEHEIM}\n").into_bytes()
        );
        assert_eq!(
            tresor::oeffnen(&platte, &pin("2469")).map(|_| ()),
            Err(tresor::Oeffnungsfehler::PinFalschOderVeraendert)
        );

        let _ = modell.bearbeiten(String::new());
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let leer = std::fs::read(&pfad).expect("lesen");
        assert!(
            leer.len() > tresor::KOPFLAENGE,
            "ein leerer Stand ist ein Chiffrat und keine neue Datei"
        );
        assert!(tresor::oeffnen(&leer, &pin("2468")).is_ok());
    }

    /// Die Sperre haelt auch, wenn die Erkennung sich nach dem Oeffnen
    /// aendert: eine im Klartext gelesene `.secrets.txt` wird nicht
    /// aufgenommen, und eine im Klartext gehaltene wird nicht im Klartext
    /// geschrieben. Die Datei bleibt null Bytes.
    #[test]
    fn eine_spaet_erkannte_secrets_txt_wird_weder_aufgenommen_noch_im_klartext_geschrieben() {
        let ordner = Pruefordner::neu("geheim-spaet");
        let (heim, geschrieben, _) = pruef_krkhome(&ordner);
        let pfad = geschrieben.join(".secrets.txt");
        std::fs::write(&pfad, b"").expect("leere Datei");

        // Beim Lesen noch nicht erkannt, beim Aufnehmen schon.
        let griff = Heimgriff::default();
        let mut modell = Editormodell::neu(std::rc::Rc::clone(&griff));
        assert_eq!(modell.oeffnen(&pfad, None), None);
        heimgriff::ersetzen(&griff, heim.clone());
        let satz = meldung_von(&abwarten(&mut modell));
        assert!(satz.contains("öffnet sich allein mit der PIN"), "{satz}");
        assert!(!modell.haelt_datei());

        // Beim Lesen und Aufnehmen nicht erkannt, beim Sichern schon.
        let griff = Heimgriff::default();
        let mut modell = Editormodell::neu(std::rc::Rc::clone(&griff));
        assert_eq!(modell.oeffnen(&pfad, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        let _ = modell.bearbeiten(format!("{GEHEIM}\n"));
        heimgriff::ersetzen(&griff, heim);
        let Sicherungsausgang::Gescheitert(satz) = modell.sichern() else {
            panic!("der Klartext wurde geschrieben");
        };
        assert!(satz.contains("nicht im Klartext"), "{satz}");
        assert_eq!(std::fs::metadata(&pfad).expect("stat").len(), 0);
    }

    /// Eine PIN fuer eine gewoehnliche Datei weist ab, statt sie im Klartext
    /// zu oeffnen.
    #[test]
    fn eine_pin_fuer_eine_andere_datei_weist_ab() {
        let ordner = Pruefordner::neu("geheim-andere");
        let (mut modell, _, ziel) = geheimnis_modell(&ordner);
        let ausgang = modell
            .oeffnen(&ziel.join("notes.txt"), Some(pin("0417")))
            .expect("der Ausgang steht sofort fest");
        assert!(meldung_von(&ausgang).contains("keine verschlüsselte Datei"));
        assert!(!modell.laedt_noch());
    }

    /// Nach dem Schliessen gilt die PIN nicht mehr: dieselbe Datei ohne PIN
    /// weist ab, statt die Abkuerzung zu nehmen. Und der Wechsel auf eine
    /// gewoehnliche Datei nimmt den Schutz mit dem Stand fort.
    #[test]
    fn schliessen_und_dateiwechsel_werfen_den_schutz_fort() {
        let ordner = Pruefordner::neu("geheim-fort");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        verschlossen_ablegen(&pfad, "## A\nx\n", "0417");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        modell.schliessen();
        assert!(matches!(
            modell.oeffnen(&pfad, None),
            Some(Ladeausgang::Abgewiesen(_))
        ));

        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        let gewoehnlich = ordner.datei("gewoehnlich.txt", "offen\n");
        assert_eq!(modell.oeffnen(&gewoehnlich, None), None);
        assert_eq!(abwarten(&mut modell), Ladeausgang::Geoeffnet);
        assert!(matches!(modell.schutz, Schutz::Klartext));
        let _ = modell.bearbeiten("offen und lesbar\n".to_owned());
        assert_eq!(
            modell.sichern(),
            Sicherungsausgang::Gesichert(gewoehnlich.clone())
        );
        assert_eq!(
            std::fs::read_to_string(&gewoehnlich).expect("lesen"),
            "offen und lesbar\n"
        );
    }

    /// Die Code-Zeilen einer Datei des Quellbaums vor ihrem Pruefmodul.
    fn code_vor_den_proben(pfad: &str) -> String {
        let (_, inhalt) = crate::quellbaum::quelldateien()
            .into_iter()
            .find(|(name, _)| name == pfad)
            .unwrap_or_else(|| panic!("{pfad} steht nicht im Quellbaum"));
        let code: Vec<&str> = crate::quellbaum::codezeilen(&inhalt).collect();
        let code = code.join("\n");
        match code.split_once(concat!("#[cfg(test)]\nmod ", "tests {")) {
            Some((vorne, _)) => vorne.to_owned(),
            None => code,
        }
    }

    /// Der Rumpf einer Methode, bis zur schliessenden Klammer auf der
    /// Einrueckung einer Methode.
    fn rumpf_von<'a>(code: &'a str, kopf: &str) -> &'a str {
        let beginn = code
            .find(kopf)
            .unwrap_or_else(|| panic!("{kopf} steht nicht im Code"));
        let rest = &code[beginn..];
        let ende = rest
            .find("\n    }\n")
            .unwrap_or_else(|| panic!("der Rumpf von {kopf} endet nicht"));
        &rest[..ende]
    }

    /// Die Sperre steht in `Editormodell::oeffnen` vor dem Start des Fadens,
    /// und `datei::oeffnen` hat genau einen Rufer, `text_lesen`, den allein der
    /// Leseauftrag `Text` erreicht; den erteilt `oeffnen` allein fuer eine
    /// Datei, die nicht `.secrets.txt` ist.
    #[test]
    fn die_sperre_fragt_die_sonderdatei_vor_dem_lesen() {
        let code = code_vor_den_proben("krk-ui/src/editormodell.rs");
        let oeffnen = rumpf_von(&code, concat!("pub fn oeff", "nen(&mut self"));
        let frage = oeffnen
            .find(concat!("self.ist_geheimnis", "datei(pfad)"))
            .expect("oeffnen fragt die Erkennung");
        let faden = oeffnen
            .find(concat!("Ladevorgang::", "starten("))
            .expect("oeffnen startet den Faden");
        assert!(frage < faden, "die Erkennung steht nach dem Fadenstart");
        assert!(oeffnen.contains(concat!("(false, None) => Leseauftrag::", "Text,")));
        assert_eq!(
            oeffnen.matches(concat!("Leseauftrag::", "Text")).count(),
            1,
            "der Textauftrag hat genau eine Stelle in oeffnen"
        );

        let erkennung = rumpf_von(&code, concat!("fn ist_geheimnis", "datei("));
        assert!(erkennung.contains(concat!(".sonder", "datei(pfad)")));

        let lesen = concat!("datei::", "oeffnen(");
        assert_eq!(
            code.matches(lesen).count(),
            1,
            "{lesen} hat genau einen Rufer"
        );
        assert!(rumpf_von(&code, concat!("fn text_", "lesen(")).contains(lesen));
    }

    /// Kein Weg dieses Moduls schreibt Klartext nach `.secrets.txt`:
    ///
    /// - das atomare Schreiben steht genau einmal da, in `Chiffrat::schreiben`,
    ///   und das nimmt allein ein `Chiffrat`;
    /// - ein `Chiffrat` entsteht genau an einer Stelle, aus
    ///   `tresor::verschliessen`, nach der Sicherungsform;
    /// - der Klartextweg `datei::sichern` steht genau einmal da, im Sichern,
    ///   und hinter der Frage nach `.secrets.txt`;
    /// - in der ganzen Kiste ruft allein dieses Modul `datei::sichern`.
    #[test]
    fn kein_weg_schreibt_klartext_nach_secrets_txt() {
        let code = code_vor_den_proben("krk-ui/src/editormodell.rs");

        let atomar_schreiben = concat!("atomar::", "schreiben(");
        assert_eq!(code.matches(atomar_schreiben).count(), 1);
        let schreiben = rumpf_von(&code, concat!("fn schreiben(&self, ziel", ": &Path)"));
        assert!(schreiben.contains(atomar_schreiben));
        assert!(schreiben.contains(concat!("self.", "0.as_slice()")));

        assert_eq!(code.matches(concat!("Chiffrat", "(")).count(), 2);
        assert!(code.contains(concat!("struct Chiffrat", "(Vec<u8>);")));
        let verschliessen = rumpf_von(&code, concat!("fn verschliessen(stand", ": &str"));
        assert!(verschliessen.contains(concat!("Ok(Chiffrat", "(bytes))")));
        let form = verschliessen
            .find(concat!("datei::sicherungs", "form(stand)"))
            .expect("die Sicherungsform zuerst");
        let tresor = verschliessen
            .find(concat!("tresor::", "verschliessen("))
            .expect("verschlossen wird im Tresor");
        assert!(form < tresor);
        assert_eq!(
            code.matches(concat!("tresor::", "verschliessen(")).count(),
            1
        );

        let klartext = concat!("datei::", "sichern(");
        assert_eq!(code.matches(klartext).count(), 1);
        let sichern = rumpf_von(&code, concat!("fn sichern_", "ueber("));
        let frage = sichern
            .find(concat!("self.ist_geheimnis", "datei(&pfad)"))
            .expect("der Klartextweg fragt die Erkennung");
        let schreiben = sichern.find(klartext).expect("der Klartextweg");
        assert!(frage < schreiben);

        let rufer: Vec<String> = crate::quellbaum::quelldateien()
            .into_iter()
            .filter(|(name, _)| name.starts_with("krk-ui/"))
            .filter(|(_, inhalt)| {
                crate::quellbaum::codezeilen(inhalt).any(|zeile| zeile.contains(klartext))
            })
            .map(|(name, _)| name)
            .collect();
        assert_eq!(rufer, vec!["krk-ui/src/editormodell.rs".to_owned()]);
    }

    /// Die Form des PIN-Blattes folgt der Groesse (Schritt 5.4b): null Bytes
    /// heisst festlegen, jede andere Groesse eingeben, und eine Datei ohne
    /// erhebbare Groesse wird eingegeben.
    #[test]
    fn die_form_des_pin_blattes_folgt_der_groesse() {
        assert_eq!(Pinform::nach_groesse(Some(0)), Pinform::Festlegen);
        assert_eq!(Pinform::nach_groesse(Some(1)), Pinform::Eingeben);
        assert_eq!(Pinform::nach_groesse(Some(4096)), Pinform::Eingeben);
        assert_eq!(Pinform::nach_groesse(None), Pinform::Eingeben);
    }

    // ---------------------------------------------------------------------
    // „PIN ändern" (Schritt 5.5 des Plans der krkhome-Arbeit)
    // ---------------------------------------------------------------------

    /// Holt den Ausgang des PIN-Wechsels ab, mit derselben Schranke wie
    /// [`abwarten_mit_ableitung`]: die Ableitung laeuft mit den Parametern des
    /// Codes. `schreiben` ist der Schreibweg der Naht; jede Runde, in der noch
    /// nichts geliefert ist, ruft ihn nicht.
    fn pinwechsel_abwarten(
        modell: &mut Editormodell,
        mut schreiben: impl FnMut(&Path, &Chiffrat) -> io::Result<()>,
    ) -> Pinwechselausgang {
        for _ in 0..30_000 {
            if let Some(ausgang) =
                modell.pinwechsel_einziehen_ueber(|ziel, chiffrat| schreiben(ziel, chiffrat))
            {
                return ausgang;
            }
            thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("der Faden der neuen PIN hat innerhalb von dreissig Sekunden nichts geliefert");
    }

    /// Der Alltagsweg des Schreibens, fuer [`pinwechsel_abwarten`].
    fn alltagsweg(ziel: &Path, chiffrat: &Chiffrat) -> io::Result<()> {
        chiffrat.schreiben(ziel)
    }

    /// Ein Modell, das `.secrets.txt` mit kleinen Parametern und der PIN
    /// `0417` entsperrt haelt.
    fn entsperrt(ordner: &Pruefordner, klartext: &str) -> (Editormodell, PathBuf) {
        let (mut modell, pfad, _) = geheimnis_modell(ordner);
        verschlossen_ablegen(&pfad, klartext, "0417");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("0417"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);
        assert!(modell.pin_aenderbar());
        (modell, pfad)
    }

    /// C7.15 im Modell: nach dem Aendern oeffnet allein die neue PIN die
    /// Datei und die alte nicht; das Salz ist ein anderes, und die Parameter
    /// sind die des Codes, auch wenn die Datei mit kleineren geschrieben war.
    /// Ungesicherte Aenderungen stehen danach noch im Stand und nicht in der
    /// Datei; kein Klartext steht in der Datei und keiner im Bild der
    /// Nachbardatei, das die Probe am Schreibweg abfaengt. Das naechste
    /// Sichern meldet keine fremde Aenderung und verschliesst mit dem neuen
    /// Schluessel.
    #[test]
    fn nach_dem_aendern_oeffnet_allein_die_neue_pin() {
        let ordner = Pruefordner::neu("geheim-pin-aendern");
        let gesichert = format!("## Konto\n{GEHEIM}\n");
        let (mut modell, pfad) = entsperrt(&ordner, &gesichert);
        let salz_vorher = *tresor::Kopf::lesen(&std::fs::read(&pfad).expect("lesen"))
            .expect("Kopf")
            .salz();
        let getippt = format!("{gesichert}## Neu\nungesichert\n");
        let _ = modell.bearbeiten(getippt.clone());

        assert_eq!(modell.pin_aendern(pin("0417"), pin("8642")), Ok(()));
        assert!(modell.pin_wechselt());
        let mut abbild = None;
        let ausgang = pinwechsel_abwarten(&mut modell, |ziel, chiffrat| {
            let nachbar = atomar::vorbereiten(ziel, &mut chiffrat.0.as_slice())?;
            abbild = Some(std::fs::read(nachbar.nachbarpfad())?);
            nachbar.umbenennen()
        });
        assert_eq!(ausgang, Pinwechselausgang::Geaendert(pfad.clone()));
        assert!(!modell.pin_wechselt());

        let abbild = abbild.expect("der Schreibweg wurde gerufen");
        for nadel in [GEHEIM, "Konto", "ungesichert"] {
            assert!(!enthaelt(&abbild, nadel), "die Nachbardatei traegt {nadel}");
        }
        let platte = std::fs::read(&pfad).expect("lesen");
        assert_eq!(platte, abbild);
        let kopf = tresor::Kopf::lesen(&platte).expect("Kopf");
        assert_ne!(
            *kopf.salz(),
            salz_vorher,
            "die neue PIN zieht ein neues Salz"
        );
        assert_eq!(kopf.parameter(), tresor::Parameter::DES_CODES);

        assert_eq!(
            tresor::oeffnen(&platte, &pin("0417")).map(|_| ()),
            Err(tresor::Oeffnungsfehler::PinFalschOderVeraendert),
            "die alte PIN oeffnet nicht mehr"
        );
        let neu = tresor::oeffnen(&platte, &pin("8642")).expect("die neue PIN oeffnet");
        assert_eq!(
            neu.klartext,
            gesichert.as_bytes(),
            "umgeschluesselt ist der Stand der Platte und nicht der des Editors"
        );

        assert_eq!(modell.stand(), getippt);
        assert!(modell.hat_ungesicherten_stand());
        assert!(modell.pin_aenderbar(), "der neue Kopf steht auf der Platte");
        assert!(!modell.fremd_geaendert(), "der Stempel steht neu");

        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));
        let danach = std::fs::read(&pfad).expect("lesen");
        assert_eq!(
            tresor::Kopf::lesen(&danach).expect("Kopf").salz(),
            kopf.salz(),
            "gesichert wird mit dem neuen Schluessel"
        );
        assert_eq!(
            tresor::oeffnen_mit(&danach, &neu.schluessel).expect("oeffnet"),
            getippt.as_bytes()
        );
    }

    /// Eine falsche alte PIN weist ab, bevor irgendetwas anlaeuft: kein
    /// Faden, keine Bytes geaendert, der Schutz wie vorher.
    #[test]
    fn eine_falsche_alte_pin_weist_ab_und_laesst_alles_stehen() {
        let ordner = Pruefordner::neu("geheim-pin-falsch");
        let (mut modell, pfad) = entsperrt(&ordner, "## A\nx\n");
        let vorher = std::fs::read(&pfad).expect("lesen");

        let grund = modell
            .pin_aendern(pin("1111"), pin("2222"))
            .expect_err("die falsche alte PIN weist ab");
        assert!(grund.contains("alte PIN stimmt nicht"), "{grund}");
        assert!(grund.contains("die PIN bleibt, wie sie war"), "{grund}");
        assert!(!modell.pin_wechselt());
        assert_eq!(modell.pinwechsel_einziehen(), None);
        assert_eq!(std::fs::read(&pfad).expect("lesen"), vorher);
        assert!(matches!(
            &modell.schutz,
            Schutz::Verschluesselt { pin: gehalten, .. } if *gehalten == pin("0417")
        ));
    }

    /// Eine Datei, die sich aussen geaendert hat, weist das Aendern ab wie
    /// `sichern`: vor dem Blatt, und ebenso, wenn die Aenderung waehrend der
    /// Ableitung kommt. Die fremden Bytes bleiben stehen.
    #[test]
    fn eine_fremd_geaenderte_datei_weist_das_aendern_ab() {
        let ordner = Pruefordner::neu("geheim-pin-fremd");
        let (mut modell, pfad) = entsperrt(&ordner, "## A\nx\n");
        let fremd = b"von aussen, deutlich laenger als vorher";

        std::fs::write(&pfad, fremd).expect("schreiben");
        let grund = modell
            .pin_aenderung_pruefen()
            .expect_err("die fremde Aenderung haelt das Blatt an");
        assert!(grund.contains("außerhalb von KRK geändert"), "{grund}");
        assert!(modell.pin_aendern(pin("0417"), pin("2222")).is_err());
        assert!(!modell.pin_wechselt());

        // Der Ordner lebt bis zum Ende der Probe; ein Zeitwert raeumte ihn am
        // Ende der Zeile ab, und die Datei gaelte dann als fremd geaendert.
        let spaet = Pruefordner::neu("geheim-pin-fremd-spaet");
        let (mut modell, pfad) = entsperrt(&spaet, "## A\n");
        assert_eq!(modell.pin_aendern(pin("0417"), pin("2222")), Ok(()));
        std::fs::write(&pfad, fremd).expect("schreiben");
        match pinwechsel_abwarten(&mut modell, alltagsweg) {
            Pinwechselausgang::Gescheitert(grund) => {
                assert!(grund.contains("außerhalb von KRK geändert"), "{grund}");
            }
            anderer => panic!("erwartet war eine Abweisung, gekommen ist {anderer:?}"),
        }
        assert_eq!(std::fs::read(&pfad).expect("lesen"), fremd);
    }

    /// Ein Sichern waehrend der Ableitung geht nicht verloren: umgeschluesselt
    /// wird der Stand, der im Augenblick des Schreibens auf der Platte steht.
    #[test]
    fn ein_sichern_waehrend_des_aenderns_geht_nicht_verloren() {
        let ordner = Pruefordner::neu("geheim-pin-sichern");
        let (mut modell, pfad) = entsperrt(&ordner, "## A\nalt\n");
        assert_eq!(modell.pin_aendern(pin("0417"), pin("5555")), Ok(()));
        let _ = modell.bearbeiten(format!("## A\n{GEHEIM}\n"));
        assert_eq!(modell.sichern(), Sicherungsausgang::Gesichert(pfad.clone()));

        assert_eq!(
            pinwechsel_abwarten(&mut modell, alltagsweg),
            Pinwechselausgang::Geaendert(pfad.clone())
        );
        let platte = std::fs::read(&pfad).expect("lesen");
        assert!(!enthaelt(&platte, GEHEIM));
        let neu = tresor::oeffnen(&platte, &pin("5555")).expect("die neue PIN oeffnet");
        assert_eq!(neu.klartext, format!("## A\n{GEHEIM}\n").into_bytes());
    }

    /// Eine leere `.secrets.txt`, deren PIN eben festgelegt und nie gesichert
    /// wurde, hat keine PIN zu aendern: `pin_aenderbar` sagt nein, und das
    /// Modell weist ab, ohne einen Faden zu starten.
    #[test]
    fn eine_nie_gesicherte_pin_laesst_sich_nicht_aendern() {
        let ordner = Pruefordner::neu("geheim-pin-leer");
        let (mut modell, pfad, _) = geheimnis_modell(&ordner);
        std::fs::write(&pfad, b"").expect("leere Datei");
        assert_eq!(modell.oeffnen(&pfad, Some(pin("2468"))), None);
        assert_eq!(abwarten_mit_ableitung(&mut modell), Ladeausgang::Geoeffnet);

        assert!(!modell.pin_aenderbar());
        let grund = modell
            .pin_aendern(pin("2468"), pin("1357"))
            .expect_err("ohne Kopf gibt es keine PIN zu aendern");
        assert!(grund.contains("erst sichern"), "{grund}");
        assert!(!modell.pin_wechselt());
        assert_eq!(std::fs::metadata(&pfad).expect("stat").len(), 0);
    }

    /// Wird die Datei waehrend der Ableitung geschlossen, meldet der Wechsel
    /// sich trotzdem, und zwar mit dem Satz, dass die PIN bleibt; geschrieben
    /// wird nichts.
    #[test]
    fn ein_schliessen_waehrend_des_aenderns_meldet_und_schreibt_nichts() {
        let ordner = Pruefordner::neu("geheim-pin-schliessen");
        let (mut modell, pfad) = entsperrt(&ordner, "## A\nx\n");
        let vorher = std::fs::read(&pfad).expect("lesen");
        assert_eq!(modell.pin_aendern(pin("0417"), pin("2222")), Ok(()));
        modell.schliessen();
        assert!(
            modell.pin_wechselt(),
            "der Wechsel wartet auf seine Meldung"
        );

        match pinwechsel_abwarten(&mut modell, alltagsweg) {
            Pinwechselausgang::Gescheitert(grund) => {
                assert!(grund.contains("nicht mehr offen"), "{grund}");
            }
            anderer => panic!("erwartet war eine Abweisung, gekommen ist {anderer:?}"),
        }
        assert_eq!(std::fs::read(&pfad).expect("lesen"), vorher);
    }

    /// Der Weg des Aenderns am Quelltext: die Ableitung laeuft auf einem
    /// benannten Faden, die alte PIN wird vor dem Start verglichen, und
    /// geschrieben wird allein ein `Chiffrat` aus `Chiffrat::verschliessen`,
    /// entschluesselt mit dem gehaltenen Schluessel ohne Ableitung.
    #[test]
    fn das_aendern_leitet_auf_einem_faden_ab_und_schreibt_allein_chiffrat() {
        let code = code_vor_den_proben("krk-ui/src/editormodell.rs");
        let starten = rumpf_von(&code, concat!("fn starten(pfad: PathBuf, neue", ": Pin)"));
        assert!(starten.contains(concat!(".name(\"krk-", "pin\"")));
        assert!(starten.contains(concat!("tresor::neuer_", "schluessel(&neue)")));

        let aendern = rumpf_von(&code, concat!("pub fn pin_", "aendern(&mut self"));
        let vergleich = aendern
            .find("*pin != alte")
            .expect("die alte PIN wird verglichen");
        let start = aendern
            .find(concat!("Pinwechsel::", "starten("))
            .expect("der Faden startet");
        assert!(vergleich < start);

        let umschluesseln = rumpf_von(&code, concat!("fn umschluesseln", "("));
        assert!(umschluesseln.contains(concat!("tresor::oeffnen_", "mit(&bytes, gehalten)")));
        assert!(umschluesseln.contains(concat!("Chiffrat::", "verschliessen(&stand, neuer)")));
        assert!(umschluesseln.contains(concat!("chiffrat_", "schreiben(pfad, &chiffrat)")));
        assert!(!umschluesseln.contains(concat!("datei::", "sichern(")));
        let frage = umschluesseln
            .find(concat!("self.fremd_", "geaendert()"))
            .expect("die fremde Aenderung wird gefragt");
        let lesen = umschluesseln
            .find(concat!("bis_zur_grenze_", "lesen("))
            .expect("gelesen wird");
        assert!(frage < lesen);
    }
}
