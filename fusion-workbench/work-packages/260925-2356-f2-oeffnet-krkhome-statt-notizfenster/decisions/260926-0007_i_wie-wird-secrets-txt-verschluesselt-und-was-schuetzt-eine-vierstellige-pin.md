# Wie wird .secrets.txt verschlüsselt, und was soll eine vierstellige PIN schützen?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_wie-lange-gilt-eine-eingegebene-pin.md, 260820-2242-lesezeichenverlust-nach-installation.md

---

## Question

Die Directive verlangt, dass beim Anlegen von `.secrets.txt` eine vierstellige PIN „zur Verschlüsselung" abgefragt wird und dass das Bearbeiten die PIN verlangt. Eine vierstellige PIN hat genau 10 000 mögliche Werte. **Wer eine Kopie der Datei hat, kann alle 10 000 durchprobieren, und KRK kann das nicht verhindern**: eine Sperre nach drei Fehlversuchen wirkt nur in KRK selbst, nicht gegen ein Programm, das die kopierte Datei direkt angreift. Auch eine absichtlich langsame Schlüsselableitung verschiebt nur die Dauer. Inferenz, nicht gemessen: bei einer halben Sekunde je Versuch auf einem Kern dauert der vollständige Durchlauf rund 83 Minuten, auf einem Rechner mit vielen Kernen entsprechend weniger. Die Frage ist deshalb zuerst, **wovor** die Datei schützen soll, und erst danach, wie.

Wogegen eine PIN in jeder Möglichkeit unten hilft: gegen den Blick über die Schulter, gegen die Anzeige in der Vorschau, in Quick Look und in Spotlight, gegen `grep` und den Inhaltsfilter von KRK, gegen versehentliches Teilen oder Hochladen der Datei, gegen eine Kopie in einem Cloud-Speicher, die jemand ohne Absicht öffnet. Wogegen sie in keiner Möglichkeit hilft: gegen jemanden, der an der entsperrten Sitzung des Nutzers sitzt, während die Datei in KRK offen ist; gegen Schadsoftware unter dem Konto des Nutzers; gegen den Klartext in der Zwischenablage, nachdem der Nutzer ein Geheimnis kopiert hat.

**In jeder Möglichkeit gilt: eine vergessene PIN heißt, der Inhalt ist verloren.** Es gibt keinen Weg zurück, und KRK kann keinen anbieten, ohne die Verschlüsselung selbst aufzuweichen.

## Options

1. **Nur die PIN, mit langsamer Schlüsselableitung; die Datei trägt alles, was zum Entschlüsseln nötig ist außer der PIN.**
   - Pros: die Datei ist in sich geschlossen. Sie überlebt ein Kopieren auf einen anderen Mac, eine Sicherung, eine Neuinstallation und ein Aufräumwerkzeug, das KRKs Stützdateien löscht. Das entspricht wörtlich der Directive.
   - Cons: gegen jemanden, der die Datei kopiert und gezielt angreift, schützt sie nur für Minuten bis Stunden. Für echte Zugangsdaten (Kennwörter, Schlüssel) ist das nicht ausreichend, und der Spec muss es dem Nutzer an der PIN-Abfrage sagen.
2. **PIN plus ein zufälliger Schlüssel im Schlüsselbund von macOS.** Der eigentliche Schlüssel entsteht aus der PIN und einem Geheimnis, das KRK beim Anlegen im Schlüsselbund ablegt.
   - Pros: eine kopierte Datei allein ist ohne den Schlüsselbundeintrag wertlos, auch mit allen 10 000 Versuchen. Angreifbar bleibt sie nur unter dem entsperrten Konto des Nutzers.
   - Cons: die Datei hängt an diesem Schlüsselbund. Auf einem anderen Mac ohne übertragenen Schlüsselbund, nach einem Zurücksetzen des Schlüsselbunds oder nach einem Aufräumwerkzeug, das auch Schlüsselbundeinträge der App entfernt, ist der Inhalt verloren, obwohl der Nutzer die PIN kennt. Das Projekt hat am 17.08. genau diesen Verlusttyp erlebt (Analyse zum Lesezeichenverlust, siehe Querverweis). Eine Sicherung der Datei allein sichert den Inhalt nicht mehr.
3. **Ein Kennwort beliebiger Länge statt der festen vierstelligen PIN; vier Ziffern bleiben erlaubt.** Sonst wie Möglichkeit 1.
   - Pros: in sich geschlossen wie Möglichkeit 1, und die Stärke wählt der Nutzer: wer vier Ziffern tippt, bekommt den Schutz von Möglichkeit 1, wer ein langes Kennwort wählt, bekommt einen Schutz, der auch einem gezielten Angriff standhält. Die Abfrage kann ehrlich sagen, was eine kurze Eingabe schützt.
   - Cons: weicht von der Directive ab, die ausdrücklich „4-stellige PIN" sagt. Ein längeres Kennwort ist langsamer einzugeben, und bei jedem Öffnen zum Bearbeiten fällt das ins Gewicht.

## Constraints

- Das Verfahren selbst (Algorithmus, Schlüsselableitung, Dateikopf) wählt der Planer; es muss ein anerkanntes Verfahren mit Authentifizierung sein, damit eine manipulierte oder beschädigte Datei als solche erkannt und nicht als Unsinn angezeigt wird.
- Die Zusage des Projekts, dass auf den beiden Mac-Zielen kein C-Code gebaut wird (`CLAUDE.md`, Absatz zu `syntect`, `two-face`, `zip` und `gix`), gilt auch für eine Kryptografie-Kiste.
- An keiner Stelle darf Klartext der Geheimnisse auf die Platte gelangen: nicht in `session.toml`, nicht in einer Sicherungskopie, nicht beim Beiseitelegen einer beschädigten Datei.
- Eine falsche PIN darf den Inhalt nicht zerstören und keine halb entschlüsselte Anzeige erzeugen.

## Recommendation

Wir empfehlen Möglichkeit 3, vorbehaltlich dessen, wofür der Nutzer die Datei tatsächlich nutzen will. Sie hält die Datei in sich geschlossen und damit sicher gegen den Verlusttyp, den dieses Projekt schon einmal erlitten hat, und sie lässt die vier Ziffern zu, wo sie genügen. Soll die Datei nur Dinge vor dem beiläufigen Blick verbergen, genügt Möglichkeit 1 mit einem ehrlichen Hinweis an der Abfrage. Möglichkeit 2 empfehlen wir nicht: sie tauscht einen Angriff, den der Nutzer selten erlebt, gegen einen Datenverlust, den er beim nächsten Mac-Wechsel erleben kann.

---
Answered: 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1, die vierstellige PIN, XChaCha20-Poly1305 mit Argon2id, binäre Datei mit versioniertem Kopf, eine Meldung für falsche PIN oder veränderte Datei; Bedrohungsmodell des Nutzers: allein das versehentliche Lesen durch Agenten, Werkzeuge und Indexer, ein Angriff mit einer Kopie der Datei liegt außerhalb; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 916412d — vierstellige PIN, XChaCha20-Poly1305 mit Argon2id, versionierter Kopf; Laden und Sichern in 9e40d5c
