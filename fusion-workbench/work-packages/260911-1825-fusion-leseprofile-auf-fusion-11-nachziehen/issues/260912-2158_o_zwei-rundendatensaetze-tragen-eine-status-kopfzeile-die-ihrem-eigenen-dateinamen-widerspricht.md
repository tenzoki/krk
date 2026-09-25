Zwei Rundendatensätze tragen eine Kopfzeile `**Status:**`, die ihrem eigenen Dateinamen widerspricht

---

In `fusion-workbench/circles/` stehen zwei Datensätze, deren Zustandsmarker im Dateinamen und
deren Kopfzeile `**Status:**` verschiedene Zustände nennen. Solange der Marker die Auskunft war,
fiel das nicht auf. Seit KRKs Vorschau die Kopfzeile liest, zeigt sie an diesen zwei Runden den
falschen Zustand an.

---

**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Domain:** data
**Gefunden:** beim Nachziehen der fusion-Leseprofile auf Fassung 11, bei der Gegenprobe des
neuen `feld`-Bausteins an allen 26 Containern der Werkbank.
**Betroffen:** `fusion-workbench/circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_b_circle.md`,
`fusion-workbench/circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_d_circle.md`

## Der Befund

| Datensatz | Dateiname sagt | Kopfzeile sagt |
|---|---|---|
| `260809-2040-tastenbelegung-als-markdown-in-downloads` | `_b_` beschränkt geschlossen | `anticipated` |
| `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` | `_d_` zurückgestellt | `anticipated` |

Erhoben mit einem Lauf über alle 26 Container, je Name einzeln gezählt: Dateiname-Marker gegen
die erste Zeile, die auf `**Status:**` passt.

Die zweite Zeile ist die schwerere. Der Circle des Web-Betrachters ist nicht zurückgestellt im
Sinne von „später", sondern abgesagt: der Nutzer hat am 260821-2202 entschieden, dass KRK
Web-Inhalt an den Systembrowser abgibt
(`260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
`CLAUDE.md` schreibt diese Lesart ausdrücklich aus. Die Kopfzeile `anticipated` sagt das
Gegenteil: vorgesehen, noch nicht gefahren.

## Was daran nicht der Befund ist

Neun weitere Datensätze tragen gar keine Kopfzeile `**Status:**`, sämtlich terminal geschlossen
(`_c_` oder `_b_`). Das ist **kein** Defekt: eine Aufzeichnung eines Standes behält ihren
damaligen Marker, und die Umstellung öffnet einen terminalen Datensatz nicht. KRKs Vorschau
zeigt dort den Platzhalter, und der Kommentar im Rundenprofil von
`resources/default-readers.toml` schreibt genau diesen Ausgang aus.

Der Unterschied zwischen den zwei Lagen ist der, den ein Leser braucht: „der Datensatz sagt
nichts" ist eine ehrliche Auskunft, „der Datensatz sagt etwas Falsches" ist keine.

## Behebung

Die zwei Kopfzeilen auf den Zustand setzen, den der Dateiname trägt — oder sie streichen, dann
steht dort wie bei den neun anderen der Platzhalter. Welches von beidem, hängt daran, ob diese
zwei Datensätze als Aufzeichnung gelten oder als lebende Einträge; das entscheidet der Nutzer.
