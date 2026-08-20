//! Der Buendelbau: `cargo xtask bundle`.
//!
//! Das Ergebnis ist `target/KRK.app` mit dieser Struktur:
//!
//! ```text
//! target/KRK.app/
//! └── Contents/
//!     ├── Info.plist            Kopie von resources/Info.plist, Version eingesetzt
//!     ├── PkgInfo               die acht Bytes APPL????
//!     ├── MacOS/krk             das uebersetzte Binaerziel
//!     └── Resources/KRK.icns    das Symbol, aus iconset/ erzeugt
//! ```
//!
//! **Das Symbol liegt nicht im Baum, es entsteht beim Bau.** Die Quelle sind
//! die sieben PNGs unter `iconset/`; `iconutil` macht daraus die `.icns`, und
//! der Dateiname kommt aus `CFBundleIconFile` der `resources/Info.plist`.
//! Warum erzeugt und nicht eingecheckt, steht bei [`SYMBOLGROESSEN`].
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
///
/// `pub(crate)` wie [`PLATZHALTER`], seit `release` sie fuer die Tag-Pruefung
/// braucht: dort ist `v` gefolgt von dieser Zahl der Name, den HEAD tragen
/// muss. Beide Abnehmer lesen dieselbe Konstante, damit die Zahl nicht an zwei
/// Stellen wohnt.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// Das Verzeichnis mit den PNG-Quellen des Symbols, relativ zur Projektwurzel.
const SYMBOLQUELLE: &str = "iconset";

/// Das Iconset-Verzeichnis, das der Bau unter `target/` anlegt und wieder
/// abraeumt.
///
/// `iconutil` nimmt kein loses Verzeichnis, sondern eines mit der Endung
/// `.iconset` und den von Apple festgelegten Dateinamen darin. Die Werkstatt
/// steht unter `target/`, weil sie ein Bauergebnis ist: `.gitignore` haelt
/// `/target/` heraus, und der Baum traegt die Grafik weiterhin genau einmal.
const SYMBOLWERKSTATT: &str = "krk-symbol.iconset";

/// Die zehn Eintraege des `.icns`: der von `iconutil` erwartete Name und die
/// Quelldatei unter [`SYMBOLQUELLE`].
///
/// **Warum erzeugt und nicht eingecheckt.** Eine eingecheckte `.icns` waere
/// dieselbe Grafik ein zweites Mal im Baum, und die zweite Fassung veraltet
/// still, sobald jemand ein PNG austauscht. `iconutil` gehoert zum Basissystem
/// von macOS und liegt unter `/usr/bin/iconutil`, wie `codesign`, ohne das der
/// Buendelbau ohnehin nicht durchlaeuft; es kommt also keine Voraussetzung
/// hinzu, die dieses Projekt nicht schon haette.
///
/// **Die Zuordnung der Kantenlaengen.** Apple erwartet je Punktgroesse eine
/// einfache und eine `@2x`-Fassung, und `@2x` heisst die doppelte Kantenlaenge
/// derselben Punktgroesse. Aus den sieben PNGs 16/32/64/128/256/512/1024
/// bilden sich daraus fuenf Paare, und drei Kantenlaengen treten in zweien von
/// ihnen auf: 32 ist das `@2x` von 16 **und** die einfache Fassung von 32, 256
/// und 512 ebenso. `iconutil` prueft die Kantenlaenge gegen den Namen und
/// nimmt eine falsch zugeordnete Datei nicht an. Jedes der sieben PNGs kommt
/// vor; die Probe `jede_png_quelle_wird_gebraucht` haelt das fest.
///
/// **`iconset/commander.ico` steht bewusst nicht in dieser Liste.** Sie ist das
/// Symbolformat von Windows. macOS liest sie weder als Buendelsymbol noch als
/// Quelle fuer `iconutil`; sie liegt im Baum, ohne am Bau teilzunehmen. Ebenso
/// die beiden SVGs: sie sind die Zeichenquelle, aus der die PNGs entstanden
/// sind, und kein Format, das ein Buendel traegt.
const SYMBOLGROESSEN: [(&str, &str); 10] = [
    ("icon_16x16.png", "icon-16.png"),
    ("icon_16x16@2x.png", "icon-32.png"),
    ("icon_32x32.png", "icon-32.png"),
    ("icon_32x32@2x.png", "icon-64.png"),
    ("icon_128x128.png", "icon-128.png"),
    ("icon_128x128@2x.png", "icon-256.png"),
    ("icon_256x256.png", "icon-256.png"),
    ("icon_256x256@2x.png", "icon-512.png"),
    ("icon_512x512.png", "icon-512.png"),
    ("icon_512x512@2x.png", "icon-1024.png"),
];

/// Was ein Buendelbau hinterlaesst.
///
/// **Der Binaerpfad steht hier und wird nirgends zusammengesetzt.** Wer das
/// gebaute Programm aufruft — die Messstrecke tut es —, braucht den Pfad in
/// `Contents/MacOS`, und dessen letzter Namensteil kommt aus
/// `CFBundleExecutable` der `resources/Info.plist`. Bis zum 260806 schrieb
/// `messen` dafuer `krk` als Literal hin; ein geaenderter Eintrag in der Plist
/// haette dort ein gueltiges Buendel gebaut und danach gegen einen Pfad
/// gemessen, den es nicht gibt
/// (`issues/260806-0834_*_xtask-messen-nennt-den-binaernamen-krk-als-literal-statt-aus-der-plist.md`).
pub struct Gebaut {
    /// Das fertige, signierte `target/KRK.app`.
    pub buendel: PathBuf,
    /// Das Binaerprogramm darin, `KRK.app/Contents/MacOS/<CFBundleExecutable>`.
    pub binaer: PathBuf,
    /// Die Identitaet, mit der das Buendel signiert wurde.
    ///
    /// Sie steht hier, weil der Abschlusshinweis des Unterbefehls `bundle`
    /// nach ihrer Art fragt ([`sign::weitergabehinweis`]) und ein Buendel ohne
    /// die Auskunft, womit es signiert ist, nicht vollstaendig beschrieben
    /// waere.
    pub identitaet: sign::Identitaet,
}

/// Baut `target/KRK.app` und gibt seinen Pfad zurueck.
pub fn bauen() -> Result<Gebaut, Abbruch> {
    let vorlage = vorbereiten()?;
    let identitaet = sign::bestimmen()?;

    uebersetzen(&vorlage.wurzel, &vorlage.binaername, None)?;

    let uebersetzt = zielpfad(&vorlage.wurzel, None, &vorlage.binaername);
    let buendel = vorlage.zusammensetzen(&uebersetzt)?;
    sign::signieren(&buendel, &identitaet)?;
    let binaer = vorlage.binaer_im_buendel(&buendel);
    Ok(Gebaut {
        buendel,
        binaer,
        identitaet,
    })
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
    /// Der Dateiname des Symbols aus `CFBundleIconFile`, mit Endung `.icns`.
    symbolname: String,
}

/// Liest und prueft die Buendelbeschreibung, bevor irgendetwas entsteht.
///
/// Traegt die Abbruchreihenfolge aus dem Modulkopf: Versionsersetzung,
/// Binaername, Symbolname und die Symbolquellen scheitern hier, vor dem ersten
/// Uebersetzungslauf und vor dem ersten angelegten Verzeichnis. Dass die zehn
/// PNG-Quellen schon hier geprueft werden und nicht erst bei der Montage, ist
/// derselbe Gedanke: ein fehlendes `iconset/` soll vor und nicht nach einem
/// vollstaendigen Uebersetzungslauf auffallen.
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
    let symbolname = symbolname(&vorlage)?;
    symbolquellen_pruefen(&wurzel)?;
    Ok(Vorlage {
        wurzel,
        plist,
        binaername,
        symbolname,
    })
}

impl Vorlage {
    /// Legt `target/KRK.app` aus einer bereits uebersetzten Binaerdatei an.
    ///
    /// Signiert wird hier nicht: `bundle` signiert lokal, `release` mit
    /// Developer-ID und gehaerteter Laufzeitumgebung, und beide tun das nach
    /// der Montage am fertigen Buendel.
    pub(crate) fn zusammensetzen(&self, binaerquelle: &Path) -> Result<PathBuf, Abbruch> {
        let buendel = buendelpfad(&self.wurzel);
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

        let im_buendel = self.binaer_im_buendel(&buendel);
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

        // Das Symbol entsteht hier und nicht nach der Rueckkehr: beide
        // Unterbefehle signieren am Ergebnis dieser Funktion, `bundle` lokal
        // und `release` mit gehaerteter Laufzeitumgebung. Eine `.icns`, die
        // danach ins Buendel kaeme, laege ausserhalb der Signatur, und die
        // Beglaubigung nimmt ein so veraendertes Buendel nicht an.
        let symbol_pfad = resources.join(&self.symbolname);
        symbol_bauen(&self.wurzel, &symbol_pfad)?;

        println!("Version {VERSION} in {} eingesetzt.", plist_pfad.display());
        Ok(buendel)
    }

    /// Wo das Binaerprogramm in einem fertigen Buendel liegt.
    ///
    /// Die eine Stelle, die `Contents/MacOS/<CFBundleExecutable>` bildet: die
    /// Montage legt es dorthin, und die Messstrecke ruft es von dort.
    pub(crate) fn binaer_im_buendel(&self, buendel: &Path) -> PathBuf {
        buendel
            .join("Contents")
            .join("MacOS")
            .join(&self.binaername)
    }
}

/// Das `cargo`, aus dem der Aufruf kam.
///
/// Cargo setzt `CARGO` auf den Pfad, unter dem es selbst laeuft. Den zu
/// uebernehmen haelt jeden inneren Aufruf auf derselben Werkzeugkette wie den
/// aeusseren — und auf diesem Geraet ueberhaupt auffindbar, denn `cargo` steht
/// hier nicht auf dem Standard-PATH.
///
/// Beide inneren Aufrufe lesen ihn hier: die Uebersetzung in [`uebersetzen`]
/// und das Auffrischen der `Cargo.lock` in `version`. Zwei Arten, `cargo` zu
/// finden, waeren zwei Werkzeugketten in einem Lauf.
pub(crate) fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

/// Wo das fertige Buendel liegt.
///
/// Die eine Stelle, die `target/KRK.app` zusammensetzt: die Montage legt es
/// dorthin, und `beglaubigen` sucht es dort. Ein zweites Zusammensetzen
/// anderswo waere die zweite Wahrheit darueber, wo das Buendel entsteht — und
/// der Weg, auf dem ein Umbenennen des Buendels einen Rufer zuruecklaesst, der
/// ins Leere greift.
#[must_use]
pub(crate) fn buendelpfad(wurzel: &Path) -> PathBuf {
    wurzel.join("target").join(BUENDELNAME)
}

