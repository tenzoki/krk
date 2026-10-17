//! Einen Eintrag aus einem Archiv lesen, ohne das Archiv zu entpacken: die
//! Lesequelle des Herauskopierens (E11 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).
//!
//! ```text
//!  Archivleser::oeffnen(&Fernfaden, zugang)   Inhaltsverzeichnis zur Kennung
//!    │ lesen(innen) / verweis_lesen(innen)
//!    v
//!  Zip:      ZipArchive einmal je Leser, by_index je Eintrag (wahlfrei)
//!  Tar, Tgz: Lesezeiger, der nur vorwaerts laeuft; ein Eintrag hinter dem
//!            Zeiger beginnt den Strom neu (gezaehlt in `neustarts`)
//!  Gz:       der ganze Strom
//! ```
//!
//! **Ein Durchlauf fuer einen Baum aus einem tar.gz.** Ein gepackter tar-Strom
//! laesst sich nur von vorn lesen; laese jeder Eintrag ihn neu, kostete ein
//! Ordner mit tausend Dateien tausend Durchlaeufe. Der [`Archivleser`] haelt
//! deshalb den Strom offen und spult zum Kopf des verlangten Eintrags vor.
//! Liegt der Eintrag hinter dem Zeiger, beginnt der Strom von vorn, und
//! [`Archivleser::neustarts`] zaehlt das. Wer Eintraege in der Folge ihrer
//! Fundstelle verlangt ([`Archivleser::lesefolge`]), kommt ohne Neustart aus;
//! die Maschine fuer ferne Vorgaenge tut das (`operation::fern`).
//!
//! **Gelesen wird allein, was zur Kennung des Inhaltsverzeichnisses passt.**
//! Jedes Oeffnen der Datei fragt Geraet, Inode, Groesse und Aenderungszeit am
//! Deskriptor und vergleicht sie mit der Kennung, aus der das Verzeichnis
//! gelesen ist; weichen sie ab, stimmen Fundstellen und Groessen nicht mehr,
//! und das Lesen endet mit [`Lesefehler::VonAussenVeraendert`]. Was ein
//! Schreiber an derselben Datei aendert, solange sie offen ist, sieht die
//! Frage nicht; ein Zip faengt das an der Pruefsumme, ein tar nicht.
//!
//! **Was nicht herauskommt**, sagt die [`Lesbarkeit`] des Eintrags, und zwar
//! vor jedem Zugriff: ein verschluesselter Eintrag, ein Verfahren, das
//! `deflate-flate2` nicht entpackt, ein harter Verweis und eine Sonderdatei im
//! tar ([`lesbarkeit_grund`]). Ein Ordner hat keine Daten.
//!
//! **Jeder Zugriff verlangt das Pfand `Fernfaden`** wie jeder andere Zugriff
//! auf eine Archivdatei (E9 des Plans): der Hauptfaden erreicht keinen davon.
//!
//! **Dieses Modul schreibt an keiner Datei**; der einzige Schreiber an einer
//! Archivdatei ist [`super::ersetzen`], und die Probe
//! `archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei` haelt das
//! auch fuer diese Datei.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::ops::ControlFlow;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use flate2::read::MultiGzDecoder;
use zip::ZipArchive;

use crate::fern::Fernfaden;
use crate::sprache::{Text, satz, text, zahl};
use crate::verzeichnis::Typ;
use crate::verzeichnis::sys::ohne_warten_oeffnen;

use super::HOECHSTENS_EINTRAEGE;
use super::innenpfad::Innenpfad;
use super::verzeichnis::{
    Archiveintrag, Fundstelle, Inhaltsverzeichnis, Lesbarkeit, Verzeichnisfehler,
};
use super::zipform::zipfehler;
use super::zugang::{Archivzugang, Dateikennung};

/// Die Groesse des Puffers, in dem Daten aus dem Archiv gereicht werden.
const PUFFER: usize = 256 * 1024;

/// Wie viele Bytes zwischen zwei Abbruchfragen beim Vorspulen liegen.
const VORSPULTAKT: u64 = 1 << 20;

/// Wie lang der Verweistext einer Verknuepfung im Zip hoechstens sein darf.
/// Ein laengerer ist kein Pfad, sondern ein falsch gekennzeichneter Eintrag.
const HOECHSTENS_VERWEIS: u64 = 64 * 1024;

