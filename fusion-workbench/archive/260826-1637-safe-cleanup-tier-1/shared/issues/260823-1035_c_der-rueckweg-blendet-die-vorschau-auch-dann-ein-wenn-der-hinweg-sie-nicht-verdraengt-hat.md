Der Rückweg blendet die Vorschau auch dann ein, wenn der Hinweg sie nicht verdrängt hat

---

Der Rückweg von `cmd+e` blendet die Vorschau bedingungslos ein. Hatte der Nutzer sie vorher mit
`f3` oder `cmd+y` ausgeschaltet, schaltet der Rückweg sie ihm wieder an — und der Grund, den der
Code für die Zeile angibt, trifft dann nicht zu.

---

**Aus dem Baum gelesen, am laufenden Bündel nicht bestätigt.** Der Abnahmelauf ist Nutzerarbeit.
Der Befund ist eine **Frage an den Nutzer** und nicht der Vorwurf eines Fehlers: die Zeile steht
so, wie der Entscheid vom 260823-0942 sie formuliert.

## Die Zeile und ihre Begründung

`crates/krk-ui/src/appkit/anwendung.rs:6862-6880`:

```rust
Anlass::EditorSchliessen { vorschau_danach } => {
    self.editor_ausblenden();
    // Der Rueckweg des Rundwegs holt die Vorschau zurueck, die sein
    // Hinweg verdraengt hat; `opt+cmd+e` laesst die Flaeche leer.
    …
    if vorschau_danach {
        let _ = self.bereich_einblenden(Bereich::Vorschau);
    }
}
```

Und am Feld selbst (`anwendung.rs:401-406`): „der Rueckweg von `cmd+e` holt die Vorschau
zurueck, denn er ist die **Umkehrung eines Hinwegs, der sie verdraengt hat**".

## Der Fall, in dem die Begründung nicht trägt

`f3` blendet die Vorschau aus. Danach `f4` oder `cmd+e` in der Dateiliste: der Editor bekommt
die Fläche, ohne dass er eine Vorschau verdrängt hätte — sie war schon aus. `cmd+e` im Editor
schließt ihn, und `bereich_einblenden(Bereich::Vorschau)` schaltet eine Vorschau ein, die der
Nutzer ausgeschaltet hatte.

Der Rückweg ist dann nicht die Umkehrung des Hinwegs, sondern eine Zutat.

## Was der Entscheid sagt

`shared/decisions/260820-1034_i_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`,
Abschnitt „Antwort des Nutzers, 260823-0942":

| Fokus | `cmd+e` tut |
|---|---|
| Editor | **schließt** den Editor, die Vorschau zeigt die Datei wieder, der Fokus geht in die Dateiliste |

„die Vorschau zeigt die Datei wieder" steht ohne Vorbehalt da. Der Code hält den Entscheid also
buchstabengetreu. Ob der Nutzer den Fall mitgedacht hat, in dem er die Vorschau selbst
ausgeschaltet hatte, geht aus dem Datensatz nicht hervor.

## Die Frage, und warum sie entscheidbar ist

Wollte man die Zeile bedingt machen, bräuchte der Rückweg die Auskunft „war die Vorschau vor dem
Hinweg sichtbar". Diese Auskunft hat das Programm heute nicht: `vorschau_danach` wird beim
Drücken des Rückwegs gesetzt und weiß nichts vom Hinweg. Sie ist aber **erhebbar** — das
Fenstermodell kennt die Sichtbarkeit zu jedem Zeitpunkt, sie müsste beim Einblenden des Editors
gemerkt werden. Die Frage ist damit entscheidbar und nicht anzunähern; was fehlt, ist der
Beschluss, ob sie gestellt werden soll.

Drei Möglichkeiten:

1. **Es bleibt, wie es ist.** Der Rückweg endet immer in derselben Lage, gleich wo er begonnen
   hat. Kostet den beschriebenen Fall; dafür ist die Regel in einem Satz erklärt.
