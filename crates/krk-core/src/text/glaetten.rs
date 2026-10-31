//! Singlestrip und Format der Quicknote: aus einem aus dem Chat kopierten Text
//! wird ein Befehl auf einer Zeile ([`befehl`]) oder Fliesstext mit Absaetzen
//! und Aufzaehlungspunkten auf je einer Zeile ([`fliesstext`]).
//!
//! ```text
//!  eingefuegter Text ──┬──> befehl()     Singlestrip: Einrueckung, `\`-Fortsetzung
//!                      │                              und jeder Umbruch ausserhalb
//!                      │                              fallen, was fuer die Shell
//!                      │                              Inhalt ist, bleibt
//!                      │
//!                      └──> fliesstext() Format: Terminalumbrueche fallen,
//!                                                Ueberschriften und Punkte bleiben
//!                                 │
//!                                 └──> umgebrochen(zeile, folgezeile, breite)
//! ```
//!
//! # Zwei Umformungen, zwei Annahmen
//!
//! Ob ein Zeilenumbruch im eingefuegten Text einer des Terminals oder ein
//! gewollter war, ist **aus dem Text allein nicht entscheidbar**, denn die
//! Breite des Terminals reist nicht mit. Die zwei Umformungen loesen das
//! verschieden:
//!
//! - **Singlestrip schaetzt nicht, sondern nimmt an: die Eingabe ist ein
//!   einzelner Befehl** und keine Liste von Befehlen untereinander. Dann ist
//!   jeder Umbruch ausserhalb von Anfuehrungen und Heredocs einer, der weg
//!   muss. So vom Nutzer entschieden (`261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`
//!   im Arbeitspaket `261031-1024-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format`).
//!   Bis dahin hiess die Umformung Strip und schaetzte wie Format; jede
//!   Fehlschaetzung aenderte dort die Bedeutung, in beiden Richtungen. Wer
//!   mehrere Befehle untereinander hineinsteckt, bekommt sie auf einer Zeile,
//!   der zweite als Argumente des ersten; das ist der Preis der Annahme, und
//!   `cmd+z` nimmt die Umformung zurueck.
//! - **Format schaetzt**, an genau einer Stelle, `umgebrochen`, denn fuer
//!   Fliesstext ist ein Irrtum Kosmetik. Die Regel steht unter „Format“.
//!
//! # Format
//!
//! `umgebrochen` fragt drei Bedingungen:
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
//! Bedingung fuer jedes Folgewort, denn sie ist die Breite. Ein Umbruch des
//! Terminals laesst dagegen eine Spur, die eine einzelne lange Zeile nicht
//! hat: er erzeugt mehrere Zeilen nahe derselben Breite. Erst diese Spur belegt
//! die Breite. Ein Text ohne Einrueckung wird so nie verbunden, und ein schon
//! formatierter auch nicht; Format ist dadurch idempotent. Jede Umformung ist
//! in der Quicknote eine Handlung, die `cmd+z` zuruecknimmt.
//!
//! **Ein Text, den das Terminal genau einmal umgebrochen hat**, hat nur eine
//! volle Zeile und belegt keine Breite; das trifft vor allem die Auswahl eines
//! einzelnen Absatzes oder Punkts. Seine Fortsetzung wird nicht verbunden,
//! **behaelt aber ihre Einrueckung**: jede Zeile, die mit der laengsten Zeile
//! als Breite verbunden wuerde und es allein mangels belegter Breite nicht
//! wird, steht mit ihrer Einrueckung da. So bleibt sie als Fortsetzung
//! erkennbar, und ein Format ueber einen groesseren Text, der die Breite
//! belegt, verbindet sie noch. Bis zum 261031 verlor sie die Einrueckung und
//! stand danach als eigener Absatz da, den kein Format mehr verband
//! (`261031-1226-ein-einmal-umbrochener-text-bleibt-nicht-wie-er-ist-format-loest-die-fortsetzung-ab.md`).
//!
//! Nicht entscheidbar und deshalb hingenommen:
//!
//! - **Zwei getrennte Zeilen, die beide fast die volle Breite einnehmen**,
//!   belegen sie, und die erste wird mit der zweiten verbunden, wenn die zweite
//!   eingerueckt ist.
//! - **Ein Wort, das breiter ist als die Zeile** und das der Chat mitten im Wort
//!   trennt (ein sehr langer Pfad), bekommt beim Verbinden ein Leerzeichen zu
//!   viel; in Singlestrip ebenso.
//! - **Breite Zeichen** (CJK, Emoji) zaehlen als eines; die Regel schaetzt die
//!   Breite dann zu klein und verbindet eher seltener.
//!
//! Eine Leerzeile bleibt eine und beendet den laufenden Absatz oder Punkt. Ein
//! **Aufzaehlungspunkt** ist eine Zeile, deren Text nach der Einrueckung mit
//! `- `, `* `, `• `, `<Ziffern>. ` oder `<Ziffern>) ` beginnt; er beginnt immer
//! eine eigene Zeile und steht danach mit einem Leerzeichen davor, ein tiefer
//! eingerueckter Punkt mit seinem Abstand zum flachsten Punkt des Textes
//! zusaetzlich. Jede andere Zeile steht ohne Einrueckung da (bis auf die
//! Fortsetzung ohne belegte Breite oben), ausser sie wird verbunden: das wird
//! sie, wenn sie kein Punkt ist, `umgebrochen` gilt und ihre Einrueckung die
//! Textspalte des laufenden Eintrags erreicht (die erste Zeile eines Absatzes,
//! die Spalte hinter Zeichen und Leerzeichen eines Punkts). **Ueberschriften
//! brauchen keine eigene Regel**: eine kurze Zeile ist nie umgebrochen, also
//! beginnt die Zeile danach neu. **Eine Geruestzeile** (eine Zeile mit Kasten-
//! oder Blockzeichen von `U+2500` bis `U+259F`, also Tabellen in Kastenzeichen
//! und Baumskizzen mit `├──`, oder eine, deren Text mit `|` beginnt wie eine
//! Markdown-Tabelle, oder eine aus `+`, `-`, `=`, `|` und `:` wie der Rahmen
//! einer ASCII-Tabelle) wird nie verbunden, weder an die Zeile davor noch mit
//! der danach: in einer Tabelle sind alle Zeilen gleich lang und belegen die
//! Breite, also ist sie der eine Fall, den die Breite nicht trennt. Sie
//! verliert allein die kleinste Einrueckung der Geruestzeilen ab der zweiten
//! Zeile, die erste Zeile der Eingabe ihre ganze; so bleibt die Ausrichtung der
//! Skizze stehen. Zeilen in einem Code-Zaun (drei Backticks bis zu den
//! naechsten) bleiben woertlich und verlieren allein die Einrueckung der
//! oeffnenden Zaunzeile. Ein Schlussumbruch der Eingabe bleibt.
//!
//! # Singlestrip
//!
//! Nachlaufender Leerraum faellt in jeder Zeile ausser im Heredoc-Rumpf und am
//! Ende einer Zeile in offener Anfuehrung (siehe unten), Leerzeilen am Anfang
//! und am Ende fallen, und die Ausgabe endet ohne `\n`, damit ein Einfuegen ins
//! Terminal nicht sofort ausfuehrt. Jede Zeile, die ausserhalb jeder
//! Anfuehrung und jedes Heredoc-Rumpfs beginnt, verliert ihren ganzen
//! fuehrenden Leerraum; eine Zeile in einer Anfuehrung oder einem Rumpf allein
//! die kleinste Einrueckung der nicht leeren Zeilen ab der zweiten (die
//! Kopiereinrueckung), mehr Einrueckung bleibt.
//!
//! Ein kleiner Automat liest die Lage der Shell je Zeichen: ausserhalb,
//! `'…'`, `"…"`, `$'…'`, ein Kommentar ab `#` am Wortanfang und Heredocs
//! (`<<WORT`, `<<-WORT`, `<<'WORT'`, `<<"WORT"`, mehrere je Zeile; `<<<` ist
//! keiner, und `<<` in einer Arithmetik `$(( … ))` oder `(( … ))` auch nicht,
//! solange sie auf derselben Zeile schliesst). Endet eine Zeile in einem
//! offenen Anfuehrungszeichen oder in einem Heredoc-Rumpf, bleibt ihr Umbruch,
//! denn er ist Inhalt, und mit ihm ihr nachlaufender Leerraum, etwa die zwei
//! Leerzeichen eines harten Umbruchs in Markdown; die Endzeile eines Heredocs
//! verliert ihn und steht auf eigener Zeile, damit sie ihn beendet, und ihr
//! Umbruch bleibt ebenso. Ob der Chat nachlaufenden Leerraum hinzufuegt, ist
//! nicht entscheidbar; gekuerzt wird deshalb allein dort, wo er fuer die Shell
//! keine Bedeutung hat, und vor allem hinter einem `\`. Endet eine Zeile
//! ausserhalb auf ein unmaskiertes `\`, fallen `\` und Umbruch, wie die Shell
//! sie entfernt; stand auf einer Seite Leerraum, steht dazwischen genau ein
//! Leerzeichen, sonst keins. **Jeder andere Umbruch ausserhalb wird zu genau
//! einem Leerzeichen**, ohne Breitenschaetzung, und eine Leerzeile ausserhalb
//! faellt dabei mit.
//!
//! **Ein Kommentar faellt heraus**, ab dem `#`, das ausserhalb jeder Anfuehrung
//! und jedes Heredoc-Rumpfs am Wortanfang steht, samt dem Leerraum davor; ein
//! `#` mitten im Wort, in `${#x}` oder `$#` und in einer Adresse mit Anker
//! beginnt keinen. Mit ihm fallen alle Zeilen danach, denn in einem einzelnen
//! Befehl ist ein Kommentar sein Ende oder eine Zeile davor, und ein Umbruch
//! darin stammt vom Terminal. Herausgenommen und nicht stehen gelassen wird er,
//! weil seine Bedeutung an der Shell haengt: die zsh von macOS liest `#` in der
//! interaktiven Eingabe ab Werk nicht als Kommentar (ohne `setopt
//! interactivecomments`), und jedes Wort dahinter, auch jedes aus den
//! angehaengten Folgezeilen, waere dort ein Argument des Befehls davor. Ohne
//! Kommentar liest jede Shell dasselbe. Steht die Kommentarzeile ueber dem
//! Befehl, faellt der Befehl mit, und die Ausgabe ist leer oder traegt allein,
//! was vor dem `#` stand: beim Einfuegen laeuft dann weniger, nie etwas, das
//! niemand als Befehl geschrieben hat, und `cmd+z` holt den Text zurueck. Bis
//! zum 261031 blieb der Kommentar stehen und hielt die Folgezeilen, davor blieb
//! der Umbruch nach ihm stehen
//! (`261031-1344-singlestrip-verlaesst-sich-auf-kommentare-die-zsh-ab-werk-nicht-kennt.md`).
//!
//! **Die eine Ausnahme ist ein Heredoc, den die Zeile vor dem Kommentar
//! oeffnet** (`cat <<EOF > a.txt # legt an`): sein Rumpf beginnt in jeder Shell
//! mit der naechsten Zeile, gleich was der Kommentar sagt, und ist Inhalt.
//! Dort faellt allein der Kommentar bis zum Ende seiner Zeile, und Rumpf und
//! Endzeile bleiben wie jeder andere Rumpf. Die sichere Wahl, denn den Rumpf
//! mitzunehmen haette den Befehl ohne seine Eingabe gelassen, und die Shell
//! wartete auf eine Endzeile, die nicht kommt; ob die Zeilen danach der
//! umbrochene Rest des Kommentars waren, ist nicht entscheidbar, und die Shell
//! liest sie als Rumpf, also auch Singlestrip.
//!
//! Singlestrip ist idempotent bis auf einen Fall, der nicht entscheidbar ist:
//! traegt nach dem ersten Lauf jede Zeile ab der zweiten noch Einrueckung,
//! etwa weil dort allein eine Zeile in Anfuehrung steht, die tiefer
//! eingerueckt war als die Kopiereinrueckung, und keine Endzeile eines Heredocs,
//! dann liest ein zweiter Lauf diese Einrueckung als Kopiereinrueckung und
//! nimmt sie auch.
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
    // Die laengste Zeile, auch wenn sie keine Breite belegt: an ihr misst sich,
    // ob eine Zeile allein mangels belegter Breite unverbunden bleibt.
    let laengste = zeilen.iter().map(|zeile| laenge(zeile)).max().unwrap_or(0);
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
        if let Some((vorige, spalte)) = laufend
            && breite == 0
            && einrueckung(zeile) >= spalte
            && umgebrochen(vorige, zeile, laengste)
        {
            // Eine Fortsetzung, die allein mangels belegter Breite nicht
            // verbunden wird, behaelt ihre Einrueckung: so bleibt sie eine
            // Fortsetzung, und ein spaeteres Format ueber einen Text, der die
            // Breite belegt, verbindet sie noch (Modulkopf).
            aus.push(zeile.to_owned());
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

/// Singlestrip: macht aus einem aus dem Chat kopierten, umbrochenen Shell-Befehl
/// eine Zeile, die im Terminal laeuft. Die Eingabe gilt als **ein** Befehl; die
/// Regeln stehen im Modulkopf.
#[must_use]
pub fn befehl(eingabe: &str) -> String {
    let (roh_alle, _) = zerlegen(eingabe);
    // Gekuerzt fuer die Lage und die Frage nach leeren Zeilen; roh, wo der
    // nachlaufende Leerraum Inhalt ist (Modulkopf).
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

    for (nummer, (zeile, roh)) in bereinigt.iter().zip(&roh).enumerate() {
        // Eine Zeile, die ausserhalb jeder Anfuehrung und jedes Heredoc-Rumpfs
        // beginnt, verliert ihre ganze Einrueckung: fuer die Shell ist sie
        // Leerraum zwischen zwei Woertern.
        let (zeile, roh) = if matches!(lage, Lage::Normal) {
            (zeile.trim_start(), roh.trim_start())
        } else {
            (*zeile, *roh)
        };
        let letzte = nummer + 1 == bereinigt.len();
        if matches!(lage, Lage::Normal) && zeile.is_empty() {
            // Eine Leerzeile zwischen zwei Teilen des einen Befehls ist ein
            // Umbruch wie jeder andere und ergibt ein Leerzeichen. Meist steht
            // es schon; nach einer `\`-Fortsetzung ohne Leerraum nicht, denn
            // dort hat die Leerzeile als Folgezeile keinen gezeigt.
            if !aus.is_empty() && !aus.ends_with(char::is_whitespace) {
                aus.push(' ');
            }
            continue;
        }

        // Ob der nachlaufende Leerraum dieser Zeile Inhalt ist: im Rumpf eines
        // Heredocs und am Ende in einer offenen Anfuehrung.
        let (trenner, leerraum_bleibt) = match &lage {
            Lage::Heredoc(heredoc) => {
                // Die Endzeile eines Heredocs steht immer auf eigener Zeile
                // und ohne nachlaufenden Leerraum, damit sie ihn beendet;
                // der Rumpf ist Inhalt.
                if heredoc.endet_mit(zeile) {
                    lage = wartend.pop_front().map_or(Lage::Normal, Lage::Heredoc);
                    (Trenner::Umbruch, false)
                } else {
                    (Trenner::Umbruch, true)
                }
            }
            Lage::Normal | Lage::Einfach | Lage::Doppelt | Lage::Ansi => {
                let ende = zeile_lesen(zeile, &mut lage, &mut wartend);
                if let Zeilenende::Kommentar(stelle) = ende {
                    // Der Kommentar faellt samt dem Leerraum davor (Modulkopf).
                    aus.push_str(&zeile[..stelle]);
                    let behalten = aus.trim_end().len();
                    aus.truncate(behalten);
                    if let Some(heredoc) = wartend.pop_front() {
                        // Ein Heredoc, den die Zeile vor dem Kommentar
                        // geoeffnet hat: sein Rumpf ist Inhalt und bleibt.
                        if letzte {
                            break;
                        }
                        lage = Lage::Heredoc(heredoc);
                        aus.push('\n');
                        continue;
                    }
                    // Sonst gehoert alles danach zum Kommentar und faellt mit.
                    break;
                }
                if let Some(heredoc) = wartend.pop_front() {
                    lage = Lage::Heredoc(heredoc);
                    (Trenner::Umbruch, false)
                } else if !matches!(lage, Lage::Normal) {
                    (Trenner::Umbruch, true)
                } else if ende == Zeilenende::Fortsetzung {
                    (Trenner::Fortsetzung, false)
                } else {
                    (Trenner::Verbinden, false)
                }
            }
        };
        if letzte {
            // Die Ausgabe endet ohne nachlaufenden Leerraum (Modulkopf).
            aus.push_str(zeile);
            break;
        }
        aus.push_str(if leerraum_bleibt { roh } else { zeile });
        let folgezeile = bereinigt[nummer + 1];

        match trenner {
            Trenner::Umbruch => aus.push('\n'),
            Trenner::Verbinden => aus.push(' '),
            Trenner::Fortsetzung => {
                aus.pop();
                let vorher_leer = aus.ends_with([' ', '\t']);
                let nachher_leer = folgezeile.starts_with([' ', '\t']);
                if vorher_leer || nachher_leer {
                    while aus.ends_with([' ', '\t']) {
                        aus.pop();
                    }
                    aus.push(' ');
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
    /// Ein Umbruch ausserhalb jeder Anfuehrung und jedes Heredoc-Rumpfs:
    /// genau ein Leerzeichen.
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
    /// In einem Kommentar ab `#` am Wortanfang; die Zahl ist die Byte-Stelle
    /// des `#` in der Zeile.
    Kommentar(usize),
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
                    '#' if am_wortanfang(&zeichen, stelle) => {
                        return Zeilenende::Kommentar(
                            zeichen[..stelle].iter().map(|z| z.len_utf8()).sum(),
                        );
                    }
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

/// Die Umbruchregel von [`fliesstext`]; der Modulkopf schreibt sie aus.
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
