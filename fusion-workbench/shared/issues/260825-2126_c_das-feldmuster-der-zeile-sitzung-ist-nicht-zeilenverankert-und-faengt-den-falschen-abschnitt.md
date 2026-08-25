Das Feldmuster der Zeile „Sitzung" ist nicht zeilenverankert und fängt den falschen Abschnitt

---

Die Zeile „Sitzung" der zwei fusion-Profile
(`resources/default-readers.toml:296` und `:625`) sucht mit
`'## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)'`. Das Muster ist an keinem Ende
verankert und läuft über den ganzen Dateiinhalt. Es trifft deshalb auch `### Current` — die
Zeichenfolge `## Current\n` steht darin — und ebenso ein `## Current` mitten in einer Zeile
Fließtext. Steht eines von beiden vor dem echten Abschnitt, zeigt die Vorschau dessen Inhalt
statt der laufenden Sitzung, und zwar ohne jeden Hinweis.

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:178-183` (der Absatz „Zur Verankerung",
der `(?m)` für genau diesen Fall vorschreibt), `:256-265` (der Kommentar, der das Verhalten
des Musters ausschreibt), `:296` und `:625`

## Was die Datei selbst dazu sagt

`resources/default-readers.toml:180-183`: „Das `feldmuster` läuft dagegen über den ganzen
Dateiinhalt, und dort verankern `^` und `$` am Anfang und Ende der GANZEN Eingabe. Wer eine
einzelne Zeile darin verankern will, schreibt `(?m)` an den Anfang seines Ausdrucks."

Von den sechs ausgelieferten Feldmustern ist dies das einzige, dessen Aussage an einem
Zeilenanfang hängt, und das einzige, das die Regel nicht anwendet. Das Muster der Zeile
„Directive" trägt `(?sm)` und verankert richtig; die drei JSON-Muster brauchen keine
Verankerung; das Muster über `.active-circle` verankert bewusst am Anfang der ganzen Eingabe.

Der erklärende Kommentar bei `:260-265` beschreibt vier Eigenschaften des Musters — es
überspringt Leerzeilen, hält vor der nächsten Überschrift an, überliest eine Zeile mit `#`,
verträgt ein fehlendes Zeilenende — und keine davon ist falsch. Was er nicht sagt, ist, dass
das Muster den Abschnitt gar nicht als Überschrift erkennt.

## Was gemessen ist

Gemessen am 260825-2126 über `leseprofil::zusammenfassen_gezaehlt`, Baum `8478753`, an zwei
künstlichen Werkbankwurzeln:

| `orchestrator-live.md` | Zeile „Sitzung" zeigt |
|---|---|
| `### Current` mit `FALSCHER-ABSCHNITT`, danach `## Current` mit `RICHTIG` | `FALSCHER-ABSCHNITT` |
| Fließtextzeile `siehe ## Current`, darunter `GEFANGEN-AUS-FLIESSTEXT`, danach der echte Abschnitt | `GEFANGEN-AUS-FLIESSTEXT` |
| `## Current` mit CRLF-Zeilenenden | `--` |

Der dritte Fall ist eine zweite Folge derselben Bauart: `\n` im Muster trifft `\r\n` nicht,
und das `\r` am Zeilenende landete ohnehin im gefangenen Wert. Er ist hier nachrangig, weil
fusion die Datei mit `\n` schreibt.

## Warum das zählt

Der Kommentar über dem Profil nennt die Zeile ausdrücklich als die eine, die an einer Form
hängt, die fusion ändern kann, und sagt dazu die richtige Folge: „Ändert fusion sie, zeigt
diese eine Zeile ihren Platzhalter." Der Platzhalter ist die ehrliche Auskunft. Ein Wert aus
einem anderen Abschnitt ist es nicht — er sieht aus wie eine Antwort.

`orchestrator-live.md` ist eine von fusion erzeugte Datei mit Überschriften; eine
Unterüberschrift `### Current` in einem späteren fusion-Format oder ein Zitat des Abschnitts-
namens im Fließtext genügt.

## Was zu tun wäre

`(?m)^` vor `## Current` setzen, also
`'(?m)^## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)'`, und zwar an **beiden** Stellen
(`:296` und `:625`), denn die sieben Zeilen stehen zweimal in der Datei. Damit trifft das
Muster den Abschnitt und nicht mehr seine Unterüberschrift und nicht mehr seine Erwähnung.

**Schwere:** mittel. Kein Bau hängt daran; die Zeile kann einen falschen Wert zeigen, wo sie
den Platzhalter zeigen müsste.

---
Resolved: `resources/default-readers.toml` trägt an beiden Stellen (`:324` und `:662`, nach der Verschiebung durch die neue Prosa) das Muster `'(?m)^## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)'`; der Kommentar über dem Wurzelprofil sagt, was das `(?m)^` abhält. Nachgemessen am 260825 über `leseprofil::zusammenfassen_gezaehlt` an drei künstlichen Werkbankwurzeln: `### Current` mit `FALSCHER-ABSCHNITT` vor dem echten Abschnitt zeigt `RICHTIG`, die Fließtextzeile `siehe ## Current` zeigt `RICHTIG`, CRLF zeigt `--` (der Platzhalter, wie angesagt; fusion schreibt `\n`). `cargo test -p krk-core --lib leseprofile` Exit 0.
