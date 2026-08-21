Der neue Leerbefund-Zweig belegt den einen Sicherungsplatz mit einer Datei, die den Bestand gar nicht tragen kann

---

Der Zweig, den `073448e` in `Zugang::laden` eingezogen hat, legt eine `bookmarks.toml` ohne
einen einzigen obersten Schlüssel unter `bookmarks.toml.beschaedigt` beiseite. Eine solche
Datei trägt aber definitionsgemäß **keinen** Eintrag — der oberste Schlüssel `eintraege` ist
genau der, der fehlt. Die Sicherung ist damit wertlos, und weil es je Ablagedatei nur **einen**
Sicherungsplatz gibt und die erste dort abgelegte Fassung unangetastet bleibt, sperrt sie den
Platz gegen jede spätere Sicherung, die den Bestand wirklich enthielte.

---

**Gemessen am Baumstand `e688238`**, mit einem Programm gegen die öffentliche Schnittstelle
von `krk_core::ablage` (`Ablage::oeffnen` an einer eigenen Wurzel, `durchgang`, `laden`,
`sichern`), also über denselben Weg wie das laufende Programm. Ausgabe wörtlich:

```
Start 1 (0 Bytes)        -> Beschaedigt("die Datei traegt keinen einzigen obersten Schluessel,
                            und KRK schreibt sie nie so"), Gesichert(".../bookmarks.toml.beschaedigt")
  Sicherung: exists=true laenge=Some(0)
Start 2 (echter Bestand) -> SchonVorhanden(".../bookmarks.toml.beschaedigt")
  Sicherung-Inhalt: ""
  Meldung: Die bisherige Fassung liegt seit einem frueheren Start unter
           .../bookmarks.toml.beschaedigt und bleibt dort; .../bookmarks.toml ist beschaedigt
           und wird durch den Auslieferungszustand ersetzt: TOML parse error at line 1,
           column 3 ... unknown field `lesezeichen`, expected `eintraege`
  bookmarks.toml danach: "eintraege = []\n"
```

Der Verlauf im Klartext:

1. `bookmarks.toml` steht mit 0 Bytes da. `Datei::leerbefund` sagt für sie
   `Leerbefund::Beschaedigt`, `ohne_obersten_schluessel` sagt ja, und der neue Zweig
   (`crates/krk-core/src/ablage/mod.rs:566-579`) ruft `beiseite_legen` mit `text.as_bytes()`,
   also mit null Bytes. Unter `bookmarks.toml.beschaedigt` liegt danach eine leere Datei, und
   die Ersetzung meldet `Beiseite::Gesichert`.
2. Der Nutzer spielt seinen echten Bestand zurück, hier in der Gestalt, die die zweite Zeile
   der Messtabelle des Ausgangsdefekts beschreibt: ein oberster Schlüssel, den dieser Bau
   nicht kennt. Dieser Fall trägt den Bestand — die Namen und Pfade stehen wörtlich in der
   Datei.
3. `beiseite_legen` fragt `pfad.try_exists()` (`mod.rs:776-780`), findet die 0-Byte-Datei aus
   Schritt 1 und gibt `Beiseite::SchonVorhanden` zurück, **ohne zu schreiben**. Der Bestand
   aus Schritt 2 wird nicht gesichert.
4. Der nächste gewöhnliche Lesezeichenbefehl schreibt `eintraege = []` über
   `bookmarks.toml`. Der Bestand ist fort, und aus der Sicherung ist er nicht zurückzuholen:
   sie ist leer.

**Die Eigenschaft ist allgemeiner als der 0-Byte-Fall.** Jede Datei, die der neue Zweig fängt,
hat null oberste Schlüssel — das ist seine Eintrittsbedingung. Sie kann deshalb **nie**
`eintraege` enthalten und **nie** einen Bestand tragen. Der Zweig legt also grundsätzlich
Inhalt beiseite, aus dem sich nichts wiederherstellen lässt, und verbraucht dafür jedes Mal
den einen Platz. Die Datei aus lauter Kommentaren, die
`eine_bookmarks_toml_aus_lauter_kommentaren_gilt_als_beschaedigt`
(`crates/krk-core/tests/ablage.rs`) prüft, fällt mit darunter.

**Warum das eine Verschlechterung gegenüber `01d2365` ist.** Vor `073448e` ergab eine
`bookmarks.toml` ohne obersten Schlüssel `Ok` mit null Einträgen, also keine `Ersetzung` und
keine Sicherung. Der Platz blieb frei, und der Bestand aus Schritt 2 wäre gesichert worden.
Der Zweig, der gegen den stillen Verlust gebaut ist, macht in dieser Reihenfolge genau den
Verlust unwiederbringlich, gegen den er gebaut ist.

