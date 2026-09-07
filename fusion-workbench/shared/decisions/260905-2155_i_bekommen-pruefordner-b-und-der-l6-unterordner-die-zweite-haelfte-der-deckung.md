# Bekommen Prüfordner B und der L6-Unterordner die zweite Hälfte der Deckung, oder bleibt es bei der ehrlichen Beschriftung?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-2155_*_pruefordner-b-und-der-l6-unterordner-werden-nur-gegen-ihren-steckbrief-gehalten-und-der-kommentar-sagt-b-werde-nicht-gelesen.md` (offen; der Befund, aus dem diese Frage stammt), `260826-1301_*_kein-pruefordner-ausser-dem-l6-unterordner-wird-gegen-seine-zugesagte-eintragszahl-gehalten.md` (geschlossen in `960900d`)

---

## Frage

Ein Prüfordner ist gegen seinen Inhalt erst dann gedeckt, wenn zwei Hälften stehen: der Steckbrief
gegen die Zusage (`pruefordner_pruefen`) und die tatsächlich gelesene Eintragszahl gegen denselben
Steckbrief (`Messreihe::fahren`). Die zweite Hälfte gibt es nur für Prüfordner A und den großen
Ordner, weil nur über sie eine `Messreihe` läuft.

Für **Prüfordner B** und den **L6-Unterordner** bleibt allein die Aussage einer Textdatei neben dem
Ordner. Ein Steckbrief, der 10.000 sagt, neben einem Ordner mit 3.000 Einträgen kommt bei beiden
durch, und L4, L5 und L6 messen dann auf einem Bestand, den keine Zusage meint. Der Fall, den
`260826-1301` selbst nennt — ein hineingerutschter `.DS_Store`, ein von Hand gelöschter Eintrag —,
ist für diese beiden offen.

Am 260905 ist die eine unstrittige Hälfte des Befunds behoben: der Kommentar in
`crates/krk-bench/src/bericht.rs`, der sagte, B werde nicht gelesen. Er sagt jetzt, dass die
**Anwendung** ihn bei jedem L5-Tab- und L5-Fensterwechsel liest und `krk-bench` nicht, und dass
deshalb keine gelesene Zahl danebensteht. Die Frage nach der Deckung selbst ist eine
Nutzerentscheidung und deshalb offen geblieben.

## Optionen

1. **Es bleibt bei der Beschriftung.** Der Berichtskopf unterscheidet die zwei Lagen schon
   sichtbar: A und der große Ordner tragen „Eintraege je Lauf: N (laut Steckbrief: M)", B und der
   L6-Unterordner tragen „N Eintraege laut Steckbrief". Wer den Bericht liest, sieht, welche Zahl
   bestätigt ist.
   - Pro: kein Eingriff, keine Laufzeit, und der Bericht behauptet nichts Falsches.
   - Contra: der Unterschied ist an einer Formulierung ablesbar und nirgends erklärt; und das Gate
     urteilt weiterhin auf einem für zwei von vier Ordnern ungeprüften Bestand.
2. **Die zweite Hälfte nachziehen.** Für beide genügt ein Zählen ohne Zeitmessung:
   `einen_lauf_fahren` liefert die Eintragszahl schon, und `pruefordner_pruefen` könnte sie neben
   dem Steckbrief anfordern.
   - Pro: „gelesen == zugesagt" gilt dann für jeden der vier Ordner, und der Commit-Betreff von
     `960900d` wird wahr.
   - Contra: kostet je Lauf einen Verzeichnisdurchgang über 10.000 beziehungsweise 1.000 Einträge,
     einmal vor der ersten Runde. Das ist vor dem Messlauf und außerhalb jeder gemessenen Spanne,
     wärmt aber den Cache des Systems für Ordner B — genau das, was der verschiedene Startwert von
     A und B verhindern soll (Modulkopf `crates/krk-bench/src/main.rs`).
3. **Nur der L6-Unterordner bekommt sie.** Er wird vom Lauf selbst angelegt, seine Zahl ist mit
   1.000 klein, und L6 misst auf ihm unmittelbar.
   - Pro: die Cache-Frage aus Möglichkeit 2 stellt sich für ihn nicht in derselben Schärfe.
   - Contra: drei von vier Ordnern gedeckt ist eine Regel weniger als vier von vier, und die
     Ausnahme muss wieder erklärt werden.

## Randbedingungen

- Die Eintragszahl ist Bestandteil der Zusage: L3 gilt für 10.000, L10 für 100.000, L6 für 1.000.
- Prüfordner A und B unterscheiden sich allein im Startwert; das verhindert, dass der zweite
  Lesevorgang der Prüfsitzung schon im Cache des Systems liegt und der Kaltstart zur Hälfte warm
  gemessen wird. Jede Antwort, die B vorab liest, muss diese Wirkung mitbedenken.
- Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; eine Antwort, die Laufzeit
  kostet, kostet sie dem Nutzer.

## Empfehlung

Möglichkeit 1, mit einem Satz im Berichtskopf, der den Unterschied benennt statt ihn an einer
Formulierung hängen zu lassen. Möglichkeit 2 kauft die Deckung mit genau der Cache-Wirkung, gegen
die die zwei getrennten Startwerte gebaut sind, und L4 ist die Zusage, die diese Runde am
knappsten hält. Das ist eine Empfehlung und keine Antwort; entschieden ist sie nicht.

---
Answered: 260905-2008-orchestrator-session.md `## Die letzten fuenf Entscheidungen am 260907-2334 beantwortet` — Moeglichkeit 1: es bleibt bei der Beschriftung, und der Berichtskopf bekommt einen Satz, der den Unterschied benennt statt ihn an einer Formulierung haengen zu lassen. Die zweite Haelfte kaufte die Deckung mit genau der Cache-Wirkung, gegen die die Anlage gebaut ist; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Implemented: 260907-2350-berichtskopf-benennt-die-ungeprueften-messordner.md — der Berichtskopf trägt die Zeile „Deckung der Ordner" (`crates/krk-bench/src/bericht.rs`, `DECKUNG_DER_ORDNER`): sie nennt Prüfordner A und 100k als gelesen, Prüfordner B und den L6-Unterordner als zugesagt und nicht nachgelesen, sagt, dass ein hineingerutschter Eintrag dort nicht auffiele, und nennt die daran hängenden Zusagen bei ihrer Kennung (L4 und beide L5-Zeilen auf B, L6 und die L7-Zeile für den Ordnersprung auf dem Unterordner). Statt einer Zahl steht das Zählkommando im Satz. Gehalten wird er von `der_abnahmebericht_traegt_alle_zehn_zusagen_und_den_vollen_kopf` in derselben Datei, die die Behauptung „zwei von vier" zusätzlich am Kopf nachzählt. Die zweite Hälfte der Deckung ist wie entschieden nicht nachgezogen.
