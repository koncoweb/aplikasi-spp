const { invoke } = window.__TAURI__.core;

// Authentication utilities
const Auth = {
  // Get stored token
  getToken: function () {
    return localStorage.getItem('auth_token');
  },

  // Get stored user
  getUser: function () {
    const userStr = localStorage.getItem('user');
    return userStr ? JSON.parse(userStr) : null;
  },

  // Get stored tenant
  getTenant: function () {
    const tenantStr = localStorage.getItem('tenant');
    return tenantStr ? JSON.parse(tenantStr) : null;
  },

  // Check if user is authenticated
  isAuthenticated: function () {
    return !!this.getToken();
  },

  // Get user role
  getUserRole: function () {
    const user = this.getUser();
    return user ? user.role : null;
  },

  // Check if user has specific role
  hasRole: function (role) {
    return this.getUserRole() === role;
  },

  // Clear auth data (logout)
  logout: function () {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
    localStorage.removeItem('tenant');
  },

  // Session verification - can be called on app load
  verifySession: async function () {
    const token = this.getToken();
    if (!token) {
      return { valid: false, reason: 'No token found' };
    }

    // In a production app, you would verify the token with the backend
    // For now, we just check if token exists
    try {
      // Optional: verify token format
      const parts = token.split('.');
      if (parts.length !== 3) {
        this.logout();
        return { valid: false, reason: 'Invalid token format' };
      }

      // Optional: decode JWT to check expiration
      const payload = JSON.parse(atob(parts[1]));
      if (payload.exp * 1000 < Date.now()) {
        this.logout();
        return { valid: false, reason: 'Token expired' };
      }

      return { valid: true, user: this.getUser(), tenant: this.getTenant() };
    } catch (e) {
      this.logout();
      return { valid: false, reason: 'Token verification failed' };
    }
  }
};

// Make Auth available globally
window.SwiftPOS = {
  Auth: Auth,
  invoke: invoke
};

// Handle page load
window.addEventListener('DOMContentLoaded', async () => {
  console.log('SwiftPOS initialized');

  // Verify session on page load
  const session = await Auth.verifySession();
  if (session.valid) {
    console.log('User session verified:', session.user);
  } else {
    console.log('Session verification failed:', session.reason);
  }
});

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { Auth };
}
