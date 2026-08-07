Die Meldung zur Bündelkennung sagt nicht, dass `settings.toml` erst beim Start gelesen wird

---

**Domain:** code
**Filed by:** planner (beim Einarbeiten der vier Nutzerantworten vom 260807)
**Für:** Entscheidung des Nutzers, danach `coder`
**Cross-references:** `decisions/260805-1845_*_wann-eine-von-hand-geaenderte-settings-toml-wirkt.md`,
`planning/260802-1036_*_spec-navigator-geruest.md` `### C11` (fünftes Abnahmekriterium),
`planning/260802-1428_*_plan-navigator-geruest-runde-1.md` S18c,
`crates/krk-core/src/ablage/einstellungen.rs`

---

Der Nutzer hat am 260807 entschieden, dass `settings.toml` beim einmaligen
Laden bleibt: wer die Datei ändert, startet KRK neu. Die Entscheidung ist
getroffen und wird hier nicht in Frage gestellt. Sie hat einen Preis, den der
Datensatz selbst in seiner Empfehlung benennt, und dieser Defekt hält ihn fest,
damit er nicht mit der beantworteten Frage verschwindet.

**Der Preis.** Das fünfte Abnahmekriterium von C11 verlangt, dass die
Statuszeile bei einer nicht installierten Bündelkennung den Grund meldet und
die eingestellte Kennung nennt, "damit der Nutzer die Datei berichtigen kann".
Genau das leistet die Meldung unter dem einmaligen Laden nicht zu Ende: der
Nutzer liest sie, öffnet `settings.toml`, behebt den Tippfehler, drückt
`ctrl+o` und bekommt dieselbe Meldung noch einmal. Nichts an ihr deutet darauf
hin, dass allein ein Neustart fehlt. Die Meldung erfüllt damit ihren Wortlaut
und verfehlt ihren Zweck.

**Der Vorschlag, und er ist nicht entschieden.** Die Meldung nennt neben der
Kennung den Ladezeitpunkt, etwa in der Form: "keine Anwendung mit der
Bündelkennung `com.example.gibtesnicht` installiert; `settings.toml` wird beim
Start gelesen, eine Änderung wirkt nach einem Neustart". Das kostet einen
Halbsatz in genau der Meldung, die den Fall ohnehin behandelt, und keinen
zweiten Lesepfad; die Randbedingung des Datensatzes, dass ein Zustand einen
Ladezeitpunkt hat, bleibt unberührt. Die Alternative wäre, gar nichts zu
ändern und den Nutzer den Zusammenhang selbst finden zu lassen.

**Was gegen den Vorschlag spricht**, und deshalb steht er hier und nicht im
Spec. Erstens ist er eine Verhaltensänderung an einem abgenommenen
Abnahmekriterium in der Abnahmephase der Runde 1. Zweitens sagt er dem Nutzer
in derselben Zeile zwei Dinge, den Fehler und eine Eigenschaft der Ablage, und
ob das die Statuszeile überfrachtet, ist eine Bedienfrage und keine
technische. Drittens gilt derselbe Einwand dann für jede künftige Meldung, die
aus einem einmal geladenen Wert stammt; ob das eine Regel wird oder ein
Einzelfall bleibt, gehört mitentschieden.

**Kein Abnahmekriterium hängt daran**, und die Runde schließt auch ohne die
Änderung. Sobald eine spätere Runde eine Einstellungsansicht baut, beantwortet
sie die Frage nach dem Ladezeitpunkt neu, und dieser Defekt fällt mit ihr weg.

---
Resolved:
