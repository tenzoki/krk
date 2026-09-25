# Der dreizehnte Befund der Durchsicht: die Prosa folgt dem Baum

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Durchsicht:** `260831-1444-coderev-git-bereich-runde-23.md`, Befund M5
**Geschlossen:** `260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md`
**Beantwortet:** `260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`
**Nachgetragen:** `260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md`

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`. Dazu einzeln gefahren:
`cargo doc -p krk-ui --no-deps --document-private-items`, um zu sehen, dass keiner der neu
gesetzten Doc-Verweise unaufgeloest bleibt. Die drei Warnungen, die die beruehrten Dateien
weiter melden (`gitmodell.rs:12` auf `Ordnermodell`, `tabs.rs:6` und `tabs.rs:414`), standen
vor diesem Zug schon da und gehoeren nicht dazu.

**Keine Zeile Code.** Der Zug ist reine Prosa: vier Doc- und Modulkommentare, drei
Workbench-Datensaetze. Kein Verhalten hat sich geaendert, und das ist der Gegenstand des
Nutzerentscheids und nicht eine Beschraenkung des Auftrags.

---

## Die Entscheidung des Nutzers

Der Nutzer hat am **260831-1755** die zweite Moeglichkeit des Datensatzes
`260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`
gewaehlt: **die Auswahl der Verlaufsliste faellt mit dem Tabwechsel. Der gebaute Zustand
ist der gewollte, die Prosa zieht nach.**

Damit ist die zweite Aussage der Antwortzeile vom 260831-0120 — „sie uebersteht damit den
Tabwechsel" — nicht mehr Grundlage. Die erste, die Heimat der Auswahl im `Gitmodell`, gilt
unveraendert und ist gebaut.

---

## Was am Baum gilt, in beiden Haelften

Die halbe Wahrheit war der Grund, aus dem der Befund entstanden ist, also steht sie
nirgends mehr halb da. Jede angefasste Stelle sagt jetzt beides:

- **Die Auswahl uebersteht den Wechsel des aktiven Dateifensters.** Jede `Tabliste` haelt
  ihr eigenes Gitmodell; beim Zurueckwechseln zeigt der Bereich, was dort steht.
- **Die Auswahl faellt mit dem Tabwechsel.** `Tabliste::waehlen` ruft fuer den verlassenen
  Tab `gitlauf_nachziehen_an`, und dessen dritte Zeile ist
  `self.tabs[stelle].gitmodell.zuruecksetzen()` — unbedingt und vor jeder
  Bedingungspruefung. `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()` und nimmt
  Kopf, Verlauf, Zusammenfassung **und** die Auswahl mit.

---

## Die vier Prosastellen

Der Defektdatensatz nennt drei. Bei der Suche nach weiteren ist eine vierte aufgefallen,
die dieselbe Aussage in ihrer Begruendung traegt, ohne das Wort „Auswahl" zu gebrauchen;
sie ist mitgezogen.

### 1. `crates/krk-ui/src/appkit/git.rs`, Modulkopf `# Die Auswahl wohnt im Gitmodell und nicht hier`

Stand: „es gibt **ein** Gitfenster und **ein Gitmodell je Tab**, also uebersteht die
Auswahl den Tabwechsel und den Wechsel des aktiven Dateifensters, wie es das Halteverhalten
der Tabs in KRK ueberall sonst tut."

Steht jetzt: ein Gitmodell je Dateifenster **und** Tab; die Reichweite ist die zweite
Nutzerfrage, und ihre Antwort hat zwei Haelften — der Wechsel des aktiven Dateifensters
wird ueberstanden, der Tabwechsel raeumt sie ab, und zwar an `Tabliste::waehlen` ueber
`gitlauf_nachziehen_an`. Der Verweis auf `260831-1815_*_…` steht daneben. Die zwei Regeln
darunter (C4.6 Ordnerwechsel, C4.2 nachgeladener Schwung) sind unveraendert geblieben; nur
der Satz, der sie einordnet, nennt jetzt neben dem Gitmodell auch die Tabliste als den Ort,
an dem geraeumt wird.

### 2. `crates/krk-ui/src/tabs.rs`, Doc-Kommentar von `Tabinhalt::gitmodell`

Stand: „damit sie den Tabwechsel uebersteht: es gibt **ein** Gitfenster und **ein Gitmodell
je Tab**."

Steht jetzt: die Heimat der Auswahl ist mit dem Entscheid vom 260831-0120 begruendet und
nicht mehr mit einer Reichweite, die der Baum nicht haelt; die Reichweite selbst steht als
eigener Absatz mit beiden Haelften und dem Verweis auf `260831-1815_*_…`. Der Rest des
Kommentars — das eine Feld mit einem eigenen Schreiber, Moeglichkeit 3 und woran sie
gescheitert ist — steht unveraendert.

### 3. `crates/krk-ui/src/gitmodell.rs`, der Kommentarblock vor `impl Gitmodell`

Stand: „Der Nutzer hat die Auswahl am 260831 hierher gelegt und nicht in die Ansicht (…),
damit sie den Tabwechsel uebersteht."

Steht jetzt: derselbe Satz ohne den Nebensatz, und daneben die zweite Entscheidung mit
beiden Haelften. Der Weg des Auswahlmelders darueber ist unangetastet.

