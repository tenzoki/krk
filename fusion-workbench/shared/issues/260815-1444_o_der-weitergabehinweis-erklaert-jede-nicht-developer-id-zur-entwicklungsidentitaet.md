Der Weitergabehinweis erklärt jede Nicht-Developer-ID zur Entwicklungsidentität

---

`sign::weitergabehinweis` (`xtask/src/sign.rs:171-192`) unterscheidet in einer einzigen
Verzweigung:

```rust
let lage = if identitaet.starts_with(DEVELOPER_ID_PRAEFIX) { … } else {
    format!(
        "dieses Buendel bleibt auf dieser Maschine. Signiert ist es mit {identitaet:?}, \
         einer Entwicklungsidentitaet, und Gatekeeper weist ein so signiertes Buendel auf \
         jedem anderen Mac als moegliche Schadsoftware ab"
    )
};
```

Der `else`-Zweig ist ein Auffangzweig, behauptet aber eine **positive** Einordnung: „einer
Entwicklungsidentitaet". Die trägt er für jeden Namen, der nicht mit
`Developer ID Application` beginnt — und das sind mehr Fälle als
Entwicklungsidentitäten.

---

**Schwere:** mittel. Kein Verhalten, kein Bau. Die praktische Folge des Satzes stimmt in
allen Fällen; falsch ist die Begründung, die er dafür angibt, und der erste Halbsatz
„bleibt auf dieser Maschine".
**Gefunden von:** coderev, Durchsicht des Bereichs `cd0b5b7..093a6f4`
**Betroffen:** `xtask/src/sign.rs:178-183`
**Domain:** code

## Die Fälle, die in den falschen Zweig fallen

Der Name kommt aus `Identitaet::name`, und `sign::bestimmen` (`sign.rs:47-69`) füllt ihn aus
drei Stufen. Nur die Stufen 2 und 3 liefern garantiert einen vollständigen Common Name aus
dem Schlüsselbund; für die ist die Prüfung am Namensanfang tragfähig. Die erste Stufe,
`aus_umgebung` (`sign.rs:237-244`), reicht durch, was in `KRK_SIGN_IDENTITY` steht, und
prüft daran nichts als die Nichtleere.

`codesign --sign` nimmt dort dreierlei an, und zwei davon entgehen der Prüfung:

| Wert in `KRK_SIGN_IDENTITY` | `starts_with(DEVELOPER_ID_PRAEFIX)` | Was der Hinweis sagt |
|---|---|---|
| `"Developer ID Application: … (TEAM)"` | ja | richtig |
| `"QYMPYB7MWM"` — Teilzeichenfolge des Common Name, von `codesign` als Auswahl angenommen | **nein** | „einer Entwicklungsidentitaet" |
| SHA-1-Abdruck des Zertifikats, ebenfalls eine gültige Auswahl | **nein** | „einer Entwicklungsidentitaet" |

Dazu kommt die dritte Stufe von `bestimmen`: sie nimmt die einzige gültige Identität des
Schlüsselbunds, **gleich welcher Art**. Ist das eine `Apple Distribution: …` oder eine
`3rd Party Mac Developer Application: …`, so ist der Name vollständig und trotzdem keine
Entwicklungsidentität. Der Hinweis nennt sie eine.

## Warum das genau die Falle des Quelldatensatzes ist

`shared/issues/260812-1628_c_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`
schreibt vor: „Die Meldung muss die Art der Identität lesen, nicht den Unterbefehl", und
begründet es mit dem Fall einer über `KRK_SIGN_IDENTITY` gesetzten Developer-ID. Genau
dieser Fall ist es, der durch die Hintertür der Auswahlform — Teilzeichenfolge oder
Abdruck statt Name — wieder eintritt. Die Runde hat die Tür geschlossen, die der Datensatz
benennt, und die daneben stehen lassen.

**Abschwächend, und darum mittel und nicht hoch:** die Sachaussage über Gatekeeper trifft in
allen genannten Fällen zu, denn `bundle` beglaubigt in keinem davon. Falsch sind die
Einordnung und der Satz „bleibt auf dieser Maschine"; ein Bündel mit Developer-ID-Signatur
bleibt nicht auf dieser Maschine, es ist nur nicht beglaubigt.

## Was zu tun wäre

`Identitaet` trägt das Nötige schon mit: `herkunft` (`sign.rs:38`) sagt, ob der Name aus
`UMGEBUNGSVARIABLE` stammt. Zwei Wege stehen offen.

1. **Den Auffangzweig ehrlich machen.** Er behauptet keine Art mehr, sondern sagt, was
   feststeht: dass der Name nicht die einer Developer-ID ist und das Bündel nicht
   beglaubigt. Kostet einen Satz, deckt alle vier Fälle, und die drei Proben aus
   `sign.rs:573-620` ziehen mit.
2. **Die Art am Schlüsselbund auflösen**, wenn die Herkunft die Umgebungsvariable ist: den
   gesetzten Wert gegen die Ausgabe von `security find-identity` halten und den vollen
   Namen nehmen. Trägt mehr, kostet einen Aufruf und eine Fehlerlage im Hinweispfad.

Der erste Weg ist der wahrscheinlich richtige: der Hinweis soll eine Folge nennen, nicht
ein Zertifikat einordnen.

## Herkunft

Gemeinsamer Speicher. Betrifft den Bauweg des ganzen Projekts und nicht die Directive einer
Runde.
