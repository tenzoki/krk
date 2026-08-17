# Sieht die Git-Prüfung nur den Ordner selbst an oder auch die Ebenen darüber?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (C3), `shared/analyses/260817-0419-verlust-des-speichers-shared.md`

---

## Question

Der fünfte Warngrund dieser Runde ist der Git-Arbeitsbaum. Der Nutzer hat ihn so bestimmt: ein Ordner, der selbst ein `.git` enthält, warnt laut, auch bei wenigen Einträgen, und ausdrücklich nicht jeder Pfad innerhalb eines Arbeitsbaums.

Diese Festlegung ist gefallen, bevor jemand nachgerechnet hat, ob sie den Fall trifft, der die Runde ausgelöst hat. Sie trifft ihn nicht. Der am 260817-0344 geräumte Pfad war `…/krk/fusion-workbench/shared`; dieser Ordner enthält kein `.git`, der Arbeitsbaum liegt zwei Ebenen darüber. Von den fünf Zielprüfungen hätte beim Schadensfall keine angeschlagen. Verhindert hätten ihn die unbedingte Rückfrage und die Umfangsschwelle, beide unabhängig von dieser Prüfung.

Die Frage ist damit, ob der Nutzer bei seiner Festlegung bleibt, nachdem er diese Rechnung kennt.

## Options

1. **Am Ordner selbst prüfen** — die Festlegung des Nutzers, wie sie steht.
   - Pro: eine Frage an genau einen Ort, ein Zugriff je ausgewähltem Ordner, keine Schleife nach oben. Warnt genau dann, wenn der Nutzer im Begriff ist, ein ganzes Projekt wegzuräumen.
   - Contra: der eine Fall, der diese Runde ausgelöst hat, ist nicht abgedeckt. Ein Unterordner eines Arbeitsbaums, der Monate an verfolgter Arbeit trägt, warnt aus diesem Grund nicht.

2. **Aufwärts prüfen bis zum Benutzerordner oder zur Wurzel** — jeder Pfad innerhalb eines Arbeitsbaums warnt.
   - Pro: der Schadensfall wäre erfasst, und mit ihm jede Löschung innerhalb eines Projekts.
   - Contra: wer in einem Quellbaum arbeitet, löscht dort ständig, und jede dieser Löschungen würde laut. Eine Warnung, die fast immer erscheint, wird überlesen, und dann trägt sie auch dort nichts mehr, wo sie recht hat. Der Nutzer hat diese Lesart aus genau diesem Grund abgelehnt.
   - Kosten: bis zu einem Zugriff je Ebene zwischen dem Ordner und dem Benutzerverzeichnis, bei jedem Löschbefehl.

3. **Aufwärts prüfen, aber nur als stille Ergänzung der Erläuterung** — die Frage bleibt in ihrer ruhigen Form, die Erläuterung nennt den Arbeitsbaum und seinen Pfad.
   - Pro: die Auskunft steht da, ohne dass die laute Form abstumpft. Der Nutzer sieht im Blatt, dass er innerhalb eines Projekts löscht.
   - Contra: ein dritter Grad zwischen ruhig und laut, und damit eine Fallunterscheidung mehr, als der Spec sonst kennt. Wer die Erläuterung nicht liest, hat nichts davon.

4. **Statt aufwärts zu prüfen, den Ordner nach verfolgten Änderungen fragen** — warnen, wenn der Vorgang verfolgte Dateien beträfe.
   - Pro: träfe den Schadensfall genau.
   - Contra: nicht entscheidbar mit den Mitteln dieser Runde. KRK trägt heute keine Git-Anbindung, `Kommando` führt am 260817 keine einzige Git-Variante, und den Index eines Arbeitsbaums zu lesen wäre eine eigene Runde.

## Constraints

- Die Prüfung läuft, während der Nutzer eine Taste gedrückt hat, und vor dem Erscheinen der Rückfrage. Ihre Kosten müssen beschränkt bleiben.
- KRK hat keine Git-Anbindung. Jede Prüfung dieser Runde arbeitet am Dateisystem, nicht an einem Repository.
- Unentschieden gilt als laut; ein Zugriff, der scheitert, macht die Rückfrage laut.

## Recommendation

Bei Möglichkeit 1 bleiben, also bei der Festlegung des Nutzers, und die Lücke im Spec benennen statt sie zu schließen. Die Begründung des Nutzers gegen die weite Lesart trägt weiterhin: eine Warnung, die in einem Quellbaum bei jeder Löschung erscheint, verliert ihre Wirkung. Der Schadensfall ist durch die unbedingte Rückfrage abgedeckt, und diese Abdeckung hängt an keiner Zielprüfung. Die Empfehlung ist eine Abwägung und keine geprüfte Aussage; die Entscheidung liegt beim Nutzer, weil sie über die Zahl der Warnungen entscheidet, die er täglich sieht.

## Antwort des Nutzers

**Am 260817, bei der Abnahme des Specs: Möglichkeit 2.** Die Prüfung sieht auch aufwärts. Jeder Pfad innerhalb eines Git-Arbeitsbaums löst die laute Form aus, nicht nur der Ordner, der die Verwaltung selbst trägt. Der Nutzer hat damit seine eigene Festlegung aus der zweiten Klärungsrunde umgedreht, nachdem die Kalibrierung gezeigt hat, dass die enge Form seinen Schadensfall vom 260817-0344 nicht trifft.

**Die Empfehlung des Shapers ist damit ausdrücklich verworfen, und ihr Einwand bleibt gültig.** Der Nutzer arbeitet in einem Quellbaum, und nach dieser Antwort warnt dort fast jede Löschung laut. Eine Warnung, die im Alltag beinahe immer erscheint, verliert ihre Unterscheidungskraft, und sie verliert sie genau an der Stelle, an der der Nutzer sie am häufigsten sieht. Der Einwand steht als benannte Folge im Spec, unter C3 und in der Kalibrierung; er soll beim ersten lauten Blatt auffindbar sein und nicht neu entdeckt werden müssen.

Die Reichweite des Aufwärtsgangs ist im Spec unter C3 festgelegt, ebenso die Kosten, die er der Prüfung vor der Rückfrage hinzufügt.

---
Answered: `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C3 und Abschnitt `## Was die Prüfungen beim Vorfall vom 260817 geleistet hätten` — Möglichkeit 2: die Prüfung sieht aufwärts; die Empfehlung des Shapers ist verworfen, sein Einwand steht als benannte Folge im Spec.
Implemented: `5a0f041`, `c1b52db`, `792995a` — Möglichkeit 2, der Aufwärtsgang, steht am Baum. `5a0f041` (Schritt 8): `krk_core::verzeichnis::arbeitsbaum::liegt_in_arbeitsbaum` (`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:288`) endet am mitgegebenen Benutzerverzeichnis oder an der Wurzel, `beruehrt_einen_arbeitsbaum` (`:338`) setzt ihn mit der Schleife über die Auswahl zusammen. `c1b52db` (Schritt 10): `Warngrund::Arbeitsbaum` (`crates/krk-ui/src/kommandos/loeschwarnung.rs:486`) und seine Bewertung in `warngruende` (`:642`). `792995a` (Schritt 11): die Verdrahtung in `crates/krk-ui/src/appkit/anwendung.rs`, die das Feld `Loeschziel.arbeitsbaum` füllt und das Blatt laut macht. Die Abgleichsnotiz vom 260817-1833 unter diesem Absatz hat den Beleg vorgeprüft; er ist am 260818 gegen den Baum nachgelesen und hält. Bewegt in Schritt 16 des Plans `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`.
Deferred:
Superseded by:

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **die Antwort ist am Baum vollständig
realisiert; der Marker bleibt trotzdem `_a_`.**

Möglichkeit 2 steht in drei Commits und ist einzeln nachgelesen. Der Aufwärtsgang:
`krk_core::verzeichnis::arbeitsbaum::liegt_in_arbeitsbaum`
(`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:288`) geht über `verzeichnis::aufwaerts` und
endet am mitgegebenen Benutzerverzeichnis oder an der Wurzel; `beruehrt_einen_arbeitsbaum`
(`:338`) setzt ihn mit der Schleife über die Auswahl zusammen und fragt die Auswahl nur, wenn der
Gang `Nein` sagt; elf Proben in `crates/krk-core/tests/arbeitsbaum.rs` messen es, darunter „zwei
Ebenen darüber" (`:71`) und „oberhalb der Grenze" (`:112`). Commit `5a0f041`, Schritt 8 des Plans.
Die Bewertung: `Warngrund::Arbeitsbaum` in
`crates/krk-ui/src/kommandos/loeschwarnung.rs:461` mit seinem Rang aus der
Deklarationsreihenfolge, gelesen von `warngruende` (`:642`). Commit `c1b52db`, Schritt 10. Die
Verdrahtung: `appkit/anwendung.rs:4857` füllt das Feld `Loeschziel.arbeitsbaum`, und `:4874`
macht das Blatt laut, sobald die Liste der Warngründe nicht leer ist. Commit `792995a`,
Schritt 11. `make check` — exit 0.

**Warum der Marker nicht wandert.** Der Plan
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`
hängt die Bewegung dieses und der drei anderen Datensätze ausdrücklich an Schritt 16, und der ist
nicht gefahren; die Tabelle `## Welcher Schritt welchen Entscheidungsdatensatz realisiert` nennt
für diesen Datensatz die Schritte 8, 10 und 11 und als Ziel „Schritt 16". `_i_` ist daneben ein
Endzustand, aus dem nur eine Überholung herausführt, also ist eine vorgezogene Bewegung nicht
billig zurückzunehmen. Diese Notiz steht hier, damit Schritt 16 den Beleg vorgeprüft findet und
nur noch die Zeile `Implemented:` mit den drei Commits zu setzen hat.

Die drei anderen Datensätze der Reihe `260817-0536_a_*` sind **nicht** realisiert: sie hängen an
den Schritten 12 und 13, und `resources/default-keymap.toml:151` führt `endgueltig_loeschen`
unverändert mit `["f8", "opt+cmd+delete"]`.
