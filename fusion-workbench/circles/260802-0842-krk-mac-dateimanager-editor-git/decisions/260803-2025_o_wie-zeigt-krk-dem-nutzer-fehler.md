# Wie zeigt KRK dem Nutzer einen Fehler, den er sehen muss?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-1536_c_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`, Spec C1 (Statuszeile), `circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md` Befund M3

---

## Question

KRK hat heute zwei echte Fehlerpfade, und beide melden sich allein über `eprintln!`. Ein über den Finder oder über `open` gestartetes Bündel hat keinen Standardfehler, LaunchServices hängt ihn ins Leere. In der einzigen Betriebsart, die die Abnahme zulässt, ist die Fehlerbehandlung damit still.

Die beiden Stellen:

- **Der Ordner ohne Leserecht.** `crates/krk-ui/src/appkit/tabelle.rs`, im Zweig `Meldung::Fertig`. `Schwungleser::oeffnen` liefert `Abschluss::Fehler` ohne einen einzigen Eintrag; der Nutzer steigt mit Return hinein und sieht eine leere Liste, die er von einem wirklich leeren Ordner nicht unterscheiden kann.
- **Der nicht eingerichtete Tastenabgriff.** `crates/krk-ui/src/appkit/anwendung.rs`, im `None`-Zweig. Der Kommentar daneben sagt richtig, dass eine Anwendung ohne Tastatursteuerung nicht still ausgeliefert werden darf, und wählt dann den einen Kanal, der still ist.

Warum jetzt: der Prüfbericht des Durchstichs hat den Defekt gemeldet, und der `coder` hat ihn am 260803-2025 bearbeitet. Er ist keine Reparatur, weil jede Antwort eine Oberfläche braucht, die es in Runde 1 noch nicht gibt, und weil die zweite Stelle eine Verhaltensfrage ist, die dem Nutzer gehört. Der Defekt ist deshalb mit einem Verweis auf diesen Datensatz geschlossen.

## Options

1. **Statuszeile am Fuß des Dateifensters, plus Abbruch mit `NSAlert` beim fehlenden Tastenabgriff** — die Richtung, die der Defektdatensatz vorzeichnet.
   - Pro: Die Statuszeile verlangt C1 ohnehin, und sie trägt später auch den Lesefortschritt und die Zahl der Einträge. Der fehlende Tastenabgriff ist ein Startfehler und kein Zustand, in dem KRK sinnvoll weiterläuft; ein Abbruch mit Meldung ist die Haltung, die das Projekt bei `--kalt` und bei `NSScreen` schon einnimmt.
   - Contra: Zwei verschiedene Wege für zwei Fehler. Der Schritt, der die Statuszeile baut, ist noch nicht zugeordnet.
2. **Nur die Statuszeile, auch für den Tastenabgriff** — ein Kanal für alles.
   - Pro: Eine Stelle, an der Fehler stehen, und der Nutzer verliert das Fenster nicht.
   - Contra: Eine Zeile am Fuß eines Fensters ist die falsche Lautstärke für "die Tastatursteuerung, deine erste Maxime, gibt es nicht". Der Nutzer übersieht sie und arbeitet mit einer halben Anwendung.
3. **Beides über einen modalen `NSAlert`** — kein neues Bedienelement.
   - Pro: Am wenigsten Bauarbeit; kein Eingriff in das Fensterlayout.
   - Contra: Ein Ordner ohne Leserecht ist beim Durchblättern ein Alltagsfall. Ein modaler Dialog je Fehlversuch steht der Maxime "superschnell" entgegen.

## Constraints

- Der Weg über `eprintln!` scheidet für beide Stellen aus: er ist im Bündel nachweislich still, festgehalten in `issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`.
- C1 verlangt die Statuszeile ohnehin; eine zweite Anzeigefläche daneben wäre eine zweite Wahrheit.
- Was die Antwort auch ist, sie braucht einen Schritt im Plan, der sie baut. Heute nennt kein Schritt sie.

## Recommendation

Möglichkeit 1, mit einer Ergänzung: die Statuszeile trägt den Ordnerfehler, und der fehlende Tastenabgriff bricht mit `NSAlert` ab. Die beiden Fehler sind nicht von derselben Art. Der eine betrifft ein Ziel, das der Nutzer gerade nicht erreicht, und er arbeitet weiter; der andere betrifft die Anwendung als ganze, und alles, was danach kommt, ist eine Täuschung. Für die Zuordnung: der Ordnerfehler gehört an den Schritt, der die Statuszeile baut, der `NSAlert` an einen eigenen kleinen Schritt neben S6, weil er nur den `None`-Zweig in `anwendung.rs` betrifft.

---
Answered:
Implemented:
Deferred:
Superseded by:
