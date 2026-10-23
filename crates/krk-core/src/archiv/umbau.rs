//! Der Umbau eines Archivs: jede Aenderung schreibt das ganze Archiv neu und
//! ersetzt das alte zuletzt mit einem `rename`.
//!
//! ```text
//!  Umbauplan (Wert: Entfernen, Umhaengen, Hinzufuegen)
//!    │ umbauen(&Fernfaden, &Archivzugang, plan, &mut Steuerung)
//!    v
//!  1. Format beschreibbar?        Gz, Sqlite: NurLesbar, vor jedem Zugriff
//!  2. oeffnen, fstat, Verzeichnis zur Kennung; verschluesselt, verdeckt,
//!     harte Verweise? abweisen
//!  3. Aufstellung: jede Konfliktfrage am Verzeichnis     <- kein Byte
//!  4. ersetzen::ersetzen(ziel, kennung, rechte, |neu| ...)
//!       Zip:      neue Eintraege ueber die Kiste, dann jeder alte roh
//!       Tar, Tgz: der alte Strom Kopf fuer Kopf, neue am Ende
//! ```
//!
//! # Ganz oder gar nicht
//!
//! **Ein abgebrochener oder gescheiterter Umbau laesst das Archiv
//! unveraendert**, auch die Eintraege, die bis dahin in das neue geschrieben
//! waren (E13 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Das weicht
//! bewusst vom lokalen Kopieren ab, nach dem stehen bleibt, was fertig ist;
//! die Zusage, dass ein Fehlschlag das Archiv nicht beschaedigt, geht vor.
//! Gehalten wird das allein von [`super::ersetzen`]: das Original wird nie
//! zum Schreiben geoeffnet, und erst das letzte `rename` macht das neue
//! Archiv sichtbar.
//!
//! # Alle Fragen vor dem ersten Byte
//!
//! Der Plan kommt als Wert herein und wird in zwei Haelften abgearbeitet.
//! Die erste ([`Aufstellung`]) rechnet allein am Inhaltsverzeichnis und
//! stellt jede Konfliktfrage ueber die hereingereichte [`Steuerung`], an
//! dasselbe Blatt wie jeder andere Vorgang: Ordner auf Ordner fuehrt ohne
//! Frage zusammen, „Ueberschreiben“ ersetzt den Eintrag und bei einem Ordner
//! den ganzen Unterbaum, „Umbenennen“ nimmt einen Namen, den weder das
//! Verzeichnis noch ein schon geplanter Eintrag traegt, „Ueberspringen“
//! laesst aus, und Quelle gleich Ziel sowie ein Ordner in sich selbst werden
//! abgewiesen wie lokal. Erst die zweite Haelfte schreibt, und sie fragt
//! nichts mehr.
//!
//! # Was ein unveraenderter Eintrag ist
//!
//! **Zip:** lokaler Kopf, gepackte Daten, Datenbeschreiber und der zentrale
//! Satz gehen Byte fuer Byte hinueber, bis auf den Versatz des lokalen Kopfs;
//! auch ein Eintrag mit einem Verfahren, das KRK nicht entpackt. Warum das
//! nicht `raw_copy_file` der Kiste ist, steht im Kopf von `zipform`.
//! **Tar und tar.gz:** die Koepfe des Eintrags samt seinen Vorkoepfen (lange
//! Namen, PAX) und seine Daten gehen Byte fuer Byte hinueber. Ein
//! umbenannter Eintrag behaelt Daten, Rechte, Zeiten, Besitzer und jede
//! PAX-Angabe ausser dem Namen, den ein neuer PAX-Kopf traegt.
//!
//! # Was im Fenster nicht steht
//!
//! Ein Eintrag, den das Inhaltsverzeichnis nicht zeigt, bleibt unveraendert,
//! **sofern sein Name kein gueltiger Pfad ist** (unsichere Namen, Namen, die
//! kein UTF-8 sind, und die Wurzel `./`): er kann nie erscheinen und mit
//! keinem Namen zusammenstossen. Ein nicht gezeigter Eintrag **mit** einem
//! gueltigen Pfad ist verdeckt, ein frueherer von doppelten Namen, eine
//! Datei, die einem Ordner weichen musste, oder ein Eintrag hinter einer
//! Verknuepfung. **Ein Archiv mit einem verdeckten Eintrag schreibt der Umbau
//! nicht** ([`Umbaufehler::Verdeckt`], Entscheid
//! `261017-1923-darf-jeder-umbau-die-verdeckten-eintraege-eines-archivs-verwerfen.md`,
//! Moeglichkeit 3): behielte er ihn, kaeme er nach dem Entfernen des
//! verdeckenden wieder hervor, und „Loeschen“ hiesse „Aufdecken“; verwuerfe er
//! ihn, entfernte eine Aenderung an einer Stelle Daten an einer anderen, nach
//! denen niemand gefragt hat. Gefragt wird zweimal: vor der Aufstellung an
//! [`Inhaltsverzeichnis::verdeckt`] und beim Schreiben an jedem alten Eintrag,
//! den das Verzeichnis nicht fuehrt und dessen Name ein gueltiger Pfad ist.
//!
//! # Was der Umbau nicht schreibt
//!
//! Eine Archivdatei, die dieser Prozess nicht beschreiben darf
//! ([`Umbaufehler::Schreibgeschuetzt`], gefragt ueber `access(2)` an der
//! aufgeloesten Datei): ersetzt wird sie zwar ueber ein `rename` im Ordner,
//! fuer das das Recht am Ordner genuegte, aber eine Datei mit `chmod a-w` hat
//! der Nutzer gegen Aenderungen gesperrt, und KRK oeffnet eine solche Datei
//! auch lokal nie zum Schreiben.
//! Ein Archiv mit einem verschluesselten Eintrag: dass eine rohe Kopie einen
//! solchen Eintrag unversehrt hinuebertraegt, ist nicht belegt.
//! Ein Archiv mit einem verdeckten Eintrag (oben).
//! Ein tar mit einem harten Verweis ([`Unumschreibbar::TarHarteVerweise`]):
//! ein harter Verweis hat keine eigenen Daten, sondern nennt einen frueheren
//! Eintrag, und ein Umbau, der jenen entfernt oder umhaengt, liesse den
//! Verweis ins Leere zeigen. Abgewiesen wird jedes solche tar und nicht allein
//! der Schritt, der das Ziel beruehrt: das Ziel eines Verweises kann auch ein
//! Eintrag sein, den das Fenster nicht zeigt, und die vorsichtige Regel braucht
//! keine Zuordnung von Verweis zu Ziel. Gefragt wird wie beim verdeckten
//! Eintrag vor der Aufstellung und beim Schreiben.
//! Eine gz-Datei: [`Schreibform`] hat keinen Fall fuer sie, also gibt es
//! keinen Schreiber, den der Uebersetzer fuer sie annaehme.
//! Ein Zip mit Daten vor dem ersten Eintrag oder ueber mehrere Teile, ein tar
//! mit einer GNU-Datei mit Luecken und ein tar, dessen PAX-Kopf eine andere
//! Groesse nennt als der Kopf selbst ([`Unumschreibbar`]): der Strom liesse
//! sich dort nicht Kopf fuer Kopf treu lesen.
//!
//! # Was eine Aenderung kostet
//!
//! Jede Aenderung schreibt das ganze Archiv neu: ein Zip kostet rohe Kopien
//! in seiner Groesse, ein tar.gz ein vollstaendiges Entpacken und Packen. Der
//! Ordner des Archivs braucht voruebergehend so viel freien Platz wie das
//! neue Archiv. Fortschritt und Abbruch gehen je geschriebenem Block ueber
//! [`Ausgabe`], und so reicht der Abbruch auch in die Kopie eines einzelnen
//! grossen Eintrags hinein.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::Compression;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use tar::{EntryType, Header};
use zip::result::ZipError;
use zip::write::FullFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::fern::{Fernfaden, faden};
use crate::operation::fortschritt::{
    Abschluss, Bericht, Konfliktantwort, Lauf, Meldung, Steuerung, Uebersprungen,
};
use crate::operation::{Konfliktregel, freier_name_unter, grund, name_pruefen};
use crate::sprache::{Text, satz, text, zahl};
use crate::verzeichnis::Typ;
use crate::verzeichnis::sys::{ohne_warten_oeffnen, schreiben_erlaubt};

use super::HOECHSTENS_EINTRAEGE;
use super::ersetzen::ersetzen;
use super::format::Archivformat;
use super::innenpfad::Innenpfad;
use super::verzeichnis::{
    Abbruchleser, Fundstelle, HOECHSTENS_VORKOPF, Inhaltsverzeichnis, Lesbarkeit, VORKOPF_ZU_GROSS,
    Verzeichnisfehler,
};
use super::zipform::{self, Zentralsatz};
use super::zugang::{Archivzugang, Dateikennung};

/// Die Groesse des Puffers, mit dem Daten von einem Archiv ins andere gehen.
const PUFFER: usize = 256 * 1024;

// ---------------------------------------------------------------------------
// Der Plan als Wert
// ---------------------------------------------------------------------------

/// Die Daten eines neuen Eintrags, geoeffnet: ein Leser und die Groesse, die
/// er liefern soll.
pub struct Geoeffnet<'a> {
    /// Liefert die Daten.
    pub leser: Box<dyn Read + 'a>,
    /// Wie viele Bytes der Leser liefert. Liefert er weniger oder mehr,
    /// scheitert der Umbau, und das Archiv bleibt unveraendert.
    pub groesse: u64,
}

/// Woher die Daten eines neuen Eintrags kommen.
///
/// Geoeffnet wird erst beim Schreiben, also nach allen Konfliktfragen, und
/// auf dem Arbeitsfaden des Umbaus; das Pfand reicht deshalb hinein, damit
/// eine Quelle auf einem Server ihre Anfragen stellen kann.
pub trait Datenquelle: Send {
    /// Oeffnet die Daten; wird hoechstens einmal gerufen.
    ///
    /// # Errors
    ///
    /// Der Fehler des Oeffnens; er bricht den Umbau ab, und das Archiv bleibt
    /// unveraendert.
    fn oeffnen<'a>(&'a mut self, pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>>;
}

/// Die Daten einer neuen, leeren Datei.
#[derive(Debug, Clone, Copy, Default)]
pub struct Leer;

impl Datenquelle for Leer {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        Ok(Geoeffnet {
            leser: Box::new(io::empty()),
            groesse: 0,
        })
    }
}

/// Die Daten einer Datei auf dieser Platte.
///
/// Geoeffnet wird ohne Warten, und der Typ wird am Deskriptor gefragt, aus
/// dem Grund im Kopf von `verzeichnis::sys::ohne_warten_oeffnen`.
#[derive(Debug, Clone)]
pub struct LokaleDatei {
    pfad: PathBuf,
}

impl LokaleDatei {
    /// Die Datei unter `pfad`.
    #[must_use]
    pub fn neu(pfad: PathBuf) -> Self {
        LokaleDatei { pfad }
    }
}

impl Datenquelle for LokaleDatei {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        let datei = ohne_warten_oeffnen(&self.pfad)?;
        let angaben = datei.metadata()?;
        if !angaben.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                text(Text::VorgangKeineGewoehnlicheDatei),
            ));
        }
        Ok(Geoeffnet {
            leser: Box::new(BufReader::with_capacity(PUFFER, datei)),
            groesse: angaben.len(),
        })
    }
}

/// Was ein neuer Eintrag ist.
pub enum Neuart {
    /// Ein Ordner mit seinen Kindern.
    Ordner(Vec<Neueintrag>),
    /// Eine Datei mit der Quelle ihrer Daten.
    Datei(Box<dyn Datenquelle>),
    /// Eine Verknuepfung mit ihrem Verweistext.
    Verknuepfung(String),
}

/// Ein Eintrag, der ins Archiv kommt.
pub struct Neueintrag {
    /// Der Name, eine einzelne Komponente.
    pub name: String,
    /// Woher er kommt, fuer die Konfliktfrage und den Bericht.
    pub herkunft: PathBuf,
    /// Die Rechte (`& 0o7777`).
    pub rechte: u32,
    /// Das Aenderungsdatum.
    pub geaendert: SystemTime,
    /// Was er ist.
    pub art: Neuart,
}

/// Ein lokaler Baum, wie er ins Archiv kaeme ([`Neueintrag::lokal`]).
pub struct Lokalbaum {
    /// Der Eintrag zum Pfad samt Unterbaum, oder `None`, wenn der Pfad selbst
    /// eine Sonderdatei ist.
    pub eintrag: Option<Neueintrag>,
    /// Die [`Lokalspur`] jedes Eintrags des Baums, Kinder vor ihrem Ordner:
    /// das, was ein Verschieben nach dem Umbau entfernen darf, und nichts, was
    /// erst danach hinzukam.
    pub spur: Vec<Lokalspur>,
    /// Die Sonderdateien des Baums (benannte Roehre, Sockel, Geraet), die
    /// nicht ins Archiv kommen; der Rufer nennt sie im Bericht.
    pub ausgelassen: Vec<PathBuf>,
}

impl Neueintrag {
    /// Der Eintrag zu `pfad` auf dieser Platte samt seinem Unterbaum, wie er
    /// ins Archiv kaeme, mit Spur und ausgelassenen Sonderdateien.
    ///
    /// Eine Verknuepfung bleibt eine Verknuepfung. **Was weder Datei noch
    /// Ordner noch Verknuepfung ist, kommt nicht hinein und haelt den Umbau
    /// nicht auf**: gefragt wird der Typ schon hier, an `lstat(2)`, und eine
    /// Sonderdatei steht in [`Lokalbaum::ausgelassen`], wie der ferne Weg sie
    /// mit Grund auslaesst. Die Quelle einer Datei fragt beim Oeffnen ein
    /// zweites Mal, am Deskriptor; was dazwischen zur Sonderdatei wird, laesst
    /// den Umbau scheitern und das Archiv unveraendert. Das Pfand steht da,
    /// weil jede Frage hier eine Frage an die Platte ist.
    ///
    /// # Errors
    ///
    /// Der Fehler des Systems an `pfad` oder an einem Eintrag darunter.
    pub fn lokal(_pfand: &Fernfaden, pfad: &Path) -> io::Result<Lokalbaum> {
        let mut spur = Vec::new();
        let mut ausgelassen = Vec::new();
        let eintrag = Self::lokal_lesen(pfad, &mut spur, &mut ausgelassen)?;
        Ok(Lokalbaum {
            eintrag,
            spur,
            ausgelassen,
        })
    }

    fn lokal_lesen(
        pfad: &Path,
        spur: &mut Vec<Lokalspur>,
        ausgelassen: &mut Vec<PathBuf>,
    ) -> io::Result<Option<Self>> {
        let angaben = fs::symlink_metadata(pfad)?;
        let name = pfad
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let art = if angaben.file_type().is_symlink() {
            Neuart::Verknuepfung(fs::read_link(pfad)?.to_string_lossy().into_owned())
        } else if angaben.is_dir() {
            let mut kinder = Vec::new();
            let mut namen: Vec<PathBuf> = fs::read_dir(pfad)?
                .map(|eintrag| eintrag.map(|eintrag| eintrag.path()))
                .collect::<io::Result<_>>()?;
            namen.sort();
            for kind in namen {
                if let Some(neu) = Neueintrag::lokal_lesen(&kind, spur, ausgelassen)? {
                    kinder.push(neu);
                }
            }
            Neuart::Ordner(kinder)
        } else if angaben.is_file() {
            Neuart::Datei(Box::new(LokaleDatei::neu(pfad.to_path_buf())))
        } else {
            ausgelassen.push(pfad.to_path_buf());
            return Ok(None);
        };
        spur.push(Lokalspur {
            pfad: pfad.to_path_buf(),
            ordner: angaben.is_dir(),
            kennung: Dateikennung::aus_angaben(&angaben),
        });
        Ok(Some(Neueintrag {
            name,
            herkunft: pfad.to_path_buf(),
            rechte: angaben.permissions().mode() & 0o7777,
            geaendert: angaben.modified().unwrap_or(UNIX_EPOCH),
            art,
        }))
    }
}

/// Ein lokaler Eintrag, wie er beim Zusammenstellen eines Baums stand
/// ([`Neueintrag::lokal`]).
#[derive(Debug, Clone)]
pub struct Lokalspur {
    /// Der Pfad auf der Platte.
    pub pfad: PathBuf,
    /// Ob er ein Ordner war; dann entscheidet beim Entfernen `rmdir(2)`, ob
    /// etwas hinzukam.
    pub ordner: bool,
    /// Geraet, Inode, Groesse und Aenderungszeit beim Zusammenstellen, an
    /// `lstat(2)` gefragt.
    kennung: Dateikennung,
}

impl Lokalspur {
    /// Ob unter dem Pfad noch derselbe Eintrag steht, mit derselben Groesse
    /// und Aenderungszeit wie beim Zusammenstellen. Bei einem Ordner sagt das
    /// nichts, denn das Entfernen seiner Kinder aendert seine Zeit.
    ///
    /// # Errors
    ///
    /// Der Fehler von `lstat(2)` am Pfad.
    pub fn unveraendert(&self) -> io::Result<bool> {
        let jetzt = fs::symlink_metadata(&self.pfad)?;
        Ok(Dateikennung::aus_angaben(&jetzt) == self.kennung)
    }
}

/// Ein Schritt eines Umbaus.
pub enum Umbauschritt {
    /// Entfernt einen Eintrag samt Unterbaum.
    Entfernen(Innenpfad),
    /// Haengt einen Eintrag samt Unterbaum unter einen neuen Pfad, ohne Daten
    /// zu kopieren: Umbenennen und Verschieben im selben Archiv.
    Umhaengen {
        /// Wo er steht.
        von: Innenpfad,
        /// Wo er stehen soll, samt Namen.
        nach: Innenpfad,
    },
    /// Legt einen neuen Eintrag samt Unterbaum in einen Ordner des Archivs.
    Hinzufuegen {
        /// Der Ordner im Archiv; die Wurzel ist der leere Pfad.
        ordner: Innenpfad,
        /// Der Eintrag.
        eintrag: Neueintrag,
    },
}

/// Die Schritte eines Umbaus, in der Reihenfolge, in der sie wirken.
#[derive(Default)]
pub struct Umbauplan {
    schritte: Vec<Umbauschritt>,
}

impl Umbauplan {
    /// Ein Plan ohne Schritt.
    #[must_use]
    pub fn neu() -> Self {
        Umbauplan::default()
    }

    /// Haengt einen Schritt an und gibt seine Nummer zurueck, unter der
    /// [`Umgebaut::schritte`] ihn fuehrt.
    pub fn anfuegen(&mut self, schritt: Umbauschritt) -> usize {
        self.schritte.push(schritt);
        self.schritte.len() - 1
    }

    /// Der Plan mit einem weiteren Schritt.
    #[must_use]
    pub fn mit(mut self, schritt: Umbauschritt) -> Self {
        self.schritte.push(schritt);
        self
    }
}

/// Wie ein Schritt ausgegangen ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schrittausgang {
    /// Der Schritt hat ganz gewirkt; der Pfad ist der, unter dem das Ergebnis
    /// steht (nach einer Umbenennung in der Konfliktfrage der neue).
    Vollstaendig(Innenpfad),
    /// Der Schritt hat gewirkt, aber nicht fuer jeden Eintrag darunter: eine
    /// Konfliktfrage hat einen ausgelassen.
    Teilweise(Innenpfad),
    /// Der Schritt hat nichts bewirkt.
    Ausgelassen,
}

/// Was ein Umbau bewirkt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "der Ausgang sagt, welche Schritte angekommen sind"]
pub struct Umgebaut {
    /// Je Schritt des Plans sein Ausgang.
    pub schritte: Vec<Schrittausgang>,
    /// Je Schritt die Bytes seiner neuen Daten.
    pub bytes: Vec<u64>,
    /// Ob das Archiv ersetzt worden ist; nein, wenn kein Schritt etwas
    /// geaendert haette.
    pub ersetzt: bool,
}

/// Was ein Archiv unschreibbar macht, obwohl sein Format beschreibbar ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unumschreibbar {
    /// Ein Zip mit Daten vor dem ersten Eintrag oder ueber mehrere Teile.
    ZipForm,
    /// Ein tar mit einer GNU-Datei mit Luecken.
    TarLuecken,
    /// Ein tar, dessen PAX-Kopf eine andere Groesse nennt als der Kopf.
    TarGroesseAusPax,
    /// Ein tar mit einem harten Verweis (Modulkopf).
    TarHarteVerweise,
}

/// Warum ein Umbau nichts ersetzt hat. Das Archiv ist in jedem Fall
/// unveraendert.
#[derive(Debug)]
pub enum Umbaufehler {
    /// Der Nutzer hat abgebrochen, in einer Konfliktfrage oder beim Schreiben.
    Abgebrochen,
    /// Das Format ist allein lesbar (gz, SQLite); der Grund folgt dem Format
    /// ([`Archivformat::nur_lesbar_text`]).
    NurLesbar(Archivformat),
    /// Die Archivdatei ist schreibgeschuetzt: dieser Prozess darf sie nicht
    /// beschreiben, auch wenn er ihren Ordner beschreiben duerfte (Modulkopf).
    Schreibgeschuetzt,
    /// Das Archiv traegt einen verschluesselten Eintrag.
    Verschluesselt,
    /// Das Archiv traegt einen Eintrag, den ein anderer verdeckt (Modulkopf).
    Verdeckt,
    /// Das Archiv hat sich waehrend des Umbaus von aussen geaendert.
    VonAussenVeraendert,
    /// Das Archiv hat eine Gestalt, die der Umbau nicht treu schreibt.
    Unumschreibbar(Unumschreibbar),
    /// Das alte Archiv liess sich nicht lesen.
    Lesen(Verzeichnisfehler),
    /// Die Quelle eines neuen Eintrags liess sich nicht lesen.
    Quelle {
        /// Die Quelle.
        bezeichnung: PathBuf,
        /// Der Fehler beim Lesen.
        fehler: io::Error,
    },
    /// Das neue Archiv liess sich nicht schreiben, auch ein voller
    /// Datentraeger.
    Schreiben(io::Error),
}

impl Umbaufehler {
    /// Der Grund im Klartext, wie die Abschlussliste ihn zeigt.
    #[must_use]
    pub fn grund(&self) -> String {
        match self {
            Umbaufehler::Abgebrochen => text(Text::ArchivUnveraendertNachAbbruch).to_owned(),
            Umbaufehler::NurLesbar(format) => text(format.nur_lesbar_text()).to_owned(),
            Umbaufehler::Schreibgeschuetzt => text(Text::ArchivSchreibgeschuetzt).to_owned(),
            Umbaufehler::Verschluesselt => text(Text::ArchivVerschluesseltNurLesbar).to_owned(),
            Umbaufehler::Verdeckt => text(Text::ArchivVerdecktNurLesbar).to_owned(),
            Umbaufehler::VonAussenVeraendert => text(Text::ArchivVonAussenVeraendert).to_owned(),
            Umbaufehler::Unumschreibbar(art) => text(match art {
                Unumschreibbar::ZipForm => Text::ArchivZipUngewoehnlicheForm,
                Unumschreibbar::TarLuecken => Text::ArchivTarMitLuecken,
                Unumschreibbar::TarGroesseAusPax => Text::ArchivTarGroesseAusPax,
                Unumschreibbar::TarHarteVerweise => Text::ArchivTarHarteVerweise,
            })
            .to_owned(),
            Umbaufehler::Lesen(fehler) => match fehler {
                Verzeichnisfehler::Abgebrochen => {
                    text(Text::ArchivUnveraendertNachAbbruch).to_owned()
                }
                Verzeichnisfehler::Lesen(fehler) => grund(fehler),
                Verzeichnisfehler::KeineDatei => {
                    text(Text::VorgangKeineGewoehnlicheDatei).to_owned()
                }
                Verzeichnisfehler::Ungueltig(wortlaut) => wortlaut.clone(),
                Verzeichnisfehler::Belegt => text(Text::DatenbankWirdGeschrieben).to_owned(),
                Verzeichnisfehler::ZuGross => satz(
                    Text::ArchivUmbauZuGross,
                    &[("grenze", &zahl(HOECHSTENS_EINTRAEGE))],
                ),
            },
            Umbaufehler::Quelle { fehler, .. } | Umbaufehler::Schreiben(fehler) => grund(fehler),
        }
    }

    /// Traegt den Fehler in die Abschlussliste ein: bei einer Quelle zuerst
    /// sie, und immer eine Zeile fuer das Archiv, das unveraendert ist.
    pub(crate) fn melden(&self, steuerung: &mut Steuerung, archiv: &Path) {
        if let Umbaufehler::Quelle {
            bezeichnung,
            fehler,
        } = self
        {
            let warum = grund(fehler);
            steuerung.ueberspringen(bezeichnung, warum);
        }
        let zeile = match self {
            Umbaufehler::Abgebrochen => text(Text::ArchivUnveraendertNachAbbruch).to_owned(),
            andere => satz(Text::ArchivUnveraendert, &[("grund", &andere.grund())]),
        };
        steuerung.ueberspringen(archiv, zeile);
    }
}

// ---------------------------------------------------------------------------
// Einstieg
// ---------------------------------------------------------------------------

/// Die Formate, in die der Umbau schreibt. Gz hat keinen Fall: eine
/// gzip-Datei fasst genau einen Strom (E6 des Plans), und ohne Fall gibt es
/// keinen Schreiber, den der Uebersetzer fuer sie annaehme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Schreibform {
    Zip,
    Tar,
    TarGz,
}

impl Schreibform {
    fn zu(format: Archivformat) -> Option<Self> {
        match format {
            Archivformat::Zip => Some(Schreibform::Zip),
            Archivformat::Tar => Some(Schreibform::Tar),
            Archivformat::TarGz => Some(Schreibform::TarGz),
            Archivformat::Gz | Archivformat::Sqlite => None,
        }
    }
}

/// Baut das Archiv von `zugang` nach `plan` um.
///
/// Stellt jede Konfliktfrage vor dem ersten geschriebenen Byte und schreibt
/// dann das ganze Archiv neu; siehe den Modulkopf.
///
/// # Errors
///
/// [`Umbaufehler`]; das Archiv ist dann in jedem Fall unveraendert.
pub fn umbauen(
    pfand: &Fernfaden,
    zugang: &Archivzugang,
    plan: Umbauplan,
    steuerung: &mut Steuerung,
) -> Result<Umgebaut, Umbaufehler> {
    umbauen_mit(pfand, zugang, None, plan, steuerung, None)
}

/// Wie [`umbauen`], und allein an der Datei mit der Kennung `erwartet`: traegt
/// die Archivdatei beim Oeffnen eine andere, endet der Umbau mit
/// [`Umbaufehler::VonAussenVeraendert`], bevor er ein Byte schreibt.
///
/// Das ist der Weg fuer ein Entfernen nach einem Kopieren aus dem Archiv:
/// entfernt werden darf allein, was aus genau dieser Datei kopiert ist, und
/// nicht, was ein anderes Programm inzwischen unter demselben Namen ablegte.
///
/// # Errors
///
/// [`Umbaufehler`]; das Archiv ist dann in jedem Fall unveraendert.
pub fn umbauen_an_kennung(
    pfand: &Fernfaden,
    zugang: &Archivzugang,
    erwartet: Dateikennung,
    plan: Umbauplan,
    steuerung: &mut Steuerung,
) -> Result<Umgebaut, Umbaufehler> {
    umbauen_mit(pfand, zugang, Some(erwartet), plan, steuerung, None)
}

/// Wie [`umbauen`], mit der erwarteten Kennung aus [`umbauen_an_kennung`], und
/// `grenze` laesst das Schreiben nach so vielen Bytes scheitern wie ein voller
/// Datentraeger. Die Grenze gibt es fuer die Proben; der Betrieb reicht
/// `None`.
fn umbauen_mit(
    pfand: &Fernfaden,
    zugang: &Archivzugang,
    erwartet: Option<Dateikennung>,
    plan: Umbauplan,
    steuerung: &mut Steuerung,
    grenze: Option<u64>,
) -> Result<Umgebaut, Umbaufehler> {
    let form = Schreibform::zu(zugang.format()).ok_or(Umbaufehler::NurLesbar(zugang.format()))?;
    let abbruch = steuerung.abbruchkennzeichen();

    // Eine Verknuepfung auf das Archiv bleibt eine Verknuepfung: geschrieben
    // wird an ihrem Ziel.
    let ziel = fs::canonicalize(zugang.datei())
        .map_err(|fehler| Umbaufehler::Lesen(Verzeichnisfehler::Lesen(fehler)))?;
    let alt = ohne_warten_oeffnen(&ziel)
        .map_err(|fehler| Umbaufehler::Lesen(Verzeichnisfehler::Lesen(fehler)))?;
    let angaben = alt
        .metadata()
        .map_err(|fehler| Umbaufehler::Lesen(Verzeichnisfehler::Lesen(fehler)))?;
    if !angaben.is_file() {
        return Err(Umbaufehler::Lesen(Verzeichnisfehler::KeineDatei));
    }
    let kennung = Dateikennung::aus_angaben(&angaben);
    if erwartet.is_some_and(|erwartet| erwartet != kennung) {
        return Err(Umbaufehler::VonAussenVeraendert);
    }
    // Ein Archiv, das der Nutzer gegen Aenderungen gesperrt hat, bleibt es
    // (Modulkopf, „Was der Umbau nicht schreibt“).
    if !schreiben_erlaubt(&ziel)
        .map_err(|fehler| Umbaufehler::Lesen(Verzeichnisfehler::Lesen(fehler)))?
    {
        return Err(Umbaufehler::Schreibgeschuetzt);
    }
    let verzeichnis = verzeichnis_zur_kennung(pfand, zugang, &abbruch, kennung)?;
    if verzeichnis.verschluesselt() {
        return Err(Umbaufehler::Verschluesselt);
    }
    if verzeichnis.verdeckt() > 0 {
        return Err(Umbaufehler::Verdeckt);
    }
    if verzeichnis
        .eintraege()
        .iter()
        .any(|eintrag| eintrag.lesbarkeit == Lesbarkeit::HarterVerweis)
    {
        return Err(Umbaufehler::Unumschreibbar(
            Unumschreibbar::TarHarteVerweise,
        ));
    }

    let schritte = plan.schritte.len();
    let mut aufstellung = Aufstellung::neu(&verzeichnis, zugang.datei());
    let ausgaenge = aufstellung.plan_abarbeiten(plan, steuerung)?;
    if !aufstellung.veraendert {
        return Ok(Umgebaut {
            schritte: ausgaenge,
            bytes: vec![0; schritte],
            ersetzt: false,
        });
    }
    aufstellung.erschlossene_festhalten();
    let ziele = aufstellung.ziele();
    let mut neue = aufstellung.neue;

    let rechte = angaben.permissions().mode() & 0o7777;
    let besitzer = (u64::from(angaben.uid()), u64::from(angaben.gid()));
    let archiv = zugang.datei().to_path_buf();
    let mut bytes = vec![0u64; schritte];
    ersetzen(&ziel, kennung, rechte, &abbruch, |datei| {
        let mut aus = Ausgabe {
            datei,
            steuerung,
            archiv: &archiv,
            geschrieben: 0,
            grenze,
            abbruch: Arc::clone(&abbruch),
        };
        let mut alt = alt;
        match form {
            Schreibform::Zip => zip_schreiben(
                pfand,
                &mut alt,
                &mut aus,
                &verzeichnis,
                &ziele,
                &mut neue,
                &mut bytes,
            )?,
            Schreibform::Tar | Schreibform::TarGz => tar_schreiben(
                pfand,
                alt,
                form == Schreibform::TarGz,
                &mut aus,
                &verzeichnis,
                &ziele,
                &mut neue,
                besitzer,
                &mut bytes,
            )?,
        }
        Ok(())
    })?;
    Ok(Umgebaut {
        schritte: ausgaenge,
        bytes,
        ersetzt: true,
    })
}

/// Das Verzeichnis des Archivs, und zwar das zur Kennung des offenen
/// Deskriptors. Stimmt das gespeicherte nicht, wird es einmal neu gelesen;
/// stimmt auch das nicht, hat sich die Datei zwischen Oeffnen und Lesen
/// geaendert.
fn verzeichnis_zur_kennung(
    pfand: &Fernfaden,
    zugang: &Archivzugang,
    abbruch: &AtomicBool,
    kennung: Dateikennung,
) -> Result<Arc<Inhaltsverzeichnis>, Umbaufehler> {
    for _ in 0..2 {
        let verzeichnis = zugang
            .verzeichnis(pfand, abbruch)
            .map_err(verzeichnisfehler)?;
        if verzeichnis.kennung() == kennung {
            return Ok(verzeichnis);
        }
    }
    Err(Umbaufehler::VonAussenVeraendert)
}

fn verzeichnisfehler(fehler: Verzeichnisfehler) -> Umbaufehler {
    match fehler {
        Verzeichnisfehler::Abgebrochen => Umbaufehler::Abgebrochen,
        andere => Umbaufehler::Lesen(andere),
    }
}

/// Startet einen Umbau auf einem eigenen Arbeitsfaden mit Pfand und kehrt
/// sofort zurueck.
///
/// Der [`Lauf`] meldet die Konfliktfragen, den Fortschritt in geschriebenen
/// Bytes und zuletzt den [`Bericht`]: je angekommenem Schritt ein Eintrag,
/// und bei einem Fehler oder Abbruch eine Zeile fuer das unveraenderte
/// Archiv.
pub fn starten(zugang: Arc<Archivzugang>, plan: Umbauplan, regel: Konfliktregel) -> Lauf {
    let abbruch = Arc::new(AtomicBool::new(false));
    let faden_abbruch = Arc::clone(&abbruch);
    let (sender, empfaenger) = channel();
    let abschlusssender = sender.clone();
    let notsender = sender.clone();
    let archiv = zugang.datei().to_path_buf();

    let gestartet = faden("krk-umbau".to_owned(), move |pfand| {
        let mut steuerung = Steuerung::neu(faden_abbruch, Some(sender), regel);
        let abschluss = berichten(pfand, &zugang, plan, &mut steuerung);
        let _ = abschlusssender.send(Meldung::Fertig(steuerung.bericht(abschluss)));
    });
    let faden = match gestartet {
        Ok(faden) => Some(faden),
        Err(fehler) => {
            let _ = notsender.send(Meldung::Fertig(Bericht {
                abschluss: Abschluss::Fertig,
                eintraege: 0,
                bytes: 0,
                uebersprungen: vec![Uebersprungen {
                    pfad: archiv,
                    grund: satz(Text::VorgangKeinArbeitsfaden, &[("grund", &grund(&fehler))]),
                }],
            }));
            None
        }
    };
    Lauf::neu(abbruch, empfaenger, faden)
}

/// Der Abschluss eines Vorgangs an der Stelle, nach der nichts mehr kommt,
/// das ein Abbruch noch anhielte; `ersetzte` sind die Archive, die dieser
/// Vorgang ersetzt hat.
///
/// **Ein Abbruch heisst „das Archiv ist unveraendert“** (E13 des Plans
/// `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Hat der
/// Vorgang ein Archiv ersetzt, stimmte das nicht mehr: dann endet er als
/// fertig, und ein Abbruch, der danach kam, steht je ersetztem Archiv als
/// Zeile im Bericht. Ohne ersetztes Archiv bleibt ein gesetzter Abbruch ein
/// Abbruch. Bis zum `rename` fragt [`super::ersetzen`] den Abbruch selbst;
/// dazwischen gibt es keinen dritten Ausgang.
pub(crate) fn abschluss_nach_ersetzen(
    steuerung: &mut Steuerung,
    ersetzte: &[PathBuf],
) -> Abschluss {
    if !steuerung.abgebrochen() {
        return Abschluss::Fertig;
    }
    if ersetzte.is_empty() {
        return Abschluss::Abgebrochen;
    }
    for archiv in ersetzte {
        steuerung.ueberspringen(archiv, text(Text::ArchivAbbruchNachDemErsetzen));
    }
    Abschluss::Fertig
}

/// Fuehrt den Umbau aus und traegt seinen Ausgang in die Steuerung ein.
fn berichten(
    pfand: &Fernfaden,
    zugang: &Archivzugang,
    plan: Umbauplan,
    steuerung: &mut Steuerung,
) -> Abschluss {
    let archiv = zugang.datei();
    match umbauen(pfand, zugang, plan, steuerung) {
        Ok(umgebaut) => {
            for (ausgang, bytes) in umgebaut.schritte.iter().zip(&umgebaut.bytes) {
                match ausgang {
                    Schrittausgang::Vollstaendig(innen) | Schrittausgang::Teilweise(innen) => {
                        steuerung.eintrag_fertig(&bezeichnung(archiv, innen.als_text()), *bytes);
                    }
                    Schrittausgang::Ausgelassen => {}
                }
            }
            let ersetzte: &[PathBuf] = if umgebaut.ersetzt {
                &[archiv.to_path_buf()]
            } else {
                &[]
            };
            abschluss_nach_ersetzen(steuerung, ersetzte)
        }
        Err(fehler) => {
            fehler.melden(steuerung, archiv);
            match fehler {
                Umbaufehler::Abgebrochen => Abschluss::Abgebrochen,
                _ => Abschluss::Fertig,
            }
        }
    }
}

/// Die Bezeichnung eines Pfads im Archiv fuer Konfliktfrage und Bericht, in
/// der Schreibweise des Fenstertitels: `/pfad/zum/archiv.zip:/innen/pfad`.
fn bezeichnung(archiv: &Path, innen: &str) -> PathBuf {
    PathBuf::from(format!("{}:/{innen}", archiv.display()))
}

// ---------------------------------------------------------------------------
// Erste Haelfte: die Aufstellung am Verzeichnis
// ---------------------------------------------------------------------------

/// Woher ein Eintrag des umgebauten Archivs kommt.
#[derive(Debug, Clone, Copy)]
enum Herkunft {
    /// Ein Eintrag des alten Archivs, mit seiner Stelle im Verzeichnis.
    Alt(usize),
    /// Ein Ordner ohne eigenen Eintrag, mit seinem Datum.
    Erschlossen(SystemTime),
    /// Ein neuer Eintrag, mit seiner Stelle in [`Aufstellung::neue`].
    Neu(usize),
}

#[derive(Debug, Clone, Copy)]
struct Knoten {
    typ: Typ,
    herkunft: Herkunft,
}

/// Die Daten eines neuen Eintrags nach der Aufstellung.
enum Neudaten {
    Ordner,
    Datei(Box<dyn Datenquelle>),
    Verknuepfung(String),
}

/// Ein neuer Eintrag nach der Aufstellung.
struct Neuer {
    daten: Neudaten,
    rechte: u32,
    geaendert: SystemTime,
    herkunft: PathBuf,
    /// Der Schritt, der ihn bringt; ein festgehaltener erschlossener Ordner
    /// gehoert zu keinem.
    schritt: Option<usize>,
}

