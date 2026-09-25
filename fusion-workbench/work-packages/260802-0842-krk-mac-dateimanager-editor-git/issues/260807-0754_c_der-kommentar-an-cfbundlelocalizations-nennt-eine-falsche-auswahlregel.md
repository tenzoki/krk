Der Kommentar an `CFBundleLocalizations` nennt eine falsche Auswahlregel und einen falschen Rückfall

---

`resources/Info.plist:38-40` begründet die Reihenfolge `de` vor `en` mit zwei Sätzen
über das Verhalten von Foundation. Beide sind gemessen falsch. Die Reihenfolge selbst
ist unschädlich, aber der Grund, der neben ihr steht, sagt dem nächsten Leser etwas
Unzutreffendes über die Wirkung des Schlüssels.

Zitiert wird `resources/Info.plist:38-40`:

> `de` steht vorn, weil `CLAUDE.md` Deutsch als Projektsprache fuehrt: aus
> dieser Liste gewinnt die erste Sprache, die in der Sprachliste des Nutzers
> vorkommt, und trifft keine zu, greift die erste dieser Liste.

---

**Gemessen am 260807-0752** an zwei gebauten Bündeln im Scratchpad, deren `Info.plist`
`CFBundleLocalizations = de, en` trägt, sowie über
`+[NSBundle preferredLocalizationsFromArray:forPreferences:]`. Die Sprachwahl des
Nutzers ist je Lauf über `defaults write <bundle-id> AppleLanguages` gesetzt.

**Behauptung 1: "aus dieser Liste gewinnt die erste Sprache, die in der Sprachliste
des Nutzers vorkommt."** Nach dieser Regel gewänne bei `AppleLanguages = ("en-US",
"de-DE")` die Sprache `de`, weil `de` in der Bündelliste vorn steht und in der
Nutzerliste vorkommt.

| Bündelliste | Sprachwahl des Nutzers | vorhergesagt | gemessen |
|---|---|---|---|
| `de, en` | `en-US, de-DE` | `de` | **`en`** |
| `de, en` | `de-DE` | `de` | `de` |
| `en, de` | `de-DE` | `en` | **`de`** |

Entscheidend ist die Reihenfolge der **Nutzerliste**, nicht die der Bündelliste. Die
Bündelliste sagt allein, welche Sprachen überhaupt zur Wahl stehen. Die Reihenfolge
`de` vor `en` ist damit in jedem Fall, in dem der Nutzer eine der beiden Sprachen
führt, ohne Wirkung: die dritte Zeile zeigt, dass auch die umgekehrte Reihenfolge
`de` liefert.

**Behauptung 2: "trifft keine zu, greift die erste dieser Liste."** Nach dieser Regel
bekäme ein Nutzer mit `AppleLanguages = ("fr-FR")` die Sprache `de`.

| Bündelliste | Sprachwahl des Nutzers | vorhergesagt | gemessen |
|---|---|---|---|
| `de, en` | `fr-FR` | `de` | **`en`** |
| `en, de` | `fr` | `en` | `en` |
| `de, fr` | `ja` | `de` | `de` |
| `fr, de` | `ja` | `fr` | `fr` |

Die Regel "erste der Liste" gilt nur für eine Bündelliste **ohne** `en`. Sobald `en`
in der Liste steht, gewinnt `en` den Rückfall, gleich an welcher Stelle es steht. Für
die Liste, die KRK führt, sagt die Behauptung deshalb das Falsche voraus.

Der Rückfall, der hier wirklich greift, ist `CFBundleDevelopmentRegion` — der Schlüssel,
den `resources/Info.plist` nicht führt. Was das kostet, steht in
`issues/260807-0745_*_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md` und
in dem Defekt, der dessen Dringlichkeitsangabe berichtigt.

---

**Was zu tun ist.** Die beiden Sätze in `resources/Info.plist:38-40` durch die
gemessene Regel ersetzen: die Bündelliste nennt die Sprachen, die zur Wahl stehen;
gewählt wird nach der Reihenfolge der Sprachwahl des Nutzers; passt keine, entscheidet
`CFBundleDevelopmentRegion` und, solange der Schlüssel fehlt, Englisch. Die Reihenfolge
`de` vor `en` kann bleiben — sie schadet nicht, sie trägt nur nicht, was der Kommentar
ihr zuschreibt.

**Nicht betroffen ist die Änderung selbst.** Der Schlüssel `CFBundleLocalizations`
löst den Größenformatierer-Defekt vollständig, wie
`issues/260806-1215_*_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
gemessen festhält und wie die Durchsicht bestätigt. Berührt ist allein der Kommentar.

**Ausführender:** `ontocoder`. `resources/Info.plist` ist eine Bündelbeschreibung,
keine Programmdatei.

**Dringlichkeit.** Mittel. Kein Nutzer sieht es und kein Abnahmekriterium ist berührt,
aber der Kommentar ist die einzige Stelle im Projekt, die die Wirkung des Schlüssels
erklärt, und wer nach ihr handelt, zieht falsche Schlüsse — etwa den, die Reihenfolge
umstellen zu müssen, um die Sprache zu wechseln.

**Aufgefallen bei:** der ontologischen Durchsicht der Datenänderungen nach Turn 25 der
Sitzung 260806-2257 (Commit `880cb70`, Aufgabe D8).

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0745_c_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md`
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1215_c_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`

---
Resolved: `resources/Info.plist:30-45` ersetzt die beiden widerlegten Sätze durch die
gemessene Regel. Der Kommentar sagt jetzt, dass die Reihenfolge dieser Liste nicht
auswählt: Foundation geht die Sprachliste des Nutzers der Reihe nach durch und nimmt
die erste Sprache, die die Bündelliste anbietet; führt die Sprachwahl keine davon,
entscheidet `CFBundleDevelopmentRegion`; ein Rückfall auf die erste Sprache der
Bündelliste greift nur, solange `en` nicht darin steht. Als Beleg stehen drei
gemessene Fälle daneben, und dieser Datensatz ist genannt.
**Die Reihenfolge `de` vor `en` steht unverändert**, mit dem ausdrücklichen Zusatz,
dass sie eine Nutzerentscheidung vom 260807 ist, unschädlich, und nur nichts
auswählt. Die Messung ist am selben Tag nachgefahren, an zwei Bündeln, deren
`Info.plist` wörtlich die von `cargo xtask bundle` erzeugte ist, mit vier
Sprachwahlen: `de, en` mit `en-US, de-DE` liefert `en`, `de, en` mit `fr-FR` und mit
`ja` liefert ohne Entwicklungsregion `en` und mit ihr `de`.
`plutil -lint` und `xmllint --noout` gültig, `__KRK_VERSION__` unberührt,
`make check` grün.
Bericht: `history/260807-0952-ontocoder-entwicklungsregion-auswahlregel-und-das-letzte-pfadzitat.md`.