### 4. `crates/krk-ui/src/gitmodell.rs`, Modulkopf `# Ein Gitmodell je Tab` — **die vierte, nicht im Datensatz genannte Stelle**

Stand: „Ein Modell beim Fenster statt beim Tab muesste bei jedem Tabwechsel neu gefuellt
werden und haette den Stand des verlassenen Tabs schon weggeworfen, bevor der Nutzer
zurueckwechselt."

Das ist derselbe Befund in einer Begruendung statt in einer Zusage, und er ist unter dem
gebauten Verhalten in beiden Haelften falsch: das Modell am Tab **wird** bei jedem
Tabwechsel neu gefuellt, und der Stand des verlassenen Tabs **ist** weggeworfen, bevor der
Nutzer zurueckwechselt. Der Absatz trug damit ein Unterscheidungsmerkmal vor, das nicht
unterscheidet.

Steht jetzt: der Grund, der wirklich traegt — ein Modell beim Git-Bereich waere **ein**
Stand fuer zwei Dateifenster und muesste bei jedem Wechsel des aktiven Dateifensters neu
gefuellt werden —, und darunter ein eigener Absatz, der den Tabwechsel ausdruecklich
ausnimmt und den Entscheid vom 260831 nennt.

---

## Wo die Aussage nicht steht

Gesucht ist ueber `crates/`, `xtask/`, `resources/`, `README.md`, `CLAUDE.md` und den
ganzen Workbench-Baum nach `Tabwechsel`, `ueberste`/`uebersteh`, `ueberleb` und
`zurueckwechsel`.

- **Spec und Plan der Runde 23 tragen sie nicht.** Der Spec nennt die Auswahl an C2.7,
  C3.5, C4.2 und in der Verlaufsgrafik, keine dieser Stellen redet vom Tabwechsel; A9
  fuehrt den Tabwechsel als Ausloeser des Gitbefunds und sagt damit das Gegenteil. Der Plan
  nennt an Zeile 61 den Tabwechsel als das, was ein Gitmodell **am Fenster** neu fuellen
  muesste — dieselbe Formulierung wie die vierte Stelle oben, aber im Konjunktiv ueber eine
  verworfene Bauform und nicht als Zusage ueber die gebaute; sie bleibt stehen.
- **`crates/krk-ui/src/appkit/tabelle.rs`** (`gitauswahl_setzen`) und die uebrigen
  Auswahl-Kommentare in `appkit/git.rs` (`Auswahlmelder`, das Ivar-Feld, `zeigen`,
  `kommando_ausfuehren`) sagen ueber die Reichweite nichts und sind unangetastet.
- **Keine Probe** behauptet sie. `ein_ordnerwechsel_setzt_den_verlauf_auf_die_ersten_fuenfzig_zurueck`
  prueft C4.6, nicht den Tabwechsel.
- **Aufzeichnungen bleiben stehen** und bekommen keine Aenderung: die Durchsicht
  `reviews/260831-1444-coderev-git-bereich-runde-23.md` (M5), die History-Eintraege
  `260831-0120-coder-schritt-7-der-git-bereich-als-ansicht.md` und
  `260831-1815-coder-die-acht-prosabefunde-der-durchsicht.md`, und
  `shared/history/260830-0950-orchestrator-session.md:126`. Die Ortsregel dieses Projekts
  laesst eine Aufzeichnung ihren damaligen Stand behalten; was damals dastand, ist der
  Beleg.

Nach diesem Zug behauptet **keine** Stelle im Baum mehr, die Auswahl uebersteht den
Tabwechsel — ausser den Aufzeichnungen, die es sollen.

---

## Die drei Datensaetze

- **`260831-1444_*_drei-prosastellen-…`** — `Resolved:`-Zeile angehaengt, `_o_` → `_c_`.
  Der Abnahmetest des Datensatzes nannte beide Wege; gegangen ist der zweite.
- **`260831-1815_*_faellt-die-auswahl-…`** — `Answered:` und `Implemented:` angehaengt,
  `_o_` → `_i_`. **Warum `_i_` und nicht `_a_`:** die Antwort lautet „der gebaute Zustand
  gilt", und ihre Realisierung ist genau der Nachzug der Prosa. Der ist mit diesem Zug
  vollstaendig auf der Platte und mit `path:line` zitierbar; ein `_a_` hiesse, es stuende
  noch etwas aus. Es steht nichts mehr aus.
- **`260831-0120_*_wo-wohnt-die-auswahl-…`** — Wortlaut, Marker `_i_`, `Answered:` und
  `Implemented:` bleiben unangetastet; ein Abschnitt `## Nachtrag 260831-1755` haengt
  darunter. Er sagt, welche der zwei Aussagen der Antwortzeile nicht gebaut worden ist, wer
  es gefunden hat und wie der Nutzer entschieden hat, und verweist auf den neuen Datensatz.
  **Kein Ueberschreiben:** der Datensatz ist eine Aufzeichnung, und der urspruengliche
  Wortlaut ist der Beleg dafuer, dass die Frage so gestellt und so beantwortet war.

---

## Was offen bleibt

Nichts aus diesem Auftrag. Der dreizehnte Befund der Durchsicht ist der letzte gewesen; die
zwoelf davor waren vor diesem Zug schon geschlossen.
