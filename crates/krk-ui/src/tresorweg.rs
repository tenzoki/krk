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
//!
//! # Die zwei Wege der Verbindungen (Schritte 6 und 7 des Plans)
//!
//! ```text
//!  Klick ──> verbindung_vorbereiten(pfad, name, mittel)
//!              geheimnisse_lesen ──> geheimnisse::finden ──> verbindung::verbindungsziel
//!          ──> Vorbereitung { zu_merken, plan: Verbindungsplan | Verbindungshindernis }
//!
//!  „Verbinden…“ ──> anhaengen(pfad, entwurf, mittel)
//!              geheimnisse_lesen ──> name_frei ──> geheimnisse::hinzufuegen
//!              ──> Chiffrat::verschliessen ──> Chiffrat::schreiben
//!          ──> Anhaengausgang
//! ```
//!
//! **Beide laufen auf einem Arbeitsfaden und lesen den gesicherten Stand der
//! Platte**, nie den ungesicherten des Editors (Entscheidung 15). Beide
//! sagen, ob ein Schluessel aus einer eben geprueften PIN stammt
//! (`zu_merken`); gemerkt wird er beim Anwendungsdelegierten und nicht hier.
//! [`anhaengen`] ist der **zweite Rufer** von [`Chiffrat::schreiben`] neben
//! dem Sichern des Editors, und die Probe
//! `kein_weg_schreibt_klartext_nach_secrets_txt` fuehrt ihn mit Namen.
//!
//! **Das Passwort reist hier als `Passwort` und nie als Text**: aus
//! [`verbindung_vorbereiten`] im [`Verbindungsplan::Einhaengen`], und in
//! [`anhaengen`] im `Geheimnisentwurf`, den [`Verbindungsentwurf::entwurf`]
//! baut. Herausgegeben wird es allein in `appkit/verbinden.rs`, an NetFS.
//!
//! **Was die zwei Wege tilgen und was nicht.** Der entschluesselte Text der
//! Datei steht in [`Entsperrt::stand`] und vorher in den entschluesselten
//! Bytes, beide in einem `Zeroizing`, und faellt getilgt, sobald der Gang
//! endet; ebenso der neue Text, den [`anhaengen`] verschliesst. Nicht getilgt
//! werden die uebrigen Felder des Eintrags neben dem Text (Name, Protokoll,
//! Adresse, Konto, Notizen), die inneren Arbeitsdaten der Verfahren und Reste,
//! die eine Zeichenkette beim Wachsen zuruecklaesst, etwa die der
//! Sicherungsform. `HowTo.md` sagt dem Nutzer dasselbe, unter „Was getilgt
//! wird und was nicht“.

use std::borrow::Cow;
use std::cell::RefCell;
use std::io;
use std::path::Path;
use std::time::Duration;

