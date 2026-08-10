Ein Öffnen im Editor stößt kein Sitzungsschreiben an

---

S30 sagt zu, dass die Sitzung sich die geöffnete Datei merkt. Die Zusage hält auf
normalem Weg, aber nicht nach einem Absturz.

`sitzung_bauen` (`crates/krk-ui/src/appkit/anwendung.rs:4032`) ruft `editordatei()`
(`:4321`), und die liest den Pfad aus dem lebenden Editor. Der Wert ist also
richtig, **sobald irgendetwas ein Sitzungsschreiben anstößt**.

**Keiner der drei Öffnungswege stößt eines an.** Weder F4 noch der Übergang aus der
Vorschau noch der Sprung auf eine Textmarke ruft `sitzung_vormerken`
(`:4051`). Aufgefallen ist das dem `coder` bei S38/S39, festgehalten in seinem
Sitzungsbericht.

---

## Was das kostet

Beendet der Nutzer KRK auf normalem Weg, wird die Sitzung geschrieben und die
Datei steht darin. Wird KRK abgebrochen oder stürzt es ab, bevor ein anderer
Anlass ein Schreiben angestoßen hat, fehlt sie — und beim nächsten Start ist der
Editor leer, obwohl er beim letzten Mal eine Datei hielt.

`inference:` Ungeprüft ist, wie lange dieses Fenster in der Praxis offen steht.
Ein Ordnerwechsel, ein Tabwechsel und mehrere andere Handlungen stoßen ein
Schreiben an; wer nach dem Öffnen weiterarbeitet, schließt es von selbst. Wer F4
drückt und dann nichts mehr tut, hält es offen.

## Der Zuschnitt der Behebung

Ein Aufruf von `sitzung_vormerken` an der einen Stelle, an der ein Öffnen gelingt.
S23 hat gemessen, dass alle drei Wege durch `Editorbereich::datei_oeffnen` und
dessen Behandlung in `editorausgang_behandeln` laufen — dort steht die eine
Stelle, und drei Aufrufe sind nicht nötig.

**Zu prüfen, nicht anzunehmen:** ob das Schließen des Editors (`opt+cmd+e`)
dasselbe braucht. Es hinterlässt die Sitzung mit einer Datei, die nicht mehr
offen ist, und beim nächsten Start käme sie zurück.

Der Takt des `Sitzungsschreibers` bündelt ohnehin auf höchstens ein Schreiben je
zwei Sekunden (`SITZUNGSTAKT` in `crates/krk-core/src/ablage/sitzung.rs:33`), ein
zusätzlicher Anlass kostet also nichts.

**Aufgefallen bei:** S38/S39 am 260810-0204, beim Durchsehen der drei
Öffnungswege.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260810-0204-coder-s38-zweite-haelfte-und-s39-die-textmarke.md`,
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260810-0146-s23-s30-s31-uebergang-sitzung-fremdaenderung.md`
