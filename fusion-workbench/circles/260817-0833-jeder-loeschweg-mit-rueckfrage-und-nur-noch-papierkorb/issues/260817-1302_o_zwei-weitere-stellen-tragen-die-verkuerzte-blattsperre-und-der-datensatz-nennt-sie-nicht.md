Zwei weitere Stellen tragen die verkuerzte Blattsperre, und der Datensatz zu Befund 6 nennt sie nicht

---

`260817-1111_*_die-begruendung-an-loeschauftrag-stellen-nennt-eine-ausnahme-es-sind-vier.md`
nennt neben der behobenen Stelle zwei weitere Traeger derselben verkuerzten Formulierung:
`blaetter/mod.rs:272` und `CLAUDE.md`. Eine Suche ueber den ganzen Baum findet zwei, die er
nicht nennt:

- `crates/krk-ui/src/appkit/anwendung.rs:2840`, der Kopfkommentar von `kommando_ausfuehren`
  selbst: „Die vier Bestandteile und ihre Herleitung stehen in `kommandos::zulaessigkeit`.
  Kurz: ein Blatt laesst allein den Abbruch durch, … ein fremdes Schluesselfenster haelt
  alles ausser der Ausnahmeliste auf, …". Die Ausnahmeliste steht hier ausdruecklich da,
  aber allein am dritten Glied; `immer_erreichbar` hebt auch das erste auf.
- `crates/krk-ui/src/appkit/editor.rs:1298`: „Solange das Blatt aus C4 steht, kommt kein
  weiterer Oeffnungsbefehl durch: der Anwendungsdelegierte weist jedes Kommando ausser dem
  Abbruch ab, solange ein Blatt am Fenster haengt." Die Aussage traegt dort eine
  Entwurfsbegruendung — „ein zweites Feld fuer die zurueckgehaltene Datei unterschiede damit
  etwas, das nicht auseinanderlaufen kann".

---

**Gefunden am:** 260817-1302, waehrend der Behebung der Befunde 4 bis 7 aus
`reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`
**Gefunden von:** coder
**Schwere:** niedrig. Kein Fehlverhalten am Code, und beide Schluesse halten: keiner der drei
Befehle der Ausnahmeliste oeffnet eine Datei, und keiner aendert eine Lage, auf die eine der
beiden Stellen sich stuetzt. Falsch ist allein die Begruendung.
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:2840`,
`crates/krk-ui/src/appkit/editor.rs:1298`
**Baumstand:** `3fcd375` mit den Aenderungen von T2 dieser Sitzung
**Domain:** code

## Was der Baum traegt

Waehrend ein Blatt steht, laesst `zulaessigkeit::zulaessig` genau vier Kommandos durch:
`Kommando::Abbrechen` ueber `operationen::waehrend_blatt_erlaubt`
(`crates/krk-ui/src/kommandos/operationen.rs:266-268`) und `Kommando::Beenden`,
`Kommando::FensterSchliessen` und `Kommando::FensterEinblenden` ueber
`zulaessigkeit::immer_erreichbar` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:197-202`),
das die Blattsperre ausdruecklich mit aufhebt.

Zwei weitere Stellen, die dieselben Worte tragen, sind **keine** Befunde und stehen hier nur,
damit die naechste Erhebung sie nicht mitzaehlt: `krk-core/src/tasten/belegung.rs:640` und
`:955` sagen beide etwas ueber `waehrend_blatt_erlaubt` und nicht ueber die ganze Sperre, und
was sie darueber sagen, ist richtig. Ebenso `anwendung.rs:405`, eine Aussage ueber den Stand
bis S16.

## Warum die Stellen nicht im selben Zug behoben sind

Der Auftrag zu T2 vom 260817 zog seine Grenze um die vier genannten Datensaetze und die zwei
Dateien, die sie betreffen; `blaetter/mod.rs` durfte allein an der einen von `260817-1111`
genannten Zeile angefasst werden. `editor.rs` steht in keinem der vier Datensaetze.
`anwendung.rs:2840` liegt in einer der beiden Dateien, aber ausserhalb der vier Korrekturen,
und eine fuenfte Prosaaenderung waere in den Commit gefallen, den der Orchestrator gegen die
vier Datensaetze liest.

`CLAUDE.md:123` bleibt ebenfalls offen und ist von `260817-1111` genannt. Dort steht neben
der verkuerzten Formulierung eine zweite Verengung derselben Art: die Aufzaehlung der vier
Bestandteile von `zulaessigkeit::zulaessig` nennt `immer_erreichbar` gar nicht, obwohl es
drei der vier aufhebt.
