Zwei Datensätze nennen L4, L5 und L6 als betroffene Zusagen; seit dem 260907 hängt auch L7 am L6-Unterordner

---

Der Befund `260826-2155` und die daraus hervorgegangene Frage `260905-2155` zählen beide auf, welche Zeitzusagen auf den zwei ungedeckten Messordnern messen: „L4, L5 und L6 messen dann auf einem Bestand, den keine Zusage meint". Diese Aufzählung war am 260826 vollständig und ist es seit dem 260907 nicht mehr. Mit `1936a0f` misst L7 zwei Spannen statt einer, und die zweite ist der Sprung auf den L6-Unterordner (`crates/krk-bench/src/messen.rs:1306-1310`, `was: "Vorschau des ausgewaehlten Eintrags sichtbar (Ordner, 1.000 Eintraege)"`; `crates/krk-ui/src/messmodus.rs:975-987`). Damit hängt an der ungeprüften Eintragszahl des Unterordners nicht nur L6, sondern auch die L7-Zeile für den Ordnersprung.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `9e6ce8c`
**Affected:** `260826-2155_*_pruefordner-b-und-der-l6-unterordner-werden-nur-gegen-ihren-steckbrief-gehalten-und-der-kommentar-sagt-b-werde-nicht-gelesen.md` (Rumpf, erster Absatz und Abschnitt „Die zwei Hälften und wer sie bekommt"), `260905-2155_*_bekommen-pruefordner-b-und-der-l6-unterordner-die-zweite-haelfte-der-deckung.md` (Abschnitt „Frage")
**Cross-references:** `260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` — der Entscheid, der den Ordnersprung in L7 gebracht hat

## Warum das zählt

Beide Datensätze bleiben als Bestand stehen und werden weitergelesen, der eine geschlossen, der andere umgesetzt. Wer die Aufzählung als vollständig nimmt, hält die L7-Zeile für gedeckt und sucht bei einem verfehlten L7 nicht beim Bestand des Unterordners. Der Berichtskopf selbst nennt seit dem 260907-2350 alle vier betroffenen Zeilen (`crates/krk-bench/src/bericht.rs`, `DECKUNG_DER_ORDNER`); die zwei Datensätze daneben nennen drei.

## Abnahmebedingung

In beiden Datensätzen steht die L7-Zeile für den Ordnersprung neben L4, L5 und L6, oder die Aufzählung ist durch einen Zeiger auf `DECKUNG_DER_ORDNER` ersetzt, der sie nicht wiederholt.

---
Resolved: Beide Datensätze tragen seit dem 260908 einen Nachtrag, der die L7-Zeile für den
Ordnersprung neben L4, L5 und L6 stellt und für die vollständige Liste auf
`DECKUNG_DER_ORDNER` (`crates/krk-bench/src/bericht.rs`) zeigt, statt sie zu wiederholen —
also die zweite der beiden Abnahmebedingungen. Der Rumpf beider Datensätze bleibt unangetastet:
sie sind Aufzeichnungen eines Standes, und ihre Aufzählung war zu ihrer Zeit vollständig. Beim
Entscheidungsdatensatz nennt die `Implemented:`-Zeile die L7-Zeile ohnehin schon; unvollständig
war allein der Abschnitt `## Frage`, und der Nachtrag sagt es dort.
