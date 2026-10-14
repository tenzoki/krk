Der Leerbefund-Zweig verschweigt eine dastehende Sicherung, die den Bestand trägt

---

Seit `d771ec6` gibt der Leerbefund-Zweig in `Zugang::laden` unbedingt `Beiseite::Nicht` zurück
und ruft `beiseite_legen` nicht mehr. Damit entfällt neben der wertlosen Sicherung auch die
Auskunft `Beiseite::SchonVorhanden`: liegt unter `bookmarks.toml.beschaedigt` bereits eine
Sicherung, die den **echten** Bestand des Nutzers trägt, nennt die Meldung sie nicht mehr.
Der Bestand ist unversehrt, der Weg zu ihm wird verschwiegen — und zwar genau in dem Moment,
in dem der Nutzer ihn sucht.

---

**Gemessen am Baumstand `d771ec6`**, über `Ablage::oeffnen`, `durchgang` und `laden`, also über
denselben Weg wie das laufende Programm. Ausgabe wörtlich:

```
SCHRITT1 beiseite=Gesichert(".../bookmarks.toml.beschaedigt")
SCHRITT2 beiseite=Nicht
SCHRITT2 meldung=.../bookmarks.toml ist beschaedigt und wird durch den Auslieferungszustand
        ersetzt: die Datei traegt keinen einzigen obersten Schluessel, und KRK schreibt sie
        nie so
SCHRITT2 sicherung_da=Ok(true)
        inhalt=Some("[[lesezeichen]]\nname = \"P\"\nordner = \"/\"\n")
```

Der Verlauf im Klartext:

1. `bookmarks.toml` trägt den echten Bestand in einer Gestalt, die dieser Bau nicht liest
   (fremder oberster Schlüssel). `Zugang::laden` legt sie über `beiseite_legen` beiseite;
   `Beiseite::Gesichert`, und unter `bookmarks.toml.beschaedigt` steht der Bestand wörtlich.
2. Der Nutzer liest die Meldung, öffnet `bookmarks.toml`, findet `eintraege = []` oder leert
   sie selbst, und startet KRK erneut.
3. Der Leerbefund-Zweig greift (`crates/krk-core/src/ablage/mod.rs:609-624`) und gibt
   `Beiseite::Nicht` zurück, **ohne `beiseite_legen` zu fragen**. Die Meldung nennt den
   Beiseitepfad deshalb nicht.
4. Die Sicherung aus Schritt 1 liegt unangetastet da und trägt den Bestand. Der Nutzer erfährt
   davon nichts mehr.

**Vor `d771ec6` war der Satz da.** Der Zweig rief `beiseite_legen`, dieses fragte
`pfad.try_exists()` (`mod.rs:822-826`), fand die Sicherung und gab `Beiseite::SchonVorhanden`
zurück; `Display` erzeugte daraus „Die bisherige Fassung liegt seit einem früheren Start unter
… und bleibt dort" (`mod.rs:403-408`).

**Das ist kein Argument gegen `d771ec6`.** Der Zweig durfte den einen Sicherungsplatz nicht
belegen, und `Beiseite::Nicht` ist dafür die richtige Antwort; der Datensatz
`260821-1023_c_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`
trägt die Begründung. Es geht allein um die zweite Auskunft, die mit derselben Zeile
weggefallen ist. Sie hängt nicht am Sichern, sondern am **Fragen**, ob schon etwas dasteht,
und diese Frage ist folgenlos.

**Der Preis ist im geschlossenen Datensatz nicht benannt.** Dessen `Resolved:`-Notiz nennt
genau einen Preis — der Wortlaut einer Datei aus lauter Kommentaren bleibt nicht erhalten —
und sagt zur `SchonVorhanden`-Auskunft nichts. Auch die Nachricht von `d771ec6` sagt nur, die
Meldung „behauptet keine Sicherung mehr, die es nicht gibt". Sie verschweigt jetzt zusätzlich
eine, die es gibt.

**Die Zusage, gegen die das läuft.** Der Doc-Kommentar an `Beiseite` (`mod.rs:289-291`) sagt
zu, „dass keine Meldung eine Datei verspricht, die es nicht gibt". Die Umkehrung steht dort
nicht und wäre die nützlichere: die Meldung soll die Datei nennen, die es gibt. Die Ordnung
des Satzes bei `Display` (`mod.rs:366-374`) sagt es allerdings selbst — „Der Satz sagt zuerst,
was der Nutzer tun kann" —, und hier kann der Nutzer etwas tun.

**Wie eng ist die Reihenfolge?** Enger als die des behobenen Defekts, aber erreichbar. Nach
Schritt 1 schreibt der nächste gewöhnliche Lesezeichenbefehl `eintraege = []`, und das ist ein
oberster Schlüssel — der Zweig greift dann nicht. Es braucht also, dass `bookmarks.toml`
zwischen den beiden Starts von außen schlüssellos wird: der Nutzer leert sie von Hand, oder er
löscht ihren Inhalt in der Absicht, den alten Bestand hineinzukopieren. Genau dieser Nutzer
sucht die Sicherung.

## Vorschlag

Zu entscheiden, nicht abzuleiten; die drei Wege unterscheiden sich in dem, was sie an anderer
Stelle kosten.

1. **Fragen ohne zu schreiben.** Der Zweig fragt `atomar::beiseitepfad` und `try_exists`, gibt
   bei `Ok(true)` `Beiseite::SchonVorhanden(pfad)` zurück und sonst `Beiseite::Nicht`. Der
   Platz bleibt frei, die Auskunft kommt zurück. Kostet eine zweite Stelle im Baum, die den
   Beiseitepfad bildet — heute tut das allein `beiseite_legen` —, und `SchonVorhanden` trüge
   dann zwei Bedeutungen: „ich wollte schreiben und ließ es" und „ich wollte gar nicht
   schreiben".
2. **Ein sechster Wert an `Beiseite`.** Etwa `NichtsZuSichern { vorhanden: Option<PathBuf> }`.
   Trennt die beiden Bedeutungen sauber, hält die vollständige Fallunterscheidung an jeder
   Stelle, die sie auseinandernimmt, und kostet einen Zweig in `Display` und in jedem
   Auswerter.
3. **Stehen lassen.** Der Bestand ist nicht verloren, der Ordner ist der Ablageordner, und der
   Nutzer hat die Meldung aus Schritt 1 gesehen. Dann gehört der Preis in die `Resolved:`-Notiz
   des geschlossenen Datensatzes und in den Doc-Kommentar an `Beiseite::Nicht`, damit er nicht
   ein zweites Mal gefunden wird.

**Schwere:** mittel. Kein Datenverlust — die Sicherung bleibt liegen und ist vollständig. Der
Ausgang ist eine fehlende Auskunft an einen Nutzer, der gerade seinen Bestand wiederherstellt,
und sie war bis `d771ec6` da.

**Gefunden:** coderev, Durchsicht des Commits `d771ec6` am 260821-1401, Bereich
`073448e..d771ec6`

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:609-624` (der Leerbefund-Zweig),
`:289-291` (die Zusage an `Beiseite`), `:294-321` (`Beiseite::Nicht`), `:366-414` (`Display`),
`:822-826` (`beiseite_legen`, die entfallene Frage)

**Domain:** code

**Verwandt:**
`shared/issues/260821-1023_c_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`
— der behobene Defekt; dieser Befund steht an seiner Behebung und widerspricht ihr nicht.
`shared/issues/260821-0142_o_eine-nicht-lesbare-ablagedatei-wird-nicht-gesichert-und-vom-naechsten-schreibvorgang-ueberschrieben.md`
— `Beiseite::Nicht` hat mit `d771ec6` einen dritten Erzeuger bekommen, und der Schlusssatz
jenes Datensatzes gilt für ihn mit. Der Modulkopf schreibt es seit `d771ec6` aus
(`mod.rs:162-167`), der Datensatz selbst führt die dritte Gestalt noch nicht.
`shared/issues/260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
— ob der Nutzer die Meldung aus Schritt 1 überhaupt gesehen hat, hängt daran.
