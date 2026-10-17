//! Ein Pfad in einem Archiv, und die eine Regel, welcher Eintragsname einer
//! ist.
//!
//! **Jede Rechnung an einem [`Innenpfad`] ist rein lexikalisch und stellt
//! keinen Systemaufruf**, wie an einem `Fernpfad`: Aufwaerts laesst die
//! letzte Komponente fallen, Eintreten haengt einen Namen an. Die Wurzel ist
//! der leere Pfad, und kein Pfad beginnt oder endet mit `/`.
//!
//! # Die eine Namensregel fuer alle drei Formate
//!
//! [`Innenpfad::aus_archivname`] entscheidet fuer Zip, tar und gz, welcher
//! Eintragsname ins Inhaltsverzeichnis kommt. Ein Name, den sie abweist, steht
//! nicht im Fenster, und keine Auswahl erreicht ihn; damit entsteht er beim
//! Herauskopieren nie. Das ist der erste der zwei Wege aus einem Zielordner
//! heraus, die `operation::entpacken` fuer das Entpacken auf die Platte
//! versperrt (`../x`, `a/../../x`); der zweite, ueber eine Verknuepfung im
//! selben Archiv, ist eine Frage an alle Eintraege zugleich und steht deshalb
//! im Sammler des Inhaltsverzeichnisses (E4 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).

use std::fmt;

use crate::fern::ist_einzelner_name;

/// Ein relativer Pfad in einem Archiv: UTF-8, `/` als Trenner, ohne leere
/// Glieder, ohne `.` und `..` und ohne NUL. Die Wurzel ist der leere Pfad.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Innenpfad(String);

impl Innenpfad {
    /// Die Wurzel des Archivs.
    #[must_use]
    pub fn wurzel() -> Self {
        Innenpfad(String::new())
    }

    /// Der Pfad zu einem Eintragsnamen, wie ein Archiv ihn fuehrt, oder `None`,
    /// wenn der Name unsicher ist.
    ///
    /// Abgestreift werden fuehrende `/` (aus `/etc/passwd` wird
    /// `etc/passwd`, wie es `enclosed_name` beim Entpacken tut), danach ein
    /// fuehrendes `./` und ein abschliessendes `/`, das einen Ordner
    /// kennzeichnet. Was dann bleibt, muss aus einzelnen Namen nach
    /// [`ist_einzelner_name`] bestehen: kein leeres Glied, kein `.`, kein `..`
    /// und kein NUL. Ein Name, der nach dem Abstreifen leer ist (`./`, `/`),
    /// ist die [`Innenpfad::wurzel`]; der Sammler laesst ihn ohne Zaehlung aus,
    /// denn er nennt den Ordner, in dem das Fenster ohnehin steht.
    ///
    /// Ein Backslash ist ein Zeichen des Namens und kein Trenner.
    #[must_use]
    pub fn aus_archivname(name: &str) -> Option<Self> {
        let ohne_wurzel = name.trim_start_matches('/');
        let ohne_punkt = ohne_wurzel.strip_prefix("./").unwrap_or(ohne_wurzel);
        let rumpf = ohne_punkt.strip_suffix('/').unwrap_or(ohne_punkt);
        if rumpf.is_empty() || rumpf == "." {
            return Some(Self::wurzel());
        }
        if rumpf.split('/').all(ist_einzelner_name) {
            Some(Innenpfad(rumpf.to_owned()))
        } else {
            None
        }
    }

    /// Der Pfad mit `name` darunter, oder `None`, wenn `name` keine einzelne
    /// Komponente ist.
    #[must_use]
    pub fn verbinden(&self, name: &str) -> Option<Self> {
        if !ist_einzelner_name(name) {
            return None;
        }
        if self.ist_wurzel() {
            Some(Innenpfad(name.to_owned()))
        } else {
            Some(Innenpfad(format!("{}/{name}", self.0)))
        }
    }

    /// Der uebergeordnete Pfad, oder `None` an der Wurzel.
    #[must_use]
    pub fn aufwaerts(&self) -> Option<Self> {
        if self.ist_wurzel() {
            return None;
        }
        Some(match self.0.rfind('/') {
            Some(schnitt) => Innenpfad(self.0[..schnitt].to_owned()),
            None => Self::wurzel(),
        })
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
        self.0.is_empty()
    }

    /// Ob `self` echt unter `vorfahr` liegt, rein lexikalisch.
    ///
    /// Jeder Pfad ausser der Wurzel liegt unter der Wurzel; kein Pfad liegt
    /// unter sich selbst.
    #[must_use]
    pub fn liegt_unter(&self, vorfahr: &Innenpfad) -> bool {
        if vorfahr.ist_wurzel() {
            return !self.ist_wurzel();
        }
        self.0.len() > vorfahr.0.len()
            && self.0.starts_with(&vorfahr.0)
            && self.0.as_bytes()[vorfahr.0.len()] == b'/'
    }

    /// Der Pfad als Text, ohne fuehrenden `/`; die Wurzel ist leer.
    #[must_use]
    pub fn als_text(&self) -> &str {
        &self.0
    }
}

/// Zeigt den Pfad mit fuehrendem `/`, die Wurzel als `/`, wie der
/// Fenstertitel ihn hinter den Doppelpunkt setzt.
impl fmt::Display for Innenpfad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.0)
    }
}
