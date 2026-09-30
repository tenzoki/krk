//! Das Eingabeblatt fuer einen Namen (C4, Anlegen; seit dem 260930 auch die
//! Frage nach dem Namen eines Duplikats).
//!
//! **Ein Blatt fuer beide Anlegebefehle.** `f7` und `shift+cmd+n` legen einen
//! Ordner an, `ctrl+cmd+n` eine leere Datei; beide fragen dieselbe Frage, und
//! beide pruefen den Namen mit derselben Funktion. Zwei Blaetter dafuer waeren
//! zwei Wahrheiten darueber, was ein zulaessiger Name ist. Was mit dem Namen
//! geschieht, entscheidet der Befehl und nicht dieses Blatt: es liefert einen
//! gewoehnlichen Rust-Wert zurueck.
//!
//! ```text
//!  Kommando::OrdnerAnlegen ─┐
//!                           ├──> zeigen ──> frei_zeigen ──┐
//!  Kommando::DateiAnlegen ──┘        (Result<String, Namensfehler>)
//!                                                          │
//!  Lesezeichen anlegen / umbenennen ──> frei_zeigen ───────┤
//!                                            (String)      │
//!                                                          ├──> geprueft_zeigen ──> Blatt::neu
//!  Duplizieren (Schritt 3) ──> geprueft_zeigen ────────────┘        (Option<String>)
//! ```
//!
//! # Zwei Gestalten, ein Bauer
//!
//! [`geprueft_zeigen`] ist der eine Bauer, und [`Blatt::neu`] steht in dieser
//! Datei genau einmal, in seinem Rumpf. Seine [`Vorlage`] traegt neben Frage,
//! Beschriftung und Vorgabe einen wahlfreien Grund und eine wahlfreie
//! Pruefung, und an diesen zwei Feldern entscheidet sich die Gestalt:
//!
//! - **Ohne Grund und ohne Pruefung** ist das Blatt das Feld allein unter der
//!   Frage, wie seit der Runde 1. So rufen es [`zeigen`] und [`frei_zeigen`],
//!   und das Anlegen und die zwei Lesezeichenblaetter sehen aus und wirken wie
//!   vor dem 260930.
//! - **Mit Grund oder Pruefung** steht unter dem Feld eine Zeile, nach dem
//!   Vorbild des PIN-Blattes ([`super::pin`]). Die Zeile zeigt beim Aufgehen
//!   den Grund, aus dem die Pruefung die Vorgabe abweist, sonst den
//!   uebergebenen Grund, sonst nichts; mit dem ersten Anschlag faellt der
//!   uebergebene Grund, und die Zeile zeigt allein das Urteil der Pruefung
//!   ueber den neuen Text. Die bestaetigende Schaltflaeche ist genau dann
//!   eingeschaltet, wenn die Pruefung die getrimmte Eingabe annimmt, und die
//!   Eingabetaste im Feld bestaetigt allein dann
//!   ([`super::Blatt::bestaetigung_pruefen`]); sonst schreibt sie den Grund in
//!   die Zeile und laesst das Blatt stehen.
//!
//! Was die Zeile zeigt und ob bestaetigt werden darf, rechnet [`blattstand`]
//! als reine Funktion; der Bauer ruft sie ueberall, wo er Zeile und
//! Schaltflaeche setzt, und im Abschluss noch einmal. So bleibt die Regel
//! ohne AppKit pruefbar.
//!
//! **Der Grund fuer die zweite Gestalt** ist das Duplizieren aus dem
//! Kontextmenue (Plan `260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md`,
//! Entscheidungen 6 und 11): was am Text entscheidbar ist, soll das Blatt
//! nicht schliessen, und was allein das Dateisystem weiss, ein vergebener
//! Name, oeffnet es erneut mit dem Grund unter dem Feld. Ein zweites Blatt
//! dafuer waere ein zweites Erscheinungsbild und eine zweite Tastaturbedienung
//! fuer dieselbe Frage.
//!
//! **Der Abschluss ruft `fertig` auf jedem Weg genau einmal**, mit
//! `Some(getrimmter Name)`, wenn bestaetigt wurde und die Pruefung annimmt,
//! sonst mit `None`. Das muss so sein, weil beim erneuten Aufgehen ein
//! Arbeitsfaden auf die Antwort wartet und ein Abbruch ihm gemeldet werden
//! muss. [`frei_zeigen`] uebersetzt das in den aelteren Vertrag: sein Rueckruf
//! laeuft beim Abbruch gar nicht, wie bei der Pfadeingabe der Abbruch die
//! Abwesenheit einer Eingabe ist.
//!
//! # Warum das Blatt prueft, obwohl das Anlegen es auch tut
//!
//! [`name_pruefen`] laeuft hier **und** in `operation::anlegen`; das ist keine
//! zweite Pruefung, sondern dieselbe an zwei Stellen des Weges. Der Gewinn ist
//! der Grund im Klartext: `ordner_anlegen` liefert einen [`std::io::Error`],
//! und daraus laesst sich nicht mehr ablesen, ob der Name leer war oder einen
//! Schraegstrich trug. Der Nutzer liest hier "der Name ist leer" statt
//! "ungueltige Eingabe".
//!
//! **Das Feld ist nicht als eigene Textflaeche angemeldet**, wie das Feld
//! jedes Blattes: `Esc` schliesst das Blatt nur, weil der Ersthelfer AppKit
//! gehoert (Modulkopf von [`super`], Abschnitt „Warum das Textfeld eines
//! Blattes nicht als eigene Textflaeche angemeldet wird“).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField` (ueber `NSControl`, `NSView` und `NSResponder`), `NSView`,
//! `NSWindow` und `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `alloc`, `initWithFrame:`, `setStringValue:`, `stringValue`, `selectText:`,
//! `setFrame:`, `addSubview:` und `setEnabled:`. `NSPoint`, `NSRect` und
//! `NSSize` sind blosse Strukturen und tragen keine Verfuegbarkeitsangabe;
//! `MainThreadMarker` gehoert `objc2` und nicht AppKit, `Rc` der
//! Standardbibliothek.
//!
//! **Eine einzige Beruehrung ist juenger als 10.0**, und sie liegt unter dem
//! Zielsystem: `labelWithString:` steht seit 10.12 (`NSTextField.h`) und baut
//! die Zeile unter dem Feld. Alles Weitere, das Aufgehen am Fenster, der
//! Eingabewaechter, die Schaltflaechen, geht durch [`super::Blatt`]; die
//! Untergrenzen dazu nennt der Modulkopf von [`super`] und nicht dieser.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme. Was `NSAlert` betrifft,
//! steht im Kopf von [`Blatt`].

