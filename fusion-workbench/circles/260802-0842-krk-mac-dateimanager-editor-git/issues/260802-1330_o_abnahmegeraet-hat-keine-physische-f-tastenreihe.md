Das Abnahmegerät hat keine physische F-Tastenreihe, C3 setzt sie voraus

---

Der Spec legt in C3 fest, dass KRK die Norton-Funktionen ab Werk auf Fn+F3 bis Fn+F8 belegt. Die Begründung, festgehalten in `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`, lautet: die Fn-Kombination funktioniert auf jedem Mac ohne Systemeingriff, während die nackten Funktionstasten vom System verbraucht werden.

Die Vorprüfung vom 260802-1224 hat vor jedem Tastendruck festgestellt, dass das Abnahmegerät gar keine Funktionstastenreihe besitzt. `sysctl -n hw.model` meldet `MacBookPro15,1`; `ioreg` findet `Touch Bar Display` und `TouchBarUserDevice`. Das MacBook Pro 15 Zoll von 2018 hat statt der Tastenreihe einen Touch Bar.

Damit heißt "Fn+F3" auf genau dem Gerät, auf dem C3 abgenommen wird, nicht "zwei Tasten drücken", sondern "fn gedrückt halten und F3 auf dem Glas antippen". Ohne gehaltenes fn existiert dort überhaupt keine F3. Für eine Anwendung, deren erste Maxime die Tastatursteuerung ist, verlangt der Kopier-Befehl auf dem Abnahmegerät einen Blick nach unten auf ein Glasfeld ohne fühlbare Tastengrenzen.

---

**Warum das mehr als eine Fußnote ist.** Die beantwortete Entscheidung wählte die Fn-Kombination gerade deshalb, weil sie ohne Systemeingriff auskommt. Auf dem Abnahmegerät kehrt sich dieser Vorteil um: die Fn-Kombination ist dort die unbequemste der drei Möglichkeiten, die dem Nutzer damals vorlagen. Die Entscheidungsgrundlage kannte den Touch Bar nicht.

**Was das nicht ist.** Kein Fehler des Specs im Sinne einer inneren Widersprüchlichkeit, und kein Grund, die Entscheidung von sich aus umzustoßen. Der Nutzer arbeitet nach eigener Angabe überwiegend an einem Apple-Silicon-Mac (er nennt ihn "M2 Pro Max"), und ob dessen Tastatur eine Funktionstastenreihe trägt, ist hier nicht geprüft. Ebenso wenig geprüft ist, welche Tastatur er tatsächlich benutzt: an einem MacBook mit Touch Bar kann eine externe Tastatur hängen.

**Was zu klären ist**, in dieser Reihenfolge:

1. Welche Tastatur benutzt der Nutzer im Alltag, und hat sie eine physische Funktionstastenreihe? Ohne diese Angabe ist nicht entscheidbar, ob der Befund die Bedienung wirklich trifft oder nur das Abnahmegerät.
2. Falls er überwiegend ohne Funktionstastenreihe arbeitet: ist die Norton-Reihe auf Fn+F3 bis Fn+F8 dann noch die richtige Vorbelegung, oder gehört sie auf Kombinationen, die eine Tastatur ohne Funktionsreihe erreicht?
3. Unabhängig davon: die Belegung bleibt nach C3 frei konfigurierbar. Der Befund betrifft die Vorbelegung, nicht die Fähigkeit.

**Belege:** `spikes/fn-tasten/README.md` (Abschnitt mit den Vorabbefunden), Historie `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1224-spike-fn-tasten.md`. Die eigentliche Messung der Ereigniszustellung steht noch aus; sie braucht Tastendrücke des Nutzers.

**Aufgefallen bei:** der Vorprüfung zu C3, beauftragt vom Orchestrator in der Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1014-orchestrator-session.md`.
