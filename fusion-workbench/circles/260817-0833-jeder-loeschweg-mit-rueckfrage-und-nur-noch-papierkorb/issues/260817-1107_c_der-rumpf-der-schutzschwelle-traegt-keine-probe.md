# Der Rumpf, der die Schutzschwelle trägt, ist von keiner Probe gedeckt

**Datum:** 260817-1107
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 2
**Schwere:** Mittel
**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/kommandos/loeschwarnung.rs`
**Baumstand:** `472eb81`

## Der Befund

`loeschen_nach_rueckfrage` und `loeschauftrag_stellen` (`anwendung.rs:4603-4697`) sind die
Mechanik, um derentwillen diese Runde läuft. Vier Eigenschaften tragen sie, und keine ist
geprüft:

1. der laufende Vorgang wird **vor** dem Blatt gemeldet und nicht nach der Bestätigung,
2. die leere Auswahl kommt gar nicht bis zum Blatt,
3. ein Abbruch stellt keinen Auftrag,
4. der bestätigte Auftrag trägt die Auswahl, die im Blatt stand.

`kommandos/loeschwarnung.rs` trägt fünf Proben, aber allein über die beiden Texte. `krk-ui`
hat kein Bibliotheksziel, und ein Blatt lässt sich unter `libtest` nicht bedienen; der heutige
Zuschnitt ist ohne Umbau nicht prüfbar. Abgenommen ist er ebenfalls nicht, denn der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit.

## Richtung

Die vierstufige Reihenfolge ist eine Regel über vier Wahrheitswerte (Vorgang läuft, Auswahl
leer, Papierkorb vorhanden, bestätigt) und keine AppKit-Sache. Als reine Funktion neben
`kommandos::rueckschritt` und `kommandos::loeschwarnung` wäre sie mit einer ausgeschriebenen
Tafel prüfbar, so wie dieses Projekt seine übrigen Regeln hält.

Bündel B setzt mit der Papierkorbprüfung eine fünfte Stufe in dieselbe Kette. Wenn die Regel
umzieht, dann dort, nicht danach: ein Umzug nach Bündel B änderte dieselbe Stelle zweimal.

---
Abgleich 260817-1129 (reconciler): **offen, am Baum nachgelesen.** Weiterhin keine Probe über `loeschen_nach_rueckfrage` (`anwendung.rs:4606`) oder `loeschauftrag_stellen` (`:4684`). `kommandos/loeschwarnung.rs` trägt fünf Proben, alle allein über die beiden Texte. `cargo test --workspace` läuft grün, misst diesen Rumpf aber nicht.

---
Resolved: 260817-1359 (coder, Aufgabe T5, zusammen mit Schritt 6 des Plans). Die Stufenfolge
ist als reine Funktion `kommandos::loeschwarnung::vor_der_rueckfrage(vorgang_laeuft,
auswahl_leer, papierkorb) -> Vorstufe` ausgezogen, mit ausgeschriebener Tafel über zwölf
Kombinationen, ohne Auffangzweig und mit `#[must_use]`; die fünfte Stufe aus Bündel B ist als
Zeile derselben Tafel eingetragen, statt die Stelle zweimal zu ändern. `loeschen_nach_rueckfrage`
(`appkit/anwendung.rs`) beschafft nur noch die drei Tatsachen und führt die vier Ausgänge aus.
Sieben Proben in `kommandos::loeschwarnung::tests`, darunter die Aufruferzählung
`die_stufenregel_hat_genau_einen_aufrufer`; `make check` exit 0.

**Zwei der vier Eigenschaften sind damit geprüft und zwei nicht, und das ist der Grund für den
Abschluss statt des Offenbleibens.** Geprüft: der laufende Vorgang wird vor dem Blatt gemeldet
(über alle sechs Kombinationen der beiden anderen Tatsachen, also der Vorrang und nicht nur
der Ausgang), und die leere Auswahl kommt nicht bis zum Blatt. Ungeprüft bleiben „ein Abbruch
stellt keinen Auftrag" und „der bestätigte Auftrag trägt die gezeigte Auswahl": beide sind
Aussagen über den Rückruf des Blattes — dass `bestaetigt == false` bei Esc, Return und
„Abbrechen" ankommt, und dass die `Cell` genau die gezeigte Auswahl herausgibt. Ein Blatt
lässt sich unter `libtest` nicht bedienen, und am Code ist dafür nichts mehr zu tun; sie
gehören in den Abnahmelauf, der KRK im Vordergrund verlangt und Nutzerarbeit ist. Die
Abrechnung im Einzelnen steht in
`history/260817-1359-coder-t5-pruefung-vor-dem-blatt-und-die-stufenregel.md`.
