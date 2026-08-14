# Abgleich der neunten Runde gegen den Baum

**Agent:** reconciler
**Datum:** 260814-1002
**Domäne:** code
**Stand:** `79dab20`, Sitzungsspanne `6d05bef..HEAD`, sieben Commits
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Sitzungsprotokoll:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260813-2342-orchestrator-session.md` (Abschnitt `## Coherence`)
**Kein Bündelbau, kein `cargo xtask`, kein Vordergrundlauf.** `target/KRK.app` ist unberührt geblieben.

---

## Was in Zahlen geprüft ist

| Gegenstand | Gelesen | Ergebnis |
|---|---|---|
| Planschritte | 16 | alle 16 auf `[DONE]` und alle 16 am Baum bestätigt |
| Abnahmekriterien C1 bis C5 | 72 | 43 am Baum nachweisbar, 29 nur am laufenden Bündel |
| davon am Baum gehalten | 43 | 40 ohne Einschränkung, 3 mit einer benannten |
| Entscheidungsdatensätze im Circle | 2 | beide von `_a_` auf `_i_` gezogen, jeder mit Commit und Fundstelle |
| Defekte im Circle vorher / nachher | 11 / 18 | sieben aus diesem Abgleich neu abgelegt |
| davon geschlossen | 2 | beide Behebungen nachgelesen, keine widerlegt |
| davon offen | 9 / 16 | alle neun bestehenden zu Recht offen, jeder an seiner Stelle nachgesehen |
| Durchsichten | 3 | keine widerlegt; eine Zusage stärker belegt als dort behauptet |
| Planabweichungen der Bauer | 7 | stichprobenartig nachgeprüft, die zwei zählbaren nachgezählt |
| Bau | — | `make check` exit 0, „alle vier gruen" |

**Zur Zahl 72.** Der Spec führt je Fähigkeit zwei Listen. Die erste ist am Baum nachweisbar und
ein Agent kann sie fahren, die zweite verlangt KRK im Vordergrund. Aufgeteilt: C1 elf und acht,
C2 vier und fünf, C3 fünf und fünf, C4 zwölf und sieben, C5 elf und vier.

---

## 1. Halten die sechzehn Planschritte gegen den Baum?

**Alle sechzehn, einzeln gegen die Dateien und Zeilen gelesen, die sie selbst nennen.** Gefragt
war je Schritt dasselbe: existiert der Typ oder die Funktion, steht die behauptete Bedingung
dort, ist die Probe da, ist die Prosa mitgeändert. Die Tabelle im `## Reconciliation Log` des
Plans nennt die Fundstellen strangweise; hier stehen die Stellen, an denen mehr als ein Blick
nötig war.

### Strang A — die Ablage

`text::datei::lesen` (`crates/krk-core/src/text/datei.rs:411`) trägt den einen Befund mit vier
Ausgängen, und `oeffnen` (`:494`) ist zur Übersetzung geworden, ohne Signatur oder
Rückgabewerte zu ändern — nachgelesen Zweig für Zweig gegen die vier `Abweisung`-Werte. Der
Deskriptor wird vor jeder Rückkehr mit `Unlesbar` zurückgespult (`:473`). `EDITORGRENZE` steht
im Baum an genau einer Stelle (`:164`); jede weitere Fundstelle liegt in
`crates/krk-core/tests/text.rs` und `tests/textkopien.rs` und liest sie als `datei::EDITORGRENZE`.

