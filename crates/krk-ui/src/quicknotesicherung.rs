//! Die Sicherung der Quicknote in `quicknote.txt`, ohne AppKit (Schritt 3 des
//! Plans `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`;
//! der Dateiname und die Uebernahme der frueheren Datei aus Schritt 2 des Plans
//! `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`).
//!
//! ```text
//!  Editorbereich (Hauptfaden)                 Faden krk-quicknote
//!  ───────────────────────────                ───────────────────
//!  Sicherung::starten ── Lesen ─────────────> alte Datei uebernehmen,
//!                                             stat, Stempel, lesen
//!                                               │
//!  einziehen() <── Uebernahme (hoechstens ────┤
//!                  einmal, davor)             │
//!  einziehen() <── Gelesen / Fehlt / ─────────┘
//!                  NichtGelesen
//!  schreiben(text) ── Schreiben ────────────> juengsten nehmen, Stempel
//!                                             vergleichen, sichern
//!  einziehen() <── Gesichert / OrdnerFehlt / ─┘
//!                  FremdGeaendert / Gescheitert
//!  abschliessen(2 s) ── Abschliessen ───────> Quittung nach allem davor
//! ```
//!
//! **Diese Datei ist der eine Schreiber von `quicknote.txt`.** Den Pfad bekommt
//! sie hereingereicht, gebaut ueber `Heimordner::quicknotedatei`; die Probe
//! `quicknote_txt_hat_genau_einen_schreiber` im Pruefmodul haelt das, soweit eine
//! Nadel es kann.
//!
//! # Erst gelesen, dann geschrieben
//!
//! [`Sicherung::starten`] gibt dem Faden als ersten Auftrag `Lesen`, und
//! derselbe Faden bedient danach die Schreibauftraege; dass nichts vor dem
//! Lesen geschrieben wird, ist damit eine Eigenschaft der Reihenfolge. **Dazu
//! nimmt [`Sicherung::schreiben`] keinen Auftrag an, bevor [`Sicherung::einziehen`]
//! die Antwort auf das Lesen abgeholt hat**: sonst koennte ein Text, der vor dem
//! Einsetzen des gelesenen Inhalts entstand (der leere Puffer), die Datei
//! ueberschreiben, obwohl der Faden sie zuerst gelesen hat.
//!
//! # Die fruehere Datei wird vor dem ersten Lesen uebernommen
//!
//! Bis zum 261111 hiess die Datei anders (`heimordner::ALTE_QUICKNOTEDATEI`).
//! **Unmittelbar vor dem ersten Lesen, auf diesem Faden, gibt
//! `heimordner::ohne_ersetzen_umbenennen` einer Datei unter dem frueheren
//! Namen den neuen**: ueber einen harten Verweis, der an einer vorhandenen
//! `quicknote.txt` scheitert, und ohne Pruefung davor. Eine vorhandene
//! `quicknote.txt` wird damit nie ersetzt, auch nicht, wenn sie in diesem
//! Augenblick entsteht (C2.4 des Spec
//! `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`). Was
//! danach gelesen wird, sagt die Tafel aus E3 des Plans:
//!
//! ```text
//!  Ausgang                    gezeigt wird               auf der Platte
//!  ─────────────────────────  ─────────────────────────  ─────────────────────────
//!  keine fruehere Datei       quicknote.txt oder leer    unveraendert
//!  Umbenannt                  der Inhalt, jetzt unter    dieselbe Inode, der alte
//!                             quicknote.txt              Name ist weg
//!  BeideStehen                quicknote.txt              beide unveraendert
//!  AlterNameBleibt            quicknote.txt              zwei Namen, eine Inode
//!  Gescheitert                der Inhalt der frueheren   die fruehere Datei
//!                             Datei                      unveraendert; quicknote.txt
//!                                                        entsteht mit dem ersten
//!                                                        Sichern exklusiv
//! ```
//!
//! Der letzte Fall ist ein Notizordner auf einem Dateisystem ohne harte
//! Verweise: `quicknote.txt` gilt dem Faden dann als fehlend, das erste Sichern
//! legt sie exklusiv an, und die fruehere Datei schreibt dieser Faden nie.
//! **Steht `quicknote.txt` bei einem gescheiterten Verweis doch**, etwa weil
//! unter dem frueheren Namen ein Ordner liegt, wird sie gelesen wie in der
//! Zeile `BeideStehen`; die Tafel nennt den Fall nicht, und so bleibt eine
//! lesbare `quicknote.txt` nicht hinter einem unlesbaren frueheren Namen
//! verborgen. Jeder Ausgang ausser dem ersten meldet sich einmal
//! ([`Sicherungsmeldung::Uebernahme`]), vor der Antwort auf das Lesen, und
//! sperrt nichts. Ein zweiter Leseauftrag benennt nichts um.
//!
//! # Wann nicht mehr gesichert wird
//!
//! - **Von aussen geaendert** ([`Sicherungsmeldung::FremdGeaendert`]): vor
//!   jedem Schreiben vergleicht der Faden den Stempel der Datei mit dem nach
//!   dem Lesen oder dem letzten eigenen Schreiben; weicht er ab, oder ist die
//!   Datei da, wo sie fehlen sollte, schreibt er nicht und sperrt sich bis zum
//!   Ende des Prozesses. Die Datei bleibt unberuehrt.
//! - **Nicht lesbar** ([`Sicherungsmeldung::NichtGelesen`]): zu gross, kein
//!   UTF-8, keine gewoehnliche Datei, ein Fehler. Die Quicknote oeffnet leer,
//!   und der Faden sperrt sich ebenso.
//! - **Der Notizordner fehlt** ([`Sicherungsmeldung::OrdnerFehlt`]): das sperrt
//!   nicht. Geschrieben wird nichts und kein Ordner angelegt, denn allein F2
//!   legt ihn an und entscheidet dabei ueber die Uebernahme der alten Zettel;
//!   die naechste Aenderung, das Verlassen und das Beenden versuchen es
//!   wieder ([`Vormerkung`]). Ohne Heimordner (kein
//!   Benutzerverzeichnis) gilt dasselbe.
//!
//! Jede Meldung kommt einmal je Uebergang und nicht je Auftrag: eine
//! gesperrte Sicherung schweigt, ein fehlender Ordner meldet sich beim ersten
//! vergeblichen Versuch, ein gescheitertes Schreiben beim ersten in Folge.
//!
//! # Wie geschrieben wird
//!
//! Ueber `datei::sichern` mit `Zeilenenden::WieGelesen` und ohne Marke, also
//! Byte fuer Byte und ohne angehaengtes `\n`, ueber `ablage::atomar`. **Fehlt
//! die Datei, legt der Faden sie zuerst exklusiv mit den Rechten `0600` an**;
//! `atomar` uebertraegt die Rechte einer bestehenden Datei auf jede spaetere
//! Fassung. Die Quicknote nimmt eingefuegten Text auch aus `secrets.txt` an,
//! und eine fuer jedes Konto lesbare Datei waere ein Leck. Scheitert das
//! Anlegen an `EEXIST`, ist die Datei von anderer Hand entstanden, und es gilt
//! die Sperre.
//!
//! Warten tut allein [`Sicherung::abschliessen`], beim Beenden und mit einer
//! Hoechstzeit; jeder andere Weg ist auf dem Hauptfaden ohne Systemaufruf.
//!
//! # Wann der Editorbereich einen Auftrag gibt
//!
//! Die Vormerk- und Abgabelogik steht in [`Vormerkung`] und nicht im
//! AppKit-Teil, damit eine Probe sie ohne Flaeche faehrt. **„Faellig“ und
//! „ungesichert“ sind zwei Felder**: faellig ist eine Sicherung
//! `QUICKNOTE_RUHE` nach der letzten Aenderung, und das faellt mit der Abgabe;
//! ungesichert ist der Text seit der letzten Aenderung, und das faellt erst,
//! wenn der Faden fuer den Auftrag mit dem juengsten Text `Gesichert` gemeldet
//! hat ([`Sicherung::gesichert_bis`]). Verlassen und Beenden geben einen
//! Auftrag, solange der Text ungesichert ist, auch ohne Vormerkung; nach
//! `OrdnerFehlt` oder `Gescheitert` landet der letzte Stand damit, sobald die
//! Ursache behoben ist, und nicht erst nach einer weiteren Aenderung. Bis zum
//! 261031 teilten beide Zustaende ein Feld, und das Beenden nach F2 verlor den
//! Text.

use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use krk_core::heimordner::{self, Umbenennung};
use krk_core::text::datei::{self, Dateiform, Lesehindernis, Platzhalterregel, Zeilenenden};

use crate::editormodell::Stempel;

