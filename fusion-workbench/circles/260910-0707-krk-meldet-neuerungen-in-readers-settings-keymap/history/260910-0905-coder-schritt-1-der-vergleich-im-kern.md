# Coder-Sitzung: Schritt 1, der Vergleich im Kern

**Date:** 2026-09-10, 260910-0905
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 1
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

### Das neue Modul `crates/krk-core/src/ablage/neuerungen.rs`

Es hält die eingebettete Auslieferungsfassung gegen die Datei des Nutzers und liefert die
Namen der Unterschiede in beide Richtungen. Geschrieben wird nichts.

- `Vergleichsform` — vollständig über `Datei`, ohne Auffangzweig, mit den drei Werten
  `ObersteSchluessel` (`settings.toml`), `Tischfolge { tisch, schluessel }`
  (`readers.toml` als `profil`/`name`, `keymap.toml` als `funktion`/`id`) und `Nicht`.
  `Vergleichsform::namen` zieht aus einer `toml::Table` die Namen in der Reihenfolge der
  Datei; ein Tisch ohne sein Schlüsselfeld fällt heraus.
- `Befund` — was der Vergleich vorgefunden hat: `Verglichen`, `Fehlt`, `Ersetzt`.
- `Neuerungen` — je Datei: welche, voller Pfad, `Befund`, `nur_ausgeliefert`,
  `nur_beim_nutzer`, dazu `traegt_unterschied`.
- `Bestand` — der Ablageordner und eine Zeile je verglichener Datei, in der Reihenfolge
  von `Datei::ALLE`, mit `ordner`, `dateien`, `fuer` und `traegt_unterschied`.
- `erheben(&Zugang)` — läuft über `Datei::ALLE`, überspringt jede Datei mit
  `Vergleichsform::Nicht`, überspringt eine nicht vorhandene Nutzerdatei und hält sonst
  `zugang.laden::<toml::Table>` gegen die eingebettete Fassung.
- `startzeile` und `blatttext` — die zwei reinen Formatierer.

Die eingebetteten Fassungen kommen als `LazyLock<toml::Table>` aus den drei vorhandenen
`AUSLIEFERUNGSTEXT`-Konstanten.

### Die drei Festlegungen des Nutzers

**Die Startzeile nennt nur die Dateien mit einem Unterschied.** `startzeile` filtert über
`!nur_ausgeliefert.is_empty()` und gibt `None`, wenn nichts übrig bleibt. Gezählt wird
dabei die Hinrichtung und nicht beides zusammen: die Zeile kündigt an, was diese Fassung
mitbringt, und ein eigenes Profil des Nutzers ist keine Neuerung dieser Fassung. Die
Gegenrichtung steht im Blatttext.

**Eine nicht vorhandene Nutzerdatei liefert keine Neuerung.** `eine_datei` fragt
`pfad.try_exists()` **vor** dem Laden und gibt `Befund::Fehlt` ohne Namen zurück. Der
Grund für die Reihenfolge steht als Kommentar an der Stelle: für `Zugang::laden` sind eine
fehlende Datei und eine gültige Datei ohne einen einzigen obersten Schlüssel dasselbe
Ergebnis, hier sind sie zweierlei. Ein *Fehler* beim Fragen gilt nicht als „sie fehlt";
dann liegt sie da und ist nur nicht zu erreichen, und das Laden darunter sagt es genauer.

**Die Gegenrichtung füllt sich nur bei `readers.toml`.** Dafür steht eine dritte
vollständige Fallunterscheidung im Modul, `eigene_eintraege_moeglich`. Findet der
Vergleich bei `settings.toml` oder `keymap.toml` einen Eintrag, den die
Auslieferungsfassung nicht kennt, ist das ein Schaden und keine Abweichung: die Datei
trägt `Befund::Ersetzt` und keine der beiden Richtungen. Sie kommt damit wirklich nicht
bis zum Vergleich, statt dass der Modulkopf es nur behauptet.

### Die drei angefassten Bestandsdateien

- `ablage/mod.rs`: `pub mod neuerungen;`, das Modul in der Skizze des Kopfes und ein
  Absatz darüber, was es tut und was es nicht tut.
