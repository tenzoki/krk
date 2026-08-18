//! Was mit einem Abwurf aus einer fremden Anwendung geschieht: welches der
//! zwei Ziele gilt, und ob KRK ihn kopiert, verschiebt oder abweist
//! (C4, C5 und C6 der Runde 13).
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die Tatsachen beschafft die Ansicht — die
//! Ablage des Ziehvorgangs, das Schreibrecht des Ziels, die angebotene Menge,
//! der laufende Vorgang —, die Regel selbst steht hier und ist ohne Fenster
//! pruefbar. Das ist der Zuschnitt dieser Runde: alles, was ohne Ziehsitzung zu
//! entscheiden ist, wird von `cargo test` gehalten und nicht von der Hand des
//! Nutzers.
//!
//! ```text
//!  auf_die_zeile ────┬──> marke()  ──> Abwurfmarke (Zeile oder Liste)
//!  typ_der_zeile ────┘
//!
//!  traegt_dateien ───────┐
//!  vorgang_laeuft ───────┤
//!  schreibrecht ─────────┼──> urteil() ──> Abwurfurteil
//!  ziel_ist_quellordner ─┤                 (Kopieren, Verschieben, Abweisen)
//!  bietet_kopieren ──────┤
//!  bietet_verschieben ───┘
//! ```
//!
//! # Warum diese Regel eigens dasteht
//!
//! Aus demselben Grund wie die Tafel in [`super::rueckschritt`] (`:30-32`): als
//! Bedingungskette im Annahmezweig der Tabelle waere sie an keiner Probe zu
//! fassen. Der Annahmezweig lebt in `NSTableViewDataSource`, und ein
//! `NSDraggingInfo` laesst sich ohne Ziehsitzung nicht bauen; jede Zeile, die
//! dort entschiede, waere damit allein von Hand nachpruefbar. Hier drueben sind
//! es zwei reine Funktionen ueber Wahrheitswerte und zwei geschlossene
//! Aufzaehlungen.
//!
//! # Woran die Regel nicht haengt
//!
//! Drei Groessen liegen nahe und stehen bewusst **nicht** in der Signatur:
//!
//! - **Keine Zusatztaste.** KRK deutet weder `cmd` noch `opt` noch `shift`
//!   selbst. Das System verengt die Menge der angebotenen Vorgaenge bereits aus
//!   den gehaltenen Tasten, bevor KRK sie zu sehen bekommt; die Regel liest die
//!   Menge, die ihr gereicht wird, und schliesst nicht von Tasten auf sie
//!   zurueck. Wer nur `shift` laese, wollte kopieren, waehrend die Quelle nach
//!   einem gehaltenen `cmd` allein das Verschieben anbietet — der Zeiger zeigte
//!   das eine und KRK taete das andere. Der tragende Datensatz ist
//!   `shared/decisions/260818-1453_*_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md`.
//! - **Kein `NSDragOperation`.** Die Menge des Systems wird **einmal** in die
//!   zwei Wahrheitswerte dieser Regel uebersetzt, in `abwurf::angebot`, und die
//!   Rueckrichtung ebenfalls einmal, in `abwurf::zeiger`. Stuende die
//!   Uebersetzung hier, stuende sie zweimal im Baum.
//! - **Nicht, warum eine Ablage keine Dateien liefert.** [`Abwurflage`] traegt
//!   `traegt_dateien` und keinen Grund dahinter. Ob die abgebende Anwendung
//!   eine Zusagedatei anbietet, ihre Daten nur im Speicher haelt oder etwas
//!   Drittes tut, ist aus der Ablage nicht zu belegen: KRK **misst**, ob
//!   Dateiverweise kommen, statt eine fremde Anwendung einzuordnen (C7).
//!
//! # Warum `Schreibrecht::Unbekannt` durchlaesst
//!
//! Nur ein **gemessenes** `false` weist ab. Ein Schreibrecht, das sich nicht
//! feststellen laesst — kein gueltiges UTF-8 im Pfad, ein Fehler bei der
//! Abfrage, ein fehlender Wert —, nimmt den Abwurf an.
//!
//! **Das ist eine Festlegung des Nutzers und keine Ableitung**, und sie steht
//! gegen die Zusage, die die Runde 12 fuer den Loeschweg gegeben hat:
//! „Unentschieden gilt als laut". Jene Zusage kaufte Sicherheit gegen eine
//! **sichtbare** Rueckfrage. Hier gaebe es dafuer nur ein stummes
//! Verbotszeichen am Zeiger, ohne einen Satz dazu, warum; ein Ordner, den KRK
//! nicht einordnen kann, waere damit ohne Erklaerung unbenutzbar. Eine Regel,
//! die den Nutzer nicht erreicht, schuetzt ihn nicht, sie hindert ihn bloss.
//! Was danach doch scheitert, wird nachtraeglich entschieden und nicht
//! vorhergesagt: der Eintrag erscheint mit seinem Grund in der Abschlussliste
//! des Vorgangs, auf demselben Weg, den F5 und F6 heute gehen.
//!
//! Der Datensatz ist
//! `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/decisions/260818-1633_*_gilt-ein-unentscheidbares-schreibrecht-beim-abwurf-als-erlaubnis-oder-als-abweisung.md`,
//! beantwortet am Plan-Gate der Runde 13. Der Uebergang auf die
//! entgegengesetzte Antwort kostet eine Zeile in der Tafel von [`urteil`].
//!
//! # Die zwei Aufrufer
//!
//! `DateifensterQuelle::abwurf_pruefen` (`crate::appkit::tabelle`) ruft beide
//! Funktionen, und je eine Aufruferzaehlung haelt die Zahl fest. **Bis der
//! Aufrufer stand, erwarteten beide Proben null** und die Stuecke unten trugen
//! `#[cfg_attr(not(test), expect(dead_code, ...))]`. Beides gehoert zusammen:
//! eine Probe, die schon eins erwartet haette, waere bis dahin rot gewesen, und
//! eine mit „hoechstens eins" fuer immer gruen und ohne Aussage. `expect` und
//! nicht `allow`, damit die Ausnahme ihr Ablaufdatum selbst durchsetzt — mit
//! dem Aufrufer wird die Erwartung unerfuellt, und der Bau haelt unter
//! `-D warnings` an, bis die Zeilen weg sind.

