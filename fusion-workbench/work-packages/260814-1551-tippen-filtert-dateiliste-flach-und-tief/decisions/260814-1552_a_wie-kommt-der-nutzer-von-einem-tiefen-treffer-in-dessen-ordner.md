# Wie kommt der Nutzer von einem tiefen Treffer in dessen Ordner?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/angezeigtedatei.rs:1-58` (`welche`, die Quelle des Ordnersprungs); `crates/krk-core/src/tasten/belegung.rs:866-880` (`Kommando::OrdnerDerDatei`); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` (die Runde, die den Ordnersprung gebaut hat)

---

## Question

Eine tiefe Trefferliste zeigt Dateien aus vielen Unterordnern und filtert die Ordner selbst heraus. Der Nutzer findet darin eine Datei und will als nächstes dorthin, wo sie liegt, oder mit ihrem Ordner arbeiten. Heute gibt es dafür keinen Weg von der ausgewählten Zeile aus. Der Ordnersprung aus der Runde 6 fragt `angezeigtedatei::welche`, und das kennt genau zwei Quellen: die Datei der sichtbaren Vorschau und die des sichtbaren Editors. Die ausgewählte Zeile der Dateiliste ist keine seiner vier Eingaben. Der Einstieg mit dem Rechts-Pfeil trägt in der tiefen Liste ebenfalls nichts, weil dort keine Ordner stehen. Ohne eine Antwort ist die tiefe Suche eine Liste, aus der der Nutzer nicht herauskommt, außer über den Umweg Vorschau einblenden und dann springen.

## Options

1. **Der Ordnersprung bekommt eine dritte Quelle.** Steht der Fokus im Dateifenster, ist die angezeigte Datei die ausgewählte Zeile.
   - Pro: ein vorhandener Befehl deckt einen Fall mehr ab; kein neuer Befehl, keine neue Taste, kein neuer Eintrag im Hauptmenü.
   - Kontra: `welche` ist heute eine vollständige, überschneidungsfreie Fallunterscheidung über vier Eingaben, und ihr Modulkopf begründet ausdrücklich, warum die Sichtbarkeit und nicht das Halten entscheidet. Eine dritte Quelle mit einer fünften Eingabe muss diese Begründung mittragen und darf nicht zwei Antworten auf eine Frage zulassen.
2. **Der Einstieg wechselt in der tiefen Liste seine Bedeutung.** Der Rechts-Pfeil geht auf einem Treffer in dessen Ordner und leert dabei den Filter.
   - Pro: die vertraute Taste an der Stelle, an der der Nutzer sie sucht; das Verlassen der Trefferliste und das Ankommen im Ordner sind ein Griff.
   - Kontra: ein Befehl, dessen Bedeutung vom Zustand abhängt. Die Belegungsansicht und das Hauptmenü zeigen je Befehl eine Bedeutung, und eine zweite passt dort nicht hinein.
3. **Ein eigener Befehl für die Trefferliste**, der in den Ordner des ausgewählten Treffers wechselt und den Filter leert.
   - Pro: eine Bedeutung je Befehl, sauber in Belegungsansicht und Hauptmenü zu führen; der Ordnersprung bleibt unangetastet.
   - Kontra: die 84. Zeile in einer Belegung mit heute 83 Einträgen, für einen Fall, der bereits einen fast passenden Befehl hat.
4. **Kein eigener Weg.** Der Nutzer blendet die Vorschau ein und benutzt den vorhandenen Ordnersprung.
   - Pro: nichts kommt hinzu.
   - Kontra: zwei Griffe für den häufigsten nächsten Schritt nach einem Treffer, und einer davon ist das Einblenden eines Bereichs, den der Nutzer für diesen Zweck nicht braucht.

## Constraints

- `angezeigtedatei::welche` bleibt die eine Rechnung für den Begriff „die angezeigte Datei". Zwei Rechnungen nebeneinander wären zwei Antworten auf eine Frage, und die zweite fiele erst am Bündel auf.
- Jeder neue Befehl braucht eine Zeile in `Kommando::wirkungsbereich` und in `bereich_des_kommandos`, einen Eintrag in `resources/default-keymap.toml` und einen Platz im vollständigen Hauptmenü der Runde 7.
- Was mit dem Filtertext geschieht, wenn der Nutzer den Ordner wechselt, ist bereits entschieden: er wird geleert. Jede Antwort hier muss dazu passen.

## Recommendation

Möglichkeit 1, sofern die Fallunterscheidung in `welche` dabei vollständig und überschneidungsfrei bleibt. Der Begriff, den der Nutzer im Kopf hat, ist in allen drei Fällen derselbe: der Ordner der Datei, um die es gerade geht. Ein zweiter Befehl für denselben Begriff wäre die zweite Tür, die dieses Projekt an anderer Stelle bereits abgelehnt hat. Fällt die Prüfung anders aus, weil die fünfte Eingabe die Überschneidungsfreiheit bricht, ist Möglichkeit 3 die richtige und nicht Möglichkeit 2.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1610 — die Frage loest sich mit der Bauform der tiefen Ansicht auf. Der Nutzer navigiert bei eingeschaltetem "Deep" gewoehnlich in die Ordner hinein; ausgefiltert werden allein die Ordner, unter denen kein Treffer liegt. Es gibt damit keine Trefferzeile ausserhalb ihres Ordners, aus der heraus ein Ordnersprung noetig waere, und `angezeigtedatei::welche` bekommt keine dritte Quelle. Woertlich: "User kann normal hinnavigieren, nur die Pfade, die Treffer erhalten, werden nicht ausgefiltert."