use krk_core::ablage::atomar;
use krk_core::heimordner::eintraege::geheimnisse::{self, Geheimnisentwurf, Passwort, Suchfehler};
use krk_core::heimordner::schluesselmerker::Schluesselmerker;
use krk_core::heimordner::tresor::{
    self, Oeffnungsfehler, Oeffnungsmittel, Schluessel, Tresorfehler,
};
use krk_core::heimordner::verbindung::{self, Verbindungsziel, Zielfehler};
use krk_core::sprache::{Text, text};
use krk_core::text::Abweisung;
use krk_core::text::datei::{self, Lesehindernis};
use zeroize::Zeroizing;

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
    ///
    /// Eine Sicherungsform, die eine eigene Zeichenkette braucht (der Stand
    /// endet ohne Umbruch), tilgt sich beim Fallen.
    pub fn verschliessen(stand: &str, schluessel: &Schluessel) -> Result<Chiffrat, Tresorfehler> {
        let mut gewachsen: Option<Zeroizing<String>> = None;
        let form: &str = match datei::sicherungsform(stand) {
            Cow::Borrowed(form) => form,
            Cow::Owned(form) => gewachsen.insert(Zeroizing::new(form)),
        };
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
///
/// **Der Stand tilgt sich beim Fallen**, denn er traegt jedes Passwort der
/// Datei: Klick und „Verbinden…“ lassen ihn am Ende ihres Fadens fallen, und
/// der Editor nimmt ihn ueber `std::mem::take` heraus, sodass in seinem Fall
/// eine leere Zeichenkette faellt und der Text im Editor weiterlebt. `Debug`
/// zeigt ihn nicht
/// (`issues/261016-0532_*_howto-verspricht-ein-tilgen-des-verbindungspassworts-das-klick-und-anhaengeweg-nicht-halten.md`).
pub struct Entsperrt {
    /// Der Klartext in gehaltener Form, getilgt beim Fallen.
    pub stand: Zeroizing<String>,
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

impl std::fmt::Debug for Entsperrt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entsperrt")
            .field("stand", &format_args!("<{} Bytes>", self.stand.len()))
            .field("schluessel", &self.schluessel)
            .field("kopf_auf_platte", &self.kopf_auf_platte)
            .field("eingegeben", &self.eingegeben)
            .finish()
    }
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
                (Zeroizing::new(Vec::new()), schluessel, true)
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
            Zeroizing::new(geoeffnet.klartext),
            geoeffnet.schluessel,
            geoeffnet.eingegeben,
        )
    };
    // Gewandelt wird geliehen und in genau eine neue Zeichenkette, die sich
    // beim Fallen tilgt; die entschluesselten Bytes tilgt ihr `Zeroizing`.
    // `datei::einlesen` naehme die Bytes und liesse sie bei einem Fehler
    // ungetilgt fallen.
    let stand = std::str::from_utf8(&klartext)
        .map(|text| Zeroizing::new(datei::gehaltene_form(text).into_owned()))
        .map_err(|_| {
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

/// Die Felder des Blattes „Verbinden“, so wie der Nutzer sie getippt hat
/// (Entscheidung 13 des Plans).
///
/// **Sie bleiben bis zum Ausgang des Schreibgangs im Speicher**, damit eine
/// falsche PIN oder ein vergebener Name keine Eingabe kostet: das Blatt geht
/// mit genau diesen Feldern wieder auf. Das Passwort steht dafuer als Text in
/// einem `Zeroizing`, das sich beim Fallen tilgt, und nicht als `Passwort`:
/// das Feld des Blattes muss es wieder anzeigen koennen, und ein zweiter Weg
/// aus `Passwort` heraus naehme dem Typ die Zusage, die er haelt (sein einer
/// Weg heraus geht an NetFS). Zum `Passwort` wird es in [`Self::entwurf`],
/// unmittelbar vor dem Schreibgang. `Debug` zeigt es nicht.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Verbindungsentwurf {
    /// Der Name des Eintrags und der Beschriftung in der Leiste.
    pub name: String,
    /// Das Protokoll als Text.
    pub protokoll: String,
    /// Die Adresse.
    pub adresse: String,
    /// Das Konto.
    pub konto: String,
    /// Das Passwort, getilgt beim Fallen.
    pub passwort: Zeroizing<String>,
    /// Die Notizen, im Blatt einzeilig.
    pub notizen: String,
}

impl std::fmt::Debug for Verbindungsentwurf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verbindungsentwurf")
            .field("name", &self.name)
            .field("protokoll", &self.protokoll)
            .field("adresse", &self.adresse)
            .field("konto", &self.konto)
            .field("passwort", &"****")
            .field("notizen", &self.notizen)
            .finish()
    }
}

impl Verbindungsentwurf {
    /// Der Eintrag fuer `geheimnisse::hinzufuegen`, die Felder ohne Leerraum
    /// an den Enden bis auf Passwort und Notizen, die so gelten, wie sie
    /// getippt sind.
    #[must_use]
    pub fn entwurf(&self) -> Geheimnisentwurf {
        Geheimnisentwurf {
            name: self.name.trim().to_owned(),
            protokoll: self.protokoll.trim().to_owned(),
            adresse: self.adresse.trim().to_owned(),
            konto: self.konto.trim().to_owned(),
            passwort: Passwort::neu(self.passwort.to_string()),
            notizen: self.notizen.clone(),
        }
    }
}