- `ablage/pfade.rs`: **ein** Absatz im Modulkopf, der auf die vierte je Datei beantwortete
  Frage zeigt und sagt, warum sie nicht dort steht — sonst zählt der Kopf weiter drei und
  ein Neuling glaubt ihm. Die drei dort sind unverändert.
- `tasten/belegung.rs`: `AUSLIEFERUNGSTEXT` ist `pub` und trägt den Grund als
  Doc-Kommentar. Der Vergleich braucht den ausgelieferten **Text** und nicht die daraus
  gebaute `Belegung`: `Belegung::bauen` nimmt jede ungenannte Funktion unbelegt hinzu und
  machte den Unterschied damit unsichtbar.

## Eine Abweichung vom Wortlaut des Plans, und warum

Der Plan schreibt für `erheben`: „überspringt eine nicht vorhandene Nutzerdatei und sonst
`zugang.laden::<toml::Table>` gegen die eingebettete Fassung hält". Zwei Fälle stehen
daneben, die dieser Satz nicht nennt und die ohne eigene Antwort still falsch geworden
wären:

1. **Ein Laden mit `Ersetzung`.** Kommt aus dem Laden eine `Ersetzung` — kaputtes TOML,
   keine gültige UTF-8-Folge —, dann ist der Wert die leere Vorgabe, und ein Vergleich
   dagegen meldete jeden ausgelieferten Eintrag als Neuerung. Das ist derselbe Fehler wie
   bei der fehlenden Datei, nur an einer anderen Ursache. `Befund::Ersetzt`, keine Namen.
   Der Plan meint diesen Fall unter „eine beschädigte Datei ist beim zweiten Lesen
   beiseitegelegt"; im Baum bleibt sie liegen, das Zur-Seite-Legen ist eine Kopie.
2. **Ein unbekannter Eintrag in `settings.toml` oder `keymap.toml`.** Über `toml::Table`
   gelesen ist eine solche Datei tadellos, und die Gegenrichtung hätte sich gefüllt —
   entgegen der Festlegung des Nutzers, dass sie das bauartbedingt nicht tut. Deshalb
   `eigene_eintraege_moeglich` und `Befund::Ersetzt` (siehe oben).

Beide Antworten machen die Zeile über die Dateien wahr, mit denen KRK **arbeitet**; eine
Datei, die ihr Leser abgewiesen hat, ist im Betrieb durch den Auslieferungszustand
ersetzt, und eine Aussage über ihren Inhalt hilft niemandem.

## Was der Übersetzer hier nicht hält

`Vergleichsform::fuer` und die private `auslieferung` sind je für sich vollständig über
`Datei`; dass sie **dieselben** Dateien bejahen, hält keine der beiden. Eine achte
Ablagedatei mit einer Vergleichsform, aber ohne eingebettete Fassung fiele still aus dem
Bestand. Gehalten wird die Paarung deshalb von der Probe
`jede_verglichene_ablagedatei_hat_eine_eingebettete_fassung`, und `erheben` trägt daneben
ein `debug_assert!` an der Stelle, an der es aufflöge.

## Proben (`crates/krk-core/tests/ablage.rs`, neuer Abschnitt am Ende)

Alle laufen gegen einen `Pruefordner` der Fassung dieser Kiste
(`crates/krk-core/tests/gemeinsam/mod.rs`), keine vierte Fassung, und keine fasst das
echte Benutzerverzeichnis an.

