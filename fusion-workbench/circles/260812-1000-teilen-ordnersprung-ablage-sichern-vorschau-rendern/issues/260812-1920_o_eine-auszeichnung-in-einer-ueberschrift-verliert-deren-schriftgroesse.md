Eine Auszeichnung in einer Überschrift verliert deren Schriftgröße, und die beiden Datensätze zur Rangordnung nennen diesen Fall nicht

---

`## Ein **fetter** Teil` zeigt „fetter" in der Grundgröße statt in der
Überschriftsgröße. Der Fall gehört zu derselben Ursache wie die zwei bereits
aufgeschriebenen — `addAttributes:` ersetzt bei gleichem Merkmalsnamen —, aber
**keiner der beiden Datensätze nennt ihn**, und der dort genannte Weg zur
Behebung würde ihn nicht beheben.

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:152`,
unverändert in ein Prüfprogramm kopiert; die Wirkung in AppKit aus
`crates/krk-ui/src/appkit/textmerkmale.rs:204-215` gelesen):

```
Quelle : "# Titel **fett** danach"
Auszeichnungen: Ueberschrift{1}(0,17), StarkeBetonung(6,4)
  -> 0..17 bekommt boldSystemFontOfSize(grundgroesse * 1.7)
  -> 6..10 bekommt danach boldSystemFontOfSize(grundgroesse)
  -> "fett" steht in der Grundgroesse mitten in einer Ueberschrift.

Quelle : "# Titel `code` danach"
Auszeichnungen: Ueberschrift{1}(0,17), FesteSchrift(6,4)
  -> "code" steht in fester Schrift, aber in der Grundgroesse.

Quelle : "## Ein *kursiv* Teil"
Auszeichnungen: Ueberschrift{2}(0,15), Betonung(4,6)
  -> "kursiv" steht kursiv, aber in der Grundgroesse.
```

Die Faktoren stehen in `UEBERSCHRIFTSFAKTOREN`
(`textmerkmale.rs:136`): 1,7 für Stufe 1 bis 1,05 für Stufe 6. Bei Stufe 1
verliert das ausgezeichnete Stück 41 Prozent seiner Höhe gegenüber seinen
Nachbarn in derselben Zeile.

**Warum die vorhandenen Datensätze ihn nicht abdecken.**

- `260812-1805_o_der-ueberschneidungssatz-in-textmerkmale-anwenden-gilt-seit-markdown-rs-nicht-mehr.md`
  nennt in seinem Abschnitt „Was fehlt: Punkt 2" genau zwei Paarungen: „Fett
  **und** kursiv oder feste Schrift **und** fett". Die Überschrift kommt
  darin nicht vor. Sein gemessener Überschriftsfall
  (`` # `Code` im Titel ``) wird sogar als „wie beabsichtigt" verbucht — die
  feste Schrift gewinnt —, ohne dass die dabei verlorene Größe genannt würde.
- `260812-1851_d_zwei-schriftschnitte-legen-sich-nicht-zusammen-fett-in-kursiv-bleibt-aufrecht.md`
  (vom Nutzer zurückgestellt) nennt dieselben zwei Fälle und keinen dritten.
- Der berichtigte Kommentar in `textmerkmale.rs:229-256` schreibt „der
  Quelltext in einer Ueberschrift bekommt seine feste Schrift" als das
  gewollte Ergebnis auf, ohne zu sagen, dass er dabei auf Grundgröße fällt.

**Der genannte Behebungsweg trägt hier nicht.** Beide Datensätze nennen
`NSFontDescriptor`-Merkmale beziehungsweise `applyFontTraits:range:`. Diese
legen **Schnitte** zusammen, nicht **Größen**: `TraitBold | TraitItalic` löst
fett-in-kursiv, aber der Größenverlust entsteht dadurch, dass der innere
Eintrag mit `NSFont::boldSystemFontOfSize(grundgroesse)` eine ganz neue
Schrift setzt. Wer nur die Schnitte zusammenlegt, hat den Fall hier nicht
angefasst. Die Größe müsste aus der bereits gesetzten Schrift der Stelle
übernommen werden — also derselbe „Schriftzustand je Stelle", den Punkt 2
verlangt, aber mit einer zweiten Größe im Zustand.

**Es ist keine Verschlechterung dieses Turns.** Die fünf Werte von
`Auszeichnung` und der zweite Erzeuger `crate::markdown` stammen aus
Planschritt 8, also aus Turn 2. Der Datensatz steht hier, weil der Turn-3-Lauf
den Datensatz zur Rangordnung ausdrücklich um einen Absatz „was fehlt"
ergänzt hat, und dieser Absatz die Lücke nicht vollständig nennt.

**Was zu tun ist:** Den Fall in die Beschreibung dessen aufnehmen, was noch
fehlt — in `260812-1805_o_…überschneidungssatz…` und in den zurückgestellten
Datensatz —, samt der Feststellung, dass ein reines Zusammenlegen der Schnitte
ihn nicht löst. Ob er behoben wird, gehört zur zurückgestellten Frage und wird
hier nicht vorweggenommen.

**Gewicht:** mittel. Sichtbarer und häufiger als die beiden aufgeschriebenen
Fälle — `## Ein **fetter** Teil` und `## \`Kommandoname\` erklärt` kommen in
den Markdown-Dateien dieses Projekts selbst reihenweise vor —, aber es ist eine
Anzeigefrage einer nicht bearbeitbaren Vorschau, keine falsche Auskunft über
den Inhalt.

**Herkunft:** Circle der Runde 6, Planschritt 8 (C4.2); aufgefallen bei der
Nachprüfung des Turn-3-Nachtrags an `260812-1805_o_…überschneidungssatz…`.
