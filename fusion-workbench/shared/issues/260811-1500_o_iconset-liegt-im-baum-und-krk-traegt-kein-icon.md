`iconset/` liegt im Baum, und KRK trägt kein Icon

---

Der Ordner `iconset/` enthält seit dem 260811-1455 zehn Dateien mit Symbolen für KRK. **Eingebunden
ist keine davon.** Das gebaute Bündel `target/KRK.app` trägt kein Symbol; der Finder und das Dock
zeigen das Standardsymbol für eine Anwendung ohne eigenes.

Vom Nutzer am 260811-1500 zur Nachverfolgung gegeben.

---

**Schwere:** Niedrig — nichts ist falsch, es fehlt etwas
**Gefunden:** Nutzer
**Betroffen:** `resources/Info.plist`, `xtask/src/bundle.rs`, neu: `resources/` oder `iconset/`
**Domain:** code

## Der Bestand, am 260811-1500 geprüft

```
iconset/commander-icon-small.svg      1.344 B
iconset/commander-icon.svg            2.329 B
iconset/commander.ico                98.287 B
iconset/icon-16.png  icon-32.png  icon-64.png  icon-128.png
iconset/icon-256.png  icon-512.png  icon-1024.png
```

Sieben PNGs in den Kantenlängen 16 bis 1024, zwei SVGs, eine `.ico`.

**Was heute fehlt, an drei Stellen:**

1. `resources/Info.plist` trägt **keinen** Icon-Schlüssel. Ein `grep -i icon` liefert null Treffer.
2. Im ganzen Baum liegt **keine** `.icns`. macOS liest genau dieses Format aus dem Bündel.
3. `xtask/src/bundle.rs:11` schreibt im Modulkopf über das Bündelverzeichnis: „`Resources/` noch
   leer, spaetere Schritte legen hier ab". Der Schritt ist nie gekommen.

## Was zu tun wäre

Die sieben PNGs tragen genau die Kantenlängen, die `iconutil` erwartet — der Weg über ein
`.iconset`-Verzeichnis und `iconutil -c icns` ist der von Apple vorgesehene und braucht kein
zusätzliches Werkzeug. Zu klären ist dabei:

- **Wird die `.icns` eingecheckt oder beim Bau erzeugt?** Erzeugen hält eine Quelle statt zweier,
  verlangt aber `iconutil` auf dem Baugerät. Einchecken macht den Bau unabhängig, legt aber
  dieselbe Grafik ein zweites Mal in den Baum.
- **Welcher Schlüssel:** `CFBundleIconFile` ist der ältere Weg und nimmt eine `.icns` aus
  `Resources/`. `CFBundleIconName` verlangt einen Asset-Katalog und damit `actool`, also
  Xcode-Werkzeuge, die dieses Projekt bisher nicht braucht. **Der erste Weg passt zum Bestand.**
- **Die Kantenlängen sind unvollständig für ein sauberes `.icns`.** Apple erwartet je Größe eine
  einfache und eine `@2x`-Fassung. Aus 16/32/64/128/256/512/1024 lassen sich die Paare bilden
  (32 ist das `@2x` von 16 und so fort), aber das ist eine Zuordnung, die jemand treffen muss.
- **`commander.ico` gehört nicht hierher.** Sie ist das Windows-Format; auf dem Mac hat sie keine
  Verwendung. Ob sie im Baum bleibt, ist eine eigene kleine Frage.

## Warum das kein Circle ist

Es ist ein abzählbarer Handgriff an zwei Dateien und keine Runde: eine Zeile in `Info.plist`, ein
Schritt in `bundle.rs`, der die `.icns` nach `Contents/Resources/` legt. Wenn beim Anfassen
auffällt, dass ein Asset-Katalog oder mehrere Symbolvarianten gewollt sind, wächst es — dann ist
es ein Circle, und dieser Datensatz ist sein Anlass.

## Zusammenhang

Der Bündelbau signiert seit der Runde 1 (`cargo xtask bundle`). **Eine Änderung am Bündelinhalt
ändert die Signatur**, was hier folgenlos ist, aber beim Auslieferungspaket
(`cargo xtask release`, mit Beglaubigung) mitgedacht gehört: die `.icns` muss vor dem Signieren
im Bündel liegen, nicht danach.
