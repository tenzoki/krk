//! Die eine Zulaessigkeitsfrage: darf dieser Befehl hier gerade wirken (C2)?
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die vier Eingaben liest der
//! Anwendungsdelegierte und stellt sie als [`Lage`] zusammen; die Regel selbst
//! steht hier und ist ohne Fenster pruefbar.
//!
//! ```text
//!  Kommando ─┬─> Wirkungsbereich ─┐
//!            └─> Ausnahmeliste ───┤
//!                                 ├──> zulaessig()
//!  Lage ─────┬─> blatt_steht ─────┤
//!            ├─> Ersthelferbefund ┤
//!            ├─> Fokus ───────────┤
//!            └─> Schluesselfenster┘
//! ```
//!
//! # Eine Frage, zwei Frager
//!
//! Der Ereignisabgriff fragt ueber `Anwendungsdelegierter::kommando_ausfuehren`,
//! ob er den Tastendruck schlucken darf; das Hauptmenue fragt ueber
//! `validateMenuItem:`, ob es seinen Eintrag freigeben darf. Beide fragen
//! **dieselbe** Funktion auf **derselben** [`Lage`], und ihre Antworten koennen
//! deshalb nicht auseinanderlaufen. Genau das ist der Grund, aus dem die Regel
//! eine Funktion ist und nicht zwei Abfragen an zwei Stellen: ein freigegebener
//! Menueeintrag zu einem abgewiesenen Tastendruck fuehrte den Befehl aus, den
//! der Abgriff eben verweigert hat.
//!
//! # Die vier Bestandteile
//!
//! 1. **Es steht kein Blatt**, oder der Befehl ist waehrend eines Blattes
//!    erlaubt. Die zweite Haelfte beantwortet
//!    [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt)
//!    und keine zweite Fassung daneben.
//! 2. **Der Ersthelfer des Schluesselfensters gehoert nicht AppKit.** Dieselbe
//!    Frage, die der Fokusvorbehalt bis zum 260813 im Ereignisabgriff selbst
//!    gestellt hat; sie ist hierher gewandert und steht nicht mehr daneben.
//! 3. **[`fokus::wirkt`](super::fokus::wirkt) sagt ja** zum Wirkungsbereich des
//!    Befehls und zum Fokus des Augenblicks.
//! 4. **Das Schluesselfenster gehoert KRK.** Es ist das Hauptfenster oder ein
//!    Blatt, das daran haengt; steht ein fremdes Fenster vorn, wirkt kein
//!    Befehl.
//!
//! **Der vierte Bestandteil ist der juengste, und er schliesst die Luecke, die
//! der Ueber-Dialog aufreisst.** Ein freistehendes Panel ist kein Blatt, also
//! sagt (1) nichts dazu, und welchen Ersthelfer AppKit darin einsetzt, ist
//! nicht zugesagt, also traegt (2) den Fall nicht. Bestandteil (3) weist vor
//! einem fremden Fenster schon heute jeden Befehl ab, dessen Wirkungsbereich
//! ein Bereich ist, denn `Anwendungsdelegierter::fokus` antwortet dann
//! `Anderswo`; durch kommt genau, was `Wirkungsbereich::Ueberall` traegt, und
//! das ist der Rest, den (4) aufhaelt. Entschieden hat das der Nutzer am
//! 260813-1055 mit Moeglichkeit 2 aus
//! `decisions/260813-1037_*_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`.
//!
//! **Ein anhaengendes Blatt braucht dafuer keinen Sonderfall.** Es **ist** das
//! Schluesselfenster, also sagt (4) fuer ein stehendes Blatt ja, und ueber das
//! Blatt entscheidet allein (1) wie bisher. Der Abbruch aus dem Blatt heraus
//! bleibt damit erreichbar, ohne dass eine Ausnahme dafuer geschrieben waere.
//!
//! **(4) steht neben (1) und (2) und nicht ueber ihnen.** Alle drei fragen nach
//! der Lage, und die Ausnahmeliste hebt sie deshalb gemeinsam auf; nur (3)
//! fragt nach dem Wirkungsbereich und bleibt unberuehrt. Der naechste Abschnitt
//! fuehrt es aus.
//!
//! **Der zweite Bestandteil ist der, den man weglaesst, und er traegt den
//! gefaehrlichsten Fall.** Beim Umbenennen direkt in der Dateiliste haelt der
//! Feldeditor eines `NSTextField` den Ersthelferrang. Es steht dabei **kein**
//! Blatt, und `Anwendungsdelegierter::fokus` antwortet fuer diesen Feldeditor
//! `Dateifenster`, weil er eine Unteransicht des Dateifensters ist. Die beiden
//! anderen Bestandteile sagen also beide freundlich ja, und ohne den zweiten
//! waere jeder Befehl des Dateifensters freigegeben — `up`, `down`, `return`,
//! `space` und `tab` liegen in `resources/default-keymap.toml` ohne Zusatztaste.
//! Der Nutzer benennt um, drueckt `up`, und die Auswahl in der Liste springt.
//!
//! # Die Ausnahmeliste
//!
//! [`immer_erreichbar`] hebt die Bestandteile (1), (2) und (4) auf, den
//! dritten **nicht**. In einem Satz: sie hebt jede Sperre auf, die nach der
//! **Lage** fragt, und keine, die nach dem **Wirkungsbereich** fragt.
//!
//! Sie ist aus „kein Verlust gegenueber heute" abgeleitet: `beenden`
//! und `fenster_schliessen` sind heute waehrend einer Umbenennung in der Liste
//! und waehrend eines stehenden Blattes allein ueber ihren Menueeintrag
//! erreichbar, weil der Abgriff den Tastendruck an AppKit weiterreicht und das
//! Hauptmenue ihn aufnimmt. Sobald jeder Eintrag des Menues seine Zulaessigkeit
//! von hier bezieht, naehme die Regel ohne Ausnahme genau diesen Weg weg.
//!
//! **Dass sie auch (4) aufhebt, ist eine Wahl und keine Ableitung.** Der
//! Wortlaut des Entscheids sagt „wirkt kein Befehl", und unter dieser Fassung
//! wirken `beenden` und `fenster_schliessen` doch. Der Grund ist dieselbe
//! Randbedingung: vor dem Freigabewaehler der Runde 6 beendet Cmd+Q die
//! Anwendung heute, und die strenge Lesart naehme diesen Weg weg, ohne dass
//! ihn jemand genannt haette. Der Nutzer hat es am 260813 mit Moeglichkeit 1
//! aus
//! `decisions/260813-1110_*_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`
//! so festgelegt.
//!
//! Dass sie den dritten Bestandteil nicht aufhebt, faellt heute nicht auf:
//! beide Befehle tragen `Wirkungsbereich::Ueberall`, und
//! [`fokus::wirkt`](super::fokus::wirkt) sagt dafuer in jedem Fokus ja. Mit
//! einem dritten Eintrag, der einen Bereich braucht, fiele der Unterschied an,
//! und die Probe `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf` haelt
//! ihn fest.

