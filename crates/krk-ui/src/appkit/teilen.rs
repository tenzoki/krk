//! Die eine Beruehrung mit den Freigabediensten des Systems, die C1 der
//! Runde 6 braucht.
//!
//! ```text
//!  Fokus ──> worauf ──> Quelle (drei Werte, ohne Auffangzweig)
//!                          │
//!  Pfade ──> NSURL::fileURLWithPath: ──> NSSharingServicePicker
//!                          │                    │
//!                          │                    ├─> anbieten ──> showRelativeToRect:
//!                          │                    └─> eintrag_anfuegen ──> standardShareMenuItem
//!                          │                                                    │
//!                          └──────────────────────────────────> Freigabedienste des Systems
//! ```
//!
//! Die eine Frage dieses Moduls: **wie kommt ein Eintrag an die
//! Freigabedienste des Systems.** Ein eigenes Modul und kein Zusatz zu
//! [`super::zwischenablage`], und der Grund ist nicht die Ordnung, sondern die
//! Sache: **das Teilen legt nichts in die Zwischenablage.** Jener Kopf
//! beantwortet, was in der Zwischenablage steht und wohin KRK damit geht; ein
//! Freigabedialog, der `NSPasteboard` nicht anfasst, gaebe ihm eine zweite
//! Frage. Es gibt weiterhin genau **eine** Huelle um `NSPasteboard`, und eine
//! zweite daneben waere der Fehler, den jene Datei ausdruecklich vermeidet.
//! Der Zuschnitt ist damit derselbe wie bei [`super::standardprogramm`], das
//! aus demselben Grund neben der Zwischenablage steht: ein Modul je Frage,
//! eine sichere Huelle je Aufruf, und was die Huelle verlaesst, ist ein
//! gewoehnlicher Rust-Wert. Ein `NSSharingServicePicker` kommt aus dieser
//! Datei nicht heraus.
//!
//! # Ein Menue, ein Bauer, drei Flaechen
//!
//! [`eintrag_anfuegen`] ist der **eine** Menuebauer. Die Dateiliste, der
//! Editor und die Vorschau beantworten allein, welche Eintraege betroffen
//! sind; sie bauen kein Menue. Drei Menuebauer nebeneinander waeren die
//! Wiederholung, die dieses Projekt an `appkit/nummernspalte.rs` und
//! `appkit/tableiste.rs` bereits zweimal vermieden hat (C1, siebtes
//! Kriterium). **Diese Datei kennt keine der drei Flaechen**; sie stellt den
//! Bauer bereit, und angehaengt haben sich die Flaechen selbst.
//!
//! **Zwei Anschlussarten, ein Bauer**, und der Unterschied ist nicht
//! Geschmack, sondern die Bauart der Flaeche:
//!
//! ```text
//!   baut ihr Kontextmenue selbst und bietet einen Delegiertenhaken
//!     Textflaeche des Editors  ─┐
//!     Textanzeige der Vorschau ─┴─> textView:menu:forEvent:atIndex: ─┐
//!                                                                    ├─> eintrag_anfuegen
//!   baut keines und nimmt das Menue der Ansicht                      │
//!     Dateiliste               ─┐                                    │
//!     Bildansicht              ─┼─> setMenu: + menuNeedsUpdate: ─────┘
//!     Inhaltsflaeche           ─┘
//! ```
//!
//! Der Haken der `NSTextView` bekommt das Menue, das AppKit gebaut hat, und
//! gibt es **ergaenzt** zurueck; damit tritt KRKs Eintrag neben das, was
//! AppKit von sich aus gibt, statt es zu ersetzen. Die andere Art leert das
//! Menue und baut es bei **jedem** Rechtsklick neu, weil die betroffenen
//! Eintraege sich zwischen zwei Klicks aendern.
//!
//! **Dass es bei einem Bauer bleibt, misst diese Datei selbst**: die beiden
//! Zaehlproben unter `mod tests` lesen den Quellbaum von `krk-ui` und halten
//! an, sobald ein zweiter `NSSharingServicePicker` oder ein zweiter Bauer
//! danebentritt.
//!
//! # Die drei Werte von [`Quelle`] sind die ganze Fokusverzweigung
//!
//! Worauf das Teilen wirkt, entscheidet der Fokus, und [`worauf`] beantwortet
//! es als reine Rechnung ueber alle fuenf Fokuswerte. Sie steht hier und nicht
//! beim Anwendungsdelegierten, weil sie ohne Fenster pruefbar ist und dort
//! nicht mehr waere; der Delegierte verzweigt danach nur noch ueber die drei
//! Werte, die sie liefert, und holt zu jedem seine Pfade.
//!
//! # Diese beiden Huellen tragen keine Probe, und das ist Absicht
//!
//! Ein Aufruf oeffnet einen Dialog des Systems. Eine Probe, die ihn ausloeste,
//! oeffnete bei jedem `make check` ein Fenster, das niemand bestellt hat; das
//! ist derselbe Grund, aus dem [`super::standardprogramm::oeffnen`] und
//! [`super::zwischenablage::text_schreiben`] keine tragen. Geprueft wird
//! stattdessen, was ohne AppKit pruefbar ist: die Fokusverzweigung in
//! [`worauf`], die Menge der betroffenen Eintraege in
//! [`crate::kommandos::operationen::betroffene`] und der Satz in
//! [`crate::kommandos::operationen::nichts_zu_teilen`]. Dass der Dialog
//! aufgeht und AirDrop darin steht, sieht der Nutzer am gebauten Buendel
//! (C1, erstes Kriterium).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSharingServicePicker` steht seit macOS 10.8
//! (`NSSharingService.h:253`), ebenso `initWithItems:` (`:261`) und
//! `showRelativeToRect:ofView:preferredEdge:` (`:271`, ohne eigene Angabe
//! unter der Klassenangabe). **Die hoechste Untergrenze dieser Datei ist
//! `standardShareMenuItem` mit 13.0** (`:281`). `NSURL`, `NSArray`, `NSString`,
//! `NSView` und `NSMenu` tragen keine eigene Angabe und stehen damit seit
//! macOS 10.0, ebenso `fileURLWithPath:`, `NSView.bounds` (`NSView.h:139`),
//! `NSMenu.insertItem:atIndex:` (`NSMenu.h:89`), `NSMenu.numberOfItems`
//! (`NSMenu.h:118`) und `NSMenuItem.separatorItem` (`NSMenuItem.h:27`).
//! `NSRectEdge` ist eine Aufzaehlung aus Foundation (`NSGeometry.h:38`) und
//! traegt ebenfalls keine Angabe. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen,
//! und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::RefCell;
use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AnyThread, MainThreadMarker};
use objc2_app_kit::{NSMenu, NSMenuItem, NSSharingServicePicker, NSView};
use objc2_foundation::{NSArray, NSRect, NSRectEdge, NSString, NSURL};

use crate::kommandos::fokus::Fokus;

thread_local! {
    /// Der offene Freigabedialog, solange er offen ist.
    ///
    /// **Ohne diesen Halt faellt der Dialog dem Nutzer unter den Haenden weg,
    /// und der Fehler waere still.** `showRelativeToRect:ofView:preferredEdge:`
    /// kehrt sofort zurueck; der Dialog lebt danach weiter und gehoert dem
    /// `NSSharingServicePicker`, der ihn gezeigt hat. Ein `Retained`, das am
    /// Ende von [`anbieten`] faellt, nimmt ihm seinen Besitzer, und was AppKit
    /// mit einem Dialog ohne Besitzer tut, ist keine Zusage, auf die sich
    /// bauen liesse.
    ///
    /// **Einer und keine Reihe.** Der naechste Aufruf setzt den vorigen ab; es
    /// gibt genau einen Freigabedialog vor dem Nutzer, und ein zweiter
    /// festgehaltener waere ein Halt ohne Dialog. Abgesetzt wird er erst,
    /// nachdem der neue steht — deshalb die Zuweisung nach dem Zeigen und
    /// nicht davor.
    ///
    /// `thread_local!` und kein `static`: ein `Retained` ist nicht `Sync`, und
    /// hier gehoert es ohnehin dem Hauptfaden, auf dem AppKit allein arbeitet.
    static OFFENER_DIALOG: RefCell<Option<Retained<NSSharingServicePicker>>> =
        const { RefCell::new(None) };
}

