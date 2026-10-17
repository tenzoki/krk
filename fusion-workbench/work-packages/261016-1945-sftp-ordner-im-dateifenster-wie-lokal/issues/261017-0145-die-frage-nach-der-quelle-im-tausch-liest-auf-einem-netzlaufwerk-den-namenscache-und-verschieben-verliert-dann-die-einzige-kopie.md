Die Frage nach der Quelle im Tausch liest auf einem Netzlaufwerk den Namenscache, und Verschieben verliert dann die einzige Kopie
---
`tauschen` erkennt eine Quelle, die über einen zweiten Weg das Ziel ist, daran, dass sie nach dem Beiseitelegen des Ziels unter ihrem Namen fehlt (`quelle.angaben`, lokal `symlink_metadata`). Liegt die lokale Seite auf einem eingehängten Netzlaufwerk (NFS, SMB), kann der Client den Namen aus seinem Cache beantworten, obwohl der Server ihn eben über sftp umbenannt hat. Dann wird getauscht, das alte Ziel (die Quelle) entfernt, und `ueber_die_grenze` entfernt danach den Namen der Quelle, unter dem jetzt der Ersatz steht. Die Datei ist an keinem Ort mehr.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Verlust der einzigen Kopie nach „Überschreiben“, unter einer seltenen Vorbedingung; `HowTo.md` sagt für genau diesen Fall „die Quelle bleibt“ ohne Einschränkung)
**Domain:** code
**Cross-references:** `261017-0145-reviewer-schlussdurchsicht-tausch-a97e4fbb-bis-0284ace2.md`, `261017-0113-verschieben-mit-ueberschreiben-loescht-die-einzige-kopie-wenn-quelle-und-ziel-ueber-zwei-wege-dieselbe-datei-sind.md`

**Belege** (Code gelesen; das Cacheverhalten ist `inference:` aus dem NFS- und SMB-Client, nicht gemessen)

- `crates/krk-core/src/operation/fern.rs`, `tauschen`: Schritt 1 `beiseite_legen` über die ferne Stelle (sftp), dann `quelle.angaben(pfand)` über die lokale Stelle. Allein ein Fehler mit `ist_nicht_da` ergibt `Tausch::QuelleBeiseite`.
- Ein umbenannter Eintrag behält auf dem Server sein Dateihandle. Ein Client, dessen Namenseintrag `x → Handle` noch gilt, beantwortet `lstat(x)` mit den Angaben des Eintrags, der jetzt `.krk-alt-…` heißt. `inference:` die Gültigkeit hängt an der Frist für Ordnerangaben des Clients, und zwischen Schritt 1 und der Frage liegen Millisekunden.
- Danach in `tauschen`: Ersatz auf den Namen, `alt.baum_entfernen` entfernt das alte Ziel, also die Daten der Quelle.
- `fern.rs`, `ueber_die_grenze`: nichts übersprungen, also `quelle.baum_entfernen(pfand, &nie_abgebrochen)`. Das `REMOVE` geht mit dem Namen an den Server und trifft den Ersatz.
- Die Probe `verschieben_mit_ueberschreiben_verliert_keine_quelle_die_ueber_zwei_wege_das_ziel_ist` (`crates/krk-core/tests/fern_operation.rs`) fährt den lokalen Weg auf einer lokalen Platte ohne Client-Cache und kann den Fall nicht zeigen.
- `HowTo.md`, sftp-Abschnitt: „Erreichen beide Seiten denselben Speicher auf zwei Wegen, etwa ein NAS, das eingehängt und zugleich über sftp geöffnet ist, fragt KRK zwar nach, überschreibt aber nichts: die Quelle bleibt“. Das NAS-Beispiel ist genau der Fall mit Cache.

**Behebung (Richtung)**

Die Frage „steht die Quelle noch“ ist über einen Client mit Namenscache nicht entscheidbar. Eine Frage, die sich aus den Eingaben beantworten lässt: liegt die lokale Seite auf einem Dateisystem ohne `MNT_LOCAL` (`statfs`), gilt der Befund von `derselbe_eintrag` als unklar, und `ersetzen_erlaubt` lässt beim Verschieben über eine Grenze nicht ersetzen, wie bei einem `REALPATH` ohne Antwort. Mindestens aber die Grenze im Modulkopf unter „Was dieses Modul nicht haelt“ und in `HowTo.md` nennen.

**Abnahme**

- Ein Verschieben mit „Überschreiben“ zwischen einer lokalen Quelle auf einem Netzlaufwerk und einem fernen Ziel entfernt die Quelle nie, bevor die Frage nach ihr eine Antwort hat, die nicht aus einem Client-Cache stammt; oder es ersetzt dort gar nicht und sagt es im Bericht.
- Modulkopf und `HowTo.md` behaupten für zwei Wege nichts, was der Code dort nicht hält.

---
Resolved: Verschieben über eine Grenze mit „Überschreiben“ ersetzt nicht mehr, wenn die lokale Seite (Eintrag oder sein Ordner, Quelle wie Ziel) kein `MNT_LOCAL` trägt oder sich nicht fragen lässt; der Eintrag steht mit eigenem Grund (`Text::VorgangNetzlaufwerkNichtErsetzt`, de/fr/en) bzw. mit `VorgangZielNichtErsetzt` und dem Systemgrund im Bericht. Kopieren mit „Überschreiben“ bleibt unverändert. `crates/krk-core/src/operation/fern.rs`: `ersetzen_erlaubt` mit neuem Parameter `Zweck`, reine Regel `netzbefund`, Abfrage `lokale_seite_liegt_lokal`; `Zweck` durch `eintrag_kopieren`, `ordner_fuellen`, `ordnerinhalt_fuellen` gereicht, `ueber_die_grenze` setzt `Zweck::Verschieben`. Neue Systemabfrage `verzeichnis::sys::auf_lokalem_dateisystem` über `statfs(2)` (x86_64: `statfs$INODE64`), ohne neue Kiste und ohne `libc`; Zählzeilen in `sys.rs`, `lib.rs`, `verzeichnis/mod.rs` auf neun Schnittstellen und dreizehn Bindungen nachgezogen. Modulkopf (`Liegt mit dem Ziel auch die Quelle beiseite`, `Was dieses Modul nicht haelt`) und `HowTo.md` (sftp-Abschnitt, Satz zu zwei Wegen) nachgezogen. Proben: `verschieben_ersetzt_nicht_ueber_ein_netzlaufwerk_und_kopieren_fragt_nicht` (fern.rs), `die_wurzel_liegt_lokal_und_ein_fehlender_pfad_ist_ein_fehler` und `struct_statfs_hat_die_form_aus_sys_mount_h` (sys.rs). Ein echtes Netzlaufwerk fährt keine Probe.
