//! Die Tabellenhandlungen, je als reine Funktion vom alten Text auf einen
//! [`Trenntextstand`] (E5, E7).
//!
//! # Jede Handlung ersetzt allein Byte-Bereiche beruehrter Saetze
//!
//! Eine Handlung rechnet eine Liste von Ersetzungen am alten Text, jede ein
//! Byte-Bereich und sein neuer Inhalt, und setzt den neuen Text daraus in
//! einem Zug zusammen ([`spleissen`]). Was ausserhalb der Bereiche liegt, wird
//! kopiert und nicht neu geschrieben: der Vorspann, jeder unberuehrte Satz und
//! jedes Zeilenende, das die Handlung nicht verschiebt.
//!
//! - **Eine geaenderte Zelle** schreibt ihren Satz neu, und die anderen Felder
//!   darin als ihre rohen Bytes samt Anfuehrungszeichen; nur das geaenderte
//!   Feld geht durch [`feld_kodieren`].
//! - **Eine Zeilenhandlung** fuegt Saetze ein, entfernt oder vertauscht sie.
//! - **Eine Spaltenhandlung** ersetzt in jedem Satz, dem die Spalte gehoert,
//!   allein die Stelle zwischen zwei Feldern.
//!
//! # Ungleich lange Saetze (E7)
//!
//! Ein Satz mit weniger Feldern als [`Satzindex::spaltenzahl`] hat leere
//! Felder am Ende, und sie werden erst geschrieben, wenn eine Handlung sie
//! braucht. Eine Zelle hinter dem letzten Feld fuellt mit Trennzeichen bis zu
//! sich auf; ein leerer Wert dort aendert nichts. Eine Spalte an Stelle `k`
//! bekommt jeder Satz mit mindestens `k` Feldern, ein kuerzerer bleibt
//! unberuehrt; mit `k` gleich der Spaltenzahl ist das das Anhaengen. Eine neue
//! Zeile traegt so viele Trennzeichen, wie die Spaltenzahl weniger eins angibt.
//!
//! # Das Dateiende gehoert dem Text und nicht dem Satz
//!
//! Endet der Text ohne Zeilenende, bleibt das nach jeder Handlung so, wie bei
//! den Eintraegen des Heimordners: wandert der letzte Satz nach oben, bekommt
//! er ein Zeilenende, und der neue letzte verliert seins; wird der letzte
//! geloescht, verliert der vorige seins. **Die eine Ausnahme ist ein leerer
//! letzter Satz**: ohne Zeilenende waere er kein Satz mehr, und die Tabelle
//! verloere eine Zeile. Er behaelt deshalb sein Zeilenende oder bekommt eins.
//!
//! Ein neues Zeilenende nimmt die Form des ersten im Text, `\r\n` oder `\n`;
//! der gehaltene Stand des Editors traegt allein `\n`, die Vorschau rechnet
//! keine Handlung, und so bleibt die Rechnung auch an Bytes mit `\r\n` richtig.
//!
//! **Was diese Regel nicht haelt:** endet der Text ohne Zeilenende auf ein
//! einzelnes `\r`, das der Leser als Zeichen des letzten Feldes liest, und
//! haengt eine Handlung einen Satz an, wird aus `\r` und dem neuen `\n` ein
//! `\r\n`, und das Feld verliert sein letztes Zeichen. Der gehaltene Stand des
//! Editors kommt so nur zustande, wenn jemand ein einzelnes `\r` ans Dateiende
//! schreibt; eine Probe haelt den Fall nicht.
//!
//! # Was eine Handlung abweist
//!
//! Jede Handlung antwortet `None`, wenn sie nichts aendert: eine Zeile, die es
//! nicht gibt, ein leerer Bereich, ein Wert, der schon dasteht, und jeder
//! Index mit einem unterminierten Anfuehrungszeichen, weil ein angehaengter
//! Satz dort im offenen Feld laege. Das Loeschen der Spalten antwortet
//! zusaetzlich [`LetzteSpalte`], wenn es jede Spalte naehme.

use std::borrow::Cow;
use std::ops::Range;

use super::dialekt::Trennzeichen;
use super::feld::feld_kodieren;
use super::index::{BOM, Satzindex};
use super::{Trenntextstand, Zellbereich};
use crate::heimordner::eintraege::Richtung;

/// Die Antwort des Spaltenloeschens, wenn die Auswahl jede Spalte umfasst: eine
/// Tabelle behaelt mindestens eine Spalte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetzteSpalte;

