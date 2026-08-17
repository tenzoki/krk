//! Abnahme der Pruefung auf einen Git-Arbeitsbaum (Schritt 8 der Loeschrunde).
//!
//! Alle Proben laufen ohne Fenster und ohne AppKit. Ihre Pruefordner kommen aus
//! `tests/gemeinsam/`, der einen Fassung fuer alle Abnahmeproben des Kerns; sie
//! tragen Prozesskennung und Laufnummer und raeumen sich in `Drop` selbst ab.
//!
//! # Warum diese Proben hier stehen und nicht neben dem Modul
//!
//! Weil sie echte Ordner mit echten `.git`-Eintraegen brauchen und `Pruefordner`
//! unter `tests/gemeinsam/` liegt: eine Probe in `#[cfg(test)]` neben dem Modul
//! erreicht ihn nicht. Ein eigenes Ziel und nicht ein Anhang an
//! `tests/verzeichnis.rs`, weil die Frage eine andere ist — dort steht die
//! Abnahme des Verzeichnislesers, des Ordnermodells und des Durchlaufs.
//!
//! # Wovon diese Datei die Grenze ist
//!
//! **Sie prueft das Verhalten und nicht das Zugriffsmuster.** Die Zusage
//! „Abbruch beim ersten Treffer" ist mit echten Ordnern nicht messbar, und der
//! Grund liegt in `Loeschzielbefund::oder`: `Ja` ist dort aufsaugend, und ein
//! Gang, der nach dem ersten Treffer weiterlaeuft, liefert dasselbe `Ja`. Kein
//! Baum, den man hier aufbaut, unterscheidet die beiden Faelle. Gemessen wird
//! sie deshalb neben dem Modul, mit eingesetzter Pruefung und einer Mitschrift
//! der besuchten Ebenen.
//!
//! **Eine Abbruchzusage steht trotzdem hier**, naemlich die einschliessende
//! Grenze am Benutzerverzeichnis: die ist am Ergebnis abzulesen, denn ein `.git`
//! oberhalb der Grenze darf nicht gefunden werden.
//!
//! Jede Probe hier setzt das Benutzerverzeichnis auf den Pruefordner. Ohne diese
//! Grenze liefe der Aufwaertsgang bis zur Wurzel und haenge damit an dem, was
//! zwischen `/var/folders` und `/` gerade steht.

use std::path::PathBuf;

use krk_core::verzeichnis::Loeschzielbefund;
use krk_core::verzeichnis::arbeitsbaum::{
    beruehrt_einen_arbeitsbaum, liegt_in_arbeitsbaum, traegt_arbeitsbaum,
};

mod gemeinsam;
use gemeinsam::Pruefordner;

// ---------------------------------------------------------------------------
// Der Aufwaertsgang: der Ordner selbst, die Ebenen darueber, keiner im Ast
// ---------------------------------------------------------------------------

/// Ein `.git` unmittelbar im angezeigten Ordner ist ein Treffer.
#[test]
fn der_arbeitsbaum_am_ordner_selbst_wird_gefunden() {
    let ordner = Pruefordner::neu("arbeitsbaum-selbst");
    let projekt = ordner.ordner("projekt");
    std::fs::create_dir(projekt.join(".git")).expect(".git laesst sich nicht anlegen");

    assert_eq!(traegt_arbeitsbaum(&projekt), Loeschzielbefund::Ja);
    assert_eq!(
        liegt_in_arbeitsbaum(&projekt, Some(ordner.pfad())),
        Loeschzielbefund::Ja
    );
}

/// Ein `.git` zwei Ebenen ueber dem angezeigten Ordner ist ebenfalls ein
/// Treffer.
///
/// Das ist der Fall des Vorfalls vom 260817-0344: geraeumt wurde
/// `…/krk/fusion-workbench/shared`, und der Arbeitsbaum liegt zwei Ebenen
/// darueber. Die enge Form der Pruefung, die der Nutzer am Spec-Gate umgedreht
/// hat, haette hier geschwiegen — [`traegt_arbeitsbaum`] am Ordner selbst sagt
/// `Nein`, und die Probe schreibt beide Antworten aus, damit der Unterschied
/// dasteht.
#[test]
fn der_arbeitsbaum_zwei_ebenen_darueber_wird_gefunden() {
    let ordner = Pruefordner::neu("arbeitsbaum-darueber");
    let projekt = ordner.ordner("projekt");
    std::fs::create_dir(projekt.join(".git")).expect(".git laesst sich nicht anlegen");
    let tief = projekt.join("werkbank/geteilt");
    std::fs::create_dir_all(&tief).expect("Unterbau laesst sich nicht anlegen");

    assert_eq!(
        traegt_arbeitsbaum(&tief),
        Loeschzielbefund::Nein,
        "der Ordner selbst traegt kein .git; die enge Form haette hier geschwiegen"
    );
    assert_eq!(
        liegt_in_arbeitsbaum(&tief, Some(ordner.pfad())),
        Loeschzielbefund::Ja,
        "der Aufwaertsgang findet den Arbeitsbaum zwei Ebenen darueber nicht"
    );
}

/// Kein `.git` im ganzen Ast: die ruhige Antwort.
#[test]
fn ohne_arbeitsbaum_im_ganzen_ast_bleibt_es_ruhig() {
    let ordner = Pruefordner::neu("arbeitsbaum-keiner");
    let tief = ordner.pfad().join("eins/zwei/drei");
    std::fs::create_dir_all(&tief).expect("Unterbau laesst sich nicht anlegen");

    let befund = liegt_in_arbeitsbaum(&tief, Some(ordner.pfad()));
    assert_eq!(befund, Loeschzielbefund::Nein);
    assert!(
        !befund.ist_warnwuerdig(),
        "ein Ast ohne Arbeitsbaum macht die Rueckfrage laut"
    );
}

/// Ein `.git` **oberhalb** der Grenze wird nicht gefunden.
///
/// Die einzige Abbruchzusage, die an echten Ordnern messbar ist. Der Baum ist so
/// gebaut, dass ein Gang ohne Grenze den Arbeitsbaum finden **wuerde**: er liegt
/// eine Ebene ueber dem Benutzerverzeichnis dieser Probe. Wer die Grenze
/// entfernt oder sie ausschliessend macht, bekommt hier `Ja`.
#[test]
fn ein_arbeitsbaum_oberhalb_der_grenze_wird_nicht_gefunden() {
    let ordner = Pruefordner::neu("arbeitsbaum-grenze");
    std::fs::create_dir(ordner.unter(".git")).expect(".git laesst sich nicht anlegen");
    let zuhause = ordner.ordner("zuhause");
    let tief = zuhause.join("projekt/quelle");
    std::fs::create_dir_all(&tief).expect("Unterbau laesst sich nicht anlegen");

    assert_eq!(
        liegt_in_arbeitsbaum(&tief, Some(&zuhause)),
        Loeschzielbefund::Nein,
        "der Gang hat die Grenze ueberschritten"
    );
    assert_eq!(
        liegt_in_arbeitsbaum(&tief, None),
        Loeschzielbefund::Ja,
        "ohne Grenze muesste derselbe Baum den Arbeitsbaum darueber finden; \
         tut er das nicht, prueft die Probe darueber nichts"
    );
}

/// Ein `.git` **am** Benutzerverzeichnis wird noch gefunden.
///
/// Die andere Haelfte derselben Zusage: die Grenze ist einschliessend. Der Preis
/// steht im Modulkopf des Moduls — traegt das Benutzerverzeichnis selbst einen
/// Arbeitsbaum, ist jede Loeschung darunter aus diesem Grund laut.
#[test]
fn ein_arbeitsbaum_am_benutzerverzeichnis_wird_gefunden() {
    let ordner = Pruefordner::neu("arbeitsbaum-am-zuhause");
    let zuhause = ordner.ordner("zuhause");
    std::fs::create_dir(zuhause.join(".git")).expect(".git laesst sich nicht anlegen");
    let tief = zuhause.join("unterlagen/notizen");
    std::fs::create_dir_all(&tief).expect("Unterbau laesst sich nicht anlegen");

    assert_eq!(
        liegt_in_arbeitsbaum(&tief, Some(&zuhause)),
        Loeschzielbefund::Ja
    );
}

