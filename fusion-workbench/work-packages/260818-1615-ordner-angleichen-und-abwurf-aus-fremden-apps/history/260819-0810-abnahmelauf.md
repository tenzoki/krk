# Abnahmelauf der Runde 13 — Verzeichnis-Angleichen und Abwurf aus fremden Anwendungen

**Datum:** 260819
**Gefahren von:** dem Nutzer, an KRK im Vordergrund
**Bündel:** `target/KRK.app`, Version 0.5.2, gebaut mit `cargo xtask bundle`, signiert mit
„KRK Entwicklung"
**Baumstand:** `801d594`
**Ergebnis:** alle zehn Prüfungen halten. Wortlaut der Rückmeldung: „alles ok".

## Warum dieser Datensatz besteht

Der Abnahmelauf dieses Projekts verlangt KRK im Vordergrund und ist damit Nutzerarbeit; kein
Agent kann ihn fahren, und die Messstrecke meldet aus dem Hintergrund `NICHT_IM_VORDERGRUND`
statt Zahlen. Zehn der zwölf vorigen Runden schließen deshalb beschränkt (`_b_`).

Die Runde 13 schließt kohärent, und die Grundlage dafür ist allein dieser Lauf. Stünde er nur
in der Abschlussnotiz des Circle-Datensatzes, hinge der Beleg an der Aussage, die er belegen
soll. Die Runde 8, die einzige vorige mit einem gefahrenen Lauf, hat ihre Liste hinterlegt;
diese Datei tut dasselbe. Der Anstoß kam vom `playmaker` beim Nachziehen des Portfolios, der
den fehlenden Datensatz als Warnung gemeldet hat.

**Was dieser Datensatz nicht ist.** Er ist keine Messung. Die zehn Zeitzusagen aus C8 der
Runde 1 sind hier nicht gefahren; sie stehen weiterhin auf dem Lauf vom 260810
(`messungen/260810-1918-alle-zusagen.txt`), der vor den Runden 5 bis 13 liegt. Diese Runde
setzt keine elfte Zusage und verlangt keinen neuen Lauf.

## Die zehn Prüfungen

Vorbedingung, vorher geprüft: unter `~/Library/Application Support/KRK/` liegt keine eigene
`keymap.toml`, also kommt `ordner_angleichen` mit `opt+cmd+s` belegt heraus. Ohne diese
Prüfung schlüge jedes Kriterium zu C1 und C2 fehl, obwohl der Code stimmt
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`,
offen).

### Fähigkeit C1 bis C3 — das Angleichen

| | Prüfung | Kriterium |
|---|---|---|
| 1 | Beide Dateifenster auf verschiedene Ordner, `opt+cmd+s`: das andere steht danach auf dem Ordner des aktiven, der Fokus bleibt stehen | C1 |
| 2 | Zielfenster ausgeblendet, `opt+cmd+s`: es kommt hervor **und** steht auf dem Ordner | C2, erstes Kriterium |
| 3 | Fenster so schmal, dass die Mindestbreiten nicht passen: das Zielfenster bleibt ausgeblendet und auf seinem alten Ordner, die Statuszeile nennt den Grund | C2, zweites und drittes Kriterium |
| 4 | Fokus in Vorschau oder Editor: `opt+cmd+s` löst nichts aus, der Menüeintrag ist ausgegraut | C1, letztes Kriterium |

### Fähigkeit C4 bis C7 — der Abwurf

| | Prüfung | Kriterium |
|---|---|---|
| 5 | Datei aus dem Finder über eine Ordnerzeile: die Zeile hebt sich hervor, Loslassen kopiert hinein | C4 |
| 6 | Dieselbe Datei über eine Dateizeile oder freien Bereich: die ganze Liste markiert sich, Loslassen kopiert in den angezeigten Ordner | C4 |
| 7 | Mit `cmd` gezogen wird verschoben, mit `opt` kopiert, und der Zeiger stimmt mit der Wirkung überein | C5 |
| 8 | Über das **nicht aktive** Dateifenster gezogen, nachdem vorher ein Befehl eine Antwort geschrieben hat: die Abwurfmeldung erscheint | C7, und der Befund `260818-2332` |
| 9 | Mail-Anhang oder Bild aus „Fotos": der Zeiger weist ab, die Statuszeile nennt den Grund, im Zielordner entsteht nichts | C7 |
| 10 | Abwurf während eines laufenden Kopiervorgangs: der Zeiger weist schon vor dem Loslassen ab | C6 |

Prüfung 8 ist die, die vor `4d27c1c` fehlgeschlagen wäre: der Abwurf schrieb seine Meldung in
Rang 1, ohne die andere Seite zu räumen, und die stehende Befehlsantwort gewann.

## Was dieser Lauf nicht abdeckt

Die Kriterien zu C4 und C5, die mehrere Einträge, einen ganzen Ordner, die Konfliktrückfrage,
Fortschritt und Abbruch sowie eine Quelle verlangen, die allein Kopieren anbietet, sind in
diesen zehn Prüfungen nicht einzeln aufgeführt. Sie sind vom Nutzer mit „alles ok" mit
abgenommen worden; wer es genauer braucht, fährt sie einzeln nach der Tafel unter
`## Nutzerarbeit` des Plans
(`planning/260818-1633_*_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`).
