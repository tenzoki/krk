# Die Vorschau lädt Cloud-Dateien verzögert, einzeln und abbrechbar

## Directive

Auf einem Cloud-Laufwerk (Google Drive im Streaming-Modus) soll das Bewegen der Auswahl flüssig bleiben, während die Vorschau eine nur in der Cloud liegende Datei weiterhin herunterlädt und zeigt. Die Vorgaben des Nutzers, wörtlich vom 261025: „krk sollte in der vorschau schon downloaden / größenlimit einführen / nur ein download zur zeit, wenn cursor weitergeht, abbrechen. / download erst nach 0,2 sec starten, so dass bei schnellen durchscrollen keiner angestoßen wird“. Erreicht ist das Ziel, wenn schnelles Durchscrollen eines Ordners mit Platzhalterdateien keinen Download anstößt, nach 0,2 s Verweilen höchstens ein Download läuft, ein Auswahlwechsel ihn abbricht und eine Datei über der Größengrenze gar nicht geladen wird.

Grundlage: `shared/analyses/261025-1109-warum-die-navigation-auf-google-drive-zuweilen-langsam-ist.md` und die Defekte `shared/issues/261025-1109-bis-zur-grenze-lesen-oeffnet-die-datei-vor-der-groessenpruefung-und-holt-auf-einem-cloudlaufwerk-jede-zu-grosse-datei-herunter.md` und `shared/issues/261025-1110-die-vorschau-liest-platzhalterdateien-eines-cloudlaufwerks-schon-beim-bewegen-der-auswahl-und-ein-auswahlwechsel-bricht-das-herunterladen-nicht-ab.md`.
