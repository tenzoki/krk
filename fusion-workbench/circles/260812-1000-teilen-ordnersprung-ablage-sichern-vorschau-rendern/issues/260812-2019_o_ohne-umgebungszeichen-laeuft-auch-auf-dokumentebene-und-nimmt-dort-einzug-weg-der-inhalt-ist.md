`ohne_umgebungszeichen` läuft auch auf Dokumentebene und nimmt dort einen Einzug weg, der Inhalt ist

---

`ohne_umgebungszeichen` (`markdown.rs:394-400`) sagt in seinem
Doc-Kommentar, es nehme einer Lücke, „was ihre **Umgebung** auf jeder ihrer
Zeilen wiederholt" — das `>` eines Zitats und der Einzug eines Punktes.
`Zerlegung::luecke_bis` ruft es aber unbedingt, also auch dann, wenn gar kein
Element offen ist. Auf Dokumentebene gibt es keine Umgebung, die etwas
wiederholt; der führende Leerraum dort ist Inhalt und wird trotzdem
abgeschnitten.

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:182`,
beide Fassungen unverändert in dasselbe Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`):

```
Quelle : "[ZIEL]: http://z.example\n      \"Titel\"\n"
f401dcc: "[ZIEL]: http://z.example\n      \"Titel\""
c35f8b1: "[ZIEL]: http://z.example\n\"Titel\""

Quelle : "- Text\n\n  [ZIEL]:\n      http://z.example\n"
c35f8b1: "• Text\n\n[ZIEL]:\nhttp://z.example"
```

Der Einzug der Fortsetzungszeile ist weg. In der zweiten Zeile steht er für
die Zugehörigkeit zur Verweisdefinition, nicht für eine Umgebung.

**Die Ursache.** `luecke_bis` (`markdown.rs:652-682`) ruft
`ohne_umgebungszeichen` an genau einer Stelle, hinter der Prüfung auf
`Inhaltsart::Bloecke` und hinter dem Vorspann-Zweig. Beide Zweige laufen nur,
wenn `self.offen.last()` etwas liefert; ist der Stapel leer, fällt der Ablauf
unmittelbar auf den Aufruf durch. Vor `c35f8b1` stand dort
`quelle[self.gelesen..bis].trim()`, was nur am Anfang und am Ende der ganzen
Lücke schneidet und jede Zeile dazwischen unangetastet lässt.

**Wie weit es reicht.** Auf Dokumentebene ist eine Lücke nur dann mehrzeilig
und eingerückt, wenn `pulldown-cmark` sie gar nicht meldet — praktisch die
mehrzeilige Verweisdefinition mit Titel oder umbrochener Adresse. Ein Block
mit vier Leerzeichen Einzug wäre ein Quelltextblock und käme als Ereignis.

**Ein `>` mitten in einer Zeile bleibt stehen.** `trim_start_matches` schneidet
nur vorn, und die Probe dazu ist gemessen:

```
Quelle : "> Text\n>\n> [ZIEL]: http://z.example \"> Titel\"\n"
c35f8b1: "Text\n\n[ZIEL]: http://z.example \"> Titel\""
```

Der Titel behält sein `>`.

**Keine Probe fängt es.** `ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text`
(`markdown.rs:1273`) misst den Fall **im** Zitat, also den, für den die
Funktion geschrieben ist. Der Dokumentebenen-Fall ist ungemessen.

**Ein Zuschnitt** (nicht gewählt): `luecke_bis` könnte
`ohne_umgebungszeichen` nur dann rufen, wenn ein Element offen ist, und auf
Dokumentebene beim `trim()` bleiben. Das wären zwei Wege statt einem — der
Modulkopf hält seine Deckungssätze ausdrücklich mechanisch und ohne
Fallaufzählung, und diese Unterscheidung wäre eine. Die Alternative ist, den
Doc-Kommentar der Funktion an das anzupassen, was sie tut.

**Gewicht: niedrig.** Kosmetisch, seltene Quelle, kein Inhaltsverlust — die
Zeichen stehen da, nur der Einzug nicht. Der Befund ist die Abweichung
zwischen dem Doc-Kommentar und dem Aufrufort.

**Herkunft:** Circle der Runde 6, Turn 4, `c35f8b1`.
