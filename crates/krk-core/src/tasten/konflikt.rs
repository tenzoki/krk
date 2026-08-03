//! Doppelt vergebene Kombinationen, und wie KRK sie benennt.
//!
//! C3 des Specs verlangt: "Belegt der Nutzer eine Kombination, die bereits
//! einer anderen Funktion gehoert, meldet KRK den Konflikt und **nennt die
//! andere Funktion**, statt die Belegung stillschweigend zu ueberschreiben.
//! Mehrere Kombinationen auf derselben Funktion sind kein Konflikt."
//!
//! Beide Haelften dieses Satzes stehen hier. [`Konflikt`] traegt die Namen
//! beider beteiligten Funktionen, weil eine Meldung, die nur die Kombination
//! nennt, den Nutzer suchen laesst. Und ein Konflikt entsteht nur zwischen
//! **verschiedenen** Funktionen: dass F5 und `shift+cmd+k` beide das Kopieren
//! ausloesen, ist der ausgelieferte Normalfall und keine Kollision.

use std::fmt;

use super::parser::Kombination;

/// Eine Funktion, so wie eine Meldung sie benennt.
///
/// Die Kennung fuer die Datei, die Beschriftung fuer den Nutzer. Beide, weil
/// eine Meldung ohne Beschriftung nicht verstaendlich und eine ohne Kennung in
/// `keymap.toml` nicht auffindbar ist.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Funktionsname {
    /// Der maschinenlesbare Bezeichner aus der Belegungsdatei.
    pub kennung: String,
    /// Die deutsche Beschriftung fuer die Belegungsansicht.
    pub name: String,
}

impl Funktionsname {
    /// Ein Name aus Kennung und Beschriftung.
    pub fn neu(kennung: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            kennung: kennung.into(),
            name: name.into(),
        }
    }
}

impl fmt::Display for Funktionsname {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(ausgabe, "\"{}\" ({})", self.name, self.kennung)
    }
}

/// Eine Kombination, die zwei verschiedene Funktionen beanspruchen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Konflikt {
    /// Die umstrittene Kombination.
    pub kombination: Kombination,
    /// Die Funktion, die sie bereits traegt.
    pub andere: Funktionsname,
    /// Die Funktion, die sie bekommen sollte.
    pub bewerber: Funktionsname,
}

impl fmt::Display for Konflikt {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            ausgabe,
            "die Kombination {} gehoert schon der Funktion {} und laesst sich nicht \
             zusaetzlich der Funktion {} zuweisen",
            self.kombination, self.andere, self.bewerber
        )
    }
}

impl std::error::Error for Konflikt {}
