Verschieben über eine Grenze nennt einen vollständigen Ersatz „nicht vollständig kopiert“
---
Beim Verschieben über eine Grenze mit „Überschreiben“ steht der Ersatz vollständig am Ziel, wenn der Tausch mit `Tausch::AltesBlieb` endet oder der Server nur Rechte und Zeit ablehnt. Beide Hinweise gehen unter dem Namen des Ziels in den Bericht. `ueber_die_grenze` zählt sie als Lücke: die Quelle bleibt, und der Bericht fügt für sie „nicht vollständig kopiert, in der Quelle geblieben“ hinzu. Die Quelle steht damit zugleich als fertig und als übersprungen im Bericht, mit einem falschen Grund.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (kein Datenverlust, die Quelle bleibt; der Grund ist falsch, und die Zusage aus `261017-0113-ueberschreiben-verwirft-einen-vollstaendigen-ersatz-wenn-nur-rechte-oder-zeit-nicht-gesetzt-werden.md`, kein Eintrag sei zugleich fertig und übersprungen, gilt auf diesem Weg nicht)
**Domain:** code
**Cross-references:** `261017-0145-reviewer-schlussdurchsicht-tausch-a97e4fbb-bis-0284ace2.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/fern.rs`, `kopie_ersetzt_ziel`, Zweig `Tausch::AltesBlieb`: `eintrag_fertig(&bezeichnung, …)`, dann `altes_ziel_melden`, das unter `ziel.bezeichnung()` überspringt. Zweig `Tausch::Getauscht`: `hinweise_nachtragen` mit `weg = Angabenweg::Bericht` überspringt ebenso unter dem Namen des Ziels.
- `fern.rs`, `ueber_die_grenze`: `seither` ist nicht leer, `selbst_genannt` ist falsch, weil der Pfad das Ziel und nicht die Quelle ist, also `ueberspringen(&bezeichnung, VorgangNichtVollstaendigKopiert)`. Text in `crates/krk-core/src/sprache/tabelle/de.rs`: „nicht vollständig kopiert, in der Quelle geblieben“.
- Dieselbe Form bestand für den Angabenhinweis ohne Konflikt schon vorher (`datei_fuellen` verbucht fertig, dann der Hinweis); die Nachdurchsicht `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.md` hat das Stehenlassen der Quelle dort als bewusst vorsichtig gelesen. Neu ist der Fall `AltesBlieb`, in dem der Ersatz vollständig ist und allein ein Rest des alten Ziels liegt.

**Behebung (Richtung)**

`ueber_die_grenze` unterscheidet „Inhalt fehlt“ von „Hinweis zu einem angekommenen Eintrag“. Ob die Quelle in den Hinweisfällen bleibt, ist eine Wahl; bleibt sie, bekommt sie einen Grund, der sagt, dass die Kopie vollständig ist und warum die Quelle trotzdem bleibt.

**Abnahme**

- Ein Verschieben über eine Grenze mit „Überschreiben“, dessen Tausch mit `AltesBlieb` endet, meldet die Quelle nicht als „nicht vollständig kopiert“.
- Kein Eintrag steht zugleich als fertig und als übersprungen im Bericht, oder `HowTo.md` und der Modulkopf sagen, wann das gewollt ist.

---
Resolved: `ueber_die_grenze` trennt Hinweise von Lücken. `crates/krk-core/src/operation/fern.rs`: `Angabenweg` heißt `Hinweisweg`, sein Zweig `Bericht { hinweise }` zählt jeden Hinweis, der in den Bericht geht; auch der Rest eines alten Ziels (`altes_ziel_melden`) geht jetzt über diesen Weg. Eine Lücke gibt es, wenn seit dem Beginn mehr Einträge im Bericht stehen, als der Weg Hinweise gegeben hat; allein dann bleibt die Quelle mit „nicht vollständig kopiert“. Nach Hinweisen allein (`Tausch::AltesBlieb`, abgelehnte Rechte und Zeit, auch ohne Konflikt) wird die Quelle entfernt, und sie steht nicht mehr zugleich als fertig und als übersprungen im Bericht. Ein Ordner mit fehlendem Kind steht weiter als kopiert und als in der Quelle geblieben; Modulkopf (Absatz „Ein Hinweis ist keine Luecke“) und `HowTo.md` sagen das. Probe: `verschieben_ueber_eine_grenze_entfernt_die_quelle_auch_wenn_allein_hinweise_bleiben` (`crates/krk-core/tests/fern_operation.rs`).
