# Auslieferungsdurchsicht vor 1.0.0: `28cbb7b..b58e9d1`, die Deckungslücke

**Reviewed-range:** `28cbb7b..b58e9d1`
**Not-opened:** `fusion-workbench/.asset-provenance`, `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/shared/decisions/260818-0201_o_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`, `fusion-workbench/shared/decisions/260820-1034_i_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`, `fusion-workbench/shared/history/260823-0442-orchestrator-session.md`, `fusion-workbench/shared/history/260823-1137-coder-acht-befunde-aus-zwei-durchsichten.md`, `fusion-workbench/shared/history/260823-1340-reconciliation.md`, `fusion-workbench/shared/issues/260820-2056_o_dreissig-entscheidungsdatensaetze-tragen-eine-leere-vorlagenzeile-vor-der-gefuellten.md`, `fusion-workbench/shared/issues/260823-0731_o_ein-klick-in-das-andere-dateifenster-nimmt-eine-ziehbewegung-zurueck.md`, `fusion-workbench/shared/issues/260823-0732_o_der-neue-nachzug-laeuft-vor-dem-fokusumzug-und-oeffnet-den-ring-den-eine-probe-offen-haelt.md`, `fusion-workbench/shared/issues/260823-1336_o_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-der-baum-traegt-seit-dem-260819-zwei.md`, `fusion-workbench/shared/issues/260823-1336_o_die-zeilenzitate-der-zwei-offen-gebliebenen-befunde-und-beider-durchsichten-zeigen-nach-52fba42-ins-leere.md`, `fusion-workbench/stilwerk/chat-voice-en.yaml`, `fusion-workbench/stilwerk/default-voice-de.yaml`, `fusion-workbench/stilwerk/default-voice-en.yaml`

**Getragen aus der vorigen Durchsicht:** none. `260823-1040-coderev-cmd-e-wird-der-rundweg.md`
führt `**Not-opened:** none`; es war nichts nachzuholen.

**Durchgesehen von:** coderev, 260823-1450
**Bereich als beauftragt:** acht Commits, davon einer mit Code (`52fba42`, sechs Dateien).
**Am Baum gefahren:** `cargo test --workspace` (dreimal: einmal rot, zweimal grün),
`cargo test -p krk-core --test text <eine Probe>` (fünfmal grün),
`cargo clippy --workspace --all-targets -- -D warnings` (sauber),
`cargo fmt --all --check` (sauber).

**Die fünf Doc-Kommentare, die keine Durchsicht bestellt hatte, sind ungeöffnet weggelassen
worden — von den fünfzehn Nicht-Geöffneten oben trägt keine Code.** Alle sechs Codedateien des
Bereichs sind ganz gelesen.

## Summary

Die acht Befunde der zwei vorigen Durchsichten sind alle behoben, und ich habe jeden einzeln gegen
den Baum gelesen statt gegen seine `Resolved:`-Notiz. Kein Befund dieser Durchsicht ist ein
Verhaltensfehler; das ausgelieferte Erzeugnis verhält sich nach allem, was ohne laufendes Bündel
zu entscheiden ist, wie zugesagt. Der schwerste Befund ist ein Satz: „`kommando_ausfuehren`
liefert seit der Runde 7 immer `true`" trifft nicht zu, steht an vier Codestellen und seit dem
260823-1350 in der `Implemented:`-Zeile eines bindenden Entscheidungsdatensatzes — die
Nutzerentscheidung selbst ist davon **nicht** betroffen, weil Frage und `Answered:`-Zeile die
richtige Fassung tragen. Dazu kommt ein Befund am Prüfstand: eine Wettrennprobe mit fester
15-Sekunden-Frist ist unter Last rot geworden, und das ist der erste rote `cargo test`-Lauf
dieses Projekts, dessen Ausgabe erhalten ist.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 3 |

## Steht der Auslieferung von 1.0.0 etwas im Wege?

**Nein, kein Befund dieser Durchsicht ist ein Auslieferungshindernis.** Keiner betrifft Verhalten,
keiner betrifft eine der zehn Zeitzusagen, und keiner betrifft die Auslieferungskette selbst.

Drei Sachen gehören trotzdem vor `./release.sh 1.0.0` gesagt.

**1. Der grüne `make check` vor dem Auslieferungslauf ist weniger wert, als er aussieht.** Die
Probe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` hat eine feste Frist von 15 Sekunden
und braucht allein gefahren 8,3 bis 9,2 davon. Unter `cargo test --workspace` ist sie in einem von
drei Läufen darüber gekommen. Ein roter Lauf beweist damit keinen Defekt, und, was schwerer wiegt,
ein roter Lauf kann als „der flackernde" abgetan werden, wenn er einmal wirklich einen Defekt
zeigt. Die Auslieferungskette fängt das nicht auf: **keine ihrer acht Stationen fährt Proben.**
Station 1 prüft Tag, Arbeitsbaum und `gh`, Station 2 die AppKit-Grenze, Station 3 übersetzt. Was
zwischen einer kaputten Probe und einer öffentlichen Releaseseite steht, ist allein der `make
check`, den der Nutzer selbst fährt. Datensatz `260823-1436`.

**2. Die zehn Zeitzusagen sind in diesem Bereich nicht berührt, und der Bereich ist nicht der
Grund, sie zu messen.** `52fba42` ändert am ausgelieferten Erzeugnis genau eine ausgeführte Zeile:
`editor_rundweg` bekommt den Fokus als Argument, statt `self.fokus()` ein zweites Mal zu rufen —
also **eine Erhebung weniger** pro `cmd+e`. Alles Übrige sind Doc-Kommentare und zwei
`#[cfg(test)]`-Module, die im Releaseprofil nicht entstehen. Der offene Befund
`shared/issues/260823-0732_o_…` führt die L1-Frage zu Recht als ungemessen, aber sein Anlass ist
`df8163d` und liegt **vor** diesem Bereich. Wer vor 1.0.0 messen will, misst wegen `df8163d` und
nicht wegen `52fba42`.

**3. Der Auslöser, den `260823-1030` sich selbst gesetzt hat, rückt mit 1.0.0 näher.** Der
Datensatz ist als angenommene Lage geschlossen, geprüft und richtig: die umbenannte Kennung
`editor_rundweg` weist jede bestehende `keymap.toml` **ganz** ab, und auf beiden Maschinen des
Nutzers liegt keine. Sein Auslöser lautet „der erste Nutzer außer dem Entwickler". Eine 1.0.0 auf
einer öffentlichen Releaseseite ist die Einladung dazu. Das ist kein Hindernis und keine
Neuerhebung; es ist der Hinweis, dass jede **künftige** Umbenennung einer Kennung ab dann
Datenverlust beim Nutzer ist. Der Datensatz ist nicht angefasst.

## Was ich geprüft und nicht beanstandet habe

**Die acht Behebungen tragen, jede einzeln am Baum nachgelesen.**

| Befund | Was der Baum zeigt |
|---|---|
| `260823-0730` Aufruferzählung | ersetzt durch eine Regel ohne Zahl; die drei Messer stimmen (`anwendung.rs:3025`, `:6476`, `:7235`, gezählt über `grep -n bildschirmbreiten_uebernehmen`) |
| `260823-0733` Probe deckt einen Zweig | `die_editorfortsetzung_misst_als_erste_anweisung` prüft jetzt die Stellung statt einer Reihenfolge; der Helfer `erste_anweisung` überspringt die Signaturzeile richtig, und `editorausgang_behandeln` misst als erste Anweisung (`:6476`) |
| `260823-1030` Kennung weist Belegung ab | als Lage angenommen, auf beiden Maschinen geprüft; `resources/default-keymap.toml:825-829` führt die Umbenennung aus |
| `260823-1031` zweite Fokuserhebung | `fn editor_rundweg(&self, fokus: Fokus)`, Zweig ruft `self.editor_rundweg(fokus)`; `self.fokus()` hat wieder genau fünf Aufrufer (1364, 1909, 4359, 6515, 7009), und die Zahl „fünf" bei `:5709` stimmt damit wieder |
| `260823-1032` zwei Zahlen im Modulkopf | gestrichen statt korrigiert; „Zehn Module" bleibt und stimmt (`ls crates/krk-ui/src/kommandos/` ohne `mod.rs`: zehn) |
| `260823-1033` drei Stellen zum `false` | die örtliche Aussage ist an allen Stellen richtiggestellt, die Begründung überzieht — siehe Befund 1 unten |
| `260823-1034` `vorschau_danach` ungeprüft | vier Proben, jede gegen die Zeile gelesen, die sie hält: `:3170` trägt `false`, `:7083` trägt `true`, `anlass_ausfuehren` liest `if vorschau_danach`, `anlass_unterbleibt` beantwortet `EditorSchliessen { .. }` mit leerem Rumpf und nennt das Feld nur in Kommentarzeilen, die `rumpf` abzieht |
| `260823-1036` Zuschreibung zweier Proben | beide Doc-Kommentare tragen den Absatz zur Umbenennung |

**Die vier neuen `rundwegproben` lösen wirklich aus.** Die Nadeln sind zusammengesetzt
(`concat!`), treffen sich also nicht selbst; `rumpf` schneidet ab `fn <name>(` und zieht
Kommentarzeilen ab, die vier gelesenen Rümpfe enden alle an der ersten schließenden Klammer auf
vier Leerzeichen Einrückung, und keine der vier Nadeln kommt in Prosa vor. Ein vertauschtes
`true`/`false` an `:3170` oder `:7083` lässt Probe 1 beziehungsweise 2 fallen, ein gestrichenes
`if vorschau_danach` Probe 3, ein ergänztes in `anlass_unterbleibt` Probe 4.

**Die fünf zusätzlich gefundenen Stellen stimmen gegen den Baum, mit einer Einschränkung.**
`bildschirmbreiten_uebernehmen` (drei Messer, gezählt), `sitzung_bauen` (verweist auf die Regel
statt zu zählen), beide Leerwege von `bereichskommando` (`Fokus::Vorschau` und `Fokus::Editor =>
false`, beide sagen jetzt richtig, dass kein Nachzug anfällt und der Tastendruck trotzdem
verbraucht ist) und `umbenennung_beginnen` in `tabelle.rs:2206` — dessen Formulierung „schluckt
jeden **zulässigen** Befehl" ist die einzige unter den fünf, die den Satz aus Befund 1 richtig
stellt, und sie ist die Vorlage für seine Behebung. `terminal_oeffnen` und
`weitere_instanz_starten` liefern wirklich immer `true`, das habe ich an ihren Rümpfen gelesen
(`:1961-1972`, `:1984-1990`).

**Die Zeilenzitate in den Workbench-Datensätzen sind nicht das Ganze, aber `52fba42` hat kein
neues erzeugt.** Antwort auf die vierte Frage der Beauftragung: im Quelltext gibt es nur **ein**
Zitat nach `anwendung.rs`, `xtask/src/release.rs:796`, und es war schon bei `28cbb7b` falsch.
Siehe Befund 4.

**Der Ereignisabgriff ist nicht berührt.** `52fba42` meldet keine Textfläche an und nimmt keine ab;
`ist_eigene_textflaeche` und `ersthelfer_gehoert_appkit` sind unverändert. Die Zulässigkeitsregel
ist unverändert, ebenso die vier Aufzählungen aus dem Abschnitt „Projektstand" der `CLAUDE.md`.
Kein `#[must_use]` ist stillgelegt: `rundweg()` wird weiterhin über `let Some(weg) = … else`
verbraucht, `editor_schliessen` steht als Rumpfausdruck seiner Zweige.

## Befunde nach Thema

### Was ein Rückgabewert bedeutet — Medium

**`kommando_ausfuehren` liefert nicht „immer `true`".**
`shared/issues/260823-1433_o_kommando-ausfuehren-liefert-nicht-immer-true-*`

