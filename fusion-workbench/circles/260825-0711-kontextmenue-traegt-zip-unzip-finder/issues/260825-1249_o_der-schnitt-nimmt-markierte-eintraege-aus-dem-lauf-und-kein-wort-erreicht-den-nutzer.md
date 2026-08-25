Der Schnitt nimmt markierte Eintraege aus dem Lauf, und kein Wort erreicht den Nutzer

---

`packziel` und `ohne_die_eigenen_ziele` nehmen einen markierten Eintrag aus den Quellen und melden
es nirgends: keine Statuszeile, keine Abschlussliste, kein Blatt. Von drei markierten Archiven
entsteht ein Ordner, und die zwei uebrigen bleiben ohne Ordner und ohne Wort. Genau diesen Ausgang
weist derselbe Modulkopf fuenfzig Zeilen weiter oben ausdruecklich zurueck, als er die Frage nach dem
unbrauchbaren Namen entscheidet.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Die Stelle, die dagegensteht

`crates/krk-ui/src/kommandos/kontextmenue.rs:340-344`, im Doc-Kommentar von `brauchbarer_stamm`:

```
/// [`super::operationen::kein_archiv`] („hier steht keine Datei mit der Endung
/// .zip") waere vor seinen Augen die Unwahrheit. Und den Eintrag
/// stillschweigend aus [`Entpackbefund::Archive`] zu nehmen waere schlechter
/// als beides: von drei markierten Archiven bliebe eines ohne Ordner und ohne
/// Wort.
```

Der Satz entscheidet die Frage fuer den unbrauchbaren Namen, und er entscheidet sie gegen das
stille Herausnehmen. `ohne_die_eigenen_ziele` (`kontextmenue.rs:612`) nimmt seit `dd74b0e` genau so
heraus.

## Der Ablauf am Bildschirm

**Entpacken.** Neben `a.zip` steht `a.zip.zip` — die anhaengende Endungsregel dieser Runde legt es
dorthin. Der Nutzer markiert beide und waehlt Unzip. Der Lauf traegt ein Paar, `a.zip.zip` →
`<ordner>/a.zip`. Weil dort schon die Datei `a.zip` steht, geht das Konfliktblatt der
Zielordnerklaerung auf und nennt **den Zielpfad**; von `a.zip` als markiertem Archiv, das nicht
entpackt wird, steht dort nichts. Waehlt der Nutzer „Ueberspringen", ist der Lauf zu Ende: kein
Ordner entstanden, ein Archiv unangetastet, kein Satz darueber, dass das zweite gar nicht erst im
Auftrag stand.

**Packen.** Markiert sind `a.txt` und `Projekte.zip`; gepackt wird allein `a.txt`. Die Positionszahl
in der Statuszeile zaehlt seit `dd74b0e` die verbliebenen Quellen (`anwendung.rs:6132`), zeigt also
„1" statt „2". Das ist folgerichtig und beantwortet die Frage nicht, warum es eins ist.

## Was die Zusage deckt und was nicht

Die Nutzerantwort vom 260825 sagt: „Ein Eintrag, dessen Pfad dem gerechneten Archivnamen gleicht,
faellt aus den Quellen heraus." Das Herausfallen ist damit entschieden und steht hier nicht in
Frage. Ueber die Meldung sagt die Antwort nichts — und der neue Modulkopf von `zippen.rs`
beschreibt die einzige Meldung, die es je gab, als die des **nicht** geschnittenen Falls:
„Steht er doch einmal auf der Quellenliste, geht er in den Papierkorb und fehlt dem Lauf danach als
Quelle, die er als ausgelassen meldet" (`crates/krk-core/src/operation/zippen.rs:71-72`). Wer
geschnitten wird, kommt in dieser Meldung nicht vor: er steht nicht im Auftrag, und die
Abschlussliste kennt nur, was darin steht. Die Meldung ist nicht verworfen worden, sie ist mit der
Quelle weggefallen.

Die Directive dieser Runde bindet daneben: die drei Eintraege sind immer da und immer bedienbar, und
wo ein Befehl nichts vorfindet, meldet er es in der Statuszeile.

## Vorschlag

Zwei Wege, und der erste ist der kleinere.

1. **Eine Meldung in der Statuszeile.** `packziel` und `entpackziel` geben neben den Quellen die
   Zahl der geschnittenen Eintraege heraus; die zwei auftragstellenden Zweige in
   `crates/krk-ui/src/appkit/anwendung.rs` haengen sie an die Antwort, die sie ohnehin zeigen. Ein
   Satz nach dem Muster der bestehenden `operationen`-Meldungen, an einer Stelle formuliert.
2. **Den Eintrag im Auftrag lassen und die Abschlussliste ihn als ausgelassen melden.** Das ist der
   Weg, den der Nutzer am 260825 verworfen hat (Weg 1 des Vorschlags: der Kern haelt die Zusage), und
   er kommt hier nur der Vollstaendigkeit halber vor.

**Schwere:** mittel. Kein Verlust und keine falsche Wirkung, aber ein Befehl tut wortlos weniger,
als der Nutzer markiert hat, und der Baum hat diese Frage fuer die Schwesterlage schon anders
entschieden.

**Betroffen:** `crates/krk-ui/src/kommandos/kontextmenue.rs` (`packziel`, `ohne_die_eigenen_ziele`),
`crates/krk-ui/src/appkit/anwendung.rs` (`zipauftrag_stellen`, `entpackauftrag_stellen`),
`crates/krk-ui/src/kommandos/operationen.rs` (der Wortlaut).
