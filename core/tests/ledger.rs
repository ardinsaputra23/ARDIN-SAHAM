use ardin_core::*;
use rust_decimal::Decimal;
fn event(id: i64, kind: &str, shares: i64, price: &str, fee: &str, amount: &str) -> Event {
    Event {
        id,
        date: format!("2026-01-{:02}", id),
        kind: kind.into(),
        stock: if ["DEPOSIT", "WITHDRAW"].contains(&kind) {
            "".into()
        } else {
            "NCKL".into()
        },
        shares,
        price: price.into(),
        fee: fee.into(),
        amount: amount.into(),
        journal: "Alasan keputusan".into(),
    }
}
fn deposit() -> Event {
    event(1, "DEPOSIT", 0, "0", "0", "100000000")
}
fn buys() -> Vec<Event> {
    vec![
        deposit(),
        event(2, "BUY", 10000, "1000", "0", "0"),
        event(3, "BUY", 10000, "900", "0", "0"),
        event(4, "BUY", 20000, "800", "0", "0"),
    ]
}
#[test]
fn weighted_average() {
    let s = replay(&buys()).unwrap();
    assert_eq!(s.positions["NCKL"].shares, 40000);
    assert_eq!(s.positions["NCKL"].cost, Decimal::from(35000000));
    assert_eq!(
        s.positions["NCKL"].cost / Decimal::from(40000),
        Decimal::from(875)
    );
    assert_eq!(s.classifications[&3], "Average Down");
}
#[test]
fn partial_sale_and_addition() {
    let mut e = buys();
    e.push(event(5, "SELL", 10000, "1000", "1000", "0"));
    let s = replay(&e).unwrap();
    assert_eq!(s.realized, Decimal::from(1249000));
    assert_eq!(s.positions["NCKL"].cost, Decimal::from(26250000));
    e.push(event(6, "BUY", 10000, "1100", "2000", "0"));
    let s = replay(&e).unwrap();
    assert_eq!(s.classifications[&6], "Average Up");
    assert_eq!(s.positions["NCKL"].cost, Decimal::from(37252000));
}
#[test]
fn close_and_reopen() {
    let mut e = buys();
    e.push(event(5, "SELL", 40000, "1000", "0", "0"));
    e.push(event(6, "BUY", 100, "700", "0", "0"));
    let s = replay(&e).unwrap();
    assert_eq!(s.closed.len(), 1);
    assert_eq!(s.positions["NCKL"].cycle, 2);
    assert_eq!(s.positions["NCKL"].cost, Decimal::from(70000));
}
#[test]
fn cash_and_overselling() {
    assert!(replay(&[event(2, "BUY", 100, "1000", "0", "0")]).is_err());
    assert!(replay(&[deposit(), event(2, "SELL", 100, "1000", "0", "0")]).is_err());
    assert!(replay(&[deposit(), event(2, "WITHDRAW", 0, "0", "0", "100000001")]).is_err());
}
#[test]
fn fees_and_additional() {
    let s = replay(&[
        deposit(),
        event(2, "BUY", 100, "1000", "150.25", "0"),
        event(3, "BUY", 100, "1000", "150.25", "0"),
    ])
    .unwrap();
    assert_eq!(s.positions["NCKL"].cost.to_string(), "200300.50");
    assert_eq!(s.classifications[&3], "Additional Buy");
}
#[test]
fn opening_and_dividends() {
    let s = replay(&[
        event(1, "OPENING", 100, "1000", "0", "0"),
        event(2, "DIVIDEND", 0, "0", "0", "2000"),
    ])
    .unwrap();
    assert_eq!(s.cash, Decimal::from(2000));
    assert_eq!(s.contributed, Decimal::from(100000));
    assert_eq!(s.dividends, Decimal::from(2000));
}
#[test]
fn unordered_and_invalid() {
    let mut e = buys();
    e.reverse();
    assert_eq!(replay(&e).unwrap().cash, Decimal::from(65000000));
    for (shares, price) in [
        (0, "100"),
        (101, "100"),
        (-100, "100"),
        (100, "-1"),
        (100, "1.001"),
    ] {
        assert!(replay(&[deposit(), event(2, "BUY", shares, price, "0", "0")]).is_err())
    }
    let mut e = deposit();
    e.date = "2026-02-30".into();
    assert!(replay(&[e]).is_err());
}
#[test]
fn take_profit_allocation() {
    let s = replay(&buys()).unwrap();
    let p = &s.positions["NCKL"];
    let t = targets(p, [1100.into(), 1200.into(), 1350.into()], [25, 35, 40], 25).unwrap();
    assert_eq!(
        t.iter().map(|t| t.lots).collect::<Vec<_>>(),
        vec![100, 140, 160]
    );
    assert_eq!(t[2].remaining_lots, 0);
    assert!(targets(p, [1100.into(), 1200.into(), 1350.into()], [50, 35, 40], 25).is_err());
}
#[test]
fn rounding_lots() {
    let s = replay(&[deposit(), event(2, "BUY", 300, "1000", "0", "0")]).unwrap();
    let t = targets(
        &s.positions["NCKL"],
        [1100.into(), 1200.into(), 1300.into()],
        [25, 35, 40],
        0,
    )
    .unwrap();
    assert_eq!(t.iter().map(|t| t.lots).collect::<Vec<_>>(), vec![0, 1, 1]);
    assert_eq!(t[2].remaining_lots, 1);
}
#[test]
fn persistence_edits_delete_backup_restore() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("main.db");
    let mut db = Store::open(&path).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    for mut e in buys() {
        e.id = 0;
        db.save_event(e).unwrap();
    }
    assert_eq!(db.events().unwrap().len(), 4);
    assert!(db.delete_event(1).is_err());
    let mut bad = db.events().unwrap()[0].clone();
    bad.amount = "1".into();
    assert!(db.save_event(bad).is_err());
    assert_eq!(db.events().unwrap()[0].amount, "100000000");
    let backup = dir.path().join("backup.db");
    db.backup(&backup).unwrap();
    assert!(db.backup(&backup).is_err());
    db.delete_event(4).unwrap();
    db.restore(&backup, &dir.path().join("safety.db")).unwrap();
    assert_eq!(db.events().unwrap().len(), 4);
    drop(db);
    assert_eq!(Store::open(&path).unwrap().events().unwrap().len(), 4);
}
#[test]
fn notes_and_foreign_keys() {
    let dir = tempfile::tempdir().unwrap();
    let db = Store::open(&dir.path().join("db")).unwrap();
    let r = Record {
        id: 0,
        kind: "catalyst".into(),
        stock: "NCKL".into(),
        date: "2026-01-01".into(),
        data: serde_json::json!({"type":"Positive","title":"Kontrak baru"}),
    };
    assert!(db.save_record(r.clone()).is_err());
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    db.save_record(r).unwrap();
    assert_eq!(db.records().unwrap()[0].data["title"], "Kontrak baru");
}
#[test]
fn rules_require_evidence_and_free_float_threshold() {
    let mut r = Record {
        id: 1,
        kind: "rules".into(),
        stock: "NCKL".into(),
        date: "2026-01-01".into(),
        data: serde_json::json!({"Aturan (1–9)":"1","Status (Terpenuhi / Tidak terpenuhi / Belum diverifikasi)":"Terpenuhi","Nilai data":"Syariah","Sumber verifikasi":"Daftar DES"}),
    };
    assert!(!screening(&[], "NCKL")["mandatory_pass"].as_bool().unwrap());
    let one = r.clone();
    r.id = 2;
    r.data["Aturan (1–9)"] = "2".into();
    r.data["Nilai data"] = "25".into();
    assert!(validate_record(&r).is_err());
    r.data["Nilai data"] = "24.99".into();
    assert!(screening(&[one, r.clone()], "NCKL")["mandatory_pass"]
        .as_bool()
        .unwrap());
    r.data["Sumber verifikasi"] = "".into();
    assert!(validate_record(&r).is_err());
}
#[test]
fn plan_stop_and_recalculation() {
    let r = Record {
        id: 0,
        kind: "plan".into(),
        stock: "NCKL".into(),
        date: "2026-01-01".into(),
        data: serde_json::json!({"TP1 harga":"1100","TP2 harga":"1200","TP3 harga":"1350","TP1 alokasi %":"25","TP2 alokasi %":"35","TP3 alokasi %":"40","Stop-loss":"700","Fee jual (bps)":"25"}),
    };
    let s = replay(&buys()).unwrap();
    let v = plan_preview(&s.positions["NCKL"], &r).unwrap();
    assert_eq!(v["targets"][0]["lots"], 100);
    assert_eq!(v["stop_pl"], "-7070000.0000");
    let mut bad = r.clone();
    bad.data["TP1 alokasi %"] = "101".into();
    assert!(validate_record(&bad).is_err());
    assert!(targets(
        &s.positions["NCKL"],
        [1.into(), 2.into(), 3.into()],
        [u32::MAX, 2, 1],
        0
    )
    .is_err());
}
#[test]
fn future_schema_rejected_without_modification() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("future.db");
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch("PRAGMA user_version=99").unwrap();
    assert!(Store::open(&path).is_err());
    assert_eq!(
        conn.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        99
    );
}
#[test]
fn noncanonical_dates_rejected() {
    let mut e = deposit();
    e.date = "2026-1-1".into();
    assert!(replay(&[e]).is_err());
}
#[test]
fn dividend_receipt_is_atomic_and_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let mut db = Store::open(&dir.path().join("db")).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    for mut e in buys() {
        e.id = 0;
        db.save_event(e).unwrap();
    }
    db.save_record(Record{id:0,kind:"dividend".into(),stock:"NCKL".into(),date:"2026-01-10".into(),data:serde_json::json!({"Cum date":"2026-01-05","Payment date":"2026-01-10","Dividend per share":"10","Potongan":"40000","Status (diumumkan / diharapkan / diterima)":"diharapkan"})}).unwrap();
    db.receive_dividend(1).unwrap();
    assert_eq!(
        replay(&db.events().unwrap()).unwrap().cash,
        Decimal::from(65360000)
    );
    assert!(db.receive_dividend(1).is_err());
    assert!(db.delete_event(5).is_err());
    assert_eq!(db.events().unwrap().len(), 5);
}
#[test]
fn plan_revisions_preserve_old_records() {
    let dir = tempfile::tempdir().unwrap();
    let db = Store::open(&dir.path().join("db")).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    let mut r = Record {
        id: 0,
        kind: "plan".into(),
        stock: "NCKL".into(),
        date: "2026-01-01".into(),
        data: serde_json::json!({"TP1 harga":"1100","TP2 harga":"1200","TP3 harga":"1350","TP1 alokasi %":"25","TP2 alokasi %":"35","TP3 alokasi %":"40","Stop-loss":"700"}),
    };
    db.save_record(r.clone()).unwrap();
    r.id = 1;
    r.data["TP3 harga"] = "1500".into();
    db.save_record(r).unwrap();
    let rows = db.records().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].data["TP3 harga"], "1350");
    assert_eq!(rows[1].data["TP3 harga"], "1500");
}
#[test]
fn invalid_dividend_rolls_back_and_can_be_corrected() {
    let dir = tempfile::tempdir().unwrap();
    let mut db = Store::open(&dir.path().join("db")).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    let mut r = Record {
        id: 0,
        kind: "dividend".into(),
        stock: "NCKL".into(),
        date: "2026-01-01".into(),
        data: serde_json::json!({"Cum date":"2026-01-01","Payment date":"2026-01-02","Dividend per share":"10","Lembar berhak":"100"}),
    };
    db.save_record(r.clone()).unwrap();
    assert!(db.receive_dividend(1).is_err());
    assert!(db.events().unwrap().is_empty());
    r.id = 1;
    r.data["Alasan koreksi lembar"] = "Hak dari broker sebelum migrasi".into();
    db.save_record(r).unwrap();
    db.receive_dividend(1).unwrap();
    assert_eq!(db.events().unwrap().len(), 1);
}

