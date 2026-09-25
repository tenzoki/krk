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

---

## Die Ursache genauer, als der Datensatz sie beim Anlegen kannte

Ein Anlass steht sehr wohl an allen drei Wegen — er steht nur zu früh.

F4 und der Übergang aus der Vorschau laufen durch `kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:2088`), und die merkt jedem
ausgeführten Befehl die Sitzung vor. Der Sprung auf eine Textmarke kommt über
`lesezeichen_anspringen` (`:1069`) auf denselben Weg. Nur hält der Editor in
diesem Augenblick noch die **vorige** Datei: gelesen wird seit S24 auf dem
Arbeitsfaden, und `editordatei()` (`:4321`) antwortet aus dem Modell, das erst
mit dem eingezogenen Ladeausgang nachzieht (`Editormodell::einziehen`,
`crates/krk-ui/src/editormodell.rs:843`). Vorgemerkt wurde also ein Stand, und
er war der falsche.

Für die Folge macht das keinen Unterschied — die neue Datei stand in keiner
`session.toml`, bis ein späterer Anlass zufällig eine schrieb —, für den
Zuschnitt der Behebung ebenfalls nicht: der Aufruf gehört hinter das gelungene
Öffnen, und genau dort steht er jetzt.

## Zur Prüffrage: das Schließen braucht nichts

`opt+cmd+e` schreibt bereits, auf beiden Wegen, und zwar nach dem Schließen.
`editor_ausblenden` (`:3864`) ruft `Editorbereich::schliessen`, und das gibt die
Datei im Modell sofort auf; nichts daran läuft über einen Arbeitsfaden.

- **Ohne ungesicherten Stand** führt `anlass_beginnen` (`:3722`) den Anlass
  sofort aus. `anlass_ausfuehren` merkt die Sitzung an seinem Ende vor
  (`:3823`), und `kommando_ausfuehren` ein zweites Mal (`:2088`); beide Male
  hält der Editor schon keine Datei mehr.
- **Mit ungesichertem Stand** steht erst das Blatt. Antwortet der Nutzer mit
  "sichern" oder "verwerfen", läuft `anlass_ausfuehren` mit seinem Vormerken
  hinterher; antwortet er mit "abbrechen", bleibt der Editor offen, und die
  Sitzung hat nichts Neues zu melden.

Eine Zeile im Schließen wäre damit die zweite Stelle mit einer Meinung darüber,
wann die Sitzung nachzieht, und sie hätte nichts zu tun.

---
Resolved: `sitzung_vormerken` steht jetzt in `editorausgang_behandeln`
(`crates/krk-ui/src/appkit/anwendung.rs`), im Zweig
`Geoeffnet | SchonOffen` und im Block, den ein ausgeführter Befehl nachzieht —
also neben Fokus und Titel und unter derselben Bedingung `!aus_sitzung`. Die
Wiederherstellung beim Start schreibt damit nicht zurück, was sie eben gelesen
hat. Alle drei Öffnungswege laufen über diese eine Stelle; drei Aufrufe sind
nicht entstanden.

Das Schließen bleibt unverändert, aus dem Grund im Abschnitt darüber.

Neue Probe:
`crates/krk-ui/src/editormodell.rs::tests::der_gehaltene_pfad_wechselt_erst_mit_dem_eingezogenen_ausgang`
hält die Zeitspanne fest, die die Ursache war: während der Arbeitsfaden liest,
nennt `pfad()` unverändert die vorige Datei. Die Aufrufstelle selbst ist ohne
Fenster nicht prüfbar — `Anwendungsdelegierter` braucht AppKit und den
Hauptfaden, und keine Probe des Projekts baut ihn.
