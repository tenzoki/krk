//! Der Notizordner, ab Werk `~/krkhome/`: die eine Stelle, die ihn erkennt.
//!
//! F2 fuehrt in einen Dateilisten-Tab auf den Notizordner, und an diesem Ordner
//! haengen mehrere Regeln: die Vorschau rendert `notes.txt` und `tasks.txt`
//! dort mit Aufgabenkaestchen, der Editor zeigt sie in der Formatansicht als
//! Tabelle, und `secrets.txt` bekommt dort vom Inhaltsfilter keinen Auftrag.
//! **Jede dieser Regeln fragt dieses Modul und keine eigene Erkennung** (C2 des Spec
//! `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
//! Die Form der Eintraege in den zwei Dateien steht in [`eintraege`], das
//! Anlegen von Ordner und Dateien samt der einmaligen Uebernahme der alten
//! Zettel in [`bereitstellen`], das Dateiformat von `secrets.txt` mit Kopf,
//! Ableitung und Verschluesselung in [`tresor`], das Lesen und Pruefen des
//! eingestellten Orts in [`ort`].
//!
//! # Zwei Formen, verglichen als Text
//!
//! [`Heimordner`] haelt den Ordner in zwei Formen: der **geschriebenen** (dem
//! eingestellten Ort, ab Werk `<benutzerverzeichnis>/krkhome`) und, wenn der
//! Ort ein symbolischer Verweis ist, der **aufgeloesten**. [`Heimordner::ist`] und
//! [`Heimordner::sonderdatei`] vergleichen den gefragten Pfad als Text mit
//! beiden und **stellen dabei keinen Systemaufruf**. Das ist der Kern der
//! Sache und nicht eine Sparsamkeit: gefragt wird je Lesevorgang eines Tabs
//! und auf dem Hauptfaden, und ein `realpath(3)` am Tab-Ordner blockierte an
//! einem haengenden Netzlaufwerk die Ereignisschleife. Die Probe
//! `ist_und_sonderdatei_stellen_keinen_systemaufruf` in
//! `crates/krk-core/tests/heimordner.rs` liest die beiden Ruempfe und haelt das.
//!
//! Die aufgeloeste Form entsteht an zwei Stellen, und keine davon beruehrt den
//! gefragten Pfad:
//!
//! - **beim Bau** ([`Heimordner::am_ort`]) leicht, ueber `symlink_metadata`
//!   und `read_link` am Verweis, **und nur fuer einen Ort unmittelbar im
//!   Benutzerverzeichnis**. Allein dort treffen beide Aufrufe das
//!   Benutzerverzeichnis und nie das Ziel des Verweises; ein Ziel auf einem
//!   haengenden Laufwerk haelt den Start also nicht auf. Fuer jeden anderen Ort
//!   trafe derselbe Aufruf den Ordner darueber, und der kann selbst auf einem
//!   haengenden Laufwerk liegen; dort entsteht beim Bau keine aufgeloeste Form,
//!   und bis zum ersten F2 erkennt KRK den Ordner allein an der geschriebenen.
//!   Ein relatives Ziel wird gegen das Benutzerverzeichnis gesetzt und
//!   **lexikalisch** bereinigt, also ohne nachzusehen, ob unterwegs ein
//!   weiterer Verweis steht.
//! - **bei F2** ([`Heimordner::aufgeloest_erneuern`]) ueber `canonicalize`,
//!   nachdem das Anlegen am Ziel ohnehin gearbeitet hat, und ebenso bei „Ort
//!   waehlen…“ am gewaehlten Ort. Erst diese Form loest auch Verweise auf, die
//!   im Ziel des ersten Verweises stehen.
//!
//! # Was diese Erkennung nicht sieht
//!
//! **Eine dritte Schreibweise desselben Ordners wird nicht erkannt**: ein
//! zweiter Verweis an anderer Stelle, der auf denselben Ordner zeigt, oder ein
//! Pfad, dessen Bestandteile selbst Verweise sind, die die aufgeloeste Form
//! aufloest und der gefragte Pfad nicht. Dort zeigt KRK `notes.txt` und
//! `tasks.txt` wie jede andere Textdatei und verliert nichts. So entschieden vom
//! Nutzer am 260926
//! (`260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md`,
//! Moeglichkeit 1). **Die Gegenmoeglichkeit liegt ausgearbeitet in demselben
//! Datensatz**: Geraet und Inode, erhoben im Lesefaden am offenen Deskriptor.
//! Sie ist der Weg, wenn sich die Luecke im Gebrauch zeigt.
//!
//! **Fuer `secrets.txt` gilt seit dem 260926 eine Ausnahme davon**, und nur
//! an zwei Stellen: beim Oeffnen und beim Sichern fragt der Editor
//! [`Heimordner::sonderdatei_genau`], die fuer eine Datei dieses Namens
//! Geraet und Inode gegen die geschriebene Form vergleicht
//! (`260926-1119_*_bekommt-secrets-txt-unter-einer-dritten-schreibweise-eine-zusatzpruefung-und-gilt-ziehen-als-kopieren.md`,
//! Moeglichkeit 1). Sonst ginge eine leere `secrets.txt` unter einer dritten
//! Schreibweise ueber den Klartextweg. Dieselbe Regel fragt die Vorschau auf
//! ihrem Lesefaden; alle uebrigen Frager bleiben beim Textvergleich ohne
//! Systemaufruf.
//!
//! Aus derselben Bauart folgt eine zweite, kleinere Eigenschaft: die leichte
//! Form aus `read_link` und die kanonische aus `canonicalize` koennen
//! voneinander abweichen, wenn das Ziel des Verweises selbst ueber einen
//! weiteren Verweis fuehrt. Vor dem ersten F2 einer Sitzung erkennt KRK den
//! Ordner dann ueber den Pfad, den der Verweis nennt, danach ueber den
//! kanonischen; [`Heimordner`] haelt eine aufgeloeste Form und nicht zwei.
//!
//! # Der Ort
//!
//! **Der Ort ist einstellbar**, ueber den Schluessel `notizordner` in
//! `settings.toml` (`260926-1447_*_bekommt-krkhome-ein-eigenes-menue-und-einen-einstellbaren-ort.md`);
//! gelesen und geprueft wird der Text in [`ort`], gebaut wird der Wert der
//! Erkennung in [`Heimordner::am_ort`]. Die Frage nach den Schreibweisen erbt
//! er unveraendert: die Erkennung bleibt ein Textvergleich gegen zwei Formen.
//!
//! [`ORDNERNAME`] ist der Name des **Vorgabeorts** `~/krkhome` und die einzige
//! Stelle im Code, die `krkhome` schreibt; die Auslieferungsfassung von
//! `settings.toml` traegt denselben Ort als Daten. Am Vorgabeort haengt eine
//! Regel, die an keinem anderen gilt: allein dort uebernimmt F2 die alten
//! Zettel ([`Heimordner::ist_vorgabeort`], Modulkopf von `bereitstellen`).

