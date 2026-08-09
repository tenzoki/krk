//! Die Kombinationsschreibweise, die eine Tabelle der Tastencodes und die
//! Frage, **wonach** eine Taste nachgeschlagen wird.
//!
//! Drei Dinge stehen hier, und jedes genau einmal im ganzen Programm:
//!
//! 1. **[`TASTEN`], die Tabelle der virtuellen Tastencodes.** Sie ordnet jedem
//!    Namen der Schreibweise eine Zahl zu und sagt zu jeder Zahl, woher sie
//!    stammt. Wer irgendwo einen Tastencode braucht, holt ihn ueber
//!    [`code_von`] oder [`code_von_pflicht`] hier ab; eine zweite Zahl daneben
//!    waere eine zweite Wahrheit ueber dieselbe Taste.
//! 2. **[`Tastenkennung`], wonach eine Taste nachgeschlagen wird.** Buchstaben
//!    und Ziffern ueber das gemeldete **Zeichen**, alles uebrige ueber den
//!    virtuellen **Tastencode**. Der naechste Abschnitt schreibt aus, warum.
//! 3. **[`Kombination`], die gelesene Form von `shift+cmd+k`.** Sie traegt die
//!    Taste und die normalisierte Maske und schreibt sich ueber [`fmt::Display`]
//!    wieder in genau die Zeichenkette zurueck, aus der sie gelesen wurde.
//!
//! # Zwei Nachschlagarten, und warum es zwei sein muessen
//!
//! Ein virtueller Tastencode benennt eine **Stelle** auf der Tastatur und kein
//! Zeichen. Fuer die Funktionstasten und den Pfeilblock ist das die richtige
//! Groesse: F3 liefert denselben Code auf jeder Tastaturbelegung und auch dann,
//! wenn der Nutzer fn haelt. Fuer die Buchstaben ist es die falsche. Die Stelle
//! `kVK_ANSI_Y` traegt den Code 16, und auf einer deutschen Tastatur steht dort
//! ein **Z**; wer die Taste mit der Aufschrift Y drueckt, erzeugt Code 6. Ein
//! `cmd+y` ueber den Code lag damit unter der falschen Aufschrift, und `cmd+z`
//! aus dem Hauptmenue kollidierte mit ihm auf einer Taste
//! (`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/
//! 260809-1642_*_auf-einer-deutschen-tastatur-schluckt-cmd-y-das-rueckgaengig-des-editors.md`).
//!
//! **Der Zuschnitt beendet eine Asymmetrie, statt eine zu schaffen.** Das
//! Hauptmenue schlaegt seit S13b bereits ueber das Zeichen nach:
//! `NSMenuItem.keyEquivalent` nimmt eine **Zeichenkette** entgegen
//! (`crates/krk-ui/src/appkit/menue.rs:322-342`, die Zuordnung in
//! `zeichen_der_taste` dort trennt einbuchstabige Namen von den uebrigen genau
//! wie [`Taste::kennung`] hier), und genau deshalb wirken `cmd+c` und `cmd+v`
//! auf jeder Tastaturbelegung an der beschrifteten Stelle. Die zeichenbasierte
//! Nachschlagart ist im Projekt keine fremde Mechanik, sondern die, die vier
//! Funktionen schon tragen; bis zum 260809 trug der Ereignisabgriff sie nur
//! nicht mit.
//!
//! **Die Festlegung aus C3 der Runde 1 bleibt und wird gegenstandslos.** KRK
//! erkennt die Tastaturbauart nicht und liefert keine geraeteabhaengige
//! Vorbelegung aus; es liest zu jedem Tastendruck das Zeichen, das das System
//! ohnehin meldet, und braucht dafuer nicht zu wissen, welche Tastatur davor
//! steht. Jede Kombination, die etwas ausloest, steht weiter in der Belegung,
//! wird von der Konflikterkennung gesehen und ist umbelegbar.
//!
//! Nutzerentscheid vom 260808-0155,
//! `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/
//! 260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`.
//!
//! # Die Schreibweise
//!
//! `[ctrl+][opt+][shift+][cmd+]<taste>`, in genau dieser Reihenfolge. Sie ist
//! die Reihenfolge, in der macOS die Zusatztasten schreibt (⌃⌥⇧⌘), und der
//! Kopf von `resources/default-keymap.toml` beschreibt sie als Vertrag dieses
//! Parsers. Die Namen der vier Zusatztasten stehen nicht hier, sondern in
//! [`ModMaske::BENANNT`]; sie gehoeren der Maske und werden zum Lesen wie zum
//! Schreiben von dort geholt.
//!
//! Die Reihenfolge wird **erzwungen** und nicht bloss angeboten. `cmd+shift+k`
//! ist ein Fehler und nicht die zweite Schreibweise fuer `shift+cmd+k`: zwei
//! Schreibweisen fuer eine Kombination waeren zwei Zeilen in der Belegungsdatei,
//! die dasselbe meinen, und der Vergleich zweier Belegungen muesste sie erst
//! wieder auf eine Form bringen.
//!
//! Die fn-Taste ist keine Zusatztaste dieser Schreibweise. C3 des Specs
//! verlangt das ausdruecklich, und
//! [`normalisieren`](super::normalisierung::normalisieren) loescht das Bit
//! schon vor dem Nachschlag; [`Schreibfehler::FnAlsZusatztaste`] sagt es dem,
//! der es dennoch schreibt.
//!
//! # Woher die Tastencodes stammen
//!
//! Jeder Eintrag traegt seine [`Herkunft`], und die Unterscheidung ist keine
//! Formalitaet, sondern die Belegkette. **Drei Codes sind am Referenzgeraet
//! gemessen** (F3, F5 und F8 mit 99, 96 und 100, `spikes/fn-tasten/messung-A.txt`
//! Ereignisse #03 bis #05); **alle uebrigen sind nur dokumentiert**, aus der
//! Carbon-Tabelle `kVK_*` in `HIToolbox.framework/Headers/Events.h` des
//! macOS-SDK, nachgesehen am 260803 und fuer die acht Nachtraege erneut am
//! 260804. Fuer F4, F6 und F7 heisst das: 118, 97 und 98 hat in diesem Projekt
//! niemand gedrueckt. Sie stehen hier, weil die Auslieferungsbelegung sie
//! braucht, und sie stehen als [`Herkunft::Dokumentiert`], damit niemand sie
//! fuer gemessen haelt. Dasselbe gilt fuer `left`, `right` und die sechs
//! Funktionstasten ausserhalb der Norton-Reihe.
//!
//! # Wann die Tabelle waechst
//!
//! **Um ganze Tastengruppen, nie um einzelne Tasten.** Eine halbe Gruppe kostet
//! genau das, was dieses Projekt mit "Pfeil hoch und runter ja, links und
//! rechts nein" schon einmal bezahlt hat: die Bereichsbreiten aus C7 stehen
//! seither auf den Behelfsbelegungen `ctrl+b` und `ctrl+s`. Ein Eintrag hier
//! ist **keine Belegung**: eine Taste, die in keiner Tastenliste von
//! `resources/default-keymap.toml` steht, loest nichts aus. Er ist die
//! Voraussetzung dafuer, dass der Nutzer sie ueberhaupt belegen kann: die
//! Belegungsansicht weist eine Kombination zu, indem der Nutzer sie drueckt,
//! und [`Kombination::aus_tastendruck`] liefert fuer eine Taste ohne Namen
//! `None`.

