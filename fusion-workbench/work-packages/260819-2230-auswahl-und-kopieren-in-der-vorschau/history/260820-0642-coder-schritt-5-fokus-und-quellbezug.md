# Schritt 5: Der Fokus zeigt auf die Textanzeige, und der Quellbezug kommt mit dem Inhalt

**Agent:** coder
**Datum:** 2026-08-20
**Plan:** `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel B, Schritt 5
**Status:** Complete

---

## Was der Schritt behebt

Zwei Lücken, die Schritt 3 hinterlassen hat.

Die erste: `Vorschaufenster::fokusansicht` lieferte unverändert die Inhaltsfläche. Wer den Fokus über einen Befehl holt statt mit der Maus, landete damit auf der Fläche **unter** dem Text, und `cmd+a` und `cmd+c` hätten dort nichts zu greifen gehabt. C1.8 verlangt das Ergebnis und lässt den Weg offen; gebaut ist der Weg innerhalb der einen Zuordnung.

Die zweite: der Merkposten `Vorschautext`-Quellbezug stand seit Schritt 3 da, und niemand füllte ihn. Beide Zugänge trugen deshalb `#[expect(dead_code, …)]`. Der Setzer hat jetzt seine zwei Rufer, und seine Zeile ist gefallen; der Leser behält seine bis Schritt 7, weil sein Rufer die Überschreibung ist.

## Was gebaut ist

Eine Datei berührt: `crates/krk-ui/src/appkit/vorschau.rs`.

**`fokusansicht` beantwortet ab jetzt eine Frage mit.** Steht die Bildlaufansicht, ist die Antwort die Textanzeige; sonst bleibt es bei der Inhaltsfläche und damit beim heutigen Verhalten für ein Bild. Gefragt wird `NSScrollView::isHidden`.

**Der Quellbezug hat je einen Ort für das Setzen und für das Löschen.** `text_zeigen` ruft `quellbezug_setzen(None)`, unmittelbar neben `textmerkmale::zuruecksetzen`; der Markdown-Zweig von `anzeigen` ruft `quellbezug_setzen(Some(…))`, unmittelbar neben `formatierung_anwenden`. Das ist die Symmetrie, die zwischen Rücknahme und Anwendung der Textmerkmale schon besteht, und keine dritte Stelle daneben.

Der `Arc` wandert dabei und wird nicht geklont: `inhalt` in `anzeigen` ist der Klon des aktiven Tabs und gehört der Funktion; der Quellbezug im Modell bleibt unberührt.

## Warum die Verzweigung nach der sichtbaren Anzeige fragt

Nicht danach, was der Tab zeigt, und der Grund steht im Doc-Kommentar, weil er sonst nirgends steht. `Anwendungsdelegierter::fokusansicht` liefert die Ansicht nicht nur als Ersthelfer, sondern seit C1 der Runde 6 auch als **Anker** für den Freigabedialog (`anwendung.rs:2165-2172`): ein solcher Dialog hängt sich an eine Fläche und an deren Rechteck. Eine ausgeblendete Ansicht taugt für keines von beidem. Sie nimmt den Ersthelferrang nicht an, und ein Anker ohne sichtbares Rechteck setzt den Dialog ins Nichts.

Die zwei Zweige sind vollständig und überschneidungsfrei, weil `text_zeigen` und `bild_zeigen` die beiden Schalter immer gegenläufig setzen. Auch der Fehlschlagpfad von `bild_zeigen` fällt auf `text_zeigen` zurück und lässt keinen dritten Zustand entstehen.

**Es bleibt bei einer Zuordnung von Fokuswert auf Ansicht** (C1.8). Die Verzweigung steht innerhalb davon; eine zweite Zuordnung daneben wären zwei Wahrheiten darüber, welche Fläche zu `Fokus::Vorschau` gehört.

## C1.13 fällt heraus, ohne dass eine Regel dafür entsteht

Die Zusage lautet: die Auswahl fällt mit jedem Inhaltswechsel. Beide Hälften folgen aus dem, was ohnehin dasteht, und der Doc-Kommentar von `text_zeigen` schreibt es aus, statt einen Mechanismus dafür zu bauen.

- Jeder Inhaltswechsel läuft über `text_zeigen` — ein Tabwechsel, eine andere Datei, ein neuer Lesevorgang —, also fällt der Quellbezug mit ihm.
- `setString:` ersetzt den Textspeicher **ganz**, also lässt AppKit die sichtbare Auswahl von sich aus fallen.

Eine Auswahl je Tab zu merken, wäre die Alternative gewesen; sie hätte `Tabinhalt` ein Feld gekostet und eine Auswahl einen Lesevorgang überleben lassen, der ihren Text ausgetauscht hat.

## Die zwei Proben

Beide sind Zählproben über `crate::quellbaum`, beide mit zusammengesetzten Nadeln, damit keine sich selbst findet.

**`der_quellbezug_wird_an_genau_zwei_stellen_gesetzt`.** Eine Aufruferzählung über `quellbaum::aufrufstellen`, und sie steht hier zu Recht: der Kopf jenes Moduls lässt eine solche Zählung dort zu, wo ein Abnahmekriterium die Zahl selbst zusagt, und C1.13 tut das. Erwartet ist genau eine Datei mit genau zwei Rufern. Die zweite Hälfte prüft die beiden Formen einzeln, `…(None)` und `…(Some(`: die bloße Zahl zwei ließe zu, dass beide Rufer setzen und keiner zurücknimmt, und dann trüge eine Auswahl im rohen Text der nächsten Datei den Quelltext der vorigen Markdown-Datei.

**`die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal`.** Eine Erklärungszählung über `concat!("fn ", "fokusansicht")`. Sie erwartet **zwei** Fundstellen und nicht eine: `Anwendungsdelegierter::fokusansicht` trägt denselben Namen und beantwortet die andere Hälfte derselben Frage, nämlich welcher Fokuswert welchem Bereich gehört. Der Plan spricht an dieser Stelle nur von `vorschau.rs`; eine Erwartung, die den Delegierten nicht mitführt, wäre von Anfang an rot gewesen. Die Probe schreibt beide Stellen mit ihrer Zahl aus, wie es der Befund `issues/260820-0604_c_die-zaehlprobe-aus-schritt-3-kann-nicht-null-erwarten-…` für die Schwesterprobe verlangt hat.

Keine der beiden baut eine `NSTextView` oder behauptet den Hauptfaden; `krk-ui` hat kein Bibliotheksziel, und `MainThreadMarker::new_unchecked` ist der bekannte Defekt `issues/260810-1001_*`.

## Die Angabe zur Untergrenze

Eine Berührung ist neu und steht jetzt im Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`: `isHidden`, der Leser der Eigenschaft `hidden` von `NSView`. Am SDK gelesen unter `NSView.h:92`; die Eigenschaft trägt dort keine Verfügbarkeitsangabe und steht damit seit 10.0. Ihr Setzer `setHidden:` wird in dieser Datei seit der Runde 1 gerufen.

## Was der Schritt nicht angefasst hat

- Die Überschreibung `writeSelectionToPasteboard:types:` und den Leser `quellbezug`. Beide gehören Schritt 7; das `#[expect(dead_code, …)]` am Leser bleibt deshalb stehen.
- `crates/krk-ui/src/appkit/textautomatik.rs`. Der offene Datensatz dazu gehört Schritt 8.
- Die Belegung, die vier gewachsenen Aufzählungen und jede fremde Kiste. Kein Zuwachs, wie C4.1 und C4.6 es zusagen.
- Die Markerwanderung der Entscheidungsdatensätze. Sie gehört dem Abschluss der Runde.

## Prüfung

`make check` (`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`), Rückgabewert 0, alle vier grün. 730 Proben in `krk-ui` durchgelaufen, darunter die zwei neuen.

**Nicht committet** — das tut der Orchestrator.
