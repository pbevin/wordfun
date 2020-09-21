// Return true if the argument is null or a string consisting
// entirely of whitespace (including the empty string).
function isBlank(string) {
  return string === null || string.match(/^\s*$/);
}

export default isBlank;
