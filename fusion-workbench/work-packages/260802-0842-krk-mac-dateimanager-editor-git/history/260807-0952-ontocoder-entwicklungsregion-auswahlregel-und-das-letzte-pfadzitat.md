# Die Entwicklungsregion, die Auswahlregel und das letzte Pfadzitat (Aufräumaufgabe aus Turn 26)

**Agent:** ontocoder
**Status:** Complete
**Quellen:**
- `issues/260807-0755_*_ein-zitat-in-der-info-plist-schreibt-den-zustandsmarker-noch-aus.md`
- `issues/260807-0754_*_der-kommentar-an-cfbundlelocalizations-nennt-eine-falsche-auswahlregel.md`
- `issues/260807-0745_*_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md`
- `issues/260807-0756_*_die-dringlichkeitsangabe-zur-fehlenden-entwicklungsregion-ist-gemessen-zu-niedrig.md`
- `reviews/260807-0757-ontorev-turn-25-buendelsprache-und-pfadzitate.md`

**Zum Stilprofil:** `fusion-rules ontocoder` gab allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`. Für diesen Bericht gilt deshalb kein Langform-Schreibprofil; das Fehlen ist hier vermerkt, wie `rules/agent-setup.md` es verlangt.

**Angefasst sind zwei Dateien:** `resources/Info.plist` und die vier Defektdateien. Kein Programmtext, kein Spec, kein Plan.

---

## Teil 1: das letzte ausgeschriebene Pfadzitat

`resources/Info.plist:116` zitierte den Defekt zu den beiden unwirksamen Menü-Schlüsseln mit ausgeschriebenem `_c_`. Die Zeile trägt jetzt die Sternform:

```
`issues/260805-0753_*_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
```

**Gegen den Dateibestand aufgelöst.** Alle vier Zitate der Datei, die beiden alten und die beiden neu entstandenen, wurden über `find fusion-workbench -path '*/<zitat mit _?_ statt _*_>'` gegen den Bestand geprüft: vier Zitate, je genau ein Treffer, keines mit zwei Zielen und keines ohne. Die Zeile steht in einem XML-Kommentar; kein Wertfeld ist berührt.

Damit gilt die Sternform im Programm- und Auslieferungsbestand ausnahmslos: 80 Zitate in `.rs`, 15 in `.toml`, 4 in `resources/Info.plist`, kein einziges mit ausgeschriebenem Marker.

## Teil 2: die Auswahlregel neben `CFBundleLocalizations`

Der Kommentar bei `resources/Info.plist:38-40` begründete die Reihenfolge `de` vor `en` mit zwei Sätzen, die der `ontorev` an gebauten Bündeln widerlegt hat. Beide sind ersetzt. **Die Reihenfolge selbst bleibt** — sie ist eine Nutzerentscheidung vom 260807 —, und der neue Text sagt ausdrücklich, dass sie unschädlich ist und nur nichts auswählt.

Der neue Kommentar sagt drei Dinge und belegt sie mit der Messung:

1. Foundation geht die **Sprachliste des Nutzers** der Reihe nach durch und nimmt die erste Sprache, die die Bündelliste anbietet. Die Bündelliste sagt allein, welche Sprachen zur Wahl stehen.
2. Führt die Sprachwahl keine davon, entscheidet `CFBundleDevelopmentRegion`.
3. Ein Rückfall auf die erste Sprache der Bündelliste greift nur, solange `en` nicht darin steht.

## Teil 3: die Entwicklungsregion — gesetzt, nicht an S23 verwiesen

`resources/Info.plist` führt jetzt `CFBundleDevelopmentRegion` mit dem Wert `de`, mit einem Kommentar daneben, der die Wirkung und die Messung nennt.

**Der Datensatz `260807-0745` verwies die Frage an S23. Diese Verweisung trägt nicht, und sie trug schon beim Anlegen nicht.**

- **S23 ist abgenommen.** Der Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md:1181` führt `#### 23. [DONE] **Auslieferungspaket**`, und `xtask/src/release.rs` steht seit dem Commit `d577295` vom 260806-0821, also einen Tag vor dem Anlegen des Datensatzes am 260807-0745. Eine Frage an einen abgenommenen Schritt zu verweisen heißt, sie nirgendwohin zu verweisen.
- **S23 nimmt die Bündelbeschreibung nicht ab.** Seine Dateiliste (`:1184`) nennt `xtask/src/release.rs`, `xtask/src/main.rs`, `README.md` und lesend `bundle.rs` und `sign.rs` — `resources/Info.plist` steht nicht darin. Sein Abnahmekriterium (`:1191`) prüft `lipo -archs`, die gehärtete Laufzeitumgebung und die AppKit-Grenze, keinen einzigen Schlüssel der Beschreibung. Die Schritte, die diese Datei führen, sind S4 (`:625`) und S4b (`:633`), beide `ontocoder`-Schritte und beide abgenommen.
- **Der Hebel ist der einzige, den dieser Nutzerkreis hat.** Für eine Sprachwahl ohne Deutsch und ohne Englisch entscheidet allein dieser Schlüssel zwischen deutscher und englischer Beschriftung, an denselben drei Stellen, die der Größenformatierer-Defekt aufzählt.
- **Der Handgriff ist eine Zeile in einer Datendatei**, die dieser Agent führt. Kein Programmtext, kein Abnahmekriterium, keine der zehn Zeitzusagen aus C8.

Der Gegengrund, den der Datensatz nennt — "nicht in einen eigenen Handgriff nebenbei" — wiegt gegen einen Nutzerkreis, der die falsche Sprache sieht, nicht auf, sobald der Halt, auf den er verweist, nicht mehr existiert.

## Die Messung

**Aufbau.** Ein Foundation-Programm im Scratchpad (`clang -framework Foundation`) liest `developmentLocalization` und `preferredLocalizations` seines eigenen Hauptbündels und legt denselben `NSByteCountFormatter` mit `CountStyle::File` an, den `crates/krk-ui/src/appkit/tabelle.rs:419-420` anlegt. Es läuft in zwei Bündeln, deren `Info.plist` **wörtlich die von `cargo xtask bundle` erzeugte** ist (`target/KRK.app/Contents/Info.plist`, 17 Schlüssel, Version eingesetzt); die beiden unterscheiden sich allein darin, ob `CFBundleDevelopmentRegion` darin steht, und tragen eine eigene Bündelkennung, damit die Vorgaben des echten KRK unberührt bleiben. Die Sprachwahl ist je Lauf über die Argumentebene `-AppleLanguages` gesetzt.

**Ergebnis.** Sechs Werte, vier Sprachwahlen, zwei Bündel:

| Sprachwahl | ohne `CFBundleDevelopmentRegion` | mit `CFBundleDevelopmentRegion = de` |
|---|---|---|
| `de-DE` | `de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`, `1 KB`, `10 KB` | gleich |
| `en-US, de-DE` | `en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`, `1 KB`, `10 KB` | gleich |
| **`fr-FR`** | **`en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`, `1 KB`, `10 KB`** | **`de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`, `1 KB`, `10 KB`** |
| **`ja`** | **`en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`, `1 KB`, `10 KB`** | **`de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`, `1 KB`, `10 KB`** |

Die beiden oberen Zeilen sind in beiden Bündeln gleich: **die Änderung nimmt keinem heutigen Nutzer etwas weg.** Die beiden unteren sind der Gewinn. Die Werte 1.000 und 10.240 lauten in beiden Sprachen `1 KB` und `10 KB` und unterscheiden nicht; sie sind mitgemessen, weil die Aufgabe sie nennt.

**Gegengeprüft auf dem zweiten Weg.** Derselbe Fall `fr-FR` noch einmal über `defaults write <bundle-id> AppleLanguages -array fr-FR` statt über die Argumentebene, also über den Weg, den der `ontorev` gefahren ist: dasselbe Ergebnis, `en` ohne und `de` mit dem Schlüssel. Beide Vorgabendomänen sind danach über `defaults delete` wieder entfernt, nachgeprüft mit `defaults read` (meldet "does not exist").

**Die Auswahlregel aus Teil 2 fällt aus derselben Messreihe.** Zeile 2 zeigt, dass die Reihenfolge der Bündelliste nicht auswählt: `de` steht dort vorn und `en` gewinnt, weil `en-US` in der Nutzerliste vorn steht. Zeile 3 und 4 zeigen, dass ohne den neuen Schlüssel nicht die erste Sprache der Bündelliste den Rückfall gewinnt, sondern `en`.

