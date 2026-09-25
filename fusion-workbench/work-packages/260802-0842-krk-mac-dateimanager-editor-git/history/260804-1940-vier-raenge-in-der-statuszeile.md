# Vier Ränge in der Statuszeile

**Status:** Complete
**Agent:** coder
**Datum:** 260804-1940
**Circle:** 260802-0842-krk-mac-dateimanager-editor-git

Behoben sind die beiden Defekte aus der Abnahme von S16b vom 260804-1915:

- `issues/260804-1915_c_der-abschlusstext-ueberschreibt-die-verdraengte-fenstermeldung.md`
- `issues/260804-1915_c_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`

Neu abgelegt: `issues/260804-1940_o_s16b-beschreibt-die-statuszeile-mit-drei-raengen-gebaut-sind-vier.md`, der Plananteil, den dieser Schritt nicht anfassen darf.

## Der eine Grund unter beiden Defekten

Beide wohnten im Feld `fenstermeldung`, und die Diagnose der Abnahme hat den Ort richtig benannt. Sie hat den Grund noch nicht benannt: **das Feld trug zwei verschiedene Sorten von Aussage.** Die eine ist ein Ereignis, das der Nutzer nicht angefordert hat, der ausgeworfene Datenträger und die beschädigte Belegungsdatei. Die andere ist die Antwort auf einen Tastenbefehl, den er eben gemacht hat: "es läuft bereits eine Operation", "es ist nichts ausgewählt", "die Zwischenablage ist leer", der Abschlusstext eines Vorgangs.

Die beiden haben verschiedene Lebensdauern und verschiedene Dringlichkeit. In einem Feld mit einem Rang ließen sie sich nicht beide halten, und daran sind beide Defekte hängengeblieben. Der zweite F5 erbte den Rang eines Ereignisses und verschwand deshalb hinter dem eigenen Fortschritt; der Abschlusstext überschrieb die Auswurfmeldung, weil beide in dasselbe Feld schrieben.

Der Plan hat vor genau dieser Lage gewarnt, an derselben Stelle, an der er die Vorgangsanzeige ihr eigenes Feld bekommen lässt: "Ein Feld mit zwei Löschregeln wäre genau der Sonderfall, den die Maxime supersimpel ausschließt." Der Satz galt auch für die Trennung, die S16b nicht gezogen hat.

## Die Regel, die jetzt gilt

**Verdrängt wird nichts gelöscht.** Jede Aussage steht in ihrem eigenen Feld, bis ihre eigene Lebensdauer endet; die Zeile zeigt die oberste, die noch steht. Eine verdrängte Aussage erscheint, sobald alles über ihr gefallen ist.

Die Rangfolge ist die Nähe zum letzten Tun des Nutzers:

```
Rang  Quelle             Was sie sagt                          Fällt
────  ─────────────────  ────────────────────────────────────  ──────────────────────────
 1    Befehlsantwort     was KRK auf einen Tastenbefehl        mit dem nächsten
                         zu sagen hat                          Tastenbefehl
 2    Vorgangsanzeige    der Stand einer laufenden Operation   mit dem Bericht
 3    Fenstermeldung     ein Ereignis am Fenster, das          beim nächsten Ordner-
                         niemand angefordert hat               oder Tabwechsel
 4    Tabmeldung         der Zustand des sichtbaren Ordners    mit der Ordnerliste
```

Das ist kein neues Ordnungsprinzip, sondern das vorhandene zu Ende geführt. S14 stellt die Fenstermeldung über die Tabmeldung, "weil ein Ereignis neuer ist als ein Zustand". S16b stellt die Vorgangsanzeige darüber, weil eine laufende Operation neuer ist als ein Ereignis. Die Abnahme hat den nächsten Schritt selbst schon aufgeschrieben: eine Antwort auf einen Tastendruck, den der Nutzer gerade gemacht hat, ist neuer als beides. Nach demselben Prinzip steht sie oben.

**Die drei Randbedingungen halten.**

- **Kein Zeitgeber.** Jede der vier Lebensdauern hängt an einem Ereignis: an einem Tastenbefehl, an einem Bericht, an einem Ordnerwechsel, an einer Ordnerliste. Es gibt keine Uhr, kein Wecken im Leerlauf, keinen zusätzlichen Lebenszyklus. Der vierte Rang bringt ausdrücklich nicht die "eigene Lebensdauer" mit, vor der der Defekteintrag gewarnt hat.
- **Keine zweite Zeile.** Die Statuszeile bleibt einzeilig und trägt weiterhin genau einen Text.
- **Eine Regel, kein Sonderfall je Meldungsart.** Die Auswahl steht in einer Funktion, `statuszeile::zeile`, die vier `Option<&str>` nimmt und eines davon zurückgibt. Kein Zweig darin fragt, um welche Art von Meldung es geht.

## Der Preis, ehrlich benannt

Zwei Texte nacheinander gehen ohne Zeitgeber nicht: es gibt keine Uhr, die den ersten nach einer Weile durch den zweiten ersetzt. Die Zeile kann sie nur staffeln, und das Staffelmaß ist das Tun des Nutzers.

Damit hält die Zusage des Plans der Sache nach, nicht im Wortlaut. Die Auswurfmeldung ist **nicht verloren**, sie erscheint. Sie erscheint aber nicht "sobald die Vorgangsanzeige endet": in diesem Augenblick steht der Abschlusstext in der Zeile, und der ist eine Befehlsantwort. Sie erscheint einen Tastenbefehl später. Der Plananteil ist als Defekt abgelegt und nicht selbst geändert.

Der zweite Preis ist kleiner und läuft in die andere Richtung: solange die Meldung "es läuft bereits eine Operation" steht, ist der Fortschritt verdeckt. Das endet mit dem nächsten Tastendruck, und einer folgt gewöhnlich sofort, weil der Nutzer eben einen gemacht hat. Am Bündel gemessen: nach einem Pfeil ab stand der Fortschritt wieder da.

## Was geändert wurde

| Datei | Was |
|---|---|
| `crates/krk-ui/src/appkit/statuszeile.rs` | Die Regel als Funktion `zeile`, ohne AppKit in der Signatur und damit prüfbar. Sechs Prüfungen, darunter je eine je Defekt. |
| `crates/krk-ui/src/appkit/tabelle.rs` | Das Feld `befehlsantwort` in `QuelleIvars`, die Zugänge `befehlsantwort_zeigen` und `befehlsantwort_loeschen`, `meldung_anzeigen` als dünner Aufrufer der Regel. Fünf Meldungen der Datenquelle sind von `meldung_zeigen` auf `befehlsantwort_zeigen` umgestellt. |
| `crates/krk-ui/src/appkit/anwendung.rs` | `melden` heißt `antwort_zeigen` und schreibt in den obersten Rang; fünf Aufrufer mit umgestellt, darunter der Abschlusstext. Die eine Löschregel steht in `kommando_ausfuehren`. Zwei Meldungen gehen an das aktive statt fest an das linke Dateifenster. |
| `crates/krk-ui/src/kommandos/operationen.rs` | Unverändert. Der Text der Meldung war nie das Problem, nur die Zeile, in der er landete. |

**Die Entscheidung steht weiterhin an genau einer Stelle**, wie das Abnahmekriterium von S16b es verlangt. Sie ist von `meldung_anzeigen` in `statuszeile::zeile` gezogen, weil sie dort ohne ein laufendes AppKit prüfbar ist; `meldung_anzeigen` liest die vier Felder und entscheidet selbst nichts.

**Zwei weitere Meldungen standen in der falschen Zeile**, aus einem verwandten, aber anderen Grund: nicht vom Rang verdrängt, sondern in der falschen Seite gelandet. Die Startmeldungen über eine beschädigte Belegungs- oder Sitzungsdatei und die Meldung über einen gescheiterten Schreibvorgang der Sitzung gingen fest an das linke Dateifenster. Beide betreffen die Anwendung und keine Seite; sie gehen jetzt an das aktive, dieselbe Wahl, die die fehlgeschlagene Dateisystembeobachtung schon vorher traf. Hat die Sitzung das rechte Dateifenster als aktiv wiederhergestellt, sah der Nutzer sie vorher in der Zeile, auf die er nicht blickt.

## Die Messung am laufenden Bündel

Prüfordner mit 30.000 Einträgen in 30 Unterordnern unter `/tmp`, kopiert auf denselben APFS-Datenträger, hinterher entfernt.

**Ein zweiter Operationsbefehl während eines laufenden Vorgangs, im Fenster des Vorgangs:**

| Zeitpunkt | Zeile |
|---|---|
| während der Kopie | `Kopieren: 801 Einträge, 19,2 kB, eine ausgewählte Position · datei-0268.txt · Esc bricht ab` |
| nach dem zweiten F5 | `es läuft bereits eine Operation: Kopieren` |
| nach dem nächsten Tastenbefehl | `Kopieren: 2.201 Einträge, 52,7 kB, eine ausgewählte Position · datei-0313.txt · Esc bricht ab` |
| nach dem Ende | `Kopieren abgebrochen: 2.214 Einträge, 53,0 kB (eine ausgewählte Position) übertragen` |

Gestartet hat der zweite Befehl nichts: 801, 2.201, 2.214 ist eine ununterbrochen laufende Zählung eines Vorgangs, und am Ende steht genau ein Abschlusstext.

