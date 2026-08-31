# Wo wohnt die Auswahl der Verlaufsliste — im `Gitfenster` oder im `Gitmodell`?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Entscheidung 5, Schritt 7, Schritt 8); `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` (C3.4, C3.5, C4.2, C4.6); `crates/krk-ui/src/appkit/git.rs` (Modulkopf `# Die Auswahl wohnt in der Liste`, `GitfensterIvars::auswahl`); `crates/krk-ui/src/gitmodell.rs` (Feld `auswahl`, `auswahl`, `auswahl_setzen`, `ausgewaehlter_commit`); `crates/krk-ui/src/tabs.rs` (`Tabinhalt::gitmodell`, „Nur zu lesen"); `history/260831-0100-coder-schritt-6-der-lauf-am-tab-und-die-markenzelle.md`

---

## Question

Schritt 6 hat der Auswahl im `Gitmodell` ein Feld gegeben, Schritt 7 hat sie in
den `Gitfenster`-Ivars gebaut, und beide stehen jetzt nebeneinander. Der Grund
ist keine Nachlässigkeit, sondern ein Schnitt, den die zwei Schritte
verschieden gelesen haben: der Kommentar am `expect(dead_code)` in
`gitmodell.rs` nennt `auswahl_setzen` unter den Ablesern von
`Gitfenster::zeigen`, aber `zeigen` bekommt das Modell nach dem Plan **lesend**
(`zeigen(&Gitmodell)`), und `kommando_ausfuehren(kommando)` bekommt es
überhaupt nicht. Ein Schreiber aus dem Git-Bereich heraus gibt es damit nicht,
und `Tabinhalt::gitmodell` sagt ausdrücklich, dass ein Schreiber von außen eine
zweite Quelle für denselben Stand wäre.

Die Frage muss jetzt beantwortet werden, weil sie **Verhalten** entscheidet, das
der Nutzer sieht, und weil Schritt 8 den Anwendungsdelegierten baut, der den
Rückweg tragen müsste: es gibt ein `Gitfenster` für das ganze Fenster, aber ein
`Gitmodell` je Tab. Wohnt die Auswahl in der Ansicht, fällt sie beim Tabwechsel;
wohnt sie im Modell, kommt sie beim Zurückwechseln wieder.

## Options

1. **Sie bleibt in der Ansicht** (der gebaute Stand aus Schritt 7)
   - Pros: keine zweite Meldung und kein zweiter Melder; `zeigen` bleibt lesend
     und `Tabinhalt::gitmodell` bleibt „nur zu lesen"; die Zusagen C3.5, C4.2
     und C4.6 hält der Schritt heute schon, weil `zeigen` die Auswahl nur dort
     stehen lässt, wo die Zeile wortgleich dieselbe geblieben ist.
   - Cons: die Auswahl überlebt keinen Tabwechsel und keinen Wechsel des
     aktiven Dateifensters; `Gitmodell::auswahl`, `auswahl_setzen` und
     `ausgewaehlter_commit` behalten nach dieser Runde keinen Rufer im
     ausgelieferten Bau, und das `expect(dead_code)` in `gitmodell.rs` bleibt
     mit einem Grund stehen, der einen erledigten Schritt nennt.

2. **Sie zieht in das `Gitmodell`, und der Git-Bereich meldet sie nach oben**
   - Pros: eine Heimat für einen Stand; die Auswahl überlebt den Tabwechsel,
     wie es das „ein Gitmodell je Tab" aus dem Modulkopf für Kopf und Verlauf
     schon zusagt; die drei Leser bekommen ihren Rufer und das
     `expect(dead_code)` fällt.
   - Cons: ein zweiter Melder neben dem Nachlademelder, den Entscheidung 5
     nicht vorsieht; der Anwendungsdelegierte braucht einen schreibenden Zugang
     zum Gitmodell des sichtbaren Tabs, also eine Ausnahme von „nur zu lesen";
     zwischen dem Tastendruck und dem nächsten `zeigen` steht die Liste einen
     Augenblick auf einem Stand, den das Modell noch nicht trägt.

3. **Sie zieht in das `Gitmodell`, und `zeigen` bekommt es veränderlich**
   - Pros: kein zweiter Melder; genau die sieben Ableser, die der
     History-Eintrag von Schritt 6 aufzählt.
   - Cons: `zeigen(&mut Gitmodell)` und `kommando_ausfuehren` bräuchte es
     ebenso; damit fiele die Zusage aus `Tabinhalt::gitmodell` ganz, und die
     Ansicht schriebe in den Stand, den der Einzugstakt füllt — genau die zwei
     Schreiber, die jene Zeile ausschließt.

## Constraints

- C3.5 bleibt: ohne Auswahl bleibt die Fläche der Einzelheiten leer, und es
  steht kein Platzhaltertext.
- C4.2 bleibt: am letzten Eintrag bewegt sich nichts, die Liste springt nicht,
  und der Nachschlag wird angefordert.
