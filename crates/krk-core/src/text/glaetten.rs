//! Strip und Format der Quicknote: aus einem aus dem Chat kopierten Text wird
//! ein lauffaehiger Befehl ([`befehl`]) oder Fliesstext mit Absaetzen und
//! Aufzaehlungspunkten auf je einer Zeile ([`fliesstext`]).
//!
//! ```text
//!  eingefuegter Text ──┬──> befehl()     Strip:  Kopiereinrueckung, `\`-Fortsetzung
//!                      │                         und Terminalumbruch fallen, was fuer
//!                      │                         die Shell Inhalt ist, bleibt
//!                      │
//!                      └──> fliesstext() Format: Terminalumbrueche fallen,
//!                                                Ueberschriften und Punkte bleiben
//!                                 │
//!            beide fragen ────────┴──> umgebrochen(zeile, folgezeile, breite)
//! ```
//!
//! # Die eine Frage, und warum sie geschaetzt wird
//!
//! Beide Umformungen haengen an derselben Frage: war ein Zeilenumbruch im
//! eingefuegten Text einer des Terminals oder ein gewollter? **Aus dem Text
//! allein ist sie nicht entscheidbar**, denn die Breite des Terminals reist
//! nicht mit. Beantwortet wird sie deshalb an genau einer Stelle,
//! `umgebrochen`, mit drei Bedingungen:
//!
//! - **Breite** `B` ist die laengste Zeile der Eingabe in Zeichen
//!   (Unicode-Skalarwerte, Einrueckung mitgezaehlt; nachlaufender Leerraum
//!   faellt vorher). Sie gilt erst als **belegt**, wenn mindestens
//!   [`MINDESTZAHL_VOLLER_ZEILEN`] Zeilen sie bis auf den Nachlass erreichen
//!   (`laenge >= B - B / NACHLASS_TEILER`), und unter [`MINDESTBREITE`] nie.
//!   Ohne belegte Breite wird nichts verbunden.
//! - Die Folgezeile ist nicht leer und mit mindestens einem Leerzeichen
//!   eingerueckt.
//! - Das erste Wort der Folgezeile haette nicht mehr in die Zeile gepasst:
//!   `laenge(zeile) + 1 + laenge(wort) > B - B / NACHLASS_TEILER`.
//!
//! **Warum die Breite belegt sein muss.** Bei Text aus dem Chat ist die
//! Einrueckung kein Signal, denn der Chat rueckt jede Zeile nach der ersten
//! ein. Bliebe allein die Laenge, erfuellte die laengste Zeile die dritte
//! Bedingung fuer jedes Folgewort, denn sie ist die Breite: ein Kommentar ueber
//! einem Befehl verschluckte den Befehl, und von zwei Befehlen untereinander
//! wuerde der laengere mit dem naechsten verbunden. Ein Umbruch des Terminals
//! laesst dagegen eine Spur, die eine einzelne lange Zeile nicht hat: er
//! erzeugt mehrere Zeilen nahe derselben Breite. Erst diese Spur belegt die
//! Breite. Der Preis: ein Text, den das Terminal genau einmal umgebrochen hat,
//! hat nur eine volle Zeile und bleibt, wie er ist.
//!
//! Ein Text ohne Einrueckung wird so nie verbunden, und ein schon geglaetteter
//! auch nicht; beide Umformungen sind dadurch idempotent. Jede Umformung ist in
//! der Quicknote eine Handlung, die `cmd+z` zuruecknimmt; die Schaetzung darf
//! sich deshalb irren, solange sie es selten und in der Richtung tut, die
//! nichts verbindet.
//!
//! # Was nicht entscheidbar ist und deshalb hier steht
//!
//! - **Ein einmal umgebrochener Text** (eine volle Zeile und ihr Rest) wird
//!   nicht verbunden, denn eine einzelne lange Zeile belegt keine Breite. Ein
//!   so umgebrochener Befehl bleibt nach Strip auf zwei Zeilen.
//! - **Zwei getrennte Zeilen, die beide fast die volle Breite einnehmen**,
//!   belegen sie, und die erste wird mit der zweiten verbunden, wenn die zweite
//!   eingerueckt ist. In Strip gilt das nicht nach einer Kommentarzeile (siehe
//!   dort).
//! - **Ein Wort, das breiter ist als die Zeile** und das der Chat mitten im Wort
//!   trennt (ein sehr langer Pfad), bekommt beim Verbinden ein Leerzeichen zu
//!   viel.
//! - **Breite Zeichen** (CJK, Emoji) zaehlen als eines; die Regel schaetzt die
//!   Breite dann zu klein und verbindet eher seltener.
//!
//! # Format
//!
//! Eine Leerzeile bleibt eine und beendet den laufenden Absatz oder Punkt. Ein
//! **Aufzaehlungspunkt** ist eine Zeile, deren Text nach der Einrueckung mit
//! `- `, `* `, `• `, `<Ziffern>. ` oder `<Ziffern>) ` beginnt; er beginnt immer
//! eine eigene Zeile und steht danach mit einem Leerzeichen davor, ein tiefer
//! eingerueckter Punkt mit seinem Abstand zum flachsten Punkt des Textes
//! zusaetzlich. Jede andere Zeile steht ohne Einrueckung da, ausser sie wird
//! verbunden: das wird sie, wenn sie kein Punkt ist, `umgebrochen` gilt und
//! ihre Einrueckung die Textspalte des laufenden Eintrags erreicht (die erste
//! Zeile eines Absatzes, die Spalte hinter Zeichen und Leerzeichen eines
//! Punkts). **Ueberschriften brauchen keine eigene Regel**: eine kurze Zeile ist
//! nie umgebrochen, also beginnt die Zeile danach neu. **Eine Geruestzeile**
//! (eine Zeile mit Kasten- oder Blockzeichen von `U+2500` bis `U+259F`, also
//! Tabellen in Kastenzeichen und Baumskizzen mit `├──`, oder eine, deren Text
//! mit `|` beginnt wie eine Markdown-Tabelle, oder eine aus `+`, `-`, `=`, `|`
//! und `:` wie der Rahmen einer ASCII-Tabelle) wird nie verbunden, weder an die
//! Zeile davor noch mit der danach: in einer Tabelle sind alle Zeilen gleich
//! lang und belegen die Breite, also ist sie der eine Fall, den die Breite
//! nicht trennt. Sie verliert allein die kleinste Einrueckung der Geruestzeilen
//! ab der zweiten Zeile, die erste Zeile der Eingabe ihre ganze; so bleibt die
//! Ausrichtung der Skizze stehen. Zeilen in einem
//! Code-Zaun (drei Backticks bis zu den naechsten) bleiben woertlich und
//! verlieren allein die Einrueckung der oeffnenden Zaunzeile. Ein Schlussumbruch
//! der Eingabe bleibt.
//!
//! # Strip
//!
//! Nachlaufender Leerraum faellt in jeder Zeile ausser im Heredoc-Rumpf und am
//! Ende einer Zeile in offener Anfuehrung (siehe unten), Leerzeilen am Anfang
//! und am Ende fallen, und die Ausgabe endet ohne `\n`, damit ein Einfuegen ins
//! Terminal nicht sofort ausfuehrt. Die erste Zeile verliert ihren ganzen
//! fuehrenden Leerraum, jede weitere die kleinste Einrueckung der nicht leeren
//! Zeilen ab der zweiten (die Kopiereinrueckung); mehr Einrueckung bleibt.
//!
//! Ein kleiner Automat liest die Lage der Shell je Zeichen: ausserhalb,
//! `'…'`, `"…"`, `$'…'`, ein Kommentar ab `#` am Wortanfang und Heredocs
//! (`<<WORT`, `<<-WORT`, `<<'WORT'`, `<<"WORT"`, mehrere je Zeile; `<<<` ist
//! keiner, und `<<` in einer Arithmetik `$(( … ))` oder `(( … ))` auch nicht,
//! solange sie auf derselben Zeile schliesst). Endet eine Zeile in einem
//! offenen Anfuehrungszeichen oder in einem Heredoc-Rumpf, bleibt ihr Umbruch,
//! denn er ist Inhalt, und mit ihm ihr nachlaufender Leerraum, etwa die zwei
//! Leerzeichen eines harten Umbruchs in Markdown; die Endzeile eines Heredocs
//! verliert ihn, damit sie ihn beendet. Ob der Chat nachlaufenden Leerraum
//! hinzufuegt, ist nicht entscheidbar; gekuerzt wird deshalb allein dort, wo er
//! fuer die Shell keine Bedeutung hat, und vor allem hinter einem `\`. **Endet eine Zeile in
//! einem Kommentar, bleibt ihr Umbruch immer**: was danach steht, waere sonst
//! Teil des Kommentars und liefe nicht. Ein Kommentar, den der Chat umbrochen
//! hat, bleibt dafuer auf mehreren Zeilen, und die Shell liest jede weitere als
//! Befehl; Strip laesst den Text damit sichtbar unfertig, statt still einen
//! Befehl zu verschlucken. Endet sie ausserhalb
//! auf ein unmaskiertes `\`, fallen `\` und Umbruch, wie die Shell sie
//! entfernt; stand auf einer Seite Leerraum, steht dazwischen genau ein
//! Leerzeichen, sonst keins. Sonst wird die Folgezeile angehaengt, wenn
//! `umgebrochen` gilt, und bleibt andernfalls auf eigener Zeile, denn ein
//! Umbruch ist fuer die Shell ein `;`.
//!
//! Kein Systemaufruf, kein Dateisystem, keine Kiste: Rechnung auf einer
//! Zeichenkette wie der Rest dieses Verzeichnisses.

