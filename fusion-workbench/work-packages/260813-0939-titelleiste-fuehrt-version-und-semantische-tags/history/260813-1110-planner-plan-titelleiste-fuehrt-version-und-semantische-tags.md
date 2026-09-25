# Planner: Umsetzungsplan für die Titelleisten- und Tag-Runde

**Datum:** 260813-1110
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/`
**Eingaben:** der Spec vom 260813-1037, der Circle-Datensatz, die vier beantworteten Entscheide in `decisions/`, der Diagrammbericht vom 260813-1049

---

## Was entstanden ist

| Datei | Art |
|---|---|
| `planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md` | Umsetzungsplan, vier Stränge, sechzehn Schritte, drei Mermaid-Bilder |
| `decisions/260813-1110_o_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md` | offene Nutzerfrage aus Schritt A1 |
| `issues/260813-1110_o_der-entscheid-zum-ueber-dialog-nennt-zwei-befehle-die-heute-schon-nicht-durchkommen.md` | Befund am vierten Entscheid |
| `issues/260813-1110_o_die-schluesselfensterfrage-erreicht-den-freigabewaehler-nicht-weil-er-kein-fenster-ist.md` | Befund am vierten Entscheid |
| `issues/260813-1110_o_eine-vierte-wegwerfordner-fassung-steht-in-xtask-und-die-probe-liest-die-kiste-nicht.md` | Befund am Prüfbestand, nicht Gegenstand der Runde |

Alle drei Mermaid-Blöcke sind mit `@mermaid-js/mermaid-cli` nach SVG gerendert worden und parsen.

## Wie erhoben wurde

Zwei Analysten liefen nebeneinander: einer über `xtask/`, das `Makefile`, `README.md` und die Wurzel-`Cargo.toml`, einer über das Menümodell, den Menübau und die Zulässigkeitsregel. Der Rest ist selbst gelesen: `zulaessigkeit.rs` ganz, die einschlägigen Stellen in `anwendung.rs`, `ereignisse.rs`, `fenster.rs`, `belegung.rs`, `quellbaum.rs`, `teilen.rs`, dazu der Kopf des Systems für `NSTitlebarAccessoryViewController` und die Bindungen in `objc2-app-kit 0.3.2`.

## Vier Befunde, die den Zuschnitt verschoben haben

**Die Zulässigkeitsregel hat zwei Frager und nicht drei.** Spec, Entscheid und Auftrag sprechen von drei; eine Probe im Baum hält die Zahl auf zwei. Der dritte ist ein Abnehmer der Lage und nicht ein Frager der Regel. Der Unterschied ist nicht kosmetisch: der Zeichenzweig ruft `zulaessig` nicht und braucht die vierte Bedingung deshalb auch nicht.

**Die Lücke ist enger als beschrieben und benennbar.** `Anwendungsdelegierter::fokus` fragt schon heute nach dem Schlüsselfenster. Durch kommen allein die vierundzwanzig Befehle mit `Wirkungsbereich::Ueberall`; die beiden Beispiele des Entscheids, `F5` und `delete`, tragen `Wirkungsbereich::Dateifenster` und kommen nicht durch. Das macht den Bau kleiner und die Abnahme genauer: der Unterschied zwischen alter und neuer Regel fällt allein in einer Zeile der Tafel an, und ohne eine eigene Probe dafür zeigte keine bestehende ihn.

**Der Freigabewähler ist kein Fenster.** Er erscheint über `showRelativeToRect:`, also als Verfolgungsschleife. Der Nebeneffekt, den der Entscheid für Möglichkeit 2 in Aussicht stellt, ist damit nicht belegt. Der Plan schliesst den Defekt der Runde 6 deshalb nicht im Voraus, sondern trägt die eine Beobachtung, die ihn entscheidet.

**Die Stationszählung des Auslieferungswegs steht an drei Stellen und ist unvollständig.** Sechs numerierte Stationen, drei unnumerierte Vorläufe dazwischen. Das ist der mittlere Befund B3 des Diagrammprüfers, und der Plan legt die Reihenfolge fest: sieben durchgehend numerierte Stationen, die neue Prüfung als Station 1, die drei Vorläufe mit Buchstaben und der Station, der sie zuarbeiten.

## Was der Plan offen lässt

Eine Nutzerfrage, und sie ist eine Zeile im Code: hebt die Ausnahmeliste auch die vierte Bedingung auf? Die Empfehlung sagt ja, damit Cmd+Q vor einem fremden Fenster weiter beendet; der Wortlaut des Entscheids sagt nein. Der Plan fährt auf der Empfehlung und ändert bei der anderen Antwort einen Schritt und sonst nichts.

## Was der Plan ausdrücklich nicht anfasst

Der Defekt am Auslieferungsort (`shared/issues/260813-0026_*_...`) bleibt draussen, wie der Spec es begründet. Ebenso der Defekt zur Signaturmeldung von `bundle`. Die vierte Wegwerfordner-Fassung in `xtask` ist als Befund abgelegt und nicht in die Runde gezogen; der Plan umgeht sie, indem keine seiner Proben ein Verzeichnis braucht.

## Nächster Schritt

Das Plan-Tor. Der Nutzer liest den Plan, beantwortet die eine offene Frage oder lässt sie auf der Empfehlung stehen, und danach beginnt die Ausführung mit Strang A, weil Strang C ihn als Vorbedingung hat.
