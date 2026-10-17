//! Vorgaenge, die in ein Archiv schreiben oder aus einem Archiv entfernen:
//! die Zeilen der Tafel in E10 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`, die nicht die
//! Maschine fuer ferne Vorgaenge selbst abarbeitet.
//!
//! ```text
//!  Kopieren, Verschieben  Ziel im Archiv        ──> hinein
//!                           je Quelle ein Schritt: Umhaengen (selbes Archiv,
//!                           Verschieben) oder Hinzufuegen (sonst)
//!                           ein Umbau des Zielarchivs, dann allein die
//!                           vollstaendig angekommenen Quellen entfernen
//!  Verschieben            Quelle im Archiv,
//!                         Ziel lokal oder fern   ──> Maschine kopiert, dann
//!                                                    aus_archiven_entfernen
//!  Umbenennen, Anlegen,
//!  Loeschen               Quelle im Archiv       ──> im_archiv_abarbeiten
//!                                                    ein Umbau je Archiv
//! ```
//!
//! **Ein Vorgang ist ein Umbau je Archiv** und nie ein Schreiben Eintrag fuer
//! Eintrag (E13): alle Konfliktfragen stehen vor dem ersten geschriebenen
//! Byte, und ein Abbruch oder Fehler laesst das Archiv unveraendert
//! (`archiv::umbau`).
//!
//! **Eine Quelle wird beim Verschieben allein entfernt, wenn der Umbau sie
//! vollstaendig gemeldet hat und das Archiv ersetzt ist.** Vollstaendig heisst
//! zweierlei: der Schritt endet in `Schrittausgang::Vollstaendig`, und beim
//! Zusammenstellen ihres Baums ist nichts in den Bericht gekommen, also kein
//! Kind, das sich nicht lesen liess oder nicht herauskommt. Danach gilt je
//! Herkunft eine eigene Regel:
//!
//! - **Lokal wird Eintrag fuer Eintrag entfernt, und allein, was
//!   zusammengestellt wurde** ([`lokal_entfernen`] ueber die `Lokalspur` des
//!   Baums): eine Datei oder Verknuepfung nur, wenn Geraet, Inode, Groesse und
//!   Aenderungszeit noch die beim Zusammenstellen sind, ein Ordner mit
//!   `rmdir(2)`, das an einem Ordner scheitert, in den inzwischen etwas kam.
//!   Was stehen bleibt, nennt der Bericht. So faellt nichts mit, was waehrend
//!   der Konfliktfragen oder des Umbaus in den Quellordner kam.
//! - Eine ferne Quelle entfernt die Maschine wie bisher als Baum, ohne
//!   Abbruchfrage wie `ueber_die_grenze`.
//! - Eine Quelle in einem anderen Archiv entfernt ein zweiter Umbau jenes
//!   Archivs, **und allein an der Datei, aus der kopiert wurde**
//!   ([`Entfernposten`] traegt deren Kennung, `umbau::umbauen_an_kennung`).
//!   Hat ein anderes Programm das Archiv inzwischen ersetzt, entfernt er
//!   nichts. Scheitert er, stehen die Eintraege in beiden Archiven und nicht
//!   in keinem, und der Bericht nennt jeden.
//!
//! **Eine lokale Quelle, die das Zielarchiv ist oder es enthaelt, wird vor
//! dem Umbau abgewiesen**, beim Verschieben wie beim Kopieren
//! ([`zielarchiv_in_quelle`]): verschoben verloere sie mit dem Entfernen das
//! eben ersetzte Archiv, kopiert stuende das alte Archiv als Datei im neuen.
//! Verglichen werden Geraet und Inode der Quelle mit denen der aufgeloesten
//! Archivdatei und jedes ihrer Vorfahren, nicht der Pfadtext; laesst sich
//! eines davon nicht fragen, wird ebenso abgewiesen. **Ob eine Archivquelle im
//! Zielarchiv liegt, entscheiden ebenso Geraet und Inode** und nicht der
//! Pfadtext: zwei Schreibweisen derselben Datei, auch ueber eine
//! Verknuepfung, sind dasselbe Archiv, und verschoben wird darin durch
//! Umhaengen.
//!
//! **Die Daten einer neuen Datei liest ein eigener Faden**, wenn sie auf einem
//! Server oder in einem Archiv liegen ([`Zulieferung`]). Der Umbau zieht die
//! Bytes ueber `Read`, waehrend der Leser eines Archivs und das Gespraech mit
//! einem Server sie in Bloecken an einen Rueckruf reichen; ein Kanal mit
//! wenigen Bloecken verbindet beide. Der Faden kommt aus `fern::faden` und
//! traegt damit ein eigenes Pfand: auch sein Zugriff erreicht den Hauptfaden
//! nicht. Alle Dateien eines Quellarchivs teilen einen Leser, damit ein Baum
//! aus einem tar-Strom nicht je Datei von vorn beginnt.
//!
//! **Ob ein Name im Archiv vergeben ist, fragt das Umbenennen und das Anlegen
//! am Inhaltsverzeichnis**, bevor der Umbau beginnt: im Archiv gibt es kein
//! `rename(2)` und kein `O_EXCL`, das die Frage beantwortete, und ohne
//! Vorpruefung stellte der Umbau die Konfliktfrage, wo lokal das Scheitern mit
//! `AlreadyExists` steht. Gemeldet wird derselbe Grund wie lokal
//! (`operation::grund`), und die Oberflaeche liest ihn ueber
//! `operation::fehlerart` zurueck.
//!
//! # Was dieses Modul nicht haelt
//!
//! - Eine **ferne** Quelle wird beim Verschieben als Baum entfernt, wie in
//!   `ueber_die_grenze`: was zwischen dem Zusammenstellen und dem Entfernen in
//!   einen fernen Quellordner kam, faellt mit.
//! - Zwischen der Frage nach Geraet, Inode, Groesse und Zeit einer lokalen
//!   Datei und ihrem Entfernen liegt kein Schutz: was ein anderes Programm in
//!   diesem Augenblick in sie schreibt, geht mit ihr.

use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};
use std::ops::ControlFlow;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, sync_channel};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::archiv::lesen::lesbarkeit_grund;
use crate::archiv::umbau::{
    self, Datenquelle, Geoeffnet, Leer, Lokalspur, Neuart, Neueintrag, Schrittausgang, Umbaufehler,
    Umbauplan, Umbauschritt,
};
use crate::archiv::{
    Archiveintrag, Archivleser, Archivzugang, Dateikennung, Inhaltsverzeichnis, Innenpfad,
    Lesefehler,
};
use crate::fern::{self as fernzugriff, Fernfaden};
use crate::ort::{Archivort, Ort};
use crate::sprache::{Text, satz, text};
use crate::verzeichnis::Typ;

use super::{
    Ablauf, Abschluss, Angaben, Anlegetyp, Fehlschlag, Fernart, Stelle, Steuerung, bezeichnung,
    grund, kinder_lesen, name_pruefen, namensgrund_ohne_ziel, nie_abgebrochen, unterangaben_holen,
};

/// Wie viele Bloecke zwischen dem Faden einer [`Zulieferung`] und dem Umbau
/// unterwegs sein duerfen.
const ZULIEFERTIEFE: usize = 4;

/// Die Bezeichnung eines Eintrags im Archiv, wie [`bezeichnung`] sie fuer
/// einen Archivort gibt: `/pfad/zum/archiv.zip:/innen/pfad`.
fn archivbezeichnung(datei: &Path, innen: &Innenpfad) -> PathBuf {
    PathBuf::from(format!("{}:{innen}", datei.display()))
}

/// Der Grund eines vergebenen Namens, derselbe wie lokal.
fn vergeben() -> String {
    crate::operation::grund(&io::Error::from(io::ErrorKind::AlreadyExists))
}

// ----------------------------------------------------------------------
// Hinein: Kopieren und Verschieben mit einem Archiv als Ziel
// ----------------------------------------------------------------------

/// Woher eine Quelle kam, die ins Zielarchiv geht; beim Verschieben sagt es,
/// wer sie danach entfernt.
enum Herkunft {
    /// Aus dem Zielarchiv selbst: verschoben wird durch Umhaengen, und es
    /// bleibt nichts zu entfernen.
    Zielarchiv,
    /// Lokal: entfernt wird Eintrag fuer Eintrag, was die Spur fuehrt
    /// ([`lokal_entfernen`]).
    Lokal(Vec<Lokalspur>),
    /// Fern: die Maschine entfernt sie.
    Stelle(Stelle),
    /// Aus einem anderen Archiv: ein zweiter Umbau entfernt sie.
    Archiv(Entfernposten),
}

/// Eine Quelle im Plan des Zielarchivs.
struct Posten {
    bezeichnung: PathBuf,
    schritt: usize,
    /// Ob beim Zusammenstellen ihres Baums nichts in den Bericht kam.
    ganz: bool,
    herkunft: Herkunft,
}

/// Ein Eintrag eines Archivs, der nach einem vollstaendigen Kopieren aus dem
/// Archiv entfernt wird, samt der Kennung der Datei, aus der kopiert wurde.
pub(super) struct Entfernposten {
    zugang: Arc<Archivzugang>,
    innen: Innenpfad,
    bezeichnung: PathBuf,
    kennung: Dateikennung,
}

impl Entfernposten {
    /// Der Eintrag `innen` im Archiv hinter `zugang`, kopiert aus der Datei
    /// mit der Kennung `kennung` (die des Inhaltsverzeichnisses, aus dem der
    /// Leser las).
    pub(super) fn neu(
        zugang: Arc<Archivzugang>,
        innen: Innenpfad,
        bezeichnung: PathBuf,
        kennung: Dateikennung,
    ) -> Self {
        Entfernposten {
            zugang,
            innen,
            bezeichnung,
            kennung,
        }
    }
}

/// Ob zwei Pfade dieselbe Datei nennen, an Geraet und Inode nach dem
/// Aufloesen jeder Verknuepfung. Gleicher Pfadtext ist dieselbe Datei ohne
/// Frage.
fn dieselbe_datei(eine: &Path, andere: &Path) -> io::Result<bool> {
    if eine == andere {
        return Ok(true);
    }
    let (eine, andere) = (fs::metadata(eine)?, fs::metadata(andere)?);
    Ok(eine.dev() == andere.dev() && eine.ino() == andere.ino())
}

/// Ob die lokale Quelle `quelle` die Archivdatei `archiv` ist oder sie
/// enthaelt (Modulkopf).
///
/// Die Quelle wird an ihrem eigenen Eintrag gefragt (`lstat(2)`): eine
/// Verknuepfung wird als Verknuepfung hineingelegt und entfernt, und das
/// Archiv, auf das sie zeigt, bleibt unberuehrt. Das Archiv wird aufgeloest,
/// denn der Umbau schreibt an seinem Ziel, und mit ihm jeder seiner
/// Vorfahren gegen Geraet und Inode der Quelle gehalten.
///
/// # Errors
///
/// Der Fehler einer der Fragen; der Rufer weist die Quelle dann ab.
fn zielarchiv_in_quelle(quelle: &Path, archiv: &Path) -> io::Result<bool> {
    let quelle = fs::symlink_metadata(quelle)?;
    if quelle.file_type().is_symlink() {
        return Ok(false);
    }
    let archiv = fs::canonicalize(archiv)?;
    for stufe in archiv.ancestors() {
        let angaben = fs::metadata(stufe)?;
        if angaben.dev() == quelle.dev() && angaben.ino() == quelle.ino() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Die Leser der Quellarchive dieses Vorgangs, je Archivdatei einer, geteilt
/// zwischen dem Faden des Vorgangs und den Faeden der [`Zulieferung`].
#[derive(Default)]
struct Leserliste {
    leser: Vec<(PathBuf, Result<GeteilterLeser, String>)>,
}

/// Ein Leser eines Quellarchivs, geteilt mit den Faeden der [`Zulieferung`].
type GeteilterLeser = Arc<Mutex<Archivleser>>;

impl Leserliste {
    /// Der Leser des Archivs hinter `zugang`, beim ersten Mal geoeffnet; ein
    /// gescheitertes Oeffnen wird je Archiv einmal versucht.
    fn fuer(
        &mut self,
        pfand: &Fernfaden,
        zugang: &Arc<Archivzugang>,
        steuerung: &Steuerung,
    ) -> Result<Arc<Mutex<Archivleser>>, Lesefehler> {
        if let Some((_, vorhanden)) = self.leser.iter().find(|(datei, _)| datei == zugang.datei()) {
            return vorhanden
                .as_ref()
                .map(Arc::clone)
                .map_err(|grund| Lesefehler::Unlesbar(grund.clone()));
        }
        let geoeffnet =
            match Archivleser::oeffnen(pfand, Arc::clone(zugang), steuerung.abbruchkennzeichen()) {
                Ok(leser) => Ok(Arc::new(Mutex::new(leser))),
                Err(Lesefehler::Abgebrochen) => return Err(Lesefehler::Abgebrochen),
                Err(fehler) => Err(fehler.grund()),
            };
        self.leser
            .push((zugang.datei().to_path_buf(), geoeffnet.clone()));
        geoeffnet.map_err(Lesefehler::Unlesbar)
    }
}

/// Kopiert oder verschiebt die Quellen in den Ordner `ziel` eines Archivs,
/// ueber genau einen Umbau dieses Archivs.
///
/// Zuerst wird je Quelle ihr Baum als Wert zusammengestellt, ohne ein Byte zu
/// schreiben und ohne Konfliktfrage; dann stellt der Umbau jede Konfliktfrage
/// am Inhaltsverzeichnis, schreibt das neue Archiv und ersetzt das alte. Erst
/// danach entfernt ein Verschieben die Quellen, die vollstaendig angekommen
/// sind (Modulkopf). Ein Archiv, das KRK allein lesen darf, nimmt nichts an.
pub(super) fn hinein(
    pfand: &Fernfaden,
    quellen: &[Ort],
    ziel: &Archivort,
    verschieben: bool,
    steuerung: &mut Steuerung,
) -> Abschluss {
    if !ziel.format().beschreibbar() {
        for ort in quellen {
            steuerung.ueberspringen(&bezeichnung(ort), text(Text::ArchivNurLesbar));
        }
        return Abschluss::Fertig;
    }
    let mut plan = Umbauplan::neu();
    let mut posten: Vec<Posten> = Vec::with_capacity(quellen.len());
    let mut leser = Leserliste::default();
    for ort in quellen {
        if steuerung.abgebrochen() {
            return Abschluss::Abgebrochen;
        }
        let stand = steuerung.uebersprungen_stand();
        let gebaut = match ort {
            Ort::Archiv(quelle) => match dieselbe_datei(quelle.datei(), ziel.datei()) {
                Ok(true) => {
                    aus_dem_zielarchiv(pfand, quelle, ziel, verschieben, &mut leser, steuerung)
                }
                Ok(false) => aus_einem_archiv(pfand, quelle, &ziel.innen, &mut leser, steuerung)
                    .map(|schritt| {
                        schritt.map(|(schritt, kennung)| {
                            let herkunft = Herkunft::Archiv(Entfernposten::neu(
                                Arc::clone(&quelle.zugang),
                                quelle.innen.clone(),
                                bezeichnung(ort),
                                kennung,
                            ));
                            (schritt, herkunft)
                        })
                    }),
                Err(fehler) => {
                    steuerung.ueberspringen(&bezeichnung(ort), crate::operation::grund(&fehler));
                    Ok(None)
                }
            },
            Ort::Lokal(pfad) => match zielarchiv_in_quelle(pfad, ziel.datei()) {
                Ok(false) => match Neueintrag::lokal_mit_spur(pfand, pfad) {
                    Ok((eintrag, spur)) => Ok(Some((
                        Umbauschritt::Hinzufuegen {
                            ordner: ziel.innen.clone(),
                            eintrag,
                        },
                        Herkunft::Lokal(spur),
                    ))),
                    Err(fehler) => {
                        steuerung.ueberspringen(pfad, crate::operation::grund(&fehler));
                        Ok(None)
                    }
                },
                Ok(true) => {
                    steuerung.ueberspringen(pfad, text(Text::VorgangZielLiegtInDerQuelle));
                    Ok(None)
                }
                Err(fehler) => {
                    steuerung.ueberspringen(pfad, crate::operation::grund(&fehler));
                    Ok(None)
                }
            },
            Ort::Fern(_) => match Stelle::von(ort) {
                Some(stelle) => {
                    von_einer_stelle(pfand, &stelle, &ziel.innen, steuerung).map(|schritt| {
                        schritt.map(|schritt| (schritt, Herkunft::Stelle(stelle.clone())))
                    })
                }
                None => {
                    steuerung.ueberspringen(
                        &bezeichnung(ort),
                        text(Text::VorgangPfadBenenntKeinenEintrag),
                    );
                    Ok(None)
                }
            },
        };
        match gebaut {
            Err(Ablauf::Abgebrochen) => return Abschluss::Abgebrochen,
            Err(Ablauf::Weiter) | Ok(None) => {}
            Ok(Some((schritt, herkunft))) => {
                let ganz = steuerung.uebersprungen_seit(stand).is_empty();
                let schritt = plan.anfuegen(schritt);
                posten.push(Posten {
                    bezeichnung: bezeichnung(ort),
                    schritt,
                    ganz,
                    herkunft,
                });
            }
        }
    }
    if posten.is_empty() {
        return abschluss(steuerung);
    }

    let umgebaut = match umbau::umbauen(pfand, &ziel.zugang, plan, steuerung) {
        Ok(umgebaut) => umgebaut,
        Err(fehler) => return umbaufehler_melden(&fehler, ziel.datei(), steuerung),
    };

    let mut aus_archiven = Vec::new();
    for eintrag in posten {
        let vollstaendig = match umgebaut.schritte.get(eintrag.schritt) {
            Some(Schrittausgang::Vollstaendig(_)) => true,
            Some(Schrittausgang::Teilweise(_)) => false,
            Some(Schrittausgang::Ausgelassen) | None => continue,
        };
        let bytes = umgebaut.bytes.get(eintrag.schritt).copied().unwrap_or(0);
        steuerung.eintrag_fertig(&eintrag.bezeichnung, bytes);
        if !verschieben {
            continue;
        }
        // Allein nach einem ersetzten Archiv und allein vollstaendig
        // angekommene Quellen (Modulkopf).
        if !umgebaut.ersetzt || !vollstaendig || !eintrag.ganz {
            steuerung.ueberspringen(
                &eintrag.bezeichnung,
                text(Text::VorgangNichtVollstaendigKopiert),
            );
            continue;
        }
        match eintrag.herkunft {
            Herkunft::Zielarchiv => {}
            Herkunft::Lokal(spur) => lokal_entfernen(&spur, steuerung),
            Herkunft::Stelle(stelle) => {
                // Ohne Abbruchfrage, aus dem Grund in `ueber_die_grenze`.
                if let Err(fehler) = stelle.baum_entfernen(pfand, &nie_abgebrochen) {
                    steuerung.ueberspringen(
                        &eintrag.bezeichnung,
                        satz(
                            Text::VorgangKopiertAberInQuelleGeblieben,
                            &[("grund", &grund(&fehler))],
                        ),
                    );
                }
            }
            Herkunft::Archiv(posten) => aus_archiven.push(posten),
        }
    }
    aus_archiven_entfernen(pfand, aus_archiven, steuerung);
    abschluss(steuerung)
}

/// Abgebrochen, wenn der Nutzer abgebrochen hat, sonst fertig.
fn abschluss(steuerung: &Steuerung) -> Abschluss {
    if steuerung.abgebrochen() {
        Abschluss::Abgebrochen
    } else {
        Abschluss::Fertig
    }
}

/// Traegt einen gescheiterten Umbau ein: bei einer Quelle zuerst sie, und
/// immer eine Zeile fuer das unveraenderte Archiv.
fn umbaufehler_melden(fehler: &Umbaufehler, archiv: &Path, steuerung: &mut Steuerung) -> Abschluss {
    fehler.melden(steuerung, archiv);
    match fehler {
        Umbaufehler::Abgebrochen => Abschluss::Abgebrochen,
        _ => abschluss(steuerung),
    }
}

/// Entfernt eine lokale Quelle nach ihrer Spur, Kinder vor ihrem Ordner, und
/// allein, was beim Zusammenstellen dastand (Modulkopf).
///
/// Eine Datei oder Verknuepfung geht nur, wenn sie noch dieselbe ist; ein
/// Ordner geht mit `rmdir(2)`, also nur leer. Was stehen bleibt, kommt mit
/// Grund in den Bericht; ein Ordner, in dem schon ein Kind stehen blieb, nicht
/// noch einmal, denn sein Bleiben erklaert das Kind. Ein Eintrag, der schon
/// fort ist, gilt als entfernt.
fn lokal_entfernen(spur: &[Lokalspur], steuerung: &mut Steuerung) {
    let mut geblieben: HashSet<&Path> = HashSet::new();
    for eintrag in spur {
        let pfad = eintrag.pfad.as_path();
        let ausgang = if eintrag.ordner {
            if geblieben.iter().any(|kind| kind.parent() == Some(pfad)) {
                geblieben.insert(pfad);
                continue;
            }
            fs::remove_dir(pfad).map_err(|fehler| crate::operation::grund(&fehler))
        } else {
            match eintrag.unveraendert() {
                Ok(true) => {
                    fs::remove_file(pfad).map_err(|fehler| crate::operation::grund(&fehler))
                }
                Ok(false) => Err(text(Text::VorgangQuelleSeitDemLesenVeraendert).to_owned()),
                Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(()),
                Err(fehler) => Err(crate::operation::grund(&fehler)),
            }
        };
        if let Err(warum) = ausgang {
            steuerung.ueberspringen(
                pfad,
                satz(
                    Text::VorgangKopiertAberInQuelleGeblieben,
                    &[("grund", &warum)],
                ),
            );
            geblieben.insert(pfad);
        }
    }
}

/// Der Schritt fuer eine Quelle im Zielarchiv selbst: beim Verschieben ein
/// Umhaengen ohne Datenkopie, beim Kopieren ein neuer Eintrag aus den alten
/// Daten.
///
/// Ein Kopieren in den eigenen Ordner und eines Ordners in sich selbst weist
/// es ab wie lokal; beim Umhaengen tut das der Umbau selbst.
fn aus_dem_zielarchiv(
    pfand: &Fernfaden,
    quelle: &Archivort,
    ziel: &Archivort,
    verschieben: bool,
    leser: &mut Leserliste,
    steuerung: &mut Steuerung,
) -> Result<Option<(Umbauschritt, Herkunft)>, Ablauf> {
    let wo = archivbezeichnung(quelle.datei(), &quelle.innen);
    let Some(nach) = quelle
        .innen
        .name()
        .and_then(|name| ziel.innen.verbinden(name))
    else {
        steuerung.ueberspringen(&wo, text(Text::VorgangPfadBenenntKeinenEintrag));
        return Ok(None);
    };
    if verschieben {
        return Ok(Some((
            Umbauschritt::Umhaengen {
                von: quelle.innen.clone(),
                nach,
            },
            Herkunft::Zielarchiv,
        )));
    }
    let abweisung = if quelle.innen.aufwaerts().as_ref() == Some(&ziel.innen) {
        Some(Text::VorgangQuelleUndZielDerselbeEintrag)
    } else if ziel.innen == quelle.innen || ziel.innen.liegt_unter(&quelle.innen) {
        Some(Text::VorgangZielLiegtInDerQuelle)
    } else {
        None
    };
    if let Some(schluessel) = abweisung {
        steuerung.ueberspringen(&wo, text(schluessel));
        return Ok(None);
    }
    let schritt = aus_einem_archiv(pfand, quelle, &ziel.innen, leser, steuerung)?;
    Ok(schritt.map(|(schritt, _)| (schritt, Herkunft::Zielarchiv)))
}

/// Der Schritt, der den Eintrag `quelle` samt Unterbaum aus seinem Archiv in
/// den Ordner `ordner` des Zielarchivs bringt, mit der Kennung der
/// Quellarchivdatei, aus der gelesen wird.
fn aus_einem_archiv(
    pfand: &Fernfaden,
    quelle: &Archivort,
    ordner: &Innenpfad,
    leser: &mut Leserliste,
    steuerung: &mut Steuerung,
) -> Result<Option<(Umbauschritt, Dateikennung)>, Ablauf> {
    let wo = archivbezeichnung(quelle.datei(), &quelle.innen);
    let geteilt = match leser.fuer(pfand, &quelle.zugang, steuerung) {
        Ok(geteilt) => geteilt,
        Err(Lesefehler::Abgebrochen) => return Err(Ablauf::Abgebrochen),
        Err(fehler) => {
            steuerung.ueberspringen(&wo, fehler.grund());
            return Ok(None);
        }
    };
    let verzeichnis = geteilt
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .geteiltes_verzeichnis();
    let Some(eintrag) = verzeichnis.eintrag(&quelle.innen) else {
        steuerung.ueberspringen(&wo, text(Text::ArchivEintragFehlt));
        return Ok(None);
    };
    let neu = archiveintrag_bauen(
        pfand,
        &geteilt,
        &verzeichnis,
        quelle.datei(),
        eintrag,
        steuerung,
    )?;
    Ok(neu.map(|eintrag| {
        (
            Umbauschritt::Hinzufuegen {
                ordner: ordner.clone(),
                eintrag,
            },
            verzeichnis.kennung(),
        )
    }))
}

/// Ein Eintrag eines Quellarchivs samt Unterbaum als neuer Eintrag des
/// Zielarchivs, oder `None`, wenn er nicht herauskommt; der Grund steht dann
/// im Bericht.
fn archiveintrag_bauen(
    pfand: &Fernfaden,
    leser: &Arc<Mutex<Archivleser>>,
    verzeichnis: &Inhaltsverzeichnis,
    datei: &Path,
    eintrag: &Archiveintrag,
    steuerung: &mut Steuerung,
) -> Result<Option<Neueintrag>, Ablauf> {
    if steuerung.abgebrochen() {
        return Err(Ablauf::Abgebrochen);
    }
    let wo = archivbezeichnung(datei, &eintrag.innen);
    if let Some(warum) = lesbarkeit_grund(&eintrag.lesbarkeit) {
        steuerung.ueberspringen(&wo, warum);
        return Ok(None);
    }
    let Some(name) = eintrag.innen.name() else {
        steuerung.ueberspringen(&wo, text(Text::VorgangPfadBenenntKeinenEintrag));
        return Ok(None);
    };
    let art = match eintrag.typ {
        Typ::Ordner => {
            let mut kinder = Vec::new();
            for kind in verzeichnis.kinder(&eintrag.innen).into_iter().flatten() {
                if let Some(neu) =
                    archiveintrag_bauen(pfand, leser, verzeichnis, datei, kind, steuerung)?
                {
                    kinder.push(neu);
                }
            }
            Neuart::Ordner(kinder)
        }
        Typ::Verknuepfung => {
            let gelesen = leser
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .verweis_lesen(pfand, &eintrag.innen);
            let verweis = match gelesen {
                Ok(verweis) => verweis,
                Err(Lesefehler::Abgebrochen) => return Err(Ablauf::Abgebrochen),
                Err(fehler) => {
                    steuerung.ueberspringen(&wo, fehler.grund());
                    return Ok(None);
                }
            };
            match String::from_utf8(verweis) {
                Ok(verweis) => Neuart::Verknuepfung(verweis),
                Err(_) => {
                    steuerung.ueberspringen(&wo, grund(&Fehlschlag::NameOhneUtf8));
                    return Ok(None);
                }
            }
        }
        Typ::Datei => Neuart::Datei(Box::new(Zulieferung::aus_archiv(
            Arc::clone(leser),
            eintrag.innen.clone(),
            eintrag.groesse,
        ))),
    };
    Ok(Some(Neueintrag {
        name: name.to_owned(),
        herkunft: wo,
        rechte: eintrag.rechte.unwrap_or(match eintrag.typ {
            Typ::Ordner => 0o755,
            Typ::Datei | Typ::Verknuepfung => 0o644,
        }),
        geaendert: eintrag.geaendert,
        art,
    }))
}

/// Der Schritt, der eine ferne Quelle samt Unterbaum in den Ordner `ordner`
/// des Zielarchivs bringt.
fn von_einer_stelle(
    pfand: &Fernfaden,
    stelle: &Stelle,
    ordner: &Innenpfad,
    steuerung: &mut Steuerung,
) -> Result<Option<Umbauschritt>, Ablauf> {
    let angaben = match stelle.angaben(pfand) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            steuerung.ueberspringen(&stelle.bezeichnung(), grund(&fehler));
            return Ok(None);
        }
    };
    let neu = stelleneintrag_bauen(pfand, stelle, &angaben, steuerung)?;
    Ok(neu.map(|eintrag| Umbauschritt::Hinzufuegen {
        ordner: ordner.clone(),
        eintrag,
    }))
}

/// Eine Stelle samt Unterbaum als neuer Eintrag des Zielarchivs, oder `None`,
/// wenn sie nicht herueberkommt; der Grund steht dann im Bericht. Die Kinder
/// liest derselbe Weg wie beim Kopieren ([`kinder_lesen`]).
fn stelleneintrag_bauen(
    pfand: &Fernfaden,
    stelle: &Stelle,
    angaben: &Angaben,
    steuerung: &mut Steuerung,
) -> Result<Option<Neueintrag>, Ablauf> {
    if steuerung.abgebrochen() {
        return Err(Ablauf::Abgebrochen);
    }
    let wo = stelle.bezeichnung();
    let name = match stelle.name() {
        Ok(name) => name,
        Err(schluessel) => {
            steuerung.ueberspringen(&wo, text(schluessel));
            return Ok(None);
        }
    };
    let art = match angaben.typ {
        Typ::Datei if !angaben.gewoehnlich => {
            steuerung.ueberspringen(&wo, text(Text::VorgangKeineGewoehnlicheDatei));
            return Ok(None);
        }
        Typ::Datei => Neuart::Datei(Box::new(Zulieferung::von_stelle(
            stelle.clone(),
            angaben.groesse,
        ))),
        Typ::Ordner => {
            let kinder = match kinder_lesen(pfand, stelle, steuerung) {
                Ok(kinder) => kinder,
                Err(Ablauf::Abgebrochen) => return Err(Ablauf::Abgebrochen),
                Err(Ablauf::Weiter) => return Ok(None),
            };
            let mut neue = Vec::with_capacity(kinder.len());
            for (kind, mitgebracht) in kinder {
                let Some(unterangaben) = unterangaben_holen(
                    mitgebracht,
                    &kind.bezeichnung(),
                    || kind.angaben(pfand),
                    steuerung,
                ) else {
                    continue;
                };
                if let Some(neu) = stelleneintrag_bauen(pfand, &kind, &unterangaben, steuerung)? {
                    neue.push(neu);
                }
            }
            Neuart::Ordner(neue)
        }
        Typ::Verknuepfung => match stelle.verknuepfung_lesen(pfand) {
            Ok(verweis) => match String::from_utf8(verweis) {
                Ok(verweis) => Neuart::Verknuepfung(verweis),
                Err(_) => {
                    steuerung.ueberspringen(&wo, grund(&Fehlschlag::NameOhneUtf8));
                    return Ok(None);
                }
            },
            Err(fehler) => {
                steuerung.ueberspringen(&wo, grund(&fehler));
                return Ok(None);
            }
        },
    };
    Ok(Some(Neueintrag {
        name,
        herkunft: wo,
        rechte: angaben.rechte.unwrap_or(match angaben.typ {
            Typ::Ordner => 0o755,
            Typ::Datei | Typ::Verknuepfung => 0o644,
        }),
        geaendert: angaben.geaendert.unwrap_or(UNIX_EPOCH),
        art,
    }))
}

