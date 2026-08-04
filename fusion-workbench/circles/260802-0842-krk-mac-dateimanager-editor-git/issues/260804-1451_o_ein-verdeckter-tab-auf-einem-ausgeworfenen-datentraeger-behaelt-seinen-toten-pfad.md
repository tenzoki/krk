Ein verdeckter Tab auf einem ausgeworfenen Datenträger behält seinen toten Pfad

---

S14 holt beim Auswerfen eines Datenträgers den **sichtbaren** Tab jedes
betroffenen Dateifensters auf das Benutzerverzeichnis herunter. Ein verdeckter
Tab, der auf denselben Datenträger zeigt, bleibt stehen. Wechselt der Nutzer
später auf ihn, sieht er eine leere Liste; erst der nächste Lesevorgang meldet
den Grund in der Statuszeile.

---

**Warum es so gebaut ist.** C9 formuliert die Zusage am Dateifenster: "Wird ein
eingehängtes Volume während der Arbeit ausgeworfen, meldet das betroffene
Dateifenster den Verlust und wechselt auf einen erreichbaren Ordner." Die
Meldung kann nur der sichtbare Tab tragen, weil die Statuszeile zum
Dateifenster gehört und nicht zum Tab. Für den verdeckten Tab gäbe es also
einen Wechsel ohne Meldung, und `crates/krk-ui/src/auffrischung.rs` müsste
dafür eine zweite Regel bekommen.

**Wie groß der Schaden ist.** Klein. Der verdeckte Tab hält einen Pfad, den es
nicht mehr gibt; beim Hinwechseln liest KRK ihn, scheitert und meldet den
Grund. Das ist derselbe Weg, den jeder inzwischen gelöschte Ordner nimmt, und
er ist nicht still. Der Fall tritt außerdem nur ein, wenn der Nutzer zwei Tabs
desselben Dateifensters auf denselben Datenträger gestellt hat.

**Was zu entscheiden wäre.** Drei Möglichkeiten: es so lassen und die
Einschränkung in C9 ausschreiben; jeden Tab herunterholen und die Meldung
weiterhin nur einmal je Dateifenster zeigen; oder den verdeckten Tab beim
nächsten Hinwechseln stillschweigend auf das Benutzerverzeichnis stellen. Die
zweite ist die geradlinigste, verlangt aber eine Naht in
`crates/krk-ui/src/tabs.rs`, die es heute nicht gibt.

**Aufgefallen bei:** der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.
