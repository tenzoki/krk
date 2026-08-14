//! Der Durchlauf ueber die Unterbaeume: liegt unter diesem Ordner ein Treffer?
//!
//! Das Ordnermodell entscheidet die Sichtbarkeit einer Zeile aus fuenf
//! Eingaben, und vier davon hat es selbst. Die fuenfte ist der Befund ueber den
//! Unterbaum eines Ordners, und den ermittelt dieses Modul: je Ordner des
//! angezeigten Ordners, **dessen eigener Name den Filtertext nicht traegt**,
//! genau einen von zwei Befunden. Wer den Auftrag zusammenstellt, ist nicht
//! Sache dieser Datei; sie bekommt die Liste beim Start vollstaendig
//! uebergeben.
//!
//! ```text
//! ist er eine Verknuepfung? ─ ja ──> kein Treffer darunter
//!            │ nein
//! laesst er sich oeffnen?   ─ nein ─> kein Treffer darunter
//!            │ ja
//! naechsten Stapel holen    ─ leer ─> zurueck zum uebergeordneten Ordner
//!            │                        (oder: kein Treffer darunter)
//! Name traegt die Folge?    ─ ja ──> Treffer, der Rest bleibt ungelesen
//!            │ nein
//! ist es ein Ordner?        ─ ja ──> absteigen
//!            └ nein ─────────────────> naechster Eintrag im Stapel
//! ```
//!
//! # Die Bauart ist die des Lesevorgangs
//!
//! Ein Arbeitsfaden je Durchlauf, ein Kanal mit der Kapazitaet eines Stapels,
//! ein Abbruchkennzeichen als `Arc<AtomicBool>`, und `Drop` setzt es. Gelesen
//! wird ueber [`Schwungleser`], dieselbe Huelle, die auch
//! [`super::leser::lesen`] und der gestueckelte Lesevorgang benutzen; eine
//! zweite Lesemechanik entsteht nicht.
//!
//! **Der Kanal ist so tief wie ein Stapel, und die Einheit ist eine andere.**
//! Beim Lesevorgang haelt der Kanal Stapel von Eintraegen, und seine Tiefe 1
//! ist eine Aussage ueber den Speicher. Hier haelt er Befunde zu je einem
//! Eintragsindex und einem Wahrheitswert; dieselbe Zahl 1.024 kostet damit acht
//! Kilobyte statt eines zweiten Ordnerbestands. Mit der Tiefe 1 blockierte der
//! Arbeitsfaden nach jedem einzelnen Befund bis zum naechsten Einzugstakt, also
//! bis zu 16 ms je entschiedenem Ordner.
//!
//! **Die Abbruchzusage haengt nicht an der Kanaltiefe, sondern an der
//! Stapelgrenze.** Das Kennzeichen wird gelesen, bevor der naechste Stapel
//! geholt wird, und ausdruecklich **nicht** beim Absteigen: ein Ordner mit
//! fuenfzigtausend gewoehnlichen Eintraegen und ohne einen einzigen Unterordner
//! steigt kein einziges Mal ab, passiert die Stapelgrenze aber neunundvierzig
//! Mal. Haenge man die Pruefung ans Absteigen, waere genau dieser Ordner von
//! der Zusage ausgenommen.
//!
//! # Was dieses Modul nicht hat, und warum
//!
//! **Keine Tiefengrenze und keine Zaehlung gegen eine Grenze.** Der Abstieg
//! laeuft ueber einen eigenen Stapel von [`Ebene`]n und nicht ueber die
//! Rekursion des Arbeitsfadens; ein tiefer Baum sprengt damit keinen
//! Fadenstapel, und es gibt nichts, wogegen zu zaehlen waere. Der Preis ist ein
//! offener Deskriptor je Ebene, denn der uebergeordnete Ordner wird nach der
//! Rueckkehr aus dem Abstieg weitergelesen.
//!
//! **Keinen mitgefuehrten Zustand ueber besuchte Ordner.** In eine symbolische
//! Verknuepfung wird nicht abgestiegen, und damit ist kein Ordner zweimal zu
//! erreichen; eine Besuchtliste haette nichts zu verhindern.
//!
//! **Keinen Deckel auf die Trefferzahl.** Es wird nicht gezaehlt: der erste
//! Treffer entscheidet den Auftrag, in welcher Tiefe er auch liegt, und der
//! Rest unter ihm bleibt ungelesen.
//!
//! **Kein `warten`.** Der [`Lesevorgang`](super::leser::Lesevorgang) haelt sein
//! Fadenstueck fuer Aufrufer, die auf den Abschluss warten; hier gibt es
//! keinen, der wartet. Dass der Faden geendet hat, sagt der geschlossene Kanal.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

