# Zweiter Abgleich der neunten Runde: was die Abnahme belegt und was nicht

**Agent:** reconciler
**Datum:** 260814-1247
**Domäne:** code
**Stand:** `a6098d9`, ein Codecommit seit dem ersten Abgleich
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Erster Abgleich:** `history/260814-1002-reconciliation.md` — was dort steht, ist hier nicht wiederholt
**Sitzungsprotokoll:** `history/260813-2342-orchestrator-session.md` (Abschnitt `## Coherence`)
**Kein Bündelbau, kein `cargo xtask`, kein Vordergrundlauf.** `target/KRK.app` ist unberührt geblieben.

---

## Das Ergebnis in einem Absatz

**Der Baum ist so weit, wie ein Agent ihn bringen kann.** Die zwei mittleren Befunde sind
behoben, beide Behebungen sind gegen den Baum gelesen, die drei Kriterien aus dem C5-Nachtrag
halten, `make check` steht auf 0. **Die Abnahme ist es nicht.** Sie hat zwölf Beobachtungen für
29 Kriterien mit Bündelanteil gefahren; acht davon sind belegt, fünf zur Hälfte berührt, und
sechzehn hat keine Beobachtung angefasst. Die Zahl „71 von 72" ist an beiden Enden unrichtig:
die Grundmenge ist seit dem 260814-1010 nicht mehr 72, und der Zähler setzt eine Deckung an,
die die Liste nicht hat.

---

## 1. Was Turn 3 behoben hat, gegen den Baum gelesen

**Beide Behebungen halten, und beide Datensätze stehen zu Recht auf geschlossen.**

| Datensatz | Was der Baum trägt |
|---|---|
| `260814-0910_c_` unbegrenzte Kopie | `quelle.by_ref().take(EDITORGRENZE)` (`crates/krk-core/src/ablage/mod.rs:720`), `Beiseite::Gekuerzt` als fünfter Wert (`:261`) mit eigener Meldung (`:320`), Unterscheidung über `begrenzt.limit()` und `steht_noch_etwas_an` statt über das erschöpfte Budget |
| `260814-0911_c_` acht falsche Verweise | alle acht zeigen jetzt auf `textautomatik::automatiken_abschalten` (`crates/krk-ui/src/appkit/editor.rs:316`, `:4233`, `:4257`, `:4273`, `:4365`, `:4427`, `:4829`, `:4859`); die sieben Stellen, an denen `textflaeche_bauen` als Erzeuger gemeint ist, sind unangetastet |

**`EDITORGRENZE` steht weiterhin an genau einer Stelle im Quelltext** (`crates/krk-core/src/text/datei.rs:164`).
Über `crates/` gezählt: jede weitere Fundstelle liest die Konstante, keine wiederholt die Zahl.
Die Zusage „eine Zahl, zwei Verwendungen" hält damit wörtlich.

**Der Grenzfall ist gebaut und geprobt.** Eine Zetteldatei von genau `EDITORGRENZE` Bytes
schöpft das Budget aus und wird trotzdem als vollständig gemeldet
(`eine_zetteldatei_genau_auf_der_grenze_geht_ganz_beiseite`, `crates/krk-core/tests/ablage.rs:1644`).
Die Probe fällt an der Bytefolge heraus und nicht an der Größe, so wie das Kriterium es
verlangt: das erste Byte ist `0xff`, der Rest ein Loch.

**`make check` beim Abgleich selbst gefahren:** Rückgabewert 0, „alle vier gruen".

---

## 2. Die Zählung: die Grundmenge ist 75 und nicht 72

Derselbe Commit `a6098d9`, der die zwei Befunde behob, hat **drei Kriterien in die erste Liste
von C5 eingetragen**. Die Abnahmeliste ist eine Stunde später geschrieben und rechnet weiter mit
der Zahl von davor.

Am Dateibestand nachgezählt, Zeile für Zeile:

| Fähigkeit | Am Baum | Am Bündel | Zeilen im Spec |
|---|---|---|---|
| C1 | 11 | 8 | 168–178, 181–188 |
| C2 | 4 | 5 | 203–206, 209–213 |
| C3 | 5 | 5 | 226–230, 233–237 |
| C4 | 12 | 7 | 250–261, 264–270 |
| C5 | **14** | 4 | 290–303, 306–309 |
| **Summe** | **46** | **29** | **75** |

