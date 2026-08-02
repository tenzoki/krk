Frage 7 und Schritt 5 widersprechen sich: erzeugt der Bündelbau die Signaturidentität oder nicht?

---

Zwei Stellen im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` sagen Gegenteiliges über dieselbe Sache.

**`### Frage 7`** schreibt: "Für die Entwicklung **erzeugt** S5 eine lokale, selbstsignierte Code-Signing-Identität im Schlüsselbund."

**Die `Änderungen` von Schritt 5** schreiben, der Schritt sucht eine vorhandene Identität und "bricht mit einer Anleitung zu ihrer Erzeugung ab, wenn auch die fehlt".

Der Fall ist bei der Umsetzung eingetreten: das Referenzgerät hatte null Identitäten (`security find-identity -p codesigning` meldete `0 identities found`). Unter der Lesart von Frage 7 hätte der Schritt eine anlegen müssen; unter der Lesart von Schritt 5 bricht er ab. Das Abnahmekriterium verlangt aber, dass `codesign` die Identität nennt — durch den Schritt allein war es damit nicht erreichbar.

---

**Wie es umgesetzt wurde.** Der `coder` hat die speziellere Stelle umgesetzt: `cargo xtask bundle` legt nichts im Schlüsselbund an, sondern bricht mit einer Anleitung ab. Die Anleitung steht in `README.md`, Abschnitt "Entwicklungsidentität anlegen", mit zwei Wegen: über den Zertifikatsassistenten der Schlüsselbundverwaltung und über die Kommandozeile.

Das ist die richtige Wahl, und zwar nicht nur, weil die speziellere Stelle gewinnt: ein Bauwerkzeug, das ungefragt Schlüsselmaterial in den Anmeldeschlüsselbund des Nutzers schreibt, überschreitet seine Zuständigkeit. Der Nutzer soll wissen, dass auf seinem Gerät ein Signierschlüssel entsteht.

**Was zu tun ist.** Der `planner` streicht die Zusage in `### Frage 7` und zieht sie auf den umgesetzten Stand: S5 sucht, findet oder bricht mit Anleitung ab. Am Dateibestand ändert sich nichts.

---

**Zweiter, kleinerer Punkt derselben Meldung.** Das Abnahmekriterium von Schritt 5 verlangt, dass `codesign -dv` die Identität nennt. Das tut es nicht: `-dv` gibt die Zeile `Authority=` gar nicht aus, sie erscheint erst bei `-dvv`. `-dv` zeigt `flags=0x0(none)`, was eine Ad-hoc-Signatur ausschließt, aber die Identität nicht benennt. Nachgeprüft am 260802-1935:

```
$ codesign -dvv target/KRK.app
CodeDirectory v=20400 size=3433 flags=0x0(none) hashes=101+3 location=embedded
Authority=KRK Entwicklung
```

Der `planner` zieht das Kriterium auf `-dvv`.

**Aufgefallen bei:** der Umsetzung von Schritt 5, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1927-buendelbau-versionsersetzung-und-lokale-signierung.md`. Der `coder` hat beides gemeldet statt eigenmächtig aufzulösen.
