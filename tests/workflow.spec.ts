import { test, expect } from "@playwright/test";
test("frontend + Rust + SQLite: buy, oversell rejection, reload and rules", async ({
  page,
}) => {
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: unknown) => {
        const r = await fetch("http://127.0.0.1:4319/rpc", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ cmd, args }),
        });
        const data = await r.json();
        if (data.error) throw data.error;
        return data.ok;
      },
    };
  });
  await page.goto("/");
  await expect(page.getByText("Saldo kas", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Tambah saham", exact: true }).click();
  await page.getByLabel("code", { exact: true }).fill("NCKL");
  await page.getByLabel("name", { exact: true }).fill("Trimegah");
  await page.getByLabel("sector", { exact: true }).fill("Komoditas");
  await page.getByRole("button", { name: "Simpan", exact: true }).click();
  async function event(
    kind: string,
    lots: string,
    price: string,
    amount: string,
  ) {
    await page.getByRole("button", { name: "Catat transaksi / kas" }).click();
    await page.locator('select[name="kind"]').selectOption(kind);
    if (kind !== "DEPOSIT")
      await page.locator('select[name="stock"]').selectOption("NCKL");
    await page.getByLabel("Jumlah lot", { exact: true }).fill(lots);
    await page.getByLabel("Harga per lembar (Rp)", { exact: true }).fill(price);
    await page
      .getByLabel("Nominal setoran / penarikan / dividen bersih")
      .fill(amount);
    await page.getByRole("button", { name: "Simpan", exact: true }).click();
  }
  await event("DEPOSIT", "0", "0", "100000000");
  await expect(page.locator(".modal")).toHaveCount(0);
  await event("BUY", "100", "1000", "0");
  await expect(page.locator(".modal")).toHaveCount(0);
  await event("BUY", "100", "900", "0");
  await expect(page.locator(".modal")).toHaveCount(0);
  await event("SELL", "201", "1100", "0");
  await expect(page.locator('.modal [role="alert"]')).toContainText(
    "Overselling",
  );
  await page.getByRole("button", { name: "Tutup", exact: true }).click();
  await page.reload();
  await page.getByRole("button", { name: /02\s*Portfolio/ }).click();
  await expect(
    page.getByRole("cell", { name: /^Rp\s950(?:,00)?$/ }),
  ).toBeVisible();
  await page.getByRole("button", { name: /04\s*Watchlist/ }).click();
  await page.locator("main select").selectOption("NCKL");
  await expect(
    page.getByText("Syarat wajib belum terpenuhi / belum diverifikasi"),
  ).toBeVisible();
  await page.getByRole("button", { name: /02\s*Portfolio/ }).click();
  await page.locator("main select").selectOption("NCKL");
  await page.getByRole("button", { name: "Trading Plan", exact: true }).click();
  await page
    .getByRole("button", { name: "Tambah catatan", exact: true })
    .click();
  for (const [name, value] of Object.entries({
    "TP1 harga": "1100",
    "TP2 harga": "1200",
    "TP3 harga": "1350",
    "TP1 alokasi %": "25",
    "TP2 alokasi %": "35",
    "TP3 alokasi %": "40",
    "Stop-loss": "800",
  }))
    await page.locator(`input[name="${name}"]`).fill(value);
  await page.getByRole("button", { name: "Simpan", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Estimasi TP dari posisi saat ini" }),
  ).toBeVisible();
  await expect(
    page.getByRole("row").filter({ hasText: "TP1" }).first(),
  ).toContainText("50");
  await page.getByRole("button", { name: /06\s*Dividend Tracker/ }).click();
  await page
    .getByRole("button", { name: "Tambah catatan", exact: true })
    .click();
  await page.locator('select[name="stock"]').selectOption("NCKL");
  const day = await page.locator('input[name="date"]').inputValue();
  for (const [name, value] of Object.entries({
    "Cum date": day,
    "Payment date": day,
    "Dividend per share": "10",
    Potongan: "0",
  }))
    await page.locator(`input[name="${name}"]`).fill(value);
  await page.getByRole("button", { name: "Simpan", exact: true }).click();
  await expect(page.locator(".modal")).toHaveCount(0);
  page.once("dialog", (dialog) => dialog.accept());
  await page
    .getByRole("button", { name: "Terima dividen", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Terima dividen", exact: true }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("cell").filter({ hasText: "Dividen bersih:" }),
  ).toContainText("200000");
  await page.getByRole("button", { name: /08\s*Settings & Backup/ }).click();
  await expect(
    page.getByRole("button", { name: "Backup database", exact: true }),
  ).toBeVisible();
});
