macOS stellt dem Menü "Bearbeiten" ein AutoFill-Untermenü dazu, das niemand bestellt hat

---

Das gebaute Hauptmenü trägt im Menü "Bearbeiten" einen Trenner und darunter einen Eintrag **"AutoFill"**, den `crates/krk-ui/src/appkit/menue.rs` nicht anlegt. Nachgesehen am 260805-1455 über `make menue`, also `target/KRK.app/Contents/MacOS/krk --menue-protokoll`:

```
menue="Bearbeiten" eintrag="Alles auswählen" kombination=cmd+a ... selektor=selectAll:
menue="Bearbeiten" trenner
menue="Bearbeiten" eintrag="AutoFill" kombination=(keines) kuerzel="" zusatztasten=1048576 zweitform=nein verdeckt=nein selektor=submenuAction:
```

Der Selektor `submenuAction:` weist es als Untermenü aus. KRK legt im Menü "Bearbeiten" genau vier Einträge an, und keiner davon ist dieser.

---

**Es ist der dritte Fall derselben Art in diesem Vorhaben**, und die ersten beiden sind behoben:

- `issues/260804-1040_*_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md` — "Close All" auf Opt+Shift+Cmd+W, gelöst über einen eigenen Selektor statt `performClose:`.
- `issues/260805-0753_*_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` — "Quit and Keep Windows" auf Opt+Cmd+Q, gelöst über einen eigenen Selektor statt `terminate:`.
- `issues/260805-0753_*_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md` — "Emoji & Symbole" und "Diktat starten", gelöst über `registerDefaults:`.

**Dieser Fall ist harmloser als die drei anderen, und das ist der Grund, warum er kein Kürzel bricht:** `kombination=(keines)`. AutoFill trägt keine Tastenkombination und liegt damit nicht außerhalb der Konflikterkennung aus C3; die Zusage, dass jede auslösende Kombination in `resources/default-keymap.toml` steht, ist unberührt. Er belegt keine Taste, die der Nutzer umbelegen wollen könnte.

**Was trotzdem dagegen spricht, ihn stehen zu lassen.** Das Menü zeigt einen Eintrag, den KRK nicht kennt und dessen Verhalten es nicht verantwortet. Für einen Dateimanager ohne Formularfelder hat AutoFill keine Bedeutung; wer ihn anklickt, bekommt ein Untermenü ohne brauchbaren Inhalt. Die Maxime "supersimpel" spricht gegen ein Menü, das mehr anbietet, als die Anwendung kann.

**Was zu tun ist.** Der `coder` prüft, ob es dafür eine Nutzervorgabe gibt, so wie `NSDisabledCharacterPaletteMenuItem` und `NSDisabledDictationMenuItem` sie für die beiden anderen Systemzusätze liefern. Ein naheliegender Kandidat ist `NSDisabledAutoFillMenuItem`; **das ist eine Vermutung nach dem Muster der beiden bekannten Schlüssel und nicht nachgeschlagen.** Greift keine Vorgabe, ist der zweite Weg derselbe wie bei "Close All" und "Quit and Keep Windows": AppKit hängt den Zusatz an einen bestimmten Selektor, und ein eigener Selektor an seiner Stelle nimmt ihm den Anker. Welcher der vier Einträge ihn trägt, ist zu messen und nicht zu raten.

**Dringlichkeit.** Gering. Kein Kürzel, keine gebrochene Zusage, kein Fehlverhalten. Fällig, wenn ein Schritt `menue.rs` ohnehin anfasst; S20 ist der nächste, der das Hauptmenü berührt.

---

**Aufgefallen bei:** dem ersten Lauf von `make menue`, unmittelbar nach dem Anlegen des Makefiles am 260805-1455. Die Marke `--menue-protokoll` aus S13c ist genau dafür gebaut worden: das gebaute Menü auslesen, statt die heute bekannten Systemzusätze aufzuzählen. Sie hat beim ersten bequemen Aufruf einen vierten gefunden.
