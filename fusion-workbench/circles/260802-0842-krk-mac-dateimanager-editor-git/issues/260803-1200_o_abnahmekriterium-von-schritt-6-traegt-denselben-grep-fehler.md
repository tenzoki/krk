Das Abnahmekriterium von Schritt 6 trägt denselben grep-Fehler wie die von Schritt 2 und Schritt 15

---

Das Abnahmekriterium von Schritt 6 im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` verlangt wörtlich:

> `grep -rn 'unsafe' crates/krk-ui/src --include='*.rs' -l` liefert ausschließlich Dateien unterhalb von `src/appkit/`.

Das kann nicht aufgehen. Schritt 1 verlangt `#![warn(unsafe_code)]` als erste Zeile von `crates/krk-ui/src/main.rs`, und diese Zeile enthält die Zeichenkette `unsafe`. Die `main.rs` liegt nicht unterhalb von `src/appkit/`, und der grep nennt sie deshalb zwangsläufig, gleich wie sauber der Code ist.

Nachgeprüft am 260803-1200, am heutigen Bestand, in dem `src/appkit/` noch gar nicht existiert:

```
$ head -1 crates/krk-ui/src/main.rs
#![warn(unsafe_code)]

$ grep -rn 'unsafe' crates/krk-ui/src --include='*.rs' -l
crates/krk-ui/src/main.rs
```

**Der gemeinte Sachverhalt hält, nur die Prüfvorschrift trifft ihn nicht.** Es ist derselbe Fehler, den die Meldung `issues/260802-1810_c_abnahmekriterium-mit-grep-unsafe-kann-nicht-aufgehen.md` für die Schritte 2 und 15 beschreibt, an einem dritten Ort. Er ist bei der Behebung jener Meldung aufgefallen, war aber nicht ihr Gegenstand: sie nennt Schritt 2 und Schritt 15 namentlich, und der Auftrag an den `planner` war auf diese beiden begrenzt. Deshalb steht er hier als eigener Defekt und wurde nicht nebenbei mitverändert.

---

**Warum die Auflösung nicht wörtlich dieselbe sein kann.** In `krk-core` steht `#![deny(unsafe_code)]`, und der Bau scheitert dort, sobald `unsafe` außerhalb einer Datei mit `#[allow(unsafe_code)]` auftaucht. Das trägt die halbe Zusage maschinell, und die andere Hälfte prüft der grep auf das Attribut. In `krk-ui` steht `#![warn(unsafe_code)]`, nicht `deny`: eine Warnung bricht den Bau nicht ab. Ein reiner Attribut-grep würde dort also weniger belegen als in `krk-core`, weil der Bau nichts erzwingt.

**Was zu tun ist.** Der `planner` ersetzt die Prüfvorschrift in Schritt 6 durch eine, die den gemeinten Sachverhalt trifft. Zwei Wege bieten sich an, die Wahl liegt beim `planner`:

- Auf den `unsafe`-Block und die `unsafe`-Blöcke prüfen statt auf das Wort, etwa über die Verankerung, die für die Schritte 2 und 15 gewählt wurde, ergänzt um eine Prüfung, dass außerhalb von `src/appkit/` keine Zeile mit `unsafe` als Sprachkonstrukt steht. Trifft den Code, nicht das Lint-Attribut.
- `#![warn(unsafe_code)]` in `krk-ui` durch `#![deny(unsafe_code)]` ersetzen und die Vorschrift dann wie in Schritt 2 formulieren. Das macht die Grenze maschinell durchsetzbar statt nur beobachtbar, ändert aber eine Festlegung aus `## Aufbau` und aus Schritt 1, die ausdrücklich zwischen `warn` und `deny` unterscheidet. Diese Festlegung ist begründet und sollte nicht nebenbei fallen; der Weg gehört dem Nutzer vorgelegt, nicht still gewählt.

**Aufgefallen bei:** der Behebung von `issues/260802-1810_c_abnahmekriterium-mit-grep-unsafe-kann-nicht-aufgehen.md` und `issues/260802-1935_c_frage-7-und-schritt-5-widersprechen-sich-bei-der-signaturidentitaet.md` durch den `planner`, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260803-1200-zwei-plandefekte-signaturidentitaet-und-unsafe-pruefung.md`. Schritt 6 ist noch nicht umgesetzt; der Defekt schlägt zu, sobald er abgenommen wird.
