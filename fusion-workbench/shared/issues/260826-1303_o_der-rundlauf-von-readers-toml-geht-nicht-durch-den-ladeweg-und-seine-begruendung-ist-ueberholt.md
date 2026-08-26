# Der Rundlauf von `readers.toml` geht nicht durch den Ladeweg, und seine Begründung ist überholt

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Affected:** `crates/krk-core/tests/ablage.rs:489-494` (die Zusicherung), `:430-432` (die überholte Begründung), `:125-128` (die widersprechende Stelle in derselben Datei)
**Tree state:** `4a57028`

---

## Was ist

`alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` (`:434`) führt für
vier der fünf TOML-Dateien einen echten Rundlauf: schreiben, dann über
`Zugang::laden` beziehungsweise `einstellungen::laden` zurückholen und den Wert
vergleichen (`:477-501`). Für `readers.toml` endet der Weg bei einem rohen
Lesevorgang der Probe:

```rust
// tests/ablage.rs:489-494
assert_eq!(
    fs::read_to_string(ablage.pfad(Datei::Leser))
        .expect("readers.toml laesst sich nicht lesen"),
    LESEPROFILTEXT,
    "der Rundlauf hat readers.toml veraendert"
);
```

Zwischen dem `atomar::schreiben` bei `:462-467` und diesem `read_to_string` ruft
die Probe **keinen** KRK-Ladeweg für diese Datei an. Es gibt also nichts, was den
Wortlaut hätte ändern können; die Zusicherung kann nur fallen, wenn
`atomar::schreiben` selbst kaputt ist, und das hält
`die_nachbardatei_liegt_neben_dem_ziel_und_verschwindet_nach_dem_umbenennen`
(`:2355`) schon. Die Meldung „der Rundlauf hat readers.toml verändert" benennt
einen Vorgang, der nicht stattgefunden hat.

## Die Begründung widerspricht derselben Datei

Der Doc-Kommentar sagt (`:430-432`):

> Für `readers.toml` endet der Rundlauf beim Text und nicht bei einem geladenen
> Wert: die Ablage kennt von dieser Datei bislang nur Namen und Pfad, und wer
> ihren Inhalt auswertet, kommt mit einem späteren Schritt.

Dreihundert Zeilen darüber steht in derselben Datei das Gegenteil (`:125-128`):

> Über `Zugang::laden` gehen seit Schritt 8 der Runde 16 alle fünf.

Und der Ladeweg steht als Helfer schon bereit: `geladene_leseprofile` (`:111`),
von sechs anderen Proben derselben Datei benutzt. Der „spätere Schritt" ist
gefahren; die Begründung ist mit ihm überholt.

## Was zu tun wäre

Die vorletzte Zeile durch denselben Weg ersetzen, den die vier anderen gehen:

```rust
let zurueck = geladene_leseprofile(&ablage);
assert!(!zurueck.ist_ersetzt(), "{:?}", zurueck.ersetzung);
assert_eq!(
    zurueck.wert.iter().map(Profil::name).collect::<Vec<_>>(),
    ["eigener Ordner"],
);
```

Der Vergleich des Wortlauts darf daneben stehen bleiben — er hält die zweite
Zusage, dass der Ladeweg die von Hand gepflegte Datei nicht überschreibt —, aber
seine Meldung sollte dann sagen, was sie meint. Und der Doc-Kommentar bei
`:430-432` fällt mit weg.

**Schwere:** gering. Die Sache selbst ist von
`eine_fehlende_readers_toml_entsteht_byteweise_und_bleibt_beim_zweiten_start_liegen`
(`:2030`) gedeckt, die die Nutzerdatei sehr wohl über den Ladeweg holt. Was hier
fehlt, ist die Deckung an der Stelle, deren Name „alle" sagt, plus eine
Prosastelle, die der Datei widerspricht.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.