/// Worauf das Teilen wirkt, sobald der Fokus feststeht (C1, zweites
/// Kriterium).
///
/// Drei Werte und kein Auffangzweig. Der Uebersetzer haelt an, sobald eine
/// vierte Herkunft dazukommt, und erzwingt damit ihre Einordnung beim
/// Anwendungsdelegierten, der die Pfade dazu holt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quelle {
    /// Die betroffenen Eintraege des aktiven Dateifensters, nach der Regel der
    /// Runde 4: die Markierung hat den Vorrang, sonst gilt der Eintrag unter
    /// der Auswahl.
    BetroffeneEintraege,
    /// Die angezeigte Datei, also die der sichtbaren Vorschau oder die des
    /// sichtbaren Editors.
    AngezeigteDatei,
    /// Nichts. Der Befehl wirkt, findet aber keine Menge und meldet es.
    Nichts,
}

/// Die Fokusverzweigung des Teilens, vollstaendig ueber alle fuenf Fokuswerte
/// (C1, zweites Kriterium).
///
/// **Der Fokus entscheidet nicht, ob der Befehl wirkt, sondern worauf.** Dass
/// er ueberhaupt durchkommt, hat [`Kommando::Teilen`](krk_core::tasten::Kommando::Teilen)
/// mit `Wirkungsbereich::Ueberall` schon entschieden; diese Rechnung
/// beantwortet die zweite Frage und keine davon ein zweites Mal.
///
/// Fuenf Werte, drei Antworten:
///
/// - [`Fokus::Dateifenster`] nimmt die betroffenen Eintraege des aktiven
///   Dateifensters. Das ist die Regel der Runde 4, und Teilen wird damit der
///   siebte Abnehmer von [`crate::kommandos::operationen::betroffene`]; eine
///   zweite Auswahlregel entsteht nicht.
/// - [`Fokus::Anderswo`] geht denselben Weg. Es ist kein Bereich, sondern ein
///   Befund: der Ersthelfer gehoert keinem der vier Bereiche. "Der Bereich vor
///   dem Nutzer" hat dann keine Antwort, das aktive Dateifenster aber immer
///   eine — dieselbe Erwaegung, die `cmd+w` seit C4 der Runde 4 dorthin
///   fuehrt.
/// - [`Fokus::Vorschau`] und [`Fokus::Editor`] nehmen die angezeigte Datei.
///   Welche das ist, beantwortet [`crate::angezeigtedatei::welche`] aus der
///   Sichtbarkeit der beiden Bereiche und nicht aus dem Fokus; der Fokus sagt
///   hier allein, dass die Datei gemeint ist und nicht die Liste.
/// - [`Fokus::Leiste`] findet nichts. Ein Lesezeichen ist ein Ort und kein
///   Eintrag, den ein Freigabedienst annaehme, und die Liste der Geraete
///   daneben erst recht nicht. Der Befehl wirkt trotzdem und meldet es;
///   wortlos nichts zu tun ist nach C2 der Runde 1 nicht zulaessig.
#[must_use]
pub fn worauf(fokus: Fokus) -> Quelle {
    match fokus {
        Fokus::Dateifenster | Fokus::Anderswo => Quelle::BetroffeneEintraege,
        Fokus::Vorschau | Fokus::Editor => Quelle::AngezeigteDatei,
        Fokus::Leiste => Quelle::Nichts,
    }
}

/// Zeigt den Freigabedialog des Systems fuer die genannten Eintraege (C1).
///
/// Liefert, ob ein Dialog gezeigt wurde. **Bei leerer Liste geschieht nichts
/// und die Antwort ist `false`**; die Meldung dazu gehoert dem Aufrufer, der
/// als einziger weiss, in welche Statuszeile sie soll (C1, fuenftes
/// Kriterium).
///
/// **Der Typ eines Eintrags wird nicht geprueft und die Menge nicht
/// beschraenkt.** Ordner gehen mit; was ein Dienst mit einem Ordner kann,
/// entscheidet der Dienst und nicht KRK (C1, viertes Kriterium). Dieselbe
/// Zurueckhaltung wie bei [`super::standardprogramm::oeffnen`], und aus
/// demselben Grund: eine Vorpruefung waere eine Vermutung ueber fremde
/// Dienste.
///
/// **Ein Aufruf fuer die ganze Menge, und keiner je Eintrag.** Anders als beim
/// Standardprogramm, wo fuenf markierte Dateien zu fuenf verschiedenen
/// Programmen gehoeren koennen: ein Freigabedienst nimmt eine Liste entgegen,
/// und fuenf Dialoge hintereinander waeren kein Teilen, sondern fuenfmal
/// dasselbe Fenster.
///
/// Der Anker ist ein Rechteck in den Koordinaten von `flaeche`; der Aufrufer
/// gibt deren `bounds`, und der Dialog haengt sich mit
/// [`NSRectEdge::MinY`] darunter.
#[must_use]
pub fn anbieten(pfade: &[PathBuf], flaeche: &NSView, rechteck: NSRect) -> bool {
    if pfade.is_empty() {
        return false;
    }
    let auswaehler = auswaehler_bauen(pfade);
    auswaehler.showRelativeToRect_ofView_preferredEdge(rechteck, flaeche, NSRectEdge::MinY);
    // Erst zeigen, dann festhalten: die Zuweisung setzt den vorigen Dialog ab,
    // und der soll gehen, nachdem der neue steht.
    OFFENER_DIALOG.with(|halt| *halt.borrow_mut() = Some(auswaehler));
    true
}

/// Haengt den Teilen-Eintrag des Systems an den Anfang eines Menues (C1,
/// sechstes und siebtes Kriterium).
///
/// **Der eine Menuebauer.** Alle drei Flaechen der Runde rufen ihn; keine baut
/// sich ein eigenes Menue. Bei leerer Liste geschieht nichts, und das Menue
/// bleibt, wie es war — ein Eintrag, der nichts zu teilen haette, waere der
/// stille Fehlschlag, den C1 ausschliesst.
///
/// **Der Eintrag kommt vom System und nicht von KRK.**
/// `standardShareMenuItem` liefert ihn samt Untermenue der Dienste; ein selbst
/// gebauter Eintrag muesste die Dienste selbst aufzaehlen und waere beim
/// naechsten Systemwechsel ueberholt.
///
/// **Er steht oben, und ein Trenner steht zwischen ihm und dem Bestand.** Was
/// AppKit einer `NSTextView` von sich aus gibt, bleibt vollstaendig stehen und
/// rueckt nach unten; KRKs Eintrag tritt daneben und nimmt nichts weg. Der
/// Trenner entsteht nur, wenn es einen Bestand gibt: in einem leeren Menue
/// waere er eine Linie ohne zwei Seiten.
///
/// **Fuenf Ansichten rufen ihn, auf zwei Wegen.** Ueber den Delegiertenhaken
/// der `NSTextView` die Textflaeche des Editors und die der Vorschau; ueber
/// `menuNeedsUpdate:` die Dateiliste, die Bildansicht der Vorschau und deren
/// Inhaltsflaeche. Auf dem zweiten Weg leert der Aufrufer das Menue vorher,
/// auf dem ersten nicht — dort ist der Bestand das, was AppKit gebaut hat.
pub fn eintrag_anfuegen(menue: &NSMenu, pfade: &[PathBuf], mtm: MainThreadMarker) {
    if pfade.is_empty() {
        return;
    }
    let eintrag = auswaehler_bauen(pfade).standardShareMenuItem(mtm);
    if menue.numberOfItems() > 0 {
        menue.insertItem_atIndex(&NSMenuItem::separatorItem(mtm), 0);
    }
    menue.insertItem_atIndex(&eintrag, 0);
}

