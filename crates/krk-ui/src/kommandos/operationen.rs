//! Der Ablauf der Dateioperationen aus C4, ohne AppKit.
//!
//! **Am Fuss der Datei stehen daneben die Texte zweier Befehlsgruppen, die
//! keine Dateioperation sind.** Seit Schritt 18c die beiden Antworten des
//! Terminal-Befehls aus C11, seit dem 260811 die Form und die Saetze der drei
//! Befehle der Runde 4: [`pfadtext`], [`pfadzeilen`], [`kopiermeldung`],
//! [`nichts_zu_kopieren`] und [`ablage_weist_ab`] fuer die beiden Pfadkopierer
//! aus C1 und C2, [`nichts_zu_oeffnen`] und [`oeffnungsmeldung`] fuer die
//! Uebergabe an das Standardprogramm aus C3, und seit dem 260812
//! [`nichts_zu_teilen`] fuer das Teilen aus C1 der Runde 6, und seit der
//! Runde 17 [`nichts_zu_packen`], [`kein_archiv`], [`mehrere_archive`] und
//! [`kein_finder`] fuer die drei Eintraege des Kontextmenues, deren Regel in
//! [`super::kontextmenue`] steht, und seit der Runde 22 [`Dateiablage`],
//! [`namenszeilen`], [`ablagemeldung`] und [`verweise_abgewiesen`] fuer die
//! Dateiverweise, die `cmd+c` und `cmd+x` im Dateifenster ablegen. Die Texte der
//! Runde 4 tragen den Zuschnitt der Dateioperationen vollstaendig: ein Befehl,
//! der auf den sichtbaren Tab des aktiven Dateifensters wirkt und seine
//! Antwort als Befehlsantwort in die Statuszeile schreibt. **Das Teilen weicht
//! in der ersten Haelfte ab und nicht in der zweiten**: es wirkt aus jedem
//! Fokus und damit auch auf die angezeigte Datei, schreibt seine Antwort aber
//! in dieselbe Statuszeile wie jeder Befehl davor. Ein eigenes Modul fuer
//! diese Saetze waere ein sechstes unter [`crate::kommandos`] mit einer
//! einzigen Frage.
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die Ansichten dazu sind die vier Blaetter unter
//! `appkit/blaetter/`, die Zuleitung steht am Anwendungsdelegierten.
//!
//! ```text
//!  Tastenbefehl ──> Auswahl ──> Auftrag ──> krk_core::operation::starten
//!                                                     │
//!   Hauptfaden <── Weckruf <── Vermittlerfaden <── Meldung (Kanal)
//!        │                          │
//!        ├─ Statuszeile des         └─ Buendelung: verwirft den Weckruf,
//!        │  Quellfensters              solange der vorige nicht gezeichnet ist
//!        ├─ Konfliktblatt
//!        ├─ Loeschbestaetigung
//!        └─ Abschlussliste
//! ```
//!
//! # Die 150-ms-Regel
//!
//! C4 verlangt einen Fortschritt ab 100 Eintraegen oder 100 MB, L8 verlangt ihn
//! 200 ms nach dem Start sichtbar. Den Umfang eines Ordnerbaums vorher zu
//! bestimmen kostet einen eigenen Durchlauf, der die 200 ms selbst aufbrauchen
//! kann. Statt zweier Schwellen gilt deshalb eine Zeitspanne: die
//! Vorgangsanzeige erscheint, sobald die Operation [`ANZEIGEVERZUG`] gelaufen
//! ist (`### Frage 6` des Plans). Eine kleine Kopie ist vorher fertig und laesst
//! keine Zeile aufblitzen.
//!
//! **Der Verzug wird an einer Meldung gemessen und nicht an einem Zeitgeber.**
//! Die Zeile erscheint mit der ersten Meldung, die nach 150 ms eintrifft. Der
//! Arbeitsfaden meldet jeden fertigen Eintrag und, waehrend einer einzelnen
//! grossen Datei, alle 8 ms einen Zwischenstand; die Spanne zwischen dem
//! Ablauf der 150 ms und dem Erscheinen der Zeile ist damit im Regelfall
//! kleiner als eine Bildlaenge. Sie ist es **nicht** bei einer Operation, die
//! ueber Sekunden gar nichts meldet; der einzige solche Fall in dieser Runde
//! ist `NSFileManager.trashItemAtURL:` auf einem sehr grossen Ordner, der als
//! ein Eintrag zaehlt und erst am Ende meldet. Das ist der Preis der Wahl
//! "kein Takt" und im Bericht zu S16 ausgeschrieben.
//!
//! # Seit dem 260804-1832 traegt die Statuszeile den Fortschritt
//!
//! Bis S16 zeigte ihn ein Blatt am Fenster. Ein Blatt sperrt genau die
//! Oberflaeche, die C4 waehrend einer laufenden Operation bedienbar zusagt, und
//! braucht auf dem Referenzgeraet 354 bis 403 ms zum Aufgehen, waehrend L8 den
//! Fortschritt 200 ms nach Start sichtbar zusagt. Der Nutzer hat ihn in die
//! Statuszeile des Dateifensters verlegt, das die Operation begonnen hat
//! (`decisions/260804-1832_*_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`).
//! Zwei Folgen stehen in dieser Datei: [`vorgangszeile`] nennt den Abbruch im
//! eigenen Text, weil er seine Schaltflaeche verloren hat, und
//! [`waehrend_blatt_erlaubt`] gilt nur noch fuer ein stehendes Blatt.
//!
//! # Der Fokusvorbehalt der Loeschtasten ist mit Schritt 18 hier ausgezogen
//!
//! Er stand bis dahin in dieser Datei und galt allein fuer `delete` und `f8`.
//! Seit die Leiste aus C5 einen zweiten fokussierbaren Bereich stellt, ist die
//! Frage fuer jedes Kommando faellig und keine Frage der Dateioperationen mehr;
//! sie wohnt jetzt als eine Regel in [`crate::kommandos::fokus`], und die
//! Loeschtasten tragen sie ueber `Wirkungsbereich::Dateifenster` wie jeder
//! andere Befehl. Eine zweite Abfrage daneben gibt es nicht.
//!
//! # Die Buendelung ohne Takt
//!
//! Der Nutzer hat am 260804 Weg 3 aus
//! `issues/260803-2007_*_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`
//! gewaehlt: **gar kein Takt.** Der Arbeitsfaden meldet jeden Fortschritt, und
//! der Hauptfaden verwirft einen Weckruf, solange der vorige noch nicht
//! gezeichnet ist. Kein Zeitgeber, kein dritter Lebenszyklus neben Messlauf und
//! Anwendung, und kein Wecken des Prozesses im Leerlauf.
//!
//! Verworfen wird der **Weckruf** und nicht die Meldung: der Stand steht in
//! [`Vorgangszustand`], und der Hauptfaden liest beim naechsten Durchgang den
//! neuesten. Eine verworfene Meldung waere ein verlorener Zwischenstand, ein
//! verworfener Weckruf ist ein gesparter Zeichendurchgang.
//!
//! Die Reihenfolge auf dem Hauptfaden ist bindend und in
//! `die_buendelung_haelt_die_zahl_der_weckrufe_klein` festgehalten: **erst
//! [`Buendelung::gezeichnet`], dann den Stand lesen, dann zeichnen.** Umgekehrt
//! ginge eine Meldung verloren, die waehrend des Zeichnens eintrifft.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::SyncSender;
use std::time::{Duration, Instant};

use krk_core::operation::{
    Abbruchgriff, Abschluss, Art, Bericht, Fortschritt, Konfliktentscheid, Uebersprungen,
    name_pruefen,
};
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::Ordnermodell;

/// Wie lange eine Operation laufen muss, bevor die Vorgangsanzeige erscheint.
///
/// 150 ms lassen L8 ("Fortschritt in der Statuszeile sichtbar, 200 ms nach
/// Start") 50 ms Reserve. Eine Operation ueber 100 Eintraege oder 100 MB, die
/// C4 als Schwelle nennt, ist nach 150 ms nachweislich noch nicht fertig.
///
/// Der Name hiess bis zum 260804-1832 `BLATTVERZUG`. Er trug das Blatt in sich,
/// und das gibt es seit S16b nicht mehr.
pub const ANZEIGEVERZUG: Duration = Duration::from_millis(150);

/// Hoechstens so viele uebersprungene Eintraege stehen einzeln in der
/// Abschlussliste.
///
/// Eine Kopie ueber einen Ordner ohne Leserechte kann Tausende erzeugen; ein
/// Blatt, das den Bildschirm ueberragt, ist keine Auskunft mehr. Der Rest wird
/// gezaehlt.
const HOECHSTENS_EINZELN: usize = 12;

/// Ob die Vorgangsanzeige jetzt faellig ist.
pub fn anzeige_faellig(begonnen: Instant, jetzt: Instant) -> bool {
    jetzt.duration_since(begonnen) >= ANZEIGEVERZUG
}

// ----------------------------------------------------------------------
// Worauf ein Auftrag wirkt
// ----------------------------------------------------------------------

/// Die Eintraege, auf die ein Tastenbefehl wirkt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Auswahl {
    /// Die vollen Pfade, in der Reihenfolge, in der sie auf dem Schirm stehen.
    pub pfade: Vec<PathBuf>,
    /// Wie viele davon Ordner sind. Die Rueckfrage vor dem Raeumen in den
    /// Papierkorb nennt die Zahl gesondert (C4).
    pub ordner: usize,
}

impl Auswahl {
    /// Ob gar nichts betroffen ist.
    pub fn ist_leer(&self) -> bool {
        self.pfade.is_empty()
    }

    /// Wie viele Eintraege betroffen sind.
    pub fn zahl(&self) -> usize {
        self.pfade.len()
    }
}

/// Worauf ein Tastenbefehl im genannten Ordner wirkt (C4).
///
/// **Die Markierung hat den Vorrang, sonst gilt der Eintrag unter der
/// Auswahl.** Das ist die Regel jedes Zweifensterverwalters, und sie steht hier
/// einmal statt in jedem der vier Befehle. Gezaehlt werden allein die
/// *sichtbaren* Eintraege, in Sichtreihenfolge: eine Markierung, die der Nutzer
/// beim Druecken der Taste nicht vor sich hatte, gehoert nicht in den Auftrag.
pub fn betroffene(modell: &Ordnermodell, ordner: &Path) -> Auswahl {
    let mut auswahl = Auswahl::default();
    for zeile in 0..modell.zeilenzahl() {
        let Some(index) = modell.eintragsindex(zeile) else {
            continue;
        };
        if !modell.ist_markiert(index) {
            continue;
        }
        if let Some(eintrag) = modell.zeile(zeile) {
            auswahl.pfade.push(ordner.join(&eintrag.name));
            auswahl.ordner += usize::from(eintrag.ist_ordner());
        }
    }
    if !auswahl.ist_leer() {
        return auswahl;
    }

    let Some(zeile) = modell.auswahl_zeile() else {
        return auswahl;
    };
    if let Some(eintrag) = modell.zeile(zeile) {
        auswahl.pfade.push(ordner.join(&eintrag.name));
        auswahl.ordner += usize::from(eintrag.ist_ordner());
    }
    auswahl
}

/// Auf welche Zeile ein Rechtsklick die Auswahl setzt, **bevor**
/// [`betroffene`] gefragt wird (Nutzerentscheid vom 260812-1200).
///
/// `None` heisst: die Auswahl bleibt stehen. `Some(zeile)` heisst: der
/// Aufrufer setzt sie auf diese Zeile, und zwar auf demselben Weg wie ein
/// Tastenbefehl, damit die Vorschau davon erfaehrt.
///
/// **Eine zweite Auswahlregel entsteht hier nicht.** [`betroffene`] bleibt
/// unangetastet und beantwortet weiterhin allein, worauf ein Befehl wirkt;
/// geaendert wird die Auswahl vor ihr. Der Datensatz dieser Runde,
/// `decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`,
/// entscheidet genau so und lehnt die beiden anderen Moeglichkeiten
/// ausdruecklich ab: den Rechtsklick ohne jede Wirkung, weil ein Menue, das
/// auf A zeigt und auf B wirkt, bei einem spaeteren Eintrag mit
/// zerstoerender Wirkung der teuerste Fehler einer Oberflaeche ist; und das
/// Setzen ohne Ausnahme, weil es die Markierung des Nutzers wegnaehme.
///
/// **Die Ausnahme traegt die Antwort.** Ist die angeklickte Zeile markiert,
/// bleiben Auswahl und Markierung stehen: wer dreissig Eintraege markiert hat
/// und mit rechts auf einen davon klickt, verliert nichts. Ist sie es nicht,
/// rueckt die Auswahl auf sie, und der Klick zeigt auf dasselbe, worauf er
/// wirkt.
///
/// **Was die Ausnahme nicht deckt**, und der Preis gehoert genannt: ein Klick
/// auf eine **un**markierte Zeile, waehrend anderswo in der Liste etwas
/// markiert ist, rueckt die Auswahl zwar nach, aendert aber nichts am
/// Ergebnis, weil die Markierung in [`betroffene`] den Vorrang behaelt. Das
/// Aufheben der Markierung waere die abgelehnte dritte Moeglichkeit.
///
/// `angeklickt` ist der Wert von `NSTableView.clickedRow`. Drei Faelle fuehren
/// zu `None` und einer zu `Some`:
///
/// - **negativ** — der Klick fiel auf keine Zeile, also unter die letzte oder
///   auf die leere Flaeche der Liste. Er darf in keine Auswahl laufen.
/// - **ausserhalb der Sichtreihenfolge** — [`Ordnermodell::eintragsindex`]
///   findet keinen Eintrag, etwa in einer leeren Liste.
/// - **markiert** — die Ausnahme oben.
/// - sonst die angeklickte Zeile.
#[must_use]
pub fn rechtsklick_zielzeile(modell: &Ordnermodell, angeklickt: isize) -> Option<usize> {
    let zeile = usize::try_from(angeklickt).ok()?;
    let eintrag = modell.eintragsindex(zeile)?;
    if modell.ist_markiert(eintrag) {
        return None;
    }
    Some(zeile)
}