use super::eintrag::Typ;
use super::leser::STAPELGROESSE;
use super::sys::Schwungleser;

/// Ein Ordner des angezeigten Ordners, ueber den noch nichts bekannt ist.
///
/// Der Eintragsindex und nicht die Zeile: die Sichtreihenfolge wird bei jedem
/// Sortierwechsel neu gebaut, der Bestand nicht. Der Name statt des vollen
/// Pfades, weil alle Auftraege im selben Ordner liegen und der einmal beim
/// Start uebergeben wird.
#[derive(Debug, Clone)]
pub struct Auftrag {
    /// Der Index des Ordners im Bestand des Ordnermodells.
    pub index: u32,
    /// Sein Name ohne Pfad.
    pub name: String,
}

/// Was der Arbeitsfaden ueber einen Auftrag meldet.
///
/// Genau eine Meldung je Auftrag, und sie beendet ihn. **`treffer: false` ist
/// nicht dasselbe wie „noch nichts gemeldet"**: der erste heisst „gelesen, es
/// liegt nichts darunter", das Ausbleiben heisst „noch nicht entschieden". Der
/// Unterschied ist die Zusage C3.13, und er entsteht dadurch, dass ein
/// abgebrochener Durchlauf gar nichts meldet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Befundmeldung {
    /// Der Eintragsindex aus dem [`Auftrag`].
    pub index: u32,
    /// Wahr, wenn unter diesem Ordner ein Eintrag liegt, dessen Name den
    /// Filtertext traegt.
    pub treffer: bool,
}

/// Ein laufender Durchlauf auf einem eigenen Faden.
pub struct Durchlauf {
    abbruch: Arc<AtomicBool>,
    befunde: Receiver<Befundmeldung>,
}

impl Durchlauf {
    /// Startet den Durchlauf und kehrt sofort zurueck.
    ///
    /// `filter_klein` ist der bereits kleingeschriebene Filtertext: er wird
    /// einmal je Suche umgeschrieben und nicht einmal je gelesenem Namen.
    /// `generation` benennt allein den Arbeitsfaden (`krk-durchlauf-<n>`),
    /// damit ein Fadenprotokoll lesbar bleibt; den Befunden liegt sie nicht
    /// bei, weil jeder Tab seinen eigenen Durchlauf haelt und allein aus dessen
    /// Kanal liest.
    pub fn starten(
        auftraege: Vec<Auftrag>,
        ordner: PathBuf,
        filter_klein: String,
        generation: u64,
    ) -> Self {
        let abbruch = Arc::new(AtomicBool::new(false));
        let (sender, befunde) = sync_channel(STAPELGROESSE);
        let faden_abbruch = Arc::clone(&abbruch);
        thread::Builder::new()
            .name(format!("krk-durchlauf-{generation}"))
            .spawn(move || {
                durchlauffaden(&auftraege, &ordner, &filter_klein, &faden_abbruch, &sender);
            })
            .expect("Arbeitsfaden fuer den Durchlauf laesst sich nicht starten");
        Self { abbruch, befunde }
    }

    /// Der Kanal, aus dem der Hauptfaden die Befunde holt.
    ///
    /// Er schliesst, wenn der Arbeitsfaden geendet hat. Ein geschlossener Kanal
    /// ohne weitere Meldung heisst nicht, dass die restlichen Auftraege keinen
    /// Treffer tragen: er heisst, dass sie nicht entschieden sind.
    pub fn befunde(&self) -> &Receiver<Befundmeldung> {
        &self.befunde
    }

    /// Bricht den Durchlauf ab.
    ///
    /// Der Arbeitsfaden bemerkt es an der naechsten Stapelgrenze und meldet
    /// nichts mehr. Bereits gesendete Befunde bleiben gueltig.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }
}

impl Drop for Durchlauf {
    /// Fordert den Abbruch an, wartet aber nicht auf den Faden.
    ///
    /// Warten hiesse, dass ein getipptes Zeichen auf den Durchlauf des vorigen
    /// Filtertexts wartet. Der Faden endet von selbst: entweder bemerkt er das
    /// Abbruchkennzeichen, oder sein naechstes Senden scheitert, weil der
    /// Empfaenger mit dem `Durchlauf` gefallen ist.
    fn drop(&mut self) {
        self.abbrechen();
    }
}

