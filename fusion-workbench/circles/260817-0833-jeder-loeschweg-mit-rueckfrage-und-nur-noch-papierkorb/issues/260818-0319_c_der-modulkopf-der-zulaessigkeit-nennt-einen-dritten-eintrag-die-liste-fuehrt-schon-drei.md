Der Modulkopf der Zulässigkeit nennt einen „dritten" Eintrag, und die Liste führt schon drei
---
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, Modulkopf: „Mit einem **dritten** Eintrag, der einen Bereich braucht, fiele der Unterschied an". Die Ausnahmeliste, um die es geht, führt bereits drei Einträge. Gemeint ist ein vierter.

Gefunden vom `coder` beim Beheben der Befunde zur verkürzten Blattsperre (Commit `285b58f`), außerhalb der sechs Datensätze jenes Auftrags und deshalb stehen gelassen statt mitgezogen.
---
**Die Klasse ist bekannt und in dieser Sitzung mehrfach gemessen.** Eine Prosastelle nennt eine Zahl, ein späterer Schritt fügt einen Eintrag hinzu, und niemand zählt nach. Commit `926377f` hat sechs solche Stellen behoben und an drei davon eine Zahl durch eine Regel ersetzt, die nicht veraltet. Hier bietet sich dieselbe Behandlung an: der Satz will sagen, dass der Unterschied erst auffällt, wenn ein Eintrag der Liste einen Bereich braucht. Das lässt sich ohne Ordnungszahl sagen.

Beim Nachziehen ist die Liste zu zählen und nicht die Zahl im Satz fortzuschreiben — die Sitzung `260818` hat zweimal erlebt, dass der gemeldete Umfang eines Befundes kleiner war als der gemessene.

**Domain:** code
**Filed by:** orchestrator
**Related:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/history/260818-0530-coder-sechs-befunde-der-buendel-c-und-d.md` (der Bericht, der die Stelle nennt)

---
Resolved 260818-0350 (coder): **die Ordnungszahl ist weg, und mit ihr die zweite Zahl im selben
Absatz.**

Gezählt wurde die Liste und nicht die Zahl im Satz fortgeschrieben: `immer_erreichbar`
(`zulaessigkeit.rs:197-202`) führt genau drei Einträge — `Kommando::Beenden`,
`Kommando::FensterSchliessen` und `Kommando::FensterEinblenden`. Gemeint war also ein vierter.

Statt die Zahl auf „vierten" zu heben, steht dort jetzt die Regel, so wie es Commit `926377f`
an drei Stellen gemacht hat:

    Dass sie den dritten Bestandteil nicht aufhebt, faellt heute nicht auf:
    jeder Eintrag der Liste traegt `Wirkungsbereich::Ueberall`, und
    [`fokus::wirkt`](super::fokus::wirkt) sagt dafuer in jedem Fokus ja. Sobald
    ein Eintrag der Liste einen Bereich braucht, faellt der Unterschied an, und
    die Probe `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf` haelt ihn
    fest. Eine Ordnungszahl steht hier bewusst nicht: sie altert mit jedem
    neuen Eintrag, die Regel darueber nicht.

Zwei Zahlen sind damit gefallen. „Mit einem **dritten** Eintrag" war die gemeldete; „alle
**drei** Befehle tragen `Wirkungsbereich::Ueberall`" stand im Satz davor und wäre beim nächsten
Eintrag auf dieselbe Weise falsch geworden, obwohl sie heute stimmt. Beide sagen jetzt „jeder
Eintrag der Liste" beziehungsweise „ein Eintrag der Liste". Die Ordnungszahl „dritter
Bestandteil" ist geblieben: sie zeigt auf die vier durchnummerierten Bestandteile des
Abschnitts darüber und ist keine Zählung einer wachsenden Liste.

Die Aussage ist am Baum nachgeprüft: `Kommando::wirkungsbereich`
(`krk-core/src/tasten/belegung.rs`) ordnet `FensterEinblenden`, `FensterSchliessen` und
`Beenden` demselben Zweig `Wirkungsbereich::Ueberall` zu.

Der Doc-Kommentar an `immer_erreichbar` selbst sagt weiter „Alle drei Eintraege stammen aus
‚kein Verlust gegenueber heute'". Er steht unmittelbar über der Liste, die er zählt, und ist
absichtlich stehen geblieben: dort ist die Zahl beim Lesen sofort zu prüfen.

`make check` — Exit 0.
