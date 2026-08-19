CLAUDE.md sagt, den Tag setze der Nutzer — seit dem 260813-1534 setzt ihn das Werkzeug

---

CLAUDE.md schreibt im Abschnitt „Bauen und prüfen":

> **Seit der Runde 8 bricht `cargo xtask release` ab, wenn HEAD keinen Tag
> `v<version>` trägt, der zur `Cargo.toml` passt** — den Tag setzt der Nutzer,
> nicht das Werkzeug.

Der zweite Halbsatz ist überholt. `README.md`, Abschnitt `### Versionsstufen`,
sagt das Gegenteil und nennt den Grund:

> **Jede Auslieferung bekommt einen Tag `v<version>`, und den setzt das
> Werkzeug.** Bis zum 260813-1534 galt das Gegenteil […] Der Nutzer hat diese
> Festlegung am selben Tag zurückgenommen, weil sie einen Auslieferungsweg in
> einem Kommando unmöglich macht.

Bindend ist `shared/decisions/260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`;
er überholt `circles/260813-0939-.../decisions/260813-0939_*_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md`.

---

**Schwere:** mittel. Der erste Halbsatz stimmt weiter, die Prüfung besteht. Wer
aber CLAUDE.md folgt, setzt den Tag von Hand und läuft danach in „ein vergebener
Name hält den Lauf an" — die Auslieferung bricht ab, und die Ursache steht in
der Datei, der er gefolgt ist.
**Gefunden von:** Orchestrator, beim Vorbereiten einer Auslieferung am 260816
**Betroffen:** `CLAUDE.md`, Abschnitt „Bauen und prüfen", der Absatz über
`./release.sh`
**Domain:** knowledge

## Was zu tun ist

Den Halbsatz streichen oder umdrehen. Die Zahl wählt weiterhin der Nutzer, und
zwar im Argument von `./release.sh <version>`; der Tag folgt daraus mechanisch.
Genau so steht es im README, und der Satz dort ist die richtige Fassung.

---
Also seen: 260819-1230 by orchestrator — in der Praxis eingetreten, und zwar genau in der hier vorhergesagten Form. Der Nutzer fragte vor einer Auslieferung, ob 0.5.3 oder 0.5.4 zu nehmen sei; der Tag v0.5.3 stand bereits auf dem Versions-Commit 0b57157, HEAD war einen Commit weiter und ungetaggt, und ein `./release.sh 0.5.3` waere am vergebenen Tagnamen abgebrochen. Zwei Belegstellen, die dieser Datensatz noch nicht nennt: die Hilfe des Kommandos in `xtask/src/main.rs:48-58` schreibt das Taggen ausdruecklich aus, und `xtask/src/version.rs:78-88` fuehrt drei Vorhaben, von denen zwei taggen (`NurTaggen`, `SetzenEintragenTaggen`).

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und am Werkzeug nachgelesen.** `CLAUDE.md:106` sagt wortgleich „den Tag setzt der Nutzer, nicht das Werkzeug". Der Baum sagt das Gegenteil: `xtask/src/version.rs:97` baut die Zeichenkette `v{zahl}`, `:216` trägt `fn taggen`, und `vorhaben_bestimmen` (`:329`) führt drei Vorhaben, von denen zwei taggen (`NurTaggen`, `SetzenEintragenTaggen`); die Tafel dazu steht als Kommentar bei `:307-310`. Der Nutzer liefert allein die Zahl, über `./release.sh <version>`.

**Zur Doppelung.** Der Commit `ee6d033` trägt eine Meldung über die Berichtigung dieses Befunds, sein Diff an `CLAUDE.md` ist aber leer; `77dcd48` hat den zweiten Datensatz derselben Sache wieder zurückgenommen (`shared/issues/260819-1230_c_claude-md-sagt-den-tag-setze-der-nutzer-das-werkzeug-setzt-ihn-seit-dem-260813.md`). Dieser Datensatz hier ist der lebende. Der Marker bleibt `_o_` für den Durchgang des Kurators.