use std::fmt;
use std::str::FromStr;

use super::Tastendruck;
use super::normalisierung::ModMaske;

/// Woher der Tastencode eines Eintrags stammt.
///
/// Beide Faelle nennen den Carbon-Namen. Der Unterschied liegt allein darin, ob
/// das Projekt die Zahl selbst gesehen hat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Herkunft {
    /// Am Referenzgeraet gemessen. `beleg` nennt die Fundstelle im Messbericht.
    Gemessen {
        /// Der Name aus der Carbon-Tabelle `kVK_*`.
        kvk: &'static str,
        /// Die Fundstelle der Messung.
        beleg: &'static str,
    },
    /// Aus der Carbon-Tabelle uebernommen und im Projekt nie gemessen.
    Dokumentiert {
        /// Der Name aus der Carbon-Tabelle `kVK_*`.
        kvk: &'static str,
    },
}

impl Herkunft {
    /// Der Name aus der Carbon-Tabelle, in beiden Faellen.
    pub const fn kvk(self) -> &'static str {
        match self {
            Herkunft::Gemessen { kvk, .. } | Herkunft::Dokumentiert { kvk } => kvk,
        }
    }

    /// Wahr, wenn das Projekt diesen Tastencode selbst gemessen hat.
    pub const fn ist_gemessen(self) -> bool {
        matches!(self, Herkunft::Gemessen { .. })
    }
}

/// Wonach ein Tastendruck nachgeschlagen wird.
///
/// Die eine Stelle, an der die beiden Nachschlagarten des Modulkopfes
/// auseinandergehen. Sie sind **verschieden und vollstaendig**: jede Taste der
/// Tabelle traegt genau eine der beiden Kennungen, und zwei Varianten sind nie
/// gleich. Genau das haelt sie auseinander, wo eine Tastaturbelegung sie
/// kreuzt: auf einer franzoesischen Tastatur meldet die Stelle
/// `kVK_ANSI_Semicolon` ein `m`, und die Stelle `kVK_ANSI_M` meldet ein Komma.
/// Die erste findet ueber [`Tastenkennung::Zeichen`] die Taste `m`; die zweite
/// traegt kein Zeichen der Schreibweise, faellt auf
/// [`Tastenkennung::Code`] und findet nichts — statt ueber ihren Code 46
/// dieselbe Taste `m` ein zweites Mal zu treffen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tastenkennung {
    /// Ueber das gemeldete Zeichen: die Buchstaben und die Ziffern.
    ///
    /// Immer ein ASCII-Kleinbuchstabe oder eine ASCII-Ziffer;
    /// [`zeichen_als_kennung`] stellt das fuer jeden Weg hierher sicher.
    Zeichen(char),
    /// Ueber den virtuellen Tastencode: die Funktionstasten, der Pfeilblock und
    /// die Steuertasten.
    Code(u16),
}

