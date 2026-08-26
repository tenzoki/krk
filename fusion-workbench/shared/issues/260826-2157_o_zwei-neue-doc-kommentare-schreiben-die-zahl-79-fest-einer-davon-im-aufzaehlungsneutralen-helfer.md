Zwei neue Doc-Kommentare schreiben die Zahl 79 fest, einer davon im aufzählungsneutralen Helfer

---

`9a4e495` legt `varianten_der_aufzaehlung` ausdrücklich ohne den Namen einer bestimmten Aufzählung an, „damit der zweite Plan ihn für `Wirkungsbereich` wiederverwendet". Sein Doc-Kommentar nennt trotzdem dreimal die Zahl 79 (`crates/krk-core/tests/gemeinsam/mod.rs:377-378`), und der Doc-Kommentar der neuen Probe wiederholt sie noch einmal (`crates/krk-core/tests/belegung.rs:1751-1752`). Genau diese Zahl führt `CLAUDE.md` als die, die „in dieser Datei viermal in vier Tagen falsch geworden" ist, und nennt sie deshalb selbst nicht mehr.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-core/tests/gemeinsam/mod.rs:375-381`, `crates/krk-core/tests/belegung.rs:1748-1753`
**Cross-references:** `shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`; `CLAUDE.md`, Abschnitt „Projektstand", Absatz über die gewachsenen Aufzählungen

## Der Befund

`gemeinsam/mod.rs:376-379`:

```
/// **Wozu.** Etliche Listen dieses Baums stehen neben einer Aufzaehlung und
/// sollen sie vollstaendig fuehren: `Kommando::KENNUNGEN` ist die programmweite
/// Kommandoliste, und die Laengenangabe `[(Kommando, &'static str); 79]` zwingt
/// zu 79 Eintraegen und sagt nichts darueber, **welche** 79.
```

`belegung.rs:1750-1752`:

```
/// Der Uebersetzer haelt davon nichts: die Laengenangabe
/// `[(Kommando, &'static str); 79]` zwingt zu 79 Eintraegen und sagt nicht,
/// welche 79
```

Am 260826 nachgezählt: `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs | grep -cE '^\s+[A-Za-z]'` liefert 79, und `belegung.rs:697` trägt `; 79]`. Beide Angaben stimmen **heute**. Sie werden mit dem nächsten Kommando falsch, und zwar ohne dass irgendetwas rot wird: die Zahl im Code zieht der Übersetzer nach, die drei in `gemeinsam/mod.rs` und die zwei in `belegung.rs` zieht niemand nach.

`CLAUDE.md` schreibt zu genau dieser Zahl: „Für `Kommando` … steht hier keine Zahl: sie wächst mit fast jeder Runde und ist in dieser Datei viermal in vier Tagen falsch geworden."

## Warum es beim Helfer schwerer wiegt

`varianten_der_aufzaehlung` ist der eine Helfer für **jede** Aufzählung dieses Baums; der zweite Plan nimmt ihn für `Wirkungsbereich`, und weitere Listen folgen. Sein Doc-Kommentar ist damit die Stelle, an der ein Leser die Bauform lernt, und er erklärt sie an einer Zahl, die zu einer einzigen Aufzählung gehört. Die Begründung trägt ohne sie genauso: „die Längenangabe zwingt zu so vielen Einträgen, wie sie nennt, und sagt nichts darüber, **welche**."

## Vorschlag

Die Zahl aus beiden Doc-Kommentaren nehmen und die Längenangabe als Form zitieren (`[(Kommando, &'static str); N]`). Wer die aktuelle Zahl braucht, liest sie an `belegung.rs:697` oder zählt sie mit dem Kommando aus `CLAUDE.md`.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.
