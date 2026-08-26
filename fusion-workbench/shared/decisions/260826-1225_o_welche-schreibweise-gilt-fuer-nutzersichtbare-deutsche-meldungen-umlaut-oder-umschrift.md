# Welche Schreibweise gilt für nutzersichtbare deutsche Meldungen — Umlaut oder Umschrift?

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/ablage/mod.rs` (`Grund::beschreibung`, `Ersetzung::fmt`), `crates/krk-core/src/leseprofil/mod.rs` (`Ortsmangel::grund`, `Wert::als_text`), `crates/krk-core/src/leseprofil/datei.rs` (die Meldungen aus `pruefen`), `crates/krk-core/src/text/datei.rs:263-273`, `crates/krk-ui/src/appkit/anwendung.rs:7730`

---

## Question

KRK schreibt seine Meldungen in die eine Statuszeile am Fuß des Dateifensters, und sie stehen
dort in zwei verschiedenen Schreibweisen des Deutschen nebeneinander: einmal mit Umlauten und
ß, einmal in der Umschrift `ae`/`oe`/`ue`/`ss`, die dieses Projekt sonst für Doc-Kommentare und
Bezeichner verwendet. Welche der beiden für **nutzersichtbaren** Text gilt, ist nirgends
entschieden, und der Code fällt deshalb je nach Modul verschieden aus.

Der Widerspruch steht innerhalb **einer** Datei. `crates/krk-core/src/leseprofil/mod.rs`
schreibt an `:726`

```rust
format!("mindestens {gezaehlt} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)")
```

und zweihundert Zeilen darüber, in `Ortsmangel::grund`, dessen Doc-Kommentar ausdrücklich sagt
„so wie ihn die Statuszeile zeigt":

```rust
Ortsmangel::LeeresStueck  => "traegt ein leeres Stueck",
Ortsmangel::Punktstueck   => "traegt ein Stueck . oder ..",
```

Beide Sätze können in derselben Sitzung in derselben Zeile erscheinen.

**Die Umschrift ist im Umfang dieser Durchsicht die Regel und nicht die Ausnahme.** Elf
nutzersichtbare Zeichenketten unter `crates/krk-core/src/{ablage,leseprofil}/` tragen sie:
`ablage/mod.rs:281` („ist beschaedigt"), `:282` („liess sich nicht anlegen"), `:283` („ist zu
gross"), `:438` („liegt gekuerzt unter"), `:450` („liess sich nicht zur Seite legen"), `:655`
(„die Datei traegt keinen einzigen obersten Schluessel"), `:777` („keine gueltige UTF-8-Folge"),
`ablage/atomar.rs:98` („traegt keinen Dateinamen"), `leseprofil/mod.rs:530`, `:531`, `:533`,
dazu die Meldungen aus `leseprofil/datei.rs:440`, `:447`, `:471`, `:500` („laesst sich nicht
uebersetzen", „traegt … Fanggruppen", „braucht dafuer ihren Pfad").

**Der übrige Baum hält es umgekehrt.** `crates/krk-core/src/text/datei.rs:263-273` schreibt
„lässt sich nicht im Editor öffnen" und „ist … zu groß für den Editor",
`crates/krk-core/src/tasten/belegung.rs:325` „Lesezeichen- und Geräteleiste",
`crates/krk-ui/src/spalten.rs:87` „Größe", `crates/krk-ui/src/leistenmodell.rs:51` „Geräte und
Orte", `crates/krk-ui/src/menuemodell.rs:117` „Über KRK". Diese Texte erscheinen in derselben
Anwendung, teils in derselben Zeile. Daneben steht in `krk-ui` mindestens eine Meldung in
Umschrift, `appkit/anwendung.rs:7730` („die Sitzung liess sich nicht sichern"), die Frage geht
also über `krk-core` hinaus.

Nichts im Baum entscheidet die Frage. `CLAUDE.md` sagt allein: „Prosa in diesem Projekt ist
deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen
englischen Konventionen." Über die Schreibweise nutzersichtbarer Prosa steht dort nichts, und
kein Entscheidungsdatensatz führt sie.

## Options

1. **Umlaute für jede nutzersichtbare Zeichenkette, Umschrift bleibt für Doc-Kommentare und
   Bezeichner.** Die Naht liegt dort, wo sie ohnehin schon liegt: was der Nutzer liest, ist
   Prosa; was der Übersetzer liest, ist Code.
   - Pro: Es ist die Schreibweise, die das Menü, die Spaltenüberschriften, die Leiste und die
     Editormeldungen heute schon tragen — die Mehrheit dessen, was der Nutzer überhaupt zu
     sehen bekommt. Sie ist außerdem die richtige deutsche Schreibweise, und KRKs Oberfläche ist
     deutsch. Die Frage ist damit für jede künftige Zeichenkette an einer Stelle beantwortet.
   - Contra: Rund fünfzehn Zeichenketten allein im Umfang dieser Durchsicht sind nachzuziehen,
     und wer eine übersieht, hat die Mischung nicht beseitigt, sondern nur verkleinert. Proben,
     die auf den Wortlaut einer Meldung prüfen, ziehen mit.
   - Was sie ausschließt: nichts Späteres. Die Umschrift bleibt dort, wo sie steht.

2. **Umschrift überall, auch nutzersichtbar.** Eine Schreibweise im ganzen Baum.
   - Pro: Eine Regel statt zweier, und keine Naht, an der jemand entscheiden muss, ob eine
     Zeichenkette nutzersichtbar ist.
   - Contra: Sie macht die deutsche Oberfläche falsch geschrieben — „Groesse" als
     Spaltenüberschrift, „Ueber KRK" im Menü. Die Zahl der nachzuziehenden Stellen ist dabei
     größer als bei Möglichkeit 1, denn die Oberfläche trägt die Umlaute an mehr Stellen als
     der Kern die Umschrift.
   - Was sie ausschließt: eine spätere Rückkehr zu Umlauten kostet denselben Durchgang noch
     einmal.

3. **Es bleibt, wie es ist, und die Mischung wird als Eigenschaft geführt.**
   - Pro: Kostet nichts.
   - Contra: Der Nutzer sieht in derselben Statuszeile „ist beschaedigt" und „Einträgen". Das
     ist keine Eigenschaft, die sich begründen ließe, und die nächste Runde entscheidet die
     Frage für ihre eigenen Zeichenketten wieder von vorn — so ist die heutige Lage entstanden.

## Constraints

- Die Antwort gilt für `krk-core` und `krk-ui` gemeinsam. Eine Regel je Kiste wäre keine
  Antwort: die Meldungen beider landen in derselben Statuszeile.
- Doc-Kommentare und Bezeichner sind von der Frage nicht berührt. Dass sie Umschrift tragen,
  ist gesetzt und steht hier nicht zur Wahl.
- Der Quelltext ist bereits UTF-8 und trägt an über dreißig Stellen Umlaute in
  Zeichenketten; es ist keine Frage der Kodierung, sondern der Schreibweise.
- Nach der Antwort ist der Durchgang mechanisch, aber nicht durch ein Suchmuster zu erledigen:
  „traegt" ist in einem Doc-Kommentar richtig und in einer Meldung falsch. Gesucht wird an den
  Stellen, die eine Meldung bauen, nicht an der Zeichenfolge.

## Recommendation

Möglichkeit 1. Der Grund ist nicht die Rechtschreibung, sondern die Naht: die Frage „liest das
ein Mensch oder ein Übersetzer" ist an jeder Zeichenkette entscheidbar, und sie trennt die
beiden Schreibweisen vollständig und überschneidungsfrei. Die Naht von Möglichkeit 2 gibt es
nicht — dort müsste die Oberfläche falsch geschrieben werden, damit der Kern nicht zwei
Schreibweisen führt.

Der Umfang ist zu nennen und nicht zu verschweigen: rund fünfzehn Zeichenketten unter
`crates/krk-core/src/{ablage,leseprofil}/`, dazu die Meldungen der übrigen Module und
mindestens eine in `krk-ui`. Eine vollständige Erhebung gehört vor die Umsetzung, und sie geht
über die Stellen, die eine Meldung **bauen**, nicht über ein Wortmuster — der Datensatz
`shared/issues/260826-1225_*_drei-prosastellen-der-ablage-nennen-die-zahl-der-dateien-falsch-und-jedes-bisherige-suchmuster-musste-sie-uebersehen.md`
misst an derselben Ablage, was ein Wortmuster hier übersieht.