/// Die Projektwurzel.
///
/// Aus dem Manifestordner von `xtask` abgeleitet und nicht aus dem
/// Arbeitsverzeichnis: `cargo xtask` laesst sich aus jedem Unterordner rufen,
/// und das Buendel soll trotzdem immer an derselben Stelle entstehen.
pub(crate) fn wurzel() -> PathBuf {
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

/// Liest den Dateinamen des Symbols aus der Buendelbeschreibung.
///
/// Dieselbe Vorschrift wie bei [`binaername`]: der Name steht in der
/// `Info.plist` und wird von dort gelesen, statt in `bundle.rs` ein zweites Mal
/// zu stehen. macOS sucht das Symbol unter genau dem Namen, den
/// `CFBundleIconFile` nennt; eine Abweichung zwischen beiden waere ein Buendel,
/// das eine `.icns` traegt und trotzdem das Standardsymbol zeigt.
///
/// Die Endung `.icns` darf im Wert fehlen — macOS ergaenzt sie. Diese Funktion
/// ergaenzt sie ebenfalls, damit die geschriebene Datei in beiden Schreibweisen
/// dort landet, wo gesucht wird.
fn symbolname(vorlage: &str) -> Result<String, Abbruch> {
    let name = plist_zeichenkette(vorlage, "CFBundleIconFile").ok_or_else(|| {
        Abbruch::Lauf(
            "resources/Info.plist nennt keinen Schluessel CFBundleIconFile mit einer \
             Zeichenkette. Ohne ihn ist nicht bestimmt, unter welchem Namen das Symbol in \
             Contents/Resources liegen soll, und das Buendel traegt das Standardsymbol einer \
             Anwendung ohne eigenes."
                .to_owned(),
        )
    })?;
    if name.is_empty() {
        return Err(Abbruch::Lauf(
            "CFBundleIconFile in resources/Info.plist ist leer.".to_owned(),
        ));
    }
    if name.ends_with(".icns") {
        Ok(name)
    } else {
        Ok(format!("{name}.icns"))
    }
}

/// Prueft, dass jede Quelldatei aus [`SYMBOLGROESSEN`] im Baum liegt.
///
/// Ohne diese Pruefung faende ein geloeschtes oder umbenanntes PNG erst bei der
/// Montage auf, also nach einem vollstaendigen Uebersetzungslauf.
fn symbolquellen_pruefen(wurzel: &Path) -> Result<(), Abbruch> {
    for (_, quelldatei) in SYMBOLGROESSEN {
        let quelle = wurzel.join(SYMBOLQUELLE).join(quelldatei);
        if !quelle.is_file() {
            return Err(Abbruch::Lauf(format!(
                "{} fehlt. Aus den PNGs unter {SYMBOLQUELLE}/ entsteht das Symbol des Buendels; \
                 ohne sie zeigten Finder und Dock das Standardsymbol einer Anwendung ohne \
                 eigenes. Es wird kein Buendel gebaut.",
                quelle.display()
            )));
        }
    }
    Ok(())
}

/// Erzeugt die `.icns` aus den PNGs unter `iconset/` und legt sie unter `ziel`
/// ab.
///
/// Der Weg ist der von Apple vorgesehene: ein Verzeichnis mit der Endung
/// `.iconset` und den festgelegten Dateinamen darin, danach
/// `iconutil --convert icns`. Die Werkstatt entsteht unter `target/` und wird
/// nach dem Umwandeln abgeraeumt; ein gescheiterter Lauf laesst sie zum
/// Nachsehen stehen, und der naechste Lauf entfernt sie zu Beginn.
fn symbol_bauen(wurzel: &Path, ziel: &Path) -> Result<(), Abbruch> {
    let werkstatt = wurzel.join("target").join(SYMBOLWERKSTATT);
    if werkstatt.exists() {
        fs::remove_dir_all(&werkstatt)
            .map_err(|fehler| schreibfehler("entfernen", &werkstatt, &fehler))?;
    }
    fs::create_dir_all(&werkstatt)
        .map_err(|fehler| schreibfehler("anlegen", &werkstatt, &fehler))?;

    for (im_iconset, quelldatei) in SYMBOLGROESSEN {
        let quelle = wurzel.join(SYMBOLQUELLE).join(quelldatei);
        let hin = werkstatt.join(im_iconset);
        fs::copy(&quelle, &hin).map_err(|fehler| {
            Abbruch::Lauf(format!(
                "{} laesst sich nicht nach {} kopieren: {fehler}",
                quelle.display(),
                hin.display()
            ))
        })?;
    }

    let status = Command::new("iconutil")
        .args(["--convert", "icns", "--output"])
        .arg(ziel)
        .arg(&werkstatt)
        .status()
        .map_err(|fehler| {
            Abbruch::Lauf(format!(
                "iconutil laesst sich nicht starten: {fehler}. Es gehoert zum Basissystem von \
                 macOS und liegt unter /usr/bin/iconutil."
            ))
        })?;
    if !status.success() {
        return Err(Abbruch::Lauf(format!(
            "iconutil ist an {} gescheitert ({status}). Das Werkzeug prueft die Kantenlaenge \
             jeder PNG-Datei gegen ihren Namen im Iconset; die Zuordnung steht bei \
             SYMBOLGROESSEN in xtask/src/bundle.rs.",
            werkstatt.display()
        )));
    }

    fs::remove_dir_all(&werkstatt)
        .map_err(|fehler| schreibfehler("entfernen", &werkstatt, &fehler))?;
    println!("Symbol aus {SYMBOLQUELLE}/ erzeugt: {}", ziel.display());
    Ok(())
}

/// Liest den Wert eines Schluessels aus einer Property-Liste im XML-Format.
///
/// `pub(crate)` seit dem 260820: `beglaubigung` liest damit die Versionszahl
/// aus der `Info.plist` des **gebauten** Buendels, waehrend die drei Rufer
/// hier die Vorlage aus `resources/` lesen. Zwei Leser fuer dasselbe Muster
/// waeren zwei Regeln darueber, was `<key>…</key><string>…</string>` bedeutet.
///
/// Bewusst kein Parser: gebraucht wird ein einziger Wert aus einer Datei, die
/// im selben Projekt liegt und dem Muster `<key>…</key><string>…</string>`
/// folgt. Steht zwischen Schluessel und Wert ein weiterer `<key>`, ist der
/// gesuchte Schluessel nicht mit einer Zeichenkette belegt, und die Funktion
/// liefert nichts, statt den Wert des naechsten Schluessels auszugeben.
pub(crate) fn plist_zeichenkette(plist: &str, schluessel: &str) -> Option<String> {
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
    let cargo = cargo();
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
    fn die_ausgelieferte_plist_nennt_die_symboldatei() {
        assert_eq!(symbolname(AUSGELIEFERTE_PLIST).unwrap(), "KRK.icns");
    }

    #[test]
    fn ein_symbolname_ohne_endung_bekommt_sie() {
        let plist = "<key>CFBundleIconFile</key><string>KRK</string>";
        assert_eq!(symbolname(plist).unwrap(), "KRK.icns");
    }

    #[test]
    fn ohne_cfbundleiconfile_entsteht_kein_buendel() {
        let plist = "<key>CFBundleName</key><string>KRK</string>";
        assert!(symbolname(plist).is_err());
    }

    #[test]
    fn jede_symbolquelle_liegt_im_baum() {
        symbolquellen_pruefen(&wurzel()).unwrap();
    }

    #[test]
    fn kein_eintrag_des_iconsets_kommt_zweimal_vor() {
        // Zwei Eintraege gleichen Namens hiessen, dass eine Kantenlaenge
        // stillschweigend die andere ueberschreibt und `iconutil` die zweite
        // nie sieht.
        let mut namen: Vec<&str> = SYMBOLGROESSEN.iter().map(|(name, _)| *name).collect();
        namen.sort_unstable();
        let anzahl = namen.len();
        namen.dedup();
        assert_eq!(namen.len(), anzahl);
    }

    /// Jede PNG-Datei unter `iconset/` wird gebraucht.
    ///
    /// Die Probe schlaegt an, wenn jemand eine Kantenlaenge dazulegt, ohne sie
    /// zuzuordnen: das PNG laege dann im Baum und kaeme nicht ins Buendel.
    /// Dieselbe Absicht wie bei den Fallunterscheidungen ohne Auffangzweig —
    /// die Ergaenzung soll eine bewusste sein.
    #[test]
    fn jede_png_quelle_wird_gebraucht() {
        let quellen = wurzel().join(SYMBOLQUELLE);
        for eintrag in fs::read_dir(&quellen).unwrap() {
            let pfad = eintrag.unwrap().path();
            if pfad.extension().is_none_or(|endung| endung != "png") {
                continue;
            }
            let name = pfad.file_name().unwrap().to_str().unwrap().to_owned();
            assert!(
                SYMBOLGROESSEN
                    .iter()
                    .any(|(_, quelldatei)| *quelldatei == name),
                "{name} liegt unter {SYMBOLQUELLE}/ und steht in keinem Eintrag von \
                 SYMBOLGROESSEN; es kaeme nicht ins Buendel."
            );
        }
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
