# Playmaker-Lauf 260820-1044, unmittelbare Nutzerzuteilung

**Status:** Complete
**Playmaker**, Domäne `code` (aus der Zeile `**Domain:** code` der Zuteilung gelesen).
**Anlass:** Der Nutzer hat die Runde 14 (`260819-2230-auswahl-und-kopieren-in-der-vorschau`)
kohärent geschlossen und die Neuerzeugung des Portfolios beauftragt, mit dem ausdrücklichen
Auftrag, die Alterung der Grundlage des vorgesehenen Web-Betrachter-Circles zu prüfen.

## Bestand

| Marker | Bedeutung | Anzahl |
|---|---|---|
| `_t_` | aktiv | 0 |
| `_a_` | vorgesehen | 1 |
| `_c_` | kohärent geschlossen | 4 |
| `_b_` | beschränkt geschlossen | 10 |
| `_s_` | überholt | 0 |
| `_d_` | zurückgestellt | 1 |

Gefahrene Runden: 14. `fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz
trägt `_t_`. Das ist der reguläre Zustand nach einem Abschluss; keine der vier
Zeiger-Warnungen (`STALE-POINTER`, `POINTER-MISMATCH`, `MISSING-POINTER`, `MULTIPLE-ACTIVE`)
trifft zu.

## Rangfolge

**Vorgesehene Circles.** Rang 1 von 1:
`260804-0933-eingebauter-web-betrachter-im-vorschaufenster`. Ein bindender offener
Entscheidungsdatensatz (Verfügbarkeitsprüfung für Schnittstellen ab macOS 26), eine
Abhängigkeitskante auf die terminale Runde 1. Kein Abzug für beschränkten Abschluss angesetzt,
weil `CLAUDE.md` diesen Marker im Projekt als Auskunft über die Verfügbarkeit des Nutzers
ausweist und nicht über die Reife einer Runde.

**Rückstandsspeicher.** Rang 1 von 1:
`shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`.
Eine Idee, Datensätze auf der Platte zitiert, heute shapbar.

## Rückstandsspeicher im Einzelnen

- Gelesen: 1 Datei, Marker `_p_` (empfohlen). Keine mit `_o_`, `_c_` oder `_d_`.
- Unterschiedliche Ideen darin gefunden: 1.
- Doppelgänger-Gruppen gefunden: 0.
- An `## Warnings` abgegeben, weil defekt- oder entscheidungsförmig: 0 aus dem Speicher.

**Durchgeführte Schreibvorgänge: keine.** Der eine Eintrag steht auf `_p_` und bleibt dort; die
Umbenennung wäre die eine selbstständige Operation gewesen und war nicht angezeigt. Geteilt,
zusammengeführt, geschlossen und zurückgestellt wurde nichts.

**Vorgeschlagene und nicht durchgeführte Operationen: keine.** Der Speicher hat einen Eintrag mit
einer lebenden, fälligen Idee. Es gab nichts zu teilen, nichts zusammenzuführen, nichts zu
schließen und nichts zurückzustellen, also fehlte auch keine Bestätigung. Dieser Lauf hielt keine.

**Nicht gefilt.** Die drei Datensätze aus dem Abnahmelauf des Nutzers vom 260820-1030
(zwei Defekte, eine offene Frage) beschreiben zusammen mit dem Rückstandseintrag den Bereich
„Bewegung zwischen Editor und Vorschau". Als Rückstandseintrag ist der Bereich nicht gefilt
worden: das Filen ist dem Nutzer vorbehalten. Der Bereich steht in `## Warnings`, Punkt 2, und
in `## Backlog — ranked` als Einschränkung der Empfehlung.

## Abhängigkeitszyklen

Keiner. Der gerichtete Graph über die nicht-terminalen Circles enthält einen Knoten und keine
Kante innerhalb der Menge. Kein `## Dependency warning` in einen Circle-Datensatz geschrieben.

## Gealterte Elterngrundlage

`parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster
child=260819-2230-auswahl-und-kopieren-in-der-vorschau`

