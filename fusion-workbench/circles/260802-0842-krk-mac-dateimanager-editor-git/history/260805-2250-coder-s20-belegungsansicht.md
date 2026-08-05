# S20: Belegungsansicht (C3) — coder

**Status:** Complete
**Agent:** coder
**Auftrag:** Planschritt 20 aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` — die Belegungsansicht, die F1 ihre Wirkung gibt.

## Was entstanden ist

- `crates/krk-core/src/tasten/belegung.rs` (erweitert): die Aufzählung `Kommando` trägt `BelegungAnsehen` mit der Kennung `belegung_ansehen`; `Kommando::aus_kennung("belegung_ansehen")` liefert damit nicht mehr `None`. Wirkungsbereich: **Ueberall** — die Ansicht zeigt die Belegung der ganzen Anwendung und gehört keinem Bereich, dasselbe Muster wie das Ein- und Ausblenden der Bereiche (F3).
- `crates/krk-ui/src/belegungsmodell.rs` (neu, ohne objc2): die Arbeitskopie der Belegung, solange die Ansicht offen ist. Zeilen = `Belegung::funktionen()` (eine je Funktion, alle Kombinationen darin), Zuweisung über `Belegung::zuweisen`, Zurücksetzen über `Belegung::zuruecksetzen`, Konfliktmeldung wörtlich aus dem Kern. Anzeigeform rein mechanisch aus der Schreibweise (`shift+cmd+k` → `Shift+Cmd+K`, `f3` → `F3`) — keine zweite Namensliste. Acht Prüfungen.
- `crates/krk-ui/src/appkit/belegungsansicht.rs` (neu): das Blatt am Hauptfenster über die `Blatt`-Hülle: Tabelle (Funktion | Belegung), Schaltflächen Zuweisen (Leertaste), Auslieferungszustand (Cmd+R), Fertig (Eingabetaste), Meldungszeile. Reservierte Funktionen und Menü-Zusteller stehen im Funktionstext.
- `crates/krk-ui/src/appkit/ereignisse.rs` (erweitert): der Abgriff bekommt einen **Fänger** vor Fokusvorbehalt und Nachschlag. Während der Aufnahme ist der Tastendruck Eingabe und kein Befehl; auch eine vergebene Kombination erreicht so die Konfliktmeldung, statt die Funktion auszulösen. Es bleibt ein Abgriff — keine Ansicht bekommt eine eigene `keyDown:`-Behandlung.
- `crates/krk-ui/src/appkit/anwendung.rs` (erweitert): `belegung` und `tastenabgriff` sind veränderlich (`RefCell`); `belegung_ansehen` öffnet das Blatt (Griff nach `offenes_blatt`, damit `esc` es wie jede Rückfrage schließt), `belegungsansicht_verlassen` sichert bei Änderung nach `keymap.toml`, setzt die neue Belegung und zieht Hauptmenü und Abgriff nach — die Umbelegung wirkt sofort, nicht erst nach Neustart. Ohne Änderung bleibt `keymap.toml` unberührt.
- `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/appkit/mod.rs`: Einbindung und Modulkopf.

## Prüfung der sechs C3-Abnahmekriterien

Am laufenden Bündel, mit synthetischen Tastendrücken über KRKs eigene Ereignisschlange (`postEvent:atStart:`, temporäre Prüfsonde, danach entfernt):

1. **Listen + Zuweisung durch Drücken:** F1 → Ansicht offen; Leertaste → Aufnahme; f9 → Meldung „»Vorschau anzeigen und ausblenden« liegt jetzt auf F9." (Schirmabzug s20-c).
2. **Eine Zeile je Funktion:** am Bündel sichtbar (Papierkorb und Endgültig löschen als zwei Zeilen); per Prüfung `eine_zeile_je_funktion` (Modell zählt die Funktionen der Belegung ab, keine doppelt).
3. **Konfliktmeldung mit Nennung der anderen Funktion:** am Bündel — Aufnahme, dann f5: „die Kombination f5 gehoert schon der Funktion "In das andere Fenster kopieren" (kopieren) …" (Schirmabzug s20-g); zusätzlich Prüfung `eine_vergebene_kombination_meldet_die_andere_funktion`.
4. **Zurücksetzen:** am Bündel — F1, Cmd+R, Fertig; die geschriebene `keymap.toml` ist nach Entfernen der Kommentarzeilen **byte-identisch** mit `resources/default-keymap.toml` (diff leer).
5. **Überlebt Beenden und Neustart:** per Kommando — nach dem Verlassen steht `~/Library/Application Support/KRK/keymap.toml` mit `tasten = ["f3", "cmd+y", "f9"]` bei `vorschau_umschalten`; nach Neustart blendet f9 das Vorschaufenster aus (Schirmabzüge s20-e/f, vorher/nachher).
6. **F1 bis F12, kein „Fn+":** Textsuche über den Diff findet kein `Fn+` und kein `fn+`; Prüfung `keine_zeile_traegt_fn_und_die_funktionstasten_heissen_f1_bis_f12`.

Dazu: F4-Eintrag als „reserviert für den Editor" (Bündel sichtbar + Prüfung), Taste ohne Namen → Auskunft statt Zeile (Prüfung `eine_taste_ohne_namen_wird_gemeldet_statt_geschrieben`), fn unterscheidet keine Kombination (Prüfung `fn_unterscheidet_keine_kombination`, S7-Normalisierung), `belegung_ansehen` in der Aufzählung `Kommando` (Diff + Prüfung).

**make check:** alle vier grün (build, test, clippy -D warnings, fmt). Bündel signiert gebaut.

## Prüfstand

`~/Library/Application Support/KRK/keymap.toml` war vor der Prüfung nicht vorhanden und ist wieder gelöscht; `session.toml` aus der Sicherung wiederhergestellt. Die Prüfsonde (KRK_PROBE) ist vollständig aus dem Code entfernt; Schirmabzüge liegen im Sitzungs-Scratchpad.

## Entscheidungen im Rahmen des Schritts

- **Wirkungsbereich Ueberall** für `belegung_ansehen` (Begründung im Enum-Kommentar).
- **Sofortwirkung statt Neustart:** Nach dem Verlassen mit Änderung werden Hauptmenü und Tastenabgriff aus der neuen Belegung neu aufgebaut — C3 nennt die freie Belegbarkeit die Grundhaltung, und ein gesetzter F9, der bis zum Neustart nichts tut, wäre ein Defektbericht mit Ansage. Kein neuer Mechanismus: derselbe Aufbauweg wie beim Start.
- **esc bricht die Aufnahme ab** (nacktes esc; mit Zusatztaste bleibt es zuweisbar). Preis: esc selbst ist nur von Hand in `keymap.toml` belegbar — es trägt ab Werk ohnehin `abbrechen`.
- **Kein Entfernen einzelner Kombinationen** in der Ansicht: der Planschritt nennt es nicht; Rückwege sind das Zurücksetzen und die handgepflegte Datei.
