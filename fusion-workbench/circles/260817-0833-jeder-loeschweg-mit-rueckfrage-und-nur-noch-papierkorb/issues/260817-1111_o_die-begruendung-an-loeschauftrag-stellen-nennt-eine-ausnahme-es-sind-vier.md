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
