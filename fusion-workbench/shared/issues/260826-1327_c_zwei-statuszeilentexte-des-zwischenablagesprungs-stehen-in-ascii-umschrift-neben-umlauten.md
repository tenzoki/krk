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


---
Resolved: Beide Meldungen von `zwischenablage_springen` (`crates/krk-ui/src/appkit/tabelle.rs`) schreiben jetzt Umlaute wie die uebrigen Texte derselben Zeile: „{adresse} ließ sich nicht an den Systembrowser übergeben" und „die Zwischenablage trägt weder einen absoluten Pfad noch eine Web-Adresse". Keine Probe haelt einen der beiden Saetze wortwoertlich (`grep -rn` ueber `crates/` liefert nur diese zwei Stellen); `cargo test -p krk-ui` — exit 0.
