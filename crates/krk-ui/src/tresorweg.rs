//! Der Weg von `secrets.txt` zwischen Platte und Speicher: lesen und
//! entsperren, verschliessen und schreiben, und die Uhr des
//! Schluesselmerkers (Schritt 5 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//!
//! **Herausgehoben aus `editormodell.rs`, damit es ein Weg bleibt.** Bis zum
//! 261016 las und schrieb allein der Editor `secrets.txt`, und Leseroutine und
//! Schreibtyp standen in seinem Modell. Mit den Verbindungen lesen auch der
//! Klick auf ein Verbindungs-Lesezeichen und „Verbinden…“ die Datei; ein
//! zweiter Leseweg daneben haette die Sperre, die Grenze und die Abweisungen
//! ein zweites Mal formuliert. Hier stehen sie einmal:
//!
//! ```text
//!  geheimnisse_lesen(pfad, mittel) ──> bis_zur_grenze_lesen(GEHEIMNISGRENZE)
//!                                       leer?  Pin: neuer_schluessel (Festlegen)
//!                                              Schluessel: gesperrt, mittel_abgewiesen
//!                                       sonst: tresor::oeffnen_mittels
//!                                   ──> Entsperrt { stand, schluessel, eingegeben }
//!
//!  Chiffrat::verschliessen(stand, schluessel) ──> Chiffrat::schreiben(ziel)
//! ```
//!
//! # `secrets.txt` geht allein als Chiffrat auf die Platte
//!
//! [`Chiffrat`] ist der eine Typ, den der Schreibweg nimmt; sein Feld ist
//! privat, und er entsteht an genau einer Stelle,
//! [`Chiffrat::verschliessen`], aus `tresor::verschliessen` nach der
//! Sicherungsform. Die Probe `kein_weg_schreibt_klartext_nach_secrets_txt`
//! (`editormodell.rs`) liest diese Datei und das Modell des Editors zusammen
//! und haelt die Baustelle, das eine atomare Schreiben und die benannten Rufer
//! von [`Chiffrat::schreiben`].
//!
//! # Die Uhr des Merkers
//!
//! Der gemerkte Schluessel gilt zehn Minuten ab der Eingabe
//! (`krk_core::heimordner::schluesselmerker`), gemessen mit `CLOCK_MONOTONIC`
//! und nicht mit `std::time::Instant`, das im Ruhezustand stillsteht. **In
//! dieser Kiste liest allein [`wachzeit`] die Uhr**; die Probe
//! `die_wachzeit_wird_in_dieser_kiste_an_einer_stelle_gelesen` haelt das.
//! Geschrieben wird der Merker allein beim Anwendungsdelegierten
//! (`schluessel_merken`, `schluessel_verwerfen`); gelesen wird er ueber
//! [`gemerkter_schluessel`].

use std::cell::RefCell;
use std::io;
use std::path::Path;
use std::time::Duration;

use krk_core::ablage::atomar;
use krk_core::heimordner::schluesselmerker::Schluesselmerker;
use krk_core::heimordner::tresor::{
    self, Oeffnungsfehler, Oeffnungsmittel, Schluessel, Tresorfehler,
};
use krk_core::sprache::{Text, text};
use krk_core::text::Abweisung;
use krk_core::text::datei::{self, Lesehindernis};

/// Die Grenze fuer das Lesen einer `secrets.txt`: die Editorgrenze fuer den
/// Klartext, dazu der Kopf und die 16 Byte Pruefwert von Poly1305 (Kopftabelle
/// im Modulkopf von `krk_core::heimordner::tresor`). Mehr kann ein Chiffrat
/// nicht tragen, dessen Klartext der Editor annimmt.
pub const GEHEIMNISGRENZE: u64 = datei::EDITORGRENZE + tresor::KOPFLAENGE as u64 + 16;

/// Die Bytes, die von einem verschluesselten Stand auf die Platte gehen.
///
/// **Ein eigener Typ, damit der Schreibweg fuer `secrets.txt` keinen
/// Klartext annehmen kann**: [`Chiffrat::schreiben`] nimmt allein diesen Wert,
/// und er entsteht an genau einer Stelle, in [`Chiffrat::verschliessen`], aus
/// `tresor::verschliessen`. Das Feld ist privat; die Probe
/// `kein_weg_schreibt_klartext_nach_secrets_txt` haelt die eine Baustelle.
#[derive(Debug)]
pub struct Chiffrat(Vec<u8>);

