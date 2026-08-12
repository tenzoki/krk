YAML-Front-Matter erscheint in der Vorschau als Trennlinie und Überschrift statt als Quelltext

---

Eine Markdown-Datei mit YAML-Front-Matter — die übliche Form in Obsidian, Hugo
und Jekyll — wird von `markdown::rendern` nicht als Front-Matter erkannt. Die
erste `---`-Zeile wird zur Trennlinie, und die zweite `---`-Zeile macht aus der
ersten Metadatenzeile eine Setext-Überschrift der Stufe 2. Der Nutzer sieht
oben in der Vorschau eine große fette Zeile mit dem Inhalt „title: Sache".

---

**Gemessen** (mit `markdown::rendern` aus
`crates/krk-ui/src/markdown.rs`, unverändert in ein Prüfprogramm kopiert):

```
Quelle : "---\ntitle: Sache\n---\n\nText\n"
Ausgabe: "---\n\ntitle: Sache\n\nText"
          Ueberschrift{stufe: 2} über "title: Sache"
```

Der Rest des Front-Matter-Blocks — jede Zeile außer der ersten — verschwindet
ebenso wenig wie sie erscheint: bei mehr als einem Schlüssel wird der Block zu
einem Absatz plus Setext-Überschrift, und die Reihenfolge liest sich
verschoben.

**Was daran gegen die Zusage steht.** Das dritte Abnahmekriterium von C4 sagt:
„Alles außerhalb dieses Umfangs erscheint als der Quelltext, der dasteht."
Front-Matter liegt außerhalb des Umfangs und erscheint weder als Quelltext noch
richtig gerendert, sondern als etwas Drittes: als Überschrift, die die Datei
nicht hat.

**Es ist CommonMark-treu und trotzdem falsch für diese Vorschau.** Jeder
CommonMark-Zerleger ohne Front-Matter-Erweiterung tut dasselbe; der Fehler
liegt nicht in `pulldown-cmark`. Die Kiste bietet dafür
`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS`; damit lieferte sie
`Tag::MetadataBlock`, und die vorhandene Auffangregel
(`Behandlung::Woertlich`, `markdown.rs:196-198`) gäbe den Block wörtlich aus,
ohne dass eine Sonderregel entstünde. Ein Merkmal ist dafür nicht nötig, die
Option ist Teil des Grundumfangs der Kiste.

**Die Gegenrechnung gehört dazu.** `Options::empty()` ist im Plan ausdrücklich
gewählt, und jede Option, die dazukommt, verändert die Zerlegung auch an
anderen Stellen. Die Wahl zwischen „Option einschalten" und „Front-Matter als
Überschrift hinnehmen" ist eine Entscheidung und keine Zeile.

**Gewicht:** niedrig. Betrifft eine Dateiform, die in diesem Projekt nicht
vorkommt, aber in fremden Ordnern häufig ist; die Anzeige ist falsch, nicht
kaputt.

**Herkunft:** Circle der Runde 6, Planschritt 8 (C4.3).