/// `.git` als **Datei** ist derselbe Treffer wie `.git` als Verzeichnis.
///
/// So legt Git einen verknuepften Arbeitsbaum an. Gefragt ist die Anwesenheit
/// des Eintrags und nicht seine Art; die Probe haelt fest, dass daraus kein
/// Sonderfall geworden ist.
#[test]
fn ein_git_als_datei_ist_derselbe_treffer() {
    let ordner = Pruefordner::neu("arbeitsbaum-als-datei");
    let projekt = ordner.ordner("projekt");
    std::fs::write(projekt.join(".git"), b"gitdir: /woanders\n")
        .expect(".git laesst sich nicht schreiben");

    assert_eq!(traegt_arbeitsbaum(&projekt), Loeschzielbefund::Ja);
}

// ---------------------------------------------------------------------------
// Die Auswahl
// ---------------------------------------------------------------------------

/// Ein ausgewaehlter Unterordner, der selbst die Wurzel eines Arbeitsbaums ist.
///
/// Der angezeigte Ordner liegt in keinem Arbeitsbaum; ohne die Schleife ueber
/// die Auswahl bliebe die Rueckfrage hier ruhig, obwohl ein ganzes Projekt
/// wegkaeme.
#[test]
fn ein_ausgewaehlter_unterordner_als_wurzel_wird_gefunden() {
    let ordner = Pruefordner::neu("arbeitsbaum-auswahl");
    let ablage = ordner.ordner("ablage");
    let projekt = ablage.join("projekt");
    std::fs::create_dir(&projekt).expect("Projekt laesst sich nicht anlegen");
    std::fs::create_dir(projekt.join(".git")).expect(".git laesst sich nicht anlegen");
    let harmlos = ablage.join("harmlos");
    std::fs::create_dir(&harmlos).expect("Ordner laesst sich nicht anlegen");

    assert_eq!(
        liegt_in_arbeitsbaum(&ablage, Some(ordner.pfad())),
        Loeschzielbefund::Nein,
        "der angezeigte Ordner liegt in keinem Arbeitsbaum; sonst prueft die Probe nichts"
    );
    assert_eq!(
        beruehrt_einen_arbeitsbaum(
            &ablage,
            Some(ordner.pfad()),
            &[harmlos.clone(), projekt.clone()],
        ),
        Loeschzielbefund::Ja,
        "die Schleife ueber die Auswahl hat den Arbeitsbaum nicht gefunden"
    );
    assert_eq!(
        beruehrt_einen_arbeitsbaum(&ablage, Some(ordner.pfad()), &[harmlos]),
        Loeschzielbefund::Nein,
        "eine Auswahl ohne den Arbeitsbaum macht die Rueckfrage laut"
    );
}

/// Gewoehnliche Dateien und verschwundene Pfade in der Auswahl bleiben ruhig.
///
/// Der Zweig, der `ENOTDIR` von `ENOENT` trennt, ist nicht kosmetisch: ohne ihn
/// machte **jede** ausgewaehlte Datei die Rueckfrage unentschieden und damit
/// laut, und das waere in einem Dateimanager der Normalfall.
#[test]
fn dateien_und_verschwundene_pfade_in_der_auswahl_bleiben_ruhig() {
    let ordner = Pruefordner::neu("arbeitsbaum-dateien");
    let datei = ordner.datei("notiz.txt", b"x");
    let weg = ordner.unter("gibt-es-nicht");

    assert_eq!(
        traegt_arbeitsbaum(&datei),
        Loeschzielbefund::Nein,
        "eine Datei kann keinen Eintrag tragen und ist damit entschieden"
    );
    assert_eq!(traegt_arbeitsbaum(&weg), Loeschzielbefund::Nein);
    assert_eq!(
        beruehrt_einen_arbeitsbaum(ordner.pfad(), Some(ordner.pfad()), &[datei, weg]),
        Loeschzielbefund::Nein
    );
}

