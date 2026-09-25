# Abschluss-Abgleich der Editor-Runde — 260810-1404

**Agent:** `reconciler`
**Domäne:** `code`
**Anlass:** Abschluss-Abgleich vor dem Schließen des Circles `260807-2116-eingebauter-editor-mit-textmarken`
**Sitzung:** `history/260810-0845-orchestrator-session.md`, Commits `38a02b2..0140df7` (17 Commits)
**Status:** Complete

---

## Der eine Satz

**Keine der 52 Behebungen dieser Sitzung ist erfunden.** Jede ist am Arbeitsbaum
belegt. Was der Bestand an Abweichungen trägt, ist Buchhaltung: ein Marker, der
nicht nachgezogen war, vierzehn Datensätze mit einem Fingerzeig auf die falsche
Zeile, sechs sachlich falsche Angaben in Behebungstexten, ein leerer Beleg in
einem Entscheidungsdatensatz und vier Kopfzeilen, die einen Marker behaupten, den
der Bestand nicht mehr trägt.

## Zahlen

| Gegenstand | Zahl | Anmerkung |
|---|---|---|
| Plandateien geprüft | 2 | Plan `_c_`, Spec `_o_`; beide mit Abgleichseintrag versehen |
| Defektdatensätze geprüft | 96 | 85 im Circle, 8 im gemeinsamen Speicher, 3 aus der Runde 1 mitgelesen |
| Defektdatensätze geändert | 3 | ein Marker, zwei Anmerkungen |
| Entscheidungsdatensätze geprüft | 58 | 13 Circle, 6 gemeinsam, 34 Runde 1, 5 vorgesehene Circles |
| Entscheidungsdatensätze geändert | 3 | ein Beleg nachgetragen, zwei Anmerkungen |
| Durchsichten annotiert | 2 | die beiden dieser Sitzung |
| Neue Defekte angelegt | 1 | `issues/260810-1404_o_vierzehn-geschlossene-datensaetze-…` |
| Abweichungen gefunden | 26 | keine an einer behaupteten Behebung |

## Der Baum, selbst gefahren

Nicht übernommen, sondern gefahren, mit `export PATH="$HOME/.cargo/bin:$PATH"`:

```
cargo test --workspace       16 Ziele, 753 Proben bestanden, 0 Fehlschläge, 1 übergangen
cargo clippy --workspace --all-targets    keine Meldung
cargo fmt --all --check                   keine Ausgabe
```

Die eine übergangene Probe ist `crates/krk-core/tests/ablage.rs:1188`, eine
Kindprobe, die der Elternteil über `KRK_PROBE_ABBRUCH` startet; das `#[ignore]`
trägt seinen Grund am Attribut. Die Durchsicht vom 260810-1248 nennt 744 Proben,
diese Zählung 753 — der Unterschied sind die Proben, die die drei letzten Commits
`bf0fe18`, `0140df7` und `1472846` mitgebracht haben.

---

## 1. Die 52 geschlossenen Defekte gegen den Code

Ausgangslage am Commit `38a02b2`: 57 Datensätze im Circle, davon 28 mit dem
Marker `_o_`. Heute: 86 Datensätze, davon 5 offen (die vier des Nutzers und der
in diesem Abgleich neu angelegte). Gerechnet: 28 offene am Anfang, 28 in der
Sitzung neu angelegte, 52 geschlossen.

**Die Zählung des Nutzers weicht in zwei Stellen ab, und der Bestand entscheidet.**
Der Auftrag nennt 26 neu gefundene und 50 geschlossene; es sind 28 und 52. Die
zwei zusätzlichen sind über `git ls-tree 38a02b2` gegen den heutigen Bestand
ermittelt, nicht geschätzt. Die 28 offenen am Anfang und die 4 offenen am Ende
stimmen genau.

**Alle 52 sind einzeln gegen den Baum gelesen.** Für jeden Datensatz ist der
Behebungsteil auf prüfbare Behauptungen zerlegt worden — Datei, Funktion,
Aufzählungswert, Probenname, Modulkopfzeile, Zahl — und jede Behauptung mit
`grep` oder Lesen am Arbeitsbaum belegt:

| Verdikt | Zahl |
|---|---|
| vollständig gedeckt | 45 |
| gedeckt, mit abgewanderter oder falscher Nebenangabe | 7 |
| **nicht gedeckt** | **0** |

**Der Fund, nach dem der Auftrag ausdrücklich gefragt hat — eine behauptete
Behebung, die im Code nicht steht — gibt es nicht.** Das ist das Ergebnis und
nicht eine Auslassung: die Prüfung ist auf ihn zugeschnitten gewesen und hat ihn
nicht gefunden.

