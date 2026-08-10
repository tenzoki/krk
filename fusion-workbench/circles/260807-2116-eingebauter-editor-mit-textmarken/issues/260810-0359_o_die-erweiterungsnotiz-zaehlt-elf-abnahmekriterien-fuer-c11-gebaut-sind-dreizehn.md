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
