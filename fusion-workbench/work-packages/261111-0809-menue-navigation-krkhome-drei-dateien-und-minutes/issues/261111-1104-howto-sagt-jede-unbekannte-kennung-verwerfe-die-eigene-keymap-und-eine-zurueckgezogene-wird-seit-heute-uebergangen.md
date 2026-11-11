`HowTo.md` sagt, jede unbekannte Kennung verwerfe die eigene `keymap.toml`, und eine zurückgezogene Kennung wird seit dem 261111 übergangen
---
`HowTo.md` schreibt unter „Die Tastaturbelegung“: „Eine fehlerhafte `keymap.toml` wird als Ganzes verworfen und nicht teilweise übernommen. Unbekannte Kennung, doppelte Kennung, unlesbare Tastenschreibweise oder ein Kombinationskonflikt: KRK meldet es beim Start und fährt mit der Auslieferungsbelegung.“ Seit `871f1062` gilt das für eine Kennung nicht mehr: `Belegung::bauen` übergeht in einer Nutzerdatei einen Eintrag, dessen Kennung in `ZURUECKGEZOGENE_KENNUNGEN` steht (heute `termine_richtung_umkehren`), ohne Meldung, und die übrige Datei gilt (C3.10, E6 des Plans). Die Anleitung sagt dem Nutzer mit einer solchen Datei damit das Gegenteil dessen, was geschieht.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

Gefunden in der Schlussdurchsicht des Bereichs `02c2f859..2cae597a` (Domäne Code). Schweregrad: niedrig. Kein Verhalten ist falsch; falsch ist ein Satz der Anleitung, die im Releasepaket mitreist.

Beleg:

- `HowTo.md`, Abschnitt `## Die Tastaturbelegung`, der Absatz, der mit „Eine fehlerhafte `keymap.toml` wird als Ganzes verworfen“ beginnt.
- `crates/krk-core/src/tasten/belegung.rs`, `Belegung::bauen`, der Zweig `None if ZURUECKGEZOGENE_KENNUNGEN.contains(…) => continue`, und die Konstante `ZURUECKGEZOGENE_KENNUNGEN` mit ihrem Doc-Kommentar.
- `grep -n 'termine_richtung\|zurückgezogen' HowTo.md README.md` liefert nichts: keine der zwei Anleitungen nennt die Duldung.

Die offene Nutzerfrage `261111-0852-gilt-die-duldung-einer-zurueckgezogenen-kennung-in-der-eigenen-keymap-fuer-jede-kuenftige-und-rueckwirkend.md` entscheidet, wie weit die Liste reicht; der eine Eintrag ist jeder ihrer Antworten gemeinsam, und der Satz der Anleitung ist unter jeder Antwort zu weit.

Abnahme: der Absatz in `HowTo.md` nennt die Ausnahme in einem Satz (eine Kennung einer zurückgezogenen Funktion, die KRK duldet, wird übergangen, die übrige Datei gilt, und das nächste Sichern der Belegung schreibt sie nicht zurück), ohne eine Zahl geduldeter Kennungen zu nennen. `make check` bleibt grün.

---
Resolved: `HowTo.md`, `## Die Tastaturbelegung`, der Absatz „Eine fehlerhafte `keymap.toml` wird als Ganzes verworfen“ nennt jetzt die eine Ausnahme: einen Eintrag mit der Kennung `termine_richtung_umkehren` des entfallenen Befehls übergeht KRK ohne Meldung, die übrige Datei gilt, und das nächste Schreiben der Belegung führt ihn nicht mehr. Eine Zahl geduldeter Kennungen steht nicht da. `README.md` nennt die Regel an keiner Stelle und blieb unverändert. Geprüft am Rumpf von `Belegung::bauen` und an der Probe `eine_keymap_mit_der_entfallenen_richtung_der_termine_laedt_und_sichert_ohne_sie`.
