Das Erhebungsmuster aus C9.4 ist zu eng, und das Gegenbeispiel steht in `belegungsausgabe.rs`
---
C9.4 des Specs der Runde 23 erhebt die nachzuziehenden Zählaussagen mit einem `grep`-Muster aus
vierzehn Wortformen und kommt auf 92 Treffer in 21 Dateien. Die Zahl ist am 260830-1310 gegen den
Stand `2059138` nachgeprüft und stimmt. Das Muster ist trotzdem zu eng, und der Beleg ist eine
falsche Zählaussage derselben Sorte, die es nicht findet:

`crates/krk-ui/src/belegungsausgabe.rs:234-235` sagt im Doc-Kommentar zu `NICHT_EINGEORDNET`:

> Die uebrigen Zellen dieser Spalte nennen Orte — "Editor", "Textfelder und Editor", die sieben
> Beschriftungen von [`Wirkungsbereich`](krk_core::tasten::Wirkungsbereich).

`awk '/^pub enum Wirkungsbereich/,/^}/' crates/krk-core/src/tasten/belegung.rs` zählt **acht**
Werte, und `Wirkungsbereich::beschriftung` (`:361`) liefert für jeden einen eigenen Text; die Probe
`keine_zwei_wirkungsbereiche_teilen_sich_eine_beschriftung` hält es fest. Die Zahl sieben ist mit
`Wirkungsbereich::Vorschau` aus der Runde 20 falsch geworden und steht seither da. Keine der
vierzehn Wortformen aus C9.4 trifft `sieben Beschriftungen`.

**Die Runde 23 macht diese Stelle nicht falsch — sie ist es schon.** Aber sie belegt, dass die
Erhebung, auf die C9.4 sich stützt, eine Untergrenze liefert und keine Zahl, und damit greift die
dritte Bedingung aus `## Stops when` des Specs: das Muster ist zu erweitern und die Erhebung zu
wiederholen, bevor gezählt wird.

Wortformen, die dieselbe Sorte Aussage in anderer Gestalt tragen und dem Muster fehlen, ohne
Anspruch auf Vollständigkeit: `sieben Beschriftungen`, `acht Wirkungsbereiche`, `vier fokussierbaren`,
`fuenf Kaesten`, `fuenf Rahmen`, `fuenf Teilbaeume`, `fuenf Werte`, `fuenf Werten`, `fuenf Bereichen`,
`sechs Bereiche`. Dazu die allgemeine Falle, die dieses Projekt schon einmal bezahlt hat: ein Muster,
das nur die heutige Zahl sucht, findet die morgige nicht.

**Abnahme:** das Muster aus C9.4 ist um mindestens die zehn Wortformen oben erweitert, die Erhebung
ist damit wiederholt, ihre neue Zahl steht im History-Eintrag des Nachzugsschritts, und
`belegungsausgabe.rs:234-235` nennt danach keine unrichtige Zahl mehr.
---
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
Gefunden beim Schreiben des Plans der Runde 23, bei der stellengenauen Erhebung für
`belegungsausgabe.rs` (Frage 8 aus `## Open for Planner`).
Verwandt: `260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`
(derselbe Befundtyp: fünf Erhebungen haben dieselben acht Stellen nicht gesehen, weil ihr Muster zu
eng geschnitten war).
