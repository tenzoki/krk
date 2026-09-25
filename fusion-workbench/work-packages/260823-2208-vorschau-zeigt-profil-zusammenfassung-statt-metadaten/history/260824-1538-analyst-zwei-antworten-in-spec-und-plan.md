# Die zwei Antworten vom 260824-1505 in Spec und Plan eingetragen

**Datum:** 2026-08-24 15:38
**Agent:** analyst
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Auftrag:** Reine Buchführung. Der Nutzer hat am 260824-1505 zwei offene Entscheidungen mit Möglichkeit 2 beantwortet; Spec und Plan sollen die Antworten tragen, der Plan die Umsetzung als eigenen Schritt, und ein Defektdatensatz soll schließen. Keine Datei unter `crates/` und keine unter `resources/` anfassen — dort arbeiten andere Agenten.

---

## Ergebnis

**Drei Abnahmekriterien des freigegebenen Specs sind berichtigt** (C5.2, C5.3, C5.6), dazu die Festlegung A7. Jede Berichtigung steht **neben** ihrem ursprünglichen Wortlaut, in der Form vom 260824-1224, und keine ersetzt ihn: der freigegebene Wortlaut ist der Beleg dafür, was der Nutzer am Tor bestätigt hat.

**Der Plan trägt die Umsetzung als Schritt 14 in Bündel C**, Executor `ontocoder`, Abhängigkeit Schritt 7, zwei Zeilen TOML und keine Zeile Rust. Der Schritt liegt vor Schritt 12, weil Schritt 12 C6.7 gegen die eingebettete Auslieferungsfassung misst; die Bedingung steht im Schritt, in der Dependencies-Zeile von Schritt 12 und als Kante `s14 --> s12` im Graphen.

**Die falsche Zahl ist an beiden Planstellen berichtigt.** „Genau ein Verzeichnis" fällt aus den drei Zustandszeilen — es sind zwei, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` und `260816-2255-befehle-absetzen-und-makros-speichern`, beide `_d_`, unter achtzehn Verzeichnissen. Der Defektdatensatz `260824-1313_*_der-datensatz-zur-vierten-zustandszeile-…` ist geschlossen und auf `_c_` umbenannt.

| Datei | Was geändert ist |
|---|---|
| `planning/260824-0613_o_spec-…` | Zeilen 4, 90, 278, 279, 282, 288–292 |
| `planning/260824-0640_o_plan-…` | Zeilen 4, 229, 234, 243–245, 276, 280, 338–342, 370, 395, 410, 413, 427, 599–600, 618–619, 621 |
| `issues/260824-1313_c_…es-sind-zwei.md` | `Resolved:` eingetragen, `_o_` → `_c_` mit `git mv` |

## Was bewusst stehen geblieben ist

- **Die Marker der zwei Entscheidungsdatensätze bleiben auf `_a_`.** Auf `_i_` gehen sie erst, wenn die zwei Zeilen TOML in `resources/default-readers.toml` stehen; das ist Schritt 14.
- **Die Zeile „Der Zustand eines Circles über drei Vorhandensein-Zeilen: Festlegung A7"** unter `**Decisions made:**` von C5 im Spec. Sie hält fest, woraus C5.6 abgeleitet ist; die vier Berichtigungen vom 260824-1224 haben aus demselben Grund keine Decisions-made-Zeile angefasst.
- **`## Open for Planner` des Specs** („Wie die drei Zustandszeilen aus C5.6 in der Auslieferungsfassung stehen, wenn Festlegung A7 am Tor hält") und das Beispiel-TOML unter C5, dessen Pfadmuster sechs Namen zeigt. Beide zeichnen den Stand beim Verfassen auf; das Beispiel sagt von sich selbst, dass es beispielhaft ist.
- **Die Antwortzeile des Planers** in der Tabelle unter „Die sieben Antworten auf `## Open for Planner`" und der Wortlaut von Schritt 7. Beide halten fest, was zur Planungszeit beantwortet und was am 260824-1313 gebaut worden ist; die vierte Zeile kommt mit Schritt 14 und nicht rückwirkend in Schritt 7.
- **Die Kopfzeile des Plans nennt weiter „die Schritte 1 bis 7 und 13" als erledigt**, obwohl Schritt 8 in der Schrittliste auf `[DONE]` steht. Diese Abweichung ist älter als dieser Lauf und gehört nicht zum Auftrag; sie ist nur um den offenen Schritt 14 ergänzt.

## Nächster Schritt

Schritt 14 an den `ontocoder`, vor Schritt 12. Danach gehen die zwei Entscheidungsdatensätze auf `_i_`.
