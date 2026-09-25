Die beiden `Info.plist`-Schlüssel gegen die Systemeinträge im Menü "Bearbeiten" greifen nicht

---

S13b hat `resources/Info.plist` die Schlüssel `NSDisabledCharacterPaletteMenuItem` und `NSDisabledDictationMenuItem` mit dem Wert `true` gegeben, damit macOS dem Menü "Bearbeiten" nicht von sich aus "Emoji & Symbole" und "Diktat starten" dazustellt. Der Plan hat das als `inference:` geführt und die Prüfung dem Abnahmekriterium von S13c überlassen.

**Sie greifen nicht.** Gemessen am 260805-0753 am signierten Bündel.

Die Schlüssel stehen richtig in der gebauten Beschreibung:

```
$ plutil -extract NSDisabledCharacterPaletteMenuItem raw target/KRK.app/Contents/Info.plist
true
$ plutil -extract NSDisabledDictationMenuItem raw target/KRK.app/Contents/Info.plist
true
```

Das Menü trug sie trotzdem. `--menue-protokoll`, gebaut allein mit diesen Schlüsseln:

```
menue="Bearbeiten" eintrag="Start Dictation…"  kombination=d               ...
menue="Bearbeiten" eintrag="Start Dictation…"  kombination=d               verdeckt=ja ...
menue="Bearbeiten" eintrag="Emoji & Symbols"   kombination=cmd+space       ...
menue="Bearbeiten" eintrag="Emoji & Symbols"   kombination=ctrl+cmd+space  verdeckt=ja ...
menue="Bearbeiten" eintrag="Emoji & Symbols"   kombination=e               verdeckt=ja ...
```

Dieselben zwei Namen als **Nutzervorgabe** wirken dagegen. Derselbe Bau, um zwei Argumente ergänzt:

```
$ ./target/KRK.app/Contents/MacOS/krk --menue-protokoll \
    -NSDisabledCharacterPaletteMenuItem YES -NSDisabledDictationMenuItem YES
```

Danach standen im Menü "Bearbeiten" nur noch die vier eigenen Einträge, ein Trenner und ein Untermenü "AutoFill" ohne Kürzel.

---

## Warum es zählt

Ctrl+Cmd+Leertaste und Cmd+Leertaste lösen etwas aus, stehen in keiner Tastenliste, werden von der Konflikterkennung nicht gesehen und sind nicht umbelegbar. Das Abnahmekriterium von S13c nennt Ctrl+Cmd+Leertaste ausdrücklich als das, was nicht vorkommen darf.

## Was S13c getan hat

AppKit liest die beiden Namen aus `NSUserDefaults` und nicht aus der Bundle-Beschreibung. `crates/krk-ui/src/appkit/menue.rs` trägt deshalb seit S13c die Funktion `systemzusaetze_unterdruecken`, die beide über `registerDefaults:` in die unterste Vorgabenebene stellt, bevor `NSApplication` entsteht. Sie schreibt nichts auf die Platte und lässt sich von einer ausdrücklichen Einstellung des Nutzers überschreiben. Damit ist das Abnahmekriterium erfüllt, nachgemessen am 260805-0753.

## Was zu tun ist

Zu entscheiden, was mit den beiden Schlüsseln in `resources/Info.plist` geschieht. Sie sind seither wirkungslos, und zwei Stellen, die dasselbe zusagen, sind eine Wahrheit zu viel: wer die Datei liest, hält die Unterdrückung für dort geregelt und sucht die Ursache am falschen Ort, wenn sie einmal ausfällt.

Vorgeschlagen: beide Schlüssel und ihren Kommentar aus `resources/Info.plist` entfernen und im Kommentar über den verbleibenden Schlüsseln einen Satz hinterlassen, der auf `systemzusaetze_unterdruecken` verweist. `resources/Info.plist` ist eine Datendatei und gehört dem `ontocoder`; S13c durfte sie nur lesen.

Denkbar wäre auch, sie stehen zu lassen, falls eine spätere macOS-Version sie doch auswertet. Dann gehört an dieselbe Stelle ein Satz, der die Messung von heute nennt, damit niemand sie für den wirksamen Weg hält.

---

Herkunft: gefunden bei der Abnahme von S13c am 260805-0753, beim Auslesen des gebauten Hauptmenüs mit `--menue-protokoll`.