mod bereitstellen;
pub mod eintraege;
pub mod ort;
pub mod tresor;

pub use bereitstellen::{
    ALTE_ZETTEL, ALTER_GEHEIMNISNAME, AlteGeheimnisse, AlterZettel, Bereitstellung, Hindernis,
    Uebernahme, Uebernahmeausgang, Zettelbefund, bereitstellen,
};

use std::path::{Component, Path, PathBuf};

use crate::ablage::pfade;

/// Der Name des Vorgabeorts im Benutzerverzeichnis.
///
/// Gilt, solange `settings.toml` keinen anderen Ort nennt; der Modulkopf sagt,
/// was sonst an ihm haengt.
pub const ORDNERNAME: &str = "krkhome";

/// Eine der Dateien im Heimordner, die KRK als Eintragsdatei behandelt.
///
/// **Vollstaendig und ohne Auffangzweig**, damit eine weitere Datei den Bau an
/// jeder Stelle anhaelt, die nach der Sorte fragt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sonderdatei {
    /// `notes.txt`: Notizen aus Thema und Text.
    Notizen,
    /// `tasks.txt`: Aufgaben mit Erledigt-Kaestchen.
    Aufgaben,
    /// `secrets.txt`: Eintraege wie in `notes.txt`, verschluesselt nach dem
    /// Format aus [`tresor`].
    ///
    /// Sie entsteht bei F2 wie die zwei anderen ueber [`bereitstellen`] und
    /// dort **mit null Bytes**: eine leere Datei traegt noch keinen Kopf, und
    /// die PIN legt erst das erste Oeffnen im Editor fest
    /// (`260926-0007_*_wann-entsteht-secrets-txt-und-was-geschieht-mit-fehlenden-dateien.md`).
    /// Ihr Name traegt keinen Punkt vorn, sie steht also in jeder Liste wie
    /// jede andere Datei
    /// (`260926-1308_*_heisst-die-geheimnisdatei-secrets-txt-ohne-punkt.md`);
    /// eine `.secrets.txt` aus der Zeit davor benennt [`bereitstellen`] um. Im
    /// erkannten Ordner bekommt sie vom Inhaltsfilter keinen Auftrag; die Regel
    /// steht bei [`Heimordner::ohne_inhaltsauftrag`].
    Geheimnisse,
}

