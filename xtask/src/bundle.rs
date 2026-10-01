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
//!     └── Resources/
//!         ├── KRK.icns          das Symbol, aus iconset/ erzeugt
//!         ├── de.lproj/         Kopie von resources/de.lproj/, die Erlaubnistexte
//!         ├── fr.lproj/         Kopie von resources/fr.lproj/
//!         └── en.lproj/         Kopie von resources/en.lproj/
//! ```
//!
//! **Das Symbol liegt nicht im Baum, es entsteht beim Bau.** Die Quelle sind
//! die sieben PNGs unter `iconset/`; `iconutil` macht daraus die `.icns`, und
//! der Dateiname kommt aus `CFBundleIconFile` der `resources/Info.plist`.
//! Warum erzeugt und nicht eingecheckt, steht bei [`SYMBOLGROESSEN`].
//!
//! **Die Sprachordner liegen im Baum und werden kopiert.** Welche es sind,
//! sagt [`SPRACHEN`], und dieselbe Liste muss `CFBundleLocalizations` der
//! `resources/Info.plist` nennen: ein Ordner ohne Eintrag in der Liste wird
//! von Foundation nicht angeboten, ein Eintrag ohne Ordner traegt keine
//! Erlaubnistexte. Beides prueft [`sprachen_pruefen`], bevor ein Verzeichnis
//! entsteht. Was die Ordner tragen und wie macOS die Sprache daraus waehlt,
//! steht an `CFBundleLocalizations` in der `resources/Info.plist`.
//!
//! **Die Reihenfolge ist Absicht.** Alles, was scheitern kann, scheitert bevor
//! ein Verzeichnis entsteht: erst die Versionsersetzung, dann der Name des
//! Binaerprogramms, dann die Symbol- und die Sprachquellen, dann die
//! Signaturidentitaet, und erst danach wird uebersetzt und geschrieben. Ein
//! abgebrochener Lauf hinterlaesst so kein halbes Buendel, und wer die
//! Identitaet noch nicht angelegt hat, erfaehrt es vor und nicht nach einem
//! vollstaendigen Uebersetzungslauf.

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
pub(crate) const PLATZHALTER: &str = "__KRK_VERSION__";

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
/// von macOS, wie `codesign`, ohne das der Buendelbau ohnehin nicht
/// durchlaeuft; es kommt also keine Voraussetzung hinzu, die dieses Projekt
/// nicht schon haette.
///
/// Gerufen wird es als `/usr/bin/iconutil`, also mit vollem Pfad, wie die Regel
/// im Kopf von [`crate`] es fuer ein mitgeliefertes Programm vorsieht. Eine
/// eigene Begruendung braucht das hier nicht; sie stuende an dieser Stelle nur,
/// wenn der Aufruf von der Regel abwiche.
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
/// Die Sprachen, deren `.lproj`-Ordner ins Buendel wandern, als Kennungen.
///
/// Je Kennung liegt `resources/<kennung>.lproj/` im Baum und kommt nach
/// `Contents/Resources/<kennung>.lproj/`. Die Liste ist dieselbe wie
/// `CFBundleLocalizations` in `resources/Info.plist` und dieselbe wie
/// `krk_core::sprache::Sprache::ALLE`; dass die ersten beiden gleich sind,
/// haelt [`sprachen_pruefen`] bei jedem Bau, dass die dritte dazu passt, die
/// Probe `die_sprachen_des_buendels_sind_die_der_tabelle` im Pruefmodul.
/// `xtask` haengt dafuer nicht an `krk-core`: die Kennungen stehen als
/// Zeichenketten da, weil das Bauwerkzeug den Kern nicht uebersetzen soll,
/// und die Probe liest sie aus dem Quelltext.
///
/// Die Reihenfolge ist die der `Info.plist` und entscheidet nichts; welche
/// Sprache gilt, waehlt macOS aus der Sprachwahl des Nutzers.
const SPRACHEN: [&str; 3] = ["de", "fr", "en"];

/// Die Datei, die jeder Sprachordner mindestens traegt: die fuenf
/// Erlaubnistexte des Systemmechanismus fuer Transparenz, Zustimmung und
/// Kontrolle in dieser Sprache.
///
/// macOS liest sie unter genau diesem Namen aus dem Ordner der gewaehlten
/// Sprache; fehlt sie, zeigt der Berechtigungsdialog den Text aus der
/// `Info.plist`, also Deutsch, gleich welche Sprache gewaehlt ist.
const ERLAUBNISTEXTE: &str = "InfoPlist.strings";

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
/// Binaername, Symbolname, die Symbolquellen und die Sprachquellen scheitern
/// hier, vor dem ersten Uebersetzungslauf und vor dem ersten angelegten
/// Verzeichnis. Dass die zehn PNG-Quellen und die drei Sprachordner schon hier
/// geprueft werden und nicht erst bei der Montage, ist derselbe Gedanke: ein
/// fehlendes `iconset/` oder `fr.lproj/` soll vor und nicht nach einem
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
    sprachen_pruefen(&vorlage, &wurzel)?;
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

        // Aus demselben Grund hier und nicht nach der Rueckkehr: ein
        // Sprachordner ausserhalb der Signatur liesse die Beglaubigung
        // scheitern.
        sprachordner_kopieren(&self.wurzel, &resources)?;

        println!("Version {VERSION} in {} eingesetzt.", plist_pfad.display());
        Ok(buendel)
    }

    /// Wo das Binaerprogramm in einem fertigen Buendel liegt.
    ///
    /// Die eine Stelle, die `Contents/MacOS/<CFBundleExecutable>` bildet: die
    /// Montage legt es dorthin, und die Messstrecke ruft es von dort.
    #[must_use]
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
/// **Der Rueckfall auf den blossen Namen ist der Suchpfad**, und damit die
/// Seite, auf die die Regel im Kopf von [`crate`] ein nachinstalliertes
/// Programm stellt. Die Variable davor ist keine zweite Regel, sondern eine
/// genauere Auskunft aus der Umgebung.
///
/// Jeder innere Aufruf liest ihn hier: die Uebersetzung in [`uebersetzen`],
/// das Auffrischen der `Cargo.lock` in `version` und der Ruf nach `krk-bench`
/// in `messen`. Zwei Arten, `cargo` zu finden, waeren zwei Werkzeugketten in
/// einem Lauf; `messen` hat den Ausdruck bis zum 260905 nachgebaut, statt diese
/// Funktion zu rufen
/// (`shared/issues/260826-1448_*_iconutil-wird-ueber-den-suchpfad-gerufen-waehrend-kommentar-und-meldung-usr-bin-iconutil-sagen-und-messen-rs-liest-cargo-ein-zweites-mal.md`).
/// Eine Zahl steht hier nicht: sie waere mit dem naechsten inneren Aufruf
/// falsch. Nachgezaehlt wird mit `grep -rn 'bundle::cargo()' xtask/src`.
#[must_use]
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

/// Der Buendelpfad fuer die Meldungsproben der beiden spaeten Stationen.
///
/// Die Proben in `beglaubigung` und `veroeffentlichung` reichen einen Pfad in
/// eine reine Meldungsfunktion; welcher, ist fuer ihr Urteil gleichgueltig, und
/// gerade deshalb standen dort bis zum 260905 zwei Helfer mit dem
/// ausgeschriebenen Pfad des Referenzgeraets
/// (`shared/issues/260826-1453_*_zwei-pruefhelfer-und-eine-konstante-in-xtask-tragen-den-absoluten-pfad-des-referenzgeraets.md`).
/// Der Weg ueber [`buendelpfad`] und [`wurzel`] liefert denselben Wert auf
/// jedem Geraet und laesst die eine Stelle, die `target/KRK.app`
/// zusammensetzt, die eine bleiben.
#[cfg(test)]
#[must_use]
pub(crate) fn pruefbuendel() -> PathBuf {
    buendelpfad(&wurzel())
}

/// Die Projektwurzel.
///
/// Aus dem Manifestordner von `xtask` abgeleitet und nicht aus dem
/// Arbeitsverzeichnis: `cargo xtask` laesst sich aus jedem Unterordner rufen,
/// und das Buendel soll trotzdem immer an derselben Stelle entstehen.
#[must_use]
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

/// Wo der Sprachordner einer Kennung im Baum liegt.
#[must_use]
fn sprachquelle(wurzel: &Path, kennung: &str) -> PathBuf {
    wurzel.join("resources").join(format!("{kennung}.lproj"))
}

/// Prueft, dass die Sprachen des Buendels an beiden Stellen dieselben sind
/// und jede ihre Erlaubnistexte im Baum hat.
///
/// Die eine Stelle ist `CFBundleLocalizations` in der Buendelbeschreibung,
/// die andere [`SPRACHEN`]; sie muessen dieselbe Menge nennen, in derselben
/// Reihenfolge, damit der Vergleich keine zweite Regel darueber braucht, was
/// „dieselbe Liste“ heisst. Dazu muss je Kennung
/// `resources/<kennung>.lproj/InfoPlist.strings` liegen. Alles drei scheitert
/// hier, vor dem ersten Uebersetzungslauf, wie die Symbolquellen.
fn sprachen_pruefen(vorlage: &str, wurzel: &Path) -> Result<(), Abbruch> {
    let genannt = plist_zeichenkettenliste(vorlage, "CFBundleLocalizations").ok_or_else(|| {
        Abbruch::Lauf(
            "resources/Info.plist nennt keinen Schluessel CFBundleLocalizations mit einer Liste \
             von Zeichenketten. Ohne ihn ist KRK fuer Foundation ein englisches Programm, \
             gleich welche Sprache der Nutzer gewaehlt hat."
                .to_owned(),
        )
    })?;
    if genannt != SPRACHEN {
        return Err(Abbruch::Lauf(format!(
            "CFBundleLocalizations in resources/Info.plist nennt {genannt:?}, SPRACHEN in \
             xtask/src/bundle.rs nennt {SPRACHEN:?}. Beide Listen muessen gleich sein: ein \
             Ordner ohne Eintrag wird nicht angeboten, ein Eintrag ohne Ordner traegt keine \
             Erlaubnistexte. Es wird kein Buendel gebaut."
        )));
    }
    for kennung in SPRACHEN {
        let texte = sprachquelle(wurzel, kennung).join(ERLAUBNISTEXTE);
        if !texte.is_file() {
            return Err(Abbruch::Lauf(format!(
                "{} fehlt. Jeder Sprachordner traegt die Erlaubnistexte in seiner Sprache; ohne \
                 sie zeigte der Berechtigungsdialog in dieser Sprache den deutschen Text aus der \
                 Info.plist. Es wird kein Buendel gebaut.",
                texte.display()
            )));
        }
    }
    Ok(())
}

