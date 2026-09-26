//! Das PIN-Blatt vor `.secrets.txt` (C7 der krkhome-Arbeit, Schritt 5.4b).
//!
//! ```text
//!  PIN für die Geheimnisse eingeben          Neue PIN für die Geheimnisse festlegen
//!  <HINWEIS>                                 <HINWEIS>
//!  ┌ PIN:  [••••] ┐                          ┌ Neue PIN:    [••••] ┐
//!  └ <Grund>      ┘                          │ Wiederholen: [••••] │
//!  [Abbrechen]  [Öffnen]                     └ <Grund>             ┘
//!                                            [Abbrechen]  [Festlegen]
//! ```
//!
//! **Zwei Formen, ein Blatt.** Welche gilt, sagt [`Pinform`], und entschieden
//! wird sie an der Groesse der Datei in `Editorbereich::datei_oeffnen`: eine
//! leere `.secrets.txt` hat noch keine PIN, und der Nutzer legt eine fest,
//! **zweimal eingegeben**, weil eine vertippte neue PIN den Inhalt so
//! endgueltig verschloesse wie eine vergessene (Spec, C7). Jede andere gibt er
//! einmal ein. Das Blatt liefert eine [`Pin`] und oeffnet nichts; geoeffnet
//! wird ueber `Editorbereich::geheimnisse_oeffnen`.
//!
//! **Geprueft wird ueber [`Pin::aus_eingabe`], und eine abgewiesene Eingabe
//! schliesst das Blatt nicht.** `NSAlert` beendet ein Blatt mit jedem Druck
//! auf eine Schaltflaeche; deshalb ist die bestaetigende abgeschaltet, solange
//! [`eingabe_pruefen`] nein sagt, und die Eingabetaste im Feld prueft vor dem
//! Bestaetigen ([`super::Blatt::bestaetigung_pruefen`]). Warum nicht, steht in
//! der Zeile unter den Feldern.
//!
//! **Die Felder sind `NSSecureTextField`**: die Ziffern erscheinen als Punkte,
//! und das System haelt sie aus der Zwischenablage und aus der
//! Bedienungshilfe heraus. Die PIN geht weder in ein Protokoll noch in eine
//! Meldung; `Pin` zeigt in `Debug` keine Ziffern, und dieses Modul schreibt
//! die Eingabe nirgends hin ausser in [`eingabe_pruefen`].
//!
//! **Das Feld ist nicht als eigene Textflaeche angemeldet**, wie das Feld
//! jedes Blattes: `Esc` schliesst das Blatt nur, weil der Ersthelfer AppKit
//! gehoert (Modulkopf von [`super`]).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSecureTextField` (ueber `NSTextField`, `NSControl`, `NSView` und
//! `NSResponder`), `NSTextField`, `NSButton`, `NSView`, `NSWindow` und
//! `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso die Aufzaehlung
//! `NSTextAlignment` und die Zugriffe `initWithFrame:`, `stringValue`,
//! `setStringValue:`, `setEnabled:`, `setNextKeyView:`, `setAlignment:`,
//! `setFrame:` und `addSubview:`.
//!
//! **Eine einzige Beruehrung ist juenger als 10.0**, und sie liegt unter dem
//! Zielsystem: `labelWithString:` steht seit 10.12 (`NSTextField.h`). Alles
//! Weitere — das Aufgehen am Fenster, der Eingabewaechter, die Schaltflaechen
//! — geht durch [`super::Blatt`]; die Untergrenzen dazu nennt der Modulkopf
//! von [`super`] und nicht dieser.
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
//! C-Strukturen (`NSGeometry.h`); alle uebrigen tragen im SDK keine eigene
//! Verfuegbarkeitsangabe und stehen damit seit 10.0.

use std::rc::Rc;

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSSecureTextField, NSTextAlignment, NSTextField, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use krk_core::heimordner::tresor::{Pin, Pinfehler};

use crate::editormodell::Pinform;

use super::{Blatt, Blattgriff};

/// Der Text unter der Frage, in beiden Formen derselbe (C7).
///
/// **Der Wortlaut des Spec, ohne Zeitangabe**: wovor die PIN schuetzt, wovor
/// nicht, und dass eine vergessene PIN den Inhalt endgueltig verschliesst. Mit
/// Umlauten, weil der Nutzer ihn durch KRKs Oberflaeche liest; die Probe
/// `der_wortlaut_des_blattes` haelt ihn fest.
pub const HINWEIS: &str = "Die PIN hält Programme und Agenten vom Mitlesen ab. \
    Gegen jemanden, der die Datei kopiert und gezielt angreift, schützt sie nicht. \
    Eine vergessene PIN verschließt den Inhalt endgültig.";

/// Der Grund, wenn die zweite Eingabe beim Festlegen von der ersten abweicht.
pub const ABWEICHUNG: &str = "Die beiden Eingaben stimmen nicht überein.";

/// Die Breite der Beigabe in Punkten; sie bestimmt die Breite des Blattes.
const BREITE: f64 = 300.0;

/// Die Breite der Beschriftungsspalte links.
const BESCHRIFTUNG: f64 = 110.0;

/// Die Breite eines Eingabefeldes: vier Ziffern brauchen wenig Platz.
const FELDBREITE: f64 = 90.0;

/// Die Hoehe einer Eingabezeile.
const ZEILENHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen zwei Zeilen.
const ZEILENABSTAND: f64 = 6.0;

/// Die Hoehe einer Beschriftung und der Zeile mit dem Grund.
const BESCHRIFTUNGSHOEHE: f64 = 17.0;

/// Der Abstand zwischen Beschriftung und Feld.
const SPALTENABSTAND: f64 = 8.0;

/// Die Frage oben im Blatt.
#[must_use]
pub fn frage(form: Pinform) -> &'static str {
    match form {
        Pinform::Festlegen => "Neue PIN für die Geheimnisse festlegen",
        Pinform::Eingeben => "PIN für die Geheimnisse eingeben",
    }
}

/// Die Beschriftung der bestaetigenden Schaltflaeche.
#[must_use]
pub fn bestaetigen(form: Pinform) -> &'static str {
    match form {
        Pinform::Festlegen => "Festlegen",
        Pinform::Eingeben => "Öffnen",
    }
}

/// Die Beschriftungen der Felder, von oben nach unten; ihre Zahl ist die Zahl
/// der Felder.
#[must_use]
pub fn beschriftungen(form: Pinform) -> &'static [&'static str] {
    match form {
        Pinform::Festlegen => &["Neue PIN:", "Wiederholen:"],
        Pinform::Eingeben => &["PIN:"],
    }
}

/// Warum eine Eingabe im Blatt nicht angenommen wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eingabefehler {
    /// Die Eingabe ist keine PIN; den Satz liefert der Kern.
    KeinePin(Pinfehler),
    /// Beim Festlegen weicht die zweite Eingabe von der ersten ab.
    Abweichung,
}

impl Eingabefehler {
    /// Der Satz fuer die Zeile unter den Feldern.
    #[must_use]
    pub fn meldung(self) -> &'static str {
        match self {
            Eingabefehler::KeinePin(fehler) => fehler.meldung(),
            Eingabefehler::Abweichung => ABWEICHUNG,
        }
    }
}

/// Die Pruefung hinter dem Blatt: die erste Eingabe ist eine PIN, und beim
/// Festlegen gleicht die zweite ihr.
///
/// **Die eine Pruefung**, gefragt von der Schaltflaeche, von der Eingabetaste
/// und vom Abschluss des Blattes; die Regel fuer vier Ziffern steht allein in
/// [`Pin::aus_eingabe`]. `zweite` ist `None` in der Form
/// [`Pinform::Eingeben`].
pub fn eingabe_pruefen(erste: &str, zweite: Option<&str>) -> Result<Pin, Eingabefehler> {
    let pin = Pin::aus_eingabe(erste).map_err(Eingabefehler::KeinePin)?;
    match zweite {
        Some(wiederholt) if wiederholt != erste => Err(Eingabefehler::Abweichung),
        Some(_) | None => Ok(pin),
    }
}

/// Der Grund, der schon waehrend des Tippens unter den Feldern steht; `None`,
/// solange die Eingabe noch gueltig werden kann.
///
/// **Die Zeile meldet sich erst, wenn Weitertippen nicht mehr hilft**: ein
/// Zeichen, das keine Ziffer ist, eine fuenfte Ziffer, oder eine Wiederholung,
/// die nicht mehr der Anfang der ersten Eingabe ist. Eine halbe PIN ist kein
/// Fehler; wer mit ihr bestaetigt, bekommt den Grund ueber
/// [`eingabe_pruefen`].
#[must_use]
pub fn grund_beim_tippen(erste: &str, zweite: Option<&str>) -> Option<&'static str> {
    let kann_pin_werden =
        |text: &str| text.len() <= 4 && text.bytes().all(|zeichen| zeichen.is_ascii_digit());
    if !kann_pin_werden(erste) {
        return Some(Pinfehler::KeineVierZiffern.meldung());
    }
    match zweite {
        Some(wiederholt) if !erste.starts_with(wiederholt) => Some(ABWEICHUNG),
        Some(_) | None => None,
    }
}

