`verdeckten_tab_setzen` baut denselben frischen `Tabinhalt` und trägt zwei von vier Werten hinüber

---

`Tabliste` hat zwei Stellen, die einen Tab auf einen anderen Ordner setzen, indem sie einen
frischen `Tabinhalt` an die Stelle des alten setzen und vorher einzelne Werte aus dessen
`Ordnermodell` retten. Seit B2 tragen die beiden verschieden viel:

| Stelle | Sortierung | Verstecke | Filter der Tiefe | Filtertext |
|---|---|---|---|---|
| `ordner_setzen` (`crates/krk-ui/src/tabs.rs:592`) | ja | ja | ja | **ja, unbedingt** |
| `verdeckten_tab_setzen` (`crates/krk-ui/src/tabs.rs:440`) | ja | ja | **nein** | **nein** |

---

**Wann das auffällt.** `verdeckten_tab_setzen` hat genau einen Aufrufer, `tab_ordner_setzen`
in `crates/krk-ui/src/appkit/tabelle.rs:615`, und der bedient den Auswurf eines
Datenträgers aus `crate::auffrischung::datentraeger_verloren`. Ein **verdeckter** Tab, der
auf dem ausgeworfenen Datenträger stand, fällt auf den Standardordner zurück; steht in ihm
ein Filtertext und ist der Filter der Tiefe an, sind beide danach weg. Der Nutzer sieht das
erst, wenn er auf diesen Tab wechselt.

**Warum es der Rede wert ist.** C1.10 des Spec sagt zu: „Ist ‚Deep' an, übersteht der
Filtertext **jeden** Ordnerwechsel." Ob der Auswurf eines Datenträgers ein Ordnerwechsel in
diesem Sinne ist, sagt weder der Spec noch der Plan; B2 nennt in seinen `Changes` allein
`Tabliste::ordner_setzen`. Der Zustand ist damit nicht falsch, sondern unentschieden — und
er steht als zweite Fassung derselben Übertragung da, also genau in der Form, die A1 dieser
Runde am Prüfschritt für die Sichtbarkeit beseitigt hat
(`issues/260814-2102_c_der-pruefschritt-fuer-die-sichtbarkeit-steht-im-ordnermodell-zweimal-wortgleich-da.md`).

**Zwei Auswege.** Entweder die vier Werte an einer Stelle übertragen, die beide Wege rufen —
dann ist die Übertragung eine Regel und keine Aufzählung, die beim nächsten fünften Wert
wieder auseinanderläuft. Oder ausdrücklich festhalten, dass ein verlorener Datenträger den
Tab vollständig zurücksetzt, und den Unterschied an beiden Stellen als Prosa begründen.

Der Grund, warum B2 keinen der beiden gegangen ist: beide ändern das Verhalten eines Wegs,
den der Plan nicht nennt, und die Wahl zwischen ihnen ist eine Entwurfsfrage.

---

**Nachtrag 260815-1130 (coder, Aufgabe T4).** Zwei Aussagen dieses Datensatzes sind mit
`897605e` überholt und hier berichtigt; der Datensatz selbst bleibt offen, denn an
`verdeckten_tab_setzen` hat sich nichts geändert.

1. **Die Befundtabelle** führte für `ordner_setzen` „ja, wenn der Filter der Tiefe an ist".
   Seit `897605e` trägt `ordner_setzen` den Filtertext unbedingt hinüber, gleich wie der
   Filter der Tiefe steht. Die Spalte ist auf „ja, unbedingt" nachgezogen, und die beiden
   Zeilennummern stehen auf dem heutigen Stand.
2. **Die Einordnung als „unentschieden"** stützte sich auf den damaligen Wortlaut von
   C1.10, „ist ‚Deep' an, übersteht der Filtertext **jeden** Ordnerwechsel". Dieser Vorbehalt
   ist entfallen: C1.9 sagt seit dem Nutzerentscheid vom 260815-0955
   (`decisions/260814-1830_i_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`,
   Möglichkeit 2) unbedingt zu, dass der Filtertext jeden Ordnerwechsel übersteht, und
   C1.10 ist dabei keine Ausnahme mehr, sondern ein Fall von C1.9. Der Auswurf unter einem
   verdeckten Tab ist damit **kein unentschiedener Fall mehr, sondern ein Widerspruch zum
   Wortlaut von C1.9** — festgehalten in
   `shared/issues/260815-1047_o_c1-9-und-der-doc-kommentar-nennen-zwei-loeschwege-des-filtertextes-der-baum-hat-fuenf.md`.

Der Nutzer hat am 260815-1055 für jenen Datensatz Möglichkeit 1 gewählt: die Aufzählung der
Löschwege wird geöffnet, das Verhalten der drei Wege bleibt, wie es ist. C1.9 und der
Doc-Kommentar von `Tabliste::ordner_setzen` nennen den Auswurf unter einem verdeckten Tab
seither ausdrücklich als einen der drei Wege, die den Filtertext ohne Zutun des Nutzers
wegnehmen. **Die Entwurfsfrage dieses Datensatzes bleibt offen**: ob die vier Werte an einer
Stelle übertragen werden, hat der Nutzer nicht beantwortet, und beide Auswege oben stehen
unverändert.
