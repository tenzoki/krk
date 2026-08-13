//! Die Menueleiste als Wert, ohne AppKit (C2 der Runde 7).
//!
//! **Keine Zeile AppKit.** Dieses Modul liegt neben `appkit` und nicht darin;
//! es nennt keine `objc2`-Kiste. Was hier entsteht, ist die **Beschreibung**
//! der Leiste: welche Obermenues es gibt, wie sie heissen, welche Eintraege sie
//! tragen, welche Beschriftung und welche Kombination jeder Eintrag bekommt und
//! wer ihn ausfuehrt. [`super::appkit::menue`] setzt diese Beschreibung danach
//! in `NSMenu` und `NSMenuItem` um und tut sonst nichts mehr.
//!
//! ```text
//!   Belegung ──> belegungsmodell::nach_bereichen ──> aufbau() ──> Vec<Obermenue>
//!                (die eine Gliederung,                             │
//!                 dritter Abnehmer)                                ▼
//!                                                        appkit::menue::hauptmenue
//! ```
//!
//! # Was dieser Zuschnitt kauft
//!
//! Die Menueleiste traegt nach dieser Runde einen Eintrag je Funktion der
//! Belegung statt der zehn von vorher, und ohne
//! dieses Modul waeren sie samt Reihenfolge, Titeln und Kuerzeln allein am
//! laufenden Buendel nachzusehen. `krk-ui` hat kein Bibliotheksziel, und eine
//! Probe, die ein `NSMenu` baut, braucht den Hauptfaden, den `libtest` nicht
//! hergibt. Ein Wert dagegen ist eine gewoehnliche Probe: die Kriterien C2.1 bis
//! C2.4 und C2.9 laufen hier unten ohne Fenster durch.
//!
//! # Eine Gliederung, drei Abnehmer
//!
//! [`aufbau`] fragt [`belegungsmodell::nach_bereichen`] und keine eigene
//! Ordnung. Es ist damit deren **dritter** Abnehmer neben der Belegungsansicht
//! und der Markdown-Ausgabe der Runde 3 (C2.2). Je besetztem
//! [`Funktionsbereich`] entsteht ein Obermenue, in der Reihenfolge von
//! [`Funktionsbereich::ALLE`], mit dem Titel aus
//! [`Funktionsbereich::name`] (C2.3); innerhalb eines Bereichs bleibt die
//! Reihenfolge von `resources/default-keymap.toml` erhalten.
//!
//! Dass diese Folge mit „Anwendung" beginnt und mit „Fenster" endet und dass
//! das Obermenue der Textbefehle „Bearbeiten" heisst, ist die Leistung von
//! [`Funktionsbereich`] selbst und keine Tabelle hier. Der Grund steht dort;
//! eine zweite Ordnung neben ihr waere genau die Verdopplung, die diese Runde
//! einspart.
//!
//! # Drei Sorten Eintrag, und wer sie ausfuehrt
//!
//! - **[`Eintrag::Befehl`]** traegt ein [`Kommando`] und laeuft ueber
//!   `Anwendungsdelegierter::kommando_ausfuehren`, also ueber denselben einen
//!   Ausfuehrungsweg wie der Tastendruck (C2.14). Sein Kommando steht am
//!   `NSMenuItem` im `tag`; die Uebersetzung dorthin und zurueck macht
//!   [`super::appkit::menue`] und nicht dieses Modul, denn `tag` ist ein
//!   AppKit-Begriff.
//! - **[`Eintrag::Textbefehl`]** traegt statt eines Kommandos den Namen eines
//!   AppKit-Selektors und laeuft ueber die Antwortkette (C2.8). Es sind genau
//!   die sechs Funktionen mit `gehalten_von = "menue"`, und die Zuordnung ihrer
//!   Kennungen zu den Selektoren steht als [`ZUSTELLER`] an dieser einen
//!   Stelle; sie hat bis zur Runde 7 im Programmtext von `hauptmenue` gestanden
//!   und steht dort nicht mehr daneben.
//! - **[`Eintrag::Sonderposten`]** ist ein benannter Zusatz, der in keiner
//!   Belegung steht. Es sind zwei, beide im Anwendungsmenue und beide bewusst
//!   ohne Kennung und ohne Kuerzel: „Über KRK" ganz oben, das den
//!   Standard-Ueber-Dialog von AppKit oeffnet (C5.1), und „Tastenbelegung
//!   als Markdown sichern" ueber dem Beenden (C2.9). Beide tragen einen
//!   Selektor, und **beantwortet werden sie nicht an derselben Stelle**: der
//!   der Markdown-Ausgabe beim Anwendungsdelegierten, der des Ueber-Eintrags
//!   eine Station davor bei `NSApplication`.
//!
//! Dazu [`Eintrag::Trenner`]. Die Aufzaehlung ist **vollstaendig und ohne
//! Auffangzweig**: eine neue Sorte haelt den Bau in der Umsetzung an, statt
//! dort stillschweigend zu verschwinden.
//!
//! # Zwei Funktionen auf einer Kombination, und nur ein Kuerzel
//!
//! Eine Menueleiste vertraegt dieselbe Tastenentsprechung nicht zweimal: AppKit
//! nimmt sie dem spaeter stehenden Eintrag still weg. Ausgeliefert faellt das an
//! genau einer Kombination an, `cmd+a`. Wer sie behaelt und warum, steht an
//! [`zugestellte_kuerzel`]; kurz: der Zusteller, weil ein Befehl von KRK
//! ohnehin ueber den Ereignisabgriff erreichbar bleibt und eine zugestellte
//! Funktion nicht.
//!
//! # Was diese Runde am sichtbaren Menue aendert, ohne es zu wollen
//!
//! Die Reihenfolge innerhalb eines Obermenues kommt jetzt aus der
//! Belegungsdatei. Im Menue „Bearbeiten" stehen „Rueckgaengig" und
//! „Wiederholen" deshalb **unter** den vier Zwischenablage-Befehlen und nicht
//! mehr darueber, und der Trenner zwischen beiden Gruppen faellt weg — das
//! Modell fuehrt allein die zwei Trenner des Anwendungsmenues, den unter dem
//! Ueber-Eintrag und den ueber dem Beenden. Beides ist die
//! Folge davon, dass die Gliederung die eine Quelle ist, und beides ist am
//! billigsten in `resources/default-keymap.toml` zu beheben, nicht hier.
//! Gemeldet als
//! `issues/260813-0420_*_das-menue-bearbeiten-verliert-seine-mac-uebliche-reihenfolge-und-seinen-trenner.md`.

use core::ffi::CStr;

use krk_core::tasten::{Belegung, Funktion, Kombination, Kommando};

use crate::belegungsmodell::{self, Funktionsbereich};

/// Die Kennung des Befehls, ueber dem der Markdown-Eintrag steht.
///
/// „Beenden" liegt auf dem Mac unten im Anwendungsmenue; der Sonderposten und
/// sein Trenner schieben sich davor. Fuehrt die Belegung diese Kennung nicht,
/// haengt der Sonderposten ans Ende des Anwendungsmenues, statt zu
/// verschwinden.
const BEENDEN: &str = "beenden";

/// Die Beschriftung des Markdown-Sonderpostens (C1 der Runde 3).
const MARKDOWN_BESCHRIFTUNG: &str = "Tastenbelegung als Markdown sichern";

/// Der Selektor, den der Anwendungsdelegierte fuer den Markdown-Sonderposten
/// fuehrt.
const MARKDOWN_SELEKTOR: &CStr = c"tastenbelegungSichern:";

/// Die Beschriftung des Ueber-Sonderpostens (C5.1).
///
/// Die Mac-Gewohnheit setzt diesen Eintrag ganz oben ins Anwendungsmenue, so
/// wie sie das Beenden nach unten setzt.
const UEBER_BESCHRIFTUNG: &str = "Über KRK";

/// Der Selektor, den die Antwortkette fuer den Ueber-Sonderposten beantwortet.
///
/// **Er steht an `NSApplication` und nicht am Anwendungsdelegierten.** AppKit
/// baut daraus den Standard-Ueber-Dialog und liest, was darin steht, aus der
/// `Info.plist` des Buendels; KRK stellt weder eine eigene Flaeche noch einen
/// eigenen Inhalt daneben (C5.3, C5.4).
const UEBER_SELEKTOR: &CStr = c"orderFrontStandardAboutPanel:";

/// Die sechs vom Menue zugestellten Textbefehle mit ihrem AppKit-Selektor.
///
/// **Die eine Zuordnung, und sie ist ein Wert und kein Programmtext.** Bis zur
/// Runde 7 stand sie als sechs `sel!`-Literale mitten im Aufbau des Menues und
/// war damit weder aufzaehlbar noch ohne Fenster pruefbar.
///
/// Die sechs sind genau die Funktionen, die `resources/default-keymap.toml` mit
/// `gehalten_von = "menue"` fuehrt und die deshalb kein [`Kommando`] tragen: wo
/// sie wirken, entscheidet die Antwortkette von AppKit und nicht KRK. Welche
/// Klasse jeden von ihnen beantwortet, ist gemessen und steht im Modulkopf von
/// [`super::appkit::menue`].
const ZUSTELLER: [(&str, &CStr); 6] = [
    ("text_ausschneiden", c"cut:"),
    ("text_kopieren", c"copy:"),
    ("text_einfuegen", c"paste:"),
    ("text_alles_auswaehlen", c"selectAll:"),
    ("text_rueckgaengig", c"undo:"),
    ("text_wiederholen", c"redo:"),
];

/// Ein Obermenue der Leiste: ein [`Funktionsbereich`] mit seinen Eintraegen.
///
/// Der Titel des **ersten** wirkt nicht: macOS ersetzt ihn durch den Namen aus
/// der `Info.plist`. Er steht trotzdem da, weil er in der Belegungsansicht und
/// in der Markdown-Ausgabe wirkt und diese Runde keine zweite Namensliste
/// aufmacht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obermenue<'a> {
    /// Die Aufschrift, aus [`Funktionsbereich::name`].
    pub titel: &'static str,
    /// Die Eintraege, in der Reihenfolge der Belegungsdatei.
    pub eintraege: Vec<Eintrag<'a>>,
}

