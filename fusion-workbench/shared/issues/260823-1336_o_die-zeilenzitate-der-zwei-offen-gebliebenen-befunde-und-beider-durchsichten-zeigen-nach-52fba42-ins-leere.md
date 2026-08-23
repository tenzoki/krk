Die Zeilenzitate der zwei offen gebliebenen Befunde und beider Durchsichten zeigen nach `52fba42` ins Leere

---

`52fba42` hat `crates/krk-ui/src/appkit/anwendung.rs` um rund 220 Zeilen verlängert. Jede
Zeilenangabe in `anwendung.rs`, die vor diesem Commit geschrieben wurde, zeigt seither auf eine
andere Zeile. Betroffen sind die beiden Datensätze, die offen geblieben sind und deshalb noch
gelesen werden — `260823-0731` und `260823-0732` —, und beide Durchsichtsberichte der Sitzung.
Der Versatz beträgt zwischen 59 und 168 Zeilen und ist nicht gleichmäßig, also auch nicht durch
eine einzige Verschiebung im Kopf zu korrigieren.

---

**Gemessen am Baumstand `616ad5e`, Stichprobe von fünfzehn Angaben; keine einzige traf.**

## Die Stichprobe

| Angabe | was dort stehen sollte | was dort steht | trägt es heute |
|---|---|---|---|
| `anwendung.rs:4320` | `fn aktives_setzen` | eine Zeile Doc-Kommentar über den Editor | `:4379` |
| `anwendung.rs:4507` | `fn bildschirmbreiten_uebernehmen` | Doc-Kommentar über den Ereignisabgriff | `:4578` |
| `anwendung.rs:4530` | `fn aufteilung_nachziehen` | eine leere Doc-Zeile | `:4601` |
| `anwendung.rs:7061` | die Messung in `sitzung_bauen` | der Doc-Kommentar „Keine zweite Fokusabfrage" | `:7229` |
| `anwendung.rs:4194` | der Rumpf von `sichtbarkeit_aendern` | ein Doc-Kommentar über `bildschirmbreiten_uebernehmen` | `:4253` |
| `anwendung.rs:8043` | `mod fokusnachzugproben` | `.sum();` aus einer fremden Probe | `:8217` |
| `anwendung.rs:1130-1131` | die zwei Empfänger des Melders | Kommentartext über den schwachen Rückruf | `:1163-1164` |

Die übrigen acht verhalten sich gleich. Die vollständige Umrechnung für die zwei offenen
Datensätze steht dort jeweils als Tafel im Abgleichsvermerk vom 260823-1336; dieser Datensatz
führt sie nicht doppelt.

## Warum das hier steht und nicht bloß in den zwei Datensätzen

Es ist die **dritte** Serie derselben Sorte in einer Sitzung, und die ersten beiden sind je als
Befund abgelegt worden: `260823-0730` (drei Prosastellen, mit `df8163d` falsch geworden) und
`260823-1032` nebst den fünf abhängigen Stellen, die `52fba42` nachgezogen hat. Beide Male ging
es um Prosa **im Code**, die eine Menge oder eine Anzahl nannte. Diese dritte Serie ist dasselbe
in der Workbench: eine Zahl, die eine andere Datei beschreibt und mit deren nächster Änderung
falsch wird.

**Der Unterschied ist, dass hier keine Behebung an der Zahl hilft.** Wer die fünfzehn Angaben
umrechnet, hat sie bis zum nächsten Commit an `anwendung.rs` richtig. Der Baum hat für genau
diese Gestalt schon zweimal die andere Antwort gewählt: an `bildschirmbreiten_uebernehmen` steht
seit `52fba42` eine Regel statt einer Aufzählung, und im Modulkopf der Kommandos sind zwei
Zahlen gestrichen statt korrigiert.

## Was zu entscheiden wäre

Ob ein Datensatz, der auf Code zeigt, den Ort über den **Namen** benennt (Funktion, Modul,
Probe) statt über die Zeilennummer, und die Zeilennummer allenfalls als Beifang führt. Die
Gegenrechnung: eine Zeilennummer ist beim Lesen billiger, und ein Name ist nicht überall
eindeutig. Das gehört entschieden und nicht nebenbei gegriffen; solange es nicht entschieden
ist, kostet jede Durchsicht dem nächsten Leser die Suche.

**Schwere:** niedrig für den Inhalt, mittel für die Nachverfolgbarkeit. Kein Befund ist falsch
geworden; jeder ist nur teurer zu prüfen.

**Gefunden:** reconciler, Abgleich der Sitzung `260823-0442` am 260823-1336, Bereich
`ab11eb8..616ad5e`

**Betroffen:** `shared/issues/260823-0731_*`, `shared/issues/260823-0732_*`,
`shared/reviews/260823-0735-coderev-einblenden-erreicht-den-schirm.md`,
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`

**Domain:** code

**Verwandt:**
`shared/issues/260823-0730_*_drei-prosastellen-um-den-neuen-nachzug-*` — die erste Serie.
`shared/issues/260823-1032_*_zwei-zahlen-im-modulkopf-der-kommandos-*` — die zweite, dort mit
der Antwort „Regel statt Zahl".
`shared/decisions/260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`
— dieselbe Frage für den Marker im Dateinamen statt für die Zeilennummer im Code.
