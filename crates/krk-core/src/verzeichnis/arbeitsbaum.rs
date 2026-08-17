//! Beruehrt dieser Loeschvorgang einen Git-Arbeitsbaum? Gefragt am Ordner
//! selbst, an jeder Ebene darueber und an jedem ausgewaehlten Eintrag (C3).
//!
//! Der fuenfte Ausloeser der lauten Rueckfrage. Gefragt wird nach der
//! **Anwesenheit** eines Eintrags [`VERWALTUNGSEINTRAG`], und die Antwort ist
//! ein [`Loeschzielbefund`], also dreiwertig.
//!
//! ```text
//!  traegt_arbeitsbaum(&Path)                 ──> lstat: <Pfad>/.git
//!    ^                                            │ da:            Ja
//!    │                                            │ nicht da:      Nein
//!    │                                            └ nicht lesbar:  Unentschieden
//!    │
//!  liegt_in_arbeitsbaum(&Path, Option<&Path>)
//!    ^      der Ordner selbst, dann aufwaerts ueber `verzeichnis::aufwaerts`,
//!    │      Abbruch beim ersten `Ja`, Grenze: Benutzerverzeichnis oder Wurzel
//!    │
//!  beruehrt_einen_arbeitsbaum(&Path, Option<&Path>, &[PathBuf])
//!           zuerst der Aufwaertsgang; nur wenn der `Nein` sagt, die Schleife
//!           ueber die Auswahl, ebenfalls mit Abbruch beim ersten `Ja`
//! ```
//!
//! # Keine Anbindung an Git
//!
//! **Es wird nichts von Git gelesen und nichts von Git gerufen.** Geprueft wird,
//! ob ein Eintrag namens `.git` unmittelbar in einem Ordner steht — nicht, ob er
//! ein Verzeichnis oder eine Datei ist, nicht was darin steht, nicht ob der
//! Arbeitsbaum sauber ist, nicht ob der zu loeschende Pfad verfolgt wird. KRK
//! traegt am 260817 keine Git-Anbindung, `Kommando` fuehrt keine einzige
//! Git-Variante, und den Index eines Arbeitsbaums zu lesen waere eine eigene
//! Runde
//! (`shared/decisions/260817-0536_a_sieht-die-git-pruefung-nur-den-ordner-selbst-oder-auch-aufwaerts.md`,
//! Moeglichkeit 4, verworfen als mit den Mitteln dieser Runde nicht
//! entscheidbar).
//!
//! Dass `.git` auch eine **Datei** sein kann — so legt Git einen verknuepften
//! Arbeitsbaum an — ist deshalb kein Sonderfall, der hier zu behandeln waere,
//! sondern genau der Grund, aus dem nach dem Eintrag und nicht nach einem
//! Verzeichnis gefragt wird.
//!
//! # Die Grenze am Benutzerverzeichnis begrenzt allein die Kosten
//!
//! Der Aufwaertsgang endet am mitgegebenen Benutzerverzeichnis oder an der
//! Wurzel, je nachdem, was zuerst erreicht ist. **Welche der beiden Grenzen
//! gilt, ist fuer den Nutzer nicht sichtbar**, und das ist keine Nachlaessigkeit:
//! ein Pfad oberhalb des Benutzerverzeichnisses loest die laute Form schon ueber
//! den ersten Ausloeser aus („ausserhalb des Benutzerordners"), und der steht in
//! der Rangfolge des Specs **vor** dem Git-Arbeitsbaum. Die Rueckfrage wird
//! also ohnehin laut, und sie nennt ohnehin einen anderen Grund. Was die Grenze
//! spart, sind die Zugriffe zwischen dem Benutzerverzeichnis und der Wurzel —
//! auf diesem Geraet drei Ebenen, an denen nichts zu gewinnen ist.
//!
//! **Die Grenze ist einschliessend**: das Benutzerverzeichnis selbst wird noch
//! geprueft, erst danach hoert der Gang auf. Wer es ausliesse, haette einen
//! blinden Fleck genau an der Grenze — ein Benutzerverzeichnis, das selbst ein
//! `.git` traegt, ist keine Erfindung, sondern die gewoehnliche Form eines
//! Arbeitsbaums fuer Konfigurationsdateien. Der Preis ist benannt: liegt dort
//! ein `.git`, ist **jede** Loeschung unterhalb des Benutzerverzeichnisses aus
//! diesem Grund laut. Das ist die Antwort des Nutzers, konsequent
//! weitergerechnet, und nicht ein zusaetzlicher Fall.
//!
//! # Was die Reichweite kostet, und warum sie trotzdem so gewaehlt ist
//!
//! **Der Nutzer hat diese Antwort am Spec-Gate umgedreht.** Seine Festlegung der
//! zweiten Klaerungsrunde lautete: nur der Ordner, der die Verwaltung selbst
//! traegt, warnt. Die Kalibrierung zeigte, dass diese enge Form seinen eigenen
//! Schadensfall vom 260817-0344 nicht trifft — geraeumt wurde
//! `…/krk/fusion-workbench/shared`, und der Arbeitsbaum liegt zwei Ebenen
//! darueber. Er hat daraufhin die aufwaerts sehende Form gewaehlt, gegen die
//! Empfehlung des Shapers.
//!
//! **Der Einwand dagegen bleibt gueltig, und er steht hier, damit er beim ersten
//! lauten Blatt auffindbar ist und nicht neu entdeckt werden muss.** Wer in
//! einem Quellbaum arbeitet, loescht dort taeglich, und nach dieser Festlegung
//! wird jede dieser Loeschungen laut. **In diesem Projekt selbst liegt jeder
//! Pfad unterhalb von `/Users/k1/Projects/productive/krk` in einem
//! Arbeitsbaum**: die laute Form ist dort der Normalfall, die ruhige die
//! Ausnahme. Eine Warnung, die fast immer erscheint, verliert ihre
//! Unterscheidungskraft, und sie verliert sie zuerst dort, wo sie am haeufigsten
//! gesehen wird. Der Nutzer kennt diese Folge und hat sie angenommen; ob sie
//! sich im Gebrauch bestaetigt, ist eine Beobachtung fuer eine spaetere Runde
//! und keine Zusage dieser. Der Spec haelt sie unter C3 und in der Kalibrierung
//! fest.
//!
//! **Wer die Reichweite spaeter zuruecknehmen will, aendert eine Zeile und keine
//! Bauform**: [`liegt_in_arbeitsbaum`] gibt es dann nicht mehr, und
//! [`beruehrt_einen_arbeitsbaum`] fragt nur noch [`traegt_arbeitsbaum`] am
//! Ordner und an der Auswahl. Die Umkehrung ist also billig, und das ist
//! Absicht.
//!
//! # Auf welcher Polaritaet die drei Rueckgabewerte liegen
//!
//! **Auf der ersten**, und alle drei auf derselben:
//! [`Loeschzielbefund::Ja`] ist der Warngrund, und
//! [`Loeschzielbefund::Unentschieden`] gehoert zu ihm. Der Aufrufer fragt
//! [`Loeschzielbefund::ist_warnwuerdig`], nicht auf `Ja` selbst. Die beiden
//! Polaritaeten und warum die Unterscheidung sicherheitsrelevant ist, stehen im
//! Modulkopf von [`super::loeschzielbefund`]; die zweite Polaritaet traegt in
//! dieser Runde allein die Frage nach dem Papierkorb.
//!
//! # Gemerkt wird nichts
//!
//! Kein Zwischenspeicher, kein Zustand, keine Lebensdauer. Die Frage wird je
//! Loeschbefehl genau einmal gestellt, und ein Speicher ueber die Dauer eines
//! Vorgangs haette keinen zweiten Frager. Er waere ausserdem falsch, sobald
//! jemand zwischen zwei Loeschbefehlen ein `.git` anlegt oder wegraeumt.
//!
//! # Warum die dritte Funktion nicht `befund` heisst
//!
//! Die API-Tafel des Plans nannte sie `arbeitsbaum::befund`. Eine Funktion
//! dieses Namens, die einen [`Loeschzielbefund`] liefert, staende neben
//! [`super::modell::Ordnermodell::befund`], das einen [`super::modell::Befund`]
//! liefert — **dieselbe Verwechslung, die der Befund `260817-1419` eine Ebene
//! hoeher gerade aufgeloest hat**, nur eine Ebene tiefer wieder aufgebaut. Der
//! Plan hat die Wahl deshalb dem Ausfuehrenden ueberlassen, mit der Vorgabe, sie
//! **nach ihrer Frage** und nicht nach ihrem Rueckgabetyp zu benennen.
//!
//! Gewaehlt ist [`beruehrt_einen_arbeitsbaum`]. „Beruehren" ist das Verb, das
//! beide Haelften der Frage traegt: der Vorgang **liegt in** einem Arbeitsbaum
//! (aufwaerts) oder er **nimmt einen mit** (die Auswahl). „Liegt in" allein
//! benennte nur die erste Haelfte, und der Name stuende dann zweimal fast
//! gleich im Modul.
//!
//! Verworfen sind daneben:
//!
//! - **`traegt_der_ast_einen_arbeitsbaum`**, der zweite Vorschlag des Plans:
//!   „Ast" ist in diesem Modulbaum kein Wort, es braeuchte hier seine eigene
//!   Definition, und es benennte eine Gestalt im Baum statt der Frage. Dass die
//!   ausgewaehlten Eintraege **unter** dem Ordner haengen und die geprueften
//!   Ebenen **ueber** ihm, macht aus beidem zusammen keinen Ast.
//! - **`liegt_das_loeschziel_darin`**: `Loeschziel` wird im Schritt 9 ein
//!   `struct` mit einem Feld `arbeitsbaum`, das **diese** Funktion fuellt. Der
//!   Name liesse sich als „nimmt ein `&Loeschziel`" und waere damit im Kreis.
//! - **`wird_beruehrt`**: liest am Aufruf `arbeitsbaum::wird_beruehrt` am
//!   schoensten und steht ohne den Modulpfad fuer nichts. Ein Name muss auch
//!   eingefuehrt tragen, und `use …::arbeitsbaum::wird_beruehrt;` ist erlaubt.
//!
//! Dass der Name das Wort seines Moduls wiederholt, ist die Bauform von
//! [`super::inhalt::traegt_der_inhalt`] und kein Versehen.
//!
//! # Die drei stehen nicht in den Wiederausfuhren von [`super`]
//!
//! Aus demselben Grund, aus dem [`super::umfang::zaehlen`] dort nicht steht: der
//! Modulname ist der Gegenstand jeder der drei Fragen, und er soll am Aufruf
//! stehen. `arbeitsbaum::beruehrt_einen_arbeitsbaum(…)` sagt am Aufrufort, wovon
//! die Rede ist; eine Wiederausfuhr waere nachzutragen, sobald ein zweiter
//! Aufrufer zeigt, dass sie fehlt.
//!
//! # Wer sie ruft
//!
//! Zum Zeitpunkt dieses Schrittes niemand. Der Aufrufer entsteht in derselben
//! Runde im Schritt 10, in `krk-ui`, wenn der Anwendungsdelegierte die
//! Tatsachen fuer die Auslesertafel beschafft; er loest den angezeigten Ordner
//! und das Benutzerverzeichnis **einmal je Loeschbefehl** auf und reicht beide
//! hier herein. `dead_code` trifft das Modul trotzdem nicht, denn `krk-core` ist
//! eine Bibliothek und alles hier ist von ihrer Wurzel aus erreichbar; eine
//! Ausnahme nach dem Vorbild von `krk-ui/src/kommandos/rueckschritt.rs` braucht
//! es nicht.
//!
//! Die bindende Grundlage ist
//! `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`
//! und, fuer die Reichweite,
//! `shared/decisions/260817-0536_a_sieht-die-git-pruefung-nur-den-ordner-selbst-oder-auch-aufwaerts.md`.

