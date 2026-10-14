Textstand::Unlesbar trägt einen offenen Deskriptor, den seit dem Wegfall des Zettelwegs kein Aufrufer liest

---

`krk_core::text::datei::Textstand::Unlesbar { datei: File, grund }` gibt die offene, zurückgespulte Datei an den Aufrufer zurück. Gebaut ist das für `ablage::Zugang::text_laden`, das eine unlesbare Zetteldatei aus diesem Deskriptor beiseitelegte, ohne den Pfad ein zweites Mal aufzulösen. Mit Schritt 1.3b des Plans `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md` ist `text_laden` gefallen. Die beiden verbliebenen Übersetzer des Befunds, `text::datei::oeffnen` und die Übernahme der alten Zettel in `heimordner::bereitstellen`, lassen den Deskriptor beide fallen (`..` im Muster).

---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Beleg.** `grep -rn 'Textstand::Unlesbar' crates/*/src` nach 1.3b: `text/datei.rs` (Erzeuger und `oeffnen`, beide ohne Leser des Feldes `datei`) und `heimordner/bereitstellen.rs:410` (`Textstand::Unlesbar { grund, .. }`). Die Probe `der_befund_deckt_alle_vier_ausgaenge_und_spult_zurueck` in `crates/krk-core/tests/text.rs` hält weiter, dass der Deskriptor zurückgespult ist. Einen Leser im Betrieb hat sie nicht mehr. Die Prosa in `text/datei.rs` und der Doc-Kommentar der Probe sagen das seit 1.3b ausdrücklich.

**Warum nicht in 1.3b behoben.** Der Plan gibt `text/datei.rs` diesem Schritt allein für die Prosa. Das Feld zu streichen ändert den Befund, die Zusage „der Deskriptor steht am Anfang“ und die Probe dazu. Sinnvoll ist das nur, wenn feststeht, dass kein späterer Schritt ihn braucht. Stufe 5 (`.secrets.txt`) liest eine Datei über denselben Leseweg und könnte ihn wieder brauchen.

**Zu entscheiden.** Das Feld fällt, mitsamt dem Zurückspulen und der Probenhälfte dazu, oder es bleibt als Zusage ohne Leser stehen und nennt seinen Zweck.

**Abnahme.** Entweder trägt `Textstand::Unlesbar` keinen `File` mehr, und `make check` ist grün. Oder ein Aufrufer im Baum liest ihn, oder der Doc-Kommentar des Feldes begründet, warum er ohne Leser steht.
