# Nachdurchsicht des Archivpakets, `8c8579af..c9651443`

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `8c8579af..c9651443`
**Not-opened:** `crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/tests/sprache.rs`, `fusion-workbench/orchestrator-events.jsonl`, `261017-1923-*.md` und `261017-1923-*.record.json` der dreizehn Issues, `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.evidence.json`, `261017-1029-zip-und-gz-wie-ordner-im-dateifenster.record.json`
**Domain:** code
**Work-item:** 261017-1029-zip-und-gz-wie-ordner-im-dateifenster
**Maßstab:** die dreizehn Issues und der Entscheid der Durchsicht `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`; dazu `HowTo.md` und `CLAUDE.md` gegen den Code
**Urteil:** revise (allein zwei Low-Befunde, kein Blocker)

## Zusammenfassung

Alle dreizehn Befunde und der Entscheid sind im Code behoben. Ich habe keinen neuen Weg zu Datenverlust gefunden und keinen neuen Zugriff auf dem Hauptfaden.

Offen bleiben zwei Low-Befunde:

- `CLAUDE.md` nennt eine Funktion, die es nicht gibt, und eine Zahl von Abweisungen, die nicht mehr stimmt.
- Ein Archivtab, der über eine Verknüpfung in einen anderen Ordner betreten wurde, sieht Änderungen von außen nicht.

Das Urteil heißt „revise“, weil beide Befunde einen Fix verlangen. Für die Auslieferung blockiert keiner von beiden.

Geprüft habe ich durch Lesen jedes geänderten Codepfads und durch Läufe:

- `cargo test -p krk-core` für `archiv`, `archiv_umbau`, `archiv_operation`, `archiv_lesevorgang` und `operation`: grün.
- `cargo test -p krk-ui -- archiv verknuepfung`: grün, 43 Proben.
- Die ignorierte Probe `eine_datei_ueber_vier_gib_kommt_in_ein_zip` im Profil release, in einer Kopie des Stands `c9651443` im Scratchpad: grün, 22 s.
- Eine eigene Probe über reale Archivformen, ebenfalls in dieser Kopie.

Im Projekt habe ich nichts geändert. `make check` habe ich nicht gefahren.

## Summen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 2 |

## Befunde nach Thema

### Die dreizehn Befunde und der Entscheid, je gegen den Code

**Quelle, die das Zielarchiv ist oder enthält (Critical): behoben.** `zielarchiv_in_quelle` (`crates/krk-core/src/operation/archiv.rs`) fragt die Quelle an `lstat(2)`. Dann hält es Gerät und Inode der Quelle gegen die aufgelöste Archivdatei und jeden ihrer Vorfahren. Bei einem Fehler weist es ab. Die Quelle wird beim Verschieben wie beim Kopieren abgewiesen, bevor ein Byte geschrieben ist.

Beim Lesen geprüft habe ich diese Fälle:

- **Harter Verweis auf das Archiv als Quelle:** abgewiesen, denn die Archivdatei selbst steht unter den Vorfahren.
- **Zieltab über einen verknüpften Ordner:** `canonicalize` löst ihn auf.
- **Quelle ist eine Verknüpfung:** sie wird als Verknüpfung hineingelegt und entfernt, ihr Ziel bleibt unberührt.
- **Archivquelle unter einer zweiten Schreibweise desselben Archivs:** `dieselbe_datei` vergleicht Gerät und Inode und hängt dann um, statt zu kopieren.

Die Probe `eine_lokale_quelle_die_das_zielarchiv_ist_oder_enthaelt_wird_abgewiesen` deckt Archiv und Ordner mal drei Formate mal Kopieren und Verschieben.

**tar mit hartem Verweis (High): behoben.** `umbauen_mit` weist vor der Aufstellung jedes Verzeichnis mit `Lesbarkeit::HarterVerweis` ab. `Tarstrom::schreiben` weist an `EntryType::Link` ein zweites Mal ab, auch bei einem Verweis mit unsicherem Namen.

