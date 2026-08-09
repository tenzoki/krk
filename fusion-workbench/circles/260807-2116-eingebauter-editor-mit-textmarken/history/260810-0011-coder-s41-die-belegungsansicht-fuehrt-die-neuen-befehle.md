# S41: Die Belegungsansicht führt die neuen Befehle

**Status:** Complete
**Ausführender:** coder
**Datum:** 260810-0011
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken

## Auftrag

S41 aus `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
umsetzen. Der Schritt ist als Abnahme angelegt und nicht als Bau: er prüft, dass
die zwölf Kommandos aus S5 und die dreizehn Funktionen aus S6 in der
Belegungsansicht ankommen, weil das achte Abnahmekriterium von C7 eine Zusage
über diese Ansicht macht und sie sonst niemand prüft.

Zwei Punkte hat der Auftrag ausdrücklich mitgegeben: ob die Ansicht die zwölf
Zweige aus `bereich_des_kommandos` wirklich alle und im richtigen
Funktionsbereich zeigt, und ob die Beschriftung nach S2 noch das Richtige
anzeigt — auf einer deutschen Tastatur ist `cmd+y` nicht mehr die Taste, auf der
`y` steht.

Ausgeklammert und nicht angefasst, weil dort ein paralleles Bündel arbeitet:
`appkit/anwendung.rs`, `appkit/editor.rs`, `editormodell.rs`. Keine der drei war
nötig.

## Das Ergebnis: die Vermutung des Schrittes hat gehalten

Der Plan sagt „Vermutlich keine [Änderungen]", und das trifft zu. Kein
Programmteil war zu ändern, weder in `belegungsmodell.rs` noch in
`appkit/belegungsansicht.rs`. Vier Proben sind hinzugekommen, und alle vier
waren im ersten Lauf grün.

Das ist ein Befund und kein Nichtergebnis: die Zusage von C7 stand bis eben auf
niemandes Prüfbestand. `jede_kennung_hat_einen_funktionsbereich` fängt eine
Kennung **ohne** Bereich, bevor `gliederung` am laufenden Blatt abbricht; sie
sagt nichts darüber, ob eine Kennung im **richtigen** Bereich landet. Genau
diese Hälfte steht jetzt da.

## Was geprüft wurde

### Die zwölf Befehle stehen unter „Editor"

`der_bereich_editor_fuehrt_die_zwoelf_befehle_der_runde` liest die Gliederung so,
wie der Nutzer sie liest: ab der Überschrift „Editor" bis zur nächsten
Überschrift. Der Helfer `funktionen_unter` misst damit die Gliederung und nicht
die Zuordnung, aus der sie entsteht.

Die zwölf Kennungen stehen **ausgeschrieben** und sind nicht aus
`bereich_des_kommandos` abgeleitet. Eine Ableitung prüfte die Zuordnung gegen
sich selbst und liefe mit jedem Umzug stillschweigend mit; die Probe soll gerade
den Umzug melden. Geprüft wird in beide Richtungen: jede der zwölf Kennungen hat
eine Zeile unter „Editor" und trägt mindestens eine Kombination, und der
Abschnitt führt genau zwölf Zeilen, also schiebt sich keine dreizehnte
Funktion dazwischen.

`die_beiden_neuen_textbefehle_stehen_unter_textbefehle` hält die Gegenprobe fest:
Rückgängig und Wiederholen kommen aus dieser Runde, gehören aber nicht dem
Editor. Das Menü „Bearbeiten" stellt sie zu, und im Textfeld wirken sie genauso
wie im Editor. Der Abschnitt „Textbefehle" führt danach sechs Funktionen.

### Jede der dreizehn neuen Kennungen ist umbelegbar

`jede_neue_kennung_der_editor_runde_ist_umbelegbar` weist jeder der dreizehn
Kennungen aus S6 die freie Taste F9 zu, jede auf einem frischen Modell, und
erwartet `Zuweisung::Zugewiesen` samt der Kombination in der Zeile.

Der Weg geht über `Belegungsmodell::zuweisen` und nicht direkt über
`Belegung::zuweisen`. Das ist der Grund, aus dem die Proben in
`crates/krk-ui/src/belegungsmodell.rs` stehen und nicht in
`crates/krk-core/tests/belegung.rs`, wie die Dateizeile des Plans es vorsah:
allein das Modell bildet eine **Zeilennummer der Ansicht** auf ihre Funktion ab,
und die Zusage von C7 gilt der Ansicht. Eine Probe im Kern prüfte den
Zuweisungsweg und ließe die Gliederung ungeprüft.

Die beiden Textbefehle stehen mit in der Liste, obwohl das Menü sie zustellt.
Die Zusage von C3 gilt jeder Kombination, die in KRK etwas auslöst, gleich wer
sie zustellt, und `Belegung::zuweisen` vergleicht den Konflikt je Zusteller —
F9 ist auf beiden Seiten frei.

### Die Beschriftung nennt seit S2 die Aufschrift

Der zweite Punkt des Auftrags hat sich als eine Frage erwiesen, deren Antwort
nicht in diesem Schritt liegt, sondern in S2, und die genau deshalb festzuhalten
war.

Die Ansicht schreibt den **Namen** der Taste auf, über
`Kombination::to_string` und die eine Tastentabelle des Kerns. Bis S2 benannte
ein einbuchstabiger Name eine **Stelle**: die Zeile las sich `Cmd+Y`, und auf
einer deutschen Tastatur wirkte die Kombination unter der Aufschrift Z. Seit S2
benennt derselbe Name das **Zeichen** (`Taste::kennung` →
`Tastenkennung::Zeichen`), und der Ereignisabgriff schlägt Buchstaben über
dasselbe Zeichen nach. Dieselbe Zeile ist damit wahr geworden, ohne dass dieses
Modul einen Zweig dafür bekommen hätte.

`die_beschriftung_nennt_die_taste_auf_einer_deutschen_tastatur` misst es in
beiden Richtungen und dazu als Regel:

- Die Zeile von `vorschau_umschalten` schreibt `Cmd+Y`.
- Ein Druck auf `kVK_ANSI_Z` mit gemeldetem Zeichen `y` — das ist die Taste mit
  der Aufschrift Y auf einer deutschen Tastatur — ergibt `Cmd+Y`.
- Die Gegenprobe: ein Druck auf `kVK_ANSI_Y` mit gemeldetem `z` ergibt `Cmd+Z`
  und schaltet die Vorschau nicht um.
- Und die Regel statt des Einzelfalls: keine Kombination der
  Auslieferungsbelegung, deren Taste über ihr Zeichen nachgeschlagen wird, trägt
  in der Anzeigeform ein anderes Zeichen als dieses.

Die ersten drei wären vor S2 gefallen. Der Modulkopf von `belegungsmodell.rs`
trägt den Zusammenhang jetzt aus, damit niemand die Beschriftung für einen
Zufall hält.

### Die Ansicht selbst

`appkit/belegungsansicht.rs` ist unverändert und war es zu Recht. Die
Bereichsüberschriften kommen als Gruppenzeilen über `tableView:isGroupRow:`,
sind über `tableView:shouldSelectRow:` nicht auswählbar, und die Anfangsauswahl
steht auf der ersten Funktionszeile. Der Kommentar an `TABELLENHOEHE` nennt neun
Bereichsüberschriften und stimmt.

**Eine Beobachtung ohne Handlungsbedarf, damit sie nicht verlorengeht:** die
Tabelle zeigt fünfzehn Zeilen ohne Rollen, und die Ansicht führt inzwischen 71
Funktionen und 9 Überschriften, also 80 Zeilen. Der Abschnitt „Editor" steht
zuunterst. Das ist kein Defekt — die Zusage ist „aufgeführt und umbelegbar", und
der Rollbalken ist da —, aber der Nutzer rollt für die neuen Befehle bis ans
Ende. Ob die Beigabe höher werden soll, ist eine Gestaltungsfrage und keine, die
dieser Schritt beantworten darf.

## Ein Defekt gefiled

`issues/260810-0011_o_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md`,
Ausführender `ontocoder`, weil `.toml` nicht dem `coder` gehört.

`resources/default-keymap.toml` begründet an zwei Stellen (Zeilen 484–492 und
617–628), warum die elf Editor-Kombinationen `y` und `z` meiden: „der
Ereignisabgriff schlägt über den virtuellen Tastencode nach, also über die
Stelle auf der Tastatur". Das hat S2 abgelöst. Der Modulkopf von
`appkit/ereignisse.rs` ist mitgezogen worden, die Belegungsdatei nicht — und sie
ist nach C7 und C11 der Runde 1 von Hand lesbar, also Nutzerdokumentation.

Der Defekt gehört zu zweien, die schon offen sind
(`260809-1746_o_…wandernden-stellen…`, `260809-1527_o_…verbietet-y-und-z…`);
alle drei ziehen dieselbe gegenstandslos gewordene Begründung aus einem anderen
Dokument, und einzeln erledigt bleibt sie an den übrigen Orten stehen.

## Geänderte Dateien

- `crates/krk-ui/src/belegungsmodell.rs` — vier Proben, ein Prüfhelfer
  (`funktionen_unter`), ein Absatz im Modulkopf über die Beschriftung nach S2.
  Kein Programmteil.

## Abnahme

Alle vier Kommandos mit Rückgabewert 0:

```
cargo build --workspace              exit=0
cargo test --workspace               exit=0    (u. a. 17 Proben in belegungsmodell)
cargo clippy --workspace --all-targets exit=0
cargo fmt --all --check              exit=0
```

## Nutzerarbeit

Der Plan verlangt sie ausdrücklich, und keine Probe ersetzt sie: `f1` zeigt die
Belegungsansicht, und der Abschnitt „Editor" ganz unten führt zwölf Befehle mit
ihren Kombinationen — Bearbeiten (F4), Im Editor bearbeiten (Cmd+E), Fokus in
den Editor (Shift+Cmd+E), Editor schließen (Opt+Cmd+E), Zwischen Roh- und
Formatansicht wechseln (Ctrl+Cmd+E), Sichern (Cmd+S), Zu Zeile springen
(Cmd+J), Im Text suchen (Cmd+F), Weitersuchen (Cmd+G), Rückwärts weitersuchen
(Ctrl+Cmd+G), Ersetzen (Shift+Cmd+R), Alle ersetzen (Ctrl+Cmd+R). Unter
„Textbefehle" stehen sechs, darunter Rückgängig (Cmd+Z) und Wiederholen
(Shift+Cmd+Z).

Zwei Dinge, die nur am laufenden Bündel zu sehen sind: dass jede dieser
Kombinationen unter der **Aufschrift** liegt, die die Ansicht nennt, und dass
„Zuweisen" eine gedrückte Kombination in derselben Schreibweise aufnimmt.