use krk_core::tasten::Kommando;

use super::fokus::{self, Fokus};
use super::operationen;

/// Was die Oberflaeche im Augenblick der Frage ueber sich weiss.
///
/// Die vier Eingaben der Zulaessigkeitsfrage an **einer** Stelle, damit die
/// Frage rein bleibt und die Tafel aus 280 Faellen sie ohne Fenster stellen
/// kann. Erhoben werden sie von `Anwendungsdelegierter::lage`, und zwar einmal
/// je Eingabe: der Kommandozweig gibt die `Lage` an [`zulaessig`], der
/// Zeichenzweig der Sprungmarke liest drei davon heraus. Zwei Erhebungen
/// desselben Augenblicks koennten auseinanderlaufen, eine kann es nicht.
///
/// `Copy`, weil der Wert vier kleine Felder traegt und die Tafel ihn
/// zweihundertachtzigmal durchreicht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lage {
    /// Ob am Hauptfenster gerade ein Blatt haengt.
    pub blatt_steht: bool,
    /// Ob der Ersthelfer des Schluesselfensters seine AppKit-Bedeutung behaelt.
    ///
    /// Die Textflaeche des eingebauten Editors ist die eine Ausnahme davon und
    /// meldet hier `false`; die Naemlichkeitsfrage dahinter beantwortet der
    /// Anwendungsdelegierte, der die Flaeche haelt.
    pub ersthelfer_gehoert_appkit: bool,
    /// Ob das Schluesselfenster KRKs Hauptfenster oder ein daran haengendes
    /// Blatt ist.
    ///
    /// **Ein anhaengendes Blatt meldet hier `true`**, denn es ist selbst das
    /// Schluesselfenster; ueber das Blatt entscheidet allein
    /// [`blatt_steht`](Self::blatt_steht) zusammen mit
    /// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt).
    /// Die beiden Felder sind unabhaengig: steht ein Blatt und oeffnet der
    /// Nutzer den Ueber-Dialog, ist `blatt_steht` wahr und dieses Feld `false`.
    ///
    /// `false` meldet jedes fremde Fenster, das freistehende Panel des
    /// Ueber-Dialogs so gut wie das Fenster einer anderen Anwendung, und
    /// ebenso ein KRK ohne Schluesselfenster, also im Hintergrund.
    pub schluesselfenster_gehoert_krk: bool,
    /// Wo der Eingabefokus steht.
    pub fokus: Fokus,
}

