//! Die Adresse einer sftp-Verbindung: Rechner, Port, Konto und Startordner.
//!
//! **Eine reine Funktion fuer das Blatt „Verbinden“ und fuer den Klick**
//! ([`fernziel`]), damit beide dasselbe annehmen, wie
//! `heimordner::verbindung::adresse_pruefen` es fuer jede Adresse tut
//! (Entscheidung 7 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`). Die Adresse
//! lautet `rechner[:port][/pfad]`, eine IPv6-Adresse mit Port als
//! `[adresse]:port`; das Konto kommt aus dem eigenen Feld des Eintrags.
//!
//! | Adresse | Startordner |
//! |---|---|
//! | `ssh.strato.de` oder `ssh.strato.de:22` | das Anmeldeverzeichnis |
//! | `ssh.strato.de/~/htdocs` | `htdocs` unter dem Anmeldeverzeichnis |
//! | `ssh.strato.de/htdocs` | `/htdocs`, absolut, wie in einer `sftp://`-URL |
//!
//! **Abgewiesen wird, was `ssh` anders laese, als der Nutzer es meint:** ein
//! leerer Rechner, ein Rechner mit Leerraum, mit einem Steuerzeichen, mit
//! fuehrendem `-` (das `ssh` als Option laese, stuende der Rechner nicht hinter
//! `--`) oder mit `@` (das `ssh` als Konto vor dem Rechner laese, an dem Konto
//! des Eintrags vorbei), und ein Port ausserhalb von 1 bis 65535.
//!
//! Aufgeloest wird der Startordner hier nicht: das tut der Zugang auf dem
//! Server mit `REALPATH` und `STAT` (`super::zugang`).

use std::fmt;

use super::pfad::Fernpfad;

/// Wohin eine Verbindung auf dem Server zeigt, wenn sie steht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Startpfad {
    /// Das Anmeldeverzeichnis, `REALPATH(".")`.
    Anmeldeverzeichnis,
    /// Ein Pfad unter dem Anmeldeverzeichnis, ohne fuehrendes `/`, so wie er
    /// hinter `~/` stand; der Server loest ihn gegen das Anmeldeverzeichnis
    /// auf, denn dort steht sein Arbeitsordner.
    UnterAnmeldeverzeichnis(String),
    /// Ein absoluter Pfad, lexikalisch bereinigt.
    Absolut(Fernpfad),
}

/// Was einen Server fuer KRK ausmacht: Rechner, Port und Konto.
///
/// **Ueber diesen Wert entscheidet sich, ob zwei Verbindungen einen Zugang
/// teilen und ob zwei ferne Orte gleich sind** (Entscheidung 1 des Plans).
/// Der Rechner steht klein geschrieben, weil `SSH.Strato.de` und
/// `ssh.strato.de` derselbe Rechner sind; Port und Konto stehen, wie sie
/// angegeben sind, und fehlen sie, gilt, was `ssh` aus `~/.ssh/config` nimmt.
/// `ssh.strato.de` und `ssh.strato.de:22` sind deshalb zwei Kennungen, obwohl
/// sie meist denselben Server nennen: ob die Konfiguration einen anderen Port
/// setzt, weiss allein `ssh`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Serverkennung {
    rechner: String,
    port: Option<u16>,
    konto: Option<String>,
}

impl Serverkennung {
    /// Die Kennung zu Rechner, Port und Konto; der Rechner wird klein
    /// geschrieben.
    #[must_use]
    pub fn neu(rechner: &str, port: Option<u16>, konto: Option<&str>) -> Self {
        Serverkennung {
            rechner: rechner.to_lowercase(),
            port,
            konto: konto.map(str::to_owned),
        }
    }

    /// Der Rechner, klein geschrieben.
    #[must_use]
    pub fn rechner(&self) -> &str {
        &self.rechner
    }

    /// Der Port, wenn die Adresse einen nennt.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Das Konto, wenn der Eintrag eines nennt.
    #[must_use]
    pub fn konto(&self) -> Option<&str> {
        self.konto.as_deref()
    }
}

impl fmt::Display for Serverkennung {
    /// `konto@rechner` oder `konto@rechner:port`, ohne Konto allein der
    /// Rechner; eine IPv6-Adresse mit Port in eckigen Klammern.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(konto) = &self.konto {
            write!(f, "{konto}@")?;
        }
        match self.port {
            Some(port) if self.rechner.contains(':') => write!(f, "[{}]:{port}", self.rechner),
            Some(port) => write!(f, "{}:{port}", self.rechner),
            None => f.write_str(&self.rechner),
        }
    }
}

