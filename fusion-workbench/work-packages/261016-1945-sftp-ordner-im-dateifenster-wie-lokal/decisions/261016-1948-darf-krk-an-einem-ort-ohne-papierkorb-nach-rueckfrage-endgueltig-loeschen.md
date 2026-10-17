# Darf KRK an einem Ort ohne Papierkorb nach Rückfrage endgültig löschen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb, 261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md

---

## Question

Seit der Runde 12 (`260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`) kennt KRK allein das Räumen in den Papierkorb; ein Ziel ohne Papierkorb wird gemeldet statt gelöscht. Ein SFTP-Server hat keinen Papierkorb, und eine eingehängte smb-Freigabe vermutlich auch nicht. Darf KRK dort endgültig löschen?

## Options

1. **Ja, nach einer eigenen Rückfrage, die „endgültig“ sagt**, für jeden Ort ohne Papierkorb, also auch smb.
2. **Nein**, Löschen bleibt an solchen Orten gesperrt.

---
Answered: 261016-1948-darf-krk-an-einem-ort-ohne-papierkorb-nach-rueckfrage-endgueltig-loeschen.md — ja („endgültig löschen? ja“), nach einer Rückfrage, die das Endgültige ausspricht, an jedem Ort ohne Papierkorb; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: crates/krk-core/src/operation/loeschen.rs (Modulkopf), crates/krk-ui/src/kommandos/loeschwarnung.rs — eigene Rueckfrage, die endgueltig ausspricht, an jedem Ort ohne Papierkorb; unentschiedener Befund loescht nicht (545c8d31)
