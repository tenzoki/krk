//! Das Register der Archivzugaenge: ein Zugang je Archivdatei und Prozess.
//!
//! **Zwei Tabs auf demselben Archiv teilen einen Zugang** und damit ein
//! Inhaltsverzeichnis (E2 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`): wer in einem
//! zweiten Fenster dasselbe grosse tar.gz betritt, wartet nicht ein zweites
//! Mal auf den Durchlauf durch den ganzen Strom.
//!
//! **Gefragt wird allein auf dem Hauptfaden, und die Frage stellt keinen
//! Systemaufruf**: sie schlaegt den Pfad in einer Tafel nach und legt, wenn
//! dort kein lebender Zugang steht, einen neuen an, der die Datei noch nicht
//! oeffnet ([`Archivzugang::neu`]). Der Schluessel ist der Pfad, wie er im
//! Tab steht, und nicht der aufgeloeste; zwei Schreibweisen derselben Datei
//! bekommen zwei Zugaenge, und das kostet allein ein zweites Lesen.
//!
//! **Das Register haelt die Zugaenge nicht am Leben**, es merkt sie schwach
//! (`Weak`). Ein Zugang lebt, solange ein Tab ihn zeigt; danach faellt er samt
//! seinem Verzeichnis, und das naechste Betreten liest neu.
//!
//! # Abweichung vom Plan
//!
//! Der Plan legt das Register beim Anwendungsdelegierten ab. Hier steht es in
//! einer fadengebundenen Tafel dieses Moduls: der Einstieg geschieht in der
//! Datenquelle jedes Dateifensters, und ein Register beim Delegierten
//! braeuchte einen weiteren Rueckruf in beide Datenquellen, nur um eine Tafel
//! herumzureichen. Die Zusage bleibt dieselbe: es gibt eines je Prozess fuer
//! den Hauptfaden, gefragt ohne Systemaufruf, und kein Arbeitsfaden fragt es
//! (er saehe eine eigene, leere Tafel). Ohne AppKit, also ohne Fenster
//! pruefbar.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

use krk_core::archiv::{Archivformat, Archivzugang};

thread_local! {
    /// Die Tafel des Hauptfadens.
    static REGISTER: RefCell<HashMap<PathBuf, Weak<Archivzugang>>> =
        RefCell::new(HashMap::new());
}

/// Der Zugang zur Archivdatei `datei` im Format `format`: der lebende aus dem
/// Register, sonst ein neuer, der dort vermerkt wird.
///
/// Stellt keinen Systemaufruf. Ein vermerkter Zugang mit einem anderen Format
/// gilt nicht; das kann allein ein Name ergeben, dessen Endung sich zwischen
/// zwei Fragen nicht geaendert hat, und steht deshalb nur der Vollstaendigkeit
/// halber da.
#[must_use]
pub fn zugang(datei: &Path, format: Archivformat) -> Arc<Archivzugang> {
    REGISTER.with(|register| {
        let mut register = register.borrow_mut();
        if let Some(lebend) = register.get(datei).and_then(Weak::upgrade)
            && lebend.format() == format
        {
            return lebend;
        }
        // Tote Eintraege fallen hier und nicht in einem eigenen Lauf: das
        // Register waechst damit hoechstens um die Archive, die zugleich
        // gezeigt werden, und ein weiterer.
        register.retain(|_, zugang| zugang.strong_count() > 0);
        let neu = Arc::new(Archivzugang::neu(datei.to_path_buf(), format));
        register.insert(datei.to_path_buf(), Arc::downgrade(&neu));
        neu
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derselbe_pfad_teilt_den_zugang_solange_er_lebt() {
        let datei = Path::new("/gibt/es/nicht/a.zip");
        let erster = zugang(datei, Archivformat::Zip);
        let zweiter = zugang(datei, Archivformat::Zip);
        assert!(Arc::ptr_eq(&erster, &zweiter));
        let anderer = zugang(Path::new("/gibt/es/nicht/b.zip"), Archivformat::Zip);
        assert!(!Arc::ptr_eq(&erster, &anderer));
        drop((erster, zweiter));
        let spaeter = zugang(datei, Archivformat::Zip);
        assert_eq!(spaeter.datei(), datei);
        assert_eq!(Arc::strong_count(&spaeter), 1);
    }
}
