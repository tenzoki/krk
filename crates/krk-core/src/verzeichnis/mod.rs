//! Verzeichnisleser und Ordnermodell.
//!
//! Dreizehn Module, in der Reihenfolge, in der die Daten sie durchlaufen:
//!
//! ```text
//! sys ──> leser ──> eintrag ──> modell <── sortierung
//!  │                     ^         ^ ^
//!  │               kollation       │ │
//!  ├──> durchlauf ─────────────────┘ │
//!  │      ^     ^                    │
//!  │      │     └──── filter ────────┘
//!  │      │                       │
//!  │      └──── inhalt <──────────┘
//!  │
//!  └──> umfang        (liest, haengt aber an keinem der uebrigen)
//!
//! verweisziel        (steht allein, an keinem der anderen)
//!
//! loeschzielbefund   (ein Typ und kein Schritt, an keinem der anderen)
//!        ^
//!        └──── arbeitsbaum   (liest ueber `std::fs`, nicht ueber sys;
//!                             haengt daneben an `aufwaerts` in dieser Datei)
//! ```
//!
//! [`sys`] ist die einzige Stelle im Kern mit einem Fremdaufruf und bindet
//! `getattrlistbulk(2)` fuer das Lesen, seit Schritt 15 `copyfile(3)` und
//! `renamex_np(2)` fuer die Operationsmaschine und seit dem Defekt
//! `260809-1652` `fcntl(2)` fuer `ohne_warten_oeffnen`, den gemeinsamen Eingang
//! von `text::datei::lesen` und, seit dem Defekt `260810-1247`, von
//! `text::datei::bis_zur_grenze_lesen`, seit der Runde 7 `flock(2)` fuer
//! die beiden Sperren der Ablage und seit der Runde 18 `localtime_r(3)` fuer
//! die buergerliche Ortszeit eines Zeitpunkts. Das sind sechs Schnittstellen
//! und zehn gebundene Funktionen, denn `copyfile(3)` braucht seine vier
//! `copyfile_state_*`-Helfer. [`leser`] macht aus der ersten der sechs
//! Schnittstellen den gestueckelten Lesevorgang auf einem Arbeitsfaden. [`eintrag`] beschreibt, was ein Eintrag traegt, und
//! laesst sich von [`kollation`] die beiden Sortierschluessel bauen.
//! [`modell`] haelt Eintraege und Sichtreihenfolge getrennt, und [`sortierung`]
//! liefert die acht Ordnungen.
//!
//! [`filter`] steht als einziges Modul **unter** mehreren anderen und nicht in
//! der Kette: es traegt die drei Regeln des Filters, welche Zeichen
//! aufgenommen werden, wann ein Name den Filtertext traegt und ab welcher
//! Laenge der Filtertext auch Inhalte meint, und jede Regel steht dort je
//! einmal. Der Vergleich hat seit der Runde 11 drei Rufer, [`modell`] fuer die
//! angezeigte Zeile, [`durchlauf`] fuer jeden Namen im Unterbaum und
//! [`inhalt`] fuer den Text einer Datei; zwei Fassungen davon hiessen, dass
//! eine tiefe Suche etwas anderes faende als eine flache und der Inhalt etwas
//! anderes als der Name. Bis zum 260815 hiess das Modul `sprungmarke` und trug
//! die Sprungmarke aus C2 der Runde 1, die die Runde 10 abgeloest hat.
//!
//! [`inhalt`] steht wie [`filter`] neben der Kette und beantwortet die eine
//! Frage, die der Name allein nicht beantwortet: traegt der **Text** dieser
//! Datei die Folge? Gelesen wird ueber
//! [`crate::text::datei::bis_zur_grenze_lesen`] und hoechstens so weit, wie
//! der Aufrufer es zulaesst; verglichen wird mit derselben Regel aus
//! [`filter`], die schon ueber Namen entscheidet. **Wer ihn ruft, haelt den
//! Abbruch**: [`inhalt`] beantwortet eine Frage ueber eine Datei und weiss von
//! Faeden nichts. Sein Rufer ist deshalb der [`durchlauf`], der den Abbruch
//! schon fuehrt und die Frage je Datei stellt.
//!
//! [`durchlauf`] steht neben [`leser`] und nicht unter ihm: er liest ueber
//! dieselbe Huelle `sys::Schwungleser` und auf derselben Bauart, beantwortet
//! aber eine andere Frage. Der Leser liefert den Bestand eines Ordners, der
//! Durchlauf je Auftrag einen Wahrheitswert — ueber den ganzen Unterbaum eines
//! Ordners oder, seit der Runde 11, ueber den Text einer gewoehnlichen Datei.
//! **Zwei Auftragsarten in einer Maschine und nicht zwei Maschinen**: beide
//! Fragen entstehen nebenlaeufig, tragen einen Eintragsindex und lassen die
//! Sicht neu aufbauen. Er ist die sechste Eingabe des Pruefschritts in
//! [`modell`], und die einzige, die von aussen kommt.
//!
//! [`verweisziel`] steht wie [`filter`] neben der Kette und nicht in ihr, und
//! haengt als einziges lesendes Modul an gar keinem anderen: es beantwortet die eine
//! Frage, die der Leser bewusst offenlaesst, naemlich worauf eine Verknuepfung
//! zeigt. Gefragt wird sie am Namen ueber `std::fs::metadata`, und erst dann,
//! wenn jemand in eine Verknuepfung einsteigen will; der Lesevorgang bekommt
//! dafuer keinen zusaetzlichen Systemaufruf, weil an seiner Rechnung die
//! Zusagen L3 und L10 haengen. Bis zum 260815 fragte das Modul am Deskriptor
//! und hing dafuer an [`sys`]; warum das die falsche Frage war, steht in
//! seinem eigenen Modulkopf.
//!
//! [`umfang`] haengt wie [`durchlauf`] unmittelbar an [`sys`] und an keinem
//! der uebrigen: es beantwortet die eine Frage, die vor einer Rueckfrage ueber
//! ein Loeschziel zu stellen ist, naemlich ob mehr als
//! [`umfang::SCHWELLE`] Eintraege an der Auswahl haengen. Gelesen wird ueber
//! dieselbe Huelle `sys::Schwungleser`, gezaehlt wird bis zu einem Deckel, und
//! **die Bauform ist die des [`durchlauf`]**: ein Ordner ganz, seine
//! Unterordner als Pfad auf einen Stapel, zu jedem Zeitpunkt genau ein offener
//! Verzeichnisdeskriptor. Der Deckel begrenzt die Kosten, nicht die Deskriptoren;
//! warum das zwei verschiedene Zusagen sind und welche davon der Defekt
//! `260815-0211` verletzt hatte, steht in seinem eigenen Modulkopf.
//!
//! **[`umfang::SCHWELLE`] und [`umfang::zaehlen`] stehen bewusst nicht in den
//! Wiederausfuhren unten**, [`Umfang`] dagegen schon. Ein nacktes `SCHWELLE`
//! oder `zaehlen` an dieser Ebene sagte nicht, wovon es die Schwelle ist und
//! was es zaehlt; der Modulname traegt diese Auskunft und soll sie am Aufruf
//! tragen. Der Typ dagegen nennt seinen Gegenstand selbst.
//!
//! [`loeschzielbefund`] ist das einzige Modul hier, das **nichts liest**: kein
//! Systemaufruf, kein Deskriptor, kein Pfad. Es traegt die dreiwertige Antwort
//! [`Loeschzielbefund`] und die eine Verknuepfung darauf, und es steht in diesem
//! Verzeichnis, weil die Fragen, die es beantwortet, am Dateisystem entschieden
//! werden und nicht am Fenster. Die dritte Antwort `Unentschieden` ist die
//! Verallgemeinerung dessen, was [`sys::ist_deskriptormangel`] seit der Runde 10
//! am [`durchlauf`] leistet: ein Mangel von aussen laesst einen Auftrag
//! unentschieden, statt ihn negativ zu entscheiden. Warum die Loeschrunde diese
//! Unterscheidung braucht und warum sie nicht auf einen Wahrheitswert
//! zusammenfaellt, steht in seinem eigenen Modulkopf.
//!
//! **Er heisst ausdruecklich nicht `Befund`, denn dieser Name gehoert hier einem
//! anderen Typ.** Der Wortstamm traegt in diesem Modulbaum mehrere Typen, und
//! diese drei gehoeren zusammen: [`modell::Befund`], die dreiwertige Auskunft
//! `Unentschieden`/`Treffer`/`KeinTreffer` darueber, ob ein Eintrag die getippte
//! Folge traegt; [`Befundmeldung`], mit der der [`durchlauf`] sie meldet; und
//! [`Inhaltsbefund`], der dieselbe Frage fuer den Text einer Datei beantwortet.
//! [`Loeschzielbefund`] gehoert nicht zu ihnen: er beantwortet keine Frage des
//! Filters, sondern die Pruefungen der Loeschrunde an einem Loeschziel.
//! Bis zum 260817 hiessen er und [`modell::Befund`] beide `Befund`, und dieser
//! Absatz hier erklaerte den einen, ohne den anderen zu nennen — genau daran
//! entstand der Befund
//! (`issues/260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`).
//! Welcher der beiden umbenannt wurde und warum nicht der andere, steht im
//! Modulkopf von [`loeschzielbefund`].
//!
//! [`arbeitsbaum`] beantwortet den fuenften Ausloeser derselben Runde: beruehrt
//! dieser Loeschvorgang einen Git-Arbeitsbaum? Es ist das einzige lesende Modul
//! hier, das **nicht** ueber [`sys`] liest, und es liest auch nicht ueber
//! [`leser`]: gefragt wird nach einem einzigen Namen in einem einzigen Ordner,
//! und dafuer reicht `std::fs::symlink_metadata`, also `lstat(2)`. Der Gegenstand
//! ist die **Anwesenheit** eines Eintrags `.git` und nicht sein Inhalt; eine
//! Anbindung an Git entsteht dabei nicht. Es haengt an [`loeschzielbefund`] fuer
//! die Antwort und an [`aufwaerts`] in dieser Datei fuer den Gang nach oben —
//! dessen einziger anderer Rufer ist die Navigation in `krk-ui`.
//!
//! **Seine drei Funktionen stehen bewusst nicht in den Wiederausfuhren unten**,
//! aus demselben Grund wie [`umfang::zaehlen`]: der Modulname ist der Gegenstand
//! jeder der drei Fragen, und `arbeitsbaum::beruehrt_einen_arbeitsbaum(…)` sagt
//! am Aufrufort, wovon die Rede ist. Was die Reichweite des Aufwaertsgangs
//! kostet — in diesem Projekt selbst wird nach ihm fast jede Loeschung laut —
//! steht in seinem eigenen Modulkopf; der Nutzer kennt die Folge und hat sie
//! angenommen.
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

