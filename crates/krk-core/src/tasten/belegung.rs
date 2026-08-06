//! Die Belegungsmaschine: welche Taste welche Funktion ausloest.
//!
//! ```text
//! resources/default-keymap.toml ──include_str!──> AUSLIEFERUNG
//!                                                      │
//!            ~/Library/.../KRK/keymap.toml ──Ablage──> Belegung ──> Nachschlag
//! ```
//!
//! # Eine Tabelle, kein Nebeneinander
//!
//! Schritt 7 hatte fuenf Tasten fest verdrahtet, damit der Durchstich eine
//! Auswahl bewegen kann. Diese Tabelle ist mit Schritt 11 **abgeloest und nicht
//! ergaenzt**: es gibt genau einen Weg von einem Tastendruck zu einer Funktion,
//! und er beginnt in `resources/default-keymap.toml`. Zwei Nachschlagewege
//! nebeneinander waeren zwei Wahrheiten darueber, welche Taste was ausloest,
//! und die erste Abweichung zwischen ihnen faende keine Pruefung.
//!
//! Dasselbe gilt fuer die Tastencodes: sie stehen allein in
//! [`TASTEN`](super::parser::TASTEN).
//!
//! # Die Nutzerdatei ersetzt, sie ergaenzt nicht
//!
//! `keymap.toml` haelt die **vollstaendige** Belegung des Nutzers, nicht seine
//! Abweichungen vom Auslieferungszustand. Wer eine Zeile daraus loescht, hat die
//! Funktion unbelegt gemacht; wer die Datei loescht, bekommt beim naechsten
//! Start die Auslieferungsbelegung. Der Weg dorthin ist [`laden`], und er geht
//! ueber [`Ablage::laden`] aus Schritt 10: ein zweiter Ablageweg entsteht nicht,
//! und jede Meldung nimmt [`ablage::melden`](crate::ablage::melden).
//!
//! Eine Belegung des Nutzers wird gegen den **Wortschatz** der
//! Auslieferungsbelegung geprueft: sie darf jede Kombination frei verteilen,
//! aber nur auf Funktionen, die KRK kennt. Funktionen, die ihre Datei nicht
//! nennt, treten unbelegt hinzu, damit die Belegungsansicht aus C3 jede Funktion
//! auffuehren kann und der Nutzer sie wieder erreichbar machen kann.
//!
//! # Was ein Nachschlag antwortet
//!
//! Drei Faelle, siehe [`Nachschlag`]. Der dritte ist die Sprungmarke aus C2:
//! eine Taste **ohne** Zusatztaste, die keiner Funktion gehoert, gehoert dem
//! Tippen der Anfangsbuchstaben. Der Kern sagt nur, dass der Tastendruck dorthin
//! faellt; welches Zeichen er traegt, weiss allein die Oberflaeche, denn ein
//! Tastencode benennt eine Stelle auf der Tastatur und kein Zeichen.
//!
//! # Der Zusteller, und was er fuer den Konflikt bedeutet
//!
//! Seit Schritt 13b fuehrt die Belegung Funktionen, die nicht der
//! Ereignisabgriff ausfuehrt, sondern das Hauptmenue zustellt. Wer zustellt,
//! steht in [`Funktion::gehalten_von`]: ohne das Feld der Abgriff, mit dem Wert
//! `menue` das Hauptmenue. Der Fokusvorbehalt aus C2 teilt jeden Tastendruck
//! genau einem der beiden zu, eine Funktion ist deshalb in genau einem
//! Fokuszustand erreichbar.
//!
//! Daraus folgt die vollstaendige Regel, und sie ist eine Regel und keine
//! Ausnahme fuer eine einzelne Taste: **zwei Funktionen sind genau dann ein
//! Konflikt, wenn sie dieselbe Kombination tragen und denselben Zusteller
//! haben.** Sie greift an vier Stellen, und keine davon ist entbehrlich:
//!
//! | Stelle | Warum |
//! |---|---|
//! | [`Belegung::konflikte`] | jedes Einlesen laeuft ueber [`Belegung::bauen`] darauf |
//! | [`Belegung::zuweisen`] | die Umbelegung durch den Nutzer aus C3 |
//! | [`Belegung::nachschlag`] | der Abgriff darf nur sehen, was er selbst zustellt |
//! | [`Funktion::kommando`] | eine zugestellte Funktion fuehrt KRK nie selbst aus |
//!
//! Die dritte Stelle traegt die Regel erst: der Nachschlag liefert den ersten
//! Treffer, und ohne das Ueberspringen haenge das Verhalten an der Reihenfolge
//! der Eintraege. Stuende `text_alles_auswaehlen` in der Datei des Nutzers vor
//! `alle_markieren`, faende der Abgriff eine Funktion ohne Kommando, reichte den
//! Tastendruck weiter, und das Markieren aller Eintraege waere still tot.
//!
//! Nutzerentscheid vom 260805,
//! `decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`.
//!
//! # Der Wirkungsbereich: welcher Bereich den Fokus haben muss
//!
//! Seit Schritt 18 gibt es zwei fokussierbare Bereiche, die beiden
//! Dateifenster und die Lesezeichenleiste aus C5. Damit wird die Frage "darf
//! dieser Befehl hier ueberhaupt wirken" fuer **jedes** Kommando faellig:
//! `delete` darf in der Leiste keine Datei loeschen, `right` dort in keinen
//! Ordner einsteigen, und `lesezeichen_loeschen` umgekehrt nicht wirken,
//! solange der Fokus im Dateifenster steht.
//!
//! Die Antwort ist **eine Eigenschaft je Kommando** und keine Abfrage je
//! Aufrufstelle: [`Kommando::wirkungsbereich`]. Vier oder fuenf handgeschriebene
//! Vorbehalte an vier oder fuenf Stellen waeren das Dickicht aus Sonderregeln,
//! das die Maxime "supersimpel" ausschliesst; die Zuleitung in `krk-ui` fragt
//! die Eigenschaft **einmal**, bevor sie ein Kommando ausfuehrt. Die einzelne
//! Abfrage der Loeschtasten aus Schritt 16 ist darin aufgegangen.
//!
//! **Die Eigenschaft steht im Kern, die Antwort in der Oberflaeche.** Dass das
//! endgueltige Loeschen das Dateifenster braucht, ist eine Aussage ueber den
//! Befehl und kein AppKit-Wissen; sie ist deshalb hier ohne Fenster pruefbar.
//! Welcher Bereich den Fokus gerade hat, weiss allein `krk-ui`. Die
//! Aufrufrichtung bleibt von oben nach unten.

