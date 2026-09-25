# Zwei Plandefekte nachgezogen: Signaturidentität und die `unsafe`-Prüfvorschrift

**Agent:** planner
**Datum:** 260803-1200
**Status:** Complete
**Geänderte Datei:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

---

## Auftrag

Zwei gemeldete Defekte am Plan der ersten Runde beheben, ohne einen neuen Plan zu schreiben und ohne Code, Daten oder Dokumentation außerhalb der Plandatei anzufassen. Schrittnummern und Abhängigkeiten bleiben unverändert, die Schritte 1 bis 5 einschließlich 4b bleiben `[DONE]`.

- `issues/260802-1935_c_frage-7-und-schritt-5-widersprechen-sich-bei-der-signaturidentitaet.md`
- `issues/260802-1810_c_abnahmekriterium-mit-grep-unsafe-kann-nicht-aufgehen.md`

## Was geändert wurde

**`### Frage 7`, Abschnitt "Zweitens".** Der Abschnitt sagte zu, S5 *erzeuge* eine lokale selbstsignierte Identität im Schlüsselbund; die `Änderungen` desselben Schritts sagten, er *suche* sie und breche sonst ab. Umgesetzt ist die zweite Lesart, und sie ist die richtige: ein Bauwerkzeug, das ungefragt Schlüsselmaterial in den Anmeldeschlüsselbund schreibt, geht über seine Aufgabe hinaus. `### Frage 7` steht jetzt auf dem umgesetzten Stand und beschreibt die dreistufige Suche, die seit Commit `4884f85` gilt: `KRK_SIGN_IDENTITY`, dann der Name `KRK Entwicklung` über `security find-identity -p codesigning` ohne `-v`, dann die genau eine gültige Identität über dieselbe Abfrage mit `-v`. Bei null und bei mehr als einer greift die dritte Stufe nicht, weil die Wahl geraten wäre. Greift keine, bricht der Bau mit einer Anleitung ab. Die Vorlage dafür war `README.md`, Abschnitte "Signierung" und "Entwicklungsidentität anlegen", die den Stand bereits korrekt beschreiben.

**Die `Änderungen` von Schritt 5.** Sie nannten nur zwei Stufen, weil die dritte erst nach dem Schreiben des Plantexts dazukam. Jetzt nennen sie alle drei und verweisen für die Begründung auf `### Frage 7`. Ohne diese Angleichung hätte die Behebung des ersten Defekts einen zweiten, feineren Widerspruch an derselben Stelle hinterlassen.

**Das Abnahmekriterium von Schritt 5.** `codesign -dv` gibt die Zeile `Authority=` nicht aus und benennt die Identität damit nicht; es zeigt nur `flags=0x0(none)`, was eine Ad-hoc-Signatur ausschließt. Das Kriterium verlangt jetzt `codesign -dvv`, nennt die Zeile `Authority=` als das Geprüfte und schreibt aus, warum zwei `v` nötig sind.

**Die Abnahmekriterien der Schritte 2 und 15.** Beide verlangten, `grep -rln 'unsafe' crates/krk-core/src` nenne genau eine Datei. Das kann nicht aufgehen, weil `crates/krk-core/src/lib.rs` die Zeile `#![deny(unsafe_code)]` trägt und damit selbst die gesuchte Zeichenkette enthält. Beide prüfen jetzt auf das Attribut, verankert am Zeilenanfang.

## Welcher der beiden Wege gewählt wurde, und warum

Der Defekt bot zwei Wege an: auf den `unsafe`-Block prüfen (`grep -rn 'unsafe {'`) oder auf das Attribut `#[allow(unsafe_code)]`. Gewählt ist der zweite, und zwar aus dem Grund, den der Defekt selbst nennt: er trifft die eigentliche Zusage. Die Zusage lautet nicht "es gibt genau einen `unsafe`-Block", sondern "es gibt genau eine Stelle, an der die Sperre geöffnet ist". Der Block-grep ist außerdem gegenüber `unsafe fn`, `unsafe impl` und `unsafe extern "C"` unvollständig, und die letzte Form kommt im Modul tatsächlich vor (`crates/krk-core/src/verzeichnis/sys.rs:88`).

Der Wortlaut brauchte eine Verankerung, die der Defekt nicht vorwegnehmen konnte. Ein blosses `grep -rln 'allow(unsafe_code)' crates/krk-core/src` liefe in dieselbe Falle wie die alte Vorschrift, weil der Modulkommentar der `lib.rs` das Attribut in Zeile 11 erwähnt. Nachgeprüft am 260803-1200 auf dem Referenzgerät:

```
$ grep -rln 'allow(unsafe_code)' crates/krk-core/src
crates/krk-core/src/lib.rs
crates/krk-core/src/verzeichnis/sys.rs

$ grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src
crates/krk-core/src/verzeichnis/sys.rs
```

Der verankerte Ausdruck fasst beide Attributformen, die innere `#![allow(...)]` am Modulkopf und eine äußere `#[allow(...)]` an einem einzelnen Element, und lässt Kommentarzeilen aus, weil `//!` und `//` nicht mit `#` beginnen.

Beide Kriterien nennen daneben den erfolgreichen `cargo build -p krk-core` als zweite Hälfte des Belegs. Die Aufteilung ist sauber: der Bau erzwingt über `#![deny(unsafe_code)]`, dass außerhalb einer Datei mit der Ausnahme kein `unsafe` steht, und der grep zeigt, dass es die Ausnahme genau einmal gibt. Der `coder` hat die Wirksamkeit des `deny` am 260802-1803 mit einem probeweise eingesetzten Block belegt.

## Was daneben aufgefallen ist

Das Abnahmekriterium von Schritt 6 trägt denselben grep-Fehler ein drittes Mal: es verlangt, `grep -rn 'unsafe' crates/krk-ui/src --include='*.rs' -l` liefere ausschließlich Dateien unterhalb von `src/appkit/`, während `crates/krk-ui/src/main.rs` die Zeile `#![warn(unsafe_code)]` trägt und nicht dort liegt. Schritt 6 war nicht Gegenstand der Meldung und ist nicht mitverändert worden. Der Punkt liegt als eigener Defekt: `issues/260803-1200_o_abnahmekriterium-von-schritt-6-traegt-denselben-grep-fehler.md`.

Die Auflösung kann dort nicht wörtlich dieselbe sein. `krk-ui` trägt `warn`, nicht `deny`, und eine Warnung bricht den Bau nicht ab; die Hälfte des Belegs, die in `krk-core` der Bau trägt, fehlt in `krk-ui`. Der Defekt nennt zwei Wege, darunter den Wechsel auf `deny`. Dieser Wechsel würde eine begründete Festlegung aus `## Aufbau` und aus Schritt 1 aufheben und gehört deshalb dem Nutzer vorgelegt, nicht still gewählt.

Schritt 6 ist noch nicht umgesetzt. Der Defekt schlägt zu, sobald er abgenommen wird.

## Buchführung

- Beide bearbeiteten Defekte tragen eine `Resolved:`-Zeile und den Marker `_c_`.
- Der Plankopf trägt eine Zeile `**Nachzug 260803-1200:**` mit beiden Punkten, dem Muster der drei vorangegangenen Nachzüge folgend.
- Der Abschnitt `## Angelegte Defekte und Entscheidungen` führt die beiden geschlossenen Meldungen jetzt mit auf; seine Einleitung steht auf sechs Rückläufern aus den Umsetzungen der Schritte 2 bis 5 statt auf vier.
- Der Entwurf selbst ist unverändert. Keine Schrittnummer, keine Abhängigkeit und kein `[DONE]`-Vermerk wurde angefasst, und außerhalb der Plandatei, der beiden Defektdateien, des neuen Defekts und dieses Protokolls hat sich nichts geändert.
- Nicht committet, wie beauftragt.