**Die Regel, die dabei bricht, ist nicht `SchonVorhanden`, sondern ihre Voraussetzung.** Der
Datensatz vom 260812-1105 hat den einen Platz mit „die **erste** zur Seite gelegte Fassung ist
die wertvollere" begründet (`crates/krk-core/src/ablage/atomar.rs:68-86`, `beiseitepfad`). Für
eine Fassung ohne obersten Schlüssel gilt der Satz nicht: sie ist nicht die wertvollere,
sondern die einzige, die sicher nichts enthält.

**Die Meldung sagt daneben das Gegenteil.** `Beiseite::Gesichert` erzeugt in Schritt 1 den
Satz „Die bisherige Fassung liegt unter …" über eine leere Datei, und
`Beiseite::SchonVorhanden` in Schritt 2 den Satz „Die bisherige Fassung liegt seit einem
früheren Start unter … und bleibt dort". Der Doc-Kommentar an `Beiseite`
(`crates/krk-core/src/ablage/mod.rs:272-277`) sagt zu, „dass keine Meldung eine Datei
verspricht, die es nicht gibt". Die Datei gibt es, ihr Inhalt fehlt, und der Satz liest sich
für den Nutzer wie eine Zusage über seinen Bestand.

**Ob der Nutzer die Meldung aus Schritt 2 überhaupt sieht, ist offen.** Sie ist eine
Startmeldung, und
`shared/issues/260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
misst, dass von n Startmeldungen die n-te ankommt.

## Vorschlag

Kein zweiter Mechanismus, eine Zeile weniger statt einer mehr: der neue Zweig gibt
`Beiseite::Nicht` zurück, statt `beiseite_legen` zu rufen. Er hat nichts zu sichern, `Nicht`
sagt genau das, und der Platz bleibt für die Gestalt frei, die den Bestand trägt. Der
Doc-Kommentar an `Beiseite::Nicht` (`mod.rs:280-285`) nennt heute zwei Fälle („von einer
Datei, die sich nicht lesen ließ, gibt es keinen Inhalt zu sichern, und eine fehlende Datei
ist der erste Start") und bekäme einen dritten dazu: aus einer Datei ohne obersten Schlüssel
gibt es keinen **Bestand** zu sichern.

Die Gegenrechnung gehört dazu und ist zu entscheiden, nicht abzuleiten: mit `Nicht` bleibt der
Wortlaut einer Datei aus lauter Kommentaren nicht erhalten. Die Datei selbst bleibt allerdings
liegen — `laden` überschreibt nie, und `beiseite_legen` kopiert statt umzubenennen —, sie geht
also erst beim nächsten gewöhnlichen Schreibvorgang verloren, und bis dahin hat der Nutzer die
Schadensmeldung gesehen. Wer den Wortlaut trotzdem halten will, braucht einen zweiten
Sicherungsplatz oder eine Rangfolge darauf, und beides widerspräche dem Datensatz vom
260812-1105.

Eine Probe für die Reihenfolge fehlt in jedem Fall: heute prüft keine Stelle im Baum, was
geschieht, wenn auf einen Leerbefund ein zweiter Ladevorgang mit echtem Bestand folgt.

**Schwere:** hoch. Der Ausgang ist der unwiederbringliche Verlust des Lesezeichenbestands, also
genau die Klasse, gegen die der Mechanismus der Runde 6 gebaut ist, und er ist mit `073448e`
neu erreichbar. Die Reihenfolge, die ihn auslöst, ist eng: es braucht einen Start mit einer
`bookmarks.toml` ohne obersten Schlüssel **vor** dem Start mit dem echten Bestand.

**Gefunden:** coderev, Durchsicht des Turns 1 am 260821-1023, Bereich `01d2365..e688238`

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:566-579` (der neue Zweig),
`:771-795` (`beiseite_legen`), `:272-314` (`Beiseite`),
`crates/krk-core/src/ablage/atomar.rs:68-86` (`beiseitepfad`)

**Domain:** code

**Verwandt:**
`shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`
— der behobene Ausgangsdefekt; dieser Befund steht an seiner Behebung.
`shared/issues/260821-0142_*_eine-nicht-lesbare-ablagedatei-wird-nicht-gesichert-und-vom-naechsten-schreibvorgang-ueberschrieben.md`
— dritte Gestalt desselben Verlusts, dort über `Grund::NichtLesbar`; hier über
`Grund::Beschaedigt` und `Beiseite::SchonVorhanden`, also ein anderer Zweig.
`shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`
— wird die strenge Lesart auf `session.toml` ausgeweitet, gilt dieser Befund dort mit.

