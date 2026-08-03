# Abnahmemaß für L1 und L9 geändert

**Datum:** 260803-1819
**Agent:** planner
**Status:** Complete
**Auslöser:** Nutzerentscheidung vom 260803-1810 zum Entscheidungsdatensatz `decisions/260803-1755_a_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`
**Geändert:** `planning/260802-1036_o_spec-navigator-geruest.md` (C8 und zwei Kopfzeilen), `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (`### Frage 5`, Kopf von `## Implementierungsschritte`, S8, S21, `## Angelegte Defekte und Entscheidungen`, Datumszeile), `decisions/260803-1755_a_l1-verfehlt-die-16-ms-zusage-am-bildrand.md` (Antwort, Marker `_o_` → `_a_`), `issues/260803-1755_c_dateiliste-von-schritt-8-nennt-fuenf-noetige-dateien-nicht.md` und `issues/260803-1755_c_schritt-8-legt-perzentil-und-bericht-in-eine-datei-die-nur-eine-haelfte-kennt.md` (beide `Resolved:`, Marker `_o_` → `_c_`)
**Neu angelegt:** `issues/260803-1819_o_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`, diese Datei
**Nicht angefasst:** `crates/`, `xtask/`, `messungen/`, `README.md`, `CLAUDE.md`. Kein Commit.
**Stilprofil:** `stilwerk/chat-voice-de.yaml` und `stilwerk/default-voice-de.yaml` geladen.

## Die Entscheidung und was sie ändert

Der Nutzer hat Möglichkeit 2 gewählt: L1 und L9 nehmen nicht mehr über 16 ms für das 95. Perzentil der Zeitspanne ab, sondern über den Anteil der Eingaben, die ihr nächstes Bild erreichen. Möglichkeit 4, den Technologieentscheid aufzumachen, hat er ausdrücklich abgelehnt; Rust mit `objc2` bleibt.

Zwei Begründungen tragen dieselbe Wahl, und der Datensatz hält beide getrennt fest. Die erste ist messtechnisch und stand schon in der Vorlage: die 16 ms liegen innerhalb der Streuung ihres eigenen Messverfahrens. Die zweite kommt vom Nutzer und argumentiert wahrnehmungsseitig: eine Spanne dieser Größe ist für einen Menschen nicht unterscheidbar, die Zahl beschreibt also keine erlebbare Eigenschaft.

## Wie das neue Maß formuliert ist

Die Vorschrift steht in C8 des Specs unter `Die Vorschrift, prüfbar formuliert` und in `### Frage 5` des Plans in derselben Fassung. Drei Bestandteile:

- **Was ein Bild ist.** Der Kehrwert der Bildwiederholrate des Bildschirms, auf dem das gemessene Fenster steht, gelesen aus `NSScreen.maximumFramesPerSecond`. Am Referenzgerät sind das 60 Hz und damit 16,667 ms.
- **Wann eine Eingabe ihr Bild erreicht.** Wenn die Spanne vom Zeitstempel des Tastenereignisses bis zum Ende des Zeichendurchgangs höchstens eine Bildlänge beträgt. Ist sie größer, wird die Änderung erst mit dem übernächsten Bild sichtbar.
- **Über wie viele Wiederholungen.** Zwanzig je Runde, wie C8 es für alle zehn Zusagen vorschreibt. Die Zusage hält in einer Runde, wenn mindestens 95 Prozent der Eingaben ihr Bild erreichen; bei zwanzig Wiederholungen darf also höchstens eine verpassen. Über mehrere Runden gilt unverändert, dass gehalten in jeder Runde gehalten heißt.

Fehlt die Bildwiederholrate, bricht die Auswertung ab, statt 60 Hz zu unterstellen. Dieselbe Haltung wie bei `--kalt` ohne Rechte und bei einem Fenster ohne Bildschirm.

## Der Einwand, den C8 jetzt ausschreibt

Vor der Änderung nahm C8 alle zehn Zusagen einheitlich über das 95. Perzentil einer Zeitspanne ab. Für L1 und L9 steht danach ein zweites Abnahmemaß daneben, und das arbeitet gegen die Maxime "supersimpel". Der Einwand bleibt in C8 stehen und wird dort mit zwei Gründen beantwortet, damit ein späterer Leser die Ausnahme nicht für eine Nachlässigkeit hält.

