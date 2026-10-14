# Die Directive der Runde 10 und ein Planschritt schreiben das alte Leeren weiter fest

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/_b_circle.md:14` (`## Directive`); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md:340-341` (Schritt B2, `Changes`); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-…:308-309` (C1.9, C1.10 — nachgezogen); `CLAUDE.md` (`## Worum es geht`, „Die vollständige Directive steht im Circle-Datensatz …, Abschnitt `## Directive`")

---

`f8297b6` hat den Spec nachgezogen (C1.9 und C1.10) und den Plan an zwei Stellen. Zwei
normative Stellen sind dabei stehengeblieben, und beide sagen das Gegenteil des
Nutzerentscheids vom 260815-0955.

## 1. Die Directive der Runde 10

`_b_circle.md:14`, erster Absatz des Abschnitts `## Directive`:

> Der Filter gehört dem Tab und **wird beim Ordnerwechsel geleert**; `Esc` nimmt zuerst
> den Filtertext zurück, bevor es seine übrigen Bedeutungen bekommt.

`CLAUDE.md` erklärt die Directive im Circle-Datensatz ausdrücklich zur verbindlichen
Formulierung und die eigene Kurzfassung zur unverbindlichen. Damit steht die
verbindliche Formulierung heute gegen den Code und gegen C1.9.

Der Satz ist zugleich die Vorbelegung, die der Entscheidungsdatensatz
`decisions/260814-1830_i_bleibt-der-filtertext-…` in seinem Abschnitt `## Question`
wörtlich zitiert und deren Umkehrung der Nutzer gewählt hat. Wer die Directive liest,
ohne den Datensatz zu kennen, bekommt die verworfene Antwort als bindende Vorgabe.

## 2. Der `Changes`-Block von Schritt B2

`plan …:340-341` trägt zwei aufeinanderfolgende Aufzählungspunkte, die einander
widersprechen:

```text
Zeile 340:  … Bei ausgeschaltetem „Deep" wird der Filtertext dabei geleert (C1.9),
            bei eingeschaltetem übernommen (C1.10). …
Zeile 341:  Beantwortet durch decisions/…, Nutzerentscheid vom 260815-0955.
            Die Antwort lautet „stehen lassen", also fällt die Bedingung weg …
```

`f8297b6` hat den zweiten Punkt umgeschrieben und den ersten unberührt gelassen. Der
Plan beschreibt seinen eigenen Schritt B2 damit in zwei Zeilen zweimal verschieden;
die Zeile 341 verweist außerdem auf `let filtertext_ueberlebt = tief;` als das, was
„zu einem `true` wird" — die Variable ist ersatzlos entfallen, also gibt es die Zeile
nicht mehr.

## Warum die Suche des `coder` das nicht gefunden hat

`shared/history/260815-1019-coder-filtertext-uebersteht-jeden-ordnerwechsel.md` hält
fest: „Gesucht wurde über `Ordnerwechsel`, `Filtertext`, `geleert` und `leert` im
ganzen Baum." Die Aussage steht dort im Abschnitt über die **Proben**, und für Proben
hält sie: außerhalb von `tabs.rs` schreibt keine das Leeren fest, nachgeprüft. Die
Werkbank hat dieselbe Suche nicht bekommen. Beide Fundstellen tragen das Wort
„geleert" beziehungsweise „geleert (C1.9)" und wären ihr aufgefallen.

## Was zu tun ist

- **Der Planschritt** wird berichtigt: die Zeile 340 sagt, was der Schritt gebaut hat,
  und die Zeile 341 bleibt als Herkunftsvermerk stehen. Kein Gewissensfall.
- **Die Directive** wird **nicht** stillschweigend umgeschrieben. Die Runde ist
  beschränkt geschlossen, und der Satz war zur Zeit des Abschlusses richtig. Angemessen
  ist dieselbe Form, die C1.9 im Spec bekommen hat: der berichtigte Satz mit dem
  ausgeschriebenen Vermerk, wie er bis zum 260815-0955 lautete und welcher
  Nutzerentscheid ihn abgelöst hat. Sonst liest der nächste Leser die Umkehrung als
  Versehen — genau das, was der Abschnitt „Was diese Runde fallen lässt" in derselben
  Datei für einen anderen Fall schon einmal verhindert hat.

---

## Stand am 260815-1145: die Hälfte ist erledigt

**Der Planschritt B2 ist berichtigt.** Seine Zeile 340 beschreibt jetzt, was gebaut wurde, mit dem Vermerk, was bis zum 260815-0955 dastand; der Herkunftsvermerk darunter nennt den Datensatz unter seinem heutigen Marker und die entfallene Zeile `let filtertext_ueberlebt = tief;`.

**Die Directive der Runde 10 steht unverändert.** Der Orchestrator darf den Abschnitt `## Directive` eines Circle-Datensatzes nicht schreiben — er darf am Datensatz allein die Closure-Notiz, den Turn-Log und drei Kopffelder anfassen. Der Nachtrag ist deshalb in die Closure-Notiz gesetzt: der überholte Satz bleibt oben stehen, und die Notiz sagt, dass er seit dem 260815-0955 nicht mehr gilt, was an seine Stelle tritt und wo es entschieden wurde.

**Dieser Datensatz bleibt offen**, weil die Empfehlung, die Directive selbst in berichtigter Fassung mit ausgeschriebenem Vermerk zu führen, damit nicht eingelöst ist. Wer sie einlöst, ist der `shaper`; der Nutzer entscheidet, ob es sich lohnt.
