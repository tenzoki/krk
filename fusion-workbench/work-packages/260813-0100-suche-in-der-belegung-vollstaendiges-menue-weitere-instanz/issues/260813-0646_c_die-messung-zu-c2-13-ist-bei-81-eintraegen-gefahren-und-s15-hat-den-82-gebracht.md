Die Messung zu C2.13 ist bei 81 Einträgen gefahren, und S15 hat den 82. gebracht

---

Kriterium C2.13 des Spec sagt zu: „Nach dieser Runde zeigt `--menue-protokoll` weder
‚Emoji & Symbols' noch ‚Start Dictation…' noch das Untermenü ‚AutoFill', und **zu keinem neuen
Eintrag** stellt AppKit eine Zweitform mit eigener Kombination. **(Probe** über das Protokoll**)**"

**Die Messung ist gefahren, aber vor dem letzten Eintrag.** Das Verlaufsprotokoll
`history/260813-0445-coder-s4-bis-s6-vollstaendiges-menue.md` führt den Lauf am 260813-0445 mit
**81** Menüeinträgen. S15 (`40b5fb0`, 05:23) hat `weitere_instanz` in
`resources/default-keymap.toml` eingetragen und damit den **82.** Eintrag erzeugt. Danach ist
`--menue-protokoll` nicht mehr gelaufen.

**Ungemessen bleibt damit genau der Fall, den das Kriterium benennt:** ob AppKit dem neuen
Eintrag „Weitere Instanz starten" auf `opt+cmd+n` eine Zweitform mit eigener Kombination
beistellt. Für die anderen 81 Einträge ist die Aussage gemessen.

**Eine Probe hält das Kriterium nicht.** Am 260813 im Baum nachgesehen: es gibt keine
`#[test]`, die `--menue-protokoll` ausliest oder seine Ausgabe prüft. Das Kriterium hängt
allein an diesem Lauf.

---

**Schwere:** gering. Der Lauf kostet Sekunden und braucht kein Bündel und kein Fenster:
`cargo run -q -p krk-ui --bin krk -- --menue-protokoll`, so wie der Plan es unter S6 vorsieht.

**Warum er in diesem Abgleich nicht gefahren ist:** der Modus legt beim Start eine Sperrdatei
in `~/Library/Application Support/KRK/` an, und der Auftrag dieses Abgleichs schließt jede
Änderung außerhalb der Werkbank aus.

**Gefunden:** reconciler, Abgleich der Runde 7, beim Nachlesen der Abnahmekriterien gegen den Baum

**Betroffen:** `shared/planning/260813-0053_*_spec-…md` (C2.13),
`history/260813-0445-coder-s4-bis-s6-vollstaendiges-menue.md` (der Lauf),
`resources/default-keymap.toml` (`weitere_instanz`)

**Domain:** code

## Behebung

Den Lauf wiederholen und sein Ergebnis festhalten. Zeigt er zum 82. Eintrag keine Zweitform,
ist C2.13 nachgewiesen und der Datensatz geschlossen. Zeigt er eine, ist es ein eigener Defekt
mit eigenem Gewicht — dieselbe Klasse wie die Zweitform „Quit and Keep Windows" auf
Opt+Cmd+Q, die die Abnahmeliste des Plans ohnehin führt.

---
Resolved: Am 260813 mit 82 Eintraegen nachgemessen, `cargo run -q -p krk-ui --bin krk --
--menue-protokoll`, Exit 0. Das Protokoll zaehlt **84 Zeilen**, **neun Obermenues**
(Anwendung, Bearbeiten, Dateilisting, Dateioperationen, Editor, Fenster, Leiste und Fokus,
Tabs, Vorschau) und **76** Eintraege mit `krkKommando:`.

Der neue Eintrag steht darin:
`menue="Anwendung" eintrag="Weitere Instanz starten" kombination=opt+cmd+n kuerzel="n"
zusatztasten=1572864 zweitform=nein verdeckt=nein selektor=krkKommando:`

**Keine Doppelvergabe.** Ueber Kuerzel und Zusatztasten zusammen gibt es genau einen
Mehrfachtreffer, und der ist das leere Kuerzel ohne Zusatztaste, also die Eintraege ohne
Kombination. Kein Eintrag traegt `zweitform=ja` oder `verdeckt=ja`.

Damit ist genau der Fall gemessen, den das Kriterium benennt, und er haelt. Der Lauf braucht
kein Buendel und keinen Vordergrund; er kostet Sekunden und ist deshalb vom Orchestrator
unmittelbar nach dem Abgleich gefahren worden, statt ihn dem Nutzer aufzuladen.

