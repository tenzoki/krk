//! Was das Kontextmenue der Dateiliste traegt und worauf jeder Eintrag wirkt
//! (Runde 17).
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Gebaut wird das Menue in
//! `crate::appkit::tabelle`, ausgefuehrt wird es beim Anwendungsdelegierten;
//! **was** darin steht und **worauf** es zeigt, steht hier und ist ohne Fenster
//! pruefbar.
//!
//! ```text
//!  betroffene Eintraege ─┐
//!  angezeigter Ordner ───┼──> packziel()     ──> die Quellen und der Pfad des Archivs
//!                        │      └─> archivname()
//!                        │
//!  sichtbare Zeilen ─────┴──> entpackziel()  ──> Archive mit ihren Zielordnern
//! ```
//!
//! # Die eine Frage dieses Moduls
//!
//! Sie lautet: **was traegt das Kontextmenue, und worauf wirkt jeder Eintrag.**
//! Nicht dazu gehoert, ob eine Datei sich als Archiv oeffnen laesst — das ist
//! aus einem Namen nicht entscheidbar, und der Plan dieser Runde sagt es
//! ausdruecklich. Der Eintrag steht unabhaengig davon da, der Vorgang versucht
//! das Oeffnen, und ein Scheitern nennt die Abschlussliste mit dem Wortlaut der
//! Kiste. An der Stelle, an der die Frage entscheidbar ist, steht damit eine
//! Entscheidung, und an der anderen eine Meldung statt einer Vermutung.
//!
//! # Warum die Suche ueber die sichtbaren Zeilen laeuft
//!
//! [`entpackziel`] fragt [`Ordnermodell::zeilen`] und nicht den ungefilterten
//! Bestand — aus demselben Grund, aus dem
//! [`super::operationen::betroffene`] allein die sichtbaren Eintraege zaehlt:
//! ein Eintrag, den der Nutzer beim Klicken nicht vor sich hatte, gehoert nicht
//! in den Auftrag. Steht ein Filtertext, sieht Unzip damit dieselbe Liste, die
//! der Nutzer vor sich hat. Der Datensatz
//! `decisions/260825-0727_*_nimmt-unzip-die-betroffenen-eintraege-oder-allein-die-ausgewaehlte-zeile.md`
//! schreibt es in seinen Randbedingungen aus.
//!
//! # Drei Nutzerentscheidungen tragen dieses Modul
//!
//! Alle drei liegen als Datensaetze in `decisions/` dieses Circles und sind am
//! 260824-2120 beantwortet:
//!
//! 1. **Der Archivname haengt die Endung an, statt sie zu ersetzen**
//!    (`260825-0711_*_wie-heisst-das-archiv-einer-einzelnen-datei-mit-endung.md`,
//!    Moeglichkeit 1). Aus `bericht.txt` wird `bericht.txt.zip`.
//!    **Allein diese Wahl macht das Paar [`archivname`] und
//!    [`ordnername_zum_archiv`] umkehrbar**: unter der ersetzenden Regel wuerde
//!    aus `bericht.txt` das Archiv `bericht.zip` und daraus der Ordner
//!    `bericht`, und der Ursprungsname waere verloren. Da beide Befehle im
//!    selben Menue stehen, saehe der Nutzer den Verlust unmittelbar. Wo der
//!    Rundweg endet, endet er in beiden Richtungen gleich; die eine Stelle,
//!    an der das entschieden wird, ist [`brauchbarer_stamm`].
//! 2. **Ein Archiv wird an der Endung erkannt, ohne Ruecksicht auf Gross- und
//!    Kleinschreibung und ohne Dateizugriff**
//!    (`260825-0711_*_woran-erkennt-unzip-dass-eine-datei-ein-zip-ist.md`,
//!    Moeglichkeit 1). Die Pruefung an den ersten Bytes ist ausdruecklich der
//!    spaetere Ausbau und steht deshalb nicht daneben.
//! 3. **Unzip wirkt auf die betroffenen Eintraege und entpackt jedes Archiv
//!    darin**
//!    (`260825-0727_*_nimmt-unzip-die-betroffenen-eintraege-oder-allein-die-ausgewaehlte-zeile.md`,
//!    Moeglichkeit 3). Drei markierte Archive ergeben drei Zielordner in einem
//!    Vorgang; deshalb liefert [`entpackziel`] eine Liste von Paaren und passt
//!    damit auf `Art::Entpacken { ziele }`.
//!
//! # Was die Endungsregel kostet, und warum sie trotzdem eine bleibt
//!
//! Sie fragt den Namen und nicht die Sache. Ein Ordner, der `sicherung.zip`
//! heisst, wird deshalb als Archiv angeboten, und der Vorgang ueberspringt ihn
//! mit dem Grund, den die Kiste nennt. Der Ausweg waere, in
//! [`entpackziel`] den Typ des [`Eintrag`](krk_core::verzeichnis::Eintrag)
//! mitzulesen — er steht dort ohne Dateizugriff da. Er ist nicht genommen,
//! weil er die Regel entzweite: die betroffenen Eintraege kommen als blosse
//! Pfade herein und trueegen den Typ nicht mit, und dann entschiede ueber den
//! Ordner `sicherung.zip` einmal so und einmal anders, je nachdem, ob er
//! markiert ist. Eine Regel mit einem seltenen, gemeldeten Fehlversuch ist
//! besser als zwei Regeln, die auseinanderlaufen koennen.
//!
//! # Das Ziel eines Laufs liegt nie auf einer seiner Quellen
//!
//! **Die Zusage steht hier und nicht im Kern**, weil hier beide Listen
//! zusammenkommen und sonst nirgends: [`packziel`] rechnet das Archiv aus
//! denselben betroffenen Eintraegen, die es als Quellen weiterreicht, und
//! [`entpackziel`] rechnet jeden Zielordner aus einem Archiv, das neben den
//! uebrigen in derselben Liste steht. Der Kern bekommt dafuer ausdruecklich
//! keinen Pfadvergleich (Nutzerantwort vom 260825 auf
//! `issues/260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-papierkorb-*`,
//! der zweite und kleinere der zwei Wege).
//!
//! Beide Gestalten entstehen von selbst, sobald derselbe Befehl ein zweites Mal
//! auf denselben Ordner faellt:
//!
//! ```text
//!  Zip, zweiter Lauf     Projekte/{a.txt, Projekte.zip}  ──> Projekte/Projekte.zip
//!  Unzip ueber beide     Projekte/{a.zip, a.zip.zip}     ──> Projekte/a.zip
//! ```
//!
//! In beiden Faellen traegt die Markierung den gerechneten Zielpfad selbst. Er
//! **faellt** deshalb aus den Quellen heraus ([`ist_ziel_des_laufs`]); die
//! Rueckfrage des Vorgangs greift danach wie sonst, denn der Zieleintrag steht
//! ja weiterhin auf der Platte. Ohne diesen Schnitt raeumte „Ueberschreiben"
//! eine Quelle desselben Laufs in den Papierkorb, und der Lauf meldete sie
//! danach als ausgelassen.
//!
//! **Drei Bestimmungen gehoeren zum Schnitt, und alle drei sind Antworten auf
//! die dritte Durchsicht dieser Runde:**
//!
//! 1. **Verglichen wird der letzte Bestandteil ohne Ruecksicht auf die
//!    Schreibung**, denn das Bauziel faltet sie und [`ist_zipname`] tut es
//!    ausdruecklich mit. Was das kostet, steht bei [`ist_ziel_des_laufs`].
//! 2. **Der Entpackschnitt ist ein Festpunkt**: ein Archiv faellt nur fuer
//!    einen Anspruch, den ein **bleibender** Lauf erhebt. Aus der Kette
//!    `{a.zip, a.zip.zip, a.zip.zip.zip}` bleiben zwei Paare und nicht eines;
//!    siehe [`ohne_die_eigenen_ziele`].
//! 3. **Der Schnitt bleibt nicht stumm.** Wie viele Eintraege er genommen hat,
//!    nennt der Abschlusstext des Vorgangs
//!    ([`super::operationen::abschlusstext`]). Ein Befehl, der wortlos weniger
//!    tut, als der Nutzer markiert hat, waere genau der Ausgang, den der
//!    Doc-Kommentar von [`brauchbarer_stamm`] fuenfzig Zeilen weiter unten
//!    zurueckweist.
//!
//! # Wer hier hereinruft
//!
//! Zwei Stellen, und beide stehen unter `crate::appkit`. Die Datenquelle der
//! Dateiliste (`tabelle`) fragt beim Menuebau [`Kontextbefehl::ALLE`],
//! [`Kontextbefehl::titel`] und [`Kontextbefehl::menuemarke`], beim Klick
//! [`Kontextbefehl::von_menuemarke`] und in ihrem `entpackbefund` das
//! [`entpackziel`] — dort und nicht beim Ausfuehrenden, weil diese Regel die
//! **sichtbaren Zeilen** braucht und das Ordnermodell jener Quelle gehoert. Die
//! Ausfuehrung beim Anwendungsdelegierten fragt [`packziel`] und liest den
//! [`Entpackbefund`], den sie sich von der Quelle geben laesst.
//!
//! **Bis zur Runde 17 stand hier `expect(dead_code)` am ganzen Modul**, mit
//! einem Ablaufdatum: `krk-ui` hat kein Bibliotheksziel, also ist `pub` hier
//! keine Wurzel, und bis zum ersten Aufrufer meldete der Uebersetzer jedes
//! Stueck als unbenutzt. Mit dem Ausfuehrungszweig ist die Erwartung
//! unerfuellt geworden und die Ausnahme gefallen — so, wie es eine Ausnahme
//! mit Ablaufdatum soll.

