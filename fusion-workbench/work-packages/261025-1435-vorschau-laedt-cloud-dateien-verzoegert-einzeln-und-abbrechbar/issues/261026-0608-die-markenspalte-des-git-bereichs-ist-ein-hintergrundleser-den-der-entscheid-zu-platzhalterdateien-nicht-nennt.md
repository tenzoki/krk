Die Markenspalte des Git-Bereichs ist ein Hintergrundleser, den der Entscheid zu Platzhalterdateien nicht nennt
---
`Gitleser::marken` fährt bei jedem Ordner in einem Repository `gix`-Status über den Arbeitsbaum; ob das eine Datei mit `SF_DATALESS` öffnet, ist ungeprüft.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`

**Befund.** Der Entscheid `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md` (Möglichkeit 1, beantwortet) bindet „jeden künftigen Leser“: kein Hintergrundleser lädt eine Platzhalterdatei. Er nennt drei Leser (Aufnahmedatum, Inhaltsfilter, Profilwerte), und das Paket hat genau diese drei umgestellt. Ein vierter steht im Baum: `Gitleser::marken` in `crates/krk-core/src/git/leser.rs` (gerufen aus `crates/krk-core/src/git/lauf.rs`, `leser.marken(ordner, abbruch)`) fährt `repo.status(…).into_iter(muster)` für jeden gelesenen Ordner innerhalb eines Repositorys, also ohne dass der Nutzer eine Datei gewählt hat.

**Ungeprüft.** `inference:` der Index-gegen-Arbeitsbaum-Vergleich von `gix` liest und hasht eine verfolgte Datei, wenn ihre `stat`-Angaben vom Index abweichen. `inference:` das Auslagern durch den File Provider setzt `SF_DATALESS` über eine Änderung der Kennzeichen und hebt damit `st_ctime`. Treffen beide zu, lädt die Markenspalte jede ausgelagerte verfolgte Datei eines Ordners herunter, sobald der Nutzer ihn öffnet. Keines der beiden ist gemessen; der Reviewer hat kein Repository auf einem Cloudlaufwerk geöffnet.

**Abnahme.** Eines von beiden: (a) eine Messung an einem Repository auf dem Cloudlaufwerk des Nutzers, mit seiner Zustimmung, zeigt mit `ls -lO` vor und nach dem Öffnen des Ordners in KRK, dass keine verfolgte Datei ihr `dataless` verliert, und der Modulkopf von `git/leser.rs` hält das Ergebnis fest; oder (b) die Markenspalte öffnet keine Datei mit `SF_DATALESS` und nennt sie wie der Inhaltsfilter als ungelesen, mit einer Probe an einem eingespritzten Kennzeichen.
