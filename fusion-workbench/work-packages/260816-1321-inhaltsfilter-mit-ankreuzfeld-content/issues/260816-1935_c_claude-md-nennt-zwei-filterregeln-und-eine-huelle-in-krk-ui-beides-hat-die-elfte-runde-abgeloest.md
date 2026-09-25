CLAUDE.md nennt zwei Filterregeln, zwei Rufer je Regel und eine Lesehülle in `krk-ui` — alle drei Aussagen hat die elfte Runde abgelöst

---

Vier Stellen in `CLAUDE.md` beschreiben den Stand vor dieser Runde. Zwei davon sind
Aussagen, die die Datei ausdrücklich als „was man nicht sieht, wenn man es nicht weiß"
führt — also genau die Sätze, auf die ein neuer Leser sich verlässt.

**1. `CLAUDE.md:127` — zwei Regeln, es sind drei; zwei Rufer je Regel, `traegt_die_folge`
hat drei.** Der Satz lautet: „Das Modul heißt jetzt `krk-core/src/verzeichnis/filter.rs`
und trägt **zwei** Regeln, die je genau einmal dastehen […]. **Jede hat genau zwei
Rufer**, und eine Zählprobe hält das fest."

Am Baum: `filter.rs` trägt seit C1 drei Regeln — `traegt_ein_dateiname`, `traegt_die_folge`
und `inhaltsschwelle` (`verzeichnis/filter.rs:90`, `:122`, `:157`), und der Modulkopf der
Datei sagt das auch so (`filter.rs:1-2`, „Die drei Regeln des Filters"). Die Rufer sind
verschieden viele: `traegt_ein_dateiname` hat weiter zwei, `traegt_die_folge` seit A2 drei
(`verzeichnis/modell.rs`, `verzeichnis/durchlauf.rs`, `verzeichnis/inhalt.rs`),
`inhaltsschwelle` einen (`Ordnermodell::inhalt_wirkt`). Die Zählprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
(`crates/krk-core/tests/verzeichnis.rs`) ist mit C6.3 auf drei nachgezogen worden; die
Zeile in `CLAUDE.md` ist es nicht.

**2. `CLAUDE.md:135` — die Lesehülle wohnt nicht mehr in `krk-ui`.** Der Satz lautet:
„**Die Hülle hat zwei Aufrufer**, den Editor (`krk-core/src/text/datei.rs`, `oeffnen`) und
seit der Runde 2 auch die Vorschau (`krk-ui/src/vorschaumodell.rs`, `bis_zur_grenze_lesen`)."

Am Baum: Schritt A1 (`5c7f5b9`) hat `bis_zur_grenze_lesen` nach
`crates/krk-core/src/text/datei.rs:606` verschoben; in `vorschaumodell.rs` steht sie nicht
mehr, und ein Suchender findet sie dort nicht. Die zwei Aufrufer von `ohne_warten_oeffnen`
liegen seither beide in `text/datei.rs` (`:421` und `:606`), und die Rufer der **Hülle**
sind zwei, die Vorschau (`krk-ui/src/vorschaumodell.rs:626`, `:640`) und der Inhaltsfilter
(`krk-core/src/verzeichnis/inhalt.rs:134`). `sys.rs` selbst ist nachgezogen worden
(`sys.rs:794-802`), `CLAUDE.md` nicht.

**3. `CLAUDE.md:131` — der Deskriptorhaushalt hat eine zweite Hälfte bekommen.** Der Satz
„Der Durchlauf über den Unterbaum hält genau **einen** Verzeichnisdeskriptor, gleich wie
tief der Baum ist" stimmt weiter und ist nicht mehr die ganze Zusage: bei gesetztem
„Content" hält er während eines Lesens zusätzlich **einen Dateideskriptor**, und dass der
vor dem nächsten Kandidaten frei ist, ist die eigentliche neue Aussage (C3.5,
`verzeichnis/durchlauf.rs:108-114` schreibt sie aus). Wer nur `CLAUDE.md` liest, hält die
Dateiseite für ungeregelt.

**4. `CLAUDE.md:11` und die Tabelle darunter führen zehn Runden.** Die elfte ist aktiv
(`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_t_circle.md`), und der Absatz
darüber sagt selbst, dass der Dateibestand verbindlich ist und nicht die Zeile. Die
Eintragung gehört üblicherweise in den Abschluss der Runde; sie steht hier, damit sie beim
Abschluss nicht übersehen wird, und nicht als Vorwurf an den laufenden Stand.

---

Die Ablage ist der Circle und nicht der gemeinsame Speicher: alle vier Stellen sind durch
diese Runde falsch geworden, keine war es vorher.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.
Verwandt: `issues/260816-1934_o_sechs-prosastellen-im-baum-beschreiben-den-stand-vor-der-elften-runde.md`
(dieselbe Sorte Befund, andere Fläche).

---
Resolved: Alle **vier** Stellen dieses Datensatzes sind berichtigt, drei durch den Kuratorenlauf
`260819-1500` (Einträge L10 und L11, Commit `5886d04`) und die vierte durch den Lauf `260820-1119`
(Eintrag L03, Commit `7da3098`). Am Baumstand `f5300f4` einzeln nachgelesen:

1. **Zwei Regeln, zwei Rufer je Regel** — `CLAUDE.md:131` sagt jetzt „trägt seit der Runde 11
   **drei** Regeln, die je genau einmal dastehen" und nennt `traegt_ein_dateiname`,
   `traegt_die_folge` und `inhaltsschwelle`. Die Rufer sind nicht mehr beziffert: „Wie viele Rufer
   jede hat, sagt die Zählprobe […] und nicht diese Zeile."
2. **Lesehülle in `krk-ui`** — `grep -c 'krk-ui/src/vorschau' CLAUDE.md` liefert 0. Die Zeile sagt
   jetzt „Die Hülle hat zwei Aufrufer, und beide liegen seit der Runde 11 in
   `krk-core/src/text/datei.rs`: `oeffnen` für den Editor (`:421`) und `bis_zur_grenze_lesen`
   (`:605`) für die Vorschau und den Inhaltsfilter."
3. **Deskriptorhaushalt** — der Absatz trägt jetzt die zweite Hälfte: „**Seit der Runde 11 legt der
   Inhaltsfilter genau einen Dateideskriptor dazu, und nur während eines Lesens**".
4. **Die Rundentabelle** — sie führt vierzehn Zeilen, die elfte darunter, und der Absatz nennt gar
   keine Zahl mehr, sondern das Kommando `ls fusion-workbench/circles/*/_*_circle.md`.

**Ein Nachbefund, der aus dieser Prüfung stammt und nicht mehr hierher gehört:** die Zählprobe, die
`CLAUDE.md:131` unter Punkt 1 als Beleg anbietet, heißt dort
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_…`; im Baum heißt sie
`die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`
(`crates/krk-core/tests/verzeichnis.rs:3095`), und der zitierte Name hat null Treffer. Gefilt als
`shared/issues/260820-2056_o_claude-md-nennt-eine-zaehlprobe-unter-einem-namen-den-der-baum-nicht-traegt.md`.
