//! Der Zugang zu einem Server: ein Gespraech je Server und Prozess, und das
//! Register, ueber das ein zweiter Aufbau den lebenden Zugang findet
//! (Entscheidung 3 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
//!
//! ```text
//!  Faden krk-verbinden (mit &Fernfaden)
//!    Zugangsregister::zugang(kennung, vorbereiten)
//!      lebt ein Zugang und ist offen?     ──> derselbe Arc
//!      lebt einer und ist getrennt?       ──> vorbereiten(), neues Gespraech hinein
//!      lebt keiner?                       ──> vorbereiten(), Fernzugang::aufbauen
//!    Fernzugang::startordner(startpfad)   ──> REALPATH, STAT
//! ```
//!
//! **Ein Zugang lebt, solange jemand ein `Arc` auf ihn haelt**: ein Tab, ein
//! Lesevorgang, ein Vorgang. Das Register haelt allein ein `Weak` und haelt ihn
//! damit nicht am Leben. Faellt der letzte `Arc`, faellt das Gespraech, und mit
//! ihm die Standardeingabe von `ssh`; `ssh` endet darauf, und der
//! Gespraechsfaden erntet den Prozess. **Das Fallenlassen wartet auf nichts**
//! und ist deshalb auch auf dem Hauptfaden erlaubt (Modulkopf von
//! [`super::gespraech`]).
//!
//! **Neu aufgebaut wird nie still**, denn ein Aufbau kann die PIN und damit
//! den Nutzer brauchen: er geht ueber denselben Klick auf die Verbindung, und
//! [`Zugangsregister::zugang`] setzt dann ein neues Gespraech in den
//! getrennten Zugang ein, sodass jeder, der ihn haelt, das neue nutzt.
//!
//! **Das Register gehoert dem Faden `krk-verbinden`.** Der Hauptfaden haelt eine
//! Abschrift des Griffs, um sie dem Faden mitzugeben, und fasst die Tabelle
//! darin nie an; jede Methode, die sie liest, verlangt ein [`Fernfaden`].
//!
//! # Was dieses Modul nicht haelt
//!
//! Zwei Klicks zugleich auf zwei Verbindungen zum selben Server bauten zwei
//! Zugaenge, und der zweite verdraengte den ersten aus dem Register. Dass das
//! nicht geschieht, haelt der Anwendungsdelegierte, der hoechstens einen Gang
//! an `secrets.txt` zugleich laufen laesst, und nicht dieses Modul.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::adresse::{Serverkennung, Startpfad};
use super::faden::Fernfaden;
use super::gespraech::{Fernfehler, Gespraech, Trennung, Trennungsgrund, Zustand};
use super::kanal::Kanalplan;
use super::pfad::Fernpfad;

/// Ein gesprochenes Gespraech und das Anmeldeverzeichnis, das es beim Aufbau
/// genannt hat.
struct Verbunden {
    gespraech: Arc<Gespraech>,
    anmeldeverzeichnis: Fernpfad,
}

/// Der Zugang zu einem Server.
///
/// Nicht `Clone`: geteilt wird er als `Arc`, und Gleichheit zweier Orte geht
/// ueber [`Fernzugang::kennung`] und nie ueber den Zeiger.
pub struct Fernzugang {
    kennung: Serverkennung,
    verbindung: String,
    stand: Mutex<Verbunden>,
}

impl std::fmt::Debug for Fernzugang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fernzugang")
            .field("kennung", &self.kennung)
            .field("verbindung", &self.verbindung)
            .finish_non_exhaustive()
    }
}

/// Ein vergifteter Mutex wird uebergangen, wie in [`super::gespraech`].
fn sperren<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(wache) => wache,
        Err(vergiftet) => vergiftet.into_inner(),
    }
}

/// Wohin eine Verbindung zeigt, wenn sie steht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Startordner {
    /// Der Ordner, aufgeloest auf dem Server.
    pub pfad: Fernpfad,
    /// Der verlangte Ordner, wenn er fehlte oder kein Ordner war; [`Self::pfad`]
    /// ist dann das Anmeldeverzeichnis.
    pub fehlte: Option<String>,
}

