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
