Der Vorgangsstatus sagt „abgebrochen“, obwohl das Archiv ersetzt und die Quellen entfernt sind
---
Ein Abbruch, der nach dem letzten geschriebenen Block ankommt, also während `sync_all`, `rename` oder dem Entfernen der Quellen, hält nichts mehr auf. Das Archiv wird ersetzt, und `hinein` entfernt die Quellen ohne Abbruchfrage. Danach gibt `abschluss(steuerung)` trotzdem `Abschluss::Abgebrochen` zurück, weil das Kennzeichen steht. Die Statuszeile meldet „… abgebrochen: … übertragen“. Der Plan sagt dem Nutzer zugleich, ein Abbruch lasse das Archiv unverändert (E13). Die Zahl der übertragenen Einträge steht dabei, deshalb ist die Meldung nicht falsch, aber sie leitet fehl.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (irreführende Meldung; der Bau nennt die Grenze selbst)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/archiv.rs`, `hinein`: nach dem gelungenen `umbau::umbauen` folgt `stelle.baum_entfernen(pfand, &nie_abgebrochen)` und am Ende `abschluss(steuerung)`, das allein `steuerung.abgebrochen()` fragt.
- `crates/krk-core/src/archiv/umbau.rs`, `Ausgabe::write`: der Abbruch wirkt allein im nächsten `write`. `ersetzen` fragt ihn nicht mehr.
- `crates/krk-ui/src/kommandos/operationen.rs`: `Abschluss::Abgebrochen => satz(Text::VorgangAbgebrochen, &werte)`, und `de.rs` hat dafür „{was} abgebrochen: {uebertragen} übertragen“.

**Behebung (Richtung)**

Ist `umgebaut.ersetzt` wahr, endet der Vorgang als `Fertig`, und ein spät gekommener Abbruch bekommt eine Zeile im Bericht, etwa „Abbruch kam nach dem Ersetzen“. Dasselbe gilt für `im_einen_archiv` und `aus_archiven_entfernen`.

**Abnahme**

Eine Probe setzt das Abbruchkennzeichen in dem Augenblick, in dem der Lauf das letzte Fortschrittsereignis meldet. Danach gilt eines von zwei Paaren: der Bericht sagt `Fertig` und das Archiv ist ersetzt, oder der Bericht sagt `Abgebrochen` und das Archiv ist Byte für Byte unverändert. Ein drittes Paar kommt nicht vor.
