import fs from "node:fs";
import path from "node:path";
import { expect, test } from "@playwright/test";

const releaseScreenshots = Boolean(process.env.DECOMPROOF_RELEASE_SCREENSHOTS);

test.skip(!releaseScreenshots, "release screenshot capture runs only in release validation");

test("captures real dashboard views backed by generated proof data", async ({ page }) => {
  const output = path.join(process.cwd(), "release-screenshots");
  fs.mkdirSync(output, { recursive: true });

  const views = [
    { name: "overview", route: "/", readyText: "Readiness" },
    { name: "evidence", route: "/evidence", readyText: "Evidence matrix" },
    { name: "graph", route: "/graph", readyText: "Dependency graph" },
    { name: "proof", route: "/proof", readyText: "Proof artifact viewer" },
  ] as const;

  for (const { name, route, readyText } of views) {
    await page.goto(route);
    await expect(page.getByText("DecomProof", { exact: true })).toBeVisible();
    await expect(page.getByText(readyText, { exact: true }).first()).toBeVisible();
    await expect(page.locator("main")).not.toContainText(
      /No proof data yet|API unavailable|Resolving latest proof|Loading evidence/i,
    );
    await page.screenshot({ path: path.join(output, `${name}.png`), fullPage: true });
  }
});
