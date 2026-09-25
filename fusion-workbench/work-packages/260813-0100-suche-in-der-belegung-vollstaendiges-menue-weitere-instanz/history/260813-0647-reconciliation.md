# Abgleich der siebten Runde gegen den Baum

**Agent:** reconciler
**Datum:** 260813-0647
**Domäne:** code
**Stand:** `1cd7788`, Sitzungsspanne `188b81a..HEAD`, 16 Commits
**Circle:** `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/`
**Sitzungsprotokoll:** `shared/history/260813-0040-orchestrator-session.md` (Abschnitt `## Coherence`)
**Kein Bündelbau, kein Vordergrundlauf.** `target/KRK.app` ist unberührt geblieben.

---

## Was in Zahlen geprüft ist

| Gegenstand | Gelesen | Ergebnis |
|---|---|---|
| Planschritte | 15 | alle `[DONE]`, alle am Baum bestätigt |
| Abnahmekriterien C1 bis C4 | 58 | 40 durch eine benannte Probe gehalten, 9 ohne Probe trotz Zusage, 8 teilweise, 1 mit Absicht ohne |
| Geschlossene Defekte mit `Resolved:`-Zeile | 18 | keiner widerlegt, 4 mit einer unzutreffenden Nebenbehauptung |
| Offene Defekte im Circle vorher / nachher | 4 / 9 | fünf aus diesem Abgleich neu abgelegt |
| Offene Fragen, zwei Speicher / alle Speicher | 10 / 19 | vier davon gebaut und trotzdem unbeantwortet |
| Tote Verweise in lebenden Dokumenten | 39 | 2 aus dieser Runde, hier berichtigt |
| Bau | — | `cargo test --workspace` und `cargo clippy --workspace --all-targets -- -D warnings` grün, exit 0 |

**Zur Probenzahl.** Die Commit-Nachricht von `dff167a` nennt „1003 Proben über 19 Ziele". Der
Workspace führt **16** Ziele — eine Bibliothek, drei Binärprogramme, zwölf Prüfziele — mit
**1001** Proben, dazu fünf übersprungene Kindproben in `tests/ablage.rs` und zwei
Kindprozessläufe, die sich selbst noch einmal aufrufen. Zählt man die Ergebniszeilen von
`cargo test` naiv, kommt man auf 1003 und 19. Beide Zählweisen sind vertretbar; grün ist der
Baum in beiden. Kein Befund, nur eine Anmerkung für den, der die Zahl zitiert.

---

## 1. Halten die Marker gegen den Baum?

**Ja, in der Sache alle achtzehn.** Jede `Resolved:`-Zeile ist an den Stellen nachgelesen
worden, die sie nennt: existiert die Datei, steht der behauptete Satz dort, existiert die
genannte Probe, läuft sie grün, ist die Änderung in der Sache eingetreten und nicht nur im
Kommentar. Kein Datensatz behauptet eine Behebung, die es nicht gibt.

**Der schwerste ist auch der bestgebaute.** `der-messmodus-schreibt-die-sitzung-ohne-sitzungsrecht`
ist am Typ behoben und nicht mit einem Satz: `Sitzungsschreiber::neu` und `::mit_takt`
verlangen ein `&Sitzungsrecht` und liefern `Option<Self>`
(`crates/krk-core/src/ablage/sitzung.rs:452-467`). Die Probe
`ohne_sitzungsrecht_entsteht_kein_sitzungsschreiber` prüft alle drei Fälle. Daran kann niemand
mehr vorbeilaufen, und genau das war der Fehler.

### Vier Nebenbehauptungen, die wörtlich nicht stimmen

Jede ist an ihren Datensatz angehängt worden; keiner ändert seinen Marker, weil der jeweilige
Gegenstand behoben ist.

