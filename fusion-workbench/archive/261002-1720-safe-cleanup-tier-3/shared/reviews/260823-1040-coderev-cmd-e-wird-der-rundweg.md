# Durchsicht: `28cbb7b` — `cmd+e` wird der Rundweg zwischen Dateiliste und Editor

**Reviewed-range:** `a8be186..28cbb7b`
**Not-opened:** none

**Getragen aus der vorigen Durchsicht:** none. `260823-0735-coderev-einblenden-erreicht-den-schirm.md`
führt `**Not-opened:** none`; es war nichts nachzuholen.

**Durchgesehen von:** coderev, 260823-1040
**Bereich als beauftragt:** ein Commit, elf Dateien, davon eine neu
(`crates/krk-ui/src/kommandos/rundweg.rs`) und eine Workbench-Datei.
**Am Baum gefahren:** `cargo clippy --workspace --all-targets -- -D warnings` (sauber),
`cargo test --workspace` (alle Ziele grün, 0 Fehlschläge).

## Summary

Die Umsetzung hält, und sie hält die Auflage des Nutzers wörtlich: die Fallunterscheidung steht
als reine Funktion mit genau einem Rufer in `kommandos/rundweg.rs`, alle drei Zweige rufen
bestehende Rümpfe, `editor_schliessen` ist herausgezogen statt abgeschrieben, und das Feld
`vorschau_danach` reist richtig bis hinter die C4-Nachfrage. Der Eingriff in die Aufzählung
`Wirkungsbereich` ist sauber durchgezogen; ich habe keinen Träger gefunden, der unter
`Dateibereiche` etwas anderes erlebt als vorher unter `Vorschau`. Was bleibt, ist ein Befund am
Rand des Commits, der Nutzer außerhalb dieses Geräts trifft, ein Griff, der eine schriftliche
Zusage dieses Baums verletzt, eine ungeprüfte Naht und drei Sorten Prosaschuld — darunter
dieselbe Gestalt, die die vorige Durchsicht schon einmal gefunden hat.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 4 |
| Low | 2 |

Kein Befund ist ein Hindernis für die Abnahme von Hand. Der High-Befund ist ein Hindernis für
die **Auslieferung**.

## Die vier Fragen der Beauftragung, beantwortet

### 1. `Wirkungsbereich::Vorschau` → `Dateibereiche`: die Folgen, die eine Zählung nicht sieht

**Sauber.** Der Wert trug genau einen Befehl, und der ist derselbe geblieben.

- Alle sieben Werte sind belegt: `crates/krk-core/src/tasten/belegung.rs:323-332` (Beschriftung),
  `crates/krk-ui/src/kommandos/fokus.rs:345-360` (`wirkt`), `crates/krk-core/tests/belegung.rs:
  1868-1900` (Beschriftungstafel und `stelle_in_den_sieben`),
  `crates/krk-ui/src/kommandos/zulaessigkeit.rs:309-316` (Stellvertretertafel). Die Aufzählung
  bleibt bei sieben, `CLAUDE.md` Zeile 78 bleibt wahr.
- Die Fokuszeile ist positiv aufgezählt und nicht als Verneinung der Leiste
  (`fokus.rs:350-353`). Das ist richtig und ausdrücklich begründet: `fokus != Fokus::Leiste`
  ließe `Fokus::Anderswo` durch, und `cmd+e` schlösse den Editor vor einer stehenden Rückfrage.
- Die Tafel in `zulaessigkeit.rs:402-408` ist mit `[true, false, true, true, false]` richtig
  nachgezogen; die Spaltenordnung stimmt gegen `Fokus::ALLE` (Dateifenster, Leiste, Vorschau,
  Editor, Anderswo), abgeglichen an der Tafel in `fokus.rs:400-410`.

