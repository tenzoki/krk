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
//!                           ├──> zeigen ───────────────────┐
//!  Kommando::DateiAnlegen ──┘   (Result<String, Namensfehler>)
//!                                                          │
//!  Lesezeichen anlegen / umbenennen ──> frei_zeigen ───────┤
//!                                            (String)      │
//!                                                          ├──> geprueft_zeigen ──> blatt_bauen ──> Blatt::neu
//!  Duplizieren (Schritt 3) ──> geprueft_zeigen ────────────┘        (Option<String>)
//! ```
//!
//! # Zwei Gestalten, ein Bauer
//!
//! [`geprueft_zeigen`] ist der eine Bauer, und [`Blatt::neu`] steht in dieser
//! Datei genau einmal, im Rumpf von `blatt_bauen`, das er als Erstes ruft und
//! das allein dafuer abgetrennt ist, dass eine Probe das Blatt bauen kann,
//! ohne es an einem Fenster zu zeigen. Seine [`Vorlage`] traegt neben Frage,
//! Beschriftung und Vorgabe einen wahlfreien Grund und eine wahlfreie
//! Pruefung, und an diesen zwei Feldern entscheidet sich die Gestalt:
//!
//! - **Ohne Grund und ohne Pruefung** ist das Blatt das Feld allein unter der
//!   Frage, wie seit der Runde 1. So rufen es [`frei_zeigen`] und [`zeigen`]
//!   ohne Hinweis, und das Anlegen ausserhalb von `minutes/` und die zwei
//!   Lesezeichenblaetter sehen aus und wirken wie vor dem 260930.
//! - **Mit Grund oder Pruefung** steht unter dem Feld eine Zeile, nach dem
//!   Vorbild des PIN-Blattes ([`super::pin`]). Die Zeile zeigt beim Aufgehen
//!   den Grund, aus dem die Pruefung die Vorgabe abweist, sonst den
//!   uebergebenen Grund, sonst nichts; mit dem ersten Anschlag faellt der
//!   uebergebene Grund, und die Zeile zeigt allein das Urteil der Pruefung
//!   ueber den neuen Text. **Ein Grund mit [`Vorlage::grund_bleibt`] faellt
//!   nicht**: er sagt nichts ueber die Eingabe von vorhin, sondern etwas, das
//!   fuer jede Eingabe gilt, und steht, solange der Nutzer tippt. So traegt
//!   das Anlegen in `minutes/` seinen Hinweis ([`zeigen`]). Die bestaetigende
//!   Schaltflaeche ist genau dann
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
//! `setFrame:`, `frame`, `sizeToFit`, `addSubview:` und `setEnabled:`. `NSPoint`, `NSRect` und
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
    /// Ob der Grund ueber den ersten Anschlag hinaus stehen bleibt. `false`
    /// fuer einen Grund, der die Eingabe von vorhin betrifft (der vergebene
    /// Name beim Duplizieren); `true` fuer einen Hinweis, der fuer jede
    /// Eingabe gilt (C4.10 des Spec
    /// `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`).
    /// Ein bleibender Grund bekommt ausserdem die Breite, die sein Text
    /// braucht, weil er laenger ist als das Feld und sonst abgeschnitten
    /// dastuende. Ohne Grund ohne Wirkung.
    pub grund_bleibt: bool,
    /// Die Pruefung der getrimmten Eingabe; ohne sie darf jede Eingabe
    /// bestaetigt werden.
    pub pruefen: Option<Namenspruefung>,
}

impl<'a> Vorlage<'a> {
    /// Der Grund, der nach einer Textaenderung noch gilt: der bleibende, sonst
    /// keiner. Die eine Stelle der Regel, gefragt bei jeder Textaenderung und
    /// von der Eingabetaste im Feld.
    #[must_use]
    pub fn grund_nach_anschlag(&self) -> Option<&'a str> {
        self.grund.filter(|_| self.grund_bleibt)
    }
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
///
/// `hinweis` ist die Zeile unter dem Feld, die steht, solange der Nutzer
/// tippt ([`anlegevorlage`]); ohne ihn ist das Blatt das Feld allein. Gesetzt
/// ist er, wenn der entstehende Name vom getippten abweicht, heute allein fuer
/// eine Datei in `minutes/` des Notizordners. Geprueft wird der Name in beiden
/// Gestalten erst nach dem Bestaetigen, mit [`name_pruefen`], und der Grund
/// einer Abweisung geht an `fertig`.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    bestaetigen: &str,
    hinweis: Option<&str>,
    fertig: impl Fn(Result<String, Namensfehler>) + 'static,
) -> Blattgriff {
    let vorlage = anlegevorlage(frage, bestaetigen, hinweis);
    geprueft_zeigen(mtm, fenster, vorlage, move |name| {
        // Der Abbruch ist die Abwesenheit einer Eingabe, wie in `frei_zeigen`.
        if let Some(name) = name {
            fertig(name_pruefen(&name).map(|()| name));
        }
    })
}

