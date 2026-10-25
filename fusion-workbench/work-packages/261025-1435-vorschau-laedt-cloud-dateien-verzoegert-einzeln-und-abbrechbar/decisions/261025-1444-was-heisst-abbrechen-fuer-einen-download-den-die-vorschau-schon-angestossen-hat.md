# Was heißt „abbrechen“ für einen Download, den die Vorschau schon angestoßen hat?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1435-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1109-warum-die-navigation-auf-google-drive-zuweilen-langsam-ist.md`, `261025-1110-die-vorschau-liest-platzhalterdateien-eines-cloudlaufwerks-schon-beim-bewegen-der-auswahl-und-ein-auswahlwechsel-bricht-das-herunterladen-nicht-ab.md`

---

## Question

Der Nutzer verlangt: „nur ein download zur zeit, wenn cursor weitergeht, abbrechen“. Für einen Download, der noch nicht begonnen hat, ist das Abbrechen eine Sache von KRK und vollständig baubar: die Vorschau öffnet die Datei dann einfach nicht. Für einen Download, der schon läuft, steht der Faden `krk-vorschau` im `open(2)` oder im ersten `read(2)` und wartet, bis der File-Provider-Prozess die Datei geholt hat. **Die SDK-Köpfe zeigen keinen Weg, auf dem ein fremdes Programm diesen Abruf beim Provider beenden kann** (gelesen am 261025 in `FileProvider.framework/Headers`): `NSFileProviderReplicatedExtension.h` sagt zu `fetchContentsForItemWithIdentifier`, das System rufe `cancel` auf dem `NSProgress` des Abrufs, wenn der Nutzer ihn abbricht oder wenn er keinen Fortschritt mehr macht; `NSFileProviderManager.h` bietet für eine Domäne `requestDownloadForItemWithIdentifier` und `evictItemWithIdentifier` an, aber kein Abbrechen, und den Manager einer fremden Domäne bekommt KRK `inference:` nicht, weil `managerForDomain:` die Domäne des eigenen Providers erwartet. `NSFileCoordinator.h` sagt zu `cancel`, dass ein wartender `coordinate…`-Aufruf sofort mit `NSUserCancelledError` zurückkehrt. Ob der Provider seinen Abruf dann ebenfalls beendet, sagt kein Kopf, und gemessen ist es nicht. Die Frage ist, welche Eigenschaft der Plan „abbrechen“ nennt, bevor er gebaut wird, denn die Antwort entscheidet, ob ein zweiter Leseweg über `NSFileCoordinator` entsteht.

## Options

1. **KRK hört auf zu warten und beginnt nichts Neues, der Abruf des Systems läuft zu Ende.** Ein Auswahlwechsel setzt die Abbruchmarke des Vorgangs. Ein Faden, der noch in der Wartezeit von 0,2 s oder vor dem einen Ladeplatz steht, endet, ohne die Datei zu öffnen. Ein Faden, der schon liest, prüft die Marke zwischen zwei Lesestücken und endet dort; das Ergebnis wird verworfen. Den Ladeplatz gibt er erst frei, wenn sein Systemaufruf zurückgekehrt ist, sodass höchstens ein von der Vorschau ausgelöster Download zur Zeit läuft, auch ein überholter. Die Größengrenze begrenzt, was ein überholter Download kostet.
   - Pros: kein neuer Leseweg, keine neue Schnittstelle, keine Klasse aus Foundation im Modell; jede Zusage ist an KRKs eigenem Code prüfbar. Auf einem Provider, der stückweise holt, und auf einem Netzlaufwerk über NetFS bricht das stückweise Lesen den Abruf wirklich ab (`inference:` für Google Drive nicht gemessen).
   - Cons: auf einem Provider, der die ganze Datei holt, bevor `open(2)` oder das erste `read(2)` zurückkehrt, läuft der überholte Download bis zum Ende weiter, und der nächste wartet auf ihn, im schlimmsten Fall um die Zeit, die die Grenze (64 MB) über die Leitung braucht.
2. **Lesen über einen koordinierten Zugriff mit `NSFileCoordinator`, abgebrochen über `cancel`.** Die Vorschau liest eine Platzhalterdatei innerhalb von `coordinateReadingItemAtURL:options:error:byAccessor:`; ein Auswahlwechsel ruft `cancel` auf dem Koordinator des Vorgangs.
   - Pros: KRK wartet nachweislich nicht mehr (Kopf `NSFileCoordinator.h`, `cancel`); `speculation:` der Provider könnte den Abruf mit dem letzten Wartenden beenden.
   - Cons: ob der Abruf endet, ist unbelegt und vor dem Bau nur durch eine Messung mit einem echten Download zu klären; ein zweiter Leseweg neben `bis_zur_grenze_lesen` entsteht, und er braucht eine Foundation-Klasse auf dem Arbeitsfaden, also im Modell, das heute keine Zeile `objc2` trägt, oder einen Umweg über `appkit/`.
3. **Ein Hilfsprozess liest, und ein Auswahlwechsel beendet ihn.** Nach dem Muster der Anmeldehilfe startet KRK sich selbst als Leser und beendet den Prozess bei einem Wechsel.
   - Pros: der Faden von KRK ist sofort frei.
   - Cons: ob der Provider einen Abruf beendet, dessen Anfordernder stirbt, ist so unbelegt wie bei Möglichkeit 2; ein Prozess je Download, eine Röhre für die Bytes und ein zweiter Startweg des Binärziels sind viel Fläche für eine unbelegte Wirkung.

## Constraints

- Kein C-Code im Bau, kein Paket mit einem Namen auf `-sys` und kein `cc` auf den Mac-Zielen.
- `#![deny(unsafe_code)]` außerhalb von `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`.
- Der Hauptfaden stellt keinen Systemaufruf, der auf das Netz warten kann.
- Was ein Agent nicht gemessen hat, steht als `inference:` und wird nicht zur Zusage.

## Recommendation

Möglichkeit 1, vorbehaltlich einer Messung. Sie liefert alles, was KRK selbst in der Hand hat, und jede Möglichkeit danach baut auf ihr auf: Wartezeit, Ladeplatz, Grenze und das stückweise Lesen braucht auch ein koordinierter Leseweg. Möglichkeit 2 lohnt erst, wenn eine Messung an einem echten Platzhalter zeigt, dass `cancel` den Abruf beim Provider beendet; der Plan nennt diese Messung unter `## Open Questions`, und ihr Ergebnis wäre ein neuer Entscheid, der diesen hier ablöst. Möglichkeit 3 empfehlen wir nicht.

---
Answered: 261025-1444-was-heisst-abbrechen-fuer-einen-download-den-die-vorschau-schon-angestossen-hat.md `## Recommendation` — Möglichkeit 1; ruled by user, Kai Stalmann <kai@stalmann.org>, am 261025 mit der Freigabe des Plans.
