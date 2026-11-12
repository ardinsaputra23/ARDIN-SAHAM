use chrono::NaiveDate;
use rusqlite::{params, Connection};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path, str::FromStr};
pub type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn dec(s: &str) -> Result<Decimal> {
    Decimal::from_str(s).map_err(err)
}
fn date(s: &str) -> Result<()> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(err)
        .and_then(|d| {
            if d.format("%Y-%m-%d").to_string() == s {
                Ok(())
            } else {
                Err("Tanggal harus YYYY-MM-DD".into())
            }
        })
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub date: String,
    pub kind: String,
    pub stock: String,
    pub shares: i64,
    pub price: String,
    pub fee: String,
    pub amount: String,
    pub journal: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Position {
    pub stock: String,
    pub cycle: u32,
    pub shares: i64,
    pub cost: Decimal,
    pub realized: Decimal,
    pub opened: String,
    pub closed: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct Ledger {
    pub cash: Decimal,
    pub contributed: Decimal,
    pub dividends: Decimal,
    pub realized: Decimal,
    pub positions: BTreeMap<String, Position>,
    pub closed: Vec<Position>,
    pub classifications: BTreeMap<i64, String>,
}
fn money(s: &str) -> Result<Decimal> {
    let n = dec(s)?;
    if n < Decimal::ZERO || n.scale() > 2 || n > Decimal::from(1_000_000_000_000_000i64) {
        return Err("Nominal harus positif, maksimal 2 desimal dan Rp1.000 triliun".into());
    }
    Ok(n)
}
pub fn replay(events: &[Event]) -> Result<Ledger> {
    let mut state = Ledger::default();
    let mut ordered = events.to_vec();
    ordered.sort_by(|a, b| (&a.date, a.id).cmp(&(&b.date, b.id)));
    for e in ordered {
        date(&e.date)?;
        let price = money(&e.price)?;
        let fee = money(&e.fee)?;
        let amount = money(&e.amount)?;
        if e.shares < 0 || e.shares > 1_000_000_000 {
            return Err("Jumlah lembar tidak valid".into());
        }
        match e.kind.as_str() {
            "DEPOSIT" => {
                if amount.is_zero() {
                    return Err("Setoran harus lebih dari nol".into());
                }
                state.cash += amount;
                state.contributed += amount;
            }
            "WITHDRAW" => {
                if amount.is_zero() {
                    return Err("Penarikan harus lebih dari nol".into());
                }
                state.cash -= amount;
                state.contributed -= amount;
            }
            "DIVIDEND" => {
                if e.stock.is_empty() || amount.is_zero() {
                    return Err("Dividen memerlukan saham dan nominal bersih".into());
                }
                state.cash += amount;
                state.dividends += amount;
            }
            "BUY" | "SELL" | "OPENING" => {
                if e.stock.is_empty() || e.shares == 0 || e.shares % 100 != 0 || price.is_zero() {
                    return Err("Saham, harga positif, dan lot bulat wajib diisi".into());
                }
                let p = state
                    .positions
                    .entry(e.stock.clone())
                    .or_insert_with(|| Position {
                        stock: e.stock.clone(),
                        ..Default::default()
                    });
                let qty = Decimal::from(e.shares);
                let gross = price * qty;
                if e.kind == "SELL" {
                    if e.shares > p.shares {
                        return Err(format!("Overselling {} pada {}", e.stock, e.date));
                    }
                    if fee > gross {
                        return Err("Fee melebihi penjualan".into());
                    }
                    let basis = if e.shares == p.shares {
                        p.cost
                    } else {
                        p.cost / Decimal::from(p.shares) * qty
                    };
                    let profit = gross - fee - basis;
                    p.cost -= basis;
                    p.shares -= e.shares;
                    p.realized += profit;
                    state.realized += profit;
                    state.cash += gross - fee;
                    if p.shares == 0 {
                        p.closed = Some(e.date.clone());
                        state.closed.push(p.clone());
                    }
                } else {
                    let cost = gross + fee;
                    let class = if p.shares == 0 {
                        "First Buy"
                    } else {
                        let prev = p.cost / Decimal::from(p.shares);
                        let next = cost / qty;
                        if next < prev {
                            "Average Down"
                        } else if next > prev {
                            "Average Up"
                        } else {
                            "Additional Buy"
                        }
                    };
                    state.classifications.insert(e.id, class.into());
                    if p.shares == 0 {
                        p.cycle += 1;
                        p.opened = e.date.clone();
                        p.closed = None;
                        p.realized = Decimal::ZERO;
                    }
                    p.shares += e.shares;
                    p.cost += cost;
                    if e.kind == "OPENING" {
                        state.contributed += cost;
                    } else {
                        state.cash -= cost;
                    }
                }
            }
            _ => return Err("Jenis transaksi tidak dikenal".into()),
        }
        if state.cash < Decimal::ZERO {
            return Err(format!("Saldo kas tidak cukup pada {}", e.date));
        }
    }
    Ok(state)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: i64,
    pub kind: String,
    pub stock: String,
    pub date: String,
    pub data: serde_json::Value,
}
pub struct Store {
    pub conn: Connection,
}
impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path).map_err(err)?;
        let version: i64 = conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(err)?;
        if version > 1 {
            return Err("Database berasal dari versi aplikasi lebih baru".into());
        }
        conn.busy_timeout(std::time::Duration::from_secs(5))
            .map_err(err)?;
        conn.execute_batch("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL; BEGIN IMMEDIATE; CREATE TABLE IF NOT EXISTS migrations(version INTEGER PRIMARY KEY); INSERT OR IGNORE INTO migrations VALUES(1); CREATE TABLE IF NOT EXISTS stocks(code TEXT PRIMARY KEY, name TEXT NOT NULL, sector TEXT NOT NULL,created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP); CREATE TABLE IF NOT EXISTS events(id INTEGER PRIMARY KEY AUTOINCREMENT,date TEXT NOT NULL,kind TEXT NOT NULL,stock TEXT REFERENCES stocks(code),payload TEXT NOT NULL,created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP); CREATE INDEX IF NOT EXISTS event_order ON events(date,id); CREATE TABLE IF NOT EXISTS records(id INTEGER PRIMARY KEY AUTOINCREMENT,kind TEXT NOT NULL,stock TEXT REFERENCES stocks(code),date TEXT NOT NULL,payload TEXT NOT NULL,created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP); PRAGMA user_version=1; COMMIT;").map_err(err)?;
        Ok(Self { conn })
    }
    pub fn stocks(&self) -> Result<Vec<serde_json::Value>> {
        let mut s = self
            .conn
            .prepare("SELECT code,name,sector FROM stocks ORDER BY code")
            .map_err(err)?;
        let r=s.query_map([],|r|Ok(serde_json::json!({"code":r.get::<_,String>(0)?,"name":r.get::<_,String>(1)?,"sector":r.get::<_,String>(2)?}))).map_err(err)?.collect::<std::result::Result<Vec<_>,_>>().map_err(err)?;
        Ok(r)
    }
    pub fn stock(&self, code: &str, name: &str, sector: &str) -> Result<()> {
        if code.is_empty()
            || code.len() > 10
            || !code
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            || name.trim().is_empty()
        {
            return Err("Kode dan nama saham tidak valid".into());
        }
        self.conn.execute("INSERT INTO stocks(code,name,sector) VALUES(?1,?2,?3) ON CONFLICT(code) DO UPDATE SET name=excluded.name,sector=excluded.sector,updated_at=CURRENT_TIMESTAMP",params![code,name,sector]).map_err(err)?;
        Ok(())
    }
    pub fn events(&self) -> Result<Vec<Event>> {
        let mut s = self
            .conn
            .prepare("SELECT id,payload FROM events ORDER BY date,id")
            .map_err(err)?;
        let rows = s
            .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))
            .map_err(err)?;
        rows.map(|r| {
            let (id, p) = r.map_err(err)?;
            let mut e: Event = serde_json::from_str(&p).map_err(err)?;
            e.id = id;
            Ok(e)
        })
        .collect()
    }
    pub fn save_event(&mut self, e: Event) -> Result<()> {
        self.guard_dividend_event(e.id)?;
        let tx = self.conn.transaction().map_err(err)?;
        let payload = serde_json::to_string(&e).map_err(err)?;
        let stock = if e.stock.is_empty() {
            None
        } else {
            Some(&e.stock)
        };
        if e.id == 0 {
            tx.execute(
                "INSERT INTO events(date,kind,stock,payload) VALUES(?1,?2,?3,?4)",
                params![e.date, e.kind, stock, payload],
            )
            .map_err(err)?;
        } else {
            if tx.execute("UPDATE events SET date=?1,kind=?2,stock=?3,payload=?4,updated_at=CURRENT_TIMESTAMP WHERE id=?5",params![e.date,e.kind,stock,payload,e.id]).map_err(err)?!=1{return Err("Transaksi tidak ditemukan".into())}
        }
        let mut events = Vec::new();
        {
            let mut q = tx.prepare("SELECT id,payload FROM events").map_err(err)?;
            let rows = q
                .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))
                .map_err(err)?;
            for row in rows {
                let (id, p) = row.map_err(err)?;
                let mut e: Event = serde_json::from_str(&p).map_err(err)?;
                e.id = id;
                events.push(e);
            }
        }
        replay(&events)?;
        tx.commit().map_err(err)
    }
    pub fn delete_event(&mut self, id: i64) -> Result<()> {
        self.conn.execute_batch("BEGIN IMMEDIATE").map_err(err)?;
        let result = (|| {
            self.guard_dividend_event(id)?;
            if self
                .conn
                .execute("DELETE FROM events WHERE id=?1", [id])
                .map_err(err)?
                != 1
            {
                return Err("Transaksi tidak ditemukan".into());
            }
            replay(&self.events()?)?;
            Ok(())
        })();
        match result {
            Ok(()) => self.conn.execute_batch("COMMIT").map_err(err),
            Err(e) => {
                self.conn.execute_batch("ROLLBACK").map_err(err)?;
                Err(e)
            }
        }
    }
    pub fn records(&self) -> Result<Vec<Record>> {
        let mut q = self
            .conn
            .prepare("SELECT id,kind,coalesce(stock,''),date,payload FROM records ORDER BY date,id")
            .map_err(err)?;
        let rows = q
            .query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, String>(4)?,
                ))
            })
            .map_err(err)?;
        rows.map(|r| {
            let (id, kind, stock, date, p) = r.map_err(err)?;
            Ok(Record {
                id,
                kind,
                stock,
                date,
                data: serde_json::from_str(&p).map_err(err)?,
            })
        })
        .collect()
    }
    pub fn save_record(&self, mut r: Record) -> Result<()> {
        date(&r.date)?;
        validate_record(&r)?;
        if r.kind == "dividend" {
            if !field(&r, "ID transaksi kas dividen").is_empty()
                || field(&r, "Status (diumumkan / diharapkan / diterima)") == "diterima"
            {
                return Err("Gunakan tindakan Terima dividen untuk mencatat penerimaan".into());
            }
            if r.id > 0
                && self
                    .records()?
                    .iter()
                    .any(|old| old.id == r.id && !field(old, "ID transaksi kas dividen").is_empty())
            {
                return Err("Dividen diterima tidak dapat diubah".into());
            }
        }
        if r.kind == "plan" {
            r.id = 0;
        }
        if ![
            "watchlist",
            "rules",
            "fundamental",
            "technical",
            "plan",
            "catalyst",
            "evaluation",
            "dividend",
            "price",
            "snapshot",
        ]
        .contains(&r.kind.as_str())
        {
            return Err("Jenis catatan tidak valid".into());
        }
        if !r.data.is_object() {
            return Err("Data catatan tidak valid".into());
        }
        if r.kind == "price" {
            money(r.data["price"].as_str().ok_or("Harga wajib diisi")?)?;
        }
        let stock = if r.stock.is_empty() {
            None
        } else {
            Some(&r.stock)
        };
        let p = r.data.to_string();
        if r.id == 0 {
            self.conn
                .execute(
                    "INSERT INTO records(kind,stock,date,payload) VALUES(?1,?2,?3,?4)",
                    params![r.kind, stock, r.date, p],
                )
                .map_err(err)?;
        } else {
            if self.conn.execute("UPDATE records SET kind=?1,stock=?2,date=?3,payload=?4,updated_at=CURRENT_TIMESTAMP WHERE id=?5",params![r.kind,stock,r.date,p,r.id]).map_err(err)?!=1{return Err("Catatan tidak ditemukan".into())}
        }
        Ok(())
    }
    pub fn backup(&self, path: &Path) -> Result<()> {
        if path.exists() {
            return Err("Tujuan backup sudah ada; gunakan nama baru".into());
        }
        self.conn
            .backup(rusqlite::DatabaseName::Main, path, None)
            .map_err(err)
    }
    pub fn restore(&mut self, path: &Path, safety: &Path) -> Result<()> {
        let src = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(err)?;
        let integrity: String = src
            .query_row("PRAGMA integrity_check", [], |r| r.get(0))
            .map_err(err)?;
        let version: i64 = src
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(err)?;
        if integrity != "ok" || version != 1 {
            return Err("Backup tidak valid atau versi tidak didukung".into());
        }
        let candidate = Self { conn: src };
        replay(&candidate.events()?)?;
        candidate.stocks()?;
        candidate.records()?;
        let fk: i64 = candidate
            .conn
            .query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| {
                r.get(0)
            })
            .map_err(err)?;
        if fk != 0 {
            return Err("Relasi backup rusak".into());
        }
        self.backup(safety)?;
        self.conn
            .restore(
                rusqlite::DatabaseName::Main,
                path,
                None::<fn(rusqlite::backup::Progress)>,
            )
            .map_err(err)
    }
}
#[derive(Debug, Serialize)]
pub struct Target {
    pub lots: i64,
    pub proceeds: Decimal,
    pub profit: Decimal,
    pub remaining_lots: i64,
}
pub fn targets(
    p: &Position,
    prices: [Decimal; 3],
    percent: [u32; 3],
    fee_bps: u32,
) -> Result<Vec<Target>> {
    if percent.iter().any(|x| *x > 100)
        || percent.iter().map(|x| u64::from(*x)).sum::<u64>() > 100
        || fee_bps > 10000
        || p.shares <= 0
        || prices.iter().any(|x| *x <= Decimal::ZERO)
    {
        return Err("Alokasi/harga target tidak valid".into());
    }
    let lots = p.shares / 100;
    let mut remaining = lots;
    Ok(prices
        .iter()
        .zip(percent)
        .map(|(price, pct)| {
            let n = lots * i64::from(pct) / 100;
            remaining -= n;
            let gross = *price * Decimal::from(n * 100);
            let proceeds = gross * (Decimal::ONE - Decimal::from(fee_bps) / Decimal::from(10000));
            Target {
                lots: n,
                proceeds,
                profit: proceeds - p.cost / Decimal::from(p.shares) * Decimal::from(n * 100),
                remaining_lots: remaining,
            }
        })
        .collect())
}

