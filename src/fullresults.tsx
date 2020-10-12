import React, { useEffect, MouseEvent } from "react";
import PropTypes from "prop-types";
import Modal from "react-modal";

import closeIcon from "./icons/close.svg";

export type DictEntry = {
  word: string;
  score: number | null;
  definition: string | null;
};

function FullResults(props: { words: DictEntry[] }) {
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
      <LinkModal isOpen={modalIsOpen} url={modalUrl} onCloseRequest={closeModal} />
    </div>
  );
}

FullResults.propTypes = {
  words: PropTypes.arrayOf(
    PropTypes.exact({
      word: PropTypes.string.isRequired,
      score: PropTypes.number,
      definition: PropTypes.string
    })
  )
};

function Entry(props: { entry: DictEntry; onClick: (url: string) => void }) {
  const entry = props.entry;
  const word = entry.word;
  const score = entry.score;
  const definition = entry.definition;
  const onClick = props.onClick;

  const classes = score && score > 0 ? "entry good" : "entry";
  const dictUrl = "https://www.thefreedictionary.com/" + encodeURIComponent(word);

  function handleClick(e: MouseEvent) {
    e.preventDefault();
    onClick(dictUrl);
  }

  return (
    <div className={classes}>
      <a className="word" href={dictUrl} target="wf_lookup" onClick={handleClick}>
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
  onClick: PropTypes.func.isRequired
};

// This is the popup that loads a definition from thefreedictionary.com.
function LinkModal(props: { isOpen: boolean; onCloseRequest: () => void; url: string | null }) {
  useEffect(() => {
    function handleKeyUp(e: KeyboardEvent) {
      if (e.keyCode === 27) {
        props.onCloseRequest();
      }
    }

    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  });

  return (
    <Modal
      isOpen={props.isOpen}
      onRequestClose={props.onCloseRequest}
      className="modal"
      overlayClassName="overlay"
      contentLabel="Example Modal"
    >
      <button className="modal-close" onClick={props.onCloseRequest}>
        <img src={closeIcon} width="32" alt="close" />
      </button>
      <iframe className="dictionary-iframe" title="dictionary" src={props.url || ""}></iframe>
    </Modal>
  );
}

LinkModal.propTypes = {
  isOpen: PropTypes.bool.isRequired,
  url: PropTypes.string,
  onCloseRequest: PropTypes.func.isRequired
};

export default FullResults;
