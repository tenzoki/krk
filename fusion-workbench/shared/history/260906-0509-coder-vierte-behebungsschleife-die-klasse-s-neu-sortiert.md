# coder: vierte Behebungsschleife — die Klasse S neu sortiert und abgetragen

**Status:** Complete
**Agent:** coder
**Baumstand bei Beginn:** `5cb5110`
**Grenze:** `crates/`, `xtask/`, und die Spec- und Plandateien unter `fusion-workbench/circles/*/planning/`, letztere ausschließlich als Nachsatz.

## Verification

```
make check — exit 0
```

Fünf Kommandos, das fünfte `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`. Vor dem
ersten Eingriff einmal gefahren (exit 0), nach der Baumarbeit einmal, nach den Nachsätzen einmal,
nach den Schließungen einmal.

## Der Anlass

Drei Behebungsbahnen haben 33 offene Defektdatensätze als „Aussage über einen eingefrorenen Spec-
oder Plantext, deshalb nicht behebbar" eingeordnet und unangetastet gelassen. Der Fehlschluss ist
benennbar: die Regel sagt, dass ein Spec nicht umgeschrieben wird — über den **Baum** sagt sie
nichts. „Ich darf den Spec nicht umschreiben" ist gelesen worden als „dieser Befund ist nicht
behebbar". Das sind zwei verschiedene Sätze.

## Der Bestand: 36 und nicht 33

Die Menge ist neu erhoben, nicht aus den Listen übernommen: alle 209 offenen Datensätze gelesen,
die 99 der Rundenverzeichnisse einzeln. **36 gehören zur Klasse S**, die Analyse
`shared/analyses/260906-0239-der-boden-des-bestands.md` zählt 33. Die drei zusätzlichen liegen im
gemeinsamen Speicher; die Analyse führt dort einen, es sind drei. **Nur 16 der 36 waren irgendwo
namentlich aufgeführt** — die neun in `260906-0202` und die sieben in der Cross-References-Zeile
von `260906-0203`.

| Eimer | Zahl |
|---|---|
| **A** — Mangel im lebenden Baum, der Spec nur Fundstelle | 6 |
| **B** — Berichtigung des eingefrorenen Textes durch Nachsatz | 29 |
| **C** — weder noch | 1 |

**A ist häufiger, als die drei Bahnen angenommen haben, und trotzdem nicht die Mehrheit.** Für
keinen der sechs A-Fälle hat `260906-0202` einen Handgriff genannt; für alle sechs gibt es einen.
Die Mehrheit bleibt B, und das ist die eigentliche Auskunft: 29 waren mit einem datierten Nachsatz
zu schließen, und der Nutzerentscheid dafür lag seit dem 260906 vor.

**Der Eimer B ist dabei weiter gefasst als der Auftrag ihn beschreibt.** Er nennt „veralteter
Zeiger in einem Beleg"; angewandt ist er auf jede Berichtigung, deren Ort der eingefrorene Text
ist — auch dort, wo nicht eine Zahl überholt ist, sondern eine Aussage von Anfang an nicht
zutraf (C1.1 der Runde 23) oder ein Eintrag fehlt (die Abnahmeliste der Runde 7). Anders wären
diese Fälle in den Eimer C gefallen und offen geblieben, und der Nutzer hat ausdrücklich
Reparatur verlangt.

## Die Form des Nachsatzes

Über alle zwölf Dateien gleich, am Dateiende, nach einer waagerechten Linie:

```markdown
---

## Nachsatz vom 260906-0448

**Spätere Zutat, lange nach dem Rundenabschluss angehängt. Der Bestandstext darüber ist Zeichen
für Zeichen unverändert** und bleibt der Wortlaut, gegen den diese Runde abgenommen wurde; wer
die Abnahmenotiz jener Runde liest, liest sie weiterhin gegen denselben Text. Zulässig nach der
Nutzerentscheidung zu
`shared/decisions/260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`:
berichtigt wird als Nachsatz und nicht im Text. Jede Angabe hier ist am Baumstand `5cb5110`
gemessen, und das Kommando steht dabei.

**1. <Ort im Text>: <was gilt>.** <Was dasteht>. Gemessen mit `<kommando>`: <ergebnis>.
Anlass: `issues/<datensatz>`.
```

Je Datei **ein** Abschnitt mit numerierten Punkten, einer je Datensatz — nicht ein Abschnitt je
Datensatz. Ein Spec mit fünf Nachsatz-Überschriften wäre unlesbarer als einer mit fünf Punkten
unter einer. Bei einem Plan lautet der zweite Halbsatz „gegen den diese Runde **gebaut und**
abgenommen wurde", bei einer nicht abgenommenen Runde „nach dem Bau angehängt".

