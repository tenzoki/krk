# Schritt 4: Die Anmeldung im Ereignisabgriff — aus einer Fläche werden zwei

**Agent:** coder
**Datum:** 2026-08-20
**Plan:** `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel B, Schritt 4
**Status:** Complete

---

## Was der Schritt behebt

Seit Schritt 3 ist die Textanzeige der Vorschau auswählbar und nimmt damit den Ersthelferrang. `ersthelfer_gehoert_appkit` meldete dafür `true`, und Bestandteil (2) der Zulässigkeitsregel wies mit dem Fokus in der Vorschau **jeden** Befehl von KRK ab — die vier Tabbefehle aus C1 der Runde 2 eingeschlossen, die C1.6 des Specs unverändert verlangt. Der Schritt stellt den Zustand von vor Schritt 3 wieder her, und zwar an der Stelle, an der die Frage schon beantwortet wird.

## Was gebaut ist

Vier Dateien berührt.

**`crates/krk-ui/src/appkit/vorschau.rs`** — `Vorschaufenster::textflaeche(&self) -> &NSTextView` als Zugang, wortgleich zu `Editorbereich::textflaeche`. Der engere Typ der Unterklasse geht nicht mit hinaus: verglichen wird ein Objektzeiger, und dafür trägt `NSTextView` alles Nötige. Keine neu angesprochene AppKit-Klasse, also kein Zuwachs im Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`; `NSTextView` steht dort seit Schritt 3.

**`crates/krk-ui/src/appkit/anwendung.rs`** — `ist_editorflaeche` heißt jetzt `ist_eigene_textflaeche` und fragt zwei `isEqual`-Vergleiche in **einer** Funktion: den gegen die Textfläche des Editors und den gegen die Textanzeige der Vorschau. Beide gehen weiterhin über `get` und nicht über `expect`, weil die Reihenfolge im Aufbau der Oberfläche keine Zusage dieser Funktion ist. Die drei übrigen Nennungen des alten Namens in derselben Datei ziehen mit.

**`crates/krk-ui/src/appkit/ereignisse.rs`** — der Parametername zieht mit, `ist_eigene_textflaeche`. **Der Abschluss bleibt einer, der Parameter bleibt einer**, und keine Liste entsteht in dieser Datei: sie kennt weder den Editor noch die Vorschau und soll beide nicht kennenlernen.

**`crates/krk-ui/src/kommandos/zulaessigkeit.rs`** — die beiden neuen Proben, siehe unten.

## Warum die Menge beim Delegierten steht

Das ist die Antwort auf den vierten offenen Punkt des Specs, und der Plan hat sie getroffen. Zwei Formen waren möglich und sind beide nicht gebaut:

- **Ein zweiter Parameter** an `ersthelfer_gehoert_appkit`. Dann wüchse die Signatur mit jeder eigenen Textfläche, und die Datei, die die Frage nur stellt, führte Buch über die Antworten.
- **Eine Liste im Abgriff**. Dann entstünde dort eine Menge, die der Delegierte füllt, und die Frage stünde an zwei Stellen.

Gebaut ist die dritte: ein Abschluss, ein Parameter, zwei Vergleiche im Rumpf beim Delegierten. Eine dritte eigene Fläche wäre ein dritter Vergleich dort und keine Änderung im Abgriff.

## Die Fallunterscheidung, die der Doc-Kommentar ausschreibt

Der Doc-Kommentar an `ist_eigene_textflaeche` nennt beide Flächen einzeln und sagt, warum die Fläche eines **Blattes** weiterhin nicht angemeldet wird: dort ist erwünscht, dass die Tasten AppKit gehören. Nur solange ihr Ersthelfer AppKit gehört, bleibt `Kommando::Abbrechen` unzulässig, läuft der Tastendruck unverändert weiter, und `Esc` schließt den Notizzettel (`appkit/blaetter/zettel.rs`, Modulkopf). Die Warnung in `CLAUDE.md` — „wer eine zweite bedienbare Textfläche baut, meldet sie dort an" — zeigt für ein Blatt in die falsche Richtung; wer sie ohne diese Unterscheidung liest, meldet die falsche Fläche an.

## Die vier Prosastellen im Modulkopf von `ereignisse.rs`

Drei sind nachgezogen, eine steht ausdrücklich unverändert.

1. „Die eine Ausnahme ist die Textfläche des Editors" → „Die Ausnahmen sind die eigenen Textflächen von KRK", mit beiden benannt.
2. „eine Liste von Ausnahmen entsteht nirgends" → die genauere Aussage: die Menge steht beim Anwendungsdelegierten, und in **dieser** Datei entsteht keine. Die alte Formulierung wäre nach diesem Schritt schlicht falsch gewesen.
3. „Der Abgriff kennt den Editor nicht" → „kennt weder den Editor noch die Vorschau", mit dem Zusatz, dass eine dritte eigene Fläche ein dritter Vergleich beim Delegierten wäre.
4. **Unverändert:** der Absatz, der die Nämlichkeit gegen die Art begründet. Er trägt wörtlich weiter, und sein Beispiel — der Feldeditor eines Textfeldes ist dieselbe Klasse wie die Textfläche des Editors — ist mit der zweiten Fläche nicht schwächer geworden, sondern stärker.

Dazu ein neuer Absatz im Modulkopf, der das Blatt als ausdrückliche Nicht-Ausnahme führt, und zwei Sätze am Doc-Kommentar von `ersthelfer_gehoert_appkit`, dass der Abschluss und nicht diese Datei entscheidet, welche Flächen dazugehören.

## Die Proben

**Bestehend und grün geblieben:** `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` (`appkit/ereignisse.rs`). Ihre Nadel ist `concat!("fn ", "ersthelfer_gehoert_appkit")` und trifft die Erklärung, nicht den Parameternamen; die Umbenennung geht an ihr vorbei, und die Klassenprüfung steht weiterhin allein in dieser Datei.

**Neu, beide in `kommandos/zulaessigkeit.rs`, beide auf der vorhandenen `Lage` gerechnet, ohne Fenster:**

- `die_vier_tabbefehle_wirken_mit_dem_fokus_in_der_vorschau` (C1.6, Probenhälfte). Sie prüft `TabNeu`, `TabSchliessen`, `TabNaechster` und `TabVoriger` in der Lage, die die Anmeldung herstellt: kein Blatt, KRKs eigenes Schlüsselfenster, `Fokus::Vorschau`, Ersthelferbefund `false`. **Die zweite Zusicherung ist die eigentliche Aussage**: dieselben vier sind mit Ersthelferbefund `true` abgewiesen. Ohne sie bliebe offen, ob die Anmeldung überhaupt den Unterschied macht; mit ihr wird die Probe rot, wenn die Anmeldung wieder fällt. Bei `TabSchliessen` trägt das der vierte Bestandteil: der Befehl hat `Wirkungsbereich::Ueberall`, steht aber nicht auf der Ausnahmeliste `immer_erreichbar`.
- `die_beiden_pfeiltasten_bleiben_in_der_vorschau_zulaessig` (C1.10, Probenhälfte für die Zulässigkeit). `AuswahlHoch` und `AuswahlRunter` tragen `Wirkungsbereich::Navigator`, der die Vorschau seit der Runde 1 mitführt. Zulässig heißt hier ausdrücklich nicht „bewegt etwas": beide werden entgegengenommen, von der Vorschau nicht ausgeführt und erreichen AppKit deshalb nicht. Wären sie unzulässig, liefen sie weiter, und die Schreibmarke der auswählbaren Textanzeige begänne zu wandern. Der Verbrauch ist die andere Hälfte und wird am Bündel abgenommen.

Keine neue Probe baut eine `NSTextView` oder behauptet den Hauptfaden; `krk-ui` hat kein Bibliotheksziel, und der Griff `MainThreadMarker::new_unchecked` ist der bekannte Defekt `issues/260810-1001_*`.

## Was der Schritt nicht angefasst hat

- `crates/krk-ui/src/appkit/textautomatik.rs`. Der offene Datensatz `issues/260820-0604_o_der-modulkopf-von-textautomatik-nennt-die-vorschau-nicht-auswaehlbar-…` gehört Schritt 8.
- Die Zählprobe über `fn ist_eigene_textflaeche` (C1.7). Sie steht erst, wenn alles gebaut ist, und gehört Schritt 8.
- Die Belegung, die vier gewachsenen Aufzählungen und jede fremde Kiste. Kein Zuwachs, wie C4.1 und C4.6 es zusagen.

## Prüfung

`make check` (`cargo build`, `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`), Rückgabewert 0, alle vier grün. 728 Proben in `krk-ui` durchgelaufen, darunter die drei oben genannten.

**Nicht committet** — das tut der Orchestrator.
