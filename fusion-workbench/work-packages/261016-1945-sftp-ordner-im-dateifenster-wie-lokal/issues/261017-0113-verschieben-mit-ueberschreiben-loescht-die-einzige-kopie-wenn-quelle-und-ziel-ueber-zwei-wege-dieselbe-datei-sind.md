Verschieben mit „Überschreiben“ löscht die einzige Kopie, wenn Quelle und Ziel über zwei Wege dieselbe Datei sind
---
`derselbe_eintrag` antwortet „nein“ für zwei verschiedene Kennungen und für lokal gegen fern. Zeigen beide Seiten trotzdem auf denselben Speicher, verläuft ein Verschieben mit „Überschreiben“ so: `kopie_ersetzt_ziel` kopiert vollständig unter einen Zwischennamen, `tauschen` entfernt das Ziel, also die Quelle, und benennt den Ersatz um. Danach entfernt `ueber_die_grenze` die Quelle, deren Pfad jetzt den Ersatz trägt. Die Datei ist damit an keinem Ort mehr vorhanden. Beim Kopieren ist derselbe Fall seit `545c8d31` sicher, beim Verschieben nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Datenverlust nach Bestätigung, aber nur unter einer seltenen Vorbedingung; bestand schon vor der Behebung)
**Domain:** code
**Cross-references:** `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.md`, `261017-0004-ueberschreiben-zerstoert-die-quelle-wenn-ziel-und-quelle-auf-dem-server-ueber-eine-verknuepfung-derselbe-eintrag-sind.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/fern.rs`, `Stelle::derselbe_eintrag`: `if kennung != andere_kennung { return Gleichheit::Nein; }`. Lokal gegen fern ergibt immer `Gleichheit::Nein`.
- `crates/krk-core/src/fern/adresse.rs`, Doc an `Serverkennung`: „`ssh.strato.de` und `ssh.strato.de:22` sind deshalb zwei Kennungen, obwohl sie meist denselben Server nennen“.
- `fern.rs:740` `kopie_ersetzt_ziel` → `tauschen` (`:820`): entfernt das Ziel, dann `umbenennen_in`.
- `fern.rs:1339` `ueber_die_grenze`: findet nichts Übersprungenes und ruft `quelle.baum_entfernen(pfand, &nie_abgebrochen)`.
- Vorbedingung, Beispiele:
  - zwei Tabs auf `rechner` und `rechner:22`;
  - ein NAS, lokal per SMB unter `/Volumes/…` eingehängt und zugleich per sftp geöffnet;
  - sftp auf `localhost`.
- Der Modulkopf (`fern.rs:97`) nennt zwei Kennungen als Grenze, nicht aber den Datenverlust beim Verschieben.

**Behebung (Richtung)**

Nach dem Entfernen des Ziels in einem Verschieben über eine Grenze fragen, ob die Quelle noch steht (`LSTAT`, lokal `symlink_metadata`). Fehlt sie, waren beide derselbe Eintrag. Dann den Ersatz umbenennen, die Quelle nicht entfernen und den Eintrag mit eigenem Grund melden. Mindestens aber den Datenverlust im Modulkopf und in `HowTo.md` beim Überschreiben ausdrücklich nennen.

**Abnahme**

Eine Probe gegen `/usr/libexec/sftp-server`:
- Eine Datei unter einem Prüfordner wird lokal gegen fern mit demselben Pfad verschoben, oder zwischen den Kennungen `rechner` und `rechner:22`.
- Die Antwort ist „Überschreiben“.
- Danach steht die Datei mit ihrem Inhalt am Ziel.

---
Resolved: `tauschen` in `crates/krk-core/src/operation/fern.rs` fragt beim Kopieren und beim Verschieben über eine Grenze nach dem Beiseitelegen des Ziels, ob die Quelle noch unter ihrem Namen steht (`LSTAT`, lokal `symlink_metadata`). Fehlt sie, kommt das Ziel zurück, der Ersatz fällt weg, und der Eintrag steht mit dem neuen Text `VorgangQuelleIstTeilDesZiels` im Bericht (`Tausch::QuelleBeiseite`); das Verschieben entfernt die Quelle dann nicht. Regel und Grenze (zwei harte Verweise) im Modulkopf unter „Liegt mit dem Ziel auch die Quelle beiseite“ und „Was dieses Modul nicht haelt“, dazu `HowTo.md` im sftp-Abschnitt. Probe `verschieben_mit_ueberschreiben_verliert_keine_quelle_die_ueber_zwei_wege_das_ziel_ist` in `crates/krk-core/tests/fern_operation.rs` (lokal nach fern, fern nach lokal, zwei Kennungen verschieben und kopieren), gegen den Stand vor der Behebung rot.
