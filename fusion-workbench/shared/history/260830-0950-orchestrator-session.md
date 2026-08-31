# Orchestrator-Sitzung — 260830-0950

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Directive:** KRK bekommt eine Git-Anbindung, in Stufen. Stufe A zuerst: ein Git-Bereich am rechten Rand als sechster Wert von `Bereich`, der den Status des angezeigten Ordners, den aktuellen Branch und den Verlauf anzeigt; die Statusmarken in der Dateiliste sind über ein Ankreuzfeld der Bereichsleiste zuschaltbar. Danach Stufe B, die vier Operationen der Runde 1 (hinzufügen, committen, Änderungen verwerfen, Versions-Schieberegler), danach eine Auslieferung. Stufe C (Branches wechseln) ist möglich, Stufe E (PRs und Review-Freigabe) unwahrscheinlich. Bibliothek: `gix` (gitoxide) wird probiert. Dass die zehn Zeitzusagen aus C8 dabei fallen können, hat der Nutzer ausdrücklich in Kauf genommen.
**Mode:** custom — erst Machbarkeitsanalyse, dann Shaper
**Status:** In Arbeit — die Bounded Closure vom 260831-1430 ist zurückgenommen, nachdem die Durchsicht zwei ernste Defekte fand

## Ausgangslage

Am Sitzungsbeginn erhoben:

| | |
|---|---|
| Domäne | `code` (163 Quelldateien, 12 Datendateien, `git-ls-files`) |
| Turn-Budget | 12 |
| Offene Defekte | 197 (`shared/issues`, alle `_o_`) |
| Offene Pläne | 6 |
| Offene Entscheidungen | 20 (`shared/decisions`) |
| Circles | 12 beschränkt (`_b_`), 9 kohärent (`_c_`), 2 zurückgestellt (`_d_`); keiner aktiv, keiner vorgesehen |
| HEAD zu Beginn | `d1fbaac` |
| Unterbrochene Sitzung | keine |
| Circle-Hinweis | nicht ausgegeben (keine vorgesehenen oder aktiven Circles) |

Setup-Nachträge: Der Setup-Marker steht auf Plugin-Version 10.20.0, der Monitor ist neu
kopiert, die vier Stilprofile stimmen mit der ausgelieferten Fassung überein. In
`.gitignore` war `fusion-workbench/.fusion-setup` ausgeschlossen; die Negation ist
angehängt. `portfolio.md` und `.cadence-anchors` sind eingecheckt, obwohl die
Vier-Klassen-Partition sie als Laufzeitstand führt — gemeldet, nicht repariert. In
`.guard-state/` liegen drei Dateien aus abgeschalteten Mechanismen (`escalation.json`,
`churn.json`, `state-drift.json`), ohne gesetzte Haltemarke; die Löschfrage steht beim
Nutzer und ist noch nicht beantwortet.

## Vorlauf: die Chat-Diskussion vor der Direktive

Der Nutzer hat einen Vorschlag für die Git-Erweiterung erbeten. Die Erhebung im Baum und
in der Werkbank hat drei Befunde geliefert, die den Zuschnitt bestimmt haben.

Git ist in KRK nicht gebaut. Die Aufzählung `Kommando` trägt 82 Varianten, und keine
davon betrifft Git. Die Directive der Runde 1 nennt Git, grenzt es aber auf vier
Operationen ein, und die offene Entscheidung
`260802-0842_*_git-verwerfen-bedeutung.md` hält in ihren Constraints ausdrücklich fest,
dass Branches, Merges, Remotes, Push und Pull draußen bleiben. Der Wunsch des Nutzers
nach Status, Branches, Verlauf, Vergleich und Freigabe liegt damit vollständig außerhalb
der bestehenden Directive; er verlangt deren Erweiterung und nicht ihre Umsetzung.

