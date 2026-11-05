HowTo sagt, nach einem anderen Tun geschehe nichts mehr; das Löschblatt geht trotzdem auf
---
Schwere: Niedrig. Die Anleitung verspricht für alle fünf Befehle etwas, das Löschen und F2 nicht tun. Geschehen kann nichts ohne Klick, das Blatt nennt seinen Gegenstand, aber es kann lange nach dem Befehl aufgehen, wenn eine Freigabe aufwacht.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `HowTo.md:207` bis `:209`, im neuen Abschnitt „Wenn eine Platte langsam ist oder nicht antwortet“, der alle fünf Befehle aufzählt: „Hast du inzwischen etwas anderes getan, geschieht danach nichts mehr: der Befehl gilt dann als überholt“.

- **Löschen.** `loeschbefund_einziehen` (`crates/krk-ui/src/appkit/anwendung.rs:9054`) fragt allein die Nummer des Löschbefehls (`:9059`) und keinen Stand. Tab, Dateifenster oder Ordner gewechselt: das Blatt geht auf, solange kein neueres Löschen kam und kein Blatt steht. Dass das Verhalten selbst vertretbar ist, sagt die vorige Durchsicht (`261105-1830-reviewer-abschlussdurchsicht-nacharbeit-hauptfaden.md`, „Cross-cutting observations“), und ich ändere daran nichts. Falsch ist der Satz in der Anleitung.
- **F2.** `notizordnerausgang` kennt `Notizordnerfolge::NurSaetze` und `griff_ersetzen` (`anwendung.rs:1006`, `:1035`, Anwendung `:6951`): bei anderem Tab wird der Griff ersetzt, und die Sätze über das Angelegte erscheinen in der Statuszeile. Auf der Platte ist ohnehin schon angelegt.

Aus dem Quelltext gelesen, im Programm nicht gefahren.

**Richtung.** Den Satz auf das bringen, was gilt, ohne Fachwort: bei Sprung, Lesezeichen und F4 geschieht danach nichts mehr, das Löschen fragt trotzdem noch, und F2 hat seine Ordner dann schon angelegt und sagt es. Ob das Löschblatt auch nach einem Tabwechsel aufgehen soll, kann der Nutzer dabei mit entscheiden; ein Stand wie bei den anderen vier wäre die Änderung am Code statt am Text.

**Abnahme.** Der Absatz liest sich gegen `loeschbefund_einziehen` und `notizordnerausgang` richtig. Am Wortlaut hängt keine Probe.
---
Resolved: `HowTo.md` im Abschnitt „Wenn eine Platte langsam ist oder nicht antwortet“ sagt je Befehl, was nach einem anderen Tun geschieht; das Löschblatt und die Sätze von F2 bleiben nach Nutzerentscheid, wie sie sind. Am Code nichts geändert.
