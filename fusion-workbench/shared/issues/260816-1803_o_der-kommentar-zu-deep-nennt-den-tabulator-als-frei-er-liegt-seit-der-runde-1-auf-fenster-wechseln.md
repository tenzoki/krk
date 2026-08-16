Der Kommentar zu „Deep" nennt den nackten Tabulator als frei — er liegt seit der Runde 1 auf `fenster_wechseln`

---

Der Kommentarblock der Funktion `tiefe_suche_umschalten` in
`resources/default-keymap.toml` nennt vier Kombinationen als „weiterhin frei":
`shift+cmd+f`, `opt+cmd+f`, `ctrl+cmd+f` und den nackten Tabulator. Die ersten
drei stimmen. Der vierte nicht: `tab` liegt seit der Runde 1 auf
`fenster_wechseln`.

Die Aussage stammt aus der Antwortzeile von
`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche.md`
und ist von dort in den Kommentar gewandert.

---

**Schwere:** niedrig. Kein Verhalten ist falsch, und keine Probe hält die
Aussage — genau deshalb steht sie noch da. Falsch wird sie erst für den
Nächsten, der eine freie Kombination sucht und den Tabulator für eine hält.

**Gefunden von:** `ontocoder`, bei Schritt E2 der elften Runde
**Betroffen:** `resources/default-keymap.toml`, Kommentarblock bei
`tiefe_suche_umschalten`; die Antwortzeile des zitierten Entscheids trägt
dieselbe Aussage
**Domain:** data

## Warum der gemeinsame Speicher und nicht der Circle

Der Befund ist beim Arbeiten an der elften Runde aufgefallen, aber nicht durch
sie entstanden: die Aussage steht seit der Runde 10 da und wäre ohne den
Inhaltsfilter genauso falsch. Herkunftsregel — gefunden daneben, nicht daraus
hervorgegangen.

## Was zu tun ist

Den Tabulator aus der Aufzählung nehmen, an beiden Stellen. Ob die
Antwortzeile eines geschlossenen Entscheids nachgezogen wird, ist die zweite
Frage: sie ist die Aufzeichnung eines damaligen Standes, und die Aussage war
schon damals falsch. Eine Berichtigungsnotiz darunter träfe es besser als eine
stille Änderung.
