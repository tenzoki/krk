Die Kommentarzeilen der Auslieferungsfassung sagen nicht, dass ein Schreibfehler die ganze Datei kostet

---

`resources/default-readers.toml` entsteht mit Schritt 7 und erklärt nach C5.10 alle vier
Bausteine. Was sie daneben sagen muss und heute in keinem Plan steht: ein verschriebener
Schlüssel in einer `[[profil.zeile]]` kostet nicht die eine Zeile, sondern die **ganze
Datei**. Sie gilt dann nach C1.6 als beschädigt, wird beiseitegelegt, und KRK arbeitet ohne
jedes Profil weiter. Der Nutzer liest die Datei genau an der Stelle, an der er sie bearbeitet;
dort gehört der Satz hin und nicht allein in einen Modulkopf.

---

**Der Rest jenes Befundes ist am 260824-1242 behoben**, siehe
`issues/260824-1217_c_ein-tippfehler-in-einem-bausteintisch-kostet-alle-profile-und-die-meldung-nennt-ihn-nicht.md`.
Die Meldung nennt den Schlüssel seit `leseprofil::datei` ohne die unmarkierte Auswahl, und der
Modulkopf jener Datei schreibt die drei Reichweiten einzeln aus. Offen bleibt allein die
Hälfte, die in einer Datei steht, die dieser Baum noch nicht führt.

**Was zu tun ist**, sobald `resources/default-readers.toml` dasteht: eine Kommentarzeile im
Kopf der Datei, die zwei Dinge sagt — ein Schreibfehler in einem Baustein kostet die ganze
Datei und nicht die eine Zeile, und eine Zeile trägt genau einen der vier Bausteine
`zaehlung`, `juengste`, `feld`, `vorhandensein`.

**Warum das kein Teil von Schritt 7 ist, so wie er dasteht:** seine Vorgabe ist C5.10, und die
verlangt eine Erklärung der vier Bausteine. Die Reichweite eines Schreibfehlers ist eine
Aussage über das Laden und steht in keinem Abnahmekriterium; wer Schritt 7 nach seinem
Wortlaut fährt, schreibt sie nicht.

**Schwere:** niedrig. Kein Fehlverhalten. Der Preis ist, dass ein Nutzer mit einem
Buchstabendreher alle Zusammenfassungen verliert und in der Datei, die er gerade bearbeitet,
nichts darüber gelesen hat.

**Gefunden:** coder, bei der Befundräumung am 260824-1242.

**Betroffen:** `resources/default-readers.toml` (steht noch aus, Schritt 7, `ontocoder`)

**Domain:** data

---
Resolved: `resources/default-readers.toml` steht seit dem 260824-1313 (Schritt 7) und trägt
im Kopf den Abschnitt „Was ein Schreibfehler kostet". Er sagt beides, was hier gefordert war:
ein verschriebener Schlüssel in einem Baustein kostet die ganze Datei und nicht die eine
Zeile, und er zählt die drei Reichweiten einzeln auf. Dass eine Zeile genau einen der vier
Bausteine `zaehlung`, `juengste`, `feld` und `vorhandensein` trägt, steht im Abschnitt
darunter, „Der Aufbau: Profil, Zeile, genau ein Baustein". Beide stehen vor dem ersten
`[[profil]]`, also an der Stelle, an der der Nutzer die Datei zu bearbeiten anfängt.
