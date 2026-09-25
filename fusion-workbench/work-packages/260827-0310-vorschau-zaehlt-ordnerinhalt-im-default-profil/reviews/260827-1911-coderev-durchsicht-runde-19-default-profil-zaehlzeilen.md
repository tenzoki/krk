# Durchsicht der Runde 19: die Vorschau zählt den Ordnerinhalt im eingebauten Default-Profil

**Reviewed-range:** `a5c7a46..d444879`
**Not-opened:** none
**Sender:** coderev
**Massstab:** Spec `planning/260827-0646_*_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (C1 bis C4, Festlegungen A1 bis A7, Constraints 1 bis 6), Plan `planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (Schritte 1 bis 8), dazu die Regeln aus `CLAUDE.md` zu `#[must_use]`, zum Deskriptorhaushalt und zu den vollständigen Fallunterscheidungen ohne Auffangzweig.
**Übernommene Not-opened-Liste einer vorigen Durchsicht:** keine; dies ist die erste Durchsicht der Runde (`carried=(not recorded)`).
**Gelesen:** alle elf geänderten Dateien des Bereichs `crates`/`resources` im vollen Diff, dazu die ungeänderten Nachbarstellen `crates/krk-core/src/leseprofil/bausteine.rs:395-600` (`Lauf`, Merkstelle, `zielordner`), `crates/krk-core/src/leseprofil/erkennung.rs:15-100`, `crates/krk-ui/src/vorschaumodell.rs:700-760` (`laden`), `resources/default-readers.toml:1-60,219-260`.
**Gemessen, nicht nur gelesen:** `make check` auf `d444879` (Arbeitsbaum an `crates`/`resources`/`xtask` sauber): alle vier Kommandos grün, Exit 0. Kein Abnahmelauf am Bündel: der verlangt KRK im Vordergrund und ist als Schritt 8 vom Nutzer gefahren.

## Summary

Der Bau tut, was Spec und Plan verlangen, und an jeder der sechs Stellen, auf die die Dispatch-Anweisung besonders zeigt, hält er: die drei neuen Fallunterscheidungen sind vollständig ohne Auffangzweig, der Rückfallweg läuft im selben `Lauf` und kostet nachweislich einen Leselauf in allen vier geprüften Profilsätzen, die A4-Frage steht mit `symlink_metadata` am ausgewählten Pfad, `UeberGrenze` steht vor der `versteckt`-Frage, kein neuer Öffner ist entstanden, und jede neue öffentliche Funktion trägt `#[must_use]`. Zwei Befunde, beide Low, beide Prosa, die den Stand vor dieser Runde beschreibt: drei Sätze im Kommentarteil der Auslieferungsfassung und zwei Stellen im Modulkopf und Doc-Kommentar von `erkennung.rs`. Kein Befund hält den Abschluss der Runde auf.

## Totals

Critical 0 / High 0 / Medium 0 / Low 2.

## Prüfpunkt 1: Vollständigkeit ohne Auffangzweig bei `Wert`, `Auskunft`, `Typdatei`

**Hält an allen drei.**

- `Wert` (`crates/krk-core/src/leseprofil/mod.rs:748`) trägt sieben Werte — `Zahl`, `ZahlMitVersteckten`, `UeberGrenze`, `Vorhanden`, `Text`, `Titel`, `Nicht`, nachgezählt mit `awk '/^pub enum Wert/,/^}/'` —, und der Doc-Kommentar sagt korrekt „ein achter Wert hält die Anzeige an". `Wert::als_text` (`mod.rs:811-817`) hat den siebten Zweig `format!("{zahl} ({versteckt})")` und keinen `_`.
- `Auskunft` (`mod.rs:645`) trägt zwei Werte; der eine Rufer im Programm, `vorschaumodell::laden` (`crates/krk-ui/src/vorschaumodell.rs:748-759`), verzweigt über `Some(Erkannt)`, `Some(Default)`, `None` ohne Auffangzweig.
- `Typdatei` (`crates/krk-core/src/leseprofil/datei.rs:282`) mit `#[serde(rename_all = "lowercase")]`; die Zuordnung `typ` (`datei.rs:575-581`) nennt alle drei Werte plus `None` und keinen `_`. Ein vierter Wert von `Typ` hält den Bau dort an, wie der Plan zusagt.

## Prüfpunkt 2: ein Leselauf, keine zusätzliche Öffnung auf dem Rückfallweg (C4.1 bis C4.4)

**Hält, und zwar durch die Bauart und nicht durch eine Sonderregel.**

`zusammenfassen_gezaehlt` (`bausteine.rs:281-308`) legt **einen** `Lauf` an und rechnet beide Zweige darin: `lauf.zeilen_rechnen(profil)` für das erkannte, `lauf.zeilen_rechnen(defaultprofil())` für das Default-Profil. Die drei Default-Zeilen tragen `Ortsangabe::wurzel()` (`defaultprofil.rs:557`); `zielordner` liefert für leere Teile `self.wurzel.to_path_buf()` (`bausteine.rs:548-550`), also exakt den `Ort::Einer(wurzel)`, unter dem `Lauf::stand` (`bausteine.rs:436-439`) die Lesung des zweiten Erkennungsdurchgangs merkt. Damit findet die erste Zählzeile den Stand der Erkennung vor, und die zweite und dritte den der ersten.

Belegt von `die_drei_zaehlzeilen_kosten_einen_leselauf_und_keine_oeffnung_auch_nach_der_erkennung` (`crates/krk-core/tests/leseprofil.rs`), vier Profilsätze, jeder auf `leselaeufe() == 1` und `oeffnungen() == 0` gehalten; die Fälle „Kennzeichendatei, die nicht trifft" und „drei Profile gemischt" sind die, in denen die Erkennung den Lauf bezahlt hat. Über der Schranke hält `ueber_der_schranke_sagen_die_drei_zeilen_mindestens_…` ebenfalls `leselaeufe() == 1` (C4.4).

`ist_selbst_ein_verzeichnis` (`bausteine.rs:321-323`) ist ein `lstat(2)` ohne Deskriptor und ohne Öffnung; er geht in keinen Haushaltswert ein. Kein neuer Rufer von `ohne_warten_oeffnen` ist entstanden: `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` liefert dieselben Stellen wie vor der Runde (Textwege, Packen, Entpacken, `Schwungleser::oeffnen`).

## Prüfpunkt 3: `symlink_metadata` am ausgewählten Pfad (A4, C1.7)

**Hält.** `ist_selbst_ein_verzeichnis(ordner)` fragt am **übergebenen** Pfad, nicht an `wurzel` (`bausteine.rs:301-306`): `wurzel` ist nach `canonicalize` immer ein Verzeichnis, und die Frage lautet gerade, ob der Nutzer eine Verknüpfung darauf gewählt hat. Belegt von `eine_verknuepfung_auf_einen_ordner_bekommt_keine_zaehlzeilen` (`tests/leseprofil.rs`) in beiden Richtungen: `None` für `auf-ordner` und `auf-datei`, `Auskunft::Default` für das Ziel `planning`, und `Auskunft::Erkannt` für dieselbe Verknüpfung unter einem Pfadmuster — der Beleg, dass das `None` aus dem Rückfallzweig kommt und nicht aus der C2.6-Prüfung. Die Probe `eine_verknuepfung_und_eine_datei_tragen_keine_zaehlzeile` in `vorschaumodell.rs` hält dasselbe auf der `Inhalt`-Seite.

**Beobachtung, kein Befund:** die Frage wird auf dem Weg zweimal gestellt. `vorschaumodell::laden` (`vorschaumodell.rs:714`) hat den Eintrag schon mit `symlink_metadata` befragt und kennt `Typ::Verknuepfung`, bevor es `zusammenfassen` ruft; der Kern fragt auf dem Rückfallweg ein zweites Mal. Der Plan hat das bewusst so entschieden (Abschnitt 3: die Zusage steht im Kern und nicht am Zweig eines Rufers, aus demselben Grund wie C2.6). Ein Systemaufruf, kein Deskriptor, keine Haushaltszahl; ich trage es nicht als Defekt ein.

## Prüfpunkt 4: `UeberGrenze` unabhängig von `versteckt` (C2.10)

