Der gemessene Start lädt die Lesezeichen nicht, und die Leiste schweigt darüber mit einer Begründung, die für diesen Weg nicht gilt

---

In jeder der vier Messaufgaben kehrt `sitzung_laden` zurück, **bevor** es
`ivars.ablage` setzt. `leiste_einrichten` bekommt danach `Sperrhindernis::OhneOrdner` und
nimmt die leere Lesezeichenliste, ohne etwas zu sagen — mit der Begründung im Kommentar, die
Meldung habe `sitzung_laden` schon gestellt. Auf diesem Weg hat es keine gestellt.
Zwei Folgen: der Lauf, der L4 misst, misst einen Start ohne den Ablagedurchgang für
`bookmarks.toml`, den ein echter Start fährt, und der Grund dafür steht nirgends.

---

**Die vier Rückkehrstellen** in `crates/krk-ui/src/appkit/anwendung.rs`:

- `:1380-1382` — `Aufgabe::Start` und `Aufgabe::Spannen`: `return (Sitzung::default(), Vec::new())`.
- `:1383-1393` — `Aufgabe::Sitzung`: `return (plan.sitzung.clone(), Vec::new())`.
- `:1394-1425` — `Aufgabe::SitzungsStart`: öffnet eine **örtliche** `Ablage`, liest
  `session.toml` und gibt `return (sitzung, Vec::new())`; die örtliche Ablage fällt beim
  Verlassen weg.

`*ivars.ablage.borrow_mut() = Some(ablage)` steht erst bei `:1497`, also auf dem gewöhnlichen
Weg. `leiste_einrichten` (`:1588-1607`) fragt danach über `unter_der_sperre` (`:1358-1366`)
und bekommt `Err(Sperrhindernis::OhneOrdner)`, weil `ivars.ablage` `None` ist.

**Die Begründung im Kommentar hält für zwei von drei Lagen, in denen dieses `None` entsteht**
(`:1598-1600`). Sie hält für `Ablage::im_benutzerverzeichnis()`-Fehlschlag (`:1429-1437`,
Meldung gestellt) und für den fehlgeschlagenen Schreibgriff (`:1481-1488`, Meldung gestellt).
Für die Messaufgaben hält sie nicht: dort ist der Meldungsvektor leer.

**Was daraus für L4 folgt.** `Aufgabe::Start` ist L4 am Durchstich, `Aufgabe::SitzungsStart`
ist L4 auf der Prüfsitzung (`crates/krk-ui/src/messmodus.rs:123-145`). Beide messen also
einen Start, dem gegenüber dem echten ein Weg fehlt: `Ablage::oeffnen` mit `create_dir_all`
und dem Öffnen der Sperrdatei, das Nehmen der Schreibsperre, das Lesen und Zerlegen von
`bookmarks.toml`, und der Aufbau der Leistenzeilen aus den Einträgen. Auf dem
Referenzgerät ist das wenig; gemessen ist es nicht, und die Zusage L4 spricht vom Start.

**Schwere:** gering bis mittel. Die Messstrecke misst nicht denselben Start, den der Nutzer
fährt, und der Unterschied ist im Baum nirgends benannt.

**Gefunden:** analyst, forensische Untersuchung „Lesezeichen nach Installation weg" am 260820-2235

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:1376-1425`, `:1497`, `:1588-1607`,
`crates/krk-ui/src/messmodus.rs:123-145`

**Domain:** code

## Vorschlag

Erst entscheiden, dann bauen: **soll der gemessene Start den Ablagedurchgang der Lesezeichen
mitfahren?** Beide Antworten sind vertretbar und keine ist ableitbar.

- **Ja** — dann setzt auch der Messweg `ivars.ablage`, und L4 misst, was der Nutzer erlebt.
  Der Preis ist, dass die Messläufe die echte `bookmarks.toml` des Nutzers lesen; geschrieben
  wird sie dabei nicht, denn `sichern` hängt allein an einem Lesezeichenbefehl.
- **Nein** — dann bleibt es wie heute, und der Kommentar bei `:1598-1600` sagt aus, was
  wirklich gilt: dass in den Messaufgaben niemand meldet, weil niemand zusieht. Dazu gehört
  ein Satz an L4, der die weggelassene Arbeit benennt.

Der Kommentar ist in beiden Fällen zu berichtigen; er behauptet heute etwas über einen Weg,
den er nicht kennt.
