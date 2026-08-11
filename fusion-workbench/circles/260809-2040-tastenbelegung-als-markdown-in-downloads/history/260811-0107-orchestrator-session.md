# Orchestrator Session — 260811-0107

**Directive:** Die geltende Tastenbelegung als Markdown-Datei im Downloads-Ordner.
**Mode:** custom (Circle-Aktivierung)
**Status:** In Arbeit

## Aktivierung

Der Circle `260809-2040-tastenbelegung-als-markdown-in-downloads` ist am 260811-0107 auf
ausdrückliche Wahl des Nutzers aktiviert worden: der Datensatz ging von `_a_` auf `_t_`, und
`fusion-workbench/.active-circle` trägt den Verzeichnisnamen. Damit zeigen alle `OUT_*` in den
Circle; die `SCAN_*` decken den Circle und den gemeinsamen Speicher ab.

**Kein Plane-Push.** `plane.config.yaml` ist die unveränderte Vorlage (`plane.example.com`,
`your-workspace-slug`, Null-UUID), und `$PLANE_API_KEY` ist nicht gesetzt. Der Mirror ist damit
nicht eingerichtet, und der Push entfällt — nicht weil er fehlschlug, sondern weil es nichts zu
spiegeln gibt.

## Vorabprüfung zur fünften Frage

Der Datensatz
`decisions/260809-2040_*_welche-belegung-schreibt-die-ausgabe-bei-offener-belegungsansicht.md`
trägt eine ausdrückliche Aufforderung: ob ein Menüeintrag bei stehendem Blatt noch anschlägt, sei
nicht gemessen und vor der Antwort nachzusehen. Nachgesehen am 260811-0107:

- Die Belegungsansicht wird über `beginSheetModalForWindow_completionHandler`
  (`crates/krk-ui/src/appkit/blaetter/mod.rs:508`) gezeigt, also **dokumentmodal** und nicht über
  `runModal`. Eine eigene Ereignisschleife bringt sie nicht mit.
- Eine eigene `validateMenuItem`-Überschreibung gibt es im ganzen Baum nicht; die Suche liefert
  null Treffer.

`inference:`, nicht gemessen: ein dokumentmodales Blatt lässt die Menüleiste bedienbar, und ohne
eigene Prüfung der Menüeinträge schlägt ein solcher Eintrag an. Damit ist die fünfte Frage beim
Menüweg **nicht** gegenstandslos, sondern zu beantworten. Beim Belegungsweg bleibt sie es, weil
der Ereignisabgriff bei stehendem Blatt allein `abbrechen` durchlässt.

## Verlauf

- 260811-0107 — Circle aktiviert, Pfade neu aufgelöst, fünf offene Fragen dem Nutzer vorgelegt.

## Die sieben Klärungsfragen des Spec, beantwortet am 260811-0115

Der `shaper` hat sie zurückgegeben, statt sie selbst zu stellen: als Unteragent hat er kein
`AskUserQuestion` und hat das benannt, statt ein Werkzeug zu behaupten. Seine Vorarbeit liegt in
`history/260811-0446-shaper-klaerungsrunde-tastenbelegung-ausgabe.md`.

| # | Frage | Antwort |
|---|---|---|
| 1 | Dateiname | `KRK-Tastenbelegung.md` |
| 2 | Kopfzeile | nur eine Überschrift, kein Zeitstempel |
| 3 | Beschriftung der dritten Spalte | ausgeschrieben, ohne Legende |
| 4 | dritte Spalte bei den sechs Textbefehlen | „Textfelder und Editor" |
| 5 | Erfolgsmeldung | mit vollem Pfad |
| 6 | Überschreiben melden? | nein, eine Meldung für beide Fälle |
| 7 | gesicherten Stand melden? | **nein** |

**Zwei Antworten tragen einen Vorbehalt, und der gehört in den Spec statt in die Erinnerung.**

Antwort 4 steht auf einer Ableitung. Dass die sechs Textbefehle „in Textfeldern und im Editor"
wirken, ist aus dem Aufbau der Antwortkette geschlossen und **nicht gemessen**. Der Spec
kennzeichnet es als Annahme, und wer den Planschritt baut, prüft es zuerst — eine Spalte, die
eine falsche Zusicherung gibt, ist schlimmer als eine leere. Dieselbe Fehlerform hat die
Durchsicht in der vorigen Sitzung zweimal gefunden.

Antwort 7 weicht von der Empfehlung ab, und der Preis ist damit angenommen: **wer bei offener
Belegungsansicht ausgibt, bekommt eine Datei, die dem Schirm widerspricht, ohne es zu
erfahren.** Die Abweichung besteht nur bis zum Sichern und entsteht allein durch eine eigene
Handlung des Nutzers. Der Spec schreibt den Fall aus, statt ihn wegzulassen.

**Was der Spec ohne Rückfrage festhält**, vom `shaper` am Code erhoben: die Schreibweise der
Kombinationen kommt aus `anzeige()` (`crates/krk-ui/src/belegungsmodell.rs:527`), also
`Shift+Cmd+K` und `F3` — eine abweichende Schreibweise wäre die zweite Aufbereitung, die die
Directive ausschließt. Keine der vier bestehenden vollständigen Fallunterscheidungen wächst; die
Belegung bleibt bei 71 Funktionen, weil der Menüeintrag kein Kommando mitbringt. Neu hinzu kommt
dagegen **eine** vollständige Fallunterscheidung: die Beschriftung der sieben Wirkungsbereiche,
zu bauen ohne Auffangzweig, wie `Funktionsbereich::name()` es vormacht.
