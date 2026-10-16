//! Die Protokolle einer Verbindung aus `secrets.txt` und das Ziel, das aus
//! Protokoll, Adresse und Konto entsteht.
//!
//! Ein Eintrag in `secrets.txt` nennt ein Protokoll als Text (`protocol: smb`);
//! dieses Modul sagt, welche Texte KRK kennt, auf welchem Weg eine Verbindung
//! damit hergestellt wird und welche URL dazu gehoert (Entscheidung 10 des
//! Plans `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//! **Reine Funktionen ohne Systemaufruf**: das Einhaengen und die Uebergabe an
//! das System stehen in `krk-ui`, und sie fragen hier nur, was sie tun sollen.
//!
//! # Zwei Wege
//!
//! | Protokoll | Weg | URL |
//! |---|---|---|
//! | `smb`, `afp`, `nfs`, `ftp` | einhaengen | `<protokoll>://<adresse>` |
//! | `webdav` | einhaengen | `http://<adresse>` |
//! | `webdavs` | einhaengen | `https://<adresse>` |
//! | `http`, `https` | uebergeben | `<protokoll>://<adresse>` |
//! | `ssh`, `sftp`, `vnc` | uebergeben | `<protokoll>://[<konto>@]<adresse>` |
//!
//! **Beim Einhaengen steht das Konto nicht in der URL**, sondern reist als
//! eigener Wert neben ihr ([`Verbindungsziel::Einhaengen`]), weil die Bindung
//! an NetFS Konto und Zugangsdaten als eigene Argumente nimmt. **Beim
//! Uebergeben steht es prozentkodiert in der URL**, bei `http` und `https`
//! nie, weil ein Konto in einer Webadresse dort nichts oeffnet, was der Nutzer
//! erwartet.
//!
//! # Was dieses Modul nicht kennt
//!
//! Die Zugangsdaten des Eintrags. Kein Typ hier nimmt sie an und kein Wert
//! traegt sie; sie bleiben im Typ von `heimordner::eintraege::geheimnisse`, und
//! die Probe `das_verbindungsziel_nennt_den_typ_der_zugangsdaten_nicht`
//! (`tests/heimordner.rs`) haelt am Code dieser Datei, dass er den Typ nicht
//! nennt. Kommentarzeilen liest sie nicht.
//!
//! Die Adresse wird nicht kodiert: sie steht so in der URL, wie der Nutzer sie
//! eingetragen hat, ohne Leerraum an den Enden. Eine Adresse mit Zeichen, die
//! keine URL traegt, weist erst der Weg in `krk-ui` ab, der die URL baut.

use crate::sprache::{Text, satz, text};

/// Ein Protokoll, das KRK fuer eine Verbindung kennt.
///
/// **Vollstaendig und ohne Auffangzweig** an jeder Stelle, die nach dem
/// Protokoll fragt; [`Protokoll::ALLE`] fuehrt jeden Wert, und
/// [`Protokoll::aus_text`] liest allein Werte daraus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protokoll {
    /// SMB, die Freigaben von Windows und macOS.
    Smb,
    /// AFP, die alten Freigaben von macOS.
    Afp,
    /// NFS.
    Nfs,
    /// WebDAV ueber `http`.
    Webdav,
    /// WebDAV ueber `https`.
    Webdavs,
    /// FTP, eingehaengt und nur lesend, wie macOS es kann.
    Ftp,
    /// Eine Webadresse ueber `http`, an den Browser uebergeben.
    Http,
    /// Eine Webadresse ueber `https`, an den Browser uebergeben.
    Https,
    /// SSH, an das Terminal uebergeben.
    Ssh,
    /// SFTP, an das Programm uebergeben, das das System dafuer nennt.
    Sftp,
    /// VNC, an die Bildschirmfreigabe uebergeben.
    Vnc,
}

/// Auf welchem Weg eine Verbindung hergestellt wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weg {
    /// KRK haengt die Freigabe selbst ein, ueber NetFS, mit Konto und
    /// Zugangsdaten als eigenen Argumenten.
    Einhaengen,
    /// KRK gibt die URL an das System, das das zustaendige Programm oeffnet;
    /// ohne Zugangsdaten.
    Uebergeben,
}

