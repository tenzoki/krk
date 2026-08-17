# Das Konfliktblatt gibt seinem Namensfeld keinen Eingabewächter

**Datum:** 260817-1241
**Gefunden von:** coder, beim Nachziehen aller Blätter für T1
**Schwere:** Mittel
**Betrifft:** `crates/krk-ui/src/appkit/blaetter/konflikt.rs`
**Baumstand:** `472eb81` und der Stand nach T1 — an dieser Stelle ist nichts angefasst

## Der Befund

`konflikt.rs:102` hängt sein Namensfeld mit `beigabe_setzen` unter die Frage und ruft
weder `textfeld_setzen` noch `waechter_anhaengen`. Es ist damit das eine Blatt im Baum mit
einem Textfeld **ohne** `Eingabewaechter`; die fünf übrigen Feldblätter (`pfadeingabe`,
`namenseingabe`, `zeilennummer`, `suche`, `stapelumbenennen`) haben einen.

Der Modulkopf von `blaetter/mod.rs` sagt, wozu der Wächter da ist, und nennt die Messung:
„Am laufenden Bündel gemessen am 260804: ohne ihn lässt sich das Blatt weder mit der
Eingabe- noch mit der Escape-Taste schließen." Ein Textfeld im Bearbeitungszustand
verbraucht beide Tasten selbst.

Solange der Nutzer nicht in das Feld klickt, ist nichts betroffen: `konflikt.rs` ruft kein
`ersthelfer_setzen`, der Ersthelfer ist also eine Schaltfläche und die vier
Tastenentsprechungen greifen. Sobald er hineinklickt, um den Namen für „Umbenennen" zu
tippen, hält das Feld den Ersthelfer.

Dann greift daneben `kommandos::zulaessigkeit::zulaessig` nicht mehr: `abbrechen` steht
nicht in `immer_erreichbar` (`zulaessigkeit.rs:197-202`), und `ersthelfer_gehoert_appkit`
ist bei einem Textfeld wahr. Der Weg zum Abbruch, auf den sich `loeschbestaetigung` beruft,
steht hier also auch nicht offen.

## Was nicht belegt ist

Ob der Feldeditor `Opt+Return` (die Taste von „Umbenennen") wirklich verbraucht. Verbraucht
er es, ist keine der vier Antworten des Blattes mit der Tastatur erreichbar, sobald der
Nutzer im Feld getippt hat, und „Umbenennen" ist überhaupt nur mit der Maus zu erreichen.
Verbraucht er es nicht, bleiben Eingabetaste und Escape betroffen. Beides ist am laufenden
Bündel zu messen, und das verlangt KRK im Vordergrund, also Nutzerarbeit.

Der Befund steht auf Mittel und nicht höher, weil eine Antwort mit der Maus in jedem Fall
möglich bleibt.

## Richtung

`waechter_anhaengen(mtm, &feld)` an der einen Stelle, so wie es die fünf übrigen
Feldblätter tun. Zu entscheiden ist dabei, was der Wächter im Konfliktblatt bedeuten soll:
er kennt zwei Antworten (bestätigt und abgebrochen), das Blatt hat vier. Die bestätigende
Seite läuft heute fest auf die **erste** Schaltfläche, und die ist hier „Überschreiben"; das
wäre für einen Return im Namensfeld die falsche Antwort. Die Frage gehört damit an
`blaetter/mod.rs` und nicht an dieses Blatt, siehe
`260817-1242_o_die-bestaetigende-seite-des-eingabewaechters-liegt-fest-auf-der-ersten-schaltflaeche.md`.
