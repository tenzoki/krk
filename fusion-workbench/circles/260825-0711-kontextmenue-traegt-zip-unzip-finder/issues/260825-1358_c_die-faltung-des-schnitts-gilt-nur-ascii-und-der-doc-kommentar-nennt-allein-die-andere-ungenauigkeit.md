Die Faltung des Schnitts gilt nur ASCII, und der Doc-Kommentar nennt allein die andere Ungenauigkeit

---

`gleicher_eintrag` (`crates/krk-ui/src/kommandos/kontextmenue.rs:689-699`) faltet den letzten
Bestandteil ueber `eq_ignore_ascii_case`. Das faltet ASCII-Buchstaben und sonst nichts: `uebersicht`
und `Uebersicht` sind fuer die Regel derselbe Eintrag, `übersicht` und `Übersicht` nicht. Auf einem
APFS-Datentraeger in der Vorgabe sind sie einer, und damit steht der Befund
`260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-*` fuer jeden Namen weiter, dessen
Schreibungsunterschied auf einem Buchstaben ausserhalb ASCII liegt.

Der Doc-Kommentar bei `ist_ziel_des_laufs` (`:669-675`) schreibt die **andere** Ungenauigkeit aus,
den zu weiten Schnitt auf einem schreibungsempfindlich formatierten Datentraeger. Von dieser hier
steht dort nichts, und die Ueberschrift daneben (`:656-657`) sagt ohne Einschraenkung „ohne
Ruecksicht auf Gross- und Kleinschreibung".

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `95e55da` am 260825-1358, in der vierten Durchsicht der Runde 17
(`f464bc5..95e55da`).**

## Gemessen, nicht geschlossen

Der Vergleich ist ausserhalb des Baums nachgestellt und gefahren, mit dem Rumpf von
`gleicher_eintrag` Zeichen fuer Zeichen:

| Eintrag auf der Platte | gerechnetes Ziel | `gleicher_eintrag` | auf APFS in der Vorgabe |
|---|---|---|---|
| `PROJEKTE.ZIP` | `Projekte.zip` | **true** | derselbe Eintrag |
| `ÜBERSICHT.ZIP` | `Übersicht.zip` | **true** | derselbe Eintrag |
| `übersicht.zip` | `Übersicht.zip` | **false** | derselbe Eintrag |
| `äpfel.zip` | `Äpfel.zip` | **false** | derselbe Eintrag |

Die zweite Zeile faellt richtig aus, weil `Ü` in beiden Namen dasselbe Byte-Paar ist; gefaltet wird
allein `BERSICHT` gegen `bersicht`. Die dritte und die vierte sind der Befund: `Ü` (`C3 9C`) und `ü`
(`C3 BC`) sind fuer `eq_ignore_ascii_case` zwei Bytes, denn die Funktion vergleicht jedes Byte
ausserhalb ASCII buchstabengetreu.

## Wie die Lage entsteht

Genau die Lage, die den Vorgaengerbefund getragen hat, mit einem Umlaut im Ordnernamen. Im Ordner
`Übersicht` liegt ein von fremder Hand angelegtes `übersicht.zip` — ein Werkzeug, das Namen
kleinschreibt, ein Uebertrag von einem anderen System, eine Umbenennung. Der Nutzer markiert einige
Eintraege und ruft Zip.

1. `archivname` rechnet `Übersicht.zip` aus dem Namen des angezeigten Ordners.
2. `packziel` haelt jeden markierten Eintrag dagegen; `übersicht.zip` faellt **nicht** heraus und
   bleibt Quelle.
3. Das Konfliktblatt fragt, weil der Zieleintrag auf der Platte steht.
4. „Ueberschreiben" raeumt `übersicht.zip` in den Papierkorb — eine Quelle desselben Laufs, und der
   Lauf meldet sie danach als uebersprungen.

Dasselbe auf der Entpackseite: neben `Äpfel.zip` steht `äpfel.zip.zip`, dessen Zielordner `äpfel.zip`
ist. `ohne_die_eigenen_ziele` nimmt `Äpfel.zip` nicht heraus, und beide Laeufe treffen sich.