/// Warum ein Eintrag nicht gelesen wurde.
#[derive(Debug)]
pub enum Lesefehler {
    /// Der Nutzer hat abgebrochen.
    Abgebrochen,
    /// Das Inhaltsverzeichnis liess sich nicht lesen.
    Verzeichnis(Verzeichnisfehler),
    /// Der Pfad benennt keinen Eintrag des Verzeichnisses, oder der Eintrag
    /// steht nicht an der Fundstelle, die das Verzeichnis nennt.
    Fehlt,
    /// Der Eintrag ist ein Ordner und hat keine Daten.
    KeineDaten,
    /// Der Eintrag kommt nicht heraus; der Text ist der Grund
    /// ([`lesbarkeit_grund`]).
    Unlesbar(String),
    /// Die Datei ist nicht mehr die, aus der das Verzeichnis gelesen ist.
    VonAussenVeraendert,
    /// Ein Systemaufruf an der Archivdatei ist gescheitert.
    Lesen(io::Error),
    /// Die Daten sind kein gueltiges Archiv mehr, etwa eine falsche
    /// Pruefsumme; der Text ist der Wortlaut der Kiste.
    Ungueltig(String),
}

impl Lesefehler {
    /// Der Grund im Klartext, wie die Abschlussliste ihn zeigt.
    #[must_use]
    pub fn grund(&self) -> String {
        match self {
            // Ein Abbruch beendet den Vorgang und kommt nicht in die
            // Abschlussliste; der Zweig steht, weil die Unterscheidung
            // vollstaendig ist.
            Lesefehler::Abgebrochen => text(Text::ArchivUnveraendertNachAbbruch).to_owned(),
            Lesefehler::Verzeichnis(fehler) => verzeichnisgrund(fehler),
            Lesefehler::Fehlt => text(Text::ArchivEintragFehlt).to_owned(),
            Lesefehler::KeineDaten => text(Text::VorgangKeineGewoehnlicheDatei).to_owned(),
            Lesefehler::Unlesbar(grund) | Lesefehler::Ungueltig(grund) => grund.clone(),
            Lesefehler::VonAussenVeraendert => text(Text::ArchivBeimLesenVeraendert).to_owned(),
            Lesefehler::Lesen(fehler) => crate::operation::grund(fehler),
        }
    }

    /// Ob die Archivdatei unter ihrem Namen fehlt.
    #[must_use]
    pub fn ist_nicht_da(&self) -> bool {
        match self {
            Lesefehler::Lesen(fehler)
            | Lesefehler::Verzeichnis(Verzeichnisfehler::Lesen(fehler)) => {
                fehler.kind() == io::ErrorKind::NotFound
            }
            Lesefehler::Abgebrochen
            | Lesefehler::Verzeichnis(_)
            | Lesefehler::Fehlt
            | Lesefehler::KeineDaten
            | Lesefehler::Unlesbar(_)
            | Lesefehler::VonAussenVeraendert
            | Lesefehler::Ungueltig(_) => false,
        }
    }
}

/// Der Grund zu einem Verzeichnisfehler beim Herauskopieren.
fn verzeichnisgrund(fehler: &Verzeichnisfehler) -> String {
    match fehler {
        Verzeichnisfehler::Abgebrochen => text(Text::ArchivUnveraendertNachAbbruch).to_owned(),
        Verzeichnisfehler::Lesen(fehler) => crate::operation::grund(fehler),
        Verzeichnisfehler::KeineDatei => text(Text::VorgangKeineGewoehnlicheDatei).to_owned(),
        Verzeichnisfehler::Ungueltig(wortlaut) => wortlaut.clone(),
        Verzeichnisfehler::ZuGross => satz(
            Text::ArchivUmbauZuGross,
            &[("grenze", &zahl(HOECHSTENS_EINTRAEGE))],
        ),
    }
}

/// Warum ein Eintrag dieser Lesbarkeit nicht herauskommt, oder `None`, wenn
/// er herauskommt. Vollstaendig ueber [`Lesbarkeit`] und ohne Auffangzweig.
#[must_use]
pub fn lesbarkeit_grund(lesbarkeit: &Lesbarkeit) -> Option<String> {
    match lesbarkeit {
        Lesbarkeit::Lesbar => None,
        Lesbarkeit::Verschluesselt => Some(text(Text::ArchivEintragVerschluesselt).to_owned()),
        Lesbarkeit::Verfahren(verfahren) => Some(satz(
            Text::ArchivVerfahrenUnbekannt,
            &[("verfahren", verfahren)],
        )),
        Lesbarkeit::HarterVerweis => Some(text(Text::ArchivHarterVerweis).to_owned()),
        Lesbarkeit::Sonderdatei => Some(text(Text::VorgangKeineGewoehnlicheDatei).to_owned()),
    }
}

/// Liest Eintraege aus einer Archivdatei, solange ein Vorgang laeuft.
///
/// Ein Leser je Archiv und Vorgang: er haelt das Inhaltsverzeichnis, aus dem
/// die Maschine Angaben und Kinder nimmt, und die offene Datei, aus der sie
/// die Daten liest.
pub struct Archivleser {
    zugang: Arc<Archivzugang>,
    verzeichnis: Arc<Inhaltsverzeichnis>,
    abbruch: Arc<AtomicBool>,
    offen: Option<Offen>,
    geoeffnet: bool,
    neustarts: u64,
}

/// Was der Leser zwischen zwei Eintraegen offen haelt.
enum Offen {
    /// Ein Zip mit seinem Zentralverzeichnis.
    Zip(ZipArchive<BufReader<File>>),
    /// Ein tar-Strom mit dem Zeiger auf die Stelle darin.
    Tar(Zeiger),
}

/// Ein tar-Strom, gepackt oder nicht, und wie viele Bytes davon gelesen sind.
struct Zeiger {
    strom: Strom,
    stelle: u64,
}

enum Strom {
    Roh(BufReader<File>),
    Gepackt(Box<MultiGzDecoder<BufReader<File>>>),
}

impl Read for Strom {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        match self {
            Strom::Roh(datei) => datei.read(puffer),
            Strom::Gepackt(strom) => strom.read(puffer),
        }
    }
}

impl Zeiger {
    /// Ueberliest `bytes` Bytes: ungepackt mit einem Sprung, gepackt durch
    /// Entpacken und Verwerfen, mit einer Abbruchfrage je MiB.
    fn vorspulen(&mut self, bytes: u64, abbruch: &AtomicBool) -> io::Result<()> {
        match &mut self.strom {
            Strom::Roh(datei) => {
                let sprung = i64::try_from(bytes)
                    .map_err(|_| io::Error::other("tar: seek position out of bounds"))?;
                datei.seek_relative(sprung)?;
                self.stelle += bytes;
                Ok(())
            }
            Strom::Gepackt(strom) => {
                let mut rest = bytes;
                let mut puffer = vec![0u8; 64 * 1024];
                let mut seit_frage = 0u64;
                while rest > 0 {
                    let stueck = usize::try_from(rest.min(puffer.len() as u64)).unwrap_or(0);
                    let gelesen = match strom.read(&mut puffer[..stueck]) {
                        Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
                        Ok(gelesen) => gelesen,
                        Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
                        Err(fehler) => return Err(fehler),
                    };
                    rest -= gelesen as u64;
                    self.stelle += gelesen as u64;
                    seit_frage += gelesen as u64;
                    if seit_frage >= VORSPULTAKT {
                        seit_frage = 0;
                        if abbruch.load(Ordering::Acquire) {
                            return Err(io::Error::other("abgebrochen"));
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

/// Der Strom ab der Stelle, an der ein `tar::Archive` auf ihm beginnt.
///
/// Die Kiste rechnet ihre Stellen ab dem Anfang ihres Archivs und setzt sie
/// nach einem Sprung auf das, was `seek` antwortet; geantwortet wird deshalb
/// die Stelle relativ zu `basis` und nicht die im ganzen Strom. Gesprungen
/// wird allein vorwaerts.
struct Ausschnitt<'a> {
    zeiger: &'a mut Zeiger,
    basis: u64,
    abbruch: &'a AtomicBool,
}

impl Read for Ausschnitt<'_> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let gelesen = self.zeiger.strom.read(puffer)?;
        self.zeiger.stelle += gelesen as u64;
        Ok(gelesen)
    }
}

impl Seek for Ausschnitt<'_> {
    fn seek(&mut self, ziel: SeekFrom) -> io::Result<u64> {
        match ziel {
            SeekFrom::Current(sprung) if sprung >= 0 => {
                self.zeiger.vorspulen(sprung.unsigned_abs(), self.abbruch)?;
                Ok(self.zeiger.stelle - self.basis)
            }
            SeekFrom::Current(_) | SeekFrom::Start(_) | SeekFrom::End(_) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "tar: the archive reader only seeks forward",
            )),
        }
    }
}

impl Archivleser {
    /// Ein Leser fuer das Archiv hinter `zugang`, mit dessen
    /// Inhaltsverzeichnis zur jetzigen Kennung der Datei.
    ///
    /// `abbruch` ist das Kennzeichen des Vorgangs; gefragt wird es beim Lesen
    /// des Verzeichnisses und beim Vorspulen im Strom.
    ///
    /// # Errors
    ///
    /// [`Lesefehler::Abgebrochen`] oder der Fehler des Verzeichnisses.
    pub fn oeffnen(
        pfand: &Fernfaden,
        zugang: Arc<Archivzugang>,
        abbruch: Arc<AtomicBool>,
    ) -> Result<Self, Lesefehler> {
        let verzeichnis = zugang
            .verzeichnis(pfand, &abbruch)
            .map_err(|fehler| match fehler {
                Verzeichnisfehler::Abgebrochen => Lesefehler::Abgebrochen,
                andere => Lesefehler::Verzeichnis(andere),
            })?;
        Ok(Archivleser {
            zugang,
            verzeichnis,
            abbruch,
            offen: None,
            geoeffnet: false,
            neustarts: 0,
        })
    }

    /// Der lokale Pfad der Archivdatei.
    #[must_use]
    pub fn datei(&self) -> &Path {
        self.zugang.datei()
    }

    /// Das Inhaltsverzeichnis, aus dem dieser Leser liest.
    #[must_use]
    pub fn verzeichnis(&self) -> &Inhaltsverzeichnis {
        &self.verzeichnis
    }

    /// Dasselbe Inhaltsverzeichnis, geteilt: fuer einen Rufer, der es haelt,
    /// waehrend er den Leser veraendert, etwa beim Lesen einer Verknuepfung.
    #[must_use]
    pub fn geteiltes_verzeichnis(&self) -> Arc<Inhaltsverzeichnis> {
        Arc::clone(&self.verzeichnis)
    }

    /// Der Zugang zur Archivdatei, aus der dieser Leser liest.
    #[must_use]
    pub fn zugang(&self) -> &Arc<Archivzugang> {
        &self.zugang
    }

    /// Ob das Archiv ein tar-Strom ist, gepackt oder nicht, der sich allein
    /// vorwaerts lesen laesst; dann lohnt die [`Archivleser::lesefolge`].
    #[must_use]
    pub fn liest_strom(&self) -> bool {
        match self.zugang.format() {
            super::Archivformat::Tar | super::Archivformat::TarGz => true,
            super::Archivformat::Zip | super::Archivformat::Gz => false,
        }
    }

    /// Wie oft ein tar-Strom von vorn beginnen musste, weil ein Eintrag hinter
    /// dem Zeiger verlangt wurde. Das erste Oeffnen zaehlt nicht.
    #[must_use]
    pub fn neustarts(&self) -> u64 {
        self.neustarts
    }

    /// Die Stelle, an der ein Eintrag samt Unterbaum im Strom beginnt, als
    /// Schluessel der Lesefolge: die kleinste Fundstelle unter ihm.
    ///
    /// Ein Ordner zaehlt mit seinem Inhalt, denn manches tar fuehrt den Kopf
    /// eines Ordners erst hinter seinem Inhalt, und ein erschlossener Ordner
    /// hat gar keinen. Wer Geschwister nach diesem Schluessel ordnet und jeden
    /// Ordner ganz abarbeitet, bevor das naechste Geschwister kommt, liest ein
    /// tar, das Ordner vor oder hinter ihrem Inhalt fuehrt, ohne Neustart.
    #[must_use]
    pub fn lesefolge(&self, innen: &Innenpfad) -> u64 {
        let eigene = self.verzeichnis.eintrag(innen).map_or(u64::MAX, |eintrag| {
            fundstellenschluessel(eintrag.fundstelle)
        });
        let darunter = self.verzeichnis.kinder(innen).map_or(u64::MAX, |kinder| {
            kinder
                .map(|kind| self.lesefolge(&kind.innen))
                .min()
                .unwrap_or(u64::MAX)
        });
        eigene.min(darunter)
    }