**Hält.** `zaehlen` (`bausteine.rs:796-819`) fragt `stand.abgeschnitten` **vor** `versteckt`; die Klammer entfällt durch die Reihenfolge der Zweige, wie der Plan es beschreibt, und nicht durch eine zweite Regel. Belegt am Wert und am Text von `ueber_der_schranke_…`: `Wert::UeberGrenze(2000)`, `UeberGrenze(0)`, `UeberGrenze(0)` mit dem Wortlaut von `Wert::als_text` und ohne zweite Klammer.

## Prüfpunkt 5: `#[must_use]`

**Hält.** Neu und öffentlich: `zeilen_als_text` (`mod.rs:674`) und `defaultprofil` (`defaultprofil.rs:571`), beide tragen es; `zusammenfassen` und `zusammenfassen_gezaehlt` behalten es mit angepasstem Text. Die neuen privaten Funktionen `Lauf::zeilen_rechnen` (`bausteine.rs:613`) trägt es; `trifft` und `ist_selbst_ein_verzeichnis` sind `bool`-Prädikate ohne, wie die vorhandenen Prädikate derselben Datei (`innerhalb`, `zielordner`). `make check` fährt clippy über alle Ziele und ist grün.

## Prüfpunkt 6: die zwei vom Coder gemeldeten Abweichungen in Schritt 5

**Beide vertretbar und im Baum ehrlich dokumentiert.**

- **C3.7 nur unter `crates/*/src`.** `genau_drei_dateien_lesen_das_kennzeichen_versteckt_und_fragen_nach_dem_typ` (`tests/baum.rs`) filtert `name.contains("/src/")`. Der Doc-Kommentar der Probe sagt, warum: eine Abnahmeprobe unter `tests/` stellt beide Fragen an einen gelesenen Eintrag und gruppiert nichts. C3.7 sagt „im Baum"; die Einschränkung ist die einzige, die die Probe nicht rot macht, ohne dass ein zweiter Zählweg entstünde. Der Kommentar nennt außerdem ausdrücklich, was die Nadel nicht sieht (Musterbindung, `matches!`). Keine Nachbesserung nötig.
- **Der gerenderte „sechs plus drei"-Text nur strukturell.** `die_zaehlzeilen_folgen_in_metadaten_text_auf_die_zeile_typ` (`vorschau.rs`) hält am Quelltext: genau ein Rufer von `zeilen_als_text` außerhalb des Prüfmoduls, im Rumpf von `metadaten_text`, und die Formatzeile endet auf `Typ: {}{}`. Die Textseite braucht `NSByteCountFormatter` und `NSDateFormatter`, also eine Instanz, und dieses Prüfmodul behauptet den Hauptfaden nicht (`CLAUDE.md`, „`krk-ui` hat kein Bibliotheksziel"). Was die Probe nicht sieht, deckt Schritt 8, und der ist gefahren. Vertretbar.

## Prüfpunkt 7: C3.4 — kein Profilblock der Auslieferungsfassung geändert

**Hält.** `git diff a5c7a46..d444879 -- resources/default-readers.toml` fügt 39 Zeilen hinzu, alle mit `#`; kein `[[profil]]`- oder `[[profil.zeile]]`-Block ist berührt. Die Probe `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` (`crates/krk-core/src/ablage/leseprofile.rs`) schneidet je Zeile hinter dem ersten `#` ab, bevor sie sucht, und hält daneben am geprüften `Profile`-Wert, dass keine Zählung `typ: Some(_)` oder `versteckt: true` trägt — die zweite Hälfte ist die tragende, weil sie den Deserialisierer und nicht eine Nadel fragt.

## Findings by theme

### Prosa, die den Stand vor der Runde 19 beschreibt

**F1 (Low) — Drei Sätze im Kommentarteil von `resources/default-readers.toml` widersprechen dem neuen Abschnitt derselben Datei.**

- `:18-19`: „Trifft keines zu, bleibt die Metadatenanzeige, wie sie war." — seit `5e506e6` treten drei Zählzeilen unter sie.
- `:229`: „3. Hat auch das nicht getroffen: die gewohnte Metadatenanzeige." — dritter Schritt unter „Welches Profil gewinnt"; der Gewinner ist seit dieser Runde das Default-Profil, und der Abschnitt darunter (`:236-252`) sagt genau das.
- `:43-47`: die erste Reichweite unter „Was ein Schreibfehler kostet" nennt „ein Wert für `zeigt`, den es nicht gibt", aber nicht den unbekannten Wert für `typ` und den Nicht-Wahrheitswert für `versteckt`, die seit `9f91f92` dieselbe Reichweite haben (`datei.rs:49-57` zieht das nach, die Auslieferungsfassung nicht).

C3.9 und C3.10 sagen zu, dass ein Nutzer, der **allein diese Datei** liest, die Auskunft bekommt; drei Sätze darin geben ihm die alte. Das ist ein Defektdatensatz und keine Notiz: der Nebenbefund des Ontocoders nennt zwei der drei Stellen, die dritte kommt hier dazu. Umfang: Kommentarzeilen, kein Block; Executor `ontocoder`. Die Probe `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` bleibt davon unberührt, weil sie hinter `#` abschneidet. Datensatz: `issues/260827-1911_o_drei-saetze-im-kommentarteil-der-auslieferungsfassung-beschreiben-den-stand-vor-der-runde-19.md`.

**F2 (Low) — `crates/krk-core/src/leseprofil/erkennung.rs` sagt an zwei Stellen, `None` heiße „die heutige Metadatenanzeige".**

- `:24` im Ablaufbild des Modulkopfs: „sonst: die heutige Metadatenanzeige".
- `:94-96` im Doc-Kommentar von `erkennen`: „`None` als Rückgabe heißt: kein Profil greift, und die Vorschau zeigt die heutige Metadatenanzeige (C2.5). Das ist derselbe Zweig, den sie ohne diese Runde ohnehin genommen hätte."

Seit `bf3a91d` tritt auf `None` der Rückfallzweig in `zusammenfassen_gezaehlt` ein (`bausteine.rs:300-305`): Default-Profil für ein Verzeichnis, `None` nur noch für eine Verknüpfung. Die Nachbarn `bausteine.rs` (Modulkopf, `:11-17`) und `mod.rs` (Ablaufbild, `:22-31`) sind nachgezogen, `erkennung.rs` nicht. Was `erkennen` selbst tut, ist unverändert und richtig beschrieben; falsch ist nur, was es über seinen Rufer sagt. Executor `coder`. Datensatz: `issues/260827-1911_o_erkennung-rs-sagt-none-heisse-die-heutige-metadatenanzeige-und-das-ist-seit-der-runde-19-der-rueckfallzweig.md`.

## Cross-cutting observations

- **Die zwei Befunde sind ein Muster:** derselbe Satz „trifft kein Profil, bleibt die Metadatenanzeige" stand an vier Stellen im Baum (`bausteine.rs`, `mod.rs`, `erkennung.rs`, `default-readers.toml`); zwei sind nachgezogen, zwei nicht. Wer F1 und F2 behebt, hat alle vier. Eine fünfte Stelle habe ich mit `grep -rn -i 'Metadatenanzeige' crates/*/src resources README.md` nicht gefunden; `README.md:44-63` beschreibt allein die Übernahme neuer Leseprofile und ist unberührt.
- **Die Reihenfolge Erkennung vor A4-Frage** (`bausteine.rs:292-305`) bedeutet, dass eine Verknüpfung auf einen Ordner bei einem Profilsatz mit Kennzeichendateien den Ordner liest und dann `None` liefert. Das ist seit der Runde 16 so und nicht neu; umkehren lässt es sich nicht, weil ein Pfadmuster die Verknüpfung erkennen darf (die Probe hält genau das). Kein Befund, festgehalten, damit die nächste Durchsicht es nicht für einen hält.
- **`Inhalt::Metadaten` als Strukturwert statt achter `Inhalt`-Wert** (`vorschaumodell.rs:285-300`) hat sich bewährt: die drei vollständigen Fallunterscheidungen über `Inhalt` (`zeigt_dateitext`, `einzufaerben`, die Anzeige) brauchten allein `{ .. }` und keine Antwort auf eine Frage, die überall gleich lautet.

## Recommended sequencing

Kein Release-Blocker. F1 und F2 sind Aufräumarbeit mit je einem Executor und lassen sich in einer Behebungssitzung zusammen erledigen; keiner der beiden hält den Abschluss der Runde 19 auf, und keiner ändert eine Probe oder einen Wert.
