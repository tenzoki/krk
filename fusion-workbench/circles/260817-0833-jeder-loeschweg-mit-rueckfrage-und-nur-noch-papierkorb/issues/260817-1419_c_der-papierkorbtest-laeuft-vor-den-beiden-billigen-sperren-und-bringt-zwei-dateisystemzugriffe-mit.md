Der Papierkorbtest läuft vor den beiden billigen Sperren und bringt zwei Dateisystemzugriffe auf den Hauptfaden mit

---

`loeschen_nach_rueckfrage` erhebt alle drei Tatsachen der Stufenregel, bevor eine Stufe
entschieden ist. Damit laufen `std::fs::canonicalize` und `papierkorb::fuehrt_einen_papierkorb`
auch dann, wenn schon ein Vorgang läuft oder die Auswahl leer ist — in Lagen also, in denen der
Befehl nichts tut. Auf einem hängenden Netzlaufwerk blockiert ein `delete` ohne Auswahl jetzt
den Hauptfaden, wo vorher eine Abfrage im Speicher genügte.

---

**Schwere:** Niedrig. Kein falsches Ergebnis: die Tafel führt für die ersten beiden Zeilen
„gleichgültig", also ändert die Reihenfolge der Erhebung am Ausgang nichts. Der Preis ist
Verzögerung an einer Stelle, an der der Nutzer keine erwartet.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:4655-4668`,
`crates/krk-ui/src/appkit/papierkorb.rs:78-86`
**Verwandt:** `issues/260817-1108_o_die-loeschfrage-entsteht-vor-beiden-sperren-und-im-leeren-fall-mit-null-eintraegen.md`
— dieselbe Stelle und dasselbe Muster, andere Kosten: dort verworfene Texte, hier zwei
Zugriffe auf das Dateisystem. Schritt 11 löst nach seinem heutigen Zuschnitt nur den ersten.
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

```rust
// crates/krk-ui/src/appkit/anwendung.rs:4655-4668
let vorgang_laeuft = self.ivars().vorgang.borrow().is_some();
let quelle = self.dateifenster(aktiv).quelle();
let auswahl = quelle.betroffene_eintraege();
let quellordner = quelle.angezeigter_ordner();
let papierkorb_am_ziel = std::fs::canonicalize(&quellordner)
    .map_or(Befund::Unentschieden, |aufgeloest| {
        papierkorb::fuehrt_einen_papierkorb(&aufgeloest)
    });
