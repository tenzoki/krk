# Hebt `immer_erreichbar` auch die neue Schlüsselfensterfrage auf, oder wirkt wirklich kein Befehl?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-1037_a_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md` (der Entscheid, aus dem diese Frage folgt); `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md` (Schritt A1); `crates/krk-ui/src/kommandos/zulaessigkeit.rs` (`zulaessig`, `immer_erreichbar`)

---

## Question

Der Nutzer hat am 260813-1055 Möglichkeit 2 gewählt: die Zulässigkeitsregel bekommt die Frage nach dem Schlüsselfenster, und „ist es keines von beidem, wirkt kein Befehl". Die Regel trägt heute aber eine benannte Ausnahmeliste, `immer_erreichbar`, und die hebt die beiden bestehenden Sperren auf — das stehende Blatt und den Ersthelfer, der AppKit gehört. Auf der Liste stehen `beenden` und `fenster_schliessen`. Die Frage, die der Bau jetzt stellt: gilt die Ausnahmeliste auch gegen die neue vierte Bedingung, oder steht die vierte über ihr?

Die Frage muss vor dem Bau beantwortet sein, weil sie eine Zeile in `zulaessig` und die Erwartung der Tafel entscheidet, und weil die beiden Antworten sich am Verhalten unterscheiden: mit dem Über-Dialog im Vordergrund beendet Cmd+Q die Anwendung oder es tut nichts.

## Options

1. **Die Ausnahmeliste hebt auch die vierte Bedingung auf.** In `zulaessig` steht die neue Frage neben dem Blattstand und dem Ersthelferbefund, innerhalb desselben `durchgelassen`-Ausdrucks.
   - Pro: kein Verlust gegenüber heute, und das ist eine Randbedingung dieses Spec. Der Freigabedialog der Runde 6 steht heute schon vor dem Fenster, und Cmd+Q beendet KRK dabei. Möglichkeit 2 nähme diesen Weg weg, ohne dass jemand ihn genannt hätte.
   - Pro: die Ausnahmeliste behält eine Bedeutung, die sich in einem Satz sagen lässt: sie hebt jede Sperre auf, die nach der Lage fragt, und keine, die nach dem Wirkungsbereich fragt.
   - Contra: der Wortlaut der gewählten Möglichkeit sagt „wirkt kein Befehl", und zwei Befehle wirken doch.
2. **Die vierte Bedingung steht über der Ausnahmeliste.** Ist das Schlüsselfenster fremd, wirkt buchstäblich kein Befehl, auch `beenden` nicht.
   - Pro: genau der Wortlaut, den der Nutzer gewählt hat, ohne Auslegung.
   - Contra: mit dem Über-Dialog oder dem Freigabewähler im Vordergrund ist Cmd+Q tot und der Menüeintrag ausgegraut. Auf dem Mac beendet Cmd+Q eine Anwendung auch dann. Für den Freigabewähler ist das ein Verlust gegenüber heute.
   - Contra: die Ausnahmeliste bekäme eine dritte Lesart. Heute hebt sie zwei von drei Bestandteilen auf und den dritten nicht; danach zwei von vier, und welche zwei, stünde nur im Code.

## Constraints

- Die Regel steht an genau einer Stelle und bleibt dort. Eine zweite Fassung daneben ist ausgeschlossen (Randbedingung des Spec).
- `beenden` und `fenster_schliessen` tragen beide `Wirkungsbereich::Ueberall`; der dritte Bestandteil lässt sie deshalb in jedem Fokus durch. Der Unterschied zwischen den beiden Möglichkeiten fällt allein an der vierten Bedingung an.
- Cmd+W liegt auf `tab_schliessen` und steht **nicht** auf der Ausnahmeliste. Beide Möglichkeiten sperren es vor einem fremden Schlüsselfenster, und damit ist der Fall gesperrt, den der Defekt zum Freigabedialog namentlich nennt.
- Shift+Cmd+W liegt auf `fenster_schliessen`. Unter Möglichkeit 1 schlösse es mit dem Über-Dialog im Vordergrund das Hauptfenster dahinter. Das ist überraschend, aber es ist heute schon so, wenn der Freigabewähler steht.

## Recommendation

Möglichkeit 1. Der Grund ist der Preis, den nur sie nicht zahlt: Möglichkeit 2 nimmt Cmd+Q vor einem Systemfenster weg, das KRK nicht gebaut hat und dessen Ersthelfer KRK nicht bestimmt. „Kein Verlust gegenüber heute" ist eine ausgeschriebene Randbedingung dieses Spec, und der Freigabewähler der Runde 6 ist der Fall, an dem sie hier greift.

Der Wortlaut „wirkt kein Befehl" ist dabei nicht gebrochen, sondern gelesen wie die beiden Sätze daneben: der Modulkopf sagt heute ebenso „es steht kein Blatt, oder der Befehl ist während eines Blattes erlaubt", und die Ausnahmeliste ist die benannte Öffnung, die zu jedem dieser Sätze gehört. Der Plan fährt bis zur Antwort auf dieser Empfehlung.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md, Abschnitt "Die fünfte Frage: die Ausnahmeliste" — Antwort: Möglichkeit 1, die Ausnahmeliste hebt auch die vierte Bedingung auf; beenden und fenster_schliessen kommen durch.

---
Implemented: c3ada4d, erweitert in ed0388e — Möglichkeit 1 ist gebaut. `immer_erreichbar(kommando)` steht als erster Operand des `durchgelassen`-Ausdrucks (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:172-180`) und hebt damit auch die vierte Bedingung auf; der Modulkopf sagt es ausdrücklich (`:77-79`): die Liste hebt die Bestandteile (1), (2) und (4) auf, den dritten nicht. `beenden` und `fenster_schliessen` kommen vor einem fremden Schlüsselfenster durch, gehalten von `die_ausnahmeliste_kommt_durch_blatt_und_textfeld` (`:562`), und `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf` (`:592`) hält die Gegenrichtung.

**Die Antwort hat einen Fall nicht mitgeprüft, und er ist in Turn 2 nachgetragen worden.** Die Begründung „kein Verlust gegenüber heute" ist allein an den beiden Befehlen geprüft worden, die schon auf der Liste standen. `Kommando::FensterEinblenden` (Cmd+N) ist der Rückweg, nachdem das Fenster geschlossen wurde, und `NSApplication::keyWindow` liefert in dieser Lage nichts — die vierte Bedingung hat ihn damit gerade dort abgewiesen, wo er gebraucht wird. Behoben in `ed0388e` über denselben Weg, den dieser Datensatz der Liste zuweist; die Liste führt seither drei Befehle (`zulaessigkeit.rs:198-201`), Probe `ohne_schluesselfenster_kommt_fenster_einblenden_durch` (`:527`). Datensatz: `issues/260813-1258_c_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`, geschlossen. Abgeglichen am 260813-1345.
