//! Das kurze Skript, mit dem KRK eine Datei im Terminal bearbeiten laesst:
//! die Wahl zwischen `vim` und `sudo -e`, die Pruefung der Datei und das
//! Anlegen des Skripts.
//!
//! `opt+cmd+e` in der Dateiliste oeffnet die Datei unter dem Cursor in der
//! eingestellten Terminal-Anwendung (`settings.toml`, `terminal`). KRK waehlt
//! selbst, welches Programm sie bekommt; eine zweite Taste gibt es nicht
//! (Nutzerentscheid vom 261029 im Narrativ des Arbeitspakets
//! `261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen`). Dieses Modul
//! traegt den Teil davon, der ohne AppKit steht: [`befehl_fuer`] waehlt,
//! [`skripttext`] schreibt den Text, [`vorbereiten`] fragt die Platte und legt
//! die Datei an. Die Uebergabe an die Terminal-Anwendung steht in `krk-ui`.
//!
//! # Die Regel der Wahl
//!
//! Gefragt werden zwei Dinge, beide mit `access(2)` und `W_OK` ueber die eine
//! Huelle [`crate::verzeichnis::sys::schreiben_erlaubt`]: ob der Nutzer die
//! Datei schreiben darf und ob er ihren Ordner beschreiben darf.
//!
//! - Datei schreibbar: `vim -- <datei>`.
//! - Datei nicht schreibbar, Ordner schreibbar: ebenfalls `vim -- <datei>`.
//! - sonst: `sudo -e -- <datei>`.
//!
//! Die zweite Zeile ist der Zusatz, den der Nutzer am 261029 gewaehlt hat
//! (`261029-0957-geht-eine-nicht-schreibbare-datei-in-einem-beschreibbaren-ordner-an-sudo-e-oder-an-vim.md`,
//! Moeglichkeit 2). Der Grund: `sudo -e` bearbeitet keine Datei in einem
//! Ordner, den der aufrufende Nutzer beschreiben darf, und weigert sich dort
//! sicher. Ohne den Zusatz fuehrte jede eigene schreibgeschuetzte Datei im
//! eigenen Ordner in diese Weigerung.
//!
//! **Der Preis des Zusatzes ist eine Wirkung, die der Nutzer kennen muss.**
//! Eine Datei, die root gehoert und in einem Ordner des Nutzers liegt, oeffnet
//! in vim und nicht als root. vim zeigt sie als schreibgeschuetzt, und `:w!`
//! ersetzt sie durch eine Datei, die dem Nutzer gehoert, weil der Ordner das
//! erlaubt. Als root geoeffnet wird allein eine Datei, deren Ordner der Nutzer
//! ebenfalls nicht beschreiben darf, etwa unter `/etc`. Eine schreibbare Datei
//! als root zu oeffnen, geht mit diesem Befehl nicht.
//!
//! [`befehl_fuer`] ist rein und kennt kein Dateisystem; [`vorbereiten`] ist
//! sein einziger Rufer im Betriebscode, und eine Probe haelt das fest. **Ein
//! Fehlschlag der Frage ist kein Nein**: er kommt als [`Skriptfehler::Pruefen`]
//! zurueck, und es startet weder vim noch sudo. Wer aus einem unklaren Befund
//! `sudo -e` macht, fragt nach einem Passwort, ohne dass das noetig war.
//!
//! Die Antwort gilt im Augenblick der Frage. Aendert sich das Recht bis zum
//! Start, oeffnet vim die Datei schreibgeschuetzt oder sudo fragt unnoetig;
//! beides sieht der Nutzer im Terminal, und eine zweite Pruefung im Skript
//! waere eine zweite Regel neben dieser einen.
//!
//! KRK haelt, speichert und uebergibt kein Passwort. `sudo` fragt es selbst im
//! Terminal.
//!
//! # Die Form des Skripts
//!
//! ```sh
//! #!/bin/sh
//! rm -f -- "$0"
//! cd -- '<ordner>' || exit 1
//! exec vim -- '<datei>'
//! ```
//!
//! Fuer `sudo -e` lautet die letzte Zeile `exec sudo -e -- '<datei>'`. Jeder
//! Pfad steht in einfachen Anfuehrungszeichen, ein `'` darin wird zu `'\''`.
//! Damit sind Leerzeichen, `"`, `$`, Backtick, `;`, `*` und ein Zeilenumbruch
//! ohne Sonderfall abgedeckt, und das `--` haelt einen Namen mit fuehrendem
//! `-` davon ab, als Schalter gelesen zu werden. Ein Pfad ohne gueltiges UTF-8
//! wird abgewiesen und nie geglaettet, ebenso ein Pfad, der nicht absolut ist
//! oder keinen Elternordner hat.
//!
//! # Aufgeraeumt wird vom Skript selbst
//!
//! `rm -f -- "$0"` ist die erste Handlung, und die Shell haelt die geoeffnete
//! Datei weiter. KRK fuehrt keine Liste offener Skripte. Fuehrt die
//! Terminal-Anwendung das Skript nie aus, bleibt eine Datei mit `0700` im
//! nutzereigenen Temporaerverzeichnis liegen; scheitert schon die Uebergabe,
//! verwirft der Rufer sie sofort mit [`verwerfen`].
//!
//! # Ort und Name
//!
//! [`skriptverzeichnis`] ist die eine Stelle mit `std::env::temp_dir()`, auf
//! macOS `$TMPDIR` unter `/var/folders/…/T/`, ein Ordner mit `0700`, der dem
//! Nutzer gehoert. Der Name ist `krk-terminal-<pid>-<nanosekunden>-<zaehler>.command`,
//! angelegt mit `create_new` und Modus `0o700`. Ein Name, den es schon gibt,
//! ist ein Fehler und wird nie ueberschrieben. Das Modul traegt kein `Drop`
//! und raeumt keinen Ordner ab; es ist kein Pruefordner.
//!
//! # Allein auf einem Arbeitsfaden
//!
//! [`vorbereiten`] stellt Systemaufrufe (`metadata`, zweimal `access(2)`, das
//! Schreiben des Skripts) und wird allein auf einem Arbeitsfaden gerufen: die
//! Datei kann auf einer eingehaengten Freigabe liegen, und eine haengende
//! Freigabe hielte sonst die Ereignisschleife an. Die eine Ausnahme ist
//! [`verwerfen`] nach einer gescheiterten Uebergabe; es entfernt eine Datei,
//! die KRK selbst im Temporaerverzeichnis angelegt hat, und das liegt auf der
//! Startplatte.
//!
//! # Warum ein Skript und kein AppleScript
//!
//! Ein AppleScript an Terminal.app braeuchte die Automation-Berechtigung,
//! baende den Befehl an eine Anwendung und liesse die eingestellte
//! Buendelkennung fuer ihn leerlaufen. Das Skript geht ueber dieselbe Huelle
//! um `NSWorkspace`, ueber die `ctrl+o` einen Ordner uebergibt
//! (`261029-0920-wie-kommt-ein-befehl-wie-vim-datei-in-die-eingestellte-terminal-anwendung.md`,
//! Moeglichkeit 1). Ob eine andere Anwendung als Terminal.app eine
//! `.command`-Datei ausfuehrt, entscheidet sie selbst, und KRK kann es nicht
//! vorab feststellen.

