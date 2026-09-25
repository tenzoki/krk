Der Datensatz zur vierten Zustandszeile nennt ein Verzeichnis außerhalb der drei Zeilen, es sind zwei

---

`decisions/260824-0634_o_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`
sagt in Zeile 25, von den achtzehn Circle-Verzeichnissen dieser Werkbank falle „heute genau
eines" aus den drei Zustandszeilen aus A7. Der Plan wiederholt die Angabe unter
`### Die fünf mitgelieferten Profile` („dieser Werkbank betrifft das heute genau ein
Verzeichnis") und unter `## Open Questions`.

**Es sind zwei.** Nachgezählt am 260824-1313 mit `ls circles/*/_d_circle.md`:

    circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_d_circle.md
    circles/260816-2255-befehle-absetzen-und-makros-speichern/_d_circle.md

Beide antworten auf `^_a_circle\.md$`, `^_t_circle\.md$` und `^_[cb]_circle\.md$` mit „nein",
gemessen am 260824-1313 mit der fertigen `resources/default-readers.toml` gegen die echte
Werkbank. Kein Circle-Datensatz trägt heute `_a_` oder `_s_`.

**Schwere:** niedrig. Die Zahl ist kein Bauauftrag und hält keinen Schritt auf; sie ist die
Grundlage, auf der der Nutzer die offene Frage nach der vierten Zustandszeile entscheidet, und
sie beziffert deren Nutzen heute um die Hälfte zu klein.

**Gefunden:** ontocoder, bei Schritt 7, beim Halten der Ausdrücke gegen den echten Bestand.

**Betroffen:** `decisions/260824-0634_o_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`,
`planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`

**Domain:** data

---
Resolved: Beide betroffenen Stellen tragen die gemessene Zahl. Der Entscheidungsdatensatz `decisions/260824-0634_a_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md` nennt die zwei Verzeichnisse in seinem `Answered:`-Abschnitt und sagt dazu, dass der ursprüngliche Wortlaut in `## Question` als Beleg der Fragegrundlage stehen bleibt; der Plan `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` ist am 260824-1538 an seinen zwei Stellen berichtigt, unter `### Die fünf mitgelieferten Profile` und unter `## Open Questions`, jeweils mit den beiden Verzeichnisnamen und dem Vermerk, welche Zahl dort stand. Nachgemessen am 260824-1508: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` und `260816-2255-befehle-absetzen-und-makros-speichern`, beide `_d_`, unter achtzehn Circle-Verzeichnissen; kein Datensatz trägt heute `_a_` oder `_s_`. Die Antwort des Nutzers vom 260824-1505 ist von der falschen Zahl unberührt — sie bezifferte den Nutzen der vierten Zustandszeile zu klein und nicht zu groß. Kein Code ist angefasst.