use std::collections::VecDeque;

/// Unter dieser Breite der laengsten Zeile wird nichts verbunden: ein so
/// schmaler Text kommt aus keinem Terminalumbruch des Chats.
pub const MINDESTBREITE: usize = 40;

/// So viele Zeilen muessen die laengste bis auf den Nachlass erreichen, bevor
/// ihre Laenge als Breite eines Terminalumbruchs gilt: ein Umbruch erzeugt
/// mehrere fast volle Zeilen, eine einzelne lange Zeile belegt keinen.
pub const MINDESTZAHL_VOLLER_ZEILEN: usize = 2;

/// Der Nachlass auf die Breite ist `B / NACHLASS_TEILER`, ein Zehntel. Ohne ihn
/// bliebe im Beispiel des Nutzers die erste Zeile unverbunden, deren Folgewort
/// gerade noch in die gemessene laengste Zeile gepasst haette.
const NACHLASS_TEILER: usize = 10;

/// Format: macht aus terminal-umbrochenem Fliesstext Absaetze und
/// Aufzaehlungspunkte auf je einer Zeile. Die Regeln stehen im Modulkopf.
#[must_use]
pub fn fliesstext(eingabe: &str) -> String {
    let (zeilen, schluss) = zerlegen(eingabe);
    let zeilen: Vec<String> = zeilen
        .iter()
        .map(|zeile| zeile.trim_end().to_owned())
        .collect();
    let breite = belegte_breite(&zeilen);
    let geruestabzug = zeilen
        .iter()
        .skip(1)
        .filter(|zeile| ist_geruest(text_von(zeile)))
        .map(|zeile| einrueckung(zeile))
        .min()
        .unwrap_or(0);
    let flachster_punkt = zeilen
        .iter()
        .filter(|zeile| punktzeichen(text_von(zeile)).is_some())
        .map(|zeile| einrueckung(zeile))
        .min()
        .unwrap_or(0);

    let mut aus: Vec<String> = Vec::with_capacity(zeilen.len());
    // Die laufende Zeile, an die verbunden werden darf, und die Textspalte
    // ihres Eintrags; `None` nach einer Leerzeile, in einem Zaun und am Anfang.
    let mut laufend: Option<(&str, usize)> = None;
    // Die Einrueckung der oeffnenden Zaunzeile, solange ein Zaun offen ist.
    let mut zaun: Option<usize> = None;

    for (nummer, zeile) in zeilen.iter().enumerate() {
        let zeile = zeile.as_str();
        let text = text_von(zeile);
        if let Some(abzug) = zaun {
            aus.push(zeile[abzug.min(einrueckung(zeile))..].to_owned());
            if ist_zaun(text) {
                zaun = None;
            }
            continue;
        }
        if text.is_empty() {
            aus.push(String::new());
            laufend = None;
            continue;
        }
        if ist_zaun(text) {
            zaun = Some(einrueckung(zeile));
            aus.push(text.to_owned());
            laufend = None;
            continue;
        }
        if ist_geruest(text) {
            let abzug = if nummer == 0 {
                einrueckung(zeile)
            } else {
                geruestabzug.min(einrueckung(zeile))
            };
            aus.push(zeile[abzug..].to_owned());
            laufend = None;
            continue;
        }
        if let Some(zeichen) = punktzeichen(text) {
            let tiefe = einrueckung(zeile);
            aus.push(format!("{}{text}", " ".repeat(1 + tiefe - flachster_punkt)));
            laufend = Some((zeile, tiefe + zeichen));
            continue;
        }
        if let Some((vorige, spalte)) = laufend
            && einrueckung(zeile) >= spalte
            && umgebrochen(vorige, zeile, breite)
            && let Some(letzte) = aus.last_mut()
        {
            letzte.push(' ');
            letzte.push_str(text);
            laufend = Some((zeile, spalte));
            continue;
        }
        aus.push(text.to_owned());
        laufend = Some((zeile, einrueckung(zeile)));
    }

    let mut ergebnis = aus.join("\n");
    if schluss {
        ergebnis.push('\n');
    }
    ergebnis
}

/// Strip: macht aus einem aus dem Chat kopierten Shell-Befehl einen, der im
/// Terminal laeuft. Die Regeln stehen im Modulkopf.
#[must_use]
pub fn befehl(eingabe: &str) -> String {
    let (roh_alle, _) = zerlegen(eingabe);
    // Gekuerzt fuer die Lage, die Breite und die Frage nach leeren Zeilen; roh,
    // wo der nachlaufende Leerraum Inhalt ist (Modulkopf).
    let alle: Vec<String> = roh_alle
        .iter()
        .map(|zeile| zeile.trim_end().to_owned())
        .collect();
    let Some(anfang) = alle.iter().position(|zeile| !zeile.is_empty()) else {
        return String::new();
    };
    let ende = alle
        .iter()
        .rposition(|zeile| !zeile.is_empty())
        .map_or(anfang, |stelle| stelle + 1);
    let zeilen = &alle[anfang..ende];
    let breite = belegte_breite(zeilen);
    let kopiereinrueckung = zeilen[1..]
        .iter()
        .filter(|zeile| !zeile.is_empty())
        .map(|zeile| einrueckung(zeile))
        .min()
        .unwrap_or(0);
    let bereinigt: Vec<&str> = zeilen
        .iter()
        .enumerate()
        .map(|(nummer, zeile)| ohne_kopiereinrueckung(nummer, zeile, kopiereinrueckung))
        .collect();
    let roh: Vec<&str> = roh_alle[anfang..ende]
        .iter()
        .enumerate()
        .map(|(nummer, zeile)| ohne_kopiereinrueckung(nummer, zeile, kopiereinrueckung))
        .collect();

    let mut aus = String::new();
    let mut lage = Lage::Normal;
    let mut wartend: VecDeque<Heredoc> = VecDeque::new();
    // Ob die naechste Zeile ohne ihren fuehrenden Leerraum angehaengt wird.
    let mut naechste_kuerzen = false;

    for (nummer, (zeile, roh)) in bereinigt.iter().zip(&roh).enumerate() {
        let (zeile, roh) = if naechste_kuerzen {
            (zeile.trim_start(), roh.trim_start())
        } else {
            (*zeile, *roh)
        };
        naechste_kuerzen = false;
        let Some(folgezeile) = bereinigt.get(nummer + 1) else {
            // Die Ausgabe endet ohne nachlaufenden Leerraum (Modulkopf).
            aus.push_str(zeile);
            break;
        };

        // Ob der nachlaufende Leerraum dieser Zeile Inhalt ist: im Rumpf eines
        // Heredocs und am Ende in einer offenen Anfuehrung.
        let (trenner, leerraum_bleibt) = match &lage {
            Lage::Heredoc(heredoc) => {
                // Die Endzeile eines Heredocs steht immer auf eigener Zeile und
                // ohne nachlaufenden Leerraum, damit sie ihn beendet; der Rumpf
                // ist Inhalt.
                if heredoc.endet_mit(zeile) {
                    lage = wartend.pop_front().map_or(Lage::Normal, Lage::Heredoc);
                    (Trenner::Umbruch, false)
                } else {
                    (Trenner::Umbruch, true)
                }
            }
            Lage::Normal | Lage::Einfach | Lage::Doppelt | Lage::Ansi => {
                let ende = zeile_lesen(zeile, &mut lage, &mut wartend);
                if let Some(heredoc) = wartend.pop_front() {
                    lage = Lage::Heredoc(heredoc);
                    (Trenner::Umbruch, false)
                } else if !matches!(lage, Lage::Normal) {
                    (Trenner::Umbruch, true)
                } else if ende == Zeilenende::Fortsetzung {
                    (Trenner::Fortsetzung, false)
                } else if ende == Zeilenende::Kommentar {
                    // Was angehaengt wuerde, stuende im Kommentar und liefe nicht.
                    (Trenner::Umbruch, false)
                } else if umgebrochen(&zeilen[nummer], &zeilen[nummer + 1], breite) {
                    (Trenner::Verbinden, false)
                } else {
                    (Trenner::Umbruch, false)
                }
            }
        };
        aus.push_str(if leerraum_bleibt { roh } else { zeile });

        match trenner {
            Trenner::Umbruch => aus.push('\n'),
            Trenner::Verbinden => {
                aus.push(' ');
                naechste_kuerzen = true;
            }
            Trenner::Fortsetzung => {
                aus.pop();
                let vorher_leer = aus.ends_with([' ', '\t']);
                let nachher_leer = folgezeile.starts_with([' ', '\t']);
                if vorher_leer || nachher_leer {
                    while aus.ends_with([' ', '\t']) {
                        aus.pop();
                    }
                    aus.push(' ');
                    naechste_kuerzen = true;
                }
            }
        }
    }
    aus
}

