Drei Prosastellen in `menue.rs` nennen 85 Funktionen, die Belegung führt 92

---

`crates/krk-ui/src/appkit/menue.rs` nennt an drei Stellen „die 85 Funktionen" und an einer davon
zusätzlich „79 der 85". Beide Zahlen sind falsch geworden. `resources/default-keymap.toml` führt
heute 92 Einträge, von denen 6 `gehalten_von = "menue"` tragen und damit kein `Kommando`; die
Prosa müsste also 92 und 86 sagen. Die Zahl 6 stimmt weiter und ist die einzige der drei, die
eine Probe hält.

---

**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260815-1448_*_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-und-die-benannte-ursache-traegt-keinen-datensatz.md` (derselbe Mechanismus, dort als Ursache ohne eigenen Datensatz benannt); `260812-0810_*_die-zahl-39-im-kopf-der-belegungsdatei-steht-im-praesens-und-ist-ungeprueft.md` (dieselbe Sorte an der Belegungsdatei); `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` (die offene Frage nach der Bauform, die solche Zahlen hält)

## Die drei Stellen

| Stelle | Was dort steht |
|---|---|
| `crates/krk-ui/src/appkit/menue.rs:164` | „Die sechs sind die einzigen der 85 Funktionen ohne `Kommando`" |
| `crates/krk-ui/src/appkit/menue.rs:865` | „Sie tragen als einzige der 85 …" |
| `crates/krk-ui/src/appkit/menue.rs:944` | „Fuer 79 der 85 Funktionen ist der Wirkungsbereich aus der Belegung entscheidbar" |

## Gemessen

```
grep -c '^id = ' resources/default-keymap.toml                 -> 92
grep -c 'gehalten_von = "menue"' resources/default-keymap.toml ->  6
awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs \
  | grep -cE '^    [A-Z][A-Za-z0-9]*,'                         -> 86
```

92 minus die 6 ohne `Kommando` ergibt 86, und das ist genau die Zahl der Varianten von
`Kommando` im Baum. Die Rechnung geht auf; allein die Prosa steht sieben daneben.

## Was daran hängt

Die Aussage bei `:944` begründet, worauf die dritte Spalte der Markdown-Ausgabe aus C3 ruht. Der
tragende Teil davon ist die Sechs, und die hält die Probe
`die_sechs_zugestellten_textbefehle_werden_von_diesen_klassen_beantwortet` mit `GEMESSEN`
(`menue.rs:903`, sechs Einträge, vom Übersetzer an der Feldbreite gehalten). Die 85 und die 79
hält nichts, und sie sind mit den Runden 19 bis 23 gewachsen, ohne dass jemand sie nachgezogen
hätte.

**Schwere:** niedrig. Kein Bau, kein Verhalten, keine Nutzerauskunft ist betroffen: die Ausgabe
zählt selbst und nimmt die Zahl nicht aus der Prosa.

**Abnahme:** entweder tragen die drei Stellen die heutige Zahl mit dem Datum des Nachzählens,
oder sie nennen statt der Zahl das Zählkommando, wie `CLAUDE.md` es für dieselbe Größe schon tut.
Nachprüfbar daran, dass `grep -c '85 Funktionen\|85 ' crates/krk-ui/src/appkit/menue.rs` keine
Zeile mehr liefert, die eine Gesamtzahl behauptet.

**Gefunden:** beim Abgleich am 260905-2046, bei der Nachprüfung von
`260815-1448_*_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-…`. Acht der neun dort
genannten Stellen sind inzwischen zahlenfrei; diese eine ist geblieben und hat sich neu verzählt.


---
Resolved: Alle drei Stellen tragen statt der Zahl den Zeiger auf die Erhebung. Der Modulkopf (`crates/krk-ui/src/appkit/menue.rs`, Abschnitt „Wer die sechs Textbefehle beantwortet") sagt jetzt, die sechs seien die einzigen Funktionen der Belegung ohne `Kommando`, und nennt die zwei Zaehlkommandos `grep -c '^id = ' resources/default-keymap.toml` und `grep -c 'gehalten_von = "menue"' resources/default-keymap.toml`. Der Doc-Kommentar an `die_sechs_zugestellten` verweist auf den Modulkopf, und der an `die_sechs_zugestellten_textbefehle_werden_von_diesen_klassen_beantwortet` sagt jetzt „fuer jede Funktion der Belegung, die ein Kommando traegt" statt „fuer 79 der 85". Die Sechs bleibt, weil die Probe daneben sie haelt. Nachgeprueft: `grep -n '85' crates/krk-ui/src/appkit/menue.rs` liefert keine Zeile mehr, die eine Gesamtzahl behauptet.
