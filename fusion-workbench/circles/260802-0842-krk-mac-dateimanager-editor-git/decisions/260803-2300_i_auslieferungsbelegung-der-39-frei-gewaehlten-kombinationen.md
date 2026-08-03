# Welche Kombinationen tragen die 39 Funktionen, die der Spec nicht festlegt?

---
**Domain:** data
**Status:** implemented
**Filed by:** planner
**Cross-references:** `planning/260802-1036_o_spec-navigator-geruest.md` (C3, Abschnitt "Getroffene Festlegungen"), `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 9), `history/260803-2045-auslieferungsbelegung-als-datentabelle.md`, `issues/260803-2045_c_c3-nennt-f6-verschieben-und-umbenennen-die-belegungstabelle-nur-verschieben.md`

---

## Frage

`resources/default-keymap.toml` führt 46 Funktionen mit zusammen 52 Kombinationen. Sieben dieser Belegungen schreibt der Spec fest: die sechs Zeilen der Kürzel-Tabelle in C3 und F4 als unbelegt. Für die übrigen 39 sagt der Spec nur "ein Tastenbefehl", ohne einen zu nennen. Der `ontocoder` hat sie beim Schreiben der Datei gewählt und die Wahl im Datensatz `history/260803-2045-auslieferungsbelegung-als-datentabelle.md` offengelegt. Damit stand eine Frage offen, die keine technische ist: trägt die Anwendung diese 39 Kombinationen so aus, wie sie jetzt in der Datei stehen?

Die Frage musste vor S11 beantwortet werden. Die Belegungsmaschine bindet die Datei über `include_str!` ein und macht sie damit zum Auslieferungszustand, auf den der Zurücksetzen-Befehl aus C3 zurückführt. Jede spätere Änderung an einer der 39 Kombinationen ändert ab dann das Verhalten einer bereits abgenommenen Fähigkeit.

## Optionen

1. **Die Datei annehmen, wie sie ist** — die 39 Kombinationen gelten für diese Runde als gewählt und werden nicht einzeln durchverhandelt.
   - Pro: Die Runde kommt weiter. Die drei Wahlregeln unten sind nachvollziehbar und in der Datei kommentiert, wo sie von der Mac-Gewohnheit abweichen. Eine Belegung, die dem Nutzer im Gebrauch nicht liegt, ändert er in der Belegungsansicht aus C3, ohne dass eine Zeile Code fällt.
   - Contra: Die Annahme beruht auf einer Durchsicht, nicht auf einem Gebrauch. Erst die laufende Anwendung zeigt, welche Kombination im Alltag stört.
2. **Die 39 Kombinationen einzeln entscheiden** — jede Belegung wird dem Nutzer vorgelegt und in den Spec geschrieben.
   - Pro: Jede Belegung ist ausdrücklich gewollt.
   - Contra: 39 Einzelfragen vor der ersten bedienbaren Fassung, ohne dass der Nutzer die Anwendung in der Hand hatte. Der Spec verdoppelte dabei die Datendatei, womit die Belegung zwei Quellen hätte.
3. **Die Wahl zurückstellen und die Datei vorläufig unbelegt lassen** — nur die sieben festgeschriebenen Belegungen ausliefern.
   - Pro: Keine ungeprüfte Zusage.
   - Contra: Verletzt das erste Abnahmekriterium von C2, das für jede Funktion aus C1 bis C7 mindestens einen Tastenbefehl verlangt. S11 bis S20 hätten keine Kommandos zum Nachschlagen.

## Constraints

- Die Belegung hat genau eine Quelle. Der Spec darf die Datendatei nicht verdoppeln, sonst laufen beide beim ersten Nachzug auseinander.
- Jede Funktion aus C1 bis C7 trägt mindestens einen Tastenbefehl (C2, erstes Abnahmekriterium).
- Die Auslieferungsbelegung ist in sich konfliktfrei (C3).
- Die Kombinationsschreibweise ist `[ctrl+][opt+][shift+][cmd+]<taste>` in dieser Reihenfolge (Plan, Schritt 9).

## Antwort des Nutzers

**Möglichkeit 1, angenommen am 260803-2110.** Der Nutzer hat die vollständige Auslieferungsbelegung vorgelegt bekommen, alle 46 Funktionen durchgesehen und sie angenommen. Seine Formulierung war "passt erstmal so".

Angenommen ist damit der Dateibestand von `resources/default-keymap.toml` im Stand des Commits `d1a8ab1` ("feat(keymap): S9 Auslieferungsbelegung als Datentabelle", 2026-08-03): 46 Funktionen, 52 Kombinationen, keine doppelt vergeben.

**Das "erstmal" gehört zur Antwort und ist hier mitfestgehalten.** Die Annahme gilt für diese Runde. Sie ist keine Festschreibung auf Dauer und kein Grund, eine spätere Änderung als Rückschritt zu behandeln. Wer eine der 39 Kombinationen ändern will, ändert die Datei und schreibt einen Datensatz, der diesen hier überholt; ein neuer Entscheidungsgang über die übrigen 38 entsteht dadurch nicht.

**Was der Spec selbst festlegt und was nicht.** Sieben der 46 Belegungen stehen im Spec: die sechs Zeilen der Tabelle "Die ausgelieferten Cmd-Kürzel" in C3, also F3 mit Cmd+Y, F5 mit Cmd+Shift+K, F6 mit Cmd+Shift+V, F7 mit Cmd+Shift+N, F8 mit Cmd+Opt+Delete und Delete mit Cmd+Delete, dazu F4 als benannt und unbelegt. Die übrigen 39 stammen nicht aus dem Spec, sondern aus dieser Annahme.

**Die drei Regeln, nach denen der `ontocoder` gewählt hat**, gelten mit angenommen und binden künftige Ergänzungen der Datei:

1. Wo der Mac für dieselbe Sache ein Kürzel kennt, steht es unverändert: `cmd+up` für den Aufstieg, `shift+cmd+g` für die Pfadeingabe, `cmd+a` für "alle markieren", `cmd+t` und `cmd+w` für die Tabs, `cmd+d` für das Lesezeichen.
2. Sonst, wo Norton Commander oder Total Commander eine Form haben, die auf dem Mac frei ist: `tab` für den Fensterwechsel, `space` für das Markieren, `shift+f6` für das Umbenennen.
3. Sonst der Anfangsbuchstabe des deutschen Verbs, wie C3 es mit `shift+cmd+k` und `shift+cmd+v` vormacht: `ctrl+b` breiter, `ctrl+s` schmaler, `shift+cmd+u` umbenennen, `cmd+r` Sortierrichtung.

## Was diese Annahme nicht abdeckt

Zwei Punkte an derselben Datei sind mit ihr nicht entschieden und stehen weiter offen:

- `issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md` — Cmd+W hat zwei Parteien, und die eine steht im Menü, wo die Konflikterkennung aus C3 sie nicht sieht. Der Punkt hängt an `decisions/260803-2007_o_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`.
- `issues/260803-2045_o_die-kombinationsschreibweise-kennt-die-links-und-rechts-pfeile-nicht.md` — die erlaubten Tastennamen decken drei naheliegende Mac-Belegungen nicht ab. Der Nutzer nimmt die Lücke für diese Runde in Kauf; behoben ist sie damit nicht.

---
Implemented: `d1a8ab1` — `resources/default-keymap.toml` trägt die 46 Funktionen mit den 52 Kombinationen, die diese Annahme abdeckt. Der Dateibestand ist seither unverändert (`git diff d1a8ab1 HEAD -- resources/default-keymap.toml` ist leer, geprüft am 260803-2300). Antwort und Umsetzung fallen zusammen, weil der Nutzer eine bereits geschriebene Datei durchgesehen und angenommen hat; ein Zwischenstand "beantwortet, noch nicht umgesetzt" hat nie bestanden.