`atomar::vorbereiten` und `atomar::schreiben` nehmen `&mut impl Read` und schreiben über
`io::copy` (`ablage/atomar.rs:153`, `:167`, `:156`); `sync_all` und das zweistufige Umbenennen
sind unangetastet. Die fünf bestehenden Aufrufstellen schreiben `&mut text.as_bytes()`, und die
Baumprobe `nur_benannte_dateien_erreichen_das_atomare_schreiben` (`tests/baum.rs:189`) führt
weiterhin dieselben fünf Quelldateien; geändert ist allein ihr Kommentar („vier Schreiber hinter
einem `Zugang`" statt drei, `:192`).

`Datei::ALLE` ist `[Datei; 6]` (`ablage/pfade.rs:142`) mit `Zettel(Zettel)` als Nutzlast statt
zweier Varianten nebeneinander — damit ist „das Blatt führt genau zwei Zettel" eine Aussage über
einen Typ. Die sieben Fundstellen von `Datei::ALLE` liegen alle in `tests/ablage.rs`; vier davon
laufen über `format() == Format::Toml` (`:105`), drei fragen weiter nach Pfad, Name und
Nichtanlage. `ueber_der_ablage_stehen_genau_zwei_absprachen` (`tests/baum.rs:214`) ist nicht
angefasst.

### Strang B — der Befehl

`resources/default-keymap.toml` führt 83 `[[funktion]]`-Blöcke, die Kopfzahl steht auf 83
Funktionen mit 90 Kombinationen (`:34`), und der Unterschied gegen `6d05bef` besteht allein aus
Zufügungen. `Kommando::KENNUNGEN` hat 77 Paare (`belegung.rs:579`), `Kommando::wirkungsbereich`
ordnet den Notizzettel `Wirkungsbereich::Ueberall` zu (`:782`), `bereich_des_kommandos`
`Funktionsbereich::Anwendung` (`belegungsmodell.rs:345`).

**`belegungsansicht.rs` und `menuemodell.rs` sind in dieser Runde nicht angefasst.** Genau das
ist die Zusage von Schritt 7: die drei Flächen — Ansicht, Markdown-Ausgabe, Menüleiste — führen
die eine Zeile, ohne dass jemand sie dort einträgt. `belegungsausgabe.rs` hat zwei Zeilen
geändert, und die gehören zur Signatur von `atomar::schreiben` und nicht zum Notizzettel.

### Strang C bis F

`textautomatik::automatiken_abschalten` (`appkit/textautomatik.rs:111`) hat zwei Aufrufer, die
Fläche des Editors (`editor.rs:3124`) und die des Zettels (`blaetter/zettel.rs:470`).
`Zettelmodell` (`zettelmodell.rs:95`) trägt `Wechsel` als vollständige Aufzählung, `oeffnen` mit
`#[must_use]` und der Regel „der getippte Stand gewinnt" (`:173`), `zu_sichern` als Aufzählung
über `Zettel::ALLE` (`:248`) und `etwas_zu_sichern` daraus abgeleitet (`:262`).
`Zettelwaechter` ist ein `NSTextViewDelegate` und beantwortet `textView:doCommandBySelector:`
(`blaetter/zettel.rs:191`, `:211`); `uebernimmt` (`:313`) nimmt `cancelOperation:` und nicht
`insertNewline:`. `Sitzung::zettel` steht vor den drei Tabellen (`ablage/sitzung.rs:361`).
`die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` (`editor.rs:4873`) misst an zwei
gebauten Flächen gegen einen Zeugen (`:4885-4887`).

**Ein Zusatz gegenüber dem Plan, ohne eigene Abweichungszeile:** `Zettelmodell` trägt
`offenen_setzen` (`:118`) neben den im Plan aufgezählten Methoden. Es ist der Weg, auf dem der
Aufbau der Oberfläche die geladene Sitzung in das Modell setzt, und damit die andere Hälfte von
Schritt 15. Kein Befund.

---

## 2. Die 72 Abnahmekriterien, sortiert nach ihrem Nachweisweg

Der Auftrag verlangt drei Körbe: was der Baum trägt, was nur ein Mensch sehen kann, und was
einen Prüfaufbau braucht. Der dritte Korb ist nicht leer, und er ist nicht derselbe wie der
zweite.

### Korb 1 — der Baum trägt es, 43 Kriterien

**Vierzig halten ohne Einschränkung.** Jedes ist an der Stelle gelesen, die es nennt; die
Fundstellen stehen strangweise im `## Reconciliation Log` des Plans und in Abschnitt 1.

**Drei halten mit einer benannten Einschränkung**, und jede ist als Datensatz abgelegt:

| Kriterium | Was hält | Was nicht hält |
|---|---|---|
| C1, zweites | Keine bestehende Funktion verliert eine Kombination — der Unterschied an der Belegungsdatei besteht allein aus Zufügungen | „keine Kombination steht danach zweimal" ist als Satz über den Dateibestand falsch: `cmd+a` steht zweimal, gewollt und dreimal ausgeschrieben, und älter als diese Runde (`issues/260814-1002_o_c1-verlangt-dass-keine-kombination-zweimal-steht-…`) |
| C3, erstes | Die Regel trägt genau eine Ausnahme, den Rückruf `ist_editorflaeche` mit einem Aufrufer; die Fläche des Zettels ist nirgends angemeldet | Die zugesagte Probe zählt die **Erklärungen** der Regel und nicht ihre Ausnahmen (`issues/260814-1002_o_zwei-in-c3-zugesagte-proben-stehen-nicht-im-baum.md`) |
| C5, achtes | `EDITORGRENZE` steht an genau einer Stelle | Die Fundstelle im Kriterium sagt `:153`, die Konstante steht an `:164` — Drift aus dieser Runde selbst (`issues/260814-1002_o_c5-zitiert-editorgrenze-an-zeile-153-sie-steht-an-164.md`) |

**Drei Kriterien tragen keine eigene Probe und sind über eine allgemeine gehalten**, und das ist
kein Mangel, sondern die Bauform: C1 neun bis elf (eine Zeile in Ansicht, Markdown und Menü)
hängen an `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`
(`crates/krk-core/tests/belegung.rs:211`) und `eine_zeile_je_funktion`
(`krk-ui/src/belegungsmodell.rs:890`), die über **alle** Funktionen laufen. Der Notizzettel ist
darin Mitglied und kein Sonderfall — genau die stärkere Form, die die Durchsicht bei den drei
Zulässigkeitsproben lobt.

### Korb 2 — nur ein Mensch kann es sehen, 24 Kriterien

Sichtbarkeit, Aussehen, Erscheinen: die zwei anklickbaren Tabs und die Erkennbarkeit des offenen
(C2), die Lesbarkeit in beiden Erscheinungsbildern des Systems (C3), das Ausgrauen des
Menüeintrags und der Fokusrahmen nach dem Schließen (C1), das Öffnen einer Zetteldatei in einem
fremden Textprogramm (C5). Dazu jeder Ablauf, der einen Tastendruck an das laufende Bündel
verlangt: dass `f2` aus jedem der fünf Bereiche öffnet, dass `Esc` schließt und der zweite Druck
auf `f2` nichts tut, dass die Eingabetaste eine Zeile setzt, dass die Textautomatiken am
laufenden Programm kein Zeichen ersetzen. Kein Prüfaufbau nimmt einem das ab, und der Grund
steht in `CLAUDE.md`: aus dem Hintergrund gestartet weist die Wirkungsbereichsprüfung jeden
fokusgebundenen Befehl ab.

### Korb 3 — es braucht einen Prüfaufbau, 5 Kriterien

Diese fünf verlangen kein Auge, sondern eine **hergestellte Lage**, die im Baum heute niemand
herstellt. Sie sind der Grund, aus dem der Korb überhaupt zu trennen ist: wer sie unter Korb 2
zählt, hält sie für ein Sehproblem und baut nie die Vorrichtung, die sie prüfbar machte.

| Kriterium | Was herzustellen ist |
|---|---|
| C4, viertes bis siebtes der zweiten Liste | Der Datei eines Zettels das Schreibrecht **nehmen**, tippen, schließen, das Recht zurückgeben — die gescheiterte Sicherung, ihre Meldung und die Behauptung, dass der getippte Stand danach steht |
| C4, siebtes | Beide Zettel zugleich abweichend machen und KRK bei sauberem Editor beenden |
| C5, drittes und viertes der zweiten Liste | Eine Zetteldatei von außen mit einer ungültigen Bytefolge füllen und den Beiseitepfad danach ansehen |
| Plan, „Nutzerarbeit" Punkt 2 | Was AppKit mit `performClose:` an einem Fenster mit anhängendem Blatt tut, mit Ergebnis nach `messungen/` |

**Vier der fünf sind am Modell schon gedeckt, und das ist die Nachricht.** Die drei Proben aus
dem Nachtrag vom 260814-0941 (`zettelmodell.rs:417`, `:438`, `:456`) bilden die gescheiterte
Sicherung dadurch ab, dass `gesichert` gerade **nicht** gerufen wird, und die drei Zettelproben
in `tests/ablage.rs` (`:1484`, `:1530`, `:1567`) stellen die ungültige Bytefolge und die zu
große Datei her. Was am Bündel offen bleibt, ist die Verbindung zwischen Modell und Fläche —
also genau das, was `krk-ui` ohne Bibliotheksziel nicht prüfen kann
(`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`,
zurückgestellt).

Der fünfte, die `performClose:`-Messung, ist der einzige, für den der Plan die Zusage
ausdrücklich **von der Antwort unabhängig** gemacht hat: das Sichern läuft unbedingt und
vorgängig, und die Messung trägt nur nach, welche der zwei gezeichneten Kanten das Bündel geht.
Am Baum nachgelesen und nicht angenommen: `fenster_schliessen` (`anwendung.rs:3942`) sichert bei
`:3947` und ruft `performClose(None)` bei `:3949`.

---

## 3. Die zwei Entscheidungsdatensätze, beide auf umgesetzt

| Datensatz | Vorher | Nachher | Beleg |
|---|---|---|---|
| `260814-0656_*_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md` | `_a_` | `_i_` | `bfea397`, Probe `jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab` (`appkit/textautomatik.rs:271`) |
| `260813-2348_*_was-tut-der-zettel-mit-einer-zetteldatei-die-er-nicht-lesen-kann.md` | `_a_` | `_i_` | `9362034`, `Zugang::text_laden` (`ablage/mod.rs:564`) mit vier Ausgängen |

**Den ersten hat die Durchsicht vorgeschlagen, und der Vorschlag hält.** Nachgeprüft ist mehr
als die Existenz der Probe: der blinde Fleck, den die Empfehlung des Datensatzes ausdrücklich
verlangt, steht im Doc-Kommentar der Probe (`:252-269`) und nicht nur im Sitzungsprotokoll, und
er steht dort **zweimal** — die Nadel bindet an zwei Schreibweisen, und sie sieht kein
bearbeitbares `NSTextField`. Alle drei Rahmenbedingungen des Datensatzes halten: `EINSTELLUNGEN`
steht an einer Stelle, die Messung an einer gebauten Fläche ist geblieben und deckt jetzt zwei
Flächen, und die Vorschau ist von der Regel nicht eingefangen.

**Den zweiten nennt die Durchsicht nicht, und er ist ebenso vollständig umgesetzt.** Möglichkeit
3 verlangt drei Dinge: beiseitelegen über den bestehenden Weg, ein leerer Zettel, `EDITORGRENZE`
als Schranke. Alle drei stehen, und `beiseite_legen` hat jetzt genau zwei Aufrufer
(`ablage/mod.rs:499` und `:596`). Die Fehlzählung im Contra jener Möglichkeit — „ein sechster
Aufrufer" — bleibt im Datensatz stehen, wie die Ortsregel es für Aufzeichnungen eines Standes
vorsieht, und ist im Spec berichtigt.

---

## 4. Die neun offenen Defekte des Circles

**Alle neun sind zu Recht offen**, jeder an der Stelle nachgesehen, die sein Datensatz nennt.
Keiner ist stillschweigend behoben, und keine Zahl in einem der neun ist widerlegt.

| Datensatz | Am Stand `79dab20` |
|---|---|
| `0628_o_` Diagrammbefunde ohne Eigentümer | Bestandsaufnahme, kein Codegegenstand. Der Weg, auf dem ein Befund zu seinem Bearbeiter kommt, besteht weiterhin nicht |
| `0637_o_` Directive nennt drei Momente | besteht; die Aufstellung ist eine Stelle zu kurz, siehe Abschnitt 5 |
| `0910_o_` Zetteldatei über der Grenze wird unbegrenzt kopiert | `io::copy` in `atomar.rs:156` ohne `take`, ohne Obergrenze; der Zweig `Unlesbar` reicht den Deskriptor bei `mod.rs:596` weiter |
| `0911_o_` acht Verweise in `editor.rs` | alle acht stehen; die teure bei `:4854` ist die Handlungsanweisung |
| `0912_o_` neun Stellen sagen „vier Dateien" | bestehen; die Erhebung ist unvollständig, siehe Abschnitt 5 |
| `0913_o_` „die vier übrigen Gründe" | `mod.rs:213`; `einzelheit` läuft über drei ohne Kopie und einen mit |
| `0914_o_` Feld `schalter` mit falscher Begründung | `zettel.rs:157`; `setSelectedSegment` steht allein in `zeigen` (`:371`) und in keinem der zwei Abweisungszweige von `tab_gewechselt` (`:250`, `:260`) |
| `0915_o_` zerrissene Zeichenkette | `operationen.rs:1315`, vierzehn Leerzeichen |
| `0916_o_` `NSSegmentedControl` seit 10.0 | `zettel.rs:79-81`, die Klasse steht in der 10.0-Liste |

**Die zwei geschlossenen Datensätze halten**, beide mit Bestätigungsnotiz versehen. Die Zusage
„jeder abweichende Zettel" ist am Modell nachgelesen, und die vier Zählproben aus Schritt 14
sind ohne Anpassung grün geblieben — das ist der Beleg dafür, dass die Behebung die eine
Erklärung mit ihren vier Aufrufern nicht angefasst hat.

---

## 5. Die zwei beauftragten Prüfpunkte, und was daneben aufgefallen ist

### Der eine Durchgang beim Beenden

**Nachgezählt: in `wird_beendet` (`anwendung.rs:842`) steht genau ein `unter_der_sperre`**
(`:860`), und der vierte Sicherungsmoment liegt darin (`:884`), neben dem Sitzungsschreiber. Der
Kommentar zum Defekt `260813-0540` steht unverändert darüber (`:845-851`), und die frühe
Rückkehr ist ein `if let Some(schreiber)` **innerhalb** dieses einen Durchgangs geworden. Die
Behauptung der Durchsicht ist bestätigt und nicht übernommen.

Der `let _ =` am vierten Moment trägt beide Hälften seines Grundes (`:876-884`): dass es dort
keine Statuszeile mehr gibt, und dass der Nutzer von einem Fehlschlag deshalb nichts erfährt —
der Preis aus C4 samt Verweis auf die Alternative unter „Ausdrücklich außerhalb dieser Runde".

### Die drei Zulässigkeitsregeln

**Byte für Byte unverändert, gemessen und nicht gelesen.** `kommandos/zulaessigkeit.rs` und
`kommandos/operationen.rs` sind gegen `6d05bef` verglichen, jeweils bis zum `#[cfg(test)]`
abgeschnitten: 10 718 zu 10 718 und 43 895 zu 43 895 Zeichen, identisch. Geändert sind allein
Prüfmodule und Doc-Kommentare. `zulaessig` behält seine vier Bestandteile,
`waehrend_blatt_erlaubt` seine eine Ausnahme, `immer_erreichbar` seine drei Einträge.

### Die sieben Planabweichungen, stichprobenartig gegengelesen

Drei der sieben sind einzeln nachgeprüft, die vier übrigen an ihrer Fundstelle gesichtet. Keine
ist widerlegt.

- **Abweichung 1**, `Textstand::KeinGueltigesZiel` trägt `fehlt: bool`: `text_laden` trennt die
  fehlende Datei über genau dieses Feld (`mod.rs:578`) und macht daraus einen leeren Zettel ohne
  Meldung, während der andere Zweig eine `Ersetzung` mit `Beiseite::Nicht` liefert (`:581-592`).
  `oeffnen` wirft das Feld mit `..` weg (`datei.rs:515`). Ohne das Feld wäre C5 sechstes
  Kriterium nicht einlösbar. Trägt.
- **Abweichung 5**, der Tabklick sicherte schon in Strang C: `zettel_zurueckschreiben` kommt im
  Baum nicht mehr vor, und `das_sichern_des_zettels_ist_genau_einmal_erklaert`
  (`anwendung.rs:6649`) hält es. Trägt.
- **Abweichung 6**, `applicationWillTerminate:` kehrt nicht mehr früh zurück: siehe oben, ein
  Durchgang. Trägt.

### Der C4-Nachtrag: passen Spec und Plan zusammen?

**Ja, in jeder Zusage.** Die zwölf am Baum nachweisbaren Kriterien von C4 sind einzeln gegen den
Baum gelesen, und alle zwölf halten; der Plan deckt sie über die Schritte 10 bis 14 ab, und die
sechs am 260814-0941 nachgezogenen Stellen tragen die zwei neuen Zusagen wörtlich. Das siebte
vom Nachtrag berührte Kriterium, das in C5 zur von außen geänderten Zetteldatei, trägt seine
Einschränkung mit Verweis auf C4.

**Zwei Stellen des Plans sind nicht mitgezogen worden**, beide ohne Widerspruch zum Spec und als
Datensatz abgelegt (`issues/260814-1002_o_zwei-stellen-des-plans-sind-mit-dem-nachtrag-vom-0941-nicht-mitgezogen-worden.md`):
die `**Decidability:**`-Zeile nennt als dritte Eingabe „den offenen Zettel", wo es seit dem
Nachtrag der gehaltene Stand **beider** Zettel ist; und `## Testing Strategy` führt für das
Zettelmodell drei Gegenstände und kennt den vierten nicht. Beide sagen weniger, als der Bau
leistet, und nicht etwas anderes.

**Der Satz „Der Zettel liest seine Datei bei jedem Öffnen neu" steht in der Prosa zum
Leseweg-Bild unbeschränkt da**, und das ist kein Rest: dort trägt er die Aussage „also gibt es
keinen Startpfad daneben", und die hält unverändert. Gelesen wird weiter bei jedem Öffnen; was
aus dem Gelesenen wird, entscheidet seit dem Nachtrag das Modell. Schritt 12 schreibt genau
diese Unterscheidung aus.

### Zwei Erhebungen sind eine Stelle zu kurz

Beides sind Zähldefekte in Datensätzen dieser Runde, beide niedrig, beide abgelegt.

- **Die Directive-Abweichung steht an drei Stellen und nicht an zwei.** `_t_circle.md:48`
  zählt die Momente einzeln auf und nennt drei; der Datensatz vom 260814-0637 nennt nur die
  Directive-Zeile und die Überschrift darüber. Die aufzählende Stelle ist die, die beim
  Berichtigen am meisten verlangt (`issues/260814-1002_o_die-directive-abweichung-steht-an-drei-stellen-…`).
- **Die Erhebung zu „vier Ablagedateien" nennt neun Stellen, ihr eigenes Muster liefert
  siebzehn.** Die Auswertung hat `crates/krk-core/tests/` weggelassen, ohne es zu sagen; drei
  der übersehenen sind dieselbe Sorte (`issues/260814-1002_o_die-erhebung-zu-vier-ablagedateien-…`).

**Und ist die Directive-Abweichung mit dem C4-Nachtrag weiter aufgegangen? Nein, und das ist
geprüft.** Der Nachtrag betrifft den Vorrang der beiden Stände und die Zahl der geschriebenen
Zettel je Moment. Über beides sagt der Circle-Datensatz nichts: `## Directive` nennt keinen
Vorrang, und `## Grounding snapshot` erwähnt das Neulesen an keiner Stelle — nachgesehen über
`drei`, `Sicherungsmoment`, `shift+cmd+w`, `liest`, `neu` und `Punkten`. Der Spec sagt es selbst:
„Die Directive ist unangetastet." Es bleibt bei der einen Abweichung, der Zahl der Momente.

---

## 6. Der blinde Fleck hat zweimal zugeschlagen, beide Male bei diesem Abgleich

`CLAUDE.md` führt als Eigenschaft dieses Projekts: „Jedes Suchmuster dieses Projekts, das `\.md`
verlangt, hat einen blinden Fleck." Er ist breiter als die Endung.

- **Verweise in Kurzform.** Ein `grep` auf den vollen Dateinamen des Plans findet zwölf
  Verweise; zwei weitere schreiben ``260814-0656_o_plan-…`` mit einer Ellipse und entgehen ihm,
  beide in den geschlossenen Defektdatensätzen der Durchsicht. Erst ein Muster auf
  `260814-0656_o_plan-` findet sie.
- **Zahlen über einem Zeilenumbruch.** Ein `grep` auf `79 Funktionen` findet in
  `krk-ui/src/appkit/menue.rs` zwei der drei Stellen; bei `:799-801` liegt der Umbruch zwischen
  „79" und „Funktionen". Dieser Abgleich hat daraus zuerst geschlossen, die Stelle sei
  verschwunden, und die Schlussfolgerung war falsch. Die Berichtigung steht im Datensatz
  `shared/issues/260813-1345_o_fuenf-stellen-nennen-79-funktionen-…`, mit dem Hinweis für den
  nächsten Zähler: die Zahl **ohne** ihr Substantiv suchen.

---

## 7. Die Grundlage über alle Speicher

**Neunzehn Fragen sind offen**, sieben im gemeinsamen Speicher und zwölf über sieben Circles.
Keine widerspricht der Directive dieser Runde. Zwei tragen eine Grundlage, die diese Runde
verschoben hat, und beide liegen außerhalb dieses Circles:

- `shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`
  spricht von „dieselben **vier** Dateien" unter `~/Library/Application Support/KRK/`. Es sind
  sechs. Der Sache nach ändert das nichts: die Frage ist, was zwei Instanzen teilen, und die
  zwei Zettel teilen sie genauso.
- `circles/260813-0100-…/decisions/260813-0320_o_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`
  nennt `Esc` als die eine Stelle im Baum mit zwei Empfängern. Es sind jetzt drei: der Wächter
  des Zettels ist der dritte. Der Fall widerspricht der Frage nicht — der Weg des Zettels hängt
  gerade daran, dass seine Fläche **nicht** in `ersthelfer_gehoert_appkit` angemeldet ist —,
  aber die Aufzählung im Datensatz ist damit eine kürzer als der Baum.

Beide liegen in Speichern, die dieser Abgleich nicht beschreiben darf; der zweite gehört dem
Circle einer geschlossenen Runde. Sie stehen hier, damit die nächste Runde sie nicht als neuen
Befund entdeckt.

**Die eine Frage, die alles trägt, steht weiter offen:** wie KRK für den Abnahmelauf in den
Vordergrund kommt
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_o_…`). Sie ist der
Grund, aus dem acht Runden dieses Projekts beschränkt geschlossen haben, und sie hält keinen
Planschritt der neunten auf.

---

## 8. `CLAUDE.md` ist um fünf Runden zurück

Die Datei sagt „Vier Runden sind gefahren" (`:11`) mit einer Tabelle von vier Zeilen und datiert
den Projektstand auf 260811-2230 (`:32`). Der Baum steht bei der **neunten**. Drei Zahlen des
Absatzes zu den gewachsenen Aufzählungen sind mit dieser Runde erneut unrichtig geworden:
`Kommando` trägt 77 statt der genannten 68, die Belegung 83 Funktionen mit 90 Kombinationen,
und die Untergrenzen-Deckung liegt bei 38 von 40 Dateien unter `appkit/` — ohne den Abschnitt sind wie bisher nur `koordinaten.rs` und `mod.rs`, beide begründet.

**Die Revision gehört nicht in einen Abgleich.** Zwei Datensätze im gemeinsamen Speicher halten
die Zähldefekte schon fest, und beide haben mit diesem Abgleich ihre Notiz für die Runde 9
bekommen (`260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-…`,
`260813-1345_o_fuenf-stellen-nennen-79-funktionen-…`). Die Häufung ist der Anlass, `/fusion:revise-claude-md`
anzusetzen, und nicht ein weiterer Datensatz.

---

## 9. Was dieser Abgleich geändert hat

**Marker und Stände**

- `decisions/260814-0656_a_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md` → `_i_`,
  `**Status:** answered` → `implemented`, `Implemented:`-Zeile mit `bfea397` und Fundstelle.
- `decisions/260813-2348_a_was-tut-der-zettel-mit-einer-zetteldatei-die-er-nicht-lesen-kann.md`
  → `_i_`, ebenso, mit `9362034`.
- `planning/260814-0656_o_plan-…` → `_c_`, `**Status:** Draft` → `Complete`, dazu ein
  `## Reconciliation Log` und zwei am Plan-Tor angenommene offene Fragen abgehakt.

**Nicht geändert**

- Der Spec behält seinen offenen Marker. Er schließt mit der Runde, so wie in der Runde 8, und
  29 seiner 72 Kriterien sind Nutzerarbeit.
- Die 72 Kriterien-Kästchen sind nicht abgehakt. Die Aufteilung nach Nachweisweg steht in
  Abschnitt 2 dieses Protokolls; ein Häkchen am Kriterium behauptete eine Abnahme, die erst der
  Lauf des Nutzers gibt.
- `_t_circle.md` und `agentstate.yaml` sind unberührt. Beide zeigen auf den alten Dateinamen des
  Plans und gehören dem Orchestrator; der Datensatz dazu ist
  `issues/260814-1002_o_zwei-lebende-zeiger-auf-den-plan-sind-mit-seiner-schliessung-gestorben.md`.

**Angemerkt, ohne Eingriff in den Inhalt**

- Die drei Durchsichten tragen je eine Abgleichnotiz. Keine ihrer Aussagen ist widerlegt; die
  Zusage zu den drei Zulässigkeitsregeln ist stärker belegt als dort behauptet.
- Die zwei geschlossenen Defektdatensätze tragen eine Bestätigungsnotiz mit Fundstellen.
- Die zwei Zähldatensätze im gemeinsamen Speicher tragen ihre Notiz für die Runde 9.

**Sieben neue Defekte, alle im Circle** (`issues/260814-1002_o_…`), weil alle sieben aus dieser
Directive entstanden sind und nicht bloß hier aufgefallen: die zwei fehlenden Proben, die
Zeilennummer an C5, die zwei nicht mitgezogenen Planstellen, die dritte Stelle der
Directive-Abweichung, die zweimal doppelte Kombination in C1, die unvollständige Erhebung zu
„vier Dateien", und die zwei gestorbenen Zeiger auf den Plan.

**Kein Datensatz ist falsch abgelegt.** Die beiden Bestandsaufnahmen `0628_o_` und `0637_o_`
sind daraufhin gelesen worden: beide beschreiben einen Zustand, der zu berichtigen ist, und
keine legt eine Wahl zwischen Möglichkeiten vor. Sie gehören in `issues/` und nicht in den
Entscheidungsspeicher. Der Ortshinweis in `0628_o_` bleibt gültig: das Muster reicht über diesen
Circle hinaus, und der Nutzer hat den aktiven Circle als Ort vorgegeben.

---

**Status:** Complete
