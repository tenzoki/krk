# Was heißt „fragt das Dateisystem auf dem Hauptfaden“ bei einer Übergabe an das System?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md, 261016-1956-der-loeschweg-fragt-canonicalize-und-den-papierkorb-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md, 261102-1304-dateisystemfragen-vom-hauptfaden-nehmen.md

---

## Question

Die Directive des Pakets verlangt, dass keiner der Wege aus `261030-1259-…` auf dem Hauptfaden das Dateisystem fragt. Für KRKs eigene Fragen ist das am Quelltext entscheidbar: `std::fs`, `canonicalize`, `NSURL resourceValuesForKeys:`, Abfragen an `NSFileManager` und `fileURLWithPath:` ohne `isDirectory:` stehen im Rumpf oder nicht. Am Ende mehrerer Wege steht aber eine Übergabe an das System, deren Inneres KRK nicht sieht: `NSWorkspace openURL:` (Standardprogramm), `URLsForApplicationsToOpenURL:` (Untermenü „Öffnen mit“, gebaut beim Rechtsklick), `openURLs:withApplicationAtURL:…`, `activateFileViewerSelectingURLs:` (Im Finder anzeigen), `NSPasteboard writeObjects:` mit Dateiadressen (`cmd+c`/`cmd+x`) und `NSSharingServicePicker` (Teilen). `inference:` Mindestens `URLsForApplicationsToOpenURL:` muss die Art des Eintrags kennen und fragt dafür vermutlich selbst die Platte; gemessen ist das an keiner hängenden Freigabe. Die Frage ist, ob die Zusage des Pakets diese Übergaben einschließt. Sie bindet über den Plan hinaus, weil jede spätere Arbeit an einem Hauptfadenweg dieselbe Linie braucht.

## Options

1. **Die Zusage reicht bis zu KRKs eigenen Fragen.** Jede Frage, die KRK selbst stellt, geht auf einen Nebenfaden; jede Dateiadresse entsteht über `fileURLWithPath:isDirectory:` mit der Art aus dem Ordnermodell. Die Übergaben an LaunchServices, Finder, Ablage und Freigabedienst bleiben auf dem Hauptfaden, als benannter Rest in den Modulköpfen. Ob einer davon an einer hängenden Freigabe die Ereignisschleife anhält, entscheidet eine Messung durch den Nutzer und keine Vorhersage.
   - Pros: am Quelltext prüfbar, Quelltextproben wie bei `ctrl+o` halten es; keine neue Kiste; Verhalten unverändert.
   - Cons: ein Rest bleibt, dessen Größe niemand kennt, bis gemessen ist.
2. **Auch die Übergaben gehen vom Hauptfaden.** `openURL:` und die übrigen `NSWorkspace`-Aufrufe laufen auf dem Nebenfaden; das Untermenü „Öffnen mit“ füllt sich nachträglich.
   - Pros: schließt den Rest, soweit die Aufrufe abseits des Hauptfadens erlaubt sind.
   - Cons: ob `NSWorkspace` abseits des Hauptfadens zugesagt ist, ist für jede Methode einzeln nachzuweisen (`speculation:` für einen Teil ja); ein Menü, das sich nach dem Öffnen umbaut, ist ein neues Verhalten; Ablage und Freigabedienst sind AppKit-Objekte des Hauptfadens und bleiben ohnehin dort.
3. **Möglichkeit 1, und „Öffnen mit“ fragt nach dem Inhaltstyp statt nach der Datei.** `URLsForApplicationsToOpenContentType:` (macOS 12) mit einem `UTType` aus Endung und Ordnerangabe fragt die Platte nicht.
   - Pros: schließt den wahrscheinlichsten Rest ohne nachträgliches Menü.
   - Cons: neue Kiste `objc2-uniform-type-identifiers`; eine Datei ohne Endung und eine von Hand gesetzte Zuordnung je Datei bekommen eine andere Liste als heute.

## Constraints

Kein neues `unsafe` außerhalb von `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/`; keine Klasse oder Methode über macOS 15; eine neue Kiste nur mit Begründung in der Wurzel-`Cargo.toml` und ohne `cc` oder ein Paket mit einem Namen auf `-sys` auf den Mac-Zielen.

## Recommendation

Möglichkeit 1. Sie ist die Frage, die der Quelltext beantwortet, und sie schließt jeden Weg aus `261030-1259-…` bis auf die Übergabe selbst. Der Rest wird gemessen statt vermutet: zeigt der Lauf an einer hängenden Freigabe, dass eine Übergabe die Ereignisschleife anhält, ist Möglichkeit 3 für „Öffnen mit“ und Möglichkeit 2 für die übrigen der Gegenstand eines eigenen Pakets. Der Plan des Pakets baut auf Möglichkeit 1; keiner seiner Schritte wird unter 2 oder 3 überflüssig.

---
Answered: Der Nutzer am 261102 mit „1“ auf die Vorlage „Plan freigeben, A und B wie vorgeschlagen“. Damit gilt Möglichkeit 1: die Zusage reicht bis zu KRKs eigenen Fragen; die Übergaben an das System bleiben als benannter Rest und werden an einer hängenden Freigabe gemessen. Ruled by user, Kai Stalmann <ks@qantr.com>.

Nachtrag 261105: Der Lauf schließt auch `readObjectsForClasses:options:` und `setDirectoryURL:` mit dem Blatt dahinter ein (`261104-2001-die-liste-der-uebergaben-in-der-regel-laesst-setdirectoryurl-und-readobjects-aus.md`).
