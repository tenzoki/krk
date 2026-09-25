`readers.toml` fällt beim `zip` der Beiseitelegeprobe still heraus

---

`crates/krk-core/tests/ablage.rs` paart in `jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt`
den Rundlauf über `toml_dateien()` mit der Liste aus `ersetzungen_der_toml_dateien`. Seit
Schritt 2 der Runde 16 liefert die linke Seite fünf Dateien und die rechte vier; `zip` kürzt
auf die kürzere, und `readers.toml` wird ohne ein Wort übersprungen. Die Probe bleibt grün und
prüft eine Datei weniger, als ihr Rundlauf verspricht.

---

**Gemessen am Baumstand `abecfb2` plus dem Stand dieses Schritts.**

Die Probe schreibt `KAPUTT` in **jede** Datei aus `toml_dateien()`, also seit diesem Schritt
auch in `readers.toml`. Danach läuft sie über
`toml_dateien().zip(ersetzungen_der_toml_dateien(&ablage))`. `ersetzungen_der_toml_dateien`
lädt `keymap.toml`, `bookmarks.toml`, `session.toml` und `settings.toml` und gibt vier Werte
zurück — `readers.toml` hat in diesem Baum noch keinen Ladeweg, also kann sie dort nicht
stehen. Das Paar endet nach vier Durchgängen, und für `readers.toml` wird weder die Meldung
noch der Beiseitepfad geprüft.

**Der Schaden ist nicht die heutige Lücke, sondern ihre Stille.** Beide Doc-Kommentare in
`tests/ablage.rs` schreiben die Lücke seit diesem Schritt aus, und das ist alles, was dieser
Schritt tun konnte: eine Zusicherung über die Länge wäre heute rot, weil die vierte Zahl der
rechten Seite gegen die fünfte der linken steht, und ein fünfter Eintrag verlangt den
Ladeweg, den erst Schritt 7 des Plans baut
(`planning/260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`,
Bündel C).

**Schritt 7 nennt `crates/krk-core/tests/ablage.rs` in seiner Dateiliste nicht.** Wer ihn nach
seinem Wortlaut fährt, baut `ablage::leseprofile::laden` und lässt die Paarung hier
unverändert; die Lücke bliebe dann bestehen, obwohl der Ladeweg dasteht. Deshalb dieser
Datensatz: er ist die Stelle, an der das Nachziehen verlangt wird.

**Was zu tun ist**, sobald `ablage::leseprofile::laden` steht: `readers.toml` in
`ersetzungen_der_toml_dateien` an fünfter Stelle nachtragen, in der Reihenfolge von
`toml_dateien()`, und die zwei Doc-Kommentare, die heute die Lücke ausschreiben, wieder auf
den vollen Rundlauf setzen. Danach hält eine Zusicherung, dass beide Seiten gleich lang sind,
und das `zip` kann nicht mehr still kürzen.

**Schwere:** niedrig, solange `readers.toml` keinen Ladeweg hat — es gibt nichts zu prüfen.
Mittel, sobald sie einen hat: C1.6 verlangt für sie genau das Beiseitelegen, das diese Probe
misst.

**Gefunden:** coder, Schritt 2 der Runde 16 am 260824-0940.

**Betroffen:** `crates/krk-core/tests/ablage.rs`, `ersetzungen_der_toml_dateien` und
`jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt`.

**Domain:** code

---
Also seen: 260824-1014 by coderev — Befund am Baumstand `b76800b` bestätigt, mit zwei Berichtigungen: der Ladeweg entsteht in **Schritt 8** (`ablage/leseprofile.rs`, Bündel C) und nicht in Schritt 7 (`resources/default-readers.toml`, `ontocoder`), so wie es der Datensatz `260824-0955_o_die-files-zeile-eines-planschritts-…` bereits richtig führt; und die Paarung ist heute nur deshalb noch die richtige, weil `Datei::Leser` in `Datei::ALLE` hinter `Datei::Einstellungen` und damit als letzte TOML-Datei steht — wer die Reihenfolge in `ALLE` ändert, bekommt statt der stillen Kürzung eine falsch gepaarte Zusicherung mit irreführendem Meldetext. Im ganzen Baum ist dies das einzige `zip`, das still kürzen kann; die übrigen dreizehn laufen über Felder fester, typgeprüfter Länge oder über dieselbe Quelle.

---
Resolved: Die stille Kürzung ist weg, und an ihrer Stelle steht eine Zusicherung, die heute grün
ist und rot wird, sobald wieder eine Datei herausfällt.

`crates/krk-core/tests/ablage.rs` führt jetzt die benannte Ausnahme `const OHNE_LADEWEG: [Datei;
1] = [Datei::Leser]` und daraus abgeleitet `toml_dateien_mit_ladeweg()`. Die Probe — jetzt
`jede_toml_datei_mit_ladeweg_wird_bei_beschaedigung_zur_seite_gelegt` — beschädigt weiter **jede**
TOML-Datei, hält vor dem `zip` `toml_dateien_mit_ladeweg().count()` gegen
`ersetzungen_der_toml_dateien(&ablage).len()` und paart erst danach.

**Der vom Datensatz vorgeschlagene Weg war, `readers.toml` nachzutragen; er ist heute nicht
gangbar**, denn `ablage::leseprofile::laden` entsteht erst mit Schritt 8. Die Ausnahmeliste ist
die Fassung, die den Befund trotzdem schließt: sie schreibt die Auslassung dort aus, wo sie
gezählt wird, statt sie dem `zip` zu überlassen. Wer den Ladeweg baut, nimmt den Eintrag heraus
und trägt die Datei in `ersetzungen_der_toml_dateien` nach; wer nur eines von beidem tut, bekommt
die Zusicherung rot. Nachgestellt und gemessen: mit leerer Ausnahmeliste meldet sie `left: 5,
right: 4` samt dem Satz, welche der zwei Seiten zu berichtigen ist.

Damit fällt auch die zweite Hälfte der `Also seen`-Zeile vom 260824-1014 weg: die Paarung hängt
nicht mehr daran, dass `Datei::Leser` in `Datei::ALLE` als letzte TOML-Datei steht. Eine geänderte
Reihenfolge in `ALLE` liefert jetzt keine falsch gepaarte Zusicherung mit irreführendem Meldetext
mehr, weil `OHNE_LADEWEG` nach Wert filtert und nicht nach Stelle.

Die zwei Doc-Kommentare, die die Lücke ausschrieben, sind auf die neue Fassung gesetzt. Die
Berichtigung des Datensatzes ist übernommen: Schritt **8** baut den Ladeweg, nicht Schritt 7.