/// Die Zeile `nummer` ohne die Kopiereinrueckung: die erste verliert ihren
/// ganzen fuehrenden Leerraum, jede weitere hoechstens `kopiereinrueckung`
/// Leerzeichen.
fn ohne_kopiereinrueckung(nummer: usize, zeile: &str, kopiereinrueckung: usize) -> &str {
    if nummer == 0 {
        zeile.trim_start()
    } else {
        &zeile[kopiereinrueckung.min(einrueckung(zeile))..]
    }
}

/// Was zwischen einer Zeile und ihrer Folgezeile in der Ausgabe von
/// [`befehl`] steht.
enum Trenner {
    /// Der Umbruch bleibt.
    Umbruch,
    /// Ein Terminalumbruch: genau ein Leerzeichen.
    Verbinden,
    /// `\` und Umbruch fallen, wie die Shell sie entfernt.
    Fortsetzung,
}

/// Wie eine Zeile ausserhalb jeder Anfuehrung endet, gelesen von
/// [`zeile_lesen`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Zeilenende {
    /// Auf etwas anderes als die zwei Faelle darunter.
    Gewoehnlich,
    /// Auf ein unmaskiertes `\`.
    Fortsetzung,
    /// In einem Kommentar ab `#` am Wortanfang.
    Kommentar,
}

