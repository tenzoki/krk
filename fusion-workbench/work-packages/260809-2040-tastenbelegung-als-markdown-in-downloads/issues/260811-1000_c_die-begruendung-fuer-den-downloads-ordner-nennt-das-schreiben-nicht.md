Die Begründung für den Downloads-Ordner nennt das Schreiben nicht

---

`resources/Info.plist:178-179`:

```xml
<key>NSDownloadsFolderUsageDescription</key>
<string>KRK zeigt deinen Download-Ordner an, damit du geladene Dateien von dort öffnen,
kopieren, verschieben und umbenennen kannst.</string>
```

Der Satz beschreibt fünf Handlungen, und keine davon ist die, die die Runde 3 hinzugefügt hat:
KRK legt seit `fd863e3` von sich aus eine Datei `KRK-Tastenbelegung.md` in diesem Ordner an.

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
**Betroffen:** `resources/Info.plist:178-179`
**Domain:** code

## Warum das eine Zeichenkette mit Gewicht ist

Dieser Satz ist das, was macOS dem Nutzer in der Rückfrage nach dem Mechanismus für
Transparenz, Zustimmung und Kontrolle vorlegt. Er ist damit die einzige Auskunft, die der Nutzer
in dem Augenblick hat, in dem er zustimmt oder ablehnt. Der Spec sagt in C3 der Runde 3 über
die dritte Spalte: „Eine Spalte, die eine falsche Zusicherung gibt, ist schlechter als eine
leere." Für diesen Satz gilt dasselbe, und er steht an einer Stelle mit mehr Folgen als eine
Tabellenzelle: wer liest, KRK wolle den Ordner nur **anzeigen**, und daraufhin zustimmt, hat
einer anderen Handlung zugestimmt als der, die folgt.

Die Rückfrage tritt zudem nach dem Plan gerade in dieser Runde zum ersten Mal an dieser Stelle
auf: S4 misst als erste seiner vier Fragen, „ob macOS beim ersten Schreiben eine Rückfrage
zeigt". Der Satz wird also zum ersten Mal in diesem Zusammenhang gelesen.

## Warum er bisher richtig war

Die Zeile stammt aus einer früheren Runde und beschrieb den Bestand richtig: bis zur Runde 3
schrieb KRK ausschließlich dorthin, wohin der Nutzer navigiert war. C2 hält diesen Unterschied
selbst fest — „Neu an diesem Schreibvorgang ist allein, dass KRK den Zielordner selbst wählt."
Genau diese Neuerung fehlt im Satz.

## Behebung

Ein Halbsatz, der die neue Handlung nennt, etwa: „… und damit KRK deine Tastenbelegung als
Markdown-Datei dorthin sichern kann." Er gehört zu S4 und nicht zu S3: erst dort ist gemessen,
ob und wann die Rückfrage überhaupt erscheint und wie ein abgelehnter Zugriff aussieht. Wer den
Satz vorher ändert, kann nicht prüfen, ob der Nutzer ihn je zu sehen bekommt.

---
Resolved: Der Text zu `NSDownloadsFolderUsageDescription` in `resources/Info.plist` nennt das
Schreiben jetzt: "... auf deinen Befehl hin sichert KRK ausserdem die Tastenbelegung als
Markdown-Datei in diesen Ordner." `plutil -lint` meldet OK.

**Der Befund wog schwerer, als dieser Datensatz vermutete, und das gehoert festgehalten.** Der
`coder` hat zuerst geprueft, ob ueberhaupt etwas zu tun ist — die Vermutung des Orchestrators
lautete, KRK laufe ausserhalb der Sandbox und der Ordner sei nicht TCC-geschuetzt. Sie traegt
nicht. Am Geraet belegt, macOS 15 (Darwin 24.6.0): `TCC.framework/Resources/Localizable.loctable`
fuehrt den Rueckfragetext `REQUEST_ACCESS_SERVICE_kTCCServiceSystemPolicyDownloadsFolder`, und
`TCC.framework/Support/tccd` traegt den Schluessel `NSDownloadsFolderUsageDescription` selbst,
liest ihn also aus dem Buendel des Anfragenden. Nichts daran haengt an der Sandbox.

**Und der eigentliche Fund:** eine TCC-Zusage gilt je Paar aus Programm und Dienst, nicht je
Vorgang. KRK zeigt den Downloads-Ordner seit Runde 1 an und loest die Rueckfrage damit beim
**Anzeigen** aus, lange vor dem ersten Schreiben. Der Nutzer stimmt einmal zu, auf Grundlage
dieses Satzes, und dieselbe Zusage deckt danach das Schreiben mit ab, ohne eine zweite
Rueckfrage. Der Satz war also nicht bloss unvollstaendig — er beschaffte Zustimmung fuer eine
Handlung, die er nicht nannte.

Ob und wann die Rueckfrage am gebauten Buendel wirklich erscheint, bleibt in S4 zu messen; der
Satz ist jetzt schon richtig, wenn sie erscheint.

Geschlossen in der Sitzung `history/260811-0107-orchestrator-session.md`, Turn 1.
