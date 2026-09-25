Kopfkommentar der Auslieferungsbelegung nennt C10 nicht

---

`resources/default-keymap.toml` sagt in seinem Kopfkommentar, Zeile 8 bis 9:

    # Quelle: Spec `260802-1036_o_spec-navigator-geruest.md`, Faehigkeiten C1 bis
    # C7.

Seit S9b trägt die Datei zwei Funktionen aus C10, der Zwischenablage als
Quelle. Die Herkunftsangabe ist damit unvollständig; sie müsste "C1 bis C7
sowie C10" heißen.

---

Nicht mit S9b behoben, weil der Auftrag den Eingriff ausdrücklich auf drei neue
Blöcke begrenzt hat und der heutige Stand der Datei vom Nutzer abgenommen ist.
Die Behebung ist eine Zeile und gehört dem `ontocoder`; sie braucht nur die
Freigabe, einen vorhandenen Kommentar anzufassen.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260804-0907-s9b-drei-kombinationen-nachgetragen.md`

---
Resolved: Die Herkunftsangabe im Kopf von `resources/default-keymap.toml` heisst jetzt "Faehigkeiten C1 bis C7 sowie C10". Mitgenommen bei der Umsetzung von Schritt 11c am 260804-1214, weil derselbe Textblock ohnehin angefasst wurde.
