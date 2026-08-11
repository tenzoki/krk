# Welche vier Kombinationen gelten ab Werk für die neuen Befehle?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md

---

## Question

Drei der vier neuen Funktionen brauchen eine Kombination ab Werk; die vierte, Cmd+W, behält ihre. Der Nutzer kann jede Belegung in der Belegungsansicht ändern, die Auslieferungsbelegung entscheidet aber, was ohne Zutun gilt, und C3 schließt zwei Funktionen auf einer Kombination aus.

Belegt sind heute 71 Funktionen. Frei sind unter anderem `return`, `cmd+b`, `cmd+i`, `cmd+k`, `cmd+l`, `cmd+m`, `cmd+o`, `cmd+p`, `cmd+u`, `shift+cmd+c`, `opt+cmd+c` und `shift+cmd+o`. Die Eingabetaste ist ab Werk unbelegt und vom Nutzer in C2 ausdrücklich freigegeben.

Cmd+C und Cmd+V sind ein Sonderfall. Beide liegen als `text_kopieren` und `text_einfuegen` mit `gehalten_von = "menue"` in der Belegung (`resources/default-keymap.toml:651` und `:658`), wirken also im Textfeld und im Editor über das Menü. Der Vorgang von Cmd+A zeigt, dass eine Kombination mit zwei Zustellern kein Konflikt ist, solange der Fokusvorbehalt die beiden nie zusammenbringt: in der Liste markiert Cmd+A alle Einträge, im Eingabefeld wählt es den Text aus.

## Options

1. **`return` öffnen, `shift+cmd+c` Eintragspfad, `opt+cmd+c` Ordnerpfad.**
   - Pro: die Eingabetaste zum Öffnen ist die Gewohnheit aus Norton Commander und dem Finder, und sie ist ausdrücklich freigehalten. `shift+cmd+c` ist die Kombination, die ForkLift für "Pfad kopieren" verwendet.
   - Contra: `opt+cmd+c` ist im Finder mit "Pfadname kopieren" belegt und dort auf den Eintrag gemünzt, nicht auf den Ordner. Wer beides nebeneinander benutzt, vertauscht sie leicht.
2. **`cmd+o` öffnen, `shift+cmd+c` Eintragspfad, `opt+cmd+c` Ordnerpfad.**
   - Pro: Cmd+O heißt auf dem Mac überall "Öffnen".
   - Contra: die freigehaltene Eingabetaste bleibt ungenutzt, und für den Griff aus der Liste heraus ist sie der kürzere Weg.
3. **`cmd+c` für den Eintragspfad, nach dem Vorbild von Cmd+A**, dazu `return` öffnen und `shift+cmd+c` Ordnerpfad.
   - Pro: entspricht dem Finder, wo Cmd+C auf einer Datei sie in die Zwischenablage legt, und der Vorgang von Cmd+A trägt die Konstruktion mit zwei Zustellern schon.
   - Contra: der Fokusvorbehalt muss die beiden zuverlässig trennen, sonst kopiert Cmd+C im Editor einen Pfad statt des markierten Textes. Cmd+A hat diese Trennung, ihr Vorliegen für Cmd+C ist aber ungeprüft.

## Constraints

- C3: zwei Funktionen auf einer Kombination sind ein Konflikt. Die Konflikterkennung im Code hält das fest, und die Menükürzel gehen seit `260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` mit hinein.
- Cmd+W bleibt auf `tab_schliessen`, Shift+Cmd+W auf `fenster_schliessen` (Nutzerentscheid vom 260811-1250).
- Eine Funktion darf mehrere Tasten tragen; `tasten` ist eine Liste.

## Recommendation

Option 1. Sie nimmt die einzige Taste, die eigens freigehalten wurde, für die Handlung, für die Nutzer sie erwarten, und sie lässt Cmd+C unangetastet, solange nicht gemessen ist, ob der Fokusvorbehalt dort so trennt wie bei Cmd+A.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Moeglichkeit 1.** Nutzerantwort am 260811-1505.

| Funktion | Kombination |
|---|---|
| mit dem Standardprogramm oeffnen | `return` |
| Pfad des betroffenen Eintrags kopieren | `shift+cmd+c` |
| Pfad des angezeigten Ordners kopieren | `opt+cmd+c` |

`cmd+w` bekommt keine neue Kombination — die vierte Funktion dieser Runde ist eine Erweiterung
des Wirkungsbereichs einer bestehenden Belegung, siehe
`260811-1257_*_wie-weit-soll-cmd-w-reichen.md`. Ab Werk kommen damit **drei** neue Kombinationen
hinzu, nicht vier.

`return` ist die Taste, die C2 der Runde 1 eigens freigehalten hat, und sie bekommt die Handlung,
fuer die der Nutzer sie erwartet. **`cmd+c` bleibt unangetastet**, solange nicht gemessen ist, ob
der Fokusvorbehalt es sauber vom Kopieren in Textfeldern und im Editor trennt; die dritte
Moeglichkeit haette genau diese ungemessene Groesse eingefuehrt.