```

Der Kommentar darüber (`:4649-4654`) begründet die Reihenfolge ausdrücklich: „alle drei
erhoben, bevor eine Stufe entschieden ist. Die Reihenfolge, in der sie hier anfallen,
entscheidet nichts". Für das **Ergebnis** stimmt das und ist gut so — die Entscheidung liegt in
`vor_der_rueckfrage`. Für die **Kosten** stimmt es nicht.

## Was das kostet

`canonicalize` löst jedes Glied des Pfades auf und braucht dafür Zugriffe auf das Dateisystem;
`fuehrt_einen_papierkorb` ruft `NSFileManager.URLForDirectory:…` mit dem Ordner als
`appropriateForURL:`. Beides läuft auf dem Hauptfaden und je Löschbefehl, jetzt auch in diesen
zwei Lagen:

- Es läuft schon ein Vorgang. Die Meldung darüber hätte ohne jeden Zugriff feststanden.
- Es ist nichts ausgewählt. `delete` auf einer leeren Auswahl war vor dieser Änderung eine
  Abfrage im Speicher.

Der benannte Rest in `papierkorb.rs:85-86` deckt allein den zweiten der beiden Aufrufe: „haengt
der Datentraeger unter dem angezeigten Ordner, verzoegert sich das Blatt um die Antwort des
Systems". `canonicalize` steht in keinem Modulkopf als Kostenstelle, und es öffnet dieselbe
Verzögerung eine Zeile früher, in einer anderen Datei.

Keine der zehn Zusagen aus C8 der Runde 1 vermisst diese Spanne; das ist in `papierkorb.rs:83`
nachgelesen und stimmt. Der Befund hängt nicht an einer Zusage, sondern daran, dass ein Befehl,
der nichts tut, dafür auf das Dateisystem greift.

## Richtung

Die Reihenfolge bleibt, wo sie ist: in der Tafel. Der Rumpf darf den dritten Befund faul
erheben, ohne die Regel anzufassen — `vor_der_rueckfrage` bekommt statt eines `Befund` einen
`impl FnOnce() -> Befund`, oder der Rumpf fragt die Regel zweimal, einmal ohne und einmal mit
dem Papierkorbbefund. Die erste Form hält die Tafel unverändert und ist ohne Fenster prüfbar
wie heute; die zweite verdoppelt den Aufruf und damit die Aufruferzählung, die gerade auf eins
festgeschrieben ist.

Schritt 11 fasst denselben Rumpf an. Wenn der Zuschnitt dort ohnehin geändert wird, gehört
dieser Befund dazu.

Hinweis (260817, Aufgabe T5b): Im Ausschnitt oben heißt der Typ inzwischen `Loeschzielbefund`
statt `Befund`; die Umbenennung stammt aus
`260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`
und ändert am Befund nichts.

---
Resolved: 260817-1806 (coder, Aufgabe T10, Schritt 11 des Plans). Genommen ist die **erste** der
beiden Formen aus **Richtung**: `vor_der_rueckfrage` bekommt statt eines `Loeschzielbefund` ein
`impl FnOnce() -> Loeschzielbefund` und ruft es allein im Feld `(false, false)` seiner Tafel.
Die Aufruferzählung bleibt damit auf eins festgeschrieben, und die Regel ist ohne Fenster
prüfbar wie zuvor.

**Die Reihenfolge der Stufen ist unverändert**, und das war die Bedingung. Sie steht weiter in
der Tafel und nirgends sonst; verschoben hat sich allein, **wann** die teure Tatsache anfällt,
und das entscheidet jetzt die Regel selbst statt ihres Aufrufers. Die beiden ersten Zeilen der
Tafel tragen in der Spalte des Papierkorbbefunds jetzt „ungefragt" statt „gleichgültig" — am
Ausgang ändert das nichts, am Preis alles.

**Gemessen und nicht behauptet:** die Probe
`die_teure_tatsache_bleibt_in_den_ersten_zwei_stufen_ungefragt` zählt die Aufrufe des Abschlusses
über eine `Cell` und erwartet null in den drei Kombinationen, in denen eine billige Stufe schon
anhält, und genau eins im vierten Feld. Die letzte Zeile ist die Gegenprobe: ohne sie wäre eine
Regel, die den Papierkorbtest **nie** ruft, grün und die dritte Stufe stillgelegt.

**Mitgenommen ist mehr als der Befund verlangt.** Auch die fünf Tatsachen der Auslösertafel
fallen erst im vierten Zweig an, und die teuerste davon ist `umfang::zaehlen` mit bis zu 26
geöffneten Verzeichnissen. Ein Löschbefehl, den ein laufender Vorgang oder eine leere Auswahl
anhält, greift damit gar nicht mehr auf das Dateisystem. Für `f8` (endgültiges Löschen, bis
Bündel D) fallen sie überhaupt nicht an, weil seine Texte aus `operationen::loeschfrage` kommen
und keinen Warngrund tragen.

Verifikation: `make check` — exit 0. Die Abrechnung im Einzelnen steht in
`history/260817-1806-coder-t10-die-laute-warnform.md`.

---
**Addendum 260818-0201 (analyst).** The `Resolved: 260817-1806` line above states a resolution time that no
clock produced. The work it closes is in commit `792995a`, author time `260817-1739` — 27 minutes *earlier* than the
time the line claims. A pass that orders closures against commits gets this one out of order, so the
author time is what binds; the line itself stays as written, because this store keeps a record of a
state in the wording it had. Finding:
`issues/260817-1807_*_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md`.
