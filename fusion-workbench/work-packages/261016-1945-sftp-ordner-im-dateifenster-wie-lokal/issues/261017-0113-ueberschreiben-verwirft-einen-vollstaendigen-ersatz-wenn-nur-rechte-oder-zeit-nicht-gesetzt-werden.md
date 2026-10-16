„Überschreiben“ verwirft einen vollständigen Ersatz, wenn nur Rechte oder Zeit nicht gesetzt werden
---
`kopie_ersetzt_ziel` hält den Ersatz für unvollständig, sobald seit seinem Beginn irgendetwas übersprungen wurde. `datei_fuellen` und `ordner_fuellen` überspringen aber auch dann, wenn nur `SETSTAT` für Rechte und Änderungszeit scheitert, und der Inhalt vollständig dasteht. Auf einem Server, der `SETSTAT` ablehnt, gelingt „Überschreiben“ deshalb nie: der Ersatz fällt weg, das Ziel bleibt. Ein Kopieren ohne Konflikt gelingt dort mit einem Hinweis. Dazu verbucht der Bericht denselben Eintrag als fertig und als übersprungen, und der Hinweis nennt den Pfad `.krk-ersatz-…` statt des Ziels.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (kein Datenverlust; „Überschreiben“ wirkt an solchen Servern nicht, und der Bericht ist widersprüchlich)
**Domain:** code
**Cross-references:** `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.md`

**Belege** (Code gelesen, nicht gegen einen solchen Server ausgeführt)

- `crates/krk-core/src/operation/fern.rs:958` (`datei_fuellen`): scheitert `angaben_setzen`, folgt `ueberspringen(&ziel.bezeichnung(), …VorgangDateiangabenNichtKopiert…)`. Das Ziel ist hier der Zwischenname. Danach folgt `eintrag_fertig` (`:967`).
- `fern.rs:1022` (`ordner_fuellen`): dieselbe Form mit `VorgangOrdnerangabenNichtKopiert`.
- `fern.rs:740` `kopie_ersetzt_ziel`: `if !steuerung.uebersprungen_seit(stand).is_empty() { rest_wegraeumen(…); …VorgangErsatzUnvollstaendig… }`.
- Bei `Tausch::ZielBlieb` und `Tausch::ZielFort` ist der Eintrag ebenfalls schon in `datei_fuellen` oder `ordner_fuellen` als fertig verbucht und wird danach übersprungen. Nur der Zweig für Verknüpfungen verbucht erst nach `Tausch::Getauscht`.

**Behebung (Richtung)**

- Für die Vollständigkeit allein die Gründe zählen, die Inhalt betreffen. Die zwei Angabengründe gehen nach dem Tausch mit dem Namen des Ziels in den Bericht, wie beim Kopieren ohne Konflikt.
- `eintrag_fertig` für Datei und Ordner erst nach `Tausch::Getauscht` verbuchen, wie für die Verknüpfung.

**Abnahme**

Eine Probe mit einer Attrappe oder einem Ziel, an dem `SETSTAT` scheitert, zeigt:
- „Überschreiben“ ersetzt das Ziel.
- Der Bericht nennt den Angabengrund unter dem Namen des Ziels.
- Kein Eintrag steht zugleich als fertig und als übersprungen im Bericht.

---
Resolved: `crates/krk-core/src/operation/fern.rs`: neuer Typ `Angabenweg` mit `angaben_uebernehmen`; `datei_fuellen` und `ordner_fuellen` sind in `datei_uebertragen` und `ordnerinhalt_fuellen` geteilt. `kopie_ersetzt_ziel` hält Hinweise auf Rechte und Zeit über `Angabenweg::Ersatz` bis zum Tausch zurück, zählt sie nicht zur Vollständigkeit und gibt sie nach dem Tausch unter dem Namen des Ziels weiter (`hinweise_nachtragen`); `eintrag_fertig` kommt für Datei, Ordner und Verknüpfung erst nach dem Tausch. Modulkopf „Rechte und Aenderungszeit zaehlen nicht zur Vollstaendigkeit des Ersatzes“. Probe `ueberschreiben_gelingt_auch_wenn_der_server_rechte_und_zeit_ablehnt` in `crates/krk-core/tests/fern_operation.rs` gegen `sftp-server -P setstat,fsetstat,lsetstat`, gegen den Stand vor der Behebung rot.
