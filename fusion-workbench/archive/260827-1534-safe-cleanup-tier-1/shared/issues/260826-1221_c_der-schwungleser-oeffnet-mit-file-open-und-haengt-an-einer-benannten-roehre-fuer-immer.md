Der Schwungleser oeffnet mit `File::open` und haengt an einer benannten Roehre fuer immer

---

`Schwungleser::oeffnen` (`crates/krk-core/src/verzeichnis/sys.rs:229-236`) ist der eine
Oeffner dieser Datei, der **nicht** ueber `sys::ohne_warten_oeffnen` geht. Er ruft
`File::open(pfad)` ohne `O_NONBLOCK`. Zeigt der Pfad in diesem Augenblick auf eine benannte
Roehre ohne Schreiber, kehrt der Aufruf nie zurueck. Der Faden, der ihn tut, ist danach
verloren; ein Abbruchkennzeichen erreicht ihn nicht, weil es erst **nach** dem Oeffnen
gelesen wird.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** High
**Affected:** `crates/krk-core/src/verzeichnis/sys.rs:229-236`; Rufer in
`crates/krk-core/src/verzeichnis/umfang.rs:254`,
`crates/krk-core/src/verzeichnis/durchlauf.rs:512`,
`crates/krk-core/src/verzeichnis/leser.rs:235` und `:281`
**Tree state:** `004ff72`
**Domain:** code

## Was dasteht

```rust
// crates/krk-core/src/verzeichnis/sys.rs:229-236
pub fn oeffnen(pfad: &Path) -> io::Result<Self> {
    let verzeichnis = File::open(pfad)?;
    if !verzeichnis.metadata()?.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("{} ist kein Verzeichnis", pfad.display()),
        ));
    }
```

Die **Typpruefung** steht richtig am Deskriptor, so wie CLAUDE.md es zusagt. Das **Oeffnen**
selbst ist ungeschuetzt. Sechshundertsechzig Zeilen tiefer in derselben Datei steht die
Huelle, die genau dafuer gebaut wurde:

```rust
// crates/krk-core/src/verzeichnis/sys.rs:889-896
pub fn ohne_warten_oeffnen(pfad: &Path) -> io::Result<File> {
    let datei = OpenOptions::new()
        .read(true)
        .custom_flags(O_NONBLOCK)
        .open(pfad)?;
```

Ihr Doc-Kommentar (`sys.rs:801-808`) beschreibt den Schaden wortgleich: „`File::open` auf eine
benannte Roehre ohne Schreiber haengt, bis jemand hineinschreibt."

## Warum das der Verzeichnisleser trotzdem nicht hat

