Der Satz über `kennzeichnen` gilt nur für ein Profil ohne `pfad` daneben

---

`resources/default-readers.toml:63-65` sagt: „`kennzeichnen` statt `kennzeichen` lässt das
Profil ohne Erkennungsmuster zurück und fällt in die zweite Reichweite." Das stimmt für ein
Profil, das allein über `kennzeichen` erkennt, und für jedes der zwölf mitgelieferten. Es
stimmt nicht für ein Profil, das daneben einen `pfad` trägt: dort übergeht der
`[[profil]]`-Block den verschriebenen Schlüssel still, das Profil bleibt mit seinem `pfad`
allein stehen, und keine Meldung sagt, dass die zweite Erkennung fehlt.

---

**Filed by:** ontorev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `resources/default-readers.toml:60-66`;
`crates/krk-core/src/leseprofil/datei.rs`, Modulkopf „Wo `deny_unknown_fields` steht und wo
nicht" und `pruefen`;
`shared/issues/260825-2126_c_die-drei-reichweiten-eines-schreibfehlers-ueberschneiden-sich-und-sind-unvollstaendig.md`

## Was gemessen ist

Gemessen am 260825-2233, Baum `1ac5dde`, über `leseprofil::datei::pruefen` an einer
abgewandelten Fassung der Datei, in der die zwei Profile mit dem Pfadmuster
`fusion-workbench/(shared|circles/[^/]+)/…` je eine Zeile `kennzeichnen = 'x'` dazubekommen:

```
Profile: 12
Meldungen: []
fusion-workbench/shared/history  →  Profil „ein Speicher", 1 Leselauf, 10 Öffnungen
```

Zwölf Profile, keine Meldung, das Profil greift über seinen `pfad` wie zuvor. Zum Vergleich
dieselbe Verschreibung am Wurzelprofil, das allein `kennzeichen` trägt: elf Profile und die
Meldung „es nennt weder ein Pfadmuster noch eine Kennzeichendatei" — das ist der Fall, den
der Satz beschreibt.

Der Quelltext sagt beides: `Profilblock` trägt kein `deny_unknown_fields`, und `pruefen`
meldet allein den Fall `pfad.is_none() && kennzeichen.is_none()`. Der Modulkopf von
`datei.rs` begründet den fehlenden `deny_unknown_fields` mit genau dem Satz, der jetzt in der
Profildatei steht („ein verschriebenes `pfad` laesst das Profil ohne Pfadmuster und ohne
Kennzeichen zurueck, und genau das weist `pruefen` mit einer Meldung ab") — und trägt
dieselbe Lücke.

## Warum das zählt

Der Absatz `:60-66` ist neu und ist die Antwort auf den Befund M5 der ersten Durchsicht: er
soll dem Nutzer sagen, was ein Schreibfehler im `[[profil]]`-Block kostet. Die Datei sagt
`:72-74` ausdrücklich, dass ein Profil `pfad` **oder** `kennzeichen` **oder beides** tragen
darf. Wer dem dritten Weg folgt und sich beim zweiten Schlüssel verschreibt, bekommt nicht
die zweite Reichweite mit Meldung, die der Absatz zusagt, sondern den einen Fall in dieser
Datei, der weder eine Meldung noch einen Platzhalter zeigt: das Profil greift, nur an weniger
Orten als geschrieben. Die Zusage „und das ohne jede Meldung" am Satzende gilt heute für
`zeilen`; für `kennzeichnen` gilt sie je nach Nachbarschlüssel auch, und der Absatz sagt das
Gegenteil.

Schwere **niedrig**: kein mitgeliefertes Profil trägt beide Schlüssel, und der Fall entsteht
erst durch eine Nutzeränderung.

## Möglichkeiten

1. Ein Halbsatz in `:63-65`: „… fällt in die zweite Reichweite — oder, steht ein `pfad`
   daneben, lässt das Profil still mit dem `pfad` allein stehen, ohne Meldung." Eine Zeile,
   keine Änderung am Mechanismus.
2. `deny_unknown_fields` an `Profilblock`, womit der Fall in die erste Reichweite fiele wie
   jeder andere unbekannte Schlüssel; dann fällt der ganze Absatz `:60-66` weg. Das ist eine
   Entscheidung am Quelltext, die der Modulkopf von `datei.rs` bewusst anders getroffen hat,
   und gehört zu `coder`, nicht in diese Datei.

Die erste Möglichkeit ist die, die zum Zuschnitt der Runde passt.
---
Resolved: Möglichkeit 1 gewählt. `resources/default-readers.toml:61-67` sagt jetzt: „`kennzeichnen`
statt `kennzeichen` nimmt dem Profil sein Erkennungsmuster, und es fällt in die zweite Reichweite
oder greift, steht ein `pfad` daneben, still über diesen allein". Der Mechanismus ist nicht
angefasst; `Profilblock` trägt weiter kein `deny_unknown_fields`. Beide Lagen am 260826-0124 über
`leseprofil::datei::pruefen` an abgewandelten Fassungen gemessen: `kennzeichnen = 'x'` neben dem
`pfad` der zwei Speicherprofile → 12 Profile, keine Meldung, `fusion-workbench/shared/history`
bekommt sein Profil mit 1 Leselauf und 10 Öffnungen; `kennzeichnen` statt `kennzeichen` am
Wurzelprofil, das keinen `pfad` trägt → 11 Profile und die Meldung „es nennt weder ein Pfadmuster
noch eine Kennzeichendatei", `fusion-workbench` ohne Profil. `cargo test -p krk-core --lib
leseprofile`: 10 grün, `die_eingebettete_fassung_besteht_ihre_eigene_pruefung` darunter.
Die gleichlautende Lücke im Modulkopf von `crates/krk-core/src/leseprofil/datei.rs` („Wo
`deny_unknown_fields` steht und wo nicht") ist nicht berührt: sie gehört `coder`.
