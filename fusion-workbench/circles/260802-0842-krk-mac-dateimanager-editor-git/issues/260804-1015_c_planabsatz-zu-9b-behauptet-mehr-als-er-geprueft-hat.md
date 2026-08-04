Planabsatz zu 9b behauptet mehr, als er geprüft hat

---

Der Absatz unter `#### 9b.` in
`planning/260802-1428_o_plan-navigator-geruest-runde-1.md` schließt aus einer
Prüfung in eine Richtung auf eine Zusage in beide. Er sagt sinngemäß: "Die drei
Kennungen dürfen vor ihren Kommandos landen, und das ist geprüft", und leitet
daraus ab, S9b breche keine Prüfung. Geprüft ist damit allein die Richtung
Kommando → Kennung, nämlich
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`. Die
Zählprüfungen der Belegung lagen quer dazu: sie hingen an der Größe der Datei
und nicht an den Kennungen, und genau sie sind mit S9b umgefallen.

Zu tun: den Absatz so umformulieren, dass er nur behauptet, was die genannte
Prüfung deckt, und die Zählprüfungen als eigenen Punkt nennt oder als erledigt
verweist.

Ausführender: wer die Plandatei führt (`planner`). Der Rust-Teil ist erledigt
und liegt außerhalb dieses Datensatzes.

---

Gemeldet am 260804-1015 beim Schließen von
`260804-0907_c_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`,
dessen letzter Abschnitt diese Nachbesserung verlangt. Der Datensatz wurde
geschlossen, weil sein Code-Teil vollständig abgearbeitet ist; die
Planänderung liegt außerhalb der Grenzen jener Aufgabe und würde sonst mit dem
Schließen verschwinden.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 9b),
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-0907_c_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260804-1015-zaehlpruefungen-ohne-literale.md`

---
Resolved: Der Absatz unter 9b behauptet jetzt nur noch, was die genannte Prüfung deckt, nämlich die Richtung Kommando nach Kennung. Die Zählprüfungen stehen als eigener Punkt daneben, mit Verweis auf den erledigten Rust-Teil und der Regel, dass jede Prüfung, die an einer Anzahl hängt statt an einem Namen, von einem Nachtrag in dieser Datei betroffen ist und ohne Literal gehört. Nachgezogen am 260804-2318 vom `planner`.
