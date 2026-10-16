# Darf das Passwort einer Verbindung KRK außer an NetFS auch an ssh (und sshfs) gehen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0306-auf-welchem-weg-verlassen-zugangsdaten-aus-secrets-txt-krk-wenn-eine-verbindung-hergestellt-wird.md, 261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md

---

## Question

Bisher verlässt ein Passwort aus `secrets.txt` KRK allein als Argument von `NetFSMountURLSync`; `Passwort::preisgeben` hat genau einen Rufer, gehalten von `das_passwort_wird_allein_an_netfs_preisgegeben`. Ein SFTP-Zugang über `/usr/bin/ssh` braucht das Passwort ebenfalls, über `SSH_ASKPASS` mit `SSH_ASKPASS_REQUIRE=force` (ein Hilfsprogramm, das ssh fragt), nie in Befehl, Datei oder einer Umgebungsvariable, die das Passwort selbst trägt.

## Options

1. **Ja, ssh wird der zweite Empfänger**, gehalten wie NetFS: genau ein weiterer Rufer von `preisgeben`, mit Namen in der Probe.
2. **Nein**, sftp nur mit Schlüssel im ssh-Agenten.

---
Answered: 261016-1947-darf-das-passwort-einer-verbindung-krk-ausser-an-netfs-auch-an-ssh-gehen.md — ja („Passwort an ssh und sshfs ? ja“); ssh ist der zweite Empfänger, auf genau einem benannten Weg; ruled by user, Kai Stalmann <kai@stalmann.org>
