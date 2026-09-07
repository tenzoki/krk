//! Die dreiwertigen Antworten auf eine Frage an ein Loeschziel: ja, nein, oder
//! nicht zu entscheiden (C3, C4) — **zwei Typen fuer die zwei Richtungen**, in
//! die eine solche Frage zeigen kann.
//!
//! **Zwei Typen und keine Pruefung.** Das Modul oeffnet keinen Deskriptor,
//! liest kein Verzeichnis und ruft nichts auf; es traegt die Form der Antwort
//! und die eine Verknuepfung, die mehrere Pruefungen dieser Runde teilen.
//! Deshalb steht es neben der Kette des Verzeichnislesers und nicht in ihr.
//!
//! ```text
//!  loeschzielbefund ──> Warnbefund::{Ja, Nein, Unentschieden}
//!                    │    ├── ist_warnwuerdig()  ──> bool   (Ja | Unentschieden)
//!                    │    └── oder(andere)       ──> Warnbefund
//!                    │
//!                    └──> Erlaubnisbefund::{Ja, Nein, Unentschieden}
//!                         └── erlaubt()          ──> bool   (nur Ja)
//! ```
//!
//! # Warum zwei Typen und nicht einer
//!
//! **Weil dieselbe dreiwertige Antwort zwei entgegengesetzte Fragen
//! beantwortet, und der Uebersetzer die Verwechslung nicht sah.** Bei „liegt
//! das Ziel auf einem Netzlaufwerk" ist `Ja` der **Warngrund**, bei „fuehrt das
//! Ziel einen Papierkorb" ist `Ja` die **Erlaubnis**. Bis zum 260907 trugen
//! beide Seiten den einen Typ `Loeschzielbefund`; eine vertauschte Zuweisung
//! uebersetzte, bestand jede Probe und drehte den Sinn um. Genau das ist am
//! 260817-1640 eingetreten — `ist_lokal` lieferte die Umkehrung des Feldes, das
//! es fuellte —, und gefunden hat es der Nutzer und keine Probe
//! (`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1623_*_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`).
//!
//! **`Unentschieden` ist dabei ein Fixpunkt jeder Umkehrung**, und das machte
//! den Fehler doppelt schwer zu sehen: die Zusage „Unentschieden gilt als laut"
//! blieb sichtbar erfuellt, waehrend der genannte **Grund** in den beiden
//! entschiedenen Faellen falsch war.
//!
//! Seit dem 260907 sind es zwei Typen, gewaehlt vom Nutzer als Moeglichkeit 2
//! aus
//! `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/decisions/260818-0249_*_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md`.
//! **Die Verdrehung uebersetzt damit nicht mehr**, und das ist der ganze Ertrag
//! des Schnitts: `Warnbefund` und `Erlaubnisbefund` sind verschiedene Typen,
//! zwischen denen keine Umrechnung steht, und ein `Erlaubnisbefund` an einer
//! Stelle, die einen `Warnbefund` erwartet, haelt den Bau an.
//!
//! Verworfen ist ein `umgekehrt()` am einen Typ: der Nutzer hat es am
//! 260817-1640 ausdruecklich abgelehnt, zusammen mit der Umkehrung von Hand im
//! Aufrufer. Beide lassen die Verdrehung uebersetzbar — wer die Umkehrung
//! weglaesst, bekommt keinen Fehler, sondern das falsche Ergebnis.
//!
//! ```
//! use krk_core::verzeichnis::{Erlaubnisbefund, Warnbefund};
//!
//! fn warnt_das_ziel(_: Warnbefund) {}
//! fn darf_geloescht_werden(_: Erlaubnisbefund) {}
//!
//! warnt_das_ziel(Warnbefund::Ja);
//! darf_geloescht_werden(Erlaubnisbefund::Ja);
//! ```
//!
//! ```compile_fail
//! use krk_core::verzeichnis::{Erlaubnisbefund, Warnbefund};
//!
//! fn warnt_das_ziel(_: Warnbefund) {}
//!
//! // Die Verdrehung: die Antwort auf „fuehrt das Ziel einen Papierkorb"
//! // (`Ja` = Erlaubnis) an einer Stelle, die den Warngrund erwartet.
//! warnt_das_ziel(Erlaubnisbefund::Ja);
//! ```
//!
//! **Was die zwei Typen nicht leisten.** Sie halten die Richtung, nicht den
//! Gegenstand: zwei Fragen **derselben** Richtung — Netzlaufwerk und
//! Arbeitsbaum — tragen beide `Warnbefund`, und wer ihre Antworten
//! untereinander vertauscht, bekommt weiterhin keinen Uebersetzerfehler. Dagegen
//! haelt, was schon vorher dagegen hielt: die Felder heissen nach ihrer Frage,
//! und `loeschwarnung::warngruende` schreibt jede Antwort einzeln aus.
//!
//! # Warum die Typen so heissen
//!
//! **Die Richtung steht vorn, damit sie an der Aufrufstelle zu lesen ist.** Wer
//! `-> Warnbefund` sieht, weiss ohne einen Blick in den Rumpf, dass `Ja` warnt;
//! wer `-> Erlaubnisbefund` sieht, dass `Ja` erlaubt. Das weicht von der Regel
//! ab, nach der in diesem Modulbaum der **Gegenstand** vor dem Wortstamm steht
//! ([`super::inhalt::Inhaltsbefund`], frueher `Loeschzielbefund`), und die
//! Abweichung ist der Zweck des Schnitts: unterschieden werden hier nicht zwei
//! Gegenstaende, sondern zwei Richtungen an demselben.
//!
//! **`Befund` heisst weiter keiner von beiden**, denn dieser Modulbaum fuehrt
//! schon einen: [`super::modell::Befund`] aus der Runde 10,
//! `Unentschieden`/`Treffer`/`KeinTreffer`. Bis zum 260817 hiess der Typ hier
//! ebenso, und zwei dreiwertige Typen desselben Namens unter [`super`] waren
//! der Befund
//! (`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`).
//!
//! **Der Modulname bleibt [`super::loeschzielbefund`]**, obwohl der Typ dieses
//! Namens nicht mehr steht. Er benennt den Gegenstand und nicht einen Typ:
//! beides sind Befunde ueber ein Loeschziel, und jeder von beiden traegt genau
//! eine Richtung. Ein Modul `loeschziel` daneben liesse „Ziel" ein drittes Mal
//! im Baum stehen, neben [`super::verweisziel::Verweisziel`] und dem
//! `Kopierziel` der Operationsmaschine.
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
//! **Sie bleibt in beiden Typen**, und in beiden mit derselben Bedeutung: keine
//! Aussage ueber das Ziel, sondern eine ueber KRKs Kenntnis von ihm.
//! Verschieden ist allein, wohin sie faellt — beim [`Warnbefund`] zu `Ja`, beim
//! [`Erlaubnisbefund`] zu `Nein` —, und genau diesen Unterschied tragen jetzt
//! die Typen statt der Prosa.
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
//! Beide Typen sind die Verallgemeinerung jener Haltung und keine neue Idee.
//! Sie **ersetzen** `ist_deskriptormangel` nicht: das dort ist ein Praedikat
//! ueber einen `io::Error` und beantwortet die Frage, ob dieser Fehler etwas
//! ueber das Ziel sagt oder ueber den Vorrat der laufenden Sitzung. Hier steht,
//! was sein Aufrufer mit der Antwort tut.
//!
//! # Wer sie beantwortet
//!
//! Vier Pruefungen liefern sie, und jede steht dort, wo ihre Frage zu
//! beantworten ist: die Frage nach dem Papierkorb und die nach dem
//! Netzlaufwerk in `krk-ui/src/appkit`, weil beide AppKit brauchen, die Frage
//! nach dem Git-Arbeitsbaum und die gedeckelte Zaehlung des Umfangs hier im
//! Kern.
//!
//! ```text
//!  Papierkorb       krk-ui/src/appkit/papierkorb.rs  fuehrt_einen_papierkorb
//!                     ──> Erlaubnisbefund
//!  Netzlaufwerk     krk-ui/src/appkit/volumes.rs     liegt_auf_netzlaufwerk
//!                     ──> Warnbefund
//!  Git-Arbeitsbaum  super::arbeitsbaum               beruehrt_einen_arbeitsbaum
//!                     ──> Warnbefund
//!  Umfang           super::umfang                    zaehlen
//!                     ──> Umfang (eigener Typ, gleiche Haltung)
//! ```
//!
//! Drei liefern einen der beiden Typen selbst. Die vierte antwortet mit
//! [`super::umfang::Umfang`], dessen Ausgang `Unentschieden` auf den
//! [`Warnbefund`] verweist und dieselbe Haltung traegt: eine Zaehlung, die nicht
//! zustande kam, ist eine Aussage ueber KRKs Kenntnis und nicht ueber die
//! Auswahl. Sie bleibt ein eigener Typ, weil ihre entschiedenen Antworten
//! **Zahlen** tragen und kein Ja.
//!
//! **Ob `dead_code` sie trifft, haengt nicht daran, wer sie ruft.** `krk-core`
//! ist eine Bibliothek, und beide sind von ihrer Wurzel aus erreichbar; eine
//! Ausnahme nach dem Vorbild von `krk-ui/src/kommandos/rueckschritt.rs`
//! braeuchten sie auch dann nicht, wenn in dieser Kiste kein Aufrufer stuende.
//!
//! Die bindende Grundlage ist
//! `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.

/// Was eine Pruefung ueber ein Loeschziel sagt, wenn **`Ja` der Warngrund ist**.
///
/// Die erste der beiden Richtungen: „liegt das Ziel auf einem Netzlaufwerk",
/// „beruehrt der Vorgang einen Git-Arbeitsbaum". [`Warnbefund::Unentschieden`]
/// gehoert hier zu [`Warnbefund::Ja`], und [`Warnbefund::ist_warnwuerdig`] fasst
/// genau die beiden zusammen.
///
/// Drei Werte und kein Wahrheitswert mit Beipackzettel: `Unentschieden` ist
/// kein Sonderfall von `Nein` und keiner von `Ja`, sondern ein eigener Ausgang
/// mit eigenem Wortlaut in der Rueckfrage. Warum, steht im Modulkopf.
///
/// Die Fallunterscheidungen darunter tragen keinen Auffangzweig. Eine vierte
/// Antwort haelt damit den Bau an und erzwingt eine bewusste Einordnung, statt
/// still in einen bestehenden Zweig zu fallen.
///
/// **Kein abgeleitetes [`Ord`]**: eine Ordnung waere hier eine Behauptung ohne
/// Gegenstand, und [`Warnbefund::oder`] schreibt seine Tafel deshalb aus, statt
/// ein `max` zu sein.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Warnbefund {
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist ja: der
    /// Ausloeser trifft zu. **Das ist der Warngrund.**
    Ja,
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist nein. Das
    /// ist die **einzige** Antwort, die eine Rueckfrage ruhig laesst.
    Nein,
    /// Die Pruefung ist nicht dazu gekommen, die Frage zu beantworten: ein Pfad
    /// liess sich nicht aufloesen, ein Datentraeger nicht einordnen, ein
    /// Ressourcenwert nicht lesen. Das ist keine Aussage ueber das Ziel,
    /// sondern eine ueber KRKs Kenntnis von ihm, und sie gilt als laut.
    Unentschieden,
}

