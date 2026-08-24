Der Kopf des Speicherprofils nennt achtzehn Orte und neun Speicher je Runde; es sind 99 und fünf

---

`resources/default-readers.toml:205-206` sagt: „Ein Profil für achtzehn Orte: die neun Speicher
unter `shared/` und dieselben neun in jeder Runde." Gemessen trifft das Profil 99 Ordner, und vier
der neun Namen können in einer Runde gar nicht vorkommen. Die Zahl ist mit `b5bf2e3` von „zwölf"
auf „achtzehn" nachgezogen worden und dabei nach demselben Rechenweg falsch geblieben.

---

**Gemessen am 260824-1651 mit `regex` 1.13.1** gegen alle 154 Verzeichnisse unter
`fusion-workbench/`, mit dem Pfadmuster aus `resources/default-readers.toml:216`:

| | Ordner |
|---|---|
| Profil „ein Speicher" | **99** |
| Profil „ein Defektspeicher" | 19 |
| Profil „eine Runde" | 18 |
| Profil „die Wurzel" | 1 |
| Profil „alle Runden" | 1 |
| ohne Profil | 16 |

Die 99 zerfallen in **9** Speicher unter `shared/` und **90** in den achtzehn Runden, also **fünf**
je Runde und nicht neun. Nachgezählt: jedes der achtzehn Rundenverzeichnisse führt genau sechs
Unterordner, `analyses`, `decisions`, `history`, `issues`, `planning`, `reviews`; `issues` gehört
dem Nachbarprofil, bleiben fünf.

**Die vier fehlenden Namen können in einer Runde nicht stehen, und das ist keine Eigenheit dieser
Werkbank.** `rules/fusion-workbench-conventions.md:78` legt fest: „`investigations/`, `consult/`,
`memos/` und `backlog/` exist only in `shared/`". Gemessen: null Vorkommen in achtzehn Runden. Die
vier Alternativen im Pfadmuster schaden nichts, aber der Satz „dieselben neun in jeder Runde"
behauptet etwas, das die Konventionen ausschließen.

**Warum das zählt.** Die Datei ist zur Hälfte Kommentar, und das ist ihr Zweck: sie ist die eine
Stelle, an der der Nutzer die Sprache dieser Datei nachschlägt. Eine Zahl darin, die der Bestand
nicht trägt, ist derselbe Fehlertyp, den `shared/issues/260812-1438_*_…` und
`shared/issues/260812-2253_*_…` für `CLAUDE.md` festhalten.

**Vorschlag.** Den Satz auf das umstellen, was er beschreiben soll, nämlich die Namen und nicht die
Orte: „Ein Profil für die neun Speichernamen unter `shared/` und die fünf, die daneben in jeder
Runde stehen." Eine Zahl der getroffenen Ordner gehört nicht in den Kommentar; sie wächst mit jeder
Runde.

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

---
Resolved: Der Kopf des Speicherprofils steht jetzt auf den Namen und nicht mehr auf den Orten:
„Ein Profil für die neun Speichernamen unter `shared/` und die fünf, die daneben in jeder Runde
stehen." Die vier Namen, die eine Runde nicht führen kann — `backlog`, `consult`,
`investigations`, `memos` —, sind einzeln genannt, mitsamt dem Grund, warum sie trotzdem in der
Aufzählung des Pfadmusters stehen. Eine Zahl der getroffenen Ordner steht nicht mehr da; an ihrer
Stelle sagt der Kommentar ausdrücklich, dass der Bestand sie nennt und sie mit jeder Runde wächst.

Der zweite Satz desselben Kopfes ist mitberichtigt worden, weil die Änderung am Nachbarprofil ihn
falsch gemacht hätte: „weil er zwei Zählungen trägt und nicht eine" stimmte nach der Räumung von
`issues/260824-1649_*_…` nicht mehr, der Defektspeicher trägt jetzt drei. Er lautet jetzt „weil er
nach Markern aufschlüsselt".

**Nachgerechnet am 260824-1739** mit `regex` 1.13.1 gegen alle 154 Verzeichnisse unter
`fusion-workbench/`: das Profil trifft weiterhin 99 Ordner, 9 unter `shared/` und 90 in den
achtzehn Runden, also fünf je Runde. `find circles -maxdepth 2 -type d` über die vier Namen
liefert null Vorkommen in Runden. Die Erkennung ist unberührt: 1 / 99 / 19 / 1 / 18 und 16 ohne
Profil, wie vor der Änderung.
