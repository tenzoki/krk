//! Der gestueckelte Lesevorgang.
//!
//! Ein Arbeitsfaden liest das Verzeichnis und schickt Stapel zu 1.024
//! Eintraegen an den Hauptfaden. Der erste Stapel fuellt die Dateiliste und
//! traegt damit die Zusage L2 ("erste Bildschirmseite sichtbar und bedienbar");
//! die weiteren haengen an, und erst der Abschluss stellt Sortierung und
//! Bildlaufleiste fest (L3, L10).
//!
//! Zwei Entwurfsentscheidungen tragen den Rest:
//!
//! **Die Generationsnummer** liegt jedem Stapel bei, und sie ist heute kleiner
//! zugeschnitten als bei ihrer Einfuehrung. Gedacht war sie als Filter am
//! Hauptfaden: wer schnell durch Ordner navigiert, hat mehrere Lesevorgaenge
//! unterwegs, und der Hauptfaden sollte jeden Stapel verwerfen, dessen
//! Generation nicht mehr die aktuelle ist. Dieser Filter konnte nie greifen und
//! ist am 260803-2025 entfernt worden
//! (`issues/260803-1536_*_die-generationspruefung-kann-nicht-greifen-und-verdeckt-den-wirksamen-mechanismus.md`):
//! jeder Tab haelt seinen eigenen [`Lesevorgang`] und liest allein aus dessen
//! Kanal, ein Ordnerwechsel wirft den alten Vorgang samt Kanal weg, und damit
//! kann ein fremder Stapel gar nicht erst ankommen.
//!
//! Was die Nummer noch traegt, sind zwei Dinge: sie benennt den Lesefaden
//! (`krk-verzeichnisleser-<n>`), was ein Fadenprotokoll lesbar macht, und sie
//! sagt [`Ordnermodell::leeren`](super::modell::Ordnermodell::leeren), zu
//! welchem Lauf der Inhalt des Modells gehoert. Beides sind Aufgaben des
//! **Aufrufers**, der die Nummer ohnehin haelt; der Lesevorgang und die Meldung
//! mussten sie ihm deshalb nie zurueckgeben, und die Leser dafuer sind am
//! 260805-0841 entfallen
//! (`issues/260803-2025_*_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md`).
//! Sie liegt der Meldung weiter bei, weil `krk-bench` sie dort liest.
//!
//! **Der Kanal hat eine Kapazitaet von einem Stapel.** Ein unbegrenzter Kanal
//! wuerde bei einem Ordner mit 100.000 Eintraegen den ganzen Bestand ein
//! zweites Mal im Speicher halten, und ein Abbruch traefe einen Faden, der
//! laengst fertig ist. Mit der Kapazitaet 1 laeuft der Leser hoechstens zwei
//! Stapel vor, der Abbruch greift also innerhalb von zwei Stapeln.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::{self, JoinHandle};

use super::eintrag::Eintrag;
use super::sys::Schwungleser;

/// Wie viele Eintraege ein Stapel traegt.
pub const STAPELGROESSE: usize = 1024;

/// Wie viele fertige Stapel im Kanal warten duerfen.
const KANALTIEFE: usize = 1;

/// Wie ein Lesevorgang geendet hat.
#[derive(Debug)]
pub enum Abschluss {
    /// Das Verzeichnis ist vollstaendig gelesen.
    Vollstaendig,
    /// Der Lesevorgang wurde abgebrochen. Die bis dahin gesendeten Stapel sind
    /// gueltig, der Bestand ist unvollstaendig.
    Abgebrochen,
    /// Der Lesevorgang ist gescheitert. Bereits gesendete Stapel sind gueltig.
    Fehler(io::Error),
}

impl Abschluss {
    /// Wahr, wenn der Lesevorgang abgebrochen wurde.
    pub fn ist_abgebrochen(&self) -> bool {
        matches!(self, Abschluss::Abgebrochen)
    }

    /// Wahr, wenn das Verzeichnis vollstaendig gelesen ist.
    pub fn ist_vollstaendig(&self) -> bool {
        matches!(self, Abschluss::Vollstaendig)
    }
}

/// Was der Arbeitsfaden an den Hauptfaden schickt.
#[derive(Debug)]
pub enum Meldung {
    /// Ein Stapel gelesener Eintraege. Der letzte Stapel eines Laufs kann
    /// weniger als [`STAPELGROESSE`] Eintraege tragen.
    Stapel {
        /// Die Generation, zu der dieser Stapel gehoert.
        generation: u64,
        /// Die Eintraege in Lesereihenfolge.
        eintraege: Vec<Eintrag>,
    },
    /// Der Lauf ist zu Ende. Kommt genau einmal je Lesevorgang und immer nach
    /// allen Stapeln desselben Laufs.
    Fertig {
        /// Die Generation, zu der dieser Lauf gehoert.
        generation: u64,
        /// Wie der Lauf geendet hat.
        abschluss: Abschluss,
    },
}

/// Ein laufender Lesevorgang auf einem eigenen Faden.
pub struct Lesevorgang {
    abbruch: Arc<AtomicBool>,
    meldungen: Receiver<Meldung>,
    faden: Option<JoinHandle<()>>,
}

