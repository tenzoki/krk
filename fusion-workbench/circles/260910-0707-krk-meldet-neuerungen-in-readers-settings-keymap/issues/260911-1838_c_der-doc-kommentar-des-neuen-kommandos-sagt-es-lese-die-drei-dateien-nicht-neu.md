Der Doc-Kommentar des neuen Kommandos sagt, es lese die drei Dateien nicht neu; seit Schritt 9 tut es das im häufigsten Fall

---

`Kommando::NeuerungenZeigen` trägt in `crates/krk-core/src/tasten/belegung.rs:847-849` den
Satz „er zeigt den Bestand **vom Start** und liest die drei Dateien nicht neu". Der
Nutzerentscheid vom 260910-1600 hat das umgekehrt: hat der Start nichts erhoben — der Fall
bei jedem zweiten und jedem weiteren Start derselben Fassung —, erhebt der Befehl auf
Verlangen nach. Der Satz ist damit im häufigsten Fall falsch.

---

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** Durchsicht der Runde 24, Bereich `feecd6c..d9535c4`
**Cross-references:** `260910-1600_i_was-zeigt-das-blatt-auf-abruf-wenn-der-start-nichts-erhoben-hat.md`
**Betroffen:** `crates/krk-core/src/tasten/belegung.rs:847-849`

## Warum die Stelle stehengeblieben ist

Sie stammt aus Schritt 7 (`6cd9e74`) und damit aus der Zeit vor dem Entscheid. Schritt 9
(`aba36d6`) hat die drei anderen Stellen nachgezogen, die denselben Satz trugen — den
Schlusssatz in `ablage::neuerungen::blatttext` samt seinem Abschnitt „Der Schlusssatz sagt
nicht mehr ‚der Stand vom Start'" (`neuerungen.rs:521-539`), den Kopf von
`krk-ui/src/appkit/blaetter/neuerungen.rs:6-7` und das Feld
`AnwendungsIvars::neuerungen` (`anwendung.rs:744`) — und diese eine nicht, weil sie im
anderen Crate liegt.

Erhoben mit `grep -rn "vom Start" --include='*.rs' crates`: sechs Stellen tragen die Wendung
heute im Zusammenhang mit diesem Befehl, fünf davon richtig.

## Abnahme

Der Doc-Kommentar an `Kommando::NeuerungenZeigen` sagt dasselbe wie
`Anwendungsdelegierter::neuerungen_zeigen` (`anwendung.rs:4550-4554`): hat der Start
erhoben, zeigt jeder Abruf denselben Stand vom Start; sonst zeigt jeder Abruf, was gerade
auf der Platte steht. `grep -rn "vom Start" --include='*.rs' crates` nennt danach keine
Stelle mehr, die das Gegenteil behauptet.

---
Resolved: 78e381e — der Doc-Kommentar an `Kommando::NeuerungenZeigen` sagt jetzt dasselbe wie
`Anwendungsdelegierter::neuerungen_zeigen`: hat der Start erhoben, zeigt jeder Abruf denselben
Stand vom Start; sonst traegt der Befehl die Erhebung nach und zeigt, was auf der Platte steht.
Der wahre Teil des alten Satzes ist erhalten und an die Stelle geruckt, an der er zutrifft:
womit KRK arbeitet, aendert die Nacherhebung nicht. `grep -rn "vom Start" --include='*.rs'
crates` nennt danach keine Stelle mehr, die das Gegenteil behauptet; die fuenf uebrigen
Fundstellen waren schon richtig. `make check` mit Exit 0, alle fuenf Kommandos.
