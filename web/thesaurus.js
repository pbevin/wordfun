import React, { useState, useEffect, useRef } from "react";
import PropTypes from "prop-types";

import isBlank from "./isblank";

function Thesaurus() {
  const [query, setQuery] = useState("");
  const handleSubmit = (e) => {
    e.preventDefault();
  };

  return (
    <section id="sect-thesaurus">
      <h2>
        <label htmlFor="th">Thesaurus</label>
      </h2>
      <div className="help info">
        <p>Enter a word to get synonyms. For example,</p>
        <ul>
          <li>
            "17. May be tuna sandwiches always hot (8)" (Financial Times 16,569) suggests "fish" (a
            synonym of "tuna") enveloping "ever" (a synonym of "always"), giving{" "}
            <samp>FEVERISH</samp>.
          </li>
        </ul>
      </div>

      <form onSubmit={handleSubmit}>
        <div className="tool">
          <input
            autoCapitalize="off"
            autoCorrect="off"
            type="text"
            id="th"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <button>Thesaurus</button>
        </div>
        <ThesaurusPreview query={query} onSearch={setQuery} />
      </form>
    </section>
  );
}

Thesaurus.propTypes = {};

function ThesaurusPreview(props) {
  const query = props.query.trim();
  const [result, setResult] = useState(null);
  const inflight = useRef();

  useEffect(() => {
    inflight.current = query;
    if (isBlank(query)) {
      setResult(null);
    } else {
      fetch(`/preview/thesaurus?q=${query}`)
        .then((response) => response.json())
        .then((preview) => {
          if (inflight.current === query) {
            setResult(preview);
          }
        });
    }
  }, [query]);

  if (!result) {
    return null;
  } else if (result.words.length === 0) {
    return <div className="thesaurus-preview">{query}: No matches.</div>;
  }

  return (
    <ul className="thesaurus-preview">
      <li>
        {query}: {result.count}
      </li>
      {result.words.map(([len, words]) => (
        <li key={len}>
          <ThesaurusGroup len={len} words={words} onSearch={props.onSearch} />
        </li>
      ))}
    </ul>
  );
}

ThesaurusPreview.propTypes = {
  query: PropTypes.string,
  onSearch: PropTypes.func.isRequired,
};

function ThesaurusGroup(props) {
  const handleSearch = (word) => (e) => {
    e.preventDefault();
    props.onSearch(word);
  };
  const wordList = props.words.map((word) => (
    <a key={word} href={`#/thesaurus/${word}`} onClick={handleSearch(word)}>
      {word}
    </a>
  ));
  const words = wordList.map((word, i) => [i > 0 && ", ", word]);
  return (
    <>
      {props.len}: {words}
    </>
  );
}

ThesaurusGroup.propTypes = {
  len: PropTypes.string.isRequired,
  words: PropTypes.arrayOf(PropTypes.string.isRequired).isRequired,
  onSearch: PropTypes.func.isRequired,
};

export default Thesaurus;
