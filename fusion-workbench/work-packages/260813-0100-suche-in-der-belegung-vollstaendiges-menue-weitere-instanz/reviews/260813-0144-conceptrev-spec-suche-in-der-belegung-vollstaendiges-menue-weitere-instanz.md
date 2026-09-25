# Concept Evaluation: Spec Suche in der Belegungsansicht, vollständiges Menü, weitere Instanz (zweiter Durchgang)

**Date:** 2026-08-13 01:44
**Target:** `fusion-workbench/shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md`
**Verdict:** acceptable
**Diagrams evaluated:** 3  |  **Validation:** by-tool (mmdc 11.16.0 über `npx`, alle drei Blöcke nach PNG gerendert und angesehen)
**Vorgänger:** `260813-0109-conceptrev-…` (Spruch `tangled`), Nachzug des Shapers `history/260813-0135-shaper-nachzug-diagrammpruefung.md`

## Spruch

**Der Entwurf trägt, und nichts von dem, was wir gefunden haben, gehört vor die Planung.** Die zwei Befunde, die den ersten Spruch getragen haben, sind an der Sache behoben und nicht an der Zeichnung: die Zulässigkeitsregel deckt das Gegenbeispiel jetzt ab, und aus einer Sperre sind zwei Mechanismen mit zwei Lebensdauern geworden. Beide Änderungen haben wir am Baum nachgeprüft und nicht aus dem Nachzugsprotokoll übernommen.

Was bleibt, sind drei Stellen, an denen die Bilder weniger sagen als der Text, und keine davon ändert eine Regel. Die schwerste steckt im neuen Zustandsautomaten: seine zwei Regionen tragen keine Wächter, und formal gelesen behauptet er damit das Gegenteil von C1.15, wonach die Suche während einer Aufnahme nichts aufnimmt. Der Vorrang zwischen den zwei Bedeutungen von `esc` bleibt aus demselben Grund offen. Das Flussdiagramm daneben zeichnet denselben Vorrang richtig, also widersprechen sich zwei Bilder desselben Spec. Der Unterschied zum ersten Durchgang ist entscheidend und trägt den Spruch: damals zeigten die Bilder eine Lücke im Entwurf, heute zeigen sie eine Regel unvollständig, die im Kriterienteil steht und dort stimmt. Ein Wächter je Kante schließt es; ein Nachzug der Planung ist es nicht.

## Die drei Aussagen des Shapers, nachgeprüft

| Aussage | Befund |
|---|---|
| 1 · Die Frage steht als **ein** Funktionsknoten mit zwei gestrichelten Aufrufkanten, der Sonderweg des Fokusvorbehalts ist weg | **Zutrifft.** Im gerenderten Bild münden genau zwei gepunktete, mit „fragt" beschriftete Kanten von `A1` und `A2` in `REGEL`. Ein eigener Vorbehaltsknoten existiert nicht mehr; die Frage ist Bestandteil 2 der Regel. |
| 2 · Aus einer Sperre sind zwei geworden | **Zutrifft.** `SR` (Sitzungsrecht, ein Halter vom Start bis zum Prozessende) und `SS` (Schreibsperre, je Durchgang) sind zwei Knoten mit zwei Formen, zwei Aufschriften und zwei Lebensdauern. Die verneinende Kante von Instanz 2 ist fort, ihr Kasten trägt keinen Sitzungsschreiber mehr. |
| 3 · Neu ist ein `stateDiagram-v2` mit zwei nebenläufigen Regionen | **Zutrifft der Form nach, nicht der Begründung nach.** Die zwei Regionen stehen im Bild. Die Prosa darunter begründet sie mit Unabhängigkeit, und unabhängig sind die Regionen nicht: C1.15 gibt der Aufnahme den Vorrang beim selben Ereignis. Siehe Befund 1. |

Die vom Shaper selbst berichtete Kante, die im Bild am falschen Knoten zu enden schien, ist behoben: `FREI --> TUN` und `A1 -->|ja| TUN` laufen beide sichtbar in `kommando_ausfuehren` und in keinen anderen Knoten. Keine Kante läuft gegen die Leserichtung.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `stateDiagram-v2` | 4 Zustände (+1 Verbund, 2 Pseudo) | 8 | 2,00 | 2 (`kein Suchtext`) | 2 (`Suchtext steht`) | 2 (gewollt, Selbstübergänge und Aufnahmepaar) | ja, zwei Regionen | 0 | acceptable |
| 2 | `flowchart TD` | 15 | 17 | 1,13 | 3 (`A1`, `A2`) | 2 (`WEITER`, `TUN`, `REGEL`) | 0 | ja, zwei `subgraph`-Blöcke | 0 | acceptable |
| 3 | `flowchart LR` | 13 | 12 | 0,92 | 4 (`NB`) | 6 (`SS`) | 0 | ja, drei `subgraph`-Blöcke | 0 | clean |

Diagramm 2 hat eine Quelle (`E`) und fünf Senken (`ZUW`, `SUCH`, `ERST`, `TUN`, `REGEL`); alle 15 Knoten sind von `E` aus erreichbar. Unbeschriftet sind 5 von 17 Kanten, davon vier ohne Bedeutungsverlust; `FREI --> TUN` trägt die Aussage „ein Mausklick führt über dieselbe Stelle aus" und trüge eine Beschriftung besser.

Diagramm 3 hat sechs Quellen und vier Senken. Unbeschriftet ist eine von zwölf Kanten (`SS --> NB`). Der Eingangsgrad 6 an `SS` ist kein Gott-Knoten, sondern die Bauform einer Sperre; wir beanstanden ihn ausdrücklich nicht.

Die zwei Zyklen in Diagramm 1 sind die Selbstübergänge und das Paar `keine Aufnahme` und `Aufnahme`. In einem Zustandsautomaten sind das Zustandswechsel und keine Abhängigkeitszyklen.

## Befunde

### 1. Die zwei Regionen tragen keine Wächter, und damit sagt der Automat das Gegenteil von C1.15 (substanziell für das Bild, Diagramm 1)

Zwei nebenläufige Regionen bedeuten in der Semantik dieses Diagrammtyps, dass ein Ereignis an **beide** aktiven Regionen zugestellt wird. Der Automat sagt damit zweierlei, das der Spec ausschließt:

- Ein Suchzeichen, das während einer laufenden Aufnahme getippt wird, löst `kein Suchtext --> Suchtext steht` aus. C1.15 sagt: „Während der Aufnahme nimmt die Suche nichts auf."
- Der Übergang `Belegungsansicht --> [*]` hängt am Verbundzustand und feuert unabhängig davon, welcher Teilzustand in der Aufnahmeregion aktiv ist. Damit verlässt `esc` die Ansicht auch mitten in einer Aufnahme. C1.13 und C1.15 sagen: der Fänger steht vorn, während der Aufnahme bricht `esc` allein sie ab.

Die zwei Bedeutungen von `esc` **stehen** im Bild, einmal als „oder nacktes esc bricht ab" an der Aufnahmekante und einmal als „Fertig (Cmd+Eingabe) oder esc" am Ausgang. Was fehlt, ist der Vorrang zwischen ihnen, und genau der war der Grund, für die Betriebsarten überhaupt ein Diagramm zu verlangen. Die Prosa fängt es mit dem Satz auf, der Vorrang liege beim Fänger. Ein Automat, den die Prosa korrigieren muss, misst nicht mehr, was er messen soll.

Die Regionen selbst sind die richtige Wahl, und die Begründung darunter ist es nicht: unabhängig sind die **Zustände** (der Suchtext übersteht eine Aufnahme, C1.12), nicht die **Ereigniszustellung** (die Aufnahme hat Vorrang, C1.15). Drei Wächter bringen beides zur Deckung:

```
A --> B: Suchzeichen [keine Aufnahme]
B --> B: … [keine Aufnahme]
Belegungsansicht --> [*]: Fertig (Cmd+Eingabe) oder esc [keine Aufnahme]
```

Nebenbefund am gerenderten Bild: die Beschriftung „Eingabetaste, Rücktaste: nichts" des Selbstübergangs überlagert das Wort „Suchzeichen" der Kante daneben. Rein zeichnerisch, ohne Aussagewert.

### 2. Der Nachschlag hat im Code drei Ausgänge, im Bild zwei, und der fehlende ist der gefährliche (mittel, Diagramm 2)

