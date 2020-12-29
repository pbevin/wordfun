import React from "react";
import infoIcon from "./icons/info.svg";

export default function Intro() {
  return (
    <section className="intro">
      <div id="intro">
        <h1>Crossword Solver</h1>
        <p>
          I enjoy crosswords, and wrote these tools to help me. There are well over a quarter of a
          million words and phrases in the dictionary, thanks to{" "}
          <a href="http://www.crosswordman.com/">Ross Beresford</a>'s UKACD dictionary and a huge
          list of phrases from Ross Withey.
        </p>
      </div>
    </section>
  );
}
