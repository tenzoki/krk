„Kein Schreibweg an der Sperre vorbei" ist nicht typgesichert und ungeprüft

---

Der Modulkopf der Ablage sagt es als Eigenschaft der Typen zu:

> `crates/krk-core/src/ablage/mod.rs:24-26` — „**[`Zugang`] steht zwischen der Ablage und
> [`atomar::schreiben`], und das ist eine Eigenschaft der Typen und keine Verabredung in
> Kommentaren.**"

**Heute stimmt die Aussage über den Baum, aber nicht über die Typen.** Am 260813 sind alle
Aufrufstellen von `atomar::schreiben` einzeln nachgelesen, und keine schreibt an der Sperre
vorbei in den Ablageordner: `Zugang::sichern` (`ablage/mod.rs:457`), `Zugang::beiseite_legen`
(`:493`), `einstellungen::anlegen_falls_fehlt`, das einen `&Zugang` entgegennimmt
(`ablage/einstellungen.rs:184`), dazu die zwei Schreiber außerhalb des Ordners,
`crates/krk-ui/src/belegungsausgabe.rs:456` und `crates/krk-core/src/text/datei.rs:545`.

Die Tür steht trotzdem offen, und sie wird bereits benutzt:

- `pub mod atomar` (`ablage/mod.rs:101`) macht `schreiben` für jeden erreichbar.
- `Ablage::pfad` (`ablage/mod.rs:345`) und `Ablageort::datei` liefern den Pfad einer der vier
  Dateien **ohne** Durchgang.
- `crates/krk-core/tests/belegung.rs:53` nimmt genau diesen Weg schon:
  `fs::write(ablage.pfad(Datei::Belegung), keymap)`. Eine Probe, aber im selben Baum, und der
  nächste Leser hält sie für erlaubt.

Es gibt keine Probe über diese Zusage. Die Abnahme von C3.7 ist eine Zwei-Prozess-Probe
(`crates/krk-core/tests/ablage.rs:1909-1944`); sie zeigt, dass die Sperre wirkt, und nicht,
dass niemand an ihr vorbeischreibt.

---

**Schwere:** mittel. Kein heutiger Defekt, sondern eine Zusage ohne Wache an dem Strang mit
dem größten Schadenspotential: ein Schreibweg an der Sperre vorbei stellt genau das Gemisch in
der Nachbardatei wieder her, gegen das die Runde gebaut ist.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:24-34`, `:101`, `:345`,
`crates/krk-core/tests/belegung.rs:53`

**Domain:** code

## Vorschlag

Eine Erklärungs- beziehungsweise Aufrufzählung in `crates/krk-core/tests/baum.rs`, wo der
Leser alle Kisten sieht: `atomar::schreiben(` steht an genau fünf Stellen, und die Probe nennt
sie beim Namen wie `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` es tut. Eine
Aufrufzählung ist hier die richtige Sorte, weil die Zusage selbst eine über Aufrufstellen ist.

Zusätzlich zu erwägen und billiger als es aussieht: `Ablage::pfad` auf `pub(crate)` setzen. Die
drei Aufrufer außerhalb der Kiste — `crates/krk-ui/src/messmodus.rs:304` und die Proben —
brauchen den Pfad nur für Meldungen; `Zugang::pfad` deckt den Rest ab. Solange das nicht
geschieht, gehört der Satz im Modulkopf abgeschwächt: die Typen führen den **einen** Weg, sie
versperren die anderen nicht.
