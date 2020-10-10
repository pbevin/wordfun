import React, {useState, useEffect, useRef, FormEvent, MouseEvent} from 'react';
import PropTypes from 'prop-types';
import Modal from 'react-modal';

import Footer from './footer';
import {pluralize} from './inflections';
import fetchPreview from './preview';
import Intro from './intro';

import closeIcon from './icons/close.svg';

// We treat Anagram and FindWord mostly the same, and this type
// parameterizes functions that can do both.
type QueryType = 'an' | 'fw';

// A query for full results, initiated when the user submits the
// anagram or find word form.
type Query = {
  type: string,
  q: string,
};

type FullResult = {
  q: string,
  words: DictEntry[],
};

type DictEntry = {
  word: string,
  score: number | null,
  definition: string | null,
};

type PreviewResult = {
  full_count: number,
  lengths: string,
  query: string,
  words: PreviewWord[],
};

type PreviewWord = {
  ranked?: boolean,
  text: string,
};

type SubmitHandler = (query: string) => void;

type ThesaurusResult = {
  count: string,
  words: ThesaurusWordGroup[],
};

type ThesaurusWordGroup = [string, string[]];

function Wordfun() {
  const [result, setResult] = useState<FullResult | null>(null);
  const inflight = useRef<Query | null>(null);

  // The back-end is fast, but it's still possible for responses to come
  // back out of order.
  async function runQuery(type: QueryType, q: string): Promise<void> {
    const query = {type, q};
    inflight.current = query;
    const response = await fetch(`/words/${type}?q=${encodeURIComponent(q)}`);
    const results: {words: DictEntry[]} = await response.json();
    if (inflight.current && inflight.current === query) {
      const words = results.words;
      setResult({q, words});
    }
  }

  return (
    <>
      <IntroOrResults result={result} />
      <Anagram onSubmit={q => runQuery('an', q)} />
      <FindWord onSubmit={q => runQuery('fw', q)} />
      <Thesaurus />
      <Footer />
    </>
  );
}

function IntroOrResults(props: {result: FullResult | null}) {
  const result = props.result;
  if (!result) {
    return <Intro />;
  } else if (result.words.length === 0) {
    return <section className="results">{result.q}: No matches.</section>;
  } else {
    return (
      <section>
        <h1>
          Results for <em>{result.q}</em>
        </h1>
        <FullResults words={result.words} />
      </section>
    );
  }
}

IntroOrResults.propTypes = {
  query: PropTypes.exact({
    q: PropTypes.string.isRequired,
    type: PropTypes.oneOf(['an', 'fw']),
  }),
};

function Anagram(props: {onSubmit: SubmitHandler}) {
  return (
    <section id="sect-an">
      <h2>
        <label htmlFor="an">Find an Anagram</label>
      </h2>
      <div className="help info">
        <p>
          Type in a word or series of words here to get valid words and phrases
          from the dictionary. For example,
        </p>
        <ul>
          <li>
            “Pioneering tsar’s exotic voyages (14)” (Guardian 28,233): searching
            for “pioneering tsar” gives the answer <samp>PEREGRINATIONS</samp>
          </li>
          <li>
            From the same puzzle, “State capital one missed when touring (3,6)”
            suggests searching for “one missed”, which again gives the right
            answer <samp>DES MOINES</samp>.
          </li>
          <li>
            You can find partial anagrams, too: just pad out the search with
            dots. For example, “Set up an unusual lab aboard vessel in
            Devonport, perhaps (5,4)” (Guardian Quiptic 1,087) has “lab”, and
            you might guess that “vessel” is “vase”. That just needs two more
            letters, and searching for <kbd>labvase..</kbd> yields 5 results,
            one of which is the correct answer, <samp>NAVAL BASE</samp>.
          </li>
        </ul>
      </div>

      <Tool
        type="an"
        label="Anagram"
        autoFocus={true}
        onSubmit={props.onSubmit}
      />
    </section>
  );
}
Anagram.propTypes = {
  onSubmit: PropTypes.func.isRequired,
};