**Die Ausgrauung im Hauptmenü** geht denselben Weg wie vorher und braucht nichts Eigenes:
`eintrag_pruefen` (`crates/krk-ui/src/appkit/anwendung.rs:820-834`) ruft
`zulaessigkeit::zulaessig(kommando, self.lage())` — dieselbe Funktion auf derselben `Lage` wie
der Ereignisabgriff. Der Eintrag ist damit in drei Bereichen bedienbar statt in einem, und die
zwei Frager können nicht auseinanderlaufen. Der Menüname kommt aus `resources/default-keymap.toml`
und ist mitgezogen: „In den Editor und zurück".

Eine Beobachtung ohne Befundcharakter: mit dem Fokus im Editor stehen jetzt zwei Einträge des
Abschnitts „Editor" schwarz, die beide den Editor schließen — „Editor schließen" (`opt+cmd+e`)
und „In den Editor und zurück" (`cmd+e`, holt zusätzlich die Vorschau). Das ist die Folge des
Nutzerentscheids und kein Fehler; wer es später stört, greift die Beschriftung an und nicht den
Code.

### 2. `vorschau_danach` auf allen Wegen

**Auf allen bestehenden Wegen richtig gesetzt, und „Abbrechen" trifft beide Hälften.**

- Es gibt nur **einen** alten Weg, nicht drei. `Anlass` trägt genau drei Werte
  (`anwendung.rs:395-425`): `EditorSchliessen`, `AndereDatei`, `Beenden`. Die
  Sitzungswiederherstellung und das Beenden benutzen `EditorSchliessen` **nicht** — `grep -rn
  "EditorSchliessen" crates` liefert keine weitere Aufrufstelle. Zu prüfen war also allein
  `opt+cmd+e`, und der übergibt `false` (`anwendung.rs:3158`).
- **„Abbrechen" trifft beide Hälften.** `anlass_unterbleibt` beantwortet
  `Anlass::EditorSchliessen { .. }` mit einem leeren Rumpf (`anwendung.rs:6922-6924`) und liest
  das Feld nicht. Der Editor bleibt stehen, die Vorschau bleibt draußen. Das ist genau das, was
  der Doc-Kommentar am Feld verspricht.
- **Die Reihenfolge in `anlass_ausfuehren` ist richtig geprüft und stimmt.** Der Kommentar
  behauptet, `editor_ausblenden` setze über `nach_dem_sichtbarkeitswechsel` den Fokus in die
  Dateiliste und `bereich_einblenden` lasse ihn dort. Nachgelesen: `nach_dem_sichtbarkeitswechsel`
  (`anwendung.rs:4249-4285`) ruft `fokus_setzen(Fokus::Dateifenster)` genau dann, wenn ein
  Bereich ohne `seite()` unsichtbar wird — der Editor. Beim Einblenden der Vorschau greift der
  Zweig nicht, es läuft allein `vorschau_nachtragen()`. Die Zeile steht richtig.
- **Der stumme `let _ =` ist konsistent.** `bereich_einblenden` trägt ein `#[must_use]`, und die
  zweite Stelle im Baum, die die Vorschau hervorholt (`anwendung.rs:1600`), schweigt genauso.
- **Die Sitzung zieht nach:** `anlass_ausfuehren` endet auf `aufteilung_nachziehen()` und
  `sitzung_vormerken()` (`anwendung.rs:6905-6907`), die neue Sichtbarkeit der Vorschau landet
  also in der `session.toml`.

Zwei Befunde bleiben: **keine Probe hält irgendeine der drei Zusagen** (Medium, Datensatz
`260823-1034`), und **der Rückweg blendet die Vorschau auch dann ein, wenn der Hinweg sie nicht
verdrängt hat** (Low, Datensatz `260823-1035` — eine Frage an den Nutzer, keine Verletzung des
Entscheids).

### 3. Die Umbenennung `editor_aus_vorschau` → `editor_rundweg`

**Die Abweisung trifft wirklich die ganze Datei, und die Folgen reichen über dieses Gerät
hinaus.** Das ist der einzige High-Befund dieser Durchsicht (Datensatz `260823-1030`).

Nachgelesen: `Belegung::bauen` (`crates/krk-core/src/tasten/belegung.rs:1420-1424`) beendet
sich beim **ersten** unbekannten Bezeichner mit `return Err`; `laden`
(`belegung.rs:1493-1513`) beantwortet jeden Fehler mit der Auslieferungsbelegung und einer
Meldung. Die Datei bleibt auf der Platte stehen — bis der Nutzer die nächste Taste zuweist:
`sichern` schreibt dann den Rückfallstand darüber, und `Belegungsdatei::from` trägt **jede**
Funktion mit (`belegung.rs:1651-1677`), weshalb jede von KRK geschriebene `keymap.toml`
zwangsläufig `editor_aus_vorschau` führt.

Auf dem Referenzgerät trifft das niemanden; ich habe es selbst nachgesehen
(`ls ~/Library/Application Support/KRK/`: keine `keymap.toml`). Seit der Runde 15 gibt es eine
öffentliche Releaseseite, und dort trifft es jeden, der jemals eine Taste umbelegt hat.

Der Kommentar an der Stelle, an der ein Nutzer nachschlägt
(`resources/default-keymap.toml:825-829`), sagt „wird beim Start als unbekannte Funktion
abgewiesen" — das liest sich, als träfe es den Eintrag. Die Commit-Nachricht schreibt es richtig
aus. Die Belegungsdatei sollte es auch.

Das ist die stärkere Gestalt des offenen Datensatzes
`shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`:
dort kommt eine Funktion tot an, hier kommt die ganze Datei nicht an. Keiner der drei Wege, die
jener Datensatz vorschlägt, löst diesen Fall.

### 4. Die elf Prosastellen, und die zwölfte

**Die elf sind richtig.** Ich habe jede gegen den Baum gelesen; keine behauptet etwas, was der
Code nicht tut. Die Spannung, die der Entscheidungsdatensatz benannt haben wollte, steht
ausgeschrieben in `resources/default-keymap.toml:865-871` und ist sachlich richtig aufgelöst.

**Die zwölfte gibt es, und sie steht zu dritt.** Alle drei in Dateien, die dieser Commit
angefasst hat:

1. `crates/krk-ui/src/kommandos/mod.rs:45` — „vor den **sieben** uebrigen Tastenbefehlsmodulen".
   Es sind acht. `crates/krk-ui/src/kommandos/mod.rs:54` — „vor den **sechs** uebrigen". Es sind
   sieben. Der Commit hat „Neun Module" auf „Zehn Module" gezogen und die zwei Zahlen stehen
   lassen, die von derselben Menge abhängen. Datensatz `260823-1032`.
2. `crates/krk-ui/src/appkit/anwendung.rs:5672` — „[`Self::fokus`] bleibt fuer die **fuenf**
   uebrigen Aufrufer stehen". Vor dem Commit stimmte die Zahl (fünf Aufrufe in `a8be186`), jetzt
   sind es sechs. Sie wird von selbst wieder richtig, wenn der Griff aus Befund 2 unten
   verschwindet; deshalb steht sie dort als Nebenbefund und nicht als eigener Datensatz.

**Die Vermutung des `coder` zur Probe stimmt** und betrifft zwei Stellen, nicht eine:
`die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung`
(`crates/krk-core/tests/belegung.rs:1968`) und `jede_neue_kennung_der_editor_runde_ist_umbelegbar`
(`crates/krk-ui/src/belegungsmodell.rs:1446`) schreiben `editor_rundweg` der Editor-Runde zu, die
den Eintrag unter anderem Namen angelegt hat. Datensatz `260823-1036`, Low.

## Befunde nach Themen

### Ausgelieferter Bestand

**High — Die umbenannte Kennung weist jede bestehende `keymap.toml` vollständig ab.**
`crates/krk-core/src/tasten/belegung.rs:1420-1424`, `:1493-1513`, `:1651-1677`.
Datensatz: `shared/issues/260823-1030_o_…`. Vier Möglichkeiten stehen dort, eine davon ohne
Codeänderung. Die Wahl gehört dem Nutzer.

### Die eine Erhebung des Fokus

**Medium — Der Rundweg erhebt den Fokus ein zweites Mal.**
`crates/krk-ui/src/appkit/anwendung.rs:7026`: `rundweg(self.fokus())`, während acht Zeilen
über dem rufenden Zweig `let fokus = lage.fokus;` mit dem Kommentar steht „Ein zweites
`self.fokus()` waere eine zweite Erhebung desselben Augenblicks" (`anwendung.rs:2994-2997`).
Drei Geschwister im selben `match` reichen den Wert durch (`tab_schliessen(fokus)`,
`teilen(fokus)`, `bereichskommando(fokus, …)`); der Rundweg ist der einzige, der ihn neu holt.
`inference:` heute geben beide Erhebungen dieselbe Antwort — zwischen ihnen laufen nur
`befehlsantwort_beidseitig_loeschen` und `bildschirmbreiten_uebernehmen`, und keine der beiden
setzt einen Ersthelfer (nachgelesen: `anwendung.rs:4696-4700` und `:4541-4549`). Die Zusage ist
trotzdem gebrochen, und zwar im Zweig, der sie am nötigsten hat: liefen die beiden auseinander,
öffnete `cmd+e` eine Datei, wo es den Editor schließen sollte.
Datensatz: `shared/issues/260823-1031_o_…`. Fix: ein Argument.

### Was ein `false` bedeutet

