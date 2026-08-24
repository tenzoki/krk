# Portfolio

**Generated:** 260824-2017 (by playmaker session 260824-2017)
**Domain bias:** code

Bestand: 0 vorgesehen, 0 aktiv, 5 kohärent geschlossen, 11 beschränkt geschlossen,
0 überholt, 2 zurückgestellt. Summe 18 Circle-Datensätze.

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` fehlt, und kein Datensatz trägt den Marker `_t_`. Das ist
der reguläre Zustand nach einem Abschluss und keine Zeigerwarnung. Die Runde 16,
`260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`, ist am 260824-1810
beschränkt geschlossen worden.

## Anticipated (_a_) — ranked

(keiner)

**Das Portfolio hat nichts zu rangieren, und das ist eine leere Vorbereitung und kein
Stillstand.** `ls fusion-workbench/circles/*/_a_circle.md` gibt seit dem Abschluss der Runde
16 nichts aus. Die beiden Wege dorthin liegen sechzehn Tage auseinander: der Web-Betrachter
im Vorschaufenster ist am 260821-2202 vom Nutzer abgesagt worden, und der einzige danach
angelegte vorgesehene Circle ist am 260824 aktiviert und noch am selben Tag geschlossen
worden. Kein Kandidat wartet auf eine Vorbedingung, keiner ist übergangen worden.

**Was der Nutzer als Nächstes vorhat, trägt kein Datensatz.** Auf einen Neustart und ein
Update soll eine ausgedehnte Durchsichts- und Behebungsrunde folgen. Sie ist heute weder ein
Circle noch ein Rückstandseintrag, also kann das Portfolio sie weder rangieren noch zur
Aktivierung vorschlagen. Zwei Wege machen sie sichtbar: `/fusion:direct <Entwurf>` legt aus
einem Satz unmittelbar einen vorgesehenen Circle an, `/fusion:memo` legt sie zuerst als
Rückstandseintrag ab, wenn die Runde vor dem Zuschnitt noch reifen soll. Der Playmaker legt
selbst keinen Eintrag an; das Einreichen ist die Sache des Nutzers.

**Der Bestand für eine solche Runde liegt bereit, gezählt am 260824-2017.** Offene
Defektdatensätze: 170, davon 56 im gemeinsamen Speicher und 114 in den Speichern der
sechzehn gefahrenen Runden. Offene Entscheidungsdatensätze: 35, davon 14 gemeinsam und 21 in
Circle-Speichern. Gezählt mit `find shared/issues circles/*/issues -maxdepth 1 -name
'*_o_*.md'` und dem gleichlautenden Aufruf über die Entscheidungsspeicher; `archive/` ist
darin nicht enthalten und gehört auch nicht hinein. Sechs der offenen Defekte stammen aus
der eben geschlossenen Runde. Zwei ihrer Befunde betreffen die Sitzungsmechanik und nicht
KRK und liegen deshalb im gemeinsamen Speicher: der geteilte git-Index
(`shared/issues/260824-1745_*_ein-commit-des-orchestrators-nimmt-die-git-mv-umbenennungen-eines-laufenden-agenten-mit.md`)
und Zeitstempel in Dateinamen, die der Uhr um bis zu 208 Minuten vorausliefen
(`shared/issues/260824-1758_*_die-zeitstempel-in-dateinamen-laufen-der-uhr-voraus-bis-zu-drei-stunden.md`).
Eine Durchsichtsrunde, die beide mitnimmt, räumt an ihrem eigenen Werkzeug und nicht an KRK.

## Backlog — ranked

Recommended to split first: `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`
— 2 Ideen, die obere ist `leseoperationen-je-erkanntem-ort`.

**1. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`** (`_o_`,
eingereicht 260823-2136)

Profile in einer Definitionsdatei vereinfachen den Zugriff auf Ordner und Dateien: welche
Orte welche Leseoperationen erfordern, und was im Vorschaufenster erscheint. Der Eintrag
nennt diese zwei Hälften ausdrücklich. **Die zweite ist seit dem 260824-1810 gebaut**, die
erste ist unberührt. Der Shaper hat den Eintrag am 260823-2208 offen gelassen, weil eine
Schließung die erste Hälfte ungelesen mitgenommen hätte; mit dem Abschluss der Runde ist
diese Lage nicht besser geworden, sondern eindeutiger. Ein `/fusion:direct` auf diesen
Eintrag machte heute einen Circle aus einer erledigten und einer offenen Hälfte, deshalb
steht keine Aufrufzeile darunter.

Die verbliebene Hälfte steht besser da als beim letzten Lauf: sie rechnet nicht mehr gegen
eine Absicht, sondern gegen eine gebaute Maschinerie. `readers.toml` liegt als siebte
Ablagedatei im Bestandsort, die Erkennung eines Ortes über Pfadmuster und Kennzeichendatei
läuft, und fünf Profile werden mitgeliefert. Was „welche Leseoperationen ein erkannter Ort
erfordert" heißen soll, ist damit gegen etwas Vorhandenes zu klären statt gegen eine Skizze.

- Vorgeschlagene Teilung: `leseoperationen-je-erkanntem-ort` — Ein Profil sagt, welche
  Leseoperationen ein erkannter Ort erfordert; `profil-zusammenfassung-im-vorschaufenster` —
  Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung. Der zweite
  Eintrag ist unmittelbar nach seiner Entstehung gegen den Circle
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` schließbar. Die
  Schließung legt ein späterer Lauf vor, weil sie einen Pfad nennen muss, den erst die
  Teilung erzeugt.

**2. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`, eingereicht 260813-2033)

Der Eintrag verlangt eine zweite, besser erreichbare Kombination neben `F4` für den Einstieg
in den Editor. Die Idee ist gebaut, ohne dass sie je ein Circle war. Im Dateifenster öffnen
`f4` und `cmd+e` denselben ausgewählten Eintrag und laufen durch denselben Rumpf; nachgelesen
am 260824-2017 in `resources/default-keymap.toml`, Kommentar am Eintrag `bearbeiten`, Zeilen
174 bis 177. Auch die Auflage des Eintrags ist erfüllt: der Kommentar, der ein Cmd-Kürzel
ausdrücklich für entbehrlich erklärte, ist ersetzt und nicht übergangen worden.

- Vorgeschlagene Schließung: der Eintrag ist erledigt.

**Kein Marker ist in diesem Lauf bewegt worden, und beide Einträge stehen weiter auf `_o_`.**
Der Rang `_p_` empfiehlt einen Eintrag zur Formung als Circle. Der erste ist zur Teilung
empfohlen und nicht zur Formung, der zweite ist gebaut; für keinen von beiden wäre eine
Empfehlung zur Formung richtig.

Vorgeschlagen und nicht durchgeführt, weil dieser Lauf keine Bestätigung des Nutzers hält.
Ein nicht-interaktiver Phase-4-Auftrag trägt keinen Block `**Confirmed operations:**`, und
der Lauf hat als untergeordneter Agent keinen Kanal zum Nutzer:

```
split shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md into: leseoperationen-je-erkanntem-ort — Ein Profil sagt, welche Leseoperationen ein erkannter Ort erfordert; profil-zusammenfassung-im-vorschaufenster — Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung
close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — im Dateifenster öffnen f4 und cmd+e denselben Eintrag durch denselben Rumpf; die Idee ist gebaut
```

Beide standen wortgleich schon im Portfolio vom 260823-2241 und sind seither nicht
ausgeführt worden. Wer sie ausführen will, ruft `/fusion:next` auf: dort steht der Kanal zum
Nutzer offen, über den eine Bestätigung entstehen kann.

## Recently closed (_c_ / _b_)

1. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (`_b_`, 260824-1810) —
   Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung statt der
   Metadaten. Gebaut ist, was die Directive verlangt, über vierzehn Planschritte. Beschränkt
   geschlossen, weil sieben Abnahmekriterien KRK im Vordergrund verlangen. Zwei Reste binden
   weiter: vier Abnahmekriterien stimmen am Baum, ohne dass eine Probe sie hält
   (`issues/260824-1852_*`), und wie die Arbeit dieser Runde je gegen die Zeitzusage L7
   gemessen wird, ist offen
   (`decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`).
2. `260821-1644-veroeffentlichen-als-achte-station` (`_c_`) — Veröffentlichen als achte
   Station der Auslieferungskette; am 260821-2024 ist `KRK 0.5.6` über sie ausgeliefert und
   vom Nutzer am laufenden Dienst abgenommen worden.
3. `260819-2230-auswahl-und-kopieren-in-der-vorschau` (`_c_`) — Die Vorschaufläche ist
   auswählbar, kopiert wird der Quelltext; abgenommen am gebauten Bündel 0.5.4.
4. `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` (`_c_`) — `opt+cmd+s` gleicht
   die Ordner der zwei Dateifenster an, und eine Dateiliste nimmt Abwürfe aus fremden
   Anwendungen entgegen; zehn Prüfungen am Bündel 0.5.2 abgenommen.
5. `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` (`_c_`) — Genau ein
   Löschweg, er führt in den Papierkorb, und jeder Vorgang fragt einmal nach.

**Der Marker unterscheidet hier die Verfügbarkeit des Nutzers und nicht die Reife der
Runde.** Elf der sechzehn gefahrenen Runden sind beschränkt geschlossen, und stets aus
demselben Grund: der Abnahmelauf verlangt KRK im Vordergrund, und kein Agent kann ihn fahren.
Eine Rangheuristik, die allein `_c_` als erfüllte Vorbedingung zählt, gibt in diesem Projekt
eine irreführende Auskunft, und dieser Lauf legt keine an.

## Archived (_s_ / _d_)

- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, zurückgestellt am 260817-0445)
  — Befehle absetzen und gespeicherte Makros ausführen. Nichts gebaut; Spec mit 54
  Abnahmekriterien und Plan mit 22 Schritten liegen vollständig vor. Die Runde ist nicht
  gescheitert, sie war nur nicht dran. Für eine Durchsichtsrunde ist sie damit der einzige
  Bestand im Portfolio, der ohne neue Klärung wieder aufgenommen werden könnte; der Weg
  dorthin ist ein neuer Circle, der sie über `## Dependencies` zitiert, denn `_d_` ist
  terminal.
- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`, zurückgestellt am
  260821-2202) — Eingebauter Web-Betrachter. **Fallengelassen, nicht aufgeschoben.** Der
  Nutzer hat entschieden, dass das Abgeben an den Systembrowser genügt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`,
  Möglichkeit 2). Das Vokabular kennt für eine Absage keinen eigenen Marker; wer `_d_` hier
  als „später" liest, liest es falsch.

## Warnings

- `CLAUDE.md` sagt über die vorgesehenen Circles: „Seit dem 260821-2202 gibt das Kommando
  nichts aus. Der Web-Betrachter im Vorschaufenster (`260804-0933-…`) war der letzte
  vorgesehene." Der erste Satz stimmt für heute wieder und für die Zeit dazwischen nicht: vom
  260823-2208 bis zum 260824 gab `ls fusion-workbench/circles/*/_a_circle.md` einen Datensatz
  aus. Der zweite Satz ist falsch, denn der letzte vorgesehene Circle war
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`.
- `CLAUDE.md` führt in seiner Rundentabelle fünfzehn Runden; der Dateibestand trägt sechzehn
  geschlossene. Die Runde 16 fehlt in der Tabelle. Die Tabelle nennt sich selbst ein
  Verweisregister für die Pfadregel im Absatz darunter, also fehlt mit der Zeile auch der
  Bezugspunkt für Pfade dieser Runde.
- Beide Stellen sind Arbeit des Kurators, `/fusion:cleanup --only claude-md`. Der Playmaker
  schreibt keine Defektdatensätze. Zwei weitere Aussagen in `CLAUDE.md`, die mit der Runde 16
  falsch geworden sind, trägt bereits
  `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1852_*_zwei-aussagen-in-claude-md-sind-mit-dieser-runde-falsch-geworden-und-kein-datensatz-traegt-sie.md`.
- Der Circle-Datensatz `260816-2255-befehle-absetzen-und-makros-speichern` trägt unter
  `## Closure note` die Platzhalterzeile `(offen)` **über** der ausgeschriebenen Notiz. Ein
  Leser, der nur die erste Zeile des Abschnitts nimmt, hält die Notiz für ungeschrieben. Der
  Befund steht seit dem Lauf vom 260823-2241 unverändert.
- Keine Abhängigkeitszyklen. Der Graph der nicht-terminalen Circles hat in diesem Lauf weder
  Ecke noch Kante, weil kein Datensatz `_a_` oder `_t_` trägt.
- Keine veraltete Elterngrundlage. Die Runde 16 ist nach `_b_` gewechselt, und die Prüfung
  auf Elternteile, deren `## Grounding snapshot` sie zitiert, läuft über dieselbe leere Menge
  nicht-terminaler Circles. Kein Abschnitt `## Parent grounding stale` geschrieben.
- Kein Zeigerbefund: `.active-circle` fehlt, und kein Datensatz trägt `_t_`.
