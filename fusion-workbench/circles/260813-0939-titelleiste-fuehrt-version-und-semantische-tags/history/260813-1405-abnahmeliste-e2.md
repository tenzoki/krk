# Abnahmeliste E2 — die achte Runde am laufenden Bündel

**Datum:** 260813-1405
**Bündel:** `target/KRK.app`, gebaut und signiert am 260813-1403 mit „KRK Entwicklung"
**Das beglaubigte Bündel vom 260813 ist vorher gesichert worden** nach `~/Library/Caches/krk-beglaubigt-260813-1402/KRK.app`; die Beglaubigung dort ist mit `xcrun stapler validate` bestätigt.

Elf Abnahmekriterien haben einen Anteil, den nur ein Mensch am laufenden Bündel im Vordergrund sehen kann. Sie stehen hier als Beobachtungen. Der Baumanteil ist bereits abgenommen: 48 der 59 Kriterien sind allein am Baum nachweisbar und alle 48 halten.

## Die Beobachtungen

| # | Was zu tun ist | Was zu sehen sein soll | Kriterium |
|---|---|---|---|
| 1 | KRK starten und die Titelleiste ansehen | Links steht `KRK 0.1.0`, in der Mitte der volle Pfad des angezeigten Ordners | C1.1 |
| 2 | Das Fenster schmal ziehen | Der Name links bleibt stehen; kürzt etwas, dann macOS den Pfad in der Mitte, nicht KRK | C1.9, C2.9 |
| 3 | Auf hell und auf dunkel umschalten | Der Zusatz bleibt in beiden lesbar und trägt keine eigene Farbe, sondern die Systemfarbe | C1.11 |
| 4 | Auf `KRK 0.1.0` klicken | Nichts passiert. Der Zusatz nimmt keinen Ersthelferrang an, kein Cursor, keine Auswahl | C1.6 |
| 5 | `Shift+Cmd+W`, dann `Cmd+N` | Das Fenster ist weg und kommt mit `Cmd+N` zurück. **Das ist die Regression aus Turn 1** | C1.10 |
| 6 | Menü „KRK" öffnen | „Über KRK" steht ganz oben, darunter ein Trenner | C5.1 |
| 7 | „Über KRK" anklicken | Der Standard-Dialog von macOS öffnet sich, mit Symbol, Name und Version 0.1.0 | C5.3 |
| 8 | Während der Dialog steht: `F5` drücken | Nichts geschieht im Fenster dahinter. Keine Kopieroperation startet | C5.6 |
| 9 | Während der Dialog steht: `Cmd+Q` drücken | KRK beendet sich. Die Ausnahmeliste lässt `beenden` durch | C5.6 |
| 10 | `cargo run` ohne Bündel starten, „Über KRK" öffnen | Der Dialog zeigt, was ohne `Info.plist` zu zeigen ist. Nur notieren, keine Zusage daran | C5.5 |
| 11 | **`Shift+Cmd+S` öffnen, dann `Cmd+W` drücken** | Siehe unten. Daran hängt ein Defekt der Runde 6 | — |

## Beobachtung 11 im Einzelnen

Der Freigabewähler entsteht über `showRelativeToRect:`, ist also eine Verfolgungsschleife und kein Fenster. Die neue Schlüsselfensterbedingung erreicht ihn deshalb möglicherweise nicht.

- **Schließt sich der Tab:** der Defekt `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md` bleibt offen und trägt danach einen gemessenen Befund statt einer Vermutung.
- **Geschieht nichts:** der Defekt wird mit dem Ergebnis geschlossen.

## Der erste Tag

Nach der Abnahme setzt der Nutzer `v0.1.0` auf den Commit, der die Runde schließt (C3.15). Erst danach lässt sich der grüne Fall der neuen Prüfung an einem echten `cargo xtask release` sehen; abgenommen ist er ohnehin schon an der reinen Vergleichsfunktion aus Schritt D1.

```sh
git tag v0.1.0 <abschlusscommit>
```

`README.md`, Abschnitt „Versionsstufen", sagt dazu: `v0.1.0` benennt den ersten getaggten Stand und keine Weitergabe.

## Danach

Das beglaubigte Bündel unter `target/KRK.app` ist durch den Entwicklungsbau ersetzt. Wer es zurück will, kopiert es aus `~/Library/Caches/krk-beglaubigt-260813-1402/` oder fährt `cargo xtask release` neu — Letzteres verlangt jetzt den Tag auf HEAD und einen unveränderten verfolgten Baum.

## Ergebnisse

**Gefahren am 260813-1410 vom Nutzer, am Bündel `target/KRK.app` im Vordergrund. Alle elf Beobachtungen bestanden.**

| # | Ergebnis |
|---|---|
| 1 | bestanden — `KRK 0.1.0` links, voller Pfad mittig |
| 2 | bestanden — der Name bleibt, KRK kürzt nicht |
| 3 | bestanden — in hell und dunkel lesbar, keine eigene Farbe |
| 4 | bestanden — der Klick tut nichts, kein Ersthelferrang |
| 5 | bestanden — `Cmd+N` holt das Fenster zurück. Die Regression aus Turn 1 ist am laufenden Bündel bestätigt behoben |
| 6 | bestanden — „Über KRK" steht ganz oben, Trenner darunter |
| 7 | bestanden — der Systemdialog zeigt Symbol, Name und 0.1.0 |
| 8 | bestanden — `F5` wirkt nicht ins Fenster dahinter |
| 9 | bestanden — `Cmd+Q` beendet KRK, die Ausnahmeliste lässt `beenden` durch |
| 10 | bestanden — notiert, keine Zusage daran |
| 11 | **bestanden: es geschieht nichts.** `Cmd+W` schließt bei stehendem Freigabewähler keinen Tab |

**Was Beobachtung 11 entscheidet.** Der Defekt `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_*_die-blattregel-sieht-den-freigabedialog-nicht.md` ist damit gemessen und wird geschlossen. Die Vermutung aus Schritt A3, die neue Schlüsselfensterbedingung erreiche den Wähler nicht, weil er über `showRelativeToRect:` als Verfolgungsschleife entsteht, ist am laufenden Bündel widerlegt: der Befehl kommt nicht durch. Der Befund `issues/260813-1110_o_die-schluesselfensterfrage-erreicht-den-freigabewaehler-nicht-weil-er-kein-fenster-ist.md` ist damit ebenfalls entschieden und wird geschlossen.

**Damit sind alle elf Kriterien mit Bündelanteil abgenommen:** C1.1, C1.6, C1.9, C1.10, C1.11, C2.9, C5.1, C5.3, C5.5, C5.6. Offen bleibt allein C3.15, der Tag `v0.1.0`, den der Nutzer auf den Abschlusscommit setzt.
