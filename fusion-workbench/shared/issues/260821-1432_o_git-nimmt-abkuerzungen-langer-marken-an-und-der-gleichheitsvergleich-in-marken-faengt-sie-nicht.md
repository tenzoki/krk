`git` nimmt Abkürzungen langer Marken an, und der Gleichheitsvergleich in `MARKEN` fängt sie nicht

---

`gewaltbefund` (`xtask/src/git.rs:314`) prüft die sieben Wörter aus `MARKEN` und das eine aus
`UEBERGEHENDE` auf **Gleichheit**. `git` nimmt lange Marken aber auch abgekürzt entgegen,
solange die Abkürzung eindeutig ist: `git tag --del` löscht einen Tag, `git commit --ame`
ändert den letzten Eintrag, `git push --mirr` und `--al` kommen durch den Optionszerleger.
Keines dieser Wörter steht in `MARKEN`, also hält die Aufsicht keines an. Der Modulkopf sagt
dagegen zu, dass keine Liste „eine Marke aus `MARKEN`" trägt.

---

**Gemessen am Baumstand `94855a7`.**

## Der Befund

`MARKEN` (`git.rs:228-237`) trägt sieben Wörter, `UEBERGEHENDE` (`git.rs:242`) eines. Beide
werden mit `contains(&wort)` geprüft, also auf Gleichheit. Der Doc-Kommentar von `MARKEN` sagt
das ausdrücklich: „Verglichen wird das ganze Wort." Er nennt auch den Grund, aus dem die
`--force`-Familie danebensteht — sie hat Formen mit **Anhang**, die ein Gleichheitsvergleich
nicht fängt.

Die Formen mit **Abschlag** hat niemand betrachtet, und `git` nimmt sie an. Nachgemessen in
einem Wegwerf-Verzeichnis:

```
$ git tag t1 && git tag --del t1
Tag 't1' gelöscht (war a7cda2a)

$ git commit --ame --no-edit -a          # ändert den letzten Eintrag, kein Abbruch

$ git push --mirr /nonexistent-remote
Schwerwiegend: '/nonexistent-remote' does not appear to be a git repository
        ^ der Optionszerleger hat --mirr angenommen; erst die Gegenseite fehlt

$ git push --al /nonexistent-remote
Schwerwiegend: '/nonexistent-remote' does not appear to be a git repository
```

Die Aufsicht lässt alle vier durch. Aus dem nachgebauten Lauf über `aufsichtsbefund`:

```
DURCH      git push --del origin refs/tags/v1
DURCH      git push --dele origin refs/tags/v1
DURCH      git push --mirr origin
DURCH      git push --al origin
DURCH      git push --tag origin
DURCH      git commit --ame -m x
```

**Eine Abkürzung fängt `git` selbst ab:** `--no-ver` ist mehrdeutig zwischen `--no-verbose`
und `--no-verify` und wird abgewiesen. Das ist Zufall und keine Sicherung — `--no-verif` wäre
eindeutig.

## Zwei weitere Wörter, die keine Liste kennt

Beim selben Nachlauf fallen zwei Marken auf, die `git push` mitbringt und die in `MARKEN`
fehlen:

```
DURCH      git push --exec=/bin/sh origin HEAD
DURCH      git push --receive-pack=/bin/sh origin HEAD
```

Beide benennen das Programm, das auf der Gegenseite den Empfang abwickelt. Sie tragen einen
Anhang hinter `=`, also fängt sie auch ein Gleichheitsvergleich auf ihre Langform nicht.

## Die Prosastelle, die dadurch zu stark ist

`git.rs:30-37`, die drei Sätze, die die ganze Zusage tragen:

```
//! ihre Wirkung nennt. Die Aufsicht auf dem Weg haelt, dass keine Liste — auch
//! die einer ungeprueften neuen Variante — einen fremden Unterbefehl, eine Marke
//! aus [`MARKEN`], eine `--force`-Form, eine kurze Gewaltmarke oder einen
//! erzwingenden Verweis mit `+` traegt.
```

„Eine Marke aus `MARKEN`" hält sie nicht: `--del` **ist** für `git` die Marke `--delete`.

Der Doc-Kommentar von `aufsichtsbefund` führt unter „Was sie nicht kann" den Vorbehalt, der
vierte Punkt sei eine Verbotsliste und nie beweisbar vollständig. Der Vorbehalt deckt eine
Marke, an die niemand gedacht hat. Er deckt **nicht**, dass eine Marke, die in der Liste steht,
in ihrer abgekürzten Schreibweise vorbeikommt.

## Was heute nicht bedroht ist

Aus den sieben Varianten entsteht keine dieser Formen; die belegbaren Wortplätze tragen eine
geprüfte Versionszahl, einen Tagnamen und zwei feste Dateinamen. **Die Lücke liegt in dem,
wofür die Aufsicht gebaut ist**, und das schreibt der Prüfkommentar von
`die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen` (`git.rs:574-577`) selbst
aus: „Die Aufsicht ist für die Änderung von morgen gebaut."

## Abhilfe

Punktuell ließe sich der Vergleich umdrehen: ein Wort mit `--` am Anfang wird angehalten,
wenn **irgendein** Eintrag aus `MARKEN` oder `UEBERGEHENDE` mit ihm beginnt (nach Abtrennen
eines `=`-Anhangs). Das fängt `--del`, `--ame`, `--mirr`, `--al`, `--tag`, und `--exec=` und
`--receive-pack=` kämen als zwei weitere Einträge dazu.

**Das bleibt aber eine Verbotsliste**, und die nächste Marke, an die niemand gedacht hat,
steht wieder nicht darin. Der eine Schnitt, der auch den verwandten Doppelpunkt-Befund
schließt: `Auftrag` ist eine geschlossene Aufzählung mit sieben festen Wortformen. Statt zu
prüfen, was ein freies Wort **nicht** sein darf, lässt sich je Variante angeben, welche Wörter
fest sind und welche Plätze belegt werden — und für jeden belegten Platz, welche Gestalt er
tragen darf. Was nicht so aussieht, kommt nicht durch, und niemand muss eine Marke verbieten,
an die er nicht gedacht hat. Es ist dieselbe Bauart, die das Modul für die Unterbefehle schon
führt und in deren Doc-Kommentar begründet (`git.rs:208-212`).

Ein Nebeneffekt derselben Umstellung: die heutige Aufsicht liest auch die Eintragsmeldung
hinter `-m` als hätte sie eine Marke sein können. `git commit --only -m "-a" -- Cargo.toml`
wird angehalten, obwohl `-a` dort der Wert einer Option und kein Schalter ist. Aus dem
heutigen Baum entsteht die Lage nicht, weil die Meldung fest gefügt wird; ein Aufsichtsmodell,
das Plätze kennt, hätte sie ohnehin nicht.

**Schwere:** hoch, und ausdrücklich **kein Auslieferungshindernis**. Aus den heutigen sieben
Varianten ist der Fall nicht erreichbar. Hoch, weil hier nicht eine vergessene Marke fehlt,
sondern die Marken vorbeikommen, die ausdrücklich dastehen — und weil der Modulkopf das
Gegenteil zusagt.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `xtask/src/git.rs:222-251` (`MARKEN`, `GEWALTANFANG`, `UEBERGEHENDE`),
`:314-341` (`gewaltbefund`), `:30-37` (Modulkopf, die drei Sätze), `:253-278`
(`aufsichtsbefund`, Abschnitt „Was sie nicht kann")

**Domain:** code

**Verwandt:**
`shared/issues/260821-1432_o_ein-refspec-mit-doppelpunkt-kommt-durch-die-aufsicht-und-loescht-eine-referenz-auf-der-gegenseite.md`
— dieselbe Wurzel und dieselbe Abhilfe.
`shared/reviews/260821-1346-coderev-artefakt-und-release.md`, Befund A2 — dort ist der
Gleichheitsvergleich schon als Ursache dreier Lücken benannt; geschlossen wurden die drei
Wörter, nicht der Vergleich.
