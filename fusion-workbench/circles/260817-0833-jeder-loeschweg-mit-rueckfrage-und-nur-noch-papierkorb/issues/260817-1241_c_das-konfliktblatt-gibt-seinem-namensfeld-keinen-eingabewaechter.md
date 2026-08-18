# Das Konfliktblatt gibt seinem Namensfeld keinen Eingabewächter

**Datum:** 260817-1241
**Gefunden von:** coder, beim Nachziehen aller Blätter für T1
**Schwere:** Mittel
**Betrifft:** `crates/krk-ui/src/appkit/blaetter/konflikt.rs`
**Baumstand:** `472eb81` und der Stand nach T1 — an dieser Stelle ist nichts angefasst

## Der Befund

`konflikt.rs:102` hängt sein Namensfeld mit `beigabe_setzen` unter die Frage und ruft
weder `textfeld_setzen` noch `waechter_anhaengen`. Es ist damit das eine Blatt im Baum mit
einem Textfeld **ohne** `Eingabewaechter`; die fünf übrigen Feldblätter (`pfadeingabe`,
`namenseingabe`, `zeilennummer`, `suche`, `stapelumbenennen`) haben einen.

Der Modulkopf von `blaetter/mod.rs` sagt, wozu der Wächter da ist, und nennt die Messung:
„Am laufenden Bündel gemessen am 260804: ohne ihn lässt sich das Blatt weder mit der
Eingabe- noch mit der Escape-Taste schließen." Ein Textfeld im Bearbeitungszustand
verbraucht beide Tasten selbst.

Solange der Nutzer nicht in das Feld klickt, ist nichts betroffen: `konflikt.rs` ruft kein
`ersthelfer_setzen`, der Ersthelfer ist also eine Schaltfläche und die vier
Tastenentsprechungen greifen. Sobald er hineinklickt, um den Namen für „Umbenennen" zu
tippen, hält das Feld den Ersthelfer.

Dann greift daneben `kommandos::zulaessigkeit::zulaessig` nicht mehr: `abbrechen` steht
nicht in `immer_erreichbar` (`zulaessigkeit.rs:197-202`), und `ersthelfer_gehoert_appkit`
ist bei einem Textfeld wahr. Der Weg zum Abbruch, auf den sich `loeschbestaetigung` beruft,
steht hier also auch nicht offen.

## Was nicht belegt ist

Ob der Feldeditor `Opt+Return` (die Taste von „Umbenennen") wirklich verbraucht. Verbraucht
er es, ist keine der vier Antworten des Blattes mit der Tastatur erreichbar, sobald der
Nutzer im Feld getippt hat, und „Umbenennen" ist überhaupt nur mit der Maus zu erreichen.
Verbraucht er es nicht, bleiben Eingabetaste und Escape betroffen. Beides ist am laufenden
Bündel zu messen, und das verlangt KRK im Vordergrund, also Nutzerarbeit.

Der Befund steht auf Mittel und nicht höher, weil eine Antwort mit der Maus in jedem Fall
möglich bleibt.

## Richtung

`waechter_anhaengen(mtm, &feld)` an der einen Stelle, so wie es die fünf übrigen
Feldblätter tun. Zu entscheiden ist dabei, was der Wächter im Konfliktblatt bedeuten soll:
er kennt zwei Antworten (bestätigt und abgebrochen), das Blatt hat vier. Die bestätigende
Seite läuft heute fest auf die **erste** Schaltfläche, und die ist hier „Überschreiben"; das
wäre für einen Return im Namensfeld die falsche Antwort. Die Frage gehört damit an
`blaetter/mod.rs` und nicht an dieses Blatt, siehe
`260817-1242_o_die-bestaetigende-seite-des-eingabewaechters-liegt-fest-auf-der-ersten-schaltflaeche.md`.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, unverändert.**
`crates/krk-ui/src/appkit/blaetter/konflikt.rs:102` ruft weiter allein `beigabe_setzen`; weder
`textfeld_setzen` noch `waechter_anhaengen` kommen in der Datei vor.

---
Resolved 260818 (coder, Bündel C/D-Nachzug): **das Namensfeld hat seinen Wächter, und
`260817-1242` war die Vorbedingung dafür.**

**Die Reihenfolge war nicht beliebig.** Dieser Datensatz nennt die Frage, die zuerst zu
beantworten war, und sie ist eine harte Vorbedingung und keine Zugabe: hätte der Wächter hier
gehangen, solange seine bestätigende Seite fest auf der **ersten** Schaltfläche lag, hätte ein
Return im Namensfeld „Überschreiben" ausgelöst und den Eintrag am Ziel gelöscht. Der Modulkopf
dieser Datei schließt genau diese Bewegung für die Vorgabeschaltfläche ausdrücklich aus. Der
Wächter allein wäre also kein halber Fix gewesen, sondern ein neuer Defekt, und zwar auf dem
zerstörenden Ausgang.

Gebaut wurde deshalb erst `blaetter::bestaetigungsstelle` (`260817-1242`), dann hier
`waechter_anhaengen(mtm, &feld)` — die dritte der drei Handlungen von `textfeld_setzen` und
nicht dieses selbst, denn das Feld soll weiterhin **nicht** Ersthelfer werden; warum, sagt der
Kopf dieser Datei unverändert.

**Was der Wächter in diesem Blatt bedeutet, ist damit abgeleitet und nicht gewählt.** Die
Eingabetaste geht an die Schaltfläche, die sie trägt, und das ist hier „Überspringen"; die
Escape-Taste fällt über `abbruchstelle` auf „Abbrechen". Der Wächter sagt im Feld damit genau
das, was die Erläuterung des Blattes dem Nutzer ansagt („Return überspringt, … Esc bricht
ab"), und keine der beiden Tasten führt etwas aus, das der Nutzer nicht erwartet.

**Was weiterhin nicht belegt ist**, und im Modulkopf jetzt als offen dasteht: ob der
Feldeditor `Cmd+Return` und `Opt+Return` durchlässt. Das ist am laufenden Bündel zu messen und
verlangt KRK im Vordergrund. Der Befund selbst hängt nicht daran: die beiden Tasten waren vor
dieser Änderung ebenso ungemessen, und erreichbar bleiben „Überschreiben" und „Umbenennen" in
jedem Fall, indem der Nutzer das Feld verlässt oder die Maus nimmt.

**Der Baum trägt damit kein Textfeld ohne Wächter mehr.** Nachgezählt: sechs Blätter halten
einen — `pfadeingabe`, `namenseingabe` und `zeilennummer` über `textfeld_setzen`, `suche`
(zwei Felder), `stapelumbenennen` (vier Felder in einer Schleife) und jetzt `konflikt`
unmittelbar über `waechter_anhaengen`. Die drei letzten rufen einzeln und nicht über
`textfeld_setzen`, jedes aus seinem eigenen Grund: bei `suche` und `stapelumbenennen` ist die
Beigabe der Rahmen um mehrere Felder und nicht eines davon, beim Konfliktblatt soll das Feld
nicht Ersthelfer werden.

Abnahme: `make check` — Exit 0.
