„Überschreiben“ entfernt einen Zielordner zur Hälfte und wirft dann den vollständigen Ersatz weg
---
`tauschen` entfernt das Ziel mit `baum_entfernen`, bevor es den fertigen Ersatz umbenennt. Ist das Ziel ein Ordner und scheitert das Entfernen an einem Kind, sind die Kinder davor schon fort. Beim Kopieren räumt `kopie_ersetzt_ziel` danach den vollständigen Ersatz weg. Am Ende steht ein halber Zielordner und kein Ersatz, und der Bericht sagt „Ziel nicht ersetzt“. Der Modulkopf sagt für diesen Fall, das Ziel bleibe, wie es war.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Ziel halb zerstört, Ersatz verworfen; dieselbe Klasse wie der behobene Befund `261017-0004-jedes-scheitern-beim-fernen-anlegen-…`, unter engerer Vorbedingung)
**Domain:** code
**Cross-references:** `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.md`, `261017-0004-jedes-scheitern-beim-fernen-anlegen-gilt-als-konflikt-und-ueberschreiben-entfernt-dann-ein-ziel-ohne-es-zu-ersetzen.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/fern.rs:820` `tauschen`: zuerst `ziel.baum_entfernen(pfand, &nie_abgebrochen)`, bei `Err` → `Tausch::ZielBlieb(fehler)`.
- `fern.rs:2251` `fern_baum_entfernen` hält am ersten Fehler an. Was vorher entfernt wurde, bleibt entfernt. Lokal gilt dasselbe für `loeschen::baum_entfernen` (`crates/krk-core/src/operation/loeschen.rs:124`).
- `fern.rs:785` `kopie_ersetzt_ziel`, Zweig `Tausch::ZielBlieb`: ruft `rest_wegraeumen(pfand, &zwischen, …)` und entfernt den vollständigen Ersatz.
- `fern.rs:810` (Doc an `Tausch::ZielBlieb`): „Das Ziel liess sich nicht entfernen und steht unveraendert“. `fern.rs:69` (Modulkopf): „scheitert das Entfernen, ebenso [bleibt das Ziel, wie es war]“. Beides gilt nur für ein Ziel, das eine Datei ist.
- Auslöser: ein Zielordner mit einem Unterordner ohne Schreibrecht oder einem gesperrten Eintrag, überschrieben von einer Datei oder einem Ordner gleichen Namens, der keine Zusammenführung auslöst (Datei auf Ordner).

**Behebung (Richtung)**

Das Ziel zuerst beiseite benennen und nicht zuerst entfernen:
1. `RENAME ziel → .krk-alt-<pid>-<n>`. Scheitert das, bleibt das Ziel wirklich unverändert.
2. `RENAME zwischen → ziel`. Scheitert das, kommt das alte Ziel per `RENAME` zurück.
3. Danach das beiseitegelegte Ziel endgültig entfernen. Bleibt dabei etwas stehen, nennt der Bericht den Namen `.krk-alt-…`, und der Ersatz steht schon an seinem Platz.

Ein `RENAME` eines Ordners braucht nur Schreibrecht im Elternordner, nicht in seinem Inhalt. Das Zeitfenster ohne Ziel aus dem Modulkopf entfällt damit. Modulkopf und Doc an `Tausch` danach nachziehen.

**Abnahme**

Eine Probe in `crates/krk-core/tests/fern_operation.rs` gegen `/usr/libexec/sftp-server`:
- Am Ziel steht ein Ordner `d/` mit `a.txt` und `zu/` (Rechte `0555`, darin `b.txt`).
- Kopiert wird eine Datei `d` mit „Überschreiben“.
- Danach steht entweder die neue Datei `d` da, oder `d/` steht vollständig mit `a.txt` und `zu/b.txt`.
- Kein halber Ordner `d/` bleibt, und kein `.krk-ersatz-*` liegt im Zielordner.

---
Resolved: `tauschen` in `crates/krk-core/src/operation/fern.rs` tauscht in drei Schritten: das Ziel per `RENAME` beiseite unter `.krk-alt-<pid>-<n>` (`beiseite_legen`, `altname`), den Ersatz auf den Zielnamen, erst dann das alte Ziel entfernen. Scheitert das Beiseitelegen, bleibt das Ziel unverändert und der Ersatz fällt weg; scheitert das Einsetzen, kommt das Ziel zurück (`Tausch::ZielBlieb`, sonst `Tausch::BeideBeiseite` mit beiden Zwischennamen); scheitert erst das Entfernen, steht der Ersatz am Ziel und der Bericht nennt den Zwischennamen (`Tausch::AltesBlieb`, `altes_ziel_melden`, neuer Text `VorgangAltesZielBlieb`; `VorgangErsatzUnterZwischennamen` nennt jetzt `{alt}` und `{name}`). Modulkopf „Ueberschreiben“ und Doc an `Tausch` nachgezogen, `HowTo.md` im sftp-Abschnitt „Kopieren und Verschieben“. Probe `ueberschreiben_entfernt_einen_zielordner_nie_zur_haelfte` in `crates/krk-core/tests/fern_operation.rs`, gegen den Stand vor der Behebung rot.
