`CLAUDE.md` schreibt den zweiten Empfänger der Ersthelfermeldung der Runde 14 zu, er landete ohne aktiven Circle

---

Der mit `fb50fcd` neu gefasste Absatz sagt: „**Am Melder hängen seit der Runde 14 zwei
Empfänger**". Der zweite Empfänger `aktives_dem_ersthelfer_nachziehen` ist mit `76ceb68` am
**2026-08-19 11:20** in den Baum gekommen, also rund elf Stunden **bevor** die Runde 14
überhaupt bestand. Die Zuschreibung ist falsch; richtig ist das Datum, das dieselbe Datei an
zwei anderen Stellen und der Quelltext selbst nennen: seit dem 260819.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

**Domain:** code

**Betroffen:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß", der Absatz
über `makeFirstResponder:`; und `shared/issues/260823-1336_*_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-der-baum-traegt-seit-dem-260819-zwei.md`,
aus dem die Zuschreibung stammt.

## Selbst gefahren am 260826-0923

```
git log --format='%h %ad %s' --date=iso -S 'aktives_dem_ersthelfer_nachziehen' \
    -- crates/krk-ui/src/appkit/anwendung.rs
  → 76ceb68  2026-08-19 11:20:18 +0200  fix(ui): ein Klick unter die letzte Zeile holt das
             Dateifenster als aktives

git log --format='%h %ad %s' --date=iso \
    -- 'fusion-workbench/circles/260818-1615-ordner-angleichen-…/_c_circle.md'
  → c09ff3a  2026-08-19 08:12:00 +0200  die Runde 13 schliesst kohaerent

git log --diff-filter=A --format='%h %ad %s' --date=iso \
    -- 'fusion-workbench/circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/*circle.md'
  → 258bd7c  2026-08-19 22:31:40 +0200  die Runde 14 ist geformt und aktiv

git show --stat 76ceb68 | grep fusion-workbench
  → (leer)
```

Die Runde 13 war um 08:12 geschlossen, die Runde 14 wurde um 22:31 aktiv. Der Commit dazwischen
trägt kein einziges Werkbank-Artefakt; seine Artefakte liegen unter `shared/`
(`shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`,
`shared/history/260819-1043-klick-holt-den-fokus-nicht.md`). Er ist eine Defektbehebung zwischen
zwei Runden, gemeldet vom Nutzer am 260819, und gehört zu keiner Runde.

## Warum das trägt

Der Fehler ist genau die Gestalt, die `fb50fcd` an vier anderen Stellen derselben Datei
ausdrücklich behebt: nicht jede Arbeit gehört einer Runde, und wer sie einer zuschreibt, macht
eine prüfbare Aussage, die nicht hält
(`shared/issues/260826-0149_*_die-runde-18-hat-keinen-circle-datensatz-….md`). Der Eintrag L07
desselben Laufs schreibt sie neu ein.

Der Quelltext sagt es richtig und braucht keine neue Erhebung. `anwendung.rs`, der
Doc-Kommentar von `der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an`: „**Seit dem 260819**
trägt sie ihr Gewicht wirklich." Der Kommentar am Melder nennt gar kein Datum. `CLAUDE.md`
selbst nennt an zwei Stellen des Absatzes darüber ebenfalls Daten und keine Runden.

Die Zuschreibung stammt nicht vom Kuratorenlauf, sondern aus dem Datensatz, den er umgesetzt
hat: `260823-1336` schreibt zweimal „Runde 14" (im Rumpf und in der Belegzeile „`76ceb683` vom
260819 (Runde 14)"). Beide Stellen sind mitzuberichtigen, sonst kommt die Zuschreibung beim
nächsten Lauf zurück.

## Vorschlag

„seit der Runde 14" durch „seit dem 260819" ersetzen, in `CLAUDE.md` und in `260823-1336`.
Kein Verhalten hängt daran; der Rest des Absatzes bleibt wie er steht und ist am Baum belegt
(`anwendung.rs:1225-1230`, `:4648`, `:5057`).

**Schwere:** mittel. Eine falsche Tatsachenbehauptung in der Datei, die jeder Agent zuerst
liest, und sie steht in demselben Commit, der dieselbe Fehlerklasse an vier Stellen behebt.

**Gefunden:** coderev, Durchsicht von `e5ec81a..20c9833` am 260826-0923

---
Resolved: c95f28b — `CLAUDE.md:141` sagt jetzt „Am Melder hängen seit dem 260819 zwei Empfänger
(`76ceb68`)" statt „seit der Runde 14". Am 260826-1017 nachgemessen: `76ceb68` datiert
2026-08-19 11:20, der Schluss der Runde 13 `c09ff3a` auf 08:12, der Beginn der Runde 14
`258bd7c` auf 22:31, und `git show --stat 76ceb68` nennt keine Datei unter `circles/`.

Die zweite Hälfte des Vorschlags — dieselbe Berichtigung im Quelldatensatz
`shared/issues/260823-1336_*_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-…` — ist als
`Revised by:`-Vermerk am Ende jenes Datensatzes ausgeführt und nicht als Ersetzung im Rumpf. Die
Konvention lässt dem Abgleich keine andere Form: er ändert Markierungen, Abgleichprotokolle und
Belegzeilen und nicht die Beschreibung eines Defekts, und für eine umgezogene Begründung ist der
`Revised by:`-Vermerk ausdrücklich die vorgesehene Gestalt. Der Vermerk nennt beide Stellen des
Rumpfs, damit ein späterer Lauf sie vor Augen hat, statt sie ein zweites Mal zu übernehmen.