/// Wohin ein Eintrag nach der Konfliktfrage geht.
enum Zielwahl {
    /// Unter diesen Pfad; er ist frei.
    Frei(String),
    /// In diesen vorhandenen Ordner, zusammengefuehrt.
    Zusammenfuehren(String),
    /// Nirgends.
    Ueberspringen,
}

/// Der Baum des umgebauten Archivs, gerechnet am Verzeichnis, bevor ein Byte
/// geschrieben ist.
///
/// Die Schluessel sind die Pfade als Text. Ein Unterbaum ist damit ein
/// Bereich: jeder Pfad unter `p` beginnt mit `p/`, und `0` ist das Zeichen
/// nach `/`.
struct Aufstellung<'v> {
    archiv: &'v Path,
    baum: BTreeMap<String, Knoten>,
    neue: Vec<Neuer>,
    veraendert: bool,
}

fn verbinden(eltern: &str, name: &str) -> String {
    if eltern.is_empty() {
        name.to_owned()
    } else {
        format!("{eltern}/{name}")
    }
}

fn eltern_von(pfad: &str) -> &str {
    pfad.rfind('/').map_or("", |schnitt| &pfad[..schnitt])
}

fn name_von(pfad: &str) -> &str {
    pfad.rfind('/').map_or(pfad, |schnitt| &pfad[schnitt + 1..])
}

/// Ob `pfad` echt unter `vorfahr` liegt.
fn liegt_unter(pfad: &str, vorfahr: &str) -> bool {
    if vorfahr.is_empty() {
        return !pfad.is_empty();
    }
    pfad.len() > vorfahr.len()
        && pfad.starts_with(vorfahr)
        && pfad.as_bytes()[vorfahr.len()] == b'/'
}

fn innenpfad(pfad: &str) -> Innenpfad {
    Innenpfad::aus_archivname(pfad).unwrap_or_default()
}

impl<'v> Aufstellung<'v> {
    fn neu(verzeichnis: &'v Inhaltsverzeichnis, archiv: &'v Path) -> Self {
        let baum = verzeichnis
            .eintraege()
            .iter()
            .enumerate()
            .map(|(stelle, eintrag)| {
                let herkunft = if eintrag.ist_erschlossen() {
                    Herkunft::Erschlossen(eintrag.geaendert)
                } else {
                    Herkunft::Alt(stelle)
                };
                (
                    eintrag.innen.als_text().to_owned(),
                    Knoten {
                        typ: eintrag.typ,
                        herkunft,
                    },
                )
            })
            .collect();
        Aufstellung {
            archiv,
            baum,
            neue: Vec::new(),
            veraendert: false,
        }
    }

    fn bezeichnung(&self, pfad: &str) -> PathBuf {
        bezeichnung(self.archiv, pfad)
    }

    fn ist_ordner(&self, pfad: &str) -> bool {
        pfad.is_empty()
            || self
                .baum
                .get(pfad)
                .is_some_and(|knoten| knoten.typ == Typ::Ordner)
    }

    /// Die Pfade des Unterbaums unter `pfad`, ihn selbst eingeschlossen.
    fn unterbaum(&self, pfad: &str) -> Vec<String> {
        let mut pfade: Vec<String> = Vec::new();
        if self.baum.contains_key(pfad) {
            pfade.push(pfad.to_owned());
        }
        pfade.extend(
            self.baum
                .range(format!("{pfad}/")..format!("{pfad}0"))
                .map(|(schluessel, _)| schluessel.clone()),
        );
        pfade
    }

    /// Die Namen unmittelbar unter dem Ordner `pfad`, der nicht die Wurzel
    /// ist.
    fn kinder(&self, pfad: &str) -> Vec<String> {
        self.baum
            .range(format!("{pfad}/")..format!("{pfad}0"))
            .map(|(schluessel, _)| &schluessel[pfad.len() + 1..])
            .filter(|rest| !rest.contains('/'))
            .map(str::to_owned)
            .collect()
    }

    fn entfernen_baum(&mut self, pfad: &str) {
        for schluessel in self.unterbaum(pfad) {
            self.baum.remove(&schluessel);
        }
        self.veraendert = true;
    }

    fn verschieben_baum(&mut self, von: &str, nach: &str) {
        for schluessel in self.unterbaum(von) {
            if let Some(knoten) = self.baum.remove(&schluessel) {
                self.baum
                    .insert(format!("{nach}{}", &schluessel[von.len()..]), knoten);
            }
        }
        self.veraendert = true;
    }

    /// Arbeitet die Schritte ab und gibt je Schritt seinen Ausgang zurueck.
    fn plan_abarbeiten(
        &mut self,
        plan: Umbauplan,
        steuerung: &mut Steuerung,
    ) -> Result<Vec<Schrittausgang>, Umbaufehler> {
        let mut ausgaenge = Vec::with_capacity(plan.schritte.len());
        for (nummer, schritt) in plan.schritte.into_iter().enumerate() {
            if steuerung.abgebrochen() {
                return Err(Umbaufehler::Abgebrochen);
            }
            let ausgang = match schritt {
                Umbauschritt::Entfernen(pfad) => self.schritt_entfernen(&pfad, steuerung),
                Umbauschritt::Umhaengen { von, nach } => {
                    self.schritt_umhaengen(&von, &nach, steuerung)?
                }
                Umbauschritt::Hinzufuegen { ordner, eintrag } => {
                    self.schritt_hinzufuegen(&ordner, eintrag, nummer, steuerung)?
                }
            };
            ausgaenge.push(ausgang);
        }
        Ok(ausgaenge)
    }

    fn schritt_entfernen(&mut self, pfad: &Innenpfad, steuerung: &mut Steuerung) -> Schrittausgang {
        let text_pfad = pfad.als_text();
        if text_pfad.is_empty() || !self.baum.contains_key(text_pfad) {
            let wo = self.bezeichnung(text_pfad);
            steuerung.ueberspringen(&wo, text(Text::VorgangGibtEsNichtMehr));
            return Schrittausgang::Ausgelassen;
        }
        self.entfernen_baum(text_pfad);
        Schrittausgang::Vollstaendig(pfad.clone())
    }

    fn schritt_umhaengen(
        &mut self,
        von: &Innenpfad,
        nach: &Innenpfad,
        steuerung: &mut Steuerung,
    ) -> Result<Schrittausgang, Umbaufehler> {
        let (von, nach) = (von.als_text(), nach.als_text());
        let wo = self.bezeichnung(von);
        let abweisung = if von.is_empty() || !self.baum.contains_key(von) {
            Some(Text::VorgangGibtEsNichtMehr)
        } else if nach.is_empty() {
            Some(Text::VorgangPfadBenenntKeinenEintrag)
        } else if !self.ist_ordner(eltern_von(nach)) {
            Some(Text::VorgangZielordnerFehlt)
        } else if nach == von {
            Some(Text::VorgangQuelleUndZielDerselbeEintrag)
        } else if liegt_unter(nach, von) {
            Some(Text::VorgangZielLiegtInDerQuelle)
        } else {
            None
        };
        if let Some(schluessel) = abweisung {
            steuerung.ueberspringen(&wo, text(schluessel));
            return Ok(Schrittausgang::Ausgelassen);
        }
        let ergebnis = self.umhaengen(von, eltern_von(nach), name_von(nach), steuerung)?;
        Ok(ausgang(ergebnis))
    }

    fn umhaengen(
        &mut self,
        von: &str,
        eltern: &str,
        name: &str,
        steuerung: &mut Steuerung,
    ) -> Result<Option<(bool, String)>, Umbaufehler> {
        let ordner = self.ist_ordner(von);
        let wo = self.bezeichnung(von);
        match self.ziel_klaeren(&wo, eltern, name, ordner, Some(von), steuerung)? {
            Zielwahl::Ueberspringen => Ok(None),
            Zielwahl::Frei(ziel) => {
                self.verschieben_baum(von, &ziel);
                Ok(Some((true, ziel)))
            }
            Zielwahl::Zusammenfuehren(ziel) => {
                let mut ganz = true;
                for kind in self.kinder(von) {
                    let quelle = verbinden(von, &kind);
                    match self.umhaengen(&quelle, &ziel, &kind, steuerung)? {
                        Some((kind_ganz, _)) => ganz &= kind_ganz,
                        None => ganz = false,
                    }
                }
                if self.kinder(von).is_empty() {
                    self.baum.remove(von);
                    self.veraendert = true;
                } else {
                    ganz = false;
                }
                Ok(Some((ganz, ziel)))
            }
        }
    }

    fn schritt_hinzufuegen(
        &mut self,
        ordner: &Innenpfad,
        eintrag: Neueintrag,
        nummer: usize,
        steuerung: &mut Steuerung,
    ) -> Result<Schrittausgang, Umbaufehler> {
        let ordner = ordner.als_text();
        if !self.ist_ordner(ordner) {
            steuerung.ueberspringen(&eintrag.herkunft, text(Text::VorgangZielordnerFehlt));
            return Ok(Schrittausgang::Ausgelassen);
        }
        if let Err(fehler) = name_pruefen(&eintrag.name) {
            steuerung.ueberspringen(&eintrag.herkunft, fehler.grund());
            return Ok(Schrittausgang::Ausgelassen);
        }
        let ergebnis = self.hinzufuegen(ordner, eintrag, nummer, steuerung)?;
        Ok(ausgang(ergebnis))
    }

    fn hinzufuegen(
        &mut self,
        eltern: &str,
        eintrag: Neueintrag,
        nummer: usize,
        steuerung: &mut Steuerung,
    ) -> Result<Option<(bool, String)>, Umbaufehler> {
        let Neueintrag {
            name,
            herkunft,
            rechte,
            geaendert,
            art,
        } = eintrag;
        let ordner = matches!(art, Neuart::Ordner(_));
        let wahl = self.ziel_klaeren(&herkunft, eltern, &name, ordner, None, steuerung)?;
        let (typ, daten, kinder) = match art {
            Neuart::Ordner(kinder) => (Typ::Ordner, Neudaten::Ordner, kinder),
            Neuart::Datei(quelle) => (Typ::Datei, Neudaten::Datei(quelle), Vec::new()),
            Neuart::Verknuepfung(verweis) => (
                Typ::Verknuepfung,
                Neudaten::Verknuepfung(verweis),
                Vec::new(),
            ),
        };
        let ziel = match wahl {
            Zielwahl::Ueberspringen => return Ok(None),
            Zielwahl::Zusammenfuehren(ziel) => ziel,
            Zielwahl::Frei(ziel) => {
                let stelle = self.neue.len();
                self.neue.push(Neuer {
                    daten,
                    rechte,
                    geaendert,
                    herkunft,
                    schritt: Some(nummer),
                });
                self.baum.insert(
                    ziel.clone(),
                    Knoten {
                        typ,
                        herkunft: Herkunft::Neu(stelle),
                    },
                );
                self.veraendert = true;
                ziel
            }
        };
        let mut ganz = true;
        for kind in kinder {
            if let Err(fehler) = name_pruefen(&kind.name) {
                steuerung.ueberspringen(&kind.herkunft, fehler.grund());
                ganz = false;
                continue;
            }
            match self.hinzufuegen(&ziel, kind, nummer, steuerung)? {
                Some((kind_ganz, _)) => ganz &= kind_ganz,
                None => ganz = false,
            }
        }
        Ok(Some((ganz, ziel)))
    }

    /// Klaert, wohin ein Eintrag namens `name` unter `eltern` geht, und
    /// stellt bei einem vorhandenen Namen die Konfliktfrage.
    ///
    /// `ordner` sagt, ob die Quelle ein Ordner ist: auf einen vorhandenen
    /// Ordner fuehrt sie dann ohne Frage zusammen. `quelle` ist der Pfad der
    /// Quelle im selben Archiv, sofern sie dort steht; ein Ziel, das sie
    /// selbst ist oder unter dem sie liegt, wird abgewiesen, denn
    /// „Ueberschreiben“ entfernte dort die Quelle mit.
    fn ziel_klaeren(
        &mut self,
        wo: &Path,
        eltern: &str,
        name: &str,
        ordner: bool,
        quelle: Option<&str>,
        steuerung: &mut Steuerung,
    ) -> Result<Zielwahl, Umbaufehler> {
        let mut name = name.to_owned();
        loop {
            let ziel = verbinden(eltern, &name);
            if let Some(quelle) = quelle {
                let abweisung = if quelle == ziel {
                    Some(Text::VorgangQuelleUndZielDerselbeEintrag)
                } else if liegt_unter(quelle, &ziel) {
                    Some(Text::VorgangQuelleIstTeilDesZiels)
                } else {
                    None
                };
                if let Some(schluessel) = abweisung {
                    steuerung.ueberspringen(wo, text(schluessel));
                    return Ok(Zielwahl::Ueberspringen);
                }
            }
            match self.baum.get(&ziel) {
                None => return Ok(Zielwahl::Frei(ziel)),
                Some(knoten) if ordner && knoten.typ == Typ::Ordner => {
                    return Ok(Zielwahl::Zusammenfuehren(ziel));
                }
                Some(_) => {}
            }
            let ziel_wo = self.bezeichnung(&ziel);
            let baum = &self.baum;
            let antwort = steuerung.konflikt_loesen_mit(wo, &ziel_wo, || {
                freier_name_unter(Path::new(&name), |kandidat| {
                    !baum.contains_key(&verbinden(eltern, kandidat))
                })
            });
            match antwort {
                Konfliktantwort::Ueberschreiben => {
                    self.entfernen_baum(&ziel);
                    return Ok(Zielwahl::Frei(ziel));
                }
                Konfliktantwort::Ueberspringen => return Ok(Zielwahl::Ueberspringen),
                Konfliktantwort::UmbenennenIn(neu) => {
                    if let Err(fehler) = name_pruefen(&neu) {
                        steuerung.ueberspringen(wo, fehler.grund());
                        return Ok(Zielwahl::Ueberspringen);
                    }
                    name = neu;
                }
                Konfliktantwort::Abbrechen => return Err(Umbaufehler::Abgebrochen),
            }
        }
    }

    /// Haelt jeden erschlossenen Ordner fest, unter dem kein Eintrag mehr
    /// steht: er bekommt einen eigenen Ordnereintrag und bleibt stehen wie in
    /// einem Dateisystem.
    fn erschlossene_festhalten(&mut self) {
        let mut getragen: HashSet<String> = HashSet::new();
        let traegt = |pfad: &str, getragen: &mut HashSet<String>| {
            let mut vorfahr = eltern_von(pfad);
            while !vorfahr.is_empty() && getragen.insert(vorfahr.to_owned()) {
                vorfahr = eltern_von(vorfahr);
            }
        };
        for (pfad, knoten) in &self.baum {
            if !matches!(knoten.herkunft, Herkunft::Erschlossen(_)) {
                traegt(pfad, &mut getragen);
            }
        }
        // Rueckwaerts kommt jeder Pfad vor seinen Vorfahren.
        let mut festzuhalten = Vec::new();
        for (pfad, knoten) in self.baum.iter().rev() {
            if let Herkunft::Erschlossen(zeit) = knoten.herkunft
                && !getragen.contains(pfad)
            {
                festzuhalten.push((pfad.clone(), zeit));
                traegt(pfad, &mut getragen);
            }
        }
        for (pfad, zeit) in festzuhalten {
            let stelle = self.neue.len();
            let herkunft = self.bezeichnung(&pfad);
            self.neue.push(Neuer {
                daten: Neudaten::Ordner,
                rechte: 0o755,
                geaendert: zeit,
                herkunft,
                schritt: None,
            });
            self.baum.insert(
                pfad,
                Knoten {
                    typ: Typ::Ordner,
                    herkunft: Herkunft::Neu(stelle),
                },
            );
        }
    }

    /// Wohin jeder alte Eintrag geht, und die neuen in der Folge ihrer Pfade,
    /// Ordner vor ihrem Inhalt.
    fn ziele(&self) -> Ziele {
        let mut alt = HashMap::new();
        let mut neu = Vec::new();
        for (pfad, knoten) in &self.baum {
            match knoten.herkunft {
                Herkunft::Alt(stelle) => {
                    alt.insert(stelle, pfad.clone());
                }
                Herkunft::Neu(stelle) => neu.push((pfad.clone(), stelle)),
                Herkunft::Erschlossen(_) => {}
            }
        }
        Ziele { alt, neu }
    }
}

fn ausgang(ergebnis: Option<(bool, String)>) -> Schrittausgang {
    match ergebnis {
        None => Schrittausgang::Ausgelassen,
        Some((true, pfad)) => Schrittausgang::Vollstaendig(innenpfad(&pfad)),
        Some((false, pfad)) => Schrittausgang::Teilweise(innenpfad(&pfad)),
    }
}

/// Das Ergebnis der Aufstellung, wie die Schreiber es brauchen.
struct Ziele {
    /// Je Stelle im alten Verzeichnis der neue Pfad; fehlt die Stelle, faellt
    /// der Eintrag weg.
    alt: HashMap<usize, String>,
    /// Die neuen Eintraege: Pfad und Stelle in [`Aufstellung::neue`].
    neu: Vec<(String, usize)>,
}

/// Was ein alter Eintrag beim Schreiben wird.
enum Altwahl {
    /// Byte fuer Byte hinueber.
    Roh,
    /// Unter diesem Namen, wie er im Archiv steht.
    Umbenannt(String),
    /// Faellt weg.
    Weg,
}

/// Der Name eines Eintrags, wie er im Archiv steht: ein Ordner mit `/`.
fn archivname(pfad: &str, typ: Typ) -> String {
    match typ {
        Typ::Ordner => format!("{pfad}/"),
        Typ::Datei | Typ::Verknuepfung => pfad.to_owned(),
    }
}

/// Ob ein nicht gezeigter Eintrag verdeckt ist: sein Name ist ein gueltiger
/// Pfad und nicht die Wurzel (Modulkopf).
fn verdeckt(name: &[u8]) -> bool {
    std::str::from_utf8(name)
        .ok()
        .and_then(Innenpfad::aus_archivname)
        .is_some_and(|pfad| !pfad.ist_wurzel())
}

impl Ziele {
    /// Was der alte Eintrag an der Stelle `stelle` des Verzeichnisses wird.
    fn fuer(&self, verzeichnis: &Inhaltsverzeichnis, stelle: usize) -> Altwahl {
        let eintrag = &verzeichnis.eintraege()[stelle];
        match self.alt.get(&stelle) {
            None => Altwahl::Weg,
            Some(pfad) if pfad == eintrag.innen.als_text() => Altwahl::Roh,
            Some(pfad) => Altwahl::Umbenannt(archivname(pfad, eintrag.typ)),
        }
    }
}

// ---------------------------------------------------------------------------
// Zweite Haelfte: schreiben
// ---------------------------------------------------------------------------

/// Die neue Archivdatei, wie die Schreiber sie sehen: zaehlt die Bytes,
/// meldet sie getaktet und fragt je Block nach dem Abbruch.
///
/// Ist der Abbruch gesetzt, scheitert das naechste Schreiben, und der Umbau
/// endet wie bei einem Fehler; so erreicht er auch eine Kopie, die einen
/// ganzen Eintrag in einem Aufruf schreibt. `grenze` laesst das Schreiben
/// nach so vielen Bytes mit `StorageFull` scheitern, fuer die Proben eines
/// vollen Datentraegers.
struct Ausgabe<'a> {
    datei: &'a mut File,
    steuerung: &'a mut Steuerung,
    archiv: &'a Path,
    geschrieben: u64,
    grenze: Option<u64>,
    abbruch: Arc<AtomicBool>,
}

impl Write for Ausgabe<'_> {
    fn write(&mut self, puffer: &[u8]) -> io::Result<usize> {
        if self.abbruch.load(Ordering::Acquire) {
            return Err(io::Error::other("abgebrochen"));
        }
        let erlaubt = match self.grenze {
            Some(grenze) => {
                let rest = grenze.saturating_sub(self.geschrieben);
                if rest == 0 && !puffer.is_empty() {
                    return Err(io::Error::from(io::ErrorKind::StorageFull));
                }
                puffer
                    .len()
                    .min(usize::try_from(rest).unwrap_or(usize::MAX))
            }
            None => puffer.len(),
        };
        let geschrieben = self.datei.write(&puffer[..erlaubt])?;
        self.geschrieben += geschrieben as u64;
        self.steuerung.zwischenstand(self.archiv, self.geschrieben);
        Ok(geschrieben)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.datei.flush()
    }
}

impl Seek for Ausgabe<'_> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        self.datei.seek(stelle)
    }
}

impl Read for Ausgabe<'_> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        self.datei.read(puffer)
    }
}

fn schreibfehler(fehler: io::Error, abbruch: &AtomicBool) -> Umbaufehler {
    if abbruch.load(Ordering::Acquire) {
        Umbaufehler::Abgebrochen
    } else {
        Umbaufehler::Schreiben(fehler)
    }
}

fn lesefehler(fehler: io::Error, abbruch: &AtomicBool) -> Umbaufehler {
    verzeichnisfehler(Verzeichnisfehler::aus_strom(fehler, abbruch))
}

fn quellfehler(herkunft: &Path, fehler: io::Error, abbruch: &AtomicBool) -> Umbaufehler {
    if abbruch.load(Ordering::Acquire) {
        Umbaufehler::Abgebrochen
    } else {
        Umbaufehler::Quelle {
            bezeichnung: herkunft.to_path_buf(),
            fehler,
        }
    }
}

/// Ein Fehler der rohen Zip-Form: `InvalidData` heisst eine Gestalt, die der
/// Umbau nicht schreibt, alles andere ein Lesefehler.
fn zipformfehler(fehler: io::Error, abbruch: &AtomicBool) -> Umbaufehler {
    if fehler.kind() == io::ErrorKind::InvalidData {
        Umbaufehler::Unumschreibbar(Unumschreibbar::ZipForm)
    } else {
        lesefehler(fehler, abbruch)
    }
}

fn zipschreibfehler(fehler: ZipError, abbruch: &AtomicBool) -> Umbaufehler {
    match fehler {
        ZipError::Io(fehler) => schreibfehler(fehler, abbruch),
        andere => Umbaufehler::Schreiben(io::Error::other(andere.to_string())),
    }
}

/// Warum eine Uebertragung gescheitert ist, getrennt nach der Seite.
enum Kopierfehler {
    Lesen(io::Error),
    Schreiben(io::Error),
    /// Die Quelle war vor der genannten Menge zu Ende.
    ZuKurz,
}

/// Uebertraegt genau `menge` Bytes.
fn uebertragen(von: &mut dyn Read, nach: &mut dyn Write, menge: u64) -> Result<(), Kopierfehler> {
    let mut puffer = vec![0u8; PUFFER];
    let mut rest = menge;
    while rest > 0 {
        let stueck = puffer
            .len()
            .min(usize::try_from(rest).unwrap_or(usize::MAX));
        let gelesen = match von.read(&mut puffer[..stueck]) {
            Ok(0) => return Err(Kopierfehler::ZuKurz),
            Ok(gelesen) => gelesen,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
            Err(fehler) => return Err(Kopierfehler::Lesen(fehler)),
        };
        nach.write_all(&puffer[..gelesen])
            .map_err(Kopierfehler::Schreiben)?;
        rest -= gelesen as u64;
    }
    Ok(())
}

/// Uebertraegt Bytes des alten Archivs; zu kurz heisst kaputt.
fn alt_uebertragen(
    von: &mut dyn Read,
    nach: &mut dyn Write,
    menge: u64,
    abbruch: &AtomicBool,
) -> Result<(), Umbaufehler> {
    uebertragen(von, nach, menge).map_err(|fehler| match fehler {
        Kopierfehler::Lesen(fehler) => lesefehler(fehler, abbruch),
        Kopierfehler::Schreiben(fehler) => schreibfehler(fehler, abbruch),
        Kopierfehler::ZuKurz => lesefehler(io::Error::from(io::ErrorKind::UnexpectedEof), abbruch),
    })
}

/// Oeffnet die Quelle eines neuen Eintrags, ohne zu schreiben.
fn quelle_oeffnen<'a>(
    quelle: &'a mut Box<dyn Datenquelle>,
    pfand: &'a Fernfaden,
    herkunft: &Path,
    abbruch: &AtomicBool,
) -> Result<Geoeffnet<'a>, Umbaufehler> {
    quelle
        .oeffnen(pfand)
        .map_err(|fehler| quellfehler(herkunft, fehler, abbruch))
}

/// Schreibt die Daten einer geoeffneten Quelle, genau so viele Bytes, wie sie
/// angekuendigt hat; liefert sie weniger oder mehr, hat sie sich beim Lesen
/// geaendert.
fn quelle_schreiben(
    geoeffnet: &mut Geoeffnet<'_>,
    nach: &mut dyn Write,
    herkunft: &Path,
    abbruch: &AtomicBool,
) -> Result<u64, Umbaufehler> {
    let veraendert = || {
        io::Error::new(
            io::ErrorKind::InvalidData,
            text(Text::ArchivQuelleVeraendert),
        )
    };
    uebertragen(&mut geoeffnet.leser, nach, geoeffnet.groesse).map_err(|fehler| match fehler {
        Kopierfehler::Lesen(fehler) => quellfehler(herkunft, fehler, abbruch),
        Kopierfehler::Schreiben(fehler) => schreibfehler(fehler, abbruch),
        Kopierfehler::ZuKurz => quellfehler(herkunft, veraendert(), abbruch),
    })?;
    let mut noch = [0u8; 1];
    loop {
        match geoeffnet.leser.read(&mut noch) {
            Ok(0) => return Ok(geoeffnet.groesse),
            Ok(_) => return Err(quellfehler(herkunft, veraendert(), abbruch)),
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => {}
            Err(fehler) => return Err(quellfehler(herkunft, fehler, abbruch)),
        }
    }
}

// ---------------------------------------------------------------------------
// Zip
// ---------------------------------------------------------------------------

