//! Das Dateiformat von `.secrets.txt`: Kopf, Schluesselableitung und
//! Verschluesselung (Schritt 5.1 des Plans
//! `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`,
//! Faehigkeit C7 des Spec).
//!
//! Verschluesselt wird mit XChaCha20-Poly1305, der Schluessel entsteht mit
//! Argon2id aus einer vierstelligen PIN
//! (`260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md`,
//! Moeglichkeit 1). **Das Bedrohungsmodell ist allein das versehentliche Lesen
//! durch Agenten, Werkzeuge und Indexer**; gegen jemanden, der die Datei kopiert
//! und die zehntausend moeglichen PINs durchprobiert, schuetzt das Format nicht,
//! und es sagt das auch nicht zu. Ebenso wenig sagt es zu, dass Schluessel oder
//! Klartext im Speicher getilgt werden: der Editor haelt beide, solange die
//! Datei offen ist, und ein Tilgen an dieser einen Stelle erreichte die
//! Textflaeche nicht (Spec, `## Constraints`).
//!
//! # Der Kopf, Byte fuer Byte
//!
//! Eine nicht leere `.secrets.txt` ist eine Binaerdatei aus [`KOPFLAENGE`]
//! Bytes Kopf und dem Chiffrat dahinter. Zahlen stehen als vorzeichenlose
//! Ganzzahlen in der Bytefolge *little-endian*.
//!
//! | Versatz | Laenge | Inhalt |
//! |---:|---:|---|
//! | 0 | 6 | Kennung, die ASCII-Zeichen `KRKSEC` |
//! | 6 | 1 | Formatversion, heute [`FORMATVERSION`] = 1 |
//! | 7 | 1 | Kennung der Ableitung, heute [`ABLEITUNG_ARGON2ID`] = 1: Argon2id, Fassung `0x13` |
//! | 8 | 4 | Speicher der Ableitung in KiB (`m`) |
//! | 12 | 4 | Durchlaeufe der Ableitung (`t`) |
//! | 16 | 4 | Parallelitaet der Ableitung (`p`) |
//! | 20 | 16 | Salz |
//! | 36 | 24 | Nonce von XChaCha20-Poly1305 |
//! | 60 | Rest | Chiffrat samt 16 Byte Pruefwert von Poly1305 |
//!
//! Der Schluessel sind 32 Byte aus Argon2id ueber die vier Ziffern der PIN als
//! ASCII-Bytes (`"0417"` sind die Bytes `30 34 31 37`) und das Salz, ohne
//! Geheimwert und ohne zusaetzliche Daten. **Die ganzen 60 Byte des Kopfes
//! gehen als zusaetzliche authentifizierte Daten in den Pruefwert ein**: wer
//! eine Zahl im Kopf aendert, bekommt dieselbe Abweisung wie bei einer falschen
//! PIN. Mit dieser Tabelle laesst sich eine Datei ohne KRK entschluesseln; die
//! Probe `eine_von_hand_gebaute_datei_mit_kleineren_parametern_oeffnet` in
//! `crates/krk-core/tests/heimordner.rs` baut eine allein aus ihr.
//!
//! # Wann abgeleitet wird
//!
//! **Das Salz entsteht allein beim Festlegen und beim Aendern der PIN**, jede
//! Sicherung zieht dagegen eine neue Nonce
//! (`260926-0050_*_zieht-jede-sicherung-von-secrets-txt-ein-neues-salz-wenn-das-eine-halbe-sekunde-je-cmd-s-kostet.md`,
//! Moeglichkeit 3). Abgeleitet wird deshalb an genau zwei Stellen, in
//! [`oeffnen`] und in [`neuer_schluessel`]; [`verschliessen`] nimmt den
//! gehaltenen [`Schluessel`] und leitet nicht ab, und ein `cmd+s` an
//! `.secrets.txt` kostet so viel wie an jeder anderen Datei. Die 24 Byte der
//! Nonce von XChaCha20 sind gross genug, um sie je Sicherung zufaellig zu
//! ziehen, ohne einen Zaehler zu verwalten.
//!
//! Die Parameter stehen im Kopf und nicht im Code: eine Datei oeffnet mit den
//! Parametern, mit denen sie geschrieben wurde, und eine gewoehnliche Sicherung
//! uebernimmt sie aus dem gehaltenen Schluessel. **Angehobene Parameter des
//! Codes ([`Parameter::DES_CODES`]) erreichen eine bestehende Datei deshalb erst
//! mit dem naechsten Aendern der PIN** und nicht mit dem naechsten Sichern.
//!
//! # Die Parameter, gemessen auf dem Referenzgeraet
//!
//! [`Parameter::DES_CODES`] ist Argon2id mit 128 MiB Speicher, sieben
//! Durchlaeufen und einer Spur. Gemessen am 260926 auf dem Referenzgeraet
//! (`MacBookPro15,1`, Intel Core i9-9880H mit 2,3 GHz, 16 GB;
//! `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_*_leistungszusagen-navigator.md`)
//! mit der Probe `argon2_parameter_auf_dem_referenzgeraet_messen` in
//! `crates/krk-core/tests/heimordner.rs`, im Profil `release`, der Median aus
//! fuenf Ableitungen je Reihe, aus dem letzten von drei Laeufen (die zwei
//! davor wichen je Reihe um hoechstens 0,03 s ab):
//!
//! | Speicher | Durchlaeufe | Median |
//! |---:|---:|---:|
//! | 64 MiB | 3 | 0,113 s |
//! | 128 MiB | 2 | 0,162 s |
//! | 128 MiB | 3 | 0,236 s |
//! | 128 MiB | 4 | 0,309 s |
//! | 128 MiB | 5 | 0,384 s |
//! | 128 MiB | 6 | 0,453 s |
//! | **128 MiB** | **7** | **0,537 s** |
//! | 192 MiB | 3 | 0,435 s |
//! | 256 MiB | 2 | 0,435 s |
//! | 256 MiB | 3 | 0,606 s |
//! | 256 MiB | 4 | 0,733 s |
//! | 384 MiB | 2 | 0,657 s |
//! | 512 MiB | 2 | 0,900 s |
//!
//! Das Ziel war eine halbe Sekunde je Ableitung, und gewaehlt ist die Reihe,
//! die ihr am naechsten liegt: **128 MiB mit sieben Durchlaeufen**, 0,037 s
//! darueber. Die naechsten Reihen liegen 0,047 s darunter (128 MiB mit sechs
//! Durchlaeufen) und 0,065 s darunter (192 MiB mit drei, 256 MiB mit zwei). Die
//! Parallelitaet bleibt bei eins: ohne das Merkmal `parallel` von `argon2`
//! rechnete eine zweite Spur ohnehin nacheinander und kaufte nichts. Wer die
//! Parameter anhebt, misst mit derselben Probe neu und traegt die Tabelle
//! hier und die Zahl in der Wurzel-`Cargo.toml` nach.
//!
//! # Was ein beschaedigter Kopf ist
//!
//! [`Oeffnungsfehler`] trennt zwei Faelle und nicht mehr: einen Kopf, der sich
//! gar nicht lesen laesst ([`Kopfschaden`]), und alles, was die Pruefung des
//! Verfahrens abweist. **Eine falsche PIN und ein veraendertes Byte hinter dem
//! Kopf sind fuer ein AEAD-Verfahren nicht zu unterscheiden**, und der Spec legt
//! sie zu einer Meldung zusammen. Zum Kopfschaden zaehlt neben falscher
//! Kennung, zu kurzer Datei, unbekannter Version und unbekannter Ableitung auch
//! ein Satz Parameter ausserhalb der Grenzen aus [`Parameter::neu`]: ein Kopf,
//! der eine Ableitung ueber Terabytes oder Millionen Durchlaeufe verlangte,
//! hielte den Ladefaden sonst unabsehbar lange an, bevor die Pruefung ihn
//! abweisen koennte.
//!
//! # Zufall
//!
//! Salz und Nonce kommen vom Betriebssystem, ueber `getrandom`, das
//! `chacha20poly1305` mit seinem Merkmal `getrandom` hereinholt; KRK nennt die
//! Kiste nicht selbst. Scheitert das Betriebssystem an einem Zufallswert, ist
//! das [`Tresorfehler::KeinZufall`] und keine Panik.

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Generate, KeyInit, Payload};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};

