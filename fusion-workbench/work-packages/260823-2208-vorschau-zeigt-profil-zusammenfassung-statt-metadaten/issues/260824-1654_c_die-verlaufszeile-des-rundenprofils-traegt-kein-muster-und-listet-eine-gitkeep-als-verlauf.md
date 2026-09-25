Die Verlaufszeile des Rundenprofils trägt kein `muster` und listet eine `.gitkeep` als Verlauf

---

`resources/default-readers.toml:302-304` schreibt
`juengste = { ordner = "history", anzahl = 10 }` ohne `muster`. Die zwei anderen `juengste`-Zeilen
der Datei tragen `muster = '\.md$'`, und das Beispiel im Kommentarkopf (`:92-94`) auch. Am Bestand
dieser Werkbank kostet die Auslassung genau einen falschen Eintrag.

---

**Gemessen am 260824-1654.** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/history/`
führt genau einen Eintrag, `.gitkeep`, 0 Bytes. Der Baustein filtert allein auf
`Eintrag::typ == Typ::Datei` (`bausteine.rs:352`), nimmt die Datei also auf; `titel`
(`bausteine.rs:474-479`) findet in einer leeren Datei keine Titelzeile und fällt auf den
Dateinamen zurück. Die Zusammenfassung dieses Rundenverzeichnisses zeigt damit:

```
Die jüngsten zehn Verläufe:
    .gitkeep
```

Es ist der einzige Fall im Bestand: die übrigen achtzehn `history`-Ordner führen ausschließlich
`.md`-Dateien, nachgezählt mit `find … -type f ! -name '*.md'` über alle neunzehn Verlaufsspeicher.

**Warum es kein reiner Schönheitsfehler ist.** Der Circle `260804-0933-…` ist zurückgestellt
(`_d_`) und einer der zwei, für die die vierte Zustandszeile aus `b5bf2e3` gebaut wurde. Die
Zusammenfassung, die diese Runde für ihn erzeugt, sagt in einer Zeile richtig „Abgelegt: ja" und in
der nächsten, sein jüngster Verlauf heiße `.gitkeep`.

**Vorschlag.** `muster = '\.md$'` ergänzen, wie in den zwei anderen `juengste`-Zeilen. Der
Haushalt ändert sich dadurch nicht nach oben: die Zahl der Öffnungen kann nur sinken, und die
Messung zu C6.7 in `crates/krk-core/tests/leseprofil.rs:2136-2139` prüft gegen einen Prüfordner mit
zehn `.md`-Verläufen und bleibt bei (5, 11).

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

---
Resolved: Die Verlaufszeile des Rundenprofils trägt jetzt `muster = '\.md$'` wie die zwei anderen
`juengste`-Zeilen der Datei und das Beispiel im Kommentarkopf.

**Gemessen am 260824-1739** mit `regex` 1.13.1 über die achtzehn Rundenverzeichnisse: die Zahl der
aufgenommenen Verläufe fällt von 151 auf 150, und der eine wegfallende Eintrag ist die `.gitkeep`
in `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/history/`. Kein Eintrag ohne
`.md` bleibt übrig.

Der Haushalt ist unverändert, wie der Vorschlag es vorhergesagt hat: das Rundenprofil steht
weiterhin bei fünf Leseläufen und elf Öffnungen, und
`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` bleibt bei `(5, 11)` grün.
`make check` grün, Exit 0.
