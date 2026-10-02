# Shaper, Klärungsrunde 2 zu „Ins gleiche Verzeichnis gehen" und „Abwurf aus einer fremden App"

**Datum:** 2026-08-18
**Baumstand:** `8d5baf6`
**Modus:** dispatched (kein `AskUserQuestion`), zweite Klärungsrunde. Die erste hat ein voriger Lauf gefahren; dieser Lauf hatte keinen Zugriff auf dessen Gedächtnis und hat jeden übernommenen Befund neu am Baum geprüft.
**Ergebnis:** kein Spec. Vier Fragen gehen an den Nutzer zurück, ein Entscheidungsdatensatz ist angelegt.

## Was nachgeprüft ist

Alles Folgende ist an diesem Baum oder am macOS-SDK gelesen, nicht übernommen.

- **Ziehen und Ablegen fehlt in beide Richtungen.** `registerForDraggedTypes`, `draggingEntered`, `draggingUpdated`, `performDragOperation`, `beginDraggingSession`, `NSDragOperation` und `NSFilePromise` kommen in `crates/` nicht vor. `NSPasteboardTypeFileURL` steht allein in `appkit/zwischenablage.rs`.
- **`Kommando` trägt 78 Werte**, keiner stellt ein Dateifenster auf einen fremden Ordner.
- **`DateifensterQuelle::ordner_lesen`** (`appkit/tabelle.rs:853`) ist der eine Weg, ein Dateifenster umzustellen; die Fähigkeit 1 wird sein vierter Rufer.
- **Das andere Dateifenster wird schon so aufgelöst, wie die Fähigkeit 1 es braucht:** `Anwendungsdelegierter::uebertragen` (`anwendung.rs:4428`) liest `self.dateifenster(aktiv.andere()).quelle().angezeigter_ordner()`.
- **`auftrag_stellen`** (`anwendung.rs:5302`) nimmt seine Quellen aus der Auswahl des aktiven Fensters. Ein Abwurf braucht deshalb einen eigenen Einstieg daneben; `Auftrag::kopieren` und `Auftrag::verschieben` (`krk-core/src/operation/auftrag.rs:89`, `:94`) nehmen beliebige Pfade und stehen bereit.
- **KRK hält genau einen Vorgang** (`anwendung.rs:5348`).
- **Ein umgestelltes Dateifenster behält mehr als seinen Filtertext.** `Tabliste::ordner_setzen` (`tabs.rs:653-681`) trägt unbedingt vier Größen des Tabs hinüber: Filtertext, `tief` (Deep), `inhalt` (Inhaltsfilter) und die Sortierung samt der Anzeige versteckter Dateien. Zwei Dateifenster können danach denselben Ordner zeigen und sichtbar verschiedene Bestände führen. Der Auftrag an diesen Lauf nannte nur den Filtertext.
- **Freie Kombinationen** für die Fähigkeit 1, am 260818 aus `resources/default-keymap.toml` ausgezählt: `opt+cmd+s`, `opt+cmd+up`, `opt+cmd+down`, `shift+cmd+o`, `ctrl+cmd+left`, `ctrl+cmd+right`, `shift+cmd+t`, `cmd+l`. Die Datei erklärt in ihrem Kommentar zu `opt+cmd+o` selbst, dass die `opt+cmd`-Reihe trägt, „was einen Ordner herstellt oder liefert".
- **`Fenstermodell::einblenden`** (`fenstermodell.rs:735`) trägt `#[must_use]` und liefert `false`, wenn die Mindestbreiten bei der aktuellen Fensterbreite nicht mehr nebeneinander passen.

## Was am SDK geprüft ist

Gelesen unter `MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/`.

- **Die zwei Abwurfziele, die der Nutzer verlangt hat, kennt AppKit bereits, samt zwei unterscheidbaren Markierungen.** `NSTableView.h:317` schreibt aus: `setDropRow:dropOperation:` mit `row = -1` und `NSTableViewDropOn` bezeichnet „a drop on the entire tableview". Der Ordner unter dem Zeiger ist derselbe Aufruf mit der Zeilennummer. Die Zeilenhervorhebung und die Hervorhebung der ganzen Tabelle sind damit keine Eigenbauten.
- **`NSTableViewDropOperation` kennt genau zwei Werte** (`NSTableView.h:25-28`), `DropOn` und `DropAbove`. `DropAbove` zeichnet eine Einfügelinie und meint eine Reihenfolge; für „in den angezeigten Ordner" ist `row = -1` die zutreffende Form.
- **Das Ziel wählt aus `draggingSourceOperationMask`** (`NSDragging.h:72`) und gibt seine Wahl aus `draggingEntered:` oder `draggingUpdated:` zurück (`:118-119`).
- **`draggingUpdated:` läuft bei stillstehender Maus nicht von selbst**, wenn das Ziel `wantsPeriodicDraggingUpdates` verneint (`:126-127`).
- **Keine berührte Klasse liegt über macOS 15.** `registerForDraggedTypes:` und `NSDraggingInfo` seit 10.0, `NSFilePromiseReceiver` seit 10.12 (`NSFilePromiseReceiver.h:19`).
- **Zusagefeilen liefern ihre Dateien nebenläufig.** `receivePromisedFilesAtDestination:options:operationQueue:reader:` (`NSFilePromiseReceiver.h:41`) schreibt selbst in einen Zielordner, auf einer eigenen Warteschlange. Das ist ein zweiter Schreibweg in den Zielordner, den `Auftrag`, der Fortschritt und die Konfliktrückfrage nicht abdecken.

## Was dieser Lauf angelegt hat

`shared/decisions/260818-1453_*_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md`, offen. Er hält fest, warum die Nutzerantwort „Kopieren als Vorgabe, `shift` erzwingt Verschieben" nachgeprüft werden musste: `shift` ist im Ziehdienst zwar frei, aber `opt` und `cmd` sind es nicht, und sie verengen die Vorgangsmenge, aus der das Ziel wählen darf, bevor KRK sie sieht. Zwei Instanzen beantworten dann dieselbe Frage und widersprechen sich im häufigsten Fall. Fünf Möglichkeiten stehen im Datensatz.

## Was offen bleibt

Die vier Fragen, die dieser Lauf zurückgibt: die Zusatztaste (der Datensatz oben), der Umfang der Fähigkeit 2 (Zusagefeilen, KRK als Ziehquelle), was mit Filter, Deep und Inhaltsfilter des umgestellten Dateifensters geschieht, und was ein Abwurf tut, der nicht ausgeführt werden kann.

Drei Kleinigkeiten gehen als begründete Vorgaben in den Spec und nicht als Frage: die Tastenkombination (`opt+cmd+s`, aus der Familienregel der Belegungsdatei), die zwei Abwurfmarkierungen (die von AppKit, siehe oben) und der Fokus nach dem Abgleich (er bleibt stehen, denn der Befehl ist kein Fokusbefehl).
