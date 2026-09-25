Der Doc-Kommentar am Bausteinsatz sagt drei Zustandszeilen, die Auslieferungsfassung trägt vier

---

`crates/krk-core/src/leseprofil/mod.rs:259-265` begründet den festen Bausteinsatz mit
Festlegung A7:

```
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig. Ein fuenfter
/// Baustein haelt jeden Rechner an und erzwingt eine bewusste Einordnung;
/// Festlegung A7 haelt die Zahl vier fest und nennt sie ausdruecklich als das,
/// was den Zustand eines Circles auf drei Vorhandensein-Zeilen verteilt,
/// statt einen Baustein fuer Dateinamen aufzunehmen.
```

Seit Schritt 14 der Runde 16 sind es vier. `resources/default-readers.toml` trägt im Profil des
einzelnen Circles die Beschriftungen `Vorgesehen` (Zeile 271), `Aktiv` (275), `Geschlossen`
(279) und `Abgelegt` (283); die vierte hat der Nutzer am 260824-1505 beschlossen
(`decisions/260824-0634_i_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`).
Der Spec trägt die Berichtigung unter C5.6, und A7 bleibt im Wortlaut mit einem Verweis darauf
stehen.

Diese Codezeile ist die einzige Stelle im Quellbaum, die die Zahl noch nennt; nachgezählt am
260824-1649 mit `grep -rn "drei Vorhandensein\|drei Zustandszeilen\|A7" crates/`.

---

**Warum es zählt.** Der Kommentar trägt die tragende Hälfte von A7 — vier Bausteine und kein
fünfter —, und die hält unverändert. Die Zahl daneben ist der Beleg dafür, was der Preis dieser
Wahl ist, und wer ihn heute nachzählt, findet vier Zeilen und hält den Kommentar für veraltet
oder die Datei für falsch. Es ist genau die Art Prosastelle, die dieses Projekt in `CLAUDE.md`
viermal in vier Tagen verloren hat.

**Was zu tun ist.** Der Satz nennt entweder vier Zeilen und verweist wie A7 auf die
Berichtigung, oder er lässt die Zahl weg und sagt „auf mehrere Vorhandensein-Zeilen". Das
zweite ist die haltbarere Form: die Zahl steht bereits in der Auslieferungsfassung und im
Spec, und eine dritte Stelle dafür ist eine dritte, die veraltet.

**Schwere:** niedrig.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1649.

**Betroffen:** `crates/krk-core/src/leseprofil/mod.rs` (Doc-Kommentar an `enum Baustein`, Zeile 263-265)

**Domain:** code

---
Resolved: 260824-1740 vom coder. Die haltbarere der zwei vorgeschlagenen Formen gewählt: der Satz nennt keine Zahl mehr, sondern „mehrere Vorhandensein-Zeilen", und sagt ausdrücklich, dass die Zahl in `resources/default-readers.toml` steht und eine dritte Stelle dafür eine dritte wäre, die veraltet. Die tragende Hälfte von A7 — vier Bausteine und kein fünfter — bleibt unverändert stehen.
