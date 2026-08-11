# Portfolio

**Generated:** 260811-1326 (by playmaker session 260811-1326-playmaker-direct-dispatch)
**Domain bias:** code

Sechs Circles liegen unter `circles/`: einer aktiv, drei vorgesehen, zwei beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des
Zustandsmarkers eine Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht
und seine Zitate zwischen zwei Läufen altern. Ausgenommen sind die Stellen, an denen der
Marker selbst die Aussage ist. Der Defekt dazu ist
`shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`;
er sitzt in der Anweisung des Plugins und bleibt offen.

## Active (_t_)

`260809-2040-tastenbelegung-als-markdown-in-downloads` — **Runde 3, die Belegungsausgabe.**

KRK schreibt die geltende Tastenbelegung als Markdown-Datei nach
`~/Downloads/KRK-Tastenbelegung.md`, ausgelöst über einen Eintrag im Hauptmenü, gegliedert
nach denselben neun Funktionsbereichen wie die Belegungsansicht am Bildschirm.

- Aktiv seit 260811-0107, Zeiger `fusion-workbench/.active-circle` stimmt mit dem Marker überein.
- Spec: `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md`, vier Fähigkeiten C1 bis C4, 41 Abnahmekriterien.
- Plan: `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0838_*_plan-tastenbelegung-als-markdown-in-downloads.md`, vier Schritte.
- Sitzungshistorie: `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/history/260811-0107-orchestrator-session.md`.
- Stand am Dateibestand: **S1 bis S3 tragen `[DONE]`, S4 ist am 260811-1215 vom Nutzer
  gestrichen.** Der gestrichene Schritt war die Abnahme am gebauten Bündel; sie verlangt KRK
  im Vordergrund und ist Nutzerarbeit. Die 41 Abnahmekriterien stehen deshalb sämtlich auf
  `- [ ]`.
- Gelandete Arbeit: `e43f21a..caf6375`, darunter `fd863e3` (die Ausgabe selbst), `33cc083`
  (die Beschriftung der sieben Wirkungsbereiche) und `39687f3` (die Messung der sechs
  zugestellten Textbefehle).

Der Plan selbst benennt die Folge: gebaut ist die richtige Aussage über diese Runde,
abgenommen ist sie nicht. Ein kohärenter Abschluss ist damit nicht erreichbar, und die Runde
läuft auf denselben beschränkten Abschluss zu wie die beiden davor.

## Anticipated (_a_) — ranked

Recommended next: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` — kleinster
Zuschnitt, frischeste Grundlage, und keine seiner vier offenen Fragen verlangt eine Messung.
Die Aktivierung setzt allerdings voraus, dass die laufende Runde 3 vorher geschlossen wird.

### Rang 1: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`

**Vier Tastenbefehle: Ordnerpfad und Eintragspfad in die Zwischenablage, eine Datei an das
Standardprogramm des Systems, und Cmd+W auch außerhalb eines Bereichs mit Tabs.**

Dieser Circle steht vorn, weil er als einziger der drei keine unbeantwortete technische
Größe trägt. Seine Grundlage ist am 260811-1257 am Baum erhoben, mit Zeilenverweisen auf
jede tragende Feststellung: `crates/krk-ui/src/appkit/zwischenablage.rs` ist die eine Hülle
um `NSPasteboard` und heute reine Quelle, `betroffene()`
(`crates/krk-ui/src/kommandos/operationen.rs:157`) beantwortet die Frage "worauf wirkt
dieser Befehl" bereits für vier bestehende Befehle, und `NSWorkspace` ist über drei Module
schon im Haus. Der Bau besteht aus vier Zeilen in `resources/default-keymap.toml`, vier
Werten in `Kommando` und je vier Zeilen in `Kommando::wirkungsbereich`
(`crates/krk-core/src/tasten/belegung.rs`) und `bereich_des_kommandos`
(`crates/krk-ui/src/belegungsmodell.rs`). Keine dieser vier Fallunterscheidungen hat einen
Auffangzweig, der Übersetzer nennt die Stellen also von selbst. Die vier offenen
Entscheidungsdatensätze in `decisions/` dieses Circles sind Zuschnittfragen an den Nutzer,
jede mit ihren Möglichkeiten und ihren Folgen aufgeschrieben, und jede in einer
Klärungsrunde beantwortbar. Der Circle hat zudem eine Sperre bereits als Absicht erkannt
statt als Lücke: dass Cmd+W bei stehendem Blatt nicht durchkommt, liegt an
`waehrend_blatt_erlaubt` und ist eine bewusste Regel, nicht eine vergessene Zeile.

Gegen eine sofortige Aktivierung sprechen zwei Punkte, und der erste wiegt schwerer als der
zweite. **Die laufende Runde 3 ist nicht geschlossen.** Ein zweiter aktiver Circle wäre die
Lage `MULTIPLE-ACTIVE`, und der Zeiger `.active-circle` trägt genau einen Namen. Der Circle
nennt die Runde 3 außerdem selbst unter seinen Abhängigkeiten. Diese Bindung ist inhaltlich
schwach, weil die Belegungsausgabe zählt, was die Belegung führt, und vier neue Funktionen
dort ohne Zutun erscheinen. Formal bleibt das Kennzeichen trotzdem stehen: nach der
Rangheuristik zählt allein ein kohärenter Abschluss (`_c_`) als erfüllte Vorbedingung, und
die zweite Abhängigkeit `260802-0842-krk-mac-dateimanager-editor-git` ist beschränkt
abgeschlossen (`_b_`). Der zweite Punkt ist eine Zusicherung, die diese Runde bricht: der
Modulkopf von `zwischenablage.rs` sagt in zwei Sätzen zu, dass KRK die Zwischenablage in
keinem Fall schreibt. Er gehört mit derselben Änderung umgeschrieben, und der
Circle-Datensatz sagt das von sich aus.

- Abhängigkeiten: `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`),
  `260809-2040-tastenbelegung-als-markdown-in-downloads` (`_t_`). Keine davon kohärent
  geschlossen, Kennzeichen gesetzt.
- Offene Entscheidungen: vier, alle im eigenen `decisions/` dieses Circles.
- Grundlage erhoben am: 260811-1257.

### Rang 2: `260811-1304-statusleiste-mit-bereichsschaltern`

**Eine Leiste am unteren Fensterrand mit einem Schalter je Bereich, dazu eine proportionale
Neuaufteilung der Fensterzeile.**

Dieser Circle steht auf Rang 2, obwohl er den einzigen gemeldeten Nutzerdefekt der drei
mitnimmt. Seine Grundlage ist ebenso frisch und ebenso sorgfältig wie die des Erstplatzierten,
und sie enthält den Befund, der den Rang bestimmt: **die heutige Breitenregel ist nicht
proportional.** `bereichsbreiten` (`crates/krk-ui/src/fenstermodell.rs:609`) gibt den drei
festen Bereichen ihre gespeicherte Punktzahl in einer Vorrangordnung und verteilt allein den
Rest im Verhältnis. Das Beispiel der Directive, zwei Bereiche im Verhältnis 2:1 behalten
dieses Verhältnis beim Einblenden eines dritten, trifft heute nur zu, wenn beide Bereiche
Dateifenster sind. Der Entwurf verlangt damit eine neue Fassung der einen Breitenregel, und
was aus der Vorrangordnung wird, die der Nutzer am 260808 festgelegt hat, ist die erste der
sieben offenen Fragen. Solange sie unbeantwortet ist, steht der Umfang der Runde nicht fest.
Zwei weitere Fragen berühren abgenommene Fähigkeiten anderer Runden: ob die neue Leiste auch
Meldungen trägt, entscheidet über C1 der Runde 1, und wie zwei Schalter eine Fläche zeigen,
die nur einer haben kann, hängt an C1 der Editor-Runde.