/// Die Kennung am Anfang jeder nicht leeren `.secrets.txt`.
pub const KENNUNG: [u8; 6] = *b"KRKSEC";

/// Die Formatversion, die dieser Code schreibt. Gelesen wird jede Version, die
/// er kennt, und heute ist das allein diese.
pub const FORMATVERSION: u8 = 1;

/// Die Kennung der Ableitung Argon2id in der Fassung `0x13`.
pub const ABLEITUNG_ARGON2ID: u8 = 1;

/// Die Laenge des Salzes in Byte.
pub const SALZLAENGE: usize = 16;

/// Die Laenge der Nonce von XChaCha20-Poly1305 in Byte.
pub const NONCELAENGE: usize = 24;

/// Die Laenge des abgeleiteten Schluessels in Byte.
const SCHLUESSELLAENGE: usize = 32;

/// Die Laenge des Kopfes in Byte; dahinter beginnt das Chiffrat.
pub const KOPFLAENGE: usize = 6 + 1 + 1 + 4 + 4 + 4 + SALZLAENGE + NONCELAENGE;

/// Die obere Grenze des Speichers, den ein Kopf verlangen darf: 4 GiB, das
/// Zweiunddreissigfache von [`Parameter::DES_CODES`]. Die drei Grenzen lassen
/// Raum, die Parameter des Codes spaeter anzuheben; eine Datei mit Parametern
/// darueber gilt einem aelteren KRK als beschaedigt, und ihr Inhalt bleibt
/// unberuehrt.
const SPEICHER_HOECHSTENS_KIB: u32 = 4 * 1024 * 1024;