/// Zeigt das PIN-Blatt in der genannten Form am Fenster.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden mit der
/// gepruefte PIN, wenn der Nutzer bestaetigt hat; bricht er ab, laeuft es gar
/// nicht, wie bei jedem Eingabeblatt dieses Verzeichnisses.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    form: Pinform,
    fertig: impl Fn(Pin) + 'static,
) -> Blattgriff {
    let namen = beschriftungen(form);
    let zeilen = namen.len() as f64;
    let hoehe = zeilen.mul_add(ZEILENHOEHE + ZEILENABSTAND, BESCHRIFTUNGSHOEHE);
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, hoehe)),
    );

    // Unten die Zeile mit dem Grund, darueber die Felder, von unten nach oben
    // gebaut, weil AppKit von unten nach oben misst.
    let grund = NSTextField::labelWithString(&NSString::from_str(""), mtm);
    grund.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(BREITE, BESCHRIFTUNGSHOEHE),
    ));
    beigabe.addSubview(&grund);
    let mut felder: Vec<Retained<NSSecureTextField>> = namen
        .iter()
        .rev()
        .enumerate()
        .map(|(stelle, name)| {
            let unterkante = (stelle as f64).mul_add(
                ZEILENHOEHE + ZEILENABSTAND,
                BESCHRIFTUNGSHOEHE + ZEILENABSTAND,
            );
            eingabezeile(mtm, &beigabe, name, unterkante)
        })
        .collect();
    felder.reverse();

    // Der Ring, den der Tabulator abgeht, ausdruecklich und geschlossen, wie
    // in der Suche. Bei einem Feld verweist es auf sich selbst.
    // SAFETY: `setNextKeyView:` verlangt vom Nachfolger allein, dass er eine
    // Ansicht ist und lebt. Alle Felder haengen in der Beigabe und leben,
    // solange das Blatt steht; die Kette verweist auf keine Ansicht ausserhalb.
    for (stelle, feld) in felder.iter().enumerate() {
        let naechstes = &felder[(stelle + 1) % felder.len()];
        unsafe { feld.setNextKeyView(Some(naechstes)) };
    }

    let mut blatt = Blatt::neu(mtm, frage(form), bestaetigen(form));
    blatt.erlaeuterung_setzen(HINWEIS);
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&felder[0]);
    for feld in &felder {
        blatt.waechter_anhaengen(mtm, feld);
    }

    // Was in den Feldern steht, fuer die drei Frager derselben Pruefung.
    let felder = Rc::new(felder);
    let lesen = {
        let felder = Rc::clone(&felder);
        move || -> (String, Option<String>) {
            let erste = felder[0].stringValue().to_string();
            let zweite = felder.get(1).map(|feld| feld.stringValue().to_string());
            (erste, zweite)
        }
    };
    let lesen = Rc::new(lesen);

    if let Some(knopf) = blatt.bestaetigende_schaltflaeche() {
        knopf.setEnabled(false);
        let lesen = Rc::clone(&lesen);
        let grund = grund.clone();
        blatt.textaenderung_melden(Box::new(move || {
            let (erste, zweite) = lesen();
            knopf.setEnabled(eingabe_pruefen(&erste, zweite.as_deref()).is_ok());
            let satz = grund_beim_tippen(&erste, zweite.as_deref()).unwrap_or("");
            grund.setStringValue(&NSString::from_str(satz));
        }));
    }
    {
        let lesen = Rc::clone(&lesen);
        let grund = grund.clone();
        blatt.bestaetigung_pruefen(Box::new(move || {
            let (erste, zweite) = lesen();
            match eingabe_pruefen(&erste, zweite.as_deref()) {
                Ok(_) => true,
                Err(fehler) => {
                    grund.setStringValue(&NSString::from_str(fehler.meldung()));
                    false
                }
            }
        }));
    }

    blatt.zeigen(fenster, move |bestaetigt| {
        if !bestaetigt {
            return;
        }
        // Noch einmal geprueft und nicht vorausgesetzt: die Schaltflaeche und
        // die Taste haben schon geprueft, aber eine Antwort von AppKit, die zu
        // keiner Schaltflaeche gehoert, kommt ohne sie hierher.
        let (erste, zweite) = lesen();
        if let Ok(pin) = eingabe_pruefen(&erste, zweite.as_deref()) {
            fertig(pin);
        }
    })
}

