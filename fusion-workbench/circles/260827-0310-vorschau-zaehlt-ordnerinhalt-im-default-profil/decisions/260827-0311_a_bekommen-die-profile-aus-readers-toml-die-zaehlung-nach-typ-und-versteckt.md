# Bekommen die Profile aus `readers.toml` die Zählung nach Typ und nach versteckt, oder bleibt sie dem eingebauten Default-Profil vorbehalten?

---
**Domain:** code
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/leseprofil/mod.rs` (`Baustein::Zaehlung`, Zeilen 296-300); `crates/krk-core/src/verzeichnis/eintrag.rs` (`Typ`, Zeilen 16-25; `Eintrag::versteckt`, Zeile 60); `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Abschnitt „Warum es kein fünfter Baustein wird, und kein größerer Deckel"); `shared/decisions/260825-1725_*_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md`

---

## Question

Die drei Zählzeilen dieser Runde trennen nach zwei Größen, die der vorhandene Baustein `zaehlung` nicht kennt: nach dem Typ des Eintrags (Datei, Ordner, Verknüpfung) und danach, ob er versteckt ist. Der Baustein zählt heute Einträge, deren **Name** ein Muster erfüllt, und sieht dabei auf Namen jeden Typs. Die Frage ist nicht, ob KRK diese Trennung leisten kann, denn `Typ` und `Eintrag::versteckt` liegen im Kern bereit. Die Frage ist, wo die Erweiterung wohnt: als Fähigkeit, die jedes Profil in `readers.toml` beschreiben kann, oder als Sonderweg, den allein das eingebaute Default-Profil geht. Sie muss vor dem Plan beantwortet werden, weil sie über die Gestalt der Profildatei entscheidet, die der Nutzer von Hand pflegt, und die Runde 18 aus einem verwandten Anlass bereits gegen einen fünften Baustein entschieden hat.

## Options

1. **Der Baustein `zaehlung` bekommt zwei weitere Kriterien** — neben dem Namensmuster nimmt er einen Typ und eine Behandlung der versteckten Einträge entgegen, beide freiwillig. Das Default-Profil benutzt dieselbe Maschine wie jedes Profil aus `readers.toml`.
   - Pros: Kein zweiter Zählweg im Baum. Der Nutzer kann in seinen eigenen Profilen dasselbe ausdrücken, was er in der Vorschau sieht. Die Zahl der Bausteine bleibt bei vier, und Festlegung A7 bleibt unangetastet.
   - Cons: Die Profildatei wächst um zwei Schlüssel, die der Nutzer verstehen und der Prüfschritt abweisen muss. Jeder neue Schlüssel ist eine Stelle, an der ein Tippfehler eine Meldung verlangt (`issues/260824-1217`).
2. **Das Default-Profil rechnet seine drei Zeilen selbst** — es entsteht als eigener Weg neben der Auswertung der vier Bausteine, und `readers.toml` bleibt unverändert.
   - Pros: Die Profildatei bleibt, wie sie ist. Die drei Zeilen sind fest und brauchen weder Muster noch Prüfschritt.
   - Cons: Ein zweiter Zählweg neben `zaehlung`, der dasselbe an denselben Einträgen tut. Genau die Doppelung, die die Runde 18 beim fünften Baustein vermieden hat, an anderer Stelle.
3. **Das Default-Profil wird selbst in der Sprache der Profile geschrieben und nur nicht ausgeliefert** — es steht als fester Text im Programm, geht durch denselben Prüfschritt wie `readers.toml` und braucht dafür die zwei Kriterien aus Möglichkeit 1.
   - Pros: Ein Weg, eine Maschine, und der Nutzer sieht die Kriterien in der Profildatei nur, wenn er sie sucht.
   - Cons: Ein Profil, das die Datei beschreiben könnte, aber nicht darin steht, ist schwerer zu erklären als eines, das gar nicht beschreibbar ist. Der Nutzerentscheid vom 260827 sagt ausdrücklich, dass das Default-Profil nicht anpassbar ist, und diese Möglichkeit lädt zur Gegenfrage ein.

## Constraints

- Der Nutzer hat am 260827 entschieden: das Default-Profil ist in KRK eingebaut, steht in keinem Block in `readers.toml` und ist weder anpassbar noch abschaltbar.
- Die Zählung läuft flach über eine Ebene, nicht über den Unterbaum (Festlegung A2 der Runde 16, `C3.2`).
- `Baustein` trägt vier Werte, und sieben Stellen halten die Vollständigkeit dieser Aufzählung. Ein fünfter Wert ist von der Runde 18 begründet verworfen.
- Ein Default-Profil hat im heutigen Bau ohnehin keinen Ort: das Werk kennt allein „ein Profil greift" und „keines greift" (`crates/krk-core/src/leseprofil/erkennung.rs:6-8`). Weil der Nutzer es als eingebaut und nicht als Block in `readers.toml` bestimmt hat, entsteht es als Zweig neben `erkennen`. Diese Frage entscheidet daneben, ob dieser Zweig die vorhandene Auswertung ruft oder eine eigene mitbringt.

## Recommendation

Möglichkeit 1. Der Grund ist derselbe, aus dem die Runde 18 die Erweiterung in die `Ortsangabe` gelegt hat statt in einen neuen Baustein: die Frage „welche Einträge zähle ich" ist die Frage, die `zaehlung` schon beantwortet, und zwei Kriterien mehr beantworten sie genauer, während ein zweiter Zähler sie ein zweites Mal beantwortet. Dass der Nutzer das Default-Profil nicht anpassen kann, bleibt davon unberührt: es steht weiter nicht in `readers.toml`, und ob ein eigenes Profil dieselben Kriterien nutzen darf, ist eine andere Frage als die, ob dieses eine anpassbar ist.

---
Answered: circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/history/260827-0622-orchestrator-session.md:47 — Möglichkeit 1: der Baustein `zaehlung` bekommt zwei weitere, freiwillige Kriterien (Typ und Behandlung der versteckten Einträge); das Default-Profil benutzt dieselbe Maschine wie jedes Profil aus `readers.toml`.