**Was die Prüfung nicht leisten kann, und das bleibt getrennt.** Jede
Laufzeitzahl der Datensätze ist unbelegt geblieben: die Millisekunden und MB/s
der Einfärbungsmessung, die Anlagenzählungen des Richtens, die Bytemessungen des
Stapelbudgets, die AppKit-Messungen an macOS 15.7.7, die Gegenproben, die einen
Lauf beschreiben. Statisch belegbar ist, dass der Code die gemessene Größe
überhaupt bildet, und dass die Probe existiert, die sie hält. Ob die Zahl selbst
stimmt, ließe sich nur durch Nachmessen klären. Bei vier Zahlen ist es
gegenstandslos, weil eine Probe sie bei jedem `cargo test` mitführt.

### Die 26 Abweichungen

Alle in `issues/260810-1404_o_vierzehn-geschlossene-datensaetze-zeigen-auf-zeilen-die-ihre-eigene-sitzung-verschoben-hat.md`
einzeln aufgeführt, mit Ursache und heutiger Stelle. Zusammengefasst:

- **Vierzehn Defektdatensätze** zeigen auf mindestens eine Zeile, die es so nicht
  mehr gibt. Dreizehnmal hat ein späterer Commit derselben Sitzung sie verschoben,
  am häufigsten das Stapelbudget `0140df7` und die Öffnungsherkunft `8807844`.
- **Sechs Angaben sind nicht gewandert, sondern falsch.** Die schärfste steht in
  `260810-0303`: „alle sieben Aufrufstellen nennen ihn", und es sind acht, schon
  am behebenden Commit. Das ist ein Zählfehler und keine Abwanderung. Drei davon
  sind in diesem Abgleich am Datensatz selbst berichtigt.
- **Neun Belegzeilen in Entscheidungsdatensätzen** zeigen ebenfalls daneben, fünf
  davon durch diese Sitzung verschoben.

Die vorgeschlagene Behebung ist keine neue Regel, sondern die, die diese Sitzung
an drei anderen Stellen schon angewandt hat: die Zahl weglassen, das Stück
benennen.

## 2. Die fünf geschlossenen Defekte ohne kanonische `Resolved:`-Zeile

Die Konvention verlangt am Ende eines geschlossenen Defekts einen Block
`---\nResolved: …`. Fünf der 81 geschlossenen Datensätze im Circle tragen ihre
Abschlussnotiz in anderer Form, und alle fünf tragen sie:

```text
260808-0931  "**Geschlossen.** Der Befund traf zu, …"      (Zeile 72)
260808-1413  "## Resolved: 260809-1527, gegenstandslos …"  (Zeile 85, als Überschrift)
260809-1631  "Resolved am 260810-0204 bei der Umsetzung …" (Zeile 51, im Satz)
260809-1738  "## Behoben am 260809 mit S43"                (Zeile 84)
260809-2029  "**Geschlossen.** Die drei Fragen sind …"     (Zeile 76)
```

**Nicht angefasst, und der Grund gehört dazu.** Eine zweite Zeile daneben zu
setzen erzeugt genau die Doppelung, die dieser Circle schon einmal als Defekt
geführt hat (`issues/260808-0021_*_die-fuenf-beantworteten-datensaetze-tragen-zwei-answered-zeilen-und-einen-veralteten-kopf.md`).
Der Bestand ist damit inhaltlich vollständig und maschinell nicht durchgängig
greppbar; wer die Form vereinheitlichen will, ersetzt die vorhandene Notiz und
stellt keine zweite daneben. Die Entscheidung gehört dem Nutzer.

## 3. Der eine Marker, der nicht zum Bestand passte

`shared/issues/260809-1106_o_die-probenordner-der-vorschau-tragen-feste-namen-im-temporaerverzeichnis.md`
stand auf offen, obwohl sein Gegenstand am Code erledigt war. Derselbe Gegenstand
lief im aktiven Circle als `260810-1256` und ist dort geschlossen: die sieben
festen Ordnernamen in `crates/krk-ui/src/vorschaumodell.rs` sind fort, alle
sieben Rufe gehen über `Pruefordner::neu`, der Name trägt Prozesskennung und
Laufnummer, `Drop` räumt ab.

**Umbenannt auf `_c_` mit einer `Resolved:`-Zeile**, die zwei Reste ausdrücklich
mitnimmt: die beiden Fehlschläge vom 260809 um 11:00 sind bis heute nur mit dem
naheliegendsten Verdacht erklärt und nicht mit einer belegten Ursache, und dass
dieselbe Bauform zwölfmal im Baum steht, ist ein anderer Gegenstand und läuft
weiter als `260810-1330`.

Es ist der **einzige** Marker im ganzen Bestand, der nach dieser Sitzung nicht
mehr zum Code passte. Alle 81 `_c_` im Circle tragen eine Abschlussnotiz, keiner
der offenen trägt eine.

## 4. Der Plan

`planning/260808-0140_c_plan-eingebauter-editor-mit-textmarken.md`.

**Die Zusage des Auftrags hält, am Text nachgemessen.** Die 48 Schrittmarken der
Form `#### N. [DONE]` sind gegenüber `38a02b2` unverändert, `[IN PROGRESS]` kommt
nicht vor, und Zeile 4 mit dem Kopfstatus ist byteweise dieselbe. Die zwei
Commits, die den Plan angefasst haben, sind `8d59993` und `c0b96a6`, und beide
sind Prosa.

Eine `grep`-Zählung auf `[DONE]` steigt von 55 auf 56. Der Zuwachs ist der Satz
„Kein `[DONE]` und kein Status im Kopf ist angefasst" selbst, der die
Zeichenfolge in Klammern trägt. Wer nur zählt, liest hier einen Verstoß, wo eine
Zusage steht.

**Die drei Berichtigungen sind am Code belegt**, jede einzeln:

1. Die Schnittstellen-Regel und der Nachzug von S11: `leistenmodell.rs` steht in
   S11s Dateiliste (Zeile 653), das Abnahmekriterium verlangt
   `cargo build --workspace` (Zeile 659), und der Umsetzungsvermerk nennt
   `65c8efa` (Zeile 661), dessen Commit-Botschaft den Nachzug ausschreibt.
2. Das vierte Kriterium von S32 ist geteilt (Zeilen 992 bis 993), die
   Ersatzmessung steht bei Zeile 1000, das Bündelkriterium in S33 bei Zeile 1014
   mit der Messung bei 1015.
3. `dump-create` ist an beiden Stellen gestrichen (Zeilen 104 bis 106 und 994),
   samt Begründung.

Dazu die drei gekürzten Defektverweise bei Zeile 1352, jetzt mit vollem Namen und
Sternstelle. Zeile 716 trägt weiter eine gekürzte Form mit festem Marker `_o_`,
und der Datensatz `260810-0918` vermerkt sie ausdrücklich als das, was
nachbleibt.

**Alle 40 Verweise des Plans lösen auf**, 39 mit Sternstelle und einer mit festem
Marker (Zeile 1013, er trifft). Kein toter Verweis, auch nicht nach den 52
Umbenennungen.

**Eine Kopfzeile ist veraltet.** Zeile 6 nennt „die sechs `_a_`-Datensätze unter
`circles/260807-2116-…/decisions/`". Kein Datensatz dieses Speichers trägt `_a_`:
es sind zehn `_i_`, zwei `_o_` und ein `_s_`. Sie steht im Kopf, wo die
Sternstellen-Regel des Plans ausdrücklich nicht greift, und der `reconciler`
schreibt keine Kopfzeilen um. Als Abgleichseintrag im Plan vermerkt.

## 5. Der Spec und sein Marker

`planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, in dieser
Sitzung nicht angefasst.

**Die Zählung hält.** 108 Kästchen in den elf Fähigkeiten, zwei im
Zeitzusagen-Abschnitt, zusammen 110, davon null abgehakt. Je Fähigkeit
nachgezählt: C1 sieben, C2 elf, C3 zwölf, C4 elf, C5 neun, C6 vierzehn, C7 acht,
C8 fünf, C9 acht, C10 zwölf, C11 elf.

**`_o_` passt nicht zum Bestand, und der Bestand gibt eine Antwort her.** Die
Frage des Auftrags lautet, ob `_o_` der richtige Marker ist oder ob der Bestand
etwas anderes hergibt. Er gibt etwas anderes her, und zwar aus der Runde 1:

```text
circles/260802-0842-…/planning/260802-1036_c_spec-navigator-geruest.md
  110 Kästchen, davon 0 abgehakt
  **Status:** Complete
  Marker _c_, gesetzt zum beschränkten Abschluss am 260807-1035
```

**Der Spec der Runde 1 ist in genau derselben Lage und trägt `_c_`.** Er sagt es in
seiner Statuszeile selbst: „Die Beschränkung betrifft allein den Beleg der
Zeitzusagen aus C8, nicht die Abnahmekriterien." Das Kästchen ist in diesem
Projekt also nicht die Größe, an der der Marker eines Specs hängt.

Der Begründungssatz, den der Spec der Runde 2 für sein `_o_` anführt — „ein Spec
ist geschlossen, wenn seine Zusagen abgenommen sind" — ist als Regel richtig und
als Marker-Begründung nicht durchgehalten worden. Nach ihr müsste der Spec der
Runde 1 seit dem 260807 ebenfalls `_o_` tragen, und er tut es nicht.

**Was gegen `_d_` spricht.** Zurückgestellt ist nicht der Spec, sondern der
Abnahmelauf, und der hat im Bestand längst seinen eigenen Ort: die offene Frage
`circles/260802-0842-…/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`.
Ein `_d_` am Spec würde diese Frage ein zweites Mal führen, an der falschen
Stelle.

**Empfehlung: `_c_` mit einer Statuszeile nach dem Vorbild der Runde 1**, die die
Beschränkung benennt. Die Umbenennung selbst gehört dem Nutzer und dem
Orchestrator, weil sie die Zeile `**Active spec/plan:**` im Circle-Datensatz
nachzieht.

**Zwei Stellen des Specs behaupten den Marker `_a_`** (Zeile 10 im Gatehinweis,
Zeile 583) und einer beruft sich auf eine Dreizahl in `CLAUDE.md`, die dort seit
dem 260810 nicht mehr steht. Beides als Abgleichseintrag im Spec vermerkt, nicht
im Text geändert.

## 6. Die vier offenen Defekte und die Lesart des Auftrags

Die Lesart des Auftrags lautet: drei der vier hängen an derselben offenen
Entscheidung `decisions/260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`,
weil alle drei eine Probe mit einer echten AppKit-Instanz brauchen.

**Die Lesart trifft für zwei von vier, nicht für drei.** Jeder Datensatz ist
daraufhin gelesen worden, woran er hängt:

| Defekt | Hängt an | Braucht eine AppKit-Instanz |
|---|---|---|
| `260810-1001` die neuen Proben behaupten den Hauptfaden | **`260810-1044`**, wörtlich: „bleibt offen, bis das Prüfziel steht" | ja |
| `260810-1341` die Freigabe des Blocks ist nicht gemessen | **`260810-1044`** für Weg 1; Weg 2 ist ausdrücklich „eine Probe allein am Block, ohne AppKit" | für Weg 1 |
| `260810-1207` die Spanne zwischen Blatt und Antwort | **KRK im Vordergrund**, also `circles/260802-0842-…/decisions/260806-1303_o_…` | nein |
| `260810-1330` derselbe Prüfordner steht zwölfmal | **nichts** | nein |

**`260810-1207` hängt an einer anderen Sperre.** Sein Abschnitt „Was zuerst zu tun
wäre" sagt: „Messen, nicht bauen … Das verlangt KRK im Vordergrund und ist damit
Nutzerarbeit." Seine `Cross-references:` nennen `260810-1102`, `260810-1029`, C4
und C6 — `260810-1044` steht dort nicht. Es ist dieselbe Klasse wie der
ausstehende Abnahmelauf und nicht dieselbe wie das Bibliotheksziel.

**`260810-1330` hängt an nichts.** Es ist ein Zusammenlegen von zwölf Fassungen
desselben Prüfordners, sechs in `krk-core/tests/`, vier in `krk-ui/src/`, zwei als
`Wegwerfordner` in `krk-bench/`. Nachgezählt, alle zwölf stehen noch. Der
Datensatz nennt selbst den kleinen ersten Schritt: die vier in `krk-ui/src/` auf
ein `#[cfg(test)] mod pruefordner;` derselben Kiste zusammenlegen. **Ein `coder`
könnte das heute tun**, ohne eine einzige offene Frage zu berühren. Es ist der
einzige der vier, von dem das gilt.

**Der Satz, mit dem der Circle schließt, ist also zwei Sätze:** zwei Defekte
warten auf eine Entscheidung, die der Nutzer trifft, einer auf eine Messung am
laufenden Bündel, die der Nutzer fährt, und einer auf gar nichts.

## 7. Die zwei offenen Entscheidungen

**Die Einschätzung des Auftrags trifft. Es sind echte Nutzerfragen, und kein Agent
darf sie beantworten.** Widersprochen wird nicht.

`260810-0959_o_schliesst-c4-die-schreibwerkzeuge-aus.md`. Die Frage ist eine
Lesart der eigenen Zusage: gilt C4 als „kein Zeichen ohne Zutun des Nutzers" oder
als „der gesicherte Stand ist der getippte"? Beide Lesarten sind vertretbar, und
der Datensatz sagt selbst, dass der Unterschied zwischen den sieben
abgeschalteten Automatiken und den Schreibwerkzeugen die Frage stellt und nicht
entscheidet. Das ist keine Frage, die ein Messwert schließt. Vier Einstellungen
sind gemessen und stehen in der Tabelle; die Auslegung bleibt.

`260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.
Der Datensatz stellt seit dem Nachtrag vom 260810-1139 eine andere Frage als bei
seiner Anlage: nicht mehr „wie kommen die vier Proben an ihre Messstücke",
sondern „bekommt `krk-ui` ein Bibliotheksziel". Am Bestand geprüft:
`crates/krk-ui/Cargo.toml` führt allein `[[bin]] name = "krk"`, ein
`crates/krk-ui/src/lib.rs` gibt es nicht. Damit fallen die beiden ursprünglichen
Optionen, wie der Nachtrag es sagt, und die Empfehlung ist auf Option 4
gewechselt, einen Umbau der ganzen Kiste. **Ein Umbau, der jede Datei einer Kiste
berührt, ist keine Agentenentscheidung.**

## 8. Die übrigen Entscheidungsspeicher

Alle vier Speicher durchgesehen: der Circle (13), der gemeinsame (6), die Runde 1
(34), die beiden vorgesehenen Circles (5 und 0).

**Es gibt im ganzen Bestand keinen einzigen Datensatz mit dem Marker `_a_`.** Der
Fund, nach dem der Auftrag gefragt hat — ein `_a_`, das inzwischen `_i_` wäre —
kann es deshalb nicht geben. Was es stattdessen gibt:

**Ein `_i_` ohne auflösbaren Beleg, und er ist nachgetragen.**
`circles/260802-0842-…/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md:73`
trug seit dem 260805 den Platzhalter `Implemented: <Hash offen — trägt der
Orchestrator nach>`. Er war der einzige der 42 `_i_`-Datensätze ohne Beleg.
Ermittelt über `git log -S 'gehalten_von' -- crates/krk-core/src/tasten/belegung.rs`
und `git log -S 'der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege' --
crates/krk-core/tests/belegung.rs`: beide nennen genau einen Commit, `58465bf`.
**Im Datensatz eingesetzt**, mit dem Weg der Ermittlung daneben.

**Ein `_i_`, dessen Antwort hält und dessen Belegzeile überholt ist.**
`decisions/260810-0822_i_wie-die-formatansicht-ihre-auszeichnung-setzt-und-warum-an-zwei-orten.md`
beschreibt in seiner `Implemented:`-Zeile `formatierung_anwenden` als die Stelle,
die über beide Wege setzt. Seit der Behebung von `260810-1245` nimmt eine neue
Funktion `merkmale_zuruecksetzen` (`editor.rs:2815`) zurück, und
`setTemporaryAttributes_forCharacterRange` sitzt allein dort (`:2827`). Der
Schnitt selbst ist unberührt, der Marker bleibt richtig. **Als Abgleichsanmerkung
mit beiden neuen Pfaden angehängt.** `260810-1139` widerspricht dem Datensatz
nicht: es hat eine `SAFETY`-Begründung berichtigt, über die der Datensatz nichts
behauptet.

**`260810-0021_i_was-verwirft-verwerfen-…` ist nicht überholt.** Geprüft, weil
`260810-1102` denselben Weg betrifft: `anwendung.rs:2016` ruft
`bereich_umschalten(Bereich::Vorschau)` unmittelbar, wie der Datensatz es sagt;
`260810-1102` ist als „der Befund hält nicht" geschlossen und hat nur den
Modulkopf von `appkit/ereignisse.rs` geändert.

**Vier `_i_`-Datensätze tragen zwei Fußblöcke**, der erste leer, der zweite mit
dem Beleg: `260802-1134_i_`, `260802-1810_i_`, `260803-1208_i_`, `260803-2025_i_`.
Wer den ersten liest, hält den Datensatz für unbelegt. Nicht angefasst, aus
demselben Grund wie unter Punkt 2: das Doppeln ist hier der Befund, und ihn durch
ein drittes Vorkommen zu heilen wäre die falsche Richtung.

**Die zwölf offenen Entscheidungen sind alle zu Recht offen.** Für jede ist
geprüft, ob auf der Platte inzwischen eine Antwort steht:

- `shared/…_o_git-verwerfen-bedeutung` und `_o_code-sdk-fuer-ki-integration`:
  ungebaut, kein Treffer auf `git2`, `libgit`, `anthropic`, `openai` in `crates/`
  oder `Cargo.toml`. Beide liegen außerhalb dieses Circles.
- `…260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund`: die
  Sitzung hat keine Datei unter `crates/krk-bench/` angefasst.
- `…260806-1730_o_welche-sprache-bestimmt-die-sortierordnung`:
  `verzeichnis/kollation.rs` steht nicht im Sitzungsdiff.
- `…260807-0010_o_kann-der-auffrischungsaufschub-entfallen` und
  `…260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben`: **weder
  `crates/krk-ui/src/auffrischung.rs` noch `crates/krk-core/src/verzeichnis/modell.rs`
  noch `crates/krk-ui/src/tabs.rs` sind in `38a02b2..HEAD` angefasst.** Die
  Sitzung hat den Gegenstand nicht berührt. Die Einschätzung des Circle-Datensatzes,
  dass beide das Dateifenster und nicht den Editor betreffen, hält.
- `…260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2`:
  **teilweise beantwortet, Marker bleibt `_o_`.** Ausführung unten.
- Die fünf im vorgesehenen Circle `260809-2040-tastenbelegung-als-markdown-in-downloads`:
  die Sitzung hat an `resources/default-keymap.toml` nur Kommentartext geändert,
  keine Funktion und keine Taste; `belegungsansicht.rs` und `belegungsmodell.rs`
  stehen nicht im Diff. Der Sachstand, auf dem die fünf aufsetzen, hält: 71
  Funktionen, sechs mit `gehalten_von = "menue"`, also 65 mit Kommando.

### Die `objc2`-Verfügbarkeitsfrage

Sie ist der eine Fall, in dem diese Sitzung eine offene Frage der Runde 1
berührt hat, und die Berührung reicht nicht für `_a_`. Der Datensatz fragt, wie
KRK aus dem **ausgelieferten Code** eine Schnittstelle anspricht, die es erst ab
macOS 26 gibt. Die Sitzung hat einen Fall derselben Art gelöst, aber nur im
**Prüfcode**: `crates/krk-ui/src/appkit/editor.rs:4924` (`merkmal_falls_vorhanden`)
ist der einzige `respondsToSelector`-Aufruf im ganzen Baum und steht in einer
Prüfhilfe. Der ausgelieferte Code setzt keinen Schalter über der Untergrenze.

**Der Marker bleibt zu Recht `_o_`, der Datensatzinhalt war veraltet.** Drei
Belegstellen fehlten ihm, und sie sind als Abgleichsanmerkung nachgetragen: die
halb gebaute Möglichkeit 1 samt ihrer ausgeschriebenen Regel
(`editor.rs:4893-4903`), ein **fünfter Mechanismus**, den die vier Möglichkeiten
des Datensatzes nicht führen (`editor.rs:277-280`: `objc2` bildet keine
Verfügbarkeitsgrenze ab, schaltet die beiden neuen Setzer aber über ein
Cargo-Merkmal), und die Feststellung, dass `objc2` 0.6 an `AnyProtocol` keine
Mitgliederliste führt (`objc2-0.6/src/runtime/mod.rs:1045-1090`).

## 9. Die Durchsichten

**Alle zehn Befunde der beiden Durchsichten dieser Sitzung sind geschlossen.**
Jeder hat seinen eigenen Datensatz bekommen, und jeder trägt heute `_c_`: die
sieben des `coderev` vom 260810-1248 (`260810-1241` bis `-1247`) und die drei des
`ontorev` vom 260810-1217 (`260810-1217` bis `-1219`).

**Über alle sechs Durchsichten des Circles hinweg gibt es keinen offenen Befund.**
Maschinell geprüft: jeder von einer Durchsicht zitierte Defektdatensatz wurde im
Bestand gesucht und sein heutiger Marker gelesen; alle tragen `_c_`. Das gilt auch
für die Durchsichten vom 260808-1413, 260809-1700, 260810-0752 und die
`conceptrev` vom 260807-2202.

Beide Durchsichten dieser Sitzung haben eine Anmerkung des Abgleichs erhalten, mit
Befundnummer, Datensatz und Commit. Die Befundtexte selbst sind nicht angefasst:
eine Durchsicht ist ein Zeitstand.

## 10. Was `CLAUDE.md` jetzt falsch sagt

Geprüft, nicht geändert. Der Abschnitt „Projektstand" trägt „Geprüft am
260810-0714" und beschreibt den Stand vor dieser Sitzung.

**Was noch stimmt, am Code nachgezählt und nicht angenommen** — damit die Liste
darunter nicht als Rundumverdacht gelesen wird:

```text
Kommando               65 Varianten   (behauptet: 65)   belegung.rs
Wirkungsbereich         7 Varianten   (behauptet: 7)    belegung.rs
Bereich                 5 Varianten   (behauptet: 5)    fenstermodell.rs
Fokus                   5 Varianten   (behauptet: 5)    kommandos/fokus.rs
allow(unsafe_code)      2 Dateien     (behauptet: 2)    sys.rs, appkit/mod.rs
```

Ebenso vorhanden und unter dem genannten Namen: `bereich_des_kommandos`
(`belegungsmodell.rs:166`), `schiebt_auffrischung_auf` (`auffrischung.rs:265`),
`lesevorgang_beginnen` (`verzeichnis/modell.rs:151`), `auswahl_auf_namen`
(`tabs.rs:552`), `ersthelfer_gehoert_appkit` (`appkit/ereignisse.rs:516`). Die
Zahl 110 der Abnahmekriterien und ihre Aufteilung in 108 plus zwei stimmt.

**Was falsch oder unvollständig ist, in absteigender Schärfe:**

1. **Die Prüfdatumzeile.** „Geprüft am 260810-0714" liegt vor 17 Commits, die
   Code an neunzehn Dateien geändert haben.

2. **Der Satz über den Prüfordner ist unvollständig, und es gibt jetzt einen
   Defekt dazu.** Der Abschnitt „Was man nicht sieht" verweist auf `Pruefordner`
   in `krk-core/tests/verzeichnis.rs` als **die** Form des Projekts. Dieselbe Form
   steht zwölfmal getrennt im Baum, unter zwei Namen, und der Modulkopf jener
   Datei sagt in Zeilen 3 bis 5 weiterhin, ein Erzeuger sei „bewusst noch nicht"
   da und komme mit Schritt 3 — ein Satz, der mit dem Abschluss der Runde 2
   überholt ist. Der Defekt ist `260810-1330_o_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.

