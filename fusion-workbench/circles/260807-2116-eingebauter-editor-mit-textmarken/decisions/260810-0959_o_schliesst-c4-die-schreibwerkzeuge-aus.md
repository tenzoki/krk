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
Answered:
Implemented:
Deferred:
Superseded by:
