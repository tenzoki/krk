//! Was AppKit ueber einen Ziehvorgang und ueber einen Ordner sagt (C5, C6 und
//! C7 der Runde 13).
//!
//! Die Gegenseite von [`crate::kommandos::abwurfregel`]. Dort steht die Regel
//! und keine Zeile AppKit; hier stehen die vier Beruehrungen, die ihr die
//! Tatsachen beschaffen, und keine Zeile Regel. Beides zusammen ist ein
//! Abwurf, und die Naht dazwischen laeuft genau hier entlang:
//!
//! ```text
//!  NSView.registerForDraggedTypes: <── sorten()          welche Sorten KRK ueberhaupt sieht
//!
//!  Zielordner  ──> NSURLIsWritableKey ──> beschreibbarkeit() ──> Schreibrecht
//!  NSDraggingInfo ──> draggingSourceOperationMask ──> angebot() ──> (kopieren, verschieben)
//!
//!                     Abwurfurteil ──> zeiger() ──> NSDragOperation
//! ```
//!
//! Die drei oberen Pfeile laufen von AppKit in die Regel hinein, der untere
//! aus ihr heraus. Was die Regel daraus macht, entscheidet sie allein; was
//! AppKit gesagt hat, sagt allein diese Datei.
//!
//! # Die eine Umsetzung von `NSDragOperation`
//!
//! [`angebot`] und [`zeiger`] sind die **einzigen** zwei Stellen im Baum, an
//! denen ein `NSDragOperation` in die Sprache der Regel und wieder zurueck
//! uebersetzt wird. Der Modulkopf von [`crate::kommandos::abwurfregel`] sagt
//! unter „Woran die Regel nicht haengt", dass sie diesen Typ nicht kennt, und
//! diese beiden Funktionen sind der Grund, aus dem sie ihn nicht zu kennen
//! braucht. Eine dritte Stelle, die eine Maske selbst ausliest oder selbst
//! zusammensetzt, waere eine zweite Antwort auf die Frage, was der Zeiger
//! zeigt — und C5 sagt zu, dass der Zeiger und die Wirkung nach dem Loslassen
//! uebereinstimmen. Zwei Antworten koennten auseinanderlaufen.
//!
//! **Die Richtungen sind getrennt und nicht eine Funktion mit zwei Ausgaengen.**
//! Sie werden zu verschiedenen Zeitpunkten gebraucht: [`angebot`] am Anfang von
//! `validateDrop:`, [`zeiger`] an dessen Ende, und dazwischen liegt die ganze
//! Regel.
//!
//! # Warum KRK Sorten anmeldet, die es abweist
//!
//! [`sorten`] meldet neben dem Dateiverweis auch die Zusagesorten aus
//! `NSFilePromiseReceiver` an, obwohl KRK jede Zusagedatei abweist. Das ist
//! kein Widerspruch, sondern der ganze Mechanismus von C7: **eine Sorte, fuer
//! die sich keine Ansicht angemeldet hat, erreicht die Ansicht nie.** Der
//! Zeiger traegt dann das Verbotszeichen des Systems, `validateDrop:` wird gar
//! nicht erst gerufen, und KRK bekommt keine Gelegenheit, irgendetwas zu sagen.
//! Wer allein `NSPasteboardTypeFileURL` anmeldet, bekommt fuer einen
//! Mail-Anhang also nichts, und der Nutzer steht vor einer Anwendung, die
//! schweigt.
//!
//! Der Nutzer hat die andere Form verlangt: KRK sieht die Zusagedatei, misst
//! ueber [`super::zwischenablage::dateiverweise`], dass sie keine Datei auf dem
//! Datentraeger liefert, weist ab und **sagt es in der Statuszeile**. Der
//! Unterschied ist der zwischen „KRK kann das nicht" und „KRK sagt, dass es das
//! nicht kann".
//!
//! **KRK fordert deshalb trotzdem keine Zusagedatei an und schreibt keine.**
//! `receivePromisedFilesAtDestination:…` wird in diesem Baum nirgends gerufen;
//! `NSFilePromiseReceiver` steht hier allein als Auskunftsstelle darueber,
//! welche Sortennamen anzumelden sind. Der Spec haelt das unter C7 als
//! Festlegung fest.
//!
//! # Warum das Schreibrecht hier gemessen und nicht hier beurteilt wird
//!
//! [`beschreibbarkeit`] liefert einen [`Schreibrecht`] und kein „ja, wirf ab".
//! Ob ein `Unbekannt` durchlaesst oder abweist, entscheidet
//! [`crate::kommandos::abwurfregel::urteil`] an einer Stelle fuer alle
//! Ausloeser; der zugrunde liegende Datensatz und die Begruendung stehen in
//! dessen Modulkopf. Dieselbe Arbeitsteilung wie bei
//! [`super::volumes::liegt_auf_netzlaufwerk`], das den Ausloeser beantwortet
//! und die Rangfolge dem Modul ueberlaesst, das sie fuehrt.
//!
//! **Gemessen ist dabei nicht zugesagt.** Der Ressourcenwert antwortet nach dem
//! EUID, also nach der Kennung, die gleich schreiben wird, und er antwortet
//! fuer den Augenblick der Frage. Zwischen der Messung waehrend des Ziehens und
//! dem ersten Systemaufruf danach kann sich das Recht aendern. Was dann
//! scheitert, erscheint mit seinem Grund in der Abschlussliste des Vorgangs, auf
//! demselben Weg, den F5 und F6 heute gehen; ein zweiter Weg dafuer entsteht
//! nicht.
//!
//! # Die Aufrufer entstehen mit Schritt 10
//!
//! Alle vier Funktionen tragen bis dahin einen `expect(dead_code)`-Vermerk nach
//! dem Vorbild aus [`super::zwischenablage::dateiverweise`] und
//! [`crate::kommandos::abwurfregel`]. `expect` und nicht `allow`, damit die
//! Ausnahme ihr Ablaufdatum selbst durchsetzt: mit dem Aufrufer wird die
//! Erwartung unerfuellt, und der Bau haelt unter `-D warnings` an, bis die Zeile
//! weg ist.
//!
//! **Drei der Vermerke stehen unter `cfg_attr(not(test), …)`, der vierte nicht**,
//! und der Unterschied ist keine Nachlaessigkeit: die Proben unten rufen
//! [`sorten`], [`beschreibbarkeit`] und [`zeiger`], also sind diese drei im
//! Probenbau lebendig und der Vermerk waere dort unerfuellt. [`angebot`] traegt
//! keine Probe, weil sich ein `NSDraggingInfo` ohne Ziehsitzung nicht bauen
//! laesst; es ist in beiden Bauarten tot und traegt seinen Vermerk deshalb
//! unbedingt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSArray`, `NSDictionary`, `NSString`, `NSNumber` und `NSURL` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso `fileURLWithPath:` (`NSURL.h:52`, die Form
//! ohne weitere Argumente), `objectForKey:` (`NSDictionary.h:17`) und
//! `boolValue` (`NSValue.h:73`). Ohne Verfuegbarkeitsangabe im Kopf und damit
//! ebenfalls seit 10.0 stehen die vier Beruehrungen des Ziehvorgangs: die
//! Aufzaehlung `NSDragOperation` (`NSDragging.h:25`) mit ihren Werten
//! `NSDragOperationNone` (`:26`), `NSDragOperationCopy` (`:27`) und
//! `NSDragOperationMove` (`:31`), das Protokoll `NSDraggingInfo`
//! (`NSDragging.h:69`) und seine Eigenschaft `draggingSourceOperationMask`
//! (`NSDragging.h:72`). Ebenfalls ohne Angabe und damit seit 10.0 steht der
//! Sortentyp `NSPasteboardType` (`NSPasteboard.h:23`), ein `typedef` auf
//! `NSString` und keine eigene Klasse, sowie der Schluesseltyp
//! `NSURLResourceKey` (`NSURL.h:17`) aus demselben Grund. Vier Beruehrungen
//! sind juenger: `resourceValuesForKeys:error:` (`NSURL.h:183`,
//! `API_AVAILABLE(macos(10.6), …)`) steht seit 10.6, `NSURLIsWritableKey`
//! (`NSURL.h:247`, `API_AVAILABLE(macos(10.7), …)`) seit 10.7, die Klasse
//! `NSFilePromiseReceiver` (`NSFilePromiseReceiver.h:19`,
//! `API_AVAILABLE(macos(10.12))`) und ihre Klasseneigenschaft
//! `readableDraggedTypes` (`NSFilePromiseReceiver.h:23`, ohne eigene Angabe und
//! damit auf der Angabe ihrer Klasse) seit 10.12, und `NSPasteboardTypeFileURL`
//! (`NSPasteboard.h:39`, `API_AVAILABLE(macos(10.13))`) seit 10.13. Jeder Name
//! traegt seine eigene Zeilenangabe: eine Angabe, die fuer ein Paar gilt, stimmt
//! beim Nachlesen fuer hoechstens einen der beiden. Alle Zeilenangaben sind am
//! 260818 in `$(xcrun --show-sdk-path)/System/Library/Frameworks/` nachgelesen,
//! unter `AppKit.framework/Headers/` und `Foundation.framework/Headers/`, und
//! nicht uebernommen. Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine
//! von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in dieser
//! Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit. `objc2`
//! fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::path::Path;