/// Ob dieser Befehl in dieser Lage wirken darf.
///
/// **Die eine Stelle, an der die Frage beantwortet wird**, und die eine
/// Antwort, die der Ereignisabgriff und die Ausgrauung des Hauptmenues
/// gemeinsam bekommen. Die vier Bestandteile und ihre Herleitung stehen im
/// Modulkopf.
///
/// Eine Sonderbehandlung fuer ein einzelnes Kommando gibt es hier nicht:
/// welcher Bereich noetig ist, sagt der [`Wirkungsbereich`] des Kerns, was
/// waehrend eines Blattes durchkommt, sagt
/// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt),
/// und was ohne Ruecksicht auf beides erreichbar bleibt, sagt
/// [`immer_erreichbar`].
///
/// [`Wirkungsbereich`]: krk_core::tasten::Wirkungsbereich
pub fn zulaessig(kommando: Kommando, lage: Lage) -> bool {
    let kein_blatt_oder_erlaubt =
        !lage.blatt_steht || operationen::waehrend_blatt_erlaubt(kommando);
    let durchgelassen = immer_erreichbar(kommando)
        || (lage.schluesselfenster_gehoert_krk
            && kein_blatt_oder_erlaubt
            && !lage.ersthelfer_gehoert_appkit);

    durchgelassen && fokus::wirkt(kommando.wirkungsbereich(), lage.fokus)
}