/// Arbeitet die Auftraege der Reihe nach ab und meldet je einen Befund.
///
/// Endet ohne weitere Meldung, sobald der Abbruch greift oder der Empfaenger
/// verschwunden ist. Die Reihenfolge ist die der Liste; keine Zusage haengt an
/// ihr, und ein Ordner mit grossem Unterbaum ohne Treffer verzoegert die nach
/// ihm.
fn durchlauffaden(
    auftraege: &[Auftrag],
    ordner: &Path,
    filter_klein: &str,
    abbruch: &AtomicBool,
    sender: &SyncSender<Befundmeldung>,
) {
    for auftrag in auftraege {
        let pfad = ordner.join(&auftrag.name);
        let Some(treffer) = unterbaum_entscheiden(&pfad, filter_klein, abbruch) else {
            return;
        };
        let meldung = Befundmeldung {
            index: auftrag.index,
            treffer,
        };
        if sender.send(meldung).is_err() {
            return;
        }
    }
}

/// Schreitet den Unterbaum ab, bis der erste Treffer faellt oder nichts mehr
/// offen ist.
///
/// `None` heisst abgebrochen, und das ist der eine Ausgang, der keinen Befund
/// traegt. `Some(true)` ist der Treffer, `Some(false)` der negative Befund; er
/// entsteht auf drei Wegen, und alle drei stehen unten im Rumpf: eine
/// symbolische Verknuepfung, ein Ordner, der sich nicht oeffnen laesst, und ein
/// abgeschrittener Unterbaum ohne Fund. Keiner der drei haelt den Durchlauf an,
/// und keiner erzeugt eine Meldung ueber den Befund hinaus.
fn unterbaum_entscheiden(wurzel: &Path, filter_klein: &str, abbruch: &AtomicBool) -> Option<bool> {
    // Erster Zweig: eine symbolische Verknuepfung ist ohne Lesen entschieden.
    //
    // Gefragt wird mit `symlink_metadata`, also `lstat(2)`, und das ist hier
    // die richtige Frage und keine Rueckkehr zur Pfadpruefung, die der Defekt
    // `260809-1652` abgeschafft hat: jene fragte nach dem Typ, um danach zu
    // oeffnen, und blockierte an einer benannten Roehre. `lstat(2)` oeffnet
    // nichts und kann deshalb nicht blockieren, und die Frage „ist der Pfad
    // selbst eine Verknuepfung?" ist an einem Deskriptor gar nicht zu stellen:
    // wer oeffnet, ist ihr schon gefolgt.
    if ist_verknuepfung(wurzel) {
        return Some(false);
    }

    // Zweiter Zweig: was sich nicht oeffnen laesst, ist ebenfalls entschieden.
    let Ok(leser) = Schwungleser::oeffnen(wurzel) else {
        return Some(false);
    };

    let mut ebenen = vec![Ebene::neu(leser, wurzel.to_path_buf())];
    while let Some(ebene) = ebenen.last_mut() {
        if let Some(kandidat) = ebene.stapel.next() {
            if traegt_die_folge(&kandidat.name, filter_klein) {
                // Der erste Treffer entscheidet den Auftrag, in welcher Tiefe
                // er auch liegt. Die offenen Ebenen fallen mit `ebenen`, und
                // der Rest unter ihnen bleibt ungelesen.
                return Some(true);
            }
            // „Ist es ein Ordner?" beantwortet auch eine Verknuepfung auf einen
            // Ordner mit ja; es ist derselbe Schnitt, den die Sichtbarkeit
            // zieht. Erst der Zweig danach trennt die beiden, und er steht am
            // Kopf dieser Funktion fuer den Auftrag und hier fuer den Abstieg:
            // in eine Verknuepfung wird nicht abgestiegen, sie traegt damit
            // nichts bei.
            if kandidat.typ == Typ::Ordner {
                let pfad = ebene.pfad.join(&kandidat.name);
                // Ein Unterordner, der sich nicht oeffnen laesst, ist
                // uebergangen und nicht gemeldet: derselbe Zweig wie oben, nur
                // eine Ebene tiefer.
                if let Ok(leser) = Schwungleser::oeffnen(&pfad) {
                    ebenen.push(Ebene::neu(leser, pfad));
                }
            }
            continue;
        }

        // Der Stapel ist aufgebraucht. Hier und nur hier steht die
        // Abbruchgrenze, und sie gilt auch fuer einen Ordner, der kein einziges
        // Mal absteigt.
        if abbruch.load(Ordering::Relaxed) {
            return None;
        }

        // Ein leerer Stapel und ein Lesefehler mitten im Ordner enden beide
        // gleich: diese Ebene ist fertig. Ein Fehler beim Weiterlesen sagt
        // dasselbe wie ein Fehler beim Oeffnen, naemlich dass von hier nichts
        // mehr zu holen ist, und er haelt den Durchlauf ebenso wenig an.
        match ebene.stapel_holen() {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                ebenen.pop();
            }
        }
    }

    // Dritter Zweig: abgeschritten, kein Treffer gefunden.
    Some(false)
}