3. **Eine Eigenschaft fehlt der Liste der sieben, und sie hat schon einen
   Fehlbefund erzeugt.** Ein stehendes Blatt wird **nicht** vom Fokusvorbehalt in
   `appkit/ereignisse.rs` angehalten, sondern beim Anwendungsdelegierten:
   `kommando_ausfuehren` weist jedes Kommando außer dem Abbruch ab, solange ein
   Blatt am Fenster hängt (`kommandos::operationen::waehrend_blatt_erlaubt`), und
   `eingabe_ausfuehren` hält daneben das getippte Zeichen an. Zwei Stellen, zwei
   verschiedene Fragen. Wer nur `ereignisse.rs` liest, hält den Vorbehalt für die
   einzige Sperre — genau so ist der Fehlbefund `260810-1102` entstanden, und der
   Modulkopf trägt die Warnung seit `8807844`.

4. **`krk-ui` hat kein Bibliotheksziel, und das ist tragend geworden.** Die Kiste
   führt allein `[[bin]] name = "krk"`; ein `src/lib.rs` gibt es nicht. Eine
   Prüflaufdatei unter `tests/` ist deshalb eine eigene Kiste und erreicht nichts
   aus `krk-ui`, ob `pub` oder nicht (`error[E0433]: cannot find module or crate
   krk_ui`). Daran hängen zwei offene Defekte und die offene Frage `260810-1044`.
   `crates/krk-ui/tests/syntaxkiste.rs` läuft nur, weil es allein `syntect` und
   `two-face` anspricht.

