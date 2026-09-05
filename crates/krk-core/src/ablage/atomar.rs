//! Atomares Schreiben: erst vollstaendig in eine Nachbardatei, dann
//! `rename(2)` auf das Ziel.
//!
//! `rename` innerhalb eines Dateisystems ist unteilbar. Wer die Zieldatei
//! liest, sieht deshalb entweder den alten Inhalt ganz oder den neuen ganz,
//! nie eine halb geschriebene Datei. Ein Absturz vor dem `rename` laesst die
//! alte Datei genau so stehen, wie sie war.
//!
//! **Der Vorgang ist in zwei Schritte geteilt, und das ist keine Bequemlichkeit
//! fuer die Pruefung.** [`vorbereiten`] endet genau in der Luecke zwischen
//! Schreiben und Umbenennen, [`Nachbardatei::umbenennen`] schliesst sie. Nur so
//! laesst sich die Zusage "ein Abbruch dazwischen laesst die alte Datei
//! unveraendert" an einem Prozess pruefen, der wirklich stirbt, statt an einem,
//! der die Luecke nur nachspielt. [`schreiben`] setzt beide Schritte zusammen
//! und ist der Weg, den der Alltag geht.
//!
//! # Geschrieben wird aus einem Leser und nicht aus einer Zeichenkette
//!
//! Bis zur Runde 9 nahmen beide Funktionen `&str`. Die Runde 9 hat den
//! Notizzettel gebracht, und mit ihm zwei Nutzlasten, die keine Zeichenkette
//! sind und es nicht werden koennen:
//!
//! - **Eine Zetteldatei, die kein gueltiges UTF-8 traegt.** Sie wird
//!   beiseitegelegt, also Byte fuer Byte kopiert; eine ungueltige Bytefolge ist
//!   definitionsgemaess kein `&str`, und ein Ersatzzeichen darin waere genau der
//!   Verlust, den das Beiseitelegen verhindern soll.
//! - **Eine Zetteldatei ueber `text::datei::EDITORGRENZE`.** Sie darf zu keinem
//!   Zeitpunkt vollstaendig im Arbeitsspeicher stehen; aus einem Leser flieszt
//!   sie ueber [`io::copy`] in Stuecken auf die Platte.
//!
//! **Eine obere Schranke fuer die Menge steht hier nicht und gehoert nicht
//! hierher.** Diese Datei schreibt, was ihr gereicht wird, bis die Quelle zu
//! Ende ist; wer begrenzen will, reicht einen `Take` herein und liest an dessen
//! `limit()` ab, ob die Quelle vorher zu Ende war. So macht es
//! `Zugang::beiseite_legen`, und dort steht auch der Grund.
//!
//! Wer eine Zeichenkette hat, uebergibt `&mut text.as_bytes()`; `&[u8]` ist
//! selbst ein Leser. Eine zweite Schreibfunktion neben dieser waere der zweite
//! Schreibweg, den der Datensatz vom 260812-1105 ausschliesst.
//!
//! # Die Rechte der Zieldatei ueberleben das Schreiben
//!
//! `rename` ersetzt die Datei, und alles, was am ersetzten Eintrag hing, haengt
//! danach am neuen. Bis zum 260905 hiess das: die Nachbardatei kam aus
//! `fs::File::create`, trug also `0666 & ~umask`, und dieser Modus stand nach
//! dem Umbenennen an der Stelle des Ziels. Eine Datei auf `600` war danach fuer
//! jeden Nutzer des Geraetes lesbar, und ein Script auf `755` lief nicht mehr
//! (`shared/issues/260904-1902_*_das-atomare-schreiben-weitet-die-rechte-einer-600-datei-auf-644.md`).
//!
//! [`vorbereiten`] uebertraegt deshalb die neun Rechtebits eines **bestehenden**
//! Ziels auf die Nachbardatei, und zwar **bevor** der Inhalt in sie flieszt. Die
//! Reihenfolge ist der Punkt: stuende die Uebertragung erst vor dem `rename`,
//! laege der Inhalt einer `600`-Datei fuer die Dauer des Schreibens unter `644`
//! neben ihr.
//!
//! **Was diese Datei nicht uebertraegt, uebertraegt niemand**, und das steht
//! hier, damit ein spaeterer Leser nicht danach sucht: Besitzer und Gruppe, die
//! erweiterten Attribute samt Finder-Marken, die Zugriffslisten, die
//! Dateiflags, das Anlagedatum und die harten Verweise auf denselben Inhalt
//! gehen mit dem ersetzten Eintrag verloren; die drei Sonderbits `setuid`,
//! `setgid` und `sticky` bleiben absichtlich aussen vor, siehe [`RECHTEMASKE`].
//! Das Aenderungsdatum steht danach auf jetzt, und das ist richtig: die Datei
//! **ist** gerade geaendert worden. Der Datensatz dazu ist
//! `shared/issues/260905-0406_*_das-atomare-schreiben-verliert-besitzer-attribute-und-zugriffslisten-der-ersetzten-datei.md`.

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Die Endung, die die Nachbardatei vom Ziel unterscheidet.
pub const NACHBARENDUNG: &str = "neu";

