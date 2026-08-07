# Wann wirkt eine von Hand geänderte `settings.toml` — erst nach einem Neustart oder sofort?

---
**Domain:** code
**Status:** implemented
**Filed by:** coder
**Cross-references:** `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` (`### Frage 4`, `#### Wohin die Terminal-Einstellung geht`, S18c), `decisions/260805-1623_*_taste-und-einstellbarkeit-des-terminal-befehls.md`, `crates/krk-core/src/ablage/einstellungen.rs`

---

## Frage

`settings.toml` ist die eine Ablagedatei, die keine Oberfläche hat und die der
Nutzer von Hand pflegt. Schritt 18c lädt sie **einmal beim Start** und hält den
Wert am Anwendungsdelegierten, dort, wo auch die Belegung und die Sitzung
hängen; der Plan schreibt das so vor. Daraus folgt eine Eigenschaft, die der
Plan nirgends ausspricht: wer die Datei bei laufendem KRK ändert, sieht die
Änderung erst nach einem Neustart.

Für `keymap.toml` stellt sich die Frage nicht, weil die Belegungsansicht aus C3
sie schreibt und der Nutzer sie in der laufenden Anwendung ändert. Für
`settings.toml` gibt es diesen Weg in dieser Runde nicht — von Hand ändern ist
der einzige Weg, und der Nutzer, der gerade `com.apple.Terminal` durch die
Kennung seines eigenen Terminals ersetzt hat, wird `ctrl+o` drücken und nicht
KRK beenden.

Die Frage ist damit: bleibt es beim einmaligen Laden, oder liest KRK die Datei
erneut, und wenn ja, wann?

## Möglichkeiten

1. **Es bleibt beim einmaligen Laden.** Der heutige Stand. Wer die Datei ändert,
   startet KRK neu.
   - Für: Ein Zustand, ein Ladezeitpunkt, kein zweiter Lesepfad. Genau der
     Zuschnitt, den der Plan für die Belegung und die Sitzung schon fährt.
   - Gegen: Die einzige Rückmeldung auf eine berichtigte Kennung ist, dass
     weiter das falsche oder gar kein Terminal aufgeht. Der Nutzer sieht die
     Meldung "keine Anwendung mit der Bündelkennung … installiert" auch dann
     noch, wenn er den Tippfehler eben behoben hat, und hat keinen Hinweis
     darauf, dass ein Neustart fehlt.
2. **Jeder Terminal-Befehl liest die Datei.** `ctrl+o` lädt `settings.toml`
   frisch, bevor es die Kennung auflöst.
   - Für: Eine Änderung wirkt sofort, ohne neuen Mechanismus: derselbe
     `einstellungen::laden`, nur an einer anderen Stelle gerufen. Die
     beschädigte Datei würde damit zur echten Befehlsantwort und der Widerspruch
     aus `issues/260805-1845_o_s18c-nennt-die-beschaedigte-einstellungsdatei-eine-befehlsantwort.md`
     löste sich auf.
   - Gegen: Ein Dateizugriff je Tastendruck. Er fällt gegen den Start einer
     Anwendung nicht ins Gewicht, ist aber ein Lesevorgang auf dem Hauptfaden
     und damit an L1 zu messen. Der geladene Wert am Delegierten wäre entweder
     überflüssig oder eine zweite Wahrheit daneben.
3. **KRK beobachtet die Datei.** Die FSEvents-Bindung aus S14 nimmt den
   Ablageordner dazu.
   - Für: Sofort wirksam, ohne Lesevorgang je Tastendruck.
   - Gegen: Der teuerste Zuschnitt für einen Wert, den der Nutzer im Jahr
     vielleicht zweimal ändert. S14 beobachtet die *angezeigten* Ordner; der
     Ablageordner wäre ein zweiter Beobachtungszweck mit eigener Lebensdauer.

## Randbedingungen

- Die Maxime "supersimpel" schließt einen zweiten Ladepfad neben dem ersten aus:
  gewählt wird einer, nicht beide nebeneinander.
- Die Datei darf in dieser Runde nicht geschrieben werden, egal welche Antwort
  fällt. Ein Schreibpfad löschte die Kommentare, die ihren Zweck ausmachen.
- Sobald eine spätere Runde eine Einstellungsansicht baut, beantwortet sie die
  Frage neu; die Antwort hier gilt für die Runde ohne Oberfläche.

## Empfehlung

Möglichkeit 2, und der Grund ist nicht die Bequemlichkeit, sondern die
Fehlermeldung. Die Antwort "keine Anwendung mit der Bündelkennung … installiert"
existiert, damit der Nutzer die Datei berichtigen kann. Unter Möglichkeit 1
berichtigt er sie und bekommt dieselbe Meldung noch einmal — die Meldung
verfehlt damit genau den Zweck, für den sie geschrieben ist. Der Preis ist ein
`read_to_string` über rund drei Kilobyte je `ctrl+o`, und der Befehl startet im
selben Atemzug eine Anwendung.

## Antwort des Nutzers vom 260807

**Möglichkeit 1, es bleibt beim einmaligen Laden. Der Nutzer hat damit gegen
die Empfehlung dieses Datensatzes entschieden**, die auf Möglichkeit 2 lautete,
`settings.toml` bei jedem Terminal-Befehl frisch zu lesen. Wer die Datei ändert,
startet KRK neu.

**"Es bleibt, wie es ist" ist auch hier eine Entscheidung und keine
Nicht-Entscheidung.** Für sie spricht der Zuschnitt, den die Randbedingungen
oben verlangen: ein Zustand, ein Ladezeitpunkt, kein zweiter Lesepfad neben dem
ersten. Genau diesen Zuschnitt fährt die Runde für die Belegung und für die
Sitzung bereits, und die Maxime "supersimpel" schließt zwei Ladepfade
nebeneinander aus. Möglichkeit 2 hätte den geladenen Wert am
Anwendungsdelegierten entweder überflüssig gemacht oder eine zweite Wahrheit
danebengestellt; Möglichkeit 3 wäre der teuerste Zuschnitt für einen Wert
gewesen, den der Nutzer im Jahr vielleicht zweimal ändert.

**Der Preis ist der, den die Empfehlung oben nennt, und er wird getragen.** Die
Meldung "keine Anwendung mit der Bündelkennung … installiert" existiert nach dem
fünften Abnahmekriterium von C11, damit der Nutzer die Datei berichtigen kann.
Unter dem einmaligen Laden berichtigt er sie, drückt `ctrl+o` und **bekommt
dieselbe Meldung noch einmal**, ohne einen Hinweis darauf, dass allein ein
Neustart fehlt. Die Meldung erfüllt damit ihren Wortlaut und verfehlt ihren
Zweck. Das ist kein gedachter Fall: die Bündelkennung ist eine Zeichenkette,
die der Nutzer von Hand tippt, und der Tippfehler ist der Anlass, aus dem die
Meldung überhaupt geschrieben wurde.

**Was mit dem Preis geschieht.** Ein Vorschlag, ihn ohne einen zweiten Lesepfad
zu mildern, liegt als eigener Defekt und ist **nicht** mitentschieden:
`issues/260807-0930_*_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md`.
Er schlägt vor, dass die Meldung den Ladezeitpunkt selbst nennt, und führt auf,
was dagegen spricht. Über ihn befindet der Nutzer.

**Kein Abnahmekriterium ändert sich, und kein Schritt des Plans.** S18c bleibt
abgenommen und unverändert; die Antwort bestätigt, was er gebaut hat. Die
Randbedingung, dass die Datei in dieser Runde nicht geschrieben wird, bleibt
bestehen, und sobald eine spätere Runde eine Einstellungsansicht baut,
beantwortet sie die Frage neu.

---
Answered: `planning/260802-1036_*_spec-navigator-geruest.md`:478 — Möglichkeit 1, einmaliges Laden, gegen die Empfehlung dieses Datensatzes; der Preis an der Fehlmeldung steht dort und als Defekt `issues/260807-0930_*_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md`.
Implemented: `crates/krk-core/src/ablage/einstellungen.rs` — die Antwort lautet „es bleibt beim einmaligen Laden", und der Programmstand erfüllt sie ohne Eingriff: S18c lädt die Datei beim Start und hält den Wert am Anwendungsdelegierten, ein zweiter Lesepfad entsteht nicht. S18c bleibt unverändert. Der Marker steht auf umgesetzt, weil der Zustand auf der Platte die Entscheidung trägt, nicht weil etwas gebaut worden wäre. Der Preis der Wahl, eine Fehlmeldung, die nach der Berichtigung ein zweites Mal erscheint, liegt als `issues/260807-0930_*_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md` und ist nicht entschieden.
Deferred:
Superseded by:
