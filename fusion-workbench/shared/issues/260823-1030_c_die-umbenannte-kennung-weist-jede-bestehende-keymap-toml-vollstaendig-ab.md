Die umbenannte Kennung weist jede bestehende `keymap.toml` vollständig ab

---

`28cbb7b` benennt die Belegungskennung `editor_aus_vorschau` in `editor_rundweg` um. Wer eine
eigene `keymap.toml` hat, verliert damit beim ersten Start der neuen Fassung **seine ganze
Belegung** und nicht nur diese eine Zeile — und ein späteres Zuweisen einer Taste in der
Belegungsansicht schreibt den Verlust auf die Platte fest.

Auf dem Referenzgerät liegt keine `keymap.toml` (`ls ~/Library/Application Support/KRK/` am
260823-1020: `bookmarks.toml`, `note-1.txt`, `note-2.txt`, `schreiben.lock`, `session.toml`,
`settings.toml`, `sitzungsrecht.lock`). Der Defekt trifft hier also niemanden. Seit der Runde 15
gibt es eine öffentliche Releaseseite, und dort trifft er jeden, der KRK jemals eine Taste
umbelegt hat.

---

**Aus dem Baum gelesen, nicht am laufenden Bündel bestätigt.** Der Abnahmelauf verlangt KRK im
Vordergrund und ist Nutzerarbeit; nachvollziehbar wäre der Defekt, indem man eine `keymap.toml`
mit dem alten Bezeichner anlegt und KRK startet.

## Die Kette

**1. Der erste unbekannte Bezeichner bricht das ganze Einlesen ab.**
`crates/krk-core/src/tasten/belegung.rs:1420-1424`:

```rust
if let Some(wortschatz) = wortschatz
    && wortschatz.funktion(&eintrag.id).is_none()
{
    return Err(Belegungsfehler::UnbekannteFunktion(eintrag.id.clone()));
}
```

Ein `return Err` in der Schleife über alle Einträge. Kein Überspringen, kein Sammeln — die
Funktion ist an dieser Stelle zu Ende, und mit ihr die ganze Datei.

**2. Der Fehler führt auf die Auslieferungsbelegung zurück.**
`crates/krk-core/src/tasten/belegung.rs:1493-1513`: `laden` beantwortet jeden
`Belegungsfehler` mit `wert: Belegung::auslieferung()` und einer `Ersetzung`. Die Datei auf der
Platte bleibt stehen, und KRK meldet den Grund — beides richtig und beides kein Trost: die
Sitzung läuft mit der Auslieferungsbelegung, jede Umbelegung des Nutzers ist weg.

**3. Betroffen ist jede vorhandene `keymap.toml`, nicht nur eine mit einer Zeile für `cmd+e`.**
`Belegung::sichern` schreibt `Belegungsdatei::from(self)`, und dieser Rückweg trägt **jede**
Funktion mit (`belegung.rs:1651-1677`). Eine Datei, die KRK selbst geschrieben hat, führt damit
notwendig auch `editor_aus_vorschau`. Wer seit der Runde 7 einmal eine Taste in der
Belegungsansicht zugewiesen hat, hat genau eine solche Datei.

**4. Der nächste Schreibvorgang macht den Verlust dauerhaft.** Nach dem Rückfall hält KRK die
Auslieferungsbelegung im Speicher. Weist der Nutzer danach irgendeine Taste zu, schreibt
`sichern` diesen Stand über seine `keymap.toml`. Die alte Datei ist dann überschrieben, und die
Meldung aus Schritt 2 hat er längst weggeklickt.

## Verhältnis zum offenen Datensatz `260814-0656`

`shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`
beschreibt die schwächere Gestalt: eine **hinzugefügte** Funktion kommt beim Nutzer unbelegt an.
Eine **umbenannte** Kennung ist die stärkere: dort kommt eine Funktion tot an, hier kommt die
ganze Datei nicht an. Der Datensatz von damals nennt drei Wege und keiner ist gegangen; keiner
der drei löst den Umbenennungsfall, denn alle drei setzen voraus, dass das Einlesen überhaupt
bis zum Vergleich kommt.

## Was zu entscheiden ist

Die Umbenennung selbst ist sachlich richtig: `editor_aus_vorschau` beschreibt nicht mehr, was
der Befehl tut. Der Commit nennt den Preis in seiner Nachricht und hat ihn auf dem
Referenzgerät geprüft. Was fehlt, ist die Behandlung des Falls bei allen anderen. Vier
Möglichkeiten, keine davon hier gewählt:

1. **Ein Umbenennungsverzeichnis im Kern.** `alt → neu` an einer Stelle, das `Belegung::bauen`
   vor dem Wortschatzvergleich anwendet. Kostet einen dauerhaft wachsenden Eintragsbestand und
   löst jeden künftigen Umbenennungsfall mit.
2. **Ein unbekannter Bezeichner wird übersprungen und gemeldet**, statt die Datei abzuweisen.
   Ändert das Verhalten für jeden Tippfehler mit und ist damit die weitreichendste der vier.
3. **Die alte Kennung bleibt stehen**, nur `name` und Kommentar wechseln. Kostet, dass der
   Bezeichner und die Sache auseinanderlaufen — genau das, was die Umbenennung behoben hat.
4. **Es bleibt, wie es ist**, und die Releaseseite sagt es dem Nutzer: „vor dem ersten Start
   der neuen Fassung `keymap.toml` löschen oder `editor_aus_vorschau` von Hand in
   `editor_rundweg` ändern." Kostet nichts am Code und verlässt sich darauf, dass der Nutzer
   den Hinweis liest, bevor er startet.

