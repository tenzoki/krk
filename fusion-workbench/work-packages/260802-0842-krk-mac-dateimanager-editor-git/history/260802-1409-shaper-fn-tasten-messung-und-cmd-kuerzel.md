# Messergebnis der Fn-Tasten-Prüfung und zweiter Weg über Cmd-Kürzel

**Agent:** shaper (in-Circle clarification)
**Zeitpunkt:** 260802-1409 bis 260802-1425
**Status:** Complete
**Auftrag:** Das Messergebnis aus `spikes/fn-tasten/messung-A.txt` und drei Antworten des Nutzers in den Spec einarbeiten, den Entscheidungsdatensatz zu den F-Tasten fortschreiben, und begründet entscheiden, ob die ungemessene Frage 3 den Plan bindet.

## Ausgangslage

Die Vorprüfung zu C3 des Navigator-Specs war gemessen, die Antworten des Nutzers lagen vor. Der Spec beschrieb die Tastenbelegung noch in der Schreibweise "Fn+F3 bis Fn+F8" und begründete sie damit, dass die Fn-Kombination auf jedem Mac ankommt, während die nackten Funktionstasten vom System verbraucht werden.

## Was die Messung ergeben hat, und wie sie zu lesen ist

Gemessen am Abnahmegerät `MacBookPro15,1`, macOS 15.7.7, Systemeinstellung "F1, F2 usw." aus.

| Frage | Befund | Beleg |
|---|---|---|
| 1. Kommen Fn+F3 bis Fn+F8 an? | ja, Tastencodes 99, 96, 100, Zeichen U+F706, U+F708, U+F70B, Modifikator `function` gesetzt | `spikes/fn-tasten/messung-A.txt`, #03 bis #05 |
| 2. Kommen die nackten F3 bis F8 an? | ungemessen, entgegen der Selbstauswertung des Programms | ebd., #08 bis #12 |
| 3. Wirkung der Systemeinstellung | ungemessen, auf einem Gerät ohne Funktionstastenreihe nicht sinnvoll messbar | — |
| 4. Löst fn selbst ein `flagsChanged` aus? | ja, Tastencode 63, kein `keyDown`; Kontrollprobe mit Shift bestätigt den Abgriff | ebd., #14 bis #17 |

Zu Frage 2: die Datei meldet "JA, 3 von 3". Im rohen Protokoll steht bei #08 ein `flagsChanged geändert=+function` unmittelbar vor den drei Tastendrücken des zweiten Abschnitts und bei #12 das zugehörige `-function`. Der Nutzer musste fn halten, weil sein Gerät ohne fn keine F3 erzeugt; Abschnitt 2 hat Abschnitt 1 wiederholt. Die Auswertungslogik prüft nur, ob ein Tastendruck ankam, nicht ob fn dabei gedrückt war. Der Programmfehler ist parallel von einem anderen Agenten behoben worden. Die korrigierte Auswertung in `spikes/fn-tasten/messung-A-neuauswertung.txt` deckt sich mit der Lesart, die der Spec festhält: Frage 2 ist auf diesem Gerät nicht messbar.

Die tragende Folgerung: KRK belegt Tastencode 99 und nicht "Fn+F3". Beide Wege erzeugen dasselbe Ereignis. Das stützt die frühere Festlegung, dass eine Belegung je Funktion genügt, und macht die Beschriftung "Fn+F3" für Nutzer mit echter Funktionstastenreihe falsch.

## Die drei Antworten des Nutzers

1. Alltagstastatur ist die eingebaute Tastatur eines Apple-Silicon-MacBooks mit echter Funktionstastenreihe. Der Touch Bar steht nur beim Abnahmegerät im Weg.
2. Die Norton-Reihe bleibt auf den Funktionstasten, und jede dieser Funktionen trägt ab Werk zusätzlich ein Cmd-Kürzel. Zwei Wege ab Werk auf dieselbe Funktion, keine zweite Belegungsart. Die konkreten Kürzel hat der Nutzer dem Shaper überlassen.
3. Die Belegungsansicht beschriftet die Funktionstaste als `F3`, nicht als `Fn+F3`.

## Was geschrieben wurde

**Spec** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Marker unverändert `_o_`:

- C3 trägt jetzt drei neue Erklärabschnitte: was KRK technisch belegt, wie die Messung zu lesen ist, und was ungemessen bleibt.
- Die Abnahmekriterien in C3 sind neu geschnitten. Neu sind die Tabelle der Cmd-Kürzel, die Beschriftungsregel, das Verbot von fn als Zusatztaste einer Belegung, die Konfliktfreiheit der Auslieferungsbelegung und die Reservierung von Cmd+C und Cmd+V.
- Die Ein-Zeilen-Regel der Belegungsansicht ist verallgemeinert: die Zeile gehört der Funktion, alle Kombinationen stehen darin.
- C4, C7, die Randbedingungen und der Abschnitt "Nicht in dieser Runde" sind auf die Tastencode-Schreibweise umgestellt.
- Der Abschnitt "Abgleich mit der Circle-Directive" nennt die neue Abweichung und verweist auf den dafür abgelegten Defekt.

**Entscheidungsdatensatz** `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`, Marker unverändert `_a_`: Nachtrag mit der Messung, der geänderten Lage und der Feststellung, dass die Wahl zwischen Möglichkeit 1 und Möglichkeit 3 technisch nie bestand. Die `Answered:`-Zeile zitiert jetzt den Abschnitt statt einer Zeilennummer, die bei jeder Überarbeitung verrutscht.

