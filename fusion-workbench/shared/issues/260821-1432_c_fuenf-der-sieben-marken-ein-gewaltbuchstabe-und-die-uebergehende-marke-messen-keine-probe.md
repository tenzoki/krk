Fünf der sieben Marken, ein Gewaltbuchstabe und die übergehende Marke messen keine Probe

---

Die 146 Proben von `xtask` sehen `--prune`, `--follow-tags`, `-f`, `-d` und die
`--force`-Familie als angehalten nach. **Nicht nachgesehen sind `--tags`, `--all`, `--mirror`,
`--delete`, `--amend`, `--no-verify` und der Gewaltbuchstabe `a`.** Wer einen dieser Einträge
aus `MARKEN` oder `UEBERGEHENDE` löscht, lässt jede Probe grün. Das Abnahmekriterium C3.4
verlangt in seinem zweiten Satz genau das Gegenteil: „Die Probe verwirft jede der sechs oben
genannten Marken."

---

**Gemessen am Baumstand `94855a7`.**

## Was gezählt wurde

Jede Marke aus `MARKEN` (`xtask/src/git.rs:228-237`), `UEBERGEHENDE` (`:242`) und
`GEWALTBUCHSTABEN` (`:251`), gezählt im Prüfmodul von `git.rs`:

| Wort | Vorkommen im Prüfmodul | als angehalten nachgesehen |
|---|---|---|
| `--tags` | 0 | nein |
| `--follow-tags` | 1 | ja |
| `--all` | 0 | nein |
| `--mirror` | 0 | nein |
| `--delete` | 0 | nein |
| `--prune` | 1 | ja |
| `--amend` | 0 | nein |
| `--no-verify` | 0 | nein |
| `--force` (bar) | 1 | **nein** — die eine Stelle ist `die_aufsichtsmeldung_nennt_kommando_und_befund` (`:663`), und die prüft den Wortlaut der Meldung, nicht dass die Marke angehalten wird |
| `-f` | ja | ja (`:657`) |
| `-d` | ja | ja (`:581-582`) |
| `-a` | 0 | nein |

Nachgezählt mit

```sh
awk '/^#\[cfg\(test\)\]/,0' xtask/src/git.rs | grep -c -- '"--all"'
```

und den entsprechenden Mustern.

Die `--force`-Familie ist über `--force-with-lease`, `--force-with-lease=…` und
`--force-if-includes` in `die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen`
(`:578-596`) nachgesehen; der Wortanfang `--force` wird dadurch mitgeprüft. Die Lücke betrifft
den Gleichheitsvergleich, nicht den Präfixvergleich.

## Warum das gerade jetzt zählt

Die alte Probe `die_schreibenden_kommandos_tragen_keine_gewalt` prüfte in die andere Richtung:
sie sah nach, dass die **gebauten** Listen keine der neun Marken tragen. Dafür brauchte sie
keine Anhalteprobe je Marke, denn sie las die Listen und nicht die Aufsicht.

Seit `94855a7` ist die Aufsicht ein eigener Mechanismus mit einer eigenen Liste. Ein
Mechanismus mit einer Liste, deren Einträge niemand einzeln misst, ist eine Liste, aus der ein
Eintrag unbemerkt verschwinden kann — und das ist genau die Klasse, aus der die vorige
Durchsicht ihren Befund A1 gezogen hat: eine Aufzählung, die zusagt, vollständig zu sein, ohne
dass jemand sie nachzählt.

## Dieselbe Lage bei den Beispiel-Listen

`beispiele()` (`:443-458`) und `die_auftraege_stehen_wort_fuer_wort` (`:507-556`) zählen die
sieben Varianten von Hand auf. Beide sind `vec!`-Literale und keine Fallunterscheidungen, also
hält der Übersetzer sie **nicht**: eine achte Variante lässt beide grün. Der Prüfkommentar von
`beispiele()` sagt das für die eigene Zusage aus („Bleibt eine neue Variante hier stehen, fängt
die Aufsicht sie trotzdem — nur eben später"), und das trägt. Für C3.4, dessen erster Satz an
`die_auftraege_stehen_wort_fuer_wort` hängt, trägt es nicht: eine achte Variante wäre dort
ungesehen.

Der Doc-Kommentar „Die sieben Aufträge, Wort für Wort" (`:507`) trägt zusätzlich eine Zahl in
Prosa, die mit der achten Variante falsch wird und die nichts hält. CLAUDE.md führt diese
Klasse unter „Projektstand" ausdrücklich als wiederkehrenden Defekt dieses Projekts.

## Abhilfe

Zwei Handgriffe, beide klein:

1. **Je Eintrag eine Anhalteprobe.** Über `MARKEN` und `UEBERGEHENDE` laufen und für jeden
   Eintrag `gewaltbefund(eintrag).is_some()` behaupten, dazu `-a`, `-f`, `-d` einzeln. Das
   bindet die Listen an eine Messung und erfüllt den zweiten Satz von C3.4 wörtlich.
2. **`beispiele()` erschöpfend machen.** Eine Fallunterscheidung über `Auftrag` ohne
   Auffangzweig, die je Variante ein Beispiel liefert, hält der Übersetzer — dieselbe Bauart,
   die `worte()` und `wirkung()` schon tragen. Die Zahl in „Die sieben Aufträge" fällt dann
   weg, statt zu veralten.

**Schwere:** mittel. Kein Fehlverhalten am gebauten Stand — die Aufsicht hält alle zwölf
Wörter an, ich habe es nachgebaut und laufen lassen. Was fehlt, ist die Messung, die das
morgen noch zusagt.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `xtask/src/git.rs:228-251` (die drei Listen), `:443-458` (`beispiele`),
`:507-556` (`die_auftraege_stehen_wort_fuer_wort`), `:578-596`
(`die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen`),
`shared/planning/260821-1115_o_spec-artefakt-und-release.md`, C3.4

**Domain:** code

**Verwandt:**
`shared/issues/260821-1432_o_der-nachtrag-im-plan-nennt-c3-4-und-c3-5-als-haltend-obwohl-beide-eine-probe-nennen-die-es-nicht-mehr-gibt.md`
— dort steht die Aufzeichnungsseite desselben Sachverhalts.

---

Resolved: Behoben am 260821, beide Handgriffe der „Abhilfe" gefahren.

**Je Eintrag eine Anhalteprobe, aber nicht als Schleife über die Liste.** Der Datensatz schlägt
vor, über `MARKEN` und `UEBERGEHENDE` zu laufen; das prüfte, was dasteht, und bliebe grün, wenn
ein Eintrag verschwände — also genau die Lage, die der Datensatz beklagt.
`git::tests::jede_einzelne_marke_wird_angehalten` schreibt die elf langen Marken und die drei
Gewaltbuchstaben deshalb **aus** und sieht jedes Wort einzeln als angehalten nach; ein
gelöschter Eintrag wird damit rot. Nachgemessen als Mutation: `--all` aus `MARKEN` entfernt
lässt zwei Proben ausfallen. `jeder_eintrag_der_listen_steht_in_der_anhalteprobe` nimmt die
Gegenrichtung und fängt einen Eintrag, den jemand hinzufügt, ohne ihn nachzusehen.

**`beispiele()` hängt jetzt am Übersetzer.** Statt eines `vec!`-Literals entsteht die Liste aus
`naechster`, einer vollständigen Fallunterscheidung ohne Auffangzweig: eine achte Variante hält
den Bau an, nachgefahren (drei Anhaltestellen — `wortplaetze`, `wirkung`, `naechster`). Was die
Kette nicht hält, steht in ihrem Doc-Kommentar: wer den neuen Zweig auf `None` setzt und den
bisherigen letzten stehenlässt, hängt seine Variante nicht ein. Der Doc-Kommentar „Die sieben
Aufträge" ist gefallen; `die_auftraege_stehen_wort_fuer_wort` zählt jetzt gegen
`beispiele().len()` nach, statt eine Zahl in Prosa zu führen.

**Damit hält der zweite Satz von C3.4 wieder**, und die sechs Marken, die er nennt — `--force`,
`-f`, `--tags`, `--all`, `--mirror`, `--delete` — stehen alle in der ausgeschriebenen Liste. Der
Plan ist entsprechend berichtigt (verwandter Datensatz).
