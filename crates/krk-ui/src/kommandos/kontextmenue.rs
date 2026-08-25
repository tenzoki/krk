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
//!  angezeigter Ordner ───┼──> archivname()   ──> der Pfad des Archivs
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
//!    selben Menue stehen, saehe der Nutzer den Verlust unmittelbar.
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
//! # Die Ausnahme mit Ablaufdatum
//!
//! Bis zum Menuebau in `crate::appkit::tabelle` und der Ausfuehrung beim
//! Anwendungsdelegierten ruft niemand ausser den Proben in dieses Modul
//! hinein. **`krk-ui` hat kein Bibliotheksziel**, also ist `pub` hier keine
//! Wurzel: der Uebersetzer meldet jedes Stueck als unbenutzt, und
//! `-D warnings` haelt den Bau an.
//!
//! Deshalb steht darunter `expect` und nicht `allow`, und es steht am Modul
//! und nicht elfmal am einzelnen Stueck. Die Erwartung erlischt in dem
//! Augenblick, in dem das letzte Stueck einen Aufrufer bekommt: dann meldet der
//! Uebersetzer die unerfuellte Erwartung, und der Bau haelt an, bis die Zeilen
//! weg sind. Eine Ausnahme mit Ablaufdatum statt einer, die stehen bleibt und
//! niemandem mehr sagt, warum — dieselbe Form, die
//! [`super::rueckschritt`] in seinem Kopf beschreibt.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "die Schritte 6 und 7 dieser Runde setzen die Aufrufer im \
                  Menuebau und beim Anwendungsdelegierten; bis dahin rufen \
                  allein die Proben"
    )
)]

use std::path::{Path, PathBuf};

use krk_core::operation::umbenennen::namen_teilen;
use krk_core::verzeichnis::Ordnermodell;

/// Die Endung, an der ein Archiv erkannt und mit der eines benannt wird.
///
/// **Eine Zeichenfolge fuer beide Richtungen.** [`ist_zipname`] vergleicht
/// gegen sie, [`archivname`] haengt sie an, und [`ordnername_zum_archiv`]
/// nimmt sie wieder ab. Zwei Schreibweisen nebeneinander waeren genau die Lage,
/// in der der Rundweg spaeter unbemerkt aufhoert zu schliessen.
const ENDUNG: &str = ".zip";

/// Der Stamm, wenn weder ein Eintrag noch der angezeigte Ordner einen Namen
/// hergibt.
///
/// Der Fall ist das Wurzelverzeichnis: `Path::new("/").file_name()` liefert
/// `None`, und ein Archiv namens `.zip` waere keines (siehe [`ist_zipname`]).
/// Ein Ersatzname ist hier die richtige Antwort und keine Notluege: der Nutzer
/// sieht ihn im Konfliktblatt und in der Statuszeile, bevor irgendetwas
/// entsteht.
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
    Archive(Vec<(PathBuf, PathBuf)>),
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
/// **Ein leeres `betroffen` erreicht diese Funktion im Betrieb nicht**: der
/// Aufrufer faengt es vorher mit [`super::operationen::nichts_zu_packen`] ab.
/// Beantwortet wird der Fall trotzdem, denn eine Rechnung mit einer Luecke
/// waere an der Stelle unbrauchbar, an der sie geprueft wird.
#[must_use]
pub fn archivname(betroffen: &[PathBuf], ordner: &Path) -> PathBuf {
    let stamm = match betroffen {
        [einziger] => einziger.file_name(),
        _ => None,
    }
    .or_else(|| ordner.file_name())
    .map_or_else(
        || ERSATZSTAMM.to_owned(),
        |name| name.to_string_lossy().into_owned(),
    );
    ordner.join(format!("{stamm}{ENDUNG}"))
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
#[must_use]
pub fn ordnername_zum_archiv(archiv: &Path) -> String {
    let Some(name) = archiv.file_name() else {
        return ERSATZSTAMM.to_owned();
    };
    let name = name.to_string_lossy();
    if !ist_zipname(&name) {
        return name.into_owned();
    }
    let (stamm, _) = namen_teilen(&name);
    stamm.to_owned()
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
#[must_use]
pub fn entpackziel(modell: &Ordnermodell, betroffen: &[PathBuf], ordner: &Path) -> Entpackbefund {
    let betroffene_archive: Vec<(PathBuf, PathBuf)> = betroffen
        .iter()
        .filter(|pfad| ist_archivpfad(pfad))
        .map(|pfad| paar(pfad.clone(), ordner))
        .collect();
    if !betroffene_archive.is_empty() {
        return Entpackbefund::Archive(betroffene_archive);
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
    Entpackbefund::Archive(vec![paar(erstes, ordner)])
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

#[cfg(test)]
mod tests {
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
            Entpackbefund::Archive(vec![
                (ordner.join("eins.zip"), ordner.join("eins")),
                (ordner.join("zwei.zip"), ordner.join("zwei")),
                (ordner.join("drei.zip"), ordner.join("drei")),
            ])
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
            Entpackbefund::Archive(vec![(ordner.join("eins.zip"), ordner.join("eins"))])
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
            Entpackbefund::Archive(vec![(
                ordner.join("sicherung.zip"),
                ordner.join("sicherung")
            )])
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
            Entpackbefund::Archive(vec![(ordner.join("eins.zip"), ordner.join("eins"))])
        );
    }
}
