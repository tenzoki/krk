//! Ein Pfad auf einem Server: absolut, POSIX, UTF-8.
//!
//! **Jede Rechnung an einem `Fernpfad` ist rein lexikalisch und stellt keine
//! Anfrage.** Aufwaerts heisst: die letzte Komponente fallen lassen;
//! Eintreten heisst: einen Namen anhaengen. Ob am Ergebnis ein Ordner steht,
//! sagt erst der Lesevorgang, der ihn oeffnet. Damit beantwortet ein Pfad die
//! Fragen der Navigation ohne Systemaufruf und ohne Netz, wie
//! `verzeichnis::aufwaerts` sie lokal am `PathBuf` beantwortet.
//!
//! Lexikalisch heisst auch: `..` nimmt die vorige Komponente weg, ohne zu
//! fragen, ob sie eine Verknuepfung ist. Auf dem Server kann `a/link/..` damit
//! woanders hinfuehren als `a`. Das ist dieselbe Entscheidung, die eine
//! `sftp://`-URL trifft, und sie gilt allein fuer Texte, die der Nutzer
//! eintippt; KRK selbst baut Pfade nur ueber [`Fernpfad::verbinden`], das
//! weder `.` noch `..` annimmt.
//!
//! UTF-8 ist eine Einschraenkung und keine Eigenschaft des Protokolls: SFTP
//! traegt Bytes. Ein Name, der kein UTF-8 ist, laesst sich deshalb nicht
//! ansteuern; der Leser eines Ordners ueberspringt ihn und sagt, wie viele
//! (Entscheidung 8 des Plans).

use std::fmt;

/// Ein absoluter Pfad auf einem Server, lexikalisch bereinigt: er beginnt mit
/// `/`, traegt kein `//`, kein `.`, kein `..` und endet nicht auf `/`, ausser
/// er ist die Wurzel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fernpfad(String);

impl Fernpfad {
    /// Die Wurzel `/`.
    #[must_use]
    pub fn wurzel() -> Self {
        Fernpfad("/".to_owned())
    }

    /// Ein Pfad aus einem Text, oder `None`, wenn der Text nicht mit `/`
    /// beginnt. `.` faellt weg, `..` nimmt die vorige Komponente weg und
    /// endet an der Wurzel, mehrfache `/` fallen zusammen.
    #[must_use]
    pub fn aus_text(text: &str) -> Option<Self> {
        if !text.starts_with('/') {
            return None;
        }
        let mut teile: Vec<&str> = Vec::new();
        for teil in text.split('/') {
            match teil {
                "" | "." => {}
                ".." => {
                    teile.pop();
                }
                name => teile.push(name),
            }
        }
        Some(Fernpfad(format!("/{}", teile.join("/"))))
    }

    /// Der Pfad mit `name` darunter, oder `None`, wenn `name` keine einzelne
    /// Komponente ist ([`ist_einzelner_name`]).
    #[must_use]
    pub fn verbinden(&self, name: &str) -> Option<Self> {
        if !ist_einzelner_name(name) {
            return None;
        }
        if self.ist_wurzel() {
            Some(Fernpfad(format!("/{name}")))
        } else {
            Some(Fernpfad(format!("{}/{name}", self.0)))
        }
    }

    /// Der uebergeordnete Pfad, oder `None` an der Wurzel.
    #[must_use]
    pub fn aufwaerts(&self) -> Option<Self> {
        if self.ist_wurzel() {
            return None;
        }
        let schnitt = self.0.rfind('/').unwrap_or(0);
        if schnitt == 0 {
            Some(Self::wurzel())
        } else {
            Some(Fernpfad(self.0[..schnitt].to_owned()))
        }
    }

    /// Die letzte Komponente, oder `None` an der Wurzel.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        if self.ist_wurzel() {
            None
        } else {
            self.0.rsplit('/').next()
        }
    }

    /// Ob der Pfad die Wurzel ist.
    #[must_use]
    pub fn ist_wurzel(&self) -> bool {
        self.0 == "/"
    }

    /// Der Pfad als Text.
    #[must_use]
    pub fn als_text(&self) -> &str {
        &self.0
    }
}

/// Ob `name` eine einzelne Komponente eines Pfads auf einem Server ist: nicht
/// leer, nicht `.` oder `..`, ohne `/` und ohne NUL.
///
/// **Die eine Namensregel fuer ferne Namen.** [`Fernpfad::verbinden`] fragt
/// sie, bevor es einen Pfad baut, und der Leser eines fernen Ordners
/// (`verzeichnis::leser::fern_eintrag`) fragt sie fuer jeden Namen aus
/// `READDIR`; ein Name, den sie abweist, laesst sich nicht ansteuern und
/// steht deshalb nicht in der Liste. NUL weist sie ab, weil SFTP einen Pfad
/// mit seiner Laenge traegt, der Server ihn aber als C-Zeichenkette an sein
/// System gibt: dort endete der Name am NUL, und die Anfrage meinte einen
/// anderen Eintrag als den der Liste.
#[must_use]
pub fn ist_einzelner_name(name: &str) -> bool {
    !(name.is_empty() || name == "." || name == ".." || name.contains(['/', '\0']))
}

impl fmt::Display for Fernpfad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
