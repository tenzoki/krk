# Heißt die Geheimnisdatei `secrets.txt` ohne Punkt?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_was-heisst-immer-gelistet-fuer-secrets-txt.md, 260926-0007_*_wann-entsteht-secrets-txt-und-was-geschieht-mit-fehlenden-dateien.md, 260926-0050_*_wie-weit-reicht-der-inhaltsfilter-liest-secrets-txt-nicht-wenn-das-kennzeichen-versteckt-ihn-nicht-haelt.md

---

## Question

Die Directive nannte `.secrets.txt`, eine „unsichtbare“ Datei, die im Heimordner trotzdem immer gelistet ist. Bei der ersten Prüfung am Bündel stand sie nicht in der Liste. Der Nutzer stellt fest, dass das Verstecken bei einer verschlüsselten Datei nichts schützt.

## Options

1. **`secrets.txt` ohne Punkt.** Die Datei ist kein versteckter Eintrag, steht in jeder Liste wie jede andere, und die Ausnahme „steht immer“ entfällt. Die übrigen Zusagen bleiben: verschlüsselt, Vorschau nur mit Hinweis, nie in der Sitzung, im Heimordner kein Inhaltsauftrag. Eine vorhandene `.secrets.txt` wird von F2 zu `secrets.txt`, wenn es diese noch nicht gibt.
2. **Beim Namen `.secrets.txt` bleiben** und die Ausnahme reparieren.

## Constraints

- Kein verschlüsselter Inhalt geht beim Umbenennen verloren; eine vorhandene `secrets.txt` wird nie überschrieben.

## Recommendation

Möglichkeit 1.

---
Answered: Nutzeraussage vom 260926 („Wir benennen sie in secrets.txt um“) — Möglichkeit 1; das Umbenennen einer vorhandenen `.secrets.txt` und das Beibehalten der Inhaltsfilter-Ausnahme hat der Orchestrator im autonomen Modus ergänzt; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 24375c5 — secrets.txt ohne Punkt, Ausnahme im Inhaltszweig, Umbenennen einer alten .secrets.txt ohne Ersetzen; Doku in 2841526
