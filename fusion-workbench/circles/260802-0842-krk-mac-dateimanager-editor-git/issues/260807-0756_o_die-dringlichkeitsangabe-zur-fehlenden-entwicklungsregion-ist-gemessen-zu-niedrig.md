Die Dringlichkeitsangabe zur fehlenden Entwicklungsregion ist gemessen zu niedrig

---

`issues/260807-0745_*_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md` trägt
den Satz:

> **Dringlichkeit.** Gering. Kein Nutzer sieht es, kein Abnahmekriterium ist
> berührt, keine der zehn Zeitzusagen aus C8 betroffen.

Der erste Halbsatz ist gemessen falsch. Ein Nutzer, dessen Sprachwahl weder Deutsch
noch Englisch führt, sieht den fehlenden Schlüssel unmittelbar: er bekommt englische
Byte-Angaben, wo er mit dem Schlüssel deutsche bekäme. Das ist genau die Ausgabe, um
derentwillen `CFBundleLocalizations` überhaupt eingeführt wurde.

---

**Gemessen am 260807-0753** an zwei gebauten Bündeln im Scratchpad. Beide führen
`CFBundleLocalizations = de, en` wie `resources/Info.plist` heute; sie unterscheiden
sich allein darin, ob `CFBundleDevelopmentRegion = de` daneben steht. Die Sprachwahl
des Nutzers ist je Lauf über `defaults write <bundle-id> AppleLanguages` gesetzt,
gelesen werden `preferredLocalizations` und derselbe `NSByteCountFormatter` mit
`CountStyle::File`, den `crates/krk-ui/src/appkit/tabelle.rs:426` anlegt.

| Sprachwahl des Nutzers | ohne Entwicklungsregion (KRK heute) | mit `CFBundleDevelopmentRegion = de` |
|---|---|---|
| `de-DE` | `de` → `0 KB`, `1 Byte`, `512 Byte` | `de` → `0 KB`, `1 Byte`, `512 Byte` |
| `en-US, de-DE` | `en` → `Zero KB`, `1 byte`, `512 bytes` | `en` → `Zero KB`, `1 byte`, `512 bytes` |
| **`fr-FR`** | **`en` → `Zero KB`, `1 byte`, `512 bytes`** | **`de` → `0 KB`, `1 Byte`, `512 Byte`** |

Die ersten beiden Zeilen sind gleich, und darauf beruht die Einschätzung "folgenlos"
im Datensatz — sie stimmt für den Fall, an dem sie geprüft wurde. Die dritte Zeile ist
der Fall, den niemand geprüft hat, und dort entscheidet allein
`CFBundleDevelopmentRegion` zwischen der deutschen und der englischen Beschriftung.

**Der Datensatz beschreibt den Mechanismus richtig** — "Sie ist der Rückfall, wenn ein
System keine der angebotenen Sprachen spricht" trifft genau zu. Falsch ist der Schluss
daraus, dieser Rückfall bleibe unsichtbar. Sichtbar ist er in der Größenspalte des
Dateifensters (C1), in den Metadatenzeilen der Vorschau (C6) und im fünften Rang der
Statuszeile (C1), also an denselben drei Stellen, die der Größenformatierer-Defekt
aufzählt.

**Der zweite Rückfall greift nicht.** Naheliegend wäre, die Stellung von `de` an erster
Stelle in `CFBundleLocalizations` als Ersatz zu lesen. Sie ist es nicht: mit der Liste
`de, en` und der Sprachwahl `fr-FR` liefert Foundation gemessen `en`, nicht `de`. Sobald
`en` in der Bündelliste steht, gewinnt `en` den Rückfall unabhängig von seiner Stellung.
Der Kommentar in `resources/Info.plist:38-40` behauptet das Gegenteil; dazu steht ein
eigener Defekt.

---

**Was zu tun ist.** Zwei Zeilen im vorhandenen Datensatz berichtigen, nicht ein neuer
Handgriff an der `Info.plist`:

1. Den Abschnitt "Folgenlos für das, wofür der Schlüssel eingeführt wurde" um den
   dritten Fall ergänzen: folgenlos ist er für einen Nutzer, dessen Sprachwahl `de`
   oder `en` führt, und nur für den.
2. Die Dringlichkeit von "Gering" heraufsetzen und den Halbsatz "Kein Nutzer sieht es"
   streichen.

**Die Zuordnung zu S23 bleibt richtig.** Der Datensatz weist die Frage dem Schritt zu,
der die Bündelbeschreibung als ganze abnimmt; daran ändert die Messung nichts. Sie
ändert, mit welchem Gewicht die Frage dort ankommt.

**Ausführender:** `ontocoder`. Der zu ändernde Datensatz ist ein Workbench-Artefakt.

**Dringlichkeit.** Mittel. Eine zu niedrig angesetzte Dringlichkeit ist der Grund, aus
dem eine Frage bei der nächsten Rundenplanung durchfällt.

**Aufgefallen bei:** der ontologischen Durchsicht der Datenänderungen nach Turn 25 der
Sitzung 260806-2257 (Commit `880cb70`, Aufgabe D8).

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0745_o_die-buendelbeschreibung-fuehrt-keine-entwicklungsregion.md`
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0754_o_der-kommentar-an-cfbundlelocalizations-nennt-eine-falsche-auswahlregel.md`
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1215_c_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
