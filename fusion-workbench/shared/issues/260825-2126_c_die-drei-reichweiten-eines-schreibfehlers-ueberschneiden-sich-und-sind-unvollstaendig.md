Die drei Reichweiten eines Schreibfehlers überschneiden sich und sind unvollständig

---

`resources/default-readers.toml:41-55` stellt dem Nutzer die drei Reichweiten vor, in denen
ein Schreibfehler in dieser Datei wirkt. Die Aufzählung ist weder überschneidungsfrei noch
vollständig. Reichweite 1 nennt „ein unbekannter Schlüssel" ohne Einschränkung und
beansprucht damit einen Fall, der in Wahrheit in Reichweite 2 fällt; Reichweite 3 lässt zwei
Gründe aus, die es seit dieser Runde gibt und die der Quelltext ausdrücklich führt.

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:41-55`, `:77-79`, `:99-102`;
`crates/krk-core/src/leseprofil/datei.rs` (Modulköpfe „Wo `deny_unknown_fields` steht und wo
nicht" und „Was abgewiesen wird, und wie weit")

## Die Überschneidung

`deny_unknown_fields` steht an sechs Stellen: an der Profildatei, an der Zeile und an jedem der
vier Bausteintische. **`Profilblock` trägt ihn nicht**, und das ist eine bewusste Wahl, die der
Quelltext begründet: ein verschriebenes `pfad` lässt das Profil ohne Erkennungsmuster zurück,
und genau das weist `pruefen` mit einer eigenen Meldung ab.

Gemessen am 260825-2126 über `leseprofil::datei::pruefen`, Baum `8478753`:

| Verschrieben | Reichweite laut Datei | gemessen |
|---|---|---|
| `kennzeichnen` statt `kennzeichen` (Profilebene) | 1, „ein unbekannter Schlüssel" | **2** — Datei gültig, ein Profil fällt, Meldung: „es nennt weder ein Pfadmuster noch eine Kennzeichendatei" |
| `nane` statt `name` (Profilebene) | 1 | 1 (Pflichtfeld fehlt) |
| `musster` statt `muster` (im Bausteintisch) | 1 | 1 |
| `beschriftunng` statt `beschriftung` (Zeilenebene) | 1 | 1 |
| `zahlung` statt `zaehlung` (Tischname) | 1 | 1 |
| `zeigt = "beides"` | 1 | 1 |

Die erste Zeile ist der Fall, in dem sich die zwei Reichweiten widersprechen. Reichweite 2
nennt ihn korrekt mit („oder es nennt keines von beiden"), Reichweite 1 nennt ihn zu Unrecht
mit. Der Quelltext formuliert dieselbe Aussage einschränkend richtig: „ein unbekannter
Schluessel **an einer der sechs Stellen mit `deny_unknown_fields`**".

## Die Lücke

Reichweite 3 zählt fünf Gründe auf: nicht genau ein Baustein, ein nicht übersetzbares Muster,
ein `feldmuster` ohne genau eine Fanggruppe, ein `ordner`, der aus dem erkannten Ordner
herausführt, und ein Platzhalter an einem Baustein, der keinen annimmt. Zwei weitere Gründe
liefern ebenfalls Reichweite 3 und stehen nicht in der Liste:

| Grund | gemessen |
|---|---|
| `ordner = "planning/"` (leeres Stück) | Zeile verliert ihren Baustein; Meldung: „die Ortsangabe \"planning/\" traegt ein leeres Stueck" |
| `ordner = "*/*/x"` (zwei Platzhalter) | Zeile verliert ihren Baustein; Meldung: „…traegt mehr als einen Platzhalter" |

Beide stehen anderswo in derselben Datei — der abschließende Schrägstrich bei `:77-79` mit
der richtigen Folge („kostet die Zeile trotzdem ihren Baustein"), die zwei Platzhalter bei
`:99-100` mit „werden beim Laden abgewiesen" und **ohne** Angabe der Reichweite. Wer die
Reichweite dort sucht, wo sie steht, findet für die zwei Platzhalter keinen passenden Zweig
und landet über „sonst etwas, das nicht in die erwartete Gestalt passt" bei Reichweite 1, also
beim Verlust der ganzen Datei.

## Warum das zählt

Die Aufzählung ist der einzige Ort, an dem ein Nutzer erfährt, was ihn ein Tippfehler kostet,
und ihr Zweck ist die Abschreckung an der richtigen Stelle. Ein Zweig, der zu viel
beansprucht, macht sie unglaubwürdig; einer, der zu wenig nennt, schickt den Nutzer zum
falschen Schluss. Der Quelltext hat beide Fälle richtig, die Auslieferungsfassung hat sie
nicht.

## Was zu tun wäre

Zwei Änderungen an `resources/default-readers.toml:41-55`:

1. In Reichweite 1 „ein unbekannter Schlüssel" einschränken auf einen unbekannten Schlüssel in
   einem Bausteintisch, in einer Zeile oder an der obersten Ebene — und dazusagen, dass ein
   unbekannter Schlüssel im `[[profil]]`-Block selbst in Reichweite 2 fällt.
2. In Reichweite 3 das leere Stück und den zweiten Platzhalter aufnehmen.

**Schwere:** mittel. Kein Bau hängt daran; die eine Aufstellung, aus der ein Nutzer die Folgen
seines Tippfehlers liest, ist an zwei Stellen falsch.

---
Resolved: Reichweite 1 nennt jetzt die oberste Ebene, die Zeile und den Bausteintisch als Orte des unbekannten Schlüssels, dazu den fehlenden `name` und den unbekannten Wert für `zeigt`; ein neuer Absatz sagt, dass der `[[profil]]`-Block einen unbekannten Schlüssel übergeht und der Preis das ist, was der Schreibfehler weggenommen hat (`kennzeichnen` → Reichweite 2, `zeilen` → Profil ohne Zeile, ohne Meldung). Reichweite 3 führt das leere Stück und den zweiten Platzhalter; der Platzhalter-Absatz nennt seine Reichweite. Nachgemessen am 260825 über `leseprofil::datei::pruefen` an sieben abgewandelten Fassungen: `kennzeichnen` → 11 Profile, Meldung „weder ein Pfadmuster noch eine Kennzeichendatei"; `nane` → Datei fällt (TOML-Fehler); `zeilen` → 12 Profile, keine Meldung; Fremdschlüssel `foo = 1` im Block → 12 Profile, keine Meldung; `circles/` → „traegt ein leeres Stueck"; `*/*/x` → „mehr als einen Platzhalter"; `zeigt = "beides"` → Datei fällt.
