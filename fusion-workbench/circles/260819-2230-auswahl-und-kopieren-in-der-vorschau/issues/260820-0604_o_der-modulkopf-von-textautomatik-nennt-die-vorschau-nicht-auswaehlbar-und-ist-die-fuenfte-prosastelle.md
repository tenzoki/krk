Der Modulkopf von `textautomatik.rs` nennt die Vorschau nicht auswählbar — die fünfte Prosastelle, und der Plan führt vier

---

Der Plan (`planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Abschnitt `## Was der Übersetzer einfordert, und was er nicht einfordert`) führt unter „Nichts hält es" vier Prosastellen auf, die diese Runde nachziehen muss: `appkit/ereignisse.rs:141-147`, `appkit/vorschau.rs:104-108`, `appkit/menue.rs:18` und `appkit/vorschau.rs:1093-1099`.

Es gibt eine fünfte. `crates/krk-ui/src/appkit/textautomatik.rs:19-22` sagt im Modulkopf:

> **Die Vorschau ist keine dieser Flaechen und darf es nicht werden.** Sie setzt `setEditable(false)` und `setSelectable(false)`, damit sie den Fokus nicht als Textsystem nimmt; was der Nutzer dort nicht tippen kann, kann auch keine Automatik veraendern.

Mit Schritt 3 ist der Satz an zwei Stellen falsch: `setSelectable(false)` steht dort nicht mehr, und der genannte Grund („damit sie den Fokus nicht als Textsystem nimmt") ist genau die Wirkung, die die Runde bewusst in Kauf nimmt und mit der Anmeldung beim Anwendungsdelegierten bezahlt.

Der **Schluss** des Absatzes trägt weiter und soll stehen bleiben: die Vorschau ist keine bearbeitbare Textfläche, ruft `automatiken_abschalten` nicht und soll es nicht rufen, weil `setEditable(false)` unverändert steht. Zu berichtigen ist allein die Begründung, nicht das Ergebnis.

Die Zählprobe `jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab` in derselben Datei bleibt davon unberührt: sie hängt an `setEditable(true)` und sieht die Vorschau weiterhin nicht.

---

Nicht in Schritt 3 behoben: `textautomatik.rs` liegt außerhalb der Dateiliste jenes Schrittes, und ein zweiter `coder` arbeitete zur selben Zeit im selben Baum. Gehört in Schritt 8, der laut Plan „das Netz darunter" für die Prosastellen ist — mit dieser fünften in der Liste.
