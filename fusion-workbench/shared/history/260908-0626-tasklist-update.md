# Arbeitswarteschlange gegen den Baumstand `fe5fe9c`

**Datum:** 2026-09-08 06:26
**Baumstand:** `fe5fe9c`, Branch `main`
**Angefordert von:** Nutzer, Task-ID Q1
**Bereich:** code

Die Warteschlange der Sitzung vom 260905 ist ersetzt. Sie steht in diesem Eintrag und in
keiner eigenen Datei; der Orchestrator hält sie für die Sitzung.

## Was gelesen wurde

| Speicher | Dateien |
|---|---|
| `shared/issues` | alle offenen gelesen oder klassifiziert |
| `circles/*/issues` (19 Runden mit offenem Bestand) | alle offenen |
| `shared/decisions` | die 11 offenen und die 20 beantworteten |
| `circles/*/decisions` | keine offene |
| `shared/planning` | 6 Dateien |
| `circles/*/planning` | 8 Dateien mit `_o_` |
| `shared/reviews`, `circles/*/reviews` | Kopfzeilen aller jüngsten, `Not-opened:` durchgehend `none` |
| `260905-2307-woran-die-naechste-schleife-ansetzt.md` (Standanalyse) | vollständig |

## Erhebung

Gezählt am 260908-0626 gegen `fe5fe9c`:

```sh
find fusion-workbench/shared/issues -maxdepth 1 -name '*_o_*.md' | wc -l      # 122
find fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md' | wc -l   # 67
find fusion-workbench/shared/decisions -maxdepth 1 -name '*_o_*.md' | wc -l   # 11
find fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md' | wc -l # 0
find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions \
     -maxdepth 1 -name '*_a_*.md' | wc -l                                     # 20
```

- **189 offene Defekte** (122 gemeinsam, 67 in Runden). Die Zahl des Auftrags stimmt.
- **11 offene Fragen**, alle im gemeinsamen Speicher, alle mit Zeitstempel vom 260907.
  Der Auftrag nennt neun; es sind elf.
- **0 Defekte mit Marker `_p_`.**
- **20 beantwortete, noch nicht umgesetzte Fragen.**

Der Bestand ist seit der Standanalyse vom 260905-2307 von 286 auf 189 gefallen, also um 97.

## Bündelgrenzen

Geschnitten nach Schreibziel, weil das die Grenze ist, an der mehrere Agenten gleichzeitig
schreiben können. Sechs Defekte, die `claude-md` im Dateinamen führen, sind aus dem Bestand
genommen: sie sind bereits vergeben. Die Partition über die verbleibenden 183 ist vollständig
und überschneidungsfrei:

| Bahn | Schreibziel | Datensätze | davon nur eine Zahl in Prosa |
|---|---|---|---|
| A | `krk-core`, ohne `krk-ui` | 38 | 5 |
| B | `krk-ui`, ohne `krk-core` | 75 (52 unter `appkit/`, 23 Modelle) | 14 |
| AB | beide Kisten zugleich | 39 | 11 |
| D | `xtask` und `krk-bench`, ohne die zwei Kisten | 10 | 1 |
| E | nur `resources/*.toml` | 7 | 5 |
| W | nur Werkbankdatensätze | 14 | 7 |
| | **Summe** | **183** | **43** |

Der Engpass innerhalb von B ist `crates/krk-ui/src/appkit/anwendung.rs`: 29 der 189 offenen
Datensätze zitieren die Datei. Zwei Agenten in `krk-ui` treffen sich dort, gleich wie sauber
die Aufgabenteilung ist. Eine Bahn je Kiste ist die richtige Breite.

## Die Warteschlange

Zwölf Posten in vier Stufen; die Stufen sind Abhängigkeiten, innerhalb einer Stufe laufen die
Posten parallel. Die tragende Kante ist „Verhalten vor Prosa je Kiste": eine berichtigte Zahl,
die vor der Verhaltensänderung geschrieben wird, ist danach wieder falsch.

**Stufe 1** — Q1 Werkbank-Buchhaltung (14), Q2 `krk-core` Verhalten und Proben (33),
Q3 `krk-ui/appkit` Verhalten (52), Q4 `krk-ui` Modelle (23), Q5 `xtask` und `krk-bench` (9).

**Stufe 2** — Q6 `krk-core` Zahlen (5), Q7 `krk-ui` Zahlen (14), Q8 `xtask` Zahlen (1).

**Stufe 3** — Q9 `resources/*.toml` (7, `ontocoder`), Q10 kistenübergreifend (39).

**Stufe 4** — Q11 die Entwurfsvorlagen, Q12 die Vordergrundliste.

Die zwei Nebenlisten: **acht Posten warten auf eine Antwort des Nutzers**, und **elf Datensätze
sind Nutzerarbeit**, weil ihre Schließung KRK im Vordergrund verlangt und kein Agent den Lauf
fahren kann.

Die vollständige Fassung mit Kennung, Gegenstand, Agent, Abhängigkeit und Umfang je Posten
steht im Bericht an den Auftraggeber und ist dort auch als Abhängigkeitsgraph gezeichnet.

## Was diese Sitzung nicht einplanen darf

- Die sechs Befunde gegen `CLAUDE.md` (`260906-0008`, `260907-0726`, `260907-0859`,
  `260907-2046`, `260907-2300`, `260908-0002`) sind einem eigenen Durchgang zugeteilt.
- Die 56 Fragen, die am 260906 offen standen, sind beantwortet; neunzehn Bauaufträge daraus
  sind gefahren und abgenommen.
- Das Dokumentationstor steht in allen vier Kisten auf null, `make check` fährt `cargo doc`
  als fünftes Abnahmekommando (`Makefile:73`).

## Abgelegte Datensätze

Keine. Dieser Lauf hat keinen Defekt und keine Frage abgelegt und keinen Marker bewegt.