/// Die Endung, unter der eine beschaedigte Datei zur Seite gelegt wird.
pub const BESCHAEDIGTENDUNG: &str = "beschaedigt";

/// Die Bits des Dateimodus, die diese Datei uebertraegt: die neun Rechtebits
/// und **nichts** darueber hinaus.
///
/// Ausserhalb der Maske liegen zwei Dinge, und sie bleiben aus zwei
/// verschiedenen Gruenden draussen.
///
/// - **Die Typbits.** `metadata().permissions().mode()` liefert das ganze
///   `st_mode`, also auch `S_IFREG`. `chmod(2)` laesst die Wirkung dieser Bits
///   ausdruecklich unspezifiziert; sie mitzureichen hiesse, sich auf etwas zu
///   verlassen, das keine Zusage ist.
/// - **Die Sonderbits `setuid`, `setgid` und `sticky`.** Sie bleiben bewusst
///   aussen vor. Die Nachbardatei gehoert dem schreibenden Nutzer, das
///   ersetzte Ziel muss ihm nicht gehoert haben; ein mitgetragenes `setuid`
///   uebertruege das Recht eines fremden Besitzers auf einen Inhalt, den dieser
///   Nutzer geschrieben hat. Verloren geht dabei nichts, was heute erhalten
///   bliebe: vor dem 260905 fielen alle zwoelf Bits.
pub const RECHTEMASKE: u32 = 0o777;

/// Der Pfad der Nachbardatei zu einem Ziel.
///
/// Der Name ist fest abgeleitet und traegt keine Laufnummer. Ein Absturz
/// hinterlaesst damit hoechstens eine einzige liegengebliebene Datei statt
/// einer wachsenden Reihe, und der naechste Schreibvorgang raeumt sie ab.
/// Gelesen wird sie von niemandem.
///
/// **Der Nachbar [`beiseitepfad`] traegt ebenfalls keine Laufnummer, aber aus
/// dem umgekehrten Grund**, und wer die beiden Begruendungen fuer dieselbe
/// Regel haelt, verwechselt sie beim naechsten Umbau: die Nachbardatei darf
/// ueberschrieben werden, weil niemand sie liest; die zur Seite gelegte Datei
/// darf es gerade nicht, weil sie genau dafuer dasteht.
pub fn nachbarpfad(ziel: &Path) -> io::Result<PathBuf> {
    mit_endung(ziel, NACHBARENDUNG)
}

/// Der Pfad, unter dem der Inhalt einer beschaedigten Datei liegen bleibt.
///
/// Der Name ist fest abgeleitet und traegt **keine Laufnummer**, und der Grund
/// ist der umgekehrte zu dem bei [`nachbarpfad`]: diese Datei wird gelesen, und
/// was sie wert macht, ist die **erste** zur Seite gelegte Fassung, nicht die
/// letzte. Eine Laufnummer legte eine wachsende Reihe in einem Ordner an, den
/// KRK selbst verwaltet und niemand aufraeumt; ein Ueberschreiben verschoebe
/// den Datenverlust um einen Start. So entschieden am 260812-1105
/// (`decisions/260812-1000_*_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`,
/// Moeglichkeit 1). Wer hier hinschreibt, fragt deshalb vorher, ob schon etwas
/// dasteht.
///
/// Die Endung wird **angehaengt** und ersetzt nichts: aus `bookmarks.toml` wird
/// `bookmarks.toml.beschaedigt`. Ein vorangestelltes Praefix waere gefaehrlich,
/// und ein Ersetzen der Endung ergaebe `bookmarks.beschaedigt`, was den
/// urspruenglichen Namen nicht mehr nennt. [`super::Ablageort::datei`] fragt
/// nach [`super::Datei`] und liest damit keinen der abgeleiteten Namen als
/// Ablagedatei.
pub fn beiseitepfad(ziel: &Path) -> io::Result<PathBuf> {
    mit_endung(ziel, BESCHAEDIGTENDUNG)
}

/// Haengt eine Endung an den Dateinamen eines Ziels.
///
/// Die eine Ableitung fuer beide Nachbarnamen. Zwei Kopien derselben vier
/// Zeilen waeren zwei Stellen, an denen der Umgang mit einem Pfad ohne
/// Dateinamen auseinanderlaufen koennte.
fn mit_endung(ziel: &Path, endung: &str) -> io::Result<PathBuf> {
    let Some(name) = ziel.file_name() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} traegt keinen Dateinamen", ziel.display()),
        ));
    };
    let mut nachbarname = name.to_os_string();
    nachbarname.push(".");
    nachbarname.push(endung);
    Ok(ziel.with_file_name(nachbarname))
}

/// Eine geschriebene, aber noch nicht umbenannte Nachbardatei.
///
/// Solange sie besteht, ist das Ziel unberuehrt. Wird sie fallengelassen,
/// statt umbenannt zu werden, raeumt sie sich ab. Ein Absturz fuehrt kein
/// `Drop` aus; dann bleibt die Nachbardatei liegen, und genau deshalb muss ihr
/// Name harmlos sein, siehe [`nachbarpfad`].
#[must_use = "ohne umbenennen() bleibt das Ziel unveraendert"]
#[derive(Debug)]
pub struct Nachbardatei {
    ziel: PathBuf,
    nachbar: PathBuf,
    abraeumen: bool,
}

impl Nachbardatei {
    /// Das Ziel, auf das diese Nachbardatei umbenannt wird.
    pub fn ziel(&self) -> &Path {
        &self.ziel
    }

    /// Die Nachbardatei selbst.
    pub fn nachbarpfad(&self) -> &Path {
        &self.nachbar
    }

    /// Der zweite Schritt: das unteilbare Umbenennen auf das Ziel.
    ///
    /// Scheitert es, bleibt das Ziel unveraendert und die Nachbardatei wird
    /// abgeraeumt.
    pub fn umbenennen(mut self) -> io::Result<()> {
        fs::rename(&self.nachbar, &self.ziel)?;
        self.abraeumen = false;
        Ok(())
    }
}

impl Drop for Nachbardatei {
    fn drop(&mut self) {
        if self.abraeumen {
            let _ = fs::remove_file(&self.nachbar);
        }
    }
}

/// Die neun Rechtebits eines offenen Deskriptors.
fn rechte_am_deskriptor(datei: &fs::File) -> io::Result<u32> {
    use std::os::unix::fs::PermissionsExt;
    Ok(datei.metadata()?.permissions().mode() & RECHTEMASKE)
}

/// Setzt die Rechte eines bestehenden Ziels auf die Nachbardatei.
///
/// Gefragt wird ueber `metadata` und nicht ueber `symlink_metadata`, also nach
/// dem, worauf eine Verknuepfung zeigt. Dieselbe Wahl wie in
/// `text::datei::oeffnen`, und sie muss dieselbe sein: sonst laese der Editor
/// die eine Datei und erbte die Rechte der anderen.
///
/// **Gibt es das Ziel noch nicht, bleibt es bei den Vorgaberechten des
/// Prozesses**, also bei `0666 & ~umask` aus [`fs::File::create`]. Eine neu
/// angelegte Datei hat nichts zu erben, und eine Zahl an dieser Stelle waere
/// eine Vorgabe, die diese Datei sich selbst ausdaechte, statt die des Nutzers
/// zu nehmen.
///
/// # Ein Fehlschlag haelt das Schreiben an
///
/// Und zwar auch dann, wenn `set_permissions` selbst `Ok` meldet: gefragt wird
/// danach ein zweites Mal am Deskriptor, und stimmt der Modus dann nicht,
/// scheitert der ganze Vorgang. Ein Dateisystem, das `chmod` still
/// wegwirft, wuerde sonst genau den Defekt zurueckbringen, der hier behoben
/// ist — und dieses Projekt hat am 260904 einen stillen Fehlschlag beim
/// Sichern behoben und baut keinen zweiten ein.
///
/// **Der Preis ist benannt und bewusst getragen:** auf einem Dateisystem ohne
/// Rechteverwaltung scheitert das Sichern, statt die Datei mit fremden Rechten
/// hinzulegen. Der Nutzer verliert dabei nichts — die alte Datei steht
/// unveraendert, sein Stand steht im Editor —, er bekommt eine Meldung. Und
/// der Fall bleibt eng: gefragt wird nur, wenn der Modus wirklich abweicht.
/// Ein Dateisystem, das ohnehin fuer jede Datei denselben Modus meldet, kommt
/// hier nie vorbei.
fn rechte_uebernehmen(ziel: &Path, nachbar: &fs::File) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let soll = match fs::metadata(ziel) {
        Ok(angaben) => angaben.permissions().mode() & RECHTEMASKE,
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(fehler) => return Err(fehler),
    };
    if rechte_am_deskriptor(nachbar)? == soll {
        return Ok(());
    }
    nachbar.set_permissions(fs::Permissions::from_mode(soll))?;
    let gesetzt = rechte_am_deskriptor(nachbar)?;
    if gesetzt != soll {
        return Err(io::Error::other(format!(
            "die Rechte {soll:o} von {} lassen sich nicht uebertragen; die Nachbardatei steht auf {gesetzt:o}",
            ziel.display()
        )));
    }
    Ok(())
}

