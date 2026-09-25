# S42: Abgleich, Spec-Nachträge und die Abnahmeliste für den Nutzer

**Status:** Complete
**Datum:** 260810-0714
**Agent:** `coder`
**Schritt:** S42, der letzte offene der Editor-Runde
**Plan:** `planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`
**Kein Commit.** Der Nutzer committet, sobald die Abnahme durch ist.

---

## Was der Schritt geleistet hat

Der Schritt zieht Spec, Plan und `CLAUDE.md` auf den gebauten Stand und legt dem
Nutzer die Abnahmeliste vor. Am Programm ist keine Zeile geändert; `make check`
und `cargo xtask bundle` sind trotzdem gefahren, weil sie das Abnahmekriterium
sind. Beide enden mit 0, signiert ist mit der Entwicklungsidentität aus dem
Schlüsselbund.

Alle 48 Planschritte tragen `[DONE]`.

## Aus vier Spec-Nachträgen sind sechs geworden

Der Plan zählte vier auf. Einer ist ersatzlos entfallen, drei stehen wie
vorgesehen, und drei sind dazugekommen.

**Entfallen: die beiden ersten Abnahmekriterien von C8.** Der Nachtrag hing an
der Bedingung "falls Weg 1 aus S2 gewählt wurde". Der Nutzer hat am 260808-0155
den dritten Weg gewählt, gegen die Empfehlung des Datensatzes: Buchstaben und
Ziffern werden über das gemeldete Zeichen nachgeschlagen. Damit sind die beiden
Kriterien buchstäblich erfüllbar geworden, statt umgeschrieben werden zu müssen.
C8 hat stattdessen drei Festlegungen bekommen, die die gemessene Ursache
festhalten, die Wahl und ihre Folge.

**Wie vorgesehen:** das abgeleitete Kriterium zur Sicherungsform in C4 (aus S9),
die Anmerkung in C6 zum Anlegen einer Textmarke auf dem bestehenden Befehl, die
Anmerkung in C7 zu `Wirkungsbereich` mit sieben Werten und den drei mit
umgezogenen Befehlen.

**Dazugekommen:** die Zusage, dass keine Automatik den getippten Text ändert (C4,
siehe unten), der Hinweis in C3, dass es zwei fremde Kisten geworden sind und
nicht eine, und die Lesart des zweiten Abnahmekriteriums von C9, die aus dem
Abschnitt `## Nachtrag vom 260809` kommt.

## Die C2/C4-Frage: der Befund des Ausführenden von E1 hält

Er hat festgestellt, dass C4 kein eigenes Abnahmekriterium für "keine Automatik
ändert den getippten Text" trägt. **Am Dateibestand geprüft und bestätigt.** Die
Wörter "Automatik", "typografisch" und "Rechtschreib" kamen im ganzen Spec nicht
vor; "getippt" stand dreimal und keines der drei Vorkommen sagte etwas über
Automatiken.