/// Die obere Grenze der Durchlaeufe, die ein Kopf verlangen darf.
const DURCHLAEUFE_HOECHSTENS: u32 = 64;

/// Die obere Grenze der Parallelitaet, die ein Kopf verlangen darf.
const PARALLELITAET_HOECHSTENS: u32 = 16;

/// Eine PIN: genau vier ASCII-Ziffern.
///
/// `Debug` zeigt die Ziffern nicht, damit eine PIN nicht ueber eine
/// Fehlerausgabe in ein Protokoll geraet. Das ist Umsicht und kein Tilgen: der
/// Wert steht im Speicher, solange ihn jemand haelt.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pin([u8; 4]);

/// Warum eine Eingabe keine PIN ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pinfehler {
    /// Die Eingabe besteht nicht aus genau vier Ziffern von `0` bis `9`.
    KeineVierZiffern,
}

impl Pinfehler {
    /// Der Satz fuer das Blatt der PIN-Abfrage.
    pub fn meldung(&self) -> &'static str {
        match self {
            Pinfehler::KeineVierZiffern => "Die PIN besteht aus genau vier Ziffern.",
        }
    }
}

impl Pin {
    /// Nimmt genau vier ASCII-Ziffern an und sonst nichts: keinen Leerraum
    /// davor oder dahinter, keine Ziffern anderer Schriften.
    pub fn aus_eingabe(eingabe: &str) -> Result<Pin, Pinfehler> {
        let bytes = eingabe.as_bytes();
        match <[u8; 4]>::try_from(bytes) {
            Ok(ziffern) if ziffern.iter().all(u8::is_ascii_digit) => Ok(Pin(ziffern)),
            _ => Err(Pinfehler::KeineVierZiffern),
        }
    }

    /// Die vier Ziffern als ASCII-Bytes; das ist das Kennwort der Ableitung.
    fn bytes(&self) -> &[u8; 4] {
        &self.0
    }
}

impl std::fmt::Debug for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Pin(****)")
    }
}

/// Die Parameter von Argon2id, wie sie im Kopf stehen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameter {
    speicher_kib: u32,
    durchlaeufe: u32,
    parallelitaet: u32,
}

impl Parameter {
    /// Die Parameter, mit denen dieser Code neue Schluessel ableitet; gemessen,
    /// siehe den Modulkopf. Die Probe
    /// `die_parameter_des_codes_liegen_in_den_grenzen` haelt, dass
    /// [`Parameter::neu`] sie annimmt.
    pub const DES_CODES: Parameter = Parameter {
        speicher_kib: 128 * 1024,
        durchlaeufe: 7,
        parallelitaet: 1,
    };