use std::fmt;
use std::io;
use std::path::PathBuf;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::ablage::{Ablage, Datei, Ersetzung, Geladen, Grund, melden};

use super::Tastendruck;
use super::konflikt::{Funktionsname, Konflikt};
use super::parser::{Kombination, Schreibfehler};

/// Die Auslieferungsbelegung, in das Programm einkompiliert.
///
/// Damit gibt es keinen Start ohne Belegung: eine fehlende, geloeschte oder
/// kaputte `keymap.toml` faellt immer auf diesen Text zurueck.
const AUSLIEFERUNGSTEXT: &str = include_str!("../../../../resources/default-keymap.toml");

/// Die gelesene Auslieferungsbelegung. Sie definiert den Wortschatz.
static AUSLIEFERUNG: LazyLock<Belegung> = LazyLock::new(|| {
    let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
        .expect("die eingebettete Auslieferungsbelegung ist kein gueltiges TOML");
    Belegung::bauen(&datei, None)
        .expect("die eingebettete Auslieferungsbelegung ist in sich nicht schluessig")
});

/// Welcher Bereich den Eingabefokus haben muss, damit ein Kommando wirkt (C5).
///
/// Vier Werte, und die Aufzaehlung ist mit ihnen vollstaendig: KRK hat seit
/// Schritt 19 drei fokussierbare Bereiche, die beiden Dateifenster, die Leiste
/// und das Vorschaufenster. Ein Befehl gehoert einem Dateifenster, der Leiste,
/// einem Bereich mit Tabs oder keinem. Der vierte Wert [`Wirkungsbereich::
/// Tabbereich`] ist mit dem Vorschaufenster aus C6 entstanden: die vier
/// Tabbefehle aus C1 bedienen nach C6 auch dessen Tabs, und zwar in dem
/// Bereich, der den Fokus gerade hat — so fuehrt es
/// `resources/default-keymap.toml` seit S9. Ein eigener Vorschau-Wert daneben
/// entsteht nicht, weil kein Befehl allein im Vorschaufenster wirkt.
///
/// **Der Vorbehalt ist stumm.** Ein Kommando, das hier scheitert, tut nichts
/// und meldet nichts; der Tastendruck geht unveraendert an AppKit weiter, wie
/// jeder unbelegte. Eine Meldung waere die Sonderregel, die die Maxime
/// "supersimpel" ausschliesst, und die drei Abnahmekriterien aus C5 verlangen
/// von `delete`, `right` und `lesezeichen_loeschen` ausdruecklich nur, dass sie
/// nichts tun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wirkungsbereich {
    /// Wirkt nur, wenn der Fokus in einem Dateifenster steht.
    Dateifenster,
    /// Wirkt nur, wenn der Fokus in der Lesezeichen- und Geraeteleiste steht
    /// (C5).
    Leiste,
    /// Wirkt, wenn der Fokus in einem Bereich mit Tabs steht: in einem
    /// Dateifenster oder im Vorschaufenster (C1, C6).
    ///
    /// Der Wert der vier Tabbefehle. C6 verlangt fuer die Vorschau-Tabs
    /// "dieselben Befehle zum Oeffnen, Schliessen und Wechseln wie in C1",
    /// und dieselben Befehle heisst: dieselben vier Kommandos, gerichtet an
    /// den Bereich vor dem Nutzer. Die Leiste traegt keine Tabs und bleibt
    /// aussen vor.
    Tabbereich,
    /// Wirkt ohne Vorbehalt.
    ///
    /// Zwei Sorten von Befehlen tragen ihn. Die einen gehoeren dem Fenster als
    /// ganzem und keinem Bereich darin: beenden, ein Fenster schliessen, einen
    /// Bereich ein- und ausblenden, den Fokus setzen. Die anderen bewegen die
    /// Auswahl und wirken deshalb dort, wo der Fokus gerade steht: beide
    /// Bereiche sind Listen mit einer Auswahl, und der Auf- und der Ab-Pfeil
    /// bewegen sie nach C2 wie nach C5.
    Ueberall,
}

/// Was ein Tastendruck im Dateifenster ausloest.
///
/// **Nicht der Wortschatz der Belegung.** Die Belegung kennt jede Funktion aus
/// C1 bis C7; diese Aufzaehlung kennt die, zu denen es in dieser Runde schon
/// eine Ausfuehrung gibt. Sie waechst mit den Schritten, die die uebrigen
/// Funktionen bauen. Die Bruecke zwischen beiden sind die Kennungen aus
/// `resources/default-keymap.toml`, und eine Pruefung haelt sie zusammen:
/// `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kommando {
    /// Die Auswahl einen Eintrag nach oben.
    AuswahlHoch,
    /// Die Auswahl einen Eintrag nach unten.
    AuswahlRunter,
    /// Die Auswahl eine Bildschirmseite nach oben.
    SeiteHoch,
    /// Die Auswahl eine Bildschirmseite nach unten.
    SeiteRunter,
    /// Die Auswahl an den Anfang der Liste (C2).
    Listenanfang,
    /// Die Auswahl an das Ende der Liste (C2).
    Listenende,
    /// In den ausgewaehlten Ordner hineinsteigen.
    Oeffnen,
    /// In den uebergeordneten Ordner aufsteigen (C2).
    OrdnerAufwaerts,
    /// Einen Pfad eingeben und dorthin springen (C2).
    Pfadeingabe,
    /// Den Eintrag unter der Auswahl markieren und weiterruecken (C2).
    MarkierungUmschalten,
    /// Alle Eintraege markieren (C2).
    AlleMarkieren,
    /// Jede Markierung aufheben (C2).
    MarkierungAufheben,
    /// Die Markierung umkehren (C2).
    MarkierungUmkehren,
    /// Nach Name sortieren (C2).
    SortierungName,
    /// Nach Groesse sortieren (C2).
    SortierungGroesse,
    /// Nach Aenderungsdatum sortieren (C2).
    SortierungDatum,
    /// Nach Typ sortieren (C2).
    SortierungTyp,
    /// Die Sortierrichtung umkehren (C2).
    SortierrichtungUmkehren,
    /// Versteckte Dateien ein- und ausblenden (C2).
    VersteckteUmschalten,
    /// Zu dem springen, was in der Zwischenablage steht (C10).
    ZwischenablageSpringen,
    /// Den Inhalt der Zwischenablage im Vorschaufenster ansehen (C10).
    ZwischenablageAnsehen,
    /// Einen neuen Tab im aktiven Dateifenster oeffnen (C1).
    TabNeu,
    /// Den aktiven Tab schliessen (C1).
    TabSchliessen,
    /// Zum naechsten Tab wechseln (C1).
    TabNaechster,
    /// Zum vorigen Tab wechseln (C1).
    TabVoriger,
    /// Das aktive Dateifenster wechseln (C1).
    FensterWechseln,
    /// Die Lesezeichen- und Geraeteleiste ein- und ausblenden (C7).
    LeisteUmschalten,
    /// Das zweite Dateifenster ein- und ausblenden (C7).
    ZweitesFensterUmschalten,
    /// Das Vorschaufenster ein- und ausblenden (C7, dieselbe Funktion wie
    /// "Vorschau anzeigen" aus C3).
    VorschauUmschalten,
    /// Das geschlossene Anwendungsfenster wieder nach vorn holen (C7).
    FensterEinblenden,
    /// Das Anwendungsfenster schliessen (C7).
    FensterSchliessen,
    /// Den aktiven Bereich um einen Schritt verbreitern (C7).
    BereichVerbreitern,
    /// Den aktiven Bereich um einen Schritt verschmaelern (C7).
    BereichVerschmaelern,
    /// Die Auswahl in den Ordner des anderen Dateifensters kopieren (C4).
    Kopieren,
    /// Die Auswahl in den Ordner des anderen Dateifensters verschieben (C4).
    Verschieben,
    /// Die Auswahl in den Papierkorb des Systems raeumen (C4, Taste Delete).
    InPapierkorb,
    /// Die Auswahl endgueltig loeschen (C4, F8; mit Rueckfrage davor).
    EndgueltigLoeschen,
    /// Eine laufende Dateioperation abbrechen (C4).
    Abbrechen,
    /// Einen Ordner im Ordner des aktiven Fensters anlegen (C4).
    OrdnerAnlegen,
    /// Eine leere Datei im Ordner des aktiven Fensters anlegen (C4).
    DateiAnlegen,
    /// Die markierten Eintraege im Stapel umbenennen (C4).
    UmbenennenStapel,
    /// Den ausgewaehlten Eintrag direkt in der Liste umbenennen (C4).
    Umbenennen,
    /// Den angezeigten Ordner in der eingestellten Terminal-Anwendung
    /// oeffnen (C11).
    TerminalOeffnen,
    /// Den Ordner des aktiven Dateifensters als Lesezeichen anlegen (C5).
    LesezeichenAnlegen,
    /// Das ausgewaehlte Lesezeichen umbenennen (C5).
    LesezeichenUmbenennen,
    /// Das ausgewaehlte Lesezeichen loeschen (C5).
    LesezeichenLoeschen,
    /// Das ausgewaehlte Lesezeichen einen Platz nach oben schieben (C5).
    LesezeichenHoch,
    /// Das ausgewaehlte Lesezeichen einen Platz nach unten schieben (C5).
    LesezeichenRunter,
    /// Den Eingabefokus in die Lesezeichen- und Geraeteleiste setzen (C5).
    FokusLeiste,
    /// Den Eingabefokus zurueck in das aktive Dateifenster setzen (C5).
    FokusDateifenster,
    /// Die Belegungsansicht zeigen: jede Funktion mit ihren Kombinationen,
    /// aenderbar und zuruecksetzbar (C3).
    BelegungAnsehen,
    /// Die Anwendung beenden (C3).
    Beenden,
}

