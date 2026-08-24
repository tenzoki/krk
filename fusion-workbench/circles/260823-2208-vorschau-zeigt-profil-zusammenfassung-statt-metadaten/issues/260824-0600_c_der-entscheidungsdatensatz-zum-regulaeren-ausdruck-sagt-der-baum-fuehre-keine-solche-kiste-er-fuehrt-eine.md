Der Entscheidungsdatensatz zum regulären Ausdruck sagt, der Baum führe keine solche Kiste; er führt eine

---

`decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`
nennt unter den Kosten der Möglichkeit 3: „Der Baum führt heute keine Kiste für reguläre
Ausdrücke (`Cargo.lock` am 260824-0541 nachgesehen)". Die Aussage ist falsch. `krk-ui` führt
`fancy-regex` 0.16.2 als gewöhnliche Abhängigkeit über `syntect`, und darunter stehen
`regex-automata` 0.4.18, `regex-syntax` 0.8.11, `aho-corasick` 1.1.5 und `memchr` 2.8.3.
Nachgezählt am 260824-0600 mit `cargo tree -p krk-ui -e normal`. Der Filter der ersten
Erhebung hat auf den Namen `regex` gesehen und die vier Pakete übersehen, die keinen solchen
Namen tragen.

---

**Warum es zählt.** Der Nutzer hat am 260824-0555 Möglichkeit 3 gewählt und dabei eine der
zwei genannten Kosten ausdrücklich in Kauf genommen, nämlich die fremde Kiste. Diese Kosten
sind kleiner als der Datensatz sie beziffert: die Maschinerie steht bereits im Bündel, und
`regex` 1.x setzt genau auf die vier Pakete auf, die schon dastehen, wäre also ein einziges
neues Paket und kein Zuwachs am Übersetzungsaufwand der Abhängigkeiten. Die Wahl kehrt sich
dadurch nicht um, sie wird billiger; deshalb ist der Datensatz nicht überholt, sondern trägt
eine falsche Angabe in seiner Kostenaufstellung.

**Die Angabe stand schon im Baum, an einer Stelle, die dieses Projekt als bindend führt.**
Die Wurzel-`Cargo.toml` zählt in ihrer Begründung zu `syntect` die einundzwanzig
mitgezogenen Pakete namentlich auf, `fancy-regex`, `regex-automata`, `regex-syntax`,
`aho-corasick` und `memchr` darunter. Wer die Begründung gelesen hätte, hätte `Cargo.lock`
nicht befragen müssen.

**Nicht berührt** ist die zweite Zusage desselben Absatzes: `Cargo.lock` führt am
260824-0600 kein `cc` und außer `windows-sys` kein `-sys`-Paket, und `windows-sys` baut auf
dem Bauziel nicht. Auch `regex` brächte keinen C-Code herein.

**Filed by:** shaper

---
Resolved: die Kostenangabe ist im Datensatz berichtigt; die Wahl vom 260824-0555 bleibt bestehen. Der Absatz `**Berichtigung 260824-0910.**` steht in `decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md` hinter dem Abschnitt `## Options`; der Wortlaut der Cons-Aufzählung bleibt als Beleg der Wahlgrundlage stehen, und der Marker des Datensatzes bleibt auf `_a_`. Nachgemessen am 260824-0910 mit `cargo tree -p krk-ui -e normal` und `git show HEAD:Cargo.lock`.
