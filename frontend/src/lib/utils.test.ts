import { describe, it, expect } from 'vitest';
import { cn, formatDate, formatCurrency, getStatusColor, getPriorityColor } from './utils';

// ============ cn() Tests ============

describe('cn', () => {
  it('merges class names correctly', () => {
    const result = cn('px-2 py-1', 'px-4');
    expect(result).toBe('py-1 px-4');
  });

  it('handles conditional classes', () => {
    const isActive = true;
    const result = cn('base-class', isActive && 'active-class');
    expect(result).toContain('base-class');
    expect(result).toContain('active-class');
  });

  it('handles false conditionals', () => {
    const isActive = false;
    const result = cn('base-class', isActive && 'active-class');
    expect(result).toBe('base-class');
    expect(result).not.toContain('active-class');
  });

  it('handles empty inputs', () => {
    const result = cn();
    expect(result).toBe('');
  });

  it('handles undefined and null', () => {
    const result = cn('base', undefined, null, 'end');
    expect(result).toBe('base end');
  });

  it('handles array of classes', () => {
    const result = cn(['class1', 'class2']);
    expect(result).toContain('class1');
    expect(result).toContain('class2');
  });

  it('handles tailwind conflicts correctly', () => {
    // Later classes should override earlier ones
    const result = cn('text-red-500', 'text-blue-500');
    expect(result).toBe('text-blue-500');
  });
});

// ============ formatDate() Tests ============

describe('formatDate', () => {
  it('formats valid date string', () => {
    const result = formatDate('2024-12-25');
    expect(result).toMatch(/25.*Des.*2024/);
  });

  it('returns dash for undefined', () => {
    const result = formatDate(undefined);
    expect(result).toBe('-');
  });

  it('returns dash for empty string', () => {
    const result = formatDate('');
    expect(result).toBe('-');
  });

  it('handles ISO date format', () => {
    const result = formatDate('2024-01-15T10:30:00Z');
    expect(result).toMatch(/15.*Jan.*2024/);
  });

  it('formats different months correctly', () => {
    expect(formatDate('2024-03-01')).toMatch(/Mar/);
    expect(formatDate('2024-06-15')).toMatch(/Jun/);
    expect(formatDate('2024-11-30')).toMatch(/Nov/);
  });
});

// ============ formatCurrency() Tests ============

describe('formatCurrency', () => {
  it('formats currency in IDR', () => {
    const result = formatCurrency('1000000');
    expect(result).toMatch(/Rp/);
    expect(result).toMatch(/1\.000\.000/);
  });

  it('returns dash for undefined', () => {
    const result = formatCurrency(undefined);
    expect(result).toBe('-');
  });

  it('returns dash for empty string', () => {
    const result = formatCurrency('');
    expect(result).toBe('-');
  });

  it('handles zero', () => {
    const result = formatCurrency('0');
    expect(result).toMatch(/Rp/);
    expect(result).toMatch(/0/);
  });

  it('handles large numbers', () => {
    const result = formatCurrency('1000000000');
    expect(result).toMatch(/1\.000\.000\.000/);
  });

  it('handles decimal numbers (truncates)', () => {
    const result = formatCurrency('1500.75');
    // Should format without decimal places
    expect(result).toMatch(/1\.501|1\.500/); // Depends on rounding
  });
});

// ============ getStatusColor() Tests ============

describe('getStatusColor', () => {
  // Project statuses
  it('returns correct color for Planning status', () => {
    const result = getStatusColor('Planning');
    expect(result).toBe('bg-blue-100 text-blue-800');
  });

  it('returns correct color for Active status', () => {
    const result = getStatusColor('Active');
    expect(result).toBe('bg-green-100 text-green-800');
  });

  it('returns correct color for OnHold status', () => {
    const result = getStatusColor('OnHold');
    expect(result).toBe('bg-yellow-100 text-yellow-800');
  });

  it('returns correct color for Completed status', () => {
    const result = getStatusColor('Completed');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });

  it('returns correct color for Cancelled status', () => {
    const result = getStatusColor('Cancelled');
    expect(result).toBe('bg-red-100 text-red-800');
  });

  // Task statuses
  it('returns correct color for Todo status', () => {
    const result = getStatusColor('Todo');
    expect(result).toBe('bg-slate-100 text-slate-800');
  });

  it('returns correct color for inprogress status', () => {
    const result = getStatusColor('inprogress');
    expect(result).toBe('bg-blue-100 text-blue-800');
  });

  it('returns correct color for Review status', () => {
    const result = getStatusColor('Review');
    expect(result).toBe('bg-purple-100 text-purple-800');
  });

  it('returns correct color for Done status', () => {
    const result = getStatusColor('Done');
    expect(result).toBe('bg-green-100 text-green-800');
  });

  it('returns correct color for Blocked status', () => {
    const result = getStatusColor('Blocked');
    expect(result).toBe('bg-red-100 text-red-800');
  });

  // Unknown status
  it('returns default gray for unknown status', () => {
    const result = getStatusColor('Unknown');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });

  it('returns default gray for empty string', () => {
    const result = getStatusColor('');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });
});

// ============ getPriorityColor() Tests ============

describe('getPriorityColor', () => {
  it('returns correct color for Low priority', () => {
    const result = getPriorityColor('Low');
    expect(result).toBe('bg-slate-100 text-slate-800');
  });

  it('returns correct color for Medium priority', () => {
    const result = getPriorityColor('Medium');
    expect(result).toBe('bg-blue-100 text-blue-800');
  });

  it('returns correct color for High priority', () => {
    const result = getPriorityColor('High');
    expect(result).toBe('bg-orange-100 text-orange-800');
  });

  it('returns correct color for Critical priority', () => {
    const result = getPriorityColor('Critical');
    expect(result).toBe('bg-red-100 text-red-800');
  });

  it('returns default gray for unknown priority', () => {
    const result = getPriorityColor('Unknown');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });

  it('returns default gray for empty string', () => {
    const result = getPriorityColor('');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });

  it('is case sensitive', () => {
    // lowercase should not match
    const result = getPriorityColor('low');
    expect(result).toBe('bg-gray-100 text-gray-800');
  });
});