use std::io;
use std::path::{Path, PathBuf};

use super::Loeschzielbefund;
use super::aufwaerts;

/// Der Eintrag, an dem ein Git-Arbeitsbaum zu erkennen ist.
///
/// Er steht als Konstante da und nicht als Zeichenkette im Rumpf, damit der
/// Name genau einmal im Baum steht und diese Erklaerung einen Ort hat: geprueft
/// wird die **Anwesenheit** dieses Eintrags, nicht seine Art und nicht sein
/// Inhalt. Der Modulkopf sagt, warum das keine Naeherung ist, sondern die ganze
/// Frage dieser Runde.
const VERWALTUNGSEINTRAG: &str = ".git";

/// Steht unmittelbar in diesem Ordner ein Eintrag [`VERWALTUNGSEINTRAG`]?
///
/// Ein `lstat(2)` und kein Abstieg: gefragt wird nach genau einem Namen in genau
/// diesem Ordner. Die Ebenen darueber fragt [`liegt_in_arbeitsbaum`].
///
/// Die vier Ausgaenge, und die Funktion ist ueber sie total:
///
/// - der Eintrag ist da — gleich ob als Verzeichnis, Datei oder Verknuepfung:
///   [`Loeschzielbefund::Ja`];
/// - er ist nicht da: [`Loeschzielbefund::Nein`];
/// - der Pfad ist gar kein Ordner und kann deshalb keinen Eintrag tragen:
///   [`Loeschzielbefund::Nein`]. **Dieser Zweig ist nicht kosmetisch.** Die
///   Auswahl eines Dateifensters traegt gewoehnliche Dateien, und `lstat(2)` auf
///   `datei/.git` scheitert mit `ENOTDIR` und nicht mit `ENOENT`; ohne diesen
///   Zweig machte jede ausgewaehlte Datei die Rueckfrage unentschieden und damit
///   laut;
/// - der Zugriff scheitert anders — keine Rechte, ein zu langer Name, ein
///   Datentraeger, der nicht antwortet: [`Loeschzielbefund::Unentschieden`], und
///   das ist laut. Ein Fehlschlag ist keine Aussage ueber den Ordner, sondern
///   eine ueber KRKs Kenntnis von ihm.
///
/// **Ein relativer Pfad ist [`Loeschzielbefund::Unentschieden`]**, und zwar
/// bevor irgendetwas gelesen wird. Sonst fragte `Path::new("").join(".git")`
/// nach `.git` **im Arbeitsverzeichnis des Prozesses** und lieferte eine Antwort
/// ueber einen Ordner, nach dem niemand gefragt hat. Diese eine Pruefung deckt
/// alle drei Funktionen ab, denn sie ist die einzige Stelle des Moduls, die das
/// Dateisystem anspricht. Die Aufrufer reichen aufgeloeste Pfade herein; der
/// Modulkopf sagt, wer sie aufloest.
///
/// **Zwischenbestandteile des Pfades werden dabei verfolgt**, denn das ist die
/// Bedeutung eines Pfades im Dateisystem und nicht eine Wahl dieses Moduls:
/// zeigt ein ausgewaehlter Verweis auf die Wurzel eines Arbeitsbaums, ist die
/// Antwort `Ja`, obwohl nur der Verweis wegkaeme. Der Fehler geht damit in die
/// laute Richtung, und ihn zu schliessen kostete ein zweites `lstat(2)` je
/// ausgewaehltem Eintrag. Angesichts der Reichweite, die der Modulkopf
/// beschreibt — in einem Quellbaum ist die laute Form der Normalfall — waere
/// das ein Zugriff je Eintrag fuer einen Unterschied, den in diesem Projekt
/// niemand sehen wird.
///
/// Auf der ersten Polaritaet: `Ja` ist der Warngrund, `Unentschieden` gehoert
/// zu ihm.
///
/// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: der Wert ist
/// der einzige Ertrag des Aufrufs, und ohne ihn faellt der fuenfte Ausloeser der
/// lauten Rueckfrage aus, ohne dass irgendwo etwas fehlte.
#[must_use = "der Befund ist der einzige Ertrag des Aufrufs; fallengelassen faellt der Ausloeser aus"]
pub fn traegt_arbeitsbaum(ordner: &Path) -> Loeschzielbefund {
    if !ordner.is_absolute() {
        return Loeschzielbefund::Unentschieden;
    }
    match std::fs::symlink_metadata(ordner.join(VERWALTUNGSEINTRAG)) {
        Ok(_) => Loeschzielbefund::Ja,
        // Die Fallunterscheidung ueber [`io::ErrorKind`] traegt einen
        // Auffangzweig, und sie muss ihn tragen: der Typ ist `non_exhaustive`,
        // eine vollstaendige Aufzaehlung ist dort nicht zu haben. Der
        // Auffangzweig geht deshalb in die vorsichtige Richtung — was wir nicht
        // einordnen koennen, ist unentschieden und damit laut.
        Err(fehler) => match fehler.kind() {
            io::ErrorKind::NotFound | io::ErrorKind::NotADirectory => Loeschzielbefund::Nein,
            _ => Loeschzielbefund::Unentschieden,
        },
    }
}

