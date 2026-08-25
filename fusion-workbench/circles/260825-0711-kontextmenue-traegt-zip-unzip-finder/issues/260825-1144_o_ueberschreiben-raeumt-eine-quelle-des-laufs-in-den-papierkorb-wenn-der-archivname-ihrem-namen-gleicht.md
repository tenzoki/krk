"Ueberschreiben" raeumt eine Quelle des Laufs in den Papierkorb, wenn der Archivname ihrem Namen gleicht

---

Der Modulkopf von `zippen.rs` sagt seit dem 260825 zu: "Die Quellen des Laufs faellt dieser Zweig
ohnehin nie an." Die Begruendung daneben zeigt, dass keine der zwei Loeschstellen `auftrag.quellen`
**nennt** — das ist wahr und beantwortet die Frage nicht. Entscheidend ist nicht, welche Variable
die Stelle liest, sondern ob der **Pfadwert** `ziel` mit einem der Quellpfade zusammenfaellt. Er
kann es, und der Weg dorthin ist der zweite Zip-Lauf ueber denselben Ordner.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Rechnung, Zeile fuer Zeile

1. `crates/krk-ui/src/appkit/anwendung.rs:6112` — `zipauftrag_stellen` nimmt die betroffenen
   Eintraege unveraendert als Quellen.
2. `crates/krk-ui/src/appkit/anwendung.rs:6118` — daraus rechnet
   `kontextmenue::archivname(&auswahl.pfade, &ordner)` das Ziel.
3. `crates/krk-ui/src/kommandos/kontextmenue.rs:386-397` — bei **mehreren** betroffenen Eintraegen
   ist der Stamm der Name des angezeigten Ordners, das Ziel also
   `<angezeigter Ordner>/<Ordnername>.zip`. Belegt von der Probe
   `der_archivname_haengt_die_endung_an` (`kontextmenue.rs:686-689`): zwei Eintraege in `Projekte`
   ergeben `Projekte.zip`.
4. `crates/krk-ui/src/appkit/anwendung.rs:6122-6127` — `Auftrag::zippen(auswahl.pfade, ziel)`; das
   Ziel wird aus den Quellen **nicht** herausgenommen. `Auftrag::zippen`
   (`crates/krk-core/src/operation/auftrag.rs:160-162`) nimmt beide Listen, wie sie kommen.
5. `crates/krk-core/src/operation/zippen.rs:224` — `Konfliktantwort::Ueberschreiben` ruft
   `papierkorb.in_den_papierkorb(ziel)`, ohne `ziel` gegen `auftrag.quellen` zu halten.

## Der Ablauf am Bildschirm

Im Ordner `Projekte` steht `a.txt`. Der Nutzer markiert mehrere Eintraege und waehlt Zip; es
entsteht `Projekte/Projekte.zip`. Beim naechsten Mal markiert er wieder mehrere Eintraege, und
`Projekte.zip` ist einer davon — es steht seit dem ersten Lauf in derselben Liste. Der Archivname
ist erneut `Projekte/Projekte.zip`, das Konfliktblatt geht auf, und "Ueberschreiben" raeumt eine
**Quelle des Laufs** in den Papierkorb. Der Lauf packt sie danach nicht mehr, sondern meldet sie
als ausgelassen.

Zerstoert ist damit nichts — seit der Behebung von `260825-0942` geht der Eintrag in den
Papierkorb und nicht mehr durch `baum_entfernen`. Falsch ist die **Zusage**: der Nutzer hat sie
ausdruecklich gegeben, sie steht als eigener Abschnitt im Modulkopf, und sie haelt in genau dem
Fall nicht, den ein zweiter Zip-Lauf ueber denselben Ordner von selbst herstellt.

## Dieselbe Gestalt beim Entpacken, und diese Runde stellt sie selbst her

Die vierte Nutzerentscheidung haengt die Endung an, statt sie zu ersetzen: aus `a.zip` wird
`a.zip.zip`. Danach stehen `a.zip` und `a.zip.zip` nebeneinander. Werden beide markiert und mit
Unzip genommen, rechnet `kontextmenue::paar` (`kontextmenue.rs:503-506`) fuer das zweite Archiv den
Zielordner `<ordner>/a.zip` — also den Pfad der **ersten Quelle desselben Laufs**. Sie liegt beim
zweiten Archiv noch da, das Konfliktblatt geht auf, und "Ueberschreiben" raeumt sie weg
(`crates/krk-core/src/operation/entpacken.rs`, `zielordner_klaeren`). Der Unterschied zum Packen:
das Blatt nennt den Zielpfad, der Nutzer sieht ihn also. Eine geschriebene Zusage steht dort nicht
dagegen.

## Warum die Probe es nicht sieht

`ueberschreiben_raeumt_allein_den_gleichnamigen_eintrag_in_den_papierkorb`
(`crates/krk-core/tests/operation.rs:1488-1530`) baut den Nachbarn ausdruecklich so, dass er
**anders** heisst als das Archiv: die Quelle ist `Projekte`, das Ziel `Projekte.zip`. Sie belegt
damit die eine Haelfte der Zusage ("der aehnlich heissende Nachbar bleibt") und laesst die andere
("die Quellen des Laufs nie") ungeprueft, obwohl gerade diese Haelfte im Modulkopf als die
selbstverstaendliche dasteht.

## Vorschlag

Zwei Wege, und der zweite ist der kleinere.

1. **Der Kern haelt die Zusage.** `zielarchiv_klaeren` weist "Ueberschreiben" ab, wenn `ziel` einen
   Pfad aus `auftrag.quellen` trifft, und meldet es in der Abschlussliste. Der Pfadvergleich ist
   nicht ohne Tuecke (Verknuepfungen, verschiedene Schreibweisen desselben Pfades), und der Kern
   bekommt eine Regel, die aus der Oberflaeche stammt.
2. **Die Oberflaeche legt das Ziel nicht auf eine Quelle.** `zipauftrag_stellen` nimmt einen
   Eintrag, dessen Pfad dem gerechneten Archivnamen gleicht, aus den Quellen heraus — er waere
   ohnehin das Archiv des vorigen Laufs und gehoert selten hinein — oder rechnet einen freien Namen
   daneben. Die Stelle kennt beide Listen und ist die einzige, die sie bildet.

In beiden Faellen gehoert die Probe dazu, die heute fehlt: ein Lauf, dessen Ziel einer seiner
Quellen gleicht, und die Zusage, dass diese Quelle danach noch dasteht. Und der Modulkopf ist
nachzuziehen: das Argument "keine Stelle nennt `auftrag.quellen`" traegt die Zusage nicht und soll
nicht so aussehen, als traege es sie.

Faellt der Entschluss, die Zusage in dieser Form gar nicht zu geben, ist der Modulkopf die Stelle,
an der das steht — dann aber als Aussage ueber den Namen und nicht ueber die Quellen.

## Umfang

`krk-ui`, `appkit/anwendung.rs` (`zipauftrag_stellen`) oder `krk-core`, `operation/zippen.rs`
(`zielarchiv_klaeren`), je nach gewaehltem Weg; dazu `crates/krk-core/tests/operation.rs`. Die
Entpack-Gestalt liegt in `kommandos/kontextmenue.rs` und `operation/entpacken.rs`.
