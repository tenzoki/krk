Cmd+W liegt in der Auslieferungsbelegung auf "Tab schließen" und im Menü auf "Fenster schließen"

---

`resources/default-keymap.toml` belegt `cmd+w` mit `tab_schliessen`, dem Schließen des aktiven Tabs aus C1. Der Menüeintrag "Fenster schließen" aus Schritt 6 trägt dieselbe Kombination als Menükürzel (`crates/krk-ui/src/appkit/menue.rs:33-42`). Zwei verschiedene Funktionen auf einer Kombination, und die eine steht in der Belegung, die andere im Menü, wo die Konflikterkennung aus C3 sie nicht sieht.

---

Herkunft: gefunden beim Schreiben von `resources/default-keymap.toml` (Plan Schritt 9).

Warum die Belegung trotzdem `cmd+w` nennt: Cmd+W ist auf dem Mac das Kürzel für das Schließen des vordersten Tabs, im Finder wie im Browser. Eine andere Kombination für das Schließen eines Tabs zu wählen, nur um dem Menüeintrag auszuweichen, hätte die Gewohnheit dem Behelf geopfert und den Konflikt zugleich verdeckt.

Zusammenhang mit einer offenen Frage: `decisions/260803-2007_o_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md` steht offen und entscheidet, ob Cmd+W die Anwendung beendet oder ob es einen Rückweg über einen Menüeintrag "Neues Fenster" gibt. Solange sie offen ist, ist auch nicht entschieden, wem Cmd+W gehört. Dieser Defekt löst sie nicht auf, sondern hält fest, dass die Belegung jetzt eine zweite Partei für dieselbe Kombination hat.

Zu klären vor Schritt 12, der die Tabs anlegt und den Menüeintrag ohnehin anfasst. Zwei Wege stehen offen: Cmd+W bleibt beim Tab und das Fenster wandert auf Umschalt+Cmd+W, wie es Browser halten, oder der Menüeintrag behält Cmd+W und das Schließen des Tabs bekommt eine andere Kombination in `resources/default-keymap.toml`.

---

**Stand 260804-0830: in der Sache entschieden, offen bis zur Umsetzung.** Der Nutzer hat am 260804 Möglichkeit 2 aus `decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md` gewählt und dabei ausdrücklich festgehalten, dass Cmd+W seine Bedeutung "Tab schließen" aus der Belegung behält. Damit gilt der erste der beiden oben genannten Wege: der Menüeintrag "Fenster schließen" wandert auf Shift+Cmd+W, `resources/default-keymap.toml` bleibt unverändert. Umgesetzt wird die Verschiebung in Schritt 12, der den Menüeintrag ohnehin anfasst; dieser Defekt schließt erst mit dieser Umsetzung.

---
Resolved: Mit S12 umgesetzt und damit überholt, festgestellt beim Aufräumdurchgang am 260804-2318. Der Menüeintrag "Fenster schließen" trägt seit Commit `537fda5` das Kürzel `shift+cmd+w`, und `cmd+w` gehört allein dem Schließen des Tabs, wie `resources/default-keymap.toml` es seit S9 führt und wie der Nutzer es am 260804 bestätigt hat. Die Doppelbelegung, die diese Meldung beschreibt, besteht nicht mehr. Was offen bleibt, ist eine andere Frage und hat einen eigenen Datensatz: dass Menükürzel überhaupt außerhalb der Konflikterkennung aus C3 liegen, `260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`.