// ----------------------------------------------------------------------
// Aus Archiven entfernen, nachdem kopiert ist
// ----------------------------------------------------------------------

/// Entfernt die genannten Eintraege aus ihren Archiven, ueber einen Umbau je
/// Archiv; gerufen, nachdem jeder davon vollstaendig an seinem Ziel steht.
///
/// **Scheitert ein Umbau, stehen die Eintraege doppelt da und nicht
/// nirgends**: das Archiv ist unveraendert (`archiv::umbau`), und der Bericht
/// nennt jeden Eintrag mit dem Grund. Nach einem Abbruch wird kein Archiv
/// mehr angefasst, und jeder Eintrag steht ebenso im Bericht.
///
/// **Entfernt wird allein an der Datei, aus der kopiert wurde**: der Umbau
/// laeuft ueber `umbau::umbauen_an_kennung` mit der Kennung der Posten, und
/// traegt die Archivdatei inzwischen eine andere, entfernt er nichts.
/// Nennen zwei Posten desselben Archivs verschiedene Kennungen, hat sich das
/// Archiv schon waehrend des Kopierens geaendert, und es wird ebenso nichts
/// entfernt.
pub(super) fn aus_archiven_entfernen(
    pfand: &Fernfaden,
    posten: Vec<Entfernposten>,
    steuerung: &mut Steuerung,
) {
    let mut gruppen: Vec<(Arc<Archivzugang>, Vec<Entfernposten>)> = Vec::new();
    for eintrag in posten {
        match gruppen
            .iter_mut()
            .find(|(zugang, _)| zugang.datei() == eintrag.zugang.datei())
        {
            Some((_, gruppe)) => gruppe.push(eintrag),
            None => gruppen.push((Arc::clone(&eintrag.zugang), vec![eintrag])),
        }
    }
    for (zugang, gruppe) in gruppen {
        let geblieben = |steuerung: &mut Steuerung, gruppe: &[Entfernposten], warum: &str| {
            for eintrag in gruppe {
                steuerung.ueberspringen(
                    &eintrag.bezeichnung,
                    satz(
                        Text::VorgangKopiertAberInQuelleGeblieben,
                        &[("grund", &warum)],
                    ),
                );
            }
        };
        if steuerung.abgebrochen() {
            geblieben(
                steuerung,
                &gruppe,
                text(Text::ArchivUnveraendertNachAbbruch),
            );
            continue;
        }
        let Some(kennung) = gruppe.first().map(|eintrag| eintrag.kennung) else {
            continue;
        };
        if gruppe.iter().any(|eintrag| eintrag.kennung != kennung) {
            geblieben(
                steuerung,
                &gruppe,
                &Umbaufehler::VonAussenVeraendert.grund(),
            );
            continue;
        }
        let mut plan = Umbauplan::neu();
        for eintrag in &gruppe {
            let _ = plan.anfuegen(Umbauschritt::Entfernen(eintrag.innen.clone()));
        }
        match umbau::umbauen_an_kennung(pfand, &zugang, kennung, plan, steuerung) {
            Ok(_) => {}
            Err(fehler) => geblieben(steuerung, &gruppe, &fehler.grund()),
        }
    }
}