use std::cmp::Reverse;
use std::path::{Path, PathBuf};

use krk_core::operation::umbenennen::{Namensfehler, name_pruefen, namen_teilen};
use krk_core::verzeichnis::Ordnermodell;

/// Die Endung, an der ein Archiv erkannt und mit der eines benannt wird.
///
/// **Eine Zeichenfolge fuer beide Richtungen.** [`ist_zipname`] vergleicht
/// gegen sie, [`archivname`] haengt sie an, und [`ordnername_zum_archiv`]
/// nimmt sie wieder ab. Zwei Schreibweisen nebeneinander waeren genau die Lage,
/// in der der Rundweg spaeter unbemerkt aufhoert zu schliessen.
const ENDUNG: &str = ".zip";

/// Der Stamm, wenn die Rechnung keinen **brauchbaren** Namen hergibt.
///
/// Zwei Lagen fuehren hierher, und die zweite ist seit dem 260825 dabei:
///
/// 1. **Es steht kein Name da.** Der Fall ist das Wurzelverzeichnis:
///    `Path::new("/").file_name()` liefert `None`, und ein Archiv namens `.zip`
///    waere keines (siehe [`ist_zipname`]).
/// 2. **Es steht einer da, der keiner ist.** Aus dem Archiv `..zip` faellt der
///    Stamm `.`, aus `...zip` der Stamm `..`; beide weist
///    [`name_pruefen`] ab, und [`brauchbarer_stamm`] fragt es.
///
/// Ein Ersatzname ist in beiden Lagen die richtige Antwort und keine Notluege:
/// der Nutzer sieht ihn im Konfliktblatt und in der Statuszeile, bevor
/// irgendetwas entsteht.
const ERSATZSTAMM: &str = "Archiv";

/// Was das Kontextmenue der Dateiliste an eigenen Eintraegen traegt.
///
/// **Der Freigabeeintrag steht nicht darin, und das ist der Zuschnitt und keine
/// Auslassung.** Er kommt als `standardShareMenuItem` vom System, traegt dessen
/// Ziel und dessen Handlung, und KRK fuehrt ihn nicht aus (siehe
/// `crate::appkit::teilen`). Diese Aufzaehlung beschreibt, was KRK **selbst**
/// ausfuehrt.
///
/// **Sie ist zugleich die Sperre gegen den Menueeintrag, der nichts tut.** Die
/// drei Eintraege teilen sich einen Selektor und unterscheiden sich allein in
/// ihrer Marke; die Ausfuehrung beim Anwendungsdelegierten verzweigt ueber
/// diesen Wert vollstaendig und ohne Auffangzweig. Ein vierter Wert haelt damit
/// den Bau an, statt still nichts zu tun — die Falle, die `CLAUDE.md` fuer
/// Tastenbefehle beschreibt und die hier ein `NSMenuItem` waere, dessen
/// Selektor nirgends ankommt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kontextbefehl {
    /// Die betroffenen Eintraege in ein Archiv im angezeigten Ordner packen.
    Zippen,
    /// Jedes betroffene Archiv in einen eigenen neuen Ordner entpacken.
    Entpacken,
    /// Den angezeigten Ordner im Finder oeffnen.
    ImFinderZeigen,
}

impl Kontextbefehl {
    /// Alle drei Befehle, in der Reihenfolge, in der sie im Menue stehen.
    ///
    /// **Kein `#[cfg(test)]` davor, anders als bei
    /// `Fokus::ALLE` in [`super::fokus`]**: der Menuebau laeuft ueber
    /// diese Liste und baut nicht drei Eintraege von Hand. Damit ist die
    /// Reihenfolge im Menue dieselbe Angabe wie die Reihenfolge hier, und ein
    /// vierter Befehl erscheint, ohne dass jemand eine zweite Stelle nachzieht.
    ///
    /// **Die Feldbreite steht in der Typangabe.** Ein vierter Wert haelt damit
    /// den Bau an, wie es die Aufzaehlungen dieses Baums durchweg tun; die
    /// Vollstaendigkeit der Liste selbst erzwingt der Uebersetzer nicht, und
    /// dafuer steht die Probe `die_tafel_nennt_jeden_befehl_genau_einmal`.
    pub const ALLE: [Kontextbefehl; 3] = [
        Kontextbefehl::Zippen,
        Kontextbefehl::Entpacken,
        Kontextbefehl::ImFinderZeigen,
    ];

    /// Der Titel, den der Nutzer im Menue liest.
    ///
    /// **Zwei tragen den Namen des Werkzeugs, einer die Handlung**, und das ist
    /// keine Nachlaessigkeit: „Zip" und „Unzip" hat der Nutzer im Entwurf
    /// dieser Runde selbst so genannt, und beide sind als Werkzeugnamen
    /// gelaeufiger als jede deutsche Fassung. „Finder" allein waere dagegen ein
    /// Hauptwort ohne Handlung; der Eintrag oeffnet den angezeigten Ordner, und
    /// genau das sagt sein Titel.
    #[must_use]
    pub fn titel(self) -> &'static str {
        match self {
            Kontextbefehl::Zippen => "Zip",
            Kontextbefehl::Entpacken => "Unzip",
            Kontextbefehl::ImFinderZeigen => "Im Finder öffnen",
        }
    }

    /// Die Marke, die der Menueeintrag traegt (`NSMenuItem::setTag:`).
    ///
    /// **Die Zaehlung beginnt bei eins und nicht bei null, und daran haengt
    /// eine Zusage.** Ein `NSMenuItem`, an dem niemand eine Marke gesetzt hat,
    /// traegt die Null; begaenne die Zaehlung dort, liefe jeder solche Eintrag
    /// beim Zurueckrechnen auf [`Kontextbefehl::Zippen`] hinaus. So liefert
    /// [`Kontextbefehl::von_menuemarke`] fuer ihn `None`, und der Aufrufer tut
    /// nichts, statt zu packen.
    ///
    /// **Sie heisst nicht schlicht `marke`, und das hat einen nachpruefbaren
    /// Grund.** Diesen Namen traegt in dieser Kiste bereits die Abwurfmarke aus
    /// [`super::abwurfregel`], und deren Zusage „genau ein Aufrufer" wird von
    /// einer Zaehlprobe ueber den Quelltext gehalten
    /// (`quellbaum::aufrufstellen`). Eine Zaehlung ueber Namen kann
    /// zwei Begriffe desselben Namens nicht trennen: die kurze Fassung hier
    /// liess jene Probe mit sechs Aufrufern rot werden. Der Name ist damit
    /// nicht nur genauer, sondern die Bedingung dafuer, dass die fremde Zusage
    /// stehen bleibt.
    #[must_use]
    pub fn menuemarke(self) -> isize {
        match self {
            Kontextbefehl::Zippen => 1,
            Kontextbefehl::Entpacken => 2,
            Kontextbefehl::ImFinderZeigen => 3,
        }
    }

    /// Der Befehl zu einer Marke, sofern sie einen benennt.
    ///
    /// Die Umkehrung von [`Kontextbefehl::menuemarke`]. `None` heisst: diese Marke
    /// gehoert keinem der eigenen Eintraege — die Null eines ungesetzten
    /// `NSMenuItem` eingeschlossen.
    ///
    /// **Gerechnet wird ueber [`Kontextbefehl::ALLE`] und nicht mit einer
    /// zweiten Tafel.** Eine zweite Verzweigung von Hand haette dieselben drei
    /// Zahlen ein zweites Mal getragen, und die erste Abweichung zwischen
    /// beiden waere ein Menueeintrag, der den falschen Befehl ausloest.
    #[must_use]
    pub fn von_menuemarke(marke: isize) -> Option<Kontextbefehl> {
        Kontextbefehl::ALLE
            .into_iter()
            .find(|befehl| befehl.menuemarke() == marke)
    }
}

/// Was Unzip vorgefunden hat.
///
/// **Eine Aufzaehlung und kein `Option`**, damit der Aufrufer nicht aus einem
/// leeren Wert raten muss, welcher der beiden Fehlbefunde vorlag: sie tragen
/// verschiedene Saetze in der Statuszeile
/// ([`super::operationen::kein_archiv`] und
/// [`super::operationen::mehrere_archive`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entpackbefund {
    /// Diese Archive werden entpackt, jedes in den Ordner daneben.
    ///
    /// **Eine Liste und nicht ein einzelnes Archiv**, weil der Nutzer am
    /// 260824-2120 gewaehlt hat, dass Unzip jedes Archiv unter den betroffenen
    /// Eintraegen entpackt. Die Paare passen Stelle fuer Stelle auf
    /// `Auftrag::entpacken`, das sie in `quellen` und
    /// `Art::Entpacken { ziele }` auftrennt.
    ///
    /// Nie leer: eine leere Menge ist [`Entpackbefund::Keines`].
    Archive {
        /// Archiv und Zielordner, Stelle fuer Stelle.
        paare: Vec<(PathBuf, PathBuf)>,
        /// Wie viele **markierte** Archive [`ohne_die_eigenen_ziele`] aus dem
        /// Lauf genommen hat.
        ///
        /// **Die Zahl reist mit, weil der Aufrufer sie nicht nachrechnen
        /// kann.** Beim Packen kann er es: dort haelt er die betroffenen
        /// Eintraege selbst in der Hand, und die Zahl der geschnittenen ist die
        /// Differenz zu den Quellen, die [`packziel`] zurueckgibt. Hier hat er
        /// keine Ausgangsmenge, gegen die er zaehlen koennte — [`entpackziel`]
        /// entscheidet zwischen zwei Regeln, und unter der Ersatzregel hat die
        /// Markierung mit dem Ergebnis gar nichts zu tun.
        ///
        /// Ohne sie naehme der Schnitt markierte Eintraege aus dem Lauf, und
        /// kein Wort erreichte den Nutzer
        /// (`issues/260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-*`).
        /// Gesagt wird es im Abschlusstext des Vorgangs
        /// ([`super::operationen::abschlusstext`]) und nicht in einer eigenen
        /// Meldung davor: eine Befehlsantwort steht ueber der Vorgangsanzeige
        /// und verdeckte den Fortschritt, den sie ankuendigt.
        ausgelassen: usize,
    },
    /// Weder unter den betroffenen Eintraegen noch unter den sichtbaren Zeilen
    /// steht ein Archiv.
    Keines,
    /// Die sichtbaren Zeilen tragen mehr als ein Archiv, und unter den
    /// betroffenen Eintraegen steht keines.
    ///
    /// **Der Befund gilt allein der Ersatzregel.** Mehrere **betroffene**
    /// Archive sind kein Fehlbefund, sondern der Regelfall aus der dritten
    /// Nutzerentscheidung: sie werden alle entpackt.
    Mehrere,
}

