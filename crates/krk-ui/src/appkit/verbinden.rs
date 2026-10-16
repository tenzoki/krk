//! Wie ein Klick auf eine Verbindung die Verbindung herstellt (261016,
//! Schritt 6 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//!
//! ```text
//!  Hauptfaden                      Faden krk-verbinden (fern::faden, mit Pfand)
//!  herstellen(pfad, name, mittel, register)
//!                                ──> tresorweg::verbindung_vorbereiten
//!                                       │  Einhaengen: NetFSMountURLSync(url, konto, passwort)
//!                                       │              EEXIST: Einhaengepunkt suchen
//!                                       │  Uebergeben: nur die URL
//!                                       │  Sftp: register.zugang(kennung, …)
//!                                       │          lebend: nehmen
//!                                       │          sonst: rechnerschluessel::vorpruefen,
//!                                       │                 anmeldehilfe::passwort_reichen,
//!                                       │                 ssh_plan, Gespraech
//!                                       │        zugang.startordner(startpfad)
//!                                       v
//!  fertig(mtm, Verbindungsergebnis) <── DispatchQueue::main()
//!  uebergeben(url) ──> NSWorkspace.openURL:
//!
//!  Hauptfaden                      Faden krk-tresor
//!  anhaengen(pfad, entwurf, mittel) ──> tresorweg::anhaengen
//!  fertig(mtm, entwurf, ausgang)   <── DispatchQueue::main()
//! ```
//!
//! **Der zweite Weg, „Verbinden…“, haengt hier allein seinen Faden an**
//! ([`anhaengen`]): der Schreibgang selbst steht in
//! `crate::tresorweg::anhaengen`, und der Entwurf reist zurueck, damit das
//! Blatt mit denselben Feldern wieder aufgehen kann.
//!
//! Die eine Frage dieses Moduls: **wie kommt eine Verbindung beim System an,
//! ohne den Hauptfaden aufzuhalten, und wie kommt die Antwort zurueck.** Der
//! Zuschnitt ist der von [`super::auswurf`]: eine sichere Huelle je Aufruf,
//! was sie verlaesst, ist ein gewoehnlicher Rust-Wert, und die Antwort kommt
//! ueber die Hauptschlange. **Welcher Eintrag zu welchem Plan wird, steht
//! nicht hier**, sondern in `crate::tresorweg::verbindung_vorbereiten` und
//! darunter im Kern; was danach auf dem Schirm geschieht, beim
//! Anwendungsdelegierten.
//!
//! # Das Passwort verlaesst KRK an zwei Stellen: hier an NetFS, und an `ssh`
//!
//! **Ein Rufer von `Passwort::preisgeben` steht in [`einhaengen`], der zweite
//! in `crate::anmeldehilfe::passwort_reichen`**, und die Probe
//! `das_passwort_wird_allein_an_netfs_und_ssh_preisgegeben` haelt beide am
//! Quelltext der Kisten (Entscheidung 11 des Plans der sechs Felder, Entscheid
//! `261016-1947-darf-das-passwort-einer-verbindung-krk-ausser-an-netfs-auch-an-ssh-gehen.md`).
//! Der Sftp-Zweig dieser Datei reicht das Passwort als `Passwort` an die
//! Anmeldehilfe weiter und sieht seinen Text nie. Der herausgegebene Text geht
//! unmittelbar in ein `NSString` und von dort als Argument `passwd` an
//! `NetFSMountURLSync`; er steht in keiner Meldung, keiner URL und keinem
//! Protokoll. Die Uebergabe an das System ([`uebergeben`]) bekommt nie ein
//! Passwort, denn [`crate::tresorweg::Verbindungsplan::Uebergeben`] traegt
//! keines.
//!
//! # sftp: KRK spricht selbst, und der Hauptfaden wartet auf nichts
//!
//! Eine sftp-Verbindung wird kein Einhaengepunkt, sondern ein ferner Ort
//! (`krk_core::ort::FernerOrt`) auf einem Zugang je Server
//! (`krk_core::fern::zugang`). **Der Faden `krk-verbinden` startet ueber
//! `fern::faden` und haelt damit das Pfand**, ohne das keine Anfrage an einen
//! Server uebersetzt; das gilt fuer jedes Protokoll, die anderen Zweige
//! uebergehen das Pfand. Das Register der Zugaenge reist als Abschrift seines
//! Griffs mit; gelesen und geschrieben wird es allein auf diesem Faden.
//!
//! **Woran `ssh` scheiterte, liest [`trennungsart`] aus den Zeilen seiner
//! Fehlerausgabe**, und nur bei Ausgang 255, mit dem `ssh` eigene Fehler
//! meldet. OpenSSH uebersetzt seine Meldungen nicht, also tragen sie in jeder
//! Sprache denselben Wortlaut. Das ist eine Bequemlichkeit und keine Zusage:
//! eine Zeile, die keiner der Listen entspricht, geht unveraendert in die
//! Statuszeile, und eine falsche Einordnung verliert deshalb nie den Grund.
//!
//! **Dieser Weg steht neben und nicht unter der Regel um
//! `Editormodell::haelt_geheimnisse`.** Jene Regel gilt Editorinhalt; der
//! Klick liest den gesicherten Stand der Platte auf seinem eigenen Faden, und
//! er ist eine Ausfuhr durch den Nutzer an einen festen Empfaenger, den
//! Rechner, an dem er sich anmeldet.
//!
//! **Was die Probe nicht sieht**, und was die Zusage deshalb nicht groesser
//! macht, als sie ist: einen Weg, der den herausgegebenen Text **nach** dem
//! einen Ruf weiterreicht, etwa in eine Meldung; sie zaehlt Nennungen des
//! Namens, Funktionsverweise ohne Klammer eingeschlossen, und liest keinen
//! Datenfluss. Und **das `NSString` wird nicht getilgt**: es gehoert
//! Foundation, und KRK kann seine Bytes nicht ueberschreiben. Getilgt wird der
//! `String` aus `preisgeben` (`Zeroizing`), sobald [`einhaengen`] endet.
//!
//! # Warum `NetFSMountURLSync` und kein eigener Mechanismus
//!
//! NetFS ist der Weg, den der Finder fuer „Mit Server verbinden“ geht: es
//! kennt SMB, AFP, NFS, WebDAV und FTP, waehlt den Einhaengepunkt unter
//! `/Volumes` und nimmt Konto und Passwort als **eigene Argumente**, nicht in
//! der URL (Entscheidung 10). Die synchrone Form laeuft auf dem Faden
//! `krk-verbinden`; die asynchrone mit Abbruch waere die Antwort auf die
//! offene Frage des Plans, ob ein laufendes Einhaengen einen Abbruch braucht.
//! `UIOption = NoUI` laesst NetFS keinen eigenen Dialog zeigen: die Antwort
//! gehoert in KRKs Statuszeile.
//!
//! **Keine Kiste bindet NetFS**: im Register dieses Geraets steht keine
//! `objc2-net-fs`, und NetFS ist eine C-Schnittstelle. Gebunden wird deshalb
//! von Hand, wie FSEvents in [`super::fsevents`], mit `#[link]` am Block, damit
//! das Framework ausdruecklich gebunden wird. Die Parameter sind die Typen von
//! Foundation, deren Gegenstuecke in CoreFoundation gebrueckt sind (`NSURL` fuer
//! `CFURLRef`, `NSString` fuer `CFStringRef`, `NSMutableDictionary` fuer
//! `CFMutableDictionaryRef`); die Liste der Einhaengepunkte kommt als
//! `CFArray` von `CFString` zurueck, wie bei FSEvents. Kein Paket mit einem
//! Namen auf `-sys` kommt dafuer in den Baum.
//!
//! # Diese Huelle traegt keine Probe mit echtem Einhaengen, und das ist Absicht
//!
//! Eine Probe, die einhaengte, braeuchte einen Rechner im Netz mit Freigabe und
//! Zugangsdaten und haengte bei jedem `make check` etwas ein, wie
//! [`super::auswurf`] bei jedem Lauf ein Medium auswuerfe. Geprueft wird
//! stattdessen, was ohne Netz pruefbar ist: der Plan aus
//! `tresorweg::verbindung_vorbereiten`, die Einordnung der Rueckgabewerte
//! ([`fehlerart`]) und der Vergleich der Freigaben ([`gleiche_freigabe`]). Das
//! echte Einhaengen ist Nutzerarbeit am laufenden Buendel.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NetFSMountURLSync` steht laut `NetFS.h` seit macOS 10.8
//! (`__OSX_AVAILABLE_STARTING(__MAC_10_8, __IPHONE_NA)`); die Schluessel
//! `kNAUIOptionKey` und `kNAUIOptionNoUI` sind dort Makros auf
//! `CFSTR("UIOption")` und `CFSTR("NoUI")` und kein Laufzeitsymbol, deshalb
//! stehen sie hier als Zeichenketten. `NSURL`, `NSString`, `NSArray`,
//! `NSMutableDictionary`, `NSFileManager` und `NSWorkspace` stehen seit 10.0,
//! ebenso `openURL:`, `sharedWorkspace`, `defaultManager`,
//! `path`, `scheme`, `host` und `absoluteString`.
//! `mountedVolumeURLsIncludingResourceValuesForKeys:options:` und
//! `resourceValuesForKeys:error:` stehen seit 10.6 (`NSFileManager.h`,
//! `NSURL.h`), `NSURLVolumeURLForRemountingKey` seit 10.7 (`NSURL.h`,
//! `API_AVAILABLE(macos(10.7))`). **`URLWithString:encodingInvalidCharacters:`
//! steht seit macOS 14** (`NSURL.h`, `API_AVAILABLE(macos(14.0))`) — die
//! hoechste Untergrenze dieser Datei, und sie liegt unter dem Zielsystem. Die
//! CoreFoundation-Typen `CFArray` und `CFString` tragen in ihren Koepfen keine
//! Angabe und stehen seit 10.0; `CFRetained` ist der zaehlende Zeiger der Kiste
//! `objc2-core-foundation` und kein Systemtyp.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); nichts davon ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und an einer von Hand geschriebenen
//! Bindung sieht der Uebersetzer ohnehin keine; die Nennung hier ist die
//! Gegenmassnahme.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist ein Rust-Typ der
//! Kiste und hat kein macOS-Alter; das Makro `ns_string!` baut die
//! Zeichenkette beim Uebersetzen; `NSVolumeEnumerationOptions` ist eine
//! Optionsmenge aus `NSFileManager.h` ohne eigene Angabe; `DispatchQueue` aus
//! `dispatch2` ist die Rust-Huelle um `libdispatch`, seit 10.6 im System.

