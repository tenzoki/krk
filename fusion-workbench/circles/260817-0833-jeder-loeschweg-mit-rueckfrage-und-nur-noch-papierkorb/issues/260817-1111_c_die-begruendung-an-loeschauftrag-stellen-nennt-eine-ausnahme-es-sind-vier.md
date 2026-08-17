# Die Begründung an `loeschauftrag_stellen` nennt eine Ausnahme, es sind vier

**Datum:** 260817-1111
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 6
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`
**Baumstand:** `472eb81`

## Der Befund

`anwendung.rs:4676-4680` begründet, warum `loeschauftrag_stellen` die Fensterseite im Rückruf
neu lesen darf:

> Solange ein Blatt steht, weist `Anwendungsdelegierter::kommando_ausfuehren` jedes Kommando
> ausser dem Abbruch ab

Es sind vier Ausnahmen und nicht eine. `zulaessigkeit::immer_erreichbar` (`:197-202`) lässt
neben dem Abbruch auch `Kommando::Beenden`, `Kommando::FensterSchliessen` und
`Kommando::FensterEinblenden` durch; die Ausnahmeliste hebt die Blattsperre ausdrücklich mit
auf (`zulaessigkeit.rs`, Abschnitt „Die Ausnahmeliste").

## Was trotzdem hält

Der Schluss. Einzeln nachgerechnet: keiner der drei ändert `modell.aktiv()`. `beenden` ruft
`terminate:` (`:4104`), `fenster_schliessen` ruft `performClose:` (`:4080`), das ein Fenster
mit anhängendem Blatt nicht schließt, `fenster_zeigen` ruft `makeKeyAndOrderFront:` und
`activate` (`:4029-4030`). Falsch ist die Begründung, nicht das Ergebnis.

## Richtung

Die Begründung auf die vier Ausnahmen stellen und den Schluss daran hängen: keiner der drei
zusätzlich zugelassenen Befehle ändert die aktive Seite. Dieselbe verkürzte Formulierung steht
in `blaetter/mod.rs:272` und in `CLAUDE.md`; hier trägt sie zum ersten Mal eine Zusage über
eine zerstörende Handlung, und deshalb gehört sie hier ausgeschrieben, gleich ob die anderen
beiden mitgezogen werden.

---
Abgleich 260817-1129 (reconciler): **offen, am Baum nachgelesen.** Die verkürzte Begründung steht unverändert an `anwendung.rs:4676-4678` („jedes Kommando ausser dem Abbruch ab"), während `zulaessigkeit::immer_erreichbar` weiter vier Kommandos durchlässt.

---
Resolved 260817-1302 (coder, T2): **behoben.** Die Begruendung an `loeschauftrag_stellen`
steht jetzt ausgeschrieben und sagt voran, warum sie es hier tut: die Zusage einer
zerstoerenden Handlung haengt daran. Sie nennt die vier durchgelassenen Kommandos mit ihren
beiden Quellen — `Abbrechen` ueber `operationen::waehrend_blatt_erlaubt`, `Beenden`,
`FensterSchliessen` und `FensterEinblenden` ueber `zulaessigkeit::immer_erreichbar`, das die
Blattsperre ausdruecklich mit aufhebt — und haengt den Schluss einzeln daran: der Abbruch
schliesst das Blatt, und der Rueckruf laeuft dann mit `bestaetigt == false` und stellt keinen
Auftrag; `beenden` ruft `terminate:`, `fenster_schliessen` ruft `performClose:`, das ein
Fenster mit anhaengendem Blatt nicht schliesst, `fenster_zeigen` ruft
`makeKeyAndOrderFront:` und `activate`. Geschrieben wird `aktiv` von keinem davon.

`blaetter/mod.rs` ist mitgezogen: die Stelle liegt nach T1 an `:296-298` und nicht mehr an
`:272`. Sie nennt jetzt die vier und dazu, warum ihr Schluss haelt — keiner der drei
zusaetzlich zugelassenen Befehle liegt ab Werk auf einer Eingabetasten-Kombination des Blattes,
sie liegen auf `cmd+q`, `shift+cmd+w` und `cmd+n` (`resources/default-keymap.toml`).

**`CLAUDE.md` ist nicht mitgezogen**, und zwei weitere Traeger derselben Formulierung, die
dieser Datensatz nicht nennt, ebenfalls nicht: `anwendung.rs:2840` und `editor.rs:1298`. Sie
stehen als eigener Datensatz,
`260817-1302_o_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-und-der-datensatz-nennt-sie-nicht.md`,
mit der Begruendung fuer den Aufschub und mit den drei Stellen, die dieselben Worte tragen und
**keine** Befunde sind (`belegung.rs:640`, `belegung.rs:955`, `anwendung.rs:405`).
