# Codedurchsicht: Turn 1 der Notizzettel-Runde

**Date:** 2026-08-14
**Sender:** coderev
**Reviewed-range:** `6d05bef..dd2643e`
**Not-opened:** `fusion-workbench/circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/_t_circle.md`, `history/260813-2342-orchestrator-session.md`, `history/260813-2348-shaper-spec-notizzettel.md`, `history/260814-0628-shaper-spec-nachtrag.md`, `history/260814-0656-planner-plan-notizzettel.md`, `issues/260814-0628_o_diagrammbefunde-haben-keinen-eigentuemer-und-bleiben-deshalb-liegen.md`, `reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, `reviews/260814-0711-conceptrev-plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`, `fusion-workbench/orchestrator-events.jsonl`
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Gelesen gegen:** Spec `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` (Fassung 260814-0628), Plan `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, beide Entscheide in `decisions/`
**Eigener Lauf:** `make check` am 260814-0908 gefahren, Rückgabewert 0, „alle vier gruen". Kein `make bundle`, kein `cargo xtask`.

**Alle 24 geänderten Quelldateien unter `crates/` und `resources/` sind geöffnet.** Bei drei
großen Dateien — `krk-ui/src/appkit/anwendung.rs`, `krk-ui/src/appkit/editor.rs` und
`krk-core/tests/ablage.rs` — ist der vollständige Unterschied gelesen und der Rest gezielt
durchsucht, nicht Zeile für Zeile. Die drei neuen Dateien (`blaetter/zettel.rs`,
`zettelmodell.rs`, `textautomatik.rs`) sind ganz gelesen.

---

## Zusammenfassung

Der Bau setzt die fünf Fähigkeiten um, und die vier teuren Zusagen halten alle vier: die
Zulässigkeitsregel der achten Runde ist unangetastet, die Textfläche des Zettels ist in
`ersthelfer_gehoert_appkit` nicht angemeldet, der Schreibfokus geht nach jedem Tabklick
unbedingt in die Fläche zurück, und `Datei::ALLE`, `Format` sowie die Baumprobe mit ihren
fünf Dateien stehen wie zugesagt. Auch die sieben Planabweichungen tragen ihre Begründung;
die eine, an der es zu zählen gab — bleibt es bei **einem** Durchgang durch die Ablage beim
Beenden —, ist geprüft und stimmt.

**Ein Befund hält die Runde auf.** Die Zusage aus C4 „eine gescheiterte Sicherung wirft den
Stand nicht weg" gilt in `zettel_sichern` und wird eine Aufrufebene höher gebrochen: das
Neulesen beim Öffnen und beim Tabwechsel setzt den gehaltenen Stand des Zettels auf den
Dateiinhalt zurück und löscht damit genau den Text, den die gescheiterte Sicherung stehen
lassen sollte. Zwei Zusagen des Spec widersprechen sich an dieser Stelle, und der Bau hat
sie stillschweigend zugunsten der zweiten aufgelöst.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 3 |
| Niedrig | 5 |

Jeder Befund liegt als eigener Datensatz unter `issues/`; die Tabelle unten nennt die
Dateinamen.

| # | Schwere | Befund | Datensatz |
|---|---|---|---|
| 1 | hoch | Neuöffnen oder Tabwechsel nach gescheiterter Sicherung wirft den ungesicherten Zettelstand weg | `260814-0908_o_ein-neuoeffnen-nach-gescheiterter-sicherung-…` |
| 2 | mittel | Je Sicherungsmoment höchstens ein Zettel; beim Beenden gibt es kein nächstes Mal | `260814-0909_o_je-sicherungsmoment-wird-hoechstens-ein-zettel-…` |
| 3 | mittel | Eine Zetteldatei über `EDITORGRENZE` wird unbegrenzt auf dem Hauptfaden kopiert | `260814-0910_o_eine-zetteldatei-ueber-editorgrenze-wird-…` |
| 4 | mittel | Acht Verweise in `editor.rs` schicken zu `textflaeche_bauen`, wo die Automatikzeilen nicht mehr stehen | `260814-0911_o_acht-verweise-in-editor-rs-…` |
| 5 | niedrig | Neun Stellen sagen „vier Dateien"; es sind sechs | `260814-0912_o_neun-stellen-sprechen-weiter-von-vier-…` |
| 6 | niedrig | `Grund::einzelheit` nennt „die vier übrigen Gründe"; es sind drei | `260814-0913_o_grund-einzelheit-nennt-vier-uebrige-…` |
| 7 | niedrig | Das Feld `schalter` begründet sich mit einer Rückstellung, die nicht gebaut ist | `260814-0914_o_das-feld-schalter-des-zettelwaechters-…` |
| 8 | niedrig | Vierzehn Leerzeichen mitten in einer Probenmeldung | `260814-0915_o_eine-probenmeldung-in-operationen-rs-…` |
| 9 | niedrig | `NSSegmentedControl` steht seit 10.3, die Angabe sagt 10.0 | `260814-0916_o_nssegmentedcontrol-steht-seit-macos-10-3-…` |

---

## Befunde nach Thema

### Die Sicherung des Zettels: drei Befunde mit einem gemeinsamen Auslöser

**Befund 1 (hoch), Befund 2 (mittel) und die fehlende Meldung beim Beenden hängen
zusammen.** Alle drei werden von derselben Lage ausgelöst — einer Sicherung, die nicht
geschrieben hat — und sie verstärken einander:

```
Sicherung scheitert   ──> Modell hält den Zettel weiter als abweichend   (richtig)
        │
        ├─> naechstes Oeffnen / Tabwechsel: oeffnen(zettel, gelesen)
        │        setzt gehalten = gelesen                                (Befund 1)
        │
        ├─> beide Zettel abweichend: zu_sichern liefert nur einen        (Befund 2)
        │
        └─> beim Beenden: let _ = zettel_sichern(zugang), keine Meldung
```

`zettel_sichern` (`crates/krk-ui/src/appkit/anwendung.rs:3440`) ist richtig gebaut und
schreibt seine Zusage selbst aus. Gebrochen wird sie von den zwei Aufrufern, die danach
`Zettelmodell::oeffnen` rufen (`:3276` und `:3358`). Wer nur einen der drei Wege behebt,
lässt die anderen offen; der Datensatz zu Befund 2 verweist deshalb ausdrücklich auf den zu
Befund 1.

**Der Widerspruch liegt im Spec und nicht erst im Bau.** C4 sagt beides zu: „Eine
gescheiterte Sicherung … wirft den Stand nicht weg" und „Der Zettel liest seine Datei bei
jedem Öffnen neu". Beide gelten nur gemeinsam, wenn das Neulesen einen abweichenden Stand
nicht antastet. Welcher der beiden Stände gewinnt, ist eine Frage an den Nutzer und keine,
die eine Behebung nebenbei entscheiden sollte.

### Die Grenze `EDITORGRENZE` bindet das Laden und nicht das Beiseitelegen

Befund 3. Der Spec sagt im Abschnitt zu den zehn Zeitzusagen zu, die obere Schranke für die
Arbeit auf dem Hauptfaden sei `EDITORGRENZE` mit 16 MB. Für das Laden hält das. Der Zweig
`Textstand::Unlesbar` in `Zugang::text_laden` reicht dagegen den offenen Deskriptor an
`beiseite_legen`, und `io::copy` kopiert die ganze Datei — bei jedem `f2`, synchron, unter
dem gehaltenen Schreibgriff. Der Editor kopiert an derselben Stelle nichts; das Verhalten
ist mit dieser Runde neu.

### Doppelte Wahrheiten, die der Umzug hinterlassen hat

Befunde 4, 5 und 6 sind alle vom selben Typ: eine Zahl oder ein Verweis, die vor dem Umzug
stimmten. Zwei davon sind bloße Zählungen. Der dritte, Befund 4, ist eine
**Handlungsanweisung**: „Wer eine zehnte Einstellung als `Abgeschaltet` einträgt, ohne die
Zeile in `textflaeche_bauen` zu schreiben, bekommt hier den Fehlschlag." Wer ihr folgt, gibt
die Einstellung dem Editor und nicht dem Zettel — also genau die zwei Wahrheiten, die
Schritt 9 beseitigt hat. Das hebt ihn über die anderen beiden.

### Kleinigkeiten am neuen Code

Befunde 7, 8 und 9. Ein Feld, dessen Begründung eine Rückstellung beschreibt, die nicht
gebaut ist; eine zerrissene Zeichenkette in einer Probenmeldung; eine Untergrenzenangabe,
die für eine von neun Klassen 10.0 statt 10.3 sagt. Keiner der drei berührt Verhalten.

---