/// Die Form, in der `quicknote.txt` geschrieben wird: der Puffer Byte fuer Byte.
const FORM: Dateiform = Dateiform {
    marke: false,
    zeilenenden: Zeilenenden::WieGelesen,
};

/// Die Rechte einer neu angelegten `quicknote.txt`: allein das eigene Konto.
const RECHTE: u32 = 0o600;

/// Was der Faden meldet. **Vollstaendig und ohne Auffangzweig** zu behandeln.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sicherungsmeldung {
    /// `quicknote.txt` war lesbar; der Text in gehaltener Form.
    Gelesen(String),
    /// `quicknote.txt` fehlt (oder es gibt keinen Heimordner): ein leerer Puffer.
    Fehlt,
    /// `quicknote.txt` liess sich nicht lesen; die Sicherung ist gesperrt.
    NichtGelesen(Unlesbar),
    /// Der juengste Text steht auf der Platte.
    Gesichert,
    /// Der Notizordner fehlt; nichts geschrieben, der naechste Auftrag
    /// versucht es wieder.
    OrdnerFehlt,
    /// Die Datei wurde ausserhalb der Quicknote geaendert; die Sicherung ist
    /// gesperrt.
    FremdGeaendert,
    /// Das Schreiben ist an etwas anderem gescheitert; der Grund vom System.
    Gescheitert(String),
    /// Was aus einer Datei unter dem frueheren Namen geworden ist (Modulkopf,
    /// „Die fruehere Datei wird vor dem ersten Lesen uebernommen“). Kommt
    /// hoechstens einmal je Prozess, vor der Antwort auf das Lesen, und
    /// aendert den Stand nicht: sie sperrt nichts.
    Uebernahme(Umbenennung),
}

/// Warum `quicknote.txt` sich nicht lesen liess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unlesbar {
    /// Ein Grund der Leseschleuse, vollstaendig aus `Lesehindernis`.
    Hindernis(Lesehindernis),
    /// Gelesen, aber kein gueltiges UTF-8.
    KeinText,
}

/// Wo die Sicherung steht, so wie der Hauptfaden es aus den abgeholten
/// Meldungen weiss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sicherungsstand {
    /// Die Antwort auf das Lesen ist noch nicht abgeholt.
    Ungelesen,
    /// Gelesen oder als fehlend erkannt; Aenderungen werden gesichert.
    Bereit,
    /// Der Notizordner fehlte beim letzten Versuch; Aenderungen werden weiter
    /// versucht.
    OrdnerFehlt,
    /// Bis zum Ende des Prozesses wird nicht mehr gesichert.
    Gesperrt,
}

impl Sicherungsstand {
    /// Ob eine Aenderung einen Schreibauftrag bekommen soll.
    #[must_use]
    pub fn nimmt_auftraege(self) -> bool {
        match self {
            Sicherungsstand::Bereit | Sicherungsstand::OrdnerFehlt => true,
            Sicherungsstand::Ungelesen | Sicherungsstand::Gesperrt => false,
        }
    }
}

/// Ein Auftrag an den Faden.
enum Auftrag {
    Lesen,
    Schreiben(String),
    /// Quittiert, sobald alles davor erledigt ist.
    Abschliessen(SyncSender<()>),
}

/// Eine Antwort des Fadens: eine Meldung oder keine, und wie viele
/// Schreibauftraege mit ihr erledigt sind (zusammengefasste eingerechnet).
struct Antwort {
    meldung: Option<Sicherungsmeldung>,
    erledigt: usize,
}

/// Die Sicherung der Quicknote: die Kanaele zum Faden `krk-quicknote` und
/// was der Hauptfaden aus seinen Antworten weiss.
pub struct Sicherung {
    auftraege: Sender<Auftrag>,
    antworten: Receiver<Antwort>,
    stand: std::cell::Cell<Sicherungsstand>,
    /// Gesendete Schreibauftraege, deren Antwort noch nicht abgeholt ist.
    unterwegs: std::cell::Cell<usize>,
    /// Wie viele Schreibauftraege gesendet sind; die Nummer des juengsten.
    gesendet: std::cell::Cell<u64>,
    /// Wie viele Schreibauftraege beantwortet sind, zusammengefasste
    /// eingerechnet.
    beantwortet: std::cell::Cell<u64>,
    /// Bis zu welcher Auftragsnummer der Text auf der Platte steht: der Stand
    /// von `beantwortet` bei der letzten Meldung `Gesichert`.
    gesichert_bis: std::cell::Cell<u64>,
}

impl Sicherung {
    /// Startet den Faden fuer `pfad` und gibt ihm als ersten Auftrag `Lesen`.
    ///
    /// `pfad` ist `None`, wenn es keinen Heimordner gibt; dann meldet das
    /// Lesen [`Sicherungsmeldung::Fehlt`] und jedes Schreiben
    /// [`Sicherungsmeldung::OrdnerFehlt`]. `frueher` ist der Pfad der Datei
    /// unter ihrem frueheren Namen, die der Faden vor dem ersten Lesen
    /// uebernimmt; ohne `pfad` oder ohne `frueher` wird nichts uebernommen.
    /// `grenze` ist die Grenze des Puffers, `QUICKNOTEGRENZE`.
    pub fn starten(
        pfad: Option<PathBuf>,
        frueher: Option<PathBuf>,
        grenze: u64,
    ) -> io::Result<Self> {
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        thread::Builder::new()
            .name("krk-quicknote".to_owned())
            .spawn(move || {
                faden(
                    pfad.as_deref(),
                    frueher.as_deref(),
                    grenze,
                    &eingang,
                    &ausgang,
                );
            })?;
        let sicherung = Self::aus_kanaelen(auftraege, antworten);
        // Der Faden lebt, also nimmt der Kanal den Auftrag an; scheitert es
        // trotzdem, bleibt die Sicherung ungelesen und nimmt nichts an.
        let _ = sicherung.auftraege.send(Auftrag::Lesen);
        Ok(sicherung)
    }

    fn aus_kanaelen(auftraege: Sender<Auftrag>, antworten: Receiver<Antwort>) -> Self {
        Self {
            auftraege,
            antworten,
            stand: std::cell::Cell::new(Sicherungsstand::Ungelesen),
            unterwegs: std::cell::Cell::new(0),
            gesendet: std::cell::Cell::new(0),
            beantwortet: std::cell::Cell::new(0),
            gesichert_bis: std::cell::Cell::new(0),
        }
    }

    /// Wo die Sicherung nach den abgeholten Meldungen steht.
    #[must_use]
    pub fn stand(&self) -> Sicherungsstand {
        self.stand.get()
    }

    /// Ob ein Schreibauftrag gesendet und seine Antwort noch nicht abgeholt
    /// ist; solange das gilt, braucht der Editorbereich seinen Takt.
    #[must_use]
    pub fn unterwegs(&self) -> bool {
        self.unterwegs.get() > 0
    }

    /// Gibt dem Faden den ganzen Text zum Sichern.
    ///
    /// Antwortet, ob der Auftrag angenommen ist: nein, solange die Antwort auf
    /// das Lesen nicht abgeholt ist, wenn die Sicherung gesperrt ist und wenn
    /// der Faden nicht mehr lebt.
    #[must_use = "ein abgewiesener Auftrag heisst, dass dieser Stand nicht auf die Platte kommt"]
    pub fn schreiben(&self, text: String) -> bool {
        if !self.stand.get().nimmt_auftraege() {
            return false;
        }
        if self.auftraege.send(Auftrag::Schreiben(text)).is_err() {
            return false;
        }
        self.unterwegs.set(self.unterwegs.get() + 1);
        self.gesendet.set(self.gesendet.get() + 1);
        true
    }

    /// Die Nummer des zuletzt angenommenen Schreibauftrags; 0 vor dem ersten.
    #[must_use]
    pub fn gesendet(&self) -> u64 {
        self.gesendet.get()
    }

    /// Bis zu welcher Auftragsnummer der Text nach den abgeholten Meldungen auf
    /// der Platte steht. Der Faden bedient die Auftraege in ihrer Reihenfolge
    /// und schreibt von wartenden den juengsten, also deckt ein `Gesichert`
    /// jeden Auftrag bis zu dem, den es beantwortet.
    #[must_use]
    pub fn gesichert_bis(&self) -> u64 {
        self.gesichert_bis.get()
    }

    /// Holt die Meldungen des Fadens ab, ohne zu warten, und fuehrt den Stand
    /// nach.
    #[must_use = "die Meldungen sind die Auskunft an den Nutzer und das gelesene Ergebnis"]
    pub fn einziehen(&self) -> Vec<Sicherungsmeldung> {
        let mut meldungen = Vec::new();
        loop {
            match self.antworten.try_recv() {
                Ok(antwort) => {
                    self.unterwegs
                        .set(self.unterwegs.get().saturating_sub(antwort.erledigt));
                    self.beantwortet
                        .set(self.beantwortet.get() + antwort.erledigt as u64);
                    if antwort.meldung == Some(Sicherungsmeldung::Gesichert) {
                        self.gesichert_bis.set(self.beantwortet.get());
                    }
                    if let Some(meldung) = antwort.meldung {
                        self.stand.set(stand_nach(self.stand.get(), &meldung));
                        meldungen.push(meldung);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // Ein Faden, der nicht mehr antwortet, sichert nichts mehr;
                    // was unterwegs war, kommt nicht mehr an.
                    self.unterwegs.set(0);
                    self.stand.set(Sicherungsstand::Gesperrt);
                    break;
                }
            }
        }
        meldungen
    }

    /// Wartet hoechstens `warten`, bis der Faden jeden Auftrag davor erledigt
    /// hat; fuer das Beenden. Antwortet, ob die Quittung kam.
    #[must_use = "ohne Quittung ist der letzte Stand vielleicht nicht auf der Platte"]
    pub fn abschliessen(&self, warten: Duration) -> bool {
        let (quittung, eingang) = mpsc::sync_channel(1);
        if self
            .auftraege
            .send(Auftrag::Abschliessen(quittung))
            .is_err()
        {
            return false;
        }
        eingang.recv_timeout(warten).is_ok()
    }
}

/// Wann der Editorbereich der [`Sicherung`] einen Schreibauftrag gibt
/// (Modulkopf, „Wann der Editorbereich einen Auftrag gibt“); ohne AppKit und
/// ohne Uhr, die Zeit kommt herein.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vormerkung {
    /// Wann die naechste Abgabe am Takt faellig ist.
    faellig: Option<Instant>,
    /// Ob der Text seit der letzten Aenderung nicht bestaetigt auf der Platte
    /// steht.
    ungesichert: bool,
    /// Die Nummer des Auftrags mit dem juengsten Text, solange seine Antwort
    /// aussteht und keine Aenderung dazwischenkam.
    offen: Option<u64>,
}

impl Vormerkung {
    /// Eine Aenderung am Text: ungesichert, und faellig `ruhe` nach `jetzt`.
    /// Nimmt die Sicherung keine Auftraege, wird nichts vorgemerkt.
    pub fn geaendert(&mut self, sicherung: &Sicherung, jetzt: Instant, ruhe: Duration) {
        if !sicherung.stand().nimmt_auftraege() {
            return;
        }
        self.ungesichert = true;
        self.offen = None;
        self.faellig = Some(jetzt + ruhe);
    }

    /// Am Takt: ist die Vormerkung faellig, geht der Text als Auftrag.
    pub fn am_takt(
        &mut self,
        sicherung: &Sicherung,
        jetzt: Instant,
        text: impl FnOnce() -> String,
    ) {
        if self.faellig.is_some_and(|faellig| jetzt >= faellig) {
            self.abgeben(sicherung, text);
        }
    }

    /// Beim Verlassen und beim Beenden: solange der Text ungesichert ist,
    /// geht er als Auftrag, gleich ob eine Vormerkung laeuft.
    pub fn sofort(&mut self, sicherung: &Sicherung, text: impl FnOnce() -> String) {
        if self.ungesichert {
            self.abgeben(sicherung, text);
        }
    }

    fn abgeben(&mut self, sicherung: &Sicherung, text: impl FnOnce() -> String) {
        self.faellig = None;
        if sicherung.schreiben(text()) {
            self.offen = Some(sicherung.gesendet());
        }
    }

    /// Nach dem Abholen der Meldungen: steht der Auftrag mit dem juengsten
    /// Text auf der Platte, ist der Text gesichert.
    pub fn nach_dem_einziehen(&mut self, sicherung: &Sicherung) {
        if self
            .offen
            .is_some_and(|nummer| sicherung.gesichert_bis() >= nummer)
        {
            self.ungesichert = false;
            self.offen = None;
        }
    }

    /// Ob eine Abgabe am Takt aussteht.
    #[must_use]
    pub fn wartet(&self) -> bool {
        self.faellig.is_some()
    }
}

/// Der Stand, den eine Meldung herstellt; vollstaendig und ohne Auffangzweig.
fn stand_nach(vorher: Sicherungsstand, meldung: &Sicherungsmeldung) -> Sicherungsstand {
    match meldung {
        Sicherungsmeldung::Gelesen(_) | Sicherungsmeldung::Fehlt | Sicherungsmeldung::Gesichert => {
            Sicherungsstand::Bereit
        }
        Sicherungsmeldung::OrdnerFehlt => Sicherungsstand::OrdnerFehlt,
        Sicherungsmeldung::NichtGelesen(_) | Sicherungsmeldung::FremdGeaendert => {
            Sicherungsstand::Gesperrt
        }
        // Ein gescheitertes Schreiben laesst die Datei, wie sie war; der
        // naechste Auftrag versucht es wieder. Der Ausgang der Uebernahme
        // kommt vor der Antwort auf das Lesen und sagt ueber den Stand nichts.
        Sicherungsmeldung::Gescheitert(_) | Sicherungsmeldung::Uebernahme(_) => vorher,
    }
}

/// Was der Faden ueber die Datei weiss.
enum Zustand {
    Ungelesen,
    /// `stempel` ist der Stempel nach dem Lesen oder dem letzten eigenen
    /// Schreiben, `None` fuer eine Datei, die fehlt.
    Bereit {
        stempel: Option<Stempel>,
        ordner_fehlte: bool,
        gescheitert: bool,
    },
    Gesperrt,
}

/// Der Rumpf des Fadens: bedient die Auftraege in ihrer Reihenfolge und fasst
/// wartende Schreibauftraege zum juengsten zusammen.
fn faden(
    pfad: Option<&Path>,
    frueher: Option<&Path>,
    grenze: u64,
    eingang: &Receiver<Auftrag>,
    ausgang: &Sender<Antwort>,
) {
    let mut zustand = Zustand::Ungelesen;
    let mut naechster: Option<Auftrag> = None;
    loop {
        let auftrag = match naechster.take() {
            Some(auftrag) => auftrag,
            None => match eingang.recv() {
                Ok(auftrag) => auftrag,
                Err(_) => return,
            },
        };
        let antwort = match auftrag {
            Auftrag::Lesen => {
                // Allein vor dem ersten Lesen: ein zweiter Leseauftrag
                // benennt nichts um und liest nichts.
                let uebernahme = matches!(zustand, Zustand::Ungelesen)
                    .then(|| uebernehmen(pfad, frueher))
                    .flatten();
                if let Some(ausgang_der_uebernahme) = &uebernahme {
                    let antwort = Antwort {
                        meldung: Some(Sicherungsmeldung::Uebernahme(
                            ausgang_der_uebernahme.clone(),
                        )),
                        erledigt: 0,
                    };
                    if ausgang.send(antwort).is_err() {
                        return;
                    }
                }
                Antwort {
                    meldung: lesen(pfad, frueher, uebernahme.as_ref(), grenze, &mut zustand),
                    erledigt: 0,
                }
            }
            Auftrag::Schreiben(mut text) => {
                let mut erledigt = 1;
                loop {
                    match eingang.try_recv() {
                        Ok(Auftrag::Schreiben(juenger)) => {
                            text = juenger;
                            erledigt += 1;
                        }
                        Ok(anderer) => {
                            naechster = Some(anderer);
                            break;
                        }
                        Err(_) => break,
                    }
                }
                Antwort {
                    meldung: schreiben(pfad, &text, &mut zustand),
                    erledigt,
                }
            }
            Auftrag::Abschliessen(quittung) => {
                let _ = quittung.send(());
                continue;
            }
        };
        if ausgang.send(antwort).is_err() {
            return;
        }
    }
}

/// Gibt einer Datei unter dem frueheren Namen den Namen `quicknote.txt`, ohne
/// je eine vorhandene zu ersetzen. `None`, wenn es keine fruehere Datei gibt,
/// der Notizordner fehlt oder einer der zwei Pfade nicht bekannt ist.
fn uebernehmen(pfad: Option<&Path>, frueher: Option<&Path>) -> Option<Umbenennung> {
    heimordner::ohne_ersetzen_umbenennen(frueher?, pfad?)
}

/// Ob an `pfad` nichts steht.
fn fehlt(pfad: &Path) -> bool {
    std::fs::metadata(pfad).is_err_and(|fehler| fehler.kind() == ErrorKind::NotFound)
}

