Drei Aufrufer von eintrag_waehlen werfen den Auswahlversuch weg

---

D7 hat die Aufrufstelle der Messstrecke geschlossen: sie liest den
`Auswahlversuch` jetzt und bricht bei `Unbekannt` ab. Drei weitere Aufrufer von
`Tabellenquelle::eintrag_waehlen` werfen den Rückgabewert weiterhin weg.

---

**Die Stellen:** `crates/krk-ui/src/appkit/anwendung.rs:1885`, `:1908`, `:2316`.

**Warum es heute nichts kaputtmacht.** Alle drei treffen eine gerade lesende
Liste und bekommen deshalb `Auswahlversuch::Vorgemerkt` — die Auswahl springt
mit dem Abschluss des Lesevorgangs auf den Namen, und das ist der normale Weg.
Ein `Unbekannt` ist an diesen drei Stellen unwahrscheinlich, aber nicht
ausgeschlossen; der Kopfkommentar von `eintrag_waehlen` schreibt selbst, dass
ein abgewiesener Versuch eine Auskunft an den Nutzer wert wäre.

**Was der Nutzer heute erlebt, wenn es doch eintritt.** Nichts. Die Auswahl
bleibt, wo sie war, und niemand sagt ihm, dass der Eintrag, zu dem er wollte,
nicht da ist. Die Statuszeile aus C1 ist der Ort für solche Auskünfte und trägt
sie an anderen Stellen bereits.

**Denkbarer Weg.** An jeder der drei Stellen den `Unbekannt`-Fall in die
Statuszeile melden, mit Name und Ordner, so wie die Messstrecke ihn jetzt in
ihre Abbruchmeldung schreibt. Kein neuer Mechanismus: `melden` steht an allen
drei Stellen bereits zur Verfügung.

**Dringlichkeit.** Gering. Unbeobachtet, kein Abnahmekriterium berührt, keine
der zehn Zeitzusagen aus C8 betroffen.

**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`.

**Aufgefallen bei:** der Umsetzung von D7, Turn 25 der Sitzung 260806-2257,
`history/260807-0218-coder-eine-abgewiesene-auswahl-bricht-die-messstrecke-ab.md`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1304_o_der-sitzungslauf-blieb-einmal-von-drei-malen-bei-l6-stehen.md`

---

## Nachgeprüft am 260807 gegen `5d7e299`: der Befund gilt, aber nur noch an einer der drei Stellen

Der Defekt bleibt **offen**. Die vorgeschlagene Änderung ist eine sichtbare
Änderung am Verhalten und gehört dem Nutzer vorgelegt. Was sich geändert hat,
ist die Begründung: `5d7e299` hat zwei der drei Stellen aus dem Befund
herausgenommen, und die dritte steht schwächer da, als der Bericht oben sie
zeichnet.

**Die Stellen, mit den heutigen Zeilennummern.** Die drei Aufrufer sind
dieselben geblieben und werfen den Rückgabewert weiterhin weg:

- `crates/krk-ui/src/appkit/anwendung.rs:1937` — `anlegen_ausfuehren`
- `crates/krk-ui/src/appkit/anwendung.rs:1960` — `umbenennen_ausfuehren`
- `crates/krk-ui/src/appkit/anwendung.rs:2378` — `vorgang_beenden`, Zweig
  `Art::UmbenennenImStapel`

`eintrag_waehlen` ist **nicht** nach `tabs.rs` gewandert. Die Methode, die
diese drei rufen, steht weiterhin als `Tabellenquelle::eintrag_waehlen` in
`crates/krk-ui/src/appkit/tabelle.rs:1075`. Gewandert ist der **Entscheid**
darüber, was ein Name ergibt: er sitzt jetzt in
`Tabliste::auswahl_auf_namen` (`crates/krk-ui/src/tabs.rs:552`), und
`eintrag_waehlen` reicht ihn nur noch durch und setzt bei `Gewaehlt` die Zeile.

**Was `5d7e299` an der Sache ändert.** `auswahl_auf_namen` fragt seither
`tab.liest()` **zuerst** und merkt den Namen vor, statt erst im angezeigten
Bestand zu suchen. Läuft ein Lesevorgang, ist die Antwort damit ohne Ausnahme
`Vorgemerkt`; `Unbekannt` ist in dieser Spanne nicht mehr erreichbar, sondern
ausgeschlossen. Alle drei Stellen rufen unmittelbar davor
`auffrischung::ordner_neu_lesen`, und `Tabellenquelle::neu_lesen`
(`crates/krk-ui/src/appkit/tabelle.rs:624`) startet den Lesevorgang synchron.

Damit trennen sich die drei:

- **`umbenennen_ausfuehren` (:1960)** nimmt seinen Ordner aus
  `quelle().angezeigter_ordner()`. Die Auffrischung trifft diese Seite deshalb
  immer, ein Lesevorgang läuft immer, und `Unbekannt` kann dort nicht mehr
  eintreten. Die Stelle trägt den Befund nicht mehr.
- **`anlegen_ausfuehren` (:1937)** hält den Ordner seit dem Öffnen des Blattes
  fest (`:1887`), und das Blatt sperrt die Navigation, solange es steht. Der
  Ordner ist beim Ausführen also weiterhin der angezeigte. Dasselbe Ergebnis:
  `Unbekannt` ist ausgeschlossen. Auch diese Stelle trägt den Befund nicht mehr.
- **`vorgang_beenden` (:2378)** ist die eine, die bleibt. Zwischen dem Start des
  Stapel-Umbenennens und seinem Abschluss läuft der Vorgang im Hintergrund, und
  der Nutzer kann in der Zwischenzeit den Ordner wechseln. Dann frischt
  `ordner_neu_lesen` auf dieser Seite nichts auf, kein Lesevorgang läuft, und
  `auswahl_auf_namen` befragt das Modell des **anderen** Ordners. `Unbekannt`
  ist dort erreichbar, und der Rückgabewert fällt wortlos weg.

**Was das für den Nutzerentscheid bedeutet.** Der Vorschlag "an jeder der drei
Stellen den `Unbekannt`-Fall melden" ist an zwei Stellen toter Code geworden:
ein Zweig, den nichts mehr erreicht. Vorzulegen ist damit die eine Stelle, und
dort ist die Antwort nicht offensichtlich. Der einzige Weg, auf dem `Unbekannt`
noch entsteht, ist der Ordnerwechsel während eines laufenden
Stapel-Umbenennens; eine Meldung "«datei-1» steht nicht in der Liste" träfe den
Nutzer dann in einem Ordner, über den er gerade gar nichts wissen wollte. Das
ist eher Rauschen als Auskunft. Eine ernstzunehmende Alternative wäre, an
dieser Stelle gar nichts zu melden und den Rückgabewert ausdrücklich mit einer
Begründung zu verwerfen (`let _ = …` mit Kommentar), damit der nächste Leser
sieht, dass das Wegwerfen eine Entscheidung ist und kein Versehen.

**Nachgeprüft von:** `coder`, Aufräumturn 26, ohne Codeänderung.

---
Resolved: Der Nutzer hat am 260810-1717 die zweite der beiden Moeglichkeiten gewaehlt: der
Rueckgabewert wird an der verbliebenen Stelle ausdruecklich und begruendet verworfen, nicht
gemeldet. In `vorgang_beenden`, Zweig `Art::UmbenennenImStapel`
(`crates/krk-ui/src/appkit/anwendung.rs`), steht jetzt `let _ = ...` mit einem Kommentar, der
drei Dinge nennt: dass `Unbekannt` hier als einziger der drei Stellen erreichbar ist, auf
welchem Weg (Ordnerwechsel waehrend des im Hintergrund laufenden Vorgangs), und warum trotzdem
nichts gemeldet wird. Eine Meldung traefe den Nutzer in einem Ordner, ueber den er gerade
nichts wissen wollte.

**Der Titel dieses Datensatzes ist in zwei Punkten ungenau, und beide sind nachgeprueft.**
Erstens sind es nicht mehr drei Stellen, sondern eine: seit `5d7e299` fragt
`Tabliste::auswahl_auf_namen` zuerst `tab.liest()` und merkt den Namen vor, womit `Unbekannt`
in `anlegen_ausfuehren` und `umbenennen_ausfuehren` ausgeschlossen ist. Beide tragen jetzt
einen Einzeiler im Doc-Kommentar, damit der Befund nicht ein drittes Mal erhoben wird.
Zweitens gibt es einen **vierten** Aufrufer, den dieser Datensatz nie gefuehrt hat:
`messhandlung`, Zweig `Handlung::Auswaehlen`. Der behandelt den Rueckgabewert bereits
vollstaendig und meldet `Unbekannt` als Abbruchgrund an den Messlauf.

Der urspruengliche Vorschlag "an jeder der drei Stellen melden" ist damit an zwei Stellen
toter Code und ist bewusst nicht umgesetzt. Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 1.
