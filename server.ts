// server.ts
import { createServer } from "node:http";
import { readFile, readdir } from "node:fs/promises";
import { resolve, join } from "node:path";

const criterionRoot = resolve("target/criterion/radix");
const IGNORED_DIRS = new Set(["report", "base", "change"]);
const PROFILE_PATTERN = /profile_(\d+)/;
const ITEMS_PATTERN = /(\d+)_items?/;
const BUCKET_PATTERN = /buckets?_(\d+)|(\d+)_buckets?/;

interface BenchmarkResult {
  algorithm: string;
  profile: number;
  itemCount: number;
  buckets: number | null;
  meanNs: number;
  ciLowerNs: number;
  ciUpperNs: number;
}

interface CriterionEstimate {
  mean: {
    point_estimate: number;
    confidence_interval: {
      lower_bound: number;
      upper_bound: number;
    };
  };
}

function parseSegments(segments: string[]) {
  const path = segments.join("/");
  const profileMatch = path.match(PROFILE_PATTERN);
  const itemsMatch = path.match(ITEMS_PATTERN);
  if (!profileMatch || !itemsMatch) return null;
  const bucketMatch = path.match(BUCKET_PATTERN);
  const algorithm = segments
    .map((segment) =>
      segment
        .replace(/_?profile_\d+/, "")
        .replace(/_?(?:buckets?_\d+|\d+_buckets?)/, "")
        .replace(/_?\d+_items?/, "")
        .replace(/^_+|_+$/g, ""),
    )
    .filter(Boolean)
    .join("/");
  if (!algorithm) return null;
  return {
    algorithm,
    profile: Number(profileMatch[1]),
    itemCount: Number(itemsMatch[1]),
    buckets: bucketMatch ? Number(bucketMatch[1] ?? bucketMatch[2]) : null,
  };
}

async function walk(
  dir: string,
  segments: string[],
  results: BenchmarkResult[],
): Promise<void> {
  const entries = await readdir(dir, { withFileTypes: true });
  if (entries.some((entry) => entry.isDirectory() && entry.name === "new")) {
    const parsed = parseSegments(segments);
    if (!parsed) return;
    try {
      const content = await readFile(
        join(dir, "new", "estimates.json"),
        "utf8",
      );
      const estimate = JSON.parse(content) as CriterionEstimate;
      results.push({
        ...parsed,
        meanNs: estimate.mean.point_estimate,
        ciLowerNs: estimate.mean.confidence_interval.lower_bound,
        ciUpperNs: estimate.mean.confidence_interval.upper_bound,
      });
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    return;
  }
  for (const entry of entries) {
    if (!entry.isDirectory() || IGNORED_DIRS.has(entry.name)) continue;
    await walk(join(dir, entry.name), [...segments, entry.name], results);
  }
}

async function collectResults() {
  const results: BenchmarkResult[] = [];
  await walk(criterionRoot, [], results);
  return {
    algorithms: [...new Set(results.map((r) => r.algorithm))].sort(),
    profiles: [...new Set(results.map((r) => r.profile))].sort((a, b) => a - b),
    buckets: [
      ...new Set(
        results.map((r) => r.buckets).filter((b): b is number => b !== null),
      ),
    ].sort((a, b) => a - b),
    results,
  };
}

const server = createServer(async (request, response) => {
  if (request.url !== "/api/criterion/results") {
    response.writeHead(404);
    response.end();
    return;
  }
  try {
    const data = await collectResults();
    response.writeHead(200, { "Content-Type": "application/json" });
    response.end(JSON.stringify(data));
  } catch (error) {
    response.writeHead(500, { "Content-Type": "application/json" });
    response.end(JSON.stringify({ error: String(error) }));
  }
});
server.listen(3000, () => {
  console.log("Criterion API: http://localhost:3000");
});