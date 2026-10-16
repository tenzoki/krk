//! Wie ein Klick auf eine Verbindung die Verbindung herstellt (261016,
//! Schritt 6 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//!
//! ```text
//!  Hauptfaden                      Faden krk-verbinden
//!  herstellen(pfad, name, mittel) ──> tresorweg::verbindung_vorbereiten
//!                                       │  Einhaengen: NetFSMountURLSync(url, konto, passwort)
//!                                       │              EEXIST: Einhaengepunkt suchen
//!                                       │  Uebergeben: nur die URL
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
//! # Das Passwort verlaesst KRK allein hier, als Argument von NetFS
//!
//! **Der eine Rufer von `Passwort::preisgeben` steht in [`einhaengen`]**, und
//! die Probe `das_passwort_wird_allein_an_netfs_preisgegeben` haelt das am
//! Quelltext der Kisten (Entscheidung 11). Der herausgegebene Text geht
//! unmittelbar in ein `NSString` und von dort als Argument `passwd` an
//! `NetFSMountURLSync`; er steht in keiner Meldung, keiner URL und keinem
//! Protokoll. Die Uebergabe an das System ([`uebergeben`]) bekommt nie ein
//! Passwort, denn [`crate::tresorweg::Verbindungsplan::Uebergeben`] traegt
//! keines.
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

use krk_core::heimordner::eintraege::geheimnisse::Passwort;
use krk_core::heimordner::tresor::{Oeffnungsmittel, Schluessel};

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
    fertig: impl FnOnce(MainThreadMarker, Verbindungsergebnis) + Send + 'static,
) -> io::Result<()> {
    let _faden = std::thread::Builder::new()
        .name("krk-verbinden".to_owned())
        .spawn(move || {
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
#[must_use]
fn gleiche_freigabe(eingehaengt: &str, gesucht: &str) -> bool {
    fn teile(url: &str) -> Option<(String, String, String)> {
        let (schema, rest) = url.split_once("://")?;
        let (vorn, pfad) = rest.split_once('/').unwrap_or((rest, ""));
        let rechner = vorn.rsplit_once('@').map_or(vorn, |(_, rechner)| rechner);
        let freigabe = pfad.split('/').next().unwrap_or("");
        Some((
            schema.to_ascii_lowercase(),
            rechner.to_ascii_lowercase(),
            freigabe.to_lowercase(),
        ))
    }
    match (teile(eingehaengt), teile(gesucht)) {
        (Some(links), Some(rechts)) => links == rechts,
        _ => false,
    }
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

    /// `Passwort::preisgeben` hat genau einen Rufer, in [`einhaengen`] dieser
    /// Datei (Akzeptanz (6) von Schritt 6, Entscheidung 11).
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
    fn das_passwort_wird_allein_an_netfs_preisgegeben() {
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
        assert_eq!(rufer, [("krk-ui/src/appkit/verbinden.rs".to_owned(), 1)]);
        let quelle = include_str!("verbinden.rs");
        let rumpf = quelle
            .split_once("fn einhaengen(")
            .and_then(|(_, rest)| rest.split_once("\n}\n"))
            .map(|(rumpf, _)| rumpf)
            .expect("einhaengen steht im Code");
        assert!(rumpf.contains(concat!("passwort.preis", "geben()")));

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
}
