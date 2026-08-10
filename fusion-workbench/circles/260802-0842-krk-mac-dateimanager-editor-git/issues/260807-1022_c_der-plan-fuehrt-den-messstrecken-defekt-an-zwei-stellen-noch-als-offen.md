Der Plan führt den Messstrecken-Defekt an zwei Stellen noch als offen

---

**Domain:** code
**Filed by:** reconciler (Abgleich 260807-1022, Sitzung 260806-2257)
**Für:** `planner`
**Cross-references:** `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` (Kopfzeile 23 und `### Frage 5`),
`issues/260807-0832_c_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md`,
`crates/krk-bench/src/messen.rs`, Commit `d569f8a`

---

Der Plan sagt an zwei Stellen, die Auswertung der Messstrecke könne die neue
zweiteilige Fassung der Zusage L9 nicht abnehmen. Das war am 260807-0832 richtig
und ist seit `d569f8a` (260807-0856, 16 Minuten später) falsch.

**Erste Stelle, Kopfzeile 23, Nachzug 260807-0832.** Der Absatz schließt mit
"Offen bleibt daraus ein Defekt an der Messstrecke,
`issues/260807-0832_*_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md`".
Der Defekt trägt seit `d569f8a` den Marker `_c_`.

**Zweite Stelle, `### Frage 5`, Zeile 264.** Der Absatz "L9 teilt diese Regel
seit dem 260807-0832 nur noch zur Hälfte" schließt mit: "Die Auswertung in
`crates/krk-bench/src/messen.rs` bildet beides heute nicht ab, weil
`Abnahmemass::AnteilImBild` allein die Bildlänge trägt und den geforderten
Anteil aus der Konstanten `ANTEIL_IM_BILD_PROZENT` für L1 und L9 gemeinsam
nimmt; der Defekt dazu ist `issues/260807-0832_*_…`."

**Was auf der Platte steht.** `Abnahmemass::AnteilImBild`
(`crates/krk-bench/src/messen.rs:390-405`) trägt heute drei Felder: `bildlaenge`,
`mindestanteil_prozent` und `obergrenze_bilder`. Die Konstante
`ANTEIL_IM_BILD_PROZENT` gibt es im ganzen Projektbaum nicht mehr; eine Suche
über alle Dateien liefert null Treffer. `Zusage::gehalten_in`
(ebd.:572-610) prüft beide Hälften in derselben Runde. L9 steht in der
Zusagenliste des Abnahmelaufs (ebd.:1117-1129) auf
`mindestanteil_prozent: 85, obergrenze_bilder: Some(2)`.

**Warum das mehr ist als ein Datumsfehler.** `### Frage 5` ist die Stelle, an
der der Plan die Auswertungsvorschrift führt. Wer sie heute liest, glaubt, die
Messstrecke gebe für L9 weiter das alte Urteil aus, und käme bei einem
Abnahmelauf zu dem Schluss, die Runde könne nicht schließen. Genau umgekehrt
ist es: die Auswertung nimmt L9 in der neuen Fassung ab, und der Defekt ist
geschlossen.

**Der Wortlaut der Zusage selbst ist an allen vier Stellen gleich** — im
Vorspann der Abnahmekriterien von C8, in der Zusagentabelle, in der
Messvorschrift und in `crates/krk-bench/src/messen.rs`. Nachgeprüft am
260807-1022; dieser Defekt betrifft allein die Aussage darüber, ob die
Auswertung sie schon abbildet.

**Dringlichkeit.** Mittel. Kein Abnahmekriterium hängt daran, und keine der
zehn Zahlen aus C8 ändert sich. Aber der Plan ist die Stelle, an der der
Rundenabschluss abgelesen wird, und er widerspricht dort dem Code.

---
Resolved:

---
Resolved: In `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` sind zwei Stellen
berichtigt. Zeile 25 haelt weiter fest, dass der Nachzug vom 260807-0832 einen Defekt
hinterliess — sonst erklaerte sich dieser Datensatz nicht mehr — und nennt im selben Satz seine
Schliessung durch `d569f8a` vom 260807-0856. Zeile 1458, die dieser Datensatz nicht kannte,
behauptete "Zwei Stellen dieses Plans sagen aber ..." und steht jetzt im Praeteritum mit dem
Vermerk, dass beide Stellen berichtigt sind.

Von den zwei gemeldeten Stellen stand nur eine offen: `### Frage 5` (heute Zeile 267) war mit
`f11b36d` vom 260807-1923 bereits berichtigt und sagt selbst "ist damit geschlossen". Dafuer kam
Zeile 1458 als dritte hinzu. Die Zeilennummern des Datensatzes (23 und 264) waren um zwei bis
drei Zeilen abgewandert.

Nachgeprueft: `ANTEIL_IM_BILD_PROZENT` liefert im Codebaum null Treffer, `Abnahmemass::AnteilImBild`
traegt drei Felder (`crates/krk-bench/src/messen.rs:395-410`), `Zusage::gehalten_in` prueft beide
Haelften mit `anteil_haelt && grenze_haelt` (ebd.:577-612). Der Plan behaelt den Marker `_c_`;
berichtigt ist der Inhalt, nicht der Zustand.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 1.