Weil er den Fall an vier Stellen fuer ausgeschlossen haelt, und an drei davon ist es eine
Pruefung **am Pfad vor** dem Oeffnen — genau die Bauform, die der Defekt
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`
abgeschafft hat:

| Rufer | Was vor dem Oeffnen geprueft wird | Faden |
|---|---|---|
| `umfang::zaehlen:239` | `symlink_metadata(...).is_dir()` am Pfad, dann `oeffnen` am selben Pfad | **Hauptfaden** (`umfang.rs:77-84`) |
| `umfang::zaehlen:274` | `Typ::Ordner` aus dem vorigen Schwung, dann `oeffnen` | **Hauptfaden** |
| `durchlauf::unterbaum_entscheiden:512` | `Typ::Ordner` bzw. der Auftragstyp | Arbeitsfaden |
| `leser::lesen_und_senden:281` | nichts; der Pfad kommt aus der Navigation | Arbeitsfaden |
| `leser::lesen_hoechstens:235` | **nichts** | Arbeitsfaden der Vorschau |

Der letzte ist der ohne jedes Zeitfenster-Argument: `leseprofil::bausteine`
(`crates/krk-core/src/leseprofil/bausteine.rs:422` und `:467`) reicht einen aus
`readers.toml` zusammengesetzten Pfad ohne jede Typfrage an `lesen_hoechstens` weiter. Steht im
angezeigten Ordner eine Roehre unter dem Namen, den eine Profilzeile nennt, bleibt der
Vorschaufaden dort stehen.

Die zwei Rufer in `umfang` sind die schwersten, und zwar wegen des Fadens: die Zaehlung laeuft
laut ihrem eigenen Modulkopf ausdruecklich auf dem Hauptfaden. Zwischen dem `lstat(2)` und dem
`File::open` liegt ein Zeitfenster; wird der Eintrag darin ersetzt, steht die ganze Oberflaeche.

## Warum das nicht der zurueckgestellte Defekt zum Netzpfad ist

`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-0000_d_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`
beschreibt denselben **Schaden** aus einer anderen **Ursache**, und die drei Unterschiede sind
genau die, an denen jene Zurueckstellung haengt:

1. **Messbar ohne Server.** Jener Datensatz ist zurueckgestellt, weil „ohne Server jede Wahl
   unbelegt bleibt". Eine Roehre legt `mkfifo(1)` auf der lokalen Platte an; der `Pruefordner`
   des Kerns kann sie herstellen.
2. **Die Abhilfe liegt schon im Baum.** Jener Datensatz laesst als einzigen Weg eine
   Zeitschranke uebrig. Hier ist es ein Austausch des Oeffners gegen `ohne_warten_oeffnen`,
   also derselbe Griff, den `260809-1652` fuer den Editor, `260810-1247` fuer die Vorschau und
   `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0942_c_das-packen-haengt-an-einer-benannten-roehre-mit-schreiber-und-die-probe-kann-es-nicht-sehen.md`
   fuer das Packen schon getan haben.
3. **`umfang` gab es damals nicht.** Jener Datensatz haelt fest, C9 halte, weil kein Aufrufer
   je auf einen Lesefaden wartet. Das gilt fuer `umfang::zaehlen` nicht: es laeuft auf dem
   Hauptfaden selbst.

## Richtung

`Schwungleser::oeffnen` nimmt `sys::ohne_warten_oeffnen(pfad)` statt `File::open(pfad)`. Die
`metadata()?.is_dir()`-Pruefung dahinter bleibt unveraendert; sie steht schon am Deskriptor und
ist damit die Antwort, die `ohne_warten_oeffnen` von jedem Aufrufer verlangt. Zu klaeren waere
allein, ob `O_NONBLOCK` an einem Verzeichnisdeskriptor auf `getattrlistbulk(2)` durchschlaegt —
`blockierend_stellen` nimmt es unmittelbar nach dem Oeffnen wieder ab, also `inference:` nein.

Eine Probe dazu: eine Roehre ohne Schreiber im Pruefordner, `Schwungleser::oeffnen` darauf,
Erwartung ein `Err` und kein Haenger. Sie braucht eine Zeitschranke im Pruefrahmen, sonst haengt
der Testlauf statt rot zu werden — dasselbe Problem, das `260825-0942` fuer das Packen benannt
hat.

---
Resolved: 260826-1930 — Schwungleser::oeffnen nimmt die Huelle ohne_warten_oeffnen, Typpruefung bleibt am Deskriptor; Probe mit benannter Roehre unter 5-s-Schranke, rot vor der Behebung (Plan 260826-1811 Schritt 2).

Reconciled: 260826-2205 — gegen den Baum `bc5991d` geprueft und zutreffend: der Commit ist `9c02863`, `crates/krk-core/src/verzeichnis/sys.rs:240` ruft `ohne_warten_oeffnen`, die Typpruefung an `:241-246` bleibt am Deskriptor, die Probe steht an `crates/krk-core/tests/verzeichnis.rs:3525`; `make check` ueber `bc5991d` gruen. Der Hash steht hier als Abgleichsbeleg, nicht als Berichtigung der `Resolved:`-Zeile (`shared/issues/260826-1933_*_die-zwei-resolved-zeilen-der-schritte-1-und-2-tragen-den-sitzungsstempel-statt-des-commits.md`).
