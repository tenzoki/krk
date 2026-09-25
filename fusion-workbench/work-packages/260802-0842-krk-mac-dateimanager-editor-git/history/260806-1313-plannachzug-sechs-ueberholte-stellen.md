# Plannachzug: sechs überholte Stellen in Runde 1

**Datum:** 2026-08-06, 13:13
**Agent:** planner
**Status:** Complete
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

## Auftrag

Sechs Stellen im Plandokument beschrieben einen Bestand, den spätere Schritte
verlassen haben: vier Abnahmekriterien, die an überholte Grep-Ausdrücke oder an
eine überholte Zuordnung gebunden waren, und zwei Dateilisten. Keine neue
Planung, keine Änderung an einer Zusage. Fünf Defektdateien lagen vor, die
sechste Stelle kam aus der Behebung von
`issues/260806-0834_*_die-appkit-grenzpruefung-uebersieht-pub-use-und-use-mit-fuehrendem-doppelpunkt.md`
(Commit `4195aa3`).

## Was geändert ist

Alle Änderungen stehen im Plandokument. Der Marker bleibt `_o_`, die Statuszeile
ist unberührt.

1. **S6b, Abnahmekriterium.** Das `eprintln!`-Kriterium prüft jetzt
   `tastenabgriff_einrichten` und `tastenabgriff_nachziehen` statt die ganze
   `anwendung.rs`; die sechs verbliebenen Vorkommen gehören dem Messmodus aus S8
   und S21 und sind dort der richtige Kanal. Das `NSAlert`-Kriterium verlangt
   für `hinweis.rs` den einzigen **anwendungsmodalen** `NSAlert`, weil
   `blaetter/mod.rs` seit S13 einen als Blatt am Fenster anlegt.
2. **S18c, Abnahmekriterium.** Der Zähler für den Verzicht auf einen
   Unterprozess läuft über `Command::new\|process::Command` statt über
   `std::process`, das jedes `exit` und jedes `id` mittraf.
3. **S18c, Dateiliste.** `kommandos/fokus.rs` und `kommandos/mod.rs`
   nachgetragen, beide mit dem Grund, aus dem der Schritt sie anfassen muss.
4. **S18c, Rangzuordnung.** Die beschädigte `settings.toml` ist eine
   Startmeldung auf Rang 3 (Fenstermeldung) und keine Befehlsantwort auf Rang 1;
   die beiden übrigen Fehler aus C11 behalten Rang 1. Der Absatztitel steht
   jetzt auf "Drei Fehler, zwei Ränge".
5. **S17, Dateiliste und Abnahmekriterium.** Das Kernmodul heißt seit dem 260805
   `stapelumbenennen`; Verzeichnis, Einbindung in `lib.rs`, Abnahmedatei und das
   Testprogramm im Kriterium sind nachgezogen. `operation/umbenennen.rs` behält
   seinen Namen.
6. **S23, Abnahmekriterium.** Das Grep liest ein Sichtbarkeitspräfix (`pub`,
   `pub(crate)`, `pub(super)`, `pub(in …)`) und ein führendes `::` mit und
   beschreibt damit dieselbe Vorschrift wie `ist_objc2_use` in
   `xtask/src/release.rs`. S18c führt denselben Ausdruck und ist mitgezogen.

Dazu ein `**Nachzug 260806-1313:**` im Kopf des Plans, je ein Absatz an den
sechs Stellen, der sagt, warum die alte Fassung nicht mehr trägt, und ein
Verweis am betroffenen Punkt des Reconciliation Log vom 260806-0904, der als
Aufzeichnung jenes Abgleichs stehen bleibt.

## Am Bestand geprüft

- Sechs `eprintln!` in `anwendung.rs` (669, 678, 687, 2407, 2473, 2483), jedes
  gefolgt von `std::process::exit`; keines in den beiden Abgriff-Funktionen.
- `NSAlert::new` an genau zwei Stellen, `hinweis.rs:55` (`runModal`) und
  `blaetter/mod.rs:341` (`beginSheetModalForWindow_completionHandler`).
- `grep -rn 'Command::new\|process::Command' crates/krk-ui/src crates/krk-core/src`
  liefert null Treffer; die alte Fassung liefert elf.
- `crates/krk-core/src/stapelumbenennen/` und
  `crates/krk-core/tests/stapelumbenennen.rs` existieren, `lib.rs:18` trägt
  `pub mod stapelumbenennen;`.
- Die Prüfung in `fokus.rs:247` heißt
  `der_terminal_befehl_wird_in_der_leiste_stumm_abgewiesen` und nennt
  `Kommando::TerminalOeffnen`; der Modulkopf von `kommandos/mod.rs` führt für
  `operationen` C11 neben C4.
- Die Startmeldungen gehen in `anwendung.rs:642` über `meldung_zeigen` in die
  Statuszeile, also Rang 3 nach der Tabelle in S16b.
- Das neue Grep für S23 trifft alle sieben in `release.rs` als gültig geprüften
  Schreibweisen und keine der fünf abgelehnten; außerhalb von `appkit/` gibt es
  keine Zeile aus.

## Spec

**Kein Punkt berührt den Spec.** Die kritischste Stelle war Punkt 4: C11 sagt
für die beschädigte Ablagedatei allein zu, dass die Vorbelegung gilt und eine
Meldung erscheint, "genau wie bei den drei vorhandenen Ablagedateien", und nennt
keinen Rang. Die Korrektur folgt dem Spec, statt ihm zu widersprechen. Ein
Entscheidungsdatensatz war deshalb nicht nötig.

## Offen geblieben

Die Statuszeile des Plans (Zeile 4) trägt "35 von 36 Schritten `[DONE]`, offen
allein S6b", während S6b im Rumpf `[DONE]` trägt. Dieselbe Aussage steht im
Reconciliation Log vom 260806-0904. Beide sind nicht geändert, weil der Auftrag
die Statuszeile ausdrücklich stehen ließ; der Punkt ist dem Nutzer gemeldet.

Die Defektdateien sind unverändert und nicht umbenannt; der Nutzer schließt sie
selbst. Es wurde nichts committet.