/// Ein Eintrag der Tastentabelle: ein Name der Schreibweise, sein Tastencode
/// und dessen Herkunft.
///
/// Ein virtueller Tastencode benennt die **Stelle** auf der Tastatur und nicht
/// das Zeichen. Fuer die Funktionstasten ist das die richtige Groesse, fuer die
/// Buchstaben nicht; welche der beiden Kennungen ein Eintrag traegt, sagt
/// [`Taste::kennung`], und der Modulkopf sagt, warum es zwei sind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Taste {
    /// Der Name, unter dem die Schreibweise diese Taste kennt.
    pub name: &'static str,
    /// Der virtuelle Tastencode aus `NSEvent.keyCode`.
    pub code: u16,
    /// Woher der Tastencode stammt.
    pub herkunft: Herkunft,
}

impl Taste {
    /// Wonach diese Taste nachgeschlagen wird.
    ///
    /// **Eine Regel und keine Liste von Sonderfaellen.** Ein einbuchstabiger
    /// Name aus einem ASCII-Kleinbuchstaben oder einer ASCII-Ziffer ist sein
    /// eigenes Zeichen; jeder andere Name benennt eine Stelle. Das deckt die
    /// Tabelle vollstaendig ab, weil ihre Namen genau in diese beiden Sorten
    /// zerfallen, und es ist dieselbe Regel, nach der
    /// `zeichen_der_taste` in `crates/krk-ui/src/appkit/menue.rs` seit S13b das
    /// Menuekuerzel bildet.
    pub const fn kennung(self) -> Tastenkennung {
        let name = self.name.as_bytes();
        if name.len() == 1 && (name[0].is_ascii_lowercase() || name[0].is_ascii_digit()) {
            return Tastenkennung::Zeichen(name[0] as char);
        }
        Tastenkennung::Code(self.code)
    }

    /// Das Zeichen dieser Taste, falls sie ueber eines nachgeschlagen wird.
    pub const fn zeichen(self) -> Option<char> {
        match self.kennung() {
            Tastenkennung::Zeichen(zeichen) => Some(zeichen),
            Tastenkennung::Code(_) => None,
        }
    }
}

/// Ein gemessener Eintrag der Tabelle.
const fn gemessen(name: &'static str, code: u16, kvk: &'static str, beleg: &'static str) -> Taste {
    Taste {
        name,
        code,
        herkunft: Herkunft::Gemessen { kvk, beleg },
    }
}

/// Ein nur dokumentierter Eintrag der Tabelle.
const fn dokumentiert(name: &'static str, code: u16, kvk: &'static str) -> Taste {
    Taste {
        name,
        code,
        herkunft: Herkunft::Dokumentiert { kvk },
    }
}

/// Die Fundstelle der Messung, je Funktionstaste.
const MESSUNG: &str = "spikes/fn-tasten/messung-A.txt";