/// Eine beschriftete Eingabezeile mit einem verdeckten Feld, in die Beigabe
/// gehaengt; derselbe Zuschnitt wie in der Suche.
fn eingabezeile(
    mtm: MainThreadMarker,
    beigabe: &NSView,
    beschriftung: &str,
    unterkante: f64,
) -> Retained<NSSecureTextField> {
    let text = NSTextField::labelWithString(&NSString::from_str(beschriftung), mtm);
    text.setFrame(NSRect::new(
        NSPoint::new(0.0, unterkante + 3.0),
        NSSize::new(BESCHRIFTUNG - SPALTENABSTAND, BESCHRIFTUNGSHOEHE),
    ));
    text.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&text);

    let feld = NSSecureTextField::initWithFrame(
        NSSecureTextField::alloc(mtm),
        NSRect::new(
            NSPoint::new(BESCHRIFTUNG, unterkante),
            NSSize::new(FELDBREITE, ZEILENHOEHE),
        ),
    );
    beigabe.addSubview(&feld);
    feld
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Der Wortlaut des Blattes, Zeichen fuer Zeichen und mit Umlauten (C7:
    /// „eine Probe haelt ihren Wortlaut fest").
    #[test]
    fn der_wortlaut_des_blattes() {
        assert_eq!(
            HINWEIS,
            "Die PIN hält Programme und Agenten vom Mitlesen ab. Gegen jemanden, der die \
             Datei kopiert und gezielt angreift, schützt sie nicht. Eine vergessene PIN \
             verschließt den Inhalt endgültig."
        );
        assert_eq!(
            frage(Pinform::Festlegen),
            "Neue PIN für die Geheimnisse festlegen"
        );
        assert_eq!(frage(Pinform::Eingeben), "PIN für die Geheimnisse eingeben");
        assert_eq!(bestaetigen(Pinform::Festlegen), "Festlegen");
        assert_eq!(bestaetigen(Pinform::Eingeben), "Öffnen");
        assert_eq!(
            beschriftungen(Pinform::Festlegen),
            ["Neue PIN:", "Wiederholen:"]
        );
        assert_eq!(beschriftungen(Pinform::Eingeben), ["PIN:"]);
        assert_eq!(ABWEICHUNG, "Die beiden Eingaben stimmen nicht überein.");
        assert_eq!(
            Eingabefehler::KeinePin(Pinfehler::KeineVierZiffern).meldung(),
            "Die PIN besteht aus genau vier Ziffern."
        );
    }

    /// Eine neue PIN wird zweimal eingegeben, eine vorhandene einmal (C7).
    #[test]
    fn festlegen_fragt_zweimal_und_eingeben_einmal() {
        assert_eq!(beschriftungen(Pinform::Festlegen).len(), 2);
        assert_eq!(beschriftungen(Pinform::Eingeben).len(), 1);
    }

    /// Hinter dem Blatt steht `Pin::aus_eingabe`: drei Ziffern, fuenf
    /// Ziffern, Buchstaben und Leerraum kommen nicht durch, vier Ziffern
    /// schon.
    #[test]
    fn das_blatt_nimmt_nur_vier_ziffern_an() {
        for falsch in ["123", "12345", "12a4", " 1234", "1234 ", ""] {
            assert_eq!(
                eingabe_pruefen(falsch, None),
                Err(Eingabefehler::KeinePin(Pinfehler::KeineVierZiffern)),
                "{falsch:?} kam durch"
            );
        }
        assert_eq!(
            eingabe_pruefen("0427", None),
            Pin::aus_eingabe("0427").map_err(Eingabefehler::KeinePin)
        );
        assert!(eingabe_pruefen("0000", None).is_ok());
        assert!(eingabe_pruefen("9999", None).is_ok());
    }

    /// Beim Festlegen muss die Wiederholung gleichen; die Regel fuer vier
    /// Ziffern gilt davor.
    #[test]
    fn die_wiederholung_muss_gleichen() {
        assert!(eingabe_pruefen("1234", Some("1234")).is_ok());
        assert_eq!(
            eingabe_pruefen("1234", Some("1243")),
            Err(Eingabefehler::Abweichung)
        );
        assert_eq!(
            eingabe_pruefen("1234", Some("")),
            Err(Eingabefehler::Abweichung)
        );
        assert_eq!(
            eingabe_pruefen("123", Some("123")),
            Err(Eingabefehler::KeinePin(Pinfehler::KeineVierZiffern))
        );
    }

    /// Waehrend des Tippens meldet sich die Zeile erst, wenn Weitertippen
    /// nicht mehr hilft.
    #[test]
    fn der_grund_beim_tippen() {
        assert_eq!(grund_beim_tippen("", None), None);
        assert_eq!(grund_beim_tippen("12", None), None);
        assert_eq!(grund_beim_tippen("1234", None), None);
        let vier_ziffern = Some(Pinfehler::KeineVierZiffern.meldung());
        assert_eq!(grund_beim_tippen("12a", None), vier_ziffern);
        assert_eq!(grund_beim_tippen("12345", None), vier_ziffern);
        assert_eq!(grund_beim_tippen("1234", Some("12")), None);
        assert_eq!(grund_beim_tippen("1234", Some("13")), Some(ABWEICHUNG));
        assert_eq!(grund_beim_tippen("12", Some("1234")), Some(ABWEICHUNG));
        assert_eq!(grund_beim_tippen("1234", Some("1234")), None);
    }
}