    /// Nimmt einen Satz Parameter an, wenn Argon2id ihn annimmt und er unter
    /// den oberen Grenzen dieses Moduls bleibt (4 GiB, 64 Durchlaeufe, 16
    /// Spuren). Sonst `None`. Die Grenzen begrenzen, was ein beschaedigter
    /// Kopf den Ladefaden kosten kann; aufheben koennen sie es nicht.
    pub fn neu(speicher_kib: u32, durchlaeufe: u32, parallelitaet: u32) -> Option<Parameter> {
        let in_grenzen = speicher_kib <= SPEICHER_HOECHSTENS_KIB
            && durchlaeufe <= DURCHLAEUFE_HOECHSTENS
            && parallelitaet <= PARALLELITAET_HOECHSTENS;
        let kandidat = Parameter {
            speicher_kib,
            durchlaeufe,
            parallelitaet,
        };
        (in_grenzen && kandidat.argon2().is_ok()).then_some(kandidat)
    }

    /// Der Speicher in KiB.
    pub fn speicher_kib(&self) -> u32 {
        self.speicher_kib
    }

    /// Die Zahl der Durchlaeufe.
    pub fn durchlaeufe(&self) -> u32 {
        self.durchlaeufe
    }

    /// Die Zahl der Spuren.
    pub fn parallelitaet(&self) -> u32 {
        self.parallelitaet
    }

    fn argon2(&self) -> Result<Params, argon2::Error> {
        Params::new(
            self.speicher_kib,
            self.durchlaeufe,
            self.parallelitaet,
            Some(SCHLUESSELLAENGE),
        )
    }
}

/// Der aus PIN und Salz abgeleitete Schluessel, samt dem Salz und den
/// Parametern, aus denen er stammt.
///
/// Der Editor haelt ihn, solange `.secrets.txt` offen ist, und jede Sicherung
/// verschliesst mit ihm, ohne neu abzuleiten. Salz und Parameter reisen mit,
/// weil [`verschliessen`] sie in den Kopf schreibt. `Debug` zeigt die
/// Schluesselbytes nicht; getilgt werden sie nicht (Modulkopf).
#[derive(Clone, PartialEq, Eq)]
pub struct Schluessel {
    schluessel: [u8; SCHLUESSELLAENGE],
    salz: [u8; SALZLAENGE],
    parameter: Parameter,
}

impl Schluessel {
    /// Das Salz, aus dem der Schluessel stammt.
    pub fn salz(&self) -> &[u8; SALZLAENGE] {
        &self.salz
    }

    /// Die Parameter, mit denen er abgeleitet ist.
    pub fn parameter(&self) -> Parameter {
        self.parameter
    }
}

impl std::fmt::Debug for Schluessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schluessel")
            .field("schluessel", &"<32 Byte>")
            .field("salz", &self.salz)
            .field("parameter", &self.parameter)
            .finish()
    }
}

/// Der gelesene Kopf einer `.secrets.txt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kopf {
    version: u8,
    ableitung: u8,
    parameter: Parameter,
    salz: [u8; SALZLAENGE],
    nonce: [u8; NONCELAENGE],
}

/// Warum sich ein Kopf nicht lesen laesst.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kopfschaden {
    /// Die ersten sechs Byte sind nicht `KRKSEC`.
    FalscheKennung,
    /// Die Datei ist kuerzer als der Kopf.
    Abgeschnitten,
    /// Eine Formatversion, die dieser Code nicht kennt.
    UnbekannteVersion(u8),
    /// Eine Kennung der Ableitung, die dieser Code nicht kennt.
    UnbekannteAbleitung(u8),
    /// Parameter, die Argon2id nicht annimmt oder die ueber den Grenzen aus
    /// [`Parameter::neu`] liegen.
    UngueltigeParameter,
}

