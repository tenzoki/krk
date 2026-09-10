# Coder-Sitzung: Schritt 6, das Blatt auf Abruf

**Date:** 2026-09-10, 260910-1520
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 6
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

Das Blatt auf Abruf steht. Es zeigt je verglichener Ablagedatei ihren **vollen Pfad**, den
Unterschied in beide Richtungen und einen Satz darüber, was ein Unterschied bei dieser
Datei kostet. Es geht auch dann auf, wenn es keinen Unterschied gibt — das ist der
Unterschied zur Abschlussliste der übersprungenen Einträge, wo ohne Einträge kein Blatt
aufgeht: dort meldet KRK ungefragt, hier hat der Nutzer gefragt.

Einen Rufer hat es noch nicht. Der Ausführungszweig beim Anwendungsdelegierten und der Weg
über das Hauptmenü sind Schritt 9.

### `crates/krk-ui/src/appkit/blaetter/neuerungen.rs` (neu)

Gebaut nach dem Vorbild `blaetter/uebersprungen.rs` und `blaetter/startmeldungen.rs`:

- `FRAGE` — die Kopfzeile, „Ihre Ablagedateien und was diese Fassung mitbringt". **Eine
  Zeichenkette und nicht zwei.** Eine Überschrift, die von Neuerungen spräche, wäre falsch,
  sobald es keine gibt, und genau dann geht dieses Blatt trotzdem auf; ob es einen
  Unterschied gibt, sagt der Text darunter Datei für Datei.
- `schaltflaechen()` — eine reine Funktion, genau eine Schaltfläche „Schließen" auf der
  Eingabetaste, `Wirkung::Liegenlassen`. Der Grund ist hier stärker als bei den zwei
  Vorbildern: dieses Blatt zeigt Dateien, die KRK ausdrücklich **nicht** schreibt; eine
  zweite Schaltfläche „übernehmen" wäre die Zusage der Runde in ihr Gegenteil verkehrt.
- `zeigen(mtm, fenster, bestand, fertig)` — `Blatt::mit_schaltflaechen`,
  `erlaeuterung_setzen` mit `krk_core::ablage::neuerungen::blatttext(bestand)`,
  `zeigen_mit_wahl`. **Der Text kommt fertig aus dem Kern**; diese Datei setzt nichts
  zusammen, dieselbe Arbeitsteilung wie bei `uebersprungen` und `uebersprungenliste`.
- Der Modulkopf trägt den Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`
  und nennt `NSWindow` und `MainThreadMarker` namentlich; die zwei Proben in
  `crates/krk-core/tests/baum.rs` verlangen beides.

**`pub fn zeigen` trägt ein `#[expect(dead_code, reason = …)]` und kein `#[allow]`.** Unter
`-D warnings` ist die tote Funktion sonst ein Fehler, und `make check` liefe rot. Das
`#[expect]` hat ein Ablaufdatum: sobald Schritt 9 den Ausführungszweig baut, wird die
Ausnahme selbst zur Warnung und muss fallen. Ein `#[allow]` bliebe ohne Ablaufdatum stehen —
die Lehre aus dem Kopf von `krk-ui/src/editormodell.rs`, der genau diesen Unterschied
ausschreibt. Die Ausnahme steht dabei nur an `zeigen`: `FRAGE` und `schaltflaechen` gelten
darüber wieder als erreichbar, und im Prüfziel sind sie es ohnehin.

### `crates/krk-ui/src/appkit/blaetter/mod.rs`

Die Modulzeile `pub mod neuerungen;` und die Zählung im Kopf: „elf" wird „zwoelf", das
auswärtige Blatt der Belegungsansicht wird vom zwölften zum dreizehnten, und „elf Blaetter
… elf Stellen" wird „zwoelf". Der neue Satz nennt das Blatt als das einzige, das auch ohne
Befund aufgeht.

## Abweichung von der Dateiliste des Schrittes

