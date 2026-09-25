Die Meldung zum verschwundenen Lesezeichen behauptet eine Löschung

---

`Lesezeichenliste::anwenden` liefert `Ausgang::Verschwunden`, wenn
`stelle_von(welches)` das genannte Lesezeichen in der frisch gelesenen Liste nicht findet
(`crates/krk-core/src/ablage/lesezeichen.rs:459-475`). Der Delegierte macht daraus einen
Satz für die Statuszeile:

> `crates/krk-ui/src/appkit/anwendung.rs:1556-1560` — „das Lesezeichen gibt es nicht mehr;
> eine andere Instanz von KRK hat es **geloescht**"

**Der Ausgang trägt mehr Fälle als das Löschen.** `stelle_von` vergleicht den **ganzen**
Eintrag, Name und Ziel (`lesezeichen.rs:428-431`). Hat die andere Instanz das Lesezeichen
umbenannt oder sein Ziel geändert, findet die Suche es ebenso wenig, und der Nutzer liest, es
sei gelöscht worden. Der Doc-Kommentar von `Ausgang::Verschwunden` sagt es selbst enger als
der Code: „Den genannten Eintrag gibt es in der frisch gelesenen Liste nicht mehr" — das ist
richtig, die Meldung darüber ist es nicht.

Der Fall ist nicht konstruiert: Umbenennen ist einer der vier Befehle, um die es geht, und die
Runde hat die Lage gerade erst geschaffen, in der zwei Instanzen dieselbe `bookmarks.toml`
führen.

---

**Schwere:** gering. Kein Datenverlust, keine falsche Wirkung; eine Auskunft, die den Nutzer
in die falsche Richtung schickt — er sucht ein gelöschtes Lesezeichen, das umbenannt in der
Leiste steht.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:1556-1560`,
`crates/krk-core/src/ablage/lesezeichen.rs:459-475`

**Domain:** code

## Vorschlag

Den Satz auf das umschreiben, was der Ausgang wirklich sagt, etwa: „dieses Lesezeichen steht
nicht mehr so in der Liste; eine andere Instanz von KRK hat es geändert oder gelöscht". Die
Leiste zeigt in diesem Ausgang ohnehin schon die frisch gelesene Liste, der Nutzer sieht die
Änderung also unmittelbar daneben.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Der Satz in der Statuszeile heisst jetzt „dieses Lesezeichen steht nicht mehr so in der Liste; eine andere Instanz von KRK hat es geaendert oder geloescht" und deckt damit alle Faelle, die `Ausgang::Verschwunden` traegt. Ein Kommentar daneben nennt den Grund: `Lesezeichenliste::stelle_von` vergleicht den ganzen Eintrag, Name und Ziel, und findet ein umbenanntes Lesezeichen ebenso wenig wie ein geloeschtes. Weder der Ausgang noch `stelle_von` sind angefasst; der Befund betraf allein die Auskunft.