/// Die eine Regel, woran ein Archiv erkannt wird.
///
/// Der Name endet auf [`ENDUNG`], ohne Ruecksicht auf Gross- und
/// Kleinschreibung. **Kein Dateizugriff**, und das ist die Bedingung und keine
/// Sparsamkeit: dieselbe Regel durchsucht bei jedem Rechtsklick die ganze
/// sichtbare Liste, und eine Regel, die den Inhalt lesen wollte, oeffnete dabei
/// jede Datei darin.
///
/// **Getrennt wird ueber
/// [`namen_teilen`]** und nicht
/// ueber `str::ends_with`. Der Unterschied ist der fuehrende Punkt: eine Datei,
/// die schlicht `.zip` heisst, ist nach jener Regel ein Stamm ohne Endung und
/// damit **kein** Archiv. Genau so muss es sein, denn ihr Zielordner traege
/// sonst den leeren Namen.
#[must_use]
pub fn ist_zipname(name: &str) -> bool {
    let (_, endung) = namen_teilen(name);
    endung.eq_ignore_ascii_case(ENDUNG)
}

/// Ein gerechneter Stamm, sofern er als Name taugt — sonst [`ERSATZSTAMM`].
///
/// **Die eine Sperre gegen einen Zielordner, der keiner ist.** Aus dem Archiv
/// `..zip` liefert [`namen_teilen`] den Stamm `.`, aus `...zip` den Stamm `..`.
/// `Path::join` normalisiert nichts, und `<angezeigter Ordner>/..` trifft beim
/// `symlink_metadata` der Zielordnerklaerung den **Elternordner**: wer im
/// Konfliktblatt dann „Ueberschreiben" waehlt, gibt den angezeigten Ordner oder
/// dessen Elternordner an den Papierkorb, und wer es nicht waehlt, bekommt den
/// Archivinhalt eine Ebene zu hoch geschrieben
/// (`issues/260825-0942_*_ein-archivname-aus-punkten-macht-den-angezeigten-ordner-oder-seinen-elternordner-zum-entpackziel.md`).
///
/// **Die zwei gebauten Sperren des Kerns sehen das nicht, und das ist kein
/// Versaeumnis an ihnen.** `ZipFile::enclosed_name` und `kette_anlegen`
/// versperren jeden Weg **aus dem Zielordner heraus**, und beide rechnen dabei
/// relativ zu dem `ziel`, das diese Kiste ihnen reicht. Ist dieses `ziel` schon
/// falsch, geschieht der Ausbruch vor ihnen und nicht an ihnen vorbei. Die
/// Pruefung gehoert deshalb hierher, wo der Name entsteht.
///
/// **Gefragt wird [`name_pruefen`] und keine eigene Punktregel.** Was kein Name
/// ist, beantwortet dieses Projekt an einer Stelle; die Zielordnerklaerung des
/// Entpackens stellt dieselbe Frage im Zweig `Konfliktantwort::UmbenennenIn`
/// bereits. Eine zweite Fassung daneben deckte heute dieselben Faelle und liefe
/// spaeter auseinander. Nebenbei faengt die bestehende auch den Stamm `␣␣` aus
/// `␣␣.zip`, den eine Regel ueber Punkte durchgelassen haette.
///
/// # Warum ein Ersatzname und nicht eine Meldung
///
/// Die Directive dieser Runde sagt: die drei Eintraege sind immer da und immer
/// bedienbar, und wo ein Befehl **nichts vorfindet**, meldet er es in der
/// Statuszeile. Unzip findet hier aber etwas vor — der Nutzer hat auf eine
/// Datei geklickt, die die Endung sichtbar traegt.
/// [`super::operationen::kein_archiv`] („hier steht keine Datei mit der Endung
/// .zip") waere vor seinen Augen die Unwahrheit. Und den Eintrag
/// stillschweigend aus [`Entpackbefund::Archive`] zu nehmen waere schlechter
/// als beides: von drei markierten Archiven bliebe eines ohne Ordner und ohne
/// Wort.
///
/// Unbrauchbar ist allein der **gerechnete Name**, und fuer genau diesen Fall
/// steht die Antwort seit dem Wurzelverzeichnis schon da. Sie wird deshalb
/// erweitert und nicht verdoppelt: aus „kein Name" wird „kein brauchbarer
/// Name", und die Statuszeile bleibt der Lage vorbehalten, in der wirklich
/// nichts dasteht.
///
/// **Zwei solche Archive nebeneinander zielen damit auf denselben Ordner.** Das
/// ist keine neue Lage: `a.zip` neben `a.ZIP` tut es seit dem ersten Tag dieser
/// Runde, weil die Endung ohne Ruecksicht auf die Schreibung erkannt wird. Die
/// Zielordnerklaerung des Vorgangs fragt beim zweiten nach, einmal je Archiv.
///
/// **Kein Auffangzweig ueber [`Namensfehler`].** Alle vier Gruende fuehren auf
/// dieselbe Antwort, und ein fuenfter soll den Bau anhalten statt still
/// mitzulaufen — so wie es die Aufzaehlungen dieses Baums durchweg tun.
fn brauchbarer_stamm(stamm: &str) -> String {
    match name_pruefen(stamm) {
        Ok(()) => stamm.to_owned(),
        Err(
            Namensfehler::Leer
            | Namensfehler::Schraegstrich
            | Namensfehler::Nullbyte
            | Namensfehler::Punktname,
        ) => ERSATZSTAMM.to_owned(),
    }
}

/// Der volle Pfad des Archivs, das Zip anlegt.
///
/// Es entsteht immer im **angezeigten** Ordner, auch wenn die betroffenen
/// Eintraege anderswo laegen; so hat der Nutzer es in der zweiten
/// Klaerungsrunde des Shapers beschrieben.
///
/// Der Stamm kommt aus zwei Quellen, und die Fallunterscheidung ist die des
/// Nutzers:
///
/// | betroffene Eintraege | Stamm | Beispiel |
/// |---|---|---|
/// | genau einer | sein voller Name, Endung eingeschlossen | `bericht.txt` → `bericht.txt.zip` |
/// | mehrere | der Name des angezeigten Ordners | `Projekte` → `Projekte.zip` |
/// | keiner | ebenso | — |
///
/// **Die Endung wird angehaengt und nicht ersetzt**, und daran haengt die
/// Umkehrbarkeit; siehe den Modulkopf. Ein Ordner traegt in aller Regel keine
/// Endung, sodass die zweite Zeile derselben Regel folgt und kein Sonderfall
/// ist: aus `Projekte` wird `Projekte.zip`.
///
/// **Hier wird nichts getrennt, und das ist die Folge der Nutzerwahl.** Der
/// Plan dieser Runde sah vor, auch diese Rechnung ueber
/// [`namen_teilen`] zu fuehren;
/// die anhaengende Regel braucht Stamm und Endung gar nicht auseinander. Die
/// eine Trennung des Paares steht deshalb allein in
/// [`ordnername_zum_archiv`] und in [`ist_zipname`], und eine zweite daneben
/// gibt es nicht.
///
/// **Auch dieser Stamm geht durch [`brauchbarer_stamm`]**, obwohl `.` und `..`
/// ihn nicht erreichen koennen: `Path::file_name` liefert fuer beide `None`,
/// und der Ersatzstamm steht dort ohnehin. Erreichen kann ihn der leere Stamm,
/// etwa aus einer Datei namens `␣␣`. Vor allem aber haelt der gemeinsame Weg
/// das Paar symmetrisch — beide Richtungen antworten auf einen unbrauchbaren
/// Namen dasselbe, statt dass die eine ihn durchliesse und die andere ihn
/// ersetzte.
///
/// **Ein leeres `betroffen` erreicht diese Funktion**, seit die Frage „gibt es
/// etwas zu packen" hinter [`packziel`] steht: gerechnet wird zuerst, gefragt
/// wird an dessen Ergebnis. Der Name aus dem angezeigten Ordner faellt dabei an
/// und wird verworfen, denn der Aufrufer meldet
/// [`super::operationen::nichts_zu_packen`] und stellt keinen Auftrag. Die
/// Rechnung bleibt darum vollstaendig: eine Luecke waere an der Stelle
/// unbrauchbar, an der sie geprueft wird.
#[must_use]
pub fn archivname(betroffen: &[PathBuf], ordner: &Path) -> PathBuf {
    let stamm = match betroffen {
        [einziger] => einziger.file_name(),
        _ => None,
    }
    .or_else(|| ordner.file_name())
    .map_or_else(
        || ERSATZSTAMM.to_owned(),
        |name| brauchbarer_stamm(&name.to_string_lossy()),
    );
    ordner.join(format!("{stamm}{ENDUNG}"))
}

