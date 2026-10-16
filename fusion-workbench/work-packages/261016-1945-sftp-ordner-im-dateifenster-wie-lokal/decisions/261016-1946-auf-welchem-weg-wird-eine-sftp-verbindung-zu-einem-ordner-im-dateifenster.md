# Auf welchem Weg wird eine sftp-Verbindung zu einem Ordner im Dateifenster?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md, 261016-1945-sftp-ordner-im-dateifenster-wie-lokal.md

---

## Question

macOS 15 hängt sftp ohne Drittsoftware nicht ein (NetFS kennt kein sftp, FSKit kann Netzdateisysteme erst ab macOS 26), und jede fertige Rust-Kiste für SFTP bringt C mit. Wie öffnet KRK eine sftp-Verbindung so, dass sie im Dateifenster wie ein lokaler Ordner wirkt?

## Options

1. **sshfs, von KRK gestartet** — FUSE-T oder macFUSE plus sshfs installiert der Nutzer.
   - Pros: sofort der volle Umfang, auch Vorschau, Editor, Git.
   - Cons: Drittsoftware; ein hängender Server trifft den Hauptfaden.
2. **KRK spricht SFTP selbst über `/usr/bin/ssh -s … sftp`** — kein Installieren, keine neue Kiste, eine Ortsabstraktion (`Ort`: lokal oder entfernt) in Tabs, Aufträgen und Lesezeichen.
   - Pros: keine Drittsoftware, C-Freiheit hält.
   - Cons: großer Umbau; Git, Inhaltsfilter, Zip zunächst ausgegraut; entfernte Ordner laden mit Netzlaufzeit.
3. **FSKit oder File Provider** — nicht empfohlen (macOS 26 bzw. zweite Programmeinheit).

## Constraints

C-Freiheits-Zusage (kein `cc`, kein `-sys` auf beiden Mac-Zielen); lokale Ordner und die zehn Zeitzusagen aus C8 bleiben unberührt.

---
Answered: 261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md — Möglichkeit 2, KRK spricht SFTP über das ssh des Systems; ruled by user, Kai Stalmann <kai@stalmann.org>