use std::ffi::c_int;
use std::io;
use std::path::PathBuf;
use std::ptr::NonNull;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2_app_kit::NSWorkspace;
use objc2_core_foundation::{CFArray, CFRetained, CFString};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSFileManager, NSMutableDictionary, NSString, NSURL,
    NSURLVolumeURLForRemountingKey, NSVolumeEnumerationOptions, ns_string,
};

use krk_core::fern::rechnerschluessel::{self, Schluesselstand};
use krk_core::fern::{
    Anmeldung, Fernfaden, Fernfehler, Fernziel, Herkunft, Kanalplan, Trennung, Zugangsfehler,
    Zugangsregister, ssh_plan,
};
use krk_core::heimordner::eintraege::geheimnisse::Passwort;
use krk_core::heimordner::tresor::{Oeffnungsmittel, Schluessel};
use krk_core::ort::FernerOrt;

use crate::anmeldehilfe::{self, Anmeldehilfe};
use crate::tresorweg::{self, Verbindungshindernis, Verbindungsplan};

/// `EEXIST` aus `sys/errno.h`: NetFS meldet damit eine schon eingehaengte
/// Freigabe.
const EEXIST: c_int = 17;

/// `EAUTH` aus `sys/errno.h`: die Anmeldung ist abgewiesen.
const EAUTH: c_int = 80;