function FindWord(props: {onSubmit: SubmitHandler}) {
  return (
    <section id="sect-fw">
      <h2>
        <label htmlFor="fw">Complete a Word or Phrase</label>
      </h2>
      <div className="help info">
        <p>
          Type in what you have, with dots for the missing letters (e.g.,{' '}
          <kbd>h.r...i.m</kbd>) to get matching words and phrases from the
          dictionary. You can match word boundaries with forward slashes, like
          this: <kbd>h.r./...l../e.g</kbd>
        </p>
      </div>
      <Tool type="fw" label="Find Word" onSubmit={props.onSubmit} />
    </section>
  );
}
FindWord.propTypes = {
  onSubmit: PropTypes.func.isRequired,
};

function Thesaurus() {
  return (
    <section id="sect-thesaurus">
      <h2>
        <label htmlFor="th">Thesaurus</label>
      </h2>
      <div className="help info">
        <p>Enter a word to get synonyms. For example,</p>
        <ul>
          <li>
            “17. May be tuna sandwiches always hot (8)” (Financial Times 16,569)
            suggests “fish” (a synonym of “tuna”) enveloping “ever” (a synonym
            of “always”), giving <samp>FEVERISH</samp>.
          </li>
        </ul>
      </div>
      <ThesaurusTool />
    </section>
  );
}

function Tool(props: {
  type: QueryType,
  label: string,
  autoFocus?: boolean,
  onSubmit: SubmitHandler,
}) {
  const [query, setQuery] = useState<string>('');
  const [result, setResult] = useState<PreviewResult | null>(null);
  const inflight = useRef<string | null>(null);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    props.onSubmit(query);
  };

  async function handleInput(input: string) {
    input = input.trim();
    setQuery(input);
    try {
      const path = `/preview/${props.type}`;
      const result = await fetchPreview(path, input, inflight);
      setResult(result);
    } catch (e) {
      console.log('Not current?', e);
      // Not current
    }
  }

  return (
    <div className="tool">
      <ToolForm
        onInput={handleInput}
        onSubmit={handleSubmit}
        label={props.label}
        id={props.type}
        value={query}
        autoFocus={props.autoFocus}
      />
      <Preview query={query} result={result} />
    </div>
  );
}

Tool.propTypes = {
  type: PropTypes.oneOf(['an', 'fw']).isRequired,
  label: PropTypes.string.isRequired,
  autoFocus: PropTypes.bool,
  onSubmit: PropTypes.func.isRequired,
};

function ToolForm(props: {
  onSubmit: (e: FormEvent) => void,
  onInput: (input: string) => Promise<void>,
  id: string,
  label: string,
  value: string | null,
  autoFocus?: boolean,
}) {
  const input = useRef<HTMLInputElement>(null);

  const clearInput = () => {
    if (input.current) {
      input.current.value = '';
      input.current.focus();
      props.onInput('');
    }
  };

  return (
    <form onSubmit={props.onSubmit}>
      <input
        id={props.id}
        autoCapitalize="off"
        autoCorrect="off"
        autoComplete="off"
        autoFocus={props.autoFocus}
        ref={input}
        value={props.value || ''}
        type="text"
        onChange={e => props.onInput(e.target.value)}
      />
      <button type="submit" className="btn">
        {props.label}
      </button>
      <button type="button" onClick={clearInput} className="btn-clear">
        Clear
      </button>
    </form>
  );
}

function Preview(props: {query: string, result: PreviewResult | null}) {
  const {query, result} = props;
  if (!result) {
    return null;
  }
  if (result.full_count === 0) {
    return (
      <div className="preview">
        {query} ({result.lengths}): No matches.
      </div>
    );
  }

  let count = pluralize(result.full_count, 'result', 'results');
  let wordList: React.ReactNode[] = result.words.map(w => {
    if (w.ranked) {
      return <strong key={w.text}>{w.text}</strong>;
    } else {
      return <span key={w.text}>{w.text}</span>;
    }
  });
  if (wordList.length < result.full_count) {
    wordList.push('...');
  }

  const words = wordList.map((word, i) => [i > 0 && ', ', word]);

  return (
    <div className="preview">
      {query} ({result.lengths}): {count} ({words})
    </div>
  );
}

Preview.propTypes = {
  query: PropTypes.string.isRequired,
  result: PropTypes.shape({
    full_count: PropTypes.number.isRequired,
    lengths: PropTypes.string.isRequired,
    query: PropTypes.string.isRequired,
    words: PropTypes.arrayOf(
      PropTypes.shape({
        text: PropTypes.string.isRequired,
        ranked: PropTypes.bool.isRequired,
      }).isRequired,
    ).isRequired,
  }),
};

