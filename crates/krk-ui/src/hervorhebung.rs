//! Was die Formatansicht aus C3 ueber einen Text weiss: welche Stelle welche
//! Farbe traegt, welche unterstrichen ist und welche eine Markdown-Auszeichnung
//! ist.
//!
//! **Keine Zeile AppKit.** Wie [`crate::editormodell`] daneben steht hier keine
//! `use`-Zeile aus einer Objective-C-Bindungskiste. Diese Datei rechnet, und
//! `crate::appkit::editor` setzt das Ergebnis in Merkmale um; die Abnahme von
//! S16 misst die Grenze, indem sie den Kistennamen zaehlt.
//!
//! Die Syntaxkisten wohnen in `krk-ui` und nicht im Kern, weil sie Farben
//! liefern und Farben in KRK zur Oberflaeche gehoeren; die Begruendung im
//! Einzelnen steht an der Versionsangabe in der Wurzel-`Cargo.toml`.
//!
//! # Zwei Listen, und warum es zwei sein muessen
//!
//! ```text
//!   Text + Pfad + Dateityp + Tafel
//!            │
//!            v  ein Durchgang durch die Kiste
//!   ┌──────────────────────────────────────────────────┐
//!   │ einfaerbungen  ──> voruebergehende Merkmale      │  Layoutverwalter
//!   │                    (Farbe, Unterstreichung)      │
//!   │ auszeichnungen ──> Merkmale des Textspeichers    │  NSTextStorage
//!   │                    (Schriftgroesse, Schnitt,     │
//!   │                     feste Schrift, Einzug)       │
//!   └──────────────────────────────────────────────────┘
//! ```
//!
//! Der Plan (`### Frage 7`) und der Datensatz
//! `decisions/260808-0140_*_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md`
//! nennen beide **eine** Mechanik, die voruebergehenden Merkmale des
//! Layoutverwalters, und zaehlen darunter auch "Ueberschriften groesser und
//! fett, Listen eingerueckt, Quelltextbloecke in fester Schrift" auf. Das geht
//! so nicht, und der Grund steht nicht in einem Bericht, sondern im Kopf des
//! Systems selbst:
//!
//! > Temporary attributes provide a way to override attributes for drawing on a
//! > per-layout manager basis, without affecting the underlying stored text.
//! > Clients may set any attributes they wish, but **the only attributes that
//! > the layout manager will recognize for drawing are those that do not affect
//! > layout** (color, underline, etc.).
//! >
//! > `MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSLayoutManager.h:351`
//!
//! Eine Schriftgroesse, ein Schriftschnitt, eine feste Schrift und ein
//! Absatzeinzug aendern die Auslegung. Als voruebergehendes Merkmal gesetzt tun
//! sie schlicht nichts — nicht falsch, sondern gar nichts. Die Fallunterscheidung
//! ist deshalb nicht "Farbe gegen Rest", sondern **"wirkt auf die Auslegung
//! oder nicht"**, und sie ist trennscharf und vollstaendig: jedes Merkmal
//! faellt in genau einen der beiden Faelle, und welcher es ist, sagt der Kopf
//! oben und nicht eine Liste in KRK.
//!
//! **Die Zusage, an der der Plan haengt, haelt trotzdem, und sie haengt nicht an
//! den voruebergehenden Merkmalen.** Sie lautet: die Auszeichnung kann beim
//! Sichern nicht in die Datei geraten. Der Sicherungsweg liest
//! [`crate::editormodell::Editormodell::stand`], eine gewoehnliche
//! `String`, und die kommt aus `NSTextView::string` — den **Zeichen** der
//! Flaeche. Kein Merkmal, ob im Layoutverwalter oder im Textspeicher, wird auf
//! diesem Weg auch nur gelesen. Der Textspeicher traegt damit dieselbe
//! Unbedenklichkeit wie der Layoutverwalter, und der Unterschied zwischen den
//! beiden Listen ist allein der, ob AppKit das Merkmal beachtet.
//!
//! Der Befund ist als offener Eintrag festgehalten:
//! `issues/260810-0053_o_der-plan-legt-die-markdown-auszeichnung-in-voruebergehende-merkmale-und-die-tragen-sie-nicht.md`.
//!
//! # Ein Durchgang, zwei Verbraucher
//!
//! Die Kiste wird **einmal** ueber den Text gefuehrt.
//! [`syntect::easy::ScopeRegionIterator`] liefert je Stueck zugleich den
//! Wortartenstapel und, ueber [`Highlighter::style_for_stack`], die Farbe. Zwei
//! Durchgaenge, einer fuer die Farbe und einer fuer die Markdown-Auszeichnung,
//! kosteten das Doppelte und koennten auseinanderlaufen.
//!
//! Gemessen am 260810 auf diesem Geraet, `--release`, an
//! `crates/krk-ui/src/appkit/anwendung.rs` (193 kB) und Vielfachen davon:
//!
//! ```text
//!   nur parsen                   0,3 MB/s
//!   parsen + HighlightIterator   0,3 MB/s
//!   parsen + Stapel + Farbe      0,3 MB/s   <- dieser Weg
//! ```
//!
//! Der Aufwand steckt vollstaendig in [`ParseState::parse_line`], also in den
//! Sprachregeln von Sublime Text und `fancy-regex`. Die Farbe kostet nichts
//! dazu, und der Wortartenstapel ebenso wenig. **0,3 MB/s heisst zugleich: das
//! gehoert nicht auf den Hauptfaden.** Eine Datei von 190 kB kostet 0,6 s, eine
//! von 1,5 MB 4,5 s, und die Grenze des Editors liegt bei 16 MB. Deshalb steht
//! [`Einfaerbungsvorgang`] am Ende dieser Datei, und er traegt denselben
//! Zuschnitt wie `Ladevorgang` in [`crate::editormodell`]: ein Faden je Anfrage,
//! `sync_channel(1)`, kein Generationszaehler, weil eine neue Anfrage den alten
//! Empfaenger fallen laesst.
//!
//! **Der Preis, der damit bleibt, ist benannt und nicht verschwiegen:** die
//! Einfaerbung haengt beim Tippen um einen ganzen Durchgang hinterher, in einer
//! Datei von 1,5 MB also um rund 4,5 s. Der Ausweg — `ParseState` je Zeile
//! fortschreiben und an der geaenderten Zeile wieder einsteigen — steht in
//! `issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`,
//! zusammen mit dem Grund, ihn mit
//! `issues/260809-2322_*_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`
//! zusammen zu bewerten: beide Stellen stellen dieselbe Frage.
//!
//! # Zwei Koordinaten, ein Anfang
//!
//! Gerechnet wird in Byteversaetzen, AppKit rechnet in UTF-16-Einheiten. Die
//! Umrechnung geschieht **im Durchgang** und nicht in einem zweiten danach: die
//! Stuecke des Iterators decken die Zeile lueckenlos ab, also ist die Summe
//! ihrer UTF-16-Laengen die Stelle des naechsten. Dieselbe Frage und dieselbe
//! Antwort wie in `crate::appkit::nummernspalte`; ohne sie truege jede Stelle
//! hinter dem ersten Umlaut eine falsche Farbe.
//!
//! # Was diese Datei nicht tut
//!
//! Sie kennt keine Farbe von KRK. Die beiden Tafeln kommen fertig aus der
//! Kiste, und [`Tafel`] waehlt zwischen ihnen; eine eigene Tabelle entsteht
//! nicht. Warum das der Unterschied ist, auf den es ankommt, steht an
//! [`Tafel`].

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};
use std::thread;

