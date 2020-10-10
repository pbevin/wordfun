export function pluralize(
  count: number,
  singular: string,
  plural: string,
): string {
  if (count === 1) {
    return `1 ${singular}`;
  } else {
    return `${count} ${plural}`;
  }
}
