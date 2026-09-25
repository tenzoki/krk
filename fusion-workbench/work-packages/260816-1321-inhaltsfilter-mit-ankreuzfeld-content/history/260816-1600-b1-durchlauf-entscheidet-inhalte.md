# B1: Der Durchlauf entscheidet auch Inhalte

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt B1
**Baumstand vor der Arbeit:** `7283d55`
**Vorbedingungen:** A1 (`5c7f5b9`), C1 (`4a54212`), A2 (`7283d55`)
**Erfüllt:** C1.1, C1.9, C1.10, C1.11, C3.1, C3.3, C3.4, C3.5, C3.6, C3.7, C4.1, C4.2, C4.6, C4.7 (teilweise, siehe unten)

## Was entstanden ist

**`crates/krk-core/src/verzeichnis/durchlauf.rs`** bekommt die zweite
Auftragsart in die vorhandene Maschine, nicht eine zweite Maschine daneben:

- `pub enum Auftragsart { Unterbaum, Inhalt }` — zwei Werte, ohne Auffangzweig.
- `Auftrag` trägt das Feld `art`; Index und Name bleiben, wie sie waren.
- `Durchlauf::starten` nimmt `inhaltsgrenze: Option<u64>` zwischen dem
  Filtertext und der Generation. `None` heißt „der Inhalt zählt bei diesem Lauf
  nicht", `Some(n)` heißt „er zählt, und `n` ist die Grenze".
- `Durchlauf` hält ein zweites geteiltes Kennzeichen neben dem Abbruch,
  `Arc<AtomicU64>`, gelesen über `Durchlauf::zu_gross()`. Kein zweiter Kanal —
  über den Kanal geht weiterhin genau eine `Befundmeldung` je Auftrag.
- `durchlauffaden` verzweigt über `(auftrag.art, inhaltsgrenze)`, vollständig
  und ohne Auffangzweig.
- `datei_entscheiden` ist die eine Übersetzung von `Inhaltsbefund` in die
  Antwort des Durchlaufs: `Traegt` → `Some(true)`, `TraegtNicht` →
  `Some(false)`, `ZuGross` → `Some(false)` und der Zähler steigt,
  `Unentschieden` → `None`.
- `unterbaum_entscheiden` bekommt die Grenze und liest in der
  Kandidatenschleife jede gewöhnliche Datei, deren Name die Folge nicht trägt.
  Die Fallunterscheidung über `kandidat.typ` ist jetzt ein `match` mit drei
  ausgeschriebenen Zweigen statt eines `if`.

**Der Deskriptormangel bleibt unentschieden.** `Inhaltsbefund::Unentschieden`
wird zu `None`, und `None` beendet den Faden ohne Meldung — dieselbe Regel, die
die Runde 10 für den Unterbaum aufgestellt hat, jetzt auch für die einzelne
Datei. Der Zweig ist an keiner Stelle zu einem `TraegtNicht` verkürzt.

**Ein Deskriptor mehr, und nur einer.** `traegt_der_inhalt` öffnet, liest und
gibt frei, bevor der nächste Kandidat drankommt; gehalten werden während eines
Lesens ein Verzeichnisdeskriptor und ein Dateideskriptor, gleich wie tief der
Baum ist. Der Modulkopf sagt es und benennt den Defekt `260815-0211` als das,
was eine Liste offener Dateien zurückholte.

**Die Abbruchgrenze steht an drei Stellen, zwei davon im Unterbaum.** Der
Modulkopf schreibt die Regel neu aus — geprüft wird vor jeder Einheit, die
dauern kann, und das sind seit dieser Runde zwei —, und die Begründung dafür,
dass beim Absteigen nicht geprüft wird (der Ordner mit fünfzigtausend
gewöhnlichen Einträgen), steht wörtlich unverändert.

