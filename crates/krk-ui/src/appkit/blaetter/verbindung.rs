//! Das Blatt „Verbinden“ hinter dem Kontextmenue der Dateiliste (261016,
//! Schritt 7 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//!
//! ```text
//!  Neue Verbindung
//!  <HINWEIS>
//!  ┌ Name:      [               ] ┐
//!  │ Protokoll: [               ] │
//!  │ Adresse:   [               ] │
//!  │ Konto:     [               ] │
//!  │ Passwort:  [•••••••••••••••] │
//!  │ Notizen:   [               ] │
//!  └ <Grund>                      ┘
//!  [Abbrechen]  [Anlegen]
//! ```
//!
//! **Das Blatt liefert einen Entwurf und schreibt nichts** (Entscheidung 13):
//! ob der Name in `secrets.txt` frei ist, weiss erst der Schreibgang nach der
//! PIN, und geht er schief, oeffnet der Anwendungsdelegierte dieses Blatt mit
//! denselben Feldern und dem Grund wieder. Die Notizen sind hier einzeilig;
//! Absaetze schreibt der Nutzer in der Tabelle des Editors.
//!
//! **Geprueft wird ueber [`tresorweg::entwurf_pruefen`], und eine abgewiesene
//! Eingabe schliesst das Blatt nicht**, wie im PIN-Blatt: die bestaetigende
//! Schaltflaeche ist abgeschaltet, solange die Pruefung nein sagt, und die
//! Eingabetaste prueft vor dem Bestaetigen. Warum nicht, steht in der Zeile
//! unter den Feldern.
//!
//! **Drei Schutzvorkehrungen fuer die Zugangsdaten** (Entscheidung 16): das
//! Passwortfeld ist ein `NSSecureTextField`; der Griff traegt
//! [`Blattgriff::verdeckt_machen`], also schreibt `--tasten-protokoll` fuer
//! jedes Feld dieses Blattes `(verdeckt)`; und bei jedem Beginn einer
//! Bearbeitung schaltet [`Blatt::bearbeitungsbeginn_melden`] am Feldeditor die
//! Automatiken ab ([`textautomatik::automatiken_abschalten`]), denn die Felder
//! bekommen den Feldeditor des Blattfensters, den KRK nicht baut. Die Probe
//! `das_verbindungsblatt_schaltet_die_automatiken_ab` haelt den Aufruf, weil
//! `jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab` Felder mit dem
//! Feldeditor des Fensters nicht sieht.
//!
//! **Die Felder sind nicht als eigene Textflaeche angemeldet**, wie das Feld
//! jedes Blattes: `Esc` schliesst das Blatt nur, weil der Ersthelfer AppKit
//! gehoert (Modulkopf von [`super`]).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSecureTextField`, `NSTextField`, `NSTextView`, `NSView`, `NSWindow` und
//! `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso die Aufzaehlung
//! `NSTextAlignment` und die Zugriffe `initWithFrame:`, `stringValue`,
//! `setStringValue:`, `setEnabled:`, `setNextKeyView:`, `setAlignment:`,
//! `setFrame:` und `addSubview:`. **Eine Beruehrung ist juenger als 10.0**,
//! und sie liegt unter dem Zielsystem: `labelWithString:` steht seit 10.12
//! (`NSTextField.h`). Das Aufgehen am Fenster, der Eingabewaechter und die
//! Schaltflaechen gehen durch [`super::Blatt`]; die Untergrenzen dazu nennt der
//! Modulkopf von [`super`].
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

use krk_core::sprache::{Text, text};

use crate::tresorweg::{self, Verbindungsentwurf};

use super::super::textautomatik;
use super::{Blatt, Blattgriff};

/// Die Breite der Beigabe in Punkten; sie bestimmt die Breite des Blattes.
const BREITE: f64 = 380.0;

/// Die Breite der Beschriftungsspalte links.
const BESCHRIFTUNG: f64 = 100.0;

/// Die Hoehe einer Eingabezeile.
const ZEILENHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen zwei Zeilen.
const ZEILENABSTAND: f64 = 6.0;

/// Die Hoehe einer Beschriftung und der Zeile mit dem Grund.
const BESCHRIFTUNGSHOEHE: f64 = 17.0;

/// Der Abstand zwischen Beschriftung und Feld.
const SPALTENABSTAND: f64 = 8.0;

/// Die Stelle des Passwortfeldes unter den sechs, von oben gezaehlt.
const PASSWORTSTELLE: usize = 4;

