# Shaper: der Spec der Runde 16

**Datum:** 2026-08-24
**Agent:** shaper (user-direct, aktiver Circle in Bereich)
**Status:** Complete — Spec geschrieben, keine weitere Frage gestellt

## Was eingegeben war

Der Circle-Datensatz `_t_circle.md`, die zwei Klärungsverläufe `260824-0541-shaper-klaerungsrunde-vor-dem-spec.md` und `260824-0600-shaper-zweite-klaerungsrunde-vor-dem-spec.md`, die acht Entscheidungsdatensätze unter `decisions/` (alle auf `_a_`, alle mit `Answered`-Zeile), der Defektdatensatz `issues/260824-0600_o_…-er-fuehrt-eine.md` und der Backlogeintrag `shared/backlog/260823-2136_o_readerconventions-profile-fuer-dateizugriff.md`.

Der Auftrag nannte die drei Antworten der zweiten Runde vom 260824-0610, verlangte die vier abgeleiteten Festlegungen und die zwei Messungen sichtbar im Spec, und benannte den Lieferumfang: prüfbare Abnahmekriterien, den Bausteinsatz einzeln, die Erkennungsregel samt Vorrang ausgeschrieben, das Verhalten ohne Treffer, die zwei Fehlerlagen der `readers.toml`, für L7 ein abzählbares Kriterium ohne Zeitmessung, die sechs Zusammenfassungen als Abnahmefall und eine ausdrückliche Abgrenzung.

## Was entstanden ist

`planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, sechs Fähigkeiten mit 56 Abnahmekriterien:

- **C1** Die Definitionsdatei `readers.toml` als siebte Ablagedatei, acht Kriterien, darunter die vier Fehlerlagen entlang des vorhandenen `Grund`-Wegs.
- **C2** Die Ortserkennung mit dem in drei Schritten ausgeschriebenen Vorrang, acht Kriterien, darunter die Zusage, dass ein Muster aus der `readers.toml` die Vorschau nicht anhalten kann.
- **C3** Der Bausteinsatz, vier Bausteine einzeln benannt und mit vierzehn Kriterien belegt.
- **C4** Die Anzeige im Vorschaufenster, sieben Kriterien.
- **C5** Das mitgelieferte fusion-Profil, zehn Kriterien, je eines für die sechs skizzierten Zusammenfassungen.
- **C6** Die abzählbaren Grenzen an Stelle einer Zeitmessung gegen L7, neun Kriterien.

Dazu ein Mermaid-Bild über den Weg von der Auswahl über die Erkennung zum Bausteinsatz, die vier Constraints des Baums, sechs Punkte Abgrenzung und sieben Fragen an den Plan.

## Die Berichtigung, die der Spec zusammenhält

Zwei der acht Datensätze sind zusammen zu lesen. Der Nutzer hatte am 260824-0555 zur Frage nach den jüngsten zehn die Möglichkeit 2 gewählt, also Sortierung nach Änderungsdatum **und** Titel aus der Überschriftenzeile. Die Messung am Bestand hat danach ergeben, dass die Überschriftenzeile keinen einzigen Defektdatensatz erreicht: 82 Dateien in `shared/issues/` und 157 im größten Speicher eines Circles tragen ihren Titel in der ersten Zeile, aber ohne `#`. Der Nutzer hat die Titelhälfte am 260824-0610 berichtigt und den älteren Datensatz um eine zweite `Answered`-Zeile ergänzt. Der Spec zitiert beide Datensätze nebeneinander und sagt, welcher für welche Hälfte gilt.

## Was ohne Rückfrage entschieden ist

Sieben Festlegungen stehen im Spec unter `## Abgeleitete Festlegungen` beisammen und sind am Spec-Tor überstimmbar. Vier stammen aus der Klärungsrunde vom 260824-0541: erstes passendes Profil in der Datei gewinnt und das Pfadmuster geht der Kennzeichendatei vor; die Zählung läuft flach; die Zusammenfassung entsteht beim Auswählen; sie trägt eine Obergrenze gelesener Einträge. Drei sind neu. Die Obergrenze liegt bei 2.000, also weit über den 157 des größten heutigen Speichers. Die Kopfzeile der Zusammenfassung behält Name und vollen Pfad aus der Metadatenanzeige, weil der Nutzer sonst Zahlen ohne Gegenstand sähe. Und der Zustand eines Circles wird über drei Vorhandensein-Zeilen ausgedrückt, weil er im Dateinamen steht und der feste Bausteinsatz keinen Baustein kennt, der einen Dateinamen liefert; das ist die schwächste der sieben und im Spec als solche ausgewiesen.

## Was am Bestand gemessen ist, Stand 260824-0613 und `278a008`

18 Circle-Verzeichnisse; 82 Dateien in `shared/issues/`, davon 54 offen; 118 in `shared/history/` als größtem gemeinsamen Speicher; 157 in `circles/260802-0842-…/issues/` als größtem eines Circles. Die Wurzel trägt `.fusion-setup` als einzeiliges JSON mit `setup_at`, `setup_pwd` und `plugin_version`, `.active-circle` als nackte Zeile und `orchestrator-live.md`; `agentstate.yaml` fehlt weiter. `Datei::ALLE` führt sechs Einträge, `readers.toml` wird an drei Stellen die siebte. Die Ausdrucksmaschinerie steht über `syntect` schon im Bündel, und `Cargo.lock` führt kein `cc` und außer `windows-sys` kein `-sys`-Paket.

## Nächster Schritt

Der Nutzer liest den Spec und entscheidet am Tor über die sieben abgeleiteten Festlegungen. Danach ist der Planner zu beauftragen; der Plan entsteht unter `planning/` dieses Circles. Der Shaper hat den Circle-Datensatz nicht angefasst: das Kopffeld `**Active spec/plan:**` setzt der Orchestrator.
