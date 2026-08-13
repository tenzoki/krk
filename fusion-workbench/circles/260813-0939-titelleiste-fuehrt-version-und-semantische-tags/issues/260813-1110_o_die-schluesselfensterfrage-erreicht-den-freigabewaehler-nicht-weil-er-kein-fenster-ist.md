Die Schlüsselfensterfrage erreicht den Freigabewähler nicht, weil er kein Fenster ist

---

Der Entscheid `decisions/260813-1037_a_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`
führt für Möglichkeit 2 als Vorteil an:

> eine Bedingung an einer Stelle, kein Sonderfall für den Über-Dialog, und der
> offene Defekt zum Freigabedialog fällt mit weg.

Der zweite Halbsatz ist nicht belegt und nach dem, was im Baum steht,
wahrscheinlich falsch. Der Freigabewähler entsteht in
`crates/krk-ui/src/appkit/teilen.rs:222` über
`showRelativeToRect_ofView_preferredEdge`. Das ist eine Verfolgungsschleife an
einer Ansicht und kein eigenes Fenster; der Defekt
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md`
hält genau das fest. Bleibt das Hauptfenster dabei das Schlüsselfenster, dann
antwortet die neue vierte Bedingung mit ja, und sie sperrt nichts.

---

**Die drei Ausgänge, und nur einer schließt den Defekt über die neue Regel**

1. Der lokale Ereignisabgriff wird während der Verfolgungsschleife gar nicht
   gerufen. Dann erreicht kein Tastendruck KRK, der Defekt ist gegenstandslos,
   und die neue Regel hat damit nichts zu tun. Das ist die Vermutung, die der
   Defekt selbst als `inference:` führt.
2. Der Abgriff wird gerufen, und das Schlüsselfenster bleibt das Hauptfenster.
   Dann kommen die Befehle mit `Wirkungsbereich::Ueberall` weiter durch, und die
   neue Regel ändert daran nichts. Der Defekt bleibt offen.
3. Der Abgriff wird gerufen, und der Wähler nimmt den Schlüsselrang. Dann
   schließt die neue Regel den Fall, und der Vorteil aus dem Entscheid trifft zu.

Welcher der drei gilt, ist am Baum nicht zu entscheiden. Zu entscheiden ist es
allein am laufenden Bündel im Vordergrund, und das ist Nutzerarbeit — dieselbe
Grenze, an der jede Abnahme dieses Vorhabens steht.

**Was für den Über-Dialog gilt und hier nicht in Frage steht**

`orderFrontStandardAboutPanel:` öffnet ein eigenes `NSPanel`. Ein Panel nimmt
den Schlüsselrang, also greift die neue Bedingung dort. `inference:` gemessen
ist auch das nicht, aber es hängt nicht an einer Verfolgungsschleife, sondern an
der gewöhnlichen Fensterordnung von AppKit.

**Was zu tun ist**

Zwei Dinge, und sie hängen nicht aneinander.

Erstens den Vorteilssatz im Entscheid berichtigen: die eine Bedingung schließt
jedes fremde **Fenster**, und der Freigabewähler ist keines. Der gewählte Ausgang
Möglichkeit 2 bleibt davon unberührt; sein Grund ist die Verallgemeinerung, nicht
der eine Nebeneffekt.

Zweitens den Defekt zum Freigabedialog **nicht** ungeprüft schließen. Die
Beobachtung, die er selbst verlangt, steht weiter aus: Freigabewähler über
Shift+Cmd+S öffnen und, während er steht, Cmd+W drücken. Der Umsetzungsplan
dieser Runde führt sie als Abnahmepunkt in Strang E.

Gefunden beim Bau des Umsetzungsplans dieser Runde
(`planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
Schritt A3).

---

**Abgleich 260813-1345: zu Recht offen, beide Punkte.**

Der erste ist unerledigt: der Vorteilssatz zu Möglichkeit 2 im Entscheid
`decisions/260813-1037_i_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`
sagt unverändert, der Defekt zum Freigabedialog falle mit weg. Der Abgleich hat die Berichtigung
stattdessen an den Entscheid selbst angehängt, weil er ihn beim Markerwechsel ohnehin öffnete;
der Wortlaut im Abschnitt `## Options` steht weiter da.

Der zweite ist eingehalten: `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md`
trägt weiter `_o_` und ist nicht ungeprüft geschlossen worden. Schritt A3 hat ihm einen
Abschnitt über die Reichweite der neuen Regel angehängt (`:83-113`); die Beobachtung selbst
steht in Planschritt E2 und ist Nutzerarbeit.

Am Baum nachgelesen und bestätigt: `crates/krk-ui/src/appkit/teilen.rs:222` zeigt den Wähler
über `showRelativeToRect_ofView_preferredEdge`. Die drei Ausgänge dieses Datensatzes sind am
Baum weiterhin nicht zu unterscheiden.

Ein Nebensatz jenes Nachtrags ist inzwischen falsch (die Ausnahmeliste führt seit `ed0388e`
drei Befehle, nicht zwei); abgelegt als
`260813-1345_o_der-nachtrag-aus-a3-zaehlt-die-ausnahmeliste-mit-zwei-eintraegen-und-turn-2-hat-einen-dritten-gebracht.md`.