use krk_core::verzeichnis::Typ;

/// Welche der zwei Marken aus C4 waehrend des Ziehens steht.
///
/// Zwei Werte und nicht ein Wahrheitswert, weil beide Marken zugleich sagen,
/// **welcher Ordner das Ziel ist**: die hervorgehobene Zeile meint den Ordner
/// dieser Zeile, die umrandete Liste den angezeigten Ordner. Eine Marke ohne
/// ihr Ziel waere Zierat, und ein Ziel ohne seine Marke machte den Abwurf zum
/// Ratespiel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abwurfmarke {
    /// Die Ordnerzeile ist hervorgehoben; das Ziel ist dieser Ordner.
    Zeile,
    /// Die ganze Liste ist umrandet; das Ziel ist der angezeigte Ordner.
    Liste,
}

/// Was KRK ueber das Schreibrecht des Ziels **gemessen** hat.
///
/// Drei Werte, weil `Unbekannt` und `Nein` verschieden behandelt werden:
/// `Nein` weist ab, `Unbekannt` laesst durch. Der Grund steht im Modulkopf
/// unter „Warum `Schreibrecht::Unbekannt` durchlaesst"; wer die beiden Werte
/// zusammenlegt, entscheidet jene Frage nebenbei mit.
///
/// **Diese Aufzaehlung wird hier gelesen und nirgends gebaut**, und deshalb
/// traegt sie ihre eigene Ausnahme statt der beiden Funktionen unten: gebaut
/// wird sie in `abwurf::beschreibbarkeit`, also in Schritt 8 und nicht in
/// Schritt 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "die drei Werte entstehen ab Schritt 8 dieser Runde in \
                  abwurf::beschreibbarkeit; mit ihm wird die Erwartung \
                  unerfuellt und diese Zeile faellt"
    )
)]
pub enum Schreibrecht {
    /// `NSURLIsWritableKey` hat `true` geliefert.
    Ja,
    /// `NSURLIsWritableKey` hat `false` geliefert. Der einzige Wert, der
    /// abweist.
    Nein,
    /// Die Frage blieb ohne Antwort: kein gueltiges UTF-8 im Pfad, ein Fehler
    /// bei der Abfrage oder ein fehlender Wert.
    Unbekannt,
}

/// Warum ein Abwurf abgewiesen wird.
///
/// Fuenf Gruende, und **nur [`Abwurfgrund::KeineDatei`] traegt eine Meldung**
/// in die Statuszeile (C7). Die vier anderen zeigen sich allein am Zeiger, weil
/// der Nutzer in ihnen sieht, was los ist: ein laufender Vorgang steht in
/// derselben Statuszeile, der Quellordner steht im anderen Fenster vor ihm.
/// Dass die Gruende trotzdem einzeln benannt sind, ist kein Vorrat: die Ansicht
/// entdoppelt ihre Meldung ueber den Vergleich zweier Gruende und braucht dafuer
/// den Unterschied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abwurfgrund {
    /// Die Ablage liefert keinen Dateiverweis. Der einzige Grund mit Meldung.
    KeineDatei,
    /// KRK haelt schon einen Vorgang, und es haelt genau einen.
    VorgangLaeuft,
    /// Der Zielordner hat ein gemessenes `false` als Schreibrecht.
    NichtBeschreibbar,
    /// Ziel und Quelle sind derselbe Ordner. Dieselbe Antwort, die
    /// `auftrag_stellen` fuer F5 und F6 gibt.
    SelberOrdner,
    /// Die Quelle bietet weder Kopieren noch Verschieben an.
    KeinAngebot,
}

/// Der Vorgang, in den ein angenommener Abwurf muendet.
///
/// Er traegt keine eigene Auftragsart in die Operationsmaschine: die Ansicht
/// setzt ihn in `Auftrag::kopieren` beziehungsweise `Auftrag::verschieben` um,
/// also in dieselben zwei Erzeuger, die F5 und F6 benutzen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abwurfvorgang {
    /// Die Quellen bleiben liegen, wo sie liegen.
    Kopieren,
    /// Die Quellen sind danach an ihrem alten Ort verschwunden.
    Verschieben,
}

