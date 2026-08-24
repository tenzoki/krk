# Portfolio

**Generated:** 260823-2241 (by playmaker session 260823-2241)
**Domain bias:** code

Bestand: 1 vorgesehen, 0 aktiv, 5 kohärent geschlossen, 10 beschränkt geschlossen,
0 überholt, 2 zurückgestellt. Summe 18 Circle-Datensätze.

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` fehlt, und kein Datensatz trägt den Marker `_t_`. Das ist
der reguläre Zustand nach einem Abschluss, keine Zeigerwarnung. Die letzte Runde,
`260821-1644-veroeffentlichen-als-achte-station`, ist am 260821-2110 kohärent geschlossen
worden.

## Anticipated (_a_) — ranked

Recommended next: `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` — keine
Abhängigkeit, kein offener fremder Entscheidungsdatensatz in der Grundlage, die zwei offenen
Fragen sind Fragen der Runde selbst.

**1. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`** (`_a_`, Domäne
`code`, angelegt am 260823-2208 vom Shaper)

Directive: Das Vorschaufenster beantwortet für erkannte Orte die Frage, was dort liegt, ohne
dass der Nutzer den Ort betritt. Eine von Hand gepflegte `readers.toml` im Bestandsort trägt
Profile, ein Profil erkennt seinen Ort über Pfadmuster oder Kennzeichendatei und beschreibt
aus einem festen Bausteinsatz die Zusammenfassung; greift kein Profil, bleibt die heutige
Metadatenanzeige stehen.

Der Circle ist der einzige Kandidat und zugleich der am besten vorbereitete, den das
Projekt seit dem 260821 hatte. `## Dependencies` ist leer, es wartet also keine Vorbedingung
auf einen Abschluss. Die Grundlage zitiert keinen einzigen offenen Entscheidungsdatensatz
aus einem fremden Speicher: von den vier Verweisen nach draußen führt einer auf den Spec der
Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`),
einer auf einen umgesetzten Datensatz derselben Runde
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`),
einer auf einen beantworteten im gemeinsamen Speicher
(`shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`)
und einer auf den Rückstandseintrag, aus dem die Runde stammt. Offen sind allein die zwei
Fragen, die der Shaper in `decisions/` dieses Circles abgelegt hat: ob ein Profil nur für
Ordner oder auch für einzelne Dateien gilt, und ob KRK ein fertiges fusion-workbench-Profil
mitliefert. Beide gehören in die Klärung der Runde und nicht vor ihre Aktivierung. Zwei
Punkte hat der Plan zu tragen: `readers.toml` wird die siebte Datei einer Aufzählung ohne
Auffangzweig im Bestandsort, was der Übersetzer einfordert, und die Zusammenfassung fällt in
die Endbedingung der Zeitzusage L7, die seit dem 260819-2242 ohnehin auf den Gegenständen
der späteren Messrunde steht.

Der Aktivierungsvorschlag steht im Datensatz selbst, Abschnitt `## Activation proposal`.

## Backlog — ranked

Recommended to split first: `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`
— 2 Ideen, die obere ist `leseoperationen-je-erkanntem-ort`.

**1. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`** (`_o_`,
eingereicht 260823-2136)

Profile in einer Definitionsdatei vereinfachen den Zugriff auf Ordner und Dateien: welche
Orte welche Leseoperationen erfordern, und was im Vorschaufenster erscheint. Der Eintrag
nennt diese zwei Hälften ausdrücklich, und eine davon ist bereits ein Circle. Der Shaper hat
den Eintrag am 260823-2208 offen gelassen, weil eine Schließung die erste Hälfte ungelesen
mitnähme. Genau diese Lage löst ein Teilen auf.

- Vorgeschlagene Teilung: `leseoperationen-je-erkanntem-ort` — Ein Profil sagt, welche
  Leseoperationen ein erkannter Ort erfordert; `profil-zusammenfassung-im-vorschaufenster` —
  Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung. Die zweite Hälfte
  ist mit Circle `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` bereits
  vergeben; der aus ihr entstehende Eintrag ist danach gegen diesen Circle schließbar, was
  ein späterer Lauf vorlegt.

**2. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`, eingereicht 260813-2033, Empfehlung in diesem Lauf zurückgenommen)

Der Eintrag verlangt eine zweite, besser erreichbare Kombination neben `F4` für den Einstieg
in den Editor. Die Idee ist gebaut: seit dem 260823 öffnet `cmd+e` im Dateifenster denselben
ausgewählten Eintrag wie `f4` und läuft durch denselben Rumpf
(`shared/history/260823-1010-coder-cmd-e-wird-der-rundweg-zwischen-dateiliste-und-editor.md`;
`resources/default-keymap.toml`, Eintrag `bearbeiten`). Auch die Auflage des Eintrags ist
erfüllt: der Kommentar an `bearbeiten`, der ein Cmd-Kürzel ausdrücklich für entbehrlich
erklärte, ist ersetzt und nicht übergangen worden.

- Vorgeschlagene Schließung: der Eintrag ist erledigt, ohne dass ihn jemand als Circle
  gefahren hätte.

Vorgeschlagen und nicht durchgeführt, weil dieser Lauf keine Bestätigung des Nutzers hält:

```
split shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md into: leseoperationen-je-erkanntem-ort — Ein Profil sagt, welche Leseoperationen ein erkannter Ort erfordert; profil-zusammenfassung-im-vorschaufenster — Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung
close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e öffnet seit dem 260823 im Dateifenster denselben Eintrag wie f4, durch denselben Rumpf; die Idee ist gebaut
```

Performed this run:

```
Marker 260813-2033_p_ → 260813-2033_o_ (Empfehlung zurückgenommen, die Idee ist gebaut)
```

## Recently closed (_c_ / _b_)

1. `260821-1644-veroeffentlichen-als-achte-station` (`_c_`) — Veröffentlichen als achte
   Station der Auslieferungskette; am 260821-2024 ist `KRK 0.5.6` über sie ausgeliefert und
   vom Nutzer am laufenden Dienst abgenommen worden.
2. `260819-2230-auswahl-und-kopieren-in-der-vorschau` (`_c_`) — Die Vorschaufläche ist
   auswählbar, kopiert wird der Quelltext; abgenommen am gebauten Bündel 0.5.4.
3. `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` (`_c_`) — `opt+cmd+s` gleicht
   die Ordner der zwei Dateifenster an, und eine Dateiliste nimmt Abwürfe aus fremden
   Anwendungen entgegen; zehn Prüfungen am Bündel 0.5.2 abgenommen.
4. `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` (`_c_`) — Genau ein
   Löschweg, er führt in den Papierkorb, und jeder Vorgang fragt einmal nach.
5. `260816-1321-inhaltsfilter-mit-ankreuzfeld-content` (`_b_`) — Der Filter berücksichtigt
   den Inhalt; im Baum erreicht, aber nicht am laufenden Bündel abgenommen, weil der
   Abnahmelauf KRK im Vordergrund verlangt und Nutzerarbeit ist.

**Der Marker unterscheidet hier die Verfügbarkeit des Nutzers und nicht die Reife der
Runde.** Zehn der fünfzehn gefahrenen Runden sind beschränkt geschlossen, und stets aus
demselben Grund: der Abnahmelauf verlangt KRK im Vordergrund, und kein Agent kann ihn
fahren. Eine Rangheuristik, die allein `_c_` als erfüllte Vorbedingung zählt, gibt in diesem
Projekt eine irreführende Auskunft.

## Archived (_s_ / _d_)

- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, zurückgestellt am 260817-0445)
  — Befehle absetzen und gespeicherte Makros ausführen. Nichts gebaut; Spec mit 54
  Abnahmekriterien und Plan mit 22 Schritten liegen vollständig vor. Die Runde ist nicht
  gescheitert, sie war nur nicht dran.
- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`, zurückgestellt am
  260821-2202) — Eingebauter Web-Betrachter. **Fallengelassen, nicht aufgeschoben.** Der
  Nutzer hat entschieden, dass das Abgeben an den Systembrowser genügt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`,
  Möglichkeit 2). Das Vokabular kennt für eine Absage keinen eigenen Marker; wer `_d_` hier
  als „später" liest, liest es falsch.

## Warnings

- `CLAUDE.md` behauptet, es gebe keinen vorgesehenen Circle: „Seit dem 260821-2202 gibt das
  Kommando nichts aus." Seit dem 260823-2208 gibt `ls fusion-workbench/circles/*/_a_circle.md`
  einen Datensatz aus. Die Aussage ist falsch, und sie steht in dem Abschnitt, der
  ausdrücklich den Dateibestand für verbindlich erklärt. Der Playmaker schreibt keine
  Defektdatensätze; ob das ein Befund für `issues/` oder eine Sache für den nächsten
  Kurations-Lauf ist, entscheidet der Nutzer.
- Der Circle-Datensatz `260816-2255-befehle-absetzen-und-makros-speichern` trägt unter
  `## Closure note` die Platzhalterzeile `(offen)` **über** der ausgeschriebenen Notiz. Ein
  Leser, der nur die erste Zeile des Abschnitts nimmt, hält die Notiz für ungeschrieben.
- Keine Abhängigkeitszyklen. Der einzige nicht-terminale Circle führt
  `## Dependencies` = „(keine)"; der Graph hat eine Ecke und keine Kante.
- Keine veraltete Elterngrundlage. Der vorgesehene Circle zitiert Artefakte aus dem
  beschränkt geschlossenen `260802-0842-krk-mac-dateimanager-editor-git`, aber seine
  Grundlage ist am 260823 gegen den heutigen Baum erhoben worden, also nach jenem Abschluss
  vom 260807. Kein Circle ist in diesem Lauf nach `_b_` gewechselt.
- Kein Zeigerbefund: `.active-circle` fehlt, und kein Datensatz trägt `_t_`.