/// Eine leere Auswahl ist genau der Aufwaertsgang.
#[test]
fn eine_leere_auswahl_ist_der_aufwaertsgang() {
    let ordner = Pruefordner::neu("arbeitsbaum-leer");
    let projekt = ordner.ordner("projekt");
    std::fs::create_dir(projekt.join(".git")).expect(".git laesst sich nicht anlegen");

    assert_eq!(
        beruehrt_einen_arbeitsbaum(&projekt, Some(ordner.pfad()), &[]),
        Loeschzielbefund::Ja
    );
    assert_eq!(
        beruehrt_einen_arbeitsbaum(ordner.pfad(), Some(ordner.pfad()), &[]),
        Loeschzielbefund::Nein
    );
}

/// Ein ausgewaehlter Verweis auf einen Arbeitsbaum antwortet `Ja`.
///
/// **Das ist die benannte Folge und keine Zusage, die jemand haben wollte**:
/// wegkaeme nur der Verweis, gemeldet wird der Arbeitsbaum. Der Fehler geht in
/// die laute Richtung, und ihn zu schliessen kostete ein zweites `lstat(2)` je
/// ausgewaehltem Eintrag; die Abwaegung steht im Doc-Kommentar von
/// [`traegt_arbeitsbaum`]. Die Probe steht hier, damit die Folge gemessen ist
/// und nicht bloss behauptet — und damit sie rot wird, wenn jemand sie
/// stillschweigend aendert.
#[test]
fn ein_ausgewaehlter_verweis_auf_einen_arbeitsbaum_antwortet_ja() {
    let ordner = Pruefordner::neu("arbeitsbaum-verweis");
    let projekt = ordner.ordner("projekt");
    std::fs::create_dir(projekt.join(".git")).expect(".git laesst sich nicht anlegen");
    let ablage = ordner.ordner("ablage");
    let verweis = ablage.join("zeigt-aufs-projekt");
    std::os::unix::fs::symlink(&projekt, &verweis).expect("Verweis laesst sich nicht anlegen");

    assert_eq!(
        beruehrt_einen_arbeitsbaum(&ablage, Some(ordner.pfad()), &[verweis]),
        Loeschzielbefund::Ja
    );
}

// ---------------------------------------------------------------------------
// Der dritte Wert an echten Pfaden
// ---------------------------------------------------------------------------

/// Ein Zugriff, der weder „da" noch „nicht da" beantwortet, ist unentschieden.
///
/// Ausgeloest ueber einen Namensbestandteil von 300 Zeichen: das Dateisystem
/// nimmt hoechstens 255 und antwortet mit `ENAMETOOLONG`, also weder `ENOENT`
/// noch `ENOTDIR`.
///
/// **Der Weg ueber einen zu langen Namen und nicht ueber entzogene Rechte** ist
/// Absicht: eine Probe mit `chmod 0o000` bestuende unter einem Lauf als `root`
/// nicht, weil dort die Rechtepruefung entfaellt, und behauptete dann still das
/// Gegenteil. Der zu lange Name trifft jeden Lauf gleich.
///
/// Die Fortpflanzung gehoert zur Zusage: der Zweifel an der ersten Ebene
/// ueberlebt den Aufwaertsgang, auch wenn die Ebene darueber ein entschiedenes
/// `Nein` liefert, und er macht die Rueckfrage laut.
#[test]
fn ein_unlesbarer_zugriff_bleibt_unentschieden() {
    let ordner = Pruefordner::neu("arbeitsbaum-zu-lang");
    let zu_lang = ordner.unter(&"a".repeat(300));

    assert_eq!(
        traegt_arbeitsbaum(&zu_lang),
        Loeschzielbefund::Unentschieden,
        "ein Zugriff, der weder da noch nicht da beantwortet, ist entschieden worden"
    );

    let befund = liegt_in_arbeitsbaum(&zu_lang, Some(ordner.pfad()));
    assert_eq!(
        befund,
        Loeschzielbefund::Unentschieden,
        "der Zweifel der ersten Ebene hat den Aufwaertsgang nicht ueberlebt"
    );
    assert!(befund.ist_warnwuerdig());

    let auswahl: Vec<PathBuf> = vec![zu_lang];
    assert_eq!(
        beruehrt_einen_arbeitsbaum(ordner.pfad(), Some(ordner.pfad()), &auswahl),
        Loeschzielbefund::Unentschieden,
        "ein unlesbarer Eintrag in der Auswahl ist entschieden worden"
    );
}