/// Die Vorlage des Anlegeblatts: leeres Feld, keine Pruefung im Blatt, und
/// der Hinweis als Grund, der bleibt.
///
/// Eine eigene Funktion, damit die Probe zu C4.10 dieselbe Vorlage baut wie
/// [`zeigen`] und nicht eine, die ihr nur gleicht.
#[must_use]
fn anlegevorlage<'a>(
    frage: &'a str,
    bestaetigen: &'a str,
    hinweis: Option<&'a str>,
) -> Vorlage<'a> {
    Vorlage {
        frage,
        bestaetigen,
        vorgabe: "",
        grund: hinweis,
        grund_bleibt: hinweis.is_some(),
        pruefen: None,
    }
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
/// **Ein Blatt und kein zweites.** [`zeigen`] und diese Funktion rufen beide
/// [`geprueft_zeigen`]: diese ohne Grund und ohne Pruefung, [`zeigen`] mit
/// seinem wahlfreien Hinweis und der Namenspruefung nach dem Bestaetigen.
/// Beide lassen den Abbruch fallen, den jener meldet. Zwei Eingabeblaetter fuer einen Namen waeren
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
        grund_bleibt: false,
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
    let Gebaut { blatt, feld, .. } = blatt_bauen(mtm, vorlage);
    let pruefen = vorlage.pruefen;

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

/// Was [`blatt_bauen`] liefert: das Blatt, sein Feld und, in der Gestalt mit
/// Zeile, die Zeile unter dem Feld.
struct Gebaut {
    /// Das Blatt, noch an keinem Fenster.
    blatt: Blatt,
    /// Das Eingabefeld; der Abschluss liest daraus den Namen.
    feld: Retained<NSTextField>,
    /// Die Zeile unter dem Feld, `None` in der Gestalt ohne Zeile. Gelesen
    /// allein von den Proben; das Blatt haelt sie ueber seine Beigabe.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "allein die Proben lesen die Zeile")
    )]
    zeile: Option<Retained<NSTextField>>,
}

/// Baut das Blatt einer [`Vorlage`], ohne es zu zeigen: Feld, Zeile,
/// Schaltflaechen und die Rueckrufe des Waechters.
///
/// Abgetrennt von [`geprueft_zeigen`], seinem einen Rufer im Betriebscode,
/// damit eine Probe das Blatt bauen und seine Zeile lesen kann; ein Blatt an
/// einem Fenster zu zeigen braucht eine Ereignisschleife, die keine Probe hat.
fn blatt_bauen(mtm: MainThreadMarker, vorlage: Vorlage<'_>) -> Gebaut {
    let feld = feld_bauen(mtm, vorlage.vorgabe);
    let pruefen = vorlage.pruefen;

    let mut blatt = Blatt::neu(mtm, vorlage.frage, vorlage.bestaetigen);
    if vorlage.grund.is_none() && pruefen.is_none() {
        // Die Gestalt ohne Zeile: das Feld allein als Beigabe, Ersthelfer und
        // bewachtes Feld in einem Zug, wie seit der Runde 1.
        blatt.textfeld_setzen(mtm, &feld);
        return Gebaut {
            blatt,
            feld,
            zeile: None,
        };
    }

    // Was nach dem ersten Anschlag noch unter dem Feld steht: der bleibende
    // Grund, sonst nichts. Eine eigene Abschrift, weil die Rueckrufe das
    // Blatt ueberleben muessen und die Vorlage nur geliehen ist.
    let bleibend = vorlage.grund_nach_anschlag().map(str::to_owned);
    let (beigabe, zeile) = beigabe_mit_zeile(mtm, &feld, bleibend.as_deref());
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&feld);
    blatt.waechter_anhaengen(mtm, &feld);

    // Der eine Weg, auf dem Zeile und Schaltflaeche gesetzt werden: er
    // fragt `blattstand` und sagt, ob bestaetigt werden darf. Drei Rufer,
    // das Aufgehen, jede Textaenderung und die Eingabetaste im Feld.
    let anwenden: Standanwender = {
        let feld = feld.clone();
        let zeile = zeile.clone();
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
    // faellt er, wenn er nicht bleibt, und die Zeile zeigt dann allein das
    // Urteil der Pruefung.
    anwenden(vorlage.grund);
    {
        let anwenden = Rc::clone(&anwenden);
        let bleibend = bleibend.clone();
        blatt.textaenderung_melden(Box::new(move || {
            anwenden(bleibend.as_deref());
        }));
    }
    blatt.bestaetigung_pruefen(Box::new(move || anwenden(bleibend.as_deref())));

    Gebaut {
        blatt,
        feld,
        zeile: Some(zeile),
    }
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
///
/// **Ein bleibender Grund bestimmt die Breite mit.** Er steht, solange das
/// Blatt steht, und ist laenger als das Feld; eine Zeile in dessen Breite
/// schnitte ihn ab. Die Beigabe ist deshalb so breit wie sein Text, gemessen
/// mit `sizeToFit` an der Zeile selbst, mindestens aber [`FELDBREITE`], und
/// das Feld reicht ueber dieselbe Breite. Ohne bleibenden Grund bleibt es bei
/// [`FELDBREITE`], wie beim Duplizieren seit dem 260930: dessen Grund nennt
/// einen Namen beliebiger Laenge und darf das Blatt nicht aufziehen.
fn beigabe_mit_zeile(
    mtm: MainThreadMarker,
    feld: &NSTextField,
    bleibend: Option<&str>,
) -> (Retained<NSView>, Retained<NSTextField>) {
    let zeile = NSTextField::labelWithString(&NSString::from_str(bleibend.unwrap_or("")), mtm);
    let breite = if bleibend.is_some() {
        zeile.sizeToFit();
        zeile.frame().size.width.ceil().max(FELDBREITE)
    } else {
        FELDBREITE
    };
    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(
            NSPoint::ZERO,
            NSSize::new(breite, FELDHOEHE + ZEILENABSTAND + GRUNDHOEHE),
        ),
    );
    zeile.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(breite, GRUNDHOEHE)));
    beigabe.addSubview(&zeile);
    feld.setFrame(NSRect::new(
        NSPoint::new(0.0, GRUNDHOEHE + ZEILENABSTAND),
        NSSize::new(breite, FELDHOEHE),
    ));
    beigabe.addSubview(feld);
    (beigabe, zeile)
}

