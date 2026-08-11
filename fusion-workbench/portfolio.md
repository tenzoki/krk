# Portfolio

**Generated:** 260811-1415 (by playmaker session 260811-1415-playmaker-direct-dispatch)
**Domain bias:** code

Sechs Circles liegen unter `circles/`: keiner aktiv, drei vorgesehen, drei beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Die Runde 3 ist am 260811-1415
geschlossen, und damit steht das Projekt zum ersten Mal seit dem 260802 ohne laufende Runde da.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers
eine Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate
zwischen zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die
Aussage ist. Der Defekt dazu ist
`shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`;
er sitzt in der Anweisung des Plugins und bleibt offen.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden.
Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: die Runde 3
ist am 260811-1415 als beschränkter Abschluss geschlossen worden, und der Orchestrator löscht
den Zeiger bei dieser Umbenennung.

Der nächste Schritt liegt beim Nutzer. Er wählt über `/fusion:next`, welcher der drei
vorgesehenen Circles aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` — kleinster Zuschnitt,
Grundlage am Baum erhoben, keine unbeantwortete technische Größe, und die Bedingung des letzten
Vorschlags ist mit dem Abschluss der Runde 3 erfüllt.

Alle drei vorgesehenen Circles hängen ausschließlich an beschränkt abgeschlossenen Runden.
Nach der Rangheuristik zählt allein ein kohärenter Abschluss (`_c_`) als erfüllte Vorbedingung,
also tragen alle drei dasselbe Kennzeichen, und es unterscheidet sie nicht. Die Rangfolge
entsteht deshalb aus den übrigen Signalen.

### Rang 1: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`

**Vier Tastenbefehle: Pfade kopieren, mit dem Standardprogramm öffnen, Cmd+W überall.**
Angelegt am 260811-1257, Domain `code`.

KRK legt auf Tastendruck zwei Sorten von Pfaden in die Zwischenablage, den des angezeigten
Ordners und den des betroffenen Eintrags. Eine Datei geht per Doppelklick und per
Tastenkombination an das Standardprogramm des Systems, und Cmd+W schließt den aktiven Tab auch
dann, wenn der Fokus außerhalb eines Bereichs mit Tabs steht. Die Zwischenablage wird damit zum
ersten Mal auch Ziel und nicht mehr nur Quelle.

Dieser Circle steht auf Rang 1, weil er als einziger der drei keine unbeantwortete technische
Größe trägt. Seine Grundlage ist am 260811-1257 am Baum erhoben, mit einem Zeilenverweis auf
jede tragende Feststellung, und der Bau ist an vier Stellen abzählbar: vier Zeilen in
`resources/default-keymap.toml`, vier Werte in der Aufzählung `Kommando` und je vier Zeilen in
`Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs`) und
`bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs`). Keine dieser vier
Fallunterscheidungen hat einen Auffangzweig, der Übersetzer nennt die Stellen also von selbst.
Die Bauteile, die der Circle erbt, liegen auf der Platte: die eine Hülle um `NSPasteboard` in
`crates/krk-ui/src/appkit/zwischenablage.rs`, die Regel "worauf wirkt dieser Befehl" in
`betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:157`), die aktive Fensterseite in
`Fenstermodell::aktiv()` (`crates/krk-ui/src/fenstermodell.rs:318`) und `NSWorkspace`, das über
drei Module schon im Haus ist. Vier offene Entscheidungsdatensätze liegen in seinem eigenen
`decisions/`, und alle vier sind Zuschnittfragen an den Nutzer, keine Untersuchungen: die
Reichweite von Cmd+W, was der Pfadkopierer bei stehender Markierung nimmt, was ein Doppelklick
auf einen Ordner tut und welche vier Kombinationen ab Werk gelten. Für die Gewichtung `code`
ist ein Zählwert von vier offenen Fragen kein guter Wert, und der Playmaker unterschlägt es
nicht: der Web-Betrachter auf Rang 3 zitiert nur einen einzigen. Der Zählwert misst hier die
falsche Größe, weil der eine Datensatz dort eine ungemessene technische Frage ist und die vier
hier in einer Klärungsrunde beantwortbar sind. Neu seit dem letzten Lauf: eine offene Frage der
Runde 1 bindet die zweite dieser vier Fragen. Die Markierung fällt heute mit jedem Lesevorgang,
weil sie eine Menge von Eintragsindizes ist, während die Auswahl über den Namen getragen wird
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md`).
Der Pfadkopierer setzt auf derselben Markierung auf.

- **Abhängigkeiten:** `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`) und
  `260809-2040-tastenbelegung-als-markdown-in-downloads` (`_b_`). Beide beschränkt, keine
  kohärent; Kennzeichen gesetzt, inhaltlich beide schwach.
- **Offene Entscheidungen im eigenen Speicher:** vier, alle `_o_`, alle Zuschnittfragen.
- **Angefügt in diesem Lauf:** `## Parent grounding stale` und `## Activation proposal`.
- **Zu beachten:** die Runde bricht eine schriftliche Zusicherung. Der Modulkopf von
  `crates/krk-ui/src/appkit/zwischenablage.rs` sagt zu, dass KRK die Zwischenablage in keinem
  Fall schreibt; die beiden Kopierbefehle brechen genau das, also gehört der Modulkopf mit
  derselben Änderung umgeschrieben.

### Rang 2: `260811-1304-statusleiste-mit-bereichsschaltern`

**KRK trägt eine Statusleiste mit Schaltern für die fünf Bereiche.** Angelegt am 260811-1304,
Domain `code`.

Eine Leiste am unteren Fensterrand führt für jeden der fünf Bereiche der Fensterzeile einen
Schalter, zeigt an, ob sein Bereich steht, und schaltet ihn per Maus oder Tastatur um. Jede
Änderung der Sichtbarkeit teilt die Fensterzeile proportional zur zuletzt sichtbaren Aufteilung
neu auf, und der gemeldete Rückfall der Vorschaubreite ist mit dieser Runde behoben.

Rang 2 statt Rang 1, weil der Zuschnitt größer und noch nicht entschieden ist. Der zentrale
Befund der Grundlage sagt das selbst: die heutige Breitenregel `bereichsbreiten`
(`crates/krk-ui/src/fenstermodell.rs:609`) ist nicht proportional, sie gibt den festen
Bereichen absolute Punktzahlen und ein Verhältnis allein den beiden Dateifenstern. Der Entwurf
verlangt damit eine neue Fassung der einen Breitenregel und keine Ergänzung daneben, und was
aus der Vorrangordnung vom 260808 wird, ist die erste seiner sieben offenen Fragen. Rang 2 vor
Rang 3 steht trotz der sieben, weil alle sieben Zuschnittfragen mit benannten Möglichkeiten
sind und die Grundlage am 260811-1304 am Baum erhoben wurde. Ein Argument für einen früheren
Zugriff steht daneben und ist nicht unterschlagen: dieser Circle trägt als einziger der drei
einen laufenden, vom Nutzer selbst gemeldeten Defekt
(`shared/issues/260811-1245_*_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`),
und seine siebte Frage ist genau die, ob dieser Defekt in der Runde oder vor ihr behoben wird.

- **Abhängigkeiten:** `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`, über C7, C1 und C8)
  und `260807-2116-eingebauter-editor-mit-textmarken` (`_b_`, über C1 und die fünf Bereiche).
  Der Web-Betrachter bindet ihn ausdrücklich nicht.
- **Offene Entscheidungen im eigenen Speicher:** sieben, alle `_o_`.
- **Angefügt in diesem Lauf:** nichts.

### Rang 3: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**KRK zeigt Web-Seiten in einem eigenen Betrachter.** Angelegt am 260804-0933, Domain `code`.

Eine Web-Adresse erscheint in einem gewöhnlichen Tab des Vorschaufensters statt im
Systembrowser. Bedient wird der Betrachter über die Tastatur, mit Sprungmarken auf jedem
sichtbaren Link. Kein Verlauf, kein dauerhaftes Adressfeld, kein Herunterladen.

Rang 3 trotz des besten Zählwerts bei den offenen Entscheidungen. Er zitiert genau einen
offenen Datensatz, und der ist das Problem: die Verfügbarkeitsprüfung für
macOS-26-Schnittstellen in `objc2`
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
ist eine ungemessene technische Frage, und der Circle hält daneben selbst fest, dass auch das
Mittel der Darstellung von Web-Inhalt offen ist und "in eine eigene Untersuchung vor dem Plan"
gehört. Dazu kommt das Alter seiner Grundlage. Sie stammt vom 260804 und kennt weder die
Editor-Runde noch die Belegungs-Runde; sein Abschnitt `## Dependencies` nennt die Runde 1
weiterhin "den aktiven Circle", und sein Aktivierungsvorschlag vom 260807-1042 bezeichnet ihn
weiterhin als empfohlenen nächsten Kandidaten. Beides steht unten unter den Warnungen. Der
Circle bleibt inhaltlich tragfähig; er verlangt vor einem Plan mehr Vorarbeit als die beiden
anderen.

- **Abhängigkeiten:** `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`). Trägt seit dem
  260807-1042 einen Abschnitt `## Parent grounding stale` zu dieser Runde.
- **Offene Entscheidungen im eigenen Speicher:** keine; ein zitierter Datensatz der Runde 1.
- **Angefügt in diesem Lauf:** nichts.

## Recently closed (_c_ / _b_)

Drei geschlossene Circles, keiner davon kohärent. Alle drei sind aus demselben Grund
beschränkt: der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit, die kein Agent
leisten kann
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).

| Circle | Marker | Geschlossen | Abschlussnotiz in einem Satz |
|---|---|---|---|
| `260809-2040-tastenbelegung-als-markdown-in-downloads` | `_b_` | 260811-1415 | KRK schreibt die Belegung als Markdown in den Downloads-Ordner; die drei gefahrenen Planschritte tragen `[DONE]`, der Abnahmeschritt S4 ist vom Nutzer gestrichen, und damit stehen alle 41 Abnahmekriterien offen. |
| `260807-2116-eingebauter-editor-mit-textmarken` | `_b_` | 260810-1445 | Der Editor steht als fünfter Fokusbereich mit Roh- und Formatansicht, Zeilensprung, Suchen, Ersetzen und Textmarken; alle 48 Planschritte tragen `[DONE]`, der Abnahmelauf über 110 Kriterien steht aus. |
| `260802-0842-krk-mac-dateimanager-editor-git` | `_b_` | 260807-1035 | Das Navigator-Gerüst der Runde 1 steht mit allen 38 Planschritten auf `[DONE]`; sieben der zehn Zeitzusagen stehen auf einer Messreihe, die drei spätere Commits haben altern lassen. |

**Die drei Abschluss-Artefakte in je einem Satz**, weil sie die spätere Arbeit binden:

- Runde 3: eine Zusicherung stand dreimal in dieser Sitzung im Text stärker da als im Code, und
  jedes Mal hat erst die Durchsicht sie zurückgezogen. Der Spec hat für diese Fehlerform eine
  Gewohnheit, `inference:` kennzeichnen und die Prüfung zum Kriterium machen, aber keinen
  Mechanismus.
- Runde 2: ein stehendes Blatt hält Tastenbefehle beim Anwendungsdelegierten an und nicht über
  den Fokusvorbehalt; die andere Lesart hat einen Fehlbefund erzeugt.
- Runde 1: eine Messreihe altert an jedem Commit, der einen gemessenen Pfad berührt, und sie
  sagt es nicht selbst.

## Archived (_s_ / _d_)

**(keiner)** — kein Circle-Datensatz trägt `_s_` überholt oder `_d_` zurückgestellt.

Dieser Abschnitt führt lebende Circle-Datensätze unter `circles/` mit diesen beiden Markern.
Er hat nichts mit dem Speicher `archive/` zu tun; dessen Inhalt erscheint im Portfolio nicht.

## Warnings

Keine Zeigerlage und kein Zyklus. Sechs Befunde stehen an, fünf davon aus früheren Läufen
unverändert.

- **Kein `STALE-POINTER`, kein `POINTER-MISMATCH`, kein `MULTIPLE-ACTIVE`, kein
  `MISSING-POINTER`.** `fusion-workbench/.active-circle` ist nicht vorhanden, und kein
  Circle-Datensatz trägt `_t_`. Beides zusammen ist der reguläre Zustand nach einem Abschluss.
- **Kein `dependency-cycle-detected`.** Der gerichtete Graph über die drei nicht-terminalen
  Circles hat keine einzige Kante zwischen zwei nicht-terminalen Knoten mehr: alle Kanten enden
  auf beschränkt abgeschlossenen Runden. Ein Zyklus ist damit ausgeschlossen. Kein Abschnitt
  `## Dependency warning` angefügt.
- **Warnung 1 — der Kopf des Datensatzes der Runde 3 widerspricht seinem Marker.**
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_b_circle.md` trägt in seinem
  Kopf `**Status:** anticipated`, während der Dateiname `_b_` sagt. Der Datensatz hat seit dem
  260809 zwei Übergänge durchlaufen, vorgesehen auf aktiv und aktiv auf beschränkt
  abgeschlossen, und keiner der beiden hat das Kopffeld nachgezogen. Die Runden 1 und 2 tragen
  an derselben Stelle korrekt `bounded`, der Fehler ist also nicht systematisch, sondern in
  dieser Runde zweimal durchgerutscht. Aufgenommen als
  `shared/issues/260811-0932_*_die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md`,
  weiterhin offen.
- **Warnung 2 — der Aktivierungsvorschlag im Datensatz des Web-Betrachters ist überholt.** Der
  Abschnitt `## Activation proposal` vom 260807-1042 in
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md` bezeichnet
  diesen Circle als empfohlenen nächsten Kandidaten. Zwei spätere Läufe haben ihn auf Rang 3
  gesetzt. Weil der Playmaker anfügt und nicht umschreibt, steht der alte Vorschlag ohne
  Widerspruch daneben; wer nur ihn liest, liest das Gegenteil der heutigen Empfehlung.
- **Warnung 3 — die Grundlage des Web-Betrachters kennt zwei Runden nicht.** Sein Abschnitt
  `## Dependencies` nennt `260802-0842-krk-mac-dateimanager-editor-git` "den aktiven Circle";
  jene Runde ist seit dem 260807-1035 geschlossen. Seine Grundlage beschreibt das
  Vorschaufenster in dem Zustand, den die Runde 1 hinterließ, und kennt weder den Umbau durch
  die Editor-Runde noch die Belegungs-Runde. Die Auslösebedingung für einen Abschnitt
  `## Parent grounding stale` greift für beide Runden nicht: der Datensatz zitiert sie an keiner
  Stelle. Der Befund gehört in die Klärungsrunde bei seiner Aktivierung.
- **Warnung 4 — der Spec der Runde 3 bleibt auf `_o_`, und das ist so gewollt.** Die 41
  Abnahmekriterien in
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md`
  stehen sämtlich auf `- [ ]`, ebenso die Abnahmeanleitung
  `.../planning/260811-1130_*_abnahmeanleitung-tastenbelegung-als-markdown.md`. Der Plan steht
  korrekt auf `_c_`. Der Zustand ist kein Versehen, sondern der Grund der Beschränkung; er steht
  hier, damit ein späterer Lauf ihn nicht als Nachlässigkeit liest. Dieselbe Lage trägt die
  Runde 2 mit ihrem Spec
  `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md`
  und ihren 110 Kriterien.
- **Warnung 5 — die Erzeugung dieser Datei setzt die Sternform nicht von selbst.** Die
  Portfolio-Vorlage in `rules/circle-records.md` schweigt zur Zitierform, und das
  Musterbeispiel in `agents/playmaker.md` führt einen ausgeschriebenen Marker vor. Dieser Lauf
  hat die Sternform von Hand durchgehalten. Der Defekt
  `shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`
  bleibt offen; er ist aus diesem Projekt heraus nicht behebbar, weil er in der installierten
  Kopie des Plugins sitzt.
- **Warnung 6 — sieben Defekte im gemeinsamen Speicher gehören keinem Circle und laufen mit.**
  Der Playmaker legt keine Defekte an und schließt keine; der Hinweis steht hier, weil vier der
  sieben das Werkzeug betreffen und nicht KRK. Verbindlich ist der Dateibestand:

  ```sh
  find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
  ```

  Einer davon trifft die nächste Runde unmittelbar:
  `shared/issues/260811-1245_*_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`
  ist der vom Nutzer am 260811-1240 gemeldete Rückfall der Vorschaubreite und zugleich die
  siebte offene Frage des Circles auf Rang 2.
