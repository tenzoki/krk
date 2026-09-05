Der Probenhelfer `liste` verspricht „eine Tabliste, die nie liest", und drei Proben starten Lesevorgänge gegen `/`, `/a` und das Temporärverzeichnis

---

`crates/krk-ui/src/tabs.rs:1153-1160` führt den Helfer `liste` mit dem Doc-Kommentar „Eine
Tabliste, die nie liest. … ein Lesevorgang je Probe startete einen Arbeitsfaden gegen einen
Ordner, den es nicht gibt." Der Helfer selbst liest nicht, die Proben darauf schon:

- `das_schliessen_ruecht_die_sichtbare_stelle_nach` (`:1227-1238`): `waehlen(2)` ruft
  `ungelesenen_aktiven_nachlesen` (`:527`, `:1023-1028`) und startet einen `Lesevorgang` gegen
  `/c`, danach gegen `/b`; `schliessen` gegen `/a`.
- `der_naechste_und_der_vorige_tab_laufen_um` (`:1190-1199`): vier Lesevorgänge gegen `/a`,
  `/b`, `/c`.
- `ein_ordnerwechsel_laesst_den_filtertext_stehen_…`, `der_aufstieg_…`,
  `mit_tiefer_suche_…`, `ein_ordnerwechsel_traegt_den_stand_von_content`,
  `der_inhaltsfilter_geht_auch_ohne_filtertext_hinueber`,
  `die_tiefe_suche_geht_auch_ohne_filtertext_hinueber` (`:1447-1450`,
  `zwei_vorhandene_ordner`): `ordner_setzen("/")` startet je Probe einen Arbeitsfaden, der die
  **Wurzel des Dateisystems** liest, und der Helfer nimmt daneben `std::env::temp_dir()`.

Die Fäden laufen gegen nicht vorhandene Pfade ins Leere oder lesen `/` und `/tmp`, bis ihr
`Drop` das Abbruchkennzeichen setzt (`krk-core/src/verzeichnis/leser.rs:150-159`, wartet
nicht). Sichtbar wird das nicht — deshalb ist die Schwere niedrig —, aber dieselbe Datei
benutzt ab `:1995` für dieselbe Aufgabe `crate::pruefordner::Pruefordner`, also genau die Fassung,
die `CLAUDE.md` für diese Kiste vorschreibt („Wer einen Prüfordner braucht, nimmt die Fassung
seiner Kiste"). Zwei Praktiken in einer Datei, und die eine hat einen Kommentar, der nicht
stimmt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/tabs.rs` (Prüfmodul: `liste`, `zwei_vorhandene_ordner`,
`gelesene_liste`)
**Baumstand:** `ca8072d`

## Verwandt

`shared/issues/260826-1309_*_eine-probe-in-bericht-rs-loescht-einen-festen-namen-im-echten-temporaerverzeichnis.md`
führt denselben Griff ins echte Temporärverzeichnis für `krk-bench`.

## Weg

`zwei_vorhandene_ordner` und `gelesene_liste` bauen ihre Ordner mit `Pruefordner::neu`, wie die
Durchlaufproben derselben Datei; der Doc-Kommentar von `liste` sagt dann wahr, was er heute
verspricht, oder er fällt.

---
Resolved: Beide Hälften, und eine dritte, die der Befund nicht führt.

**Kein Griff mehr ins echte Temporärverzeichnis und keiner an die Wurzel.**
`grep -c 'std::env::temp_dir' crates/krk-ui/src/tabs.rs` liefert 0, vorher 4 —
der Befund nennt zwei davon (`gelesene_liste`, `zwei_vorhandene_ordner`), die
zwei anderen standen in `eine_auffrischung_nimmt_ordner_auswahl_und_bildlauf_mit`
und `eine_auffrischung_laesst_die_liste_stehen_bis_ihr_erster_stapel_da_ist`
und sind mitgegangen. Der Pfad `"/"` als zweiter Ordner ist ebenfalls gefallen.

**Ein Bauplatz statt vier.** Neu ist der Helfer `vorhandene_ordner(namen)`, der
so viele Unterordner unter einem `crate::pruefordner::Pruefordner` anlegt, wie
Namen hereinkommen, und ihn mit herausgibt; `zwei_vorhandene_ordner` ist die
Abkürzung darauf. `gelesene_liste` legt sich seinen eigenen Prüfordner an.
Beide Helfer geben den `Pruefordner` zurück, weil sein `Drop` sonst abräumte,
während der Faden noch liest; die 14 Rufer binden ihn als `_ordner`.

**Auch die Leseläufe gegen `/a`, `/b`, `/c` sind weg**, obwohl der Befund sie
nur benennt und sein "Weg" sie nicht verlangt: `der_naechste_und_der_vorige_tab_laufen_um`
und `das_schliessen_ruecht_die_sichtbare_stelle_nach` fahren jetzt über
`vorhandene_ordner(&["a", "b", "c"])` und vergleichen gegen die gelieferten
Pfade statt gegen die Zeichenketten. Damit gibt es in dieser Datei nur noch eine
Praxis, und das war der eigentliche Befund.

**Der Doc-Kommentar von `liste` sagt jetzt, was zutrifft:** der Helfer startet
keinen Lesevorgang, `waehlen`, `schliessen` und `ordner_setzen` auf dem Ergebnis
sehr wohl, und wer einen davon ruft, gibt der Probe vorhandene Ordner. Wer
allein die Tabverwaltung prüft, darf weiter erfundene Namen nehmen; das steht
dort als Regel und nicht mehr als Zusage, die die Probe bricht.

Beleg: `cargo test -p krk-ui tabs` → 56 bestanden;
`cargo clippy -p krk-ui --all-targets -- -D warnings` → 0.
