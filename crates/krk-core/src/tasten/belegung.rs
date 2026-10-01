//! Die Belegungsmaschine: welche Taste welche Funktion ausloest.
//!
//! ```text
//! resources/default-keymap.toml ──include_str!──> AUSLIEFERUNG
//!                                                      │
//!            ~/Library/.../KRK/keymap.toml ──Ablage──> Belegung ──> Nachschlag
//! ```
//!
//! # Eine Tabelle, kein Nebeneinander
//!
//! Schritt 7 hatte fuenf Tasten fest verdrahtet, damit der Durchstich eine
//! Auswahl bewegen kann. Diese Tabelle ist mit Schritt 11 **abgeloest und nicht
//! ergaenzt**: es gibt genau einen Weg von einem Tastendruck zu einer Funktion,
//! und er beginnt in `resources/default-keymap.toml`. Zwei Nachschlagewege
//! nebeneinander waeren zwei Wahrheiten darueber, welche Taste was ausloest,
//! und die erste Abweichung zwischen ihnen faende keine Pruefung.
//!
//! Dasselbe gilt fuer die Tastencodes: sie stehen allein in
//! [`TASTEN`](super::parser::TASTEN).
//!
//! # Die Nutzerdatei ersetzt, sie ergaenzt nicht
//!
//! `keymap.toml` haelt die **vollstaendige** Belegung des Nutzers, nicht seine
//! Abweichungen vom Auslieferungszustand. Wer eine Zeile daraus loescht, hat die
//! Funktion unbelegt gemacht; wer die Datei loescht, bekommt beim naechsten
//! Start die Auslieferungsbelegung. Der Weg dorthin ist [`laden`], und er geht
//! ueber [`Zugang::laden`] aus Schritt 10: ein zweiter Ablageweg entsteht nicht,
//! und jede Meldung nimmt [`ablage::melden`](crate::ablage::melden).
//!
//! Eine Belegung des Nutzers wird gegen den **Wortschatz** der
//! Auslieferungsbelegung geprueft: sie darf jede Kombination frei verteilen,
//! aber nur auf Funktionen, die KRK kennt. Funktionen, die ihre Datei nicht
//! nennt, treten unbelegt hinzu, damit die Belegungsansicht aus C3 jede Funktion
//! auffuehren kann und der Nutzer sie wieder erreichbar machen kann.
//!
//! **Den Zusteller `gehalten_von` nimmt [`Belegung::vom_nutzer`] aus dem
//! Wortschatz und nicht aus der Datei.** Er ist die tragende Haelfte der
//! Zustellerregel weiter unten: ein von Hand gesetztes `gehalten_von = "menue"`
//! an einem gebauten Befehl nahm ihn bis zum 260908 aus dem Ereignisabgriff und
//! liess [`Funktion::kommando`] `None` liefern — ohne Konflikt, ohne Meldung,
//! ohne Rueckfall auf die Auslieferung. Der Befehl stand danach in der
//! Belegungsansicht und tat nichts
//! (`shared/issues/260826-1223_*_die-nutzerdatei-setzt-den-zusteller-frei-*`).
//! In der geschriebenen Datei ist das Feld damit eine **Duldung beim Lesen** und
//! keine Eingabe; rueckwaertsvertraeglich ist das, weil die Belegungsansicht
//! denselben Wert zurueckschreibt.
//!
//! **`name` ist seit der Sprachtabelle wie `gehalten_von` eine Duldung beim
//! Lesen, und `reserviert_fuer` kommt weiter aus der Datei.** Der Name einer
//! Funktion kommt aus `crate::sprache` in der geltenden Sprache, geschluesselt
//! ueber [`Funktionsschluessel`] (Spec
//! `261001-0735_*_spec-oberflaeche-lokalisierbar-deutsch-und-franzoesisch.md`,
//! C3): ein Feld `name` in einer eigenen `keymap.toml` wird gelesen, damit
//! eine Datei aus einer frueheren Fassung ohne Fehler und ohne Meldung laedt,
//! und nie angezeigt; beim Sichern schreibt [`Belegung::sichern`] es in der
//! geltenden Sprache, damit die Datei lesbar bleibt, wenn der Nutzer sie ueber
//! „Tastaturdefinition oeffnen“ ansieht. Die Umbenennung ueber die eigene
//! Datei, die bis dahin niemandem schadete, ist damit gefallen, weil ein
//! deutscher Name in der Datei einer franzoesischen Oberflaeche schadete.
//! `reserviert_fuer` entscheidet weiter nicht, ob ein Befehl ankommt, haengt
//! allein einen Zusatz an den Text der Belegungsansicht
//! (`krk-ui/src/belegungsmodell.rs`, `funktionstext`), und bleibt deshalb
//! Sache der Datei: es aus dem Wortschatz zu nehmen verwuerfe still das
//! `reserviert_fuer` einer `keymap.toml` aus einer aelteren Fassung, mit dem
//! jene Stelle ausdruecklich rechnet.
//!
//! # Was ein Nachschlag antwortet
//!
//! Drei Faelle, siehe [`Nachschlag`]. Der dritte ist [`Nachschlag::Tippen`]:
//! eine Taste, die keiner Funktion gehoert und **keine Befehlstaste** haelt,
//! gehoert dem Tippen. Der Kern sagt nur, dass der Tastendruck dorthin faellt;
//! welches Zeichen es ist, entscheidet die Oberflaeche am Ereignis, weil dort
//! auch die Grossschreibung und die Eingabemethoden stehen. Wohin das Zeichen
//! danach geht, hat die Runde 10 geaendert — bis dahin in den Suchpuffer der
//! Sprungmarke aus C2 der Runde 1, seither in den Filtertext des sichtbaren
//! Tabs —, und der Kern der Belegung hat davon nichts gemerkt.
//!
//! # Schreibtasten und Befehlstasten
//!
//! **Umschalt und Wahl schreiben, Befehl und Steuerung befehlen.** Die vier
//! Zusatztasten aus [`ModMaske`] taugen alle als Teil einer Belegung, aber sie
//! taugen nicht alle als Grund, einen unbelegten Tastendruck vom Tippen
//! fernzuhalten: `shift` und `opt` sind auf jeder Tastatur der Weg zu einem
//! anderen **Zeichen** — auf einer deutschen zu `_`, zu jedem Grossbuchstaben,
//! zur Umschalt-Interpunktion und ueber die Wahltaste zu `@`, `|`, `~`, `\` —,
//! waehrend `cmd` und `ctrl` kein Zeichen aufschliessen, sondern einen Befehl
//! erwarten. Ein unbelegtes `cmd+irgendwas` tippt deshalb weiterhin nichts.
//!
//! Es ist der Zuschnitt, den macOS selbst faehrt. Bis zum 260816 stand hier
//! statt der Unterscheidung die Frage, ob die Maske **leer** ist; damit war
//! jede Taste mit Zusatztaste fuer den Filtertext verloren, und der
//! Unterstrich, den die Datensaetze dieses Projekts in jedem Dateinamen
//! tragen, war nicht zu tippen. Nutzerentscheid vom 260816-1105,
//! `shared/issues/260816-1101_*_kein-zeichen-mit-umschalttaste-erreicht-den-dateifilter.md`.
//!
//! **Die Unterscheidung steht hinter der Belegungssuche und kann keinem
//! belegten Kuerzel etwas wegnehmen.** [`Belegung::nachschlag`] durchlaeuft
//! erst alle Funktionen und kommt nur dorthin, wenn keine passt; `shift+f2`
//! bleibt also `shift+f2`, gleich was hier steht.
//!
//! # Wonach nachgeschlagen wird
//!
//! **Buchstaben und Ziffern ueber das gemeldete Zeichen, alles uebrige ueber
//! den virtuellen Tastencode.** Die Regel steht in
//! [`Tastenkennung`](super::parser::Tastenkennung), ihre Begruendung im Kopf
//! von [`super::parser`], und der Vergleich in [`Belegung::nachschlag`], wo
//! auch steht, warum eine zweite Nachschlagart hier keine Sonderregel ist.
//!
//! # Der Zusteller, und was er fuer den Konflikt bedeutet
//!
//! Seit Schritt 13b fuehrt die Belegung Funktionen, die nicht der
//! Ereignisabgriff ausfuehrt, sondern das Hauptmenue zustellt. Wer zustellt,
//! steht in [`Funktion::gehalten_von`]: ohne das Feld der Abgriff, mit dem Wert
//! `menue` das Hauptmenue. Der Fokusvorbehalt aus C2 teilt jeden Tastendruck
//! genau einem der beiden zu, eine Funktion ist deshalb in genau einem
//! Fokuszustand erreichbar.
//!
//! Daraus folgt die vollstaendige Regel, und sie ist eine Regel und keine
//! Ausnahme fuer eine einzelne Taste: **zwei Funktionen sind genau dann ein
//! Konflikt, wenn sie dieselbe Kombination tragen, denselben Zusteller haben
//! und ihre Wirkungsbereiche einander nicht ausschliessen.** Die zweite Haelfte
//! steht im naechsten Abschnitt. Die Regel greift an vier Stellen, und keine
//! davon ist entbehrlich:
//!
//! | Stelle | Warum |
//! |---|---|
//! | [`Belegung::konflikte`] | jedes Einlesen laeuft ueber [`Belegung::bauen`] darauf; fragt `begegnen` |
//! | [`Belegung::zuweisen`] | die Umbelegung durch den Nutzer aus C3; fragt dasselbe `begegnen` |
//! | [`Belegung::nachschlag`] | der Abgriff darf nur sehen, was er selbst zustellt |
//! | [`Funktion::kommando`] | eine zugestellte Funktion fuehrt KRK nie selbst aus |
//!
//! Die dritte Stelle traegt die Regel erst: ohne das Ueberspringen haenge das
//! Verhalten an der Reihenfolge der Eintraege. Stuende `text_alles_auswaehlen`
//! in der Datei des Nutzers vor `alle_markieren`, faende der Abgriff eine
//! Funktion ohne Kommando, reichte den Tastendruck weiter, und das Markieren
//! aller Eintraege waere still tot.
//!
//! Nutzerentscheid vom 260805,
//! `decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`.
//!
//! # Zwei Funktionen eines Zustellers auf einer Kombination
//!
//! **Seit dem 260926 duerfen zwei Funktionen desselben Zustellers eine
//! Kombination tragen, und es gibt dafuer zwei Arten des Teilens.** Beide
//! entscheidet der Kern aus Eingaben, die er selbst hat:
//!
//! - **Ausschluss**: jeder [`Wirkungsbereich`] steht ueber eine vollstaendige
//!   Fallunterscheidung auf einer [`Seite`], und zwei Bereiche schliessen
//!   einander genau dann aus, wenn der eine allein im Editor und der andere
//!   allein ausserhalb wirkt ([`Wirkungsbereich::schliesst_aus`]). Dann ist nie
//!   mehr als eine der zwei zulaessig.
//! - **Verengung** (seit der Bildfolge, Entscheid
//!   `260929-1423_*_wie-teilen-zwei-funktionen-eine-kombination-wenn-die-eine-nur-bei-stehender-bildfolge-wirkt.md`,
//!   Moeglichkeit 1): ein Bereich kann einen anderen verengen
//!   ([`Wirkungsbereich::weiter`], [`Wirkungsbereich::verengt`]); wo der engere
//!   zulaessig ist, ist es der weitere auch, und dann geht der engere vor.
//!   `Bildfolge` verengt `Dateifenster`: `cmd+up` heisst mit stehender
//!   Bildfolge „Voriges Bild“ und sonst „In den übergeordneten Ordner“.
//!
//! Eine Funktion ohne Kommando hat keinen Wirkungsbereich und teilt mit
//! keiner. **Eine Kombination traegt je Zusteller hoechstens zwei
//! Funktionen**, und das ist seit der Verengung eine Regel und keine Folgerung
//! mehr: eine Funktion des Editors, eine des Dateifensters und eine der
//! Bildfolge vertragen sich paarweise, und ohne die Regel ginge die dritte im
//! Nachschlag still verloren. `konflikte` und `zuweisen` fragen dafuer
//! dieselbe private Funktion `im_weg`, die den Bewerber gegen **alle**
//! bisherigen Traeger der Kombination haelt; zwei Fassungen der Regel gibt es
//! nicht.
//!
//! **Die Regel ist groeber als die wirkliche Zulaessigkeit**, und der Fehler
//! faellt immer auf die Seite des gemeldeten Konflikts: zwei Bereiche derselben
//! Seite, etwa `Eintraege` und `Editortext`, sind der Form nach nie zugleich
//! zulaessig und bleiben trotzdem ein Konflikt. Dass die Seiten mit der
//! wirklichen Regel uebereinstimmen, haelt in `krk-ui` die Probe
//! `einander_ausschliessende_bereiche_sind_nie_zugleich_zulaessig` ueber jeden
//! Fokus, jede Form und jede Lage.
//!
//! Der Nachschlag antwortet fuer eine solche Kombination mit
//! [`Nachschlag::Geteilt`], und welche der zwei Funktionen gemeint ist,
//! entscheidet die Oberflaeche aus der einen Lage, die sie je Eingabe ohnehin
//! erhebt; nach der Regel oben ist hoechstens eine zulaessig, oder die engere
//! geht vor, und der Nachschlag nennt sie deshalb zuerst. Im Hauptmenue
//! zeigt der Eintrag das Kuerzel, der in der Leiste frueher steht. Entscheid
//! `260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md`,
//! Moeglichkeit 1.
//!
//! # Der Wirkungsbereich: welcher Bereich den Fokus haben muss
//!
//! Seit Schritt 18 gibt es mehr als einen fokussierbaren Bereich: die beiden
//! Dateifenster und die Lesezeichenleiste aus C5, seit Schritt 19 dazu das
//! Vorschaufenster aus C6. Damit wird die Frage "darf
//! dieser Befehl hier ueberhaupt wirken" fuer **jedes** Kommando faellig:
//! `delete` darf in der Leiste keine Datei loeschen, `right` dort in keinen
//! Ordner einsteigen, und `lesezeichen_loeschen` umgekehrt nicht wirken,
//! solange der Fokus im Dateifenster steht.
//!
//! Die Antwort ist **eine Eigenschaft je Kommando** und keine Abfrage je
//! Aufrufstelle: [`Kommando::wirkungsbereich`]. Vier oder fuenf handgeschriebene
//! Vorbehalte an vier oder fuenf Stellen waeren das Dickicht aus Sonderregeln,
//! das die Maxime "supersimpel" ausschliesst; die Zuleitung in `krk-ui` fragt
//! die Eigenschaft **einmal**, bevor sie ein Kommando ausfuehrt. Die einzelne
//! Abfrage der Loeschtasten aus Schritt 16 ist darin aufgegangen.
//!
//! **Die Eigenschaft steht im Kern, die Antwort in der Oberflaeche.** Dass das
//! Raeumen in den Papierkorb das Dateifenster braucht, ist eine Aussage ueber
//! den Befehl und kein AppKit-Wissen; sie ist deshalb hier ohne Fenster
//! pruefbar.
//! Welcher Bereich den Fokus gerade hat, weiss allein `krk-ui`. Die
//! Aufrufrichtung bleibt von oben nach unten.
//!
//! **Der Wirkungsbereich hat seit der Runde 3 eine zweite Verwendung, und sie
//! ist keine Sperre, sondern eine Auskunft.** Neben dem stummen Fokusvorbehalt
//! liest ihn die Tastenbelegung als Markdown-Datei: ihre dritte Spalte sagt dem
//! Nutzer, wo ein Befehl wirkt, und nimmt den Text dafuer aus
//! [`Wirkungsbereich::beschriftung`]. Damit ist die Datei die einzige Stelle in
//! KRK, an der der stumme Vorbehalt ueberhaupt erklaert wird — wer sonst
//! `cmd+backspace` im Editor drueckt und nichts geschehen sieht, hat keinen Weg
//! zum Grund ausser dem Quelltext. Wer die Aufzaehlung erweitert, zieht deshalb
//! zwei Fallunterscheidungen nach und nicht eine.

use std::fmt;
use std::io;
use std::path::PathBuf;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::ablage::{Ablage, Beiseite, Datei, Ersetzung, Geladen, Grund, Zugang, melden};
use crate::sprache::{Sprache, Text, funktionsname, satz, text};

use super::konflikt::{Funktionsname, Konflikt};
use super::parser::{Kombination, Schreibfehler};
use super::{ModMaske, Tastendruck};

/// Die Auslieferungsbelegung, in das Programm einkompiliert.
///
/// Damit gibt es keinen Start ohne Belegung: eine fehlende, geloeschte oder
/// kaputte `keymap.toml` faellt immer auf diesen Text zurueck.
///
/// **Sichtbar und nicht modulprivat**, seit
/// [`ablage::neuerungen`](crate::ablage::neuerungen) die Nutzerdatei gegen sie
/// haelt: der Vergleich braucht den ausgelieferten Text selbst und nicht die
/// daraus gebaute [`Belegung`], denn er fragt, welche Funktionen die Datei
/// **nennt**, und nicht, welche KRK am Ende fuehrt — [`Belegung::bauen`] nimmt
/// jede ungenannte Funktion unbelegt hinzu und macht den Unterschied damit
/// unsichtbar. Die zwei Schwestern in [`crate::ablage`] sind aus demselben
/// Grund laengst sichtbar.
pub const AUSLIEFERUNGSTEXT: &str = include_str!("../../../../resources/default-keymap.toml");

/// Die gelesene Auslieferungsbelegung. Sie definiert den Wortschatz.
static AUSLIEFERUNG: LazyLock<Belegung> = LazyLock::new(|| {
    let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
        .expect("die eingebettete Auslieferungsbelegung ist kein gueltiges TOML");
    Belegung::bauen(&datei, None)
        .expect("die eingebettete Auslieferungsbelegung ist in sich nicht schluessig")
});

