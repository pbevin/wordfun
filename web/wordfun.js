import React, { useState, useEffect, useRef } from "react";
import PropTypes from "prop-types";
import Modal from "react-modal";

import Thesaurus from "./thesaurus";
import isBlank from "./isblank";
import checkVersion from "./check-version";

import closeIcon from "./icons/close.svg";

function Wordfun(props) {
  const [query, setQuery] = useState(null);

  useEffect(() => {
    async function check() {
      const result = await checkVersion();
      if (result.mismatched) {
        await fetch(`/version.txt?ui=${process.env.COMMIT_ID}&server=${result.serverVersion}`);
        console.warn("Running a different version from the server");
      }
    }

    check();
  }, []);

  return (
    <>
      <IntroOrResults intro={props.intro} query={query} />
      <Anagram onSubmit={(q) => setQuery({ type: "an", q })} />
      <FindWord onSubmit={(q) => setQuery({ type: "fw", q })} />
      <Thesaurus />
    </>
  );
}
Wordfun.propTypes = {
  intro: PropTypes.string.isRequired,
};

async function fetchQuery(path, query, inflight, setResult) {
  let response = await fetch(`${path}?q=${encodeURIComponent(query)}`);
  let results = await response.json();
  if (inflight.current.q === query) {
    setResult(results);
  }
}

function IntroOrResults(props) {
  const query = props.query;
  const [result, setResult] = useState();
  const inflight = useRef();

  useEffect(() => {
    inflight.current = query;
    if (!query) {
      setResult(null);
    } else {
      fetchQuery(`/words/${query.type}`, query.q, inflight, setResult);
    }
  }, [query]);

  if (!result) {
    return <section className="intro" dangerouslySetInnerHTML={{ __html: props.intro }}></section>;
  } else if (result.full_count === 0) {
    return <section className="results">{query}: No matches.</section>;
  }
  if (result) {
    return (
      <section>
        <h1>
          Results for <em>{query.q}</em>
        </h1>
        <FullResults words={result.words} />
      </section>
    );
  }
}

IntroOrResults.propTypes = {
  intro: PropTypes.string.isRequired,
  query: PropTypes.exact({
    q: PropTypes.string.isRequired,
    type: PropTypes.oneOf(["an", "fw"]),
  }),
};

function Anagram(props) {
  const [query, setQuery] = useState("");
  const input = useRef();

  const handleSubmit = (e) => {
    e.preventDefault();
    props.onSubmit(query);
  };

  const clearInput = () => {
    setQuery("");
    input.current.focus();
  };

  return (
    <section id="sect-an">
      <h2>
        <label htmlFor="an">Find an Anagram</label>
      </h2>
      <div className="help info">
        <p>
          Type in a word or series of words here to get valid words and phrases from the dictionary.
          For example,
        </p>
        <ul>
          <li>
            “Pioneering tsar’s exotic voyages (14)” (Guardian 28,233): searching for “pioneering
            tsar” gives the answer <samp>PEREGRINATIONS</samp>
          </li>
          <li>
            From the same puzzle, “State capital one missed when touring (3,6)” suggests searching
            for “one missed”, which again gives the right answer <samp>DES MOINES</samp>.
          </li>
          <li>
            You can find partial anagrams, too: just pad out the search with dots. For example, “Set
            up an unusual lab aboard vessel in Devonport, perhaps (5,4)” (Guardian Quiptic 1,087)
            has “lab”, and you might guess that “vessel” is “vase”. That just needs two more
            letters, and searching for <kbd>labvase..</kbd> yields 5 results, one of which is the
            correct answer, <samp>NAVAL BASE</samp>.
          </li>
        </ul>
      </div>

      <form onSubmit={handleSubmit}>
        <div className="tool">
          <input
            id="an"
            autoCapitalize="off"
            autoCorrect="off"
            autoComplete="off"
            autoFocus={true}
            ref={input}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <button type="submit" className="btn">
            Anagram
          </button>
          <button type="button" onClick={clearInput} className="btn-clear">
            Clear
          </button>
        </div>
        <Preview query={query} type="an" />
      </form>
    </section>
  );
}
Anagram.propTypes = {
  onSubmit: PropTypes.func.isRequired,
};

function FindWord(props) {
  const [query, setQuery] = useState("");
  const input = useRef();

  const handleSubmit = (e) => {
    e.preventDefault();
    props.onSubmit(query);
  };

  const clearInput = () => {
    setQuery("");
    input.current.focus();
  };

  return (
    <section id="sect-fw">
      <h2>
        <label htmlFor="fw">Complete a Word or Phrase</label>
      </h2>
      <div className="help info">
        <p>
          Type in what you have, with dots for the missing letters (e.g., <kbd>h.r...i.m</kbd>) to
          get matching words and phrases from the dictionary. You can match word boundaries with
          forward slashes, like this: <kbd>h.r./...l../e.g</kbd>
        </p>
      </div>
      <form onSubmit={handleSubmit}>
        <div className="tool">
          <input
            autoCapitalize="off"
            autoCorrect="off"
            autoComplete="off"
            ref={input}
            name="fw"
            id="fw"
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <button type="submit" className="btn">
            Find Word
          </button>
          <button type="button" onClick={clearInput} className="btn-clear">
            Clear
          </button>
        </div>
        <Preview query={query} type="fw" />
      </form>
    </section>
  );
}
FindWord.propTypes = {
  onSubmit: PropTypes.func.isRequired,
};

function Preview(props) {
  const query = props.query.trim();
  const [result, setResult] = useState();
  const inflight = useRef();

  useEffect(() => {
    inflight.current = query;
    if (isBlank(query)) {
      setResult(null);
    } else {
      fetch(`/preview/${props.type}?q=${query}`)
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
  } else if (result.full_count === 0) {
    return <div className="preview">{query}: No matches.</div>;
  } else {
    let count;
    if (result.full_count == 1) {
      count = "1 result";
    } else {
      count = `${result.full_count} results`;
    }
    let wordList = result.words.map((w) => {
      if (w.ranked) {
        return <strong key={w.text}>{w.text}</strong>;
      } else {
        return <span key={w.text}>{w.text}</span>;
      }
    });
    if (wordList.length < result.full_count) {
      wordList.push("...");
    }

    const words = wordList.map((word, i) => [i > 0 && ", ", word]);

    return (
      <div className="preview">
        {query} ({result.lengths}): {count} ({words})
      </div>
    );
  }
}

function FullResults(props) {
  const [modalIsOpen, setIsOpen] = React.useState(false);
  const [modalUrl, setUrl] = React.useState(null);

  function closeModal() {
    setIsOpen(false);
  }

  function openUrl(url) {
    setUrl(url);
    setIsOpen(true);
  }

  return (
    <div className="results">
      {props.words.map((e) => (
        <Entry key={e.word} onClick={openUrl} {...e} />
      ))}
      <LinkModal isOpen={modalIsOpen} url={modalUrl} onCloseRequest={closeModal} />
    </div>
  );
}

FullResults.propTypes = {
  words: PropTypes.arrayOf(
    PropTypes.exact({
      word: PropTypes.string.isRequired,
      score: PropTypes.number,
      definition: PropTypes.string,
    })
  ),
};

function Entry({ word, score, definition, onClick }) {
  const classes = score > 0 ? "entry good" : "entry";
  const dictUrl = "https://www.thefreedictionary.com/" + encodeURIComponent(word);
  function handleClick(e) {
    e.preventDefault();
    onClick(dictUrl);
  }

  return (
    <entry className={classes}>
      <a className="word" href={dictUrl} target="wf_lookup" onClick={handleClick}>
        {word}
      </a>
      <dfn className="dfn">{definition}</dfn>
    </entry>
  );
}

Entry.propTypes = {
  word: PropTypes.string,
  score: PropTypes.number,
  definition: PropTypes.string,
  onClick: PropTypes.func.isRequired,
};

// This is the popup that loads a definition from thefreedictionary.com.
function LinkModal(props) {
  useEffect(() => {
    function handleKeyUp(e) {
      if (e.keyCode == 27) {
        props.onCloseRequest();
      }
    }

    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, []);

  return (
    <Modal
      isOpen={props.isOpen}
      onRequestClose={props.onCloseRequest}
      className="modal"
      overlayClassName="overlay"
      contentLabel="Example Modal"
    >
      <button className="modal-close" onClick={props.onCloseRequest}>
        <img src={closeIcon} width="32" />
      </button>
      <iframe className="dictionary-iframe" src={props.url}></iframe>
    </Modal>
  );
}

LinkModal.propTypes = {
  isOpen: PropTypes.bool.isRequired,
  url: PropTypes.string,
  onCloseRequest: PropTypes.func.isRequired,
};

export default Wordfun;
