# Die Git-Markenspalte lädt keine Cloud-Dateien herunter

## Directive

Die Markenspalte des Git-Bereichs soll in einem Repository auf einem Cloud-Laufwerk (Google Drive im Streaming-Modus) keine Datei herunterladen, die nur in der Cloud liegt. Auftrag des Nutzers vom 261026, gewählt mit „1“ auf die Möglichkeit: „Eine Behebung planen lassen, als neue Arbeit. Ein möglicher Weg: Die Markenspalte fasst Dateien, die nur in der Cloud liegen, nicht an und zeigt für sie „unbekannt“. Ob `gix` das zulässt, klärt der Plan.“ Dazu, wörtlich: „Der Ordner /Users/k1/Google\ Drive/Meine\ Ablage/shared/krk ist eine Wegwerfkopie.“ Er darf für Messungen und Proben frei verändert werden, auch mit Downloads.

Erreicht ist das Ziel, wenn `Gitleser::marken` an einer Repository-Kopie mit ausgelagerten Dateien zurückkehrt, ohne eine davon herunterzuladen, und die Spalte für diese Dateien eine erkennbare Marke „unbekannt“ zeigt statt einer falschen Aussage.

Grundlage: der Defekt `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md` mit seiner Messung vom 261026 (Commit `b3c78eed`): bei 261 ausgelagerten Dateien kehrte `marken` nach über zehn Minuten nicht zurück, die Fäden standen in `gix_status::index_as_worktree::traits::read_data::Stream::read_exact`, zwei Dateien wurden heruntergeladen.

## Abschluss

Geschlossen am 261026 als `done` (`completed`), Commits `b7ee81d4..dc6f4130` (`3c065f79`, `8c059dfc`, `dc6f4130`). Gebaut sind die Schritte 1 bis 3 des Plans; `make check` ist auf `dc6f4130` grün. **Die Abnahmemessung aus Schritt 4 ist nicht gelaufen**: Google Drive stand auf diesem Gerät im Zustand `UNAVAILABLE_WHILE_OFFLINE` und lieferte keine Datei aus, und in der Wegwerfkopie war inzwischen auch `.git/index` ausgelagert. Der Nutzer hat entschieden, ohne sie zu schließen, und alle zehn Bedingungen aus `## Where this work stops` mit „alles ok“ als geltend bestätigt. Der Sache nach offen bleiben damit: ob eine echte Platzhalterdatei ohne Download durchläuft, ob das Auslagern allein ctime hebt (gemessen an drei Dateien), und die Zahl ausgelagerter Dateien unter `.git`. Die Wirkung belegen allein die Proben in `crates/krk-core/tests/git.rs` (Abnahme (b)). Lokale Vergleichszahl: `marken` auf `crates/krk-core/src` 119 ms.

Abschlussdurchsicht `reviews/261026-1947-reviewer-abschlussdurchsicht-git-markenspalte-cloud-dateien.md`, Urteil `revise`, Bereich vollständig gedeckt. Ihre drei Befunde stehen offen unter `issues/`: Modulkopf und `HowTo.md` sagen, der Lauf lade nichts, während `gix` `.gitignore` und `.gitattributes` liest (vor der nächsten Auslieferung zu beheben); ein gescheitertes `lstat` gibt auch einer lokalen Datei `?`; drei Prüfkommentare nennen noch fünf Marken. Der Defekt `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md` bleibt offen, weil seine `Resolved:`-Zeile an Schritt 4 hing.