/// Welcher Bereich den Eingabefokus haben muss, damit ein Kommando wirkt (C5).
///
/// Wie viele Werte es sind, steht hier nicht: die Zahl stand bis zur
/// krkhome-Arbeit auf acht und ist mit ihr falsch geworden. Gezaehlt wird sie
/// mit `awk '/^pub enum Wirkungsbereich/,/^}/'` ueber diese Datei.
///
/// Vier davon tragen die Runde 1. KRK hatte seit Schritt 19 drei fokussierbare
/// Bereiche, die beiden Dateifenster, die Leiste und das Vorschaufenster, und
/// ein Befehl gehoerte einem Dateifenster, der Leiste, einem Bereich mit Tabs
/// oder keinem. [`Wirkungsbereich::Tabbereich`] ist mit dem Vorschaufenster aus
/// C6 entstanden: die vier Tabbefehle aus C1 bedienen nach C6 auch dessen Tabs,
/// und zwar in dem Bereich, der den Fokus gerade hat — so fuehrt es
/// `resources/default-keymap.toml` seit S9. **Drei von ihnen tragen den Wert
/// noch**; `tab_schliessen` ist mit C4 der Runde 4 zu
/// [`Wirkungsbereich::Ueberall`] gewechselt, und die Begruendung steht bei
/// [`Kommando::wirkungsbereich`] an seinem Zweig.
///
/// **Drei weitere kommen mit dem eingebauten Editor.** Jeder von ihnen ist
/// sachlich begruendet und nicht bequem:
///
/// - [`Wirkungsbereich::Dateibereiche`], weil der Rundweg in den Editor und
///   zurueck in den drei Bereichen wirkt, in denen eine Datei im Spiel ist.
/// - [`Wirkungsbereich::Editor`], weil die Befehle aus C3, C4, C5 und C6 der
///   Editor-Runde allein im Editor wirken.
/// - [`Wirkungsbereich::Navigator`], weil drei Befehle bis dahin
///   [`Wirkungsbereich::Ueberall`] trugen, deren Taste im Editor der
///   Textflaeche gehoert.
///
/// **Der erste der drei hiess bis zum 260823 `Vorschau` und wirkte allein im
/// Vorschaufenster.** Er trug genau einen Befehl, den Uebergang aus der
/// Vorschau in den Editor; seit der Nutzerentscheid vom 260823-0942 daraus den
/// Rundweg gemacht hat, wirkt derselbe Befehl in drei Bereichen, und der Wert
/// fuer die Vorschau allein ist mit seinem einzigen Traeger gefallen. Der
/// Datensatz ist
/// `shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`.
///
/// **Der achte ist derselbe Wert, mit der Runde 20 zurueckgekommen:**
/// [`Wirkungsbereich::Vorschau`] traegt die drei Zoombefehle des
/// PDF-Betrachters, die allein im Vorschaufenster etwas bedeuten. Kein anderer
/// Wert sagt das: `Dateibereiche` schliesst Dateifenster und Editor ein,
/// `Tabbereich` das Dateifenster, `Navigator` Dateifenster und Leiste.
///
/// **Drei weitere kommen mit den Eintragstabellen aus `~/krkhome/`** (Schritt
/// 3.3 der krkhome-Arbeit): [`Wirkungsbereich::Editortext`],
/// [`Wirkungsbereich::Eintraege`] und [`Wirkungsbereich::Aufgaben`]. Alle drei
/// verlangen den Fokus im Editor wie [`Wirkungsbereich::Editor`] und dazu eine
/// Form dessen, was er zeigt. Die Form kennt der Kern nicht; `krk_ui` fragt sie
/// in derselben Regel, die den Fokus fragt, und die Ausgrauung des Menues folgt
/// daraus ohne eigenen Weg. Ein vierter, [`Wirkungsbereich::Geheimnisse`],
/// kommt mit Schritt 5.5 fuer „PIN ändern" und fragt statt der Form, ob der
/// Editor `secrets.txt` mit einem Kopf auf der Platte haelt. Seit dem 260926
/// traegt das Verschieben [`Wirkungsbereich::Reihenfolge`], weil die
/// Termintabelle nach dem Datum ordnet und es dort nicht annimmt, und das
/// Umkehren ihrer Richtung [`Wirkungsbereich::Termine`], weil es allein in ihr
/// etwas bedeutet. Seit dem 260927 tragen das Kopieren und das Leeren der
/// Quicknote [`Wirkungsbereich::Quicknote`], weil sie allein an deren Puffer
/// wirken und der Editor ihn nur in dieser Form zeigt.
///
/// Der Preis dafuer, dass der Fokusvorbehalt **eine** Regel bleibt und keine
/// Abfrage je Aufrufstelle wird. Neue Werte in einer Aufzaehlung sind
/// billiger als handgeschriebene Sonderfaelle im Code.
///
/// **Der Vorbehalt ist stumm.** Ein Kommando, das hier scheitert, tut nichts
/// und meldet nichts; der Tastendruck geht unveraendert an AppKit weiter, wie
/// jeder unbelegte. Eine Meldung waere die Sonderregel, die die Maxime
/// "supersimpel" ausschliesst, und die drei Abnahmekriterien aus C5 verlangen
/// von `delete`, `right` und `lesezeichen_loeschen` ausdruecklich nur, dass sie
/// nichts tun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wirkungsbereich {
    /// Wirkt nur, wenn der Fokus in einem Dateifenster steht.
    Dateifenster,
    /// Wirkt nur, wenn der Fokus in der Lesezeichen- und Geraeteleiste steht
    /// (C5).
    Leiste,
    /// Wirkt in einem Dateifenster, im Vorschaufenster und im Editor, aber
    /// nicht in der Lesezeichen- und Geraeteleiste.
    ///
    /// Der Wert eines einzigen Befehls, des Rundwegs in den Editor und zurueck
    /// (`editor_rundweg`, Nutzerentscheid vom 260823-0942). Er bedeutet in
    /// jedem der drei Bereiche etwas: in der Dateiliste den ausgewaehlten
    /// Eintrag oeffnen, in der Vorschau die angezeigte Datei uebernehmen, im
    /// Editor ihn wieder schliessen. In der Leiste bedeutet er nichts, denn
    /// dort gibt es keine Datei, die er meinte.
    ///
    /// **Positiv aufgezaehlt und nicht als Verneinung der Leiste**, aus
    /// demselben Grund wie bei [`Wirkungsbereich::Navigator`]: "ueberall ausser
    /// in der Leiste" liesse den Fokuswert `Anderswo` durch, den `krk_ui` fuer
    /// ein stehendes Blatt und fuer ein Textfeld fuehrt, und der Rundweg wirkte
    /// dann vor einer Rueckfrage.
    ///
    /// **Bis zum 260823 hiess der Wert `Vorschau`** und verlangte den Fokus im
    /// Vorschaufenster. Die Vorschau-Richtung des Befehls ist unveraendert
    /// geblieben; hinzugekommen sind die Dateiliste und der Rueckweg aus dem
    /// Editor. Den Namen traegt seit der Runde 20 wieder ein eigener Wert,
    /// [`Wirkungsbereich::Vorschau`], mit den drei Zoombefehlen als Traegern.
    Dateibereiche,
    /// Wirkt nur, wenn der Fokus im eingebauten Editor steht (C3 bis C6 der
    /// Editor-Runde).
    ///
    /// Sichern, die Ansicht umschalten und schliessen wirken allein in der
    /// Datei, die der Editor haelt. Mit dem Fokus anderswo gibt es keine
    /// solche Datei. Sie wirken in jeder Form, in der der Editor die Datei
    /// zeigt, als Text und als Eintragstabelle.
    Editor,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor die
    /// Textflaeche zeigt (Schritt 3.3 der krkhome-Arbeit).
    ///
    /// Der Wert des Zeilensprungs, der Suche und des Ersetzens: sie setzen
    /// Schreibmarke und Auswahl der Textflaeche. Zeigt der Editor eine
    /// Eintragstabelle aus `~/krkhome/`, ist die Textflaeche ausgeblendet, und
    /// ein Treffer landete unsichtbar darin. **Welche Form der Editor zeigt,
    /// weiss der Kern nicht**; die Frage stellt `krk_ui` in seiner
    /// Zulaessigkeitsregel, und dieser Wert sagt nur, dass sie zu stellen ist.
    Editortext,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor eine
    /// Eintragstabelle zeigt (C6 des Spec
    /// `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
    ///
    /// Der Wert der drei Befehle, die Eintraege anlegen, bearbeiten und
    /// loeschen. Die Form fragt, wie bei [`Wirkungsbereich::Editortext`],
    /// `krk_ui`. Das Verschieben traegt seit dem 260926
    /// [`Wirkungsbereich::Reihenfolge`].
    Eintraege,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor eine
    /// Eintragstabelle **in Dateireihenfolge** zeigt.
    ///
    /// Der Wert der zwei Befehle, die einen Eintrag nach oben oder unten
    /// verschieben. Eine Tabelle, die ihre Zeilen nach etwas anderem ordnet als
    /// der Datei, nimmt sie nicht an: die Termintabelle ordnet nach dem Datum,
    /// und dort hiesse „nach oben" nichts, was der Nutzer sieht (Entscheidung 2
    /// des Plans `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
    /// Ein eigener Wert und keine Frage nach dem Kommando, weil die
    /// Zulaessigkeit in `krk_ui` allein den Wirkungsbereich und die Form fragt;
    /// der Menueeintrag ist dort damit ausgegraut. Die Form fragt, wie bei
    /// [`Wirkungsbereich::Eintraege`], `krk_ui`.
    Reihenfolge,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor die
    /// Aufgabentabelle zeigt (C6).
    ///
    /// Der Wert allein des Abhakens: eine Notiz hat kein Kaestchen.
    Aufgaben,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor die
    /// Termintabelle zeigt (Schritt 8 des Plans
    /// `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
    ///
    /// Der Wert allein von „Termine: Sortierrichtung umkehren": eine andere
    /// Tabelle ordnet nicht nach dem Datum und hat keine Richtung, die sich
    /// umkehren liesse. Die Form fragt, wie bei [`Wirkungsbereich::Eintraege`],
    /// `krk_ui`. Er steht auf der Seite des Editors und teilt sich deshalb
    /// `cmd+1` mit dem Sortieren nach Name im Dateifenster (Entscheid
    /// `260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md`).
    Termine,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor
    /// `secrets.txt` entsperrt haelt, deren PIN schon in einem Kopf auf der
    /// Platte steht (C7 des Spec, Schritt 5.5 der krkhome-Arbeit).
    ///
    /// Der Wert allein von „PIN ändern". **Ob die Datei gehalten wird und einen
    /// Kopf traegt, weiss der Kern nicht**; die Frage stellt `krk_ui` in seiner
    /// Zulaessigkeitsregel, wie bei [`Wirkungsbereich::Editortext`] die Form.
    /// Die Form selbst fragt der Wert nicht: auch in der Rohansicht haelt der
    /// Editor dieselbe Datei mit derselben PIN.
    Geheimnisse,
    /// Wirkt nur, wenn der Fokus im Editor steht und der Editor die Quicknote
    /// zeigt (Q2 des Spec
    /// `260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`).
    ///
    /// Der Wert des Kopierens und des Leerens: beide wirken allein am Puffer
    /// der Quicknote, und mit einer Datei im Editor gibt es keinen. **Ob die
    /// Quicknote offen ist, weiss der Kern nicht**; die Form fragt, wie bei
    /// [`Wirkungsbereich::Editortext`], `krk_ui`. Das Oeffnen und Schliessen
    /// traegt [`Wirkungsbereich::Ueberall`], weil es die Quicknote aus jedem
    /// Bereich heraus holt.
    Quicknote,
    /// Wirkt, wenn der Fokus in einem Bereich mit Tabs steht: in einem
    /// Dateifenster oder im Vorschaufenster (C1, C6).
    ///
    /// Der Wert von drei der vier Tabbefehle: [`Kommando::TabNeu`],
    /// [`Kommando::TabNaechster`] und [`Kommando::TabVoriger`]. C6 verlangt
    /// fuer die Vorschau-Tabs "dieselben Befehle zum Oeffnen, Schliessen und
    /// Wechseln wie in C1", und dieselben Befehle heisst: dieselben vier
    /// Kommandos, gerichtet an den Bereich vor dem Nutzer. Die Leiste traegt
    /// keine Tabs und bleibt aussen vor.
    ///
    /// **Der vierte ist seit C4 der Runde 4 keiner mehr.**
    /// [`Kommando::TabSchliessen`] traegt [`Wirkungsbereich::Ueberall`]: es
    /// schliesst einen Tab und setzt deshalb keinen Bereich mit Tabs im Fokus
    /// voraus, sondern eine aktive Fensterseite, und die gibt es immer. Die
    /// Zuordnung "der Bereich vor dem Nutzer" gilt fuer ihn unveraendert
    /// weiter, sobald der Fokus in einem Dateifenster oder in der Vorschau
    /// steht; sie entsteht dann aber beim Aufrufer und nicht mehr hier. Die
    /// Begruendung im Langen steht bei [`Kommando::wirkungsbereich`].
    Tabbereich,
    /// Wirkt in den Bereichen des Navigators, also im Dateifenster, in der
    /// Leiste, im Vorschaufenster und im Git-Bereich, aber nicht im Editor.
    ///
    /// **Der Git-Bereich ist seit der Runde 23 dabei**, und aus der Regel, die
    /// die drei anderen schon traegt: der Verlauf ist eine Liste mit einer
    /// Auswahl, und der Auf- und der Ab-Pfeil bewegen die Auswahl der Liste,
    /// vor der der Nutzer steht. Ein neunter Wirkungsbereich waere dafuer ein
    /// eigener Wert fuer eine Regel, die es schon gibt.
    ///
    /// Der Wert der Befehle, deren Taste im Editor der Textflaeche gehoert:
    /// `fenster_wechseln` auf `tab`, `auswahl_hoch` auf `up` und
    /// `auswahl_runter` auf `down`. Sie sind in der Runde 1 mit
    /// [`Wirkungsbereich::Ueberall`] entstanden, weil es damals nichts gab,
    /// wovon sie auszunehmen waeren. Ohne diesen Wert bewegte `up` im Editor
    /// die Auswahl im Dateifenster statt der Schreibmarke, und das erste
    /// Abnahmekriterium von C7 waere gebrochen.
    ///
    /// **Positiv formuliert und nicht als Verneinung von
    /// [`Wirkungsbereich::Editor`].** Der Unterschied zaehlt: ein stehendes
    /// Blatt und ein Textfeld sind kein Bereich des Navigators und bleiben
    /// damit ausgeschlossen, so wie sie es unter `Dateifenster` und `Leiste`
    /// schon sind. "Ueberall ausser im Editor" schloesse sie ein, und ein
    /// `up` vor der Loeschrueckfrage bewegte die Auswahl im Ordner dahinter.
    Navigator,
    /// Wirkt nur, wenn der Fokus im Vorschaufenster steht.
    ///
    /// Der Wert der drei Zoombefehle des PDF-Betrachters aus der Runde 20:
    /// [`Kommando::VorschauVergroessern`], [`Kommando::VorschauVerkleinern`]
    /// und [`Kommando::VorschauAusgangsgroesse`]. Sie veraendern die Ansicht
    /// des Betrachters, und der steht allein im Vorschaufenster; mit dem Fokus
    /// im Dateifenster, in der Leiste oder im Editor haben sie keinen
    /// Gegenstand, und ihre Menueeintraege sind dort ausgegraut (C3.5 der
    /// Runde 20).
    ///
    /// **Bis zum 260823 stand der Wert schon einmal hier, mit einem Traeger**,
    /// dem Uebergang aus der Vorschau in den Editor; der ist im Rundweg
    /// aufgegangen und traegt seither [`Wirkungsbereich::Dateibereiche`]. Der
    /// Wert ist mit dem Verlust seines Traegers gefallen und mit drei neuen
    /// zurueckgekommen. Die Alternative, die drei mit
    /// [`Wirkungsbereich::Ueberall`] durchzulassen und im Ausfuehrungszweig
    /// nach dem Fokus zu fragen, waere die Abfrage je Aufrufstelle, die der
    /// Modulkopf ausschliesst, und sie graute den Menueeintrag nicht aus.
    ///
    /// Ob ein PDF angezeigt wird, fragt der Wert nicht: die Zulaessigkeit
    /// haengt am Fokus und nicht am Inhalt (A6 der Runde 20). Mit dem Fokus in
    /// der Vorschau und ohne PDF werden die drei entgegengenommen und tun
    /// nichts.
    Vorschau,
    /// Wirkt nur, wenn der Fokus in einem Dateifenster steht **und** die
    /// Vorschau eine Bildfolge zeigt (Spec
    /// `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`, C3
    /// und C4).
    ///
    /// Der Wert der drei Befehle der Bildfolge: [`Kommando::BildVor`],
    /// [`Kommando::BildZurueck`] und [`Kommando::ZumBild`]. Er **verengt**
    /// [`Wirkungsbereich::Dateifenster`] ([`Wirkungsbereich::weiter`]): wo er
    /// zulaessig ist, ist es jener auch, und deshalb duerfen `cmd+up` und
    /// `return` je einem Befehl beider Bereiche gehoeren. **Ob die Vorschau eine
    /// Bildfolge zeigt, weiss der Kern nicht**; die Frage stellt `krk_ui` in
    /// seiner Zulaessigkeitsregel, wie bei [`Wirkungsbereich::Editortext`] die
    /// Form.
    Bildfolge,
    /// Wirkt ohne Vorbehalt.
    ///
    /// Zwei Sorten von Befehlen tragen ihn. Die einen gehoeren dem Fenster als
    /// ganzem und keinem Bereich darin: beenden, ein Fenster schliessen, einen
    /// Bereich ein- und ausblenden. Die anderen sind die fuenf Fokusbefehle
    /// und das Anlegen eines Lesezeichens: ein Befehl, der den Fokus **holt**,
    /// kann nicht voraussetzen, wo er gerade steht.
    ///
    /// Der Auf- und der Ab-Pfeil standen bis zum eingebauten Editor hier, weil
    /// sie die Auswahl des Bereichs bewegen, der den Fokus hat. Sie tragen
    /// seither [`Wirkungsbereich::Navigator`]: im Editor bewegen dieselben
    /// beiden Tasten die Schreibmarke.
    Ueberall,
}

impl Wirkungsbereich {
    /// Die Beschriftung dieses Bereichs fuer den Nutzer.
    ///
    /// **Fuer den Nutzer bestimmt und nicht fuer den Programmtext.** Der Name
    /// der Variante ist die Auskunft an den Leser dieser Datei; die
    /// Beschriftung ist die an den Leser der Tastenbelegung als Markdown.
    /// Deshalb steht hier "Dateifenster, Leiste, Vorschau und Git-Bereich" und
    /// nicht "Navigator": eine Datei, die ihre Spalte erst ueber eine Legende
    /// verstaendlich macht, verlangt vom Leser genau das Wissen, das sie ihm
    /// geben soll. Ausgeschrieben, ohne Legende, Nutzerentscheid vom
    /// 260811-0115.
    ///
    /// **Vollstaendige Fallunterscheidung ohne Auffangzweig**, nach dem Vorbild
    /// von [`Kommando::wirkungsbereich`] darueber. Ein weiterer Wert der
    /// Aufzaehlung braucht hier eine Zeile, bevor er uebersetzt; ein `_`-Zweig
    /// gaebe ihm stillschweigend die Beschriftung eines Nachbarn und damit eine
    /// falsche Zusicherung in einer Datei, die der Nutzer liest.
    ///
    /// Keine zwei Werte tragen dieselbe Beschriftung. Zwei gleiche Texte waeren
    /// eine Spalte, die zwei verschiedene Regeln gleich benennt; die Probe
    /// `keine_zwei_wirkungsbereiche_teilen_sich_eine_beschriftung` haelt es
    /// fest.
    ///
    /// **Der Wortlaut steht in der Sprachtabelle** (`crate::sprache`), hier
    /// steht je Wert sein Schluessel; die Beschriftung kommt in der geltenden
    /// Sprache. Deshalb ist die Funktion seit der Sprachtabelle kein
    /// `const fn` mehr: die geltende Sprache ist ein Wert des Prozesses.
    #[must_use]
    pub fn beschriftung(self) -> &'static str {
        match self {
            Wirkungsbereich::Dateifenster => text(Text::WirkungsbereichDateifenster),
            Wirkungsbereich::Leiste => text(Text::WirkungsbereichLeiste),
            Wirkungsbereich::Dateibereiche => text(Text::WirkungsbereichDateibereiche),
            Wirkungsbereich::Editor => text(Text::WirkungsbereichEditor),
            Wirkungsbereich::Editortext => text(Text::WirkungsbereichEditortext),
            Wirkungsbereich::Eintraege => text(Text::WirkungsbereichEintraege),
            Wirkungsbereich::Reihenfolge => text(Text::WirkungsbereichReihenfolge),
            Wirkungsbereich::Aufgaben => text(Text::WirkungsbereichAufgaben),
            Wirkungsbereich::Termine => text(Text::WirkungsbereichTermine),
            Wirkungsbereich::Geheimnisse => text(Text::WirkungsbereichGeheimnisse),
            Wirkungsbereich::Quicknote => text(Text::WirkungsbereichQuicknote),
            Wirkungsbereich::Tabbereich => text(Text::WirkungsbereichTabbereich),
            Wirkungsbereich::Navigator => text(Text::WirkungsbereichNavigator),
            Wirkungsbereich::Vorschau => text(Text::WirkungsbereichVorschau),
            Wirkungsbereich::Bildfolge => text(Text::WirkungsbereichBildfolge),
            Wirkungsbereich::Ueberall => text(Text::WirkungsbereichUeberall),
        }
    }

    /// Auf welcher Seite des Editors dieser Bereich wirkt.
    ///
    /// **Vollstaendige Fallunterscheidung ohne Auffangzweig**: ein weiterer
    /// Wert haelt den Bau hier an und bekommt seine Seite bewusst, denn an ihr
    /// haengt, ob er eine Kombination mit einer anderen Funktion teilen darf
    /// (Modulkopf, „Zwei Funktionen eines Zustellers auf einer Kombination“).
    /// Die Seite folgt der Tafel von `fokus::wirkt` in `krk-ui`: `Editor`
    /// heisst „wirkt allein mit dem Fokus im Editor“, `Ausserhalb` „wirkt nie
    /// mit dem Fokus im Editor“.
    #[must_use]
    pub const fn seite(self) -> Seite {
        match self {
            Wirkungsbereich::Editor
            | Wirkungsbereich::Editortext
            | Wirkungsbereich::Eintraege
            | Wirkungsbereich::Reihenfolge
            | Wirkungsbereich::Aufgaben
            | Wirkungsbereich::Termine
            | Wirkungsbereich::Geheimnisse
            | Wirkungsbereich::Quicknote => Seite::Editor,
            Wirkungsbereich::Dateifenster
            | Wirkungsbereich::Leiste
            | Wirkungsbereich::Tabbereich
            | Wirkungsbereich::Navigator
            | Wirkungsbereich::Vorschau
            | Wirkungsbereich::Bildfolge => Seite::Ausserhalb,
            Wirkungsbereich::Dateibereiche | Wirkungsbereich::Ueberall => Seite::Beide,
        }
    }

    /// Ob dieser Bereich und `andere` nie zugleich wirken koennen: genau
    /// dann, wenn der eine auf der Seite [`Seite::Editor`] und der andere auf
    /// [`Seite::Ausserhalb`] steht.
    ///
    /// Gleiche Seiten schliessen einander nie aus, und [`Seite::Beide`]
    /// schliesst nichts aus. Die Regel ist damit symmetrisch.
    #[must_use]
    pub const fn schliesst_aus(self, andere: Wirkungsbereich) -> bool {
        matches!(
            (self.seite(), andere.seite()),
            (Seite::Editor, Seite::Ausserhalb) | (Seite::Ausserhalb, Seite::Editor)
        )
    }

    /// Der Bereich, den dieser verengt, oder keiner: wo dieser zulaessig ist,
    /// ist jener es auch.
    ///
    /// **Vollstaendige Fallunterscheidung ohne Auffangzweig**: ein weiterer
    /// Wert haelt den Bau hier an und wird bewusst eingeordnet. Daraus folgt
    /// ohne Zutat, dass jeder Bereich hoechstens einen verengt. Zwei
    /// Kernproben halten, dass kein Bereich sich selbst verengt und dass eine
    /// Verengung auf derselben [`Seite`] bleibt; damit sind Ausschluss und
    /// Verengung disjunkt. Eine Verengung ueber mehrere Stufen gibt es bewusst
    /// nicht: `Bildfolge` liegt der Sache nach auch in `Navigator`, und ein
    /// Teilen mit dessen Befehlen bleibt ein Konflikt, die sichere Richtung
    /// des Fehlers.
    #[must_use]
    pub const fn weiter(self) -> Option<Wirkungsbereich> {
        match self {
            Wirkungsbereich::Bildfolge => Some(Wirkungsbereich::Dateifenster),
            Wirkungsbereich::Dateifenster
            | Wirkungsbereich::Leiste
            | Wirkungsbereich::Dateibereiche
            | Wirkungsbereich::Editor
            | Wirkungsbereich::Editortext
            | Wirkungsbereich::Eintraege
            | Wirkungsbereich::Reihenfolge
            | Wirkungsbereich::Aufgaben
            | Wirkungsbereich::Termine
            | Wirkungsbereich::Geheimnisse
            | Wirkungsbereich::Quicknote
            | Wirkungsbereich::Tabbereich
            | Wirkungsbereich::Navigator
            | Wirkungsbereich::Vorschau
            | Wirkungsbereich::Ueberall => None,
        }
    }

    /// Ob dieser Bereich `andere` verengt, also `self.weiter()` genau
    /// `andere` ist.
    ///
    /// Anders als [`Wirkungsbereich::schliesst_aus`] nicht symmetrisch: die
    /// Richtung sagt, welche von zwei Funktionen vorgeht.
    #[must_use]
    pub const fn verengt(self, andere: Wirkungsbereich) -> bool {
        match self.weiter() {
            // Verglichen ueber die Stelle, weil eine `const fn` `PartialEq`
            // nicht rufen kann; die Aufzaehlung traegt keine Daten.
            Some(weiter) => weiter as u8 == andere as u8,
            None => false,
        }
    }
}

