//! Ein Verzeichniseintrag, wie ihn der Leser liefert.
//!
//! Der Eintrag wird beim Lesen einmal gefuellt und danach nicht mehr
//! veraendert. Insbesondere die beiden Sortierschluessel entstehen genau hier
//! und nicht bei jedem Sortierschritt: bei 100.000 Eintraegen liefe die
//! sprachsensitive Kollation sonst rund 1,7 Millionen Mal statt 200.000 Mal.
//! Wie der Schluessel gebaut wird, steht in [`super::kollation`].

use std::time::SystemTime;

use super::kollation;
use super::sys::RohEintrag;

/// Die Art eines Eintrags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Ein Kollationsschluessel des vollen Namens: bytweise verglichen ergibt
    /// er dieselbe Reihenfolge wie der sprachsensitive Vergleich der Namen.
    /// Zwei Namen, die sich nur in der Schreibung unterscheiden, haben
    /// verschiedene Schluessel, damit die Ordnung total ist und zwei Laeufe
    /// dieselbe Reihenfolge ergeben.
    pub sortierschluessel: Box<[u8]>,
    /// Der einmal berechnete Schluessel fuer die Sortierung nach Typ.
    ///
    /// Derselbe Aufbau, gebildet ueber [`Eintrag::endung`]. Er ist leer, wo es
    /// keine Endung gibt, und der leere Schluessel steht vor jedem anderen:
    /// Eintraege ohne Endung stehen damit am Anfang ihrer Gruppe.
    pub endungsschluessel: Box<[u8]>,
    /// Die Groesse der Daten in Bytes. Ordner tragen 0.
    pub groesse: u64,
    /// Der Zeitpunkt der letzten Aenderung.
    pub geaendert: SystemTime,
    /// Wo in [`Eintrag::name`] die Endung beginnt, hinter ihrem Punkt.
    ///
    /// Der Versatz statt einer zweiten Zeichenkette: die Endung steht bereits
    /// im Namen, und ein `u32` kommt in der Luecke unter, die die Ausrichtung
    /// der Struktur ohnehin laesst. Er kostet damit kein Byte.
    endung_ab: u32,
    /// Ordner, Datei oder symbolische Verknuepfung.
    pub typ: Typ,
    /// Wahr, wenn der Name mit einem Punkt beginnt oder das Dateisystem den
    /// Eintrag als versteckt kennzeichnet.
    pub versteckt: bool,
}

impl Eintrag {
    /// Baut einen Eintrag aus seinen Bestandteilen und berechnet dabei die
    /// beiden Sortierschluessel und die Lage der Endung.
    ///
    /// Dies ist der einzige Weg zu einem Eintrag. Ein Eintrag, dessen
    /// Schluessel nicht zu seinem Namen passt, sortiert falsch, ohne dass es
    /// auffiele; die Struktur laesst sich deshalb nicht Feld fuer Feld
    /// zusammensetzen.
    pub fn neu(name: String, groesse: u64, geaendert: SystemTime, typ: Typ) -> Self {
        let versteckt = name.starts_with('.');
        Self::mit_versteckt(name, groesse, geaendert, typ, versteckt)
    }

    /// Wie [`Eintrag::neu`], aber mit dem Kennzeichen `versteckt` von aussen.
    ///
    /// Der Leser braucht diese Form, weil ein Eintrag auch ohne fuehrenden
    /// Punkt versteckt sein kann: das Dateisystem kennzeichnet ihn dann mit
    /// `UF_HIDDEN`.
    pub fn mit_versteckt(
        name: String,
        groesse: u64,
        geaendert: SystemTime,
        typ: Typ,
        versteckt: bool,
    ) -> Self {
        let endung_ab = endung_ab(&name);
        let endungsschluessel = kollation::schluessel(&name[endung_ab as usize..]);
        Self {
            sortierschluessel: kollation::schluessel(&name),
            endungsschluessel,
            name,
            groesse,
            geaendert,
            endung_ab,
            typ,
            versteckt,
        }
    }

    /// Die Dateiendung ohne ihren Punkt, und der Schluessel der Sortierung
    /// nach Typ.
    ///
    /// Leer, wo es keine gibt. Was als Endung zaehlt, steht bei [`endung_ab`].
    pub fn endung(&self) -> &str {
        &self.name[self.endung_ab as usize..]
    }

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
        Self::mit_versteckt(name, roh.groesse, roh.geaendert, roh.typ, versteckt)
    }
}

