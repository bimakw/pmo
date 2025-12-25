import { describe, it, expect, beforeEach, vi } from 'vitest';
import { useAuthStore } from './auth';
import type { User } from '@/types';

// Mock localStorage
const localStorageMock = {
  store: {} as Record<string, string>,
  getItem: vi.fn((key: string) => localStorageMock.store[key] || null),
  setItem: vi.fn((key: string, value: string) => {
    localStorageMock.store[key] = value;
  }),
  removeItem: vi.fn((key: string) => {
    delete localStorageMock.store[key];
  }),
  clear: vi.fn(() => {
    localStorageMock.store = {};
  }),
};

Object.defineProperty(global, 'localStorage', {
  value: localStorageMock,
});

// Helper to create mock user
const createMockUser = (overrides?: Partial<User>): User => ({
  id: 'user-123',
  email: 'test@example.com',
  name: 'Test User',
  role: 'Member',
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  ...overrides,
});

describe('useAuthStore', () => {
  beforeEach(() => {
    // Reset store state before each test
    useAuthStore.setState({
      user: null,
      token: null,
      isAuthenticated: false,
    });
    // Clear localStorage mock
    localStorageMock.store = {};
    vi.clearAllMocks();
  });

  // ============ Initial State Tests ============

  describe('initial state', () => {
    it('starts with null user', () => {
      const { user } = useAuthStore.getState();
      expect(user).toBeNull();
    });

    it('starts with null token', () => {
      const { token } = useAuthStore.getState();
      expect(token).toBeNull();
    });

    it('starts with isAuthenticated false', () => {
      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBe(false);
    });
  });

  // ============ setAuth Tests ============

  describe('setAuth', () => {
    it('sets user correctly', () => {
      const mockUser = createMockUser();
      const mockToken = 'jwt-token-123';

      useAuthStore.getState().setAuth(mockUser, mockToken);

      const { user } = useAuthStore.getState();
      expect(user).toEqual(mockUser);
    });

    it('sets token correctly', () => {
      const mockUser = createMockUser();
      const mockToken = 'jwt-token-123';

      useAuthStore.getState().setAuth(mockUser, mockToken);

      const { token } = useAuthStore.getState();
      expect(token).toBe(mockToken);
    });

    it('sets isAuthenticated to true', () => {
      const mockUser = createMockUser();
      const mockToken = 'jwt-token-123';

      useAuthStore.getState().setAuth(mockUser, mockToken);

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBe(true);
    });

    it('stores token in localStorage', () => {
      const mockUser = createMockUser();
      const mockToken = 'jwt-token-123';

      useAuthStore.getState().setAuth(mockUser, mockToken);

      expect(localStorageMock.setItem).toHaveBeenCalledWith('token', mockToken);
    });

    it('handles Admin role user', () => {
      const adminUser = createMockUser({ role: 'Admin' });
      const mockToken = 'admin-token';

      useAuthStore.getState().setAuth(adminUser, mockToken);

      const { user } = useAuthStore.getState();
      expect(user?.role).toBe('Admin');
    });

    it('handles Manager role user', () => {
      const managerUser = createMockUser({ role: 'Manager' });
      const mockToken = 'manager-token';

      useAuthStore.getState().setAuth(managerUser, mockToken);

      const { user } = useAuthStore.getState();
      expect(user?.role).toBe('Manager');
    });

    it('handles user with avatar_url', () => {
      const userWithAvatar = createMockUser({
        avatar_url: 'https://example.com/avatar.jpg',
      });
      const mockToken = 'token';

      useAuthStore.getState().setAuth(userWithAvatar, mockToken);

      const { user } = useAuthStore.getState();
      expect(user?.avatar_url).toBe('https://example.com/avatar.jpg');
    });
  });

  // ============ logout Tests ============

  describe('logout', () => {
    it('clears user', () => {
      const mockUser = createMockUser();
      useAuthStore.getState().setAuth(mockUser, 'token');

      useAuthStore.getState().logout();

      const { user } = useAuthStore.getState();
      expect(user).toBeNull();
    });

    it('clears token', () => {
      const mockUser = createMockUser();
      useAuthStore.getState().setAuth(mockUser, 'token');

      useAuthStore.getState().logout();

      const { token } = useAuthStore.getState();
      expect(token).toBeNull();
    });

    it('sets isAuthenticated to false', () => {
      const mockUser = createMockUser();
      useAuthStore.getState().setAuth(mockUser, 'token');

      useAuthStore.getState().logout();

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBe(false);
    });

    it('removes token from localStorage', () => {
      const mockUser = createMockUser();
      useAuthStore.getState().setAuth(mockUser, 'token');

      useAuthStore.getState().logout();

      expect(localStorageMock.removeItem).toHaveBeenCalledWith('token');
    });

    it('can logout even if not authenticated', () => {
      // Should not throw
      expect(() => useAuthStore.getState().logout()).not.toThrow();

      const { isAuthenticated } = useAuthStore.getState();
      expect(isAuthenticated).toBe(false);
    });
  });

  // ============ State Transitions Tests ============

  describe('state transitions', () => {
    it('can login and logout multiple times', () => {
      const mockUser = createMockUser();

      // First login
      useAuthStore.getState().setAuth(mockUser, 'token1');
      expect(useAuthStore.getState().isAuthenticated).toBe(true);

      // Logout
      useAuthStore.getState().logout();
      expect(useAuthStore.getState().isAuthenticated).toBe(false);

      // Second login
      useAuthStore.getState().setAuth(mockUser, 'token2');
      expect(useAuthStore.getState().isAuthenticated).toBe(true);
      expect(useAuthStore.getState().token).toBe('token2');
    });

    it('can update auth with different user', () => {
      const user1 = createMockUser({ id: 'user-1', email: 'user1@example.com' });
      const user2 = createMockUser({ id: 'user-2', email: 'user2@example.com' });

      useAuthStore.getState().setAuth(user1, 'token1');
      expect(useAuthStore.getState().user?.id).toBe('user-1');

      useAuthStore.getState().setAuth(user2, 'token2');
      expect(useAuthStore.getState().user?.id).toBe('user-2');
      expect(useAuthStore.getState().token).toBe('token2');
    });
  });
});
