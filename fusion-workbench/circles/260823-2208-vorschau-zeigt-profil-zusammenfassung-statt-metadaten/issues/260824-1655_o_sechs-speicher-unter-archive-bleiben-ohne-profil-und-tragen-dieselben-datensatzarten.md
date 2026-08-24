Sechs Speicher unter `archive/` bleiben ohne Profil und tragen dieselben Datensatzarten

---

Das Pfadmuster des Speicherprofils (`resources/default-readers.toml:216`) verlangt hinter
`fusion-workbench/` entweder `shared` oder `circles/<name>`. Die Speicher, die der Archivschritt
von `/fusion:cleanup` nach `fusion-workbench/archive/<lauf>/shared/` verschoben hat, erreicht es
damit nicht. Sechs Ordner mit denselben Datensatzarten zeigen weiter die Metadatenanzeige.

---

**Gemessen am 260824-1655** gegen alle 154 Verzeichnisse unter `fusion-workbench/`. Ohne Profil
bleiben 16, und diese sechs davon sind Speicher:

```
archive/260819-1613-safe-cleanup-tier-1/shared/backlog
archive/260819-1613-safe-cleanup-tier-1/shared/decisions
archive/260819-1613-safe-cleanup-tier-1/shared/issues
archive/260819-1613-safe-cleanup-tier-1/shared/planning
archive/260820-2115-safe-cleanup-tier-1/shared/decisions
archive/260820-2115-safe-cleanup-tier-1/shared/issues
```

Die übrigen zehn sind keine Speicher und sollen keines bekommen: `.guard-state` (dreimal),
`archive` selbst, die zwei Laufordner darunter, deren zwei `shared`-Hüllen, `fusion-workbench/shared`
als Hülle und `stilwerk`.

**Die Zahl aus `b5bf2e3` ist richtig und deckt diesen Fall nicht ab.** Die Commit-Nachricht sagt
„ohne Profil bleiben 0 statt 21" und nennt ihren Bezugsrahmen ausdrücklich: „Gemessen an den 118
Unterordnern unter `shared/` und den achtzehn Runden". Nachgerechnet: 78 + 19 + 21 = 118 vorher,
99 + 19 + 0 = 118 nachher. Innerhalb dieses Rahmens stimmt sie Stelle für Stelle. Der Rahmen
schließt `archive/` aus, und die Berichtigung zu C5.2/C5.3 im Spec (`planning/260824-0613_*_…:288`)
tut es ebenso.

**Ob das ein Mangel ist, ist eine Frage und keine Feststellung.** Ein archivierter Datensatz ist
eingefrorener Bestand: `rules/fusion-workbench-conventions.md:48` führt `archive/` als „target of
cleanup's archive step" und nicht als Speicher neben den übrigen. Dagegen steht, dass der Nutzer
diese Ordner im Dateifenster genauso anwählt wie die lebenden und dort dieselben Datensätze
findet — im größten der sechs liegen die geschlossenen Defekte, die aus `shared/issues`
herausgeräumt wurden.

**Wenn es eines werden soll**, kostet es eine Alternative im Pfadmuster und keinen Baustein:
`fusion-workbench/(shared|circles/[^/]+|archive/[^/]+/shared)/(…)$`. Der Haushalt bleibt
unberührt, C6.7 ebenso.

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.
