// Test-only bridge to the real Rust/SQLite core. Never shipped with the application.
import http from "node:http";
import { spawn } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
const dir = mkdtempSync(join(tmpdir(), "ardin-e2e-"));
const server = http.createServer((req, res) => {
  res.setHeader("Access-Control-Allow-Origin", "http://127.0.0.1:5173");
  res.setHeader("Access-Control-Allow-Headers", "content-type");
  if (req.method === "OPTIONS") {
    res.end();
    return;
  }
  if (req.url === "/health") {
    res.end("ok");
    return;
  }
  if (req.method !== "POST" || req.url !== "/rpc") {
    res.writeHead(404).end();
    return;
  }
  let body = "";
  req.on("data", (c) => (body += c));
  req.on("end", () => {
    const child = spawn(
      resolve(
        "core/target/debug/examples/test_rpc" +
          (process.platform === "win32" ? ".exe" : ""),
      ),
      [],
      { env: { ...process.env, ARDIN_TEST_DB: join(dir, "test.db") } },
    );
    let out = "";
    child.stdout.on("data", (c) => (out += c));
    child.on("error", (e) =>
      res.writeHead(500).end(JSON.stringify({ error: e.message })),
    );
    child.on("close", () => {
      if (!res.writableEnded) {
        res.setHeader("Content-Type", "application/json");
        res.end(out);
      }
    });
    child.stdin.end(body);
  });
});
server.listen(4319, "127.0.0.1");
process.on("SIGTERM", () => {
  server.close();
  rmSync(dir, { recursive: true, force: true });
  process.exit();
});