/// Die Codes, mit denen NetFS eine abgewiesene Anmeldung meldet: `EAUTH`,
/// `EACCES` (13) und aus `NetFS.h` `ENETFSNOAUTHMECHSUPP` (-5997),
/// `ENETFSPWDNEEDSCHANGE` (-5045), `ENETFSPWDPOLICY` (-5046) und
/// `ENETFSACCOUNTRESTRICTED` (-5999).
const ANMELDUNG_ABGEWIESEN: [c_int; 6] = [EAUTH, 13, -5997, -5045, -5046, -5999];

/// Die Codes, mit denen NetFS einen unerreichbaren Rechner meldet, aus
/// `sys/errno.h`: `ENETUNREACH` (51), `ETIMEDOUT` (60), `ECONNREFUSED` (61),
/// `EHOSTDOWN` (64), `EHOSTUNREACH` (65).
const NICHT_ERREICHBAR: [c_int; 5] = [51, 60, 61, 64, 65];

/// Der Ausgang, mit dem `ssh` einen eigenen Fehler meldet (`man ssh`, EXIT
/// STATUS).
const SSH_EIGENER_FEHLER: i32 = 255;

/// Zeilen, mit denen `ssh` eine abgewiesene Anmeldung meldet.
const SSH_ANMELDUNG_ABGEWIESEN: [&str; 1] = ["Permission denied"];

/// Zeilen, mit denen `ssh` einen unerreichbaren Rechner meldet; der Wortlaut
/// ist der von `strerror(3)` und `gai_strerror(3)` hinter dem Rechnernamen.
const SSH_NICHT_ERREICHBAR: [&str; 7] = [
    "Could not resolve hostname",
    "Connection refused",
    "Connection timed out",
    "Operation timed out",
    "No route to host",
    "Network is unreachable",
    "Host is down",
];

#[link(name = "NetFS", kind = "framework")]
unsafe extern "C" {
    /// `NetFS.h`: haengt die Freigabe zu `url` ein und liefert 0, einen
    /// `errno`-Wert oder einen negativen `OSStatus`. Bei 0 traegt
    /// `mountpoints` eine eigene Referenz auf ein `CFArray` von `CFString`,
    /// die der Rufer freigibt.
    fn NetFSMountURLSync(
        url: &NSURL,
        mountpath: Option<&NSURL>,
        user: Option<&NSString>,
        passwd: Option<&NSString>,
        open_options: Option<&NSMutableDictionary<NSString, NSString>>,
        mount_options: Option<&NSMutableDictionary<NSString, NSString>>,
        mountpoints: *mut *const CFArray<CFString>,
    ) -> c_int;
}

/// Was aus einem Klick auf eine Verbindung geworden ist.
#[derive(Debug)]
pub enum Verbindungsausgang {
    /// Die Freigabe ist an diesem Einhaengepunkt eingehaengt.
    Eingehaengt(PathBuf),
    /// NetFS meldet, sie sei schon eingehaengt; der Einhaengepunkt, wenn er
    /// sich unter den eingehaengten Datentraegern findet.
    SchonEingehaengt(Option<PathBuf>),
    /// Die URL geht an das System; der Hauptfaden reicht sie ueber
    /// [`uebergeben`] weiter.
    Uebergeben(String),
    /// Vor NetFS gescheitert: Sperre, Suche oder Ziel.
    Hindernis(Verbindungshindernis),
    /// Aus der URL wird kein `NSURL`.
    UngueltigeUrl,
    /// Die Anmeldung ist abgewiesen.
    AnmeldungAbgewiesen,
    /// Der Rechner ist nicht erreichbar.
    NichtErreichbar,
    /// Jeder andere Code von NetFS.
    Fehlercode(c_int),
    /// Eine sftp-Verbindung steht: der ferne Ort des Startordners, und der
    /// verlangte Startordner, wenn er fehlte und das Anmeldeverzeichnis an
    /// seine Stelle trat.
    Fern {
        /// Der Startordner auf dem Server.
        ort: FernerOrt,
        /// Der verlangte Ordner, wenn er fehlte.
        startordner_fehlte: Option<String>,
        /// Ob ein getrennter Zugang ein neues Gespraech bekommen hat; dann
        /// liest jeder Tab auf diesem Server neu (Entscheidung 3 des Plans).
        neu_verbunden: bool,
    },
    /// `ssh` kennt den Rechnerschluessel nicht; der Name, unter dem
    /// gesucht wurde.
    RechnerschluesselUnbekannt(String),
    /// Eine sftp-Verbindung scheiterte aus einem anderen Grund als Anmeldung
    /// oder Erreichbarkeit; die letzte Zeile von `ssh` oder der Grund des
    /// Gespraechs.
    FernAbgewiesen(String),
}

/// Was der Faden zurueckgibt: den Ausgang und den Schluessel, falls er aus
/// einer eben geprueften PIN stammt und gemerkt werden darf.
#[derive(Debug)]
pub struct Verbindungsergebnis {
    /// Der Schluessel aus der PIN, oder `None`.
    pub zu_merken: Option<Schluessel>,
    /// Was geschehen ist.
    pub ausgang: Verbindungsausgang,
}

/// Stellt die Verbindung `name` aus `secrets.txt` auf dem Faden
/// `krk-verbinden` her und ruft danach `fertig` auf dem Hauptfaden.
///
/// **Kehrt sofort zurueck**; auf dem Hauptfaden steht allein das Starten des
/// Fadens. Lesen, Entschluesseln (samt Ableitung bei einer PIN), Finden und
/// Einhaengen laufen dort, und Klartext, Eintrag und Passwort fallen dort
/// (Entscheidung 11). Laesst sich der Faden nicht starten, kommt der Fehler
/// zurueck und `fertig` laeuft nie.
pub fn herstellen(
    pfad: PathBuf,
    name: String,
    mittel: Oeffnungsmittel,
    register: Zugangsregister,
    fertig: impl FnOnce(MainThreadMarker, Verbindungsergebnis) + Send + 'static,
) -> io::Result<()> {
    let _faden = krk_core::fern::faden("krk-verbinden".to_owned(), move |pfand| {
        let vorbereitung = tresorweg::verbindung_vorbereiten(&pfad, &name, &mittel);
        drop(mittel);
        let ausgang = match vorbereitung.plan {
            Err(hindernis) => Verbindungsausgang::Hindernis(hindernis),
            Ok(Verbindungsplan::Uebergeben { url }) => Verbindungsausgang::Uebergeben(url),
            Ok(Verbindungsplan::Einhaengen {
                url,
                konto,
                passwort,
            }) => einhaengen(&url, konto.as_deref(), passwort),
            Ok(Verbindungsplan::Sftp { ziel, passwort }) => {
                fern_verbinden(pfand, &register, &name, &ziel, passwort)
            }
        };
        let ergebnis = Verbindungsergebnis {
            zu_merken: vorbereitung.zu_merken,
            ausgang,
        };
        DispatchQueue::main().exec_async(move || {
            let Some(mtm) = MainThreadMarker::new() else {
                // Kann nicht eintreten: die Hauptschlange laeuft auf dem
                // Hauptfaden. Dieselbe Erwaegung wie in `auswurf`.
                return;
            };
            fertig(mtm, ergebnis);
        });
    })?;
    Ok(())
}

/// Warum die Vorbereitung eines sftp-Aufbaus abbrach, bevor `ssh` startete.
enum Fernhindernis {
    /// Der Rechnerschluessel ist unbekannt; der Suchname.
    Unbekannt(String),
    /// Eine Frage der Vorpruefung oder der Start der Anmeldehilfe scheiterte.
    Gescheitert(String),
}

/// Stellt eine sftp-Verbindung her; laeuft auf dem Faden `krk-verbinden`
/// (Entscheidungen 3 bis 7 des Plans
/// `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
///
/// Ein lebender, offener Zugang zum selben Server wird genommen, ohne
/// Vorpruefung und ohne dass das Passwort die Datei verlaesst; sonst fragt
/// [`rechnerschluessel::vorpruefen`], ob `ssh` den Rechner kennt, und erst
/// dann entsteht mit Passwort die Anmeldehilfe. Sie lebt bis zum Ende des
/// Aufbaus und faellt dann, gleich wie er ausging.
fn fern_verbinden(
    pfand: &Fernfaden,
    register: &Zugangsregister,
    name: &str,
    ziel: &Fernziel,
    passwort: Passwort,
) -> Verbindungsausgang {
    let vorbereiten = move || -> Result<(Kanalplan, Option<Anmeldehilfe>), Fernhindernis> {
        match rechnerschluessel::vorpruefen(ziel) {
            Ok(Schluesselstand::Bekannt) => {}
            Ok(Schluesselstand::Unbekannt { suchname }) => {
                return Err(Fernhindernis::Unbekannt(suchname));
            }
            Err(grund) => return Err(Fernhindernis::Gescheitert(grund)),
        }
        if passwort.ist_leer() {
            return Ok((ssh_plan(ziel, Anmeldung::OhnePasswort), None));
        }
        let programm = std::env::current_exe()
            .map_err(|fehler| Fernhindernis::Gescheitert(fehler.to_string()))?;
        let hilfe = anmeldehilfe::passwort_reichen(passwort, &anmeldehilfe::hilfeort())
            .map_err(|fehler| Fernhindernis::Gescheitert(fehler.to_string()))?;
        let plan = ssh_plan(
            ziel,
            Anmeldung::Passwort {
                hilfe: hilfe.socket(),
                programm: &programm,
            },
        );
        Ok((plan, Some(hilfe)))
    };
    let (zugang, herkunft) = match register.zugang(pfand, &ziel.kennung(), name, vorbereiten) {
        Ok(gefunden) => gefunden,
        Err(Zugangsfehler::Vorbereitung(Fernhindernis::Unbekannt(suchname))) => {
            return Verbindungsausgang::RechnerschluesselUnbekannt(suchname);
        }
        Err(Zugangsfehler::Vorbereitung(Fernhindernis::Gescheitert(grund))) => {
            return Verbindungsausgang::FernAbgewiesen(grund);
        }
        Err(Zugangsfehler::Fern(fehler)) => return fernfehlerart(&fehler),
    };
    match zugang.startordner(pfand, &ziel.startpfad) {
        Ok(start) => Verbindungsausgang::Fern {
            ort: FernerOrt {
                zugang,
                pfad: start.pfad,
            },
            startordner_fehlte: start.fehlte,
            neu_verbunden: herkunft == Herkunft::NeuVerbunden,
        },
        Err(fehler) => fernfehlerart(&fehler),
    }
}

/// Die Art eines Fehlers beim sftp-Aufbau. Rein und ohne Netz pruefbar.
#[must_use]
fn fernfehlerart(fehler: &Fernfehler) -> Verbindungsausgang {
    match fehler {
        Fernfehler::Getrennt(trennung) => trennungsart(trennung),
        Fernfehler::Status { .. }
        | Fernfehler::Protokoll(_)
        | Fernfehler::NichtAngeboten(_)
        | Fernfehler::Abgebrochen => Verbindungsausgang::FernAbgewiesen(fehler.to_string()),
    }
}

/// Die Art einer Trennung beim Aufbau, nach den Zeilen der Fehlerausgabe von
/// `ssh` (Modulkopf). Ohne passende Zeile, oder wenn nicht `ssh` selbst mit
/// 255 endete, geht die letzte Zeile oder der Grund weiter.
#[must_use]
fn trennungsart(trennung: &Trennung) -> Verbindungsausgang {
    let traegt = |muster: &[&str]| {
        trennung
            .fehlerausgabe
            .iter()
            .any(|zeile| muster.iter().any(|teil| zeile.contains(teil)))
    };
    if trennung.ausgang == Some(SSH_EIGENER_FEHLER) {
        if traegt(&SSH_ANMELDUNG_ABGEWIESEN) {
            return Verbindungsausgang::AnmeldungAbgewiesen;
        }
        if traegt(&SSH_NICHT_ERREICHBAR) {
            return Verbindungsausgang::NichtErreichbar;
        }
    }
    let grund = trennung
        .fehlerausgabe
        .last()
        .cloned()
        .unwrap_or_else(|| trennung.grund.to_string());
    Verbindungsausgang::FernAbgewiesen(grund)
}