Möglichkeit 4 ist die einzige, die ohne Codeänderung auskommt, und sie trifft dieselbe Sorte
Betriebsregel, die die Runde 15 für den Bestandsverlust bei der Installation aufgestellt hat
(`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`). Die Wahl gehört dem
Nutzer.

## Was der Kommentar in der Belegungsdatei heute sagt

`resources/default-keymap.toml:825-829` sagt es dem Leser abgeschwächt:

```
# **Der Eintrag hiess bis zum 260823 "editor_aus_vorschau" ...** Eine
# `keymap.toml`, die den alten Bezeichner noch fuehrt, wird beim Start als
# unbekannte Funktion abgewiesen; KRK meldet es und faehrt mit dieser
# Auslieferungsbelegung.
```

„Abgewiesen" liest sich, als träfe es den Eintrag. Es trifft die Datei. Die Commit-Nachricht
schreibt es richtig aus („komplett abgewiesen, nicht nur an dieser Zeile"); die Stelle, an der
ein Nutzer nachschlägt, tut es nicht.

**Schwere:** High. Kein Absturz, keine Sperre, aber der Verlust einer Nutzereinstellung ohne
Zutun des Nutzers, mit einem zweiten Schritt, der ihn festschreibt.

**Filed by:** coderev

---
Resolved: Als Lage angenommen, nicht behoben. Der Nutzer hat am 260823-1125 entschieden: die
Umbenennung auf `editor_rundweg` bleibt, und der Grund ist, dass es **noch keine Nutzer gibt**.
Ohne eine einzige `keymap.toml` im Umlauf hat der Defekt heute kein Opfer, und weder die
Rücknahme des Namens noch eine Umstiegstabelle wären ihren Preis wert.

**Der Mechanismus besteht unverändert fort, und der Auslöser ist benannt.** `Belegung::bauen`
(`crates/krk-core/src/tasten/belegung.rs:1423`) bricht beim ersten unbekannten Bezeichner mit
`return Err` ab, `Belegungsdatei::from` (`:1660`) schreibt alle 85 Funktionen statt der geänderten,
und `belegungsansicht_verlassen` (`crates/krk-ui/src/appkit/anwendung.rs:3653`) sichert bei jeder
Änderung. Jede künftige Umbenennung einer Kennung trifft damit jede bestehende Belegungsdatei,
und zwar ganz und nicht an der einen Zeile.

**Was die Annahme aufhebt:** der erste Nutzer außer dem Entwickler. Ab dann ist eine Umbenennung
ohne Umstiegsweg ein Datenverlust beim Nutzer, und dieser Datensatz ist gegen den Baum wieder zu
lesen, statt neu erhoben zu werden.

---
Revised by: 260823-1140, dieselbe Sitzung — die Begründung der Schließung darüber war falsch
belegt, das Ergebnis bleibt.

**Falsch war „es gibt noch keine Nutzer".** Der Nutzer betreibt KRK auf zwei Maschinen, und die
zweite ist genau der Fall, für den dieser Datensatz geschrieben ist. Der Satz stammt aus einer
Antwort auf eine Frage, die die zweite Maschine nicht kannte.

**Richtig und geprüft ist:** auf keiner der beiden Maschinen liegt eine
`~/Library/Application Support/KRK/keymap.toml`. Der Nutzer hat am 260823-1140 auf der zweiten
nachgesehen und keine gefunden, für das Referenzgerät steht die Prüfung weiter oben. Ohne eine
solche Datei hat die Umbenennung nichts, woran sie scheitern könnte; sie ist damit folgenlos, und
nicht bloß ungefährlich mangels Publikum.

**Der Auslöser ist damit schärfer als oben formuliert.** Es genügt nicht, dass ein Nutzer
hinzukommt. Gefährlich wird erst die Verbindung aus beidem: jemand hat eine `keymap.toml`, weil er
einmal eine Taste umbelegt hat, **und** installiert danach eine Fassung, die eine Kennung
umbenannt hat. Wer künftig eine Kennung umbenennt, prüft deshalb nicht „gibt es Nutzer", sondern
„liegt irgendwo eine `keymap.toml`" — und die Antwort darauf kennt der Entwickler nur für seine
eigenen Geräte.

**Die Gegenmaßnahme ist erhoben und kostet nichts:** in der betroffenen Datei die alte Kennung auf
die neue umschreiben. Der Anzeigename in derselben Zeile bliebe der alte, weil `Belegung::bauen`
(`crates/krk-core/src/tasten/belegung.rs:1446-1447`) `name` aus der Nutzerdatei übernimmt und
nicht aus der Auslieferung; das ist kosmetisch. Das Löschen der Datei wirkt ebenfalls, kostet aber
die eigene Belegung. **Nie den Ordner löschen**, nur die eine Datei: daneben liegen Lesezeichen,
Sitzung, Notizzettel und Einstellungen.

---
**Nachtrag 260823-1710.** Der Abgleich zur Auslieferung von 1.0.0 hat die Grundlage dieser
Schließung erneut geprüft und einen Einwand gefunden: die öffentliche Releaseseite weist für
`KRK-0.5.6.zip` vier Ladevorgänge aus, für `KRK-1.0.0.zip` binnen anderthalb Stunden zwei. Der
Satz „auf keiner der zwei Maschinen liegt eine `keymap.toml`" spricht für die Geräte des
Entwicklers und kann für Ladende nicht sprechen.

Der Nutzer hat den Einwand am 260823-1710 entkräftet: die Ladevorgänge sind seine eigenen. Die
Schließung steht damit unverändert, und ihr Auslöser bleibt der oben formulierte — nicht „es gibt
Nutzer", sondern „jemand hat eine `keymap.toml` **und** installiert danach eine Fassung mit
umbenannter Kennung".