impl Kommando {
    /// Die Kennung, unter der die Belegungsdatei die zugehoerige Funktion
    /// fuehrt, je Kommando.
    pub const KENNUNGEN: [(Kommando, &'static str); 52] = [
        (Kommando::AuswahlHoch, "auswahl_hoch"),
        (Kommando::AuswahlRunter, "auswahl_runter"),
        (Kommando::SeiteHoch, "seite_hoch"),
        (Kommando::SeiteRunter, "seite_runter"),
        (Kommando::Listenanfang, "listenanfang"),
        (Kommando::Listenende, "listenende"),
        (Kommando::Oeffnen, "oeffnen"),
        (Kommando::OrdnerAufwaerts, "ordner_aufwaerts"),
        (Kommando::Pfadeingabe, "pfadeingabe"),
        (Kommando::MarkierungUmschalten, "markierung_umschalten"),
        (Kommando::AlleMarkieren, "alle_markieren"),
        (Kommando::MarkierungAufheben, "markierung_aufheben"),
        (Kommando::MarkierungUmkehren, "markierung_umkehren"),
        (Kommando::SortierungName, "sortierung_name"),
        (Kommando::SortierungGroesse, "sortierung_groesse"),
        (Kommando::SortierungDatum, "sortierung_datum"),
        (Kommando::SortierungTyp, "sortierung_typ"),
        (
            Kommando::SortierrichtungUmkehren,
            "sortierrichtung_umkehren",
        ),
        (Kommando::VersteckteUmschalten, "versteckte_umschalten"),
        (Kommando::ZwischenablageSpringen, "zwischenablage_springen"),
        (Kommando::ZwischenablageAnsehen, "zwischenablage_ansehen"),
        (Kommando::TabNeu, "tab_neu"),
        (Kommando::TabSchliessen, "tab_schliessen"),
        (Kommando::TabNaechster, "tab_naechster"),
        (Kommando::TabVoriger, "tab_voriger"),
        (Kommando::FensterWechseln, "fenster_wechseln"),
        (Kommando::LeisteUmschalten, "leiste_umschalten"),
        (
            Kommando::ZweitesFensterUmschalten,
            "zweites_fenster_umschalten",
        ),
        (Kommando::VorschauUmschalten, "vorschau_umschalten"),
        (Kommando::FensterEinblenden, "fenster_einblenden"),
        (Kommando::FensterSchliessen, "fenster_schliessen"),
        (Kommando::BereichVerbreitern, "bereich_verbreitern"),
        (Kommando::BereichVerschmaelern, "bereich_verschmaelern"),
        (Kommando::Kopieren, "kopieren"),
        (Kommando::Verschieben, "verschieben"),
        (Kommando::InPapierkorb, "in_papierkorb"),
        (Kommando::EndgueltigLoeschen, "endgueltig_loeschen"),
        (Kommando::Abbrechen, "abbrechen"),
        (Kommando::OrdnerAnlegen, "ordner_anlegen"),
        (Kommando::DateiAnlegen, "datei_anlegen"),
        (Kommando::UmbenennenStapel, "umbenennen_stapel"),
        (Kommando::Umbenennen, "umbenennen"),
        (Kommando::TerminalOeffnen, "terminal_oeffnen"),
        (Kommando::LesezeichenAnlegen, "lesezeichen_anlegen"),
        (Kommando::LesezeichenUmbenennen, "lesezeichen_umbenennen"),
        (Kommando::LesezeichenLoeschen, "lesezeichen_loeschen"),
        (Kommando::LesezeichenHoch, "lesezeichen_hoch"),
        (Kommando::LesezeichenRunter, "lesezeichen_runter"),
        (Kommando::FokusLeiste, "fokus_leiste"),
        (Kommando::FokusDateifenster, "fokus_dateifenster"),
        (Kommando::BelegungAnsehen, "belegung_ansehen"),
        (Kommando::Beenden, "beenden"),
    ];

    /// Das Kommando zu einer Kennung, falls es in dieser Runde schon eines gibt.
    ///
    /// `None` heisst nicht "unbekannte Funktion", sondern "noch nicht gebaut".
    /// Ob die Kennung ueberhaupt zum Wortschatz gehoert, hat die Belegung schon
    /// beim Einlesen geprueft.
    pub fn aus_kennung(kennung: &str) -> Option<Kommando> {
        Self::KENNUNGEN
            .into_iter()
            .find(|(_, benannt)| *benannt == kennung)
            .map(|(kommando, _)| kommando)
    }

    /// Welcher Bereich den Eingabefokus haben muss, damit dieses Kommando
    /// wirkt (C5).
    ///
    /// **Genau einer je Kommando, und das erzwingt der Uebersetzer.** Die
    /// Zuordnung steht als vollstaendige Fallunterscheidung ohne
    /// Auffangzweig: ein neues Kommando uebersetzt nicht, bevor es hier seinen
    /// Bereich genannt hat, und mehr als einen kann keines tragen. Eine
    /// Tabelle mit Auffangzweig gaebe einem vergessenen Kommando
    /// stillschweigend den Bereich des Nachbarn.
    ///
    /// Vier Gruppen, und die Grenze zwischen ihnen ist die Frage, **wer den
    /// Befehl ausfuehrt**. Was das Fenstermodell traegt, wirkt ueberall; was
    /// ein Dateifenster traegt, braucht dessen Fokus; was die Leiste traegt,
    /// den ihren; die vier Tabbefehle bedienen den Bereich mit Tabs, der den
    /// Fokus hat (C1 wie C6). Drei Befehle folgen ihr nicht und stehen
    /// deshalb hier:
    /// [`Kommando::AuswahlHoch`] und [`Kommando::AuswahlRunter`] bewegen die
    /// Auswahl des Bereichs, der den Fokus hat, und gehoeren deshalb keinem
    /// von beiden allein; [`Kommando::LesezeichenAnlegen`] liest den Ordner
    /// des aktiven Dateifensters und schreibt in die Leiste, braucht also
    /// keinen von beiden im Fokus.
    pub const fn wirkungsbereich(self) -> Wirkungsbereich {
        match self {
            // Das Fenster als ganzes. Die Belegungsansicht aus C3 steht hier,
            // weil sie aus jedem Fokus heraus erreichbar sein muss: sie zeigt
            // die Belegung der ganzen Anwendung und gehoert keinem Bereich,
            // so wenig wie das Ein- und Ausblenden der Bereiche auf F3.
            Kommando::FensterWechseln
            | Kommando::LeisteUmschalten
            | Kommando::ZweitesFensterUmschalten
            | Kommando::VorschauUmschalten
            | Kommando::FensterEinblenden
            | Kommando::FensterSchliessen
            | Kommando::BereichVerbreitern
            | Kommando::BereichVerschmaelern
            | Kommando::Abbrechen
            | Kommando::BelegungAnsehen
            | Kommando::Beenden => Wirkungsbereich::Ueberall,
            // Die Auswahl des fokussierten Bereichs (C2 und C5).
            Kommando::AuswahlHoch | Kommando::AuswahlRunter => Wirkungsbereich::Ueberall,
            // Der Fokuswechsel selbst und das Anlegen eines Lesezeichens (C5).
            Kommando::FokusLeiste | Kommando::FokusDateifenster | Kommando::LesezeichenAnlegen => {
                Wirkungsbereich::Ueberall
            }
            // Die Leiste (C5).
            Kommando::LesezeichenUmbenennen
            | Kommando::LesezeichenLoeschen
            | Kommando::LesezeichenHoch
            | Kommando::LesezeichenRunter => Wirkungsbereich::Leiste,
            // Die vier Tabbefehle aus C1. Sie bedienen nach C6 auch die Tabs
            // des Vorschaufensters und wirken auf den Bereich mit Tabs, der
            // den Fokus hat; so fuehrt es die Auslieferungsbelegung seit S9.
            Kommando::TabNeu
            | Kommando::TabSchliessen
            | Kommando::TabNaechster
            | Kommando::TabVoriger => Wirkungsbereich::Tabbereich,
            // Alles, was ein Dateifenster ausfuehrt: Bewegung ueber die Liste
            // hinaus, Navigation, Markierung, Sortierung, die Dateioperationen
            // aus C4, die beiden Zwischenablage-Befehle aus C10
            // (Nutzerentscheid vom 260805-0000) und der Terminal-Befehl aus
            // C11, der den Ordner des sichtbaren Tabs uebergibt.
            Kommando::SeiteHoch
            | Kommando::SeiteRunter
            | Kommando::Listenanfang
            | Kommando::Listenende
            | Kommando::Oeffnen
            | Kommando::OrdnerAufwaerts
            | Kommando::Pfadeingabe
            | Kommando::MarkierungUmschalten
            | Kommando::AlleMarkieren
            | Kommando::MarkierungAufheben
            | Kommando::MarkierungUmkehren
            | Kommando::SortierungName
            | Kommando::SortierungGroesse
            | Kommando::SortierungDatum
            | Kommando::SortierungTyp
            | Kommando::SortierrichtungUmkehren
            | Kommando::VersteckteUmschalten
            | Kommando::ZwischenablageSpringen
            | Kommando::ZwischenablageAnsehen
            | Kommando::Kopieren
            | Kommando::Verschieben
            | Kommando::InPapierkorb
            | Kommando::EndgueltigLoeschen
            | Kommando::OrdnerAnlegen
            | Kommando::DateiAnlegen
            | Kommando::UmbenennenStapel
            | Kommando::Umbenennen
            | Kommando::TerminalOeffnen => Wirkungsbereich::Dateifenster,
        }
    }

    /// Die Kennung dieses Kommandos in der Belegungsdatei.
    pub const fn kennung(self) -> &'static str {
        let mut stelle = 0;
        while stelle < Self::KENNUNGEN.len() {
            let (kommando, kennung) = Self::KENNUNGEN[stelle];
            if kommando as u8 == self as u8 {
                return kennung;
            }
            stelle += 1;
        }
        panic!("jedes Kommando steht in KENNUNGEN")
    }
}