impl Sonderdatei {
    /// Jede Eintragsdatei, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Sonderdatei; 3] = [
        Sonderdatei::Notizen,
        Sonderdatei::Aufgaben,
        Sonderdatei::Geheimnisse,
    ];

    /// Der Dateiname im Heimordner.
    pub const fn dateiname(self) -> &'static str {
        match self {
            Sonderdatei::Notizen => "notes.txt",
            Sonderdatei::Aufgaben => "tasks.txt",
            Sonderdatei::Geheimnisse => "secrets.txt",
        }
    }
}

/// Der Heimordner in seiner geschriebenen und, wenn es eine gibt, seiner
/// aufgeloesten Form.
///
/// Ein Wert und kein Zugriff: er haelt zwei Pfade und fragt an ihnen nichts
/// ab. Wer die aufgeloeste Form erneuern will, bekommt von
/// [`Heimordner::aufgeloest_erneuern`] einen neuen Wert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heimordner {
    /// Der eingestellte Ort, lexikalisch bereinigt, unabhaengig davon, was dort
    /// steht.
    geschrieben: PathBuf,
    /// Das Ziel, wenn der Ort ein Verweis ist; `None` sonst, wenn es sich nicht
    /// lesen liess, oder solange der Ort nicht unmittelbar im
    /// Benutzerverzeichnis liegt und noch kein F2 ihn aufgeloest hat.
    aufgeloest: Option<PathBuf>,
    /// Ob der Ort der Vorgabeort `<benutzerverzeichnis>/krkhome` ist.
    vorgabeort: bool,
    /// Der Ort, wie die Statuszeile ihn nennt: `~/…` unter dem
    /// Benutzerverzeichnis, sonst ausgeschrieben.
    anzeige: String,
}

