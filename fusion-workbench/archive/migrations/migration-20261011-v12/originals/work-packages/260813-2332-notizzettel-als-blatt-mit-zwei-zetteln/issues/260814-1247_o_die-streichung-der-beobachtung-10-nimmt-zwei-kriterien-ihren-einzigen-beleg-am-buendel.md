Die Streichung der Beobachtung 10 nimmt zwei Kriterien ihren einzigen Beleg am Bündel

---

Beobachtung 10 der Abnahmeliste — Schreibrecht am Ablageordner nehmen, `f2`, „abc" tippen,
`Esc` — ist am 260814-1115 gestrichen worden, mit dieser Begründung:

> Die Logik ist am Modell in drei Proben abgenommen, und der Rückgabewert trägt seit Turn 2
> `#[must_use]`; die Beobachtung hätte der Verdrahtung nichts hinzugefügt, was der Übersetzer
> nicht schon hält.

**Die Begründung trägt für die eine Hälfte des Kriteriums und nicht für die andere.**
Beobachtung 10 gehört zu Kriterium C4, fünftes der zweiten Liste (`:268`), und das sagt zwei
Dinge zu: die Meldung nennt den Grund, **und** „abc" steht danach unverändert im Zettel.

**Was hält, und wo es steht.** Die drei Proben am Modell sind da und sagen, was die Begründung
behauptet: `das_oeffnen_setzt_den_abweichenden_stand_nicht_zurueck`
(`crates/krk-ui/src/zettelmodell.rs:417`), `ein_sauberer_zettel_bekommt_den_neuen_dateiinhalt`
(`:438`), `jeder_abweichende_zettel_steht_zur_sicherung_an` (`:456`). `#[must_use]` steht an
`Zettelmodell::oeffnen` (`:172`) und ist mit `79dab20` gekommen, also in Turn 2. Beide
Aufrufstellen nehmen den Rückgabewert und nicht das Gelesene
(`crates/krk-ui/src/appkit/anwendung.rs:3305-3312` und `:3418-3423`); von Hand gelesen, nicht
erschlossen.

**Was die Begründung zu weit trägt.** `#[must_use]` verbietet allein das stille Fallenlassen.
Es zwingt keinen Aufrufer, den zurückgegebenen Stand in die Textfläche zu setzen statt des
Gelesenen — genau der Verlust, um den es geht. `let _ =` davor ist in diesem Baum eine erlaubte
und ausdrücklich beschriebene Schreibweise, und der Doc-Kommentar an `oeffnen` sagt es selbst.
Was die Verdrahtung hält, ist nicht der Übersetzer, sondern die zwei richtig geschriebenen
Stellen. Dazu die zweite Einschränkung aus `CLAUDE.md`: `unused_must_use` ist erst unter
`-D warnings` ein Fehler. `make check` fährt clippy so, `cargo build` und `cargo test` nicht.

**Was ohne Beleg bleibt.** Die Meldung. Sie entsteht in `zettel_sichern`
(`crates/krk-ui/src/appkit/anwendung.rs:3514`) und geht über `zettel_sicherung_melden` (`:3553`)
in die Statuszeile. Keine Probe erreicht diesen Weg, und keine kann es: `krk-ui` hat kein
Bibliotheksziel. Beobachtung 10 war die einzige Stelle im Abnahmelauf, an der eine gescheiterte
Sicherung überhaupt hergestellt worden wäre. Mit ihrer Streichung stehen **zwei** Kriterien
ohne Beleg da: `:268` zur Hälfte und `:267` („Eine gescheiterte Sicherung an einem der ersten
drei Momente meldet ihren Grund an einer Stelle, an der der Nutzer sie sieht") ganz — letzteres
war ohnehin in keiner Beobachtung.

---

**Schwere:** mittel. Der Weg der Meldung ist am Baum gelesen und sieht richtig aus; gesehen hat
ihn niemand.

**Was zu tun ist.** Beobachtung 10 wieder aufnehmen und beim nächsten Abnahmelauf fahren. Sie
kostet zwei Handgriffe (`chmod` hin und zurück) und deckt dabei drei Kriterien statt eines:
`:267`, `:268` und, mit einem Tabklick statt `Esc` als Fortsetzung, auch `:269`.

**Kontext**

- Gefunden beim zweiten Abgleich der Runde 9, `history/260814-1247-reconciliation.md`.
- Der Herkunftsnachweis für `#[must_use]`: `git log -S` über `crates/krk-ui/src/zettelmodell.rs`, ein Treffer, `79dab20`.
