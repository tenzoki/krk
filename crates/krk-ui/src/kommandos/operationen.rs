//! Der Ablauf der Dateioperationen aus C4, ohne AppKit.
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
//!        ├─ Fortschrittsblatt       └─ Buendelung: verwirft den Weckruf,
//!        ├─ Konfliktblatt              solange der vorige nicht gezeichnet ist
//!        ├─ Loeschbestaetigung
//!        └─ Abschlussliste
//! ```
//!
//! # Die 150-ms-Regel
//!
//! C4 verlangt einen Fortschritt ab 100 Eintraegen oder 100 MB, L8 verlangt ihn
//! 200 ms nach dem Start sichtbar. Den Umfang eines Ordnerbaums vorher zu
//! bestimmen kostet einen eigenen Durchlauf, der die 200 ms selbst aufbrauchen
//! kann. Statt zweier Schwellen gilt deshalb eine Zeitspanne: das
//! Fortschrittsblatt erscheint, sobald die Operation [`BLATTVERZUG`] gelaufen
//! ist (`### Frage 6` des Plans). Eine kleine Kopie ist vorher fertig und laesst
//! kein Blatt aufblitzen.
//!
//! **Der Verzug wird an einer Meldung gemessen und nicht an einem Zeitgeber.**
//! Das Blatt geht mit der ersten Meldung auf, die nach 150 ms eintrifft. Der
//! Arbeitsfaden meldet jeden fertigen Eintrag und, waehrend einer einzelnen
//! grossen Datei, alle 8 ms einen Zwischenstand; die Spanne zwischen dem
//! Ablauf der 150 ms und dem Aufgehen des Blattes ist damit im Regelfall
//! kleiner als eine Bildlaenge. Sie ist es **nicht** bei einer Operation, die
//! ueber Sekunden gar nichts meldet; der einzige solche Fall in dieser Runde
//! ist `NSFileManager.trashItemAtURL:` auf einem sehr grossen Ordner, der als
//! ein Eintrag zaehlt und erst am Ende meldet. Das ist der Preis der Wahl
//! "kein Takt" und im Bericht zu S16 ausgeschrieben.
//!
//! # Die Buendelung ohne Takt
//!
//! Der Nutzer hat am 260804 Weg 3 aus
//! `issues/260803-2007_o_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`
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

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::SyncSender;
use std::time::{Duration, Instant};

use krk_core::operation::{Abschluss, Art, Bericht, Fortschritt, Konfliktentscheid, Uebersprungen};
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::Ordnermodell;

/// Wie lange eine Operation laufen muss, bevor das Fortschrittsblatt aufgeht.
///
/// 150 ms lassen L8 ("Fortschritt sichtbar, 200 ms nach Start") 50 ms Reserve.
/// Eine Operation ueber 100 Eintraege oder 100 MB, die C4 als Schwelle nennt,
/// ist nach 150 ms nachweislich noch nicht fertig.
pub const BLATTVERZUG: Duration = Duration::from_millis(150);

/// Hoechstens so viele uebersprungene Eintraege stehen einzeln in der
/// Abschlussliste.
///
/// Eine Kopie ueber einen Ordner ohne Leserechte kann Tausende erzeugen; ein
/// Blatt, das den Bildschirm ueberragt, ist keine Auskunft mehr. Der Rest wird
/// gezaehlt.
const HOECHSTENS_EINZELN: usize = 12;

/// Ob das Fortschrittsblatt jetzt faellig ist.
pub fn blatt_faellig(begonnen: Instant, jetzt: Instant) -> bool {
    jetzt.duration_since(begonnen) >= BLATTVERZUG
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
// Der Fokusvorbehalt der Loeschtasten
// ----------------------------------------------------------------------

/// Wo der Eingabefokus steht, soweit es die Loeschtasten angeht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fokus {
    /// Im Hauptfenster, also in einer der beiden Dateilisten.
    Dateifenster,
    /// Irgendwo sonst: in einem Blatt oder in einem Textfeld.
    Anderswo,
}

/// Ob die Loeschtasten wirken duerfen (C4).
///
/// "Die Loeschtasten loesen nur dann eine Loeschung aus, wenn der Eingabefokus
/// in einem Dateifenster steht." Der Vorbehalt fuer Textfelder sitzt seit S13
/// im Ereignisabgriff und faengt die Pfadeingabe ab, bevor ein Kommando
/// ueberhaupt entsteht. Diese Regel faengt den zweiten Fall: ein **Blatt** mit
/// Schaltflaechen, dessen Ersthelfer kein Textfeld ist. Ohne sie loeschte ein
/// Delete vor dem offenen Fortschrittsblatt in dem Ordner dahinter.
pub fn loeschtaste_wirkt(fokus: Fokus) -> bool {
    fokus == Fokus::Dateifenster
}