/// Eine Ersetzung am alten Text.
type Ersetzung = (Range<usize>, String);

/// Setzt den neuen Text aus dem alten und den Ersetzungen zusammen. Die
/// Ersetzungen stehen aufsteigend und ueberlappen nicht.
fn spleissen(text: &str, mut ersetzungen: Vec<Ersetzung>) -> String {
    ersetzungen.sort_by_key(|(bereich, _)| bereich.start);
    let zuwachs: usize = ersetzungen.iter().map(|(_, neu)| neu.len()).sum();
    let mut neu = String::with_capacity(text.len() + zuwachs);
    let mut stelle = 0;
    for (bereich, inhalt) in ersetzungen {
        debug_assert!(bereich.start >= stelle, "Ersetzungen ueberlappen");
        neu.push_str(&text[stelle..bereich.start]);
        neu.push_str(&inhalt);
        stelle = bereich.end;
    }
    neu.push_str(&text[stelle..]);
    neu
}

/// Das Zeilenende, das eine Handlung neu schreibt: die Form des ersten im
/// Text.
fn zeilenende_des_textes<'a>(text: &'a str, index: &Satzindex) -> &'a str {
    let erstes = if index.zeilenzahl() > 0 {
        &text[index.zeilenende(0)]
    } else {
        // Ohne Satz bleibt allein das Zeilenende einer `sep=`-Zeile.
        let vorspann = &text[index.vorspann()];
        if vorspann.ends_with("\r\n") {
            "\r\n"
        } else {
            "\n"
        }
    };
    if erstes == "\r\n" { "\r\n" } else { "\n" }
}

/// Eine leere Zeile ohne Zeilenende: Spaltenzahl weniger eins Trennzeichen.
fn leere_zeile(spaltenzahl: usize, trenn: Trennzeichen) -> String {
    let zeichen = trenn.byte() as char;
    std::iter::repeat_n(zeichen, spaltenzahl.saturating_sub(1)).collect()
}

/// Ob der Text am Ende ein Zeilenende braucht, bevor ein Satz angehaengt wird:
/// er traegt mehr als eine Bytefolgenmarke und endet nicht an `\n`.
fn braucht_abschluss(text: &str) -> bool {
    let ohne_marke = text.strip_prefix(BOM).unwrap_or(text);
    !ohne_marke.is_empty() && !ohne_marke.ends_with('\n')
}

/// Ob der Satz `satz` ohne Zeilenende am Ende des Textes steht.
fn ohne_zeilenende(index: &Satzindex, satz: usize) -> bool {
    index.zeilenende(satz).is_empty()
}

/// Der rohe Bereich jedes Feldes eines Satzes, ohne seinen Wert zu bilden.
fn feldbereiche(text: &str, index: &Satzindex, satz: usize) -> Vec<Range<usize>> {
    index.feldbereiche(text, satz)
}

/// Haelt an, wenn der Index nicht zum Text gehoert, und sagt, ob eine Handlung
/// auf ihm rechnen darf.
fn rechenbar(text: &str, index: &Satzindex) -> bool {
    index.pruefen(text);
    index.unterminiert().is_none()
}

/// Setzt die Zelle in Zeile `zeile`, Spalte `spalte` auf `wert`.
///
/// Liegt die Zelle hinter dem letzten Feld ihres Satzes, fuellt die Handlung
/// mit Trennzeichen bis zu ihr auf; ein leerer Wert dort aendert nichts. Eine
/// Zeile, die es nicht gibt, legt diese Handlung nicht an; das tut
/// [`bereich_einsetzen`].
#[must_use]
pub fn zelle_setzen(
    text: &str,
    index: &Satzindex,
    zeile: usize,
    spalte: usize,
    wert: &str,
) -> Option<Trenntextstand> {
    if zeile >= index.zeilenzahl() {
        return None;
    }
    bereich_einsetzen(text, index, zeile, spalte, &[vec![wert.to_owned()]])
}

