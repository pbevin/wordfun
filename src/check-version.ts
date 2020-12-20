import * as Sentry from '@sentry/browser';

type VersionInfo = {
  serverVersion: string | null;
  serverResponse: string | null;
  uiVersion: string | null;
  mismatched: boolean;
};

// For production, the build process supplies SOURCE_COMMIT; in development, it's
// likely to be null unless you've set it explicitly.  Either way, the bundler
// substitutes a static value.
const UI_VERSION = process.env.SOURCE_COMMIT;
const VERSION_REGEXP = /^[0-9a-f]{40}$/i;

// Ask the server for its version number. If they don't match, we are probably
// using an old cached version of the UI.
//
// There are various cases:
//   1. The HTTP request fails. It's safe to ignore this; it may be
//      a transient error anyway.
//   2. The response is something other than a version number. That would
//      be a server error, and it's none of the UI's business.
//   3. The response is a version number that's the same as ours. That's
//      great! Keep going!
//   4. The response is a different version number. That presumably means that
//      the server is running a newer version, and we should update.
export default async function checkVersion(): Promise<VersionInfo> {
  if (!UI_VERSION) {
    return {
      serverVersion: null,
      serverResponse: null,
      uiVersion: null,
      mismatched: false,
    };
  }
  // We pass our own UI_VERSION just as an FYI. The server doesn't do anything
  // with it, but it will appear in log files, and it will be interesting to see
  // how long old versions stick around.
  //
  // The `cache` argument means "Don't query a cache before requesting, and
  // don't store the result in cache".  We need to be sure that we're getting
  // the live result. If the version number came from a cache, this test would
  // be useless.
  try {
    const resp = await fetch(`/version.txt?ui=${UI_VERSION}`, {
      cache: 'no-store',
    });
    const text = await resp.text();

    let response: VersionInfo = {
      serverVersion: null,
      serverResponse: text,
      uiVersion: UI_VERSION,
      mismatched: false,
    };
    if (!text.match(VERSION_REGEXP)) {
      // Case 2: got something other than a version number.
      response.mismatched = false;
    } else if (text === UI_VERSION) {
      // Case 3: the server is running the same version as we are!
      response.serverVersion = text;
      response.mismatched = false;
    } else {
      // Case 4: the server has returned a reasonable looking version
      // number, but it's not what we have.
      response.serverVersion = text;
      response.mismatched = true;
    }
    return response;
  } catch (e) {
    Sentry.captureException(e);
    throw e;
  }
}
