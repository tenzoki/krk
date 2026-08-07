# Portfolio

**Generated:** 260807-2125 (by playmaker session 260807-2125-playmaker-direct-dispatch)
**Domain bias:** code

## Active (_t_)

(keiner)

Kein Circle-Datensatz trägt die Marke `_t_` für aktiv, und `fusion-workbench/.active-circle` fehlt. Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Störung. Die Runde 1 wurde am 260807-1035 mit beschränktem Abschluss geschlossen, und der Zeiger wurde dabei gelöscht.

## Anticipated (_a_) — ranked

**Recommended next:** `260807-2116-eingebauter-editor-mit-textmarken` — der Nutzer hat den Editor am 260807-1930 als nächste Runde gewählt und den Circle dafür am 260807-2116 anlegen lassen.

**Die Rangfolge hat seit dem letzten Lauf ein zweites Element**, und die beiden Signale zeigen in verschiedene Richtungen. Die Gewichtung `code` bevorzugt vorgesehene Circles mit wenigen unbeantworteten Fragen; nach diesem Maß läge der Web-Betrachter vorn, mit einem zitierten offenen Entscheidungsdatensatz gegen vier beim Editor. Die Übergabe `shared/history/260807-1930-uebergabe-an-die-editor-runde.md` hält dagegen eine ausdrückliche Wahl des Nutzers fest: der Editor ist die nächste Runde, und vom Web-Betrachter sagt dasselbe Dokument, er sei "nicht der gewählte nächste Schritt". Eine festgehaltene Wahl beantwortet die Frage, welcher Circle gewollt ist; der Zählwert beantwortet nur, welcher weniger Klärung braucht. Die Rangfolge folgt der Wahl und benennt den Zählwert daneben.

### 1. `260807-2116-eingebauter-editor-mit-textmarken`

**Directive:** KRK öffnet Text, Code und Markdown in einem eingebauten Editor, der als vierter Fokusbereich neben Lesezeichenleiste, den beiden Dateifenstern und dem Vorschaufenster steht und über die freigehaltene Taste F4 erreichbar ist. Er trägt eine Rohansicht und eine Formatansicht, springt zu einer Zeilennummer, sucht und ersetzt innerhalb der geöffneten Datei und setzt Marken auf Textstellen, die als Lesezeichen neben den Ordnermarken in `bookmarks.toml` stehen.

**Abhängigkeiten:** eine, `260802-0842-krk-mac-dateimanager-editor-git`, beschränkt abgeschlossen (`_b_`) am 260807-1035. Damit ist die Vorbedingung nach der Rangheuristik gekennzeichnet und nicht erfüllt, denn als erfüllt zählt allein der kohärente Abschluss (`_c_`).

**Warum er vorn steht.** Der Nutzer hat die Reihenfolge selbst festgelegt, und zwar zweimal an einem Tag: in der Übergabe vom 260807-1930 als Aussage und am 260807-2116 als Handlung, mit dem Anlegen dieses Circles über `/fusion:direct`. Die sechs Bauteile, die der Circle laut seinem Grounding von der Runde 1 erbt, liegen auf der Platte und sind am Code geprüft: die F4-Reservierung in `resources/default-keymap.toml:130-137`, die Bereichsaufzählung in `crates/krk-ui/src/fenstermodell.rs:48-70`, die Lesezeichenliste in `crates/krk-core/src/ablage/lesezeichen.rs`, die vier Ablagedateien in `crates/krk-core/src/ablage/pfade.rs`, die Aufteilung der Fensterzeile in `crates/krk-ui/src/appkit/aufteilung.rs` und die Statuszeile in `crates/krk-ui/src/appkit/statuszeile.rs`.

