# Shaper: Nachzug des Spec nach der Diagrammprüfung

**Datum:** 260813-0135
**Agent:** shaper (user-direct, autonom — keine Rückfrage an den Nutzer)
**Anlass:** Spruch `tangled` der Diagrammprüfung `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/reviews/260813-0109-conceptrev-spec-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`

## Was geändert wurde

**Ein Spec, zwei Entscheidungsdatensätze, kein neuer Datensatz.**

- `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` — Ort und Name unverändert, `**Status:**` auf „Überarbeitet nach der Diagrammprüfung", Zeile `**Überarbeitet:**` dazu, angehängter Abschnitt `## Nachzug vom 260813-0130`. Die Historie ist nicht umgeschrieben.
- `shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md` — Möglichkeit 1 trennt jetzt Schreibsperre und Sitzungsrecht, die Empfehlung stützt sich auf das Sitzungsrecht statt auf „die Sperre", und eine Randbedingung zur Freigabe beim Absturz ist dazugekommen.
- `shared/decisions/260813-0053_o_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md` — „zulässig" trägt drei Bestandteile statt zwei; die zwei immer erreichbaren Befehle stehen als Randbedingung darin. Die drei Möglichkeiten und die Empfehlung sind unberührt.

## Die zwei Befunde am Entwurf

**1. Das Loch in der Zulässigkeitsregel — am Baum bestätigt, nicht übernommen.** Nachgesehen: `crates/krk-ui/src/appkit/anwendung.rs:3528` (`fokus()` antwortet für den Feldeditor `Dateifenster`), `crates/krk-ui/src/appkit/ereignisse.rs:488` und `:536` (`ersthelfer_gehoert_appkit`, Prüfung auf `NSTextView`/`NSTextField`/`NSText` mit der Nämlichkeitsausnahme für die Editorfläche), `crates/krk-ui/src/appkit/anwendung.rs:2109` (`kommando_ausfuehren` fragt Blatt und `fokus::wirkt` und sonst nichts), `crates/krk-ui/src/kommandos/fokus.rs:334` (`wirkt`), `crates/krk-ui/src/kommandos/operationen.rs:266` (`waehrend_blatt_erlaubt` ist eine Zeile). Der Prüfer hat recht: mit zwei Bestandteilen ist beim Umbenennen in der Liste jeder Befehl des Dateifensters freigegeben.

Die Regel trägt jetzt drei Bestandteile plus eine benannte Ausnahmeliste. Text **und** Bild sind nachgezogen: das erste Diagramm hat drei `subgraph`-Schichten, die Frage steht als **ein** Funktionsknoten mit zwei gestrichelten Aufrufkanten, und der Sonderweg des Fokusvorbehalts an der Frage vorbei ist verschwunden.

Beim Zuschneiden sind zwei Folgen aufgefallen, die der Prüfer nicht genannt hat und die der Spec jetzt trägt:

- **Ohne Ausnahmeliste nähme die neue Regel Wirkung weg.** Cmd+Q und Shift+Cmd+W sind heute während einer Umbenennung in der Liste und während eines stehenden Blattes allein über ihren Menüeintrag erreichbar. Die Liste ist aus „kein Verlust gegenüber heute" abgeleitet und trägt genau diese beiden; `fenster_einblenden` steht mit Begründung nicht darauf.
- **Die Ausgrauung kostet auch den Mausklick.** Das steht jetzt als C2.19 im Spec, damit der Preis am Tor sichtbar ist. Die Alternative — Kürzel abgeben, Eintrag klickbar lassen — ist verworfen und begründet: zweiter Mechanismus, zwei Antworten auf eine Frage, und eine Datei ließe sich mitten in einer Umbenennung in den Papierkorb klicken.

**2. Zwei Mechanismen unter einem Wort.** Aus „der Sperre" sind **Schreibsperre** (kurzlebig, je Lesen-Ändern-Schreiben) und **Sitzungsrecht** (langlebig, vom Start bis zum Prozessende) geworden. Der Entscheidungsdatensatz ist mitgezogen, weil seine Empfehlung genau auf der langlebigen Lesart ruht. Zwei Löcher sind beim Trennen aufgefallen und geschlossen: die Sperre muss den ganzen Durchgang umfassen und nicht nur das Schreiben (C3.8), und ein Prozess muss beides auch beim Absturz freigeben (C3.13) — das schließt ein Sperrverzeichnis ohne Aufräumregel aus und steht jetzt im Planner-Punkt.

## Das dritte Diagramm

Gebaut, der Beurteilung ist gefolgt. `stateDiagram-v2` mit zwei nebenläufigen Regionen: Suchtext und Aufnahme sind unabhängig, und das ist genau die Aussage von C1.12. Der Automat hat eine Lücke aufgedeckt — was Eingabetaste und Rücktaste bei **leerem** Suchtext tun, stand nirgends. C1.17 sagt es jetzt, mit der Begründung von C1.8.

## Die sieben übrigen Befunde

Alle sieben abgearbeitet, keiner stehen gelassen. Die Zuordnung steht als Tabelle im angehängten Abschnitt des Spec.

## Werkzeug

Alle drei Mermaid-Blöcke sind mit `mmdc` 11.16.0 über `npx` nach SVG und PNG gerendert und angesehen worden. Die erste Fassung des ersten Diagramms hat die Kante „Eintrag bedienbar → kommando_ausfuehren" hinter dem Regelknoten durchgeführt, sodass sie im Bild am falschen Knoten zu enden schien — dieselbe Sorte Befund, die der Prüfer an der alten Fassung erhoben hat. Der `subgraph`-Kasten um die geteilten Knoten ist deshalb wieder herausgenommen worden.

Kein `cargo`-Lauf, kein Vordergrundlauf, kein Bündelbau. `target/KRK.app` ist unangetastet.

## Was offen bleibt

Die vier Nutzerfragen vom 260813-0053 stehen weiter offen; ein fünfter Datensatz ist nicht entstanden, weil sich jede Frage dieses Nachzugs ableiten ließ. Die Runde fährt weiter auf den vier Empfehlungen. Nächster Schritt ist die Planung.