**Kennungsprüfung beim Entfernen nach dem Verschieben (Medium): behoben.** `Entfernposten` trägt die Kennung des Verzeichnisses, aus dem der Leser las. Sie kommt aus `aus_einem_archiv` in `hinein` und aus `aus_dem_archiv_verschieben` in `fern.rs`.

`umbau::umbauen_an_kennung` weist ab, bevor ein Byte geschrieben ist. Zwei Posten derselben Gruppe mit verschiedenen Kennungen entfernen nichts.

**Eintragsweises Entfernen mit `Lokalspur` (Medium): behoben.** So läuft das Entfernen:

- `Neueintrag::lokal` legt je Eintrag eine Spur an, Kinder vor ihrem Ordner.
- `lokal_entfernen` entfernt eine Datei allein bei gleicher Kennung, also bei gleichem Gerät, Inode, Größe und Änderungszeit.
- Einen Ordner entfernt es mit `rmdir(2)`.

Ein stehen gebliebenes Kind hält jeden Vorfahren an, ohne dass der Bericht ihn doppelt nennt. Eine Datei, an deren Stelle inzwischen ein Ordner steht, fällt an der Inode-Prüfung durch. Ein Ordner, an dessen Stelle eine Verknüpfung steht, scheitert an `rmdir`. Die Restgrenze zwischen Prüfung und `unlink` schreibt der Modulkopf aus.

**ZIP64-Regel (Medium): behoben, auch Ende zu Ende.** `zipform::grosse_form_waehlen` gilt im Umbau (`neuen_zip_eintrag`) und im Zippen (`dateiwahl`). Die Schwelle `ZIP64_AB` liegt ein 256tel unter `u32::MAX`. Gepackte Daten wachsen bei Deflate um höchstens 5 Bytes je 64 KiB, also reicht der Abstand.

Die ignorierte Probe mit einer dünnen Datei von 4 GiB plus 4 KiB läuft im Profil release durch.

**`Kopfwache` (Medium): behoben.** Die Kiste `tar-0.4.46` springt in `next_entry_raw` vor jedem Kopf über `skip`, auch um null Bytes (`archive.rs`, Zeilen 292 bis 294 und 555 bis 561). Damit misst die Wache jeden Vorkopf einzeln, wie ihr Modulkopf sagt.

Für den gepackten Strom sorgt `Vorwaerts` dafür, dass auch dort gesprungen wird. Es liest unter dem Abbruchleser und an der Wache vorbei. Inhaltsverzeichnis und Lesezeiger gehen beide durch die Wache. Der Umbau liest roh und prüft die Größe vor `read_to_end`.

Einen weiteren Leser der Kiste `tar` gibt es im Baum nicht (`grep -rn "tar::Archive::new" crates/krk-core/src`).

**Verdeckte Einträge nur lesbar (Entscheid, Möglichkeit 3): umgesetzt.** `Inhaltsverzeichnis::verdeckt` zählt die drei Regeln des Sammlers. Der Umbau weist zweimal ab: vor der Aufstellung und beim Schreiben an jedem alten Eintrag mit gültigem Namen, den das Verzeichnis nicht führt.

Meine Probe über reale Formen ergab:

| Form | verdeckt |
|---|---|
| `tar -C d .` | 0 |
| `.tgz` derselben Art | 0 |
| `tar d` | 0 |
| `zip -r` | 0 |
| `ditto -c -k --keepParent` | 0 |
| `zip -g` | 0 |
| `tar -rf` mit einer erneut angehängten Datei | 2 |
| `tar -rf -n` mit einem erneut angehängten Ordner | 2 |

Ein tar nach einem `-r`-Anhang ist damit allein lesbar, auch wenn nur ein Ordnereintrag doppelt steht. Das folgt aus der gewählten Regel und ist kein Defekt. Löschte der Umbau den sichtbaren Ordner, käme der frühere Ordnereintrag als leerer Ordner wieder hervor.

