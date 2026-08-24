# Shaper: die zweite Klärungsrunde vor dem Spec der Runde 16

**Datum:** 2026-08-24
**Agent:** shaper (user-direct, aktiver Circle in Bereich)
**Status:** Complete — angehalten mit drei Fragen an den Nutzer, kein Spec geschrieben

## Was eingegeben war

Der Circle-Datensatz `_t_circle.md`, der Klärungsverlauf `260824-0541-shaper-klaerungsrunde-vor-dem-spec.md`,
die drei am 260824-0555 beantworteten Entscheidungsdatensätze, die zwei am 260824-0530
beantworteten und der Backlogeintrag `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`.
Der Auftrag nannte die vier ohne Rückfrage abgeleiteten Festlegungen als im Spec sichtbar zu
machen und verlangte prüfbare Abnahmekriterien, den Bausteinsatz einzeln, die Form der
Erkennungsregel samt Vorrang, die zwei Fehlerlagen der `readers.toml`, ein messbares Kriterium
zur Zeitzusage L7 und die sechs Zusammenfassungen als Abnahmefall.

## Was am Baum erhoben ist, Stand `278a008`

- **Die Ausdruckskiste steht schon im Bündel.** `cargo tree -p krk-ui -e normal` zeigt
  `syntect` → `fancy-regex` 0.16.2 → `regex-automata` 0.4.18 → `aho-corasick` 1.1.5 und
  `memchr` 2.8.3, dazu `regex-syntax` 0.8.11. Die Wurzel-`Cargo.toml` zählt dieselben Pakete
  in ihrer Begründung zu `syntect` namentlich auf. Die Kostenangabe im Datensatz vom
  260824-0541 ist damit falsch; der Defekt dazu ist gefiltert unter `issues/260824-0600_o_…-er-fuehrt-eine.md`.
- **Die Zusage über C-Code hält.** `Cargo.lock` führt am 260824-0600 kein `cc` und außer
  `windows-sys` kein `-sys`-Paket; 97 Pakete insgesamt.
- **`Eintrag` trägt den Änderungszeitpunkt bereits** (`krk-core/src/verzeichnis/eintrag.rs:47`).
  Das Sortieren nach Änderungsdatum kostet damit keinen zusätzlichen Systemaufruf; die Kosten
  der Antwort vom 260824-0555 beschränken sich auf die zehn Dateiöffnungen.
- **Kein Defektdatensatz trägt eine Markdown-Überschrift.** Das Dateiformat für Defekte
  schreibt eine nackte Titelzeile vor; nachgezählt sind 82 Dateien in `shared/issues/` und 157
  im größten Speicher eines Circles. Entscheidungs-, Verlaufs-, Analyse- und
  Planungsdatensätze beginnen sämtlich mit `# `.
- **Der größte Speicher der Werkbank trägt 157 Einträge**, der größte gemeinsame 118. Eine
  Obergrenze gelesener Einträge oberhalb dieser Zahl greift an keinem heutigen Ort.
- **`agentstate.yaml` fehlt weiterhin**, in `.gitignore` geführt; `orchestrator-live.md` und
  `.active-circle` stehen da. `.fusion-setup` trägt `setup_at`, `setup_pwd` und
  `plugin_version` in einer Zeile JSON.
- **Die Werkbank führt 18 Circles**, davon einer aktiv, zehn beschränkt und fünf kohärent
  geschlossen, zwei zurückgestellt.

## Warum wieder kein Spec entstanden ist

Drei Fragen stehen zwischen den Antworten des Nutzers und den Abnahmekriterien, und keine ist
aus dem Vorliegenden ableitbar. Die Form des Pfadmusters ist nie gestellt worden, und der
Auftrag verlangt sie ausdrücklich im Spec; ohne sie lässt sich nicht schreiben, was der Nutzer
in seine Datei tippt und ob der einzelne Circle überhaupt erkennbar ist. Die zweite Frage
entsteht erst durch die Messung am Bestand: die Antwort vom 260824-0555 sollte Sätze statt
Dateilisten liefern und liefert für die zwei größten Speicher der Werkbank Dateilisten. Die
dritte betrifft eine Angabe des mitgelieferten Profils, deren Quelldatei in dieser Werkbank
nicht existiert.

## Was entstanden ist

Drei Entscheidungsdatensätze unter `decisions/` dieses Circles und ein Defektdatensatz unter
`issues/`:

- `decisions/260824-0600_o_welche-form-hat-das-pfadmuster-und-welche-die-kennzeichendatei.md`
- `decisions/260824-0600_o_der-titel-aus-der-ueberschriftenzeile-erreicht-keinen-einzigen-defektdatensatz.md`
- `decisions/260824-0600_o_woher-nimmt-die-wurzelzusammenfassung-ihre-sitzungsinfo.md`
- `issues/260824-0600_o_der-entscheidungsdatensatz-zum-regulaeren-ausdruck-sagt-der-baum-fuehre-keine-solche-kiste-er-fuehrt-eine.md`

## Was der Spec nach den Antworten tragen wird

Die vier abgeleiteten Festlegungen der ersten Runde stehen unverändert und kommen sichtbar in
den Spec: erstes passendes Profil in der Datei gewinnt, Pfadmuster vor Kennzeichendatei; die
Zählung läuft flach über einen Ordner; die Zusammenfassung entsteht beim Auswählen; sie trägt
eine Obergrenze gelesener Einträge. Dazu kommt eine fünfte, aus der heutigen Messung: die
Obergrenze liegt oberhalb von 157, dem größten heutigen Speicher, sodass keine Zählung des
Beispielfalls gekappt wird.

Für die Zeitzusage L7 wird der Spec ein abzählbares Kriterium tragen und keine Zeitmessung:
wie viele Verzeichnisleseläufe und wie viele Dateiöffnungen eine Zusammenfassung höchstens
auslöst. Ein Zeitkriterium ließe sich ohne den Abnahmelauf im Vordergrund nicht prüfen, und
der ist Nutzerarbeit. Das folgt der Form, die die Runde 2 für ihr Verhältnis zu C8 gewählt hat.

## Nächster Schritt

Der Nutzer beantwortet die drei Fragen. Danach ist der Shaper erneut zu beauftragen; der Spec
entsteht dann unter `planning/` dieses Circles.