impl Fernzugang {
    /// Startet den Kanal nach `plan`, eroeffnet das Gespraech und fragt das
    /// Anmeldeverzeichnis.
    ///
    /// `verbindung` ist der Name der Verbindung in `secrets.txt`, fuer
    /// Meldungen. Scheitert irgendetwas, ist der Kindprozess beendet und
    /// geerntet, und der Fehler traegt die Fehlerausgabe.
    pub fn aufbauen(
        faden: &Fernfaden,
        kennung: Serverkennung,
        verbindung: &str,
        plan: &Kanalplan,
    ) -> Result<Arc<Fernzugang>, Fernfehler> {
        let verbunden = verbinden(faden, &kennung, plan)?;
        Ok(Arc::new(Fernzugang {
            kennung,
            verbindung: verbindung.to_owned(),
            stand: Mutex::new(verbunden),
        }))
    }

    /// Die Kennung des Servers.
    #[must_use]
    pub fn kennung(&self) -> &Serverkennung {
        &self.kennung
    }

    /// Der Name der Verbindung, mit dem der Zugang aufgebaut wurde.
    #[must_use]
    pub fn verbindung(&self) -> &str {
        &self.verbindung
    }

    /// Das Gespraech, das gerade gilt, als geteilter Griff.
    ///
    /// Ein Wiederaufbau setzt ein neues ein; wer den alten Griff haelt,
    /// spricht weiter mit dem alten und bekommt [`Fernfehler::Getrennt`].
    #[must_use]
    pub fn gespraech(&self) -> Arc<Gespraech> {
        Arc::clone(&sperren(&self.stand).gespraech)
    }

    /// Das Anmeldeverzeichnis, wie der Server es beim letzten Aufbau nannte.
    #[must_use]
    pub fn anmeldeverzeichnis(&self) -> Fernpfad {
        sperren(&self.stand).anmeldeverzeichnis.clone()
    }

    /// Ob das Gespraech getrennt ist.
    #[must_use]
    pub fn getrennt(&self) -> bool {
        matches!(self.gespraech().zustand(), Zustand::Getrennt(_))
    }

    /// Baut ein neues Gespraech nach `plan` und setzt es ein.
    ///
    /// Scheitert der Aufbau, bleibt das getrennte Gespraech stehen.
    pub fn neu_verbinden(&self, faden: &Fernfaden, plan: &Kanalplan) -> Result<(), Fernfehler> {
        let verbunden = verbinden(faden, &self.kennung, plan)?;
        *sperren(&self.stand) = verbunden;
        Ok(())
    }

    /// Loest den Startpfad auf: `REALPATH`, dann `STAT`; ist der verlangte
    /// Ordner nicht da oder kein Ordner, gilt das Anmeldeverzeichnis, und
    /// [`Startordner::fehlte`] nennt den verlangten (Entscheidung 7 des
    /// Plans).
    ///
    /// Ein Fehler kommt allein von einem getrennten Gespraech oder einer
    /// Antwort, die das Protokoll verletzt; ein Statuscode des Servers heisst
    /// hier, dass der Ordner fehlt.
    pub fn startordner(
        &self,
        faden: &Fernfaden,
        startpfad: &Startpfad,
    ) -> Result<Startordner, Fernfehler> {
        let anmeldeverzeichnis = self.anmeldeverzeichnis();
        let verlangt = match startpfad {
            Startpfad::Anmeldeverzeichnis => {
                return Ok(Startordner {
                    pfad: anmeldeverzeichnis,
                    fehlte: None,
                });
            }
            Startpfad::UnterAnmeldeverzeichnis(rest) => rest.clone(),
            Startpfad::Absolut(pfad) => pfad.als_text().to_owned(),
        };
        let gespraech = self.gespraech();
        let gefunden = match gespraech.realpath(faden, &verlangt) {
            Ok(bytes) => pfad_aus_bytes(bytes)?,
            Err(Fernfehler::Status { .. }) => None,
            Err(fehler) => return Err(fehler),
        };
        let ordner = match gefunden {
            Some(pfad) => match gespraech.stat(faden, pfad.als_text()) {
                Ok(attribute) if attribute.ist_ordner() => Some(pfad),
                Ok(_) | Err(Fernfehler::Status { .. }) => None,
                Err(fehler) => return Err(fehler),
            },
            None => None,
        };
        Ok(match ordner {
            Some(pfad) => Startordner { pfad, fehlte: None },
            None => Startordner {
                pfad: anmeldeverzeichnis,
                fehlte: Some(verlangt),
            },
        })
    }
}

