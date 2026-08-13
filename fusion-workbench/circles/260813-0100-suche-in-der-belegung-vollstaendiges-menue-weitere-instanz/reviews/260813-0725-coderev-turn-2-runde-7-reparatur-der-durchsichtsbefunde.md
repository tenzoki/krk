# Codeprüfung: Turn 2 der siebten Runde, die Reparatur der Durchsichtsbefunde

**Sender:** coderev
**Reviewed-range:** `a34bf17..dff167a`
**Not-opened:** none
**Datum:** 260813-0725
**Umfang:** der eine Commit `dff167a`, 41 Dateien — 19 Rust-Dateien, `resources/default-keymap.toml`,
die Verlaufsdatei des `coder` und 20 Datensätze unter `issues/`. Der Baum ist dabei am Stand
`dff167a` gelesen worden und nicht nur der Diff.

---

## Zusammenfassung

**Die achtzehn Behebungen halten, alle achtzehn am Baum nachgelesen, und die vier Punkte des
Auftrags sind einzeln nachgeprüft statt geglaubt.** Der Baum ist grün: `cargo fmt --all
--check`, `cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace`
laufen am 260813-0705 ohne Fehlschlag und ohne Warnung durch. Kein Befund dieser Durchsicht ist
ein Freigabehindernis, und keiner macht den Abnahmelauf am Bündel unmöglich.

**Der Reparatur-Turn hat trotzdem etwas eingeschleppt, und es ist dieselbe Sorte, gegen die er
angetreten ist.** Sechs Befunde, alle in der neuen Bauform der Zählproben und ihren Texten: eine
Probe sagt eine Vollständigkeit zu, die Rust nicht hergibt; ein Modulkopf sagt, eine Lücke sei
bewacht, und die genannte Probe bewacht die andere Hälfte; eine neue Zahl ist genau die Sorte
Aufruferzählung, die derselbe Commit drei Absätze weiter verbietet. Keiner ändert Verhalten,
alle sind ein Absatz oder eine Zeile.

**Beide Ermessensentscheidungen des Ausführenden tragen.** Die vierte Prüfordner-Fassung ohne
Rückfrage zu entfernen war richtig, und `Ablage::pfad` öffentlich zu lassen ebenfalls; die
Begründungen dazu stehen unten unter „Zwei Entscheidungen".

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch (Freigabehindernis, Sicherheit, Datenverlust) | 0 |
| Hoch (Korrektheitsfehler, gebrochener Ablauf) | 0 |
| Mittel | 2 |
| Gering | 4 |

Alle sechs liegen als eigene Datensätze unter
`circles/260813-0100-…/issues/`, mit `260813-0715` bis `260813-0720` im Namen.

---

## Die vier Punkte des Auftrags

### 1. Die neue Bauform der Zählproben

**Die drei Folgerungen im Kopf von `crates/krk-ui/src/quellbaum.rs:56-71` sind richtig gezogen
und angewandt und nicht nur aufgeschrieben.** Die C4.6-Gegenprobe sucht jetzt den Gegenstand —
`impl Drop for `, `temp_dir()` und `remove_dir_all` in derselben Datei — statt des Namens
`Pruefordner` (`crates/krk-core/tests/baum.rs:136-150`). Sie ist gegen das eigene Gegenbeispiel
geprüft: der gelöschte `struct Ordner` trug alle drei Zeichen und wäre gefunden worden, und die
drei anerkannten Fassungen tragen sie ebenfalls, nachgesehen in
`crates/krk-core/tests/gemeinsam/mod.rs:51-91`, `crates/krk-ui/src/pruefordner.rs:62-131` und
`crates/krk-bench/src/wegwerfordner.rs:41-57`. Die Vorkehrung gegen den Selbstfund ist nötig
und richtig gebaut: `im_code` (`tests/baum.rs:50-54`) sucht nur in Code-Zeilen, weil die
Doc-Kommentare darüber jede Nadel im Klartext nennen.

**Die Behauptung über Rust hält nicht.** Der Doc-Kommentar von
`nur_benannte_dateien_erreichen_das_atomare_schreiben` sagt (`tests/baum.rs:164-170`), es gebe
genau zwei Wege an eine fremde Funktion, beide nennten das Modul, und „ein anderer Weg besteht
nicht". Zwei Gegenbeispiele, beide mit `rustc --edition 2024` übersetzt:

```rust
use krk_core::ablage::atomar as werkzeug;   // Modul unter anderem Namen
werkzeug::schreiben(&pfad, &text)?;         // keine der drei Nadeln in dieser Datei
```

```rust
pub use atomar::schreiben;                  // in ablage/mod.rs, schon auf der Liste
krk_core::ablage::schreiben(&pfad, &text)?; // ueberall sonst, ohne jede Nadel
```

Der Schluss „beide nennen das Modul, **also** steht dort `atomar::`" ist der Fehler; ein `use`
darf umbenennen. Der zweite Weg ist der unangenehmere, weil die Zeile, die ihn öffnet, in einer
bereits erlaubten Datei steht und die Probe damit für den ganzen Baum blind macht, ohne sie rot
werden zu lassen. Benannt ist an der Probe allein der über zwei Zeilen umbrochene Pfad — also
gerade nicht die zwei Wege, die es wirklich gibt. Das verletzt Folgerung 3 aus demselben
Commit.
→ `issues/260813-0715_o_die-neue-atomar-probe-behauptet-eine-vollstaendigkeit-die-rust-nicht-hergibt.md`

**`quelldateien` auf alle Kisten umzustellen trägt, und keine Probe ist dadurch woanders falsch
geworden.** Nachgesehen an allen fünfzehn Aufrufstellen in `krk-ui`: neun Erwartungen tragen
jetzt das Kistenpräfix, und keine der Nadeln kommt außerhalb von `krk-ui` vor — nachgeprüft für
`isKindOfClass`, `downcast_ref::<NSText`, `NSSharingServicePicker::`, `hauptmenue(`,
`setAutoenablesItems(` und die Ersthelfer-Erklärung. Der teurere der zwei vorgeschlagenen Wege
war der richtige: C2.16 sagt „an genau einer Stelle" ohne Kistengrenze zu, und die billige
Variante hätte die Doc-Kommentare auf „in `krk-ui`" umgeschrieben und die Zusage ungedeckt
gelassen.

Eine Anmerkung ohne Befundcharakter: „alle Kisten" heißt in beiden Lesern `crates/`, und der
vierte Workspace-Teilnehmer `xtask/` liegt daneben. Für die Zusagen dieser Proben ist das
folgenlos — `xtask` hat keine einzige Abhängigkeit (`xtask/Cargo.toml:8`) und kann `krk-core`
nicht erreichen —, und `crates/` ist die eingeführte Wurzel dieses Baums, die auch die
AppKit-Grenzprüfung in `xtask/src/release.rs:75` nimmt.

**Das neue Werkzeug `aufrufstellen` (`quellbaum.rs:117-153`) tut, was sein Doc-Kommentar sagt.**
Die drei Abzüge sind einzeln nachgerechnet, und die eigene Probe darüber ist die richtige
Vorsichtsmaßnahme; dass ihr Beispielname erfunden ist, ist kein Zierrat, sondern die Lehre aus
dem ersten Versuch. Blind bleibt es gegen `use … as`, und das steht dort.

### 2. Der Messmodus

**Am Entstehen kommt niemand mehr vorbei, und das ist die Zusage, um die es ging.**
`Sitzungsschreiber::neu` und `::mit_takt` verlangen ein `&Sitzungsrecht` und liefern `None`
(`crates/krk-core/src/ablage/sitzung.rs:452-467`). `Sitzungsrecht` hat ein privates Feld und
genau zwei Erzeuger (`crates/krk-core/src/ablage/sperre.rs:172-201`): `nehmen`, das die
`flock`-Sperre wirklich versucht, und `ohne`, das `gehalten` verneint. Ein gehaltenes Recht ist
ohne Sperre nicht herzustellen. Nachgesehen ist auch, dass es keinen zweiten Schreibweg auf
`session.toml` gibt: `zugang.sichern(Datei::Sitzung, …)` steht im ganzen Baum einmal, in
`Sitzungsschreiber::schreiben` (`sitzung.rs:516`).

**Die Verklemmungsfrage ist mitgeprüft und in Ordnung.** `Messplan::herstellen` nimmt erst das
Recht, dann den Schreibgriff (`crates/krk-ui/src/messmodus.rs:308-342`) — dieselbe Reihenfolge
wie der gewöhnliche Start (`crates/krk-ui/src/appkit/anwendung.rs:1229-1249`), also kein Ring.
Zwei Rechte zugleich hält niemand: der Messlauf kehrt aus dem `Aufgabe::Sitzung`-Zweig zurück,
bevor der gewöhnliche Weg das Recht überhaupt anfasst.

**Ein Weg bleibt offen, und es ist die Lebensdauer.** Das Recht wird nur geliehen; ein
`Sitzungsschreiber` kann es überleben, weil er keine Lebenszeit trägt. Der Übersetzer hält „war
Halterin, als der Schreiber entstand", der Doc-Kommentar am Feld `sitzungsrecht` sagt „nur die
Halterin schreibt die Sitzung" (`anwendung.rs:538-541`). Zwischen beidem liegt die Schreibspanne.
Kein Aufrufer nimmt den Weg heute — alle drei sind einzeln nachgelesen.
→ `issues/260813-0719_o_ein-sitzungsschreiber-kann-sein-sitzungsrecht-ueberleben.md`

### 3. Die vier umgezogenen Proben

**Alle vier prüfen dasselbe wie vorher; keine ist still schwächer geworden.** Einzeln
gegenübergestellt:

| Probe | Vorher / nachher |
|---|---|
| `das_sitzungsrecht_bekommt_nur_der_erste_halter` | gleich, dazu ein ausdrückliches `ort.anlegen()` |
| `ein_abgegebenes_sitzungsrecht_ist_wieder_zu_haben` | gleich, dazu `ort.anlegen()` |
| `ein_recht_ohne_ablageordner_wird_nicht_gehalten` | wortgleich |
| `zwei_ablagen_eines_prozesses_teilen_die_schreibsperre_nicht` | gleiche Zusicherungen, andere Herkunft des Deskriptors |

Die vierte ist die einzige mit einem Unterschied, der einen Satz braucht. Sie brauchte das
kisteninterne `sperre::sperrdatei_oeffnen` und hat jetzt einen eigenen Öffner in
`crates/krk-core/tests/ablage.rs:1813-1822`, den `kind_meldet_die_schreibsperre` mitbenutzt. Die
drei Zusicherungen sind unverändert, und die Sichtbarkeit von `sperrdatei_oeffnen` ist nicht
gehoben worden — das ist die richtige Reihenfolge. Der Preis ist eine Kopie der
`OpenOptions`-Kette; ändert sie sich in `sperre.rs`, merkt die Probe es nicht. Das ist eine
Anmerkung und kein Datensatz: die Probe braucht nur einen Deskriptor auf die Sperrdatei, und
den bekommt sie so wie so.

Eine Genauigkeit zur Begründung des `coder`: „keine der vier braucht das kistenintern sichtbare
`Schreibgriff::nehmen`" stimmt wörtlich, aber eine der vier brauchte sehr wohl etwas
Kisteninternes, nämlich `sperrdatei_oeffnen`. Die Begründung von damals war also nicht
grundlos, sondern nannte das falsche Symbol. Am Ergebnis ändert das nichts.

### 4. Die Belegungsdatei

**Ausschließlich Kommentarzeilen, selbst nachgezählt.** `git diff a34bf17..dff167a --
resources/default-keymap.toml` liefert 36 geänderte Zeilen; davon beginnen alle mit `#`. Die
Gegenprobe mit `grep -vE '^[+-]#'` über die Diff-Zeilen ohne Kopf ergibt **null** Treffer.
Keine Kennung, keine Taste, keine Zahl ist berührt. Die drei angebauten Absätze — Blockreihenfolge,
Eingabetaste hinter dem Nachschlag, Fokusvorbehalt hinter statt vor dem Nachschlag — treffen den
Baum, nachgelesen gegen `crates/krk-ui/src/appkit/ereignisse.rs:495-560` und
`crates/krk-ui/src/belegungsmodell.rs`.

---

## Befunde nach Themen

### Thema A: Was die neuen Wachen zusagen und was sie halten

**A1 · Die neue `atomar`-Probe behauptet eine Vollständigkeit, die Rust nicht hergibt.**
*(mittel)* Oben unter Punkt 1 ausgeführt.
→ `issues/260813-0715_o_die-neue-atomar-probe-behauptet-eine-vollstaendigkeit-die-rust-nicht-hergibt.md`