/// Auf welcher Seite des Editors ein [`Wirkungsbereich`] wirkt.
///
/// Die grobe Einteilung, an der die Belegung entscheidet, ob zwei Funktionen
/// eines Zustellers eine Kombination teilen duerfen; sie ist disjunkt und
/// vollstaendig, weil jeder Bereich in [`Wirkungsbereich::seite`] in genau
/// einem Zweig steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Seite {
    /// Wirkt allein mit dem Fokus im Editor.
    Editor,
    /// Wirkt nie mit dem Fokus im Editor.
    Ausserhalb,
    /// Wirkt im Editor und ausserhalb.
    Beide,
}

/// Was ein Tastendruck im Dateifenster ausloest.
///
/// **Nicht der Wortschatz der Belegung.** Die Belegung kennt jede Funktion aus
/// C1 bis C7; diese Aufzaehlung kennt die, zu denen es in dieser Runde schon
/// eine Ausfuehrung gibt. Sie waechst mit den Schritten, die die uebrigen
/// Funktionen bauen. Die Bruecke zwischen beiden sind die Kennungen aus
/// `resources/default-keymap.toml`, und eine Pruefung haelt sie zusammen:
/// `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kommando {
    /// Die Auswahl einen Eintrag nach oben.
    AuswahlHoch,
    /// Die Auswahl einen Eintrag nach unten.
    AuswahlRunter,
    /// Die Auswahl eine Bildschirmseite nach oben.
    SeiteHoch,
    /// Die Auswahl eine Bildschirmseite nach unten.
    SeiteRunter,
    /// Die Auswahl an den Anfang der Liste (C2).
    Listenanfang,
    /// Die Auswahl an das Ende der Liste (C2).
    Listenende,
    /// In den ausgewaehlten Ordner hineinsteigen.
    Oeffnen,
    /// In den uebergeordneten Ordner aufsteigen (C2).
    OrdnerAufwaerts,
    /// Den Ordner der angezeigten Datei im aktiven Dateifenster zeigen, mit
    /// der Auswahl auf dieser Datei (C2 der Runde 6).
    ///
    /// "Die angezeigte Datei" ist die der sichtbaren Vorschau, sonst die des
    /// sichtbaren Editors; die Rechnung dazu steht in `krk-ui` unter
    /// `angezeigtedatei::welche` und nicht hier, weil sie die Sichtbarkeit von
    /// Bereichen braucht, die der Kern nicht kennt.
    OrdnerDerDatei,
    /// Das andere Dateifenster auf den Ordner stellen, den das aktive zeigt
    /// (C1 der Runde 13).
    ///
    /// **Die Richtung ist eine und nicht zwei:** die Quelle ist immer das
    /// aktive Dateifenster, das Ziel immer das andere. Ein Befehl, der die
    /// Richtung aus dem Fokus ablaese und dabei beide Seiten als Quelle
    /// zuliesse, waere derselbe Befehl mit einem verborgenen Schalter.
    ///
    /// Was beim Ausfuehren geschieht — das andere Dateifenster gegebenenfalls
    /// einblenden, die beiden angezeigten Ordner vergleichen und nur bei
    /// Ungleichheit lesen —, steht in `krk-ui` unter
    /// `Anwendungsdelegierter::ordner_angleichen` und nicht hier: es braucht
    /// die Sichtbarkeit von Bereichen, die der Kern nicht kennt.
    OrdnerAngleichen,
    /// Einen Pfad eingeben und dorthin springen (C2).
    Pfadeingabe,
    /// Den Eintrag unter der Auswahl markieren und weiterruecken (C2).
    MarkierungUmschalten,
    /// Alle Eintraege markieren (C2).
    AlleMarkieren,
    /// Jede Markierung aufheben (C2).
    MarkierungAufheben,
    /// Die Markierung umkehren (C2).
    MarkierungUmkehren,
    /// Nach Name sortieren (C2).
    SortierungName,
    /// Nach Groesse sortieren (C2).
    SortierungGroesse,
    /// Nach Aenderungsdatum sortieren (C2).
    SortierungDatum,
    /// Nach Typ sortieren (C2).
    SortierungTyp,
    /// Die Sortierrichtung umkehren (C2).
    SortierrichtungUmkehren,
    /// Versteckte Dateien ein- und ausblenden (C2).
    VersteckteUmschalten,
    /// Die Spalte Groesse in beiden Dateilisten ein- und ausblenden (C3 der
    /// Bereichsleisten-Runde).
    ///
    /// **Die Spaltenschalter treffen beide Dateilisten zugleich** und
    /// setzen deshalb kein Dateifenster im Fokus voraus; Nutzerentscheid vom
    /// 260812-0306. Ab Werk tragen sie keine Kombination, ebenfalls nach
    /// Entscheid vom 260812-0306: die freien Kombinationen sind knapp, und eine
    /// Spaltensichtbarkeit ist eine Einstellung, die man einmal trifft. Wer eine
    /// Taste dafuer will, weist sie in der Belegungsansicht zu oder traegt sie
    /// in `resources/default-keymap.toml` ein.
    ///
    /// **Die Spalte Name hat kein Gegenstueck hier**, und das ist Absicht: eine
    /// Dateiliste ohne sie zeigt nichts, was den Eintrag benennt.
    SpalteGroesseUmschalten,
    /// Die Spalte Aenderungsdatum in beiden Dateilisten ein- und ausblenden
    /// (C3 der Bereichsleisten-Runde).
    ///
    /// Der Befehl heisst nach dem Namen, den der Nutzer dem Schalter gegeben
    /// hat; die Spaltenueberschrift lautet weiterhin "Änderungsdatum". Alles
    /// Weitere steht an [`Kommando::SpalteGroesseUmschalten`].
    SpalteDatumUmschalten,
    /// Die Spalte Typ in beiden Dateilisten ein- und ausblenden (C3 der
    /// Bereichsleisten-Runde).
    ///
    /// Siehe [`Kommando::SpalteGroesseUmschalten`].
    SpalteTypUmschalten,
    /// Den stehenden Filter der Dateiliste auf den Unterbaum ausdehnen und
    /// wieder einholen (C5 der Filter-Runde).
    ///
    /// **Die Kennung ist deutsch, die Aufschrift des Kaestchens nicht.** Der
    /// Schalter in der Bereichsleiste traegt "Deep", weil der Nutzer den Namen
    /// so gewaehlt hat; die Kennung folgt der Schreibweise der 77 vorhandenen,
    /// die durchweg deutsch und mit Unterstrichen sind. Eine Aufschrift ist
    /// eine Entscheidung ueber die Anzeige und keine ueber den Wortschatz der
    /// Belegung.
    ///
    /// **Steht kein Filtertext, kippt der Befehl den Schalter und meldet
    /// nichts.** Ueber die Zulaessigkeit entscheidet der Wirkungsbereich und
    /// nicht, ob der Befehl etwas findet; ein Befehl, der von seinem eigenen
    /// Ergebnis abhinge, waere die zweite Regel neben dem Wirkungsbereich.
    ///
    /// **Er wirkt seit dem 260908 erst ab drei Zeichen**, derselben
    /// `krk_core::verzeichnis::filter::ZEICHENSCHWELLE`, die auch "Content"
    /// haelt: unter drei Zeichen filtert KRK flach und allein ueber die Namen,
    /// ab drei greifen Unterbaum und Inhalt. Bis dahin stiess schon der erste
    /// Anschlag den Durchlauf ueber den Unterbaum an.
    ///
    /// Ab Werk traegt er keine Kombination, wie die drei Spaltenschalter
    /// darueber; Nutzerentscheid vom 260814-1610. Wer eine Taste dafuer will,
    /// weist sie in der Belegungsansicht zu oder traegt sie in
    /// `resources/default-keymap.toml` ein.
    TiefeSucheUmschalten,
    /// Den stehenden Filter der Dateiliste auch auf den Inhalt der Dateien
    /// anwenden und wieder zuruecknehmen (C2 der Inhaltsfilter-Runde).
    ///
    /// **Der zweite Schalter derselben Art**, neben
    /// [`Kommando::TiefeSucheUmschalten`] darueber, und er folgt ihm in jedem
    /// Stueck: die Kennung ist deutsch, die Aufschrift des Kaestchens lautet
    /// "Content", der Wirkungsbereich ist `Ueberall`, und ab Werk traegt er
    /// keine Kombination. Die Wahl der Aufschrift ist eine Entscheidung ueber
    /// die Anzeige und keine ueber den Wortschatz der Belegung.
    ///
    /// **Er wirkt erst ab einer Mindestlaenge des Filtertexts**, drei Zeichen;
    /// die Regel steht als `krk_core::verzeichnis::filter::ZEICHENSCHWELLE` an
    /// einer Stelle und gilt seit dem 260908 fuer "Deep" ebenso. Der Befehl
    /// fragt sie nicht: er kippt das Kennzeichen, und ob das Kennzeichen gerade
    /// etwas bewirkt, ist eine andere Frage als die, ob der Befehl zulaessig
    /// war. Damit steht er neben "Deep", das dieselbe Trennung zieht.
    ///
    /// Ab Werk ohne Kombination, wie "Deep" und die drei Spaltenschalter
    /// darueber; die Nutzerantwort vom 260814-1610 hat das fuer den ersten
    /// Schalter dieser Art entschieden, und ein zweiter derselben Art folgt
    /// derselben Form, statt eine der frei gehaltenen Kombinationen zu belegen.
    /// Wer eine Taste dafuer will, weist sie in der Belegungsansicht zu oder
    /// traegt sie in `resources/default-keymap.toml` ein.
    InhaltssucheUmschalten,
    /// Zu dem springen, was in der Zwischenablage steht (C10).
    ZwischenablageSpringen,
    /// Den Inhalt der Zwischenablage im Vorschaufenster ansehen (C10).
    ZwischenablageAnsehen,
    /// Einen neuen Tab im aktiven Dateifenster oeffnen (C1).
    TabNeu,
    /// Den aktiven Tab schliessen (C1).
    TabSchliessen,
    /// Zum naechsten Tab wechseln (C1).
    TabNaechster,
    /// Zum vorigen Tab wechseln (C1).
    TabVoriger,
    /// Das aktive Dateifenster wechseln (C1).
    FensterWechseln,
    /// Die Lesezeichen- und Geraeteleiste ein- und ausblenden (C7).
    LeisteUmschalten,
    /// Das erste, linke Dateifenster ein- und ausblenden (C5 der
    /// Bereichsleisten-Runde).
    ///
    /// Bis zu jener Runde war das linke Dateifenster der eine Bereich, der
    /// immer stand; seither unterscheidet es sich von den anderen nicht mehr,
    /// und die Regel heisst "eines der beiden Dateifenster bleibt". Sie steht
    /// im Fenstermodell und nicht hier.
    ErstesFensterUmschalten,
    /// Das zweite Dateifenster ein- und ausblenden (C7).
    ZweitesFensterUmschalten,
    /// Das Vorschaufenster ein- und ausblenden (C7, dieselbe Funktion wie
    /// "Vorschau anzeigen" aus C3).
    VorschauUmschalten,
    /// Das geschlossene Anwendungsfenster wieder nach vorn holen (C7).
    FensterEinblenden,
    /// Das Anwendungsfenster schliessen (C7).
    FensterSchliessen,
    /// Den aktiven Bereich um einen Schritt verbreitern (C7).
    BereichVerbreitern,
    /// Den aktiven Bereich um einen Schritt verschmaelern (C7).
    BereichVerschmaelern,
    /// Die Auswahl in den Ordner des anderen Dateifensters kopieren (C4).
    Kopieren,
    /// Die Auswahl in den Ordner des anderen Dateifensters verschieben (C4).
    Verschieben,
    /// Die Auswahl in den Papierkorb des Systems raeumen (C4, Taste Delete).
    InPapierkorb,
    /// Eine laufende Dateioperation abbrechen (C4).
    Abbrechen,
    /// Einen Ordner im Ordner des aktiven Fensters anlegen (C4).
    OrdnerAnlegen,
    /// Eine leere Datei im Ordner des aktiven Fensters anlegen (C4).
    DateiAnlegen,
    /// Die markierten Eintraege im Stapel umbenennen (C4).
    UmbenennenStapel,
    /// Den ausgewaehlten Eintrag direkt in der Liste umbenennen (C4).
    Umbenennen,
    /// Den angezeigten Ordner in der eingestellten Terminal-Anwendung
    /// oeffnen (C11).
    TerminalOeffnen,
    /// Den Pfad des angezeigten Ordners in die Zwischenablage legen (C1 der
    /// Runde 4).
    OrdnerpfadKopieren,
    /// Die Pfade der betroffenen Eintraege in die Zwischenablage legen (C2 der
    /// Runde 4).
    ///
    /// "Betroffen" ist dieselbe Menge, auf der die Dateioperationen aus C4
    /// arbeiten: die Markierung, falls es eine gibt, sonst der ausgewaehlte
    /// Eintrag. Der Befehl hat damit keine eigene Auswahlregel.
    EintragspfadKopieren,
    /// Die betroffenen Eintraege an das Standardprogramm des Systems
    /// uebergeben (C3 der Runde 4).
    MitStandardprogrammOeffnen,
    /// Die betroffenen Eintraege oder die angezeigte Datei an die
    /// Freigabedienste des Systems uebergeben (C1 der Runde 6).
    ///
    /// **Worauf der Befehl wirkt, entscheidet der Fokus**, und zwar in drei
    /// Antworten ueber fuenf Werte: in einem Dateifenster die betroffenen
    /// Eintraege, in Vorschau und Editor die angezeigte Datei, in der Leiste
    /// nichts. Die Verzweigung steht in `krk-ui` unter `appkit::teilen::worauf`
    /// und nicht hier, weil sie die Bereiche der Oberflaeche braucht, die der
    /// Kern nicht kennt.
    Teilen,
    /// Den Ordner des aktiven Dateifensters als Lesezeichen anlegen (C5).
    LesezeichenAnlegen,
    /// Das ausgewaehlte Lesezeichen umbenennen (C5).
    LesezeichenUmbenennen,
    /// Das ausgewaehlte Lesezeichen loeschen (C5).
    LesezeichenLoeschen,
    /// Das ausgewaehlte Lesezeichen einen Platz nach oben schieben (C5).
    LesezeichenHoch,
    /// Das ausgewaehlte Lesezeichen einen Platz nach unten schieben (C5).
    LesezeichenRunter,
    /// Den Eingabefokus in die Lesezeichen- und Geraeteleiste setzen (C5).
    FokusLeiste,
    /// Den Eingabefokus zurueck in das aktive Dateifenster setzen (C5).
    FokusDateifenster,
    /// Den Eingabefokus in das Vorschaufenster setzen (C2, C6).
    ///
    /// Das dritte Stueck des Fokuswechsels, und ohne es waeren die vier
    /// Tabbefehle aus C1 in den Vorschau-Tabs allein per Maus erreichbar —
    /// eine Spannung zu C2, das jede Funktion ueber mindestens einen
    /// Tastenbefehl verlangt (Nutzerentscheid vom 260807,
    /// `decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md`).
    ///
    /// Die Taste ist `shift+cmd+y`, und der Buchstabe ist nicht frei gewaehlt:
    /// die Vorschau traegt in dieser Belegung schon das `y` (`cmd+y` blendet
    /// sie ein und aus), und der Fokusbefehl erbt ihn, wie `l` und `d` es fuer
    /// die Leiste und das Dateifenster tun. Wo die Kombination steht, sagt
    /// allein `resources/default-keymap.toml`; hier steht sie als Begruendung
    /// und nicht als zweite Wahrheit.
    FokusVorschau,
    /// Den ausgewaehlten Eintrag des Dateifensters im eingebauten Editor
    /// oeffnen (F4, die Norton-Bedeutung "Bearbeiten").
    ///
    /// Der erste der beiden Einstiegswege in den Editor. Der Eintrag stand seit
    /// der Runde 1 als `reserviert_fuer = "editor"` in der
    /// Auslieferungsbelegung und traegt seit der Editor-Runde die Taste und
    /// dieses Kommando.
    ///
    /// **Seit dem 260823 fuehrt eine zweite Taste auf denselben Rumpf**, naemlich
    /// `cmd+e` mit dem Fokus in der Dateiliste ([`Kommando::EditorRundweg`]).
    /// Der Weg ist derselbe und nicht eine Kopie daneben; dieses Kommando ist
    /// unveraendert und bleibt an `f4`.
    Bearbeiten,
    /// Der Rundweg in den Editor und zurueck (C2 der Editor-Runde,
    /// Nutzerentscheid vom 260823-0942).
    ///
    /// **Ein Befehl mit drei fokusabhaengigen Bedeutungen**, und er ist der
    /// einzige Traeger von [`Wirkungsbereich::Dateibereiche`]:
    ///
    /// | Fokus | was der Befehl tut |
    /// |---|---|
    /// | Dateifenster | den ausgewaehlten Eintrag im Editor oeffnen, wie F4 |
    /// | Vorschau | die angezeigte Datei im Editor oeffnen |
    /// | Editor | den Editor schliessen und in die Dateiliste zurueckgehen |
    ///
    /// Die Fallunterscheidung selbst steht nicht hier, sondern als reine
    /// Funktion in `krk_ui::kommandos::rundweg`: sie fragt nach dem Fokus, und
    /// der Kern kennt keinen. Hier steht allein, dass die drei Bereiche den
    /// Befehl durchlassen.
    ///
    /// **Bis zum 260823 hiess er `EditorAusVorschau`** und trug allein die
    /// mittlere Zeile der Tafel. Die Vorschau-Richtung ist unveraendert; die
    /// erste und die dritte Zeile sind hinzugekommen, weil der Fokus nach `f3`
    /// in der Dateiliste bleibt und ein Umschalter, der die Vorschau als
    /// Ausgangspunkt naehme, den haeufigen Fall verfehlte.
    EditorRundweg,
    /// Den Eingabefokus in den eingebauten Editor setzen (C1 der
    /// Editor-Runde).
    ///
    /// Der vierte Fokusbefehl. Er holt einen ausgeblendeten Editor hervor,
    /// sofern dieser eine Datei haelt; die Bedingung steht beim Aufrufer, die
    /// Zuordnung von einem Fokusziel auf einen Bereich in `krk_ui`.
    FokusEditor,
    /// Den Editor schliessen: die Datei freigeben und ihn ausblenden (C1 der
    /// Editor-Runde).
    ///
    /// **Nicht dasselbe wie [`Kommando::EditorUmschalten`] darunter**, und die
    /// beiden bestehen nebeneinander: dieser Befehl gibt die gehaltene Datei
    /// auf und loest damit die Nachfrage nach einem ungesicherten Stand aus
    /// (C4 der Editor-Runde). Er traegt deshalb
    /// [`Wirkungsbereich::Editor`] — ohne Fokus im Text gibt es keine Datei,
    /// die er aufgaebe.
    EditorSchliessen,
    /// Den Editor ein- und ausblenden, ohne seine Datei anzufassen (C6 der
    /// Bereichsleisten-Runde).
    ///
    /// **Nicht dasselbe wie [`Kommando::EditorSchliessen`] darueber.** Dieser
    /// Befehl behaelt die Datei samt Stand und fragt nichts nach; er blendet
    /// allein die Flaeche aus und wieder ein, wie es
    /// [`Kommando::VorschauUmschalten`] fuer die Vorschau tut. Er traegt
    /// deshalb [`Wirkungsbereich::Ueberall`] und nicht
    /// [`Wirkungsbereich::Editor`]: ein Schalter in der Bereichsleiste muss
    /// aus jedem Fokus wirken.
    ///
    /// Haelt der Editor keine Datei und ist er ausgeblendet, geschieht nichts.
    /// Die Bedingung steht beim Aufrufer in `krk_ui`, wie die gleichlautende
    /// an [`Kommando::FokusEditor`]: das Fenstermodell weiss von Dateien
    /// nichts.
    EditorUmschalten,
    /// Zwischen Rohansicht und Formatansicht wechseln (C3 der Editor-Runde).
    EditorAnsichtUmschalten,
    /// Die im Editor geoeffnete Datei sichern (C4 der Editor-Runde).
    EditorSichern,
    /// Die Schreibmarke auf eine eingegebene Zeilennummer setzen (C5 der
    /// Editor-Runde).
    EditorZeileSpringen,
    /// Im Text des Editors suchen (C5 der Editor-Runde).
    EditorSuchen,
    /// Den naechsten Treffer der laufenden Suche anspringen (C5 der
    /// Editor-Runde).
    EditorWeitersuchen,
    /// Den vorigen Treffer der laufenden Suche anspringen (C5 der
    /// Editor-Runde).
    EditorRueckwaertsSuchen,
    /// Den naechsten Treffer ersetzen (C5 der Editor-Runde).
    EditorErsetzen,
    /// Jeden Treffer im ganzen Text ersetzen (C5 der Editor-Runde).
    EditorAlleErsetzen,
    /// Die Quicknote oeffnen, den Fokus in sie holen oder sie schliessen (Q1
    /// des Spec `260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`).
    ///
    /// Traegt [`Wirkungsbereich::Ueberall`]: F10 holt die Quicknote aus jedem
    /// Bereich, und ein Vorbehalt verlangte den Zustand, den der Befehl erst
    /// herstellt. Welche der drei Wirkungen ein Anschlag hat, entscheidet
    /// `krk_ui` aus der Lage.
    QuicknoteUmschalten,
    /// Den ganzen Text der Quicknote in die Zwischenablage legen, den Puffer
    /// leeren und die Quicknote schliessen (Q2).
    ///
    /// Traegt [`Wirkungsbereich::Quicknote`]: ohne offene Quicknote gibt es
    /// keinen Puffer, den er kopierte.
    QuicknoteKopieren,
    /// Den Puffer der Quicknote leeren, mit `cmd+z` zuruecknehmbar (Q2).
    ///
    /// Traegt [`Wirkungsbereich::Quicknote`] wie das Kopieren und ist ab Werk
    /// unbelegt; erreichbar ist er ueber die Schaltflaeche und das Menue.
    QuicknoteLeeren,
    /// Am Ende der Eintragstabelle einen leeren Eintrag anlegen und seine
    /// Zelle in Bearbeitung setzen (C6 des Spec
    /// `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
    ///
    /// Die fuenf `Eintrag*`-Befehle tragen [`Wirkungsbereich::Eintraege`]: sie
    /// wirken allein mit dem Fokus im Editor und nur, solange er eine
    /// Eintragstabelle zeigt. Welche Form das ist, fragt `krk_ui` in seiner
    /// Zulaessigkeitsregel; der Kern kennt den Bereich und nicht die Form.
    EintragHinzufuegen,
    /// Die Zelle des gewaehlten Eintrags in Bearbeitung setzen, oder eine
    /// laufende Zelle uebernehmen (C6).
    EintragBearbeiten,
    /// Den gewaehlten Eintrag eine Stelle nach oben ruecken (C6).
    EintragHoch,
    /// Den gewaehlten Eintrag eine Stelle nach unten ruecken (C6).
    EintragRunter,
    /// Den gewaehlten Eintrag entfernen; mit `cmd+z` zuruecknehmbar (C6).
    EintragLoeschen,
    /// Die gewaehlte Aufgabe abhaken oder wieder oeffnen (C6).
    ///
    /// Traegt [`Wirkungsbereich::Aufgaben`] und nicht
    /// [`Wirkungsbereich::Eintraege`]: eine Notiz hat kein Kaestchen, und in
    /// der Notiztabelle soll der Befehl ausgegraut sein.
    AufgabeAbhaken,
    /// Die PIN der offenen `secrets.txt` aendern: alte PIN, neue PIN
    /// zweimal, dann verschluesselt KRK den Stand auf der Platte mit einem
    /// neuen Schluessel aus frischem Salz (C7 des Spec, Schritt 5.5 der
    /// krkhome-Arbeit).
    ///
    /// Traegt [`Wirkungsbereich::Geheimnisse`]: er wirkt allein mit dem Fokus
    /// im Editor und nur, solange der Editor `secrets.txt` haelt und ihre
    /// PIN schon in einem Kopf auf der Platte steht.
    PinAendern,
    /// Die Richtung der Termintabelle umkehren: aufsteigend nach dem Datum
    /// oder absteigend (T5 des Spec
    /// `260926-2253_*_spec-termine-als-weitere-datei-im-heimordner.md`).
    ///
    /// Traegt [`Wirkungsbereich::Termine`] und ist ausgeliefert auf `cmd+1`,
    /// derselben Kombination wie das Sortieren nach Name: die beiden Bereiche
    /// schliessen einander aus, und welche der zwei Funktionen ein Anschlag
    /// meint, entscheidet die Lage. Die Richtung ist eine Angabe der Anzeige;
    /// die Datei aendert der Befehl nicht, und die Sitzung merkt sich die
    /// Richtung.
    TermineRichtungUmkehren,
    /// Die Belegungsansicht zeigen: jede Funktion mit ihren Kombinationen,
    /// aenderbar und zuruecksetzbar (C3).
    BelegungAnsehen,
    /// Die Belegungsdatei des Nutzers im Vorschaufenster zeigen
    /// (Nutzerauftrag vom 260901).
    ///
    /// **Nicht zu verwechseln mit [`Kommando::BelegungAnsehen`] daneben.** Jenes
    /// zeigt die geltende Belegung als aenderbare Tabelle in einem Blatt; dieses
    /// zeigt die Datei, aus der sie beim Start gelesen wurde,
    /// `~/Library/Application Support/KRK/keymap.toml`, als Text in der
    /// Vorschau. Der Weg von dort in den Editor ist `cmd+e`, der Rundweg, und
    /// deshalb holt der Befehl den Fokus in die Vorschau.
    ///
    /// **Die eingebaute Auslieferungsbelegung meint er nicht.**
    /// `resources/default-keymap.toml` ist zur Bauzeit einkompiliert und liegt
    /// im ausgelieferten Buendel nicht als Datei; ein Befehl, der sie zeigte,
    /// haette am Referenzgeraet nichts zu oeffnen.
    ///
    /// **Ab Werk ohne Kombination**, wie die vier Spaltenschalter und die zwei
    /// Filterschalter: erreichbar ueber das Hauptmenue, und wer eine Taste will,
    /// vergibt sie in der Belegungsansicht.
    BelegungsdateiAnsehen,
    /// Die Anwendung beenden (C3).
    Beenden,
    /// Eine weitere, eigenstaendige Instanz von KRK starten (C3 der Runde 7).
    WeitereInstanz,
    /// Das aktive Dateifenster auf den Heimordner `~/krkhome/` fuehren und
    /// Ordner und Eintragsdateien dabei anlegen, wo sie fehlen (C1 und C2 des
    /// Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
    ///
    /// Steht im aktiven Dateifenster schon ein Tab auf den Ordner, wird er
    /// sichtbar; sonst entsteht ein neuer. Ein Blatt oeffnet sich nie. Bis zu
    /// dieser Arbeit oeffnete derselbe Befehl das Notizblatt der Runde 9; seine
    /// Kennung `notizzettel` ist aelter als er, und warum sie bleibt, steht bei
    /// ihrer Zeile in [`Kommando::KENNUNGEN`].
    ///
    /// Ausgeliefert auf zwei Kombinationen, `f2` und `cmd+k`, in **einer**
    /// Zeile der Belegung; die Begruendung steht bei ihrem Eintrag in
    /// `resources/default-keymap.toml`.
    Notizordner,
    /// Den Ort des Notizordners ueber einen Ordnerdialog waehlen und in
    /// `settings.toml` schreiben (H3 des Spec
    /// `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`).
    ///
    /// Der Dialog beginnt beim geltenden Ort und erlaubt, einen Ordner
    /// anzulegen. Der gewaehlte Ort gilt sofort; verschoben und angelegt wird
    /// nichts, auch kein Tab, und F2 fuehrt danach dorthin. Haelt der Editor
    /// eine Datei des geltenden oder des gewaehlten Orts, verweigert sich der
    /// Befehl und nennt sie.
    ///
    /// **Ab Werk ohne Kombination**: ein seltener Einrichtungsbefehl, erreichbar
    /// ueber das Menue „Home“ und in F1 belegbar. **Wirkt ueberall**, wie
    /// [`Kommando::Notizordner`] daneben: sein Dialog faehrt am Hauptfenster
    /// herunter, und sein Gegenstand ist der Notizordner, nicht einer der
    /// Bereiche der Fensterzeile.
    OrtWaehlen,
    /// Die Seite im PDF-Betrachter des Vorschaufensters um eine Stufe
    /// vergroessern (C3 der Runde 20).
    ///
    /// Schrittweite, Untergrenze und Obergrenze des Zooms setzt der Betrachter
    /// (A2); an der Obergrenze aendert ein weiterer Anschlag nichts und meldet
    /// nichts. Mit dem Fokus in der Vorschau, aber ohne angezeigtes PDF, wird
    /// der Befehl entgegengenommen und tut nichts (A6): die Zulaessigkeit
    /// haengt am Fokus, nicht am Inhalt.
    VorschauVergroessern,
    /// Die Seite im PDF-Betrachter des Vorschaufensters um eine Stufe
    /// verkleinern (C3 der Runde 20).
    ///
    /// Dieselben Regeln wie bei [`Kommando::VorschauVergroessern`], an der
    /// Untergrenze (A2, A6).
    VorschauVerkleinern,
    /// Die Seite im PDF-Betrachter auf die Ausgangsgroesse stellen (C3 der
    /// Runde 20).
    ///
    /// Die Ausgangsgroesse passt die Seitenbreite in die Breite des
    /// Vorschaufensters ein und folgt dessen Breite, solange der Nutzer nicht
    /// gezoomt hat (A1). Ohne angezeigtes PDF wird der Befehl entgegengenommen
    /// und tut nichts (A6).
    VorschauAusgangsgroesse,
    /// Den Git-Bereich ein- und ausblenden (C1 der Runde 23).
    ///
    /// Der sechste Bereichsumschalter, und er traegt
    /// [`Wirkungsbereich::Ueberall`] aus derselben Erwaegung wie die fuenf
    /// vorhandenen: ein Umschalter braucht seinen Bereich nicht, er stellt ihn
    /// her, und sein Schalter in der Bereichsleiste muss aus jedem Fokus
    /// wirken.
    ///
    /// Passt die Mindestbreite des Git-Bereichs nicht in das Fenster, weist
    /// das Fenstermodell das Einblenden ab und der Befehl tut nichts; das ist
    /// dieselbe Abweisung, die die fuenf anderen Umschalter kennen, und sie
    /// steht dort und nicht hier.
    GitBereichUmschalten,
    /// Den Eingabefokus in den Git-Bereich setzen (C2 der Runde 23).
    ///
    /// Der fuenfte Fokusbefehl. Er holt einen ausgeblendeten Git-Bereich
    /// hervor, wie es die vier vorhandenen fuer ihren Bereich tun, und traegt
    /// aus demselben Grund wie sie [`Wirkungsbereich::Ueberall`]: ein Befehl,
    /// der den Fokus **holt**, kann nicht voraussetzen, wo er gerade steht.
    ///
    /// **Der Buchstabe wird nicht vom Umschalter geerbt**, anders als bei
    /// Leiste, Dateifenster und Vorschau. Wo die Kombination steht, sagt
    /// allein `resources/default-keymap.toml`; hier stuende sie als zweite
    /// Wahrheit.
    FokusGit,
    /// Die Spalte Marke in beiden Dateilisten ein- und ausblenden (C5 der
    /// Runde 23).
    ///
    /// Der vierte Spaltenschalter. Alles Weitere steht an
    /// [`Kommando::SpalteGroesseUmschalten`]: er trifft **beide** Dateilisten
    /// zugleich, setzt deshalb kein Dateifenster im Fokus voraus und traegt ab
    /// Werk keine Kombination.
    SpalteMarkeUmschalten,
    /// Das Blatt zeigen, das die Neuerungen an `readers.toml`,
    /// `settings.toml` und `keymap.toml` im Einzelnen auffuehrt (Runde der
    /// Ablageneuerungen).
    ///
    /// Beim Start meldet KRK von sich aus, dass eine der drei Dateien hinter
    /// der Auslieferungsfassung zurueckliegt, einmal je Fassung. Dieser Befehl
    /// ist der Weg zum Einzelnen auf Verlangen, und was er zeigt, haengt daran,
    /// ob der Start erhoben hat: hat er es, zeigt jeder Abruf denselben Stand
    /// vom Start; sonst — der haeufigste Fall, denn er tritt bei jedem zweiten
    /// Start derselben Fassung ein — traegt der Befehl die Erhebung auf
    /// Verlangen nach und zeigt, was gerade auf der Platte steht. So
    /// entschieden vom Nutzer am 260910-1600; die Begruendung steht an
    /// `Anwendungsdelegierter::neuerungen_zeigen` in `krk-ui`. Womit KRK
    /// **arbeitet**, aendert das nicht: die Leseprofile und die Belegung der
    /// laufenden Anwendung sind in beiden Faellen die vom Start oder vom
    /// letzten [`Kommando::Werkseinstellungen`].
    ///
    /// **Wirkt ueberall, wie [`Kommando::Notizordner`] daneben.** Das Blatt
    /// faehrt am Hauptfenster herunter und betrifft die Ablage, nicht einen
    /// der Bereiche der Fensterzeile.
    NeuerungenZeigen,
    /// `readers.toml`, `settings.toml` und `keymap.toml` nach einer Rueckfrage
    /// auf den Zustand eines ersten Starts zuruecksetzen, die alten Fassungen
    /// mit Zeitstempel beiseitelegen und den neuen Stand sofort einlesen (Spec
    /// `260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md`).
    ///
    /// Der Notizordner bleibt dabei, wie er ist: `settings.toml` behaelt den
    /// Wert von `notizordner`. Den Vorgang im Kern traegt
    /// `ablage::werkszustand`.
    ///
    /// **Wirkt ueberall, wie [`Kommando::NeuerungenZeigen`] daneben**, und ab
    /// Werk ohne Kombination: ein versehentlicher Anschlag naehme alle eigenen
    /// Tastenzuweisungen aus dem Betrieb.
    Werkseinstellungen,
    /// Das naechste Foto einer stehenden Bildfolge zeigen (C3 des Spec
    /// `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`).
    ///
    /// Traegt [`Wirkungsbereich::Bildfolge`]: der Fokus steht in der
    /// Dateiliste, die Wirkung in der Vorschau, und die Auswahl der Dateiliste
    /// bleibt, wo sie ist. Auf dem letzten Foto tut der Befehl nichts.
    BildVor,
    /// Das vorige Foto einer stehenden Bildfolge zeigen (C3).
    ///
    /// Teilt `cmd+up` mit [`Kommando::OrdnerAufwaerts`] nach der Verengung aus
    /// dem Modulkopf; mit stehender Bildfolge geht er vor. Auf dem ersten Foto
    /// tut er nichts.
    BildZurueck,
    /// Die Dateiliste in den Ordner des angezeigten Fotos fuehren und es dort
    /// auswaehlen (C4).
    ///
    /// Teilt `return` mit [`Kommando::MitStandardprogrammOeffnen`], wie
    /// [`Kommando::BildZurueck`] `cmd+up` teilt.
    ZumBild,
}

