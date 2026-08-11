# Planner: der Umsetzungsplan der Belegungsausgabe

**Datum:** 2026-08-11, 08:38
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Status:** Abgeschlossen
**Agent:** planner, als Unteragent dispatcht

## Was diese Sitzung geliefert hat

`planning/260811-0838_o_plan-tastenbelegung-als-markdown-in-downloads.md`: vier Schritte, alle an `coder`, dazu acht Befunde am Code, die Antworten auf die acht Punkte, die der Spec dem Planner überlässt, drei Schaubilder und eine Zuordnungstabelle über alle 40 Abnahmekriterien des Specs.

| Schritt | Gegenstand | Hängt an |
|---|---|---|
| S1 | Messung: wer beantwortet die sechs zugestellten Textbefehle | keinem |
| S2 | `Wirkungsbereich::beschriftung` im Kern, ohne Auffangzweig | keinem |
| S3 | Ausgabemodul, Menüeintrag, Meldung in der Statuszeile | S1 und S2 |
| S4 | Abnahme am gebauten Bündel, `Nutzerarbeit` | S3 |

## Die tragende Frage und warum sie zweigeteilt ist

Die Entscheidbarkeitszeile des Plans trennt, was der Spec zusammen behandelt. Für 65 der 71 Funktionen ist die dritte Spalte ohne Näherung aus der Belegung ableitbar: sie tragen ein Kommando, `Kommando::wirkungsbereich` ist eine totale Funktion darüber, und die Beschriftung der sieben Werte ist eine zweite. Für die sechs vom Hauptmenü zugestellten Textbefehle ist sie aus der Belegung **nicht** entscheidbar, weil sie kein Kommando tragen und die Antwortkette von AppKit entscheidet, in die die Belegung keine Eingabe hat.

Der Plan wechselt deshalb für diese sechs den Mechanismus, statt die Antwort zu schätzen. S1 misst am Objective-C-Laufzeitsystem über `AnyClass::responds_to`, welche Klassen die sechs Selektoren beantworten. Der Weg braucht keine Instanz, keinen Hauptfaden, kein Fenster und keinen Vordergrund und ist damit von einem Agenten fahrbar, anders als die Beobachtung am laufenden Bündel. Was er nicht entscheidet, sagt der Schritt ausdrücklich: für `undo:` und `redo:` liefert die Abfrage voraussichtlich `false`, weil der Rückgängigverwalter über die Kette erreicht wird, und ein `false` ist dort kein Beleg für "niemand antwortet".

## Acht Befunde am Code, in der Reihenfolge ihres Gewichts

Zwei nehmen Arbeit weg. Die Belegung des Betriebs liegt als Wert im Delegierten (`anwendung.rs:332`) und nicht im Rückgabewert von `fuer_den_betrieb()`, das genau einmal beim Start läuft; die Ausgabe leiht ihn und liest `keymap.toml` nicht erneut. Und weil die Belegungsansicht auf einer Kopie arbeitet (`anwendung.rs:2159`), fällt der gesicherte Stand bei offenem Blatt ohne einen einzigen Zweig an.

Drei legen den Schnitt fest. Sechs von 71 Funktionen tragen kein Kommando, und es sind genau die sechs mit `gehalten_von = "menue"`; nachgezählt am 260811-0838 mit 71 Blöcken `[[funktion]]`, 65 Paaren in `Kommando::KENNUNGEN` und sechs Zeilen `gehalten_von`. Der Menüeintrag geht an keiner der beiden Sperren vorbei, die KRK für Tastenbefehle hält, weil beide auf dem Weg zum Kommando sitzen; zwei Abnahmekriterien von C1 fallen daraus ohne Bau an. Und ein Modul ohne Aufrufer macht `make check` rot, weil `krk-ui` kein Bibliotheksziel hat und `make lint` mit `-D warnings` fährt; deshalb stehen Ausgabemodul und Auslöser in einem Schritt.

