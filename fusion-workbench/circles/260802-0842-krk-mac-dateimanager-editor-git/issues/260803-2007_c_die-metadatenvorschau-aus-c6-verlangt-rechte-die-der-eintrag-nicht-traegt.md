Die Metadatenvorschau aus C6 verlangt Rechte, die `Eintrag` nicht trägt

---

S19 zeigt für alles, was weder Text noch Bild ist, "Metadaten mit Name,
vollständigem Pfad, Größe, Änderungsdatum, Rechten und Typ". Fünf dieser sechs
Angaben stehen in `Eintrag` aus S2. Die Rechte nicht, und der Plan sagt an
keiner Stelle, woher sie kommen.

---

**Der Nachweis.** Der Abschnitt `## Datenstrukturen` führt `Eintrag` mit
`name`, `sortierschluessel`, `groesse`, `geaendert`, `typ` und `versteckt`. Die
Umsetzung in `crates/krk-core/src/verzeichnis/eintrag.rs` folgt dem. Ein
Rechtefeld gibt es nicht, und die Attributliste des Lesers in
`crates/krk-core/src/verzeichnis/sys.rs` fragt keines ab.

Der vollständige Pfad fehlt in `Eintrag` ebenfalls, ist aber unproblematisch:
er entsteht aus dem angezeigten Ordner und dem Namen, und den Ordner führt die
Datenquelle seit S7 als eigenes Feld.

**Zwei Auflösungen, und die zweite ist vermutlich die richtige.**

1. **`Eintrag` um die Rechte erweitern.** `getattrlistbulk` liefert
   `ATTR_CMN_ACCESSMASK` im selben Aufruf, die Erweiterung kostet also keinen
   zusätzlichen Systemaufruf je Eintrag. Sie kostet aber Speicher und
   Kopieraufwand bei 100.000 Einträgen, und zwar für eine Angabe, die je
   Ordner höchstens einmal angezeigt wird. Berührt S2, dessen Struktur
   abgenommen ist, und damit auch die Messungen aus S8.
2. **Die Rechte erst beim Anzeigen erheben.** Die Vorschau zeigt genau einen
   Eintrag; ein `stat(2)` auf diesen einen Pfad ist ein Systemaufruf und
   fällt gegenüber dem Lesen der Vorschaudatei nicht ins Gewicht, das ohnehin
   auf einem Arbeitsfaden läuft. `Eintrag` bleibt so schmal, wie L10 es
   verlangt, und S2 bleibt unberührt.

`inference:` Weg 2 dürfte richtig sein, weil die Zusagen L3 und L10 an der
Größe von `Eintrag` hängen und C6 die Rechte nur für einen einzelnen Eintrag
verlangt. Gemessen ist der Unterschied nicht.

**Warum das ein eigener Eintrag ist.** Die Auflösung ändert entweder eine
abgenommene Datenstruktur oder fügt S19 einen Systemaufruf hinzu, den seine
`Änderungen` heute nicht nennen. Beides geht über das Ergänzen einer
Dateiliste hinaus.

**Dringlichkeit.** Bindet S19 und keinen Schritt davor.

**Aufgefallen bei:** der Durchsicht der Dateilisten von S9 bis S23 unter der
erweiterten Regel, `issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`.

---
Resolved: Weg 2 aus dem Defekt, umgesetzt mit S19 — die Rechte erhebt der Arbeitsfaden erst beim Anzeigen (stat(2) über std::fs::symlink_metadata auf den einen angezeigten Pfad in vorschaumodell::laden). Eintrag aus S2 bleibt unberührt, die Messungen aus S8 sind nicht betroffen.