**Die übliche Auslösebedingung war nicht erfüllt.** Der Auftrag knüpft den Vermerk an ein Kind,
das beschränkt (`_b_`) schließt; die Runde 14 hat kohärent geschlossen. Der Nutzer hat die
Prüfung für diesen Lauf ausdrücklich beauftragt, und sie fiel bejahend aus. Der Vermerk nennt
fünf Punkte, vier davon Arbeit für die Klärungsrunde:

1. Die Vorschaufläche nimmt jetzt den Fokus und ist auswählbar (`vorschau.rs:1437`,
   Unterklasse `Vorschautext` bei `:395`). Der Schnappschuss vom 260804 hielt genau das für
   ausgeschlossen.
2. `ist_eigene_textflaeche` prüft zwei Textflächen statt einer. Das Muster der Anmeldung über die
   Nämlichkeit ist damit ein gebauter Präzedenzfall; der Klassentest in
   `ersthelfer_gehoert_appkit` greift für eine Web-Ansicht weiterhin nicht.
3. Die Auswahl im gerenderten Markdown liefert Quelltext über eine Kachelung
   (`markdown.rs`, `Zerlegung::kacheln`, Commit `13be459`), die es nur für Markdown gibt. Die
   zweite offene Frage des Circles, gerendertes HTML, kostet dadurch mehr als bisher.
4. Die erste Hälfte von `260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-…` bindet den
   Circle unverändert, steht aber seit der Runde 14 hinter dem Marker für überholt und fällt aus
   jeder Suche nach aktiver Grundlage heraus.
5. Unverändert am 260820 nachgezählt: `Kommando` 79 Varianten, `default-keymap.toml` 85
   Funktionsblöcke, `Rang` sechs Werte.

## Geschriebene Abschnitte

- `## Parent grounding stale` an
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`, angehängt.
- `## Activation proposal` an denselben Datensatz, angehängt. Rang 1 von 1, mit den zwei
  Kandidaten daneben, die nicht vorgesehen sind: der neue Bereich aus dem Abnahmelauf und der
  zurückgestellte Circle `260816-2255-befehle-absetzen-und-makros-speichern`.

Der Datensatz trägt danach 21 Playmaker-Abschnitte aus zwölf Läufen auf 1437 Zeilen. Kein
Circle-Marker umbenannt, `.active-circle` nicht angefasst.

## Warnungen im Portfolio

Fünfzehn, in dieser Reihenfolge: kein Abhängigkeitszyklus; der Bereich zwischen Editor und
Vorschau ohne Circle; der Vermerk zur gealterten Grundlage ohne die übliche Auslösebedingung;
die halb überholte Grundlage; vier Lesarten des Markers `_c_` und die offene Frage dazu; der
Abnahmelauf der Runde 14 ohne Datensatz auf der Platte; zwei nicht committete Datensätze der
Runde 14; die falsche Aussage in `CLAUDE.md` über die eine Ausnahme im Ereignisabgriff und die
fehlende vierzehnte Runde in der Tabelle; das offene Auslieferungstor (22 Commits seit `v0.5.4`,
kein Tag an HEAD); der Abnahmelauf der zehn Zeitzusagen seit 260810-1918 nicht gefahren; drei
ausstehende Abnahmeläufe; 145 offene Defektdatensätze (vorher 138); 33 offene und 14 beantwortete
Entscheidungsdatensätze (vorher 29 und 12); der mögliche Defekt im Rumpf des
Rückstandseintrags; die Länge des Web-Betrachter-Datensatzes.

## Sprache

Dieser Lauf schreibt deutsch. Der Lauf vom 260819-0804 schrieb englisch unter der Deklaration
`**Artifact language:** en`, die der Nutzer am 260819-2032 zurückgenommen hat
(`shared/decisions/260819-1500_*_gilt-die-artefaktsprache-en-fuer-den-ganzen-bestand-oder-wird-die-deklaration-zurueckgenommen.md`,
Möglichkeit 2). `bin/fusion-rules` gibt seither `stilwerk/default-voice-de.yaml` und
`stilwerk/chat-voice-de.yaml` aus; beide Profile waren vorhanden und sind angewandt worden.
Übersetzt wurde nichts.

## Portfolio

`fusion-workbench/portfolio.md`, vollständig neu erzeugt.
