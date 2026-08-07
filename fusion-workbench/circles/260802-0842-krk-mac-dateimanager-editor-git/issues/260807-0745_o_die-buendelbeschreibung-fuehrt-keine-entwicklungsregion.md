Die Bündelbeschreibung führt keine Entwicklungsregion

---

`resources/Info.plist` führt seit dem 260807 den Schlüssel
`CFBundleLocalizations` mit `de` vor `en`, nicht aber `CFBundleDevelopmentRegion`.
`Bundle.main.developmentLocalization` liefert deshalb `nil`.

---

**Folgenlos für das, wofür der Schlüssel eingeführt wurde.** Die Byte-Angaben
sind seit dem 260807 deutsch, gemessen am gebauten Bündel; daran ändert die
fehlende Entwicklungsregion nichts. `preferredLocalizations` liefert `["de"]`.

**Wofür sie trotzdem steht.** `CFBundleDevelopmentRegion` sagt, in welcher
Sprache die Zeichenketten des Programms ursprünglich geschrieben sind. Sie ist
der Rückfall, wenn ein System keine der angebotenen Sprachen spricht, und sie
steht in den Angaben, die das System und der Finder über ein Bündel führen.
Für ein Programm, dessen Prosa durchgängig deutsch ist, wäre `de` die richtige
Angabe.

**Wo es hingehört.** S23 baut das Auslieferungspaket und ist der Schritt, der
die Bündelbeschreibung als ganze abnimmt. Dort gehört die Frage geprüft, nicht
in einen eigenen Handgriff nebenbei.

**Ausführender:** `ontocoder`. `resources/Info.plist` ist eine
Bündelbeschreibung, keine Programmdatei.

**Dringlichkeit.** Gering. Kein Nutzer sieht es, kein Abnahmekriterium ist
berührt, keine der zehn Zeitzusagen aus C8 betroffen.

**Aufgefallen bei:** der Umsetzung von D8, Turn 25 der Sitzung 260806-2257,
`history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1215_c_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
