Zwei Abnahmekriterien aus C5 sind weder durch eine Probe belegt noch stehen sie unter Nutzerarbeit

---

Der Plan setzt als erste seiner neun Schlussbedingungen: „Alle sechsundfünfzig Abnahmekriterien
aus C1 bis C6 des Specs sind eingelöst, und jedes ist entweder durch eine Probe belegt oder steht
unter `## Nutzerarbeit`." Für C5.8 und C5.9 trifft weder das eine noch das andere zu.
`## Nutzerarbeit` Punkt 7 nennt „C5.1 bis C5.7"; C5.8 und C5.9 stehen dahinter.

---

**C5.8** verlangt, dass die Zeile „Sitzung" ihren Platzhalter zeigt, wenn `orchestrator-live.md`
unter einem anderen Namen liegt, während die sechs übrigen Zeilen der Wurzelzusammenfassung
weiter stimmen. Das Kriterium schreibt seinen Prüfweg selbst aus: „Geprüft wird, indem man
`orchestrator-live.md` unter einen anderen Namen legt."

**C5.9** verlangt, dass das **mitgelieferte** Profil nur in einer fusion-Werkbank greift.

**Was es gibt, und warum es nicht dasselbe ist.** Der zugrundeliegende Mechanismus ist belegt:
`crates/krk-core/tests/leseprofil.rs::das_feld_zieht_die_erste_fanggruppe_des_ersten_treffers`
(`:1114`) hält einen Feldbaustein, der ins Leere greift, gegen seinen Platzhalter, und
`::ohne_profiltreffer_entsteht_keine_zusammenfassung` (`:1451`) hält einen Ordner ohne Treffer.
Beide arbeiten mit einem von Hand gebauten Profilsatz. Die Zusagen aus C5.8 und C5.9 sprechen
aber über die **Auslieferungsfassung**, und `ausgelieferte()` (`:1556`) hat im ganzen Baum genau
einen Rufer: die Messung zu C6.7 (`:2123`). Was die fünf mitgelieferten Profile in einem fremden
Ordner tun, misst niemand.

**Der Preis ist klein und die Lage keine Gefahr.** Beide Kriterien sind am 260824 von Hand
nachgesehen und stimmen: kein Pfadmuster der Datei trifft ohne `fusion-workbench/` im Pfad, und
die Wurzel wird über `.fusion-setup` erkannt und nicht über ihren Namen. Ungehalten ist die
Zusage nur für die Zukunft.

**Abstellen:** zwei Proben gegen `ausgelieferte()` — ein beliebiger Ordner liefert kein Profil,
und ein Prüfordner in Werkbankgestalt ohne `orchestrator-live.md` liefert sechs Werte und einen
Platzhalter. Arbeit für den `coder`. Oder, wenn der Nutzer sie am laufenden Bündel abnehmen will,
zwei Zeilen in `## Nutzerarbeit` des Plans.

Gefunden beim Abgleich zum Abschluss der Runde 16, 260824-1852.