/// Schreibt das umgebaute Zip.
///
/// Zuerst packt die Kiste die neuen Eintraege an den Anfang der neuen Datei
/// und schliesst sie ab; ihr Zentralverzeichnis wird gelesen und
/// abgeschnitten. Danach geht jeder behaltene alte Eintrag roh dahinter,
/// zuletzt das gemeinsame Zentralverzeichnis, die alten Saetze in ihrer
/// Folge vor den neuen, und das Ende mit dem Kommentar des alten Archivs.
#[allow(clippy::too_many_arguments)]
fn zip_schreiben(
    pfand: &Fernfaden,
    alt: &mut File,
    aus: &mut Ausgabe<'_>,
    verzeichnis: &Inhaltsverzeichnis,
    ziele: &Ziele,
    neue: &mut [Neuer],
    bytes: &mut [u64],
) -> Result<(), Umbaufehler> {
    let abbruch = Arc::clone(&aus.abbruch);

    let ende = zipform::ende_lesen(alt).map_err(|fehler| zipformfehler(fehler, &abbruch))?;
    let saetze = zipform::zentralsaetze_lesen(alt, &ende)
        .map_err(|fehler| zipformfehler(fehler, &abbruch))?;
    if saetze.iter().any(Zentralsatz::verschluesselt) {
        return Err(Umbaufehler::Verschluesselt);
    }
    let satz_zu_stelle = zip_saetze_zuordnen(alt, verzeichnis, &saetze, &abbruch)?;

    // Wo die Daten eines Satzes enden: am naechsten lokalen Kopf oder am
    // Zentralverzeichnis. So kommt ein Datenbeschreiber mit, ohne dass er
    // gedeutet werden muss.
    let mut grenzen: Vec<u64> = saetze.iter().map(Zentralsatz::lokal).collect();
    grenzen.push(ende.anfang);
    grenzen.sort_unstable();
    grenzen.dedup();
    let spannenende = |anfang: u64| -> Result<u64, Umbaufehler> {
        let naechste = grenzen.partition_point(|&grenze| grenze <= anfang);
        grenzen
            .get(naechste)
            .copied()
            .ok_or(Umbaufehler::Unumschreibbar(Unumschreibbar::ZipForm))
    };

    // Die neuen Eintraege zuerst, ueber die Kiste.
    let mut neue_saetze = Vec::new();
    if !ziele.neu.is_empty() {
        {
            let mut schreiber =
                ZipWriter::new(BufWriter::with_capacity(PUFFER, &mut *aus)).set_auto_large_file();
            for (pfad, stelle) in &ziele.neu {
                let neuer = &mut neue[*stelle];
                let geschrieben = neuen_zip_eintrag(&mut schreiber, pfand, pfad, neuer, &abbruch)?;
                if let Some(schritt) = neuer.schritt {
                    bytes[schritt] += geschrieben;
                }
            }
            let puffer = schreiber
                .finish()
                .map_err(|fehler| zipschreibfehler(fehler, &abbruch))?;
            puffer
                .into_inner()
                .map_err(|fehler| schreibfehler(fehler.into_error(), &abbruch))?;
        }
        let neues_ende =
            zipform::ende_lesen(aus).map_err(|fehler| schreibfehler(fehler, &abbruch))?;
        neue_saetze = zipform::zentralsaetze_lesen(aus, &neues_ende)
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
        aus.datei
            .set_len(neues_ende.anfang)
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
        aus.seek(SeekFrom::Start(neues_ende.anfang))
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
    }

    // Die alten Eintraege, roh.
    let mut zentral: Vec<Vec<u8>> = Vec::with_capacity(saetze.len() + neue_saetze.len());
    for (nummer, satz) in saetze.iter().enumerate() {
        let wahl = match satz_zu_stelle.get(&nummer) {
            Some(&stelle) => ziele.fuer(verzeichnis, stelle),
            // Vor der Aufstellung schon abgewiesen; steht er trotzdem hier,
            // haben Verzeichnis und Strom verschieden gezaehlt (Modulkopf).
            None if verdeckt(satz.name()) => return Err(Umbaufehler::Verdeckt),
            None => Altwahl::Roh,
        };
        let name = match wahl {
            Altwahl::Weg => continue,
            Altwahl::Roh => None,
            Altwahl::Umbenannt(name) => Some(name.into_bytes()),
        };
        let anfang = satz.lokal();
        let menge = spannenende(anfang)? - anfang;
        let neue_stelle = aus
            .stream_position()
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
        alt.seek(SeekFrom::Start(anfang))
            .map_err(|fehler| lesefehler(fehler, &abbruch))?;
        let mut leser = BufReader::with_capacity(PUFFER, &mut *alt);
        match &name {
            None => alt_uebertragen(&mut leser, aus, menge, &abbruch)?,
            Some(name) => {
                let (kopf, alte_laenge) = zipform::lokalkopf_umbenannt(&mut leser, name)
                    .map_err(|fehler| zipformfehler(fehler, &abbruch))?;
                aus.write_all(&kopf)
                    .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
                let rest = menge
                    .checked_sub(alte_laenge)
                    .ok_or(Umbaufehler::Unumschreibbar(Unumschreibbar::ZipForm))?;
                alt_uebertragen(&mut leser, aus, rest, &abbruch)?;
            }
        }
        zentral.push(
            satz.verlegt(neue_stelle, name.as_deref())
                .map_err(|fehler| zipformfehler(fehler, &abbruch))?,
        );
    }
    for satz in &neue_saetze {
        zentral.push(
            satz.verlegt(satz.lokal(), None)
                .map_err(|fehler| schreibfehler(fehler, &abbruch))?,
        );
    }

    // Das Zentralverzeichnis und das Ende.
    let anfang = aus
        .stream_position()
        .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
    let groesse: u64 = zentral.iter().map(|satz| satz.len() as u64).sum();
    let mut puffer = BufWriter::with_capacity(PUFFER, &mut *aus);
    for satz in &zentral {
        puffer
            .write_all(satz)
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
    }
    zipform::ende_schreiben(
        &mut puffer,
        zentral.len() as u64,
        anfang,
        groesse,
        &ende.kommentar,
    )
    .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
    puffer
        .flush()
        .map_err(|fehler| schreibfehler(fehler, &abbruch))
}

/// Ordnet jeden zentralen Satz, den das Verzeichnis zeigt, seiner Stelle im
/// Verzeichnis zu.
///
/// Das Verzeichnis kennt einen Eintrag am Platz der Kiste; die Kiste nennt
/// dazu die Stelle seines zentralen Satzes, und dort muss derselbe lokale
/// Kopf stehen, den der Satz nennt. Tut er es nicht, rechnet die Kiste mit
/// Daten vor dem ersten Eintrag, und der Umbau schreibt das Archiv nicht.
fn zip_saetze_zuordnen(
    alt: &File,
    verzeichnis: &Inhaltsverzeichnis,
    saetze: &[Zentralsatz],
    abbruch: &AtomicBool,
) -> Result<HashMap<usize, usize>, Umbaufehler> {
    let zweitleser = alt
        .try_clone()
        .map_err(|fehler| lesefehler(fehler, abbruch))?;
    let mut kiste = ZipArchive::new(BufReader::new(zweitleser)).map_err(|fehler| {
        Umbaufehler::Lesen(match fehler {
            ZipError::Io(fehler) => Verzeichnisfehler::Lesen(fehler),
            andere => Verzeichnisfehler::Ungueltig(andere.to_string()),
        })
    })?;
    let nach_stelle: HashMap<u64, usize> = saetze
        .iter()
        .enumerate()
        .map(|(nummer, satz)| (satz.stelle(), nummer))
        .collect();
    let mut zuordnung = HashMap::new();
    for (stelle, eintrag) in verzeichnis.eintraege().iter().enumerate() {
        let Fundstelle::Zip(platz) = eintrag.fundstelle else {
            continue;
        };
        let datei = kiste.by_index_raw(platz).map_err(|fehler| {
            Umbaufehler::Lesen(Verzeichnisfehler::Ungueltig(fehler.to_string()))
        })?;
        let nummer = nach_stelle
            .get(&datei.central_header_start())
            .copied()
            .filter(|&nummer| saetze[nummer].lokal() == datei.header_start())
            .ok_or(Umbaufehler::Unumschreibbar(Unumschreibbar::ZipForm))?;
        zuordnung.insert(nummer, stelle);
    }
    Ok(zuordnung)
}

/// Packt einen neuen Eintrag ueber die Kiste, mit Rechten und den Zeitfeldern
/// ueber dieselben Helfer wie `operation::zippen`. Gibt die Bytes seiner Daten
/// zurueck.
fn neuen_zip_eintrag<W: Write + Seek>(
    schreiber: &mut ZipWriter<W>,
    pfand: &Fernfaden,
    pfad: &str,
    neuer: &mut Neuer,
    abbruch: &AtomicBool,
) -> Result<u64, Umbaufehler> {
    let wahl = zipform::zeit_eintragen(FullFileOptions::default(), neuer.geaendert, None)
        .unix_permissions(neuer.rechte);
    let fehler = |fehler| zipschreibfehler(fehler, abbruch);
    match &mut neuer.daten {
        Neudaten::Ordner => {
            schreiber
                .add_directory(archivname(pfad, Typ::Ordner), wahl)
                .map_err(fehler)?;
            Ok(0)
        }
        Neudaten::Verknuepfung(verweis) => {
            schreiber
                .add_symlink(pfad, verweis.as_str(), wahl)
                .map_err(fehler)?;
            Ok(verweis.len() as u64)
        }
        Neudaten::Datei(quelle) => {
            let mut geoeffnet = quelle_oeffnen(quelle, pfand, &neuer.herkunft, abbruch)?;
            let wahl = zipform::grosse_form_waehlen(wahl, Some(geoeffnet.groesse));
            schreiber.start_file(pfad, wahl).map_err(fehler)?;
            quelle_schreiben(&mut geoeffnet, schreiber, &neuer.herkunft, abbruch)
        }
    }
}

// ---------------------------------------------------------------------------
// Tar und tar.gz
// ---------------------------------------------------------------------------

/// Die Saetze eines PAX-Kopfs als Schluessel und Wert.
type Paxsaetze = Vec<(Vec<u8>, Vec<u8>)>;

/// Ein Vorkopf eines tar-Eintrags: langer Name, langer Verweis oder PAX.
struct Vorkopf {
    kopf: Header,
    daten: Vec<u8>,
}

/// Schreibt das umgebaute tar oder tar.gz.
#[allow(clippy::too_many_arguments)]
fn tar_schreiben(
    pfand: &Fernfaden,
    alt: File,
    gepackt: bool,
    aus: &mut Ausgabe<'_>,
    verzeichnis: &Inhaltsverzeichnis,
    ziele: &Ziele,
    neue: &mut [Neuer],
    besitzer: (u64, u64),
    bytes: &mut [u64],
) -> Result<(), Umbaufehler> {
    let abbruch = Arc::clone(&aus.abbruch);
    let roh = BufReader::with_capacity(PUFFER, alt);
    let eingang: Box<dyn Read> = if gepackt {
        Box::new(MultiGzDecoder::new(roh))
    } else {
        Box::new(roh)
    };
    let eingang = Abbruchleser::neu(eingang, &abbruch);
    let puffer = BufWriter::with_capacity(PUFFER, &mut *aus);
    let mut strom = Tarstrom {
        pfand,
        verzeichnis,
        ziele,
        besitzer,
        abbruch: &abbruch,
    };
    if gepackt {
        let mut packer = GzEncoder::new(puffer, Compression::new(6));
        strom.schreiben(eingang, &mut packer, neue, bytes)?;
        let mut puffer = packer
            .finish()
            .map_err(|fehler| schreibfehler(fehler, &abbruch))?;
        puffer
            .flush()
            .map_err(|fehler| schreibfehler(fehler, &abbruch))
    } else {
        let mut puffer = puffer;
        strom.schreiben(eingang, &mut puffer, neue, bytes)?;
        puffer
            .flush()
            .map_err(|fehler| schreibfehler(fehler, &abbruch))
    }
}

/// Was das Schreiben eines tar-Stroms von aussen braucht.
struct Tarstrom<'a> {
    pfand: &'a Fernfaden,
    verzeichnis: &'a Inhaltsverzeichnis,
    ziele: &'a Ziele,
    besitzer: (u64, u64),
    abbruch: &'a AtomicBool,
}

/// Fuellt bis zur naechsten Blockgrenze von 512 Bytes auf.
fn auffuellen(aus: &mut dyn Write, laenge: u64) -> io::Result<()> {
    let rest = (512 - laenge % 512) % 512;
    aus.write_all(&[0u8; 512][..usize::try_from(rest).unwrap_or(0)])
}

/// Zerlegt die Saetze eines PAX-Kopfs: `<laenge> <schluessel>=<wert>\n`.
fn pax_saetze(daten: &[u8]) -> Option<Paxsaetze> {
    let mut saetze = Vec::new();
    let mut rest = daten;
    while !rest.is_empty() && !rest.iter().all(|&byte| byte == 0) {
        let leer = rest.iter().position(|&byte| byte == b' ')?;
        let laenge: usize = std::str::from_utf8(&rest[..leer]).ok()?.parse().ok()?;
        if laenge <= leer + 1 || laenge > rest.len() {
            return None;
        }
        let satz = &rest[leer + 1..laenge];
        let satz = satz.strip_suffix(b"\n").unwrap_or(satz);
        let gleich = satz.iter().position(|&byte| byte == b'=')?;
        saetze.push((satz[..gleich].to_vec(), satz[gleich + 1..].to_vec()));
        rest = &rest[laenge..];
    }
    Some(saetze)
}

