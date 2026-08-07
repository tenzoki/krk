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
