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

---
Resolved: Die Frage dieses Defekts, "ob die Schreibweise um `left`, `right` und einige Satzzeichen wächst", ist am 260804-1122 entschieden und steht als Schritt S11b im Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`. Ausgelöst hat die Entscheidung der Nutzerentscheid vom selben Tag: der Auf- und Abstieg im Verzeichnisbaum liegt jetzt auf `cmd+left` und `cmd+right`, und die Belegungsansicht auf `f1`. Zwei dieser drei Kombinationen ließen sich in der Schreibweise nicht einmal hinschreiben; die Lücke bindet damit, was sie am 260803 noch nicht tat.

Entschieden ist dreierlei. Erstens wächst die Schreibweise um acht Namen: `left` und `right` schließen den Pfeilblock, `f1`, `f2` und `f9` bis `f12` schließen die Funktionstastenreihe. Die Regel dahinter steht in S11b: die Schreibweise wächst um ganze Tastengruppen, nie um einzelne Tasten, weil eine halbe Gruppe genau den Zustand herstellt, den dieser Defekt meldet. Zweitens bleiben die Satzzeichen draußen. Ein virtueller Tastencode benennt eine Stelle auf der Tastatur, und bei den Satzzeichen läuft die Beschriftung dieser Stelle je nach Tastaturbelegung weit auseinander: `kVK_ANSI_LeftBracket` trägt auf einer deutschen Tastatur ein `ü`, `kVK_ANSI_Quote` ein `ä`. Ein Name `bracketleft` in der Belegungsdatei bezeichnete für einen deutschen Nutzer eine Taste, die er nicht findet. Drittens erledigen sich damit zwei der drei Punkte oben: die Belegungsansicht erreicht der Nutzer ab S11c über `f1` und braucht kein Komma, und die versteckten Dateien bleiben auf `shift+cmd+h`.

Der erste Punkt oben, die Bereichsbreiten auf `ctrl+b` und `ctrl+s`, bleibt als Belegung bestehen und ist keine Lücke mehr, sondern eine Wahl. Beide gehören zu den 39 Kombinationen, die der Nutzer am 260803-2110 angenommen hat; sie ohne ihn zu ändern verstieße gegen jene Annahme. Ob sie auf `ctrl+left` und `ctrl+right` wandern, steht als `decisions/260804-1122_o_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md`.

Die Codeänderung selbst ist S11b, die Datenänderung S11c. Beide sind sofort lauffähig.
