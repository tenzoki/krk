# Schritt 3: Die Textanzeige der Vorschau wird eine eigene Klasse und auswählbar

**Agent:** coder
**Datum:** 2026-08-20
**Plan:** `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel B, Schritt 3
**Status:** Complete

---

## Was gebaut ist

Eine Datei berührt: `crates/krk-ui/src/appkit/vorschau.rs`.

- **`Vorschautext`** als Unterklasse von `NSTextView` über `define_class!`, mit `#[thread_kind = MainThreadOnly]` und dem Merkposten `RefCell<Option<Arc<Quellbezug>>>`. Vorbild ist `Inhaltsflaeche` in derselben Datei, einschließlich der Erzeugung über `msg_send![super(this), initWithFrame: rahmen]` in `Vorschautext::neu`. **Keine Überschreibung** — `writeSelectionToPasteboard:types:` gehört Schritt 7.
- Zwei Methoden ohne Objective-C-Berührung: `quellbezug_setzen` und `quellbezug`. Beide tragen bis zu ihrem Rufer (Schritt 5 und Schritt 7) je ein `#[expect(dead_code, reason = …)]`; ohne die Zeilen hält `-D warnings` den Bau an. `expect` und nicht `allow`, damit die Ausnahme ihr Ablaufdatum selbst durchsetzt. **Ohne `cfg_attr(not(test), …)`**, anders als das Vorbild in `kommandos/rueckschritt.rs`: dort nutzten die Proben die Stücke schon, hier nutzt sie auch der Probenbau nicht, und die Erwartung wäre unter `cfg(test)` unerfüllt geblieben.
- `textanzeige` baut die neue Klasse statt einer nackten `NSTextView` und setzt **`setSelectable(true)`**. `setEditable(false)` steht unverändert.
- `VorschaufensterIvars::text` wechselt den Typ auf `Retained<Vorschautext>`. Keine weitere Anpassung nötig: jede Berührung — `setString`, `setDelegate`, `textmerkmale::zuruecksetzen`, `textmerkmale::anwenden`, `Nummernspalte::einhaengen` — läuft über die Ableitung auf `NSTextView`.

## Die abgelöste Zusage

`setSelectable(false)` war das achte Abnahmekriterium von C4 der Runde 6, vom Nutzer am 260819 **ersetzt und nicht ergänzt**. Drei Prosastellen in dieser Datei stehen jetzt auf dem neuen Stand:

1. Der Modulkopf: der Abschnitt, der die Nichtauswählbarkeit begründete, ist durch `# Die Textanzeige ist auswählbar, und das löst eine Zusage ab` ersetzt. Er sagt, dass der alte Grund nicht widerlegt, sondern bezahlt ist, und dass `setEditable(false)` **aus einem anderen Grund** stehen bleibt: die Nichtauswählbarkeit war ein Mittel gegen den Fokus, die Nichtbearbeitbarkeit ist eine Aussage darüber, was die Vorschau ist (C1.4).
2. Der Doc-Kommentar von `textanzeige`: dieselbe Unterscheidung, an der Stelle, an der beide Zeilen nebeneinanderstehen.
3. Der Doc-Kommentar von `Inhaltsflaeche`: er sagte, die Textanzeige lehne den Fokus ab. Jetzt sagt er, warum die Fläche trotzdem bleibt — Erscheinungsbildmeldung und der Klick auf Bild und leeren Rand.

Dazu das ASCII-Bild im Kopf und der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`: `NSTextView` (`NSTextView.h:76`), dessen `initWithFrame:` (`:86`) sowie `setEditable:` und `setSelectable:` als Setzer der Eigenschaften `editable` und `selectable` von `NSText` (`NSText.h:89-90`). Keine der vier trägt im Kopf des Systems eine Verfügbarkeitsangabe und steht damit seit 10.0. Am SDK `MacOSX26.2.sdk` gelesen, nicht geraten.

## Die Probe

`die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort`, eine Zählprobe über `crate::quellbaum` im Prüfmodul der Datei. **Keine Probe, die eine Instanz baut** — `krk-ui` hat kein Bibliotheksziel, und eine Probe, die dafür den Hauptfaden behauptet, wäre der Defekt `260810-1001`.

Sie weicht in der Erwartung vom Plantext ab; warum, steht im Datensatz `issues/260820-0604_c_die-zaehlprobe-aus-schritt-3-kann-nicht-null-erwarten-…`.

## Zwei Datensätze abgelegt

- `issues/260820-0604_o_der-modulkopf-von-textautomatik-nennt-die-vorschau-nicht-auswaehlbar-…` — eine fünfte Prosastelle, die der Plan nicht führt. Außerhalb der Dateiliste dieses Schrittes, gehört in Schritt 8.
- `issues/260820-0604_c_die-zaehlprobe-aus-schritt-3-kann-nicht-null-erwarten-…` — im Schritt selbst gelöst.

## Prüfung

`make check` — exit 0 (build, test, fmt-check, lint mit `-D warnings`).

Der erste Lauf brach in `crates/krk-ui/src/markdown.rs` ab, der Datei eines **gleichzeitig laufenden** zweiten `coder` (Schritt 2). Der Schritt ist deshalb zusätzlich gegen eine Kopie des Baums mit `markdown.rs` auf dem Stand `13be459` geprüft worden; dort liefen alle vier grün, bevor der Lauf am echten Baum es tat. Dass `make check` den ganzen Arbeitsbereich prüft und bei parallelen Agenten an fremden Dateien abbricht, ist als `shared/issues/260820-0602_o_…` bereits erfasst.

**Nicht committet** — die Vorlage schreibt es dem Orchestrator zu.
