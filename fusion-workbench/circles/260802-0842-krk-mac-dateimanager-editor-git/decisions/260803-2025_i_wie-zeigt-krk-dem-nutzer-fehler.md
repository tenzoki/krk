# Wie zeigt KRK dem Nutzer einen Fehler, den er sehen muss?

---
**Domain:** code
**Status:** answered
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

## Antwort des Nutzers, 260804-0830

**Möglichkeit 1 gewählt, wie empfohlen.** Eine Statuszeile am Fuß des
Dateifensters trägt die laufenden Fehler; allein der fehlende Tastenabgriff
bricht mit einem `NSAlert` ab.

### Eine Annahme dieses Datensatzes hält nicht: C1 verlangt die Statuszeile nicht

Der Pro-Punkt von Möglichkeit 1 und die dritte Randbedingung sagen beide, C1
verlange die Statuszeile ohnehin. **Das stimmt nicht.** Der Spec nennt an keiner
Stelle eine Statuszeile: die sechs Abnahmekriterien von C1 regeln Tabs, aktives
Fenster, Standardordner, Sitzungswiederherstellung und getrennte Auswahl, und
eine Textsuche über den ganzen Spec am 260804 findet weder "Statuszeile" noch
"Lesefortschritt" noch eine Zusage über die Zahl der Einträge. Die Behauptung
stammt aus dem Defektdatensatz
`issues/260803-1536_c_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md`
und ist von dort in diesen Datensatz gewandert, ohne dass jemand sie gegen den
Spec gehalten hat.

Für die Wahl ändert das nichts, für ihre Buchführung schon: die Statuszeile ist
eine **Erweiterung des Umfangs** dieser Runde und keine Präzisierung einer
bestehenden Zusage. Sie steht deshalb ab 260804 als eigenes Abnahmekriterium in
C1, mit dem Vermerk, woher sie kommt. Die zweite Randbedingung des Datensatzes
bleibt davon unberührt und wird durch die Aufnahme in C1 erst wahr: eine zweite
Anzeigefläche neben der Statuszeile wäre eine zweite Wahrheit.

### Die Zuordnung, die dieser Datensatz offengelassen hat

**Die Statuszeile baut S12.** Das ist der Schritt, der die vier Bereiche und die
beiden Dateifenster anlegt; die Statuszeile sitzt am Fuß des Dateifensters und
gehört damit in dieselbe Ansicht. Ein eigener Schritt daneben hätte für das
Layout des Dateifensters eine zweite Partei geschaffen, und die erste Abweichung
zwischen beiden fände keine Prüfung.

**Der Kern hört damit auf, selbst auszugeben.** `crates/krk-core/src/ablage/mod.rs`
bündelt den Ausgabeweg bereits in `ablage::melden`, der einzigen Stelle im Kern
mit `eprintln!`; der Modulkopf dort nennt diesen Datensatz und sagt zu, die
Antwort koste eine Zeile und nicht fünf. S12 löst das ein: `melden` reicht die
Meldung an die Statuszeile, und `krk-core` schreibt danach nichts mehr auf die
Standardfehlerausgabe. Denselben Weg nimmt der unvollständig gelesene Ordner aus
`crates/krk-ui/src/appkit/tabelle.rs`.

**Der `NSAlert` bekommt einen eigenen kleinen Schritt, S6b.** Er betrifft allein
den `None`-Zweig in `crates/krk-ui/src/appkit/anwendung.rs` und hängt an keinem
Bauteil aus S12. Als Nachtrag zu S6 ist er der nächste unausgeführte Schritt der
Nummernfolge und kann sofort laufen; S6 selbst bleibt abgenommen und unverändert.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 12 für die Statuszeile, Schritt 6b für den `NSAlert`) — Möglichkeit 1; die Statuszeile ist dabei eine Erweiterung des Umfangs und steht ab 260804 als eigenes Abnahmekriterium in C1.
Implemented:
Deferred:
Superseded by:

---
Implemented: S6b (dieser Commit) — die Statuszeile trug S12, der NSAlert beim fehlgeschlagenen Tastenabgriff steht jetzt in crates/krk-ui/src/appkit/hinweis.rs; beide None-Zweige des Tastenabgriffs in anwendung.rs zeigen ihn und beenden danach die Anwendung. Damit ist Möglichkeit 1 aus diesem Datensatz vollständig umgesetzt.
