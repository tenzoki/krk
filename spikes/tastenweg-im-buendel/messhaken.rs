// KRK — Messhaken zum Defekt 260926-1735 (neue Tastenkombinationen im Buendel).
//
// WEGWERF-PRUEFCODE, kein Produktcode. Dieses Modul stand fuer die Messung
// voruebergehend am Ende von crates/krk-ui/src/appkit/anwendung.rs, gerufen am
// Ende von `oberflaeche_aufbauen` mit `diag_tasten::starten(self);`, und ist
// wieder entfernt. Umgebungsvariablen: KRK_DIAG_TASTEN=<datei in ~/krkhome>,
// KRK_DIAG_ROH, KRK_DIAG_CG (Ereignis ueber CGEventCreateKeyboardEvent und
// eventWithCGEvent:), KRK_DIAG_RESET (F1, cmd+r, cmd+return vorab),
// KRK_DIAG_WEG (F2, Filter, down, F4), KRK_DIAG_KLICK/_EINZEL/_DX,
// KRK_DIAG_OHNE_FOKUS, KRK_DIAG_EXTERN. Befund im Defektdatensatz.
// DIAG-TASTEN: voruebergehende Messung, wird entfernt.
mod diag_tasten {
    use super::*;
    use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::{NSIndexSet, NSPoint, NSProcessInfo};
    use std::time::Duration;

    fn spaeter(sekunden: f64, zeiger: usize, tun: impl FnOnce(&Anwendungsdelegierter) + Send + 'static) {
        let zeit = dispatch2::DispatchTime::try_from(Duration::from_secs_f64(sekunden)).unwrap();
        let _ = DispatchQueue::main().after(zeit, move || {
            let selbst = unsafe { &*(zeiger as *const Anwendungsdelegierter) };
            tun(selbst);
        });
    }

    const TASTEN: &[(&str, u16, &str, usize)] = &[
        // name, code, zeichen, flaggen-bits (shift 1<<17, ctrl 1<<18, opt 1<<19, cmd 1<<20, numpad 1<<21, fn 1<<23)
        ("shift+cmd+return", 36, "\r", (1 << 17) | (1 << 20)),
        ("cmd+return", 36, "\r", 1 << 20),
        ("opt+cmd+down", 125, "\u{f701}", (1 << 19) | (1 << 20) | (1 << 21) | (1 << 23)),
        ("opt+cmd+up", 126, "\u{f700}", (1 << 19) | (1 << 20) | (1 << 21) | (1 << 23)),
        ("shift+cmd+x", 7, "X", (1 << 17) | (1 << 20)),
        ("shift+cmd+delete", 51, "\u{7f}", (1 << 17) | (1 << 20)),
        ("shift+cmd+p", 35, "P", (1 << 17) | (1 << 20)),
        ("f2", 120, "\u{f705}", 1 << 23),
    ];

