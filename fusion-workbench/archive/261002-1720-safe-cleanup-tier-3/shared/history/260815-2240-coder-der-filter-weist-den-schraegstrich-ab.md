# Der Filter der Dateiliste weist den Schraegstrich ab

**Status:** Complete
**Agent:** coder
**Datum:** 260815-2240
**Quelldatensatz:** `shared/issues/260815-2208_*_der-filter-nimmt-den-schraegstrich-auf-den-kein-name-tragen-kann-und-seit-dem-ordnerzeichen-steht-er-auf-dem-schirm.md`

## Auftrag

`traegt_ein_dateiname` (`krk-core/src/verzeichnis/filter.rs`) beantwortet die Frage, ob
ein Dateiname ein Zeichen tragen kann, und antwortet fuer `/` mit ja. `name_pruefen`
(`krk-core/src/operation/umbenennen.rs`) weist denselben Schraegstrich ab. Seit `3b128c3`
steht er als Ordnerzeichen in der Namensspalte: wer `Bilder/` liest und `bilder/` tippt,
bekommt eine leere Liste, weil `traegt_die_folge` gegen `eintrag.name` vergleicht.

Der Datensatz nennt zwei Wege und macht Weg 1 (die Zeichenregel um den Schraegstrich
erweitern) von einer Vorpruefung abhaengig: die Zeichenregel hat zwei Rufer, und der
zweite ist die Tippsuche der Belegungsansicht aus der Runde 7.

## Die Vorpruefung, am Dateibestand gemessen

Gefragt war, ob die Belegungssuche den Schraegstrich braucht. Gesucht wird dort ueber
genau zwei Spalten, `funktionstext` und `tastentext` (`krk-ui/src/belegungsmodell.rs`,
`zeile_traegt`). Drei Messungen am Baum, keine Annahme:

| Frage | gemessen |
|---|---|
| Steht `/` in `resources/default-keymap.toml` ausserhalb der Kommentare? | **Nein.** 36 Fundstellen, alle in Kommentarzeilen, alle Teil eines Verweispfades. Keine Zeile `tasten =`, `id =` oder `name =` traegt ihn. |
| Kann eine Anzeigeform einer Kombination ihn zeigen? | **Nein.** `anzeige` schreibt allein die Namen aus `krk_core::tasten::parser::TASTEN` gross. Die Tabelle traegt 61 Namen: `f1` bis `f12`, `delete`, `return`, `tab`, `esc`, `space`, der Pfeilblock samt `pageup`/`pagedown`/`home`/`end`, `a` bis `z`, `0` bis `9`. Eine Schraegstrich-Taste steht nicht darin, also ist sie auch nicht zuweisbar. |
| Traegt eine Beschriftung ihn ueber `reserviert_fuer` oder `gehalten_von`? | **Nein.** Die beiden Zusaetze sind `den Editor` und `Kuerzel des Menues`, sonst der Rohtext aus derselben Datei. |

Damit vertraegt die Belegungssuche den Eingriff, und Weg 1 traegt die Aufgabe. Eine
zweite Zeichenregel oder ein Parameter je Rufer war nicht noetig und ist nicht gebaut.

## Was geaendert wurde

**`crates/krk-core/src/verzeichnis/filter.rs`**

- `traegt_ein_dateiname` bekommt `&& zeichen != '/'`. Der Schraegstrich steht als Literal
  da und nicht als Konstante, wie in `name_pruefen` auch; die Klammer `ORDNERZEICHEN` in
  `krk-ui/src/appkit/tabelle.rs` bleibt davon getrennt, denn sie benennt eine Anzeige und
  `krk-core` kennt `krk-ui` nicht.
- Der Doc-Kommentar zaehlt jetzt drei Klassen statt zweier und nennt bei der dritten den
  Grund und die zweite Stelle, die ihn schon abweist.
- Der Modulkopf bekommt den Absatz, der die Bedienfolge aus dem Datensatz festhaelt: das
  Ordnerzeichen ist Anzeige und nie Name, der Vergleich liest weiter `eintrag.name`, und
  darum kann der Filter den Schraegstrich nicht aufnehmen.
- Zwei Proben. `ein_schraegstrich_traegt_kein_dateiname` haelt die Zeile fest.
  `was_die_zeichenregel_aufnimmt_traegt_auch_ein_name` haelt die Richtung fest, in der der
  Widerspruch stand: was die Zeichenregel aufnimmt, muss `name_pruefen` durchlassen. **Die
  Umkehrung ist ausdruecklich nicht behauptet** — ein Name mit Zeilenumbruch ist unter
  macOS zulaessig, taugt als Filtereingabe aber nicht, und eine Gleichheitsprobe waere
  daran gescheitert.

**`crates/krk-ui/src/belegungsmodell.rs`**

- Der Doc-Kommentar von `Suchlage::zeichen_anhaengen` zaehlte die Klassen der Zeichenregel
  auf und wurde durch die Aenderung unvollstaendig. Er nennt jetzt die dritte und traegt
  die Vorpruefung von oben in zwei Saetzen: kein Tastenname der Tabelle traegt den
  Schraegstrich, also kostet die Aenderung diese Suche nichts.

`crates/krk-ui/src/appkit/tabelle.rs` ist nicht angefasst; eine andere Aufgabe derselben
Runde haelt die Datei. Ihr Doc-Kommentar bleibt wahr: „Ein Zeichen, das kein Dateiname
tragen kann, ist deshalb nicht verbraucht" — der Schraegstrich laeuft jetzt eben unter
diesen Satz und an AppKit weiter.

## Was die Aenderung fuer die Bedienung heisst

Ein Tastendruck auf `/` laesst den Filtertext unveraendert stehen, dieselbe Regel wie bei
der Eingabetaste und keine neue. Wer `bilder` tippt, findet `Bilder/` weiterhin; der
angehaengte Schraegstrich der Anzeige stoert dabei nicht, weil der Vergleich ihn nie
sieht.

## Abnahme

`make check` — exit 0. Die Zaehlprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
(`krk-core/tests/verzeichnis.rs`) laeuft gruen: die Zeichenregel steht weiter genau einmal
und hat weiter genau zwei Rufer.

Nicht committet, wie beauftragt.
