Die Meldung der Teillesungsprobe sagt, die Zählung sage, dass es mehr sind; die berichtigte Fassung sagt das Gegenteil

---

`crates/krk-core/tests/leseprofil.rs:1382` trägt als Fehlermeldung den Satz „die Zaehlung sagt,
dass es mehr sind, und keine Zahl". Genau das sagt die gebaute Zählung nicht mehr. Die
Berichtigung von C6.5 vom 260824-1722 schreibt aus: „Dass es **mehr** sind, sagt sie nicht, denn
ein Treffer hinter dem Abbruch ist möglich und nicht gesichert."

---

Die Meldung ist der letzte Rest der Fassung „über 2.000", die drei Räumungen dieser Runde aus
Spec, Plan, Modulkopf und Kommentarzeilen genommen haben. Sie steht an der Probe, die die Regel
über die Teillesung belegt, also an der Stelle, an der ein Leser die Regel nachschlägt.

**Gemessen am 260824-1852.** Der Doc-Kommentar von `Wert::UeberGrenze`
(`crates/krk-core/src/leseprofil/mod.rs:530-540`) und der Modulkopf von
`crates/krk-core/src/leseprofil/bausteine.rs:47` sagen seit `79209c8` beide „mindestens"; die
Anzeige lautet `mindestens {Treffer} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)`
(`mod.rs:574`). Die Kommentarzeile der Auslieferungsfassung
(`resources/default-readers.toml:164`) sagt es ebenfalls richtig. Übrig ist diese eine Meldung.

**Die Probe selbst misst richtig.** Sie prüft `Wert::UeberGrenze(HOECHSTENS_EINTRAEGE as u64)`,
also die Zahl der Treffer, und die drei übrigen Zeilen der Probe prüfen die zwei anderen
Anwendungen der Regel. Falsch ist allein der Satz, den sie im Fehlerfall ausgibt — und der
erscheint nur, wenn jemand die Zusage bricht und dann die falsche Begründung liest.

**Abstellen:** die Meldung auf „die Zählung nennt die Treffer und den Abbruch, nicht eine Zahl"
ziehen. Eine Zeile Arbeit für den `coder`; der Abgleich darf Code nicht anfassen.

Gefunden beim Abgleich zum Abschluss der Runde 16, 260824-1852.