/// Alle Tasten, die die Schreibweise benennen kann.
///
/// Die eine Tabelle. Sie deckt die Funktionstastenreihe `f1` bis `f12`, den
/// Pfeilblock `up`, `down`, `left` und `right`, dazu `delete`, `pageup`,
/// `pagedown`, `home`, `end`, `return`, `tab`, `esc`, `space` sowie die
/// Buchstaben und die Ziffern.
///
/// Nicht enthalten sind die Satzzeichen und der Zehnerblock. Ein virtueller
/// Tastencode benennt eine **Stelle** auf der Tastatur, und bei den
/// Satzzeichen laeuft die Beschriftung dieser Stelle je nach Tastaturbelegung
/// weit auseinander: `kVK_ANSI_LeftBracket` traegt auf einer deutschen
/// Tastatur ein `ü`. Ein Name `bracketleft` bezeichnete fuer einen deutschen
/// Nutzer eine Taste, die er nicht findet.
pub const TASTEN: [Taste; 61] = [
    // Die Norton-Reihe. Drei dieser sechs Codes sind gemessen, drei nicht.
    gemessen("f3", 99, "kVK_F3", MESSUNG),
    dokumentiert("f4", 118, "kVK_F4"),
    gemessen("f5", 96, "kVK_F5", MESSUNG),
    dokumentiert("f6", 97, "kVK_F6"),
    dokumentiert("f7", 98, "kVK_F7"),
    gemessen("f8", 100, "kVK_F8", MESSUNG),
    // Der Rest der Funktionstastenreihe. F1 bis F12 traegt jede Mac-Tastatur,
    // auch die Touch Bar des Referenzgeraets. F13 aufwaerts gibt es allein auf
    // der Tastatur mit Zehnerblock und steht deshalb nicht hier.
    dokumentiert("f1", 122, "kVK_F1"),
    dokumentiert("f2", 120, "kVK_F2"),
    dokumentiert("f9", 101, "kVK_F9"),
    dokumentiert("f10", 109, "kVK_F10"),
    dokumentiert("f11", 103, "kVK_F11"),
    dokumentiert("f12", 111, "kVK_F12"),
    // Steuertasten.
    dokumentiert("delete", 51, "kVK_Delete"),
    dokumentiert("return", 36, "kVK_Return"),
    dokumentiert("tab", 48, "kVK_Tab"),
    dokumentiert("esc", 53, "kVK_Escape"),
    dokumentiert("space", 49, "kVK_Space"),
    // Der Pfeilblock, vollstaendig, und die uebrige Bewegung im Blatt.
    dokumentiert("up", 126, "kVK_UpArrow"),
    dokumentiert("down", 125, "kVK_DownArrow"),
    dokumentiert("left", 123, "kVK_LeftArrow"),
    dokumentiert("right", 124, "kVK_RightArrow"),
    dokumentiert("pageup", 116, "kVK_PageUp"),
    dokumentiert("pagedown", 121, "kVK_PageDown"),
    dokumentiert("home", 115, "kVK_Home"),
    dokumentiert("end", 119, "kVK_End"),
    // Die Buchstaben, in der Reihenfolge des Alphabets und nicht der Codes.
    dokumentiert("a", 0, "kVK_ANSI_A"),
    dokumentiert("b", 11, "kVK_ANSI_B"),
    dokumentiert("c", 8, "kVK_ANSI_C"),
    dokumentiert("d", 2, "kVK_ANSI_D"),
    dokumentiert("e", 14, "kVK_ANSI_E"),
    dokumentiert("f", 3, "kVK_ANSI_F"),
    dokumentiert("g", 5, "kVK_ANSI_G"),
    dokumentiert("h", 4, "kVK_ANSI_H"),
    dokumentiert("i", 34, "kVK_ANSI_I"),
    dokumentiert("j", 38, "kVK_ANSI_J"),
    dokumentiert("k", 40, "kVK_ANSI_K"),
    dokumentiert("l", 37, "kVK_ANSI_L"),
    dokumentiert("m", 46, "kVK_ANSI_M"),
    dokumentiert("n", 45, "kVK_ANSI_N"),
    dokumentiert("o", 31, "kVK_ANSI_O"),
    dokumentiert("p", 35, "kVK_ANSI_P"),
    dokumentiert("q", 12, "kVK_ANSI_Q"),
    dokumentiert("r", 15, "kVK_ANSI_R"),
    dokumentiert("s", 1, "kVK_ANSI_S"),
    dokumentiert("t", 17, "kVK_ANSI_T"),
    dokumentiert("u", 32, "kVK_ANSI_U"),
    dokumentiert("v", 9, "kVK_ANSI_V"),
    dokumentiert("w", 13, "kVK_ANSI_W"),
    dokumentiert("x", 7, "kVK_ANSI_X"),
    dokumentiert("y", 16, "kVK_ANSI_Y"),
    dokumentiert("z", 6, "kVK_ANSI_Z"),
    // Die Ziffern der oberen Reihe. Der Zehnerblock traegt eigene Codes und
    // steht nicht in der Schreibweise.
    dokumentiert("0", 29, "kVK_ANSI_0"),
    dokumentiert("1", 18, "kVK_ANSI_1"),
    dokumentiert("2", 19, "kVK_ANSI_2"),
    dokumentiert("3", 20, "kVK_ANSI_3"),
    dokumentiert("4", 21, "kVK_ANSI_4"),
    dokumentiert("5", 23, "kVK_ANSI_5"),
    dokumentiert("6", 22, "kVK_ANSI_6"),
    dokumentiert("7", 26, "kVK_ANSI_7"),
    dokumentiert("8", 28, "kVK_ANSI_8"),
    dokumentiert("9", 25, "kVK_ANSI_9"),
];

/// Zeichenweiser Vergleich zweier Namen zur Uebersetzungszeit.
///
/// `str`-Vergleich ist in einer `const fn` nicht zu haben; dieser Ersatz macht
/// [`code_von`] in einem `const`-Zusammenhang benutzbar und damit die Zusage
/// "die Tabelle steht an genau einer Stelle" auch dort einloesbar, wo bisher
/// eine abgeschriebene Zahl stand.
const fn namen_gleich(links: &str, rechts: &str) -> bool {
    let (links, rechts) = (links.as_bytes(), rechts.as_bytes());
    if links.len() != rechts.len() {
        return false;
    }
    let mut stelle = 0;
    while stelle < links.len() {
        if links[stelle] != rechts[stelle] {
            return false;
        }
        stelle += 1;
    }
    true
}

/// Der Tastencode zu einem Namen der Schreibweise, falls die Tabelle ihn kennt.
///
/// Zur Uebersetzungszeit auswertbar, damit auch eine Konstante ihre Zahl von
/// hier holen kann statt sie abzuschreiben.
pub const fn code_von(name: &str) -> Option<u16> {
    let mut stelle = 0;
    while stelle < TASTEN.len() {
        if namen_gleich(TASTEN[stelle].name, name) {
            return Some(TASTEN[stelle].code);
        }
        stelle += 1;
    }
    None
}

/// Wie [`code_von`], aber ein unbekannter Name bricht die Uebersetzung ab.
///
/// Fuer Konstanten, deren Name im Programmtext steht und den die Tabelle
/// deshalb kennen muss. Ein Tippfehler wird zum Uebersetzungsfehler und nicht
/// zu einer toten Taste.
pub const fn code_von_pflicht(name: &str) -> u16 {
    match code_von(name) {
        Some(code) => code,
        None => panic!("die Tastentabelle kennt diesen Namen nicht"),
    }
}

