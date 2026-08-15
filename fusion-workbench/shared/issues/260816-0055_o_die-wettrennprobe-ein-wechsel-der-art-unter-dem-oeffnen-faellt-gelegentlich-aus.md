Die Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` fällt gelegentlich aus

---

`crates/krk-core/tests/text.rs`, Probe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`. Sie belegt, dass die Typprüfung am
Deskriptor ein Wettrennen zwischen Öffnen und Tauschen aushält, und läuft dafür 20.000
Durchläufe gegen einen Tauscherfaden.

**Am 260816 zweimal von acht Läufen ausgefallen**, danach in fünfzehn weiteren Läufen —
davon einer unter künstlicher Volllast — nicht wieder. Die Laufzeit derselben Probe streut
zwischen 6,9 s und 15,2 s; die beiden Ausfälle lagen bei 15,0 s und 15,1 s, also genau an
der Notbremse `recv_timeout(Duration::from_secs(15))`.

Die Probe hat drei Zahlen, die zusammenpassen müssen: `DURCHLAEUFE = 20_000`,
`MINDESTENS_GETAUSCHT = 2_000` und die 15 Sekunden. Auf einem belasteten Gerät reicht die
Zeit für die Durchläufe nicht, und die Probe meldet dann „das Oeffnen haengt an der
benannten Roehre" — eine Aussage über den Prüfling, obwohl es eine über das Gerät ist.

---

**Schwere:** niedrig für den Prüfling, mittel für die Abnahme. Der Prüfling ist nicht
betroffen; betroffen ist `make check`, das ohne Zutun rot wird und damit als Torwächter an
Aussagekraft verliert. Ein Ausfall, der zweimal von acht kommt, wird beim dritten Mal
weggeklickt.
**Gefunden von:** coder, bei der Abnahme des Aufschubs vom 260816-0040 (die Änderung
berührt `krk-core` nicht)
**Betroffen:** `crates/krk-core/tests/text.rs`
**Domain:** code

## Was zu entscheiden ist

Die Notbremse trennt heute zwei Fälle nicht: „das Öffnen hängt" (der Defekt, den die Probe
sucht) und „das Gerät war langsam" (kein Defekt). Wer sie anhebt, verlängert jeden Lauf;
wer die Durchläufe senkt, schwächt den Beleg. Ein dritter Weg wäre, den Ausfall an der
Frage aufzuhängen, die die Probe wirklich stellt — ob **ein einzelner** Aufruf hängt —,
statt an der Gesamtdauer.

---

## Nachmessung des Orchestrators vom 260816-0100

Die Quote ist deutlich schlechter als „gelegentlich", und der Ausfall hängt
nicht an der Änderung dieser Runde:

| Stand | Läufe | Ausfälle |
|---|---|---|
| Arbeitsbaum mit dem Aufschub (T4) | 5 | 4 |
| Unveränderter HEAD `ca81e82`, dieselbe Maschine, dieselbe Minute | 3 | 2 |

Jeder Ausfall endet nach 15,0 Sekunden, also an der Notbremse, und nicht an
einer Zusage der Probe. Der Vergleich am unveränderten HEAD ist mit
`git stash push` über die drei geänderten Codedateien gefahren; die Probe liegt
in `krk-core` und kann von einer Änderung in `krk-ui` ohnehin nicht erreicht
werden.

**Die Folge wiegt schwerer als die Probe.** `make check` ist das Abnahmekommando
dieses Projekts, und es fällt derzeit in rund zwei Dritteln der Läufe aus einem
Grund aus, der mit dem Geprüften nichts zu tun hat. Ein Tor, das zufällig
schließt, wird umgangen. Verwandt: `shared/issues/260815-1019_o_die-wettrennprobe-des-oeffnens-ist-lastabhaengig-und-ihre-marge-traegt-keinen-parallelen-bau.md`.
