# coder — Schreibwerkzeuge abgewählt, Blatt-Spanne nachgefahren, Planordner zusammengelegt

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Datum:** 260810-1520

Drei Aufgaben, vom Nutzer über den Orchestrator gestellt: ein Verhaltensfehler zum
Messen, eine getroffene Entscheidung zum Einlösen, eine Aufräumung.

## 1. Die Blatt-Spanne — die Sorge trägt nicht

Datensatz `issues/260810-1207_*_die-spanne-zwischen-dem-schliessen-des-blattes-und-seiner-antwort-ist-ungemessen.md`.

Der Datensatz war **schon gemessen und getragen** vorgefunden, und `spikes/blatt-spanne/`
lag im Baum (unversioniert). Statt der Datei zu glauben, beide Arme neu gebaut und ein
zweites Mal gefahren. Der Befund reproduziert: `attachedSheet` steht im Abschlussblock
noch gesetzt und fällt erst 269 bis 272 ms danach auf `nil`. Die vermutete Spanne ist
die **Umkehrung** der tatsächlichen Reihenfolge; die Sperre aus `blatt_steht` greift bis
zur ausgeführten Antwort einschließlich. Kein Takt und kein Tastendruck fiel hinein.

**Kein Code geändert.** `anlass_unterbleibt` bleibt, wie es ist. Der Datensatz trägt den
nachgefahrenen Lauf als Zusatz.

## 2. Die Schreibwerkzeuge — ausgeschlossen

Datensatz `decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`, Antwort
Option 1. `Answered:` und `Implemented:` eingetragen.

Die Umsetzung fällt **anders aus, als Option 1 sie beschrieb**, und das ist der Kern
dieser Aufgabe. Vor dem Schreiben an einer echten `NSTextView` gemessen, was die vier
Setzer hergeben:

| Einstellung | Werkswert | Aus-Wert |
|---|---|---|
| `writingToolsBehavior` | `Default` (0) | `None` (−1) |
| `allowsWritingToolsAffordance` | an (1) | 0 |
| `allowedWritingToolsResultOptions` | 0 | **keiner** |
| `writingToolsAllowedInputOptions` | 0 | **keiner** |

Die beiden letzten sind Bitmasken, deren Null `…ResultDefault` heißt — „das System
wählt" — und nicht „nichts". Option 1 verlangte „die drei übrigen auf den Wert, der
nichts zulässt"; **diesen Wert führt Apple für zwei von ihnen nicht**, und beide stehen
ab Werk schon auf Null. Eine Zeile wäre ein Aufruf ohne Wirkung gewesen, und
`Abgeschaltet` hätte die Probe rot gefärbt, weil ihre zweite Hälfte einen Unterschied
zur frischen Fläche verlangt.

Also: zwei Zeilen und eine fünfte Antwort. `Einordnung::Gegenstandslos(traeger)` sagt,
dass eine Einstellung beschreibt, **was** eine Fähigkeit dürfte, die abgeschaltet ist.
Mitgemessen und für die Abgrenzung tragend: `setWritingToolsBehavior(None)` lässt die
beiden Bitmasken **unberührt**, sie sind also keine zweiten Türen.

Die Untergrenze ist gewahrt. `setWritingToolsBehavior:` steht seit macOS 15.0 und wird
unmittelbar gerufen; `setAllowsWritingToolsAffordance:` erst ab 15.4 und nur an
`NSTextField`, sie geht über die neue Funktion `setzen_falls_vorhanden`, die
`respondsToSelector:` vorher fragt. Der `performSelector:withObject:`-Weg wurde
verworfen — er übergibt einen Objektzeiger, wo der Setzer ein `BOOL` erwartet, und ein
Zeiger ist nie `NO`; gesetzt wird deshalb über die Schlüsselwertkodierung, wie es die
lesende Seite schon tut. `setzername` ist dafür aus `mod tests` in den Modulrumpf
gewandert, damit beide Seiten **denselben** Namen bilden.

Belegt, dass der gehütete Setzer wirklich setzt und nicht still übersprungen wird: der
Lauf mit `--nocapture` gibt keinen Hinweis aus, die Zusicherung wurde also ausgewertet.

Nachgezogen: `aus_bedeutet` um `Behavior:` und `Affordance:` (`Options:` bleibt mit
Absicht im Abbruchzweig), `jede_tuer_…` → `jeder_verweis_zeigt_auf_beantwortete_einstellungen`,
`die_sieben_…` → `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` mit neun
statt sieben Einstellungen und Hinweis statt Abbruch bei einer fehlenden,
`der_vorgabewert_…` misst jetzt die frische Fläche statt KRKs, neu
`die_gegenstandslosen_stehen_unberuehrt_und_ihr_traeger_steht_aus`. Der Modulkopf ist
auf neun umgeschrieben und trägt einen eigenen Abschnitt zu den Schreibwerkzeugen.

Der Übersetzer hat den Umbau wie vorgesehen angehalten: die neue Variante fiel in
`ziele_von` als nicht abgedeckt auf. Sie ist dort eingehängt, was die Prüfung „der
genannte Träger trägt selbst eine Antwort" für alle drei verweisenden Antworten in
einem Zug erledigt, statt sie zweimal zu schreiben.

## 3. Der Planordner — zusammengelegt

Datensatz `issues/260810-1430_*_planordner-in-messmodus-ist-die-dreizehnte-fassung-…`,
`Resolved:` eingetragen.

`Planordner` hält jetzt einen `Pruefordner`. Weggefallen: eigener `AtomicU64`, eigener
`Drop`, eigener Namensbau. Geblieben: die vier Ordnernamen und der `Messplan` — die
Fachlogik, die der Datensatz ausdrücklich draußen halten will. `Pruefordner::ordner(name)`
ist dazugekommen, wie vorhergesagt. Über den vorgezeichneten Umbau hinaus zwei Zugänge
`Planordner::unter` und `::pfad`, damit die rund zwanzig Probenstellen nicht weiter durch
das Feld hindurchgreifen.

## Abnahme

Am ganzen Arbeitsbereich, nach allen drei Aufgaben:

```
cargo build --workspace                     exit 0
cargo test --workspace                      exit 0   (16 Probenprogramme, 334 in krk-ui)
cargo clippy --workspace --all-targets      exit 0
cargo fmt --all --check                     exit 0
```

## Was offen bleibt

- Der Abnahmelauf am laufenden Bündel ist Nutzerarbeit und dadurch nicht berührt. Dass
  die Schreibwerkzeuge im Kontextmenü **nicht mehr erscheinen**, ist am Bündel zu
  sehen; die Proben sagen nur, dass die Einstellungen stehen und greifen.
- `Einordnung::NochOffen` steht heute leer und trägt `#[allow(dead_code)]`. Absichtlich
  behalten, weil die nächste Lesart wieder eine braucht — aber es ist eine Variante
  ohne Fall, und wer sie beim nächsten Aufräumen streichen will, hat einen Punkt.
- Die Marker der drei Datensätze benennt der Nutzer um, nicht dieser Agent.