`NACH` („Belegung nachschlagen") trägt zwei Ausgänge: „Funktion mit Kommando" und „kein Treffer oder Funktion ohne Kommando". Am Baum sind es drei. `crates/krk-ui/src/appkit/ereignisse.rs:498-513` unterscheidet `Nachschlag::Funktion`, `Nachschlag::Sprungmarke` und `Nachschlag::Unbelegt`, und der mittlere Zweig gibt das getippte Zeichen an die Senke, die es verbrauchen kann. Eine Taste ohne Zusatztaste, die keiner Funktion gehört, ist deshalb heute **kein** Fall von „unverändert an AppKit", sondern der Fall der Sprungmarke aus C2 der Runde 1.

Das Gewicht liegt nicht in der Vollständigkeit als Selbstzweck. Der Spec führt die Stelle selbst als Risiko, unter „Offen für den Planner": „Der Fokusvorbehalt steht heute **vor** dem Nachschlag und muss den getippten Zeichen der Sprungmarke erhalten bleiben, auch wenn die Kommandos die Frage später stellen; ein Zeichen, das während einer Umbenennung in den Sprungmarkenpuffer liefe, wäre derselbe Defekt in klein." Das Bild zieht den Vorbehalt in die Zulässigkeitsregel hinein und zeigt die Station nicht mehr, an der er zusätzlich stehen bleiben muss. Wer den Plan aus dem Bild schreibt, baut `zulaessig` mit zwei Aufrufern und lässt die Wache vor der Sprungmarke fallen.

Daran hängt die Aufschrift des Regelknotens, „eine Funktion, zwei Frager". Für `zulaessig(Kommando)` stimmt sie. Die Teilfrage aus Bestandteil 2, ob der Ersthelfer AppKit gehört, hat danach **drei** Frager: die Regel, die Wache vor der Sprungmarke und, unverändert, `ersthelfer_gehoert_appkit` selbst. Das ist kein Doppelbau, solange alle drei dieselbe Funktion rufen. Der Spec sollte den Satz führen, und ein dritter Ausgang an `NACH`, der die Sprungmarke zeigt, machte es im Bild sichtbar.

### 3. Der Ja-Zweig der Menüprüfung ist aus dem gezeichneten Eingang nicht erreichbar (mittel, Diagramm 2)

`FREI` („Eintrag bedienbar, per Mausklick und per Kürzel") hängt allein an `A2 -->|ja|`. Jeder Weg nach `MENUE` kommt aus `WEITER`, und `WEITER` hat zwei Eingänge:

```
A1  --|nein|--------------------> WEITER    dieselbe Funktion, dieselben Eingaben, also sagt A2 auch nein
NACH --|kein Treffer / ohne Kommando|--> WEITER    kein Menüeintrag, oder ein Textbefehl ohne Zulässigkeitsregel (C2.8)
```

Für einen Tastendruck ist `FREI` damit unerreichbar, und das ist die richtige Aussage: genau darauf beruht C2.17. Nur fehlt dem Bild der zweite Benutzer des Menüs. Ein Mausklick ist kein Tastendruck, kommt nie durch den Abgriff und ist der einzige Weg, auf dem `FREI` etwas bedeutet. Eine zweite Quelle „Mausklick auf einen Eintrag" mit einer Kante nach `A2` schlösse die Lücke und brächte zugleich C2.19 ins Bild, den benannten Preis der Runde: dass die Ausgrauung dem Eintrag auch den Mausklick nimmt. Heute steht dieser Preis allein in der Prosa, obwohl er die Fähigkeit spürbar beschneidet.

### 4. Die vierte Zeile des Regelknotens ist weiter gefasst als C2.5 (geringfügig, Diagramm 2)

Der Knoten liest sich als `zulässig = (1 ∧ 2 ∧ 3) ∨ immer_erreichbar`. C2.5 sagt etwas Engeres: die benannte Liste hebt Bestandteil (1) und (2) auf, Bestandteil (3) gilt weiter. Praktisch fällt der Unterschied heute nicht an, denn beide Befehle der Liste tragen `Wirkungsbereich::Ueberall` (`crates/krk-core/src/tasten/belegung.rs:747-752`), und für die ist (3) immer wahr. Wächst die Liste, fällt er an. Ein Zusatz „ohne Rücksicht auf 1 und 2" in der Zeile kostet vier Wörter.

### 5. Der Knoten des allerersten Starts schwebt neben dem falschen Kasten (geringfügig, Diagramm 3)

`ERST` liegt außerhalb aller drei `subgraph`-Blöcke und kommt im gerenderten Bild unmittelbar über dem Kasten von Instanz 2 zu liegen. Seine Aufschrift sagt „gleich in welcher Instanz" und rettet die Aussage; die Lage legt das Gegenteil nahe. Rein zeichnerisch.

## Die zwei Fragen des Auftrags

**Deckt die dreiteilige Regel das Gegenbeispiel ab? Ja, und wir haben es am Baum durchgerechnet statt es zu glauben.** Beim Umbenennen direkt in der Liste hält der Feldeditor den Ersthelferrang, `ersthelfer_gehoert_appkit` liefert wahr (Aufrufstelle `crates/krk-ui/src/appkit/ereignisse.rs:488`, Funktion ab `:536`), Bestandteil (2) ist damit falsch, und die Konjunktion fällt, obwohl (1) und (3) beide wahr bleiben: es steht kein Blatt, und `fokus()` antwortet für den Feldeditor weiterhin `Dateifenster` (`crates/krk-ui/src/appkit/anwendung.rs:3528`, im Doc-Kommentar wörtlich). Der Menüeintrag zu `up` ist ausgegraut, die Taste läuft zum Feldeditor, und C2.6 ist erfüllt. Die benannte Ausnahmeliste öffnet das Loch nicht wieder: `beenden` und `fenster_schliessen` sind Cmd-Kombinationen und in einem Textfeld ohne Bedeutung, und ihre Aufnahme ist aus „kein Verlust gegenüber heute" abgeleitet, nicht gewählt.

**Trägt der Zustandsautomat die zwei Bedeutungen von `esc`, und stimmen seine Regionen mit den Kriterien überein? Die Bedeutungen ja, ihr Vorrang nein, die Regionen nur mit Wächtern.** Befund 1 führt es aus. Der Automat hat dabei geleistet, wofür er gebaut wurde: er hat die Lücke bei leerem Suchtext aufgedeckt, die jetzt als C1.17 im Spec steht. Dass er an einer zweiten Stelle noch untermodelliert ist, spricht nicht gegen ihn, sondern für drei Wächter.

## Nicht zu beanstanden

**Alle drei Blöcke parsen, und wir haben die Bilder angesehen.** `mmdc` 11.16.0 hat alle drei nach PNG erzeugt. Kein Syntaxbefund.

**Die Typwahl stimmt dreimal.** Ein Zustandsautomat für die Betriebsarten, ein gerichteter Fluss für den Weg eines Tastendrucks, ein `LR`-Fluss mit Kästen für die Ablage. Die Typtafel der Regel sieht genau diese drei Zeilen vor.

**Diagramm 3 ist sauber, und der erste Durchgang hatte an ihm fünf Befunde.** Zwei Sperren mit zwei Formen und zwei Lebensdauern, jede Schreiberkante mit Zieldatei und Anlass beschriftet, `keymap.toml` und `settings.toml` haben ihre Erzeuger bekommen, die Nachbardatei steht als Knoten zwischen Sperre und Dateien, und die verneinende Kante ist fort. Dass Instanz 2 die Sitzung nicht schreibt, steht jetzt als Fehlen eines Knotens in ihrem Kasten, und das ist die einzige Form, in der ein Graph etwas verneinen kann.

**Die Schichtung von Diagramm 2 trägt die tragende Aussage.** Der Schichtwechsel zwischen Abgriff und AppKit ist die Stelle, an der die Zulässigkeitsfrage zum zweiten Mal gestellt wird, und die zwei gestrichelten Kanten in denselben Knoten machen die Nämlichkeit zur Eigenschaft des Graphen statt zur Behauptung einer Beschriftung. Das war Befund 1 des ersten Durchgangs, und er ist an der Wurzel behoben.

**Die Zahlen des Spec stimmen, soweit wir sie geprüft haben.** `Kommando` trägt heute 75 Varianten und `resources/default-keymap.toml` 81 Funktionen; C4.1 und C4.2 rechnen von dort aus richtig weiter. Die Tafel aus C2.5 kommt mit sieben Wirkungsbereichen mal fünf Fokuswerten mal zwei mal zwei auf die genannten 140 Fälle. (Am Rand, außerhalb unseres Auftrags: `CLAUDE.md` nennt für `Kommando` noch 68 Varianten, Stand 260811.)

## Was ein sauberer Nachzug verlangt

**Nichts davon gehört vor die Planung.** Wir sagen das ausdrücklich, weil der Auftrag danach gefragt hat: der Entwurf ist tragfähig, die zwei Änderungen des ersten Durchgangs sitzen, und keiner der fünf Befunde ändert eine Regel, eine Fähigkeit oder ein Abnahmekriterium. Der nächste Schritt kann die Planung sein.

Zwei Dinge sollte der Plan aber mitnehmen, und beide sind billig:

**Erstens die Sprungmarke.** Der Planner-Punkt zur Aufteilung der Zulässigkeitsfrage nennt sie, das Bild nicht. Wer den Plan schreibt, prüfe ausdrücklich, dass die Wache vor dem Sprungmarkenpuffer stehen bleibt, wenn der Fokusvorbehalt zum Bestandteil der Regel wird. Ein dritter Ausgang an `NACH` machte es im Bild sichtbar und kostet eine Zeile Mermaid.

**Zweitens die drei Wächter im Zustandsautomaten.** Solange sie fehlen, zeigt der Automat einen anderen Vorrang als das Flussdiagramm daneben, und zwei Bilder desselben Spec geben zwei Antworten auf dieselbe Frage. Der Plan liest beide.

Die Befunde 3 bis 5 sind Sache einer Nachbesserung der Zeichnung und keiner Eile wert. Der Spruch ist beratend; die Entscheidung liegt beim Nutzer.