/// Der `NSSharingServicePicker` fuer eine Menge von Pfaden.
///
/// Die eine Stelle, an der ein Pfad zu einem `NSURL` und eine Menge davon zu
/// einem `NSArray` wird. Beide Huellen darueber gehen durch sie; zwei
/// Umwandlungen nebeneinander waeren zwei Wahrheiten darueber, was KRK dem
/// System uebergibt.
///
/// **`fileURLWithPath:` fragt das Dateisystem nicht**, und diese Stelle fragt
/// es auch nicht: ein Eintrag, den es nicht mehr gibt, geht bis zum Dienst
/// durch, und was der dazu sagt, ist seine Sache. Dieselbe Zurueckhaltung wie
/// in [`super::standardprogramm`].
fn auswaehler_bauen(pfade: &[PathBuf]) -> Retained<NSSharingServicePicker> {
    let eintraege: Vec<Retained<AnyObject>> = pfade
        .iter()
        .map(|pfad| {
            let adresse = NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy()));
            Retained::into_super(Retained::into_super(adresse))
        })
        .collect();
    let liste = NSArray::from_retained_slice(&eintraege);
    // SAFETY: `initWithItems:` verlangt, dass jedes Element `NSPasteboardWriting`
    // erfuellt oder ein `NSItemProvider` oder ein `NSDocument` ist
    // (`NSSharingService.h:259`). Die Liste traegt ausschliesslich `NSURL`, und
    // `NSURL` erfuellt `NSPasteboardWriting` (`NSPasteboard.h:469`). Anderes
    // kommt nicht herein, weil die Umwandlung in dieser Funktion steht und
    // nicht beim Aufrufer.
    unsafe { NSSharingServicePicker::initWithItems(NSSharingServicePicker::alloc(), &liste) }
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::quelldateien;

    use super::*;

    /// Die erwartete Antwort je Fokuswert, von Hand geschrieben.
    ///
    /// **Von Hand und nicht aus [`worauf`] abgeleitet.** Eine Ableitung
    /// pruefte die Verzweigung gegen sich selbst und liefe mit jeder Aenderung
    /// stillschweigend mit; dieselbe Erwaegung, die
    /// `der_bereich_editor_fuehrt_genau_die_befehle_des_editors` in
    /// `belegungsmodell.rs` ihre Liste von Hand schreiben laesst.
    const TAFEL: [(Fokus, Quelle); 5] = [
        (Fokus::Dateifenster, Quelle::BetroffeneEintraege),
        (Fokus::Leiste, Quelle::Nichts),
        (Fokus::Vorschau, Quelle::AngezeigteDatei),
        (Fokus::Editor, Quelle::AngezeigteDatei),
        (Fokus::Anderswo, Quelle::BetroffeneEintraege),
    ];

    /// Die Fokusverzweigung als Tafel ueber alle fuenf Werte, an einem Stueck.
    #[test]
    fn jeder_der_fuenf_fokuswerte_traegt_seine_quelle() {
        for (fokus, erwartet) in TAFEL {
            assert_eq!(
                worauf(fokus),
                erwartet,
                "{fokus:?} sollte auf {erwartet:?} fuehren"
            );
        }
    }

    /// Die Tafel darueber nennt jeden Fokuswert genau einmal.
    ///
    /// Die zweite Haelfte der Vollstaendigkeit: der Uebersetzer erzwingt, dass
    /// [`worauf`] jeden Wert beantwortet, aber nicht, dass die Tafel jeden
    /// nennt. Ohne diese Probe liefe ein sechster Fokuswert ungeprueft mit,
    /// obwohl `worauf` ihn einordnen musste.
    #[test]
    fn die_tafel_nennt_jeden_fokuswert_genau_einmal() {
        for wert in Fokus::ALLE {
            assert_eq!(
                TAFEL.iter().filter(|(fokus, _)| *fokus == wert).count(),
                1,
                "{wert:?} steht nicht genau einmal in der Tafel"
            );
        }
        assert_eq!(TAFEL.len(), Fokus::ALLE.len());
    }

    /// Allein die Leiste findet nichts.
    ///
    /// Der Fall, um dessentwillen die Verzweigung drei Antworten hat und nicht
    /// zwei: mit dem Fokus in der Leiste gibt es weder betroffene Eintraege
    /// noch eine angezeigte Datei, auf die der Befehl ausweichen duerfte. Ein
    /// Rueckfall auf das aktive Dateifenster waere hier moeglich und ist
    /// ausdruecklich nicht gewaehlt — er teilte etwas, das der Nutzer nicht vor
    /// sich hat.
    #[test]
    fn allein_die_leiste_findet_nichts() {
        let ohne_quelle: Vec<Fokus> = Fokus::ALLE
            .into_iter()
            .filter(|fokus| worauf(*fokus) == Quelle::Nichts)
            .collect();
        assert_eq!(ohne_quelle, vec![Fokus::Leiste]);
    }

    /// Diese Datei ist die einzige, die den Freigabewaehler baut (C1, siebtes
    /// und achtes Kriterium).
    ///
    /// Gezaehlt werden Dateien und nicht Fundstellen: in dieser hier steht der
    /// Name mehrfach, im Kopf, im Rumpf und in dieser Probe.
    ///
    /// **Die Nadel traegt die beiden Doppelpunkte, und das ist der ganze
    /// Unterschied zwischen Nennen und Bauen.** Der Kopf von
    /// [`super`] nennt die Klasse in Prosa, und das ist keine Beruehrung. Wer
    /// einen Waehler baut, kommt an
    /// `NSSharingServicePicker::alloc` und `::initWithItems` nicht vorbei, und
    /// an einen fertigen kommt er nicht heran: aus dieser Datei kommt keiner
    /// heraus. Die Schreibweise mit Doppelpunkten trennt damit genau die
    /// beiden Faelle, um die es geht.
    #[test]
    fn allein_diese_datei_baut_den_freigabewaehler() {
        let bauer: Vec<String> = quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains("NSSharingServicePicker::"))
            .map(|(name, _)| name)
            .collect();
        assert_eq!(bauer, vec!["krk-ui/src/appkit/teilen.rs".to_owned()]);
    }

    /// Es gibt genau einen Menuebauer (C1, siebtes Kriterium).
    ///
    /// Zwei Zaehlungen, weil der Bauer zwei Haelften hat, die einzeln
    /// abwandern koennten: die Funktion selbst und der Eintrag des Systems,
    /// den sie holt. Gezaehlt werden hier Fundstellen und nicht Dateien; eine
    /// zweite Erklaerung in dieser Datei waere genauso ein zweiter Bauer wie
    /// eine in einer anderen.
    #[test]
    fn es_gibt_genau_einen_menuebauer() {
        // **Beide Nadeln stehen zusammengesetzt da, und das ist kein
        // Schnoerkel.** Die Probe liegt in dem Baum, den sie liest; als ein
        // Stueck geschrieben faende jede Nadel sich selbst und zaehlte eine
        // Fundstelle zu viel.
        let bauer = concat!("fn ", "eintrag_anfuegen");
        let systemeintrag = concat!(".standardShare", "MenuItem(");
        let dateien = quelldateien();
        let zaehlen = |nadel: &str| -> usize {
            dateien
                .iter()
                .map(|(_, inhalt)| inhalt.matches(nadel).count())
                .sum()
        };
        assert_eq!(
            zaehlen(bauer),
            1,
            "`eintrag_anfuegen` ist nicht genau einmal erklaert"
        );
        assert_eq!(
            zaehlen(systemeintrag),
            1,
            "der Teilen-Eintrag des Systems wird nicht genau einmal geholt"
        );
    }
}
