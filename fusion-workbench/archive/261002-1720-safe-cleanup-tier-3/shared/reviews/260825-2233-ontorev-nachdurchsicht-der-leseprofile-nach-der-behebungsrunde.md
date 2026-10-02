# Ontoreview: Nachdurchsicht der Leseprofile nach der Behebungsrunde

**Reviewed-range:** `ecd7e4b..1ac5dde`
**Not-opened:** `crates/krk-core/src/operation/zippen.rs`, `crates/krk-core/tests/operation.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `fusion-workbench/shared/history/260825-2210-coder-aktives-setzen-hat-genau-zwei-aufrufer-und-eine-probe-haelt-es.md`, `fusion-workbench/shared/history/260825-2216-coder-ein-gepackter-eintrag-mit-ersatzdatum-bleibt-stumm.md`, `fusion-workbench/shared/history/260825-2222-coder-die-probe-zu-c6-7-misst-den-gemeinsamen-speicher.md`, `fusion-workbench/shared/issues/260825-2127_c_die-probe-zu-c6-7-misst-nicht-mehr-das-groesste-mitgelieferte-profil.md`, `fusion-workbench/shared/issues/260825-2127_c_ein-dritter-weg-nach-aktives-setzen-haelt-den-bau-nicht-an-und-keine-probe-faengt-ihn.md`, `fusion-workbench/shared/issues/260825-2127_c_ein-gepackter-eintrag-mit-ersatzdatum-steht-in-der-liste-der-uebersprungenen.md`

**Sender:** ontorev
**Gegenstand:** `resources/default-readers.toml` nach `1ac5dde`, gegen die fünf `Resolved:`-Vermerke
unter `shared/issues/260825-2126_c_*.md` und gegen den Quelltext in
`crates/krk-core/src/leseprofil/{datei,erkennung,bausteine,mod}.rs`. Die Quelltextänderungen des
Bereichs bekommen ihre eigene Durchsicht durch `coderev`; von `crates/krk-core/tests/leseprofil.rs`
ist die Probe zu C6.7 gelesen, weil sie die Beispielzahlen der Profildatei hält. Die Not-opened-Liste
nennt die Dateien des Bereichs, die diese Durchsicht nicht betrifft.

## Summary

Alle fünf mittleren Befunde der ersten Durchsicht sind behoben, und jeder `Resolved:`-Vermerk hält
der Nachmessung stand: das `(?m)^` steht an beiden Stellen und wehrt die drei gemessenen Fälle ab,
die Öffnungsrechnung der Zeile „Projekt" stimmt, die drei Reichweiten sind gegen `datei.rs`
überschneidungsfrei und vollständig, die Leselaufregel zählt den Erkennungslauf mit, und die zwei
Projektwurzelprofile nennen ihren Preis. Drei neue Befunde, alle niedrig, alle an der neuen
Prosa: ein Satz gilt nur für Profile ohne `pfad`, ein Beispiel nennt einen Zustand, den fusion
nicht herstellt, und eine der zwei Beispielzahlen hält keine Probe.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 3 |

Die drei sind unter `shared/issues/260825-2233_o_*.md` abgelegt. Die drei niedrigen Befunde der
ersten Durchsicht (L1 `.DS_Store`, L2 flight-Doppelungshinweis, L3 drei Felder) sind unverändert
offen und hier nicht gedoppelt; die neue Prosa berührt keinen davon.

## Wie geprüft wurde

Nicht durch Hinsehen. Die Messhilfe der ersten Durchsicht liegt noch im Sitzungsverzeichnis
(`scratchpad/profilprobe`, vom Ontocoder um `--toml <pfad>` erweitert); sie lädt die Datei über
`toml::from_str` und `leseprofil::datei::pruefen` und fährt `zusammenfassen_gezaehlt`. Gefahren
gegen fünf wirkliche Orte (`krk`, `krk/fusion-workbench`, `…/shared`, `example`,
`example/flight-workbench`), sieben künstliche Ordner und zwölf abgewandelte Fassungen der Datei.
Der Baum ist unverändert; die Profildatei ist nicht angefasst.

`cargo test -p krk-core --lib leseprofile`: 2 Proben grün, `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
eingeschlossen. `cargo test -p krk-core --test leseprofil`: 47 grün, 1 ignoriert.

## Die fünf Vermerke, einzeln nachgemessen

### 1. Sitzung (`:324`, `:662`) — hält

Beide Stellen tragen `(?m)^## Current\n…`. Gemessen an vier künstlichen Werkbankwurzeln:

| `orchestrator-live.md` enthält | Zeile „Sitzung" |
|---|---|
| `### Current` + `FALSCHER-ABSCHNITT` vor dem echten `## Current` | `RICHTIG` |
| Fließtextzeile `Text siehe ## Current` vor dem echten Abschnitt | `RICHTIG` |
| CRLF | `--` |
| `## Current`, dann Leerzeile, dann `## Next` | `--` |

Der vierte Fall ist meiner, nicht der des Vermerks: er hält die ältere Zusage `:284-287` („ist der
Abschnitt leer, steht der Platzhalter da und nicht der Name des nächsten Abschnitts") auch mit
dem neuen Präfix. Der neue Absatz `:288-292` beschreibt genau das, was `(?m)^` im Ausdruck tut.
CRLF zeigt den Platzhalter; die Prosa verspricht dazu nichts, und das ist die richtige Menge.

### 2. Zeile „Projekt" (`:270-278`) — hält

Gemessen an `krk/fusion-workbench`: 3 Leseläufe, 4 Öffnungen (drei `.fusion-setup`, eine
`orchestrator-live.md`, `.active-circle` fehlt und kostet keine). Der Satz „jeder Feldbaustein
öffnet seine Datei selbst" ist `bausteine.rs`, `Lauf::feld`: `oeffnungen_nehmen(1)` je Zeile, vor
dem Lesen. „Drei der vierundzwanzig" ist `HOECHSTENS_OEFFNUNGEN = 24`; die Zahl steht in der Datei
schon `:219` in der Grenzenliste, also nicht neu an einer zweiten Stelle.

### 3. Die drei Reichweiten (`:41-66`, `:110-111`) — hält, mit einem Vorbehalt (N1)

Gegen `datei.rs` gehalten: `deny_unknown_fields` an `Profildatei`, `Zeilendatei` und den vier
Bausteintischen, nicht an `Profilblock`; `name: String` ohne `default`; `Anzeigedatei` mit zwei
Werten. Gemessen an elf abgewandelten Fassungen:

| Verschreibung | Datei sagt | gemessen |
|---|---|---|
| `foo = 1` an der obersten Ebene, in einer Zeile, in einem Bausteintisch | 1 | Datei fällt |
| `name` fehlt | 1 | Datei fällt |
| `zeigt = "beides"` | 1 | Datei fällt |
| `datei` fehlt im Feldbaustein („sonst etwas") | 1 | Datei fällt |
| `kennzeichnen` an einem Profil ohne `pfad` | 2 | 11 Profile, Meldung |
| `zeilen` statt `zeile` | Profil ohne Zeilen, keine Meldung | 12 Profile, keine Meldung, Wurzel zeigt 2 Zeilen |
| `foo = 1` im `[[profil]]`-Block | übergangen | 12 Profile, keine Meldung |
| `ordner = "circles/"` | 3 | Zeile „Runden": „traegt ein leeres Stueck" |
| `ordner = "*/*/x"` | 3 | Zeile „Runden": „mehr als einen Platzhalter" |
| **`kennzeichnen` an einem Profil mit `pfad`** | 2 | **12 Profile, keine Meldung, Profil greift über `pfad`** |

Die letzte Zeile ist der Vorbehalt: der Satz `:63-65` beschreibt den Fall ohne `pfad` und gilt
als Regel nicht. Befund N1.

### 4. Leselaufregel (`:227-239`, `:478-480`) — hält, mit einem Vorbehalt (N3)

Gegen `erkennung.rs` und `bausteine.rs` gehalten: der zweite Durchgang ruft `lauf.eintraege()`,
das über `wurzelstand` und `stand_am` läuft und dort `leselauf_nehmen` bucht — der Erkennungslauf
ist damit ein Lauf des Haushalts, wie `HOECHSTENS_LESELAEUFE` es im Doc-Kommentar sagt. Ein
Feldbaustein ohne `ordner` nimmt denselben `wurzelstand`, also teilt er sich die Lesung. Der erste
Durchgang sieht allein auf `to_string_lossy()` des Pfades und bucht nichts. Gemessen: Wurzel 3,
Projektwurzel 4, gemeinsamer Speicher 10, flight-Wurzel 5, flight-Projektwurzel 6. Die Drei hält
die Probe zu C6.7 genau, die Vier hält nichts. Befund N3.

### 5. Projektwurzelprofile (`:627-635`, `:764-766`) — hält, mit einem Vorbehalt (N2)

„Ein `kennzeichen` sieht allein die Namen der Einträge" ist `erkennen`: `kennzeichen.is_match(&eintrag.name)`.
Gemessen: leeres `fusion-workbench` → sieben `--`, Datei `fusion-workbench` → sieben `--`, leeres
`flight-workbench` → sieben `--`, je 2 Leseläufe und 0 Öffnungen. Der Preis steht jetzt für fusion
**und** flight, wie verlangt. Der Halbsatz „also der Zustand vor `/fusion:setup`" stimmt nicht:
Setup legt das Verzeichnis mit `mkdir -p` samt Unterordnern an, oder es hält davor an und legt
gar nichts an. Befund N2.

## Neue Befunde

**N1 — Der Satz über `kennzeichnen` gilt nur für ein Profil ohne `pfad` daneben.** `:63-65`. Mit
einem `pfad` daneben wird der verschriebene Schlüssel still übergangen, das Profil greift über
den `pfad` weiter, und keine Meldung nennt die fehlende zweite Erkennung. Gemessen: 12 Profile,
keine Meldung, `shared/history` bekommt sein Profil wie zuvor. Der Absatz endet auf „und das
ohne jede Meldung" und meint `zeilen`; für `kennzeichnen` gilt es je nach Nachbarschlüssel auch.
Fix: ein Halbsatz.
→ `shared/issues/260825-2233_o_der-satz-ueber-kennzeichnen-gilt-nur-fuer-ein-profil-ohne-pfad-daneben.md`
· Schwere **niedrig**

**N2 — Ein leeres `fusion-workbench` ist nicht der Zustand vor `/fusion:setup`.** `:629-631`.
`skills/setup/SKILL.md:80` fährt `mkdir -p` über zwölf Unterordner und schreibt danach den
Marker; bei `OLD=1` (`:68-69`) fährt es das `mkdir` gar nicht. git führt keine leeren
Verzeichnisse. Der Preis ist richtig und dreifach gemessen; das Beispiel nennt einen Zustand,
den keiner der zwei Wege herstellt. Fix: den Halbsatz streichen.
→ `shared/issues/260825-2233_o_ein-leeres-fusion-workbench-ist-nicht-der-zustand-vor-fusion-setup.md`
· Schwere **niedrig**

**N3 — Die Beispielzahl „vier" des Projektwurzelprofils hält keine Probe.** `:236-239`. Die Drei
der Wurzel hält `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` als
`assert_eq!(…, (3, 5))`, mit der Begründung, dass „unter der Grenze" den Schritt nicht meldete;
für die Projektwurzel gibt es keinen Prüfordner. Die Vier ist die eine Zahl, die den neuen
Halbsatz „plus einen Lauf für die Erkennung" belegt. Fix: ein vierter Prüfordner in jener Probe
(`coder`), oder beide Zahlen streichen und die Herleitung stehen lassen (`ontocoder`).
→ `shared/issues/260825-2233_o_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md`
· Schwere **niedrig**

## Die drei Fragen der Dispatch

**Sagt jeder neue Satz, was der Code tut?** Bis auf zwei Halbsätze ja, und beide sind oben Befund.
Alle übrigen neuen Aussagen sind gegen eine Stelle im Quelltext gehalten: die Orte des
`deny_unknown_fields` (`datei.rs`, Modulkopf und die sieben Strukturen), `zeile` mit
`#[serde(default)]`, `Ortsangabe::aus_angabe` für das leere Stück und den zweiten Platzhalter,
`Lauf::feld` für die Öffnung je Feldbaustein, `erkennen` für die Namensprüfung und die
Reihenfolge der Durchgänge, `Lauf::eintraege` → `stand_am` → `leselauf_nehmen` für den
Erkennungslauf. Die Zahlen sind gemessen und nicht abgelesen.

**Werden die Beispielzahlen mit der nächsten Runde falsch?** Die 24 nicht anders als bisher: sie
ist `HOECHSTENS_OEFFNUNGEN` und stand schon `:219`. Die 3 und die 4 hängen an den sieben Zeilen
der zwei Wurzelprofile und an der Bauart der Erkennung; eine Runde, die einem der Profile eine
Zeile mit neuem Ort gibt oder den Erkennungslauf anders bucht, macht sie falsch. `CLAUDE.md` löst
das mit einer Probe, die die Zahl hält, oder mit dem Zählweg statt der Zahl. Die 3 hat die
Probe, die 4 nicht (N3). Der Satz daneben trägt die Herleitung, sodass ein Nutzer die Zahl auch
ohne Probe nachrechnen kann — das ist die Form, die `CLAUDE.md` für Zahlen ohne Halter wählt,
und sie steht schon da. „Drei der 24" ist eine Rechnung über drei Zeilen, die vor einem stehen,
und keine Zählung über den Baum; sie wird falsch, wenn jemand die dritte `.fusion-setup`-Zeile
streicht, und dann ist der Satz zwei Zeilen darüber ohnehin zu ändern.

**Handbuch oder Vertrag?** 433 von 801 Zeilen sind Kommentar, vorher 392 von 760; der Anteil ist
von 52 auf 54 Prozent gestiegen. Die 58 neuen Zeilen verteilen sich auf sechs Stellen, und jede
steht dort, wo die Frage entsteht, die sie beantwortet: die Reichweiten beim Abschnitt „Was ein
Schreibfehler kostet", der Erkennungslauf bei der Leselaufregel, das `(?m)` über dem Muster, das
es trägt, der Preis über dem Profil, das ihn zahlt. Nichts davon ist ein Abschnitt, den man
überspringen muss, um zu den Profilen zu kommen; die Profile selbst sind unverändert lang. Was
mit dieser Runde gewachsen ist, ist die Zahl der **Vorbehalte** in den Absätzen — „plus einen
Lauf, wenn … und keine seiner Zeilen …", „oder, steht ein `pfad` daneben, …" (N1 würde einen
dritten hinzufügen). Das ist die Stelle, an der ein Handbuch zum Vertrag wird: nicht an der
Länge, sondern daran, dass ein Satz drei Bedingungen trägt, bevor er etwas sagt. Die
Leselaufregel `:227-239` ist heute ein Absatz von dreizehn Zeilen für eine Regel, die in
`bausteine.rs` ein Satz ist („die Zahl der verschiedenen genannten Orte"). Ein Vorschlag ohne
Datensatz, weil er eine Gestaltungsfrage ist und kein Defekt: die Regel als **zwei** Sätze
schreiben — einer für den Mechanismus (jeder Ort einmal, der erkannte Ordner ist ein Ort, ob ihn
eine Zeile oder die Erkennung liest), einer für die Folge (`pfad` liest nichts) — und die
Beispielzahlen dahinter. Das sagt dasselbe mit einer Bedingung weniger.

## Cross-cutting

**Die zwei Vorbehalte (N1, N2) sind Sätze über etwas außerhalb dieser Datei.** N1 über eine
Profilform, die kein mitgeliefertes Profil hat; N2 über einen Zustand, den fusion herstellt. Die
erste Durchsicht fand fünf Stellen, an denen die Prosa vom Mechanismus abwich; die Behebung hat
alle fünf an den Mechanismus gebunden und dabei an zwei Stellen über ihn hinausgesprochen. Wer
die Datei künftig erweitert, hat neben den zwei Fragen der ersten Durchsicht (sagt der Absatz
dasselbe wie der Quelltext, dasselbe wie der Absatz zweihundert Zeilen weiter oben) eine dritte:
sagt er etwas, das erst an einem Profil gilt, das es nicht gibt.

**Der Modulkopf von `datei.rs` trägt dieselbe Lücke wie N1.** „Ein verschriebenes `pfad` laesst
das Profil ohne Pfadmuster und ohne Kennzeichen zurueck, und genau das weist `pruefen` mit einer
Meldung ab" gilt ebenso nur ohne den zweiten Schlüssel. Die Profildatei hat den Satz von dort
übernommen. Als Beobachtung für `coderev` festgehalten, ohne Datensatz von mir; der Datensatz zu
N1 nennt die Stelle.

## Empfohlene Reihenfolge

Nichts hält eine Auslieferung auf. Alle drei sind Aufräumarbeit:

1. **N2** — ein Halbsatz weg. Keine Nebenwirkung.
2. **N1** — ein Halbsatz dazu, in derselben Zeile, in der `datei.rs` es auch sagen sollte.
3. **N3** — ein Prüfordner in der Probe zu C6.7, oder die zwei Zahlen weg. Die erste Form passt
   zur Probe, wie sie ist.

Dazu die drei offenen niedrigen Befunde der ersten Durchsicht (`260825-2126_o_*`), unverändert.

## Was ich nicht geprüft habe

- **Die Anzeige.** Was das Vorschaufenster aus `Zusammenfassung::als_text` macht, ist an einer
  laufenden Anwendung zu sehen.
- **Der Quelltext des Bereichs** außer `leseprofil/` und der Probe zu C6.7: `zippen.rs`,
  `anwendung.rs`, `tabelle.rs` und ihre Proben gehören zur parallelen Durchsicht.
- **Der Weg zur Nutzerdatei.** Ob eine bestehende `readers.toml` das berichtigte `(?m)^` je
  erreicht, ist die offene Frage `shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-…`;
  für M3 ist sie die eigentliche, denn das falsche Muster liegt bei jedem Nutzer, der KRK vor
  `1ac5dde` gestartet hat.
