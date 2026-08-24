# Wie wird die Arbeit dieser Runde jemals gegen L7 gemessen, wenn die Messstrecke sie nicht sieht?

---
**Domain:** code
**Filed by:** reconciler
**Cross-references:** `planning/260824-0613_*_spec-…` (Abschnitt „Verhältnis zur Zeitzusage L7"); `planning/260824-0640_*_plan-…` (`## Risks & Mitigations`, Zeilen zum Messmodus und zum zusätzlichen Leselauf); `shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`; `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`; `crates/krk-bench/src/messen.rs:1110-1114`; `crates/krk-ui/src/messmodus.rs:832`; `crates/krk-ui/src/appkit/anwendung.rs:1446-1493`

---

## Question

Der Spec dieser Runde sagt, die Zusammenfassung falle in die Endbedingung von L7 und die Runde
schulde damit denselben späteren Messlauf wie die Runde 14. **Beim Abgleich am 260824-1900 hat
sich gezeigt, dass ein solcher Lauf die Arbeit dieser Runde nicht messen könnte**, und zwar aus
zwei voneinander unabhängigen Gründen.

**Erstens wählt L7 eine Datei und keinen Ordner.** Die Sitzungsstrecke springt für L7 auf
`a/datei-2` und wartet, bis deren Vorschau steht (`crates/krk-ui/src/messmodus.rs:832`, Probe
`die_l7_messung_wartet_auf_die_vorschau_des_neuen_eintrags`). Die Zusammenfassung entsteht nur
für einen Eintrag, der **keine** gewöhnliche Datei ist (`krk-ui/src/vorschaumodell.rs:701-709`).
Der gemessene Weg berührt sie also nicht.

**Zweitens lädt der Messmodus die Ablage nicht.** Alle vier Messaufgaben kehren aus
`Anwendungsdelegierter::sitzung_laden` zurück, bevor der eine Durchgang läuft, der seit dieser
Runde auch `leseprofile::laden` ruft (`anwendung.rs:1446-1560`). Im Messmodus bleibt
`AnwendungsIvars::profile` leer, `profile_setzen` überträgt einen leeren Profilsatz, und ein
Ordner zeigt seine Metadaten wie bis zur Runde 15. Selbst wenn die Strecke einen Ordner auswählte,
mäße sie den Stand vor dieser Runde.

**Damit unterscheidet sich die Lage von der der Runde 14**, und der Unterschied ist der Grund,
aus dem diese Frage überhaupt gestellt wird. Die Runde 14 hat ihre Arbeit in den Renderweg einer
Textdatei gelegt, also genau in den Weg, den L7 misst; ihr Rückstand wäre mit dem nächsten Lauf
abgetragen. Der Rückstand dieser Runde wäre es nicht. Wer beide unter dieselbe Formel „schuldet
denselben späteren Lauf" stellt, hält eine Schuld für beglichen, die kein Lauf begleicht.

**Was ungemessen bleibt, ist benannt und nicht klein.** Der Plan führt es in seiner Risikotabelle:
jeder ausgewählte Ordner, für den kein Pfadmuster trifft, kostet seit dieser Runde einen
Verzeichnisleselauf, den es vorher nicht gab, gedeckelt auf 2.000 Einträge. Das trifft **jeden**
Ordner in jedem Projekt und nicht nur die Orte, für die ein Profil greift. Für einen erkannten
Ordner kommen bis zu zwölf Leseläufe und vierundzwanzig Dateiöffnungen dazu; das größte
mitgelieferte Profil kostet gemessene fünf und elf, aber gemessen ist die **Zahl der Aufrufe**
und nicht die Zeit.

## Options

1. **Stehen lassen und richtig aufschreiben.** Die Runde bleibt ungemessen, und der Spec sagt
   nicht mehr „schuldet denselben späteren Lauf wie die Runde 14", sondern „liegt außerhalb dessen,
   was die Messstrecke sieht".
   - Pro: kostet nichts, und die Aussage stimmt danach.
   - Contra: eine Zusage, deren Endbedingung Arbeit enthält, die nie gemessen wird, ist an dieser
     Stelle keine Zusage mehr. Die Zahl der Runden, die L7 berühren und nicht messen, wächst
     weiter, und keine spätere Messrunde trägt sie ab.

2. **Die Sitzungsstrecke bekommt einen Ordnersprung.** Eine weitere gemessene Größe innerhalb von
   L7: Auswahl eines Ordners, für den kein Profil greift, bis seine Anzeige steht. Der Prüfordner
   der Strecke trägt Unterordner schon (`a-l6` mit 1.000 Einträgen).
   - Pro: misst genau den Preis, den jeder Ordner in jedem Projekt zahlt, also die Hälfte, die
     alle Nutzer trifft. Ohne eine einzige `readers.toml`.
   - Contra: eine neue gemessene Größe in der Strecke, und der Nutzer muss den Lauf fahren.
     Ob sie unter L7 fällt oder eine elfte Zusage wäre, ist selbst zu entscheiden — und dieses
     Projekt hat in fünfzehn Runden keine elfte gesetzt.

3. **Der Messmodus lädt die Leseprofile mit.** Dazu käme Möglichkeit 2, denn ohne einen
   Ordnersprung nützt der geladene Profilsatz nichts.
   - Pro: misst die vollständige Arbeit dieser Runde, Zusammenfassung eingeschlossen.
   - Contra: der Messmodus lädt heute mit Absicht nichts aus der Ablage, damit die Zahlen nicht
     vom Bestand des Geräts abhängen. Ein mitgelieferter Profilsatz im Prüfordner wäre eine neue
     Abhängigkeit der Messung von einer Datei unter `resources/`.

## Constraints

- Eine Zusage ohne Messstrecke wäre ein Wunsch; dieses Projekt hat in fünfzehn Runden keine elfte
  gesetzt und keine der zehn angefasst.
- Kein Agent kann den Lauf fahren: er verlangt KRK im Vordergrund
  (`circles/260802-0842-…/decisions/260806-1303_*_…`). Jede Möglichkeit außer 1 ist Nutzerarbeit.
- Die abzählbaren Grenzen aus C6 bleiben, wie sie sind. Sie zählen Aufrufe und keine
  Millisekunden, und sie ersetzen keine Zeitmessung, sondern stehen an ihrer Stelle.

## Recommendation

**Möglichkeit 2**, und getrennt davon die Berichtigung des Satzes aus Möglichkeit 1. Der
Ordnersprung misst die Hälfte des Preises, die jeden Nutzer und jedes Projekt trifft, und er
braucht weder eine `readers.toml` noch eine Änderung am Messmodus. Ob die Zahl unter L7 fällt
oder daneben steht, ist die eigentliche Frage an den Nutzer; der Abgleich hält sie nicht für
entschieden.

**Nichts davon hält den Abschluss dieser Runde auf.** Die Runde ist gebaut, ihre abzählbaren
Grenzen sind belegt, und die Frage betrifft die Messbarkeit einer Zusage aus der Runde 1 und
nicht die Einlösung eines Kriteriums dieser Runde.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:
