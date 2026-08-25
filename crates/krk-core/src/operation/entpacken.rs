//! Entpacken: jedes Archiv in **seinen eigenen** neuen Ordner.
//!
//! ```text
//! archiv_entpacken ──> sys::ohne_warten_oeffnen ──> ZipArchive::new
//!                  ──> zielordner_klaeren ──> steuerung.konflikt_loesen
//!                                         └─> "ueberschreiben" ──> Papierkorb
//!                  ──> create_dir_all
//!                  ──> je Eintrag: enclosed_name ──> None ──> auslassen
//!                                              ──> Ordner       ──> create_dir
//!                                              ──> Verknuepfung ──> symlink
//!                                              ──> Datei        ──> Stueck fuer Stueck
//!                  ──> die Ordnerrechte zuletzt
//! ```
//!
//! # Warum das Entpacken **in** der Quelle-fuer-Quelle-Schleife laeuft
//!
//! Es ist das Spiegelbild des Packens und nicht seine Umkehrung. Das Packen
//! zieht viele Quellen in **ein** Ziel und steht deshalb neben der Schleife
//! ([`super::zippen`]); das Entpacken gibt jeder Quelle ihr **eigenes** Ziel und
//! gehoert damit in dieselbe Bahn wie das Kopieren. Ein Vorgang traegt
//! moeglicherweise mehrere Archive: der Nutzer hat am 260824-2120 gewaehlt, dass
//! Unzip auf die betroffenen Eintraege wirkt und jedes Archiv darin entpackt
//! (`decisions/260825-0727_*_nimmt-unzip-die-betroffenen-eintraege-oder-allein-
//! die-ausgewaehlte-zeile.md`, Moeglichkeit 3).
//!
//! # Das Archiv wird geoeffnet, bevor der Zielordner geklaert wird
//!
//! Die Reihenfolge ist die Umkehrung der des Packens, und der Grund ist
//! derselbe: **es soll nichts verloren gehen, bevor feststeht, dass etwas
//! entsteht.** Beim Packen schnitte `File::create` die vorhandene Datei ab,
//! deshalb steht dort die Rueckfrage vorn. Hier raeumt die Antwort
//! "ueberschreiben" einen ganzen Ordnerbaum in den Papierkorb; waere danach das
//! Archiv keines, haette der Nutzer seinen Ordner fuer nichts hergegeben. Das
//! Oeffnen schreibt kein Byte, und die Zusage "die Rueckfrage kommt, bevor ein
//! Eintrag geschrieben wird" haelt unveraendert.
//!
//! # "Ueberschreiben" raeumt in den Papierkorb und loescht nicht
//!
//! Seit der Runde 12 gibt es genau einen Loeschweg, und der fuehrt in den
//! Papierkorb. Der vorhandene Zielordner geht deshalb ueber die hereingereichte
//! [`Papierkorb`]-Schnittstelle und **nicht** ueber
//! [`super::loeschen::baum_entfernen`], das der Zip-Lauf fuer seine einzelne
//! Zieldatei nimmt. Der Nutzer hat die Rueckfrage selbst gewaehlt und dabei die
//! Bindung mitgegeben (`decisions/260825-0711_*_was-tut-unzip-wenn-der-
//! zielordner-schon-dasteht.md`, Moeglichkeit 2).
//!
//! # Zwei Wege aus dem Zielordner heraus, und beide sind versperrt
//!
//! Ein Archiv ist eine fremde Datei, und seine Eintragsnamen sind kein
//! Versprechen. **Der erste Weg ist der Name selbst**: `../../etc/passwd` oder
//! `/etc/passwd`. Ihn versperrt [`ZipFile::enclosed_name`](zip::read::ZipFile::enclosed_name),
//! das fuer jeden Namen `None` liefert, der aus dem Zielordner herausfuehrte;
//! der Eintrag wird ausgelassen und in der Abschlussliste genannt.
//!
//! **Der zweite Weg geht ueber zwei Eintraege und kaeme daran vorbei**: der
//! erste legt eine Verknuepfung `hinaus -> /etc` an, deren Name im Zielordner
//! liegt und `enclosed_name` deshalb genuegt; der zweite heisst
//! `hinaus/passwd`, liegt ebenfalls im Zielordner und landete trotzdem in
//! `/etc`. Ihn versperrt [`kette_anlegen`]: jeder Ordner auf dem Weg zu einem
//! Eintrag muss ein **wirklicher** Ordner sein, keine Verknuepfung. Damit ist
//! jede Verknuepfung, die das Archiv mitbringt, eine Verknuepfung im Ergebnis
//! und kein Weg fuer den naechsten Eintrag; abgelegt wird sie unveraendert, so
//! wie [`super::kopieren`] und [`super::zippen`] eine Verknuepfung unveraendert
//! weitergeben.
//!
//! # Das Aenderungsdatum kommt nicht mit, und das ist ein offener Befund
//!
//! Eine entpackte Datei traegt die Uhrzeit des Entpackens und nicht den
//! Zeitstempel ihres Archiveintrags. Das ist die Gegenrichtung des Defekts
//! `issues/260825-0838_*_jeder-gepackte-eintrag-traegt-den-1-januar-1980-*`, und
//! **beide Enden gehoeren in denselben Zug**: solange der Packlauf jedem Eintrag
//! den 1. Januar 1980 gibt, machte ein Entpacken, das den Zeitstempel
//! uebernaehme, aus jeder Datei eine von 1980. Der Datensatz nennt die drei
//! Wege und diese Datei als betroffen; hier steht bewusst keine halbe Loesung.
//!
//! # Nach einem Abbruch bleibt stehen, was schon entpackt ist
//!
//! Wie bei einer abgebrochenen Kopie. Weggeraeumt wird allein die **halbe
//! Datei**, an der der Abbruch traf: sie sieht aus wie ein Ergebnis und ist ein
//! Rest. Der Zielordner samt allem, was fertig geworden ist, bleibt; er ist,
//! anders als ein halbes Archiv, benutzbar, und ihn wegzuraeumen waere ein
//! Loeschen ohne Auftrag.

use std::fs::{self, File, Permissions};
use std::io::{self, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};

use zip::ZipArchive;

use crate::verzeichnis::sys::ohne_warten_oeffnen;

use super::fortschritt::Steuerung;
use super::umbenennen::name_pruefen;
use super::{Ablauf, Konfliktantwort, Papierkorb, Quelle, STUECK, Zielentscheid, grund};

/// Entpackt ein Archiv in den genannten Ordner.
///
/// Der Ordner ist der **neue** Ordner selbst und nicht der Ordner, in dem er
/// entsteht; gerechnet hat ihn die Oberflaeche.
pub(crate) fn archiv_entpacken(
    quelle: &Quelle<'_>,
    ziel: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let datei = match ohne_warten_oeffnen(quelle.pfad) {
        Ok(datei) => datei,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    let mut archiv = match ZipArchive::new(BufReader::new(datei)) {
        Ok(archiv) => archiv,
        // Der Wortlaut kommt von der Kiste und nicht von hier. Sie sagt genauer
        // als eine eigene Formulierung, woran das Oeffnen gescheitert ist, und
        // eine erfundene Uebersetzung waere ungenauer als das Original; so
        // haelt es [`grund`] mit den Systemfehlern.
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, fehler.to_string());
            return Ablauf::Weiter;
        }
    };

    let zielordner = match zielordner_klaeren(quelle, ziel, papierkorb, steuerung) {
        Zielentscheid::Nach(ordner) => ordner,
        Zielentscheid::Ueberspringen => return Ablauf::Weiter,
        Zielentscheid::Abbrechen => return Ablauf::Abgebrochen,
    };
    if let Err(fehler) = fs::create_dir_all(&zielordner) {
        steuerung.ueberspringen(&zielordner, grund(&fehler));
        return Ablauf::Weiter;
    }

    let mut ordnerrechte = Vec::new();
    let ablauf = eintraege_entpacken(
        quelle.pfad,
        &mut archiv,
        &zielordner,
        &mut ordnerrechte,
        steuerung,
    );
    ordnerrechte_nachtragen(&ordnerrechte);
    ablauf
}