Der vom Nutzer vorgeschlagene Modus-Umschalter trifft ein Muster, das im Baum schon
zweimal steht. Die Fensterzeile ist eine `NSSplitView` mit fünf Bereichen, von denen
sich Vorschau und Editor denselben Platz am rechten Rand teilen und nie zugleich
sichtbar sind. Ein dritter Bewohner dieses Platzes ist keine neue Mechanik. Dazu kommt
ein Nebeneffekt, der Arbeit spart: die Bereichsleiste am Fensterfuß reiht ihre fünf
Bereichsschalter über `Bereich::ALLE`, also erzeugt ein sechster Wert der Aufzählung
seinen Schalter von selbst.

Widersprochen habe ich dem Vorschlag, die Branches in die Lesezeichenleiste zu
hängen. Mechanisch wäre es billig, denn die Leiste ist eine einzige Liste mit
Überschriften. Semantisch bricht es: beide bestehenden Abschnitte öffnen einen Ort,
einen Ordner oder eine Textstelle, während ein Branch ein Zustand ist, dessen Wechsel
den ganzen Baum betrifft. Dieselbe Zeilenoptik für zwei so verschiedene Wirkungen ist
die Falle, und die Branches gehören in den Git-Bereich, wo ihr Kontext steht.

## Entscheidungen des Nutzers in dieser Sitzung

Vier Antworten, alle im Chat gegeben und hier festgehalten, bis sie als
Entscheidungsdatensätze stehen:

1. Reihenfolge A, B, Auslieferung, danach möglicherweise C. Stufe E gilt als
   unwahrscheinlich.
2. Die zehn Zeitzusagen aus C8 dürfen fallen.
3. Als Bibliothek wird `gix` (gitoxide) probiert.
4. Die Statusmarken in der Dateiliste werden zuschaltbar, über ein weiteres
   Ankreuzfeld der Bereichsleiste.

## Verlauf

- Analyst zur Machbarkeit von `gix` für Stufe A dispatcht.

## Zwei Entscheidungen vor dem Plan, vom Nutzer am 260830 beantwortet

Der Shaper hatte sie ausdrücklich nicht in die Klärung der Direktive genommen, weil
beide nicht berühren, was der Nutzer am Programm sieht. Vorgelegt hat sie der
Orchestrator, der Nutzer hat mit `1a 2a` geantwortet.

**Die Git-Anbindung wohnt in `krk-core/src/git/`, mit `gix` als Abhängigkeit des
Kerns.** Das ist dieselbe Einordnung, die `icu_collator`, `regex` und `zip` bekommen
haben, und sie folgt der Regel, nach der dieses Projekt seine Kisten schneidet: alles
ohne Fensterwerkzeug gehört in den Kern und wird dort geprüft. Ein Gitleser liefert
Namen, Marken, Hashes und Zeitpunkte, also keine Darstellung. Der Preis ist benannt und
angenommen: `cargo test -p krk-core` übersetzt 98 zusätzliche Pakete mit, und `libc`
kommt in den Teilbaum des Kerns. Eine fünfte Kiste `krk-git` ist damit abgelehnt.

**Die C-Freiheits-Zusage bezieht sich künftig auf das Bauziel und nicht auf den Inhalt
von `Cargo.lock`.** Sie lautet fortan, dass auf den beiden Mac-Zielen weder `cc` noch ein
`-sys`-Paket im Abhängigkeitsbaum ankommt, während `Cargo.lock` daneben `windows-sys` und
`linux-raw-sys` führt, beide an fremden Zielen. Das Prüfmittel wird
`cargo tree --target <ziel> -e normal,build` statt eines `grep` in `Cargo.lock`, und
`windows-sys` verliert seinen Ausnahmestatus: es ist der erste Fall der Regel statt der
Ausnahme von ihr. Fünf Prosastellen sind nachzuziehen; das gehört in den Plan der Runde.

## Der Blocker in Schritt 3 und die Antwort des Nutzers

Schritt 3 hat `gix` aufgenommen und ist an einer Probe der Runde 8 hängengeblieben.
`xtask_ruft_git_an_genau_einer_stelle` (`xtask/src/release.rs`) liest jede `.rs`-Datei
unter der Projektwurzel und verlangt genau einen `git`-Aufruf; die zwei Prüfrepositorys
in `crates/krk-core/tests/git.rs` bringen zwei weitere. Unvermeidlich, weil die Stufe A
nicht schreibt und `gix` deshalb kein Repository anlegen kann.

**Der Nutzer hat entschieden: die Prüfung klammert `crates/*/tests/` aus.** Die Zusage
bleibt damit eine Zahl statt einer Liste und sagt danach, was sie meint: ein Eingang zu
`git` im Programm und im Bauwerkzeug. Zwei Möglichkeiten sind ausdrücklich verworfen,
die Ausnahmeliste nach dem Muster der Prüfordner-Zählprobe und die Beschränkung der
Prüfung auf `xtask/`.

**Der Preis ist benannt und angenommen.** Ein zweiter `git`-Rufer unter `crates/*/tests/`
fällt danach nie mehr auf. Und die Grenze trifft nicht, was sie zu treffen vorgibt:
`krk-ui` führt kein Bibliotheksziel und prüft deshalb in `#[cfg(test)]`-Modulen unter
`src/`, die ebensowenig ausgeliefert werden und weiter gezählt bleiben. Ein späterer
Test dort, der `git` ruft, macht die Probe wieder rot.

## Wo die Auswahl der Verlaufsliste wohnt

Schritt 7 hat sie in die Ivars des `Gitfenster` gelegt und die Frage gefilt, weil die
Signaturen aus Entscheidung 5 dem Bereich keinen Schreibweg ins Gitmodell geben. Der
sichtbare Unterschied ist einer: ob eine Auswahl im Verlauf einen Tabwechsel übersteht.

**Der Nutzer hat Möglichkeit 2 gewählt: die Auswahl zieht in das `Gitmodell`, und der
Git-Bereich meldet sie nach oben.** Damit überlebt sie den Tabwechsel und den Wechsel des
aktiven Dateifensters, wie es das Halteverhalten der Tabs in KRK überall sonst tut und wie
das Gitmodell es für Kopf und Verlauf schon zusagt. Die drei Leser `auswahl`,
`auswahl_setzen` und `ausgewaehlter_commit` bekommen damit ihren Rufer, und die
Ablaufmarke in `gitmodell.rs` fällt.

Der Preis ist benannt und angenommen: ein zweiter Melder neben dem Nachlademelder, den
Entscheidung 5 nicht vorsieht, und ein schreibender Zugang des Anwendungsdelegierten zum
Gitmodell des sichtbaren Tabs, also eine Ausnahme von der Zusage „nur zu lesen" an
`Tabinhalt::gitmodell`. Möglichkeit 3 ist verworfen, weil sie zwei Schreiber auf dasselbe
Feld setzte: die Ansicht und den Einzugstakt.

**Nichts davon berührt das Repository.** Geschrieben wird eine Zahl in ein Feld im
Arbeitsspeicher. Bedingung 2 der Runde steht unverändert: kein Weg dieser Runde ruft eine
schreibende `gix`-Funktion.

## Der gemessene Posten und die Antwort des Nutzers

Schritt 10 hat den Posten beziffert, den die Machbarkeitsanalyse ausdrücklich ungemessen
gelassen hatte. Er kostet das 1,7- bis 9,5-fache der Statusabfrage selbst und fällt bei
jedem Ordnerwechsel erneut an: 56 gegen 12 Millisekunden im KRK-Klon, 36 gegen 21 bei
zehntausend Einträgen, 1369 gegen 147 bei hunderttausend. Damit griff die zweite
Endbedingung der Runde, und die drei Möglichkeiten lagen dem Nutzer erneut vor.

**Der Nutzer hat Möglichkeit 1 gewählt: nicht zurückschreiben, Stufe A bleibt
schreibfrei.** Der Preis ist jetzt beziffert statt unbekannt und in dieser Form
angenommen. Kein Schreibweg, keine Sperrdatei, kein Konflikt mit einem gleichzeitig
laufenden `git` im Terminal, und kein Fall, in dem ein Repository eines anderen Benutzers
eine Fehlerbehandlung verlangte, die es nicht gibt.