/// Haengt den Entwurf auf dem Faden `krk-tresor` an `secrets.txt` an und ruft
/// danach `fertig` auf dem Hauptfaden, mit dem Entwurf und dem Ausgang
/// (Entscheidungen 13 und 20).
///
/// **Kehrt sofort zurueck**; Lesen, Entschluesseln samt Ableitung,
/// Verschliessen und das atomare Schreiben laufen auf dem Faden. Laesst sich
/// der Faden nicht starten, kommt der Fehler zurueck und `fertig` laeuft nie.
pub fn anhaengen(
    pfad: PathBuf,
    entwurf: tresorweg::Verbindungsentwurf,
    mittel: Oeffnungsmittel,
    fertig: impl FnOnce(MainThreadMarker, tresorweg::Verbindungsentwurf, tresorweg::Anhaengausgang)
    + Send
    + 'static,
) -> io::Result<()> {
    let _faden = std::thread::Builder::new()
        .name("krk-tresor".to_owned())
        .spawn(move || {
            let ausgang = tresorweg::anhaengen(&pfad, &entwurf.entwurf(), &mittel);
            drop(mittel);
            DispatchQueue::main().exec_async(move || {
                let Some(mtm) = MainThreadMarker::new() else {
                    // Kann nicht eintreten, wie in `herstellen`.
                    return;
                };
                fertig(mtm, entwurf, ausgang);
            });
        })?;
    Ok(())
}

/// Haengt die Freigabe ueber NetFS ein; laeuft auf dem Faden
/// `krk-verbinden`.
///
/// **Der eine Rufer von `Passwort::preisgeben`** (Modulkopf). Ein leeres
/// Passwort geht als `NULL`, damit NetFS eine Anmeldung ohne Passwort
/// versucht statt eine mit leerem.
fn einhaengen(url: &str, konto: Option<&str>, passwort: Passwort) -> Verbindungsausgang {
    let Some(adresse) = url_aus(url) else {
        return Verbindungsausgang::UngueltigeUrl;
    };
    let konto = konto.map(NSString::from_str);
    let leer = passwort.ist_leer();
    let klartext = passwort.preisgeben();
    let passwd = (!leer).then(|| NSString::from_str(&klartext));
    let optionen =
        NSMutableDictionary::from_slices(&[ns_string!("UIOption")], &[ns_string!("NoUI")]);
    let mut punkte: *const CFArray<CFString> = std::ptr::null();
    // SAFETY: Die Argumente sind lebende Objekte der gebrueckten Typen, die
    // `NetFS.h` verlangt, oder `NULL` an den Stellen, die der Kopf dafuer
    // freigibt (`mountpath`, `user`, `passwd`, `mount_options`). `punkte` ist
    // ein gueltiger Ausgabezeiger; NetFS schreibt dort allein bei Erfolg eine
    // eigene Referenz hinein, die unten genau einmal freigegeben wird.
    let code = unsafe {
        NetFSMountURLSync(
            &adresse,
            None,
            konto.as_deref(),
            passwd.as_deref(),
            Some(&optionen),
            None,
            &mut punkte,
        )
    };
    drop(passwd);
    drop(klartext);
    if code == 0 {
        let Some(punkte) = NonNull::new(punkte.cast_mut()) else {
            return Verbindungsausgang::Fehlercode(code);
        };
        // SAFETY: Bei 0 traegt `punkte` eine Referenz, die der Rufer besitzt
        // (`NetFS.h`, „returned as a CFStringRef in mountpoints“, Create-Regel
        // von CoreFoundation); `from_raw` uebernimmt sie und gibt sie beim
        // Fallen frei.
        let punkte = unsafe { CFRetained::from_raw(punkte) };
        return match punkte.get(0) {
            Some(punkt) => Verbindungsausgang::Eingehaengt(PathBuf::from(punkt.to_string())),
            None => Verbindungsausgang::Fehlercode(code),
        };
    }
    if code == EEXIST {
        return Verbindungsausgang::SchonEingehaengt(eingehaengte_freigabe(url));
    }
    fehlerart(code)
}

/// Die Art eines Fehlercodes von NetFS, der weder 0 noch `EEXIST` ist.
///
/// Rein und ohne System pruefbar; die zwei Listen stehen oben mit ihrer
/// Herkunft.
#[must_use]
fn fehlerart(code: c_int) -> Verbindungsausgang {
    if ANMELDUNG_ABGEWIESEN.contains(&code) {
        Verbindungsausgang::AnmeldungAbgewiesen
    } else if NICHT_ERREICHBAR.contains(&code) {
        Verbindungsausgang::NichtErreichbar
    } else {
        Verbindungsausgang::Fehlercode(code)
    }
}

/// Der Einhaengepunkt der schon eingehaengten Freigabe zu `url`, wenn einer
/// der eingehaengten Datentraeger sie traegt (Entscheidung 10); laeuft auf
/// dem Faden `krk-verbinden`.
///
/// Gefragt wird je Datentraeger `NSURLVolumeURLForRemountingKey`, die URL,
/// mit der das System ihn wieder einhaengen wuerde; verglichen wird mit
/// [`gleiche_freigabe`].
fn eingehaengte_freigabe(url: &str) -> Option<PathBuf> {
    // SAFETY: Ein Fremdsymbol von Foundation, der Schluesselname der URL zum
    // Wiedereinhaengen. Es wird gelesen und nicht geschrieben.
    let schluessel_url = unsafe { NSURLVolumeURLForRemountingKey };
    let schluessel = NSArray::from_slice(&[schluessel_url]);
    let orte = NSFileManager::defaultManager()
        .mountedVolumeURLsIncludingResourceValuesForKeys_options(
            Some(&schluessel),
            NSVolumeEnumerationOptions::SkipHiddenVolumes,
        )?;
    orte.iter().find_map(|ort| {
        let wieder = ort
            .resourceValuesForKeys_error(&schluessel)
            .ok()?
            .objectForKey(schluessel_url)?
            .downcast::<NSURL>()
            .ok()?;
        let wieder = wieder.absoluteString()?.to_string();
        if !gleiche_freigabe(&wieder, url) {
            return None;
        }
        Some(PathBuf::from(ort.path()?.to_string()))
    })
}