/// Was eine Pruefung ueber ein Loeschziel sagt, wenn **`Ja` die Erlaubnis ist**.
///
/// Die zweite der beiden Richtungen, und in dieser Runde traegt sie genau eine
/// Frage: „fuehrt das Ziel einen Papierkorb" (C4).
/// [`Erlaubnisbefund::Unentschieden`] gehoert hier zu
/// [`Erlaubnisbefund::Nein`] — ein Ziel, dessen Papierkorb sich nicht
/// feststellen laesst, wird nicht geloescht —, und [`Erlaubnisbefund::erlaubt`]
/// ist die eine Frage an den Wert.
///
/// **Eine Verknuepfung wie [`Warnbefund::oder`] traegt dieser Typ nicht**, weil
/// keine Stelle im Baum zwei Erlaubnisse zusammenfasst. Sie waere Vorrat, und
/// ihre Tafel muesste ausserdem anders herum abgeleitet werden: dort saugt `Ja`
/// auf, hier muesste es `Nein`.
///
/// Wie beim [`Warnbefund`] kein abgeleitetes [`Ord`] und kein Auffangzweig bei
/// den Aufrufern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Erlaubnisbefund {
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist ja. **Das
    /// ist die Erlaubnis**, und sie ist die einzige.
    Ja,
    /// Die Pruefung hat die Frage beantwortet, und die Antwort ist nein: es
    /// wird nicht geloescht.
    Nein,
    /// Die Pruefung ist nicht dazu gekommen, die Frage zu beantworten. Das ist
    /// keine Aussage ueber das Ziel, sondern eine ueber KRKs Kenntnis von ihm,
    /// und es wird ebenfalls nicht geloescht.
    Unentschieden,
}