/// Liest `quicknote.txt` (E3 des Plans der Sicherung) nach dem Ausgang der
/// Uebernahme (Modulkopf, die Tafel). Ein zweiter Leseauftrag liest nichts.
///
/// Allein nach einem gescheiterten Verweis und ohne `quicknote.txt` wird die
/// Datei unter dem frueheren Namen gelesen; `quicknote.txt` gilt dann als
/// fehlend (kein Stempel), sodass das erste Sichern sie exklusiv anlegt und
/// an einer inzwischen entstandenen scheitert, statt sie zu ersetzen.
fn lesen(
    pfad: Option<&Path>,
    frueher: Option<&Path>,
    uebernahme: Option<&Umbenennung>,
    grenze: u64,
    zustand: &mut Zustand,
) -> Option<Sicherungsmeldung> {
    if !matches!(zustand, Zustand::Ungelesen) {
        return None;
    }
    let als_fehlend = |zustand: &mut Zustand| {
        *zustand = Zustand::Bereit {
            stempel: None,
            ordner_fehlte: false,
            gescheitert: false,
        };
        Some(Sicherungsmeldung::Fehlt)
    };
    let Some(pfad) = pfad else {
        return als_fehlend(zustand);
    };
    let aus_der_frueheren = match (uebernahme, frueher) {
        (Some(Umbenennung::Gescheitert(_)), Some(frueher)) if fehlt(pfad) => Some(frueher),
        (
            None
            | Some(
                Umbenennung::Umbenannt
                | Umbenennung::BeideStehen
                | Umbenennung::AlterNameBleibt(_)
                | Umbenennung::Gescheitert(_),
            ),
            _,
        ) => None,
    };
    let pfad = aus_der_frueheren.unwrap_or(pfad);
    if fehlt(pfad) {
        return als_fehlend(zustand);
    }
    // Der Stempel vor dem Lesen, aus dem Grund an `Ladevorgang::starten`: eine
    // Aenderung waehrend des Lesens laesst ihn aelter sein als den Inhalt und
    // faellt beim ersten Schreiben auf. Aus der frueheren Datei gelesen, gibt
    // es keinen: der Stempel gilt `quicknote.txt`, und die fehlt.
    let stempel = match aus_der_frueheren {
        Some(_) => None,
        None => Stempel::von_pfad(pfad),
    };
    let gelesen = datei::bis_zur_grenze_lesen(pfad, grenze, Platzhalterregel::Laden, &|| true)
        .map_err(Unlesbar::Hindernis)
        .and_then(|bytes| datei::einlesen(bytes).ok_or(Unlesbar::KeinText));
    match gelesen {
        Ok(text) => {
            *zustand = Zustand::Bereit {
                stempel,
                ordner_fehlte: false,
                gescheitert: false,
            };
            Some(Sicherungsmeldung::Gelesen(text))
        }
        Err(grund) => {
            *zustand = Zustand::Gesperrt;
            Some(Sicherungsmeldung::NichtGelesen(grund))
        }
    }
}

/// Sichert `text` (E5, E6) und antwortet mit der Meldung des Uebergangs.
fn schreiben(pfad: Option<&Path>, text: &str, zustand: &mut Zustand) -> Option<Sicherungsmeldung> {
    let Zustand::Bereit {
        stempel,
        ordner_fehlte,
        gescheitert,
    } = zustand
    else {
        return None;
    };
    let ausgang = match pfad {
        Some(pfad) => sichern(pfad, text, stempel),
        None => Ausgang::OrdnerFehlt,
    };
    match ausgang {
        Ausgang::Gesichert => {
            *ordner_fehlte = false;
            *gescheitert = false;
            Some(Sicherungsmeldung::Gesichert)
        }
        Ausgang::OrdnerFehlt => {
            let neu = !*ordner_fehlte;
            *ordner_fehlte = true;
            neu.then_some(Sicherungsmeldung::OrdnerFehlt)
        }
        Ausgang::FremdGeaendert => {
            *zustand = Zustand::Gesperrt;
            Some(Sicherungsmeldung::FremdGeaendert)
        }
        Ausgang::Gescheitert(fehler) => {
            let neu = !*gescheitert;
            *gescheitert = true;
            neu.then(|| Sicherungsmeldung::Gescheitert(fehler.to_string()))
        }
    }
}

/// Der Ausgang eines Schreibversuchs.
enum Ausgang {
    Gesichert,
    OrdnerFehlt,
    FremdGeaendert,
    Gescheitert(io::Error),
}

/// Ein Schreibversuch an `pfad`: Stempel vergleichen, wenn noetig exklusiv
/// anlegen, sichern, neuen Stempel merken.
fn sichern(pfad: &Path, text: &str, stempel: &mut Option<Stempel>) -> Ausgang {
    let jetzt = Stempel::von_pfad(pfad);
    match (stempel.as_ref(), jetzt) {
        (Some(bekannt), Some(jetzt)) if bekannt.gleicht(&jetzt) => {}
        (Some(_), Some(_)) => return Ausgang::FremdGeaendert,
        (Some(_), None) => {
            // Die bekannte Datei ist fort: weil der Ordner fort ist, oder weil
            // eine andere Hand sie entfernt hat.
            return if ordner_fehlt(pfad) {
                Ausgang::OrdnerFehlt
            } else {
                Ausgang::FremdGeaendert
            };
        }
        (None, Some(_)) => return Ausgang::FremdGeaendert,
        (None, None) => match exklusiv_anlegen(pfad) {
            Ok(()) => *stempel = Stempel::von_pfad(pfad),
            Err(fehler) if fehler.kind() == ErrorKind::AlreadyExists => {
                return Ausgang::FremdGeaendert;
            }
            Err(fehler) if fehler.kind() == ErrorKind::NotFound => return Ausgang::OrdnerFehlt,
            Err(fehler) => return Ausgang::Gescheitert(fehler),
        },
    }
    if let Err(fehler) = datei::sichern(pfad, text, FORM) {
        return if fehler.kind() == ErrorKind::NotFound {
            Ausgang::OrdnerFehlt
        } else {
            Ausgang::Gescheitert(fehler)
        };
    }
    *stempel = Stempel::von_pfad(pfad);
    Ausgang::Gesichert
}

/// Legt `pfad` leer und exklusiv mit [`RECHTE`] an; einen Ordner legt dieser
/// Weg nie an.
fn exklusiv_anlegen(pfad: &Path) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(RECHTE)
        .open(pfad)
        .map(drop)
}

/// Ob der Ordner von `pfad` fehlt.
fn ordner_fehlt(pfad: &Path) -> bool {
    pfad.parent().is_none_or(|ordner| {
        std::fs::metadata(ordner).is_err_and(|fehler| fehler.kind() == ErrorKind::NotFound)
    })
}

#[cfg(test)]
mod proben {
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::pruefordner::Pruefordner;
    use crate::quellbaum::{codezeilen, quelldateien};

    const GRENZE: u64 = 1024;

    /// Wartet, bis der Faden mindestens eine Meldung geschickt hat, und holt
    /// alle bis dahin eingetroffenen ab.
    fn abholen(sicherung: &Sicherung) -> Vec<Sicherungsmeldung> {
        let beginn = Instant::now();
        loop {
            let meldungen = sicherung.einziehen();
            if !meldungen.is_empty() {
                return meldungen;
            }
            assert!(
                beginn.elapsed() < Duration::from_secs(5),
                "der Faden antwortet nicht"
            );
            thread::sleep(Duration::from_millis(2));
        }
    }

    /// Schreibt `text` und wartet auf die Quittung des Fadens.
    fn sichern_und_warten(sicherung: &Sicherung, text: &str) -> Vec<Sicherungsmeldung> {
        assert!(sicherung.schreiben(text.to_owned()), "Auftrag abgewiesen");
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        sicherung.einziehen()
    }

    fn gestartet(pfad: &Path) -> Sicherung {
        Sicherung::starten(Some(pfad.to_path_buf()), None, GRENZE).expect("Faden startet")
    }

    fn rechte(pfad: &Path) -> u32 {
        std::fs::metadata(pfad)
            .expect("Datei steht")
            .permissions()
            .mode()
            & 0o777
    }