/// Die benannte Liste der Befehle, die ein Blatt, ein Textfeld und ein fremdes
/// Schluesselfenster nicht aufhalten.
///
/// **Bewusst keine vollstaendige Fallunterscheidung.** Die uebrigen
/// Fallunterscheidungen dieses Projekts zaehlen jedes Kommando auf, damit ein
/// neues den Bau anhaelt und eine bewusste Einordnung erzwingt. Hier waere das
/// falsch herum: die Liste soll **nicht** mit jedem neuen Befehl wachsen,
/// sondern nur mit einem genannten Grund. Der Vorgabewert ist „gehoert nicht
/// dazu", und ein neues Kommando bekommt ihn stillschweigend und richtig.
///
/// Beide Eintraege stammen aus „kein Verlust gegenueber heute"; die Herleitung
/// steht im Modulkopf. Die Liste hebt die Bestandteile (1), (2) und (4) auf und
/// den dritten nicht: sie hebt jede Sperre auf, die nach der Lage fragt, und
/// keine, die nach dem Wirkungsbereich fragt.
pub fn immer_erreichbar(kommando: Kommando) -> bool {
    matches!(kommando, Kommando::Beenden | Kommando::FensterSchliessen)
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::Wirkungsbereich;

    use crate::quellbaum::quelldateien;

    use super::*;

    /// Die Zulaessigkeitsregel ist im Baum genau einmal erklaert (C2.16, erste
    /// Haelfte).
    ///
    /// **Eine Erklaerungszaehlung und keine Aufruferzaehlung.** Sie haelt die
    /// eine Haelfte: eine zweite Fassung derselben Regel laesst sie rot werden.
    /// Die andere Haelfte, dass beide Frager diese eine Stelle rufen, zaehlt
    /// [`beide_frager_rufen_die_eine_regel`] daneben. Die Begruendung fuer die
    /// Unterscheidung steht in [`crate::quellbaum`].
    ///
    /// Die Nadel steht zusammengesetzt da, wie bei
    /// `es_gibt_genau_einen_menuebauer` in
    /// [`crate::appkit::teilen`]: als ein Stueck
    /// geschrieben faende sie sich selbst und zaehlte eine Fundstelle zu viel.
    #[test]
    fn die_zulaessigkeitsregel_ist_genau_einmal_erklaert() {
        let regel = concat!("fn ", "zulaessig(");
        let erklaerungen: usize = quelldateien()
            .iter()
            .map(|(_, inhalt)| inhalt.matches(regel).count())
            .sum();
        assert_eq!(
            erklaerungen, 1,
            "die Zulaessigkeitsregel ist nicht genau einmal erklaert"
        );
    }

    /// Genau zwei Stellen rufen die Regel, und es sind der Abgriff und die
    /// Ausgrauung (C2.16, zweite Haelfte).
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier, weil C2.16 die Zahl selbst
    /// zusagt.** Der eine Frager ist `Anwendungsdelegierter::kommando_ausfuehren`
    /// und entscheidet, ob der Ereignisabgriff den Tastendruck schluckt; der
    /// andere ist `validateMenuItem:` und entscheidet, ob der Menueeintrag
    /// bedienbar ist. Beide fragen dieselbe Funktion auf derselben [`Lage`], und
    /// **daran** haengt die Runde: gaeben sie verschiedene Antworten, fuehrte
    /// ein freigegebener Eintrag den Befehl aus, den der Abgriff eben verweigert
    /// hat — mit dem Fokus im Editor bewegte ein Auf-Pfeil die Dateiliste.
    ///
    /// Rot wird die Probe, wenn ein dritter berechtigter Frager hinzukommt. Die
    /// richtige Antwort darauf ist die Zahl hier und nicht das Streichen eines
    /// Fragers; was eine Aufruferzaehlung leistet und was nicht, steht in
    /// [`crate::quellbaum`].
    ///
    /// **Gezaehlt wird der Aufruf und nicht seine Schreibweise.** Die Nadel war
    /// bis zur Runde 7 `zulaessigkeit::zulaessig(`, und ein dritter Frager mit
    /// einem `use` und einem unqualifizierten Aufruf waere ihr entgangen — also
    /// genau der Fall, fuer den die Probe steht
    /// (`issues/260813-0540_*_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`).
    /// [`crate::quellbaum::aufrufstellen`] zaehlt jetzt jede Empfaengerform und
    /// jeden Pfad.
    ///
    /// **Diese Datei bleibt aussen vor**, so wie
    /// `das_menue_wird_an_zwei_anlaessen_gebaut` `menue.rs` aussen vor laesst:
    /// hier stehen die Erklaerung und die Tafel aus 280 Faellen, die
    /// [`zulaessig`] zweihundertachtzigmal ruft. Ein dritter Frager **in** dieser
    /// Datei waere Teil der Regel und nicht ein zweiter Weg an ihr vorbei.
    #[test]
    fn beide_frager_rufen_die_eine_regel() {
        let zuhause = "krk-ui/src/kommandos/zulaessigkeit.rs";
        let name = concat!("zulaes", "sig");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| crate::quellbaum::aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 2,
            "die Regel hat nicht die zwei Frager Ereignisabgriff und Ausgrauung"
        );
    }

    /// Die Aufzaehlung der Pruefungen ist die des Programms.
    ///
    /// Dieselbe Begruendung wie in [`super::super::fokus`]: eine zweite Liste
    /// derselben fuenf Werte pruefte womoeglich eine andere Menge als die, ueber
    /// die das Programm laeuft.
    const JEDER_FOKUS: [Fokus; 5] = Fokus::ALLE;

    /// Ein Befehl je Wirkungsbereich, und keiner von ihnen steht auf der
    /// Ausnahmeliste oder kommt waehrend eines Blattes durch.
    ///
    /// Die Tafel unten braucht zu jedem der sieben Wirkungsbereiche ein
    /// Kommando, denn [`zulaessig`] fragt nach einem Kommando und nicht nach
    /// einem Bereich. Die Paarung ist nicht behauptet: die Probe
    /// `jeder_stellvertreter_traegt_den_bereich_den_er_vertritt` haelt sie gegen
    /// [`Kommando::wirkungsbereich`], und sie haelt daneben fest, dass keiner
    /// der sieben eine der beiden Ausnahmen traegt. Ohne das zweite koennte ein
    /// Stellvertreter die drei abweisenden Viertel der Tafel gruen faerben,
    /// ohne dass die Regel sie traegt.
    ///
    /// Die Feldbreite steht in der Typangabe: ein achter Wirkungsbereich haelt
    /// den Bau dieser Probe an.
    const STELLVERTRETER: [(Wirkungsbereich, Kommando); 7] = [
        (Wirkungsbereich::Dateifenster, Kommando::Oeffnen),
        (Wirkungsbereich::Leiste, Kommando::LesezeichenLoeschen),
        (Wirkungsbereich::Vorschau, Kommando::EditorAusVorschau),
        (Wirkungsbereich::Editor, Kommando::EditorSichern),
        (Wirkungsbereich::Tabbereich, Kommando::TabNeu),
        (Wirkungsbereich::Navigator, Kommando::AuswahlHoch),
        (Wirkungsbereich::Ueberall, Kommando::LeisteUmschalten),
    ];

    /// Die Lage aus vier Werten, kurz geschrieben.
    ///
    /// Die Reihenfolge ist die der Felder: Blattstand, Ersthelferbefund,
    /// Schluesselfenster, Fokus.
    fn lage(
        blatt_steht: bool,
        ersthelfer_gehoert_appkit: bool,
        schluesselfenster_gehoert_krk: bool,
        fokus: Fokus,
    ) -> Lage {
        Lage {
            blatt_steht,
            ersthelfer_gehoert_appkit,
            schluesselfenster_gehoert_krk,
            fokus,
        }
    }

    /// Die eine Lage ohne jedes Hindernis: kein Blatt, ein Ersthelfer, der
    /// nicht AppKit gehoert, und KRKs eigenes Schluesselfenster.
    ///
    /// Die Reihenfolge ist die von [`lage`], ohne den Fokus.
    const OHNE_HINDERNIS: (bool, bool, bool) = (false, false, true);

    /// Die sieben Lagen, in denen mindestens eine der drei aufhebbaren
    /// Bedingungen einen Befehl aufhaelt.
    ///
    /// Alle acht Wahrheitskombinationen aus Blattstand, Ersthelferbefund und
    /// Schluesselfenster **ausser** [`OHNE_HINDERNIS`]. Die Liste steht
    /// ausgeschrieben und nicht gerechnet, aus demselben Grund wie die Tafel:
    /// eine gerechnete Menge waere die Umsetzung ein zweites Mal.
    const HINDERNISSE: [(bool, bool, bool); 7] = [
        (false, false, false),
        (false, true, false),
        (false, true, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ];

    #[test]
    fn jeder_stellvertreter_traegt_den_bereich_den_er_vertritt() {
        for (bereich, kommando) in STELLVERTRETER {
            assert_eq!(
                kommando.wirkungsbereich(),
                bereich,
                "{kommando:?} vertritt {bereich:?} nicht"
            );
            assert!(
                !immer_erreichbar(kommando),
                "{kommando:?} steht auf der Ausnahmeliste und taugt nicht als Stellvertreter"
            );
            assert!(
                !operationen::waehrend_blatt_erlaubt(kommando),
                "{kommando:?} kommt waehrend eines Blattes durch und taugt nicht als \
                 Stellvertreter"
            );
        }
    }

    /// Die ganze Regel auf einen Blick: sieben Wirkungsbereiche mal fuenf
    /// Fokuswerte mal zwei Blattstaende mal zwei Ersthelferbefunde mal zwei
    /// Schluesselfensterbefunde, also 280 Faelle.
    ///
    /// Die Tafel steht in der Form der Tafel aus [`super::super::fokus`], nur um
    /// drei Wahrheitswerte erweitert. Ein Achtel traegt die Zeilen des
    /// Fokusvorbehalts, die sieben uebrigen weisen jeden Befehl ab: ein Blatt
    /// haelt alles auf, was nicht waehrend eines Blattes erlaubt ist, ein
    /// Ersthelfer, der AppKit gehoert, ebenso, und ein fremdes
    /// Schluesselfenster ebenso. Dass die sieben Achtel wirklich leer sind und
    /// nicht bloss so aussehen, haengt an den Stellvertretern; die Probe
    /// darueber haelt ihre beiden Voraussetzungen fest.
    ///
    /// Die Pruefungen darunter zeigen einzelne Felder dieser Tafel mit ihrer
    /// Begruendung; die Tafel zeigt, dass keine Zeile und keine Spalte fehlt.
    #[test]
    fn die_tafel_aus_zweihundertachtzig_faellen_geht_auf() {
        // Eine Zeile je Wirkungsbereich; die Spalten stehen in der Reihenfolge
        // von JEDER_FOKUS: Dateifenster, Leiste, Vorschau, Editor, Anderswo.
        // Es ist dieselbe Tafel, die `fokus::wirkt` traegt, und sie steht hier
        // ausgeschrieben und nicht gerechnet: eine gerechnete Erwartung waere
        // die Umsetzung ein zweites Mal.
        const OHNE_SPERRE: [[bool; 5]; 7] = [
            [true, false, false, false, false],
            [false, true, false, false, false],
            [false, false, true, false, false],
            [false, false, false, true, false],
            [true, false, true, false, false],
            [true, true, true, false, false],
            [true, true, true, true, true],
        ];
        const ALLES_ABGEWIESEN: [[bool; 5]; 7] = [[false; 5]; 7];

        // blatt_steht, ersthelfer_gehoert_appkit, schluesselfenster_gehoert_krk,
        // und welches Achtel gilt.
        let achtel: [(bool, bool, bool, [[bool; 5]; 7]); 8] = [
            (false, false, true, OHNE_SPERRE),
            (false, false, false, ALLES_ABGEWIESEN),
            (false, true, true, ALLES_ABGEWIESEN),
            (false, true, false, ALLES_ABGEWIESEN),
            (true, false, true, ALLES_ABGEWIESEN),
            (true, false, false, ALLES_ABGEWIESEN),
            (true, true, true, ALLES_ABGEWIESEN),
            (true, true, false, ALLES_ABGEWIESEN),
        ];

        let mut geprueft = 0usize;
        for (blatt, ersthelfer, schluessel, tafel) in achtel {
            for ((_, kommando), zeile) in STELLVERTRETER.into_iter().zip(tafel) {
                for (fokus, erwartet) in JEDER_FOKUS.into_iter().zip(zeile) {
                    assert_eq!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        erwartet,
                        "{kommando:?} bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?}"
                    );
                    geprueft += 1;
                }
            }
        }
        assert_eq!(geprueft, 280, "die Tafel deckt nicht alle 280 Faelle ab");
    }

    /// Mit dem Fokus im Editor ist ein Befehl des Dateifensters unzulaessig.
    ///
    /// Der Bestandteil (3) allein, ohne Blatt und ohne Textfeld: `return` liegt
    /// auf [`Kommando::Oeffnen`], und mit der Schreibmarke im Editor soll es
    /// einen Zeilenumbruch setzen und keine Datei an das Standardprogramm
    /// uebergeben.
    #[test]
    fn ein_befehl_des_dateifensters_wirkt_im_editor_nicht() {
        assert!(!zulaessig(
            Kommando::Oeffnen,
            lage(false, false, true, Fokus::Editor)
        ));
        assert!(zulaessig(
            Kommando::Oeffnen,
            lage(false, false, true, Fokus::Dateifenster)
        ));
    }

    /// Beim Umbenennen direkt in der Liste ist derselbe Befehl unzulaessig,
    /// obwohl kein Blatt steht und der Fokus im Dateifenster liegt.
    ///
    /// **Der Fall, um dessentwillen die Regel drei Bestandteile hat.** Der
    /// Feldeditor der Namensspalte haelt den Ersthelferrang, es steht kein
    /// Blatt, und `Anwendungsdelegierter::fokus` antwortet `Dateifenster`. Ohne
    /// Bestandteil (2) waere hier jeder Befehl des Dateifensters freigegeben.
    #[test]
    fn beim_umbenennen_in_der_liste_wirkt_kein_befehl_des_dateifensters() {
        let umbenennung = lage(false, true, true, Fokus::Dateifenster);
        for kommando in [
            Kommando::Oeffnen,
            Kommando::AuswahlHoch,
            Kommando::AuswahlRunter,
            Kommando::MarkierungUmschalten,
            Kommando::FensterWechseln,
        ] {
            assert!(
                !zulaessig(kommando, umbenennung),
                "{kommando:?} kommt waehrend einer Umbenennung in der Liste durch"
            );
        }
    }

    /// Vor einem fremden Schluesselfenster wirkt auch ein fensterweiter Befehl
    /// nicht (C5.6).
    ///
    /// **Die eine Probe, die den Unterschied zwischen der Regel der Runde 7 und
    /// der dieser Runde zeigt.** Vor einem fremden Fenster antwortet
    /// `Anwendungsdelegierter::fokus` schon vorher [`Fokus::Anderswo`], und
    /// damit weist Bestandteil (3) jeden Befehl ab, dessen Wirkungsbereich ein
    /// Bereich ist. Uebrig bleibt allein die Zeile `Ueberall`, fuer die
    /// [`fokus::wirkt`](super::super::fokus::wirkt) in jedem Fokus ja sagt;
    /// dort und nur dort faellt der vierte Bestandteil ueberhaupt an. Ohne
    /// diese Probe zeigte keine der uebrigen ihn.
    ///
    /// Der Stellvertreter ist [`Kommando::LeisteUmschalten`], derselbe, den die
    /// Tafel fuer `Ueberall` fuehrt: er traegt den Bereich und steht nicht auf
    /// der Ausnahmeliste. Die zweite Zusicherung haelt fest, dass die neue
    /// Bedingung nichts anderes wegnimmt: vor KRKs eigenem Schluesselfenster
    /// kommt derselbe Befehl in jedem Fokus durch.
    #[test]
    fn vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl() {
        let kommando = Kommando::LeisteUmschalten;
        assert_eq!(kommando.wirkungsbereich(), Wirkungsbereich::Ueberall);
        assert!(!immer_erreichbar(kommando));

        for fokus in JEDER_FOKUS {
            assert!(
                !zulaessig(kommando, lage(false, false, false, fokus)),
                "{kommando:?} kommt vor einem fremden Schluesselfenster in {fokus:?} durch"
            );
            assert!(
                zulaessig(kommando, lage(false, false, true, fokus)),
                "{kommando:?} kommt vor KRKs eigenem Schluesselfenster in {fokus:?} nicht durch"
            );
        }
    }

    /// `beenden` und `fenster_schliessen` bleiben in jeder Lage erreichbar.
    ///
    /// Die Pruefung der Ausnahmeliste und zugleich ihre Herleitung: heute
    /// erreichen beide Befehle waehrend einer Umbenennung und waehrend eines
    /// Blattes ihr Ziel ueber den Menueeintrag, und die neue Regel naehme ihnen
    /// das ohne Ausnahme weg.
    ///
    /// **Der vierte Bestandteil steht seit der Runde 8 mit in der Schleife**,
    /// und die Probe haelt damit die Wahl aus
    /// `decisions/260813-1110_*_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`
    /// fest: `beenden` kommt auch vor einem fremden Schluesselfenster durch,
    /// also beendet Cmd+Q die Anwendung, waehrend der Ueber-Dialog steht.
    /// Ohne diese Zeile waere der Weg lautlos weg.
    #[test]
    fn die_ausnahmeliste_kommt_durch_blatt_und_textfeld() {
        for kommando in [Kommando::Beenden, Kommando::FensterSchliessen] {
            assert!(immer_erreichbar(kommando));
            for fokus in JEDER_FOKUS {
                for (blatt, ersthelfer, schluessel) in HINDERNISSE {
                    assert!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        "{kommando:?} kommt bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?} nicht durch"
                    );
                }
            }
        }
    }

    /// Die Ausnahmeliste hebt den Fokusvorbehalt **nicht** auf.
    ///
    /// **Was die Zusage genau sagt:** fuer einen Befehl auf der Liste
    /// entscheidet allein der dritte Bestandteil, und zwar in jeder Lage. Die
    /// Erwartung steht deshalb als [`fokus::wirkt`](super::super::fokus::wirkt)
    /// da und nicht als `true`.
    ///
    /// **Warum sie so und nicht mit einem Gegenbeispiel geprueft ist:** die
    /// Liste geht ueber [`Kommando`], und beide heutigen Eintraege tragen
    /// `Wirkungsbereich::Ueberall`, fuer das `wirkt` in jedem Fokus ja sagt. Ein
    /// Befehl mit einem engeren Bereich laesst sich nicht dazuerfinden. Traegt
    /// ein kuenftiger dritter Eintrag einen engeren Bereich, faellt der
    /// Unterschied an, und diese Probe misst ihn dann; heute haelt sie fest,
    /// dass die Rechnung ihn ueberhaupt stellt.
    #[test]
    fn die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf() {
        for kommando in Kommando::KENNUNGEN.map(|(kommando, _)| kommando) {
            if !immer_erreichbar(kommando) {
                continue;
            }
            for fokus in JEDER_FOKUS {
                for (blatt, ersthelfer, schluessel) in
                    HINDERNISSE.into_iter().chain([OHNE_HINDERNIS])
                {
                    assert_eq!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        fokus::wirkt(kommando.wirkungsbereich(), fokus),
                        "{kommando:?} bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?} haengt nicht allein am \
                         Wirkungsbereich"
                    );
                }
            }
        }
    }

    /// Waehrend eines Blattes kommt der Abbruch durch, und sonst nur die
    /// Ausnahmeliste.
    ///
    /// Der Durchgang geht ueber **alle** Kommandos und nicht ueber eine zweite
    /// Liste: die Zusage aus C2.7 handelt von jedem Eintrag des Menues.
    #[test]
    fn waehrend_eines_blattes_kommt_allein_der_abbruch_und_die_ausnahmeliste_durch() {
        let blatt = lage(true, false, true, Fokus::Anderswo);
        for kommando in Kommando::KENNUNGEN.map(|(kommando, _)| kommando) {
            let erwartet =
                operationen::waehrend_blatt_erlaubt(kommando) || immer_erreichbar(kommando);
            assert_eq!(
                zulaessig(kommando, blatt),
                erwartet,
                "{kommando:?} verhaelt sich waehrend eines Blattes anders als zugesagt"
            );
        }
    }

    /// Steht die Schreibmarke im Textfeld eines Blattes, ist auch der Abbruch
    /// abgewiesen.
    ///
    /// Die zweite Haelfte von C2.7: `esc` erreicht dann AppKit wie heute und
    /// schliesst das Blatt ueber dessen eigene Abbruchschaltflaeche.
    #[test]
    fn im_textfeld_eines_blattes_ist_auch_der_abbruch_abgewiesen() {
        assert!(zulaessig(
            Kommando::Abbrechen,
            lage(true, false, true, Fokus::Anderswo)
        ));
        assert!(!zulaessig(
            Kommando::Abbrechen,
            lage(true, true, true, Fokus::Anderswo)
        ));
    }
}
