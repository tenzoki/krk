C1.7 verlangt eine Probe für die Reihenfolge von `Esc`, und B2 hat keinen Ort dafür

---

Der Spec kennzeichnet C1.7 mit **(Probe** für die Reihenfolge, **Bündel** für den
Tastendruck**)**. Die Bündelhälfte ist in G2 vergeben. Die Probenhälfte ist nirgends
vergeben: B2 nennt in seinen `Changes` nur, dass `Esc` seine Stelle in
`Anwendungsdelegierter::abbrechen` bekommt, und die `## Testing Strategy` des Plans zählt
fünf reine Funktionen auf, unter denen die Rangfolge des Abbruchbefehls nicht vorkommt.

Umgesetzt ist die Rangfolge deshalb als gerade Folge von drei Rängen mit frühem Rücksprung
in `crates/krk-ui/src/appkit/anwendung.rs`, `Anwendungsdelegierter::abbrechen`. Die
Reihenfolge steht dort an genau einer Stelle und ist im Modulkommentar als Skizze
ausgeschrieben, aber sie hängt an drei Ivars des Anwendungsdelegierten (`offenes_blatt`,
`vorgang`, das Tabmodell des aktiven Dateifensters) und ist damit ohne laufendes AppKit
nicht zu stellen. `krk-ui` hat kein Bibliotheksziel; eine Probe daneben müsste einen
Anwendungsdelegierten bauen.

---

**Was der Zustand ist.** Von den fünf Kriterien, die B2 erfüllt, sind C1.8, C1.9 und C1.10
in `crates/krk-ui/src/tabs.rs` als Proben abgenommen. C3.5 ist eine Aussage über eine
abwesende Sache — es entsteht kein eigener Rang für das Anhalten des Durchlaufs — und am
Code abzulesen. Für C1.7 ist die Probenhälfte offen.

**Der naheliegende Weg, und warum ihn dieser Schritt nicht gegangen ist.** Die Rangfolge
ließe sich als reine Funktion über drei Wahrheitswerte stellen, in der Bauart von
`kommandos::rueckschritt` aus C1 dieser Runde und mit einer ausgeschriebenen Tafel über
acht Fälle. Ihr Ort wäre `crates/krk-ui/src/kommandos/operationen.rs`, neben
`waehrend_blatt_erlaubt` und `abbruchzeile`, die schon heute die beiden anderen reinen
Stücke des Abbruchbefehls tragen; eine sechste Datei unter `kommandos/` entstünde nicht.

Zwei Gründe sprechen dagegen, das ungefragt in B2 zu tun. Erstens zählt die
`## Data Structures` des Plans **sechs** neue Typen der Runde einzeln auf; ein siebter
(`Abbruchziel`) widerspräche der Aufstellung. Zweitens müsste jeder der drei Zweige den
Wert danach ein zweites Mal holen, den die Regel gerade gesehen hat — beim Blatt ein
`take()` mit `if let Some`, das die Regel nicht mehr ausdrücken kann. Ob dieser Preis die
Probe wert ist, ist eine Entwurfsfrage und gehört dem Planner.

**Wer das aufnimmt, entscheidet zwischen zwei Auswegen:** die reine Funktion samt Tafel
bauen und die Aufstellung der Typen um einen Eintrag erweitern, oder C1.7 ausdrücklich
ganz auf die Bündelhälfte in G2 stellen und die Kennzeichnung im Spec nachziehen. Der
zweite Weg ist billiger und lässt die Reihenfolge ungeprüft; der erste kostet die beiden
oben genannten Punkte.

**Verwandt:** `issues/260814-2357_o_c2-nennt-zwei-dateien-der-weg-an-den-filtertext-des-tabs-fuehrt-durch-eine-dritte.md`
sammelt die zu kurzen `Files:`-Zeilen. Dieser Datensatz gehört nicht dorthin: B2 hat genau
die drei genannten Dateien angefasst, und was fehlt, ist keine vierte Datei, sondern eine
Zusage ohne Träger.
