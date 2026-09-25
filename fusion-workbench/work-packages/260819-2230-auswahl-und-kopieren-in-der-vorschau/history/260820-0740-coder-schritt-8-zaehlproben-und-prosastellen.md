# Schritt 8: Was nur zusammen zu zählen ist, und die Prosastellen

**Agent:** coder
**Datum:** 2026-08-20
**Plan:** `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel D, Schritt 8
**Status:** Complete

---

## Was der Schritt einlöst

Das Netz unter der Runde. Er baut nichts, was die Anwendung tut, und hält zwei Sorten Zusagen fest, die sonst niemand hält: die Zählproben über Stellen, die erst nachzumessen sind, wenn alles steht, und die Prosastellen, an denen der Baum etwas über sich selbst behauptet, das diese Runde falsch gemacht hat.

## Die Zählproben: zwei neue, eine vorgefundene, eine unberührte

Der Plan nennt drei. **Eine davon stand schon.** Schritt 7 hat `die_abfangstelle_steht_im_baum_genau_einmal` (`crates/krk-ui/src/appkit/vorschau.rs`, Prüfmodul) mitgeliefert, weil sein eigener Probenauftrag sie ebenfalls verlangte; sie ist nachgeprüft und grün. Eine zweite daneben wäre der Doppelbau gewesen, gegen den diese Proben stehen.

**Neu ist `die_menge_der_eigenen_textflaechen_steht_an_genau_einer_stelle`** (`crates/krk-ui/src/appkit/ereignisse.rs`, Prüfmodul, C1.7). Sie steht neben `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` und zählt die andere Hälfte derselben Frage: jene hält fest, dass der Abgriff die Frage genau einmal stellt, diese, dass genau eine Stelle sie beantwortet. Bis zu dieser Runde war die Antwort ein einzelner Vergleich und brauchte keine Probe; mit der zweiten eigenen Textfläche ist sie eine Aufzählung geworden, und eine Aufzählung an zwei Stellen wüsste von der dritten Fläche nichts. Erwartet wird die Datei mit dabei, `krk-ui/src/appkit/anwendung.rs`: eine Wanderung nach `ereignisse.rs` ließe die Zahl bei eins und wäre trotzdem die verworfene Bauart.

**Neu ist `die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei`** (`crates/krk-ui/src/appkit/zwischenablage.rs`, Prüfmodul, C2.10 und erste Hälfte von C4.7). Zwei Nadeln, weil die Hülle zwei Hälften hat, die einzeln abwandern könnten: das Schreiben in eine Ablage und der Griff nach der Ablage des angemeldeten Nutzers. Der Anlass ist die Runde selbst — sie legt mit `Vorschautext::auswahl_ablegen` einen Rufer aus einer ganz anderen Ecke an, und der naheliegende falsche Weg wäre gewesen, dort selbst zu schreiben.

**Erwartet wird dort die Datei und nicht die Zahl der Fundstellen**, und das ist eine bewusste Wahl gegen die Gestalt, die `260820-0646_o_…` beschreibt: der Griff nach der allgemeinen Ablage steht heute an drei Zeilen und morgen vielleicht an zwei, ohne dass sich an der Zusage etwas ändert. Die Zusage ist eine über den Ort.

**Für die drei Prüfordner-Fassungen ist keine Zeile entstanden.** `genau_drei_pruefordner_fassungen_stehen_im_baum` (`crates/krk-core/tests/baum.rs:114`) misst sie seit der Runde 1 und ist im Lauf dieses Schrittes grün; damit ist die zweite Hälfte von C4.7 ohne Arbeit eingelöst.

Alle Nadeln stehen über `concat!` zusammengesetzt da, und beide neuen Erwartungen sind vor dem Schreiben am Baum erhoben worden: `fn ist_eigene_textflaeche` steht einmal, `setString_forType` und `generalPasteboard` stehen als Code allein in `zwischenablage.rs`. Beide trafen zu; anders als bei den drei Fällen des Befunds `260820-0646_o_…` war nichts zu berichtigen.

## Die Prosastellen: der Plan führt vier, es sind neun

Die vier des Plans hatten die Schritte 3, 4 und 7 schon mitgezogen. Nachgelesen und unverändert gelassen: `appkit/ereignisse.rs` (Modulkopf, „die Ausnahmen sind die eigenen Textflächen von KRK"), `appkit/vorschau.rs` (Modulkopf und Doc-Kommentar von `textanzeige`, beide mit der ausdrücklichen Aussage, dass C4.8 der Runde 6 ersetzt und nicht ergänzt ist), `appkit/menue.rs` (Modulkopf, `copy:` erreicht drei Flächen). Der Schritt ist das Netz und nicht der erste Ort; was schon stimmte, blieb.

**Fünf waren nachzuziehen.** Eine lag als Befund vor (`260820-0604_c_…`, `appkit/textautomatik.rs`), vier sind bei diesem Durchgang aufgefallen und in `260820-0731_c_…` festgehalten:

| Datei | Was zu berichtigen war |
|---|---|
| `appkit/textautomatik.rs` | die Vorschau setze `setSelectable(false)`, „damit sie den Fokus nicht als Textsystem nimmt" — beide Hälften falsch; der Schluss des Absatzes trägt weiter und steht unverändert |
| `appkit/editor.rs` | dieselbe Aussage an einer zweiten Datei, im Doc-Kommentar von `textflaeche_bauen` |
| `kommandos/zulaessigkeit.rs` | am Feld `Lage::ersthelfer_gehoert_appkit`: „die eine Ausnahme" — es sind zwei |
| `appkit/menue.rs` | im Prüfmodul: `NSTextView` stehe zweimal im Programm — es sind drei, und die Klassenliste bleibt trotzdem bei sechs |
| `kommandos/fokus.rs` | zu `Fokus::Vorschau`: der Fokus komme „per Mausklick in die Inhaltsfläche" — seit Schritt 5 nimmt auch die Textanzeige den Rang |

Jede Berichtigung nennt die Runde und lässt den tragenden Teil der alten Aussage stehen. Die letzte ist eine Unvollständigkeit und keine Falschaussage; sie steht mit dabei, weil sie in dieselbe Richtung irreführt.

**Der Befund über den Plan gehört dazu.** Neun statt vier heißt, dass die Erhebung weniger als die Hälfte gefunden hat. Der wahrscheinliche Grund steht in `260820-0731_c_…`: die vier des Plans liegen alle in Dateien, die die Runde ohnehin anfasst, und drei der vier neuen sagen dasselbe wie eine davon, nur anderswo. Eine Erhebung, die von den geänderten Dateien ausgeht, findet sie nicht.

## Was der Schritt nicht getan hat

- **Keine Markerwanderung an den sieben Entscheidungsdatensätzen** unter `shared/decisions/260819-2216_a_*.md`. Das gehört dem Rundenabschluss.
- **Keine Berichtigung an `CLAUDE.md`.** Die Runde hat dort Spuren zu hinterlassen, aber in einem eigenen Durchgang.
- **Kein Vorgriff auf die Bündelabnahme.** Fünfzehn Kriterien sind nur am laufenden `KRK.app` im Vordergrund zu sehen, und das ist Nutzerarbeit.
- **Nicht committet.** Das tut der Orchestrator.

## Berührte Dateien

- `crates/krk-ui/src/appkit/ereignisse.rs` — die neue Zählprobe zu C1.7
- `crates/krk-ui/src/appkit/zwischenablage.rs` — die neue Zählprobe zu C2.10 und C4.7, dazu der Import von `quelldateien`
- `crates/krk-ui/src/appkit/textautomatik.rs` — die fünfte Prosastelle
- `crates/krk-ui/src/appkit/editor.rs` — die sechste
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs` — die siebte
- `crates/krk-ui/src/appkit/menue.rs` — die achte
- `crates/krk-ui/src/kommandos/fokus.rs` — die neunte

Die drei zuletzt genannten Dateien und `editor.rs` stehen nicht in der Dateiliste des Plans für diesen Schritt. Die Abweichung ist bewusst: der Schritt ist als Netz unter den Prosastellen beschrieben, und die Aussagen stehen dort, wo sie stehen.

## Prüfung

`make check`, alle vier Kommandos grün, Rückgabewert 0. Die beiden neuen Proben laufen mit, `die_abfangstelle_steht_im_baum_genau_einmal`, `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` und `genau_drei_pruefordner_fassungen_stehen_im_baum` bleiben grün.