**A2 · Die bewachte Lücke ist nicht die Lücke.** *(mittel)* Der Modulkopf der Ablage sagt seit
Turn 2, `nur_benannte_dateien_erreichen_das_atomare_schreiben` bewache die Lücke
(`crates/krk-core/src/ablage/mod.rs:41-45`). Die Probe zählt, wer `atomar::schreiben` erreichen
kann. Ein Schreibweg an der Sperre vorbei braucht `atomar::schreiben` gar nicht: `Ablage::pfad`
plus jede Schreibfunktion der Standardbibliothek genügt, und das ist genau der Weg, den der
ursprüngliche Befund zitierte. Elf Stellen dieser Bauart stehen weiter in
`crates/krk-core/tests/ablage.rs` (505, 552, 612, 692, 894, 950, 1031, 1167, 1192, 2201, 2346);
zwei gleichartige sind gezogen worden, elf nicht, und der schließende Datensatz nennt sie nicht.
Die elf sind der Sache nach vertretbar — sie stellen einen Altbestand oder eine beschädigte
Datei her, was keine Serialisierung liefern kann. Der Befund ist der Satz im Modulkopf, nicht
der Code. Kein Produktionsweg ist betroffen.
→ `issues/260813-0716_o_die-bewachte-luecke-ist-nicht-die-luecke-elf-schreibwege-an-der-sperre-vorbei-bleiben.md`

**A3 · Die neue Gesamtzahl acht ist eine Aufruferzählung ohne Kriterium.** *(gering)*
`der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten`
(`crates/krk-ui/src/appkit/menue.rs:1226-1234`) sichert jetzt zusätzlich `alle == 8` zu. Der Kopf
von `quellbaum.rs:41-47`, im selben Commit geschrieben, sagt: eine Aufruferzählung steht nur
dort, wo ein Abnahmekriterium die Zahl selbst zusagt, „und nirgends als Stellvertreter für ‚es
gibt keinen Doppelbau'". Die Acht sagt kein Kriterium zu, und ihr Doc-Kommentar nennt genau
diesen Dienst als Zweck. Dazu stimmt ihre Erklärung für zwei der acht nicht: `anwendung.rs:5496`
und `:5524` sind keine Weiterreichungen, sondern `messhandlung`, das
`Tabellenquelle::kommando_ausfuehren` direkt ruft und damit am Delegierten vorbei.
→ `issues/260813-0718_o_die-neue-gesamtzahl-acht-ist-eine-aufruferzaehlung-ohne-kriterium-und-nennt-zwei-falsch.md`

**A4 · Die C4.6-Nadel benennt ihren wahrscheinlichsten blinden Fleck nicht.** *(gering)* Genannt
sind zwei Restblindheiten (`tests/baum.rs:109-112`), eine über zwei Dateien verteilte Fassung und
eine, die Eintrag für Eintrag abräumt. Nicht genannt ist die dritte, und es ist die, nach der
dieses Projekt schon einmal gefragt hat: eine Fassung, die ihren Ordner nicht unter
`std::env::temp_dir()` anlegt. Alle drei anerkannten Fassungen tragen einen eigenen Absatz, der
den Messplatz unter `~/Library/Caches/krk-messplatz` ausdrücklich ausschließt.
→ `issues/260813-0720_o_die-c4-6-nadel-benennt-ihren-wahrscheinlichsten-blinden-fleck-nicht.md`

**A5 · Ein Sitzungsschreiber kann sein Sitzungsrecht überleben.** *(gering)* Oben unter Punkt 2.
→ `issues/260813-0719_o_ein-sitzungsschreiber-kann-sein-sitzungsrecht-ueberleben.md`

### Thema B: Prosa, die den Baum nicht trifft

