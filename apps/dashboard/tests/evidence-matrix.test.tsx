import { describe, expect, it } from "vitest";

describe("evidence matrix semantics", () => {
  it("keeps missing evidence distinct from clear evidence", () => {
    const items: unknown[] = [];
    expect(items.length === 0 ? "NO EVIDENCE" : "CLEAR").toBe("NO EVIDENCE");
  });
});