Verloren geht nichts: der Eintrag liegt im Papierkorb. Der Ausgang ist derselbe, den die
Nutzerentscheidung vom 260825 schliessen wollte.

## Was das fuer die Nutzerentscheidung heisst

Der Nutzer hat „falten" gewaehlt und die Ungenauigkeit ausdruecklich in Kauf genommen, die eine
Faltung auf einem schreibungsempfindlichen Datentraeger kostet. Er hat dabei **nicht** gewaehlt, dass
die Faltung nur die Haelfte des Alphabets erreicht, das dieses Vorhaben ueberall sonst schreibt: die
Prosa ist deutsch, Ordnernamen mit Umlauten sind der Normalfall und keine Ausnahme.

Der Baum hat fuer den Dateifilter dieselbe Wahl anders begruendet und ausgeschrieben — `CLAUDE.md`
sagt zu `verzeichnis/filter.rs`: „Der Vergleich ist eine Teilzeichenfolge ohne Ruecksicht auf Gross-
und Kleinschreibung und faltet keine Umlaute." Dort kostet die Verengung einen ausbleibenden Treffer
in einer Liste. Hier kostet sie einen Eintrag im Papierkorb.

## Drei Wege

**Weg 1 — die Faltung auf Unicode ziehen.** `dieser.to_string_lossy().to_lowercase()` gegen
`jener.to_string_lossy().to_lowercase()`, an derselben einen Stelle. Kostet je Vergleich zwei
`String`, und die Zahl der Vergleiche ist die der markierten Eintraege mal die der Ziele; L9 bleibt
unberuehrt, denn keiner davon fasst die Platte an. Trifft `übersicht` gegen `Übersicht` und `äpfel`
gegen `Äpfel`. Trifft `Straße` gegen `STRASSE` nicht — das tut die Faltung von APFS ebenso wenig,
also ist es kein Unterschied zum Bauziel.

**Weg 2 — den Doc-Kommentar nachziehen und sonst nichts.** Zwei Saetze bei `ist_ziel_des_laufs`, die
sagen, dass die Faltung ASCII gilt und was daneben faellt. Der Fall bleibt offen, aber er steht
dann da, und die naechste Runde haelt den Weg nicht fuer vollstaendig. Billigster Weg, und er ist
das Mindeste: die Ueberschrift bei `:656` verspricht heute mehr, als der Rumpf haelt.

**Weg 3 — beides.** Weg 1 gebaut, und im Doc-Kommentar steht, welche Faltung gilt und dass sie die
von APFS nur annaehert. Eine Probe mit zwei Umlautnamen neben den drei Proben, die die Runde fuer
die ASCII-Faltung schon gestellt hat.

**Schwere:** mittel. Derselbe Ausgang wie beim Vorgaengerbefund, auf einem engeren Feld: es braucht
einen Namen, dessen Schreibungsunterschied auf einem Buchstaben ausserhalb ASCII liegt. In einem
Vorhaben mit deutschen Ordnernamen ist das kein Randfall.

**Betroffen:** `crates/krk-ui/src/kommandos/kontextmenue.rs` — `gleicher_eintrag` (`:689-699`) und
der Doc-Kommentar von `ist_ziel_des_laufs` (`:648-675`). Beide Rufer erben es, weil die Regel genau
einmal dasteht.

**Querverweise:** `issues/260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-waehrend-das-dateisystem-und-die-endungsregel-die-schreibung-falten.md`
(geschlossen, Moeglichkeit 1 vom Nutzer gewaehlt) — dieser Datensatz ist sein Rest und nicht sein
Widerruf.

