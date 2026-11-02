# Schuldet das Hauptfadenpaket einen Abnahmelauf gegen die zehn Zusagen aus C8?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261016-1956-der-loeschweg-fragt-canonicalize-und-den-papierkorb-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md, 261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261102-1312-was-heisst-fragt-das-dateisystem-auf-dem-hauptfaden-bei-einer-uebergabe-an-das-system.md

---

## Question

Der Defekt zum Löschweg schreibt in seine Abnahme, dass die Behebung einen Abnahmelauf gegen L1 und L9 schuldet. Das Paket legt Fragen an die Platte vom Hauptfaden auf einen Nebenfaden; auf dem Hauptfaden bleibt je Weg ein Fadenstart und ein Rücksprung über die Hauptschlange. Seit dem 261017 stehen alle zehn Zusagen auf den Gegenständen der späteren Messrunde. Der Lauf verlangt KRK im Vordergrund und ist Nutzerarbeit.

## Options

1. **Messen**, L1 bis L10 am Bündel nach dem letzten Schritt, dazu der Lauf an einer hängenden Freigabe aus `261102-1312-…`.
2. **Nicht messen** — die Zusagen bleiben auf den Gegenständen der späteren Messrunde; der Lauf an einer hängenden Freigabe bleibt eine eigene Nutzerprüfung und kein Abnahmelauf gegen C8.

## Constraints

Kein Agent kann den Lauf fahren. Das Paket liefert nicht aus.

## Recommendation

Möglichkeit 2, wie bei den Paketen seit dem sftp-Paket. Das Paket nimmt dem Hauptfaden Arbeit ab und legt ihm keine neue auf außer dem Fadenstart; einen Grund, die zehn Zusagen gerade jetzt einzeln zu messen, liefert es nicht.

---
Answered: Der Nutzer am 261102 mit „1“ auf die Vorlage „Plan freigeben, A und B wie vorgeschlagen“. Damit gilt Möglichkeit 2: nicht messen, die zehn Zusagen bleiben auf den Gegenständen der späteren Messrunde. Ruled by user, Kai Stalmann <ks@qantr.com>.