**Zwei weitere Kriterien stehen außerhalb der fünf Fähigkeiten**, unter
`## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1` (`:334`, `:335`). Der Spec sagt über
sie: „Sie sind Bestandteil der Abnahme dieser Runde." Der erste Abgleich hat sie ausgelassen,
ohne es zu sagen. Wer sie mitzählt, kommt auf **77 mit 48 am Baum**. Beide Zahlen stehen im
Datensatz, damit die nächste Erhebung die Wahl bewusst trifft.

**Die 46 am Baum halten**, mit denselben drei benannten Einschränkungen wie beim ersten
Abgleich. Zwei davon sind mehr als Zitierfehler und deshalb hier genannt: C1, zweites Kriterium
verlangt „keine Kombination steht danach zweimal", und `cmd+a` steht zweimal; C3, erstes
Kriterium sagt eine Probe zu, welche die **Ausnahmen** der Regel zählt, und die vorhandene zählt
ihre **Erklärungen**. Beide sind seit dem 260814-1002 abgelegt und nicht behoben.

---

## 3. Die Abnahme: acht belegt, fünf halb, sechzehn nie berührt

**Zwölf Beobachtungen für 29 Kriterien.** Jedes Kriterium der zweiten Listen ist gegen die zwölf
Beobachtungen gelesen worden.

| Stand | Anzahl | Kriterien (Zeile im Spec) |
|---|---|---|
| belegt | 8 | 184, 209, 210, 211, 212, 213, 237, 308 |
| teilweise berührt | 5 | 181, 182, 183, 233, 236 |
| nie berührt | 16 | 185, 186, 187, 188, 234, 235, 264, 265, 266, 267, 268, 269, 270, 306, 307, 309 |

**Die fünf halben sind je zur Hälfte gesehen.** `f2` und `cmd+k` sind gedrückt, aber nicht aus
jedem der fünf Bereiche. `Esc` schließt, der zweite Druck auf `f2` ist ungeprüft. Von „kein
Befehl außer den dreien wirkt" sind zwei Befehle probiert. Getippte Zeichen erscheinen, die
Eingabetaste ist ungeprüft. „Leere Textfläche" trägt „keine Zeilennummern" nur mittelbar.

**Drei Gruppen unter den sechzehn wiegen schwerer als der Rest.**

- **Die vier Zwischenablagebefehle in der Zettelfläche** (`:234`): `cmd+v`, `cmd+x`, `cmd+c`,
  `cmd+a`, `cmd+z`. Am Baum ist dafür nichts da, und am Bündel hat es niemand gedrückt.
- **Die sieben Textautomatiken** (`:235`). Am Baum über die Zählprobe aus Turn 1 gedeckt, am
  laufenden Programm ungeprüft — und die Zählprobe hat ihren blinden Fleck ausdrücklich benannt.
- **Die drei Beenden-Kriterien von C4** (`:264`, `:265`, `:266`). Beobachtung 8 fährt Beenden
  und Neustart, prüft aber, **welcher** Zettel offen ist, und nicht, ob der getippte Text steht;
  getippt wird in Beobachtung 8 nicht.

**Warum die Runde 8 an dieser Stelle sauber war und die Runde 9 nicht.** Ihr Spec kennzeichnete
jedes Kriterium einzeln mit `(Probe)` oder `(Bündel)`; zehn trugen `(Bündel)`, und der
Abnahmelauf führte elf Beobachtungen — eine je Kriterium und eine für den Tag. Der Satz „alle 59
Abnahmekriterien abgenommen bis auf eines" war dort nachrechenbar. Die Runde 9 führt zwei Listen
je Fähigkeit statt einer Kennzeichnung je Kriterium, und die Bindung zwischen Beobachtung und
Kriterium ist dabei verloren gegangen.

---

## 4. Die zwei beauftragten Prüfpunkte

### Beobachtung 10: die Begründung trägt für die eine Hälfte

