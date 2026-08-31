# Abgleich zum Abschluss der Runde 23

**Filed by:** reconciler, Kai Stalmann <kai@qantr.com>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Domäne:** `code`
**Stand vor der Runde:** `d1fbaac` — **Stand beim Abgleich:** `2976520`, 24 Commits dazwischen
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`
**Spec:** `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md`

---

## Verification

```
cargo build --workspace                              — exit 0
cargo test --workspace                               — exit 0, 24 Ziele, kein roter Lauf
cargo clippy --workspace --all-targets -- -D warnings — exit 0
cargo fmt --all --check                              — exit 0
cargo tree --target {x86_64,aarch64}-apple-darwin -e normal,build — je 197 Pakete, null Treffer auf `cc` und `-sys`
```

---

## Was geprüft ist

| Gegenstand | gelesen | berührt |
|---|---|---|
| Pläne (Circle und gemeinsam) | 2 | 2 |
| Defekte (Circle und gemeinsam) | 14 im Circle, der gemeinsame Speicher im Umfang dieser Runde | 2 |
| Entscheidungen (Circle und gemeinsam) | 5 im Circle, 31 im gemeinsamen | 5 |
| Durchsichten | 0 im Circle, 6 im gemeinsamen (alle vor der Runde) | 0 |
| History-Einträge des Circles | 20 | 0 |

Der Umfang ist nicht abgekürzt: `bin/fusion-cadence-anchor changed-files last_reconcile_commit` nennt den ganzen Circle, also ist jede Datei gelesen worden, die er führt, und dazu die Entscheidungsspeicher beider Seiten.

---

## 1. Der Plan gegen den Baum

**Sechzehn der siebzehn Schritte stehen auf `[DONE]`, und jede der sechzehn Erledigungen ist einzeln gegen den Baum gelesen. Keine ist unbelegt, und keine ist überzeichnet.** Die Belegtabelle je Schritt steht im `## Reconciliation Log` des Plans und wird hier nicht wiederholt.

Der siebzehnte Schritt ist der Abnahmelauf am laufenden Bündel. Er ist nicht gefahren, kann von keinem Agenten gefahren werden, und die Runde schließt deshalb beschränkt (`_b_`). Das ist die Bauart dieses Projekts und kein Fehlschlag: die Wirkungsbereichs-Prüfung weist aus dem Hintergrund jeden fokusgebundenen Befehl ab (`260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`, offen seit der Runde 1).

**Drei Nachprüfungen, die über das Lesen hinausgehen:**

- `make check` ist am Abgleichsstand nachgefahren und grün, alle vier Kommandos. Die Behauptung von Schritt 16 hält.
- Die `#[must_use]`-Zählung von Schritt 16 (139 vorher, 169 danach) ist an beiden Ständen nachgezählt und stimmt. Bei der ersten Zählung hatte ich `\s` in einem `grep -E` benutzt, das POSIX-ERE nicht kennt, und bekam 90 statt 139; das war mein Fehler und nicht der des Schritts.
- Die Erhebungsvorschrift für die C-Freiheits-Zusage aus `CLAUDE.md:89` ist nachgefahren und findet sieben Stellen, jede mit der Wendung „Namen auf `-sys`", keine mit einer Zahl.

---

## 2. Die 90 Abnahmekriterien

Die `Kriterien:`-Zeilen der Schritte 1 bis 16 nennen 88 der 90, die Zeile von Schritt 17 nennt 25. **Kein Kriterium steht ohne Zuordnung.** Der Schnitt:

- **65 sind belegt.** Sie tragen allein eine Stelle im Baum oder eine Probe und kommen in Schritt 17 nicht vor. Darunter alle Kriterien von C8, C9 und C10, die Aufzählungs- und Tafelkriterien von C1, C2 und C5, die drei Sonderzustände C3.6, C3.7 und C3.10 und die Nebenläufigkeitskriterien von C7 außer C7.2 und C7.3.
- **23 sind geteilt.** Ihre Bau- oder Probenhälfte ist belegt, ihre Anzeigehälfte liegt bei Schritt 17: C1.2, C1.3, C1.6, C1.10, C2.2, C2.4, C2.7, C2.8, C3.1, C3.3, C3.4, C3.5, C3.9, C4.2, C4.4, C5.5, C5.11, C6.1, C6.2, C6.3, C6.6, C6.7, C7.3.
- **2 liegen ganz beim Nutzer** und haben keine Agentenhälfte: **C5.4** (zwei Ordner aus zwei verschiedenen Repositorys nebeneinander zeigen zwei verschiedene Markensätze) und **C7.2** (die erste Bildschirmseite eines Ordners steht nach der Runde nicht später da als vor ihr).

**Ohne Beleg bleiben also 25, und alle 25 sind Nutzerarbeit und keine Lücke.** Die Zahl deckt sich mit der, die der Spec selbst in `## Zur Zählung der Abnahmekriterien` nennt; dass jene Liste sich in zwei Sätzen über C3.1 und C3.3 widerspricht, ist gefilt (`260830-1317_*_die-25er-liste-der-nutzerarbeit-…`) und ändert an der Zahl nichts.

Die Kästchen des Specs sind **nicht** angehakt. Ein Kriterium, dessen Anzeigehälfte niemand gesehen hat, ist nicht abgenommen.

---

## 3. Die Marker

**Bewegt: drei, alle im gemeinsamen Entscheidungsspeicher, alle `_a_` → `_i_`.** Jeder trug seine Antwort in der Zeile `Answered:`, und der Baum trägt sie inzwischen:

| Datensatz | Beleg |
|---|---|
| `260830-1006_*_bekommt-der-git-bereich-einen-sechsten-fokuswert-oder-ist-er-nicht-fokussierbar.md` | `c99d433` — `Fokus::Git`, `Fokus::ALLE` mit sechs Einträgen, die vier stillen Stellen von Hand nachgezogen |
| `260830-1006_*_wohnt-die-git-anbindung-in-krk-core-oder-in-einer-fuenften-kiste-krk-git.md` | `1d84f2b` — `crates/krk-core/src/git/`, `gix` in `krk-core/Cargo.toml:46`, Workspace weiter vier Mitglieder |
| `260830-1006_*_was-zeigen-git-bereich-ankreuzfeld-und-dateiliste-in-einem-ordner-ohne-repository.md` | `7264daf` — `KEIN_REPOSITORY` (`git/texte.rs:29`), kein Gitrufer von `sichtbar_setzen`, Markenspalte steht und bleibt leer |

Bei den ersten und dritten steht in der `Implemented:`-Zeile ausdrücklich, dass die Anzeigehälfte aus Schritt 17 noch nicht abgenommen ist: `_i_` sagt, dass der Baum die Antwort trägt, und nicht, dass jemand sie gesehen hat.

**Nicht bewegt, und das ist Absicht: die drei `_o_` im Circle.**

- `260830-1251_*_haengt-der-gitbefund-zusaetzlich-an-einem-beobachter-auf-git.md` — unbeantwortet, keine Vorbelegung gebaut. Bleibt `_o_`.
- `260830-1317_*_bekommt-der-git-bereich-einen-eigenen-funktionsbereich-und-damit-ein-zehntes-obermenue.md` — die Vorbelegung des Plans **ist gebaut** (`Funktionsbereich::Git`, `belegungsmodell.rs:101`; zehntes Obermenü an achter Stelle in `make menue`).
- `260830-1317_*_wird-die-fadenzahl-von-gix-gedeckelt-und-woran-waere-die-zahl-zu-messen.md` — die Vorbelegung ist gebaut (`thread_limit` wird nirgends gesetzt).

Bei den letzten zwei wäre `_a_` oder `_i_` nach dem Buchstaben der Markerregel vertretbar und der Sache nach falsch: **gebaut ist nicht beantwortet.** Der Plan schreibt unter `## Where this Circle stops` ausdrücklich aus, dass beide auf `_o_` stehen bleiben, sofern der Nutzer sie nicht beantwortet, und ein Marker jenseits von `_o_` nähme sie aus der Suche nach aktiver Grundlage heraus — dieselbe Falle, die `CLAUDE.md` am zurückgestellten L9-Datensatz beschreibt. Beide haben stattdessen einen Abgleichsbeleg bekommen, der sagt, was gebaut ist und warum der Marker trotzdem steht.

**Geprüft und richtig befunden: die zwei `_i_` im Circle** (`260830-1612_*_darf-eine-probe-git-rufen-…` mit `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` in `xtask/src/release.rs`, grün im Probenlauf; `260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-…` mit `Gitfenster::auswahlmelder_setzen` und `Gitmodell::auswahl`), die zwei `_i_` im gemeinsamen Speicher aus dieser Runde und die fünf `_c_`-Defekte.

**Geprüft und richtig befunden: die neun offenen Defekte des Circles.** Stichprobe an dreien gegen den Baum: `resources/default-keymap.toml:464` und `:478` sagen weiter „wie die drei Spaltenschalter darueber", und darüber stehen vier — der Defekt steht zu Recht offen und gehört dem `ontocoder`. `crates/krk-ui/src/kommandos/kontextmenue.rs:204` behauptet weiter, eine Feldbreite halte den Bau an (gemeinsamer Speicher, `260831-1212_*`). `EntryStatus::NeedsUpdate(_) => return None` steht unverändert in `git/leser.rs:398` und ist nach dem Befund von Schritt 10 unerreichbar.

**Keiner der neun hält den Abschluss auf.** Zwei bestanden vor der Runde, sieben sind Papierbefunde des Nachziehens.

---

## 4. Die Endbedingungen aus `## Where this Circle stops`

Die Tabelle steht im `## Reconciliation Log` des Plans. Zusammengefasst: **von sechzehn Bedingungen stehen fünfzehn, und die eine, die nicht steht, ist genau die, deren Nichterfüllung den beschränkten Abschluss ausmacht** — „alle siebzehn Schritte stehen auf `[DONE]`". Sechzehn stehen.

Die zwei Bedingungen, die die Runde anhalten konnten, sind beide geprüft und beide bereits vom Nutzer erledigt: die C-Freiheit auf beiden Mac-Zielen greift nicht (nachgefahren, je 197 Pakete, null Treffer), und der gemessene Index-Posten hat gegriffen und ist am 260831 entschieden (`1888ef0`, Datensatz auf `_i_`).

Die übrigen dreizehn sind einzeln nachgefahren, nicht abgelesen: `write_changes` ohne Aufrufstelle, `NeedsUpdate` mit genau einer Lesestelle, die zehn Zeitzusagen unverändert, `Schluessel` bei vier Werten, jede angefasste `appkit/`-Datei mit ihrem Untergrenzen-Abschnitt und den zwei begründeten Ausnahmen, `genau_drei_pruefordner_fassungen_stehen_im_baum` grün, keine Auslieferung vor dem Abschluss (`version = "1.4.0"`, HEAD ohne Tag).

---

## 5. Die Abweichungen

**Vier gefunden, drei in diesem Lauf berichtigt.**

1. **Plan und Spec trugen `**Status:** Draft`,** während sechzehn Schritte auf `[DONE]` standen und der Dateimarker `_p_` lautete. Beide auf `Partially Complete` gesetzt. Der Dateimarker bleibt `_p_`: `_c_` wäre erst nach Schritt 17 richtig.
2. **Drei Entscheidungsdatensätze standen auf `_a_`,** während der Baum ihre Antwort trägt. Auf `_i_` gezogen, siehe Abschnitt 3. Das ist derselbe Befund, der in dieser Runde schon einmal von Hand nachgezogen wurde.
3. **Der Spec-Absatz zu den zehn Zeitzusagen ruht auf zwei Schalterständen, und einer davon steht.** Der Defekt war vom `planner` schon gefilt (`260830-1317_*_der-spec-schuetzt-die-messstrecke-mit-einem-schalterstand-den-a13-auf-ein-stellt.md`); dieser Abgleich hat ihn am gebauten Baum belegt und den Beleg angehängt: `gitbedarf_nachziehen` (`appkit/anwendung.rs:4642`) rechnet `sichtbar(Bereich::Git) || spalte_sichtbar(Spalte::Marke)`, die Markenspalte steht ab Werk (`Spaltensichtbarkeit::default`, `sitzung.rs:344`), und `messmodus::tests::pruefsitzung` erbt den Stand über `..Sitzung::default()`. Auch die erste Hälfte der Spec-Begründung trägt nicht: `messmodus.rs` hält nach seinem eigenen Modulkopf nur, „was kein AppKit beruehrt", während `Aufgabe::Spannen` und `Aufgabe::Sitzung` in der laufenden Anwendung messen. **Was die Aussage wirklich trägt, ist der Ort des Messplatzes** — `~/Library/Caches/krk-messplatz` liegt in keinem Repository, geprüft in Schritt 16. Der Defekt bleibt offen; die Schlussfolgerung des Specs stimmt, ihre Begründung nicht.
4. **Ich habe beim ersten Anlauf ein Duplikat gefilt** und wieder gelöscht: der Befund unter 3 war schon da. Das gehört hierher, weil ein zweiter Datensatz zu derselben Sache die Buchführung genau so beschädigt wie ein fehlender.

**Keine einzige Abweichung liegt am Baum.** Alle vier sind Buchführung.

---

## 6. Ein neuer Defekt

`260831-1417_*_die-runde-23-schliesst-ohne-durchsicht-und-vierundzwanzig-commits-sind-ungedeckt.md`

Weder `coderev` noch `ontorev` ist in dieser Runde gelaufen. `bin/fusion-review-coverage` meldet am Abgleichsstand `commits=24 reviews=0 uncovered=24 verdict=uncovered`. Der Plan verlangt keine Durchsicht, und `CLAUDE.md` bindet sie an eine Auslieferung, die nicht gefahren ist — nach dem Buchstaben fehlt nichts. Die Runde 21 hat ihre Durchsicht ohne Auslieferungsanlass gefahren, und die Runde 23 ist die umfangreichste seither: eine fremde Kiste mit 197 Paketen, ein sechster Bereich, ein sechster Fokuswert, eine fünfte Spalte, ein Arbeitsfaden mit Kanal, ein zweiter Befundvektor im Ordnermodell.

**Der Defekt hält den Abschluss nicht auf**, und die Abnahme lässt ausdrücklich zwei Wege offen: eine Durchsicht über `d1fbaac..HEAD`, oder eine Entscheidung des Nutzers, dass diese Runde ohne eine schließt.

---

## 7. Misfiled — should be a decision

Keiner. Die neun offenen Defekte des Circles sind sämtlich Aussagen über einen Baum- oder Textstand, die sich durch einen Diff prüfen lassen; keiner von ihnen ist eine Wahl zwischen Möglichkeiten, die aufzuschreiben wäre.