impl Heimordner {
    /// Der Notizordner am genannten Ort.
    ///
    /// `ort` ist ein Pfad aus [`ort::ort_lesen`], also absolut und lexikalisch
    /// bereinigt. **Das Benutzerverzeichnis kommt als Argument herein**, damit
    /// Proben den Ordner in einen Pruefordner legen koennen; dieselbe Erwaegung
    /// wie bei [`pfade::gekuerzt_fuer_anzeige`].
    ///
    /// **Die aufgeloeste Form entsteht nur fuer einen Ort unmittelbar im
    /// Benutzerverzeichnis**, und dort leicht: ist der Ort laut
    /// `symlink_metadata` ein Verweis, gilt sein Ziel aus `read_link`, sonst
    /// gibt es keine. Beide Aufrufe treffen allein den Eintrag im
    /// Benutzerverzeichnis und nie das Ziel. Fuer jeden anderen Ort stellt der
    /// Bau **keinen** Systemaufruf; der Modulkopf sagt, warum.
    pub fn am_ort(ort: PathBuf, benutzerverzeichnis: Option<&Path>) -> Self {
        let im_benutzerverzeichnis =
            benutzerverzeichnis.filter(|zuhause| ort.parent() == Some(*zuhause));
        let aufgeloest = im_benutzerverzeichnis.and_then(|zuhause| {
            let ist_verweis = std::fs::symlink_metadata(&ort)
                .is_ok_and(|angaben| angaben.file_type().is_symlink());
            if !ist_verweis {
                return None;
            }
            std::fs::read_link(&ort)
                .ok()
                .map(|ziel| lexikalisch_bereinigt(&zuhause.join(ziel)))
        });
        let vorgabeort = benutzerverzeichnis.is_some_and(|zuhause| ort == zuhause.join(ORDNERNAME));
        let anzeige = pfade::gekuerzt_fuer_anzeige(&ort, benutzerverzeichnis);
        Self {
            geschrieben: ort,
            aufgeloest,
            vorgabeort,
            anzeige,
        }
    }

    /// Der Vorgabeort unter dem genannten Benutzerverzeichnis,
    /// `<benutzerverzeichnis>/krkhome`, gebaut ueber [`Heimordner::am_ort`].
    pub fn im_benutzerverzeichnis(benutzerverzeichnis: &Path) -> Self {
        Self::am_ort(
            benutzerverzeichnis.join(ORDNERNAME),
            Some(benutzerverzeichnis),
        )
    }

    /// Der Vorgabeort des angemeldeten Benutzers, oder `None`, wenn das System
    /// kein Benutzerverzeichnis nennt.
    pub fn des_benutzers() -> Option<Self> {
        pfade::benutzerverzeichnis().map(|zuhause| Self::im_benutzerverzeichnis(&zuhause))
    }

    /// Ein neuer Wert, dessen aufgeloeste Form ueber `canonicalize` erhoben ist.
    ///
    /// **Allein fuer F2 und „Ort waehlen…“ gedacht**: F2 ruft es, nachdem das
    /// Anlegen am Ziel ohnehin gearbeitet hat, „Ort waehlen…“ am Ort, den der
    /// Nutzer eben im Dialog gewaehlt hat. `canonicalize` beruehrt das Ziel
    /// und darf deshalb weder beim Start noch je Lesevorgang laufen. Scheitert es, bleibt die bisherige Form
    /// stehen: ein Heimordner, der sich gerade nicht aufloesen laesst, ist immer
    /// noch unter seiner geschriebenen Form zu erkennen.
    #[must_use = "der erneuerte Wert ist die ganze Wirkung; fallengelassen bleibt die alte Form im Umlauf"]
    pub fn aufgeloest_erneuern(&self) -> Self {
        match std::fs::canonicalize(&self.geschrieben) {
            Ok(kanonisch) => Self {
                aufgeloest: Some(kanonisch),
                ..self.clone()
            },
            Err(_) => self.clone(),
        }
    }

    /// Die geschriebene Form, der eingestellte Ort.
    ///
    /// Auf sie oeffnet F2 einen neuen Tab: der Tab zeigt dann den Pfad, den der
    /// Nutzer kennt, auch wenn `krkhome` ein Verweis ist.
    pub fn geschrieben(&self) -> &Path {
        &self.geschrieben
    }

    /// Die aufgeloeste Form, falls es eine gibt.
    ///
    /// Gefragt von „Ort waehlen…“: gleicht die kanonische Form des gewaehlten
    /// Orts der aufgeloesten des geltenden, ist es derselbe Ordner unter einer
    /// anderen Schreibweise. Ein Wert, kein Systemaufruf.
    pub fn aufgeloest(&self) -> Option<&Path> {
        self.aufgeloest.as_deref()
    }

    /// Ob der Ort der Vorgabeort `<benutzerverzeichnis>/krkhome` ist.
    ///
    /// Ein Vergleich der geschriebenen Form beim Bau, kein Systemaufruf. Allein
    /// dort uebernimmt F2 die alten Zettel (Modulkopf von `bereitstellen`).
    pub fn ist_vorgabeort(&self) -> bool {
        self.vorgabeort
    }

    /// Der Ort, wie die Statuszeile ihn nennt: `~/…` fuer einen Ort unter dem
    /// Benutzerverzeichnis, sonst der ausgeschriebene Pfad.
    pub fn anzeigename(&self) -> &str {
        &self.anzeige
    }

    /// Ob der gefragte Ordner der Heimordner ist, in der geschriebenen oder der
    /// aufgeloesten Form.
    ///
    /// Verglichen wird ueber die Gleichheit von [`Path`], also Bestandteil fuer
    /// Bestandteil. Ein Schlussstrich und doppelte Trennstriche sind damit
    /// ohne eigenen Schritt gleichgueltig. **Kein Systemaufruf**; der Modulkopf
    /// sagt, warum, und was daraus folgt.
    pub fn ist(&self, ordner: &Path) -> bool {
        ordner == self.geschrieben.as_path()
            || self
                .aufgeloest
                .as_deref()
                .is_some_and(|aufgeloest| ordner == aufgeloest)
    }

    /// Welche Eintragsdatei der gefragte Pfad ist, oder `None`.
    ///
    /// Gefragt wird erst der Dateiname und nur bei einem der Namen der
    /// Elternordner ueber [`Heimordner::ist`]. **Kein Systemaufruf.**
    pub fn sonderdatei(&self, pfad: &Path) -> Option<Sonderdatei> {
        let name = pfad.file_name()?;
        let sorte = Sonderdatei::ALLE
            .into_iter()
            .find(|sorte| name == sorte.dateiname())?;
        let ordner = pfad.parent()?;
        self.ist(ordner).then_some(sorte)
    }

    /// [`Heimordner::sonderdatei`], und fuer eine Datei namens `secrets.txt`
    /// unter jeder Schreibweise des Ordners: **Geraet und Inode** gegen die
    /// geschriebene Form `<heimordner>/secrets.txt`.
    ///
    /// **Allein fuer das Oeffnen und das Sichern gedacht**
    /// (`260926-1119_*_bekommt-secrets-txt-unter-einer-dritten-schreibweise-eine-zusatzpruefung-und-gilt-ziehen-als-kopieren.md`,
    /// Moeglichkeit 1): erkennt der Pfadtext die Datei nicht, heisst sie aber
    /// `secrets.txt`, fragt diese Funktion das Dateisystem, und zwar mit
    /// `metadata` an beiden Pfaden, also dem Verweis folgend. Jeder andere Name
    /// kommt ohne Systemaufruf zurueck, und die zwei Formen fuer `notes.txt` und
    /// `tasks.txt` bleiben, wie sie sind: an ihnen haengt kein Klartext.
    ///
    /// **Der Name wird ohne Ruecksicht auf Gross- und Kleinschreibung
    /// verglichen**, weil `Secrets.txt` auf einem Volume ohne diese
    /// Unterscheidung dieselbe Inode ist; ueber die Gleichheit entscheidet dann
    /// die Inode und nicht der Name.
    ///
    /// **Im Zweifel ist es die Geheimnisdatei.** Fehlt `secrets.txt` im
    /// Heimordner oder der gefragte Pfad, kann er sie nicht sein; laesst sich
    /// eine der beiden Seiten aus einem anderen Grund nicht erheben (Rechte,
    /// Ein-/Ausgabefehler), antwortet die Funktion ja. Die falsche Seite des
    /// Irrtums waere ein Klartext in der Datei der Geheimnisse; die richtige
    /// ist ein PIN-Blatt, das eine gewoehnliche Datei nicht oeffnet.
    pub fn sonderdatei_genau(&self, pfad: &Path) -> Option<Sonderdatei> {
        if let Some(sorte) = self.sonderdatei(pfad) {
            return Some(sorte);
        }
        let geheimnisse = Sonderdatei::Geheimnisse.dateiname();
        let traegt_den_namen = pfad
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case(geheimnisse));
        if !traegt_den_namen {
            return None;
        }
        let bezug = self.geschrieben.join(geheimnisse);
        dieselbe_datei(pfad, &bezug).then_some(Sonderdatei::Geheimnisse)
    }

    /// Der Name des Eintrags, dem der Inhaltsfilter im gefragten Ordner nie
    /// einen Auftrag gibt, oder `None`.
    ///
    /// **Eine Eigenschaft des Ordners und keine des Eintrags**: gefragt wird
    /// einmal je Lesevorgang und nicht je Eintrag, und das Ordnermodell haelt
    /// die Antwort (`Ordnermodell::ohne_inhaltsauftrag_setzen`). Im erkannten
    /// Ordner ist es `secrets.txt`, in jedem anderen nichts
    /// (`260926-0050_*_wie-weit-reicht-der-inhaltsfilter-liest-secrets-txt-nicht-wenn-das-kennzeichen-versteckt-ihn-nicht-haelt.md`,
    /// Moeglichkeit 1). Bis zum 260926 hiess sie `immer_gelistet` und hielt
    /// zugleich eine versteckte `.secrets.txt` in der Liste; mit dem Namen ohne
    /// Punkt ist dieser Teil entfallen
    /// (`260926-1308_*_heisst-die-geheimnisdatei-secrets-txt-ohne-punkt.md`).
    /// **Kein Systemaufruf**, denn gefragt wird allein [`Heimordner::ist`].
    pub fn ohne_inhaltsauftrag(&self, ordner: &Path) -> Option<&'static str> {
        self.ist(ordner)
            .then_some(Sonderdatei::Geheimnisse.dateiname())
    }
}

