Die Ablage-Aufzählung steht neben dem Zeiger, der sie ersetzen sollte, und veraltet mit der achten Datei

---

Der Eintrag L06 von `fb50fcd` setzt in `CLAUDE.md` den Satz „**Welche Dateien das sind, sagt
`Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs`) und nicht diese Zeile**" — und lässt die
Aufzählung, die er ersetzen soll, unmittelbar davor stehen, jetzt auf sechs Glieder erweitert.
Damit trägt die Stelle beides: den Zeiger und die Kopie, die mit der nächsten Ablagedatei
falsch wird. Genau diesen Fall führt derselbe Satz als Begründung an („die Aufzählung hier hat
die Einstellungen und die Leseprofile übergangen, seit es sie gibt").

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

**Domain:** code

**Betroffen:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß", der erste
Absatz („KRKs Bestand liegt außerhalb des Bündels")

## Selbst gefahren am 260826-0923

```
awk '/pub const ALLE/,/\];/' crates/krk-core/src/ablage/pfade.rs
  → pub const ALLE: [Datei; 7] = [ Belegung, Lesezeichen, Sitzung, Einstellungen, Leser,
                                   Zettel(Erster), Zettel(Zweiter) ];
```

Sieben Dateien, sechs Glieder in der Prosa (die zwei Zettel zusammengefasst). Der Stand stimmt
heute. Er stimmt nicht mehr, sobald `Datei` einen achten Wert bekommt — und `Datei` ist
gewachsen: die `settings.toml` und die `readers.toml` sind dieselben zwei, deren Fehlen der
Satz jetzt einräumt.

## Warum das trägt

`CLAUDE.md` behandelt dieselbe Fehlerklasse an drei anderen Stellen anders, und zwar
ausdrücklich:

- `Kommando`: „steht hier **keine Zahl**: sie wächst mit fast jeder Runde und ist in dieser
  Datei viermal in vier Tagen falsch geworden."
- `ohne_warten_oeffnen`: „Wie viele Aufrufer die Hülle hat und wo sie liegen, sagt
  `grep -rn …` **und nicht diese Zeile**" — dort steht danach keine Kopie der Liste, sondern
  eine grobe Ortsangabe.
- Die Aufzählungen `Wirkungsbereich`, `Bereich`, `Fokus`: Zahlen mit ausgeschriebenem
  Zähldatum („Am 260825 nachgezählt").

Der neue Absatz nimmt keine der drei Formen an. Er trägt den Zeiger **und** die vollständige
Kopie, ohne Zähldatum. Ein Leser, der die Kopie für die Auskunft nimmt, hat keinen Anlass, den
Zeiger zu ziehen — das ist der Grund, aus dem die anderen Stellen die Kopie weggelassen haben.

**Schwere:** gering. Heute richtig; die Fehlerklasse ist dieselbe, die die Stelle beschreibt.

## Vorschlag

Eine der drei bestehenden Formen wählen. Am billigsten: die Aufzählung fallen lassen und beim
Zeiger bleiben, wie bei `ohne_warten_oeffnen`. Wer sie behalten will, setzt ein Zähldatum
daneben, wie bei `Wirkungsbereich`.

**Gefunden:** coderev, Durchsicht von `e5ec81a..20c9833` am 260826-0923