## Die sieben Planabweichungen, einzeln geprüft

| # | Abweichung | Trägt die Begründung? |
|---|---|---|
| 1 | `Textstand::KeinGueltigesZiel` trägt `fehlt: bool` | **Ja.** Ohne das Feld ist „die Datei gibt es nicht" aus einer Zeichenkette nicht verlässlich zu erkennen, und das Kriterium „fehlt eine Zetteldatei, ist der Zettel leer, und KRK meldet keinen Fehler" wäre nicht einlösbar. Ein fünfter Wert daneben hätte den Editor zu einer Unterscheidung gezwungen, die er nicht trifft — `oeffnen` wirft `fehlt` folgerichtig mit `..` weg. |
| 2 | `Grund::einzelheit()` gibt `Cow<'_, str>` | **Ja.** `ZuGross` trägt eine Zahl und keinen Satz; entstünde der Satz beim Erzeugen, stünde `EDITORGRENZE` ein zweites Mal im Baum. Die drei übrigen Gründe kosten weiterhin keine Kopie. Die Zählung im Doc-Kommentar ist dabei falsch, siehe Befund 6. |
| 3 | Gescheitertes `rewind` liefert `KeinGueltigesZiel` statt `Unlesbar` | **Ja, mit einer Anmerkung.** Ein Deskriptor an unbekannter Stelle ergäbe eine abgeschnittene Sicherung, die aussieht wie eine vollständige — das ist schlimmer als eine Meldung. Die Anmerkung: der Plan sagt zu, `oeffnen` behalte „Signatur und Rückgabewerte Zeichen für Zeichen", und streng genommen kommt ein Weg hinzu, auf dem eine zu große Datei als `KeinGueltigesZiel` statt als `ZuGross` abgewiesen wird. Auf einer gewöhnlichen Datei ist er nicht zu erreichen, und weiter kommt keine andere. Kein Befund. |
| 4 | Strang A hat `krk-ui/src/belegungsausgabe.rs` angefasst | **Ja.** Die Datei steht im Dateienverzeichnis von Schritt 2 des Plans, sie ist die fünfte Aufrufstelle von `atomar::schreiben`, und ohne sie übersetzt der Arbeitsbereich nicht. Die Abweichung war vom Auftrag und nicht vom Plan; die Änderung sind zwei Zeilen, und sie kollidiert mit keinem Schritt von Strang B. |
| 5 | Der Tabklick sicherte schon in Strang C | **Ja, und die Spur ist wieder weg.** `zettel_zurueckschreiben` kommt im Baum nicht mehr vor; `zettel_sichern` hat den Vorgänger ersetzt und steht nicht daneben. Die Zählprobe `das_sichern_des_zettels_ist_genau_einmal_erklaert` hält es. |
| 6 | `applicationWillTerminate:` kehrt nicht mehr früh zurück | **Ja, und es ist bei einem Durchgang geblieben.** Nachgezählt: in `wird_beendet` (`anwendung.rs:842`) steht genau ein `unter_der_sperre`; die frühe Rückkehr ist zu einem `if let Some(schreiber)` **innerhalb** dieses einen Durchgangs geworden, der Kommentar zum Defekt `260813-0540` steht unverändert darüber. Der `RefMut` auf `sitzungsschreiber` wird jetzt über den Durchgang gehalten, und nichts im Rumpf leiht ihn ein zweites Mal. Ohne die Änderung hätte die zweite laufende Instanz von KRK ihren Zettel beim Beenden nie geschrieben — C4 nimmt für zwei Instanzen den Preis „die zuletzt schließende gewinnt" in Kauf und nicht „die zweite schreibt nie". |
| 7 | Das Lesen der Textfläche steht als eigene Stelle | **Ja.** Beim Tabklick muss die Übernahme vor `Zettelmodell::wechseln` laufen, sonst ginge der Stand der Fläche in den falschen der beiden Zettel; `zettel_sichern` weiß nicht, ob eben gewechselt wurde, und eine Fallunterscheidung darüber wäre der Zweig, den der Plan nicht will. Die Reihenfolge ist im Code so gebaut. |

## Die vier teuren Zusagen, einzeln nachgezählt

