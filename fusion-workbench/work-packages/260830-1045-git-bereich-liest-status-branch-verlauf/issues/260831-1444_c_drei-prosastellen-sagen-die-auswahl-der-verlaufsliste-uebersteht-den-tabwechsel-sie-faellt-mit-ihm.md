Drei Prosastellen sagen, die Auswahl der Verlaufsliste überstehe den Tabwechsel; sie fällt mit ihm

---
Der Nutzerentscheid vom 260831 (`decisions/260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md`, Möglichkeit 2) legt die Auswahl in das `Gitmodell`, und drei Stellen im Baum geben als Grund an, dass sie damit den Tabwechsel übersteht:

- `crates/krk-ui/src/appkit/git.rs:76-79` — „es gibt **ein** Gitfenster und **ein Gitmodell je Tab**, also übersteht die Auswahl den Tabwechsel und den Wechsel des aktiven Dateifensters"
- `crates/krk-ui/src/tabs.rs:199-201` — „damit sie den Tabwechsel übersteht"
- `crates/krk-ui/src/gitmodell.rs:129-132` — dieselbe Begründung, auf den Datensatz verweisend

Der Tabwechsel wirft sie weg. `Tabliste::waehlen` (`tabs.rs:665-667`) ruft `gitlauf_nachziehen_an(verlassen)`, und dessen dritte Zeile ist `self.tabs[stelle].gitmodell.zuruecksetzen()` (`tabs.rs:1202`), unbedingt und vor jeder Bedingungsprüfung. `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()` (`gitmodell.rs:95`) und nimmt Kopf, Verlauf, Zusammenfassung **und die Auswahl** mit. Beim Zurückwechseln entsteht der Verlauf neu, und die Auswahl steht auf `None`.

Der Wechsel des **aktiven Dateifensters** übersteht sie dagegen, weil jede `Tabliste` ihren eigenen Lauf hält; die halbe Aussage trifft also zu.

**Abnahmetest:** entweder überlebt die Auswahl den Tabwechsel — dann darf `gitlauf_nachziehen_an` das Modell des verlassenen Tabs nicht zurücksetzen —, oder die drei Prosastellen nennen als Grund, was zutrifft (der Wechsel des aktiven Dateifensters), und der Datensatz bekommt einen Nachtrag. Welche der beiden Antworten gilt, ist eine Nutzerfrage: sie ändert das Verhalten, das der Entscheid vom 260831 begründet hat.

**Nicht geschlossen, und warum:** 260831, beim Abarbeiten der Prosabefunde der Durchsicht. Der Baum ist einzeln nachgelesen und bestätigt den Befund: `Tabliste::waehlen` ruft `gitlauf_nachziehen_an(verlassen)`, dessen dritte Zeile `self.tabs[stelle].gitmodell.zuruecksetzen()` unbedingt und vor jeder Bedingungsprüfung steht, und `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()`. Die Auswahl fällt mit dem Tabwechsel, in beide Richtungen. **Die Prosa ist damit nicht das Problem:** sie gibt den Wortlaut des Nutzerentscheids vom 260831-0120 wieder („sie übersteht damit den Tabwechsel"), und sie an den Baum anzugleichen nähme eine Nutzerentscheidung stillschweigend zurück. Die Abnahme dieses Datensatzes nennt beide Wege und sagt selbst, dass die Wahl zwischen ihnen eine Nutzerfrage ist. Sie ist als Datensatz gefilt: `260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`. Die drei Prosastellen stehen bis zur Antwort unverändert.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23, beim Lesen von `Tabliste::waehlen` gegen den Modulkopf von `appkit/git.rs`.

---
Resolved: 260831-1755 hat der Nutzer den zweiten Weg dieser Abnahme gewählt — die Prosa zieht nach, das Verhalten bleibt (`260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`, Möglichkeit 2). Keine Zeile Code. Die drei genannten Stellen — `crates/krk-ui/src/appkit/git.rs` (Modulkopf `# Die Auswahl wohnt im Gitmodell und nicht hier`), `crates/krk-ui/src/tabs.rs` (Doc-Kommentar von `Tabinhalt::gitmodell`) und `crates/krk-ui/src/gitmodell.rs` (Kommentarblock vor `impl Gitmodell`) — sagen jetzt **beide** Hälften: die Auswahl übersteht den Wechsel des aktiven Dateifensters, weil jede `Tabliste` ihr eigenes Gitmodell hält, und sie fällt mit dem Tabwechsel, weil `Tabliste::waehlen` für den verlassenen Tab `gitlauf_nachziehen_an` ruft und das dessen Gitmodell unbedingt zurücksetzt. **Eine vierte Stelle ist dabei aufgefallen und mitgezogen:** der Modulkopf `# Ein Gitmodell je Tab` in `crates/krk-ui/src/gitmodell.rs` begründete die Heimat am Tab damit, ein Modell beim Fenster „hätte den Stand des verlassenen Tabs schon weggeworfen, bevor der Nutzer zurückwechselt" — genau das tut der gebaute Stand, also trug der Absatz ein Unterscheidungsmerkmal vor, das nicht unterscheidet; er nennt jetzt den Grund, der trägt (ein Stand für zwei Dateifenster), und nimmt den Tabwechsel ausdrücklich aus. Spec und Plan der Runde tragen die Aussage nicht; die Aufzeichnungen (Durchsicht, History) behalten sie nach der Ortsregel. `make check` — exit 0. Beleg: `260831-1820-coder-der-dreizehnte-befund-die-prosa-folgt-dem-baum.md`.
