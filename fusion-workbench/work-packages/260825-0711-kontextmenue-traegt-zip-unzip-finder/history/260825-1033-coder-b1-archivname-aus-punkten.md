# Coder: B1 — ein Archivname aus Punkten macht den angezeigten Ordner zum Entpackziel

**Datum:** 2026-08-25 10:33
**Status:** Complete
**Agent:** coder
**Baumstand:** `428fbc4` plus die Änderungen dieses Schritts und die parallel laufende Arbeit eines zweiten Coders in `krk-core`

## Auftrag

Der kritische Befund B1 der ersten Durchsicht dieser Runde,
`issues/260825-0942_*_ein-archivname-aus-punkten-macht-den-angezeigten-ordner-oder-seinen-elternordner-zum-entpackziel.md`,
Durchsicht `reviews/260825-0942-coderev-runde-17-zip-unzip-und-die-regel-des-kontextmenues.md`.

`kontextmenue::ordnername_zum_archiv` gibt den Stamm aus `namen_teilen` unverändert heraus. Aus
`..zip` fällt der Stamm `.`, aus `...zip` der Stamm `..`; `paar` macht daraus `ordner.join(stamm)`,
`Path::join` normalisiert nichts, und `symlink_metadata` löst das `..` zum Elternordner auf. Wer im
Konfliktblatt „Überschreiben" wählt, gibt den angezeigten Ordner oder dessen Elternordner an den
Papierkorb.

`crates/krk-core/` war für diesen Schritt gesperrt: ein zweiter Coder arbeitet dort gleichzeitig an
B2 und B3.

## Was entschieden ist, und warum

Zur Wahl standen drei Antworten: den Eintrag unterdrücken, eine Meldung in die Statuszeile setzen,
oder den Namen anders bilden. **Gewählt ist die dritte**, und die Begründung steht ausgeschrieben am
Kopf von `brauchbarer_stamm`:

Die Directive dieser Runde sagt, die drei Einträge seien immer da und immer bedienbar, und wo ein
Befehl **nichts vorfinde**, melde er es in der Statuszeile. Unzip findet hier aber etwas vor — der
Nutzer hat auf eine Datei geklickt, die die Endung sichtbar trägt. `operationen::kein_archiv()`
(„hier steht keine Datei mit der Endung .zip") wäre vor seinen Augen die Unwahrheit. Den Eintrag
stillschweigend aus `Entpackbefund::Archive` zu nehmen wäre schlechter als beides: von drei
markierten Archiven bliebe eines ohne Ordner und ohne Wort.

Unbrauchbar ist allein der **gerechnete Name**. Für genau diesen Fall trägt das Modul seit dem
ersten Tag der Runde eine Antwort, nämlich `ERSATZSTAMM` für das Wurzelverzeichnis. Sie ist deshalb
**erweitert und nicht verdoppelt**: aus „kein Name" wird „kein brauchbarer Name". `operationen.rs`
ist unangetastet geblieben; ein neuer Satz für die Statuszeile war nicht nötig.

## Was entstanden ist

Eine Datei: `crates/krk-ui/src/kommandos/kontextmenue.rs`.

- **`brauchbarer_stamm(stamm) -> String`**, privat, die eine Sperre. Sie fragt
  `krk_core::operation::umbenennen::name_pruefen` — den Weg, der schon dasteht und den die
  Zielordnerklärung des Entpackens im Zweig `Konfliktantwort::UmbenennenIn` bereits ruft — und
  fällt bei jedem der vier `Namensfehler` auf `ERSATZSTAMM` zurück. **Keine zweite Namensprüfung
  daneben**, und keine eigene Punktregel: die bestehende fängt nebenbei den Stamm `␣␣` aus
  `␣␣.zip`, an den eine Punktregel nicht gedacht hätte.
- **Kein Auffangzweig über `Namensfehler`.** Die vier Varianten stehen einzeln in einem
  Oder-Muster; eine fünfte hält den Bau an.
- **`ordnername_zum_archiv`** mündet mit **beiden** Wegen — dem Stamm des Archivnamens wie dem
  unveränderten Namen ohne Endung — in `brauchbarer_stamm`.
- **`archivname`** ebenso, für die Umkehrrichtung.

### Warum die Prüfung nicht in `paar` steht

Defektdatensatz und Durchsicht schlagen `paar` vor. Sie steht stattdessen in
`ordnername_zum_archiv`, und der Grund gehört zur Sache: die Zusage „das ist ein Name" gehört der
Funktion, die den Namen herausgibt, und nicht einem ihrer Aufrufer. `ordnername_zum_archiv` ist
`pub`; eine Prüfung in `paar` ließe den öffentlichen Rückweg weiterhin `..` herausgeben, und der
nächste Aufrufer — der Ausführungszweig aus Schritt 7 — müsste sie ein zweites Mal mitbringen.
`paar` ist damit unverändert und trotzdem gedeckt.

### Die Umkehrbarkeit

Sie bleibt erhalten und ist zusätzlich symmetrisch geworden. Der Rundweg
Name → Archiv → Name schließt für jeden Namen, den `name_pruefen` durchlässt; die bestehende Probe
`archivname_und_ordnername_kehren_einander_um` läuft unverändert grün. Wo er endet, endet er jetzt
in **beiden** Richtungen gleich: `␣␣` ergibt `Archiv.zip`, und `␣␣.zip` ergibt `Archiv`. Prüfte nur
eine der beiden Richtungen, stünden zwei Regeln, wo eine steht.

### Die Lücke in `archivname`

Sie besteht der Sache nach **nicht**: `Path::file_name` liefert für `.` und `..` `None`, also
konnte dort nie ein Punktname ankommen; der Ersatzstamm stand für diesen Weg ohnehin schon da.
Erreichen konnte `archivname` allein den leeren Stamm, etwa aus einer Datei namens `␣␣`, und das ist
kein Ausbruch, sondern ein unbrauchbarer Name im angezeigten Ordner. Der gemeinsame Weg ist trotzdem
gebaut, wegen der Symmetrie des Paares.

## Proben

Vier neue, alle in `#[cfg(test)]` neben dem Code:

- `ein_archiv_aus_zwei_punkten_zielt_nicht_auf_den_angezeigten_ordner` — `..zip`, voller Weg über
  `entpackziel`, Zweig der betroffenen Einträge.
- `ein_archiv_aus_drei_punkten_zielt_nicht_auf_den_elternordner` — `...zip`, über die Ersatzregel,
  damit beide Zweige von `entpackziel` durch `paar` belegt sind.
- `kein_entpackziel_verlaesst_den_angezeigten_ordner` — zehn Namen, und geprüft wird die **Gestalt**
  statt einer Liste erwarteter Namen: der Zielordner liegt unmittelbar im angezeigten Ordner
  (`ziel.parent() == Some(ordner)`) und sein letzter Bestandteil ist ein `Component::Normal`.
  **Beide Zeilen sind nötig, und jede fängt eine der zwei Gestalten**: `<ordner>/.` trägt
  `Projekte` als letzten Bestandteil — `Path` streicht den Punkt weg — und fällt über den
  Elternteil `/tmp` auf; `<ordner>/..` behält den angezeigten Ordner als Elternteil und fällt allein
  über den `ParentDir` auf. Mit nur einer der beiden bliebe die Probe bei einem der zwei Defekte
  grün.
- `ein_unbrauchbarer_stamm_bekommt_in_beiden_richtungen_den_ersatz` — die Grenze der Umkehrbarkeit.

**Gegenprobe gefahren:** mit `brauchbarer_stamm` auf `stamm.to_owned()` zurückgesetzt werden genau
diese vier rot und die übrigen achtzehn bleiben grün. Die Proben beißen also an der Stelle, an der
der Defekt sitzt.

## Abnahme

- `cargo test -p krk-ui` — exit 0, 784 + 5 Proben.
- `cargo clippy -p krk-ui --all-targets -- -D warnings` — exit 0.
- `cargo fmt --all --check` — exit 0.
- `make check` — **exit 2**, und nicht durch diesen Schritt: es fällt
  `eine_benannte_roehre_mit_schreiber_haelt_das_packen_nicht_an` in
  `crates/krk-core/tests/operation.rs`. Die Probe ist neu und gehört zur laufenden Arbeit des
  zweiten Coders an B2; sie liegt in einer für diesen Schritt gesperrten Datei. Alle übrigen
  Probenläufe des Workspace sind grün.

## Was offen bleibt

Nichts in `krk-ui`. Für `krk-core` ist **keine** Änderung nötig — der Kern tut, was der Auftrag
sagt, und die Prüfung gehört an die Stelle, an der der Name entsteht.

Ein Hinweis für den Menübau und den Ausführungszweig (Schritte 6 und 7): zwei Archive, deren
Stämme beide unbrauchbar sind, zielen jetzt auf denselben Ordner `Archiv`. Das ist keine neue Lage
— `a.zip` neben `a.ZIP` tut es seit dem ersten Tag dieser Runde, weil die Endung ohne Rücksicht auf
die Schreibung erkannt wird —, und die Zielordnerklärung des Vorgangs fragt beim zweiten nach,
einmal je Archiv.