use std::rc::Rc;

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextField, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use krk_core::operation::{Namensfehler, name_pruefen};

use super::{Blatt, Blattgriff};

/// Die Breite des Eingabefeldes in Punkten.
///
/// Schmaler als die der Pfadeingabe: ein Name ist kein Pfad, und ein Feld, das
/// dreimal so breit ist wie sein laengster erwarteter Inhalt, sieht aus wie ein
/// Fehler. In der Gestalt mit der Zeile ist es zugleich die Breite der Beigabe.
const FELDBREITE: f64 = 280.0;

/// Die Hoehe einer Zeile im Eingabefeld in Punkten.
const FELDHOEHE: f64 = 24.0;

/// Die Hoehe der Zeile mit dem Grund unter dem Feld, wie im PIN-Blatt.
const GRUNDHOEHE: f64 = 17.0;

/// Der senkrechte Abstand zwischen Feld und Grund, wie im PIN-Blatt.
const ZEILENABSTAND: f64 = 6.0;

/// Eine Pruefung der getrimmten Eingabe: `None` heisst angenommen, `Some`
/// nennt den Grund, aus dem sie nicht bestaetigt werden darf.
///
/// Ein Funktionszeiger und kein Abschluss, damit die [`Vorlage`] `Copy`
/// bleibt und die Pruefung in jeden Rueckruf des Blattes ohne Zaehlung
/// mitreist.
pub type Namenspruefung = fn(&str) -> Option<&'static str>;