use objc2::Message;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{
    NSDragOperation, NSDraggingInfo, NSFilePromiseReceiver, NSPasteboardType,
    NSPasteboardTypeFileURL,
};
use objc2_foundation::{NSArray, NSNumber, NSString, NSURL, NSURLIsWritableKey};

use crate::kommandos::abwurfregel::{Abwurfurteil, Abwurfvorgang, Schreibrecht};

/// Die Sorten, fuer die sich eine Dateiliste anmeldet (C7).
///
/// Zwei Gruppen in einer Liste, und die zweite ist die ueberraschende:
///
/// 1. `NSPasteboardTypeFileURL` — der Dateiverweis. Das ist die Sorte, die KRK
///    tatsaechlich annehmen kann.
/// 2. `NSFilePromiseReceiver::readableDraggedTypes()` — die Zusagesorten. Sie
///    werden angemeldet, damit KRK sie **abweisen und melden** kann.
///
/// **Warum eine abgewiesene Sorte trotzdem angemeldet wird**, steht im
/// Modulkopf und ist der ganze Mechanismus von C7: eine nicht angemeldete Sorte
/// erreicht die Ansicht nie, `validateDrop:` wird nicht gerufen, und KRK
/// bekaeme keine Gelegenheit, etwas zu sagen. Der Nutzer saehe allein das
/// Verbotszeichen des Systems und keinen Satz dazu, warum.
///
/// **Ohne `MainThreadMarker`**, obwohl die Funktion beim Aufbau der Oberflaeche
/// gerufen wird: keine ihrer beiden Zeilen verlangt den Hauptfaden.
/// `NSPasteboardTypeFileURL` ist eine Konstante, und `NSFilePromiseReceiver`
/// ist in `objc2-app-kit 0.3.2` keine `MainThreadOnly`-Klasse, weshalb
/// `readableDraggedTypes` ohne Marke gebunden ist. Ein Parameter, den keine
/// Zeile verbraucht, muesste `_mtm` heissen; diese Kiste fuehrt heute keine
/// einzige solche Stelle, und eine erste zu eroeffnen behauptete eine
/// Bedingung, die die Bindung nicht kennt.
///
/// **Die Liste kann kuerzer ausfallen, als sie aussieht.** Liefert
/// `readableDraggedTypes` auf einem System eine leere Liste, bleibt allein der
/// Dateiverweis uebrig, und eine Zusagedatei erreicht KRK wieder nicht. Das ist
/// am gebauten Buendel zu messen und steht als erstes Kriterium der C7-Zeile in
/// der Nutzerarbeit des Plans.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der Aufrufer entsteht in Schritt 10 der Runde 13, in \
                  Dateifenster::bauen; siehe Modulkopf"
    )
)]
#[must_use = "die Liste ist die Anmeldung selbst; fallengelassen nimmt die Dateiliste keinen Abwurf an"]
pub fn sorten() -> Retained<NSArray<NSPasteboardType>> {
    // SAFETY: Ein Fremdsymbol von AppKit, der Name der Dateiverweis-Sorte. Es
    // wird gelesen und nicht geschrieben, wie die Sortenkonstanten in
    // `super::zwischenablage`.
    let dateiverweis = unsafe { NSPasteboardTypeFileURL };

    let mut alle: Vec<Retained<NSPasteboardType>> = vec![dateiverweis.retain()];
    alle.extend(NSFilePromiseReceiver::readableDraggedTypes().to_vec());
    NSArray::from_retained_slice(&alle)
}

/// Was das System ueber das Schreibrecht des Zielordners sagt (C6, Lage 2).
///
/// Gefragt wird der Ressourcenwert `NSURLIsWritableKey` am `NSURL` des Ordners.
/// Der Kommentar der Kopfdatei nennt ihn „true if this process (as determined
/// by EUID) can write to the resource" — also die Kennung, die gleich schreiben
/// wird, und nicht die des angemeldeten Nutzers.
///
/// **Drei Ausgaenge, und nur einer weist ab:**
///
/// - [`Schreibrecht::Ja`] — der Wert kam als `true` zurueck.
/// - [`Schreibrecht::Nein`] — der Wert kam als `false` zurueck. **Der einzige
///   Ausgang, der einen Abwurf abweist.**
/// - [`Schreibrecht::Unbekannt`] — die Frage blieb ohne Antwort: der Pfad ist
///   kein gueltiges UTF-8, das System nennt einen Fehler, oder es liefert den
///   Schluessel ohne Wert beziehungsweise mit einem Wert, der kein `NSNumber`
///   ist.
///
/// **Keiner der drei unentscheidbaren Faelle wird als `Nein` gelesen**, und das
/// ist keine Bequemlichkeit, sondern eine Nutzerentscheidung: ein
/// unentscheidbares Schreibrecht laesst den Abwurf durch. Der Datensatz ist
/// `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/decisions/260818-1633_*_gilt-ein-unentscheidbares-schreibrecht-beim-abwurf-als-erlaubnis-oder-als-abweisung.md`,
/// die Begruendung steht im Modulkopf von
/// [`crate::kommandos::abwurfregel`], und die Umsetzung steht dort in der Tafel
/// von [`crate::kommandos::abwurfregel::urteil`]. Wer hier ein `Nein`
/// zurueckgaebe, drehte jene Entscheidung still um, ohne die Tafel anzufassen.
///
/// **Der `NSURL` entsteht bei jedem Aufruf frisch**, wie in
/// [`super::volumes::liegt_auf_netzlaufwerk`] und aus demselben Grund: die
/// Kopfdatei sagt zu `resourceValuesForKeys:error:` ausdruecklich, dass die
/// Antwort im `NSURL`-Objekt **zwischengespeichert** wird und ein spaeterer
/// Aufruf den gemerkten Wert zurueckgibt, statt das Dateisystem noch einmal zu
/// fragen (`NSURL.h:181`). Ein gehaltener `NSURL` lieferte damit waehrend eines
/// ganzen Ziehvorgangs das Recht von vorhin. Ein neuer je Aufruf hat keinen
/// Zwischenspeicher, den er zeigen koennte.
///
/// Diese Funktion ruft weder `canonicalize` noch sonst etwas am Dateisystem;
/// eine Verknuepfung wird nicht verfolgt, und ein Pfad, den es nicht gibt,
/// zaehlt als [`Schreibrecht::Unbekannt`].
///
/// **Mit dieser Funktion ist die `expect(dead_code)`-Ausnahme an
/// [`Schreibrecht`] gefallen**, denn hier entstehen die drei Werte. Der
/// Uebersetzer hat das nicht eingefordert; warum nicht, steht am Doc-Kommentar
/// jener Aufzaehlung.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der Aufrufer entsteht in Schritt 10 der Runde 13, in \
                  DateifensterQuelle::abwurf_pruefen; siehe Modulkopf"
    )
)]
#[must_use = "wer das gemessene Schreibrecht fallen laesst, schreibt in einen Ordner, der es verweigert"]
pub fn beschreibbarkeit(ordner: &Path) -> Schreibrecht {
    let Some(text) = ordner.to_str() else {
        return Schreibrecht::Unbekannt;
    };
    let url = NSURL::fileURLWithPath(&NSString::from_str(text));
    // SAFETY: Ein Fremdsymbol von Foundation, der Schluesselname des
    // Schreibrechts. Es wird gelesen und nicht geschrieben, ebenso wie der
    // Schluessel der Ortsangabe in `super::volumes`.
    let schluessel_schreibbar = unsafe { NSURLIsWritableKey };
    let schluessel = NSArray::from_slice(&[schluessel_schreibbar]);

    let Some(wert) = url
        .resourceValuesForKeys_error(&schluessel)
        .ok()
        .and_then(|werte| werte.objectForKey(schluessel_schreibbar))
        .and_then(|wert| wert.downcast::<NSNumber>().ok())
    else {
        return Schreibrecht::Unbekannt;
    };

    if wert.boolValue() {
        Schreibrecht::Ja
    } else {
        Schreibrecht::Nein
    }
}