/// Die Aufzaehlung passt in die Umwandlung, ueber die [`Kommando::kennung`]
/// vergleicht.
///
/// **Die Schranke steht hier und nicht als stille Nebenwirkung eines
/// Umwandlungsoperators.** `kennung` ist eine `const fn` und kann `PartialEq`
/// deshalb nicht rufen; sie vergleicht ueber `self as u16`. Waechst die
/// Aufzaehlung darueber hinaus, schnitte die Umwandlung ab, zwei Kommandos
/// vergleichen sich gleich, und `kennung` lieferte die Kennung des falschen,
/// statt anzuhalten. Bis zum 260905 stand dort `as u8` und die Schranke lag bei
/// 256, ohne dass eine Stelle im Baum sie genannt haette
/// (`shared/issues/260826-1223_*_kommando-kennung-vergleicht-ueber-as-u8-und-liefert-oberhalb-von-256-varianten-still-die-falsche-kennung.md`).
///
/// Gehalten wird `KENNUNGEN` und nicht die Aufzaehlung selbst, und das ist
/// dieselbe Menge: `jede_variante_von_kommando_steht_genau_einmal_in_kennungen`
/// (`crates/krk-core/tests/belegung.rs`) haelt beide gegeneinander.
const _: () = assert!(Kommando::KENNUNGEN.len() <= u16::MAX as usize);