/// Das Zeichen der Taste an dieser Stelle, falls die Tabelle sie kennt und
/// ueber ein Zeichen nachschlaegt.
///
/// Zur Uebersetzungszeit auswertbar, damit [`Tastendruck::neu`] es sein bleibt.
pub const fn zeichen_der_stelle(code: u16) -> Option<char> {
    let mut stelle = 0;
    while stelle < TASTEN.len() {
        if TASTEN[stelle].code == code {
            return TASTEN[stelle].zeichen();
        }
        stelle += 1;
    }
    None
}

/// Das Zeichen, unter dem ein gemeldetes Zeichen nachgeschlagen wird.
///
/// **Die eine Stelle, die ein gemeldetes Zeichen auf die Form der Tabelle
/// bringt.** Sie beantwortet zwei Fragen auf einmal: taugt dieses Zeichen
/// ueberhaupt als Kennung, und in welcher Schreibung. Gross- und Kleinbuchstabe
/// sind dieselbe Taste — die Umschalttaste steht als eigenes Bit in der Maske,
/// und ein `Y` neben einem `y` waeren zwei Eintraege fuer eine Taste. Alles
/// ausserhalb von ASCII faellt weg: die Tabelle fuehrt keine Umlaute und keine
/// Satzzeichen (siehe [`TASTEN`]), und die Funktionstasten melden Zeichen aus
/// dem privaten Bereich von Unicode, die zur Stelle und nicht zum Zeichen
/// gehoeren.
///
/// `None` heisst deshalb: dieser Tastendruck wird ueber seinen Tastencode
/// nachgeschlagen.
pub fn zeichen_als_kennung(gemeldet: char) -> Option<char> {
    let klein = gemeldet.to_ascii_lowercase();
    klein.is_ascii_alphanumeric().then_some(klein)
}

/// Der Tabelleneintrag zu einem Namen der Schreibweise.
pub fn taste_mit_namen(name: &str) -> Option<Taste> {
    TASTEN.into_iter().find(|taste| taste.name == name)
}

/// Der Tabelleneintrag zu einem Tastencode.
///
/// **Ohne Ruecksicht darauf, wonach die gefundene Taste nachgeschlagen wird.**
/// Wer eine Taste sucht, weil ein Tastendruck sie ausgeloest hat, nimmt
/// [`Kombination::aus_tastendruck`]; das fragt zuerst die Kennung und findet
/// eine Buchstabentaste nicht ueber ihre Stelle.
pub fn taste_mit_code(code: u16) -> Option<Taste> {
    TASTEN.into_iter().find(|taste| taste.code == code)
}

/// Der Tabelleneintrag zu einem gemeldeten Zeichen.
///
/// Das Zeichen geht durch [`zeichen_als_kennung`]; ein `Y` findet damit
/// dieselbe Taste wie ein `y`.
pub fn taste_mit_zeichen(gemeldet: char) -> Option<Taste> {
    let kennung = Tastenkennung::Zeichen(zeichen_als_kennung(gemeldet)?);
    TASTEN.into_iter().find(|taste| taste.kennung() == kennung)
}

/// Warum eine Zeichenkette keine Kombination ergibt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schreibfehler {
    /// Vor dem letzten `+` steht kein Tastenname.
    LeereTaste,
    /// Ein Teil vor dem letzten `+` ist keine der vier Zusatztasten.
    UnbekannteZusatztaste(String),
    /// `fn` ist nach C3 keine Zusatztaste einer Belegung.
    FnAlsZusatztaste,
    /// Dieselbe Zusatztaste steht zweimal.
    ZusatztasteDoppelt(String),
    /// Die Zusatztasten stehen nicht in der vorgeschriebenen Reihenfolge.
    ReihenfolgeVerletzt {
        /// Die Zusatztaste, die zu spaet steht.
        zusatztaste: String,
        /// Die, hinter der sie steht und vor der sie stehen muesste.
        hinter: String,
    },
    /// Die Tabelle [`TASTEN`] kennt diesen Tastennamen nicht.
    UnbekannterTastenname(String),
}

impl fmt::Display for Schreibfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Schreibfehler::LeereTaste => ausgabe.write_str("es fehlt der Tastenname"),
            Schreibfehler::UnbekannteZusatztaste(text) => write!(
                ausgabe,
                "\"{text}\" ist keine Zusatztaste; erlaubt sind {}",
                zusatztasten_aufzaehlen()
            ),
            Schreibfehler::FnAlsZusatztaste => ausgabe.write_str(
                "fn ist keine Zusatztaste einer Belegung; KRK belegt den Tastencode, \
                 und F3 mit gehaltener fn erzeugt denselben wie ein nacktes F3",
            ),
            Schreibfehler::ZusatztasteDoppelt(text) => {
                write!(ausgabe, "die Zusatztaste \"{text}\" steht zweimal")
            }
            Schreibfehler::ReihenfolgeVerletzt {
                zusatztaste,
                hinter,
            } => write!(
                ausgabe,
                "\"{zusatztaste}\" steht hinter \"{hinter}\"; \
                 die Reihenfolge ist {}",
                zusatztasten_aufzaehlen()
            ),
            Schreibfehler::UnbekannterTastenname(text) => {
                write!(
                    ausgabe,
                    "\"{text}\" ist kein Tastenname dieser Schreibweise"
                )
            }
        }
    }
}

