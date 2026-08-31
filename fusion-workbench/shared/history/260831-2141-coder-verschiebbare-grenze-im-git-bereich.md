# Coder — die Grenze im Git-Bereich wird verschiebbar — 260831-2141

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Auftrag:** Die Flächenaufteilung des Git-Bereichs ändern: die Einzelheiten bekommen die
Hälfte der Höhe unter dem Kopf, und die Grenze zwischen Verlaufsliste und Einzelheiten
wird mit der Maus verschiebbar. Nutzerbefund aus dem laufenden Programm, gemeldet nach
der Auslieferung.
**Herkunft:** außerhalb einer laufenden Runde — die Runde 23 ist beschränkt geschlossen —,
deshalb liegt dieser Datensatz im gemeinsamen Speicher.
**Status:** Complete

## Ergebnis in einem Satz

Eine waagerechte `NSSplitView` unter dem Kopf teilt die Fläche ab Werk hälftig zwischen
Verlaufsliste und Einzelheiten, die Trennlinie lässt sich ziehen, beide Flächen tragen
eine begründete Mindesthöhe, und die Teilung übersteht das Beenden als Anteil in
`session.toml`.

## Wie die Aufteilung gebaut ist

```text
Gitsicht (Trägeransicht)
├── Kopf (NSTextField, feste KOPFHOEHE, oben angeschlagen)
└── Teiler (NSSplitView, waagerecht, füllt den Rest)
    ├── Verlaufsliste  (NSScrollView + NSTableView)
    ╞══ Trennlinie 0 ══
    └── Einzelheiten   (NSScrollView + Einzelheitenflaeche + NSTextField)
```

Der Kopf behält seine feste Höhe und bleibt eine Unteransicht der Trägeransicht. Alles
darunter gehört dem Teiler, und der ordnet seine zwei Unteransichten von oben nach unten
(`NSSplitView.h:71`). Das Verhältnis hält AppKit über jede Größenänderung von selbst
(`NSSplitView.h:63`), es gibt also keine zweite Rechenvorschrift neben der der Fensterzeile.

`Gitfenster` bedient jetzt eine dritte Delegiertenrolle: neben `NSTableViewDataSource` und
`NSTableViewDelegate` auch `NSSplitViewDelegate`, mit den zwei Grenzmethoden. Der Grund ist
derselbe wie bei den beiden anderen: was sie beantworten, sind die zwei Mindesthöhen dieses
Bereichs, und ein eigenes Objekt dafür wäre ein zweiter Halter derselben Auskunft.

Drei reine Funktionen tragen die Rechnung und sind ohne Fenster prüfbar:
`trennlinienspanne` (beide Grenzen zusammen, nie mit vertauschten Enden), `trennlinienlage`
(Anteil → Lage, gedeckelt auf die Spanne) und `anteil` (Lage → Anteil, `None` vor dem
ersten Auslegen).

## Die Mindesthöhen

| Fläche | Wert | Begründung |
|---|---|---|
| Verlaufsliste | `3.0 * ZEILENHOEHE` = 60 | In dieser Liste bewegt der Nutzer eine Auswahl mit `AuswahlHoch` und `AuswahlRunter`. Unter drei Zeilen sieht er den Eintrag über und unter dem ausgewählten nicht mehr zugleich, und die Bewegung verliert ihren Bezugspunkt. |
| Einzelheiten | `5.0 * EINZELHEITENZEILE + 2.0 * RAND` = 82 | Die vier Angaben aus E13 in ihrer kürzesten Gestalt: eine Zeile Nachricht, die Leerzeile, Autor mit Anschrift, Datum, Objektname. Wer weniger sieht, sieht von den vier Angaben nicht mehr alle. |

Beide sind hergeleitet und nicht gesetzt, und beide gehen durch `trennlinienspanne`, die
zusichert, dass die kleinste Lage nie über der größten liegt — auch nicht in einem Bereich,
der zu niedrig für beide wäre. Die Mindesthöhe des Fensters lässt diesen Fall nicht zu
(`appkit::fenster::MINDESTGROESSE`, Fensterzeile mindestens 300 Punkte gegen 60 + 82 +
Trennerdicke).

