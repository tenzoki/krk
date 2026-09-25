Ein Doc-Kommentar begründet „Bearbeiten" mit einem Mechanismus, den es nicht gibt

---

S4 hat den Anzeigenamen von `Funktionsbereich::Textbefehle` von „Textbefehle" auf
„Bearbeiten" umgestellt. Der Doc-Kommentar an `Funktionsbereich::name` begründet das so:

> `crates/krk-ui/src/belegungsmodell.rs:159-163` — „macOS haengt seine eigenen Textzusaetze an
> ein Menue dieses Namens, und `appkit::menue::systemzusaetze_unterdruecken` **setzt genau
> dort an**; ein anders benanntes Obermenue stellte die Zusage aus C2.13 auf eine ungepruefte
> Annahme."

**`systemzusaetze_unterdruecken` setzt nirgends an einem Menütitel an.** Die Funktion trägt
drei Namen in `NSUserDefaults` ein und rührt kein Menü an
(`crates/krk-ui/src/appkit/menue.rs:283-303`). Ihr eigener Doc-Kommentar sagt es und nennt die
Messung dazu: „AppKit liest die beiden also aus `NSUserDefaults` und nicht aus der
Bundle-Beschreibung" (`menue.rs:252-255`).

**Der zugehörige Entscheidungsdatensatz sagt es ebenfalls richtig** und ist damit die Stelle,
an der der Widerspruch sichtbar wird:

> `decisions/260813-0159_o_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`,
> Randbedingungen — „`systemzusaetze_unterdruecken` setzt heute über `NSUserDefaults` an und
> nicht über den Menütitel. Ob die Unterdrückung ohne ein Menü namens „Bearbeiten" trägt, ist
> ungemessen."

Aus einer ungemessenen Annahme ist im Code eine Tatsachenbehauptung geworden, und sie zitiert
als Beleg eine Funktion, die das Gegenteil belegt.

---

**Schwere:** gering. Der Name „Bearbeiten" ist richtig gewählt, die Umbenennung steht nicht in
Frage, und C2.13 hält am Bündel oder hält nicht — unabhängig von diesem Satz. Falsch ist die
Begründung, und `CLAUDE.md` behandelt Begründungen in Modul- und Doc-Köpfen dieses Baums als
tragend.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/belegungsmodell.rs:155-165`

**Domain:** code

## Vorschlag

Den Satz auf den Stand des Datensatzes bringen: der Name ist gewählt, weil die
Mac-Gewohnheit ihn verlangt und weil die sechs Funktionen genau die Einträge jenes Menüs sind;
ob macOS seine Zusätze an den **Titel** hängt, ist ungemessen, und
`systemzusaetze_unterdruecken` wirkt unabhängig davon über `NSUserDefaults`.

**Nicht zu verwechseln** mit der offenen Frage
`decisions/260813-0159_o_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`.
Die fragt, ob umbenannt werden **darf**; dieser Befund betrifft allein die Begründung im Code.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Der Doc-Kommentar an `Funktionsbereich::name` begruendet den Namen „Bearbeiten" jetzt mit dem, was traegt: die Mac-Gewohnheit verlangt ihn, und die sechs Funktionen tragen saemtlich `gehalten_von = "menue"` und sind genau die Eintraege jenes Menues. Ein zweiter Absatz sagt ausdruecklich, dass ungemessen ist, ob macOS seine Textzusaetze an den Menue**titel** haengt, und dass `systemzusaetze_unterdruecken` ueber `NSUserDefaults` wirkt und keinen Menuetitel kennt; er nennt diesen Datensatz und weist auf die andere, offene Frage `decisions/260813-0159_*` hin. Aus der Tatsachenbehauptung ist damit wieder die ungemessene Annahme geworden, die der Entscheidungsdatensatz selbst formuliert.
