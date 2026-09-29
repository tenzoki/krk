Die Auslieferungsfassung von settings.toml sagt, das Zurücksetzen nenne bei einem Verweis die Zeile zum Eintragen von Hand
---
Der Kopf von `resources/default-settings.toml` behauptet für beide Schreibwege, dass KRK bei einem symbolischen Verweis „die Zeile zum Eintragen von Hand“ nennt. Das Zurücksetzen tut das nicht; nur „Ort wählen…“ tut es. Der Kommentar reist wörtlich in jede neu angelegte oder zurückgesetzte `settings.toml` des Nutzers.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>

## Befund

- `resources/default-settings.toml`, Kopfkommentar (Zeilen 6 bis 15 im Stand `52be37a`): „Ist die Datei ein symbolischer Verweis, schreibt KRK sie auf keinem der zwei Wege und nennt die Zeile zum Eintragen von Hand.“
- `crates/krk-core/src/ablage/werkszustand.rs`, `Werkshindernis::meldung`, Zweig `Verweis`: „{datei} ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch eine Datei; nichts ist zurückgesetzt.“ Keine Zeile zum Eintragen.
- `crates/krk-core/src/ablage/einstellungen.rs`, `Schreibhindernis::meldung`, Zweig `Verweis`: nennt die Zeile, allein für „Ort wählen…“.
- `README.md`, Abschnitt `## Neuerungen an den eigenen Dateien übernehmen`, sagt es richtig: „bei „Ort wählen…“ nennt die Statuszeile im zweiten Fall die Zeile“.

## Richtung

Den Kommentar in `resources/default-settings.toml` auf den Wortlaut der `README.md` bringen: auf keinem der zwei Wege geschrieben, die Zeile zum Eintragen nennt allein „Ort wählen…“. Nur Kommentarzeilen, kein Wert.

## Abnahme

- Der Kopf von `resources/default-settings.toml` schreibt die Zeile zum Eintragen allein „Ort wählen…“ zu.
- `git diff` zeigt allein Zeilen mit `#`; `die_auslieferungsfassung_traegt_ihre_kommentare` und `die_eingebettete_fassung_besteht_ihre_eigene_pruefung` bleiben grün.

---
Resolved: Der Kopf von `resources/default-settings.toml` schreibt die Zeile zum Eintragen allein „Ort wählen…“ zu; das Zurücksetzen bricht bei einem Verweis ab und nennt allein die Datei.