Ausschlaggebend waren die zwei Fälle, die ein Schreibweg mitbrächte. Der seltene ist das
fremde Repository, dessen Dateirechte den Schreibversuch abweisen; der häufige ist die
Sperrdatei `.git/index.lock`, die ein `git commit` im Terminal scheitern ließe, und zwar
gerade in dem Repository, das der Nutzer in KRK offen hat.

**Die Antwort ist unmittelbar realisiert**, weil der Baum sie schon trägt: `write_changes(`
hat null Aufrufstellen unter `crates/`, nachgezählt am 260831.

## Nachtrag 260831-1321: die fünf Prosastellen sind sieben

Eingetragen in Schritt 15 der Runde 23 (`analyst`, Kriterium C9.3). Der Abschnitt über die
zwei vor dem Plan beantworteten Entscheidungen sagt oben: „Fünf Prosastellen sind
nachzuziehen; das gehört in den Plan der Runde." **Die Zahl ist falsch. Sie bleibt als Stand
vom 260830 stehen und wird hier berichtigt und nicht überschrieben.**

Im Baum standen am 260830 sechs Stellen, und seit der Aufnahme von `gix` in Schritt 3 sind
es sieben. An die Stelle der Zahl tritt eine Erhebungsvorschrift, die seit Schritt 13 an
einer Stelle steht, in `CLAUDE.md` beim Absatz zur Zusage:

```sh
grep -rn --exclude-dir=fusion-workbench --include='*.md' --include='*.toml' --include='*.rs' 'Namen auf `-sys`' .
```

Ihre sieben Treffer, am 260831-1321 gegen den Stand `9566973` nachgefahren: `Cargo.toml:93`
(Begründung zu `regex`), `:153` (`zip`), `:279` (`objc2-pdf-kit`), `:361` (`syntect` und
`two-face`), `:515` (`gix`), `CLAUDE.md:89` und
`crates/krk-core/src/verzeichnis/sys.rs:75`.

**Der Befund ist nicht die Zahl, sondern der Grund, aus dem sie danebenlag.** Die Erhebung,
die dieser Sitzung zugrunde lag, suchte nach dem Wortlaut der alten Zusage, und genau die
sechste Stelle führte die Zusage ohne diesen Wortlaut: `crates/krk-core/src/verzeichnis/sys.rs`
sprach vom „ersten `-sys`-Paket neben `windows-sys`" und blieb deshalb für jede Suche nach
der zitierten Form unsichtbar. Die neue Vorschrift ist nur deshalb vollständig, weil jede
der sieben Stellen die Wendung „Namen auf `-sys`" führt. Wer eine achte schreibt, schreibt
die Wendung mit; sonst läuft dieselbe Erhebung ein zweites Mal an ihr vorbei.

Belege: `260830-1106_*_der-entscheid-zur-c-freiheits-zusage-nennt-fuenf-prosastellen-im-baum-stehen-sechs.md`,
`260831-1258-coder-die-c-freiheits-zusage-an-ihren-sechs-stellen.md`,
`260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`
(derselbe Nachtrag).

---

## Coherence

<!-- RECONCILER-OWNED -->

Erhoben vom `reconciler` am 260831-1417, Domäne `code`, Circle `260830-1045-git-bereich-liest-status-branch-verlauf`, Stand `d1fbaac..2976520` (24 Commits), zwei Turns.

**Verdict:** review-needed

**Edges:**

- Artifact↔Grounding: 16 von 16 `[DONE]`-Schritten gegen den Baum gelesen und belegt, 65 von 90 Abnahmekriterien mit Stelle oder Probe belegt (die übrigen 25 sind Nutzerarbeit aus Schritt 17 und keine Lücke), `make check` nachgefahren und grün (`cargo build/test/clippy -D warnings/fmt`, je exit 0) / **4 Abweichungen (Grundlage im Fehler)**: `Status: Draft` an Plan und Spec bei sechzehn erledigten Schritten (berichtigt auf `Partially Complete`), drei Entscheidungsdatensätze auf `_a_`, während der Baum ihre Antwort trägt (`260830-1006_*_bekommt-der-git-bereich-einen-sechsten-fokuswert-…` → `c99d433`, `…_wohnt-die-git-anbindung-in-krk-core-…` → `1d84f2b`, `…_was-zeigen-git-bereich-ankreuzfeld-…` → `7264daf`; alle drei auf `_i_` gezogen), und der Spec-Absatz zu den zehn Zeitzusagen, dessen Begründung auf zwei Schalterständen ruht, während die Markenspalte ab Werk steht (`Spaltensichtbarkeit::default`, `crates/krk-core/src/ablage/sitzung.rs:344`; `gitbedarf_nachziehen`, `crates/krk-ui/src/appkit/anwendung.rs:4642`) — gefilt und offen / **0 Befunde aus coderev und ontorev, weil keine Durchsicht gelaufen ist**: `bin/fusion-review-coverage` meldet `commits=24 reviews=0 uncovered=24 verdict=uncovered`, gefilt als `260831-1417_*_die-runde-23-schliesst-ohne-durchsicht-und-vierundzwanzig-commits-sind-ungedeckt.md`. Kein einziger Befund liegt am Baum; alle vier sind Buchführung, drei davon in diesem Lauf berichtigt.
- Artifact↔Directive: **Die Commits bewegen sich auf die Directive zu.** Dreiundzwanzig der vierundzwanzig tragen die Stufe A und nichts sonst: `c99d433` (sechster Bereich, sechster Fokuswert), `4f6b880` (fünfte Spalte), `1d84f2b` und `437fd69` (`gix` im Kern, Gitleser und Gitlauf), `7ad8978` (Befundvektor im Ordnermodell), `3090441` (Lauf am Tab, Markenzelle), `7264daf` (die drei Flächen), `7079519` und `5a1cbe8` (drei Befehle, zehnter Funktionsbereich, drei Belegungseinträge), `d1f86ba` und `1888ef0` (Messung und Entscheid zum Index-Posten), `c68f843`, `2ab5328`, `ad7c2f2`, `9566973`, `6c3927e`, `63aa690` (Nachzüge, C-Freiheits-Zusage, CLAUDE.md, Abnahme) sowie die sechs Workbench-Commits der Vorbereitung. Der einzige, der nicht auf die Directive zeigt, ist `2976520`: ein vom Nutzer während der Sitzung beauftragter Nebengang zu den Leseprofilen, der keine Zeile im Baum geändert hat und nach der Herkunftsregel in `shared/` gefilt ist. Kein Commit bewegt sich von der Directive weg.
- Grounding↔Directive: **59 aktive Entscheidungen (`_o_` und `_a_`) über beide Speicher, keine widersprechend.** Zwei berühren die Directive unmittelbar und stützen sie: `260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md` ist der Grund, aus dem Schritt 17 Nutzerarbeit bleibt, und `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` hat durch diese Runde ihren härtesten Beleg bekommen (neun stille Stellen, von Hand nachgezogen, vom Übersetzer keine genannt). `260802-0842_*_git-verwerfen-bedeutung.md` bindet die Stufe B und ist von der schreibfreien Stufe A nicht berührt. Eine Aussage hat die Runde bewegt, ohne sie zu widerlegen: `260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md` fragt über einen Stand von 81 Funktionen und neun Obermenüs, während die Belegung jetzt 91 Funktionen und zehn Obermenüs führt; der Datensatz ist eine Aufzeichnung seines Standes und behält ihn nach der Ortsregel, die Frage selbst bleibt offen und gültig.

**Rebalance recommendation:** revise Grounding

Die Empfehlung ist beratend. Sie zielt auf zwei Stellen und nicht auf die Runde: der Spec-Absatz zu den zehn Zeitzusagen begründet ein richtiges Ergebnis falsch, und vierundzwanzig Commits stehen ohne Durchsicht. Beides ist gefilt, keines hält den beschränkten Abschluss auf. Die Directive selbst ist nicht zu berichtigen: sie ist gestellt, im Baum erreicht und allein in ihrer Anzeigehälfte unabgenommen, und genau das drückt der Marker `_b_` aus.

## Die Auslieferung ohne Abnahmelauf

Der Nutzer hat am 260831 entschieden, die Fassung 1.5.0 auszuliefern, **ohne** den
Abnahmelauf aus Schritt 17 gefahren zu haben. Die Wahl ist ihm mit ihren Folgen vorgelegt
worden und hier festgehalten, weil sie über diese Runde hinaus wirkt.

**Was damit ungeprüft veröffentlicht wird:** 25 der 90 Abnahmekriterien, und es sind die
sichtbaren. Dass der Git-Bereich erscheint und seine drei Flächen füllt, dass die
Markenspalte die fünf Buchstaben an den richtigen Zeilen zeigt, dass `opt+cmd+r` und
`shift+cmd+b` tun, was die Belegung verspricht, dass beim Ordnerwechsel nichts flackert
und die erste Bildschirmseite nicht später steht als vor der Runde. Geprüft ist der Bau:
`make check` grün, `cargo xtask bundle` signiert, 65 Kriterien durch Proben oder Stellen
im Baum belegt.

**Zwei Bedingungen sind eingehalten worden.** Die Durchsicht der Runde geht der
Auslieferung voraus und nicht umgekehrt — der Plan verlangt es ausdrücklich, weil der
umgekehrte Fall in diesem Projekt schon eingetreten ist (v10.0.0 wurde getaggt und
geschoben, bevor die Durchsicht lief). Und die Zahl hat der Nutzer gewählt: 1.5.0 nach der
Minor-Regel des README, weil die Runde eine neue Fähigkeit bringt und keine Tastenbedeutung
ändert.

**Die Runde bleibt beschränkt geschlossen.** Eine Auslieferung nimmt den Abnahmelauf nicht
vorweg; sie macht ihn dringlicher.

## Die Durchsicht nimmt die Bounded Closure zurück

Die Durchsicht der Runde ist über die vollen 24 Commits gelaufen und hat 13 Defekte
gefunden, 50 der 51 Dateien geöffnet. Damit steht die Deckung des Bereichs auf
`uncovered=0`. Zwei der Befunde haben den Nutzer seine Entscheidung ändern lassen, und die
Reihenfolge ist der Grund: er hatte die Auslieferung ohne Abnahmelauf gewählt, während die
Durchsicht noch lief.

**Der Nachschlag des Verlaufs verliert jeden Nebenzweig.** Er setzt am letzten angezeigten
Commit an und läuft über dessen Vorfahren; bei der Vorbelegung `BreadthFirst` stehen
mehrere Zweige nebeneinander in der Warteschlange, und jeder Commit, der beim Schwungende
darin stand und kein Vorfahre des letzten ist, kommt nie mehr in die Liste. **KRKs eigenes
Repository ist linear, und der Abnahmelauf hätte den Fehler dort nicht gesehen** — er wäre
erst einem Nutzer mit verzweigter Historie aufgefallen.

**Dateien mit zerlegten Namen bekommen keine Marke.** Der Bestand kommt aus `readdir`, der
Befund aus `gix`, und `gix` liefert vorkomponiert, weil `git` auf macOS
`core.precomposeUnicode` ab Werk setzt. Ein Name in NFD trägt eine andere Bytefolge und
fällt durch; die Zeile bleibt leer und zählt in der Zusammenfassung nicht mit. Derselbe
Vergleich steht an der Konfliktprüfung schon als offener Defekt.

**Der Nutzer hat entschieden, beide vor der Auslieferung zu beheben.** Die Bounded Closure
ist damit zurückgenommen, der Circle bleibt aktiv, und die Auslieferung von 1.5.0 folgt
nach den zwei Behebungen und einer erneuten Abnahme.
