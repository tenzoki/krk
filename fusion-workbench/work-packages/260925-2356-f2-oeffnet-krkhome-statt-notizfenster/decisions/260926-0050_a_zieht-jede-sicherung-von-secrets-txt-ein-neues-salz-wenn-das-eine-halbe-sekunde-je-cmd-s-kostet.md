# Zieht jede Sicherung von .secrets.txt ein neues Salz, wenn das eine halbe Sekunde je cmd+s kostet?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md, 260926-0017-zweitlesung-spec-f2-krkhome.md, 260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md

---

## Question

C7 des Spec verlangt zwei Dinge, die zusammen einen Preis haben, den der Spec nicht nennt. Das erste: „Jede Sicherung zieht Salz und Nonce neu.“ Das zweite: der Schlüssel entsteht mit Argon2id aus der PIN, mit Parametern, die auf dem Referenzgerät etwa eine halbe Sekunde je Ableitung kosten. Ein neues Salz ist ein neuer Schlüssel, und ein neuer Schlüssel ist eine neue Ableitung. Wörtlich umgesetzt kostet damit **jedes `cmd+s` an `.secrets.txt` eine halbe Sekunde**, und dazu jede Sicherung aus der Rückfrage beim Schließen und beim Beenden.

Heute sichert der Editor auf dem Hauptfaden (`Editormodell::sichern` in `crates/krk-ui/src/editormodell.rs`, gerufen aus `Editorbereich::sichern` in `crates/krk-ui/src/appkit/editor.rs`). Eine halbe Sekunde dort friert das Fenster ein. Die Frage muss vor Stufe 5 beantwortet sein, weil sie bestimmt, was der Editor zwischen Öffnen und Schließen von `.secrets.txt` im Speicher hält und ob der Sicherungsweg einen Arbeitsfaden bekommt.

Die Zweitlesung, aus der das Kriterium stammt, begründet das neue Salz nicht eigens. Sie nennt als Zweck des versionierten Kopfes, dass sich die Parameter der Ableitung später anheben lassen, ohne alte Dateien zu verlieren; dafür genügt, dass jede Datei ihr Salz und ihre Parameter im Kopf trägt.

## Options

1. **Wörtlich: jede Sicherung zieht Salz und Nonce neu, auf dem Hauptfaden.** Der Editor hält die PIN, und `sichern` leitet jedes Mal ab.
   - Pros: das Kriterium steht, wie es geschrieben ist; der Sicherungsweg bleibt, wie er ist.
   - Cons: jedes `cmd+s` friert KRK für etwa eine halbe Sekunde ein, gemessen am Referenzgerät. Das widerspricht der Maxime „superschnell“ an der Stelle, an der der Nutzer am häufigsten drückt.
2. **Wörtlich, aber die Sicherung von `.secrets.txt` läuft auf einem Arbeitsfaden.** Der Hauptfaden bleibt frei, die Sicherung meldet ihr Ende nachträglich.
   - Pros: das Kriterium steht, und das Fenster friert nicht ein.
   - Cons: der Editor bekommt einen zweiten Sicherungsweg mit eigenem Zwischenzustand („wird gesichert“). Die Abweichungsmarke, die Rückfrage beim Schließen und beim Beenden und `applicationWillTerminate:` müssen auf das Ende einer laufenden Sicherung warten oder sie abbrechen. Das ist für eine einzige Datei ein Mechanismus, den keine andere Datei braucht.
3. **Das Salz entsteht, wenn die PIN festgelegt oder geändert wird; jede Sicherung zieht eine neue Nonce.** Beim Öffnen leitet der Editor den Schlüssel einmal aus der PIN und dem Salz im Kopf ab, auf dem Arbeitsfaden, auf dem er ohnehin lädt. Er hält den abgeleiteten Schlüssel, solange die Datei offen ist, und sichert ohne neue Ableitung. Beim Ändern der PIN entstehen neues Salz und neuer Schlüssel, und dabei greifen auch angehobene Parameter.
   - Pros: `cmd+s` kostet so viel wie bei jeder anderen Datei. Die 24 Bytes der Nonce von XChaCha20 sind zufällig je Sicherung und brauchen keine Zählerverwaltung; ein Schlüssel über viele Sicherungen ist für dieses Verfahren vorgesehen. Die halbe Sekunde fällt einmal beim Öffnen an, auf dem Arbeitsfaden.
   - Cons: das Kriterium „jede Sicherung zieht Salz und Nonce neu“ gilt nur für die Nonce und wird entsprechend umformuliert. Der Speicher hält statt der PIN den abgeleiteten Schlüssel. Das ist nach dem Spec kein neuer Verlust: er hält ohnehin den Klartext, und der Spec sagt kein Tilgen zu.

## Constraints

- Klartext gelangt an keine Stelle auf der Platte (C7); das gilt in jeder Möglichkeit.
- Die Parameter der Ableitung stehen im Kopf der Datei und nicht im Code, damit eine Datei mit älteren Parametern weiter öffnet (C7).
- Auf dem Referenzgerät (`MacBookPro15,1`, `260802-1036_*_leistungszusagen-navigator.md`) wird die Ableitung auf etwa eine halbe Sekunde eingestellt; gemessen, nicht geschätzt.

## Recommendation

Wir empfehlen Möglichkeit 3. Sie hält jede Sicherung so schnell wie bei jeder anderen Datei und braucht keinen zweiten Sicherungsweg, und sie verliert gegenüber dem Wortlaut nichts, was das Bedrohungsmodell des Nutzers verlangt: das versehentliche Lesen durch Agenten und Werkzeuge wehrt die Verschlüsselung in allen drei Möglichkeiten gleich ab. Möglichkeit 2 kauft den Wortlaut mit einem Zustand, der die Rückfrage beim Schließen und beim Beenden berührt, also genau die Wege, auf denen nach der Zusage des Projekts nichts Getipptes ohne Meldung verschwinden darf. Der Plan baut Stufe 5 auf Möglichkeit 3 und benennt an Schritt 5.4, was sich bei Möglichkeit 1 oder 2 ändert.

---
Answered: dieser Datensatz `## Options` und 260926-0107-zweitlesung-plan-f2-krkhome.md — Möglichkeit 3, neues Salz nur beim Festlegen oder Ändern der PIN, neue Nonce je Sicherung, der abgeleitete Schlüssel wird gehalten, solange die Datei offen ist; ruled by user, Kai Stalmann <kai@stalmann.org>
