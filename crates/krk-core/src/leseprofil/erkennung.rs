//! Welches Profil ein ausgewaehlter Ordner bekommt.
//!
//! Ein Profil erkennt seinen Ort ueber ein **Pfadmuster** auf dem vollen Pfad
//! des Ordners oder ueber eine **Kennzeichendatei**, also ein Muster auf den
//! Namen der Eintraege darin, oder ueber beides. [`erkennen`] ist die eine
//! Stelle, an der diese Frage beantwortet wird; greift kein Profil, bleibt es
//! bei der heutigen Metadatenanzeige, und das ist kein Fehlerfall, sondern der
//! Nutzerwille vom 260823.
//!
//! # Zwei Durchgaenge, und der zweite ist kein Nachtrag zum ersten
//!
//! Die Regel aus C2 sind **zwei** Regeln und nicht eine, und sie stehen
//! uebereinander:
//!
//! 1. Das Pfadmuster geht der Kennzeichendatei vor. Das ist die aeussere
//!    Ordnung, und sie entscheidet, welcher Durchgang zuerst laeuft.
//! 2. Innerhalb eines Durchgangs gewinnt das erste Profil in der Reihenfolge
//!    der Datei (Festlegung A1). Das ist die innere Ordnung, und der Nutzer
//!    stellt sie ein, indem er die `[[profil]]`-Bloecke verschiebt (C2.2).
//!
//! ```text
//! Durchgang 1:  Profil 1 … Profil n,  je nur das Pfadmuster
//! Durchgang 2:  Profil 1 … Profil n,  je nur die Kennzeichendatei
//! sonst:        die heutige Metadatenanzeige
//! ```
//!
//! **Ein dritter Durchgang entsteht nicht, und ein einziger genuegte nicht.**
//! Eine Schleife, die je Profil beide Muster prueft, waere kuerzer und
//! antwortete anders: sie liesse die Dateireihenfolge ueber die Vorrangregel
//! siegen und gaebe einem frueheren Profil mit passender Kennzeichendatei den
//! Ort, den ein spaeteres ueber sein Pfadmuster beansprucht. Die zwei
//! Durchgaenge sind damit nicht die umstaendliche Fassung der einen Schleife,
//! sondern die einzige, die beide Regeln in ihrer Ordnung stehen laesst.
//!
//! # Warum C2.3 daraus folgt und nicht danebensteht
//!
//! C2.3 verlangt, dass ein Pfadmuster eines **spaeteren** Profils die
//! Kennzeichendatei eines **frueheren** schlaegt. Das ist keine Ausnahme, die
//! hier als dritter Fall zu pruefen waere, sondern genau das, was die zwei
//! Durchgaenge tun: der erste Durchgang ist ganz vorbei, bevor der zweite
//! beginnt, also kommt keine Kennzeichendatei zum Zug, solange irgendein
//! Pfadmuster getroffen hat. Eine eigene Regel dafuer waere ein vierter Fall
//! neben drei vollstaendigen und muesste gegen sie abgegrenzt werden.
//!
//! Denselben Weg geht das mitgelieferte Profil des einzelnen Circles: es
//! erkennt seinen Ort allein ueber die Kennzeichendatei `^_._circle\.md$`,
//! faellt also in den zweiten Durchgang und kann dort von keinem Pfadmuster
//! mehr ueberholt werden, weil keines der mitgelieferten auf ein
//! Circle-Verzeichnis passt (C2.4, C5.7).
//!
//! # Warum die Eintraege als Abschluss hereinkommen
//!
//! Der erste Durchgang kostet keinen Systemaufruf: er sieht auf einen Pfad,
//! den der Aufrufer ohnehin haelt. Der zweite braucht die Namen der Eintraege
//! und damit einen Verzeichnisleselauf. Kaemen sie als Feld herein, faende
//! dieser Lauf **vor** der Frage statt, ob ihn jemand braucht, und jeder
//! Ordner mit Pfadmustertreffer zahlte ihn umsonst.
//!
//! Als Abschluss faellt er dort an, wo er gebraucht wird, naemlich beim ersten
//! Profil mit Kennzeichendatei, und die Zahlen aus C6.7 fallen aus dieser
//! Bauart. [`erkennen`] ruft den Abschluss deshalb **hoechstens einmal** und
//! merkt sich seine Antwort fuer die uebrigen Profile des zweiten Durchgangs;
//! ob der Aufrufer sie daneben ein zweites Mal aufhebt, ist seine Sache und
//! nicht die dieser Stelle.
//!
//! **`None` heisst „die Eintraege stehen nicht zur Verfuegung"** und nicht „der
//! Ordner ist leer": ein leerer Ordner liefert einen leeren Ausschnitt. Ohne
//! Eintraege kann keine Kennzeichendatei treffen, also endet der zweite
//! Durchgang dann ohne Profil — unentschieden und nicht negativ entschieden,
//! derselbe Rueckgriff, den `verzeichnis::sys::ist_deskriptormangel` seit der
//! Runde 10 im Durchlauf traegt.
//!
//! # Der Pfad als Text
//!
//! Das Pfadmuster laeuft gegen [`Path::to_string_lossy`], also gegen dieselbe
//! verlustbehaftete Umschrift, die `verzeichnis::sys` beim Lesen auf jeden
//! Eintragsnamen anwendet (`String::from_utf8_lossy`). Beide Haelften der
//! Erkennung sprechen damit eine Form, und ein Ordner mit einem Namen, der
//! kein UTF-8 ist, bleibt erkennbar, statt die Erkennung anzuhalten.

use std::path::Path;

use crate::verzeichnis::Eintrag;

use super::{Profil, Profile};

/// Sucht das Profil eines ausgewaehlten Ordners, in zwei Durchgaengen.
///
/// `voller_pfad` ist der Pfad des Ordners, wie ihn der Nutzer ausgewaehlt hat.
/// `eintraege` liefert die Namen darin und wird **hoechstens einmal** gerufen,
/// naemlich beim ersten Profil mit Kennzeichendatei; `None` heisst, dass die
/// Eintraege nicht zur Verfuegung stehen. Der Modulkopf schreibt beides aus.
///
/// `None` als Rueckgabe heisst: kein Profil greift, und die Vorschau zeigt die
/// heutige Metadatenanzeige (C2.5). Das ist derselbe Zweig, den sie ohne diese
/// Runde ohnehin genommen haette.
#[must_use = "wer das erkannte Profil fallen laesst, hat den Ordner umsonst gelesen \
              und zeigt dem Nutzer Metadaten, obwohl ein Profil greift"]
pub fn erkennen<'p, 'e>(
    profile: &'p Profile,
    voller_pfad: &Path,
    eintraege: &dyn Fn() -> Option<&'e [Eintrag]>,
) -> Option<&'p Profil> {
    let pfadtext = voller_pfad.to_string_lossy();
    for profil in profile.iter() {
        if profil
            .pfad()
            .is_some_and(|muster| muster.is_match(&pfadtext))
        {
            return Some(profil);
        }
    }

    // Erst hier faellt der Verzeichnisleselauf an, und nur, wenn ueberhaupt ein
    // Profil eine Kennzeichendatei nennt.
    let mut vorrat: Option<Option<&[Eintrag]>> = None;
    for profil in profile.iter() {
        let Some(kennzeichen) = profil.kennzeichen() else {
            continue;
        };
        let Some(gelesen) = *vorrat.get_or_insert_with(eintraege) else {
            // Ohne Eintraege trifft keine Kennzeichendatei, auch keine der
            // uebrigen Profile.
            return None;
        };
        if gelesen
            .iter()
            .any(|eintrag| kennzeichen.is_match(&eintrag.name))
        {
            return Some(profil);
        }
    }

    None
}
