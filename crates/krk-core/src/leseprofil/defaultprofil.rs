//! Das eingebaute Default-Profil: drei Zaehlzeilen fuer jeden Ordner, den kein
//! Profil aus `readers.toml` erkennt.
//!
//! Es ist ein gewoehnliches [`Profil`] mit drei Zeilen, in Rust gebaut statt
//! aus TOML gelesen: „Dateien", „Ordner" und „Verknuepfungen", in dieser
//! Reihenfolge (Festlegung A1, C2.1), jede ein [`Baustein::Zaehlung`] ohne
//! Muster, auf den erkannten Ordner selbst, auf einen der drei Typen
//! eingeschraenkt und mit der Klammer der versteckten (C3.5). Es benutzt damit
//! dieselbe Zaehlmaschine wie jedes Profil der Datei, und ein zweiter Zaehlweg
//! entsteht nicht (C3.7).
//!
//! **Es erkennt nichts.** Ein Profil aus der Datei traegt ein Pfadmuster oder
//! eine Kennzeichendatei und tritt ein, wo eines davon trifft; dieses hier
//! tritt ein, wo **nichts** getroffen hat. Sein Name und seine zwei Muster
//! stehen deshalb auf dem, was [`super::erkennung::erkennen`] nie zu sehen
//! bekommt: der Name dient allein Meldungen, die Muster sind `None`. Gefragt
//! wird es an genau einer Stelle, dem Rueckfallzweig in
//! [`super::bausteine::zusammenfassen_gezaehlt`], nachdem beide
//! Erkennungsdurchgaenge leer ausgegangen sind.
//!
//! # Warum es kein Block in `readers.toml` ist
//!
//! Die Profildatei beantwortet, was an einem **erkannten** Ort liegt, und ein
//! Block darin muesste treffen, um zu gelten. Ein Block, der jeden Pfad
//! traefe, stuende in der Reihenfolge der Datei vor oder hinter den anderen
//! und verdraengte je nach Stelle jedes Profil danach — der Nutzer ordnet
//! seine Profile ueber die Blockreihenfolge (C2.2 der Runde 16), und ein Block
//! mit Allesfaenger machte diese Ordnung zur Falle. Dazu kommt, dass eine
//! Nutzerdatei, die bis auf den letzten Block geleert oder beim Start
//! beiseitegelegt ist, die drei Zeilen nicht mitnehmen darf (C1.3, C1.4): was
//! aus der Datei kaeme, fiele mit ihr.
//!
//! # Warum es sich nicht abschalten laesst
//!
//! Die drei Zeilen treten **unter** die sechs Metadatenangaben und ersetzen
//! nichts; wer sie nicht will, verliert nichts, wenn er sie ueberliest. Ein
//! Schalter in `settings.toml` oder ein Schluessel in `readers.toml` waere ein
//! zweiter Weg, auf dem ein Ordner ohne Profiltreffer eine andere Anzeige
//! bekaeme, und Constraint 4 des Specs laesst genau einen Rueckfallweg zu
//! (C1.5). Wer die drei Zeilen fuer einen Ort anders haben will, schreibt ein
//! Profil, das den Ort erkennt: es verdraengt die Metadatenanzeige samt den
//! drei Zeilen, und C3.5 sagt zu, dass es dieselben Zahlen liefern kann.
//!
//! # Warum die drei Beschriftungen hier stehen und nicht in der Ansicht
//!
//! Die Ansicht in `krk-ui` bekommt fertige [`super::Zusammenfassungszeile`]n
//! mit Beschriftung und Wert und setzt sie mit [`super::zeilen_als_text`] zu
//! Text, wie sie es fuer jede Profilzeile tut. Stuenden die drei Woerter dort,
//! gaebe es zwei Stellen, an denen eine Zeile ihre Beschriftung bekommt, und
//! eine Probe ohne Fenster koennte die drei nicht sehen (C4.5). Hier stehen
//! sie als Zeilen desselben Profils, das sie rechnet, und die Proben in
//! `crates/krk-core/tests/leseprofil.rs` lesen sie ab wie jede andere.

use std::sync::LazyLock;

use crate::verzeichnis::Typ;

use super::{Baustein, Ortsangabe, Profil, Zeile};

/// Die Beschriftungen der drei Zeilen, in der Reihenfolge der Anzeige
/// (Festlegung A1).
///
/// Die dritte heisst „Verknuepfungen" mit Umlaut, weil sie Anzeigetext ist
/// und keine Kennung; der Bezeichner daneben traegt ihn nicht, wie jeder
/// Bezeichner dieses Baums.
const BESCHRIFTUNGEN: [(&str, Typ); 3] = [
    ("Dateien", Typ::Datei),
    ("Ordner", Typ::Ordner),
    ("Verknüpfungen", Typ::Verknuepfung),
];

/// Der Name, den das Profil fuer Meldungen traegt. Angezeigt wird er nicht.
const NAME: &str = "Eingebautes Default-Profil";

/// Das Profil, beim ersten Zugriff gebaut und danach fuer die Lebensdauer
/// des Programms gehalten.
///
/// Ein `LazyLock` und kein `const`, weil [`Ortsangabe::wurzel`] einen `Vec`
/// anlegt. Die drei Zeilen tragen kein Muster, also uebersetzt der erste
/// Zugriff keinen regulaeren Ausdruck.
static DEFAULTPROFIL: LazyLock<Profil> = LazyLock::new(|| {
    let zeilen = BESCHRIFTUNGEN
        .iter()
        .map(|(beschriftung, typ)| {
            Zeile::neu(
                (*beschriftung).to_owned(),
                Some(Baustein::Zaehlung {
                    ort: Ortsangabe::wurzel(),
                    muster: None,
                    typ: Some(*typ),
                    versteckt: true,
                }),
            )
        })
        .collect();
    Profil::neu(NAME.to_owned(), None, None, zeilen)
});

/// Das eingebaute Default-Profil.
///
/// Der eine Zugang; der Modulkopf sagt, wer ihn ruft und warum sonst niemand.
#[must_use]
pub fn defaultprofil() -> &'static Profil {
    &DEFAULTPROFIL
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Festlegung A1: drei Zeilen, diese Beschriftungen, diese Reihenfolge,
    /// und jede zaehlt genau einen Typ mit Klammer im erkannten Ordner selbst.
    #[test]
    fn das_default_profil_traegt_genau_die_drei_zaehlzeilen() {
        let profil = defaultprofil();
        assert!(profil.pfad().is_none() && profil.kennzeichen().is_none());
        let zeilen = profil.zeilen();
        assert_eq!(zeilen.len(), 3);
        for (zeile, (beschriftung, typ)) in zeilen.iter().zip(BESCHRIFTUNGEN) {
            assert_eq!(zeile.beschriftung(), beschriftung);
            let Some(Baustein::Zaehlung {
                ort,
                muster,
                typ: gezaehlt,
                versteckt,
            }) = zeile.baustein()
            else {
                panic!("die Zeile {beschriftung} ist keine Zaehlung");
            };
            assert_eq!(*ort, Ortsangabe::wurzel());
            assert!(muster.is_none());
            assert_eq!(*gezaehlt, Some(typ));
            assert!(*versteckt);
        }
    }
}
