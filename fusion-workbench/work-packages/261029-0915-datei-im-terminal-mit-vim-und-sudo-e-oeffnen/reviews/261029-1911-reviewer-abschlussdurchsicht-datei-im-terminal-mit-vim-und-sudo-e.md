# Abschlussdurchsicht: Datei im Terminal mit vim oder sudo -e öffnen

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `fd82056a..7a69a051`
**Not-opened:** none
**Review domain:** both (code; Belegungsdatei als Datenteil)
**Cross-references:** 261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0920-wie-kommt-ein-befehl-wie-vim-datei-in-die-eingestellte-terminal-anwendung.md, 261029-0921-welcher-menueeintrag-zeigt-opt-cmd-e-editor-schliessen-oder-mit-vim-bearbeiten.md, 261029-0957-geht-eine-nicht-schreibbare-datei-in-einem-beschreibbaren-ordner-an-sudo-e-oder-an-vim.md, 261029-1911-drei-proben-zitieren-den-plan-des-terminalpakets-in-sternform-und-der-markerlose-name-loest-nicht-auf.md, 261029-1911-der-modulkopf-von-terminal-rs-nennt-zwei-wege-und-einen-ordner-seit-dem-skript-sind-es-drei-wege.md

## Zusammenfassung

Das Paket hält seine Sicherheits- und Fadenzusagen. Die drei Nutzerentscheide sind umgesetzt, auch der zur Ordnerfrage (Möglichkeit 2), obwohl der Plan auf Möglichkeit 1 gebaut war. Gefunden sind zwei Dokumentationsmängel niedriger Schwere und kein Defekt im Verhalten. Gelesen ist alles am Quelltext. Gebaut und geprüft ist hier nichts: `make check` und die Abnahme hat der Nutzer laut Auftrag auf seinem Mac gefahren.

## Summen

Critical 0 · High 0 · Medium 0 · Low 2

## Geprüft und gehalten

**Skript und Quoting** (`crates/krk-core/src/terminalskript.rs`, `gequotet`, `skripttext`). Jeder Pfad steht in einfachen Anführungszeichen, `'` wird zu `'\''`, und beide Zeilen tragen `--` (`cd -- '<o>'`, `exec vim|sudo -e -- '<d>'`). Damit sind Leerzeichen, `"`, `$`, Backtick, `;`, `*`, Zeilenumbruch und ein führendes `-` ohne Sonderfall abgedeckt. Ein Pfad ohne UTF-8 oder ohne Elternordner wird vor jedem Systemaufruf abgewiesen (`gepruefter_pfad`). Die Probe `das_skript_reicht_jeden_namen_unverfaelscht_weiter` (`crates/krk-core/tests/terminalskript.rs`) fährt das Skript unter `/bin/sh` mit Ersatz-`vim`/`sudo`, und zwar für alle zehn Namen, beide Befehle und auch einen Ordner mit `'` und Leerzeichen. Sie vergleicht die Argumente byteweise und prüft, dass das Skript danach weg ist.

**Anlegen und Selbstlöschung.** `OpenOptions::create_new(true).mode(0o700)` ist `O_CREAT|O_EXCL`: eine vorhandene Datei oder eine Verknüpfung am Namen lässt das Anlegen scheitern, überschrieben wird nichts. `rm -f -- "$0"` steht vor `exec` (Probe `der_skripttext_loescht_sich_vor_dem_start`). Scheitert `write_all`, wird das Skript verworfen. Der Name trägt pid, Nanosekunden und einen Zähler. inference: Unter `$TMPDIR` (0700, nutzereigen) kann kein anderes Konto Inhalt unterschieben. Fiele `temp_dir()` auf `/tmp` zurück, bliebe es bei einem möglichen Fehlschlag des Anlegens, aber ohne Einschleusung, denn die Datei hat 0700 und `/tmp` ist sticky. Wie sich `temp_dir()` der Toolchain 1.97.1 ohne `TMPDIR` verhält, ist nicht geprüft: die Quelle von std ist auf dem Gerät nicht installiert.