Trifft der Nutzertest solche Archive oft, ist Möglichkeit 2 des Entscheids der vorgesehene Folgeschritt.

**Abbruch nach dem Ersetzen (Low): behoben.** `ersetzen` fragt den Abbruch nach `auf_die_platte_bringen` und vor dem `rename`. Danach gibt es kein „abgebrochen“ mehr.

`umbau::abschluss_nach_ersetzen` schließen alle Wege ab, die ein Archiv ersetzen:

- `hinein`;
- `aus_archiven_entfernen` und `im_archiv_abarbeiten`, die ihre ersetzten Archive zurückgeben;
- `ausfuehren` in `fern.rs`;
- `berichten` im Umbau.

Beide `#[must_use]` stehen. In `fern.rs` bleibt `if abgebrochen { return Abschluss::Abgebrochen; }` nach `aus_archiven_entfernen` stimmig: bei gesetztem Abbruch ersetzt jene Funktion kein Archiv.

**Sonderdateien (Low): behoben.** `lokal_lesen` fragt `is_file()` an `lstat(2)`. Eine Sonderdatei kommt in `Lokalbaum::ausgelassen` und von dort in den Bericht. Weil die Zeilen vor dem Vergleich mit `stand` geschrieben werden, wird der Posten unvollständig, und ein Verschieben entfernt die Quelle dann nicht.

**Schreibrecht (Low): behoben.** `sys::schreiben_erlaubt` fragt `access(2)` mit `W_OK`, auf dem Faden. Nachgeprüft: eine Datei mit `chflags uchg`, wie Finder sie über „Geschützt“ setzt, beantwortet `access` mit nein. Die Zusage in `HowTo.md` zu „über Informationen gesperrt“ hält also.

**Abwurf auf Ordnerzeile (Low): behoben.** `Tabinhalt::ort_der_zeile` (`crates/krk-ui/src/tabs.rs`) liefert im Archivtab den Archivort der Zeile. `abwurf_pruefen` und `abwurf_annehmen` nutzen ihn über `ort_in_zeile`. Für ein beschreibbares Archiv bleibt das Schreibrecht `Unbekannt`, also kein Systemaufruf. Lokal und fern antwortet die Funktion wie die zwei früheren Wege.

**Verknüpfung auf ein Archiv über einen Arbeitsfaden (Low): behoben.** So läuft der Weg:

1. `verknuepfung_betreten` fragt zuerst den eigenen Namen.
2. Danach startet `Tabliste::verknuepfung_pruefen` einen Faden, der `canonicalize` stellt.
3. Der Einzug betritt das Archiv oder reicht die Verknüpfung beim Doppelklick an das System.

Die Antwort fällt mit jedem Ortswechsel, weil `ort_setzen_an` den Tab neu baut. Die Rumpfprobe liest den neuen Weg mit. Was die Behebung nicht mitzieht, steht im Befund weiter unten.

**Rückfall des Tabs (Low): behoben.** `Archivfehler::KeinOrdner { vorhanden }` nennt den nächsten Vorfahren, der im Verzeichnis ein Ordner ist. `ausweichziel` geht dorthin oder bei `NotFound` der Archivdatei in den Rückkehrordner. Die Wurzel ist immer ein Ordner, eine Schleife entsteht also nicht.

**Statuszeilentext (Low): behoben.** Statt des Satzrests steht dort jetzt `Text::ArchivAlleinLesbar` in allen drei Tabellen. Die neuen Platzhalter `{ziel}`, `{datei}` und `{ordner}` stehen in `de.rs`, `en.rs` und `fr.rs` gleich.

### Dokumentation gegen den Code

**Low: `CLAUDE.md` nennt `Neueintrag::lokal_mit_spur` und „drei Arten Archiv“.**