impl Lesevorgang {
    /// Startet den Lesevorgang und kehrt sofort zurueck.
    pub fn starten(pfad: impl Into<PathBuf>, generation: u64) -> Self {
        let pfad = pfad.into();
        let abbruch = Arc::new(AtomicBool::new(false));
        let (sender, meldungen) = sync_channel(KANALTIEFE);
        let faden_abbruch = Arc::clone(&abbruch);
        let faden = thread::Builder::new()
            .name(format!("krk-verzeichnisleser-{generation}"))
            .spawn(move || lesefaden(&pfad, generation, &faden_abbruch, &sender))
            .expect("Arbeitsfaden fuer den Verzeichnisleser laesst sich nicht starten");
        Self {
            abbruch,
            meldungen,
            faden: Some(faden),
        }
    }

    /// Der Kanal, aus dem der Hauptfaden die Stapel holt.
    pub fn meldungen(&self) -> &Receiver<Meldung> {
        &self.meldungen
    }

    /// Bricht den Lesevorgang ab.
    ///
    /// Der Arbeitsfaden bemerkt den Abbruch vor dem naechsten Stapel und meldet
    /// ihn als [`Abschluss::Abgebrochen`]. Bereits gesendete Stapel bleiben
    /// gueltig.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }

    /// Wartet, bis der Arbeitsfaden geendet hat.
    ///
    /// Nur fuer Aufrufer, die den Kanal schon leergeraeumt haben; wer vorher
    /// wartet, verklemmt sich am Kanal mit der Tiefe 1.
    pub fn warten(mut self) {
        if let Some(faden) = self.faden.take() {
            let _ = faden.join();
        }
    }
}

impl Drop for Lesevorgang {
    /// Fordert den Abbruch an, wartet aber nicht auf den Faden.
    ///
    /// Warten hiesse, dass eine Navigation auf den Leser des verlassenen
    /// Ordners wartet. Der Faden endet von selbst: entweder bemerkt er das
    /// Abbruchkennzeichen, oder sein naechstes Senden scheitert, weil der
    /// Empfaenger weg ist.
    fn drop(&mut self) {
        self.abbrechen();
    }
}

/// Liest ein Verzeichnis auf dem aufrufenden Faden vollstaendig ein.
///
/// Fuer Tests und fuer Aufrufer, die ohnehin warten muessen. Der gestueckelte
/// Weg ueber [`Lesevorgang`] und dieser hier benutzen denselben Leser.
pub fn lesen(pfad: &Path) -> io::Result<Vec<Eintrag>> {
    let mut leser = Schwungleser::oeffnen(pfad)?;
    let mut eintraege = Vec::new();
    loop {
        let geliefert = leser.naechster_schwung(|roh| eintraege.push(Eintrag::aus_roh(roh)))?;
        if geliefert == 0 {
            return Ok(eintraege);
        }
    }
}

fn lesefaden(pfad: &Path, generation: u64, abbruch: &AtomicBool, sender: &SyncSender<Meldung>) {
    if let Some(abschluss) = lesen_und_senden(pfad, generation, abbruch, sender) {
        let _ = sender.send(Meldung::Fertig {
            generation,
            abschluss,
        });
    }
}

/// Liest und sendet, bis das Verzeichnis zu Ende ist, der Abbruch greift oder
/// ein Fehler auftritt.
///
/// Liefert `None`, wenn der Empfaenger verschwunden ist: dann gibt es niemanden
/// mehr, dem eine Abschlussmeldung nuetzt.
fn lesen_und_senden(
    pfad: &Path,
    generation: u64,
    abbruch: &AtomicBool,
    sender: &SyncSender<Meldung>,
) -> Option<Abschluss> {
    let mut leser = match Schwungleser::oeffnen(pfad) {
        Ok(leser) => leser,
        Err(fehler) => return Some(Abschluss::Fehler(fehler)),
    };

    let mut gesammelt: Vec<Eintrag> = Vec::with_capacity(STAPELGROESSE);
    loop {
        if abbruch.load(Ordering::Relaxed) {
            rest_senden(generation, sender, &mut gesammelt)?;
            return Some(Abschluss::Abgebrochen);
        }

        let ergebnis = leser.naechster_schwung(|roh| gesammelt.push(Eintrag::aus_roh(roh)));
        match ergebnis {
            Ok(0) => {
                rest_senden(generation, sender, &mut gesammelt)?;
                return Some(Abschluss::Vollstaendig);
            }
            Ok(_) => {}
            Err(fehler) => {
                rest_senden(generation, sender, &mut gesammelt)?;
                return Some(Abschluss::Fehler(fehler));
            }
        }

        // Ein einziger Systemaufruf liefert je nach Namenslaenge mehrere
        // tausend Eintraege. Der Abbruch wird deshalb auch zwischen zwei
        // Stapeln geprueft und nicht nur zwischen zwei Systemaufrufen; sonst
        // liefe ein abgebrochener Ordner mit 5.000 Eintraegen trotzdem
        // vollstaendig durch.
        while gesammelt.len() >= STAPELGROESSE {
            if abbruch.load(Ordering::Relaxed) {
                return Some(Abschluss::Abgebrochen);
            }
            let rest = gesammelt.split_off(STAPELGROESSE);
            let voller_stapel = std::mem::replace(&mut gesammelt, rest);
            sender
                .send(Meldung::Stapel {
                    generation,
                    eintraege: voller_stapel,
                })
                .ok()?;
        }
    }
}

/// Schickt den angefangenen Stapel, falls er nicht leer ist.
fn rest_senden(
    generation: u64,
    sender: &SyncSender<Meldung>,
    gesammelt: &mut Vec<Eintrag>,
) -> Option<()> {
    if gesammelt.is_empty() {
        return Some(());
    }
    let eintraege = std::mem::take(gesammelt);
    sender
        .send(Meldung::Stapel {
            generation,
            eintraege,
        })
        .ok()
}