/// Der erste Schritt: den Inhalt vollstaendig in die Nachbardatei schreiben.
///
/// Nach der Rueckkehr stehen die Daten auf der Platte, das Ziel ist noch alt.
/// `sync_all` sorgt dafuer, dass das Umbenennen nicht einen Namen auf einen
/// noch nicht geschriebenen Inhalt setzt.
///
/// Die Quelle wird bis zu ihrem Ende gelesen, und eine eigene Obergrenze setzt
/// diese Funktion nicht; wer eine braucht, reicht einen begrenzten Leser
/// herein. Warum ein Leser und keine Zeichenkette, steht im Modulkopf.
///
/// # Drei Schritte in dieser Reihenfolge, und jeder aus einem Grund
///
/// 1. **Eine liegengebliebene Nachbardatei kommt weg**, statt ueberschrieben zu
///    werden. `fs::File::create` oeffnet zum Schreiben und scheitert an einer
///    Datei ohne Schreibrecht mit `EACCES`; ein Rest, den ein Absturz nach der
///    Rechteuebernahme eines `444`-Ziels hinterlaesst, sperrte sonst jedes
///    weitere Sichern dieser Datei, bis der Nutzer ihn von Hand entfernt. Der
///    Rueckgabewert faellt weg, weil der haeufigste Fall "es lag nichts da"
///    ist; scheitert das Abraeumen aus einem anderen Grund, meldet das
///    `create` in der Zeile darunter.
/// 2. **Die Rechte des Ziels gehen auf die Nachbardatei**, siehe
///    [`rechte_uebernehmen`].
/// 3. **Dann erst flieszt der Inhalt.** Die Rechtepruefung eines offenen
///    Deskriptors steht beim Oeffnen; ein `chmod` auf `400` dazwischen nimmt
///    dem schon offenen `datei` das Schreiben nicht.
pub fn vorbereiten(ziel: &Path, quelle: &mut impl Read) -> io::Result<Nachbardatei> {
    let nachbar = nachbarpfad(ziel)?;
    let _ = fs::remove_file(&nachbar);
    let mut datei = fs::File::create(&nachbar)?;
    // Ab hier haelt der Wert die Nachbardatei, und sein `Drop` raeumt sie ab.
    // Er steht vor den drei Schritten und nicht hinter ihnen, damit ein
    // Fehlschlag in ihnen keinen Rest hinterlaesst; `datei` bleibt dabei offen,
    // was auf einer bereits entfernten Datei nichts ausmacht.
    let angelegt = Nachbardatei {
        ziel: ziel.to_path_buf(),
        nachbar,
        abraeumen: true,
    };
    rechte_uebernehmen(ziel, &datei)?;
    io::copy(quelle, &mut datei)?;
    datei.sync_all()?;
    drop(datei);
    Ok(angelegt)
}

/// Schreibt den Inhalt atomar auf das Ziel.
pub fn schreiben(ziel: &Path, quelle: &mut impl Read) -> io::Result<()> {
    vorbereiten(ziel, quelle)?.umbenennen()
}
