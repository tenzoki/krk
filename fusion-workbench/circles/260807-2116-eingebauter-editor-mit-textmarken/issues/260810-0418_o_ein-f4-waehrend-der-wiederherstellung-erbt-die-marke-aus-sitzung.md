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