---
Resolved: Der Leerbefund-Zweig in `Zugang::laden` (`crates/krk-core/src/ablage/mod.rs`) gibt
`Beiseite::Nicht` zurück, statt `beiseite_legen` zu rufen. Der Vorschlag der Durchsicht ist
übernommen, weil er sich am gebauten Ergebnis bestätigt hat: die Probe unten scheitert vor der
Änderung genau an Schritt 1 („der Leerbefund hat den einen Sicherungsplatz belegt") und läuft
danach durch. Der Zweig ist nicht zurückgenommen — eine `bookmarks.toml` ohne obersten
Schlüssel ist weiterhin kein erster Start, erzeugt weiterhin eine `Ersetzung`, und der
Auslieferungszustand springt weiterhin ein. Allein die wertlose Sicherung entfällt.

**Die Meldung ist mitgezogen, ohne dass `Display` angefasst werden musste.** Mit `Beiseite::Nicht`
greift ein anderer Zweig von `melden`, und der verspricht keine Datei. Gemessen über
`Ablage::oeffnen`, `durchgang` und `laden`, für die 0-Byte-Datei und für die Datei aus lauter
Kommentaren, Ausgabe wörtlich und für beide gleich:

```
beiseite=Nicht
meldung=.../bookmarks.toml ist beschaedigt und wird durch den Auslieferungszustand
        ersetzt: die Datei traegt keinen einzigen obersten Schluessel, und KRK
        schreibt sie nie so
platz_belegt=Ok(false)
```

Die Zusage im Doc-Kommentar an `Beiseite`, „dass keine Meldung eine Datei verspricht, die es
nicht gibt", hält damit wieder. Die Probe hält sie fest: sie prüft, dass die Meldung den
Beiseitepfad nicht nennt, statt einen Wortlaut zu vergleichen.

**Die Probe für die gemessene Reihenfolge** ist
`nach_einem_leerbefund_bleibt_der_sicherungsplatz_fuer_den_echten_bestand_frei`
(`crates/krk-core/tests/ablage.rs`), die vier Schritte dieses Datensatzes als vier Abschnitte:
0-Byte-Datei beim ersten Start und der Platz bleibt frei, echter Bestand mit fremdem obersten
Schlüssel beim zweiten und er wird gesichert, die Sicherung trägt den Bestand und nicht den
Leerbefund, der nächste gewöhnliche Schreibvorgang nimmt `bookmarks.toml` und lässt die
Sicherung stehen. Es ist die eine Probe im Baum, die zweimal lädt — genau der blinde Fleck, den
die Durchsicht an den sechs Proben des Ausgangsdefekts benannt hat.

Die drei vorhandenen Proben des Ausgangsdefekts bleiben stehen. Ihr gemeinsamer Rumpf
`beschaedigte_lesezeichen` nimmt jetzt die erwartete `Sicherungslage` als Argument: `Wortlaut`
für den fremden obersten Schlüssel, der einen Bestand trägt und gesichert wird, `Frei` für die
0-Byte-Datei und die Datei aus lauter Kommentaren. Zwei Werte und kein Wahrheitswert, damit an
der Rufstelle die Frage neben der Antwort steht.

Der Preis ist der im Vorschlag benannte und angenommene: der Wortlaut einer Datei aus lauter
Kommentaren wird nicht mehr gesichert. Die Datei bleibt liegen, `laden` überschreibt nie; sie
geht erst beim nächsten gewöhnlichen Schreibvorgang verloren, und bis dahin hat der Nutzer die
Schadensmeldung gesehen. Ein zweiter Sicherungsplatz oder eine Rangfolge darauf ist nicht
gebaut worden, beides widerspräche dem Datensatz vom 260812-1105.

**Kein zweiter Mechanismus, und eine Zeile weniger statt einer mehr.**
`nur_benannte_dateien_erreichen_das_atomare_schreiben` (`crates/krk-core/tests/baum.rs`) zählt
unverändert fünf Dateien: `beiseite_legen` bleibt, es hat nur einen Rufer weniger.
`Datei::leerbefund` ist unangetastet, die drei übrigen TOML-Dateien und die zwei Zettel laufen
wie zuvor — `eine_leere_datei_meldet_bei_den_drei_uebrigen_toml_dateien_nichts` läuft grün.

Vier Prosastellen sind mitgezogen, weil die Änderung sie falsch gemacht hätte: die erste der
vier Regeln im Modulkopf („Nur eine beschädigte Datei wird gesichert" — jetzt „und auch die
nicht immer"), der Absatz „Zwei Stellen beantworten die weitere Frage", der nicht mehr sagt,
beide mündeten in `beiseite_legen`, der Doc-Kommentar an `Beiseite::Nicht` (zwei Fälle, jetzt
drei, samt der Begründung, warum der dritte nicht doch sichert) und der Doc-Kommentar an
`Zugang::laden`. Keine davon gehört zu den sieben Stellen des Befunds „Niedrig"
(`260821-1023_*_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`);
deren Zeilennummern haben sich allerdings verschoben — `mod.rs:142` steht jetzt auf `:150`,
`:425` auf `:460`, `:427` auf `:462`, `:467` auf `:502`, `:508` auf `:543`. Der Wortlaut der
sieben ist unverändert.

Abnahme: `make check`, Exit 0.
