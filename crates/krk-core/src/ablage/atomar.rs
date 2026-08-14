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

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Die Endung, die die Nachbardatei vom Ziel unterscheidet.
pub const NACHBARENDUNG: &str = "neu";

/// Die Endung, unter der eine beschaedigte Datei zur Seite gelegt wird.
pub const BESCHAEDIGTENDUNG: &str = "beschaedigt";

/// Der Pfad der Nachbardatei zu einem Ziel.
///
/// Der Name ist fest abgeleitet und traegt keine Laufnummer. Ein Absturz
/// hinterlaesst damit hoechstens eine einzige liegengebliebene Datei statt
/// einer wachsenden Reihe, und der naechste Schreibvorgang ueberschreibt sie.
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

/// Der erste Schritt: den Inhalt vollstaendig in die Nachbardatei schreiben.
///
/// Nach der Rueckkehr stehen die Daten auf der Platte, das Ziel ist noch alt.
/// `sync_all` sorgt dafuer, dass das Umbenennen nicht einen Namen auf einen
/// noch nicht geschriebenen Inhalt setzt.
///
/// Die Quelle wird bis zu ihrem Ende gelesen, und eine eigene Obergrenze setzt
/// diese Funktion nicht; wer eine braucht, reicht einen begrenzten Leser
/// herein. Warum ein Leser und keine Zeichenkette, steht im Modulkopf.
pub fn vorbereiten(ziel: &Path, quelle: &mut impl Read) -> io::Result<Nachbardatei> {
    let nachbar = nachbarpfad(ziel)?;
    let mut datei = fs::File::create(&nachbar)?;
    io::copy(quelle, &mut datei)?;
    datei.sync_all()?;
    drop(datei);
    Ok(Nachbardatei {
        ziel: ziel.to_path_buf(),
        nachbar,
        abraeumen: true,
    })
}

/// Schreibt den Inhalt atomar auf das Ziel.
pub fn schreiben(ziel: &Path, quelle: &mut impl Read) -> io::Result<()> {
    vorbereiten(ziel, quelle)?.umbenennen()
}