/// Steckt dieser Ordner in einem Arbeitsbaum — er selbst oder eine Ebene ueber
/// ihm?
///
/// Der Gang beginnt am Ordner selbst und steigt ueber [`aufwaerts`] auf. Er
/// endet, sobald eines von drei Dingen eintritt:
///
/// 1. eine Ebene antwortet [`Loeschzielbefund::Ja`] — der erste Treffer bricht
///    ab;
/// 2. die gerade gepruefte Ebene **ist** das mitgegebene
///    Benutzerverzeichnis — die Grenze ist einschliessend;
/// 3. [`aufwaerts`] liefert `None` — die Wurzel ist erreicht.
///
/// `benutzerverzeichnis` ist ein [`Option`], und `None` heisst „keine Grenze
/// ausser der Wurzel". **Es kommt als Argument herein und wird hier nicht
/// erfragt**; dieselbe Bauform traegt
/// [`crate::ablage::pfade::gekuerzt_fuer_anzeige`], und ihr Grund gilt hier
/// genauso: die Funktion ist damit ohne Zugriff auf das echte
/// Benutzerverzeichnis pruefbar, und der Aufrufer fragt einmal je Loeschbefehl
/// statt einmal je Pruefung.
///
/// # Der Gang endet, und warum das keine Annahme ist
///
/// [`aufwaerts`] nimmt je Schritt genau einen Pfadbestandteil weg und liefert
/// `None`, wenn keiner mehr da ist. Die Zahl der Bestandteile faellt damit
/// streng, und die Schleife laeuft hoechstens so oft, wie der Pfad Bestandteile
/// hat. Ein Ringschluss ueber Verknuepfungen ist nicht moeglich, denn gerechnet
/// wird am Pfad und nicht am Dateisystem.
///
/// # Ein Zweifel unterwegs haelt den Gang nicht an
///
/// Antwortet eine Ebene [`Loeschzielbefund::Unentschieden`], wird der Zweifel
/// ueber [`Loeschzielbefund::oder`] mitgenommen und weitergegangen. Das ist
/// kein Widerspruch zum Abbruch beim ersten Treffer, sondern seine
/// Voraussetzung: ein `Ja` weiter oben ist eine gewusste Tatsache und liefert
/// den **richtigen** Grund fuer die laute Form, wo der Zweifel nur „liess sich
/// nicht einordnen" hergibt. Bleibt es beim Zweifel, ist die Antwort
/// `Unentschieden` und die Rueckfrage laut.
///
/// Auf der ersten Polaritaet: `Ja` ist der Warngrund, `Unentschieden` gehoert
/// zu ihm.
///
/// `#[must_use]`, aus demselben Grund wie bei [`traegt_arbeitsbaum`].
#[must_use = "der Befund ist der einzige Ertrag des Aufrufs; fallengelassen faellt der Ausloeser aus"]
pub fn liegt_in_arbeitsbaum(ordner: &Path, benutzerverzeichnis: Option<&Path>) -> Loeschzielbefund {
    aufwaerts_mit(ordner, benutzerverzeichnis, traegt_arbeitsbaum)
}