/// Der eine Weg im Bauer, auf dem Zeile und Schaltflaeche gesetzt werden:
/// nimmt den Grund, der noch gilt, und sagt, ob bestaetigt werden darf.
///
/// Ein `Rc`, weil ihn drei Rufer teilen (das Aufgehen, jede Textaenderung
/// und die Eingabetaste im Feld) und der [`super::Eingabewaechter`] seine
/// Rueckrufe als `Box` und nicht als Verweis nimmt.
type Standanwender = Rc<dyn Fn(Option<&str>) -> bool>;

/// Was ein Namensblatt beim Aufgehen zeigt und wie es prueft.
///
/// Frage, Beschriftung der bestaetigenden Schaltflaeche und Vorgabe traegt
/// jedes Namensblatt; `grund` und `pruefen` sind wahlfrei und entscheiden die
/// Gestalt (Modulkopf, „Zwei Gestalten, ein Bauer“). Die abbrechende
/// Schaltflaeche heisst in jedem Blatt „Abbrechen“ und steht nicht hier; sie
/// kommt aus `standardschaltflaechen` in [`super`].
#[derive(Debug, Clone, Copy)]
pub struct Vorlage<'a> {
    /// Die Frage ueber dem Feld.
    pub frage: &'a str,
    /// Die Beschriftung der bestaetigenden Schaltflaeche.
    pub bestaetigen: &'a str,
    /// Was beim Aufgehen ausgewaehlt im Feld steht; leer heisst leer.
    pub vorgabe: &'a str,
    /// Ein Grund, der beim Aufgehen unter dem Feld steht und mit dem ersten
    /// Anschlag faellt; beim Duplizieren der vergebene Name.
    pub grund: Option<&'a str>,
    /// Die Pruefung der getrimmten Eingabe; ohne sie darf jede Eingabe
    /// bestaetigt werden.
    pub pruefen: Option<Namenspruefung>,
}

/// Der Stand des Blattes zu einer Eingabe: ob bestaetigt werden darf, und was
/// die Zeile unter dem Feld zeigt.
///
/// Die eine Regel hinter Schaltflaeche, Eingabetaste, Zeile und Abschluss,
/// als reine Funktion ohne AppKit. Geprueft wird die **getrimmte** Eingabe,
/// wie jedes Namensblatt sie liefert.
///
/// # Die Tafel
///
/// | `pruefen` | `pruefen(getrimmt)` | `grund` | Ergebnis |
/// |---|---|---|---|
/// | `None` | — | `None` | `(true, "")` |
/// | `None` | — | `Some(g)` | `(true, g)` |
/// | `Some` | `None` (angenommen) | `None` | `(true, "")` |
/// | `Some` | `None` (angenommen) | `Some(g)` | `(true, g)` |
/// | `Some` | `Some(s)` (abgewiesen) | beliebig | `(false, s)` |
///
/// Der Grund macht die Eingabe nie unbestaetigbar: beim Duplizieren ist er
/// „es gibt schon einen Eintrag namens …“, und ob der Name inzwischen frei
/// ist, weiss allein das Dateisystem beim naechsten Versuch. Das Urteil der
/// Pruefung geht ihm vor, weil es die Eingabe betrifft, die jetzt im Feld
/// steht, und der Grund die von vorhin.
#[must_use]
pub fn blattstand(
    eingabe: &str,
    grund: Option<&str>,
    pruefen: Option<Namenspruefung>,
) -> (bool, String) {
    let getrimmt = eingabe.trim();
    match pruefen.and_then(|pruefen| pruefen(getrimmt)) {
        Some(satz) => (false, satz.to_owned()),
        None => (true, grund.unwrap_or("").to_owned()),
    }
}

/// Zeigt die Namenseingabe am Fenster und liefert den geprueften Namen.
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden, wenn der Nutzer
/// bestaetigt hat; bricht er ab, laeuft es gar nicht. Der Name kommt getrimmt
/// an: fuehrende und schliessende Leerzeichen sind so gut wie immer ein
/// Versehen, und ein Ordner, den man von seinem Nachbarn nicht unterscheiden
/// kann, ist keine Hilfe.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    bestaetigen: &str,
    fertig: impl Fn(Result<String, Namensfehler>) + 'static,
) -> Blattgriff {
    frei_zeigen(mtm, fenster, frage, bestaetigen, "", move |name| {
        fertig(name_pruefen(&name).map(|()| name))
    })
}

/// Zeigt dieselbe Namenseingabe, ohne den Namen gegen das Dateisystem zu
/// pruefen.
///
/// Der Weg der Lesezeichen aus C5. Ein Lesezeichenname ist eine Beschriftung
/// und kein Eintrag im Dateisystem: "Projekte/2026" ist ein zulaessiger Name
/// dafuer, und [`name_pruefen`] wiese ihn ab. Welche Regel gilt, entscheidet
/// deshalb der Aufrufer; fuer das Lesezeichen ist es
/// `krk_core::ablage::lesezeichen::name_pruefen`.
///
/// **Ein Blatt und kein zweites.** [`zeigen`] laeuft ueber dieselbe Funktion
/// und legt seine Pruefung darum; diese Funktion wiederum ruft
/// [`geprueft_zeigen`] ohne Grund und ohne Pruefung und laesst den Abbruch
/// fallen, den jener meldet. Zwei Eingabeblaetter fuer einen Namen waeren
/// zwei Erscheinungsbilder und zwei Tastaturbedienungen fuer dieselbe Frage.
///
/// `vorgabe` steht beim Aufgehen im Feld und ist ausgewaehlt: beim Umbenennen
/// ist es der alte Name, beim Anlegen der Name des Ordners. Wer sie behalten
/// will, bestaetigt; wer nicht, tippt darueber.
pub fn frei_zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    bestaetigen: &str,
    vorgabe: &str,
    fertig: impl Fn(String) + 'static,
) -> Blattgriff {
    let vorlage = Vorlage {
        frage,
        bestaetigen,
        vorgabe,
        grund: None,
        pruefen: None,
    };
    geprueft_zeigen(mtm, fenster, vorlage, move |name| {
        // Der Abbruch ist hier die Abwesenheit einer Eingabe: der Rueckruf
        // laeuft dann gar nicht, wie seit der Runde 1.
        if let Some(name) = name {
            fertig(name);
        }
    })
}

/// Der eine Bauer jedes Namensblatts; welche Gestalt entsteht, sagt die
/// [`Vorlage`] (Modulkopf, „Zwei Gestalten, ein Bauer“).
///
/// Kehrt sofort zurueck. `fertig` laeuft auf dem Hauptfaden **auf jedem Weg
/// genau einmal**: mit `Some(getrimmter Name)`, wenn der Nutzer bestaetigt hat
/// und die Pruefung die Eingabe annimmt, sonst mit `None`. Eine Antwort von
/// AppKit, die zu keiner Schaltflaeche gehoert, faellt damit auf `None`, und
/// ein Arbeitsfaden, der auf die Antwort wartet, wartet nie vergebens.
///
/// Die Vorgabe steht beim Aufgehen ausgewaehlt im Feld, in beiden Gestalten.
pub fn geprueft_zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    vorlage: Vorlage<'_>,
    fertig: impl Fn(Option<String>) + 'static,
) -> Blattgriff {
    let feld = feld_bauen(mtm, vorlage.vorgabe);
    let pruefen = vorlage.pruefen;

    let mut blatt = Blatt::neu(mtm, vorlage.frage, vorlage.bestaetigen);
    if vorlage.grund.is_none() && pruefen.is_none() {
        // Die Gestalt ohne Zeile: das Feld allein als Beigabe, Ersthelfer und
        // bewachtes Feld in einem Zug, wie seit der Runde 1.
        blatt.textfeld_setzen(mtm, &feld);
    } else {
        let (beigabe, zeile) = beigabe_mit_zeile(mtm, &feld);
        blatt.beigabe_setzen(&beigabe);
        blatt.ersthelfer_setzen(&feld);
        blatt.waechter_anhaengen(mtm, &feld);

        // Der eine Weg, auf dem Zeile und Schaltflaeche gesetzt werden: er
        // fragt `blattstand` und sagt, ob bestaetigt werden darf. Drei Rufer,
        // das Aufgehen, jede Textaenderung und die Eingabetaste im Feld.
        let anwenden: Standanwender = {
            let feld = feld.clone();
            let knopf = blatt.bestaetigende_schaltflaeche();
            Rc::new(move |grund| {
                let (darf, satz) = blattstand(&feld.stringValue().to_string(), grund, pruefen);
                if let Some(knopf) = &knopf {
                    knopf.setEnabled(darf);
                }
                zeile.setStringValue(&NSString::from_str(&satz));
                darf
            })
        };
        // Beim Aufgehen zaehlt der uebergebene Grund; mit dem ersten Anschlag
        // faellt er, und die Zeile zeigt allein das Urteil der Pruefung.
        anwenden(vorlage.grund);
        {
            let anwenden = Rc::clone(&anwenden);
            blatt.textaenderung_melden(Box::new(move || {
                anwenden(None);
            }));
        }
        blatt.bestaetigung_pruefen(Box::new(move || anwenden(None)));
    }

    blatt.zeigen(fenster, move |bestaetigt| {
        // Noch einmal gerechnet und nicht vorausgesetzt: Schaltflaeche und
        // Taste haben schon geprueft, aber eine Antwort von AppKit, die zu
        // keiner Schaltflaeche gehoert, kommt ohne sie hierher.
        let eingabe = feld.stringValue().to_string();
        let (darf, _) = blattstand(&eingabe, None, pruefen);
        let name = (bestaetigt && darf).then(|| eingabe.trim().to_owned());
        fertig(name);
    })
}

/// Das Eingabefeld mit der Vorgabe, ausgewaehlt, noch an keiner Ansicht.
fn feld_bauen(mtm: MainThreadMarker, vorgabe: &str) -> Retained<NSTextField> {
    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );
    if !vorgabe.is_empty() {
        feld.setStringValue(&NSString::from_str(vorgabe));
        // Die Vorgabe steht ausgewaehlt da: wer sie behalten will, bestaetigt,
        // wer nicht, tippt darueber. Ohne diese Zeile haengt der neue Name an
        // den alten an. Dieselbe Zeile aus demselben Grund tragen
        // `super::pfadeingabe` und `super::suche`; dass AppKit den Inhalt von
        // sich aus auswaehlt, sobald `setInitialFirstResponder:` den Rang
        // vergibt, ist an keinem Buendel gemessen
        // (`260826-1334_*_frei-zeigen-sagt-die-vorgabe-stehe-ausgewaehlt-im-feld-und-ruft-selecttext-nicht.md`).
        // SAFETY: `selectText:` ist eine gewoehnliche Aktion von `NSControl`;
        // sie stellt keine Bedingung an ihren Absender, und `None` ist der
        // Wert, den ein programmatischer Aufruf dafuer setzt.
        unsafe { feld.selectText(None) };
    }
    feld
}

