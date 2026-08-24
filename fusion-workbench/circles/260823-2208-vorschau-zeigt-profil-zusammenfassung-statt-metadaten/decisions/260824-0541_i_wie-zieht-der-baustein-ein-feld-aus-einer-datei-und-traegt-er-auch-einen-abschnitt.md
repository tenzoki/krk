# Wie zieht der Baustein „ein Feld aus einer Datei" seinen Wert, und trägt er auch einen mehrzeiligen Abschnitt?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`, `crates/krk-core/src/text/datei.rs` (`bis_zur_grenze_lesen`), `Cargo.toml` (Arbeitsbereichs-Abhängigkeiten)

---

## Question

Der Bausteinsatz trägt vier Glieder, und eines davon ist bisher nur dem Namen nach bestimmt: „ein aus einer Datei gezogenes Feld". Die sechs skizzierten Zusammenfassungen verlangen von diesem einen Baustein vier verschiedene Zugriffe. `.fusion-setup` ist einzeiliges JSON und trägt Setup-Datum und Plugin-Version als Schlüssel darin (am 260824-0541 nachgesehen); `.active-circle` ist eine nackte Zeile ohne Schlüssel; `agentstate.yaml` ist YAML und stand zum Zeitpunkt dieser Erhebung gar nicht da; `*_circle.md` trägt Kopffelder der Form `**Domain:** code` und daneben die Directive als Abschnitt über mehrere Zeilen. Die Antwort entscheidet, wie viele der sechs Zusammenfassungen die Runde überhaupt einlösen kann, und sie entscheidet mit, ob die Runde eine fremde Kiste aufnimmt. Sie muss vor den Abnahmekriterien fallen, weil die Zusammenfassung eines einzelnen Circles ohne sie kein Kriterium bekommt.

## Options

1. **Eine Zeile mit einem Präfix** — Das Profil nennt einen Präfix; die erste Zeile der Datei, die ihn trägt, gibt den Rest der Zeile als Wert.
   - Pros: Kein Formatwissen, keine fremde Kiste, in einer Handvoll Zeilen gebaut und ohne Fenster prüfbar. Trägt die Kopffelder von `*_circle.md`, die Zeilen von `agentstate.yaml` und, mit leerem Präfix, die nackte Zeile von `.active-circle`.
   - Cons: Trägt die JSON-Felder von `.fusion-setup` nicht, weil dort alles in einer Zeile steht. Trägt die Directive nicht. Die Zusammenfassung eines Circles zeigt dann Zustand, Vorhandensein von Spec und Plan und die Zählungen, nicht aber die Directive, und die Wurzelzusammenfassung nennt weder Setup-Datum noch Plugin-Version.
2. **Eine Zeile mit einem Präfix, oder ein Abschnitt zwischen zwei Überschriften** — Derselbe Baustein trägt zwei Zugriffe: entweder die Zeile aus Möglichkeit 1, oder den Text von einer genannten Markdown-Überschrift bis zur nächsten Überschrift derselben oder einer höheren Stufe.
   - Pros: Löst fünf der sechs Zusammenfassungen ganz ein, die Directive eingeschlossen, und bleibt ohne fremde Kiste. Der Abschnittszugriff ist genau die Form, in der die fusion-Datensätze ihren Inhalt ablegen, also zahlt er sich am Beispielfall der Runde aus.
   - Cons: Die JSON-Felder von `.fusion-setup` bleiben auch hier draußen; Setup-Datum und Plugin-Version fehlen der Wurzelzusammenfassung. Der Baustein trägt zwei Zugriffsarten statt einer, und die Definitionsdatei muss beide unterscheidbar schreiben.
