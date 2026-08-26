Zwei Statuszeilentexte des Zwischenablagesprungs stehen in ASCII-Umschrift neben Umlauten

---

Die Meldungen der Statuszeile in `tabelle.rs` schreiben Umlaute ("lässt sich nicht öffnen",
"Datenträger", "Verknüpfung"); die zwei Meldungen von `zwischenablage_springen` schreiben "liess",
"uebergeben" und "traegt". Der Nutzer sieht in derselben Zeile zwei Schreibweisen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/tabelle.rs:2409-2411`: `"{adresse} liess sich nicht an den Systembrowser
  uebergeben"`.
- `:2414-2416`: `"die Zwischenablage traegt weder einen absoluten Pfad noch eine Web-Adresse"`.
- Dagegen `:539` ("Datenträger"), `:2322` ("lässt sich nicht öffnen"), `:4952` ("Verknüpfung").

Prosa in Kommentaren ist in diesem Baum bewusst in Umschrift; Texte, die der Nutzer liest, sind es
sonst nicht.

## Umfang

`krk-ui`, `appkit/tabelle.rs`.