**Defekte:**

- `issues/260802-1417_o_directive-zeile-sagt-freie-funktionstasten-zu.md` — die Directive-Zeile im Circle-Datensatz sagt zu, die nackten Funktionstasten frei zu lassen, was die Messung als unerfüllbar ausweist, und nennt den zweiten Weg über die Cmd-Kürzel nicht.
- `issues/260802-1417_o_circle-datensatz-status-widerspricht-dem-marker.md` — der Datensatz führt Status "anticipated" bei Marker `_t_` und meldet keine aktive Sitzungshistorie, obwohl sechs Protokolle vorliegen.
- `issues/260802-1330_c_abnahmegeraet-hat-keine-physische-f-tastenreihe.md` — geschlossen, weil der Nutzer beide offenen Fragen des Defekts beantwortet hat.

## Die festgelegten Cmd-Kürzel

| Funktion | Norton-Taste | Cmd-Kürzel | Herkunft |
|---|---|---|---|
| Vorschau anzeigen | F3 | Cmd+Y | Finder: Übersicht |
| Kopieren | F5 | Cmd+Shift+K | eigene Form, K wie Kopieren |
| Verschieben | F6 | Cmd+Shift+V | eigene Form, V wie Verschieben |
| Ordner anlegen | F7 | Cmd+Shift+N | Finder: Neuer Ordner |
| Endgültig löschen | F8 | Cmd+Opt+Delete | Finder: sofort löschen |
| In den Papierkorb | Delete | Cmd+Delete | Finder: In den Papierkorb legen |

Die Regel dahinter: wo der Mac für genau dieselbe Funktion bereits ein Kürzel kennt, übernimmt KRK es unverändert. Für die beiden Übertragungen zwischen den Dateifenstern gibt es kein Vorbild, weil der Finder nur den Zweischritt über die Zwischenablage kennt; sie erhalten eine eigene, einheitliche Form mit dem Anfangsbuchstaben des deutschen Verbs. Cmd+C und Cmd+V bleiben unbelegt, weil das Kopieren in KRK ein Einschrittvorgang zwischen den Fenstern ist und nicht das Ablegen in der Zwischenablage.

Die letzte Zeile der Tabelle, Cmd+Delete für den Papierkorb, geht über die Antwort des Nutzers hinaus: er hat nur die Norton-Reihe genannt. Der Shaper hat sie ergänzt, weil ein Papierkorb-Befehl ohne Cmd-Weg neben fünf Funktionen mit Cmd-Weg die Art von Asymmetrie ist, die später eine Sonderregel nach sich zieht.

Die Prüfung gegen die Systemkürzel von macOS beruht auf der dokumentierten Kürzelliste, nicht auf einer Messung. Das steht im Spec so gekennzeichnet.

## Prüfung gegen "supersimpel"

Bestanden. Die zweite Vorbelegung fügt keinen Mechanismus hinzu: die Belegung bildet ohnehin Kombination auf Funktion ab, und mehrere Kombinationen auf einer Funktion sind in einer frei konfigurierbaren Belegung der Normalfall. Keine Ausnahme, keine Fallunterscheidung, kein Rückfallweg. Der einzige Preis liegt in der Belegungsansicht und ist mit der Ein-Zeilen-Regel bezahlt. Ausgeschlossen sind ausdrücklich drei Wege, die die Maxime verletzen würden: Geräteerkennung mit je nach Tastatur verschiedener Vorbelegung, ein Umschaltmodus, und zwei getrennte Belegungstabellen.

## Entscheidung zur ungemessenen Frage 3

Frage 3 bindet den Plan nicht. Begründung: KRK belegt den Tastencode und kann die beiden Wege nicht unterscheiden, also hängt kein Verhalten und kein Abnahmekriterium am Ergebnis. Die Systemeinstellung verschiebt allenfalls, welche Fingerhaltung den Tastencode erzeugt. Für den Nutzer, der sie einschaltet, bleibt die Beschriftung "F3" richtig, und das Cmd-Kürzel steht ohnehin daneben. Dieselbe Begründung trägt für die ebenfalls ungemessene Frage 2.

Nachzuholen ist die Messung, sobald eine von zwei Bedingungen eintritt: ein Abnahmekriterium einer späteren Runde sagt ein bestimmtes Verhalten bei eingeschalteter Systemeinstellung zu, oder ein Nutzer meldet, dass eine Funktionstaste bei ihm nicht auslöst. Die Anleitung steht in `spikes/fn-tasten/README.md` als Durchgang B und C.

## Vom Shaper selbst entschieden

Zwei Punkte, die der Nutzer nicht angesprochen hat, stehen als begründete Festlegung im Spec:

- Die Gleichsetzung von "Vorschau anzeigen" aus C3 mit dem Ein- und Ausblenden des Vorschaufensters aus C7. Zwei getrennte Funktionen mit fast gleicher Wirkung wären eine Sonderregel ohne Gegenwert. Das ist der Punkt dieser Runde, den der Nutzer am ehesten anders sehen könnte.
- Cmd+Delete als zweiter Weg für den Papierkorb, siehe oben.

## Nicht angefasst

Der Circle-Datensatz, `spikes/`, jeder Plan, jeder Code. Kein Commit. Der Spec trägt weiterhin den Marker `_o_`.
