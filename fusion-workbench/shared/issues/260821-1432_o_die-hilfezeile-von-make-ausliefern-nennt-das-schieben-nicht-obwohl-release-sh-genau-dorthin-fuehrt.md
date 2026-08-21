Die Hilfezeile von `make ausliefern` nennt das Schieben nicht, obwohl `release.sh` genau dorthin führt

---

`94855a7` hat den Befund F1 der Durchsicht vom 260821-1346 an der `##`-Zeile von `release`
behoben und eine Probe daran gebunden. Der Weg, den der Nutzer tatsächlich tippt, ist aber
`./release.sh <zahl>`, und der führt nach `make ausliefern` — dessen `##`-Zeile das Schieben
weiterhin nicht nennt. In `make help` steht die berichtigte Zeile also neben der
unberichtigten, und die unberichtigte ist die des Haupteingangs.

---

**Gemessen am Baumstand `94855a7`.**

## Der Befund

`Makefile:125`:

```make
ausliefern: ## Version setzen, eintragen, taggen und ausliefern: make ausliefern VERSION=0.2.0
```

`Makefile:135`, mit `94855a7` berichtigt:

```make
release: ## Bauen, signieren, beglaubigen, HEAD und Tag zu origin schieben, veroeffentlichen
```

`make help` gibt beide aus, nachgefahren am Baumstand:

```
  ausliefern     Version setzen, eintragen, taggen und ausliefern: make ausliefern VERSION=0.2.0
  release        Bauen, signieren, beglaubigen, HEAD und Tag zu origin schieben, veroeffentlichen
```

`ausliefern` ruft `cargo xtask version` und danach `$(MAKE) release` (`Makefile:126-128`),
schiebt also alles, was `release` schiebt, und setzt zusätzlich einen Tag. `release.sh` endet
auf `exec make -C … ausliefern VERSION="$1"` (`release.sh:36`); der Kopfkommentar desselben
Skripts zeichnet die Kette und nennt `ausliefern` als die Schicht darunter.

## Warum die Begründung des Commits gerade hier trägt

Der Kommentar über dem `release`-Ziel (`Makefile:129-133`) begründet die Berichtigung so:

```
# **Dieses Ziel wirkt ueber das Geraet hinaus.** … Die
# ##-Zeile darunter nennt sie deshalb, denn sie ist es, die `make help` vor dem
# Tippen ausgibt.
```

Das Argument gilt für `ausliefern` mit demselben Gewicht und einem Zusatz: `ausliefern` wirkt
über das Gerät hinaus **und** schreibt zusätzlich in den Arbeitsbaum. Wer `make help` liest,
bevor er tippt, tippt nach dem, was `release.sh` ihm nahelegt, also `ausliefern`.

## Was die neue Probe nicht sieht

`release::tests::die_hilfezeile_des_makefiles_nennt_das_schieben` (`xtask/src/release.rs:1218-1236`)
sucht die Zeile, die mit `release: ##` beginnt, und prüft sie auf `origin` und `schieben`. Für
`ausliefern` gibt es nichts Entsprechendes. Der Prüfkommentar der Probe schreibt die Einsicht
aus, die dazu führt — „eine Zählprobe fängt, was falsch **dasteht**, nie, was fehlt" — und
wendet sie dann auf ein Ziel an statt auf beide.

## Abhilfe

Die `##`-Zeile von `ausliefern` das Schieben nennen lassen, etwa: „Version setzen, eintragen,
taggen, ausliefern und zu origin schieben: make ausliefern VERSION=0.2.0". Die Probe
gleichzeitig über beide Ziele laufen lassen, statt den Zielnamen fest einzusetzen — sonst
steht dieselbe Lücke beim nächsten wirkenden Ziel wieder da.

**Schwere:** mittel. Kein Fehlverhalten am Code. Es ist die Auskunft, die der Nutzer als
letzte liest, bevor er die einzige nicht zurücknehmbare Wirkung dieses Projekts auslöst — und
sie steht am Ziel, das er nicht tippt.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `Makefile:124-128`, `xtask/src/release.rs:1218-1236`

**Domain:** code

**Verwandt:** `shared/reviews/260821-1346-coderev-artefakt-und-release.md`, Befund F1 — dieser
Datensatz ist sein Rest.
