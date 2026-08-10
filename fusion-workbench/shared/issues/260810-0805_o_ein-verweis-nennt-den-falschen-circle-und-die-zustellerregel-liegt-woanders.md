Ein Verweis nennt den falschen Circle, und die Zustellerregel liegt woanders

---

`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md:7` führt unter `**Cross-references:**` den Pfad

```
circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md
```

Diese Datei gibt es nicht. Der Datensatz liegt im Circle der Runde 1:

```
circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md
```

Geprüft am 260810-0805: `find fusion-workbench -name '260805-0713*'` liefert genau einen Treffer, und der steht im Circle der Runde 1.

---

## Warum das mehr ist als ein Tippfehler

Der Verweis trägt die Klammer „die Zustellerregel, zitiert wo sie liegt". Genau das leistet er nicht. Die Herkunftsregel aus `rules/fusion-workbench-conventions.md`, Abschnitt `## Origin Rule`, sagt: „Reach is cited, never placed. One record, one location, many citations." Ein Verweis, der die Stelle falsch nennt, hebt den einen Nutzen dieser Regel auf — wer ihm folgt, findet nichts und legt im Zweifel eine zweite Fassung an.

Der Fehler ist auch nicht die bekannte Sorte. Der offene Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` führt Verweise mit **veraltetem Zustandsmarker**; die lösen sich, indem man den Marker durch eine Sternstelle ersetzt. Hier stimmt der Marker (`_i_`), und falsch ist das **Verzeichnis**. Eine Sternstelle repariert das nicht.

## Umfang

Der Datensatz führt in derselben Zeile einen zweiten Verweis auf denselben Speicher, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`, und der stimmt. Es ist also eine Zeile mit zwei Verweisen auf denselben Circle, von denen einer den Circle vertauscht. Ungeprüft ist, ob die übrigen vier Datensätze desselben Circles denselben Fehler tragen; wer den Defekt anfasst, prüft alle fünf.

## Zuständigkeit

`ontocoder`. Es ist eine Zeile in einem Datensatz, kein Programmteil.

---

**Gefunden von:** reconciler, Abschluss-Abgleich der Sitzung 260810-0244
**Domain:** data
**Schwere:** Low
**Betroffen:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md:7`
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` (die verwandte, aber andere Sorte)

Warum im gemeinsamen Speicher und nicht im aktiven Circle: der Defekt sitzt in einem vorgesehenen Circle, den die Directive dieser Sitzung nicht berührt. Die Herkunftsregel nennt genau diesen Fall — ein Fund neben der Arbeit, nicht aus ihr.
