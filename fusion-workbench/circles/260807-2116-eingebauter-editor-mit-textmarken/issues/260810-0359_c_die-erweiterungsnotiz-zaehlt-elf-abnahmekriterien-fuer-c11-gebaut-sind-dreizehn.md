# Die Erweiterungsnotiz zählt elf Abnahmekriterien für C11, der Spec führt dreizehn

---
Der Abschnitt `## Die vier später hinzugekommenen Fähigkeiten` des Specs beziffert die Erweiterung vom 260809 mit „drei Fähigkeiten mit einunddreißig" Abnahmekriterien und schlüsselt sie auf: acht für die Fokusanzeige (C9), zwölf für die Zeilennummern (C10), elf für den Fenstertitel (C11). Gezählt am Dateibestand trägt C11 dreizehn Kriterien, nicht elf; die Summe der drei ist damit dreiunddreißig und nicht einunddreißig. C9 und C10 stimmen.

---
Gefunden am 260810-0359 beim Streichen des sechsten Abnahmekriteriums von C4 (`decisions/260810-0021_a_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`). Der Defekt ist älter und hat mit jener Änderung nichts zu tun: die Zahl für C4 in derselben Notiz stimmte vorher und ist mitgezogen worden, die für C11 stimmte schon vorher nicht.

Gemessen mit

```sh
awk '/^### C[0-9]+:/{cap=$2} /^- \[ \]/{n[cap]++} END{for(c in n) print c, n[c]}' \
  planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md
```

Ergebnis: C9 acht, C10 zwölf, C11 dreizehn.

Zwei Lesarten sind möglich, und der Datensatz entscheidet nicht zwischen ihnen. Entweder ist die Zahl beim Schreiben der Notiz falsch abgeschrieben worden, oder C11 hat nach der Notiz zwei Kriterien dazubekommen. Wer den Nachtrag fährt, prüft das an der Fassungsgeschichte und schreibt danach die Aufschlüsselung und die Summe fort.

Betroffen ist eine einzige Zeile des Specs, der Absatz `**Was die Erweiterung den Zuschnitt kostet, ist benannt und nicht klein.**`. Am Code hängt nichts: die Zahl ist eine Angabe über den Zuschnitt und keine Zusage, die eine Probe prüft.

---
Resolved: **Kein Defekt. Der Spec stand richtig, und die Messung war falsch.** C11
trägt elf Abnahmekriterien, die Erweiterungsnotiz nennt elf, und die Summe der
drei Fähigkeiten ist einunddreißig. Nachgezählt am 260810-0714:

```sh
sed -n '/^### C11:/,/^## /p' planning/260807-2147_*_spec-*.md | grep -c '^- \[ \]'
# 11
```

**Der Fehler steckt im Zählweg dieses Datensatzes.** Sein `awk` setzt die
Überschrift allein auf `^### C[0-9]+:` und trägt sie danach unverändert weiter.
Hinter C11 folgt der Abschnitt `## Verhältnis zu den zehn Zeitzusagen aus C8 der
Runde 1`, und der führt zwei eigene Abnahmekriterien: die Bedienbarkeit während
des Ladens und die Unberührtheit der zehn Zahlen. Beide sind C11 zugeschlagen
worden, weil eine `##`-Zeile die Variable `cap` nicht zurücksetzt. Elf plus zwei
ergibt die dreizehn dieses Datensatzes.

Richtig zählt ein Weg, der die Überschrift jeder Ebene übernimmt:

```sh
awk '/^#/{h=$0} /^- \[ \]/{n[h]++} END{for(k in n) print n[k], k}' \
  planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md | sort -k2
```

Die vom Datensatz aufgeworfene Frage nach der Fassungsgeschichte ist dabei
mitbeantwortet und stützt denselben Schluss: C11 trägt seit dem Commit `85293c2`,
der die drei Fähigkeiten eingesetzt hat, unverändert dieselbe Zahl. Weder ist
abgeschrieben worden noch ist C11 gewachsen.

**Am Spec ist an dieser Stelle nichts geändert worden**, außer dass die Falle jetzt
dort benannt steht: der Absatz `**Was die Erweiterung den Zuschnitt kostet**`
führt den richtigen Zählweg und sagt, warum der andere danebengreift. Der
veraltende Teil desselben Absatzes, die Zahl der Kriterien von C1 bis C8, ist
gestrichen; sie hat sich in vier Tagen dreimal geändert.
