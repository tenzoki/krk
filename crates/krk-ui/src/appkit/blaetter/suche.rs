//! Das Eingabeblatt der Suche und des Ersetzens (C5).
//!
//! ```text
//!  ┌ Suchen nach:    [____________________] ┐  zwei Eingabefelder,
//!  │ Ersetzen durch: [____________________] ┘  ein Eingabewaechter
//!  └ [Abbrechen] [Alle ersetzen] [Ersetzen] [Weitersuchen]
//! ```
//!
//! **Das Blatt traegt seit dem 260929 alle drei Handlungen als Schaltflaeche.**
//! Bis dahin hatte es allein "Suche", und Ersetzen, Alle ersetzen und
//! Weitersuchen waren nur ueber ihre Tastenbefehle und das Menue "Editor" zu
//! erreichen; wer im Blatt nach ihnen suchte, fand sie nicht und hielt sie fuer
//! nicht gebaut. Die Schaltflaechen fuehren dieselben Wege des Editorbereichs
//! aus wie die Tastenbefehle und bauen keinen zweiten; welche Handlung was
//! bedeutet, steht bei [`Suchwahl`].
//!
//! **Ein Blatt fuer beide Befehle.** Der Spec traegt Suchen und Ersetzen unter
//! einem Buchstaben (C5), und der Ersatztext gehoert zum Suchtext: `cmd+f`
//! fragt nach beiden, `cmd+g` und `ctrl+cmd+g` gehen durch die Treffer,
//! `shift+cmd+r` und `ctrl+cmd+r` setzen den Ersatz ein. Ein zweites Blatt
//! allein fuer den Ersatztext waere eine zweite Stelle, an der der Nutzer
//! dieselbe Suche noch einmal beschreiben muesste.
//!
//! **Dieses Blatt sucht nicht.** Es liefert zwei Zeichenketten und die gewaehlte
//! [`Suchwahl`]; gesucht und
//! ersetzt wird in `krk_core::text::suche`, gehalten wird der Suchlauf in
//! `crate::editormodell`. Gross- und Kleinschreibung, regulaere Ausdruecke und
//! die Suchrichtung sind nach dem Spec **nicht** festgelegt und kommen nicht
//! hinzu; deshalb traegt das Blatt kein einziges Kaestchen. Eine Schaltflaeche
//! fuer das Rueckwaertssuchen traegt es aus demselben Grund nicht: sie waere
//! die fuenfte, und `ctrl+cmd+g` erreicht den vorigen Treffer, sobald das Blatt
//! zu ist. Jeder Schalter
//! waere ein Bedienelement und ein Abnahmekriterium mehr.
//!
//! **Ein Blatt haelt genau einen Eingabewaechter, auch bei zwei Feldern.** Der
//! Grund steht im Modulkopf von [`super`]: der Waechter entscheidet nicht nach
//! Feld, sondern beantwortet zwei Tasten, und die bedeuten in jedem Feld
//! dasselbe. Das Stapel-Umbenennen macht es mit vier Feldern vor.
//!
//! **Die beiden Startwerte kommen von der letzten Suche.** Wer `cmd+f` ein
//! zweites Mal drueckt, findet seinen Suchtext ausgewaehlt vor und tippt
//! entweder einen neuen oder bestaetigt den alten. Dieselbe Wahl und derselbe
//! Grund wie beim Startwert der Pfadeingabe.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField`, `NSView`, `NSWindow` und `NSString` stehen seit macOS 10.0
//! zur Verfuegung, ebenso die Aufzaehlung `NSTextAlignment` und die Zugriffe
//! `selectText:`, `setNextKeyView:`, `alignment`, `stringValue` und `frame`.
//!
//! **Eine einzige Beruehrung ist juenger als 10.0**, und sie liegt unter dem
//! Zielsystem: `labelWithString:` steht seit 10.12 (`NSTextField.h`). Alles
//! Weitere, was dieses Blatt braucht — das Aufgehen am Fenster, der
//! Eingabewaechter, die Schaltflaechen —, geht durch [`super::Blatt`]; die
//! Untergrenzen dazu nennt der Modulkopf von [`super`] und nicht dieser.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist ein Rust-Typ der
//! Kiste und hat kein macOS-Alter; `NSPoint`, `NSRect` und `NSSize` sind
//! C-Strukturen (`NSGeometry.h:23`, `:33` und `:28`); alle uebrigen tragen im
//! SDK keine eigene Verfuegbarkeitsangabe und stehen damit seit 10.0.

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextAlignment, NSTextField, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use krk_core::sprache::{Text, text};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Was der Nutzer im Blatt gewaehlt hat.
///
/// **Jeder Wert fuehrt einen Weg aus, den es als Tastenbefehl schon gibt**,
/// naemlich `cmd+g`, `shift+cmd+r` und `ctrl+cmd+r`; der Unterschied ist
/// allein, dass das Blatt die beiden Texte zuerst uebernimmt. Wie ein Wert
/// ausgefuehrt wird, entscheidet `Editorbereich::suchblatt_beantworten` und
/// nicht diese Datei.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suchwahl {
    /// Den naechsten Treffer ansteuern; bei einem neuen Suchtext den ersten ab
    /// der Schreibmarke.
    Weitersuchen,
    /// Den angesteuerten Treffer ersetzen und den naechsten ansteuern.
    Ersetzen,
    /// Alle Treffer in einem Zug ersetzen.
    AlleErsetzen,
}

