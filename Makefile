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

# ── Ausliefern ───────────────────────────────────────────────────────────────

# Die Beglaubigung braucht ein hinterlegtes notarytool-Schluesselbundprofil;
# xtask liest seinen Namen aus KRK_NOTARY_PROFILE. Der Name steht hier als
# Vorgabe und nicht fest im Rezept: `make release NOTARPROFIL=anderes` waehlt
# ein anderes, und ein bereits gesetztes KRK_NOTARY_PROFILE gilt weiter.
#
# KRK_SIGN_IDENTITY steht hier bewusst nicht. Es war bis zum 260813 die
# Umgehung eines Defekts der Identitaetssuche
# (`shared/issues/260812-2357_*_die-identitaetssuche-zaehlt-jede-identitaet-doppelt…`);
# der ist behoben, und xtask findet die Developer-ID selbst. Eine Umgehung im
# Makefile waere ein Dauerzustand geworden.
NOTARPROFIL := $(or $(KRK_NOTARY_PROFILE),krk-notar)

# Der ganze Auslieferungsweg, ein Ziel und ein Wert. ./release.sh reicht hierher
# weiter, und dieses Ziel reicht an xtask weiter; die Logik steht dort.
#
# Die zwei Schritte stehen als eigene Zeilen und nicht als Voraussetzungen, aus
# demselben Grund wie bei `frisch`: make darf Voraussetzungen in beliebiger
# Reihenfolge abarbeiten, hier aber muss der Versionsschritt vor dem
# Auslieferungsschritt liegen. Bricht der erste ab, endet die Kette dort.
#
# **Und sie stehen als zwei Prozesse, weil es zwei sein muessen.** xtask liest
# die Versionszahl beim Uebersetzen ueber env!("CARGO_PKG_VERSION"). Erst wenn
# der erste Prozess geendet hat, uebersetzt cargo das Werkzeug mit der neuen
# Zahl neu; Station 1 des zweiten vergleicht dann die neu eingebackene Zahl mit
# dem Tag und faellt durch, wenn ein altes Werkzeug stehengeblieben ist.
#
# **Auch dieses Ziel wirkt ueber das Geraet hinaus**, und es schreibt zusaetzlich
# in den Arbeitsbaum. Es ist ueberdies das Ziel, zu dem `./release.sh` fuehrt,
# also das, was der Nutzer wirklich faehrt; die ##-Zeile darunter nennt das
# Schieben deshalb ebenso wie die von `release`. Bis zum 260821 tat sie es nicht
# (Durchsicht 260821-1432, C1).
.PHONY: ausliefern
ausliefern: ## Version setzen, eintragen, taggen, ausliefern und HEAD und Tag zu origin schieben: make ausliefern VERSION=0.2.0
	@test -n "$(VERSION)" || { echo "make ausliefern braucht eine Zahl: make ausliefern VERSION=0.2.0"; exit 2; }
	$(CARGO) xtask version $(VERSION)
	$(MAKE) release

# **Dieses Ziel wirkt ueber das Geraet hinaus.** Die achte Station schiebt HEAD
# und den Tag zu origin und legt eine oeffentliche Releaseseite an; das ist die
# einzige Wirkung der ganzen Kette, die sich nicht zuruecknehmen laesst. Die
# ##-Zeile darunter nennt sie deshalb, denn sie ist es, die `make help` vor dem
# Tippen ausgibt. Bis zum 260821 tat sie es nicht (Durchsicht 260821-1346, F1).
.PHONY: release
release: ## Bauen, signieren, beglaubigen, HEAD und Tag zu origin schieben, veroeffentlichen
	KRK_NOTARY_PROFILE=$(NOTARPROFIL) $(CARGO) xtask release

# Der Nur-Beglaubigungsweg: fuer den Lauf, der erst an der siebten Station
# gescheitert ist, waehrend das fertige Buendel unter $(BUENDEL) liegt und
# allein das Ticket fehlt. `make release` faengt in dieser Lage von vorn an und
# braeche zuvor an Station 1 ab, weil der Tag nach dem Lauf nicht mehr allein
# auf HEAD steht.
#
# Die Zahl ist kein Zierat: xtask haelt sie gegen die Info.plist des gebauten
# Buendels, damit nicht ein $(BUENDEL) von vorgestern bei Apple landet. Das
# Ziel baut deshalb auch nichts und haengt an keiner Voraussetzung — ein
# `bundle` davor ueberschriebe genau das Buendel, um das es geht.
.PHONY: beglaubigen
beglaubigen: ## Ein fertiges Buendel allein beglaubigen: make beglaubigen VERSION=0.2.0
	@test -n "$(VERSION)" || { echo "make beglaubigen braucht eine Zahl: make beglaubigen VERSION=0.2.0"; exit 2; }
	KRK_NOTARY_PROFILE=$(NOTARPROFIL) $(CARGO) xtask beglaubigen $(VERSION)

# Die zwei Beglaubigungspruefungen duerfen das Ziel nicht scheitern lassen, und
# der Grund ist kein Nachlassen: ein mit `make bundle` gebautes Buendel traegt
# eine Entwicklungsidentitaet und **muss** bei spctl durchfallen. Das ist der
# richtige Befund und kein Fehler. Ein Ziel, das danach mit Exit 1 abbricht,
# waere nach jedem Entwicklungsbau unbrauchbar — und `frisch` ruft es. Beide
# Auskuenfte stehen deshalb im Klartext da, und die Legende darueber sagt, wie
# der Leser die zwei Faelle auseinanderhaelt.
#
# Die erste Pruefung, codesign --verify, bleibt hart: sie gilt fuer jedes
# Buendel gleich, ob entwickelt oder ausgeliefert.
.PHONY: signatur
signatur: ## Signatur und Beglaubigung des gebauten Buendels pruefen
	codesign --verify --deep --strict $(BUENDEL)
	codesign -dvv $(BUENDEL) 2>&1 | grep -E 'Authority|flags'
	@echo
	@echo "Beglaubigung — zwei erwartbare Befunde, beide richtig:"
	@echo "  rejected / origin=Apple Development       aus 'make bundle', nur lokal"
	@echo "  accepted / source=Notarized Developer ID  aus 'make release', ausliefbar"
	@echo
	xcrun stapler validate $(BUENDEL) || true
	spctl -a -vvv -t exec $(BUENDEL) || true

# ── Messen ───────────────────────────────────────────────────────────────────

# Der Messplatz liegt ausserhalb des Quellbaums: die Pruefordner sind gross,
# reproduzierbar aus ihrem Startwert und gehoeren keinem Commit an.
#
# **Und er liegt nicht mehr unter /tmp.** In der Nacht zum 260806 hat eine
# Systembereinigung dort saemtliche Unterordner aller vier Bestaende geloescht
# und die Dateien stehen lassen: leer plus alter Aenderungszeitstempel, und der
# Pruefordner-Erzeuger datiert seine Zeitstempel absichtlich breit zurueck,
# damit die Sortierung nach Datum etwas zu sortieren hat. Ein frisch neu
# erzeugter Bestand verlor seine Unterordner binnen Minuten erneut. Eine
# Messreihe laeuft damit stillschweigend auf einem beschnittenen Bestand,
# sobald eine naechtliche Bereinigung dazwischenliegt. Unter
# ~/Library/Caches/ hat seit dem 260806 kein Bestand mehr Eintraege verloren;
# derselbe APFS-Datentraeger wie zuvor, also misst L8 dort weiter dasselbe.
# Der Befund steht in
# `issues/260806-0014_*_pruefordner-unter-tmp-verlieren-leere-unterordner-an-die-systembereinigung.md`.
MESSPLATZ   := $(HOME)/Library/Caches/krk-messplatz
ORDNER_A    := $(MESSPLATZ)/pruefordner-a
ORDNER_B    := $(MESSPLATZ)/pruefordner-b
ORDNER_100K := $(MESSPLATZ)/pruefordner-gross
KOPIERZIEL  := $(MESSPLATZ)/kopierziel

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
