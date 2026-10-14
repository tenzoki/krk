Ein gerendertes Markdown behält nach dem Wechsel auf Dunkel die Verweisfarbe der hellen Tafel

---

Seit Planschritt 9 zieht die Vorschau ihre Farbtafel nach, wenn das System auf
Hell oder Dunkel umstellt: `Vorschaufenster::erscheinung_nachziehen`
(`crates/krk-ui/src/appkit/vorschau.rs`) setzt die neue Tafel und fordert die
Einfärbung neu an. Für Quelltext ist die Sache damit erledigt — der
Einfärbungslauf rechnet mit der neuen Tafel.

**Für gerendertes Markdown nicht.** Die Farbe eines Verweises entsteht in
`markdown::rendern` und damit auf dem Arbeitsfaden des Modells, beim Lesen der
Datei; sie steht als `Einfaerbung` im `Inhalt::Markdown` des Tabs. Ein Wechsel
des Erscheinungsbildes rührt diesen Wert nicht an. Ein Markdown-Tab trägt
danach die Verweisfarbe der Tafel, mit der er gerendert wurde, bis er seine
Datei erneut anzeigt.

Alles Übrige an ihm wechselt mit: Schrift, Einzug und die Fließtextfarbe kommen
aus dem System (`NSColor::textColor`), und die löst sich in beiden
Erscheinungsbildern gegen den Grund auf.

---

**Wie schwer es wiegt**

Sichtbar ist ein Verweis in einer Farbe, die für den neuen Hintergrund nicht
gewählt wurde. Die beiden Tafeln des Vorgabesatzes liefern dafür (208, 135, 112)
in Hell und (235, 203, 139) in Dunkel; beide sind auf beiden Gründen lesbar, es
ist also kein unsichtbarer Text, sondern eine falsche Farbe. Betroffen ist
allein der Zeitraum zwischen dem Wechsel und der nächsten Anzeige derselben
Datei, und der Wechsel geschieht selten.

**Warum Schritt 9 es nicht behoben hat**

Der Plan verlangt an dieser Stelle „die Tafel nachziehen und neu anfordern", und
genau das ist gebaut. Das Nachziehen der Markdown-Farbe ist keine Anforderung
mehr, sondern ein zweiter Lesevorgang: die Datei müsste noch einmal von der
Platte gelesen und noch einmal zerlegt werden, und zwar **in jedem** Tab, der
Markdown zeigt, nicht nur im aktiven. Das ist ein Zuschnitt und kein Handgriff,
und Schritt 9 hätte ihn nebenbei entschieden.

**Was zu tun ist**

Zuerst zu entscheiden, dann zu bauen. Drei Zuschnitte sind erkennbar, keiner ist
hier gewählt:

1. **So lassen.** Der Fall ist selten und die Folge gering; der Kommentar an
   `erscheinung_nachziehen` sagt sie bereits aus. Dann ist dieser Datensatz als
   Lage angenommen zu schließen.
2. **Den aktiven Tab neu laden.** `erscheinung_nachziehen` ruft
   `datei_anzeigen` mit dem angezeigten Pfad, wenn der aktive Tab Markdown
   zeigt. Ein Lesevorgang, dieselbe Bauart wie jede andere Anzeige — aber die
   Datei kann sich inzwischen geändert haben, und der Nutzer sähe nach einem
   Wechsel des Erscheinungsbildes einen anderen Text als davor. Die inaktiven
   Tabs bleiben stehen.
3. **Die Farbe aus der Formatierung herausnehmen.** Ein Verweis bekäme nicht
   die Farbe der Tafel mitgeliefert, sondern eine Kennzeichnung „Verweis", und
   `textmerkmale::anwenden` schlüge die Farbe beim Setzen nach. Dann zieht jeder
   Tab mit, ohne dass eine Datei zweimal gelesen wird. Das kostet einen neuen
   Wert in `Auszeichnung` oder `Einfaerbung` und berührt damit die Schnittstelle
   zwischen `hervorhebung.rs`, `markdown.rs` und `textmerkmale.rs`.

**Kontext**

Gefunden und benannt beim Bau von Planschritt 9; die Stelle im Code trägt den
Sachverhalt im Doc-Kommentar von `Vorschaufenster::erscheinung_nachziehen`.
Herkunft: Circle der Runde 6, weil die Sache mit C4 dieser Runde entsteht.
