import { test, expect } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

// Serve packaged assets in-memory; invoke real Rust in a separate process per
// command. No HTTP bridge: browser stays offline, as native IPC would allow.
test("offline production assets: journal edit, watchlist, statistics and reopen", async ({
  browser,
}) => {
  const dir = mkdtempSync(join(tmpdir(), "ardin-offline-"));
  const context = await browser.newContext({ offline: true });
  const rpc = (cmd: string, args = {}) => {
    const result = JSON.parse(
      execFileSync(
        resolve(
          "core/target/debug/examples/test_rpc" +
            (process.platform === "win32" ? ".exe" : ""),
        ),
        {
          input: JSON.stringify({ cmd, args }),
          encoding: "utf8",
          env: { ...process.env, ARDIN_TEST_DB: join(dir, "local.db") },
        },
      ),
    );
    if (result.error) throw new Error(result.error);
    return result.ok;
  };
  try {
    const external: string[] = [];
    await context.route("**/*", async (route) => {
      const url = new URL(route.request().url());
      if (url.origin !== "http://ardin.local") {
        external.push(url.href);
        await route.abort();
        return;
      }
      const file = url.pathname === "/" ? "index.html" : url.pathname.slice(1);
      if (!/^(index\.html|assets\/[\w.-]+\.(js|css))$/.test(file)) {
        await route.abort();
        return;
      }
      await route.fulfill({
        body: readFileSync(resolve("dist", file)),
        contentType: file.endsWith(".js")
          ? "text/javascript"
          : file.endsWith(".css")
            ? "text/css"
            : "text/html",
      });
    });
    await context.exposeBinding("offlineInvoke", (_source, cmd, args) =>
      rpc(cmd, args),
    );
    await context.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: (cmd: string, args: unknown) =>
          (window as any).offlineInvoke(cmd, args),
      };
    });
    rpc("save_stock", {
      code: "BBCA",
      name: "Bank Central Asia",
      sector: "Bank",
    });
    rpc("save_record", {
      record: {
        id: 0,
        kind: "watchlist",
        stock: "BBCA",
        date: "2026-01-01",
        data: { Status: "Dipantau", Catatan: "Tunggu support" },
      },
    });
    for (const [date, kind, shares, price, amount] of [
      ["2026-01-01", "DEPOSIT", 0, "0", "1000000"],
      ["2026-01-02", "BUY", 200, "1000", "0"],
      ["2026-01-03", "SELL", 200, "1100", "0"],
    ])
      rpc("save_event", {
        event: {
          id: 0,
          date,
          kind,
          stock: kind === "DEPOSIT" ? "" : "BBCA",
          shares,
          price,
          amount,
          fee: "0",
          journal: "Rencana awal",
        },
      });
    let page = await context.newPage();
    await page.goto("http://ardin.local/");
    expect(await page.evaluate(() => navigator.onLine)).toBe(false);
    await page.getByRole("button", { name: /03\s*Transactions/ }).click();
    const buy = page.getByRole("row").filter({ hasText: "BUY" });
    await buy.getByRole("button", { name: "Edit", exact: true }).click();
    await page
      .getByLabel("Jurnal keputusan & strategi")
      .fill("Beli bertahap\nRisiko telah ditinjau");
    await page.getByRole("button", { name: "Simpan", exact: true }).click();
    await expect(buy).toContainText("Risiko telah ditinjau");
    await page.close();
    page = await context.newPage();
    await page.goto("http://ardin.local/");
    await page.getByRole("button", { name: /03\s*Transactions/ }).click();
    await expect(
      page.getByRole("row").filter({ hasText: "BUY" }),
    ).toContainText("Risiko telah ditinjau");
    await page.getByRole("button", { name: /05\s*Performance/ }).click();
    await expect(page.getByLabel("Statistik siklus")).toContainText(
      "Menang: 1 · Kalah: 0 · Impas: 0",
    );
    await expect(page.getByLabel("Statistik siklus")).toContainText("20.000");
    await page.getByRole("button", { name: /04\s*Watchlist/ }).click();
    await expect(
      page.getByText("Tunggu support", { exact: false }),
    ).toBeVisible();
    expect(external).toEqual([]);
  } finally {
    await context.close();
    rmSync(dir, { recursive: true, force: true });
  }
});