| Datensatz | Behauptung | Was zutrifft |
|---|---|---|
| `eine-vierte-pruefordner-fassung` | „Der Punkt bleibt in `shared/issues/260810-1925_*` aufgehoben", Fundstelle `sys.rs:950` | Jener Datensatz ist **geschlossen** und handelt von `krk-bench/src/messen.rs`. Es sind **zwei** Proben, an `crates/krk-core/src/verzeichnis/sys.rs:962` und `:1004` |
| `weitereinstanz-fragt-den-buendelort-zweimal` | die Zählprobe „läuft unveraendert gruen" | Sie ist in `dff167a` mitgeändert worden (Pfaderwartung mit Kistenpräfix). Von der `NSURL`-Umstellung ist sie nicht berührt, und das war gemeint |
| `ein-kommentar-in-blaetter-mod-rs` | die Begründung darunter „steht unveraendert" | Ein Wort ist mitgewandert: „ohne das Loeschen" heißt jetzt „ohne das Ueberschreiben" |
| `die-zaehlproben-in-krk-ui` | „Beide Modulkoepfe sagen es jetzt" | Der Verweis besteht beidseitig, aber nur einer steht im Modulkopf; der andere am Doc-Kommentar von `quelldateien` (`crates/krk-core/tests/gemeinsam/mod.rs:217-227`) |

### Ein Rest, den niemand hält

Der erste der vier ist mehr als eine Ungenauigkeit. Ein schließender Datensatz hat einen
ausdrücklich **nicht** behobenen Rest in einen geschlossenen, fremden Datensatz geschoben; der
Rest ist damit mit dem Schließen verschwunden. Abgelegt als
`issues/260813-0644_*_ein-rest-ist-in-einem-geschlossenen-fremden-datensatz-aufgehoben-worden.md`.

Der zweite verlorene Punkt kommt aus der Durchsicht selbst: von den vier Zuträgen des
`ontorev` an bestehende Datensätze ist einer nirgends angekommen — der dritte Gegenbeleg zur
`opt+cmd`-Reihenordnung, der an `circles/260812-1000-…/issues/260812-1527_*_…` gehört. Abgelegt
als `issues/260813-0643_*_ein-zutrag-des-ontorev-an-die-runde-6-ist-nirgends-eingetragen.md`.

---

## 2. Stimmen die Zitate?

**In dieser Runde sind zwei Verweise gestorben, beide im Plan, beide auf denselben Datensatz.**
Die Zeilen 318 und 505 nannten `issues/260813-0201_o_ein-kommentar-in-blaetter-mod-rs-…md` mit
ausgeschriebenem Marker; die Datei trägt seit Turn 2 `_c_`. Beide stehen jetzt in Sternform.
Das ist derselbe Fehler wie zweimal in der Runde 6.

**Der Circle-Datensatz dieser Runde ist sauber.** Er zitiert durchgehend in Sternform, fünf
Stellen, keine ausgeschriebene. Das ist die Verbesserung gegenüber der Runde 5, deren
`_b_circle.md` neun tote Verweise trägt.

**Der bekannte blinde Fleck hat diesmal nichts verdeckt.** Gesucht ist ohne Endungsanker
worden, mit `[0-9]{6}-[0-9]{4}_[a-z]_[a-z0-9-]+`. Ergebnis: **kein einziger toter Verweis in
einem lebenden Dokument steht in Kurzform.** Alle 39 tragen `.md`. Tote Kurzform-Verweise gibt
es, 97 Vorkommen, aber ausnahmslos in `history/`, `issues/` und `reviews/` und damit unter der
Ortsregel zulässig.

### Der Gesamtbestand, den die Erhebung nebenbei geliefert hat

39 tote Verweise in lebenden Dokumenten, davon 10 schon in den zwei offenen Datensätzen vom
260812-2253 erfasst und 29 nirgends. Sie verteilen sich auf drei Nester: die
Warteschlangen-Datei `shared/planning/260811-1420_c_abgearbeitete-warteschlange-…md` (10),
den Plan der Runde 5 (8) und ihren `_b_circle.md` (9). Dazu drei in
`crates/krk-ui/src/appkit/editor.rs`, die niemand führt.

**Zwei Stellen sind den bestehenden Datensätzen entgangen.** `_b_circle.md:89` der Runde 5
wiederholt den Verweis aus Zeile 22, und der Fließtext jenes Datensatzes sagt „sieben", wo
seine Tabelle acht Zeilen führt und mit Zeile 89 neun stehen. Und
`circles/260812-1000-…/planning/260812-1145_c_…:537` ist die dritte Fundstelle desselben
Rechtsklick-Datensatzes; der Defekt der Runde 6 kennt nur eine.

Das ist Arbeit der Runden 5 und 6 und nicht dieser. Sie ist hier nicht abgelegt worden, weil
beide Speicher außerhalb der zwei Speicher dieses Circles liegen; die Zahlen stehen hier,
damit die nächste Erhebung sie nicht ein viertes Mal ermitteln muss.

**Vierzehn Verweise sind lebend und trotzdem fragil.** Spec (4) und Plan (10) nennen die vier
Fragen vom 260813-0053 mit ausgeschriebenem `_o_`. Sie sterben in dem Augenblick, in dem der
Nutzer eine davon beantwortet.

---

## 3. Die offenen Fragen und was sie binden

**Zehn über die zwei Speicher dieses Circles, neunzehn über alle.** Alle Speicher binden
weiter, auch die der geschlossenen Runden.

### Der eigene Speicher der Runde 7 (3)

| Frage | Bindet |
|---|---|
| `260813-0159_*_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md` | S4, über S4 auch S5 und S6; C2.2, C2.3, C2.13. Eine andere Antwort wirft **einen** Schritt um, dafür ist S4 eigens geschnitten |
| `260813-0320_*_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md` | die Schluckregel aus S3 und den ersten der zwei hingenommenen Verluste. Der saubere Weg (Möglichkeit 3, `hasMarkedText` als vierter Wert der `Lage`) verdoppelt die Tafel auf 280 Fälle und ist ohne Umbau nachziehbar |
| `260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md` | `menuemodell::zugestellte_kuerzel`, den Defekt `260813-0416` und zwei Sätze in `resources/default-keymap.toml`, die bewusst noch nicht geschrieben sind |

### Der gemeinsame Speicher (7)

| Frage | Bindet |
|---|---|
| `260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-…` | C1.16, Schritt S9. **Gebaut auf der Empfehlung:** Cmd+T, Cmd+Eingabe, Cmd+R |
| `260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md` | C2.3, Schritte S5 und S6. **Gebaut:** neun Obermenüs, eines je Funktionsbereich |
| `260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md` | C3.7 bis C3.11, Schritte S11 bis S14. **Gebaut:** Schreibsperre je Durchgang, Sitzungsrecht beim Start. Der Datensatz sagt daneben mehr zu, als der Bau hält — siehe Punkt 4 |
| `260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md` | C2.15, Schritte S1 bis S3 und S6. **Gebaut:** den zulässigen. Trägt den `esc`-Verlust |
| `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` | C4.4. Die Deckung ist zum vierten Mal von Hand nachgezogen worden (35 von 37), und der Datensatz führt drei Stufen mit Kosten |
| `260802-0842_*_code-sdk-fuer-ki-integration.md` | nichts Gegenwärtiges. Die KI-Anbindung liegt außerhalb aller sieben Runden |
| `260802-0842_*_git-verwerfen-bedeutung.md` | die spätere Git-Anbindung. Außerhalb aller sieben Runden |

**Vier davon sind gebaut und trotzdem offen, und das ist die auffälligste Lage der Runde.**
Der Baum hat je eine Möglichkeit umgesetzt, die aktive Grundlage sagt weiterhin
„unentschieden". Solange das so steht, ist jede der vier ein Änderungsauftrag auf Abruf.

### Die neun in den Speichern anderer Runden

