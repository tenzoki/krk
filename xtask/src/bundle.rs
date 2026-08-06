//! Der Buendelbau: `cargo xtask bundle`.
//!
//! Das Ergebnis ist `target/KRK.app` mit dieser Struktur:
//!
//! ```text
//! target/KRK.app/
//! └── Contents/
//!     ├── Info.plist      Kopie von resources/Info.plist, Version eingesetzt
//!     ├── PkgInfo         die acht Bytes APPL????
//!     ├── MacOS/krk       das uebersetzte Binaerziel
//!     └── Resources/      noch leer, spaetere Schritte legen hier ab
//! ```
//!
//! **Die Reihenfolge ist Absicht.** Alles, was scheitern kann, scheitert bevor
//! ein Verzeichnis entsteht: erst die Versionsersetzung, dann der Name des
//! Binaerprogramms, dann die Signaturidentitaet, und erst danach wird
//! uebersetzt und geschrieben. Ein abgebrochener Lauf hinterlaesst so kein
//! halbes Buendel, und wer die Identitaet noch nicht angelegt hat, erfaehrt es
//! vor und nicht nach einem vollstaendigen Uebersetzungslauf.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Abbruch;
use crate::sign;

/// Der Platzhalter, den `resources/Info.plist` seit Schritt 4b traegt.
///
/// Die Version wohnt allein im Feld `version` unter `[workspace.package]` der
/// `Cargo.toml`. `xtask` erbt sie ueber `version.workspace = true`, `env!` holt
/// sie beim Uebersetzen aus dem Manifest, und diese Zeichenkette markiert die
/// Stelle, an der sie in die Kopie im Buendel wandert.
pub const PLATZHALTER: &str = "__KRK_VERSION__";

/// Die Version aus `[workspace.package]`, geerbt ueber `version.workspace = true`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Der Name des Buendels unter `target/`.
const BUENDELNAME: &str = "KRK.app";

/// Das Cargo-Paket, dessen Binaerziel ins Buendel wandert.
const PAKET: &str = "krk-ui";

/// Das Bauprofil.
///
/// `release` und nicht `debug`, weil dasselbe Buendel spaeter die Zeitzusagen
/// aus C8 misst. Eine Zahl, die an einem unoptimierten Bau entsteht, sagt ueber
/// die Zusagen nichts aus und wuerde das Gate aus Schritt 8 grundlos reissen.
const PROFIL: &str = "release";

/// Der Inhalt von `Contents/PkgInfo`: Buendeltyp und Erzeugerkennung.
///
/// Dieselben vier Zeichen wie `CFBundlePackageType` in der `Info.plist`; die
/// Erzeugerkennung ist unbelegt, dafuer stehen die vier Fragezeichen.
const PKGINFO: &str = "APPL????";

/// Baut `target/KRK.app` und gibt seinen Pfad zurueck.
pub fn bauen() -> Result<PathBuf, Abbruch> {
    let vorlage = vorbereiten()?;
    let identitaet = sign::bestimmen()?;

    uebersetzen(&vorlage.wurzel, &vorlage.binaername, None)?;

    let uebersetzt = zielpfad(&vorlage.wurzel, None, &vorlage.binaername);
    let buendel = vorlage.zusammensetzen(&uebersetzt)?;
    sign::signieren(&buendel, &identitaet)?;
    Ok(buendel)
}

/// Die geprueften Zutaten des Buendels, vor jedem Uebersetzungslauf gesammelt.
///
/// `release` (Schritt 23) baut dasselbe Buendel wie `bundle`, nur ueber eine
/// universelle Binaerdatei. Damit daneben kein zweiter Buendelbauer entsteht,
/// die zweite Wahrheit ueber die Struktur von `KRK.app`, sind die Pruefungen
/// und die Montage hier herausgeloest: beide Unterbefehle rufen dieselben
/// Funktionen und unterscheiden sich allein darin, welche Binaerdatei in
/// `Contents/MacOS` wandert und womit signiert wird.
pub(crate) struct Vorlage {
    /// Die Projektwurzel, aus dem Manifestordner von `xtask` abgeleitet.
    pub(crate) wurzel: PathBuf,
    /// Die Buendelbeschreibung mit bereits eingesetzter Version.
    plist: String,
    /// Der Name des Binaerprogramms aus `CFBundleExecutable`.
    pub(crate) binaername: String,
}

/// Liest und prueft die Buendelbeschreibung, bevor irgendetwas entsteht.
///
/// Traegt die Abbruchreihenfolge aus dem Modulkopf: Versionsersetzung und
/// Binaername scheitern hier, vor dem ersten Uebersetzungslauf und vor dem
/// ersten angelegten Verzeichnis.
pub(crate) fn vorbereiten() -> Result<Vorlage, Abbruch> {
    let wurzel = wurzel();
    let vorlage_pfad = wurzel.join("resources").join("Info.plist");
    let vorlage = fs::read_to_string(&vorlage_pfad).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} ist nicht lesbar: {fehler}",
            vorlage_pfad.display()
        ))
    })?;

    let plist = version_einsetzen(&vorlage)?;
    let binaername = binaername(&vorlage)?;
    Ok(Vorlage {
        wurzel,
        plist,
        binaername,
    })
}

impl Vorlage {
    /// Legt `target/KRK.app` aus einer bereits uebersetzten Binaerdatei an.
    ///
    /// Signiert wird hier nicht: `bundle` signiert lokal, `release` mit
    /// Developer-ID und gehaerteter Laufzeitumgebung, und beide tun das nach
    /// der Montage am fertigen Buendel.
    pub(crate) fn zusammensetzen(&self, binaerquelle: &Path) -> Result<PathBuf, Abbruch> {
        let buendel = self.wurzel.join("target").join(BUENDELNAME);
        let contents = buendel.join("Contents");
        let macos = contents.join("MacOS");

        if buendel.exists() {
            fs::remove_dir_all(&buendel)
                .map_err(|fehler| schreibfehler("das alte Buendel entfernen", &buendel, &fehler))?;
        }
        fs::create_dir_all(&macos).map_err(|fehler| schreibfehler("anlegen", &macos, &fehler))?;
        let resources = contents.join("Resources");
        fs::create_dir_all(&resources)
            .map_err(|fehler| schreibfehler("anlegen", &resources, &fehler))?;

        let im_buendel = macos.join(&self.binaername);
        fs::copy(binaerquelle, &im_buendel).map_err(|fehler| {
            Abbruch::Lauf(format!(
                "{} laesst sich nicht nach {} kopieren: {fehler}",
                binaerquelle.display(),
                im_buendel.display()
            ))
        })?;

        let plist_pfad = contents.join("Info.plist");
        fs::write(&plist_pfad, &self.plist)
            .map_err(|fehler| schreibfehler("schreiben", &plist_pfad, &fehler))?;
        let pkginfo_pfad = contents.join("PkgInfo");
        fs::write(&pkginfo_pfad, PKGINFO)
            .map_err(|fehler| schreibfehler("schreiben", &pkginfo_pfad, &fehler))?;

        println!("Version {VERSION} in {} eingesetzt.", plist_pfad.display());
        Ok(buendel)
    }
}

/// Die Projektwurzel.
///
/// Aus dem Manifestordner von `xtask` abgeleitet und nicht aus dem
/// Arbeitsverzeichnis: `cargo xtask` laesst sich aus jedem Unterordner rufen,
/// und das Buendel soll trotzdem immer an derselben Stelle entstehen.
fn wurzel() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("der Manifestordner von xtask liegt in der Projektwurzel")
        .to_path_buf()
}