/// Der Ausgang der Regel: ausfuehren mit einem Vorgang, oder abweisen mit einem
/// Grund.
///
/// Zwei Zweige und kein dritter. Ein „vielleicht" gibt es nicht, weil der
/// Zeiger in jedem Augenblick genau eines von beiden zeigen muss und weil das,
/// was er zeigt, mit dem uebereinstimmen soll, was nach dem Loslassen geschieht
/// (C5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abwurfurteil {
    /// KRK nimmt an und faehrt diesen Vorgang.
    Ausfuehren(Abwurfvorgang),
    /// KRK weist ab. Nichts geschieht beim Loslassen.
    Abweisen(Abwurfgrund),
}

/// Die sechs Tatsachen, die [`urteil`] braucht, und keine siebte.
///
/// Jede hat genau einen Beschaffer, und keine wird zweimal erhoben:
/// `traegt_dateien` kommt aus `zwischenablage::dateiverweise`, `vorgang_laeuft`
/// aus `Anwendungsdelegierter::vorgang_laeuft`, `schreibrecht` aus
/// `abwurf::beschreibbarkeit`, `ziel_ist_quellordner` aus dem Vergleich zweier
/// Pfade, und die beiden letzten aus `abwurf::angebot`.
///
/// **Sie steht als Struktur und nicht als sechs Parameter da**, weil sechs
/// Wahrheitswerte in einer Reihe an der Aufrufstelle nicht mehr zu lesen sind
/// und ein vertauschtes Paar der Uebersetzung nicht auffiele. Die drei
/// Groessen, die bewusst **fehlen**, nennt der Modulkopf.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abwurflage {
    /// Ob die Ablage des Ziehvorgangs mindestens einen Dateiverweis liefert.
    pub traegt_dateien: bool,
    /// Ob KRK gerade schon einen Vorgang haelt.
    pub vorgang_laeuft: bool,
    /// Was ueber das Schreibrecht des Zielordners gemessen wurde.
    pub schreibrecht: Schreibrecht,
    /// Ob der Zielordner derselbe ist, aus dem gezogen wird.
    pub ziel_ist_quellordner: bool,
    /// Ob die angebotene Menge das Kopieren enthaelt.
    pub bietet_kopieren: bool,
    /// Ob die angebotene Menge das Verschieben enthaelt.
    pub bietet_verschieben: bool,
}

/// Welche Marke waehrend des Ziehens steht, und damit, welcher Ordner das Ziel
/// ist (C4).
///
/// `auf_die_zeile` ist wahr, wenn der Zeiger auf einer Zeile steht und nicht
/// zwischen zweien; `typ_der_zeile` ist `None`, wenn die vorgeschlagene Zeile
/// keinen Eintrag benennt — die leere Flaeche unter der letzten Zeile.
///
/// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht gerechnet:
///
/// | `auf_die_zeile` | `typ_der_zeile` | Marke | Zeile aus C4 |
/// |---|---|---|---|
/// | ja | `Some(Typ::Ordner)` | [`Abwurfmarke::Zeile`] | ueber einer Ordnerzeile |
/// | ja | `Some(Typ::Datei)` | [`Abwurfmarke::Liste`] | ueber einer Dateizeile |
/// | ja | `Some(Typ::Verknuepfung)` | [`Abwurfmarke::Liste`] | ueber einer Verknuepfungszeile |
/// | ja | `None` | [`Abwurfmarke::Liste`] | ein Index ausserhalb der Liste |
/// | nein | gleichgueltig | [`Abwurfmarke::Liste`] | zwischen zwei Zeilen oder unter der letzten |
///
/// **Fuenf Zeilen ueber acht Kombinationen**, ohne Auffangzweig; [`Typ`] ist
/// geschlossen, also haelt der Uebersetzer die Vollstaendigkeit, und eine
/// vierte Eintragsart hielte den Bau an. Die Probe
/// `die_tafel_der_marke_geht_auf` schreibt alle acht aus, aus demselben Grund,
/// aus dem die Tafel in [`super::rueckschritt`] ausgeschrieben dasteht: eine
/// gerechnete Erwartung waere die Umsetzung ein zweites Mal.
///
/// **Eine Dateizeile ist keine Abweisung**, und der Sprung der Marke von der
/// Zeile auf die ganze Liste ist genau das, woran der Nutzer vor dem Loslassen
/// sieht, dass die Datei nicht das Ziel ist.
///
/// **Eine Verknuepfung auf einen Ordner zaehlt nicht als Ordner.**
/// `verweisziel::bestimmen` steht ausdruecklich nicht in dieser Tafel: eine
/// Verknuepfung in der Liste ist sie selbst, und was hinter ihr liegt, gehoert
/// ihr nicht. Dieselbe Festlegung hat die Loeschrunde fuer die Zaehlung des
/// Umfangs getroffen.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: verlorenginge dabei nicht nur die Marke, sondern die Bestimmung des
/// Zielordners, und die Ansicht entschiede sie ein zweites Mal auf eigene
/// Faust — eine zweite Wahrheit ueber dieselbe Sache.
#[must_use]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der eine Aufrufer entsteht in Schritt 10 dieser Runde, in \
                  DateifensterQuelle::abwurf_pruefen; mit ihm wird die \
                  Erwartung unerfuellt und diese Zeile faellt"
    )
)]
pub fn marke(auf_die_zeile: bool, typ_der_zeile: Option<Typ>) -> Abwurfmarke {
    match (auf_die_zeile, typ_der_zeile) {
        // Ueber einer Ordnerzeile ist dieser Ordner das Ziel, und die Zeile
        // sagt es.
        (true, Some(Typ::Ordner)) => Abwurfmarke::Zeile,
        // Eine Datei ist kein Ziel; die Marke springt auf die ganze Liste, und
        // der Abwurf landet im angezeigten Ordner.
        (true, Some(Typ::Datei)) => Abwurfmarke::Liste,
        // Eine Verknuepfung wird als sie selbst behandelt, auch wenn sie auf
        // einen Ordner zeigt.
        (true, Some(Typ::Verknuepfung)) => Abwurfmarke::Liste,
        // Ein Zeilenindex, der keinen Eintrag benennt: die leere Flaeche unter
        // der letzten Zeile.
        (true, None) => Abwurfmarke::Liste,
        // Zwischen zwei Zeilen gibt es keinen Eintrag, auf den zu zielen waere.
        (false, _) => Abwurfmarke::Liste,
    }
}

