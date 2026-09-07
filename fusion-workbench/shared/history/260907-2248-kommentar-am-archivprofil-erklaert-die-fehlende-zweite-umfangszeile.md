# Der Kommentar am Archivprofil erklärt, warum die zweite Umfangszeile fehlt

**Status:** Complete
**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Aufgabe K17. Der Nutzer hat am 260907 zu
`260831-1353_*_bekommt-das-ablageprofil-eine-zweite-umfangszeile-obwohl-die-dateizahl-nicht-zaehlbar-ist.md`
Möglichkeit 1 gewählt: das Profil für `fusion-workbench/archive/` behält seine
eine Zählzeile, und der Kommentar erklärt die Lücke. Zu schreiben war der
erklärende Satz, ohne eine Zahl, die veraltet, und ohne die Zählzeilen selbst
anzufassen.

## Was geschrieben wurde

`resources/default-readers.toml`, Zeilen 501–527, drei Absätze an den bestehenden
Kommentarblock über `[[profil]] "fusion-Werkbank: der Ablagespeicher"` angehängt:

1. Welche zweite Lesart gemeint war (die archivierten Dateien) und warum sie mit
   den vier Bausteinen nicht zählbar ist: die Ablage liegt drei Ebenen tief,
   `zaehlung` läuft flach über eine Ebene, und `*/shared/*` wäre der zweite
   Platzhalter und wird beim Laden abgewiesen. Dazu das Zählkommando außerhalb
   von KRK, `find fusion-workbench/archive -type f | wc -l`.
2. Warum die naheliegende Ersatzzeile `zaehlung = { ordner = "*/shared" }`
   irreführend wäre und nicht bloß ungenau: sie stünde unter „Läufe" und läse
   sich wie deren feinere Auflösung. Die drei Messwerte (5, 15, 167) stehen mit
   dem Datumsstempel 260907 daneben; das Kommando trägt sie fort.
3. Warum die sachlich richtige Antwort, eine Tiefenangabe am Baustein, nicht in
   diese Datei gehört, mit Verweis auf den Entscheidungsdatensatz.

Die zwei Zählzeilen selbst sind unverändert.

## Was geprüft wurde

Das Profil steht noch so da, wie der Datensatz vom 260831 es beschreibt: zwei
Zeilen, „Läufe" (`zaehlung = { }`) und „Zuletzt abgelegt"
(`juengste = { anzahl = 1, zeigt = "datum" }`). Auch die drei Messwerte des
Datensatzes gelten unverändert: fünf Läufe, fünfzehn abgelegte Speicher, 167
Dateien im Unterbaum, keine Datei unmittelbar in `archive/`.

Verification: `cargo test -p krk-core` — exit 0. Die Proben lesen die
Auslieferungsfassung ein und halten ihre Gültigkeit;
`keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` schneidet jede Zeile am
ersten `#` ab, ein Kommentar erreicht sie also nicht.

Kein Nebenbefund. Nichts committet, wie beauftragt; der Datensatz steht weiter
auf `_a_`, weil die `Implemented:`-Zeile einen Commit-Hash zitieren muss, den
erst der Orchestrator erzeugt.
