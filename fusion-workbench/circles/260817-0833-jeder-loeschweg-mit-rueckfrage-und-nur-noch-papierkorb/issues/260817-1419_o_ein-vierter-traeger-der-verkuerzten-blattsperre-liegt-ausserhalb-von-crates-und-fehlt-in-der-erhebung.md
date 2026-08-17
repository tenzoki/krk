Ein vierter Träger der verkürzten Blattsperre liegt außerhalb von `crates` und fehlt in der Erhebung

---

`resources/default-keymap.toml:708` sagt, bei stehender Nachfrage weise der
Anwendungsdelegierte „jeden Befehl ausser dem Abbruch" ab. Es sind vier. Der Datensatz
`260817-1302`, der genau diese Formulierung erhebt, sagt von sich „eine Suche ueber den ganzen
Baum findet zwei" und nennt diese Stelle nicht: gelesen wurde `crates/`.

---

**Schwere:** Niedrig. Der Schluss an der Stelle hält, und zwar nachgerechnet: die drei Befehle
der Ausnahmeliste liegen ab Werk auf `cmd+q`, `shift+cmd+w` und `cmd+n` und nicht auf `return`,
über das der Absatz spricht. Falsch ist die Begründung, nicht das Ergebnis. Der Befund steht,
weil eine Erhebung, die „der ganze Baum" sagt und eine Kiste liest, beim nächsten Durchgang
dieselbe Stelle wieder nicht sieht.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `resources/default-keymap.toml:706-709`
**Verwandt:** `issues/260817-1302_o_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-und-der-datensatz-nennt-sie-nicht.md`,
`issues/260817-1111_c_die-begruendung-an-loeschauftrag-stellen-nennt-eine-ausnahme-es-sind-vier.md`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

```toml
# resources/default-keymap.toml:706-709, im Abschnitt von mit_standardprogramm_oeffnen
# Ein Blatt faengt die Taste weiterhin ab, bevor sie hier ankommt: bei stehender
# Nachfrage weist der Anwendungsdelegierte jeden Befehl ausser dem Abbruch ab,
# und der Tastendruck laeuft unveraendert an AppKit weiter, wo ihn die
# Vorgabeschaltflaeche beantwortet.
```

Durchgelassen werden vier Kommandos: `Kommando::Abbrechen` über
`kommandos::operationen::waehrend_blatt_erlaubt` (`operationen.rs:266-268`) und
`Kommando::Beenden`, `Kommando::FensterSchliessen` und `Kommando::FensterEinblenden` über
`kommandos::zulaessigkeit::immer_erreichbar` (`zulaessigkeit.rs:197-202`), das die Blattsperre
ausdrücklich mit aufhebt. Beide Stellen selbst gelesen.

Die vollständige Erhebung über den Baum **und** die Werkbankgrenze hinweg:

```
$ grep -rn "ausser dem Abbruch\|außer dem Abbruch" crates/ CLAUDE.md resources/
crates/krk-ui/src/appkit/anwendung.rs:406    Aussage ueber den Stand bis S16, kein Befund (so in 1302)
crates/krk-ui/src/appkit/anwendung.rs:6312   nennt die Ausnahmeliste, richtig
crates/krk-ui/src/appkit/editor.rs:1298      Befund, in 1302 genannt
CLAUDE.md:123                                Befund, in 1111 genannt, offen
resources/default-keymap.toml:708            Befund, in keinem Datensatz genannt
```

`anwendung.rs:2840`, den `1302` als seinen ersten Träger nennt, trägt die Worte nicht wörtlich
(„ein Blatt laesst allein den Abbruch durch"); die Aussage dort ist dieselbe und der Befund
gilt. Die Zeile ist heute `:2841`.

## Warum die Stelle besonders zählt

Zwei Ebenen daneben steht seit T1 dieselbe Begründung ausgeschrieben und richtig:
`blaetter/mod.rs:296-307` nennt die vier Befehle, nennt beide Quellen und rechnet nach, dass
keiner der drei zusätzlich zugelassenen ab Werk auf einer Eingabetasten-Kombination des Blattes
liegt. Genau diesen Schluss zieht der Kommentar in der Belegungsdatei, und er zieht ihn aus der
verkürzten Prämisse.

Es ist die dritte Ausprägung eines Musters, das dieses Projekt schon führt: CLAUDE.md hält
fest, dass jedes Suchmuster mit `\.md` einen blinden Fleck hat, und `shared/issues/260810-1851`
trägt den Fall, in dem fünf Erhebungen dieselben acht Stellen nicht sahen. Hier ist der blinde
Fleck die Ordnergrenze `crates/`.

## Richtung

Die Zeile in `resources/default-keymap.toml` nachziehen, mit `1302` und `1111` in einem Zug —
sie tragen denselben Nachzug. Und die Nadel der nächsten Erhebung um `resources/` und
`CLAUDE.md` erweitern, bevor gezählt wird.