/// Eine Funktion mit allen ihren Kombinationen: eine Zeile der
/// Belegungsansicht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Funktion {
    kennung: String,
    name: String,
    tasten: Vec<Kombination>,
    reserviert_fuer: Option<String>,
    gehalten_von: Option<String>,
}

impl Funktion {
    /// Der maschinenlesbare Bezeichner, unter dem `keymap.toml` sie fuehrt.
    pub fn kennung(&self) -> &str {
        &self.kennung
    }

    /// Die deutsche Beschriftung fuer die Belegungsansicht.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Alle Kombinationen, die diese Funktion ausloesen.
    pub fn tasten(&self) -> &[Kombination] {
        &self.tasten
    }

    /// Gesetzt, wenn die Funktion benannt, aber einer spaeteren Runde
    /// vorbehalten ist.
    pub fn reserviert_fuer(&self) -> Option<&str> {
        self.reserviert_fuer.as_deref()
    }

    /// Wer den Tastendruck zustellt, falls es nicht der Ereignisabgriff ist.
    ///
    /// `None` heisst: der Abgriff aus C2 stellt zu. `Some("menue")` heisst: ein
    /// `NSMenuItem` traegt die Kombination als Kuerzel, und die Antwortkette
    /// entscheidet, wer sie beantwortet. Siehe den Modulkopf; das Feld sagt,
    /// **wer zustellt**, und nicht, was der Tastendruck tut.
    pub fn gehalten_von(&self) -> Option<&str> {
        self.gehalten_von.as_deref()
    }

    /// Das Kommando dieser Funktion, falls diese Runde es schon ausfuehrt.
    ///
    /// Eine zugestellte Funktion hat nie eines: was das Hauptmenue zustellt,
    /// fuehrt die Antwortkette aus und nicht KRK. Ohne diese Zeile haenge die
    /// Zusage daran, dass [`Kommando::KENNUNGEN`] die vier Textbefehle zufaellig
    /// nicht nennt — die vierte Stelle der Zustellerregel aus dem Modulkopf.
    pub fn kommando(&self) -> Option<Kommando> {
        if self.gehalten_von.is_some() {
            return None;
        }
        Kommando::aus_kennung(&self.kennung)
    }

    /// Wie eine Meldung diese Funktion benennt.
    pub fn benennung(&self) -> Funktionsname {
        Funktionsname::neu(&self.kennung, &self.name)
    }
}

/// Was ein Tastendruck in der Belegung findet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nachschlag<'a> {
    /// Die Kombination gehoert dieser Funktion.
    Funktion(&'a Funktion),
    /// Keine Funktion, und keine Zusatztaste gehalten: der Tastendruck faellt
    /// auf die Sprungmarke aus C2 durch, das Tippen der Anfangsbuchstaben.
    Sprungmarke,
    /// Keine Funktion, und eine Zusatztaste gehalten: nichts geschieht.
    Unbelegt,
}

/// Die vollstaendige Belegung: jede Funktion mit ihren Kombinationen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Belegung {
    funktionen: Vec<Funktion>,
}

impl Belegung {
    /// Die eingebettete Auslieferungsbelegung.
    pub fn auslieferung() -> Self {
        AUSLIEFERUNG.clone()
    }

    /// Eine Belegung des Nutzers, geprueft gegen den Wortschatz der
    /// Auslieferungsbelegung.
    // Der Fehlerwert traegt einen [`Konflikt`] und damit die Namen beider
    // beteiligten Funktionen; er ist gross fuer einen `Err`. Ihn zu verpacken
    // spart Platz auf einem Pfad, den KRK hoechstens beim Start und bei einer
    // Umbelegung geht, und kostete an jeder Fundstelle eine Zeile, die vom
    // Sachverhalt ablenkt. Dieselbe Abwaegung gilt fuer `zuweisen` und `bauen`.
    #[allow(clippy::result_large_err)]
    pub fn vom_nutzer(datei: &Belegungsdatei) -> Result<Self, Belegungsfehler> {
        Self::bauen(datei, Some(&AUSLIEFERUNG))
    }

