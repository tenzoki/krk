# Sechs Defekte an Messstrecke und Bauwerkzeug

**Status:** Complete
**Agent:** coder
**Datum:** 260806-1305

## Auftrag

Sechs offene Defekte rund um die Messstrecke und das Bauwerkzeug beheben,
beginnend mit dem Abbruch des Sitzungslaufs bei L5-Tab.

## Der L5-Tab-Abbruch: kein Commit, sondern der Vordergrund

Der Defekt `260806-1235_o_…l5-tab…` vermutete einen der vier Commits seit S22.
Das ist nachgemessen und **widerlegt**. Der Abbruch tritt mit dem Stand von
S22 (`e8626b6`) genauso auf:

| Arbeitsbaum | Ergebnis |
|---|---|
| HEAD (`fd5e3c5`) | Abbruch bei l5-tab nach 10 s |
| `crates/krk-ui/src` auf `194ea16` zurückgenommen | derselbe Abbruch |
| `crates/krk-ui/src` auf `e8626b6` (S22) zurückgenommen | derselbe Abbruch |
| zusätzlich `xtask/src` auf `e8626b6` zurückgenommen | derselbe Abbruch |

Zurückgenommen wurde jeweils über `git checkout <ref> -- <pfade>`; der
Arbeitsbaum steht danach wieder auf HEAD, kein Branch wurde gewechselt und
nichts committet.

Die Ursache steckt nicht im Code, sondern in der Bedingung, unter der der Lauf
startet. Eine Sonde im `Anweisung::Funktionstaste`-Zweig von
`Anwendungsdelegierter::messen_weiter` gab je Tastendruck den Fokus aus:

```
PROBE kennung=auswahl_runter fokus=Anderswo keyfenster=false
PROBE kennung=tab_naechster  fokus=Anderswo keyfenster=false
```

`Anwendungsdelegierter::fokus` liefert `Fokus::Anderswo`, sobald das
Schlüsselfenster nicht das Hauptfenster ist, und `kommando_ausfuehren` weist
dann jeden Befehl mit einem Wirkungsbereich ab. Übrig bleibt
`auswahl_runter` mit `Wirkungsbereich::Ueberall` — genau die Taste, mit der L1
und L7 gemessen werden. `tab_naechster` liegt im Wirkungsbereich
`Tabbereich` und löst nichts mehr aus; die Messung wartet zehn Sekunden und
bricht ab. Die Reihenfolge der Strecke erklärt damit vollständig, warum der
Abbruch immer bei L5-Tab liegt.

Warum das Fenster nicht Schlüsselfenster wird: macOS 15 lässt eine Anwendung,
die aus einem Prozess im Hintergrund gestartet wurde, nicht nach vorn. Beide
Wege dagegen sind gemessen und wirkungslos: `NSApplication::activate()` und
`activateIgnoringOtherApps(true)` ließen `isActive()` auf `false`. Aus einem
Terminalfenster im Vordergrund gestartet, lief derselbe Bau durch:
`fokus=Dateifenster key=true`, sieben Reihen mit je zwanzig Werten, Rückgabe 0.

**Behoben ist die falsche Diagnose, nicht die Bedingung.** Die
Sitzungsstrecke verweigert die erste Messung jetzt mit
`messmodus::NICHT_IM_VORDERGRUND`, sobald KRK nicht vorn steht — dieselbe
Haltung wie `OHNE_BILDSCHIRM`. Die Frage, wie der Abnahmelauf künftig
sicherstellt, dass KRK nach vorn darf, liegt als Entscheidungsdatensatz
`decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`
beim Nutzer.

## Die übrigen fünf

- **Session-toml-Verlust** (`260806-0834_o_…session-toml…`):
  `Gesamtlauf::fahren` legt vor der ersten Runde eine `Sitzungssicherung` an,
  die den vorigen Stand in `Drop` zurückspielt — auch wenn eine Runde mit `?`
  abbricht. Gab es keine Sitzung, wird die Prüfsitzung wieder entfernt. Zwei
  Prüfungen decken beide Fälle ab.
- **Prüfordner unter /tmp** (`260806-0014_o_…`): das `Makefile` führt
  `MESSPLATZ := $(HOME)/Library/Caches/krk-messplatz` und leitet die vier
  Pfade daraus ab; der Kommentar trägt den Befund. `README.md` nannte `/tmp`
  nicht und blieb unberührt.
- **AppKit-Grenzprüfung** (`260806-0834_o_…pub-use…`): `ist_objc2_use` liest
  jetzt ein Sichtbarkeitspräfix (`pub`, `pub(crate)`, `pub(in …)`) und ein
  führendes `::` mit; `pubuse` und `public_use` fallen weiter durch. Sieben
  neue Behauptungen in zwei Prüfungen. **Offen bleibt die Planpflege:** das
  Abnahmekriterium von S23 nennt weiter das alte Grep und wurde nicht
  angefasst (Plan ist Workbench-Datei).
- **Binärname als Literal** (`260806-0834_o_…binaernamen…`): `bundle::bauen`
  liefert `Gebaut { buendel, binaer }`; der Binärpfad entsteht in
  `Vorlage::binaer_im_buendel` aus `CFBundleExecutable`, und `xtask messen`
  reicht ihn durch statt `Contents/MacOS/krk` zu bilden.
- **Menü-Protokoll** (`260805-0841_o_…zweitformen…`): nachgemessen mit einer
  Sonde, die `terminate:` vorübergehend wieder eintrug. Die Zweitform "Quit
  and Keep Windows" erscheint an **keinem** Auslesezeitpunkt der Marke: nicht
  nach `finishLaunching`, nicht nach 0,5 s und nicht nach 2 s Ereignisschleife,
  auch nicht nach `activate` — die Marke öffnet kein Fenster, und ohne Fenster
  wird die Anwendung nicht aktiv (`isActive() == false` in allen sechs
  Messungen). Es gilt damit die zweite Antwort des Defekts: der Kopf von
  `protokollieren` hält jetzt fest, was die Marke nicht sieht und wo die
  zweite Hälfte des C3-Kriteriums zu prüfen ist.

## Nachweis

`make check` grün. `make alle RUNDEN=1`, aus einem Terminalfenster im
Vordergrund gefahren, liefert für alle zwölf Berichtszeilen Zahlen; die sechs
Größen der Sitzungsstrecke stehen wieder: L1 100 % im Bild, L5-Tab p95
37,1 ms, L5-Fenster 14,4 ms, L6 47,1 ms, L7 34,2 ms, L8 168,0 ms. Verfehlt ist
allein L9 mit 80 % statt 95 % im Bild — der bekannte offene Datensatz
`decisions/260806-0014_o_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`,
kein Abbruch. Die `session.toml` des Nutzers war nach dem Lauf byteweise
unverändert.

## Neu gefiled

- `issues/260806-1304_o_der-sitzungslauf-blieb-einmal-von-drei-malen-bei-l6-stehen.md`
- `decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`

## Geänderte Dateien

`Makefile`, `crates/krk-bench/src/messen.rs`,
`crates/krk-ui/src/messmodus.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/menue.rs`, `xtask/src/bundle.rs`,
`xtask/src/main.rs`, `xtask/src/messen.rs`, `xtask/src/release.rs`.

Nicht committet, wie beauftragt.