/// Die Schaltflaechen des Blattes, in bindender Reihenfolge.
///
/// Die erste steht rechts und ist die hervorgehobene. Die Eingabetaste gehoert
/// "Weitersuchen", weil sie nichts veraendert; die beiden Ersetzungen liegen
/// auf Eingabetaste mit Befehls- und mit Wahltaste, damit eine reflexhafte
/// Bestaetigung nie ersetzt, und die Escape-Taste bricht ab. Die Stellen
/// liest [`suchwahl_von_stelle`] zurueck; dass beide zueinander passen, haelt
/// die Probe `jede_stelle_hat_ihre_wahl`.
///
/// `pub(super)`, damit die Proben in [`super`] diese Liste lesen, statt sie
/// nachzubauen.
#[must_use]
pub(super) fn schaltflaechen() -> [Schaltflaeche<'static>; 4] {
    [
        Schaltflaeche::neu(
            text(Text::SucheWeitersuchen),
            Taste::Eingabe,
            Wirkung::Ausfuehren,
        ),
        Schaltflaeche::neu(
            text(Text::SucheErsetzen),
            Taste::EingabeMitBefehl,
            Wirkung::Ausfuehren,
        ),
        Schaltflaeche::neu(
            text(Text::SucheAlleErsetzen),
            Taste::EingabeMitWahl,
            Wirkung::Ausfuehren,
        ),
        Schaltflaeche::neu(
            text(Text::BlattAbbrechen),
            Taste::Escape,
            Wirkung::Liegenlassen,
        ),
    ]
}

/// Welche Wahl die Schaltflaeche an dieser Stelle von [`schaltflaechen`]
/// bedeutet; `None` fuer den Abbruch und fuer jede Stelle, die es nicht gibt.
#[must_use]
fn suchwahl_von_stelle(stelle: usize) -> Option<Suchwahl> {
    match stelle {
        0 => Some(Suchwahl::Weitersuchen),
        1 => Some(Suchwahl::Ersetzen),
        2 => Some(Suchwahl::AlleErsetzen),
        _ => None,
    }
}

/// Der Satz unter der Frage, der die beiden Kombinationen mit Zusatztaste
/// nennt.
///
/// Ohne ihn waeren sie unauffindbar; so verlangt es der Doc-Kommentar von
/// [`Taste`]. Der Wortlaut folgt dem des Konfliktblattes (`Cmd+Return`,
/// `Opt+Return`) und steht als `Text::SucheErlaeuterung` in der Sprachtabelle;
/// die Probe `die_erlaeuterung_nennt_beide_zusatztasten` haelt ihn am
/// deutschen Eintrag gegen die angelegten Tasten.
#[must_use]
fn erlaeuterung() -> &'static str {
    text(Text::SucheErlaeuterung)
}

/// Die Breite der Beigabe in Punkten.
///
/// Sie bestimmt zugleich die Breite des Blattes: `NSAlert` waechst mit seiner
/// Beigabe.
const BREITE: f64 = 420.0;

/// Die Breite der Beschriftungsspalte links.
const BESCHRIFTUNG: f64 = 120.0;

/// Die Hoehe einer Eingabezeile.
const ZEILENHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen den beiden Eingabezeilen.
const ZEILENABSTAND: f64 = 6.0;

/// Die Hoehe einer Beschriftung.
const BESCHRIFTUNGSHOEHE: f64 = 17.0;

