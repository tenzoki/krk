# Die Oberfläche folgt der Systemsprache: Deutsch, Französisch, Englisch als Rückfall

---
**Domain:** code
**Status:** claimed
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260930-2319
**Mode:** autonomous
**Active spec/plan:** 261001-0735_*_spec-oberflaeche-lokalisierbar-deutsch-und-franzoesisch.md (Spec), 261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md (Plan)
**Cross-references:** 260930-1914-kontextmenue-traegt-duplicate-mit-namensdialog.md
**Filed by:** user, Kai Stalmann <kai@stalmann.org>

---

## Directive

"Plane, spezifiziere, implementiere im auto-Modus ein neues Arbeitspaket zu i8n. Alles was jetzt im UI nur auf Detusch erscheint, sollte lokalisiert werden können: außer Deutsch biete msl noch Französisch an. Auswhal im Krk Menu." Auf die drei Klärungsfragen des Spec-Entwurfs vom 261001-0735 (Vorgabesprache und Menü; Zeitpunkt und Ort der Wahl; das Feld `name` der eigenen `keymap.toml`): "Ja, ne, wir machen das noch andders: folge der Systemsprache, Fallback auf English. Führe English also auch ein." Auf die Nachfrage, ob die Auswahl im Menü „KRK“ bleibt (A) und was aus dem Feld `name` wird (B): "a1 b1", also: die Sprache folgt allein dem System, kein Menüeintrag und kein Wert in `settings.toml`; der Befehlsname kommt immer aus der Sprachtabelle. Erreicht ist das, wenn jede Zeichenkette, die ein Mensch durch KRKs Oberfläche liest, aus einer Lokalisierung kommt statt fest im Code zu stehen, Deutsch, Französisch und Englisch als Sprachen vorliegen, KRK der Systemsprache folgt und bei jeder anderen auf Englisch zurückfällt; was der Spec darüber hinaus festlegt, gilt mit ihm.
