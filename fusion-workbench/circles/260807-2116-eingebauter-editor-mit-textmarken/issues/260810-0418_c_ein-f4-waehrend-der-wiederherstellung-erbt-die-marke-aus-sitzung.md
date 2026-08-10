# Ein F4 während der Wiederherstellung erbt die Marke „aus Sitzung"

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs` (`editor_wiederherstellen:3362`, `editorausgang_behandeln:3401`, `AnwendungsIvars::editor_aus_sitzung:454`), `crates/krk-ui/src/editormodell.rs` (`Editormodell::oeffnen`)
**Cross-references:** `issues/260810-0240_c_ein-oeffnen-im-editor-stoesst-kein-sitzungsschreiben-an.md`, Commit `97891be`, C7

---

## Der Befund

`editor_wiederherstellen` setzt die Marke und startet dann das Lesen:

```rust
self.ivars().editor_aus_sitzung.set(true);
editor.datei_oeffnen(pfad);
```

Gelesen wird seit S24 auf dem Arbeitsfaden. Die Marke wird erst in
`editorausgang_behandeln` mit `replace(false)` verbraucht, also erst, wenn ein
Ladeausgang eintrifft. **Zwischen diesen beiden Zeitpunkten läuft die
Ereignisschleife**, und ein Tastendruck kommt durch.

Drückt der Nutzer in dieser Spanne F4, so ersetzt `Editormodell::oeffnen` den
laufenden Ladevorgang ohne Weiteres:

```rust
self.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf()));
```

Der Ausgang, der danach ankommt, gehört zur Datei des Nutzers, trägt aber die
Marke der Wiederherstellung. Der Zweig `Geoeffnet | SchonOffen` läuft dann in
seiner Startform:

- kein `fokus_holen(Fokus::Editor)` — der Editor bleibt ohne Eingabefokus,
  obwohl das zweite Abnahmekriterium von C2 ihn zusagt;
- kein `titel_nachziehen` — der Fenstertitel nennt weiter die vorige Datei;
- **kein `sitzung_vormerken`** — genau die Fehlwirkung, die `260810-0240` in
  dieser Sitzung behoben hat, kehrt für diese eine Öffnung zurück.

## Wie schmal die Spanne ist

Sie beginnt in `applicationDidFinishLaunching`, sobald `editor_wiederherstellen`
den Faden gestartet hat, und endet mit dem ersten Einzugstakt, der eine Meldung
findet. Für eine kleine Datei auf einer örtlichen Platte sind das
Millisekunden; für eine Datei nahe der 16-MB-Grenze oder auf einem langsamen
Datenträger deutlich mehr. Erreichbar ist sie, häufig ist sie nicht — deshalb
Low und nicht höher.

## Was zu prüfen wäre

Die Marke bezeichnet heute **den Zustand des Delegierten** und nicht **den
Ladevorgang**. Ein Ladevorgang, der seine Herkunft selbst trägt, hätte die
Frage nicht: `Editormodell::oeffnen` bekäme mit, wer geöffnet hat, und der
Ladeausgang brächte die Antwort zurück, statt sie in einer `Cell` neben der
Kette zu hinterlegen. Ob das den Schnitt zwischen `editormodell` und `appkit`
verletzt — das Modell wüsste dann von Befehlen —, ist offen und gehört in die
Antwort.

Der billigere Weg wäre, die Marke beim zweiten Öffnen zu löschen, also in
`Editormodell::oeffnen` bzw. an den drei Befehlswegen. Er ist der schlechtere:
er verteilt eine Zusage auf drei Aufrufstellen, und die erste vergessene fände
keine Prüfung.

## Was heute hält

Kein Textverlust und kein falscher Sitzungsinhalt: geschrieben wird nichts
Falsches, es wird nur nichts geschrieben. Der nächste Befehl schreibt die
Sitzung über den Nachzug in `kommando_ausfuehren` ohnehin.

---
Resolved: Die Marke bezeichnet jetzt **das zuletzt begonnene Öffnen** und nicht
mehr „den Zustand des Delegierten". Gebaut ist damit die Richtung, die dieser
Datensatz als die bessere beschreibt — das Öffnen trägt seine Herkunft —, nur
liegt die Erzwingung im Delegierten und nicht am `Editorbereich`; der Grund und
der noch offene Rest stehen unten.

**Die eine Stelle.** `Anwendungsdelegierter::editor_oeffnen_lassen`
(`crates/krk-ui/src/appkit/anwendung.rs`) ist die einzige Stelle, an der der
Delegierte den Editor eine Datei aufnehmen lässt, und die einzige, die
`AnwendungsIvars::editor_aus_sitzung` schreibt. Sie nimmt die neue Aufzählung
`Oeffnungsherkunft` mit den Werten `Befehl` und `Sitzung` als
**Pflichtargument**. Alle vier Öffnungswege gehen jetzt durch sie:

| Weg | Herkunft |
|---|---|
| `im_editor_oeffnen` (F4) | `Befehl` |
| `editor_aus_vorschau` (`cmd+e`) | `Befehl` |
| `textmarke_anspringen` (C6) | `Befehl` |
| `editor_wiederherstellen` (C7) | `Sitzung` |

Keiner von ihnen ruft `Editorbereich::datei_oeffnen` noch selbst, und keiner
fasst die `Cell` noch an. Der `let Some(editor) = … else` der vier Stellen ist
dabei mitgewandert und steht ebenfalls nur noch einmal.

**Warum das die Frage beantwortet und nicht nur den Fall abfängt.** Höchstens
das zuletzt begonnene Öffnen liefert einen Ladeausgang: `Editormodell::oeffnen`
ersetzt den laufenden Ladevorgang, dessen Empfänger fällt, und das `send` des
überholten Fadens scheitert still. Ein Feld, das bei **jedem** Öffnen gesetzt
wird, beantwortet deshalb am Ladeausgang genau die Frage, die dort gestellt wird
— „wer hat dieses Öffnen verlangt" —, statt sie aus einem älteren Zustand zu
erraten. Der billigere Weg, die Marke an den drei Befehlswegen zu löschen, ist
damit nicht gegangen: es gibt nichts zu löschen und nichts zu vergessen, weil
kein Weg ohne Angabe der Herkunft übersetzt.

**Die Projekteigenschaft über das nicht vorab geleerte Ordnermodell trifft hier
nicht zu**, und deshalb steht kein Vergleich gegen den gehaltenen Bestand in der
Antwort. Der naheliegende Gegenentwurf wäre gewesen, die Marke als Pfad zu
führen und am Ausgang gegen `editordatei()` zu vergleichen. Er scheitert an
`Ladeausgang::Abgewiesen`: eine Abweisung fasst den gehaltenen Stand nicht an,
also nennt das Modell dort weiter die vorige Datei, und der Vergleich hielte
jede Abweisung aus der Sitzung für eine Befehlsantwort — womit gerade das
gefallen wäre, was der Zweig heute richtig macht (Editor ausblenden, Meldung
einen Rang tiefer). Den Pfad aus der `Abweisung` zu holen wäre eine zweite
vollständige Fallunterscheidung über deren drei Varianten, und `Abweisung` hat
keinen Zugriff darauf.

Zwei Restbefunde sind als eigene Datensätze abgelegt:

- `issues/260810-1028_*_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`
  — die Erzwingung endet an der Grenze des Delegierten; ein Aufruf von
  `Editorbereich::datei_oeffnen` von woanders ginge weiter vorbei. Der Weg dahin
  liegt in `appkit/editor.rs`, das in dieser Sitzung außerhalb der Dateigrenze
  lag.
- `issues/260810-1029_*_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`
  — der eine Fall, in dem ein Ladeausgang eintrifft, während ein älteres Lesen
  noch läuft. Für diese Behebung ohne Folgen: beim Start, der einzigen Spanne mit
  der Herkunft `Sitzung`, ist die Abkürzung unerreichbar, weil der Editor dann
  noch keine Datei hält.

Neue Probe: keine. Die Stelle ist ohne Fenster nicht prüfbar —
`Anwendungsdelegierter` braucht AppKit und den Hauptfaden —, dieselbe Grenze, die
`260810-0240` schon festgehalten hat. Was der Übersetzer dafür prüft, ist das
Pflichtargument.

Abnahme am 260810-1030: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` jeweils
`exit 0`.