/// Welche Vorgaenge die Quelle des Ziehvorgangs anbietet (C5).
///
/// Zurueck kommt `(bietet_kopieren, bietet_verschieben)`, also genau die zwei
/// Felder, die [`crate::kommandos::abwurfregel::Abwurflage`] dafuer fuehrt.
///
/// **Das ist die eine Umsetzung von `NSDragOperation` in die Sprache der
/// Regel.** Der Modulkopf von [`crate::kommandos::abwurfregel`] sagt unter
/// „Woran die Regel nicht haengt", dass sie diesen Typ nicht kennt; hier steht
/// der Grund, aus dem sie ihn nicht zu kennen braucht. Eine zweite Stelle, die
/// eine Maske selbst ausliest, waere eine zweite Antwort auf die Frage, was
/// angeboten ist.
///
/// **Gelesen wird die Menge und keine Zusatztaste.** Das System hat sie aus den
/// gehaltenen Tasten bereits verengt, bevor KRK sie zu sehen bekommt: wer aus
/// dem Finder mit gehaltenem `cmd` zieht, liefert eine Menge ohne das Kopieren.
/// Ein KRK, das stattdessen selbst nach `shift` oder `cmd` fragte, koennte
/// einen Vorgang waehlen, der gar nicht angeboten ist — der Zeiger zeigte das
/// eine und KRK taete das andere. Der Spec schreibt es unter C5 aus.
///
/// **Zweimal `contains` und keine Gleichheit**: `NSDragOperation` ist eine
/// Menge von Bits, und eine Quelle bietet regelmaessig mehr an als diese zwei
/// Werte (`Link`, `Generic`, `Private`, `Delete` stehen daneben). Ein Vergleich
/// auf Gleichheit verfehlte jede Quelle, die einen dritten Vorgang mit
/// anbietet.
#[expect(
    dead_code,
    reason = "der Aufrufer entsteht in Schritt 10 der Runde 13, in \
              DateifensterQuelle::abwurf_pruefen. Der Vermerk steht unbedingt \
              und nicht unter cfg_attr(not(test), ...), weil ein \
              NSDraggingInfo sich ohne Ziehsitzung nicht bauen laesst und \
              diese Funktion deshalb auch im Probenbau keinen Aufrufer hat"
)]
#[must_use = "das Angebot ist eine der sechs Tatsachen der Abwurflage; fallengelassen urteilt die Regel ueber eine Luecke"]
pub fn angebot(zug: &ProtocolObject<dyn NSDraggingInfo>) -> (bool, bool) {
    let angebotene = zug.draggingSourceOperationMask();
    (
        angebotene.contains(NSDragOperation::Copy),
        angebotene.contains(NSDragOperation::Move),
    )
}

/// Was der Zeiger waehrend des Ziehens zeigen soll (C5, C6).
///
/// Die Rueckrichtung zu [`angebot`], und sie steht aus demselben Grund hier:
/// damit die Umsetzung zwischen `NSDragOperation` und der Sprache der Regel an
/// **einer** Stelle je Richtung dasteht. Der Rueckgabewert ist zugleich der
/// Rueckgabewert von `tableView:validateDrop:…`, und AppKit ruft
/// `tableView:acceptDrop:…` nur, wenn hier etwas anderes als
/// `NSDragOperation::None` herauskam. Damit haelt diese eine Zeile die Zusage
/// aus C5, dass der Zeiger und die Wirkung nach dem Loslassen uebereinstimmen.
///
/// Jede Abweisung wird zu `NSDragOperation::None`, gleich aus welchem der fuenf
/// Gruende: der Zeiger kennt nur „nimmt an" und „nimmt nicht an". **Der Grund
/// geht dabei nicht verloren** — er bleibt beim Aufrufer, der die Meldung aus
/// C7 daraus schreibt und sie ueber den gemerkten letzten Grund entdoppelt.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der Aufrufer entsteht in Schritt 10 der Runde 13, in \
                  DateifensterQuelle::abwurf_pruefen; siehe Modulkopf"
    )
)]
#[must_use = "der Wert ist die Antwort an AppKit; fallengelassen zeigt der Zeiger, was das System raet"]
pub fn zeiger(gefaellt: Abwurfurteil) -> NSDragOperation {
    match gefaellt {
        Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren) => NSDragOperation::Copy,
        Abwurfurteil::Ausfuehren(Abwurfvorgang::Verschieben) => NSDragOperation::Move,
        Abwurfurteil::Abweisen(_) => NSDragOperation::None,
    }
}