#[test]
fn statistics_closed_cycles_exclude_partial_sales_and_deposits() {
    let empty = analytics(&Ledger::default());
    assert!(empty["win_rate"].is_null());
    let mut events = vec![
        deposit(),
        event(2, "BUY", 200, "1000", "0", "0"),
        event(3, "SELL", 100, "1100", "0", "0"),
    ];
    assert_eq!(analytics(&replay(&events).unwrap())["closed_count"], 0);
    events.extend([
        event(4, "SELL", 100, "1100", "0", "0"),
        event(5, "BUY", 100, "1000", "0", "0"),
        event(6, "SELL", 100, "900", "0", "0"),
        event(7, "BUY", 100, "1000", "0", "0"),
        event(8, "SELL", 100, "1000", "0", "0"),
    ]);
    let result = analytics(&replay(&events).unwrap());
    assert_eq!(result["closed_count"], 3);
    assert_eq!(result["wins"], 1);
    assert_eq!(result["losses"], 1);
    assert_eq!(result["breakeven"], 1);
    assert_eq!(result["average_win"], "20000");
    assert_eq!(result["average_loss"], "-10000");
    assert_eq!(result["profit_factor"], "2");
    assert_eq!(result["best"]["cycle"], 1);
    assert_eq!(result["worst"]["cycle"], 2);
}

#[test]
fn journal_edit_persists_and_restores_without_changing_ledger() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("journal.db");
    let mut db = Store::open(&path).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    for mut e in buys() {
        e.id = 0;
        db.save_event(e).unwrap();
    }
    let before = db.state().unwrap()["ledger"].clone();
    let mut e = db.events().unwrap()[1].clone();
    e.journal = "Strategi: beli bertahap\nRisiko: harga turun; keyakinan 70%".into();
    db.save_event(e.clone()).unwrap();
    assert_eq!(db.state().unwrap()["ledger"], before);
    let backup = dir.path().join("backup.db");
    db.backup(&backup).unwrap();
    drop(db);
    db = Store::open(&path).unwrap();
    assert_eq!(db.events().unwrap()[1].journal, e.journal);
    e.journal = "Koreksi".into();
    db.save_event(e).unwrap();
    db.restore(&backup, &dir.path().join("safety.db")).unwrap();
    assert!(db.events().unwrap()[1].journal.contains("beli bertahap"));
}

#[test]
fn watchlist_survives_reopen_and_buy() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("watch.db");
    let mut db = Store::open(&path).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    db.save_record(Record{id:0,kind:"watchlist".into(),stock:"NCKL".into(),date:"2026-01-01".into(),data:serde_json::json!({"Status":"Dipantau","Target entry":"900","Catatan":"Tunggu support"})}).unwrap();
    for mut e in buys() {
        e.id = 0;
        db.save_event(e).unwrap();
    }
    drop(db);
    let db = Store::open(&path).unwrap();
    assert_eq!(db.records().unwrap()[0].data["Catatan"], "Tunggu support");
    assert_eq!(
        db.state().unwrap()["ledger"]["positions"]["NCKL"]["shares"],
        40000
    );
}

#[test]
fn take_profit_status_uses_latest_dated_manual_quote() {
    let dir = tempfile::tempdir().unwrap();
    let mut db = Store::open(&dir.path().join("tp.db")).unwrap();
    db.stock("NCKL", "Trimegah", "Komoditas").unwrap();
    for mut e in buys() {
        e.id = 0;
        db.save_event(e).unwrap();
    }
    db.save_record(Record{id:0,kind:"plan".into(),stock:"NCKL".into(),date:"2026-01-04".into(),data:serde_json::json!({"TP1 harga":"1100","TP2 harga":"1200","TP3 harga":"1350","TP1 alokasi %":"25","TP2 alokasi %":"35","TP3 alokasi %":"40","Stop-loss":"700"})}).unwrap();
    assert_eq!(
        db.state().unwrap()["previews"]["NCKL"]["price_statuses"][0],
        "Harga belum diisi"
    );
    for (date, price) in [("2026-01-10", "1200"), ("2026-01-09", "1400")] {
        db.save_record(Record {
            id: 0,
            kind: "price".into(),
            stock: "NCKL".into(),
            date: date.into(),
            data: serde_json::json!({"price":price}),
        })
        .unwrap();
    }
    let state = db.state().unwrap();
    assert_eq!(
        state["previews"]["NCKL"]["price_statuses"],
        serde_json::json!([
            "Harga mencapai target",
            "Harga mencapai target",
            "Harga di bawah target"
        ])
    );
    assert_eq!(state["previews"]["NCKL"]["price_date"], "2026-01-10");
    assert_eq!(state["ledger"]["positions"]["NCKL"]["shares"], 40000);
}