fn field<'a>(r: &'a Record, k: &str) -> &'a str {
    r.data[k].as_str().unwrap_or("")
}
pub fn validate_record(r: &Record) -> Result<()> {
    if r.kind != "snapshot" && r.stock.is_empty() {
        return Err("Saham wajib dipilih".into());
    }
    if r.kind == "price" && money(field(r, "price"))?.is_zero() {
        return Err("Harga harus lebih dari nol".into());
    }
    if r.kind == "rules" {
        let n = field(r, "Aturan (1–9)").parse::<u8>().map_err(err)?;
        let status = field(
            r,
            "Status (Terpenuhi / Tidak terpenuhi / Belum diverifikasi)",
        );
        if !(1..=9).contains(&n)
            || !["Terpenuhi", "Tidak terpenuhi", "Belum diverifikasi"].contains(&status)
        {
            return Err("Nomor atau status aturan tidak valid".into());
        }
        if status != "Belum diverifikasi"
            && (field(r, "Sumber verifikasi").trim().is_empty()
                || field(r, "Nilai data").trim().is_empty())
        {
            return Err("Verifikasi memerlukan nilai dan sumber".into());
        }
        if n == 2 && status == "Terpenuhi" && money(field(r, "Nilai data"))? >= Decimal::from(25) {
            return Err("Free float harus kurang dari 25%".into());
        }
    }
    if r.kind == "plan" {
        plan_inputs(r)?;
    }
    Ok(())
}
fn plan_inputs(r: &Record) -> Result<([Decimal; 3], [u32; 3], Decimal, u32)> {
    let prices = [
        money(field(r, "TP1 harga"))?,
        money(field(r, "TP2 harga"))?,
        money(field(r, "TP3 harga"))?,
    ];
    let pct = [
        field(r, "TP1 alokasi %").parse::<u32>().map_err(err)?,
        field(r, "TP2 alokasi %").parse::<u32>().map_err(err)?,
        field(r, "TP3 alokasi %").parse::<u32>().map_err(err)?,
    ];
    let stop = money(field(r, "Stop-loss"))?;
    let fees = if field(r, "Fee jual (bps)").is_empty() {
        0
    } else {
        field(r, "Fee jual (bps)").parse::<u32>().map_err(err)?
    };
    if prices.iter().any(|p| p.is_zero())
        || stop.is_zero()
        || prices[0] > prices[1]
        || prices[1] > prices[2]
        || stop >= prices[0]
        || pct.iter().any(|n| *n > 100)
        || pct.iter().map(|n| u64::from(*n)).sum::<u64>() > 100
        || fees > 10000
    {
        return Err(
            "Harga TP harus berurutan, stop di bawah TP1, total alokasi maksimal 100%".into(),
        );
    }
    Ok((prices, pct, stop, fees))
}
pub fn plan_preview(p: &Position, r: &Record) -> Result<serde_json::Value> {
    let (prices, pct, stop, fees) = plan_inputs(r)?;
    let targets = targets(p, prices, pct, fees)?;
    let avg = p.cost / Decimal::from(p.shares);
    let returns = prices.map(|v| (v / avg - Decimal::ONE) * Decimal::from(100));
    let risk = avg - stop;
    let rr = prices.map(|v| {
        if risk > Decimal::ZERO {
            Some((v - avg) / risk)
        } else {
            None
        }
    });
    Ok(
        serde_json::json!({"targets":targets,"average":avg,"returns":returns,"risk_reward":rr,"stop_pl":stop*Decimal::from(p.shares)*(Decimal::ONE-Decimal::from(fees)/Decimal::from(10000))-p.cost,"shares":p.shares,"rounding":"Lot dibulatkan ke bawah; basis setiap TP adalah seluruh lot posisi saat ini."}),
    )
}
pub fn screening(records: &[Record], stock: &str) -> serde_json::Value {
    let mut status = vec!["Belum diverifikasi".to_string(); 9];
    let mut ordered = records
        .iter()
        .filter(|r| r.stock == stock && r.kind == "rules")
        .collect::<Vec<_>>();
    ordered.sort_by(|a, b| (&a.date, a.id).cmp(&(&b.date, b.id)));
    for r in ordered {
        if validate_record(r).is_ok() {
            if let Ok(n) = field(r, "Aturan (1–9)").parse::<usize>() {
                status[n - 1] = field(
                    r,
                    "Status (Terpenuhi / Tidak terpenuhi / Belum diverifikasi)",
                )
                .into();
            }
        }
    }
    serde_json::json!({"mandatory_pass":status[0]=="Terpenuhi"&&status[1]=="Terpenuhi","statuses":status})
}
pub fn analytics(ledger: &Ledger) -> serde_json::Value {
    let wins = ledger
        .closed
        .iter()
        .filter(|p| p.realized > Decimal::ZERO)
        .collect::<Vec<_>>();
    let losses = ledger
        .closed
        .iter()
        .filter(|p| p.realized < Decimal::ZERO)
        .collect::<Vec<_>>();
    let gain: Decimal = wins.iter().map(|p| p.realized).sum();
    let loss: Decimal = losses.iter().map(|p| p.realized).sum();
    let count = ledger.closed.len();
    let days: i64 = ledger
        .closed
        .iter()
        .filter_map(|p| {
            Some(
                (NaiveDate::parse_from_str(p.closed.as_ref()?, "%Y-%m-%d").ok()?
                    - NaiveDate::parse_from_str(&p.opened, "%Y-%m-%d").ok()?)
                .num_days(),
            )
        })
        .sum();
    serde_json::json!({"average_win":if wins.is_empty(){None}else{Some(gain/Decimal::from(wins.len() as u64))},"average_loss":if losses.is_empty(){None}else{Some(loss/Decimal::from(losses.len() as u64))},"profit_factor":if loss.is_zero(){None}else{Some(gain/(-loss))},"average_holding_days":if count==0{None}else{Some(Decimal::from(days)/Decimal::from(count as u64))},"best":ledger.closed.iter().max_by_key(|p|p.realized),"worst":ledger.closed.iter().min_by_key(|p|p.realized)})
}
impl Store {
    pub fn state(&self) -> Result<serde_json::Value> {
        let events = self.events()?;
        let records = self.records()?;
        let stocks = self.stocks()?;
        let ledger = replay(&events)?;
        let mut previews = BTreeMap::new();
        let mut screening_results = BTreeMap::new();
        for s in &stocks {
            let code = s["code"].as_str().ok_or("Kode saham tidak valid")?;
            screening_results.insert(code.to_string(), screening(&records, code));
            if let Some(p) = ledger.positions.get(code).filter(|p| p.shares > 0) {
                if let Some(r) = records
                    .iter()
                    .rev()
                    .find(|r| r.kind == "plan" && r.stock == code)
                {
                    previews.insert(
                        code.to_string(),
                        match plan_preview(p, r) {
                            Ok(v) => v,
                            Err(e) => serde_json::json!({"error":e}),
                        },
                    );
                }
            }
        }
        Ok(
            serde_json::json!({"stocks":stocks,"events":events,"records":records,"analytics":analytics(&ledger),"ledger":ledger,"previews":previews,"screening":screening_results}),
        )
    }
}
impl Store {
    pub fn receive_dividend(&mut self, id: i64) -> Result<()> {
        self.conn.execute_batch("BEGIN IMMEDIATE").map_err(err)?;
        let result = (|| {
            let mut r = self
                .records()?
                .into_iter()
                .find(|r| r.id == id && r.kind == "dividend")
                .ok_or("Catatan dividen tidak ditemukan")?;
            if !field(&r, "ID transaksi kas dividen").is_empty() {
                return Err("Dividen sudah diterima".into());
            }
            let cum = field(&r, "Cum date");
            date(cum)?;
            let payment = field(&r, "Payment date").to_string();
            date(&payment)?;
            if payment.as_str() < cum {
                return Err("Payment date sebelum cum date".into());
            }
            let events = self.events()?;
            let at_cum = events
                .iter()
                .filter(|e| e.date.as_str() <= cum)
                .cloned()
                .collect::<Vec<_>>();
            let owned = replay(&at_cum)?
                .positions
                .get(&r.stock)
                .map(|p| p.shares)
                .unwrap_or(0);
            let shares = if field(&r, "Lembar berhak").is_empty() {
                owned
            } else {
                field(&r, "Lembar berhak").parse::<i64>().map_err(err)?
            };
            if shares <= 0 || shares > 1_000_000_000 {
                return Err("Lembar berhak harus positif".into());
            }
            if shares != owned && field(&r, "Alasan koreksi lembar").trim().is_empty() {
                return Err("Perubahan lembar berhak memerlukan alasan".into());
            }
            let dps = money(field(&r, "Dividend per share"))?;
            let deductions = money(if field(&r, "Potongan").is_empty() {
                "0"
            } else {
                field(&r, "Potongan")
            })?;
            let gross = dps * Decimal::from(shares);
            let net = gross - deductions;
            if net <= Decimal::ZERO {
                return Err("Dividen bersih harus positif".into());
            }
            let e=Event{id:0,date:payment.clone(),kind:"DIVIDEND".into(),stock:r.stock.clone(),shares:0,price:"0".into(),fee:"0".into(),amount:net.to_string(),journal:format!("Penerimaan dividen catatan #{id}; {shares} lembar; bruto {gross}; potongan {deductions}")};
            self.conn
                .execute(
                    "INSERT INTO events(date,kind,stock,payload) VALUES(?1,?2,?3,?4)",
                    params![
                        payment,
                        e.kind,
                        e.stock,
                        serde_json::to_string(&e).map_err(err)?
                    ],
                )
                .map_err(err)?;
            let event_id = self.conn.last_insert_rowid();
            replay(&self.events()?)?;
            r.data["ID transaksi kas dividen"] = event_id.to_string().into();
            r.data["Status (diumumkan / diharapkan / diterima)"] = "diterima".into();
            r.data["Lembar berhak"] = shares.to_string().into();
            r.data["Dividen bruto"] = gross.to_string().into();
            r.data["Dividen bersih"] = net.to_string().into();
            self.conn
                .execute(
                    "UPDATE records SET payload=?1,updated_at=CURRENT_TIMESTAMP WHERE id=?2",
                    params![r.data.to_string(), id],
                )
                .map_err(err)?;
            Ok(())
        })();
        match result {
            Ok(()) => self.conn.execute_batch("COMMIT").map_err(err),
            Err(e) => {
                self.conn.execute_batch("ROLLBACK").map_err(err)?;
                Err(e)
            }
        }
    }
    fn guard_dividend_event(&self, id: i64) -> Result<()> {
        if id > 0
            && self.records()?.iter().any(|r| {
                r.kind == "dividend" && field(r, "ID transaksi kas dividen") == id.to_string()
            })
        {
            return Err(
                "Transaksi terhubung ke dividen diterima; koreksi otomatis belum didukung".into(),
            );
        }
        Ok(())
    }
}
