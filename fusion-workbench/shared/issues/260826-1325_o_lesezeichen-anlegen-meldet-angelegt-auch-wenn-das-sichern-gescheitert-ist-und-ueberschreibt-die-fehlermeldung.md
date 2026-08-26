`lesezeichen_anlegen_ausfuehren` meldet „Lesezeichen angelegt", auch wenn die Sperre nicht zu nehmen oder das Schreiben gescheitert ist, und überschreibt damit die Fehlermeldung

---

`Anwendungsdelegierter::lesezeichen_anlegen_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:2135-2148`) ruft `lesezeichen_aendern` und schreibt danach **unbedingt** `Lesezeichen „…“ angelegt` als Befehlsantwort. `lesezeichen_aendern` hat auf zwei seiner Ausgänge gerade eine Fehlermeldung in dieselbe Befehlsantwort gestellt; die zweite Zeile ersetzt sie, und der Nutzer liest „angelegt", während `bookmarks.toml` den Eintrag nicht trägt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum

`anwendung.rs:2140-2147`:

```
self.lesezeichen_aendern(seite, &Aenderung::Anlegen { … });
self.antwort_zeigen(seite, &format!("Lesezeichen „{}“ angelegt", name.trim()));
```

`lesezeichen_aendern` (`anwendung.rs:1902-1970`) meldet und kehrt zurück:

- `Err(Sperrhindernis::Gesperrt(fehler))` → „die Lesezeichen liessen sich nicht aendern, die Schreibsperre der Ablage ist nicht zu nehmen" (`1922-1931`), **ohne** die Liste zu ändern;
- `geschrieben == Some(Err(fehler))` → „die Lesezeichen liessen sich nicht sichern" (`1964-1969`); die Leiste zeigt den Eintrag, die Datei trägt ihn nicht.

Beide Sätze gehen über `antwort_zeigen` in die Befehlsantwort des Dateifensters, und `DateifensterQuelle::befehlsantwort_zeigen` **ersetzt** den Wert (`crates/krk-ui/src/appkit/tabelle.rs:3306-3309`: `*self.ivars().befehlsantwort.borrow_mut() = Some(antwort.to_owned())`). Die Zeile `2147` läuft in jedem Fall danach und gewinnt.

Der Rückgabewert fehlt: `lesezeichen_aendern` liefert `()`, also hat der Aufrufer keine Handhabe, den Erfolg zu unterscheiden. Die drei Geschwister (`lesezeichen_umbenennen_ausfuehren`, `lesezeichen_loeschen`, `lesezeichen_verschieben`) schreiben keine eigene Erfolgsmeldung und sind nicht betroffen.

## Warum das zählt

Die Betriebsregel aus `CLAUDE.md` („KRKs Bestand liegt außerhalb des Bündels") und der Analysebericht `shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md` machen die Lesezeichen zur Datei, deren Verlust dem Nutzer am teuersten war. Eine Meldung „angelegt" über einem gescheiterten Schreiben ist genau die Auskunft, die ihn beim nächsten Start überrascht.

## Vorschlag

`lesezeichen_aendern` liefert `bool` (geschrieben oder ohne Ablageordner gerechnet) oder das `Result` seines Durchgangs, und `lesezeichen_anlegen_ausfuehren` meldet „angelegt" nur auf `true`. Die Meldung „ohne Ablageordner gerechnet, nicht gesichert" bleibt beim Start, wie der Doc-Kommentar es vorsieht.

Gefunden bei der Vollbaum-Durchsicht R7 an HEAD `7ac511a`.