**B1 · Ein Doc-Kommentar in `tests/belegung.rs` nennt `atomar::schreiben`, und der Code nimmt
`fs::write`.** *(gering)* Der angebaute Absatz an `ablage_mit` endet mit „der Vorgang aus
`atomar::schreiben`, wie bei `settings.toml`" (`crates/krk-core/tests/belegung.rs:41`); der Rumpf
drei Zeilen weiter ruft `fs::write` (`:45-48`). Bei `settings.toml`
(`crates/krk-core/tests/ablage.rs:372-380`) steht wirklich `atomar::schreiben`. Der Unterschied
ist nicht folgenlos: hätte `ablage_mit` den genannten Aufruf genommen, wäre `tests/belegung.rs`
die sechste Datei geworden, die A1s Probe zählt, und jene rot.
→ `issues/260813-0717_o_ein-doc-kommentar-in-tests-belegung-rs-nennt-atomar-schreiben-und-der-code-nimmt-fs-write.md`

---

## Was geprüft und in Ordnung befunden ist

Diese Punkte sind einzeln am Baum verfolgt worden und tragen keinen Befund.

**Der Wegfall von `sitzung_vormerken()` beim Beenden ist verhaltensgleich und behebt eine echte
Doppelnahme.** `applicationWillTerminate:` (`crates/krk-ui/src/appkit/anwendung.rs:807-833`) baut
den Stand einmal, merkt ihn vor und beendet, alles in einem Durchgang. Vorher lief
`sitzung_vormerken()` mit einem eigenen Durchgang davor und wurde von den Zeilen darunter
überschrieben. Eine Verklemmung durch die neue Form gibt es nicht: `unter_der_sperre`
(`:1139-1147`) fasst nur `ivars.ablage` an, nicht `ivars.sitzungsschreiber`, dessen `RefMut`
über den Aufruf gehalten wird.

**Der Kürzelfilter im Menümodell ist an der richtigen Stelle gelandet.** `eigenes` steht jetzt vor
der Fallunterscheidung (`crates/krk-ui/src/menuemodell.rs:251`) und gilt für beide
`Eintrag::Befehl`-Zweige; der `Eintrag::Textbefehl`-Zweig behält bewusst die ungefilterte
Kombination, weil er selbst der Zusteller ist. Das ist richtig und eine Zeile kürzer als der
Vorschlag im Datensatz.

**Die Verengung des `setEnabled`-Verbots ist sauber geschnitten.** Verboten ist es nur noch in
Dateien, die `NSMenuItem` überhaupt nennen (`crates/krk-ui/src/appkit/menue.rs:1136-1155`);
`setAutoenablesItems` bleibt im ganzen Baum verboten, weil die Methode `NSMenu` gehört. Die
Restblindheit steht am Doc-Kommentar.

**`weitereinstanz::starten` fragt den Bündelort wirklich nur noch einmal.**
`eigenes_buendel` liefert die `NSURL` (`crates/krk-ui/src/appkit/weitereinstanz.rs:91-98`), und
`starten` schöpft allein daraus; der `PathBuf` und sein `use` sind fort, und der falsche
Nebenausgang mit ihnen.

**Die Doppelnennung von `getipptes_zeichen` ist beseitigt, ohne die Reihenfolge zu ändern.** Der
Wert wird an derselben Stelle geholt wie vorher der erste Aufruf
(`crates/krk-ui/src/appkit/ereignisse.rs:509-512`); der Sprungmarkenzweig liest die Bindung.

**Die Ersthelfer-Nadel deckt jetzt beide Schreibweisen**, und die Wahl von
`downcast_ref::<NSText` statt `downcast_ref::<` ist die richtige: sie fasst die drei Textklassen
und lässt die `NSView`-Frage in `anwendung.rs:4070` heraus, die keine Textklasse nennt.

---

## Zwei Entscheidungen