**Passwort.** Im Diff steht keine Nennung von `preisgeben`, `anmeldehilfe`, `sudo -A` oder `Passwort` außer in Prosa (Doc-Kommentare, `HowTo.md`). `verbinden.rs` ist nicht im Bereich, die Probe `das_passwort_wird_allein_an_netfs_und_ssh_preisgegeben` ist also unverändert.

**unsafe, extern, Prozesse.** Im Diff unter `crates/` steht kein hinzugefügtes `unsafe` und kein `extern`. `sys.rs` ändert allein einen Doc-Kommentar und die Diagrammzeile `└─> terminalskript::vorbereiten`. Das einzige neue `Command::new` steht in `crates/krk-core/tests/terminalskript.rs` und gehört damit zu den Proben, nicht zum Betriebscode.

**Hauptfaden.** `im_terminal_bearbeiten` (`appkit/anwendung.rs`) liest den Ordner über `angezeigter_lokaler_ordner`, den Pfad über `auswahl_pfad` und die Kennung aus den Einstellungen. Danach übergibt er an `terminal::vorbereiten_lassen`, das `metadata`, zweimal `access(2)` und das Schreiben auf dem Faden `krk-terminal` stellt und über `DispatchQueue::main().exec_async` zurückkehrt (`terminal.rs`). Scheitert der Fadenstart, geht das über `Arc<Mutex<Option<F>>>` als `Skriptfehler::Schreiben` ebenfalls über die Hauptschlange zurück, die Antwort wird also nicht verschluckt. Auf dem Hauptfaden bleibt allein `terminalskript::verwerfen` nach einer gescheiterten Übergabe, die benannte Ausnahme E4. Beides halten `der_terminalbefehl_stellt_auf_dem_hauptfaden_keinen_systemaufruf` und `der_terminalbefehl_reisst_nirgends_ab`.

**Wahl.** `befehl_fuer(datei, ordner)` liefert vim, wenn eines von beiden schreibbar ist, sonst `sudo -e`, wie in Entscheidung 261029-0957, Möglichkeit 2. Ein `Err` aus `schreiben_erlaubt` wird zu `Skriptfehler::Pruefen` und startet nichts. Die Tafel der vier Fälle steht in `die_wahl_nimmt_vim_wenn_datei_oder_ordner_schreibbar_sind`, an echten Rechten geprüft in `die_wahl_folgt_den_rechten_von_datei_und_ordner`. Dass `befehl_fuer` einen und `schreiben_erlaubt` zwei Rufer hat, hält `die_wahl_und_die_rechtefrage_haben_genau_ihre_rufer`.

**Belegung.** In `resources/default-keymap.toml` steht `im_terminal_bearbeiten` mit `opt+cmd+e` hinter `terminal_oeffnen`, und der Kommentar bei `editor_schliessen` ist ergänzt. Die Kopfzeile sagt 117 Funktionen mit 118 Kombinationen. Nachgezählt: 117 `[[funktion]]` und 118 Kombinationen. `opt+shift+cmd+e` ist unbelegt, das hält `die_auslieferung_teilt_opt_cmd_e_zwischen_terminal_und_editor`, dazu `konflikte()` leer und `Geteilt` in Dateireihenfolge. Welche der zwei Funktionen ein Anschlag meint, prüft `opt_cmd_e_oeffnet_im_dateifenster_das_terminal_und_schliesst_im_editor` über jede Lage. Dass das Menü das Kürzel beim neuen Befehl zeigt (Entscheidung 261029-0921, Möglichkeit 1), prüft `opt_cmd_e_zeigt_im_menue_das_terminal_und_nicht_das_schliessen`. Ausgegraut ist der Befehl fern und im Archiv, gehalten über die Listen `AUSGEGRAUT`, `LOKALER_PFAD` und `LOKAL`.

**Sprachtabellen.** Die sechs neuen `Text`-Schlüssel und `kommandoname` stehen in `de.rs`, `fr.rs` und `en.rs`, `Text::ALLE` ist von 761 auf 767 gewachsen. Im Betriebscode steht außerhalb der Tabelle kein Stringliteral mit Umlaut (geprüft am Diff ohne `sprache/tabelle`).