impl Kommando {
    /// Die Kennung, unter der die Belegungsdatei die zugehoerige Funktion
    /// fuehrt, je Kommando.
    pub const KENNUNGEN: [(Kommando, &'static str); 103] = [
        (Kommando::AuswahlHoch, "auswahl_hoch"),
        (Kommando::AuswahlRunter, "auswahl_runter"),
        (Kommando::SeiteHoch, "seite_hoch"),
        (Kommando::SeiteRunter, "seite_runter"),
        (Kommando::Listenanfang, "listenanfang"),
        (Kommando::Listenende, "listenende"),
        (Kommando::Oeffnen, "oeffnen"),
        (Kommando::OrdnerAufwaerts, "ordner_aufwaerts"),
        (Kommando::OrdnerDerDatei, "ordner_der_datei"),
        (Kommando::OrdnerAngleichen, "ordner_angleichen"),
        (Kommando::Pfadeingabe, "pfadeingabe"),
        (Kommando::MarkierungUmschalten, "markierung_umschalten"),
        (Kommando::AlleMarkieren, "alle_markieren"),
        (Kommando::MarkierungAufheben, "markierung_aufheben"),
        (Kommando::MarkierungUmkehren, "markierung_umkehren"),
        (Kommando::SortierungName, "sortierung_name"),
        (Kommando::SortierungGroesse, "sortierung_groesse"),
        (Kommando::SortierungDatum, "sortierung_datum"),
        (Kommando::SortierungTyp, "sortierung_typ"),
        (
            Kommando::SortierrichtungUmkehren,
            "sortierrichtung_umkehren",
        ),
        (Kommando::VersteckteUmschalten, "versteckte_umschalten"),
        (
            Kommando::SpalteGroesseUmschalten,
            "spalte_groesse_umschalten",
        ),
        (Kommando::SpalteDatumUmschalten, "spalte_datum_umschalten"),
        (Kommando::SpalteTypUmschalten, "spalte_typ_umschalten"),
        (Kommando::TiefeSucheUmschalten, "tiefe_suche_umschalten"),
        (Kommando::InhaltssucheUmschalten, "inhaltssuche_umschalten"),
        (Kommando::ZwischenablageSpringen, "zwischenablage_springen"),
        (Kommando::ZwischenablageAnsehen, "zwischenablage_ansehen"),
        (Kommando::TabNeu, "tab_neu"),
        (Kommando::TabSchliessen, "tab_schliessen"),
        (Kommando::TabNaechster, "tab_naechster"),
        (Kommando::TabVoriger, "tab_voriger"),
        (Kommando::FensterWechseln, "fenster_wechseln"),
        (Kommando::LeisteUmschalten, "leiste_umschalten"),
        (
            Kommando::ErstesFensterUmschalten,
            "erstes_fenster_umschalten",
        ),
        (
            Kommando::ZweitesFensterUmschalten,
            "zweites_fenster_umschalten",
        ),
        (Kommando::VorschauUmschalten, "vorschau_umschalten"),
        (Kommando::FensterEinblenden, "fenster_einblenden"),
        (Kommando::FensterSchliessen, "fenster_schliessen"),
        (Kommando::BereichVerbreitern, "bereich_verbreitern"),
        (Kommando::BereichVerschmaelern, "bereich_verschmaelern"),
        (Kommando::Kopieren, "kopieren"),
        (Kommando::Verschieben, "verschieben"),
        (Kommando::InPapierkorb, "in_papierkorb"),
        (Kommando::Abbrechen, "abbrechen"),
        (Kommando::OrdnerAnlegen, "ordner_anlegen"),
        (Kommando::DateiAnlegen, "datei_anlegen"),
        (Kommando::UmbenennenStapel, "umbenennen_stapel"),
        (Kommando::Umbenennen, "umbenennen"),
        (Kommando::TerminalOeffnen, "terminal_oeffnen"),
        (Kommando::OrdnerpfadKopieren, "ordnerpfad_kopieren"),
        (Kommando::EintragspfadKopieren, "eintragspfad_kopieren"),
        (
            Kommando::MitStandardprogrammOeffnen,
            "mit_standardprogramm_oeffnen",
        ),
        (Kommando::Teilen, "teilen"),
        (Kommando::LesezeichenAnlegen, "lesezeichen_anlegen"),
        (Kommando::LesezeichenUmbenennen, "lesezeichen_umbenennen"),
        (Kommando::LesezeichenLoeschen, "lesezeichen_loeschen"),
        (Kommando::LesezeichenHoch, "lesezeichen_hoch"),
        (Kommando::LesezeichenRunter, "lesezeichen_runter"),
        (Kommando::FokusLeiste, "fokus_leiste"),
        (Kommando::FokusDateifenster, "fokus_dateifenster"),
        (Kommando::FokusVorschau, "fokus_vorschau"),
        (Kommando::Bearbeiten, "bearbeiten"),
        (Kommando::EditorRundweg, "editor_rundweg"),
        (Kommando::FokusEditor, "fokus_editor"),
        (Kommando::EditorSchliessen, "editor_schliessen"),
        (Kommando::EditorUmschalten, "editor_umschalten"),
        (
            Kommando::EditorAnsichtUmschalten,
            "editor_ansicht_umschalten",
        ),
        (Kommando::EditorSichern, "editor_sichern"),
        (Kommando::EditorZeileSpringen, "editor_zeile_springen"),
        (Kommando::EditorSuchen, "editor_suchen"),
        (Kommando::EditorWeitersuchen, "editor_weitersuchen"),
        (
            Kommando::EditorRueckwaertsSuchen,
            "editor_rueckwaerts_suchen",
        ),
        (Kommando::EditorErsetzen, "editor_ersetzen"),
        (Kommando::EditorAlleErsetzen, "editor_alle_ersetzen"),
        (Kommando::QuicknoteUmschalten, "quicknote_umschalten"),
        (Kommando::QuicknoteKopieren, "quicknote_kopieren"),
        (Kommando::QuicknoteLeeren, "quicknote_leeren"),
        (Kommando::EintragHinzufuegen, "eintrag_hinzufuegen"),
        (Kommando::EintragBearbeiten, "eintrag_bearbeiten"),
        (Kommando::EintragHoch, "eintrag_hoch"),
        (Kommando::EintragRunter, "eintrag_runter"),
        (Kommando::EintragLoeschen, "eintrag_loeschen"),
        (Kommando::AufgabeAbhaken, "aufgabe_abhaken"),
        (Kommando::PinAendern, "pin_aendern"),
        (
            Kommando::TermineRichtungUmkehren,
            "termine_richtung_umkehren",
        ),
        (Kommando::BelegungAnsehen, "belegung_ansehen"),
        (Kommando::BelegungsdateiAnsehen, "belegungsdatei_ansehen"),
        (Kommando::Beenden, "beenden"),
        (Kommando::WeitereInstanz, "weitere_instanz"),
        // Die Kennung ist aelter als der Befehl: sie stammt aus der Runde 9,
        // als F2 das Notizblatt oeffnete, und bleibt, damit eine vollstaendige
        // eigene Belegung weiter laedt. `Belegung::bauen` weist eine unbekannte
        // Kennung ab und setzt dann die ganze Auslieferungsbelegung ein (C1 des
        // Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
        (Kommando::Notizordner, "notizzettel"),
        (Kommando::OrtWaehlen, "ort_waehlen"),
        (Kommando::VorschauVergroessern, "vorschau_vergroessern"),
        (Kommando::VorschauVerkleinern, "vorschau_verkleinern"),
        (
            Kommando::VorschauAusgangsgroesse,
            "vorschau_ausgangsgroesse",
        ),
        (Kommando::GitBereichUmschalten, "git_bereich_umschalten"),
        (Kommando::FokusGit, "fokus_git"),
        (Kommando::SpalteMarkeUmschalten, "spalte_marke_umschalten"),
        (Kommando::NeuerungenZeigen, "neuerungen_zeigen"),
        (Kommando::Werkseinstellungen, "werkseinstellungen"),
        (Kommando::BildVor, "bild_vor"),
        (Kommando::BildZurueck, "bild_zurueck"),
        (Kommando::ZumBild, "zum_bild"),
    ];

    /// Das Kommando zu einer Kennung, falls die Kennung eines benennt.
    ///
    /// `None` heisst nicht fuer sich "unbekannte Funktion": die vom Hauptmenue
    /// zugestellten Funktionen tragen kein Kommando und stehen in
    /// [`Zugestellt::KENNUNGEN`]. Wer beide Arten sucht, fragt
    /// [`Funktionsschluessel::aus_kennung`].
    pub fn aus_kennung(kennung: &str) -> Option<Kommando> {
        Self::KENNUNGEN
            .into_iter()
            .find(|(_, benannt)| *benannt == kennung)
            .map(|(kommando, _)| kommando)
    }

    /// Welcher Bereich den Eingabefokus haben muss, damit dieses Kommando
    /// wirkt (C5).
    ///
    /// **Genau einer je Kommando, und das erzwingt der Uebersetzer.** Die
    /// Zuordnung steht als vollstaendige Fallunterscheidung ohne
    /// Auffangzweig: ein neues Kommando uebersetzt nicht, bevor es hier seinen
    /// Bereich genannt hat, und mehr als einen kann keines tragen. Eine
    /// Tabelle mit Auffangzweig gaebe einem vergessenen Kommando
    /// stillschweigend den Bereich des Nachbarn.
    ///
    /// Sechs Gruppen, und die Grenze zwischen ihnen ist die Frage, **wer den
    /// Befehl ausfuehrt**. Was das Fenstermodell traegt, wirkt ueberall; was
    /// ein Dateifenster traegt, braucht dessen Fokus; was die Leiste traegt,
    /// den ihren; was der Editor traegt, den seinen; drei der vier Tabbefehle
    /// bedienen den Bereich mit Tabs, der den Fokus hat (C1 wie C6); und drei
    /// Befehle bedienen den Navigator als ganzen, ohne den Editor. Zwei
    /// Befehle folgen keiner der sechs Regeln und stehen deshalb hier:
    /// [`Kommando::LesezeichenAnlegen`] liest den Ordner des aktiven
    /// Dateifensters oder die Zeile der Schreibmarke im Editor und schreibt
    /// beides in die Leiste, braucht also keinen Bereich im Fokus; und die
    /// fuenf Fokusbefehle koennen nicht voraussetzen, wo der Fokus steht.
    ///
    /// **Ein dritter ist mit C4 der Runde 4 dazugekommen:**
    /// [`Kommando::TabSchliessen`] hat den Zweig der Tabbefehle verlassen und
    /// traegt seither [`Wirkungsbereich::Ueberall`]. Der Grund steht als
    /// Kommentar an seinem Zweig.
    ///
    /// **Ein vierter kommt mit C2 der Runde 6 dazu:**
    /// [`Kommando::OrdnerDerDatei`] traegt [`Wirkungsbereich::Ueberall`], weil
    /// seine Quelle die angezeigte Datei ist und nicht der Fokus. Der Grund
    /// steht ebenfalls als Kommentar an seinem Zweig.
    ///
    /// **Ein fuenfter mit C1 derselben Runde:** [`Kommando::Teilen`] traegt
    /// ihn aus einer anderen Erwaegung als die vier davor. Bei ihnen haengt
    /// die Quelle nicht am Fokus; bei ihm haengt sie daran, und gerade deshalb
    /// muss er ueberall durchkommen. Der Grund steht als Kommentar an seinem
    /// Zweig.
    #[must_use]
    pub const fn wirkungsbereich(self) -> Wirkungsbereich {
        match self {
            // Das Fenster als ganzes. Die Belegungsansicht aus C3 steht hier,
            // weil sie aus jedem Fokus heraus erreichbar sein muss: sie zeigt
            // die Belegung der ganzen Anwendung und gehoert keinem Bereich,
            // so wenig wie das Ein- und Ausblenden der Bereiche auf F3.
            //
            // **Die beiden Umschalter der Bereichsleisten-Runde stehen mit
            // hier, und `editor_umschalten` faellt dabei auf**: jeder andere
            // Befehl mit `editor_` im Namen traegt weiter unten
            // `Wirkungsbereich::Editor`. Der Unterschied ist derselbe wie
            // zwischen `vorschau_umschalten` und den Befehlen, die in der
            // Vorschau arbeiten: ein Umschalter braucht seinen Bereich nicht,
            // er stellt ihn her. Ein Schalter in der Bereichsleiste muss
            // daneben aus jedem Fokus wirken, auch mit der Schreibmarke im
            // Text; mit `Wirkungsbereich::Editor` waere genau der Klick
            // abgewiesen, der den Editor wieder loswerden will.
            // **Die drei Spaltenschalter stehen mit hier, obwohl sie in den
            // Dateilisten wirken**, und der Grund ist nicht derselbe wie bei
            // den Umschaltern darueber: sie stellen keinen Bereich her, sie
            // treffen **beide** Listen zugleich (Nutzerentscheid vom
            // 260812-0306). Ein Befehl, der beide angeht, kann nicht eine von
            // ihnen im Fokus voraussetzen; mit `Wirkungsbereich::Dateifenster`
            // waere er von der Bereichsleiste aus, mit der Schreibmarke im
            // Editor oder mit dem Fokus in der Leiste abgewiesen, obwohl es
            // keine Seite gibt, auf die er sich bezoege.
            Kommando::LeisteUmschalten
            | Kommando::ErstesFensterUmschalten
            | Kommando::ZweitesFensterUmschalten
            | Kommando::VorschauUmschalten
            | Kommando::EditorUmschalten
            | Kommando::SpalteGroesseUmschalten
            | Kommando::SpalteDatumUmschalten
            | Kommando::SpalteTypUmschalten
            // Der vierte Spaltenschalter aus C5 der Runde 23 steht neben den
            // drei anderen und aus genau derselben Erwaegung: er trifft beide
            // Dateilisten zugleich, und ein Befehl, der beide angeht, kann
            // keine von ihnen im Fokus voraussetzen.
            | Kommando::SpalteMarkeUmschalten
            // Der sechste Bereichsumschalter aus C1 der Runde 23 steht bei den
            // fuenf anderen und aus derselben Erwaegung wie sie: er braucht
            // seinen Bereich nicht, er stellt ihn her, und sein Schalter in der
            // Bereichsleiste faellt aus jedem Fokus an.
            | Kommando::GitBereichUmschalten
            // Der Schalter "Deep" aus C5 der Filter-Runde steht neben den drei
            // Spaltenschaltern darueber und aus derselben Erwaegung: er ist ein
            // Schalter der Bereichsleiste, und ein Klick auf die Leiste faellt
            // aus jedem Fokus an. Mit einem engeren Bereich waere er genau dann
            // abgewiesen, wenn der Nutzer ihn braucht — etwa mit der
            // Schreibmarke im Editor oder mit dem Fokus in der Leiste.
            //
            // **Der Unterschied zu den Spaltenschaltern liegt allein im Ziel**,
            // nicht in der Begruendung: sie treffen beide Dateilisten, er das
            // Modell des sichtbaren Tabs im aktiven Dateifenster. Ein
            // `Wirkungsbereich::Dateifenster` daraus zu machen hiesse, den
            // Klick auf ein Kaestchen davon abhaengig zu machen, wo die
            // Schreibmarke gerade steht.
            | Kommando::TiefeSucheUmschalten
            // Der Schalter "Content" aus C2 der Inhaltsfilter-Runde steht
            // neben "Deep" und aus genau derselben Erwaegung: er ist ein
            // Schalter der Bereichsleiste, er trifft das Modell des sichtbaren
            // Tabs im aktiven Dateifenster, und ein Klick auf die Leiste faellt
            // aus jedem Fokus an. Die beiden gehoeren zusammen und stehen
            // deshalb auch hier nebeneinander.
            | Kommando::InhaltssucheUmschalten
            | Kommando::FensterEinblenden
            | Kommando::FensterSchliessen
            | Kommando::BereichVerbreitern
            | Kommando::BereichVerschmaelern
            | Kommando::Abbrechen
            | Kommando::BelegungAnsehen
            // Die Belegungsdatei steht hier aus demselben Grund wie die
            // Belegungsansicht darueber: sie ist Bestand der Anwendung und
            // gehoert keinem der sechs Bereiche. Der Befehl **holt** die
            // Vorschau hervor und den Fokus hinein; ein
            // `Wirkungsbereich::Vorschau` verlangte damit genau den Zustand,
            // den er selbst herstellt, und der Nutzer bekaeme seine Datei aus
            // dem Dateifenster heraus nicht mehr zu sehen. Dieselbe Erwaegung
            // traegt schon `Kommando::FokusVorschau`.
            | Kommando::BelegungsdateiAnsehen
            | Kommando::Beenden
            // Die weitere Instanz aus C3 der Runde 7 steht hier aus demselben
            // Grund wie das Beenden daneben: sie betrifft die Anwendung als
            // ganze und keinen ihrer Bereiche. Wer sie aus dem Editor heraus
            // ruft, will nicht den Editor verlassen, sondern ein zweites KRK.
            | Kommando::WeitereInstanz
            // Der Notizordner aus C1 der krkhome-Arbeit wirkt aus jedem
            // Bereich heraus: F2 soll aus dem Editor, der Vorschau oder der
            // Leiste ebenso nach `~/krkhome/` fuehren wie aus dem Dateifenster,
            // und er setzt den Fokus selbst in das Dateifenster. Ein
            // Wirkungsbereich, der einen Bereich verlangte, verlangte damit
            // den Zustand, den der Befehl erst herstellt.
            | Kommando::Notizordner
            // „Ort waehlen…“ aus demselben Grund wie F2 darueber: sein Dialog
            // haengt am Hauptfenster und nicht an einem Bereich, und wer den
            // Ort aus dem Editor heraus wechseln will, soll dafuer nicht erst
            // den Fokus umsetzen. Ob der Editor dabei eine Datei des
            // Notizordners haelt, fragt der Ausfuehrungszweig und nicht der
            // Wirkungsbereich: eine Antwort hier graute den Menueeintrag aus,
            // ohne zu sagen, warum.
            | Kommando::OrtWaehlen
            // Das Blatt der Ablageneuerungen steht hier, weil es am
            // Hauptfenster herunterfaehrt und sein Gegenstand die Ablage ist,
            // nicht einer der Bereiche der Fensterzeile. Wer beim Start gelesen hat, dass
            // seine `readers.toml` zurueckliegt, will das Einzelne von dort
            // aus sehen, wo er gerade steht, und nicht erst den Fokus
            // umsetzen.
            | Kommando::NeuerungenZeigen
            // Das Zuruecksetzen auf Werkseinstellungen aus demselben Grund wie
            // die Neuerungen darueber: seine Rueckfrage haengt als Blatt am
            // Hauptfenster, sein Gegenstand ist die Ablage, und kein Bereich
            // der Fensterzeile ist davon betroffen.
            | Kommando::Werkseinstellungen
            // F10 holt die Quicknote aus jedem Bereich heraus und setzt den
            // Fokus selbst in den Editor; ein Vorbehalt verlangte den Zustand,
            // den der Befehl erst herstellt, wie beim Notizordner darueber.
            // Blatt, Textfeld und fremdes Fenster halten ihn weiter an.
            | Kommando::QuicknoteUmschalten => Wirkungsbereich::Ueberall,
            // Die drei Befehle des Navigators, deren Taste im Editor der
            // Textflaeche gehoert.
            //
            // Sie sind in der Runde 1 mit `Ueberall` entstanden, weil es
            // damals nichts gab, wovon sie auszunehmen waeren: `tab`, `up` und
            // `down` fanden im ganzen Fenster keinen Bereich, der sie selbst
            // braucht. Mit dem eingebauten Editor gibt es einen. Ohne diesen
            // Umzug bewegte ein `up` mit dem Fokus im Editor die Auswahl im
            // Dateifenster statt der Schreibmarke, und `tab` wechselte das
            // Dateifenster, statt einen Tabulator zu schreiben; das erste
            // Abnahmekriterium von C7 der Editor-Runde waere gebrochen.
            //
            // `fenster_wechseln` ist dabei eine Ableitung des Planners und
            // keine Antwort des Nutzers: C7 sagt zu, dass eine Zeichentaste im
            // Editor ihr Zeichen einfuegt, und zaehlt den Befehl unter denen,
            // die dort wirken muessen, nicht auf.
            Kommando::FensterWechseln | Kommando::AuswahlHoch | Kommando::AuswahlRunter => {
                Wirkungsbereich::Navigator
            }
            // Der Fokuswechsel selbst und das Anlegen eines Lesezeichens (C5).
            //
            // Jeder Fokusbefehl steht hier, und sie muessen es: ein
            // Befehl, der den Fokus **holt**, kann nicht voraussetzen, wo er
            // gerade steht. Traege einer von ihnen den Bereich, in den er
            // fuehrt, waere er allein von dort aus erreichbar und damit
            // nutzlos.
            Kommando::FokusLeiste
            | Kommando::FokusDateifenster
            | Kommando::FokusVorschau
            | Kommando::FokusEditor
            // Der fuenfte Fokusbefehl aus C2 der Runde 23. Er steht hier aus
            // demselben Satz wie die vier darueber, und der Satz gilt ihm
            // woertlich: ein Befehl, der den Fokus in den Git-Bereich holt,
            // waere mit `Wirkungsbereich::Git` allein von dort aus erreichbar.
            | Kommando::FokusGit
            | Kommando::LesezeichenAnlegen => Wirkungsbereich::Ueberall,
            // Der Rundweg in den Editor und zurueck (C2 der Editor-Runde,
            // Nutzerentscheid vom 260823-0942), der einzige Befehl mit diesem
            // Wert. Er bedeutet in jedem der drei Bereiche etwas — in der
            // Dateiliste den ausgewaehlten Eintrag, in der Vorschau die
            // angezeigte Datei, im Editor den Rueckweg —, und ein
            // Wirkungsbereich, der einen davon nennte, schnitte die beiden
            // anderen ab. Die Leiste bleibt draussen: dort gibt es keine Datei,
            // die der Befehl meinte.
            //
            // Bis zum 260823 stand hier `Wirkungsbereich::Vorschau`, und der
            // Befehl hiess `EditorAusVorschau`. Welche der drei Bedeutungen
            // gilt, entscheidet `krk_ui::kommandos::rundweg` und nicht dieser
            // Zweig; der Wirkungsbereich sagt, ob die Taste durchkommt, und
            // nicht, was sie dann tut.
            Kommando::EditorRundweg => Wirkungsbereich::Dateibereiche,
            // Die drei Zoombefehle des PDF-Betrachters aus der Runde 20. Der
            // Betrachter steht allein im Vorschaufenster, und dort allein
            // bedeuten die drei etwas; mit dem Fokus anderswo sind sie
            // abgewiesen und im Hauptmenue ausgegraut (C3.5). Ob ein PDF
            // angezeigt wird, fragt der Bereich nicht (A6).
            Kommando::VorschauVergroessern
            | Kommando::VorschauVerkleinern
            | Kommando::VorschauAusgangsgroesse => Wirkungsbereich::Vorschau,
            // Die drei Befehle der Bildfolge: der Fokus in der Dateiliste und
            // eine Bildfolge in der Vorschau. Der Bereich verengt das
            // Dateifenster, und deshalb teilen `bild_zurueck` und `zum_bild`
            // ihre Kombination mit `ordner_aufwaerts` und
            // `mit_standardprogramm_oeffnen` (Modulkopf).
            Kommando::BildVor | Kommando::BildZurueck | Kommando::ZumBild => {
                Wirkungsbereich::Bildfolge
            }
            // Die Befehle, die an der Datei arbeiten, die der Editor haelt,
            // gleich wie er sie zeigt (C3 und C4 der Editor-Runde). Mit dem
            // Fokus anderswo gibt es keine solche Datei.
            //
            // `bearbeiten` steht **nicht** hier, sondern beim Dateifenster:
            // F4 oeffnet dessen ausgewaehlten Eintrag und setzt den Editor
            // nicht voraus, sondern fuellt ihn.
            Kommando::EditorSchliessen
            | Kommando::EditorAnsichtUmschalten
            | Kommando::EditorSichern => Wirkungsbereich::Editor,
            // Der Zeilensprung, die Suche und das Ersetzen (C5 der
            // Editor-Runde) wirken an der Schreibmarke und der Auswahl der
            // Textflaeche. Zeigt der Editor eine Eintragstabelle, ist die
            // Textflaeche ausgeblendet, und ein Treffer oder ein Sprung
            // landete unsichtbar darin; `krk_ui` weist die sechs dann ab und
            // graut sie im Menue aus (Schritt 3.3 der krkhome-Arbeit).
            Kommando::EditorZeileSpringen
            | Kommando::EditorSuchen
            | Kommando::EditorWeitersuchen
            | Kommando::EditorRueckwaertsSuchen
            | Kommando::EditorErsetzen
            | Kommando::EditorAlleErsetzen => Wirkungsbereich::Editortext,
            // Die Befehle der Eintragstabelle (C6 der krkhome-Arbeit). Diese
            // drei wirken in jeder Tabellenform; das Abhaken allein in der
            // Aufgabentabelle, denn eine Notiz hat kein Kaestchen.
            Kommando::EintragHinzufuegen
            | Kommando::EintragBearbeiten
            | Kommando::EintragLoeschen => Wirkungsbereich::Eintraege,
            // Das Verschieben wirkt allein in einer Tabelle, die in der
            // Reihenfolge der Datei steht; die Termintabelle ordnet nach dem
            // Datum (Entscheidung 2 des Plans
            // `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
            Kommando::EintragHoch | Kommando::EintragRunter => Wirkungsbereich::Reihenfolge,
            Kommando::AufgabeAbhaken => Wirkungsbereich::Aufgaben,
            // „PIN ändern" (C7, Schritt 5.5 der krkhome-Arbeit): allein an der
            // entsperrten `secrets.txt`, deren PIN schon in einem Kopf steht;
            // ob das so ist, fragt `krk_ui`.
            Kommando::PinAendern => Wirkungsbereich::Geheimnisse,
            // Allein in der Termintabelle, die nach dem Datum ordnet (Schritt 8
            // des Plans der Termine).
            Kommando::TermineRichtungUmkehren => Wirkungsbereich::Termine,
            // Allein am Puffer der offenen Quicknote (Q2 des Spec der
            // Quicknote); ob sie offen ist, fragt `krk_ui` ueber die Form.
            Kommando::QuicknoteKopieren | Kommando::QuicknoteLeeren => Wirkungsbereich::Quicknote,
            // Die Leiste (C5).
            Kommando::LesezeichenUmbenennen
            | Kommando::LesezeichenLoeschen
            | Kommando::LesezeichenHoch
            | Kommando::LesezeichenRunter => Wirkungsbereich::Leiste,
            // Drei der vier Tabbefehle aus C1. Sie bedienen nach C6 auch die
            // Tabs des Vorschaufensters und wirken auf den Bereich mit Tabs,
            // der den Fokus hat; so fuehrt es die Auslieferungsbelegung seit
            // S9.
            Kommando::TabNeu | Kommando::TabNaechster | Kommando::TabVoriger => {
                Wirkungsbereich::Tabbereich
            }
            // Der vierte, seit C4 der Runde 4 allein (Nutzerantwort vom
            // 260811-1505).
            //
            // Der Befehl schliesst einen Tab und setzt deshalb keinen Bereich
            // mit Tabs im Fokus voraus, sondern eine aktive Fensterseite — und
            // die gibt es immer. Mit dem Fokus in der Leiste oder im Editor
            // schliesst `cmd+w` seither den aktiven Tab des aktiven
            // Dateifensters; mit dem Fokus in einem Dateifenster oder in der
            // Vorschau bleibt es bei dem Bereich vor dem Nutzer. Welcher der
            // beiden Wege gilt, entscheidet die eine Verzweigung in
            // `Anwendungsdelegierter::tab_schliessen`, und der Editor wird auf
            // keinem von beiden angefasst.
            //
            // **Der Preis ist benannt und angenommen:** bei stehendem Blatt
            // kommt `cmd+w` weiterhin nicht durch, denn `waehrend_blatt_erlaubt`
            // laesst allein den Abbruch durch. In der Belegungsansicht — sie
            // ist ein Blatt und kein Fenster — bleibt `cmd+w` damit
            // wirkungslos, und `esc` bleibt der Weg heraus.
            Kommando::TabSchliessen => Wirkungsbereich::Ueberall,
            // Der Ordnersprung aus C2 der Runde 6, aus derselben Erwaegung
            // wie `tab_schliessen` darueber: **seine Quelle haengt nicht am
            // Fokus, und sein Ziel gibt es immer.** Woher der Pfad kommt,
            // beantwortet `angezeigtedatei::welche` aus der Sichtbarkeit von
            // Vorschau und Editor, und wo der Ordner erscheint, ist das aktive
            // Dateifenster. Mit `Wirkungsbereich::Dateifenster` waere der
            // Befehl genau dort abgewiesen, wo er am meisten taugt: mit dem
            // Fokus in der Vorschau oder im Editor, also vor der Datei, um
            // deren Ordner es geht.
            //
            // Gibt es keine angezeigte Datei, wirkt der Befehl trotzdem und
            // meldet es (C2, fuenftes Kriterium); der Wirkungsbereich
            // entscheidet, ob eine Taste durchkommt, und nicht, ob sie etwas
            // findet.
            Kommando::OrdnerDerDatei => Wirkungsbereich::Ueberall,
            // Das Teilen aus C1 der Runde 6, und die Erwaegung ist hier eine
            // andere als bei den vier Befehlen darueber: **der Fokus
            // entscheidet nicht, ob dieser Befehl wirkt, sondern worauf.** Er
            // bedeutet in drei Bereichen etwas — im Dateifenster die
            // betroffenen Eintraege, in der Vorschau und im Editor die
            // angezeigte Datei —, und ein Wirkungsbereich, der einen davon
            // nennte, schnitte die beiden anderen ab. Die Verzweigung selbst
            // wohnt in `krk-ui` unter `appkit::teilen::worauf`, weil sie
            // Bereiche der Oberflaeche kennt und der Kern sie nicht kennt.
            //
            // Mit dem Fokus in der Leiste findet der Befehl nichts und meldet
            // es. Auch das ist kein Fall fuer den Wirkungsbereich: der
            // entscheidet, ob eine Taste durchkommt, und nicht, ob sie etwas
            // findet — derselbe Satz wie beim Ordnersprung darueber.
            Kommando::Teilen => Wirkungsbereich::Ueberall,
            // Alles, was ein Dateifenster ausfuehrt: Bewegung ueber die Liste
            // hinaus, Navigation, Markierung, Sortierung, die Dateioperationen
            // aus C4, die beiden Zwischenablage-Befehle aus C10
            // (Nutzerentscheid vom 260805-0000), der Terminal-Befehl aus C11,
            // der den Ordner des sichtbaren Tabs uebergibt, und F4 aus C1 der
            // Editor-Runde, das den **ausgewaehlten Eintrag** des
            // Dateifensters im Editor oeffnet.
            //
            // Die drei Befehle der Runde 4 stehen aus demselben Grund hier: sie
            // brauchen den angezeigten Ordner oder die betroffenen Eintraege,
            // und beides gibt es nur mit dem Fokus im Dateifenster. Die Folge
            // ist benannt und gewollt: mit dem Fokus im Editor kopiert
            // `shift+cmd+c` keinen Pfad und tut nichts. Der Editor haelt eine
            // Datei, und deren Pfad steht im Fenstertitel; ein Kopierbefehl
            // dafuer waere eine eigene Funktion.
            Kommando::Bearbeiten
            | Kommando::SeiteHoch
            | Kommando::SeiteRunter
            | Kommando::Listenanfang
            | Kommando::Listenende
            | Kommando::Oeffnen
            | Kommando::OrdnerAufwaerts
            // Das Angleichen aus C1 der Runde 13 steht **auf der anderen Seite
            // der Linie** als der Ordnersprung darueber, und das ist kein
            // Widerspruch. Der Ordnersprung traegt `Ueberall`, weil seine
            // Quelle nicht am Fokus haengt: er wird aus Vorschau und Editor
            // heraus gedrueckt, also aus Bereichen, die keine Dateifenster
            // sind, und mit `Dateifenster` waere er genau dort abgewiesen, wo
            // er am meisten taugt. Das Angleichen liegt umgekehrt: seine
            // Quelle **ist** der angezeigte Ordner eines Dateifensters, und
            // ausserhalb eines Dateifensters hat der Befehl keinen Gegenstand.
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
            | Kommando::ZwischenablageSpringen
            | Kommando::ZwischenablageAnsehen
            | Kommando::Kopieren
            | Kommando::Verschieben
            | Kommando::InPapierkorb
            | Kommando::OrdnerAnlegen
            | Kommando::DateiAnlegen
            | Kommando::UmbenennenStapel
            | Kommando::Umbenennen
            | Kommando::TerminalOeffnen
            | Kommando::OrdnerpfadKopieren
            | Kommando::EintragspfadKopieren
            | Kommando::MitStandardprogrammOeffnen => Wirkungsbereich::Dateifenster,
        }
    }

    /// Die Kennung dieses Kommandos in der Belegungsdatei.
    ///
    /// **Verglichen wird ueber die Zahl der Variante und nicht ueber
    /// `PartialEq`**, weil `PartialEq` keine `const`-Faehigkeit ist und diese
    /// Funktion zur Uebersetzungszeit auswertbar bleiben soll.
    ///
    /// **`u16` und nicht `u8`, und die Schranke steht daneben statt still zu
    /// sein.** Ueber `as u8` schnitt die Umwandlung ab der 257. Variante ab:
    /// zwei Kommandos verglichen sich gleich, und die Funktion lieferte die
    /// Kennung des falschen, statt anzuhalten — im Hauptmenue, in der
    /// Belegungsansicht und in der Belegungsausgabe. Die Aufzaehlung hat keine
    /// gesetzte Obergrenze und waechst mit fast jeder Runde, also darf die
    /// einzige Schranke im Code nicht die Nebenwirkung eines
    /// Umwandlungsoperators sein. Die Zusicherung unmittelbar unter der
    /// Aufzaehlung [`Kommando`] haelt sie beim Uebersetzen
    /// (`shared/issues/260826-1223_*_kommando-kennung-vergleicht-ueber-as-u8-und-liefert-oberhalb-von-256-varianten-still-die-falsche-kennung.md`).
    #[must_use]
    pub const fn kennung(self) -> &'static str {
        let mut stelle = 0;
        while stelle < Self::KENNUNGEN.len() {
            let (kommando, kennung) = Self::KENNUNGEN[stelle];
            if kommando as u16 == self as u16 {
                return kennung;
            }
            stelle += 1;
        }
        panic!("jedes Kommando steht in KENNUNGEN")
    }
}

/// Die Funktionen, die nicht der Ereignisabgriff ausfuehrt, sondern das
/// Hauptmenue zustellt: die Textbefehle des Menues „Bearbeiten“ und das
/// Einfuegen in den Filter.
///
/// Sie tragen kein [`Kommando`], weil KRK sie nie selbst ausfuehrt (vierte
/// Stelle der Zustellerregel im Modulkopf), und brauchten deshalb bis zu
/// dieser Arbeit keine Aufzaehlung: ihr Name kam aus der Belegungsdatei. Seit
/// der Name aus der Sprachtabelle kommt, braucht jede von ihnen einen
/// Schluessel, und die Tabellen verzweigen ueber diese Aufzaehlung ohne
/// Auffangzweig, wie ueber `Kommando`: eine achte zugestellte Funktion haelt
/// den Bau an, bis sie drei Namen hat.
///
/// Wie bei [`Kommando`] ist [`Zugestellt::KENNUNGEN`] die eine Verbindung zur
/// Kennung in der Datei, und die Probe
/// `jede_variante_von_zugestellt_steht_genau_einmal_in_kennungen`
/// (`crates/krk-core/tests/belegung.rs`) haelt die Liste gegen die
/// Aufzaehlung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zugestellt {
    /// Die Zwischenablage an den Filtertext der Dateiliste haengen.
    FilterEinfuegen,
    /// Ausschneiden im Textfeld.
    TextAusschneiden,
    /// Kopieren im Textfeld.
    TextKopieren,
    /// Einfuegen im Textfeld.
    TextEinfuegen,
    /// Alles auswaehlen im Textfeld.
    TextAllesAuswaehlen,
    /// Rueckgaengig im Textfeld.
    TextRueckgaengig,
    /// Wiederholen im Textfeld.
    TextWiederholen,
}

/// Dieselbe Schranke wie unter [`Kommando`]: [`Zugestellt::kennung`]
/// vergleicht ueber `as u16`.
const _: () = assert!(Zugestellt::KENNUNGEN.len() <= u16::MAX as usize);

impl Zugestellt {
    /// Die Kennung, unter der die Belegungsdatei die zugehoerige Funktion
    /// fuehrt, je zugestellter Funktion.
    pub const KENNUNGEN: [(Zugestellt, &'static str); 7] = [
        (Zugestellt::FilterEinfuegen, "filter_einfuegen"),
        (Zugestellt::TextAusschneiden, "text_ausschneiden"),
        (Zugestellt::TextKopieren, "text_kopieren"),
        (Zugestellt::TextEinfuegen, "text_einfuegen"),
        (Zugestellt::TextAllesAuswaehlen, "text_alles_auswaehlen"),
        (Zugestellt::TextRueckgaengig, "text_rueckgaengig"),
        (Zugestellt::TextWiederholen, "text_wiederholen"),
    ];

    /// Die zugestellte Funktion zu einer Kennung, falls es eine ist.
    pub fn aus_kennung(kennung: &str) -> Option<Zugestellt> {
        Self::KENNUNGEN
            .into_iter()
            .find(|(_, benannt)| *benannt == kennung)
            .map(|(zugestellt, _)| zugestellt)
    }

    /// Die Kennung dieser zugestellten Funktion; das Gegenstueck zu
    /// [`Zugestellt::aus_kennung`], nach dem Muster von [`Kommando::kennung`].
    #[must_use]
    pub const fn kennung(self) -> &'static str {
        let mut stelle = 0;
        while stelle < Self::KENNUNGEN.len() {
            let (zugestellt, kennung) = Self::KENNUNGEN[stelle];
            if zugestellt as u16 == self as u16 {
                return kennung;
            }
            stelle += 1;
        }
        panic!("jede zugestellte Funktion steht in KENNUNGEN")
    }
}

/// Der Schluessel einer Funktion: das, worunter die Sprachtabelle ihren Namen
/// fuehrt.
///
/// Jede Funktion der Belegung ist entweder ein [`Kommando`] oder eine vom
/// Hauptmenue [`Zugestellt`]e, und eine dritte Art gibt es nicht:
/// [`Belegung::bauen`] weist eine Kennung ab, die keines von beiden ist, fuer
/// die Auslieferung wie fuer die Nutzerdatei. Damit ist die Kennung aus der
/// Datei nur noch die Schreibweise des Schluessels, und [`Funktion`] haelt den
/// Schluessel statt eines Namens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Funktionsschluessel {
    /// Eine Funktion, die der Ereignisabgriff als Kommando ausfuehrt.
    Kommando(Kommando),
    /// Eine Funktion, die das Hauptmenue zustellt.
    Zugestellt(Zugestellt),
}

impl Funktionsschluessel {
    /// Der Schluessel zu einer Kennung, aus beiden `KENNUNGEN`; `None` ist
    /// eine Kennung, die KRK nicht kennt.
    pub fn aus_kennung(kennung: &str) -> Option<Self> {
        if let Some(kommando) = Kommando::aus_kennung(kennung) {
            return Some(Self::Kommando(kommando));
        }
        Zugestellt::aus_kennung(kennung).map(Self::Zugestellt)
    }

    /// Die Kennung, unter der die Belegungsdatei diese Funktion fuehrt.
    #[must_use]
    pub const fn kennung(self) -> &'static str {
        match self {
            Self::Kommando(kommando) => kommando.kennung(),
            Self::Zugestellt(zugestellt) => zugestellt.kennung(),
        }
    }
}

/// Eine Funktion mit allen ihren Kombinationen: eine Zeile der
/// Belegungsansicht.
///
/// Sie haelt ihren [`Funktionsschluessel`] und keinen Namen: der Name kommt
/// aus der Sprachtabelle, in der Sprache, die beim Aufruf von
/// [`Funktion::name`] gilt, und die Kennung ist die Schreibweise des
/// Schluessels in der Datei.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Funktion {
    schluessel: Funktionsschluessel,
    tasten: Vec<Kombination>,
    reserviert_fuer: Option<String>,
    gehalten_von: Option<String>,
}

impl Funktion {
    /// Der maschinenlesbare Bezeichner, unter dem `keymap.toml` sie fuehrt.
    #[must_use]
    pub fn kennung(&self) -> &'static str {
        self.schluessel.kennung()
    }

    /// Der Schluessel, unter dem die Sprachtabelle ihren Namen fuehrt.
    #[must_use]
    pub fn schluessel(&self) -> Funktionsschluessel {
        self.schluessel
    }

    /// Die Beschriftung fuer die Belegungsansicht, das Hauptmenue und jede
    /// Meldung, in der geltenden Sprache.
    ///
    /// Aus der Sprachtabelle und nie aus einer Datei: ein `name` in der
    /// eigenen `keymap.toml` wird geduldet und nicht gelesen (Modulkopf,
    /// „Die Nutzerdatei ersetzt, sie ergaenzt nicht“).
    #[must_use]
    pub fn name(&self) -> &'static str {
        funktionsname(self.schluessel)
    }

    /// Die Beschriftung in einer ausdruecklich genannten Sprache; fuer die
    /// Proben, die jede Sprache sehen wollen, waehrend `cargo test` immer
    /// Deutsch gilt.
    #[must_use]
    pub fn name_in(&self, sprache: Sprache) -> &'static str {
        sprache.funktionsname(self.schluessel)
    }

    /// Alle Kombinationen, die diese Funktion ausloesen.
    #[must_use]
    pub fn tasten(&self) -> &[Kombination] {
        &self.tasten
    }

    /// Gesetzt, wenn die Funktion benannt, aber einer spaeteren Runde
    /// vorbehalten ist.
    pub fn reserviert_fuer(&self) -> Option<&str> {
        self.reserviert_fuer.as_deref()
    }

    /// Wer den Tastendruck zustellt, falls es nicht der Ereignisabgriff ist.
    ///
    /// `None` heisst: der Abgriff aus C2 stellt zu. `Some("menue")` heisst: ein
    /// `NSMenuItem` traegt die Kombination als Kuerzel, und die Antwortkette
    /// entscheidet, wer sie beantwortet. Siehe den Modulkopf; das Feld sagt,
    /// **wer zustellt**, und nicht, was der Tastendruck tut.
    pub fn gehalten_von(&self) -> Option<&str> {
        self.gehalten_von.as_deref()
    }

    /// Das Kommando dieser Funktion, falls sie eines traegt.
    ///
    /// Eine zugestellte Funktion hat nie eines: was das Hauptmenue zustellt,
    /// fuehrt die Antwortkette aus und nicht KRK. Die Frage nach dem Zusteller
    /// steht vor der nach dem Schluessel, damit die Zusage nicht daran haengt,
    /// dass der Schluessel einer zugestellten Funktion zufaellig kein
    /// [`Kommando`] ist — die vierte Stelle der Zustellerregel aus dem
    /// Modulkopf.
    pub fn kommando(&self) -> Option<Kommando> {
        if self.gehalten_von.is_some() {
            return None;
        }
        match self.schluessel {
            Funktionsschluessel::Kommando(kommando) => Some(kommando),
            Funktionsschluessel::Zugestellt(_) => None,
        }
    }

    /// Der Wirkungsbereich des Kommandos dieser Funktion, oder `None`, wenn
    /// sie keines hat.
    ///
    /// Eine Funktion ohne Kommando hat keinen Wirkungsbereich und schliesst
    /// deshalb nichts aus; fuer die Konfliktregel begegnet sie jeder anderen
    /// desselben Zustellers.
    pub fn wirkungsbereich(&self) -> Option<Wirkungsbereich> {
        self.kommando().map(Kommando::wirkungsbereich)
    }

    /// Wie eine Meldung diese Funktion benennt: mit der Kennung und dem Namen
    /// in der Sprache, die im Augenblick des Aufrufs gilt.
    #[must_use]
    pub fn benennung(&self) -> Funktionsname {
        Funktionsname::neu(self.kennung(), self.name())
    }
}

/// Was ein Tastendruck in der Belegung findet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nachschlag<'a> {
    /// Die Kombination gehoert dieser Funktion.
    Funktion(&'a Funktion),
    /// Die Kombination gehoert zwei Funktionen, deren Wirkungsbereiche
    /// einander ausschliessen oder von denen die eine die andere verengt.
    ///
    /// Die Reihenfolge ist bei einem Ausschluss die der Belegung und bei einer
    /// Verengung **die engere zuerst**, gleich wie die Datei sie ordnet.
    /// Welche gemeint ist, entscheidet der Aufrufer an der Lage, in der der
    /// Tastendruck ankommt: nach der Konfliktregel ist hoechstens eine der
    /// beiden zulaessig, oder die engere geht vor (Modulkopf, „Zwei Funktionen
    /// eines Zustellers auf einer Kombination“).
    Geteilt(&'a Funktion, &'a Funktion),
    /// Keine Funktion, und keine Befehlstaste gehalten: der Tastendruck faellt
    /// auf das Tippen durch.
    ///
    /// **Der Wert hiess bis zum 260816 `Sprungmarke`**, nach der Sprungmarke
    /// aus C2 der Runde 1, die die Runde 10 abgeloest hat. Der Name blieb
    /// damals stehen, weil er weiter zutraf: „eine Taste **ohne** Zusatztaste,
    /// die keiner Funktion gehoert". Genau dieser Satz stimmt seit dem
    /// Nutzerentscheid vom 260816-1105 nicht mehr — `shift` und `opt` fallen
    /// jetzt ebenfalls hierher —, und damit war der Name faellig. `Tippen`
    /// benennt, was der Wert aussagt: dieser Tastendruck gehoert dem Tippen.
    ///
    /// Wohin das getippte Zeichen laeuft, sagt er nicht und hat er nie gesagt:
    /// seit der Runde 10 ist es der Filtertext des sichtbaren Tabs.
    Tippen,
    /// Keine Funktion, und eine Befehlstaste gehalten: nichts geschieht.
    Unbelegt,
}

/// Die vollstaendige Belegung: jede Funktion mit ihren Kombinationen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Belegung {
    funktionen: Vec<Funktion>,
}

impl Belegung {
    /// Die eingebettete Auslieferungsbelegung.
    #[must_use]
    pub fn auslieferung() -> Self {
        AUSLIEFERUNG.clone()
    }

    /// Eine Belegung des Nutzers, geprueft gegen den Wortschatz der
    /// Auslieferungsbelegung.
    // Der Fehlerwert traegt einen [`Konflikt`] und damit die Namen beider
    // beteiligten Funktionen; er ist gross fuer einen `Err`. Ihn zu verpacken
    // spart Platz auf einem Pfad, den KRK hoechstens beim Start und bei einer
    // Umbelegung geht, und kostete an jeder Fundstelle eine Zeile, die vom
    // Sachverhalt ablenkt. Dieselbe Abwaegung gilt fuer `zuweisen` und `bauen`.
    #[allow(clippy::result_large_err)]
    pub fn vom_nutzer(datei: &Belegungsdatei) -> Result<Self, Belegungsfehler> {
        Self::bauen(datei, Some(&AUSLIEFERUNG))
    }

    /// Alle Funktionen, in der Reihenfolge der Datei.
    #[must_use]
    pub fn funktionen(&self) -> &[Funktion] {
        &self.funktionen
    }

    /// Die Funktion zu einer Kennung.
    pub fn funktion(&self, kennung: &str) -> Option<&Funktion> {
        self.funktionen
            .iter()
            .find(|funktion| funktion.kennung() == kennung)
    }

    /// Was ein Tastendruck ausloest.
    ///
    /// Der Durchlauf ist eine gewoehnliche Schleife ueber die wenigen Dutzend
    /// ausgelieferten Kombinationen und kein Nachschlagbaum. Verglichen werden
    /// zwei ganze Zahlen; gegen die Zusage L1 von einer Bildlaenge faellt das
    /// nicht ins Gewicht, und eine abgeleitete Tabelle daneben waere ein
    /// zweiter Bestand, den jede Aenderung mitfuehren muesste. Die Groessen-
    /// ordnung traegt das Argument, die genaue Zahl nicht: sie waechst mit
    /// jeder Runde, und ein Literal an dieser Stelle veraltet ungeprueft.
    ///
    /// **Eine vom Hauptmenue zugestellte Funktion kommt hier nicht vor.** Der
    /// Abgriff laeuft nur ausserhalb eines Textfeldes und darf deshalb nur
    /// sehen, was er selbst zustellt; der Modulkopf schreibt aus, warum das
    /// keine Zutat, sondern die tragende Haelfte der Zustellerregel ist. Nach
    /// dem Ueberspringen meint diese Antwort, was ihr Aufrufer braucht: was
    /// dieser Tastendruck **ausserhalb eines Textfeldes** ausloest.
    ///
    /// # Die zweite Nachschlagart, und warum sie hier keine Sonderregel ist
    ///
    /// Verglichen werden die Maske und die
    /// [`Tastenkennung`](super::parser::Tastenkennung): fuer Buchstaben
    /// und Ziffern das gemeldete **Zeichen**, fuer alles uebrige der virtuelle
    /// **Tastencode** (Nutzerentscheid vom 260808-0155, `decisions/
    /// 260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`).
    ///
    /// Der Vorgaengerdatensatz vom 260803 nannte eine zweite Nachschlagart
    /// "genau die Sonderregel, die die Maxime supersimpel meidet". Drei Gruende
    /// stehen dem hier entgegen, und alle drei sind am Code nachzusehen:
    ///
    /// 1. **Es ist keine zweite, sondern die schon vorhandene.** Das Hauptmenue
    ///    schlaegt seit S13b ueber das Zeichen nach; `NSMenuItem.keyEquivalent`
    ///    nimmt eine Zeichenkette (`crates/krk-ui/src/appkit/menue.rs`,
    ///    `zeichen_der_taste`; die Zeilenangabe, die hier bis zum 260908 stand,
    ///    zeigte ins Leere).
    ///    Vier Funktionen tragen sie bereits. Der Ereignisabgriff zieht damit
    ///    nach, und der Zuschnitt **beendet eine Asymmetrie**, statt eine zu
    ///    schaffen.
    /// 2. **Es ist ein Vergleich und nicht zwei.** Die Wahl steckt vollstaendig
    ///    in der Kennung, und beide Seiten leiten sie aus derselben Regel ab:
    ///    die Belegung ueber [`Taste::kennung`](super::parser::Taste::kennung),
    ///    der Tastendruck ueber [`Tastendruck::kennung`]. Hier steht deshalb
    ///    eine Gleichheit und kein Zweig je Tastensorte. Ohne diesen Zuschnitt
    ///    stuende an dieser Stelle kein einfacherer Vergleich, sondern eine
    ///    Ausnahmeliste je Kombination und je Tastaturbelegung.
    /// 3. **Ohne sie waere der Nachschlag nicht einmal eindeutig.** Zwei
    ///    Varianten der Kennung sind nie gleich; ein Buchstabe wird
    ///    nur ueber sein Zeichen gefunden und eine Stelle nur ueber ihren Code.
    ///    Ein Nachschlag, der beides gegen den Code fuehrte, traefe auf einer
    ///    franzoesischen Tastatur zwei verschiedene Tasten auf derselben
    ///    Funktion, und die Konflikterkennung saehe das nie.
    ///
    /// # Zwei Treffer
    ///
    /// **Der Lauf sammelt die Treffer, statt beim ersten zurueckzukehren**:
    /// eine Kombination kann seit dem 260926 zwei Funktionen gehoeren, deren
    /// Wirkungsbereiche einander ausschliessen oder einander verengen, und dann
    /// antwortet er mit [`Nachschlag::Geteilt`]. Bei einer Verengung nennt er
    /// die engere zuerst, **im Code und nicht nach der Datei**: eine eigene
    /// `keymap.toml` kann die zwei in beliebiger Reihenfolge fuehren, und der
    /// Vorrang der engeren darf daran nicht haengen. Mehr als zwei kann
    /// [`Belegung::bauen`] nicht durchlassen, denn die Konfliktregel laesst je
    /// Zusteller hoechstens zwei zu (`im_weg`). Der Lauf ist damit fuer jeden
    /// Treffer so lang wie der, den ein Anschlag ohne Funktion schon immer
    /// faehrt, und nie laenger.
    #[must_use]
    pub fn nachschlag(&self, druck: Tastendruck) -> Nachschlag<'_> {
        let mut erste: Option<&Funktion> = None;
        for funktion in &self.funktionen {
            if funktion.gehalten_von.is_some() {
                continue;
            }
            if funktion.tasten.iter().any(|kombination| {
                kombination.maske() == druck.maske
                    && kombination.taste().kennung() == druck.kennung()
            }) {
                match erste {
                    None => erste = Some(funktion),
                    Some(erste) => {
                        let zweite_verengt =
                            match (funktion.wirkungsbereich(), erste.wirkungsbereich()) {
                                (Some(zweite), Some(erste)) => zweite.verengt(erste),
                                _ => false,
                            };
                        return if zweite_verengt {
                            Nachschlag::Geteilt(funktion, erste)
                        } else {
                            Nachschlag::Geteilt(erste, funktion)
                        };
                    }
                }
            }
        }
        if let Some(funktion) = erste {
            return Nachschlag::Funktion(funktion);
        }
        // Hinter der Suche und nicht davor: was oben eine Funktion gefunden hat,
        // kommt hier nicht mehr an, und diese Unterscheidung kann deshalb keinem
        // belegten Kuerzel etwas wegnehmen. Sie trennt allein die beiden
        // Schreibtasten von den beiden Befehlstasten; der Modulkopf schreibt
        // aus, warum das die Trennung ist und nicht die leere Maske.
        //
        // Die Frage steht als zwei `enthaelt` und nicht als eine Maske mit zwei
        // Bits: `enthaelt` verlangt **alle** genannten Bits, und `cmd+ctrl+x`
        // haelt eine Befehlstaste schon mit einer von beiden.
        if druck.maske.enthaelt(ModMaske::BEFEHL) || druck.maske.enthaelt(ModMaske::STEUERUNG) {
            Nachschlag::Unbelegt
        } else {
            Nachschlag::Tippen
        }
    }

    /// Gibt einer Funktion eine weitere Kombination.
    ///
    /// Traegt die Funktion sie schon, geschieht nichts und es ist kein Fehler.
    /// Steht ihr ein **bisheriger Traeger** im Weg, bleibt die Belegung
    /// unveraendert und der [`Konflikt`] nennt beide Funktionen.
    ///
    /// Gefragt wird dasselbe `im_weg` wie in [`Belegung::konflikte`]: sonst
    /// meldete die Belegungsansicht aus C3 einen Konflikt, den das Einlesen
    /// nicht kennt, oder liesse einen durch, den das Einlesen abweist, und die
    /// beiden Wege in dieselbe Belegung widersprachen einander.
    #[allow(clippy::result_large_err)]
    pub fn zuweisen(
        &mut self,
        kennung: &str,
        kombination: Kombination,
    ) -> Result<(), Zuweisungsfehler> {
        let Some(stelle) = self
            .funktionen
            .iter()
            .position(|funktion| funktion.kennung() == kennung)
        else {
            return Err(Zuweisungsfehler::UnbekannteFunktion(kennung.to_owned()));
        };
        let bewerber = &self.funktionen[stelle];
        let traeger: Vec<&Funktion> = self
            .funktionen
            .iter()
            .filter(|funktion| {
                funktion.kennung() != kennung && funktion.tasten.contains(&kombination)
            })
            .collect();
        if let Some(andere) = im_weg(&traeger, bewerber) {
            return Err(Zuweisungsfehler::Konflikt(Konflikt {
                kombination,
                andere: andere.benennung(),
                bewerber: self.funktionen[stelle].benennung(),
            }));
        }

        if !self.funktionen[stelle].tasten.contains(&kombination) {
            self.funktionen[stelle].tasten.push(kombination);
        }
        Ok(())
    }

    /// Setzt die gesamte Belegung auf den Auslieferungszustand zurueck.
    pub fn zuruecksetzen(&mut self) {
        *self = Self::auslieferung();
    }

    /// Jede Kombination, an der eine Funktion ihre frueheren Traeger nicht
    /// mittragen darf: desselben Zustellers, und weder schliessen die
    /// Wirkungsbereiche einander aus noch verengt der eine den anderen, oder
    /// die Kombination traegt beim selben Zusteller schon zwei.
    ///
    /// **Je Funktion und Kombination hoechstens ein Konflikt**, denn `im_weg`
    /// nennt den ersten Traeger, an dem der Bewerber scheitert. Bis zur
    /// Bildfolge stand hier je frueherem Partner einer; kein Rufer zaehlt die
    /// Laenge, `bauen` nimmt den ersten.
    ///
    /// Leer fuer jede Belegung, die [`Belegung::vom_nutzer`] oder
    /// [`Belegung::auslieferung`] geliefert hat: beide weisen eine
    /// widerspruechliche Datei schon beim Einlesen ab. Die Pruefung steht
    /// trotzdem als eigener Aufruf da, weil das Abnahmekriterium von C3 sie
    /// woertlich verlangt.
    ///
    /// Der Zusteller und die Wirkungsbereiche gehoeren in den Vergleich, weil
    /// zwei Funktionen einander nur begegnen koennen, wenn beide im selben
    /// Fokuszustand erreichbar sind; der Modulkopf schreibt die Regel aus.
    #[must_use]
    pub fn konflikte(&self) -> Vec<Konflikt> {
        let mut gefunden = Vec::new();
        for (stelle, funktion) in self.funktionen.iter().enumerate() {
            for kombination in &funktion.tasten {
                let traeger: Vec<&Funktion> = self
                    .funktionen
                    .iter()
                    .take(stelle)
                    .filter(|vorige| vorige.tasten.contains(kombination))
                    .collect();
                if let Some(vorige) = im_weg(&traeger, funktion) {
                    gefunden.push(Konflikt {
                        kombination: *kombination,
                        andere: vorige.benennung(),
                        bewerber: funktion.benennung(),
                    });
                }
            }
        }
        gefunden
    }

    /// Schreibt die Belegung nach `keymap.toml`, atomar ueber die Ablage.
    ///
    /// Der [`Zugang`] ist seit der Runde 7 noetig und nicht die Ablage selbst:
    /// geschrieben wird unter der Schreibsperre, damit eine zweite Instanz von
    /// KRK nicht dieselbe Nachbardatei beschreibt.
    pub fn sichern(&self, zugang: &Zugang<'_>) -> io::Result<()> {
        zugang.sichern(Datei::Belegung, &Belegungsdatei::from(self))
    }

    /// Baut eine Belegung aus der gelesenen Datei.
    ///
    /// `wortschatz` ist `None` fuer die Auslieferungsbelegung, die ihn erst
    /// festlegt, und `Some` fuer jede Belegung des Nutzers, die sich daran
    /// messen lassen muss. **Eine Kennung, die kein [`Funktionsschluessel`]
    /// ist, weist beide Wege ab**: die Nutzerdatei mit
    /// [`Belegungsfehler::UnbekannteFunktion`] und dem Rueckfall auf die
    /// Auslieferung, die Auslieferung selbst mit dem Abbruch beim ersten
    /// Zugriff auf [`AUSLIEFERUNG`], denn eine Funktion ohne Schluessel haette
    /// in keiner Sprache einen Namen.
    #[allow(clippy::result_large_err)]
    fn bauen(
        datei: &Belegungsdatei,
        wortschatz: Option<&Belegung>,
    ) -> Result<Self, Belegungsfehler> {
        let mut funktionen: Vec<Funktion> = Vec::with_capacity(datei.funktionen.len());
        for eintrag in &datei.funktionen {
            // **Der Zusteller kommt aus dem Wortschatz und nicht aus der
            // Datei.** Er ist die tragende Haelfte der Zustellerregel, und ein
            // von Hand gesetztes `gehalten_von = "menue"` nahm bis zum 260908
            // einen gebauten Befehl still aus dem Ereignisabgriff — ohne
            // Konflikt, ohne Meldung, ohne Rueckfall
            // (`shared/issues/260826-1223_*_die-nutzerdatei-setzt-den-zusteller-frei-*`).
            // Ohne Wortschatz baut diese Stelle die Auslieferungsbelegung
            // selbst, und dann ist die Datei auch dafuer die Quelle.
            let bekannt = match wortschatz {
                Some(wortschatz) => match wortschatz.funktion(&eintrag.id) {
                    Some(bekannt) => Some(bekannt),
                    None => {
                        return Err(Belegungsfehler::UnbekannteFunktion(eintrag.id.clone()));
                    }
                },
                None => None,
            };
            // `eintrag.name` wird hier nicht gelesen: der Name kommt aus der
            // Sprachtabelle, und das Feld ist in der Nutzerdatei eine Duldung
            // wie `gehalten_von` (Modulkopf).
            let Some(schluessel) = Funktionsschluessel::aus_kennung(&eintrag.id) else {
                return Err(Belegungsfehler::UnbekannteFunktion(eintrag.id.clone()));
            };
            if funktionen
                .iter()
                .any(|funktion| funktion.schluessel == schluessel)
            {
                return Err(Belegungsfehler::FunktionDoppelt(eintrag.id.clone()));
            }

            let mut tasten = Vec::with_capacity(eintrag.tasten.len());
            for text in &eintrag.tasten {
                let kombination =
                    Kombination::lesen(text).map_err(|fehler| Belegungsfehler::Schreibweise {
                        kennung: eintrag.id.clone(),
                        text: text.clone(),
                        fehler,
                    })?;
                if !tasten.contains(&kombination) {
                    tasten.push(kombination);
                }
            }

            funktionen.push(Funktion {
                schluessel,
                tasten,
                reserviert_fuer: eintrag.reserviert_fuer.clone(),
                gehalten_von: match bekannt {
                    Some(bekannt) => bekannt.gehalten_von.clone(),
                    None => eintrag.gehalten_von.clone(),
                },
            });
        }

        // Funktionen, die die Nutzerdatei nicht nennt, treten unbelegt hinzu.
        // Die Belegungsansicht fuehrt damit weiter jede Funktion, und der
        // Nutzer kann eine, die er geloescht hat, wieder erreichbar machen.
        if let Some(wortschatz) = wortschatz {
            for bekannt in &wortschatz.funktionen {
                if !funktionen
                    .iter()
                    .any(|funktion| funktion.schluessel == bekannt.schluessel)
                {
                    funktionen.push(Funktion {
                        tasten: Vec::new(),
                        ..bekannt.clone()
                    });
                }
            }
        }

        let belegung = Self { funktionen };
        match belegung.konflikte().into_iter().next() {
            Some(konflikt) => Err(Belegungsfehler::Konflikt(konflikt)),
            None => Ok(belegung),
        }
    }
}