function FullResults(props: {words: DictEntry[]}) {
  const [modalIsOpen, setIsOpen] = React.useState<boolean>(false);
  const [modalUrl, setUrl] = React.useState<string | null>(null);

  function closeModal() {
    setIsOpen(false);
  }

  function openUrl(url: string) {
    setUrl(url);
    setIsOpen(true);
  }

  return (
    <div className="results">
      {props.words.map(e => (
        <Entry key={e.word} onClick={openUrl} entry={e} />
      ))}
      <LinkModal
        isOpen={modalIsOpen}
        url={modalUrl}
        onCloseRequest={closeModal}
      />
    </div>
  );
}

FullResults.propTypes = {
  words: PropTypes.arrayOf(
    PropTypes.exact({
      word: PropTypes.string.isRequired,
      score: PropTypes.number,
      definition: PropTypes.string,
    }),
  ),
};

function Entry(props: {entry: DictEntry, onClick: (url: string) => void}) {
  const entry = props.entry;
  const word = entry.word;
  const score = entry.score;
  const definition = entry.definition;
  const onClick = props.onClick;

  const classes = score && score > 0 ? 'entry good' : 'entry';
  const dictUrl =
    'https://www.thefreedictionary.com/' + encodeURIComponent(word);

  function handleClick(e: MouseEvent) {
    e.preventDefault();
    onClick(dictUrl);
  }

  return (
    <div className={classes}>
      <a
        className="word"
        href={dictUrl}
        target="wf_lookup"
        onClick={handleClick}>
        {word}
      </a>
      <dfn className="dfn">{definition}</dfn>
    </div>
  );
}

Entry.propTypes = {
  word: PropTypes.string,
  score: PropTypes.number,
  definition: PropTypes.string,
  onClick: PropTypes.func.isRequired,
};

// This is the popup that loads a definition from thefreedictionary.com.
function LinkModal(props: {
  isOpen: boolean,
  onCloseRequest: () => void,
  url: string | null,
}) {
  useEffect(() => {
    function handleKeyUp(e: KeyboardEvent) {
      if (e.keyCode === 27) {
        props.onCloseRequest();
      }
    }

    window.addEventListener('keyup', handleKeyUp);
    return () => window.removeEventListener('keyup', handleKeyUp);
  });

  return (
    <Modal
      isOpen={props.isOpen}
      onRequestClose={props.onCloseRequest}
      className="modal"
      overlayClassName="overlay"
      contentLabel="Example Modal">
      <button className="modal-close" onClick={props.onCloseRequest}>
        <img src={closeIcon} width="32" alt="close" />
      </button>
      <iframe
        className="dictionary-iframe"
        title="dictionary"
        src={props.url || ''}></iframe>
    </Modal>
  );
}

LinkModal.propTypes = {
  isOpen: PropTypes.bool.isRequired,
  url: PropTypes.string,
  onCloseRequest: PropTypes.func.isRequired,
};

function ThesaurusTool() {
  const [query, setQuery] = useState<string>('');
  const [result, setResult] = useState<ThesaurusResult | null>(null);
  const inflight = useRef<string | null>(null);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
  };

  async function handleInput(input: string) {
    input = input.trim();
    setQuery(input);

    try {
      const result = await fetchPreview(`/preview/thesaurus`, input, inflight);
      setResult(result);
    } catch (e) {
      // Not current
    }
  }

  return (
    <div className="tool">
      <ToolForm
        onInput={handleInput}
        onSubmit={handleSubmit}
        value={query}
        label="Thesaurus"
        id="th"
      />
      <ThesaurusPreview query={query} result={result} onSearch={handleInput} />
    </div>
  );
}

Thesaurus.propTypes = {};

function ThesaurusPreview(props: {
  query: string,
  result: ThesaurusResult | null,
  onSearch: (word: string) => void,
}) {
  let {query, result} = props;

  if (!result) {
    return null;
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

function ThesaurusGroup(props: {
  len: string,
  words: string[],
  onSearch: (word: string) => void,
}) {
  function handleSearch(e: MouseEvent, word: string) {
    e.preventDefault();
    props.onSearch(word);
  }
  const wordList = props.words.map(word => (
    <a
      key={word}
      href={`#/thesaurus/${word}`}
      onClick={e => handleSearch(e, word)}>
      {word}
    </a>
  ));
  const words = wordList.map((word, i) => [i > 0 && ', ', word]);
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

export default Wordfun;
