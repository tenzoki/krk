Der Nutzerentscheid vom 260811-0935 steht allein im Programmtext, und der Spec widerspricht ihm

---

Der Programmtext beruft sich an drei Stellen auf einen „Nutzerentscheid vom 260811-0935", der
`text_rueckgaengig` und `text_wiederholen` die dritte Spalte „Editor" gibt:

- `crates/krk-ui/src/belegungsausgabe.rs:53` (Tabelle im Modulkopf)
- `crates/krk-ui/src/belegungsausgabe.rs:246-256` (der Zweig selbst)
- `crates/krk-ui/src/belegungsausgabe.rs:646` (die Probe)

**Im Arbeitsbereich steht dieser Entscheid nirgends.** `grep -rn "0935"` über den Circle und
über `shared/` liefert am 260811-0956 keinen Treffer. Es gibt keinen Entscheidungsdatensatz,
keine Zeile in einem Sitzungsbericht und keinen Nachtrag im Spec.

---

**Schwere:** Mittel
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
**Betroffen:** `planning/260811-0753_*_spec-*.md` (C3), `crates/krk-ui/src/belegungsausgabe.rs`
**Domain:** code

## Was der Spec sagt

C3 führt das Kriterium:

> Die sechs vom Hauptmenü zugestellten Textbefehle tragen in der dritten Spalte „Textfelder und
> Editor".

und die Tabelle `## Die sieben Beschriftungen` trägt die Zeile

> `(kein Kommando: die sechs Textbefehle)` | Textfelder und Editor | Nutzerantwort vom
> 260811-0115, unter Vorbehalt der Prüfung

Der Code trägt heute **drei** verschiedene Werte für diese sechs: „Textfelder und Editor" für
drei, leer für einen, „Editor" für zwei. Der Spec ist an keiner Stelle nachgezogen.

## Warum die leere Zelle gedeckt ist und „Editor" nicht

Der Spec deckt eine Berichtigung ausdrücklich ab:

> Trifft „Textfelder und Editor" für einen von ihnen nicht zu, wird seine Zelle berichtigt oder
> bleibt leer.

Für `text_alles_auswaehlen` ist genau das geschehen, und der Datensatz
`issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-fuer-alles-auswaehlen-*.md`
hält die Messung fest. Bis dahin ist alles am Platz.

Für `text_rueckgaengig` und `text_wiederholen` liegt der Fall anders, und der Plan sagt es in
seiner Risikotabelle selbst:

> Eine berichtigte Beschriftung wäre eine neue Vorbelegung und gehört an das Gate, nicht in den
> stillen Bau.

Dazu kommt: derselbe Datensatz `260811-0930` führt `undo:` und `redo:` als **nicht
entscheidbar** und leitet daraus die Regel des Plans ab — „wo die Messung keine Antwort gibt,
bleibt die Zelle leer". Der Code füllt sie trotzdem. Das mag richtig sein — der Beleg
`setAllowsUndo(true)` in `crates/krk-ui/src/appkit/editor.rs:3376` steht am Baum und ist geprüft,
und „Editor" sagt über Textfelder nichts und beansprucht damit nicht mehr, als er hergibt.
**Aber die Begründung ist ein Nutzerentscheid, und ein Nutzerentscheid, den nur ein Kommentar
kennt, ist beim nächsten Lesen des Specs eine Abweichung ohne Herkunft.**

## Behebung

Zwei Zeilen Arbeit, keine am Code:

1. Ein Entscheidungsdatensatz unter `decisions/` dieses Circles mit Frage, Antwort und
   `Answered:`-Zeile, oder — falls die Antwort im Gespräch am Gate gefallen ist — eine Zeile im
   Sitzungsbericht des Schrittes, auf die die drei Kommentarstellen verweisen können.
2. Der Spec zieht C3 und die Tabelle `## Die sieben Beschriftungen` nach: die sechs
   Textbefehle tragen drei verschiedene Werte, nicht einen.

Solange beides fehlt, steht im Baum eine Beschriftung, die der abgenommene Spec verneint.

---
Resolved: Beides nachgeholt. Der Entscheid steht als
`decisions/260811-1010_a_was-traegt-die-dritte-spalte-bei-rueckgaengig-und-wiederholen.md`, und
C3 im Spec ist auf den gemessenen Stand berichtigt — die einheitliche Aussage ueber alle sechs
ist der Dreiteilung gewichen, 18 Abnahmekriterien sind 19 geworden.

**Die Ursache liegt beim Orchestrator und ist im Datensatz festgehalten**, damit sie nicht als
Versaeumnis des Programmtexts gelesen wird: die Frage wurde im Chat gestellt, die Antwort
bekommen und in Commit-Nachrichten und Kommentare geschrieben, aber in keinen Datensatz. Es ist
dieselbe Form wie am 260810 bei der fehlenden Durchsichtsdatei.

Geschlossen in der Sitzung `history/260811-0107-orchestrator-session.md`, Turn 1.