2. **Der Hinweg merkt sich die Sichtbarkeit der Vorschau, der Rückweg stellt sie her.** Kostet
   einen Zustand mehr und die Frage, wann er verfällt (was gilt nach `opt+cmd+b` dazwischen?).
3. **Der Rückweg blendet nie ein und lässt die Fläche leer**, wie `opt+cmd+e`. Kostet die Zeile,
   die der Nutzer ausdrücklich verlangt hat.

**Schwere:** Low. Kein Datenverlust, kein Absturz, nichts Unumkehrbares — `f3` schaltet die
Vorschau wieder aus. Es ist eine Frage nach der gewollten Bedeutung des Rückwegs.

**Filed by:** coderev

---

In Arbeit: 260823-1137 durch coder. **Kein Verhalten geaendert**, und das ist die
Antwort auf einen Datensatz, der sich selbst als Frage an den Nutzer bezeichnet. Die
drei Moeglichkeiten sind mit ihren Folgen als Entscheidungsdatensatz vorgelegt:
`shared/decisions/260823-1137_o_holt-der-rueckweg-von-cmd-e-die-vorschau-auch-dann-zurueck-wenn-der-nutzer-sie-selbst-ausgeschaltet-hatte.md`.

Geaendert ist allein, was der Code ueber die Zeile behauptet. Beide Prosastellen sagten,
der Rueckweg sei „die Umkehrung eines Hinwegs, der sie verdraengt hat"; diese Begruendung
traegt nur fuer einen Teil der Faelle. Sie sagen jetzt die Regel, die wirklich gebaut
ist: der Rueckweg endet immer in derselben Lage, gleich wo er begonnen hat. Dazu steht
dort, warum die Unterscheidung aus der Lage beim Druecken nicht abzulesen ist — der
gegenseitige Ausschluss aus C1 haelt die Vorschau ausgeblendet, solange der Editor die
Flaeche hat, gleich aus welchem Grund — und der Verweis auf die offene Frage.

Der schwerste Einwand gegen Moeglichkeit 2 steht im Entscheidungsdatensatz und war im
Befund noch nicht ausgeschrieben: „der Hinweg" ist gar nicht wohldefiniert. Der Fokus
kommt auch ueber `f4`, ueber `opt+cmd+b` und ueber die Wiederherstellung aus der Sitzung
in den Editor, und aus jeder dieser Lagen ist der Rueckweg erreichbar; entweder merken
sich alle diese Wege die Sichtbarkeit, oder der Rueckweg findet keinen oder einen
veralteten Wert vor.

---
Dieser Datensatz bleibt in Arbeit. Das Verhalten ist mit `52fba42` **nicht** geändert; geändert ist
allein die überzogene Begründung im Code. Die Sachfrage liegt beim Nutzer als
`shared/decisions/260823-1137_o_holt-der-rueckweg-von-cmd-e-die-vorschau-auch-dann-zurueck-wenn-der-nutzer-sie-selbst-ausgeschaltet-hatte.md`,
und dort ist beim Ausschreiben der Folgen ein Einwand aufgetaucht, den dieser Datensatz nicht
führt: „der Hinweg" ist nicht wohldefiniert, denn der Fokus kommt auch über `f4`, `opt+cmd+b` und
die Sitzungswiederherstellung in den Editor.

---
Resolved: `52fba42`, entschieden am 260823-1235. Der Befund war richtig und die Behebung ist
nicht die, die er nahelegte: nicht das Verhalten war zu eng, sondern die Begründung im Code zu
weit. Der Nutzer hat das Verhalten bestätigt (Möglichkeit 1 von
`shared/decisions/260823-1137_i_holt-der-rueckweg-von-cmd-e-die-vorschau-auch-dann-zurueck-wenn-der-nutzer-sie-selbst-ausgeschaltet-hatte.md`),
und die überzogene Prosa ist durch die Regel ersetzt, die wirklich gebaut ist.
