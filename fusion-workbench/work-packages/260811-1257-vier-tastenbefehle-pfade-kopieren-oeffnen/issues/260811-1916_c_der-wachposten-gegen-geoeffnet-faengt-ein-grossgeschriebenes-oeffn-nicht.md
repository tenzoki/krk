Der Wachposten gegen "geöffnet" fängt ein großgeschriebenes "Öffn…" nicht

---

Die Probe `die_oeffnungsmeldung_behauptet_kein_geoeffnet` (`crates/krk-ui/src/kommandos/operationen.rs:1481-1509`) hält den Wortlaut der Öffnungsmeldung fest. Ihre erste Zusicherung lautet:

```rust
// "öffn" faengt "öffnet" und "geöffnet" zugleich.
assert!(!meldung.contains("öffn"), "{meldung}");
```

`str::contains` vergleicht buchstabengenau. `"Öffnet".contains("öffn")` ist `false`: das große `Ö` ist ein anderes Zeichen als das kleine `ö`. Eine spätere Umformulierung, die den Satz mit "Öffnet …" oder "Öffnen …" beginnt, läuft an dieser Zeile vorbei.

---

**Der Wachposten ist nicht leer, und das gehört zum Befund.** Dieselbe Schleife verlangt `meldung.contains("System")` (`operationen.rs:1506`). Jede Umformulierung, die das Wort "System" fallen lässt, fällt dort auf, und die naheliegenden falschen Sätze tun genau das: "Bericht.pdf geöffnet" enthält weder "System" noch entgeht es dem ersten `assert`, weil "geöffnet" das kleine `ö` trägt. Der Weg vorbei ist deshalb eng: ein Satz, der "System" nennt **und** ein Wort mit großem `Ö` beginnt, etwa "Öffnen über das System angestoßen: Bericht.pdf".

Der Befund ist damit keine Lücke, die heute etwas durchlässt, sondern eine Zusicherung, die weniger hält, als ihr Doc-Kommentar sagt: "Die Probe hält den Wortlaut daran fest, damit ein späteres Umformulieren nicht unbemerkt mehr zusagt, als KRK weiß."

**Vorschlag für die Behebung.** Vor dem Vergleich kleinschreiben, dann fängt eine Zeile beide Schreibweisen:

```rust
let klein = meldung.to_lowercase();
assert!(!klein.contains("öffn"), "{meldung}");
assert!(!klein.contains("gestartet"), "{meldung}");
```

Dieselbe Stelle steht ein zweites Mal in `der_satz_fuer_die_leere_menge_gilt_beiden_befehlen` (`operationen.rs:1440`), wo `nichts_betroffen()` gegen "kopier" und "öffn" geprüft wird; sie trägt dieselbe Eigenschaft und gehört mit derselben Änderung nachgezogen.

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: Beide Pruefungen vergleichen jetzt kleingeschrieben, damit ein "Oeffnet …" am
Satzanfang auffaellt. Der Wachposten haelt damit, wofuer er gebaut ist.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behauptung traegt, und beide Stellen sind
nachgezogen.** `crates/krk-ui/src/kommandos/operationen.rs:1479-1486` schreibt in
`der_satz_fuer_die_leere_menge_gilt_beiden_befehlen` beide Meldungen ueber `to_lowercase()` klein,
bevor es auf „oeffn" und „kopier" prueft; `:1556-1559` tut dasselbe in
`die_oeffnungsmeldung_behauptet_kein_geoeffnet`. Der Vergleich auf „System" steht unveraendert am
ungewandelten Text, was richtig ist: er sucht ein grossgeschriebenes Wort.
