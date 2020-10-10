import ReactDOM from 'react-dom';
import React from 'react';
import Modal from 'react-modal';
import * as Sentry from '@sentry/react';

// workaround for "regeneratorRuntime not defined":
import 'regenerator-runtime/runtime';

import Wordfun from './wordfun';
import checkVersion from './check-version';

import 'whatwg-fetch';
import './sentry';
import './app.css';

const App = Sentry.withErrorBoundary(Wordfun, {fallback: 'an error occurred'});

const elt = document.getElementById('root');
checkServerVersion();
if (elt) {
  Modal.setAppElement(elt);
  ReactDOM.render(<App />, elt, () => {
    // The #app element is initially declared with class `uninit`,
    // which gives it enough height that the footer doesn't obviously
    // jump around during page load.  Clearing the class here turns
    // off the height style so that the app can be its natural height.
    elt.className = '';
  });
}

async function checkServerVersion() {
  const result = await checkVersion();
  if (result.mismatched) {
    await fetch(
      `/version.txt?ui=${process.env.COMMIT_ID}&server=${result.serverVersion}`,
    );
    console.warn('Running a different version from the server');
  }
}
