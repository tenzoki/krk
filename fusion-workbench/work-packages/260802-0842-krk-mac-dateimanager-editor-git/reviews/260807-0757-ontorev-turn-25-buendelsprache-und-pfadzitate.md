# Ontologische Durchsicht: Bündelsprache und Pfadzitate (Turn 25, Commit `880cb70`)

**Sender:** ontorev
**Umfang:** `resources/Info.plist`, `resources/default-keymap.toml`, `resources/default-settings.toml` aus dem Diff `f9a0462..HEAD`
**Programmtext:** nicht Gegenstand, geprüft parallel vom `coderev`
**Grundlage:** `history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`

---

## Zusammenfassung

Die Datenänderung trägt. Beide `.toml`-Dateien sind gültig und außerhalb der
Kommentare byteweise unverändert, alle 13 umgestellten Zitate lösen auf eine
vorhandene Datei auf, die `Info.plist` ist wohlgeformt und der Versionsplatzhalter
unberührt. Drei Befunde betreffen nicht die Änderung selbst, sondern die Aussagen,
die neben ihr stehen: der Kommentar, der die neue Sprachliste begründet, nennt eine
gemessen falsche Auswahlregel und einen gemessen falschen Rückfall, ein Zitat in
derselben Datei ist bei der Umstellung übrig geblieben, und die Dringlichkeitsangabe
des Folgedefekts zur fehlenden Entwicklungsregion ist um einen ganzen Nutzerkreis zu
niedrig angesetzt.

## Zahlen

| Gewicht | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 3 |
| Gering | 0 |

## Was geprüft ist und trägt

**Die `Info.plist` ist wohlgeformt und vollständig.** `plutil -lint` und
`xmllint --noout` beide gültig, nachgeprüft statt geglaubt. Zusätzlich über
`plutil -convert json` in eine zweite Darstellung überführt: 15 Schlüssel, alle mit
dem erwarteten Typ, `CFBundleLocalizations` als Feld `["de", "en"]`.

**Der Versionsplatzhalter ist unberührt.** `resources/Info.plist:68` trägt
`__KRK_VERSION__`, und das ist wörtlich die Konstante `PLATZHALTER` aus
`xtask/src/bundle.rs:34`. `CFBundleVersion` steht unverändert auf `1`.

**Die Schreibweise der beiden Sprachkennungen ist die, die Foundation erwartet.**
Gemessen am gebauten Bündel: `de` greift bei einer Sprachwahl `de-DE`, `en` bei
`en-US`. Foundation schneidet die Region weg und vergleicht auf der Sprachebene.

**Alle 13 umgestellten Zitate tragen auf eine wirkliche Datei.** Jedes Zitat wurde
über `find` mit dem Marker als Einzelzeichen-Platzhalter gegen den Dateibestand
aufgelöst: 13 Zitate, 13 Treffer, kein Zitat mit zwei Zielen, kein Zitat ohne Ziel.
Das vierzehnte, neu in der `Info.plist` entstandene Zitat löst ebenfalls auf. Auch die
Verzeichnisteile stimmen: `shared/decisions/` steht bei den beiden projektweiten
Datensätzen, `decisions/` und `issues/` bei den Datensätzen des Circles, entsprechend
der Lesart, die `CLAUDE.md` für Pfade dieser Form festlegt.

**Die Sternform ist einheitlich, mit genau einer Ausnahme.** Gezählt: 80 Zitate in
Sternform unter `crates/` und `xtask/`, 15 in `.toml`-Dateien, kein einziges mit
ausgeschriebenem Marker. Die eine Ausnahme ist Befund 2.

**Beide `.toml`-Dateien sind gültig und semantisch unverändert.** Der Vergleich des
Standes vor und nach der Änderung, jeweils ohne Kommentarzeilen, meldet für beide
Dateien Gleichheit. Keine Tastenbelegung, keine Kennung und kein Wert ist mitgewandert.
Kommentare am Zeilenende einer Wertzeile, die ein solcher Vergleich übersehen könnte,
gibt es in keiner der beiden Dateien.

**Die Aussage zum Abnahmekriterium von S20 hält der Nachprüfung stand.** Der `ontocoder`
berichtet, der Vergleich laufe über `toml::to_string` und kenne deshalb keine Kommentare.
Nachgeprüft an beiden Enden des Weges: das Zurücksetzen geht über `Belegung::auslieferung()`
und damit über `toml::from_str(AUSLIEFERUNGSTEXT)` (`crates/krk-core/src/tasten/belegung.rs:113`),
das Sichern über `toml::to_string` (`crates/krk-core/src/ablage/mod.rs:269`). Beide
Richtungen lassen Kommentare fallen. Eine Änderung an einer Kommentarzeile kann diesen
Vergleich nicht berühren; am Plan ist nichts nachzuziehen.

**Die zweite Auslieferungsdatei geht mit ihren Kommentaren an den Nutzer.**
`resources/default-settings.toml` wird beim ersten Start wörtlich nach
`~/Library/Application Support/KRK/settings.toml` geschrieben, samt Kommentaren; die
zwei geänderten Zitate stehen künftig also in der Nutzerdatei. Der Test
`eine_fehlende_settings_toml_liefert_die_vorbelegung_und_entsteht_mit_kommentaren`
(`crates/krk-core/tests/ablage.rs:591-628`) prüft die `mdls`-Zeile und eine
Kommentarzeilenzahl über 20; beides ist unberührt. Ein Befund ist das nicht: die
Zitate standen dort schon vorher, und ein Workbench-Pfad ist auf dem Gerät des Nutzers
weder mit noch ohne Marker auflösbar.

## Befunde

### 1. Der Kommentar an `CFBundleLocalizations` nennt eine falsche Auswahlregel — Mittel

`resources/Info.plist:38-40`. Der Kommentar begründet die Reihenfolge `de` vor `en` mit
zwei Sätzen über das Verhalten von Foundation, und beide sagen das Falsche voraus.

Gemessen am 260807-0752 an zwei gebauten Bündeln im Scratchpad sowie über
`+[NSBundle preferredLocalizationsFromArray:forPreferences:]`:

| Bündelliste | Sprachwahl des Nutzers | Kommentar sagt | gemessen |
|---|---|---|---|
| `de, en` | `en-US, de-DE` | `de` | **`en`** |
| `en, de` | `de-DE` | `en` | **`de`** |
| `de, en` | `fr-FR` | `de` | **`en`** |

Es entscheidet die Reihenfolge der **Nutzerliste**, nicht die der Bündelliste; die
Bündelliste sagt allein, welche Sprachen zur Wahl stehen. Und der Rückfall "erste der
Liste" gilt nur für eine Bündelliste ohne `en`: gegengeprüft liefert `de, fr` bei
Sprachwahl `ja` tatsächlich `de` und `fr, de` liefert `fr`, aber sobald `en` in der
Liste steht, gewinnt `en` unabhängig von seiner Stellung.

Die Reihenfolge `de` vor `en` ist damit unschädlich, aber wirkungslos. Falsch ist nur
der Grund, der neben ihr steht — und er ist die einzige Stelle im Projekt, die die
Wirkung des Schlüssels erklärt.

Die Änderung selbst ist davon nicht berührt: `CFBundleLocalizations` löst den
Größenformatierer-Defekt vollständig, wie gemessen.

Defekt: `issues/260807-0754_o_der-kommentar-an-cfbundlelocalizations-nennt-eine-falsche-auswahlregel.md`

### 2. Ein Zitat in der `Info.plist` schreibt den Zustandsmarker noch aus — Mittel

`resources/Info.plist:116` zitiert
`issues/260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
mit ausgeschriebenem `_c_`. Es ist das letzte Zitat dieser Form im gesamten Programm-
und Auslieferungsbestand und steht zwölf Zeilen unter einem Zitat in der neuen Form
(`resources/Info.plist:28`).

Das Ziel steht heute richtig, die Referenz ist also nicht tot. Sie wird beim nächsten
Markerwechsel des Ziels stillschweigend falsch — der Fall, gegen den die Sternform
eingeführt wurde, und der bei zehn der 13 umgestellten Zitate bereits eingetreten war.

Durchgerutscht ist es, weil kein Auftrag die `Info.plist` je auf ihre Pfadzitate
durchgesehen hat: der Zitat-Defekt nennt in Titel und Umfang allein die beiden
`resources/default-*.toml`, und die `Info.plist` kam über den anderen Defekt derselben
Aufgabe in den Umfang.

Defekt: `issues/260807-0755_o_ein-zitat-in-der-info-plist-schreibt-den-zustandsmarker-noch-aus.md`

### 3. Die Dringlichkeitsangabe zur fehlenden Entwicklungsregion ist zu niedrig — Mittel

`issues/260807-0745_*_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md` trägt
"Dringlichkeit. Gering. Kein Nutzer sieht es." Gemessen ist der zweite Satz falsch.

Zwei Bündel, beide mit `CFBundleLocalizations = de, en`, unterschieden allein durch
`CFBundleDevelopmentRegion = de`:

| Sprachwahl des Nutzers | ohne Entwicklungsregion (KRK heute) | mit `CFBundleDevelopmentRegion = de` |
|---|---|---|
| `de-DE` | `0 KB`, `1 Byte`, `512 Byte` | gleich |
| `en-US, de-DE` | `Zero KB`, `1 byte`, `512 bytes` | gleich |
| **`fr-FR`** | **`Zero KB`, `1 byte`, `512 bytes`** | **`0 KB`, `1 Byte`, `512 Byte`** |

Für jeden Nutzer, dessen Sprachwahl weder Deutsch noch Englisch führt, entscheidet
allein dieser Schlüssel zwischen deutscher und englischer Beschriftung — an denselben
drei Stellen, die der Größenformatierer-Defekt aufzählt. Der Datensatz beschreibt den
Mechanismus richtig und zieht den falschen Schluss daraus.

Die Zuordnung zu S23 bleibt richtig; sie kommt dort nur mit anderem Gewicht an.

Defekt: `issues/260807-0756_o_die-dringlichkeitsangabe-zur-fehlenden-entwicklungsregion-ist-gemessen-zu-niedrig.md`

## Reihenfolge der Bearbeitung

Befund 2 zuerst, weil er ein Handgriff an einer Zeile ist und die Regel danach
ausnahmslos gilt. Dann Befund 1, weil er dieselbe Datei betrifft und im selben Zug
mitläuft. Befund 3 zuletzt: er ändert keinen Programm- und keinen Auslieferungsbestand,
sondern zwei Zeilen in einem Workbench-Datensatz, und er wirkt erst bei der nächsten
Rundenplanung.

Alle drei sind `ontocoder`-Arbeit. Keiner berührt eine der zehn Zeitzusagen aus C8,
keiner ein Abnahmekriterium, keiner den Programmtext.

## Messgrundlage

Die Messungen zu Befund 1 und 3 sind Foundation-Programme im Scratchpad dieser Sitzung,
gebaut mit `clang -framework Foundation`, ausgeführt als Bündel mit eigener
`Info.plist`; die Sprachwahl je Lauf über `defaults write <bundle-id> AppleLanguages`.
In den Programmtext von KRK ist dafür nichts eingebaut worden, und im Projekt ist keine
Datei angefasst worden außer den drei Defektdateien unter `issues/` und diesem Bericht.