#[cfg(test)]
mod proben {
    use super::*;
    use crate::kommandos::abwurfregel::Abwurfgrund;
    use crate::pruefordner::Pruefordner;

    /// Die anzumeldende Liste traegt den Dateiverweis, und die Zusagesorten
    /// stehen daneben (C7).
    ///
    /// **Die Zahl der Zusagesorten wird nicht behauptet.** Sie stammt vom
    /// System und darf sich zwischen zwei macOS-Fassungen aendern; die Probe
    /// prueft deshalb, dass jede von ihnen in der Liste steht, und nicht, wie
    /// viele es sind. Was sie damit festhaelt, ist die Aussage des Modulkopfs:
    /// die Liste ist nicht nur der Dateiverweis.
    ///
    /// **Am 260818 auf Darwin 24.6 gemessen:** `readableDraggedTypes` liefert
    /// drei Sorten (`com.apple.NSFilePromiseItemMetaData`,
    /// `com.apple.pasteboard.promised-file-content-type` und eine dynamische),
    /// die Liste traegt damit vier. Die Zahl steht hier als Aufzeichnung und
    /// nicht als Behauptung: der Plan der Runde 13 fuehrt eine leere Liste als
    /// Risiko, und auf diesem Geraet ist sie es nicht.
    #[test]
    fn die_liste_traegt_den_dateiverweis_und_die_zusagesorten() {
        let liste = sorten();
        let namen: Vec<String> = liste
            .to_vec()
            .iter()
            .map(|sorte| sorte.to_string())
            .collect();

        // SAFETY: Ein Fremdsymbol von AppKit, gelesen und nicht geschrieben,
        // wie im Rumpf von `sorten`.
        let dateiverweis = unsafe { NSPasteboardTypeFileURL }.to_string();
        assert!(
            namen.contains(&dateiverweis),
            "C7: ohne den Dateiverweis naehme die Liste gar keinen Abwurf an; sie traegt {namen:?}"
        );

        for zusage in NSFilePromiseReceiver::readableDraggedTypes().to_vec() {
            let name = zusage.to_string();
            assert!(
                namen.contains(&name),
                "C7: die Zusagesorte {name} ist nicht angemeldet; \
                 ohne sie erreicht eine Zusagedatei die Dateiliste nie, \
                 und KRK kann sie weder abweisen noch melden"
            );
        }
    }

    /// Ein frisch angelegter Ordner ist beschreibbar (C6, Lage 2).
    #[test]
    fn ein_frischer_ordner_ist_beschreibbar() {
        let ordner = Pruefordner::neu("beschreibbar");

        assert_eq!(
            beschreibbarkeit(ordner.pfad()),
            Schreibrecht::Ja,
            "C6: ein eben angelegter Ordner meldet kein Schreibrecht"
        );
    }