/// Die Lage der Shell am Ende einer gelesenen Zeile.
enum Lage {
    Normal,
    /// In `'…'`.
    Einfach,
    /// In `"…"`, mit `\` als Maskierung.
    Doppelt,
    /// In `$'…'`, mit `\` als Maskierung.
    Ansi,
    /// Im Rumpf eines Heredocs.
    Heredoc(Heredoc),
}

/// Ein Heredoc, dessen Rumpf mit der naechsten Zeile beginnt oder laeuft.
struct Heredoc {
    /// Das Wort, das allein auf einer Zeile den Rumpf beendet.
    ende: String,
    /// `<<-`: fuehrende Tabulatoren der Endzeile zaehlen nicht.
    tabulatoren: bool,
}

impl Heredoc {
    fn endet_mit(&self, zeile: &str) -> bool {
        let gepruefte = if self.tabulatoren {
            zeile.trim_start_matches('\t')
        } else {
            zeile
        };
        gepruefte == self.ende
    }
}

/// Liest eine Zeile ausserhalb eines Heredoc-Rumpfs Zeichen fuer Zeichen,
/// fuehrt `lage` fort und haengt jeden in ihr geoeffneten Heredoc an
/// `wartend`. Antwortet, ob die Zeile ausserhalb jeder Anfuehrung auf ein
/// unmaskiertes `\` oder in einem Kommentar endet.
fn zeile_lesen(zeile: &str, lage: &mut Lage, wartend: &mut VecDeque<Heredoc>) -> Zeilenende {
    let zeichen: Vec<char> = zeile.chars().collect();
    let mut stelle = 0;
    let mut nach_dollar = false;
    // Offene Klammern einer Arithmetik `$(( … ))` oder `(( … ))`; darin ist
    // `<<` eine Verschiebung und kein Heredoc. Gezaehlt wird je Zeile.
    let mut arithmetik: usize = 0;
    while stelle < zeichen.len() {
        let jetzt = zeichen[stelle];
        match lage {
            Lage::Normal => {
                let dollar_davor = nach_dollar;
                nach_dollar = jetzt == '$';
                match jetzt {
                    '\\' => {
                        if stelle + 1 == zeichen.len() {
                            return Zeilenende::Fortsetzung;
                        }
                        stelle += 1;
                        nach_dollar = false;
                    }
                    '\'' if dollar_davor => *lage = Lage::Ansi,
                    '\'' => *lage = Lage::Einfach,
                    '"' => *lage = Lage::Doppelt,
                    '#' if am_wortanfang(&zeichen, stelle) => return Zeilenende::Kommentar,
                    '$' if zeichen.get(stelle + 1) == Some(&'(')
                        && zeichen.get(stelle + 2) == Some(&'(') =>
                    {
                        arithmetik += 2;
                        stelle += 2;
                        nach_dollar = false;
                    }
                    '(' if arithmetik > 0 => arithmetik += 1,
                    ')' if arithmetik > 0 => arithmetik -= 1,
                    '(' if zeichen.get(stelle + 1) == Some(&'(')
                        && am_wortanfang(&zeichen, stelle) =>
                    {
                        arithmetik = 2;
                        stelle += 1;
                    }
                    '<' if arithmetik == 0 && zeichen.get(stelle + 1) == Some(&'<') => {
                        stelle = heredoc_lesen(&zeichen, stelle, wartend);
                        nach_dollar = false;
                        continue;
                    }
                    _ => {}
                }
            }
            Lage::Einfach => {
                if jetzt == '\'' {
                    *lage = Lage::Normal;
                }
            }
            Lage::Doppelt | Lage::Ansi => {
                let schluss = if matches!(lage, Lage::Doppelt) {
                    '"'
                } else {
                    '\''
                };
                if jetzt == '\\' {
                    stelle += 1;
                } else if jetzt == schluss {
                    *lage = Lage::Normal;
                }
            }
            Lage::Heredoc(_) => return Zeilenende::Gewoehnlich,
        }
        stelle += 1;
    }
    Zeilenende::Gewoehnlich
}

