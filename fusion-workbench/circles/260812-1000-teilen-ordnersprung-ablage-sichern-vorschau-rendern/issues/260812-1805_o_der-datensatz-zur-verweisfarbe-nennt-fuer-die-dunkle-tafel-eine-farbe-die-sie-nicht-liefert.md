Der Datensatz zur Verweisfarbe nennt für die dunkle Tafel eine Farbe, die sie nicht liefert

---

`issues/260812-1701_o_ein-gerendertes-markdown-behaelt-nach-dem-wechsel-auf-dunkel-die-verweisfarbe-der-hellen-tafel.md`
schreibt unter „Wie schwer es wiegt": „Die beiden Tafeln des Vorgabesatzes
liefern dafür (208, 135, 112) in Hell und (235, 203, 139) in Dunkel". Die
zweite Zahl stimmt nicht. Beide Tafeln liefern für den Verweisstapel dieselbe
Farbe, (208, 135, 112). Damit hat der beschriebene Defekt heute **keine
sichtbare Wirkung**, und die Abwägung, die der Datensatz dem Nutzer vorlegt,
steht auf einer falschen Zahl.

---

**Gemessen am 260812**, mit denselben Kisten und Merkmalen wie das Projekt
(`syntect 5.3.0` ohne Vorgabemerkmale, `two-face 0.5.2` mit `syntect-fancy`,
`ThemeSet::load_defaults`) und dem Wortartenstapel aus
`hervorhebung::VERWEISSTAPEL`:

```
base16-ocean.light
   Grundfarbe                    79/91/102
   VERWEISSTAPEL (linkfarbe)     208/135/112
base16-ocean.dark
   Grundfarbe                    192/197/206
   VERWEISSTAPEL (linkfarbe)     208/135/112
   Wortarten mit 235/203/139     support.class, entity.name.class,
                                 entity.name.type.class, markup.bold
```

(235, 203, 139) kommt in der dunklen Tafel vor, aber für Klassennamen und für
fette Auszeichnung, nicht für Verweise.

**Was das an dem offenen Datensatz ändert.** Er stellt drei Zuschnitte zur
Wahl und beschreibt den Schaden als „kein unsichtbarer Text, sondern eine
falsche Farbe". Tatsächlich ist es **dieselbe** Farbe: ein Markdown-Tab, der in
Hell gerendert wurde und in Dunkel weiterlebt, zeigt seine Verweise in genau
der Farbe, die er auch nach einem Neurendern zeigte. Möglichkeit 1 („so
lassen") ist damit deutlich stärker, als der Datensatz sie darstellt, und
Möglichkeit 2 („den aktiven Tab neu laden") kauft heute nichts.

**Was am Datensatz stehen bleibt.** Der beschriebene Bau ist richtig
wiedergegeben: die Farbe entsteht beim Rendern und zieht bei einem Wechsel des
Erscheinungsbildes nicht nach. Der Datensatz ist nicht falsch, seine
Gewichtung ist es. Er sollte die berichtigte Messung tragen, bevor der Nutzer
zwischen den drei Zuschnitten wählt — die Wahl fiele sonst auf einer Grundlage,
die es nicht gibt.

**Zwei Nachbarstellen tragen dieselbe Ungenauigkeit, wenn auch schwächer.** Der
Doc-Kommentar an `hervorhebung::linkfarbe`
(`crates/krk-ui/src/hervorhebung.rs:443-457`) sagt, mit dem Nachschlag folgten
„Hell und Dunkel dem System wie bisher"; für diese eine Farbe folgt nichts, sie
ist in beiden dieselbe. Und die Probe
`markdown::tests::der_verweis_traegt_die_farbe_seiner_tafel`
(`crates/krk-ui/src/markdown.rs:540`) läuft über beide Tafeln und vergleicht
jede mit `linkfarbe(tafel)`; sie hielte auch dann, wenn beide Tafeln dieselbe
Farbe lieferten — was sie tun. Das ist keine falsche Probe, aber sie misst
nicht, was ihr Name nahelegt.

**Nicht betroffen:** die Messung, die zur Abweichung vom Plan geführt hat. Dass
`markup.underline.link` allein in **beiden** Tafeln die Grundfarbe liefert und
erst der volle Stapel `meta.link` trifft, ist nachgemessen und stimmt genau; die
Ersatzlösung über `VERWEISSTAPEL` ist richtig.

**Gewicht:** niedrig, mit einer Einschränkung: der Datensatz ist offen und
wartet auf eine Nutzerentscheidung, und die Zahl darin trägt diese
Entscheidung.

**Herkunft:** Circle der Runde 6, Planschritt 9.