**Was er an Klärung mitbringt.** Vier offene Entscheidungsdatensätze stehen in seinem Grounding, und nur einer bindet vor dem ersten Planschritt: `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` fragt, was die Formatansicht bei Text, bei Code und bei Markdown zeigt, führt drei Möglichkeiten und empfiehlt die dritte, eine schreibgeschützte Leseansicht für alle drei Dateitypen. Die drei anderen ordnet der Circle selbst als zu prüfen ein: die Verfügbarkeitsprüfung für macOS-26-Schnittstellen, der Auffrischungsaufschub und das Überleben der Markierung. Dazu kommen vier eigene Fragen für die Klärungsrunde bei der Aktivierung, von der Herkunft der Datei über den Umgang mit ungespeicherten Änderungen bis zu der Frage, ob eine Textmarke an eine Zeilennummer oder an einen Textinhalt gebunden ist.

**Der schärfste Einwand gegen eine sofortige Aktivierung** ist ein offener Defekt am Zugangsweg. `shared/issues/260807-2112_o_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md` hält fest, dass am laufenden Bündel `cmd+y` und `shift+cmd+y` nicht wirken, `f3` dagegen schon. Befehl und Empfänger sind laut Defekt in Ordnung; der Fehlschlag liegt auf dem Weg vom Tastendruck zum Nachschlagen und trifft nur die Formen mit Zusatztaste. Ein Fokusbefehl für den Editor mit Zusatztaste liefe in denselben Fehler. Die beiden im Defekt genannten Verdächtigen sind nicht gemessen.

### 2. `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter im Vorschaufenster an, statt sie an den Systembrowser abzugeben. Bedient wird er über die Tastatur, mit Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten:** dieselbe eine, `260802-0842-krk-mac-dateimanager-editor-git`, mit demselben Kennzeichen wegen des beschränkten Abschlusses.

Nach der reinen Zählung offener Fragen ist dieser Circle der reifere der beiden: sein Grounding zitiert einen einzigen offenen Entscheidungsdatensatz, die Verfügbarkeitsprüfung für macOS-26-Schnittstellen, und ordnet die Bindung selbst als Schlussfolgerung und nicht als geprüfte Aussage ein. Der Nutzer hat ihn dennoch zurückgestellt. Sein Datensatz trägt bereits einen Abschnitt `## Parent grounding stale` vom Lauf 260807-1042, der zwei Stellen benennt, an denen die Beschränkung der Runde 1 in ihn hineinreicht.

## Recently closed (_c_ / _b_)

Ein Circle ist geschlossen; die Vorlage sieht bis zu fünf vor.

- `260802-0842-krk-mac-dateimanager-editor-git` — beschränkter Abschluss (`_b_`), 260807-1035. Alle 38 Planschritte sind am Code belegt, aber sieben der zehn Zeitzusagen standen beim Abschluss noch auf einer Messreihe, die drei spätere Commits gealtert hatten; das Gelernte ist, dass eine Messreihe an jedem Commit altert, der einen gemessenen Pfad berührt, und es nicht selbst sagt.

## Archived (_s_ / _d_)

(keiner)

Kein Circle-Datensatz trägt `_s_` für überholt oder `_d_` für zurückgestellt.

## Warnings

**1. Beide vorgesehenen Circles hängen an einem beschränkt abgeschlossenen Vorgänger.** `260802-0842-krk-mac-dateimanager-editor-git` trägt `_b_` und nicht `_c_`. Nach der Rangheuristik zählt allein der kohärente Abschluss als erfüllte Vorbedingung, deshalb tragen beide Kandidaten dasselbe Kennzeichen. Es blockiert keinen von beiden. Beim Web-Betrachter reicht die Beschränkung inhaltlich in den Circle hinein, dokumentiert in seinem eigenen Abschnitt `## Parent grounding stale`; beim Editor hat der Nutzer die Restarbeit der Runde 1 ausdrücklich ausgeklammert und den Preis im Grounding benannt.

