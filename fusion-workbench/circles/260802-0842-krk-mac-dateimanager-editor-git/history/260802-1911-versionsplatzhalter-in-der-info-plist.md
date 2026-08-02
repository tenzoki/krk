# Versionsplatzhalter in der Info.plist (Schritt 4b)

**Datum:** 260802-1911
**Agent:** ontocoder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `## Implementierungsschritte`, Schritt 4b (eingefügt am 260802-1859)
**Hintergrund:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1835_c_versionsnummer-steht-an-zwei-stellen-ohne-abgleich.md`, geschlossen
**Geänderte Dateien:** `resources/Info.plist`

## Was geändert wurde

Drei Stellen, alle in derselben Datei. Kein Schlüssel ist dazugekommen oder
weggefallen; die Datei trägt weiterhin fünfzehn Schlüssel.

| Stelle | Vorher | Nachher |
|---|---|---|
| Wert von `CFBundleShortVersionString` | `0.1.0` | `__KRK_VERSION__` |
| Kommentar an dieser Stelle | "Führt `version = "0.1.0"` … von Hand nach" | Platzhalter, Version wohnt allein in `[workspace.package]`, `bundle` setzt sie ein und bricht ohne Platzhalter ab |
| Kopfkommentar der Datei | "kopiert diese Datei unveraendert …; es gibt keine Ersetzung von Platzhaltern beim Bauen" | kopiert sie und setzt dabei den Versionsplatzhalter ein, sonst unverändert |

`CFBundleVersion` steht unberührt auf `1`. Es ist die Baunummer, steht nirgends
ein zweites Mal und gehört nicht zu der Doppelung, die Schritt 4b auflöst.

## Der Kopfkommentar war nicht ausdrücklich genannt

Schritt 4b nennt in seiner Änderungsliste nur den Kommentar **an der Stelle** des
Versionsschlüssels. Der Kopfkommentar der Datei, ebenfalls am 260802-1829
geschrieben, behauptete daneben wörtlich: "es gibt keine Ersetzung von
Platzhaltern beim Bauen". Genau das kehrt Schritt 5 um ("Die `Info.plist` wird
nicht mehr unverändert kopiert."). Stehengelassen hätte die Datei über sich
selbst etwas Falsches ausgesagt, unmittelbar über einem Platzhalter, dessen
Ersetzung sie bestreitet.

Der Satz ist deshalb mitgezogen. Das verletzt kein Abnahmekriterium von Schritt
4b: dessen Bedingung "die fünf TCC-Texte und die übrigen Schlüssel sind im Diff
unverändert" spricht von Schlüsseln, und kein Schlüssel ist angefasst. Die Lücke
in der Formulierung von Schritt 4b ist dem Nutzer gemeldet; am Plandokument ist
nichts geändert.

## Kein Widerspruch zwischen Schritt 4b und Schritt 5

Beide Schritte wurden gegeneinander gelesen, ebenso der geschlossene Defekt. Sie
greifen sauber ineinander:

- Schritt 4b setzt die Zeichenkette `__KRK_VERSION__`, Schritt 5 sucht dieselbe
  Zeichenkette. Buchstabengleich, zweimal in beiden Schritten geschrieben.
- Schritt 5 bricht ab, wenn der Platzhalter fehlt. Schritt 4b hält die
  Reihenfolge ausdrücklich fest: ein Schritt 5 vor Schritt 4b findet nichts und
  bricht ab, wie es soll. Die Abhängigkeit steht in beiden Richtungen im Plan
  (Schritt 4b hängt an Schritt 4, Schritt 5 hängt an Schritt 4b).
- Schritt 5 setzt `env!("CARGO_PKG_VERSION")` ein und macht die Erbschaft zur
  Voraussetzung. Nachgeprüft an `xtask/Cargo.toml`: dort steht
  `version.workspace = true`, das Bauwerkzeug erbt also dieselbe Zahl, die
  `[workspace.package]` der `Cargo.toml` führt (derzeit `0.1.0`).
- Schritt 5 lässt `resources/Info.plist` unangetastet und wirkt nur auf die Kopie
  im Bündel. Der Platzhalter bleibt also dauerhaft in der Quelldatei, was Schritt
  4b voraussetzt.
- Das Abnahmekriterium von Schritt 5 vergleicht gegen die `Cargo.toml` statt
  gegen ein Literal. Sonst wäre dort die dritte Stelle entstanden, an der die
  Zahl steht.

## Abnahme

Die drei Kommandos des Abnahmekriteriums, am 260802-1911 auf dem Referenzgerät
aus dem Projektwurzelverzeichnis ausgeführt.

```
$ plutil -lint resources/Info.plist
resources/Info.plist: OK
Rueckgabewert: 0

$ plutil -extract CFBundleShortVersionString raw resources/Info.plist
__KRK_VERSION__
Rueckgabewert: 0

$ grep -q '0\.1\.0' resources/Info.plist
Rueckgabewert: 1   (kein Treffer, wie verlangt)
```

**Der Lint nimmt den Platzhalter an.** Das war die offene Frage: `plutil -lint`
prüft die Wohlgeformtheit der Property-Liste, also XML und Typen, nicht das
Format eines einzelnen Wertes. `__KRK_VERSION__` ist eine gewöhnliche
Zeichenkette und passiert die Prüfung. `inference:` Der Versionsstring wird erst
vom System ausgewertet, wenn ein fertiges Bündel startet, und dort steht der
Platzhalter nie: Schritt 5 ersetzt ihn beim Kopieren und bricht ab, wenn er ihn
nicht findet. Nachgemessen an einem laufenden Bündel ist das nicht, weil Schritt
5 noch nicht gebaut ist.

Zusätzlich geprüft, über das Kriterium hinaus:

- `plutil -extract CFBundleVersion raw` liefert `1`, der Wert ist unberührt.
- Die Datei nach JSON gewandelt trägt fünfzehn Schlüssel, dieselben wie nach
  Schritt 4, mit den fünf Rückfragetexten wortgleich.
- `git diff -- resources/Info.plist` zeigt genau drei Änderungsblöcke: zwei
  Kommentare und die eine Wertzeile. Keine Zeile sonst.

## Nicht gemacht

Kein Bündelbau, keine Ersetzung im Bauwerkzeug, kein `codesign` — das ist
Schritt 5. `crates/`, `xtask/`, `Cargo.toml` und `spikes/` sind unberührt. Am
Plandokument nichts geändert, auch der Schrittstatus nicht; am geschlossenen
Defekt nichts geändert. `CLAUDE.md`, Aktivitätsprotokoll und Wurzel-`README.md`
unberührt. Keine Auslieferungsbelegung (`resources/default-keymap.toml`), das ist
Schritt 9 und der Nutzer will vorher gefragt werden. Kein Commit, keine
Aufwandsschätzung.