impl std::error::Error for Schreibfehler {}

/// Die vier Zusatztasten als Aufzaehlung, fuer die Fehlermeldungen.
fn zusatztasten_aufzaehlen() -> String {
    ModMaske::BENANNT
        .iter()
        .map(|(_, name)| *name)
        .collect::<Vec<&str>>()
        .join(", ")
}

/// Eine gelesene Kombination: eine Taste und die Zusatztasten davor.
///
/// Der Nachschlag geht ueber [`Kombination::tastendruck`] und damit ueber die
/// **normalisierte** Maske; die rohen Flaggen eines AppKit-Ereignisses kommen
/// hier nie an.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Kombination {
    taste: Taste,
    maske: ModMaske,
}

impl Kombination {
    /// Eine Kombination aus einem Tabelleneintrag und einer Maske.
    pub const fn neu(taste: Taste, maske: ModMaske) -> Self {
        Self { taste, maske }
    }

    /// Liest die Schreibweise `[ctrl+][opt+][shift+][cmd+]<taste>`.
    pub fn lesen(text: &str) -> Result<Self, Schreibfehler> {
        let teile: Vec<&str> = text.split('+').collect();
        let Some((name, zusaetze)) = teile.split_last() else {
            return Err(Schreibfehler::LeereTaste);
        };

        let mut maske = ModMaske::LEER;
        let mut letzte: Option<usize> = None;
        for zusatz in zusaetze {
            if *zusatz == "fn" {
                return Err(Schreibfehler::FnAlsZusatztaste);
            }
            let Some(stelle) = ModMaske::BENANNT
                .iter()
                .position(|(_, benannt)| benannt == zusatz)
            else {
                return Err(Schreibfehler::UnbekannteZusatztaste((*zusatz).to_owned()));
            };
            let (bit, _) = ModMaske::BENANNT[stelle];
            if maske.enthaelt(bit) {
                return Err(Schreibfehler::ZusatztasteDoppelt((*zusatz).to_owned()));
            }
            if let Some(vorige) = letzte
                && stelle < vorige
            {
                return Err(Schreibfehler::ReihenfolgeVerletzt {
                    zusatztaste: (*zusatz).to_owned(),
                    hinter: ModMaske::BENANNT[vorige].1.to_owned(),
                });
            }
            maske |= bit;
            letzte = Some(stelle);
        }

        if name.is_empty() {
            return Err(Schreibfehler::LeereTaste);
        }
        let Some(taste) = taste_mit_namen(name) else {
            return Err(Schreibfehler::UnbekannterTastenname((*name).to_owned()));
        };
        Ok(Self::neu(taste, maske))
    }

    /// Die Kombination zu einem Tastendruck, falls die Tabelle die Taste kennt.
    ///
    /// `None` heisst: diese Taste hat in der Schreibweise keinen Namen und
    /// laesst sich deshalb nicht in `keymap.toml` ablegen. Der Aufrufer sagt das
    /// dem Nutzer, statt eine Zeile zu schreiben, die niemand wieder einlesen
    /// kann.
    ///
    /// **Gesucht wird ueber die Kennung und nicht ueber den Tastencode.** Das
    /// ist die Bedingung dafuer, dass die Belegungsansicht aus C3 auf jeder
    /// Tastaturbelegung die Taste aufschreibt, die der Nutzer gedrueckt hat:
    /// wer auf einer deutschen Tastatur die Taste mit der Aufschrift Y drueckt,
    /// bekommt `y` in seine `keymap.toml` und nicht `z`.
    ///
    /// Die Stellensuche laesst dabei die Buchstaben und Ziffern **aus**. Ein
    /// Tastendruck ohne brauchbares Zeichen darf nicht ueber seinen Code bei
    /// einer Taste landen, die selbst ueber ihr Zeichen nachgeschlagen wird;
    /// [`Tastenkennung`] fuehrt den Fall aus, in dem das ohne diese Zeile
    /// einträte.
    pub fn aus_tastendruck(druck: Tastendruck) -> Option<Self> {
        let taste = match druck.kennung() {
            Tastenkennung::Zeichen(zeichen) => taste_mit_zeichen(zeichen),
            Tastenkennung::Code(code) => taste_mit_code(code)
                .filter(|taste| matches!(taste.kennung(), Tastenkennung::Code(_))),
        }?;
        Some(Self::neu(taste, druck.maske))
    }

    /// Der Tabelleneintrag der Taste.
    pub const fn taste(self) -> Taste {
        self.taste
    }

    /// Die normalisierte Maske der Zusatztasten.
    pub const fn maske(self) -> ModMaske {
        self.maske
    }

    /// Der Tastendruck, unter dem diese Kombination nachgeschlagen wird.
    ///
    /// Er traegt beides: die Stelle und, fuer eine Buchstaben- oder
    /// Zifferntaste, das Zeichen. Welches von beidem der Nachschlag vergleicht,
    /// entscheidet [`Tastendruck::kennung`].
    pub const fn tastendruck(self) -> Tastendruck {
        Tastendruck {
            code: self.taste.code,
            zeichen: self.taste.zeichen(),
            maske: self.maske,
        }
    }
}

impl fmt::Display for Kombination {
    /// Schreibt die Kombination in genau der Form, aus der sie gelesen wurde.
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.maske.ist_leer() {
            write!(ausgabe, "{}+", self.maske)?;
        }
        ausgabe.write_str(self.taste.name)
    }
}

impl FromStr for Kombination {
    type Err = Schreibfehler;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::lesen(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jeder_name_und_jeder_code_steht_genau_einmal() {
        for (stelle, taste) in TASTEN.into_iter().enumerate() {
            for andere in TASTEN.into_iter().skip(stelle + 1) {
                assert_ne!(taste.name, andere.name, "der Name steht zweimal");
                assert_ne!(taste.code, andere.code, "der Tastencode steht zweimal");
            }
        }
    }

    #[test]
    fn die_tabelle_deckt_die_ganze_schreibweise_ab() {
        let benannt: Vec<&str> = TASTEN.iter().map(|taste| taste.name).collect();
        for name in [
            "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "delete",
            "up", "down", "left", "right", "pageup", "pagedown", "home", "end", "return", "tab",
            "esc", "space",
        ] {
            assert!(benannt.contains(&name), "{name} fehlt in der Tabelle");
        }
        for buchstabe in 'a'..='z' {
            assert!(
                code_von(&buchstabe.to_string()).is_some(),
                "{buchstabe} fehlt"
            );
        }
        for ziffer in '0'..='9' {
            assert!(code_von(&ziffer.to_string()).is_some(), "{ziffer} fehlt");
        }
    }

    /// Die Fallunterscheidung der beiden Nachschlagarten ist verschieden und
    /// vollstaendig, und die Tabelle zerfaellt genau in die beiden Sorten.
    #[test]
    fn jede_taste_traegt_genau_eine_kennung_und_keine_zwei_dieselbe() {
        for (stelle, taste) in TASTEN.into_iter().enumerate() {
            match taste.kennung() {
                Tastenkennung::Zeichen(zeichen) => {
                    assert_eq!(
                        taste.name,
                        zeichen.to_string(),
                        "{} wird ueber ein Zeichen nachgeschlagen, das nicht sein Name ist",
                        taste.name
                    );
                    assert!(zeichen.is_ascii_alphanumeric() && !zeichen.is_ascii_uppercase());
                }
                Tastenkennung::Code(code) => {
                    assert_eq!(code, taste.code);
                    assert!(
                        taste.name.len() > 1,
                        "{} ist einbuchstabig und wird trotzdem ueber die Stelle nachgeschlagen",
                        taste.name
                    );
                }
            }
            // Zwei Tasten mit derselben Kennung waeren zwei Funktionen auf
            // einem Tastendruck, die die Konflikterkennung nicht sieht.
            for andere in TASTEN.into_iter().skip(stelle + 1) {
                assert_ne!(
                    taste.kennung(),
                    andere.kennung(),
                    "{} und {} werden unter derselben Kennung nachgeschlagen",
                    taste.name,
                    andere.name
                );
            }
        }
    }

    #[test]
    fn buchstaben_und_ziffern_gehen_ueber_das_zeichen_die_uebrigen_ueber_den_code() {
        for zeichen in ('a'..='z').chain('0'..='9') {
            let Some(taste) = taste_mit_zeichen(zeichen) else {
                panic!("die Tabelle findet {zeichen} nicht ueber das Zeichen");
            };
            assert_eq!(taste.kennung(), Tastenkennung::Zeichen(zeichen));
            // Gross geschrieben ist es dieselbe Taste: die Umschalttaste steht
            // in der Maske und nicht im Zeichen.
            assert_eq!(taste_mit_zeichen(zeichen.to_ascii_uppercase()), Some(taste));
        }
        for name in ["f3", "down", "delete", "space", "esc"] {
            let Some(taste) = taste_mit_namen(name) else {
                panic!("die Tabelle kennt {name} nicht");
            };
            assert_eq!(taste.kennung(), Tastenkennung::Code(taste.code));
            assert_eq!(taste.zeichen(), None);
        }
    }

    /// Was als Kennung taugt, und was auf den Tastencode zurueckfaellt.
    #[test]
    fn nur_ascii_buchstaben_und_ziffern_taugen_als_zeichenkennung() {
        assert_eq!(zeichen_als_kennung('y'), Some('y'));
        assert_eq!(zeichen_als_kennung('Y'), Some('y'));
        assert_eq!(zeichen_als_kennung('7'), Some('7'));
        // Ein Umlaut: die Tabelle fuehrt ihn nicht, siehe ihren Kopf.
        assert_eq!(zeichen_als_kennung('ü'), None);
        // Das Zeichen, das AppKit einer F3 beilegt (`NSF3FunctionKey`). Es
        // gehoert zur Stelle und nicht zum Zeichen.
        assert_eq!(zeichen_als_kennung('\u{F706}'), None);
        // Ein Satzzeichen, etwa das, das die Stelle `kVK_ANSI_M` auf einer
        // franzoesischen Tastatur meldet.
        assert_eq!(zeichen_als_kennung(','), None);
    }

    #[test]
    fn code_von_ist_zur_uebersetzungszeit_auswertbar() {
        const PFEIL_AB: u16 = code_von_pflicht("down");
        assert_eq!(PFEIL_AB, 125);
        assert_eq!(code_von("gibtsnicht"), None);
    }

    #[test]
    fn die_reihenfolge_der_zusatztasten_wird_erzwungen() {
        assert!(Kombination::lesen("shift+cmd+k").is_ok());
        assert_eq!(
            Kombination::lesen("cmd+shift+k"),
            Err(Schreibfehler::ReihenfolgeVerletzt {
                zusatztaste: "shift".to_owned(),
                hinter: "cmd".to_owned(),
            })
        );
    }

    #[test]
    fn fn_wird_als_zusatztaste_abgewiesen() {
        assert_eq!(
            Kombination::lesen("fn+f3"),
            Err(Schreibfehler::FnAlsZusatztaste)
        );
    }

    #[test]
    fn eine_doppelte_zusatztaste_ist_ein_fehler() {
        assert_eq!(
            Kombination::lesen("cmd+cmd+k"),
            Err(Schreibfehler::ZusatztasteDoppelt("cmd".to_owned()))
        );
    }

    #[test]
    fn eine_fehlende_oder_unbekannte_taste_ist_ein_fehler() {
        assert_eq!(Kombination::lesen(""), Err(Schreibfehler::LeereTaste));
        assert_eq!(Kombination::lesen("cmd+"), Err(Schreibfehler::LeereTaste));
        // `arrowleft` ist die Schreibweise anderer Systeme fuer die Taste, die
        // hier `left` heisst. Als Beispiel taugt der Name dauerhaft: die
        // Tabelle darf ihn nie aufnehmen, weil er den Tastencode 123 braeuchte,
        // den `left` schon haelt, und
        // `jeder_name_und_jeder_code_steht_genau_einmal` laesst keine zwei
        // Eintraege auf denselben Code. Ein Name aus einer noch fehlenden
        // Gruppe, etwa `f13` oder ein Satzzeichen, waere hier falsch: die
        // naechste Erweiterung holte ihn ein.
        assert_eq!(
            Kombination::lesen("cmd+arrowleft"),
            Err(Schreibfehler::UnbekannterTastenname("arrowleft".to_owned()))
        );
        assert_eq!(
            Kombination::lesen("meta+k"),
            Err(Schreibfehler::UnbekannteZusatztaste("meta".to_owned()))
        );
    }

    #[test]
    fn jede_taste_der_tabelle_ueberlebt_lesen_und_schreiben() {
        let masken = [
            ModMaske::LEER,
            ModMaske::BEFEHL,
            ModMaske::UMSCHALT | ModMaske::BEFEHL,
            ModMaske::STEUERUNG | ModMaske::WAHL | ModMaske::UMSCHALT | ModMaske::BEFEHL,
        ];
        for taste in TASTEN {
            for maske in masken {
                let kombination = Kombination::neu(taste, maske);
                let geschrieben = kombination.to_string();
                assert_eq!(
                    Kombination::lesen(&geschrieben),
                    Ok(kombination),
                    "{geschrieben} laesst sich nicht wieder einlesen"
                );
            }
        }
    }

    #[test]
    fn eine_taste_ohne_namen_ergibt_keine_kombination() {
        // Tastencode 10 ist auf einer deutschen Tastatur die Taste links neben
        // der 1; die Schreibweise kennt keinen Namen dafuer.
        let druck = Tastendruck::neu(10, ModMaske::LEER);
        assert_eq!(Kombination::aus_tastendruck(druck), None);
    }

    /// Die Belegungsansicht schreibt auf, was auf der Taste steht, und nicht,
    /// wo sie liegt.
    #[test]
    fn ein_druck_wird_ueber_sein_zeichen_zur_kombination_und_nicht_ueber_seine_stelle() {
        // Eine deutsche Tastatur: die Taste mit der Aufschrift Y liegt auf der
        // Stelle `kVK_ANSI_Z` (Code 6) und meldet ein `y`.
        let druck = Tastendruck::aus_ereignis(
            code_von_pflicht("z"),
            Some('y'),
            super::super::normalisierung::roh::BEFEHL,
        );
        let Some(kombination) = Kombination::aus_tastendruck(druck) else {
            panic!("der Tastendruck ergibt keine Kombination");
        };
        assert_eq!(kombination.to_string(), "cmd+y");
    }

    /// Ein Tastendruck ohne brauchbares Zeichen darf nicht ueber seine Stelle
    /// bei einer Buchstabentaste landen.
    #[test]
    fn eine_stelle_ohne_zeichen_findet_keine_buchstabentaste() {
        // Auf einer franzoesischen Tastatur meldet die Stelle `kVK_ANSI_M`
        // (Code 46) ein Komma; die Taste mit der Aufschrift M liegt anderswo.
        let druck = Tastendruck::aus_ereignis(code_von_pflicht("m"), Some(','), 0);
        assert_eq!(Kombination::aus_tastendruck(druck), None);
    }
}
