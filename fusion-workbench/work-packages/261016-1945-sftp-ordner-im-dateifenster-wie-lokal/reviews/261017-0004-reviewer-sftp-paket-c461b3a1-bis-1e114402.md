# Durchsicht des sftp-Pakets, `c461b3a1..1e114402`

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `c461b3a1..1e114402`
**Not-opened:** `crates/krk-core/src/git/texte.rs`, `crates/krk-core/src/heimordner/verbindung.rs`, `crates/krk-core/tests/heimordner.rs`, `crates/krk-core/tests/operation.rs`, `crates/krk-ui/src/appkit/bereichsleiste.rs`, `crates/krk-ui/src/appkit/blaetter/verbindung.rs`, `crates/krk-ui/src/fernprobe.rs`, `crates/krk-ui/src/gitmodell.rs`, `README.md`
**Domain:** code
**Work-item:** 261016-1945-sftp-ordner-im-dateifenster-wie-lokal
**Maßstab:** Plan `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md` (E1–E17) und die vier Entscheide des Pakets
**Urteil:** revise

## Zusammenfassung

Die tragenden Zusagen halten am Code:
- Das Passwort steht in keinem Argument, keiner Umgebungsvariable, keiner Datei und keinem `Debug`.
- Der Socket liegt in einem `0700`-Ordner und gibt höchstens einmal heraus.
- `ssh` läuft mit `StrictHostKeyChecking=yes` und dem Rechner hinter `--`.
- Kein Netzaufruf erreicht den Hauptfaden.
- Lokale Wege sind wörtlich unverändert.
- Endgültiges Löschen gibt es nur nach dem Blatt mit „Abbrechen“ als Vorgabe.

Offen bleiben fünf Befunde mittlerer Schwere: ein Weg, `known_hosts` mit einem Muster zu vergiften, zwei Wege, die nach „Überschreiben“ Daten verlieren, eine unbelegte Behauptung im Löschblatt und ein unbegrenzter Speicherverbrauch an der Fehlerausgabe von `ssh`. Keiner blockiert die Auslieferung so, wie es ein Fehler ohne Rückfrage täte. Jeder sollte aber vor dem Nutzertest gegen `ssh.strato.de` behoben sein.

Geprüft habe ich am Baum `1e114402`. `cargo test -p krk-core --test fern --test fern_operation` lief grün (26 und 11 Proben). Weder `make check` noch das Bündel habe ich gefahren.

## Summen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 5 |
| Low | 6 |

## Befunde nach Thema

### Sicherheit an der Grenze zu `ssh`

**M1. Ein Rechnername mit `,` oder `*` vergiftet `known_hosts`.**
- `fern/adresse.rs` `fernziel` lässt `,`, `*`, `?` und `!` im Rechner zu.
- `rechnerschluessel.rs` `gescannt_aus` schreibt den Suchnamen aus `ssh -G` ungeprüft vor die gescannte Zeile.
- Geprüft: `ssh -G -- 'evil.example,*'` gibt `hostname evil.example,*` aus.
- Ein angenommener Schlüssel gälte damit für jeden Rechner, in jedem `ssh` des Nutzers.
- Issue `261017-0004-ein-rechnername-mit-komma-oder-stern-schreibt-ein-muster-nach-known-hosts-das-fremde-rechner-deckt.md`.

**Geprüft und in Ordnung:**
- `anmeldehilfe.rs`:
  - Der Ordner wird mit `0700` angelegt, ein vergebener Name führt zu einem neuen.
  - Höchstens eine Verbindung wird angenommen, danach fällt der Horcher.
  - `Drop` weckt den Faden über `beendet` und räumt Socket und Ordner weg.
  - Die Klartextabschrift ist `Zeroizing`.
  - Die Grenze „ein Prozess desselben Kontos“ ist im Modulkopf ehrlich benannt.