**Die vierte Prüfordner-Fassung ohne Rückfrage zu entfernen war richtig.** Die Frage, die der
Datensatz dem Nutzer vorlegte, ist in `CLAUDE.md` beantwortet („genau drei Fassungen, eine je
Kiste, und das soll so bleiben"), und `genau_drei_pruefordner_fassungen_stehen_im_baum` trägt sie
als Abnahmekriterium C4.6. Eine bestehende Zusage einzuhalten ist keine Entscheidung; vorzulegen
wäre allein gewesen, sie zu ändern. Dazu kommt: die Begründung, die die vierte Fassung trug, war
nachprüfbar falsch — nachgesehen an allen vier Proben, keine ruft `Schreibgriff::nehmen`. Sie
brauchten `sperrdatei_oeffnen`, und dafür genügen sechs Zeilen im Testziel.

**`Ablage::pfad` öffentlich zu lassen ist ebenfalls richtig, und die Begründung ist nur zur
Hälfte die genannte.** Nachgezählt: 97 Aufrufstellen von `.pfad(` im Baum, 39 davon mit einer
`Datei`-Variante als Argument, die große Mehrheit lesend. Der entscheidende Punkt steht im
Datensatz nicht: `crates/krk-core/tests/` ist eine **eigene Kiste**, `pub(crate)` machte alle
Integrationsproben unübersetzbar, und ein Bibliotheksziel für die Proben zu bauen wäre derselbe
Umbau, an dem die Runde 2 schon einmal hängengeblieben ist. Die Entscheidung ist also nicht nur
vertretbar, sondern ohne Alternative. Die Aussage „Lesen an der Sperre vorbei ist nicht der
Fehler, gegen den die Zusage steht" trägt daneben — sie erklärt aber nicht die elf **Schreib**wege,
die durch `pfad()` weiterlaufen; das ist A2.

---

## Was quer durch den Turn läuft

**Erstens: das Muster der Runde ist um eine Stufe verschoben und nicht weg.** Turn 1 fand
Zählproben, die eine Zusage an eine Schreibweise banden. Turn 2 hat die Nadeln repariert und die
Bauanleitung geschrieben — und in derselben Bewegung drei neue Texte hinterlassen, die mehr
zusagen als ihre Probe hält: „ein anderer Weg besteht nicht" (A1), „diese eine Lücke bewacht
deshalb eine Probe" (A2), „nur die Halterin schreibt die Sitzung hält der Übersetzer" (A5). Das
ist kein Rückfall in die alte Sorte; es ist die neue Sorte derselben Familie. Die drei
Folgerungen im Kopf von `quellbaum.rs` sind die richtige Antwort darauf, und ihre dritte —
Blindheit benennen — ist die, die in diesem Turn am häufigsten übersprungen worden ist.

**Zweitens: die Bauform selbst ist ein Gewinn und soll bleiben.** Der Umstieg von `struct
Pruefordner` auf `impl Drop` neben einem Temporärordner ist der Unterschied zwischen einer Nadel,
die einen Namen zählt, und einer, die eine Sache findet. Er ist gegen das eigene Gegenbeispiel
geprüft. Dasselbe gilt für `aufrufstellen`: die drei Abzüge sind richtig gewählt, und die Probe
über das Werkzeug ist die Vorkehrung, die eine solche Funktion braucht.

**Drittens, und positiv:** dieser Turn hat kein Verhalten verschlechtert. Die zwei Runden 6, auf
die der Auftrag verweist, sind hier nicht wiederholt worden — nachgeprüft an den drei Stellen,
an denen es hätte passieren können: der Wegfall von `sitzung_vormerken()`, der vorgezogene
Kürzelfilter und die Hebung von `getipptes_zeichen`. Alle drei sind verhaltensgleich oder
strikt besser, und alle drei sind gegen den Rumpf nachgerechnet und nicht gegen den Kommentar.

---

## Reihenfolge

**Nichts hält die Runde auf.** Kein Befund ist ein Freigabehindernis. Die Abnahmeliste am Bündel
gilt unverändert; dieser Turn hat ihr nichts hinzugefügt und nichts abgenommen.

**Vor dem nächsten Turn, weil sie Zusagen betreffen, auf die spätere Runden bauen:**

1. A1 und A2 — die zwei Sätze am Sperrstrang. Beide sagen eine Dichtheit zu, die nicht besteht,
   und beide stehen an der Stelle, an der die nächste Runde nachsehen wird.
2. A3 — die Acht, bevor jemand sie zum ersten Mal hochzählt.

**Aufräumen, wenn Zeit ist:** A4, A5 und B1. Alle drei sind ein Absatz.

**Unverändert offen und nicht von diesem Turn zu erledigen:** die acht Datensätze, die der
Auftrag nennt, allen voran `260813-0540_o_die-belegung-wird-weiter-blind-ueberschrieben…`. Es
ist der einzige der sechzehn Befunde aus Turn 1, der stehengeblieben ist, und das ist richtig
so — er verlangt eine Nutzerentscheidung und keine Codeänderung.
