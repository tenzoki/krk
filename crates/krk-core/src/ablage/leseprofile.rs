//! `readers.toml`: die zweite Ablagedatei, die der Nutzer von Hand pflegt
//! (C1 der Runde 16).
//!
//! ```text
//! resources/default-readers.toml ──include_str!──> AUSLIEFERUNGSTEXT
//!                                                     │        │
//!                            erster Start ──atomar────┘        │
//!                                                              v
//!      ~/Library/.../KRK/readers.toml ──Ablage──> Profildatei ──pruefen──> Profile
//! ```
//!
//! # Die Vorlage ist [`super::einstellungen`], und die Abweichungen sind zwei
//!
//! Der Weg ist derselbe: die Auslieferungsfassung steht einkompiliert da, wird
//! beim ersten Start **woertlich** geschrieben und danach von KRK nie wieder
//! angefasst. Der Grund ist derselbe: `serde` kennt keine Kommentare, und die
//! Kommentarzeilen der Auslieferungsfassung sind der Zweck der Datei
//! und nicht ihre Verzierung — wie viele es sind, sagt der Dateibestand und
//! nicht diese Zeile, denn die Zahl waechst mit jedem Profil. Sie nennen die
//! vier Bausteinnamen, die
//! Vorrangregel der Erkennung und die Zahlen des Haushalts. Eine
//! Serialisierung von [`Profile`] hinterliesse sie ohne all das.
//!
//! **Erste Abweichung: hier wird angelegt, bevor gelesen wird.** Bei den
//! Einstellungen steht die Anlage hinter dem Laden, weil dort jedes fehlende
//! Feld ohnehin aus der Auslieferungsfassung kommt und die Reihenfolge nichts
//! entscheidet. Hier entscheidet sie: eine fehlende Datei und eine gueltige
//! Datei ohne einen einzigen Block sind fuer [`super::Zugang::laden`]
//! dasselbe Ergebnis (Auslieferungszustand, keine Meldung), und wer erst laedt,
//! haette den ersten Start entweder ohne Profile verbracht oder die
//! eingebettete Fassung ein zweites Mal zerlegen muessen. Angelegt und dann
//! gelesen, arbeitet KRK mit genau dem, was in der Datei steht — auch in der
//! Sitzung, die sie angelegt hat.
//!
//! **Zweite Abweichung: eine beschaedigte Datei fuehrt hier nicht zur
//! Auslieferungsfassung, sondern zu gar keinem Profil** (C1.6). Bei den
//! Einstellungen springt der ausgelieferte Wert ein, weil KRK ohne
//! Terminal-Kennung keinen Ordner im Terminal oeffnen koennte. Hier gibt es
//! nichts, was einspringen muesste: ohne Profil zeigt jeder Ordner die
//! Metadatenanzeige, die er bis zur Runde 15 immer gezeigt hat. Die
//! Auslieferungsfassung an die Stelle der beschaedigten Nutzerdatei zu setzen,
//! hiesse dem Nutzer Profile unterzuschieben, die er vielleicht gerade
//! herausgenommen hat.
//!
//! # Warum die geprueften Profile woanders wohnen als die gelesenen
//!
//! Die Ablage kennt Pfad, Format und Fehlerbehandlung und nicht den Inhalt;
//! das steht so im Kopf von [`super`] und gilt fuer diese Datei wie fuer die
//! sechs anderen. Was ein Profil bedeutet, ob sich sein Muster uebersetzen
//! laesst und welche Zeile ihren Baustein verliert, entscheidet deshalb
//! [`crate::leseprofil`] und nicht dieses Modul. Hier steht der Weg von der
//! Platte zu [`datei::Profildatei`]; von dort zu [`Profile`] fuehrt der eine
//! Pruefschritt [`datei::pruefen`], und dieses Modul reicht ihn nur durch.
//!
//! Die Meldungen jenes Pruefschritts gehen deshalb **neben** der [`Ersetzung`]
//! heraus und nicht in ihr: eine `Ersetzung` sagt, dass eine Datei ersetzt
//! wurde, und genau das ist bei einem abgewiesenen Profil nicht geschehen. Die
//! Datei ist in Ordnung, ein Profil darin ist es nicht.
//!
//! **Die Meldung zu dieser Datei verspricht seit dem 260824 keinen
//! Auslieferungszustand mehr.** Der Satzteil stand bis dahin als feststehende
//! Prosa im Formatierer von [`Ersetzung`] und war fuer `readers.toml` als
//! einzige der sieben falsch; er kommt jetzt aus
//! [`Datei::ersatz`](super::Datei::ersatz), das ihn je Datei beantwortet.

