# Bugfix: Das Wegschalten einer Spalte verteilt die Breiten nicht neu

**Datum:** 2026-08-12 09:25
**Status:** Complete
**Auslöser:** Nutzerauftrag mit dem Defektdatensatz
`shared/issues/260812-0907_*_das-wegschalten-einer-spalte-verteilt-die-breiten-nicht-neu-und-schiebt-datum-aus-dem-bild.md`
**Schreibziel dieses Protokolls:** `shared/history/`, weil kein Circle aktiv ist.

## Fehler

Der Nutzer hat am laufenden Bündel gesehen: schaltet er über die Bereichsleiste die Spalten Größe
und Typ weg, bekommt die Namensspalte zu viel Platz, und die Spalte Datum steht trotz des frei
gewordenen Raums außerhalb des sichtbaren Bereichs.

## Messung vor der Änderung

KRK selbst lässt sich nicht starten: der Abnahmelauf verlangt das Programm im Vordergrund und ist
Nutzerarbeit. Gemessen wurde deshalb an einer `NSTableView` **ohne Fenster**, gebaut mit denselben
vier Breitenpaaren und derselben Betriebsart wie `Dateifenster::bauen`. Das Messprogramm lief
außerhalb des Projektbaums (eigene Kiste gegen `objc2-app-kit` 0.3.2 im Ablagebereich der
Sitzung); im Baum bleibt davon nichts stehen, weil `krk-ui` kein Bibliotheksziel hat und eine
Probe mit `MainThreadMarker::new_unchecked` die als Lage angenommene Bauart vermehrt hätte.

**Vermutung 1 hält.** `setHidden:` hält die Gesamtbreite der Tabelle fest und schlägt die frei
werdenden Punkte samt einem Zellenabstand von 17 Punkten allein der Namensspalte zu. Bei 700
Punkten Sichtfläche: Name 337 → 434 (Größe weg) → 541 (Typ zusätzlich weg). Keine andere Spalte
wird angefasst, kein Punkt geht zurück an die Tabelle.

**Vermutung 2 hält in ihrer Formulierung nicht.** Die Gesamtbreite bleibt nicht „größer als die
Sichtfläche", sie ändert sich überhaupt nicht. Vier Spalten in ihrer natürlichen Breite brauchen
603 Punkte; ein Dateifenster, das schmaler ist — zwei nebeneinander in einem gewöhnlichen Fenster
sind es —, zeigt die Tabelle schon vor jedem Schalten mit Überstand. Das Wegschalten von Größe und
Typ macht 204 Punkte frei, die genau diesen Überstand auflösen würden; sie gehen an Name, die
Tabelle bleibt 603 breit, und Datum steht weiter außerhalb. Beide Teile der Meldung stammen aus
diesem einen Mechanismus.

**Was AppKit von sich aus richtig macht.** Ändert sich die Sichtfläche, trifft
`FirstColumnOnlyAutoresizingStyle` die vom Nutzer gewählte Regel schon heute: 900 Punkte → Name
537, 500 → 137, 400 → Mindestbreite 100, die drei schmalen unangetastet. Defekt ist allein der Weg
über `setHidden:`.

## Ursache

`crates/krk-ui/src/appkit/anwendung.rs`, `spaltenanzeige_nachziehen`: der Nachzug schrieb allein
die Sichtbarkeit und ließ die Breiten AppKit. AppKit erhält beim Verbergen die Gesamtbreite der
Tabelle und gibt die frei werdenden Punkte der ersten Spalte — das ist die Betriebsart aus
`tabelle.rs:2237` und kein Fehler von AppKit, aber es ist nicht die Regel, die der Nutzer will,
und es gibt den gewonnenen Platz nie an die Sichtfläche zurück.

## Fix

Die Regel steht im Nutzerentscheid vom 260812-0910, Möglichkeit 1: die drei schmalen Spalten
stehen bei jeder Schalterstellung auf ihrer natürlichen Breite, Name nimmt den Rest, begrenzt
durch seine Mindestbreite. Die Betriebsart bleibt unverändert.

| Datei | Änderung |
|---|---|
| `crates/krk-ui/src/appkit/tabelle.rs:2441` | Neu: `Dateifenster::spaltenbreiten_verteilen` — drei Durchgänge (sichtbare Spalten auf natürliche Breite, rechten Rand über `rectOfColumn:` messen, Namensspalte festlegen). |
| `crates/krk-ui/src/appkit/tabelle.rs:2487` | Neu: die reine Funktion `namensbreite` mit `#[must_use]`, ohne AppKit und damit ohne Fenster prüfbar. |
| `crates/krk-ui/src/appkit/tabelle.rs:2527` | Zwei Proben auf `namensbreite`: die Verteilung und der Vorrang der Mindestbreite. |
| `crates/krk-ui/src/appkit/tabelle.rs:2373` | Der Kopf von `spalte_verbergen` sagt jetzt, dass die Breiten dort ungerechnet bleiben. |
| `crates/krk-ui/src/appkit/tabelle.rs:108` | Der Modulkopf nennt die drei neu angesprochenen Abfragen mit ihrer Untergrenze: `rectOfColumn:` (`NSTableView.h:393`), `columnWithIdentifier:` (`:238`), `width`/`minWidth` (`NSTableColumn.h:42`, `:48`) — alle ohne `API_AVAILABLE`, also seit 10.0. |
| `crates/krk-ui/src/appkit/anwendung.rs:2495` | `spaltenanzeige_nachziehen` ruft die Verteilung einmal je Dateifenster, nach dem Durchgang über die vier Spalten. Damit erreicht sie beide Dateifenster und beide Anlässe (Aufbau und Schalter). |

**Warum der rechte Rand gemessen und nicht gerechnet wird.** Zwischen der Summe der
Spaltenbreiten und der Breite der Tabelle liegen der Zellenabstand je Spalte und die
Randpolsterung des `NSTableViewStyle` (`NSTableView.h:81`); beide sind nirgends zugesagt, und
gemessen liegen sie bei 17 Punkten je Spalte und knapp minus 5 dazu. `rectOfColumn:` trägt beides
fertig, eine verborgene Spalte liefert dabei ein leeres Feld.

**Warum die Namensspalte zuletzt kommt.** Ihr Zuwachs verschiebt den Rand, an dem er gemessen
wird. Deshalb erst alle sichtbaren Spalten auf ihre natürliche Breite, dann messen, dann Name.

## Prüfung

- [x] Der gemeldete Fehler ist weg — gegengeprobt am kopflosen Aufbau mit demselben Verfahren wie
      die Messung: bei 900, 700, 500 und 400 Punkten Sichtfläche und in jeder durchlaufenen
      Schalterstellung sitzt der rechte Rand der letzten sichtbaren Spalte danach genau auf der
      Sichtfläche. Die eine Ausnahme ist die Sichtfläche unter rund 463 Punkten, in die vier
      Spalten auch mit Name auf seiner Mindestbreite nicht passen; dort bleibt der waagerechte
      Schieber, und so steht es im Entscheid.
- [x] `make check` — Exit 0 (Bau, 16 Probenläufe, `fmt --check`, `clippy -D warnings`).
- [x] `make bundle` — Exit 0, `target/KRK.app` gebaut und signiert.
- [ ] **Am laufenden Bündel ungesehen.** Der Abnahmelauf verlangt KRK im Vordergrund und ist
      Nutzerarbeit; kein Agent kann ihn fahren.

## Nebenwirkung, benannt statt verschwiegen

Die Verteilung läuft auch beim Aufbau, weil `spaltenanzeige_nachziehen` dort schon gerufen wird.
Ein Dateifenster, das schmaler ist als 603 Punkte, zeigt seine vier Spalten deshalb ab jetzt schon
beim Start ohne Überstand: Name steht dann unter seiner Anfangsbreite von 240. Das ist dieselbe
Regel, die AppKit bei jeder Größenänderung des Fensters ohnehin anwendet, und keine zweite.

## Weitere gefundene Defekte oder Fragen

Keine.