- Die Hilfe lebt in `Zugangsregister::zugang` bis nach `INIT` und `REALPATH(".")` und fällt dann (`drop(halten)`).
- `Passwort` hat ein eigenes `Debug` (`Passwort(****)`).
- `Kanalplan` und `Anmeldung` tragen allein Pfade.
- `kanal.rs` `ssh_plan` hat absolute Programmpfade, `--` vor dem Rechner und `-l` mit eigenem Argument.
- Zusätzlich zum Plan stehen `RemoteCommand=none` und `RequestTTY=no` da.
- Mit Passwort ist `PubkeyAuthentication=no` gesetzt, ohne Passwort `BatchMode=yes`.
- `main` fragt `anmeldehilfe::rolle()` als erste Handlung. Die Probe dazu liest die erste Codezeile.
- `anhaengen` hängt nur an, mit `0700`/`0600`, ohne die Datei neu zu schreiben, und prüft vorher das Zeilenende.
- Typ und Schlüssel der gescannten Zeilen sind auf den Zeichensatz von `known_hosts` beschränkt.

**Am Rand, ohne Issue:**
- Die Statuszeile zeigt bei einer Trennung die letzte Zeile der Fehlerausgabe von `ssh`, die auch vom Server stammen kann. Das ist Text in der eigenen Statuszeile und keine Ausführung.
- Fragt `ssh` über eine Konfiguration mit `ControlMaster ask` nach, bekäme diese Bestätigung das Passwort als Antwort. Es bleibt dabei bei `ssh` und geht nicht ins Netz.

### Korrektheit der Operationen

**M2. Überschreiben über eine Verknüpfung auf dem Server zerstört die Quelle.**
- `operation/fern.rs` `Stelle::derselbe_eintrag` vergleicht ferne Stellen am Pfadtext, lokale an Gerät und Inode.
- Benennen zwei Pfade dieselbe Datei, entfernt „Überschreiben“ (`konflikt_klaeren` → `baum_entfernen`) die Quelle.
- Der Kopierweg meldet danach Erfolg mit leerer Datei.
- Code gelesen, nicht ausgeführt.
- Issue `261017-0004-ueberschreiben-zerstoert-die-quelle-wenn-ziel-und-quelle-auf-dem-server-ueber-eine-verknuepfung-derselbe-eintrag-sind.md`.

**M3. Jedes Scheitern des Anlegens gilt als Konflikt.**
- `Fehlschlag::ist_kein_konflikt` gibt für `Lokal(_)` und `Fern(_)` immer `false` zurück.
- `namensgrund` in derselben Datei schließt `NO_SUCH_FILE` und `PERMISSION_DENIED` dagegen aus.
- Nach „Überschreiben“ eines Ziels, das wegen fehlender Rechte gar nicht ersetzbar ist, ist das Ziel weg.
- Issue `261017-0004-jedes-scheitern-beim-fernen-anlegen-gilt-als-konflikt-und-ueberschreiben-entfernt-dann-ein-ziel-ohne-es-zu-ersetzen.md`.

**L1. `RENAME` hat keinen Ausweichweg über eine Mount-Grenze des Servers.**
- Lokal weicht KRK auf `ueber_datentraeger` aus, fern endet der Weg in `ueberspringen`.
- Issue `261017-0004-verschieben-auf-demselben-server-ueber-eine-mount-grenze-des-servers-scheitert-ohne-ausweichweg.md`.

**Geprüft und in Ordnung:**
- Kein Name vom Server bricht aus dem lokalen Zielordner aus. Ferne Kinder entstehen nur über `Fernpfad::verbinden`, lokale Zielnamen nur aus `Fernpfad::name()` danach oder über `name_pruefen`.
- „Ordner auf Ordner“ fragt lokal `symlink_metadata` und fern `LSTAT`. Eine Verknüpfung am Ziel wird nie zusammengeführt.
- Dateien entstehen exklusiv: lokal über `create_new`, fern über `CREAT|EXCL`.
- Ein Abbruch räumt das halbe Ziel in allen drei Richtungen ab, nachdem die offenen `WRITE` abgewartet sind. Die Probe fährt alle drei.
- Verschieben über eine Grenze entfernt die Quelle nur, wenn nichts übersprungen wurde.
- `RENAME` statt `posix-rename` (E10) ist eingehalten.
- Das endgültige Löschen folgt lokal (`baum_entfernen`) und fern (`fern_baum_entfernen`) keiner Verknüpfung und hält je Eintrag am ersten Fehler.
- Ein kurzes `DATA` wird nachgelesen, ein überlanges ist ein Protokollfehler, die Versätze beim Schreiben laufen monoton.
- Ein Ordner in sich selbst ist lexikalisch abgewiesen.