Drei beantworten je eine Planner-Frage. Das atomare Schreiben ist keine Abwägung, sondern Präzedenz: der Editor sichert über denselben Weg (`text/datei.rs:544-546`). Die Beschriftungen gehören zu ihrer Aufzählung und damit nach `krk-core`, wo auch der Übersetzerfehler bei einem achten Wert aufschlägt. Und der Fehlerfall wird am `io::ErrorKind` des Schreibens unterschieden statt an einer Vorabprüfung des Ordners, nach derselben Lehre, die dieses Projekt für die Typprüfung am Deskriptor gezogen hat.

## Wiederverwendung statt zweiter Aufbereitung

Die Directive schließt eine zweite Aufbereitung aus, und der Plan löst das an drei Stellen ein. Die Gruppierung nach Funktionsbereich wird aus `gliederung` zu `nach_bereichen` herausgezogen und bekommt damit zwei Abnehmer statt einer Kopie. Die Schreibweise der Kombinationen einer Funktion wird aus `tastentext` zu `tastenliste` herausgezogen, samt ihres Trenners. Die dritte Spalte fragt über `Kommando::aus_kennung`, statt die Liste der sechs zugestellten Befehle ein viertes Mal aufzuschreiben; eine Probe hält fest, dass "ohne Kommando" und "zugestellt" dieselbe Menge sind.

## Angelegte Datensätze

`decisions/260811-0838_o_schreibt-krk-einen-pfad-fuer-den-nutzer-je-gekuerzt.md`: der Spec überweist die Schreibweise des Pfades in der Erfolgsmeldung dem Planner und nennt sie eine Kleinigkeit. Sie ist zugleich eine Konvention, die dieses Projekt schon einmal gesetzt hat: der Fenstertitel aus C11 der Runde 2 kürzt ausdrücklich nicht, auf Nutzerverlangen vom 260809. Der Nutzer hat die Meldung mit Tilde genannt. Der Plan empfiehlt den ausgeschriebenen Pfad und legt die Wahl vor, weil sie beide Flächen desselben Fensters betrifft und nicht nur diese eine Meldung. Sie hält keinen Schritt auf.

`issues/260811-0838_o_antwort-zeigen-nennt-vier-raenge-die-statuszeile-fuehrt-fuenf.md`: der Doc-Kommentar von `antwort_zeigen` (`anwendung.rs:3290-3291`) nennt vier Ränge und verweist auf die Datei, die fünf führt. Die Aussage über den obersten Rang bleibt richtig, die Zahl ist bei der Erweiterung um den fünften Rang nicht mitgezogen worden. Der Plan fasst die Datei in S3 an, nimmt die Zeile aber ausdrücklich nicht mit.

## Was der Plan nicht entscheidet

Sechs Vorbelegungen des Specs bleiben am Gate, und der Plan legt zwei eigene daneben: die Überschrift der Datei ("Tastenbelegung von KRK") und die Stellung des Menüeintrags vor dem Beenden statt dahinter. Jede davon ist eine Zeichenkette oder eine Zeile.

Vier Größen sind am gebauten Bündel zu messen und stehen in S4 als `Nutzerarbeit`: die Rückfrage des Systems nach dem Zugriff auf den Downloads-Ordner samt ihrer Ablehnung, die Auswählbarkeit des Menüeintrags bei stehender Belegungsansicht, die Sichtbarkeit der Meldung in derselben Lage, und ob der Aufruf die Oberfläche sichtbar anhält.

## Prüfung dieser Sitzung

Kein Code, keine Datendatei und kein Datensatz des Specs ist angefasst worden. Geschrieben wurden vier Dateien: der Plan, der Entscheidungsdatensatz, der Defektdatensatz und dieser Bericht. Alle Zeilennummern und Zahlen im Plan sind am Baum nachgelesen und nicht aus dem Spec übernommen; drei Zitate wurden nach der Gegenprobe berichtigt.
