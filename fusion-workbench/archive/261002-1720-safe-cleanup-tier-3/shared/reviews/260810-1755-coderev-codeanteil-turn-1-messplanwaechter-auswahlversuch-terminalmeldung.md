# Durchsicht: Codeanteil von Turn 1 (`4e66607..HEAD`)

**Sender:** coderev
**Gegenstand:** `crates/krk-bench/src/messen.rs` (`ed5c896`),
`crates/krk-ui/src/kommandos/operationen.rs` (`788c8d8`),
`crates/krk-ui/src/appkit/anwendung.rs` (`6964dde`)
**Sitzung:** `shared/history/260810-1647-orchestrator-session.md`
**Stand des Baus:** grün (`make check`, exit 0)

## Zusammenfassung

Alle drei Änderungen erreichen, wofür sie gebaut sind. Der `Messplanwaechter` schließt die
Lücke, für die er gebaut wurde, und folgt der Bauform der beiden vorhandenen Wächter statt
eine vierte zu erfinden; die Terminal-Meldung nennt die Kennung weiterhin und legt keinen
zweiten Lesepfad an; die verworfenen Rückgabewerte tragen jetzt eine Begründung. Drei
Befunde bleiben, alle niedrig, keiner ein Release-Blocker. Zwei betreffen den Wortlaut von
Zusicherungen, einer eine Prüflücke; einer betrifft einen zusätzlichen Ausgang, den der
Wächter nicht deckt.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 0 |
| Niedrig | 3 |

## Befunde

### 1. Zwei Zusicherungen über den `Auswahlversuch` halten so nicht (niedrig)

`crates/krk-ui/src/appkit/anwendung.rs:2661`, `:3188`.
Datensatz: `shared/issues/260810-1751_o_zwei-zusicherungen-ueber-den-auswahlversuch-halten-so-nicht-wie-sie-geschrieben-sind.md`

**Teil 1.** Für `umbenennen_ausfuehren` ist die Behauptung „`Vorgemerkt` und nie
`Unbekannt`" haltbar: `ordner` kommt unmittelbar vor `ordner_neu_lesen` aus derselben Seite,
alles läuft in einem synchronen Durchgang, `gleicher_ordner` vergleicht denselben Wert mit
sich selbst, und `lesen_starten` (`tabs.rs:642-656`) setzt `lesevorgang` bedingungslos.

Für `anlegen_ausfuehren` hält sie nicht. `ordner` und `seite` werden vor dem Blatt
festgehalten (`anwendung.rs:2635-2636`), das Blatt läuft über
`beginSheetModalForWindow:completionHandler:` (`blaetter/mod.rs:508`) ohne eigene
Ereignisschleife, und die Datenträgerwache ist kein Befehl und wird vom Blatt nicht
angehalten. Ein `willUnmount` während des stehenden Blattes schiebt beide Seiten über
`datentraeger_verloren` (`auffrischung.rs:368-370`) auf das Benutzerverzeichnis, während der
Datenträger noch eingehängt ist (ausdrücklich so im Kommentar bei `anwendung.rs:1821-1823`).
Danach gelingt das Anlegen, `ordner_neu_lesen` findet keine passende Seite, und
`eintrag_waehlen` liefert `Unbekannt`. Der Weg ist derselbe, den der neue Kommentar im Zweig
`UmbenennenImStapel` für sich selbst beschreibt — nur ausgelöst von der Datenträgerwache
statt von einem Befehl.

**Teil 2.** „von den drei Aufrufern allein hier erreichbar" — `eintrag_waehlen` hat fünf
Aufrufstellen, und zwei davon werten `Unbekannt` aus: `eintrag_anspringen`
(`tabelle.rs:1057`) meldet genau den Satz, den der neue Kommentar zwei Zeilen weiter als
Rauschen verwirft, und `messhandlung` (`anwendung.rs:4245-4262`) macht daraus einen
Abbruchgrund des Messlaufs. Gemeint sind die drei Aufrufer aus dem zitierten Datensatz
`260807-0219`, also die drei, die den Wert wegwerfen; im Programmtext steht die Zahl ohne
diesen Bezug.

### 2. Der `Messplanwaechter` entsteht erst nach dem Schreiben (niedrig)

