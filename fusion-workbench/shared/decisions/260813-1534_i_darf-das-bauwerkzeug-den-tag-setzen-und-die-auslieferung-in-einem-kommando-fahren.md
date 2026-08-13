# Darf das Bauwerkzeug den Tag setzen und die Auslieferung in einem Kommando fahren?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (auf Anweisung des Nutzers)
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_*_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md` (der überholte Entscheid); `shared/issues/260813-1515_*_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md`; `xtask/src/release.rs`; `README.md`, Abschnitt „Versionsstufen"

---

## Question

Die achte Runde hat am 260813 festgelegt, dass `cargo xtask release` einen Tag `v<version>` auf HEAD **verlangt** und ihn **nie erzeugt**. Der Nutzer setzt ihn von Hand. Die Begründung war, dass ein Tag eine Auslieferung benennt und ein Werkzeug, das ihn erzeugt, in die Git-Historie schreibt.

Am selben Tag, wenige Stunden nach der Umsetzung, hat der Nutzer einen Auslieferungsweg in **einem** Kommando mit **einem** Argument verlangt: die Versionsnummer. Ein solcher Weg setzt notwendig den Tag, denn sonst bräuchte er zwei Kommandos und die Ersparnis entfiele.

Die beiden Festlegungen stehen im Widerspruch. Diese hier hebt die frühere auf.

## Options

1. **Das Werkzeug setzt den Tag und liefert aus, ein Kommando, ein Argument.** Der frühere Entscheid wird überholt.
   - Pro: der Auslieferungsweg ist ein Handgriff. Die Versionszahl wird an einer Stelle gesetzt, und alles Weitere folgt daraus mechanisch.
   - Contra: das Werkzeug schreibt in die Git-Historie. Ein Fehllauf hinterlässt einen Tag, der von Hand abzuräumen ist.
2. **Bei der früheren Festlegung bleiben**, dazu einen Abschnitt in `README.md` mit den vier Zeilen, die der Nutzer selbst tippt.
   - Pro: der Tag behält seine Bedeutung als bewusster Akt.
   - Contra: beantwortet die Frage des Nutzers nicht, sondern wiederholt die Ablehnung.
3. **Nur die Version setzen**, das Taggen bleibt Handarbeit.
   - Pro: der frühere Entscheid bleibt unberührt.
   - Contra: spart fast nichts; der Rest ist ohnehin ein Kommando.

## Constraints

- Die Versionszahl bleibt einquellig in `[workspace.package]` der Wurzel-`Cargo.toml`. Ein zweiter Ort ist ausgeschlossen.
- `xtask` liest die Zahl über `env!("CARGO_PKG_VERSION")` zur Übersetzungszeit. Wer sie ändert, muss `xtask` neu übersetzen lassen, bevor die Tag-Prüfung die neue Zahl kennt.
- Die Prüfung auf einen unveränderten verfolgten Arbeitsbaum bleibt. Sie ist der Kern dessen, was die achte Runde gebaut hat, und wird von dieser Antwort nicht berührt.
- Das Projekt hat **ein** Bauwerkzeug, `xtask`, und eine Hülle darum, das `Makefile`. Ein drittes Werkzeug daneben ist ausgeschlossen; ein Skript darf nur weiterreichen.

## Recommendation

Möglichkeit 1, vom Nutzer am 260813-1534 gewählt.

---
Answered: Der Nutzer am 260813-1534 wörtlich: „Wir nehmen den Entscheid einfach zurück, das Werkzeug soll taggen, releasen und fertig." Damit gilt Möglichkeit 1. Der frühere Entscheid `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_s_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md` ist überholt.
Implemented: 801f5cc — ./release.sh <zahl> setzt die Zahl, traegt sie ein, taggt und liefert aus; die Logik in xtask/src/version.rs und xtask/src/git.rs, die Huellen in Makefile (Ziel `ausliefern`) und release.sh.
Deferred:
Superseded by:
