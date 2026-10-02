# Shaper: Verzeichnis angleichen und Abwurf aus fremden Anwendungen

**Datum:** 2026-08-18
**Agent:** shaper (Modus: user-direct, dritte Dispatchierung)
**Baumstand:** `8d5baf6`

## Auftrag

Den Spec zu den zwei Wünschen des Nutzers vom 260818 schreiben. Die beiden vorigen Dispatchierungen haben die Klärungsrunden gefahren; alle Fragen waren beantwortet. Diese Dispatchierung hatte keinen Zugriff auf `AskUserQuestion` und keine Erinnerung an die vorigen; sämtliche übernommenen Befunde sind gegen den Baum nachgeprüft.

## Was geschrieben wurde

- `shared/planning/260818-1510_o_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md` — sieben Fähigkeiten, zwei Mermaid-Diagramme, Abgrenzung, acht Punkte für den Planner, keine ausstehende Nutzerentscheidung.
- `shared/decisions/260818-1453_o_…` → `_a_`, mit `Answered:`-Zeile auf C5 des Specs und `**Status:** answered`.

## Was nachgeprüft wurde, und was dabei anders ausfiel

Gehalten haben: keine Ziehunterstützung im Baum (null Treffer über `crates/` für `registerForDraggedTypes`, `draggingEntered`, `performDragOperation`, `mouseDragged`, `NSDragOperation`, `beginDraggingSession`, `NSFilePromise`); `Kommando` trägt 78 Varianten und `KENNUNGEN` die Länge 78 im Typ; `opt+cmd+s` ist ab Werk frei; `Auftrag` nimmt beliebige Quellpfade und `Konfliktregel::Fragen` ist die Vorgabe; `Tabliste::ordner_setzen` trägt Sortierung, ausgeblendete Einträge, „Deep", Inhaltsfilter und Filtertext unverändert hinüber; alle berührten AppKit-Schnittstellen stehen unter macOS 15.

Eine Angabe aus dem Auftrag stimmt so nicht:

1. **`ordner_lesen` hat nicht drei Rufer, sondern elf Aufrufstellen** in `tabelle.rs` und `anwendung.rs`. Der Spec nennt deshalb keine Zahl und sagt allein, dass die Funktion ein Dateifenster auf einen Ordner stellt.

Zwei Befunde kamen bei der Prüfung hinzu:

2. **`Anwendungsdelegierter::bereich_einblenden` besteht bereits** (`anwendung.rs:3862`) und ist der Weg für Befehle, die einen Bereich brauchen statt ihn umzuschalten. C2 des Specs nennt ihn.
3. **`Fenstermodell::einblenden` liefert `false` in zwei verschiedenen Lagen**, „war schon sichtbar" und „passt nicht hinein". Nur die zweite ist eine Abweisung. Der Spec schreibt daraus eine eigene Festlegung: der Befehl fragt die Sichtbarkeit vorher.

Sämtliche Zeilenangaben des Specs sind einzeln gegen `8d5baf6` gelesen: `bereich_einblenden` 3862, `uebertragen` 4428, `auftrag_stellen` 5302, „Quelle und Ziel sind derselbe Ordner" 5320, `vorgang_laeuft_schon` 5348, `ordner_lesen` `tabelle.rs:853`, `einblenden` `fenstermodell.rs:735`.

## Was der Shaper selbst festgelegt hat

Vier Festlegungen, alle am Spec-Gate umstoßbar und im Abschnitt „Abgeleitet und nicht gefragt" mit ihrer Begründung: `opt+cmd+s`, die zwei Abwurfmarken aus AppKit, der unveränderte Fokus, und die Abweisung eines Abwurfs in den eigenen Quellordner. Die vierte war im Auftrag nicht vorgesehen und folgt der bestehenden Meldung aus `auftrag_stellen`.

Daneben zwei Ableitungen im Text der Fähigkeiten: der Befehl bleibt wirkungslos, wenn das andere Dateifenster den Ordner schon zeigt (C1), und KRK meldet die Zusagesorten mit an, um eine Zusagedatei überhaupt abweisen und melden zu können (C7).

## Was nicht getan wurde

Kein Circle angelegt, kein Plan geschrieben, kein Agent dispatchiert. Der Circle dieser Runde und die Aktivierung sind Sache des Nutzers oder des Orchestrators.