**`immer_erreichbar`, `waehrend_blatt_erlaubt` und `zulaessigkeit::zulaessig` sind
unangetastet.** Am Unterschied nachgesehen: in `kommandos/zulaessigkeit.rs` und
`kommandos/operationen.rs` ist außerhalb von `mod tests` keine Zeile geändert. Die drei
Proben aus Strang B halten es in der stärkeren Form, die zählt statt aufzuzählen: beide
laufen über `Kommando::KENNUNGEN` und zählen, welche Kommandos die Funktion selbst bejaht.
`die_ausnahmeliste_fuehrt_dieselben_drei_befehle_wie_vor_dieser_runde` prüft Länge **und**
Mitgliedschaft, und erst beides zusammen sagt „genau diese drei".

**Die Textfläche des Zettels ist in `ersthelfer_gehoert_appkit` nicht angemeldet.**
`appkit/ereignisse.rs` ist in diesem Bereich überhaupt nicht angefasst, und keine Stelle
meldet die Fläche an. Die Kette dahinter ist geprüft: steht das Blatt, ist das
Schlüsselfenster das des Blattes, sein Ersthelfer ist die `NSTextView` des Zettels,
`isKindOfClass(NSTextView)` trifft, die Lage meldet „gehört AppKit", `Abbrechen` ist damit
unzulässig, und der Tastendruck läuft unverändert weiter. Der Modulkopf von `zettel.rs`
schreibt es aus und nennt die entgegenlautende Warnung in `CLAUDE.md`.

**Der Schreibfokus geht nach jedem Tabklick zurück, und zwar unbedingt.**
`Zettelwaechter::tab_gewechselt` ruft `makeFirstResponder(Some(&flaeche))` außerhalb jedes
Zweigs — auch dann, wenn der Klick dem bereits offenen Tab galt und der Text stehen bleibt.
Damit trägt die Zusage in beiden Ausgängen der ungemessenen Frage, ob ein Klick auf den
Tabschalter den Rang überhaupt nimmt.

