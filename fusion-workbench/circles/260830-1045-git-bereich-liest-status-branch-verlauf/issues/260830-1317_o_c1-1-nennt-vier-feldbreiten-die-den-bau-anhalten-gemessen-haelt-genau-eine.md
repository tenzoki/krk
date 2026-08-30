C1.1 nennt vier Feldbreiten, die den Bau anhalten; gemessen hält genau eine
---
Das Abnahmekriterium C1.1 des Specs der Runde 23 sagt: „die vier Feldbreiten in
`Aufteilung::rahmen`, `Bereichsleiste::bereichsschalter`, `Aufteilung::gemessene_breiten` und
`Fenstermodell::breiten_uebernehmen` halten den Bau an, sobald `ALLE` gewachsen ist". Denselben
Satz trägt `260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`
in seinem Abschnitt „Was wirklich hält". Er stimmt für **eine** der vier.

**Gemessen am 260830-1300**, eigenständig übersetzt in einem Wegwerf-Workspace außerhalb des
Projektbaums, mit `rustc` über `cargo build`, Kante 2024. Nachgestellt wurden eine sechswertige
Aufzählung, eine auf sechs mitgewachsene `ALLE`-Liste und daneben die vier Bauformen, wie sie im
Baum stehen:

| Stelle | Bauform im Baum | Ergebnis |
|---|---|---|
| `Bereichsleiste::bereichsschalter` (`appkit/bereichsleiste.rs:423`) | Feld `[_; 5]`, gebaut aus `Bereich::ALLE.map(…)` (`:466`) | **Bau angehalten**: `error[E0308]: expected an array with a size of 5, found one with a size of 6` |
| `Aufteilung::rahmen` (`appkit/aufteilung.rs:244`) | Feld `[_; 5]`, gebaut aus einem fünfgliedrigen Literal (`:275-281`) | grün übersetzt; `panicked at 'index out of bounds: the len is 5 but the index is 5'` |
| `Aufteilung::gemessene_breiten` (`appkit/aufteilung.rs:352`) | `let mut breiten = [0.0; 5]`, gefüllt über `ALLE` | grün übersetzt; derselbe Absturz |
| `Fenstermodell::breiten_uebernehmen` (`fenstermodell.rs:920`) | Parameter `[f64; 5]` | grün übersetzt; beide Seiten bleiben fünf |

Dazu `bereichsbreiten` (`fenstermodell.rs:1056`) mit `[0.0_f64; 5]`, dieselbe Form wie
`gemessene_breiten` und derselbe Ausgang.

**Der Unterschied ist die Bauform und nicht die Feldbreite.** `ALLE.map` erzeugt ein Feld, dessen
Länge aus der Aufzählung folgt, und die Zuweisung an ein Feld anderer Länge ist ein Typfehler. Ein
Literal und ein `[0.0; N]` erzeugen ihre Länge selbst; sie bricht erst am Index.

**Warum das bindet:** C1.1 ist ein Abnahmekriterium dieser Runde, und die Bedingung 1 des Specs
stützt sich auf denselben Satz („Erst danach hält der Übersetzer, was er halten kann" ist richtig
formuliert, C1.1 ist es nicht). Ein Coder, der nach dem Eintrag in `Bereich::ALLE` die Fehlerliste
des Übersetzers abarbeitet und dann fertig ist, liefert ein Bündel, das beim Start abstürzt — laut,
aber erst, wenn jemand es startet, und kein Agent kann das.

**Abnahme:** C1.1 nennt nach der Berichtigung eine Stelle, die den Bau anhält, und drei, die es
nicht tun; der Plan der Runde zählt die drei in seinem Schritt 1 namentlich auf. Die Prosastellen
im Baum, die dasselbe behaupten, zieht C9.8 nach.
---
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
Gefunden beim Schreiben des Plans der Runde 23, bei der Prüfung der Bedingung 1 gegen den Baum.
Stand `2059138`. Verwandt: `260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`
(derselbe Mechanismus, andere Aussage: jener Datensatz sagt, dass die Liste ungesichert ist, dieser,
dass auch die Sicherung dahinter nur zu einem Viertel greift).