/// Ein Eintrag eines Obermenues.
///
/// **Vollstaendig und ohne Auffangzweig.** Die Umsetzung in
/// [`super::appkit::menue`] zaehlt jede Sorte auf; eine neue haelt dort den Bau
/// an und erzwingt eine bewusste Einordnung, statt still nicht im Menue zu
/// erscheinen.
///
/// Die Zeichenketten sind geliehen und nicht kopiert: sie stehen in der
/// [`Belegung`], die [`aufbau`] gereicht bekommt, und leben laenger als das
/// Modell. Der Aufbau liegt auf dem Startpfad und damit in der Messstrecke von
/// L4; 162 Zeichenketten je Start waeren dort Arbeit ohne Gegenwert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Eintrag<'a> {
    /// Ein Befehl von KRK. Sein [`Kommando`] laeuft ueber
    /// `kommando_ausfuehren`, also ueber den einen Ausfuehrungsweg (C2.14).
    ///
    /// `kommando` ist `None` fuer eine benannte Funktion, die diese Runde noch
    /// nicht ausfuehrt. Der Eintrag steht dann ohne Aktion da und ist grau —
    /// eine ehrliche Auskunft, und keine, die den Nutzer ins Leere greifen
    /// laesst. In der Auslieferungsbelegung kommt der Fall nicht vor.
    Befehl {
        /// Die Aufschrift, aus [`Funktion::name`].
        beschriftung: &'a str,
        /// Die Kennung der Funktion, aus [`Funktion::kennung`].
        kennung: &'a str,
        /// Die **erste** Kombination der Funktion, oder keine (C2.4).
        kombination: Option<Kombination>,
        /// Das Kommando, falls diese Runde es ausfuehrt.
        kommando: Option<Kommando>,
    },
    /// Einer der sechs zugestellten Textbefehle: AppKit-Selektor statt
    /// Kommando, Antwortkette statt `kommando_ausfuehren` (C2.8).
    Textbefehl {
        /// Die Aufschrift, aus [`Funktion::name`].
        beschriftung: &'a str,
        /// Die Kennung der Funktion, aus [`Funktion::kennung`].
        kennung: &'a str,
        /// Die **erste** Kombination der Funktion, oder keine (C2.4).
        kombination: Option<Kombination>,
        /// Der Selektor aus [`ZUSTELLER`].
        selektor: &'static CStr,
    },
    /// Ein benannter Zusatz, der in keiner Belegung steht: der Ueber-Eintrag
    /// (C5.1) und der Eintrag der Markdown-Ausgabe (C2.9). Ohne Kennung und
    /// ohne Kuerzel.
    Sonderposten {
        /// Die Aufschrift.
        beschriftung: &'static str,
        /// Der Selektor, den die Antwortkette beantwortet.
        ///
        /// **Nicht notwendig der Anwendungsdelegierte**, und bis zur
        /// Titelleisten-Runde stand hier genau das. `tastenbelegungSichern:`
        /// erreicht ihn, weil die Kette bei ihm endet;
        /// `orderFrontStandardAboutPanel:` steht an `NSApplication` und wird
        /// eine Station davor beantwortet. Dieses Feld sagt, **was** gerufen
        /// wird, und nicht, wer antwortet.
        selektor: &'static CStr,
    },
    /// Eine Trennlinie.
    Trenner,
}

/// Die ganze Menueleiste aus einer Belegung.
///
/// **Eine reine Rechnung.** Sie liest die Belegung und sonst nichts; dieselbe
/// Belegung liefert dieselbe Leiste. Aufgerufen wird sie an den zwei Stellen,
/// die das Menue bauen — beim Start und nach einer Aenderung in der
/// Belegungsansicht (C2.11) —, und zwar aus [`super::appkit::menue::hauptmenue`].
///
/// Jede Funktion der Belegung bekommt genau einen Eintrag (C2.1); die Zahl
/// steht nirgends im Programmtext, sie ergibt sich aus der Belegung. Ein
/// Bereich ohne Funktion bekommt kein Obermenue — in der Auslieferungsbelegung
/// ist jeder besetzt.
pub fn aufbau(belegung: &Belegung) -> Vec<Obermenue<'_>> {
    let funktionen = belegung.funktionen();
    let zugestellt = zugestellte_kuerzel(belegung);
    belegungsmodell::nach_bereichen(belegung)
        .into_iter()
        .map(|(bereich, stellen)| {
            let mut eintraege: Vec<Eintrag<'_>> = stellen
                .into_iter()
                .map(|stelle| eintrag(&funktionen[stelle], &zugestellt))
                .collect();
            if bereich == Funktionsbereich::Anwendung {
                ueber_eintrag_einfuegen(&mut eintraege);
                markdownausgabe_einfuegen(&mut eintraege);
            }
            Obermenue {
                titel: bereich.name(),
                eintraege,
            }
        })
        .collect()
}

/// Der Eintrag zu einer Funktion.
///
/// **Die Frage lautet, wer den Befehl ausfuehrt, und
/// [`Funktion::kommando`] beantwortet sie schon.** Es liefert `None` fuer jede
/// zugestellte Funktion, denn was das Hauptmenue zustellt, fuehrt die
/// Antwortkette aus. Erst danach fragt [`ZUSTELLER`] nach dem Selektor; eine
/// zweite Stelle, die „ist das ein Textbefehl" entscheidet, entsteht damit
/// nicht.
fn eintrag<'a>(funktion: &'a Funktion, zugestellt: &[Kombination]) -> Eintrag<'a> {
    let beschriftung = funktion.name();
    let kennung = funktion.kennung();
    // C2.4: mehrere Kombinationen zeigen die erste, keine zeigt keine. Ein
    // `NSMenuItem` haelt genau eine Tastenentsprechung; der zweite Weg bleibt
    // ueber den Ereignisabgriff erreichbar.
    let kombination = funktion.tasten().first().copied();
    // Der Zusteller hat den Vortritt; der Grund steht an
    // [`zugestellte_kuerzel`]. **Der Filter steht vor der Fallunterscheidung
    // und nicht in einem ihrer Zweige**: er gilt fuer beide Sorten von
    // `Eintrag::Befehl` gleich, die mit Kommando und die ohne. Bis zur Runde 7
    // filterte allein der Kommandozweig, und eine benannte Funktion ohne
    // Kommando haette dieselbe Kombination ungefiltert in die Leiste getragen
    // (`issues/260813-0540_*_der-kuerzelfilter-des-menuemodells-greift-nur-am-kommandozweig.md`).
    // Der Textbefehlszweig unten behaelt die ungefilterte Kombination, denn er
    // **ist** der Zusteller.
    let eigenes = kombination.filter(|k| !zugestellt.contains(k));
    match funktion.kommando() {
        Some(kommando) => Eintrag::Befehl {
            beschriftung,
            kennung,
            kombination: eigenes,
            kommando: Some(kommando),
        },
        None => match zusteller(kennung) {
            Some(selektor) => Eintrag::Textbefehl {
                beschriftung,
                kennung,
                kombination,
                selektor,
            },
            None => Eintrag::Befehl {
                beschriftung,
                kennung,
                kombination: eigenes,
                kommando: None,
            },
        },
    }
}

/// Die Kombinationen, die eine zugestellte Funktion als Menuekuerzel traegt.
///
/// **Zwei Menueeintraege mit demselben Kuerzel vertraegt eine Menueleiste
/// nicht, und AppKit entscheidet den Streit still.** Am 260813 ueber
/// `--menue-protokoll` gemessen: tragen zwei Eintraege dieselbe
/// Tastenentsprechung, behaelt der **frueher** stehende sie, und dem spaeteren
/// wird das Zeichen entfernt — die Maske bleibt, das Zeichen ist leer. Bis zur
/// Runde 7 fiel das nicht an, weil das Menue zehn Eintraege trug und die
/// Doppelung nicht darunter war.
///
/// **Ausgeliefert gibt es genau einen solchen Fall, und der Entscheid vom
/// 260805 hat ihn ausdruecklich erlaubt**: `cmd+a` markiert im Dateifenster
/// ueber `alle_markieren` alle Eintraege und waehlt im Textfeld ueber
/// `text_alles_auswaehlen` den Text aus. Die Begruendung dort lautete, zwei
/// Funktionen mit verschiedenen Zustellern begegneten einander nie. In der
/// Belegungsdatei stimmt das weiterhin; in der Menueleiste stimmt es seit dieser
/// Runde nicht mehr.
///
/// **Das Kuerzel bekommt der Zusteller, und das ist kein Muenzwurf.** Ein
/// Befehl von KRK braucht sein Menuekuerzel nicht: der Ereignisabgriff sieht
/// jeden Tastendruck **vor** dem Menue und fuehrt ihn aus, wo er zulaessig ist.
/// Eine zugestellte Funktion hat keinen zweiten Weg — `Belegung::nachschlag`
/// ueberspringt sie, und ohne Menuekuerzel erreicht `cmd+a` den Feldeditor auf
/// keinem Weg (gemessen am 260804-1309,
/// `issues/260804-1309_*_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`).
/// Andersherum entschieden verloere der Nutzer das Auswaehlen im Textfeld; so
/// entschieden verliert er nichts als die **Anzeige** des Kuerzels an einem
/// Eintrag, der es weiterhin annimmt.
///
/// Der Preis steht damit im Menue: „Alle Eintraege markieren" zeigt kein
/// `Cmd+A`, obwohl `Cmd+A` es ausloest. Die Belegungsansicht und die
/// Markdown-Ausgabe zeigen die Kombination unveraendert; sie fragen die
/// Belegung und nicht diese Datei. Der Datensatz dazu ist
/// `decisions/260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`.
fn zugestellte_kuerzel(belegung: &Belegung) -> Vec<Kombination> {
    belegung
        .funktionen()
        .iter()
        .filter(|funktion| zusteller(funktion.kennung()).is_some())
        .filter_map(|funktion| funktion.tasten().first().copied())
        .collect()
}

/// Der AppKit-Selektor zu einer zugestellten Kennung, falls sie eine ist.
fn zusteller(kennung: &str) -> Option<&'static CStr> {
    ZUSTELLER
        .into_iter()
        .find(|(gefuehrt, _)| *gefuehrt == kennung)
        .map(|(_, selektor)| selektor)
}

/// Stellt den Ueber-Eintrag samt Trenner an den Anfang des Anwendungsmenues
/// (C5.1).
///
/// **An den Anfang, gespiegelt zu [`markdownausgabe_einfuegen`]:** das Beenden
/// liegt auf dem Mac unten im Anwendungsmenue, der Ueber-Eintrag ganz oben.
/// Eine Kennung, an der die Stelle sich festmachen muesste, braucht es dafuer
/// nicht — der Anfang der Liste ist der Anfang, ob sie nun leer ist oder
/// nicht.
///
/// Ein Kuerzel traegt der Eintrag nicht (C5.2). Es waere nach dem
/// Nutzerentscheid vom 260805-0000 zwingend ein Belegungseintrag geworden und
/// haette die Bauart geaendert, nicht nur die Bequemlichkeit; der Nutzer hat
/// den Eintrag am 260813-1010 ausdruecklich ohne Kuerzel bestellt.
fn ueber_eintrag_einfuegen(eintraege: &mut Vec<Eintrag<'_>>) {
    let zusatz = [
        Eintrag::Sonderposten {
            beschriftung: UEBER_BESCHRIFTUNG,
            selektor: UEBER_SELEKTOR,
        },
        Eintrag::Trenner,
    ];
    eintraege.splice(0..0, zusatz);
}

