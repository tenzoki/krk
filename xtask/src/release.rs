//! Das Auslieferungspaket: `cargo xtask release` (Schritt 23).
//!
//! Der Weg in sechs Stationen, jede scheitert mit einer benennenden Meldung:
//!
//! 1. **AppKit-Grenze pruefen:** keine `use objc2`-Zeile ausserhalb von
//!    `crates/krk-ui/src/appkit/`. Die Pruefung traegt die Grenzzusage aus dem
//!    Plan maschinell, weil `#![deny(unsafe_code)]` sie nur zur Haelfte
//!    erzwingt: ein grosser Teil der `objc2`-Bindungen ist als sicher
//!    deklariert und uebersetzt ausserhalb anstandslos. Defekt
//!    `260803-1530_*_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen`.
//! 2. **Beide Ziele uebersetzen:** dieselbe Uebersetzung wie `bundle`, einmal
//!    je Tripel aus `rust-toolchain.toml`.
//! 3. **`lipo`:** die beiden Binaerdateien zu einer universellen
//!    zusammenfuegen; `lipo -archs` muss danach beide Architekturen melden.
//! 4. **Montage:** dasselbe Buendel wie `bundle`, ueber `bundle::Vorlage` —
//!    ein zweiter Buendelbauer waere die zweite Wahrheit ueber die Struktur
//!    von `KRK.app`.
//! 5. **Signieren:** die Identitaetssuche aus `sign` mit Developer-ID statt
//!    Entwicklungsidentitaet, `codesign` mit `--options runtime`.
//! 6. **Beglaubigen:** `xcrun notarytool submit --wait` und
//!    `xcrun stapler staple`. Beides verlangt das vollstaendige Xcode, die
//!    Beglaubigung zusaetzlich ein Apple-Entwicklerkonto; fehlt eines von
//!    beidem, bricht allein diese Station ab, und das gebaute, signierte
//!    Buendel bleibt liegen. Der Plan nimmt den Schritt auch in diesem Fall
//!    ab, deshalb werden die Voraussetzungen der Beglaubigung erst hier
//!    geprueft und nicht, wie sonst ueblich, vor dem ersten
//!    Uebersetzungslauf.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Abbruch;
use crate::bundle;
use crate::sign;

/// Die beiden Ziel-Tripel der universellen Binaerdatei.
///
/// Dieselben zwei wie in `rust-toolchain.toml`; `rustup` haelt sie darueber
/// installiert.
const ZIELE: [&str; 2] = ["x86_64-apple-darwin", "aarch64-apple-darwin"];

/// Die Architekturnamen, die `lipo -archs` danach melden muss.
const ARCHITEKTUREN: [&str; 2] = ["x86_64", "arm64"];

/// Die Umgebungsvariable mit dem Namen des notarytool-Schluesselbundprofils.
pub const NOTAR_PROFIL_VARIABLE: &str = "KRK_NOTARY_PROFILE";

/// Baut, signiert und beglaubigt das Auslieferungspaket.
pub fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    if let Some(ueberzaehlig) = argumente.first() {
        return Err(Abbruch::Aufruf(format!(
            "release kennt {ueberzaehlig:?} nicht"
        )));
    }

    let vorlage = bundle::vorbereiten()?;
    appkit_grenze_pruefen(&vorlage.wurzel)?;
    let identitaet = sign::bestimmen_fuer_release()?;
    if !identitaet.name.starts_with(sign::DEVELOPER_ID_PRAEFIX) {
        println!(
            "Hinweis: {:?} ist keine Developer-ID-Identitaet. Signiert wird trotzdem; die \
             Beglaubigung nimmt ein so signiertes Buendel nicht an.",
            identitaet.name
        );
    }
    ziele_pruefen()?;

    for ziel in ZIELE {
        bundle::uebersetzen(&vorlage.wurzel, &vorlage.binaername, Some(ziel))?;
    }
    let universell = zusammenfuegen(&vorlage)?;

    let buendel = vorlage.zusammensetzen(&universell)?;
    sign::signieren_gehaertet(&buendel, &identitaet)?;
    println!(
        "Universell gebaut und mit gehaerteter Laufzeitumgebung signiert: {}",
        buendel.display()
    );

    beglaubigen(&buendel)?;
    println!("Beglaubigt und angeheftet: {}", buendel.display());
    Ok(())
}

/// Prueft, dass ausserhalb von `crates/krk-ui/src/appkit/` keine
/// `use objc2`-Zeile steht.
///
/// Dieselbe Vorschrift wie im Abnahmekriterium von Schritt 23:
///
/// ```text
/// grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src --include='*.rs' \
///   | grep -v '^crates/krk-ui/src/appkit/'
/// ```
///
/// Eine `objc2`-Bindung kommt ohne eine `use`-Zeile aus einer der
/// `objc2`-Kisten nicht zustande, gleich ob die Kiste sie als `pub fn` oder
/// als `pub unsafe fn` fuehrt; die `use`-Zeile faengt damit beide Haelften der
/// Grenze. Die Verankerung am Zeilenanfang ist Pflicht: unverankert traefe die
/// Suche die Modulkommentare der Form "In dieser Datei steht keine
/// `use objc2`-Zeile", gemessen am 260805-0000 mit sechs Treffern.
fn appkit_grenze_pruefen(wurzel: &Path) -> Result<(), Abbruch> {
    let quellwurzel = wurzel.join("crates").join("krk-ui").join("src");
    let ausgenommen = quellwurzel.join("appkit");
    let mut verstoesse = Vec::new();
    dateien_pruefen(&quellwurzel, &ausgenommen, &mut verstoesse)?;
    if !verstoesse.is_empty() {
        verstoesse.sort();
        let aufzaehlung: Vec<String> = verstoesse
            .iter()
            .map(|pfad| format!("\x20      {}", pfad.display()))
            .collect();
        return Err(Abbruch::Lauf(format!(
            "Die AppKit-Grenze ist verletzt: `use objc2` ausserhalb von \
             crates/krk-ui/src/appkit/ in\n\
             \n\
             {}\n\
             \n\
             Jeder AppKit-Aufruf liegt hinter einer sicheren Huelle unter \
             crates/krk-ui/src/appkit/; der Aufruf gehoert dorthin verschoben. Es entsteht \
             kein Auslieferungspaket.",
            aufzaehlung.join("\n")
        )));
    }
    println!(
        "AppKit-Grenze geprueft: keine `use objc2`-Zeile ausserhalb von \
         crates/krk-ui/src/appkit/."
    );
    Ok(())
}

/// Geht die `.rs`-Dateien unter `ordner` durch und sammelt die Verstoesse.
///
/// Der Teilbaum `ausgenommen` wird nicht betreten: dort, und nur dort, ist
/// `use objc2` erlaubt.
fn dateien_pruefen(
    ordner: &Path,
    ausgenommen: &Path,
    verstoesse: &mut Vec<PathBuf>,
) -> Result<(), Abbruch> {
    let eintraege = fs::read_dir(ordner).map_err(|fehler| {
        Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", ordner.display()))
    })?;
    for eintrag in eintraege {
        let eintrag = eintrag.map_err(|fehler| {
            Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", ordner.display()))
        })?;
        let pfad = eintrag.path();
        if pfad == ausgenommen {
            continue;
        }
        if pfad.is_dir() {
            dateien_pruefen(&pfad, ausgenommen, verstoesse)?;
            continue;
        }
        if pfad.extension().is_none_or(|endung| endung != "rs") {
            continue;
        }
        let inhalt = fs::read_to_string(&pfad).map_err(|fehler| {
            Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", pfad.display()))
        })?;
        if inhalt.lines().any(ist_objc2_use) {
            verstoesse.push(pfad);
        }
    }
    Ok(())
}

/// Ob eine Zeile eine `use objc2`-Zeile ist.
///
/// Gelesen wird: Einrueckung, eine mitgeschriebene Sichtbarkeit, `use`, ein
/// Trenner, ein moegliches fuehrendes `::`, dann ein Pfad, der mit `objc2`
/// beginnt. Ein Modulkommentar wie "In dieser Datei steht keine `use
/// objc2`-Zeile" beginnt nach der Einrueckung mit `//` und faellt durch —
/// genau die sechs Treffer, die eine unverankerte Suche gefunden haette.
///
/// **Zwei Schreibweisen, die bis zum 260806 durchkamen.** Die Vorgaengerin
/// verlangte `use` unmittelbar nach der Einrueckung und `objc2` unmittelbar
/// nach dem Zwischenraum. `pub use objc2_app_kit::NSView;` beginnt aber mit
/// `pub`, und `use ::objc2::rc::Retained;` schiebt `::` dazwischen; beide sind
/// gueltiges Rust, und ein Reexport der ersten Sorte haette jedem weiteren
/// Verbraucher die eigene `use objc2`-Zeile erspart. Einen Verstoss gab es
/// nicht, die Luecke war trotzdem da
/// (`issues/260806-0834_*_die-appkit-grenzpruefung-uebersieht-pub-use-und-use-mit-fuehrendem-doppelpunkt.md`).
fn ist_objc2_use(zeile: &str) -> bool {
    let ohne_sichtbarkeit = sichtbarkeit_abstreifen(zeile.trim_start());
    let Some(nach_use) = ohne_sichtbarkeit.strip_prefix("use") else {
        return false;
    };
    let getrimmt = nach_use.trim_start();
    // Nach `use` steht ein Trenner: Zwischenraum oder das fuehrende `::`.
    // Ohne beides ist es ein Bezeichner wie `useobjc2`.
    let pfad = match getrimmt.strip_prefix("::") {
        Some(rest) => rest.trim_start(),
        None if getrimmt.len() < nach_use.len() => getrimmt,
        None => return false,
    };
    pfad.starts_with("objc2")
}

/// Streift ein mitgeschriebenes Sichtbarkeitspraefix ab.
///
/// `pub`, `pub(crate)`, `pub(super)`, `pub(in ::eine::stelle)` — alles, was
/// vor `use` stehen darf. Steht keines da oder faengt das Wort nur mit `pub`
/// an (`public_use`), kommt die Zeile unveraendert zurueck.
fn sichtbarkeit_abstreifen(zeile: &str) -> &str {
    let Some(nach_pub) = zeile.strip_prefix("pub") else {
        return zeile;
    };
    if let Some(offen) = nach_pub.strip_prefix('(') {
        // Die erste schliessende Klammer ist die zugehoerige: der Inhalt einer
        // Sichtbarkeitsangabe traegt selbst keine Klammern.
        return match offen.find(')') {
            Some(stelle) => offen[stelle + 1..].trim_start(),
            None => zeile,
        };
    }
    let getrimmt = nach_pub.trim_start();
    if getrimmt.len() < nach_pub.len() {
        getrimmt
    } else {
        zeile
    }
}

/// Prueft, dass beide Ziel-Tripel installiert sind.
///
/// Ein fehlendes Tripel soll mit seinem Namen und dem Kommando dagegen
/// abbrechen, nicht erst mitten im zweiten Uebersetzungslauf. Laeuft `rustup`
/// selbst nicht (etwa bei einer Werkzeugkette ohne `rustup`), faellt die
/// Vorpruefung aus, und der Uebersetzungslauf meldet ein fehlendes Ziel
/// selbst.
fn ziele_pruefen() -> Result<(), Abbruch> {
    let Ok(ausgabe) = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
    else {
        println!(
            "Hinweis: rustup laesst sich nicht starten, die Zielpruefung entfaellt. Ein \
             fehlendes Ziel meldet der Uebersetzungslauf selbst."
        );
        return Ok(());
    };
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "rustup target list --installed ist gescheitert ({}): {}",
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    let installiert = String::from_utf8_lossy(&ausgabe.stdout).into_owned();
    for ziel in ZIELE {
        if !installiert.lines().any(|zeile| zeile.trim() == ziel) {
            return Err(Abbruch::Lauf(format!(
                "Das Ziel {ziel} ist nicht installiert; die universelle Binaerdatei braucht \
                 beide Ziele aus rust-toolchain.toml. Abhilfe: rustup target add {ziel}"
            )));
        }
    }
    Ok(())
}

/// Fuegt die beiden uebersetzten Binaerdateien mit `lipo` zusammen.
///
/// Ergebnis ist `target/universal/<binaername>`; `lipo -archs` prueft es
/// sofort gegen beide Architekturen, damit ein halbes Ergebnis nicht erst am
/// ausgelieferten Buendel auffaellt.
fn zusammenfuegen(vorlage: &bundle::Vorlage) -> Result<PathBuf, Abbruch> {
    let ordner = vorlage.wurzel.join("target").join("universal");
    fs::create_dir_all(&ordner).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} laesst sich nicht anlegen: {fehler}",
            ordner.display()
        ))
    })?;
    let ausgabe_pfad = ordner.join(&vorlage.binaername);

    let mut kommando = Command::new("/usr/bin/lipo");
    kommando.arg("-create");
    for ziel in ZIELE {
        kommando.arg(bundle::zielpfad(
            &vorlage.wurzel,
            Some(ziel),
            &vorlage.binaername,
        ));
    }
    kommando.arg("-output").arg(&ausgabe_pfad);
    let ausgabe = kommando
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("lipo laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "lipo -create ist gescheitert ({}): {}",
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }

    let archs = Command::new("/usr/bin/lipo")
        .arg("-archs")
        .arg(&ausgabe_pfad)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("lipo laesst sich nicht starten: {fehler}")))?;
    if !archs.status.success() {
        return Err(Abbruch::Lauf(format!(
            "lipo -archs ist gescheitert ({}): {}",
            archs.status,
            String::from_utf8_lossy(&archs.stderr).trim()
        )));
    }
    let gemeldet = String::from_utf8_lossy(&archs.stdout).into_owned();
    for architektur in ARCHITEKTUREN {
        if !gemeldet.split_whitespace().any(|wort| wort == architektur) {
            return Err(Abbruch::Lauf(format!(
                "lipo -archs meldet {:?} statt beider Architekturen {}; die Binaerdatei ist \
                 nicht universell.",
                gemeldet.trim(),
                ARCHITEKTUREN.join(" ")
            )));
        }
    }
    println!("lipo -archs: {}", gemeldet.trim());
    Ok(ausgabe_pfad)
}

/// Reicht das Buendel zur Beglaubigung ein und heftet das Ergebnis an.
///
/// Die Voraussetzungen — vollstaendiges Xcode fuer `notarytool` und `stapler`,
/// ein hinterlegtes Zugangsprofil des Apple-Entwicklerkontos — werden bewusst
/// erst hier geprueft: fehlt eine, bleibt das gebaute, signierte Buendel
/// liegen, und die Meldung benennt, was fehlt. Siehe den Modulkopf.
fn beglaubigen(buendel: &Path) -> Result<(), Abbruch> {
    werkzeug_pruefen("notarytool", buendel)?;
    werkzeug_pruefen("stapler", buendel)?;

    let profil = match std::env::var(NOTAR_PROFIL_VARIABLE) {
        Ok(wert) if !wert.trim().is_empty() => wert.trim().to_owned(),
        _ => {
            return Err(Abbruch::Lauf(format!(
                "Die Beglaubigung braucht die Zugangsdaten eines Apple-Entwicklerkontos, und \
                 die Umgebungsvariable {NOTAR_PROFIL_VARIABLE} nennt kein \
                 Schluesselbundprofil. Das gebaute und signierte Buendel liegt unter {}.\n\
                 \n\
                 Einmalig hinterlegen:\n\
                 \x20      xcrun notarytool store-credentials <Profilname> \
                 --apple-id <Apple-ID> --team-id <Team-Kennung> \
                 --password <app-spezifisches Passwort>\n\
                 \n\
                 Danach: {NOTAR_PROFIL_VARIABLE}=<Profilname> cargo xtask release",
                buendel.display()
            )));
        }
    };

    // notarytool nimmt kein nacktes Buendel an; ditto packt es so, wie Apple
    // es fuer die Einreichung vorschreibt.
    let zip = buendel.with_extension("zip");
    let gepackt = Command::new("/usr/bin/ditto")
        .arg("-c")
        .arg("-k")
        .arg("--keepParent")
        .arg(buendel)
        .arg(&zip)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("ditto laesst sich nicht starten: {fehler}")))?;
    if !gepackt.status.success() {
        return Err(Abbruch::Lauf(format!(
            "ditto ist gescheitert ({}): {}",
            gepackt.status,
            String::from_utf8_lossy(&gepackt.stderr).trim()
        )));
    }

    // --wait blockiert bis zum Urteil von Apple; der Fortschritt laeuft
    // durchgereicht ins Terminal, deshalb status() statt output().
    let eingereicht = Command::new("/usr/bin/xcrun")
        .args(["notarytool", "submit"])
        .arg(&zip)
        .args(["--keychain-profile", &profil, "--wait"])
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    let _ = fs::remove_file(&zip);
    if !eingereicht.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun notarytool submit --wait ist gescheitert ({eingereicht}). Das gebaute und \
             signierte Buendel liegt unter {}; das Protokoll der Einreichung nennt \
             \"xcrun notarytool log\" mit der oben gemeldeten Einreichungskennung.",
            buendel.display()
        )));
    }

    let angeheftet = Command::new("/usr/bin/xcrun")
        .args(["stapler", "staple"])
        .arg(buendel)
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    if !angeheftet.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun stapler staple ist gescheitert ({angeheftet}), das Buendel unter {} ist \
             beglaubigt, traegt die Beglaubigung aber nicht angeheftet.",
            buendel.display()
        )));
    }
    Ok(())
}

/// Prueft, dass `xcrun` das genannte Werkzeug findet.
///
/// `notarytool` und `stapler` liegen im vollstaendigen Xcode, nicht in den
/// Command Line Tools; die Meldung benennt genau das.
fn werkzeug_pruefen(name: &str, buendel: &Path) -> Result<(), Abbruch> {
    let gefunden = Command::new("/usr/bin/xcrun")
        .args(["--find", name])
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    if !gefunden.status.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun findet {name} nicht: die Beglaubigung braucht das vollstaendige Xcode, die \
             Command Line Tools genuegen nicht. Das gebaute und signierte Buendel liegt unter \
             {}.\n\
             \n\
             Abhilfe: Xcode installieren und die Werkzeugkette umstellen mit\n\
             \x20      sudo xcode-select -s /Applications/Xcode.app/Contents/Developer",
            buendel.display()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eine_use_zeile_aus_einer_objc2_kiste_ist_ein_verstoss() {
        assert!(ist_objc2_use("use objc2::rc::Retained;"));
        assert!(ist_objc2_use("use objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("    use objc2_foundation::NSString;"));
        assert!(ist_objc2_use("\tuse  objc2::MainThreadMarker;"));
    }

    #[test]
    fn ein_modulkommentar_ueber_die_grenze_ist_kein_verstoss() {
        // Genau die sechs Treffer, die die unverankerte Suche am 260805-0000
        // gefunden haette: Kommentare der Form "keine `use objc2`-Zeile".
        assert!(!ist_objc2_use(
            "// In dieser Datei steht keine `use objc2`-Zeile."
        ));
        assert!(!ist_objc2_use(
            "//! In dieser Datei steht keine `use objc2`-Zeile."
        ));
    }

    #[test]
    fn andere_use_zeilen_sind_kein_verstoss() {
        assert!(!ist_objc2_use("use std::path::PathBuf;"));
        assert!(!ist_objc2_use("use crate::appkit;"));
        assert!(!ist_objc2_use("useobjc2::x;"));
        assert!(!ist_objc2_use("user objc2"));
        assert!(!ist_objc2_use("use"));
        assert!(!ist_objc2_use("pub use crate::appkit;"));
        assert!(!ist_objc2_use("pub(crate) use std::fmt;"));
        // Ein Bezeichner, der mit `pub` anfaengt, ist keine Sichtbarkeit.
        assert!(!ist_objc2_use("public_use objc2::x;"));
    }

    /// Die beiden Schreibweisen, die bis zum 260806 durchkamen.
    #[test]
    fn sichtbarkeit_und_fuehrendes_doppelkolon_kommen_nicht_durch() {
        assert!(ist_objc2_use("pub use objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("pub(crate) use objc2::rc::Retained;"));
        assert!(ist_objc2_use("pub(super) use objc2_foundation::NSString;"));
        assert!(ist_objc2_use("pub(in crate::appkit) use objc2::sel;"));
        assert!(ist_objc2_use("use ::objc2::rc::Retained;"));
        assert!(ist_objc2_use("use::objc2_app_kit::NSView;"));
        assert!(ist_objc2_use("    pub use ::objc2::MainThreadMarker;"));
    }

    #[test]
    fn release_nimmt_keine_weiteren_marken() {
        let argumente = vec!["--adhoc".to_owned()];
        assert!(matches!(ausfuehren(&argumente), Err(Abbruch::Aufruf(_))));
    }
}