// ----------------------------------------------------------------------
// Was die Blattsperre selbst durchlaesst
// ----------------------------------------------------------------------

/// Was die Blattsperre selbst durchlaesst.
///
/// Genau der Abbruchbefehl.
///
/// **Das ist nicht dasselbe wie „was durchkommt, solange ein Blatt steht".**
/// Diese Regel ist einer von zwei Eingaengen; der zweite ist
/// `crate::kommandos::zulaessigkeit::immer_erreichbar`, das die Blattsperre
/// ausdruecklich mit aufhebt und drei weitere Befehle durchlaesst. Waehrend
/// eines Blattes kommen also **vier** Kommandos durch und nicht eines, und
/// welche vier, schreibt
/// `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` aus.
/// Bis zum 260818 stand die Zusammenfassung hier in der verkuerzten Form, und
/// vier weitere Stellen im Baum haben sie von hier uebernommen
/// (`issues/260817-1302_*_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-*.md`,
/// `issues/260817-1419_*_ein-vierter-traeger-der-verkuerzten-blattsperre-*.md`).
///
/// Alles uebrige geht unveraendert an AppKit weiter, damit das Blatt seine
/// eigene Tastaturbedienung behaelt und der Abgriff kein Ereignis ins Leere
/// schluckt.
///
/// **Ohne diese Sperre waere kein Blatt mit mehr als einer Antwort bedienbar.**
/// Der Tabulator liegt in `resources/default-keymap.toml` auf
/// `fenster_wechseln`; der Abgriff schluckte ihn und wechselte hinter dem Blatt
/// das aktive Dateifenster, statt den Fokus im Blatt weiterzuruecken. Ebenso
/// loeste die Taste Delete vor der stehenden Rueckfrage eine Loeschung in dem
/// Ordner dahinter aus.
///
/// **Eine laufende Operation zaehlt seit S16b nicht mehr dazu.** Bis dahin
/// sperrte dieselbe Regel auch, solange ein Vorgang lief, weil er ein Blatt
/// zeigte. Der Fortschritt steht jetzt in der Statuszeile, es gibt nichts mehr
/// zu sperren, und die Bedienbarkeit waehrend der Operation ist genau die
/// Zusage aus C4, um die es geht. Der Name bleibt und trifft ab hier genau:
/// die Regel gilt fuer ein stehendes Blatt.
pub fn waehrend_blatt_erlaubt(kommando: Kommando) -> bool {
    kommando == Kommando::Abbrechen
}

// ----------------------------------------------------------------------
// Die Buendelung ohne Takt
// ----------------------------------------------------------------------

/// Die Buendelung der Fortschrittsmeldungen, ohne Zeitgeber.
///
/// Siehe den Modulkopf. `melden` sagt dem Vermittlerfaden, ob er den
/// Hauptfaden wecken soll; `gezeichnet` meldet vom Hauptfaden zurueck, dass der
/// naechste Weckruf wieder gebraucht wird.
#[derive(Debug, Default)]
pub struct Buendelung {
    /// Wahr, solange ein Weckruf unterwegs oder unbearbeitet ist.
    offen: AtomicBool,
}

impl Buendelung {
    /// Eine Buendelung ohne ausstehenden Weckruf.
    pub const fn neu() -> Self {
        Self {
            offen: AtomicBool::new(false),
        }
    }

    /// Meldet einen Fortschritt. Liefert, ob der Hauptfaden zu wecken ist.
    ///
    /// Der Tausch ist atomar, damit zwei Arbeitsfaeden nicht beide `false`
    /// lesen und beide wecken. KRK laeuft heute mit einem, und die Richtigkeit
    /// haengt nicht daran.
    pub fn melden(&self) -> bool {
        !self.offen.swap(true, Ordering::AcqRel)
    }

    /// Der Hauptfaden hat den Stand uebernommen.
    ///
    /// **Vor** dem Lesen des Standes zu rufen, nicht danach: sonst faellt eine
    /// Meldung, die waehrend des Zeichnens eintrifft, zwischen die beiden
    /// Schritte und wird nie gezeichnet.
    pub fn gezeichnet(&self) {
        self.offen.store(false, Ordering::Release);
    }

    /// Ob gerade ein Weckruf aussteht. Nur zum Ablesen und fuer die Pruefung.
    #[cfg(test)]
    pub fn steht_aus(&self) -> bool {
        self.offen.load(Ordering::Acquire)
    }
}

// ----------------------------------------------------------------------
// Der Zustand, den beide Faeden teilen
// ----------------------------------------------------------------------

/// Ein Namenskonflikt, der auf die Antwort des Nutzers wartet (C4).
pub struct Konfliktfrage {
    /// Der Eintrag, der uebertragen werden soll.
    pub quelle: PathBuf,
    /// Der Eintrag, der schon da ist.
    pub ziel: PathBuf,
    /// Der Weg zurueck zum Arbeitsfaden.
    pub antwort: SyncSender<Konfliktentscheid>,
}

/// Was der Hauptfaden anzeigt, gefuellt vom Vermittlerfaden.
#[derive(Default)]
pub struct Anzeigestand {
    /// Der neueste Zwischenstand.
    pub fortschritt: Option<Fortschritt>,
    /// Die uebersprungenen Eintraege, in der Reihenfolge ihres Auftretens.
    pub uebersprungen: Vec<Uebersprungen>,
    /// Eine unbeantwortete Konfliktfrage.
    pub konflikt: Option<Konfliktfrage>,
    /// Der Abschlussbericht. Kommt genau einmal und zuletzt.
    pub bericht: Option<Bericht>,
}

/// Der Zustand eines laufenden Vorgangs, geteilt zwischen Haupt- und
/// Vermittlerfaden.
pub struct Vorgangszustand {
    /// Die Buendelung der Weckrufe.
    pub buendelung: Buendelung,
    /// Der Griff an das Abbruchkennzeichen des Laufs.
    ///
    /// **Das Kennzeichen des Laufs selbst und keine Kopie davon.** Bis zum
    /// 260805 stand hier ein zweiter `AtomicBool`: der Hauptfaden setzte ihn,
    /// und der Vermittlerfaden reichte ihn an den Lauf weiter, sobald die
    /// naechste Meldung ihn aufweckte. Bei einer Operation, die ueber Sekunden
    /// nichts meldet, wirkte der Abbruch entsprechend spaet
    /// (`issues/260804-1816_*_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md`).
    /// Seit `Lauf::abbruchgriff` das Kennzeichen herausgibt, setzt der
    /// Hauptfaden es unmittelbar, und der Arbeitsfaden liest es beim naechsten
    /// Eintrag oder im Statusrueckruf von `copyfile(3)`.
    abbruch: Abbruchgriff,
    stand: Mutex<Anzeigestand>,
}

impl Vorgangszustand {
    /// Ein leerer Zustand zu einem eben gestarteten Lauf.
    pub fn neu(abbruch: Abbruchgriff) -> Self {
        Self {
            buendelung: Buendelung::neu(),
            abbruch,
            stand: Mutex::new(Anzeigestand::default()),
        }
    }

    /// Der Nutzer hat abgebrochen.
    pub fn abbrechen(&self) {
        self.abbruch.abbrechen();
    }

    /// Reicht eine Aenderung an den Anzeigestand durch.
    ///
    /// Ein vergifteter Mutex wird uebergangen: er entstuende nur aus einem
    /// Absturz auf dem anderen Faden, und ein zweiter Absturz hier machte den
    /// ersten nicht besser.
    pub fn aendern<T>(&self, aendern: impl FnOnce(&mut Anzeigestand) -> T) -> T {
        let mut stand = match self.stand.lock() {
            Ok(stand) => stand,
            Err(vergiftet) => vergiftet.into_inner(),
        };
        aendern(&mut stand)
    }
}

// ----------------------------------------------------------------------
// Die Texte, die der Nutzer liest
// ----------------------------------------------------------------------

/// Wie die Zeile ihre Angaben trennt.
///
/// Ein Mittelpunkt statt eines Zeilenumbruchs: die Statuszeile ist einzeilig
/// (`NSTextField::setMaximumNumberOfLines(1)`), und ein Umbruch waere dort
/// abgeschnitten statt gelesen.
const TRENNER: &str = " · ";

/// Was die Zeile ueber den Abbruch sagt.
///
/// Er hat mit dem Blatt seine Schaltflaeche verloren und liegt weiter auf
/// `esc` (`resources/default-keymap.toml`, Kennung `abbrechen`). Damit der
/// Nutzer ihn findet, nennt die Zeile ihn; das ist die Antwort auf den
/// Einwand, eine Zeile am Fuss sei leichter zu uebersehen als ein Blatt.
const ABBRUCHHINWEIS: &str = "Esc bricht ab";

/// Womit eine Operation in der Zeile benannt wird.
fn ueberschrift(art: &Art) -> &'static str {
    match art {
        Art::Kopieren { .. } => "Kopieren",
        Art::Verschieben { .. } => "Verschieben",
        Art::InDenPapierkorb => "In den Papierkorb räumen",
        Art::UmbenennenImStapel { .. } => "Umbenennen",
        Art::Zippen { .. } => "Packen",
        Art::Entpacken { .. } => "Entpacken",
    }
}

/// Ob der Vorgang genau **eine** Zieldatei erzeugt.
///
/// Die zweite vollstaendige Rechnung ueber [`Art`], und sie steht neben der
/// ersten: wer einen siebten Wert hinzufuegt, findet beide Stellen an einem
/// Fleck statt an zweien.
///
/// **Gefragt wird sie vom Konfliktblatt**, das in dieser Lage drei Antworten
/// statt vier zeigt und das Ankreuzfeld „fuer alle weiteren" weglaesst: es
/// haette keinen Gegenstand, und „Ueberspringen" fiele mit „Abbrechen"
/// zusammen, weil der Vorgang danach ohnehin endet. So hat der Nutzer es am
/// 260824-2120 gewaehlt
/// (`decisions/260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`,
/// Moeglichkeit 2).
///
/// Die sechs Werte und ihre Antwort:
///
/// | Art | genau ein Ziel | warum |
/// |---|---|---|
/// | Kopieren | nein | je Quelle ein Ziel |
/// | Verschieben | nein | ebenso |
/// | InDenPapierkorb | nein | kein Ziel, und kein Konflikt |
/// | UmbenennenImStapel | nein | je Quelle ein neuer Name |
/// | Zippen | **ja** | ein Archiv fuer den ganzen Lauf |
/// | Entpacken | wenn genau ein Archiv | je Archiv ein Zielordner |
///
/// **Das Entpacken haengt an einer Zahl und nicht am Wert**, und das ist die
/// Folge der dritten Nutzerentscheidung dieser Runde: seit ein Vorgang mehrere
/// Archive tragen kann, wird der Zielordner-Konflikt je Archiv gefragt, und
/// dann traegt das Ankreuzfeld wieder seinen Gegenstand. Bei genau einem Archiv
/// traegt es ihn nicht, und die Antwort ist dieselbe wie beim Packen.
///
/// `#[must_use]`: der Rueckgabewert entscheidet, welche Gestalt das Blatt
/// annimmt; fiele er still weg, stuende das Blatt in der vierantwortigen
/// Gestalt da, ohne dass eine Probe es saehe.
///
/// Gefragt wird sie an genau einer Stelle, naemlich in
/// `Anwendungsdelegierter::konflikt_fragen`, und die reicht die Antwort an
/// `crate::appkit::blaetter::konflikt::zeigen` weiter. Das Blatt selbst kennt
/// die [`Art`] nicht und soll sie nicht kennenlernen.
#[must_use]
pub fn erzeugt_genau_ein_ziel(art: &Art) -> bool {
    match art {
        Art::Kopieren { .. }
        | Art::Verschieben { .. }
        | Art::InDenPapierkorb
        | Art::UmbenennenImStapel { .. } => false,
        Art::Zippen { .. } => true,
        Art::Entpacken { ziele } => ziele.len() == 1,
    }
}

/// Die Zeile, die waehrend eines laufenden Vorgangs in der Statuszeile steht.
///
/// Sie nennt vier Dinge: was laeuft, wie weit es ist, worauf es wirkt und wie
/// man es abbricht. Der Abbruchhinweis steht am Ende und nicht am Anfang, weil
/// der Stand die Angabe ist, auf die der Nutzer wartet.
///
/// **Zwei Zahlen, weil eine von beiden je nach Fall nichts sagt.** Die Zahl der
/// Eintraege ist die des Kerns: was die Operation angefasst hat. Beim
/// Verschieben innerhalb eines Datentraegers ist das genau ein Eintrag je
/// Position, weil `rename(2)` den Inhalt nie beruehrt; beim Kopieren desselben
/// Ordners sind es Hunderte. Die Zahl der Positionen daneben ist ueber alle
/// Faelle dieselbe. Der Widerspruch ist als Defekt
/// `issues/260804-1649_*_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren.md`
/// festgehalten; dieser Schritt entscheidet ihn nicht, er zeigt beide Zahlen
/// nebeneinander und benennt sie.
pub fn vorgangszeile(art: &Art, fortschritt: Option<&Fortschritt>, positionen: usize) -> String {
    let was = ueberschrift(art);
    let Some(fortschritt) = fortschritt else {
        return format!(
            "{was} wird vorbereitet: {}{TRENNER}{ABBRUCHHINWEIS}",
            positionen_text(positionen)
        );
    };
    let name = fortschritt
        .eintrag
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    format!(
        "{was}: {}, {}, {}{TRENNER}{name}{TRENNER}{ABBRUCHHINWEIS}",
        eintraege_text(fortschritt.eintraege as usize),
        menge(fortschritt.bytes),
        positionen_text(positionen),
    )
}

