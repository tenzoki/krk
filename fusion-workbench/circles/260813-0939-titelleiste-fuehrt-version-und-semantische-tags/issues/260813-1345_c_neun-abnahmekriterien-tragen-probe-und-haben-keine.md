Neun Abnahmekriterien tragen **(Probe)** und haben keine

---

Der Spec kennzeichnet jedes Kriterium damit, wie es nachzuweisen ist: **(Probe)** heisst „eine
Prüfung im Baum weist es nach, ein Agent kann es abnehmen". Neun der 59 tragen die Kennzeichnung
und haben keine benannte Probe. Sie sind am Baum lesbar und von Hand nachgelesen; maschinell
abgenommen ist keines.

---

**Schwere:** niedrig. Alle neun halten in der Sache, beim Abgleich einzeln gegen den Baum
gelesen. Was fehlt, ist die Zusage, dass sie beim nächsten Umbau rot werden.

| Kriterium | Was es verlangt | Wie es heute steht |
|---|---|---|
| C2.8 | Steht ein Blatt, bleibt der Titel stehen | Am Baum lesbar: `fokusanzeige_nachziehen` steigt bei `blatt_steht()` früh aus (`crates/krk-ui/src/appkit/anwendung.rs:3754-3756`). Keine Probe, weil der Ausstieg in AppKit-Code steht |
| C2.10 | Der Titel zieht bei vier Anlässen nach, eine Auswahlbewegung ändert ihn nicht | Am Baum lesbar: `titel_nachziehen` hat vier Aufrufanlässe, und der Doc-Kommentar (`anwendung.rs:3775-3780`) begründet, warum die Auswahlbewegung nicht dazugehört (L1 aus C8 der Runde 1). Keine Probe |
| C4.1 bis C4.7 | Der Abschnitt `### Versionsstufen` in `README.md` und seine sieben Aussagen | Alle sieben stehen im Text (`README.md:317-369`), einzeln nachgelesen. Der Plan sieht in D4 keine Probe vor, und der Ausführer hat das vermerkt |

**Die Durchsicht hat den C4-Teil schon eingeordnet** und richtig: „Wer sie maschinell will,
braucht einen Schritt, der eine Probe an der Datei vorsieht — das ist eine Planlücke und kein
Befund an diesem Bau." Dieser Datensatz hält die Lücke fest, damit sie nicht mit der Runde
verschwindet, und nimmt C2.8 und C2.10 dazu, die die Durchsicht nicht genannt hat.

**Derselbe Befund ist in der Runde 7 schon einmal abgelegt worden**
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0647_o_neun-abnahmekriterien-versprechen-eine-probe-und-haben-keine.md`,
offen). Dass die Zahl beide Male neun ist, ist Zufall; dass der Fall wiederkehrt, ist keiner.

**Was zu tun ist**

Zwei Wege, und sie schliessen einander nicht aus:

1. **Die sieben C4-Kriterien maschinell abnehmen.** Eine Probe über `README.md` nach dem Muster
   der Zählproben dieses Baums: die Überschrift `### Versionsstufen` steht unter
   `## Versionspflege`, und die sieben Aussagen tragen je eine Nadel. Der Preis ist eine Probe,
   die an einer Umformulierung rot wird, ohne dass etwas kaputt wäre — deshalb gehören die
   Nadeln an die Sache (`v<version>`, `~/Library/Application Support/KRK/`) und nicht an ganze
   Sätze.
2. **Die Kennzeichnung berichtigen.** Wo keine Probe möglich oder gewollt ist, sagt der Spec
   das, statt eine zuzusagen. C2.8 und C2.10 sind Kandidaten dafür: beide hängen an AppKit-Code
   ohne Bibliotheksziel.

Der zweite Weg ist der billigere und der ehrlichere; der erste ist der, der die Zusage einlöst.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Von den 59 Kriterien tragen 48 allein **(Probe)**, sieben **(Probe)** und **(Bündel)**, drei
  allein **(Bündel)** und eines **(Nutzerarbeit)**. Die neun hier stehen unter den 48.

---
Resolved: Die sieben C4-Kriterien haben jetzt eine Probe, und fuer C2.8 und C2.10 steht die Berichtigung der Kennzeichnung im Nachsatz des Spec (`planning/260813-1037_c_spec-…`, Punkt 3). Gebaut ist `die_readme_traegt_den_abschnitt_ueber_die_versionsstufen` in `xtask/src/release.rs`, wo schon zwei andere Proben die `README.md` lesen. Sie geht den vom Datensatz empfohlenen Weg 1: Nadeln an der Sache (die drei Stufennamen, `~/Library/Application Support/KRK/`, `v<version>`, `Mindest-Zielsystem`, `Unbeachtete Dateien`, `cargo xtask bundle`) und nicht an ganzen Saetzen, dazu die Ordnung der zwei Abschnitte und C4.7 als Gegenprobe — die Herkunft der Zahl steht in der Versionspflege und wird im neuen Abschnitt nicht wiederholt. **Sie haelt den heutigen Stand und nicht den Wortlaut von 260813**: C4.3 ist vom Nutzerentscheid 260813-1534 ueberholt (den Tag setzt das Werkzeug), und `v0.1.0` aus C4.4 steht in der `README.md` nicht mehr (`grep -n "v0\.1\.0" README.md` → keine Fundstelle); beides schreibt der Nachsatz aus. C2.8 und C2.10 haengen an AppKit-Code in einer Kiste ohne Bibliotheksziel und sind der Sache nach (Buendel). `make check` exit 0.
