Die Blockreihenfolge der Belegungsdatei steuert seit S6 die Menüleiste, und der Dateikopf sagt es nicht

---

Der Kopf von `resources/default-keymap.toml` erklärt auf über hundert Zeilen, was ein Eintrag
trägt: `id`, `name`, `tasten`, `reserviert_fuer`, `gehalten_von`, die Ein-Zeilen-Regel, die
Schreibweise der Kombinationen, die ab Werk freien Tasten, die Zählzeile. **Er sagt an keiner
Stelle, dass die Reihenfolge der Blöcke etwas bewirkt.** Ein `grep` über `Reihenfolge` und
`Ordnung` in der Datei trifft genau zwei Stellen, beide über die Schreibweise der
Zusatztasten (`:37`) und über einen Modulkopf im Code (`:656`).

Die Reihenfolge bewirkt aber etwas, und seit dieser Runde mehr als vorher.

**`belegungsmodell::nach_bereichen` gibt die Stellen jeder Gruppe in Dateireihenfolge
zurück** (`crates/krk-ui/src/belegungsmodell.rs:806-818`): es läuft über
`Funktionsbereich::ALLE`, sammelt je Gruppe die Indizes in der Reihenfolge, in der
`belegung.funktionen()` sie führt, und das ist die Reihenfolge der `[[funktion]]`-Blöcke.
Bis zur Runde 7 hingen daran die Belegungsansicht und die Markdown-Ausgabe. **Seit S6 hängt
die Menüleiste von macOS mit daran**, denn `menuemodell::aufbau` baut jedes Obermenü aus
demselben Aufruf (`crates/krk-ui/src/menuemodell.rs:204-224`).

Der Modulkopf von `menuemodell.rs` sagt es und zieht die richtige Folgerung:

> `crates/krk-ui/src/menuemodell.rs:76-82` — „Die Reihenfolge innerhalb eines Obermenues kommt
> jetzt aus der Belegungsdatei. […] Beides ist am billigsten in
> `resources/default-keymap.toml` zu beheben, nicht hier."

Die Belegungsdatei selbst sagt es nicht.

## Was das kostet

**Erstens ist der Befund `260813-0420` genau dieser Fall.** Er schlägt vor, `text_rueckgaengig`
und `text_wiederholen` in der Datei vor `text_ausschneiden` zu ziehen, um dem Menü
„Bearbeiten" seine Mac-übliche Reihenfolge zurückzugeben. Wer diese Verschiebung am Kopf der
Datei nachliest, findet keinen Satz, der sie erklärt, und keinen, der davor warnt, sie später
aus Lesbarkeitsgründen wieder rückgängig zu machen.

**Zweitens ist der neue Block aus S15 ohne diesen Prüfschritt eingeordnet worden.** Der
Kommentar über `weitere_instanz` (`resources/default-keymap.toml:874-880`) und der
Verlaufseintrag des Ausführenden begründen den Ort auf Sachgruppe und Lesbarkeit und nennen
die Wirkung auf das Menü nicht. Sie ist trotzdem eingetreten: das Obermenü „Anwendung" liest
sich jetzt „Tastaturbelegung anzeigen", „Weitere Instanz starten", „Belegung als Markdown …",
Trenner, „KRK beenden". Das Ergebnis ist brauchbar; geprüft ist es nicht, und beim nächsten Block trifft
es sich vielleicht nicht mehr.

**Drittens hält keine Probe die Reihenfolge fest.** Abschnittsköpfe prüft nichts (so schon
`260812-1528`), und die Blockreihenfolge ebenso wenig. Ein Umsortieren aus Lesbarkeitsgründen
verändert die Menüleiste, ohne dass der Baum rot wird.

---

**Schwere:** gering bis mittel. Kein Befehl fällt aus, keine Kombination ändert sich. Die Datei
ist die eine Quelle, und sie führt seit dieser Runde eine Zusicherung, die sie nicht nennt.

**Gefunden:** ontorev, bei der Durchsicht von `resources/default-keymap.toml` über
`ca66c39..40b5fb0` am 260813-0534.

**Betroffen:** `resources/default-keymap.toml` (Dateikopf).

**Domain:** data — die Behebung gehört dem `ontocoder`.

**Nicht betroffen:** die Gliederung nach `Funktionsbereich`. Welche Gruppe eine Funktion
bekommt, entscheidet `belegungsmodell::bereich_des_kommandos` und nicht diese Datei; allein die
Reihenfolge **innerhalb** einer Gruppe kommt von hier.

## Empfehlung

Einen Absatz in den Dateikopf, neben die Ein-Zeilen-Regel, etwa:

> Die Reihenfolge der Blöcke ist nicht gleichgültig. `belegungsmodell::nach_bereichen` gibt
> die Funktionen einer Gruppe in der Reihenfolge zurück, in der sie hier stehen, und drei
> Abnehmer zeigen sie so an: die Belegungsansicht, die Markdown-Ausgabe und seit der Runde 7
> die Menüleiste. Wer einen Block verschiebt, verschiebt einen Menüeintrag mit. Die Gruppe
> selbst kommt nicht von hier, sondern aus `belegungsmodell::bereich_des_kommandos`.

**Zusammen mit `260813-0420` zu behandeln**, denn dessen Behebung ist die erste Verschiebung,
die diesen Absatz braucht.
