import {MutableRefObject} from 'react';
import isBlank from './isblank';

async function fetchPreview(
  path: string,
  input: string,
  inflight: MutableRefObject<string | null>,
): Promise<any> {
  inflight.current = input;
  if (isBlank(input)) {
    return null;
  } else {
    const response = await fetch(`${path}?q=${encodeURIComponent(input)}`);
    const result = await response.json();
    if (inflight.current === input) {
      return result;
    } else {
      throw new Error('Not current');
    }
  }
}

export default fetchPreview;