use std::io;

use super::{Beiseite, Datei, Ersetzung, Geladen, Grund, Zugang, atomar};
use crate::leseprofil::{Profile, datei};

/// Die Auslieferungsfassung der Leseprofile, in das Programm einkompiliert.
///
/// Damit gibt es keinen Start ohne die mitgelieferten Profile, und die
/// Anlage beim ersten Start braucht keinen Zugriff auf das Buendel. Dieselbe
/// Ueberlegung traegt [`super::einstellungen::AUSLIEFERUNGSTEXT`].
pub const AUSLIEFERUNGSTEXT: &str = include_str!("../../../../resources/default-readers.toml");

/// Laedt `readers.toml` und legt sie beim ersten Start an.
///
/// Scheitert nie. Die fuenf Faelle:
///
/// | Auf der Platte | Ergebnis |
/// |---|---|
/// | keine Datei | die Datei entsteht und wird gelesen, **keine** Meldung |
/// | gueltige Datei | ihre Profile, nichts wird geschrieben |
/// | Datei ohne obersten Schluessel | kein Profil, **keine** Meldung (C1.5) |
/// | kaputte Datei | kein Profil, Meldung, die Datei wird beiseitegelegt (C1.6) |
/// | nicht anlegbar | kein Profil, Meldung (C1.7) |
///
/// Die zweite Haelfte des Rueckgabepaares sind die Meldungen aus
/// [`datei::pruefen`]: je eine Zeile fuer ein abgewiesenes Profil und fuer eine
/// Zeile, die ihren Baustein verloren hat. Warum sie nicht in die [`Ersetzung`]
/// gehoeren, steht im Modulkopf.
///
/// **Hoechstens eine [`Ersetzung`] kann anfallen.** Die zwei Quellen schliessen
/// einander in der Sache aus: eine Datei, die sich nicht anlegen liess, ist
/// beim Lesen nicht da und traegt dort keine Ersetzung. Trifft dennoch beides
/// zu, gilt die gelesene: sie benennt einen Schaden an einer Datei, die es
/// gibt.
#[must_use = "die zweite Haelfte des Paares sind die Meldungen ueber abgewiesene \
              Profile und Zeilen; wer sie fallen laesst, verschweigt dem Nutzer, \
              warum ein Profil seiner readers.toml nicht greift"]
pub fn laden(zugang: &Zugang<'_>) -> (Geladen<Profile>, Vec<String>) {
    // Angelegt wird vor dem Lesen, damit der erste Start mit den Profilen
    // arbeitet, die er gerade geschrieben hat; die Begruendung steht im
    // Modulkopf unter "Erste Abweichung".
    let anlage = anlegen_falls_fehlt(zugang);
    let roh: Geladen<datei::Profildatei> = zugang.laden(Datei::Leser);
    let (wert, meldungen) = datei::pruefen(roh.wert);
    let ersetzung = roh.ersetzung.or_else(|| {
        anlage.err().map(|fehler| Ersetzung {
            datei: zugang.pfad(Datei::Leser),
            welche: Datei::Leser,
            grund: Grund::NichtAnlegbar(fehler.to_string()),
            // Eine Datei, die es nicht gibt, hat keinen Inhalt zu sichern.
            beiseite: Beiseite::Nicht,
        })
    });
    (Geladen { wert, ersetzung }, meldungen)
}