**Medium — Drei Stellen behaupten, ein `false` gebe den Tastendruck weiter.**
`kommando_ausfuehren` liefert seit der Runde 7 immer `true` (`anwendung.rs:3228-3233`, dazu der
eigene Doc-Kommentar `:2972-2982`). Die falsche Auskunft stand in
`editor_oeffnen_lassen` (`anwendung.rs:6256-6258`) und ist mit diesem Commit an zwei neue
Stellen gewandert: `crates/krk-ui/src/kommandos/rundweg.rs:117-119` und
`anwendung.rs:7020-7024`. Fünfundvierzig Zeilen unter der alten Stelle steht die richtige
Auskunft (`anwendung.rs:6303-6305`, „`true` verbraucht den Tastendruck"). Kein Verhalten ist
betroffen — der `None`-Zweig ist heute unerreichbar, und die Probe
`der_wirkungsbereich_und_die_regel_lassen_dieselben_bereiche_durch` hält das fest.
Datensatz: `shared/issues/260823-1033_o_…`.

### Was keine Probe hält

**Medium — `vorschau_danach` hält keine Probe, auf keinem seiner drei Wege.**
`grep -rn vorschau_danach crates` liefert sechs Treffer, alle in `anwendung.rs`, keinen in einem
Prüfmodul. Der Übersetzer hält weder das `false` an `opt+cmd+e` noch das `true` am Rückweg noch
das Nichtlesen in `anlass_unterbleibt`. Dieser Baum hat für genau diese Lage ein Werkzeug: die
Quelltextproben (`sichtbarkeitsproben`, `fokusnachzugproben`, `crate::quellbaum`).
Datensatz: `shared/issues/260823-1034_o_…`.

### Prosaschuld

**Medium — Zwei Zahlen im Modulkopf der Kommandos.**
`crates/krk-ui/src/kommandos/mod.rs:45` und `:54`.
Datensatz: `shared/issues/260823-1032_o_…`.

**Low — Zwei Proben schreiben `editor_rundweg` der Editor-Runde zu.**
`crates/krk-core/tests/belegung.rs:1968`, `crates/krk-ui/src/belegungsmodell.rs:1446`.
Datensatz: `shared/issues/260823-1036_o_…`.

### Verhalten, das der Nutzer entscheiden muss

**Low — Der Rückweg blendet die Vorschau auch dann ein, wenn der Hinweg sie nicht verdrängt hat.**
`anwendung.rs:6862-6880`. Der Code hält den Entscheid vom 260823-0942 buchstabengetreu; die
Frage ist, ob der Fall mitgedacht war.
Datensatz: `shared/issues/260823-1035_o_…`.

## Was gut ist, und warum es hier steht

Drei Sachen tragen über diesen Commit hinaus und sollten beim nächsten Mal die Vorlage sein.

**`rundweg.rs` hält die Form der Vorlage, und an einer Stelle geht es darüber hinaus.** Reine
Funktion, keine Zeile AppKit, ausgeschriebene Tafel über alle fünf Fokuswerte ohne Auffangzweig,
Aufruferzählung nach dem Vorbild von `rueckschritt`. Die Zählprobe hält wirklich, was sie sagt:
`aufrufstellen` (`crates/krk-ui/src/quellbaum.rs:131-149`) schließt Treffer aus, deren
Vorzeichen zu einem Bezeichner gehört, `self.editor_rundweg()` zählt also nicht mit — geprüft,
nicht angenommen.

**`der_wirkungsbereich_und_die_regel_lassen_dieselben_bereiche_durch`** ist die beste Probe des
Commits. Sie hält die zwei Regeln aneinander, die zusammen entscheiden, was `cmd+e` tut, und
macht damit eine spätere Weitung des Wirkungsbereichs auffällig, ohne dass jemand daran denken
muss. Genau die Gestalt, die als Defekt gemeldet war — ein Befehl, der durchkommt und nichts
findet —, kann so nicht wiederkehren.

**`editor_schliessen` ist herausgezogen und nicht abgeschrieben.** Ein zweiter Rumpf daneben
wäre eine zweite Stelle, die die Nachfrage aus C4 stellt, und die erste Abweichung zwischen
beiden fände keine Prüfung. Der Doc-Kommentar sagt das selbst und hat recht.

## Was ich nicht beurteilen kann

**Alles, was das laufende Bündel zeigt.** Der Abnahmelauf verlangt KRK im Vordergrund und ist
Nutzerarbeit. Was aus dem Baum entscheidbar war, ist oben entschieden; was der Nutzer drücken
muss, steht darunter.

## Was der Nutzer drücken soll

Sechs Handgriffe, in dieser Reihenfolge. KRK aus einem Terminalfenster im Vordergrund starten.

1. **Hinweg aus der Dateiliste.** Eine Textdatei auswählen, `cmd+e`. Erwartet: der Editor
   erscheint, zeigt die Datei, der Fokus steht in der Textfläche (Fokusrahmen, Fenstertitel).
   Dasselbe wie `f4`.
2. **Rückweg ohne ungesicherten Stand.** Sofort `cmd+e`. Erwartet: der Editor verschwindet, die
   Vorschau ist da und zeigt dieselbe Datei, der Fokus steht in der Dateiliste. Keine Rückfrage.
3. **Rückweg mit ungesichertem Stand, Antwort „Abbrechen".** Wieder `cmd+e` hinein, ein Zeichen
   tippen, `cmd+e`. Erwartet: die Nachfrage aus C4 steht. Auf „Abbrechen": der Editor steht
   unverändert mit dem getippten Zeichen da, **und die Vorschau bleibt draußen.** Das ist der
   Handgriff, den keine Probe deckt.
4. **Rückweg mit ungesichertem Stand, Antwort „Verwerfen".** Nochmal `cmd+e`, diesmal verwerfen.
   Erwartet: Editor weg, Vorschau da, Fokus in der Dateiliste.
5. **Hinweg aus der Vorschau, unverändert.** Mit `f3` in die Vorschau, dort `shift+cmd+y` für den
   Fokus, `cmd+e`. Erwartet: wie bisher, die angezeigte Datei im Editor.
6. **Die drei, die sich nicht geändert haben dürfen.** `f4` (öffnet wie bisher), `opt+cmd+e`
   (schließt, **Fläche bleibt leer — keine Vorschau danach**), `opt+cmd+b` (blendet um, ohne
   Nachfrage, Stand bleibt).

Ein siebter Handgriff prüft den Low-Befund aus `260823-1035`, falls er interessiert: `f3` zum
Ausblenden der Vorschau, dann `f4` in den Editor, dann `cmd+e`. Heute erscheint die Vorschau,
obwohl sie ausgeschaltet war.

## Empfohlene Reihenfolge

**Vor der nächsten Auslieferung:** `260823-1030` (die `keymap.toml`) — und zwar mindestens als
Entscheidung, auch wenn sie „es bleibt, wie es ist, und die Releaseseite sagt es" lautet. Alles
andere ist keine Auslieferungssperre.

**Beim nächsten Anfassen dieser Dateien:** `260823-1031` (ein Argument, macht den Nebenbefund
zur Aufruferzahl gegenstandslos), `260823-1032` und `260823-1033` (Prosa, zusammen wenige
Zeilen), `260823-1034` (die Quelltextprobe für `vorschau_danach`).

**Wenn der Nutzer gefragt ist:** `260823-1035`.

**Sammelbar:** `260823-1036`.

## Berührte offene Datensätze — nicht angefasst

- `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`
  — `260823-1030` ist dessen stärkere Gestalt und verweist darauf, ändert ihn aber nicht.
- `shared/issues/260823-0730_o_drei-prosastellen-um-den-neuen-nachzug-sind-mit-df8163d-falsch-geworden.md`
  — `260823-1032` ist derselbe Fehlschlag im nächsten Commit. Der ältere Datensatz bleibt
  unberührt; die zwei laufen getrennt.
- `260823-0731`, `260823-0732`, `260823-0733` — von diesem Commit nicht berührt. `28cbb7b` fasst
  weder `sichtbarkeit_aendern` noch `editorausgang_behandeln` an; die neue Zeile
  `bereich_einblenden(Bereich::Vorschau)` läuft durch dieselbe Stelle, die `260823-0732`
  beschreibt, ändert dort aber nichts.

---

## Abgleich 260823-1336

Alle sieben Befunde sind gegen den Baumstand `616ad5e` nachgelesen, jeder einzeln und nicht über
den Commit-Text.

| Befund | Stand | Beleg |
|---|---|---|
| `260823-1030` Kennung weist die `keymap.toml` ab | geschlossen als **angenommene Lage**, nicht behoben | der Mechanismus besteht fort (`belegung.rs:1423` bricht beim ersten unbekannten Bezeichner ab, nachgelesen); der Datensatz trägt `Resolved:` und darunter das vorgeschriebene `Revised by:`, weil die Schließungsbegründung am 260823-1140 richtiggestellt wurde |
| `260823-1031` zweite Fokuserhebung | geschlossen | `52fba42`; `fn editor_rundweg(&self, fokus: Fokus)`, der Zweig ruft `self.editor_rundweg(fokus)`, und `self.fokus()` hat wieder genau fünf Aufrufer |
| `260823-1032` zwei Zahlen im Modulkopf | geschlossen | `52fba42`; beide Zahlen gestrichen, „Zehn Module" bleibt und stimmt: `crates/krk-ui/src/kommandos/` trägt zehn Module neben `mod.rs` |
| `260823-1033` drei Stellen zum `false` | geschlossen | `52fba42`; `umbenennung_beginnen` in `tabelle.rs` sagt jetzt, der Wert entscheide über den Nachzug und nicht über das Weiterlaufen |
| `260823-1034` `vorschau_danach` ungeprüft | geschlossen | `52fba42`; `mod rundwegproben` mit vier Proben steht in `anwendung.rs` ab `:8412` |
| `260823-1035` Rückweg blendet immer ein | geschlossen ohne Verhaltensänderung | `52fba42`; der Nutzer hat am 260823-1235 Möglichkeit 1 gewählt, geändert ist die Begründung im Code |
| `260823-1036` Zuschreibung zweier Proben | geschlossen | `52fba42`; beide Doc-Kommentare tragen den Absatz, `resources/default-keymap.toml:825` führt die Umbenennung |

**Die drei Zeilenangaben dieses Berichts nach `anwendung.rs` gelten nicht mehr**, aus demselben
Grund wie im Bericht vom 260823-0735:
`shared/issues/260823-1336_*_die-zeilenzitate-der-zwei-offen-gebliebenen-befunde-*`.

Das im Bericht genannte Auslieferungshindernis ist mit `260823-1030` als angenommene Lage
erledigt und nicht behoben; der Auslöser für die Zukunft steht in jenem Datensatz.
