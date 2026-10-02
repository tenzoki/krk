Zwei Proben bleiben unter `root` grün und verlieren dabei genau den Beleg, um dessentwillen sie den Prüffall über Rechte herstellen

---

`jeder_auftrag_bekommt_genau_einen_befund` und
`ein_namenstreffer_im_unterbaum_bleibt_ungelesen` in
`crates/krk-core/tests/verzeichnis.rs` entziehen einem Eintrag die Rechte und
bleiben unter `root` trotzdem grün, weil ihr Erwartungswert auf einem zweiten
Weg zustande kommt. Der Beleg fällt weg, ohne dass etwas rot würde. Das ist eine
dritte Klasse neben „schweigt" und „fällt aus", und
`260826-1302_*_schweigt-eine-probe-die-unter-root-nichts-messen-kann-oder-faellt-sie-aus.md`
kennt sie nicht; die Antwort vom 260907-0823 entscheidet sie deshalb nicht mit.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Die zwei Stellen

**`jeder_auftrag_bekommt_genau_einen_befund`.** Der Ordner `gesperrt` bekommt
`0o000` und ist leer. Erwartet wird `treffer: false`. Unter `root` liest er sich,
ist leer und liefert `false` auf dem anderen Weg. Der Kopf der Probe sagt „der
negative kommt auf drei Wegen"; unter `root` sind es zwei, und die Probe merkt es
nicht.

**`ein_namenstreffer_im_unterbaum_bleibt_ungelesen`.** Die eine Datei trägt die
gesuchte Folge im Namen, ist leer und hat `0o000`. Erwartet wird `treffer: true`,
und der Kopf schreibt aus, warum: „`treffer: true` ist damit der Beleg, dass
nicht gelesen wurde". Unter `root` ließe sich die Datei lesen, ihr leerer Inhalt
trüge die Folge nicht, und der Namenstreffer entschiede den Ordner ohnehin — die
Probe bliebe grün und der zitierte Satz wäre falsch.

## Warum die Antwort vom 260907 hier nicht greift

Die Antwort lautet: eine Probe, die ihre Zusage nicht messen kann, bricht mit
klarem Text ab. Sie ist für Proben entschieden, die unter `root` **still
übersprungen** werden oder **mit falschem Grund ausfallen**. Diese zwei tun
weder das eine noch das andere; ein Aufruf von
`gemeinsam::rechtesperre_haelt_oder_abbruch` wäre hier eine Ausweitung der
Antwort und keine Umsetzung, und deshalb steht er nicht da.

## Abnahme

Entweder tragen beide Proben denselben Abbruch wie die acht Aufrufstellen aus
diesem Durchgang, oder ihre Köpfe sagen ausdrücklich, was unter `root` von ihrer
Aussage übrig bleibt — so, wie es die zwei Verweisziel-Proben am Ende derselben
Datei bereits tun (`:3900` und `:3930` am Baum `90f352d` zuzüglich der Änderungen
dieses Durchgangs). Der Satz „`treffer: true` ist damit der Beleg, dass nicht
gelesen wurde" darf in keiner Lage unbelegt dastehen.

Gefunden bei der Erhebung zu Auftrag 2 der Aufgabe K5,
`260907-0858-drei-kleine-nutzerantworten-in-code.md`.

---
Resolved: Die erste der zwei Abnahmeformen — beide Proben tragen jetzt denselben Abbruch wie
die acht Aufrufstellen aus jenem Durchgang, `gemeinsam::rechtesperre_haelt_oder_abbruch`
(`crates/krk-core/tests/verzeichnis.rs`).

Genommen ist sie und nicht die zweite (der Kopf sagt, was unter `root` uebrig bleibt), weil
der Satz "`treffer: true` ist damit der Beleg, dass nicht gelesen wurde" nach der Abnahme
**in keiner Lage** unbelegt dastehen darf. Ein Kopf, der die Luecke einraeumt, laesst ihn
genau dort unbelegt; der Abbruch nimmt ihm die Lage. Bei `jeder_auftrag_bekommt_genau_einen_befund`
ist es dieselbe Sache: der Kopf sagt "der negative kommt auf drei Wegen", und unter `root`
waeren es zwei.

Dass das die Ausweitung ist, die dieser Datensatz benennt, steht so da und wird hier nicht
bestritten: die Antwort vom 260907-0823 war fuer still uebersprungene und fuer mit falschem
Grund ausfallende Proben entschieden, und diese zwei sind eine dritte Klasse. Die Ausweitung
faellt aber in dieselbe Richtung wie die Antwort und kostet nichts, was die Antwort geschont
haette: `rechtesperre_haelt_oder_abbruch` fuehrt genau den Zugriff aus, den die Probe verboten
hat, und sagt mit klarem Text, dass dieser Lauf die Zusage nicht messen kann.
