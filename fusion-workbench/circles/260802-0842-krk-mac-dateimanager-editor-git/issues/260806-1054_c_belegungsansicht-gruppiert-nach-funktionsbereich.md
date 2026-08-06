Die Belegungsansicht zeigt ihre Zeilen ungegliedert statt nach Funktionsbereich

---

Nutzerauftrag vom 260806: die Belegungsansicht aus S20 soll ihre Zeilen
strukturiert nach Funktionsbereich anzeigen (Dateilisting, Editor, ...).
Heute listet sie alle Funktionen in einer flachen Folge, eine Zeile je
Funktion. Gewünscht sind sichtbare Gruppen, damit die Belegung eines
Bereichs auf einen Blick beisammensteht.

---

Erweiterung zu C3, kein Defekt im engeren Sinn. Die Gruppierung ist eine
Anzeigefrage der Ansicht; die Belegungsmaschine und `keymap.toml` bleiben
unberührt. Die Zuordnung Funktion → Bereich gehört in den Code der Ansicht
oder des Modells, nicht als neues Datenfeld in `resources/default-keymap.toml`
(das wäre ontocoder-Gebiet und braucht erst eine Entscheidung, falls die
Code-Zuordnung nicht trägt).

---
Resolved: Die Belegungsansicht gliedert ihre Zeilen jetzt nach neun Funktionsbereichen (Dateilisting, Dateioperationen, Tabs, Vorschau, Leiste und Fokus, Fenster, Anwendung, Textbefehle, Editor). Zuordnung an einer Stelle im Modell (belegungsmodell.rs, vollständige Fallunterscheidung ohne Auffangzweig, 12 Prüfungen), Überschriften als nicht auswählbare Gruppenzeilen; resources/default-keymap.toml unberührt. Am Bündel geprüft.