/// Warum das Blatt „Verbinden“ seine Felder noch nicht annimmt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entwurfsfehler {
    /// Der Name ist leer.
    NameLeer,
    /// Protokoll oder Adresse taugen nicht; den Satz liefert der Kern.
    Ziel(Zielfehler),
    /// Ein Feld traegt einen Zeilenumbruch.
    Umbruch,
    /// Aus Protokoll und Adresse entsteht auch kodiert keine URL; gefragt
    /// vom Blatt ueber `appkit::verbinden::url_aus`, nicht von
    /// [`entwurf_pruefen`], das ohne Foundation auskommt.
    KeineUrl,
}

impl Entwurfsfehler {
    /// Der Satz fuer die Zeile unter den Feldern des Blattes.
    #[must_use]
    pub fn meldung(&self) -> String {
        match self {
            Entwurfsfehler::NameLeer => text(Text::VerbindenNameLeer).to_owned(),
            Entwurfsfehler::Ziel(fehler) => fehler.meldung(),
            Entwurfsfehler::Umbruch => text(Text::AbweisungUmbruchImGeheimnisfeld).to_owned(),
            Entwurfsfehler::KeineUrl => text(Text::VerbindenKeineUrl).to_owned(),
        }
    }
}

/// Die Pruefregel des Blattes „Verbinden“ (Entscheidung 16): Name nicht
/// leer, Protokoll bekannt, Adresse nicht leer und ohne `://`, kein
/// Zeilenumbruch in einem Feld.
///
/// **Protokoll und Adresse fragen dieselben zwei Regeln wie der Klick**
/// (`verbindung::protokoll_pruefen`, `verbindung::adresse_pruefen`). Rein
/// und ohne Fenster pruefbar, und deshalb ohne die dritte Frage des Klicks,
/// ob aus der Adresse eine URL entsteht: die stellt Foundation, und das Blatt
/// fragt sie danach selbst (`appkit::blaetter::verbindung::pruefen`), damit
/// es nichts annimmt, was der Klick danach abweist.
pub fn entwurf_pruefen(entwurf: &Verbindungsentwurf) -> Result<(), Entwurfsfehler> {
    let felder = [
        entwurf.name.as_str(),
        &entwurf.protokoll,
        &entwurf.adresse,
        &entwurf.konto,
        entwurf.passwort.as_str(),
        &entwurf.notizen,
    ];
    if felder.iter().any(|feld| feld.contains(['\n', '\r'])) {
        return Err(Entwurfsfehler::Umbruch);
    }
    if entwurf.name.trim().is_empty() {
        return Err(Entwurfsfehler::NameLeer);
    }
    verbindung::protokoll_pruefen(&entwurf.protokoll).map_err(Entwurfsfehler::Ziel)?;
    verbindung::adresse_pruefen(&entwurf.adresse).map_err(Entwurfsfehler::Ziel)?;
    Ok(())
}

/// Was [`anhaengen`] ausgerichtet hat (Entscheidung 13).
#[derive(Debug)]
#[must_use = "der Ausgang entscheidet, ob ein Lesezeichen entsteht oder das Blatt wieder aufgeht"]
pub enum Anhaengausgang {
    /// Der Eintrag steht am Ende von `secrets.txt`.
    Angehaengt {
        /// Der Schluessel, wenn er aus einer eben geprueften PIN stammt.
        zu_merken: Option<Schluessel>,
    },
    /// Ein Eintrag gleichen Namens steht schon da; geschrieben ist nichts.
    NameVergeben {
        /// Wie bei [`Anhaengausgang::Angehaengt`]: die PIN war richtig.
        zu_merken: Option<Schluessel>,
    },
    /// Das Mittel oeffnet die Datei nicht: falsche PIN, fremder Schluessel,
    /// veraendertes Byte, oder ein Schluessel an einer leeren Datei.
    MittelAbgewiesen {
        /// Ob die Datei leer gelesen wurde; die PIN wird dann festgelegt.
        datei_leer: bool,
        /// Der Satz fuer die Statuszeile, derselbe wie im Editor.
        grund: String,
    },
    /// Jeder andere Fehler, mit seinem Satz; geschrieben ist nichts, oder
    /// das Schreiben selbst ist gescheitert.
    Gescheitert(String),
}