/// Setzt einen Block von Werten ab Zeile `zeile`, Spalte `spalte` ein (E12):
/// er ueberschreibt, haengt Zeilen an und fuellt Saetze auf, wo er ueber das
/// Ende reicht, in einer Handlung. Eine kuerzere Zeile des Blocks gilt als mit
/// leeren Werten aufgefuellt; der Block ist ein Rechteck.
///
/// Die Auswahl danach ist der eingesetzte Bereich. `zeile` darf hoechstens die
/// Zeilenzahl sein; dann haengt der Block allein an.
#[must_use]
pub fn bereich_einsetzen(
    text: &str,
    index: &Satzindex,
    zeile: usize,
    spalte: usize,
    block: &[Vec<String>],
) -> Option<Trenntextstand> {
    if !rechenbar(text, index) || zeile > index.zeilenzahl() {
        return None;
    }
    let breite = block.iter().map(Vec::len).max().unwrap_or(0);
    if block.is_empty() || breite == 0 {
        return None;
    }
    let trenn = index.trennzeichen();
    let trennzeichen = (trenn.byte() as char).to_string();
    let ende = zeilenende_des_textes(text, index);
    let zeilenzahl = index.zeilenzahl();
    let neue_zeilen = (zeile + block.len()).saturating_sub(zeilenzahl);

    let mut ersetzungen = Vec::new();
    for (versatz, werte) in block.iter().enumerate() {
        let satz = zeile + versatz;
        if satz >= zeilenzahl {
            break;
        }
        let felder = index.felder(text, satz).unwrap_or_default();
        let wert_an =
            |stelle: usize| -> &str { werte.get(stelle - spalte).map_or("", String::as_str) };
        // Die Feldzahl danach: die vorhandenen, und dahinter so weit, wie der
        // Block einen nicht leeren Wert setzt.
        let mut feldzahl = felder.len();
        for stelle in (spalte..spalte + breite).rev() {
            if stelle < feldzahl {
                break;
            }
            if !wert_an(stelle).is_empty() {
                feldzahl = stelle + 1;
                break;
            }
        }
        let mut geaendert = feldzahl != felder.len();
        let mut teile: Vec<Cow<'_, str>> = Vec::with_capacity(feldzahl);
        for stelle in 0..feldzahl {
            let gesetzt = (spalte..spalte + breite).contains(&stelle);
            match felder.get(stelle) {
                Some(feld) if !gesetzt || feld.wert == wert_an(stelle) => {
                    teile.push(Cow::Borrowed(&text[feld.roh.clone()]));
                }
                Some(_) => {
                    geaendert = true;
                    teile.push(feld_kodieren(wert_an(stelle), trenn));
                }
                None if gesetzt => teile.push(feld_kodieren(wert_an(stelle), trenn)),
                None => teile.push(Cow::Borrowed("")),
            }
        }
        if !geaendert {
            continue;
        }
        let mut inhalt = teile.join(trennzeichen.as_str());
        if inhalt.is_empty() && ohne_zeilenende(index, satz) && neue_zeilen == 0 {
            inhalt.push_str(ende);
        }
        ersetzungen.push((index.saetze()[satz].clone(), inhalt));
    }

    if neue_zeilen > 0 {
        let spaltenzahl = index.spaltenzahl().max(spalte + breite);
        let mut angehaengt = String::new();
        if braucht_abschluss(text) {
            angehaengt.push_str(ende);
        }
        for werte in &block[block.len() - neue_zeilen..] {
            let teile: Vec<Cow<'_, str>> = (0..spaltenzahl)
                .map(|stelle| {
                    if (spalte..spalte + breite).contains(&stelle) {
                        feld_kodieren(werte.get(stelle - spalte).map_or("", String::as_str), trenn)
                    } else {
                        Cow::Borrowed("")
                    }
                })
                .collect();
            angehaengt.push_str(&teile.join(trennzeichen.as_str()));
            angehaengt.push_str(ende);
        }
        ersetzungen.push((text.len()..text.len(), angehaengt));
    }

    if ersetzungen.is_empty() {
        return None;
    }
    Some(Trenntextstand {
        text: spleissen(text, ersetzungen),
        auswahl: Zellbereich {
            zeilen: zeile..zeile + block.len(),
            spalten: spalte..spalte + breite,
        },
    })
}

/// Fuegt ueber der Auswahl so viele leere Zeilen ein, wie sie Zeilen umfasst,
/// mindestens eine. Die Auswahl danach sind die neuen Zeilen in den Spalten
/// der alten Auswahl. Mit der ersten Zeile hinter der letzten haengt die
/// Handlung an.
#[must_use]
pub fn zeilen_einfuegen(
    text: &str,
    index: &Satzindex,
    auswahl: &Zellbereich,
) -> Option<Trenntextstand> {
    if !rechenbar(text, index) {
        return None;
    }
    let vor = auswahl.zeilen.start;
    let anzahl = auswahl.zeilen.len().max(1);
    if vor > index.zeilenzahl() {
        return None;
    }
    let ende = zeilenende_des_textes(text, index);
    let mut zeile = leere_zeile(index.spaltenzahl(), index.trennzeichen());
    zeile.push_str(ende);
    let mut einschub = String::new();
    let stelle = if let Some(satz) = index.saetze().get(vor) {
        satz.start
    } else {
        if braucht_abschluss(text) {
            einschub.push_str(ende);
        }
        text.len()
    };
    for _ in 0..anzahl {
        einschub.push_str(&zeile);
    }
    Some(Trenntextstand {
        text: spleissen(text, vec![(stelle..stelle, einschub)]),
        auswahl: Zellbereich {
            zeilen: vor..vor + anzahl,
            spalten: auswahl.spalten.clone(),
        },
    })
}

/// Haengt eine leere Zeile an; gewaehlt ist danach ihre erste Zelle.
#[must_use]
pub fn zeile_anhaengen(text: &str, index: &Satzindex) -> Option<Trenntextstand> {
    let zeilenzahl = index.zeilenzahl();
    let mut stand = zeilen_einfuegen(text, index, &Zellbereich::zelle(zeilenzahl, 0))?;
    stand.auswahl = Zellbereich::zelle(zeilenzahl, 0);
    Some(stand)
}

/// Loescht die Zeilen der Auswahl. Gewaehlt ist danach die Zeile, die an die
/// Stelle der ersten geloeschten rueckt, oder die letzte, wenn keine nachrueckt,
/// in den Spalten der alten Auswahl.
#[must_use]
pub fn zeilen_loeschen(
    text: &str,
    index: &Satzindex,
    auswahl: &Zellbereich,
) -> Option<Trenntextstand> {
    if !rechenbar(text, index) {
        return None;
    }
    let Range { start, end } = auswahl.zeilen.clone();
    let zeilenzahl = index.zeilenzahl();
    if start >= end || end > zeilenzahl {
        return None;
    }
    let saetze = index.saetze();
    let bereich = if end < zeilenzahl {
        saetze[start].start..saetze[end].start
    } else if start > 0 && ohne_zeilenende(index, zeilenzahl - 1) && !saetze[start - 1].is_empty() {
        // Der letzte Satz geht und trug kein Zeilenende: der vorige gibt seins
        // ab, damit das Dateiende bleibt. Ein leerer voriger behaelt es, sonst
        // waere er kein Satz mehr.
        saetze[start - 1].end..text.len()
    } else {
        saetze[start].start..text.len()
    };
    let uebrig = zeilenzahl - (end - start);
    let gewaehlt = start.min(uebrig.saturating_sub(1));
    Some(Trenntextstand {
        text: spleissen(text, vec![(bereich, String::new())]),
        auswahl: Zellbereich {
            zeilen: if uebrig == 0 {
                0..0
            } else {
                gewaehlt..gewaehlt + 1
            },
            spalten: auswahl.spalten.clone(),
        },
    })
}

/// Verschiebt die Zeilen der Auswahl als Block um eine Stelle. Die Zeilenenden
/// bleiben an ihren Stellen, die Inhalte wandern; die Auswahl wandert mit.
#[must_use]
pub fn zeilen_verschieben(
    text: &str,
    index: &Satzindex,
    auswahl: &Zellbereich,
    richtung: Richtung,
) -> Option<Trenntextstand> {
    if !rechenbar(text, index) {
        return None;
    }
    let Range { start, end } = auswahl.zeilen.clone();
    let zeilenzahl = index.zeilenzahl();
    if start >= end || end > zeilenzahl {
        return None;
    }
    // Das Fenster aus Block und Nachbar, und die neue Folge der Inhalte darin.
    let (fenster, folge, neue_zeilen): (Range<usize>, Vec<usize>, Range<usize>) = match richtung {
        Richtung::Hoch => {
            if start == 0 {
                return None;
            }
            let folge = (start..end).chain(std::iter::once(start - 1)).collect();
            (start - 1..end, folge, start - 1..end - 1)
        }
        Richtung::Runter => {
            if end == zeilenzahl {
                return None;
            }
            let folge = std::iter::once(end).chain(start..end).collect();
            (start..end + 1, folge, start + 1..end + 1)
        }
    };
    let saetze = index.saetze();
    let ende = zeilenende_des_textes(text, index);
    let mut neu = String::new();
    for (platz, &herkunft) in fenster.clone().zip(folge.iter()) {
        let inhalt = &text[saetze[herkunft].clone()];
        neu.push_str(inhalt);
        let zeilenende = &text[index.zeilenende(platz)];
        if zeilenende.is_empty() && inhalt.is_empty() {
            neu.push_str(ende);
        } else {
            neu.push_str(zeilenende);
        }
    }
    let letzter = fenster.end - 1;
    let bereich = saetze[fenster.start].start..index.zeilenende(letzter).end;
    Some(Trenntextstand {
        text: spleissen(text, vec![(bereich, neu)]),
        auswahl: Zellbereich {
            zeilen: neue_zeilen,
            spalten: auswahl.spalten.clone(),
        },
    })
}