/// Worauf Zip wirkt: die Quellen des Laufs und der Pfad seines Archivs.
///
/// **Die eine Stelle, an der beide Listen zusammenkommen**, und deshalb die
/// Stelle, an der das Ziel von den Quellen getrennt wird. Ein Eintrag, dessen
/// Pfad dem gerechneten Archivnamen gleicht, faellt aus den Quellen heraus: er
/// ist das Archiv des vorigen Laufs, und es wandert nicht in sich selbst. Wie
/// die Lage entsteht, steht im Modulkopf.
///
/// **Gerechnet wird der Name aus der ungefilterten Markierung, und danach wird
/// geschnitten.** Ein zweiter Durchgang ueber die verbliebenen Quellen taete
/// etwas anderes: aus `{a.txt, Projekte.zip}` wuerde nicht mehr
/// `Projekte.zip`, sondern `a.txt.zip`, weil dann nur noch ein Eintrag
/// dastuende. Der Nutzer bekaeme bei jedem zweiten Lauf einen anderen
/// Archivnamen als beim ersten, und die Zusage, die [`archivname`] gibt, haette
/// zwei Fassungen.
///
/// **Die Liste kommt nicht leer zurueck, wenn etwas markiert war.** Bei einem
/// einzelnen Eintrag traegt das Ziel dessen Namen **und** die angehaengte
/// Endung, ist also ein anderer Name; bei mehreren traegt es den Namen des
/// angezeigten Ordners, und den kann hoechstens einer der Eintraege tragen.
/// Der Aufrufer fragt die Leere trotzdem, und mit derselben Meldung wie bei
/// einer leeren Markierung: eine Zusage, die nur ein Beweis in Prosa haelt,
/// gehoert nicht zwischen den Nutzer und einen leeren Auftrag.
///
/// **Was geschnitten wurde, sagt die Laengendifferenz**, und deshalb gibt diese
/// Funktion keine dritte Zahl heraus: `betroffen.len() - quellen.len()` ist die
/// Zahl der Eintraege, die der Schnitt genommen hat, und der Aufrufer haelt
/// beide Listen. Er meldet sie im Abschlusstext des Vorgangs, damit der Nutzer
/// nicht wortlos weniger bekommt, als er markiert hat
/// (`issues/260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-*`).
/// Die Entpackseite kann das nicht nachrechnen und traegt die Zahl deshalb im
/// Befund mit; der Grund steht bei [`Entpackbefund::Archive`].
#[must_use]
pub fn packziel(betroffen: &[PathBuf], ordner: &Path) -> (Vec<PathBuf>, PathBuf) {
    let ziel = archivname(betroffen, ordner);
    let quellen = betroffen
        .iter()
        .filter(|pfad| !ist_ziel_des_laufs(pfad, std::slice::from_ref(&ziel)))
        .cloned()
        .collect();
    (quellen, ziel)
}

/// Der Name des Ordners, in den ein Archiv entpackt wird.
///
/// Die Umkehrung von [`archivname`]: die Endung faellt ab, alles davor bleibt
/// stehen. Aus `bericht.txt.zip` wird `bericht.txt`, aus `Projekte.zip` wird
/// `Projekte`.
///
/// **Ein Name ohne die Endung bleibt, wie er ist.** Der Fall entsteht nicht
/// ueber [`entpackziel`], das allein Archivnamen weiterreicht; er entsteht,
/// wenn jemand die Funktion einzeln ruft. Der Ordner traegt dann den Namen des
/// Archivs selbst und stiesse mit ihm zusammen — beantwortet wird das nicht
/// hier, sondern von der Zielordnerklaerung des Vorgangs, die einen freien
/// Namen daneben waehlt.
///
/// Zurueck kommt ein **Name** und kein Pfad: wo der Ordner entsteht, sagt der
/// angezeigte Ordner und nicht das Archiv.
///
/// **Und es kommt ein Name zurueck, der einer ist.** Beide Wege — der Stamm des
/// Archivnamens wie der unveraenderte Name ohne Endung — muenden in
/// [`brauchbarer_stamm`]; aus `..zip` und `...zip` kommt damit [`ERSATZSTAMM`]
/// und nicht `.` oder `..`. **Die Pruefung steht hier und nicht in [`paar`]**,
/// obwohl [`paar`] heute der einzige Weg ins Dateisystem ist: die Zusage „das
/// ist ein Name" gehoert der Funktion, die den Namen herausgibt, und nicht
/// einem ihrer Aufrufer. Sonst gaebe der oeffentliche Rundweg weiterhin `..`
/// heraus, und der naechste Aufrufer — der Ausfuehrungszweig aus Schritt 7 —
/// muesste die Pruefung ein zweites Mal mitbringen.
#[must_use]
pub fn ordnername_zum_archiv(archiv: &Path) -> String {
    let Some(name) = archiv.file_name() else {
        return ERSATZSTAMM.to_owned();
    };
    let name = name.to_string_lossy();
    let stamm = if ist_zipname(&name) {
        namen_teilen(&name).0
    } else {
        &name
    };
    brauchbarer_stamm(stamm)
}

/// Worauf Unzip wirkt: welche Archive gemeint sind und wohin jedes geht.
///
/// Die Regel steht in zwei Stufen, und die erste ist die bestehende:
///
/// 1. **Die betroffenen Eintraege**, also die Markierung und ersatzweise die
///    ausgewaehlte Zeile ([`super::operationen::betroffene`]). Jedes Archiv
///    darunter wird entpackt; drei markierte ergeben drei Zielordner in **einem**
///    Vorgang.
/// 2. **Die Ersatzregel**, wenn darunter keines steht: das eine Archiv unter
///    den sichtbaren Zeilen. Liegt dort keines, kommt
///    [`Entpackbefund::Keines`] zurueck, liegen mehrere,
///    [`Entpackbefund::Mehrere`]; beide meldet der Aufrufer in der
///    Statuszeile.
///
/// **Eine zweite Auswahlregel entsteht damit nicht.** Der Nutzer hat sie fuer
/// Zip abgelehnt, und das Kontextmenue traegt beide Befehle nebeneinander; zwei
/// Eintraege desselben Menues, die auf verschiedene Mengen wirken, saehe man
/// dem Menue nicht an.
///
/// **Gesucht wird ueber [`Ordnermodell::zeilen`]**, also ueber die sichtbaren
/// Zeilen; siehe den Modulkopf. `betroffen` ist von derselben Herkunft, weil
/// [`super::operationen::betroffene`] ebenfalls nur sichtbare Eintraege zaehlt.
///
/// Der Zielordner entsteht im **angezeigten** Ordner und nicht neben dem
/// Archiv. Beides faellt heute zusammen, weil die betroffenen Eintraege aus
/// ebendiesem Ordner kommen; ausgeschrieben steht es, weil die Zusage dem
/// angezeigten Ordner gilt und nicht dem Zufall.
///
/// **Ein Archiv, das derselbe Lauf schon als Zielordner beansprucht, faellt aus
/// den Quellen heraus** ([`ohne_die_eigenen_ziele`]). Die Lage stellt die
/// anhaengende Endungsregel selbst her: neben `a.zip` steht nach einem Zip-Lauf
/// `a.zip.zip`, und der Zielordner des zweiten ist der Pfad des ersten. Die
/// Ersatzregel darunter braucht den Schnitt nicht, denn sie liefert genau ein
/// Paar, und ein Archiv ist nie sein eigenes Ziel: sein Zielname ist der um die
/// Endung gekuerzte Archivname und damit ein anderer.
#[must_use]
pub fn entpackziel(modell: &Ordnermodell, betroffen: &[PathBuf], ordner: &Path) -> Entpackbefund {
    let markierte: Vec<(PathBuf, PathBuf)> = betroffen
        .iter()
        .filter(|pfad| ist_archivpfad(pfad))
        .map(|pfad| paar(pfad.clone(), ordner))
        .collect();
    let markiert = markierte.len();
    let betroffene_archive = ohne_die_eigenen_ziele(markierte);
    if !betroffene_archive.is_empty() {
        return Entpackbefund::Archive {
            ausgelassen: markiert - betroffene_archive.len(),
            paare: betroffene_archive,
        };
    }

    let mut sichtbare = modell
        .zeilen()
        .filter(|eintrag| ist_zipname(&eintrag.name))
        .map(|eintrag| ordner.join(&eintrag.name));
    let Some(erstes) = sichtbare.next() else {
        return Entpackbefund::Keines;
    };
    if sichtbare.next().is_some() {
        return Entpackbefund::Mehrere;
    }
    // Die Ersatzregel schneidet nichts: sie liefert genau ein Paar, und ein
    // Archiv ist nie sein eigenes Ziel.
    Entpackbefund::Archive {
        paare: vec![paar(erstes, ordner)],
        ausgelassen: 0,
    }
}

/// Ob der letzte Bestandteil eines Pfades ein Archivname ist.
///
/// Die Bruecke von [`ist_zipname`], das einen Namen prueft, zu den betroffenen
/// Eintraegen, die als Pfade hereinkommen. Ein Pfad ohne letzten Bestandteil
/// — das Wurzelverzeichnis — ist keiner.
fn ist_archivpfad(pfad: &Path) -> bool {
    pfad.file_name()
        .is_some_and(|name| ist_zipname(&name.to_string_lossy()))
}

/// Ein Archiv mit dem Ordner, in den es entpackt wird.
///
/// Die eine Stelle, an der das Paar entsteht; beide Zweige von [`entpackziel`]
/// gehen durch sie, damit der Zielordner nicht an zwei Stellen gerechnet wird.
fn paar(archiv: PathBuf, ordner: &Path) -> (PathBuf, PathBuf) {
    let ziel = ordner.join(ordnername_zum_archiv(&archiv));
    (archiv, ziel)
}

/// Ob ein Eintrag als Quelle ausfaellt, weil derselbe Lauf ihn schon als Ziel
/// beansprucht.
///
/// **Die eine Fassung der Regel, und sie hat zwei Rufer**: [`packziel`] haelt
/// die betroffenen Eintraege gegen das eine Archiv des Packlaufs,
/// [`ohne_die_eigenen_ziele`] jedes Archiv gegen die Zielordner aller Paare
/// desselben Entpacklaufs. Zwei Fassungen nebeneinander waeren die Lage, in der
/// die eine Gestalt des Befundes behoben bleibt und die andere zurueckkommt.
///
/// Verglichen werden **Pfade, wie sie dastehen**, ohne `canonicalize` und ohne
/// Ruecksicht auf Verknuepfungen. Der Weg dahin ist bei beiden Rufern derselbe:
/// beide Listen entstehen als `ordner.join(name)` ueber demselben angezeigten
/// Ordner ([`super::operationen::betroffene`] auf der einen, [`archivname`] und
/// [`paar`] auf der anderen Seite). Ein Zugriff auf die Platte gehoert nicht in
/// dieses Modul, und deshalb steht hier ein Vergleich und keine Frage an das
/// Dateisystem.
///
/// **Der letzte Bestandteil wird ohne Ruecksicht auf Gross- und
/// Kleinschreibung verglichen, alles davor buchstabengetreu** — die Wahl des
/// Nutzers vom 260825 auf
/// `issues/260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-*`,
/// Moeglichkeit 1. Der Grund liegt bei [`archivname`]: es **bildet** den
/// Zielnamen und haengt dabei die kleingeschriebene Konstante [`ENDUNG`] an,
/// waehrend [`ist_zipname`] ein vorhandenes Archiv ausdruecklich ohne
/// Ruecksicht auf die Schreibung erkennt. Ein Eintrag `PROJEKTE.ZIP` und das
/// gerechnete Ziel `Projekte.zip` sind auf dem Bauziel derselbe Eintrag; ein
/// buchstabengetreuer Vergleich hielt sie fuer zwei, und „Ueberschreiben" im
/// Konfliktblatt raeumte damit doch wieder eine Quelle desselben Laufs in den
/// Papierkorb.
///
/// **Die Ungenauigkeit, die das kostet, gehoert dazu:** APFS laesst sich
/// gross-/kleinschreibungsempfindlich formatieren, und auf einem so
/// formatierten Datentraeger sind `Projekte.zip` und `PROJEKTE.ZIP` zwei
/// Dateien. Dort faellt gelegentlich eine Quelle heraus, die keine Kollision
/// waere. Der Nutzer verliert dabei nichts — der Eintrag bleibt, wie er ist —,
/// ihm fehlt einer im Archiv. Die genaue Antwort braeuchte die Platte, und die
/// hat dieses Modul nicht.
fn ist_ziel_des_laufs(pfad: &Path, ziele: &[PathBuf]) -> bool {
    ziele.iter().any(|ziel| gleicher_eintrag(ziel, pfad))
}

/// Ob zwei gerechnete Pfade denselben Eintrag meinen.
///
/// Die eine Stelle, an der die Faltung steht; warum sie gefaltet wird und was
/// sie kostet, steht bei [`ist_ziel_des_laufs`].
///
/// **Ohne letzten Bestandteil bleibt nichts zu falten.** Das Wurzelverzeichnis
/// und ein Pfad, der auf `..` endet, gehen deshalb durch den buchstabengetreuen
/// Vergleich; sie erreichen diese Stelle nicht, denn [`brauchbarer_stamm`]
/// haelt beide Rechnungen davon ab, einen solchen Namen herauszugeben.
fn gleicher_eintrag(einer: &Path, anderer: &Path) -> bool {
    match (einer.file_name(), anderer.file_name()) {
        (Some(dieser), Some(jener)) => {
            einer.parent() == anderer.parent()
                && dieser
                    .as_encoded_bytes()
                    .eq_ignore_ascii_case(jener.as_encoded_bytes())
        }
        _ => einer == anderer,
    }
}