/// Haengt einen Eintrag in der Grundform an `secrets.txt` an, in einem
/// Schreibgang (Entscheidung 13): lesen, oeffnen, Name frei?, anhaengen,
/// verschliessen, atomar schreiben. Laeuft auf dem Faden `krk-tresor`.
///
/// **Ein vergebener Name und ein abgewiesenes Mittel schreiben nichts**, und
/// jede Abweisung vor dem Schreiben laesst Bytes und Stempel der Datei, wie
/// sie sind. Eine leere Datei bekommt mit einer PIN ihren ersten Schluessel
/// und ihr erstes Chiffrat, wie das erste Sichern im Editor.
pub fn anhaengen(
    pfad: &Path,
    entwurf: &Geheimnisentwurf,
    mittel: &Oeffnungsmittel,
) -> Anhaengausgang {
    let entsperrt = match geheimnisse_lesen(pfad, mittel) {
        Ok(entsperrt) => entsperrt,
        Err(gesperrt) if gesperrt.mittel_abgewiesen => {
            return Anhaengausgang::MittelAbgewiesen {
                datei_leer: gesperrt.datei_leer,
                grund: gesperrt.abweisung.meldung(),
            };
        }
        Err(gesperrt) => return Anhaengausgang::Gescheitert(gesperrt.abweisung.meldung()),
    };
    let zu_merken = entsperrt.eingegeben.then(|| entsperrt.schluessel.clone());
    if !geheimnisse::name_frei(&entsperrt.stand, &entwurf.name) {
        return Anhaengausgang::NameVergeben { zu_merken };
    }
    let neutext = match geheimnisse::hinzufuegen(&entsperrt.stand, entwurf) {
        Ok(neustand) => Zeroizing::new(neustand.text),
        Err(abweisung) => return Anhaengausgang::Gescheitert(abweisung.meldung().to_owned()),
    };
    let chiffrat = match Chiffrat::verschliessen(&neutext, &entsperrt.schluessel) {
        Ok(chiffrat) => chiffrat,
        Err(fehler) => return Anhaengausgang::Gescheitert(fehler.meldung()),
    };
    match chiffrat.schreiben(pfad) {
        Ok(()) => Anhaengausgang::Angehaengt { zu_merken },
        Err(fehler) => Anhaengausgang::Gescheitert(fehler.to_string()),
    }
}

/// Was der Faden `krk-verbinden` nach dem Lesen tun soll (Entscheidung 10).
#[derive(Debug)]
pub enum Verbindungsplan {
    /// Ueber NetFS einhaengen, Konto und Passwort als eigene Argumente.
    Einhaengen {
        /// `<schema>://<adresse>`, ohne Konto.
        url: String,
        /// Das Konto, wenn der Eintrag eines nennt.
        konto: Option<String>,
        /// Das Passwort; leer heisst, NetFS bekommt keines.
        passwort: Passwort,
    },
    /// An das System uebergeben, ohne Passwort.
    Uebergeben {
        /// `<schema>://[konto@]adresse`.
        url: String,
    },
}

/// Warum aus einem Klick keine Verbindung entsteht, bevor NetFS gefragt ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verbindungshindernis {
    /// `secrets.txt` laesst sich mit dem Mittel nicht entsperren.
    Gesperrt(Gesperrt),
    /// Kein Eintrag oder mehrere tragen den Namen.
    Suche(Suchfehler),
    /// Protokoll oder Adresse des Eintrags taugen nicht.
    Ziel(Zielfehler),
}