/// Die Beigabe der Gestalt mit Zeile: das Feld oben, darunter die Zeile fuer
/// den Grund, von unten nach oben gebaut, weil AppKit von unten nach oben
/// misst; derselbe Zuschnitt wie im PIN-Blatt.
///
/// Liefert die Beigabe und die Zeile; das Feld haengt danach in der Beigabe.
fn beigabe_mit_zeile(
    mtm: MainThreadMarker,
    feld: &NSTextField,
) -> (Retained<NSView>, Retained<NSTextField>) {
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(
            NSPoint::ZERO,
            NSSize::new(FELDBREITE, FELDHOEHE + ZEILENABSTAND + GRUNDHOEHE),
        ),
    );
    let zeile = NSTextField::labelWithString(&NSString::from_str(""), mtm);
    zeile.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(FELDBREITE, GRUNDHOEHE),
    ));
    beigabe.addSubview(&zeile);
    feld.setFrame(NSRect::new(
        NSPoint::new(0.0, GRUNDHOEHE + ZEILENABSTAND),
        NSSize::new(FELDBREITE, FELDHOEHE),
    ));
    beigabe.addSubview(feld);
    (beigabe, zeile)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Pruefung, die die Tafel mitbringt: ein leerer Name und ein
    /// Schraegstrich werden abgewiesen, alles andere angenommen.
    fn probepruefung(name: &str) -> Option<&'static str> {
        if name.is_empty() {
            Some("der Name ist leer")
        } else if name.contains('/') {
            Some("der Name enthält einen Schrägstrich")
        } else {
            None
        }
    }

    /// Die Tafel von [`blattstand`], Zeile fuer Zeile, ohne AppKit.
    #[test]
    fn die_tafel_des_blattstands() {
        // Ohne Pruefung darf jede Eingabe bestaetigt werden, auch die leere.
        assert_eq!(blattstand("", None, None), (true, String::new()));
        assert_eq!(blattstand("  a/b  ", None, None), (true, String::new()));
        assert_eq!(
            blattstand("", Some("vergeben"), None),
            (true, "vergeben".to_owned()),
            "ohne Pruefung zeigt die Zeile den uebergebenen Grund"
        );

        // Mit annehmender Pruefung: bestaetigen erlaubt, Zeile leer oder Grund.
        assert_eq!(
            blattstand("Bericht.txt", None, Some(probepruefung)),
            (true, String::new())
        );
        assert_eq!(
            blattstand("Bericht.txt", Some("vergeben"), Some(probepruefung)),
            (true, "vergeben".to_owned()),
            "ein Grund macht eine angenommene Eingabe nicht unbestaetigbar"
        );

        // Mit abweisender Pruefung: bestaetigen verboten, Zeile zeigt ihr Urteil,
        // und der uebergebene Grund tritt dahinter zurueck.
        assert_eq!(
            blattstand("", None, Some(probepruefung)),
            (false, "der Name ist leer".to_owned())
        );
        assert_eq!(
            blattstand("a/b", Some("vergeben"), Some(probepruefung)),
            (false, "der Name enthält einen Schrägstrich".to_owned()),
            "das Urteil der Pruefung geht dem Grund vor"
        );
    }

    /// Geprueft wird die getrimmte Eingabe: Leerraum allein ist ein leerer
    /// Name, und Leerraum um einen Namen aendert das Urteil nicht.
    #[test]
    fn der_blattstand_prueft_die_getrimmte_eingabe() {
        assert_eq!(
            blattstand("   ", None, Some(probepruefung)),
            (false, "der Name ist leer".to_owned())
        );
        assert_eq!(
            blattstand("  Bericht.txt  ", None, Some(probepruefung)),
            (true, String::new())
        );
    }

    /// Der Grund faellt mit der ersten Textaenderung: der Bauer ruft
    /// [`blattstand`] beim Aufgehen mit dem uebergebenen Grund und danach
    /// mit `None`, und die Tafel gibt fuer dieselbe Eingabe dann eine leere
    /// Zeile.
    #[test]
    fn der_grund_faellt_mit_der_textaenderung() {
        let vorgabe = "Bericht.txt";
        let (darf_vorher, zeile_vorher) =
            blattstand(vorgabe, Some("vergeben"), Some(probepruefung));
        let (darf_nachher, zeile_nachher) = blattstand(vorgabe, None, Some(probepruefung));
        assert!(darf_vorher && darf_nachher);
        assert_eq!(zeile_vorher, "vergeben");
        assert_eq!(zeile_nachher, "");
    }
}
