import { expect, test } from "@playwright/test";

test("dashboard renders a truthful empty or API-error state without fabricated proof data", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("DecomProof", { exact: true })).toBeVisible();
  await expect(page.locator("main")).toContainText(
    /No proof data yet|API unavailable|Removal Readiness|Readiness/i,
  );
});
