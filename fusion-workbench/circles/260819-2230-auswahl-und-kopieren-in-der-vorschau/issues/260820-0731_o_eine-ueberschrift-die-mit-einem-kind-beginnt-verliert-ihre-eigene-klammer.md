Eine Überschrift, die mit einem Kind beginnt, verliert ihre eigene Klammer — eine Auswahl in ihrem Schwanz liefert den Text ohne `#`

---

`Zerlegung::klammer_verbuchen` (`crates/krk-ui/src/markdown.rs:1044-1075`) verbucht die
Klammer beim **innersten** offenen Element und beschneidet den Abschnitt dabei auf dessen
Quellbereich. Beginnt ein Element aus Zeichen (`Inhaltsart::Zeichen`) unmittelbar mit einem
Kind, so fallen seine eigenen Auszeichnungszeichen in den ersten Abschnitt des **Kindes**,
werden dort verbucht und dem Vater nie zugeschrieben. Der Vater bleibt ohne Klammer, und
eine Auswahl hinter dem Kind zieht ihn nicht mehr mit.

C2.2 des Specs sagt zu: „Wer eine als große fette Zeile dargestellte Überschrift kopiert,
hat `# Überschrift` in der Zwischenablage." Für eine Überschrift, die mit einer Betonung,
einem Quelltextstück oder einem Verweis beginnt, gilt das nicht mehr.

---

**Gemessen, nicht erschlossen.** Kopie des Baums auf dem Stand `b28cdd6`, eine Probe im
Prüfmodul von `markdown.rs`, `cargo test -p krk-ui`:

```
"# **Titel** und noch ein Stueck Text\n"      Auswahl "noch ein"  -> "noch ein"
"## `code` und noch ein Stueck Text\n"        Auswahl "noch ein"  -> "noch ein"
"# [V](https://e.com) und noch ein Stueck\n"  Auswahl "noch ein"  -> "noch ein"
```

Zum Vergleich dieselbe Frage an einer Überschrift ohne führendes Kind:

```
"# Titel\n"                                   Auswahl "itel"      -> "# Titel\n"
```

Die Klammerliste zeigt den Grund unmittelbar: bei `"# **Titel** hier\n"` steht
`[(0..17, false), (2..11, true)]` — die Überschrift `false`, die Betonung `true`.

**Warum es geschieht.** Innerhalb eines Elements aus Zeichen rückt der Lesestand nicht vor
(`Zerlegung::luecke_bis` kehrt für `Inhaltsart::Zeichen` ohne Abtrag zurück,
`markdown.rs:1253-1268`). Der erste Schreibvorgang im Kind legt deshalb einen Abschnitt an,
dessen Quellbereich beim Anfang des **Vaters** beginnt und die Auszeichnungszeichen beider
trägt. `klammer_verbuchen` beschneidet ihn auf den Bereich des innersten Elements
(`markdown.rs:1070-1074`) — die Bytes des Vaters fallen dabei weg. Der Doc-Kommentar an
dieser Stelle benennt die Beschneidung und nennt `# **fett**` sogar als Beispiel, zieht aber
den Schluss für die Klammer des Vaters nicht.

**Warum es an den vorhandenen Proben nicht auffällt.** `eine_auswahl_in_einer_ueberschrift_liefert_ihr_doppelkreuz`
(`markdown.rs:2618-2622`) prüft `"# Überschrift\n"`, also eine Überschrift ohne führendes
Kind, und ihre Auswahl liegt im ersten Abschnitt, dessen Quellbereich das `# ` ohnehin
mitträgt. Die Klammer der Überschrift geht dort in die Antwort gar nicht ein.

**Wurzel und Richtung.** Dieselbe Funktion wie beim Befund
`260820-0728_o_ein-absatz-mit-entitaet-…`, und die beiden Befunde zeigen in
entgegengesetzte Richtungen: dort bekommt ein Element eine Klammer, das keine haben soll,
hier bekommt eines keine, das eine haben muss. Beide verschwinden, wenn die Klammer an den
**Vorspann und den Nachspann des Elements selbst** gebunden wird — also an die Bytes
zwischen dem Anfang seines Quellbereichs und dem ersten darin geschriebenen Zeichen sowie
zwischen dem letzten und seinem Ende — statt an die Art eines beliebigen Abschnitts im
Inneren. Diese Auskunft steht dem Durchgang zur Verfügung: `Offen::quelle` und
`Offen::anfang` liegen beim Öffnen fest, der Lesestand beim Schließen.

**Schwere:** hoch. Das Ergebnis ist stilles, unvollständiges Markdown: eine kopierte
Überschriftshälfte kommt als gewöhnlicher Absatz in der Zieldatei an.
**Baumstand:** `b28cdd6`.