    pub(super) fn starten(selbst: &Anwendungsdelegierter) {
        let Some(datei) = std::env::var_os("KRK_DIAG_TASTEN") else { return };
        let roh = std::env::var_os("KRK_DIAG_ROH").is_some();
        let zeiger = Retained::into_raw(selbst.retain()) as usize;
        let v = if std::env::var_os("KRK_DIAG_RESET").is_some() { 4.0 } else { 0.0 };
        if v > 0.0 {
            spaeter(1.0, zeiger, |s| {
                #[allow(deprecated)]
                NSApplication::sharedApplication(s.mtm()).activateIgnoringOtherApps(true);
                s.ivars().fenster.get().unwrap().makeKeyAndOrderFront(None);
                eprintln!("DIAG f1");
                taste(s, 122, "\u{f704}", NSEventModifierFlags(1 << 23));
            });
            spaeter(2.0, zeiger, |s| { eprintln!("DIAG cmd+r"); taste(s, 15, "r", NSEventModifierFlags(1 << 20)); });
            spaeter(3.0, zeiger, |s| { eprintln!("DIAG fertig cmd+return"); taste(s, 36, "\r", NSEventModifierFlags(1 << 20)); });
            spaeter(3.8, zeiger, |s| {
                let b = s.ivars().belegung.borrow();
                eprintln!("DIAG nach reset: eintrag_hinzufuegen={:?} blatt={}", b.funktion("eintrag_hinzufuegen").map(|f| (f.tasten().to_vec(), f.kommando())), s.blatt_steht());
            });
        }
        if std::env::var_os("KRK_DIAG_WEG").is_some() {
            spaeter(1.0, zeiger, |s| {
                #[allow(deprecated)]
                NSApplication::sharedApplication(s.mtm()).activateIgnoringOtherApps(true);
                s.ivars().fenster.get().unwrap().makeKeyAndOrderFront(None);
                eprintln!("DIAG f2");
                taste(s, 120, "\u{f705}", NSEventModifierFlags(1 << 23));
            });
            spaeter(2.0, zeiger, |s| {
                for (c, z) in [(17u16, "t"), (0, "a"), (1, "s"), (40, "k"), (1, "s")] { taste(s, c, z, NSEventModifierFlags(0)); }
            });
            spaeter(2.6, zeiger, |s| { taste(s, 125, "\u{f701}", NSEventModifierFlags((1 << 21) | (1 << 23))); });
            spaeter(3.0, zeiger, |s| { zeigen(s, "vor f4"); taste(s, 118, "\u{f707}", NSEventModifierFlags(1 << 23)); });
            spaeter(4.5, zeiger, |s| zeigen(s, "nach f4"));
            let mut t = 5.0;
            for &(name, code, zeichen, bits) in TASTEN {
                spaeter(t, zeiger, move |s| {
                    if std::env::var_os("KRK_DIAG_KLICK").is_some() { klick(s); }
                });
                spaeter(t + 2.8, zeiger, move |s| {
                    zeigen(s, &format!("vor {name}"));
                    taste(s, code, zeichen, NSEventModifierFlags(bits));
                });
                spaeter(t + 3.6, zeiger, move |s| zeigen(s, &format!("nach {name}")));
                t += 4.2;
            }
            spaeter(t + 0.5, zeiger, |_| std::process::exit(0));
            return;
        }
        let spaeter = move |t: f64, z: usize, f: Box<dyn FnOnce(&Anwendungsdelegierter) + Send>| spaeter(t + v, z, f);
        spaeter(1.5, zeiger, Box::new(move |s: &Anwendungsdelegierter| {
            #[allow(deprecated)]
            NSApplication::sharedApplication(s.mtm()).activateIgnoringOtherApps(true);
            s.ivars().fenster.get().unwrap().makeKeyAndOrderFront(None);
            let pfad = std::env::home_dir().unwrap().join("krkhome").join(&datei);
            eprintln!("DIAG oeffne {}", pfad.display());
            let _ = s.editor_oeffnen_lassen(&pfad, Oeffnungsherkunft::Befehl);
        }));
        spaeter(3.0, zeiger, Box::new(move |s: &Anwendungsdelegierter| {
            let e = s.ivars().editor.get().unwrap();
            if roh != (e.form() == Editorform::Text) {
                let _ = e.ansicht_umschalten();
            }
        }));
        let mut t = 4.0;
        eprintln!("DIAG pid {}", std::process::id());
        for &(name, code, zeichen, bits) in TASTEN {
            let erste = t == 4.0;
            spaeter(t, zeiger, Box::new(move |s: &Anwendungsdelegierter| {
                if erste || std::env::var_os("KRK_DIAG_OHNE_FOKUS").is_none() { fokus_setzen(s, roh); }
                zeigen(s, &format!("vor {name}"));
                if std::env::var_os("KRK_DIAG_EXTERN").is_none() {
                    taste(s, code, zeichen, NSEventModifierFlags(bits));
                }
            }));
            spaeter(t + 0.6, zeiger, Box::new(move |s: &Anwendungsdelegierter| zeigen(s, &format!("nach {name}"))));
            t += 1.2;
        }
        spaeter(t + 0.5, zeiger, Box::new(|_: &Anwendungsdelegierter| std::process::exit(0)));
    }

