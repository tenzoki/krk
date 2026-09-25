Ein alleinstehender Wagenrücklauf mitten im Text gilt der Reinigung nicht als Zeilenende
---
A3 Schritt 2 sagt: steht nach dem Abschneiden der Zeilenenden am Ende noch ein Zeilenende im Text, ist er mehrzeilig. `filtertext_aus` (`crates/krk-core/src/zwischenablage.rs`) schneidet am Ende `\n` **und** `\r` ab (`trim_end_matches(['\n', '\r'])`), prüft danach aber allein `rest.contains('\n')`. Ein Text `erste\rzweite` (Wagenrücklauf allein, wie ihn manche Terminalausgaben und alte Mac-Exporte tragen) ist damit nicht `Mehrzeilig`; der `\r` fällt in Schritt 4 als Steuerzeichen still heraus, und der Filtertext lautet `erstezweite`. Die Regel behandelt dasselbe Zeichen am Ende als Zeilenende und mittendrin nicht. Keine Probe deckt den Fall; `ein_inneres_zeilenende_ist_mehrzeilig` prüft nur `\n`.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** coder

Fix: `rest.contains(['\n', '\r'])` an derselben Stelle, und die Probe um `erste\rzweite` erweitern. Abnahme: `filtertext_aus(&Einfuegequelle::Text("erste\rzweite"))` liefert `Err(Einfuegehindernis::Mehrzeilig)`; `Name\r\n` und `Name\r` liefern weiter `Ok("Name")`.

---
Reconciled 260829-1223: weiter offen. Kein Commit zwischen `79d507a` und `8d64859` fasst die genannte Stelle an; die Lage ist am Baum nachgelesen (siehe `history/260829-1223-reconciliation.md` dieses Circles für den Beleg je Datensatz). Keine Vorbedingung des Abschlusses der Runde 21.

---
Resolved: Der vorgeschlagene Fix uebernommen und am Baum geprueft. `filtertext_aus` (`crates/krk-core/src/zwischenablage.rs`) fragt jetzt `rest.contains(['\n', '\r'])`, also nach denselben zwei Zeichen, die eine Zeile darueber am Ende abgeschnitten werden; der Kommentar daneben nennt den Grund und den alten Ausgang `erstezweite`. Der Modulkopf, Schritt 2 der fuenf, sagt es ebenso und schreibt aus, dass die zwei Zeichen an beiden Stellen dieselben sein muessen. Proben erweitert: `ein_inneres_zeilenende_ist_mehrzeilig` haelt jetzt zusaetzlich `erste\rzweite` und `erste\r\nzweite` gegen `Einfuegehindernis::Mehrzeilig`, `zeilenenden_am_ende_fallen` zusaetzlich `Name\r` gegen `Ok("Name")`. Die Abnahme des Datensatzes ist damit vollstaendig gemessen.