/// Beruehrt dieser Loeschvorgang irgendwo einen Arbeitsbaum?
///
/// Die ganze Frage des fuenften Ausloesers, in einem Aufruf: der angezeigte
/// Ordner, jede Ebene ueber ihm bis zur Grenze, und jeder ausgewaehlte Eintrag.
/// Warum die Funktion so heisst und nicht `befund`, steht im Modulkopf.
///
/// # Die Reihenfolge ist die Kostenrechnung
///
/// **Zuerst der Aufwaertsgang, und nur wenn der [`Loeschzielbefund::Nein`] sagt,
/// die Schleife ueber die Auswahl.** Damit kostet die Pruefung im haeufigen Fall
/// — der Nutzer loescht innerhalb eines Projekts — gar keinen Zugriff je
/// ausgewaehltem Eintrag, sondern hoert beim ersten Treffer auf dem Weg nach
/// oben auf. Im seltenen Fall — der angezeigte Ordner liegt in keinem
/// Arbeitsbaum, ein ausgewaehlter Unterordner ist selbst einer — kostet sie ein
/// `lstat(2)` je ausgewaehltem Eintrag, und auch diese Schleife bricht beim
/// ersten Treffer ab.
///
/// **Der Aufwaertsgang schneidet auch mit [`Loeschzielbefund::Unentschieden`]
/// ab, und der Preis dafuer ist benannt**: die Rueckfrage ist dann schon laut,
/// aber ihr Grund heisst „liess sich nicht einordnen" statt „aus einem
/// Git-Arbeitsbaum", obwohl ein ausgewaehlter Eintrag den genauen Grund
/// vielleicht hergegeben haette. Der Plan hat die billigere Form gewaehlt; der
/// Fall verlangt einen Ordner zwischen dem Ziel und der Grenze, der sich nicht
/// lesen laesst, und ist damit selten. Wer ihn anders will, streicht den
/// Abbruch auf `Unentschieden` und zahlt einen Zugriff je Eintrag.
///
/// # Was die Auswahl beitraegt, und was nicht
///
/// Je ausgewaehlter Eintrag genau eine Frage: traegt **er** unmittelbar ein
/// `.git`? Nicht gefragt wird nach seinem Unterbaum — ein Arbeitsbaum drei
/// Ebenen unter einem ausgewaehlten Ordner bleibt unentdeckt. Das ist die
/// Festlegung des Specs und keine Ersparnis: der Abstieg waere ein Durchlauf
/// ueber den ganzen Unterbaum, und der Umfang jeder Auswahl, die so etwas
/// enthaelt, loest die laute Form ueber den sechsten Ausloeser ohnehin aus.
///
/// Nach den Ebenen **ueber** einem ausgewaehlten Eintrag wird ebenfalls nicht
/// gefragt, und das ist keine Luecke: das sind der angezeigte Ordner und alles
/// darueber, und die hat der Aufwaertsgang schon gesehen.
///
/// Eine leere Auswahl ist damit genau der Aufwaertsgang.
///
/// Auf der ersten Polaritaet: `Ja` ist der Warngrund, `Unentschieden` gehoert
/// zu ihm.
///
/// `#[must_use]`, aus demselben Grund wie bei [`traegt_arbeitsbaum`].
#[must_use = "der Befund ist der einzige Ertrag des Aufrufs; fallengelassen faellt der Ausloeser aus"]
pub fn beruehrt_einen_arbeitsbaum(
    ordner: &Path,
    benutzerverzeichnis: Option<&Path>,
    auswahl: &[PathBuf],
) -> Loeschzielbefund {
    beruehrt_mit(ordner, benutzerverzeichnis, auswahl, traegt_arbeitsbaum)
}

