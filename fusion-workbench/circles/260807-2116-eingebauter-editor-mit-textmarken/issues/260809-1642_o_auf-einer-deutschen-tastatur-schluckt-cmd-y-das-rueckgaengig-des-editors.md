# Auf einer deutschen Tastatur schluckt `cmd+y` das Rückgängig des Editors

---
**Domain:** code
**Schwere:** High
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `resources/default-keymap.toml:98-101`, `resources/default-keymap.toml:660-673`, `crates/krk-ui/src/appkit/menue.rs:209-223`
**Cross-references:** `decisions/260808-0140_a_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`, `issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`, S2 (offen), S7 (erledigt)

---

## Der Befund

S7 hat "Rückgängig" auf `cmd+z` und "Wiederholen" auf `shift+cmd+z` gelegt, als
Menükürzel über das **Zeichen**. S2, der die Nachschlagart des Abgriffs von der
Taste auf das Zeichen umstellt, ist **nicht** umgesetzt. Der Abgriff schlägt
weiterhin über den virtuellen Tastencode nach, also über die **Stelle** auf der
Tastatur.

Auf einer deutschen Tastatur sind Z und Y vertauscht. Damit gilt heute:

| Der Nutzer drückt | Tastencode | Der Abgriff findet | Ergebnis |
|---|---|---|---|
| Cmd + Taste mit der Aufschrift **Z** | `kVK_ANSI_Y` (16) | `cmd+y` → `vorschau_umschalten` | Die Vorschau klappt auf oder zu; das Ereignis ist verbraucht |
| Shift+Cmd + Taste mit der Aufschrift **Z** | `kVK_ANSI_Y` (16) | `shift+cmd+y` → `fokus_vorschau` | Der Fokus springt in die Vorschau; das Ereignis ist verbraucht |
| Cmd + Taste mit der Aufschrift **Y** | `kVK_ANSI_Z` (6) | unbelegt, geht durch | Das Menü sieht das Zeichen `z` und führt `undo:` aus |

Der Abgriff sitzt vor dem Hauptmenü: `NSApplication` ruft die Beobachter aus
`addLocalMonitorForEventsMatchingMask:` auf, bevor `sendEvent:` das Kürzel des
Hauptmenüs prüft. Wer das Ereignis dort verbraucht, nimmt es dem Menü weg.

**Beide Einträge, die S7 gebaut hat, sind auf dem Referenzgerät nicht unter
ihrer Aufschrift erreichbar.** Rückgängig liegt unter der Taste mit der
Aufschrift Y, und die Taste mit der Aufschrift Z blendet die Vorschau um.

## Warum das zählt

`decisions/260808-0140_a_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`
beschreibt genau diesen Zusammenstoß und empfiehlt Weg 2 **ausdrücklich nicht**,
mit der Begründung:

> der Plan dieser Runde legt Rückgängig auf `cmd+z` als Menüeintrag, und der
> Menüeintrag schlägt über das Zeichen an.

Der Nutzer hat am 260808-0155 Möglichkeit 3 gewählt: Buchstaben und Ziffern
werden künftig über das gemeldete Zeichen nachgeschlagen. Das ist S2. Solange S2
offen ist, gilt der Zustand, vor dem der Datensatz gewarnt hat — und S7 hat ihn
nicht abgewartet, sondern hergestellt.

Das ist ein anderer Befund als
`issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`:
jener hält fest, dass der Plan sich selbst widerspricht. Dieser hält fest, dass
die Anwendung heute auf dem Gerät des Nutzers zwei Menüeinträge trägt, die nicht
greifen.

## Vorschlag

Zwei Wege, und nur einer davon ist neu:

1. **S2 vorziehen.** Die Reihenfolge im Plan hat S7 vor S2 gestellt; sachlich
   hängt S7 an S2. Nach S2 schlägt der Abgriff `cmd+y` über das Zeichen nach,
   die Taste mit der Aufschrift Z liefert `z`, findet keine Funktion und läuft
   ins Menü. Der Zusammenstoß löst sich ohne eine Änderung an der Belegung.
2. **Bis dahin als bekannte Einschränkung führen.** Der Abnahmelauf S42 ist
   Nutzerarbeit; wer dort "Rückgängig im Editor" prüft, muss wissen, unter
   welcher Taste es heute liegt.

Nicht empfohlen: `vorschau_umschalten` von `cmd+y` wegzunehmen. Der Datensatz
nennt das Weg 2 und rät davon ab, und die Belegung führt das y für die Vorschau
an drei Stellen mit ausgeschriebener Begründung.

Gemeldet von: `coderev`, Durchsicht Turn 2.
