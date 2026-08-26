# Der Doppelungshinweis sagt „vor der Ortsangabe", und fünf von sieben fusion-Zeilen haben keine

---
**Domain:** data
**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml`, die vier Doppelungshinweise über `fusion-Werkbank: die Wurzel`, `Projektwurzel mit fusion-Werkbank`, `flight-Werkbank: die Wurzel` und `Projektwurzel mit flight-Werkbank`; `shared/issues/260825-2126_c_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md` (der Datensatz, dessen Behebung die Formulierung auf den vierten Block gebracht hat)

---

## Was ist

Alle vier Doppelungshinweise sagen denselben Satz. In der flight-Fassung, seit `180fc53`:

> DIESE ELF ZEILEN STEHEN EIN ZWEITES MAL IN DIESER DATEI, nämlich im Profil
> „Projektwurzel mit flight-Werkbank" ganz unten, dort jede mit `flight-workbench/` vor
> der Ortsangabe.

„Jede mit `<werkbank>/` vor der Ortsangabe" trifft nur die Zeilen, die im Wurzelprofil
schon eine Ortsangabe tragen. Die Feldzeilen tragen keine — sie meinen den erkannten
Ordner selbst — und bekommen im Projektwurzelprofil den ganzen Schlüssel `ordner` dazu:

```
Wurzelprofil        feld = { datei = '^\.flight-setup$', feldmuster = … }
Projektwurzel       feld = { ordner = "flight-workbench", datei = '^\.flight-setup$', feldmuster = … }
```

Es sind drei der elf flight-Zeilen und **fünf der sieben** fusion-Zeilen: „Projekt",
„Eingerichtet", „fusion-Fassung", „Aktive Runde" und „Sitzung". Beim fusion-Paar
beschreibt der Satz damit die Minderheit seiner eigenen Zeilen.

Die Angabe ist älter als `180fc53`; sie stammt vom fusion-Paar und ist mit der Behebung
von `260825-2126` wörtlich auf den vierten Block gekommen. Nachgemessen: normalisiert man
beide Schreibungen — den gestrichenen Schlüssel **und** das Präfix —, sind beide Paare
zeichengleich. Inhaltlich stimmt der Hinweis also; die Regel, die er ausschreibt, ist
unvollständig.

## Warum das zählt

Dieser Satz ist die einzige Beschreibung der Umformung, die ein Bearbeiter von Hand
anwenden muss, und die Datei setzt ausdrücklich auf ihn statt auf einen Mechanismus. Wer
ihm folgt und eine achte Feldzeile hinzufügt, schreibt sie im zweiten Block ohne
Ortsangabe hin — der Satz nennt für seinen Fall keine — und bekommt eine Zeile, die im
Projektwurzelprofil auf den ausgewählten Ordner statt auf die Werkbank darin sieht. Sie
lädt, meldet nichts und antwortet still falsch.

Der Satz trifft dieselbe Stelle noch ein zweites Mal: eine Probe, die die zwei Blöcke
gegeneinander hält (`shared/issues/260826-0903_*_die-zeichengleichheit-der-zwei-…`),
wird nach diesem Satz gebaut und ist dann an einem gesunden Stand rot.

## Was zu tun wäre

Den Halbsatz um den zweiten Fall ergänzen, in allen vier Hinweisen gleichlautend, etwa:
„dort jede mit `flight-workbench/` vor der Ortsangabe, und die Feldzeilen, die hier keine
tragen, mit `ordner = \"flight-workbench\"` davor."

**Zuständig:** `ontocoder`.

**Schwere:** niedrig. Eine unvollständige Regel in einem Handbuchteil, deren Wirkung
heute an keiner Zeile falsch ist.