// ---------------------------------------------------------------------------
// Die zwei Schleifen, getrennt von dem Zugriff, den sie tun
// ---------------------------------------------------------------------------
//
// Beide bekommen die Pruefung als Argument, und die beiden oeffentlichen
// Funktionen darueber sind je eine Zeile, die [`traegt_arbeitsbaum`]
// einsetzt.
//
// **Das ist keine Vorratsallgemeinheit, sondern die einzige Art, die
// Kostenzusage zu messen statt sie zu behaupten.** „Abbruch beim ersten
// Treffer" ist am Rueckgabewert **nicht** abzulesen: [`Loeschzielbefund::oder`]
// macht `Ja` aufsaugend, und genau deshalb liefert ein Gang, der nach dem
// ersten Treffer weiterlaeuft, dasselbe `Ja`. Eine Probe mit echten Ordnern
// kann die Zusage darum nicht pruefen, gleich wie sie gebaut ist. Mit der
// eingesetzten Pruefung zaehlen die Proben unten die besuchten Ebenen ab und
// vergleichen die **Liste** — das ist eine Messung des Zugriffsmusters.
//
// Was dabei ungemessen bleibt und bleiben muss: dass die oeffentlichen
// Funktionen wirklich [`traegt_arbeitsbaum`] einsetzen. Das ist je eine Zeile
// und mit einem Blick zu pruefen; die Proben in `tests/arbeitsbaum.rs` fahren
// dieselben Faelle ausserdem ueber echte Ordner.

