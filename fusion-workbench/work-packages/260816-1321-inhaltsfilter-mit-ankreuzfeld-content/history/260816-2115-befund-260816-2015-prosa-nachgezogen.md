# Befund 260816-2015: die Prosa an drei Orten nachgezogen

**Status:** Complete
**Agent:** coder
**Datum:** 260816-2115
**Circle:** 260816-1321-inhaltsfilter-mit-ankreuzfeld-content

## Auftrag

Den Befund `issues/260816-2015_*_der-vergleich-hat-drei-rufer-und-die-prosa-an-seinem-ort-nennt-zwei.md`
abtragen. Nur Prosa: kein Verhalten, keine Signatur, keine Probe. Selbst nachzählen
statt die Zahlen des Befunds zu übernehmen.

## Nachgezählt am Baum

| Regel | Rufer | Stellen |
|---|---|---|
| `traegt_die_folge` | 3 | `verzeichnis/modell.rs:653`, `verzeichnis/durchlauf.rs:497`, `verzeichnis/inhalt.rs:139` |
| `traegt_ein_dateiname` | 2 | `krk-ui/src/appkit/tabelle.rs:1295`, `krk-ui/src/belegungsmodell.rs:695` |
| `ohne_warten_oeffnen` | 2 | `text/datei.rs:421` (`lesen`), `text/datei.rs:606` (`bis_zur_grenze_lesen`) |
| `ist_deskriptormangel` | 2 | `verzeichnis/durchlauf.rs:472`, `text/datei.rs:609` |

Die Zahlen des Befunds halten. Kein Rufer liegt außerhalb von `krk-core`.

## Geändert

**`crates/krk-core/src/verzeichnis/filter.rs`** — vier Stellen, alle im Befund genannt:

1. Das Bild im Modulkopf führt drei Pfeile auf den Vergleich statt zweier
   (`modell::sichtbar`, `durchlauf`, `inhalt`). Der Knoten heißt jetzt
   `traegt_die_folge(Text, Filtertext)` statt `(Name, …)`, weil der dritte Rufer
   den gelesenen Text einer Datei hereingibt und keinen Namen. Gleiche Länge,
   also blieb die Ausrichtung der Kästen erhalten.
2. Der Abriss darunter: „der Vergleich hat drei" mit allen drei Stellen einzeln.
3. Der Absatz „Der eine Vergleich" nennt die drei Rufer einzeln. Dazu ein neuer
   Absatz, warum das Argument weiter `name` heißt, obwohl der dritte Rufer keinen
   Namen vergleicht — die Frage stellt sich, sobald das Bild `Text` sagt.
4. Die Vorbedingung nennt Prüfschritt und Durchlauf beim Namen statt „ihre beiden
   Rufer" und begründet für den dritten getrennt: der Inhaltsbefund entsteht ohne
   Filtertext gar nicht, weil `inhaltsschwelle` nicht erreicht ist. Ein
   „drei Rufer haben den Zweig davor" wäre für den dritten schlicht falsch gewesen.

**`crates/krk-core/src/verzeichnis/sys.rs`** — drei Stellen, zwei davon im Befund:

5. Der Absatz zum Ort der Aufrufer (Modulkopf): beide liegen seit dieser Runde in
   `text/datei.rs`, beim Namen genannt; Vorschau und Inhaltsfilter rufen die Hülle
   und nicht diese Stelle; die frühere Lage in `krk-ui` steht als Vergangenheit da.
6. Der Absatz zu `ist_deskriptormangel`: zwei Frager statt eines, und für den
   zweiten steht daneben, an welchem Zweig er hängt.
7. **Nicht im Befund:** das Bild im Modulkopf führte für `fcntl(2)` die Zeile
   `└─> krk-ui: vorschaumodell` und in der Zeile darüber `text::datei::oeffnen`.
   Beide sind falsch — der zweite Aufrufer ist `text::datei::bis_zur_grenze_lesen`,
   der erste `text::datei::lesen` (`oeffnen` liegt eine Ebene darüber und ruft
   `lesen`). Der Befund nennt für `sys.rs` „zwei Absätze"; das Bild ist der dritte Ort.

**`crates/krk-core/src/verzeichnis/mod.rs`** — eine Stelle, **nicht im Befund**:

8. Der Modulkopf verortete den zweiten Aufrufer von `ohne_warten_oeffnen` noch
   „vom Leseweg der Vorschau in `krk-ui`" und den ersten als `text::datei::oeffnen`.
   Beide nennen jetzt die Funktion, die tatsächlich ruft.

## Was nicht geändert ist und warum

- **Die Zählprobe** `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`
  (`crates/krk-core/tests/verzeichnis.rs:2867`) ist unberührt. Sie führt drei und
  ist grün; ihr Doc-Kommentar erzählt den Übergang („seither hat er zwei … mit der
  Runde 11 werden es drei") und ist als Erzählung des Schritts richtig.
- **Keine Signatur.** `traegt_die_folge(name, filter_klein)` heißt weiter so; die
  Umbenennung des Parameters wäre eine Signaturänderung und stand nicht im Auftrag.
  Stattdessen sagt die Prosa, warum der Name bleibt.
- **Kein Verhalten, keine Probe.**

## Regel, an die sich das Nachziehen gehalten hat

Wo eine Zahl unvermeidlich war, stehen die gezählten Stellen daneben — dieselbe
Form, die `modell.rs:641-648` und die Zählprobe schon tragen. Eine nackte Zahl ohne
ihre Stellen ist genau der Defekt, den dieser Befund meldet.

## Sweep über denselben Fehlertyp

```sh
grep -rn "beiden Rufer\|zwei Rufer\|Vergleich hat zwei\|zwei Aufrufer\|beiden Aufrufer\|\
zwei Frager\|beiden Frager\|einen Rufer\|einen Aufrufer\|ausserhalb der Kiste" \
  crates --include='*.rs'
```

Alle übrigen Treffer geprüft und richtig: `text/datei.rs:125` („die zwei Aufrufer"
von `lesen` — Editor und Notizzettel), `modell.rs:576` (die zwei Frager von
`sichtbar`), `vorschaumodell.rs:68,82` (von A1 nachgezogen), `sys.rs:790` (von A1
nachgezogen), `zulaessigkeit.rs`, `menue.rs:1133`, `tabelle.rs:543,1496`.

## Abnahme

```
export PATH="$HOME/.cargo/bin:$PATH" && make check
```

`exit 0`, alle vier Kommandos grün, 19 Prüfziele mit `test result: ok`. Darunter
`die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` und die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`shared/issues/260816-0055_o_…`),
die diesmal durchgelaufen ist.

## Befund geschlossen

`issues/260816-2015_c_…` mit `Resolved:`-Note, Marker `_o_` → `_c_`.
Nicht committet — der Commit liegt beim Orchestrator.