/// Die eine Fassung der Konfliktregel: der Traeger, an dem `bewerber` auf
/// dieser Kombination scheitert, oder `None`, wenn er sie mittragen darf.
///
/// Gefragt von [`Belegung::konflikte`] und [`Belegung::zuweisen`], mit allen
/// Funktionen, die die Kombination schon tragen, und nicht mit einer davon.
/// Der Rumpf in dieser Reihenfolge:
///
/// 1. Traeger eines anderen Zustellers fallen heraus; ihnen begegnet der
///    Bewerber nie.
/// 2. Der erste verbleibende Traeger, mit dem der Bewerber nicht teilen darf,
///    wird genannt. Teilen duerfen zwei, wenn beide einen Wirkungsbereich haben
///    und der eine den anderen ausschliesst oder verengt; ohne Kommando gibt
///    es keinen Bereich und kein Teilen.
/// 3. Traegt die Kombination beim selben Zusteller schon zwei, wird der erste
///    genannt: hoechstens zwei, siehe den Modulkopf.
fn im_weg<'a>(traeger: &[&'a Funktion], bewerber: &Funktion) -> Option<&'a Funktion> {
    let gleiche: Vec<&'a Funktion> = traeger
        .iter()
        .copied()
        .filter(|funktion| funktion.gehalten_von == bewerber.gehalten_von)
        .collect();
    let duerfen_teilen =
        |eine: &Funktion| match (eine.wirkungsbereich(), bewerber.wirkungsbereich()) {
            (Some(eine), Some(andere)) => {
                eine.schliesst_aus(andere) || eine.verengt(andere) || andere.verengt(eine)
            }
            _ => false,
        };
    if let Some(unvertraeglich) = gleiche.iter().find(|funktion| !duerfen_teilen(funktion)) {
        return Some(unvertraeglich);
    }
    if gleiche.len() >= 2 {
        return gleiche.first().copied();
    }
    None
}

impl Default for Belegung {
    fn default() -> Self {
        Self::auslieferung()
    }
}

/// Laedt die Belegung des Nutzers aus `keymap.toml`.
///
/// Scheitert nie. Eine fehlende Datei ist der erste Start und liefert die
/// Auslieferungsbelegung ohne Meldung. Eine nicht lesbare, syntaktisch kaputte
/// oder inhaltlich widerspruechliche Datei liefert sie ebenfalls, dazu eine
/// [`Ersetzung`], die die Datei und den Grund nennt. Die Datei auf der Platte
/// bleibt in jedem Fall stehen; `keymap.toml` ist von Hand aenderbar, und ein
/// Tippfehler darin darf die Arbeit des Nutzers nicht loeschen.
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Belegung> {
    let roh: Geladen<Belegungsdatei> = zugang.laden(Datei::Belegung);
    match Belegung::vom_nutzer(&roh.wert) {
        Ok(belegung) => Geladen {
            wert: belegung,
            ersetzung: roh.ersetzung,
        },
        Err(fehler) => Geladen {
            wert: Belegung::auslieferung(),
            ersetzung: Some(Ersetzung {
                datei: zugang.pfad(Datei::Belegung),
                welche: Datei::Belegung,
                grund: Grund::Beschaedigt(fehler.to_string()),
                // Nichts zur Seite gelegt: die Datei war gueltiges TOML und ist
                // erst hier, eine Ebene hoeher, als widerspruechlich
                // aufgefallen. Das Zur-Seite-Legen wohnt in `Ablage::laden` und
                // sieht diesen Fall nicht; der Datensatz dazu ist
                // `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1204_*_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`.
                beiseite: Beiseite::Nicht,
            }),
        },
    }
}

/// Die Belegung fuer den laufenden Betrieb, dazu die Meldung, falls eine
/// noetig war.
///
/// Der eine Aufruf, den die Oberflaeche beim Start macht. Jede Meldung nimmt
/// [`melden`] und damit denselben Weg wie die der uebrigen Ablagedateien; eine
/// zweite Ausgabestelle entsteht nicht. Geschrieben wird sie hier nicht: der
/// Kern hat seit Schritt 12 keinen Ausgabekanal, und der Aufrufer in `krk-ui`
/// setzt den Satz in die Statuszeile.
#[must_use]
pub fn fuer_den_betrieb() -> (Belegung, Option<String>) {
    match Ablage::im_benutzerverzeichnis() {
        // **Der Durchgang umfasst hier nur das Laden, und das ist kein
        // Versehen**: schon `Zugang::laden` schreibt, wenn `keymap.toml`
        // beschaedigt ist und zur Seite gelegt wird. Scheitert das Nehmen der
        // Schreibsperre, faellt der Aufruf auf die Auslieferungsbelegung
        // zurueck und sagt es, statt ohne Sperre zu lesen.
        //
        // **Diese Ablage lebt nur bis zum Ende der Funktion.** Die bleibende
        // oeffnet die Oberflaeche danach; zwei Ablagen eines Prozesses duerfen
        // nicht zugleich einen Durchgang fahren, siehe den Kopf von
        // `ablage::sperre`.
        Ok(ablage) => match ablage.durchgang(|zugang| laden(zugang).mit_meldung()) {
            Ok(ergebnis) => ergebnis,
            Err(fehler) => (
                Belegung::auslieferung(),
                Some(melden(&Ersetzung {
                    datei: ablage.pfad(Datei::Belegung),
                    welche: Datei::Belegung,
                    grund: Grund::NichtLesbar(fehler.to_string()),
                    // Gelesen worden ist nichts, also gibt es nichts zu sichern.
                    beiseite: Beiseite::Nicht,
                })),
            ),
        },
        Err(fehler) => (
            Belegung::auslieferung(),
            Some(melden(&Ersetzung {
                datei: PathBuf::from(Datei::Belegung.dateiname()),
                welche: Datei::Belegung,
                grund: Grund::NichtLesbar(fehler.to_string()),
                // Ohne Ablageordner gibt es keinen Ort, an den etwas zur Seite
                // zu legen waere, und es ist auch nichts gelesen worden.
                beiseite: Beiseite::Nicht,
            })),
        ),
    }
}

/// Warum eine gelesene Datei keine Belegung ergibt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Belegungsfehler {
    /// Eine Kombination steht nicht in der vorgeschriebenen Schreibweise.
    Schreibweise {
        /// Die Funktion, bei der sie steht.
        kennung: String,
        /// Die Zeichenkette, wie sie in der Datei steht.
        text: String,
        /// Woran das Lesen scheiterte.
        fehler: Schreibfehler,
    },
    /// Die Datei nennt eine Funktion, die KRK nicht kennt.
    UnbekannteFunktion(String),
    /// Dieselbe Funktion steht zweimal.
    FunktionDoppelt(String),
    /// Zwei Funktionen beanspruchen dieselbe Kombination.
    Konflikt(Konflikt),
}

impl fmt::Display for Belegungsfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Belegungsfehler::Schreibweise {
                kennung,
                text,
                fehler,
            } => ausgabe.write_str(&satz(
                Text::BelegungSchreibweise,
                &[("kennung", kennung), ("text", text), ("fehler", fehler)],
            )),
            Belegungsfehler::UnbekannteFunktion(kennung) => ausgabe.write_str(&satz(
                Text::BelegungUnbekannteFunktion,
                &[("kennung", kennung)],
            )),
            Belegungsfehler::FunktionDoppelt(kennung) => ausgabe.write_str(&satz(
                Text::BelegungFunktionDoppelt,
                &[("kennung", kennung)],
            )),
            Belegungsfehler::Konflikt(konflikt) => konflikt.fmt(ausgabe),
        }
    }
}

impl std::error::Error for Belegungsfehler {}

/// Warum eine Zuweisung nicht zustande kam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zuweisungsfehler {
    /// Die Kombination gehoert bereits einer anderen Funktion.
    Konflikt(Konflikt),
    /// Die Belegung kennt keine Funktion dieser Kennung.
    UnbekannteFunktion(String),
}

impl fmt::Display for Zuweisungsfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Zuweisungsfehler::Konflikt(konflikt) => konflikt.fmt(ausgabe),
            Zuweisungsfehler::UnbekannteFunktion(kennung) => ausgabe.write_str(&satz(
                Text::BelegungUnbekannteFunktion,
                &[("kennung", kennung)],
            )),
        }
    }
}

impl std::error::Error for Zuweisungsfehler {}

/// Die Gestalt von `default-keymap.toml` und `keymap.toml`, unveraendert.
///
/// Der Zwischenschritt zwischen TOML und [`Belegung`]: hier stehen die
/// Kombinationen noch als Zeichenketten, und keine Regel ist geprueft. Erst
/// [`Belegung::vom_nutzer`] macht daraus eine Belegung.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Belegungsdatei {
    /// Ein Eintrag je Funktion, in der Reihenfolge der Datei.
    #[serde(default, rename = "funktion")]
    funktionen: Vec<Eintrag>,
}

impl Default for Belegungsdatei {
    /// Die eingebettete Auslieferungsbelegung.
    ///
    /// Damit liefert [`Zugang::laden`] bei fehlender oder kaputter
    /// `keymap.toml` den Auslieferungszustand und nicht eine leere Belegung, in
    /// der keine Taste mehr etwas tut.
    fn default() -> Self {
        Belegungsdatei::from(&Belegung::auslieferung())
    }
}

impl From<&Belegung> for Belegungsdatei {
    /// Der Rueckweg, und er traegt jedes Feld mit.
    ///
    /// Fehlte hier `gehalten_von`, schriebe [`Belegung::sichern`] eine
    /// `keymap.toml`, in der `text_alles_auswaehlen` keinen Zusteller mehr
    /// traegt; beim naechsten Start stuende `cmd+a` bei zwei Funktionen
    /// desselben Zustellers, das Einlesen meldete einen Konflikt, und der
    /// Nutzer haette eine Datei, die KRK selbst geschrieben und dann nicht mehr
    /// angenommen hat.
    ///
    /// `name` wird in der geltenden Sprache geschrieben, obwohl das Einlesen
    /// es nie liest: die Datei bleibt damit lesbar, wenn der Nutzer sie ueber
    /// „Tastaturdefinition oeffnen“ ansieht (Spec C3).
    fn from(belegung: &Belegung) -> Self {
        Self {
            funktionen: belegung
                .funktionen
                .iter()
                .map(|funktion| Eintrag {
                    id: funktion.kennung().to_owned(),
                    name: Some(funktion.name().to_owned()),
                    tasten: funktion
                        .tasten
                        .iter()
                        .map(|kombination| kombination.to_string())
                        .collect(),
                    reserviert_fuer: funktion.reserviert_fuer.clone(),
                    gehalten_von: funktion.gehalten_von.clone(),
                })
                .collect(),
        }
    }
}

/// Ein `[[funktion]]`-Block der Datei.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Eintrag {
    id: String,
    /// Die Beschriftung, wie sie die Datei fuehrt: **beim Lesen geduldet und
    /// nie gelesen**, beim Schreiben der Name in der geltenden Sprache.
    ///
    /// Bis zu dieser Arbeit war das Feld die Quelle des Namens und Pflicht;
    /// seit der Name aus der Sprachtabelle kommt, ist es optional, damit eine
    /// `keymap.toml` aus einer frueheren Fassung mit `name` ebenso laedt wie
    /// die Auslieferungsfassung ohne. Weggelassen statt leer geschrieben, aus
    /// dem Grund von `reserviert_fuer` darunter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    tasten: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reserviert_fuer: Option<String>,
    /// Wer den Tastendruck zustellt; siehe [`Funktion::gehalten_von`].
    ///
    /// Optional wie `reserviert_fuer` daneben, und aus demselben Grund
    /// weggelassen statt leer geschrieben: die weitaus meisten Funktionen
    /// tragen es nicht, und `deny_unknown_fields` oben laesst kein Feld durch,
    /// das der Parser nicht kennt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gehalten_von: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung() {
        let belegung = Belegung::auslieferung();
        for (kommando, kennung) in Kommando::KENNUNGEN {
            assert!(
                belegung.funktion(kennung).is_some(),
                "{kommando:?} nennt die Kennung {kennung}, die Auslieferungsbelegung kennt sie nicht"
            );
            assert_eq!(Kommando::aus_kennung(kennung), Some(kommando));
            assert_eq!(kommando.kennung(), kennung);
        }
    }

    /// Die zwei Zahlen im Kopf von `resources/default-keymap.toml` stimmen noch.
    ///
    /// Der Kopf der Datei sagt in einer Zeile, wie viele Funktionen und wie
    /// viele Kombinationen ausgeliefert werden. Die Zahlen stehen in einem
    /// Kommentar, ein Kommentar haelt keinen Bau an, und deshalb sind sie bisher
    /// still mit der Datei auseinandergelaufen, die sie beschreiben: zwei andere
    /// Kommentarstellen derselben Datei hatten es am 260810 schon getan
    /// (Defekte `260810-1217` und `260810-1218`). Der Defekt dazu ist
    /// `260810-1219`.
    ///
    /// # Wer diese Probe fehlschlagen sieht, zieht den Dateikopf nach
    ///
    /// Nachzuziehen ist die eine Zeile im Kopf von
    /// `resources/default-keymap.toml`, die mit `# Ausgeliefert sind` beginnt,
    /// und nichts hier. Gesucht wird sie an diesem Anfang und nicht an ihrer
    /// Zeilennummer: der Kopf der Datei waechst, und am 260810 hat die Zeile
    /// binnen eines Tages von 30 auf 33 gewechselt.
    ///
    /// Das ist die Absicht: **die Probe traegt keine eigene Zahl.** Sie liest
    /// beide aus dem Kommentar und zaehlt die Datei dagegen, und damit gibt es
    /// die Zaehlstaende weiterhin an genau einer Stelle.
    ///
    /// # Der Unterschied zur Nachbarin darunter
    ///
    /// `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren` hatte
    /// eine Vorgaengerin, die beide Zahlen als Literal im Quelltext trug; die
    /// musste weichen, weil ihr Fehlschlag nichts belegte — die Belegung war in
    /// Ordnung, allein die Zahl im Quelltext war alt. Hier ist der Fehlschlag
    /// selbst die Aussage, denn geprueft wird nicht die Groesse der Datei,
    /// sondern die Uebereinstimmung zweier Stellen **derselben** Datei.
    #[test]
    fn die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch() {
        let (funktionen_im_kopf, kombinationen_im_kopf) = zahlen_aus_dem_dateikopf();
        let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
            .expect("die eingebettete Auslieferungsbelegung ist gueltiges TOML");
        let kombinationen: usize = datei
            .funktionen
            .iter()
            .map(|eintrag| eintrag.tasten.len())
            .sum();

        assert_eq!(
            datei.funktionen.len(),
            funktionen_im_kopf,
            "der Kopf von resources/default-keymap.toml nennt {funktionen_im_kopf} Funktionen, \
             die Datei traegt {}; die Zeile \"# Ausgeliefert sind ...\" gehoert nachgezogen",
            datei.funktionen.len()
        );
        assert_eq!(
            kombinationen, kombinationen_im_kopf,
            "der Kopf von resources/default-keymap.toml nennt {kombinationen_im_kopf} \
             Kombinationen, die Datei traegt {kombinationen}; die Zeile \
             \"# Ausgeliefert sind ...\" gehoert nachgezogen"
        );
    }

    /// Die beiden Zaehlstaende aus der Kommentarzeile im Kopf der
    /// Auslieferungsbelegung, in der Reihenfolge Funktionen, Kombinationen.
    ///
    /// Gelesen wird aus [`AUSLIEFERUNGSTEXT`], also aus derselben eingebetteten
    /// Datei, aus der auch die Belegung entsteht; ein zweiter Dateizugriff im
    /// Pruefcode entsteht nicht.
    ///
    /// **Verschwindet die Zeile oder wechselt ihre Form, ist das ein Fehlschlag
    /// und kein uebersprungener Fall.** Eine Probe, die ihren Gegenstand nicht
    /// mehr findet und deshalb bejaht, wuerde genau die Luecke wieder aufmachen,
    /// die sie schliessen soll.
    fn zahlen_aus_dem_dateikopf() -> (usize, usize) {
        let anfang = "# Ausgeliefert sind ";
        let zeile = AUSLIEFERUNGSTEXT
            .lines()
            .find(|zeile| zeile.starts_with(anfang))
            .unwrap_or_else(|| {
                panic!(
                    "der Kopf von resources/default-keymap.toml hat keine Zeile, die mit \
                     {anfang:?} beginnt; sie nannte bis zum 260810 die Zahl der Funktionen und \
                     die der Kombinationen, und diese Probe haelt genau die beiden fest"
                )
            });
        let zahlen: Vec<usize> = zeile
            .split_whitespace()
            .filter_map(|wort| wort.parse().ok())
            .collect();
        assert_eq!(
            zahlen.len(),
            2,
            "die Zeile {zeile:?} nennt nicht genau zwei Zahlen; erwartet ist die Form \
             \"# Ausgeliefert sind <Funktionen> Funktionen mit zusammen <Kombinationen> \
             Kombinationen.\""
        );
        (zahlen[0], zahlen[1])
    }

    /// Was beim Bauen aus der Datei verschwinden koennte, verschwindet nicht.
    ///
    /// Die Vorgaengerin dieser Pruefung schrieb die Zahl der Funktionen und die
    /// der Kombinationen als Literal hin. Das prueft die Groesse der Datei und
    /// nicht die Arbeit von [`Belegung::bauen`]: jeder Nachtrag in
    /// `default-keymap.toml` liess sie fehlschlagen, ohne dass etwas kaputt war.
    /// Verglichen wird deshalb die gelesene Datei mit der gebauten Belegung.
    /// [`Belegung::bauen`] verwirft stillschweigend eine Kombination, die
    /// innerhalb derselben Funktion zweimal steht; genau das faellt hier auf.
    #[test]
    fn beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren() {
        let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
            .expect("die eingebettete Auslieferungsbelegung ist gueltiges TOML");
        let belegung = Belegung::auslieferung();

        assert!(
            !datei.funktionen.is_empty(),
            "die Auslieferungsbelegung nennt keine einzige Funktion"
        );
        assert_eq!(
            belegung.funktionen().len(),
            datei.funktionen.len(),
            "die gebaute Belegung fuehrt nicht so viele Funktionen wie die Datei"
        );

        let in_der_datei: usize = datei
            .funktionen
            .iter()
            .map(|eintrag| eintrag.tasten.len())
            .sum();
        let gebaut: usize = belegung
            .funktionen()
            .iter()
            .map(|funktion| funktion.tasten().len())
            .sum();
        assert_eq!(
            gebaut, in_der_datei,
            "eine Kombination der Auslieferungsbelegung steht doppelt und ist beim Bauen entfallen"
        );
    }

    #[test]
    fn eine_belegung_ueberlebt_schreiben_und_wiedereinlesen() {
        let belegung = Belegung::auslieferung();
        let text = toml::to_string(&Belegungsdatei::from(&belegung))
            .expect("die Belegung laesst sich schreiben");
        let wieder: Belegungsdatei = toml::from_str(&text).expect("und wieder einlesen");
        assert_eq!(
            Belegung::vom_nutzer(&wieder),
            Ok(belegung),
            "der Umweg ueber TOML hat die Belegung veraendert"
        );
    }
}