use syntect::easy::ScopeRegionIterator;
use syntect::highlighting::{Color, FontStyle, Highlighter, ThemeSet};
use syntect::parsing::{ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

use crate::editormodell::Dateityp;

/// Welche der beiden Farbtafeln gilt (C3, S34).
///
/// **Eine Wahl zwischen zwei fertigen Tafeln und keine eigene Tafel.**
/// `crates/krk-ui/src/appkit/leiste.rs:439-442` und der Modulkopf von
/// `crate::appkit::tableiste` begruenden beide, warum KRK das Erscheinungsbild
/// von Hell und Dunkel nicht nachbaut: man nimmt das Systemsteuerelement, und
/// es folgt dem System von selbst. Fuer Syntaxhervorhebung gibt es kein
/// Systemsteuerelement, also ist hier die erste Stelle des Projekts, an der
/// Farben ueberhaupt vorkommen.
///
/// Der Unterschied, auf den es ankommt: eine **eigene** Tafel muesste bei jeder
/// Systemaenderung nachgezogen werden, eine **Wahl** zwischen zweien nicht. Die
/// Zuordnung ist eine Zeile, [`Tafel::name`], und keine Tabelle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tafel {
    /// Fuer das helle Erscheinungsbild.
    #[default]
    Hell,
    /// Fuer das dunkle.
    Dunkel,
}

impl Tafel {
    /// Der Name der Tafel im Vorgabesatz der Kiste.
    ///
    /// Beide sind vom Pruefcode `crates/krk-ui/tests/syntaxkiste.rs` als
    /// vorhanden und als verschieden faerbend belegt; er schlaegt fehl, sobald
    /// eine von beiden aus dem Satz verschwindet.
    fn name(self) -> &'static str {
        match self {
            Tafel::Hell => "base16-ocean.light",
            Tafel::Dunkel => "base16-ocean.dark",
        }
    }
}

/// Eine Vordergrundfarbe, wie die Kiste sie liefert.
///
/// Ohne Deckkraft: die Tafeln setzen sie durchweg auf undurchsichtig, und eine
/// halbdurchsichtige Schrift ueber der Systemgrundfarbe waere in einem der
/// beiden Erscheinungsbilder blass. Das siebte Abnahmekriterium von C3 verlangt
/// Lesbarkeit in beiden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Farbe {
    /// Rotanteil.
    pub rot: u8,
    /// Gruenanteil.
    pub gruen: u8,
    /// Blauanteil.
    pub blau: u8,
}

impl From<Color> for Farbe {
    fn from(farbe: Color) -> Self {
        Self {
            rot: farbe.r,
            gruen: farbe.g,
            blau: farbe.b,
        }
    }
}

/// Welche der drei Besetzungen der Formatansicht eine Datei bekommt (C3).
///
/// **Die Grenze zwischen Code und einfachem Text wird hier gezogen und nicht im
/// Modell**, und zwar genau so, wie das sechste Abnahmekriterium von C3 sie
/// zieht: "die eingebundene Kiste kennt eine Sprache dafuer". Wer sie im Modell
/// zoege, fuehrte die Sprachliste der Kiste ein zweites Mal und waere ab der
/// ersten Fassung falsch, die eine Sprache nachreicht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Darstellungsart {
    /// Die Kiste kennt keine Sprache dafuer: Umbruch und lesbarere Schrift,
    /// keine Einfaerbung, kein Fehler.
    EinfacherText,
    /// Die Kiste kennt eine Sprache: feste Schrift und Einfaerbung.
    Code,
    /// Markdown: Einfaerbung wie Code, dazu die Auszeichnungen aus dem
    /// Datensatz vom 260808-0140.
    Markdown,
}

/// Ein Stueck Text mit einer Darstellung, die die Auslegung **nicht** aendert.
///
/// Geht als voruebergehendes Merkmal in den Layoutverwalter; siehe den
/// Modulkopf. Die Stellen sind UTF-16-Einheiten und damit unmittelbar ein
/// `NSRange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Einfaerbung {
    /// Anfang in UTF-16-Einheiten.
    pub anfang: usize,
    /// Laenge in UTF-16-Einheiten.
    pub laenge: usize,
    /// Die Vordergrundfarbe aus der Tafel.
    pub farbe: Farbe,
    /// Ob das Stueck unterstrichen wird. Bei Markdown traegt das die Zusage
    /// "Links unterstrichen" aus dem Datensatz vom 260808-0140.
    pub unterstrichen: bool,
}

/// Eine Darstellung, die die Auslegung **aendert**.
///
/// Geht in die Merkmale des Textspeichers, weil der Layoutverwalter sie als
/// voruebergehendes Merkmal nicht beachtet; siehe den Modulkopf.
///
/// **Vollstaendig und ohne Auffangzweig**, wie die vier uebrigen
/// Fallunterscheidungen dieser Art im Programm: eine neue Auszeichnung haelt den
/// Bau an und erzwingt ihre Umsetzung in `crate::appkit::editor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auszeichnung {
    /// Eine Markdown-Ueberschrift: groesser und fett.
    Ueberschrift {
        /// Die Stufe, 1 bis 6, wie `markup.heading.<n>` sie nennt.
        stufe: u8,
    },
    /// Quelltext in Markdown, waagerecht wie im Block: feste Schrift.
    FesteSchrift,
    /// Eine Zeile, die zu einer Liste gehoert: eingerueckt.
    ///
    /// **Eine ganze Zeile und kein Stueck darin.** Der Einzug ist ein Merkmal
    /// des Absatzes; AppKit dehnt ein Absatzmerkmal ohnehin auf den ganzen
    /// Absatz aus, und ein Bereich, der mitten in einer Zeile begaenne, sagte
    /// etwas anderes, als er bewirkt.
    Listenzeile,
}

/// Wo eine [`Auszeichnung`] steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Auszeichnungsstelle {
    /// Anfang in UTF-16-Einheiten.
    pub anfang: usize,
    /// Laenge in UTF-16-Einheiten.
    pub laenge: usize,
    /// Was dort steht.
    pub art: Auszeichnung,
}

/// Alles, was die Formatansicht ueber einen Text weiss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formatierung {
    /// Welche Besetzung die Datei bekommt.
    pub art: Darstellungsart,
    /// Die Laenge des Textes, aus dem diese Formatierung entstanden ist, in
    /// UTF-16-Einheiten.
    ///
    /// **Der Guertel gegen einen Programmabbruch.** Die Formatierung entsteht
    /// auf einem Arbeitsfaden ueber einer Abschrift des Standes; kaeme sie an
    /// einer Textflaeche an, die inzwischen kuerzer ist, waere jeder Bereich
    /// dahinter ein `NSRange` ausserhalb des Textes. AppKit beantwortet das mit
    /// einer Objective-C-Ausnahme, und die ist in Rust nicht zu fangen: sie
    /// beendet das Programm. `crate::appkit::editor` vergleicht deshalb diese
    /// Zahl mit der Laenge der Flaeche und laesst die Lieferung fallen, wenn
    /// sie abweicht.
    pub laenge: usize,
    /// Farbe und Unterstreichung, in Textreihenfolge und ohne Ueberschneidung.
    pub einfaerbungen: Vec<Einfaerbung>,
    /// Die Markdown-Auszeichnungen, in Textreihenfolge.
    pub auszeichnungen: Vec<Auszeichnungsstelle>,
}

impl Formatierung {
    /// Eine Formatierung ohne jede Auszeichnung.
    ///
    /// Der Rueckfall des einfachen Textes und jeder Stelle, an der die Kiste
    /// nichts liefert. **Kein Fehler und keine Meldung**, wie das sechste
    /// Abnahmekriterium von C3 es verlangt.
    fn leer(art: Darstellungsart, laenge: usize) -> Self {
        Self {
            art,
            laenge,
            einfaerbungen: Vec::new(),
            auszeichnungen: Vec::new(),
        }
    }
}

/// Der Satz der Sprachdefinitionen, einmal geladen.
///
/// Aus `two-face` und nicht aus `syntect`, weil er der einzige der beiden ist,
/// der TOML fuehrt; der gemessene Grund steht in der Wurzel-`Cargo.toml` und
/// wird von `crates/krk-ui/tests/syntaxkiste.rs::ohne_two_face_fehlt_toml`
/// festgehalten. Das Laden kostet 2,9 ms, gemessen am 260810; es einmal zu tun
/// statt je Anfrage ist der Unterschied zwischen einer Zahl und einer
/// Zahl mal der Zahl der Tastendruecke.
fn sprachsatz() -> &'static SyntaxSet {
    static SATZ: OnceLock<SyntaxSet> = OnceLock::new();
    SATZ.get_or_init(two_face::syntax::extra_newlines)
}

/// Der Vorgabesatz der Farbtafeln, einmal geladen.
fn tafelsatz() -> &'static ThemeSet {
    static SATZ: OnceLock<ThemeSet> = OnceLock::new();
    SATZ.get_or_init(ThemeSet::load_defaults)
}

/// Die Sprachdefinition, die die Kiste fuer diesen Pfad kennt; `None`, wenn sie
/// keine kennt.
///
/// Gefragt wird zuerst nach dem ganzen Dateinamen und dann nach der Endung.
/// Der Dateiname zuerst, weil Dateien ohne Endung sonst leer ausgingen, obwohl
/// die Kiste sie fuehrt: `Makefile`, `Dockerfile`, `.gitignore`. Beides ohne
/// Ruecksicht auf Gross- und Kleinschreibung, wie `MARKDOWNENDUNGEN` in
/// [`crate::editormodell`].
///
/// **Die Datei wird dabei nicht gelesen.** `syntect` kann eine Sprache auch an
/// der ersten Zeile erkennen; das kostete einen Zugriff auf die Platte fuer eine
/// Frage, die der Editor schon beantwortet hat, als er die Datei einlas.
fn sprache_fuer(pfad: Option<&Path>) -> Option<&'static SyntaxReference> {
    let pfad = pfad?;
    let satz = sprachsatz();
    if let Some(name) = pfad.file_name() {
        let name = name.to_string_lossy().to_ascii_lowercase();
        if let Some(sprache) = satz.find_syntax_by_extension(&name) {
            return Some(sprache);
        }
    }
    let endung = pfad.extension()?.to_string_lossy().to_ascii_lowercase();
    satz.find_syntax_by_extension(&endung)
}

/// Welche Besetzung der Formatansicht diese Datei bekommt (C3).
///
/// **Die eine Stelle, die es entscheidet.** `crate::appkit::editor` fragt sie
/// fuer die Grundschrift und den Umbruch, [`formatieren`] fuer die Einfaerbung;
/// zwei Antworten nebeneinander waeren eine Datei mit fester Schrift ohne
/// Einfaerbung oder umgekehrt.
pub fn art(pfad: Option<&Path>, typ: Dateityp) -> Darstellungsart {
    match typ {
        Dateityp::Markdown => Darstellungsart::Markdown,
        Dateityp::Sonstiges => match sprache_fuer(pfad) {
            Some(_) => Darstellungsart::Code,
            None => Darstellungsart::EinfacherText,
        },
    }
}

/// Der Wortartenstapel, gegen den die Markdown-Auszeichnungen gefragt werden.
///
/// Die sechs Ueberschriftsstufen einzeln, weil die Stufe die Schriftgroesse
/// bestimmt und `markup.heading` allein sie nicht nennt. Verglichen wird ueber
/// [`Scope::is_prefix_of`], also mit einem ganzzahligen Vergleich und nicht mit
/// einer Zeichenkette je Stueck.
struct Wortarten {
    ueberschriften: [Scope; 6],
    liste: Scope,
    quelltext: Scope,
    verweis: Scope,
}

impl Wortarten {
    /// Baut die Vergleichswerte. Die Namen stammen aus den Sprachdefinitionen
    /// von Sublime Text und sind am 260810 an der eingebundenen Fassung
    /// abgelesen, nicht angenommen.
    fn neu() -> Option<Self> {
        let mut ueberschriften = Vec::with_capacity(6);
        for stufe in 1..=6 {
            ueberschriften.push(Scope::new(&format!("markup.heading.{stufe}")).ok()?);
        }
        Some(Self {
            ueberschriften: ueberschriften.try_into().ok()?,
            liste: Scope::new("markup.list").ok()?,
            quelltext: Scope::new("markup.raw").ok()?,
            verweis: Scope::new("markup.underline").ok()?,
        })
    }

    /// Die Ueberschriftsstufe, falls der Stapel eine nennt.
    fn stufe(&self, stapel: &[Scope]) -> Option<u8> {
        // `stufe` laeuft von 1 bis 6, die Stelle in der Liste von 0 bis 5.
        (1u8..=6).find(|stufe| {
            let muster = self.ueberschriften[usize::from(*stufe) - 1];
            Self::traegt(muster, stapel)
        })
    }

    /// Ob der Stapel die genannte Wortart traegt.
    fn traegt(muster: Scope, stapel: &[Scope]) -> bool {
        stapel.iter().any(|art| muster.is_prefix_of(*art))
    }
}

/// Berechnet die Darstellung des Textes (C3).
///
/// Der Text muss der gehaltene Stand sein, also gueltiges UTF-8 mit `\n` als
/// einzigem Zeilenende; `krk_core::text::datei::in_gehaltene_form` stellt das
/// her, und [`crate::editormodell`] haelt nichts anderes.
///
/// **Laeuft auf einem Arbeitsfaden**, siehe [`Einfaerbungsvorgang`] und die
/// Messung im Modulkopf. Sie unmittelbar zu rufen ist zulaessig und im Pruefcode
/// der uebliche Weg; im laufenden Programm tut es niemand.
pub fn formatieren(text: &str, pfad: Option<&Path>, typ: Dateityp, tafel: Tafel) -> Formatierung {
    let art = art(pfad, typ);
    let laenge = text.encode_utf16().count();
    if art == Darstellungsart::EinfacherText {
        // Einfacher Text bekommt Umbruch und eine lesbarere Schrift, und die
        // beiden setzt die Flaeche; eine Einfaerbung gehoert nicht dazu.
        return Formatierung::leer(art, laenge);
    }

    let satz = sprachsatz();
    let sprache = sprache_fuer(pfad)
        .or_else(|| satz.find_syntax_by_extension("md"))
        .unwrap_or_else(|| satz.find_syntax_plain_text());
    let Some(farbtafel) = tafelsatz().themes.get(tafel.name()) else {
        // Der Pruefcode belegt beide Tafeln als vorhanden. Faellt eine spaeter
        // aus dem Satz, faerbt KRK nicht ein, statt anzuhalten: eine fehlende
        // Tafel ist kein Grund, dem Nutzer seine Datei vorzuenthalten.
        return Formatierung::leer(art, laenge);
    };
    let Some(wortarten) = Wortarten::neu() else {
        return Formatierung::leer(art, laenge);
    };

    let faerber = Highlighter::new(farbtafel);
    let grundfarbe = faerber.get_default().foreground;
    let markdown = art == Darstellungsart::Markdown;

    let mut zustand = ParseState::new(sprache);
    let mut stapel = ScopeStack::new();
    let mut einfaerbungen: Vec<Einfaerbung> = Vec::new();
    let mut auszeichnungen: Vec<Auszeichnungsstelle> = Vec::new();
    let mut stelle = 0usize;

    for zeile in LinesWithEndings::from(text) {
        let Ok(ops) = zustand.parse_line(zeile, satz) else {
            // Die Kiste ist mit dieser Zeile nicht fertiggeworden. Was bis
            // hierher steht, bleibt stehen; der Rest bleibt ungefaerbt, und der
            // Nutzer sieht seine Datei. Ein Abbruch waere die schlechtere
            // Antwort, weil er die Datei mitnaehme.
            break;
        };
        let zeilenanfang = stelle;
        let mut zeile_ist_liste = false;

        for (stueck, befehl) in ScopeRegionIterator::new(&ops, zeile) {
            if stapel.apply(befehl).is_err() {
                break;
            }
            let stuecklaenge = stueck.encode_utf16().count();
            if stuecklaenge == 0 {
                continue;
            }
            let arten = stapel.as_slice();
            let stil = faerber.style_for_stack(arten);
            let unterstrichen = stil.font_style.contains(FontStyle::UNDERLINE)
                || (markdown && Wortarten::traegt(wortarten.verweis, arten));

            // Ein Stueck in der Grundfarbe bekommt **kein** Merkmal und behaelt
            // damit die Systemfarbe der Flaeche. Nur die Vordergrundfarben der
            // Wortarten kommen aus der Tafel; so stimmt der Kontrast in beiden
            // Erscheinungsbildern ohne Zutun, und die Tafel muss nur ihre
            // eigenen Farben liefern.
            if stil.foreground != grundfarbe || unterstrichen {
                let neu = Einfaerbung {
                    anfang: stelle,
                    laenge: stuecklaenge,
                    farbe: stil.foreground.into(),
                    unterstrichen,
                };
                anfuegen(&mut einfaerbungen, neu);
            }

            if markdown {
                if let Some(stufe) = wortarten.stufe(arten) {
                    auszeichnung_anfuegen(
                        &mut auszeichnungen,
                        stelle,
                        stuecklaenge,
                        Auszeichnung::Ueberschrift { stufe },
                    );
                } else if Wortarten::traegt(wortarten.quelltext, arten) {
                    auszeichnung_anfuegen(
                        &mut auszeichnungen,
                        stelle,
                        stuecklaenge,
                        Auszeichnung::FesteSchrift,
                    );
                }
                zeile_ist_liste |= Wortarten::traegt(wortarten.liste, arten);
            }

            stelle += stuecklaenge;
        }

        if zeile_ist_liste && stelle > zeilenanfang {
            auszeichnungen.push(Auszeichnungsstelle {
                anfang: zeilenanfang,
                laenge: stelle - zeilenanfang,
                art: Auszeichnung::Listenzeile,
            });
        }
    }

    Formatierung {
        art,
        laenge,
        einfaerbungen,
        auszeichnungen,
    }
}