/// Die Paare eines Entpacklaufs ohne die Archive, die er schon als Zielordner
/// beansprucht.
///
/// Jedes Archiv wird gegen **alle** Ziele des Laufs gehalten und nicht nur
/// gegen sein eigenes.
///
/// **Der Schnitt ist ein Festpunkt und keine einmalige Runde.** Der Unterschied
/// wird an der Kette sichtbar, die die anhaengende Endungsregel dieser Runde
/// selbst herstellt: aus `{a.zip, a.zip.zip, a.zip.zip.zip}` bleiben **zwei**
/// Paare. `a.zip.zip` faellt, weil `a.zip.zip.zip` dorthin entpackt; `a.zip`
/// bleibt stehen, denn sein einziger Beansprucher ist eben gefallen und erhebt
/// den Anspruch nicht mehr. Bis zum 260825 entstand die Zielliste einmal ueber
/// **alle** Paare, und danach wurde gefiltert; dabei fiel `a.zip` fuer einen
/// Anspruch, den niemand mehr erhob, und der Nutzer bekam ein entpacktes Archiv
/// statt zweier
/// (`issues/260825-1249_*_der-entpackschnitt-ist-kein-festpunkt-*`).
///
/// **Entschieden wird vom laengsten Archivpfad zum kuerzesten**, und das ist
/// genau die Reihenfolge, in der ein Beansprucher vor dem Beanspruchten
/// drankommt. Sie folgt aus [`paar`] und ist keine Annahme: ein Zielname ist der
/// um [`ENDUNG`] gekuerzte Archivname und damit vier Zeichen kuerzer. Wo er das
/// nicht ist, traegt er [`ERSATZSTAMM`] — und der endet nicht auf `.zip`, kann
/// also gar kein Archiv dieser Liste treffen.
///
/// **Herausgegeben wird trotzdem in der Reihenfolge der Eingabe**, denn das ist
/// die Reihenfolge, in der die Eintraege vor dem Nutzer stehen; die
/// Laengenordnung entscheidet allein, wer bleibt.
///
/// **Leer kommt die Liste nicht zurueck, solange sie es nicht schon war.** Das
/// laengste Archiv kommt zuerst dran, und die Zielliste ist dann noch leer.
fn ohne_die_eigenen_ziele(paare: Vec<(PathBuf, PathBuf)>) -> Vec<(PathBuf, PathBuf)> {
    let mut reihenfolge: Vec<usize> = (0..paare.len()).collect();
    reihenfolge.sort_by_key(|stelle| Reverse(paare[*stelle].0.as_os_str().len()));

    let mut bleibt = vec![true; paare.len()];
    let mut ziele: Vec<PathBuf> = Vec::with_capacity(paare.len());
    for stelle in reihenfolge {
        if ist_ziel_des_laufs(&paare[stelle].0, &ziele) {
            bleibt[stelle] = false;
        } else {
            ziele.push(paare[stelle].1.clone());
        }
    }

    paare
        .into_iter()
        .zip(bleibt)
        .filter_map(|(paar, bleibt)| bleibt.then_some(paar))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Component;

    use krk_core::verzeichnis::{Eintrag, Typ};

    use super::*;

    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag::mit_versteckt(
            name.to_owned(),
            0,
            std::time::SystemTime::UNIX_EPOCH,
            typ,
            false,
        )
    }

    fn modell_mit(namen: &[&str]) -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen(namen.iter().map(|name| eintrag(name, Typ::Datei)));
        modell.abschliessen();
        modell
    }

    fn ordner() -> &'static Path {
        Path::new("/tmp/Projekte")
    }

    // ------------------------------------------------------------------
    // Die drei Befehle des Menues
    // ------------------------------------------------------------------

    /// Titel und Marke je Befehl, von Hand geschrieben.
    ///
    /// **Von Hand und nicht aus [`Kontextbefehl::titel`] abgeleitet**, nach dem
    /// Vorbild von `TAFEL` in `crate::appkit::teilen`. Eine Ableitung pruefte
    /// die Verzweigung gegen sich selbst und liefe mit jeder Aenderung
    /// stillschweigend mit; hier stuende dann ein umbenannter Menueeintrag in
    /// keiner Probe.
    const TAFEL: [(Kontextbefehl, &str, isize); 3] = [
        (Kontextbefehl::Zippen, "Zip", 1),
        (Kontextbefehl::Entpacken, "Unzip", 2),
        (Kontextbefehl::ImFinderZeigen, "Im Finder öffnen", 3),
    ];

    /// Die Tafel ueber alle drei Werte, an einem Stueck.
    #[test]
    fn jeder_befehl_traegt_seinen_titel_und_seine_marke() {
        for (befehl, titel, marke) in TAFEL {
            assert_eq!(
                befehl.titel(),
                titel,
                "{befehl:?} traegt einen anderen Titel"
            );
            assert_eq!(
                befehl.menuemarke(),
                marke,
                "{befehl:?} traegt eine andere Marke"
            );
        }
    }

    /// Die zweite Haelfte der Vollstaendigkeit.
    ///
    /// Der Uebersetzer erzwingt, dass [`Kontextbefehl::titel`] jeden Wert
    /// beantwortet, aber nicht, dass die Tafel jeden nennt. Ohne diese Probe
    /// liefe ein vierter Befehl ungeprueft mit, obwohl `titel` ihn einordnen
    /// musste.
    #[test]
    fn die_tafel_nennt_jeden_befehl_genau_einmal() {
        for wert in Kontextbefehl::ALLE {
            assert_eq!(
                TAFEL.iter().filter(|(befehl, ..)| *befehl == wert).count(),
                1,
                "{wert:?} steht nicht genau einmal in der Tafel"
            );
        }
        assert_eq!(TAFEL.len(), Kontextbefehl::ALLE.len());
    }

    /// Der Rundweg Marke → Wert → Marke ueber alle drei Befehle.
    ///
    /// Er ist die Zusage, an der der eine Selektor haengt: der Menueeintrag
    /// traegt nichts als seine Marke, und der Anwendungsdelegierte rechnet
    /// allein daraus zurueck, welcher Befehl gemeint war.
    #[test]
    fn der_rundweg_ueber_die_marke_schliesst() {
        for befehl in Kontextbefehl::ALLE {
            assert_eq!(
                Kontextbefehl::von_menuemarke(befehl.menuemarke()),
                Some(befehl),
                "{befehl:?} kommt ueber seine Marke nicht zurueck"
            );
        }
    }

    /// Keine zwei Befehle teilen sich eine Marke.
    ///
    /// Ohne diese Probe waere der Rundweg darueber auch mit zwei gleichen
    /// Marken gruen, solange der erste Treffer der richtige ist.
    #[test]
    fn keine_marke_steht_zweimal() {
        for befehl in Kontextbefehl::ALLE {
            assert_eq!(
                Kontextbefehl::ALLE
                    .iter()
                    .filter(|anderer| anderer.menuemarke() == befehl.menuemarke())
                    .count(),
                1,
                "die Marke von {befehl:?} steht nicht nur bei ihm"
            );
        }
    }

    /// Die Null eines ungesetzten `NSMenuItem` benennt keinen Befehl.
    ///
    /// **Die Zusage, um derentwillen die Zaehlung bei eins beginnt.** Ein
    /// Menueeintrag, an dem niemand `setTag:` gerufen hat, traegt die Null;
    /// begaenne die Zaehlung dort, loeste er das Packen aus.
    #[test]
    fn die_null_und_alles_daneben_benennen_keinen_befehl() {
        for marke in [-1, 0, 4, 99] {
            assert_eq!(
                Kontextbefehl::von_menuemarke(marke),
                None,
                "die Marke {marke} benennt einen Befehl"
            );
        }
    }

    // ------------------------------------------------------------------
    // Woran ein Archiv erkannt wird
    // ------------------------------------------------------------------

    /// Die Endung entscheidet, und die Schreibung nicht.
    #[test]
    fn die_endung_entscheidet_ohne_ruecksicht_auf_die_schreibung() {
        for name in ["sicherung.zip", "sicherung.ZIP", "sicherung.Zip"] {
            assert!(ist_zipname(name), "{name} gilt nicht als Archiv");
        }
    }

    /// Was kein Archiv ist.
    ///
    /// **`.zip` steht als eigener Fall dabei**: ein fuehrender Punkt ist nach
    /// [`namen_teilen`] keine
    /// Endung, und der Zielordner dieser Datei traege sonst den leeren Namen.
    #[test]
    fn ein_name_ohne_die_endung_ist_kein_archiv() {
        for name in [
            "bericht.txt",
            "zip",
            "archiv.zip.txt",
            ".zip",
            "",
            "sicherung.",
        ] {
            assert!(!ist_zipname(name), "{name} gilt als Archiv");
        }
    }

    // ------------------------------------------------------------------
    // Der Name des Archivs und der Rueckweg
    // ------------------------------------------------------------------

    /// Ein einzelner Eintrag gibt dem Archiv seinen vollen Namen.
    ///
    /// Die vier Lagen aus der Pruefstrategie des Plans, an einem Stueck: ein
    /// Ordner, eine Datei mit Endung, eine Datei ohne Endung und mehrere
    /// markierte Eintraege.
    #[test]
    fn der_archivname_haengt_die_endung_an() {
        let ordner = ordner();
        for (betroffen, erwartet) in [
            (vec![ordner.join("Unterlagen")], "Unterlagen.zip"),
            (vec![ordner.join("bericht.txt")], "bericht.txt.zip"),
            (vec![ordner.join("liesmich")], "liesmich.zip"),
            (
                vec![ordner.join("a.txt"), ordner.join("b.txt")],
                "Projekte.zip",
            ),
        ] {
            assert_eq!(
                archivname(&betroffen, ordner),
                ordner.join(erwartet),
                "{betroffen:?} ergibt nicht {erwartet}"
            );
        }
    }

    /// Ohne betroffenen Eintrag kommt der Name vom angezeigten Ordner.
    ///
    /// Der Aufrufer faengt die leere Menge vorher ab; die Rechnung bleibt
    /// trotzdem vollstaendig.
    #[test]
    fn ohne_betroffenen_eintrag_zaehlt_der_angezeigte_ordner() {
        assert_eq!(archivname(&[], ordner()), ordner().join("Projekte.zip"));
    }

    /// Das Wurzelverzeichnis hat keinen Namen und bekommt den Ersatzstamm.
    #[test]
    fn das_wurzelverzeichnis_bekommt_den_ersatzstamm() {
        assert_eq!(
            archivname(&[], Path::new("/")),
            PathBuf::from("/Archiv.zip")
        );
    }

    /// Der Rundweg Name → Archiv → Name, ueber die vier Namensgestalten.
    ///
    /// **Allein die anhaengende Regel macht ihn moeglich.** Ersetzte
    /// [`archivname`] die Endung, kaeme aus `bericht.txt` der Ordner `bericht`
    /// zurueck, und der Ursprungsname waere verloren; die Probe schriebe dann
    /// rot an. Sie ist damit die Stelle, an der die Nutzerentscheidung zum
    /// Archivnamen im Baum haengt.
    #[test]
    fn archivname_und_ordnername_kehren_einander_um() {
        let ordner = ordner();
        for name in ["Unterlagen", "bericht.txt", "liesmich", "archiv.tar.gz"] {
            let archiv = archivname(&[ordner.join(name)], ordner);
            assert_eq!(
                ordnername_zum_archiv(&archiv),
                name,
                "der Rundweg ueber {name} schliesst nicht"
            );
        }
    }

    /// Ein Name ohne die Endung bleibt beim Rueckweg stehen.
    #[test]
    fn ein_name_ohne_endung_bleibt_beim_rueckweg_stehen() {
        assert_eq!(
            ordnername_zum_archiv(Path::new("/tmp/bericht.txt")),
            "bericht.txt"
        );
        assert_eq!(ordnername_zum_archiv(Path::new("/")), "Archiv");
    }

    /// Wo die Umkehrbarkeit endet, endet sie in beide Richtungen gleich.
    ///
    /// **Die Probe zieht die Grenze, die [`brauchbarer_stamm`] gesetzt hat.**
    /// Der Name `␣␣` ist auf macOS anlegbar und faellt in [`Namensfehler::Leer`];
    /// beide Richtungen antworten darauf mit [`ERSATZSTAMM`]. Wuerde nur eine
    /// von beiden pruefen, entstuende aus `␣␣` das Archiv `␣␣.zip` und daraus
    /// der Ordner `Archiv` — zwei Regeln, wo eine steht.
    #[test]
    fn ein_unbrauchbarer_stamm_bekommt_in_beiden_richtungen_den_ersatz() {
        let ordner = ordner();
        assert_eq!(
            archivname(&[ordner.join("  ")], ordner),
            ordner.join("Archiv.zip")
        );
        assert_eq!(ordnername_zum_archiv(&ordner.join("  .zip")), ERSATZSTAMM);
    }

    // ------------------------------------------------------------------
    // Kein Entpackziel ausserhalb des angezeigten Ordners
    // ------------------------------------------------------------------

    /// Ein Archiv namens `..zip` macht nicht den angezeigten Ordner zum Ziel.
    ///
    /// **Der kritische Befund der ersten Durchsicht dieser Runde**, in der
    /// Gestalt, in der er ausloest: [`namen_teilen`] gibt aus `..zip` den Stamm
    /// `.` heraus, und `<angezeigter Ordner>/.` **ist** der angezeigte Ordner.
    /// Wer im Konfliktblatt „Ueberschreiben" waehlt, gibt ihn an den
    /// Papierkorb. Die Probe faehrt den vollen Weg ueber [`entpackziel`] und
    /// nicht [`ordnername_zum_archiv`] allein, damit sie das prueft, was der
    /// Ausfuehrungszweig spaeter bekommt.
    #[test]
    fn ein_archiv_aus_zwei_punkten_zielt_nicht_auf_den_angezeigten_ordner() {
        let ordner = ordner();
        let modell = modell_mit(&["..zip"]);
        let archiv = ordner.join("..zip");

        assert_eq!(
            entpackziel(&modell, std::slice::from_ref(&archiv), ordner),
            Entpackbefund::Archive {
                paare: vec![(archiv, ordner.join(ERSATZSTAMM))],
                ausgelassen: 0,
            }
        );
    }

    /// Ein Archiv namens `...zip` macht nicht den Elternordner zum Ziel.
    ///
    /// Die zweite Gestalt desselben Befundes, und die schwerere: der Stamm ist
    /// `..`, und `symlink_metadata` loest ihn zum **Elternordner** auf — also zu
    /// einem Ordner, den der Nutzer beim Klicken nicht einmal vor sich hatte.
    /// Sie geht ueber die Ersatzregel statt ueber die betroffenen Eintraege,
    /// damit beide Zweige von [`entpackziel`] durch [`paar`] belegt sind.
    #[test]
    fn ein_archiv_aus_drei_punkten_zielt_nicht_auf_den_elternordner() {
        let ordner = ordner();
        let modell = modell_mit(&["...zip"]);

        assert_eq!(
            entpackziel(&modell, &[], ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("...zip"), ordner.join(ERSATZSTAMM))],
                ausgelassen: 0,
            }
        );
    }

    /// Kein Entpackziel trifft den angezeigten Ordner oder dessen Elternordner.
    ///
    /// **Die Probe prueft die Gestalt und keine Liste erwarteter Namen.** Eine
    /// Liste haette gesagt, was aus den zehn Namen wird; hier steht, was aus
    /// **keinem** Namen werden darf, und das ist die Zusage: der Zielordner
    /// liegt unmittelbar im angezeigten Ordner und sein letzter Bestandteil ist
    /// ein `Component::Normal`.
    ///
    /// **Beide Bedingungen sind noetig, und jede faengt eine der zwei
    /// Gestalten.** `<ordner>/.` traegt `Projekte` als letzten Bestandteil —
    /// `Path` streicht den Punkt weg —, faellt aber aus dem angezeigten Ordner
    /// heraus, weil sein Elternteil `/tmp` ist. `<ordner>/..` behaelt umgekehrt
    /// den angezeigten Ordner als Elternteil und faellt allein ueber den
    /// `ParentDir` auf. Mit nur einer der beiden Zeilen bliebe die Probe bei
    /// einem der zwei Defekte gruen.
    ///
    /// Gefahren wird ueber [`paar`], die eine Stelle, an der das Paar entsteht,
    /// und nicht ueber [`entpackziel`]: so stehen auch die Namen in der Liste,
    /// die [`ist_zipname`] gar nicht als Archiv durchliesse.
    #[test]
    fn kein_entpackziel_verlaesst_den_angezeigten_ordner() {
        let ordner = ordner();
        for name in [
            "..zip", "...zip", "..ZIP", "....zip", " .zip", "  .zip", ".zip", "a.zip", "..", ".",
        ] {
            let (_, ziel) = paar(ordner.join(name), ordner);

            assert_eq!(
                ziel.parent(),
                Some(ordner),
                "das Ziel zu \"{name}\" liegt nicht im angezeigten Ordner"
            );
            let letzter = ziel.components().next_back();
            assert!(
                matches!(letzter, Some(Component::Normal(_))),
                "das Ziel zu \"{name}\" endet auf {letzter:?} und nicht auf einem Namen"
            );
            assert_ne!(
                ziel.as_path(),
                ordner,
                "das Ziel zu \"{name}\" ist der angezeigte Ordner"
            );
            assert_ne!(
                Some(ziel.as_path()),
                ordner.parent(),
                "das Ziel zu \"{name}\" ist der Elternordner"
            );
        }
    }

    // ------------------------------------------------------------------
    // Worauf Unzip wirkt
    // ------------------------------------------------------------------

    /// Jedes betroffene Archiv wird entpackt, und jedes in seinen eigenen
    /// Ordner.
    ///
    /// **Die dritte Nutzerentscheidung in einer Probe.** Unter der empfohlenen
    /// ersten Moeglichkeit haette dieselbe Lage eine Meldung ergeben statt
    /// dreier Zielordner.
    #[test]
    fn drei_betroffene_archive_ergeben_drei_zielordner() {
        let ordner = ordner();
        let modell = modell_mit(&["eins.zip", "zwei.zip", "drei.zip"]);
        let betroffen = vec![
            ordner.join("eins.zip"),
            ordner.join("zwei.zip"),
            ordner.join("drei.zip"),
        ];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![
                    (ordner.join("eins.zip"), ordner.join("eins")),
                    (ordner.join("zwei.zip"), ordner.join("zwei")),
                    (ordner.join("drei.zip"), ordner.join("drei")),
                ],
                ausgelassen: 0,
            }
        );
    }

    /// Was unter den betroffenen Eintraegen kein Archiv ist, faellt weg.
    ///
    /// Die Ersatzregel greift dabei **nicht**: es steht ein Archiv unter den
    /// betroffenen Eintraegen, und damit ist die Frage beantwortet, auch wenn
    /// daneben zwei weitere im Ordner liegen.
    #[test]
    fn was_kein_archiv_ist_faellt_aus_den_betroffenen_heraus() {
        let ordner = ordner();
        let modell = modell_mit(&["eins.zip", "zwei.zip", "bericht.txt"]);
        let betroffen = vec![ordner.join("bericht.txt"), ordner.join("eins.zip")];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("eins.zip"), ordner.join("eins"))],
                ausgelassen: 0,
            }
        );
    }

    /// Die Ersatzregel: das eine Archiv des angezeigten Ordners.
    ///
    /// Die Lage der Directive, naemlich nichts markiert und nichts
    /// ausgewaehlt.
    #[test]
    fn ohne_betroffenes_archiv_gilt_das_eine_des_ordners() {
        let ordner = ordner();
        let modell = modell_mit(&["bericht.txt", "sicherung.zip"]);

        assert_eq!(
            entpackziel(&modell, &[], ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("sicherung.zip"), ordner.join("sicherung"))],
                ausgelassen: 0,
            }
        );
    }

    /// Kein Archiv weit und breit.
    #[test]
    fn ohne_jedes_archiv_kommt_keines_zurueck() {
        let ordner = ordner();
        let modell = modell_mit(&["bericht.txt", "liesmich"]);

        assert_eq!(
            entpackziel(&modell, &[ordner.join("bericht.txt")], ordner),
            Entpackbefund::Keines
        );
    }

    /// Zwei Archive im Ordner und keines betroffen: der Befund `Mehrere`.
    ///
    /// Der Fall, den der Plan als Lage aus dem Datensatz ausdruecklich nennt.
    #[test]
    fn zwei_archive_im_ordner_und_keine_auswahl_ergeben_mehrere() {
        let ordner = ordner();
        let modell = modell_mit(&["eins.zip", "zwei.zip"]);

        assert_eq!(entpackziel(&modell, &[], ordner), Entpackbefund::Mehrere);
    }

    /// Der Filtertext entscheidet mit, weil die Suche ueber die sichtbaren
    /// Zeilen laeuft.
    ///
    /// **Dieselbe Lage, zwei Befunde.** Ungefiltert traegt der Ordner zwei
    /// Archive und Unzip meldet es; mit dem Filtertext `eins` steht genau eines
    /// vor dem Nutzer, und genau dieses meint der Befehl. Ohne die Rechnung
    /// ueber [`Ordnermodell::zeilen`] saehe Unzip beide und wiese den Nutzer
    /// auf etwas hin, das er gar nicht vor sich hat.
    #[test]
    fn der_filtertext_engt_die_ersatzregel_ein() {
        let ordner = ordner();
        let mut modell = modell_mit(&["eins.zip", "zwei.zip"]);
        assert_eq!(entpackziel(&modell, &[], ordner), Entpackbefund::Mehrere);

        modell.filtertext_setzen("eins");
        assert_eq!(
            entpackziel(&modell, &[], ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("eins.zip"), ordner.join("eins"))],
                ausgelassen: 0,
            }
        );
    }

    // ------------------------------------------------------------------
    // Das Ziel eines Laufs liegt nie auf einer seiner Quellen
    // ------------------------------------------------------------------

    /// Der zweite Zip-Lauf ueber denselben Ordner packt sein eigenes Archiv
    /// nicht mit.
    ///
    /// **Die Lage stellt der erste Lauf selbst her:** aus `{a.txt, b.txt}` in
    /// `Projekte` entsteht `Projekte/Projekte.zip`. Beim naechsten Mal steht es
    /// in der Markierung, [`archivname`] rechnet denselben Pfad, und ohne den
    /// Schnitt in [`packziel`] raeumte „Ueberschreiben" im Konfliktblatt eine
    /// **Quelle des Laufs** in den Papierkorb (Nutzerantwort vom 260825 auf
    /// `issues/260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-*`).
    ///
    /// Geprueft wird beides in einem Zug: das Ziel bleibt der gerechnete Name,
    /// und keine Quelle traegt ihn.
    #[test]
    fn das_archiv_des_vorigen_laufs_faellt_aus_den_quellen() {
        let ordner = ordner();
        let betroffen = vec![
            ordner.join("a.txt"),
            ordner.join("Projekte.zip"),
            ordner.join("b.txt"),
        ];

        let (quellen, ziel) = packziel(&betroffen, ordner);

        assert_eq!(ziel, ordner.join("Projekte.zip"));
        assert_eq!(
            quellen,
            vec![ordner.join("a.txt"), ordner.join("b.txt")],
            "das Archiv des vorigen Laufs steht als Quelle desselben Laufs da"
        );
        assert!(
            !quellen.contains(&ziel),
            "das Ziel des Laufs ist eine seiner Quellen"
        );
    }

    /// Ein einzelner Eintrag bleibt Quelle, auch wenn er ein Archiv ist.
    ///
    /// **Der Schnitt schneidet nicht mehr, als er soll.** Die angehaengte
    /// Endung macht aus `sicherung.zip` das Archiv `sicherung.zip.zip`, und
    /// damit ist der Eintrag nicht sein eigenes Ziel. Ein Schnitt am Namen
    /// statt am Pfad haette hier eine leere Quellenliste hinterlassen.
    ///
    /// **Sie prueft den Packschneider und nur ihn.** [`entpackziel`] kommt in
    /// ihr nicht vor; der haeufigste Unzip-Fall — ein einzelnes Archiv, ein
    /// Zielordner — steht daneben in
    /// [`ein_einzelnes_archiv_behaelt_seinen_zielordner`]
    /// (`issues/260825-1249_*_die-probe-gegen-den-zu-weiten-schnitt-prueft-nur-den-packschneider-*`).
    #[test]
    fn ein_einzelnes_archiv_bleibt_seine_eigene_quelle() {
        let ordner = ordner();
        let betroffen = vec![ordner.join("sicherung.zip")];

        let (quellen, ziel) = packziel(&betroffen, ordner);

        assert_eq!(ziel, ordner.join("sicherung.zip.zip"));
        assert_eq!(quellen, betroffen);
    }

    /// Ein Archiv, dessen Zielordner eine andere Quelle desselben Laufs ist,
    /// faellt heraus.
    ///
    /// **Dieselbe Antwort in der Entpack-Gestalt, und die Runde stellt sie
    /// selbst her:** die anhaengende Endungsregel legt `a.zip.zip` neben
    /// `a.zip`. Werden beide markiert, rechnet [`paar`] fuer das zweite den
    /// Zielordner `<ordner>/a.zip` — den Pfad der ersten Quelle. Ohne den
    /// Schnitt ginge sie ueber das Konfliktblatt in den Papierkorb, waehrend
    /// derselbe Lauf sie noch entpacken wollte.
    ///
    /// Das laengste Archiv bleibt immer stehen: sein Zielname ist der
    /// gekuerzte, und keiner der markierten traegt ihn.
    #[test]
    fn ein_archiv_das_zielordner_eines_anderen_ist_faellt_aus_den_quellen() {
        let ordner = ordner();
        let modell = modell_mit(&["a.zip", "a.zip.zip"]);
        let betroffen = vec![ordner.join("a.zip"), ordner.join("a.zip.zip")];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("a.zip.zip"), ordner.join("a.zip"))],
                ausgelassen: 1,
            },
            "a.zip wird entpackt und ist zugleich Zielordner von a.zip.zip"
        );
    }

    /// Ein einzelnes Archiv behaelt seinen Zielordner.
    ///
    /// **Die Gegenprobe zum zu weiten Schnitt auf der Entpackseite**, und die
    /// haeufigste Lage von Unzip ueberhaupt: ein Archiv markiert, ein Ordner
    /// daneben. Ihr Zwilling
    /// [`ein_einzelnes_archiv_bleibt_seine_eigene_quelle`] prueft dieselbe
    /// Frage fuer den Packschneider und ruft [`entpackziel`] nicht; bis zum
    /// 260825 hielt diesen Fall allein aelterer Bestand, der vor dem Schnitt
    /// geschrieben wurde und ihn im Namen nicht nennt
    /// (`issues/260825-1249_*_die-probe-gegen-den-zu-weiten-schnitt-prueft-nur-den-packschneider-*`).
    ///
    /// Ein Archiv ist nie sein eigenes Ziel: sein Zielname ist der um
    /// [`ENDUNG`] gekuerzte Archivname und damit ein anderer.
    ///
    /// **Das zweite Archiv im Modell ist die eigentliche Pruefung.** Ohne es
    /// bliebe die Probe auch bei einem zu weiten Schnitt gruen: die Markierung
    /// fiele leer aus, und die Ersatzregel lieferte dasselbe eine Paar wieder
    /// zurueck. Mit ihm antwortet die Ersatzregel [`Entpackbefund::Mehrere`],
    /// und der zu weite Schnitt faellt auf.
    #[test]
    fn ein_einzelnes_archiv_behaelt_seinen_zielordner() {
        let ordner = ordner();
        let modell = modell_mit(&["sicherung.zip", "anderes.zip"]);
        let betroffen = vec![ordner.join("sicherung.zip")];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("sicherung.zip"), ordner.join("sicherung"))],
                ausgelassen: 0,
            },
            "der Entpackschnitt hat das einzige Archiv des Laufs genommen"
        );
    }

    /// Die Kette aus drei Archiven ergibt zwei Laeufe und nicht einen.
    ///
    /// **Der Schnitt ist ein Festpunkt.** `a.zip.zip` faellt, weil
    /// `a.zip.zip.zip` dorthin entpackt; `a.zip` bleibt, denn sein einziger
    /// Beansprucher ist eben gefallen. Bis zum 260825 rechnete
    /// [`ohne_die_eigenen_ziele`] die Zielliste einmal ueber alle Paare und
    /// filterte danach, und `a.zip` fiel fuer einen Anspruch, den niemand mehr
    /// erhob
    /// (`issues/260825-1249_*_der-entpackschnitt-ist-kein-festpunkt-*`).
    ///
    /// Die Kette entsteht in diesem Baum von selbst: jeder Zip-Lauf ueber ein
    /// `a.zip` legt `a.zip.zip` daneben.
    #[test]
    fn aus_einer_kette_von_drei_archiven_bleiben_zwei() {
        let ordner = ordner();
        let modell = modell_mit(&["a.zip", "a.zip.zip", "a.zip.zip.zip"]);
        let betroffen = vec![
            ordner.join("a.zip"),
            ordner.join("a.zip.zip"),
            ordner.join("a.zip.zip.zip"),
        ];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![
                    (ordner.join("a.zip"), ordner.join("a")),
                    (ordner.join("a.zip.zip.zip"), ordner.join("a.zip.zip")),
                ],
                ausgelassen: 1,
            },
            "a.zip faellt fuer einen Anspruch, den a.zip.zip nicht mehr erhebt"
        );
    }

    /// Der Schnitt trifft auch, was in abweichender Schreibung dasteht.
    ///
    /// **Die Nutzerwahl vom 260825**
    /// (`issues/260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-*`,
    /// Moeglichkeit 1) in ihrer Packgestalt: im Ordner `Projekte` liegt ein von
    /// fremder Hand angelegtes `PROJEKTE.ZIP`, und [`archivname`] rechnet
    /// `Projekte.zip`, weil [`ENDUNG`] kleingeschrieben dasteht. Auf dem
    /// Bauziel ist das derselbe Eintrag; ein buchstabengetreuer Vergleich hielt
    /// ihn fuer einen anderen und liess ihn als Quelle stehen, worauf
    /// „Ueberschreiben" im Konfliktblatt eine Quelle des Laufs in den
    /// Papierkorb raeumte.
    #[test]
    fn das_archiv_des_vorigen_laufs_faellt_auch_in_abweichender_schreibung() {
        let ordner = ordner();
        let betroffen = vec![
            ordner.join("a.txt"),
            ordner.join("PROJEKTE.ZIP"),
            ordner.join("b.txt"),
        ];

        let (quellen, ziel) = packziel(&betroffen, ordner);

        assert_eq!(ziel, ordner.join("Projekte.zip"));
        assert_eq!(
            quellen,
            vec![ordner.join("a.txt"), ordner.join("b.txt")],
            "PROJEKTE.ZIP und das gerechnete Projekte.zip sind auf der Platte \
             derselbe Eintrag"
        );
    }

    /// Dieselbe Faltung in der Entpackgestalt.
    ///
    /// Neben `a.zip` steht `A.ZIP.zip`. [`paar`] rechnet fuer das zweite den
    /// Zielordner `<ordner>/A.ZIP`; auf der Platte ist das der Eintrag `a.zip`,
    /// den derselbe Lauf noch entpacken will.
    #[test]
    fn der_entpackschnitt_trifft_auch_in_abweichender_schreibung() {
        let ordner = ordner();
        let modell = modell_mit(&["a.zip", "A.ZIP.zip"]);
        let betroffen = vec![ordner.join("a.zip"), ordner.join("A.ZIP.zip")];

        assert_eq!(
            entpackziel(&modell, &betroffen, ordner),
            Entpackbefund::Archive {
                paare: vec![(ordner.join("A.ZIP.zip"), ordner.join("A.ZIP"))],
                ausgelassen: 1,
            },
            "a.zip ist der Zielordner von A.ZIP.zip, in der Schreibung der Platte"
        );
    }

    /// Was verschieden heisst, faellt nicht: die Faltung gilt dem letzten
    /// Bestandteil und nicht dem ganzen Pfad.
    ///
    /// **Die Grenze der Nutzerwahl, ausgeschrieben.** Ein Eintrag, der bis auf
    /// die Gross- und Kleinschreibung anders heisst, bleibt Quelle; gefaltet
    /// wird allein die Schreibung und nicht die Aehnlichkeit.
    #[test]
    fn ein_aehnlich_benanntes_archiv_bleibt_quelle() {
        let ordner = ordner();
        let betroffen = vec![ordner.join("a.txt"), ordner.join("Projekte 2.zip")];

        let (quellen, ziel) = packziel(&betroffen, ordner);

        assert_eq!(ziel, ordner.join("Projekte.zip"));
        assert_eq!(quellen, betroffen);
    }
}
