# L9 auf 65 Prozent: Nachtrag in Spec, Plan und Datensätzen

**Status:** Complete
**Agent:** planner
**Datum:** 260807-1914
**Auftrag:** Dokumentenpflege nach einem Nutzerentscheid, kein Circle aktiv
**Datensatz:** `shared/decisions/260807-1904_i_l9-verfehlt-auch-die-gesenkte-schwelle-wie-weiter.md`

## Was der Auftrag verlangte

Den Nutzerentscheid vom 260807-1900 in die Dokumente ziehen: L9 fordert
mindestens 65 Prozent der Eingaben im ersten Bild statt 85, bei unveränderter
Obergrenze von zwei Bildlängen. Spec und Plan der Runde 1 sind seit dem
260807-1035 geschlossen, der Circle trägt beschränkten Abschluss. Der
Orchestrator hat entschieden, beide dennoch nachzuziehen, als datierten
Nachtrag ausdrücklich nach dem Abschluss und ohne Markerwechsel.

## Nachprüfung an der Messreihe, vor jeder Änderung

Der Datensatz behauptet zwei Zahlen, beide sind an
`messungen/260807-1538-alle-zusagen.txt` nachgeprüft und stimmen.

- **Der schlechteste Rundenanteil ist 65,0 Prozent.** Die fünf Runden liegen bei
  90,0 / 75,0 / 80,0 / 65,0 / 70,0 Prozent. Runde 4 trifft die neue Schwelle
  genau, was den zweiten der drei Kostenpunkte belegt: 65 ist ein Boden ohne
  Spielraum.
- **Die Obergrenze von zwei Bildlängen hält in allen fünf Runden.** Die
  Rundenhöchstwerte liegen bei 1,13 / 1,20 / 1,26 / 1,26 / 1,70 Bildlängen.
  Kein Einzelwert erreicht zwei.

## Was geändert wurde

**Spec `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md`:**
Datumszeile (:3) um den Nachtrag ergänzt; Standabsatz im Kopf (:12) neu, als
erster Eintrag nach dem Abschluss gekennzeichnet und mit der Begründung, warum
C8 trotzdem nachgezogen wird; Vorspann der Abnahmekriterien von C8 (:359) auf
65 Prozent und auf vier statt drei Änderungen; Zeile L9 der Zusagentabelle
(:371); Messvorschrift (:386) auf 65 Prozent und höchstens sieben verpasste von
zwanzig; zweite Festlegung unter `Getroffene Festlegungen` (:413 bis :417) mit
dem Messbefund, dem Nutzerentscheid und den drei Kosten; der Absatz in
`## Offene Nutzerentscheidungen` (:568) ist als Aufzeichnung des Vormittags
gekennzeichnet, damit seine 85 Prozent nicht als geltend gelesen werden.

**Plan `…/planning/260802-1428_c_plan-navigator-geruest-runde-1.md`:**
Nachzugsabsatz im Kopf (:27) neu, in der Form der vorhandenen und als Nachtrag
nach dem Abschluss gekennzeichnet; `### Frage 5` (:267) auf 65 Prozent, dazu
richtiggestellt, dass die Auswertung die zweiteilige Fassung seit `d569f8a`
trägt und der Defekt dazu geschlossen ist; `### Frage 6` (:273) auf den
heutigen Wortlaut; S21 (:1167) um die zweite Senkung; S22 (:1182) als neuer
Punkt, der die zweite Senkung auf einen anderen Lauf zurückführt und den Schritt
abgenommen lässt; Aufstellung der Datensätze am Fuß (:1356 ergänzt, :1357 neu).
Kein Schritt verliert sein `[DONE]`.

**Abgelöster Datensatz
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_s_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`:**
Zeile `Superseded by:` mit Pfad und Grund angehängt, Statuszeile auf
"superseded", Marker `_i_` nach `_s_` umbenannt. Das ist der eine erlaubte
Übergang von einem Endzustand in einen anderen.

**Neuer Datensatz
`shared/decisions/260807-1904_i_l9-verfehlt-auch-die-gesenkte-schwelle-wie-weiter.md`:**
Zeile `Implemented:` mit allen berührten Stellen angelegt, Statuszeile auf
"implemented", Marker `_a_` nach `_i_` umbenannt.

**Defekt
`shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`:**
Abschnitt `## Die Zusage ist nachgezogen, dieser Defekt bleibt offen`
angehängt. Er sagt, dass die Änderung die Zusage betrifft und nicht die
Anwendung, und dass der Spec deshalb wieder grün aussieht, ohne dass die Ursache
geklärt wäre. Der Marker bleibt `_o_`. Die Cross-reference auf den abgelösten
Datensatz steht jetzt in der Sternform statt mit ausgeschriebenem `_i_`.

## Was ausdrücklich nicht geändert wurde

Kein Code, keine `.toml`, keine Messdatei. Die Marker von Spec, Plan und Circle
bleiben, wie sie sind. Die dateierten Aufzeichnungen des Vormittags in Spec und
Plan bleiben inhaltlich stehen, weil sie den Stand jenes Moments tragen; der
Abgleichseintrag im Plan (:1457) nennt Zeilennummern, die durch diesen Nachtrag
verschoben sind, und bleibt als Aufzeichnung seines Prüfzeitpunkts unberührt.
Nicht committet, wie beauftragt.