Sie binden ebenso. Fünf aus der Runde 1 (Verfügbarkeitsprüfung für macOS-26-Schnittstellen,
**wie KRK für den Abnahmelauf in den Vordergrund kommt**, welche Sprache die Sortierordnung
bestimmt, ob der Auffrischungsaufschub entfallen kann, ob die Markierung eine Auffrischung
überlebt), eine aus der Runde 3 (ob ein Kommentar den Rang der Statuszeile als Zahl nennt),
drei aus der Runde 6 (Rechtsklick auf eine unmarkierte Zeile, Schriftgröße der Vorschau,
Vorspann eines Containers als Lücke in C4.3).

**Eine davon trägt alles andere.**
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`
steht seit dem 260806 offen. Sie ist der Grund, aus dem alle sieben Runden beschränkt
abschließen, und die einzige, deren Beantwortung diese Reihe beenden würde.

---

## 4. Tragen die Begründungen der vier offen gelassenen Defekte?

**Ja, alle vier — aber zwei sind der Sache nach keine Defekte mehr.**

**`260813-0540_o_die-belegung-wird-weiter-blind-ueberschrieben…` (gehört dem Nutzer).** Trägt.
Am Baum bestätigt: der Lesen-Ändern-Schreiben-Durchgang ist für die Lesezeichen gebaut
(`crates/krk-ui/src/appkit/anwendung.rs:1505-1530`), die Belegung geht weiter blind darüber
(`:3039-3056`). C3.7 ist trotzdem erfüllt, die Sperre hält, ein Gemisch kann nicht entstehen —
nur die verlorene Änderung bleibt. Spec und Plan verlangen das frische Lesen ausdrücklich
allein für die Lesezeichen; der Bau folgt beiden. **Die Zusage im Entscheidungsdatensatz ist
die weiter reichende, und die hält der Baum nicht.** Der `coderev` hat diesen Befund als
einzigen an die erste Stelle seiner Reihenfolge gesetzt, und er ist der einzige seiner sechzehn,
der stehengeblieben ist. Das ist die richtige Auswahl. **Anmerkung:** der Datensatz ist der
Form nach eine Entscheidung und kein Defekt — er legt zwei Wege vor und schließt mit „Der
Nutzer entscheidet". Siehe unten unter „Falsch abgelegt".

**`260813-0420_o_das-menue-bearbeiten-verliert-seine-mac-uebliche-reihenfolge…`.** Trägt. Die
Vorbedingung ist erfüllt: der Absatz zur Blockreihenfolge steht seit Turn 2 im Kopf von
`resources/default-keymap.toml:35-42`. Die Verschiebung selbst bewegt zwei
`[[funktion]]`-Blöcke, ist damit eine Änderung an Daten und nicht an Kommentaren, und der
Auftrag des Turns hat die Datenzeilen ausdrücklich ausgenommen. Die Trennerfrage bleibt
daneben offen, mit drei Wegen und Weg 1 empfohlen. Sauber begründet, richtig offen.

**`260813-0416_o_zwei-menueeintraege-mit-cmd-a…`.** Trägt, aber nicht mehr als Defekt. Der
Defekt selbst ist im selben Schritt behoben — der Datensatz sagt es in seiner eigenen
Schwere-Zeile: „hoch, wäre er stehengeblieben; **behoben** im selben Schritt". Zwei Proben
halten es. Offen ist die **Richtung**, und die liegt als eigener Entscheidungsdatensatz vor
(`260813-0430`). Was der `_o_`-Marker hier hält, sind zwei ungeschriebene Sätze in der
Belegungsdatei, die von jener Antwort abhängen. Das ist eine vertretbare Buchführung, aber es
ist ein Warteposten und kein Defekt.

**`260813-0311_o_ein-klick-in-die-bereichsleiste…`.** Trägt in der Sache, mit **einem
falschen Satz**. Am Baum bestätigt: der Melder der Leiste ruft `kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:983-987`) und erbt den Ersthelferbestandteil; jeder
Schalter trägt weiterhin `setRefusesFirstResponder(true)`
(`crates/krk-ui/src/appkit/bereichsleiste.rs:478`). Die Wahl von Weg 1 ist gut begründet: nach
C2.19 ist während einer Umbenennung ohnehin jeder Menüeintrag grau, die Bereichsleiste gibt
also dieselbe Antwort wie zwei andere Flächen statt einer dritten. **Der Schlussabsatz sagt
aber: „Der Verlust steht auf der Abnahmeliste des Laufs am Bündel." Er steht dort nicht.**
Siehe Punkt 5.

---

## 5. C1 bis C4: was gebaut ist und was am Bündel aussteht

**Vierzig der 58 Kriterien sind durch eine benannte Probe gehalten, die genau das prüft, was
sie zusagt.** Die vollständigen vier Tafeln stehen nicht hier, sondern sind gegen den Baum
gelesen worden; hier steht, was daraus folgt.

### Was am Bündel aussteht

| Fähigkeit | Am laufenden `KRK.app` zu sehen |
|---|---|
| **C1 Suche** | die springende Auswahl beim ersten Zeichen (C1.1); die Meldungszeile mit Suchtext, Trefferzahl und Stelle (C1.9, C1.10); das Verlassen der Ansicht über `esc` (C1.13); die Bedienung der drei Schaltflächen Cmd+T, Cmd+R, Cmd+Eingabe (C1.16) |
| **C2 Menü** | **die fünf Fälle der Ausgrauung** (C2.6) — `up` und `return` im Editor, `up` und `down` beim Umbenennen, `delete` im Textfeld, `space` in beiden Textlagen; das Blattverhalten in der Belegungsansicht (C2.7); Cmd+Q und Shift+Cmd+W während Umbenennung und Blatt (C2.18); dass ein ausgegrauter Eintrag auch mit der Maus tot ist (C2.19); das Bild der neun Obermenüs (C2.3) und das Menü nach einer Umbelegung (C2.11); Opt+Cmd+Q ohne Zweitform „Quit and Keep Windows" |
| **C3 Weitere Instanz** | der Start selbst mit eigenem Fenster (C3.1, C3.5); die Meldung „schreibt die Sitzung nicht" in der Statuszeile (C3.6, C3.10) |
| **C4 Bau** | allein die Richtigkeit der Untergrenzen-Zahlen (C4.4), die am SDK nachzulesen ist |
| **Zeitzusagen** | **L4** (Kaltstart, die einzige, der diese Runde messbar Arbeit hinzufügt: 82 Menüeinträge statt 10, dazu ein Systemaufruf für das Sitzungsrecht), dazu **L1 und L9**, weil eine Herleitung keine Messung ist |

Dazu zwei Punkte, die der `coderev` eigens nennt, weil der Baum sie nicht beantworten kann: ob
Cmd+T und Cmd+R die Schaltflächen der Belegungsansicht überhaupt erreichen, und ob AppKit
`validateMenuItem:` vor jeder Tastenentsprechung erfragt.

### Die zwei hingenommenen Verluste, und wo sie fehlen

**Beide bestehen am Baum, und beide brechen den Spec-Satz „Kein Verlust gegenüber heute".**
Der Spec formuliert die Randbedingung als Alternative: entweder der Befehl steht auf der
benannten Liste aus C2.5, oder der Spec sagt, warum der Weg keine Wirkung hatte. Keiner der
zwei Fälle erfüllt eine der beiden Bedingungen. `abbrechen` steht nicht auf jener Liste, die
genau `beenden` und `fenster_schliessen` führt, und beide Wege hatten Wirkung.

1. **`esc` im Editor bricht keine Zusammensetzung mehr ab.** `abbrechen` trägt
   `Wirkungsbereich::Ueberall` (`crates/krk-core/src/tasten/belegung.rs:760`), die Textfläche
   des Editors ist die eine Ausnahme vom Ersthelfervorbehalt, also sagt `zulaessig` ja, und
   `kommando_ausfuehren` liefert seit S3 unbedingt `true`, sobald die Zulässigkeit steht. Der
   Tastendruck erreicht die `NSTextView` nicht mehr. Betroffen ist der Abbruch einer laufenden
   Zusammensetzung einer Eingabemethode. Das ist der schwerere der zwei Verluste, weil er einen
   Weg abschneidet, der außerhalb von KRKs eigener Regelwelt liegt.
2. **Ein Klick in die Bereichsleiste wirkt während einer Umbenennung nicht.** Der geringere und
   der konsequentere: er macht die Bereichsleiste zur dritten Fläche mit derselben Antwort statt
   zur Ausnahme.

**Keiner von beiden steht auf der Abnahmeliste des Plans.** Die Wörter „Bereichsleiste",
„Zusammensetzung", „Eingabemethode" und „Vorschau-Schalter" kommen im ganzen Plan nicht vor.
Beide sind bisher abgeleitet und nicht gemessen, und beide sind billig zurückzunehmen, wenn
der Lauf sie bestätigt. Abgelegt als
`issues/260813-0642_*_zwei-hingenommene-verluste-stehen-auf-keiner-abnahmeliste.md`.

### Die C4-Zahlen, nachgezählt

| Größe | vor der Runde | heute | Spec | Ergebnis |
|---|---|---|---|---|
| `Kommando` | 75 | 76 | 75 → 76 | stimmt |
| `Wirkungsbereich` | 7 | 7 | wächst nicht | stimmt |
| `Bereich` | 5 | 5 | wächst nicht | stimmt |
| `Fokus` | 5 | 5 | wächst nicht | stimmt |
| `Funktionsbereich` | 9 | 9 | wächst nicht | stimmt |
| `default-keymap.toml`, Funktionen | 81 | 82 | 82 | stimmt |
| `default-keymap.toml`, Kombinationen | 87 | 88 | 88 | stimmt |
| Dateien mit `#![allow(unsafe_code)]` | 2 | 2 | keine dritte ohne eigenen Schritt | stimmt, `flock` ist in `sys.rs` gelandet |
| Prüfordner-Fassungen | 3 | 3 | drei | stimmt, die Gegenprobe sucht seit `dff167a` die Sache statt den Namen |

