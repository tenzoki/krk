#![deny(unsafe_code)]
//! Das Binaerziel von KRK: Fenster, Menue, Dateifenster, Ereignisabgriff.
//!
//! Jeder AppKit-Aufruf ist ein unsicherer Fremdaufruf. Bezahlt wird das an
//! genau einer Stelle: das Modul `appkit` traegt `#[allow(unsafe_code)]` am
//! Kopf seiner `mod.rs` und haelt die sicheren Huellen. Ausserhalb davon
//! bricht der Bau ab, sobald ein `unsafe`-Block entsteht.
//!
//! Die Regel oben lautet `deny` und nicht `warn`: eine Warnung bricht den Bau
//! nicht ab, die Grenze zum Modul `appkit` waere damit nur beobachtbar. Unter
//! `deny` ist sie maschinell erzwungen. Entschieden am 260803,
//! `decisions/260803-1208_*_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`.
//!
//! Sie lautet auch nicht `forbid`, weil `forbid` sich nicht mehr oeffnen
//! liesse und `appkit` die Ausnahme braucht.
//!
//! Zwoelf Module liegen ausdruecklich **neben** `appkit` und nicht darin, und
//! keines von ihnen nennt eine `objc2`-Kiste. `messmodus` haelt den Ablauf der
//! Fruehmessung. `fenstermodell` haelt das aktive Dateifenster, die
//! Sichtbarkeit der fuenf Bereiche und ihre Breiten. `tabs` haelt die Tabs eines
//! Dateifensters samt ihrem Inhalt und der Reihenfolge, in der sie gelesen
//! werden. `vorschaumodell` haelt die Tabs des Vorschaufensters aus C6 samt
//! Halteverhalten und dem Lesen der Vorschaudatei auf dem Arbeitsfaden.
//! `editormodell` haelt, was der eingebaute Editor der Runde 2 ueber die
//! geoeffnete Datei weiss: ihren Pfad, ihren Stand, ob der Stand von der Datei
//! abweicht, die gewaehlte Ansicht und den laufenden Suchlauf. `hervorhebung`
//! haelt daneben, was die Formatansicht des Editors ueber einen Text weiss:
//! welche Stelle welche Farbe traegt, welche unterstrichen ist und welche eine
//! Markdown-Auszeichnung ist. `leistenmodell` haelt, was in der Lesezeichen-
//! und Geraeteleiste aus C5 steht und welche Zeile darin ausgewaehlt ist.
//! `kommandos` haelt die Rechnung hinter den Tastenbefehlen aus C2 und
//! C10, darunter die eine Stelle, die einen Pfad prueft. `auffrischung` haelt
//! den einen Auffrischungspfad aus C9: welche Dateifenster ein gemeldeter Pfad
//! angeht und was mit ihnen geschieht. `belegungsmodell` haelt die
//! Arbeitskopie der Belegung, solange die Belegungsansicht aus C3 offen ist.
//! `fenstertitel` rechnet aus dem Fokus und drei Pfaden aus, was im
//! Fenstertitel steht (C11 der Runde 2). `belegungsausgabe` erzeugt die
//! Tastenbelegung als Markdown, loest den Zielpfad im Downloads-Ordner auf,
//! schreibt sie unteilbar und formuliert die Meldung fuer die Statuszeile
//! (Runde 3); dass es keine `objc2`-Kiste nennt, ist der Grund, aus dem der
//! ganze Inhalt der Datei ohne Fenster pruefbar ist.

mod appkit;
mod auffrischung;
mod belegungsausgabe;
mod belegungsmodell;
mod editormodell;
mod fenstermodell;
mod fenstertitel;
mod hervorhebung;
mod kommandos;
mod leistenmodell;
mod messmodus;
/// Der Pruefordner der Proben dieser Kiste. Nur im Probenbau uebersetzt, weil
/// ihn kein ausgeliefertes Programm braucht; vier Probenmodule teilen ihn.
#[cfg(test)]
mod pruefordner;
mod tabs;
mod vorschaumodell;

/// Die Befehlszeilenmarke, die den Protokollmodus des Ereignisabgriffs
/// einschaltet.
const MARKE_TASTEN_PROTOKOLL: &str = "--tasten-protokoll";

/// Die Befehlszeilenmarke, die das gebaute Hauptmenue ausliest.
///
/// Derselbe Zuschnitt wie die Marke darueber und an derselben Stelle
/// abgefragt. Sie schreibt jeden Eintrag des Hauptmenues mit Beschriftung,
/// Kuerzel, Zusatztasten und Selektor auf die Standardausgabe und beendet, ohne
/// ein Fenster zu oeffnen. Damit prueft die Abnahme von C3 durch **Auslesen**
/// statt durch Aufzaehlen der heute bekannten Zusaetze; eine Aufzaehlung
/// veraltet mit der naechsten macOS-Version, und genau diesen Fall hat das
/// Vorhaben mit "Close All" schon erlebt.
const MARKE_MENUE_PROTOKOLL: &str = "--menue-protokoll";

/// Der Rueckgabewert bei einer falsch aufgerufenen Befehlszeile.
const AUFRUFFEHLER: i32 = 2;

fn main() {
    // Unbekannte Marken werden uebergangen und nicht bemaengelt: LaunchServices
    // haengt einem ueber den Finder gestarteten Buendel eigene an. Eine
    // fehlerhafte `--messmodus`-Angabe ist etwas anderes: sie ist genannt, aber
    // unvollstaendig, und daraus stillschweigend einen gewoehnlichen Start zu
    // machen hiesse, ein Fenster zu oeffnen, wo eine Messung bestellt war.
    let argumente: Vec<String> = std::env::args().skip(1).collect();
    let tasten_protokoll = argumente
        .iter()
        .any(|marke| marke == MARKE_TASTEN_PROTOKOLL);
    let menue_protokoll = argumente.iter().any(|marke| marke == MARKE_MENUE_PROTOKOLL);
    let messaufgabe = match messmodus::Aufgabe::aus_argumenten(&argumente) {
        Ok(aufgabe) => aufgabe,
        Err(meldung) => {
            eprintln!("krk: {meldung}");
            std::process::exit(AUFRUFFEHLER);
        }
    };
    appkit::starten(tasten_protokoll, menue_protokoll, messaufgabe);
}