/// Der Aufwaertsgang, mit eingesetzter Pruefung.
fn aufwaerts_mit(
    ordner: &Path,
    benutzerverzeichnis: Option<&Path>,
    mut pruefer: impl FnMut(&Path) -> Loeschzielbefund,
) -> Loeschzielbefund {
    let mut befund = Loeschzielbefund::Nein;
    let mut ebene = ordner.to_path_buf();
    loop {
        befund = befund.oder(pruefer(&ebene));
        // Die Fallunterscheidung ueber den Befund ist vollstaendig und hat
        // keinen Auffangzweig: ein vierter Wert haelt hier den Bau an, statt
        // still den Abbruch zu verlieren.
        match befund {
            Loeschzielbefund::Ja => return befund,
            Loeschzielbefund::Nein | Loeschzielbefund::Unentschieden => {}
        }
        // Die einschliessende Grenze: die Ebene ist geprueft, und weiter geht
        // es nicht.
        if benutzerverzeichnis.is_some_and(|zuhause| ebene.as_path() == zuhause) {
            return befund;
        }
        let Some((eltern, _)) = aufwaerts(&ebene) else {
            return befund;
        };
        ebene = eltern;
    }
}

/// Aufwaertsgang und Auswahl, mit eingesetzter Pruefung.
fn beruehrt_mit(
    ordner: &Path,
    benutzerverzeichnis: Option<&Path>,
    auswahl: &[PathBuf],
    mut pruefer: impl FnMut(&Path) -> Loeschzielbefund,
) -> Loeschzielbefund {
    let hoch = aufwaerts_mit(ordner, benutzerverzeichnis, &mut pruefer);
    match hoch {
        // Beide Werte sind schon laut; die Auswahl koennte den Grund nur noch
        // schaerfen. Der Doc-Kommentar von [`beruehrt_einen_arbeitsbaum`] nennt
        // den Preis.
        Loeschzielbefund::Ja | Loeschzielbefund::Unentschieden => return hoch,
        Loeschzielbefund::Nein => {}
    }

    let mut befund = Loeschzielbefund::Nein;
    for pfad in auswahl {
        befund = befund.oder(pruefer(pfad));
        match befund {
            Loeschzielbefund::Ja => return befund,
            Loeschzielbefund::Nein | Loeschzielbefund::Unentschieden => {}
        }
    }
    befund
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use Loeschzielbefund::{Ja, Nein, Unentschieden};

    /// Eine Pruefung, die aus einer Tafel antwortet und mitschreibt, wen sie
    /// gefragt wurde.
    ///
    /// Sie ist das Messgeraet dieser Proben: die **Liste** der besuchten Pfade
    /// ist die Zusage „Abbruch beim ersten Treffer", und am Rueckgabewert
    /// allein waere sie nicht abzulesen. Der Grund steht ueber
    /// [`aufwaerts_mit`].
    struct Mitschrift {
        /// Pfad und Antwort. Was nicht darin steht, ist [`Nein`].
        tafel: Vec<(&'static str, Loeschzielbefund)>,
        besucht: RefCell<Vec<String>>,
    }

    impl Mitschrift {
        fn neu(tafel: &[(&'static str, Loeschzielbefund)]) -> Self {
            Self {
                tafel: tafel.to_vec(),
                besucht: RefCell::new(Vec::new()),
            }
        }

        fn pruefer(&self) -> impl FnMut(&Path) -> Loeschzielbefund + '_ {
            move |pfad| {
                let text = pfad.to_string_lossy().into_owned();
                self.besucht.borrow_mut().push(text.clone());
                self.tafel
                    .iter()
                    .find(|(name, _)| *name == text)
                    .map_or(Nein, |(_, antwort)| *antwort)
            }
        }

        fn besucht(&self) -> Vec<String> {
            self.besucht.borrow().clone()
        }
    }

    /// Der Gang haelt beim ersten Treffer an und fragt keine Ebene darueber.
    ///
    /// Drei Ebenen werden gefragt, `/a` und `/` nicht mehr. Waere `/a` in der
    /// Liste, liefe der Gang weiter und die Zusage stimmte nicht — am
    /// Rueckgabewert `Ja` waere davon nichts zu sehen.
    #[test]
    fn der_aufwaertsgang_bricht_beim_ersten_treffer_ab() {
        let mitschrift = Mitschrift::neu(&[("/a/b", Ja)]);
        let befund = aufwaerts_mit(Path::new("/a/b/c/d"), None, mitschrift.pruefer());
        assert_eq!(befund, Ja);
        assert_eq!(
            mitschrift.besucht(),
            vec!["/a/b/c/d", "/a/b/c", "/a/b"],
            "der Gang haelt beim ersten Treffer nicht an"
        );
    }

    /// Die Grenze am Benutzerverzeichnis ist einschliessend und haelt danach an.
    ///
    /// `/a` wird noch gefragt — ein Benutzerverzeichnis mit eigenem `.git` ist
    /// kein blinder Fleck —, `/` nicht mehr.
    #[test]
    fn der_gang_prueft_das_benutzerverzeichnis_und_hoert_dort_auf() {
        let mitschrift = Mitschrift::neu(&[("/", Ja)]);
        let befund = aufwaerts_mit(
            Path::new("/a/b/c"),
            Some(Path::new("/a")),
            mitschrift.pruefer(),
        );
        assert_eq!(
            befund, Nein,
            "der Gang hat die Grenze ueberschritten und das `.git` an der Wurzel gefunden"
        );
        assert_eq!(
            mitschrift.besucht(),
            vec!["/a/b/c", "/a/b", "/a"],
            "die Grenze ist nicht einschliessend oder haelt nicht an"
        );
    }

    /// Ein `.git` am Benutzerverzeichnis selbst wird gefunden.
    ///
    /// Die andere Haelfte der einschliessenden Grenze: die Ebene wird nicht nur
    /// besucht, ihre Antwort wirkt auch.
    #[test]
    fn ein_arbeitsbaum_am_benutzerverzeichnis_wird_gefunden() {
        let mitschrift = Mitschrift::neu(&[("/a", Ja)]);
        let befund = aufwaerts_mit(
            Path::new("/a/b"),
            Some(Path::new("/a")),
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Ja);
    }

    /// Ohne Benutzerverzeichnis endet der Gang an der Wurzel.
    #[test]
    fn ohne_benutzerverzeichnis_endet_der_gang_an_der_wurzel() {
        let mitschrift = Mitschrift::neu(&[]);
        let befund = aufwaerts_mit(Path::new("/a/b"), None, mitschrift.pruefer());
        assert_eq!(befund, Nein);
        assert_eq!(mitschrift.besucht(), vec!["/a/b", "/a", "/"]);
    }

    /// Liegt der Ordner nicht unter dem Benutzerverzeichnis, gilt die Wurzel.
    ///
    /// Die Grenze ist „was zuerst erreicht ist", und hier ist das die Wurzel.
    #[test]
    fn ausserhalb_des_benutzerverzeichnisses_gilt_die_wurzel_als_grenze() {
        let mitschrift = Mitschrift::neu(&[]);
        let befund = aufwaerts_mit(
            Path::new("/Volumes/extern/tief"),
            Some(Path::new("/Users/k1")),
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Nein);
        assert_eq!(
            mitschrift.besucht(),
            vec!["/Volumes/extern/tief", "/Volumes/extern", "/Volumes", "/"]
        );
    }

    /// Ein Zweifel unterwegs haelt den Gang nicht an, und ein Treffer darueber
    /// gewinnt.
    #[test]
    fn ein_zweifel_unterwegs_haelt_den_gang_nicht_an() {
        let mitschrift = Mitschrift::neu(&[("/a/b/c", Unentschieden), ("/a", Ja)]);
        let befund = aufwaerts_mit(Path::new("/a/b/c"), None, mitschrift.pruefer());
        assert_eq!(
            befund, Ja,
            "ein Zweifel auf der ersten Ebene hat den Treffer darueber verdeckt"
        );
        assert_eq!(mitschrift.besucht(), vec!["/a/b/c", "/a/b", "/a"]);
    }

    /// Bleibt es beim Zweifel, ist die Antwort unentschieden und damit laut.
    #[test]
    fn ein_zweifel_ohne_treffer_bleibt_unentschieden() {
        let mitschrift = Mitschrift::neu(&[("/a/b", Unentschieden)]);
        let befund = aufwaerts_mit(
            Path::new("/a/b"),
            Some(Path::new("/a")),
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Unentschieden);
        assert!(befund.ist_warnwuerdig());
    }

    /// Die Auswahl wird gar nicht gefragt, wenn der Aufwaertsgang `Ja` sagt.
    ///
    /// Das ist die Kostenzusage der Reihenfolge, und sie ist am Rueckgabewert
    /// nicht abzulesen: `Ja` kaeme in beiden Faellen heraus.
    #[test]
    fn die_auswahl_wird_nur_bei_nein_gefragt() {
        let mitschrift = Mitschrift::neu(&[("/a", Ja), ("/a/b/ausgewaehlt", Ja)]);
        let befund = beruehrt_mit(
            Path::new("/a/b"),
            Some(Path::new("/a")),
            &[PathBuf::from("/a/b/ausgewaehlt")],
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Ja);
        assert_eq!(
            mitschrift.besucht(),
            vec!["/a/b", "/a"],
            "die Auswahl ist gefragt worden, obwohl der Aufwaertsgang schon Ja gesagt hat"
        );
    }

    /// Ein Zweifel im Aufwaertsgang schneidet die Auswahl ebenfalls ab.
    ///
    /// Der Preis steht im Doc-Kommentar von [`beruehrt_einen_arbeitsbaum`]: der
    /// Grund heisst dann „liess sich nicht einordnen", obwohl die Auswahl den
    /// genaueren hergegeben haette. Die Probe haelt die Wahl fest, damit sie
    /// nicht unbemerkt kippt.
    #[test]
    fn ein_zweifel_im_aufwaertsgang_schneidet_die_auswahl_ab() {
        let mitschrift = Mitschrift::neu(&[("/a", Unentschieden), ("/a/b/ausgewaehlt", Ja)]);
        let befund = beruehrt_mit(
            Path::new("/a/b"),
            Some(Path::new("/a")),
            &[PathBuf::from("/a/b/ausgewaehlt")],
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Unentschieden);
        assert_eq!(mitschrift.besucht(), vec!["/a/b", "/a"]);
    }

    /// Die Schleife ueber die Auswahl bricht beim ersten Treffer ab.
    #[test]
    fn die_schleife_ueber_die_auswahl_bricht_beim_ersten_treffer_ab() {
        let mitschrift = Mitschrift::neu(&[("/a/b/zwei", Ja)]);
        let befund = beruehrt_mit(
            Path::new("/a/b"),
            Some(Path::new("/a/b")),
            &[
                PathBuf::from("/a/b/eins"),
                PathBuf::from("/a/b/zwei"),
                PathBuf::from("/a/b/drei"),
            ],
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Ja);
        assert_eq!(
            mitschrift.besucht(),
            vec!["/a/b", "/a/b/eins", "/a/b/zwei"],
            "die Schleife hat nach dem Treffer weitergefragt"
        );
    }

    /// Ein Zweifel in der Auswahl haelt die Schleife nicht an.
    #[test]
    fn ein_zweifel_in_der_auswahl_haelt_die_schleife_nicht_an() {
        let mitschrift = Mitschrift::neu(&[("/a/b/eins", Unentschieden), ("/a/b/drei", Ja)]);
        let befund = beruehrt_mit(
            Path::new("/a/b"),
            Some(Path::new("/a/b")),
            &[
                PathBuf::from("/a/b/eins"),
                PathBuf::from("/a/b/zwei"),
                PathBuf::from("/a/b/drei"),
            ],
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Ja);
        assert_eq!(
            mitschrift.besucht(),
            vec!["/a/b", "/a/b/eins", "/a/b/zwei", "/a/b/drei"]
        );
    }

    /// Eine leere Auswahl ist genau der Aufwaertsgang.
    #[test]
    fn eine_leere_auswahl_ist_der_aufwaertsgang() {
        let mitschrift = Mitschrift::neu(&[]);
        let befund = beruehrt_mit(
            Path::new("/a/b"),
            Some(Path::new("/a")),
            &[],
            mitschrift.pruefer(),
        );
        assert_eq!(befund, Nein);
        assert_eq!(mitschrift.besucht(), vec!["/a/b", "/a"]);
    }

    /// Ein relativer Pfad fragt das Arbeitsverzeichnis nicht, sondern bleibt
    /// unentschieden.
    ///
    /// Diese Probe unterscheidet die Vorpruefung von ihrem Fehlen: ohne sie
    /// fragte der leere Pfad nach `.git` **im Arbeitsverzeichnis des
    /// Testlaufs**, also in `crates/krk-core`, und bekaeme von dort ein
    /// entschiedenes `Nein` — eine Antwort ueber einen Ordner, nach dem niemand
    /// gefragt hat.
    #[test]
    fn ein_relativer_pfad_bleibt_unentschieden() {
        assert_eq!(traegt_arbeitsbaum(Path::new("")), Unentschieden);
        assert_eq!(traegt_arbeitsbaum(Path::new("relativ/tief")), Unentschieden);
        assert_eq!(
            liegt_in_arbeitsbaum(Path::new("relativ/tief"), None),
            Unentschieden
        );
    }
}