`opt+cmd+n` war vorher unbelegt: der Diff über alle `id`- und `tasten`-Zeilen zwischen
`188b81a` und HEAD zeigt genau zwei hinzugefügte Zeilen und keine geänderte.

### Neun Kriterien versprechen eine Probe und haben keine

C2.12, C2.13, C2.15, C3.12, C4.1, C4.3, C4.4, C4.7, C4.8. Acht weitere sind nur teilweise
gedeckt. Kein Kriterium ist falsch — alle sind nachgeprüft und treffen zu —, aber die
Kennzeichnung `(Probe)` sagt Abnehmbarkeit zu, die neunmal nicht eingelöst ist. Abgelegt als
`issues/260813-0647_*_neun-abnahmekriterien-versprechen-eine-probe-und-haben-keine.md`.

**Eine davon ist überholt und nicht nur ungeprüft.** C2.13 ist am 260813-0445 über
`--menue-protokoll` gemessen worden, bei **81** Einträgen. S15 hat mit `weitere_instanz` den
82. gebracht, und danach ist nicht mehr gemessen worden. Ungemessen bleibt genau der Fall, den
das Kriterium benennt: ob AppKit dem neuen Eintrag eine Zweitform beistellt. Abgelegt als
`issues/260813-0646_*_die-messung-zu-c2-13-ist-bei-81-eintraegen-gefahren-und-s15-hat-den-82-gebracht.md`.
Der Lauf kostet Sekunden und braucht kein Bündel.