**Am Rand, ohne Issue:**
- Rechte gehen mit `& 0o7777` hinüber, eine setuid-Datei vom Server bleibt setuid.
- `inference:` Das lokale `fs::copy` behält den Modus ebenso, also gleiche Lage wie lokal.

### Protokoll und Fäden

**M4. Die Fehlerausgabe von `ssh` wird ohne Längengrenze je Zeile gelesen.**
- `fern/gespraech.rs` `fehlerausgabe_lesen` liest mit `split(b'\n')` ohne Grenze.
- Issue `261017-0004-die-fehlerausgabe-von-ssh-wird-ohne-laengengrenze-je-zeile-gelesen.md`.

**L2. Ferne Ordnerlisten fragen in Operationen keinen Abbruch.**
- Betroffen sind `fern_namen` und `fern_baum_entfernen`.
- Issue `261017-0004-ferne-ordnerlisten-in-kopieren-verschieben-und-loeschen-fragen-keinen-abbruch-und-haben-keine-obergrenze.md`.

**L3. `Getrennt` kommt erst nach dem Join des Fehlerfadens.**
- Hält ein Enkelprozess die Fehlerausgabe offen, hängt das Gespräch.
- Es gibt keine Frist für einen Server, der nicht mehr liest.
- `compile_fail` ohne `E0277`.
- Issue `261017-0004-das-gespraech-meldet-getrennt-erst-nach-dem-join-des-fehlerfadens-und-kann-an-einem-enkelprozess-haengen.md`.

**L4. Der ferne Leser lässt `/`, leere Namen und NUL in die Liste.**
- Ein Ausbrechen ist dadurch nicht möglich.
- Issue `261017-0004-der-ferne-leser-laesst-namen-mit-schraegstrich-leere-namen-und-nul-in-die-liste.md`.

**Geprüft und in Ordnung:**
- `paket_lesen` prüft die Länge gegen die Grenze von 256 KiB und weist 0 ab, bevor Speicher angelegt wird.
- Der Leser zerlegt ohne Index und ohne `unwrap` auf fremden Daten.
- Kennungen laufen mit `wrapping_add` und überspringen belegte. Eine unbekannte Kennung trennt.
- Zustand und Kennungstabelle stehen unter einer Sperre, also bleibt bei einer Trennung keine Anfrage unbeantwortet. Die Probe deckt wartende und spätere Anfragen ab.
- Der Kindprozess wird auf jedem Weg geerntet. `Drop` wartet nicht.
- `Fernfaden` ist weder `Send` noch `Sync` und entsteht allein in `fern::faden`.
- Auf dem Hauptfaden steht allein `anmeldeverzeichnis()`, dessen Sperre nie über einen Netzaufruf gehalten wird.
- Einen stillen Wiederaufbau gibt es nicht: `Zugangsregister::zugang` hat einen Rufer, auf dem Faden `krk-verbinden`.

### Lokale Unversehrtheit und Löschweg

**M5. „Unentschieden“ führt zur endgültigen Rückfrage, und der Text behauptet „kein Papierkorb“.**
- `kommandos/loeschwarnung.rs` `vor_der_rueckfrage`, Zweig `(false, false)`.
- Der Entscheid `261016-1948-…` deckt allein „ohne Papierkorb“ ab.
- Issue `261017-0004-ein-unentschiedener-papierkorbbefund-bietet-endgueltiges-loeschen-an-und-das-blatt-behauptet-einen-ort-ohne-papierkorb.md`.