/// Ob ein `#` an `stelle` ein Wort beginnt und damit einen Kommentar.
fn am_wortanfang(zeichen: &[char], stelle: usize) -> bool {
    stelle == 0
        || zeichen[stelle - 1].is_whitespace()
        || matches!(zeichen[stelle - 1], ';' | '&' | '|' | '(' | ')' | '<' | '>')
}

/// Liest ab `stelle`, an der `<<` steht, einen Heredoc-Anfang und antwortet mit
/// der Stelle, an der das Lesen weitergeht. `<<<` ist ein Here-String und
/// oeffnet keinen; ein Anfang ohne Wort oeffnet ebenso keinen.
fn heredoc_lesen(zeichen: &[char], stelle: usize, wartend: &mut VecDeque<Heredoc>) -> usize {
    if zeichen.get(stelle + 2) == Some(&'<') {
        return stelle + 3;
    }
    let mut weiter = stelle + 2;
    let tabulatoren = zeichen.get(weiter) == Some(&'-');
    if tabulatoren {
        weiter += 1;
    }
    while zeichen.get(weiter).is_some_and(|z| *z == ' ' || *z == '\t') {
        weiter += 1;
    }
    let mut ende = String::new();
    match zeichen.get(weiter) {
        Some(&anfuehrung) if anfuehrung == '\'' || anfuehrung == '"' => {
            let Some(laenge) = zeichen[weiter + 1..].iter().position(|z| *z == anfuehrung) else {
                // Ohne schliessendes Zeichen ist es kein Heredoc-Wort; das
                // Anfuehrungszeichen liest der Automat dann selbst.
                return weiter;
            };
            ende.extend(&zeichen[weiter + 1..weiter + 1 + laenge]);
            weiter += laenge + 2;
        }
        _ => {
            while let Some(&jetzt) = zeichen.get(weiter) {
                if jetzt.is_whitespace() || matches!(jetzt, ';' | '&' | '|' | '<' | '>' | '(' | ')')
                {
                    break;
                }
                if !matches!(jetzt, '\\' | '\'' | '"') {
                    ende.push(jetzt);
                }
                weiter += 1;
            }
        }
    }
    if !ende.is_empty() {
        wartend.push_back(Heredoc { ende, tabulatoren });
    }
    weiter
}

/// Die eine Umbruchregel, die [`fliesstext`] und [`befehl`] teilen; der
/// Modulkopf schreibt sie aus.
fn umgebrochen(zeile: &str, folgezeile: &str, breite: usize) -> bool {
    if breite < MINDESTBREITE {
        return false;
    }
    let text = folgezeile.trim_start_matches(' ');
    if text.is_empty() || text.len() == folgezeile.len() {
        return false;
    }
    let wort = text.split(' ').next().unwrap_or_default();
    laenge(zeile) + 1 + laenge(wort) > breite - breite / NACHLASS_TEILER
}

/// Zerlegt in Zeilen mit `\r\n` und `\r` als `\n`, und sagt, ob die
/// Eingabe auf einen Umbruch endete. Den nachlaufenden Leerraum nimmt der
/// Rufer, denn ob er Inhalt ist, weiss allein [`befehl`].
fn zerlegen(eingabe: &str) -> (Vec<String>, bool) {
    let text = eingabe.replace("\r\n", "\n").replace('\r', "\n");
    let rumpf = text.strip_suffix('\n');
    let schluss = rumpf.is_some();
    let zeilen = rumpf
        .unwrap_or(&text)
        .split('\n')
        .map(str::to_owned)
        .collect();
    (zeilen, schluss)
}

/// Die laengste Zeile als Breite, wenn sie belegt ist (Modulkopf), sonst 0,
/// womit `umgebrochen` nie gilt.
fn belegte_breite(zeilen: &[String]) -> usize {
    let breite = zeilen.iter().map(|zeile| laenge(zeile)).max().unwrap_or(0);
    let schwelle = breite - breite / NACHLASS_TEILER;
    let volle = zeilen
        .iter()
        .filter(|zeile| laenge(zeile) >= schwelle)
        .count();
    if volle >= MINDESTZAHL_VOLLER_ZEILEN {
        breite
    } else {
        0
    }
}

fn laenge(text: &str) -> usize {
    text.chars().count()
}

/// Fuehrende Leerzeichen; ein Tabulator ist keine Einrueckung des Chats.
fn einrueckung(zeile: &str) -> usize {
    zeile.len() - zeile.trim_start_matches(' ').len()
}

fn text_von(zeile: &str) -> &str {
    zeile.trim_start_matches(' ')
}

/// Eine Zeile einer Tabelle oder Skizze, die Format nie verbindet (Modulkopf).
fn ist_geruest(text: &str) -> bool {
    if text.starts_with('|') {
        return true;
    }
    if text.chars().any(|z| ('\u{2500}'..='\u{259F}').contains(&z)) {
        return true;
    }
    text.starts_with('+')
        && text
            .chars()
            .all(|z| matches!(z, '+' | '-' | '=' | '|' | ':' | ' '))
}

fn ist_zaun(text: &str) -> bool {
    text.starts_with("```")
}

/// Die Laenge des Aufzaehlungszeichens samt Leerzeichen in Zeichen, wenn der
/// Text (nach der Einrueckung) ein Aufzaehlungspunkt ist.
fn punktzeichen(text: &str) -> Option<usize> {
    if ["- ", "* ", "\u{2022} "]
        .iter()
        .any(|zeichen| text.starts_with(zeichen))
    {
        return Some(2);
    }
    let ziffern = text.chars().take_while(char::is_ascii_digit).count();
    let rest = &text[ziffern..];
    (ziffern > 0 && (rest.starts_with(". ") || rest.starts_with(") "))).then_some(ziffern + 2)
}
