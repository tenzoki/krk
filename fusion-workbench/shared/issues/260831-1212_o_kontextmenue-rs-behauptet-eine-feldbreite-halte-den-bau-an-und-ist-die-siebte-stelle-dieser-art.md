`kontextmenue.rs` behauptet eine Feldbreite halte den Bau an, und ist die siebte Stelle dieser Art

---
`crates/krk-ui/src/kommandos/kontextmenue.rs:204-207` sagt zu `Kontextbefehl::ALLE`:

> **Die Feldbreite steht in der Typangabe.** Ein vierter Wert haelt damit
> den Bau an, wie es die Aufzaehlungen dieses Baums durchweg tun; die
> Vollstaendigkeit der Liste selbst erzwingt der Uebersetzer nicht

Der erste Satz ist falsch, der letzte richtig, und beide stehen nebeneinander. `[Kontextbefehl; 3]` zwingt zu drei Gliedern und sagt nichts darüber, welche drei; eine vierte Variante der Aufzählung, die niemand in `ALLE` einträgt, übersetzt vorbei. Der Beweis ist im Datensatz `260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md` eigenständig übersetzt geführt.

Der Halbsatz „wie es die Aufzaehlungen dieses Baums durchweg tun" ist daneben in der Sache widerlegt: von den vier Feldbreiten hinter `Bereich::ALLE` hält gemessen genau eine den Bau, nämlich `Bereichsleiste::bereichsschalter` über `Bereich::ALLE.map(…)`; `Aufteilung::rahmen`, `Aufteilung::gemessene_breiten` und `Fenstermodell::breiten_uebernehmen` übersetzen grün und brechen zur Laufzeit am Index (`260830-1317_*_c1-1-nennt-vier-feldbreiten-die-den-bau-anhalten-gemessen-haelt-genau-eine.md`).

Die Stelle ist die siebte ihrer Art im Baum. Die sechs, die der Datensatz von 260830-1006 aufzählt, sind mit Schritt 11 der Runde 23 nachgezogen; diese hier steht nicht darin, weil sie weder von `Bereich` noch von `Fokus` handelt und deshalb außerhalb der Aussage jenes Nachzugsschritts liegt.

**Abnahmetest:** der Doc-Kommentar an `Kontextbefehl::ALLE` behauptet keine Sicherung durch die Feldbreite mehr, sondern sagt, was tatsächlich hält — die Probe `die_tafel_nennt_jeden_befehl_genau_einmal`, die er selbst schon nennt.

Erhebungsvorschrift für die Stellen dieser Art:

```sh
grep -rn "Feldbreite" crates/*/src
```

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden bei der erweiterten Erhebung aus Schritt 11 der Runde 23 (`260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 11). Die Runde hat diese Stelle nicht falsch gemacht; sie ist es seit der Runde 17.
Verwandt: `260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`, `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` (offene Nutzerfrage nach der Bauform).
