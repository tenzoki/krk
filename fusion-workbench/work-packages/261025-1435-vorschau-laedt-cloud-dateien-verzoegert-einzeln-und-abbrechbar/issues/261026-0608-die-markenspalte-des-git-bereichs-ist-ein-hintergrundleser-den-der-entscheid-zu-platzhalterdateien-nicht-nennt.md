Die Markenspalte des Git-Bereichs ist ein Hintergrundleser, den der Entscheid zu Platzhalterdateien nicht nennt
---
`Gitleser::marken` fährt bei jedem Ordner in einem Repository `gix`-Status über den Arbeitsbaum; ob das eine Datei mit `SF_DATALESS` öffnet, ist ungeprüft.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`

**Befund.** Der Entscheid `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md` (Möglichkeit 1, beantwortet) bindet „jeden künftigen Leser“: kein Hintergrundleser lädt eine Platzhalterdatei. Er nennt drei Leser (Aufnahmedatum, Inhaltsfilter, Profilwerte), und das Paket hat genau diese drei umgestellt. Ein vierter steht im Baum: `Gitleser::marken` in `crates/krk-core/src/git/leser.rs` (gerufen aus `crates/krk-core/src/git/lauf.rs`, `leser.marken(ordner, abbruch)`) fährt `repo.status(…).into_iter(muster)` für jeden gelesenen Ordner innerhalb eines Repositorys, also ohne dass der Nutzer eine Datei gewählt hat.

**Ungeprüft.** `inference:` der Index-gegen-Arbeitsbaum-Vergleich von `gix` liest und hasht eine verfolgte Datei, wenn ihre `stat`-Angaben vom Index abweichen. `inference:` das Auslagern durch den File Provider setzt `SF_DATALESS` über eine Änderung der Kennzeichen und hebt damit `st_ctime`. Treffen beide zu, lädt die Markenspalte jede ausgelagerte verfolgte Datei eines Ordners herunter, sobald der Nutzer ihn öffnet. Keines der beiden ist gemessen; der Reviewer hat kein Repository auf einem Cloudlaufwerk geöffnet.

**Abnahme.** Eines von beiden: (a) eine Messung an einem Repository auf dem Cloudlaufwerk des Nutzers, mit seiner Zustimmung, zeigt mit `ls -lO` vor und nach dem Öffnen des Ordners in KRK, dass keine verfolgte Datei ihr `dataless` verliert, und der Modulkopf von `git/leser.rs` hält das Ergebnis fest; oder (b) die Markenspalte öffnet keine Datei mit `SF_DATALESS` und nennt sie wie der Inhaltsfilter als ungelesen, mit einer Probe an einem eingespritzten Kennzeichen.

## Messung vom 261026

Gemessen mit Zustimmung des Nutzers an einer Kopie des Repositorys unter `~/Google Drive/Meine Ablage/shared/krk` (Arbeitskopie mit `.git`, Index vorher per `git status` bei lokalen Dateien aufgefrischt). Alle 261 Dateien unter `crates/` über `FileManager.evictUbiquitousItem` ausgelagert (`find -flags +dataless`: 261 von 261). Dann `Gitleser::oeffnen` und `Gitleser::marken` aus `krk-core` 3.11.1 auf `crates/krk-core/src`, in einem eigenen Prüfprogramm außerhalb des Baums.

- Der Lauf kehrte nach über zehn Minuten nicht zurück und wurde abgebrochen.
- `sample` zeigt die Arbeitsfäden von `gix_status::index_as_worktree` in `read_data::Stream::read_exact`: gix liest den Inhalt der ausgelagerten verfolgten Dateien.
- In dieser Zeit sind zwei Dateien heruntergeladen worden (`crates/krk-core/src/ablage/atomar.rs`, `crates/krk-core/src/trenntext/bereich.rs`); 259 blieben ausgelagert.

Befund damit bestätigt und nicht mehr Vermutung: die Markenspalte lädt in einem Repository auf einem Cloudlaufwerk jede ausgelagerte verfolgte Datei herunter, eine nach der anderen, und hält den Faden dabei minutenlang fest. Ob es `st_ctime` oder eine andere Angabe ist, die nach dem Auslagern vom Index abweicht, ist nicht gemessen.

## Stand vom 261026, nach dem Umbau

Behoben im Code durch das Paket `261026-1159-git-markenspalte-laedt-keine-cloud-dateien-herunter` (`3c065f79`, `8c059dfc`): der Inhaltsvergleich öffnet keine ausgelagerte Datei mehr, urteilt aus stat ohne ctime und zeigt `?`, wo eine Änderung möglich ist; der Abbruch erreicht `gix`. Abnahme (b) ist mit den Proben in `crates/krk-core/tests/git.rs` belegt. **Abnahme (a), die Messung an Google Drive, ist ungemessen**, weil Drive offline stand; dieser Defekt bleibt deshalb offen, bis sie läuft. Dabei auch zu messen: ob `gix` eine ausgelagerte `.gitignore` oder `.gitattributes` herunterlädt (`261026-1947-der-markenlauf-liest-ausgelagerte-gitignore-und-gitattributes-und-modulkopf-und-anleitung-sagen-er-lade-nichts.md`).