Die Funktion hat zwei Ausgänge (`crates/krk-ui/src/appkit/anwendung.rs:3003` und `:3242`): `false`
für jeden Befehl, den `zulaessigkeit::zulaessig` abweist, `true` für jeden, der durchkommt. Drei
Stellen desselben Baums sagen es richtig und stehen unverändert daneben (`anwendung.rs:5355`,
`messmodus.rs:94`, `blaetter/mod.rs:305`: „weist ab"), `CLAUDE.md` ebenso. Vier Codestellen tragen
seit `52fba42` die absolute Fassung, und ihr Weg ist nachzuverfolgen: Bericht `260823-1040` →
Datensatz `260823-1033` → vier Doc-Kommentare → `Implemented:`-Zeile von
`shared/decisions/260813-0053_i_…`.

**Die Entscheidung des Nutzers steht auf richtiger Grundlage.** Frage, Möglichkeit 1 und die
`Answered:`-Zeile jenes Datensatzes tragen alle die bedingte Fassung („der Abgriff schluckt, was
zulässig war"). Falsch ist der zusammenfassende Satz, mit dem die Umsetzung nachgetragen wurde.
An den vier Codestellen ist auch die örtliche Folgerung richtig — alle vier sitzen hinter der
Prüfung —, nur die Begründung reicht über ihren Fall hinaus.

### Der Prüfstand — Medium

**Eine Wettrennprobe mit fester Frist fällt unter Last.**
`shared/issues/260823-1436_o_die-wettrennprobe-des-oeffnens-*`

`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:802`)
fährt 20 000 Öffnungen gegen 2 000 Tausche und bricht nach 15 Sekunden Wanduhr ab. Allein
gefahren: fünfmal grün, 8,34 bis 9,16 Sekunden. Unter `cargo test --workspace`: einmal rot mit
15,06 Sekunden, zweimal grün. Die Abbruchmeldung nennt eine Ursache („das Oeffnen haengt an der
benannten Roehre"), die sie von der zweiten möglichen — die Maschine war zu langsam — nicht
trennen kann, obwohl die Probe die Zähler dafür in der Hand hält und im `Err`-Zweig nicht ausgibt.

**Das ist der rote Lauf mit erhaltener Ausgabe, den `shared/issues/260823-1210_o_…` verlangt.**
Jener Datensatz ist **nicht angefasst**; ob er damit zu schließen ist, gehört dorthin. Beweisen
lässt es sich nicht: die Ausgabe des Laufs vom 260823-1205 ist verloren.

### Zusagen, die weiter reichen als ihr Beleg — Low

**Der Modulkopf der `rundwegproben` nennt eine Abwehr, die den genannten Fall nicht abwehrt.**
`shared/issues/260823-1442_o_der-modulkopf-der-rundwegproben-*`

Er benennt seine Lücke — einen dritten Rufer von `editor_schliessen` — und verweist auf
`die_regel_hat_genau_einen_aufrufer`. Die zählt die Rufer von `rundweg`, nicht die von
`editor_schliessen`. Ein dritter Rufer bestünde sie und alle vier neuen Proben. Das Werkzeug für
die richtige Zählung liegt daneben (`crate::quellbaum::aufrufstellen`).

**Die neue Regel an `bildschirmbreiten_uebernehmen` verweist jeden Rufer an sich selbst; einer
sagt dort nichts.**
`shared/issues/260823-1445_o_die-neue-regel-verweist-jeden-rufer-an-sich-selbst-*`

Fünf Rufer von `aufteilung_nachziehen`; vier tragen an sich, was zu ihnen gehört
(`oberflaeche_aufbauen` und `anlass_ausfuehren` begründen, `kommando_ausfuehren` misst,
`aktives_setzen` steht als offener Befund `260823-0731`). `sichtbarkeit_aendern` (`:4264`) sagt
nichts. Ein Loch im Verhalten ist es nicht — seine beiden Rufer werden nur aus messenden Wegen
erreicht —, aber der Satz, der `aktives_setzen` als den einen ohne Begründung benennt, liest sich
als vollständige Abrechnung und ist es nicht.

### Zeilenzitate im Quelltext — Low

**Drei Zitate zeigen ins Leere, und keines stammt aus `52fba42`.**
`shared/issues/260823-1439_o_drei-zeilenzitate-im-quelltext-*`

`belegung.rs:1272` und `parser.rs:34` zeigen beide auf `menue.rs:322-342` und meinen
`zeichen_der_taste`, das bei `:580` steht. `hervorhebung.rs:178` zeigt auf `leiste.rs:439-442` und
meint die Begründung bei `:567`. `xtask/src/release.rs:796` zeigt auf `anwendung.rs:575` und meint
eine Zeile, die dreißigmal in der Datei steht, zuerst bei `:1101` — und war schon bei `28cbb7b`
falsch. Das einzige noch tragende Zitat ist `tabelle.rs:381` → `messen.rs:1199`.

## Übergreifend

**Ein falscher Satz ist in diesem Projekt teurer als eine falsche Zahl, weil ihn nichts hält.**
Die Sitzung hat für Zahlen die richtige Lehre gezogen: `260823-1032` streicht sie statt sie zu
korrigieren, `260823-0730` ersetzt eine Aufzählung durch eine Regel. Für Sätze gibt es dieselbe
Antwort nicht, und Befund 1 zeigt den Weg, den einer nimmt: aus einem Durchsichtsbericht in einen
Defektdatensatz, von dort in vier Doc-Kommentare, von dort in einen Entscheidungsdatensatz mit
Marker `_i_`. Vier Stationen, drei davon in **einer** Sitzung, ohne dass jemand den Satz gegen die
Funktion gelesen hätte, um die es ging. Der Baum hatte die Gegeninstanz die ganze Zeit dreimal
ausgeschrieben stehen.

**Die drei Low-Befunde sind dieselbe Gestalt.** Eine Prosastelle sagt zu, was ein Prüfstand,
eine Zählung oder ein Nachbarkommentar nicht einlöst: der Modulkopf verspricht eine Abwehr, die
Regel verspricht eine Adresse, das Zeilenzitat verspricht eine Fundstelle. Keine davon ist falsch
aus Nachlässigkeit; alle drei sind an der Naht falsch, an der eine Zusage in eine Prüfung
übergehen soll und stattdessen bei der Zusage bleibt.

## Was ich nicht beurteilen kann

Alles, was das laufende Bündel zeigt. Der Nutzer hat `f4` und den `cmd+e`-Rundweg am 260823-1320
von Hand abgenommen; der Code darunter ist geprüft. Die zwei offenen Verhaltensbefunde aus dem
Bereich davor (`260823-0731` Ziehbewegung, `260823-0732` Reihenfolge und L1) brauchen KRK im
Vordergrund und sind unverändert Nutzerarbeit. Beide sind **nicht angefasst**.

## Berührte offene Datensätze — nicht angefasst

- `shared/issues/260823-0731_o_…` — von `260823-1445` zitiert, nicht geändert.
- `shared/issues/260823-0732_o_…` — im Abschnitt zur Auslieferung genannt; der Bereich berührt
  ihn nicht, sein Anlass liegt davor.
- `shared/issues/260823-1210_o_…` — `260823-1436` liefert den roten Lauf mit Ausgabe, den er
  verlangt, und schließt ihn nicht.
- `shared/issues/260823-1336_o_die-zeilenzitate-…` — `260823-1439` ist dieselbe Klasse im
  Quelltext und läuft daneben.
- `shared/issues/260823-1336_o_claude-md-nennt-einen-empfaenger-…` — vom Bereich nicht berührt.
- `shared/decisions/260813-0053_i_…` — `260823-1433` beanstandet einen Satz seiner
  `Implemented:`-Zeile; die Entscheidung selbst bleibt, wie sie ist.

## Empfohlene Reihenfolge

**Vor `./release.sh 1.0.0`:** nichts. Kein Befund ist ein Hindernis. Wer vor dem Lauf `make check`
fährt und ihn rot bekommt, liest `260823-1436`, bevor er ihn wiederholt.

**Beim nächsten Anfassen dieser Dateien:** `260823-1433` (vier Codestellen und eine
`Implemented:`-Zeile, wenige Zeilen, die Vorlage steht in `tabelle.rs:2206`), dann `260823-1442`
und `260823-1445` — alle drei liegen in denselben zwei Dateien und gehören in einen Griff.

**Wenn der Prüfstand drankommt:** `260823-1436`. Er braucht eine Entscheidung vorab, ob die Frist
an der Zahl der Durchläufe oder am Fortschritt hängen soll; nicht nebenbei greifen.

**Sammelbar:** `260823-1439`.
