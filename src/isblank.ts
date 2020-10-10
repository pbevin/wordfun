// Return true if the argument is null or a string consisting
// entirely of whitespace (including the empty string).
function isBlank(string: string | null): boolean {
  return string === null || /^\s*$/.test(string);
}

export default isBlank;