impl Protokoll {
    /// Jedes Protokoll, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Protokoll; 11] = [
        Protokoll::Smb,
        Protokoll::Afp,
        Protokoll::Nfs,
        Protokoll::Webdav,
        Protokoll::Webdavs,
        Protokoll::Ftp,
        Protokoll::Http,
        Protokoll::Https,
        Protokoll::Ssh,
        Protokoll::Sftp,
        Protokoll::Vnc,
    ];

    /// Der Name, wie er in `secrets.txt` steht: klein und ASCII.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Protokoll::Smb => "smb",
            Protokoll::Afp => "afp",
            Protokoll::Nfs => "nfs",
            Protokoll::Webdav => "webdav",
            Protokoll::Webdavs => "webdavs",
            Protokoll::Ftp => "ftp",
            Protokoll::Http => "http",
            Protokoll::Https => "https",
            Protokoll::Ssh => "ssh",
            Protokoll::Sftp => "sftp",
            Protokoll::Vnc => "vnc",
        }
    }

    /// Das Protokoll zu einem Text, ohne Ruecksicht auf Gross- und
    /// Kleinschreibung und ohne Leerraum an den Enden; `None` fuer jeden
    /// anderen Text, auch den leeren.
    #[must_use]
    pub fn aus_text(text: &str) -> Option<Protokoll> {
        let text = text.trim();
        Protokoll::ALLE
            .into_iter()
            .find(|protokoll| protokoll.name().eq_ignore_ascii_case(text))
    }

    /// Der Weg, auf dem eine Verbindung mit diesem Protokoll entsteht.
    #[must_use]
    pub fn weg(self) -> Weg {
        match self {
            Protokoll::Smb
            | Protokoll::Afp
            | Protokoll::Nfs
            | Protokoll::Webdav
            | Protokoll::Webdavs
            | Protokoll::Ftp => Weg::Einhaengen,
            Protokoll::Http
            | Protokoll::Https
            | Protokoll::Ssh
            | Protokoll::Sftp
            | Protokoll::Vnc => Weg::Uebergeben,
        }
    }

    /// Das Schema der URL. Fuer WebDAV ist es `http` oder `https` und nicht
    /// der Name des Protokolls, denn so nimmt NetFS eine WebDAV-Freigabe an.
    #[must_use]
    pub fn url_schema(self) -> &'static str {
        match self {
            Protokoll::Webdav => "http",
            Protokoll::Webdavs => "https",
            Protokoll::Smb
            | Protokoll::Afp
            | Protokoll::Nfs
            | Protokoll::Ftp
            | Protokoll::Http
            | Protokoll::Https
            | Protokoll::Ssh
            | Protokoll::Sftp
            | Protokoll::Vnc => self.name(),
        }
    }

    /// Ob ein Konto in die URL der Uebergabe gehoert: bei `ssh`, `sftp` und
    /// `vnc` ja, bei `http` und `https` nie, und beim Einhaengen steht es nie
    /// in der URL.
    fn konto_in_der_url(self) -> bool {
        match self {
            Protokoll::Ssh | Protokoll::Sftp | Protokoll::Vnc => true,
            Protokoll::Smb
            | Protokoll::Afp
            | Protokoll::Nfs
            | Protokoll::Webdav
            | Protokoll::Webdavs
            | Protokoll::Ftp
            | Protokoll::Http
            | Protokoll::Https => false,
        }
    }
}

/// Das Ziel einer Verbindung, fertig fuer den Weg in `krk-ui`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verbindungsziel {
    /// Einhaengen ueber NetFS: die URL ohne Konto, das Konto daneben, wenn der
    /// Eintrag eines nennt.
    Einhaengen {
        /// `<schema>://<adresse>`.
        url: String,
        /// Das Konto ohne Leerraum an den Enden, oder `None`, wenn es leer ist.
        konto: Option<String>,
    },
    /// Uebergabe an das System.
    Uebergeben {
        /// `<schema>://[<konto>@]<adresse>`, das Konto prozentkodiert.
        url: String,
    },
}

/// Warum aus einem Eintrag kein Ziel entsteht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zielfehler {
    /// Das Protokoll kennt KRK nicht; der Text, wie er im Eintrag steht, ohne
    /// Leerraum an den Enden.
    UnbekanntesProtokoll(String),
    /// Die Adresse ist leer.
    LeereAdresse,
    /// Die Adresse traegt schon ein Schema (`://`); das Schema kommt aus dem
    /// Protokoll.
    SchemaInDerAdresse,
}

impl Zielfehler {
    /// Der Satz fuer die Statuszeile und das Blatt; der Wortlaut steht in der
    /// Sprachtabelle (`crate::sprache`), hier je Wert sein Schluessel.
    #[must_use]
    pub fn meldung(&self) -> String {
        match self {
            Zielfehler::UnbekanntesProtokoll(protokoll) => {
                let bekannte: Vec<&str> = Protokoll::ALLE.iter().map(|p| p.name()).collect();
                satz(
                    Text::VerbindungProtokollUnbekannt,
                    &[("protokoll", protokoll), ("bekannte", &bekannte.join(", "))],
                )
            }
            Zielfehler::LeereAdresse => text(Text::VerbindungAdresseLeer).to_owned(),
            Zielfehler::SchemaInDerAdresse => text(Text::VerbindungAdresseMitSchema).to_owned(),
        }
    }
}

/// Das Protokoll zu einem eingegebenen Text, oder der Fehler, den die
/// Statuszeile zeigt. **Die eine Regel fuer das Blatt „Verbinden“ und fuer
/// den Klick auf eine Verbindung**, damit beide dasselbe annehmen.
pub fn protokoll_pruefen(text: &str) -> Result<Protokoll, Zielfehler> {
    Protokoll::aus_text(text).ok_or_else(|| Zielfehler::UnbekanntesProtokoll(text.trim().into()))
}

/// Die Adresse ohne Leerraum an den Enden, wenn sie eine ist: nicht leer und
/// ohne eigenes Schema. Dieselbe Regel fuer Blatt und Klick wie
/// [`protokoll_pruefen`].
pub fn adresse_pruefen(adresse: &str) -> Result<&str, Zielfehler> {
    let adresse = adresse.trim();
    if adresse.is_empty() {
        Err(Zielfehler::LeereAdresse)
    } else if adresse.contains("://") {
        Err(Zielfehler::SchemaInDerAdresse)
    } else {
        Ok(adresse)
    }
}

/// Das Ziel aus Protokoll, Adresse und Konto eines Eintrags, wie sie in
/// `secrets.txt` stehen.
///
/// Geprueft wird zuerst das Protokoll, dann die Adresse. Das Konto ist frei
/// und darf leer sein; ohne Leerraum an den Enden genommen.
pub fn verbindungsziel(
    protokoll: &str,
    adresse: &str,
    konto: &str,
) -> Result<Verbindungsziel, Zielfehler> {
    let protokoll = protokoll_pruefen(protokoll)?;
    let adresse = adresse_pruefen(adresse)?;
    let konto = konto.trim();
    let schema = protokoll.url_schema();
    Ok(match protokoll.weg() {
        Weg::Einhaengen => Verbindungsziel::Einhaengen {
            url: format!("{schema}://{adresse}"),
            konto: (!konto.is_empty()).then(|| konto.to_owned()),
        },
        Weg::Uebergeben if protokoll.konto_in_der_url() && !konto.is_empty() => {
            Verbindungsziel::Uebergeben {
                url: format!("{schema}://{}@{adresse}", prozentkodiert(konto)),
            }
        }
        Weg::Uebergeben => Verbindungsziel::Uebergeben {
            url: format!("{schema}://{adresse}"),
        },
    })
}

/// Kodiert jedes Byte ausser den unreservierten Zeichen aus RFC 3986
/// (Buchstaben und Ziffern in ASCII, `-`, `.`, `_`, `~`) als `%XX`, also auch
/// `@`, `:`, Leerzeichen und jedes Byte eines Zeichens jenseits von ASCII.
fn prozentkodiert(text: &str) -> String {
    let mut kodiert = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            kodiert.push(char::from(byte));
        } else {
            kodiert.push_str(&format!("%{byte:02X}"));
        }
    }
    kodiert
}