Gestrichen mit der Begründung, die Logik sei am Modell in drei Proben abgenommen und der
Rückgabewert trage seit Turn 2 `#[must_use]`.

**Was hält, und es ist nicht wenig.** Die drei Proben stehen
(`crates/krk-ui/src/zettelmodell.rs:417`, `:438`, `:456`). `#[must_use]` steht an
`Zettelmodell::oeffnen` (`:172`) und ist mit `79dab20` gekommen, also in Turn 2; nachgesehen mit
`git log -S`, ein Treffer. **Beide Aufrufstellen nehmen den Rückgabewert und nicht das
Gelesene** (`crates/krk-ui/src/appkit/anwendung.rs:3305-3312` beim Öffnen, `:3418-3423` beim
Tabwechsel) — von Hand gelesen, nicht erschlossen.

**Was die Begründung zu weit trägt.** `#[must_use]` verbietet allein das stille Fallenlassen. Es
zwingt keinen Aufrufer, den zurückgegebenen Stand in die Textfläche zu setzen statt des
Gelesenen, und genau das wäre der Verlust. `let _ =` davor ist in diesem Baum eine erlaubte und
ausgeschriebene Schreibweise; der Doc-Kommentar an `oeffnen` sagt es selbst. Was die Verdrahtung
hält, sind die zwei richtig geschriebenen Stellen und nicht der Übersetzer. Dazu die zweite
Einschränkung aus `CLAUDE.md`: `unused_must_use` ist erst unter `-D warnings` ein Fehler.

**Was ohne jeden Beleg bleibt: die Meldung.** Sie entsteht in `zettel_sichern`
(`crates/krk-ui/src/appkit/anwendung.rs:3514`) und geht über `zettel_sicherung_melden` (`:3553`)
in die Statuszeile. Keine Probe erreicht diesen Weg, und keine kann es: `krk-ui` hat kein
Bibliotheksziel. Beobachtung 10 war die einzige Stelle im ganzen Abnahmelauf, an der eine
gescheiterte Sicherung überhaupt hergestellt worden wäre. **Mit ihrer Streichung stehen zwei
Kriterien ohne Beleg da**: `:268` zur Hälfte und `:267` ganz.

### Beobachtung 9: kein Abnahmekriterium bleibt unerfüllt

**Nein, und das ist geprüft und nicht angenommen.** Die eigentliche Zusage ist Kriterium C1,
viertes der zweiten Liste (`:184`): „Text tippen, `shift+cmd+w` drücken, das Fenster mit `cmd+n`
zurückholen, `f2`: der getippte Text steht da." Sie ist bestanden.

**Die Messung selbst steht in keiner der fünf Kriterienlisten.** Sie ist Punkt 2 unter
„Nutzerarbeit" des Plans, und der Plan hat sie ausdrücklich aus dem tragenden Weg genommen: das
Sichern läuft unbedingt und vor `performClose:` (`anwendung.rs:3947` sichert, `:3949` schließt),
also hält die Zusage in beiden Ausgängen. Was fehlt, ist die Auskunft über AppKits Verhalten für
eine spätere Runde am Schließweg — und der Vermerk, den der Plan mit dem Lauf gerade vermeiden
wollte, muss jetzt noch einmal geschrieben werden.

---

## 5. Die vierzehn offenen Defekte

**Alle vierzehn sind zu Recht offen**, jeder an der Stelle nachgesehen, die sein Datensatz nennt.
Keiner ist durch Turn 3 stillschweigend behoben worden.

Vier davon sind hier neu nachgesehen, weil Turn 3 ihre Dateien angefasst hat:

| Datensatz | Am Stand `a6098d9` |
|---|---|
| `0912_o_` neun Stellen sagen „vier Dateien" | besteht; die sechs Stellen in `ablage/mod.rs` sind mitgewandert (`:40`→`:45`, `:45`→`:59`, `:359`→`:385`, `:361`→`:387`, `:401`→`:427`, `:442`→`:468`), die Zahl in dieser Datei ist mit acht unverändert |
| `0913_o_` „die vier übrigen Gründe" | besteht; `Grund::einzelheit` (`ablage/mod.rs:218`) läuft weiter über drei ohne Kopie und einen mit |
| `0914_o_` Feld `schalter` mit falscher Begründung | besteht; `zettel.rs:157-159` unverändert |
| `1002_o_` C5 zitiert `EDITORGRENZE` an `:153` | besteht; das Kriterium (`Spec:297`) nennt weiter `:153`, die Konstante steht an `:164`. Der C5-Nachtrag hat drei Kriterien danebengesetzt und die Nummer nicht mitgezogen |

