Der Developer-ID-Zweig des Weitergabehinweises nennt die fehlende Beglaubigung und nicht die fehlende gehärtete Laufzeitumgebung

---

Der Developer-ID-Zweig von `sign::weitergabehinweis` (`xtask/src/sign.rs:172-177`) sagt:

```
signiert ist dieses Buendel mit der Developer-ID "…" und damit richtig. Beglaubigt ist es
nicht: bundle reicht nichts bei Apple ein und heftet kein Ticket an, und ohne Beglaubigung
weist Gatekeeper es auf einem anderen Mac ab
```

„und damit richtig" trifft die Identität und nicht die Signaturform. `bundle` signiert über
`sign::signieren` (`sign.rs:195-197`), und das ruft `signieren_mit(buendel, identitaet, &[])`
— ohne Zusatzmarken. `--options runtime` setzt allein `sign::signieren_gehaertet`
(`sign.rs:205-211`), und das ruft nur `release`.

Ein so signiertes Bündel ist nicht beglaubigungsfähig: `notarytool` weist es mit
„The executable does not have the hardened runtime enabled" zurück.

---

**Schwere:** mittel. Kein Verhalten, kein Bau, keine Probe hängt daran. Der Schaden ist
derselbe wie im Quelldatensatz: eine wahre Aussage, deren Folge sie nicht mitnennt.
**Gefunden von:** coderev, Durchsicht des Bereichs `cd0b5b7..093a6f4`
**Betroffen:** `xtask/src/sign.rs:172-177`, mittelbar `sign.rs:185-191`
**Domain:** code

## Warum es der Rede wert ist

Der Zweig benennt genau eine offene Sache — die Beglaubigung — und nennt sie abschließend
(„Beglaubigt ist es nicht: …"). Wer eine Developer-ID hat und den Satz liest, hat den
naheliegenden nächsten Schritt vor sich: dieses Bündel selbst einreichen. Der schlägt fehl,
und die Meldung von `notarytool` nennt eine Ursache, von der der Hinweis nichts gesagt hat.

Das ist die Form des Fehlschlags vom 260812, den
`shared/issues/260812-1628_c_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`
aufgenommen hat: die Ausgabe stimmt und lässt den Leser trotzdem in eine Wand laufen. Sie
tritt hier nur eine Station später ein.

**Der Schlusssatz füllt die Lücke nicht.** `sign.rs:188-191` beschreibt `cargo xtask
release` als „baut beide Mac-Architekturen und fuegt sie zusammen, signiert mit einer
Developer-ID und heftet nach der Beglaubigung das Ticket an" — auch dort steht die
gehärtete Laufzeitumgebung nicht, obwohl sie der eine Unterschied ist, den `release`
gegenüber einer selbst eingereichten Signatur bringt. Der Hilfetext in `main.rs:73-76`
nennt sie („signiert mit einer Developer-ID-Identitaet und gehaerteter
Laufzeitumgebung"); der Hinweis, den der Nutzer nach dem Lauf liest, nicht.

## Was zu tun wäre

Ein Halbsatz im Developer-ID-Zweig, der die zweite offene Sache mitnennt, etwa: „…, und
signiert ist es ohne gehaertete Laufzeitumgebung, ohne die Apple keine Beglaubigung
annimmt." Die Probe `eine_developer_id_wird_nicht_fuer_falsch_signiert_erklaert`
(`sign.rs:588-597`) nimmt eine weitere Zusicherung auf, ohne umgebaut zu werden.

Alternativ die Formulierung „und damit richtig" zurücknehmen, die die Signaturform als
erledigt ausgibt.

## Abgrenzung

Der Entwicklungszweig braucht den Zusatz nicht: dort ist die Identität selbst schon der
Sperrgrund, und die gehärtete Laufzeitumgebung ändert daran nichts.

## Herkunft

Gemeinsamer Speicher. Betrifft den Bauweg des ganzen Projekts und nicht die Directive einer
Runde.

---
Resolved: Beide Zweige nennen die fehlende gehärtete Laufzeitumgebung, und zwar nicht als
Zusatz je Zweig, sondern im gemeinsamen Teil: „Beglaubigt ist es nicht: bundle reicht nichts
bei Apple ein und heftet kein Ticket an, und signiert ist es ohne gehaertete
Laufzeitumgebung, ohne die Apple keine Beglaubigung annimmt." Die Wendung „und damit richtig"
ist gefallen; sie gab die Signaturform als erledigt aus, was dieser Datensatz genau
beanstandet.

Der Schlusssatz über `cargo xtask release` nennt die gehärtete Laufzeitumgebung jetzt
ebenfalls. Sie ist der eine Unterschied, den `release` gegenüber einer selbst eingereichten
Signatur bringt, und sie fehlte dort, obwohl der Hilfetext in `main.rs` sie führt.

**Die Bindung an den Hilfetext ist geprüft statt verabredet.** Die neue Probe
`beide_faelle_nennen_die_fehlende_gehaertete_laufzeitumgebung` hält den Schlusssatz mit einer
Zusicherung gegen `include_str!("main.rs")` an den Hilfetext: fällt die Wendung dort, fällt
die Probe. Damit können die zwei Beschreibungen desselben Befehls nicht mehr auseinanderlaufen
— genau der Fehlertyp, den diese Sitzung an anderer Stelle mehrfach vorgefunden hat.

**Nicht getan, und ausdrücklich:** `bundle` bekommt kein `--options runtime`. Ob es gehärtet
signieren soll, ist eine eigene Frage, die niemand gestellt hat und die Verhalten ändern
würde. Dieser Datensatz verlangt, richtig zu beschreiben, was ist, und das ist geschehen.

Ein Befund über diesen Datensatz hinaus: Gatekeeper stand in zwei Stärken nebeneinander, „auf
einem anderen Mac" gegen „auf jedem anderen Mac". Genommen ist die schwächere. Ein Bündel,
das ohne Quarantänemarke auf einen zweiten Mac kommt, etwa über rsync, scp oder git, wird
nicht angehalten; „jedem" war schon vor dieser Runde zu stark.

Geprüft mit `cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo test
--workspace`, Exit 0.
