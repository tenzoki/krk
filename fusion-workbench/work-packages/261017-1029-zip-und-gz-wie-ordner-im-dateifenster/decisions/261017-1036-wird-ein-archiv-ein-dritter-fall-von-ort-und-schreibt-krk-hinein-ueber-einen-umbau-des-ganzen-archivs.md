# Wird ein Archiv ein dritter Fall von `Ort`, und schreibt KRK in ein Archiv über einen Umbau, der das ganze Archiv neben dem Original neu schreibt und es atomar ersetzt?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261017-1029-zip-und-gz-wie-ordner-im-dateifenster.md, 261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md, 261016-1956-welche-form-hat-ein-ort-im-dateifenster-und-wie-haelt-der-bau-fernaufrufe-vom-hauptfaden-fern.md, 261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md

---

## Question

Das Arbeitspaket `261017-1029-zip-und-gz-wie-ordner-im-dateifenster` verlangt, dass ein Dateifenster eine `.zip`- oder `.gz`-Datei wie einen Ordner zeigt und dass Kopieren, Verschieben, Umbenennen, Anlegen und Löschen darin wirken, „ohne dass ein Fehlschlag das Archiv beschädigt“. Das sftp-Paket hat `Ort` mit zwei Fällen eingeführt (`Lokal`, `Fern`) und daneben eine Maschine, deren Schreibhandlungen sofort wirken: jedes Anlegen, Umbenennen und Entfernen ist eine eigene Anfrage an den Server. Ein Archiv kennt keine solche Einzelhandlung. Ein Zip lässt sich nur über ein neu geschriebenes Zentralverzeichnis ändern, ein tar.gz nur durch Neupacken des ganzen Stroms. Zu entscheiden ist, welche Form ein Ort im Archiv hat und auf welchem Weg geschrieben wird. Die Festlegung bindet über dieses Paket hinaus, weil jede spätere Arbeit an Archiven auf ihr aufsetzt: Archive auf einem Server, die Vorschau mit Inhalt und weitere Formate.

## Options

1. **`Ort::Archiv(Archivort)` als dritter Fall; gelesen wird über ein Inhaltsverzeichnis, geschrieben über genau einen Umbau je Vorgang.**
   - `Archivort` trägt einen geteilten `Archivzugang` (lokale Archivdatei, Format, zwischengespeichertes Inhaltsverzeichnis) und einen Pfad im Archiv. Ob ein Ort im Archiv liegt, entscheidet der Wert, ob eine Datei ein Archiv ist, ihre Endung; beides ohne Systemaufruf.
   - Lesen: Der Lesevorgang des Tabs und die Kopiermaschine lesen über das Inhaltsverzeichnis. Beim Zip ist das das Zentralverzeichnis, bei tar und gz ein einmaliger Durchlauf des Stroms, zwischengespeichert, solange die Datei unverändert ist.
   - Schreiben: Ein Vorgang, der ein Archiv ändert, sammelt alle Änderungen und schreibt **einmal** ein neues Archiv unter einem Zwischennamen neben das Original. Unveränderte Zip-Einträge kopiert er roh und packt sie nicht neu. Ist das neue Archiv vollständig, ersetzt `rename(2)` das Original. Abbruch, Fehler und eine Änderung des Originals von außen lassen das Original Byte für Byte stehen.
   - Herauskopieren läuft durch die vorhandene Fernmaschine mit einer Archivquelle neben der lokalen und der fernen; ihre Zielklärung, Konfliktfrage und ihr Tausch gelten unverändert für das Ziel.
   - Pros: Ein Fehlschlag kann das Archiv nicht beschädigen, weil das Original vor dem letzten `rename` nie geöffnet zum Schreiben ist. Ein Vorgang kostet einen Umbau, gleich wie viele Einträge er berührt. Tabs, Zulässigkeit, Fenstertitel, Sitzung und Konfliktblatt nehmen den dritten Fall über die Fallunterscheidungen auf, die das sftp-Paket gebaut hat; der Übersetzer nennt jede Stelle.
   - Cons: Auch eine kleine Änderung schreibt das ganze Archiv neu; bei einem Zip sind das rohe Kopien, bei einem tar.gz ein Neupacken. Ein abgebrochener Vorgang hinterlässt **nichts** im Archiv, auch nicht die Einträge, die schon fertig waren; lokal bleibt dagegen stehen, was fertig ist. Das Neuschreiben gibt dem Archiv einen neuen Inode: harte Verweise auf das alte, erweiterte Attribute und Finder-Etiketten der Archivdatei gehen verloren.
2. **Ein Archiv als weitere `Stelle` der Fernmaschine mit sofortiger Wirkung**, jede Einzelhandlung ein eigener Umbau.
   - Pros: Keine zweite Schreibbahn neben der Maschine.
   - Cons: n Einträge kosten n Umbauten, bei einem Ordner mit tausend Dateien in einem großen Archiv also tausend vollständige Neuschreibungen. Ein Abbruch nach der Hälfte lässt ein halb geändertes Archiv stehen. Der Tausch der Maschine (beiseitelegen, Ersatz auf den Namen, Rest entfernen) ist für einen Eintrag im Archiv sinnlos und müsste für diese Stelle Sonderzweige bekommen.
3. **Anhängen an das Original für das Hinzufügen** (`ZipWriter::new_append`), Umbau nur für Löschen und Umbenennen.
   - Pros: Hinzufügen kostet nur die neuen Bytes.
   - Cons: Das Anhängen überschreibt das alte Zentralverzeichnis **im Original**; ein Abbruch, ein voller Datenträger oder ein Absturz mittendrin hinterlässt ein Archiv ohne gültiges Zentralverzeichnis. Das verletzt die Directive wörtlich. Dazu kämen zwei Schreibwege mit verschiedenen Zusagen, und für tar.gz gibt es kein Anhängen.
4. **Beim Betreten in einen Arbeitsordner entpacken, beim Verlassen zurückpacken.**
   - Pros: Danach ist alles ein lokaler Ordner.
   - Cons: Ein großes Archiv kostet beim Betreten seine volle entpackte Größe an Zeit und Platz. Zwei Wahrheiten bestehen nebeneinander, und es ist offen, wann zurückgepackt wird und was bei einem Beenden oder Absturz dazwischen gilt.

## Constraints

- Ein Fehlschlag beschädigt das Archiv nicht (Directive des Pakets, wörtlich).
- Kein Lesen auf dem Hauptfaden; die zehn Zeitzusagen aus C8 und der lokale Weg bleiben unberührt.
- Vollständige Fallunterscheidungen ohne Auffangzweig; `#![deny(unsafe_code)]` mit den zwei Ausnahmedateien; kein Paket mit `cc` oder einem Namen auf `-sys` auf den Mac-Zielen.

## Recommendation

Möglichkeit 1. Sie ist die einzige, die die Zusage „das Original wird nicht beschädigt“ an einer einzigen Stelle hält, nämlich am letzten `rename` des Umbaus, statt sie über viele Einzelhandlungen zu verteilen. Sie nimmt zudem den dritten Fall über die Nähte auf, die `Ort` schon hat. Den Preis „jede Änderung schreibt das ganze Archiv“ nennt `HowTo.md`. Ein Archiv auf einem Server bleibt in diesem Paket ausgegraut; `Archivzugang` hält dafür eine lokale Archivdatei. Ein späteres Paket kann sie gegen einen `Ort` tauschen, sobald es ein atomares Ersetzen auf dem Server gibt, das Entscheidung 10 des sftp-Plans nicht verletzt.