Die Zusage lebte an drei Stellen: im Plan bei S16 ("C4 sagt zu, dass der
gesicherte Stand der getippte ist"), im Modulkopf von
`crates/krk-ui/src/appkit/editor.rs` und in den Datensätzen. Der Satz zitiert C4
als Quelle, und C4 sagte es nicht.

**Das Nächstliegende im Spec stand unter C2 und meint eine andere Sache.** Jenes
Kriterium lautet "Eine Datei, die keine gültige Textdatei ist, wird abgewiesen
und nicht mit Ersatzzeichen geöffnet. Der Editor hält nie einen Stand, der beim
Sichern Bytes der Datei ersetzen würde." Es handelt von der Binärdatei, die der
Editor gar nicht erst annimmt, nicht von der Ersetzung, die AppKit an gültigem
Text vornimmt. Die beiden Fälle haben nichts gemeinsam außer dem Wort
"ersetzen": im einen Fall geht die Datei kaputt, weil sie nie richtig gelesen
wurde, im anderen, weil AppKit dem Nutzer beim Tippen hilft.

**Folgerung: die Zusage gehört in den Spec, und zwar unter C4.** Ein Abnahmelauf
prüft, was geschrieben steht, und sieben abgeschaltete Automatiken hatten kein
Kriterium, an dem sie abzunehmen wären. C4 ist die Fähigkeit, die über den Weg
vom Getippten in die Datei zusagt; C2 ist die über das, was hineinkommt.

Der Wortlaut des neuen elften Kriteriums:

> Keine Automatik der Textfläche ändert den getippten Text. Was der Nutzer
> tippt, einfügt oder ausschneidet, steht beim Sichern Zeichen für Zeichen in
> der Datei: keine typografischen Anführungszeichen, keine Gedankenstriche,
> keine Textersetzung, keine Rechtschreibkorrektur, kein eingefügtes oder
> fortgenommenes Leerzeichen beim Einfügen und Ausschneiden, keine
> Wortvorhersage und keine Formelergänzung.

Es nennt die Wirkung und keine Zahl. Die Zahl steht im Modulkopf, wo sie
nachgezählt werden kann, und sie ist in dieser Sitzung zweimal gewachsen, von
vier über fünf auf sieben.

## Beide neuen Kriterien von C4 stehen hinten, und das ist gewählt

C4 trägt jetzt elf statt neun. Der sachliche Platz des Sicherungsform-Kriteriums
wäre hinter dem ersten gewesen, der des Automatik-Kriteriums ebenfalls weit
vorn. Beide stehen trotzdem am Ende.

Der Grund ist gemessen: **sechzehn Stellen im Programmtext zitieren ein
Kriterium von C4 nach seiner Nummer**, dazu neun im Plan. Am 260810 haben sich
diese Nummern schon einmal verschoben, als das sechste Kriterium fiel; ein
Einschub in der Mitte hätte sie am selben Tag ein zweites Mal verschoben.
Angehängt bleibt jede bestehende Zahl richtig, und die Festlegungen von C4 sagen
das jetzt dort.

## Von den vier Defekten sind drei geschlossen und einer widerlegt

**Geschlossen: `260810-0421`**, der Plan führte außerhalb von S28 weiter vier
Anlässe. Acht Stellen sind nachgezogen, zwei davon führte der Datensatz nicht:
die Schrittüberschrift von S29 ("Der vierte Anlass: das Beenden") und das
gezählte Abnahmekriterium von S28, das vier Aufrufstellen des Blattes verlangte.
Letzteres ist gestrichen statt auf drei gezogen: gebaut ist eine Aufrufstelle,
der erste Vermerk des Schrittes rechnet vor, warum das strenger hält, und eine
Zahl, die den gebauten Stand für falsch erklärt, ist kein Abnahmekriterium.

**Geschlossen: `260810-0422`**, das fünfte Abnahmekriterium von C4 nannte zwei
Einstiege, wo drei Wege eine Datei aufnehmen. Es nennt jetzt alle drei. Am Code
war nichts zu ändern, weil die Regel im Editormodell sitzt und nicht bei den
Einstiegen.

**Geschlossen: `260809-1657`**, das erste Abnahmekriterium von C2 beschrieb ein
`reserviert_fuer = "editor"`, das S6 entfernt hat. Der Satz steht auf der
eingelösten Form, und die festen Zeilennummern sind ersatzlos entfallen; sie
zeigten schon auf anderen Text.

**Widerlegt: `260810-0359`**, die Erweiterungsnotiz zähle elf Kriterien für C11,
gebaut seien dreizehn. **C11 trägt elf, und der Spec stand richtig.** Der
Zählweg des Datensatzes setzt die Überschrift allein auf `^### C[0-9]+:` und
trägt sie über die Abschnittsgrenze hinaus weiter; die zwei Abnahmekriterien aus
`## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1` sind dadurch C11
zugeschlagen worden. Elf plus zwei ergibt die dreizehn des Datensatzes.

Die vom Datensatz aufgeworfene Frage nach der Fassungsgeschichte ist
mitbeantwortet und stützt denselben Schluss: C11 trägt seit dem Commit
`85293c2`, der die drei Fähigkeiten eingesetzt hat, unverändert elf Kriterien.
Der Spec ist an dieser Stelle nicht geändert; die Falle steht jetzt dort benannt,
samt dem richtigen Zählweg.

## Die Verweise auf Datensätze tragen jetzt durchweg eine Sternstelle

Ein Nebenbefund, der beim Abgleich anfiel und die Abnahme unmittelbar betrifft.
Der Plan trägt die Regel seit dem 260808 in seinem Kopf: ein Verweis auf einen
Datensatz trägt an der Stelle des Zustandsmarkers eine Sternstelle. Er folgte ihr
selbst an fünfzehn Stellen nicht. Der Spec kannte die Regel gar nicht und trug
vierzehn feste Marker, **acht davon veraltet**.

Ein Pfad mit veraltetem Marker findet in einer Suche nichts, und die Abnahme
schlägt Datensätze nach. Beide Dokumente sind gezogen, und der Spec trägt die
Regel jetzt in einem eigenen Abschnitt.

## Was an `CLAUDE.md` geändert ist

Der Abschnitt "Projektstand" trägt das Prüfdatum 260810-0714 und den Stand der
Runde 2: der Editor als fünfter Bereich, die drei Anzeigen mit ihren Modulen, das
Kernmodul `krk-core/src/text/`, die vier gewachsenen Aufzählungen mit ihren
Zahlen und die beiden neuen fremden Kisten. Dazu der Satz, der die Aussage
gerade hält: solange der Abnahmelauf nicht gefahren ist, ist "gebaut" richtig und
"abgenommen" nicht.

Der Abschnitt "Was man nicht sieht, wenn man es nicht weiß" ist von fünf auf
sieben Einträge gewachsen, wie der Plan es vorsieht: die Nämlichkeitsfrage des
Ereignisabgriffs und der eine Auslösepunkt für jeden Wechsel des Ersthelfers.

Vier Stellen waren daneben veraltet und sind mitgezogen. Der Defekt zu L9 ist
seit dem 260807-1935 geschlossen, `CLAUDE.md` nannte ihn offen. Zwei Abschnitte
wiesen den Circle der Runde 1 als den aktiven aus. Die Aufzählung der drei
Fallunterscheidungen ohne Auffangzweig war schon vor dieser Runde unvollständig
und trägt jetzt keine Zahl mehr. Und die Untergrenze der angesprochenen
AppKit-Klassen, die der Übersetzer nicht erzwingt, steht jetzt unter
"Technologiewahl", wo sie einen Absturz auf dem Referenzgerät verhindert.

`README.md` ist unberührt: es nennt keine fremde Kiste, und der Nachtrag im Plan
stand ausdrücklich unter dieser Bedingung.

## Was dieser Schritt nicht getan hat

**Die Abnahme selbst.** Sie verlangt KRK im Vordergrund und ist Nutzerarbeit;
kein Agent kann sie fahren. Was hier steht, ist der Stand, gegen den geprüft
werden kann.

**Die Zustandsmarker der beiden Dateinamen.** Plan und Spec stehen weiter auf
offen. Die Runde 1 hat beide erst beim Schließen des Circles gezogen, die Zeile
`**Active spec/plan:**` im Circle-Datensatz nennt beide beim Namen, und der
Abschluss gehört dem Orchestrator. Der Plan trägt im Kopf `**Status:**
Complete` und sagt den Grund für den offenen Marker dazu.

**Keine Zeitzusage angefasst.** Keine der zehn Zahlen aus C8 der Runde 1 ist
geändert, gelockert oder umgedeutet, und diese Runde setzt keine eigene. Das ist
das zweite der beiden Kriterien, die an die Stelle einer Zeitzusage treten, und
es ist an dieser Stelle abgenommen.

## Geänderte Dateien

- `CLAUDE.md`
- `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`
- `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
- `issues/260810-0421_o_…` → `_c_`, `issues/260810-0422_o_…` → `_c_`,
  `issues/260809-1657_o_…` → `_c_`, `issues/260810-0359_o_…` → `_c_`

## Die Abnahmeliste für den Nutzer

110 Abnahmekriterien, 108 in elf Fähigkeiten und zwei im Abschnitt
`## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`. Alle stehen im Spec.

| Fähigkeit | Kriterien | Worum es geht |
|---|---:|---|
| C1 | 7 | Der Editor als fünfter Bereich, Breite, Fokusbefehl |
| C2 | 11 | Die beiden Einstiege und die eine Größen- und Typprüfung |
| C3 | 12 | Rohansicht und Formatansicht, Syntaxhervorhebung, Hell und Dunkel |
| C4 | 11 | Bearbeiten, Sichern, die Nachfrage, Sicherungsform, keine Automatik |
| C5 | 9 | Zeilensprung, Suchen, Ersetzen |
| C6 | 14 | Textmarken in derselben Leiste und derselben Ablagedatei |
| C7 | 8 | Die Tastatur im Editor: tippen und befehlen zugleich |
| C8 | 5 | Kombinationen mit Zusatztaste wirken |
| C9 | 8 | Der Fokus ist in allen fünf Bereichen zu sehen |
| C10 | 12 | Zeilennummern im Editor und in der Vorschau |
| C11 | 11 | Der volle Pfad im Fenstertitel |
| Zeitzusagen | 2 | Bedienbarkeit beim Laden, die zehn Zahlen unberührt |

**Vier Kriterien lassen sich erst jetzt prüfen** und nicht bei dem Schritt, der
sie gebaut hat; der Plan führt sie unter `### Die Reihenfolge gegen die achtzehn
offenen Schritte`. Es sind das fünfte, achte und neunte von C10 und zwei der
fünf Wege im dritten von C9.

**Eine Reihenfolge, die Wege spart.** C8 zuerst, weil ohne wirkende
Zusatztasten-Kombinationen die halbe Runde nicht erreichbar ist. Dann C2 und C1,
also hineinkommen und die Fläche. Dann C7, weil sich daran zeigt, ob Tippen und
Befehlen zugleich gehen. Danach C3 bis C6 in beliebiger Folge. C9, C10 und C11
laufen nebenher mit: sie sind Anzeigen und bei jedem der übrigen Schritte
sichtbar.

Der Zählweg, falls die Zahlen später wieder gebraucht werden:

```sh
awk '/^#/{h=$0} /^- \[ \]/{n[h]++; t++} END{for(k in n) print n[k], k; print t, "GESAMT"}' \
  fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-*.md
```