    fn maus(s: &Anwendungsdelegierter, art: NSEventType, punkt: NSPoint, klicks: isize) {
        let f = s.ivars().fenster.get().unwrap();
        let e = NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            art, punkt, NSEventModifierFlags::empty(), NSProcessInfo::processInfo().systemUptime(),
            f.windowNumber(), None, 0, klicks, 1.0,
        ).unwrap();
        NSApplication::sharedApplication(s.mtm()).postEvent_atStart(&e, false);
    }

    fn klick(s: &Anwendungsdelegierter) {
        let e = s.ivars().editor.get().unwrap();
        let t = e.eintragsansicht().tabelle();
        let spalte: isize = std::env::var("KRK_DIAG_SPALTE").ok().and_then(|z| z.parse().ok()).unwrap_or(1);
        let zeile: isize = std::env::var("KRK_DIAG_ZEILE").ok().and_then(|z| z.parse().ok()).unwrap_or(2);
        let dx: f64 = std::env::var("KRK_DIAG_DX").ok().and_then(|z| z.parse().ok()).unwrap_or(20.0);
        let rect = t.frameOfCellAtColumn_row(spalte, zeile);
        let mitte = NSPoint::new(rect.origin.x + dx, rect.origin.y + rect.size.height / 2.0);
        let p = t.convertPoint_toView(mitte, None::<&objc2_app_kit::NSView>);
        let f = s.ivars().fenster.get().unwrap();
        let treffer = f.contentView().and_then(|c| { let q = unsafe { c.superview() }.unwrap_or(c.clone()); let pp = q.convertPoint_fromView(p, None::<&objc2_app_kit::NSView>); q.hitTest(pp) }).map(|v| v.class().name().to_string_lossy().into_owned());
        eprintln!("DIAG klick zeile {zeile} spalte {spalte} spalten={} bei {p:?} zelle={rect:?} treffer={treffer:?}", t.numberOfColumns());
        if let Some(z) = t.viewAtColumn_row_makeIfNecessary(0, zeile, false) {
            eprintln!("DIAG   zellansicht {:?} rahmen={:?}", z.class().name(), z.frame());
            for u in z.subviews().iter() { eprintln!("DIAG     sub {:?} rahmen={:?}", u.class().name(), u.frame()); }
        }
        maus(s, NSEventType::LeftMouseDown, p, 1);
        maus(s, NSEventType::LeftMouseUp, p, 1);
        if std::env::var_os("KRK_DIAG_EINZEL").is_some() {
            let zeiger = s as *const _ as usize;
            let zeit = dispatch2::DispatchTime::try_from(Duration::from_secs_f64(1.0)).unwrap();
            let _ = DispatchQueue::main().after(zeit, move || {
                let s = unsafe { &*(zeiger as *const Anwendungsdelegierter) };
                maus(s, NSEventType::LeftMouseDown, p, 1);
                maus(s, NSEventType::LeftMouseUp, p, 1);
            });
        }
    }

    fn fokus_setzen(s: &Anwendungsdelegierter, roh: bool) {
        let f = s.ivars().fenster.get().unwrap();
        let e = s.ivars().editor.get().unwrap();
        if let Some(r) = f.firstResponder() {
            if e.bearbeitet_zelle(&r) {
                e.eintragsansicht().bearbeitung_verwerfen(f);
            }
        }
        if roh {
            let _ = f.makeFirstResponder(Some(e.textflaeche()));
        } else {
            let t = e.eintragsansicht().tabelle();
            let _ = f.makeFirstResponder(Some(t));
            if t.selectedRow() < 0 && t.numberOfRows() > 1 {
                t.selectRowIndexes_byExtendingSelection(&NSIndexSet::indexSetWithIndex(1), false);
            }
        }
    }

    fn zeigen(s: &Anwendungsdelegierter, wann: &str) {
        let f = s.ivars().fenster.get().unwrap();
        let e = s.ivars().editor.get().unwrap();
        let r = f.firstResponder();
        let klasse = r.as_ref().map(|e| e.class().name().to_string_lossy().into_owned()).unwrap_or_default();
        let t = e.eintragsansicht().tabelle();
        let lage = s.lage();
        let text = e.textflaeche().string().to_string();
        let kurz: String = text.lines().take(3).collect::<Vec<_>>().join(" | ");
        let zul: Vec<String> = [
            Kommando::EintragHinzufuegen, Kommando::EintragBearbeiten, Kommando::EintragHoch,
            Kommando::EintragRunter, Kommando::EintragLoeschen, Kommando::AufgabeAbhaken, Kommando::PinAendern,
        ].iter().map(|k| format!("{k:?}={}", zulaessigkeit::zulaessig(*k, lage))).collect();
        eprintln!(
            "DIAG {wann}: key={} ersthelfer={klasse} zelle={} form={:?} zeilen={} gewaehlt={} ungesichert={} appkit={} blatt={} fokus={:?} krk={}\nDIAG   text: {kurz}\nDIAG   zul: {}",
            f.isKeyWindow(), r.as_ref().is_some_and(|x| e.bearbeitet_zelle(x)), e.form(), t.numberOfRows(), t.selectedRow(),
            e.hat_ungesicherten_stand(), lage.ersthelfer_gehoert_appkit, lage.blatt_steht, lage.fokus,
            lage.schluesselfenster_gehoert_krk, zul.join(" ")
        );
    }

    #[repr(C)]
    struct CgEreignis { _p: [u8; 0] }
    unsafe impl objc2::encode::RefEncode for CgEreignis {
        const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&objc2::encode::Encoding::Struct("__CGEvent", &[]));
    }

    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        fn CGEventCreateKeyboardEvent(quelle: *const std::ffi::c_void, code: u16, runter: bool) -> *mut std::ffi::c_void;
        fn CGEventSetFlags(ereignis: *mut std::ffi::c_void, flaggen: u64);
        fn CFRelease(obj: *const std::ffi::c_void);
    }

    fn taste(s: &Anwendungsdelegierter, code: u16, zeichen: &str, flaggen: NSEventModifierFlags) {
        if std::env::var_os("KRK_DIAG_CG").is_some() {
            unsafe {
                let cg = CGEventCreateKeyboardEvent(std::ptr::null(), code, true);
                CGEventSetFlags(cg, flaggen.0 as u64);
                let cgp = cg as *mut CgEreignis;
                let ev: Option<Retained<NSEvent>> = msg_send![<NSEvent as objc2::ClassType>::class(), eventWithCGEvent: cgp];
                CFRelease(cg);
                let ev = ev.unwrap();
                eprintln!("DIAG cg-ereignis code={} flags={:#x} chars={:?} ign={:?} fenster={}", ev.keyCode(), ev.modifierFlags().0, ev.characters().map(|c| c.to_string()), ev.charactersIgnoringModifiers().map(|c| c.to_string()), ev.windowNumber());
                NSApplication::sharedApplication(s.mtm()).postEvent_atStart(&ev, false);
            }
            return;
        }
        let f = s.ivars().fenster.get().unwrap();
        let z = NSString::from_str(zeichen);
        let ev = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            NSEventType::KeyDown, NSPoint::ZERO, flaggen,
            NSProcessInfo::processInfo().systemUptime(), f.windowNumber(), None, &z, &z, false, code,
        ).unwrap();
        NSApplication::sharedApplication(s.mtm()).postEvent_atStart(&ev, false);
    }
}