/// Ob KRK diesen Abwurf ausfuehrt, und wenn nicht, warum nicht (C5 und C6).
///
/// Der Rumpf sind diese zwei Tafeln. Die erste ist die Reihenfolge aus C6, und
/// ihre letzte Zeile verzweigt in die zweite:
///
/// | `traegt_dateien` | `vorgang_laeuft` | `schreibrecht` | `ziel_ist_quellordner` | Ausgang |
/// |---|---|---|---|---|
/// | nein | gleichgueltig | gleichgueltig | gleichgueltig | `Abweisen(KeineDatei)` |
/// | ja | ja | gleichgueltig | gleichgueltig | `Abweisen(VorgangLaeuft)` |
/// | ja | nein | `Nein` | gleichgueltig | `Abweisen(NichtBeschreibbar)` |
/// | ja | nein | `Ja` oder `Unbekannt` | ja | `Abweisen(SelberOrdner)` |
/// | ja | nein | `Ja` oder `Unbekannt` | nein | die zweite Tafel |
///
/// **Fuenf Arme ueber 24 Kombinationen der vier ersten Groessen**, kein
/// Auffangzweig. `Ja` und `Unbekannt` stehen in derselben Zeile
/// **ausgeschrieben** und nicht zu einem `_` zusammengefasst, damit ein vierter
/// Wert von [`Schreibrecht`] den Bau anhaelt statt still durchzulaufen.
///
/// Die zweite Tafel ist die aus C5, und sie liest die Menge, die die Quelle
/// anbietet:
///
/// | `bietet_kopieren` | `bietet_verschieben` | Ausgang | die Lage, die dazu fuehrt |
/// |---|---|---|---|
/// | ja | ja | `Ausfuehren(Kopieren)` | der Nutzer haelt nichts oder `shift` |
/// | ja | nein | `Ausfuehren(Kopieren)` | der Nutzer haelt `opt` |
/// | nein | ja | `Ausfuehren(Verschieben)` | der Nutzer haelt `cmd` |
/// | nein | nein | `Abweisen(KeinAngebot)` | die Quelle bietet keines von beiden |
///
/// **Die vierte Spalte steht im Doc-Kommentar und nicht im Code.** KRK liest
/// keine Taste; eine Fallunterscheidung nach Tasten stuende hier an einer
/// Stelle, an der sie nichts entscheidet. Sie steht dort, weil der Leser die
/// Tabelle des Spec wiederfinden koennen muss.
///
/// Die beiden Proben `die_tafel_der_abweisungen_geht_auf` und
/// `die_tafel_des_angebots_geht_auf` schreiben beide Tafeln in allen ihren
/// Kombinationen ein drittes Mal aus, und
/// `die_vier_abweisungen_fragen_das_angebot_nicht` misst, dass die zwei Tafeln
/// wirklich unabhaengig sind.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: der Abwurf liefe dann ohne Urteil, und mit ihm fielen alle fuenf
/// Abweisungen aus C6 und C7 weg, ohne dass irgendetwas rot wuerde.
#[must_use]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der eine Aufrufer entsteht in Schritt 10 dieser Runde, in \
                  DateifensterQuelle::abwurf_pruefen; mit ihm wird die \
                  Erwartung unerfuellt und diese Zeile faellt"
    )
)]
pub fn urteil(lage: &Abwurflage) -> Abwurfurteil {
    match (
        lage.traegt_dateien,
        lage.vorgang_laeuft,
        lage.schreibrecht,
        lage.ziel_ist_quellordner,
    ) {
        // Die Ablage liefert keinen Dateiverweis. Der einzige Ausgang, der eine
        // Meldung nach sich zieht (C7).
        (false, _, _, _) => Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
        // KRK haelt genau einen Vorgang. Dieselbe Frage, die
        // `vorgang_laeuft_schon` fuer F5 und F6 beantwortet, hier ohne deren
        // Meldung: `validateDrop:` laeuft bei jeder Zeigerbewegung.
        (true, true, _, _) => Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
        // Ein gemessenes `false`, und nur dieses, weist ab.
        (true, false, Schreibrecht::Nein, _) => {
            Abwurfurteil::Abweisen(Abwurfgrund::NichtBeschreibbar)
        }
        // Ziel und Quelle sind derselbe Ordner; `auftrag_stellen` gibt dieselbe
        // Antwort. `Unbekannt` steht hier neben `Ja` und laesst durch.
        (true, false, Schreibrecht::Ja | Schreibrecht::Unbekannt, true) => {
            Abwurfurteil::Abweisen(Abwurfgrund::SelberOrdner)
        }
        // Nichts spricht dagegen: jetzt entscheidet die Menge, die die Quelle
        // anbietet, und sonst nichts.
        (true, false, Schreibrecht::Ja | Schreibrecht::Unbekannt, false) => {
            match (lage.bietet_kopieren, lage.bietet_verschieben) {
                // Beides im Angebot: die Vorgabe ist der nicht zerstoererische
                // Vorgang.
                (true, true) => Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
                // Die Quelle bietet allein das Kopieren an.
                (true, false) => Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
                // Kein Kopieren im Angebot, aber ein Verschieben.
                (false, true) => Abwurfurteil::Ausfuehren(Abwurfvorgang::Verschieben),
                // Die Quelle bietet keinen der zwei Vorgaenge an.
                (false, false) => Abwurfurteil::Abweisen(Abwurfgrund::KeinAngebot),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Die Kiste, in der ein Aufrufer ueberhaupt stehen kann.
    ///
    /// **Die zwei Zaehlungen darunter lesen `krk-ui` und nicht den ganzen
    /// Baum**, und das ist keine stille Verengung, sondern der genaue Umfang
    /// der Zusage: `krk-ui` hat kein Bibliotheksziel (`Cargo.toml` fuehrt allein
    /// `[[bin]] name = "krk"`), also erreicht keine andere Kiste dieses Modul,
    /// und ein Aufrufer ausserhalb dieses Praefixes kann es nicht geben.
    ///
    /// Fuer [`urteil`] ist die Einschraenkung ausserdem noetig: `krk-bench`
    /// fuehrt unter demselben Namen eine eigene Funktion, die das Urteil einer
    /// Zeitzusage formuliert (`krk-bench/src/messen.rs`). Sie entscheidet eine
    /// voellig andere Frage, und [`aufrufstellen`] unterscheidet Namensgleiches
    /// nicht — ohne das Praefix zaehlte die Probe deren fuenf Aufrufstellen mit.
    ///
    /// **Was damit blind bleibt**, in der Form, die [`crate::quellbaum`]
    /// verlangt: ein Aufruf unter einem anderen Namen (`use ... as anders`)
    /// wird nicht gesehen, und eine zweite Funktion `marke` oder `urteil`
    /// **innerhalb** von `krk-ui` wuerde als Aufrufer mitgezaehlt statt als
    /// Doppelbau erkannt. Die Aufruferzaehlung sagt, wie viele Stellen rufen,
    /// und nicht, wie viele Fassungen es gibt.
    const KISTE: &str = "krk-ui/";

    /// Diese Datei bleibt aussen vor: die Tafeln darunter rufen beide
    /// Funktionen vielfach, und das sind keine Aufrufer im Sinne der Zusage.
    const ZUHAUSE: &str = "krk-ui/src/kommandos/abwurfregel.rs";

    /// Zaehlt die Aufrufstellen von `name` in `krk-ui`, ohne diese Datei.
    fn aufrufer(name: &str) -> usize {
        quelldateien()
            .iter()
            .filter(|(datei, _)| datei.starts_with(KISTE) && datei != ZUHAUSE)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum()
    }

    /// Die Bestimmung der Marke hat genau einen Aufrufer (C4).
    ///
    /// **Eine Aufruferzaehlung in der Form von
    /// `die_regel_hat_genau_einen_aufrufer` in [`super::super::rueckschritt`]**,
    /// und sie steht hier aus demselben Grund: die Zusage handelt davon, dass
    /// aus einer Zeilennummer an genau einer Stelle ein Ziel wird. Ein zweiter
    /// Aufrufer waere eine zweite Antwort auf die Frage, welcher Ordner das
    /// Ziel ist, und `abwurf_pruefen` und `abwurf_annehmen` muessten dieselbe
    /// geben.
    ///
    /// **Bis Schritt 10 dieser Runde ist die erwartete Zahl null**, und das ist
    /// die einzige Zahl, die jetzt richtig ist: eine Probe, die schon eins
    /// erwartete, waere heute rot, und eine mit „hoechstens eins" waere fuer
    /// immer gruen und maesse nichts. Schritt 10 setzt sie auf eins.
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt,
    /// den sie liest.
    #[test]
    fn die_marke_hat_noch_keinen_aufrufer() {
        let name = concat!("mar", "ke");
        assert_eq!(
            aufrufer(name),
            0,
            "die Bestimmung der Abwurfmarke hat nicht genau null Aufrufer; \
             ab Schritt 10 der Runde 13 ist die erwartete Zahl eins"
        );
    }

    /// Das Urteil hat genau einen Aufrufer (C5, C6).
    ///
    /// Dieselbe Bauform und derselbe Grund wie eine Probe darueber: das Urteil
    /// wird waehrend des Ziehens gefaellt und beim Loslassen nicht wiederholt.
    /// AppKit ruft `acceptDrop:` nur, wenn `validateDrop:` einen Vorgang
    /// zurueckgegeben hat; ein zweiter Aufrufer waere eine zweite Beurteilung
    /// derselben Lage, die anders ausfallen koennte als die, die der Zeiger
    /// gezeigt hat.
    ///
    /// **Bis Schritt 10 dieser Runde ist die erwartete Zahl null.**
    #[test]
    fn das_urteil_hat_noch_keinen_aufrufer() {
        let name = concat!("urt", "eil");
        assert_eq!(
            aufrufer(name),
            0,
            "das Abwurfurteil hat nicht genau null Aufrufer; ab Schritt 10 der \
             Runde 13 ist die erwartete Zahl eins"
        );
    }

    /// Die ganze Tafel der Marke auf einen Blick: zwei Zeigerlagen mal vier
    /// Zeilenbefunde, also acht Faelle.
    ///
    /// Sie schreibt aus, was die fuenfte Zeile der Dokumentation von [`marke`]
    /// mit „gleichgueltig" zusammenfasst, und zeigt, dass keine Kombination
    /// fehlt und keine zweimal beantwortet wird. Die Erwartungen stehen als
    /// Werte da und nicht als Rechnung: eine gerechnete Erwartung waere die
    /// Umsetzung ein zweites Mal.
    #[test]
    fn die_tafel_der_marke_geht_auf() {
        // auf_die_zeile, typ_der_zeile, Marke.
        const TAFEL: [(bool, Option<Typ>, Abwurfmarke); 8] = [
            (true, Some(Typ::Ordner), Abwurfmarke::Zeile),
            (true, Some(Typ::Datei), Abwurfmarke::Liste),
            (true, Some(Typ::Verknuepfung), Abwurfmarke::Liste),
            (true, None, Abwurfmarke::Liste),
            (false, Some(Typ::Ordner), Abwurfmarke::Liste),
            (false, Some(Typ::Datei), Abwurfmarke::Liste),
            (false, Some(Typ::Verknuepfung), Abwurfmarke::Liste),
            (false, None, Abwurfmarke::Liste),
        ];

        for (auf_die_zeile, typ_der_zeile, erwartet) in TAFEL {
            assert_eq!(
                marke(auf_die_zeile, typ_der_zeile),
                erwartet,
                "auf_die_zeile={auf_die_zeile}, typ_der_zeile={typ_der_zeile:?}"
            );
        }
    }

    /// Genau ein Feld der Tafel traegt [`Abwurfmarke::Zeile`], und es ist die
    /// Ordnerzeile unter dem Zeiger (C4).
    ///
    /// Die Probe steht neben der Tafel und nicht in ihr, weil sie eine andere
    /// Aussage macht: nicht „jedes Feld stimmt", sondern „nur ein einziges Feld
    /// hebt eine Zeile hervor". Faerbte eine spaetere Aenderung auch die
    /// Verknuepfungszeile ein, bliebe die Tafel oben nur deshalb rot, weil
    /// jemand sie mitgeaendert hat; hier wuerde die Zusage selbst rot.
    #[test]
    fn allein_die_ordnerzeile_wird_hervorgehoben() {
        assert_eq!(marke(true, Some(Typ::Ordner)), Abwurfmarke::Zeile);
        for typ in [Typ::Datei, Typ::Verknuepfung] {
            assert_eq!(
                marke(true, Some(typ)),
                Abwurfmarke::Liste,
                "eine Zeile vom Typ {typ:?} wird hervorgehoben"
            );
        }
    }

    /// Eine Verknuepfung wird als sie selbst behandelt (C4).
    ///
    /// Die Probe schreibt aus, was der Doc-Kommentar von [`marke`] als
    /// Festlegung fuehrt: die Verknuepfungszeile verhaelt sich wie eine
    /// Dateizeile, gleichgueltig worauf sie zeigt. Wer spaeter
    /// `verweisziel::bestimmen` in die Regel zoege, macht sie rot — und das ist
    /// ihr Zweck, denn die Signatur von [`marke`] laesst diesen Einbau gar
    /// nicht zu, sondern nur die Uebergabe eines aufgeloesten Typs von aussen.
    #[test]
    fn eine_verknuepfung_verhaelt_sich_wie_eine_datei() {
        assert_eq!(
            marke(true, Some(Typ::Verknuepfung)),
            marke(true, Some(Typ::Datei)),
            "die Verknuepfungszeile verhaelt sich nicht wie eine Dateizeile"
        );
    }

    /// Eine Lage, in der nichts abweist, als Ausgangspunkt der Proben darunter.
    ///
    /// Der gewoehnliche Fall: eine Datei aus dem Finder, kein laufender
    /// Vorgang, ein beschreibbares Ziel, ein anderer Ordner, und beide
    /// Vorgaenge im Angebot.
    const fn durchlassende_lage() -> Abwurflage {
        Abwurflage {
            traegt_dateien: true,
            vorgang_laeuft: false,
            schreibrecht: Schreibrecht::Ja,
            ziel_ist_quellordner: false,
            bietet_kopieren: true,
            bietet_verschieben: true,
        }
    }

    /// Die erste Tafel vollstaendig: zwei mal zwei mal drei mal zwei, also 24
    /// Faelle.
    ///
    /// Das Angebot steht in allen 24 Zeilen auf „beides", dem gewoehnlichen
    /// Fall aus dem Finder; dass die vier Abweisungen es ohnehin nicht lesen,
    /// misst `die_vier_abweisungen_fragen_das_angebot_nicht` daneben. Damit
    /// stehen die beiden Tafeln in den Proben so nebeneinander wie im
    /// Doc-Kommentar von [`urteil`], statt sich zu 96 Zeilen zu multiplizieren.
    ///
    /// Die Erwartungen stehen ausgeschrieben da. Die Reihenfolge der Gruende
    /// ist die Aussage der Tafel: `KeineDatei` schlaegt `VorgangLaeuft`,
    /// dieses `NichtBeschreibbar`, dieses `SelberOrdner`.
    #[test]
    fn die_tafel_der_abweisungen_geht_auf() {
        // traegt_dateien, vorgang_laeuft, schreibrecht, ziel_ist_quellordner,
        // Ausgang bei beidem im Angebot.
        const TAFEL: [(bool, bool, Schreibrecht, bool, Abwurfurteil); 24] = [
            // Ohne Dateiverweis ist alles andere gleichgueltig.
            (
                false,
                false,
                Schreibrecht::Ja,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                false,
                Schreibrecht::Ja,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                false,
                Schreibrecht::Nein,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                false,
                Schreibrecht::Nein,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                false,
                Schreibrecht::Unbekannt,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                false,
                Schreibrecht::Unbekannt,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Ja,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Ja,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Nein,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Nein,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Unbekannt,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            (
                false,
                true,
                Schreibrecht::Unbekannt,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::KeineDatei),
            ),
            // Ein laufender Vorgang schlaegt Schreibrecht und Ordnervergleich.
            (
                true,
                true,
                Schreibrecht::Ja,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            (
                true,
                true,
                Schreibrecht::Ja,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            (
                true,
                true,
                Schreibrecht::Nein,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            (
                true,
                true,
                Schreibrecht::Nein,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            (
                true,
                true,
                Schreibrecht::Unbekannt,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            (
                true,
                true,
                Schreibrecht::Unbekannt,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::VorgangLaeuft),
            ),
            // Ein gemessenes `Nein` schlaegt den Ordnervergleich.
            (
                true,
                false,
                Schreibrecht::Nein,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::NichtBeschreibbar),
            ),
            (
                true,
                false,
                Schreibrecht::Nein,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::NichtBeschreibbar),
            ),
            // Derselbe Ordner, mit `Ja` und mit `Unbekannt`.
            (
                true,
                false,
                Schreibrecht::Ja,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::SelberOrdner),
            ),
            (
                true,
                false,
                Schreibrecht::Unbekannt,
                true,
                Abwurfurteil::Abweisen(Abwurfgrund::SelberOrdner),
            ),
            // Nichts weist ab: `Unbekannt` laesst durch wie `Ja`.
            (
                true,
                false,
                Schreibrecht::Ja,
                false,
                Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            ),
            (
                true,
                false,
                Schreibrecht::Unbekannt,
                false,
                Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            ),
        ];

        for (traegt_dateien, vorgang_laeuft, schreibrecht, ziel_ist_quellordner, erwartet) in TAFEL
        {
            let lage = Abwurflage {
                traegt_dateien,
                vorgang_laeuft,
                schreibrecht,
                ziel_ist_quellordner,
                ..durchlassende_lage()
            };
            assert_eq!(
                urteil(&lage),
                erwartet,
                "traegt_dateien={traegt_dateien}, vorgang_laeuft={vorgang_laeuft}, \
                 schreibrecht={schreibrecht:?}, \
                 ziel_ist_quellordner={ziel_ist_quellordner}"
            );
        }
    }

    /// Die zweite Tafel vollstaendig: vier Kombinationen der angebotenen Menge
    /// (C5).
    ///
    /// Sie wird nur erreicht, wenn keine der vier Abweisungen greift; die Lage
    /// ist deshalb in allen vier Zeilen die durchlassende. Die vierte Spalte
    /// des Doc-Kommentars — welche Taste zu welcher Menge fuehrt — steht hier
    /// bewusst nicht: die Regel liest keine Taste, und eine Probe darueber
    /// pruefte das System und nicht KRK.
    #[test]
    fn die_tafel_des_angebots_geht_auf() {
        // bietet_kopieren, bietet_verschieben, Ausgang.
        const TAFEL: [(bool, bool, Abwurfurteil); 4] = [
            (
                true,
                true,
                Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            ),
            (
                true,
                false,
                Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            ),
            (
                false,
                true,
                Abwurfurteil::Ausfuehren(Abwurfvorgang::Verschieben),
            ),
            (
                false,
                false,
                Abwurfurteil::Abweisen(Abwurfgrund::KeinAngebot),
            ),
        ];

        for (bietet_kopieren, bietet_verschieben, erwartet) in TAFEL {
            let lage = Abwurflage {
                bietet_kopieren,
                bietet_verschieben,
                ..durchlassende_lage()
            };
            assert_eq!(
                urteil(&lage),
                erwartet,
                "bietet_kopieren={bietet_kopieren}, \
                 bietet_verschieben={bietet_verschieben}"
            );
        }
    }

    /// Die zwei Tafeln sind wirklich unabhaengig: keine der vier Abweisungen
    /// liest das Angebot.
    ///
    /// **Das ist die Zusage, die
    /// `die_tafel_der_abweisungen_geht_auf` schuldig bleibt**, weil sie das
    /// Angebot in allen 24 Zeilen fest auf „beides" stellt. Zusammen decken die
    /// beiden Proben die 96 Kombinationen der sechs Groessen ab, ohne dass eine
    /// Tafel mit 96 Zeilen dastuende.
    #[test]
    fn die_vier_abweisungen_fragen_das_angebot_nicht() {
        // Je eine Lage, die genau einen der vier Gruende ausloest.
        let ausloeser = [
            (
                Abwurfgrund::KeineDatei,
                Abwurflage {
                    traegt_dateien: false,
                    ..durchlassende_lage()
                },
            ),
            (
                Abwurfgrund::VorgangLaeuft,
                Abwurflage {
                    vorgang_laeuft: true,
                    ..durchlassende_lage()
                },
            ),
            (
                Abwurfgrund::NichtBeschreibbar,
                Abwurflage {
                    schreibrecht: Schreibrecht::Nein,
                    ..durchlassende_lage()
                },
            ),
            (
                Abwurfgrund::SelberOrdner,
                Abwurflage {
                    ziel_ist_quellordner: true,
                    ..durchlassende_lage()
                },
            ),
        ];

        for (grund, lage) in ausloeser {
            for bietet_kopieren in [false, true] {
                for bietet_verschieben in [false, true] {
                    let lage = Abwurflage {
                        bietet_kopieren,
                        bietet_verschieben,
                        ..lage
                    };
                    assert_eq!(
                        urteil(&lage),
                        Abwurfurteil::Abweisen(grund),
                        "die Abweisung {grund:?} haengt am Angebot: \
                         bietet_kopieren={bietet_kopieren}, \
                         bietet_verschieben={bietet_verschieben}"
                    );
                }
            }
        }
    }

    /// Ein unentscheidbares Schreibrecht laesst durch, und nur ein gemessenes
    /// `Nein` weist ab.
    ///
    /// **Diese Probe ist der Grund, aus dem [`Schreibrecht`] drei Werte hat.**
    /// Wer `Unbekannt` und `Nein` zusammenlegte, machte sie rot, und das ist
    /// ihr Zweck: die Festlegung stammt vom Nutzer und steht gegen die Zusage
    /// „Unentschieden gilt als laut", die die Runde 12 fuer den Loeschweg
    /// gegeben hat. Der Datensatz steht im Modulkopf.
    #[test]
    fn ein_unbekanntes_schreibrecht_laesst_durch() {
        let unbekannt = Abwurflage {
            schreibrecht: Schreibrecht::Unbekannt,
            ..durchlassende_lage()
        };
        assert_eq!(
            urteil(&unbekannt),
            Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            "ein unentscheidbares Schreibrecht weist ab"
        );
        assert_eq!(
            urteil(&unbekannt),
            urteil(&durchlassende_lage()),
            "`Unbekannt` verhaelt sich nicht wie `Ja`"
        );

        let gemessen_nein = Abwurflage {
            schreibrecht: Schreibrecht::Nein,
            ..durchlassende_lage()
        };
        assert_eq!(
            urteil(&gemessen_nein),
            Abwurfurteil::Abweisen(Abwurfgrund::NichtBeschreibbar),
            "ein gemessenes `Nein` laesst durch"
        );
    }

    /// Kopieren ist die Vorgabe, und der zerstoererische Vorgang tritt nur ein,
    /// wenn das Kopieren gar nicht im Angebot steht (C5).
    ///
    /// Die Aussage der Probe ist eine Ungleichung und keine Tabellenzeile: wo
    /// beide Vorgaenge angeboten werden, faellt die Wahl auf den, der nichts
    /// wegnimmt. Genau daran haengt die Zusage des Spec, dass die Vorgabe die
    /// sichere Seite ist.
    #[test]
    fn kopieren_geht_dem_verschieben_vor() {
        let beides = durchlassende_lage();
        assert_eq!(
            urteil(&beides),
            Abwurfurteil::Ausfuehren(Abwurfvorgang::Kopieren),
            "bei beidem im Angebot wird verschoben"
        );

        let nur_verschieben = Abwurflage {
            bietet_kopieren: false,
            ..durchlassende_lage()
        };
        assert_eq!(
            urteil(&nur_verschieben),
            Abwurfurteil::Ausfuehren(Abwurfvorgang::Verschieben),
            "ohne Kopieren im Angebot wird nicht verschoben"
        );
    }
}