/// Haengt eine Einfaerbung an und zieht sie mit der vorigen zusammen, wenn
/// beide dasselbe sagen.
///
/// Ein Stueck der Kiste ist oft ein einzelnes Zeichen. Ohne das Zusammenziehen
/// entstuenden fuer eine Datei von 190 kB rund 25 000 Bereiche, und jeder
/// kostete eine Nachricht an AppKit; zusammengezogen sind es ein Bruchteil
/// davon. Es ist dieselbe Angabe, nur einmal statt zwanzigmal gesagt.
fn anfuegen(liste: &mut Vec<Einfaerbung>, neu: Einfaerbung) {
    if let Some(letzte) = liste.last_mut()
        && letzte.anfang + letzte.laenge == neu.anfang
        && letzte.farbe == neu.farbe
        && letzte.unterstrichen == neu.unterstrichen
    {
        letzte.laenge += neu.laenge;
        return;
    }
    liste.push(neu);
}

/// Dasselbe fuer die Auszeichnungen.
fn auszeichnung_anfuegen(
    liste: &mut Vec<Auszeichnungsstelle>,
    anfang: usize,
    laenge: usize,
    art: Auszeichnung,
) {
    if let Some(letzte) = liste.last_mut()
        && letzte.anfang + letzte.laenge == anfang
        && letzte.art == art
    {
        letzte.laenge += laenge;
        return;
    }
    liste.push(Auszeichnungsstelle {
        anfang,
        laenge,
        art,
    });
}

/// Ein laufendes Einfaerben auf einem Arbeitsfaden.
///
/// **Derselbe Zuschnitt wie `Ladevorgang` in [`crate::editormodell`]**, und aus
/// demselben Grund: ein Faden je Anfrage, der genau eine Meldung ueber einen
/// `sync_channel(1)` schickt und endet. Eine Anfragenummer braucht es nicht;
/// eine neue Anfrage laesst den alten Empfaenger fallen, und das `send` des
/// ueberholten Fadens scheitert still.
///
/// **Warum ueberhaupt ein Faden.** 0,3 MB/s, gemessen; siehe den Modulkopf. Der
/// Editor nimmt Dateien bis 16 MB, und der Hauptfaden stuende dafuer knapp eine
/// Minute. S24 hat das Lesen aus genau diesem Grund auf einen Faden gelegt, und
/// zwei Wahrheiten darueber, wann der Hauptfaden anhaelt, will dieses Programm
/// nicht; der Modulkopf von [`crate::editormodell`] schreibt den Satz aus.
#[derive(Debug)]
pub struct Einfaerbungsvorgang {
    empfaenger: Receiver<Formatierung>,
}

/// Was ein [`Einfaerbungsvorgang`] beim Nachfragen sagt.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig.** Entweder der Faden ist
/// fertig, oder er rechnet noch, oder er ist ohne Meldung gefallen und es hat
/// keinen Sinn mehr, auf ihn zu warten.
#[derive(Debug)]
pub enum Abholung {
    /// Die Formatierung steht.
    Fertig(Box<Formatierung>),
    /// Der Faden rechnet noch.
    Laeuft,
    /// Der Faden ist ohne Meldung gefallen.
    Weggefallen,
}

impl Einfaerbungsvorgang {
    /// Startet den Arbeitsfaden fuer den genannten Stand.
    ///
    /// Der Stand wird abgeschrieben und nicht ausgeliehen: er gehoert dem
    /// Modell auf dem Hauptfaden, und der Faden ueberlebt jede Ausleihe. Die
    /// Abschrift kostet einen Durchlauf ueber die Datei, das Einfaerben kostet
    /// drei Groessenordnungen mehr.
    pub fn starten(stand: String, pfad: Option<PathBuf>, typ: Dateityp, tafel: Tafel) -> Self {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        let ergebnis = thread::Builder::new()
            .name("krk-einfaerbung".to_owned())
            .spawn(move || {
                let formatierung = formatieren(&stand, pfad.as_deref(), typ, tafel);
                let _ = SyncSender::send(&sender, formatierung);
            });
        if let Err(fehler) = ergebnis {
            // Ohne Faden kommt nie eine Meldung; der Kanal ist zu diesem
            // Zeitpunkt schon ohne Sender, und der naechste Takt raeumt den
            // Vorgang als `Weggefallen` ab. Dieselbe Zeile und derselbe Grund
            // wie in `Ladevorgang::starten`.
            eprintln!("krk: der Einfaerbungsfaden liess sich nicht starten: {fehler}");
        }
        Self { empfaenger }
    }

    /// Fragt nach, ohne zu warten.
    pub fn abholen(&self) -> Abholung {
        match self.empfaenger.try_recv() {
            Ok(formatierung) => Abholung::Fertig(Box::new(formatierung)),
            Err(TryRecvError::Empty) => Abholung::Laeuft,
            Err(TryRecvError::Disconnected) => Abholung::Weggefallen,
        }
    }
}

/// Die Berechnung ist reine Rechnung und braucht kein Fenster; deshalb stehen
/// die Pruefungen hier und nicht unter `Nutzerarbeit`.
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::*;

    fn pfad(name: &str) -> PathBuf {
        PathBuf::from("/tmp").join(name)
    }

    /// Das sechste Abnahmekriterium von C3: eine Sprache, die die Kiste nicht
    /// kennt, faellt auf die Textdarstellung zurueck und meldet keinen Fehler.
    #[test]
    fn eine_unbekannte_endung_faellt_auf_einfachen_text_zurueck() {
        let datei = pfad("etwas.krk-gibt-es-nicht");
        assert_eq!(
            art(Some(&datei), Dateityp::Sonstiges),
            Darstellungsart::EinfacherText
        );

        let formatierung = formatieren(
            "irgendein Text\n",
            Some(&datei),
            Dateityp::Sonstiges,
            Tafel::Hell,
        );
        assert_eq!(formatierung.art, Darstellungsart::EinfacherText);
        assert!(formatierung.einfaerbungen.is_empty());
        assert!(formatierung.auszeichnungen.is_empty());
    }

    /// Ohne gehaltene Datei gibt es keinen Pfad, und die Kiste kennt keine
    /// Sprache dafuer.
    #[test]
    fn ohne_pfad_gibt_es_einfachen_text() {
        assert_eq!(
            art(None, Dateityp::Sonstiges),
            Darstellungsart::EinfacherText
        );
    }

    /// Das fuenfte Abnahmekriterium von C3, an den vier Sprachen gemessen, die
    /// der Nutzer in KRK selbst bearbeitet.
    #[test]
    fn die_vier_sprachen_gelten_als_code() {
        for name in ["a.rs", "a.toml", "a.sh"] {
            assert_eq!(
                art(Some(&pfad(name)), Dateityp::Sonstiges),
                Darstellungsart::Code,
                "{name} sollte als Code gelten"
            );
        }
        assert_eq!(
            art(Some(&pfad("a.md")), Dateityp::Markdown),
            Darstellungsart::Markdown
        );
    }

    /// Eine Datei ohne Endung, die die Kiste trotzdem fuehrt.
    #[test]
    fn eine_datei_ohne_endung_kann_trotzdem_code_sein() {
        assert_eq!(
            art(Some(&pfad("Makefile")), Dateityp::Sonstiges),
            Darstellungsart::Code
        );
    }

    /// Das dritte Abnahmekriterium von C3 fuer Code: die Wortarten sind
    /// gegeneinander abgesetzt.
    #[test]
    fn code_bekommt_mehrere_farben() {
        let quelle = "fn haupt() { let x = \"Text\"; } // Kommentar\n";
        let formatierung = formatieren(
            quelle,
            Some(&pfad("a.rs")),
            Dateityp::Sonstiges,
            Tafel::Dunkel,
        );
        assert_eq!(formatierung.art, Darstellungsart::Code);

        let farben: BTreeSet<_> = formatierung
            .einfaerbungen
            .iter()
            .map(|stueck| stueck.farbe)
            .collect();
        assert!(
            farben.len() >= 3,
            "nur {} Farbe(n); die Wortarten sind nicht abgesetzt",
            farben.len()
        );
    }

    /// Die beiden Tafeln faerben verschieden; sonst truege die Wahl zwischen
    /// ihnen nichts (S34).
    #[test]
    fn die_beiden_tafeln_faerben_verschieden() {
        let quelle = "fn haupt() { let x = \"Text\"; } // Kommentar\n";
        let datei = pfad("a.rs");
        let hell = formatieren(quelle, Some(&datei), Dateityp::Sonstiges, Tafel::Hell);
        let dunkel = formatieren(quelle, Some(&datei), Dateityp::Sonstiges, Tafel::Dunkel);
        assert_ne!(hell.einfaerbungen, dunkel.einfaerbungen);
    }

    /// Der Datensatz vom 260808-0140: Ueberschriften, Listen und Links werden
    /// erkannt, und die Quelltextzeichen bleiben stehen.
    #[test]
    fn markdown_traegt_ueberschriften_listen_und_verweise() {
        let quelle = "# Eins\n\n## Zwei\n\n- Punkt\n\nText mit [Wort](http://x)\n";
        let formatierung = formatieren(
            quelle,
            Some(&pfad("lies.md")),
            Dateityp::Markdown,
            Tafel::Dunkel,
        );

        let stufen: Vec<u8> = formatierung
            .auszeichnungen
            .iter()
            .filter_map(|stelle| match stelle.art {
                Auszeichnung::Ueberschrift { stufe } => Some(stufe),
                _ => None,
            })
            .collect();
        assert!(stufen.contains(&1), "keine Ueberschrift erster Stufe");
        assert!(stufen.contains(&2), "keine Ueberschrift zweiter Stufe");

        // Die Ueberschrift erster Stufe steht ganz am Anfang und deckt "# Eins"
        // ab; die Quelltextzeichen gehoeren dazu und bleiben damit sichtbar.
        let erste = formatierung
            .auszeichnungen
            .iter()
            .find(|stelle| stelle.art == Auszeichnung::Ueberschrift { stufe: 1 })
            .expect("die erste Ueberschrift");
        assert_eq!(erste.anfang, 0);
        assert!(erste.laenge >= "# Eins".len());

        assert!(
            formatierung
                .auszeichnungen
                .iter()
                .any(|stelle| stelle.art == Auszeichnung::Listenzeile),
            "keine Listenzeile erkannt"
        );
        assert!(
            formatierung
                .einfaerbungen
                .iter()
                .any(|stueck| stueck.unterstrichen),
            "kein unterstrichener Verweis"
        );
    }

    /// Eine Listenzeile deckt eine ganze Zeile ab; der Einzug ist ein
    /// Absatzmerkmal.
    #[test]
    fn eine_listenzeile_beginnt_am_zeilenanfang() {
        let quelle = "Vorspann\n\n- Punkt\n";
        let formatierung = formatieren(
            quelle,
            Some(&pfad("lies.md")),
            Dateityp::Markdown,
            Tafel::Hell,
        );
        let liste = formatierung
            .auszeichnungen
            .iter()
            .find(|stelle| stelle.art == Auszeichnung::Listenzeile)
            .expect("eine Listenzeile");
        // "Vorspann\n" sind neun Einheiten, die Leerzeile eine.
        assert_eq!(liste.anfang, 10);
        assert_eq!(liste.laenge, "- Punkt\n".len());
    }

    /// Der Grund fuer die Umrechnung: ein Umlaut kostet zwei Bytes und eine
    /// UTF-16-Einheit, ein Bildzeichen vier Bytes und zwei. Ohne den Wechsel
    /// truege jede Stelle dahinter eine falsche Farbe.
    #[test]
    fn die_stellen_zaehlen_utf16_und_nicht_bytes() {
        // Der Kommentar steht hinter einer Zeichenkette mit Umlaut und
        // Bildzeichen; in Bytes laege er deutlich weiter hinten als in UTF-16.
        let quelle = "let a = \"Äpfel 🍎\"; // Ende\n";
        let formatierung = formatieren(
            quelle,
            Some(&pfad("a.rs")),
            Dateityp::Sonstiges,
            Tafel::Dunkel,
        );
        assert_eq!(formatierung.laenge, quelle.encode_utf16().count());

        let letzte = formatierung
            .einfaerbungen
            .last()
            .expect("mindestens eine Einfaerbung");
        assert!(
            letzte.anfang + letzte.laenge <= formatierung.laenge,
            "eine Stelle liegt hinter dem Text: {letzte:?} bei Laenge {}",
            formatierung.laenge
        );
        assert!(
            formatierung.laenge < quelle.len(),
            "die Probe traegt keine Mehrbytezeichen und misst deshalb nichts"
        );
    }

    /// Die Stellen ueberschneiden sich nicht und stehen in Textreihenfolge;
    /// darauf ruht, dass die Flaeche sie der Reihe nach setzen kann.
    #[test]
    fn die_stellen_stehen_der_reihe_nach_und_ueberschneiden_sich_nicht() {
        let quelle = "# Kopf\n\n- Punkt\n\n```rust\nfn a() {}\n```\n";
        let formatierung = formatieren(
            quelle,
            Some(&pfad("lies.md")),
            Dateityp::Markdown,
            Tafel::Dunkel,
        );

        let mut ende = 0usize;
        for stueck in &formatierung.einfaerbungen {
            assert!(stueck.anfang >= ende, "{stueck:?} beginnt vor {ende}");
            assert!(stueck.laenge > 0, "{stueck:?} ist leer");
            ende = stueck.anfang + stueck.laenge;
        }
        assert!(ende <= formatierung.laenge);
    }

    /// Der leere Text laesst die Kiste nicht straucheln.
    #[test]
    fn der_leere_text_liefert_nichts_und_meldet_nichts() {
        let formatierung = formatieren("", Some(&pfad("a.rs")), Dateityp::Sonstiges, Tafel::Hell);
        assert_eq!(formatierung.laenge, 0);
        assert!(formatierung.einfaerbungen.is_empty());
    }

    /// Der Arbeitsfaden liefert dasselbe wie der unmittelbare Ruf.
    #[test]
    fn der_arbeitsfaden_liefert_dasselbe() {
        let quelle = "fn haupt() {}\n";
        let datei = pfad("a.rs");
        let unmittelbar = formatieren(quelle, Some(&datei), Dateityp::Sonstiges, Tafel::Dunkel);

        let vorgang = Einfaerbungsvorgang::starten(
            quelle.to_owned(),
            Some(datei),
            Dateityp::Sonstiges,
            Tafel::Dunkel,
        );
        let vom_faden = loop {
            match vorgang.abholen() {
                Abholung::Fertig(formatierung) => break *formatierung,
                Abholung::Laeuft => std::thread::yield_now(),
                Abholung::Weggefallen => panic!("der Faden ist ohne Meldung gefallen"),
            }
        };
        assert_eq!(unmittelbar, vom_faden);
    }
}