impl Chiffrat {
    /// Der Stand in seiner Sicherungsform, verschlossen mit dem gehaltenen
    /// Schluessel: frische Nonce, Salz und Parameter aus dem Schluessel,
    /// **keine Ableitung**. Die Sicherungsform zuerst, damit der Rundlauf
    /// derselbe ist wie bei `notes.txt` (ein Stand ohne Schlussumbruch kommt
    /// mit einem zurueck).
    pub fn verschliessen(stand: &str, schluessel: &Schluessel) -> Result<Chiffrat, Tresorfehler> {
        let form = datei::sicherungsform(stand);
        let bytes = tresor::verschliessen(form.as_bytes(), schluessel)?;
        Ok(Chiffrat(bytes))
    }

    /// Der Schreibweg des Alltags: atomar, erst in die Nachbardatei, dann
    /// `rename`. Die Nachbardatei traegt damit zu keinem Zeitpunkt Klartext.
    pub fn schreiben(&self, ziel: &Path) -> io::Result<()> {
        atomar::schreiben(ziel, &mut self.0.as_slice())
    }

    /// Die Bytes, allein fuer die Proben, die das Bild der Nachbardatei vor
    /// dem `rename` abfangen.
    #[cfg(test)]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Eine entsperrte `secrets.txt`: der Stand und der Schluessel, mit dem jede
/// Sicherung verschliesst.
#[derive(Debug)]
pub struct Entsperrt {
    /// Der Klartext in gehaltener Form.
    pub stand: String,
    /// Der Schluessel aus dem Oeffnen oder, bei einer leeren Datei, aus der
    /// eben festgelegten PIN.
    pub schluessel: Schluessel,
    /// Ob auf der Platte ein Kopf steht: falsch allein fuer eine leere Datei.
    pub kopf_auf_platte: bool,
    /// Ob der Schluessel aus einer eben geprueften Eingabe stammt (Oeffnen
    /// mit PIN, Festlegen). **Nur dann darf er gemerkt werden**; ein Oeffnen
    /// mit einem gehaltenen Schluessel verlaengert keine Frist (Entscheidung 3
    /// des Plans).
    pub eingegeben: bool,
}

/// Warum [`geheimnisse_lesen`] nichts entsperrt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gesperrt {
    /// Die Abweisung im Wortlaut des Editors: Pfad und Grund.
    pub abweisung: Abweisung,
    /// Ob das Mittel selbst abgewiesen wurde: die PIN ist falsch, der
    /// Schluessel gehoert nicht zur Datei, oder ein Byte ist veraendert, oder
    /// ein Schluessel kam an eine leere Datei. Der Rufer verwirft dann einen
    /// gemerkten Schluessel und fragt nach der PIN (Entscheidung 6 des Plans).
    pub mittel_abgewiesen: bool,
    /// Ob die Datei leer gelesen wurde; die PIN wird dann festgelegt und
    /// nicht eingegeben.
    pub datei_leer: bool,
}

/// Eine Abweisung fuer `secrets.txt`, im Wortlaut des Editors: der Pfad,
/// "laesst sich nicht im Editor oeffnen:" und der Grund.
///
/// **`KeinGueltigesZiel` und kein neuer Wert**, weil der Satz passt und die
/// Behandlung dieselbe ist: der bisherige Stand bleibt, der Grund geht in die
/// Statuszeile. Fuer eine falsche PIN und ein veraendertes Byte ist der Grund
/// derselbe, `Oeffnungsfehler::meldung`.
#[must_use]
pub fn gesperrt(pfad: &Path, grund: String) -> Abweisung {
    Abweisung::KeinGueltigesZiel {
        pfad: pfad.to_path_buf(),
        grund,
        mangel: false,
    }
}