#[cfg(test)]
pub(in crate::appkit) use tests::HAUPTFADENPROBEN;

#[cfg(test)]
mod tests {
    use krk_core::heimordner::Heimordner;
    use krk_core::heimordner::ort::Notizort;
    use krk_core::ort::Ort;
    use krk_core::sprache::{Sprache, Text};

    use crate::kommandos::operationen::{Anlegeart, anlegehinweis, im_protokollordner};

    use super::super::super::hauptfaden::{Probe, hauptfadenprobe};
    use super::*;

    /// Die Proben dieses Moduls, die ein Blatt bauen und deshalb im Pruefziel
    /// `hauptfaden` auf dem Hauptfaden laufen und nicht unter `libtest`.
    ///
    /// Eine Funktion `fn(MainThreadMarker)` dieses Moduls, die hier fehlt, ist
    /// im Bau von `libtest` toter Code und macht `make lint` rot.
    pub(crate) const HAUPTFADENPROBEN: &[Probe] = &[hauptfadenprobe!(
        das_anlegeblatt_traegt_den_hinweis_allein_fuer_eine_datei_in_minutes
    )];

    /// Die Zeile, die das Anlegeblatt fuer diese Art an diesem Ort traegt,
    /// oder `None`, wenn es das Feld allein ist: gebaut auf dem Weg von
    /// [`zeigen`], aus [`anlegehinweis`] ueber [`anlegevorlage`] und
    /// [`blatt_bauen`], nur ohne Fenster.
    fn zeile_des_anlegeblatts(
        mtm: MainThreadMarker,
        notizort: &Notizort,
        art: Anlegeart,
        ort: &Ort,
    ) -> Option<String> {
        let hinweis = anlegehinweis(art, im_protokollordner(notizort, ort));
        let gebaut = blatt_bauen(mtm, anlegevorlage(art.frage(), art.bestaetigen(), hinweis));
        gebaut.zeile.map(|zeile| zeile.stringValue().to_string())
    }

    /// C4.10: das Blatt von „Leere Datei anlegen“ traegt in `minutes/` des
    /// Notizordners die Zeile mit dem Hinweis, und in jedem anderen Ordner und
    /// fuer „Ordner anlegen“ ist es das Feld allein wie vorher.
    ///
    /// Dazu, was die Zeile ausmacht: sie steht nach einer Textaenderung noch
    /// da (die Regel dafuer, [`Vorlage::grund_nach_anschlag`], ist die, die
    /// der Rueckruf des Waechters fragt), sie macht keine Eingabe
    /// unbestaetigbar, und sie ist in jeder der drei Sprachen so breit wie ihr
    /// Text.
    ///
    /// **Was sie nicht sieht:** das Blatt am Fenster. Ob AppKit die Zeile dort
    /// so zeichnet, wie ihr Rahmen es sagt, und ob der Waechter die
    /// Textaenderung meldet, zeigt allein das laufende Buendel.
    fn das_anlegeblatt_traegt_den_hinweis_allein_fuer_eine_datei_in_minutes(mtm: MainThreadMarker) {
        let heim = Heimordner::ohne_aufloesung(
            std::path::PathBuf::from("/pruefpfad/notizen"),
            Some(std::path::Path::new("/pruefpfad")),
        );
        let minutes = heim.protokollordner();
        let notizort: Notizort = Ok(heim.clone());

        assert_eq!(
            zeile_des_anlegeblatts(
                mtm,
                &notizort,
                Anlegeart::Datei,
                &Ort::Lokal(minutes.clone())
            ),
            Some("Datum, Uhrzeit und .md werden ergänzt: JJMMTT-HHMM-<Name>.md".to_owned()),
            "eine Datei in minutes/ bekommt den Hinweis nicht"
        );
        for (art, ordner) in [
            (Anlegeart::Ordner, minutes.clone()),
            (Anlegeart::Datei, heim.geschrieben().to_path_buf()),
            (Anlegeart::Datei, minutes.join("2026")),
            (
                Anlegeart::Datei,
                std::path::PathBuf::from("/pruefpfad/minutes"),
            ),
            (Anlegeart::Ordner, std::path::PathBuf::from("/pruefpfad")),
        ] {
            assert_eq!(
                zeile_des_anlegeblatts(mtm, &notizort, art, &Ort::Lokal(ordner.clone())),
                None,
                "{art:?} in {} traegt eine Zeile unter dem Feld",
                ordner.display()
            );
        }

        // Die Zeile bleibt ueber den ersten Anschlag hinaus und haelt keine
        // Eingabe an.
        let hinweis = Sprache::De.text(Text::ProtokollnameHinweis);
        let vorlage = anlegevorlage("Frage", "Anlegen", Some(hinweis));
        assert_eq!(vorlage.grund_nach_anschlag(), Some(hinweis));
        assert_eq!(
            blattstand(
                "besprechung",
                vorlage.grund_nach_anschlag(),
                vorlage.pruefen
            ),
            (true, hinweis.to_owned())
        );
        assert_eq!(
            blattstand("", vorlage.grund_nach_anschlag(), vorlage.pruefen),
            (true, hinweis.to_owned()),
            "der Hinweis macht eine Eingabe unbestaetigbar"
        );

        // In jeder Sprache ist die Zeile so breit wie ihr Text, und das Feld
        // reicht ueber dieselbe Breite.
        for sprache in [Sprache::De, Sprache::En, Sprache::Fr] {
            let hinweis = sprache.text(Text::ProtokollnameHinweis);
            let gebaut = blatt_bauen(mtm, anlegevorlage("Frage", "Anlegen", Some(hinweis)));
            let zeile = gebaut.zeile.expect("mit Hinweis steht die Zeile");
            assert_eq!(zeile.stringValue().to_string(), hinweis);
            let breite = zeile.frame().size.width;
            let zeuge = NSTextField::labelWithString(&NSString::from_str(hinweis), mtm);
            zeuge.sizeToFit();
            assert!(
                breite >= zeuge.frame().size.width && breite >= FELDBREITE,
                "{sprache:?}: die Zeile ist {breite} breit und schneidet ihren Text ab"
            );
            assert_eq!(gebaut.feld.frame().size.width, breite);
        }
    }

    /// Ein Grund, der nicht bleibt, faellt mit dem ersten Anschlag wie seit
    /// dem 260930, und ohne Grund aendert das Kennzeichen nichts.
    #[test]
    fn allein_ein_bleibender_grund_uebersteht_den_anschlag() {
        let vorlage = |grund, grund_bleibt| Vorlage {
            frage: "Frage",
            bestaetigen: "Los",
            vorgabe: "",
            grund,
            grund_bleibt,
            pruefen: None,
        };
        assert_eq!(vorlage(Some("vergeben"), false).grund_nach_anschlag(), None);
        assert_eq!(
            vorlage(Some("Hinweis"), true).grund_nach_anschlag(),
            Some("Hinweis")
        );
        assert_eq!(vorlage(None, true).grund_nach_anschlag(), None);
        assert_eq!(vorlage(None, false).grund_nach_anschlag(), None);
    }

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