---
Resolved: Weg 3 gebaut. `gleicher_eintrag`
(`crates/krk-ui/src/kommandos/kontextmenue.rs`) vergleicht den letzten
Bestandteil jetzt ueber `to_string_lossy().to_lowercase()` statt ueber
`eq_ignore_ascii_case`; der Elternteil bleibt buchstabengetreu und wird
weiterhin zuerst gefragt, also entstehen die zwei Umschriften nur, wo sie
etwas entscheiden. Es ist dieselbe Faltung, die
`krk_core::verzeichnis::filter::traegt_die_folge` dem Filter der Dateiliste
gibt: dessen `to_lowercase` faltet `Ä` auf `ä` (Probe
`der_vergleich_faltet_keine_umlaute_und_keine_akzente`), und „faltet keine
Umlaute" in `CLAUDE.md` meint die Normalisierung `ä`→`a`, nicht die
Schreibung. Eine zweite Art zu falten steht damit nicht im Baum; der
Vergleich selbst ist nicht wiederverwendet, denn `traegt_die_folge` fragt
nach einer Teilzeichenfolge und nicht nach Gleichheit. Kein neues fremdes
Paket.

Alle vier Zeilen der Messtafel des Datensatzes fallen jetzt so aus, wie APFS
in der Vorgabe entscheidet; ausserhalb des Baums nachgestellt und gefahren.
Drei Proben dazu: `das_archiv_des_vorigen_laufs_faellt_auch_mit_umlaut`
(`übersicht.zip` gegen das gerechnete `Übersicht.zip`),
`der_entpackschnitt_trifft_auch_mit_umlaut` (`äpfel.zip` gegen `Äpfel.zip.zip`)
und `ein_zerlegt_geschriebener_umlaut_bleibt_quelle`, die die verbliebene Enge
festhaelt. Gegenprobe gefahren: mit zurueckgenommener Weitung werden die
ersten zwei rot, die dritte bleibt gruen.

Der Doc-Kommentar bei `ist_ziel_des_laufs` nennt jetzt beide Ungenauigkeiten
statt einer — zu weit auf einem schreibungsempfindlich formatierten
Datentraeger, zu eng bei zusammengesetzten Zeichen (NFC gegen NFD, wofuer eine
Normalform und damit eine Zerlegungstabelle noetig waere, die dieses Vorhaben
nicht aufnimmt) — und sagt daneben, dass `Straße` gegen `STRASSE` kein
Unterschied zum Bauziel ist. Die Ueberschrift verspricht nicht mehr „ohne
Ruecksicht auf Gross- und Kleinschreibung", sondern sagt, was der Rumpf tut:
verglichen wird die kleingeschriebene Fassung. Bei `gleicher_eintrag` steht
neu, dass ein Name ohne gueltiges UTF-8 durch `to_string_lossy` geht und zwei
solcher Namen dadurch fuer einen gehalten werden koennen — dieselbe Seite wie
die zu weite Faltung, und auf dem Bauziel entsteht der Fall nicht, weil APFS
solche Namen nicht annimmt.

Eine Folge ist mitvermerkt und nicht bloss gemacht: `ohne_die_eigenen_ziele`
ordnet nach der Bytelaenge des Archivpfads, und der Beweis, dass der
Beansprucher vor dem Beanspruchten drankommt, ruhte darauf, dass
`eq_ignore_ascii_case` nur gleich lange Bytefolgen trifft. Das
Kleinschreiben ueber Unicode kann die Laenge aendern; gemessen ueber alle
Codepunkte verliert eine Handvoll Zeichen ein Byte, allein das Kelvinzeichen
`K` (`U+212A`) zwei, und drei gewinnen eines. Der Vorsprung von vier Bytes
traegt das bis zu zwei solchen Zeichen in einem Namen; ab dreien kehrt sich
die Ordnung um. Der Doc-Kommentar von `ohne_die_eigenen_ziele` sagt das jetzt
aus, samt dem Weg, der die Grenze schloesse (nach der kleingeschriebenen
Laenge ordnen). Gebaut ist er nicht: ein Archivname mit drei Kelvinzeichen ist
kein Fall dieses Vorhabens.

`make check` gibt Exit 0 (Bau, Proben, `clippy -D warnings`, `fmt --check`).
