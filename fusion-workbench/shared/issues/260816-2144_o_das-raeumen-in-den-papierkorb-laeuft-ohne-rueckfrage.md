Das Räumen in den Papierkorb läuft ohne Rückfrage, und der Nutzer verlangt eine für jeden Löschweg

---

Der Nutzer hat am 260816-2144 verlangt, dass **jede** Datei-Löschfunktion durch eine
Rückfrage gesichert wird und der Nutzer bestätigen muss. Von den beiden Löschwegen, die
KRK führt, fragt heute nur einer nach.

---

**Schwere:** Mittel. Kein unwiederbringlicher Verlust, denn der ungesicherte Weg führt in
den Papierkorb und nicht daran vorbei. Die Taste liegt aber unter der rechten Hand, wirkt
auf die ganze Mehrfachauswahl und trägt daneben eine zweite Bedeutung.
**Gefunden von:** Nutzer, gemeldet am 260816-2144
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:4537`,
`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs`,
`crates/krk-ui/src/kommandos/operationen.rs`, `resources/default-keymap.toml:150-158`
**Domain:** code

## Der Stand am Baum

Zwei Kommandos löschen Dateien, und sie sind ungleich gesichert. Beide am 260816 gelesen.

| Funktion | Tasten | Wirkung | Rückfrage |
|---|---|---|---|
| `in_papierkorb` (`Kommando::InPapierkorb`) | `delete`, `cmd+delete` | in den Papierkorb | **keine** |
| `endgueltig_loeschen` (`Kommando::EndgueltigLoeschen`) | `f8`, `opt+cmd+delete` | ohne Papierkorb | einmal je Vorgang |

Die Tasten stehen in `resources/default-keymap.toml:150-158`, die Kommandos in
`crates/krk-core/src/tasten/belegung.rs:489-491`.

Das Bestätigungsblatt hat **genau einen Aufrufer**, `crates/krk-ui/src/appkit/anwendung.rs:4537`,
und sein Modulkopf nennt seinen Gegenstand in der ersten Zeile: „Die Rueckfrage vor dem
endgueltigen Loeschen (C4)". Vorbelegt ist dort „Abbrechen", damit ein reflexhaftes Bestätigen
mit der Eingabetaste nichts löscht.

## Warum das kein Versehen ist, sondern eine umgesetzte Festlegung

Wer diesen Defekt behebt, ohne den folgenden Absatz zu lesen, bricht eine bindende Zusage,
ohne es zu merken.

Der Zustand ist die Antwort des Nutzers vom 260802-1105, wörtlich festgehalten in
`shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md`: „Delete löscht in
Papierkorb, FN+F8 endgültig". Der Datensatz trägt den Marker `_i_`, also umgesetzt, und nennt
die Commits `daecb45` und `343a7f3`. Seine Begründung führt die Rückfrage ausdrücklich als
Preis: sie koste einen Tastendruck je Vorgang und bremse die Tastaturarbeit nicht, „weil das
alltägliche Löschen über Delete ohne jede Rückfrage läuft".

Dieselbe Aufteilung steht in der Directive der ersten Runde
(`circles/260802-0842-krk-mac-dateimanager-editor-git/_b_circle.md`, Abschnitt `## Directive`):
„Die Taste Delete und Cmd+Delete räumen in den Papierkorb, F8 und Cmd+Opt+Delete löschen
endgültig und fragen dabei einmal je Vorgang nach."

Der neue Wunsch des Nutzers hebt diese Festlegung auf. Der Vorgang ist damit kein reiner
Code-Fix: er verlangt einen Entscheidungsdatensatz, der den bestehenden überholt.

## Was die Umsetzung mitziehen muss

- `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md` bekommt eine Zeile
  `Superseded by:` und wandert auf `_s_`. Ein neuer Datensatz hält die Antwort vom 260816.
- Der Spec der ersten Runde,
  `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md:136-141`,
  trägt die alte Aufteilung als Abnahmekriterium und ist nachzuziehen.
- `CLAUDE.md` sagt unter „Was man nicht sieht" zweimal, das Räumen laufe ohne Rückfrage. Der
  Modulkopf von `crates/krk-ui/src/kommandos/rueckschritt.rs` sagt es ein drittes Mal und
  stützt darauf seine eigene Begründung.
- Der Modulkopf von `crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs` nennt in seiner
  ersten Zeile das endgültige Löschen als seinen Gegenstand. Trägt das Blatt künftig beide
  Wege, ist der Satz falsch.

