// Neon Auth Client Configuration
// Using Stack Auth (Neon Auth) for authentication
// Credentials should be provided via Tauri env or config file

const { invoke } = window.__TAURI__.core;

// Default fallback values (for development only - should be overridden in production)
const DEFAULT_AUTH_CONFIG = {
  projectId: '6c9ac121-f4a6-468d-b64b-630e7b1c55ed',
  publishableClientKey: 'pck_pfexc96q1ej8zayty0fzvmmwd4eaxhxawh1ddjp8ws780'
};

// Get config from window.ENV (set by backend) or use defaults
const getAuthConfig = async () => {
  // If window.ENV is set by Tauri, use it
  if (window.ENV?.NEON_AUTH_PROJECT_ID) {
    return {
      projectId: window.ENV.NEON_AUTH_PROJECT_ID,
      publishableClientKey: window.ENV.NEXT_PUBLIC_STACK_PUBLISHABLE_CLIENT_KEY
    };
  }

  // Otherwise try to get from Tauri invoke
  try {
    const envConfig = await invoke('get_auth_config');
    if (envConfig?.projectId) {
      return {
        projectId: envConfig.projectId,
        publishableClientKey: envConfig.publishableClientKey
      };
    }
  } catch (e) {
    console.warn('Could not load auth config from backend, using defaults');
  }

  // Fall back to defaults (warning in console)
  console.warn('Using default auth config - this should be overridden in production');
  return DEFAULT_AUTH_CONFIG;
};

// Lazy-loaded config
let _authConfig = null;
const NEON_AUTH_CONFIG = {
  get projectId() {
    return _authConfig?.projectId || DEFAULT_AUTH_CONFIG.projectId;
  },
  get publishableClientKey() {
    return _authConfig?.publishableClientKey || DEFAULT_AUTH_CONFIG.publishableClientKey;
  }
};

// Initialize config on load
(async () => {
  _authConfig = await getAuthConfig();
})();

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

  // Sign in
  signIn: async function (email, password) {
    try {
      const result = await neonAuthApi.signInEmail(email, password);

      if (result.token && result.user) {
        // Store Stack Auth token
        localStorage.setItem('neon_auth_token', result.token);
        localStorage.setItem('neon_user', JSON.stringify(result.user));
        if (result.session) {
          localStorage.setItem('neon_session', JSON.stringify(result.session));
        }

        // Now verify with backend to get SwiftPOS user data and permissions
        try {
          const backendResponse = await invoke('neon_auth_login', {
            request: { token: result.token }
          });

          if (backendResponse.success) {
            // Store backend user data
            localStorage.setItem('swiftpos_user', JSON.stringify(backendResponse.user));
            localStorage.setItem('swiftpos_tenant', backendResponse.tenant ? JSON.stringify(backendResponse.tenant) : null);
            localStorage.setItem('swiftpos_permissions', JSON.stringify(backendResponse.permissions));

            return { success: true, user: backendResponse.user, tenant: backendResponse.tenant };
          } else {
            // Stack Auth login succeeded but SwiftPOS user not found
            // Clear Stack Auth token since it won't work
            localStorage.removeItem('neon_auth_token');
            localStorage.removeItem('neon_user');
            localStorage.removeItem('neon_session');

            return { success: false, error: backendResponse.message || 'User not provisioned in SwiftPOS' };
          }
        } catch (backendError) {
          console.error('Backend verification error:', backendError);
          return { success: false, error: 'Failed to verify with backend' };
        }
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

    // Clear all storage
    localStorage.removeItem('neon_auth_token');
    localStorage.removeItem('neon_user');
    localStorage.removeItem('neon_session');
    localStorage.removeItem('swiftpos_user');
    localStorage.removeItem('swiftpos_tenant');
    localStorage.removeItem('swiftpos_permissions');

    return { success: true };
  },

  // Session verification
  verifySession: async function () {
    const token = this.getToken();
    if (!token) {
      return { valid: false, reason: 'No token found' };
    }

    try {
      // First verify with Stack Auth
      const result = await neonAuthApi.verifyToken(token);
      if (!result || !result.user) {
        // Try to get session
        const session = await neonAuthApi.getSession(token);
        if (!session || !session.user) {
          this.signOut();
          return { valid: false, reason: 'Session expired' };
        }
      }

      // Verify with backend to get SwiftPOS user data
      try {
        const backendResponse = await invoke('neon_auth_login', {
          request: { token: token }
        });

        if (backendResponse.success) {
          // Update stored data
          localStorage.setItem('swiftpos_user', JSON.stringify(backendResponse.user));
          localStorage.setItem('swiftpos_tenant', backendResponse.tenant ? JSON.stringify(backendResponse.tenant) : null);
          localStorage.setItem('swiftpos_permissions', JSON.stringify(backendResponse.permissions));

          return { valid: true, user: backendResponse.user, tenant: backendResponse.tenant };
        } else {
          this.signOut();
          return { valid: false, reason: backendResponse.message || 'User not provisioned' };
        }
      } catch (backendError) {
        console.error('Backend verification error:', backendError);
        // If backend is unavailable, allow using cached data
        const cachedUser = localStorage.getItem('swiftpos_user');
        if (cachedUser) {
          return { valid: true, user: JSON.parse(cachedUser) };
        }
        return { valid: false, reason: 'Backend verification failed' };
      }
    } catch (error) {
      console.error('Session verification error:', error);
      this.signOut();
      return { valid: false, reason: 'Verification failed' };
    }
  },

  // Check if user has a specific permission
  hasPermission: function (permission) {
    const permissions = JSON.parse(localStorage.getItem('swiftpos_permissions') || '[]');
    return permissions.includes(permission);
  },

  // Check if user has a specific role
  hasRole: function (role) {
    const user = this.getUser();
    return user && user.role === role;
  },

  // Get current user (from SwiftPOS data, not Neon Auth)
  getUser: function () {
    const userStr = localStorage.getItem('swiftpos_user');
    return userStr ? JSON.parse(userStr) : null;
  },

  // Get current tenant
  getTenant: function () {
    const tenantStr = localStorage.getItem('swiftpos_tenant');
    return tenantStr ? JSON.parse(tenantStr) : null;
  },
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
