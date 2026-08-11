# Was trägt die dritte Spalte bei Rückgängig und Wiederholen?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (nachgetragen — die Antwort fiel am 260811-0935, der Datensatz entstand am 260811-1010)
**Cross-references:** `issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-fuer-alles-auswaehlen-*.md`,
`issues/260811-0956_*_der-nutzerentscheid-vom-260811-0935-steht-allein-im-programmtext-*.md`,
`planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md` (C3),
`crates/krk-ui/src/appkit/editor.rs:3376`, `crates/krk-ui/src/belegungsausgabe.rs`

---

## Warum dieser Datensatz nachträglich entsteht

**Er hätte am 260811-0935 entstehen müssen und tat es nicht.** Der Orchestrator hat die Frage
dem Nutzer gestellt, die Antwort bekommen und sie in Commit-Nachrichten und Code-Kommentare
geschrieben — aber in keinen Datensatz. Die Durchsicht von Turn 1 hat das gefunden: der
Programmtext beruft sich an drei Stellen auf einen „Nutzerentscheid vom 260811-0935", den der
Arbeitsbereich nirgends führt (`issues/260811-0956_*_…`). Dieser Datensatz holt das nach und ist
damit selbst der Beleg dafür, dass eine Antwort, die nur im Code steht, keine festgehaltene
Antwort ist.

## Frage

Die Messung aus Schritt S1 hat die Ableitung des Specs für die sechs vom Hauptmenü zugestellten
Textbefehle in drei Teile zerlegt. Für `text_rueckgaengig` und `text_wiederholen` gab sie
**keine** Antwort: beide Selektoren stehen an `NSWindow` und nicht an der Textklasse, und
`responds_to` liefert `false` für einen weitergeleiteten Selektor. Ein `false` an `NSTextView`
belegt deshalb nicht, dass im Editor niemand antwortet.

Was trägt die dritte Spalte für diese beiden?

## Optionen

1. **„Editor"** — schmaler als die ursprüngliche Ableitung, dafür belegt.
   - Pro: `setAllowsUndo(true)` steht in `crates/krk-ui/src/appkit/editor.rs:3376`, und der
     Modulkopf von `menue.rs` hält fest, dass die beiden Menüeinträge ohne dieses Flag grau
     blieben. Über Textfelder wird nichts behauptet.
   - Contra: der Beleg ist ein Code-Fakt und keine Laufzeitmessung; er ist schwächer als das,
     was S1 für `cut:`, `copy:` und `paste:` liefert.
2. **Leer lassen** — die Regel des Plans wörtlich genommen.
   - Pro: wo die Messung keine Antwort gibt, bleibt die Zelle leer.
   - Contra: die Spalte verschwiege etwas, das am Code nachweisbar ist. Und die leere Zelle ist
     in dieser Datei bereits vergeben, nämlich an `text_alles_auswaehlen`.
3. **„Textfelder und Editor"** — die ursprüngliche Ableitung des Specs.
   - Contra: die Hälfte „Textfelder" ist für diese beiden ungemessen und hinge am Feldeditor, den
     KRK nicht konfiguriert. Genau die Sorte Zusicherung, die im Text stärker ist als im Code.

## Constraints

Der Spec verlangt in C3, dass die dritte Spalte nichts behauptet, was nicht belegt ist. Eine
leere Zelle ist eine ehrliche Auskunft, eine falsche ist es nicht.

---
Answered: **Möglichkeit 1, „Editor"** — Nutzerantwort am 260811-0935, im Chat gestellt und dort
beantwortet. Umgesetzt in `fd863e3` (`crates/krk-ui/src/belegungsausgabe.rs`, Zweig
`text_rueckgaengig | text_wiederholen`), samt einem Kommentar, der ausdrücklich sagt, dass S1
hier nichts entschieden hat und der Beleg stattdessen `setAllowsUndo(true)` ist.

Die Durchsicht von Turn 1 hat den Zweig geprüft und bestätigt, dass er genau das beansprucht,
was der Beleg hergibt: er sagt „Editor" und nicht „Textfelder und Editor".

---
Implemented: `fd863e3` — `crates/krk-ui/src/belegungsausgabe.rs:314` traegt
`"text_rueckgaengig" | "text_wiederholen" => "Editor"`, mit dem Kommentar darueber, der
ausdruecklich sagt, dass S1 hier nichts entschieden hat und der Beleg `setAllowsUndo(true)`
in `appkit/editor.rs` ist. Am Baum geprueft am 260811-1403.
