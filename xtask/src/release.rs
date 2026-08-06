//! Das Auslieferungspaket: `cargo xtask release` (Schritt 23).
//!
//! Der Weg in sechs Stationen, jede scheitert mit einer benennenden Meldung:
//!
//! 1. **AppKit-Grenze pruefen:** keine Nennung einer `objc2`-Kiste ausserhalb
//!    von `crates/krk-ui/src/appkit/`, weder als `use`-Zeile noch als
//!    ausgeschriebener Pfad, und das in allen drei Quellwurzeln des Workspace.
//!    Die Pruefung traegt die Grenzzusage aus dem Plan maschinell, weil
//!    `#![deny(unsafe_code)]` sie nur zur Haelfte erzwingt: ein grosser Teil
//!    der `objc2`-Bindungen ist als sicher deklariert und uebersetzt
//!    ausserhalb anstandslos. Defekte
//!    `260803-1530_*_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen`
//!    und
//!    `260806-1333_*_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten`.
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

/// Die Quellwurzeln der AppKit-Grenzpruefung, je mit dem einen Teilbaum
/// darunter, der als einziger eine `objc2`-Kiste nennen darf.
///
/// Alle drei Kisten des Workspace stehen hier, nicht nur `krk-ui`. Fuer
/// `krk-core` belegt zwar schon das Abnahmekriterium von S15 ueber die
/// Abhaengigkeiten der Kiste, dass sie keine `objc2`-Kiste uebersetzen kann;
/// die zweite Pruefung kostet nichts und macht die Grenze an einer Stelle
/// lesbar statt ueber zwei Kriterien verteilt. Fuer `krk-bench` gab es bis zum
/// 260806 gar keine Zusage.
const GRENZWURZELN: [(&str, Option<&str>); 3] = [
    ("crates/krk-ui/src", Some("appkit")),
    ("crates/krk-core/src", None),
    ("crates/krk-bench/src", None),
];

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

/// Prueft, dass ausserhalb von `crates/krk-ui/src/appkit/` keine `objc2`-Kiste
/// genannt wird.
///
/// Dieselbe Vorschrift wie im Abnahmekriterium von Schritt 23, und sie besteht
/// aus zwei Suchen ueber dieselben drei Quellwurzeln (`GRENZWURZELN`): die
/// `use`-Zeile aus `ist_objc2_use` und der ausgeschriebene Pfad aus
/// `nennt_objc2_pfad`. Eine `objc2`-Bindung kommt ohne eines von beidem nicht
/// zustande, gleich ob die Kiste sie als `pub fn` oder als `pub unsafe fn`
/// fuehrt; zusammen fangen die zwei Suchen beide Haelften der Grenze.
fn appkit_grenze_pruefen(wurzel: &Path) -> Result<(), Abbruch> {
    let mut verstoesse = Vec::new();
    for (quellwurzel, ausnahme) in GRENZWURZELN {
        let quellwurzel = wurzel.join(quellwurzel);
        let ausgenommen = ausnahme.map(|name| quellwurzel.join(name));
        dateien_pruefen(&quellwurzel, ausgenommen.as_deref(), &mut verstoesse)?;
    }
    if !verstoesse.is_empty() {
        verstoesse.sort();
        let aufzaehlung: Vec<String> = verstoesse
            .iter()
            .map(|pfad| format!("\x20      {}", pfad.display()))
            .collect();
        return Err(Abbruch::Lauf(format!(
            "Die AppKit-Grenze ist verletzt: eine `objc2`-Kiste ist ausserhalb von \
             crates/krk-ui/src/appkit/ genannt, als `use`-Zeile oder als ausgeschriebener \
             Pfad, in\n\
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
        "AppKit-Grenze geprueft: keine `objc2`-Kiste ausserhalb von \
         crates/krk-ui/src/appkit/, weder als `use`-Zeile noch als ausgeschriebener Pfad."
    );
    Ok(())
}

/// Geht die `.rs`-Dateien unter `ordner` durch und sammelt die Verstoesse.
///
/// Der Teilbaum `ausgenommen` wird nicht betreten: dort, und nur dort, ist
/// eine `objc2`-Kiste erlaubt. `None` heisst, dass die Quellwurzel keine
/// Ausnahme kennt.
fn dateien_pruefen(
    ordner: &Path,
    ausgenommen: Option<&Path>,
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
        if ausgenommen.is_some_and(|ausnahme| pfad == ausnahme) {
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
        if inhalt.lines().any(verletzt_grenze) {
            verstoesse.push(pfad);
        }
    }
    Ok(())
}

/// Ob eine Zeile die AppKit-Grenze verletzt.
///
/// Zwei Formen nennen eine `objc2`-Kiste: die `use`-Zeile und der
/// ausgeschriebene Pfad. Die zweite kam bis zum 260806 durch
/// (`issues/260806-1333_*_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md`);
/// `objc2::rc::Weak::from_retained(&x)` ist gueltiges Rust ohne jede
/// `use`-Zeile und steht heute mehrfach in `appkit/anwendung.rs`.
///
/// **Was als Kommentar gilt, und warum die Regel so grob ist.** Eine Zeile,
/// deren erstes nicht-leeres Zeichen ein `/` ist, wird nicht gelesen. Das ist
/// die ganze Kommentarbehandlung — kein Zustandsautomat fuer `//` und
/// `/* */`, wie der Defekt ihn erwogen hat. Drei Gruende. Erstens treffen die
/// zwoelf Kommentarzeilen des Baums, die `objc2` nennen und auf denen die
/// Pruefung nicht anschlagen darf, allesamt diese Form: sie stehen als `//!`
/// in Spalte 1. Zweitens gibt es im ganzen Verzeichnis `crates/` keinen
/// einzigen Blockkommentar, gemessen am 260806; ein Automat dafuer waere Code
/// gegen einen Fall, den es nicht gibt, und die Maxime des Vorhabens ist
/// "supersimpel". Drittens faellt die verbleibende Luecke — ein nachgestellter
/// Kommentar hinter Code, der `objc2::` nennt — zur sicheren Seite: sie meldet
/// einen Verstoss zu viel, nicht einen zu wenig, und ein Umformulieren des
/// Kommentars raeumt sie aus. Ein halber Rust-Zerteiler in einem Bauwerkzeug
/// koennte umgekehrt scheitern, und dann schweigt das Tor.
fn verletzt_grenze(zeile: &str) -> bool {
    let inhalt = zeile.trim_start();
    if inhalt.starts_with('/') {
        return false;
    }
    ist_objc2_use(inhalt) || nennt_objc2_pfad(inhalt)
}

/// Ob die Zeile einen ausgeschriebenen Pfad in eine `objc2`-Kiste nennt.
///
/// Gesucht ist ein Bezeichner, der mit `objc2` beginnt und auf den unmittelbar
/// `::` folgt: `objc2::rc::Weak`, `objc2_app_kit::NSView`,
/// `<objc2_foundation::NSString>::from_str`. Vor dem `objc2` muss ein
/// Zeichen stehen, das kein Bezeichnerzeichen ist, sonst traefe die Suche auch
/// `meinobjc2::x`, also einen fremden Namen, der nur so endet.
///
/// Die Zeile wird nicht auf Kommentare geprueft; das erledigt
/// `verletzt_grenze` vorher.
fn nennt_objc2_pfad(zeile: &str) -> bool {
    let bytes = zeile.as_bytes();
    let mut ab = 0;
    while let Some(stelle) = zeile[ab..].find("objc2") {
        let anfang = ab + stelle;
        ab = anfang + "objc2".len();
        if anfang > 0 && ist_bezeichnerzeichen(bytes[anfang - 1]) {
            continue;
        }
        let mut ende = ab;
        while ende < bytes.len() && ist_bezeichnerzeichen(bytes[ende]) {
            ende += 1;
        }
        if zeile[ende..].starts_with("::") {
            return true;
        }
    }
    false
}

/// Ob das Byte in einem Rust-Bezeichner stehen darf.
///
/// Nur die ASCII-Haelfte: ein Bezeichner darf zwar auch Unicode tragen, aber
/// keine Kiste des Vorhabens tut das, und ein Fortsetzungsbyte einer deutschen
/// Umlaut-Kommentarzeile gilt so als Grenze statt als Bezeichnerzeichen.
fn ist_bezeichnerzeichen(zeichen: u8) -> bool {
    zeichen.is_ascii_alphanumeric() || zeichen == b'_'
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

    /// Die erste der beiden Luecken vom 260806-1333: der ausgeschriebene Pfad.
    #[test]
    fn ein_ausgeschriebener_objc2_pfad_ist_ein_verstoss() {
        // Woertlich aus crates/krk-ui/src/appkit/anwendung.rs:575. Innerhalb
        // von appkit/ ist die Zeile erlaubt, ausserhalb ist sie der Verstoss,
        // den die Vorgaengerin nicht sah.
        assert!(verletzt_grenze(
            "            let schwach = objc2::rc::Weak::from_retained(&self.retain());"
        ));
        assert!(verletzt_grenze("    objc2_app_kit::NSView::alloc(mtm);"));
        assert!(verletzt_grenze(
            "    let text = <objc2_foundation::NSString>::from_str(\"x\");"
        ));
        assert!(verletzt_grenze(
            "    fn sicht(&self) -> objc2::rc::Retained<NSView> {"
        ));
        // Die `use`-Zeile bleibt ein Verstoss, jetzt ueber dieselbe Frage.
        assert!(verletzt_grenze("use objc2::rc::Retained;"));
        assert!(verletzt_grenze("pub use objc2_app_kit::NSView;"));
        // Ohne `::` ist `objc2` nur ein Wort; die `use`-Zeile faengt es.
        assert!(verletzt_grenze("use objc2_app_kit as ak;"));
    }

    /// Die zwoelf Kommentarzeilen, die es heute im Baum gibt — woertlich.
    ///
    /// Zehn unter `crates/krk-ui/src` ausserhalb von `appkit/`, zwei unter
    /// `crates/krk-core/src`. Schlaegt die Pruefung auf einer davon an, ist der
    /// Bau sofort rot, ohne dass die Grenze verletzt waere.
    #[test]
    fn die_kommentarzeilen_des_baums_sind_kein_verstoss() {
        for zeile in [
            "//! **Keine Zeile AppKit.** In diesem Verzeichnis steht keine `use objc2`-Zeile,",
            "//! keines von ihnen nennt eine `objc2`-Kiste. `messmodus` haelt den Ablauf der",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, und",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, wie",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile und",
            "//! hier keine `use objc2`-Zeile. Wo der Fokus steht, liest",
            "//! keine `use objc2`-Zeile**, und das ist nachpruefbar, nicht nur gemeint.",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die",
            "//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die",
            "//! hier keine `use objc2`-Zeile. Die Ansichten dazu sind die vier Blaetter unter",
            "//! Diese Datei ist reines Rust und nennt keine `objc2`-Kiste. Sie bekommt eine",
            "//! weiterhin von oben nach unten: `krk-core` nennt keine `objc2`-Kiste. Ein",
        ] {
            assert!(!verletzt_grenze(zeile), "schlaegt an auf: {zeile}");
        }
        // Und ein Kommentar, der den Pfad ausschreibt: heute steht er so
        // nirgends, morgen kann er es. Die Kommentarregel faengt ihn.
        assert!(!verletzt_grenze(
            "//! Die Huelle um `objc2::rc::Retained` liegt unter `appkit/`."
        ));
        assert!(!verletzt_grenze(
            "    /// Reicht `objc2_app_kit::NSView` nach draussen."
        ));
    }

    #[test]
    fn zeilen_ohne_objc2_sind_kein_verstoss() {
        assert!(!verletzt_grenze("use std::path::PathBuf;"));
        assert!(!verletzt_grenze("    let x = std::mem::take(&mut y);"));
        assert!(!verletzt_grenze(""));
        // Ein fremder Name, der nur auf `objc2` endet.
        assert!(!verletzt_grenze("    meinobjc2::rufen();"));
        // `objc2` ohne folgendes `::` und ohne `use` ist nur ein Wort.
        assert!(!verletzt_grenze("    let name = \"objc2\";"));
    }

    /// Die Pruefung am echten Baum, nicht nur an erfundenen Zeilen.
    ///
    /// Sie haengt sonst allein an `cargo xtask release`, und das verlangt eine
    /// Signaturidentitaet und zwei Uebersetzungslaeufe. So laeuft dieselbe
    /// Pruefung bei jedem `make check` mit und meldet einen Verstoss am Tag,
    /// an dem er entsteht, statt am Tag der Auslieferung.
    #[test]
    fn die_grenzpruefung_laeuft_am_baum_gruen() {
        appkit_grenze_pruefen(&bundle::wurzel()).expect("die AppKit-Grenze haelt");
    }

    #[test]
    fn release_nimmt_keine_weiteren_marken() {
        let argumente = vec!["--adhoc".to_owned()];
        assert!(matches!(ausfuehren(&argumente), Err(Abbruch::Aufruf(_))));
    }
}