5. **Vier Proben behaupten den Hauptfaden, den `libtest` ihnen nicht gibt.**
   `unsafe { MainThreadMarker::new_unchecked() }` steht in
   `appkit/editor.rs` an genau einer Stelle (`an_einer_flaeche`) und trägt vier
   Proben, die eine `NSTextView` bauen. Gemessen: `cargo test` liefert `None`,
   auch mit `--test-threads=1`; nur ein `[[test]]`-Ziel mit `harness = false`
   liefert `Some`. Der Datensatz ist `260810-1001`.

6. **Die Typprüfung vor dem Öffnen steht am Deskriptor und nicht am Pfad.**
   `krk-core/src/text/datei.rs` öffnet über
   `crate::verzeichnis::sys::ohne_warten_oeffnen` mit `O_NONBLOCK`, fragt danach
   `metadata()` am Deskriptor und schaltet `O_NONBLOCK` über `F_GETFL`/`F_SETFL`
   wieder ab. `krk-core` führt kein `libc`; die drei Konstanten und die
   variadische `fcntl`-Deklaration stehen in `verzeichnis/sys.rs`. Seit dieser
   Sitzung gilt derselbe Weg im Vorschauweg (`vorschaumodell.rs`,
   `bis_zur_grenze_lesen`). Das ist die Sorte Eigenschaft, die der Abschnitt
   „Was man nicht sieht" sammelt: `lstat(2)` und `fstat(2)` beantworten
   verschiedene Fragen, und wer die Prüfung an den Pfad zurückzieht, blockiert an
   einer Röhre.

7. **Der Rückgängigstapel des Editors ist seit `0140df7` gedeckelt.**
   `STAPELBUDGET` steht auf `krk_core::text::datei::EDITORGRENZE`, `Stapellast`
   zählt die gehaltenen Bytes und trägt in `Drop` ab. Der Verlauf ist damit
   „Budget plus eine Handlung" und nicht unbegrenzt. Ob die Freigabe des
   angemeldeten Blocks wirklich abträgt, ist geschlossen und nicht gemessen
   (`260810-1341`).

8. **Die Syntaxhervorhebung rechnet den vorigen Durchgang fort** (`3596e16`) statt
   ihn zu wiederholen, und `hervorhebung.rs` trägt dafür einen `Zerlegerstand` mit
   Haltepunkten. Die Zusage „von vorn gleicht fortgeschrieben" ist an 18 000
   Läufen gemessen; die Aufzählung im Abschnitt „Projektstand" nennt das Modul,
   aber nicht diese Eigenschaft.

9. **`Cargo.toml` trägt jetzt die gezählte Folge der zwei fremden Kisten.** Der
   Satz in `CLAUDE.md`, beide seien „ohne ihre Vorgabemerkmale eingebunden", bleibt
   richtig; nicht erwähnt ist, dass sie 21 weitere Pakete mitziehen, `Cargo.lock`
   von 72 auf 95 Einträge wächst, davon 20 auf dem Bauziel ankommen, und dass
   keines ein `-sys`-Paket ist oder `cc` mitbringt. Der größte Zuwachs am
   Abhängigkeitsbaum, den das Projekt bisher aufgenommen hat.

10. **Die Zahl der offenen Defekte, die die Zeile mit dem `find` einsammelt,
    ändert sich.** Nach diesem Abgleich: 5 im Circle der Runde 2 (die vier des
    Nutzers und der neu angelegte), 2 im gemeinsamen Speicher, 5 im Circle der
    Runde 1, 0 in den zwei vorgesehenen. Die Zeile selbst bleibt richtig — sie
    nennt keine Zahl und erklärt den Dateibestand für verbindlich.

**Ein Kandidat für die Streichung.** Der Satz „Etliche Fallunterscheidungen sind
vollständig und haben keinen Auffangzweig" führt zwei Stellen namentlich, die
jedes neue Kommando braucht. Beide stimmen. Die Aufzählung darunter ist aber seit
dem 260810 ausdrücklich ohne Zahl, und der Abschnitt „Vier Aufzählungen sind in
der Runde 2 gewachsen" nennt daneben vier Zahlen für dieselbe Sache. Zwei
Abschnitte über eine Sache, einer mit Zahlen und einer ohne.

## Was dieser Abgleich nicht geprüft hat

- **Nichts am laufenden Bündel.** Keine der 110 Abnahmekriterien des Specs, keine
  der drei Anzeigen, kein Tastendruck. Das verlangt KRK im Vordergrund.
- **Keine Laufzeitzahl eines Datensatzes nachgemessen.** Siehe Punkt 1.
- **Die Zeilenangaben in Dateien, die diese Sitzung nicht angefasst hat**, sind
  maschinell gegen die Dateilänge geprüft, aber nicht inhaltlich nachgelesen. Dort
  können weitere Abwanderungen stehen, die älter sind.
- **Der Circle-Datensatz und `agentstate.yaml` sind gelesen und nicht
  geändert.** Zwei Stellen des Circle-Datensatzes sind veraltet und gehören dem
  Orchestrator: `**Active session history:**` nennt noch
  `history/260810-0244-orchestrator-session.md` statt der Sitzung 260810-0845, und
  der Abschnitt `## Turn log` endet bei Turn 3 jener Sitzung, führt also die sechs
  Turns dieser Sitzung nicht. Ebenso die Zeile `**Directive:**` in
  `history/260810-0845-orchestrator-session.md`, die „(noch nicht gesetzt)" trägt,
  obwohl `agentstate.yaml` die Directive führt.