// ----------------------------------------------------------------------
// Umbenennen, Anlegen und Loeschen im Archiv (E10, E17)
// ----------------------------------------------------------------------

/// Eine Handlung im Archiv nach dem Umbau, fuer den Bericht.
struct Handlung {
    bezeichnung: PathBuf,
    schritt: usize,
    /// Die Groesse, die der Bericht nennt, wo der Umbau keine Bytes schreibt.
    groesse: u64,
}

/// Benennt um, legt an oder entfernt endgueltig in Archiven, ueber einen
/// Umbau je Archiv.
///
/// **Ein vergebener Name scheitert vor dem Umbau** am Inhaltsverzeichnis, mit
/// dem Grund, den lokal das Scheitern an `AlreadyExists` traegt (Modulkopf).
/// Ein Loeschen ist endgueltig, denn ein Archiv hat keinen Papierkorb; ein
/// Abbruch oder Fehler laesst das ganze Archiv stehen (E17).
pub(super) fn im_archiv_abarbeiten(
    pfand: &Fernfaden,
    art: &Fernart,
    orte: Vec<Archivort>,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let mut gruppen: Vec<Vec<Archivort>> = Vec::new();
    for ort in orte {
        match gruppen.iter_mut().find(|gruppe| {
            gruppe
                .first()
                .is_some_and(|erster| erster.datei() == ort.datei())
        }) {
            Some(gruppe) => gruppe.push(ort),
            None => gruppen.push(vec![ort]),
        }
    }
    for gruppe in gruppen {
        if steuerung.abgebrochen() {
            return Ablauf::Abgebrochen;
        }
        if im_einen_archiv(pfand, art, &gruppe, steuerung) == Ablauf::Abgebrochen {
            return Ablauf::Abgebrochen;
        }
    }
    Ablauf::Weiter
}