/// Startet den Kanal, eroeffnet das Gespraech und fragt `REALPATH(".")`.
fn verbinden(
    faden: &Fernfaden,
    kennung: &Serverkennung,
    plan: &Kanalplan,
) -> Result<Verbunden, Fernfehler> {
    let kind = plan.starten().map_err(|fehler| {
        Fernfehler::Getrennt(Trennung {
            grund: Trennungsgrund::Aufbau(format!("{}: {fehler}", plan.programm.display())),
            fehlerausgabe: Vec::new(),
            ausgang: None,
        })
    })?;
    let gespraech = Gespraech::eroeffnen(faden, kind, kennung.rechner())?;
    let anmeldeverzeichnis = pfad_aus_bytes(gespraech.realpath(faden, ".")?)?.ok_or_else(|| {
        Fernfehler::Protokoll("REALPATH(\".\") nennt keinen absoluten Pfad".to_owned())
    })?;
    Ok(Verbunden {
        gespraech: Arc::new(gespraech),
        anmeldeverzeichnis,
    })
}

/// Ein Pfad aus der Antwort auf `REALPATH`: kein UTF-8 ist ein Verstoss gegen
/// die Einschraenkung von [`Fernpfad`], ein relativer Pfad heisst `None`.
fn pfad_aus_bytes(bytes: Vec<u8>) -> Result<Option<Fernpfad>, Fernfehler> {
    let text = String::from_utf8(bytes)
        .map_err(|_| Fernfehler::Protokoll("REALPATH nennt einen Pfad ohne UTF-8".to_owned()))?;
    Ok(Fernpfad::aus_text(&text))
}

/// Warum [`Zugangsregister::zugang`] keinen Zugang liefert.
#[derive(Debug)]
pub enum Zugangsfehler<E> {
    /// Die Vorbereitung des Rufers hat abgebrochen, bevor ein Kanal startete.
    Vorbereitung(E),
    /// Der Aufbau ist gescheitert.
    Fern(Fernfehler),
}

/// Ob ein Zugang neu entstand, ein getrennter neu verbunden wurde oder ein
/// lebender genommen wurde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunft {
    /// Ein lebender, offener Zugang.
    Lebend,
    /// Ein getrennter Zugang mit neuem Gespraech.
    NeuVerbunden,
    /// Ein neuer Zugang.
    Neu,
}

/// Die Zugaenge dieses Prozesses, je Server einer, als `Weak`.
///
/// Ein Griff; Abschriften teilen dieselbe Tabelle.
#[derive(Clone, Default)]
pub struct Zugangsregister {
    tabelle: Arc<Mutex<HashMap<Serverkennung, Weak<Fernzugang>>>>,
}

impl Zugangsregister {
    /// Ein leeres Register.
    #[must_use]
    pub fn neu() -> Self {
        Self::default()
    }

    /// Der Zugang zum Server `kennung`: ein lebender offener, sonst ein
    /// getrennter mit neuem Gespraech, sonst ein neuer.
    ///
    /// `vorbereiten` laeuft allein, wenn ein Kanal zu starten ist, und
    /// liefert den Plan und einen Wert, der bis zum Ende des Aufbaus leben
    /// muss, etwa die Anmeldehilfe; er faellt, sobald das Gespraech steht
    /// oder gescheitert ist. Ein lebender Zugang braucht keine Vorbereitung,
    /// also auch kein Passwort.
    pub fn zugang<H, E>(
        &self,
        faden: &Fernfaden,
        kennung: &Serverkennung,
        verbindung: &str,
        vorbereiten: impl FnOnce() -> Result<(Kanalplan, H), E>,
    ) -> Result<(Arc<Fernzugang>, Herkunft), Zugangsfehler<E>> {
        let lebend = sperren(&self.tabelle).get(kennung).and_then(Weak::upgrade);
        if let Some(zugang) = &lebend
            && !zugang.getrennt()
        {
            return Ok((Arc::clone(zugang), Herkunft::Lebend));
        }
        let (plan, halten) = vorbereiten().map_err(Zugangsfehler::Vorbereitung)?;
        let ergebnis = match lebend {
            Some(zugang) => zugang
                .neu_verbinden(faden, &plan)
                .map(|()| (zugang, Herkunft::NeuVerbunden)),
            None => Fernzugang::aufbauen(faden, kennung.clone(), verbindung, &plan)
                .map(|zugang| (zugang, Herkunft::Neu)),
        };
        drop(halten);
        let (zugang, herkunft) = ergebnis.map_err(Zugangsfehler::Fern)?;
        let mut tabelle = sperren(&self.tabelle);
        tabelle.retain(|_, schwach| schwach.strong_count() > 0);
        tabelle.insert(kennung.clone(), Arc::downgrade(&zugang));
        Ok((zugang, herkunft))
    }
}
