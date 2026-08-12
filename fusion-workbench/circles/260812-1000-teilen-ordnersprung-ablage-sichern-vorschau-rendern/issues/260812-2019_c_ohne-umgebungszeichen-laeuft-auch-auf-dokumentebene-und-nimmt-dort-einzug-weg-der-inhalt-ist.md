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

---

**Resolved 260812** — `luecke_bis` ruft `ohne_umgebungszeichen` nur noch
innerhalb eines Elements; auf Dokumentebene bleibt es beim `trim()` der
Vorfassung. Das ist der Zuschnitt, den der Datensatz erwogen und nicht gewählt
hat, und der Einwand dagegen trägt bei näherem Hinsehen nicht.

**Es sind keine zwei Wege, sondern die zwei Sätze, die es ohnehin gibt.**
Der Einwand lautete, eine Fallunterscheidung stünde gegen den Modulkopf, der
seine Deckungssätze mechanisch und ohne Fallaufzählung hält. Gefragt wird aber
genau dasselbe `self.offen.is_empty()`, an dem Satz 1 und Satz 2 seit jeher
auseinandergehen — keine neue Frage, keine Aufzählung von Ereignisarten,
sondern die bestehende Grenze, an die eine bestehende Regel gehängt wird. Der
Modulkopf sagt Satz 1 jetzt ganz aus: dort wiederholt keine Umgebung etwas,
also ist der Leerraum Inhalt.

**Der gemessene Fall, am Baum nachgemessen:**

```
"[ZIEL]: http://z.example\n      \"Titel\"\n"
  -> "[ZIEL]: http://z.example\n      \"Titel\""
```

Der Einzug der Fortsetzungszeile steht wieder da, wie in `f401dcc`.

**Neue Probe:** `auf_dokumentebene_bleibt_der_einzug_einer_zeile_stehen`. Gegen
den Zustand vor der Behebung gegengeprüft — der Dokumentebenen-Zweig wurde
probeweise abgeschaltet, und sie schlug fehl. Die Gegenprobe im Zitat
(`ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text`) läuft
unverändert durch.

**Der Doc-Kommentar der Funktion sagt jetzt auch, von wo sie gerufen wird**,
also die Hälfte, deren Fehlen den Befund ausmachte.

**Nebenbei ist die Laufzeit zurückgekommen.** Der Aufruf lag im heißen Weg:
für jede Lücke zwischen zwei Blöcken legte er einen `Vec<&str>` und zwei
`String` an, und auf Dokumentebene ist das jede Blockgrenze einer Datei. Jetzt
steht dort ein `Cow::Borrowed` auf das `trim()` der Quelle, ohne jede
Zuweisung. Auf einer 1,05-MB-Quelle (Profil release, bestes von zwölf Läufen
zu je bestem von fünf, dieselbe Maschine, derselbe Stub wie in der Durchsicht):
`f401dcc` 18,5 ms, `c35f8b1` 23,0 ms, jetzt **20,9 ms**. Rund die Hälfte der
Verschlechterung aus `c35f8b1` ist damit weg.

**Der zweite gemessene Fall bleibt, wie er war**, und er ist nicht dieser
Defekt:

```
"- Text\n\n  [ZIEL]:\n      http://z.example\n"
  -> "• Text\n\n[ZIEL]:\nhttp://z.example"
```

Hier steht ein Element offen, also greift die Funktion bestimmungsgemäß — sie
soll dort den Einzug des Punktes wegnehmen. Dass sie mehr wegnimmt als die
Umgebung wiederholt, ist ein eigener und kleinerer Befund; er ist als
`260812-2140_o_ohne-umgebungszeichen-nimmt-innerhalb-eines-elements-mehr-einzug-weg-als-die-umgebung-wiederholt.md`
abgelegt und hier nicht mitbehoben.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0.
