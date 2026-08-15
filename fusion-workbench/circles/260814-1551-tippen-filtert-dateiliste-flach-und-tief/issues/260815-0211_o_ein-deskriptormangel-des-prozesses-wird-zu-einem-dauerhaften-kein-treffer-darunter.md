# Ein Deskriptormangel des Prozesses wird zu einem dauerhaften „kein Treffer darunter"

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C3.8, C3.10, C3.13, C2.5, C2.6; `crates/krk-core/src/verzeichnis/durchlauf.rs:50-55` (Modulkopf, „ein offener Deskriptor je Ebene"), `:226` (`Schwungleser::oeffnen` am Auftrag), `:250` (`Schwungleser::oeffnen` beim Abstieg); `crates/krk-core/tests/verzeichnis.rs`, `der_durchlauf_kennt_keine_tiefengrenze`

---

## Befund

Der Durchlauf behandelt **jeden** Fehlschlag von `Schwungleser::oeffnen` als Aussage über den Ordner. Am Auftrag wird daraus `Some(false)`, also der endgültige Befund „kein Treffer darunter" (`durchlauf.rs:226`); beim Abstieg wird der Unterordner stillschweigend übergangen (`durchlauf.rs:250`).

`EMFILE` und `ENFILE` sind aber keine Eigenschaft des Ordners, sondern ein Zustand des **Prozesses** — und der Durchlauf erzeugt ihn selbst: er hält einen offenen Deskriptor je Ebene, weil der übergeordnete Ordner nach der Rückkehr aus dem Abstieg weitergelesen wird. Der Modulkopf nennt das als Preis (`durchlauf.rs:53-55`), nicht als Fehlerquelle.

**Die Folge ist eine falsche Zeile ohne jede Meldung.** Ein Ordner, unter dem ein Treffer liegt, fällt aus der Liste, weil der Durchlauf ihn als „kein Treffer darunter" entschieden hat. C3.10 deckt das nicht: dort geht es um einen Ordner, den der Durchlauf nicht öffnen **kann**, nicht um einen, den er wegen seiner eigenen Deskriptoren nicht öffnen **konnte**.

**Zweite Hälfte:** solange der Durchlauf die Deskriptoren hält, konkurriert der Rest von KRK um dieselbe Tabelle — Editor, Vorschau, Kopiervorgänge und der Lesevorgang der zweiten Dateiliste öffnen aus demselben Vorrat.

## Nachgestellt

Ein Prüfbaum mit einem 400 Ebenen tiefen Zweig, der Treffer `ziel-xyz.txt` ganz unten, dazu ein Ordner mit einer Verknüpfung auf sich selbst. Aufträge: `tief` und `kreis`. Ein Programm außerhalb des Baums, das `krk_core::verzeichnis::Durchlauf` unmittelbar ruft:

```
$ cargo run                       # Deskriptorgrenze der Sitzung (hier 1048576)
Befunde: [(0, true), (1, false)]

$ (ulimit -n 64; cargo run)
Befunde: [(0, false), (1, false)]
```

Derselbe Baum, dieselbe Frage, zwei verschiedene Antworten. Im zweiten Lauf verschwindet `tief` aus der Liste, obwohl der Treffer darunter liegt. Der Verknüpfungskreis endet in beiden Läufen richtig; er ist nicht Gegenstand dieses Befunds.

## Warum keine Probe das sieht

`der_durchlauf_kennt_keine_tiefengrenze` legt 200 Ebenen an und ist grün. `cargo test` erbt die Deskriptorgrenze der Anmeldesitzung, und die ist auf diesem Gerät angehoben. Ein aus dem Finder gestartetes `KRK.app` erbt dagegen die Grenze von `launchd`:

```
$ launchctl limit maxfiles
	maxfiles    256            unlimited
```

Die Probe und das ausgelieferte Bündel laufen also unter verschiedenen Grenzen, und die Probe läuft unter der großzügigeren. Ob KRK.app auf dem Referenzgerät tatsächlich 256 erhält, ist **nicht gemessen** — das verlangt einen Lauf am Bündel und ist Nutzerarbeit; gemessen ist allein die Voreinstellung von `launchctl`.

## Erreichbarkeit, ehrlich

Ein Baum mit rund 250 Ebenen ist ungewöhnlich. Die Grenze ist obendrein durch `PATH_MAX` gedeckelt: der Durchlauf baut seine Pfade mit `join`, und ein Pfad über 1024 Bytes fällt ohnehin in den Zweig „lässt sich nicht öffnen". Die praktische Obergrenze liegt damit bei etwa `PATH_MAX/2` gehaltenen Deskriptoren. Der Fehler ist also selten erreichbar — aber wenn er erreicht wird, ist er unsichtbar, und die Zeile fehlt ohne Grund.

## Was zu entscheiden ist

Drei Stufen, aufsteigend im Aufwand:

1. `EMFILE`/`ENFILE` von den übrigen Öffnungsfehlern trennen und den Auftrag dann **nicht** entscheiden (kein Befund, wie beim Abbruch). Der Ordner bliebe unentschieden statt falsch entschieden.
2. Zusätzlich die Zahl der zugleich offenen Ebenen deckeln und tiefer liegende Ebenen über einen neu geöffneten Pfad fortsetzen.
3. `RLIMIT_NOFILE` beim Start auf die harte Grenze anheben. Das berührt den ganzen Prozess und gehört dann nicht in dieses Modul.

Welche Stufe, ist eine Nutzerfrage; dieser Datensatz nimmt sie nicht vorweg.
