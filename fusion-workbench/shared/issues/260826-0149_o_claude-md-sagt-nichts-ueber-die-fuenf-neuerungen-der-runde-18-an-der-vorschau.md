# `CLAUDE.md` sagt nichts über die fünf Neuerungen der Runde 18 an der Vorschau

---
**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `CLAUDE.md` (Rundentabelle Zeile 16, Abschnitt „Was man nicht sieht, wenn man es nicht weiß"); `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `README.md:44-73`; `resources/default-readers.toml`; `crates/krk-core/src/leseprofil/`; Commits `c0050bf`, `f097e0e`, `3cadb45`, `66c779c`, `9322d5d`, `5595026`, `d04e50f`

---

## Was ist

`CLAUDE.md` erwähnt die Profil-Zusammenfassung an genau einer Stelle, der Tabellenzeile zur
Runde 16: „die Vorschau zeigt für erkannte Orte eine Profil-Zusammenfassung statt der
Metadaten, `readers.toml` als siebte Ablagedatei". Die Runde 18 hat den Mechanismus an fünf
Stellen geändert, und keine davon steht in `CLAUDE.md`:

1. **Die Vorschau beschreibt ohne ausgewählte Zeile den angezeigten Ordner** und nicht mehr
   nichts (`crates/krk-ui/src/appkit/tabelle.rs`, `zu_beschreiben` und `nach_lesebeginn`). Der
   Auswahlmelder meldet seitdem immer einen Pfad und nie `None`; das ist eine Vertragsänderung
   und keine Ergänzung.
2. **Ein Ort wird je Zusammenfassung höchstens einmal gelesen.** Das kehrt eine begründete
   Festlegung der Runde 16 um, die die Asymmetrie zwischen erkanntem Ordner und Unterordner
   ausdrücklich gewollt hatte.
3. **Die Ortsangabe trägt einen Platzhalter** (`ordner = "*"`, `ordner = "*/issues"`), höchstens
   einen, und er greift allein Ordner. Damit weicht die Zahl der geöffneten Verzeichnisse
   erstmals von der Zahl der Leseläufe ab: das Profil `circles/` kostet drei Leseläufe und
   neununddreißig Verzeichnisöffnungen, gemessen in
   `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`.
4. **`juengste` kennt den Schlüssel `zeigt`** mit `titel` und `datum`; die Datumsform öffnet
   keine Datei und sieht Einträge jedes Typs.
5. **Die Auslieferungsfassung führt zwölf Profile** statt fünf, acht für fusion und vier für
   flight. Die Zahl steht an vier Stellen im Baum.

Dazu kommt eine sechste Aussage, die keine Codeänderung ist und für einen Leser dieses Projekts
schwerer wiegt als die fünf: **ein Nutzer, der KRK schon einmal gestartet hat, sieht von alledem
nichts**, bis er `~/Library/Application Support/KRK/readers.toml` beiseitelegt und KRK neu
startet. Der Weg steht in `README.md:44-73`. `CLAUDE.md` führt die Betriebsregel gegen den
Datenverlust beim Installieren, aber diese zweite Regel derselben Bauart nicht.

## Warum das zählt

Die fünf sind zusammen der Fall, den der Abschnitt „Was man nicht sieht, wenn man es nicht
weiß" für sich beansprucht: jede Änderung ist an ihrer Stelle richtig und für einen Leser, der
nur `CLAUDE.md` kennt, unsichtbar. Zwei davon kehren eine frühere Begründung um, und wer die
alte Begründung im Modulkopf der Runde 16 sucht, findet sie dort nicht mehr, in `CLAUDE.md`
aber auch keinen Hinweis, dass sie gefallen ist.

## Was zu tun wäre

Eine Zeile in der Rundentabelle, sofern die Runde 18 überhaupt einen Ort darin bekommt (siehe
`shared/issues/260826-0149_*_die-runde-18-hat-keinen-circle-datensatz-…`), und ein Absatz unter
„Was man nicht sieht" für den Handgriff mit der `readers.toml` und für die Abweichung zwischen
Leseläufen und Verzeichnisöffnungen. **Keine Zahl in Prosa**: die Profilzahl stand während
dieser Runde binnen Stunden auf fünf, acht und zwölf, und dieses Projekt hat für genau solche
Zahlen die Regel, sie durch das Kommando zu ersetzen, das sie zählt.

**Der Abgleich ändert `CLAUDE.md` nicht.** Der Auftrag hat es ausgeschlossen, und die Wahl,
was von den sechs Aussagen in eine Datei gehört, die jeder Agent zuerst liest, gehört dem
Nutzer.

**Schwere:** mittel.

**Gefunden:** reconciler, beim Abgleich der Runde 18 gegen `20eccd4..e5ec81a`.

---
Abgleich 260829-1252, am Baum `b9d9cbc`: **zum Teil erledigt, bleibt offen.** Die Tabellenzeile 18 steht seit `fb50fcd` in `CLAUDE.md` und nennt alle fünf Neuerungen in einer Zeile. Was der Datensatz daneben verlangt, fehlt weiter: `CLAUDE.md` trägt keinen Absatz über den Handgriff mit der `readers.toml` (`grep -n beiseite CLAUDE.md` trifft allein die Tabellenzeile 6, „Ablage beiseitelegen") und keinen über die Abweichung zwischen Leseläufen und Verzeichnisöffnungen. So auch der Kuratorenlauf `shared/history/260826-1637-curator-run.md` (Zeile zu diesem Datensatz). Die Änderung gehört dem Curator.