/// Schreibt die Auslieferungsfassung woertlich, falls die Datei fehlt.
///
/// Wiederholbar wie [`super::einstellungen`] es an derselben Stelle ist: eine
/// vorhandene Datei ist kein Fehler und wird nicht angefasst, gleich was in ihr
/// steht (C1.2). Auch eine leergeraeumte bleibt leer; der Nutzer hat sie so
/// gewollt.
fn anlegen_falls_fehlt(zugang: &Zugang<'_>) -> io::Result<()> {
    let pfad = zugang.pfad(Datei::Leser);
    if pfad.try_exists()? {
        return Ok(());
    }
    atomar::schreiben(&pfad, &mut AUSLIEFERUNGSTEXT.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Auslieferungsfassung nennt jeden der vier Bausteine (C5.10).
    ///
    /// Ohne die Kommentarzeilen stuende dort eine Datei, deren Sprache der
    /// Nutzer nirgends nachschlagen kann; dieselbe Zusage haelt
    /// `die_auslieferungsfassung_traegt_ihre_kommentare` fuer `settings.toml`.
    #[test]
    fn die_auslieferungsfassung_nennt_jeden_bausteinnamen() {
        for name in ["zaehlung", "juengste", "feld", "vorhandensein"] {
            assert!(
                AUSLIEFERUNGSTEXT.contains(name),
                "die Auslieferungsfassung nennt den Baustein {name} nicht"
            );
        }
        let kommentarzeilen = AUSLIEFERUNGSTEXT
            .lines()
            .filter(|zeile| zeile.trim_start().starts_with('#'))
            .count();
        assert!(
            kommentarzeilen > 100,
            "die Auslieferungsfassung traegt nur {kommentarzeilen} Kommentarzeilen"
        );
    }

    /// C3.4 der Runde 19: Keine Zeile der mitgelieferten Profile nennt einen
    /// der zwei neuen Schluessel, und es bleiben zwoelf Profile.
    ///
    /// Gezaehlt wird ueber die **Nicht-Kommentarzeilen**, und das ist der
    /// Kern der Probe: der Kommentarteil derselben Datei beschreibt `typ`
    /// und `versteckt` sehr wohl (C3.9), und ein Zaehlweg, der ihn mitlaese,
    /// waere seit Schritt 6 jener Runde rot. Was hier gehalten wird, ist,
    /// dass die Ausgabe der zwoelf Profile sich nicht aendert; der Nachweis
    /// dafuer ist, dass kein `[[profil.zeile]]`-Block die Schluessel traegt.
    ///
    /// Ein `#` mitten in einer Zeile beginnt in TOML ebenfalls einen
    /// Kommentar; die mitgelieferten Zeilen tragen keinen, und die Probe
    /// schneidet trotzdem hinter dem ersten `#` ab, damit sie einen
    /// nachgestellten Kommentar nicht fuer eine Angabe haelt.
    #[test]
    fn keine_mitgelieferte_zeile_nennt_typ_oder_versteckt() {
        let nennungen: Vec<&str> = AUSLIEFERUNGSTEXT
            .lines()
            .map(|zeile| zeile.split('#').next().unwrap_or(""))
            .filter(|zeile| zeile.contains("typ =") || zeile.contains("versteckt ="))
            .collect();
        assert!(
            nennungen.is_empty(),
            "eine mitgelieferte Profilzeile nennt typ oder versteckt: {nennungen:?}"
        );

        let gelesen: datei::Profildatei =
            toml::from_str(AUSLIEFERUNGSTEXT).expect("die Auslieferungsfassung ist kein TOML");
        let (profile, _) = datei::pruefen(gelesen);
        assert_eq!(profile.zahl(), 12, "es sind nicht mehr die zwoelf Profile");
        let zaehlungen_mit_neuen_schluesseln = profile
            .iter()
            .flat_map(|profil| profil.zeilen())
            .filter(|zeile| {
                matches!(
                    zeile.baustein(),
                    Some(crate::leseprofil::Baustein::Zaehlung { typ: Some(_), .. })
                        | Some(crate::leseprofil::Baustein::Zaehlung {
                            versteckt: true,
                            ..
                        })
                )
            })
            .count();
        assert_eq!(
            zaehlungen_mit_neuen_schluesseln, 0,
            "eine gepruefte Zaehlung der Auslieferungsfassung traegt einen Typ oder die Klammer"
        );
    }

    /// KRK liefert keine Fassung mit, die die eigene Pruefung nicht besteht.
    ///
    /// Die Probe ist der Grund, aus dem ein Tippfehler in
    /// `resources/default-readers.toml` beim Bauen auffaellt und nicht erst
    /// beim Nutzer: sie zerlegt den eingebetteten Text und laesst ihn durch
    /// denselben [`datei::pruefen`], den die Nutzerdatei durchlaeuft.
    #[test]
    fn die_eingebettete_fassung_besteht_ihre_eigene_pruefung() {
        let gelesen: datei::Profildatei = toml::from_str(AUSLIEFERUNGSTEXT)
            .expect("die eingebettete Auslieferungsfassung ist kein gueltiges TOML");
        let (profile, meldungen) = datei::pruefen(gelesen);
        assert!(
            meldungen.is_empty(),
            "die eingebettete Auslieferungsfassung wird beanstandet: {meldungen:?}"
        );
        assert_eq!(
            profile.zahl(),
            12,
            "die Auslieferungsfassung fuehrt nicht mehr die zwoelf mitgelieferten Profile"
        );
    }
}
