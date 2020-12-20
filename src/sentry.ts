import * as Sentry from "@sentry/browser";

Sentry.init({
  dsn: "https://797f518de06a45f2b671d95ba8465330@o445771.ingest.sentry.io/5422971",
  release: process.env.SOURCE_COMMIT,
});