    /// Liest die Daten des Eintrags `innen` von Anfang bis Ende in Bloecken
    /// und reicht jeden an `empfang`; `Break` dort bricht ab, und das Ergebnis
    /// ist [`Lesefehler::Abgebrochen`]. Gibt die Zahl der Bytes zurueck.
    ///
    /// # Errors
    ///
    /// [`Lesefehler`]: der Eintrag fehlt, ist ein Ordner, kommt nicht heraus,
    /// die Datei hat sich geaendert oder liess sich nicht lesen.
    pub fn lesen(
        &mut self,
        pfand: &Fernfaden,
        innen: &Innenpfad,
        mut empfang: impl FnMut(&[u8]) -> ControlFlow<()>,
    ) -> Result<u64, Lesefehler> {
        let eintrag = self.datentraeger(innen)?;
        match eintrag.fundstelle {
            Fundstelle::Zip(stelle) => {
                let archiv = self.zip(pfand)?;
                let mut daten = archiv.by_index(stelle).map_err(zip_lesefehler)?;
                durchreichen(&mut daten, &mut empfang)
            }
            Fundstelle::Tar { versatz } => {
                self.tar_eintrag(pfand, versatz, |daten| durchreichen(daten, &mut empfang))
            }
            Fundstelle::GzGanz => {
                let datei = self.datei_oeffnen(pfand)?;
                let mut strom = MultiGzDecoder::new(BufReader::with_capacity(PUFFER, datei));
                durchreichen(&mut strom, &mut empfang)
            }
            Fundstelle::Erschlossen => Err(Lesefehler::KeineDaten),
        }
    }

    /// Der Verweistext der Verknuepfung `innen`, als Bytes.
    ///
    /// # Errors
    ///
    /// [`Lesefehler`]: der Eintrag fehlt, ist keine Verknuepfung, kommt nicht
    /// heraus, oder die Datei liess sich nicht lesen.
    pub fn verweis_lesen(
        &mut self,
        pfand: &Fernfaden,
        innen: &Innenpfad,
    ) -> Result<Vec<u8>, Lesefehler> {
        let eintrag = self
            .verzeichnis
            .eintrag(innen)
            .cloned()
            .ok_or(Lesefehler::Fehlt)?;
        if eintrag.typ != Typ::Verknuepfung {
            return Err(Lesefehler::Fehlt);
        }
        if let Some(grund) = lesbarkeit_grund(&eintrag.lesbarkeit) {
            return Err(Lesefehler::Unlesbar(grund));
        }
        match eintrag.fundstelle {
            // Im Zip ist der Verweistext der Inhalt des Eintrags.
            Fundstelle::Zip(stelle) => {
                let archiv = self.zip(pfand)?;
                let daten = archiv.by_index(stelle).map_err(zip_lesefehler)?;
                let mut verweis = Vec::new();
                daten
                    .take(HOECHSTENS_VERWEIS + 1)
                    .read_to_end(&mut verweis)
                    .map_err(stromfehler)?;
                if verweis.len() as u64 > HOECHSTENS_VERWEIS {
                    return Err(Lesefehler::Ungueltig(
                        "zip: symlink target too long".to_owned(),
                    ));
                }
                Ok(verweis)
            }
            Fundstelle::Tar { versatz } => self.tar_eintrag(pfand, versatz, |daten| {
                Ok(daten
                    .link_name_bytes()
                    .map(|verweis| verweis.into_owned())
                    .unwrap_or_default())
            }),
            Fundstelle::GzGanz | Fundstelle::Erschlossen => Err(Lesefehler::Fehlt),
        }
    }

    /// Der Eintrag `innen`, sofern er Daten hat, die herauskommen.
    fn datentraeger(&self, innen: &Innenpfad) -> Result<Archiveintrag, Lesefehler> {
        let eintrag = self
            .verzeichnis
            .eintrag(innen)
            .cloned()
            .ok_or(Lesefehler::Fehlt)?;
        if eintrag.typ == Typ::Ordner {
            return Err(Lesefehler::KeineDaten);
        }
        if let Some(grund) = lesbarkeit_grund(&eintrag.lesbarkeit) {
            return Err(Lesefehler::Unlesbar(grund));
        }
        Ok(eintrag)
    }

    /// Oeffnet die Archivdatei ohne Warten und prueft am Deskriptor, dass sie
    /// die Datei des Inhaltsverzeichnisses ist.
    fn datei_oeffnen(&self, _pfand: &Fernfaden) -> Result<File, Lesefehler> {
        let datei = ohne_warten_oeffnen(self.zugang.datei()).map_err(Lesefehler::Lesen)?;
        let angaben = datei.metadata().map_err(Lesefehler::Lesen)?;
        if !angaben.is_file() {
            return Err(Lesefehler::Verzeichnis(Verzeichnisfehler::KeineDatei));
        }
        if Dateikennung::aus_angaben(&angaben) != self.verzeichnis.kennung() {
            return Err(Lesefehler::VonAussenVeraendert);
        }
        Ok(datei)
    }

    /// Das offene Zip, beim ersten Mal geoeffnet.
    fn zip(&mut self, pfand: &Fernfaden) -> Result<&mut ZipArchive<BufReader<File>>, Lesefehler> {
        if !matches!(self.offen, Some(Offen::Zip(_))) {
            let datei = self.datei_oeffnen(pfand)?;
            let archiv =
                ZipArchive::new(BufReader::with_capacity(PUFFER, datei)).map_err(zip_lesefehler)?;
            self.offen = Some(Offen::Zip(archiv));
        }
        match &mut self.offen {
            Some(Offen::Zip(archiv)) => Ok(archiv),
            // Eben gesetzt; der Zweig steht, weil die Unterscheidung
            // vollstaendig ist.
            Some(Offen::Tar(_)) | None => Err(Lesefehler::Fehlt),
        }
    }

    /// Stellt den tar-Strom auf den Eintrag mit dem Kopf bei `versatz` und
    /// gibt ihn an `mit`.
    ///
    /// Der Zeiger steht nach jedem gelesenen Eintrag hinter dessen Daten; die
    /// Bloecke eines tar liegen ab dem Anfang des Stroms auf 512 Bytes, also
    /// beginnt die naechste Kopfgruppe an der naechsten solchen Grenze. Liegt
    /// der verlangte Kopf davor, beginnt der Strom von vorn. Scheitert ein
    /// Eintrag mittendrin, ist die Stelle unbestimmt, und der naechste beginnt
    /// ebenso von vorn.
    fn tar_eintrag<T>(
        &mut self,
        pfand: &Fernfaden,
        versatz: u64,
        mit: impl FnOnce(&mut tar::Entry<'_, Ausschnitt<'_>>) -> Result<T, Lesefehler>,
    ) -> Result<T, Lesefehler> {
        let mut zeiger = match self.offen.take() {
            Some(Offen::Tar(zeiger)) if zeiger.stelle <= versatz => zeiger,
            Some(Offen::Tar(_)) | Some(Offen::Zip(_)) | None => {
                if self.geoeffnet {
                    self.neustarts += 1;
                }
                self.zeiger_oeffnen(pfand)?
            }
        };
        self.geoeffnet = true;
        let abbruch = Arc::clone(&self.abbruch);
        let ergebnis = tar_suchen(&mut zeiger, versatz, &abbruch, mit);
        if ergebnis.is_ok() {
            self.offen = Some(Offen::Tar(zeiger));
        }
        ergebnis
    }

