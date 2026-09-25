# T5 — die Prüfung vor dem Blatt, und die Stufenfolge zieht in eine reine Funktion

**Datum:** 260817-1359
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Schritt 6
(Bündel B, dritter und letzter Schritt)
**Datensatz:** `issues/260817-1107_p_der-rumpf-der-schutzschwelle-traegt-keine-probe.md`
(Befund 2 der Durchsicht, mit dieser Aufgabe auf `_c_` gesetzt)
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C4
**Baumstand vorher:** `e2760cd`

---

## Zwei Vorgänge, eine Aufgabe

Der Auftrag ist zusammengelegt, und das ist die Festlegung des Datensatzes `260817-1107`:
Bündel B setzt mit der Papierkorbprüfung eine weitere Stufe in dieselbe Kette, und ein Umzug
der Stufenfolge nach Bündel B hätte dieselbe Stelle zweimal geändert. Der Umzug ist deshalb
**vor** dem Einbau der neuen Stufe gemacht und die Stufe dann als Zeile in eine Tafel
eingetragen, die schon dastand.

## Was umgesetzt ist

Drei Dateien.

### `crates/krk-ui/src/kommandos/loeschwarnung.rs` — die Regel und ihre Meldung

- **`pub enum Vorstufe`** mit vier Ausgängen: `VorgangLaeuft`, `NichtsAusgewaehlt`,
  `OhnePapierkorb`, `Rueckfrage`. Kein Wahrheitswert mit Beipackzettel: drei der vier
  Ausgänge halten den Befehl an und sagen dem Nutzer **verschiedene** Dinge.
- **`#[must_use] pub fn vor_der_rueckfrage(vorgang_laeuft: bool, auswahl_leer: bool,
  papierkorb: Befund) -> Vorstufe`** — die eine Reihenfolge der Stufen, als reine Funktion
  mit ausgeschriebener Tafel, in der Bauform von `kommandos::rueckschritt`. Fünf
  `match`-Zweige über zwölf Kombinationen (2 × 2 × 3), **ohne Auffangzweig**: der dritte
  Zweig prüft auf `Befund::Ja`, der vierte nennt `Befund::Nein | Befund::Unentschieden`
  ausgeschrieben, damit ein vierter Befund den Bau anhält statt still in den Sperrzweig zu
  fallen.
- **`#[must_use] pub fn ohne_papierkorb() -> &'static str`** — die Meldung der dritten Stufe:
  *„das Ziel führt keinen Papierkorb, es wurde nichts gelöscht; im Finder löschen"*. Drei
  Auskünfte in einer Zeile: Befund, Folge, Ausweg.
- Beide `#[must_use]` tragen ihren Grund ausgeschrieben; bei der Regel ist er, dass ein
  fallengelassener Rückgabewert bedeutet, keine der drei Prüfungen gefahren zu haben.
- Der Modulkopf ist auf den erweiterten Gegenstand gezogen — die eine Rückfrage, **ob sie
  erscheint** und **was in ihr steht** — und trägt vier neue Abschnitte: warum die Regel
  hier steht und nicht im Rumpf, der sie ausführt; warum sie in diesem Modul steht und nicht
  in einem neunten daneben; wie sich die drei Tatsachen zu den fünf Stufen der Kette
  verhalten; und warum `Befund::ist_warnwuerdig` in dieser Datei nicht vorkommt.

### `crates/krk-ui/src/appkit/anwendung.rs` — der Rumpf beschafft und führt aus

`loeschen_nach_rueckfrage` entscheidet nichts mehr. Es erhebt die drei Tatsachen, jede aus
genau einer Quelle, und verzweigt dann über die vier Ausgänge der Regel:

- `vorgang_laeuft` aus `self.ivars().vorgang.borrow().is_some()`,
- `auswahl.ist_leer()` aus `quelle.betroffene_eintraege()` (dieselbe Lesung, die auch der
  Auftrag trägt),
- der Papierkorbbefund aus `std::fs::canonicalize(&quellordner)` und
  `papierkorb::fuehrt_einen_papierkorb`, mit `map_or(Befund::Unentschieden, …)`: **ein nicht
  auflösbarer Ordnerpfad ist damit `Unentschieden` und löscht nicht.**

Die vier Zweige: `VorgangLaeuft` ruft `vorgang_laeuft_schon`, die eine Stelle, die diese
Meldung für alle drei Frager baut; `NichtsAusgewaehlt` behält seinen bisherigen Text;
`OhnePapierkorb` zeigt `loeschwarnung::ohne_papierkorb()`; `Rueckfrage` trägt das Blatt und
den Rückruf, Zeile für Zeile unverändert.

Der Doc-Kommentar ist nachgezogen: vier Stufen sind fünf geworden, das Flussbild trägt die
neue Frage, und ein Absatz sagt, dass die Reihenfolge seit dem 260817 nicht mehr hier steht,
sondern in `loeschwarnung::vor_der_rueckfrage` — wer sie ändern will, ändert die Tafel dort
und nicht die Zeilenfolge hier.

