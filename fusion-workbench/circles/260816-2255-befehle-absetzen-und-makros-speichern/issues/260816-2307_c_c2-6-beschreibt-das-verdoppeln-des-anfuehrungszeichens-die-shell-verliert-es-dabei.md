C2.6 beschreibt das Verdoppeln des Anführungszeichens; die Shell verliert es dabei
---
Der Spec `shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md` legt in C2.6 fest: „Jeder eingesetzte Wert wird in Einzelanführung gesetzt, und ein Anführungszeichen im Wert wird verdoppelt." Die Verdopplung ist die Regel von TOML und SQL. Die Shell kennt sie nicht: innerhalb einfacher Anführungszeichen gibt es dort kein Fluchtzeichen, und zwei aufeinanderfolgende Anführungszeichen schließen den einen Abschnitt und öffnen den nächsten. Der Wert verliert das Zeichen still.

Am 260816 auf diesem Gerät nachgemessen:

    $ sh -c "printf '%s|' 'it''s'"
    its|
    $ sh -c "printf '%s|' 'it'\''s'"
    it's|

Die erste Zeile ist die Regel, die C2.6 im Wortlaut beschreibt, und sie liefert `its`. Die zweite ist die Regel, die C2.6 in seinem Abnahmekriterium **verlangt**, und sie liefert `it's`.
---
Der Datensatz hält einen Widerspruch innerhalb desselben Kriteriums fest und keine Meinungsverschiedenheit mit dem Spec. C2.6 nennt zwei Dinge: eine Beschreibung der Regel und den Nachweis, den sie führen muss. Der Nachweis lautet „ein Dateiname mit Leerzeichen, mit `'`, mit `$`, mit einem Rückwärtsstrich und mit einem Zeilenumbruch kommt bei `printf '%s\n'` als genau ein Wert an", und er ist die entscheidbare Frage, die der Spec dem unentscheidbaren Vorhersagen des Befehlsinhalts ausdrücklich entgegenstellt (`## Nicht Gegenstand dieser Runde`, vierter Punkt). Die Beschreibung besteht den eigenen Nachweis nicht.

Der Plan `circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_o_plan-befehle-absetzen-und-makros-speichern.md` setzt in Schritt B4 die Regel um, die den Nachweis besteht: der Wert wird in einfache Anführungszeichen gesetzt, und jedes Anführungszeichen im Wert wird durch die Folge Anführungszeichen, Rückwärtsstrich, Anführungszeichen, Anführungszeichen ersetzt. Vier Zeichen statt zwei.

Zu tun bleibt eine Berichtigung des Satzes in C2.6, damit die Beschreibung und der Nachweis dasselbe sagen. Der Nachweis bleibt unverändert; er war von Anfang an richtig.

---
Resolved: 260906 — der Satz in C2.6 ist berichtigt, und der Nachweis steht unverändert daneben.

**Neuer Wortlaut** (`260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md`, C2.6): „Jeder eingesetzte Wert wird in Einzelanführung gesetzt, und ein Anführungszeichen im Wert wird durch die vier Zeichen `'\''` ersetzt — die Anführung schließen, das Zeichen mit einem Rückwärtsstrich schützen, die Anführung wieder öffnen."

**Eine zweite Stelle sagte dasselbe und ist mitgezogen.** C3.7 lautete „einzeln angeführt mit verdoppeltem Anführungszeichen" und verweist jetzt auf die Vierzeichenfolge aus C2.6. Dieser Datensatz nannte sie nicht; ohne sie stünden nach der Berichtigung zwei Regeln im selben Spec.

Der Spec trägt am Ende einen datierten Vermerk über beide Änderungen. Die Beschreibung sagt damit dasselbe wie ihr eigener Nachweis und dasselbe wie Schritt B4 des Plans, der die Regel schon richtig ausschreibt.

**Warum hier berichtigt und nicht vermerkt.** Die Runde ist zurückgestellt und nicht abgeschlossen: ihr Spec hat nie einen Bau geleitet und nie einen Abnahmelauf getragen, er ist Grundlage für einen künftigen. Die Frage, ob der Spec einer **geschlossenen** Runde nachträglich angefasst werden darf, ist davon unberührt und liegt als `260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md` beim Nutzer.