/// Schiebt den Markdown-Eintrag samt Trenner ueber das Beenden (C2.9).
///
/// Der Nutzer hat den Eintrag am 260811-0110 ausdruecklich ohne Kuerzel
/// bestellt; ein Kuerzel waere nach dem Entscheid vom 260805-0000 zwingend ein
/// Belegungseintrag geworden und haette damit die Bauart geaendert, nicht nur
/// die Bequemlichkeit. Deshalb steht er hier als Sonderposten und nicht in
/// `resources/default-keymap.toml`.
fn markdownausgabe_einfuegen(eintraege: &mut Vec<Eintrag<'_>>) {
    let sonderposten = [
        Eintrag::Sonderposten {
            beschriftung: MARKDOWN_BESCHRIFTUNG,
            selektor: MARKDOWN_SELEKTOR,
        },
        Eintrag::Trenner,
    ];
    let stelle = eintraege
        .iter()
        .position(ist_beenden)
        .unwrap_or(eintraege.len());
    eintraege.splice(stelle..stelle, sonderposten);
}

/// Ob dieser Eintrag das Beenden ist.
fn ist_beenden(eintrag: &Eintrag<'_>) -> bool {
    matches!(eintrag, Eintrag::Befehl { kennung, .. } if *kennung == BEENDEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Kennung jedes Eintrags, der eine Funktion vertritt, in der
    /// Reihenfolge der Leiste.
    fn kennungen(leiste: &[Obermenue<'_>]) -> Vec<String> {
        leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
            .filter_map(|eintrag| match eintrag {
                Eintrag::Befehl { kennung, .. } | Eintrag::Textbefehl { kennung, .. } => {
                    Some((*kennung).to_owned())
                }
                Eintrag::Sonderposten { .. } | Eintrag::Trenner => None,
            })
            .collect()
    }

    /// Die Eintraege eines Obermenues mit diesem Titel.
    fn unter<'a, 'b>(leiste: &'a [Obermenue<'b>], titel: &str) -> &'a [Eintrag<'b>] {
        leiste
            .iter()
            .find(|obermenue| obermenue.titel == titel)
            .unwrap_or_else(|| panic!("die Leiste fuehrt kein Obermenue {titel}"))
            .eintraege
            .as_slice()
    }

    // -----------------------------------------------------------------------
    // C2.1: jede Funktion genau einmal
    // -----------------------------------------------------------------------

    /// **Die Zahl steht nicht im Programmtext.** Sie wird gegen
    /// [`Belegung::funktionen`] gezaehlt, denn das Menue entsteht aus der
    /// Belegung; eine Zahl hier waere eine zweite Wahrheit darueber, wie viele
    /// Funktionen KRK hat, und veraltete mit der naechsten Zeile in
    /// `resources/default-keymap.toml`.
    #[test]
    fn jede_funktion_der_belegung_steht_genau_einmal_im_menue() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let mut gefunden = kennungen(&leiste);
        let mut erwartet: Vec<String> = belegung
            .funktionen()
            .iter()
            .map(|funktion| funktion.kennung().to_owned())
            .collect();

        assert_eq!(
            gefunden.len(),
            erwartet.len(),
            "das Menue fuehrt nicht so viele Befehlseintraege, wie die Belegung \
             Funktionen hat"
        );
        gefunden.sort();
        erwartet.sort();
        assert_eq!(
            gefunden, erwartet,
            "eine Funktion fehlt im Menue oder steht doppelt darin"
        );
    }

    /// Die sechs zugestellten Textbefehle sind genau die Eintraege ohne
    /// Kommando, und jeder traegt seinen gemessenen Selektor (C2.8).
    #[test]
    fn die_sechs_zugestellten_tragen_ihren_selektor_und_kein_kommando() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let mut gefunden: Vec<(&str, &CStr)> = leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
            .filter_map(|eintrag| match eintrag {
                Eintrag::Textbefehl {
                    kennung, selektor, ..
                } => Some((*kennung, *selektor)),
                _ => None,
            })
            .collect();
        let mut erwartet: Vec<(&str, &CStr)> = ZUSTELLER.into_iter().collect();
        gefunden.sort();
        erwartet.sort();
        assert_eq!(gefunden, erwartet);

        for eintrag in leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
        {
            if let Eintrag::Befehl {
                kennung, kommando, ..
            } = eintrag
            {
                assert!(
                    kommando.is_some(),
                    "{kennung} steht ohne Kommando und ohne Selektor im Menue und \
                     waere damit grau"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // C2.2: die eine Gliederung, drei Abnehmer
    // -----------------------------------------------------------------------

    /// Genau drei Dateien nennen die Gliederung, und das sind ihre drei
    /// Abnehmer.
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier, weil C2.2 die Zahl selbst
    /// zusagt.** Gezaehlt werden Dateien und nicht Fundstellen: ein Abnehmer,
    /// der die Gliederung zweimal in derselben Datei fragt — die
    /// Markdown-Ausgabe tut das in ihrem Pruefmodul —, ist derselbe Abnehmer.
    /// Rot wird die Probe, wenn ein vierter dazukommt, und die richtige Antwort
    /// darauf ist die Zahl hier und nicht das Streichen des Abnehmers.
    #[test]
    fn die_gliederung_hat_drei_abnehmer() {
        let nadel = concat!("nach_", "bereichen");
        let dateien: Vec<String> = crate::quellbaum::quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains(nadel))
            .map(|(name, _)| name)
            .collect();
        assert_eq!(
            dateien,
            [
                "krk-ui/src/belegungsausgabe.rs".to_owned(),
                "krk-ui/src/belegungsmodell.rs".to_owned(),
                "krk-ui/src/menuemodell.rs".to_owned(),
            ],
            "die eine Gliederung hat andere Abnehmer als die Belegungsansicht, \
             die Markdown-Ausgabe und das Hauptmenue"
        );
    }

    // -----------------------------------------------------------------------
    // C2.3: neun Obermenues, ihre Folge und ihre Titel
    // -----------------------------------------------------------------------

    /// Je besetztem Bereich ein Obermenue, in der Reihenfolge und mit den
    /// Namen von [`Funktionsbereich`].
    #[test]
    fn die_obermenues_folgen_der_gliederung() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let titel: Vec<&str> = leiste.iter().map(|obermenue| obermenue.titel).collect();
        let erwartet: Vec<&str> = Funktionsbereich::ALLE
            .iter()
            .map(|bereich| bereich.name())
            .collect();
        assert_eq!(
            titel, erwartet,
            "ab Werk ist jeder Bereich besetzt, also stehen alle neun in ihrer \
             Reihenfolge"
        );
    }

    /// Das erste Obermenue traegt den Anwendungsbereich, das letzte das
    /// Fenster.
    ///
    /// Dieselbe Zusage wie die Probe an [`Funktionsbereich::ALLE`], einmal am
    /// fertigen Wert nachgerechnet: die Gliederung koennte in der richtigen
    /// Folge stehen und der Aufbau sie trotzdem umsortieren.
    #[test]
    fn die_leiste_beginnt_mit_der_anwendung_und_endet_mit_dem_fenster() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        assert_eq!(
            leiste.first().map(|obermenue| obermenue.titel),
            Some(Funktionsbereich::Anwendung.name())
        );
        assert_eq!(
            leiste.last().map(|obermenue| obermenue.titel),
            Some(Funktionsbereich::Fenster.name())
        );
    }

    /// Innerhalb eines Obermenues bleibt die Reihenfolge der Belegungsdatei
    /// erhalten; eine zweite Sortierung entsteht nicht.
    #[test]
    fn innerhalb_eines_obermenues_bleibt_die_reihenfolge_der_datei() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let erwartet: Vec<String> = belegungsmodell::nach_bereichen(&belegung)
            .into_iter()
            .flat_map(|(_, stellen)| stellen)
            .map(|stelle| belegung.funktionen()[stelle].kennung().to_owned())
            .collect();
        assert_eq!(kennungen(&leiste), erwartet);
    }

    // -----------------------------------------------------------------------
    // C2.4: das Kuerzel kommt aus der Belegung
    // -----------------------------------------------------------------------

    /// Eine Funktion mit mehreren Kombinationen zeigt die erste, eine ohne
    /// zeigt keine.
    ///
    /// Gesucht werden beide Faelle in der Auslieferungsbelegung selbst, statt
    /// eine eigene Belegung zu bauen: die Zusage gilt fuer die Belegung, die
    /// KRK wirklich ausliefert.
    #[test]
    fn ein_eintrag_zeigt_die_erste_kombination_oder_keine() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let mut mit_mehreren = 0_usize;
        let mut ohne = 0_usize;

        for eintrag in leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
        {
            let (kennung, kombination) = match eintrag {
                Eintrag::Befehl {
                    kennung,
                    kombination,
                    ..
                }
                | Eintrag::Textbefehl {
                    kennung,
                    kombination,
                    ..
                } => (*kennung, *kombination),
                Eintrag::Sonderposten { .. } | Eintrag::Trenner => continue,
            };
            let tasten = belegung
                .funktion(kennung)
                .expect("jeder Eintrag kommt aus der Belegung")
                .tasten();
            let erste = tasten.first().copied();
            // Die eine Ausnahme: ein Befehl, dessen erste Kombination schon ein
            // Zusteller als Kuerzel traegt, zeigt keines. Die Probe daneben
            // haelt die Ausnahme fest.
            let abgetreten = erste.is_some_and(|erste| {
                zugestellte_kuerzel(&belegung).contains(&erste) && zusteller(kennung).is_none()
            });
            assert_eq!(
                kombination,
                if abgetreten { None } else { erste },
                "{kennung} zeigt nicht die erste Kombination"
            );
            if tasten.len() > 1 {
                mit_mehreren += 1;
            }
            if tasten.is_empty() {
                ohne += 1;
            }
        }

        assert!(
            mit_mehreren > 0,
            "die Auslieferungsbelegung fuehrt keine Funktion mit zwei \
             Kombinationen mehr; die erste Haelfte von C2.4 ist damit ungeprueft"
        );
        assert!(
            ohne > 0,
            "die Auslieferungsbelegung fuehrt keine unbelegte Funktion mehr; die \
             zweite Haelfte von C2.4 ist damit ungeprueft"
        );
    }

    /// Teilen sich zwei Funktionen eine Kombination, traegt der Zusteller sie
    /// im Menue.
    ///
    /// **Der Fall ist am 260813 an der Auslieferungsbelegung gemessen und nicht
    /// hergeleitet**: `--menue-protokoll` zeigte „Alles auswaehlen" ohne Zeichen
    /// und mit Befehlstaste in der Maske, weil „Alle Eintraege markieren" weiter
    /// vorn in der Leiste dieselbe Entsprechung trug und AppKit sie dem
    /// spaeteren still nimmt. Ohne diese Regel erreichte `cmd+a` den Feldeditor
    /// auf keinem Weg mehr, und C2.18 waere gebrochen.
    ///
    /// Die Probe sucht sich den Fall selbst und behauptet ihn nicht: sie
    /// vergleicht die Kuerzel der Zusteller gegen die der Befehle. Faellt die
    /// Doppelung eines Tages weg, faellt auch die Probe — deshalb prueft sie
    /// zuerst, dass es sie noch gibt.
    #[test]
    fn bei_einer_doppelten_kombination_traegt_der_zusteller_das_kuerzel() {
        let belegung = Belegung::auslieferung();
        let zugestellt = zugestellte_kuerzel(&belegung);
        let doppelt: Vec<&Funktion> = belegung
            .funktionen()
            .iter()
            .filter(|funktion| zusteller(funktion.kennung()).is_none())
            .filter(|funktion| {
                funktion
                    .tasten()
                    .first()
                    .is_some_and(|erste| zugestellt.contains(erste))
            })
            .collect();
        assert!(
            !doppelt.is_empty(),
            "keine Kombination steht mehr bei zwei Zustellern; die Regel an \
             zugestellte_kuerzel hat keinen Fall mehr und gehoert geprueft"
        );

        let leiste = aufbau(&belegung);
        for funktion in doppelt {
            let eintrag = leiste
                .iter()
                .flat_map(|obermenue| obermenue.eintraege.iter())
                .find(|eintrag| {
                    matches!(eintrag, Eintrag::Befehl { kennung, .. }
                        if *kennung == funktion.kennung())
                })
                .expect("jede Funktion steht im Menue");
            assert!(
                matches!(
                    eintrag,
                    Eintrag::Befehl {
                        kombination: None,
                        ..
                    }
                ),
                "{} traegt sein Kuerzel, obwohl ein Zusteller es beansprucht: \
                 {eintrag:?}",
                funktion.kennung()
            );
        }
    }

    /// Keine zwei Eintraege der Leiste tragen dieselbe Kombination.
    ///
    /// **Die eigentliche Zusage, und die Regel darueber ist nur ihr Mittel.**
    /// Zwei gleiche Tastenentsprechungen in einer Leiste entscheidet AppKit
    /// still nach der Stellung; welcher Eintrag dabei verliert, haengt dann an
    /// der Reihenfolge der Belegungsdatei und nicht an einer Ueberlegung.
    #[test]
    fn keine_zwei_eintraege_tragen_dieselbe_kombination() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let mut gesehen: Vec<(Kombination, &str)> = Vec::new();
        for eintrag in leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
        {
            let (kennung, Some(kombination)) = (match eintrag {
                Eintrag::Befehl {
                    kennung,
                    kombination,
                    ..
                }
                | Eintrag::Textbefehl {
                    kennung,
                    kombination,
                    ..
                } => (*kennung, *kombination),
                Eintrag::Sonderposten { .. } | Eintrag::Trenner => continue,
            }) else {
                continue;
            };
            if let Some((_, andere)) = gesehen
                .iter()
                .find(|(gefuehrt, _)| *gefuehrt == kombination)
            {
                panic!("{kennung} und {andere} tragen beide {kombination} im Menue");
            }
            gesehen.push((kombination, kennung));
        }
    }

    // -----------------------------------------------------------------------
    // C2.9 und C5.1: die zwei Sonderposten
    // -----------------------------------------------------------------------

    /// Der Markdown-Eintrag steht im Anwendungsmenue, ohne Kuerzel, unmittelbar
    /// ueber dem Beenden und durch einen Trenner davon geschieden.
    #[test]
    fn der_markdown_eintrag_steht_ueber_dem_beenden() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let eintraege = unter(&leiste, Funktionsbereich::Anwendung.name());
        let stelle = eintraege
            .iter()
            .position(|eintrag| {
                matches!(eintrag, Eintrag::Sonderposten { beschriftung, .. }
                    if *beschriftung == MARKDOWN_BESCHRIFTUNG)
            })
            .expect("der Markdown-Eintrag steht im Anwendungsmenue");

        assert!(
            matches!(eintraege.get(stelle + 1), Some(Eintrag::Trenner)),
            "unter dem Markdown-Eintrag steht kein Trenner"
        );
        assert!(
            matches!(eintraege.get(stelle + 2), Some(Eintrag::Befehl { kennung, .. })
                if *kennung == BEENDEN),
            "unter dem Trenner steht nicht das Beenden"
        );
    }

    /// „Über KRK" steht als **erster** Eintrag des Anwendungsmenues,
    /// unmittelbar gefolgt von einem Trenner, und traegt den Selektor des
    /// Standard-Ueber-Dialogs (C5.1, C5.3).
    ///
    /// **Die Stelle wird relativ geprueft**, wie in
    /// [`der_markdown_eintrag_steht_ueber_dem_beenden`]: gesucht wird der
    /// Eintrag, gemessen wird seine Umgebung. Ein fester Index waere eine
    /// zweite Aussage darueber, wie viele Funktionen der Bereich
    /// „Anwendung" fuehrt, und wuerde rot, sobald jemand eine Zeile in
    /// `resources/default-keymap.toml` verschiebt, ohne dass an dieser Zusage
    /// etwas faul waere.
    ///
    /// **Der Selektorname steht hier ausgeschrieben und nicht als
    /// [`UEBER_SELEKTOR`]:** gegen die Konstante geprueft waere die Zusicherung
    /// eine Tautologie, und ein Tippfehler darin bliebe unbemerkt. Dass
    /// `NSApplication` ihn beantwortet, ist keine Aussage ueber diesen Wert und
    /// steht deshalb an der Konstanten und im Modulkopf von
    /// [`crate::appkit::menue`], nicht hier.
    #[test]
    fn der_ueber_eintrag_steht_ganz_oben() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let eintraege = unter(&leiste, Funktionsbereich::Anwendung.name());
        let (stelle, selektor) = eintraege
            .iter()
            .enumerate()
            .find_map(|(stelle, eintrag)| match eintrag {
                Eintrag::Sonderposten {
                    beschriftung,
                    selektor,
                } if *beschriftung == UEBER_BESCHRIFTUNG => Some((stelle, *selektor)),
                _ => None,
            })
            .expect("der Ueber-Eintrag steht im Anwendungsmenue");

        assert_eq!(
            stelle,
            0,
            "vor dem Ueber-Eintrag steht {:?}",
            &eintraege[..stelle]
        );
        assert!(
            matches!(eintraege.get(stelle + 1), Some(Eintrag::Trenner)),
            "unter dem Ueber-Eintrag steht kein Trenner"
        );
        assert_eq!(
            selektor.to_bytes(),
            b"orderFrontStandardAboutPanel:",
            "der Ueber-Eintrag traegt nicht den Selektor des Standard-Dialogs"
        );
    }

    /// Die ganze Leiste traegt genau zwei Sonderposten und genau zwei Trenner
    /// (C6.3).
    ///
    /// Ohne diese Zusage staende ein dritter Zusatz irgendwo, ohne dass eine
    /// Probe ihn benennt. Die Zahlen sind klein und ausdruecklich: waechst
    /// eine, gehoert der neue Zusatz hier genannt.
    ///
    /// **Gezaehlt werden beide Sorten getrennt**, denn eine Summe von vier
    /// bliebe auch dann stehen, wenn ein Sonderposten ohne seinen Trenner
    /// dazukaeme und ein anderer seinen verloere.
    #[test]
    fn die_leiste_traegt_zwei_sonderposten_und_zwei_trenner() {
        let belegung = Belegung::auslieferung();
        let leiste = aufbau(&belegung);
        let zusaetze: Vec<&Eintrag<'_>> = leiste
            .iter()
            .flat_map(|obermenue| obermenue.eintraege.iter())
            .filter(|eintrag| matches!(eintrag, Eintrag::Sonderposten { .. } | Eintrag::Trenner))
            .collect();
        let sonderposten = zusaetze
            .iter()
            .filter(|eintrag| matches!(eintrag, Eintrag::Sonderposten { .. }))
            .count();
        let trenner = zusaetze
            .iter()
            .filter(|eintrag| matches!(eintrag, Eintrag::Trenner))
            .count();
        assert_eq!(sonderposten, 2, "{zusaetze:?}");
        assert_eq!(trenner, 2, "{zusaetze:?}");
    }

    /// Der Sonderposten haengt ans Ende, wenn kein Beenden dasteht.
    ///
    /// Der Fall kommt in der Auslieferungsbelegung nicht vor, und deshalb wird
    /// hier die Einfuegung selbst gefragt und nicht der ganze Aufbau. Er steht
    /// ueberhaupt da, weil die Alternative waere, den Eintrag stillschweigend
    /// wegzulassen — und dann faende der Nutzer die Markdown-Ausgabe nirgends
    /// mehr.
    #[test]
    fn ohne_beenden_haengt_der_sonderposten_ans_ende() {
        let mut eintraege = vec![Eintrag::Befehl {
            beschriftung: "Tastaturbelegung anzeigen",
            kennung: "belegung_ansehen",
            kombination: None,
            kommando: None,
        }];
        markdownausgabe_einfuegen(&mut eintraege);
        assert!(
            matches!(
                eintraege.as_slice(),
                [_, Eintrag::Sonderposten { .. }, Eintrag::Trenner]
            ),
            "{eintraege:?}"
        );
    }

    // -----------------------------------------------------------------------
    // Die Zustellertabelle
    // -----------------------------------------------------------------------

    /// Jede Kennung aus [`ZUSTELLER`] steht in der Auslieferungsbelegung und
    /// traegt dort `gehalten_von = "menue"`.
    ///
    /// Ohne diese Zusage stuende eine Kennung in der Tabelle, die keine
    /// Funktion hat, und der zugehoerige Eintrag fehlte im Menue, ohne dass
    /// jemand es merkte.
    #[test]
    fn jede_zugestellte_kennung_steht_in_der_auslieferungsbelegung() {
        let belegung = Belegung::auslieferung();
        for (kennung, _) in ZUSTELLER {
            let Some(funktion) = belegung.funktion(kennung) else {
                panic!("die Auslieferungsbelegung kennt {kennung} nicht");
            };
            assert_eq!(
                funktion.gehalten_von(),
                Some("menue"),
                "{kennung} wird nicht vom Menue zugestellt"
            );
            assert!(
                funktion.kommando().is_none(),
                "{kennung} traegt ein Kommando und laeuft damit nicht ueber die \
                 Antwortkette"
            );
        }
    }

    /// Keine zwei Kennungen teilen sich einen Selektor, und keine steht
    /// doppelt.
    #[test]
    fn die_zustellertabelle_ist_eindeutig() {
        for (stelle, (kennung, selektor)) in ZUSTELLER.into_iter().enumerate() {
            for (andere, anderer) in ZUSTELLER.into_iter().skip(stelle + 1) {
                assert_ne!(kennung, andere, "{kennung} steht doppelt");
                assert_ne!(
                    selektor, anderer,
                    "{kennung} und {andere} teilen sich einen Selektor"
                );
            }
        }
    }
}