/// [`im_archiv_abarbeiten`] fuer die Orte eines Archivs.
fn im_einen_archiv(
    pfand: &Fernfaden,
    art: &Fernart,
    orte: &[Archivort],
    steuerung: &mut Steuerung,
) -> Ablauf {
    let Some(zugang) = orte.first().map(|ort| Arc::clone(&ort.zugang)) else {
        return Ablauf::Weiter;
    };
    if !zugang.format().beschreibbar() {
        for ort in orte {
            steuerung.ueberspringen(
                &archivbezeichnung(ort.datei(), &ort.innen),
                text(Text::ArchivNurLesbar),
            );
        }
        return Ablauf::Weiter;
    }
    let verzeichnis = match zugang.verzeichnis(pfand, &steuerung.abbruchkennzeichen()) {
        Ok(verzeichnis) => verzeichnis,
        Err(crate::archiv::Verzeichnisfehler::Abgebrochen) => return Ablauf::Abgebrochen,
        Err(fehler) => {
            let warum = Lesefehler::Verzeichnis(fehler).grund();
            for ort in orte {
                steuerung.ueberspringen(&archivbezeichnung(ort.datei(), &ort.innen), warum.clone());
            }
            return Ablauf::Weiter;
        }
    };

    let mut plan = Umbauplan::neu();
    let mut handlungen: Vec<Handlung> = Vec::with_capacity(orte.len());
    for ort in orte {
        let geplant = match art {
            Fernart::Umbenennen { neuer_name } => umbenennen_planen(ort, neuer_name, &verzeichnis),
            Fernart::Anlegen { name, typ } => anlegen_planen(ort, name, *typ, &verzeichnis),
            Fernart::Loeschen => {
                let groesse = verzeichnis
                    .eintrag(&ort.innen)
                    .filter(|eintrag| eintrag.typ == Typ::Datei)
                    .map_or(0, |eintrag| eintrag.groesse);
                Ok((
                    archivbezeichnung(ort.datei(), &ort.innen),
                    Umbauschritt::Entfernen(ort.innen.clone()),
                    groesse,
                ))
            }
            // Kopieren und Verschieben kommen nicht hierher
            // (`fern::ausfuehren`); der Zweig steht, weil die Unterscheidung
            // vollstaendig ist, und meldet den fehlenden Zielordner wie
            // `fern::an_der_stelle_abarbeiten`.
            Fernart::Kopieren { .. } | Fernart::Verschieben { .. } => Err((
                archivbezeichnung(ort.datei(), &ort.innen),
                text(Text::VorgangZielordnerFehlt).to_owned(),
            )),
        };
        match geplant {
            Ok((bezeichnung, schritt, groesse)) => {
                let schritt = plan.anfuegen(schritt);
                handlungen.push(Handlung {
                    bezeichnung,
                    schritt,
                    groesse,
                });
            }
            Err((bezeichnung, warum)) => steuerung.ueberspringen(&bezeichnung, warum),
        }
    }
    if handlungen.is_empty() {
        return Ablauf::Weiter;
    }

    match umbau::umbauen(pfand, &zugang, plan, steuerung) {
        Ok(umgebaut) => {
            for handlung in &handlungen {
                match umgebaut.schritte.get(handlung.schritt) {
                    Some(Schrittausgang::Vollstaendig(_) | Schrittausgang::Teilweise(_)) => {
                        let bytes = umgebaut
                            .bytes
                            .get(handlung.schritt)
                            .copied()
                            .unwrap_or(0)
                            .max(handlung.groesse);
                        steuerung.eintrag_fertig(&handlung.bezeichnung, bytes);
                    }
                    Some(Schrittausgang::Ausgelassen) | None => {}
                }
            }
            Ablauf::Weiter
        }
        Err(fehler) => match umbaufehler_melden(&fehler, zugang.datei(), steuerung) {
            Abschluss::Abgebrochen => Ablauf::Abgebrochen,
            Abschluss::Fertig => Ablauf::Weiter,
        },
    }
}

/// Bezeichnung, Schritt und Groesse einer geplanten Handlung, oder die
/// Bezeichnung mit dem Grund, warum sie nicht geplant wird.
type Geplant = Result<(PathBuf, Umbauschritt, u64), (PathBuf, String)>;

/// Das Umbenennen von `ort` in seinem Ordner: ein Umhaengen, sofern der Name
/// gueltig und frei ist.
fn umbenennen_planen(
    ort: &Archivort,
    neuer_name: &str,
    verzeichnis: &Inhaltsverzeichnis,
) -> Geplant {
    let wo = archivbezeichnung(ort.datei(), &ort.innen);
    if let Err(fehler) = name_pruefen(neuer_name) {
        return Err((wo, fehler.grund().to_owned()));
    }
    let Some(nach) = ort
        .innen
        .aufwaerts()
        .and_then(|ordner| ordner.verbinden(neuer_name))
    else {
        return Err((wo, namensgrund_ohne_ziel(neuer_name)));
    };
    if verzeichnis.eintrag(&nach).is_some() {
        return Err((wo, vergeben()));
    }
    Ok((
        wo,
        Umbauschritt::Umhaengen {
            von: ort.innen.clone(),
            nach,
        },
        0,
    ))
}

/// Das Anlegen eines leeren Ordners oder einer leeren Datei `name` im Ordner
/// `ort`, sofern der Name gueltig und frei ist.
fn anlegen_planen(
    ort: &Archivort,
    name: &str,
    typ: Anlegetyp,
    verzeichnis: &Inhaltsverzeichnis,
) -> Geplant {
    let Some(ziel) = ort.kind(name) else {
        return Err((
            archivbezeichnung(ort.datei(), &ort.innen).join(name),
            namensgrund_ohne_ziel(name),
        ));
    };
    let wo = archivbezeichnung(ziel.datei(), &ziel.innen);
    if let Err(fehler) = name_pruefen(name) {
        return Err((wo, fehler.grund().to_owned()));
    }
    if verzeichnis.eintrag(&ziel.innen).is_some() {
        return Err((wo, vergeben()));
    }
    let (rechte, art) = match typ {
        Anlegetyp::Ordner => (0o755, Neuart::Ordner(Vec::new())),
        Anlegetyp::Datei => (0o644, Neuart::Datei(Box::new(Leer))),
    };
    Ok((
        wo.clone(),
        Umbauschritt::Hinzufuegen {
            ordner: ort.innen.clone(),
            eintrag: Neueintrag {
                name: name.to_owned(),
                herkunft: wo,
                rechte,
                geaendert: SystemTime::now(),
                art,
            },
        },
        0,
    ))
}

// ----------------------------------------------------------------------
// Zulieferung: Daten aus einem Archiv oder von einem Server fuer den Umbau
// ----------------------------------------------------------------------

/// Wie eine [`Zulieferung`] ihre Daten liest: in Bloecken an einen
/// Rueckruf, auf einem eigenen Faden mit eigenem Pfand.
type Lesung =
    Box<dyn FnOnce(&Fernfaden, &mut dyn FnMut(&[u8]) -> ControlFlow<()>) -> io::Result<()> + Send>;

/// Die Daten einer neuen Datei, die in einem Archiv oder auf einem Server
/// liegen (Modulkopf).
///
/// Geoeffnet startet sie einen Faden, der die Daten liest und in Bloecken
/// ueber einen Kanal reicht; der Leser, den der Umbau bekommt, zieht sie von
/// dort. Endet der Faden ohne Fehler, ist das Ende der Daten erreicht; ein
/// Fehler kommt als Fehler des Lesers an, und der Umbau laesst das Archiv
/// stehen.
struct Zulieferung {
    groesse: u64,
    lesung: Option<Lesung>,
}

impl Zulieferung {
    /// Der Eintrag `innen` aus dem geteilten Leser eines Archivs.
    fn aus_archiv(leser: Arc<Mutex<Archivleser>>, innen: Innenpfad, groesse: u64) -> Self {
        Zulieferung {
            groesse,
            lesung: Some(Box::new(move |pfand, empfang| {
                leser
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .lesen(pfand, &innen, |block| empfang(block))
                    .map(|_| ())
                    .map_err(|fehler| match fehler {
                        Lesefehler::Lesen(fehler) => fehler,
                        andere => io::Error::other(andere.grund()),
                    })
            })),
        }
    }

    /// Die Datei an einer Stelle, lokal oder auf einem Server.
    fn von_stelle(stelle: Stelle, groesse: u64) -> Self {
        Zulieferung {
            groesse,
            lesung: Some(Box::new(move |pfand, empfang| {
                stelle
                    .lesen(pfand, |block| empfang(block))
                    .map(|_| ())
                    .map_err(|fehler| match fehler {
                        Fehlschlag::Lokal(fehler) => fehler,
                        andere => io::Error::other(grund(&andere)),
                    })
            })),
        }
    }
}

impl Datenquelle for Zulieferung {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        let lesung = self
            .lesung
            .take()
            .ok_or_else(|| io::Error::other(text(Text::ArchivQuelleVeraendert)))?;
        let (sender, empfaenger) = sync_channel::<io::Result<Vec<u8>>>(ZULIEFERTIEFE);
        let faden = fernzugriff::faden("krk-zulieferung".to_owned(), move |pfand| {
            let fehlerweg = sender.clone();
            let gelesen = lesung(pfand, &mut |block| {
                if sender.send(Ok(block.to_vec())).is_ok() {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            });
            if let Err(fehler) = gelesen {
                // Hoert niemand mehr zu, hat der Umbau aufgehoert zu lesen.
                let _ = fehlerweg.send(Err(fehler));
            }
        })?;
        Ok(Geoeffnet {
            leser: Box::new(Zulieferleser {
                empfang: Some(empfaenger),
                block: Vec::new(),
                stelle: 0,
                faden: Some(faden),
            }),
            groesse: self.groesse,
        })
    }
}

/// Der Leser einer geoeffneten [`Zulieferung`].
///
/// **Er wartet beim Fallenlassen auf seinen Faden**, nachdem er den Kanal
/// geschlossen hat: der Faden bricht dann am naechsten Block ab, und nach dem
/// Umbau liest keiner mehr an einer Quelle, die das Verschieben gleich
/// entfernt.
struct Zulieferleser {
    empfang: Option<Receiver<io::Result<Vec<u8>>>>,
    block: Vec<u8>,
    stelle: usize,
    faden: Option<JoinHandle<()>>,
}

impl Read for Zulieferleser {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        if puffer.is_empty() {
            return Ok(0);
        }
        while self.stelle >= self.block.len() {
            let Some(empfang) = &self.empfang else {
                return Ok(0);
            };
            match empfang.recv() {
                Ok(Ok(block)) => {
                    self.block = block;
                    self.stelle = 0;
                }
                Ok(Err(fehler)) => return Err(fehler),
                // Der Faden ist ohne Fehler zu Ende: das Ende der Daten.
                Err(_) => return Ok(0),
            }
        }
        let zahl = puffer.len().min(self.block.len() - self.stelle);
        puffer[..zahl].copy_from_slice(&self.block[self.stelle..self.stelle + zahl]);
        self.stelle += zahl;
        Ok(zahl)
    }
}

impl Drop for Zulieferleser {
    fn drop(&mut self) {
        drop(self.empfang.take());
        if let Some(faden) = self.faden.take() {
            // Ein Faden, der in Panik endete, hat nichts mehr zu sagen; der
            // Umbau hat sein Ende als Ende der Daten gelesen und an der
            // Groesse gemessen.
            let _ = faden.join();
        }
    }
}