/// Was [`verbindung_vorbereiten`] liefert.
#[derive(Debug)]
pub struct Vorbereitung {
    /// Der Schluessel, wenn er aus einer eben geprueften PIN stammt; auch
    /// dann, wenn danach kein Eintrag passt, denn die PIN war richtig.
    pub zu_merken: Option<Schluessel>,
    /// Der Plan oder das Hindernis.
    pub plan: Result<Verbindungsplan, Verbindungshindernis>,
}

/// Liest `secrets.txt`, findet den Eintrag `name` und baut daraus den Plan
/// der Verbindung (Entscheidungen 9 und 10). Laeuft auf dem Faden
/// `krk-verbinden`.
///
/// **Eine leere Datei hat keinen Eintrag**, und ein Schluessel, der dabei
/// aus einer PIN entstuende, gehoerte zu keiner Datei; er wird nicht gemerkt.
/// Der Klick kommt hier mit einer leeren Datei ohnehin nicht an, er fragt
/// vorher die Groesse (Entscheidung 14).
pub fn verbindung_vorbereiten(pfad: &Path, name: &str, mittel: &Oeffnungsmittel) -> Vorbereitung {
    let entsperrt = match geheimnisse_lesen(pfad, mittel) {
        Ok(entsperrt) => entsperrt,
        Err(gesperrt) => {
            return Vorbereitung {
                zu_merken: None,
                plan: Err(Verbindungshindernis::Gesperrt(gesperrt)),
            };
        }
    };
    if !entsperrt.kopf_auf_platte {
        return Vorbereitung {
            zu_merken: None,
            plan: Err(Verbindungshindernis::Suche(Suchfehler::Keiner)),
        };
    }
    let zu_merken = entsperrt.eingegeben.then(|| entsperrt.schluessel.clone());
    let plan = geheimnisse::finden(&entsperrt.stand, name)
        .map_err(Verbindungshindernis::Suche)
        .and_then(|daten| {
            let ziel = verbindung::verbindungsziel(&daten.protokoll, &daten.adresse, &daten.konto)
                .map_err(Verbindungshindernis::Ziel)?;
            Ok(match ziel {
                Verbindungsziel::Einhaengen { url, konto } => Verbindungsplan::Einhaengen {
                    url,
                    konto,
                    passwort: daten.passwort,
                },
                Verbindungsziel::Uebergeben { url } => Verbindungsplan::Uebergeben { url },
            })
        });
    Vorbereitung { zu_merken, plan }
}

/// Die Groesse von `secrets.txt`: der eine `stat(2)`, den die zwei Wege der
/// Verbindungen auf dem Hauptfaden stellen (Entscheidung 14).
///
/// Wie `Editorbereich::datei_oeffnen` fuer das PIN-Blatt: gelesen wird
/// nichts, entschieden wird allein, ob es die Datei gibt, ob sie leer ist und
/// welche Form das PIN-Blatt hat. Ein Fehler mit `NotFound` heisst, sie fehlt.
pub fn groesse(pfad: &Path) -> io::Result<u64> {
    std::fs::metadata(pfad).map(|daten| daten.len())
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
        assert_eq!(mit_schluessel.stand.as_str(), "## A\nx\n");
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

    /// `Debug` einer entsperrten Datei zeigt keinen Wert aus dem Klartext,
    /// allein dessen Laenge
    /// (`issues/261016-0532_*_howto-verspricht-ein-tilgen-des-verbindungspassworts-das-klick-und-anhaengeweg-nicht-halten.md`).
    #[test]
    fn debug_einer_entsperrten_datei_zeigt_keinen_klartext() {
        let ordner = Pruefordner::neu("tresorweg-debug");
        let pfad = ordner.pfad().join("secrets.txt");
        let schluessel = kleiner_schluessel("0417", 5);
        let klartext =
            "## NAS\nprotocol: smb\naddress: nas.local\naccount: kai\npassword: geheim-4711\n";
        Chiffrat::verschliessen(klartext, &schluessel)
            .expect("verschliessen")
            .schreiben(&pfad)
            .expect("schreiben");

        let entsperrt = geheimnisse_lesen(&pfad, &Oeffnungsmittel::Schluessel(schluessel))
            .expect("der eigene Schluessel oeffnet");
        assert_eq!(entsperrt.stand.as_str(), klartext);
        let gezeigt = format!("{entsperrt:?}");
        for wert in ["geheim-4711", "nas.local", "NAS", "kai", "smb"] {
            assert!(!gezeigt.contains(wert), "Debug zeigt {wert}: {gezeigt}");
        }
        assert!(
            gezeigt.contains(&format!("<{} Bytes>", klartext.len())),
            "{gezeigt}"
        );
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

    /// Ein Entwurf mit eindeutigen Feldwerten.
    fn verbindungsentwurf(name: &str) -> Verbindungsentwurf {
        Verbindungsentwurf {
            name: name.to_owned(),
            protokoll: "smb".to_owned(),
            adresse: "nas.local/daten".to_owned(),
            konto: "kai".to_owned(),
            passwort: zeroize::Zeroizing::new("geheim-4711".to_owned()),
            notizen: "Zugang nur im Büronetz.".to_owned(),
        }
    }

    /// Der Klartext einer verschluesselten Datei, geoeffnet mit der PIN.
    fn klartext(pfad: &Path, ziffern: &str) -> String {
        let bytes = std::fs::read(pfad).expect("lesen");
        let geoeffnet = tresor::oeffnen(&bytes, &pin(ziffern)).expect("die PIN oeffnet");
        String::from_utf8(geoeffnet.klartext).expect("UTF-8")
    }

    /// Akzeptanz (1) von Schritt 7: an eine leere Datei haengt der
    /// Schreibgang mit einer festgelegten PIN das erste Chiffrat, das mit
    /// dieser PIN aufgeht und genau den Eintrag in der Grundform traegt; der
    /// Schluessel ist zu merken.
    #[test]
    fn anhaengen_an_eine_leere_datei_schreibt_das_erste_chiffrat() {
        let ordner = Pruefordner::neu("tresorweg-anhaengen-leer");
        let pfad = ordner.pfad().join("secrets.txt");
        std::fs::write(&pfad, b"").expect("leere Datei");

        let ausgang = anhaengen(
            &pfad,
            &verbindungsentwurf("NAS").entwurf(),
            &Oeffnungsmittel::Pin(pin("2468")),
        );
        assert!(
            matches!(ausgang, Anhaengausgang::Angehaengt { zu_merken: Some(_) }),
            "{ausgang:?}"
        );
        assert_eq!(
            klartext(&pfad, "2468"),
            "## NAS\nprotocol: smb\naddress: nas.local/daten\naccount: kai\n\
             password: geheim-4711\nZugang nur im Büronetz.\n"
        );
    }

    /// Akzeptanz (2): an eine Datei im alten Format bleiben alle alten
    /// Eintraege Byte fuer Byte, und der neue steht hinten; mit einem
    /// gehaltenen Schluessel ist nichts zu merken.
    #[test]
    fn anhaengen_an_das_alte_format_laesst_die_alten_eintraege_stehen() {
        let ordner = Pruefordner::neu("tresorweg-anhaengen-alt");
        let pfad = ordner.pfad().join("secrets.txt");
        let schluessel = kleiner_schluessel("0417", 5);
        let alt = "Vorspann\n## Bank\nPIN 1234\n\n## Mail\nkai@example.org\n";
        Chiffrat::verschliessen(alt, &schluessel)
            .expect("verschliessen")
            .schreiben(&pfad)
            .expect("schreiben");

        let ausgang = anhaengen(
            &pfad,
            &verbindungsentwurf("NAS").entwurf(),
            &Oeffnungsmittel::Schluessel(schluessel),
        );
        assert!(
            matches!(ausgang, Anhaengausgang::Angehaengt { zu_merken: None }),
            "{ausgang:?}"
        );
        let neu = klartext(&pfad, "0417");
        assert!(neu.starts_with(alt), "die alten Eintraege bleiben: {neu}");
        assert!(
            neu[alt.len()..].contains("## NAS\nprotocol: smb\n"),
            "{neu}"
        );
    }

    /// Akzeptanz (3) und (4): ein vergebener Name und eine falsche PIN
    /// schreiben nichts; Bytes und Stempel bleiben.
    #[test]
    fn ein_vergebener_name_und_eine_falsche_pin_schreiben_nichts() {
        let ordner = Pruefordner::neu("tresorweg-anhaengen-nichts");
        let pfad = ordner.pfad().join("secrets.txt");
        let schluessel = kleiner_schluessel("0417", 5);
        Chiffrat::verschliessen("## NAS\nalt\n", &schluessel)
            .expect("verschliessen")
            .schreiben(&pfad)
            .expect("schreiben");
        let vorher = std::fs::read(&pfad).expect("lesen");
        let stempel = std::fs::metadata(&pfad)
            .and_then(|daten| daten.modified())
            .expect("Stempel");

        let vergeben = anhaengen(
            &pfad,
            &verbindungsentwurf("  NAS ").entwurf(),
            &Oeffnungsmittel::Pin(pin("0417")),
        );
        assert!(
            matches!(
                vergeben,
                Anhaengausgang::NameVergeben { zu_merken: Some(_) }
            ),
            "{vergeben:?}"
        );
        let falsch = anhaengen(
            &pfad,
            &verbindungsentwurf("Server").entwurf(),
            &Oeffnungsmittel::Pin(pin("1111")),
        );
        assert!(
            matches!(
                falsch,
                Anhaengausgang::MittelAbgewiesen {
                    datei_leer: false,
                    ..
                }
            ),
            "{falsch:?}"
        );

        assert_eq!(std::fs::read(&pfad).expect("lesen"), vorher);
        assert_eq!(
            std::fs::metadata(&pfad)
                .and_then(|daten| daten.modified())
                .expect("Stempel"),
            stempel
        );
    }

    /// Ein Schluessel an einer leeren Datei ist ein abgewiesenes Mittel mit
    /// leerer Datei: die PIN wird festgelegt (Entscheidung 6).
    #[test]
    fn ein_schluessel_an_einer_leeren_datei_fuehrt_auf_das_festlegen() {
        let ordner = Pruefordner::neu("tresorweg-anhaengen-schluessel-leer");
        let pfad = ordner.pfad().join("secrets.txt");
        std::fs::write(&pfad, b"").expect("leere Datei");
        let ausgang = anhaengen(
            &pfad,
            &verbindungsentwurf("NAS").entwurf(),
            &Oeffnungsmittel::Schluessel(kleiner_schluessel("0417", 5)),
        );
        assert!(
            matches!(
                ausgang,
                Anhaengausgang::MittelAbgewiesen {
                    datei_leer: true,
                    ..
                }
            ),
            "{ausgang:?}"
        );
        assert_eq!(std::fs::metadata(&pfad).expect("stat").len(), 0);
    }

    /// Der Plan einer Verbindung: einhaengen mit Konto und Passwort neben der
    /// URL, uebergeben ohne Passwort; keiner, mehrere und ein unbekanntes
    /// Protokoll sind Hindernisse; eine PIN liefert den Schluessel zum Merken
    /// auch dann, wenn kein Eintrag passt.
    #[test]
    fn der_plan_einer_verbindung_folgt_dem_eintrag() {
        let ordner = Pruefordner::neu("tresorweg-vorbereiten");
        let pfad = ordner.pfad().join("secrets.txt");
        let schluessel = kleiner_schluessel("0417", 5);
        let stand = "## NAS\nprotocol: smb\naddress: nas.local/daten\naccount: kai\n\
                     password: geheim\n\n## Shell\nprotocol: SSH\naddress: host\n\
                     account: root\npassword: x\n\n## Doppelt\n\n## Doppelt\n\n\
                     ## Alt\nprotocol: gopher\naddress: a\n";
        Chiffrat::verschliessen(stand, &schluessel)
            .expect("verschliessen")
            .schreiben(&pfad)
            .expect("schreiben");
        let mittel = Oeffnungsmittel::Schluessel(schluessel);

        let nas = verbindung_vorbereiten(&pfad, " NAS ", &mittel);
        assert!(
            nas.zu_merken.is_none(),
            "ein gehaltener Schluessel merkt nicht neu"
        );
        match nas.plan {
            Ok(Verbindungsplan::Einhaengen {
                url,
                konto,
                passwort,
            }) => {
                assert_eq!(url, "smb://nas.local/daten");
                assert_eq!(konto.as_deref(), Some("kai"));
                assert!(!passwort.ist_leer());
                assert!(!format!("{passwort:?}").contains("geheim"));
            }
            anders => panic!("{anders:?}"),
        }
        match verbindung_vorbereiten(&pfad, "Shell", &mittel).plan {
            Ok(Verbindungsplan::Uebergeben { url }) => assert_eq!(url, "ssh://root@host"),
            anders => panic!("{anders:?}"),
        }
        assert!(matches!(
            verbindung_vorbereiten(&pfad, "Fehlt", &mittel).plan,
            Err(Verbindungshindernis::Suche(Suchfehler::Keiner))
        ));
        assert!(matches!(
            verbindung_vorbereiten(&pfad, "Doppelt", &mittel).plan,
            Err(Verbindungshindernis::Suche(Suchfehler::Mehrdeutig(2)))
        ));
        assert!(matches!(
            verbindung_vorbereiten(&pfad, "Alt", &mittel).plan,
            Err(Verbindungshindernis::Ziel(
                Zielfehler::UnbekanntesProtokoll(_)
            ))
        ));

        let mit_pin = verbindung_vorbereiten(&pfad, "Fehlt", &Oeffnungsmittel::Pin(pin("0417")));
        assert!(mit_pin.zu_merken.is_some(), "die PIN war richtig");
        let falsch = verbindung_vorbereiten(&pfad, "NAS", &Oeffnungsmittel::Pin(pin("1111")));
        assert!(falsch.zu_merken.is_none());
        assert!(matches!(
            falsch.plan,
            Err(Verbindungshindernis::Gesperrt(Gesperrt {
                mittel_abgewiesen: true,
                ..
            }))
        ));
    }

    /// Eine leere Datei hat keinen Eintrag, und ein Schluessel, der dabei aus
    /// einer PIN entstuende, wird nicht gemerkt.
    #[test]
    fn eine_leere_datei_hat_keinen_eintrag_und_nichts_zu_merken() {
        let ordner = Pruefordner::neu("tresorweg-vorbereiten-leer");
        let pfad = ordner.pfad().join("secrets.txt");
        std::fs::write(&pfad, b"").expect("leere Datei");
        let vorbereitung = verbindung_vorbereiten(&pfad, "NAS", &Oeffnungsmittel::Pin(pin("2468")));
        assert!(vorbereitung.zu_merken.is_none());
        assert!(matches!(
            vorbereitung.plan,
            Err(Verbindungshindernis::Suche(Suchfehler::Keiner))
        ));
    }

    /// `Debug` eines Entwurfs zeigt das Passwort nicht, und der Entwurf fuer
    /// den Kern nimmt die Felder ohne Leerraum an den Enden.
    #[test]
    fn der_entwurf_verbirgt_das_passwort() {
        let mut entwurf = verbindungsentwurf(" NAS ");
        entwurf.protokoll = " smb ".to_owned();
        assert!(!format!("{entwurf:?}").contains("geheim-4711"));
        let kern = entwurf.entwurf();
        assert_eq!(kern.name, "NAS");
        assert_eq!(kern.protokoll, "smb");
        assert!(!format!("{kern:?}").contains("geheim-4711"));
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
