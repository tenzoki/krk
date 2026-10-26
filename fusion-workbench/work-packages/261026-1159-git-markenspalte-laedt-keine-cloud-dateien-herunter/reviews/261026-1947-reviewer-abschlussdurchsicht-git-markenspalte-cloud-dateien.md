# Abschlussdurchsicht: Die Git-Markenspalte lädt keine Cloud-Dateien herunter

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `b7ee81d4..dc6f4130`
**Not-opened:** none
**Review domain:** code
**Work-item:** `261026-1159-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`
**Cross-references:** `261026-1208-plan-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`, `261026-1208-zeigt-die-markenspalte-jede-ausgelagerte-datei-als-unbekannt-oder-nur-die-deren-stat-angaben-ausser-ctime-vom-index-abweichen.md`, `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md`

Geöffnet: alle Codedateien des Bereichs (`git/leser.rs`, `git/lauf.rs`, `git/mod.rs`, `git/texte.rs`, `sprache/schluessel.rs`, `sprache/tabelle/{de,fr,en}.rs`, `tests/git.rs`) und `HowTo.md`. Die zwei Werkbankdateien des Diffs (`orchestrator-events.jsonl`, die Steuerdatei des Plans) sind Buchführung und keine Gegenstände der Durchsicht; den Plan-Steuerstand habe ich gelesen. Dazu gegengelesen im Cargo-Register: `gix-status-0.34.1/src/index_as_worktree/{function.rs,traits.rs}`, `gix-0.87.1/src/status/{mod.rs,platform.rs,index_worktree.rs,iter/mod.rs}`, `gix-0.87.1/src/repository/{pathspec.rs,dirwalk.rs}`, `gix-index-0.55.0/src/entry/stat.rs`, `gix-worktree-0.56.0/src/stack/state/{attributes.rs,ignore.rs}`. Nichts ausgeführt. `make check` auf `dc6f4130` grün laut Auftrag, nicht von mir gefahren.

## Summary

Der Umbau tut im Inhaltsvergleich, was Plan und E-Stat verlangen. Dort liest kein Weg den Inhalt einer Datei, die die Platzhalterfrage als ausgelagert meldet. Der Abbruch erreicht `gix` und greift auch zwischen den beiden Läufen. Gegenüber `Platform::into_iter` ändert sich für lokale Dateien keine Marke. Der Anspruch in Modulkopf und `HowTo.md` geht aber über den Inhaltsvergleich hinaus: zwei Wege in `gix` öffnen weiter Dateien des Arbeitsbaums (`.gitignore` und `.gitattributes`), die ausgelagert sein können. Ohne die Messung aus Schritt 4 ist offen, ob sie dabei herunterladen.

## Totals

| Schwere | Zahl | Defekte |
|---|---|---|
| Critical | 0 | |
| High | 0 | |
| Medium | 1 | F1 |
| Low | 2 | F2, F3 |

## Befunde

### F1 (Medium): Zwei Wege öffnen ausgelagerte Dateien außerhalb des Inhaltsvergleichs, und Modulkopf und Anleitung sagen es nicht

`leser.rs`, Abschnitt `# Eine Datei, die nur in der Cloud liegt`, erster Satz: „Der Markenlauf oeffnet keine ausgelagerte Datei“. `HowTo.md`, Git-Bereich: „lädt die Markenspalte nicht herunter“.

- `.gitignore`: `index_worktree_status` legt den Ausschluss-Stapel auf `WorktreeThenIdMappingIfNotSkipped`. Der Durchlauf liest jede `.gitignore` über `add_patterns_file` (`gix-worktree-0.56.0/src/stack/state/ignore.rs`). Der Modulkopf nennt das in einem Halbsatz ohne die Folge. `HowTo.md` nennt es nicht.
- `.gitattributes`: `ReadDataImpl::stream_worktree_file` ruft vor dem Öffnen einer **lokalen** Datei `attr_stack.at_entry`, auf `attributes::Source::WorktreeThenIdMapping` (`gix-status-0.34.1/src/index_as_worktree/function.rs`). Damit wird jede `.gitattributes` auf ihrem Pfad gelesen. Das passiert, sobald eine lokale Datei mit abweichendem stat und gleicher Größe gehasht wird. Weder Plan noch Modulkopf nennen diesen Weg.

Beide sind Inferenz aus dem Quelltext und nicht gemessen. Schritt 4 sollte den ersten Weg messen und ist nicht gelaufen.
Defekt: `261026-1947-der-markenlauf-liest-ausgelagerte-gitignore-und-gitattributes-und-modulkopf-und-anleitung-sagen-er-lade-nichts.md`.

### F2 (Low): `?` entsteht auch aus einem gescheiterten `lstat`, und drei Doc-Stellen sagen „nur in der Cloud“ bzw. „jeder Fehlschlag führt zu `None`“

