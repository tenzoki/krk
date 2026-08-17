Der auslösende Defekt des Räumens ohne Rückfrage ist behoben und steht weiter offen

---

`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md` ist der
Defekt, der diese Runde ausgelöst hat. Was er verlangt, steht seit Bündel A am Baum: beide
Löschbefehle gehen durch dasselbe Blatt. Der Datensatz trägt weiter `_o_`, und kein Schritt des
Plans schließt ihn.

---

**Schwere:** Niedrig. Kein Fehlverhalten am Code. Der Preis ist ein Speicherstand, der die
tragende Zusage dieser Runde als offen führt — und das ist der Datensatz, an dem jemand die
Frage „ist der Nutzer geschützt" nachliest.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`,
`circles/260817-0833-.../planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Schritt 16
**Baumstand:** `ee85950`
**Domain:** code

## Was der Defekt verlangt und was am Baum steht

Der Datensatz verlangt: „dass **jede** Datei-Löschfunktion durch eine Rückfrage gesichert wird
und der Nutzer bestätigen muss. Von den beiden Löschwegen, die KRK führt, fragt heute nur einer
nach."

Am Baum `ee85950` gehen beide durch denselben Rumpf, und das ist einzeln nachgelesen:
`Kommando::InPapierkorb` über `in_den_papierkorb` (`anwendung.rs:4455`) und
`Kommando::EndgueltigLoeschen` über `endgueltig_loeschen` (`:4561`) rufen beide
`loeschen_nach_rueckfrage` (`:4639`), und dessen einziger Zweig mit einem Auftrag ist
`Vorstufe::Rueckfrage` mit bestätigtem Blatt.

## Warum der Marker nicht von selbst wandert

Der Plan führt in Schritt 16 die **Entscheidungsdatensätze** nach und in Schritt 15 die Prosa
im Baum und in `CLAUDE.md`. Ein Schritt für die Defektdatensätze steht in keinem der fünf
Bündel. Die Datensätze dieses Circles hat der `coder` beim Beheben selbst mitgezogen, wie die
sechs `_c_`-Dateien unter `issues/` zeigen; der auslösende Defekt liegt im gemeinsamen Speicher
und in keinem Aufgabenauftrag.

## Richtung

Der Datensatz bekommt seine Zeile `Resolved:` mit dem Commit von Schritt 3 (`472eb81`) und
wandert auf `_c_`, und zwar erst, wenn jemand die Zusage am Baum nachgelesen hat — das ist mit
dieser Durchsicht geschehen. Zu entscheiden bleibt, ob er jetzt wandert oder mit Bündel D, wenn
der zweite Löschweg ganz gefallen ist; sein Wortlaut ist von Bündel D nicht mehr betroffen, also
spricht nichts für das Warten.

Daneben gehört in den Plan ein Schritt für die Defektdatensätze neben Schritt 16, sonst hängt
ihr Nachzug jedes Mal daran, dass ein Aufgabenauftrag sie zufällig einschließt.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, und die tragende Behauptung
dieses Datensatzes hält nicht.**

Der Datensatz sagt: „Was er verlangt, steht seit Bündel A am Baum", und die `## Richtung`
schließt daraus, „sein Wortlaut ist von Bündel D nicht mehr betroffen, also spricht nichts für
das Warten". Beide Sätze gelten allein für die erste Hälfte von
`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`. Dessen
Abschnitt `## Verschärfung vom 260817: der endgültige Löschweg fällt ganz weg` verlangt drei
weitere Dinge, und keines davon ist gebaut: `Kommando::EndgueltigLoeschen` fällt weg, `f8` und
`opt+cmd+delete` werden frei, und die Nutzerfestlegung vom 260802 ist ganz überholt. Am Baum
stehen 22 Zeilen `EndgueltigLoeschen` in 12 Dateien, und `resources/default-keymap.toml:151`
führt `endgueltig_loeschen` unverändert. Der Abgleich vom 260817-1129 hat diesen Abschnitt
zitiert und den Marker aus genau diesem Grund stehen gelassen; die Durchsicht hat den Absatz
nicht mitgelesen.

**Was von diesem Datensatz trägt**, und weshalb er nicht geschlossen wird: sein zweiter Punkt.
Der Plan führt in Schritt 16 die Entscheidungsdatensätze nach und in Schritt 15 die Prosa, aber
keinen Schritt für die Defektdatensätze; der auslösende Defekt liegt im gemeinsamen Speicher und
in keinem Aufgabenauftrag. Das bleibt richtig und ist eine Frage an den Planner, nicht an den
Abgleich. Der Marker bleibt `_o_`.
