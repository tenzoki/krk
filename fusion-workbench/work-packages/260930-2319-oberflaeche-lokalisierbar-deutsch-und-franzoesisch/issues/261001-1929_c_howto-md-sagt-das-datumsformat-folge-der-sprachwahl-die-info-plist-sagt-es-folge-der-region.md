HowTo.md sagt, das Datumsformat folge der Sprachwahl; die Info.plist sagt, es folge der Region
---
Zwei Texte im Bereich `b81a284..e46a678` widersprechen sich.

`HowTo.md:33-37` (Abschnitt `## Die Sprache`, neu in diesem Arbeitspaket): „Mit derselben Wahl beschriftet macOS, was es selbst stellt: die Größenangaben in der Dateiliste, in der Vorschau und in der Statuszeile, **das Datumsformat**, das Kontextmenü eines Textfeldes, das Über-Fenster und die Rückfrage beim ersten Zugriff auf einen geschützten Ordner. Menüs, Blätter, Statuszeile und Vorschau von KRK folgen ihr, und beide Hälften sind so immer einig.“

`resources/Info.plist:73-75` (Kommentar an `CFBundleLocalizations`, in diesem Arbeitspaket neu gefasst): „Nicht betroffen ist die Spalte "Aenderungsdatum": `NSDateFormatter` folgt der Region des Systems und nicht dieser Liste.“

KRK formatiert das Datum in `crates/krk-ui/src/appkit/tabelle.rs:4855-4856` und `crates/krk-ui/src/appkit/vorschau.rs:867-869` über `NSDateFormatter` mit `ShortStyle` und ohne eigene `setLocale`. Inference: die numerische Kurzform folgt dem Regionsformat und nicht der Sprachwahl je Programm, wie die Info.plist sagt; gemessen ist das in diesem Arbeitspaket nicht, und die Abnahme des Nutzers (Plan, `## Where this work stops`, Teil 2) prüft das Datum nicht ausdrücklich. Wer seine Region auf Deutschland und KRK auf Französisch stellt, bekommt nach der Info.plist ein deutsches Datumsformat neben französischen Menüs, und „beide Hälften sind so immer einig“ stimmt dann nicht.

Abnahme: beide Texte sagen dasselbe über das Datumsformat, und zwar das, was der Abnahmelauf des Nutzers am gebauten Bündel zeigt (Region Deutschland, KRK auf Französisch, Spalte „Date“ und Metadatenzeile der Vorschau ansehen). Bis dahin nennt `HowTo.md` das Datumsformat nicht unter dem, was der Sprachwahl folgt.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Abschlussdurchsicht `261001-1929-reviewer-lokalisierung-abschluss.md` über `b81a284..e46a678`; Arbeitspaket `260930-2319-oberflaeche-lokalisierbar-deutsch-und-franzoesisch`. Schwere: Low.
---
Resolved: 261001 — `HowTo.md` (Abschnitt `## Die Sprache`) zählt das Datumsformat nicht mehr zu dem, was der Sprachwahl folgt. Ein eigener Absatz sagt wie `resources/Info.plist`, dass es voraussichtlich der Region folgt, mit dem Beispiel Region Deutschland und KRK auf Französisch, und dass das am gebauten Programm noch nicht geprüft ist. Die Bestätigung gehört in den Abnahmelauf des Nutzers (Plan, `## Where this work stops`, Teil 2). Nicht committet.
