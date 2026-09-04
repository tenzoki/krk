Das atomare Schreiben weitet die Rechte einer 600-Datei auf 644

---
`ablage::atomar::vorbereiten` (`crates/krk-core/src/ablage/atomar.rs`) legt die Nachbardatei mit `fs::File::create` an. Die bekommt damit `0666 & ~umask`, also auf diesem Gerät `644`, und das anschließende `rename(2)` setzt diesen Modus an die Stelle des Ziels. **Die Rechte des Ziels gehen dabei verloren und werden aufgeweitet**, wenn das Ziel enger stand als die umask des Prozesses.

**Gemessen am 260904-1848**, ohne Netzlaufwerk, in einem Wegwerfordner:

```
vorher:  -rw-------  geheim.txt          (chmod 600)
nach `krk_core::text::datei::sichern`:
         -rw-r--r--  geheim.txt
```

Betroffen ist jeder Rufer von `atomar::schreiben`, also das Sichern aus dem Editor **und** die Ablagedateien unter `~/Library/Application Support/KRK/`. Für den Editor wiegt es am schwersten: eine Datei, die der Nutzer bewusst auf `600` gesetzt hat, ist nach dem ersten `cmd+s` für jeden Nutzer des Geräts lesbar, und nichts sagt es ihm. Die Dateien unter Google Drive stehen dort durchweg auf `600`.

**Gefunden bei der Untersuchung von `260904-1827` und nicht behoben**, weil die Behebung eine Entscheidung verlangt, die diesem Defekt nicht gehört: ob die Nachbardatei die Rechte des Ziels erbt (dann braucht `vorbereiten` ein `fstat` am Ziel und ein `fchmod` am Deskriptor, und für ein noch nicht bestehendes Ziel eine Vorgabe), oder ob sie eng anfängt und erst beim Umbenennen aufgeht.

---
**Filed by:** bugfixer, Kai Stalmann <kai@qantr.com>
**Domain:** code
