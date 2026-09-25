"Ersetzt" in Frage 4 und S10 lässt offen, ob die kaputte Datei überschrieben wird

---

S10 schreibt vor: "Beschädigte oder nicht lesbare Dateien werden benannt und
durch den Auslieferungszustand ersetzt, statt den Start scheitern zu lassen."
Zwei Lesarten von "ersetzt" sind vertretbar, und sie unterscheiden sich in
dem, was der Nutzer verliert.

---

**Die beiden Lesarten.**

1. **Nur der geladene Zustand.** KRK arbeitet mit dem Auslieferungszustand
   weiter, die Datei auf der Platte bleibt liegen. Überschrieben wird sie erst
   beim nächsten gewöhnlichen Schreibvorgang.
2. **Auch die Datei.** KRK schreibt den Auslieferungszustand sofort über die
   kaputte Datei.

**Warum das nicht gleichgültig ist.** `### Frage 4` begründet die Wahl von TOML
ausdrücklich damit, dass der Nutzer `keymap.toml` lesen und **von Hand ändern**
können muss. Eine von Hand geänderte Datei mit einem Tippfehler darin ist der
Alltagsfall dieses Defektpfades, nicht der Ausnahmefall. Unter Lesart 2 kostet
ein vergessenes Anführungszeichen die ganze Belegung des Nutzers, und zwar
ohne Rückweg, weil die Ablage keine Sicherung anlegt. Unter Lesart 1 kostet er
den Start mit dem Auslieferungszustand und einer Meldung; die Datei liegt noch
da und lässt sich reparieren.

Für `session.toml` ist der Unterschied belanglos, weil KRK sie ohnehin alle
zwei Sekunden überschreibt. Für `bookmarks.toml` liegt er dazwischen.

**Was die Umsetzung getan hat.** Die Umsetzung von S10 am 260803-2051 folgt
Lesart 1 und hält das in `crates/krk-core/src/ablage/mod.rs` im Modulkopf sowie
in einer Prüfung fest
(`eine_kaputte_datei_fuehrt_zum_auslieferungszustand_und_zu_einer_meldung`
prüft, dass die kaputte Datei unverändert liegen bleibt). Die Wahl ist die
verlustärmere und deckt sich mit dem Abnahmekriterium, das vom
Auslieferungs**zustand** spricht und nicht von der Auslieferungs**datei**. Sie
ist damit begründet, aber nicht entschieden: der Plantext lässt beides zu.

**Was zu tun ist.** Entweder der Plan schreibt die Lesart aus, dann ist der
Punkt erledigt, oder der Nutzer entscheidet anders, dann ändert sich eine
Verzweigung in `Ablage::laden` und die genannte Prüfung.

**Dringlichkeit.** Bindet keinen Schritt. S11 liest `keymap.toml` über
denselben Weg und erbt die Lesart; wird sie geändert, ist S11 der letzte
Zeitpunkt, an dem das billig ist.

**Aufgefallen bei:** der Umsetzung von S10, beim Lesen von `### Frage 4`.

---
Resolved: Der Plan schreibt Lesart 1 aus, an beiden Stellen. `### Frage 4` bekommt einen eigenen Absatz mit der Herleitung aus der Handänderbarkeit von `keymap.toml`, und die `Änderungen` von S10 trennen Auslieferungs**zustand** von Auslieferungs**datei** ausdrücklich. Die Umsetzung vom 260803-2051 ist damit bestätigt; an `crates/krk-core/src/ablage/mod.rs` ändert sich nichts. Nachgezogen am 260804-2318 vom `planner`.
