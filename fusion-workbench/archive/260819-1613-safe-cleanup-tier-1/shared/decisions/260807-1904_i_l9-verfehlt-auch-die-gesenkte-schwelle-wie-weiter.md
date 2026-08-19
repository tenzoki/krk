# L9 verfehlt auch die am 260807 gesenkte Schwelle — wie weiter?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (auf Nutzerentscheid)
**Cross-references:** `shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`, `messungen/260807-1538-alle-zusagen.txt`, `messungen/260805-2207-MacBookPro15-1-abnahme.txt`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`

---

## Frage

Am 260807-0832 hat der Nutzer L9 von 95 auf 85 Prozent im ersten Bild gesenkt,
und zwar auf den gemessenen Boden der Abnahmereihe vom 260805-2207 (90, 85, 90,
100, 85). Der erste vollständige Lauf danach, `messungen/260807-1538-alle-zusagen.txt`,
liefert 90, 75, 80, 65, 70 Prozent: die Zusage hält in einer von fünf Runden.
Der Boden, auf den sie gesenkt wurde, existiert nicht mehr.

Die zweite Hälfte der Zusage hält unverändert: kein Einzelwert liegt über 1,70
von zwei erlaubten Bildlängen. Verschlechtert hat sich allein der Anteil, der
das **erste** Bild erreicht.

C8 verlangt bei einer verfehlten Zusage einen Datensatz statt einer
stillschweigenden Lockerung. Das ist dieser.

## Möglichkeiten

1. **Den Defekt zurückstellen, die Zusage bei 85 Prozent lassen.** Der Spec sagt
   weiter 85 zu, der nächste Abnahmelauf meldet die Abweichung erneut, und die
   Ursache bleibt offen sichtbar.
   - Für: die Zusage bleibt eine Zusage, und die Lücke ist aktenkundig.
   - Gegen: der Spec beschreibt einen Zustand, den die Anwendung nicht hat.
2. **Die Zusage auf den gemessenen Stand senken**, also auf 65 Prozent im ersten
   Bild bei unveränderter Obergrenze von zwei Bildlängen.
   - Für: Spec und Anwendung stimmen wieder überein.
   - Gegen: die zweite Senkung derselben Zusage binnen eines Tages. Sie verdeckt
     die Ursache, statt sie offen zu halten, und 65 ist wieder ein Boden ohne
     Spielraum: der Lauf vom 260807 traf ihn in Runde 4 genau.
3. **L9 streichen.** KRK sagt zur Tastatur während einer laufenden Kopie nichts
   mehr zu.
   - Gegen: die Maxime „superschnell" verlöre ihre einzige Zusage für den Fall,
     dass im Hintergrund etwas läuft.

## Randbedingungen

- Die Obergrenze von zwei Bildlängen bleibt in jedem Fall: sie hält gemessen in
  allen fünf Runden und ist die Hälfte der Zusage, die den Nutzer vor einer
  wirklich hängenden Oberfläche schützt.
- Der Ursachendefekt bleibt bestehen, gleich wie die Zusage lautet. Eine
  gesenkte Schwelle beantwortet nicht, warum die Anwendung langsamer geworden
  ist.

## Empfehlung

Möglichkeit 1. Eine Zusage, die zweimal an den Messwert nachgezogen wird, misst
nicht mehr die Anwendung, sondern sich selbst. Der Nutzer hat anders
entschieden, und die Gründe dafür stehen unten.

---
Answered: Der Nutzer hat am 260807-1900 **Möglichkeit 2** gewählt, gegen die
Empfehlung dieses Datensatzes. Seine Begründung: die Abweichung von den
Sollwerten ist ihm nicht kritisch genug, um die Arbeit an der Anwendung dafür
anzuhalten; die Runde 1 steht, und der nächste Schritt ist die Editor-Runde.
L9 fordert damit **mindestens 65 Prozent der Eingaben im ersten Bild und keine
einzige über zwei Bildlängen**. Beide Hälften sind durch den Lauf vom
260807-1538 belegt: der schlechteste Rundenanteil ist 65,0 Prozent, der größte
Einzelwert 1,70 Bildlängen.

**Was diese Wahl kostet, ausdrücklich festgehalten.** Erstens hat L9 damit zum
zweiten Mal an einem Tag ihre Schwelle an den Messwert angepasst; die Zusage
sagt seither weniger über die Anwendung als über die letzte Messung. Zweitens
ist 65 wieder ein Boden ohne Spielraum, und eine spätere Reihe mit einer Runde
darunter verfehlt erneut. Drittens bleibt die Ursache unaufgeklärt: der Defekt
`shared/issues/260807-1748_*_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`
führt drei ausgeschlossene Erklärungen und drei geordnete Verdächtige und wird
mit dieser Antwort **nicht** geschlossen.

Superseded by: dieser Datensatz löst
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_s_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md` ab.
Jener Datensatz trägt seit dem 260807-1900 den Zustand "überholt" und die Zeile
`Superseded by:` mit dieser Datei.

Implemented: `crates/krk-bench/src/messen.rs:1148` — `mindestanteil_prozent: 65`
in der Zusage L9, `obergrenze_bilder` unverändert `Some(2)`; der `coder` hat die
Auswertung samt Berichtstext und Proben umgestellt, festgehalten in
`shared/history/260807-1920-l9-schwelle-auf-65-prozent.md`. Die Zusage selbst
steht an vier Stellen des Specs
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md`:
Standabsatz im Kopf (:12), Vorspann der Abnahmekriterien von C8 (:359),
Zeile L9 der Zusagentabelle (:371) und Messvorschrift (:386); die drei Kosten
der Wahl stehen im selben Abschnitt unter `Getroffene Festlegungen` (:413 bis
:417). Im Plan
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_c_plan-navigator-geruest-runde-1.md`
sind der Nachzugsabsatz im Kopf (:27), `### Frage 5` (:267), `### Frage 6` (:273),
S21 (:1167), S22 (:1182) und die Aufstellung der Datensätze am Fuß (:1357)
nachgezogen. **Spec und Plan bleiben geschlossen**, und kein Planschritt verliert
sein `[DONE]`: beide Einträge sind ausdrücklich als Nachtrag nach dem Abschluss
der Runde gekennzeichnet, weil C8 den Zusagenbestand des Projekts führt und die
nächste Runde ihn erbt. Der Ursachendefekt
`shared/issues/260807-1748_*_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`
bleibt offen und trägt seit dem 260807-1900 einen Abschnitt, der das ausspricht.

Deferred:
