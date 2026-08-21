Das Abnahmekriterium C6.3 enthält die Zeichenfolge, deren Abwesenheit es verlangt

---

C6.3 des Specs „Artefakt und Release" verlangt, dass die Zeichenfolge „sieben Stationen" nach
der Änderung „an keiner Stelle des Baums mehr steht". Das Kriterium selbst enthält sie, steht im
Baum und macht sich damit selbst unerfüllbar. Dieselbe Zeichenfolge tragen drei weitere
Werkbankdatensätze, die Aufzeichnungen eines Standes sind und ihren damaligen Wortlaut nach der
Ortsregel behalten.

---

**Schwere:** niedrig. Der Umfang der Runde ist nicht betroffen, die gemeinte Zusage ist klar, und
der Plan hat sie umsetzbar gefasst.
**Gefunden von:** planner, beim Durchgehen des Specs für den Plan vom 260821-1221
**Betroffen:** `shared/planning/260821-1115_o_spec-artefakt-und-release.md`, C6.3
**Domain:** code

## Der Befund im Einzelnen

Am 260821 gezählt, mit `grep -rl "sieben Stationen"` über den ganzen Baum ohne `.git` und ohne
`target`:

| Ort | Stellen | Art |
|---|---|---|
| `README.md` | 3 | Quellbaum, wird nachgezogen |
| `xtask/src/version.rs` | 2 | Quellbaum, wird nachgezogen |
| `xtask/src/main.rs` | 1 | Quellbaum, wird nachgezogen |
| `xtask/src/release.rs` | 1 | Quellbaum, wird nachgezogen |
| `shared/planning/260821-1115_o_spec-artefakt-und-release.md` | 1 | das Kriterium selbst |
| `circles/260813-0939-.../planning/260813-1110_c_plan-...md` | 1 | Aufzeichnung eines Standes |
| `circles/260813-0939-.../issues/260813-1345_o_...md` | 1 | Aufzeichnung eines Standes |
| `shared/history/260813-1556-coder-auslieferungsweg-in-einem-kommando.md` | 1 | Aufzeichnung eines Standes |

Die drei Aufzeichnungen behalten ihren Wortlaut, weil die Ortsregel für `planning/`, `issues/`
und `history/` das so vorsieht. Bliebe allein das Kriterium, und es kann sich nicht selbst
entfernen.

## Wie der Plan damit umgeht

Der Plan vom 260821-1221 begrenzt die Zusage auf den Quellbaum, also `README.md`, `Makefile` und
die `.rs`-Dateien unter `xtask/`; dort sind es die sieben Stellen der ersten vier Zeilen. Die
Zählprobe schreibt ihre Nadel als `concat!`, damit sie sich nicht selbst mitzählt. Genau diese
Bauart führt der Baum schon: `xtask_ruft_git_an_genau_einer_stelle` und
`dieser_weg_baut_nichts` begründen sie mit demselben Satz.

## Warum es der Rede wert ist, obwohl der Plan es löst

Der Spec ist vom Nutzer abgenommen und bindet. Wer C6.3 später wörtlich nimmt und gegen den
ganzen Baum misst, findet vier Treffer und hält die Runde für unvollständig, oder er schreibt
drei Aufzeichnungen um, die nicht umzuschreiben sind. Der Datensatz hält fest, warum die
Zusage im Plan enger steht als im Spec.

## Herkunft

Gemeinsamer Speicher. Der Spec liegt dort, kein Circle ist aktiv, und der Befund betrifft die
Auslieferungskette des ganzen Projekts.