| Probe | Abnahmekriterium |
|---|---|
| `eine_nutzerdatei_die_es_nicht_gibt_liefert_keine_neuerung` | je verglichener Datei, `keymap.toml` namentlich |
| `jede_verglichene_ablagedatei_hat_eine_eingebettete_fassung` | die Paarung der zwei Fallunterscheidungen |
| `eine_nutzerdatei_wie_die_auslieferungsfassung_liefert_keine_neuerung` | Gleichstand meldet nichts |
| `ein_entferntes_profil_steht_in_der_hinrichtung_und_ein_eigenes_in_der_gegenrichtung` | beide Richtungen an `readers.toml` |
| `eine_keymap_ohne_eine_ausgelieferte_id_liefert_genau_diese_id` | genau diese `id` |
| `eine_settings_ohne_terminal_liefert_genau_diesen_schluessel` | genau dieser Schlüssel |
| `ein_unbekannter_eintrag_macht_settings_und_keymap_beschaedigt` | **warum** die Gegenrichtung dort leer bleibt |
| `die_startzeile_nennt_jede_datei_mit_unterschied_und_den_ordner` | Wortlaut, Zahl je Datei, `~`-Form des Ordners |
| `der_blatttext_nennt_jede_datei_mit_vollem_pfad_und_beide_richtungen` | Wortlaut des zweiten Formatierers |

Die Probe zum unbekannten Eintrag hält die zwei Stellen, an denen die Bauart wirklich
hängt: sie prüft zuerst, dass `einstellungen::laden` und `belegung::laden` die Datei als
beschädigt melden (`deny_unknown_fields`, `Belegungsfehler::UnbekannteFunktion`), und erst
danach, dass der Vergleich sie deshalb auslässt. Wer eine der beiden aufhebt, lässt sie rot
werden, statt die Begründung im Modulkopf still falsch zu machen. Sie hält daneben den
dritten Satz des Blatttextes, „Diese Datei ist beschädigt und wird deshalb nicht
verglichen." — wörtlich das Wort, das `Grund::beschreibung` dem Nutzer in der Statuszeile
schon hinschreibt; eine Sache, eine Formulierung.

Die zwei Wortlautproben reichen das Benutzerverzeichnis als Argument herein — der
Prüfordner steht unter `/tmp`, und `gekuerzt_fuer_anzeige` macht daraus `~/<name>`. So ist
die `~`-Form belegt, ohne das echte Benutzerverzeichnis anzufassen.

Drei Hilfsfunktionen stehen dabei: `verglichene_dateien` als abgeleitete Frage über
`Vergleichsform` statt einer zweiten Liste, `auslieferungstext` als vollständige
Fallunterscheidung über `Datei`, und `ohne_den_ersten_block`, das einen `[[profil]]`- oder
`[[funktion]]`-Block **am Text** herausschneidet. Der Schnitt am Text und nicht über einen
Rundlauf durch `toml::to_string` ist begründet: das Ausschreiben sortierte die Schlüssel
jedes Tisches neu und schöbe womöglich einen Wert hinter eine Untertabelle, was TOML nicht
zulässt. Gesucht wird dabei die Kopfzeile `\n[[profil]]\n` und nicht die Zeichenfolge —
beide Auslieferungsfassungen erklären ihren eigenen Aufbau im Kommentarkopf und schreiben
`[[profil]]` dort aus; die erste Fassung der Probe fiel genau darauf herein.

## Was ausdrücklich nicht angefasst ist

`crates/krk-ui/`, `xtask/`, `resources/`, die Werkbank. Die Schritte 2 bis 8 des Plans
sind unberührt; insbesondere gibt es weder einen Merker der gemeldeten Fassung noch einen
Aufruf von `erheben` beim Start noch ein Kommando und kein Blatt. `Kommando`,
`Kommando::KENNUNGEN` und `resources/default-keymap.toml` sind nicht angefasst.

CLAUDE.md ist nicht angefasst; der Abgleich gehört `/fusion:cleanup --only claude-md`.

## Prüfung

- `cargo test -p krk-core` — exit 0
- `cargo clippy -p krk-core --all-targets -- -D warnings` — exit 0
- `cargo fmt --all --check` — exit 0
- `RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps` — exit 0

Der Dokumentationslauf war beim ersten Mal rot: drei Doc-Verweise trugen ein ausdrückliches
Ziel, das ihre eigene Beschriftung schon auflöst (`rustdoc::redundant_explicit_links`, unter
`-D warnings` ein Fehler). Die drei Ziele sind entfernt, die Verweise bleiben Verweise. Das
ist genau die Sorte Befund, wegen der `make check` seit dem 260905 den Dokumentationslauf
als fünftes Kommando führt.
