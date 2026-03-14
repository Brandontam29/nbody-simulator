export function removeElementsByIndices<T>(array: T[], indicesToRemove: number[]): void {
  // Sort indices in descending order to avoid affecting the positions
  // of elements yet to be removed.
  indicesToRemove.sort((a: number, b: number) => b - a);

  // Remove elements from the array starting from the highest index.
  for (const index of indicesToRemove) {
    if (index >= 0 && index < array.length) {
      // Ensure the index is valid
      array.splice(index, 1);
    }
  }
}
