# Bleibt der Filtertext bei einem Ordnerwechsel stehen, wenn „Deep" aus ist?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/_t_circle.md` (`## Directive`, Abschnitt „Vorbelegungen, denen der Nutzer nicht widersprochen hat"); `crates/krk-ui/src/tabs.rs` (`Tabinhalt`, der Halter des Filters); `crates/krk-core/src/verzeichnis/sprungmarke.rs:80-88` (`zuruecksetzen`, heute bei jedem Ordnerwechsel gerufen)

---

## Question

Die Berichtigung der Directive vom 260814-1610 hat eine Vorbelegung überholt, ohne sie zu ersetzen. Die Vorbelegung lautet: der Filter gehört dem Tab und wird beim Ordnerwechsel geleert. Sie stammt aus der Zeit, in der die tiefe Suche eine flache Trefferliste war, aus der niemand navigierte. Das gewählte Modell ist ein anderes: der Nutzer steigt bei eingeschaltetem „Deep" in die gezeigten Ordner ein, und auf jeder Ebene gilt dieselbe Filterregel. Wörtlich hat er gesagt: „User kann normal hinnavigieren, nur die Pfade, die Treffer erhalten, werden nicht ausgefiltert." Ein Filter, der beim Einstieg fiele, machte den Satz falsch.

Für den eingeschalteten Zustand ist die Antwort damit gegeben: der Filtertext übersteht den Ordnerwechsel. Für den ausgeschalteten steht sie offen, und beide Antworten müssen zusammen eine Regel ergeben, die der Nutzer im Kopf behalten kann.

## Options

1. **Zwei Regeln: bei „Deep" aus geleert, bei „Deep" an stehengeblieben.**
   - Pro: die Vorbelegung des Nutzers bleibt für den Fall gültig, für den er sie gegeben hat, und die Ausnahme trägt genau so weit, wie das gewählte Modell sie erzwingt. Wer ohne „Deep" navigiert, sieht in jedem neuen Ordner den vollen Bestand und wird von keiner verkürzten Liste überrascht.
   - Kontra: der Filtertext überlebt einen Ordnerwechsel oder nicht, je nach Stand eines Ankreuzfeldes. Wer „Deep" mitten in einer Navigation abschaltet, verliert den Text beim nächsten Schritt.
2. **Eine Regel: der Filtertext übersteht jeden Ordnerwechsel.**
   - Pro: eine Regel statt zweier, und sie hängt an nichts. Der Filter gehört dem Tab, und der Tab behält ihn, bis der Nutzer ihn löscht.
   - Kontra: kehrt die Vorbelegung des Nutzers um. Wer filtert, in einen Ordner steigt und den Filter vergessen hat, hält den neuen Ordner für fast leer. Das trägt nur, solange die Statuszeile den stehenden Filtertext nennt.
3. **Eine Regel: der Filtertext wird bei jedem Ordnerwechsel geleert, auch bei „Deep" an.**
   - Pro: hält die Vorbelegung wörtlich.
   - Kontra: nimmt dem gewählten Modell seinen Gegenstand. Der erste Einstieg in einen gezeigten Ordner zeigt dessen vollen Bestand, und der Nutzer müsste den Filter auf jeder Ebene neu tippen.

## Constraints

- Das gewählte Modell der tiefen Ansicht schließt Möglichkeit 3 aus. Sie steht hier, weil eine Aufzählung, die die wörtliche Vorbelegung wegließe, den Preis der beiden anderen verschwiege.
- Der Filter gehört dem Tab. Ein Tabwechsel zeigt den Filter des anderen Tabs, gleich welche Antwort fällt.
- Möglichkeit 2 hängt daran, dass der stehende Filtertext zu sehen ist. Die Statuszeile trägt ihn nach C4 dieses Specs; wo genau, hängt an `260814-1552_o_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`.
- Der Aufstieg zählt wie der Einstieg. Eine Regel, die nur für die eine Richtung gilt, entsteht nicht.

## Recommendation

Möglichkeit 1. Sie ändert an keiner Aussage des Nutzers etwas und führt die Ausnahme genau dort ein, wo seine eigene Berichtigung sie erzwingt. Möglichkeit 2 ist die sauberere Regel und bleibt jederzeit erreichbar: aus einer Ausnahme später die allgemeine Regel zu machen kostet eine Zeile im Spec, während der umgekehrte Weg das Modell der tiefen Ansicht bricht.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: `shared/history/260815-0912-orchestrator-session.md` — Möglichkeit 2, Nutzerentscheid vom 260815-0955. Eine Regel für beide Zustände des Kennzeichens: der Filtertext übersteht jeden Ordnerwechsel und fällt erst mit `Esc`. Die Empfehlung des Datensatzes (Möglichkeit 1, zwei Regeln) ist damit verworfen. Anlass war der Bugreport des Nutzers, der das Leeren beim Wechsel als Fehlverhalten meldete: „besser stehen lassen bis escape, dann kann der nutzer den filter zu suchen nach einer datei auch ohne deep=true nutzen". Die Bedingung des Abschnitts `## Constraints`, dass der stehende Filtertext zu sehen sein muss, ist vor der Antwort am Baum geprüft und erfüllt (`crates/krk-ui/src/appkit/statuszeile.rs:369-386`, `filterstand_text` schreibt `Filter „rs": 3 von 47 angezeigt`).