/// Die Zeile, sobald der Nutzer den Abbruch angefordert hat.
///
/// Der Vorgang laeuft bis zu seinem Bericht weiter; die Zeile sagt das, statt
/// stehen zu bleiben, als waere nichts geschehen. Der Abbruchhinweis faellt
/// weg, weil er beantwortet ist.
pub fn abbruchzeile(art: &Art) -> String {
    format!(
        "{} wird abgebrochen, der Vorgang endet gleich …",
        ueberschrift(art)
    )
}

/// Die Meldung auf einen zweiten Operationsbefehl waehrend eines laufenden
/// Vorgangs (C4).
///
/// KRK haelt genau einen Vorgang. Eine Warteschlange waere die andere Antwort;
/// sie baut einen Zustand mehr, den keine Zusage verlangt.
pub fn schon_ein_vorgang(art: &Art) -> String {
    format!("es läuft bereits eine Operation: {}", ueberschrift(art))
}

/// Die Meldung, die nach dem Ende eines Vorgangs in der Statuszeile steht.
///
/// Nach einem Abbruch nennt sie, wie viele Eintraege bereits uebertragen wurden
/// (C4).
///
/// **`ausgelassen` sind Eintraege, die gar nicht erst in den Auftrag kamen**
/// (Runde 17), und deshalb steht ihre Zahl **hinter** der der uebersprungenen:
/// uebersprungen hat der Vorgang, was er angefasst und liegengelassen hat,
/// ausgelassen ist, was ihm nie vorlag. Zip und Unzip nehmen einen markierten
/// Eintrag aus dem Lauf, wenn derselbe Lauf ihn als Ziel anlegt
/// (`crate::kommandos::kontextmenue`); ohne diesen Halbsatz taete der Befehl
/// wortlos weniger, als der Nutzer markiert hat
/// (`issues/260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-*`).
/// Jeder andere Weg reicht hier null herein; welche und warum, steht bei
/// `Vorgang::ausgelassen`.
pub fn abschlusstext(
    art: &Art,
    bericht: &Bericht,
    positionen: usize,
    ausgelassen: usize,
) -> String {
    let was = ueberschrift(art);
    let uebertragen = format!(
        "{}, {} ({})",
        eintraege_text(bericht.eintraege as usize),
        menge(bericht.bytes),
        positionen_text(positionen)
    );
    let mut text = match bericht.abschluss {
        Abschluss::Abgebrochen => format!("{was} abgebrochen: {uebertragen} übertragen"),
        Abschluss::Fertig => format!("{was} fertig: {uebertragen}"),
    };
    if !bericht.uebersprungen.is_empty() {
        text.push_str(&format!(
            ", {} übersprungen",
            eintraege_text(bericht.uebersprungen.len())
        ));
    }
    if ausgelassen > 0 {
        text.push_str(&format!(
            ", {} als Ziel dieses Laufs ausgelassen",
            eintraege_text(ausgelassen)
        ));
    }
    text
}

/// Die Abschlussliste der uebersprungenen Eintraege mit ihrem Grund (C4).
///
/// `None`, wenn nichts uebersprungen wurde: dann gibt es kein Blatt.
pub fn uebersprungenliste(uebersprungen: &[Uebersprungen]) -> Option<(String, String)> {
    if uebersprungen.is_empty() {
        return None;
    }
    let frage = match uebersprungen.len() {
        1 => "Ein Eintrag wurde übersprungen".to_owned(),
        zahl => format!("{} Einträge wurden übersprungen", self::zahl(zahl)),
    };
    let mut zeilen: Vec<String> = uebersprungen
        .iter()
        .take(HOECHSTENS_EINZELN)
        .map(|eintrag| {
            let name = eintrag.pfad.file_name().map_or_else(
                || eintrag.pfad.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            );
            format!("{name}: {}", eintrag.grund)
        })
        .collect();
    if uebersprungen.len() > HOECHSTENS_EINZELN {
        zeilen.push(format!(
            "… und {} weitere",
            zahl(uebersprungen.len() - HOECHSTENS_EINZELN)
        ));
    }
    Some((frage, zeilen.join("\n")))
}

// ----------------------------------------------------------------------
// Anlegen und Umbenennen im Stapel (C4, Schritt 17)
// ----------------------------------------------------------------------

/// Was einer der beiden Anlegebefehle anlegt (C4).
///
/// Eine Aufzaehlung und keine zwei Befehlswege: die Frage an den Nutzer, die
/// Namenspruefung und die Auswahl auf dem neuen Eintrag sind fuer Ordner und
/// Datei dieselben. Verschieden ist allein die Kernfunktion, die den Eintrag
/// anlegt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anlegeart {
    /// Ein Ordner (`f7`, `shift+cmd+n`).
    Ordner,
    /// Eine leere Datei (`ctrl+cmd+n`).
    Datei,
}

impl Anlegeart {
    /// Die Frage in der Kopfzeile des Eingabeblattes.
    pub fn frage(self) -> &'static str {
        match self {
            Anlegeart::Ordner => "Wie soll der neue Ordner heißen?",
            Anlegeart::Datei => "Wie soll die neue Datei heißen?",
        }
    }

    /// Die Beschriftung der bestaetigenden Schaltflaeche.
    pub fn bestaetigen(self) -> &'static str {
        "Anlegen"
    }

    /// Wie eine Meldung den angelegten Eintrag benennt.
    pub fn benennung(self) -> &'static str {
        match self {
            Anlegeart::Ordner => "Ordner",
            Anlegeart::Datei => "Datei",
        }
    }
}

/// Die Meldung, wenn ein Eintrag angelegt wurde (C4).
pub fn angelegt_text(art: Anlegeart, name: &str) -> String {
    format!("{} „{name}“ angelegt", art.benennung())
}

/// Die Meldung, wenn ein Eintrag nicht angelegt werden konnte (C4).
///
/// Der haeufigste Fall ist der bereits vergebene Name, und er bekommt deshalb
/// einen eigenen Satz statt des Systemwortlauts. Die uebrigen behalten ihn: eine
/// erfundene Uebersetzung waere ungenauer als das Original. Dieselbe Abwaegung
/// wie in `krk_core::operation::grund`, die hier nicht wiederverwendet werden
/// kann, weil sie kistenintern ist und weil sie den Namen des Eintrags nicht
/// nennt.
pub fn anlegefehler(art: Anlegeart, name: &str, fehler: &io::Error) -> String {
    match fehler.kind() {
        io::ErrorKind::AlreadyExists => schon_vergeben(name),
        io::ErrorKind::PermissionDenied => format!(
            "keine Rechte, hier {} „{name}“ anzulegen",
            match art {
                Anlegeart::Ordner => "den Ordner",
                Anlegeart::Datei => "die Datei",
            }
        ),
        _ => format!("„{name}“ ließ sich nicht anlegen: {fehler}"),
    }
}

/// Der Satz zum bereits vergebenen Namen, fuer das Anlegen und das Umbenennen.
///
/// Beide Befehle scheitern am selben Zustand des Ordners und sagen deshalb
/// denselben Satz. Zwei Formulierungen dafuer waeren zwei Erklaerungen fuer
/// dieselbe Lage.
fn schon_vergeben(name: &str) -> String {
    format!("es gibt schon einen Eintrag namens „{name}“")
}

// ----------------------------------------------------------------------
// Umbenennen eines einzelnen Eintrags in der Liste (C4, Schritt 17b)
// ----------------------------------------------------------------------

/// Was aus dem Namen wird, den der Nutzer in die Namenszelle geschrieben hat.
///
/// Drei Ausgaenge und kein `Result`: das Gleichbleiben ist weder ein Erfolg
/// noch ein Fehler, sondern der haeufigste Ausgang ueberhaupt. Wer die Zelle
/// oeffnet und sie mit Return wieder schliesst, hat nichts umbenannt und will
/// darueber auch nichts lesen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Umbenennungswunsch {
    /// Der Name ist derselbe geblieben. Nichts tun, nichts melden.
    Unveraendert,
    /// Ein zulaessiger neuer Name, getrimmt.
    Neu(String),
    /// Der Name taugt nicht; der Grund steht im Klartext dabei.
    Abgelehnt(&'static str),
}

/// Prueft, was der Nutzer in die Namenszelle geschrieben hat (C4).
///
/// Getrimmt wird aus demselben Grund wie in der Namenseingabe des Anlegens:
/// fuehrende und schliessende Leerzeichen sind so gut wie immer ein Versehen,
/// und ein Eintrag, den man von seinem Nachbarn nicht unterscheiden kann, ist
/// keine Hilfe.
///
/// **Ob der Name schon vergeben ist, prueft diese Funktion nicht.** Das
/// beantwortet das Dateisystem beim Umbenennen selbst, und zwar in dem
/// Augenblick, in dem es zaehlt; eine Vorabprueferei gegen die gelesene Liste
/// waere eine zweite Wahrheit ueber denselben Ordner und ginge zwischen Lesen
/// und Umbenennen ohnehin ins Leere. Den Satz dazu liefert
/// [`umbenennungsfehler`].
pub fn umbenennung_pruefen(alt: &str, eingabe: &str) -> Umbenennungswunsch {
    let neu = eingabe.trim();
    if neu == alt {
        return Umbenennungswunsch::Unveraendert;
    }
    match name_pruefen(neu) {
        Ok(()) => Umbenennungswunsch::Neu(neu.to_owned()),
        Err(fehler) => Umbenennungswunsch::Abgelehnt(fehler.grund()),
    }
}

/// Die Meldung, wenn ein Eintrag nicht umbenannt werden konnte (C4).
///
/// Genannt wird der **neue** Name: der alte steht dem Nutzer noch in der Liste,
/// der neue ist der, an dem es lag. Der bereits vergebene Name bekommt denselben
/// Satz wie beim Anlegen; die uebrigen behalten den Systemwortlaut, dieselbe
/// Abwaegung wie in [`anlegefehler`].
pub fn umbenennungsfehler(neuer_name: &str, fehler: &io::Error) -> String {
    match fehler.kind() {
        io::ErrorKind::AlreadyExists => schon_vergeben(neuer_name),
        io::ErrorKind::PermissionDenied => {
            format!("keine Rechte, hier in „{neuer_name}“ umzubenennen")
        }
        _ => format!("„{neuer_name}“ ließ sich nicht vergeben: {fehler}"),
    }
}

/// Die Meldung nach einem ausgefuehrten Stapel-Umbenennen (C4).
///
/// "ein Eintrag" beziehungsweise "4.812 Einträge".
fn eintraege_text(eintraege: usize) -> String {
    match eintraege {
        1 => "ein Eintrag".to_owned(),
        zahl => format!("{} Einträge", self::zahl(zahl)),
    }
}

/// "3 Positionen" beziehungsweise "eine Position".
fn positionen_text(positionen: usize) -> String {
    match positionen {
        1 => "eine ausgewählte Position".to_owned(),
        zahl => format!("{} ausgewählte Positionen", self::zahl(zahl)),
    }
}

/// "ein Ordner" beziehungsweise "3 Ordner".
///
/// Auch der Markierungsstand aus S16c nennt die Ordner gesondert und benutzt
/// dieselbe Wendung; die Loeschfrage hat sie zuerst gebraucht, deshalb steht
/// sie hier und nicht in [`super::auswahl`]. Seit der Runde 12 nimmt sie
/// [`super::loeschwarnung`] mit, dem die Loeschfrage gehoert; die Wendung
/// bleibt trotzdem hier, denn sie ist ein Zahlwort und kein Loeschtext.
pub(crate) fn ordner_text(ordner: usize) -> String {
    match ordner {
        1 => "ein Ordner".to_owned(),
        zahl => format!("{} Ordner", self::zahl(zahl)),
    }
}

/// Eine Zahl mit Punkten als Tausendertrennung, wie sie der Nutzer liest.
///
/// Die eine Schreibweise fuer Zahlen in der Oberflaeche. [`super::auswahl`]
/// nimmt sie fuer den Markierungsstand mit, damit eine markierte Liste
/// dieselben Punkte zeigt wie ein laufender Vorgang; seit der Runde 12 auch
/// [`super::loeschwarnung`] fuer die Zahl der Eintraege in der Loeschfrage.
///
/// **`pub(crate)` und nicht `pub(super)`**, und das ist kein Versehen: der
/// dritte Aufrufer ist `crate::appkit::statuszeile` und liegt ausserhalb von
/// [`super`]. Die enge Sichtbarkeit haelt der Uebersetzer dort nicht ein.
pub(crate) fn zahl(wert: usize) -> String {
    let ziffern = wert.to_string();
    let mut aus = String::with_capacity(ziffern.len() + ziffern.len() / 3);
    for (stelle, ziffer) in ziffern.chars().enumerate() {
        if stelle > 0 && (ziffern.len() - stelle).is_multiple_of(3) {
            aus.push('.');
        }
        aus.push(ziffer);
    }
    aus
}

/// Eine Datenmenge in der Schreibweise, die der Nutzer im Blatt liest.
///
/// Dezimalpraefixe, wie der Finder sie zeigt. Die Tabelle im Dateifenster
/// formatiert ueber `NSByteCountFormatter` und bleibt dabei; sie beschriftet
/// eine Zelle fester Breite, und diese Zeile beschriftet einen Satz. Zwei
/// Aufrufer, zwei Anforderungen, und diese hier soll ohne AppKit pruefbar sein.
fn menge(bytes: u64) -> String {
    const EINHEITEN: [(u64, &str); 4] = [
        (1_000_000_000_000, "TB"),
        (1_000_000_000, "GB"),
        (1_000_000, "MB"),
        (1_000, "kB"),
    ];
    for (teiler, name) in EINHEITEN {
        if bytes >= teiler {
            let ganze = bytes / teiler;
            let zehntel = (bytes % teiler) * 10 / teiler;
            return format!("{ganze},{zehntel} {name}");
        }
    }
    format!("{bytes} Bytes")
}

// ----------------------------------------------------------------------
// Der angezeigte Ordner an einer benannten Anwendung (C11, Schritt 18c)
// ----------------------------------------------------------------------

/// Ob der Ordner noch da ist, den ein Befehl uebergeben soll.
///
/// `None`, wenn er sich uebergeben laesst; sonst der Satz fuer die Statuszeile.
/// Der Fall ist der ausgeworfene Datentraeger: der sichtbare Tab traegt den
/// Pfad noch, den Ordner gibt es nicht mehr.
///
/// **Der Name nennt das Terminal nicht mehr, und das ist der Punkt.** Der
/// Rumpf war seit C11 allgemein und die zwei Saetze sind es auch; gefragt wird
/// von jedem Befehl, der einen Ordner an eine ueber ihre Buendelkennung
/// benannte Anwendung uebergibt. Der Terminal-Befehl aus C11 stellt die Frage,
/// der Finder-Eintrag des Kontextmenues aus der Runde 17 stellt dieselbe. Ein
/// Aufruf mit "terminal" im Namen aus dem Finder-Zweig waere die
/// Doppelbenennung, die dieses Projekt vermeidet.
///
/// **Geprueft wird vor dem Aufruf und nicht nach ihm.** Der Rueckruf von
/// `openURLs:…` bleibt leer (siehe `crate::appkit::terminal`), also ist dies
/// die eine Gelegenheit, dem Nutzer etwas zu sagen, das er beheben kann.
///
/// Nicht ueber [`super::pfadeingabe::pruefen`]: jene Funktion beantwortet
/// "wohin geht KRK", und ihre Antwort traegt ein Sprungziel samt Auswahl, das
/// dieser Befehl nicht braucht und nicht auswerten wuerde. Sie verlangte zudem
/// das Leserecht, das weder eine Terminal-Sitzung noch ein Finder-Fenster in
/// einem Ordner braucht.
pub fn ordner_fehlt(ordner: &Path) -> Option<String> {
    match std::fs::metadata(ordner) {
        Ok(angaben) if angaben.is_dir() => None,
        Ok(_) => Some(format!("{} ist kein Ordner mehr", ordner.display())),
        Err(fehler) => Some(format!(
            "{} ist nicht mehr erreichbar: {fehler}",
            ordner.display()
        )),
    }
}

/// Die Meldung auf eine Kennung, zu der keine Anwendung installiert ist (C11).
///
/// **Sie nennt die eingestellte Kennung**, denn das ist die Angabe, mit der der
/// Nutzer `settings.toml` berichtigen kann. Auf die Vorbelegung weicht KRK
/// nicht aus: dann oeffnete sich ein Terminal, das er nicht gewaehlt hat, und
/// sein Tippfehler bliebe unbemerkt in der Datei stehen.
///
/// **Sie nennt daneben den Neustart**, denn `settings.toml` wird einmal beim
/// Start gelesen (Entscheid des Nutzers vom 260807,
/// `decisions/260805-1845_*_wann-eine-von-hand-geaenderte-settings-toml-wirkt.md`).
/// Ohne den Halbsatz fuehrt die Meldung das fuenfte Abnahmekriterium von C11
/// nicht zu Ende: der Nutzer behebt den Tippfehler, drueckt erneut Ctrl+O und
/// bekommt dieselbe Meldung, ohne dass etwas auf den fehlenden Neustart deutet.
/// Ein zweiter Lesepfad entsteht daraus ausdruecklich nicht.
pub fn kein_terminal(kennung: &str) -> String {
    format!(
        "keine Anwendung mit der Bündelkennung „{kennung}“ installiert; \
         settings.toml nennt sie unter terminal, eine Änderung wirkt erst \
         nach einem Neustart"
    )
}

// ----------------------------------------------------------------------
// Die Pfade in der Zwischenablage (C1 und C2 der Runde 4)
// ----------------------------------------------------------------------

/// Ein Pfad in der Form, die C1 zusagt.
///
/// Ausgeschrieben und absolut, mit einem abschliessenden Trenner allein beim
/// Wurzelverzeichnis, dessen Pfad aus ihm besteht.
///
/// **Die Form entsteht hier und nicht an der Quelle.**
/// [`super::pfadeingabe::pruefen`] uebernimmt den eingegebenen Text woertlich,
/// also traegt der Ordner eines Tabs den abschliessenden Schraegstrich, den der
/// Nutzer getippt hat. Ihn dort abzuschneiden aenderte die Identitaet des
/// angezeigten Ordners, an der `gleicher_ordner`, die Dateisystembeobachtung
/// und die Lesezeichen haengen, und loeste ein Formproblem an einer Stelle, die
/// keine Form zusagt.
///
/// **Aufgeloest wird nichts.** `canonicalize` kommt nicht vor: C1 verlangt, dass
/// ein zwischenzeitlich verschwundener Ordner trotzdem kopiert wird, und ein
/// Aufruf, der das Dateisystem fragt, braeche das Kriterium und die Zusage,
/// dass der Befehl kopiert, was auf dem Schirm steht. Ebenso wenig gerufen wird
/// die Anzeigenfunktion aus `krk_core::ablage::pfade`, die aus dem
/// Benutzerverzeichnis eine Tilde macht: eine Tilde gehoert nicht in die
/// Zwischenablage, und die Meldung nennt denselben Text, der abgelegt wurde.
pub fn pfadtext(pfad: &Path) -> String {
    let text = pfad.display().to_string();
    let ohne_trenner = text.trim_end_matches(std::path::MAIN_SEPARATOR);
    if ohne_trenner.is_empty() {
        // Der Pfad bestand aus Trennern; das ist das Wurzelverzeichnis, und
        // sein Pfad ist der Trenner selbst.
        return std::path::MAIN_SEPARATOR.to_string();
    }
    ohne_trenner.to_owned()
}

/// Mehrere Pfade als ein Text, einen je Zeile (C2).
///
/// Getrennt durch `\n`, **ohne** Schlusszeilenumbruch: die letzte Zeile endet
/// mit dem Pfad, damit ein Einfuegen in ein Terminal nicht von sich aus die
/// Eingabetaste mitbringt. Die Reihenfolge ist die der uebergebenen Pfade, also
/// die Sichtreihenfolge aus [`betroffene`].
pub fn pfadzeilen(pfade: &[PathBuf]) -> String {
    pfade
        .iter()
        .map(|pfad| pfadtext(pfad))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Die Meldung nach einem geglueckten Kopieren (C1, C2).
///
/// Bei einem Pfad nennt sie den Pfad, bei mehreren ihre Zahl. Der genannte Pfad
/// ist derselbe Text, der in der Zwischenablage steht; ein gekuerzter zeigte
/// etwas anderes an, als der Nutzer gleich einfuegt.
///
/// Eine leere Menge erreicht diese Funktion nicht: beide Aufrufer fangen sie
/// vorher mit [`nichts_zu_kopieren`] ab, weil dann auch nichts geschrieben wird.
pub fn kopiermeldung(pfade: &[PathBuf]) -> String {
    match pfade {
        [einziger] => format!("Pfad kopiert: {}", pfadtext(einziger)),
        mehrere => format!("{} Pfade kopiert", mehrere.len()),
    }
}

/// Der Satz, wenn beim Kopierer kein Eintrag betroffen ist (C2).
///
/// C2 verlangt den Wortlaut: die Statuszeile sagt, **dass nichts zu kopieren
/// war**, und nicht bloss, wie die Lage ist. Der Satz nennt deshalb die Folge
/// zuerst und die Lage danach.
///
/// **Seit der Runde 22 hat der Satz einen weiteren Rufer**, die Dateiablage
/// ueber `cmd+c` und `cmd+x` (`DateifensterQuelle::dateiverweise_ablegen`):
/// sie wirkt auf dieselbe Menge wie die zwei Pfadkopierer, naemlich auf
/// [`betroffene`], und findet auf dieselbe Weise nichts. Ein eigener Satz
/// daneben saehe wie eine andere Lage aus (C1.7 der Runde 22).
pub fn nichts_zu_kopieren() -> String {
    nichts_betroffen("kopieren")
}

/// Der Satz, wenn beim Oeffner kein Eintrag betroffen ist (C3).
///
/// Dasselbe fuer die andere Folge. C3 verlangt keinen Wortlaut fuer die leere
/// Menge; der Satz folgt trotzdem dem des Kopierers, weil derselbe Anlass zwei
/// verschieden gebaute Saetze sonst wie zwei verschiedene Lagen aussaehe.
pub fn nichts_zu_oeffnen() -> String {
    nichts_betroffen("öffnen")
}

/// Der Satz, wenn beim Packen kein Eintrag betroffen ist (Runde 17).
///
/// **Der dritte Eingang von [`nichts_betroffen`], und er steht neben seinen
/// zwei Geschwistern und nicht bei den Archivsaetzen darunter.** Zip wirkt auf
/// dieselbe Menge wie die beiden Befehle darueber, naemlich auf
/// [`betroffene`], und findet auf dieselbe Weise nichts: nichts markiert und
/// nichts ausgewaehlt. Ein eigener Satz daneben saehe wie eine andere Lage aus.
///
/// **Der Entwurf dieser Runde haette bei leerer Markierung den ganzen
/// angezeigten Ordner gepackt**; der Nutzer hat das verworfen und die
/// bestehende Regel gewaehlt. Dieser Satz ist die Stelle, an der die Wahl
/// sichtbar wird: statt einer ungefragten Handlung steht eine Auskunft.
/// **Ein Rufer**, der Zip-Zweig des Kontextmenues
/// (`Anwendungsdelegierter::zipauftrag_stellen`).
#[must_use]
pub fn nichts_zu_packen() -> String {
    nichts_betroffen("packen")
}

/// Der Satz, wenn beim Teilen nichts zu uebergeben ist (C1 der Runde 6).
///
/// **Er geht nicht durch [`nichts_betroffen`], und das ist der Unterschied
/// zwischen einem Befehl und dreien.** Die beiden Saetze darueber gelten je
/// einem Befehl im Dateifenster, und "nichts markiert und nichts ausgewaehlt"
/// ist dort die Lage. Das Teilen wirkt aus jedem Fokus und findet auf drei
/// verschiedene Weisen nichts: kein betroffener Eintrag im Dateifenster, keine
/// angezeigte Datei in Vorschau und Editor, und in der Leiste nichts, was ein
/// Freigabedienst annaehme. Ein Satz, der von Markierung und Auswahl spraeche,
/// waere in zwei der drei Lagen falsch — in der Leiste hat der Nutzer ein
/// Lesezeichen ausgewaehlt vor sich und laese, es sei nichts ausgewaehlt.
///
/// Der Satz nennt deshalb das **Ergebnis** und keine Ursache, wie es der
/// Ordnersprung aus C2 derselben Runde tut. Er stimmt in allen drei Lagen und
/// bleibt einzeilig.
pub fn nichts_zu_teilen() -> String {
    "nichts zu teilen: hier steht nichts, was an die Freigabedienste ginge".to_owned()
}

/// Der Satz, wenn Unzip kein Archiv vorfindet (Runde 17).
///
/// **Er geht nicht durch [`nichts_betroffen`]**, und der Grund ist derselbe,
/// aus dem [`nichts_zu_teilen`] es nicht tut: „nichts markiert und nichts
/// ausgewaehlt" waere hier falsch. Unzip findet auch dann nichts, wenn der
/// Nutzer eine Datei ausgewaehlt hat — sie ist nur kein Archiv, und der Ordner
/// traegt keines.
///
/// **Er nennt die Endung**, denn sie ist die ganze Regel: KRK erkennt ein
/// Archiv am Namen und nicht am Inhalt
/// (`decisions/260825-0711_*_woran-erkennt-unzip-dass-eine-datei-ein-zip-ist.md`,
/// Moeglichkeit 1). Ohne den Halbsatz suchte der Nutzer den Grund an der
/// falschen Stelle, wenn sein Archiv anders heisst.
/// **Ein Rufer**, der Unzip-Zweig des Kontextmenues
/// (`Anwendungsdelegierter::entpackauftrag_stellen`), auf den Befund
/// [`super::kontextmenue::Entpackbefund::Keines`].
#[must_use]
pub fn kein_archiv() -> String {
    "nichts zu entpacken: hier steht keine Datei mit der Endung .zip".to_owned()
}

/// Der Satz, wenn die Ersatzregel von Unzip mehrere Archive vorfindet
/// (Runde 17).
///
/// Der zweite Fehlbefund neben [`kein_archiv`]. Er gilt **allein der
/// Ersatzregel**: mehrere **betroffene** Archive sind kein Fehlbefund, sondern
/// der Regelfall seit dem Nutzerentscheid vom 260824-2120, und sie werden alle
/// entpackt.
///
/// **Er sagt, was zu tun ist, ohne den Nutzer anzureden**, wie es
/// [`kein_terminal`] mit seinem Hinweis auf `settings.toml` vormacht: die
/// Auskunft, dass die Auswahl auf keines der Archive zeigt, ist zugleich der
/// Weg heraus.
/// **Ein Rufer**, derselbe Zweig wie bei [`kein_archiv`], auf den Befund
/// [`super::kontextmenue::Entpackbefund::Mehrere`].
#[must_use]
pub fn mehrere_archive() -> String {
    "nichts zu entpacken: hier stehen mehrere Archive, und die Auswahl zeigt \
     auf keines"
        .to_owned()
}

/// Der Satz, wenn das System keinen Finder nennt (Runde 17).
///
/// **Er nennt die Buendelkennung nicht**, anders als [`kein_terminal`]. Dort
/// ist sie die Angabe, mit der der Nutzer `settings.toml` berichtigen kann;
/// hier steht sie fest im Baum, und ein Nutzer, der `com.apple.finder` liest,
/// haette nichts, was er damit anfangen koennte. Sie stuende dann zudem an zwei
/// Stellen, hier und am Aufruf.
///
/// **Der Fall ist selten und wird trotzdem gemeldet**, aus demselben Grund wie
/// bei [`ablage_weist_ab`]: ein Befehl, der still nichts tut, sieht aus wie
/// einer, der nicht angekommen ist.
/// **Ein Rufer**, der Finder-Zweig des Kontextmenues
/// (`Anwendungsdelegierter::im_finder_zeigen`), wenn
/// [`crate::appkit::terminal::ordner_oeffnen`] `false` liefert.
#[must_use]
pub fn kein_finder() -> String {
    "der Finder ist nicht erreichbar: das System hat keine Anwendung dafür genannt".to_owned()
}

/// Die gemeinsame Haelfte der beiden Saetze darueber.
///
/// **Zwei Eingaenge und ein Rumpf, und die Aufteilung hat einen Grund.** Bis
/// zum 260811 war es ein einziger Satz ohne Verb, gemeinsam fuer beide
/// Befehle; er nannte die Lage und traf damit den Wortlaut von C2 nicht.
/// Getrennt wird nur, was sich zwischen den Befehlen unterscheidet, naemlich
/// das Verb; die Lage dahinter steht weiter an einer Stelle.
///
/// Sie sagt **nicht** "der Ordner ist leer": eine leere Menge entsteht auch in
/// einem vollen Ordner, naemlich waehrend eines Lesevorgangs, nachdem
/// `Ordnermodell::ersatz_einloesen` Markierung und Auswahl geleert hat und
/// bevor die Auswahl wieder steht.
fn nichts_betroffen(verb: &str) -> String {
    format!("nichts zu {verb}: nichts markiert und nichts ausgewählt")
}

/// Die Meldung, wenn die Zwischenablage den Text nicht annimmt (C1, C2).
///
/// Der Fall ist selten und wird trotzdem gemeldet: `setString:forType:` liefert
/// ein `bool`, und ein Kopierer, der still nichts kopiert haette, liesse den
/// Nutzer einen alten Inhalt einfuegen, den er fuer den neuen haelt.
///
/// **Sie nennt den Text und nicht den Pfad**, und das ist keine Wortklauberei:
/// `ordnerpfad_kopieren` legt einen Pfad ab, `eintragspfad_kopieren` bei
/// dreissig markierten Eintraegen dreissig, und beide legen dabei **einen**
/// Text ab, denn genau einen nimmt `setString:forType:`. Der Satz braucht so
/// keine Fallunterscheidung nach der Zahl, waehrend [`kopiermeldung`] und
/// [`oeffnungsmeldung`] sie tragen: die beiden melden, was abgelegt wurde,
/// dieser meldet, dass die Ablage selbst nicht stattgefunden hat.
pub fn ablage_weist_ab() -> String {
    "die Zwischenablage hat den Text nicht angenommen".to_owned()
}

// ----------------------------------------------------------------------
// Die Dateiverweise in der Zwischenablage (Runde 22)
// ----------------------------------------------------------------------

/// Die zwei Befehle, die im Dateifenster Dateiverweise ablegen (Runde 22).
///
/// `cmd+c` und `cmd+x`, die `copy:`- und die `cut:`-Haelfte des
/// Einhaengepunkts, den Belegung und Menue "Bearbeiten" seit dem 260805
/// freihalten. **Beide legen dasselbe ab, und Ausschneiden verschiebt
/// nichts** (A4 des Specs): `NSPasteboard` traegt keine Sorte, die
/// "ausgeschnitten" bedeutet, KRK erfaehrt nie, ob das Ziel nach dem Einfuegen
/// die Quelle entfernt, und die Dateizelle bekommt keinen dritten Zustand. Der
/// ganze Unterschied zwischen den zwei Werten ist ein Satz in der Statuszeile,
/// den [`ablagemeldung`] anhaengt: das Verschieben liegt beim Ziel.
///
/// Kein `Kommando`: die zwei Befehle haben keine Zeile in der Belegung, sie
/// kommen als Aktionsselektoren beim Anwendungsdelegierten an. Der `match`
/// in [`ablagemeldung`] ist ueber die zwei Werte vollstaendig, damit ein
/// dritter den Bau anhaelt statt still den Satz des ersten zu bekommen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dateiablage {
    /// `cmd+c`, "Bearbeiten › Kopieren".
    Kopieren,
    /// `cmd+x`, "Bearbeiten › Ausschneiden": dasselbe Ablegen, ein Satz mehr.
    Ausschneiden,
}