impl Kopf {
    /// Liest den Kopf vom Anfang der Bytes. Leitet nicht ab und prueft nichts
    /// ausser dem Kopf selbst.
    pub fn lesen(bytes: &[u8]) -> Result<Kopf, Kopfschaden> {
        // Die Kennung zuerst: eine kurze Datei mit fremdem Anfang ist keine
        // abgeschnittene Geheimnisdatei, sondern gar keine.
        let kennung_da = bytes.len().min(KENNUNG.len());
        if bytes[..kennung_da] != KENNUNG[..kennung_da] {
            return Err(Kopfschaden::FalscheKennung);
        }
        let Some(kopf) = bytes.get(..KOPFLAENGE) else {
            return Err(Kopfschaden::Abgeschnitten);
        };
        let version = kopf[6];
        if version != FORMATVERSION {
            return Err(Kopfschaden::UnbekannteVersion(version));
        }
        let ableitung = kopf[7];
        if ableitung != ABLEITUNG_ARGON2ID {
            return Err(Kopfschaden::UnbekannteAbleitung(ableitung));
        }
        let zahl =
            |ab: usize| u32::from_le_bytes([kopf[ab], kopf[ab + 1], kopf[ab + 2], kopf[ab + 3]]);
        let parameter =
            Parameter::neu(zahl(8), zahl(12), zahl(16)).ok_or(Kopfschaden::UngueltigeParameter)?;
        let mut salz = [0u8; SALZLAENGE];
        salz.copy_from_slice(&kopf[20..20 + SALZLAENGE]);
        let mut nonce = [0u8; NONCELAENGE];
        nonce.copy_from_slice(&kopf[36..36 + NONCELAENGE]);
        Ok(Kopf {
            version,
            ableitung,
            parameter,
            salz,
            nonce,
        })
    }

    /// Die Formatversion.
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Die Parameter der Ableitung.
    pub fn parameter(&self) -> Parameter {
        self.parameter
    }

    /// Das Salz.
    pub fn salz(&self) -> &[u8; SALZLAENGE] {
        &self.salz
    }

    /// Die Nonce.
    pub fn nonce(&self) -> &[u8; NONCELAENGE] {
        &self.nonce
    }

    fn bytes(&self) -> [u8; KOPFLAENGE] {
        let mut kopf = [0u8; KOPFLAENGE];
        kopf[..6].copy_from_slice(&KENNUNG);
        kopf[6] = self.version;
        kopf[7] = self.ableitung;
        kopf[8..12].copy_from_slice(&self.parameter.speicher_kib.to_le_bytes());
        kopf[12..16].copy_from_slice(&self.parameter.durchlaeufe.to_le_bytes());
        kopf[16..20].copy_from_slice(&self.parameter.parallelitaet.to_le_bytes());
        kopf[20..20 + SALZLAENGE].copy_from_slice(&self.salz);
        kopf[36..36 + NONCELAENGE].copy_from_slice(&self.nonce);
        kopf
    }
}

/// Ein Versagen des Systems unter dem Verfahren: kein Zufall, kein Speicher.
/// Es sagt nichts ueber PIN oder Datei.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tresorfehler {
    /// Das Betriebssystem lieferte keinen Zufallswert; der Grund des Systems.
    KeinZufall(String),
    /// Die Ableitung scheiterte, in der Praxis am Speicher; der Grund der Kiste.
    Ableitung(String),
    /// Das Verschluesseln scheiterte; bei den Groessen, die der Editor annimmt,
    /// kommt das nicht vor.
    Verschluesselung,
}

impl Tresorfehler {
    /// Der Satz fuer die Statuszeile.
    pub fn meldung(&self) -> String {
        match self {
            Tresorfehler::KeinZufall(grund) => {
                format!("Das System liefert keinen Zufallswert: {grund}")
            }
            Tresorfehler::Ableitung(grund) => {
                format!("Der Schlüssel lässt sich nicht ableiten: {grund}")
            }
            Tresorfehler::Verschluesselung => "Der Inhalt lässt sich nicht verschlüsseln".into(),
        }
    }
}

/// Warum [`oeffnen`] keinen Klartext liefert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Oeffnungsfehler {
    /// Der Kopf laesst sich nicht lesen; abgeleitet wurde nicht.
    KopfBeschaedigt(Kopfschaden),
    /// Die Pruefung des Verfahrens weist ab: die PIN ist falsch, oder ein Byte
    /// der Datei ist veraendert. Beides ist nicht zu trennen.
    PinFalschOderVeraendert,
    /// Das System versagte unter der Ableitung; ueber PIN und Datei ist damit
    /// nichts gesagt.
    System(Tresorfehler),
}

