# Was sagen die drei Zählzeilen für einen Ordner über der Eintragsschranke von zweitausend?

---
**Domain:** code
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_EINTRAEGE`, Zeilen 125-138; `Wert::UeberGrenze`, Zeilen 664-726); `crates/krk-core/src/leseprofil/bausteine.rs` (Abschnitt „Was eine unvollständige Lesung sagen darf", Zeilen 93-115); `shared/issues/260825-1953_*_ein-platzhalterlauf-oeffnet-bis-zu-zweitausend-verzeichnisse-und-die-eintragsschranke-faengt-das-nicht.md`

---

## Question

Eine Zusammenfassung liest höchstens zweitausend Einträge und bricht dann ab. Der Baustein `zaehlung` liefert in diesem Fall nicht die Zahl, sondern `Wert::UeberGrenze`, und die Zeile lautet „mindestens 2.000 (Lesung bei 2.000 Einträgen abgebrochen)". Für die drei Zählzeilen dieser Runde bringt das eine Besonderheit, die der heutige Baustein nicht hat: die Klammer mit der Zahl der versteckten Einträge ist in einer abgebrochenen Lesung nicht bloß ungenau, sondern unentscheidbar, denn hinter dem Abbruch können beliebig viele versteckte stehen. Der Nutzer sieht diesen Fall in jedem großen Ordner, etwa einem Download-Verzeichnis oder einem `node_modules`, und er muss vor dem Plan entschieden sein, weil er die Gestalt der drei Zeilen mitbestimmt.

## Options

1. **Dieselbe Regel wie beim Baustein `zaehlung`** — jede der drei Zeilen sagt „mindestens N (Lesung bei 2.000 Einträgen abgebrochen)", und die Klammer mit den versteckten entfällt in dieser Lage ganz.
   - Pros: Eine Regel im Baum und keine zweite daneben. Sie sagt nur, was die Teillesung entscheidet, was die ausdrückliche Hausregel der Runde 16 ist.
   - Cons: Die Zeile wird lang und liest sich anders als im Normalfall. Der Nutzer bekommt für große Ordner nie eine Zahl.
2. **Die Zählung bekommt eine eigene, höhere Schranke oder gar keine** — der Ordner wird vollständig gezählt, wie groß er auch ist, und die drei Zeilen tragen immer eine Zahl.
   - Pros: Die Auskunft ist immer die, die der Nutzer wollte, gerade beim großen Ordner, wo sie am meisten wert ist. Ein flacher Leselauf über einen Ordner ist billig, auch bei hunderttausend Einträgen, und öffnet genau ein Verzeichnis.
   - Cons: Die Zusage, die `HOECHSTENS_EINTRAEGE` trägt, gilt dann nicht mehr für jeden Weg in die Zusammenfassung. Ein Ordner mit einer Million Einträgen hält den Arbeitsfaden der Vorschau spürbar auf, und gemessen ist das gegen keine der zehn Zeitzusagen.
3. **Die drei Zählzeilen fallen über der Schranke weg** — die Vorschau zeigt dann die sechs Metadatenangaben allein, wie heute.
   - Pros: Keine Zeile, die etwas Halbes sagt. Der Rückfall ist der Zustand, den der Nutzer ohnehin kennt.
   - Cons: Das Verschwinden der Zeilen ist für den Nutzer nicht erklärt; er sieht keinen Grund, warum gerade dieser Ordner keine Zahlen trägt.

## Constraints

- Was eine unvollständige Lesung sagen darf, ist in der Runde 16 als eine Regel festgelegt: es wird nur gesagt, was die Teillesung entscheidet. Ein Nichtfund in einer Teilliste ist kein Nichtvorhandensein.
- Die Zählung dieser Runde läuft über genau einen Ordner und kostet einen Leselauf. Sie verbreitert den offenen Defekt zum Platzhalterlauf nicht.
- Die Eintragsschranke wohnt im Leseprofil-Werk und nicht im Leser: `lesen_hoechstens` nimmt die Zahl von außen entgegen und hält keine eigene (`crates/krk-core/src/verzeichnis/leser.rs:238-246`). Eine andere Schranke für die Zählung wäre damit kein Eingriff in den Leser.
- Ein Ladevorgang der Vorschau kennt keinen Abbruch: fällt der Empfänger, liest der Faden trotzdem zu Ende (`crates/krk-ui/src/vorschaumodell.rs:307-311`). Eine ungedeckelte Zählung wirkt deshalb über die Zeile hinaus, auf der der Nutzer sie ausgelöst hat.
- Die Arbeit der Vorschau ist gegen L7 bis heute ungemessen (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`).

## Recommendation

Keine belastbare Empfehlung ohne eine Messung. Möglichkeit 1 hält die bestehende Regel und ist die sichere Wahl; Möglichkeit 2 gibt dem Nutzer, was er verlangt hat, kostet aber eine Zusage, deren Preis in diesem Baum nicht gemessen ist. Was die Wahl entscheiden würde, ist die Zeit eines flachen Leselaufs über einen sehr großen Ordner auf dem Referenzgerät, und die liegt nicht vor.

---
Answered: circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/history/260827-0622-orchestrator-session.md:50 — Möglichkeit 1: dieselbe Regel wie beim Baustein `zaehlung`. Jede Zeile sagt „mindestens N (Lesung bei 2.000 Einträgen abgebrochen)", die Klammer mit den versteckten entfällt in dieser Lage.
Implemented: 3ee2638 — `zaehlen` liefert bei abgeschnittenem Lesestand `Wert::UeberGrenze` vor jedem `ZahlMitVersteckten`, gleich wie `versteckt` steht (`crates/krk-core/src/leseprofil/bausteine.rs:796-812`); die Probe `ueber_der_schranke_sagen_die_drei_zeilen_mindestens_und_tragen_keine_klammer` (`891f313`, `tests/leseprofil.rs:2508`) hält es für die drei Zeilen des Default-Profils.
