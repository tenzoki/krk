# Ein Befehl setzt die Ablagedateien auf Werkseinstellungen zurück und liest sie neu ein

---
**Domain:** code
**Status:** claimed
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260929-1015
**Mode:** autonomous
**Active spec/plan:** 260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md (Spec), 260929-1025_*_plan-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"Wir brauchen einen Befehl unter Krk, der automatisch auf Werkseinstellungen zurücksetzt und die Werte neu einliest." Antworten auf die Klärungsfragen: alle drei von Hand gepflegten Dateien (readers.toml, settings.toml, keymap.toml), im laufenden Betrieb neu eingelesen, alte Dateien mit Zeitstempel beiseitegelegt, neue Dateien sind der Auslieferungsstand. Erreicht ist das, wenn der Befehl im Menü „KRK“ steht und nach Bestätigung die drei Dateien sichert, zurücksetzt und ohne Neustart wirksam macht, wie der Spec es in prüfbaren Kriterien festhält.