impl Oeffnungsfehler {
    /// Der Satz fuer die Statuszeile. Fuer eine falsche PIN und eine
    /// veraenderte Datei ist es derselbe.
    pub fn meldung(&self) -> String {
        match self {
            Oeffnungsfehler::PinFalschOderVeraendert => "PIN falsch oder Datei verändert".into(),
            Oeffnungsfehler::KopfBeschaedigt(schaden) => {
                let grund = match schaden {
                    Kopfschaden::FalscheKennung => "die Kennung am Anfang fehlt".to_string(),
                    Kopfschaden::Abgeschnitten => "die Datei ist abgeschnitten".to_string(),
                    Kopfschaden::UnbekannteVersion(v) => format!("unbekannte Formatversion {v}"),
                    Kopfschaden::UnbekannteAbleitung(a) => format!("unbekannte Ableitung {a}"),
                    Kopfschaden::UngueltigeParameter => {
                        "die Parameter der Ableitung sind ungültig".to_string()
                    }
                };
                format!("Der Kopf der Datei ist beschädigt: {grund}")
            }
            Oeffnungsfehler::System(fehler) => fehler.meldung(),
        }
    }
}

/// Eine geoeffnete Datei: der Klartext und der Schluessel, mit dem sie
/// geoeffnet wurde. Der Schluessel ist der, den eine gewoehnliche Sicherung
/// weiterverwendet.
#[derive(Debug)]
#[must_use = "traegt den Klartext und den Schluessel, mit dem jede Sicherung verschliesst"]
pub struct Geoeffnet {
    /// Der entschluesselte Inhalt.
    pub klartext: Vec<u8>,
    /// Der abgeleitete Schluessel samt Salz und Parametern aus dem Kopf.
    pub schluessel: Schluessel,
}

/// Leitet den Schluessel aus PIN, Salz und Parametern ab. Kostet die Zeit, die
/// die Parameter verlangen, bei [`Parameter::DES_CODES`] auf dem
/// Referenzgeraet rund eine halbe Sekunde; gehoert deshalb auf einen Arbeitsfaden.
#[must_use = "die Ableitung ist die ganze Wirkung, und sie ist teuer"]
pub fn schluessel_ableiten(
    pin: &Pin,
    salz: &[u8; SALZLAENGE],
    parameter: Parameter,
) -> Result<Schluessel, Tresorfehler> {
    let params = parameter
        .argon2()
        .map_err(|fehler| Tresorfehler::Ableitung(fehler.to_string()))?;
    let mut schluessel = [0u8; SCHLUESSELLAENGE];
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(pin.bytes(), salz, &mut schluessel)
        .map_err(|fehler| Tresorfehler::Ableitung(fehler.to_string()))?;
    Ok(Schluessel {
        schluessel,
        salz: *salz,
        parameter,
    })
}

/// Ein neuer Schluessel fuer eine neue PIN: frisches Salz, die Parameter des
/// Codes. Gerufen beim Festlegen und beim Aendern der PIN und sonst nie.
#[must_use = "die Ableitung ist die ganze Wirkung, und sie ist teuer"]
pub fn neuer_schluessel(pin: &Pin) -> Result<Schluessel, Tresorfehler> {
    let salz = <[u8; SALZLAENGE]>::try_generate()
        .map_err(|fehler| Tresorfehler::KeinZufall(fehler.to_string()))?;
    schluessel_ableiten(pin, &salz, Parameter::DES_CODES)
}

