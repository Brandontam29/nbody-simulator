import { describe, it, expect } from 'vitest';
import { removeElementsByIndices } from './array';

describe('removeElementsByIndices', () => {
  it('should remove elements at specified indices', () => {
    const arr = ['a', 'b', 'c', 'd', 'e'];
    removeElementsByIndices(arr, [1, 3]);
    expect(arr).toEqual(['a', 'c', 'e']);
  });

  it('should handle empty arrays', () => {
    const arr: string[] = [];
    removeElementsByIndices(arr, [0, 1]);
    expect(arr).toEqual([]);
  });

  it('should handle empty indices list', () => {
    const arr = [1, 2, 3];
    removeElementsByIndices(arr, []);
    expect(arr).toEqual([1, 2, 3]);
  });

  it('should handle out-of-bounds indices gracefully', () => {
    const arr = [10, 20, 30];
    removeElementsByIndices(arr, [-1, 5]);
    expect(arr).toEqual([10, 20, 30]);
  });

  it('should not be affected by the order of indices provided', () => {
    const arr = [1, 2, 3, 4];
    removeElementsByIndices(arr, [2, 0]);
    expect(arr).toEqual([2, 4]);
  });
});
