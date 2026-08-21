Ein Refspec mit Doppelpunkt kommt durch die Aufsicht und löscht eine Referenz auf der Gegenseite

---

`git::aufsichtsbefund` (`xtask/src/git.rs:279`) prüft jedes Wort hinter dem Unterbefehl auf ein
führendes `+`, weil ein Refspec auch ohne jede Marke erzwingen kann. Die zweite Form, mit der
ein Refspec ohne jede Marke wirkt, ist der Doppelpunkt: `<quelle>:<ziel>` schreibt eine
beliebige Referenz auf der Gegenseite, und `:<ziel>` mit leerer Quelle **löscht** sie. Die
Aufsicht kennt diese Form nicht. `Auftrag::Schub` trägt genau einen frei belegbaren
Wortplatz, und das ist der, an dem ein solcher Refspec stünde.

---

**Gemessen am Baumstand `94855a7`.**

## Der Befund

`Auftrag::Schub` baut vier Wörter (`git.rs:186`):

```rust
Auftrag::Schub { verweis } => vec!["push", "origin", "HEAD", verweis],
```

`verweis` kommt von außen. `gewaltbefund` (`git.rs:314-341`) kennt vier Fälle: die Marken aus
`MARKEN`, die aus `UEBERGEHENDE`, den Wortanfang `--force` und den Wortanfang `+`. Ein Wort
mit Doppelpunkt fällt durch alle vier.

Ich habe die Aufsicht Wort für Wort in eine eigene Kiste kopiert und die Fälle laufen lassen.
Durch kommen unter `Wirkung::Schreibt`:

```
DURCH      git push origin :refs/heads/main
DURCH      git push origin HEAD :refs/heads/main
DURCH      git push origin refs/tags/v1:refs/heads/main
DURCH      git push origin HEAD:refs/heads/main
```

Dass die Löschform wirkt, ist an einem Wegwerf-Verzeichnis nachgemessen und nicht abgeleitet.
Ein bares Zielverzeichnis, zwei Zweige darin, dann genau die vier Wörter, die `Auftrag::Schub`
baut:

```
$ git ls-remote origin
  4837ae3	refs/heads/feature
  4837ae3	refs/heads/main
$ git push origin HEAD :refs/heads/feature
  To ../remote.git
   - [deleted]         feature
$ git ls-remote origin
  4837ae3	refs/heads/main
```

Keine Marke, kein `+`, erlaubter Unterbefehl. Die Aufsicht hält nichts an.

## Warum das eine Lücke in einer Regel ist, die dieses Werkzeug führen will

Zwei Gründe, und beide stehen im Baum selbst.

**`MARKEN` trägt `--delete`** (`git.rs:233`). Die Liste ist also ausdrücklich dafür da, das
Löschen einer Referenz auf der Gegenseite zu verhindern. Der Doppelpunkt ist die zweite
Schreibweise desselben Vorgangs und die ältere; `--delete` kam erst später dazu.

**Die `+`-Regel gibt es schon** (`git.rs:333`), und ihre Begründung im Doc-Kommentar lautet:
ein Verweis, der „ohne jede Marke erzwingt". Genau dieselbe Erwägung trägt den Doppelpunkt.
Die Regel ist eine Zeile vor ihrem Ende stehengeblieben.

## Was heute nicht bedroht ist

`verweis` entsteht in `veroeffentlichung::tagverweis` als `refs/tags/{tag}`, `tag` in
`tagname` als `v{zahl}`, und `zahl` geht durch `version::versionszahl_pruefen`
(`xtask/src/version.rs:433-462`), das nur drei Gruppen von ASCII-Ziffern durchlässt. Aus dem
heutigen Baum entsteht kein Refspec mit Doppelpunkt. **Die Lücke liegt nicht in der Gegenwart.**

Sie liegt darin, wofür die Aufsicht gebaut ist. Der Prüfkommentar von
`die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen` (`git.rs:574-577`) sagt es
selbst: „Keine dieser Listen baut heute jemand — genau darum geht es. Die Aufsicht ist für die
Änderung von morgen gebaut." Und die Sicherung, die heute wirklich trägt, steht nicht in
`git.rs`, sondern in `version.rs`; die Aufsicht sagt darüber nichts.

## Abhilfe

Die punktuelle Abhilfe wäre eine fünfte Frage in `gewaltbefund`: ein Wort hinter dem
Unterbefehl `push`, das ein `:` trägt, ist ein Refspec mit Ziel und wird angehalten.

**Die bessere steht im verwandten Datensatz zur Abkürzungslücke** und schließt beide Löcher
mit einem Schnitt: `Auftrag` ist eine geschlossene Aufzählung mit sieben festen Wortformen,
also lässt sich je Variante angeben, welche Wörter fest sind und welche Plätze belegt werden —
und für jeden belegten Platz, welche Gestalt er haben darf (eine Versionszahl, ein Tagname,
ein Pfad). Aus der Verbotsliste, die nie beweisbar vollständig ist, wird dann eine
Erlaubnisliste, wie sie dieses Modul für die Unterbefehle schon führt.

**Schwere:** hoch, und ausdrücklich **kein Auslieferungshindernis**. Aus den heutigen sieben
Varianten ist der Fall nicht erreichbar. Hoch, weil `push` in dieser Runde zum ersten Mal
erlaubt ist, weil die Aufsicht genau für diesen Fall gebaut wurde und ihn nicht fängt, und
weil der Modulkopf sie stärker beschreibt, als sie ist.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `xtask/src/git.rs:279-341` (`aufsichtsbefund`, `gewaltbefund`), `:162-171`
(`Auftrag::Schub`), `:30-37` (Modulkopf, die drei Sätze)

**Domain:** code

**Verwandt:**
`shared/issues/260821-1432_o_git-nimmt-abkuerzungen-langer-marken-an-und-der-gleichheitsvergleich-in-marken-faengt-sie-nicht.md`
— dieselbe Wurzel und dieselbe Abhilfe: die Aufsicht liest freie Wörter, ohne die Gestalt zu
kennen, die an ihrem Platz zulässig ist.
`shared/reviews/260821-1346-coderev-artefakt-und-release.md`, Befund A2 — die vorige Durchsicht
hat den `+`-Refspec gefunden; behoben ist er, der Doppelpunkt blieb ungesehen.

---

Resolved: Behoben am 260821 als Umstellung und nicht als Flicken, wie der Datensatz sie unter
„Abhilfe" als die bessere benennt. `git::Auftrag::wortplaetze` sagt je Variante, welche Wörter
fest dastehen und welche Plätze der Aufrufer belegt; jeder belegte Platz nennt eine
`git::Gestalt`. Der Verweis von `Auftrag::Schub` trägt `Gestalt::Tagverweis`, also `refs/tags/`
und danach einen Tagnamen, dessen Zahl `version::versionszahl_pruefen` liest — die Prüfung, die
der Datensatz unter „Was heute nicht bedroht ist" als die wirklich tragende benennt und die die
Aufsicht jetzt selbst ruft. Ein Doppelpunkt hat in dieser Form keinen Platz. Nachgesehen von
`git::tests::die_aufsicht_faengt_den_refspec_mit_doppelpunkt` mit genau den drei Formen dieses
Datensatzes und von `ein_belegter_platz_traegt_nur_seine_gestalt`; die Löschwirkung selbst ist an
einem eigenen Wegwerf-Verzeichnis nachgemessen und nicht aus dem Datensatz übernommen. Daneben
hält `gewaltbefund` den Doppelpunkt jetzt auch an einem **festen** Wort an, für den Fall, dass
ihn jemand in eine neue Variante schreibt — dort mit dem Vorbehalt, unter dem jede Verbotsliste
steht.
