Cmd+A liegt schon auf "Alle Einträge markieren", und S13b vergibt es ein zweites Mal

---

S13b trägt fünf Menükürzel in `resources/default-keymap.toml` nach. Vier davon
sind frei. Das fünfte, `cmd+a` für `text_alles_auswaehlen`, gehört seit S9
der Funktion `alle_markieren`. Der Schritt lässt sich deshalb nicht ausführen,
ohne entweder eine Kombination doppelt zu vergeben oder eine bestehende
Belegung zu ändern. Beides schließt sein eigenes Abnahmekriterium aus.

---

## Der Nachweis

`resources/default-keymap.toml:184-187`:

```toml
[[funktion]]
id = "alle_markieren"
name = "Alle Einträge markieren"
tasten = ["cmd+a"]
```

Die Zeile steht dort seit dem Anlegen der Datei in S9, Commit `d1a8ab1`
(`git log -L '/id = "alle_markieren"/,+3:resources/default-keymap.toml'` zeigt
genau einen Treffer, das Anlegen). Sie hat nie `ctrl+a` getragen.

Vollständige Prüfung der ausgelieferten Datei am 260805-0637, am ganzen Eintrag
und nicht als Teilzeichenkette, über alle 50 Funktionen und 57 Kombinationen:

| Kombination | Stand vor S13b |
|---|---|
| `shift+cmd+w` | frei |
| `cmd+x` | frei |
| `cmd+c` | frei |
| `cmd+v` | frei |
| `cmd+a` | **`alle_markieren`** |
| `ctrl+a` | frei |

## Wo der Plan die Sache anders sieht

`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 13b,
Absatz "Die fünf Kombinationen sind gegen die 57 ausgelieferten geprüft, nicht
angenommen", schreibt:

> Nachgesehen am 260805-0000 in `resources/default-keymap.toml`: `shift+cmd+w`,
> `cmd+x`, `cmd+c`, `cmd+v` und `cmd+a` kommen dort in keiner Tastenliste vor.
> Nah daran und belegt sind `cmd+w` (Tab schließen), `shift+cmd+v`
> (Verschieben), `cmd+y` (Vorschau) und `alle_markieren` auf `ctrl+a`.

Beide Hälften des Satzes sind für `cmd+a` falsch: es kommt vor, und
`alle_markieren` steht nicht auf `ctrl+a`. Die übrigen vier Angaben stimmen.
Derselbe Absatz nennt außerdem `cmd+y` und `shift+cmd+v` als "nah daran" —
das ist genau die Teilzeichenketten-Nähe, vor der das Abnahmekriterium warnt,
und dort war die Prüfung richtig. Der eine Fall, in dem eine Kombination
tatsächlich vollständig übereinstimmt, ist durchgerutscht.

## Warum es zählt

Drei Zusagen stehen gegeneinander, und keine zwei lassen sich zugleich halten:

1. **Das Abnahmekriterium von S13b** verlangt, dass keine Kombination bei zwei
   Funktionen erscheint, und zugleich, dass sich die Tastenliste keines
   vorhandenen Blocks ändert.
2. **Spec C3, Abnahmekriterium** (`planning/260802-1036_o_spec-navigator-geruest.md`,
   Zeile 171): "Cmd+X, Cmd+C, Cmd+V und Cmd+A tragen ab Werk allein die
   Textbefehle des Menüs 'Bearbeiten' und **keine Funktion des Dateifensters**."
   `alle_markieren` ist eine Funktion des Dateifensters.
3. **Spec C3, Abnahmekriterium**: die Auslieferungsbelegung ist in sich
   konfliktfrei.

Maschinell schlägt das sofort durch. `AUSLIEFERUNG` in
`crates/krk-core/src/tasten/belegung.rs:64-69` liest die Datei über
`include_str!` und ruft `Belegung::bauen`, das bei einem Konflikt
`Belegungsfehler::Konflikt` liefert; das `expect` daneben lässt jeden Test und
jeden Programmstart abstürzen, nicht nur die Prüfung
`die_auslieferungsbelegung_ist_konfliktfrei`.

## Was zu tun ist

Eine Nutzerentscheidung, kein Textfehler. Drei Wege, und der Plan trägt keinen
davon:

1. **`alle_markieren` zieht um**, `cmd+a` geht an den Textbefehl. Der Plan
   glaubte ohnehin an `ctrl+a`, und `ctrl+a` ist frei. Preis: die Nachbarschaft
   der Markierungsbefehle zerfällt, denn `markierung_aufheben` steht auf
   `shift+cmd+a` und bliebe als einziger bei `cmd`. Der Umzug ändert außerdem
   eine Belegung, die der Nutzer am 260803-2110 durchgesehen und angenommen hat
   (`decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`).
2. **Der Textbefehl bekommt kein `cmd+a`.** Bricht C3 Zeile 171 und lässt ein
   Menükürzel außerhalb der Konflikterkennung stehen — genau den blinden Fleck,
   den der Entscheid vom 260805-0000 schließen wollte.
3. **Die Konflikterkennung lernt den Fokusvorbehalt.** Eine Funktion mit
   `gehalten_von = "menue"` und eine Funktion des Dateifensters wären dann kein
   Konflikt, weil sie sich nie begegnen. Das ist dieselbe Begründung, mit der
   der Entscheid vom 260805-0000 Cmd+C und Cmd+V nebeneinander bestehen lässt.
   Es widerspricht aber dem Abnahmekriterium von S13c, das ausdrücklich
   verlangt, die Konflikterkennung melde einen Konflikt "auch dann, wenn eine
   der beiden beteiligten Funktionen vom Menü gehalten wird".

Möglichkeit 3 ist die einzige, die keine Zusage bricht, sondern eine
Formulierung korrigiert, und sie ist zugleich die einzige, die den Fall Cmd+C
nicht ein zweites Mal aufwirft, wenn die Dateizwischenablage einer späteren
Runde `copy:` am Dateifenster beantwortet. Sie greift dafür in Code ein, den
S13c ohnehin anfasst. Eine Empfehlung ist das nicht, dafür wiegt der
Widerspruch zum Abnahmekriterium von S13c zu schwer; die Wahl gehört dem
Nutzer und braucht einen eigenen Entscheidungsdatensatz.

## Dringlichkeit

Bindet S13b und S13c. S13b ist mit vier der fünf Einträge ausgeführt; der
fünfte fehlt, und die Datei zählt deshalb 54 statt der zugesagten 55
Funktionen. S13c kann den Menüeintrag "Alles auswählen" nicht bauen, solange
offen ist, welche Kombination er trägt.

---

**Aufgefallen bei:** der Ausführung von S13b am 260805-0637, bei der Prüfung
der fünf Kombinationen gegen den vollständigen Dateibestand vor dem Schreiben.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S13b, S13c),
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C3),
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`
