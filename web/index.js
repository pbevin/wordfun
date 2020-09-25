import ReactDOM from "react-dom";
import React from "react";
import Modal from "react-modal";
import * as Sentry from "@sentry/react";

// workaround for "regeneratorRuntime not defined":
import "regenerator-runtime/runtime";

import Wordfun from "./wordfun";
const App = Sentry.withErrorBoundary(Wordfun, { fallback: "an error occurred" });

import "whatwg-fetch";
import "./sentry.js";
import "./app.css";

const elt = document.getElementById("app");
Modal.setAppElement(elt);
const intro = elt.innerHTML;
ReactDOM.render(<App intro={intro} />, elt, () => {
  // The #app element is initially declared with class `uninit`,
  // which gives it enough height that the footer doesn't obviously
  // jump around during page load.  Clearing the class here turns
  // off the height style so that the app can be its natural height.
  elt.className = "";
});
