Der verworfene Ausblendbefehl aus C7 hat keinen Auslöser und ist am laufenden Bündel nicht vorführbar

---

Das vierte Abnahmekriterium von C7 lautet: "Mindestens ein Dateifenster bleibt
immer sichtbar. Ein Befehl, der das letzte ausblenden würde, wird ohne
Fehlermeldung ignoriert." Der erste Satz hält. Der zweite beschreibt einen Fall,
den mit der ausgelieferten Belegung kein Tastendruck herbeiführt: die einzige
Funktion, die ein Dateifenster ausblendet, ist `zweites_fenster_umschalten`, und
sie trifft immer das rechte. Das linke bleibt stehen, also bleibt immer eines
sichtbar, und der zu verwerfende Befehl kommt nie vor.

---

Die Abweisung selbst gibt es, und sie ist geprüft:
`Fenstermodell::umschalten(Bereich::Links)` liefert `false` und ändert nichts,
gedeckt von der Prüfung `das_letzte_dateifenster_laesst_sich_nicht_ausblenden`
in `crates/krk-ui/src/fenstermodell.rs`. Sie steht dort mit Absicht und nicht in
der Belegungsdatei: eine spätere Belegung könnte sonst einen Weg dorthin öffnen,
den niemand prüft.

Was fehlt, ist der Weg selbst. Damit ist das Kriterium am laufenden Bündel nicht
einzeln nachweisbar, wie das Abnahmekriterium von Schritt 12 es für die sieben
C7-Kriterien verlangt; nachweisbar ist es allein im Modell.

Die Ursache liegt eine Ebene tiefer, in `krk_core::ablage::Sichtbarkeit`. Die
Struktur trägt Felder für die Lesezeichenleiste, das zweite Dateifenster und die
Vorschau, aber keines für das linke Dateifenster; ihr Modulkopf begründet das
damit, dass ein Feld, das nie `false` werden darf, eine Zusage wäre, die niemand
einhält. Diese Begründung trägt, und sie ist zugleich der Grund, aus dem C7 einen
Fall beschreibt, den es nicht gibt.

Zwei Auflösungen sind denkbar, und beide sind Nutzerentscheidungen:

1. **Den Spec nachziehen.** Der zweite Satz von C7-4 entfällt, weil der Zuschnitt
   der Runde ihn gegenstandslos macht. Die Zusage lautet dann allein "mindestens
   ein Dateifenster bleibt sichtbar", und die hält durch den Aufbau.
2. **Beide Dateifenster ausblendbar machen.** Der Befehl blendet dann das
   **aktive** Dateifenster aus statt immer das rechte, und trifft er das letzte
   sichtbare, wird er verworfen. Das macht den zweiten Satz wahr und kostet ein
   Feld in `Sichtbarkeit` sowie eine geänderte Bedeutung von
   `zweites_fenster_umschalten`, dessen Name dann nicht mehr passt.

Herkunft: aufgefallen bei der Abnahme von Schritt 12.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C7),
`crates/krk-ui/src/fenstermodell.rs`,
`crates/krk-core/src/ablage/sitzung.rs`,
`resources/default-keymap.toml`

---
Resolved: Nutzerentscheidung vom 260805-0000, Auflösung 1 dieses Datensatzes in ihrer milderen Form. Das vierte Abnahmekriterium von C7 behält beide Sätze, aber der zweite wird am Modell nachgewiesen und nicht am laufenden Bündel. Begründung des Nutzers: die Lage ist über die ausgelieferte Belegung nicht herstellbar, und ein Kürzel dafür zu erfinden hieße, eine Taste für einen Fehlerfall zu verbrauchen. Die Abweisung selbst bleibt, wo sie ist, in `crates/krk-ui/src/fenstermodell.rs` mit der Prüfung `das_letzte_dateifenster_laesst_sich_nicht_ausblenden`; sie steht dort mit Absicht, damit eine spätere Belegung keinen ungeprüften Weg dorthin öffnet. Verworfen ist Auflösung 2, den Befehl auf das aktive statt immer auf das rechte Dateifenster zu richten. Eingearbeitet in C7 des Specs, als Abnahmekriterium und als Festlegung; im Plan bekommt S12 eine Notiz und bleibt abgenommen. Entscheidungsdatensatz `decisions/260805-0000_a_nachweis-des-verworfenen-ausblendbefehls.md`. Sitzungsbericht `history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`.