**2. Fünf offene Defekte liegen in einem terminalen Circle und haben damit keinen Bearbeiter.** Unter `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/` tragen fünf Dateien den Marker `_o_`: der einmal hängengebliebene Sitzungslauf bei L6 (260806-1304), die drei Aufrufer von `eintrag_waehlen`, die den Auswahlversuch wegwerfen (260807-0219), die unvollständige Meldung zur Bündelkennung (260807-0930), der Messstrecken-Defekt, den der Plan an zwei Stellen noch als offen führt (260807-1022), und die zweiundzwanzig Verweise mit überholtem Zustandsmarker (260807-1022). Ein terminaler Circle nimmt keine Arbeit mehr auf. Wer einen dieser Defekte behandeln will, muss ihn in einen lebenden Circle oder in den gemeinsamen Speicher holen. Der Playmaker verschiebt keine Defekte; die Entscheidung liegt beim Nutzer.

**3. `CLAUDE.md` führt den L9-Defekt als offen, geschlossen ist er seit dem 260807-1935.** Zeile 43 zitiert `shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md` und schreibt "Die Ursache der Verschlechterung ist offen" sowie "der Spec sieht deshalb grüner aus, als die Anwendung ist". Auf der Platte trägt die Datei den Marker `_c_`, und ihre Resolved-Zeile hält fest: der Nutzer hat die Einbuße am 260807-1935 angenommen, die Zusage steht bei 65 Prozent im ersten Bild und höchstens zwei Bildlängen, der Abnahmelauf gilt gegen diese Fassung als gehalten, und die Ursache wird nicht weiter verfolgt. Zitat und Aussage in `CLAUDE.md` sind damit beide überholt.

**4. Der Web-Betrachter trägt einen überholten Aktivierungsvorschlag.** Sein Abschnitt `## Activation proposal` vom 260807-1042 nennt ihn "den einzigen nicht abgeschlossenen Circle im Portfolio". Seit dem 260807-2116 gibt es einen zweiten. Der Playmaker schreibt nur an, er überschreibt nicht; maßgeblich ist die Rangfolge in dieser Datei.

**5. Ein offener Entscheidungsdatensatz zitiert einen Pfad, den es nicht mehr gibt.** `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` führt unter `**Cross-references:**` den Pfad `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`. Der Datensatz dieses Circles heißt seit dem 260807-1035 `_b_circle.md`. Der Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` beschreibt diese Fehlerklasse und den Weg über die Sternform `_*_`; ob er diese Stelle führt, ist ungeprüft. Der Playmaker berichtigt keine Zitate.

**Keine Abhängigkeitszyklen.** Der gerichtete Graph über die nicht terminalen Circles hat zwei Knoten und keine Kante zwischen ihnen. Beide Kanten zeigen auf den terminalen Vorgänger und damit aus dem Graphen heraus. Der Editor-Circle schreibt in seinem Abschnitt `## Dependencies` ausdrücklich, der Web-Betrachter sei keine Abhängigkeit.

**Keine neue Kennzeichnung wegen gealterter Grundlage.** Der Lauf hat geprüft, ob der beschränkte Abschluss vom 260807-1035 die Grundlage eines nicht terminalen Circles veralten lässt. Beim Web-Betrachter steht die Kennzeichnung seit dem Lauf 260807-1042 und wird nicht wiederholt. Beim Editor-Circle entfällt sie: sein Grounding ist am 260807-2116 und damit nach dem Abschluss geschrieben, zitiert die Abschlussnotiz wörtlich und trägt den Abschnitt `### Was die Ausklammerung der Messreihen kostet`, der den Preis der Beschränkung selbst benennt. Eine Kennzeichnung "deine Grundlage könnte veraltet sein" wäre hier sachlich falsch.

**Kein Zeigerproblem.** `.active-circle` fehlt, und kein Datensatz trägt `_t_`. Die vier Fehlbedingungen aus der Bestandsaufnahme (verwaister Zeiger, Zeiger auf einen nicht aktiven Circle, mehr als ein aktiver Circle, fehlender Zeiger bei vorhandenem aktivem Circle) treffen alle nicht zu.