---

## 6. `CLAUDE.md` ist an fünf Stellen überholt

Die Revision gehört nicht in einen Abgleich; der Befund schon. Der Plan selbst schiebt sie
unter `## Nicht Gegenstand dieses Plans` ausdrücklich „an den Schluss der Runde".

| Stelle | steht da | trifft zu |
|---|---|---|
| `:11` und die Tabelle darunter | „Vier Runden sind gefahren", vier Zeilen | **sechs** beschränkt abgeschlossen, diese ist die **siebte**. Es fehlen die Runden 5 (Statusleiste), 6 (Teilen, Ordnersprung, Ablage, Vorschau) und 7 |
| `:32` | „Geprüft am 260811-2230" | zwei Tage und zwei Runden alt |
| `:54` | „Alle vier Runden sind als beschränkter Abschluss geschlossen" | alle **sechs**, aus demselben Grund |
| `:66` | „`Kommando` … 68 Varianten" | **76** (`crates/krk-core/src/tasten/belegung.rs:304`). Eigener offener Datensatz, dessen eigene Gegenzahl 75 ebenfalls überholt ist |
| `:156` | „Außerhalb der vier gefahrenen Runden" | außerhalb der **sechs** |
| `:158` | „Zwei Circles sind vorgesehen und nicht gefahren (Marker `_a_`)" | **einer**. Die Statusleiste (`260811-1304-…`) trägt `_b_` und ist als Runde 5 gefahren; vorgesehen ist allein der Web-Betrachter (`260804-0933-…`) |