**Zwei Dateien im Kern sind dazugekommen, und die Changes des Schrittes selbst verlangen
sie.** Der Schritt schreibt vor, dass „der Text aus `neuerungen::blatttext`" je Datei den
Preissatz führt und dort, wo die Gegenrichtung bauartbedingt leer ist, den Grund in einem
Halbsatz nennt. Beides gehört in `blatttext`, und `blatttext` steht seit Schritt 1 in
`crates/krk-core/src/ablage/neuerungen.rs`, dessen Changes die zwei Formatierer nur benannt
und nicht ausgeschrieben hatten.

Die Oberfläche kann keines von beiden nachreichen. Den Preissatz **je Datei** nicht, weil
`blatttext` die Absätze schon zusammengesetzt zurückgibt; ein angehängter Anhang „was es je
Datei kostet" wäre eine zweite Aufzählung derselben drei Dateien. Den Halbsatz erst recht
nicht, weil er den Gedankenstrich ersetzt, den `namenszeile` in derselben Zeile setzt. Ein
zweiter Formatierer in `krk-ui` daneben wäre genau die Doppelung, die Schritt 5 an der
Kürzung gerade beseitigt hat.

Die Testing Strategy des Plans sieht dieselbe Aufteilung vor: „der Kern trägt die Last, weil
er ohne AppKit prüfbar ist." Drei der vier Abnahmekriterien dieses Schrittes sind Aussagen
über den **Text** und nicht über AppKit; sie stehen deshalb in
`crates/krk-core/tests/ablage.rs`. Das vierte, der Bauplan, steht in der neuen UI-Datei.

Der Planschritt trägt die Abweichung als Nachtrag.

### `crates/krk-core/src/ablage/neuerungen.rs`

- `preis(Datei) -> Option<&'static str>` — **die vierte vollständige Fallunterscheidung
  dieses Moduls**, ohne Auffangzweig wie die drei daneben. Sie beantwortet eine eigene
  Frage: nicht, was ein Eintrag ist (`Vergleichsform::fuer`), nicht, wogegen er gehalten
  wird (`auslieferung`), nicht, ob ein unbekannter zulässig ist
  (`eigene_eintraege_moeglich`), sondern was sein Fehlen den Nutzer kostet. Die drei
  Antworten sind drei verschiedene:
  - `readers.toml`: „Ein Profil, das Ihre Datei nicht führt, kostet die Zusammenfassung für
    diesen Ort: die Vorschau zeigt dort die Metadaten." Belegt vom Kopf von
    `ablage/leseprofile.rs`, „ohne Profil zeigt jeder Ordner die Metadatenanzeige".
  - `settings.toml`: „Ein Schlüssel, den Ihre Datei nicht führt, kostet allein den
    erklärenden Kommentarblock; den Wert selbst nimmt KRK aus der Auslieferungsfassung."
    Belegt von `Einstellungen::aus_datei` und vom Abschnitt „Ein fehlendes Feld kommt aus
    der Auslieferungsfassung".
  - `keymap.toml`: „Eine Funktion, die Ihre Datei nicht führt, kostet ihre ausgelieferten
    Tastenkombinationen; über das Hauptmenü bleibt sie erreichbar." Belegt von
    `Belegung::bauen`, das eine fehlende Funktion unbelegt hinzunimmt — derselbe Befund, den
    der Ausführende von Schritt 8 an der Menüausgabe dieses Geräts beobachtet hat.
- `gegenrichtung(Datei, &[String]) -> String` — die Zeile „Nur in Ihrer Datei". Wo die
  Richtung leer **und** bauartbedingt leer ist, hängt sie den Grund an:
  „— (diese Datei kann keine eigenen Einträge führen; einen unbekannten Eintrag weist KRK
  als beschädigt ab)". Bei `readers.toml` bleibt der Gedankenstrich allein stehen, denn dort
  heißt er wirklich „geprüft und nichts gefunden". Der Unterschied ist der ganze Zweck: eine
  leere Richtung mit Grund und eine leere Richtung ohne Fund sind zweierlei.
- `blatttext` ruft beide. **Der Preissatz steht nur unter `Befund::Verglichen`**, und das
  ist keine Sparsamkeit: wer keine `keymap.toml` hat, arbeitet mit der Auslieferungsbelegung
  und verliert nichts, und wer eine beschädigte hat, hat das beim Start gelesen und arbeitet
  ebenfalls auf dem Auslieferungszustand. Ein Preis fällt erst an, wo es einen Unterschied
  geben kann.
- Der Modulkopf bekommt den Absatz, dass das Blatt den Grund hinschreibt statt einen
  Gedankenstrich stehen zu lassen; der Doc-Kommentar von `blatttext` nennt den Preissatz und
  seine Einschränkung auf `Verglichen`.

Der Wortlaut trägt durchweg Umlaute (Naht vom 260907), die Bezeichner und Kommentare die
Umschrift.

### `crates/krk-core/tests/ablage.rs`

Drei neue Proben, alle gegen einen `Pruefordner` der Fassung dieser Kiste
(`crates/krk-core/tests/gemeinsam/mod.rs`), keine vierte:

| Probe | Abnahmekriterium |
|---|---|
| `ohne_einen_einzigen_unterschied_nennt_das_blatt_die_drei_vollen_pfade` | ohne Unterschied steht das Blatt und zeigt die vollen Pfade |
| `jede_verglichene_ablagedatei_nennt_ihren_preis` | die drei Sätze im Wortlaut, dazu der Halbsatz der Gegenrichtung |
| `eine_namensliste_jenseits_der_kuerzungsgrenze_endet_mit_und_n_weitere` | die Kürzung greift im Blatt |

Die Preisprobe läuft über `verglichene_dateien()` und nicht über eine Liste von dreien: eine
vierte verglichene Datei ohne Preissatz lässt sie rot werden, statt still an einer Datei
weniger zu prüfen. Damit hält sie dieselbe Paarung, die
`jede_verglichene_ablagedatei_hat_eine_eingebettete_fassung` für die eingebetteten Fassungen
hält — der Übersetzer sieht sie in beiden Fällen nicht.

Die Kürzungsprobe legt eine `keymap.toml` mit genau **einer** ausgelieferten Kennung hin und
liest die erwartete Zahl aus der Auslieferungsbelegung selbst, statt eine Zahl hinzuschreiben:
die Belegung wächst mit fast jeder Runde. Die eine Kennung ist eine ausgelieferte und keine
erfundene — eine unbekannte machte die Datei beschädigt, und der Vergleich fände gar nicht
statt.

Die vorhandene Probe `der_blatttext_nennt_jede_datei_mit_vollem_pfad_und_beide_richtungen`
ist unverändert grün geblieben: sie sucht `"Nur in Ihrer Datei: —"`, und der neue Halbsatz
steht dahinter.

## Ein roter Lauf unterwegs

Die erste Fassung der Prosa an `preis` sagte „Die vier Dateien ohne Vergleich haben keinen
Preis". `keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`
(`crates/krk-core/tests/baum.rs`) hält jede Zahl vor „Dateien" unter `ablage/` gegen die acht
des Baumes und wurde rot. Heraus ist die Zahl aus dem Satz und nicht die Probe: der Satz
heißt jetzt „Eine Datei ohne Vergleich hat keinen Preis". Das ist genau der Dienst, für den
diese Probe seit Schritt 3 dasteht.

## Was ausdrücklich nicht angefasst ist

`crates/krk-ui/src/appkit/anwendung.rs` — der Ausführungszweig und das Halten des `Bestand`
sind Schritt 9 beziehungsweise Schritt 4. `resources/`, `xtask/`, `README.md`, `HowTo.md`.
Von der Werkbank allein der Planschritt und dieses Protokoll.

CLAUDE.md ist nicht angefasst; der Abgleich gehört `/fusion:cleanup --only claude-md`. Zwei
Stellen dort sind von dieser Runde betroffen und heute schon knapp: die Aufzählung der
Ablagedateien und die Beschreibung der Aufzählung `Kommando`. Neu dazu kommt nichts, was
CLAUDE.md heute zählt — die Blätter zählt sie nicht.

## Prüfung

- `make check` — exit 0, alle fünf Kommandos in ihrer Reihenfolge („alle fuenf gruen").
