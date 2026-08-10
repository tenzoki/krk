//! Der Ablauf der Dateioperationen aus C4, ohne AppKit.
//!
//! **Seit Schritt 18c stehen auch die beiden Antworten des Terminal-Befehls
//! aus C11 hier**, am Fuss der Datei. Sie sind keine Dateioperation, teilen
//! aber deren Zuschnitt vollstaendig: ein Befehl, der auf den sichtbaren Tab
//! des aktiven Dateifensters wirkt und seine Antwort als Befehlsantwort in die
//! Statuszeile schreibt. Ein eigenes Modul fuer zwei Saetze waere ein
//! sechstes unter [`crate::kommandos`] mit einer einzigen Frage.
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
    /// Wie viele davon Ordner sind. Die Rueckfrage vor dem endgueltigen
    /// Loeschen nennt die Zahl gesondert (C4).
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

// ----------------------------------------------------------------------
// Was durchkommt, solange ein Blatt steht
// ----------------------------------------------------------------------

/// Was durchkommt, solange ein Blatt steht.
///
/// Genau der Abbruchbefehl. Alles uebrige geht unveraendert an AppKit weiter,
/// damit das Blatt seine eigene Tastaturbedienung behaelt und der Abgriff kein
/// Ereignis ins Leere schluckt.
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
        Art::EndgueltigLoeschen => "Endgültig löschen",
        Art::UmbenennenImStapel { .. } => "Umbenennen",
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

/// Die beiden Zeilen der Rueckfrage vor dem endgueltigen Loeschen (C4).
///
/// Genau einmal je Vorgang, unabhaengig von der Zahl der Eintraege. Die
/// Rueckfrage nennt die Zahl der Eintraege und, falls Ordner darunter sind,
/// deren Zahl gesondert.
pub fn loeschfrage(auswahl: &Auswahl) -> (String, String) {
    let frage = match auswahl.zahl() {
        1 => "Diesen Eintrag endgültig löschen?".to_owned(),
        zahl => format!("Diese {} Einträge endgültig löschen?", self::zahl(zahl)),
    };
    let mut erlaeuterung = String::from(
        "Endgültig gelöschte Einträge lassen sich nicht wiederherstellen; \
         KRK führt keinen eigenen Rückgängig-Speicher.",
    );
    if auswahl.ordner > 0 {
        erlaeuterung.push_str(&format!(
            "\n\nDarunter {}, jeweils mit ihrem gesamten Inhalt.",
            ordner_text(auswahl.ordner)
        ));
    }
    (frage, erlaeuterung)
}

/// Die Meldung, die nach dem Ende eines Vorgangs in der Statuszeile steht.
///
/// Nach einem Abbruch nennt sie, wie viele Eintraege bereits uebertragen wurden
/// (C4).
pub fn abschlusstext(art: &Art, bericht: &Bericht, positionen: usize) -> String {
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
/// sie hier und nicht in [`super::auswahl`].
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
/// dieselben Punkte zeigt wie ein laufender Vorgang.
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
// Das Terminal im angezeigten Ordner (C11, Schritt 18c)
// ----------------------------------------------------------------------

/// Ob der Ordner noch da ist, den der Terminal-Befehl uebergeben soll (C11).
///
/// `None`, wenn er sich uebergeben laesst; sonst der Satz fuer die Statuszeile.
/// Der Fall ist der ausgeworfene Datentraeger: der sichtbare Tab traegt den
/// Pfad noch, den Ordner gibt es nicht mehr.
///
/// **Geprueft wird vor dem Aufruf und nicht nach ihm.** Der Rueckruf von
/// `openURLs:…` bleibt leer (siehe `crate::appkit::terminal`), also ist dies
/// die eine Gelegenheit, dem Nutzer etwas zu sagen, das er beheben kann.
///
/// Nicht ueber [`super::pfadeingabe::pruefen`]: jene Funktion beantwortet
/// "wohin geht KRK", und ihre Antwort traegt ein Sprungziel samt Auswahl, das
/// dieser Befehl nicht braucht und nicht auswerten wuerde. Sie verlangte zudem
/// das Leserecht, das eine Terminal-Sitzung in einem Ordner nicht braucht.
pub fn terminalordner_fehlt(ordner: &Path) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    use krk_core::verzeichnis::{Eintrag, Typ};

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
    #[test]
    fn bei_stehendem_blatt_kommt_allein_der_abbruch_durch() {
        assert!(waehrend_blatt_erlaubt(Kommando::Abbrechen));
        assert!(!waehrend_blatt_erlaubt(Kommando::InPapierkorb));
        assert!(!waehrend_blatt_erlaubt(Kommando::AuswahlRunter));
        assert!(
            !waehrend_blatt_erlaubt(Kommando::FensterWechseln),
            "der Tabulator gehoert dem Blatt, sonst ist es ohne Maus nicht zu beantworten"
        );
    }

    #[test]
    fn die_rueckfrage_nennt_die_zahl_der_eintraege_und_die_der_ordner() {
        let auswahl = Auswahl {
            pfade: vec![
                PathBuf::from("/tmp/a"),
                PathBuf::from("/tmp/b"),
                PathBuf::from("/tmp/c"),
            ],
            ordner: 2,
        };
        let (frage, erlaeuterung) = loeschfrage(&auswahl);
        assert!(frage.contains('3'), "die Zahl der Eintraege fehlt: {frage}");
        assert!(
            erlaeuterung.contains("2 Ordner"),
            "die Zahl der Ordner fehlt gesondert: {erlaeuterung}"
        );
    }

    #[test]
    fn ohne_ordner_nennt_die_rueckfrage_keine() {
        let auswahl = Auswahl {
            pfade: vec![PathBuf::from("/tmp/a")],
            ordner: 0,
        };
        let (_, erlaeuterung) = loeschfrage(&auswahl);
        assert!(!erlaeuterung.contains("Ordner"));
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
        );
        assert!(text.starts_with("Umbenennen fertig: "), "{text}");
        assert!(text.contains("48 Einträge"), "{text}");
        assert!(text.contains("50 ausgewählte Positionen"), "{text}");
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
}
