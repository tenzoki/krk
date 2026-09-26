# Bekommt `.secrets.txt` unter einer dritten Schreibweise eine Zusatzprüfung, und gilt Ziehen in den Finder als Kopieren?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md, 260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md, 260926-1051_*_eine-leere-secrets-txt-unter-einer-dritten-schreibweise-des-heimordners-geht-ueber-den-klartextweg.md, 260926-1011_*_text-aus-secrets-txt-auf-den-finder-gezogen-wird-eine-klartextdatei-und-die-zwischenablage-entscheidung-sagt-dazu-nichts.md, 260926-1047-schlussdurchsicht-f2-krkhome.md

---

## Question

Die Schlussdurchsicht hat zwei Wege gefunden, auf denen Klartext der Geheimnisse entstehen kann, die keiner der beantworteten Entscheide abdeckt: (a) unter einer dritten Schreibweise des Heimordners öffnet eine leere `.secrets.txt` als gewöhnlicher Text, weil die Erkennung an zwei Pfadformen hängt; (b) Text, der aus `.secrets.txt` auf den Finder gezogen wird, wird dort eine Klartextdatei.

## Options

1. **(a) Zusatzprüfung:** für Dateien namens `.secrets.txt` prüft KRK beim Öffnen und beim Sichern zusätzlich Gerät und Inode gegen `~/krkhome/.secrets.txt`; die Erkennung im Übrigen bleibt bei zwei Pfadformen.
2. **(a) Hinnehmen:** die Lücke bleibt und steht in der Anleitung.
3. **(b) Wie Kopieren:** Ziehen bleibt erlaubt als bewusste Handlung des Nutzers, die Anleitung nennt die entstehende Klartextdatei.
4. **(b) Sperren:** aus `.secrets.txt` lässt sich kein Text ziehen.

## Constraints

- Die Erkennung des Heimordners stellt auf den heißen Wegen keinen Systemaufruf (`260926-0115_*`); eine Prüfung am Deskriptor ist nur beim Öffnen und Sichern zulässig.

## Recommendation

Möglichkeit 1 für (a), Möglichkeit 3 für (b).

---
Answered: dieser Datensatz `## Recommendation` — (a) Zusatzprüfung über Gerät und Inode beim Öffnen und Sichern von Dateien namens `.secrets.txt`; (b) Ziehen gilt wie Kopieren, die Anleitung nennt die Klartextdatei; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: ffd0d7f — (a) Gerät und Inode beim Öffnen und Sichern; (b) HowTo.md nennt die Klartextdatei beim Ziehen in 86fe9a9
