# Durchsicht: Bündel B, kein Löschen ohne Papierkorb, und die Stufenregel

**Sender:** coderev
**Datum:** 260817-1419
**Reviewed-range:** `3fcd375..ee85950`
**Not-opened:** none

**Der Bereich als Aufzählung seiner fünf Commits**, älteste zuerst, weil `A..B` in der
Schreibweise von git den Commit `A` ausschließt und diese Verwechslung diesem Projekt schon
einen Datensatz gekostet hat (`shared/issues/260817-1122_o_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md`):

1. `873b9f4` fix(ui): die Rueckfallstelle eines Blattes ist die abbrechende und nicht die letzte
2. `8c18887` docs(ui): vier Korrekturen an der Prosa des Loeschwegs, und ein Attribut
3. `4b50cc1` feat(core): der dreiwertige Befund traegt die dritte Antwort
4. `e2760cd` feat(ui): die Huelle um den Papierkorb fragt vorher, ob das Ziel einen fuehrt
5. `ee85950` feat(ui): kein Loeschen ohne Papierkorb, und die Stufenfolge ist eine pruefbare Regel

`3fcd375` ist der Sitzungsanfang und selbst nicht Teil der Durchsicht; er steht als `<from>`,
damit die Zeile in der Schreibweise von git denselben Bereich bezeichnet wie die Aufzählung.

**Zur Anschlussfähigkeit an die vorige Durchsicht.** Sie deckt `664a0fd..472eb81`. Zwischen
`472eb81` und `3fcd375` liegen drei Commits (`a8b4bf8`, `6ff96b1`, `3fcd375`), und sie berühren
keine Codedatei; nachgezählt mit `git show --numstat --format="" <commit> | awk '{print $3}' | grep -v '^fusion-workbench/'`,
kein Treffer bei allen drei. Der Codebereich der beiden Durchsichten stößt damit lückenlos
aneinander.

**Grundlage:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (Directive,
C2, C3, C4, C5), `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md` (`## Approach`,
Bündel B, Schritte 4 bis 6, 11), `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`,
`history/260817-1240…`, `…-1302…`, `…-1320…`, `…-1345…`, `…-1359…`

## Zusammenfassung

**Die Stufenfolge des Löschwegs ist aus dem Rumpf heraus und prüfbar, und die vier
Zusagen, die der Orchestrator abgenommen hat, halten am Baum.** Die Rückfallstelle eines
Blattes kommt aus der `Wirkung` und nicht mehr aus der Reihenfolge, an den zehn anderen
Blättern ändert das nachgezählt nichts, die Papierkorbfrage liegt auf der richtigen Polarität,
und `vor_der_rueckfrage` ist überschneidungsfrei, vollständig und ohne Auffangzweig. Sechs
Befunde stehen daneben, keiner davon hoch. Zwei von ihnen betreffen dieselbe Sache: die
Sicherungen, die den nächsten Fehler dieser Art fangen sollen, sind Prosa oder unwirksam,
während die Sache selbst geprüft ist.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 2 |
| Niedrig | 4 |

## Die sieben Befunde der vorigen Durchsicht, einzeln nachgelesen

Ein Abschluss ist eine Behauptung, bis jemand sie nachliest. Sechs sind nachgelesen und halten,
einer steht unverändert offen.