/// Wo im Namen die Endung beginnt, hinter ihrem Punkt.
///
/// Drei Festlegungen, alle so, wie der Finder den Namen liest:
///
/// - Der **letzte** Punkt trennt: `sicherung.tar.gz` hat die Endung `gz`.
/// - Ein **fuehrender** Punkt trennt nicht: `.gitignore` ist ein versteckter
///   Eintrag ohne Endung, keine Datei namens `` mit der Endung `gitignore`.
/// - Ohne Punkt gibt es keine Endung. Der Versatz ist dann die Laenge des
///   Namens, und [`Eintrag::endung`] liefert die leere Zeichenkette.
fn endung_ab(name: &str) -> u32 {
    match name.rfind('.') {
        // `punkt > 0` schliesst den fuehrenden Punkt aus. `punkt + 1` liegt
        // hinter dem Punkt und damit auf einer Zeichengrenze, weil ein Punkt in
        // UTF-8 ein einzelnes Byte ist.
        Some(punkt) if punkt > 0 => (punkt + 1) as u32,
        _ => name.len() as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eintrag(name: &str) -> Eintrag {
        Eintrag::neu(name.to_owned(), 0, SystemTime::UNIX_EPOCH, Typ::Datei)
    }

    #[test]
    fn der_eintrag_bleibt_so_gross_wie_bisher() {
        // Die Zusagen L3 und L10 decken das vollstaendige Lesen samt
        // Sortieren und haengen an dieser Zahl. Sie steht hier, damit ein
        // weiteres Feld eine sichtbare Entscheidung ist und keine Nebenwirkung.
        assert_eq!(size_of::<Eintrag>(), 88);
    }

    #[test]
    fn der_versatz_der_endung_kostet_kein_byte() {
        // Er liegt in der Luecke, die die Ausrichtung ohnehin laesst.
        struct OhneVersatz {
            _name: String,
            _sortierschluessel: Box<[u8]>,
            _endungsschluessel: Box<[u8]>,
            _groesse: u64,
            _geaendert: SystemTime,
            _typ: Typ,
            _versteckt: bool,
        }
        assert_eq!(size_of::<Eintrag>(), size_of::<OhneVersatz>());
    }

    #[test]
    fn der_schluessel_ordnet_sprachsensitiv() {
        assert!(eintrag("Äpfel").sortierschluessel < eintrag("Bäume").sortierschluessel);
    }

    #[test]
    fn schluessel_ordnet_ohne_ruecksicht_auf_grossschreibung() {
        assert!(eintrag("Alpha").sortierschluessel < eintrag("beta").sortierschluessel);
    }

    #[test]
    fn schluessel_trennt_gleiche_namen_verschiedener_schreibung() {
        assert_ne!(
            eintrag("Datei").sortierschluessel,
            eintrag("datei").sortierschluessel
        );
    }

    #[test]
    fn der_letzte_punkt_trennt_die_endung_ab() {
        assert_eq!(eintrag("bericht.txt").endung(), "txt");
        assert_eq!(eintrag("sicherung.tar.gz").endung(), "gz");
    }

    #[test]
    fn ohne_punkt_gibt_es_keine_endung() {
        assert_eq!(eintrag("Makefile").endung(), "");
        assert_eq!(eintrag("bericht.").endung(), "");
    }

    #[test]
    fn ein_fuehrender_punkt_ist_keine_endung() {
        assert_eq!(eintrag(".gitignore").endung(), "");
        // Aber der zweite Punkt trennt weiterhin.
        assert_eq!(eintrag(".datei.txt").endung(), "txt");
    }

    #[test]
    fn ein_name_mit_mehrbytezeichen_wird_nicht_mitten_im_zeichen_geteilt() {
        // Der Versatz muss auf einer Zeichengrenze liegen, sonst geraet
        // `endung` in Panik.
        assert_eq!(eintrag("Größe.txt").endung(), "txt");
        assert_eq!(eintrag("名前.日本").endung(), "日本");
        assert_eq!(eintrag("Größe").endung(), "");
    }

    #[test]
    fn der_endungsschluessel_gehoert_zur_endung() {
        assert_eq!(
            eintrag("bericht.txt").endungsschluessel,
            kollation::schluessel("txt")
        );
        assert!(eintrag("Makefile").endungsschluessel.is_empty());
    }

    #[test]
    fn ein_fuehrender_punkt_gilt_als_versteckt() {
        assert!(eintrag(".gitignore").versteckt);
        assert!(!eintrag("gitignore").versteckt);
    }
}
