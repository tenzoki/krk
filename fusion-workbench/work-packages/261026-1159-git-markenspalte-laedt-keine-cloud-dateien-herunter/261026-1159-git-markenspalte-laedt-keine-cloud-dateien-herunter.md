# Die Git-Markenspalte lädt keine Cloud-Dateien herunter

## Directive

Die Markenspalte des Git-Bereichs soll in einem Repository auf einem Cloud-Laufwerk (Google Drive im Streaming-Modus) keine Datei herunterladen, die nur in der Cloud liegt. Auftrag des Nutzers vom 261026, gewählt mit „1“ auf die Möglichkeit: „Eine Behebung planen lassen, als neue Arbeit. Ein möglicher Weg: Die Markenspalte fasst Dateien, die nur in der Cloud liegen, nicht an und zeigt für sie „unbekannt“. Ob `gix` das zulässt, klärt der Plan.“ Dazu, wörtlich: „Der Ordner /Users/k1/Google\ Drive/Meine\ Ablage/shared/krk ist eine Wegwerfkopie.“ Er darf für Messungen und Proben frei verändert werden, auch mit Downloads.

Erreicht ist das Ziel, wenn `Gitleser::marken` an einer Repository-Kopie mit ausgelagerten Dateien zurückkehrt, ohne eine davon herunterzuladen, und die Spalte für diese Dateien eine erkennbare Marke „unbekannt“ zeigt statt einer falschen Aussage.

Grundlage: der Defekt `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md` mit seiner Messung vom 261026 (Commit `b3c78eed`): bei 261 ausgelagerten Dateien kehrte `marken` nach über zehn Minuten nicht zurück, die Fäden standen in `gix_status::index_as_worktree::traits::read_data::Stream::read_exact`, zwei Dateien wurden heruntergeladen.