**`Datei::ALLE` und `Format` sind vollständig ohne Auffangzweig**, `Datei::ALLE` ist
`[Datei; 6]`, `Zettel(Zettel)` trägt die Wahl als Nutzlast statt zweier Varianten
nebeneinander, und keine Stelle im Programm läuft über `Datei::ALLE` — die sieben
Fundstellen liegen alle in `tests/ablage.rs`, vier davon jetzt hinter dem Filter
`format() == Format::Toml`. `nur_benannte_dateien_erreichen_das_atomare_schreiben` bleibt
bei denselben fünf Dateien; geändert ist allein der Kommentar („vier Schreiber hinter einem
`Zugang`" statt drei). `ueber_der_ablage_stehen_genau_zwei_absprachen` ist nicht angefasst.

## Was daneben geprüft und in Ordnung ist

- **Die Eingabetaste setzt eine Zeile und schließt das Blatt nicht.** Die eine
  Schaltfläche des Blattes trägt `Taste::Escape`, und `Blatt::mit_schaltflaechen` setzt
  jeder Schaltfläche ihre Taste ausdrücklich, überschreibt also die Eingabetaste, die
  `NSAlert` der ersten von sich aus gibt. Auf die Reihenfolge zwischen
  `performKeyEquivalent:` und der Textfläche kommt es damit nicht an — die Frage bleibt
  ungemessen, aber sie entscheidet nichts.
- **`Esc` schließt in beiden Fällen dasselbe.** Ob die Escape-Taste über den Wächter
  (`cancelOperation:`) oder über die Tastenentsprechung der Schaltfläche ankommt, beides
  mündet in denselben Abschlussblock und damit in `zettel_blatt_geschlossen`. Es gibt
  keinen zweiten Schließweg.
- **Kein Verweisring.** Die Fläche hält ihren Delegierten schwach, der Tabschalter sein
  Ziel schwach, und die starke Richtung läuft vom Wächter zu beiden; der Abschlussblock
  hält den Wächter, und AppKit gibt den Block nach dem Rückruf frei. Der Wächter gibt beim
  Schließen über `Esc` seinen Schließweg mit `take()` heraus.
- **Die Textfläche wird beim Schließen abgeräumt.** `zettel_blatt_geschlossen` nimmt den
  Stand ab, leert dann `zettelflaeche` und `offenes_blatt` und sichert erst danach. Ein
  stehen gebliebener Griff auf eine Fläche eines nicht mehr stehenden Blattes entsteht
  nicht, und `zettelstand()` liefert dann `None` statt einer leeren Zeichenkette — was
  richtig ist, denn eine leere Zeichenkette hieße „der Nutzer hat alles gelöscht".
- **Keine Schachtelung von Durchgängen.** Die drei Momente mit eigenem Durchgang geben die
  Sperre vor dem nächsten Schritt wieder ab; in `fenster_schliessen` steht `performClose`
  hinter dem abgeschlossenen `unter_der_sperre`, sodass ein synchron ausgelöster
  Abschlussblock keinen zweiten Durchgang in einen ersten legt.
- **Die Sitzung trägt die Merkung und nie den Text**, das Feld steht vor den drei Tabellen,
  `Sitzung` trägt weiterhin `#[serde(default)]`, und eine `session.toml` aus der Zeit davor
  ergibt den ersten Zettel. Beide Hälften stehen als Proben in `tests/ablage.rs`.
- **Die Untergrenzenangabe deckt jetzt 38 von 40 Dateien** unter `crates/krk-ui/src/appkit/`;
  ohne sie sind wie bisher nur `koordinaten.rs` und `mod.rs`. Beide neuen Dateien tragen den
  Abschnitt. Die zwei Angaben mit Zahl in `zettel.rs` sind am SDK nachgeprüft und richtig;
  eine Klasse in der 10.0-Liste ist es nicht (Befund 9).
- **`f2` und `cmd+k` waren frei.** Über alle Tastenlisten von
  `resources/default-keymap.toml` nachgezählt; belegt ist allein `shift+cmd+k`. Die
  Kopfzahlen stehen auf 83 Funktionen und 90 Kombinationen, und die Probe dazu läuft grün.
  Dass `f2` auf einem unveränderten Mac die Helligkeit erhöht, ist dieselbe Lage wie bei
  `f1` für die Belegungsansicht; der Eintrag für `f1` schreibt sie aus, der neue nicht.
- **Der Entscheid zu den Textautomatiken ist umgesetzt.** Möglichkeit 2 steht als
  `jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab` in
  `appkit/textautomatik.rs`, mit dem blinden Fleck im Doc-Kommentar und nicht nur im
  Sitzungsprotokoll. Der Datensatz
  `decisions/260814-0656_a_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md` trägt
  dafür noch den Marker „beantwortet"; nach der Konvention gehört er auf „umgesetzt", mit
  einem `Implemented:`-Verweis auf `bfea397`. Das ist Arbeit für den Reconciler und kein
  Defekt am Code.

## Was diese Durchsicht nicht leistet

- **Die zweite Kriterienliste jeder Fähigkeit** verlangt KRK im Vordergrund und ist
  Nutzerarbeit. Nichts davon ist hier geprüft.
- **Der Messlauf zu `performClose:` an einem Fenster mit anhängendem Blatt** steht im Plan
  unter „Nutzerarbeit" und ist nicht gefahren. Der Code sagt die Kante nicht an, und die
  Reihenfolgeprobe ist gegengeprüft; die Zusage hält in beiden Ausgängen.
- **Ob AppKit `textView:doCommandBySelector:` beim Wächter des Zettels überhaupt ruft**, ist
  in diesem Baum ungemessen. Die Probe fährt die reine Funktion `uebernimmt` an einem `Sel`
  und sagt darüber nichts.

## Empfohlene Reihenfolge

**Vor dem Abschluss der Runde:** Befund 1. Er bricht ein ausgeschriebenes Abnahmekriterium
von C4 und verliert dabei Text ohne Meldung. Die Behebung braucht vorher eine Antwort des
Nutzers darauf, welcher Stand beim Öffnen gewinnt, wenn der Zettel abweicht — das ist der
Widerspruch zwischen zwei Zusagen desselben Kriterienblocks und keine Bausache.

**Mit Befund 1 zusammen, weil derselbe Auslöser:** Befund 2. Getrennt behoben lässt jede
der beiden Behebungen den anderen Weg offen.

**Vor dem Abschluss, aber ohne Nutzerfrage:** Befund 4. Eine Handlungsanweisung, die den
nächsten Bauer in die Lage zurückführt, die diese Runde beseitigt hat.

**Danach:** Befund 3 braucht eine Festlegung, wie groß „beiseite" werden darf, und die
gehört in den Spec oder in einen Entscheid; er hält die Runde nicht auf, weil er einen
Fall betrifft, den der Nutzer selbst herstellen muss.

**Aufräumen, in einem Zug:** Befunde 5 bis 9.
