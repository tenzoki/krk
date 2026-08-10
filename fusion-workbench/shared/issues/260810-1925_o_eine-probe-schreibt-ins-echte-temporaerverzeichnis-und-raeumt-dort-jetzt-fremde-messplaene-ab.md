Eine Probe schreibt ins echte Temporärverzeichnis und räumt dort jetzt fremde Messpläne ab

---

Die Probe `der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung`
(`crates/krk-bench/src/messen.rs`) ruft `plan_schreiben` und damit `Messplanwaechter::neu`. Seit
der Umsetzung von Option 4 der Entscheidung
`shared/decisions/260810-1850_*_wie-kommt-der-messplan-bei-strg-c-weg-…` räumt dieser Aufruf
**jeden fremden Messplan im echten Temporärverzeichnis** ab. Ein `cargo test` löscht damit den
Messplan eines gleichzeitig laufenden Messlaufs.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Umsetzung von Option 4 in Turn 3 der Sitzung 260810-1647
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Domain:** code

## Wie es aufgefallen ist

Die neun Altbestandsdateien vom 260805 bis 260807, die der Defekt
`shared/issues/260810-1330_*_der-messplan-bleibt-liegen-wenn-eine-runde-abbricht.md` als Beleg
führte, sind nach der Umsetzung weg. Abgeräumt hat sie aber **kein Messlauf, sondern `make
check`** — über eben diese Probe. Der Beleg für die Behebung ist damit zugleich der Beleg für
diesen Defekt.

## Warum das mehr ist als eine Schönheitsfrage

Der Entscheidungsdatensatz nennt als Voraussetzung von Option 4: „es laufen nie zwei Messläufe
gleichzeitig". Diese Zusage hält, weil der Abnahmelauf KRK im Vordergrund verlangt und zwei
Läufe sich den Vordergrund nicht teilen. **Der Testlauf ist von dieser Zusage nicht gedeckt** —
er verlangt keinen Vordergrund, läuft in jeder Sitzung mehrfach, und niemand rechnet damit, dass
`cargo test` in den Zustand eines laufenden Messlaufs greift.

Die Voraussetzung ist im Doc-Kommentar von `Messplanwaechter::neu` inzwischen mit diesem
zweiten Beteiligten aufgeschrieben, statt sie zu verschweigen. Aufgeschrieben ist sie damit
richtig — beseitigt ist sie nicht.

## Denkbarer Weg

Die neue Naht ist schon da: `Messplanwaechter::in_verzeichnis` nimmt das Verzeichnis als
Parameter, und die **neue** Probe aus Turn 3
(`ein_neuer_waechter_raeumt_fremde_plaene_ab_und_laesst_den_eigenen_stehen`) benutzt sie bereits
zusammen mit einem `Wegwerfordner` und fasst das echte Temporärverzeichnis nicht an. Die
bestehende Probe müsste denselben Weg gehen. Das ist kein neuer Mechanismus, sondern die
Anwendung eines Mechanismus, den dieselbe Datei seit Turn 3 trägt.

Zu prüfen ist dabei, ob `plan_schreiben` selbst das Verzeichnis durchreichen muss oder ob die
Probe direkt auf `in_verzeichnis` gehen kann; im zweiten Fall prüft sie nicht mehr genau das,
was sie heute prüft.

## Zusammenhang

Dieses Projekt hat denselben Befund in anderer Gestalt schon mehrfach geführt:
`shared/issues/260809-1106_c_die-probenordner-der-vorschau-tragen-feste-namen-im-temporaerverzeichnis.md`
und `circles/260807-2116-…/issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.
`CLAUDE.md` hält unter „Was man nicht sieht" fest, dass Prüfordner einzelner Testläufe nicht ins
Temporärverzeichnis gehören, sondern Prozesskennung und Laufnummer tragen und sich in `Drop`
selbst aufräumen. Diese Probe ist der verbliebene Gegenfall.

## Dringlichkeit

Gering, aber nicht null. Der Schaden tritt nur ein, wenn jemand während eines Messlaufs die
Proben fährt — was in einer Agentensitzung durchaus vorkommt, weil `make check` der übliche
Abnahmeschritt ist.
