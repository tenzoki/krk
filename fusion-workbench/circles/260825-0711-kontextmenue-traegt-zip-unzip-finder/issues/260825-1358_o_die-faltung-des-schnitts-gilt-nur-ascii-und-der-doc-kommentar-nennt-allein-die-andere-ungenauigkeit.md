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