use std::path::{Path, PathBuf};

pub mod arbeitsbaum;
pub mod durchlauf;
pub mod eintrag;
pub mod filter;
pub mod inhalt;
pub mod kollation;
pub mod leser;
pub mod loeschzielbefund;
pub mod modell;
pub mod sortierung;
pub mod sys;
pub mod umfang;
pub mod verweisziel;

pub use durchlauf::{Auftrag, Auftragsart, Befundmeldung, Durchlauf};
pub use eintrag::{Eintrag, Typ};
pub use filter::Muster;
pub use inhalt::{Inhaltsbefund, traegt_der_inhalt};
pub use leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
pub use loeschzielbefund::Loeschzielbefund;
pub use modell::{Markierungsstand, Ordnermodell};
pub use sortierung::{Richtung, Schluessel, Sortierung};
pub use umfang::Umfang;
pub use verweisziel::Verweisziel;

/// Der uebergeordnete Ordner und der Name des verlassenen (C2).
///
/// C2 verlangt beim Aufstieg, dass die Auswahl auf dem Ordner steht, aus dem
/// der Nutzer gerade kam. Der Name dafuer ist reine Pfadarithmetik und steht
/// deshalb im Kern und nicht in der Oberflaeche: er ist ohne Fenster pruefbar,
/// und `krk-ui` haengt allein die Navigation daran.
///
/// **Seit der Loeschrunde hat er einen zweiten Rufer im Kern selbst**, den
/// Aufwaertsgang in [`arbeitsbaum`]. Der braucht von den beiden
/// Rueckgabewerten nur den ersten und rechnet gerade darauf, dass die Wurzel
/// `None` liefert: das ist seine eine Abbruchbedingung neben dem
/// Benutzerverzeichnis, und sie ist auch das Argument dafuer, dass sein Gang
/// endet.
///
/// `None` fuer die Wurzel, die keinen uebergeordneten Ordner hat. Ein Aufstieg
/// von `/` ist damit kein Sonderfall mit eigener Meldung, sondern schlicht
/// keine Bewegung.
pub fn aufwaerts(ordner: &Path) -> Option<(PathBuf, String)> {
    let name = ordner.file_name()?.to_string_lossy().into_owned();
    let eltern = ordner.parent()?;
    Some((eltern.to_path_buf(), name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn der_aufstieg_nennt_den_verlassenen_ordner() {
        let (eltern, name) = aufwaerts(Path::new("/Users/k1/Projekte"))
            .expect("ein Ordner unterhalb der Wurzel hat einen uebergeordneten");
        assert_eq!(eltern, Path::new("/Users/k1"));
        assert_eq!(name, "Projekte");
    }

    #[test]
    fn der_aufstieg_aus_der_wurzel_fuehrt_nirgendwohin() {
        assert_eq!(aufwaerts(Path::new("/")), None);
    }

    #[test]
    fn ein_abschliessender_schraegstrich_aendert_nichts() {
        let (eltern, name) =
            aufwaerts(Path::new("/Users/k1/")).expect("derselbe Ordner, anders geschrieben");
        assert_eq!(eltern, Path::new("/Users"));
        assert_eq!(name, "k1");
    }
}