/// Kopiert jeden Sprachordner aus [`SPRACHEN`] nach `Contents/Resources/`.
///
/// Kopiert werden die gewoehnlichen Dateien des Ordners, flach; ein
/// Unterordner darin waere ein Fall, den dieses Projekt nicht hat, und er
/// bricht ab, statt still zu fehlen. Dass jeder Ordner da ist und die
/// Erlaubnistexte traegt, hat [`sprachen_pruefen`] vor dem Uebersetzen
/// gehalten.
fn sprachordner_kopieren(wurzel: &Path, resources: &Path) -> Result<(), Abbruch> {
    for kennung in SPRACHEN {
        let quelle = sprachquelle(wurzel, kennung);
        let ziel = resources.join(format!("{kennung}.lproj"));
        fs::create_dir_all(&ziel).map_err(|fehler| schreibfehler("anlegen", &ziel, &fehler))?;
        let eintraege = fs::read_dir(&quelle).map_err(|fehler| {
            Abbruch::Lauf(format!("{} ist nicht lesbar: {fehler}", quelle.display()))
        })?;
        for eintrag in eintraege {
            let eintrag = eintrag.map_err(|fehler| {
                Abbruch::Lauf(format!(
                    "ein Eintrag in {} ist nicht lesbar: {fehler}",
                    quelle.display()
                ))
            })?;
            let von = eintrag.path();
            if !von.is_file() {
                return Err(Abbruch::Lauf(format!(
                    "{} ist keine gewoehnliche Datei. Ein Sprachordner traegt allein Dateien; \
                     was darunter liegt, kaeme nicht ins Buendel.",
                    von.display()
                )));
            }
            let hin = ziel.join(eintrag.file_name());
            fs::copy(&von, &hin).map_err(|fehler| {
                Abbruch::Lauf(format!(
                    "{} laesst sich nicht nach {} kopieren: {fehler}",
                    von.display(),
                    hin.display()
                ))
            })?;
        }
    }
    println!(
        "Sprachordner nach {} kopiert: {}",
        resources.display(),
        SPRACHEN.join(", ")
    );
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

    let status = Command::new("/usr/bin/iconutil")
        .args(["--convert", "icns", "--output"])
        .arg(ziel)
        .arg(&werkstatt)
        .status()
        .map_err(|fehler| {
            Abbruch::Lauf(format!(
                "/usr/bin/iconutil laesst sich nicht starten: {fehler}. Es gehoert zum \
                 Basissystem von macOS und wird deshalb nach der Aufrufregel im Kopf von \
                 xtask/src/main.rs mit vollem Pfad gerufen; fehlt es dort, ist die \
                 Installation des Systems unvollstaendig."
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
#[must_use]
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

/// Liest die Zeichenketten einer Liste aus einer Property-Liste im XML-Format.
///
/// Das Gegenstueck zu [`plist_zeichenkette`] fuer das Muster
/// `<key>…</key><array><string>…</string>…</array>`, mit derselben Enge:
/// steht zwischen Schluessel und `<array>` ein weiterer `<key>`, ist der
/// gesuchte Schluessel nicht mit einer Liste belegt, und die Funktion liefert
/// nichts. Gelesen werden allein die `<string>`-Glieder bis zum `</array>`;
/// ein Glied anderen Typs stuende in einer Datei dieses Projekts nicht, und
/// es wuerde uebergangen statt gemeldet.
#[must_use]
fn plist_zeichenkettenliste(plist: &str, schluessel: &str) -> Option<Vec<String>> {
    let marke = format!("<key>{schluessel}</key>");
    let hinter_schluessel = plist.split_once(&marke)?.1;
    let (zwischenraum, hinter_beginn) = hinter_schluessel.split_once("<array>")?;
    if zwischenraum.contains("<key>") {
        return None;
    }
    let (liste, _) = hinter_beginn.split_once("</array>")?;
    let glieder = liste
        .split("<string>")
        .skip(1)
        .filter_map(|stueck| stueck.split_once("</string>"))
        .map(|(wert, _)| wert.trim().to_owned())
        .collect();
    Some(glieder)
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
#[must_use]
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

    /// `CARGO` wird an genau einer Stelle gelesen, und die ist [`cargo`].
    ///
    /// Die Zusage des Doc-Kommentars von [`cargo`] steht sonst als Prosa da und
    /// haelt nichts: `messen.rs` hat den Ausdruck bis zum 260905 nachgebaut,
    /// und keine Probe hat es gemerkt. Die Nadel steht als `concat!`, weil die
    /// Probe in derselben Datei liegt, die sie liest; ausgeschrieben zaehlte sie
    /// sich selbst mit.
    #[test]
    fn die_umgebungsvariable_cargo_wird_an_genau_einer_stelle_gelesen() {
        let nadel = concat!("env::var(\"", "CARGO\")");
        let ordner = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stellen = Vec::new();
        let mut eintraege: Vec<PathBuf> = fs::read_dir(&ordner)
            .expect("xtask/src ist lesbar")
            .map(|eintrag| eintrag.expect("der Eintrag ist lesbar").path())
            .filter(|pfad| pfad.extension().is_some_and(|endung| endung == "rs"))
            .collect();
        eintraege.sort();
        for datei in eintraege {
            let inhalt = fs::read_to_string(&datei).expect("die Datei ist lesbar");
            for _ in 0..inhalt.matches(nadel).count() {
                stellen.push(datei.clone());
            }
        }
        assert_eq!(stellen.len(), 1, "CARGO wird gelesen in {stellen:?}");
        assert!(
            stellen[0].ends_with("bundle.rs"),
            "{:?} statt bundle.rs",
            stellen[0]
        );
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

    #[test]
    fn eine_liste_wird_glied_fuer_glied_gelesen() {
        let plist = "<key>CFBundleLocalizations</key>\n\t<array>\n\t\t<string>de</string>\n\t\t<string>fr</string>\n\t</array>\n<key>X</key><string>y</string>";
        assert_eq!(
            plist_zeichenkettenliste(plist, "CFBundleLocalizations").unwrap(),
            vec!["de".to_owned(), "fr".to_owned()]
        );
    }

    #[test]
    fn ein_schluessel_ohne_liste_liefert_nicht_die_naechste_liste() {
        let plist = "<key>CFBundleName</key><string>KRK</string>\n<key>CFBundleLocalizations</key><array><string>de</string></array>";
        assert!(plist_zeichenkettenliste(plist, "CFBundleName").is_none());
        assert!(plist_zeichenkettenliste(plist, "Fehlt").is_none());
    }

    /// Die Schluessel der Erlaubnistexte, wie die Buendelbeschreibung sie
    /// fuehrt: jeder `<key>`, der auf `NS` beginnt und auf `UsageDescription`
    /// endet.
    fn erlaubnisschluessel(plist: &str) -> Vec<String> {
        plist
            .split("<key>")
            .skip(1)
            .filter_map(|stueck| stueck.split_once("</key>"))
            .map(|(schluessel, _)| schluessel.trim().to_owned())
            .filter(|schluessel| {
                schluessel.starts_with("NS") && schluessel.ends_with("UsageDescription")
            })
            .collect()
    }

    /// Die Paare einer `.strings`-Datei im Format `"Schluessel" = "Text";`.
    ///
    /// Bewusst kein Parser, wie [`plist_zeichenkette`]: die drei Dateien
    /// liegen im selben Projekt, je Paar eine Zeile, ohne maskierte
    /// Anfuehrungszeichen. Eine Zeile, die nicht mit `"` beginnt, ist ein
    /// Kommentar oder leer.
    fn strings_paare(inhalt: &str) -> Vec<(String, String)> {
        inhalt
            .lines()
            .filter_map(|zeile| {
                let zeile = zeile.trim();
                let ohne_erstes = zeile.strip_prefix('"')?;
                let (schluessel, rest) = ohne_erstes.split_once('"')?;
                let (_, wert) = rest.split_once("= \"")?;
                let wert = wert.strip_suffix("\";")?;
                Some((schluessel.to_owned(), wert.to_owned()))
            })
            .collect()
    }

    fn erlaubnistexte(kennung: &str) -> String {
        let pfad = sprachquelle(&wurzel(), kennung).join(ERLAUBNISTEXTE);
        fs::read_to_string(&pfad)
            .unwrap_or_else(|fehler| panic!("{} ist lesbar: {fehler}", pfad.display()))
    }

    /// `resources/` traegt genau die `.lproj`-Ordner aus [`SPRACHEN`].
    ///
    /// Dieselbe Absicht wie `jede_png_quelle_wird_gebraucht`: ein vierter
    /// Ordner im Baum, der in keiner Liste steht, kaeme nicht ins Buendel, und
    /// die Ergaenzung soll eine bewusste sein.
    #[test]
    fn resources_traegt_genau_die_sprachordner_aus_sprachen() {
        let mut im_baum: Vec<String> = fs::read_dir(wurzel().join("resources"))
            .unwrap()
            .map(|eintrag| eintrag.unwrap().path())
            .filter(|pfad| pfad.is_dir())
            .filter_map(|pfad| {
                pfad.file_name()?
                    .to_str()?
                    .strip_suffix(".lproj")
                    .map(str::to_owned)
            })
            .collect();
        im_baum.sort_unstable();
        let mut erwartet: Vec<String> = SPRACHEN.iter().map(|s| (*s).to_owned()).collect();
        erwartet.sort_unstable();
        assert_eq!(im_baum, erwartet);
    }

    #[test]
    fn die_ausgelieferte_plist_nennt_genau_die_sprachen() {
        assert_eq!(
            plist_zeichenkettenliste(AUSGELIEFERTE_PLIST, "CFBundleLocalizations").unwrap(),
            SPRACHEN
        );
        sprachen_pruefen(AUSGELIEFERTE_PLIST, &wurzel()).unwrap();
    }

    #[test]
    fn ohne_sprachordner_entsteht_kein_buendel() {
        let ohne = wurzel().join("target").join("ohne-sprachordner");
        assert!(matches!(
            sprachen_pruefen(AUSGELIEFERTE_PLIST, &ohne),
            Err(Abbruch::Lauf(_))
        ));
    }

    #[test]
    fn eine_abweichende_sprachliste_bricht_ab() {
        let plist =
            "<key>CFBundleLocalizations</key><array><string>de</string><string>en</string></array>";
        assert!(matches!(
            sprachen_pruefen(plist, &wurzel()),
            Err(Abbruch::Lauf(_))
        ));
    }

    /// Die Entwicklungsregion ist `en`, und das ist der Rueckfall fuer jede
    /// nicht angebotene Sprache; warum, steht an dem Schluessel in der
    /// `resources/Info.plist`.
    #[test]
    fn die_entwicklungsregion_ist_en() {
        assert_eq!(
            plist_zeichenkette(AUSGELIEFERTE_PLIST, "CFBundleDevelopmentRegion").unwrap(),
            "en"
        );
    }

    /// Jede `InfoPlist.strings` nennt genau die Erlaubnisschluessel der
    /// Buendelbeschreibung, jeden einmal, und keinen anderen.
    #[test]
    fn jede_erlaubnistextdatei_nennt_genau_die_schluessel_der_plist() {
        let mut aus_plist = erlaubnisschluessel(AUSGELIEFERTE_PLIST);
        aus_plist.sort_unstable();
        assert!(
            !aus_plist.is_empty(),
            "die Plist fuehrt keinen Erlaubnisschluessel"
        );
        for kennung in SPRACHEN {
            let paare = strings_paare(&erlaubnistexte(kennung));
            let mut schluessel: Vec<String> = paare.iter().map(|(s, _)| s.clone()).collect();
            schluessel.sort_unstable();
            assert_eq!(schluessel, aus_plist, "{kennung}.lproj/{ERLAUBNISTEXTE}");
            for (schluessel, wert) in &paare {
                assert!(!wert.trim().is_empty(), "{kennung}: {schluessel} ist leer");
            }
        }
    }

    /// Die deutsche Datei traegt die Texte der Buendelbeschreibung Zeichen
    /// fuer Zeichen: beide sind derselbe Text an zwei Stellen, und die Probe
    /// ist das, was die zwei Stellen aneinander haelt.
    #[test]
    fn die_deutschen_erlaubnistexte_sind_die_der_plist() {
        for (schluessel, wert) in strings_paare(&erlaubnistexte("de")) {
            assert_eq!(
                plist_zeichenkette(AUSGELIEFERTE_PLIST, &schluessel).as_deref(),
                Some(wert.as_str()),
                "{schluessel}"
            );
        }
    }

    /// [`SPRACHEN`] nennt dieselben Kennungen wie `Sprache::kennung` im Kern.
    ///
    /// `xtask` haengt nicht an `krk-core`, also liest die Probe die Kennungen
    /// aus dem Quelltext von `crates/krk-core/src/sprache/mod.rs`: die
    /// Literale im Rumpf von `fn kennung`, bis zur schliessenden Klammer der
    /// Funktion. Eine vierte Sprache im Kern haelt den Bau des Buendels damit
    /// an, bis sie einen Ordner hat.
    #[test]
    fn die_sprachen_des_buendels_sind_die_der_tabelle() {
        let quelle = wurzel()
            .join("crates")
            .join("krk-core")
            .join("src")
            .join("sprache")
            .join("mod.rs");
        let inhalt = fs::read_to_string(&quelle).unwrap();
        let (_, hinter) = inhalt
            .split_once("fn kennung(")
            .expect("Sprache::kennung steht im Kern");
        let (rumpf, _) = hinter.split_once("\n    }").expect("die Funktion endet");
        let mut im_kern: Vec<&str> = rumpf.split('"').skip(1).step_by(2).collect();
        im_kern.sort_unstable();
        let mut hier: Vec<&str> = SPRACHEN.to_vec();
        hier.sort_unstable();
        assert_eq!(hier, im_kern);
    }
}