/// Ob beide Pfade dieselbe Datei nennen, nach Geraet und Inode; die Regel
/// fuer Fehler steht an [`Heimordner::sonderdatei_genau`].
fn dieselbe_datei(gefragt: &Path, bezug: &Path) -> bool {
    use std::io::ErrorKind;
    use std::os::unix::fs::MetadataExt;
    let erheben = |pfad: &Path| match std::fs::metadata(pfad) {
        Ok(angaben) => Ok(Some((angaben.dev(), angaben.ino()))),
        Err(fehler) if fehler.kind() == ErrorKind::NotFound => Ok(None),
        Err(fehler) => Err(fehler),
    };
    match (erheben(gefragt), erheben(bezug)) {
        (Ok(Some(gefragt)), Ok(Some(bezug))) => gefragt == bezug,
        // Eine Seite fehlt: dann ist es nicht dieselbe Datei.
        (Ok(None), _) | (_, Ok(None)) => false,
        // Eine Seite laesst sich nicht erheben: im Zweifel die Geheimnisse.
        (Err(_), _) | (_, Err(_)) => true,
    }
}

/// Der Pfad ohne `.` und mit jedem `..` gegen den Bestandteil davor
/// aufgehoben, **ohne das Dateisystem zu fragen**.
///
/// Das ist nicht dasselbe wie `canonicalize`: steht vor einem `..` ein Verweis,
/// fuehrt das lexikalische Aufheben woanders hin als das System. Fuer die
/// leichte Form ist das hingenommen, weil sie das Ziel nicht beruehren darf;
/// F2 ersetzt sie durch die kanonische.
fn lexikalisch_bereinigt(pfad: &Path) -> PathBuf {
    let mut bereinigt = PathBuf::new();
    for teil in pfad.components() {
        match teil {
            Component::CurDir => {}
            Component::ParentDir => {
                // An der Wurzel bleibt `..` die Wurzel, wie beim System.
                let _ = bereinigt.pop();
            }
            anderer => bereinigt.push(anderer),
        }
    }
    bereinigt
}