### `crates/krk-ui/src/appkit/papierkorb.rs` — die Erwartung ist gefallen

Entfernt sind die sieben Zeilen `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]`
(der Auftrag nennt sie als vier; es sind sieben, weil `rustfmt` das Attribut aufbricht).
Sonst ist an der Datei nichts angefasst — mit dem Aufrufer wäre die Erwartung unerfüllt
geworden und `-D warnings` hätte den Bau angehalten. Genau darauf war sie gestellt.

## Die Polarität, an der ein Fehler das Gegenteil bewirkt

Geprüft ist auf `Befund::Ja` selbst und **nicht** über `ist_warnwuerdig`. Bei der Frage nach
dem Papierkorb ist `Ja` die Erlaubnis, und `Unentschieden` gehört zu `Nein`; `ist_warnwuerdig`
fasst `Ja` und `Unentschieden` zusammen und machte hier aus „wir wissen nichts" die Erlaubnis
zu löschen. Der Name kommt in `loeschwarnung.rs` nicht vor, und der Modulkopf sagt, dass das
Absicht ist. Die Probe `ohne_papierkorb_erscheint_kein_blatt` hält beide Zeilen fest; ihre
zweite ist die eigentliche.

## Sieben Proben, und was sie einzeln zeigen

Alle in `kommandos::loeschwarnung::tests`, alle ohne Fenster und ohne Hauptfaden — das ist
der ganze Grund für den Umzug.

```
kommandos::loeschwarnung::tests::die_stufenregel_hat_genau_einen_aufrufer ... ok
kommandos::loeschwarnung::tests::die_tafel_aus_zwoelf_faellen_geht_auf ... ok
kommandos::loeschwarnung::tests::ein_laufender_vorgang_kommt_nicht_bis_zum_blatt ... ok
kommandos::loeschwarnung::tests::eine_leere_auswahl_kommt_nicht_bis_zum_blatt ... ok
kommandos::loeschwarnung::tests::ohne_papierkorb_erscheint_kein_blatt ... ok
kommandos::loeschwarnung::tests::genau_ein_fall_erreicht_das_blatt ... ok
kommandos::loeschwarnung::tests::die_meldung_ohne_papierkorb_nennt_befund_folge_und_ausweg ... ok
```

- **`die_tafel_aus_zwoelf_faellen_geht_auf`** schreibt alle zwölf Kombinationen einzeln aus,
  in der Form der Tafeln aus `rueckschritt.rs` und `verzeichnis/befund.rs`. Keine gerechnete
  Erwartung: die wäre die Umsetzung ein zweites Mal.
- **`ein_laufender_vorgang_kommt_nicht_bis_zum_blatt`** fährt alle sechs Kombinationen der
  beiden anderen Tatsachen durch, auch die, in der Auswahl und Papierkorb in Ordnung sind.
  Der **Vorrang** ist die Aussage, nicht der einzelne Ausgang.
- **`eine_leere_auswahl_kommt_nicht_bis_zum_blatt`** fährt alle drei Befunde durch: ohne
  Gegenstand ist die Frage nach dem Rückweg gleichgültig.
- **`genau_ein_fall_erreicht_das_blatt`** zählt und sagt damit etwas, das keine einzelne
  Zeile sagt: die Rückfrage ist der eine Ausgang mit drei bestandenen Prüfungen.
- **`die_stufenregel_hat_genau_einen_aufrufer`** ist die Aufruferzählung über
  `crate::quellbaum`, mit zusammengesetzter Nadel und ohne die eigene Datei. Ein zweiter
  Aufrufer wäre ein zweiter Löschweg mit eigener Reihenfolge seiner Prüfungen.
- **`die_meldung_ohne_papierkorb_nennt_befund_folge_und_ausweg`** prüft drei Bestandteile
  und nicht den ganzen Satz: geprüft ist, was die Zeile leisten muss, nicht ihre
  Zeichensetzung.

## Die vier Eigenschaften aus Befund 2, einzeln abgerechnet

| Eigenschaft | Stand |
|---|---|
| 1. der laufende Vorgang wird **vor** dem Blatt gemeldet | **geprüft**, `ein_laufender_vorgang_kommt_nicht_bis_zum_blatt` über alle sechs Kombinationen |
| 2. die leere Auswahl kommt nicht bis zum Blatt | **geprüft**, `eine_leere_auswahl_kommt_nicht_bis_zum_blatt` über alle drei Befunde |
| 3. ein Abbruch stellt keinen Auftrag | **ungeprüft, Nutzerarbeit** — hängt am Rückruf des Blattes |
| 4. der bestätigte Auftrag trägt die gezeigte Auswahl | **ungeprüft, Nutzerarbeit** — hängt an derselben Stelle |

Dazu neu geprüft, was Schritt 6 hinzufügt: bei `Befund::Nein` und bei `Befund::Unentschieden`
kein Blatt und kein Auftrag, und ein nicht auflösbarer Pfad zählt als `Unentschieden`.

