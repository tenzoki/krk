//! Die dreiwertige Antwort auf eine Frage an ein Loeschziel: ja, nein, oder
//! nicht zu entscheiden (C3, C4).
//!
//! **Ein Typ und keine Pruefung.** Das Modul oeffnet keinen Deskriptor, liest
//! kein Verzeichnis und ruft nichts auf; es traegt die Form der Antwort und die
//! eine Verknuepfung, die mehrere Pruefungen dieser Runde teilen. Deshalb steht
//! es neben der Kette des Verzeichnislesers und nicht in ihr.
//!
//! ```text
//!  loeschzielbefund ──> Loeschzielbefund::{Ja, Nein, Unentschieden}
//!                         ├── ist_warnwuerdig()  ──> bool
//!                         └── oder(andere)       ──> Loeschzielbefund
//! ```
//!
//! # Warum der Typ nicht `Befund` heisst
//!
//! **Weil dieser Modulbaum schon einen `Befund` fuehrt.** Bis zum 260817 hiess
//! der Typ hier so, und damit standen unter [`super`] zwei verschiedene
//! dreiwertige Typen desselben Namens: dieser und [`super::modell::Befund`] aus
//! der Runde 10, `Unentschieden`/`Treffer`/`KeinTreffer`. Der Uebersetzer trennt
//! sie, eine Verwechslung uebersetzt nicht — fuer den Leser unterschieden sie
//! sich um einen Pfadabschnitt, und beide trugen eine Variante `Unentschieden`
//! mit derselben Bedeutung
//! (`issues/260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`).
//!
//! **Umbenannt wurde dieser und nicht der aeltere**, aus drei Gruenden, und der
//! erste ist der tragende:
//!
//! 1. Der aeltere steht in der Mitte einer Benennung, die um ihn herum gewachsen
//!    ist: [`super::durchlauf::Befundmeldung`] fuellt ihn,
//!    [`super::inhalt::Inhaltsbefund`] beantwortet seine Frage fuer eine Datei,
//!    und `Ordnermodell` traegt `befund`, `befunde_setzen` und
//!    `befund_zuruecksetzen`. Ihn allein umzubenennen liesse die halbe Familie
//!    auf einen Namen zeigen, den es nicht mehr gibt; sie mitzunehmen waere ein
//!    Umbau des Filters der Runden 10 und 11.
//! 2. Der nackte Name gehoert dem weiteren Begriff. Auf den Filterbefund
//!    arbeitet die ganze Kette dieses Verzeichnisses hin; dieser Typ hier
//!    beantwortet die Pruefungen **einer** Runde an **einem** Gegenstand.
//! 3. Er ist der juengere von beiden und hatte drei Aufrufer, alle drei in
//!    `krk-ui`. Am 260817 nachgezaehlt kostete die Umbenennung hier 25
//!    Stellen im Code; beim aelteren waeren es 48 gewesen, mehr als zwei
//!    Drittel davon in `krk-core/tests/verzeichnis.rs`.
//!
//! **Der Wortstamm bleibt, und der Gegenstand kommt davor.** Genau das tut
//! [`super::inhalt::Inhaltsbefund`] schon: `Befund` ist in diesem Baum das
//! Rollenwort fuer „was ein Lesen oder eine Pruefung ueber einen genannten
//! Gegenstand herausgefunden hat", und der Gegenstand steht vorn. Ein Name ohne
//! den Stamm — `Zielantwort`, `Pruefbefund`, `Loeschpruefung` — braeche diese
//! Regel, statt sie herzustellen, und die beiden letzten benennen ausserdem die
//! Maschine und nicht die Frage.
//!
//! **`Zielbefund` waere der kuerzere Name und der falsche.** „Ziel" ist in
//! diesem Baum zweimal vergeben: in diesem Modulbaum als
//! [`super::verweisziel::Verweisziel`] fuer das Ziel einer Verknuepfung, und in
//! der Operationsmaschine als `Kopierziel` fuer das Ziel eines Kopiervorgangs.
//! Der kurze Name tauschte also ein Wort mit zwei Lesarten gegen ein anderes.
//! Das ganze Wort `Loeschziel` ist dagegen das Wort des Specs dieser Runde und
//! hat hier genau eine Lesart.
//!
//! **Im Namen steht kein `Warn`**, und das ist kein Zufall: der Typ traegt beide
//! Polaritaeten. Bei der Frage nach dem Netzlaufwerk und der nach dem
//! Arbeitsbaum ist `Ja` der Warngrund, bei der Frage nach dem Papierkorb ist es
//! die Erlaubnis. Welche vorliegt, haengt an der Frage und nicht am Typ; der
//! Abschnitt weiter unten haelt die beiden auseinander.
//!
//! # Warum es die dritte Antwort gibt
//!
//! Der Spec dieser Runde stellt die Zusage **„Unentschieden gilt als laut"**
//! auf: laesst sich eine der Pruefungen an einem Ziel nicht beantworten, etwa
//! weil ein Pfad sich nicht aufloesen oder ein Datentraeger sich nicht
//! einordnen laesst, gilt das Ziel als warnwuerdig. Eine Pruefung, die im
//! Zweifel schweigt, waere in genau den Lagen still, in denen KRK am wenigsten
//! ueber das Ziel weiss.
//!
//! Ein Wahrheitswert kann diese Zusage nicht tragen, und zwar nicht aus
//! Bequemlichkeit, sondern weil C3 den Grund der lauten Form nennen laesst:
//! „laesst sich einer der sechs Ausloeser an diesem Ziel nicht entscheiden, ist
//! die Rueckfrage laut und nennt als Grund, dass das Ziel sich nicht einordnen
//! liess". Wer den fehlenden Befund schon an seiner Quelle in ein `Ja`
//! umdeutete, haette die Lautheit, aber der Grund waere falsch: das Blatt
//! behauptete dann ein Netzlaufwerk oder einen Arbeitsbaum, wo KRK bloss nichts
//! wusste. Die dritte Antwort ist also nicht die Bequemlichkeit des Anrufers,
//! sondern die Voraussetzung dafuer, dass der Wortlaut der Frage stimmt.
//!
//! # Wo der Baum dieselbe Unterscheidung schon einmal gebraucht hat
//!
//! In [`super::sys::ist_deskriptormangel`]. Die Durchsicht der Runde 10 fand am
//! [`super::durchlauf`] einen Fehler dieser Bauart: er erzeugte den
//! Deskriptormangel selbst und legte ihn dann als „kein Treffer darunter" aus,
//! also als negativen Befund. Seitdem trennt `ist_deskriptormangel` `EMFILE`
//! und `ENFILE` von den Fehlern, die etwas ueber den Pfad sagen, und **ein
//! Mangel von aussen laesst einen Auftrag unentschieden statt ihn negativ zu
//! entscheiden**.
//!
//! Dieser Typ ist die Verallgemeinerung jener Haltung und keine neue Idee. Er
//! **ersetzt** `ist_deskriptormangel` nicht: das dort ist ein Praedikat ueber
//! einen `io::Error` und beantwortet die Frage, ob dieser Fehler etwas ueber
//! das Ziel sagt oder ueber den Vorrat der laufenden Sitzung. Hier steht, was
//! sein Aufrufer mit der Antwort tut.
//!
//! # Die zwei Polaritaeten, und warum [`Loeschzielbefund::ist_warnwuerdig`] nur die eine trifft
//!
//! Die Fragen dieser Runde zerfallen in zwei Sorten, und der Unterschied
//! entscheidet, ob [`Loeschzielbefund::ist_warnwuerdig`] die richtige Frage an den Wert
//! ist:
//!
//! - **Ein `Ja` ist ein Warngrund** — liegt der Ordner auf einem Netzlaufwerk,
//!   steckt er in einem Git-Arbeitsbaum. Hier gehoert `Unentschieden` zu `Ja`,
//!   und genau das fasst [`Loeschzielbefund::ist_warnwuerdig`] zusammen.
//! - **Ein `Ja` ist die Erlaubnis** — fuehrt das Ziel einen Papierkorb (C4).
//!   Hier gehoert `Unentschieden` zu `Nein`, denn ein Ziel, dessen Papierkorb
//!   sich nicht feststellen laesst, wird nicht geloescht.
//!
//! **Fuer die zweite Sorte ist [`Loeschzielbefund::ist_warnwuerdig`] die falsche Frage**,
//! und der Aufrufer prueft dort auf [`Loeschzielbefund::Ja`] selbst. Wer aus Gewohnheit
//! nach der Warnwuerdigkeit fragt, macht aus „wir wissen nichts" die Erlaubnis
//! zu loeschen — genau der Fall, gegen den C4 gebaut ist. Beide Sorten folgen
//! derselben Haltung, im Zweifel die vorsichtigere Antwort zu nehmen; welche
//! das ist, haengt an der Frage und nicht am Typ.
//!
//! # Wer ihn beantwortet
//!
//! Vier Pruefungen liefern ihn, und jede steht dort, wo ihre Frage zu
//! beantworten ist: die Frage nach dem Papierkorb und die nach dem
//! Netzlaufwerk in `krk-ui/src/appkit`, weil beide AppKit brauchen, die Frage
//! nach dem Git-Arbeitsbaum und die gedeckelte Zaehlung des Umfangs hier im
//! Kern.
//!
//! ```text
//!  Papierkorb       krk-ui/src/appkit/papierkorb.rs  fuehrt_einen_papierkorb
//!  Netzlaufwerk     krk-ui/src/appkit/volumes.rs     liegt_auf_netzlaufwerk
//!  Git-Arbeitsbaum  super::arbeitsbaum               beruehrt_einen_arbeitsbaum
//!  Umfang           super::umfang                    zaehlen
//! ```
//!
//! Die ersten drei liefern diesen Typ selbst. Die vierte antwortet mit
//! [`super::umfang::Umfang`], dessen Ausgang `Unentschieden` auf ihn verweist
//! und dieselbe Haltung traegt: eine Zaehlung, die nicht zustande kam, ist eine
//! Aussage ueber KRKs Kenntnis und nicht ueber die Auswahl.
//!
//! **Ob `dead_code` ihn trifft, haengt nicht daran, wer ihn ruft.** `krk-core`
//! ist eine Bibliothek, und er ist von ihrer Wurzel aus erreichbar; eine
//! Ausnahme nach dem Vorbild von `krk-ui/src/kommandos/rueckschritt.rs`
//! braeuchte er auch dann nicht, wenn in dieser Kiste kein Aufrufer stuende.
//!
//! Die bindende Grundlage ist
//! `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.

/// Was eine Pruefung ueber ein Loeschziel sagt.
///
/// Drei Werte und kein Wahrheitswert mit Beipackzettel: `Unentschieden` ist
/// kein Sonderfall von `Nein` und keiner von `Ja`, sondern ein eigener Ausgang
/// mit eigenem Wortlaut in der Rueckfrage. Warum, steht im Modulkopf.
///
/// Die Fallunterscheidungen darunter tragen keinen Auffangzweig. Eine vierte
/// Antwort haelt damit den Bau an und erzwingt eine bewusste Einordnung, statt
/// still in einen bestehenden Zweig zu fallen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loeschzielbefund {
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist ja.
    Ja,
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist nein. Das
    /// ist die **einzige** Antwort, die eine Rueckfrage ruhig laesst.
    Nein,
    /// Die Pruefung ist nicht dazu gekommen, die Frage zu beantworten: ein Pfad
    /// liess sich nicht aufloesen, ein Datentraeger nicht einordnen, ein
    /// Ressourcenwert nicht lesen. Das ist keine Aussage ueber das Ziel,
    /// sondern eine ueber KRKs Kenntnis von ihm.
    Unentschieden,
}

impl Loeschzielbefund {
    /// Ob dieser Befund die Rueckfrage laut macht: alles ausser [`Loeschzielbefund::Nein`].
    ///
    /// Die eine Zeile der Zusage „Unentschieden gilt als laut", als Frage an den
    /// Wert. Sie fasst [`Loeschzielbefund::Ja`] und [`Loeschzielbefund::Unentschieden`] zusammen,
    /// und die Zusammenfassung ist erlaubt, weil die laute Form fuer beide
    /// dieselbe ist; **verschieden ist allein der Grund**, den die Frage nennt,
    /// und den holt sich der Aufrufer nicht hier, sondern aus dem Befund selbst.
    ///
    /// **Nicht fuer die Frage nach dem Papierkorb.** Dort ist `Ja` die Erlaubnis
    /// und nicht der Warngrund; die beiden Polaritaeten stehen im Modulkopf
    /// auseinandergehalten.
    ///
    /// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: verloren
    /// ginge die Lautheit, und die Rueckfrage erschiene ruhig ueber einem Ziel,
    /// das KRK nicht einordnen konnte.
    #[must_use = "der Wert entscheidet, ob die Rueckfrage laut wird; fallengelassen bleibt sie ruhig"]
    pub fn ist_warnwuerdig(self) -> bool {
        match self {
            Self::Ja | Self::Unentschieden => true,
            Self::Nein => false,
        }
    }