    /// Oeffnet den tar-Strom am Anfang.
    fn zeiger_oeffnen(&self, pfand: &Fernfaden) -> Result<Zeiger, Lesefehler> {
        let datei = self.datei_oeffnen(pfand)?;
        let gelesen = BufReader::with_capacity(PUFFER, datei);
        let strom = if self.zugang.format() == super::Archivformat::TarGz {
            Strom::Gepackt(Box::new(MultiGzDecoder::new(gelesen)))
        } else {
            Strom::Roh(gelesen)
        };
        Ok(Zeiger { strom, stelle: 0 })
    }
}

/// Spult den Zeiger an die naechste Blockgrenze und sucht ab dort den Kopf
/// bei `versatz`; dann `mit`, und danach der Rest der Daten des Eintrags.
fn tar_suchen<T>(
    zeiger: &mut Zeiger,
    versatz: u64,
    abbruch: &AtomicBool,
    mit: impl FnOnce(&mut tar::Entry<'_, Ausschnitt<'_>>) -> Result<T, Lesefehler>,
) -> Result<T, Lesefehler> {
    let auffuellung = (512 - zeiger.stelle % 512) % 512;
    zeiger
        .vorspulen(auffuellung, abbruch)
        .map_err(|fehler| strom_oder_abbruch(fehler, abbruch))?;
    let basis = zeiger.stelle;
    let mut archiv = tar::Archive::new(Ausschnitt {
        zeiger,
        basis,
        abbruch,
    });
    let eintraege = archiv
        .entries_with_seek()
        .map_err(|fehler| strom_oder_abbruch(fehler, abbruch))?;
    for eintrag in eintraege {
        let mut eintrag = eintrag.map_err(|fehler| strom_oder_abbruch(fehler, abbruch))?;
        let kopf = basis + eintrag.raw_header_position();
        if kopf < versatz {
            continue;
        }
        if kopf > versatz {
            return Err(Lesefehler::Fehlt);
        }
        let ergebnis = mit(&mut eintrag)?;
        // Der Rest der Daten, damit der Zeiger hinter dem Eintrag steht.
        io::copy(&mut eintrag, &mut io::sink())
            .map_err(|fehler| strom_oder_abbruch(fehler, abbruch))?;
        return Ok(ergebnis);
    }
    Err(Lesefehler::Fehlt)
}

/// Der Schluessel einer Fundstelle in der Lesefolge.
fn fundstellenschluessel(fundstelle: Fundstelle) -> u64 {
    match fundstelle {
        Fundstelle::Zip(stelle) => stelle as u64,
        Fundstelle::Tar { versatz } => versatz,
        Fundstelle::GzGanz => 0,
        Fundstelle::Erschlossen => u64::MAX,
    }
}

/// Reicht die Daten eines Lesers in Bloecken an `empfang`.
fn durchreichen(
    leser: &mut impl Read,
    empfang: &mut impl FnMut(&[u8]) -> ControlFlow<()>,
) -> Result<u64, Lesefehler> {
    let mut puffer = vec![0u8; PUFFER];
    let mut gelesen: u64 = 0;
    loop {
        let zahl = match leser.read(&mut puffer) {
            Ok(0) => return Ok(gelesen),
            Ok(zahl) => zahl,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
            Err(fehler) => return Err(stromfehler(fehler)),
        };
        gelesen += zahl as u64;
        if empfang(&puffer[..zahl]).is_break() {
            return Err(Lesefehler::Abgebrochen);
        }
    }
}

/// Ein Fehler beim Lesen eines Stroms: mit einer Nummer des Systems ein
/// Lesefehler an der Datei, sonst ein Fehler des Formats, etwa eine falsche
/// Pruefsumme.
fn stromfehler(fehler: io::Error) -> Lesefehler {
    if fehler.raw_os_error().is_some() {
        Lesefehler::Lesen(fehler)
    } else {
        Lesefehler::Ungueltig(fehler.to_string())
    }
}

/// Wie [`stromfehler`], und eine gesetzte Abbruchmarke geht vor: das
/// Vorspulen meldet den Abbruch selbst als Fehler.
fn strom_oder_abbruch(fehler: io::Error, abbruch: &AtomicBool) -> Lesefehler {
    if abbruch.load(Ordering::Acquire) {
        Lesefehler::Abgebrochen
    } else {
        stromfehler(fehler)
    }
}

fn zip_lesefehler(fehler: zip::result::ZipError) -> Lesefehler {
    match zipfehler(fehler) {
        Verzeichnisfehler::Lesen(fehler) => Lesefehler::Lesen(fehler),
        Verzeichnisfehler::Ungueltig(wortlaut) => Lesefehler::Ungueltig(wortlaut),
        andere => Lesefehler::Verzeichnis(andere),
    }
}