/// Das Ziel einer sftp-Verbindung, zerlegt und geprueft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fernziel {
    /// Der Rechner, wie er in der Adresse steht, ohne eckige Klammern.
    pub rechner: String,
    /// Der Port, wenn die Adresse einen nennt.
    pub port: Option<u16>,
    /// Das Konto ohne Leerraum an den Enden, oder `None`, wenn es leer ist.
    pub konto: Option<String>,
    /// Wohin die Verbindung zeigt, wenn sie steht.
    pub startpfad: Startpfad,
}

impl Fernziel {
    /// Die Kennung des Servers.
    #[must_use]
    pub fn kennung(&self) -> Serverkennung {
        Serverkennung::neu(&self.rechner, self.port, self.konto.as_deref())
    }
}

/// Warum aus einer Adresse kein [`Fernziel`] entsteht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Adressfehler {
    /// Vor dem Port oder dem Pfad steht kein Rechner.
    RechnerLeer,
    /// Der Rechner traegt Leerraum, ein Steuerzeichen oder `@`, beginnt mit
    /// `-` oder steht in einer unvollstaendigen eckigen Klammer.
    RechnerUngueltig(String),
    /// Der Port ist keine Zahl von 1 bis 65535.
    PortUngueltig(String),
}

/// Das Ziel aus Adresse und Konto eines Eintrags, Leerraum an den Enden
/// beider abgeschnitten.
///
/// Geprueft wird hier allein die Form; ob `://` in der Adresse steht oder sie
/// leer ist, fragt vorher `heimordner::verbindung::adresse_pruefen`.
pub fn fernziel(adresse: &str, konto: &str) -> Result<Fernziel, Adressfehler> {
    let adresse = adresse.trim();
    let (vorn, pfad) = match adresse.split_once('/') {
        Some((vorn, pfad)) => (vorn, Some(pfad)),
        None => (adresse, None),
    };
    let (rechner, port) = rechner_und_port(vorn)?;
    if rechner.is_empty() {
        return Err(Adressfehler::RechnerLeer);
    }
    if rechner.starts_with('-')
        || rechner.contains('@')
        || rechner
            .chars()
            .any(|zeichen| zeichen.is_whitespace() || zeichen.is_control())
    {
        return Err(Adressfehler::RechnerUngueltig(rechner.to_owned()));
    }
    let port = match port {
        None => None,
        Some(text) => Some(
            text.parse::<u16>()
                .ok()
                .filter(|port| *port > 0)
                .ok_or_else(|| Adressfehler::PortUngueltig(text.to_owned()))?,
        ),
    };
    let konto = konto.trim();
    Ok(Fernziel {
        rechner: rechner.to_owned(),
        port,
        konto: (!konto.is_empty()).then(|| konto.to_owned()),
        startpfad: startpfad(pfad),
    })
}

/// Rechner und Port aus dem Teil vor dem ersten `/`.
///
/// `[adresse]` und `[adresse]:port` fuer IPv6; ohne Klammer trennt genau ein
/// `:` den Port ab, und mehr als einer heisst, dass dort eine IPv6-Adresse
/// ohne Port steht.
fn rechner_und_port(vorn: &str) -> Result<(&str, Option<&str>), Adressfehler> {
    if let Some(rest) = vorn.strip_prefix('[') {
        let Some((rechner, danach)) = rest.split_once(']') else {
            return Err(Adressfehler::RechnerUngueltig(vorn.to_owned()));
        };
        return match danach {
            "" => Ok((rechner, None)),
            _ => match danach.strip_prefix(':') {
                Some(port) => Ok((rechner, Some(port))),
                None => Err(Adressfehler::RechnerUngueltig(vorn.to_owned())),
            },
        };
    }
    match vorn.matches(':').count() {
        0 => Ok((vorn, None)),
        1 => {
            let (rechner, port) = vorn.split_once(':').unwrap_or((vorn, ""));
            Ok((rechner, Some(port)))
        }
        _ => Ok((vorn, None)),
    }
}

/// Der Startpfad aus dem Teil hinter dem ersten `/`: nichts, `~` oder `~/`
/// ist das Anmeldeverzeichnis, `~/rest` liegt darunter, alles andere ist
/// absolut.
fn startpfad(pfad: Option<&str>) -> Startpfad {
    let Some(pfad) = pfad else {
        return Startpfad::Anmeldeverzeichnis;
    };
    if pfad.is_empty() || pfad == "~" {
        return Startpfad::Anmeldeverzeichnis;
    }
    match pfad.strip_prefix("~/") {
        Some(rest) => {
            let rest = rest.trim_matches('/');
            if rest.is_empty() {
                Startpfad::Anmeldeverzeichnis
            } else {
                Startpfad::UnterAnmeldeverzeichnis(rest.to_owned())
            }
        }
        // `aus_text` nimmt jeden Text mit fuehrendem `/` an, und der steht
        // hier immer davor.
        None => Startpfad::Absolut(
            Fernpfad::aus_text(&format!("/{pfad}")).unwrap_or_else(Fernpfad::wurzel),
        ),
    }
}