/// Klaert den Zielordner, bevor ein Eintrag geschrieben wird.
///
/// **Alle vier Antworten sind behandelt**, wie beim Packen: der Kern kennt vier,
/// und die Konfliktregel des Auftrags kann jede davon liefern, ohne dass ein
/// Blatt im Spiel waere.
///
/// **Ein vorhandener Ordner ist hier ein Konflikt**, anders als bei
/// [`super::ziel_klaeren`], wo ein Ordner auf einem gleichnamigen Ordner in den
/// vorhandenen hineinwandert. Der Nutzer hat das Verschmelzen ausdruecklich
/// nicht gewaehlt: er bekommt dieselbe Rueckfrage wie beim Zip, einmal je
/// Archiv, statt einer Kette von Rueckfragen ueber die Eintraege darin.
fn zielordner_klaeren(
    quelle: &Quelle<'_>,
    ziel: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Zielentscheid {
    if fs::symlink_metadata(ziel).is_err() {
        return Zielentscheid::Nach(ziel.to_path_buf());
    }

    match steuerung.konflikt_loesen(quelle.pfad, ziel) {
        Konfliktantwort::Ueberschreiben => match papierkorb.in_den_papierkorb(ziel) {
            Ok(_) => Zielentscheid::Nach(ziel.to_path_buf()),
            Err(fehler) => {
                steuerung.ueberspringen(
                    quelle.pfad,
                    format!(
                        "das Ziel liess sich nicht in den Papierkorb raeumen: {}",
                        grund(&fehler)
                    ),
                );
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Ueberspringen => {
            steuerung.ueberspringen(quelle.pfad, "am Ziel steht schon ein Eintrag");
            Zielentscheid::Ueberspringen
        }
        Konfliktantwort::UmbenennenIn(name) => match name_pruefen(&name) {
            Ok(()) => Zielentscheid::Nach(ziel.with_file_name(name)),
            Err(fehler) => {
                steuerung.ueberspringen(quelle.pfad, fehler.grund());
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Abbrechen => Zielentscheid::Abbrechen,
    }
}

/// Legt jeden Eintrag des Archivs unter dem Zielordner ab.
fn eintraege_entpacken(
    archivpfad: &Path,
    archiv: &mut ZipArchive<BufReader<File>>,
    zielordner: &Path,
    ordnerrechte: &mut Vec<(PathBuf, u32)>,
    steuerung: &mut Steuerung,
) -> Ablauf {
    for stelle in 0..archiv.len() {
        if steuerung.abgebrochen() {
            return Ablauf::Abgebrochen;
        }
        let mut eintrag = match archiv.by_index(stelle) {
            Ok(eintrag) => eintrag,
            Err(fehler) => {
                steuerung.ueberspringen(archivpfad, fehler.to_string());
                continue;
            }
        };

        // **Die eine Frage, die ein fremder Name beantworten muss.** `None`
        // heisst: der Name fuehrte aus dem Zielordner heraus, sei es ueber
        // `..`, sei es ueber einen fuehrenden Schraegstrich.
        let Some(innen) = eintrag.enclosed_name() else {
            let name = eintrag.name().to_owned();
            drop(eintrag);
            steuerung.ueberspringen(
                archivpfad,
                format!("«{name}» fuehrt aus dem Zielordner heraus und ist ausgelassen"),
            );
            continue;
        };
        let pfad = zielordner.join(&innen);
        let ist_ordner = eintrag.is_dir();
        let rechte = eintrag.unix_mode();

        // Bei einem Ordnereintrag gehoert der Eintrag selbst zur Kette, bei
        // jedem anderen nur sein uebergeordneter Ordner.
        let kette = if ist_ordner {
            innen.as_path()
        } else {
            innen.parent().unwrap_or_else(|| Path::new(""))
        };
        if let Err(fehler) = kette_anlegen(zielordner, kette) {
            let name = eintrag.name().to_owned();
            drop(eintrag);
            steuerung.ueberspringen(archivpfad, format!("«{name}»: {}", grund(&fehler)));
            continue;
        }

        if ist_ordner {
            // Die Rechte des Ordners kommen zuletzt und nicht jetzt: ein Ordner
            // ohne Schreibrecht liesse sich sonst nicht mehr befuellen. Dieselbe
            // Reihenfolge und derselbe Grund wie bei
            // [`super::kopieren`]`::ordnerangaben_uebernehmen`.
            if let Some(rechte) = rechte {
                ordnerrechte.push((pfad.clone(), rechte));
            }
            drop(eintrag);
            steuerung.eintrag_fertig(&pfad, 0);
            continue;
        }

        if let Ok(vorhanden) = fs::symlink_metadata(&pfad)
            && vorhanden.is_symlink()
        {
            let name = eintrag.name().to_owned();
            drop(eintrag);
            steuerung.ueberspringen(
                archivpfad,
                format!("«{name}»: am Ziel steht schon eine Verknuepfung"),
            );
            continue;
        }

        if eintrag.is_symlink() {
            let ergebnis = verknuepfung_ablegen(&mut eintrag, &pfad);
            drop(eintrag);
            match ergebnis {
                Ok(()) => steuerung.eintrag_fertig(&pfad, 0),
                Err(fehler) => {
                    steuerung.ueberspringen(&pfad, grund(&fehler));
                }
            }
            continue;
        }

        match datei_schreiben(&mut eintrag, &pfad, steuerung) {
            Ablauf::Weiter => {}
            Ablauf::Abgebrochen => return Ablauf::Abgebrochen,
        }
        if let Some(rechte) = rechte {
            // Ein Archiv, das die Rechte fortwirft, macht aus einem
            // ausfuehrbaren Skript eine gewoehnliche Datei. Genommen werden
            // allein die neun Rechtebits; die oberen Modusbits sagen den Typ,
            // und den hat der Weg hierher schon entschieden.
            let _ = fs::set_permissions(&pfad, Permissions::from_mode(rechte & 0o777));
        }
    }
    Ablauf::Weiter
}

/// Legt die Ordnerkette zu einem Eintrag an und weist jeden Weg ab, der durch
/// eine Verknuepfung fuehrt.
///
/// **Das ist die zweite der beiden Sperren gegen ein Archiv, das aus seinem
/// Zielordner herausschreiben will**; die erste ist `enclosed_name`, und warum
/// sie allein nicht genuegt, steht im Kopf dieses Moduls.
///
/// Gefragt wird ueber `fs::symlink_metadata`, also `lstat(2)`, und das ist hier
/// die richtige Frage: gesucht ist der Eintrag **unter diesem Namen** und nicht
/// das, worauf er zeigt. Ein `fs::metadata` saehe bei `hinaus -> /etc` einen
/// Ordner und liesse den Weg durch.
///
/// Der Preis ist ein `lstat(2)` je Ordnerebene und Eintrag. Er faellt neben dem
/// Entpacken selbst nicht ins Gewicht: ein Archiveintrag kostet ohnehin ein
/// Oeffnen, ein Schreiben und das Auspacken seiner Bytes.
fn kette_anlegen(wurzel: &Path, kette: &Path) -> io::Result<()> {
    let mut hier = wurzel.to_path_buf();
    for teil in kette.components() {
        // `enclosed_name` hat bereits jede Komponente ausser einem blossen Namen
        // ausgeschlossen. Der Zweig steht trotzdem da, weil die
        // Fallunterscheidung vollstaendig ist und weil ein leises Durchwinken
        // hier die Sperre selbst waere, die aufgehoben wird.
        let Component::Normal(name) = teil else {
            return Err(io::Error::other(
                "der Weg zum Eintrag traegt einen unzulaessigen Bestandteil",
            ));
        };
        hier.push(name);
        match fs::symlink_metadata(&hier) {
            Ok(vorhanden) if vorhanden.is_symlink() => {
                return Err(io::Error::other(
                    "der Weg zum Eintrag fuehrt durch eine Verknuepfung aus dem Zielordner heraus",
                ));
            }
            Ok(vorhanden) if vorhanden.is_dir() => {}
            Ok(_) => {
                return Err(io::Error::other(
                    "auf dem Weg zum Eintrag steht eine Datei, wo ein Ordner stehen muesste",
                ));
            }
            Err(_) => fs::create_dir(&hier)?,
        }
    }
    Ok(())
}

/// Legt eine Verknuepfung an, wie das Archiv sie traegt.
///
/// Der Inhalt des Eintrags **ist** das Verweisziel. Geprueft wird es nicht:
/// eine Verknuepfung schreibt nichts, und durch sie hindurch schreibt der Lauf
/// nicht, weil [`kette_anlegen`] jeden Weg durch eine Verknuepfung abweist.
/// Damit steht am Ende dieselbe Verknuepfung da, die der Packer gesehen hat.
fn verknuepfung_ablegen(eintrag: &mut impl Read, pfad: &Path) -> io::Result<()> {
    let mut verweis = Vec::new();
    // Ein Verweisziel ist ein Pfad und keine Datei. `PATH_MAX` ist auf macOS
    // 1024; wer mehr mitbringt, bringt kein Verweisziel mit, und die Grenze
    // haelt eine erfundene Laengenangabe im Archiv davon ab, Speicher zu
    // fordern.
    eintrag.take(1024).read_to_end(&mut verweis)?;
    let verweis = String::from_utf8(verweis)
        .map_err(|_| io::Error::other("das Verweisziel ist kein gueltiger Text"))?;
    std::os::unix::fs::symlink(verweis, pfad)
}

/// Schreibt einen Dateieintrag, Stueck fuer Stueck.
///
/// Der Abbruch wird **innerhalb** der Datei geprueft und nicht nur zwischen
/// zwei Eintraegen, aus demselben Grund wie beim Packen: eine Datei von zwei
/// Gigabyte liefe sonst zu Ende, gleich wie oft der Nutzer `Esc` drueckt.
fn datei_schreiben(eintrag: &mut impl Read, pfad: &Path, steuerung: &mut Steuerung) -> Ablauf {
    let mut ausgabe = match File::create(pfad) {
        Ok(ausgabe) => ausgabe,
        Err(fehler) => {
            steuerung.ueberspringen(pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    };

    let mut puffer = vec![0_u8; STUECK];
    let mut bytes = 0_u64;
    loop {
        let stueck = match eintrag.read(&mut puffer) {
            Ok(0) => break,
            Ok(gelesen) => gelesen,
            Err(fehler) => {
                steuerung.teilstueck(bytes);
                steuerung.ueberspringen(pfad, grund(&fehler));
                drop(ausgabe);
                halbe_datei_wegraeumen(pfad, steuerung);
                return Ablauf::Weiter;
            }
        };
        if let Err(fehler) = ausgabe.write_all(&puffer[..stueck]) {
            steuerung.teilstueck(bytes);
            steuerung.ueberspringen(pfad, grund(&fehler));
            drop(ausgabe);
            halbe_datei_wegraeumen(pfad, steuerung);
            return Ablauf::Weiter;
        }
        bytes += stueck as u64;
        steuerung.zwischenstand(pfad, bytes);
        if steuerung.abgebrochen() {
            steuerung.teilstueck(bytes);
            drop(ausgabe);
            halbe_datei_wegraeumen(pfad, steuerung);
            return Ablauf::Abgebrochen;
        }
    }

    steuerung.eintrag_fertig(pfad, bytes);
    Ablauf::Weiter
}

/// Raeumt eine Datei weg, die nicht fertig geworden ist.
///
/// **Dieselbe Ueberlegung wie bei der halben Kopie in [`super::kopieren`]:** was
/// dasteht, ist kein Ergebnis, sondern ein Rest. Der Zielordner und alles, was
/// vor ihr fertig geworden ist, bleiben stehen.
fn halbe_datei_wegraeumen(pfad: &Path, steuerung: &mut Steuerung) {
    if let Err(fehler) = fs::remove_file(pfad)
        && fehler.kind() != io::ErrorKind::NotFound
    {
        steuerung.ueberspringen(
            pfad,
            format!("nach dem Abbruch nicht weggeraeumt: {}", grund(&fehler)),
        );
    }
}

/// Setzt die Rechte der angelegten Ordner, nachdem sie befuellt sind.
///
/// **Ein Fehler bleibt hier stumm, und das ist die Ausnahme und nicht die
/// Regel.** Ein Ordner, dessen Rechte nicht zu setzen sind, steht mit seinem
/// Inhalt vollstaendig da; ihn in der Abschlussliste als uebersprungen zu
/// nennen, waere die falsche Auskunft, und ein eigener Meldeweg fuer "steht da,
/// traegt aber die Vorgaberechte" ist mehr Mechanismus, als die Sache wiegt.
/// Die letzte Ebene kommt zuerst an die Reihe, damit ein Ordner ohne
/// Schreibrecht nicht seine eigenen Unterordner sperrt.
fn ordnerrechte_nachtragen(ordnerrechte: &[(PathBuf, u32)]) {
    for (pfad, rechte) in ordnerrechte.iter().rev() {
        let _ = fs::set_permissions(pfad, Permissions::from_mode(rechte & 0o777));
    }
}