- Die Funktion gibt es nicht. Die Spur liefert `Neueintrag::lokal` über `Lokalbaum`.
- Vor der Aufstellung weist `umbauen_mit` seit `c9651443` vier Arten ab. Die schreibgeschützte Archivdatei steht sogar zuerst.

Issue `261017-2202-claude-md-nennt-eine-funktion-lokal-mit-spur-die-es-nicht-gibt-und-drei-arten-archiv-wo-der-umbau-vier-abweist.md`.

Die übrigen Aussagen zu Archiven in `CLAUDE.md` stimmen. Ich habe jeden dort genannten Probennamen im Baum gefunden. `HowTo.md` stimmt mit dem Code überein, auch bei der schreibgeschützten Datei, beim späten Abbruch, bei den Sonderdateien, beim Rückfall des Tabs und bei der Grenze von 16 MiB. `README.md` beschreibt nur die Wahl der Kisten und stimmt.

### Oberfläche

**Low: Ein über eine Verknüpfung betretenes Archiv sieht Änderungen von außen nicht, wenn sein Ziel in einem anderen Ordner liegt.**

- Der Zugang trägt den Pfad der Verknüpfung.
- `sichtbare_ordner` und `zeigt_gemeldeten_ort` (`crates/krk-ui/src/auffrischung.rs`) beobachten deren Ordner, nicht den Ordner der Archivdatei.
- `HowTo.md` verspricht „Änderungen von außen erscheinen von selbst“.

Die Lücke bestand für Verknüpfungen mit eigener Endung schon vorher. `c9651443` weitet sie auf Verknüpfungen ohne Endung aus. Datenverlust entsteht nicht: Umbau und `ersetzen` prüfen die Kennung.

Belegt ist der Befund durch Lesen des Codes, nicht am laufenden Bündel. Issue `261017-2202-ein-ueber-eine-verknuepfung-betretenes-archiv-in-einem-anderen-ordner-sieht-aenderungen-von-aussen-nicht.md`.

### Frühere `Not-opened`-Liste, soweit Code

- `crates/krk-core/src/archiv/mod.rs`: der Modulkopf stimmt mit den Untermodulen.
- `operation/entpacken.rs`: hat im ganzen Paket nur `eintragszeit` und `infozip_unix_zeit` an `zipform` abgegeben, ohne Änderung im Verhalten.
- `tests/belegung.rs`: führt die fünf Befehle mit `Ortsbedarf::Beschreibbar`.
- `tests/fern_operation.rs`: nimmt `archiv`, `umbau`, `Archiv*` und `Umbau*` als Nadeln in `der_lokale_auftrag_ist_unveraendert` auf.
- `tests/archiv_lesevorgang.rs`: prüft den Vorfahren in `KeinOrdner`.

Ohne Befund.

## Übergreifende Beobachtungen

- **Die gemeinsame Regel aus der ersten Durchsicht ist jetzt für zwei der drei Herkünfte gebaut.** Die Regel lautet: entfernt wird nur, was nachweislich unverändert ist.
  - Lokal gilt sie über die `Lokalspur`.
  - Aus einem Archiv gilt sie über die Kennung.
  - Fern entfernt die Maschine weiter den ganzen Baum. Das sagt der Modulkopf ausdrücklich, und es gleicht `ueber_die_grenze`.
- **Wer ein Archiv über eine Verknüpfung betritt, hält zwei Pfade.** Der Zugang trägt die Verknüpfung, und Umbau und Kennung rechnen mit dem aufgelösten Ziel. Schreiben, Prüfen und Abweisen lösen auf. Allein die Beobachtung in `auffrischung.rs` tut es nicht, daher der zweite Befund.

## Reihenfolge

1. Vor der Auslieferung ist nichts nötig.
2. Danach die zwei Low-Befunde. Die Zeile in `CLAUDE.md` geht schnell. Für die Beobachtung genügt notfalls ein Satz in `HowTo.md`.
