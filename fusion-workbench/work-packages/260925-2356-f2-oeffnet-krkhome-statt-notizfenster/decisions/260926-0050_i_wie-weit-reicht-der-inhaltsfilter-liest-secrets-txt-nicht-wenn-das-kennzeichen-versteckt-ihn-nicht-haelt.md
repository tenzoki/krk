# Wie weit reicht „der Inhaltsfilter liest .secrets.txt nicht“, wenn das Kennzeichen „versteckt“ ihn gar nicht aufhält?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_was-heisst-immer-gelistet-fuer-secrets-txt.md, 260926-0017-zweitlesung-spec-f2-krkhome.md, 260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md

---

## Question

C7 des Spec verlangt: „Der Inhaltsfilter liest `.secrets.txt` nicht, weil sie versteckt gekennzeichnet bleibt; eine Probe hält es fest.“ **Die Begründung trifft den Baum nicht.** Der Inhaltsfilter fragt nicht nach dem Kennzeichen `versteckt`, sondern nach der Zeile: einen Inhaltsauftrag bekommt jeder Eintrag, der den ersten Zweig des Prüfschritts übersteht, und dieser Zweig lautet „versteckt **und** ausgeblendet, fällt weg“ (`zeilengrund_von` und `auftraege` in `crates/krk-core/src/verzeichnis/modell.rs`). Drei Wege lesen die Datei deshalb heute oder nach Stufe 5:

1. **Die Ausnahme selbst.** „Steht immer“ lässt `.secrets.txt` den ersten Zweig passieren. Ohne weitere Regel bekommt sie danach wie jede Datei einen Inhaltsauftrag, sobald der Filtertext die Schwelle erreicht und „Content“ angekreuzt ist.
2. **Eingeblendete Verstecke.** Schaltet der Nutzer mit `shift+cmd+h` die versteckten Einträge ein, liest der Inhaltsfilter jede versteckte Datei, heute schon und in jedem Ordner.
3. **Die tiefe Suche aus einem übergeordneten Ordner.** Der Durchlauf über den Unterbaum (`unterbaum_entscheiden` in `crates/krk-core/src/verzeichnis/durchlauf.rs`) fragt das Kennzeichen gar nicht. Filtert der Nutzer in `~` mit „Deep“ und „Content“, liest er `~/krkhome/.secrets.txt` wie jede andere Datei darunter.

Gelesen würde in allen drei Fällen Chiffrat. Ein Treffer ist nur möglich, wenn der Filtertext zufällig als Bytefolge im Chiffrat steht, und ein Klartext kommt nie an. Die Frage ist, wie weit das Kriterium reichen soll, weil jede Reichweite über den ersten Weg hinaus eine Regel an einer Stelle braucht, die heute keine kennt.

## Options

1. **Die Zusage gilt im erkannten Ordner selbst, und dort über die Ausnahme.** Die Ausnahme „steht immer“ lässt `.secrets.txt` allein über ihren Namen stehen oder fallen und gibt ihr nie einen Inhaltsauftrag, gleich wie der Umschalter steht. Das deckt Weg 1 und Weg 2 für diese eine Datei in diesem einen Ordner. Die tiefe Suche aus einem übergeordneten Ordner liest das Chiffrat weiter, wie sie jede versteckte Datei liest.
   - Pros: kein zusätzlicher Vergleich in einem anderen Ordner und keiner im Durchlauf über den Unterbaum; die Regel steht an derselben Stelle wie die Ausnahme und wird mit ihr beim Lesen einmal gesetzt. Das Kriterium hält mit einer neuen Begründung für den Ordner, in dem der Nutzer die Datei sieht.
   - Cons: „liest nicht“ gilt nicht aus jedem Ordner. Das Kriterium im Spec wird auf den erkannten Ordner eingeschränkt und seine Begründung berichtigt.
2. **Wie 1, und der Durchlauf über den Unterbaum liest die eine Datei ebenfalls nicht.** Erkannt wird sie am offenen Deskriptor über Gerät und Inode, verglichen mit denen von `~/krkhome/.secrets.txt`, die der Auftrag einmal zu Beginn erhebt.
   - Pros: die Zusage gilt aus jedem Ordner.
   - Cons: der Durchlauf und sein Leseweg bekommen einen Vergleich je gelesener Datei und eine Eingabe von außen, die sie heute nicht haben. Das trifft den Teil des Baums, dessen Deskriptorregeln die Runden 10 und 11 mit eigenen Proben unter `ulimit -n 64` abgesichert haben.
3. **Keine Datei namens `.secrets.txt` wird je vom Inhaltsfilter gelesen, in keinem Ordner und auf keinem Weg.**
   - Pros: eine Namensregel, einfach zu prüfen, deckt alle drei Wege.
   - Cons: eine Regel, die außerhalb von `~/krkhome` wirkt, ohne dass der Nutzer sie dort bestellt hat; der Beschluss zu „immer gelistet“ hat genau diese Verallgemeinerung für die Sichtbarkeit abgelehnt.

## Constraints

- Ein Ordner, der nicht `~/krkhome/` ist, durchläuft den Prüfschritt der Sichtbarkeit ohne zusätzlichen Vergleich je Eintrag (C7, Haltestelle zu L3 und L10).
- Das Kennzeichen `versteckt` bleibt am Eintrag stehen (`260926-0007_*_was-heisst-immer-gelistet-fuer-secrets-txt.md`).
- Nach dem Bedrohungsmodell des Nutzers zählt, dass kein Werkzeug Klartext sieht; das Lesen von Chiffrat durch KRK selbst verletzt es nicht.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie hält, was das Bedrohungsmodell verlangt, und bringt die Regel an die Stelle, an der der Nutzer die Datei sieht, ohne den Durchlauf über den Unterbaum oder einen anderen Ordner anzufassen. Möglichkeit 2 ist der Weg, wenn das Kriterium aus jedem Ordner gelten soll; sie ist baubar, aber nicht umsonst. Der Plan baut Schritt 5.2 auf Möglichkeit 1 und benennt dort, was Möglichkeit 2 hinzufügte.

---
Answered: dieser Datensatz `## Options` und 260926-0107-zweitlesung-plan-f2-krkhome.md — Möglichkeit 1, die Zusage gilt im erkannten Ordner über die Ausnahme; die tiefe Suche aus einem übergeordneten Ordner liest weiter das Chiffrat, was unter dem Bedrohungsmodell des Nutzers hinnehmbar ist; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 6f50611 — im erkannten Ordner kein Inhaltsauftrag für .secrets.txt, entschieden am Namen im Zweig der Verstecke