`compare_blobs` gibt bei gescheitertem `symlink_metadata` `Befund::Ungelesen` zurück, bevor die Platzhalterfrage gestellt ist. `ausgelagert_deuten` tut das bei gescheitertem `from_path_no_follow` und `Stat::from_fs`. Der Plan wollte das so. Falsch sind die Doc-Kommentare von `Gitleser::marken` („Jeder Fehlschlag fuehrt dorthin“), `Marke::Unbekannt` und `Befund::Ungelesen`.
Defekt: `261026-1947-ein-gescheitertes-lstat-im-inhaltsvergleich-gibt-auch-einer-lokalen-datei-ein-fragezeichen-und-drei-doc-stellen-sagen-anderes.md`.

### F3 (Low): drei Prüfkommentare mit überholter Zahl oder falschem Wortlaut

`fuenf_marken` in `verzeichnis/modell.rs` („in der Reihenfolge von `Marke::ALLE`“), die Zwischenüberschrift `C5: die fuenf Markenzustaende` in `tests/git.rs`, und der Doc-Kommentar von `verfolgt` (`grund\n` statt `grund!\n`).
Defekt: `261026-1947-drei-pruefkommentare-nennen-nach-marke-unbekannt-noch-fuenf-marken-oder-einen-falschen-dateiinhalt.md`.

## Geprüft und ohne Befund

**Liest der Inhaltsvergleich eine ausgelagerte Datei?** Nein. `compute_status` erreicht den Inhalt einer verfolgten Datei allein über das `ReadData`, das es `compare_blobs` reicht (`read_blob` liest aus der Objektdatenbank, `stream_worktree_file` öffnet die Datei). Im Zweig für ausgelagerte Dateien nimmt `Inhaltsvergleich` `data` nicht an. `ausgelagert_deuten` bekommt es gar nicht. Im Einzelnen:

- *Filter:* `convert_to_git` steht nur in `stream_worktree_file` und wird für eine ausgelagerte Datei nie erreicht.
- *Symlinks:* gefragt wird das `lstat` des Verweises selbst. Ist er nicht ausgelagert, liest `FastEq` über `read_link` und lädt nichts.
- *Submodule:* ein Verzeichnis geht in `compute_status` vor dem Vergleich an `SubmoduleStatus`. Mit `Ignore::All` und `check_dirty: false` ist das wie vorher.
- *Racy:* ein racy Eintrag ruft den Vergleich auch bei passendem stat. `ausgelagert_deuten` fragt dann `is_racy` und antwortet `Ungelesen`. `is_racy` wird wie in `gix` am neuen stat gefragt und liest allein mtime, unabhängig von `trust_ctime`.
- *Ausführbarkeit:* bei einem geänderten Ausführbarkeitsbit ruft `gix` den Vergleich ohne stat-Prüfung. Das Ergebnis ist „M“ über den Arm `executable_bit_changed: true`, wie im Plan.
- *Index-Größe 0 nach Racy-Schmieren:* `stat.matches` scheitert an der Größe, also `?` und kein falsches „unverändert“.

**E-Stat Möglichkeit 2.** Umgesetzt wie entschieden. `trust_ctime = false` gilt allein in `Inhaltsvergleich.stat`, also allein für ausgelagerte Dateien. Die Größenregel ist die von `FastEq` (`traits.rs`, wörtlich gleich). `Stat::matches` wird mit den übrigen Schaltern des Repositorys gefragt, prüft also neben mtime, Größe und Inode auch uid, gid und je nach `use_stdev` das Gerät. Das ist dieselbe Menge, die der Entscheid „die übrigen Schalter wie im Repository“ nennt. Die Probe (1) hält den Ausgang ohne Marke mit einer Gegenprobe, die zeigt, dass `gix` den Vergleich an dieser Datei fragt.

**Abbruch.** `abbruch` geht unverändert an `index_worktree_status`. `gix` fragt es je Eintrag im Chunk-Lauf (`function.rs`, Kommentar „cooperative early-out within a chunk“). Zwischen den beiden Läufen steht eine eigene Frage (`marken_mit`, nach `tree_index_status`). Nach der Rückkehr wird noch einmal gefragt, und es kommt `None`. Nicht unterbrechbar bleiben `index_from_tree` im ersten Lauf (lokale `.git`-Daten) und das Hashen einer einzelnen lokalen Datei, beides benannt. Die Probe (5) prüft nur den Abbruch im zweiten Lauf. Den `Break` im Rückruf des ersten Laufs prüft keine Probe. Das ist kein Defekt, weil die Frage danach ihn auffängt.

**Regression gegen `Platform::into_iter`.** Keine gefunden. Abgelesen an `Repository::status` und `Platform::untracked_files`: `rewrites: None`, `sorting: None`, `thread_limit: None`, `dirwalk_options` aus `dirwalk_options()` mit `EmissionMode::Matching`, `TrackRenames::AsConfigured` (`Default`), dasselbe `Submodule::Given`. Das Pfadmuster des ersten Laufs entstand vorher aus `index_worktree_status_pathspec` und jetzt aus `Repository::pathspec`. Beide setzen `empty_patterns_match_prefix` auf `false` (`dirwalk/options.rs`), erben die Groß- und Kleinschreibung und fragen Attribute aus derselben Quelle. Die Ausschlüsse fehlen, und für ein reines Präfixmuster spielen sie keine Rolle. `Entry::Rewrite` bleibt bei `rewrites: None` unerreichbar, wie vorher. `NeedsUpdate` kommt jetzt an und bleibt ohne Marke. `Change::Modification { executable_bit_changed: false, content_change: None }` erzeugt `compute_status` nicht. Der Arm, der es auf „M“ legt, ist also unerreichbar und schadet nicht. Die vorhandenen Markenproben gehen weiter über `marken` mit der Betriebsfrage und halten den lokalen Weg.

**Fallunterscheidungen, `ALLE`, Sprachtabelle.** `buchstabe`, `rang`, `wort`, `posten_deuten`, `aenderung_deuten` und `indexaenderung_deuten` sind vollständig und ohne Auffangzweig. `Marke::ALLE` hat sechs Werte, `Zahlwort::ALLE` 45 (einer mehr), alle drei Tabellen tragen `MarkeUnbekannt`. Neue Literale mit Umlaut stehen außerhalb der Tabellen nur in Kommentaren. `marken_mit` trägt `#[must_use]`. Einen Untergrenzen-Abschnitt braucht es nicht, weil `krk-core` nicht unter `appkit/` liegt. Die Liste der angesprochenen `gix`-Wege im Modulkopf ist vollständig gegen die `use`-Zeilen und Aufrufe.

**`HowTo.md`.** Der neue Absatz beschreibt die drei Ausgänge und die Ordnerregel so, wie sie gebaut sind, F1 ausgenommen. „so kurz vor dem letzten Auffrischen des Index geändert“ ist die richtige Lesart von racy: mtime nicht vor dem Zeitstempel des Index.

## Tragen die Proben den Kernanspruch?

**Für den Inhaltsvergleich ja, für den ganzen Lauf nein.** Die Proben (1) bis (4) belegen mit `chmod 000` und Gegenprobe: liefert die Platzhalterfrage *ja*, öffnet der Lauf die Datei nicht, und die Marke folgt dem Bild im Modulkopf. Das ist der Weg, über den die Messung vom 261026 heruntergeladen hat (`read_data::Stream::read_exact`). Drei Dinge belegen sie nicht:

1. dass `verzeichnis::ist_platzhalter` an einer echten Platzhalterdatei *ja* sagt. Das hält die Probe des vorigen Pakets, nur an gesetzten Kennzeichen und nicht an einer File-Provider-Datei;
2. dass Google Drive beim Auslagern allein ctime hebt. Gemessen ist das an drei Dateien, und genau dafür war Schritt 4 (d) gedacht;
3. die zwei Lesewege aus F1, die außerhalb von `compare_blobs` liegen und die `chmod 000` an einer `wolke.txt` nicht berühren kann.

Wegen Punkt 2: weicht beim Auslagern eine weitere stat-Angabe ab (mtime, Inode), zeigt die Spalte `?` statt keiner Marke. Herunterladen würde sie trotzdem nicht. Ein Fehlschlag von Punkt 2 macht die Auskunft also gröber, bricht aber nicht die Zusage.

## Zum Abschluss des Pakets

- Schritt 4 ist im Plan auf `in_progress`. Der Defekt `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md` steht auf `open`, und seine `Resolved:`-Zeile war an Schritt 4 gebunden. Wer das Paket ohne Messung schließt, braucht für beide einen Vermerk: Abnahme (b) über die Proben, Abnahme (a) ungemessen.
- Bis zu einer Messung gilt laut „Where this work stops“: das Paket ist gebaut, aber nicht abgenommen.

## Querschnitt

F1 und F2 haben dasselbe Muster: die Doku formuliert die Zusage weiter, als der Code sie hält. F1 sagt „der Lauf“, gebaut ist „der Vergleich“. F2 sagt „Cloud“, gebaut ist „nicht erneut lesbar“. Die Zusage im Code stimmt jeweils, nur ihr Umfang ist zu weit beschrieben.

## Reihenfolge

Kein Befund blockiert den Abschluss, wenn der Nutzer ohne Messung schließt. F1 vor der nächsten Auslieferung beheben, weil `HowTo.md` mit dem Releasepaket zum Nutzer geht. F2 und F3 sind Aufräumarbeit.