/// Was durchkommt, solange ein Blatt steht oder eine Dateioperation laeuft.
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
    /// Der Abbruchwunsch des Nutzers.
    ///
    /// Der Hauptfaden setzt ihn, der Vermittlerfaden reicht ihn an den
    /// [`krk_core::operation::Lauf`] weiter, sobald die naechste Meldung ihn
    /// aufweckt. Der Hauptfaden kann den Lauf nicht selbst abbrechen, weil der
    /// Empfaenger seines Kanals nicht zwischen Faeden geteilt werden darf.
    abbruch: AtomicBool,
    stand: Mutex<Anzeigestand>,
}

impl Default for Vorgangszustand {
    fn default() -> Self {
        Self::neu()
    }
}

impl Vorgangszustand {
    /// Ein leerer Zustand.
    pub fn neu() -> Self {
        Self {
            buendelung: Buendelung::neu(),
            abbruch: AtomicBool::new(false),
            stand: Mutex::new(Anzeigestand::default()),
        }
    }

    /// Der Nutzer hat abgebrochen.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }

    /// Ob der Nutzer abgebrochen hat.
    pub fn abgebrochen(&self) -> bool {
        self.abbruch.load(Ordering::Relaxed)
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

/// Die Ueberschrift des Fortschrittsblattes.
pub fn ueberschrift(art: &Art) -> &'static str {
    match art {
        Art::Kopieren { .. } => "Kopieren",
        Art::Verschieben { .. } => "Verschieben",
        Art::InDenPapierkorb => "In den Papierkorb räumen",
        Art::EndgueltigLoeschen => "Endgültig löschen",
    }
}

/// Die Standzeile des Fortschrittsblattes.
///
/// **Zwei Zahlen, weil eine von beiden je nach Fall nichts sagt.** Die Zahl der
/// Eintraege ist die des Kerns: was die Operation angefasst hat. Beim
/// Verschieben innerhalb eines Datentraegers ist das genau ein Eintrag je
/// Position, weil `rename(2)` den Inhalt nie beruehrt; beim Kopieren desselben
/// Ordners sind es Hunderte. Die Zahl der Positionen daneben ist ueber alle
/// Faelle dieselbe. Der Widerspruch ist als Defekt
/// `issues/260804-1649_o_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren.md`
/// festgehalten; dieser Schritt entscheidet ihn nicht, er zeigt beide Zahlen
/// nebeneinander und benennt sie.
pub fn standtext(fortschritt: Option<&Fortschritt>, positionen: usize) -> String {
    let Some(fortschritt) = fortschritt else {
        return format!("{} wird vorbereitet …", positionen_text(positionen));
    };
    let name = fortschritt
        .eintrag
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    format!(
        "{}, {} — {}\n{}",
        eintraege_text(fortschritt.eintraege as usize),
        menge(fortschritt.bytes),
        positionen_text(positionen),
        name
    )
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
fn ordner_text(ordner: usize) -> String {
    match ordner {
        1 => "ein Ordner".to_owned(),
        zahl => format!("{} Ordner", self::zahl(zahl)),
    }
}

/// Eine Zahl mit Punkten als Tausendertrennung, wie sie der Nutzer liest.
fn zahl(wert: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    use krk_core::verzeichnis::{Eintrag, Typ};

    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag {
            name: name.to_owned(),
            sortierschluessel: Box::from(name.as_bytes()),
            groesse: 0,
            geaendert: std::time::SystemTime::UNIX_EPOCH,
            typ,
            versteckt: false,
        }
    }

    fn modell_mit(namen: &[(&str, Typ)]) -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.leeren(1);
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
    #[test]
    fn auch_ueber_die_fadengrenze_weckt_nicht_jede_meldung() {
        let zustand = Arc::new(Vorgangszustand::neu());
        let arbeiter = Arc::clone(&zustand);
        let melder =
            thread::spawn(move || (0..20_000).filter(|_| arbeiter.buendelung.melden()).count());
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
    fn das_blatt_geht_erst_nach_150_ms_auf() {
        let begonnen = Instant::now();
        assert!(!blatt_faellig(begonnen, begonnen));
        assert!(!blatt_faellig(
            begonnen,
            begonnen + Duration::from_millis(149)
        ));
        assert!(blatt_faellig(begonnen, begonnen + BLATTVERZUG));
    }

    #[test]
    fn die_loeschtasten_wirken_nur_im_dateifenster() {
        assert!(loeschtaste_wirkt(Fokus::Dateifenster));
        assert!(!loeschtaste_wirkt(Fokus::Anderswo));
    }

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
    fn zahlen_bekommen_tausenderpunkte() {
        assert_eq!(zahl(0), "0");
        assert_eq!(zahl(999), "999");
        assert_eq!(zahl(1_000), "1.000");
        assert_eq!(zahl(4_812), "4.812");
        assert_eq!(zahl(1_234_567), "1.234.567");
    }

    #[test]
    fn datenmengen_bekommen_eine_einheit() {
        assert_eq!(menge(512), "512 Bytes");
        assert_eq!(menge(1_500), "1,5 kB");
        assert_eq!(menge(200_000_000), "200,0 MB");
        assert_eq!(menge(1_200_000_000), "1,2 GB");
    }
}