    /// Alle Funktionen, in der Reihenfolge der Datei.
    pub fn funktionen(&self) -> &[Funktion] {
        &self.funktionen
    }

    /// Die Funktion zu einer Kennung.
    pub fn funktion(&self, kennung: &str) -> Option<&Funktion> {
        self.funktionen
            .iter()
            .find(|funktion| funktion.kennung == kennung)
    }

    /// Was ein Tastendruck ausloest.
    ///
    /// Der Durchlauf ist eine gewoehnliche Schleife ueber die wenigen Dutzend
    /// ausgelieferten Kombinationen und kein Nachschlagbaum. Verglichen werden
    /// zwei ganze Zahlen; gegen die Zusage L1 von einer Bildlaenge faellt das
    /// nicht ins Gewicht, und eine abgeleitete Tabelle daneben waere ein
    /// zweiter Bestand, den jede Aenderung mitfuehren muesste. Die Groessen-
    /// ordnung traegt das Argument, die genaue Zahl nicht: sie waechst mit
    /// jeder Runde, und ein Literal an dieser Stelle veraltet ungeprueft.
    ///
    /// **Eine vom Hauptmenue zugestellte Funktion kommt hier nicht vor.** Der
    /// Abgriff laeuft nur ausserhalb eines Textfeldes und darf deshalb nur
    /// sehen, was er selbst zustellt; der Modulkopf schreibt aus, warum das
    /// keine Zutat, sondern die tragende Haelfte der Zustellerregel ist. Nach
    /// dem Ueberspringen meint diese Antwort, was ihr Aufrufer braucht: was
    /// dieser Tastendruck **ausserhalb eines Textfeldes** ausloest.
    pub fn nachschlag(&self, druck: Tastendruck) -> Nachschlag<'_> {
        for funktion in &self.funktionen {
            if funktion.gehalten_von.is_some() {
                continue;
            }
            if funktion
                .tasten
                .iter()
                .any(|kombination| kombination.tastendruck() == druck)
            {
                return Nachschlag::Funktion(funktion);
            }
        }
        if druck.maske.ist_leer() {
            Nachschlag::Sprungmarke
        } else {
            Nachschlag::Unbelegt
        }
    }

    /// Gibt einer Funktion eine weitere Kombination.
    ///
    /// Traegt die Funktion sie schon, geschieht nichts und es ist kein Fehler.
    /// Traegt eine **andere Funktion desselben Zustellers** sie, bleibt die
    /// Belegung unveraendert und der [`Konflikt`] nennt beide Funktionen.
    ///
    /// Der Zusteller steht hier aus demselben Grund wie in
    /// [`Belegung::konflikte`]: sonst meldete die Belegungsansicht aus C3 einen
    /// Konflikt, den das Einlesen nicht kennt, und die beiden Wege in dieselbe
    /// Belegung widersprachen einander.
    #[allow(clippy::result_large_err)]
    pub fn zuweisen(
        &mut self,
        kennung: &str,
        kombination: Kombination,
    ) -> Result<(), Zuweisungsfehler> {
        let Some(stelle) = self
            .funktionen
            .iter()
            .position(|funktion| funktion.kennung == kennung)
        else {
            return Err(Zuweisungsfehler::UnbekannteFunktion(kennung.to_owned()));
        };
        let zusteller = self.funktionen[stelle].gehalten_von.clone();

        if let Some(andere) = self.funktionen.iter().find(|funktion| {
            funktion.kennung != kennung
                && funktion.gehalten_von == zusteller
                && funktion.tasten.contains(&kombination)
        }) {
            return Err(Zuweisungsfehler::Konflikt(Konflikt {
                kombination,
                andere: andere.benennung(),
                bewerber: self.funktionen[stelle].benennung(),
            }));
        }

        if !self.funktionen[stelle].tasten.contains(&kombination) {
            self.funktionen[stelle].tasten.push(kombination);
        }
        Ok(())
    }

    /// Setzt die gesamte Belegung auf den Auslieferungszustand zurueck.
    pub fn zuruecksetzen(&mut self) {
        *self = Self::auslieferung();
    }

    /// Jede Kombination, die zwei Funktionen **desselben Zustellers**
    /// beanspruchen.
    ///
    /// Leer fuer jede Belegung, die [`Belegung::vom_nutzer`] oder
    /// [`Belegung::auslieferung`] geliefert hat: beide weisen eine
    /// widerspruechliche Datei schon beim Einlesen ab. Die Pruefung steht
    /// trotzdem als eigener Aufruf da, weil das Abnahmekriterium von C3 sie
    /// woertlich verlangt.
    ///
    /// Der Zusteller gehoert in den Vergleich, weil zwei Funktionen einander
    /// nur begegnen koennen, wenn beide im selben Fokuszustand erreichbar sind;
    /// der Modulkopf schreibt die Regel aus. Ausgeliefert gibt es genau einen
    /// Fall: `cmd+a` markiert im Dateifenster alle Eintraege und waehlt im
    /// Textfeld den Text aus.
    pub fn konflikte(&self) -> Vec<Konflikt> {
        let mut gefunden = Vec::new();
        for (stelle, funktion) in self.funktionen.iter().enumerate() {
            for kombination in &funktion.tasten {
                for vorige in self.funktionen.iter().take(stelle) {
                    if vorige.gehalten_von == funktion.gehalten_von
                        && vorige.tasten.contains(kombination)
                    {
                        gefunden.push(Konflikt {
                            kombination: *kombination,
                            andere: vorige.benennung(),
                            bewerber: funktion.benennung(),
                        });
                    }
                }
            }
        }
        gefunden
    }

    /// Schreibt die Belegung nach `keymap.toml`, atomar ueber die Ablage.
    pub fn sichern(&self, ablage: &Ablage) -> io::Result<()> {
        ablage.sichern(Datei::Belegung, &Belegungsdatei::from(self))
    }

    /// Baut eine Belegung aus der gelesenen Datei.
    ///
    /// `wortschatz` ist `None` fuer die Auslieferungsbelegung, die ihn erst
    /// festlegt, und `Some` fuer jede Belegung des Nutzers, die sich daran
    /// messen lassen muss.
    #[allow(clippy::result_large_err)]
    fn bauen(
        datei: &Belegungsdatei,
        wortschatz: Option<&Belegung>,
    ) -> Result<Self, Belegungsfehler> {
        let mut funktionen: Vec<Funktion> = Vec::with_capacity(datei.funktionen.len());
        for eintrag in &datei.funktionen {
            if let Some(wortschatz) = wortschatz
                && wortschatz.funktion(&eintrag.id).is_none()
            {
                return Err(Belegungsfehler::UnbekannteFunktion(eintrag.id.clone()));
            }
            if funktionen
                .iter()
                .any(|funktion| funktion.kennung == eintrag.id)
            {
                return Err(Belegungsfehler::FunktionDoppelt(eintrag.id.clone()));
            }

            let mut tasten = Vec::with_capacity(eintrag.tasten.len());
            for text in &eintrag.tasten {
                let kombination =
                    Kombination::lesen(text).map_err(|fehler| Belegungsfehler::Schreibweise {
                        kennung: eintrag.id.clone(),
                        text: text.clone(),
                        fehler,
                    })?;
                if !tasten.contains(&kombination) {
                    tasten.push(kombination);
                }
            }

            funktionen.push(Funktion {
                kennung: eintrag.id.clone(),
                name: eintrag.name.clone(),
                tasten,
                reserviert_fuer: eintrag.reserviert_fuer.clone(),
                gehalten_von: eintrag.gehalten_von.clone(),
            });
        }

        // Funktionen, die die Nutzerdatei nicht nennt, treten unbelegt hinzu.
        // Die Belegungsansicht fuehrt damit weiter jede Funktion, und der
        // Nutzer kann eine, die er geloescht hat, wieder erreichbar machen.
        if let Some(wortschatz) = wortschatz {
            for bekannt in &wortschatz.funktionen {
                if !funktionen
                    .iter()
                    .any(|funktion| funktion.kennung == bekannt.kennung)
                {
                    funktionen.push(Funktion {
                        tasten: Vec::new(),
                        ..bekannt.clone()
                    });
                }
            }
        }

        let belegung = Self { funktionen };
        match belegung.konflikte().into_iter().next() {
            Some(konflikt) => Err(Belegungsfehler::Konflikt(konflikt)),
            None => Ok(belegung),
        }
    }
}