**`crates/krk-core/src/verzeichnis/mod.rs`**: `Auftragsart` wird
wiederausgeführt, das Bild im Modulkopf zeigt `inhalt` als Zulieferer des
`durchlauf` statt als Anhängsel von `filter`, und der Absatz zum `durchlauf`
nennt die zwei Auftragsarten und die **sechste** Eingabe des Prüfschritts
(vorher: fünfte).

**`crates/krk-ui/src/tabs.rs`** ist nur so weit angefasst, wie der Bau es
verlangt: der Aufrufer übergibt `art: Auftragsart::Unterbaum` und `None`, beides
mit einem Kommentar, der auf D1 verweist. Die Tafel der vier Auftragslagen zieht
dort ein und nicht hier.

## Was die Proben belegen

Neu in `crates/krk-core/tests/verzeichnis.rs`, alle grün:

- `ein_flacher_inhaltsauftrag_liest_die_datei_und_entscheidet_sie` — ein
  Treffer und ein Nichttreffer, beide an Namen, die die Folge nicht tragen.
- `ein_treffer_allein_im_text_entscheidet_den_unterbaum` (C3.1) — derselbe Baum
  zweimal, ohne Grenze unentschieden negativ, mit Grenze getroffen.
- `ein_namenstreffer_im_unterbaum_bleibt_ungelesen` (C3.4) — eine Datei ohne
  Leserecht, deren **Name** passt; `treffer: true` ist der Beleg, dass nicht
  geöffnet wurde.
- `eine_verknuepfung_im_unterbaum_wird_nicht_gelesen` (C3.7) — samt Gegenprobe
  über den echten Ort derselben Datei.
- `ohne_grenze_wird_keine_einzige_datei_geoeffnet` — der Lauf mit
  `inhaltsgrenze: None`, dazu die Erwartung, dass ein Inhaltsauftrag ohne Grenze
  unentschieden bleibt statt still negativ zu werden.
- `eine_zu_grosse_datei_bleibt_ungelesen_und_zaehlt` (C4.6) — flach und im
  Unterbaum, je mit dem Stand des Zählers.
- `ein_deskriptormangel_beim_lesen_laesst_die_datei_unentschieden` (C3.6) — die
  **Kindprobe** unter `ulimit -n 64`, in der Form der Runde 10. Das Kind stellt
  den Mangel her, indem es Deskriptoren nimmt und hält, und misst zuerst, dass
  die Grenze wirklich abgesenkt ist. Der erste Auftrag ist eine Verknüpfung und
  ohne Öffnen entschieden — nur so ist zu sehen, dass der Mangel den Durchlauf
  **ab** dem ersten Öffnen anhält.

**C4.7 ist an keiner Probe vollständig abzulesen**, und der Plan sagt warum: die
Spanne zwischen dem Setzen des Abbruchkennzeichens und dem Ende des Fadens zu
messen bräuchte eine Uhr, und in diesem Weg steht keine. Belegt ist stattdessen
**wo** geprüft wird:
`die_abbruchgrenze_steht_vor_jedem_stapel_und_vor_jeder_datei` schneidet den
Quelltext ohne Kommentarzeilen an `fn unterbaum_entscheiden` und zählt dort
genau zwei `abbruch.load(`, im flachen Zweig davor genau eine und dahinter
keine. Das ist die Aussage der Regel — vor jeder Einheit, die dauern kann — als
Probe und nicht als Diff-Lektüre.

Die vorhandenen Durchlaufproben sind auf die neue Signatur nachgezogen; der
Helfer `einen_auftrag_entscheiden` liefert jetzt Befunde **und** Zählerstand,
`einen_ordner_entscheiden` bleibt die Form ohne Inhaltsfilter.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, 78 Proben in
`tests/verzeichnis.rs` (3 Kindproben als `ignored` geführt und vom Elternteil
gefahren). Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` lief durch.

## Was offen bleibt

D1 baut die Tafel der vier Auftragslagen in `tabs.rs` und lässt die 1 MB aus
`vorschaumodell::TEXTGRENZE` in den Kern reisen; bis dahin startet die
Oberfläche jeden Lauf mit `None`.
