# Schließt C4 die Schreibwerkzeuge aus macOS 15 am Editor aus?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `issues/260810-0512_*_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`, `issues/260810-0745_*_der-stolperdraht-sieht-drei-der-vier-schreibwerkzeug-einstellungen-nicht.md`, `crates/krk-ui/src/appkit/editor.rs` (Modulkopf, `EINSTELLUNGEN`), Spec C4

---

## Question

Der Defekt `260810-0512` hat die Frage aufgeworfen und ausdrücklich nicht
entschieden: er sagt, sie gehöre dem Nutzer und nicht dem Übersetzer, und sie
binde über einen einzelnen Schalter hinaus. Deshalb steht sie hier und nicht
weiter als Defekt.

Die Zusage aus C4 lautet, dass der gesicherte Stand der getippte ist. Die
Schreibwerkzeuge aus macOS 15 ersetzen markierten Text durch umgeschriebenen,
und das Korrekturlesen wendet seine Änderungen über eine ganze Datei an. Was
danach in `NSTextView::string` steht, ist nicht mehr das Getippte, und über
`Editormodell::stand` geht es beim Sichern in die Datei.

Sie unterscheiden sich von den sieben abgeschalteten Automatiken **in der Art
und nicht im Grad**: die sieben greifen ohne Zutun des Nutzers, die
Schreibwerkzeuge auf seinen ausdrücklichen Aufruf aus dem Kontextmenü oder dem
Menü *Bearbeiten*. Genau dieser Unterschied entscheidet die Frage nicht,
sondern stellt sie:

- Liest man C4 als „kein Zeichen ohne Zutun des Nutzers", dann sind die
  Schreibwerkzeuge zulässig und bleiben an.
- Liest man C4 als „der gesicherte Stand ist der getippte", dann sind sie es
  nicht.

Der Editor ist außerdem ein **Programmtext**-Editor: ein Umschreiben von
Programmtext in flüssigere Prosa ist dort in keiner Lesart gemeint, und das
Kontextmenü steht an jeder Textfläche ohne Zutun.

**Es sind vier Einstellungen und nicht eine.** Wer die Schreibwerkzeuge
ausschließt, schließt sie über `writingToolsBehavior` allein nicht aus
(`260810-0745`):

| Einstellung | Wert an KRKs Fläche | gemessen |
|---|---|---|
| `writingToolsBehavior` | `NSWritingToolsBehaviorDefault` (0) | ja |
| `allowsWritingToolsAffordance` | **an** (1) | ja |
| `allowedWritingToolsResultOptions` | 0 | ja |
| `writingToolsAllowedInputOptions` | 0 | ja |

Alle vier sind am 260810 an der Fläche aus `textflaeche_bauen` gemessen, auf
macOS 15.7.7 (Build 24G720), von den Proben
`der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl` und
`keine_unbekannte_einstellung_steht_an_der_textflaeche` in
`crates/krk-ui/src/appkit/editor.rs`. Der Vorgabewert war bis dahin nur
vermutet.

## Options

1. **Ausschließen** — `setWritingToolsBehavior(None)` in `textflaeche_bauen`,
   dazu die drei übrigen auf den Wert, der nichts zulässt.
   - Pro: C4 gilt dann ohne Vorbehalt; der gesicherte Stand ist der getippte,
     auch nach einem Kontextmenü-Aufruf, den der Nutzer nicht überblickt.
   - Contra: nimmt dem Nutzer eine Systemfunktion, die er ausdrücklich
     aufgerufen hat; in einer Markdown-Datei kann sie gewollt sein.
2. **Anlassen** — keine Zeile, und die Einordnung in `EINSTELLUNGEN` geht von
   `NochOffen` auf `Geduldet`, mit dem Grund „der Nutzer ruft sie eigens auf".
   - Pro: der ausdrückliche Aufruf ist eine Eingabe und keine Automatik, und
     dieselbe Begründung trägt schon die Textvervollständigung.
   - Contra: das Korrekturlesen wirkt über eine ganze Datei, und ein Nutzer,
     der es an Programmtext auslöst, sieht die Folgen nicht auf einen Blick.
3. **Nach Dateityp** — an in Prosadateien, aus in Programmtext.
   - Pro: trifft die Absicht am genauesten.
   - Contra: eine dritte Stelle mit einer Meinung über Dateitypen neben
     `Editormodell` und `hervorhebung`, und die Einstellung müsste je
     Dateiwechsel nachgezogen werden. Kein Abnahmekriterium verlangt es.

## Constraints

Jede Antwort muss alle vier Einstellungen benennen, nicht nur die erste. Fällt
sie gegen die Schreibwerkzeuge, ist die Einordnung in `EINSTELLUNGEN`
`Abgeschaltet`, und die Probe
`die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` verlangt dann
die zugehörigen Zeilen in `textflaeche_bauen` — sie schlägt fehl, solange die
Einordnung und der Code auseinanderlaufen.

## Recommendation

Option 1, ausschließen. Der Grund ist nicht das Zutun des Nutzers, sondern der
Gegenstand: der Editor hält Programmtext, und die Umschreibung von Programmtext
ist in keiner Lesart von C4 gemeint. Die Fähigkeit ist damit nicht verloren —
der Nutzer hat sie in jedem Textfeld des Systems —, sie steht nur nicht an einer
Fläche, deren Inhalt Zeichen für Zeichen in eine Datei zurückgeschrieben wird.

---
Answered: **Option 1, ausschließen.** Ein Editor für Code und Text darf Text nicht
stillschweigend umschreiben lassen, und C4 verlangt dasselbe schon für die anderen
textverändernden Automatiken. Die Zusage C4 wird also so gelesen, dass sie die
Schreibwerkzeuge aus macOS 15 mit einschließt. Entschieden vom Nutzer über den
Orchestrator am 260810.

Implemented: `crates/krk-ui/src/appkit/editor.rs`, `textflaeche_bauen` — und die
Umsetzung fällt anders aus, als die Option 1 sie beschrieb. Sie verlangte
`setWritingToolsBehavior(None)` „dazu die drei übrigen auf den Wert, der nichts
zulässt". **Diesen Wert führen zwei der vier nicht**, und das ist am 260810 auf
macOS 15.7.7 gemessen, nicht der Dokumentation entnommen:

| Einstellung | Werkswert | Aus-Wert | Zeile in `textflaeche_bauen` |
|---|---|---|---|
| `writingToolsBehavior` | `Default` (0) | `None` (**−1**) | ja, unmittelbar |
| `allowsWritingToolsAffordance` | an (1) | 0 | ja, gehütet |
| `allowedWritingToolsResultOptions` | 0 | **keiner** | nein |
| `writingToolsAllowedInputOptions` | 0 | **keiner** | nein |

Die beiden letzten sind Bitmasken, deren Null `NSWritingToolsResultDefault` heißt —
„das System wählt" — und nicht „nichts". Einen Wert, der nichts zulässt, führt die
Aufzählung nicht, und beide stehen ab Werk schon auf Null; eine Zeile wäre ein
Aufruf ohne Wirkung gewesen. Sie stehen deshalb als neue, fünfte Antwort
`Einordnung::Gegenstandslos("setWritingToolsBehavior:")` in `EINSTELLUNGEN`: sie
beschreiben, **was** eine Fähigkeit dürfte, die an dieser Fläche abgeschaltet ist.
**Dass sie dabei keine zweiten Türen sind, ist mitgemessen** —
`setWritingToolsBehavior(None)` lässt beide unberührt stehen, legt sie also nicht um.

Die Entscheidung selbst ist davon nicht berührt: die Schreibwerkzeuge sind
ausgeschlossen, und was sie ausschließt, sind die beiden oberen Zeilen.

**Die Untergrenze ist gewahrt.** `setWritingToolsBehavior:` steht seit macOS 15.0
und wird unmittelbar gerufen. `setAllowsWritingToolsAffordance:` führt das SDK erst
ab macOS 15.4 und nur an `NSTextField`; sie geht über die neue Funktion
`setzen_falls_vorhanden`, die `respondsToSelector:` **vorher** fragt — dasselbe
Muster, das `merkmal_falls_vorhanden` schon las, und kein zweites daneben. Beide
bilden den Setzernamen jetzt über dieselbe Funktion `setzername`, die dafür aus
`mod tests` in den Modulrumpf gewandert ist.

**Vier Proben tragen es**, alle an einer echten `NSTextView`:
- `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` (vorher
  `die_sieben_…`) prüft jetzt **neun** statt sieben Einstellungen. Sie liest über
  `merkmal_falls_vorhanden` statt `merkmal`, weil der Abbruch bei einer an macOS
  15.0 bis 15.3 fehlenden Angebotsfläche das ganze Prüfprogramm beendet hätte;
  fehlt sie, steht ein Hinweis und kein Fehlschlag.
- `die_gegenstandslosen_stehen_unberuehrt_und_ihr_traeger_steht_aus` ist neu und
  hält fest, dass KRK die beiden Bitmasken nicht setzt und sie auf ihrer Null stehen.
- `jeder_verweis_zeigt_auf_beantwortete_einstellungen` (vorher `jede_tuer_…`) deckt
  die neue Antwort mit ab: der genannte Träger muss selbst `Abgeschaltet` sein.
- `der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl` misst jetzt
  die **frische** Fläche statt KRKs, weil KRKs beide jetzt aus stehen. Die Aussage
  ist damit: die beiden Zeilen ändern etwas und wiederholen keinen Werkswert.

`aus_bedeutet` trägt zwei neue Formen (`Behavior:` → −1, `Affordance:` → 0);
`Options:` bleibt mit Absicht im Abbruchzweig, denn dort wäre ein Aus-Wert zu
erfinden, den Apple nicht führt. `Einordnung::NochOffen` steht heute leer und bleibt
als Variante stehen, weil die nächste Lesart wieder eine braucht. Der Modulkopf ist
nachgezogen: neun statt sieben, ein eigener Abschnitt zu den Schreibwerkzeugen, und
die Verfügbarkeitsliste nennt jetzt fünf Methoden mit der einen gehüteten Berührung.

Abnahme: `cargo build --workspace`, `cargo test --workspace` (16 Probenprogramme,
334 davon in `krk-ui`), `cargo clippy --workspace --all-targets`,
`cargo fmt --all --check` — je Exit 0. Mit einem Commit-Hash kann diese Sitzung
nicht dienen.

Deferred:
Superseded by:
