# KRK — Bauwerkzeug fuer die Kommandozeile
#
# Kein zweites Bauwerkzeug neben cargo, sondern eine Huelle darum. Jedes Ziel
# hier ruft genau das Kommando, das CLAUDE.md und die Abnahmekriterien des
# Plans ohnehin nennen; wer lieber cargo tippt, verliert nichts.
#
# Der Zweck sind die zwei Zeilen darunter: cargo liegt auf diesem Geraet nicht
# auf dem Standard-PATH, und jeder Aufruf braucht sonst ein vorangestelltes
# export.
#
# CARGO steht absolut und nicht als blosser Name, und das ist kein Zierat: GNU
# make 3.81, die Fassung, die macOS mitbringt, fuehrt ein Rezept aus genau
# einem Wort ohne Sonderzeichen direkt aus und schlaegt das Programm dabei in
# seinem eigenen PATH nach, nicht in dem, den die Zeile darunter exportiert.
# `cargo build --workspace` scheiterte damit an "No such file or directory",
# waehrend `echo $$PATH` im selben Makefile den richtigen Pfad zeigte. Der
# Export bleibt trotzdem stehen: er gilt fuer alles, was die Rezepte ihrerseits
# aufrufen, etwa cargo fuer rustc.
CARGO := $(HOME)/.cargo/bin/cargo
export PATH := $(HOME)/.cargo/bin:$(PATH)

BUENDEL := target/KRK.app
BINAER  := $(BUENDEL)/Contents/MacOS/krk

.DEFAULT_GOAL := help

# ── Alltag ───────────────────────────────────────────────────────────────────

.PHONY: help
help: ## Diese Uebersicht
	@grep -hE '^[a-z][a-zA-Z0-9_-]*:.*?## ' $(MAKEFILE_LIST) \
	  | sort \
	  | awk -F':.*?## ' '{ printf "  \033[1m%-14s\033[0m %s\n", $$1, $$2 }'

.PHONY: build
build: ## Uebersetzen, alle vier Kisten
	$(CARGO) build --workspace

.PHONY: test
test: ## Alle Pruefungen
	$(CARGO) test --workspace

.PHONY: lint
lint: ## clippy ueber alle Ziele, Warnungen sind Fehler
	$(CARGO) clippy --workspace --all-targets -- -D warnings

.PHONY: fmt
fmt: ## Quelltext formatieren
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Formatierung pruefen, ohne zu aendern
	$(CARGO) fmt --all --check

.PHONY: check
check: build test fmt-check lint ## Die vier Abnahmekommandos, in der Reihenfolge der Schritte
	@echo "alle vier gruen"

# Die Schritte stehen als $(MAKE)-Aufrufe und nicht als Voraussetzungen, und
# das ist der Punkt: make darf Voraussetzungen in beliebiger Reihenfolge
# abarbeiten und bei -j sogar nebenlaeufig. `frisch: clean check bundle` haette
# das Aufraeumen mitten in den Bau legen duerfen. Untermakes laufen
# nacheinander, und bricht eines ab, endet die Kette dort.
.PHONY: frisch
frisch: ## Von Grund auf: aufraeumen, uebersetzen, pruefen, signiertes Buendel
	$(MAKE) clean
	$(MAKE) check
	$(MAKE) bundle
	$(MAKE) signatur
	@echo
	@echo "Frisch gebaut und signiert: $(BUENDEL)"

# ── Buendel ──────────────────────────────────────────────────────────────────

.PHONY: bundle
bundle: ## target/KRK.app bauen und signieren
	$(CARGO) xtask bundle

.PHONY: run
run: bundle ## Buendel bauen und starten
	open $(BUENDEL)

.PHONY: run-terminal
run-terminal: bundle ## Buendel starten und die Standardausgabe hier behalten
	@echo "Ueber open gestartet haette KRK keine Standardausgabe; deshalb direkt."
	$(BINAER)

.PHONY: tasten
tasten: bundle ## Tastencodes protokollieren, Beenden mit Cmd+Q
	$(BINAER) --tasten-protokoll

.PHONY: menue
menue: bundle ## Das gebaute Hauptmenue mit allen Kuerzeln ausgeben
	$(BINAER) --menue-protokoll

.PHONY: signatur
signatur: ## Signatur des gebauten Buendels pruefen
	codesign --verify --deep --strict $(BUENDEL)
	codesign -dvv $(BUENDEL) 2>&1 | grep -E 'Authority|flags'

# ── Messen ───────────────────────────────────────────────────────────────────

# Die Pruefordner liegen unter /tmp und nicht im Quellbaum: sie sind gross,
# reproduzierbar aus ihrem Startwert und gehoeren keinem Commit an.
ORDNER_A    := /tmp/krk-pruefordner-a
ORDNER_B    := /tmp/krk-pruefordner-b
ORDNER_100K := /tmp/krk-pruefordner-gross
KOPIERZIEL  := /tmp/krk-kopierziel

.PHONY: fixture
fixture: ## Die drei Pruefordner anlegen: A und B mit 10.000, einer mit 100.000
	$(CARGO) run -p krk-bench --release -- fixture --eintraege 10000  --seed 1 --out $(ORDNER_A)
	$(CARGO) run -p krk-bench --release -- fixture --eintraege 10000  --seed 2 --out $(ORDNER_B)
	$(CARGO) run -p krk-bench --release -- fixture --eintraege 100000 --seed 3 --out $(ORDNER_100K)

.PHONY: messen
messen: ## Kopflos lesen und sortieren messen: make messen ORDNER=/pfad
	$(CARGO) run -p krk-bench --release -- messen --kopflos --ordner $(or $(ORDNER),$(ORDNER_A))

.PHONY: alle
alle: ## Alle zehn Zusagen messen (S21): make alle RUNDEN=1
	$(CARGO) xtask messen --alle \
	  --ordner-a $(ORDNER_A) \
	  --ordner-b $(ORDNER_B) \
	  --ordner100k $(ORDNER_100K) \
	  --kopierziel $(KOPIERZIEL) \
	  --runden $(or $(RUNDEN),1)

.PHONY: durchstich
durchstich: bundle ## Die fuenf Zusagen am Buendel messen: make durchstich RUNDEN=5
	$(CARGO) run -p krk-bench --release -- durchstich \
	  --buendel $(BINAER) \
	  --ordner-a $(ORDNER_A) \
	  --ordner100k $(ORDNER_100K) \
	  --runden $(or $(RUNDEN),1)

# ── Aufraeumen ───────────────────────────────────────────────────────────────

.PHONY: clean
clean: ## Bauergebnisse loeschen
	$(CARGO) clean