/// Ein PAX-Satz mit seiner Laenge, die sich selbst mitzaehlt.
fn pax_satz(schluessel: &[u8], wert: &[u8]) -> Vec<u8> {
    let rumpf = schluessel.len() + wert.len() + 3;
    let mut laenge = rumpf + 1;
    loop {
        let neu = rumpf + laenge.to_string().len();
        if neu == laenge {
            break;
        }
        laenge = neu;
    }
    let mut satz = laenge.to_string().into_bytes();
    satz.push(b' ');
    satz.extend_from_slice(schluessel);
    satz.push(b'=');
    satz.extend_from_slice(wert);
    satz.push(b'\n');
    satz
}

/// Setzt den Namen in einen Kopf, so weit er hineinpasst. Der volle Name
/// steht dann in einem PAX-Satz `path`, den jeder heutige Leser vorzieht.
fn kopfname_setzen(kopf: &mut Header, name: &str) -> bool {
    if kopf.set_path(name).is_ok() {
        return true;
    }
    if let Some(ustar) = kopf.as_ustar_mut() {
        ustar.prefix = [0; 155];
    }
    let feld = &mut kopf.as_old_mut().name;
    *feld = [0; 100];
    let laenge = name.len().min(feld.len());
    feld[..laenge].copy_from_slice(&name.as_bytes()[..laenge]);
    false
}

impl Tarstrom<'_> {
    fn lese(&self, fehler: io::Error) -> Umbaufehler {
        lesefehler(fehler, self.abbruch)
    }

    fn schreib(&self, fehler: io::Error) -> Umbaufehler {
        schreibfehler(fehler, self.abbruch)
    }

    /// Liest den alten Strom Kopf fuer Kopf und schreibt den neuen.
    fn schreiben<R: Read, W: Write>(
        &mut self,
        eingang: R,
        aus: &mut W,
        neue: &mut [Neuer],
        bytes: &mut [u64],
    ) -> Result<(), Umbaufehler> {
        let nach_versatz: HashMap<u64, usize> = self
            .verzeichnis
            .eintraege()
            .iter()
            .enumerate()
            .filter_map(|(stelle, eintrag)| match eintrag.fundstelle {
                Fundstelle::Tar { versatz } => Some((versatz, stelle)),
                Fundstelle::Zip(_)
                | Fundstelle::GzGanz
                | Fundstelle::Erschlossen
                | Fundstelle::SqliteTabelle { .. }
                | Fundstelle::SqliteSicht => None,
            })
            .collect();

        let mut archiv = tar::Archive::new(eingang);
        let eintraege = archiv
            .entries()
            .map_err(|fehler| self.lese(fehler))?
            .raw(true);
        let mut gruppe: Vec<Vorkopf> = Vec::new();
        for eintrag in eintraege {
            let mut eintrag = eintrag.map_err(|fehler| self.lese(fehler))?;
            let kopf = eintrag.header().clone();
            let art = kopf.entry_type();
            if art.is_gnu_longname() || art.is_gnu_longlink() || art.is_pax_local_extensions() {
                let groesse = eintrag.size();
                if groesse > HOECHSTENS_VORKOPF {
                    return Err(Umbaufehler::Lesen(Verzeichnisfehler::Ungueltig(
                        VORKOPF_ZU_GROSS.to_owned(),
                    )));
                }
                let mut daten = Vec::new();
                eintrag
                    .read_to_end(&mut daten)
                    .map_err(|fehler| self.lese(fehler))?;
                gruppe.push(Vorkopf { kopf, daten });
                continue;
            }
            if art.is_pax_global_extensions() {
                self.roh_schreiben(aus, &gruppe, &kopf, &mut eintrag)?;
                gruppe.clear();
                continue;
            }
            if art.is_gnu_sparse() {
                return Err(Umbaufehler::Unumschreibbar(Unumschreibbar::TarLuecken));
            }
            // Vor der Aufstellung schon abgewiesen, sofern das Verzeichnis den
            // Verweis zeigt; hier auch einer mit unsicherem Namen (Modulkopf).
            if art == EntryType::Link {
                return Err(Umbaufehler::Unumschreibbar(
                    Unumschreibbar::TarHarteVerweise,
                ));
            }

            let pax = self.pax_der_gruppe(&gruppe)?;
            if let Some((_, wert)) = pax.iter().find(|(schluessel, _)| schluessel == b"size") {
                let angegeben = std::str::from_utf8(wert)
                    .ok()
                    .and_then(|wert| wert.parse::<u64>().ok());
                if angegeben != kopf.entry_size().ok() {
                    return Err(Umbaufehler::Unumschreibbar(
                        Unumschreibbar::TarGroesseAusPax,
                    ));
                }
            }

            let wahl = match nach_versatz.get(&eintrag.raw_header_position()) {
                Some(&stelle) => self.ziele.fuer(self.verzeichnis, stelle),
                // Wie im Zip: vor der Aufstellung schon abgewiesen.
                None if verdeckt(&gruppenname(&gruppe, &pax, &kopf)) => {
                    return Err(Umbaufehler::Verdeckt);
                }
                None => Altwahl::Roh,
            };
            match wahl {
                // Die Daten ueberliest der Strom beim naechsten Kopf selbst.
                Altwahl::Weg => {}
                Altwahl::Roh => self.roh_schreiben(aus, &gruppe, &kopf, &mut eintrag)?,
                Altwahl::Umbenannt(name) => {
                    self.umbenannt_schreiben(aus, &gruppe, pax, kopf, &mut eintrag, &name)?;
                }
            }
            gruppe.clear();
        }
        if !gruppe.is_empty() {
            return Err(Umbaufehler::Lesen(Verzeichnisfehler::Ungueltig(
                "tar: extension header without entry".to_owned(),
            )));
        }

        for (pfad, stelle) in &self.ziele.neu {
            let neuer = &mut neue[*stelle];
            let geschrieben = self.neuen_eintrag_schreiben(aus, pfad, neuer)?;
            if let Some(schritt) = neuer.schritt {
                bytes[schritt] += geschrieben;
            }
        }
        aus.write_all(&[0u8; 1024])
            .map_err(|fehler| self.schreib(fehler))
    }

    fn pax_der_gruppe(&self, gruppe: &[Vorkopf]) -> Result<Paxsaetze, Umbaufehler> {
        let mut saetze = Vec::new();
        for vorkopf in gruppe {
            if vorkopf.kopf.entry_type().is_pax_local_extensions() {
                saetze.extend(pax_saetze(&vorkopf.daten).ok_or_else(|| {
                    Umbaufehler::Lesen(Verzeichnisfehler::Ungueltig(
                        "tar: broken pax header".to_owned(),
                    ))
                })?);
            }
        }
        Ok(saetze)
    }

    /// Schreibt einen Kopf mit seinen Daten und der Auffuellung.
    fn block_schreiben(
        &self,
        aus: &mut dyn Write,
        kopf: &Header,
        daten: &[u8],
    ) -> Result<(), Umbaufehler> {
        aus.write_all(kopf.as_bytes())
            .and_then(|()| aus.write_all(daten))
            .and_then(|()| auffuellen(aus, daten.len() as u64))
            .map_err(|fehler| self.schreib(fehler))
    }

    /// Schreibt einen Eintrag samt Vorkoepfen Byte fuer Byte.
    fn roh_schreiben<W: Write, R: Read>(
        &self,
        aus: &mut W,
        gruppe: &[Vorkopf],
        kopf: &Header,
        daten: &mut R,
    ) -> Result<(), Umbaufehler> {
        for vorkopf in gruppe {
            self.block_schreiben(aus, &vorkopf.kopf, &vorkopf.daten)?;
        }
        let groesse = kopf.entry_size().map_err(|fehler| self.lese(fehler))?;
        aus.write_all(kopf.as_bytes())
            .map_err(|fehler| self.schreib(fehler))?;
        alt_uebertragen(daten, aus, groesse, self.abbruch)?;
        auffuellen(aus, groesse).map_err(|fehler| self.schreib(fehler))
    }

    /// Schreibt einen alten Eintrag unter neuem Namen: der lange Name faellt
    /// weg, ein langer Verweis bleibt, und der PAX-Kopf traegt jede alte
    /// Angabe ausser dem Namen und den neuen Namen.
    fn umbenannt_schreiben<W: Write, R: Read>(
        &self,
        aus: &mut W,
        gruppe: &[Vorkopf],
        pax: Paxsaetze,
        mut kopf: Header,
        daten: &mut R,
        name: &str,
    ) -> Result<(), Umbaufehler> {
        for vorkopf in gruppe {
            if vorkopf.kopf.entry_type().is_gnu_longlink() {
                self.block_schreiben(aus, &vorkopf.kopf, &vorkopf.daten)?;
            }
        }
        let mut saetze: Paxsaetze = pax
            .into_iter()
            .filter(|(schluessel, _)| schluessel != b"path")
            .collect();
        saetze.push((b"path".to_vec(), name.as_bytes().to_vec()));
        self.pax_schreiben(aus, &saetze)?;
        kopfname_setzen(&mut kopf, name);
        kopf.set_cksum();
        let groesse = kopf.entry_size().map_err(|fehler| self.lese(fehler))?;
        aus.write_all(kopf.as_bytes())
            .map_err(|fehler| self.schreib(fehler))?;
        alt_uebertragen(daten, aus, groesse, self.abbruch)?;
        auffuellen(aus, groesse).map_err(|fehler| self.schreib(fehler))
    }

    /// Schreibt einen PAX-Kopf mit den genannten Saetzen, sofern es welche
    /// gibt.
    fn pax_schreiben(
        &self,
        aus: &mut dyn Write,
        saetze: &[(Vec<u8>, Vec<u8>)],
    ) -> Result<(), Umbaufehler> {
        if saetze.is_empty() {
            return Ok(());
        }
        let daten: Vec<u8> = saetze
            .iter()
            .flat_map(|(schluessel, wert)| pax_satz(schluessel, wert))
            .collect();
        let mut kopf = Header::new_ustar();
        kopf.set_entry_type(EntryType::XHeader);
        kopfname_setzen(&mut kopf, "PaxHeader");
        kopf.set_mode(0o644);
        kopf.set_mtime(0);
        kopf.set_size(daten.len() as u64);
        kopf.set_cksum();
        self.block_schreiben(aus, &kopf, &daten)
    }

    /// Schreibt einen neuen Eintrag und gibt die Bytes seiner Daten zurueck.
    fn neuen_eintrag_schreiben<W: Write>(
        &self,
        aus: &mut W,
        pfad: &str,
        neuer: &mut Neuer,
    ) -> Result<u64, Umbaufehler> {
        let mut kopf = Header::new_ustar();
        kopf.set_mode(neuer.rechte & 0o7777);
        kopf.set_mtime(
            neuer
                .geaendert
                .duration_since(UNIX_EPOCH)
                .map_or(0, |seither| seither.as_secs()),
        );
        kopf.set_uid(self.besitzer.0);
        kopf.set_gid(self.besitzer.1);
        let mut saetze: Paxsaetze = Vec::new();
        let herkunft = neuer.herkunft.clone();
        let (typ, mut geoeffnet) = match &mut neuer.daten {
            Neudaten::Ordner => {
                kopf.set_entry_type(EntryType::Directory);
                (Typ::Ordner, None)
            }
            Neudaten::Verknuepfung(verweis) => {
                kopf.set_entry_type(EntryType::Symlink);
                if kopf.set_link_name(verweis.as_str()).is_err() {
                    saetze.push((b"linkpath".to_vec(), verweis.as_bytes().to_vec()));
                }
                (Typ::Verknuepfung, None)
            }
            Neudaten::Datei(quelle) => {
                kopf.set_entry_type(EntryType::Regular);
                let geoeffnet = quelle_oeffnen(quelle, self.pfand, &herkunft, self.abbruch)?;
                (Typ::Datei, Some(geoeffnet))
            }
        };
        let groesse = geoeffnet.as_ref().map_or(0, |geoeffnet| geoeffnet.groesse);
        kopf.set_size(groesse);
        let name = archivname(pfad, typ);
        if !kopfname_setzen(&mut kopf, &name) {
            saetze.push((b"path".to_vec(), name.into_bytes()));
        }
        self.pax_schreiben(aus, &saetze)?;
        kopf.set_cksum();
        aus.write_all(kopf.as_bytes())
            .map_err(|fehler| self.schreib(fehler))?;
        let Some(geoeffnet) = geoeffnet.as_mut() else {
            return Ok(0);
        };
        let geschrieben = quelle_schreiben(geoeffnet, aus, &herkunft, self.abbruch)?;
        auffuellen(aus, geschrieben).map_err(|fehler| self.schreib(fehler))?;
        Ok(geschrieben)
    }
}