## Was im Baum gebaut ist (Eimer A)

Sieben Proben, keine einzige davon eine Zeile Verhaltensänderung.

| Probe | Datei | Kriterium |
|---|---|---|
| `keine_datei_des_baums_baut_eine_web_ansicht` | `crates/krk-ui/src/appkit/vorschau.rs` | C4.5 der Runde 6 |
| `die_anwendungsweiten_befehle_wirken_aus_jedem_bereich_heraus` | `crates/krk-core/tests/belegung.rs` | C3.3 der Runde 7 |
| `das_hauptfenster_entsteht_an_genau_einer_stelle` | `crates/krk-ui/src/appkit/fenster.rs` | C3.12 der Runde 7 |
| `beide_sperrgriffe_der_ablage_tragen_must_use_mit_begruendung` | `crates/krk-core/tests/baum.rs` | C4.8 der Runde 7 |
| `die_readme_traegt_den_abschnitt_ueber_die_versionsstufen` | `xtask/src/release.rs` | C4.1–C4.7 der Runde 8 |
| `die_drei_raenge_stehen_in_der_zugesagten_reihenfolge`, `die_zwei_vorderen_raenge_springen_frueh_zurueck` | `crates/krk-ui/src/kommandos/operationen.rs` (Modul `abbruchrangfolge`) | C1.7 der Runde 10 |
| `genau_ein_durchgang_liest_die_markdown_quelle`, `der_quelltext_wird_kein_zweites_mal_von_der_platte_gelesen` | `crates/krk-ui/src/markdown.rs` (Modul `durchgaenge`) | C2.3 und C2.4 der Runde 14 |

`crates/krk-ui/src/appkit/fenster.rs` hatte kein Prüfmodul und bekommt seines mit dieser Schleife.

**Drei Entwurfsentscheidungen, die eine spätere Schleife nicht neu treffen muss:**

- **Die Probe zur `README.md` hält den heutigen Stand und nicht den Wortlaut von 260813.** Zwei der
  sieben C4-Aussagen sind vom Nutzer überholt: den Tag setzt seit dem 260813-1534 das Werkzeug, und
  `v0.1.0` steht in der Datei nicht mehr. Eine Probe, die den Wortlaut von damals einforderte, wäre
  heute rot, ohne dass etwas kaputt wäre.
- **Für C1.7 ist ein dritter Weg gegangen.** Der Befund legt zwei vor — eine reine Funktion samt
  Tafel über acht Fälle, oder das Kriterium ganz auf die Bündelhälfte — und hält gegen den ersten,
  dass er einen siebten Typ der Runde erzwänge; das ist Entwurfsarbeit und gehört dem Planer.
  Gebaut ist stattdessen eine Zählprobe am Quelltext, die die **Reihenfolge** misst, und genau die
  sagt C1.7 zu. Die Entwurfsfrage bleibt offen.
- **Eine Probe über die Zahl der eigenen Textflächen ist bewusst nicht entstanden.** C3 der Runde 9
  sagt „genau eine Ausnahme" zu; seit der Runde 14 sind es zwei. Die vorhandene Probe
  `die_menge_der_eigenen_textflaechen_steht_an_genau_einer_stelle` schreibt in ihrem Doc-Kommentar
  aus, warum sie die Zahl **nicht** zählt: eine dritte eigene Fläche wäre ein dritter Vergleich an
  derselben Stelle und eine zulässige Änderung, keine zweite Fassung. Eine Zählprobe daneben hätte
  dieser Festlegung widersprochen; die Sache steht als Nachsatz.

## Was beim Nachmessen abgefallen ist

**Drei A-Fälle waren ganz oder halb erledigt, ohne dass jemand die Datensätze angefasst hätte.**
Zwei der drei fehlenden Zählproben der Runde 6 standen längst im Baum (`C1.8`, `C6.6`), die
zweite der zwei Proben aus C3 der Runde 9 seit `210e4c1`, und C2.5 der Runde 7 ist von einer
Probe einer späteren Runde mitgedeckt. **Wer die Klasse S abarbeitet, misst vor dem Bauen.**

**Sechs der B-Fälle sind selbst veraltet, und der Nachsatz misst deshalb am Baum und schreibt
nicht vom Datensatz ab.** Die drei größten Abweichungen:

| Datensatz sagt | Baum sagt | gemessen mit |
|---|---|---|
| `EDITORGRENZE` steht an `datei.rs:164` (das Kriterium sagt `:153`) | `:186` | `grep -n 'pub const EDITORGRENZE' crates/krk-core/src/text/datei.rs` |
| der Spec zeigt sechs Stationen, der Baum sieben | der Baum trägt **acht** | `grep -rn 'sechs\|sieben\|acht Stationen' README.md Makefile xtask/src/*.rs` |
| `auftrag_starten` hat vier Rufer (der Plan sagt drei) | **sechs** | `grep -rn 'self\.auftrag_starten(' crates/krk-ui/src` |

Dazu: die Grundmenge der Abnahmekriterien der Runde 9 ist **77** und nicht 72 und nicht 75
(`grep -c '^- \[ \]'` über den Spec); die zusätzlichen Pakete der Runde 23 sind **101** und nicht
98 (`cargo tree --target aarch64-apple-darwin -e normal,build | … | wc -l` → 197 gegen 96);
`fn fokusansicht` steht **dreimal** und nicht zweimal, seit die Runde 23 den Git-Bereich gebracht
hat.

## Geschlossen: 30 von 36

Vier in den Runden 6 bis 8, sechs in der Runde 9, fünf in der Runde 10, fünf in den Runden 11 bis
14, zehn in den Runden 20 bis 23. Jeder mit `Resolved:`-Zeile am Ort und `_o_` → `_c_`. Die
Zählprobe `jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk`
(`xtask/src/werkbank.rs`) läuft danach grün.

Der offene Defektbestand fällt von 209 auf **180**.

## Offen geblieben: 6

- **Fünf hängen allein am Pfad ihrer Zieldatei** und sind als
  `shared/issues/260906-0509_*_fuenf-datensaetze-der-klasse-s-brauchen-einen-nachsatz-ausserhalb-von-circles-planning.md`
  erfasst: zwei zeigen auf `shared/planning/`, drei auf den Abschnitt `## Directive` eines
  Circle-Datensatzes. Die ersten zwei brauchen nur eine weitere Schreibfläche; die letzten drei
  eine Zuständigkeitsantwort, denn die Directive gehört dem Shaper und die Nutzerentscheidung vom
  260906 spricht von Spec und Plan.
  `circles/260813-0100-…/issues/260813-0642_*` ist dabei zur Hälfte erledigt — die Planhälfte steht
  als Nachsatz —, bleibt aber offen, weil er selbst sagt, Weg 1 allein lasse den Spec falsch
  stehen.
- **Einer ist Eimer C.**
  `shared/issues/260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-und-eine-suche-findet-nur-die-erste.md`
  verlangt eine **Umstellung** im Bestandstext, und die deckt die Nachsatzregel nicht ab. Vorgelegt
  als `shared/decisions/260906-0509_*_deckt-die-nachsatzregel-auch-eine-umstellung-im-bestandstext-ab.md`,
  drei Möglichkeiten, Empfehlung: die Regel scharf lassen und den einen Datensatz offen. Der
  Nachsatz am Spec der Runde 10 nennt inzwischen beide Abschnitte und ihre Stelle — weniger als
  eine Behebung und mehr als nichts.

## Nicht getan

`shared/decisions/260906-0203_*` bleibt auf `_a_`. Der Marker `_i_` verlangt einen Commit-Hash, und
diese Schleife committet nicht; die Antwort ist außerdem erst zu 30 von 36 Fällen umgesetzt, und
für die Directive-Fälle ist sie gar nicht gestellt.

`shared/decisions/260906-0202_*` — ob ein für immer unbehebbarer Datensatz offen bleibt — ist
**nicht** beantwortet. Sie gehört dem Nutzer. Diese Schleife macht sie weitgehend gegenstandslos:
von den 33, die sie meinte, sind 30 der 36 geschlossen, und keiner der sechs übrigen ist
unbehebbar.

## Geänderte Dateien

**Baum:**
`crates/krk-core/tests/baum.rs`, `crates/krk-core/tests/belegung.rs`,
`crates/krk-ui/src/appkit/fenster.rs`, `crates/krk-ui/src/appkit/vorschau.rs`,
`crates/krk-ui/src/kommandos/operationen.rs`, `crates/krk-ui/src/markdown.rs`,
`xtask/src/release.rs`.

**Nachsätze (zwölf Dateien):** die Spec- oder Plandatei der Runden 7, 8, 9 (beide), 10 (beide), 11,
13, 14, 20, 21 (beide), 22, 23 (beide).

**Datensätze:** 30 geschlossen, zwei neu abgelegt.
