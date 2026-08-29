# Shaper, user-direct: Spec der Runde 21, Einfügen in den Filter und `*` als Platzhalter

**Datum:** 2026-08-29, 10:47 bis 10:58
**Filed by:** shaper (user-direct, vom Orchestrator dispatched), Kai Stalmann
**Modus:** user-direct, Domain code, aktiver Circle `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/` (`_t_circle.md`)
**Weisung:** autonom bauen, keine Klärungsrunde, Lücken nach Muster als Festlegungen; das Spec-Tor gilt als vorab freigegeben.
**Status:** Complete

## Eingaben

1. Die Directive und die sechs Festlegungen des Grounding snapshot im Circle-Datensatz (Einfügen per `cmd+v`).
2. `shared/backlog/260829-0842_*_dateilistenfilter-versteht-stern-als-platzhalter.md`: Nutzerwahl 1 von 3 (Glob im ganzen Filtertext), dazu die Vorgaben des Orchestrators (kein `?`, kein Escape, `*` an Anfang und Ende ohne Wirkung, Inhaltsfilter mit demselben Muster, kein Regex).
3. Der Baum auf `c6c86cb`, insbesondere die vier Commits der Runde 22 (`4455af7..1644ada`): `dateiablage_ausfuehren`, `zulaessigkeit::dateiablage_zulaessig` mit der privaten Aufzählung `Anspruch`, `validateMenuItem:` mit dem Zweig für `copy:` und `cut:`, die Probe `dateiablageproben`, die `paste:` als unbeantwortet hält.

## Was gegen den Baum nachgelesen wurde

- Die Grundlage des Datensatzes sagt „`copy:` bleibt unbeantwortet"; seit der Runde 22 ist das falsch. Der Spec übernimmt das Muster der Runde 22 für `paste:` (A1, A6, A9) und nennt die Prosastellen, die nachziehen (C4.5).
- Die zehn Zeitzusagen: keine Messstrecke setzt einen Filtertext (`messen.rs`, `messmodus.rs`; Prüfung wie im Datensatz `shared/decisions/260826-0923_*_…`). L7 und L10 hängen nicht am Filterpfad. Der Vergleich läuft hinter dem Zweig „steht ein Filtertext?".
- Der eine Vergleich `traegt_die_folge` hat drei Rufer im Kern, die Zählprobe `crates/krk-core/tests/verzeichnis.rs:3226` nennt sie mit Namen. Die Tippsuche der Belegungsansicht hat ein eigenes `contains` (`belegungsmodell.rs:568`) und bleibt wörtlich (B9).
- `traegt_ein_dateiname` nimmt `:` und `*` an; der Doppelpunkt fällt allein beim Einfügen (A3, überstimmbar).
- Die Statuszeile trägt sieben Ränge; die Abweisungsmeldungen treten in `Befehlsantwort`, das geglückte Einfügen meldet nichts (A5).

## Festlegungen nach Muster, die der Datensatz nicht trug

A7 (anhängen als eine Änderung, Anzeige zieht einmal nach), A8 (kein „Einfügen rückgängig"), A9 (Zulässigkeit ohne Ablageinhalt), A11 (zwei Sorten), A12 (ein Weg hinein), A13 (Namen bleiben); B1 bis B9 für den Platzhalter, darunter B6: das `*` zählt nicht zur Inhaltsschwelle, weil die Schwelle die Aussagekraft der Eingabe misst und ein `*` keine trägt. Keine Festlegung widerspricht der Directive; keine Klärungsrunde zurückgegeben. Keine Entscheidung wurde zurückgestellt, kein Entscheidungsdatensatz gefilet; kein Defekt gefunden, der nicht schon offen wäre (Leertaste `260816-2144`, die zwei Low-Befunde der Runde 22).

## Ergebnis

- Spec: `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/planning/260829-1052_o_spec-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, 52 Abnahmekriterien in sieben Fähigkeiten, 22 Festlegungen, acht Constraints.
- Backlog-Eintrag `shared/backlog/260829-0842_*_dateilistenfilter-versteht-stern-als-platzhalter.md` auf `_c_` gesetzt, `Promoted:`-Zeile angehängt.
- Der Circle-Datensatz ist nicht angefasst; `**Active spec/plan:**` setzt der Orchestrator. Nichts committet.