/// Setzt die Version in die Buendelbeschreibung ein.
///
/// Findet die Ersetzung den Platzhalter nicht, entsteht kein Buendel. Ohne
/// diesen Abbruch koennte still ein Buendel mit einer veralteten oder gar
/// keiner Version herauskommen, und genau das war der Defekt, der die Version
/// in die `Cargo.toml` allein gezogen hat.
fn version_einsetzen(vorlage: &str) -> Result<String, Abbruch> {
    if !vorlage.contains(PLATZHALTER) {
        return Err(Abbruch::Lauf(format!(
            "resources/Info.plist traegt den Platzhalter {PLATZHALTER} nicht. Die Version wohnt \
             allein im Feld `version` unter [workspace.package] der Cargo.toml und wird beim \
             Buendeln an dieser Stelle eingesetzt; ohne den Platzhalter entstuende ein Buendel \
             mit einer veralteten oder gar keiner Version. Es wird keines gebaut."
        )));
    }
    Ok(vorlage.replace(PLATZHALTER, VERSION))
}

/// Liest den Namen des Binaerprogramms aus der Buendelbeschreibung.
///
/// Die `Info.plist` fuehrt ihn unter `CFBundleExecutable`, und macOS startet
/// genau die Datei, die dort steht. Der Name wird deshalb von dort gelesen und
/// nicht ein zweites Mal hier hingeschrieben: eine Abweichung zwischen beiden
/// waere ein Buendel, das sich bauen laesst und nicht startet.
fn binaername(vorlage: &str) -> Result<String, Abbruch> {
    let name = plist_zeichenkette(vorlage, "CFBundleExecutable").ok_or_else(|| {
        Abbruch::Lauf(
            "resources/Info.plist nennt keinen Schluessel CFBundleExecutable mit einer \
             Zeichenkette. Ohne ihn ist nicht bestimmt, welche Datei macOS im Buendel startet."
                .to_owned(),
        )
    })?;
    if name.is_empty() {
        return Err(Abbruch::Lauf(
            "CFBundleExecutable in resources/Info.plist ist leer.".to_owned(),
        ));
    }
    Ok(name)
}

/// Liest den Wert eines Schluessels aus einer Property-Liste im XML-Format.
///
/// Bewusst kein Parser: gebraucht wird ein einziger Wert aus einer Datei, die
/// im selben Projekt liegt und dem Muster `<key>…</key><string>…</string>`
/// folgt. Steht zwischen Schluessel und Wert ein weiterer `<key>`, ist der
/// gesuchte Schluessel nicht mit einer Zeichenkette belegt, und die Funktion
/// liefert nichts, statt den Wert des naechsten Schluessels auszugeben.
fn plist_zeichenkette(plist: &str, schluessel: &str) -> Option<String> {
    let marke = format!("<key>{schluessel}</key>");
    let hinter_schluessel = plist.split_once(&marke)?.1;
    let (zwischenraum, hinter_beginn) = hinter_schluessel.split_once("<string>")?;
    if zwischenraum.contains("<key>") {
        return None;
    }
    let (wert, _) = hinter_beginn.split_once("</string>")?;
    Some(wert.trim().to_owned())
}

/// Uebersetzt das Binaerziel, wahlweise fuer ein ausdrueckliches Ziel-Tripel.
///
/// `bundle` uebersetzt ohne Tripel fuer das laufende Geraet; `release` ruft
/// die Funktion zweimal, einmal je Tripel aus `rust-toolchain.toml`, und fuegt
/// die Ergebnisse mit `lipo` zusammen.
pub(crate) fn uebersetzen(
    wurzel: &Path,
    binaername: &str,
    ziel: Option<&str>,
) -> Result<(), Abbruch> {
    // Cargo setzt CARGO auf den Pfad, unter dem es selbst laeuft. Den zu
    // uebernehmen haelt den Bau auf derselben Werkzeugkette, aus der der Aufruf
    // kam.
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let mut argumente = vec![
        "build",
        "--profile",
        PROFIL,
        "--package",
        PAKET,
        "--bin",
        binaername,
    ];
    if let Some(tripel) = ziel {
        argumente.extend(["--target", tripel]);
    }
    let status = Command::new(&cargo)
        // Aus der Wurzel heraus, damit der Bau die .cargo/config.toml findet:
        // dort steht MACOSX_DEPLOYMENT_TARGET = "15.0", das Mindest-Zielsystem
        // aus dem Spec, und es soll auch fuer diesen inneren Aufruf gelten.
        .current_dir(wurzel)
        .args(&argumente)
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("{cargo} laesst sich nicht starten: {fehler}")))?;
    if !status.success() {
        return Err(Abbruch::Lauf(format!(
            "cargo {} ist gescheitert ({status})",
            argumente.join(" ")
        )));
    }
    Ok(())
}

/// Wo das uebersetzte Binaerprogramm liegt.
///
/// Ohne Ziel-Tripel legt Cargo es unter `target/<profil>/` ab, mit Tripel
/// unter `target/<tripel>/<profil>/`. Der Pfad wird hier hergeleitet und nicht
/// in `release` ein zweites Mal, damit ein geaendertes Profil beide
/// Unterbefehle gleichzeitig trifft.
pub(crate) fn zielpfad(wurzel: &Path, ziel: Option<&str>, binaername: &str) -> PathBuf {
    let mut pfad = wurzel.join("target");
    if let Some(tripel) = ziel {
        pfad = pfad.join(tripel);
    }
    pfad.join(PROFIL).join(binaername)
}

fn schreibfehler(was: &str, pfad: &Path, fehler: &std::io::Error) -> Abbruch {
    Abbruch::Lauf(format!(
        "{} laesst sich nicht {was}: {fehler}",
        pfad.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die ausgelieferte Buendelbeschreibung, zum Uebersetzungszeitpunkt
    /// eingebunden. Sie ist der Gegenstand, auf den `bundle` laeuft; ein Test
    /// gegen eine nachgebaute Zeichenkette allein wuerde nicht merken, wenn der
    /// Platzhalter aus der echten Datei verschwindet.
    const AUSGELIEFERTE_PLIST: &str = include_str!("../../resources/Info.plist");

    #[test]
    fn die_ausgelieferte_plist_traegt_den_platzhalter() {
        assert!(version_einsetzen(AUSGELIEFERTE_PLIST).is_ok());
    }

    #[test]
    fn die_ausgelieferte_plist_nennt_das_binaerprogramm() {
        assert_eq!(binaername(AUSGELIEFERTE_PLIST).unwrap(), "krk");
    }

    #[test]
    fn die_version_ersetzt_den_platzhalter() {
        let vorlage = format!("<string>{PLATZHALTER}</string>");
        let gesetzt = version_einsetzen(&vorlage).unwrap();
        assert_eq!(gesetzt, format!("<string>{VERSION}</string>"));
        assert!(!gesetzt.contains(PLATZHALTER));
    }

    #[test]
    fn ohne_platzhalter_bricht_die_ersetzung_ab() {
        let vorlage = "<string>0.1.0</string>";
        let fehler = version_einsetzen(vorlage);
        assert!(matches!(fehler, Err(Abbruch::Lauf(_))));
    }

    #[test]
    fn ein_bereits_ersetzter_lauf_bricht_ebenfalls_ab() {
        // Ein zweiter Lauf gegen eine schon ersetzte Datei ist derselbe Fall:
        // der Platzhalter fehlt, also entsteht kein Buendel.
        let einmal = version_einsetzen(&format!("<string>{PLATZHALTER}</string>")).unwrap();
        assert!(version_einsetzen(&einmal).is_err());
    }

    #[test]
    fn der_binaername_kommt_aus_cfbundleexecutable() {
        let plist = "<key>CFBundleExecutable</key>\n\t<string>krk</string>";
        assert_eq!(binaername(plist).unwrap(), "krk");
    }

    #[test]
    fn ein_fehlender_schluessel_liefert_nichts() {
        assert!(
            plist_zeichenkette(
                "<key>CFBundleName</key><string>KRK</string>",
                "CFBundleExecutable"
            )
            .is_none()
        );
    }

    #[test]
    fn ein_schluessel_ohne_zeichenkette_liefert_nicht_den_naechsten_wert() {
        let plist = "<key>NSHighResolutionCapable</key><true/>\n<key>CFBundleName</key><string>KRK</string>";
        assert!(plist_zeichenkette(plist, "NSHighResolutionCapable").is_none());
    }
}