/// Fuegt eine leere Spalte an Stelle `an` ein (E7): jeder Satz mit mindestens
/// `an` Feldern bekommt dort ein leeres Feld, ein kuerzerer bleibt unberuehrt.
/// Mit `an` gleich der Spaltenzahl haengt die Handlung die Spalte an.
///
/// Gewaehlt ist danach die neue Spalte in den Zeilen der alten Auswahl.
#[must_use]
pub fn spalte_einfuegen(
    text: &str,
    index: &Satzindex,
    an: usize,
    auswahl: &Zellbereich,
) -> Option<Trenntextstand> {
    if !rechenbar(text, index) || an > index.spaltenzahl() {
        return None;
    }
    let trennzeichen = (index.trennzeichen().byte() as char).to_string();
    let mut ersetzungen = Vec::new();
    for satz in 0..index.zeilenzahl() {
        let felder = feldbereiche(text, index, satz);
        let stelle = match felder.len() {
            n if n > an => felder[an].start,
            n if n == an => index.saetze()[satz].end,
            _ => continue,
        };
        ersetzungen.push((stelle..stelle, trennzeichen.clone()));
    }
    if ersetzungen.is_empty() {
        return None;
    }
    Some(Trenntextstand {
        text: spleissen(text, ersetzungen),
        auswahl: Zellbereich {
            zeilen: auswahl.zeilen.clone(),
            spalten: an..an + 1,
        },
    })
}

/// Loescht die Spalten der Auswahl (E7): jeder Satz, der in diese Spalten
/// reicht, verliert ihre Felder samt einem angrenzenden Trennzeichen, dem
/// folgenden, und reichen sie bis zu seinem letzten Feld, dem vorigen.
///
/// Umfasst die Auswahl jede Spalte, antwortet die Handlung [`LetzteSpalte`]
/// und aendert nichts. Gewaehlt ist danach die Spalte, die an die Stelle der
/// ersten geloeschten rueckt, in den Zeilen der alten Auswahl.
pub fn spalten_loeschen(
    text: &str,
    index: &Satzindex,
    auswahl: &Zellbereich,
) -> Result<Option<Trenntextstand>, LetzteSpalte> {
    if !rechenbar(text, index) {
        return Ok(None);
    }
    let spaltenzahl = index.spaltenzahl();
    let von = auswahl.spalten.start;
    let bis = auswahl.spalten.end.min(spaltenzahl);
    if von >= bis {
        return Ok(None);
    }
    if von == 0 && bis == spaltenzahl {
        return Err(LetzteSpalte);
    }
    let ende = zeilenende_des_textes(text, index);
    let mut ersetzungen = Vec::new();
    for satz in 0..index.zeilenzahl() {
        let felder = feldbereiche(text, index, satz);
        let n = felder.len();
        if n <= von {
            continue;
        }
        let satzbereich = index.saetze()[satz].clone();
        let weg = if bis < n {
            felder[von].start..felder[bis].start
        } else if von > 0 {
            felder[von - 1].end..satzbereich.end
        } else {
            satzbereich.clone()
        };
        // Wird der Satz leer, bleibt er ein Satz, am Ende des Textes also mit
        // einem Zeilenende. Das trifft nicht nur das Loeschen jedes Feldes:
        // auch `,` verliert mit seinem ersten Feld seinen ganzen Inhalt.
        let bleibt = if weg.len() == satzbereich.len() && ohne_zeilenende(index, satz) {
            ende.to_owned()
        } else {
            String::new()
        };
        ersetzungen.push((weg, bleibt));
    }
    if ersetzungen.is_empty() {
        return Ok(None);
    }
    let neue_spaltenzahl = spaltenzahl - (bis - von);
    let gewaehlt = von.min(neue_spaltenzahl - 1);
    Ok(Some(Trenntextstand {
        text: spleissen(text, ersetzungen),
        auswahl: Zellbereich {
            zeilen: auswahl.zeilen.clone(),
            spalten: gewaehlt..gewaehlt + 1,
        },
    }))
}