3. **Ein regulärer Ausdruck mit einer Fanggruppe** — Das Profil nennt einen Ausdruck; die erste Fanggruppe des ersten Treffers ist der Wert.
   - Pros: Trägt alle vier Zugriffe und damit alle sechs Zusammenfassungen vollständig, JSON eingeschlossen.
   - Cons: Der Baum führt heute keine Kiste für reguläre Ausdrücke (`Cargo.lock` am 260824-0541 nachgesehen), also kostet die Möglichkeit eine fremde Kiste oder eine eigene Maschine. Und sie holt genau die Ausdruckssprache in die Definitionsdatei zurück, die der Nutzer am 260823 für die Zusammenfassung abgelehnt hat, nur an einer anderen Stelle.

**Berichtigung 260824-0910.** Die Kostenangabe der Möglichkeit 3 ist in ihrer ersten Hälfte falsch. Der Satz „Der Baum führt heute keine Kiste für reguläre Ausdrücke (`Cargo.lock` am 260824-0541 nachgesehen)" trifft nicht zu: `krk-ui` führt über `syntect` bereits `fancy-regex` 0.16.2 als gewöhnliche Abhängigkeit, und darunter stehen `regex-automata` 0.4.18, `regex-syntax` 0.8.11, `aho-corasick` 1.1.5 und `memchr` 2.8.3. Nachgezählt am 260824-0910 mit `cargo tree -p krk-ui -e normal` und gegen `Cargo.lock` im Stand vor dieser Runde (`git show HEAD:Cargo.lock`), damit die Zahlen nicht von der Aufnahme aus Schritt 1 dieses Plans stammen. Dieselbe Auskunft stand schon an einer Stelle, die dieses Projekt als bindend führt: die Wurzel-`Cargo.toml` zählt in ihrer Begründung zu `syntect` genau diese fünf Pakete namentlich auf. Die erste Erhebung hat auf den Namen `regex` gesehen und die vier Pakete übersehen, die keinen solchen Namen tragen; der Befund ist als `issues/260824-0600_*_der-entscheidungsdatensatz-zum-regulaeren-ausdruck-sagt-der-baum-fuehre-keine-solche-kiste-er-fuehrt-eine.md` geführt.

**Was die Berichtigung ändert, und was nicht.** Die Wahl vom 260824-0555 steht: Möglichkeit 3 bleibt die Antwort dieses Datensatzes. Sie kippt durch die Berichtigung nicht, sie wird billiger — und nicht umsonst. Die Kiste selbst kommt weiterhin hinzu, ihr Unterbau steht bereits im Bündel. Gewählt hat der Plan `regex` 1.x und nicht das vorhandene `fancy-regex`, und der Grund steht in der Zeile `**Decidability:**` im Kopf des Plans: C2.8 verlangt eine Laufzeitzusage, die vom Muster unabhängig ist. `regex` arbeitet mit endlichen Automaten, seine Laufzeit ist linear in der Länge der Eingabe, gleich welches Muster in der `readers.toml` steht. `fancy-regex` wertet rückverfolgend aus und kappt seine Arbeit über eine Schrittgrenze; es nähert die Zusage an, statt sie zu geben. Die zweite genannte Kostenseite bleibt unberührt: die Ausdruckssprache kommt in die Definitionsdatei, und der Nutzer hat das am 260824-0555 in Kauf genommen.

**Der Wortlaut der Cons-Aufzählung bleibt absichtlich stehen und ist nicht überschrieben.** Er belegt, auf welcher Grundlage am 260824-0555 gewählt wurde; ein stilles Umschreiben nähme diese Auskunft weg und ließe die berichtigte Fassung später als den ursprünglichen Wortlaut lesen. Der Marker dieses Datensatzes bewegt sich mit der Berichtigung nicht: er steht auf `_a_` und geht erst mit der Umsetzung auf `_i_`.

## Constraints

Der Nutzer hat am 260823 einen festen Bausteinsatz gewählt und ausdrücklich keine Ausdruckssprache. Jede fremde Kiste dieses Projekts trägt ihre Begründung in der Wurzel-`Cargo.toml`, und `Cargo.lock` führt auf dem Bauziel kein `cc` und außer `windows-sys` kein `-sys`-Paket; eine Aufnahme muss das halten. Gelesen wird über den vorhandenen Weg `krk_core::text::datei::bis_zur_grenze_lesen`, der die Datei am offenen Deskriptor prüft und ihn vor dem nächsten Kandidaten freigibt; ein zweiter Leseweg entsteht nicht.

