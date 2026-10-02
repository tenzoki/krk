# Coder: der Schwungleser nimmt die Hülle `ohne_warten_oeffnen`

**Datum:** 260826-1930
**Agent:** coder
**Auftrag:** Schritt 2 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`
**Datensatz:** `shared/issues/260826-1221_o_der-schwungleser-oeffnet-mit-file-open-und-haengt-an-einer-benannten-roehre-fuer-immer.md` (bleibt auf `_o_`, der Orchestrator schließt beim Commit)
**Baum:** HEAD `26e8039`, ohne Commit
**Status:** Complete

## Rot vor grün

1. Die Probe `eine_benannte_roehre_ohne_schreiber_haelt_den_schwungleser_nicht_an` in `crates/krk-core/tests/verzeichnis.rs` zuerst geschrieben; dafür `mit_zeitschranke` aus `tests/text.rs` nach `tests/gemeinsam/mod.rs` gezogen (`pub fn`), die drei Rufer in `text.rs` bleiben und importieren sie aus `gemeinsam`.
2. Lauf am unveränderten `sys.rs` (`git diff --stat -- crates/krk-core/src/verzeichnis/sys.rs` leer):
   `cargo test -p krk-core --test verzeichnis eine_benannte_roehre_ohne_schreiber_haelt_den_schwungleser_nicht_an` — FAILED nach 5,01 s mit `Schwungleser::oeffnen ist nach 5s nicht zurueckgekommen; das Oeffnen haengt`. Der hängende Faden ist mit dem Prozess gestorben.
3. Behebung: `File::open(pfad)?` → `ohne_warten_oeffnen(pfad)?` in `Schwungleser::oeffnen`; die Typprüfung `metadata()?.is_dir()` unverändert.
4. Derselbe Lauf: `ok`, 0,00 s. Die Antwort ist `ErrorKind::NotADirectory` ohne Betriebssystemnummer, so wie die Probe verlangt.

## Geänderte Dateien

- `crates/krk-core/src/verzeichnis/sys.rs` — die Behebung; Doc-Kommentar von `oeffnen` sagt, warum; Abschnitt „Mehrere Aufrufer" von `ohne_warten_oeffnen` und der Modulkopf nennen den Verzeichnisleser als weiteren Aufrufer mit seiner Antwort.
- `crates/krk-core/tests/gemeinsam/mod.rs` — `mit_zeitschranke` als die eine Fassung, Begründung um den vierten Rufer erweitert.
- `crates/krk-core/tests/text.rs` — die lokale Fassung entfernt, Import aus `gemeinsam`.
- `crates/krk-core/tests/verzeichnis.rs` — die neue Probe.
- `CLAUDE.md` — allein der Absatz „Die Prüfung dessen, was da geöffnet wurde, steht am Deskriptor": der Verzeichnisleser als dritter Ort, keine Zahl.

Nicht angefasst: `inhalt_mit_zeitschranke` und die Handfassung in `eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an` (beide `tests/verzeichnis.rs`) tragen weiterhin je eine eigene Schranke; der Plan nennt allein den vierten Rufer. Wer sie zusammenzieht, hat jetzt die Fassung in `gemeinsam` dafür.

## Verifikation

`make check` im Arbeitsbaum ist rot an `genau_drei_pruefordner_fassungen_stehen_im_baum` (`tests/baum.rs`), Nadel `krk-core/src/operation/verschieben.rs` — das ist die parallel laufende, nicht committete Arbeit an Schritt 1, nicht dieser Schritt. Deshalb ein Worktree auf HEAD `26e8039` mit genau den fünf Dateien dieses Schritts und eigenem `CARGO_TARGET_DIR` (ein geteiltes `target/` hatte das Binär mit dem `CARGO_MANIFEST_DIR` des Hauptbaums wiederverwendet):

`make check` (Worktree, HEAD + die fünf Dateien) — exit 0. Alle Proben des Lesers, des Durchlaufs und des Umfangs grün, damit ist belegt, dass `O_NONBLOCK` am Verzeichnisdeskriptor `getattrlistbulk(2)` nicht erreicht.

Der Worktree ist wieder entfernt (`git worktree prune`).