- C4.6 bleibt: ein Ordnerwechsel lässt keine Auswahl des vorigen Ordners stehen.
- A8 bleibt: während des Laufs steht nichts da.
- Was auch immer gewählt wird, es gibt danach **eine** Heimat für die Auswahl
  und nicht zwei.

## Recommendation

Möglichkeit 1, solange kein Nutzerurteil dagegen steht — und das ist eine
Empfehlung über die **Kosten** und nicht über die Sache. Der sichtbare
Unterschied ist ein einziger: ob eine Auswahl im Verlauf einen Tabwechsel
übersteht. Keine Zeile des Specs verlangt das, und Möglichkeit 2 kauft es mit
einem zweiten Melder und einer Ausnahme von einer Zusage, die `tabs.rs` gerade
erst geschrieben hat.

Fällt die Wahl auf 1, gehören zwei Aufräumschritte dazu: das Feld `auswahl` und
seine drei Leser fallen aus dem `Gitmodell`, und mit ihnen das
`expect(dead_code)` und die Probe `eine_auswahl_jenseits_des_verlaufs_bleibt_leer`,
deren Gegenstand dann in `crates/krk-ui/src/appkit/git.rs` steht und dort schon
eine Probe hat.

---
Answered: shared/history/260830-0950-orchestrator-session.md:125 — Möglichkeit 2: die Auswahl zieht in das `Gitmodell` und der Git-Bereich meldet sie nach oben; sie übersteht damit den Tabwechsel. Der zweite Melder und der schreibende Zugang zum Gitmodell des sichtbaren Tabs sind angenommen, Möglichkeit 3 ist wegen der zwei Schreiber auf einem Feld verworfen.

Implemented: `crates/krk-ui/src/appkit/git.rs` — das `Gitfenster` hält keine Auswahl mehr; `GitfensterIvars::auswahl` ist gefallen, `zeigen` nimmt sie aus `Gitmodell::auswahl` und `auswahl_uebernehmen` (der eine Weg für Pfeil und Mausklick) meldet sie über den neuen `Auswahlmelder` nach oben. Der Anwendungsdelegierte trägt sie über `DateifensterQuelle::gitauswahl_setzen` → `Tabliste::gitauswahl_setzen` → `Tabinhalt::gitauswahl_setzen` in das Gitmodell des sichtbaren Tabs; die benannte Ausnahme von „Nur zu lesen" steht im Doc-Kommentar von `Tabinhalt::gitmodell` (`crates/krk-ui/src/tabs.rs`), wo die Zusage steht. Das `expect(dead_code)` in `crates/krk-ui/src/gitmodell.rs` ist entfernt, `ausgewaehlter_commit` gefallen. `zeigen` bekommt das Modell weiterhin lesend (Möglichkeit 3 verworfen); `haelt_die_auswahl` und seine drei Ansichtsproben sind weg, C4.2 und C4.6 halten die Proben am Modell.

---

## Nachtrag 260831-1755 — die zweite Aussage der Antwortzeile ist nicht gebaut worden

**Der Wortlaut oben bleibt unverändert, und der Marker bleibt `_i_`.** Dieser Datensatz ist eine Aufzeichnung: was hier stand, ist der Beleg dafür, wie die Frage gestellt und wie sie beantwortet war. Der Nachtrag berichtigt, er überschreibt nicht.

**Die Antwortzeile trägt zwei Aussagen, und nur die erste ist gebaut.** Die Heimat der Auswahl im `Gitmodell`, der Auswahlmelder und der schreibende Zugang des Anwendungsdelegierten stehen so im Baum, wie die `Implemented:`-Zeile es beschreibt. Der Nebensatz „sie übersteht damit den Tabwechsel" steht dort **nicht**: `Tabliste::waehlen` ruft für den verlassenen Tab `gitlauf_nachziehen_an`, und dessen dritte Zeile ist `self.tabs[stelle].gitmodell.zuruecksetzen()` — unbedingt und vor jeder Bedingungsprüfung. `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()` und nimmt Kopf, Verlauf, Zusammenfassung und die Auswahl mit. Die Auswahl fällt mit dem Tabwechsel; sie übersteht allein den Wechsel des **aktiven Dateifensters**, weil jede `Tabliste` ihr eigenes Gitmodell hält.

**Gefunden hat es die Durchsicht der Runde 23** (`260831-1444-coderev-git-bereich-runde-23.md`, Befund M5), und zwar an drei Prosastellen, die den Wortlaut dieser Antwortzeile als Begründung führten (`260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md`). Die Prosa an den Baum anzugleichen hätte eine Nutzerentscheidung stillschweigend zurückgenommen, den Baum anzugleichen wäre eine Verhaltensänderung gewesen; beides ist als eigene Frage vorgelegt worden.

**Der Nutzer hat am 260831-1755 den gebauten Zustand bestätigt** (`260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`, Möglichkeit 2): die Auswahl fällt mit dem Tabwechsel, kein Verhalten wird geändert, und die Prosa zieht nach. Wer diesen Datensatz heute als Grundlage liest, nimmt seine erste Aussage und nicht seine zweite.