/// Die Beschriftungen der sechs Felder, von oben nach unten.
///
/// Ein `Vec` und kein fester Verweis, weil die Eintraege erst in der
/// geltenden Sprache entstehen.
#[must_use]
pub fn beschriftungen() -> Vec<&'static str> {
    [
        Text::VerbindenFeldName,
        Text::VerbindenFeldProtokoll,
        Text::VerbindenFeldAdresse,
        Text::VerbindenFeldKonto,
        Text::VerbindenFeldPasswort,
        Text::VerbindenFeldNotizen,
    ]
    .into_iter()
    .map(text)
    .collect()
}

/// Die Felder als Entwurf, in der Reihenfolge der Beschriftungen; fehlt ein
/// Feld, gilt es als leer.
#[must_use]
pub fn entwurf_aus(felder: &[String]) -> Verbindungsentwurf {
    let feld = |stelle: usize| felder.get(stelle).cloned().unwrap_or_default();
    Verbindungsentwurf {
        name: feld(0),
        protokoll: feld(1),
        adresse: feld(2),
        konto: feld(3),
        passwort: zeroize::Zeroizing::new(feld(PASSWORTSTELLE)),
        notizen: feld(5),
    }
}

/// Die Werte eines Entwurfs in der Reihenfolge der Felder, fuer das erneute
/// Oeffnen mit denselben Feldern.
fn werte(entwurf: &Verbindungsentwurf) -> [&str; 6] {
    [
        &entwurf.name,
        &entwurf.protokoll,
        &entwurf.adresse,
        &entwurf.konto,
        entwurf.passwort.as_str(),
        &entwurf.notizen,
    ]
}

/// Zeigt das Blatt „Verbinden“ am Fenster, mit den Feldern aus `entwurf` und,
/// wenn es einen gibt, dem Grund, aus dem es wieder aufgeht.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden mit dem
/// geprueften Entwurf, wenn der Nutzer bestaetigt hat; bricht er ab, laeuft
/// es gar nicht, wie bei jedem Eingabeblatt dieses Verzeichnisses.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    entwurf: &Verbindungsentwurf,
    grund: Option<&str>,
    fertig: impl Fn(Verbindungsentwurf) + 'static,
) -> Blattgriff {
    let namen = beschriftungen();
    let zeilen = namen.len() as f64;
    let hoehe = zeilen.mul_add(ZEILENHOEHE + ZEILENABSTAND, BESCHRIFTUNGSHOEHE);
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, hoehe)),
    );

    // Unten die Zeile mit dem Grund, darueber die Felder, von unten nach oben
    // gebaut, weil AppKit von unten nach oben misst.
    let grundzeile = NSTextField::labelWithString(&NSString::from_str(grund.unwrap_or("")), mtm);
    grundzeile.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(BREITE, BESCHRIFTUNGSHOEHE),
    ));
    beigabe.addSubview(&grundzeile);
    let vorbelegt = werte(entwurf);
    let mut felder: Vec<Retained<NSTextField>> = namen
        .iter()
        .enumerate()
        .rev()
        .enumerate()
        .map(|(von_unten, (stelle, name))| {
            let unterkante = (von_unten as f64).mul_add(
                ZEILENHOEHE + ZEILENABSTAND,
                BESCHRIFTUNGSHOEHE + ZEILENABSTAND,
            );
            let feld = eingabezeile(mtm, &beigabe, name, unterkante, stelle == PASSWORTSTELLE);
            feld.setStringValue(&NSString::from_str(vorbelegt[stelle]));
            feld
        })
        .collect();
    felder.reverse();

    // Der Ring, den der Tabulator abgeht, ausdruecklich und geschlossen, wie
    // im PIN-Blatt.
    // SAFETY: `setNextKeyView:` verlangt vom Nachfolger allein, dass er eine
    // Ansicht ist und lebt. Alle Felder haengen in der Beigabe und leben,
    // solange das Blatt steht.
    for (stelle, feld) in felder.iter().enumerate() {
        let naechstes = &felder[(stelle + 1) % felder.len()];
        unsafe { feld.setNextKeyView(Some(naechstes)) };
    }

    let mut blatt = Blatt::neu(
        mtm,
        text(Text::VerbindenFrage),
        text(Text::AnlegenBestaetigen),
    );
    blatt.erlaeuterung_setzen(text(Text::VerbindenHinweis));
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&felder[0]);
    for feld in &felder {
        blatt.waechter_anhaengen(mtm, feld);
    }
    // Die Automatiken aus, bei jedem Beginn einer Bearbeitung: der
    // Feldeditor gehoert dem Blattfenster, und AppKit richtet ihn je Feld
    // neu ein.
    blatt.bearbeitungsbeginn_melden(Box::new(|feldeditor| {
        textautomatik::automatiken_abschalten(feldeditor);
    }));

    // Was in den Feldern steht, fuer die drei Frager derselben Pruefung.
    let felder = Rc::new(felder);
    let lesen = {
        let felder = Rc::clone(&felder);
        move || -> Verbindungsentwurf {
            let werte: Vec<String> = felder
                .iter()
                .map(|feld| feld.stringValue().to_string())
                .collect();
            entwurf_aus(&werte)
        }
    };
    let lesen = Rc::new(lesen);

    if let Some(knopf) = blatt.bestaetigende_schaltflaeche() {
        knopf.setEnabled(tresorweg::entwurf_pruefen(&lesen()).is_ok());
        let lesen = Rc::clone(&lesen);
        let grundzeile = grundzeile.clone();
        blatt.textaenderung_melden(Box::new(move || {
            let geprueft = tresorweg::entwurf_pruefen(&lesen());
            knopf.setEnabled(geprueft.is_ok());
            // Der Grund erst, wenn ein Feld etwas traegt, das nie gilt: ein
            // Zeilenumbruch oder ein unbekanntes Protokoll. Ein leerer Name
            // oder eine noch leere Adresse sind beim Tippen kein Fehler.
            let satz = match geprueft {
                Err(fehler @ tresorweg::Entwurfsfehler::Umbruch) => fehler.meldung(),
                Err(_) | Ok(()) => String::new(),
            };
            grundzeile.setStringValue(&NSString::from_str(&satz));
        }));
    }
    {
        let lesen = Rc::clone(&lesen);
        let grundzeile = grundzeile.clone();
        blatt.bestaetigung_pruefen(Box::new(move || {
            match tresorweg::entwurf_pruefen(&lesen()) {
                Ok(()) => true,
                Err(fehler) => {
                    grundzeile.setStringValue(&NSString::from_str(&fehler.meldung()));
                    false
                }
            }
        }));
    }

    blatt
        .zeigen(fenster, move |bestaetigt| {
            if !bestaetigt {
                return;
            }
            // Noch einmal geprueft und nicht vorausgesetzt, wie im PIN-Blatt.
            let entwurf = lesen();
            if tresorweg::entwurf_pruefen(&entwurf).is_ok() {
                fertig(entwurf);
            }
        })
        // Das Tastenprotokoll schreibt, solange dieses Blatt steht, kein
        // getipptes Zeichen.
        .verdeckt_machen()
}

/// Eine beschriftete Eingabezeile, in die Beigabe gehaengt; das Feld ist
/// verdeckt, wenn `verdeckt` gilt.
fn eingabezeile(
    mtm: MainThreadMarker,
    beigabe: &NSView,
    beschriftung: &str,
    unterkante: f64,
    verdeckt: bool,
) -> Retained<NSTextField> {
    let text = NSTextField::labelWithString(&NSString::from_str(beschriftung), mtm);
    text.setFrame(NSRect::new(
        NSPoint::new(0.0, unterkante + 3.0),
        NSSize::new(BESCHRIFTUNG - SPALTENABSTAND, BESCHRIFTUNGSHOEHE),
    ));
    text.setAlignment(NSTextAlignment::Right);
    beigabe.addSubview(&text);

    let rahmen = NSRect::new(
        NSPoint::new(BESCHRIFTUNG, unterkante),
        NSSize::new(BREITE - BESCHRIFTUNG, ZEILENHOEHE),
    );
    let feld: Retained<NSTextField> = if verdeckt {
        Retained::into_super(NSSecureTextField::initWithFrame(
            NSSecureTextField::alloc(mtm),
            rahmen,
        ))
    } else {
        NSTextField::initWithFrame(NSTextField::alloc(mtm), rahmen)
    };
    beigabe.addSubview(&feld);
    feld
}

#[cfg(test)]
mod tests {
    use super::*;
    use krk_core::heimordner::verbindung::Zielfehler;

    use crate::tresorweg::Entwurfsfehler;

    fn felder(werte: [&str; 6]) -> Vec<String> {
        werte.iter().map(|wert| (*wert).to_owned()).collect()
    }

    /// Sechs Felder, das Passwort an fuenfter Stelle, und der Wortlaut der
    /// Beschriftungen.
    #[test]
    fn sechs_felder_mit_dem_passwort_an_fuenfter_stelle() {
        assert_eq!(
            beschriftungen(),
            [
                "Name:",
                "Protokoll:",
                "Adresse:",
                "Konto:",
                "Passwort:",
                "Notizen:"
            ]
        );
        assert_eq!(beschriftungen()[PASSWORTSTELLE], "Passwort:");
    }