impl Warnbefund {
    /// Ob dieser Befund die Rueckfrage laut macht: alles ausser
    /// [`Warnbefund::Nein`].
    ///
    /// Die eine Zeile der Zusage „Unentschieden gilt als laut", als Frage an den
    /// Wert. Sie fasst [`Warnbefund::Ja`] und [`Warnbefund::Unentschieden`]
    /// zusammen, und die Zusammenfassung ist erlaubt, weil die laute Form fuer
    /// beide dieselbe ist; **verschieden ist allein der Grund**, den die Frage
    /// nennt, und den holt sich der Aufrufer nicht hier, sondern aus dem Befund
    /// selbst.
    ///
    /// **An einem [`Erlaubnisbefund`] ist sie seit dem 260907 nicht mehr zu
    /// stellen**, und darin liegt der Ertrag des Schnitts: bis dahin trugen
    /// beide Richtungen einen Typ, und wer die Frage aus Gewohnheit an die
    /// falsche Richtung hielt, machte aus „wir wissen nichts" die Erlaubnis zu
    /// loeschen. Heute uebersetzt das nicht.
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
    /// | `self` \ `andere` | [`Warnbefund::Ja`] | [`Warnbefund::Nein`] | [`Warnbefund::Unentschieden`] |
    /// |---|---|---|---|
    /// | **[`Warnbefund::Ja`]** | `Ja` | `Ja` | `Ja` |
    /// | **[`Warnbefund::Nein`]** | `Ja` | `Nein` | `Unentschieden` |
    /// | **[`Warnbefund::Unentschieden`]** | `Ja` | `Unentschieden` | `Unentschieden` |
    ///
    /// # Woraus die Tafel abgeleitet ist
    ///
    /// Nicht aus einer Lehrbuchtabelle, sondern aus zwei Saetzen des Specs. Die
    /// Ableitung steht hier, damit die Tafel nicht bloss behauptet dasteht:
    ///
    /// 1. **Ein `Ja` ist eine gewusste Tatsache, und keine zweite Antwort nimmt
    ///    sie zurueck.** Trifft ein Ausloeser zu, trifft er zu, gleich was die
    ///    andere Pruefung ergab. Damit ist [`Warnbefund::Ja`] aufsaugend: die
    ///    ganze erste Zeile und die ganze erste Spalte sind `Ja`. Das sind fuenf
    ///    der neun Felder.
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
    /// **Die Ableitung haengt an der Richtung dieses Typs**, und genau deshalb
    /// steht die Verknuepfung hier und nicht an einem gemeinsamen Rumpf beider
    /// Typen: welcher Wert aufsaugt, entscheidet die Frage und nicht die Form
    /// der Aufzaehlung.
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
    /// Die Verknuepfung ist ausserdem symmetrisch und hat [`Warnbefund::Nein`]
    /// als neutrales Element. Beides ist an der Tafel abzulesen und wird von
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

impl Erlaubnisbefund {
    /// Ob dieser Befund das Loeschen erlaubt: allein [`Erlaubnisbefund::Ja`].
    ///
    /// Die eine Frage an einen Wert dieser Richtung, und sie faellt bewusst
    /// **anders** aus als [`Warnbefund::ist_warnwuerdig`]: dort gehoert
    /// `Unentschieden` zum `Ja`, hier zum `Nein`. Wer nichts ueber den
    /// Papierkorb eines Ziels weiss, loescht dort nicht, und genau das ist die
    /// Zusage von C4.
    ///
    /// **Die Fallunterscheidung steht hier und nicht bei ihrem Aufrufer.** Bis
    /// zum 260907 schrieb `loeschwarnung::vor_der_rueckfrage` die drei Antworten
    /// selbst aus, weil der Typ die Richtung nicht kannte; die Vollstaendigkeit
    /// ist damit nicht verlorengegangen, sondern hierhergezogen — ein vierter
    /// Wert haelt diesen `match` an.
    ///
    /// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: der Wert
    /// ist die Erlaubnis selbst, und ohne ihn loeschte der Aufrufer auf einem
    /// Ziel, das keinen Rueckweg fuehrt.
    #[must_use = "der Wert ist die Erlaubnis zu loeschen; fallengelassen loescht der Aufrufer ohne Rueckweg"]
    pub fn erlaubt(self) -> bool {
        match self {
            Self::Ja => true,
            Self::Nein | Self::Unentschieden => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Die Tafeln darunter stehen in der Form der Tafel aus dem Doc-Kommentar von
    // [`Warnbefund::oder`], und die kurzen Namen halten jede Zeile lesbar auf
    // einer Zeile. Es ist eine Einfuhr der drei Werte und keine pauschale.
    use Warnbefund::{Ja, Nein, Unentschieden};

    /// Alle drei Werte und alle neun Paare, einmal als Daten.
    ///
    /// Sie stehen hier oben, weil vier der Proben sie durchfahren, und nicht
    /// damit eine Erwartung daraus gerechnet wuerde: die Erwartungen stehen in
    /// ihren Proben Feld fuer Feld da.
    const ALLE: [Warnbefund; 3] = [Ja, Nein, Unentschieden];

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

    /// Die Erlaubnis traegt genau einen der drei Werte, und es ist das `Ja`.
    ///
    /// **Die Gegenrichtung zur Probe darueber, und die beiden zusammen sind der
    /// Grund fuer die zwei Typen.** Dieselben drei Namen, dieselbe Reihenfolge,
    /// und `Unentschieden` faellt hier auf die andere Seite. Solange die zwei
    /// Richtungen einen Typ teilten, war dieser Unterschied allein in Prosa
    /// festgehalten; seit dem 260907 haelt ihn der Uebersetzer, und diese Probe
    /// schreibt aus, was er haelt.
    #[test]
    fn die_erlaubnis_traegt_nur_das_ja() {
        assert!(
            Erlaubnisbefund::Ja.erlaubt(),
            "ein festgestellter Papierkorb erlaubt das Loeschen nicht"
        );
        assert!(
            !Erlaubnisbefund::Nein.erlaubt(),
            "ein Ziel ohne Papierkorb erlaubt das Loeschen"
        );
        assert!(
            !Erlaubnisbefund::Unentschieden.erlaubt(),
            "ein unentschiedener Befund erlaubt das Loeschen, obwohl C4 ihn \
             wie das Nein behandelt"
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
        const TAFEL: [(Warnbefund, Warnbefund, Warnbefund); 9] = [
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
    /// [`Warnbefund::oder`], als Zaehlung: haette ein zweites Feld `Nein`, waere
    /// die ruhige Rueckfrage ueber einem Ziel moeglich, ueber das eine der
    /// beiden Pruefungen nichts sagen konnte.
    #[test]
    fn nur_zwei_mal_nein_bleibt_ruhig() {
        let ruhige: Vec<(Warnbefund, Warnbefund)> = ALLE
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