Nicht in der Tabelle, weil ein eigener Datensatz sie führt: die Zahl „31 von 33 Dateien" im
Abschnitt „Technologiewahl" ist heute **35 von 37**, der vierte Stand in vier Tagen. Beide
Zahldefekte sind in ihren Datensätzen mit dem heutigen Stand fortgeschrieben worden
(`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-…`,
`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-…`), und beide legen dieselbe
Wahl vor: die Zahl bei jeder Runde nachziehen, oder sie durch etwas ersetzen, das nicht
mitwächst.

---

## 7. Nebenbefunde am Verfahren

**Der Turn 2 ist von keiner Durchsicht gelesen.** Beide Durchsichten decken `ca66c39..40b5fb0`.
`dff167a` liegt dahinter und hat nicht nur Kommentare angefasst: es hat die Bauform der
Zählproben dieses Baums geändert und `quelldateien` auf `crates/` umgestellt, also neun
Erwartungen in sieben Dateien mitgezogen. Der Baum ist grün und alle achtzehn Behebungen sind
hier einzeln nachgelesen, aber gelesen hat sie kein zweites Augenpaar.

**Das Ereignisprotokoll reißt beim zweiten Turn ab.** `turn_start` um `03:51:01` ist die letzte
Zeile: kein `task_start`, kein `task_done`, kein `commit`, kein `turn_end`, obwohl zwei Commits
folgten. `orchestrator-live.md` steht unverändert auf „Turn 1/5, Tasks 6/15, Commits 9",
während `agentstate.yaml` „turn 2, 15 von 15, 16 Commits" führt. Beides ist an die zwei
bestehenden Datensätze angehängt (`shared/issues/260810-1945_*_…`,
`shared/issues/260811-2157_*_…`); der zweite Turn ist der schärfere Fall, weil diesmal nicht
nur das `turn_end` fehlt, sondern der ganze Innenteil.

**Das Sitzungsprotokoll trägt keinen Turn-Eintrag.** `## Per-Turn Log` in
`shared/history/260813-0040-orchestrator-session.md` ist leer, und `**Status:**` steht auf „In
Arbeit". Die zwei Turns sind im Circle-Datensatz beschrieben, nicht dort. Das gehört dem
Orchestrator und ist hier nicht angefasst.

**`agentstate.yaml` zeigt seit diesem Abgleich auf einen alten Plannamen.** `plan_file` nennt
`260813-0205_o_plan-…md`; die Datei heißt jetzt `_c_`. Die Zeile gehört dem Orchestrator und
liegt außerhalb dessen, was ein Abgleich schreiben darf. Bei sauberem Sitzungsende wird die
Datei ohnehin gelöscht; bricht die Sitzung vorher ab, ist die eine Zeile nachzuziehen.

**`portfolio.md` ist vom 260812-2307 und kennt diese Runde nicht.** Es zählt sieben Circles,
keinen aktiven. Der `playmaker` erzeugt es beim Abschluss neu; kein Befund.

