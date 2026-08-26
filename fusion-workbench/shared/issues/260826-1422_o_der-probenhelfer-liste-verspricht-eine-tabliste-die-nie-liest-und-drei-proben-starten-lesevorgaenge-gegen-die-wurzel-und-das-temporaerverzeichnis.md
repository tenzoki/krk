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
