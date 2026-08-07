# Planner-Sitzung 260807-0930: vier Antworten, drei Nachzüge, ein Defekt

**Status:** Complete
**Agent:** planner
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Auftrag:** R2b aus dem Orchestrator-Turn 26. Dokumentenpflege an Spec, Plan und
vier Entscheidungsdatensätzen. Kein Code, keine `.toml`, keine neue
Planungsdatei.

## Was getan wurde

**Vier Entscheidungsdatensätze sind auf `_a_` gezogen**, jeder mit einem
Abschnitt, der die Antwort und ihre Begründung trägt, und mit einer
`Answered:`-Zeile auf die Stelle im Spec.

| Datensatz | Antwort | `Answered:` zeigt auf |
|---|---|---|
| `260805-2216_a_tastenweg-des-fokus-in-das-vorschaufenster.md` | Möglichkeit 1, Fokusbefehl auf `shift+cmd+y` | Spec C6, Zeile 319 |
| `260805-1730_a_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md` | Möglichkeit 2, einblenden und Fokus setzen | Spec C5, Zeile 296 |
| `260805-2252_a_entfernen-einer-einzelnen-kombination-in-der-belegungsansicht.md` | Möglichkeit 1, so lassen | Spec C3, Zeile 234 |
| `260805-1845_a_wann-eine-von-hand-geaenderte-settings-toml-wirkt.md` | Möglichkeit 1, einmaliges Laden, gegen die Empfehlung | Spec C11, Zeile 478 |

Bei den beiden Antworten, die den gebauten Zustand stehen lassen, steht im
Datensatz, **warum** so entschieden wurde und was es kostet. Beim
Entfernen-Befehl ist der Preis, dass beim Zurücksetzen jede übrige Anpassung
mitfällt und mit der Zeit mehr zu verlieren ist. Bei der `settings.toml` ist es
die Fehlmeldung, die nach der Berichtigung ein zweites Mal erscheint; dass der
Nutzer dort gegen die Empfehlung des Datensatzes entschieden hat, steht
ausdrücklich in beiden Dokumenten.

Keiner der vier trägt `_i_`; das setzt der Orchestrator, wenn die Commits
stehen.

## Die Prüfung der drei Nachzüge

**Erstens, das vierte Abnahmekriterium von C2: der Satz ist falsch, nicht bloß
unvollständig, und er ist in beiden Richtungen falsch.** Er lautete "Ein
Tastenbefehl wirkt dann und nur dann, wenn der Eingabefokus in einem
Dateifenster oder in der Lesezeichenleiste steht". Die Formel "dann und nur
dann" behauptet über jeden Tastenbefehl zweierlei. Die Hälfte "nur dann"
bricht seit S19: `Kommando::TabNeu` und die drei übrigen Tabbefehle tragen
`Wirkungsbereich::Tabbereich`, und `wirkt` in
`crates/krk-ui/src/kommandos/fokus.rs` lässt sie bei `Fokus::Vorschau`
durch, also außerhalb der beiden genannten Bereiche. Die Hälfte "dann" bricht
schon seit S18: `Kommando::LesezeichenLoeschen` trägt
`Wirkungsbereich::Leiste` und wirkt bei Fokus im Dateifenster nicht. Das
vierte Abnahmekriterium von C5 schreibt genau diesen zweiten Fall selbst aus,
womit der Spec sich im eigenen Dokument widersprach. Berichtigt ist beides in
einem Zug; das Kriterium zählt jetzt keine Bereiche mehr auf, sondern nennt die
Regel, aus der sie folgen, und wird deshalb mit einem vierten fokussierbaren
Bereich nicht ein zweites Mal falsch.

**Zweitens, C5 schwieg zum ausgeblendeten Fall.** Bestätigt. Das vierte
Abnahmekriterium sagt den Fokusbefehl zu und sagt nichts über den Zustand, den
C7 mit `opt+cmd+l` herstellen kann. Nachgezogen sind das Kriterium und die
Festlegung.

**Drittens, die Asymmetrie des Einblendens.** Bestätigt, und am Code
nachgesehen: `Fenstermodell::einblenden` ist die eine Stelle, und sie hat genau
zwei Aufrufer im Programm, `zwischenablage_ansehen` und `fokus_holen`. Damit
sind es drei Befehle außerhalb von C7, `shift+f3`, `shift+cmd+l` und
`shift+cmd+y`, und keiner blendet je aus. C7 trägt die Regel jetzt
ausdrücklich. Der Rückweg zum Fenster auf `cmd+n` hat dieselbe Form, gehört
aber C7 selbst und ist deshalb als verwandter Fall genannt und nicht als
vierter.

## Zwei neue Planschritte

Der dritte Fokusbefehl ist neuer Umfang und hatte keinen Schritt. Nach der
Praxis dieses Plans, die für jeden Nachtrag einen Buchstabenschritt anlegt,
sind **S19b** (`coder`, Kommando und Hervorholen) und **S19c** (`ontocoder`,
der Eintrag in `resources/default-keymap.toml`) entstanden, beide
`[IN PROGRESS]`. Der Abhängigkeitsgraph ist mitgezogen: vier Kanten dazu,
nachgerechnet 38 Knoten und 60 Kanten, zyklenfrei, Eingangsgrad 9 bei S23,
jede Kante von der kleineren zur größeren Schrittnummer. Die Zahlen sind
maschinell aus dem Mermaid-Block nachgezählt und nicht geschätzt.

## Ein Defekt

`issues/260807-0930_o_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md`
trägt den Vorschlag, dass die Meldung den Ladezeitpunkt selbst nennt, samt dem,
was dagegen spricht. Er ist nicht entschieden und steht deshalb unter
`issues/` und nicht im Spec.

## Grenzen eingehalten

Kein Code, keine `.toml`, keine neue Planungsdatei. Die Marker von Spec und
Plan bleiben `_o_`. S18, S18c, S19 und S20 bleiben `[DONE]` und sind in ihrem
Wortlaut unverändert; geändert ist allein die Notiz am Fuß jedes der vier
Schritte. Nicht committet.