**Berichtigung 260824-1224.** Der letzte Satz der Constraints nennt den falschen Leseweg. `bis_zur_grenze_lesen` **weist** eine Datei über der Grenze ab, statt sie anzulesen (`crates/krk-core/src/text/datei.rs`, Zweig `angaben.len() > grenze`), und C6.6 des Specs verlangt das Anlesen: „Der Titel und das Feld entstehen aus diesen Bytes." Am Bestand gemessen ist der Unterschied entscheidend und nicht theoretisch: der größte Circle-Datensatz dieser Werkbank ist 119.614 Bytes groß bei einer Grenze von 64 KB, während seine Zeile `## Directive` bei Byte 222 steht. Mit `bis_zur_grenze_lesen` zeigte gerade dieser Circle keine Directive. Schritt 4 des Plans hat deshalb `krk_core::text::datei::anlesen` als dritte Hülle an derselben Tür gebaut, und Schritt 6 ruft sie; `bis_zur_grenze_lesen` hat in der Zusammenfassung keinen Rufer.

**Was die Berichtigung ändert, und was nicht.** Die Zusage der Bedingung steht unverändert: gelesen wird über den Deskriptor und nicht über den Pfad, und ein zweiter Öffnungsweg entsteht nicht. `anlesen` geht durch dieselbe eine Tür `verzeichnis::sys::ohne_warten_oeffnen`, prüft den Typ am `fstat` des offenen Deskriptors und gibt ihn frei, bevor der nächste Kandidat drankommt. Berichtigt ist allein, dass die Bedingung eine einzelne Funktion nannte, wo sie die Tür meinte; sie bindet jetzt die Hüllen in `krk_core::text::datei` und nicht eine von ihnen.

**Der ursprüngliche Wortlaut der Constraints bleibt absichtlich stehen und ist nicht überschrieben**, aus demselben Grund wie die Cons-Aufzählung darüber: er belegt, auf welcher Grundlage am 260824-0555 gewählt wurde. Der Marker dieses Datensatzes bewegt sich mit der Berichtigung nicht; er steht auf `_a_` und geht erst mit der Umsetzung auf `_i_`. Der Befund ist `issues/260824-1014_*_c3-14-nennt-bis-zur-grenze-lesen-als-den-leseweg-und-schritt-4-hat-anlesen-gebaut.md`.

## Recommendation

Möglichkeit 2. Sie löst den Beispielfall der Runde bis auf zwei Angaben der Wurzelzusammenfassung ein, ohne eine fremde Kiste aufzunehmen und ohne die abgelehnte Ausdruckssprache durch die Hintertür zu holen. Die zwei fehlenden Angaben aus `.fusion-setup` sind Setup-Datum und Plugin-Version, also die zwei am wenigsten veränderlichen der sieben Angaben an der Wurzel; wer sie später will, stellt die JSON-Frage als eigene Runde, ohne dass am Bausteinsatz etwas zurückzubauen wäre.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:67 — Regulärer Ausdruck mit Fanggruppe (Möglichkeit 3); trägt alle sechs Fälle, kostet eine fremde Kiste.
Implemented: 260824-1849, Commits `abecfb2` (Schritt 1), `f013227` (Schritt 3) und `abe1a31` (Schritt 6). `regex` 1.13.1 steht unter `[workspace.dependencies]` mit Begründung; der Baustein `feld` nennt eine Datei über ein Namensmuster und zieht die erste Fanggruppe des ersten Treffers, und ein Feldmuster ohne genau eine Fanggruppe wird beim Laden abgewiesen. Belegt durch `crates/krk-core/tests/leseprofil.rs::das_feld_zieht_die_erste_fanggruppe_des_ersten_treffers` und `::ein_feldmuster_ohne_genau_eine_fanggruppe_nimmt_der_zeile_ihren_baustein`.
