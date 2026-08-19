CLAUDE.md nennt zwei nachzuziehende Stellen je Kommando, die dritte hält kein Übersetzer

---

`CLAUDE.md` sagt unter „Was man nicht sieht, wenn man es nicht weiß": „Jedes neue Kommando
braucht eine Zeile in `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und in
`bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`)". Beide Aussagen stimmen, und beide
Stellen hält der Übersetzer.

**Die dritte Stelle fehlt in der Aufzählung, und sie ist die einzige, die niemand hält.** Der
Ausführungszweig in `Anwendungsdelegierter::kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:2896`) endet auf einen Auffangzweig
(`andere => self.bereichskommando(fokus, andere)`, `:2983`). Ein neues Kommando ohne eigenen
Zweig übersetzt, besteht jede Probe, erscheint im Hauptmenü und in der Belegungsansicht mit
seinem Namen und seiner Kombination — und tut nichts. Dieselbe Lage trägt
`Tabelle::kommando_ausfuehren` (`crates/krk-ui/src/appkit/tabelle.rs:1025`) mit
`_ => return false` (`:1061`).

---

**Severity:** Medium. Kein Schaden an Daten, aber die Fehlerform ist die teuerste, die dieser
Baum kennt: grüner Bau, grüne Proben, sichtbarer Menüeintrag, keine Wirkung. Wer der
Aufzählung in `CLAUDE.md` folgt und sie für vollständig hält, liefert genau das aus.
**Found by:** planner, beim Planen der Runde 13 gegen den Baum gemessen und nicht aus der Prosa
übernommen. Alle `match`-Blöcke über `Kommando` in `crates/` wurden klammerbalanciert auf einen
Auffangzweig geprüft: zwei ohne (`belegung.rs:799`, `belegungsmodell.rs:227`), sechs mit.
**Affected:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß", Absatz
„Etliche Fallunterscheidungen sind vollständig und haben keinen Auffangzweig"
**Related:** `crates/krk-ui/src/appkit/anwendung.rs:3050-3053` — der Baum selbst warnt an der
richtigen Stelle: „**Ohne diesen Zweig fiele der Befehl durch den Auffangzweig unten und täte
nichts**, und der Übersetzer sagte dazu kein Wort." Die Warnung steht im Code und nicht in
`CLAUDE.md`.
**Tree state:** `b47355e`
**Domain:** code

## Warum das nicht bloß eine fehlende Zeile ist

Der Absatz in `CLAUDE.md` handelt ausdrücklich davon, was der Übersetzer erzwingt, und schließt
mit „wer eine davon erweitert, bekommt vom Übersetzer die Liste der Stellen, die nachzuziehen
sind". Für die zwei genannten Stellen gilt das. Für die dritte gilt das Gegenteil, und gerade
sie steht nicht da. Ein Leser, der dem Absatz vertraut, zieht den Schluss, die Liste des
Übersetzers sei vollständig — und das ist sie an genau der Stelle nicht, an der ein Kommando
seine Wirkung bekommt.

## Was eine Behebung leisten müsste

Den Absatz um die dritte Stelle ergänzen und dabei die Trennung mitschreiben, die er heute
verwischt: welche Stellen der Übersetzer hält, welche eine Probe hält
(`belegungsausgabe.rs:755`, `belegung.rs:1636` und die Zahlenprobe über den Kopf von
`resources/default-keymap.toml`), und welche gar nichts hält. Die Runde 13 trägt die vollständige
Liste in ihrem Plan
(`circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_*_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`,
Abschnitt „Was der Übersetzer einfordert, und was er nicht einfordert"); sie ist von dort zu
übernehmen und nicht neu zu erheben.

**Filed by:** planner

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und die Rechnung ist ausgezählt.** `CLAUDE.md:120` nennt unverändert zwei nachzuziehende Stellen je neuem Kommando. Am Baum sind es **15**: **vier** hält der Übersetzer über vollständige Fallunterscheidungen (`crates/krk-core/src/tasten/belegung.rs:814` und zwei weitere in derselben Datei, dazu `crates/krk-ui/src/belegungsmodell.rs:227`), **zwei** hält eine Probe über die Kopfzählung von `resources/default-keymap.toml`, und **neun** hält nichts. Die zwei, die `CLAUDE.md` nennt, sind beide übersetzergehalten — also gerade die, die von selbst auffallen.

**Die zwei ungehaltenen Auffangzweige, an denen ein neues Kommando still wirkungslos bleibt, stehen heute bei** `crates/krk-ui/src/appkit/anwendung.rs:3145` (`andere => self.bereichskommando(fokus, andere)`) und `crates/krk-ui/src/appkit/tabelle.rs:1494` (`_ => return false`). Ein Kommando ohne eigenen Zweig übersetzt, besteht jede Probe, steht mit Namen und Kombination im Hauptmenü und tut nichts. Der Marker bleibt `_o_` für den Durchgang des Kurators.