**Geprüft und in Ordnung:**
- `Tabliste::lesen_starten` ruft lokal wörtlich `Lesevorgang::starten`.
- Der Papierkorbweg ist unverändert.
- `Art::EndgueltigLoeschen` und `Fernauftrag::loeschen` entstehen allein in `loeschauftrag_stellen`, nach dem Rückruf des Blatts.
- Das Blatt in der Form „endgültig“ trägt `hasDestructiveAction` und hat „Abbrechen“ als Vorgabe.
- `session.toml` schreibt für einen fernen Tab `rueckkehr`.

### Fallunterscheidungen, Sprachtabellen, Proben, Doku

**L5. „Zur Zwischenablage springen“ liest im fernen Tab einen lokalen Pfad als Serverpfad.**
- Grund: `Ortsbedarf::Ueberall`, und E13 nennt das Kommando nicht.
- Issue `261017-0004-zur-zwischenablage-springen-liest-in-einem-fernen-tab-einen-lokalen-pfad-als-serverpfad.md`.

**L6. HowTo und Kommentare weichen vom Code ab.**
- Ein Ziehen zwischen den Fenstern, das es nicht gibt.
- „Rundweg“ fehlt in der Liste der ausgegrauten Befehle.
- Teilen meldet sich entgegen E13 nicht.
- Kommentare mit „bis dahin melden“ sind veraltet.
- Ein Doc-Kommentar klebt an der falschen Probe.
- Issue `261017-0004-howto-und-kommentare-des-sftp-pakets-weichen-vom-code-ab.md`.

**Geprüft und in Ordnung:**
- `Ortsbedarf`, `Fernart` und `Kontextbefehl` werden ohne Auffangzweig verzweigt.
- Die neuen Schlüssel stehen in `de`, `fr` und `en`.
- `blaetter/rechnerschluessel.rs` trägt den Untergrenzen-Abschnitt.
- Die Quelltextproben (Preisgabe, Erstprüfung in `main`, `known_hosts`, Fernfaden, ferne Wege ohne Systemaufruf) sehen, was ihr Doc-Kommentar behauptet, und nennen ihre Grenze.
- Die Probe `eine_verengung_ist_nie_ohne_ihren_weiteren_bereich_zulaessig` ist abgeschwächt (`|| zulaessig(andere, ohne_ort)`). Ihr Doc-Kommentar sagt das. Sie ruht auf der Annahme, dass ein ferner Tab nie eine Bildfolge zeigt, und diese Annahme hält keine Probe.

## Querschnitt

- **Gleichheit fern ist Textgleichheit.**
  - M2 und die Rekursion über eine Verknüpfung haben dieselbe Ursache: `derselbe_eintrag` und `liegt_in` sind lexikalisch.
  - Der Plan setzt Textgleichheit in E1 und E2 für Orte und Tabs bewusst. Für zerstörende Entscheidungen reicht sie nicht.
  - Eine Prüfung über `REALPATH` vor „Überschreiben“ und vor „liegt in“ schließt beides.
- **Zwei Antworten auf die Frage „ist der Name vergeben?“.**
  - `ist_kein_konflikt` und `namensgrund` antworten verschieden (M3).
  - Dazu kommt die Namensregel zweimal: `Fernpfad::verbinden` und `fern_eintrag` (L4).
  - Je eine Regel an einer Stelle beseitigt beide Abweichungen.
- **Was vom Server oder aus `ssh -G` kommt, wird an zwei Stellen nicht begrenzt.** Gemeint sind der Suchname (M1) und die Fehlerausgabe (M4). Die Paketgrenze zeigt, wie es aussehen soll.

## Reihenfolge

1. Vor dem Nutzertest gegen `ssh.strato.de`: M1, M2, M3 und M5. M5 verlangt eine Antwort des Nutzers (Möglichkeit 1 oder 2).
2. Vor einer Auslieferung: M4.
3. Aufräumen: L1 bis L6.

Unverändert offen nach dem Plan sind der Nutzertest gegen `ssh.strato.de` und der Abnahmelauf L1 bis L10 aus C8 (E16). Kein Agent kann beides fahren.