/// Der Abstand zwischen Beschriftung und Feld.
const SPALTENABSTAND: f64 = 8.0;

/// Zeigt die Frage nach Such- und Ersatztext am Fenster.
///
/// Kehrt sofort zurueck. `fertig` bekommt die gewaehlte [`Suchwahl`], den
/// Suchtext und den Ersatztext, in dieser Reihenfolge, und laeuft auf dem
/// Hauptfaden, wenn der Nutzer eine der drei ausfuehrenden Schaltflaechen
/// gewaehlt hat; bricht er ab, laeuft es gar nicht.
///
/// **Beide Texte gehen unveraendert hinaus**, ohne `trim` und ohne Wandlung.
/// Ein fuehrendes Leerzeichen ist im Suchtext ein Zeichen wie jedes andere,
/// anders als in einem Pfad. Die eine Wandlung, die der Ersatztext braucht,
/// steht in `krk_core::text::datei::in_gehaltene_form` und wird vom Modell
/// vorgenommen — vor dem Ersetzen und nicht danach.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    gesucht: &str,
    ersatz: &str,
    fertig: impl Fn(Suchwahl, String, String) + 'static,
) -> Blattgriff {
    let hoehe = 2.0f64.mul_add(ZEILENHOEHE, ZEILENABSTAND);
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, hoehe)),
    );

    // Von unten nach oben, weil AppKit von unten nach oben misst.
    let ersatzfeld = eingabezeile(
        mtm,
        &beigabe,
        text(Text::BlattFeldErsetzenDurch),
        0.0,
        ersatz,
    );
    let suchfeld = eingabezeile(
        mtm,
        &beigabe,
        text(Text::BlattFeldSuchenNach),
        ZEILENHOEHE + ZEILENABSTAND,
        gesucht,
    );

    // Der ganze Suchtext steht ausgewaehlt da: wer einen anderen sucht, tippt
    // ihn einfach, wer den vorhandenen ergaenzen will, drueckt zuerst Pfeil
    // rechts. Dieselbe Wahl wie bei der Pfadeingabe.
    // SAFETY: `selectText:` ist eine gewoehnliche Aktion von `NSControl`; sie
    // stellt keine Bedingung an ihren Absender, und `None` ist der Wert, den
    // ein programmatischer Aufruf dafuer setzt.
    unsafe { suchfeld.selectText(None) };

    // Der Ring, den der Tabulator abgeht. Er steht ausdruecklich da und wird
    // nicht AppKit ueberlassen, aus demselben Grund wie beim
    // Stapel-Umbenennen: die Reihenfolge, in der der Nutzer die Felder
    // ausfuellt, ist nicht die, in der sie in der Beigabe haengen.
    // SAFETY: `setNextKeyView:` verlangt vom Nachfolger allein, dass er eine
    // Ansicht ist und lebt. Beide haengen in der Beigabe und leben, solange das
    // Blatt steht; die Kette ist geschlossen und verweist auf keine Ansicht
    // ausserhalb.
    unsafe {
        suchfeld.setNextKeyView(Some(&ersatzfeld));
        ersatzfeld.setNextKeyView(Some(&suchfeld));
    }

    let mut blatt = Blatt::mit_schaltflaechen(mtm, text(Text::SucheFrage), &schaltflaechen());
    blatt.erlaeuterung_setzen(erlaeuterung());
    // Die drei Schritte einzeln und nicht ueber `textfeld_setzen`: die Beigabe
    // ist der Rahmen um die beiden Felder und nicht eines davon, und
    // Ersthelfer ist das Suchfeld. Der Waechter ist fuer beide derselbe.
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&suchfeld);
    blatt.waechter_anhaengen(mtm, &suchfeld);
    blatt.waechter_anhaengen(mtm, &ersatzfeld);

    let suchfeld: Retained<NSTextField> = suchfeld;
    let ersatzfeld: Retained<NSTextField> = ersatzfeld;
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| {
        if let Some(wahl) = suchwahl_von_stelle(stelle) {
            fertig(
                wahl,
                suchfeld.stringValue().to_string(),
                ersatzfeld.stringValue().to_string(),
            );
        }
    })
}

/// Eine beschriftete Eingabezeile, in die Beigabe gehaengt.
///
/// Derselbe Zuschnitt wie im Stapel-Umbenennen; die Ansichten bekommen feste
/// Rahmen und keine Auslegeregeln, weil die Beigabe eines `NSAlert` nicht mit
/// dem Fenster waechst.
fn eingabezeile(
    mtm: MainThreadMarker,
    beigabe: &NSView,
    beschriftung: &str,
    unterkante: f64,
    startwert: &str,
) -> Retained<NSTextField> {
    let text = NSTextField::labelWithString(&NSString::from_str(beschriftung), mtm);
    text.setFrame(NSRect::new(
        NSPoint::new(0.0, unterkante + 3.0),
        NSSize::new(BESCHRIFTUNG - SPALTENABSTAND, BESCHRIFTUNGSHOEHE),
    ));
    text.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&text);

    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(
            NSPoint::new(BESCHRIFTUNG, unterkante),
            NSSize::new(BREITE - BESCHRIFTUNG, ZEILENHOEHE),
        ),
    );
    feld.setStringValue(&NSString::from_str(startwert));
    beigabe.addSubview(&feld);
    feld
}

#[cfg(test)]
mod tests {
    use super::super::{Taste, Wirkung, abbruchstelle, bestaetigungsstelle};
    use super::*;

    /// Jede ausfuehrende Schaltflaeche bedeutet eine Wahl, und die
    /// liegenlassende keine.
    ///
    /// Haelt [`schaltflaechen`] und [`suchwahl_von_stelle`] aneinander: wer die
    /// Reihenfolge der einen dreht, ohne die andere nachzuziehen, bekommt hier
    /// die Beschriftung der verrutschten Schaltflaeche im Fehlschlag.
    #[test]
    fn jede_stelle_hat_ihre_wahl() {
        let erwartet = [
            ("Weitersuchen", Some(Suchwahl::Weitersuchen)),
            ("Ersetzen", Some(Suchwahl::Ersetzen)),
            ("Alle ersetzen", Some(Suchwahl::AlleErsetzen)),
            ("Abbrechen", None),
        ];
        let liste = schaltflaechen();
        assert_eq!(liste.len(), erwartet.len());
        for (stelle, (schaltflaeche, (titel, wahl))) in liste.iter().zip(erwartet).enumerate() {
            assert_eq!(schaltflaeche.titel, titel, "an Stelle {stelle}");
            assert_eq!(
                suchwahl_von_stelle(stelle),
                wahl,
                "\"{titel}\" bedeutet die falsche Wahl"
            );
            assert_eq!(
                schaltflaeche.wirkung == Wirkung::Liegenlassen,
                wahl.is_none(),
                "\"{titel}\" hat eine Wirkung, die nicht zu ihrer Wahl passt"
            );
        }
        assert_eq!(suchwahl_von_stelle(liste.len()), None);
    }

    /// Die Eingabetaste ersetzt nie, und ein Abbruch faellt auf "Abbrechen".
    #[test]
    fn die_eingabetaste_sucht_und_ersetzt_nicht() {
        let liste = schaltflaechen();
        assert_eq!(
            suchwahl_von_stelle(bestaetigungsstelle(&liste)),
            Some(Suchwahl::Weitersuchen)
        );
        assert_eq!(suchwahl_von_stelle(abbruchstelle(&liste)), None);
        assert_eq!(liste[abbruchstelle(&liste)].taste, Taste::Escape);
    }

    /// Der erlaeuternde Satz nennt jede Kombination mit Zusatztaste, die eine
    /// Schaltflaeche traegt.
    ///
    /// Gemessen am deutschen Tabelleneintrag, weil die Probe die Mechanik
    /// haelt (jede angelegte Zusatztaste steht im Satz) und nicht den
    /// Wortlaut; die Tastennamen der anderen Sprachen sagt das Glossar in
    /// `krk_core::sprache::tabelle`.
    #[test]
    fn die_erlaeuterung_nennt_beide_zusatztasten() {
        let erlaeuterung = krk_core::sprache::Sprache::De.text(Text::SucheErlaeuterung);
        for schaltflaeche in schaltflaechen() {
            let angesagt = match schaltflaeche.taste {
                Taste::EingabeMitBefehl => "Cmd+Return",
                Taste::EingabeMitWahl => "Opt+Return",
                Taste::Eingabe | Taste::Escape => continue,
            };
            assert!(
                erlaeuterung.contains(angesagt),
                "die Erlaeuterung nennt {angesagt} fuer \"{}\" nicht",
                schaltflaeche.titel
            );
        }
    }
}
