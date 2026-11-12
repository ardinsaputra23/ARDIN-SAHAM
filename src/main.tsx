import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import "./style.css";
type Event = {
  id: number;
  date: string;
  kind: string;
  stock: string;
  shares: number;
  price: string;
  fee: string;
  amount: string;
  journal: string;
};
type RecordRow = {
  id: number;
  kind: string;
  stock: string;
  date: string;
  data: Record<string, string>;
};
type Position = {
  stock: string;
  cycle: number;
  shares: number;
  cost: string;
  realized: string;
  opened: string;
  closed: string | null;
};
type Preview = {
  error?: string;
  average: string;
  returns: string[];
  risk_reward: (string | null)[];
  stop_pl: string;
  shares: number;
  rounding: string;
  targets: {
    lots: number;
    proceeds: string;
    profit: string;
    remaining_lots: number;
  }[];
};
type State = {
  previews: Record<string, Preview>;
  screening: Record<string, { mandatory_pass: boolean; statuses: string[] }>;
  analytics: {
    average_win: string | null;
    average_loss: string | null;
    profit_factor: string | null;
    average_holding_days: string | null;
  };
  stocks: { code: string; name: string; sector: string }[];
  events: Event[];
  records: RecordRow[];
  ledger: {
    cash: string;
    contributed: string;
    dividends: string;
    realized: string;
    positions: Record<string, Position>;
    closed: Position[];
    classifications: Record<string, string>;
  };
};
const menus = [
  "Dashboard",
  "Portfolio",
  "Transactions",
  "Watchlist",
  "Performance & Evaluation",
  "Dividend Tracker",
  "History & Reports",
  "Settings & Backup",
];
const tabs = [
  "Overview",
  "Investment Rules",
  "Technical & Fundamental",
  "Trading Plan",
  "Catalysts & News",
  "Transaction History",
];
const today = () => {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
};
const rupiah = (v: number | string) =>
  new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    maximumFractionDigits: 2,
  }).format(Number(v));
const rules = [
  "Saham syariah (wajib)",
  "Free float <25% (wajib)",
  "Volatilitas ekstrem >2 kali setahun",
  "Kisaran harga lima tahun",
  "Entry di support kuat / bottom",
  "SSID turun selama enam bulan",
  "Sektor komoditas (preferensi)",
  "Indikasi big accumulation",
  "Perusahaan membagikan dividen",
];
const fields: Record<string, string[]> = {
  watchlist: [
    "Status",
    "Target entry",
    "Support",
    "Resistance",
    "Stop-loss",
    "Catatan",
  ],
  fundamental: [
    "Periode laporan",
    "Revenue",
    "Net income",
    "EPS",
    "Book value per share",
    "PBV",
    "PER",
    "ROE",
    "DER",
    "Dividend per share",
    "Dividend yield",
    "Free float",
    "Sumber nilai (input langsung / perhitungan)",
    "Catatan kualitas bisnis",
  ],
  technical: [
    "Support 1",
    "Support 2",
    "Resistance 1",
    "Resistance 2",
    "Area bottom",
    "Highest 52 minggu",
    "Lowest 52 minggu",
    "Volume",
    "Catatan volume",
    "Breakout",
    "Accumulation",
    "Target entry",
    "Stop-loss",
    "Catatan",
  ],
  plan: [
    "TP1 harga",
    "TP1 alokasi %",
    "TP2 harga",
    "TP2 alokasi %",
    "TP3 harga",
    "TP3 alokasi %",
    "Stop-loss",
    "Alasan invalidasi",
    "Catatan disiplin",
    "Fee jual (bps)",
  ],
  catalyst: [
    "Judul",
    "Jenis (positif / negatif)",
    "Kategori",
    "Sumber",
    "URL sumber",
    "Ringkasan",
    "Analisis pribadi",
    "Estimasi dampak",
    "Status",
    "Evaluasi",
    "ID transaksi terkait",
  ],
  evaluation: [
    "ID transaksi",
    "Kategori kesalahan (pisahkan koma)",
    "Penjelasan",
    "Tindakan seharusnya",
    "Pelajaran",
    "Sesuai trading plan (ya / tidak)",
    "Strategi",
  ],
  dividend: [
    "Cum date",
    "Ex date",
    "Recording date",
    "Payment date",
    "Dividend per share",
    "Lembar berhak",
    "Alasan koreksi lembar",
    "Potongan",
    "Status (diumumkan / diharapkan / diterima)",
  ],
  price: ["price"],
  rules: [
    "Aturan (1–9)",
    "Status (Terpenuhi / Tidak terpenuhi / Belum diverifikasi)",
    "Nilai data",
    "Sumber verifikasi",
    "Catatan",
  ],
};
function App() {
  const [page, setPage] = useState(menus[0]),
    [data, setData] = useState<State | null>(null),
    [error, setError] = useState(""),
    [busy, setBusy] = useState(false),
    [modal, setModal] = useState<string | null>(null),
    [edit, setEdit] = useState<Event | RecordRow | null>(null),
    [stock, setStock] = useState(""),
    [tab, setTab] = useState(tabs[0]),
    [search, setSearch] = useState(""),
    [from, setFrom] = useState(""),
    [to, setTo] = useState("");
  const reload = async () => {
    setData(await invoke<State>("read_state"));
  };
  useEffect(() => {
    reload().catch((e) => setError(String(e)));
  }, []);
  async function action(f: () => Promise<unknown>) {
    setBusy(true);
    setError("");
    try {
      await f();
      await reload();
      setModal(null);
      setEdit(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  const records = (kind: string) =>
    data?.records.filter(
      (r) => r.kind === kind && (!stock || r.stock === stock),
    ) ?? [];
  const latest = (kind: string, code: string) =>
    data?.records.filter((r) => r.kind === kind && r.stock === code).at(-1);
  const active = Object.values(data?.ledger.positions ?? {}).filter(
    (p) => p.shares > 0,
  );
  const price = (p: Position) => latest("price", p.stock);
  const complete = active.every((p) => price(p));
  const market = active.reduce(
    (a, p) => a + Number(price(p)?.data.price ?? 0) * p.shares,
    0,
  );
  const cost = active.reduce((a, p) => a + Number(p.cost), 0);
  const events = (data?.events ?? []).filter(
    (e) =>
      (!stock || e.stock === stock) &&
      (!from || e.date >= from) &&
      (!to || e.date <= to),
  );
  function button(label: string, kind: string) {
    return (
      <button
        onClick={() => {
          setEdit(null);
          setModal(kind);
        }}
      >
        {label}
      </button>
    );
  }
  function screeningPanel() {
    const result = data?.screening[stock];
    return result ? (
      <section>
        <h2>Verifikasi {stock}</h2>
        <p>
          {result.mandatory_pass
            ? "Syariah dan free float terverifikasi memenuhi syarat"
            : "Syarat wajib belum terpenuhi / belum diverifikasi"}
        </p>
        <ul>
          {result.statuses.map((status, i) => (
            <li key={i}>
              {rules[i]}: <b>{status}</b>
            </li>
          ))}
        </ul>
        <small>
          Hasil memakai verifikasi terbaru. Tanggal dan sumber tetap perlu
          diperiksa; bukan rekomendasi beli.
        </small>
      </section>
    ) : null;
  }
  function planPanel() {
    const p = data?.previews[stock];
    return !p ? null : p.error ? (
      <p className="error">{p.error}</p>
    ) : (
      <section>
        <h2>Estimasi TP dari posisi saat ini</h2>
        <p>
          Average cost {rupiah(p.average)} · {p.shares / 100} lot · Estimasi P/L
          stop-loss {rupiah(p.stop_pl)}
        </p>
        <div className="table-scroll">
          <table>
            <thead>
              <tr>
                {[
                  "Target",
                  "Lot",
                  "Kenaikan %",
                  "Hasil bersih",
                  "Estimasi P/L",
                  "Risk/reward",
                  "Sisa lot",
                ].map((h) => (
                  <th key={h}>{h}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {p.targets.map((t, i) => (
                <tr key={i}>
                  <td>TP{i + 1}</td>
                  <td>{t.lots}</td>
                  <td>{Number(p.returns[i]).toFixed(2)}%</td>
                  <td>{rupiah(t.proceeds)}</td>
                  <td>{rupiah(t.profit)}</td>
                  <td>
                    {p.risk_reward[i]
                      ? Number(p.risk_reward[i]).toFixed(2)
                      : "Tidak berlaku"}
                  </td>
                  <td>{t.remaining_lots}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <p>{p.rounding}</p>
        <small>
          Estimasi berubah setelah transaksi baru. Tinjau alokasi setelah
          menambah atau mengurangi posisi. Rencana tidak mengeksekusi order.
        </small>
      </section>
    );
  }
  function recordTable(kind: string) {
    return (
      <>
        <div className="toolbar">{button("Tambah catatan", kind)}</div>
        <div className="table-scroll">
          <table>
            <thead>
              <tr>
                <th>Tanggal</th>
                <th>Saham</th>
                <th>Catatan</th>
                <th>Aksi</th>
              </tr>
            </thead>
            <tbody>
              {records(kind).map((r) => (
                <tr key={r.id}>
                  <td>{r.date}</td>
                  <td>{r.stock}</td>
                  <td>
                    {Object.entries(r.data).map(([k, v]) => (
                      <div key={k}>
                        <b>{k}: </b>
                        {v}
                      </div>
                    ))}
                  </td>
                  <td>
                    <button
                      onClick={() => {
                        setEdit(r);
                        setModal(kind);
                      }}
                      disabled={
                        kind === "dividend" &&
                        !!r.data["ID transaksi kas dividen"]
                      }
                    >
                      Edit
                    </button>
                    {kind === "dividend" &&
                      !r.data["ID transaksi kas dividen"] && (
                        <button
                          disabled={busy}
                          onClick={() => {
                            if (
                              confirm(
                                "Dana dividen ini sudah benar-benar diterima? Kas akan ditambah sekali.",
                              )
                            )
                              void action(() =>
                                invoke("receive_dividend", { id: r.id }),
                              );
                          }}
                        >
                          Terima dividen
                        </button>
                      )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
          {records(kind).length === 0 && (
            <p className="empty">
              Belum ada catatan. Semua informasi dimasukkan manual.
            </p>
          )}
        </div>
      </>
    );
  }
  function transactionTable() {
    return (
      <div className="table-scroll">
        <table>
          <thead>
            <tr>
              {[
                "Tanggal",
                "Saham",
                "Jenis",
                "Lembar",
                "Harga",
                "Fee",
                "Nominal kas",
                "Jurnal",
                "Aksi",
              ].map((x) => (
                <th key={x}>{x}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {events.map((e) => (
              <tr key={e.id}>
                <td>{e.date}</td>
                <td>{e.stock || "—"}</td>
                <td>
                  {e.kind}
                  <small>{data?.ledger.classifications[e.id]}</small>
                </td>
                <td>{e.shares}</td>
                <td>{rupiah(e.price)}</td>
                <td>{rupiah(e.fee)}</td>
                <td>{rupiah(e.amount)}</td>
                <td className="journal">{e.journal || "Belum dilengkapi"}</td>
                <td>
                  <button
                    onClick={() => {
                      setEdit(e);
                      setModal("event");
                    }}
                  >
                    Edit
                  </button>
                  <button
                    className="danger"
                    onClick={() => {
                      if (
                        confirm(
                          "Hapus transaksi? Seluruh ledger akan dihitung ulang.",
                        )
                      )
                        void action(() => invoke("delete_event", { id: e.id }));
                    }}
                  >
                    Hapus
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {!events.length && (
          <p className="empty">Belum ada transaksi dalam periode ini.</p>
        )}
      </div>
    );
  }
  function exportCsv() {
    const cols = [
      "id",
      "date",
      "kind",
      "stock",
      "shares",
      "price",
      "fee",
      "amount",
      "journal",
    ] as const;
    const quote = (x: unknown) =>
      '"' +
      String(x ?? "")
        .replace(/^[=+@-]/, "'$&")
        .replaceAll('"', '""') +
      '"';
    const csv =
      "\uFEFF" +
      [
        cols.join(","),
        ...events.map((e) => cols.map((k) => quote(e[k])).join(",")),
      ].join("\r\n");
    const a = document.createElement("a");
    const url = URL.createObjectURL(
      new Blob([csv], { type: "text/csv;charset=utf-8" }),
    );
    a.href = url;
    a.download = "ardin-transactions.csv";
    a.click();
    URL.revokeObjectURL(url);
  }
  async function submit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const f = Object.fromEntries(new FormData(e.currentTarget)) as Record<
      string,
      string
    >;
    await action(async () => {
      if (modal === "stock")
        return invoke("save_stock", {
          code: f.code.toUpperCase(),
          name: f.name,
          sector: f.sector,
        });
      if (modal === "event")
        return invoke("save_event", {
          event: {
            id: edit?.id ?? 0,
            date: f.date,
            kind: f.kind,
            stock: ["DEPOSIT", "WITHDRAW"].includes(f.kind) ? "" : f.stock,
            shares: Number(f.lots || 0) * 100,
            price: f.price || "0",
            fee: f.fee || "0",
            amount: f.amount || "0",
            journal: f.journal,
          },
        });
      if (modal === "backup") return invoke("backup", { path: f.path });
      if (modal === "restore") {
        if (
          !confirm(
            "Pulihkan backup? Database saat ini akan disalin ke lokasi pengaman terlebih dahulu.",
          )
        )
          return;
        return invoke("restore", { path: f.path, safety: f.safety });
      }
      const payload = Object.fromEntries(
        (fields[modal!] ?? []).map((k) => [k, f[k] ?? ""]),
      );
      if (modal === "plan") {
        const pct = ["TP1 alokasi %", "TP2 alokasi %", "TP3 alokasi %"].map(
          (k) => Number(payload[k]),
        );
        if (
          pct.some((n) => !Number.isFinite(n) || n < 0) ||
          pct.reduce((a, b) => a + b, 0) > 100
        )
          throw new Error("Total alokasi TP harus 0–100%");
      }
      return invoke("save_record", {
        record: {
          id: edit?.id ?? 0,
          kind: modal,
          stock: f.stock,
          date: f.date,
          data: payload,
        },
      });
    });
  }
  const ev = edit && "journal" in edit ? edit : null;
  const rec = edit && "data" in edit ? edit : null;
  return (
    <div className="app">
      <aside>
        <div className="brand">
          <span className="logo">A↗</span>
          <div>
            ARDIN SAHAM<small>INVESTMENT WORKSPACE</small>
          </div>
        </div>
        <nav>
          {menus.map((m, i) => (
            <button
              className={m === page ? "selected" : ""}
              key={m}
              onClick={() => {
                setPage(m);
                setStock("");
              }}
            >
              <span>{String(i + 1).padStart(2, "0")}</span>
              {m}
            </button>
          ))}
        </nav>
        <div className="offline">
          ● Offline & pribadi<small>Harga pasar diinput manual</small>
          <small>Versi pengembangan 0.1.0</small>
        </div>
      </aside>
      <main>
        <header>
          <div>
            <p className="eyebrow">
              PERSONAL STOCK PORTFOLIO & INVESTMENT TRACKER
            </p>
            <h1>{page}</h1>
          </div>
          <span className="badge">DATA LOKAL • IDR</span>
        </header>
        {error && (
          <div role="alert" className="error">
            {error}
            {!data && (
              <p>
                Jalankan aplikasi melalui Tauri untuk mengakses database SQLite
                lokal.
              </p>
            )}
          </div>
        )}
        {!data ? (
          <p className="empty">
            {error ? "Database desktop belum terhubung." : "Memuat database…"}
          </p>
        ) : (
          <>
            {(page === "Dashboard" || page === "Performance & Evaluation") && (
              <>
                <div className="cards">
                  <article>
                    <label>Saldo kas</label>
                    <strong>{rupiah(data.ledger.cash)}</strong>
                  </article>
                  <article>
                    <label>Nilai saham manual</label>
                    <strong>
                      {complete ? rupiah(market) : "Harga belum lengkap"}
                    </strong>
                  </article>
                  <article>
                    <label>Realized P/L</label>
                    <strong>{rupiah(data.ledger.realized)}</strong>
                  </article>
                  <article>
                    <label>Unrealized P/L</label>
                    <strong>
                      {complete ? rupiah(market - cost) : "Data belum cukup"}
                    </strong>
                  </article>
                </div>
                <section>
                  <h2>Ringkasan portofolio</h2>
                  <p>
                    Setoran neto & saldo awal: {rupiah(data.ledger.contributed)}{" "}
                    · Dividen diterima: {rupiah(data.ledger.dividends)}
                  </p>
                  <p>
                    Ekuitas:{" "}
                    {complete
                      ? rupiah(market + Number(data.ledger.cash))
                      : "Data harga belum cukup"}{" "}
                    · Saham aktif: {active.length} · Siklus ditutup:{" "}
                    {data.ledger.closed.length}
                  </p>
                  <p>
                    Win rate (siklus posisi ditutup):{" "}
                    {data.ledger.closed.length
                      ? (
                          (data.ledger.closed.filter(
                            (p) => Number(p.realized) > 0,
                          ).length /
                            data.ledger.closed.length) *
                          100
                        ).toFixed(2) + "%"
                      : "Data belum cukup"}
                  </p>
                  <small>
                    Realized P/L memakai weighted average cost termasuk fee.
                    Setoran dan penarikan tidak dihitung sebagai keuntungan.
                    Nilai tampilan dibulatkan ke 2 desimal; ledger dihitung di
                    Rust.
                  </small>
                </section>
                {page === "Performance & Evaluation" &&
                  recordTable("evaluation")}
                {page === "Dashboard" && (
                  <section>
                    <h2>Mulai mencatat investasi</h2>
                    <p>
                      Daftarkan saham, catat setoran dana, lalu masukkan
                      pembelian beserta alasan keputusan. Gunakan saldo awal
                      untuk kepemilikan lama.
                    </p>
                    <div className="toolbar">
                      {button("Tambah saham", "stock")}
                      {button("Catat transaksi / kas", "event")}
                    </div>
                  </section>
                )}
              </>
            )}
            {page === "Portfolio" && (
              <>
                <div className="toolbar">
                  {button("Tambah saham", "stock")}
                  {button("Catat harga manual", "price")}
                  <label>
                    Detail saham{" "}
                    <select
                      value={stock}
                      onChange={(e) => setStock(e.target.value)}
                    >
                      <option value="">Semua saham</option>
                      {data.stocks.map((s) => (
                        <option key={s.code}>{s.code}</option>
                      ))}
                    </select>
                  </label>
                </div>
                <div className="table-scroll">
                  <table>
                    <thead>
                      <tr>
                        {[
                          "Saham",
                          "Siklus",
                          "Lot",
                          "Average cost",
                          "Biaya perolehan",
                          "Harga manual",
                          "Tanggal harga",
                          "Nilai pasar",
                          "Unrealized P/L",
                        ].map((h) => (
                          <th key={h}>{h}</th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {active
                        .filter((p) => !stock || p.stock === stock)
                        .map((p) => (
                          <tr key={p.stock}>
                            <td>
                              <button onClick={() => setStock(p.stock)}>
                                {p.stock}
                              </button>
                            </td>
                            <td>{p.cycle}</td>
                            <td>{p.shares / 100}</td>
                            <td>{rupiah(Number(p.cost) / p.shares)}</td>
                            <td>{rupiah(p.cost)}</td>
                            <td>
                              {price(p)
                                ? rupiah(price(p)!.data.price)
                                : "Belum diisi"}
                            </td>
                            <td>{price(p)?.date ?? "—"}</td>
                            <td>
                              {price(p)
                                ? rupiah(
                                    Number(price(p)!.data.price) * p.shares,
                                  )
                                : "—"}
                            </td>
                            <td>
                              {price(p)
                                ? rupiah(
                                    Number(price(p)!.data.price) * p.shares -
                                      Number(p.cost),
                                  )
                                : "—"}
                            </td>
                          </tr>
                        ))}
                    </tbody>
                  </table>
                </div>
                {!active.length && (
                  <p className="empty">Belum ada posisi aktif.</p>
                )}
                <section>
                  <h2>Posisi ditutup</h2>
                  {data.ledger.closed.map((p) => (
                    <p key={p.stock + p.cycle}>
                      {p.stock} · Siklus {p.cycle} · {p.opened} – {p.closed} ·
                      Realized {rupiah(p.realized)}
                    </p>
                  ))}
                </section>
                {stock && (
                  <>
                    <div className="tabs">
                      {tabs.map((t) => (
                        <button
                          key={t}
                          className={tab === t ? "selected" : ""}
                          onClick={() => setTab(t)}
                        >
                          {t}
                        </button>
                      ))}
                    </div>
                    <section>
                      <h2>
                        {stock} / {tab}
                      </h2>
                      {tab === "Overview" && recordTable("price")}
                      {tab === "Investment Rules" && (
                        <>
                          {screeningPanel()}
                          {recordTable("rules")}
                        </>
                      )}
                      {tab === "Technical & Fundamental" && (
                        <>
                          {recordTable("technical")}
                          {recordTable("fundamental")}
                        </>
                      )}
                      {tab === "Trading Plan" && (
                        <>
                          <p>
                            Rencana dicatat manual. Target tercapai bukan
                            transaksi jual; catat penjualan secara terpisah.
                            Simpan revisi sebagai catatan baru untuk
                            mempertahankan riwayat.
                          </p>
                          {planPanel()}
                          {recordTable("plan")}
                        </>
                      )}
                      {tab === "Catalysts & News" && recordTable("catalyst")}
                      {tab === "Transaction History" && transactionTable()}
                    </section>
                  </>
                )}
              </>
            )}
            {(page === "Transactions" || page === "History & Reports") && (
              <>
                <div className="toolbar">
                  {button("Transaksi / mutasi kas", "event")}
                  <label>
                    Dari
                    <input
                      type="date"
                      value={from}
                      onChange={(e) => setFrom(e.target.value)}
                    />
                  </label>
                  <label>
                    Sampai
                    <input
                      type="date"
                      value={to}
                      onChange={(e) => setTo(e.target.value)}
                    />
                  </label>
                  <button onClick={exportCsv}>Ekspor CSV</button>
                </div>
                {transactionTable()}
              </>
            )}
            {page === "Watchlist" && (
              <>
                <section>
                  <h2>MY INVESTMENT RULES</h2>
                  <ol>
                    {rules.map((r) => (
                      <li key={r}>{r}</li>
                    ))}
                  </ol>
                  <p>
                    Belum diverifikasi berarti belum memenuhi syarat. SSID bukan
                    bukti akumulasi institusional. Semua sumber dan bukti harus
                    diperiksa pengguna.
                  </p>
                </section>
                <div className="toolbar">
                  {button("Tambah saham", "stock")}
                  <input
                    placeholder="Cari kode / perusahaan"
                    value={search}
                    onChange={(e) => setSearch(e.target.value)}
                  />
                  <select
                    value={stock}
                    onChange={(e) => setStock(e.target.value)}
                  >
                    <option value="">Semua saham</option>
                    {data.stocks
                      .filter((s) =>
                        (s.code + s.name)
                          .toLowerCase()
                          .includes(search.toLowerCase()),
                      )
                      .map((s) => (
                        <option key={s.code} value={s.code}>
                          {s.code} — {s.name}
                        </option>
                      ))}
                  </select>
                </div>
                {screeningPanel()}
                {recordTable("watchlist")}
                {recordTable("rules")}
              </>
            )}
            {page === "Dividend Tracker" && (
              <>
                <section>
                  <h2>Dividen diumumkan dan diterima</h2>
                  <p>
                    Catatan jadwal tidak menambah kas. Setelah dana diterima,
                    gunakan tombol Terima dividen. Sistem menghitung lembar pada
                    cum date, memerlukan alasan koreksi manual, dan mencegah
                    penerimaan ganda. Pembatalan penerimaan belum tersedia.
                  </p>
                </section>
                {recordTable("dividend")}
              </>
            )}
            {page === "Settings & Backup" && (
              <section>
                <h2>Database & perlindungan data</h2>
                <p>
                  Database: ardin-saham.db pada direktori data aplikasi Windows.
                  Backup menggunakan SQLite Backup API. Restore memvalidasi
                  integritas, versi, relasi, dan ledger sebelum membuat backup
                  pengaman.
                </p>
                <div className="toolbar">
                  {button("Backup database", "backup")}
                  {button("Restore database", "restore")}
                </div>
                <p>
                  Simpan backup pada perangkat terpisah secara berkala. Tutup
                  aplikasi sebelum memindahkan data secara manual. Tidak ada
                  akun, layanan harga, atau telemetry.
                </p>
              </section>
            )}
          </>
        )}
        {modal && (
          <div className="overlay">
            <form className="modal" onSubmit={submit}>
              <h2>
                {edit ? "Edit" : "Tambah"} {modal}
              </h2>
              <button
                type="button"
                className="close"
                onClick={() => {
                  setModal(null);
                  setEdit(null);
                }}
              >
                Tutup
              </button>
              {error && (
                <p role="alert" className="error">
                  {error}
                </p>
              )}
              {modal === "stock" ? (
                <>
                  {["code", "name", "sector"].map((k) => (
                    <label key={k}>
                      {k}
                      <input name={k} required />
                    </label>
                  ))}
                </>
              ) : modal === "backup" || modal === "restore" ? (
                <>
                  <label>
                    Lokasi lengkap file backup
                    <input
                      name="path"
                      required
                      placeholder="C:\Users\Nama\Documents\ardin-backup.db"
                    />
                  </label>
                  {modal === "restore" && (
                    <label>
                      Lokasi backup pengaman baru
                      <input
                        name="safety"
                        required
                        placeholder="C:\Users\Nama\Documents\sebelum-restore.db"
                      />
                    </label>
                  )}
                </>
              ) : (
                <>
                  <div className="form-grid">
                    <label>
                      Tanggal
                      <input
                        type="date"
                        name="date"
                        defaultValue={edit?.date ?? today()}
                        required
                      />
                    </label>
                    <label>
                      Saham
                      <select
                        name="stock"
                        defaultValue={edit?.stock ?? stock}
                        required={modal !== "event"}
                      >
                        <option value="">Pilih saham / mutasi kas</option>
                        {data?.stocks.map((s) => (
                          <option key={s.code}>{s.code}</option>
                        ))}
                      </select>
                    </label>
                  </div>
                  {modal === "event" ? (
                    <>
                      <label>
                        Jenis
                        <select name="kind" defaultValue={ev?.kind ?? "BUY"}>
                          {[
                            "BUY",
                            "SELL",
                            "DEPOSIT",
                            "WITHDRAW",
                            "OPENING",
                            "DIVIDEND",
                          ].map((k) => (
                            <option key={k}>{k}</option>
                          ))}
                        </select>
                      </label>
                      <div className="form-grid">
                        <label>
                          Jumlah lot
                          <input
                            name="lots"
                            type="number"
                            min="0"
                            step="1"
                            defaultValue={ev ? ev.shares / 100 : 0}
                          />
                        </label>
                        <label>
                          Harga per lembar (Rp)
                          <input
                            name="price"
                            type="number"
                            min="0"
                            step="0.01"
                            defaultValue={ev?.price ?? "0"}
                          />
                        </label>
                        <label>
                          Fee total (Rp)
                          <input
                            name="fee"
                            type="number"
                            min="0"
                            step="0.01"
                            defaultValue={ev?.fee ?? "0"}
                          />
                        </label>
                        <label>
                          Nominal setoran / penarikan / dividen bersih
                          <input
                            name="amount"
                            type="number"
                            min="0"
                            step="0.01"
                            defaultValue={ev?.amount ?? "0"}
                          />
                        </label>
                      </div>
                      <label>
                        Jurnal keputusan & strategi
                        <textarea
                          name="journal"
                          rows={6}
                          defaultValue={ev?.journal ?? ""}
                          placeholder="Alasan beli/jual, fundamental, teknikal, support, katalis, risiko, target, batas rugi, keyakinan, strategi, kepatuhan rencana dan emosi. Dapat dilengkapi nanti."
                        />
                      </label>
                    </>
                  ) : (
                    <div className="form-grid">
                      {(fields[modal] ?? []).map((k) => (
                        <label key={k}>
                          {k}
                          {k ===
                          "Status (Terpenuhi / Tidak terpenuhi / Belum diverifikasi)" ? (
                            <select
                              name={k}
                              defaultValue={
                                rec?.data[k] ?? "Belum diverifikasi"
                              }
                            >
                              {[
                                "Belum diverifikasi",
                                "Terpenuhi",
                                "Tidak terpenuhi",
                              ].map((v) => (
                                <option key={v}>{v}</option>
                              ))}
                            </select>
                          ) : k === "Aturan (1–9)" ? (
                            <select name={k} defaultValue={rec?.data[k] ?? "1"}>
                              {rules.map((v, i) => (
                                <option key={v} value={i + 1}>
                                  {i + 1}. {v}
                                </option>
                              ))}
                            </select>
                          ) : (
                            <input
                              name={k}
                              defaultValue={
                                rec?.data[k] ??
                                (k === "Fee jual (bps)" ? "25" : "")
                              }
                            />
                          )}
                        </label>
                      ))}
                    </div>
                  )}
                </>
              )}
              <button className="primary" disabled={busy}>
                {busy ? "Menyimpan…" : "Simpan"}
              </button>
            </form>
          </div>
        )}
      </main>
    </div>
  );
}
createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
