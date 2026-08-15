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

---
Resolved: Beide Hälften des Befunds sind behoben, in dieser Reihenfolge.

**Die zweite zuerst, weil sie die erste selten macht.** `unterbaum_entscheiden` hält keinen Leser je Ebene mehr. Ein Ordner wird ganz gelesen, seine Unterordner wandern dabei als Pfad auf einen Stapel `offen`, und erst wenn er zu Ende ist, fällt sein `Schwungleser` und der nächste wird geöffnet. Der Durchlauf hält damit zu jedem Zeitpunkt **genau einen** Verzeichnisdeskriptor, gleich wie tief der Baum ist; die Kante „zurück zum übergeordneten Ordner" gibt es nicht mehr. Getauscht ist ein knapper, prozessweit geteilter Vorrat gegen einen reichlichen, der dem Durchlauf allein gehört: `offen` hält je vorgemerktem Ordner einen Pfad. Der Umbau steht im Modulkopf unter `# Ein offener Deskriptor, gleich wie tief der Baum ist`.

**Die erste ist Stufe 1 der drei genannten.** `verzeichnis::sys::ist_deskriptormangel` trennt `EMFILE` und `ENFILE` von den übrigen Öffnungsfehlern, die beiden Konstanten stehen dort neben `EXDEV`, `ECANCELED` und `EWOULDBLOCK`. Trifft einer der beiden, liefert `unterbaum_entscheiden` `None`: der Auftrag bleibt unentschieden, wie beim Abbruch, und der Durchlauf endet. Ein Warten mit erneutem Versuch (die zweite genannte Möglichkeit) stünde für eine Frage, die dieses Modul nicht beantworten kann — ob und wann ein anderer Teil von KRK einen Deskriptor freigibt — und hielte den Arbeitsfaden dabei an. Stufe 3, das Anheben von `RLIMIT_NOFILE` beim Start, ist nicht gefahren; sie berührt den ganzen Prozess und gehört nicht in dieses Modul.

**Die Probe misst den Fall jetzt.** `die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden` legt eine 200 Ebenen tiefe Kette mit dem Treffer ganz unten an und lässt sie von einer Kindprobe entscheiden, die über `/bin/sh` mit `ulimit -n 64` startet. Das Kind misst seine Grenze zuerst selbst, indem es Deskriptoren nimmt, bis keiner mehr kommt; ohne diese Zusicherung bestünde die Probe auch dann, wenn `ulimit` nicht gegriffen hätte. Angelegt und abgeräumt wird der Baum vom Elternteil, weil `remove_dir_all` selbst einen Deskriptor je Ebene hält.

**Gegenprobe gefahren:** mit dem alten `durchlauf.rs` an derselben Stelle meldet sie `treffer: false` bei 61 freien Deskriptoren, also genau den Befund der Nachstellung. Mit dem neuen meldet sie `treffer: true`.

`der_durchlauf_kennt_keine_tiefengrenze` bleibt daneben stehen: sie prüft C3.8 unter der Grenze der Sitzung, die neue prüft es unter der, die ein Bündel bekommt.

Berührte Dateien: `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/src/verzeichnis/durchlauf.rs`, `crates/krk-core/tests/verzeichnis.rs`. Folgedatensatz für den Spec: `issues/260815-0233_o_das-zweite-bild-des-spec-zeigt-den-abstieg-mit-rueckkehr-der-baum-merkt-pfade-vor.md`.