/// Ob der Pfad selbst eine symbolische Verknuepfung ist.
///
/// Ein Pfad, den es nicht gibt, ist keine; er scheitert dann am Oeffnen und
/// braucht keinen eigenen Zweig.
fn ist_verknuepfung(pfad: &Path) -> bool {
    std::fs::symlink_metadata(pfad).is_ok_and(|angaben| angaben.file_type().is_symlink())
}

/// Der Vergleich, den auch die Sichtbarkeit zieht: Teilzeichenfolge an jeder
/// Stelle, ohne Ruecksicht auf die Schreibung, ohne Umlautfaltung.
///
/// `filter_klein` ist bereits kleingeschrieben; umgeschrieben wird hier nur der
/// gelesene Name. **Dieselbe Regel steht heute in `Ordnermodell::sichtbar`**,
/// und dass sie zweimal dasteht, ist ein bekannter Uebergangszustand: Schritt
/// A2 des Plans zieht den Vergleich als reine Funktion nach
/// `verzeichnis::filter`, und diese Aufrufstelle zieht dann mit.
fn traegt_die_folge(name: &str, filter_klein: &str) -> bool {
    name.to_lowercase().contains(filter_klein)
}

/// Ein Eintrag, wie der Durchlauf ihn braucht.
///
/// Name und Typ und sonst nichts: Groesse, Aenderungszeit und vor allem die
/// beiden Sortierschluessel braucht niemand hier, und der Schluessel kostet je
/// Eintrag einen Gang durch die Kollation. Deshalb entsteht kein
/// [`Eintrag`](super::eintrag::Eintrag).
#[derive(Debug)]
struct Kandidat {
    name: String,
    typ: Typ,
}

/// Ein Ordner, der gerade gelesen wird, mit seiner Stelle im Bestand.
///
/// Der Abstieg legt eine neue Ebene oben auf, die Rueckkehr nimmt sie weg. Der
/// [`Schwungleser`] der uebergeordneten Ebene bleibt dabei offen, denn nach der
/// Rueckkehr geht es in ihrem Stapel weiter.
struct Ebene {
    leser: Schwungleser,
    /// Der Ordner dieser Ebene, um die Namen ihrer Eintraege anzuhaengen.
    pfad: PathBuf,
    /// Der Stapel, der gerade abgearbeitet wird.
    stapel: std::vec::IntoIter<Kandidat>,
    /// Was der Leser darueber hinaus geliefert hat.
    ///
    /// Ein Schwung des Lesers ist nicht auf [`STAPELGROESSE`] begrenzt: ein
    /// einziger Systemaufruf liefert je nach Namenslaenge mehrere tausend
    /// Eintraege. Der Stapel ist es, denn an seiner Grenze haengt die
    /// Abbruchzusage; was darueber hinausgeht, wartet hier auf den naechsten
    /// Griff.
    vorrat: Vec<Kandidat>,
    /// Wahr, sobald der Leser das Ende des Ordners gemeldet hat.
    erschoepft: bool,
}

impl Ebene {
    fn neu(leser: Schwungleser, pfad: PathBuf) -> Self {
        Self {
            leser,
            pfad,
            stapel: Vec::new().into_iter(),
            vorrat: Vec::with_capacity(STAPELGROESSE),
            erschoepft: false,
        }
    }

    /// Holt den naechsten Stapel von hoechstens [`STAPELGROESSE`] Eintraegen.
    ///
    /// `Ok(false)` heisst: der Ordner ist zu Ende, diese Ebene ist fertig.
    fn stapel_holen(&mut self) -> io::Result<bool> {
        let Self {
            leser,
            vorrat,
            erschoepft,
            ..
        } = self;
        while !*erschoepft && vorrat.len() < STAPELGROESSE {
            let geliefert = leser.naechster_schwung(|roh| {
                vorrat.push(Kandidat {
                    name: roh.name.into_owned(),
                    typ: roh.typ,
                });
            })?;
            if geliefert == 0 {
                *erschoepft = true;
            }
        }
        let anzahl = self.vorrat.len().min(STAPELGROESSE);
        let stapel: Vec<Kandidat> = self.vorrat.drain(..anzahl).collect();
        let traegt_eintraege = !stapel.is_empty();
        self.stapel = stapel.into_iter();
        Ok(traegt_eintraege)
    }
}
