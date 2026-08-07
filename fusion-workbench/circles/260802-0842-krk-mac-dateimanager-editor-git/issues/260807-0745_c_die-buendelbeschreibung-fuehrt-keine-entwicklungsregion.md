Die Bündelbeschreibung führt keine Entwicklungsregion

---

`resources/Info.plist` führt seit dem 260807 den Schlüssel
`CFBundleLocalizations` mit `de` vor `en`, nicht aber `CFBundleDevelopmentRegion`.
`Bundle.main.developmentLocalization` liefert deshalb `nil`.

---

**Folgenlos für den Nutzer, dessen Sprachwahl `de` oder `en` führt, und nur für
ihn.** Die Byte-Angaben sind für ihn seit dem 260807 so, wie die Sprachwahl es
vorgibt, gemessen am gebauten Bündel; daran ändert die fehlende
Entwicklungsregion nichts. Führt die Sprachwahl **keine** der beiden Sprachen,
entscheidet allein dieser Schlüssel: ohne ihn gewinnt `en` den Rückfall, und der
Nutzer sieht wieder `Zero KB`, `1 byte`, `512 bytes`. Der dritte Fall ist
nachgetragen am 260807-0952, gemessen im Defekt
`issues/260807-0756_*_die-dringlichkeitsangabe-zur-fehlenden-entwicklungsregion-ist-gemessen-zu-niedrig.md`.

**Wofür sie trotzdem steht.** `CFBundleDevelopmentRegion` sagt, in welcher
Sprache die Zeichenketten des Programms ursprünglich geschrieben sind. Sie ist
der Rückfall, wenn ein System keine der angebotenen Sprachen spricht, und sie
steht in den Angaben, die das System und der Finder über ein Bündel führen.
Für ein Programm, dessen Prosa durchgängig deutsch ist, wäre `de` die richtige
Angabe.

**Die Verweisung auf S23 trug schon beim Anlegen nicht, berichtigt am
260807-0952.** Der Absatz sagte, S23 baue das Auslieferungspaket und nehme die
Bündelbeschreibung als ganze ab, dort gehöre die Frage geprüft. Beides ist
falsch. Erstens trägt S23 im Plan seit dem Commit `d577295` vom 260806-0821 den
Vermerk `[DONE]`, also einen Tag vor dem Anlegen dieses Datensatzes; eine Frage
dorthin zu verweisen heißt, sie nirgendwohin zu verweisen. Zweitens nennt die
Dateiliste von S23 `resources/Info.plist` überhaupt nicht, und sein
Abnahmekriterium prüft `lipo`, die gehärtete Laufzeitumgebung und die
AppKit-Grenze, keinen einzigen Schlüssel der Bündelbeschreibung. Die Schritte,
die diese Datei führen, sind S4 und S4b, beide `ontocoder`-Schritte und beide
abgenommen. Es gab damit keinen späteren Halt, an dem die Frage aufgelaufen
wäre.

**Ausführender:** `ontocoder`. `resources/Info.plist` ist eine
Bündelbeschreibung, keine Programmdatei.

**Dringlichkeit.** Mittel, heraufgesetzt am 260807-0952. Ein Nutzer, dessen
Sprachwahl weder Deutsch noch Englisch führt, sieht den fehlenden Schlüssel
unmittelbar, und zwar an denselben drei Stellen, die der
Größenformatierer-Defekt aufzählt: in der Größenspalte des Dateifensters (C1),
in den Metadatenzeilen der Vorschau (C6) und im fünften Rang der Statuszeile
(C1). Kein Abnahmekriterium ist berührt, keine der zehn Zeitzusagen aus C8
betroffen.

**Aufgefallen bei:** der Umsetzung von D8, Turn 25 der Sitzung 260806-2257,
`history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1215_c_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`

---
Resolved: `resources/Info.plist:82-83` führt jetzt `CFBundleDevelopmentRegion` mit dem
Wert `de`, mit einem Kommentar daneben (`:60-81`), der die Wirkung, die Messung und
diesen Datensatz nennt. **Gesetzt und nicht an S23 verwiesen**, aus vier Gründen: S23
trägt seit dem Commit `d577295` vom 260806-0821 den Vermerk `[DONE]`, also einen Tag
vor dem Anlegen dieses Datensatzes; die Dateiliste von S23
(`planning/260802-1428_o_plan-navigator-geruest-runde-1.md:1184`) nennt
`resources/Info.plist` überhaupt nicht und sein Abnahmekriterium (`:1191`) keinen
Schlüssel der Bündelbeschreibung; für einen Nutzer, dessen Sprachwahl weder Deutsch
noch Englisch führt, ist dieser Schlüssel der einzige Hebel; und der Handgriff ist
eine Zeile in einer Datendatei, die der `ontocoder` führt.

Gemessen am 260807-0952 an zwei Bündeln, deren `Info.plist` wörtlich die von
`cargo xtask bundle` erzeugte ist und die sich allein in diesem Schlüssel
unterscheiden, mit demselben `NSByteCountFormatter` und `CountStyle::File`, den
`crates/krk-ui/src/appkit/tabelle.rs:419-420` anlegt, über die sechs Werte 0, 1, 512,
999, 1.000 und 10.240 Bytes:

| Sprachwahl | ohne den Schlüssel | mit `CFBundleDevelopmentRegion = de` |
|---|---|---|
| `de-DE` | `de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`, `1 KB`, `10 KB` | gleich |
| `en-US, de-DE` | `en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`, `1 KB`, `10 KB` | gleich |
| **`fr-FR`** | **`en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`** | **`de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`** |
| **`ja`** | **`en` → `Zero KB`, `1 byte`, `512 bytes`, `999 bytes`** | **`de` → `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`** |

Die beiden oberen Zeilen sind gleich: die Änderung nimmt keinem heutigen Nutzer etwas
weg. Gegengeprüft auf dem zweiten Weg über `defaults write <bundle-id> AppleLanguages`
mit demselben Ergebnis; beide Vorgabendomänen sind danach wieder entfernt. Am
signierten Bündel meldet `[NSBundle bundleWithPath:]` `developmentLocalization = de`.

`plutil -lint` und `xmllint --noout` gültig, 16 Schlüssel vor und 17 nach der
Änderung, `CFBundleLocalizations` unverändert `['de', 'en']`, `__KRK_VERSION__`
unberührt, `diff resources/Info.plist target/KRK.app/Contents/Info.plist` meldet genau
die Versionszeile, `make bundle` gebaut und signiert, `make check` grün. Am Plan ist
nichts nachzuziehen: S4 und S4b zählen die Schlüssel auf, die sie anlegen, verbieten
aber keinen weiteren.
Bericht: `history/260807-0952-ontocoder-entwicklungsregion-auswahlregel-und-das-letzte-pfadzitat.md`.