    #[test]
    fn eine_fehlende_datei_ergibt_einen_leeren_puffer_und_entsteht_beim_ersten_sichern_mit_0600() {
        let ordner = Pruefordner::neu("qn-fehlt");
        let pfad = ordner.unter("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert!(!pfad.exists(), "das Lesen legt nichts an");

        assert_eq!(
            sichern_und_warten(&sicherung, "erste Notiz"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"erste Notiz");
        assert_eq!(rechte(&pfad), 0o600);
        assert!(!sicherung.unterwegs());
    }

    #[test]
    fn ein_gelesener_text_kommt_byte_fuer_byte_zurueck_ohne_angehaengten_umbruch() {
        let ordner = Pruefordner::neu("qn-rundlauf");
        let pfad = ordner.unter("quicknote.txt");
        let text = "Gr\u{fc}\u{df}e\nzweite Zeile ohne Schluss";

        let erste = gestartet(&pfad);
        assert_eq!(abholen(&erste), vec![Sicherungsmeldung::Fehlt]);
        assert_eq!(
            sichern_und_warten(&erste, text),
            vec![Sicherungsmeldung::Gesichert]
        );
        drop(erste);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), text.as_bytes());

        // Ein zweiter Wert wie nach einem Neustart.
        let zweite = gestartet(&pfad);
        assert_eq!(
            abholen(&zweite),
            vec![Sicherungsmeldung::Gelesen(text.to_owned())]
        );
        assert_eq!(
            sichern_und_warten(&zweite, text),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), text.as_bytes());
    }

    #[test]
    fn vor_der_antwort_auf_das_lesen_wird_nichts_geschrieben() {
        let ordner = Pruefordner::neu("qn-reihenfolge");
        let pfad = ordner.datei("quicknote.txt", "vorhanden");
        let sicherung = gestartet(&pfad);
        // Die Antwort auf das Lesen ist nicht abgeholt: kein Auftrag.
        assert_eq!(sicherung.stand(), Sicherungsstand::Ungelesen);
        assert!(!sicherung.schreiben(String::new()));
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"vorhanden");
        assert_eq!(
            sicherung.einziehen(),
            vec![Sicherungsmeldung::Gelesen("vorhanden".to_owned())]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::Bereit);
    }

    #[test]
    fn wartende_auftraege_schreiben_nur_den_juengsten() {
        let ordner = Pruefordner::neu("qn-juengster");
        let pfad = ordner.unter("quicknote.txt");
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        let sicherung = Sicherung::aus_kanaelen(auftraege, antworten);
        // Die Auftraege stehen schon an, bevor der Faden laeuft.
        let _ = sicherung.auftraege.send(Auftrag::Lesen);
        let fuer_faden = pfad.clone();
        let faden_griff = thread::spawn(move || {
            faden(Some(fuer_faden.as_path()), None, GRENZE, &eingang, &ausgang)
        });
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);

        // Ob der Faden zwischen den drei Auftraegen aufwacht, entscheidet der
        // Planer; geprueft wird hier deshalb, dass am Ende der juengste Text
        // steht und jeder Auftrag erledigt ist. Das Zusammenfassen selbst
        // haelt die Probe darunter ohne Planer.
        for text in ["eins", "zwei", "drei"] {
            assert!(sicherung.schreiben(text.to_owned()));
        }
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        let meldungen = sicherung.einziehen();
        assert!(!meldungen.is_empty());
        assert!(
            meldungen
                .iter()
                .all(|meldung| *meldung == Sicherungsmeldung::Gesichert)
        );
        assert!(!sicherung.unterwegs(), "jeder Auftrag ist erledigt");
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
        drop(sicherung);
        faden_griff.join().expect("der Faden endet ohne Panik");
    }

    #[test]
    fn wartende_auftraege_werden_zu_einem_zusammengefasst() {
        // Dieselbe Eigenschaft ohne Planer: die Auftraege stehen im Kanal,
        // bevor der Faden ueberhaupt startet.
        let ordner = Pruefordner::neu("qn-zusammen");
        let pfad = ordner.unter("quicknote.txt");
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        for auftrag in [
            Auftrag::Lesen,
            Auftrag::Schreiben("eins".to_owned()),
            Auftrag::Schreiben("zwei".to_owned()),
            Auftrag::Schreiben("drei".to_owned()),
        ] {
            let _ = auftraege.send(auftrag);
        }
        drop(auftraege);
        faden(Some(pfad.as_path()), None, GRENZE, &eingang, &ausgang);
        let gesammelt: Vec<(Option<Sicherungsmeldung>, usize)> = antworten
            .try_iter()
            .map(|antwort| (antwort.meldung, antwort.erledigt))
            .collect();
        assert_eq!(
            gesammelt,
            vec![
                (Some(Sicherungsmeldung::Fehlt), 0),
                (Some(Sicherungsmeldung::Gesichert), 3),
            ]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
    }

    #[test]
    fn ein_fehlender_ordner_schreibt_nichts_legt_nichts_an_und_sichert_sobald_er_steht() {
        let ordner = Pruefordner::neu("qn-ordner");
        let heim = ordner.unter("krkhome");
        let pfad = heim.join("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);

        assert_eq!(
            sichern_und_warten(&sicherung, "eins"),
            vec![Sicherungsmeldung::OrdnerFehlt]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::OrdnerFehlt);
        // Einmal je Uebergang: der zweite vergebliche Versuch schweigt.
        assert_eq!(sichern_und_warten(&sicherung, "zwei"), vec![]);
        assert!(!heim.exists(), "die Sicherung legt keinen Ordner an");

        std::fs::create_dir(&heim).expect("Ordner wie nach F2");
        assert_eq!(
            sichern_und_warten(&sicherung, "drei"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
        assert_eq!(sicherung.stand(), Sicherungsstand::Bereit);
    }

    /// Was der Takt des Editorbereichs nach jeder Abgabe tut, hier ohne Takt:
    /// auf den Faden warten, die Meldungen abholen und die Vormerkung
    /// nachfuehren.
    fn abholen_und_nachfuehren(
        sicherung: &Sicherung,
        vormerkung: &mut Vormerkung,
    ) -> Vec<Sicherungsmeldung> {
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        let meldungen = sicherung.einziehen();
        vormerkung.nach_dem_einziehen(sicherung);
        meldungen
    }

    const RUHE: Duration = Duration::from_secs(1);

    #[test]
    fn nach_fehlendem_ordner_sichert_der_abschluss_ohne_weitere_aenderung() {
        let ordner = Pruefordner::neu("qn-vormerkung-ordner");
        let heim = ordner.unter("krkhome");
        let pfad = heim.join("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        let mut vormerkung = Vormerkung::default();

        let beginn = Instant::now();
        vormerkung.geaendert(&sicherung, beginn, RUHE);
        vormerkung.am_takt(&sicherung, beginn + RUHE, || "der Text".to_owned());
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::OrdnerFehlt]
        );
        assert!(vormerkung.ungesichert, "nichts steht auf der Platte");
        assert!(!vormerkung.wartet(), "der Takt darf ruhen");

        std::fs::create_dir(&heim).expect("Ordner wie nach F2");
        // Der Abschluss beim Beenden, ohne weitere Aenderung.
        vormerkung.sofort(&sicherung, || "der Text".to_owned());
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"der Text");
        assert!(!vormerkung.ungesichert);
        // Danach gibt das Verlassen keinen weiteren Auftrag.
        let gesendet = sicherung.gesendet();
        vormerkung.sofort(&sicherung, || unreachable!("gesichert"));
        assert_eq!(sicherung.gesendet(), gesendet);
    }

    #[test]
    fn nach_gescheitertem_schreiben_sichert_der_abschluss_ohne_weitere_aenderung() {
        let ordner = Pruefordner::neu("qn-vormerkung-gescheitert");
        let heim = ordner.ordner("krkhome");
        let pfad = heim.join("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        let mut vormerkung = Vormerkung::default();

        // Ein Ordner ohne Schreibrecht laesst das Anlegen scheitern, ohne dass
        // der Ordner fehlt.
        let erlaubt = std::fs::metadata(&heim).expect("Ordner").permissions();
        std::fs::set_permissions(&heim, std::fs::Permissions::from_mode(0o500))
            .expect("Rechte entzogen");
        let beginn = Instant::now();
        vormerkung.geaendert(&sicherung, beginn, RUHE);
        vormerkung.am_takt(&sicherung, beginn + RUHE, || "der Text".to_owned());
        let meldungen = abholen_und_nachfuehren(&sicherung, &mut vormerkung);
        std::fs::set_permissions(&heim, erlaubt).expect("Rechte zurueck");
        assert!(
            matches!(meldungen.as_slice(), [Sicherungsmeldung::Gescheitert(_)]),
            "{meldungen:?}"
        );
        assert!(vormerkung.ungesichert);

        vormerkung.sofort(&sicherung, || "der Text".to_owned());
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"der Text");
        assert!(!vormerkung.ungesichert);
    }

    #[test]
    fn vor_der_ruhe_wird_nichts_abgegeben_und_danach_genau_einmal() {
        let ordner = Pruefordner::neu("qn-vormerkung-ruhe");
        let pfad = ordner.unter("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        let mut vormerkung = Vormerkung::default();

        let beginn = Instant::now();
        vormerkung.geaendert(&sicherung, beginn, RUHE);
        assert!(vormerkung.wartet());
        vormerkung.am_takt(&sicherung, beginn + RUHE / 2, || "eins".to_owned());
        assert_eq!(sicherung.gesendet(), 0, "vor der Ruhe geht nichts");
        vormerkung.am_takt(&sicherung, beginn + RUHE, || "eins".to_owned());
        assert_eq!(sicherung.gesendet(), 1);
        vormerkung.am_takt(&sicherung, beginn + RUHE * 3, || "eins".to_owned());
        assert_eq!(sicherung.gesendet(), 1, "genau einmal");
        assert!(!vormerkung.wartet());
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert!(!vormerkung.ungesichert);
    }

    #[test]
    fn eine_aenderung_nach_der_abgabe_bleibt_ungesichert_bis_ihr_auftrag_gesichert_ist() {
        let ordner = Pruefordner::neu("qn-vormerkung-spaet");
        let pfad = ordner.unter("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        let mut vormerkung = Vormerkung::default();

        let beginn = Instant::now();
        vormerkung.geaendert(&sicherung, beginn, RUHE);
        vormerkung.am_takt(&sicherung, beginn + RUHE, || "eins".to_owned());
        // Getippt, waehrend der erste Auftrag unterwegs ist.
        vormerkung.geaendert(&sicherung, beginn + RUHE, RUHE);
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert!(vormerkung.ungesichert, "„zwei“ steht nicht auf der Platte");
        vormerkung.sofort(&sicherung, || "zwei".to_owned());
        assert_eq!(
            abholen_und_nachfuehren(&sicherung, &mut vormerkung),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"zwei");
        assert!(!vormerkung.ungesichert);
    }

    #[test]
    fn eine_gesperrte_sicherung_merkt_nichts_vor() {
        let ordner = Pruefordner::neu("qn-vormerkung-ungelesen");
        let pfad = ordner.datei("quicknote.txt", "vorhanden");
        let sicherung = gestartet(&pfad);
        let mut vormerkung = Vormerkung::default();
        // Vor dem Abholen der Antwort auf das Lesen nimmt die Sicherung nichts.
        vormerkung.geaendert(&sicherung, Instant::now(), RUHE);
        assert_eq!(vormerkung, Vormerkung::default());
        vormerkung.sofort(&sicherung, || unreachable!("nichts vorgemerkt"));
        assert_eq!(sicherung.gesendet(), 0);
    }

    #[test]
    fn ohne_heimordner_fehlt_die_datei_und_der_ordner() {
        let sicherung = Sicherung::starten(None, None, GRENZE).expect("Faden startet");
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert_eq!(
            sichern_und_warten(&sicherung, "eins"),
            vec![Sicherungsmeldung::OrdnerFehlt]
        );
    }

    #[test]
    fn eine_aenderung_von_aussen_sperrt_und_laesst_die_datei_unberuehrt() {
        let ordner = Pruefordner::neu("qn-fremd");
        let pfad = ordner.datei("quicknote.txt", "vorher");
        let sicherung = gestartet(&pfad);
        assert_eq!(
            abholen(&sicherung),
            vec![Sicherungsmeldung::Gelesen("vorher".to_owned())]
        );
        std::fs::write(&pfad, "von der zweiten Instanz").expect("geschrieben");

        assert_eq!(
            sichern_und_warten(&sicherung, "aus der Quicknote"),
            vec![Sicherungsmeldung::FremdGeaendert]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::Gesperrt);
        assert!(!sicherung.schreiben("noch einmal".to_owned()));
        assert_eq!(
            std::fs::read(&pfad).expect("gelesen"),
            b"von der zweiten Instanz"
        );
    }

    #[test]
    fn eine_datei_die_waehrend_der_fehlanzeige_entsteht_wird_nicht_ueberschrieben() {
        let ordner = Pruefordner::neu("qn-entsteht");
        let pfad = ordner.unter("quicknote.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        std::fs::write(&pfad, "fremd").expect("geschrieben");

        assert_eq!(
            sichern_und_warten(&sicherung, "eigen"),
            vec![Sicherungsmeldung::FremdGeaendert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"fremd");
    }

    #[test]
    fn das_exklusive_anlegen_weist_eine_vorhandene_datei_ab() {
        let ordner = Pruefordner::neu("qn-exklusiv");
        let pfad = ordner.datei("quicknote.txt", "fremd");
        let mut stempel = None;
        // Der Wettlauf zwischen Stempel und Anlegen, von Hand hergestellt: der
        // Faden glaubt die Datei fehlend, sie steht aber.
        assert!(matches!(
            exklusiv_anlegen(&pfad),
            Err(fehler) if fehler.kind() == ErrorKind::AlreadyExists
        ));
        assert!(matches!(
            sichern(&pfad, "eigen", &mut stempel),
            Ausgang::FremdGeaendert
        ));
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"fremd");
    }

    #[test]
    fn eine_zu_grosse_oder_nicht_lesbare_datei_sperrt_und_bleibt_byte_fuer_byte() {
        let ordner = Pruefordner::neu("qn-unlesbar");
        let gross = vec![b'x'; usize::try_from(GRENZE).expect("passt") + 1];
        let kein_text = vec![0xff, 0xfe, b'a'];
        for (name, inhalt, grund) in [
            (
                "gross.txt",
                gross,
                Unlesbar::Hindernis(Lesehindernis::ZuGross),
            ),
            ("binaer.txt", kein_text, Unlesbar::KeinText),
        ] {
            let pfad = ordner.datei(name, &inhalt);
            let sicherung = gestartet(&pfad);
            assert_eq!(
                abholen(&sicherung),
                vec![Sicherungsmeldung::NichtGelesen(grund)],
                "{name}"
            );
            assert_eq!(sicherung.stand(), Sicherungsstand::Gesperrt);
            assert!(!sicherung.schreiben("neu".to_owned()), "{name}");
            assert!(sicherung.abschliessen(Duration::from_secs(5)));
            assert_eq!(std::fs::read(&pfad).expect("gelesen"), inhalt, "{name}");
        }

        let roehre = ordner.roehre("quicknote.txt");
        let sicherung = gestartet(&roehre);
        assert_eq!(
            abholen(&sicherung),
            vec![Sicherungsmeldung::NichtGelesen(Unlesbar::Hindernis(
                Lesehindernis::KeineDatei
            ))]
        );
    }

    #[test]
    fn die_rechte_einer_vorhandenen_datei_bleiben() {
        let ordner = Pruefordner::neu("qn-rechte");
        let pfad = ordner.datei("quicknote.txt", "vorher");
        std::fs::set_permissions(&pfad, std::fs::Permissions::from_mode(0o640))
            .expect("Rechte gesetzt");
        let sicherung = gestartet(&pfad);
        let _ = abholen(&sicherung);
        assert_eq!(
            sichern_und_warten(&sicherung, "nachher"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(rechte(&pfad), 0o640);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"nachher");
    }

    #[test]
    fn abschliessen_wartet_hoechstens_die_genannte_zeit() {
        // Ein Faden, der nie antwortet: der Empfaenger lebt, bedient aber
        // nichts.
        let (auftraege, _eingang_haengt) = mpsc::channel();
        let (_ausgang, antworten) = mpsc::channel();
        let sicherung = Sicherung::aus_kanaelen(auftraege, antworten);
        let beginn = Instant::now();
        assert!(!sicherung.abschliessen(Duration::from_millis(100)));
        let gewartet = beginn.elapsed();
        assert!(gewartet >= Duration::from_millis(100), "{gewartet:?}");
        assert!(gewartet < Duration::from_secs(2), "{gewartet:?}");

        // Ein lebender Faden quittiert sofort.
        let ordner = Pruefordner::neu("qn-abschluss");
        let lebend = gestartet(&ordner.unter("quicknote.txt"));
        let beginn = Instant::now();
        assert!(lebend.abschliessen(Duration::from_secs(2)));
        assert!(beginn.elapsed() < Duration::from_secs(1));
    }

    // Die Uebernahme der Datei unter ihrem frueheren Namen (Modulkopf, die
    // Tafel; E3 des Plans
    // `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`). Der
    // fruehere Name steht hier als Literal: die Konstante des Kerns soll in
    // dieser Kiste keine Codezeile nennen.

    const FRUEHER: &str = "scratch.txt";
    const HEUTE: &str = "quicknote.txt";

    /// Startet die Sicherung mit beiden Pfaden, wie der Editorbereich es tut.
    fn mit_frueherer(ordner: &Pruefordner) -> (PathBuf, PathBuf, Sicherung) {
        let pfad = ordner.unter(HEUTE);
        let frueher = ordner.unter(FRUEHER);
        let sicherung = Sicherung::starten(Some(pfad.clone()), Some(frueher.clone()), GRENZE)
            .expect("Faden startet");
        (pfad, frueher, sicherung)
    }

    /// Holt die Meldungen ab, bis die Antwort auf das Lesen da ist; die
    /// Meldung der Uebernahme kommt davor und kann einen Zug frueher eintreffen.
    fn bis_gelesen(sicherung: &Sicherung) -> Vec<Sicherungsmeldung> {
        let beginn = Instant::now();
        let mut meldungen = Vec::new();
        loop {
            meldungen.extend(sicherung.einziehen());
            if sicherung.stand() != Sicherungsstand::Ungelesen {
                return meldungen;
            }
            assert!(
                beginn.elapsed() < Duration::from_secs(5),
                "der Faden antwortet nicht"
            );
            thread::sleep(Duration::from_millis(2));
        }
    }

    fn inode(pfad: &Path) -> u64 {
        use std::os::unix::fs::MetadataExt;
        std::fs::metadata(pfad).expect("Datei steht").ino()
    }

    /// Zeile 1 der Tafel: ohne fruehere Datei meldet die Uebernahme nichts,
    /// gelesen wird `quicknote.txt` oder nichts, und die Platte bleibt, wie sie
    /// war. Dazu C2.1: das erste Sichern legt `quicknote.txt` an und keine
    /// Datei unter dem frueheren Namen.
    #[test]
    fn ohne_fruehere_datei_meldet_die_uebernahme_nichts_und_gesichert_wird_als_quicknote_txt() {
        let leer = Pruefordner::neu("qn-ueb-keine");
        let (pfad, frueher, sicherung) = mit_frueherer(&leer);
        assert_eq!(bis_gelesen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert!(
            !pfad.exists() && !frueher.exists(),
            "das Lesen legt nichts an"
        );
        assert_eq!(
            sichern_und_warten(&sicherung, "getippt"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"getippt");
        assert!(
            !frueher.exists(),
            "unter dem frueheren Namen entsteht nichts"
        );

        let vorhanden = Pruefordner::neu("qn-ueb-keine-vorhanden");
        vorhanden.datei(HEUTE, "steht schon");
        let (pfad, frueher, sicherung) = mit_frueherer(&vorhanden);
        assert_eq!(
            bis_gelesen(&sicherung),
            vec![Sicherungsmeldung::Gelesen("steht schon".to_owned())]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"steht schon");
        assert!(!frueher.exists());
    }

    /// Zeile 2 der Tafel und C2.2: die fruehere Datei heisst danach
    /// `quicknote.txt`, dieselbe Inode, der alte Name ist weg, und die
    /// Quicknote zeigt ihren Inhalt. Gemeldet wird einmal, vor dem Gelesenen.
    #[test]
    fn eine_fruehere_datei_wird_umbenannt_und_ihr_inhalt_gelesen() {
        let ordner = Pruefordner::neu("qn-ueb-umbenannt");
        let alt = ordner.datei(FRUEHER, "von vorher\nzweite Zeile");
        let vorher = inode(&alt);
        let (pfad, frueher, sicherung) = mit_frueherer(&ordner);
        assert_eq!(
            bis_gelesen(&sicherung),
            vec![
                Sicherungsmeldung::Uebernahme(Umbenennung::Umbenannt),
                Sicherungsmeldung::Gelesen("von vorher\nzweite Zeile".to_owned()),
            ]
        );
        assert!(!frueher.exists(), "der alte Name ist weg");
        assert_eq!(inode(&pfad), vorher, "dieselbe Inode");
        assert_eq!(
            std::fs::read(&pfad).expect("gelesen"),
            b"von vorher\nzweite Zeile"
        );

        // Danach sichert die Quicknote wie immer, und der alte Name kommt
        // nicht wieder.
        assert_eq!(
            sichern_und_warten(&sicherung, "weiter"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"weiter");
        assert!(!frueher.exists());
    }

    /// Zeile 3 der Tafel und C2.3: stehen beide, zeigt die Quicknote
    /// `quicknote.txt`, und die fruehere Datei bleibt Byte fuer Byte, auch
    /// ueber das naechste Sichern hinweg.
    #[test]
    fn stehen_beide_gilt_quicknote_txt_und_die_fruehere_bleibt_byte_fuer_byte() {
        let ordner = Pruefordner::neu("qn-ueb-beide");
        let alter_inhalt: &[u8] = b"alt \xff\xfe kein Text\r\n";
        let alt = ordner.datei(FRUEHER, alter_inhalt);
        let alte_inode = inode(&alt);
        ordner.datei(HEUTE, "neu");
        let (pfad, frueher, sicherung) = mit_frueherer(&ordner);
        let neue_inode = inode(&pfad);
        assert_eq!(
            bis_gelesen(&sicherung),
            vec![
                Sicherungsmeldung::Uebernahme(Umbenennung::BeideStehen),
                Sicherungsmeldung::Gelesen("neu".to_owned()),
            ]
        );
        assert_eq!(std::fs::read(&frueher).expect("gelesen"), alter_inhalt);
        assert_eq!(inode(&frueher), alte_inode);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"neu");
        assert_eq!(inode(&pfad), neue_inode, "quicknote.txt ist nicht ersetzt");

        assert_eq!(
            sichern_und_warten(&sicherung, "neuer"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"neuer");
        assert_eq!(std::fs::read(&frueher).expect("gelesen"), alter_inhalt);
    }

    /// Zeile 4 der Tafel: laesst sich der alte Name nicht entfernen, nennen
    /// beide Namen eine Inode, gelesen wird `quicknote.txt`, und gesichert
    /// wird dorthin.
    ///
    /// Die Lage ist von Hand hergestellt (ein harter Verweis, der alte Name
    /// bleibt), weil ein `unlink(2)`, das nach gelungenem `link(2)` im selben
    /// Ordner scheitert, sich in einem Pruefordner nicht herstellen laesst;
    /// gefahren wird deshalb `lesen` mit dem Ausgang als Wert. Dass
    /// `ohne_ersetzen_umbenennen` diesen Ausgang liefert, sieht die Probe nicht.
    #[test]
    fn bleibt_der_alte_name_wird_quicknote_txt_gelesen_und_beide_namen_nennen_eine_inode() {
        let ordner = Pruefordner::neu("qn-ueb-altername");
        let frueher = ordner.datei(FRUEHER, "ein Inhalt");
        let pfad = ordner.unter(HEUTE);
        std::fs::hard_link(&frueher, &pfad).expect("zweiter Name");
        let ausgang = Umbenennung::AlterNameBleibt("Operation not permitted".to_owned());
        let mut zustand = Zustand::Ungelesen;
        assert_eq!(
            lesen(
                Some(&pfad),
                Some(&frueher),
                Some(&ausgang),
                GRENZE,
                &mut zustand
            ),
            Some(Sicherungsmeldung::Gelesen("ein Inhalt".to_owned()))
        );
        assert_eq!(inode(&pfad), inode(&frueher), "zwei Namen, eine Inode");
        assert!(
            matches!(
                zustand,
                Zustand::Bereit {
                    stempel: Some(_),
                    ..
                }
            ),
            "quicknote.txt ist die gelesene Datei und traegt den Stempel"
        );
        assert_eq!(
            schreiben(Some(&pfad), "weiter", &mut zustand),
            Some(Sicherungsmeldung::Gesichert)
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"weiter");
    }

    /// Zeile 5 der Tafel: ist der Verweis gescheitert (ein Dateisystem ohne
    /// harte Verweise), zeigt die Quicknote den Inhalt der frueheren Datei,
    /// die unberuehrt bleibt, und `quicknote.txt` entsteht mit dem ersten
    /// Sichern exklusiv, mit den Rechten `0600`.
    ///
    /// Gefahren wird `lesen` mit dem Ausgang als Wert, aus dem Grund der Probe
    /// darueber: ein Dateisystem ohne harte Verweise hat der Pruefordner nicht.
    #[test]
    fn nach_gescheitertem_verweis_wird_die_fruehere_gelesen_und_quicknote_txt_exklusiv_angelegt() {
        let ordner = Pruefordner::neu("qn-ueb-gescheitert");
        let frueher = ordner.datei(FRUEHER, "auf der Freigabe");
        let pfad = ordner.unter(HEUTE);
        let ausgang = Umbenennung::Gescheitert("Operation not supported".to_owned());
        let mut zustand = Zustand::Ungelesen;
        assert_eq!(
            lesen(
                Some(&pfad),
                Some(&frueher),
                Some(&ausgang),
                GRENZE,
                &mut zustand
            ),
            Some(Sicherungsmeldung::Gelesen("auf der Freigabe".to_owned()))
        );
        assert!(!pfad.exists(), "das Lesen legt quicknote.txt nicht an");
        assert!(
            matches!(zustand, Zustand::Bereit { stempel: None, .. }),
            "quicknote.txt gilt als fehlend"
        );

        assert_eq!(
            schreiben(Some(&pfad), "auf der Freigabe, weiter", &mut zustand),
            Some(Sicherungsmeldung::Gesichert)
        );
        assert_eq!(
            std::fs::read(&pfad).expect("gelesen"),
            b"auf der Freigabe, weiter"
        );
        assert_eq!(rechte(&pfad), 0o600);
        assert_eq!(
            std::fs::read(&frueher).expect("gelesen"),
            b"auf der Freigabe",
            "die fruehere Datei schreibt dieser Faden nie"
        );
    }

    /// Neben der Tafel: scheitert der Verweis und `quicknote.txt` steht doch
    /// (unter dem frueheren Namen liegt etwa ein Ordner), wird sie gelesen und
    /// nicht der fruehere Name. Hier am wirklichen Faden, weil `link(2)` einen
    /// Ordner abweist.
    #[test]
    fn ein_ordner_unter_dem_frueheren_namen_verbirgt_quicknote_txt_nicht() {
        let ordner = Pruefordner::neu("qn-ueb-ordner");
        ordner.ordner(FRUEHER);
        ordner.datei(HEUTE, "lesbar");
        let (pfad, frueher, sicherung) = mit_frueherer(&ordner);
        let meldungen = bis_gelesen(&sicherung);
        assert!(
            matches!(
                meldungen.as_slice(),
                [
                    Sicherungsmeldung::Uebernahme(
                        Umbenennung::Gescheitert(_) | Umbenennung::BeideStehen
                    ),
                    Sicherungsmeldung::Gelesen(text),
                ] if text == "lesbar"
            ),
            "{meldungen:?}"
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::Bereit);
        assert!(frueher.is_dir(), "der Ordner bleibt");
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"lesbar");
    }

    /// C2.4: eine vorhandene `quicknote.txt` wird nie ersetzt, auch nicht,
    /// wenn sie erst entsteht, nachdem der Faden sie als fehlend erkannt hat.
    ///
    /// Zwei Lagen: die Uebernahme trifft auf eine vorhandene Datei (dieselbe
    /// Inode und derselbe Inhalt danach), und nach einem gescheiterten Verweis
    /// entsteht `quicknote.txt` von anderer Hand vor dem ersten Sichern; dann
    /// sperrt die Sicherung, und beide Dateien bleiben Byte fuer Byte.
    #[test]
    fn eine_vorhandene_quicknote_txt_wird_nie_ersetzt() {
        let ordner = Pruefordner::neu("qn-ueb-nie-ersetzt");
        let frueher = ordner.datei(FRUEHER, "frueher");
        let pfad = ordner.datei(HEUTE, "heute");
        let vorher = inode(&pfad);
        assert_eq!(
            uebernehmen(Some(&pfad), Some(&frueher)),
            Some(Umbenennung::BeideStehen)
        );
        assert_eq!(inode(&pfad), vorher);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"heute");
        assert_eq!(std::fs::read(&frueher).expect("gelesen"), b"frueher");

        let ordner = Pruefordner::neu("qn-ueb-nie-ersetzt-spaet");
        let frueher = ordner.datei(FRUEHER, "frueher");
        let pfad = ordner.unter(HEUTE);
        let ausgang = Umbenennung::Gescheitert("Operation not supported".to_owned());
        let mut zustand = Zustand::Ungelesen;
        assert_eq!(
            lesen(
                Some(&pfad),
                Some(&frueher),
                Some(&ausgang),
                GRENZE,
                &mut zustand
            ),
            Some(Sicherungsmeldung::Gelesen("frueher".to_owned()))
        );
        std::fs::write(&pfad, "von anderer Hand").expect("geschrieben");
        assert_eq!(
            schreiben(Some(&pfad), "aus der Quicknote", &mut zustand),
            Some(Sicherungsmeldung::FremdGeaendert)
        );
        assert!(matches!(zustand, Zustand::Gesperrt));
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"von anderer Hand");
        assert_eq!(std::fs::read(&frueher).expect("gelesen"), b"frueher");
    }

    /// Die Uebernahme laeuft einmal: ein zweiter Leseauftrag benennt nichts um
    /// und meldet nichts, auch wenn inzwischen wieder eine Datei unter dem
    /// frueheren Namen steht. Und ohne Notizordner gibt es nichts zu
    /// uebernehmen und keinen Ordner danach.
    #[test]
    fn die_uebernahme_laeuft_einmal_und_legt_keinen_ordner_an() {
        let ordner = Pruefordner::neu("qn-ueb-einmal");
        let (pfad, frueher, sicherung) = mit_frueherer(&ordner);
        assert_eq!(bis_gelesen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        std::fs::write(&frueher, "spaeter hingelegt").expect("geschrieben");
        let _ = sicherung.auftraege.send(Auftrag::Lesen);
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        assert_eq!(sicherung.einziehen(), vec![]);
        assert!(!pfad.exists(), "nichts umbenannt");
        assert_eq!(
            std::fs::read(&frueher).expect("gelesen"),
            b"spaeter hingelegt"
        );

        let ohne = Pruefordner::neu("qn-ueb-ohne-ordner");
        let heim = ohne.unter("krkhome");
        let sicherung =
            Sicherung::starten(Some(heim.join(HEUTE)), Some(heim.join(FRUEHER)), GRENZE)
                .expect("Faden startet");
        assert_eq!(bis_gelesen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert!(!heim.exists(), "die Uebernahme legt keinen Ordner an");
    }

    /// `quicknote.txt` hat genau einen Schreiber, und den Pfad baut allein der
    /// Kern.
    ///
    /// Gehalten am Quelltext von `crates/krk-ui/src`:
    ///
    /// - das exklusive Anlegen (`create_new`) steht in dieser Kiste allein in
    ///   dieser Datei;
    /// - der Name `QUICKNOTEDATEI` steht in keiner Codezeile dieser Kiste, der
    ///   Pfad entsteht also allein ueber `Heimordner::quicknotedatei`;
    /// - `quicknotedatei(` wird in dieser Kiste genau in `appkit/editor.rs`
    ///   gerufen, der Stelle, die die Sicherung startet und ihr den Pfad
    ///   reicht, und in keiner Datei sonst;
    /// - `alte_quicknotedatei(`, der Pfad der Datei unter ihrem frueheren
    ///   Namen, ebenso allein dort: allein die Sicherung bekommt ihn, und
    ///   allein sie uebernimmt die fruehere Datei;
    /// - diese Datei ruft `datei::sichern`.
    ///
    /// Bis zum 261111 hiess die Probe nach dem frueheren Dateinamen.
    ///
    /// **Was sie nicht sieht:** einen Pfad auf `quicknote.txt`, der anders gebaut
    /// wird (ein Literal `"quicknote.txt"`, ein `join` an den Notizordner), und
    /// einen Schreiber, der den Pfad aus `quicknotedatei` bekommt und ihn an
    /// `datei::sichern` oder `ablage::atomar` in einer anderen Datei reicht.
    /// Eine Nadel entscheidet nicht, wohin ein Pfad fliesst.
    #[test]
    fn quicknote_txt_hat_genau_einen_schreiber() {
        let diese = "krk-ui/src/quicknotesicherung.rs";
        let erlaubte_rufer = ["krk-ui/src/appkit/editor.rs"];
        let anlegen = concat!("create", "_new(");
        let name = concat!("QUICKNOTE", "DATEI");
        let ruf = concat!("quicknote", "datei(");
        let alter_ruf = concat!("alte_quicknote", "datei(");
        let schreibweg = concat!("datei::", "sichern(");
        let mut diese_gelesen = false;
        let mut rufer: Vec<String> = Vec::new();
        let mut alte_rufer: Vec<String> = Vec::new();
        for (datei, inhalt) in quelldateien() {
            if !datei.starts_with("krk-ui/src/") {
                continue;
            }
            let code: Vec<&str> = codezeilen(&inhalt).collect();
            let nennt = |nadel: &str| code.iter().any(|zeile| zeile.contains(nadel));
            if datei == diese {
                diese_gelesen = true;
                assert!(
                    nennt(schreibweg),
                    "{diese} sichert nicht ueber datei::sichern"
                );
                assert!(nennt(anlegen), "{diese} legt nicht exklusiv an");
                continue;
            }
            assert!(!nennt(anlegen), "{datei} legt exklusiv an");
            assert!(!nennt(name), "{datei} nennt den Namen der Quicknote-Datei");
            if nennt(alter_ruf) {
                alte_rufer.push(datei.clone());
            }
            if nennt(ruf) {
                rufer.push(datei);
            }
        }
        assert!(diese_gelesen, "{diese} steht nicht im Baum");
        assert_eq!(
            rufer, erlaubte_rufer,
            "quicknotedatei wird nicht genau in den erlaubten Dateien gerufen"
        );
        assert_eq!(
            alte_rufer, erlaubte_rufer,
            "alte_quicknotedatei wird nicht genau in den erlaubten Dateien gerufen"
        );
    }
}