/// Die Namen der abgelegten Eintraege als ein Text, einer je Zeile (C2 der
/// Runde 22).
///
/// Die Schwester von [`pfadzeilen`]: `\n`-getrennt, **ohne** Schlusszeilen-
/// umbruch, in der Reihenfolge der uebergebenen Pfade, also der
/// Sichtreihenfolge aus [`betroffene`]. Ein Ordner steht ohne abschliessenden
/// Trenner, weil [`eintragsname`] ihn nicht mitfuehrt.
///
/// **Es ist der Name und nicht der Pfad, und das ist keine Sparsamkeit** (A3):
/// der Pfad ist die Textsorte von `shift+cmd+c`, und zwei Befehle mit
/// derselben Textsorte waeren einer zu viel. Der Finder legt beim Kopieren
/// einer Datei ebenfalls ihren Namen als Text daneben. Die Huelle um die
/// Zwischenablage deutet nicht und bekommt diesen Text fertig herein; so ist
/// der Name in der Ablage derselbe, den [`ablagemeldung`] in der Statuszeile
/// nennt.
pub fn namenszeilen(pfade: &[PathBuf]) -> String {
    pfade
        .iter()
        .map(|pfad| eintragsname(pfad))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Die Meldung nach einem geglueckten Ablegen der Dateiverweise (A6 der
/// Runde 22).
///
/// Bei einem Eintrag nennt sie seinen Namen, bei mehreren ihre Zahl (C1.8);
/// der Name ist derselbe, der ueber [`namenszeilen`] in der Ablage steht. Nach
/// [`Dateiablage::Ausschneiden`] haengt derselbe Satz den Zusatz an, dass das
/// Verschieben beim Ziel liegt (C3.2): KRK verschiebt nichts (A4), und der
/// Nutzer erfaehrt an dieser Stelle, wo er es bekommt.
///
/// Eine leere Menge erreicht diese Funktion nicht: der Rufer faengt sie vorher
/// mit [`nichts_zu_kopieren`] ab, weil dann auch nichts geschrieben wird.
#[must_use]
pub fn ablagemeldung(befehl: Dateiablage, pfade: &[PathBuf]) -> String {
    let kopiert = match pfade {
        [einziger] => format!("kopiert: {}", eintragsname(einziger)),
        mehrere => format!("{} Einträge kopiert", zahl(mehrere.len())),
    };
    match befehl {
        Dateiablage::Kopieren => kopiert,
        Dateiablage::Ausschneiden => {
            format!("{kopiert} – verschieben tut das Ziel (Finder: opt+cmd+v)")
        }
    }
}

/// Die Meldung, wenn die Zwischenablage die Dateiverweise nicht annimmt (A6,
/// A12 der Runde 22).
///
/// Nach dem Muster von [`ablage_weist_ab`]: `writeObjects:` und
/// `setString:forType:` liefern je ein `bool`, und ein Befehl, der still
/// nichts abgelegt haette, liesse den Nutzer einen alten Inhalt einfuegen, den
/// er fuer den neuen haelt.
///
/// **Sie nennt die Eintraege und nicht den Text**, denn abgelegt werden hier
/// Verweise, und der Text daneben ist die Beigabe. Wie ihr Vorbild braucht sie
/// keine Fallunterscheidung nach der Zahl: sie meldet, dass die Ablage nicht
/// stattgefunden hat, und nicht, was abgelegt wurde.
#[must_use]
pub fn verweise_abgewiesen() -> String {
    "die Zwischenablage hat die Einträge nicht angenommen".to_owned()
}

// ----------------------------------------------------------------------
// Die Uebergabe an das Standardprogramm (C3 der Runde 4)
// ----------------------------------------------------------------------

/// Der Name eines Eintrags fuer eine Meldung, ersatzweise sein Pfad.
///
/// Ein Pfad ohne letzten Bestandteil ist das Wurzelverzeichnis oder endet auf
/// `..`; dann steht der Pfad in der Meldung, denn ein leerer Name benennte
/// nichts.
fn eintragsname(pfad: &Path) -> String {
    match pfad.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => pfadtext(pfad),
    }
}

/// Die Meldung nach der Uebergabe an das Standardprogramm (C3).
///
/// **Sie sagt "an das System uebergeben" und nicht "geoeffnet", und das ist
/// keine Umstaendlichkeit.** `NSWorkspace::openURL:` meldet synchron allein, ob
/// das System die Adresse angenommen hat; ob das aufgeloeste Programm danach
/// startet, erfuehre KRK nur ueber einen Rueckruf, den es nicht fuehrt (siehe
/// [`crate::appkit::standardprogramm`]). Die Meldung behauptet deshalb genau
/// das, was die Rueckgabe hergibt, und keinen Schritt mehr.
///
/// Bei einem Eintrag nennt sie seinen **Namen** und nicht seinen Pfad — anders
/// als [`kopiermeldung`], und aus einem Grund: der Kopierer meldet, was in der
/// Zwischenablage steht und gleich eingefuegt wird, der Oeffner meldet, womit
/// der Nutzer gerade etwas getan hat. Bei mehreren nennt sie ihre Zahl.
///
/// Der abgewiesene Teil haengt hinten an, sofern es einen gibt; **warum** ein
/// Eintrag abgewiesen wurde, steht nicht darin, denn `openURL:` nennt es nicht.
/// Sind beide Mengen leer, faellt die Meldung auf [`nichts_zu_oeffnen`] zurueck;
/// der Aufrufer faengt den Fall schon vorher ab, weil dann auch nichts
/// uebergeben wird.
pub fn oeffnungsmeldung(uebergeben: &[PathBuf], abgewiesen: &[PathBuf]) -> String {
    let angenommen = match uebergeben {
        [] => None,
        [einziger] => Some(format!(
            "an das System übergeben: {}",
            eintragsname(einziger)
        )),
        mehrere => Some(format!(
            "{} Einträge an das System übergeben",
            mehrere.len()
        )),
    };
    let zurueck = match abgewiesen {
        [] => None,
        [einziger] => Some(format!(
            "das System hat {} nicht angenommen",
            eintragsname(einziger)
        )),
        mehrere => Some(format!(
            "das System hat {} von {} Einträgen nicht angenommen",
            mehrere.len(),
            uebergeben.len() + mehrere.len()
        )),
    };
    match (angenommen, zurueck) {
        (Some(genommen), Some(abgelehnt)) => format!("{genommen}; {abgelehnt}"),
        (Some(genommen), None) => genommen,
        (None, Some(abgelehnt)) => abgelehnt,
        (None, None) => nichts_zu_oeffnen(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    use krk_core::verzeichnis::{Eintrag, Typ};

    use crate::pruefordner::Pruefordner;

    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag::mit_versteckt(
            name.to_owned(),
            0,
            std::time::SystemTime::UNIX_EPOCH,
            typ,
            false,
        )
    }

    fn modell_mit(namen: &[(&str, Typ)]) -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen(namen.iter().map(|(name, typ)| eintrag(name, *typ)));
        modell.abschliessen();
        modell
    }

    #[test]
    fn ohne_markierung_wirkt_der_befehl_auf_den_eintrag_unter_der_auswahl() {
        let mut modell = modell_mit(&[("a.txt", Typ::Datei), ("b.txt", Typ::Datei)]);
        let index = modell.index_von_namen("b.txt").expect("b.txt steht da");
        modell.auswahl_setzen(Some(index));

        let auswahl = betroffene(&modell, Path::new("/tmp/x"));
        assert_eq!(auswahl.pfade, [PathBuf::from("/tmp/x/b.txt")]);
        assert_eq!(auswahl.ordner, 0);
    }

    #[test]
    fn die_markierung_hat_vorrang_vor_der_auswahl() {
        let mut modell = modell_mit(&[
            ("a.txt", Typ::Datei),
            ("unten", Typ::Ordner),
            ("c.txt", Typ::Datei),
        ]);
        let auswahl_index = modell.index_von_namen("c.txt").expect("c.txt steht da");
        modell.auswahl_setzen(Some(auswahl_index));
        for name in ["a.txt", "unten"] {
            let index = modell.index_von_namen(name).expect("steht da");
            modell.markierung_umschalten(index);
        }

        let auswahl = betroffene(&modell, Path::new("/tmp/x"));
        // Sichtreihenfolge, nicht Lesereihenfolge: die Vorgabesortierung
        // stellt Ordner vor Dateien.
        assert_eq!(
            auswahl.pfade,
            [PathBuf::from("/tmp/x/unten"), PathBuf::from("/tmp/x/a.txt")],
            "die markierten Eintraege, in Sichtreihenfolge"
        );
        assert_eq!(auswahl.ordner, 1, "der Ordner wird gesondert gezaehlt");
    }

    #[test]
    fn ohne_auswahl_und_ohne_markierung_ist_nichts_betroffen() {
        let modell = modell_mit(&[("a.txt", Typ::Datei)]);
        assert!(betroffene(&modell, Path::new("/tmp/x")).ist_leer());
    }

    // ------------------------------------------------------------------
    // Die Auswahl vor dem Rechtsklick (Nutzerentscheid vom 260812-1200)
    // ------------------------------------------------------------------

    /// Der Regelfall: die angeklickte Zeile ist nicht markiert, die Auswahl
    /// rueckt auf sie.
    #[test]
    fn der_rechtsklick_setzt_die_auswahl_auf_die_angeklickte_zeile() {
        let mut modell = modell_mit(&[("a.txt", Typ::Datei), ("b.txt", Typ::Datei)]);
        let index = modell.index_von_namen("a.txt").expect("a.txt steht da");
        modell.auswahl_setzen(Some(index));

        assert_eq!(
            rechtsklick_zielzeile(&modell, 1),
            Some(1),
            "b.txt steht in Zeile 1 und ist nicht markiert"
        );
    }

    /// Die tragende Ausnahme: auf einer markierten Zeile bleibt alles stehen.
    ///
    /// Sie ist der Grund, aus dem die dritte Moeglichkeit des Datensatzes
    /// abgelehnt ist. Geprueft wird sie mit einer Markierung ueber mehrere
    /// Zeilen, weil genau das der Fall ist, den der Nutzer verlieren wuerde.
    #[test]
    fn auf_einer_markierten_zeile_bewegt_der_rechtsklick_nichts() {
        let mut modell = modell_mit(&[
            ("a.txt", Typ::Datei),
            ("b.txt", Typ::Datei),
            ("c.txt", Typ::Datei),
        ]);
        for name in ["a.txt", "b.txt", "c.txt"] {
            let index = modell.index_von_namen(name).expect("steht da");
            modell.markierung_umschalten(index);
        }

        assert_eq!(rechtsklick_zielzeile(&modell, 0), None);
        assert_eq!(rechtsklick_zielzeile(&modell, 1), None);
        assert_eq!(rechtsklick_zielzeile(&modell, 2), None);
    }

    /// Eine Markierung anderswo haelt den Rechtsklick auf einer unmarkierten
    /// Zeile nicht auf.
    ///
    /// Die Ausnahme fragt nach der angeklickten Zeile und nicht danach, ob
    /// ueberhaupt etwas markiert ist. Was danach betroffen ist, entscheidet
    /// [`betroffene`] unveraendert weiter, und dort behaelt die Markierung
    /// den Vorrang.
    #[test]
    fn eine_markierung_anderswo_haelt_den_rechtsklick_nicht_auf() {
        let mut modell = modell_mit(&[("a.txt", Typ::Datei), ("b.txt", Typ::Datei)]);
        let index = modell.index_von_namen("a.txt").expect("a.txt steht da");
        modell.markierung_umschalten(index);

        assert_eq!(rechtsklick_zielzeile(&modell, 1), Some(1));
    }

    /// `clickedRow` liefert -1, wenn der Klick auf keine Zeile fiel.
    #[test]
    fn ein_klick_auf_keine_zeile_setzt_keine_auswahl() {
        let modell = modell_mit(&[("a.txt", Typ::Datei)]);
        assert_eq!(rechtsklick_zielzeile(&modell, -1), None);
    }

    /// Eine Zeilennummer jenseits der Liste faellt in dieselbe Antwort.
    #[test]
    fn eine_zeile_jenseits_der_liste_setzt_keine_auswahl() {
        let modell = modell_mit(&[("a.txt", Typ::Datei)]);
        assert_eq!(rechtsklick_zielzeile(&modell, 1), None);
        assert_eq!(
            rechtsklick_zielzeile(&modell_mit(&[]), 0),
            None,
            "die leere Liste hat keine Zeile 0"
        );
    }

    /// Der Beleg zur Wahl "kein Takt": viele Meldungen, wenige Weckrufe.
    ///
    /// Die Prueflast ist die des namentlichen Abnahmepunkts von S16: eine Kopie
    /// von 5.000 Eintraegen meldet 5.000-mal. Gezeichnet wird dazwischen
    /// zehnmal, und genau zehn Weckrufe duerfen es sein — je einer nach jedem
    /// Zeichendurchgang, und der erste davor.
    #[test]
    fn die_buendelung_haelt_die_zahl_der_weckrufe_klein() {
        let buendelung = Buendelung::neu();
        let mut weckrufe = 0;
        for meldung in 0..5_000 {
            if buendelung.melden() {
                weckrufe += 1;
            }
            // Alle 500 Meldungen kommt der Hauptfaden dazwischen und zeichnet.
            if meldung % 500 == 499 {
                buendelung.gezeichnet();
            }
        }
        assert_eq!(
            weckrufe, 10,
            "5.000 Meldungen bei zehn Zeichendurchgaengen ergeben zehn Weckrufe"
        );
    }

    /// Ohne einen Zeichendurchgang bleibt es bei einem einzigen Weckruf.
    #[test]
    fn ohne_zeichendurchgang_weckt_nur_die_erste_meldung() {
        let buendelung = Buendelung::neu();
        let geweckt = (0..100_000).filter(|_| buendelung.melden()).count();
        assert_eq!(geweckt, 1);
        assert!(buendelung.steht_aus());
    }

    /// Dieselbe Rechnung ueber die Fadengrenze, wie sie im Betrieb laeuft.
    ///
    /// Geteilt wird die [`Buendelung`] und nicht der ganze [`Vorgangszustand`]
    /// darum. Im Betrieb steckt sie in ihm, und er steckt in einem `Arc`, den
    /// sich Haupt- und Vermittlerfaden teilen; gerechnet wird aber allein in
    /// ihr. Seit der Zustand den Abbruchgriff eines wirklichen Laufs traegt,
    /// braeuchte diese Pruefung sonst einen Lauf, den sie nicht startet, oder
    /// einen Griff ohne Lauf, den es zu bauen nicht wert waere.
    #[test]
    fn auch_ueber_die_fadengrenze_weckt_nicht_jede_meldung() {
        let buendelung = Arc::new(Buendelung::neu());
        let arbeiter = Arc::clone(&buendelung);
        let melder = thread::spawn(move || (0..20_000).filter(|_| arbeiter.melden()).count());
        let weckrufe = melder.join().expect("der Melderfaden ist gescheitert");
        assert!(
            weckrufe <= 20_000,
            "eine Obergrenze, die immer haelt; die Aussage steht darunter"
        );
        assert_eq!(
            weckrufe, 1,
            "ohne einen einzigen Zeichendurchgang bleibt es bei einem Weckruf"
        );
    }

    #[test]
    fn die_vorgangsanzeige_erscheint_erst_nach_150_ms() {
        let begonnen = Instant::now();
        assert!(!anzeige_faellig(begonnen, begonnen));
        assert!(!anzeige_faellig(
            begonnen,
            begonnen + Duration::from_millis(149)
        ));
        assert!(anzeige_faellig(begonnen, begonnen + ANZEIGEVERZUG));
    }

    /// Die Zeile nennt den Abbruch, weil er seine Schaltflaeche verloren hat.
    #[test]
    fn die_vorgangszeile_nennt_den_abbruch_und_bleibt_einzeilig() {
        let art = Art::Kopieren {
            ziel: "/tmp".into(),
        };
        let vorbereitung = vorgangszeile(&art, None, 3);
        assert!(vorbereitung.contains("Esc bricht ab"), "{vorbereitung}");
        assert!(vorbereitung.starts_with("Kopieren"), "{vorbereitung}");

        let zeile = vorgangszeile(
            &art,
            Some(&Fortschritt {
                eintraege: 4_812,
                bytes: 1_200_000_000,
                eintrag: PathBuf::from("/tmp/quelle/beispiel.txt"),
            }),
            3,
        );
        assert!(zeile.contains("4.812"), "{zeile}");
        assert!(zeile.contains("1,2 GB"), "{zeile}");
        assert!(zeile.contains("3 ausgewählte Positionen"), "{zeile}");
        assert!(zeile.contains("beispiel.txt"), "{zeile}");
        assert!(zeile.contains("Esc bricht ab"), "{zeile}");
        for text in [&vorbereitung, &zeile] {
            assert!(
                !text.contains('\n'),
                "die Statuszeile ist einzeilig: {text}"
            );
        }
    }

    #[test]
    fn ein_zweiter_befehl_meldet_den_laufenden_vorgang() {
        let text = schon_ein_vorgang(&Art::InDenPapierkorb);
        assert!(text.contains("bereits"), "{text}");
        assert!(text.contains("In den Papierkorb räumen"), "{text}");
    }

    /// Die Sperre gilt fuer ein stehendes Blatt und nicht mehr fuer einen
    /// laufenden Vorgang; der zweite Fall wohnt in `kommando_ausfuehren`.
    ///
    /// **Der Name sagt „die Blattsperre" und nicht „ein stehendes Blatt".**
    /// Gemessen wird diese eine Regel; was insgesamt durchkommt, solange ein
    /// Blatt steht, sind vier Kommandos, und das misst
    /// `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch`.
    #[test]
    fn die_blattsperre_laesst_allein_den_abbruch_durch() {
        assert!(waehrend_blatt_erlaubt(Kommando::Abbrechen));
        assert!(!waehrend_blatt_erlaubt(Kommando::InPapierkorb));
        assert!(!waehrend_blatt_erlaubt(Kommando::AuswahlRunter));
        assert!(
            !waehrend_blatt_erlaubt(Kommando::FensterWechseln),
            "der Tabulator gehoert dem Blatt, sonst ist es ohne Maus nicht zu beantworten"
        );
    }

    /// Die eine Ausnahme bleibt die eine, und der Notizzettelbefehl steht
    /// ausdruecklich nicht darin.
    ///
    /// **Der Durchgang geht ueber alle Kommandos**, damit die Aussage „genau
    /// der Abbruch, und zwar in dieser Regel" nicht an einer Handvoll
    /// ausgesuchter Gegenbeispiele haengt wie die Nachbarin darueber. Die
    /// Aussage handelt von [`waehrend_blatt_erlaubt`] und nicht von der Lage
    /// „ein Blatt steht"; der Unterschied steht am Doc-Kommentar der Regel.
    /// Ein neues Kommando kommt hier stillschweigend
    /// und richtig mit „gehoert nicht dazu" an; wer es dennoch eintraegt, sieht
    /// diese Probe rot.
    ///
    /// **Warum der Notizzettel eine eigene Zeile bekommt, obwohl der Durchgang
    /// ihn schon deckt:** ein Eintrag fuer ihn waere der naheliegende Griff, um
    /// den Zettel mit derselben Taste wieder zu schliessen, mit der er kommt.
    /// Genau das laesst die Notizzettel-Runde ausdruecklich sein — der Weg
    /// zurueck ist `esc` ueber den Waechter des Zettels. Die Zeile nennt den
    /// Befehl deshalb beim Namen und macht den Fehlschlag lesbar.
    #[test]
    fn in_der_blattsperre_bleibt_es_bei_dem_einen_abbruch() {
        let erlaubt: Vec<Kommando> = Kommando::KENNUNGEN
            .into_iter()
            .map(|(kommando, _)| kommando)
            .filter(|kommando| waehrend_blatt_erlaubt(*kommando))
            .collect();

        assert_eq!(
            erlaubt,
            vec![Kommando::Abbrechen],
            "die Blattsperre laesst nicht mehr allein den Abbruch durch"
        );
        assert!(
            !waehrend_blatt_erlaubt(Kommando::Notizzettel),
            "der Notizzettelbefehl steht in der Ausnahme; der Zettel schliesst              mit esc und nicht mit der Taste, mit der er kommt"
        );
    }

    #[test]
    fn der_abbruch_nennt_die_uebertragene_zahl() {
        let bericht = Bericht {
            abschluss: Abschluss::Abgebrochen,
            eintraege: 4_812,
            bytes: 1_200_000_000,
            uebersprungen: Vec::new(),
        };
        let text = abschlusstext(
            &Art::Kopieren {
                ziel: "/tmp".into(),
            },
            &bericht,
            5,
            0,
        );
        assert!(text.contains("4.812"), "{text}");
        assert!(text.contains("abgebrochen"), "{text}");
        assert!(
            text.contains("5 ausgewählte Positionen"),
            "beide Zahlen stehen nebeneinander: {text}"
        );
    }

    #[test]
    fn ohne_uebersprungene_eintraege_gibt_es_kein_blatt() {
        assert!(uebersprungenliste(&[]).is_none());
    }

    #[test]
    fn eine_lange_liste_uebersprungener_eintraege_wird_gekuerzt() {
        let viele: Vec<Uebersprungen> = (0..30)
            .map(|nummer| Uebersprungen {
                pfad: PathBuf::from(format!("/tmp/datei{nummer}")),
                grund: "keine Rechte".to_owned(),
            })
            .collect();
        let (frage, liste) = uebersprungenliste(&viele).expect("30 Eintraege ergeben ein Blatt");
        assert!(frage.contains("30"), "{frage}");
        assert_eq!(liste.lines().count(), HOECHSTENS_EINZELN + 1);
        assert!(liste.ends_with("… und 18 weitere"), "{liste}");
    }

    #[test]
    fn ein_vergebener_name_meldet_den_grund_und_nicht_den_systemwortlaut() {
        let fehler = io::Error::from(io::ErrorKind::AlreadyExists);
        let text = anlegefehler(Anlegeart::Ordner, "Bilder", &fehler);
        assert!(text.contains("Bilder"), "{text}");
        assert!(text.contains("schon"), "{text}");
        assert!(!text.contains("kind"), "{text}");
    }

    #[test]
    fn beide_anlegebefehle_fragen_dieselbe_frage_mit_eigenem_gegenstand() {
        assert!(Anlegeart::Ordner.frage().contains("Ordner"));
        assert!(Anlegeart::Datei.frage().contains("Datei"));
        assert_eq!(
            Anlegeart::Ordner.bestaetigen(),
            Anlegeart::Datei.bestaetigen(),
            "die Schaltflaeche heisst in beiden Faellen gleich"
        );
        assert_eq!(
            angelegt_text(Anlegeart::Datei, "notiz.md"),
            "Datei „notiz.md“ angelegt"
        );
    }

    /// Seit S17c laeuft das Stapel-Umbenennen ueber die Operationsmaschine und
    /// bekommt denselben Abschlusstext wie die vier uebrigen Arten. Die beiden
    /// Zahlen darin sagen zusammen, was `stapelbericht` vorher in einem eigenen
    /// Satz sagte: umbenannt wurden 48 Eintraege, bestaetigt hatte der Nutzer
    /// 50 Zeilen, also sind zwei stehengeblieben.
    #[test]
    fn der_abschlusstext_des_stapels_nennt_umbenannte_und_bestaetigte_zeilen() {
        let bericht = Bericht {
            abschluss: Abschluss::Fertig,
            eintraege: 48,
            bytes: 4_812,
            uebersprungen: Vec::new(),
        };
        let text = abschlusstext(
            &Art::UmbenennenImStapel {
                neue_namen: Vec::new(),
            },
            &bericht,
            50,
            0,
        );
        assert!(text.starts_with("Umbenennen fertig: "), "{text}");
        assert!(text.contains("48 Einträge"), "{text}");
        assert!(text.contains("50 ausgewählte Positionen"), "{text}");
    }

    /// Der geschnittene Eintrag kommt im Abschlusstext vor (Runde 17).
    ///
    /// **Die eine Stelle, an der der Nutzer davon erfaehrt.** Zip nimmt das
    /// Archiv des vorigen Laufs aus den Quellen; ohne diesen Halbsatz zeigte
    /// die Statuszeile eine Position weniger und beantwortete nicht, warum
    /// (`issues/260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-*`).
    #[test]
    fn der_abschlusstext_nennt_die_ausgelassenen_eintraege() {
        let bericht = Bericht {
            abschluss: Abschluss::Fertig,
            eintraege: 2,
            bytes: 4_812,
            uebersprungen: Vec::new(),
        };
        let art = Art::Zippen {
            ziel: "/tmp/Projekte/Projekte.zip".into(),
        };

        let ohne = abschlusstext(&art, &bericht, 2, 0);
        assert!(
            !ohne.contains("ausgelassen"),
            "ein Lauf ohne Schnitt bekommt den Halbsatz nicht: {ohne}"
        );

        let mit = abschlusstext(&art, &bericht, 2, 1);
        assert!(
            mit.contains("ein Eintrag als Ziel dieses Laufs ausgelassen"),
            "{mit}"
        );
    }

    #[test]
    fn zahlen_bekommen_tausenderpunkte() {
        assert_eq!(zahl(0), "0");
        assert_eq!(zahl(999), "999");
        assert_eq!(zahl(1_000), "1.000");
        assert_eq!(zahl(4_812), "4.812");
        assert_eq!(zahl(1_234_567), "1.234.567");
    }

    // ------------------------------------------------------------------
    // Umbenennen eines einzelnen Eintrags (C4, Schritt 17b)
    // ------------------------------------------------------------------

    #[test]
    fn ein_unveraenderter_name_benennt_nichts_um() {
        assert_eq!(
            umbenennung_pruefen("bericht.txt", "bericht.txt"),
            Umbenennungswunsch::Unveraendert
        );
    }

    /// Der haeufigste Weg zum unveraenderten Namen: der Nutzer hat die Zelle
    /// geoeffnet und nur Leerzeichen angehaengt.
    #[test]
    fn leerzeichen_am_rand_fallen_weg_und_ergeben_denselben_namen() {
        assert_eq!(
            umbenennung_pruefen("bericht.txt", "  bericht.txt  "),
            Umbenennungswunsch::Unveraendert
        );
    }

    #[test]
    fn ein_neuer_name_kommt_getrimmt_zurueck() {
        assert_eq!(
            umbenennung_pruefen("alt.txt", "  neu.txt "),
            Umbenennungswunsch::Neu("neu.txt".to_owned())
        );
    }

    #[test]
    fn ein_unzulaessiger_name_wird_mit_grund_abgelehnt() {
        assert_eq!(
            umbenennung_pruefen("alt.txt", "   "),
            Umbenennungswunsch::Abgelehnt("der Name ist leer")
        );
        assert_eq!(
            umbenennung_pruefen("alt.txt", "unter/ordner"),
            Umbenennungswunsch::Abgelehnt("ein Name darf keinen Schraegstrich enthalten")
        );
        assert_eq!(
            umbenennung_pruefen("alt.txt", ".."),
            Umbenennungswunsch::Abgelehnt("'.' und '..' sind keine Namen")
        );
    }

    /// Der vergebene Name kommt vom Dateisystem und bekommt denselben Satz wie
    /// beim Anlegen.
    #[test]
    fn ein_vergebener_name_meldet_denselben_satz_wie_beim_anlegen() {
        let vergeben = io::Error::from(io::ErrorKind::AlreadyExists);
        assert_eq!(
            umbenennungsfehler("neu.txt", &vergeben),
            anlegefehler(Anlegeart::Datei, "neu.txt", &vergeben)
        );
        assert_eq!(
            umbenennungsfehler("neu.txt", &vergeben),
            "es gibt schon einen Eintrag namens „neu.txt“"
        );
    }

    #[test]
    fn ein_anderer_fehler_behaelt_den_systemwortlaut() {
        let text = umbenennungsfehler("neu.txt", &io::Error::from(io::ErrorKind::NotFound));
        assert!(
            text.starts_with("„neu.txt“ ließ sich nicht vergeben: "),
            "{text}"
        );
    }

    #[test]
    fn datenmengen_bekommen_eine_einheit() {
        assert_eq!(menge(512), "512 Bytes");
        assert_eq!(menge(1_500), "1,5 kB");
        assert_eq!(menge(200_000_000), "200,0 MB");
        assert_eq!(menge(1_200_000_000), "1,2 GB");
    }

    /// Das fuenfte Abnahmekriterium von C11 als Probe: die Meldung nennt die
    /// **eingestellte** Kennung, nicht die Vorbelegung.
    ///
    /// Bis zum 260810 hielt diese Zusage allein der Abnahmelauf, und der ist
    /// Nutzerarbeit; wer den Satz umformulierte, konnte die Kennung
    /// herausnehmen, ohne dass `make check` etwas sagte
    /// (`issues/260810-1753_*_die-beiden-meldungen-des-terminal-befehls-sind-als-einzige-ihres-moduls-ungeprueft.md`).
    /// Der Halbsatz zum Neustart steht mit in der Probe: er ist die Antwort auf
    /// `issues/260807-0930_*_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md`
    /// und stuende sonst genauso ungedeckt da.
    #[test]
    fn die_meldung_zur_buendelkennung_nennt_die_eingestellte_kennung() {
        let text = kein_terminal("com.beispiel.KeinTerminal");
        assert!(
            text.contains("com.beispiel.KeinTerminal"),
            "die Meldung nennt die eingestellte Kennung nicht: {text}"
        );
        assert!(
            text.contains("settings.toml"),
            "die Meldung sagt nicht, wo die Kennung steht: {text}"
        );
        assert!(
            text.contains("Neustart"),
            "die Meldung sagt nicht, dass eine Aenderung erst nach einem Neustart wirkt: {text}"
        );
    }

    /// Alle drei Zweige von [`ordner_fehlt`].
    ///
    /// Der Fall, den C11 meint, ist der dritte: der sichtbare Tab traegt den
    /// Pfad noch, der Datentraeger ist weg. Geprueft wird jeweils, dass der
    /// Pfad in der Meldung steht — er ist die Angabe, mit der der Nutzer etwas
    /// anfangen kann.
    #[test]
    fn ein_fehlender_terminalordner_nennt_den_pfad() {
        let ordner = Pruefordner::neu("terminalordner");
        assert_eq!(
            ordner_fehlt(ordner.pfad()),
            None,
            "ein vorhandener Ordner laesst sich uebergeben"
        );

        let datei = ordner.datei("keine-mappe.txt", b"x");
        let text = ordner_fehlt(&datei).expect("eine Datei ist kein Ordner");
        assert!(
            text.contains(&datei.display().to_string()),
            "die Meldung nennt den Pfad nicht: {text}"
        );
        assert!(text.contains("kein Ordner mehr"), "{text}");

        let fehlt = ordner.unter("ausgeworfen");
        let text = ordner_fehlt(&fehlt).expect("ein fehlender Eintrag ist kein Ordner");
        assert!(
            text.contains(&fehlt.display().to_string()),
            "die Meldung nennt den Pfad nicht: {text}"
        );
        assert!(text.contains("nicht mehr erreichbar"), "{text}");
    }

    // ------------------------------------------------------------------
    // Die Pfade in der Zwischenablage (C1 und C2 der Runde 4)
    // ------------------------------------------------------------------

    /// Die Form aus C1 an den drei Faellen, die sie unterscheidet.
    ///
    /// Der abschliessende Schraegstrich ist keine Erfindung der Probe: die
    /// Pfadeingabe uebernimmt den getippten Text woertlich, also traegt ein Tab
    /// den Ordner `/tmp/x/`, wenn der Nutzer ihn so eingegeben hat.
    #[test]
    fn ein_abschliessender_trenner_faellt_ueberall_ausser_an_der_wurzel() {
        assert_eq!(
            pfadtext(Path::new("/Users/kai/Projekte")),
            "/Users/kai/Projekte"
        );
        assert_eq!(
            pfadtext(Path::new("/Users/kai/Projekte/")),
            "/Users/kai/Projekte"
        );
        assert_eq!(pfadtext(Path::new("/")), "/");
    }

    /// Eine Tilde entsteht auf diesem Weg nicht.
    ///
    /// [`pfadtext`] ruft die Anzeigenfunktion aus `krk_core::ablage::pfade`
    /// nicht; die Probe haelt fest, dass der Pfad ausgeschrieben bleibt, auch
    /// wenn er unter dem Benutzerverzeichnis liegt.
    #[test]
    fn der_pfadtext_bleibt_ausgeschrieben() {
        let heim = std::env::var("HOME").expect("HOME steht in der Umgebung");
        let pfad = PathBuf::from(&heim).join("Schreibtisch");
        let text = pfadtext(&pfad);
        assert!(text.starts_with(&heim), "{text}");
        assert!(!text.contains('~'), "{text}");
    }

    /// Ein Pfad je Zeile, und nach der letzten Zeile kein Umbruch (C2).
    #[test]
    fn pfadzeilen_trennt_mit_umbruechen_und_endet_ohne_einen() {
        let einer = [PathBuf::from("/tmp/x/a.txt")];
        assert_eq!(pfadzeilen(&einer), "/tmp/x/a.txt");
        assert_eq!(pfadzeilen(&einer).matches('\n').count(), 0);

        let drei = [
            PathBuf::from("/tmp/x/a.txt"),
            PathBuf::from("/tmp/x/b.txt"),
            PathBuf::from("/tmp/x/Unterordner/"),
        ];
        assert_eq!(
            pfadzeilen(&drei),
            "/tmp/x/a.txt\n/tmp/x/b.txt\n/tmp/x/Unterordner"
        );
        assert_eq!(pfadzeilen(&drei).matches('\n').count(), 2);
    }

    /// Die Meldung wechselt bei zwei Pfaden vom Pfad auf die Zahl (C1, C2).
    #[test]
    fn die_kopiermeldung_nennt_einen_pfad_und_sonst_die_zahl() {
        let einer = [PathBuf::from("/Users/kai/Projekte/")];
        assert_eq!(kopiermeldung(&einer), "Pfad kopiert: /Users/kai/Projekte");

        let zwei = [PathBuf::from("/tmp/x/a.txt"), PathBuf::from("/tmp/x/b.txt")];
        assert_eq!(kopiermeldung(&zwei), "2 Pfade kopiert");
    }

    /// Jeder der beiden Saetze fuer die leere Menge nennt seine eigene Folge.
    ///
    /// C2 verlangt den Wortlaut: die Statuszeile sagt, dass nichts zu
    /// **kopieren** war. Der Satz des Oeffners sagt dasselbe fuer sein Verb,
    /// und keiner der beiden nennt das des anderen. Die Lage dahinter ist
    /// dieselbe und steht an einer Stelle.
    #[test]
    fn jeder_satz_fuer_die_leere_menge_nennt_seine_eigene_folge() {
        // Kleingeschrieben verglichen: `str::contains` vergleicht
        // buchstabengenau, und ein "Kopier..." oder "Öffn..." am Satzanfang
        // entginge einer Zusicherung gegen die Kleinschreibung.
        let kopieren = nichts_zu_kopieren();
        let klein = kopieren.to_lowercase();
        assert!(klein.contains("kopieren"), "{kopieren}");
        assert!(!klein.contains("öffn"), "{kopieren}");

        let oeffnen = nichts_zu_oeffnen();
        let klein = oeffnen.to_lowercase();
        assert!(klein.contains("öffnen"), "{oeffnen}");
        assert!(!klein.contains("kopier"), "{oeffnen}");

        let lage = "nichts markiert und nichts ausgewählt";
        assert!(kopieren.ends_with(lage), "{kopieren}");
        assert!(oeffnen.ends_with(lage), "{oeffnen}");
    }

    /// Der dritte Satz durch [`nichts_betroffen`] ist der des Packens.
    ///
    /// Er steht in derselben Form wie seine zwei Geschwister darueber und nennt
    /// dieselbe Lage, weil Zip auf dieselbe Menge wirkt wie sie: auf
    /// [`betroffene`]. Ein anders gebauter Satz saehe wie eine andere Lage aus.
    #[test]
    fn der_satz_des_packens_steht_in_der_form_seiner_zwei_geschwister() {
        let packen = nichts_zu_packen();
        let klein = packen.to_lowercase();
        assert!(klein.contains("packen"), "{packen}");
        assert!(!klein.contains("kopier"), "{packen}");
        assert!(!klein.contains("öffn"), "{packen}");
        assert!(
            packen.ends_with("nichts markiert und nichts ausgewählt"),
            "{packen}"
        );
    }

    /// Die zwei Fehlbefunde von Unzip nennen zwei verschiedene Lagen.
    ///
    /// **Sie gehen bewusst nicht durch [`nichts_betroffen`]**, aus demselben
    /// Grund wie der Satz des Teilens: Unzip findet auch dann nichts, wenn
    /// etwas ausgewaehlt ist — es ist nur kein Archiv.
    ///
    /// Der erste nennt die Endung, denn sie ist die ganze Erkennungsregel; der
    /// zweite nennt die Auswahl, denn ueber sie kommt der Nutzer heraus.
    #[test]
    fn die_zwei_fehlbefunde_von_unzip_nennen_zwei_lagen() {
        let keines = kein_archiv();
        let mehrere = mehrere_archive();
        assert_ne!(keines, mehrere);
        for satz in [&keines, &mehrere] {
            assert!(satz.to_lowercase().contains("entpacken"), "{satz}");
            assert!(!satz.contains("markiert"), "{satz}");
            assert!(
                !satz.contains('\n'),
                "die Statuszeile ist einzeilig: {satz}"
            );
        }
        assert!(keines.contains(".zip"), "{keines}");
        assert!(mehrere.contains("Auswahl"), "{mehrere}");
    }

    /// Der Satz des Finders nennt die Buendelkennung nicht.
    ///
    /// Der Unterschied zu [`kein_terminal`], und er ist keine Wortklauberei:
    /// dort ist die Kennung die Angabe, mit der der Nutzer `settings.toml`
    /// berichtigt. Fuer den Finder steht sie fest im Baum; sie zu nennen gaebe
    /// dem Nutzer nichts und stellte dieselbe Zeichenfolge an eine zweite
    /// Stelle.
    #[test]
    fn der_satz_des_finders_nennt_die_kennung_nicht() {
        let finder = kein_finder();
        assert!(finder.contains("Finder"), "{finder}");
        assert!(!finder.contains("com.apple"), "{finder}");
        assert!(!finder.contains("settings.toml"), "{finder}");
        assert!(
            !finder.contains('\n'),
            "die Statuszeile ist einzeilig: {finder}"
        );
    }

    // ------------------------------------------------------------------
    // Die Dateiverweise in der Zwischenablage (Runde 22)
    // ------------------------------------------------------------------

    /// Ein Name je Zeile, ohne Schlusszeilenumbruch, in gegebener Reihenfolge,
    /// und ein Ordner ohne abschliessenden Trenner (C2.1 bis C2.3).
    #[test]
    fn namenszeilen_tragen_namen_ohne_umbruch_am_ende_und_ohne_trenner() {
        let einer = [PathBuf::from("/tmp/x/a.txt")];
        assert_eq!(namenszeilen(&einer), "a.txt");
        assert_eq!(namenszeilen(&einer).matches('\n').count(), 0);

        let drei = [
            PathBuf::from("/tmp/x/c.txt"),
            PathBuf::from("/tmp/x/a.txt"),
            PathBuf::from("/tmp/x/b.txt"),
        ];
        assert_eq!(namenszeilen(&drei), "c.txt\na.txt\nb.txt");
        assert_eq!(namenszeilen(&drei).matches('\n').count(), 2);

        let ordner = [PathBuf::from("/tmp/x/Unterordner/")];
        assert_eq!(namenszeilen(&ordner), "Unterordner");
        assert!(!namenszeilen(&ordner).contains(std::path::MAIN_SEPARATOR));
    }

    /// Die Meldung nach `cmd+c` nennt einen Namen und sonst die Zahl (C1.8, A6).
    #[test]
    fn die_ablagemeldung_nach_kopieren_nennt_einen_namen_und_sonst_die_zahl() {
        let einer = [PathBuf::from("/tmp/x/Übergabe.txt")];
        assert_eq!(
            ablagemeldung(Dateiablage::Kopieren, &einer),
            "kopiert: Übergabe.txt"
        );

        let drei = [
            PathBuf::from("/tmp/x/a.txt"),
            PathBuf::from("/tmp/x/b.txt"),
            PathBuf::from("/tmp/x/Ordner/"),
        ];
        assert_eq!(
            ablagemeldung(Dateiablage::Kopieren, &drei),
            "3 Einträge kopiert"
        );
    }

    /// Die Meldung nach `cmd+x` traegt den Zusatz, dass das Ziel verschiebt
    /// (C3.2, A6).
    #[test]
    fn die_ablagemeldung_nach_ausschneiden_sagt_dass_das_ziel_verschiebt() {
        let einer = [PathBuf::from("/tmp/x/Übergabe.txt")];
        assert_eq!(
            ablagemeldung(Dateiablage::Ausschneiden, &einer),
            "kopiert: Übergabe.txt – verschieben tut das Ziel (Finder: opt+cmd+v)"
        );

        let drei = [
            PathBuf::from("/tmp/x/a.txt"),
            PathBuf::from("/tmp/x/b.txt"),
            PathBuf::from("/tmp/x/Ordner/"),
        ];
        assert_eq!(
            ablagemeldung(Dateiablage::Ausschneiden, &drei),
            "3 Einträge kopiert – verschieben tut das Ziel (Finder: opt+cmd+v)"
        );
    }

    /// Die zwei Befehle unterscheiden sich allein im Zusatz (C3.1, Texthaelfte).
    ///
    /// Ausschneiden ist ein Kopieren mit einem Satz (A4): die Meldung nach
    /// `cmd+x` beginnt mit der ganzen Meldung nach `cmd+c`, und was danach
    /// kommt, ist der eine Zusatz.
    #[test]
    fn die_meldung_nach_ausschneiden_beginnt_mit_der_nach_kopieren() {
        let zusatz = " – verschieben tut das Ziel (Finder: opt+cmd+v)";
        let mengen: [&[PathBuf]; 2] = [
            &[PathBuf::from("/tmp/x/a.txt")],
            &[PathBuf::from("/tmp/x/a.txt"), PathBuf::from("/tmp/x/b.txt")],
        ];
        for pfade in mengen {
            let kopieren = ablagemeldung(Dateiablage::Kopieren, pfade);
            let ausschneiden = ablagemeldung(Dateiablage::Ausschneiden, pfade);
            assert!(ausschneiden.starts_with(&kopieren), "{ausschneiden}");
            assert_eq!(&ausschneiden[kopieren.len()..], zusatz);
            assert!(!kopieren.contains("verschieben"), "{kopieren}");
        }
    }

    /// Der Satz der abweisenden Ablage nennt die Eintraege (A6, A12).
    #[test]
    fn der_satz_der_abgewiesenen_verweise_nennt_die_eintraege() {
        assert_eq!(
            verweise_abgewiesen(),
            "die Zwischenablage hat die Einträge nicht angenommen"
        );
        assert_ne!(verweise_abgewiesen(), ablage_weist_ab());
    }

    // ------------------------------------------------------------------
    // Was ein Vorgang erzeugt (Runde 17)
    // ------------------------------------------------------------------

    /// Die Tafel ueber alle sechs Werte von [`Art`], von Hand geschrieben.
    ///
    /// **Sie ist die zweite Haelfte der Vollstaendigkeit.** Der Uebersetzer
    /// erzwingt, dass [`erzeugt_genau_ein_ziel`] jeden Wert beantwortet, aber
    /// nicht, dass eine Probe jeden nennt; ein siebter Wert liefe sonst
    /// ungeprueft mit.
    #[test]
    fn genau_ein_ziel_erzeugt_allein_das_packen() {
        let ziel = PathBuf::from("/tmp/x");
        let tafel: [(Art, bool); 6] = [
            (Art::Kopieren { ziel: ziel.clone() }, false),
            (Art::Verschieben { ziel: ziel.clone() }, false),
            (Art::InDenPapierkorb, false),
            (
                Art::UmbenennenImStapel {
                    neue_namen: vec!["a".to_owned()],
                },
                false,
            ),
            (Art::Zippen { ziel: ziel.clone() }, true),
            (
                Art::Entpacken {
                    ziele: vec![ziel.clone(), ziel.clone()],
                },
                false,
            ),
        ];
        for (art, erwartet) in tafel {
            assert_eq!(
                erzeugt_genau_ein_ziel(&art),
                erwartet,
                "{art:?} wird falsch eingeordnet"
            );
        }
    }

    /// Ein Vorgang ueber **ein** Archiv erzeugt genau ein Ziel, einer ueber
    /// zwei nicht.
    ///
    /// **Der Fall, um dessentwillen das Entpacken an einer Zahl haengt und
    /// nicht am Wert.** Seit der Nutzerentscheidung vom 260824-2120 kann ein
    /// Vorgang mehrere Archive tragen; dann wird der Zielordner-Konflikt je
    /// Archiv gefragt, und das Ankreuzfeld „fuer alle weiteren" traegt wieder
    /// seinen Gegenstand. Bei genau einem traegt es ihn nicht.
    #[test]
    fn ein_einzelnes_archiv_entpackt_in_genau_ein_ziel() {
        let eines = Art::Entpacken {
            ziele: vec![PathBuf::from("/tmp/x/eins")],
        };
        assert!(erzeugt_genau_ein_ziel(&eines));

        let zwei = Art::Entpacken {
            ziele: vec![PathBuf::from("/tmp/x/eins"), PathBuf::from("/tmp/x/zwei")],
        };
        assert!(!erzeugt_genau_ein_ziel(&zwei));
    }

    /// Der Satz des Teilens nennt seine Folge und **keine** Ursache.
    ///
    /// Er steht neben den beiden darueber und geht bewusst nicht durch
    /// [`nichts_betroffen`]: das Teilen wirkt aus jedem Fokus und findet auf
    /// drei Weisen nichts, und ein Satz ueber Markierung und Auswahl waere in
    /// zweien davon falsch. In der Leiste hat der Nutzer ein Lesezeichen
    /// ausgewaehlt vor sich und laese, es sei nichts ausgewaehlt.
    ///
    /// Einzeilig wie jede Antwort der Statuszeile.
    #[test]
    fn der_satz_des_teilens_nennt_die_folge_und_keine_ursache() {
        let teilen = nichts_zu_teilen();
        assert!(teilen.to_lowercase().contains("teilen"), "{teilen}");
        assert!(!teilen.contains("markiert"), "{teilen}");
        assert!(!teilen.contains("ausgewählt"), "{teilen}");
        assert!(
            !teilen.contains('\n'),
            "die Statuszeile ist einzeilig: {teilen}"
        );
    }

    // ------------------------------------------------------------------
    // Die Uebergabe an das Standardprogramm (C3 der Runde 4)
    // ------------------------------------------------------------------

    /// Die vier Faelle der Meldung aus C3.
    ///
    /// Einer angenommen nennt seinen Namen, mehrere ihre Zahl; einer abgewiesen
    /// nennt seinen Namen, ein abgewiesener Teil nennt seine Zahl und die
    /// Gesamtzahl.
    #[test]
    fn die_oeffnungsmeldung_nennt_einen_namen_und_sonst_die_zahl() {
        let einer = [PathBuf::from("/tmp/x/Bericht.pdf")];
        let zwei = [
            PathBuf::from("/tmp/x/Bericht.pdf"),
            PathBuf::from("/tmp/x/Notiz.txt"),
        ];
        let keiner: [PathBuf; 0] = [];

        assert_eq!(
            oeffnungsmeldung(&einer, &keiner),
            "an das System übergeben: Bericht.pdf"
        );
        assert_eq!(
            oeffnungsmeldung(&zwei, &keiner),
            "2 Einträge an das System übergeben"
        );
        assert_eq!(
            oeffnungsmeldung(&keiner, &einer),
            "das System hat Bericht.pdf nicht angenommen"
        );
        assert_eq!(
            oeffnungsmeldung(&einer, &zwei),
            "an das System übergeben: Bericht.pdf; \
             das System hat 2 von 3 Einträgen nicht angenommen"
        );
    }

    /// Die Meldung behauptet nicht, ein Programm habe den Eintrag geoeffnet.
    ///
    /// Das ist die Zusage aus dem Kopf des Umsetzungsplans und aus
    /// [`crate::appkit::standardprogramm`]: `openURL:` meldet die Annahme und
    /// nicht den Start. Die Probe haelt den Wortlaut daran fest, damit ein
    /// spaeteres Umformulieren nicht unbemerkt mehr zusagt, als KRK weiss.
    #[test]
    fn die_oeffnungsmeldung_behauptet_kein_geoeffnet() {
        let einer = [PathBuf::from("/tmp/x/Bericht.pdf")];
        let drei = [
            PathBuf::from("/tmp/x/a.txt"),
            PathBuf::from("/tmp/x/b.txt"),
            PathBuf::from("/tmp/x/c.txt"),
        ];
        let keiner: [PathBuf; 0] = [];

        for meldung in [
            oeffnungsmeldung(&einer, &keiner),
            oeffnungsmeldung(&drei, &keiner),
            oeffnungsmeldung(&keiner, &einer),
            oeffnungsmeldung(&einer, &drei),
        ] {
            // "öffn" faengt "öffnet" und "geöffnet" zugleich, und die
            // Kleinschreibung des Vergleichs faengt daneben ein "Öffnet ..."
            // am Satzanfang: `str::contains` vergleicht buchstabengenau, und
            // "Ö" ist ein anderes Zeichen als "ö".
            let klein = meldung.to_lowercase();
            assert!(!klein.contains("öffn"), "{meldung}");
            assert!(!klein.contains("gestartet"), "{meldung}");
            assert!(meldung.contains("System"), "{meldung}");
        }
    }

    /// Ein Pfad ohne letzten Bestandteil steht mit seinem Pfad in der Meldung.
    ///
    /// Das Wurzelverzeichnis ist der Fall, den es wirklich gibt: er entsteht,
    /// wenn die Auswahl auf `/` steht.
    #[test]
    fn ein_eintrag_ohne_namen_erscheint_mit_seinem_pfad() {
        let wurzel = [PathBuf::from("/")];
        let keiner: [PathBuf; 0] = [];
        assert_eq!(
            oeffnungsmeldung(&wurzel, &keiner),
            "an das System übergeben: /"
        );
    }

    /// Zwei leere Mengen ergeben den Satz der leeren Menge und keinen Unsinn.
    ///
    /// Der Aufrufer faengt den Fall vorher ab; die Probe haelt fest, dass die
    /// Funktion ihn trotzdem beantwortet, statt eine Meldung ohne Gegenstand zu
    /// bauen.
    #[test]
    fn zwei_leere_mengen_ergeben_den_satz_der_leeren_menge() {
        let keiner: [PathBuf; 0] = [];
        assert_eq!(oeffnungsmeldung(&keiner, &keiner), nichts_zu_oeffnen());
    }
}