**Auswurf während einer laufenden Kopie:**

| Zeitpunkt | Zeile |
|---|---|
| während der Kopie, vor der Meldung | `Kopieren: 787 Einträge, 18,8 kB, eine ausgewählte Position · datei-0915.txt · Esc bricht ab` |
| unmittelbar nach der Auswurfmeldung | `Kopieren: 804 Einträge, 19,2 kB, eine ausgewählte Position · datei-0526.txt · Esc bricht ab` |
| nach dem Ende der Kopie | `Kopieren abgebrochen: 842 Einträge, 20,2 kB (eine ausgewählte Position) übertragen` |
| nach dem nächsten Tastenbefehl | `SICHERUNG wurde ausgeworfen; das Dateifenster zeigt jetzt /Users/k1` |

Die Auswurfmeldung ist über denselben Eingang gesetzt, den `auffrischung::datentraeger_verloren` benutzt, nämlich die Methode `melden` der Schnittstelle `Dateifenstersicht`. Ein Datenträger wurde dafür nicht körperlich ausgeworfen; was diese Messung belegt, ist der Weg der Meldung durch die Rangfolge und nicht die Erkennung des Auswurfs, die S14 gemessen hat.

## Die Sonde

`osascript` darf in dieser Sitzung keine Tastatureingaben senden; gefahren wurde deshalb wieder eine vorübergehende Sonde, geschaltet über `KRK_SONDE`, mit den Ordnern in `KRK_SONDE_QUELLE` und `KRK_SONDE_ZIEL`. Sie lag ausschließlich in `crates/krk-ui/src/appkit/anwendung.rs`: ein Zustandsautomat auf einem 5-ms-Zeitgeber, ein Sender für synthetische Tastenereignisse und ein Rücklesen des Textfelds der Statuszeile über `Dateifenster::statuszeile_sicht`.

Sie ist **vollständig zurückgenommen**. `grep -rniE 'KRK_SONDE|VORUEBERGEHENDE|sondeSchritt|sonde_|sondentakt|SONDE_CODE' crates/ xtask/ resources/` liefert null Treffer, und das Bündel ist danach ohne sie neu gebaut. Der Hinweis der vorigen Sonde hat sich bestätigt: eine Funktionstaste braucht ihr wirkliches Zeichen (F5 ist `\u{F708}`) und die Marke `function`, sonst erreicht das Ereignis den lokalen Abgriff nicht.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0, keine Warnung |
| `cargo test --workspace` | 0, 312 Prüfungen, davon 6 neue zur Rangfolge |
| Auswurf während laufender Kopie | die Meldung erscheint, einen Tastenbefehl nach dem Abschlusstext |
| zweiter Operationsbefehl | sichtbar, und gestartet wird nichts |

Nicht angefasst: `crates/krk-core/`, `crates/krk-bench/`, `xtask/`, `resources/`, die Plandatei, der Spec. Nicht committet.

## Die sieben Abnahmepunkte von S16b

Aus meiner Sicht trägt S16b jetzt sechs von sieben. Der siebte ist keiner, den dieser Schritt behebt.

| Punkt | Stand |
|---|---|
| kein Blatt während der Kopie, Fortschritt in der Statuszeile | hält, seit 260804-1915 |
| Navigation, Markierung, Tabwechsel bedienbar, `esc` bricht ab | hält, seit 260804-1915 |
| eine Kopie von 3 kleinen Dateien lässt keine Zeile aufblitzen | hält, seit 260804-1915 |
| L8 unter 200 ms, 95. Perzentil | hält, gemessen am 260804-1915 mit 168 ms |
| der Abschlusstext überlebt die Auffrischung | hält, und jetzt zusätzlich belegt: eine Auffrischung ist kein Tastenbefehl |
| die Auswurfmeldung erscheint nach dem Fortschritt | hält der Sache nach, verschoben um einen Tastenbefehl |
| der zweite Operationsbefehl meldet sich und startet nichts | hält jetzt vollständig |

Offen bleibt allein der Plantext: `cargo xtask messen` gibt es nicht (`issues/260804-1915_o_das-abnahmekriterium-von-s16b-nennt-cargo-xtask-messen-das-es-nicht-gibt.md`), und die Beschreibung der Statuszeile nennt drei Ränge statt vier (`issues/260804-1940_o_s16b-beschreibt-die-statuszeile-mit-drei-raengen-gebaut-sind-vier.md`). Beide gehören dem `planner`. **Das `[DONE]` von S16b hat dieser Schritt nicht gesetzt**, weil die Plandatei außerhalb seiner Grenzen liegt.