use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write as _};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::sprache::{Text, satz, text};
use crate::verzeichnis::sys;

/// Welches Programm das Skript im Terminal startet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terminalbefehl {
    /// `vim -- <datei>`.
    Vim,
    /// `sudo -e -- <datei>`; das Passwort fragt sudo im Terminal.
    SudoBearbeiten,
}

/// Was [`vorbereiten`] angelegt hat: das Skript und das Programm darin.
#[derive(Debug)]
pub struct Vorbereitet {
    /// Der Pfad des angelegten Skripts, Modus `0o700`.
    pub skript: PathBuf,
    /// Das Programm, das das Skript startet.
    pub befehl: Terminalbefehl,
}

/// Warum kein Skript entstanden ist.
#[derive(Debug)]
pub enum Skriptfehler {
    /// Der Pfad der Datei ist kein gueltiges UTF-8.
    KeinUtf8,
    /// Der Pfad ist nicht absolut oder hat keinen Elternordner.
    NichtAbsolut,
    /// Die Datei gibt es nicht (mehr), oder `metadata` scheitert an ihr.
    Fehlt,
    /// Unter dem Pfad steht ein Ordner.
    IstOrdner,
    /// Ob Datei oder Ordner beschreibbar sind, liess sich nicht feststellen.
    Pruefen(io::Error),
    /// Das Skript liess sich nicht anlegen oder schreiben.
    Schreiben(io::Error),
}

impl Skriptfehler {
    /// Der Satz fuer die Statuszeile, aus der Sprachtabelle.
    #[must_use]
    pub fn meldung(&self) -> String {
        match self {
            Self::KeinUtf8 => text(Text::TerminalPfadOhneUtf8).to_owned(),
            Self::NichtAbsolut => text(Text::TerminalPfadNichtAbsolut).to_owned(),
            Self::Fehlt => text(Text::TerminalDateiFehlt).to_owned(),
            Self::IstOrdner => text(Text::TerminalIstOrdner).to_owned(),
            Self::Pruefen(fehler) => satz(Text::TerminalSchreibrechtUnklar, &[("grund", fehler)]),
            Self::Schreiben(fehler) => {
                satz(Text::TerminalSkriptNichtAngelegt, &[("grund", fehler)])
            }
        }
    }
}

/// Die Wahl: `vim`, wenn der Nutzer die Datei oder ihren Ordner beschreiben
/// darf, sonst `sudo -e`. Rein, ohne Dateisystem; die Regel und ihr Preis
/// stehen im Modulkopf.
#[must_use]
pub fn befehl_fuer(datei_schreibbar: bool, ordner_schreibbar: bool) -> Terminalbefehl {
    if datei_schreibbar || ordner_schreibbar {
        Terminalbefehl::Vim
    } else {
        Terminalbefehl::SudoBearbeiten
    }
}

/// Der Pfad als UTF-8 und sein Elternordner, oder der Grund, aus dem er
/// keinen Platz im Skript hat. Stellt keinen Systemaufruf.
fn gepruefter_pfad(datei: &Path) -> Result<(&str, &str), Skriptfehler> {
    let ganz = datei.to_str().ok_or(Skriptfehler::KeinUtf8)?;
    if !datei.is_absolute() {
        return Err(Skriptfehler::NichtAbsolut);
    }
    let ordner = datei
        .parent()
        .and_then(Path::to_str)
        .ok_or(Skriptfehler::NichtAbsolut)?;
    Ok((ganz, ordner))
}

/// Ein Wort in einfachen Anfuehrungszeichen fuer `/bin/sh`; ein `'` darin wird
/// zu `'\''`.
fn gequotet(wort: &str) -> String {
    format!("'{}'", wort.replace('\'', r"'\''"))
}

/// Der Text des Skripts fuer `befehl` an `datei`, in der Form aus dem
/// Modulkopf. Rein: stellt keinen Systemaufruf.
///
/// # Errors
///
/// [`Skriptfehler::KeinUtf8`] fuer einen Pfad ohne gueltiges UTF-8,
/// [`Skriptfehler::NichtAbsolut`] fuer einen relativen Pfad oder einen ohne
/// Elternordner.
pub fn skripttext(befehl: Terminalbefehl, datei: &Path) -> Result<String, Skriptfehler> {
    let (ganz, ordner) = gepruefter_pfad(datei)?;
    let programm = match befehl {
        Terminalbefehl::Vim => "vim",
        Terminalbefehl::SudoBearbeiten => "sudo -e",
    };
    let mut skript = String::from("#!/bin/sh\nrm -f -- \"$0\"\n");
    // In einen String zu schreiben scheitert nicht.
    let _ = writeln!(skript, "cd -- {} || exit 1", gequotet(ordner));
    let _ = writeln!(skript, "exec {programm} -- {}", gequotet(ganz));
    Ok(skript)
}

/// Das Verzeichnis, in dem die Skripte entstehen: die eine Stelle mit
/// `std::env::temp_dir()`.
#[must_use]
pub fn skriptverzeichnis() -> PathBuf {
    std::env::temp_dir()
}

/// Die Laufnummer, die zwei Skripte derselben Nanosekunde trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein frischer Name `krk-terminal-<pid>-<nanosekunden>-<zaehler>.command`.
fn skriptname() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |dauer| dauer.as_nanos());
    let nummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
    format!(
        "krk-terminal-{}-{nanos}-{nummer}.command",
        std::process::id()
    )
}

/// Prueft `datei`, waehlt das Programm und legt das Skript in `verzeichnis`
/// an. Stellt Systemaufrufe und wird allein auf einem Arbeitsfaden gerufen.
///
/// Die Reihenfolge: (a) die Pfadpruefung aus [`skripttext`] vor jedem
/// Systemaufruf; (b) `fs::metadata`, das einer Verknuepfung folgt; (c)
/// [`sys::schreiben_erlaubt`] an der Datei und an ihrem Ordner, beide
/// gefragt, damit die Wahl zwei Antworten und keine Annahme sieht; (d)
/// [`befehl_fuer`]; (e) [`skripttext`]; (f) das Anlegen mit `create_new` und
/// Modus `0o700`.
///
/// # Errors
///
/// [`Skriptfehler::KeinUtf8`] und [`Skriptfehler::NichtAbsolut`] aus (a),
/// [`Skriptfehler::Fehlt`] und [`Skriptfehler::IstOrdner`] aus (b),
/// [`Skriptfehler::Pruefen`] aus (c) und [`Skriptfehler::Schreiben`] aus
/// (f). In jedem Fehlerfall bleibt kein Skript zurueck.
pub fn vorbereiten(verzeichnis: &Path, datei: &Path) -> Result<Vorbereitet, Skriptfehler> {
    let (_, ordner) = gepruefter_pfad(datei)?;
    let angaben = fs::metadata(datei).map_err(|_| Skriptfehler::Fehlt)?;
    if angaben.is_dir() {
        return Err(Skriptfehler::IstOrdner);
    }
    let datei_schreibbar = sys::schreiben_erlaubt(datei).map_err(Skriptfehler::Pruefen)?;
    let ordner_schreibbar =
        sys::schreiben_erlaubt(Path::new(ordner)).map_err(Skriptfehler::Pruefen)?;
    let befehl = befehl_fuer(datei_schreibbar, ordner_schreibbar);
    let inhalt = skripttext(befehl, datei)?;

    let skript = verzeichnis.join(skriptname());
    let mut ziel = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o700)
        .open(&skript)
        .map_err(Skriptfehler::Schreiben)?;
    if let Err(fehler) = ziel.write_all(inhalt.as_bytes()) {
        drop(ziel);
        verwerfen(&skript);
        return Err(Skriptfehler::Schreiben(fehler));
    }
    Ok(Vorbereitet { skript, befehl })
}

/// Entfernt ein Skript, das nicht ausgefuehrt wird. Ein Fehlschlag bleibt
/// still: das Skript liegt dann im privaten Temporaerverzeichnis mit `0700`,
/// und eine Meldung darueber haette der Nutzer nicht verlangt.
pub fn verwerfen(skript: &Path) {
    let _ = fs::remove_file(skript);
}
