C8.3 nennt 98 zusätzliche Pakete; am Projektbaum gemessen sind es 101
---
Das Abnahmekriterium C8.3 des Specs der Runde 23 verlangt, die Begründung an der Versionsangabe in
der Wurzel-`Cargo.toml` nenne „die 98 zusätzlichen Pakete auf dem Bauziel". Der Plan wiederholt die
Zahl in Schritt 3. Sie stammt aus der Machbarkeitsanalyse
`shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md`, Frage 2, und ist dort an einem
Wegwerf-Workspace außerhalb des Projektbaums erhoben: „KRK heute 95, das Prüfprogramm 120, davon 22
gemeinsam".

**Am Projektbaum gemessen ist die Zahl 101.** Erhoben am 260830 nach der Aufnahme von `gix 0.87.1`,
gegen einen Auszug des Standes `4f6b880` in einem Wegwerfordner:

```sh
cargo tree --target <ziel> -e normal,build | grep -oE '[a-zA-Z0-9_-]+ v[0-9]+\.[0-9.]+' | sort -u | wc -l
```

- `x86_64-apple-darwin`: vorher 96, nachher 197.
- `aarch64-apple-darwin`: vorher 96, nachher 197. Dieselben Pakete wie beim ersten Ziel.
- Neu sind 101 Einträge; keiner fällt weg. Davon sind `gix` selbst und 50 Kisten mit dem Vorsatz
  `gix-` zusammen 51, die übrigen 50 sind fremde. Zwei davon sind weitere Fassungen von
  `hashbrown` (0.14.5 und 0.16.1) neben der schon vorhandenen 0.17.1, also 100 neue **Namen**.
- `Cargo.lock`, das alle Ziele führt, wächst von 101 auf 219 Einträge. Die Analyse nennt dafür 119
  neue; gemessen sind es 118.

Die Größenordnung der Analyse stimmt, ihre Zahl nicht. Der Unterschied kommt aus der Auflösung: im
Prüf-Workspace stand kein `syntect`, kein `zip` und kein `objc2`, und `cargo` vereinigt Fassungen
über den ganzen Baum.

**Der Coder hat die gemessene Zahl geschrieben und nicht die zitierte.** Die Begründung in der
Wurzel-`Cargo.toml` nennt 101, sagt, wie sie erhoben ist, und nennt die Abweichung zur Analyse samt
Grund. Das folgt der Regel dieses Projekts, dass eine Zahl aufgezählt oder erhoben und nie behauptet
wird; C8.3 in seinem Wortlaut zu erfüllen hieße, eine falsche Zahl in den Baum zu schreiben.

**Abnahmetest:** C8.3 nennt keine Zahl mehr, sondern die Erhebungsvorschrift, mit der sie zu
gewinnen ist — wie es E7 für die C-Freiheits-Zusage schon verlangt und wie `CLAUDE.md` es für
`Kommando`, `Wirkungsbereich` und `Art` hält. Die Analyse behält ihre 98 nach der Ortsregel: sie ist
die Aufzeichnung eines Standes, und ein Nachtrag berichtigt sie, kein Überschreiben.