/// Der Name eines tar-Eintrags, wie ein Leser ihn nimmt: der lange Name vor
/// dem PAX-Satz `path` vor dem Kopf, in der Folge der Kiste `tar`.
fn gruppenname(gruppe: &[Vorkopf], pax: &[(Vec<u8>, Vec<u8>)], kopf: &Header) -> Vec<u8> {
    if let Some(lang) = gruppe
        .iter()
        .find(|vorkopf| vorkopf.kopf.entry_type().is_gnu_longname())
    {
        let mut name = lang.daten.clone();
        while name.last() == Some(&0) {
            name.pop();
        }
        return name;
    }
    if let Some((_, wert)) = pax.iter().find(|(schluessel, _)| schluessel == b"path") {
        return wert.clone();
    }
    kopf.path_bytes().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Die Proben hier brauchen eine `Steuerung` ohne Kanal und den privaten
    /// Weg mit Schreibgrenze; alle uebrigen Proben stehen in
    /// `tests/archiv_umbau.rs`.
    fn steuerung() -> Steuerung {
        Steuerung::neu(
            Arc::new(AtomicBool::new(false)),
            None,
            Konfliktregel::Fragen,
        )
    }

    static ZAEHLER: Mutex<u32> = Mutex::new(0);

    fn ordner(zweck: &str) -> PathBuf {
        let nummer = {
            let mut zaehler = ZAEHLER
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *zaehler += 1;
            *zaehler
        };
        let pfad =
            std::env::temp_dir().join(format!("krk-umbau-{zweck}-{}-{nummer}", std::process::id()));
        fs::create_dir_all(&pfad).expect("Pruefordner anlegen");
        pfad
    }

    /// Bytes, die sich nicht packen lassen, damit die Grenzen der Probe in
    /// jedes der drei Formate fallen.
    fn zufall(laenge: usize) -> Vec<u8> {
        let mut stand: u32 = 0x2545_f491;
        (0..laenge)
            .map(|_| {
                stand ^= stand << 13;
                stand ^= stand >> 17;
                stand ^= stand << 5;
                stand.to_le_bytes()[0]
            })
            .collect()
    }

    fn inhalte() -> [(&'static str, Vec<u8>); 2] {
        [("a.txt", zufall(4000)), ("b/c.txt", zufall(70_000))]
    }

    fn zip_bauen(pfad: &Path) {
        let mut schreiber = ZipWriter::new(File::create(pfad).expect("Zip anlegen"));
        for (name, inhalt) in inhalte() {
            schreiber
                .start_file(name, FullFileOptions::default())
                .expect("Eintrag beginnen");
            schreiber.write_all(&inhalt).expect("Eintrag schreiben");
        }
        schreiber.finish().expect("Zip abschliessen");
    }

    fn tar_bauen(pfad: &Path, gepackt: bool) {
        let mut daten = Vec::new();
        {
            let mut bauer = tar::Builder::new(&mut daten);
            for (name, inhalt) in inhalte() {
                let mut kopf = Header::new_gnu();
                kopf.set_size(inhalt.len() as u64);
                kopf.set_mode(0o644);
                kopf.set_cksum();
                bauer
                    .append_data(&mut kopf, name, inhalt.as_slice())
                    .expect("Eintrag packen");
            }
            bauer.finish().expect("tar abschliessen");
        }
        if gepackt {
            let mut packer = GzEncoder::new(Vec::new(), Compression::default());
            packer.write_all(&daten).expect("packen");
            daten = packer.finish().expect("gzip abschliessen");
        }
        fs::write(pfad, daten).expect("Archiv schreiben");
    }

    /// Ein voller Datentraeger, nachgestellt ueber eine Schreibgrenze, laesst
    /// jedes der drei Formate Byte fuer Byte stehen und keinen Zwischennamen
    /// zurueck, an jeder Grenze zwischen dem ersten Byte und fast dem Ende.
    #[test]
    fn ein_voller_datentraeger_laesst_jedes_format_byte_fuer_byte_stehen() {
        let wurzel = ordner("voll");
        type Bauer = fn(&Path);
        let faelle: [(&str, Bauer); 3] = [
            ("a.zip", zip_bauen),
            ("a.tar", |pfad| tar_bauen(pfad, false)),
            ("a.tgz", |pfad| tar_bauen(pfad, true)),
        ];
        for (name, bauen) in faelle {
            let archiv = wurzel.join(name);
            bauen(&archiv);
            let vorher = fs::read(&archiv).expect("vorher");
            for grenze in [0, 1, 511, 4096, 60_000] {
                let zugang = Arc::new(Archivzugang::neu(
                    archiv.clone(),
                    Archivformat::aus_namen(archiv.file_name().expect("Name")).expect("Format"),
                ));
                let fuer_faden = Arc::clone(&zugang);
                let ausgang = faden("krk-umbauprobe".into(), move |pfand| {
                    let plan = Umbauplan::neu()
                        .mit(Umbauschritt::Umhaengen {
                            von: innenpfad("a.txt"),
                            nach: innenpfad("b/a.txt"),
                        })
                        .mit(Umbauschritt::Hinzufuegen {
                            ordner: Innenpfad::wurzel(),
                            eintrag: Neueintrag {
                                name: "neu.txt".to_owned(),
                                herkunft: PathBuf::from("/probe/neu.txt"),
                                rechte: 0o644,
                                geaendert: UNIX_EPOCH,
                                art: Neuart::Datei(Box::new(Leer)),
                            },
                        });
                    let ausgang =
                        umbauen_mit(pfand, &fuer_faden, None, plan, &mut steuerung(), Some(grenze));
                    matches!(ausgang, Err(Umbaufehler::Schreiben(fehler)) if fehler.kind() == io::ErrorKind::StorageFull)
                })
                .expect("Faden")
                .join()
                .expect("keine Panik");
                assert!(
                    ausgang,
                    "{name} mit Grenze {grenze} scheitert nicht am vollen Datentraeger"
                );
                assert_eq!(
                    fs::read(&archiv).expect("nachher"),
                    vorher,
                    "{name} mit Grenze {grenze}"
                );
                let liegen: Vec<_> = fs::read_dir(&wurzel)
                    .expect("Ordner")
                    .map(|eintrag| eintrag.expect("Eintrag").file_name())
                    .filter(|name| name.to_string_lossy().starts_with(".krk-umbau-"))
                    .collect();
                assert!(liegen.is_empty(), "Zwischennamen liegen: {liegen:?}");
            }
        }
        fs::remove_dir_all(&wurzel).expect("aufraeumen");
    }

    // Die Proben von `ersetzen` an der Platte; warum sie hier stehen, sagt
    // das Pruefmodul jener Datei.

    fn zwischennamen_in(ordner: &Path) -> Vec<String> {
        fs::read_dir(ordner)
            .expect("Ordner lesbar")
            .map(|eintrag| {
                eintrag
                    .expect("Eintrag")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| name.starts_with(super::super::ersetzen::ZWISCHENVORSATZ))
            .collect()
    }

    /// Ein Schreiber, der nach einigen Bytes scheitert, laesst das Original
    /// stehen und keinen Zwischennamen zurueck.
    #[test]
    fn ein_fehler_mitten_im_schreiben_laesst_das_original_und_keinen_zwischennamen() {
        let wurzel = ordner("ersetzen-fehler");
        let archiv = wurzel.join("a.zip");
        fs::write(&archiv, b"alt").expect("Original");
        let kennung = Dateikennung::aus_angaben(&fs::metadata(&archiv).expect("Angaben"));

        let ausgang = ersetzen(&archiv, kennung, 0o640, &AtomicBool::new(false), |datei| {
            datei.write_all(b"halb").map_err(Umbaufehler::Schreiben)?;
            Err(Umbaufehler::Schreiben(io::Error::other("voll")))
        });

        assert!(matches!(ausgang, Err(Umbaufehler::Schreiben(_))));
        assert_eq!(fs::read(&archiv).expect("Original"), b"alt");
        assert!(zwischennamen_in(&wurzel).is_empty());
        fs::remove_dir_all(&wurzel).expect("aufraeumen");
    }

    /// Eine abweichende Kennung ersetzt nichts.
    #[test]
    fn eine_abweichende_kennung_ersetzt_nicht() {
        let wurzel = ordner("ersetzen-kennung");
        let archiv = wurzel.join("a.zip");
        fs::write(&archiv, b"alt").expect("Original");
        let kennung = Dateikennung::aus_angaben(&fs::metadata(&archiv).expect("Angaben"));
        fs::write(&archiv, b"von aussen").expect("Aenderung von aussen");

        let ausgang = ersetzen(&archiv, kennung, 0o644, &AtomicBool::new(false), |datei| {
            datei.write_all(b"neu").map_err(Umbaufehler::Schreiben)
        });

        assert!(matches!(ausgang, Err(Umbaufehler::VonAussenVeraendert)));
        assert_eq!(fs::read(&archiv).expect("Original"), b"von aussen");
        assert!(zwischennamen_in(&wurzel).is_empty());
        fs::remove_dir_all(&wurzel).expect("aufraeumen");
    }

    /// Ein Abbruch, der nach dem letzten geschriebenen Byte und vor dem
    /// `rename` ankommt, laesst das Original stehen und keinen Zwischennamen
    /// zurueck: der letzte Augenblick, an dem er noch wirkt (Modulkopf von
    /// `ersetzen`).
    #[test]
    fn ein_abbruch_nach_dem_letzten_byte_laesst_das_original_stehen() {
        let wurzel = ordner("ersetzen-spaeter-abbruch");
        let archiv = wurzel.join("a.zip");
        fs::write(&archiv, b"alt").expect("Original");
        let kennung = Dateikennung::aus_angaben(&fs::metadata(&archiv).expect("Angaben"));
        let abbruch = AtomicBool::new(false);

        let ausgang = ersetzen(&archiv, kennung, 0o644, &abbruch, |datei| {
            datei.write_all(b"neu").map_err(Umbaufehler::Schreiben)?;
            abbruch.store(true, Ordering::Relaxed);
            Ok(())
        });

        assert!(matches!(ausgang, Err(Umbaufehler::Abgebrochen)));
        assert_eq!(fs::read(&archiv).expect("Original"), b"alt");
        assert!(zwischennamen_in(&wurzel).is_empty());
        fs::remove_dir_all(&wurzel).expect("aufraeumen");
    }

    /// Nach einem ersetzten Archiv endet der Vorgang als fertig, auch wenn der
    /// Abbruch inzwischen steht, und der Bericht nennt ihn am Archiv; ohne
    /// ersetztes Archiv bleibt ein Abbruch ein Abbruch.
    #[test]
    fn nach_dem_ersetzen_ist_ein_abbruch_zu_spaet_und_steht_im_bericht() {
        let archiv = Path::new("/gibt/es/nicht/a.zip");
        let mut ohne = steuerung();
        assert_eq!(abschluss_nach_ersetzen(&mut ohne, &[]), Abschluss::Fertig);
        assert_eq!(
            abschluss_nach_ersetzen(&mut ohne, &[archiv.to_path_buf()]),
            Abschluss::Fertig
        );
        assert!(ohne.uebersprungen_seit(0).is_empty());

        let mut spaet = steuerung();
        spaet.abbruchkennzeichen().store(true, Ordering::Relaxed);
        assert_eq!(
            abschluss_nach_ersetzen(&mut spaet, &[]),
            Abschluss::Abgebrochen
        );
        assert_eq!(
            abschluss_nach_ersetzen(&mut spaet, &[archiv.to_path_buf()]),
            Abschluss::Fertig
        );
        let zeilen = spaet.uebersprungen_seit(0);
        assert_eq!(zeilen.len(), 1);
        assert_eq!(zeilen[0].pfad, archiv);
        assert_eq!(zeilen[0].grund, text(Text::ArchivAbbruchNachDemErsetzen));
    }

    /// Das Gelingen ersetzt den Inhalt und uebernimmt die neun Rechtebits.
    #[test]
    fn das_gelingen_ersetzt_und_uebernimmt_die_rechte() {
        let wurzel = ordner("ersetzen-gelingen");
        let archiv = wurzel.join("a.zip");
        fs::write(&archiv, b"alt").expect("Original");
        let kennung = Dateikennung::aus_angaben(&fs::metadata(&archiv).expect("Angaben"));

        ersetzen(&archiv, kennung, 0o4640, &AtomicBool::new(false), |datei| {
            datei.write_all(b"neu").map_err(Umbaufehler::Schreiben)
        })
        .expect("ersetzt");

        assert_eq!(fs::read(&archiv).expect("neu"), b"neu");
        let modus = fs::metadata(&archiv).expect("Angaben").permissions().mode();
        assert_eq!(modus & 0o7777, 0o640);
        assert!(zwischennamen_in(&wurzel).is_empty());
        fs::remove_dir_all(&wurzel).expect("aufraeumen");
    }

    #[test]
    fn ein_pax_satz_zaehlt_seine_laenge_mit() {
        for wert in [&b"x"[..], &[b'y'; 5][..], &[b'z'; 94][..], &[b'w'; 995][..]] {
            let satz = pax_satz(b"path", wert);
            let gelesen = pax_saetze(&satz).expect("lesbar");
            assert_eq!(gelesen, vec![(b"path".to_vec(), wert.to_vec())]);
        }
    }
}