Für eine frühere Aktivierung spricht ein Punkt, und er ist ernst zu nehmen. Der Defekt
`shared/issues/260811-1245_*_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`
liegt in derselben Maschinerie und ist am 260811-1240 vom Nutzer gemeldet worden, also
laufende Beeinträchtigung und keine Altlast. Ob er in dieser Runde oder in einer eigenen
davor behoben wird, ist die siebte offene Frage; der Defektdatensatz benennt zwei mögliche
Bruchstellen und verlangt, dass zuerst gemessen wird, welche es ist. Diese Messung ist aus
dem Baum heraus machbar und braucht die Runde nicht. Wer den Defekt schnell weghaben will,
lässt ihn deshalb vor dieser Runde beheben, statt die Runde vorzuziehen.

- Abhängigkeiten: `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`),
  `260807-2116-eingebauter-editor-mit-textmarken` (`_b_`). Keine kohärent geschlossen,
  Kennzeichen gesetzt. Der Circle nennt daneben ausdrücklich, dass
  `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` ihn **nicht** bindet.
- Offene Entscheidungen: sieben, alle im eigenen `decisions/` dieses Circles.
- Grundlage erhoben am: 260811-1304.

### Rang 3: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Ein eigener Betrachter für Web-Seiten in einem Tab des Vorschaufensters.**

Dieser Circle trägt den niedrigsten Zählwert an offenen Entscheidungen und steht trotzdem
hinten. Nach der reinen Zählung läge er vorn: sein Grounding zitiert einen offenen
Datensatz, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`,
gegen vier und sieben bei den beiden anderen. Der Zählwert misst hier die falsche Größe, und
zwar in beide Richtungen. Die vier und die sieben Fragen der beiden anderen Circles sind
deren eigene Aktivierungsfragen, aus dem Dateibestand beantwortbar. Der eine Datensatz hier
ist von anderer Art: er stellt eine ungemessene technische Frage, und derselbe Circle hält
in seinem Grounding fest, dass auch das Mittel der Darstellung von Web-Inhalt offen ist und
"in eine eigene Untersuchung vor dem Plan" gehört. Ein Zählwert von eins verdeckt hier mehr
ungeöffnete Arbeit als ein Zählwert von sieben dort.

Zwei weitere Gründe halten ihn hinten. **Seine Grundlage ist eine Woche alt und beschreibt
das Vorschaufenster so, wie die Runde 1 es hinterließ.** Die Editor-Runde hat genau diese
Fläche danach umgebaut: sie ist einer von fünf Fokusbereichen geworden, hat Zeilennummern
bekommen, und der Editor verdrängt sie zeitlich. Der Circle-Datensatz nennt die Editor-Runde
an keiner Stelle, weder unter ihrem Verzeichnisnamen noch über ihren Abschluss-Artefakt.
Sein Abschnitt `## Dependencies` bezeichnet daneben `260802-0842-krk-mac-dateimanager-editor-git`
weiterhin als "den aktiven Circle", obwohl jene Runde seit dem 260807-1035 beschränkt
abgeschlossen ist. **Und sein Zuschnitt ist der größte der drei:** er hebt einen
ausdrücklichen Ausschluss der Runde 1 auf, den integrierten Browser, und überholt dabei ein
abgenommenes Abnahmekriterium der Fähigkeit C10.

- Abhängigkeiten: `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`). Nicht kohärent
  geschlossen, Kennzeichen gesetzt.
- Offene Entscheidungen: eine, zitiert aus dem Speicher der Runde 1, vom Circle selbst als
  `inference:` eingeordnet. Dazu drei eigene offene Fragen im Grounding-Abschnitt, die dort
  als Prosa und nicht als Datensätze liegen.
