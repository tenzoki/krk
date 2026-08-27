# Shaper: die Vorschau zählt den Inhalt eines Ordners in einem eingebauten Default-Profil

**Datum:** 2026-08-27
**Modus:** anticipated-circle (dispatch über `/fusion:direct`)
**Ergebnis:** `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/_a_circle.md`

## Der Entwurf

Die Vorlage war der Backlog-Eintrag `shared/backlog/260826-1920_*_vorschau-default-profil-zaehlt-ordnerinhalt.md`: die Vorschau soll ein Default-Profil nutzen, das greift, wenn kein anderes Leseprofil greift, und für einen Ordner zusätzlich zu den Metadaten die Zahl der Dateien und der Unterordner zeigen, versteckte je in Klammern. Der Eintrag trug eine Idee und war damit im Ganzen beförderbar; er steht seit dieser Sitzung auf geschlossen und nennt in seiner letzten Zeile den Circle, der aus ihm geworden ist.

## Die eine Klärungsrunde

Vier Fragen, alle vom Nutzer beantwortet:

1. Ersetzt das Default-Profil die Metadaten oder ergänzt es sie? **Ergänzend.** Die sechs Metadatenangaben bleiben stehen, die Zählzeilen treten darunter.
2. Wo steht das Default-Profil? **In KRK eingebaut**, kein Block in `readers.toml`, weder anpassbar noch abschaltbar.
3. Was sagen die Zahlen? **„Dateien: 42 (3)"** heißt zweiundvierzig insgesamt, davon drei versteckt. Verknüpfungen bekommen eine dritte Zeile derselben Form.
4. Folgen die Zahlen dem Schalter für die versteckten Einträge? **Nein**, gezählt wird immer alles. Die Klammer sagt, was beim Hineingehen fehlt, solange der Schalter aus ist.

Damit war die Directive vollständig bestimmt, und der Circle ist angelegt worden. Eine zweite Runde ist nicht gefahren; was offen blieb, ist als Entscheidungsdatensatz abgelegt, weil es die Abnahmekriterien und nicht die Directive betrifft.

## Die Grundlagen-Aufnahme

Erkundet wurde der Bestand um die Vorschau und die Leseprofile, mit Belegen am Baum. Tragend für die spätere Planung sind sechs Befunde:

- Der Rückfall auf die sechs Metadatenangaben, wenn kein Profil greift, ist ein festgehaltener Nutzerwille vom 260823 und kein Fehlerfall.
- Ein Default-Profil hat im heutigen Bau keinen Ort. Weil der Nutzer es als eingebaut bestimmt hat, entsteht es als Zweig neben `erkennen` und nicht als dreizehnter Block der Auslieferungsfassung.
- Der vorhandene Baustein `zaehlung` zählt nach Namensmuster und trennt weder nach Typ noch nach versteckt. Beide Trennungen liegen dagegen am `Eintrag` bereit, `Typ` mit genau den drei Werten der drei Zählzeilen.
- Flach und nicht über den Unterbaum ist Festlegung A2 der Runde 16 und keine offene Frage.
- Der Weg, einen fremden Ordner einmal zu lesen, steht in `leser::lesen_hoechstens` und trägt heute jede Zusammenfassung.
- Der Arbeitsfaden der Vorschau kennt keinen Abbruch, und die Arbeit an der Vorschau ist gegen die Zeitzusage L7 bis heute ungemessen.

Zwei Zitate in einem ersten Entwurf der Grundlagen zeigten auf `shared/decisions/`, wo die Datensätze der Runde 18 nicht mehr liegen; sie stehen seit dem Archivlauf vom 260826-1637 unter `archive/`. Die Aufnahme im Circle-Datensatz trägt die berichtigten Pfade.

## Abgelegte Datensätze

- `decisions/260827-0311_*_bekommen-die-profile-aus-readers-toml-die-zaehlung-nach-typ-und-versteckt.md` — wird die Zählung nach Typ und versteckt eine allgemeine Fähigkeit der Profile oder ein Sonderweg des Default-Profils? Drei Möglichkeiten, Empfehlung für die erste.
- `decisions/260827-0311_*_was-sagen-die-zaehlzeilen-fuer-einen-ordner-ueber-der-eintragsschranke.md` — was zeigen die drei Zeilen über zweitausend Einträgen? Drei Möglichkeiten, keine belastbare Empfehlung ohne eine Messung.

## Was nicht geschehen ist

Kein Spec, kein Plan, keine Aktivierung. Der Circle trägt den Marker für vorgesehen, `.active-circle` ist unberührt, und die Directive wird beim Aktivieren neu geschärft.