impl Default for Belegung {
    fn default() -> Self {
        Self::auslieferung()
    }
}

/// Laedt die Belegung des Nutzers aus `keymap.toml`.
///
/// Scheitert nie. Eine fehlende Datei ist der erste Start und liefert die
/// Auslieferungsbelegung ohne Meldung. Eine nicht lesbare, syntaktisch kaputte
/// oder inhaltlich widerspruechliche Datei liefert sie ebenfalls, dazu eine
/// [`Ersetzung`], die die Datei und den Grund nennt. Die Datei auf der Platte
/// bleibt in jedem Fall stehen; `keymap.toml` ist von Hand aenderbar, und ein
/// Tippfehler darin darf die Arbeit des Nutzers nicht loeschen.
pub fn laden(ablage: &Ablage) -> Geladen<Belegung> {
    let roh: Geladen<Belegungsdatei> = ablage.laden(Datei::Belegung);
    match Belegung::vom_nutzer(&roh.wert) {
        Ok(belegung) => Geladen {
            wert: belegung,
            ersetzung: roh.ersetzung,
        },
        Err(fehler) => Geladen {
            wert: Belegung::auslieferung(),
            ersetzung: Some(Ersetzung {
                datei: ablage.pfad(Datei::Belegung),
                grund: Grund::Beschaedigt(fehler.to_string()),
            }),
        },
    }
}

/// Die Belegung fuer den laufenden Betrieb, dazu die Meldung, falls eine
/// noetig war.
///
/// Der eine Aufruf, den die Oberflaeche beim Start macht. Jede Meldung nimmt
/// [`melden`] und damit denselben Weg wie die der uebrigen Ablagedateien; eine
/// zweite Ausgabestelle entsteht nicht. Geschrieben wird sie hier nicht: der
/// Kern hat seit Schritt 12 keinen Ausgabekanal, und der Aufrufer in `krk-ui`
/// setzt den Satz in die Statuszeile.
pub fn fuer_den_betrieb() -> (Belegung, Option<String>) {
    match Ablage::im_benutzerverzeichnis() {
        Ok(ablage) => laden(&ablage).mit_meldung(),
        Err(fehler) => (
            Belegung::auslieferung(),
            Some(melden(&Ersetzung {
                datei: PathBuf::from(Datei::Belegung.dateiname()),
                grund: Grund::NichtLesbar(fehler.to_string()),
            })),
        ),
    }
}

/// Warum eine gelesene Datei keine Belegung ergibt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Belegungsfehler {
    /// Eine Kombination steht nicht in der vorgeschriebenen Schreibweise.
    Schreibweise {
        /// Die Funktion, bei der sie steht.
        kennung: String,
        /// Die Zeichenkette, wie sie in der Datei steht.
        text: String,
        /// Woran das Lesen scheiterte.
        fehler: Schreibfehler,
    },
    /// Die Datei nennt eine Funktion, die KRK nicht kennt.
    UnbekannteFunktion(String),
    /// Dieselbe Funktion steht zweimal.
    FunktionDoppelt(String),
    /// Zwei Funktionen beanspruchen dieselbe Kombination.
    Konflikt(Konflikt),
}

impl fmt::Display for Belegungsfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Belegungsfehler::Schreibweise {
                kennung,
                text,
                fehler,
            } => write!(
                ausgabe,
                "die Funktion {kennung} traegt die Kombination \"{text}\": {fehler}"
            ),
            Belegungsfehler::UnbekannteFunktion(kennung) => {
                write!(ausgabe, "KRK kennt keine Funktion namens {kennung}")
            }
            Belegungsfehler::FunktionDoppelt(kennung) => {
                write!(ausgabe, "die Funktion {kennung} steht zweimal")
            }
            Belegungsfehler::Konflikt(konflikt) => konflikt.fmt(ausgabe),
        }
    }
}

impl std::error::Error for Belegungsfehler {}

/// Warum eine Zuweisung nicht zustande kam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zuweisungsfehler {
    /// Die Kombination gehoert bereits einer anderen Funktion.
    Konflikt(Konflikt),
    /// Die Belegung kennt keine Funktion dieser Kennung.
    UnbekannteFunktion(String),
}

impl fmt::Display for Zuweisungsfehler {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Zuweisungsfehler::Konflikt(konflikt) => konflikt.fmt(ausgabe),
            Zuweisungsfehler::UnbekannteFunktion(kennung) => {
                write!(ausgabe, "KRK kennt keine Funktion namens {kennung}")
            }
        }
    }
}

impl std::error::Error for Zuweisungsfehler {}

/// Die Gestalt von `default-keymap.toml` und `keymap.toml`, unveraendert.
///
/// Der Zwischenschritt zwischen TOML und [`Belegung`]: hier stehen die
/// Kombinationen noch als Zeichenketten, und keine Regel ist geprueft. Erst
/// [`Belegung::vom_nutzer`] macht daraus eine Belegung.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Belegungsdatei {
    /// Ein Eintrag je Funktion, in der Reihenfolge der Datei.
    #[serde(default, rename = "funktion")]
    funktionen: Vec<Eintrag>,
}

impl Default for Belegungsdatei {
    /// Die eingebettete Auslieferungsbelegung.
    ///
    /// Damit liefert [`Ablage::laden`] bei fehlender oder kaputter
    /// `keymap.toml` den Auslieferungszustand und nicht eine leere Belegung, in
    /// der keine Taste mehr etwas tut.
    fn default() -> Self {
        Belegungsdatei::from(&Belegung::auslieferung())
    }
}

impl From<&Belegung> for Belegungsdatei {
    /// Der Rueckweg, und er traegt jedes Feld mit.
    ///
    /// Fehlte hier `gehalten_von`, schriebe [`Belegung::sichern`] eine
    /// `keymap.toml`, in der `text_alles_auswaehlen` keinen Zusteller mehr
    /// traegt; beim naechsten Start stuende `cmd+a` bei zwei Funktionen
    /// desselben Zustellers, das Einlesen meldete einen Konflikt, und der
    /// Nutzer haette eine Datei, die KRK selbst geschrieben und dann nicht mehr
    /// angenommen hat.
    fn from(belegung: &Belegung) -> Self {
        Self {
            funktionen: belegung
                .funktionen
                .iter()
                .map(|funktion| Eintrag {
                    id: funktion.kennung.clone(),
                    name: funktion.name.clone(),
                    tasten: funktion
                        .tasten
                        .iter()
                        .map(|kombination| kombination.to_string())
                        .collect(),
                    reserviert_fuer: funktion.reserviert_fuer.clone(),
                    gehalten_von: funktion.gehalten_von.clone(),
                })
                .collect(),
        }
    }
}

/// Ein `[[funktion]]`-Block der Datei.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Eintrag {
    id: String,
    name: String,
    tasten: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reserviert_fuer: Option<String>,
    /// Wer den Tastendruck zustellt; siehe [`Funktion::gehalten_von`].
    ///
    /// Optional wie `reserviert_fuer` daneben, und aus demselben Grund
    /// weggelassen statt leer geschrieben: die weitaus meisten Funktionen
    /// tragen es nicht, und `deny_unknown_fields` oben laesst kein Feld durch,
    /// das der Parser nicht kennt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gehalten_von: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung() {
        let belegung = Belegung::auslieferung();
        for (kommando, kennung) in Kommando::KENNUNGEN {
            assert!(
                belegung.funktion(kennung).is_some(),
                "{kommando:?} nennt die Kennung {kennung}, die Auslieferungsbelegung kennt sie nicht"
            );
            assert_eq!(Kommando::aus_kennung(kennung), Some(kommando));
            assert_eq!(kommando.kennung(), kennung);
        }
    }

    /// Was beim Bauen aus der Datei verschwinden koennte, verschwindet nicht.
    ///
    /// Die Vorgaengerin dieser Pruefung schrieb die Zahl der Funktionen und die
    /// der Kombinationen als Literal hin. Das prueft die Groesse der Datei und
    /// nicht die Arbeit von [`Belegung::bauen`]: jeder Nachtrag in
    /// `default-keymap.toml` liess sie fehlschlagen, ohne dass etwas kaputt war.
    /// Verglichen wird deshalb die gelesene Datei mit der gebauten Belegung.
    /// [`Belegung::bauen`] verwirft stillschweigend eine Kombination, die
    /// innerhalb derselben Funktion zweimal steht; genau das faellt hier auf.
    #[test]
    fn beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren() {
        let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
            .expect("die eingebettete Auslieferungsbelegung ist gueltiges TOML");
        let belegung = Belegung::auslieferung();

        assert!(
            !datei.funktionen.is_empty(),
            "die Auslieferungsbelegung nennt keine einzige Funktion"
        );
        assert_eq!(
            belegung.funktionen().len(),
            datei.funktionen.len(),
            "die gebaute Belegung fuehrt nicht so viele Funktionen wie die Datei"
        );

        let in_der_datei: usize = datei
            .funktionen
            .iter()
            .map(|eintrag| eintrag.tasten.len())
            .sum();
        let gebaut: usize = belegung
            .funktionen()
            .iter()
            .map(|funktion| funktion.tasten().len())
            .sum();
        assert_eq!(
            gebaut, in_der_datei,
            "eine Kombination der Auslieferungsbelegung steht doppelt und ist beim Bauen entfallen"
        );
    }

    #[test]
    fn eine_belegung_ueberlebt_schreiben_und_wiedereinlesen() {
        let belegung = Belegung::auslieferung();
        let text = toml::to_string(&Belegungsdatei::from(&belegung))
            .expect("die Belegung laesst sich schreiben");
        let wieder: Belegungsdatei = toml::from_str(&text).expect("und wieder einlesen");
        assert_eq!(
            Belegung::vom_nutzer(&wieder),
            Ok(belegung),
            "der Umweg ueber TOML hat die Belegung veraendert"
        );
    }
}