    /// Die Pruefregel des Blattes (Akzeptanz (9) von Schritt 7): ein leerer
    /// Name, ein unbekanntes Protokoll, eine leere Adresse, `://` in der
    /// Adresse und ein Zeilenumbruch in einem Feld werden abgewiesen; ein
    /// vollstaendiger Entwurf geht durch, auch mit leerem Konto und Passwort.
    #[test]
    fn die_pruefregel_des_blattes() {
        let gut = ["NAS", "smb", "nas.local/daten", "", "", ""];
        assert_eq!(
            tresorweg::entwurf_pruefen(&entwurf_aus(&felder(gut))),
            Ok(())
        );
        assert_eq!(
            tresorweg::entwurf_pruefen(&entwurf_aus(&felder([
                "  ",
                "smb",
                "nas.local",
                "",
                "",
                ""
            ]))),
            Err(Entwurfsfehler::NameLeer)
        );
        assert_eq!(
            tresorweg::entwurf_pruefen(&entwurf_aus(&felder([
                "NAS",
                "gopher",
                "nas.local",
                "",
                "",
                ""
            ]))),
            Err(Entwurfsfehler::Ziel(Zielfehler::UnbekanntesProtokoll(
                "gopher".to_owned()
            )))
        );
        assert_eq!(
            tresorweg::entwurf_pruefen(&entwurf_aus(&felder(["NAS", "smb", " ", "", "", ""]))),
            Err(Entwurfsfehler::Ziel(Zielfehler::LeereAdresse))
        );
        assert_eq!(
            tresorweg::entwurf_pruefen(&entwurf_aus(&felder([
                "NAS",
                "smb",
                "smb://nas.local",
                "",
                "",
                ""
            ]))),
            Err(Entwurfsfehler::Ziel(Zielfehler::SchemaInDerAdresse))
        );
        for stelle in 0..6 {
            let mut werte = felder(gut);
            werte[stelle].push('\n');
            assert_eq!(
                tresorweg::entwurf_pruefen(&entwurf_aus(&werte)),
                Err(Entwurfsfehler::Umbruch),
                "Feld {stelle}"
            );
        }
    }

    /// Ein Entwurf kehrt in dieselben Felder zurueck, das Passwort
    /// eingeschlossen, und `Debug` zeigt das Passwort nicht.
    #[test]
    fn der_entwurf_kehrt_in_dieselben_felder_zurueck() {
        let eingabe = [
            "NAS",
            "smb",
            "nas.local/daten",
            "kai",
            "geheim-4711",
            "Büro",
        ];
        let entwurf = entwurf_aus(&felder(eingabe));
        assert_eq!(werte(&entwurf), eingabe);
        assert!(!format!("{entwurf:?}").contains("geheim-4711"));
    }

    /// Die Felder dieses Blattes bekommen den Feldeditor des Blattfensters;
    /// die Automatiken schaltet der Rueckruf des Bearbeitungsbeginns ab
    /// (Akzeptanz (8) von Schritt 7, Entscheidung 16), und das Blatt ist
    /// verdeckt (Akzeptanz (7)).
    ///
    /// **Was sie nicht sieht:** ob AppKit `controlTextDidBeginEditing:`
    /// wirklich vor der ersten Ersetzung meldet; das zeigt allein das laufende
    /// Buendel.
    #[test]
    fn das_verbindungsblatt_schaltet_die_automatiken_ab() {
        let quelle = include_str!("verbindung.rs");
        let rumpf = quelle
            .split_once("pub fn zeigen(")
            .and_then(|(_, rest)| rest.split_once("\n}\n"))
            .map(|(rumpf, _)| rumpf)
            .expect("zeigen steht im Code");
        let code: Vec<&str> = rumpf
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .collect();
        let code = code.join("\n");
        let beginn = code
            .find(concat!(".bearbeitungsbeginn_", "melden("))
            .expect("das Blatt meldet den Bearbeitungsbeginn");
        assert!(
            code[beginn..].contains(concat!(
                "textautomatik::automatiken_",
                "abschalten(feldeditor)"
            )),
            "der Rueckruf schaltet die Automatiken nicht ab"
        );
        let waechter = code
            .find(concat!(".waechter_", "anhaengen("))
            .expect("die Felder bekommen den Waechter");
        assert!(
            waechter < beginn,
            "der Rueckruf haengt am Waechter und muss nach ihm gesetzt werden"
        );
        assert!(
            code.contains(concat!(".verdeckt_", "machen()")),
            "das Blatt ist nicht verdeckt"
        );
    }
}
