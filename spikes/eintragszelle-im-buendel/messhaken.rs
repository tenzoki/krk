// KRK — Messhaken zum Defekt 260926-1400 (Eintragszelle im laufenden Buendel).
//
// WEGWERF-PRUEFCODE, kein Produktcode. Dieses Modul stand fuer die Messung
// voruebergehend am Ende von crates/krk-ui/src/appkit/anwendung.rs, gerufen am
// Ende von `oberflaeche_aufbauen` mit `diag_zelle::starten(self);`, und ist
// wieder entfernt. Es faehrt KRK selbst ueber `postEvent:atStart:` in die eigene
// Ereignisschlange. Aufruf siehe messungen/260926-1401-eintragszelle-im-laufenden-buendel.txt.
// DIAG-ZELLE: voruebergehende Messung, wird entfernt.
mod diag_zelle {
    use super::*;
    use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventType, NSTextView, NSView};
    use objc2_foundation::{NSPoint, NSProcessInfo};
    use std::time::Duration;

    fn spaeter(sekunden: f64, zeiger: usize, tun: fn(&Anwendungsdelegierter)) {
        let zeit = dispatch2::DispatchTime::try_from(Duration::from_secs_f64(sekunden)).unwrap();
        let _ = DispatchQueue::main().after(zeit, move || {
            // SAFETY: Messcode; der Delegierte lebt fuer die Dauer des Programms.
            let selbst = unsafe { &*(zeiger as *const Anwendungsdelegierter) };
            tun(selbst);
        });
    }

    pub(super) fn starten(selbst: &Anwendungsdelegierter) {
        if std::env::var_os("KRK_DIAG_ZELLE").is_none() {
            return;
        }
        let zeiger = Retained::into_raw(selbst.retain()) as usize;
        let v = if std::env::var("KRK_DIAG_ZELLE").as_deref() == Ok("2") { 6.0 } else { 0.0 };
        if v > 0.0 {
            spaeter(1.5, zeiger, |s| {
                #[allow(deprecated)] NSApplication::sharedApplication(s.mtm()).activateIgnoringOtherApps(true);
                s.ivars().fenster.get().unwrap().makeKeyAndOrderFront(None);
                let pfad = std::env::home_dir().unwrap().join("krkhome/secrets.txt");
                let _ = s.editor_oeffnen_lassen(&pfad, Oeffnungsherkunft::Befehl);
            });
            spaeter(2.5, zeiger, |s| {
                lage_zeigen(s, "pinblatt");
                for (c, z) in [(18u16, "1"), (19, "2"), (20, "3"), (21, "4"), (48, "\t"), (18, "1"), (19, "2"), (20, "3"), (21, "4"), (36, "\r")] {
                    taste(s, c, z);
                }
            });
            spaeter(4.0, zeiger, |s| lage_zeigen(s, "nach-pin"));
        }
        let spaeter = |t: f64, z: usize, f: fn(&Anwendungsdelegierter)| spaeter(t + v, z, f);
        if std::env::var_os("KRK_DIAG_ROH").is_some() {
            spaeter(1.5, zeiger, schritt_oeffnen);
            spaeter(3.0, zeiger, |s| {
                lage_zeigen(s, "roh-vorher");
                let e = s.ivars().editor.get().unwrap();
                let _ = e.ansicht_umschalten();
                let f = s.ivars().fenster.get().unwrap();
                let _ = f.makeFirstResponder(Some(e.textflaeche()));
                lage_zeigen(s, "roh-umgeschaltet");
                taste(s, 7, "x");
                taste_mit(s, 123, "\u{f702}", NSEventModifierFlags::NumericPad | NSEventModifierFlags::Function);
                taste(s, 16, "y");
            });
            spaeter(4.5, zeiger, |s| {
                lage_zeigen(s, "roh-getippt");
                let e = s.ivars().editor.get().unwrap();
                eprintln!("DIAG roh stand ungesichert={} text={:?}", e.hat_ungesicherten_stand(), e.textflaeche().string().to_string().chars().rev().take(20).collect::<String>());
            });
            spaeter(5.5, zeiger, |_| std::process::exit(0));
            return;
        }
        spaeter(1.5, zeiger, schritt_oeffnen);
        spaeter(3.0, zeiger, schritt_format);
        spaeter(4.0, zeiger, schritt_beginnen);
        spaeter(5.0, zeiger, schritt_nach_tippen);
        spaeter(6.0, zeiger, schritt_doppelklick);
        spaeter(8.0, zeiger, schritt_nach_doppelklick);
        spaeter(9.0, zeiger, |s| { lage_zeigen(s, "vor-tab"); taste(s, 48, "\t"); });
        spaeter(9.5, zeiger, schritt_stand);
        spaeter(10.0, zeiger, |s| { let e = s.ivars().editor.get().unwrap(); eprintln!("DIAG sichern -> {:?}", e.sichern()); });
        spaeter(10.5, zeiger, |s| {
            let f = s.ivars().fenster.get().unwrap();
            let _ = f.makeFirstResponder(Some(s.ivars().editor.get().unwrap().eintragsansicht().tabelle()));
            let t = s.ivars().editor.get().unwrap().eintragsansicht().tabelle();
            let feld = t.viewAtColumn_row_makeIfNecessary(1, 0, false).and_then(|z| z.downcast::<objc2_app_kit::NSTableCellView>().ok()).and_then(|z| unsafe { z.textField() });
            let fe = unsafe { f.fieldEditor_forObject(true, feld.as_deref().map(|x| AsRef::<AnyObject>::as_ref(x))) };
            eprintln!("DIAG feldeditor nach ende: {:?} rahmen={:?} ersthelfer={:?}", fe.as_ref().map(|e| e.class().name().to_string_lossy().into_owned()), fe.map(|e| e.frame()), f.firstResponder().map(|e| e.class().name().to_string_lossy().into_owned()));
        });
        spaeter(11.0, zeiger, |_| std::process::exit(0));
    }

    fn lage_zeigen(selbst: &Anwendungsdelegierter, wann: &str) {
        let fenster = selbst.ivars().fenster.get().unwrap();
        if let Some(ziel) = std::env::var_os("KRK_DIAG_BILD") {
            let datei = std::path::Path::new(&ziel).join(format!("{wann}.png"));
            if let Some(inhalt) = fenster.contentView() {
                let rect = inhalt.bounds();
                if let Some(rep) = inhalt.bitmapImageRepForCachingDisplayInRect(rect) {
                    inhalt.cacheDisplayInRect_toBitmapImageRep(rect, &rep);
                    let daten = unsafe {
                        rep.representationUsingType_properties(
                            objc2_app_kit::NSBitmapImageFileType::PNG,
                            &objc2_foundation::NSDictionary::new(),
                        )
                    };
                    if let Some(daten) = daten {
                        let _ = std::fs::write(&datei, daten.to_vec());
                    }
                }
            }
        }
        let editor = selbst.ivars().editor.get().unwrap();
        let ersthelfer = fenster.firstResponder();
        let klasse = ersthelfer
            .as_ref()
            .map(|e| e.class().name().to_string_lossy().into_owned())
            .unwrap_or_default();
        let tabelle = editor.eintragsansicht().tabelle();
        eprintln!(
            "DIAG {wann}: key={} aktiv={} ersthelfer={klasse} form={:?} zeilen={} rolle_versteckt={} zelle_laeuft={}",
            fenster.isKeyWindow(),
            NSApplication::sharedApplication(selbst.mtm()).isActive(),
            editor.form(),
            tabelle.numberOfRows(),
            editor.eintragsansicht().rolle().isHidden(),
            ersthelfer.as_ref().is_some_and(|e| editor.bearbeitet_zelle(e)),
        );
        if let Some(e) = ersthelfer.as_ref().and_then(|e| e.downcast_ref::<NSTextView>()) {
            let delegat = e
                .delegate()
                .map(|d| AsRef::<AnyObject>::as_ref(&*d).class().name().to_string_lossy().into_owned());
            eprintln!(
                "DIAG {wann}:   textview feldeditor={} editierbar={} waehlbar={} text={:?} delegat={delegat:?} rahmen={:?} superview={:?} versteckt={}",
                e.isFieldEditor(),
                e.isEditable(),
                e.isSelectable(),
                e.string().to_string(),
                e.frame(),
                unsafe { e.superview() }.map(|s| s.class().name().to_string_lossy().into_owned()),
                e.isHiddenOrHasHiddenAncestor(),
            );
        }
        let lage = selbst.lage();
        eprintln!(
            "DIAG {wann}:   lage appkit={} blatt={} fokus={:?} fenster_krk={}",
            lage.ersthelfer_gehoert_appkit, lage.blatt_steht, lage.fokus, lage.schluesselfenster_gehoert_krk
        );
    }

    fn schritt_oeffnen(selbst: &Anwendungsdelegierter) {
        let mtm = selbst.mtm();
        #[allow(deprecated)] NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
        let fenster = selbst.ivars().fenster.get().unwrap();
        fenster.makeKeyAndOrderFront(None);
        let pfad = std::env::home_dir().unwrap().join("krkhome/notes.txt");
        if std::env::var_os("KRK_DIAG_SITZUNG").is_some() { eprintln!("DIAG kein oeffnen, Sitzung"); return; }
        eprintln!("DIAG oeffne {}", pfad.display());
        let _ = selbst.editor_oeffnen_lassen(&pfad, Oeffnungsherkunft::Befehl);
    }

    fn schritt_format(selbst: &Anwendungsdelegierter) {
        lage_zeigen(selbst, "nach-oeffnen");
        let editor = selbst.ivars().editor.get().unwrap();
        if editor.form() == Editorform::Text {
            let _ = editor.ansicht_umschalten();
            lage_zeigen(selbst, "nach-umschalten");
        }
    }

    fn taste(selbst: &Anwendungsdelegierter, code: u16, zeichen: &str) {
        taste_mit(selbst, code, zeichen, NSEventModifierFlags::empty());
    }

    fn taste_mit(selbst: &Anwendungsdelegierter, code: u16, zeichen: &str, flaggen: NSEventModifierFlags) {
        let fenster = selbst.ivars().fenster.get().unwrap();
        let z = NSString::from_str(zeichen);
        let e = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            NSEventType::KeyDown, NSPoint::ZERO, flaggen,
            NSProcessInfo::processInfo().systemUptime(), fenster.windowNumber(), None, &z, &z, false, code,
        ).unwrap();
        NSApplication::sharedApplication(selbst.mtm()).postEvent_atStart(&e, false);
    }

    fn schritt_beginnen(selbst: &Anwendungsdelegierter) {
        let editor = selbst.ivars().editor.get().unwrap();
        let ok = editor.eintragsansicht().zelle_beginnen(super::super::eintragsansicht::Zelle {
            zeile: 0,
            spalte: 1,
        });
        eprintln!("DIAG zelle_beginnen -> {ok}");
        lage_zeigen(selbst, "nach-beginnen");
        taste(selbst, 7, "x");
        taste(selbst, 16, "y");
    }

    fn schritt_nach_tippen(selbst: &Anwendungsdelegierter) {
        lage_zeigen(selbst, "nach-tippen");
        // Zelle verwerfen, damit der Doppelklick frisch beginnt.
        let fenster = selbst.ivars().fenster.get().unwrap();
        let editor = selbst.ivars().editor.get().unwrap();
        editor.eintragsansicht().bearbeitung_verwerfen(fenster);
        lage_zeigen(selbst, "nach-verwerfen");
    }

    fn maus(selbst: &Anwendungsdelegierter, art: NSEventType, punkt: NSPoint, klicks: isize) {
        let fenster = selbst.ivars().fenster.get().unwrap();
        let e = NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            art, punkt, NSEventModifierFlags::empty(), NSProcessInfo::processInfo().systemUptime(),
            fenster.windowNumber(), None, 0, klicks, 1.0,
        ).unwrap();
        NSApplication::sharedApplication(selbst.mtm()).postEvent_atStart(&e, false);
    }

    fn schritt_doppelklick(selbst: &Anwendungsdelegierter) {
        let editor = selbst.ivars().editor.get().unwrap();
        let tabelle = editor.eintragsansicht().tabelle();
        let zeile: isize = std::env::var("KRK_DIAG_ZEILE").ok().and_then(|z| z.parse().ok()).unwrap_or(0);
        let einzel = std::env::var_os("KRK_DIAG_EINZEL").is_some();
        eprintln!("DIAG vor klick: gewaehlt={} zeile={zeile} einzel={einzel}", tabelle.selectedRow());
        let spalte: isize = std::env::var("KRK_DIAG_SPALTE").ok().and_then(|z| z.parse().ok()).unwrap_or(1); let rect = tabelle.frameOfCellAtColumn_row(spalte, zeile);
        let mitte = NSPoint::new(rect.origin.x + 20.0, rect.origin.y + rect.size.height / 2.0);
        let imfenster = tabelle.convertPoint_toView(mitte, None::<&NSView>);
        eprintln!("DIAG doppelklick bei {imfenster:?} (zelle {rect:?})");
        if einzel {
            maus(selbst, NSEventType::LeftMouseDown, imfenster, 1);
            maus(selbst, NSEventType::LeftMouseUp, imfenster, 1);
            let p = imfenster;
            let z = selbst as *const _ as usize;
            let zeit = dispatch2::DispatchTime::try_from(Duration::from_secs_f64(1.0)).unwrap();
            let _ = DispatchQueue::main().after(zeit, move || {
                let s = unsafe { &*(z as *const Anwendungsdelegierter) };
                maus(s, NSEventType::LeftMouseDown, p, 1);
                maus(s, NSEventType::LeftMouseUp, p, 1);
            });
        } else {
            for klicks in [1, 2] {
                maus(selbst, NSEventType::LeftMouseDown, imfenster, klicks);
                maus(selbst, NSEventType::LeftMouseUp, imfenster, klicks);
            }
        }
    }

    fn schritt_stand(selbst: &Anwendungsdelegierter) {
        lage_zeigen(selbst, "nach-tab");
        let e = selbst.ivars().editor.get().unwrap();
        eprintln!("DIAG stand ungesichert={} text={:?}", e.hat_ungesicherten_stand(), e.textflaeche().string().to_string().chars().take(120).collect::<String>());
    }

    fn schritt_nach_doppelklick(selbst: &Anwendungsdelegierter) {
        lage_zeigen(selbst, "nach-doppelklick");
        taste(selbst, 7, "x");
        if std::env::var_os("KRK_DIAG_TASTEN").is_some() {
            taste(selbst, 49, " ");
            taste(selbst, 16, "y");
            taste(selbst, 49, " ");
            taste(selbst, 6, "z");
            taste(selbst, 51, "\u{7f}");
            taste_mit(selbst, 123, "\u{f702}", NSEventModifierFlags::NumericPad | NSEventModifierFlags::Function);
            taste(selbst, 0, "a");
        }
        spaeter(0.5, selbst as *const _ as usize, |s| lage_zeigen(s, "nach-doppelklick-tippen"));
    }
}
