`gix` zieht ein unlesbares `.git`-Verzeichnis und einen toten `gitdir:`-Verweis selbst zu „kein Repository" zusammen

---
`Gitleser::oeffnen` (`crates/krk-core/src/git/leser.rs`) entscheidet seit dem 260831 über die Varianten von `gix::discover::Error` statt über einen Auffangzweig; damit ist KRKs eigene Zusammenziehung weg (`260831-1444_*_jeder-fehlschlag-von-discover-ausser-dem-deskriptormangel-wird-als-kein-repository-ausgegeben.md`). Zwei der Fälle, die jener Datensatz nennt, erreichen KRK aber gar nicht als eigene Variante: `gix` hat sie vorher zusammengezogen.

Gemessen am 260831 gegen `gix` 0.87.1, je ein frisch angelegtes Prüfrepository unter dem Temporärverzeichnis, Rückgabe von `gix::discover`:

| Lage | Antwort von `gix` | was KRK daraus macht |
|---|---|---|
| `.git/config` mit `chmod 000` | `Ok(repo)` | `Offen` — `gix` übergeht die Datei |
| `.git` als Datei mit `gitdir: /gibt/es/nicht` | `Err(Discover(NoGitRepository))` | `KeinRepository` |
| `.git`-Verzeichnis mit `chmod 000` | `Err(Discover(NoGitRepository))` | `KeinRepository` |
| `.git/config` ist kein INI | `Err(Open(Config(Init(Parse))))` | `Unentschieden` |
| unbekannte `extensions.*` | `Ok(repo)` | `Offen` |

Die zweite und die dritte Zeile sind der Rest des Befunds: ein Rechteproblem und ein toter `gitdir:`-Verweis kommen beim Nutzer weiterhin als „Dieser Ordner liegt in keinem Git-Repository." an (`git/texte.rs`, `KEIN_REPOSITORY`). KRK kann sie mit den Eingaben, die es hat, nicht auseinanderhalten: `NoGitRepository` trägt einen Pfad und keine Ursache.

Wer das entscheiden will, braucht eine zweite Frage an die Platte — etwa: liegt auf dem Weg nach oben ein `.git`, das sich nicht lesen lässt? — und damit eine zweite Suche neben der von `gix`. Ob das die Auskunft wert ist, ist eine Nutzerfrage und nicht in diesem Datensatz zu entscheiden.

**Abnahmetest:** ein Prüfrepository, dessen `.git`-Verzeichnis auf `chmod 000` steht, liefert `Oeffnung::Unentschieden` und nicht `Oeffnung::KeinRepository`. Ein Vermerk im Modulkopf löst den Datensatz **nicht** ab: was der Nutzer zu sehen bekommt, ist ein falscher Satz über seinen Ordner und keine ungenaue Dokumentation. Der Modulkopf von `crates/krk-core/src/git/leser.rs` nennt die Grenze seit dem 260831 trotzdem, damit der nächste Leser sie nicht erneut messen muss.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden beim Beheben von `260831-1444_*_jeder-fehlschlag-von-discover-ausser-dem-deskriptormangel-wird-als-kein-repository-ausgegeben.md`: der Abnahmetest jenes Datensatzes nennt die unlesbare `.git/config`, und die Messung zeigt, dass `gix` sie überhaupt nicht als Fehler meldet.