Das ist die Form von `Bereich::mindestbreite`, eine Achse weiter: eine Zahl je Fläche, an
einer Stelle, mit ihrer Begründung daneben.

## Die Grenzposition übersteht das Beenden

Neues Feld `Sitzung::gitanteil: Option<f64>` in `crates/krk-core/src/ablage/sitzung.rs`,
vor den drei Tabellen (TOML verlangt Werte vor Untertabellen).

**Ein Anteil und keine Punktzahl**, anders als bei `Breiten` daneben. Das ist kein Bruch
mit der Währung der Datei, sondern eine andere Größe: die Breiten teilen die Fensterzeile,
deren Zahl von außen kommt; hier wird eine Fläche geteilt, deren Höhe mit jeder
Fenstergröße eine andere ist. Eine Punktzahl gälte nur für die Höhe, bei der sie entstanden
ist. Nebeneffekt, der die Sache trägt: die Vorgabe (hälftig) und ein gespeicherter Wert
sind dieselbe Größe und gehen denselben Weg — es gibt keine zweite Stelle, an der dieser
Bereich geteilt wird.

Der Weg hin: `Gitfenster::listenanteil` **misst** die zwei Rahmen, statt einen Wert zu
halten — dieselbe Wahl wie `Aufteilung::gemessene_breiten` für die Fensterzeile.
`Anwendungsdelegierter::gitanteil` reicht ihn an `Fenstermodell::sitzung` weiter, das dafür
einen vierten Parameter bekommt (derselbe Zuschnitt wie `editor` und `zettel`, die aus
demselben Grund von außen kommen).

Der Weg zurück: `Gitfenster::listenanteil_setzen` **merkt vor** und setzt nicht. Angewandt
wird der Anteil beim ersten Auslegen der Trägeransicht, über `Gitsicht`. Der Grund ist der
Kernpunkt und steht an `GitfensterIvars::offener_anteil`: beim Aufbau trägt der Bereich
seine Aufbaugröße, und ein Anteil, der an ihr gegen eine Mindesthöhe liefe, verlöre genau
dort seinen Wert — bevor das Fenster seine wirkliche Höhe hat. Solange die Vormerkung
steht, geht sie auch der Messung vor: sonst schriebe der Zwei-Sekunden-Takt die gespeicherte
Teilung mit der Aufbauteilung zu, bevor sie je zu sehen war.

**Was das gekostet hat:** fünf Dateien, rund 120 Zeilen samt Dokumentation und Proben. Das
ist weniger als der Rest der Aufgabe (Teiler, Mindesthöhen, Modulkopf, Untergrenzen-
Abschnitt, Proben) und lag damit unter der Abbruchschwelle des Auftrags.

## Der Modulkopf ist umgeschrieben und nicht ergänzt

Die alte Festlegung — „ohne zweite `NSSplitView`: der Nutzer soll sie nicht gegeneinander
verschieben, und ein Schieberegler im Bereich wäre ein Bedienelement, das der Spec nicht
verlangt" — steht nicht mehr als geltende Begründung da. An ihrer Stelle steht, was jetzt
gilt, und darunter, dass die Annahme aus dem Spec der Runde 23 stammt und vom Nutzer am
laufenden Programm nach der Auslieferung verworfen wurde. Der Satz „was der Spec nicht
verlangt hat, verlangt der Gebrauch" ist die Begründung dafür, dass die Aufteilung jetzt in
einer `NSSplitView` und nicht mehr in Autoresizing-Masken wohnt.

Die zweite Begründung von `EINZELHEITENHOEHE` ist erhalten und steht jetzt an
`Gitfenster::einzelheiten_einpassen`, also bei der Funktion, die sie brechen könnte: die
Fläche wächst nicht mit dem Text, weil sie sonst bei jedem Wechsel der Auswahl die Liste
darüber verschöbe. Die verschiebbare Grenze hebt das nicht auf, sondern bestätigt es — sie
hat jetzt einen Beweger, und der ist der Nutzer und nicht der Inhalt. Die Konstante selbst
ist gefallen; `ANFANGSANTEIL` trägt an ihrer Stelle die Zahl und den Nutzerbefund.

## Der Untergrenzen-Abschnitt

