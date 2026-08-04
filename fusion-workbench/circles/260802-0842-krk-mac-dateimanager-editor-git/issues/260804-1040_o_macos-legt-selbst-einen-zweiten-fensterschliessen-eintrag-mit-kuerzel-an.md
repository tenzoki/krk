macOS legt selbst einen zweiten Fenster-schließen-Eintrag mit Kürzel an

---

Im Menü "Fenster" steht nach dem Start ein vierter Eintrag, den KRK nicht
gebaut hat: **"Close All" auf Opt+Shift+Cmd+W**, mit der Aktion `closeAll:`.
AppKit fügt ihn von sich aus als Zweitform zu einem Eintrag mit `performClose:`
hinzu. Damit hat KRK eine zweite Tastenkombination, die eine Funktion auslöst,
die weder in `resources/default-keymap.toml` steht noch von der
Konflikterkennung aus C3 gesehen wird, noch vom Nutzer umbelegbar ist.

---

Gemessen am 260804-1040 im signierten Bündel. Eine vorübergehende Sonde hat das
Hauptmenü nach dem Start ausgelesen; sie ist wieder entfernt. Die vier Einträge:

| Beschriftung | Kürzel | Zusatztasten | Aktion |
|---|---|---|---|
| KRK beenden | `q` | Cmd | `terminate:` |
| Fenster einblenden | `n` | Cmd | `fensterEinblenden:` |
| Fenster schließen | `w` | Shift+Cmd | `performClose:` |
| **Close All** | `w` | **Opt+Shift+Cmd** | `closeAll:` |

Der vierte stammt nicht aus `crates/krk-ui/src/appkit/menue.rs`; die Datei baut
drei Einträge. Er ist auch nicht übersetzt: seine Beschriftung ist englisch,
während die drei eigenen deutsch sind.

Nicht dieselbe Sache wie
`issues/260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`,
aber derselbe Mangel eine Stufe weiter: jener Defekt hält fest, dass
Shift+Cmd+W außerhalb der Belegungsdatei liegt. Dieser hier sagt, dass es nicht
bei einer Kombination bleibt, sondern dass das System eine zweite dazustellt,
von der niemand im Projekt wusste. Die Zahl der Kombinationen außerhalb der
Konflikterkennung ist damit nicht eins, sondern zwei, und sie steht nicht fest:
sie hängt daran, welche Zweitformen AppKit zu welchem Selektor beisteuert.

Drei Wege stehen offen, und die Wahl gehört zum selben Entscheid wie der ältere
Defekt:

1. Den Eintrag stehen lassen und beide Kombinationen als vergeben in einem
   Kommentar der Auslieferungsbelegung ausweisen.
2. Den Eintrag unterdrücken. Ein Menü, das keinen `performClose:`-Eintrag
   trägt, bekommt auch kein "Close All"; das hieße, das Schließen des Fensters
   über einen eigenen Selektor am Anwendungsdelegierten zu führen, so wie es
   "Fenster einblenden" schon tut.
3. Ihn übersetzen und behalten. Dann ist er eine Funktion von KRK und gehört in
   die Belegungsdatei, was Weg 1 ohnehin verlangt.

Herkunft: aufgefallen bei der Abnahme von Schritt 12, beim Nachlesen der
Menükürzel im laufenden Bündel.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C3, C7),
`crates/krk-ui/src/appkit/menue.rs`