| # | Datensatz | Stand im Speicher | Am Baum nachgelesen |
|---|---|---|---|
| 1 | `260817-1106` unbekannte Antwort fällt auf die zerstörende Schaltfläche | `_c_` | **ja, hält.** `abbruchstelle` (`blaetter/mod.rs:416`) liest die erste Schaltfläche mit `Wirkung::Liegenlassen`; der Abschlussblock nimmt sie als Rückfall (`:686-689`), der Griff als Abbruchcode (`:700`). Für das Löschblatt ist das Stelle 0, also „Abbrechen". Zwei Proben halten es fest (`loeschbestaetigung.rs:155`, `:171`), selbst gefahren und grün. |
| 2 | `260817-1107` der Rumpf trägt keine Probe | `_c_` | **ja, zur Hälfte, und der Datensatz sagt das selbst.** Zwei der vier Eigenschaften sind mit einer Tafel über zwölf Fälle geprüft; zwei bleiben offen. Die Begründung dafür ist zu weit gefasst, siehe Befund 5. |
| 3 | `260817-1108` die Frage entsteht vor beiden Sperren | `_o_` | **ja, unverändert offen.** `in_den_papierkorb` (`anwendung.rs:4458-4461`) baut beide Texte weiter vor dem Aufruf des Rumpfes. Bündel B hat an derselben Stelle einen zweiten Fall desselben Musters hinzugefügt, siehe Befund 3. |
| 4 | `260817-1109` der Melder der Bereichsleiste als Weg in den Papierkorb | `_c_` | **ja, hält, und die berichtigte Zahl stimmt.** `loeschwarnung.rs:125-129` und `anwendung.rs:4484-4486` sagen jetzt zehn Umschalter. Selbst gezählt: `grep -o "Kommando::[A-Za-z]*" crates/krk-ui/src/appkit/bereichsleiste.rs` liefert zehn verschiedene, alle Umschalter, keiner `InPapierkorb`. Die vorige Durchsicht sagte elf und zählte zehn auf; der `coder` hat die Abweichung im Datensatz vermerkt. |
| 5 | `260817-1110` zwei Doc-Kommentare nennen `endgueltig_loeschen` | `_c_` | **ja, hält.** Beide Stellen nennen jetzt `loeschen_nach_rueckfrage`. Die Nachzählung im Datensatz stimmt: der Satz „es ist nichts ausgewählt" steht an vier Stellen (`anwendung.rs:4686`, `:4980`, `:5180`, `:5620`) und nicht an zwei. |
| 6 | `260817-1111` „jedes Kommando außer dem Abbruch" | `_c_` | **ja, hält an der beanstandeten Stelle.** `anwendung.rs:4752-4758` nennt die vier mit beiden Quellen, `blaetter/mod.rs:296-307` ebenso. Beide Quellen selbst gelesen: `operationen.rs:266-268` und `zulaessigkeit.rs:197-202`. Vier weitere Träger stehen: zwei im offenen Datensatz `260817-1302`, `CLAUDE.md:123` aus `260817-1111` selbst, und einer, den keiner der beiden nennt, siehe Befund 4. |
| 7 | `260817-1112` `frage_und_erlaeuterung` trägt kein `#[must_use]` | `_c_` | **ja, hält.** `loeschwarnung.rs:305`, mit ausgeschriebener Begründung. `operationen::loeschfrage`, das Gegengewicht, trägt es weiter nicht und fällt mit Bündel D; der Datensatz sagt das. |

## Die vier abgenommenen Zusagen, einzeln geprüft

### 1 — Die Vorbelegung des Blattes: hält, und an den anderen Blättern ändert sich nichts

`Schaltflaeche` trägt drei Pflichtfelder, `Wirkung` hat keine Vorgabe (`blaetter/mod.rs:343-387`),
und `abbruchstelle` ist die eine Regel darüber, mit ausgeschriebener Tafel über drei Zeilen
(`:396-421`).

**Die Zusage „an den anderen Blättern ändert sich nichts" ist nachgezählt und stimmt**, und
zwar an allen elf Bauplätzen und nicht an fünf. Verglichen habe ich die alte Ableitung (letzte
Schaltfläche mit `Taste::Escape`, `git show 3fcd375:…/blaetter/mod.rs:435-437`) mit der neuen
(erste mit `Wirkung::Liegenlassen`):

```
Blatt                        Schaltflächen                            alt  neu
loeschbestaetigung           Abbrechen(Eingabe) Vorgang(EingabeCmd)     0    0   ← der behobene Fall
                             Rückfall des Abschlussblocks war 1 (letzte)     0
Blatt::neu (5 Blätter)       bestaetigen(Eingabe) Abbrechen(Escape)     1    1
konflikt                     3× Ausfuehren, Abbrechen(Escape)          3    3
ungesichert                  Sichern, Verwerfen, Abbrechen(Escape)     2    2
uebersprungen                Schließen(Eingabe)                        0    0
zettel                       Fertig(Escape)                            0    0
belegungsansicht             Fertig(EingabeCmd)                        0    0
```

An zehn Blättern ist der Wert derselbe, am elften ändert er sich, und das ist der Defekt. Auch
der Abbruchcode des Griffs ändert sich nirgends: er kam alt aus
`abbruchstelle.map_or(NSAlertFirstButtonReturn, …)`, und wo `abbruchstelle` alt `None` war
(`loeschbestaetigung`, `uebersprungen`, `belegungsansicht`), liefert die neue Regel 0, also
denselben Code. Der Eingabewächter schickte alt `NSAlertSecondButtonReturn` fest; einen
Wächter hält nur ein Blatt aus `Blatt::neu`, dort ist die neue Stelle 1, also derselbe Wert.

Die Einordnung der `Wirkung` ist an jedem Blatt einzeln gelesen und trifft. Namentlich
„Überspringen" im Konfliktblatt trägt `Ausfuehren` und nicht `Liegenlassen`: es beantwortet die
Frage und lässt die Operation weiterlaufen, `Liegenlassen` ist nach seiner eigenen Erklärung
(`:355-363`) der Ausgang, der den Vorgang liegen lässt.

