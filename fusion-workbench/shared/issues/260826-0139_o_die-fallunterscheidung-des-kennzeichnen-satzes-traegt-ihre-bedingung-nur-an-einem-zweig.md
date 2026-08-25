Die Fallunterscheidung des `kennzeichnen`-Satzes trägt ihre Bedingung nur an einem Zweig

---

`resources/default-readers.toml:63-65` sagt seit `255ad7a`: „`kennzeichnen` statt `kennzeichen`
nimmt dem Profil sein Erkennungsmuster, und es fällt in die zweite Reichweite oder greift, steht
ein `pfad` daneben, still über diesen allein." Beide Zweige stimmen, einzeln nachgemessen. Die
Bedingung, die zwischen ihnen entscheidet, steht aber allein **im zweiten**: der erste liest sich
unbedingt. Wer den Satz einmal liest, kann ihn als „das Profil fällt weg, und obendrein kann es
über einen `pfad` weitergreifen" verstehen — zwei Ausgänge, die einander ausschließen, gelesen
als einer mit Zusatz.

---

**Filed by:** ontorev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `resources/default-readers.toml:61-67`;
`shared/issues/260825-2233_c_der-satz-ueber-kennzeichnen-gilt-nur-fuer-ein-profil-ohne-pfad-daneben.md`
(der Befund, dessen Behebung diesen Satz erzeugt hat);
`crates/krk-core/src/leseprofil/datei.rs`, `pruefen` (der eine Meldezweig
`pfad.is_none() && kennzeichen.is_none()`);
`$FUSION_PLUGIN_ROOT/rules/critical-stance.md` `## 4` (eine Fallunterscheidung ist disjunkt und
vollständig — sie ist es hier, nur sagt der Satz es nicht)

## Was gemessen ist

Gemessen am 260826-0139, Baum `96e32cb`, über `leseprofil::datei::pruefen` und
`leseprofil::bausteine::zusammenfassen_gezaehlt` an abgewandelten Fassungen der Datei. Die
Sachaussage des Satzes hält in beiden Lagen:

| Verschreibung | Profile | Meldungen | Ort | Ergebnis |
|---|---|---|---|---|
| `kennzeichnen = 'x'` **neben** dem `pfad` der zwei Speicherprofile (`:352`, `:375`) | 12 | keine | `fusion-workbench/shared/history` | Profil greift, 1 Leselauf, 10 Öffnungen |
| `kennzeichnen` **statt** `kennzeichen` am Wurzelprofil (`:305`), das keinen `pfad` trägt | 11 | „es nennt weder ein Pfadmuster noch eine Kennzeichendatei" | `fusion-workbench` | kein Profil |

Die zwei Zweige sind damit disjunkt und vollständig: steht ein `pfad` daneben, greift das Profil
still; steht keiner, fällt es mit Meldung weg. Ein dritter Ausgang existiert nicht.

Gemessen ist außerdem, dass die neue Zeichensetzung eine zweite Ungenauigkeit **behoben** hat, die
in der ersten Durchsicht nicht aufgefallen ist: der Schlusssatz „und das ohne jede Meldung" hing
vor `255ad7a` an einer Aufzählung, deren erstes Glied der `kennzeichnen`-Fall war — und der
erzeugt eine Meldung, siehe zweite Zeile der Tabelle. Das Semikolon trennt ihn jetzt ab. Der Satz
ist in der Sache **genauer** geworden, nicht nur länger.

## Warum das zählt

Nicht wegen einer falschen Aussage — es steht keine da. Sondern wegen der Stelle, an der der Satz
steht: der Abschnitt „Was ein Schreibfehler kostet" ist der Kopf, den ein Nutzer liest, **bevor**
er die Datei ändert, und sein Zweck ist, ihm die drei Reichweiten so beizubringen, dass er sie
danach selbst anwenden kann. Ein Satz, dessen Fallunterscheidung erst beim zweiten Lesen
auseinanderfällt, arbeitet gegen diesen Zweck.

Dazu kommt die Einwegigkeit dieser Datei: sie wird beim ersten Start wörtlich nach
`~/Library/Application Support/KRK/readers.toml` kopiert und danach nie wieder angefasst
(`ablage::leseprofile::anlegen_falls_fehlt`, Zusage C1.2 der Runde 16). Eine spätere Runde
erreicht die Kopie eines bestehenden Nutzers nur über den Handgriff aus `README.md:62-63`
(`shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`,
Möglichkeit 1 gewählt). „Später aufräumen" heißt für diese Datei also: bei allen, die vorher
installieren, gar nicht.

Schwere **niedrig**: die Aussage stimmt, beide Zweige stehen da, und der Leser, der stolpert,
verliert einen zweiten Lesedurchgang und keine Auskunft. Kein Auslieferungshindernis.

## Möglichkeiten

1. Die Bedingung vor den Zweig ziehen, den sie regiert, und das Verb nicht mehr von seinem
   Nachsatz trennen — dieselbe Zeilenzahl:

   ```
   # was der Schreibfehler weggenommen hat: `kennzeichnen` statt `kennzeichen`
   # nimmt dem Profil sein Erkennungsmuster; es fällt dann in die zweite
   # Reichweite, oder es greift, steht ein `pfad` daneben, still über diesen
   # allein. `zeilen` statt `zeile` lässt ein Profil ohne eine einzige Zeile
   # stehen, und das ohne jede Meldung.
   ```

   Der Gewinn steckt im `dann`: es macht den ersten Zweig hörbar bedingt.

2. Beide Zweige als eigene Sätze, mit der Bedingung voran:

   ```
   # … nimmt dem Profil sein Erkennungsmuster. Steht kein `pfad` daneben, fällt
   # es in die zweite Reichweite; steht einer, greift es still über diesen
   # allein.
   ```

   Das ist die deutlichste Form und kostet eine Zeile, die N2 (`260825-2233_c_ein-leeres-
   fusion-workbench-…`) im selben Lauf gerade freigemacht hat.

3. Stehen lassen. Der Satz ist wahr, und jede weitere Änderung an dieser Stelle ist die vierte
   in zwei Tagen.

Möglichkeit 1 ist die kleinste, die den Befund wirklich behebt. Wer den Kopf als Handbuch liest
und nicht als Vertrag, nimmt Möglichkeit 2.

**Ausdrücklich nicht Gegenstand dieses Datensatzes:** der Satz `:61-63` „Was er kostet, ist das,
was der Schreibfehler weggenommen hat". Er ist die einzige Stelle der Datei, die den
**spiegelbildlichen** Fall abdeckt — ein verschriebenes `pfad` neben einem richtigen
`kennzeichen`. Gemessen: `pfd` statt `pfad` am Defektspeicherprofil mit einem `kennzeichen`
daneben → 12 Profile, keine Meldung, das Profil greift über das `kennzeichen` allein; ohne
`kennzeichen` daneben → 11 Profile und dieselbe Meldung wie oben. Der Satz trägt diese Symmetrie
als Regel, während die zwei Beispiele dahinter nur eine Richtung ausschreiben. Er ist zu behalten.