/// Verschliesst den Klartext mit dem gehaltenen Schluessel: frische Nonce,
/// Salz und Parameter aus dem Schluessel, der ganze Kopf als zusaetzliche
/// authentifizierte Daten. **Leitet nicht ab.** Liefert die ganze Datei, Kopf
/// und Chiffrat.
#[must_use = "die verschluesselten Bytes sind die ganze Wirkung; fallengelassen ist nichts gesichert"]
pub fn verschliessen(klartext: &[u8], schluessel: &Schluessel) -> Result<Vec<u8>, Tresorfehler> {
    let nonce = <[u8; NONCELAENGE]>::try_generate()
        .map_err(|fehler| Tresorfehler::KeinZufall(fehler.to_string()))?;
    let kopf = Kopf {
        version: FORMATVERSION,
        ableitung: ABLEITUNG_ARGON2ID,
        parameter: schluessel.parameter,
        salz: schluessel.salz,
        nonce,
    }
    .bytes();
    let verfahren = XChaCha20Poly1305::new(&Key::from(schluessel.schluessel));
    let chiffrat = verfahren
        .encrypt(
            &XNonce::from(nonce),
            Payload {
                msg: klartext,
                aad: &kopf,
            },
        )
        .map_err(|_| Tresorfehler::Verschluesselung)?;
    let mut datei = Vec::with_capacity(KOPFLAENGE + chiffrat.len());
    datei.extend_from_slice(&kopf);
    datei.extend_from_slice(&chiffrat);
    Ok(datei)
}

/// Oeffnet eine nicht leere `.secrets.txt` mit der PIN: liest den Kopf, leitet
/// mit dessen Salz und Parametern ab und entschluesselt. Schreibt nie; die
/// Bytes der Datei sind nach jedem Ausgang dieselben.
///
/// Eine leere Datei ist hier ein [`Kopfschaden::Abgeschnitten`]; dass sie als
/// neue Datei gilt, die nach einer neuen PIN fragt, entscheidet der Rufer,
/// bevor er hierher kommt.
#[must_use = "traegt den Klartext oder die eine Meldung; fallengelassen oeffnet sich nichts"]
pub fn oeffnen(bytes: &[u8], pin: &Pin) -> Result<Geoeffnet, Oeffnungsfehler> {
    let kopf = Kopf::lesen(bytes).map_err(Oeffnungsfehler::KopfBeschaedigt)?;
    let schluessel =
        schluessel_ableiten(pin, &kopf.salz, kopf.parameter).map_err(Oeffnungsfehler::System)?;
    let verfahren = XChaCha20Poly1305::new(&Key::from(schluessel.schluessel));
    let klartext = verfahren
        .decrypt(
            &XNonce::from(kopf.nonce),
            Payload {
                msg: &bytes[KOPFLAENGE..],
                aad: &bytes[..KOPFLAENGE],
            },
        )
        .map_err(|_| Oeffnungsfehler::PinFalschOderVeraendert)?;
    Ok(Geoeffnet {
        klartext,
        schluessel,
    })
}

#[cfg(test)]
mod proben {
    use super::*;

    /// Kleine Parameter, damit die Proben im Profil `dev` nicht auf die halbe
    /// Sekunde der echten warten.
    fn klein() -> Parameter {
        Parameter::neu(64, 1, 1).expect("die kleinen Parameter sind gueltig")
    }

    #[test]
    fn die_parameter_des_codes_liegen_in_den_grenzen() {
        let p = Parameter::DES_CODES;
        assert_eq!(
            Parameter::neu(p.speicher_kib, p.durchlaeufe, p.parallelitaet),
            Some(p)
        );
    }

    #[test]
    fn der_kopf_ist_sechzig_byte_lang_und_liest_sich_zurueck() {
        assert_eq!(KOPFLAENGE, 60);
        let kopf = Kopf {
            version: FORMATVERSION,
            ableitung: ABLEITUNG_ARGON2ID,
            parameter: klein(),
            salz: [7; SALZLAENGE],
            nonce: [9; NONCELAENGE],
        };
        assert_eq!(Kopf::lesen(&kopf.bytes()), Ok(kopf));
    }

    #[test]
    fn debug_zeigt_weder_pin_noch_schluesselbytes() {
        let pin = Pin::aus_eingabe("0417").expect("vier Ziffern");
        assert_eq!(format!("{pin:?}"), "Pin(****)");
        let schluessel = schluessel_ableiten(&pin, &[1; SALZLAENGE], klein()).expect("Ableitung");
        let text = format!("{schluessel:?}");
        assert!(!text.contains(&format!("{:?}", schluessel.schluessel)));
        assert!(text.contains("<32 Byte>"));
    }
}
