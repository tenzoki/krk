Ohne Menü "Bearbeiten" lässt sich in kein Textfeld einfügen, und C2 verlangt genau das

---

`crates/krk-ui/src/appkit/menue.rs` baut das Hauptmenü von Hand und führt zwei Menüs, das Anwendungsmenü und "Fenster". Ein Menü "Bearbeiten" gibt es nicht.

Auf dem Mac liegen `Cmd+X`, `Cmd+C`, `Cmd+V` und `Cmd+A` für Textfelder **nicht** im Textsystem, sondern als Menükürzel im Menü "Bearbeiten". Ohne dieses Menü erreicht kein Tastendruck die Aktionen `cut:`, `copy:`, `paste:` und `selectAll:`, und ein Textfeld von KRK nimmt weder einen eingefügten Text an noch gibt es einen her.

Gemessen am 260804-1309 am laufenden Bündel: in der Pfadeingabe aus C2 den Pfad `/tmp/krk-abnahme/Beta` in die Zwischenablage gelegt, `shift+cmd+g` gedrückt, `cmd+v` gesendet und bestätigt. Das Feld trug danach unverändert seinen Startwert; eingefügt wurde nichts.

---

## Warum es zählt

C2 nennt das Einfügen ausdrücklich, nicht nebenbei:

> Ein Tastenbefehl öffnet eine Pfadeingabe. Der Nutzer **tippt oder fügt** einen absoluten Pfad **ein** und landet im Zielordner (…)

Das Abnahmekriterium ist damit heute zur Hälfte nicht erfüllbar. Das zweite Kriterium desselben Abschnitts verschärft es:

> In der Pfadeingabe, im Umbenennen-Feld und in jedem anderen Textfeld behalten **alle** Tasten ihre gewohnte Mac-Bedeutung.

`Cmd+V` gehört zu den gewohnten. Der Fall trifft nicht nur die Pfadeingabe: das Umbenennen-Feld aus C4 und die fünf Blätter aus S16 und S17 erben dieselbe Lücke, weil sie dieselbe Hülle benutzen.

## Warum es kein Widerspruch zu C3 ist

C3 hält `Cmd+C` und `Cmd+V` ab Werk **unbelegt**, und das bleibt richtig. Die beiden Kombinationen gehören dort einer Zwischenablage für Dateien einer späteren Runde, also einer Funktion des Dateifensters. Ein Menü "Bearbeiten" belegt sie nicht in der Belegungsmaschine, sondern im Menü, und der Fokusvorbehalt aus S13 sorgt dafür, dass beide Wege sich nicht ins Gehege kommen: steht die Schreibmarke in einem Textfeld, reicht der Abgriff jeden Tastendruck unverändert an AppKit weiter, und nur dort wirkt das Menü.

Ein zweiter Berührungspunkt besteht trotzdem und ist zu klären, bevor jemand das Menü baut: `issues/260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md` führt heute genau ein Menükürzel außerhalb der Konflikterkennung aus C3. Ein Menü "Bearbeiten" brächte vier weitere.

## Was zu tun ist

Nicht in diesem Schritt. Die Dateiliste von S13 nennt `menue.rs` nicht, und vier Menükürzel außerhalb der Konflikterkennung sind eine Entscheidung des Nutzers und keine Nebenwirkung einer Umsetzung.

Vorgeschlagen: ein Menü "Bearbeiten" mit den vier Standardeinträgen und ohne festes Ziel, so wie "Fenster einblenden" es seit S12 vormacht; die Antwortkette findet dann den Feldeditor. Dazu ein Satz im Spec, der sagt, wie sich diese vier Kürzel zur Konflikterkennung aus C3 verhalten.

---

Herkunft: gefunden bei der Umsetzung von Schritt 13 am 260804-1309, beim Nachweis des Abnahmekriteriums 5 aus C2 im laufenden Bündel.

---

**Nachtrag 260805-0000: der Plan trägt den Schritt, der Defekt bleibt offen bis zur Umsetzung.**

Der Nutzer hat am 260805-0000 zugleich die Frage beantwortet, die dieser Datensatz als Berührungspunkt nennt: die Menükürzel ziehen in die Konflikterkennung aus C3 ein. Damit kostet das Menü "Bearbeiten" keine vier Kombinationen außerhalb der Konflikterkennung mehr, sondern schließt umgekehrt einen blinden Fleck, den es sonst von zwei auf sechs vergrößert hätte. Beides ist demselben Schritt zugeordnet.

- **S13c** baut das Menü "Bearbeiten" mit den vier Standardeinträgen und Ziel `nil`, lässt das Hauptmenü seine Kürzel aus der Belegung nehmen und tauscht `performClose:` gegen einen eigenen Selektor, damit AppKit kein "Close All" mehr dazustellt.
- **S13b** trägt die fünf Kürzel vorher in `resources/default-keymap.toml` ein, weil die Datei dem `ontocoder` gehört.

Die Vermutung dieses Datensatzes, C3 stehe der Sache nicht entgegen, hält, und der Grund steht jetzt in C3 ausgeschrieben: ein Menükürzel und eine Belegung sind zwei Zusteller derselben Taste, und der Fokusvorbehalt aus S13 entscheidet, welcher zum Zug kommt. Die spätere Dateizwischenablage braucht deshalb keine zweite Belegung auf Cmd+C, sondern allein eine Umsetzung von `copy:` und `paste:` am Dateifenster; die Antwortkette liefert sie dorthin, wo der Fokus steht.

Entscheidungsdatensatz `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`. Geschlossen wird dieser Defekt mit der Umsetzung von S13c.

---
Resolved: S13c baut das Menü "Bearbeiten" mit den vier Standardeinträgen Ausschneiden, Kopieren, Einfügen und Alles auswählen, jeweils mit Ziel `nil`, in `crates/krk-ui/src/appkit/menue.rs`. Die vier Kürzel kommen aus `resources/default-keymap.toml`.

Gegenprobe am laufenden Bündel am 260805-0753, über die Bedienungshilfen gesendet und ausgelesen:

- Pfad `/tmp/krk-s13c-nachweis` in der Zwischenablage, `shift+cmd+g`, `cmd+a`, `cmd+v`: das Feld trug danach `/tmp/krk-s13c-nachweis` statt seines Startwerts. Nach Return steht der Ordner in `session.toml` als aktiver Tab des linken Dateifensters.
- `cmd+a` und `cmd+x` im selben Feld: Feld danach leer, Zwischenablage `/tmp/krk-s13c-nachweis`.
- `cmd+a` und `cmd+c`: Zwischenablage `/tmp/krk-s13c-nachweis`, Feld unverändert.
- Während die Schreibmarke im Feld steht, sind alle vier Einträge des Menüs "Bearbeiten" aktiv; steht sie im Dateifenster, sind alle vier grau.

Das Umbenennen-Feld aus S17 und die Blätter aus S16 erben die Behebung, weil sie dieselbe Hülle benutzen und das Menü an der Anwendung hängt, nicht am Blatt.