Die zehn übrigen sind unverändert; ihre Belege stehen im ersten Abgleich, Abschnitt 4.

**Kein Datensatz ist falsch abgelegt.** Die vierzehn beschreiben durchweg einen Zustand, der zu
berichtigen ist, und keiner legt eine Wahl zwischen Möglichkeiten vor. Sie gehören in `issues/`
und nicht in den Entscheidungsspeicher.

---

## 6. Die Grundlage

**Neunzehn Fragen sind offen**, unverändert gegenüber dem ersten Abgleich: sieben im
gemeinsamen Speicher, zwölf über sieben Circles, keine in diesem. Keine widerspricht der
Directive dieser Runde.

**Die eine Lücke des ersten Abgleichs ist geschlossen.** Er hielt fest, dass es für die Frage,
wie groß „beiseite" werden darf, keinen Datensatz gibt, obwohl `issues/260814-0910_o_` sie
ausdrücklich „in den Spec oder einen Entscheid" verwies. Der Nutzer hat sie am 260814-1010 am
Rebalance-Tor beantwortet, und die Antwort steht im Spec: drei Kriterien und drei Festlegungen
unter C5, dazu der Abschnitt „Was der Nachtrag vom 260814-1010 an C5 geändert hat". Der Spec ist
einer der zwei Orte, die der Datensatz nannte.

**Die zwei Zahlen außerhalb dieses Circles, die diese Runde verschoben hat, stehen unverändert**
(`shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-…` sagt vier Ablagedateien, es
sind sechs; `circles/260813-0100-…/decisions/260813-0320_o_esc-im-editor-…` nennt zwei
`Esc`-Empfänger, es sind drei). Beide liegen in Speichern, die dieser Abgleich nicht beschreiben
darf.

---

## 7. Was dieser Abgleich geändert hat

**Marker und Stände**

- Keine Umbenennung. Der Plan bleibt `_c_` mit Status Complete, der Spec bleibt `_o_`, die vier
  geschlossenen Defekte bleiben `_c_`, die vierzehn offenen bleiben `_o_`, die zwei Entscheide
  bleiben `_i_`.

**Vier neue Defekte, alle im Circle** (`issues/260814-1247_o_…`):

1. Die Abnahmeliste rechnet gegen 72 Kriterien; der Spec führt 75, mit 77 als zweiter
   vertretbarer Lesart. Mittel.
2. Sechzehn der 29 Bündelkriterien sind vom Abnahmelauf nie berührt worden, mit der Zuordnung
   Kriterium für Kriterium und dem Vergleich zur Runde 8. Mittel.
3. Die Streichung der Beobachtung 10 nimmt zwei Kriterien ihren einzigen Beleg am Bündel, und
   die `#[must_use]`-Begründung trägt weiter, als der Übersetzer hält. Mittel.
4. Der Plan nennt als Spec-Fassung die vom 260814-0925; es gibt eine vom 260814-1010. Niedrig.

**Angemerkt, ohne Eingriff in den Inhalt**

- Die zwei in Turn 3 geschlossenen Defekte tragen eine Bestätigungsnotiz mit Fundstellen.
- `260814-0912_o_` trägt die Notiz zur Zeilendrift.
- Spec und Plan tragen je einen zweiten Abgleichvermerk.

**Nicht angefasst:** die Abnahmeliste `history/260814-1100-abnahmeliste-notizzettel.md`. Sie ist
die Aufzeichnung eines Standes, und die Ortsregel dieses Projekts hält solche Dateien fest, wie
sie am Tag ihres Laufs standen. Die Berichtigung ihrer Zahlen steht in den zwei Datensätzen
oben.

---

**Status:** Complete
