# Der Start aktiviert die Anwendung

**Status:** Complete
**Agent:** coder
**Datum:** 260807-1628
**Auftrag:** dringende Einzelaufgabe, kein Circle aktiv
**Defekt:** `shared/issues/260807-1624_c_der-start-holt-das-fenster-nach-vorn-aber-aktiviert-die-anwendung-nicht.md`

## Was geaendert wurde

`crates/krk-ui/src/appkit/anwendung.rs`, eine Aenderung an zwei Stellen:

- Zeile 634: `oberflaeche_aufbauen` ruft `self.fenster_zeigen()` statt des
  bisherigen `if let Some(fenster) = ivars.fenster.get() { fenster.makeKeyAndOrderFront(None); }`.
  Der Kommentar darueber sagt, warum der Start aktiviert: ueber den Finder oder
  `open` gestartet nimmt LaunchServices die Aktivierung ab, als Kindprozess eines
  Terminals niemand, und die Sitzungsstrecke aus S21 misst nur, wenn KRK vorn
  steht.
- Der Kommentar zur Fokusordnung nennt jetzt `fenster_zeigen` als die Funktion,
  in der `makeKeyAndOrderFront` steckt, und haelt fest, dass `activate()` den
  ersten Beantworter nicht anruehrt.
- Der Doc-Kommentar von `fenster_zeigen` (`:1723`, die Funktion selbst `:1733`)
  nennt vier Wege statt drei
  und benennt die beiden Haelften.

Nicht angefasst: `crates/krk-bench/`, `crates/krk-ui/src/messmodus.rs`, Spec,
Plan, Entscheidungsdatensaetze, `.toml`.

## Die drei Pruefungen

1. **Die Fokusordnung haelt.** `fokus_setzen(fokus::BEIM_START)` steht weiterhin
   nach dem Aufruf, der `makeKeyAndOrderFront` ausfuehrt. `fenster_zeigen` ruft
   `makeKeyAndOrderFront` und danach `activate()`, beides synchron, beides vor
   `fokus_setzen`.
2. **Der Zugriff auf `ivars.fenster` traegt.** `oberflaeche_aufbauen` bindet
   `ivars` in Zeile 536 aus `self.ivars()`; `fenster_zeigen` liest dieselbe
   `OnceCell` ueber `self.ivars().fenster.get()`. Gesetzt wird sie in Zeile 569,
   65 Zeilen vor dem Aufruf. Beide Zugriffe sind geteilte Referenzen, der
   Aufruf mit lebendigem `ivars` ist zulaessig (die Zeilen 620 bis 624 tun
   dasselbe schon).
3. **`fenster_zeigen` tut nicht mehr als gewollt.** Der Rumpf ist die
   Wachtklausel auf `ivars.fenster`, `makeKeyAndOrderFront(None)` und
   `NSApplication::sharedApplication(self.mtm()).activate()`. Genau die zwei
   Aufrufe, die der Start braucht, plus dieselbe Wachtklausel, die der alte
   `if let`-Block hatte.

## Abnahme

- `make check` gruen: Bau, Tests, `clippy -D warnings`, `fmt --check`.
- `make bundle` gruen, signiert mit "Apple Development: Kai Stalmann
  (FJ8U4B3QAC)", `target/KRK.app` steht.
- Der Abnahmelauf wurde **nicht** gefahren. Er verlangt KRK im Vordergrund; ein
  Hintergrundprozess erzeugte genau den Fehler, den die Aenderung behebt.
- Kein Test angelegt. `oberflaeche_aufbauen` und `fenster_zeigen` brauchen eine
  laufende `NSApplication` und ein Fenster; `anwendung.rs` traegt aus demselben
  Grund kein Testmodul, und die Kiste kennt kein Muster, das Quelltext als Text
  prueft (die drei `include_str!`-Stellen betten Ressourcen ein).

## Offen

Der Beleg fehlt: `inference:`, dass `activate()` aus dem Kindprozess eines im
Vordergrund stehenden Terminals wirkt. Der naechste Abnahmelauf des Nutzers
entscheidet es.

Nicht committet, wie beauftragt.