/// Ob zwei URLs dieselbe Freigabe nennen: gleiches Schema und gleicher
/// Rechner ohne Ruecksicht auf Gross- und Kleinschreibung, und gleiches erstes
/// Pfadstueck, ebenso; ein Konto vor dem Rechner (`konto@`) zaehlt nicht.
///
/// **Das erste Pfadstueck ist die Freigabe**: `smb://nas/daten/projekte`
/// haengt die Freigabe `daten` ein. Rein und ohne System pruefbar.
///
/// **Verglichen wird dekodiert, auf beiden Seiten.** Die eingehaengte Seite
/// kommt als `absoluteString` des Systems und ist prozentkodiert
/// (`Daten%20Ordner`), die gesuchte ist die URL aus dem Plan, roh bis auf `#`
/// und `?` (`Daten Ordner`, `Daten%231`); Rechner und Freigabe werden deshalb
/// nach dem Zerlegen beide ueber [`prozent_dekodiert`] gelesen
/// (`issues/261016-0614_*_die-verbindungs-url-kuerzt-den-pfad-an-raute-und-fragezeichen-und-der-vergleich-mit-der-eingehaengten-freigabe-liest-roh-gegen-kodiert.md`).
/// **Was das nicht angleicht:** eine Freigabe, deren Umlaut das System in
/// einer anderen Unicode-Normalform meldet als der Nutzer ihn getippt hat;
/// an einer eingehaengten Freigabe nicht nachgesehen.
#[must_use]
fn gleiche_freigabe(eingehaengt: &str, gesucht: &str) -> bool {
    fn teile(url: &str) -> Option<(String, String, String)> {
        let (schema, rest) = url.split_once("://")?;
        let (vorn, pfad) = rest.split_once('/').unwrap_or((rest, ""));
        let rechner = vorn.rsplit_once('@').map_or(vorn, |(_, rechner)| rechner);
        let freigabe = pfad.split('/').next().unwrap_or("");
        Some((
            schema.to_ascii_lowercase(),
            prozent_dekodiert(rechner).to_lowercase(),
            prozent_dekodiert(freigabe).to_lowercase(),
        ))
    }
    match (teile(eingehaengt), teile(gesucht)) {
        (Some(links), Some(rechts)) => links == rechts,
        _ => false,
    }
}

