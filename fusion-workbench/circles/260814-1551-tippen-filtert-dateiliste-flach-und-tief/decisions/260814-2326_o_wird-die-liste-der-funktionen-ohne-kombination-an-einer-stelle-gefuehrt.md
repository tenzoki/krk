# Wird die Liste der Funktionen ohne Kombination an einer Stelle geführt?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `crates/krk-core/tests/belegung.rs:86-133` (`OHNE_KOMBINATION_AB_WERK`, seit E2b vierstellig); `crates/krk-ui/src/belegungsausgabe.rs:516-591` (`jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`, das zweite Literal); `issues/260814-2320_o_drei-proben-fuehren-die-funktionen-ohne-kombination-als-liste-und-e2-macht-sie-vierstellig.md` (Punkt 3 seiner Was-zu-tun-Liste); `resources/default-keymap.toml:385-408`

---

## Question

Dieselbe Aufzählung — welche Funktionen ab Werk ohne Tastenkombination ausgeliefert werden — steht an zwei Stellen im Baum: als `OHNE_KOMBINATION_AB_WERK` in der Prüfkiste von `krk-core` und als Literal im Rumpf einer Probe in `krk-ui`. Beim vierten Eintrag, `tiefe_suche_umschalten`, sind sie auseinandergelaufen und haben drei Proben zugleich rot gemacht. Der Nachzug E2b hat beide von Hand gleichgezogen und einen gegenseitigen Verweis in die Kommentare gesetzt; ob es dabei bleibt, ist offen.

Zwei Aufzählungen derselben Sache an zwei Stellen sind die Bauform, die dieses Projekt sonst vermeidet. Die Frage ist nicht, ob das stört, sondern was es kostet, sie zusammenzulegen — und die Antwort ist an jeder der drei Möglichkeiten eine andere.

## Vorab: die zwei Listen sagen heute nicht dasselbe

Das ist der Befund, ohne den die Möglichkeiten nicht zu bewerten sind. Beide Listen nennen dieselben vier Kennungen, aber sie behaupten Verschiedenes:

- `krk-core` prüft **eine Richtung**: keine Funktion außerhalb der Liste steht ohne Kombination da. Eine Funktion **in** der Liste, die später doch eine Kombination bekommt, fällt still durch den Zweig `None if OHNE_KOMBINATION_AB_WERK.contains(…) => {}` und löst nichts aus.
- `krk-ui` prüft **beide Richtungen**: sein `assert_eq!` vergleicht den ganzen Vektor der unbelegten Funktionen mit dem Literal, in Inhalt und Reihenfolge. Bekommt einer der vier eine Taste, fällt die Probe.

Wer die `krk-ui`-Liste ersatzlos streicht, verliert die Gegenrichtung. Wer sie behält, führt sie doppelt. Das ist der eigentliche Gegenstand der Frage.

## Options

1. **Es bleibt bei zwei Listen mit gegenseitigem Verweis** — der Stand nach E2b.
   - Pro: nichts umzubauen. Beide Proben bleiben in ihrer Kiste lesbar, jede nennt die Funktionen beim Namen, und der Kommentar jeder Liste zeigt auf die andere. Die Zusagen bleiben unverändert, in beiden Richtungen.
   - Kontra: die Verweise sind Prosa und binden keinen Übersetzer. Der nächste fünfte Eintrag läuft wieder auseinander, wenn niemand den Kommentar liest — genau so ist der jetzige Fehlstand entstanden. Der Preis fällt nicht heute an, sondern beim nächsten Mal.
2. **Die Liste zieht in die Bibliothek von `krk-core`**, als `pub const` neben `Belegung`, und beide Proben lesen sie.
   - Pro: eine Stelle, vom Übersetzer gehalten. `krk-ui` hängt ohnehin an `krk-core` und erreicht die Konstante ohne weiteres Zutun.
   - Kontra: **ein `#[cfg(test)]` genügt dafür nicht.** Eine Integrationsprobe unter `crates/krk-core/tests/` bindet die Bibliothek ohne `cfg(test)`; die Konstante müsste echter, ausgelieferter Bibliothekscode sein. Damit trüge das Bündel eine Aufzählung mit, die zur Laufzeit niemand liest und die aus `Belegung::auslieferung()` jederzeit abzuleiten wäre. Wer die Belegungsdatei ändert, könnte die Probe künftig grün machen, indem er Auslieferungscode anfasst — die Prüfung verlöre ihren Gegenstand, weil beide Seiten des Vergleichs dann demselben Autor gehören.
3. **Die Liste bleibt allein in `krk-core`, und die Gegenrichtung zieht mit.** Das Literal in `krk-ui` entfällt; `krk-core` bekommt dafür die fehlende zweite Richtung, eine Schleife über `OHNE_KOMBINATION_AB_WERK`, die zu jeder Kennung eine Funktion mit leerer Tastenliste verlangt.
   - Pro: eine Stelle, kein ausgelieferter Code, und **keine Zusage geht verloren** — die Gegenrichtung steht danach dort, wo die Liste steht. Die Probe in `krk-ui` fällt auf ihren eigentlichen Gegenstand zurück, die Markdown-Ausgabe; welche Funktionen ab Werk unbelegt sind, ist eine Aussage über `resources/default-keymap.toml` und gehört zur Belegungsprüfung, nicht zur Ausgabeprüfung.
   - Kontra: der Leser der `krk-ui`-Probe erfährt nicht mehr an Ort und Stelle, **welche** Funktion aus der Datei fällt; genau das nennt ihr Kommentar heute als Grund für die ausgeschriebene Form. Der Ersatz ist ein Verweis auf `krk-core` statt einer Aufzählung. Und es sind drei Proben anzufassen statt keiner — mehr, als ein Nachzug wie E2b tragen soll.

## Constraints

- `krk-ui` hat kein Bibliotheksziel (`crates/krk-ui/Cargo.toml` führt allein `[[bin]] name = "krk"`). Eine Datei unter `crates/krk-ui/tests/` ist eine eigene Kiste und erreicht nichts aus `krk-ui`; die Probe der Markdown-Ausgabe steht deshalb in einem `#[cfg(test)]`-Modul neben dem Code und bleibt dort, wie immer die Frage fällt.
- `crates/krk-core/tests/` ist ebenfalls eine eigene Kiste. Nichts aus ihr ist von außen erreichbar — auch nicht über `crates/krk-core/tests/gemeinsam/mod.rs`, das nur den Proben des Kerns dient.
- Eine gemeinsame Datendatei als dritte Stelle scheidet aus: die Aussage stünde dann dreimal, denn `resources/default-keymap.toml` trägt sie mit `tasten = []` bereits selbst.
- Der Umbau nach Möglichkeit 3 fasst Prüfcode an, keinen Auslieferungscode. `make check` deckt ihn vollständig ab.

## Recommendation

Möglichkeit 3, aber nicht als Teil dieses Nachzugs. Sie ist die einzige, die eine Stelle herstellt, ohne dafür Auslieferungscode zu bezahlen oder eine Zusage aufzugeben — vorausgesetzt, die Gegenrichtung wandert mit, sonst ist sie ein verdeckter Rückschritt. Möglichkeit 2 kauft die Einheit mit ausgeliefertem Prüfwissen und ist damit teurer als das Problem. Möglichkeit 1 ist der jetzige Stand und trägt: der Baum ist grün, beide Kommentare zeigen aufeinander, und die nächste Gelegenheit für den Umbau ist die nächste Funktion ohne Kombination.

---
Answered:
Implemented:
Deferred:
Superseded by:
