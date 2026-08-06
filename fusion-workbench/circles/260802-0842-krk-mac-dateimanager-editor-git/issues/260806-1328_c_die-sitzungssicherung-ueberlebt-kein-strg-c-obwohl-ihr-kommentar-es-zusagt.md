Die Sitzungssicherung überlebt kein Strg+C, obwohl ihr Kommentar es zusagt

---

`Sitzungssicherung` in `crates/krk-bench/src/messen.rs:1116-1189` spielt die
`session.toml` des Nutzers über `Drop` zurück. Der Modulkommentar an
`messen.rs:1129` behauptet: "Ein SIGKILL von aussen ueberlebt auch das nicht;
alles darunter schon." Das trifft nicht zu. Ohne Signalbehandlung beenden
SIGINT (Strg+C) und SIGTERM den Prozess über den Standard-Handler, ohne
Abwicklung und damit ohne `Drop`. Geprüft: `grep -rn "signal\|SIGINT\|ctrlc\|SIGTERM"
crates/krk-bench/src/` liefert keinen Treffer.

---

**Warum das mehr ist als ein falscher Kommentar.** Ein Gesamtlauf fährt mehrere
Runden mit `FRIST_SPANNEN = 300 s` je Strecke; er läuft also Minuten bis
Viertelstunden. Strg+C ist der übliche Weg, einen solchen Lauf abzubrechen,
nicht der Ausnahmefall. Genau dann bleibt die Prüfsitzung aus C8 als
`session.toml` des Nutzers liegen, und der Nutzer verliert Tabs, Ordner und
Breiten — die eine Wirkung, gegen die die Sicherung gebaut wurde.

Der `?`-Abbruch und die Panik sind dagegen sauber abgedeckt: der Workspace
setzt kein `panic = "abort"`, also wickelt eine Panik ab und `Drop` läuft. Der
Fall "vorher gab es keine `session.toml`" ist ebenfalls richtig behandelt
(`vorher: None` → `remove_file`, `NotFound` toleriert), geprüft durch
`ohne_vorigen_stand_bleibt_keine_pruefsitzung_liegen`.

**Zwei Wege, die Zusage einzulösen** (die Wahl gehört zum Fix, nicht hierher):

1. Einen Signalgriff für SIGINT und SIGTERM einhängen, der die Sicherung
   zurückspielt und danach mit dem üblichen Code endet. Das löst die Zusage
   wörtlich ein, holt aber eine Abhängigkeit oder rohes `libc` in `krk-bench`.
2. Den Kommentar auf die Wahrheit ziehen und den Weg zurück von Hand
   dokumentieren: die Sicherung greift bei `?` und bei Panik, nicht bei einem
   Signal. Zusätzlich könnte der Lauf den vorigen Stand vor der ersten Runde
   als Kopie neben die `session.toml` legen, damit ein abgebrochener Lauf sie
   nicht unwiederbringlich verliert.

**Cross-Referenz:** Der Defekt entstand mit `4195aa3` (S-Befund "Der Sitzungslauf
überschreibt die session.toml des Nutzers ohne sie zurückzuspielen").

**Betrifft:** `krk-bench`, nur die Messstrecke. Kein Einfluss auf `krk-ui` oder
`krk-core`, keine der zehn Zeitzusagen aus C8 berührt.

---
Resolved: SIGINT, SIGTERM und SIGHUP hängen jetzt einen Griff ein, der die session.toml zurückspielt und mit 128 + Signalnummer endet (crates/krk-bench/src/messen.rs). Neue Abhängigkeit signal-hook 0.4, nur in krk-bench, ohne Vorgabemerkmale: std kennt keine Signal-Schnittstelle, libc verlangte einen unsafe-Block und damit den Grenzstein aus CLAUDE.md, ein Wächterprozess schüfe ein neues Fehlerbild (verwaister Wächter schriebe in eine später angelegte Sitzung), und ctrlc zieht nix nach. signal-hook schreibt im Signalkontext nur in ein Selbstrohr; das Zurückspielen läuft auf einem gewöhnlichen Faden. Die Messung bleibt unberührt, weil die Registrierung SA_RESTART setzt und kein Systemaufruf mit EINTR abbricht.

Die Zusage im Kommentar ist auf das herabgesetzt, was tatsächlich gilt, und zählt das Ungedeckte auf: SIGKILL, SIGSTOP, und ein Signal, das nur krk-bench erreicht und nicht den laufenden krk-Kindprozess.

Nachweis: kill -INT an die Prozessgruppe während eines Laufs; die session.toml steht danach byteweise auf dem Stand vor dem Lauf (907acf51... vorher und nachher, 417f63ee... während des Laufs), Ausgangswert 130, kein krk-Prozess übrig.