/// Ein URL-Stueck mit jedem `%XX` aus zwei Hexziffern als sein Byte; ein `%`
/// ohne zwei Hexziffern dahinter bleibt stehen, und Bytefolgen, die kein UTF-8
/// ergeben, werden ersetzt statt abgewiesen. Rein und ohne System pruefbar.
#[must_use]
fn prozent_dekodiert(stueck: &str) -> String {
    fn hexwert(byte: u8) -> Option<u8> {
        char::from(byte)
            .to_digit(16)
            .and_then(|wert| u8::try_from(wert).ok())
    }
    let roh = stueck.as_bytes();
    let mut bytes = Vec::with_capacity(roh.len());
    let mut stelle = 0;
    while let Some(&byte) = roh.get(stelle) {
        let kodiert = (byte == b'%')
            .then(|| Some(hexwert(*roh.get(stelle + 1)?)? * 16 + hexwert(*roh.get(stelle + 2)?)?))
            .flatten();
        match kodiert {
            Some(wert) => {
                bytes.push(wert);
                stelle += 3;
            }
            None => {
                bytes.push(byte);
                stelle += 1;
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Gibt eine URL an das System, das das zustaendige Programm oeffnet; auf
/// dem Hauptfaden, ohne Passwort (Entscheidung 10).
///
/// Liefert, ob das System sie angenommen hat; `openURL:` nennt keinen Grund,
/// wie in [`super::standardprogramm`].
#[must_use]
pub fn uebergeben(url: &str) -> bool {
    let Some(adresse) = url_aus(url) else {
        return false;
    };
    NSWorkspace::sharedWorkspace().openURL(&adresse)
}

/// Das `NSURL` zu einer URL aus [`crate::tresorweg::Verbindungsplan`], oder
/// `None`, wenn auch kodiert keine entsteht.
///
/// **Ausdruecklich mit `encodingInvalidCharacters:` und nicht ueber
/// `URLWithString:`**: die Adresse steht so in der URL, wie der Nutzer sie
/// eingetragen hat (Modulkopf von `krk_core::heimordner::verbindung`), und ein
/// Leerzeichen oder ein Zeichen jenseits von ASCII im Pfad einer Freigabe
/// (`nas.local/Daten Ordner`) wird hier prozentkodiert. `URLWithString:`
/// tut das allein in Programmen, die gegen das SDK von macOS 14 oder spaeter
/// gebunden sind, und hinge damit an der Bauumgebung. Was auch kodiert keine
/// URL ergibt, etwa ein Leerzeichen im Rechnernamen oder ein Port aus
/// Buchstaben, weist schon das Blatt „Verbinden“ ab, ueber dieselbe Funktion.
///
/// **`#` und `?` kodiert diese Funktion nicht**, denn sie sind gueltige
/// URL-Zeichen und beenden hier den Pfad. Auf dem Einhaengeweg kommen sie
/// deshalb schon kodiert an (`%23`, `%3F`, Modulkopf von
/// `krk_core::heimordner::verbindung`), und `nas.local/Daten#1` haengt
/// `/Daten#1` ein und nicht `/Daten`; auf dem Uebergabeweg bleiben sie Anfrage
/// und Fragment.
#[must_use]
pub fn url_aus(url: &str) -> Option<Retained<NSURL>> {
    NSURL::URLWithString_encodingInvalidCharacters(&NSString::from_str(url), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein Leerzeichen und ein Umlaut im Pfad werden kodiert, eine schon
    /// kodierte Adresse bleibt, wie sie ist, und ein Leerzeichen im
    /// Rechnernamen oder ein Port aus Buchstaben ergeben keine URL.
    #[test]
    fn die_url_kodiert_was_sich_kodieren_laesst() {
        let gezeigt = |url: &str| {
            url_aus(url)
                .and_then(|adresse| adresse.absoluteString())
                .map(|text| text.to_string())
        };
        assert_eq!(
            gezeigt("smb://nas.local/Daten Ordner").as_deref(),
            Some("smb://nas.local/Daten%20Ordner")
        );
        assert_eq!(
            gezeigt("smb://nas.local/Daten%20Ordner").as_deref(),
            Some("smb://nas.local/Daten%20Ordner")
        );
        assert_eq!(
            gezeigt("smb://nas.local/Bü ro").as_deref(),
            Some("smb://nas.local/B%C3%BC%20ro")
        );
        assert_eq!(gezeigt("smb://nas local/x"), None);
        assert_eq!(gezeigt("smb://nas.local:abc/x"), None);
    }

    /// `Passwort::preisgeben` hat genau zwei Rufer: [`einhaengen`] dieser
    /// Datei, an NetFS (Akzeptanz (6) von Schritt 6, Entscheidung 11), und
    /// `passwort_reichen` in `anmeldehilfe.rs`, an `ssh` (Entscheid
    /// `261016-1947-darf-das-passwort-einer-verbindung-krk-ausser-an-netfs-auch-an-ssh-gehen.md`,
    /// Akzeptanz von Schritt 2 des Plans
    /// `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`), je
    /// einmal.
    ///
    /// **Gezaehlt wird ueber jede `.rs`-Datei unter `crates/`, Proben und
    /// Pruefmodule eingeschlossen**: auch eine Probe, die ein Passwort
    /// preisgaebe, um es zu vergleichen, waere ein zweiter Weg, und der Typ
    /// verspricht genau einen. **Gezaehlt wird jede Nennung des Namens als
    /// ganzes Wort in einer Codezeile**, nicht nur die mit Klammer dahinter:
    /// ein Funktionsverweis wie `.map(Passwort::preisgeben)` ist ein zweiter
    /// Weg heraus, den `quellbaum::aufrufstellen` nicht saehe. Allein die
    /// Erklaerung im Kern zaehlt nicht, erkannt am `fn` unmittelbar davor.
    ///
    /// **Was sie nicht sieht:** wohin der herausgegebene Text danach geht; sie
    /// zaehlt Nennungen und liest keinen Datenfluss (Modulkopf). Und einen
    /// Weg unter einem anderen Namen, etwa ueber `use … as anders;`.
    #[test]
    fn das_passwort_wird_allein_an_netfs_und_ssh_preisgegeben() {
        let name = concat!("preis", "geben");
        let nennungen = |inhalt: &str| -> usize {
            crate::quellbaum::codezeilen(inhalt)
                .map(|zeile| {
                    zeile
                        .match_indices(name)
                        .filter(|(stelle, _)| {
                            let davor = &zeile[..*stelle];
                            let danach = &zeile[stelle + name.len()..];
                            let teil = |zeichen: char| zeichen.is_alphanumeric() || zeichen == '_';
                            !davor.chars().next_back().is_some_and(teil)
                                && !danach.chars().next().is_some_and(teil)
                                && !davor.trim_end().ends_with("fn")
                        })
                        .count()
                })
                .sum()
        };
        let rufer: Vec<(String, usize)> = crate::quellbaum::quelldateien()
            .into_iter()
            .map(|(datei, inhalt)| {
                let zahl = nennungen(&inhalt);
                (datei, zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            rufer,
            [
                ("krk-ui/src/anmeldehilfe.rs".to_owned(), 1),
                ("krk-ui/src/appkit/verbinden.rs".to_owned(), 1),
            ]
        );
        let rumpf = |quelle: &'static str, kopf: &str| {
            quelle
                .split_once(kopf)
                .and_then(|(_, rest)| rest.split_once("\n}\n"))
                .map(|(rumpf, _)| rumpf)
                .unwrap_or_else(|| panic!("{kopf} steht nicht im Code"))
        };
        let ruf = concat!("passwort.preis", "geben()");
        assert!(rumpf(include_str!("verbinden.rs"), "fn einhaengen(").contains(ruf));
        assert!(
            rumpf(
                include_str!("../anmeldehilfe.rs"),
                "pub fn passwort_reichen("
            )
            .contains(ruf)
        );

        // Die Zaehlung selbst: ein Funktionsverweis zaehlt, die Erklaerung
        // und ein laengerer Name nicht.
        let verweis = concat!("let x = p.map(Passwort::preis", "geben);");
        let erklaerung = concat!("    pub fn preis", "geben(mut self) {");
        let laenger = concat!("let y = preis", "geben_alle();");
        assert_eq!(nennungen(verweis), 1);
        assert_eq!(nennungen(erklaerung), 0);
        assert_eq!(nennungen(laenger), 0);
    }

    /// Die Codes der Anmeldung und der Erreichbarkeit fallen in ihre Art,
    /// jeder andere bleibt ein Code.
    #[test]
    fn die_codes_von_netfs_fallen_in_ihre_art() {
        for code in ANMELDUNG_ABGEWIESEN {
            assert!(matches!(
                fehlerart(code),
                Verbindungsausgang::AnmeldungAbgewiesen
            ));
        }
        for code in NICHT_ERREICHBAR {
            assert!(matches!(
                fehlerart(code),
                Verbindungsausgang::NichtErreichbar
            ));
        }
        assert!(matches!(fehlerart(2), Verbindungsausgang::Fehlercode(2)));
        assert!(matches!(
            fehlerart(-128),
            Verbindungsausgang::Fehlercode(-128)
        ));
    }

    /// Eine Trennung beim sftp-Aufbau faellt nach den Zeilen von `ssh` in ihre
    /// Art, aber nur bei Ausgang 255; jede andere traegt die letzte Zeile oder
    /// den Grund weiter.
    #[test]
    fn eine_trennung_von_ssh_faellt_in_ihre_art() {
        use krk_core::fern::Trennungsgrund;
        let trennung = |zeilen: &[&str], ausgang: Option<i32>| Trennung {
            grund: Trennungsgrund::AusgabeEndete,
            fehlerausgabe: zeilen.iter().map(|zeile| (*zeile).to_owned()).collect(),
            ausgang,
        };
        assert!(matches!(
            trennungsart(&trennung(
                &["kai@ssh.example.org: Permission denied (publickey,password)."],
                Some(255)
            )),
            Verbindungsausgang::AnmeldungAbgewiesen
        ));
        for zeile in [
            "ssh: Could not resolve hostname nirgends.test: nodename nor servname provided, or not known",
            "ssh: connect to host 10.0.0.1 port 22: Operation timed out",
            "ssh: connect to host localhost port 2: Connection refused",
        ] {
            assert!(
                matches!(
                    trennungsart(&trennung(&[zeile], Some(255))),
                    Verbindungsausgang::NichtErreichbar
                ),
                "{zeile}"
            );
        }
        match trennungsart(&trennung(&["Host key verification failed."], Some(255))) {
            Verbindungsausgang::FernAbgewiesen(grund) => {
                assert_eq!(grund, "Host key verification failed.");
            }
            anders => panic!("{anders:?}"),
        }
        match trennungsart(&trennung(&["Permission denied"], Some(1))) {
            Verbindungsausgang::FernAbgewiesen(grund) => assert_eq!(grund, "Permission denied"),
            anders => panic!("{anders:?}"),
        }
        match trennungsart(&trennung(&[], None)) {
            Verbindungsausgang::FernAbgewiesen(grund) => {
                assert_eq!(grund, Trennungsgrund::AusgabeEndete.to_string());
            }
            anders => panic!("{anders:?}"),
        }
        assert!(matches!(
            fernfehlerart(&Fernfehler::Protokoll("x".to_owned())),
            Verbindungsausgang::FernAbgewiesen(_)
        ));
    }

    /// Die Freigabe wird an Schema, Rechner und erstem Pfadstueck erkannt,
    /// ein Konto davor zaehlt nicht.
    #[test]
    fn eine_freigabe_wird_an_schema_rechner_und_erstem_pfadstueck_erkannt() {
        assert!(gleiche_freigabe(
            "smb://kai@NAS.local/Daten",
            "smb://nas.local/daten"
        ));
        assert!(gleiche_freigabe(
            "smb://nas.local/daten",
            "smb://nas.local/daten/projekte"
        ));
        assert!(!gleiche_freigabe(
            "smb://nas.local/daten",
            "afp://nas.local/daten"
        ));
        assert!(!gleiche_freigabe(
            "smb://nas.local/daten",
            "smb://nas2.local/daten"
        ));
        assert!(!gleiche_freigabe(
            "smb://nas.local/daten",
            "smb://nas.local/fotos"
        ));
        assert!(!gleiche_freigabe("kein url", "smb://nas.local/daten"));
    }

    /// Die kodierte Form des Systems und die rohe aus dem Plan nennen
    /// dieselbe Freigabe (issues/261016-0614): Leerzeichen, Umlaut und die
    /// auf dem Einhaengeweg kodierte Raute.
    #[test]
    fn eine_freigabe_wird_kodiert_und_roh_als_dieselbe_erkannt() {
        assert!(gleiche_freigabe(
            "smb://nas.local/Daten%20Ordner",
            "smb://nas.local/Daten Ordner"
        ));
        assert!(gleiche_freigabe(
            "smb://kai@nas.local/B%C3%BC%20ro/",
            "smb://nas.local/Bü ro/projekte"
        ));
        assert!(gleiche_freigabe(
            "smb://nas.local/Daten%231",
            "smb://nas.local/Daten%231/x"
        ));
        assert!(!gleiche_freigabe(
            "smb://nas.local/Daten%20Ordner",
            "smb://nas.local/Daten"
        ));
        assert_eq!(prozent_dekodiert("a%20b%C3%BC"), "a bü");
        assert_eq!(prozent_dekodiert("100%"), "100%");
        assert_eq!(prozent_dekodiert("%zz%2"), "%zz%2");
    }

    /// Eine Adresse mit `#` oder `?` haengt den ganzen Pfad ein: aus dem Ziel
    /// des Kerns entsteht eine URL, deren Pfad beide Zeichen traegt, und eine
    /// Webadresse behaelt Anfrage und Fragment (issues/261016-0614).
    #[test]
    fn raute_und_fragezeichen_bleiben_im_pfad_der_freigabe() {
        use krk_core::heimordner::verbindung::{Verbindungsziel, verbindungsziel};
        let url = |protokoll: &str, adresse: &str| match verbindungsziel(protokoll, adresse, "")
            .expect("ein Ziel")
        {
            Verbindungsziel::Einhaengen { url, .. } | Verbindungsziel::Uebergeben { url } => {
                url_aus(&url).expect("eine URL")
            }
            Verbindungsziel::Sftp(ziel) => panic!("keine URL fuer {ziel:?}"),
        };
        let pfad = |adresse: Retained<NSURL>| adresse.path().map(|pfad| pfad.to_string());
        assert_eq!(
            pfad(url("smb", "nas.local/Daten#1/x")).as_deref(),
            Some("/Daten#1/x")
        );
        assert_eq!(pfad(url("smb", "nas.local/a?b")).as_deref(), Some("/a?b"));
        let netz = url("https", "example.org/suche?q=1#oben");
        assert_eq!(
            netz.path().map(|p| p.to_string()).as_deref(),
            Some("/suche")
        );
        assert_eq!(netz.query().map(|q| q.to_string()).as_deref(), Some("q=1"));
    }
}