Erstens wirkt die Maxime als Ausschlussgrund gegen eine Lösung, die eine Fähigkeit mit eigener Sonderregel, eigener Ausnahme und eigenem Rückfallweg erkauft. Das neue Maß hat weder Ausnahme noch Rückfallweg. Zweitens war die aufgegebene Einheitlichkeit keine: acht Zusagen beschreiben eine Dauer, zwei beschreiben eine Schwelle, und beide unter dasselbe Maß zu zwingen hat ein Urteil erzeugt, das mit dem Zufall der Messphase wechselte.

## Die 120-Hz-Passage

Die alte Fassung legte fest, L1 und L9 blieben auf jedem Mac bei 16 ms, damit die Zahl nicht mit dem Bildschirm wandert. Diese Festlegung hing an einer festen Zahl, die es nicht mehr gibt, und ist zurückgenommen. Abgenommen wird auf dem Referenzgerät mit 60 Hz, wie die Messbedingungen es ohnehin festlegen. `inference:` Auf einem 120-Hz-Gerät wäre die Zusage mit dem gemessenen Eigenanteil von 3 bis 8 ms nicht sicher zu halten; das ist ein Befund für eine spätere Messung.

## Die beiden geschlossenen Defekte

`260803-1755_c_dateiliste-von-schritt-8-nennt-fuenf-noetige-dateien-nicht.md`: die Dateiliste von S8 nennt jetzt alle vierzehn Dateien. Die fünf fehlenden binden nichts ein, an vieren liest der Schritt etwas ab oder löst etwas aus. Die Regel im Kopf von `## Implementierungsschritte` ist um genau diesen zweiten Fall erweitert und bindet damit jeden künftigen Schritt.

`260803-1755_c_schritt-8-legt-perzentil-und-bericht-in-eine-datei-die-nur-eine-haelfte-kennt.md`: der Absatz zur Grenze in S8 weist `messmodus.rs` jetzt Ablauf, Wiederholungen und Ausgabe der Einzelwerte zu; Auswertung und Bericht liegen bei `crates/krk-bench/`. Der Grund steht daneben: L4 beginnt in einem anderen Prozess als dem, der es beendet, und der Bedingungskopf wird seit S3 in `bericht.rs` erhoben. Dieselbe Formulierung in S21 ist mitgezogen.

## Ein neuer Defekt

Die erweiterte Dateilisten-Regel ist bisher nur auf S8 angewandt. Die Listen von S9 bis S23 stehen weiter auf dem Stand der engeren Regel vom 260802-1859. Der Nachzug ist bewusst nicht in diese Bearbeitung gefallen: fünfzehn Dateilisten spekulativ zu erweitern hätte die Änderung unüberschaubar gemacht. Gemeldet als `issues/260803-1819_o_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`.

## Ein Befund an C8, den niemand gemeldet hatte

**L5 ist die Zusage, die als nächste an dieselbe Grenze stoßen könnte.** Sie sagt 50 ms zu, hergeleitet als drei Bilder bei 60 Hz, und wird wie L1 an einer Bildgrenze beendet. Vom Budget verbraucht das Warten auf diese Grenze im 95. Perzentil rund 15,8 ms, also fast ein Drittel, bevor KRK gearbeitet hat. `inference:` Mit 34 ms verbleibender Arbeitszeit dürfte L5 halten; gemessen ist es nicht, denn L5 kommt erst in S21. Der Befund steht als Notiz in S21, nicht als Entscheidungsdatensatz: die Regel des Plans, dass eine verfehlte Zusage zu einem Datensatz führt, greift erst nach der Messung, und ein Datensatz ohne Zahlen hätte keine Möglichkeiten zum Abwägen.

L6, L7 und L8 tragen dieselbe Quantisierung. Bei 100 bis 200 ms Budget fällt ein Bild dort deutlich weniger ins Gewicht.

## Was der nächste Schritt vorfindet

Der `coder` stellt die Auswertung in `crates/krk-bench/src/messen.rs` um und fährt eine neue Messung. Was im Einzelnen umzustellen ist, steht in S8 unter `Was der coder daraufhin umzustellen hat`, in vier nummerierten Punkten. S8 bleibt ohne `[DONE]`, bis der neue Bericht vorliegt; erst dessen Commit zieht den Entscheidungsdatensatz auf "umgesetzt".

Eine kleine Korrektur nebenbei, die kein Defekt war: C8 sagte "Keiner der Werte ist an KRK gemessen, weil KRK noch nicht existiert." Seit S8 existiert KRK. Der Satz sagt jetzt, dass keiner der Werte aus einer Messung an KRK entstanden ist und es beim Aufstellen der Zusagen KRK noch nicht gab.
