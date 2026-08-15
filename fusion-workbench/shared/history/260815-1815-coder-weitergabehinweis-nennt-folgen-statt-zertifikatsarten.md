# Der Weitergabehinweis nennt Folgen und ordnet kein Zertifikat mehr ein

**Status:** Complete
**Agent:** coder
**Anlass:** Nutzerlinie vom 260815-1800 zu zwei Durchsichtsbefunden mit einer
gemeinsamen Wurzel:
`shared/issues/260815-1444_p_der-weitergabehinweis-erklaert-jede-nicht-developer-id-zur-entwicklungsidentitaet.md`
und
`shared/issues/260815-1445_p_der-developer-id-zweig-nennt-die-fehlende-beglaubigung-und-nicht-die-fehlende-gehaertete-laufzeitumgebung.md`.
Möglichkeit 1 aus 260815-1444 vom Nutzer gewählt: keine Auflösung der
Identitätsart am Schlüsselbund, kein `security`-Aufruf im Hinweispfad. Nur
`xtask/`, kein Verhalten am Signieren, kein Commit.

---

## Die gemeinsame Wurzel und der Schnitt

`sign::weitergabehinweis` ordnete ein Zertifikat ein, wo es eine Folge nennen
sollte. Daraus flossen beide Befunde, und daraus folgt der Schnitt: **alles,
was ohne Rücksicht auf die Identität feststeht, wandert in den gemeinsamen
Teil, und die Verzweigung behält allein die Aussage über den Namen.**

Vorher trug jeder Zweig seine eigene Folgenkette, und beide waren unvollständig
— der eine falsch begründet, der andere um die gehärtete Laufzeitumgebung
verkürzt. Nachher steht die Folgenkette einmal da:

```
                     ┌─ Name trägt "Developer ID Application" → "mit der Developer-ID …"
signiert ist es ─────┤
                     └─ sonst → "mit …, und dieser Name ist nicht der einer Developer-ID"
                                │
   nicht beglaubigt · ohne gehärtete Laufzeitumgebung · Gatekeeper weist ab ·
   nicht universell · Weg zur Weitergabe: cargo xtask release
```

Der Developer-ID-Zweig bleibt, weil ein erkennbarer Developer-ID-Name eine
echte Auskunft ist. Der Auffangzweig sagt jetzt, was er weiß — dass der Name
nicht der einer Developer-ID ist — statt einzuordnen, was er nicht geprüft hat.

## Was gestrichen ist und warum

- **„einer Entwicklungsidentitaet"**. Eine positive Einordnung in einem
  Auffangzweig. `codesign --sign` nimmt über `KRK_SIGN_IDENTITY` auch eine
  Teilzeichenfolge des Common Name und den SHA-1-Abdruck an, und beide können
  eine Developer-ID wählen, ohne das Präfix zu tragen; `aus_umgebung` prüft am
  Wert nur die Nichtleere. Die dritte Stufe von `bestimmen` nimmt daneben die
  einzige gültige Identität des Schlüsselbunds gleich welcher Art.
- **„bleibt auf dieser Maschine"**. Bei einer über den Abdruck gewählten
  Developer-ID stimmt es nicht.
- **„als moegliche Schadsoftware"** im Auffangzweig. Der Wortlaut stammt aus
  dem Vorfall vom 260812, und der lief über eine Entwicklungsidentität; ihn
  stehenzulassen hieße, die Art durch die Hintertür wieder zu behaupten. Im
  Modulkopf bleibt er stehen, denn dort beschreibt er den Vorfall selbst.
- **„und damit richtig"**. Traf die Identität und gab die Signaturform als
  erledigt aus. `bundle` signiert über `signieren` ohne `--options runtime`;
  `notarytool` weist ein so signiertes Bündel ab.

Neu im gemeinsamen Teil: die fehlende gehärtete Laufzeitumgebung samt ihrer
Folge, und im Schlusssatz die Wendung des Hilfetexts aus `main.rs`
(„signiert mit einer Developer-ID-Identitaet und gehaerteter
Laufzeitumgebung"), damit nicht zwei Beschreibungen desselben Befehls
nebeneinander stehen.

## Was ausdrücklich nicht geschehen ist

`bundle` bekommt **kein** `--options runtime`. Ob es gehärtet signieren soll,
ist eine eigene Frage, die niemand gestellt hat; dieser Auftrag beschreibt nur
richtig, was ist. Die Art der Identität wird nicht am Schlüsselbund aufgelöst:
das kostete einen `security`-Aufruf samt eigener Fehlerlage im Hinweispfad, und
für die Folge, die der Hinweis nennt, ändert die Art nichts.

## Proben

Angepasst:

- `eine_apple_development_identitaet_bekommt_die_maschinengrenze_genannt` →
  `ein_name_ohne_developer_id_praefix_bekommt_keine_art_zugeschrieben`. Prüft
  jetzt die Abwesenheit von „Entwicklungsidentitaet" und „bleibt auf dieser
  Maschine" und die Anwesenheit der Namensaussage.
- `eine_developer_id_wird_nicht_fuer_falsch_signiert_erklaert`. Name bleibt,
  Zusicherungen gedreht: „damit richtig" darf nicht mehr vorkommen, die
  Namensaussage des Auffangzweigs ebenso wenig.

Dazugekommen:

- `ein_sha1_abdruck_als_identitaet_bekommt_keine_art_zugeschrieben` — der Weg,
  auf dem der Befund hereinkam.
- `beide_faelle_nennen_die_fehlende_gehaertete_laufzeitumgebung` — die zweite
  Lücke, für beide Zweige, und eine Zusicherung gegen `include_str!("main.rs")`,
  die den Schlusssatz an den Hilfetext bindet.

`beide_faelle_nennen_die_architektur_und_den_weg_zur_weitergabe` zieht
unverändert mit.

## Prüfung

```
cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo test --workspace
```

Exit 0. 1195 Proben, keine gescheitert.

## Was aufgefallen und nicht Auftrag war

Der Satz über Gatekeeper stand vorher in zwei Stärken nebeneinander — „auf
einem anderen Mac" im Developer-ID-Zweig, „auf jedem anderen Mac" im
Auffangzweig. Genommen ist die schwächere, denn ein Bündel, das ohne
Quarantänemarke auf einen zweiten Mac kommt, wird von Gatekeeper nicht
angehalten. Der Hinweis behauptet damit etwas weniger, als er vorher behauptete,
und das ist die Richtung dieses Umbaus.

Die zwei Datensätze nennen es nicht: der Modulkopf von `sign.rs` trug die
Begründung „Unterschieden wird nach der Art der Identitaet und nicht nach dem
Unterbefehl" als Leitsatz. Der Satz stammt aus
`shared/issues/260812-1628_c_…` und ist unverändert richtig, was den
Unterbefehl angeht — nur ist „die Art" nicht das, was die Verzweigung liest.
Sie liest den Namen, und das ist weniger. Der Modulkopf sagt das jetzt.