---
Resolved: Beide Schlüssel sind am 260805-0820 aus `resources/Info.plist` entfernt. An ihrer Stelle steht ein Kommentar, der sagt, dass sie dort standen, dass sie nicht greifen, dass AppKit die beiden Namen aus `NSUserDefaults` liest, und dass `systemzusaetze_unterdruecken` in `crates/krk-ui/src/appkit/menue.rs` die Sache trägt. Der Kommentar nennt beide Messungen.

**Die Wahl fiel auf Entfernen und nicht auf Stehenlassen.** Eine Bündelbeschreibung sagt, was gilt. Zwei Schlüssel ohne Wirkung sagen, was jemand einmal versucht hat, und ein Leser, der sie für wirksam hält, sucht die Ursache am falschen Ort, wenn die Unterdrückung einmal ausfällt. Der Preis der Wahl, dass das Wissen um den nicht tragenden Weg nur noch im Defekt und in der Historie stünde, ist mit dem Kommentar bezahlt: er steht an genau der Stelle, an der ein Leser die Schlüssel suchen würde, und ist mit ihnen nicht zu verwechseln. Die Möglichkeit, dass eine spätere macOS-Version die Schlüssel doch auswertet, trägt das Stehenlassen nicht: sie wäre eine zweite Stelle, die dasselbe zusagt, und der Weg über `registerDefaults:` wirkt in diesem Fall ohnehin weiter.

**Eigene Nachprüfung des Befunds, statt ihn zu übernehmen.** Gemessen am 260805-0813 am gebauten Bündel `target/KRK.app` vom 260805-0800, dessen `Info.plist` beide Schlüssel noch trug:

```
$ plutil -extract NSDisabledCharacterPaletteMenuItem raw target/KRK.app/Contents/Info.plist
true
$ plutil -extract NSDisabledDictationMenuItem raw target/KRK.app/Contents/Info.plist
true
```

Die Gegenprobe geht umgekehrt zu der des `coder`. Er hat die beiden Namen als Nutzervorgabe auf YES gesetzt und die Systemzeilen verschwinden sehen; hier stehen sie auf NO, während die Schlüssel in der Beschreibung unverändert auf `true` stehen. Die Befehlszeile ist die oberste Vorgabenebene und überschreibt damit `registerDefaults:`:

```
$ ./target/KRK.app/Contents/MacOS/krk --menue-protokoll \
    -NSDisabledCharacterPaletteMenuItem NO -NSDisabledDictationMenuItem NO
```

Danach standen im Menü "Bearbeiten" wieder fünf Systemzeilen: zweimal "Start Dictation…" und dreimal "Emoji & Symbols", darunter Cmd+Leertaste sichtbar und Ctrl+Cmd+Leertaste verdeckt. Ohne die beiden Argumente, im selben Bündel, waren es null. **Wären die Schlüssel der Beschreibung wirksam, hätte eine Nutzervorgabe von NO die Zeilen nicht zurückholen können.** Der Befund des `coder` ist damit unabhängig bestätigt: AppKit liest die beiden ausschließlich aus `NSUserDefaults`.

Nebenbefund: die beiden entfernten Zeilen brachten den einzigen Doppelstrich der Datei mit, in `--menue-protokoll` innerhalb des Kommentars. Ein XML-Kommentar darf keinen tragen. `plutil -lint` hat das durchgelassen, `xmllint --noout` nicht; seit dem Entfernen enden beide mit 0. Der neue Kommentar vermeidet den Doppelstrich.

Geprüft am 260805-0820:

- `plutil -lint resources/Info.plist` endet mit 0, `xmllint --noout resources/Info.plist` ebenfalls.
- `plutil -extract NSDisabledCharacterPaletteMenuItem raw resources/Info.plist` findet den Schlüssel nicht mehr.
- Der Platzhalter `__KRK_VERSION__` steht unverändert in `CFBundleShortVersionString`.

**Ein Satz im Code wird mit dem nächsten Bündelbau falsch.** Der Kopfkommentar von `systemzusaetze_unterdruecken` in `crates/krk-ui/src/appkit/menue.rs:126` führt als Beleg an, dass `plutil -extract` für beide Schlüssel in `KRK.app/Contents/Info.plist` `true` liefert. Das galt für das Bündel vom 260805-0800 und gilt für das nächste nicht mehr, weil die Schlüssel aus der Vorlage weg sind. Die Messung selbst bleibt gültig, ihre Nachstellbarkeit nicht; der Satz gehört auf die Vergangenheitsform gezogen oder auf diesen Datensatz verwiesen. `menue.rs` ist Code und gehört dem `coder`.
