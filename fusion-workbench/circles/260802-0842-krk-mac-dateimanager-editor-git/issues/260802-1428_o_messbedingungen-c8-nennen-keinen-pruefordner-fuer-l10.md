Die Messbedingungen in C8 definieren nur den Prüfordner mit 10.000 Einträgen, L10 misst auf 100.000

---

Der Abschnitt `## Fähigkeiten` → `### C8: Messbare Geschwindigkeit` des Specs `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` führt unter **Messbedingungen** genau einen Prüfordner:

> Prüfordner ist ein eigens erzeugter, flacher Ordner mit 10.000 Einträgen aus gemischten Dateitypen und Größen.

Die Zusage L10 in derselben Tabelle misst aber auf einer anderen Größe:

> | L10 | Ordner mit 100.000 Einträgen | erste Bildschirmseite wie L2, vollständig 4 s warm | lineare Fortschreibung von L3 |

Für die 100.000 Einträge legt der Spec keine Zusammensetzung fest. Damit ist L10 als einzige der zehn Zusagen nicht vollständig nachprüfbar: zwei Messungen auf zwei verschieden zusammengesetzten Ordnern mit je 100.000 Einträgen sind nicht vergleichbar, und C8 verlangt ausdrücklich, dass eine Messung wiederholbar ist.

Der Rest der Messbedingungen trägt für L10 unverändert: zwanzig Wiederholungen, 95. Perzentil statt Mittelwert, getrennte Werte für kalten und warmen Cache, Referenzgerät `MacBookPro15,1`.

---

**Was zu tun ist:** den Satz unter **Messbedingungen** auf beide Größen ziehen, etwa: "Prüfordner sind zwei eigens erzeugte, flache Ordner aus gemischten Dateitypen und Größen, einer mit 10.000 und einer mit 100.000 Einträgen." Eine inhaltliche Änderung an den zehn Zahlen ist damit nicht verbunden; die Lücke betrifft allein die Beschreibung der Messbedingung.

**Vom Plan bereits aufgefangen, aber nicht geheilt.** Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` legt in Schritt S3 einen Erzeuger an, der beide Größen aus demselben deterministischen Verfahren mit festem Startwert erzeugt (`fixture --eintraege N --seed S`). Die Wiederholbarkeit ist damit technisch hergestellt. Der Spec sagt sie trotzdem nicht zu, und maßgeblich für die Abnahme ist der Spec.

**Aufgefallen bei:** der Beantwortung der Frage "Wie die Messungen aus C8 automatisiert und wiederholbar gemacht werden, einschließlich der Erzeugung des Prüfordners" aus `## Offen für den Planner`, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1428-planner-navigator-geruest-runde-1.md`.
