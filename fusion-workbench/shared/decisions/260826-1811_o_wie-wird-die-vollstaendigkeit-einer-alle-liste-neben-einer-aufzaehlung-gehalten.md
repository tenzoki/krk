# Wie wird die Vollständigkeit einer Liste neben einer Aufzählung gehalten: Quelltextprobe oder Ableitungsmakro?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`; `shared/issues/260826-1302_*_ein-achter-wirkungsbereich-uebersetzt-ohne-eintrag-im-beschriftungsfeld-der-doc-kommentar-sagt-das-gegenteil.md`; `shared/planning/260826-1811_*_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md` (Schritt 4); `crates/krk-core/tests/baum.rs` (die Quelltextproben über `gemeinsam::quelldateien()`)

---

## Question

Der Baum führt elf Konstanten der Form `ALLE: [Aufzaehlung; N]` und daneben `Kommando::KENNUNGEN` mit 79 Paaren (`grep -rn 'const ALLE' crates/*/src`). Keine davon hält der Übersetzer vollständig: die Längenangabe zwingt zu N Einträgen und sagt nichts darüber, welche N. Zwei Befunde der Vollbaum-Durchsicht vom 260826 hängen daran, `KENNUNGEN` und `SIEBEN_BESCHRIFTUNGEN`. Der Plan über die fünf schweren Befunde behebt den ersten mit einer Probe, die die Varianten aus dem Quelltext der Aufzählung liest und gegen die Liste hält. Die Frage, die über diesen einen Fall hinausreicht, ist, ob das die Bauform für alle elf wird oder ob das Projekt dafür eine fremde Kiste mit einem Ableitungsmakro einbindet. Zu entscheiden ist sie jetzt, weil der zweite Plan über die 116 übrigen Befunde denselben Griff an `Wirkungsbereich` tun wird und beide Stellen dieselbe Form tragen sollten.

Stabiles Rust kennt keine Zählung der Varianten einer Aufzählung (`std::mem::variant_count` ist nicht stabilisiert). Ohne Makro bleibt jede Vollständigkeitsaussage eine Probe zur Laufzeit; mit Makro wird sie ein Übersetzungsfehler.

## Options

1. **Quelltextprobe über `gemeinsam::quelldateien()`** — ein Helfer in `tests/gemeinsam/mod.rs` liest den Block `pub enum <Name> { … }` aus der genannten Datei und liefert die Variantennamen; die Probe hält sie gegen `format!("{:?}")` jedes Listeneintrags.
   - Pros: keine neue Kiste; dieselbe Bauform, die `tests/baum.rs` seit der Runde 7 für Aussagen über den Baum fährt; ein Helfer für alle elf Listen; das Zählkommando aus `CLAUDE.md` (`awk '/^pub enum Kommando/,/^}/'`) ist dieselbe Lesart.
   - Cons: eine Nadel im Quelltext, mit den Blindheiten, die `baum.rs` in seinem Kopf ausschreibt; hält zur Laufzeit der Proben und nicht beim Übersetzen; eine Variante mit Daten oder ein Kommentar mit `,` am Zeilenende verlangt eine sorgfältige Nadel.
2. **`strum` mit `EnumIter` und `EnumCount`** — die Aufzählung leitet ihre Iteration ab, `KENNUNGEN` wird aus ihr gebaut oder gegen `Kommando::COUNT` beim Übersetzen gehalten.
   - Pros: die Zusage steht dort, wo sie behauptet wird, und ein fehlender Eintrag ist ein Übersetzungsfehler; keine Nadel.
   - Cons: eine fremde Kiste samt `strum_macros` für eine Zeile je Aufzählung; jede fremde Kiste dieses Projekts trägt in der Wurzel-`Cargo.toml` ihre Begründung und ihre Merkmalsauswahl; `syn` steht schon in `Cargo.lock`, die Bauzeit wächst also wenig, aber der Kreis der Abhängigkeiten wächst um eine Kiste, die KRK zur Laufzeit nicht braucht.
3. **Eine `const fn stelle(self) -> usize` mit vollständigem `match` und einer `const _`-Zusicherung über die Liste** — ohne Kiste, beim Übersetzen.
   - Pros: keine Kiste, Übersetzungsfehler statt roter Probe.
   - Cons: die Nummern im `match` sind eine zweite Liste von Hand; eine neue Variante mit einer schon vergebenen Nummer entgeht der Zusicherung, weil diese nur über die Liste laufen kann. Die Lücke ist kleiner als heute, aber sie bleibt, und die Form ist 79 Zeilen Zahlen.

## Constraints

- Kein `unsafe` außerhalb von `sys.rs` und `appkit/mod.rs`; ein Weg über `transmute` aus dem Diskriminanten scheidet aus.
- Jede fremde Kiste steht mit Begründung und Merkmalsauswahl in der Wurzel-`Cargo.toml`; auf dem Bauziel darf kein C-Code entstehen.
- Die Antwort gilt für alle elf `ALLE`-Listen gleich; zwei Bauformen nebeneinander wären die Thicket-Lage, die `critical-stance.md` §2 benennt.

## Recommendation

Wir empfehlen Möglichkeit 1 für jetzt, mit ausdrücklicher Wiedervorlage, sobald `variant_count` stabilisiert ist oder eine dritte Liste denselben Griff braucht. Der Plan über die fünf schweren Befunde fährt sie an `KENNUNGEN`; sie ist mit Möglichkeit 2 verträglich, weil der Helfer und die Probe bei einem späteren Umstieg gestrichen und nicht umgebaut werden. Möglichkeit 3 raten wir ab: sie ersetzt eine Liste durch zwei.

---
Reconciled: 260826-2205 — bleibt offen. Der Plan `260826-1811` hat Möglichkeit 1 an **einer**
Liste gefahren (`crates/krk-core/tests/belegung.rs:1760` gegen `Kommando::KENNUNGEN`), und der
Helfer `varianten_der_aufzaehlung` (`crates/krk-core/tests/gemeinsam/mod.rs:411`) führt bewusst
keinen Aufzählungsnamen, damit der zweite Plan ihn an `Wirkungsbereich` wiederverwenden kann.
Die Frage dieses Datensatzes ist damit **nicht** beantwortet: sie lautet, ob Möglichkeit 1 die
Bauform für alle elf `ALLE`-Listen wird oder ob das Projekt dafür `strum` einbindet, und das
entscheidet der Nutzer und nicht ein Planschritt. Gesucht und nicht gefunden: keine
`Answered:`-Zeile im Datensatz; `shared/planning/` führt keinen zweiten Plan; kein Gate der
Sitzung `260826-1807` hat die Frage vorgelegt (`orchestrator-events.jsonl`, die zwei
`gate_response` der Sitzung betreffen die Planfreigabe und das Kohärenz-Gate). Die Antwort
bindet den zweiten Plan über die 116 übrigen Befunde, unter anderem
`shared/issues/260826-1302_*_ein-achter-wirkungsbereich-uebersetzt-ohne-eintrag-im-beschriftungsfeld-der-doc-kommentar-sagt-das-gegenteil.md`.
