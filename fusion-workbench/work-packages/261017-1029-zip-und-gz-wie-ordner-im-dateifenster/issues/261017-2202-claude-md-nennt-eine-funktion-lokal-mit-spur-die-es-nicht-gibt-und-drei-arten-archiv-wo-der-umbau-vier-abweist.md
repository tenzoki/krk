CLAUDE.md nennt eine Funktion `Neueintrag::lokal_mit_spur`, die es nicht gibt, und drei Arten Archiv, wo der Umbau vier abweist
---
Zwei Stellen in den Archivabsätzen von `CLAUDE.md` stimmen nach den Behebungen `b79eebd2` und `c9651443` nicht mehr mit dem Code.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Severity:** Low
**Cross-references:** `261017-2202-reviewer-nachdurchsicht-archivpaket-8c8579af-bis-c9651443.md`

1. Der Absatz „Vorgänge mit einer Archivseite sind `Fernauftrag`e“ verweist für die `Lokalspur` auf `archiv::umbau::Neueintrag::lokal_mit_spur`. Diese Funktion steht nirgends im Baum (`grep -rn lokal_mit_spur crates` findet nichts). Die Spur liefert `Neueintrag::lokal`, das seit `b79eebd2` einen `Lokalbaum` mit den Feldern `eintrag`, `spur` und `ausgelassen` zurückgibt (`crates/krk-core/src/archiv/umbau.rs`, `impl Neueintrag`).
2. Der Absatz „Ein Archiv wird über einen Umbau je Vorgang geändert“ sagt: „drei Arten Archiv weist er vor der Aufstellung ab“ (verschlüsselt, verdeckt, tar mit hartem Verweis). Seit `c9651443` weist `umbauen_mit` vor der Aufstellung auch eine Archivdatei ohne Schreibrecht ab (`Umbaufehler::Schreibgeschuetzt`, gefragt über `verzeichnis::sys::schreiben_erlaubt`). Das sind vier Arten. Die Prüfung steht im Code sogar vor den anderen drei (`crates/krk-core/src/archiv/umbau.rs`, `umbauen_mit`). `HowTo.md` führt die schreibgeschützte Datei schon richtig.

Akzeptanz: `grep -n lokal_mit_spur CLAUDE.md` findet nichts mehr, und der Umbau-Absatz nennt alle Abweisungen vor der Aufstellung, die `umbauen_mit` heute trägt. Geht das ohne Zahl, ist das besser: die Zahl veraltet mit der nächsten Abweisung, wie es den anderen Zählungen in dieser Datei ergangen ist.

---
Resolved: `CLAUDE.md` nennt für die `Lokalspur` jetzt `archiv::umbau::Neueintrag::lokal` mit seinem `Lokalbaum` (`eintrag`, `spur`, `ausgelassen`); `grep -n lokal_mit_spur CLAUDE.md` findet nichts mehr. Der Umbau-Absatz nennt keine Zahl mehr, sondern verweist auf die Rückgaben in `umbauen_mit` von der Frage nach dem Schreibrecht bis `Aufstellung::neu` (Zählkommando `awk '/schreiben_erlaubt\(&ziel\)/,/Aufstellung::neu/' crates/krk-core/src/archiv/umbau.rs | grep 'return Err'`) und zählt die heutigen auf, die schreibgeschützte Archivdatei (`Umbaufehler::Schreibgeschuetzt`) zuerst.