/// Liest `secrets.txt` und entsperrt sie mit dem Mittel des Rufers; laeuft
/// auf einem Arbeitsfaden, weil die Ableitung aus einer PIN rund eine halbe
/// Sekunde kostet.
///
/// **Die Leere wird an den gelesenen Bytes entschieden und nicht an einem
/// `stat(2)` davor**: null Bytes heisst neue Datei, und mit einer PIN entsteht
/// der Schluessel dann aus der eben festgelegten PIN mit frischem Salz.
/// **Mit einem Schluessel entsteht keine neue Datei**: eine neue Datei bekommt
/// ein frisches Salz, also gehoert zu ihr kein gehaltener Schluessel, und die
/// Antwort ist [`Gesperrt::mittel_abgewiesen`] mit [`Gesperrt::datei_leer`]
/// (Entscheidung 6 des Plans). Geschrieben wird hier nichts; eine leere Datei
/// bleibt leer, bis gesichert wird.
///
/// Jeder Fehler ist eine Abweisung mit genau einem Grund: fuer eine falsche PIN,
/// einen fremden Schluessel und eine veraenderte Datei derselbe, fuer einen
/// beschaedigten Kopf der Schaden, fuer ein Versagen des Systems dessen Satz.
pub fn geheimnisse_lesen(pfad: &Path, mittel: &Oeffnungsmittel) -> Result<Entsperrt, Gesperrt> {
    let ohne_mittel = |abweisung| Gesperrt {
        abweisung,
        mittel_abgewiesen: false,
        datei_leer: false,
    };
    let bytes = datei::bis_zur_grenze_lesen(pfad, GEHEIMNISGRENZE).map_err(|hindernis| {
        let (grund, mangel) = match hindernis {
            Lesehindernis::ZuGross => (Text::EditorGeheimnisseZuGross, false),
            Lesehindernis::KeineDatei => (Text::EditorKeineGewoehnlicheDatei, false),
            Lesehindernis::Deskriptormangel => (Text::EditorGeheimnisseKeinDateizugriff, true),
            Lesehindernis::Fehler => (Text::EditorGeheimnisseNichtLesbar, false),
        };
        ohne_mittel(Abweisung::KeinGueltigesZiel {
            pfad: pfad.to_path_buf(),
            grund: text(grund).to_owned(),
            mangel,
        })
    })?;
    let (klartext, schluessel, eingegeben) = if bytes.is_empty() {
        match mittel {
            Oeffnungsmittel::Pin(pin) => {
                let schluessel = tresor::neuer_schluessel(pin)
                    .map_err(|fehler| ohne_mittel(gesperrt(pfad, fehler.meldung())))?;
                (Vec::new(), schluessel, true)
            }
            Oeffnungsmittel::Schluessel(_) => {
                return Err(Gesperrt {
                    abweisung: gesperrt(pfad, text(Text::EditorOhnePin).to_owned()),
                    mittel_abgewiesen: true,
                    datei_leer: true,
                });
            }
        }
    } else {
        let geoeffnet = tresor::oeffnen_mittels(&bytes, mittel).map_err(|fehler| Gesperrt {
            abweisung: gesperrt(pfad, fehler.meldung()),
            mittel_abgewiesen: fehler == Oeffnungsfehler::PinFalschOderVeraendert,
            datei_leer: false,
        })?;
        (
            geoeffnet.klartext,
            geoeffnet.schluessel,
            geoeffnet.eingegeben,
        )
    };
    let stand = datei::einlesen(klartext).ok_or_else(|| {
        ohne_mittel(Abweisung::NichtAlsTextLesbar {
            pfad: pfad.to_path_buf(),
        })
    })?;
    Ok(Entsperrt {
        stand,
        schluessel,
        kopf_auf_platte: !bytes.is_empty(),
        eingegeben,
    })
}

/// Die Wachzeit, an der der Merker gefragt und beschrieben wird: die eine
/// Stelle dieser Kiste, die `verzeichnis::sys::wachzeit` liest
/// (`CLOCK_MONOTONIC`, zaehlt den Ruhezustand mit).
#[must_use]
pub fn wachzeit() -> Duration {
    krk_core::verzeichnis::sys::wachzeit()
}

/// Der gemerkte Schluessel, wenn er jetzt noch gilt, als Abschrift fuer ein
/// [`Oeffnungsmittel::Schluessel`].
///
/// **Ein Leser und kein Schreiber**: er merkt nichts und verlaengert keine
/// Frist. Ist die Frist um, faellt der Schluessel im Merker und wird getilgt,
/// wie `Schluesselmerker::gueltiger` es zusagt; das ist das Tilgen beim
/// Ablauf, das der Wecker beim Anwendungsdelegierten sonst nachholt.
#[must_use]
pub fn gemerkter_schluessel(merker: &RefCell<Schluesselmerker>) -> Option<Schluessel> {
    merker.borrow_mut().gueltiger(wachzeit()).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pruefordner::Pruefordner;
    use krk_core::heimordner::tresor::{Parameter, Pin, SALZLAENGE};

    fn pin(ziffern: &str) -> Pin {
        Pin::aus_eingabe(ziffern).expect("vier Ziffern")
    }

    /// Ein Schluessel mit kleinen Parametern, damit die Proben nicht je
    /// Ableitung eine halbe Sekunde warten.
    fn kleiner_schluessel(ziffern: &str, salz: u8) -> Schluessel {
        let klein = Parameter::neu(64, 1, 1).expect("gueltige Parameter");
        tresor::schluessel_ableiten(&pin(ziffern), &[salz; SALZLAENGE], klein).expect("Ableitung")
    }

    /// Mit dem passenden Schluessel oeffnet die Datei ohne Ableitung, und der
    /// Schluessel gilt nicht als eingegeben; mit einer PIN ist er eingegeben.
    /// Ein Schluessel mit fremdem Salz ist ein abgewiesenes Mittel.
    #[test]
    fn das_mittel_entscheidet_ob_der_schluessel_eingegeben_ist() {
        let ordner = Pruefordner::neu("tresorweg-mittel");
        let pfad = ordner.pfad().join("secrets.txt");
        let schluessel = kleiner_schluessel("0417", 5);
        let chiffrat = Chiffrat::verschliessen("## A\nx\n", &schluessel).expect("verschliessen");
        chiffrat.schreiben(&pfad).expect("schreiben");

        let mit_schluessel =
            geheimnisse_lesen(&pfad, &Oeffnungsmittel::Schluessel(schluessel.clone()))
                .expect("der eigene Schluessel oeffnet");
        assert_eq!(mit_schluessel.stand, "## A\nx\n");
        assert!(!mit_schluessel.eingegeben);
        assert!(mit_schluessel.kopf_auf_platte);

        let mit_pin = geheimnisse_lesen(&pfad, &Oeffnungsmittel::Pin(pin("0417")))
            .expect("die eigene PIN oeffnet");
        assert!(mit_pin.eingegeben);
        assert_eq!(mit_pin.schluessel, schluessel);

        let fremd = geheimnisse_lesen(
            &pfad,
            &Oeffnungsmittel::Schluessel(kleiner_schluessel("0417", 6)),
        )
        .expect_err("ein fremdes Salz oeffnet nicht");
        assert!(fremd.mittel_abgewiesen);
        assert!(!fremd.datei_leer);

        let falsch = geheimnisse_lesen(&pfad, &Oeffnungsmittel::Pin(pin("1111")))
            .expect_err("eine falsche PIN oeffnet nicht");
        assert!(falsch.mittel_abgewiesen);
    }

    /// Eine leere Datei: mit PIN entsteht ein neuer Schluessel, der als
    /// eingegeben gilt; mit einem Schluessel wird abgewiesen, und die Antwort
    /// nennt die Leere, damit der Rufer festlegen laesst.
    #[test]
    fn eine_leere_datei_nimmt_allein_eine_pin_an() {
        let ordner = Pruefordner::neu("tresorweg-leer");
        let pfad = ordner.pfad().join("secrets.txt");
        std::fs::write(&pfad, b"").expect("leere Datei");

        let gesperrt = geheimnisse_lesen(
            &pfad,
            &Oeffnungsmittel::Schluessel(kleiner_schluessel("0417", 5)),
        )
        .expect_err("ein Schluessel legt keine neue Datei an");
        assert!(gesperrt.mittel_abgewiesen && gesperrt.datei_leer);
        assert_eq!(std::fs::metadata(&pfad).expect("stat").len(), 0);

        let neu = geheimnisse_lesen(&pfad, &Oeffnungsmittel::Pin(pin("2468")))
            .expect("mit PIN entsteht ein Schluessel");
        assert!(neu.eingegeben && !neu.kopf_auf_platte);
        assert!(neu.stand.is_empty());
        assert_eq!(neu.schluessel.parameter(), Parameter::DES_CODES);
    }

    /// Ein Merker ohne Schluessel liefert keinen; mit einem frisch gemerkten
    /// liefert er eine Abschrift, und die Frage merkt nicht neu.
    #[test]
    fn der_leser_des_merkers_liefert_eine_abschrift_und_merkt_nicht() {
        let merker = RefCell::new(Schluesselmerker::neu());
        assert_eq!(gemerkter_schluessel(&merker), None);
        let schluessel = kleiner_schluessel("0417", 5);
        let jetzt = wachzeit();
        merker.borrow_mut().merken(schluessel.clone(), jetzt);
        assert_eq!(gemerkter_schluessel(&merker), Some(schluessel));
        let rest = merker
            .borrow()
            .restdauer(jetzt)
            .expect("die Frist laeuft ab dem Merken");
        assert_eq!(
            rest,
            krk_core::heimordner::schluesselmerker::GUELTIGKEIT,
            "die Frage verlaengert nichts"
        );
    }

    /// In dieser Kiste liest allein [`wachzeit`] die Uhr des Merkers, und
    /// `Instant` liest hier niemand fuer ihn.
    #[test]
    fn die_wachzeit_wird_in_dieser_kiste_an_einer_stelle_gelesen() {
        let nadel = concat!("sys::", "wachzeit(");
        let fundstellen: Vec<(String, usize)> = crate::quellbaum::quelldateien()
            .into_iter()
            .filter(|(name, _)| name.starts_with("krk-ui/"))
            .map(|(name, inhalt)| {
                let zahl = crate::quellbaum::codezeilen(&inhalt)
                    .filter(|zeile| zeile.contains(nadel))
                    .count();
                (name, zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(fundstellen, [("krk-ui/src/tresorweg.rs".to_owned(), 1)]);
    }
}
