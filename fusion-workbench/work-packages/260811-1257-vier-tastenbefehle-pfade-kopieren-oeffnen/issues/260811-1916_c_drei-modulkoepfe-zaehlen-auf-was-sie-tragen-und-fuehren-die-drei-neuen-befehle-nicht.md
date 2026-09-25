Drei Modulköpfe zählen auf, was sie tragen, und führen die drei neuen Befehle nicht

---

Der Plan führt in Befund 4 fünf Stellen, die nach dieser Runde etwas Falsches zusagen, und alle fünf sind nachgezogen. Drei weitere sind es nicht, und alle drei sind Aufzählungen, die durch die Runde 4 unvollständig geworden sind:

1. **`crates/krk-ui/src/appkit/mod.rs:70-71`** — "[`zwischenablage`] hält die beiden Berührungen aus C10, das Lesen von `NSPasteboard` und die Übergabe einer Web-Adresse an den Systembrowser." Die Datei trägt seit `d23bfdb` mit `text_schreiben` eine weitere, und die ist keine aus C10.
2. **`crates/krk-ui/src/appkit/mod.rs`** — derselbe Überblick nennt jedes Modul des Verzeichnisses beim Namen und sagt, was es hält. `standardprogramm` steht allein in der `mod`-Liste (Zeile 139) und in keinem Satz darüber.
3. **`crates/krk-ui/src/kommandos/mod.rs:15-16`** — die Tabelle der fünf Module sagt über `operationen`: "Der Ablauf der Dateioperationen: Verzug, Bündelung, Texte (C4) und die Antworten des Terminal-Befehls (C11)." Dazu stehen dort jetzt sieben Funktionen für C1, C2 und C3 der Runde 4.
4. **`crates/krk-ui/src/kommandos/operationen.rs:3-8`** — "Seit Schritt 18c stehen auch die beiden Antworten des Terminal-Befehls aus C11 hier, am Fuß der Datei." Am Fuß der Datei stehen jetzt `pfadtext`, `pfadzeilen`, `kopiermeldung`, `nichts_betroffen`, `ablage_weist_ab`, `eintragsname` und `oeffnungsmeldung` dazu.

---

**Kein Fehler im Programm, sondern Drift zwischen Aufzählung und Baum.** Es ist dieselbe Sorte wie die fünf Stellen, die der Plan geführt hat, und sie ist hier nur deshalb durchgerutscht, weil der Plan bei den beiden `mod.rs` und beim Kopf von `operationen.rs` keine Prüfung verlangt hat: `appkit/mod.rs` steht in der Dateiliste von S3 mit dem Vermerk "erweitert: `mod standardprogramm;`", `kommandos/mod.rs` steht in keiner Liste.

Die zweite Stelle wiegt am schwersten. Der Überblick in `appkit/mod.rs` ist die Landkarte des Verzeichnisses; er verzichtet ausdrücklich auf jede Zahl ("nicht als Zählung", Zeile 88-99), zählt aber die Module namentlich auf. Ein Modul, das dort fehlt, ist für einen Leser dieser Karte nicht vorhanden.

**Vorschlag für die Behebung.** Je einen Satz nachtragen, in derselben Form wie die Nachbarn:
- `appkit/mod.rs`: `zwischenablage` hält das Lesen, die Übergabe einer Web-Adresse und seit der Runde 4 die Schreibseite; `standardprogramm` hält die eine Berührung aus C3 der Runde 4, die Übergabe eines Eintrags an das Programm, das das System für ihn führt.
- `kommandos/mod.rs` und `operationen.rs`: die Texte der drei Befehle der Runde 4 neben denen des Terminal-Befehls nennen, mit demselben Grund, den der Kopf für jene schon trägt (ein eigenes Modul für Meldungstexte wäre ein sechstes mit einer einzigen Frage).

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: `appkit/mod.rs` fuehrt `standardprogramm` jetzt im Uebersichtsbild und in der Prosa,
und der Satz ueber `zwischenablage` nennt die Schreibseite. `kommandos/mod.rs` und
`kommandos/operationen.rs` fuehren die Texte der drei neuen Befehle neben denen des
Terminal-Befehls.

**Eine dritte falsche Stelle kam dabei ans Licht:** derselbe Modulkopf sagte "Zweiundzwanzig
Module", gezaehlt sind 23. Mitgezogen. In den beiden anderen Koepfen hat der `coder` bewusst auf
eine Zahl verzichtet, weil sie auf demselben Weg veraltet — dieselbe Ueberlegung wie in der
offenen Frage `circles/260809-2040-…/decisions/260811-1230_*_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behauptung traegt in allen vier Punkten, die
Zahl eingeschlossen.** `crates/krk-ui/src/appkit/mod.rs` nennt `standardprogramm` im Uebersichtsbild
(Zeile 22) und in der Prosa (Zeile 78); der Satz ueber `zwischenablage` nennt die Schreibseite
(Zeile 77-80). `crates/krk-ui/src/kommandos/mod.rs:15-17` fuehrt die Texte der drei neuen Befehle
neben denen des Terminal-Befehls, `kommandos/operationen.rs:3-13` ebenso und namentlich.

**Die berichtigte Zahl ist nachgezaehlt:** `appkit/mod.rs:10` sagt „Dreiundzwanzig Module", und die
Datei deklariert 23 (`grep -cE "^(pub )?mod " = 23`, Zeilen 132-154).
