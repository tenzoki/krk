Der Freigabedialog wird nicht aus einem Mausdruck heraus geöffnet, obwohl das SDK es verlangt

---

Der Kopf des Systems stellt an `showRelativeToRect:ofView:preferredEdge:` eine
Bedingung, die der Tastenweg `shift+cmd+s` nicht erfüllt:

```
/**
 Shows the picker, populated with sharing services related to the instance
 items. When the user selects one of the sharing services, the sharing service
 will be performed. Note that this method must be called on mouseDown.
 */
- (void)showRelativeToRect:(NSRect)rect ofView:(NSView *)view preferredEdge:(NSRectEdge)preferredEdge;
```

`NSSharingService.h:268-271`, am SDK gelesen
(`MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/`).

KRK ruft die Methode in `crates/krk-ui/src/appkit/teilen.rs:222`, und der eine
Weg dorthin ist ein Tastendruck: `ereignisse.rs` greift `NSEventMask::KeyDown`
über `addLocalMonitorForEventsMatchingMask:handler:` ab,
`Anwendungsdelegierter::kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:2240`) verzweigt auf
`Anwendungsdelegierter::teilen` (`:2470`), und die ruft `teilen::anbieten`. Ein
`mouseDown` steht nirgends in dieser Kette.

`inference:` Was daraus folgt, ist ohne laufendes Bündel nicht zu sehen. Der
Satz im Kopf ist eine Bedingung und keine Warnung; er trägt kein
`API_DEPRECATED` und keine Fehlerbeschreibung, also sagt das SDK nicht, was bei
Verletzung geschieht. Zwei Ausgänge sind denkbar und beide erklären dieselbe
Beobachtung nicht: der Dialog geht auf und bleibt stehen, oder er geht auf und
verschwindet sofort wieder, weil die Verfolgungsschleife des Menüs auf ein
Mausereignis wartet, das nicht kommt.

---

**Was zu tun ist**

Der Punkt gehört als benannter Gegenstand in den Abnahmelauf am Bündel, und
zwar getrennt von C1.1 im Ganzen: nicht „geht der Dialog auf", sondern „geht er
über die **Tastatur** auf und bleibt er stehen, bis der Nutzer wählt oder
abbricht". Über das Kontextmenü kommt der Dialog auf einem anderen Weg
zustande, über `standardShareMenuItem`, und der ist von dieser Bedingung nicht
betroffen; ein Lauf, der nur den Rechtsklick prüft, beantwortet die Frage
nicht.

Hält der Tastenweg am Bündel nicht, ist der Ausweichweg schon gebaut und
braucht keine zweite Berührung mit den Freigabediensten:
`teilen::eintrag_anfuegen` liefert den Systemeintrag, und ein `NSMenu` mit
genau diesem einen Eintrag lässt sich über `popUpMenuPositioningItem:atLocation:inView:`
an derselben Ankerfläche aufklappen, die `Anwendungsdelegierter::teilen` heute
schon aus `fokusansicht` holt. Das bliebe bei einem Menübauer und bei einer
Hülle; die Zusage C1.7 und C1.8 wären unberührt.

**Kontext**

- Betroffen ist C1.1, das erste Kriterium der Runde und ein Bündelkriterium.
  Alle Proben dieser Runde laufen grün, weil keine die Hülle anfasst — das ist
  ausdrücklich so gewollt (`teilen.rs`, Abschnitt „Diese beiden Hüllen tragen
  keine Probe"), und der Preis dieser Wahl ist genau, dass ein solcher Befund
  erst am Bündel fällt.
- Der Modulkopf von `teilen.rs` zitiert `:271` für die Verfügbarkeit
  („ohne eigene Angabe unter der Klassenangabe") und ist darin richtig: die
  Methode steht seit 10.8 mit ihrer Klasse. Gelesen worden ist die
  Verfügbarkeitszeile, nicht der Kommentar drei Zeilen darüber.
- Die Sitzungsaufzeichnung des `coder`
  (`history/260812-1434-coder-teilen-ueber-die-tastatur.md`) nennt die
  Bedingung nicht; sie ist nicht erwogen und verworfen, sondern nicht gesehen
  worden.
- Gefunden bei der Durchsicht von Turn 1 der Runde 6; nicht behoben, weil die
  Antwort am Bündel hängt und die Durchsicht nichts repariert.
