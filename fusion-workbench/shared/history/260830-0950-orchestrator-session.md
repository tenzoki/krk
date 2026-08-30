# Orchestrator-Sitzung — 260830-0950

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Directive:** KRK bekommt eine Git-Anbindung, in Stufen. Stufe A zuerst: ein Git-Bereich am rechten Rand als sechster Wert von `Bereich`, der den Status des angezeigten Ordners, den aktuellen Branch und den Verlauf anzeigt; die Statusmarken in der Dateiliste sind über ein Ankreuzfeld der Bereichsleiste zuschaltbar. Danach Stufe B, die vier Operationen der Runde 1 (hinzufügen, committen, Änderungen verwerfen, Versions-Schieberegler), danach eine Auslieferung. Stufe C (Branches wechseln) ist möglich, Stufe E (PRs und Review-Freigabe) unwahrscheinlich. Bibliothek: `gix` (gitoxide) wird probiert. Dass die zehn Zeitzusagen aus C8 dabei fallen können, hat der Nutzer ausdrücklich in Kauf genommen.
**Mode:** custom — erst Machbarkeitsanalyse, dann Shaper
**Status:** In Arbeit

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