`crates/krk-bench/src/messen.rs:1594-1596`.
Datensatz: `shared/issues/260810-1752_o_der-messplanwaechter-entsteht-erst-nach-dem-schreiben-und-deckt-das-schreiben-selbst-nicht.md`

`std::fs::write` legt an und schreibt. Scheitert das Schreiben nach dem Anlegen, kehrt das
`?` zurück, bevor es einen Wächter gibt, und die angelegte Datei bleibt liegen. `Wegwerfordner`
löst dasselbe andersherum: der Name steht fest, bevor irgendetwas angelegt wird
(`wegwerfordner.rs:39-48`).

**Geprüft und in Ordnung:** Erfolgsweg, jedes `?` in und um die Rundenschleife
(`messen.rs:1034`, `:1041`, `:1053`), Panik-Abwicklung (kein `panic = "abort"` in irgendeiner
`Cargo.toml`), kein `mem::forget`, kein `ManuallyDrop`. Die Lebensdauer hält: die einzige
Leserin ist der Unterprozess, und `warten_bis` (`messen.rs:1684-1697`) wartet auf sein Ende
oder bringt es um. `process::exit` in der Signalwache ist bekannt und liegt als
`shared/issues/260810-1745_*`.

### 3. Die beiden Meldungen des Terminal-Befehls sind ungeprüft (niedrig)

`crates/krk-ui/src/kommandos/operationen.rs:726`, `:751`.
Datensatz: `shared/issues/260810-1753_o_die-beiden-meldungen-des-terminal-befehls-sind-als-einzige-ihres-moduls-ungeprueft.md`

Das Prüfmodul desselben Moduls fasst nahezu jede andere Meldung an, mehrere davon auf den
Wortlaut hin. `kein_terminal` und `terminalordner_fehlt` haben keine Probe. Das fünfte
Abnahmekriterium von C11 — die Meldung nennt die eingestellte Kennung — hängt damit allein am
Abnahmelauf, und der steht für die Runde 2 noch aus.

## Was ausdrücklich in Ordnung ist

- **`#![deny(unsafe_code)]`** ist nicht aufgeweicht. Der Diff über `crates/` enthält weder
  `unsafe` noch ein neues `allow(`.
- **Keine Auffangzweige eingeführt.** Der `match &vorgang.art` in `vorgang_beenden`
  (`anwendung.rs:3185-3212`) zählt weiterhin alle fünf Operationsarten einzeln auf. Die
  beiden anderen Dateien führen keine neue Fallunterscheidung ein.
- **Der `Messplanwaechter` folgt der vorhandenen Bauform**, statt eine vierte zu erfinden:
  `pfad()`-Zugriff wie `Wegwerfordner::pfad`, ungemeldetes Abräumen in `Drop` mit derselben
  Begründung, `Drop`-Reihenfolge zur `Sitzungssicherung` stimmt (`plan` wird vor
  `_sitzung` gebunden und fällt nach ihr).
- **Die alte Abräumzeile ist weg** (`messen.rs`, hinter der Rundenschleife), und die Probe
  `der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung` räumt nicht mehr
  selbst ab.
- **Kein zweiter Lesepfad** für `settings.toml`: `kein_terminal` ist weiterhin eine reine
  Funktion über eine Zeichenkette, und der einzige Aufrufer bleibt `anwendung.rs:1246`.
- **Die eingestellte Kennung steht weiterhin in der Meldung**, also fällt das fünfte
  Abnahmekriterium von C11 nicht.
- **Der Verweis auf `issues/260807-0930_*`** ist eingelöst: die Datei trägt inzwischen `_c_`.

## Übergreifend

Alle drei Befunde haben dieselbe Form: die Änderung ist richtig, und die Begründung daneben
ist eine Spur breiter formuliert als das, was der Programmtext trägt. Das ist bei einem
Projekt, das seine Entwurfsentscheidungen bewusst im Doc-Kommentar ablegt, die teuerste
Fehlerart, weil der nächste Leser genau dort nachschlägt. Der Befund 3 ist die andere Seite
derselben Sache: die Zusage steht im Spec und im Kommentar, aber in keiner Probe.

## Reihenfolge

Kein Release-Blocker. Alle drei sind Aufräumarbeit und können in einem Zug laufen; Befund 1
und 2 sind je ein Absatz beziehungsweise zwei Zeilen, Befund 2 zusätzlich eine
Umstellung von drei Zeilen, Befund 3 zwei neue Proben.
