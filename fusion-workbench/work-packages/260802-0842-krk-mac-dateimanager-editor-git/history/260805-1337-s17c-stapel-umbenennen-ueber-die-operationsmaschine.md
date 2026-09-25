# S17c: das Stapel-Umbenennen läuft über die Operationsmaschine

**Status:** Complete
**Ausführender:** coder
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 17c
**Defekt:** `issues/260804-2040_o_das-stapel-umbenennen-laeuft-ohne-fortschritt-und-ohne-abbruch-auf-dem-hauptfaden.md`

## Was gebaut ist

Das Stapel-Umbenennen ist von der Schleife auf dem Hauptfaden auf die Operationsmaschine
aus S15 umgezogen. Dazugekommen sind ein Wert in der Aufzählung `Art` und ein Durchlauf
über eine Liste; Arbeitsfaden, Abbruchkennzeichen, Fortschrittskanal und die Regel, dass
eine gescheiterte Einzelposition den Stapel nicht abbricht, bringt die Maschine mit.

`Art::UmbenennenImStapel { neue_namen }` trägt die neuen Namen Stelle für Stelle zu
`Auftrag::quellen`. Zwei Listen und keine Liste aus Paaren, weil die Maschine über
`quellen` läuft wie bei jeder anderen Art; aneinander gebunden werden sie von
`Auftrag::umbenennen_im_stapel`, das die Paare auftrennt. Ein Aufrufer kann sie deshalb
nicht gegeneinander verschieben.

## Vier Abnahmekommandos, alle mit 0

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets` (0 Warnungen), `cargo fmt --all --check`. Der Testlauf zählt 372 Prüfungen,
0 gescheitert, 1 übersprungen; vor dem Schritt waren es 367.

## Die Abnahmepunkte einzeln

| Punkt | Beleg |
|---|---|
| `cargo test -p krk-core --test operation` mit 0 | 26 Prüfungen, davon drei neue, dazu zwei in `operation::auftrag` |
| Ein Stapel über 5.000 Namen läuft durch | `ein_stapel_ueber_5000_namen_laeuft_durch`: Abschluss `Fertig`, 5.000 Einträge, nichts übersprungen, alle neuen Namen auf der Platte |
| Ein Abbruch nach dem tausendsten Eintrag kehrt binnen 100 ms zurück und meldet die Zahl | `ein_abbruch_im_stapel_kehrt_binnen_100_ms_zurueck_und_meldet_die_umbenannten`: gemessen vom Abbruch bis zum Bericht, dazu die Gegenprobe, dass genau so viele Dateien unter dem neuen Namen stehen, wie der Bericht meldet |
| Ein Eintrag ohne Schreibrecht wird übersprungen und mit Grund gemeldet, die übrigen laufen durch | `ein_eintrag_ohne_schreibrecht_im_ordner_wird_uebersprungen_und_die_uebrigen_laufen_durch`: ein Eintrag mit Grund `keine Rechte`, fünf umbenannt, Abschluss `Fertig` |
| Im Bündel: ein Stapel über 5.000 Einträge zeigt den Fortschritt in der Statuszeile des Fensters, das ihn begonnen hat | Bildschirmfotos 260805-1325 während des Laufs: `Umbenennen: 2.496 Einträge, 2,4 kB, 5.000 ausgewählte Positionen · neu-02495.txt · Esc bricht ab`, in gewöhnlicher Textfarbe. Vier aufeinanderfolgende Aufnahmen zeigen die Zahl steigen (2.496, 2.608, 2.986, 3.372) |
| `esc` bricht ab | Während des Laufs Esc gesendet: `Umbenennen abgebrochen: 2.648 Einträge, 2,6 kB (5.000 ausgewählte Positionen) übertragen`. Gegenprobe auf der Platte: genau 2.648 Dateien tragen den neuen Namen, 2.352 den alten |
| Navigation, Markierung und Tabwechsel bleiben bedienbar | Teilweise belegt, siehe unten |
| Ein Stapel über 50 Einträge lässt keine Zeile aufblitzen | Vier Aufnahmen unmittelbar nach dem Ausführungsbefehl: die ersten beiden zeigen den Markierungsstand aus S16c, die letzte den Abschlusstext. Eine Vorgangsanzeige steht in keiner |
| Der Diff zeigt, dass `stapel_ausfuehren` verschwunden ist | `grep -rn "stapel_ausfuehren" crates/` liefert keinen Treffer |
| Genau ein Weg führt eine Umbenennung aus | `grep -rn "operation::umbenennen(" crates/krk-ui/src/` liefert genau eine Stelle, `anwendung.rs:1050` aus S17b. Der Stapel geht über `umbenennen::eintrag_umbenennen` im Kern, und das ruft dieselbe Funktion `umbenennen`; ein zweiter Umbenennungsweg besteht nicht |

## Was ich nicht belegen kann, und was ich stattdessen gefunden habe

**Die Bedienbarkeit während des Laufs ist nur zur Hälfte belegt.** Das Fenster nimmt
Tastendrücke an, zeichnet, und `esc` erreicht den Abbruch (siehe oben). Was ich **nicht**
zeigen konnte, ist die Auswahl, die während des Laufs wandert: die Dateiliste des Fensters
ist während des ganzen Stapels **leer**. Zwei Aufnahmen während desselben Laufs zeigen
dasselbe Bild.

Defekt dafür angelegt:
`issues/260805-1337_o_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`.
Vermutete Ursache ist die Dateisystembeobachtung aus C9: jede Umbenennung ändert den
angezeigten Ordner, FSEvents meldet es, und der neue Lesevorgang leert das Modell, bevor
der nächste ihn ablöst. Bis S17c konnte das nicht auftreten, weil der stehende Hauptfaden
jede Auffrischung bis zum Ende der Schleife aufhielt; beim Kopieren und Verschieben tritt
es nicht auf, weil sich dort der Ordner des **anderen** Dateifensters ändert. Der Defekt
ist damit eine Folge dieses Schrittes und keine verfehlte Zusage: C4 sagt Bedienbarkeit zu,
und bedienbar ist die Oberfläche.

## Zwei Entwurfsentscheidungen, die der Plan offenließ

**Der Abschlusstext ist der gemeinsame und kein eigener.** `operationen::stapelbericht`
zählte umbenannte, stehengebliebene und gescheiterte Einträge selbst zusammen; das ging,
solange die Schleife auf dem Hauptfaden lief und ihr Ergebnis unmittelbar vorlag. Über die
Operationsmaschine kommen umbenannte und gescheiterte aus dem Bericht, und ein zweiter
Abschlusstext daneben wären zwei Wahrheiten über denselben Vorgang. `stapelbericht` ist
deshalb entfallen. **Die Zahl der stehengebliebenen Zeilen sagt jetzt der vorhandene Text
über seine beiden Zahlen:** `Umbenennen fertig: 48 Einträge, … (50 ausgewählte
Positionen)`. Die Zahl der Positionen ist die der bestätigten Vorschauzeilen, die der
Einträge die der wirklich umbenannten. Genau diese zwei Zahlen führt `vorgangszeile` seit
S16 mit derselben Begründung.

**Die Auswahl nach dem Stapel braucht kein neues Feld.** Sie steht danach auf dem ersten
neuen Namen, wie vorher. Der Name kommt aus dem Auftrag selbst: `Art::UmbenennenImStapel`
trägt die Liste, und `Vorgang` trägt die Art. Ein eigenes Feld in `Vorgang` wäre ein
Zustand mehr für eine Angabe, die schon dasteht.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/operation/auftrag.rs` | `Art::UmbenennenImStapel`, `Auftrag::umbenennen_im_stapel`, `neuer_name`, `zielordner` um den neuen Wert, zwei Prüfungen |
| `crates/krk-core/src/operation/umbenennen.rs` | `eintrag_umbenennen` je Eintrag, Modulkopf nachgezogen |
| `crates/krk-core/src/operation/mod.rs` | die Stelle läuft im Durchlauf mit, neue Art in der Auswahl der Ausführung |
| `crates/krk-core/tests/operation.rs` | drei Prüfungen und der Aufbau `stapel_anlegen` |
| `crates/krk-ui/src/kommandos/operationen.rs` | `ueberschrift` um die neue Art, `stapelbericht` entfällt samt Prüfung, dafür eine Prüfung am gemeinsamen Abschlusstext |
| `crates/krk-ui/src/appkit/anwendung.rs` | `stapel_ausfuehren` entfällt, `stapel_beauftragen` kommt, `auftrag_stellen` in `vorgang_laeuft_schon` und `auftrag_starten` geteilt, `vorgang_beenden` setzt die Auswahl auf den ersten neuen Namen |

`crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs` **blieb unberührt**, obwohl der Plan
es nennt. Die Zusage "das Blatt schließt sich mit dem Ausführungsbefehl" hält schon:
`beginSheetModalForWindow:completionHandler:` ruft den Abschlussblock, nachdem das Blatt
weg ist, und der Block ist die einzige Stelle, die den Auftrag stellt. Was sich geändert
hat, ist nicht das Blatt, sondern was danach geschieht.

## Prüfdaten

`/tmp/krk-s17c-gross` (5.000 Dateien), `/tmp/krk-s17c-klein` (50 Dateien) und die
Bildschirmfotos unter `/tmp`, alle selbst angelegt und am Ende der Sitzung entfernt.
