//! Ein Verzeichniseintrag, wie ihn der Leser liefert.
//!
//! Der Eintrag wird beim Lesen einmal gefuellt und danach nicht mehr
//! veraendert. Insbesondere der Sortierschluessel entsteht genau hier und
//! nicht bei jedem Sortierschritt: bei 100.000 Eintraegen waere ein
//! zeichenweiser Vergleich des Namens der teuerste Einzelposten der
//! Zeitzusagen.

use std::time::SystemTime;

use super::sys::RohEintrag;

/// Die Art eines Eintrags.
///
/// Die Reihenfolge der Varianten ist zugleich die Reihenfolge, die eine
/// Sortierung nach Typ verwendet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Typ {
    /// Ein Verzeichnis.
    Ordner,
    /// Eine gewoehnliche Datei, und alles, was weder Ordner noch Verknuepfung
    /// ist (Geraetedatei, Fifo, Socket).
    Datei,
    /// Eine symbolische Verknuepfung. Der Leser folgt ihr nicht, er meldet die
    /// Verknuepfung selbst.
    Verknuepfung,
}

/// Ein gelesener Verzeichniseintrag.
#[derive(Debug, Clone)]
pub struct Eintrag {
    /// Der Name ohne Pfad.
    pub name: String,
    /// Der einmal berechnete Schluessel fuer die Sortierung nach Namen.
    ///
    /// Aufbau: der kleingeschriebene Name, ein Nullbyte als Trenner, dann der
    /// unveraenderte Name. Der erste Teil traegt den Vergleich ohne Ruecksicht
    /// auf Gross- und Kleinschreibung, der zweite macht die Ordnung total, so
    /// dass `Datei` und `datei` eine feste Reihenfolge haben. Ein Nullbyte
    /// kann in einem Dateinamen nicht vorkommen und taugt deshalb als Trenner.
    pub sortierschluessel: Box<[u8]>,
    /// Die Groesse der Daten in Bytes. Ordner tragen 0.
    pub groesse: u64,
    /// Der Zeitpunkt der letzten Aenderung.
    pub geaendert: SystemTime,
    /// Ordner, Datei oder symbolische Verknuepfung.
    pub typ: Typ,
    /// Wahr, wenn der Name mit einem Punkt beginnt oder das Dateisystem den
    /// Eintrag als versteckt kennzeichnet.
    pub versteckt: bool,
}

impl Eintrag {
    /// Wahr, wenn der Eintrag ein Verzeichnis ist.
    pub fn ist_ordner(&self) -> bool {
        self.typ == Typ::Ordner
    }

    /// Wahr, wenn der Eintrag eine symbolische Verknuepfung ist.
    pub fn ist_verknuepfung(&self) -> bool {
        self.typ == Typ::Verknuepfung
    }

    /// Baut den Eintrag aus dem, was die Systemschicht geliefert hat.
    pub(crate) fn aus_roh(roh: RohEintrag<'_>) -> Self {
        let name = roh.name.into_owned();
        let versteckt = roh.systemseitig_versteckt || name.starts_with('.');
        let sortierschluessel = sortierschluessel_bauen(&name);
        Self {
            name,
            sortierschluessel,
            groesse: roh.groesse,
            geaendert: roh.geaendert,
            typ: roh.typ,
            versteckt,
        }
    }
}

/// Berechnet den Sortierschluessel eines Namens.
///
/// Der Schluessel ordnet ohne Ruecksicht auf Gross- und Kleinschreibung. Eine
/// sprachsensitive Kollation (die `Aepfel` zwischen `Apfel` und `Banane`
/// einordnet) leistet er **nicht**; dafuer braeuchte es Kollationstabellen,
/// die dieser Schritt nicht mitbringt.
fn sortierschluessel_bauen(name: &str) -> Box<[u8]> {
    let mut schluessel = Vec::with_capacity(name.len() * 2 + 1);
    for zeichen in name.chars() {
        if zeichen.is_ascii() {
            schluessel.push(zeichen.to_ascii_lowercase() as u8);
        } else {
            let mut puffer = [0u8; 4];
            for klein in zeichen.to_lowercase() {
                schluessel.extend_from_slice(klein.encode_utf8(&mut puffer).as_bytes());
            }
        }
    }
    schluessel.push(0);
    schluessel.extend_from_slice(name.as_bytes());
    schluessel.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schluessel_ordnet_ohne_ruecksicht_auf_grossschreibung() {
        let a = sortierschluessel_bauen("Alpha");
        let b = sortierschluessel_bauen("beta");
        assert!(a < b);
    }

    #[test]
    fn schluessel_trennt_gleiche_namen_verschiedener_schreibung() {
        let gross = sortierschluessel_bauen("Datei");
        let klein = sortierschluessel_bauen("datei");
        assert_ne!(gross, klein);
    }
}
