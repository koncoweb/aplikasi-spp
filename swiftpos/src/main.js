// Neon Auth Client Configuration
// Using Stack Auth (Neon Auth) for authentication

const NEON_AUTH_CONFIG = {
  projectId: '6c9ac121-f4a6-468d-b64b-630e7b1c55ed',
  publishableClientKey: 'pck_pfexc96q1ej8zayty0fzvmmwd4eaxhxawh1ddjp8ws780'
};

// Base URL for Neon Auth API
const getAuthBaseUrl = () => {
  return `https://api.stack-auth.com/api/v1/projects/${NEON_AUTH_CONFIG.projectId}`;
};

// Neon Auth API helper
const neonAuthApi = {
  // Sign in with email/password
  async signInEmail(email, password) {
    const response = await fetch(`${getAuthBaseUrl()}/auth/sign-in/email`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email,
        password,
      }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Sign in failed');
    }

    return response.json();
  },

  // Sign up with email/password
  async signUpEmail(email, password, name) {
    const response = await fetch(`${getAuthBaseUrl()}/auth/sign-up/email`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email,
        password,
        name,
      }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Sign up failed');
    }

    return response.json();
  },

  // Sign out
  async signOut(sessionToken) {
    const response = await fetch(`${getAuthBaseUrl()}/auth/sign-out`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${sessionToken}`,
      },
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Sign out failed');
    }

    return response.json();
  },

  // Get current user
  async getSession(sessionToken) {
    const response = await fetch(`${getAuthBaseUrl()}/auth/session`, {
      headers: {
        'Authorization': `Bearer ${sessionToken}`,
      },
    });

    if (!response.ok) {
      return null;
    }

    return response.json();
  },

  // Verify JWT token
  async verifyToken(token) {
    try {
      const response = await fetch(`${getAuthBaseUrl()}/auth/verify-token`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ token }),
      });

      if (!response.ok) {
        return null;
      }

      return response.json();
    } catch (error) {
      console.error('Token verification error:', error);
      return null;
    }
  }
};

// Authentication utilities
const Auth = {
  // Get stored token
  getToken: function () {
    return localStorage.getItem('neon_auth_token');
  },

  // Get stored user
  getUser: function () {
    const userStr = localStorage.getItem('neon_user');
    return userStr ? JSON.parse(userStr) : null;
  },

  // Get stored session
  getSession: function () {
    const sessionStr = localStorage.getItem('neon_session');
    return sessionStr ? JSON.parse(sessionStr) : null;
  },

  // Check if user is authenticated
  isAuthenticated: function () {
    return !!this.getToken();
  },

  // Sign in
  signIn: async function (email, password) {
    try {
      const result = await neonAuthApi.signInEmail(email, password);

      if (result.token && result.user) {
        // Store auth data
        localStorage.setItem('neon_auth_token', result.token);
        localStorage.setItem('neon_user', JSON.stringify(result.user));
        if (result.session) {
          localStorage.setItem('neon_session', JSON.stringify(result.session));
        }

        return { success: true, user: result.user };
      }

      throw new Error('Invalid response from auth server');
    } catch (error) {
      console.error('Sign in error:', error);
      return { success: false, error: error.message };
    }
  },

  // Sign up
  signUp: async function (email, password, name) {
    try {
      const result = await neonAuthApi.signUpEmail(email, password, name);

      if (result.token && result.user) {
        // Store auth data
        localStorage.setItem('neon_auth_token', result.token);
        localStorage.setItem('neon_user', JSON.stringify(result.user));
        if (result.session) {
          localStorage.setItem('neon_session', JSON.stringify(result.session));
        }

        return { success: true, user: result.user };
      }

      // If no token returned, user needs to verify email
      return { success: true, requiresVerification: true };
    } catch (error) {
      console.error('Sign up error:', error);
      return { success: false, error: error.message };
    }
  },

  // Sign out
  signOut: async function () {
    const token = this.getToken();
    if (token) {
      try {
        await neonAuthApi.signOut(token);
      } catch (error) {
        console.error('Sign out error:', error);
      }
    }

    // Clear local storage
    localStorage.removeItem('neon_auth_token');
    localStorage.removeItem('neon_user');
    localStorage.removeItem('neon_session');

    return { success: true };
  },

  // Session verification
  verifySession: async function () {
    const token = this.getToken();
    if (!token) {
      return { valid: false, reason: 'No token found' };
    }

    try {
      const result = await neonAuthApi.verifyToken(token);
      if (result && result.user) {
        return { valid: true, user: result.user };
      }

      // Try to get session
      const session = await neonAuthApi.getSession(token);
      if (session && session.user) {
        localStorage.setItem('neon_user', JSON.stringify(session.user));
        return { valid: true, user: session.user };
      }

      this.signOut();
      return { valid: false, reason: 'Session expired' };
    } catch (error) {
      console.error('Session verification error:', error);
      this.signOut();
      return { valid: false, reason: 'Verification failed' };
    }
  }
};

// Make Auth available globally
window.SwiftPOS = {
  Auth: Auth,
  neonAuth: neonAuthApi,
  config: NEON_AUTH_CONFIG
};

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { Auth, neonAuthApi, NEON_AUTH_CONFIG };
}