**Anleitung.** `HowTo.md` beschreibt die Regel mit Ordnerfrage und ihren Preis (`:w!` ersetzt eine root-Datei im eigenen Ordner), die Grenze bei Verknüpfungen, die Bedingung für andere Terminal-Anwendungen und das Belegen über `cmd+t` statt `cmd+r`. In den beiden Aufzählungen „Ausgegraut“ steht der Befehl.

## Befunde

### L1 — Drei Sternform-Zitate auf einen markerlosen Plan lösen nicht auf (Low)

`crates/krk-core/tests/belegung.rs:3361`, `crates/krk-ui/src/kommandos/zulaessigkeit.rs:2707` und `crates/krk-ui/src/menuemodell.rs:1123` zitieren den Plan in Sternform. Der Plan trägt aber keinen Marker. Die Sternform verlangt nach dem Stempel einen Marker, der Name hat dort einen Bindestrich, und `find` mit diesem Muster findet nichts. Alle drei Stellen stammen aus `d5e891e0`, kein Zufall an einer einzelnen Stelle. Issue `261029-1911-drei-proben-zitieren-den-plan-des-terminalpakets-in-sternform-und-der-markerlose-name-loest-nicht-auf.md`.

### L2 — Der Modulkopf von `terminal.rs` widerspricht sich (Low)

Titelzeile (`terminal.rs:1`), Diagramm (`:6`) und Überschrift `# Zwei Wege stellen diese Frage` (`:9`) sprechen weiter von Ordnern und zwei Wegen. Der neue Absatz ab `:24` nennt einen dritten Weg mit einem Skript. Issue `261029-1911-der-modulkopf-von-terminal-rs-nennt-zwei-wege-und-einen-ordner-seit-dem-skript-sind-es-drei-wege.md`.

## Beobachtungen ohne Datensatz

- **Tastenwiederholung.** Wer `opt+cmd+e` gedrückt hält, löst je Wiederholung einen Faden, ein Skript und ein Terminalfenster aus. Für `ctrl+o` gilt dasselbe schon heute, und KRK filtert Wiederholungen allein beim Rückschritt (`kommandos/rueckschritt.rs`). Das ist kein neues Muster, deshalb ohne Issue. Bei einer hängenden Freigabe bliebe je Wiederholung ein wartender Faden stehen.
- **Schreibgeschütztes Dateisystem.** `schreiben_erlaubt` antwortet auf `EROFS` mit `Ok(false)`, für Datei und Ordner. Eine Datei auf einem schreibgeschützten Datenträger geht deshalb an `sudo -e`, das nach dem Passwort fragt und dann ebenfalls nicht schreiben kann. inference: Das ist selten, und der Nutzer sieht es im Terminal. Die Regel des Modulkopfs („Wer aus einem unklaren Befund `sudo -e` macht …“) betrifft es nicht, denn `EROFS` ist ein klares Nein. Erwähnt, damit die Frage nicht neu hergeleitet werden muss.
- **Verknüpfung.** Gefragt wird der Ordner der Verknüpfung, nicht der ihres Ziels. Eine Verknüpfung im eigenen Ordner auf eine root-Datei geht deshalb an vim und nicht an `sudo -e`. inference: `sudo -e` wiese eine Verknüpfung ohnehin ab (Vorgabe `sudoedit_follow` aus). Das Ergebnis ist gleichwertig, und `HowTo.md` nennt die Grenze.
- **Werkbank.** Die drei Entscheidungsdatensätze stehen auf `answered`, der Plan auf `in_progress` mit allen sechs Schritten `done`. Den Übergang nach `implemented` mit Commit-Verweis sendet der Dispatcher, nicht diese Durchsicht.
- Der vorhandene Befund zu `ctrl+o` (`261029-0958-ordner-im-terminal-oeffnen-fragt-metadata-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md`) bleibt offen. Der neue Weg folgt ihm ausdrücklich nicht.

## Reihenfolge

Kein Befund hält die Auslieferung an. L1 und L2 sind Aufräumarbeit für `code-implementer` und lassen sich in einem Commit erledigen.
