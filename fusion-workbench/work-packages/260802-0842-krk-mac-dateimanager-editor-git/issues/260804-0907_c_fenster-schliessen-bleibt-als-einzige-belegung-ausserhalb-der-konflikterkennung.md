"Fenster schließen" bleibt als einzige Belegung außerhalb der Konflikterkennung

---

Seit S9b steht `fenster_einblenden` mit `cmd+n` in
`resources/default-keymap.toml`. Sein Gegenstück, der Menüeintrag "Fenster
schließen" auf Shift+Cmd+W, steht dort nicht und soll dort nach dem
beschlossenen Weg auch nicht stehen. Damit hat KRK genau eine Tastenkombination,
die eine Funktion auslöst und die weder die Konflikterkennung aus C3 sieht noch
der Nutzer umbelegen kann.

---

Nicht dieselbe Sache wie
`issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`.
Jener Defekt hält den Zusammenstoß auf `cmd+w` fest und ist in der Sache
entschieden: Cmd+W bleibt beim Tab, der Menüeintrag wandert auf Shift+Cmd+W,
und die Belegungsdatei bleibt unverändert. Was diese Auflösung nicht behandelt,
ist die Folge für die neue Kombination: Shift+Cmd+W ist heute in keiner
Tastenliste, und nichts hindert einen späteren Eintrag daran, sie zu nehmen.
Die Konflikterkennung aus Schritt 11 prüft die Belegungsdatei gegen sich
selbst; ein Menükürzel ist für sie nicht vorhanden. Genau diese Blindstelle
benennt der ältere Defekt in seinem ersten Absatz, und die Auflösung vom
260804 lässt sie stehen, statt sie zu schließen.

Der Vergleich mit dem Nachbarn macht es sichtbar: das Einblenden des Fensters
liegt in der Datei, das Schließen nicht, obwohl beide dieselbe Sorte Funktion
sind und C3 für jede Funktion aus C1 bis C7 mindestens einen umbelegbaren
Tastenbefehl verlangt.

Zwei Wege stehen offen, und beide gehören vor Schritt 12 entschieden, der den
Menüeintrag anfasst:

1. **`fenster_schliessen` als Eintrag in `resources/default-keymap.toml`
   aufnehmen** und das Menü sein Kürzel von dort beziehen lassen. Dann gilt
   eine Quelle für alle Kombinationen, die Konflikterkennung sieht auch diese,
   und der Nutzer kann sie umbelegen.
2. **Es beim Menükürzel belassen** und ausdrücklich festhalten, dass
   Shift+Cmd+W von der Belegung ausgenommen und nicht umbelegbar ist. Dann
   braucht die Auslieferungsbelegung einen Kommentar, der die Kombination als
   vergeben ausweist, damit ein späterer Eintrag sie nicht ein zweites Mal
   vergibt.

Herkunft: aufgefallen beim Nachtragen von `fenster_einblenden` in S9b.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C7, Abnahmekriterium zu Shift+Cmd+W),
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`

---
Resolved: Nutzerentscheidung vom 260805-0000, Weg 1 dieses Datensatzes. Die Menükürzel ziehen in die Konflikterkennung aus C3 ein: `fenster_schliessen` bekommt einen Eintrag in `resources/default-keymap.toml`, und das Hauptmenü nimmt sein Kürzel von dort statt es im Programmtext festzulegen. Damit gilt eine Quelle für alle Kombinationen, die Konflikterkennung sieht auch diese, und der Nutzer kann sie umbelegen. Der Mechanismus ist nicht neu: Cmd+N steht seit S9b und S12 zugleich in der Belegung und am Menüeintrag "Fenster einblenden", weil der Ereignisabgriff jeden Tastendruck vor der Menübehandlung sieht. Im Plan trägt **S13b** den Eintrag ein, **S13c** baut das Menü um. Gemeinsam beantwortet mit `260804-1040_c_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md`. Entscheidungsdatensatz `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`. Sitzungsbericht `history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`.
