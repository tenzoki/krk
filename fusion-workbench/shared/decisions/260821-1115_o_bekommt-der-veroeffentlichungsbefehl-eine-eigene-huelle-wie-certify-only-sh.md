# Bekommt der Veröffentlichungsbefehl eine eigene Hülle wie `certify-only.sh` und ein Makefile-Ziel?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `shared/planning/260821-1115_o_spec-artefakt-und-release.md` (C1), `README.md` Abschnitt „Auslieferung", `Makefile` Ziele `ausliefern`, `release`, `beglaubigen`, `release.sh`, `certify-only.sh`

---

## Frage

Der Spec „Artefakt und Release" führt einen achten Schritt der Auslieferungskette ein, der wie
`beglaubigen` zwei Wege hat: als Station von `cargo xtask release` und als eigenständiger Aufruf
für den Fall, dass ein Lauf am Hochladen gescheitert ist. Für den eigenständigen Weg des
Beglaubigens hat das Projekt zwei Hüllen gebaut, `make beglaubigen VERSION=…` und
`./certify-only.sh <zahl>`. Ob der neue Befehl dieselben zwei Hüllen bekommt, deckt keine der
acht Antworten der Klärung ab, und die Frage entscheidet, was der Nutzer tippt, wenn ein
Auslieferungslauf erst am Hochladen scheitert.

Der Preis der Antwort ist niedrig, ihre Sichtbarkeit hoch: `cargo` steht auf diesem Gerät nicht
auf dem Standard-PATH, weshalb jeder direkte Aufruf den vollen Pfad braucht. Genau das nehmen
die Hüllen dem Nutzer ab.

## Optionen

1. **Keine neue Hülle.** Der Befehl ist als `cargo xtask <name> <zahl>` erreichbar, die
   `README.md` nennt den Aufruf mit vollem cargo-Pfad.
   - Pro: keine neue Datei, keine neue Zeile im Makefile, die Hüllenkette bleibt so schmal wie
     heute. Der Befehl braucht kein Notarprofil, also fügt eine Makefile-Zeile weniger hinzu als
     bei `beglaubigen`.
   - Contra: der eigenständige Weg ist der unbequemste von dreien, obwohl er der ist, den man in
     einer Störung braucht.
2. **Nur ein Makefile-Ziel.** `make veroeffentlichen VERSION=0.5.6`, zwei Zeilen, dieselbe Gestalt
   wie `make beglaubigen`.
   - Pro: löst den PATH-Punkt vollständig; folgt dem Muster, das für den zweiten Wiederaufnahmeweg
     schon dasteht.
   - Contra: eine dritte Stelle, an der der Name des Unterbefehls steht.
3. **Makefile-Ziel und Skripthülle**, also zusätzlich ein `./publish-only.sh <zahl>` neben
   `./release.sh` und `./certify-only.sh`.
   - Pro: vollständige Symmetrie der drei Wiederaufnahmewege.
   - Contra: eine dritte Skriptdatei in der Projektwurzel für einen Befehl, der seltener gebraucht
     wird als die zwei vorhandenen.

## Randbedingungen

Jede Antwort lässt die Aussage unberührt, dass `release.sh` und `certify-only.sh` keine Logik
tragen. Eine Hülle, die eine Prüfung oder eine Fehlerbehandlung mitbrächte, wäre ein drittes
Bauwerkzeug und fiele aus dem Muster; das schließt der Modulkopf von `release.sh` ausdrücklich
aus.

## Empfehlung

Der Spec fährt vorläufig auf Option 1, weil sie die schmalste ist und keine Aussage vorwegnimmt,
die eine der beiden anderen später schwer machte. Option 2 lässt sich jederzeit in zwei Zeilen
nachziehen, sobald der Nutzer den eigenständigen Weg zum ersten Mal wirklich braucht. Für Option 3
sehen wir bislang keinen Anlass: der Weg wird seltener gefahren als das Beglaubigen, und drei
Skripte in der Wurzel kosten mehr Übersicht, als sie Tipparbeit sparen.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Abgleich 260821-1532 (reconciler, Baumstand `4e810f9`): **offen. Der Marker ist nicht bewegt
worden.**

**Der Baum trägt Option 1, und das ist keine Antwort.** Nachgezählt: die Projektwurzel führt zwei
Skripte, `release.sh` und `certify-only.sh`, kein `publish-only.sh`; das `Makefile` führt kein
Ziel `veroeffentlichen`. Der Befehl ist allein als `cargo xtask veroeffentlichen <zahl>`
erreichbar. Genau so hat es der Spec unter `## Offene Nutzerentscheidungen` angekündigt — mit dem
Wort „vorläufig" —, und der Plan wiederholt es unter `## Open Questions` als ungehakt. Eine
Empfehlung, die sich selbst als vorläufig bezeichnet, und ein Baum, der ihr folgt, sind zusammen
kein `Answered:`; wer das als Antwort nähme, entschiede durch vollendete Tatsache.

**Der Nachtrag der Durchsicht vom 260821-1346 ist bis heute nicht eingetragen gewesen; er steht
hiermit.** Die `README.md` trägt die Begründung für Option 1 inzwischen ausgeschrieben, im
Abschnitt „Nur veröffentlichen" (`README.md:445-462`): dass es keine Hülle gibt, dass das die
schmalste Fassung und eine bewusste Wahl ist, dass die Frage dem Nutzer vorliegt, und dass der
Aufruf deshalb den vollen Pfad zu `cargo` braucht, samt dem Handgriff `export PATH=…`. Wer
Option 2 oder 3 wählt, zieht diese Zeilen mit — das erhöht die Kosten beider gegenüber dem, was
der Datensatz unter „Contra" annimmt, um eine Stelle in der `README.md`.