**Am wirklichen Bündel nachgesehen.** `[NSBundle bundleWithPath:@".../target/KRK.app"]` meldet `developmentLocalization = de`; der Schlüssel steht also so im signierten Bündel, wie er in der Quelldatei steht. `preferredLocalizations` ist an einem **Nebenbündel** nicht aussagekräftig: der Aufruf lieferte dort in allen vier Sprachwahlen `en`, auch bei `de-DE`, weil er die Sprachen des laufenden Prozesses und nicht die Argumentebene heranzieht. Die Tabelle oben ist deshalb am Hauptbündel gemessen und nicht hier.

## Prüfungen

| Prüfung | Ergebnis |
|---|---|
| `plutil -lint resources/Info.plist` | `OK` |
| `xmllint --noout resources/Info.plist` | gültig, keine Ausgabe |
| `plutil -extract CFBundleShortVersionString raw` | `__KRK_VERSION__`, unberührt |
| `plutil -extract CFBundleDevelopmentRegion raw` | `de` |
| Schlüsselzahl | 16 vor der Änderung, 17 danach; genau einer kommt dazu |
| `CFBundleLocalizations` | `['de', 'en']`, unverändert |
| Pfadzitate | 4 Zitate, alle in Sternform, jedes löst auf genau eine Datei auf |
| `make bundle` | gebaut und signiert mit der Entwicklungsidentität aus dem Schlüsselbund |
| `diff resources/Info.plist target/KRK.app/Contents/Info.plist` | genau eine Zeile, `__KRK_VERSION__` gegen `0.1.0` |
| `cargo test -p xtask` | `die_ausgelieferte_plist_traegt_den_platzhalter` und `die_ausgelieferte_plist_nennt_das_binaerprogramm` beide grün |
| `make check` | grün, alle vier |

**Zum Doppelstrich in XML-Kommentaren.** Die Datei bleibt nach `xmllint` gültig; die beiden neuen Kommentarblöcke führen keinen Doppelstrich, wie der Kommentar bei `:118-121` es für diese Datei festhält.

**Eine Nebenbeobachtung zu `make check`.** Der erste Lauf um 0949 brach in `xtask/src/release.rs:129` mit `cannot find value GRENZWURZELN` ab. Das ist fremde, gleichzeitig laufende Arbeit am Programmtext und nicht Folge dieser Änderung; der Lauf um 0952 war grün, nachdem der Programmtext wieder übersetzte.

## Abschluss der vier Datensätze

Alle vier tragen eine `Resolved:`-Notiz und heißen jetzt `_c_` statt `_o_`. Zwei
`Cross-references:`-Blöcke zeigten danach noch auf die alten Namen und sind mitgezogen:
`260807-0754` und `260807-0756` verweisen jetzt auf `260807-0745_c_…`, und `260807-0756`
zusätzlich auf `260807-0754_c_…`.

**Der `ontorev`-Bericht `reviews/260807-0757-…` behält seine vier Zitate mit
ausgeschriebenem `_o_`.** Er ist ein datierter Befund und hält den Stand seines Tages
fest, dieselbe Ausnahme, die der Defekt `260807-0755` für `messungen/` benennt.

## Was nicht angefasst ist

Kein Programmtext, kein Spec, kein Plan, kein Messbericht, keine `CLAUDE.md`. Die Reihenfolge `de` vor `en` steht unverändert. Der Versionsplatzhalter `__KRK_VERSION__` steht unverändert.

**Eine Festlegung ist berührt, und sie ist nicht geändert, sondern gemeldet:** der Datensatz `260807-0745` hatte die Frage an S23 verwiesen. Die Verweisung ist im Datensatz selbst berichtigt (Teil 3), nicht im Plan. Am Plan ist nichts nachzuziehen: S4 und S4b zählen die Schlüssel auf, die sie anlegen, verbieten aber keinen weiteren, und S4bs Abnahmekriterium ("die fünf TCC-Texte und die übrigen Schlüssel sind im Diff unverändert") gilt seinem eigenen Diff und nicht jedem späteren.