**Die Zusicherung dagegen greift nicht, und das ist Befund 1.** `abbruchstelle` liefert bei einer
Aufzählung ohne `Liegenlassen` die Stelle 0, das `debug_assert!` in `mit_schaltflaechen`
(`:532-537`) soll den Fall fangen, und es ist im Auslieferungsbau wegübersetzt und im Probenbau
nicht erreichbar. Heute ist der Rückfall ungefährlich, weil an jedem Blatt eine liegenlassende
Schaltfläche steht; die Sicherung dafür ist allein die Zählprobe, und die prüft je Datei.

### 2 — Die Polarität: jede Verwendung liegt richtig, die Sicherung nach vorn fehlt

Jede Stelle im Baum, die `Befund` liest oder liefert, ist geöffnet. Es sind zwei, und beide
liegen auf der Erlaubnis-Polarität:

```
papierkorb::fuehrt_einen_papierkorb (papierkorb.rs:185)  Ok → Ja, Err → Nein, kein UTF-8 → Unentschieden
loeschwarnung::vor_der_rueckfrage   (loeschwarnung.rs:242-246)  Ja → Rückfrage,
                                     Nein | Unentschieden → OhnePapierkorb (ausgeschrieben, kein `_`)
anwendung.rs:4665-4668              canonicalize scheitert → Unentschieden
```

**`ist_warnwuerdig` steht an keiner Papierkorbfrage und könnte es auch nirgends sonst**, denn es
hat im ganzen Baum keinen Aufrufer. Nachgezählt mit `grep -rn "ist_warnwuerdig" crates/`: sechs
Treffer, drei in `befund.rs` (Erklärung und zwei Proben), drei außerhalb, und alle drei stehen
in einem Doc-Kommentar. Die Erwartung, die daraus folgt, hält also — aber sie hält, weil die
zweite Polarität noch nicht existiert, und nicht weil etwas sie hält. Das ist Befund 2.

Der Typ selbst ist von hoher Güte und ausdrücklich zu nennen: die Tafel von `oder` steht mit
neun Feldern ausgeschrieben, ihre Ableitung aus zwei Sätzen des Specs steht darunter, und der
Doc-Kommentar begründet, warum sie **nicht** als `max` über ein abgeleitetes `Ord` dasteht
(`befund.rs:186-191`). Fünf Proben, darunter eine über die Rechenregel
`a.oder(b).ist_warnwuerdig() == a.ist_warnwuerdig() || b.ist_warnwuerdig()`. Selbst gefahren,
grün.

### 3 — Die Stufenregel: überschneidungsfrei, vollständig, und der Rumpf entscheidet nichts mehr

Die Tafel (`loeschwarnung.rs:184-190`) und der Rumpf (`:233-247`) sagen Zeile für Zeile
dasselbe; einzeln verglichen, keine Abweichung. Die fünf Zeilen decken zwei mal zwei mal drei
Kombinationen, der Übersetzer hält die Vollständigkeit ohne Auffangzweig, und `Nein` und
`Unentschieden` stehen ausgeschrieben statt als `_`, damit ein vierter Befund den Bau anhält.

**Gegen das erste Flussbild im `## Approach` des Plans geprüft**, Knoten für Knoten:

```
Plan                            Regel                       Stelle
R Filtertext                    außerhalb, rueckschritt.rs   anwendung.rs:4523
V Vorgang läuft                 Zeile 1                      Vorstufe::VorgangLaeuft
A Auswahl leer                  Zeile 2                      Vorstufe::NichtsAusgewaehlt
P Papierkorb                    Zeilen 3 bis 5               Vorstufe::Rueckfrage / OhnePapierkorb
Z, W, B1, B2                    Bündel C, nicht gebaut       —
F Cmd+Return                    beim Blatt, ausdrücklich      Abschlussblock anwendung.rs:4712
```

Die Reihenfolge stimmt mit dem Bild, und der Zuschnitt „drei von fünf Stufen" ist im Modulkopf
begründet (`:44-54`): was ohne Fenster prüfbar ist, steht hier, was am Blatt hängt, nicht. Der
Spec zeichnet dieselbe Kette ohne Stufe V; der Plan hat sie hinzugefügt, und der Grund steht in
seinem `## Current State`.

**Der Rumpf trägt keine Entscheidung mehr, die in die Regel gehört.** Geprüft habe ich vier
Kandidaten:

- `laut = false` in `in_den_papierkorb` (`anwendung.rs:4467`) ist eine feste Wahl im Rumpf. Sie
  ist Gegenstand von Schritt 11 und dort ausdrücklich vorgesehen, also kein Befund.
- Der Ausgang „kein Fenster, an dem das Blatt hängen könnte" (`:4696-4698`) liefert `false` und
  meldet nichts. Er steht nicht in der Tafel, ist aber am Rückgabewert der Funktion
  dokumentiert (`:4636-4638`), und ohne Fenster gibt es auch keine Statuszeile für eine Meldung.
- `let _ = self.vorgang_laeuft_schon(aktiv)` (`:4682`) liest die Tatsache ein zweites Mal. Beide
  Lesungen sind `vorgang.borrow().is_some()` (`:4655` und `:5218-5226`), also dieselbe Bedingung;
  kein Auseinanderlaufen möglich, und `let _ =` heißt hier wie im ganzen Baum „ich brauche den
  Wert nicht".
- Die Reihenfolge der Tatsachenerhebung entscheidet den Ausgang nicht, wie der Kommentar sagt.
  Sie entscheidet die **Kosten**, und das ist Befund 3.

### 4 — Zwei Aussagen über Nutzerarbeit: die Begründung ist zur Hälfte tragfähig

Ungeprüft bleiben „ein Abbruch stellt keinen Auftrag" und „der bestätigte Auftrag trägt die
gezeigte Auswahl". Was daran wirklich den Vordergrund verlangt: dass AppKit auf einen Klick, auf
`Return` und auf `Esc` den Rückgabewert liefert, den KRK erwartet. Das ist Nutzerarbeit, und die
Begründung trägt.

Was daran **nicht** den Vordergrund verlangt: die Abbildung von diesem Rückgabewert auf den
Auftrag. Sie steht als Abschlussblock in `anwendung.rs:4712-4724` und ist reine Rechnung über
`bestaetigt` und die `Cell`. Eine Hälfte davon ist schon geprüft — `stelle == AUSFUEHRENDE_STELLE`
trifft die ausführende Schaltfläche, zwei Proben in `loeschbestaetigung.rs`. Die andere Hälfte,
von `bestaetigt` zum Auftrag, wäre der Spiegel von `vor_der_rueckfrage` und mit derselben
Bauform prüfbar. Der Satz „am Code ist dafür nichts mehr zu tun" hat damit ein Gegenbeispiel;
das ist Befund 5.

## Die Verhaltensänderung an `f8`: geprüft und tragfähig

Die Papierkorbprüfung liegt im gemeinsamen Rumpf und trifft bis Bündel D auch das endgültige
Löschen, das keinen Papierkorb braucht. Die Begründung steht ausgeschrieben an
`anwendung.rs:4617-4625`. Gegen Directive, Spec und Plan gelesen:

- **Der Plan verlangt es.** Schritt 6 nennt `loeschen_nach_rueckfrage` als Ort der Prüfung, und
  das ist der geteilte Rumpf beider Befehle. Die Änderung ist planungskonform und keine
  Abweichung; was Schritt 3 zusagte („`endgueltig_loeschen` behält sein Verhalten unverändert
  bis Bündel D"), ist von Schritt 6 an dieser einen Stelle bewusst überholt.
- **Die Directive trägt sie.** „Ein Ziel ohne Papierkorb wird nicht gelöscht, sondern gemeldet"
  steht ohne Einschränkung da. Streng gelesen spricht der Satz über den einen Löschweg, den es
  nach der Runde gibt; die weitere Lesung ist die vorsichtigere, und sie widerspricht nichts.
- **Der Spec widerspricht nicht.** C4 kennt den Zwischenzustand mit zwei Löschwegen nicht und
  entscheidet ihn deshalb nicht.
- **Die Folge ist benannt und endlich.** Auf einem Datenträger ohne Papierkorb hat der Nutzer
  bis Bündel D keinen Löschweg in KRK; die Meldung nennt den Finder als Ausweg. Nach Bündel D
  ist der Zustand derselbe wie ohne diese Änderung.

Eine Kleinigkeit gehört dazu und ist kein Befund: die Meldung sagt „das Ziel führt keinen
Papierkorb", und für einen Befehl namens „Endgültig löschen" ist das die Begründung eines
anderen Vorgangs. Sie ist bis Bündel D sichtbar und danach richtig; einen eigenen Wortlaut für
zwei Tage zu bauen wäre der zweite Löschweg an der Stelle, an der diese Runde den zweiten
abschafft.

## Was geprüft ist und hält

**Die vier Abnahmekommandos laufen grün, selbst gefahren am 260817-1418.** `make check` mit
Ausgang 0, also `cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all --check`
und `cargo clippy --workspace --all-targets -- -D warnings`. Die neuen Proben einzeln gefahren
und namentlich gesehen: fünf in `verzeichnis::befund::tests`, drei in `appkit::papierkorb::tests`,
zwei in `appkit::blaetter::loeschbestaetigung::tests`, drei in `appkit::blaetter::tests`, sieben
neue in `kommandos::loeschwarnung::tests` neben den fünf bestehenden. `krk-ui` 655 Proben,
`krk-core` 161, alle grün.

**Die Untergrenzen sind am SDK geprüft und nicht erschlossen.** Fünf Angaben aus dem Modulkopf
von `papierkorb.rs` habe ich in den Kopfdateien des lokalen SDK einzeln nachgelesen, Zeile für
Zeile, und alle fünf stimmen samt Zeilennummer:

| Angabe im Modulkopf | Im SDK |
|---|---|
| `URLForDirectory:…` seit 10.6, `NSFileManager.h:127` | `API_AVAILABLE(macos(10.6), …)`, Zeile 127 |
| `NSTrashDirectory` seit 10.8, `NSPathUtilities.h:88` | `API_AVAILABLE(macos(10.8), ios(11.0))`, Zeile 88 |
| `NSSearchPathDirectory` ohne Angabe, `NSPathUtilities.h:61` | `typedef NS_ENUM`, keine Angabe, Zeile 61 |
| `NSSearchPathDomainMask` ohne Angabe, `NSPathUtilities.h:92` | `typedef NS_OPTIONS`, keine Angabe, Zeile 92 |
| `NSUserDomainMask` ohne Angabe, `NSPathUtilities.h:93` | `NSUserDomainMask = 1`, keine Angabe, Zeile 93 |

Keine liegt über 15.0. Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`
steht in jeder Datei unter `appkit/` außer `koordinaten.rs` und `mod.rs`; selbst gezählt über
alle Dateien in `appkit/` und `appkit/blaetter/`, genau diese zwei fehlen, und beide sind
begründete Ausnahmen. `blaetter/mod.rs` hat seinen Abschnitt mitgezogen und `NSAlertSecondButtonReturn`
mit Begründung ausgetragen (`:135-137`).

**`#[must_use]` steht an jedem neuen Rückgabewert, dessen Fallenlassen unbemerkt bliebe**, und
jedes mit ausgeschriebenem Grund: `Befund::ist_warnwuerdig`, `Befund::oder`,
`papierkorb::fuehrt_einen_papierkorb`, `loeschwarnung::vor_der_rueckfrage`,
`loeschwarnung::ohne_papierkorb`, `loeschwarnung::frage_und_erlaeuterung`, `blaetter::abbruchstelle`.
Sieben Stellen, keine ohne Attribut.

**Die Zahlen in der Prosa stimmen.** Nachgezählt, nicht geglaubt: zehn Blätter in
`blaetter/mod.rs:4` (zehn Module unter `blaetter/`), fünf Berührungen jünger als 10.0 (`:122`),
zehn Kommandos der Bereichsleiste (`loeschwarnung.rs:126`, `anwendung.rs:4485`), fünf Stufen und
drei Tatsachen (`loeschwarnung.rs:46-48`), zwölf Kombinationen aus zwei mal zwei mal drei
(`:192`), neun Felder der Tafel (`befund.rs:135`), vier durchgelassene Kommandos
(`anwendung.rs:4753`), elf Module unter `verzeichnis/` (`verzeichnis/mod.rs:3`), drei plus drei
Berührungen in `papierkorb.rs:92-99`.

Eine Zahl ist knapp und hält bei genauer Lesung: `blaetter/mod.rs:63` sagt „Drei Stellen dieser
Datei brauchen die Antwort" und zählt Abschlussblock, `Blattgriff::abbrechen` und
`Eingabewaechter` auf. Vierter Leser des abgeleiteten Feldes ist `Blattgriff::abbruchweg`
(`:465-470`); es ist keine vierte Entscheidung, sondern derselbe Abbruch als festhaltbarer Ruf,
und sein eigener Doc-Kommentar sagt „Der Ruf tut Zeile fuer Zeile, was
[`Blattgriff::abbrechen`] tut". Als Zählung der Entscheidungsstellen stimmt die Drei.

**Der Papierkorbtest fragt dieselbe Stelle, die gleich löschen wird**, und die
Entscheidbarkeitszeile des Plans trägt das: `NSFileManager.defaultManager()` beantwortet die
Vorprüfung und führt `trashItemAtURL:` aus, beide in derselben Datei
(`papierkorb.rs:138`, `:191`). `create:` steht auf `false`, mit einem eigenen Abschnitt darüber,
warum eine Prüfung, die im Zweifel anlegt, eine andere Frage beantwortet. Der negative Ausgang
ist an `/dev` geprüft und nicht an einem fehlenden Pfad, und der Doc-Kommentar der Probe
begründet die Wahl.

**Der Ordner wird genau einmal aufgelöst**, in `anwendung.rs:4665`, und `fuehrt_einen_papierkorb`
fasst das Dateisystem nicht an; der Grund steht an beiden Stellen. Ein Pfad, der sich nicht
auflösen lässt, zählt als `Unentschieden` und löscht nicht.

**Die Aufruferzählung der Stufenregel stimmt.** `die_stufenregel_hat_genau_einen_aufrufer`
(`loeschwarnung.rs:362`) zählt über `crate::quellbaum` und schließt die eigene Datei aus; ich
habe den einen Aufrufer selbst gesucht und `anwendung.rs:4670` gefunden, keinen zweiten.

## Befunde

### 1 — Mittel: Die Zusicherung gegen ein Blatt ohne ungefährlichen Ausgang greift in keinem Bau

`blaetter/mod.rs:532-537` ist ein `debug_assert!`. Der Auslieferungsbau übersetzt es weg:
`cargo xtask bundle` baut mit `--profile release` (`xtask/src/bundle.rs:60`, `:472`), und kein
`[profile.…]`-Abschnitt setzt `debug-assertions` — nachgezählt über die Wurzel-`Cargo.toml`, die
drei Kisten, `xtask` und `.cargo/config.toml`, kein Treffer; Cargos Vorgabe für `release` ist
`false`. Im Probenbau ist die Zeile vorhanden und wird nie ausgeführt: alle elf Bauplätze eines
`Blatt` liegen im Nicht-Probencode, und `krk-ui` hat kein Bibliotheksziel.

Die Prosa an `abbruchstelle` sagt es trotzdem unbedingt (`:405`: „laesst es im Probenbau
auffliegen"); die Stelle bei der Zählprobe sagt es mit der Bedingung „sobald das Blatt im
Probenbau wirklich aufgeht" (`:813-814`), und die tritt nicht ein. Wirksam ist allein
`jedes_blatt_nennt_seine_liegenlassende_schaltflaeche`, und die prüft je Datei, nicht je Blatt —
was bei ihr steht und namentlich `blaetter/mod.rs` selbst trifft, die einzige Datei mit mehr als
einem Blattbau.

**Richtung:** die Frage „welche Schaltfläche ist ungefährlich, wenn keine es ist" hat keine
Antwort, und `unwrap_or(0)` ist eine Näherung darauf. Der Mechanismuswechsel wäre, eine
liegenlassende Schaltfläche am Typ zu verlangen; dann ist `abbruchstelle` total und der Rückfall
entfällt.

Datensatz: `issues/260817-1419_o_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`

### 2 — Mittel: Die einzige Sicherung gegen den Polaritätsfehler ist Prosa

Drei Modulköpfe warnen davor, `ist_warnwuerdig` an die Papierkorbfrage zu halten
(`befund.rs:50-68`, `loeschwarnung.rs:61-69`, `papierkorb.rs:49-59`). Keine Probe und kein Typ
hält es davon ab. `loeschwarnung.rs:66` sagt dabei etwas, das nicht Begründung ist, sondern
Aussage über den Baum: „`Befund::ist_warnwuerdig` kommt in dieser Datei nicht vor, und das ist
Absicht" — und zweihundert Zeilen darunter steht in derselben Datei eine Zählprobe, die
`quellbaum::aufrufstellen` schon benutzt.

Bündel C bringt die erste Aufrufstelle von `ist_warnwuerdig` und drei weitere Prüfungen auf die
Warngrund-Polarität. Wer sie schreibt, hat drei Prosaabsätze und keine rote Probe als
Widerstand.

**Richtung:** eine Zählprobe auf null Aufrufstellen in den beiden Dateien der Erlaubnis-Polarität
ist billig. Der stärkere Weg sind zwei Typen für die zwei Fragen; `befund.rs:67-68` sagt selbst,
die Polarität hänge an der Frage und nicht am Typ, und genau das ließe sich am Typ festmachen.

Datensatz: `issues/260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`

### 3 — Niedrig: Der Papierkorbtest läuft vor den beiden billigen Sperren

`loeschen_nach_rueckfrage` (`anwendung.rs:4655-4668`) erhebt alle drei Tatsachen, bevor eine
Stufe entschieden ist. Damit laufen `std::fs::canonicalize` und `fuehrt_einen_papierkorb` auch
dann, wenn schon ein Vorgang läuft oder die Auswahl leer ist. Auf einem hängenden Netzlaufwerk
blockiert ein `delete` ohne Auswahl den Hauptfaden, wo vorher eine Abfrage im Speicher genügte.
Der benannte Rest in `papierkorb.rs:85-86` deckt allein den `NSFileManager`-Aufruf;
`canonicalize` steht in keinem Modulkopf als Kostenstelle.

**Richtung:** die Reihenfolge bleibt in der Tafel, der Rumpf erhebt den dritten Befund faul —
ein `impl FnOnce() -> Befund` hält die Regel unverändert prüfbar. Schritt 11 fasst denselben
Rumpf an.

Datensatz: `issues/260817-1419_o_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren-und-bringt-zwei-dateisystemzugriffe-mit.md`

### 4 — Niedrig: Ein vierter Träger der verkürzten Blattsperre liegt außerhalb von `crates`

`resources/default-keymap.toml:708` sagt „jeden Befehl ausser dem Abbruch". Es sind vier.
`issues/260817-1302` erhebt genau diese Formulierung, sagt „eine Suche ueber den ganzen Baum
findet zwei" und hat `crates/` gelesen. Der Schluss an der Stelle hält — die drei Befehle der
Ausnahmeliste liegen auf `cmd+q`, `shift+cmd+w` und `cmd+n` und nicht auf `return` —, und zwei
Ebenen daneben steht seit T1 genau diese Rechnung ausgeschrieben (`blaetter/mod.rs:296-307`).

Datensatz: `issues/260817-1419_o_ein-vierter-traeger-der-verkuerzten-blattsperre-liegt-ausserhalb-von-crates-und-fehlt-in-der-erhebung.md`

### 5 — Niedrig: Der Abschluss von `260817-1107` begründet zu weit

„Am Code ist dafür nichts mehr zu tun" hat ein Gegenbeispiel: der Abschlussblock in
`anwendung.rs:4712-4724` trägt die Abbildung von `bestaetigt` auf „Auftrag mit dieser Auswahl"
als reine Rechnung, und als reine Funktion in `loeschwarnung` wäre sie mit einer Tafel prüfbar —
der Spiegel dessen, was dieselbe Aufgabe für die Vorstufen gemacht hat. Nutzerarbeit bleibt
allein die Schicht darüber, dass AppKit die erwarteten Rückgabewerte liefert.

Datensatz: `issues/260817-1419_o_der-abschluss-von-260817-1107-begruendet-zwei-ungepruefte-eigenschaften-zu-weit.md`

### 6 — Niedrig: Zwei verschiedene dreiwertige Typen unter `verzeichnis` heißen beide `Befund`

`verzeichnis::Befund` (neu) und `verzeichnis::modell::Befund` (Runde 10) sind zwei Typen,
beide dreiwertig, beide mit einer Variante `Unentschieden`, beide über eine Frage an einen Pfad.
Der Übersetzer trennt sie, weil `modell::Befund` nicht re-exportiert ist; der Leser unterscheidet
sie um einen Pfadabschnitt. Der neue Absatz im Modulkopf von `verzeichnis/mod.rs` (`:74-82`)
nennt den Nachbarn nicht.

Datensatz: `issues/260817-1419_o_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`

### 7 — Niedrig: Der auslösende Defekt ist behoben und steht weiter offen

`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md` verlangt,
dass jede Löschfunktion nachfragt. Beide Befehle gehen seit Bündel A durch denselben Rumpf; ich
habe die vier Wege einzeln nachgelesen. Der Datensatz trägt weiter `_o_`, und kein Schritt des
Plans schließt ihn: Schritt 16 zieht die Entscheidungsdatensätze nach, für die Defektdatensätze
steht kein Schritt.

Datensatz: `issues/260817-1419_o_der-ausloesende-defekt-des-raeumens-ohne-rueckfrage-ist-behoben-und-steht-weiter-offen.md`

## Übergreifend

**Die Sache ist geprüft, die Sicherung dagegen ist Prosa.** Befund 1 und Befund 2 sind derselbe
Fehler in zwei Größen, und beide sitzen an einer Zusage, die diese Sitzung gerade eingelöst hat.
`260817-1106` ist behoben und mit zwei Proben festgehalten — die Regel, die den nächsten Fall
derselben Art fangen soll, hängt an einem `debug_assert!`, das in keinem Bau läuft. Die
Papierkorbfrage liegt auf der richtigen Polarität und ist mit einer Zwölf-Feld-Tafel geprüft — die
Regel, die den nächsten Verwender auf der richtigen Polarität hält, steht dreimal in Prosa. Das
Muster ist erkennbar und gutartig: der `coder` schreibt die Begründung sorgfältig hin und macht
sie nicht prüfbar, obwohl das Werkzeug dafür in derselben Datei liegt. Die Gegengewohnheit wäre:
wenn eine Prosaaussage eine Aussage **über den Baum** ist, bekommt sie eine Zählprobe.

**Vier Aussagen über den Baum sind in dieser Sitzung erhoben und drei davon knapp zu weit
gefasst.** „Die Bereichsleiste kennt elf Kommandos" (waren zehn, vom `coder` berichtigt), „der
Satz steht an zwei Stellen" (vier, berichtigt), „eine Suche über den ganzen Baum findet zwei"
(drei, Befund 4), „am Code ist nichts mehr zu tun" (Befund 5). Zwei hat der `coder` beim Beheben
selbst gefunden und im Datensatz vermerkt, und das ist die richtige Reihenfolge. Die zwei
verbleibenden hängen an derselben Ursache: die Reichweite der Suche und die Reichweite der
Aussage stehen nicht nebeneinander. `crate::quellbaum` hat genau dafür einen Modulkopf, und
CLAUDE.md führt den Fall für `\.md` schon.

**Der Zuschnitt „Regel raus, Rumpf beschafft" trägt und ist nachahmbar.** Er hat in einem Zug
den offenen Befund 2 der vorigen Durchsicht erledigt, die fünfte Stufe aus Bündel B mit
aufgenommen, ohne die Stelle zweimal zu ändern, und die Mechanik der Runde von null auf zwölf
geprüfte Fälle gebracht. Er ist derselbe Schnitt wie `rueckschritt` in der Runde 10, und Befund 5
sagt, wo er noch einmal anzuwenden wäre.

**Zwei Stufenketten stehen nach Bündel D nebeneinander**, und das ist keine Beanstandung,
sondern eine Beobachtung für den nächsten Plan: `vor_der_rueckfrage` ordnet „Vorgang läuft" und
„Auswahl leer" für den Löschweg, `auftrag_stellen` (`anwendung.rs:5171-5182`) ordnet dieselben
zwei Fragen für Kopieren, Verschieben und Umbenennen. Wer die eine ändert, ändert die andere
nicht mit. Die Zusage der Stufenregel ist ausdrücklich auf den Löschweg beschränkt, also trägt
sie; ein Zusammenlegen wäre Arbeit für eine eigene Runde und keine dieser.

## Empfohlene Reihenfolge

**Mit Bündel C:** Befund 2, weil Bündel C beide betroffenen Dateien anfasst und die erste
Aufrufstelle von `ist_warnwuerdig` bringt. Danach ist der Schnitt teurer, weil vier Prüfstellen
daran hängen.

**Mit Schritt 11:** Befund 3, weil Schritt 11 denselben Rumpf umbaut und die Texte
hineinzieht. Zusammen mit dem offenen `260817-1108`, das dieselbe Stelle betrifft.

**Vor Bündel D oder in ihm:** Befund 1. Er ist eine Sicherung und kein Fehlverhalten, also
kein Auslieferungsblocker; er sitzt aber in der Hülle, die Bündel D ohnehin anfasst, wenn das
Blatt seine zweite Form verliert.

**Mit Bündel E:** Befund 4 und Befund 6. Befund 4 ist derselbe Nachzug, den Schritt 15 fährt,
und die Nadel dort ist um `resources/` zu erweitern, bevor gezählt wird. Befund 6 ist eine
Umbenennung und gehört in denselben Durchgang wie die Prosazahlen.

**Ohne Bündelbindung:** Befund 5 und Befund 7. Beide sind Datensatzarbeit und blockieren
niemanden; Befund 7 sollte vor dem Sitzungsende laufen, damit die tragende Zusage der Runde
nicht als offen im Speicher steht.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **drei der sieben Befunde sind
geschlossen, vier stehen offen, und einer der offenen trägt eine widerlegte Behauptung.**
Geschlossen: `260817-1419_c_zwei-verschiedene-dreiwertige-typen…` in `17d3550`,
`260817-1419_c_der-abschluss-von-260817-1107…` und
`260817-1419_c_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren…` in `792995a`; alle
drei am Baum nachgelesen. Offen und am Baum unverändert: die Sicherung gegen den
Polaritätsfehler, die Zusicherung gegen ein Blatt ohne ungefährlichen Ausgang, der vierte Träger
der verkürzten Blattsperre.

**Der vierte offene Befund,
`260817-1419_o_der-ausloesende-defekt-des-raeumens-ohne-rueckfrage-ist-behoben-und-steht-weiter-offen.md`,
trägt eine Behauptung, die nicht hält.** Er sagt „sein Wortlaut ist von Bündel D nicht mehr
betroffen" und empfiehlt daraus den Übergang von
`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md` auf `_c_`.
Dessen Abschnitt `## Verschärfung vom 260817: der endgültige Löschweg fällt ganz weg` verlangt
den Wegfall von `Kommando::EndgueltigLoeschen`, also genau Bündel D, und der ist nicht gebaut:
22 Zeilen im Baum, `resources/default-keymap.toml:151` unverändert. Die Empfehlung ist nicht
ausgeführt, und der Marker bleibt `_o_`. Beide Datensätze tragen die Begründung.

Die Erhebung dieser Durchsicht über die elf Bauplätze der Blatt-Vorbelegung ist gegengeprüft und
stimmt: `260817-1419_o_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang…` zählt sie
einzeln auf, und `abbruchstelle` (`blaetter/mod.rs:416`) hat drei Leser.
