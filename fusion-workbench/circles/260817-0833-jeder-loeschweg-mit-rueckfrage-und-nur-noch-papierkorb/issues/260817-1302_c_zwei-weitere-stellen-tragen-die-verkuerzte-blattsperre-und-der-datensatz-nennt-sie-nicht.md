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

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, beide Stellen nachgelesen, eine
Zeilennummer verschoben.** `crates/krk-ui/src/appkit/editor.rs:1298` steht unverändert.
Der Kopfkommentar von `kommando_ausfuehren` liegt jetzt an
`crates/krk-ui/src/appkit/anwendung.rs:2866` und nicht mehr an `:2840`; die Commits des Bündels C
haben Zeilen davor eingefügt. Die dritte und vierte Stelle derselben Formulierung führt
`260817-1419_o_ein-vierter-traeger-der-verkuerzten-blattsperre-…`, dessen Erhebung über
`crates/`, `CLAUDE.md` und `resources/` fünf Zeilen zählt und den Nachzug dieses Datensatzes
mitträgt.

---
Resolved 260818 (coder, Bündel C/D-Nachzug): **beide genannten Stellen nachgezogen, und die
Erhebung hat zwei weitere gefunden, die kein Datensatz nennt.**

Behoben:
- `crates/krk-ui/src/appkit/anwendung.rs`, Kopfkommentar von `kommando_ausfuehren`: die
  Aufzählung der vier Bestandteile nennt jetzt die Ausnahmeliste als eigenen Absatz und sagt,
  dass `immer_erreichbar` drei der vier aufhebt und nicht nur den dritten. Der erste Punkt
  spricht daneben von „der Blattsperre" statt von „einem Blatt", denn er beschreibt einen der
  vier Eingänge und nicht die Lage.
- `crates/krk-ui/src/appkit/editor.rs`: die Begründung nennt die vier Kommandos, beide Quellen
  und den Schluss, dass keines der vier eine Datei öffnet.

**Zwei weitere Träger, in keinem Datensatz und in keiner vorigen Erhebung** — beide in
`crates/krk-ui/src/kommandos/operationen.rs`, und der erste ist die Wurzel, aus der die
übrigen vier ihre Formulierung haben:
- der Abschnittskopf und der Doc-Kommentar von `waehrend_blatt_erlaubt` selbst hießen „Was
  durchkommt, solange ein Blatt steht" und antworteten „Genau der Abbruchbefehl". Das ist eine
  Aussage über die Lage und nicht über diese Regel; sie heißen jetzt „Was die Blattsperre
  selbst durchlässt" und nennen den zweiten Eingang samt der Zahl vier.
- die beiden Proben derselben Regel trugen die verkürzte Aussage im Namen
  (`bei_stehendem_blatt_kommt_allein_der_abbruch_durch`,
  `waehrend_eines_blattes_bleibt_es_bei_dem_einen_abbruch`) und in ihrer Fehlschlagsmeldung,
  während die Nachbarprobe in `zulaessigkeit.rs` die vollständige Aussage im Namen trägt. Sie
  heißen jetzt `die_blattsperre_laesst_allein_den_abbruch_durch` und
  `in_der_blattsperre_bleibt_es_bei_dem_einen_abbruch`.

**Die Zahl ist jetzt gemessen und nicht behauptet.**
`kommandos::zulaessigkeit::tests::waehrend_eines_blattes_kommen_genau_diese_vier_durch` zählt,
welche Kommandos `zulaessig` bei stehendem Blatt durchlässt, prüft die Länge gegen 4 und
schreibt die vier Namen aus. Sie unterscheidet sich von der Nachbarprobe
`waehrend_eines_blattes_kommt_allein_der_abbruch_und_die_ausnahmeliste_durch`, die `zulaessig`
gegen `waehrend_blatt_erlaubt || immer_erreichbar` prüft und damit die Zusammenrechnung hält,
aber nichts über die Zahl sagt. Nachgewiesen: ein vierter Eintrag auf `immer_erreichbar`
(probeweise `Kommando::Notizzettel`) lässt sie rot werden und meldet
`[FensterEinblenden, FensterSchliessen, Abbrechen, Beenden, Notizzettel]`; zurückgenommen.

**Zwei Träger stehen weiter**, beide außerhalb des Auftragsumfangs: `CLAUDE.md:124` und
`resources/default-keymap.toml:710`. Die Erhebung dazu steht bei `260817-1419_c_ein-vierter-…`.

**Nachgelesen und weiterhin kein Befund:** `krk-core/src/tasten/belegung.rs:638` und `:952`
sagen beide etwas über ein **einzelnes** Kommando (`Notizzettel`, `TabSchliessen`), leiten es
korrekt aus `waehrend_blatt_erlaubt` her, und keines der beiden steht auf `immer_erreichbar`.
`anwendung.rs:406` ist eine Aussage über den Stand bis S16 und sagt das. `anwendung.rs:6440`
und `zulaessigkeit.rs:613` nennen die Ausnahmeliste und sind vollständig.

Abnahme: `make check` — Exit 0.
