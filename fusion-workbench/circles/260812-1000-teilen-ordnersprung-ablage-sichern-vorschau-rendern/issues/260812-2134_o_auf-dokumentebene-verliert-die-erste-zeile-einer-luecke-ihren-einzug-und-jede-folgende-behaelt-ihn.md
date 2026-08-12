Auf Dokumentebene verliert die erste Zeile einer Lücke ihren Einzug, jede folgende behält ihn

---

Seit `2c0b2a6` gibt `Zerlegung::luecke_bis` eine Lücke auf Dokumentebene über
`luecke.trim()` heraus (`crates/krk-ui/src/markdown.rs:758-762`). `trim()`
schneidet an den **beiden Enden der ganzen Lücke** und nicht je Zeile. Zwei
Zeilen mit demselben Einzug in der Quelle kommen deshalb verschieden heraus:
die erste ohne, jede weitere mit.

---

**Gemessen** (`markdown::rendern` aus `1e4e01f` und `2c0b2a6`, beide
unverändert in dasselbe Prüfprogramm kopiert, `pulldown-cmark 0.13.4`,
Tafel Hell):

```
Quelle : "  [a]: http://a.example\n  [b]: http://b.example\n"
1e4e01f: "[a]: http://a.example\n[b]: http://b.example"
2c0b2a6: "[a]: http://a.example\n  [b]: http://b.example"
```

Beide Zeilen tragen in der Quelle zwei Leerzeichen. In der Ausgabe trägt die
zweite sie und die erste nicht. `pulldown-cmark` meldet zu dieser Quelle kein
einziges Ereignis — beide Zeilen sind Verweisdefinitionen —, die ganze Datei
ist also eine Lücke.

**Der Einzug, den die zweite Zeile behält, ist keiner, den CommonMark trägt.**
Bis zu drei Leerzeichen vor einem Block sind dort bedeutungslos und gehören
nicht zum Inhalt. Der Fall, für den die Behebung von
`260812-2019_c_ohne-umgebungszeichen-laeuft-auch-auf-dokumentebene-und-nimmt-dort-einzug-weg-der-inhalt-ist.md`
angetreten ist — die **Fortsetzungszeile** einer mehrzeiligen
Verweisdefinition, deren Einzug wirklich Inhalt ist — steht daneben und ist
von diesem hier nicht zu unterscheiden, solange nur `trim()` gefragt wird.

**Die Probe misst die eine Seite und nicht die Grenze.**
`auf_dokumentebene_bleibt_der_einzug_einer_zeile_stehen`
(`crates/krk-ui/src/markdown.rs:1492-1497`) misst
`"[ZIEL]: http://z.example\n      \"Titel\"\n"`, also den Fall, in dem der
Einzug auf der **zweiten** Zeile steht und stehenbleiben soll. Die erste Zeile
jener Quelle trägt keinen Einzug, also fällt der Unterschied dort nicht auf.

**Der abgetrennte Rest deckt es nicht ab.**
`260812-2140_o_ohne-umgebungszeichen-nimmt-innerhalb-eines-elements-mehr-einzug-weg-als-die-umgebung-wiederholt.md`
begrenzt sich ausdrücklich auf „innerhalb eines Elements" und beschreibt das
Gegenteil dieses Befundes: dort wird zu **viel** weggenommen, hier zu **wenig**
und uneinheitlich. Beide sind Näherungen derselben Frage — wie viel wiederholt
die Umgebung? —, aber sie stehen auf verschiedenen Seiten der Grenze
`self.offen.is_empty()`, und nur eine von beiden ist bisher aufgeschrieben.

**Ein Zuschnitt** (nicht gewählt): derselbe, den der offene Rest 260812-2140
für die andere Seite erwägt — den kürzesten führenden Lauf über alle
nichtleeren Zeilen der Lücke bestimmen und nur diesen abziehen. Auf
Dokumentebene ergäbe das für die gemessene Quelle zwei Leerzeichen auf beiden
Zeilen und für die Quelle der bestehenden Probe null, weil deren erste Zeile
keinen Einzug trägt; beide kämen damit richtig heraus, und es wäre eine Regel
für beide Seiten statt zweier. Ungeprüft ist, was die Regel mit einer Lücke
tut, deren erste Zeile leer ist.

**Gewicht: niedrig.** Kosmetisch, seltene Quelle, kein Inhaltsverlust. Der
Befund ist, dass zwei gleiche Zeilen der Quelle verschieden herauskommen, und
dass diese Ungleichheit mit diesem Turn neu ist und nirgends steht.

**Herkunft:** Circle der Runde 6, Turn 5, `2c0b2a6`.