**Für 3 und 4 ist keine Probe gebaut, und das ist eine Entscheidung und kein Versäumnis.**
Beide sind Aussagen über den Rückruf des Blattes: dass `bestaetigt == false` bei Esc,
Return und „Abbrechen" ankommt, und dass die `Cell` beim ersten Zugriff genau die Auswahl
herausgibt, die im Blatt stand. Ein Blatt lässt sich unter `libtest` nicht bedienen, und
`krk-ui` hat kein Bibliotheksziel. Eine reine Funktion, die `Option<(Art, Auswahl, PathBuf)>`
auf sich selbst abbildet, sobald ein Wahrheitswert gesetzt ist, wäre baubar und prüfbar — sie
maß dann aber ihre eigene Durchreiche und nicht die AppKit-Verdrahtung, an der das Risiko
hängt. **Sie gehören in den Abnahmelauf**, der KRK im Vordergrund verlangt und damit
Nutzerarbeit ist.

## Eine Verhaltensänderung, die der Auftrag nicht nennt

**Die Papierkorbprüfung trifft beide Löschbefehle**, also bis Bündel D auch das endgültige
Löschen auf `f8` und `opt+cmd+delete`, obwohl das keinen Papierkorb braucht. Das ist keine
Nachlässigkeit des gemeinsamen Rumpfes, sondern die Directive dieser Runde ohne
Einschränkung: *„Ein Ziel ohne Papierkorb wird nicht gelöscht, sondern gemeldet."* Ein Ziel
ohne Rückweg ist genau das Ziel, an dem ein endgültiges Löschen am wenigsten zurückzunehmen
wäre, und ein Zweig, der die Prüfung für den einen Befehl überspringt, wäre ein zweiter
Löschweg an der Stelle, an der diese Runde den zweiten abschafft. Die Begründung steht am
Doc-Kommentar von `loeschen_nach_rueckfrage`. Kein Datensatz nötig: die Alternative wäre ein
vierter Eingang der Regel, den weder Plan noch Spec verlangen.

## Zwei kleine Kosten, benannt und nicht weggerechnet

- **Der Papierkorbtest läuft auch dann, wenn eine frühere Stufe sperrt.** Der Rumpf erhebt
  alle drei Tatsachen, bevor die Regel eine Stufe nennt; ein laufender Vorgang kostet damit
  ein `canonicalize` und einen `NSFileManager`-Aufruf, die niemand braucht. Der Auftrag
  verlangt genau diesen Zuschnitt („der Rumpf beschafft danach nur noch die Tatsachen"), und
  eine Erhebung, die von der Antwort der Regel abhängt, wäre die Reihenfolge ein zweites Mal
  — an der Stelle, an der wir sie gerade herausgezogen haben. Keine der zehn Zusagen aus C8
  vermisst diese Spanne; der Modulkopf von `papierkorb.rs` hat das für die Prüfung selbst
  schon nachgesehen.
- **`vorgang_laeuft_schon` liest den Vorgang ein zweites Mal.** Der Zweig ruft es, weil es
  die eine Stelle ist, die diese Meldung baut — sie nennt die Art des laufenden Vorgangs.
  Beide Lesungen liegen im selben Durchgang der Ereignisschleife, also liefern sie dieselbe
  Antwort; das steht als Kommentar daneben.

## Abnahme

`make check` — **exit 0**, alle vier Kommandos grün: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`. 19 Probenläufe ohne einen Fehlschlag.

Zusätzlich `cargo doc -p krk-ui --no-deps --document-private-items`: 67 Warnungen, exakt so
viele wie vorher, **keine in einer der drei Dateien** (nachgeprüft mit `touch` auf beide
geänderten `.rs` und `grep` auf die Pfade in der Ausgabe). Die neuen Doc-Verweise auf
`Befund::Ja`, `Befund::oder`, `Vorstufe::*` und `loeschwarnung::vor_der_rueckfrage` lösen auf.

## Grenzen eingehalten, und eine Stelle bewusst nicht angefasst

Angefasst sind die drei Dateien der Auftragsgrenze und keine vierte. Nichts an `blaetter/`.
Kein Commit; das macht der Orchestrator. Der Planschritt bleibt unverändert; das `[DONE]`
setzt der Orchestrator.

**Nicht angefasst und der Meldung wert:** die Modulliste im Kopf von
`crates/krk-ui/src/kommandos/mod.rs` beschreibt `loeschwarnung` als *„Die Texte der einen
Rueckfrage vor dem Raeumen in den Papierkorb, und wie ein Ziel eingeordnet wird"*. Seit
dieser Aufgabe trägt das Modul zusätzlich die Stufenfolge, also ob die Rückfrage überhaupt
erscheint. Die Datei liegt außerhalb der Auftragsgrenze, und die Zeile ist eine
Beschreibung und keine Zusage — sie gehört in denselben Durchgang, der die Liste ohnehin
anfasst, oder in eine eigene kleine Aufgabe.