    /// Ein Ordner ohne Schreibrecht meldet ein gemessenes `Nein` (C6, Lage 2).
    ///
    /// **Die Rechte werden vor dem Ende der Probe wiederhergestellt**, sonst
    /// koennte `Pruefordner::drop` den Ordner nicht mehr abraeumen. Das
    /// geschieht ausdruecklich im eigenen Ablauf und nicht in einem
    /// Aufraeumzweig: schlaegt die Behauptung darueber fehl, bricht die Probe
    /// ab, und ein Ordner mit `0o500` bliebe stehen. Die Behauptung steht
    /// deshalb **nach** der Wiederherstellung, auf einem vorher gemerkten Wert.
    #[test]
    fn ein_ordner_ohne_schreibrecht_meldet_nein() {
        use std::os::unix::fs::PermissionsExt;

        let ordner = Pruefordner::neu("nicht-beschreibbar");
        let vorher = std::fs::metadata(ordner.pfad())
            .expect("der Pruefordner steht")
            .permissions();

        std::fs::set_permissions(ordner.pfad(), std::fs::Permissions::from_mode(0o500))
            .expect("die Rechte des Pruefordners lassen sich setzen");
        let gemessen = beschreibbarkeit(ordner.pfad());
        std::fs::set_permissions(ordner.pfad(), vorher)
            .expect("die Rechte des Pruefordners lassen sich zuruecksetzen");

        assert_eq!(
            gemessen,
            Schreibrecht::Nein,
            "C6: ein Ordner mit 0o500 meldet ein Schreibrecht"
        );
    }

    /// Einen Pfad, den es nicht gibt, kann KRK nicht einordnen (C6, Lage 2).
    ///
    /// **`Unbekannt` und nicht `Nein`**, und das ist die Nutzerentscheidung aus
    /// dem Datensatz zum unentscheidbaren Schreibrecht: die Regel laesst diesen
    /// Wert durch. Wer hier `Nein` zurueckgaebe, drehte sie still um.
    #[test]
    fn ein_fehlender_ordner_bleibt_unbekannt() {
        let ordner = Pruefordner::nur_name("fehlend");

        assert_eq!(
            beschreibbarkeit(ordner.pfad()),
            Schreibrecht::Unbekannt,
            "C6: ein Pfad ohne Ordner dahinter wird eingeordnet, statt unentschieden zu bleiben"
        );
    }

    /// Ein Pfad ohne gueltiges UTF-8 bleibt unbekannt (C6, Lage 2).
    ///
    /// Der Fall ist am Dateisystem nicht zu erzeugen, ohne eine solche Datei
    /// anzulegen; gebraucht wird er auch nicht, denn die Ruecknahme steht in der
    /// ersten Zeile von [`beschreibbarkeit`] und haengt an nichts als dem Pfad.
    #[test]
    fn ein_pfad_ohne_gueltiges_utf8_bleibt_unbekannt() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let kaputt = Path::new(OsStr::from_bytes(b"/tmp/krk-\xff\xfe-kein-utf8"));

        assert_eq!(
            beschreibbarkeit(kaputt),
            Schreibrecht::Unbekannt,
            "C6: ein Pfad ohne gueltiges UTF-8 wird eingeordnet, statt unentschieden zu bleiben"
        );
    }

    /// Die Rueckrichtung, vollstaendig ausgeschrieben (C5, C6).
    ///
    /// Die Erwartungen stehen als Werte da und werden nicht gerechnet, aus
    /// demselben Grund wie die Tafeln in [`crate::kommandos::abwurfregel`]: eine
    /// gerechnete Erwartung waere die Umsetzung ein zweites Mal.
    #[test]
    fn jedes_urteil_hat_seinen_zeiger() {
        assert_eq!(
            zeiger(Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren)),
            NSDragOperation::Copy,
            "C5: das Kopieren zeigt nicht das Pluszeichen des Systems"
        );
        assert_eq!(
            zeiger(Abwurfurteil::Ausfuehren(Abwurfvorgang::Verschieben)),
            NSDragOperation::Move,
            "C5: das Verschieben zeigt den Zeiger des Kopierens"
        );

        for grund in [
            Abwurfgrund::KeineDatei,
            Abwurfgrund::VorgangLaeuft,
            Abwurfgrund::NichtBeschreibbar,
            Abwurfgrund::SelberOrdner,
            Abwurfgrund::KeinAngebot,
        ] {
            assert_eq!(
                zeiger(Abwurfurteil::Abweisen(grund)),
                NSDragOperation::None,
                "C6: die Abweisung {grund:?} laesst den Zeiger annehmen"
            );
        }
    }
}