    /// Zwei Befunde zu einem: die dreiwertige Oder-Verknuepfung.
    ///
    /// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht
    /// gerechnet. Neun Kombinationen, neun Felder:
    ///
    /// | `self` \ `andere` | [`Loeschzielbefund::Ja`] | [`Loeschzielbefund::Nein`] | [`Loeschzielbefund::Unentschieden`] |
    /// |---|---|---|---|
    /// | **[`Loeschzielbefund::Ja`]** | `Ja` | `Ja` | `Ja` |
    /// | **[`Loeschzielbefund::Nein`]** | `Ja` | `Nein` | `Unentschieden` |
    /// | **[`Loeschzielbefund::Unentschieden`]** | `Ja` | `Unentschieden` | `Unentschieden` |
    ///
    /// # Woraus die Tafel abgeleitet ist
    ///
    /// Nicht aus einer Lehrbuchtabelle, sondern aus zwei Saetzen des Specs. Die
    /// Ableitung steht hier, damit die Tafel nicht bloss behauptet dasteht:
    ///
    /// 1. **Ein `Ja` ist eine gewusste Tatsache, und keine zweite Antwort nimmt
    ///    sie zurueck.** Trifft ein Ausloeser zu, trifft er zu, gleich was die
    ///    andere Pruefung ergab. Damit ist [`Loeschzielbefund::Ja`] aufsaugend: die ganze
    ///    erste Zeile und die ganze erste Spalte sind `Ja`. Das sind fuenf der
    ///    neun Felder.
    /// 2. **Ruhig wird es nur mit Wissen.** Die ruhige Form der Rueckfrage sagt
    ///    dem Nutzer, dass an diesem Ziel nichts Ungewoehnliches ist. Sie darf
    ///    deshalb nur dastehen, wenn **beide** Seiten das entschieden haben:
    ///    `Nein.oder(Nein)` ist das einzige ruhige Feld der Tafel.
    /// 3. **Die drei uebrigen Felder behalten den Zweifel.** Sie werden nicht zu
    ///    `Ja`, denn C3 laesst den Grund nennen, und „liess sich nicht
    ///    einordnen" ist ein anderer Grund als der Wortlaut eines Ausloesers;
    ///    ein `Ja` hier machte den Grund falsch. Sie werden auch nicht zu
    ///    `Nein`, denn das schwiege ueber genau den Fall, fuer den die Zusage
    ///    da ist.
    ///
    /// # Was daraus folgt
    ///
    /// Die Verknuepfung vertraegt sich mit der Zusage, statt sie zu unterlaufen:
    ///
    /// ```text
    /// a.oder(b).ist_warnwuerdig() == a.ist_warnwuerdig() || b.ist_warnwuerdig()
    /// ```
    ///
    /// Das gilt in allen neun Feldern, und die Probe
    /// `die_lautheit_ueberlebt_die_verknuepfung` schreibt sie einzeln aus.
    /// **„Unentschieden gilt als laut" und die gewoehnliche dreiwertige Logik
    /// fallen hier also nicht auseinander** — die Tafel ist dieselbe, die eine
    /// Kleene-Logik liefert, und sie ist aus den beiden Saetzen des Specs
    /// abgeleitet und nicht von dort uebernommen. Sie unterlaufen einander erst,
    /// wenn jemand `Unentschieden` **vor** der Verknuepfung in ein `Ja`
    /// umdeutet; dann stimmt die Lautheit weiter und der Grund nicht mehr.
    ///
    /// Die Verknuepfung ist ausserdem symmetrisch und hat [`Loeschzielbefund::Nein`] als
    /// neutrales Element. Beides ist an der Tafel abzulesen und wird von
    /// Proben festgehalten, damit die Reihenfolge, in der ein Aufrufer seine
    /// Tatsachen sammelt, keine Rolle spielt.
    ///
    /// **Als `max` ueber eine Ordnung waere sie kuerzer, und sie steht trotzdem
    /// so da.** Ein abgeleitetes `Ord` legte die Bedeutung in die Reihenfolge
    /// der Aufzaehlung, wo niemand sie liest, und ein spaeteres Umsortieren der
    /// Varianten aenderte die Verknuepfung still mit. Die neun Zweige nennen
    /// jeden Ausgang selbst, und ein vierter Wert haelt den Bau an, statt in
    /// einen Auffangzweig zu fallen.
    ///
    /// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: `oder`
    /// aendert keinen der beiden Befunde, sondern liefert den dritten, und wer
    /// den Rueckgabewert nicht nimmt, hat die zweite Tatsache nie beruecksichtigt.
    #[must_use = "die Verknuepfung aendert nichts, sie liefert den zusammengefassten Befund"]
    pub fn oder(self, andere: Self) -> Self {
        match (self, andere) {
            // Erste Zeile und erste Spalte: ein gewusstes Ja bleibt Ja.
            (Self::Ja, Self::Ja) => Self::Ja,
            (Self::Ja, Self::Nein) => Self::Ja,
            (Self::Ja, Self::Unentschieden) => Self::Ja,
            (Self::Nein, Self::Ja) => Self::Ja,
            (Self::Unentschieden, Self::Ja) => Self::Ja,
            // Das einzige ruhige Feld: beide Seiten haben entschieden, und
            // beide sagen nein.
            (Self::Nein, Self::Nein) => Self::Nein,
            // Die drei Felder mit Zweifel und ohne Ja: der Zweifel bleibt
            // stehen, damit der Grund „liess sich nicht einordnen" heissen kann.
            (Self::Nein, Self::Unentschieden) => Self::Unentschieden,
            (Self::Unentschieden, Self::Nein) => Self::Unentschieden,
            (Self::Unentschieden, Self::Unentschieden) => Self::Unentschieden,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Die Tafeln darunter stehen in der Form der Tafel aus dem Doc-Kommentar von
    // [`Loeschzielbefund::oder`], und die kurzen Namen halten jede Zeile lesbar auf einer
    // Zeile. Es ist eine Einfuhr der drei Werte und keine pauschale.
    use Loeschzielbefund::{Ja, Nein, Unentschieden};

    /// Alle drei Werte und alle neun Paare, einmal als Daten.
    ///
    /// Sie stehen hier oben, weil vier der fuenf Proben sie durchfahren, und
    /// nicht damit eine Erwartung daraus gerechnet wuerde: die Erwartungen
    /// stehen in ihren Proben Feld fuer Feld da.
    const ALLE: [Loeschzielbefund; 3] = [Ja, Nein, Unentschieden];

    /// Die Zusage „Unentschieden gilt als laut", an den drei Werten
    /// ausgeschrieben.
    ///
    /// Drei Zeilen fuer drei Werte, und keine Rechnung: `Unentschieden` steht
    /// hier mit derselben Erwartung wie `Ja` da, und das ist die ganze Aussage
    /// der Zusage.
    #[test]
    fn die_lautheit_traegt_zwei_der_drei_werte() {
        assert!(
            Ja.ist_warnwuerdig(),
            "ein zutreffender Ausloeser macht die Rueckfrage nicht laut"
        );
        assert!(
            Unentschieden.ist_warnwuerdig(),
            "ein unentschiedener Befund macht die Rueckfrage nicht laut, \
             obwohl der Spec ihn als laut zusagt"
        );
        assert!(
            !Nein.ist_warnwuerdig(),
            "ein entschiedenes Nein macht die Rueckfrage laut"
        );
    }

    /// Die ganze Tafel auf einen Blick: drei Werte mal drei Werte, also neun
    /// Felder.
    ///
    /// Sie steht in der Form der Tafel aus `krk-ui/src/kommandos/rueckschritt.rs`
    /// und schreibt jedes Feld einzeln aus. Eine gerechnete Erwartung — etwa
    /// „das Maximum in der Ordnung `Nein < Unentschieden < Ja`" — waere die
    /// Umsetzung ein zweites Mal und faende keinen Fehler, den die Umsetzung
    /// nicht schon hat.
    #[test]
    fn die_tafel_aus_neun_kombinationen_geht_auf() {
        // self, andere, Ausgang.
        const TAFEL: [(Loeschzielbefund, Loeschzielbefund, Loeschzielbefund); 9] = [
            (Ja, Ja, Ja),
            (Ja, Nein, Ja),
            (Ja, Unentschieden, Ja),
            (Nein, Ja, Ja),
            (Nein, Nein, Nein),
            (Nein, Unentschieden, Unentschieden),
            (Unentschieden, Ja, Ja),
            (Unentschieden, Nein, Unentschieden),
            (Unentschieden, Unentschieden, Unentschieden),
        ];

        for (einer, anderer, ausgang) in TAFEL {
            assert_eq!(
                einer.oder(anderer),
                ausgang,
                "die Tafel stimmt nicht: {einer:?}.oder({anderer:?})"
            );
        }
    }

    /// Genau ein Feld der Tafel ist ruhig, und es ist das mit zwei Mal Wissen.
    ///
    /// Das ist die zweite Ableitungsstufe aus dem Doc-Kommentar von
    /// [`Loeschzielbefund::oder`], als Zaehlung: haette ein zweites Feld `Nein`, waere die
    /// ruhige Rueckfrage ueber einem Ziel moeglich, ueber das eine der beiden
    /// Pruefungen nichts sagen konnte.
    #[test]
    fn nur_zwei_mal_nein_bleibt_ruhig() {
        let ruhige: Vec<(Loeschzielbefund, Loeschzielbefund)> = ALLE
            .iter()
            .flat_map(|einer| ALLE.iter().map(move |anderer| (*einer, *anderer)))
            .filter(|(einer, anderer)| einer.oder(*anderer) == Nein)
            .collect();
        assert_eq!(
            ruhige,
            vec![(Nein, Nein)],
            "nicht genau eine der neun Kombinationen bleibt ruhig"
        );
    }

    /// Die Lautheit ueberlebt die Verknuepfung, in allen neun Feldern.
    ///
    /// Diese Probe misst eine Rechenregel und nicht ein Feld, und deshalb
    /// stehen hier zwei gerechnete Seiten und keine ausgeschriebene Erwartung:
    ///
    /// ```text
    /// a.oder(b).ist_warnwuerdig() == a.ist_warnwuerdig() || b.ist_warnwuerdig()
    /// ```
    ///
    /// Sie ist der Beleg dafuer, dass die Tafel die Zusage „Unentschieden gilt
    /// als laut" nicht unterlaeuft: ein Aufrufer darf beliebig viele Befunde
    /// erst zusammenfassen und dann einmal nach der Lautheit fragen, statt jeden
    /// einzeln zu fragen. Rot wird sie, wenn jemand ein Feld der Tafel
    /// verschiebt, und dann ist die Frage, welche der beiden Seiten falsch ist.
    #[test]
    fn die_lautheit_ueberlebt_die_verknuepfung() {
        for einer in ALLE {
            for anderer in ALLE {
                assert_eq!(
                    einer.oder(anderer).ist_warnwuerdig(),
                    einer.ist_warnwuerdig() || anderer.ist_warnwuerdig(),
                    "die Lautheit ueberlebt {einer:?}.oder({anderer:?}) nicht"
                );
            }
        }
    }

    /// Die Reihenfolge der Argumente spielt keine Rolle, und `Nein` ist neutral.
    ///
    /// Beides braucht der Aufrufer, der Tatsachen aus verschiedenen Quellen
    /// sammelt: er darf sie in der Reihenfolge zusammenfassen, in der sie
    /// anfallen, und er darf mit `Nein` anfangen, um ueber eine Liste zu falten.
    #[test]
    fn die_verknuepfung_ist_symmetrisch_und_nein_ist_neutral() {
        for einer in ALLE {
            for anderer in ALLE {
                assert_eq!(
                    einer.oder(anderer),
                    anderer.oder(einer),
                    "die Verknuepfung ist nicht symmetrisch: {einer:?}, {anderer:?}"
                );
            }
            assert_eq!(
                Nein.oder(einer),
                einer,
                "Nein ist nicht das neutrale Element: {einer:?}"
            );
        }
    }
}
