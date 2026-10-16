HowTo und Kommentare des sftp-Pakets weichen vom Code ab
---
Mehrere Prosastellen des Pakets sagen etwas anderes, als der Code tut: ein Ziehen aus dem anderen Dateifenster, das es nicht gibt, eine unvollständige Liste der ausgegrauten Befehle, Kommentare aus der Zeit vor den Schritten 5 bis 7 und ein Doc-Kommentar, der an der falschen Probe klebt.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

1. `HowTo.md`, Abschnitt über das Ablegen auf ein fernes Fenster: verspricht ein Ziehen „aus dem anderen Fenster“. Die Dateiliste ist aber keine Ziehquelle; `crates/krk-ui/src` führt weder `pasteboardWriterForRow` noch `beginDraggingSession` noch `NSDraggingSource`. Plan E9 trägt denselben Satz.
2. `HowTo.md`, Liste der am fernen Ort ausgegrauten Befehle: „Rundweg“ (`EditorRundweg`) fehlt, obwohl `an_einem_fernen_ort_sind_genau_diese_befehle_ausgegraut` in `crates/krk-ui/src/kommandos/zulaessigkeit.rs` ihn führt.
3. Teilen im Kontextmenü verschwindet am fernen Ort (`appkit/tabelle.rs`, leere Pfadliste; `teilen::eintrag_anfuegen` kehrt bei leerer Liste zurück), statt sich zu melden, wie Plan E13 es verlangt. Code und `HowTo.md` stimmen überein, der Plan nicht. Die Abweichung ist nirgends vermerkt.
4. `crates/krk-core/src/tasten/belegung.rs` (Doc an `Ortsbedarf`) und `crates/krk-ui/src/kommandos/zulaessigkeit.rs`: „bis dahin meldet ihr Ausfuehrungszweig …“ bzw. „bis dahin melden sie“. Die Schritte 5 bis 7 sind gebaut.
5. `crates/krk-ui/src/appkit/anwendung.rs`, Prüfmodul: der Doc-Absatz von `der_abschluss_frischt_jeden_ort_des_vorgangs_auf` steht vor `das_ferne_umbenennen_stellt_auf_dem_hauptfaden_keinen_systemaufruf`, und die Probe, zu der er gehört, hat keinen Doc-Kommentar.

**Behebung (Richtung)**

Die Prosa dem Code nachziehen. Bei Punkt 3 einen meldenden Eintrag bauen oder die Abweichung im Narrativ des Pakets vermerken. Ob Ziehen zwischen den Fenstern gewollt ist, wird ein eigener Datensatz; der Satz in `HowTo.md` fällt bis dahin.

**Abnahme**

`grep -n 'anderen Fenster' HowTo.md` findet kein Versprechen eines Ziehens zwischen den Dateifenstern mehr. Die Liste in `HowTo.md` nennt jeden Befehl, den die Probe ausschreibt. `grep -rn 'bis dahin meldet\|bis dahin melden' crates` findet keine Stelle zu `Ortsbedarf` mehr. Jede Probe im Prüfmodul von `anwendung.rs` trägt ihren eigenen Doc-Kommentar.

---

Resolved: `HowTo.md`, „Was im Ordner auf dem Server geht“: kein Ziehen aus dem anderen Fenster mehr, statt dessen „Kopieren“ (`f5`) und „Verschieben“ (`f6`) und der Satz, dass sich aus einem Dateifenster nichts ziehen laesst; „Was nicht geht“: der Rundweg (`cmd+e`) in der Liste der ausgegrauten Befehle, und „Teilen“ steht am fernen Ort nicht im Kontextmenue. Punkt 3: Code und `HowTo.md` stimmen ueberein, die Abweichung von Plan E13 (Teilen verschwindet, statt zu melden) ist hiermit vermerkt; einen meldenden Eintrag baut diese Behebung nicht. `crates/krk-core/src/tasten/belegung.rs` (Doc an `ortsbedarf`) und `crates/krk-ui/src/kommandos/zulaessigkeit.rs` (Kommentar in `an_einem_fernen_ort_sind_genau_diese_befehle_ausgegraut`) ohne „bis dahin melden“. `crates/krk-ui/src/appkit/anwendung.rs`, Pruefmodul: der Doc-Absatz steht wieder an `der_abschluss_frischt_jeden_ort_des_vorgangs_auf`. Ob Ziehen zwischen den Fenstern gewollt ist, wurde nicht als eigener Datensatz angelegt (die Behebung durfte ausser diesen Zeilen nichts unter `fusion-workbench/` schreiben).
