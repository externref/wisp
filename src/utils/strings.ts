export function handlePathName(path: string): string {
  const MAX_SHORT_LENGTH = 20;
  const MAX_LONG_LENGTH = 50;
  const pathParts = path.split("\\");
  if (path.length <= MAX_SHORT_LENGTH) {
    return path;
  }
  const abbreviatedParts = pathParts.map((part, index) => {
    if (index >= pathParts.length - 2) {
      return part;
    }
    return part.charAt(0);
  });

  const abbreviatedPath = abbreviatedParts.join("\\");
  if (abbreviatedPath.length > MAX_LONG_LENGTH) {
    const truncatedParts = abbreviatedParts.slice(
      -Math.ceil(pathParts.length / 2),
    );
    return truncatedParts.join("\\");
  }

  return abbreviatedPath;
}