- Grundlage erhoben am: 260804-0933. Trägt seit dem 260807-1042 einen Vermerk
  `## Parent grounding stale` und seit demselben Tag einen Aktivierungsvorschlag, der ihn als
  empfohlenen nächsten Kandidaten bezeichnet. Beide Angaben sind überholt; siehe Warnung 3.

## Recently closed (_c_ / _b_)

Zwei Circles sind geschlossen, beide als beschränkter Abschluss, beide aus demselben Grund:
der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit.

1. `260807-2116-eingebauter-editor-mit-textmarken` — beschränkter Abschluss (`_b_`),
   geschlossen am 260810-1445. Der Editor steht als fünfter Fokusbereich, mit Roh- und
   Formatansicht, Zeilensprung, Suchen, Ersetzen und Textmarken; alle 48 Planschritte tragen
   `[DONE]`, und die 53 Defekte der Runde sind abgearbeitet. Beschränkt wegen des
   ausstehenden Abnahmelaufs über die 110 Kriterien des Specs. Der zweite Grund, zwei
   Restdefekte an der Bibliotheksziel-Frage, ist laut Nachtrag vom 260810-1520 entfallen.
2. `260802-0842-krk-mac-dateimanager-editor-git` — beschränkter Abschluss (`_b_`),
   geschlossen am 260807-1035. Das Navigator-Gerüst der Runde 1 steht, alle 38 Planschritte
   tragen `[DONE]` und sind am Code belegt. Beschränkt, weil sieben der zehn Zeitzusagen aus
   C8 unverändert auf einer Messreihe vom 260805 stehen und drei spätere Commits gemessene
   Pfade berührt haben. Der Artefakt der Beschränkung: eine Messreihe altert an jedem Commit,
   der einen gemessenen Pfad berührt, und sie sagt es nicht selbst.

## Archived (_s_ / _d_)

(keine)

Kein Circle-Datensatz trägt den Marker für überholt (`_s_`) oder zurückgestellt (`_d_`).

## Warnings

Zeiger und Marker sind in Ordnung: `.active-circle` nennt
`260809-2040-tastenbelegung-als-markdown-in-downloads`, dessen Datensatz `_t_` trägt, und
genau ein Datensatz trägt diesen Marker. Keine der Lagen `STALE-POINTER`,
`POINTER-MISMATCH`, `MULTIPLE-ACTIVE` oder `MISSING-POINTER` liegt vor.

**Keine Abhängigkeitszyklen.** Der Graph über die vier nicht-terminalen Circles ist
zyklenfrei; die einzige Kante zwischen zwei nicht-terminalen Circles ist
`260811-1257 → 260809-2040`, und es gibt keine Gegenkante.

```
260811-1257 ─┬─> 260809-2040 (_t_) ──> 260802-0842 (_b_)
             └────────────────────────> 260802-0842 (_b_)
260811-1304 ─┬─> 260802-0842 (_b_)
             └─> 260807-2116 (_b_)
260804-0933 ───> 260802-0842 (_b_)
```

**Kein neuer Vermerk `## Parent grounding stale` in diesem Lauf.** Seit dem letzten
Playmaker-Lauf am 260810-1439 ist kein Circle auf `_b_` gewechselt. Die Editor-Runde ist am
260810-1445 geschlossen worden, und der Vermerk dazu steht bereits im Datensatz der Runde 3.
Die zwei am 260811 angelegten Circles nennen beide beschränkt abgeschlossenen Runden
zutreffend als abgeschlossen.

Sechs Befunde für den Nutzer, in absteigender Schärfe:

1. **Der Kopf des aktiven Datensatzes widerspricht seinem Marker.**
   `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_t_circle.md` trägt im Kopf
   `**Status:** anticipated`, während der Dateiname den aktiven Zustand nennt. Der Defekt ist
   aufgenommen:
   `shared/issues/260811-0932_*_die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md`.
   Er sitzt im Plugin und nicht in KRK, und aus diesem Projekt heraus ist er nicht behebbar.
   Die zwei Verweisfelder desselben Kopfes sind am 260811 von Hand nachgezogen worden und
   stimmen.

2. **Der `## Turn log` des aktiven Circles ist leer**, obwohl die Runde gelaufen ist und
   mit `e43f21a..caf6375` Arbeit gelandet hat. Der Abschnitt ist der fortgeschriebene Nachweis
   der Turn-Ergebnisse; ohne ihn steht der Verlauf allein in der Sitzungshistorie. Der
   Playmaker schreibt ihn nicht. Er gehört vom Orchestrator beim Abschluss der Runde gefüllt.

3. **Der Aktivierungsvorschlag im Datensatz des Web-Betrachters ist überholt.**
   `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md` trägt seit
   dem 260807-1042 einen Abschnitt `## Activation proposal`, der den Circle als "empfohlenen
   nächsten Kandidaten" bezeichnet. Seither hat der Nutzer zweimal anders entschieden, und
   dieser Lauf reiht ihn auf Rang 3. Der Abschnitt ist anfügend geschrieben und wird nicht
   überschrieben; wer den Datensatz liest, liest also eine Empfehlung von vor vier Tagen ohne
   Hinweis darauf. Derselbe Datensatz bezeichnet in `## Dependencies` die Runde 1 weiterhin
   als "den aktiven Circle", obwohl sie seit dem 260807-1035 beschränkt abgeschlossen ist.

4. **Die Grundlage des Web-Betrachters kennt die Editor-Runde nicht.** Sie stammt vom
   260804 und beschreibt das Vorschaufenster in dem Zustand, den die Runde 1 hinterließ. Die
   Editor-Runde hat diese Fläche danach zu einem von fünf Fokusbereichen gemacht, ihr
   Zeilennummern gegeben und den Editor sie zeitlich verdrängen lassen. Ein Vermerk
   `## Parent grounding stale` ist dafür nicht angefügt, weil der Datensatz die Editor-Runde
   an keiner Stelle zitiert, weder unter ihrem Verzeichnisnamen noch über ihren
   Abschluss-Artefakt, und die Auslösebedingung damit nicht greift. Der Befund bleibt: die
   Klärungsrunde bei der Aktivierung muss die Grundlage neu erheben und nicht fortschreiben.

5. **Plan und Spec der laufenden Runde 3 tragen `_o_`, obwohl die Ausführung steht.** Alle
   gefahrenen Schritte des Plans tragen `[DONE]`, der vierte ist gestrichen, und der Kopf
   nennt weiterhin "Bereit zur Umsetzung". Nach der Regel zur Zustandsführung setzt ein Plan
   ohne offenen Schritt `**Status:** Complete` und wechselt auf `_c_`. Ob der gestrichene
   Schritt diese Regel auslöst, entscheidet der Abschluss der Runde. Der Playmaker ändert
   keine Planungsdatei; der Punkt gehört dem Reconciler oder dem Orchestrator.

6. **Die Erzeugung dieser Datei setzt die Sternform in den Pfadzitaten nicht von selbst.**
   Der Defekt
   `shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`
   bleibt offen; er verlangt zwei Änderungen in `rules/circle-records.md` und
   `agents/playmaker.md` des Plugins. Dieser Lauf hat die Sternform von Hand durchgehalten,
   der nächste ist dazu nicht verpflichtet. Der Defekt ist aus diesem Projekt heraus nicht
   behebbar.

---

**Details und Verweise**

- Regenerierte Datei: `fusion-workbench/portfolio.md`
- Sitzungsbericht dieses Laufs:
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/history/260811-1326-playmaker-direct-dispatch.md`
- Angefügter Aktivierungsvorschlag:
  `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/_a_circle.md`, Abschnitt
  `## Activation proposal`
- Der Playmaker benennt Kandidaten und aktiviert sie nicht. Die Umbenennung des Datensatzes
  von `_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim
  Nutzer über `/fusion:next` oder beim Orchestrator.