## Was der Umsetzer nicht raten darf

Fünf Punkte sind aus dem Wunsch nicht ableitbar und gehören vor die Umsetzung. Sie sind der
Grund, aus dem hier ein Entscheidungsdatensatz nötig ist und nicht nur ein Diff.

1. Fragt die neue Rückfrage einmal je Vorgang oder je Eintrag? Beim endgültigen Löschen gilt
   „einmal je Vorgang, unabhängig von der Zahl der betroffenen Einträge".
2. Fragt sie auch bei einem einzelnen Eintrag, oder erst ab einer Zahl?
3. Ist „Abbrechen" auch hier vorbelegt? Beim endgültigen Löschen verlangt C4 es wörtlich.
4. Gilt die Rückfrage für beide Tasten des Papierkorbwegs, `delete` und `cmd+delete`, oder
   soll eine der beiden der schnelle Weg bleiben?
5. Tragen beide Löschwege dasselbe Blatt mit unterschiedlichem Text, oder bekommt der
   Papierkorb ein eigenes? Ein zweites Blatt wären zwei Wahrheiten über dieselbe Frage.

## Berührung mit der Rückschritt-Regel

`crates/krk-ui/src/kommandos/rueckschritt.rs` trennt heute drei Fälle der nackten
Rückschritt-Taste: ein Zeichen des Filtertextes zurücknehmen, gar nichts tun, oder in den
Papierkorb räumen. Sein Modulkopf begründet die Existenz der Regel damit, dass „das Raeumen
ohne Rueckfrage laeuft" und die Berichtigung eines Vertippers sonst Dateien wegräumte.

Eine Rückfrage vor dem Räumen entzieht dieser Begründung nicht die Grundlage, denn eine
Rückfrage bei jedem korrigierten Zeichen wäre unbrauchbar. Die Regel bleibt also nötig, aber
ihr Modulkopf argumentiert danach mit einer Aussage, die nicht mehr stimmt, und ist
mitzuziehen.

## Nicht betroffen

`lesezeichen_loeschen` (`Kommando::LesezeichenLoeschen`, `belegung.rs:533`) entfernt einen
Eintrag aus der Lesezeichenleiste und keine Datei vom Datenträger. Der Wunsch nennt
Datei-Löschfunktionen; ob die Lesezeichenleiste eine Rückfrage bekommen soll, ist eine
eigene Frage und hier nicht gestellt.

## Nicht geprüft

Ob eine Rückfrage vor dem Papierkorbweg eine der zehn Zeitzusagen aus C8 der ersten Runde
bewegt. Keine der zehn misst einen Löschvorgang von Hand; der Abnahmelauf ist seit dem
260810 nicht mehr gefahren und Nutzerarbeit.

## Verschärfung vom 260817: der endgültige Löschweg fällt ganz weg

Der Nutzer hat am 260817 nachgeschärft: **Löschen ohne Papierkorb wird entfernt. Jeder
Datei- und Ordner-Löschvorgang geht immer in den Papierkorb.**

Damit ändert sich der Zuschnitt der Runde, und zwar in beide Richtungen. Kleiner wird sie,
weil danach ein Löschweg existiert statt zweier: `Kommando::EndgueltigLoeschen` fällt weg,
`f8` und `opt+cmd+delete` werden frei, und die Frage aus der Tabelle oben, ob beide Wege
dasselbe Blatt tragen, beantwortet sich von selbst. Größer wird sie, weil eine Variante aus
`Kommando` zu entfernen jede vollständige Fallunterscheidung darüber anfasst und weil die
Nutzerfestlegung vom 260802 damit nicht nur in ihrer Rückfragenhälfte überholt ist, sondern
ganz.

Die überholte Festlegung `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md`
hält beide Hälften: „Delete löscht in Papierkorb, FN+F8 endgültig". Nach dieser Runde stimmt
kein Teil davon mehr.

**Was daraus offen ist und vor der Umsetzung geklärt gehört:**

Nicht jedes Ziel hat einen Papierkorb. `trashItemAtURL:` scheitert auf Datenträgern, die
keinen führen, und auf manchen Netzlaufwerken; der Finder bietet dort das endgültige Löschen
als einzigen Weg an. Wenn KRK den endgültigen Weg nicht mehr kennt, kann es auf solchen
Zielen gar nicht mehr löschen. Ob das die richtige Antwort ist — nicht löschen können statt
unwiederbringlich löschen — ist eine Nutzerfrage und keine Ableitung. Sie ist am 260817
gestellt und noch nicht beantwortet.

---
Abgleich 260817-1129 (reconciler): **offen und teilweise abgetragen; die zuletzt genannte
offene Frage ist inzwischen beantwortet.**

**Was gebaut ist.** Der Papierkorbweg fragt seit Commit `472eb81` nach. `in_den_papierkorb`
(`crates/krk-ui/src/appkit/anwendung.rs:4454`) geht durch `loeschen_nach_rueckfrage`
(`:4606`), das Blatt steht vor jedem Auftrag, und „Abbrechen" ist vorbelegt
(`appkit/blaetter/loeschbestaetigung.rs:98-105`). Damit ist der Kern der Meldung vom
260816-2144 eingelöst: keiner der beiden Löschbefehle nimmt heute noch eine Datei ohne
stehende Rückfrage vom Datenträger.

**Was offen bleibt, und warum der Marker `_o_` steht.** Die Verschärfung vom 260817 verlangt
mehr als die Rückfrage: der endgültige Löschweg fällt ganz, kein Löschen auf Zielen ohne
Papierkorb, und die überholte Festlegung wird an sechs Stellen nachgezogen. `Kommando::`
`EndgueltigLoeschen` steht am 260817 mit zwanzig Nennungen im Baum, `resources/`
`default-keymap.toml:151-153` führt `endgueltig_loeschen` unverändert, und die vier
mitzuziehenden Stellen aus dem Abschnitt `## Was die Umsetzung mitziehen muss` sind
unangetastet. Der Plan
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md`
trägt sie als Bündel B bis E, vierzehn Schritte, alle offen.

**Die Frage am Ende dieses Datensatzes ist beantwortet.** Der Schlussabsatz sagt, ob KRK auf
Zielen ohne Papierkorb gar nicht mehr löschen könne, sei „am 260817 gestellt und noch nicht
beantwortet". Sie ist am selben Tag beantwortet worden:
`shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`
hält die Antwort des Nutzers, ein Ziel ohne Papierkorb wird nicht gelöscht, sondern gemeldet.
Der Absatz oben bleibt im Wortlaut stehen; verbindlich ist diese Zeile.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **weiter offen, und der Marker wandert
ausdrücklich nicht.**

Die Durchsicht des Bündels B hat das Gegenteil empfohlen:
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1419_o_der-ausloesende-defekt-des-raeumens-ohne-rueckfrage-ist-behoben-und-steht-weiter-offen.md`
verlangt die Zeile `Resolved:` mit `472eb81` und den Übergang auf `_c_` und begründet es mit
„sein Wortlaut ist von Bündel D nicht mehr betroffen". **Der Wortlaut dieses Datensatzes
widerlegt das selbst.** Der Abschnitt `## Verschärfung vom 260817: der endgültige Löschweg
fällt ganz weg` verlangt drei Dinge, die keine davon gebaut ist: `Kommando::EndgueltigLoeschen`
fällt weg, `f8` und `opt+cmd+delete` werden frei, und die Nutzerfestlegung vom 260802 ist ganz
und nicht nur in ihrer Rückfragenhälfte überholt. Genau das ist Bündel D. Am Baum stehen am
260817-1833 22 Zeilen `EndgueltigLoeschen` in 12 Dateien, und
`resources/default-keymap.toml:151` führt `endgueltig_loeschen` unverändert mit
`["f8", "opt+cmd+delete"]`. Auch die vier Stellen aus `## Was die Umsetzung mitziehen muss`
sind unangetastet: Bündel E trägt sie als Schritte 15 bis 17.

**Was gebaut ist, ist mehr als beim letzten Abgleich.** Neben der Rückfrage aus Bündel A steht
jetzt die Papierkorbprüfung vor dem Blatt (`appkit/anwendung.rs:4713`, Statuszeile über
`kommandos::loeschwarnung::ohne_papierkorb`) und die laute Form mit ihren sechs Auslösern
(`kommandos::loeschwarnung::warngruende`, `anwendung.rs:4871`). Die eine offene Frage am
Dokumentende bleibt beantwortet wie beim Abgleich 260817-1129 vermerkt.

**Der Übergang auf `_c_` gehört an das Ende von Bündel D**, wenn `grep -rn "EndgueltigLoeschen"
crates` keinen Treffer mehr liefert. Bis dahin führt dieser Datensatz die Verschärfung als
offen, und das ist die richtige Auskunft.