Neu eingetragen, jede Untergrenze am SDK nachgelesen
(`MacOSX.sdk/.../AppKit.framework/Headers/NSSplitView.h`):

| Berührung | Untergrenze | Fundstelle |
|---|---|---|
| Klasse `NSSplitView` | 10.0 | keine eigene Angabe |
| Protokoll `NSSplitViewDelegate` | 10.0 | keine eigene Angabe |
| `setVertical:` | 10.0 | `NSSplitView.h:31` |
| `dividerThickness` | 10.0 | `NSSplitView.h:60` |
| `splitView:constrainMinCoordinate:ofSubviewAt:` | 10.0 | `NSSplitView.h:148` |
| `splitView:constrainMaxCoordinate:ofSubviewAt:` | 10.0 | `NSSplitView.h:154` |
| `setPosition:ofDividerAtIndex:` | **10.5** | `NSSplitView.h:81` |

Eine davon ist jünger als ihre Klasse (`setPosition:ofDividerAtIndex:`, 10.5) und steht
deshalb in der Liste der jüngeren Berührungen. **Keine liegt über macOS 15.** Die höchste
Untergrenze der Datei bleibt `NSTableViewStyle` mit 11.0. `dividerStyle` (10.5) ist
absichtlich nicht angefasst: die Fensterzeile setzt ihn auch nicht, und die Vorgabe ist
dieselbe.

## Eine Folge, die erst `cargo clippy` gezeigt hat

Das eine neue Feld in `Sitzung` hat `Aufgabe::Sitzung` in `krk-ui/src/messmodus.rs` über
die Schwelle von `clippy::large_enum_variant` geschoben: die Variante trägt einen
`Messplan`, der eine ganze `Sitzung` enthält, und lag mit 264 gegen 48 Bytes genau 216 über
der zweitgrößten — die Grenze steht bei 200. Vor der Änderung waren es exakt 200 und damit
noch zulässig.

Behoben, wie clippy es vorschlägt: `plan: Box<Messplan>`. Das kostet eine Belegung beim
Start des Messmodus und **keine Zeile** an den fünf Lesestellen, weil `&Box<Messplan>` als
`&Messplan` durchfällt; angefasst sind allein die acht Erzeugungsstellen, eine davon im
Produktivcode. Der bestehende Präzedenzfall im Baum ist ein begründetes
`#[allow(clippy::result_large_err)]` (`krk-core/src/tasten/belegung.rs`), und seine
Begründung — Verpacken kostete an jeder Fundstelle eine ablenkende Zeile — trifft hier
gerade nicht zu. Deshalb der Fix und kein `allow`.

## Was ausdrücklich nicht angefasst ist

- **`Fenstermodell::sichtbar_setzen` bleibt der eine Schreiber der Sichtbarkeit.** Keine
  neue Zeile in `git.rs` blendet einen Bereich ein oder aus.
- Kein whole-tree-git-Kommando gefahren, nicht committet.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/git.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/fenstermodell.rs`
- `crates/krk-ui/src/messmodus.rs` (Folge des neuen Sitzungsfelds, siehe oben)
- `crates/krk-core/src/ablage/sitzung.rs`
- `crates/krk-core/tests/ablage.rs`

## Neue Proben

In `appkit::git::tests`: `ab_werk_bekommen_beide_flaechen_die_haelfte`,
`keine_der_beiden_flaechen_laesst_sich_wegziehen`,
`die_spanne_der_trennlinie_steht_nie_verkehrt`, `der_gemessene_anteil_ist_der_gesetzte`,
`zwei_hoehenlose_flaechen_tragen_keinen_anteil`. In `krk-core/tests/ablage.rs`:
`eine_sitzung_ohne_den_gitanteil_bleibt_lesbar`,
`der_gitanteil_steht_nur_dann_in_der_datei_wenn_eine_teilung_gesetzt_ist`.

Keine davon baut eine AppKit-Ansicht; die Rechnung liegt in reinen Funktionen.

## Was diese Arbeit nicht geprüft hat

Die Anzeige selbst. Der Abnahmelauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit;
dass die Trennlinie sich greifen lässt und die zwei Flächen sich wie gerechnet verhalten,
ist am laufenden Programm zu sehen und hier nur hergeleitet.
