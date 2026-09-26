# Bekommt krkhome ein eigenes Menü und einen einstellbaren Ort?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_ist-der-ort-krkhome-fest-oder-einstellbar.md, 260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md, 260926-1119_*_bekommt-secrets-txt-unter-einer-dritten-schreibweise-eine-zusatzpruefung-und-gilt-ziehen-als-kopieren.md

---

## Question

Nach der ersten Prüfung am Bündel verlangt der Nutzer zweierlei: die Funktionen rund um krkhome in einem eigenen Menü, damit man sie findet, und einen einstellbaren Ort, Vorgabe das Benutzerverzeichnis, damit der Ordner etwa in einem geteilten Verzeichnis (Dropbox) liegen kann. Der Entscheid `260926-0007_*_ist-der-ort-krkhome-fest-oder-einstellbar.md` hatte die Einstellbarkeit als spätere Arbeit vermerkt.

## Options

1. **Menüname:** „Home“ / „Notizen“ / „krkhome“.
2. **Einstellen:** Befehl „Ort wählen…“ im neuen Menü mit Ordnerdialog, der den Pfad in `settings.toml` schreibt, von Hand dort ebenso änderbar / allein ein Schlüssel in `settings.toml`.
3. **Umzug beim Ortswechsel:** nichts verschieben, fehlende Dateien entstehen am neuen Ort leer, die Statuszeile nennt den alten Ort / KRK bietet den Umzug der drei Dateien an.

## Constraints

- Die Erkennung des Heimordners bleibt ein Textvergleich ohne Systemaufruf auf den heißen Wegen; die Zusatzprüfung für `secrets.txt` über Gerät und Inode bleibt.
- Eine vorhandene Datei wird nie überschrieben; beim Start wird nichts angelegt.

## Recommendation

Menü „Home“; „Ort wählen…“ mit Ordnerdialog und Schlüssel in `settings.toml`; nichts verschieben.

---
Answered: dieser Datensatz `## Recommendation` — Menü „Home“ mit Notizordner öffnen, den sechs Eintragsbefehlen und PIN ändern; der Ort wird über „Ort wählen…“ (Ordnerdialog) oder von Hand in `settings.toml` gesetzt, Vorgabe `~/krkhome`; beim Ortswechsel wird nichts verschoben, fehlende Dateien entstehen am neuen Ort leer und die Statuszeile nennt den alten Ort; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: c87312b (Menü Home), 00b9c77 und f3587cb (Ort aus settings.toml), 59985ed und 95e36cd (Ort wählen)