---

## Falsch abgelegt — gehört in den Entscheidungsspeicher

Zwei der offenen Defekte sind der Form nach Entscheidungen. Beide legen Möglichkeiten mit
Kosten vor und schließen mit einer Wahl, die dem Nutzer gehört, nicht mit einer Stelle, an der
etwas kaputt ist:

- `issues/260813-0540_o_die-belegung-wird-weiter-blind-ueberschrieben-obwohl-der-datensatz-mehr-zusagt.md`
  — „Zwei Wege, und der zweite ist der ehrlichere", Schlusssatz „Der Nutzer entscheidet".
- `issues/260813-0416_o_zwei-menueeintraege-mit-cmd-a-und-appkit-nimmt-dem-spaeteren-das-kuerzel.md`
  — der Defekt ist behoben; was offen bleibt, ist die Richtung, und die liegt als
  `decisions/260813-0430_*_…` schon im richtigen Speicher.

**Nichts ist verschoben worden.** Der Umzug ist Handarbeit des Nutzers: `mv` aus `issues/` nach
`decisions/` desselben Circles und den Marker von der Defektreihe (`_o_/_p_/_c_/_d_`) auf die
Entscheidungsreihe (`_o_/_a_/_i_/_d_/_s_`) setzen. Beim ersten wäre das ein echter Gewinn,
weil die reichere Reihe „beantwortet, aber noch nicht umgesetzt" ausdrücken kann und die
Defektreihe nicht. Beim zweiten reicht es, ihn zu schließen, sobald `260813-0430` beantwortet
ist.

---

## Was dieser Abgleich geändert hat

**Umbenannt (1):**
`planning/260813-0205_o_plan-…md` → `_c_`, `**Status:** Draft` → `Complete`, dazu ein
`## Reconciliation Log` am Ende. Alle fünfzehn Schritte sind ausgeführt und einzeln gegen den
Baum gelesen.

**Berichtigt (2):** die zwei toten Verweise im Plan, Zeilen 318 und 505, auf Sternform.

**Neu abgelegt (5), alle in `issues/` dieses Circles:**

| Datensatz | Gegenstand |
|---|---|
| `260813-0642_o_zwei-hingenommene-verluste-stehen-auf-keiner-abnahmeliste.md` | die zwei Verluste fehlen auf der Abnahmeliste, und der Spec-Satz ist zweimal verletzt |
| `260813-0643_o_ein-zutrag-des-ontorev-an-die-runde-6-ist-nirgends-eingetragen.md` | der dritte Gegenbeleg zur `opt+cmd`-Reihenordnung ist verlorengegangen |
| `260813-0644_o_ein-rest-ist-in-einem-geschlossenen-fremden-datensatz-aufgehoben-worden.md` | der Griff in das echte Temporärverzeichnis in `verzeichnis/sys.rs` |
| `260813-0646_o_die-messung-zu-c2-13-ist-bei-81-eintraegen-gefahren-und-s15-hat-den-82-gebracht.md` | C2.13 ist vor dem letzten Menüeintrag gemessen worden |
| `260813-0647_o_neun-abnahmekriterien-versprechen-eine-probe-und-haben-keine.md` | die Kennzeichnung `(Probe)` ist neunmal nicht eingelöst |

**Angehängt, ohne Markerwechsel (8):** vier geschlossene Defekte dieser Runde mit der
Berichtigung ihrer Nebenbehauptung; die zwei Durchsichten mit einer Anmerkung, was aus ihren
Befunden geworden ist; vier gemeinsame Defekte mit dem heutigen Stand ihrer Zahlen
beziehungsweise dem Wiederauftreten in dieser Sitzung.

**Nicht angefasst:** Code, Daten, `CLAUDE.md`, `target/KRK.app`, die Speicher der Runden 1 bis
6, `agentstate.yaml`, `orchestrator-live.md`, `portfolio.md`, der Circle-Datensatz.
