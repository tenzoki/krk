# Planner: Implementierungsplan der Runde 7

**Datum:** 260813-0215
**Agent:** planner (dispatch durch den Nutzer, autonom auf den Empfehlungen der offenen Datensätze)
**Gegenstand:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md`, Fassung nach dem Nachzug vom 260813-0130

## Was entstanden ist

| Datei | Was |
|---|---|
| `planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md` | Der Plan: fünfzehn Schritte, vier Mermaid-Bilder, Abnahmeliste für den Lauf am Bündel |
| `decisions/260813-0159_o_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md` | Neue Nutzerfrage, in der Planung aufgefallen |
| `issues/260813-0201_o_ein-kommentar-in-blaetter-mod-rs-nennt-eine-taste-variante-die-es-nicht-gibt.md` | Defekt, in der Planung aufgefallen |

Kein Code angefasst, kein `cargo build`, kein `cargo test`, kein Bündelbau, kein Vordergrundlauf, keine Messung. `target/KRK.app` ist unangetastet.

## Die Grundlage

Zwei Erhebungen am Baum liefen parallel und sind in den Plan eingegangen: eine über Ablage,
Sperren, Prüfordner und `NSWorkspace`, eine über Menü, Belegungsmodell, Belegungsansicht und
Auslieferungsbelegung. Dazu die eigene Lesung von `appkit/ereignisse.rs`,
`appkit/anwendung.rs` (`kommando_ausfuehren`, `fokus`, `blatt_steht`, `tastendruck_fangen`),
`kommandos/fokus.rs`, `kommandos/operationen.rs`, `core/text/suche.rs` und
`core/tests/belegung.rs`.

## Die vier Entwurfsentscheidungen, die der Spec dem Planner überlassen hat

**Wie ein Menüeintrag sein Kommando trägt: im `tag`.** `Kommando::KENNUNGEN` führt jedes
Kommando genau einmal, sein Index ist damit im Prozess stabil, und `representedObject`
verlangte eine Wrapperklasse um ein Rust-Enum, die der Baum sonst nirgends braucht. Ein
Sammelselektor `krkKommando:` kostet eine Methode statt sechsundsiebzig.

**Woran die Ausgrauung hängt: an `validateMenuItem:` am Anwendungsdelegierten.** Der Baum
kennt die Methode heute nicht, und `autoenablesItems` steht auf seinem Vorgabewert. Die
Ausgrauung wird damit **abgefragt** und nicht gesetzt; sie braucht keinen Beobachter am Fokus,
und die Regel aus `CLAUDE.md`, dass jede an den Fokus gehängte Anzeige durch
`makeFirstResponder:` gehen muss, greift hier nicht.

**Welche Mechanismen die beiden Sperren tragen: beide `flock`, auf zwei Dateien.** Die Std-
Mittel des Baums (`OpenOptions::create_new` in `operation/anlegen.rs:32-36`, `renamex_np` mit
`RENAME_EXCL` in `verzeichnis/sys.rs:668`) hinterlassen eine Marke, die ein Absturz nicht
wegräumt, und scheitern damit an C3.13. `flock` gibt der Kern beim Prozessende von sich aus
frei. Der Aufruf kommt in `verzeichnis/sys.rs`, die eine Datei des Kerns mit
`#![allow(unsafe_code)]`; ihr Modulkopf hat dieselbe Namensspannung für `fcntl` schon
angenommen. Eine dritte Datei mit der Ausnahme entsteht nicht, C4.5 bleibt erfüllt.

**Wo die Zulässigkeitsfunktion wohnt: in `kommandos/zulaessigkeit.rs`.** Reine Funktion neben
`fokus::wirkt`, in dem Verzeichnis, das keine Zeile AppKit führt. Die Tafel aus 140 Fällen
läuft ohne Fenster.

## Die zwei Dinge aus der Diagrammprüfung

Beide sind im Plan, und beide waren nötig.

**Die Wache vor dem Sprungmarkenpuffer.** Der Fokusvorbehalt steht heute als früher Ausstieg
**vor** dem Nachschlag (`appkit/ereignisse.rs:487-490`) und schützt damit beide gefährdeten
Zweige zugleich. Wird er zum Bestandteil einer Regel, die nur der Kommandozweig stellt,
verliert `Nachschlag::Sprungmarke` seine Wache. S2 baut sie ausdrücklich; das erste Bild des
Plans zeigt den dritten Ausgang des Nachschlags, den das Spec-Bild nicht zeigt; eine
Zählprobe hält fest, dass `ersthelfer_gehoert_appkit` drei Frager hat und alle drei dieselbe
Funktion rufen.

**Die Wächter im Zustandsautomaten.** Der Automat des Plans trägt vier `[keine Aufnahme]`, und
S10 setzt den Vorrang als zwei hintereinandergeschaltete Stationen im Fänger um, nicht als
zwei unabhängige Regionen.

## Was in der Planung aufgefallen ist und im Spec nicht steht

**C2.3 verlangt zwei Dinge, die die heutige Gliederung nicht zugleich erfüllt.** „In der
Reihenfolge von `Funktionsbereich::ALLE`" und „Anwendung vorn, Fenster hinten" widersprechen
einander: `ALLE` führt Anwendung an siebter und Fenster an sechster Stelle
(`belegungsmodell.rs:104-114`). Dazu heißt `Funktionsbereich::Textbefehle` in der Anzeige
„Textbefehle", während die Randbedingung des Menüdatensatzes ein Obermenü namens „Bearbeiten"
verlangt. Der neue Datensatz legt die Frage vor; der Plan fährt auf der Empfehlung, `ALLE`
umzusortieren und den einen Namen zu ändern, damit es bei einer Gliederung mit drei Abnehmern
bleibt.

**Die drei heutigen Menüeinträge mit Kommando laufen an `kommando_ausfuehren` vorbei.**
`beenden:`, `fensterEinblenden:` und `fensterSchliessen:` sind eigene Selektoren am
Delegierten. C2.14 schließt die Lücke, und der Wegfall von `beenden:` bringt die Frage nach
der Zweitform „Quit and Keep Windows" auf Opt+Cmd+Q zurück; sie steht in der Abnahmeliste.

**Die Ablage wird je Start zweimal geöffnet**, und die erste Öffnung liegt vor
`NSApplication` (`tasten/belegung.rs:1310-1311` gegen `appkit/anwendung.rs:997`). Beide
schreiben, denn schon `Ablage::laden` legt eine beschädigte Datei beiseite. Zwei Deskriptoren
desselben Prozesses auf dieselbe Sperrdatei blockierten einander; die Lebensdauern sind heute
getrennt, und S12 schreibt die Regel in den Doc-Kommentar.

**Die Begründung an `beiseite_legen` fällt mit der zweiten Instanz weg**
(`ablage/mod.rs:394-397`). Die Schreibsperre deckt den Fall ab; der Absatz braucht eine neue
Begründung und keine neue Bauart.

**C2.12 und C2.13 sind ohne Bündel prüfbar.** `--menue-protokoll` gibt nach `finishLaunching`
aus und kehrt ohne Fenster zurück. Der Weg ist `cargo run -p krk-ui --bin krk --
--menue-protokoll` und ausdrücklich **nicht** `make menue`, das an `bundle` hängt.

## Die Reihenfolge des Rust- und des Belegungsschritts

Beide Reihenfolgen färben Proben zwischenzeitlich rot, und die gewählte färbt genau eine.
Nach S14 (Rust) schlägt `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`
(`core/tests/belegung.rs:830`) fehl, weil die Auslieferungsbelegung `weitere_instanz` noch
nicht kennt; S15 macht sie grün. Umgekehrt wäre die eingebettete Auslieferungsbelegung
ungültig, und fast jede Probe des Kerns fiele. Die Abnahme von S14 nennt die eine rote Probe
namentlich, damit ein Fehlschlag daneben nicht als planmäßig durchgeht.

## Werkzeug

Alle vier Mermaid-Blöcke sind mit `mmdc` 11.16.0 über `npx` nach PNG gerendert und angesehen
worden. Drei Bilder sind danach nachgebessert worden: zwei mehrdeutige Kantenbeschriftungen im
Flussbild, eine Überlagerung zweier Beschriftungen im Zustandsautomaten, und die
Sperrenschicht des Ablagebildes, deren Kante am falschen Kasten vorbeizulaufen schien.

## Was offen bleibt

Fünf Nutzerfragen binden die Runde, vier vom Shaper und eine aus dieser Planung. Der Plan
nennt je Schritt, welche Empfehlung er voraussetzt. Nächster Schritt ist die Diagrammprüfung
des Plans und danach das Plan-Tor beim Nutzer.
