import React from 'react';
import infoIcon from './icons/info.svg';

export default function Intro() {
  return (
    <section className="intro">
      <div id="intro">
        <h1>Crossword Solver</h1>
        <p>
          I enjoy crosswords, and wrote these tools to help me. There are well
          over a quarter of a million words and phrases in the dictionary,
          thanks to <a href="http://www.crosswordman.com/">Ross Beresford</a>'s
          UKACD dictionary and a huge list of phrases from Ross Withey.
        </p>

        <div className="info callout">
          <h2>
            <img src={infoIcon} alt="info" className="info-icon" /> Come here
            often?
          </h2>
          <p>
            If you're a regular user of this site, you'll probably notice some
            changes over the next few weeks. Please let me know if anything
            stops working for you, or if you prefer anything the way it was.
            &mdash;<a href="mailto:pete@petebevin.com">Pete</a>.
          </p>
          <p>
            Update 3 (21 Sept): The site ought to be working for everyone now.
            If not, please let me know, and in the meantime, you can still get
            to <a href="https://old.wordfun.ca/">the old version of the site</a>{' '}
            if you need to. Also, anagram search now allows missing letters, and
            all the word boundary bugs should be fixed.
          </p>

          <p>Still left to do:</p>
          <ul className="todo">
            <li>
              Thesarus is overly pedantic about spaces (at the start, at the
              end, or in the middle!)
            </li>
            <li>Bring back the cryptogram solver!</li>
          </ul>
        </div>
      </div>
    </section>
  );
}
