Die Kombinationsschreibweise kennt die Links- und Rechts-Pfeile nicht, und drei Funktionen brauchen sie

---

Schritt 9 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` zählt die erlaubten Tastennamen auf: "`f3` bis `f8`, `delete`, `up`, `down`, `pageup`, `pagedown`, `home`, `end`, `return`, `tab`, `esc`, `space` sowie Buchstaben und Ziffern". Die Liste enthält die Pfeile nach oben und unten, aber nicht die nach links und rechts, und kein Satzzeichen.

Drei Belegungen, die auf einem Mac naheliegen, lassen sich damit nicht schreiben:

1. C7 verlangt, dass sich die Bereiche "über einen Tastenbefehl schrittweise verbreitern und verschmälern" lassen. Die Bereiche stehen nebeneinander; die Richtung dafür sind die Links- und Rechts-Pfeile.
2. C2 verlangt einen Befehl, der versteckte Dateien ein- und ausblendet. Der Finder legt das auf Cmd+Umschalt+Punkt; die Punkt-Taste hat keinen Namen.
3. C3 nennt eine eigene Belegungsansicht. Der Mac-übliche Ort dafür ist Cmd+Komma; die Komma-Taste hat ebenfalls keinen Namen.

---

Herkunft: gefunden beim Schreiben von `resources/default-keymap.toml` (Plan Schritt 9).

Wie die Datei damit umgeht: das Verbreitern und Verschmälern liegt auf `ctrl+b` und `ctrl+s`, den Anfangsbuchstaben von "breiter" und "schmaler"; die versteckten Dateien liegen auf `shift+cmd+h`. Beide Stellen tragen einen Kommentar mit dem Grund. Die Belegungsansicht steht gar nicht in der Datei: sie gehört als Einstellungen-Eintrag ins Anwendungsmenü, wo Cmd+Komma als Menükürzel erreichbar ist, ohne dass die Schreibweise es ausdrücken muss.

Was zu entscheiden ist: ob die Schreibweise um `left`, `right` und einige Satzzeichen wächst. Der Ort dafür ist Schritt 11, der den Parser schreibt; die Erweiterung kostet dort Tabelleneinträge und in `resources/default-keymap.toml` drei geänderte Zeilen. Solange sie ausbleibt, tragen die drei Funktionen Behelfsbelegungen, die der Nutzer nach C3 jederzeit selbst umbelegen kann.
