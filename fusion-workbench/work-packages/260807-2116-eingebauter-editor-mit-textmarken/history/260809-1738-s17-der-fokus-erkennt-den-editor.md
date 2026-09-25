# S17: Der Fokus erkennt den Editor

**Status:** Complete
**Agent:** coder
**Datum:** 260809-1738
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 17

## Was umgesetzt ist

`Anwendungsdelegierter::fokus` liefert `Fokus::Editor`, sobald der Ersthelfer
des Fensters die Textfläche des Editors ist. Gebaut ist dafür nicht der im
Befund vorgeschlagene vierte `if`, sondern die Ursache darunter.

Neu ist `Anwendungsdelegierter::fokusansicht(ziel) -> Option<&NSResponder>`:
eine erschöpfende Fallunterscheidung über `Fokus`, die jedem Wert die eine
Ansicht zuordnet, die seinen Ersthelferrang trägt. Sie bedient beide
Richtungen. `fokus()` läuft `Fokus::ALLE` durch und hält den Ersthelfer gegen
jede dieser Ansichten; `fokus_setzen()` geht dieselbe Zuordnung rückwärts und
macht die genannte Ansicht zum Ersthelfer. Ein sechster Fokuswert hält damit
den Bau an — genau die Erzwingung, die der `if`-Kette fehlte und deren Fehlen
den Editor bis zum 260809 stumm zum Dateifenster gemacht hat.

Als Nebenwirkung derselben Zusammenlegung fallen die drei handgeschriebenen
Sichtbarkeitsabfragen in `fokus_setzen` weg. Welcher Bereich zu einem Fokuswert
gehört, sagt `fokus::holt_hervor`, das diese Zuordnung für das Hervorholen
schon trug; das aktive Dateifenster liefert dort `None` und ist nie
ausgeblendet.

`Fokus::ALLE` ist neu in `crates/krk-ui/src/kommandos/fokus.rs`, mit der
Feldbreite `[Fokus; 5]` in der Typangabe. Die Testkonstante `JEDER_FOKUS`, die
dieselben fünf Werte ein zweites Mal aufzählte, verweist jetzt darauf.

Der Zeichenzweig in `eingabe_ausfuehren` fragt jetzt den Fokus, in einer
vollständigen Fallunterscheidung ohne Auffangzweig. Allein `Fokus::Dateifenster`
geht an die Sprungmarke aus C2; Leiste, Vorschau, Editor und `Anderswo` liefern
`false`, und nur so läuft der Tastendruck unverändert an AppKit weiter und wird
in der Textfläche zu einem Zeichen.

Drei Doc-Kommentare, die durch S4 unwahr geworden waren, sind ersetzt: der über
`fokus()` ("Die Schreibmarke in einem Textfeld kommt hier nicht vor"), der über
`eingabe_ausfuehren` ("ein getipptes Zeichen gehört immer dem aktiven
Dateifenster") und der Modulkopf-Abschnitt zum Fokusvorbehalt, der zwei
fokussierbare Bereiche nannte.

## Was bewusst nicht gebaut ist

`bereichskommando` behält `Fokus::Editor => false`. Die Änderungszeile von S17
sagt, der Zweig reiche das Kommando an den Editor; das trifft die gebaute
Architektur nicht. Die neun Befehle mit `Wirkungsbereich::Editor` holen sich
jeweils einen eigenen Zweig in `kommando_ausfuehren` (S20, S22, S23, S25, S32,
S34 und die folgenden), so wie die Fokusbefehle es tun; über `bereichskommando`
läuft keiner von ihnen. Der Zweig ist seit diesem Schritt erreichbar — das war
er vorher nicht — und `false` ist die dauerhaft richtige Antwort. Der Kommentar
sagt das jetzt, statt einen ablösenden Schritt zu nennen, der nie kommt. Der
Vermerk steht im Plan unter S17.

`crates/krk-ui/src/appkit/editor.rs` ist unberührt geblieben (reserviert für
parallel laufende Schritte). Die Fokusansicht, die die Dateizeile von S17 dort
vorsah, hat S16 als `Editorbereich::textflaeche` bereits gebaut.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/anwendung.rs` — `fokusansicht` (neu), `fokus`,
  `fokus_setzen`, `eingabe_ausfuehren`, `bereichskommando`, Modulkopf
- `crates/krk-ui/src/kommandos/fokus.rs` — `Fokus::ALLE` (neu), zwei Proben,
  `JEDER_FOKUS` auf `Fokus::ALLE` zurückgeführt

## Abnahme

Alle vier Abnahmekommandos beenden mit 0: `cargo build --workspace`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets`,
`cargo fmt --all --check`.

Zwei neue Proben in `crates/krk-ui/src/kommandos/fokus.rs`:

- `im_editor_wirkt_kein_befehl_des_dateifensters_und_jeder_des_fensters` — das
  Abnahmekriterium von S17. Erster Durchgang über die Befehle, die es
  namentlich führt (Dateioperationen aus C4, Ordnernavigation aus C2, die
  beiden Zwischenablage-Befehle aus C10), jeweils mit Gegenprobe im
  Dateifenster. Zweiter Durchgang über `Kommando::KENNUNGEN` statt über eine
  Liste, mit einer erschöpfenden Fallunterscheidung über `Wirkungsbereich`, so
  dass ein später hinzukommender Befehl mit abgedeckt ist.
- `die_aufzaehlung_der_fokuswerte_ist_vollstaendig_und_doppelt_keinen` — die
  Hälfte, die die Feldbreite `[Fokus; 5]` nicht abdeckt.

**Anmerkung zum Prüfweg.** Während der Arbeit stand
`crates/krk-core/src/tasten/parser.rs` durch einen parallel laufenden Schritt
zeitweise in einem nicht übersetzbaren Zwischenstand. Die vier Kommandos sind
deshalb zuerst auf einer Kopie gefahren worden, die aus `HEAD` plus den beiden
oben genannten Dateien bestand, und danach ein zweites Mal im
Arbeitsverzeichnis, nachdem der parallele Stand wieder übersetzte. Beide Läufe
sind grün.

## Nutzerarbeit

Kein Agent kann diese drei Punkte abnehmen; sie brauchen das laufende Bündel im
Vordergrund. Mit `[sichtbarkeit] editor = true` in `session.toml` ist die
Textfläche erreichbar, bis S18 und S20 die gewöhnlichen Wege bauen.

1. `--tasten-protokoll`: mit der Schreibmarke im Editor führt ein Druck auf
   `up` kein Kommando aus, und die Schreibmarke bewegt sich.
2. Ein Buchstabe landet im Text und nicht im Suchpuffer der Dateiliste.
3. `delete` und `f5` tun mit der Schreibmarke im Editor nichts.

Offen und nicht von diesem Schritt verursacht:
`issues/260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`
— ob eine Unteransicht der Vorschau oder der Leiste den Ersthelferrang an sich
zieht, ist am laufenden Bündel zu messen.

## Geschlossene Befunde

- `issues/260809-1640_c_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`
- `issues/260809-1648_c_die-sprungmarke-geht-ohne-fokuspruefung-in-das-aktive-dateifenster.md`

## Neuer Befund

- `issues/260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`
