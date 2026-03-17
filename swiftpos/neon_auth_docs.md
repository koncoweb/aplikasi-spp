> This page location: Backend > Neon Auth > Introduction > Overview
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Neon Auth

Managed authentication that branches with your database

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Neon Auth is a managed authentication service that stores users, sessions, and auth configuration directly in your Neon database. When you branch your database, your entire auth state branches with it. This lets you test real authentication workflows in preview environments.

**Note: Before you start** Neon Auth is in active development. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's supported and what's coming next.

## Why Neon Auth?

- **Identity lives in your database**  
  All authentication data is stored in the `neon_auth` schema. It's queryable with SQL and compatible with Row Level Security (RLS) policies.

- **Zero server management**  
  Neon Auth runs as a managed REST API service. Configure settings in the Console; use the [client SDK](https://neon.com/docs/reference/javascript-sdk) or [server SDK](https://neon.com/docs/auth/reference/nextjs-server) in your app. No infrastructure to maintain.

- **Auth that branches with your data**  
  Test sign-up, login, password reset, and OAuth flows in isolated branches without touching production data.

## Built on Better Auth

Neon Auth is powered by [Better Auth](https://www.better-auth.com/), which means you get familiar APIs. You can use Better Auth UI components or call auth methods directly to build your own UI.

Neon Auth currently supports Better Auth version **1.4.18**.

### When to use Neon Auth vs. self-hosting Better Auth

Neon Auth is a managed authentication service that integrates seamlessly with Neon's architecture and offerings:

- **Branch-aware authentication**: Every Neon branch gets its own isolated auth environment, so you can test authentication features without affecting your production branch.
- **Built-in Data API integration**: JWT token validation for the Data API has native support for Neon Auth.
- **No infrastructure to manage**: Neon Auth is deployed in the same region as your database, reducing latency without requiring you to run auth infrastructure.
- **Shared OAuth credentials for testing**: Get started quickly with out-of-the-box Google OAuth credentials, eliminating the setup complexity for testing and prototyping.

Self-hosting Better Auth makes sense if you need:

- Flexibility in auth configuration: custom plugins, hooks, and options not yet supported by Neon Auth.
- Full control over your auth code and the ability to run it inside your own infrastructure.

For more details on the SDK differences between `@neondatabase/auth` and `better-auth/client`, see [Why use @neondatabase/auth over better-auth/client](https://github.com/neondatabase/neon-js/blob/main/packages/auth/neon-auth_vs_better-auth.md).

As Neon Auth evolves, more Better Auth integrations and features will be added. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's currently supported and what's coming next.

## Basic usage

Enable Auth in your Neon project, then add authentication to your app.

**For Next.js (server-side):**

See the [Next.js Server SDK reference](https://neon.com/docs/auth/reference/nextjs-server) for complete API documentation.

```typescript filename="lib/auth/server.ts"
import { createNeonAuth } from '@neondatabase/auth/next/server';

export const auth = createNeonAuth({
  baseUrl: process.env.NEON_AUTH_BASE_URL!,
  cookies: { secret: process.env.NEON_AUTH_COOKIE_SECRET! },
});
```

```typescript filename="app/api/auth/[...path]/route.ts"
import { auth } from '@/lib/auth/server';

export const { GET, POST } = auth.handler();
```

**For React/Vite (client-side):**

See the [Client SDK reference](https://neon.com/docs/reference/javascript-sdk) for complete API documentation.

```typescript filename="src/auth.ts"
import { createAuthClient } from '@neondatabase/neon-js/auth';

export const authClient = createAuthClient(import.meta.env.VITE_NEON_AUTH_URL);
```

```tsx filename="src/App.tsx"
import { NeonAuthUIProvider, AuthView } from '@neondatabase/neon-js/auth/react/ui';
import { authClient } from './auth';

export default function App() {
  return (
    <NeonAuthUIProvider authClient={authClient}>
      <AuthView pathname="sign-in" />
    </NeonAuthUIProvider>
  );
}
```

## Use cases

- **Production authentication**  
  Use Neon Auth as the identity system for your app. Store users, sessions, and OAuth configuration directly in Postgres, and pair with RLS for secure, database-centric access control.

- **Preview environments**  
  Test full authentication flows in Vercel previews with real users and sessions

- **Multi-tenant SaaS**  
  Test complex org and role hierarchies safely in isolated branches

- **CI/CD workflows**  
  Run end-to-end auth tests without touching production. The [Neon Create Branch GitHub Action](https://github.com/marketplace/actions/neon-create-branch-github-action) supports retrieving branch-specific auth URLs for testing authentication flows in GitHub Actions workflows.

- **Development workflows**  
  Spin up complete environments instantly with database and auth together

See [Branching authentication](https://neon.com/docs/auth/branching-authentication) for details on how auth branches with your database.

## Quick start guides

Choose your framework to get started:

- [Next.js](https://neon.com/docs/auth/quick-start/nextjs): With UI components
- [React (API methods)](https://neon.com/docs/auth/quick-start/react): Build your own auth UI
- [React](https://neon.com/docs/auth/quick-start/react-router-components): With UI components
- [TanStack Router](https://neon.com/docs/auth/quick-start/tanstack-router): With UI components

## Availability

Neon Auth is currently available for AWS regions only. Azure support is not yet available.

Neon Auth does not currently support projects with [IP Allow](https://neon.com/docs/manage/projects#configure-ip-allow) or [Private Networking](https://neon.com/docs/guides/neon-private-networking) enabled.

## Pricing

Neon Auth is included in all Neon plans based on Monthly Active Users (MAU):

- **Free**: Up to 60,000 MAU
- **Launch**: Up to 1M MAU
- **Scale**: Up to 1M MAU

An MAU (Monthly Active User) is a unique user who authenticates at least once during a monthly billing period. If you need more than 1M MAU, request an increase in the [console feedback form](https://console.neon.tech/app/settings?modal=feedback\&modalparams=%22Neon%20auth%20limit%20increase%22).

See [Neon plans](https://neon.com/docs/introduction/plans#auth) for more details.

## Migration from Stack Auth

If you're using the previous Neon Auth implementation via Stack Auth, your version will continue to work. When you're ready to migrate to the new Better Auth implementation, see our [migration guide](https://neon.com/docs/auth/migrate/from-legacy-auth).

---

## Related docs (Introduction)

- [Authentication Flow](https://neon.com/docs/auth/authentication-flow)
- [Branching Authentication](https://neon.com/docs/auth/branching-authentication)
- [Roadmap](https://neon.com/docs/auth/roadmap)
> This page location: Backend > Neon Auth > Introduction > Overview
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Neon Auth

Managed authentication that branches with your database

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Neon Auth is a managed authentication service that stores users, sessions, and auth configuration directly in your Neon database. When you branch your database, your entire auth state branches with it. This lets you test real authentication workflows in preview environments.

**Note: Before you start** Neon Auth is in active development. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's supported and what's coming next.

## Why Neon Auth?

- **Identity lives in your database**  
  All authentication data is stored in the `neon_auth` schema. It's queryable with SQL and compatible with Row Level Security (RLS) policies.

- **Zero server management**  
  Neon Auth runs as a managed REST API service. Configure settings in the Console; use the [client SDK](https://neon.com/docs/reference/javascript-sdk) or [server SDK](https://neon.com/docs/auth/reference/nextjs-server) in your app. No infrastructure to maintain.

- **Auth that branches with your data**  
  Test sign-up, login, password reset, and OAuth flows in isolated branches without touching production data.

## Built on Better Auth

Neon Auth is powered by [Better Auth](https://www.better-auth.com/), which means you get familiar APIs. You can use Better Auth UI components or call auth methods directly to build your own UI.

Neon Auth currently supports Better Auth version **1.4.18**.

### When to use Neon Auth vs. self-hosting Better Auth

Neon Auth is a managed authentication service that integrates seamlessly with Neon's architecture and offerings:

- **Branch-aware authentication**: Every Neon branch gets its own isolated auth environment, so you can test authentication features without affecting your production branch.
- **Built-in Data API integration**: JWT token validation for the Data API has native support for Neon Auth.
- **No infrastructure to manage**: Neon Auth is deployed in the same region as your database, reducing latency without requiring you to run auth infrastructure.
- **Shared OAuth credentials for testing**: Get started quickly with out-of-the-box Google OAuth credentials, eliminating the setup complexity for testing and prototyping.

Self-hosting Better Auth makes sense if you need:

- Flexibility in auth configuration: custom plugins, hooks, and options not yet supported by Neon Auth.
- Full control over your auth code and the ability to run it inside your own infrastructure.

For more details on the SDK differences between `@neondatabase/auth` and `better-auth/client`, see [Why use @neondatabase/auth over better-auth/client](https://github.com/neondatabase/neon-js/blob/main/packages/auth/neon-auth_vs_better-auth.md).

As Neon Auth evolves, more Better Auth integrations and features will be added. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's currently supported and what's coming next.

## Basic usage

Enable Auth in your Neon project, then add authentication to your app.

**For Next.js (server-side):**

See the [Next.js Server SDK reference](https://neon.com/docs/auth/reference/nextjs-server) for complete API documentation.

```typescript filename="lib/auth/server.ts"
import { createNeonAuth } from '@neondatabase/auth/next/server';

export const auth = createNeonAuth({
  baseUrl: process.env.NEON_AUTH_BASE_URL!,
  cookies: { secret: process.env.NEON_AUTH_COOKIE_SECRET! },
});
```

```typescript filename="app/api/auth/[...path]/route.ts"
import { auth } from '@/lib/auth/server';

export const { GET, POST } = auth.handler();
```

**For React/Vite (client-side):**

See the [Client SDK reference](https://neon.com/docs/reference/javascript-sdk) for complete API documentation.

```typescript filename="src/auth.ts"
import { createAuthClient } from '@neondatabase/neon-js/auth';

export const authClient = createAuthClient(import.meta.env.VITE_NEON_AUTH_URL);
```

```tsx filename="src/App.tsx"
import { NeonAuthUIProvider, AuthView } from '@neondatabase/neon-js/auth/react/ui';
import { authClient } from './auth';

export default function App() {
  return (
    <NeonAuthUIProvider authClient={authClient}>
      <AuthView pathname="sign-in" />
    </NeonAuthUIProvider>
  );
}
```

## Use cases

- **Production authentication**  
  Use Neon Auth as the identity system for your app. Store users, sessions, and OAuth configuration directly in Postgres, and pair with RLS for secure, database-centric access control.

- **Preview environments**  
  Test full authentication flows in Vercel previews with real users and sessions

- **Multi-tenant SaaS**  
  Test complex org and role hierarchies safely in isolated branches

- **CI/CD workflows**  
  Run end-to-end auth tests without touching production. The [Neon Create Branch GitHub Action](https://github.com/marketplace/actions/neon-create-branch-github-action) supports retrieving branch-specific auth URLs for testing authentication flows in GitHub Actions workflows.

- **Development workflows**  
  Spin up complete environments instantly with database and auth together

See [Branching authentication](https://neon.com/docs/auth/branching-authentication) for details on how auth branches with your database.

## Quick start guides

Choose your framework to get started:

- [Next.js](https://neon.com/docs/auth/quick-start/nextjs): With UI components
- [React (API methods)](https://neon.com/docs/auth/quick-start/react): Build your own auth UI
- [React](https://neon.com/docs/auth/quick-start/react-router-components): With UI components
- [TanStack Router](https://neon.com/docs/auth/quick-start/tanstack-router): With UI components

## Availability

Neon Auth is currently available for AWS regions only. Azure support is not yet available.

Neon Auth does not currently support projects with [IP Allow](https://neon.com/docs/manage/projects#configure-ip-allow) or [Private Networking](https://neon.com/docs/guides/neon-private-networking) enabled.

## Pricing

Neon Auth is included in all Neon plans based on Monthly Active Users (MAU):

- **Free**: Up to 60,000 MAU
- **Launch**: Up to 1M MAU
- **Scale**: Up to 1M MAU

An MAU (Monthly Active User) is a unique user who authenticates at least once during a monthly billing period. If you need more than 1M MAU, request an increase in the [console feedback form](https://console.neon.tech/app/settings?modal=feedback\&modalparams=%22Neon%20auth%20limit%20increase%22).

See [Neon plans](https://neon.com/docs/introduction/plans#auth) for more details.

## Migration from Stack Auth

If you're using the previous Neon Auth implementation via Stack Auth, your version will continue to work. When you're ready to migrate to the new Better Auth implementation, see our [migration guide](https://neon.com/docs/auth/migrate/from-legacy-auth).

---

## Related docs (Introduction)

- [Authentication Flow](https://neon.com/docs/auth/authentication-flow)
- [Branching Authentication](https://neon.com/docs/auth/branching-authentication)
- [Roadmap](https://neon.com/docs/auth/roadmap)

> This page location: Backend > Neon Auth > Introduction > Overview
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Neon Auth

Managed authentication that branches with your database

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Neon Auth is a managed authentication service that stores users, sessions, and auth configuration directly in your Neon database. When you branch your database, your entire auth state branches with it. This lets you test real authentication workflows in preview environments.

**Note: Before you start** Neon Auth is in active development. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's supported and what's coming next.

## Why Neon Auth?

- **Identity lives in your database**  
  All authentication data is stored in the `neon_auth` schema. It's queryable with SQL and compatible with Row Level Security (RLS) policies.

- **Zero server management**  
  Neon Auth runs as a managed REST API service. Configure settings in the Console; use the [client SDK](https://neon.com/docs/reference/javascript-sdk) or [server SDK](https://neon.com/docs/auth/reference/nextjs-server) in your app. No infrastructure to maintain.

- **Auth that branches with your data**  
  Test sign-up, login, password reset, and OAuth flows in isolated branches without touching production data.

## Built on Better Auth

Neon Auth is powered by [Better Auth](https://www.better-auth.com/), which means you get familiar APIs. You can use Better Auth UI components or call auth methods directly to build your own UI.

Neon Auth currently supports Better Auth version **1.4.18**.

### When to use Neon Auth vs. self-hosting Better Auth

Neon Auth is a managed authentication service that integrates seamlessly with Neon's architecture and offerings:

- **Branch-aware authentication**: Every Neon branch gets its own isolated auth environment, so you can test authentication features without affecting your production branch.
- **Built-in Data API integration**: JWT token validation for the Data API has native support for Neon Auth.
- **No infrastructure to manage**: Neon Auth is deployed in the same region as your database, reducing latency without requiring you to run auth infrastructure.
- **Shared OAuth credentials for testing**: Get started quickly with out-of-the-box Google OAuth credentials, eliminating the setup complexity for testing and prototyping.

Self-hosting Better Auth makes sense if you need:

- Flexibility in auth configuration: custom plugins, hooks, and options not yet supported by Neon Auth.
- Full control over your auth code and the ability to run it inside your own infrastructure.

For more details on the SDK differences between `@neondatabase/auth` and `better-auth/client`, see [Why use @neondatabase/auth over better-auth/client](https://github.com/neondatabase/neon-js/blob/main/packages/auth/neon-auth_vs_better-auth.md).

As Neon Auth evolves, more Better Auth integrations and features will be added. Check the [roadmap](https://neon.com/docs/auth/roadmap) to see what's currently supported and what's coming next.

## Basic usage

Enable Auth in your Neon project, then add authentication to your app.

**For Next.js (server-side):**

See the [Next.js Server SDK reference](https://neon.com/docs/auth/reference/nextjs-server) for complete API documentation.

```typescript filename="lib/auth/server.ts"
import { createNeonAuth } from '@neondatabase/auth/next/server';

export const auth = createNeonAuth({
  baseUrl: process.env.NEON_AUTH_BASE_URL!,
  cookies: { secret: process.env.NEON_AUTH_COOKIE_SECRET! },
});
```

```typescript filename="app/api/auth/[...path]/route.ts"
import { auth } from '@/lib/auth/server';

export const { GET, POST } = auth.handler();
```

**For React/Vite (client-side):**

See the [Client SDK reference](https://neon.com/docs/reference/javascript-sdk) for complete API documentation.

```typescript filename="src/auth.ts"
import { createAuthClient } from '@neondatabase/neon-js/auth';

export const authClient = createAuthClient(import.meta.env.VITE_NEON_AUTH_URL);
```

```tsx filename="src/App.tsx"
import { NeonAuthUIProvider, AuthView } from '@neondatabase/neon-js/auth/react/ui';
import { authClient } from './auth';

export default function App() {
  return (
    <NeonAuthUIProvider authClient={authClient}>
      <AuthView pathname="sign-in" />
    </NeonAuthUIProvider>
  );
}
```

## Use cases

- **Production authentication**  
  Use Neon Auth as the identity system for your app. Store users, sessions, and OAuth configuration directly in Postgres, and pair with RLS for secure, database-centric access control.

- **Preview environments**  
  Test full authentication flows in Vercel previews with real users and sessions

- **Multi-tenant SaaS**  
  Test complex org and role hierarchies safely in isolated branches

- **CI/CD workflows**  
  Run end-to-end auth tests without touching production. The [Neon Create Branch GitHub Action](https://github.com/marketplace/actions/neon-create-branch-github-action) supports retrieving branch-specific auth URLs for testing authentication flows in GitHub Actions workflows.

- **Development workflows**  
  Spin up complete environments instantly with database and auth together

See [Branching authentication](https://neon.com/docs/auth/branching-authentication) for details on how auth branches with your database.

## Quick start guides

Choose your framework to get started:

- [Next.js](https://neon.com/docs/auth/quick-start/nextjs): With UI components
- [React (API methods)](https://neon.com/docs/auth/quick-start/react): Build your own auth UI
- [React](https://neon.com/docs/auth/quick-start/react-router-components): With UI components
- [TanStack Router](https://neon.com/docs/auth/quick-start/tanstack-router): With UI components

## Availability

Neon Auth is currently available for AWS regions only. Azure support is not yet available.

Neon Auth does not currently support projects with [IP Allow](https://neon.com/docs/manage/projects#configure-ip-allow) or [Private Networking](https://neon.com/docs/guides/neon-private-networking) enabled.

## Pricing

Neon Auth is included in all Neon plans based on Monthly Active Users (MAU):

- **Free**: Up to 60,000 MAU
- **Launch**: Up to 1M MAU
- **Scale**: Up to 1M MAU

An MAU (Monthly Active User) is a unique user who authenticates at least once during a monthly billing period. If you need more than 1M MAU, request an increase in the [console feedback form](https://console.neon.tech/app/settings?modal=feedback\&modalparams=%22Neon%20auth%20limit%20increase%22).

See [Neon plans](https://neon.com/docs/introduction/plans#auth) for more details.

## Migration from Stack Auth

If you're using the previous Neon Auth implementation via Stack Auth, your version will continue to work. When you're ready to migrate to the new Better Auth implementation, see our [migration guide](https://neon.com/docs/auth/migrate/from-legacy-auth).

---

## Related docs (Introduction)

- [Authentication Flow](https://neon.com/docs/auth/authentication-flow)
- [Branching Authentication](https://neon.com/docs/auth/branching-authentication)
- [Roadmap](https://neon.com/docs/auth/roadmap)

> This page location: Backend > Neon Auth > Introduction > Authentication Flow
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Authentication flow

Understanding the complete sign-in and sign-up process

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

This guide explains the authentication flow: how sign-in works from SDK call to session creation.

**Note:** Anyone can sign up for your application by default. Support for restricted signups is coming soon. Until then, consider adding a verification step by enabling [email verification](https://neon.com/docs/auth/guides/email-verification) via verification link or verification code.

## Architecture overview

Neon Auth is a managed REST API service built on Better Auth that connects directly to your Neon database. You use the SDK in your application and configure settings in the Console; no servers to manage.

```
Your App (SDK)
    ↓ HTTP requests
Neon Auth Service (REST API)
    ↓ connects to database
Your Neon Database (neon_auth schema)
```

All authentication data (users, sessions, OAuth configurations) lives in your database's `neon_auth` schema. You can query these tables directly with SQL for debugging, analytics, or custom logic.

## Complete sign-in flow

Here's what happens when a user signs in, from your code to the database:

## User signs in

Your application calls the SDK's sign-in method:

```typescript
const { data, error } = await client.auth.signIn.email({
  email: 'user@example.com',
  password: 'password',
});
```

The SDK posts to `{NEON_AUTH_URL}/auth/sign-in/email`. The Auth service validates credentials against `neon_auth.account`, creates a session in `neon_auth.session`, and returns the session cookie with user data.

**Response you receive:**

```typescript
{
  data: {
    session: {
      access_token: "eyJhbGc...",  // JWT token
      expires_at: 1763848395,
      // ... other session fields
    },
    user: {
      id: "dc42fa70-09a7-4038-a3bb-f61dda854910",
      email: "user@example.com",
      emailVerified: true,
      // ... other user fields
    }
  }
}
```

## Session cookie is set

The Auth service sets an HTTP-only cookie (`__Secure-neonauth.session_token`) in your browser. This cookie:

- Contains an opaque session token (not a JWT)
- Is automatically sent with every request to the Auth API
- Is secure (HTTPS only, HttpOnly, SameSite=None)
- Is managed entirely by the SDK; you never touch it

**Where to see it:** Open DevTools → Application → Cookies → look for `__Secure-neonauth.session_token`

## JWT token is retrieved

The SDK automatically retrieves a JWT token and stores it in `session.access_token`. You don't need to call `/auth/token` separately; the SDK handles this behind the scenes.

**What's in the JWT:**

```json
{
  "sub": "dc42fa70-09a7-4038-a3bb-f61dda854910", // User ID
  "email": "user@example.com",
  "role": "authenticated",
  "exp": 1763848395, // Expiration timestamp
  "iat": 1763847495 // Issued at timestamp
}
```

The `sub` claim contains the user ID from `neon_auth.user.id`. This is what Row Level Security policies use to identify the current user.

## JWT is used for database queries

When you query your database via Data API, the SDK automatically includes the JWT in the `Authorization` header:

```typescript
// JWT is automatically included in Authorization header
const { data } = await client.from('posts').select('*');
```

**What happens:**

1. SDK gets JWT from `session.access_token`
2. Adds `Authorization: Bearer <jwt-token>` header
3. Data API validates JWT signature using JWKS public keys
4. Data API extracts user ID from JWT and makes it available to RLS policies
5. Your query runs with the authenticated user context

## Sign-up flow

The sign-up flow creates a new user:

```typescript
const { data, error } = await client.auth.signUp.email({
  email: 'newuser@example.com',
  password: 'securepassword',
  name: 'New User',
});
```

The SDK posts to `{NEON_AUTH_URL}/auth/sign-up/email`. The Auth service creates a new row in `neon_auth.user`, stores hashed credentials in `neon_auth.account`, and returns user data. If email verification is required, it creates a verification token in `neon_auth.verification` and may delay session creation until verification.

**Note:** By default, anyone can sign up for your application. To add an additional verification layer, enable email verification (see [Email Verification](https://neon.com/docs/auth/guides/email-verification)). Built-in signup restrictions are coming soon.

## OAuth flow

OAuth authentication (Google, GitHub, Vercel, etc.):

```typescript
await client.auth.signIn.social({
  provider: 'google',
  callbackURL: 'http://localhost:3000/auth/callback',
});
```

The SDK redirects to the OAuth provider. After the user authenticates, the provider redirects back with an authorization code. The SDK exchanges the code for an access token, then the Auth service creates or updates the user in `neon_auth.user`, stores OAuth tokens in `neon_auth.account`, and creates a session.

## Database as source of truth

Neon Auth stores all data in your database's `neon_auth` schema:

- Changes are immediate (no sync delays)
- Query auth data directly with SQL
- Each branch has isolated auth data
- You own your data

```sql
SELECT id, email, "emailVerified", "createdAt"
FROM neon_auth.user
ORDER BY "createdAt" DESC;
```

## Data API integration

When you enable the [Data API](https://neon.com/docs/data-api/get-started), JWT tokens from Neon Auth are validated automatically. The user ID is available via the `auth.uid()` function, enabling Row-Level Security policies to grant data access based on the authenticated user.

**Example RLS policy:**

```sql
CREATE POLICY "Users can view own posts"
ON posts FOR SELECT TO authenticated
USING (user_id = auth.uid());
```

**Learn more about securing your data:**

- [Row-Level Security with Neon](https://neon.com/docs/guides/row-level-security)
- [Simplify RLS with Drizzle](https://neon.com/docs/guides/rls-drizzle)

## What's next

- [Branching Authentication](https://neon.com/docs/auth/branching-authentication): How auth works with database branches
- [Row Level Security](https://neon.com/docs/guides/row-level-security): Secure your data with RLS policies

---

## Related docs (Introduction)

- [Overview](https://neon.com/docs/auth/overview)
- [Branching Authentication](https://neon.com/docs/auth/branching-authentication)
- [Roadmap](https://neon.com/docs/auth/roadmap)

> This page location: Backend > Neon Auth > Introduction > Branching Authentication
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Branching authentication

How authentication works with Neon database branches

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Authentication is often one of the hardest parts of the application stack to test. In traditional architectures, identity data lives in a separate third-party service, while your business data lives in your database. This separation makes it difficult to create realistic staging environments or test changes to permissions without affecting production users.

One of Neon Auth's unique features is native support for [database branching](https://neon.com/docs/introduction/branching). Because authentication data (users, sessions, and configuration) lives directly in your database's `neon_auth` schema, it is cloned along with your business data when you create a branch.

This gives each branch its own isolated authentication environment, enabling safe testing of permission changes, new OAuth providers, or full application refactors.

**Info:** Neon Auth branching is also supported via API. See the [Neon Auth API reference](https://api-docs.neon.tech/reference/getneonauth) for a full set of REST API endpoints.

## How it works

When you create a [database branch](https://neon.com/docs/introduction/branching), you get an exact copy of all authentication data from the parent branch at that point in time:

```
Production (main)                Preview Branch (preview-pr-123)
├── Users                   →    ├── Users (copied at branch time)
├── Sessions                →    ├── Sessions (copied, but will expire)
├── Configuration           →    ├── Configuration (independent copy)
├── OAuth providers         →    ├── OAuth providers (same credentials)
├── JWKS keys               →    ├── JWKS keys (copied)
└── Organizations           →    └── Organizations (copied)
```

After branching, the environments operate independently:

1. **Data Isolation:** Changes in one branch don't affect others. Creating a user in a preview branch does not create them in production.
2. **Config Isolation:** You can modify auth settings (for example, email templates, token settings) in the branch without affecting the parent.
3. **Endpoint Isolation:** Each branch gets a unique Auth API URL. Tokens issued in one branch are not valid in another.

```
Production Branch              Preview Branch
├── New user: alice@co.com     ├── New user: test@co.com
├── Alice's sessions           ├── Test user's sessions
├── Config: email with links   ├── Config: testing email codes
└── ep-abc123.neonauth...      └── ep-xyz789.neonauth...
    (production endpoint)          (preview endpoint)
```

**Note:** Neon Auth works with your branch's **default** database (typically `neondb`) and read-write endpoint only. You cannot use Neon Auth with other databases in the same branch. This aligns with our recommended pattern of one database per branch.

## Session management details

Sessions do not transfer between branches. If you sign in to your production app and then visit your staging environment:

1. The session _record_ exists in the staging database (if it was created before the branch happened).
2. However, your browser's _cookie_ is scoped to the production domain.
3. Therefore, you will need to sign in again on the staging environment.

This isolation is intentional and prevents security issues like sessions accidentally working across environments or test actions affecting production users.

## Common Use Cases

This branch isolation enables several powerful workflows for developers, QA teams, and product managers.

### 1. Developer isolation

In a team environment, developers often step on each other's toes when sharing a single development database. With branching, each developer can have their own instance:

```bash filename="Terminal"
# Alice and Bob create their own branches
neon branches create --name dev-alice
neon branches create --name dev-bob
```

- **Alice** works on a "Delete Account" flow. She can delete users and test the full flow without worrying about affecting others.
- **Bob** works on the "User Dashboard". His user list remains intact, even though Alice is deleting users in her environment.

Because Neon Auth is part of the database, Alice and Bob don't need to set up separate auth providers or mock data. They can work in parallel without conflicts.

### 2. Testing auth configuration safely

Say you want to add Google OAuth to your production app, but you're not sure if your configuration will work. Instead of testing directly in production, create a branch:

```bash filename="Terminal"
# Create test branch from production
neon branches create --name test-google-oauth
```

```env filename=".env.local"
# Point your local app to the test branch's Auth URL
VITE_NEON_AUTH_URL=https://ep-test-google-oauth.neonauth.region.aws.neon.tech/neondb/auth
```

Now configure Google OAuth in the test branch's Console and verify the sign-in flow works locally. Your production app and users are completely unaffected. Once you confirm it works, apply the same OAuth settings to your production branch.

The same approach works for any auth changes: password reset flows, email verification settings, or testing with anonymized production data.

### 3. Preview environments for pull requests

When building full-stack applications, you often deploy "Preview Deployments" (using Vercel, Netlify, etc.) for every Pull Request. Without Auth Branching, these previews usually share a single "Staging" auth tenant. This leads to data conflicts where one developer deletes a user that another developer was testing with.

**The workflow:**

You can automate this using GitHub Actions. When a PR is opened:

1. Create a Neon branch.
2. Deploy your frontend/backend to a preview URL.
3. Inject the **Branch Auth URL** into the preview deployment's environment variables.
4. Set the Redirect URLs in the branch's Auth configuration to point to the preview URL using the [Neon API](https://api-docs.neon.tech/reference/addbranchneonauthtrusteddomain).

Because the branch contains a snapshot of production data, the preview environment is fully functional immediately. You can log in with real test accounts that exist in production, but any actions taken (changing passwords, updating profiles) happen in isolation.

**Tip:** See the [GitHub Actions guide](https://neon.com/docs/guides/branching-github-actions) for instructions on how to automate branch creation for preview environments.

### 4. Testing multi-tenant & RBAC hierarchies

For applications with complex Role-Based Access Control (RBAC) or multi-tenant architectures, testing permission changes can be risky. A misconfiguration could lock out users or expose sensitive data.

**The scenario:** You are refactoring your RLS policies to allow "Managers" to view "Department" data, but not modify it.

**The workflow:**

1. Create a branch `refactor-rbac`.
2. This branch contains your real production users and their existing role assignments.
3. Modify your RLS policies in the branch.
4. You can log in as a "Manager" user and verify they can only view the appropriate data.
5. If the policy is incorrect and you accidentally expose data or lock a user out, **it only affects the branch**. Production users are never impacted.

### 5. Major refactors and "v2" betas

When rebuilding your application from scratch or launching a major "v2" update, you often need to validate the new system with real user data before the official switch-over. Traditionally, this required complex data dumps or asking users to re-register on a beta site.

With Neon Auth, you can spin up a complete parallel environment for your new version instantly.

**The workflow:**

1. **Branch production:** Create a branch named `v2-beta` from your main production database. This clones your entire application state, including the `neon_auth` schema containing all user identities and hashed passwords.
2. **Deploy v2:** Deploy your new application code (for example, to `beta.myapp.com`) and point it to the `v2-beta` branch's Auth URL.
3. **Seamless login:** Existing users can visit your new v2 site and **log in immediately using their existing credentials**. They do not need to sign up again or reset their passwords.

This allows you to test radical architectural changes such as renaming database columns, changing table structures, or modifying authentication flows in a live environment. Your v1 application remains completely unaffected, while your v2 beta feels like a production-ready extension of your platform.

**Note: Data separation** Remember that once branched, the environments are separate. If a user changes their password on the v2 site, it will not change on the v1 site, and vice versa. This makes this workflow ideal for "Public Betas" or staging environments prior to a final cutover.

### 6. AI agents and ephemeral sandboxes

AI Agents, particularly those designed for coding or QA, require safe, isolated environments to generate code, run migrations, and validate features. Traditionally, giving an agent access to a full authentication stack was complex - you had to mock auth tokens or risk exposing production user pools.

With Neon, an agent can programmatically provision its own "sandbox." Because Neon Auth moves with the data, this branch instantly creates a working Authentication service isolated from production, complete with its own user tables, sessions, and configuration. **This ensures your entire application stack mimics production behavior without risking real user data.**

**The workflow:**

1. **Provision:** The Agent uses the Neon API to create a new database branch. It instantly receives a dedicated Auth URL for that specific environment.
2. **Interact:** The Agent uses tools like Playwright or Puppeteer to interact with the application, registering new users and performing real login flows against the branch's auth service.
3. **Validate:** The Agent runs a test suite to verify that the code it generated works correctly with the database schema, RLS policies, and authentication rules.
4. **Teardown:** Once the task is complete, the Agent deletes the branch, cleaning up all data and auth state.

This capability allows agents to spin up "full stack" environments (Database + Auth + Compute) in seconds, enabling autonomous testing loops that rigorously test user-facing security without manual setup.

**Important:** An AI agent cannot log in as a real production user in a branch. Although user records are copied, valid session cookies are domain-scoped and remain with the user's browser; they are not sent to the branch URL. Unless the agent explicitly knows a user's password, it must either perform a sign-up flow or use existing test credentials to log in.

To streamline this, consider maintaining specific test users with known credentials in your production database; these records are automatically cloned to child branches during creation, allowing agents to log in immediately without needing to perform a sign-up step.

## What's Next

- [Database Branching](https://neon.com/docs/introduction/branching): Learn about database branching fundamentals
- [Branching with CLI](https://neon.com/docs/guides/branching-neon-cli): Create and manage branches with CLI
- [Branching with GitHub Actions](https://neon.com/docs/guides/branching-github-actions): Automate branching in CI/CD
- [Row-Level Security](https://neon.com/docs/guides/row-level-security): Secure data with RLS

---

## Related docs (Introduction)

- [Overview](https://neon.com/docs/auth/overview)
- [Authentication Flow](https://neon.com/docs/auth/authentication-flow)
- [Roadmap](https://neon.com/docs/auth/roadmap)

> This page location: Backend > Neon Auth > Guides > Email verification
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Email verification

Verify user email addresses during sign-up or account creation

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Email verification ensures users own the email addresses they register with. Neon Auth supports two verification methods:

- **Verification codes** (users enter a numeric code from their email) - works with shared or custom email providers
- **Verification links** (users click a link in their email) - requires a custom email provider

**Note:** Verification links require a [custom email provider](https://neon.com/docs/auth/production-checklist#email-provider). If you're using the shared email provider, use verification codes instead.

## Enable email verification

In your project's **Settings** → **Auth** page, enable **Sign-up with Email** and **Verify at Sign-up**. Choose your verification method.

![Email verification settings in Neon Console](https://neon.com/docs/auth/email-verification-settings.png)

## Verification links

Verification links require a custom email provider. See [Email provider configuration](https://neon.com/docs/auth/production-checklist#email-provider) to set this up.

When a user clicks the verification link in their email, the Neon Auth server handles verification and redirects them back to your application. Your app checks for the new session and shows the appropriate UI.

### 1. Check session on mount (#check-session-on-mount)

Add a session check when your component mounts to detect when a user returns from clicking the verification link:

```jsx filename="src/App.jsx" {9-14}
import { useEffect, useState } from 'react';
import { authClient } from './auth';

export default function App() {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    authClient.getSession().then(({ data }) => {
      if (data?.session) {
        setUser(data.session.user);
      }
      setLoading(false);
    });
  }, []);
}
```

### 2. Handle sign-up with verification (#handle-signup-with-verification)

After calling `signUp.email()`, check if verification is required and show a message:

```jsx {16-18} filename="src/App.jsx"
const handleSignUp = async (e) => {
  e.preventDefault();
  setMessage('');

  try {
    const { data, error } = await authClient.signUp.email({
      email,
      password,
      name: name || email.split('@')[0] || 'User',
    });

    if (error) throw error;

    // Check if email verification is required
    if (data?.user && !data.user.emailVerified) {
      setMessage('Check your email for a verification link!');
    } else {
      setMessage('Account created! Please sign in.');
    }
  } catch (error) {
    setMessage(error?.message || 'An error occurred');
  }
};
```

### 3. Check verification status (#check-verification-status)

Access the `emailVerified` field from the user object:

```jsx {3} filename="src/App.jsx"
const { data } = await authClient.getSession();

if (data?.session?.user && !data.session.user.emailVerified) {
  // Show verification prompt or restrict features
  console.log('Please verify your email to continue');
}
```

## Verification codes

If you prefer verification codes, users receive a numeric code via email and enter it in your application. Your app switches between the auth form and a verification form.

### 1. Add verification state (#add-verification-state)

Add state to track which form to show:

```jsx filename="src/App.jsx"
const [step, setStep] = useState('auth'); // 'auth' or 'verify'
const [code, setCode] = useState('');
```

### 2. Handle code verification (#handle-code-verification)

Create a handler for code verification:

```jsx {6-9} filename="src/App.jsx"
const handleVerify = async (e) => {
  e.preventDefault();
  setMessage('');

  try {
    const { data, error } = await authClient.emailOtp.verifyEmail({
      email,
      otp: code,
    });

    if (error) throw error;

    // Check if auto-sign-in is enabled (default behavior)
    if (data?.session) {
      setUser(data.session.user);
      setStep('auth');
    } else {
      setMessage('Email verified! You can now sign in.');
      setStep('auth');
      setIsSignUp(false);
      setCode('');
    }
  } catch (error) {
    setMessage(error?.message || 'An error occurred');
  }
};
```

### 3. Show verification form (#show-verification-form)

When `step` is `'verify'`, show the verification form:

```jsx filename="src/App.jsx"
if (step === 'verify') {
  return (
    <div>
      <h1>Verify Your Email</h1>
      <p>Enter the code sent to {email}</p>
      <form onSubmit={handleVerify}>
        <input
          type="text"
          placeholder="Verification code"
          value={code}
          onChange={(e) => setCode(e.target.value)}
          required
        />
        <button type="submit">Verify</button>
      </form>
      {message && <p>{message}</p>}
    </div>
  );
}
```

### 4. Switch to verification after sign-up (#switch-to-verification)

After calling `signUp.email()`, switch to the verification step:

```jsx {3} filename="src/App.jsx"
if (data?.user && !data.user.emailVerified) {
  setMessage('Check your email for a verification code');
  setStep('verify'); // Switch to verification form
}
```

<details>

<summary>Complete example: App.jsx with verification codes</summary>

Here's a complete, minimal `App.jsx` file that includes sign-up, sign-in, and verification code functionality:

```jsx filename="src/App.jsx"
import { useState, useEffect } from 'react';
import { authClient } from './auth';
import './App.css';

export default function App() {
  const [session, setSession] = useState(null);
  const [user, setUser] = useState(null);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(true);
  const [message, setMessage] = useState('');
  const [step, setStep] = useState('auth'); // 'auth' or 'verify'
  const [code, setCode] = useState('');
  const [isSignUp, setIsSignUp] = useState(true);

  useEffect(() => {
    authClient.getSession().then((result) => {
      if (result.data?.session && result.data?.user) {
        setSession(result.data.session);
        setUser(result.data.user);
      }
      setLoading(false);
    });
  }, []);

  const handleSignUp = async (e) => {
    e.preventDefault();
    setMessage('');
    const { data, error } = await authClient.signUp.email({
      email,
      password,
      name: email.split('@')[0] || 'User',
    });
    if (error) {
      setMessage(error.message);
      return;
    }
    if (data?.user && !data.user.emailVerified) {
      setMessage('Check your email for a verification code');
      setStep('verify'); // Switch to verification form
    } else {
      const sessionResult = await authClient.getSession();
      if (sessionResult.data?.session && sessionResult.data?.user) {
        setSession(sessionResult.data.session);
        setUser(sessionResult.data.user);
      }
    }
  };

  const handleSignIn = async (e) => {
    e.preventDefault();
    setMessage('');
    const { data, error } = await authClient.signIn.email({ email, password });
    if (error) {
      setMessage(error.message);
      return;
    }
    if (data?.session && data?.user) {
      setSession(data.session);
      setUser(data.user);
    }
  };
  const handleVerify = async (e) => {
    e.preventDefault();
    setMessage('');
    try {
      const { data, error } = await authClient.emailOtp.verifyEmail({
        email,
        otp: code,
      });
      if (error) throw error;
      if (data?.session) {
        setSession(data.session);
        setUser(data.session.user);
        setStep('auth');
      } else {
        setMessage('Email verified! You can now sign in.');
        setStep('auth');
        setIsSignUp(false);
        setCode('');
      }
    } catch (error) {
      setMessage(error?.message || 'An error occurred');
    }
  };

  const handleSignOut = async () => {
    await authClient.signOut();
    setSession(null);
    setUser(null);
  };

  if (loading) return <div>Loading...</div>;

  if (session && user) {
    return (
      <div>
        <h1>Logged in as {user.email}</h1>
        <button onClick={handleSignOut}>Sign Out</button>
      </div>
    );
  }
  if (step === 'verify') {
    return (
      <div>
        {' '}
        <h1>Verify Your Email</h1>
        <p>Enter the code sent to {email}</p>
        <form onSubmit={handleVerify}>
          {' '}
          <input
            type="text"
            placeholder="Verification code"
            value={code}
            onChange={(e) => setCode(e.target.value)}
            required
          />{' '}
          <button type="submit">Verify</button>
        </form>{' '}
        {message && <p>{message}</p>}
      </div>
    );
  }

  return (
    <div>
      <h1>{isSignUp ? 'Sign Up' : 'Sign In'}</h1>
      {message && <p>{message}</p>}
      <form onSubmit={isSignUp ? handleSignUp : handleSignIn}>
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
        <button type="submit">{isSignUp ? 'Sign Up' : 'Sign In'}</button>
      </form>
      <p>
        <button onClick={() => setIsSignUp(!isSignUp)}>
          {isSignUp ? 'Already have an account? Sign in' : "Don't have an account? Sign up"}
        </button>
      </p>
    </div>
  );
}
```

</details>

## Resending verification emails

Both verification links and verification codes expire after **15 minutes**. Allow users to request a new one:

```jsx {3-6} filename="src/App.jsx"
const handleResend = async () => {
  try {
    const { error } = await authClient.sendVerificationEmail({
      email,
      callbackURL: window.location.origin + '/',
    });

    if (error) throw error;
    setMessage('Verification email sent! Check your inbox.');
  } catch (error) {
    setMessage(error?.message || 'An error occurred');
  }
};
```

The server sends whichever type (verification link or verification code) you configured in the Console.

## Required vs optional verification

When email verification is **required** in your Console settings, users cannot sign in until they verify. When verification is **optional**, users can sign in immediately but their `emailVerified` field remains `false` until verified.

---

## Related docs (Guides)

- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)

> This page location: Backend > Neon Auth > Guides > Set up OAuth
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Set up OAuth

Add Google or GitHub sign-in to your application

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

OAuth lets users sign in with their Google, GitHub, or Vercel account. Neon Auth handles the OAuth flow and creates a session after authorization.

## Development mode

Google OAuth is enabled by default with shared credentials for development and testing. You can start using Google sign-in immediately without any configuration.

**Note:** GitHub and Vercel OAuth require custom credentials and is not available with shared credentials. See [Production setup](https://neon.com/docs/auth/guides/setup-oauth#production-setup) to configure your own OAuth apps.

For production, configure your own OAuth app credentials for both providers. See [Production setup](https://neon.com/docs/auth/guides/setup-oauth#production-setup) below.

## Sign in with OAuth

Call `signIn.social()` with your provider (`"google"`, `"github"` or `"vercel"`). The SDK redirects the user to the provider's authorization page, then back to your `callbackURL`:

Tab: Google

```jsx {6} filename="src/App.jsx"
import { authClient } from './auth';

const handleGoogleSignIn = async () => {
  try {
    await authClient.signIn.social({
      provider: "google",
      callbackURL: window.location.origin,
    });
  } catch (error) {
    console.error("Google sign-in error:", error);
  }
};
```

Tab: GitHub

```jsx {6} filename="src/App.jsx"
import { authClient } from './auth';

const handleGitHubSignIn = async () => {
  try {
    await authClient.signIn.social({
      provider: "github",
      callbackURL: window.location.origin,
    });
  } catch (error) {
    console.error("GitHub sign-in error:", error);
  }
};
```

Tab: Vercel

```jsx {6} filename="src/App.jsx"
import { authClient } from './auth';

const handleVercelSignIn = async () => {
  try {
    await authClient.signIn.social({
      provider: "vercel",
      callbackURL: window.location.origin,
    });
  } catch (error) {
    console.error("Vercel sign-in error:", error);
  }
};
```

## Handle the callback

After the provider redirects back to your app, check for a session:

```jsx {4-9} filename="src/App.jsx"
import { authClient } from './auth';

useEffect(() => {
  authClient.getSession().then(({ data }) => {
    if (data?.session) {
      setUser(data.session.user);
    }
    setLoading(false);
  });
}, []);
```

## Custom redirect URLs

Specify different URLs for new users or errors:

```jsx {3-5} filename="src/App.jsx"
await authClient.signIn.social({
  provider: "google", // or "github", "vercel"
  callbackURL: "/dashboard",
  newUserCallbackURL: "/welcome",
  errorCallbackURL: "/error",
});
```

## Production setup

For production, configure your own OAuth app credentials. GitHub and Vercel OAuth require custom credentials, while Google OAuth works with shared credentials for development but should use custom credentials in production.

1. Create OAuth apps with your providers:
   - [Google OAuth setup](https://developers.google.com/identity/protocols/oauth2/web-server)
   - [GitHub OAuth setup](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/creating-an-oauth-app)
   - [Vercel OAuth setup](https://vercel.com/docs/sign-in-with-vercel/manage-from-dashboard#create-an-app)
2. In your project's **Settings** → **Auth** page, configure your Client ID and Client Secret for each provider

Your app will automatically use your configured credentials

- [Email Verification](https://neon.com/docs/auth/guides/email-verification): Add email verification

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)

> This page location: Backend > Neon Auth > Guides > Password reset
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Password reset

Allow users to reset forgotten passwords

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Password reset allows users to securely reset forgotten passwords. Neon Auth supports password reset via verification links sent to the user's email address.

## Enable password reset

In your project's **Settings** → **Auth** page, ensure **Sign-up with Email** is enabled. Password reset is automatically available when email authentication is enabled.

## Using UI components

The easiest way to add password reset is using the pre-built UI components `<ForgotPasswordForm>` and `<ResetPasswordForm>`.

### 1. Enable forgot password in AuthView (#enable-forgot-password-authview)

If you're using `<AuthView>`, enable the forgot password flow:

```tsx filename="src/App.tsx"
import { NeonAuthUIProvider } from '@neondatabase/neon-js/auth/react';
import { AuthView } from '@neondatabase/neon-js/auth/react/ui';
import { authClient } from './auth';

export default function App() {
  return (
    <NeonAuthUIProvider authClient={authClient}>
      <AuthView pathname="sign-in" credentials={{ forgotPassword: true }} />
    </NeonAuthUIProvider>
  );
}
```

The `<AuthView>` component automatically includes a "Forgot password?" link when `forgotPassword` is enabled.

### 2. Use standalone form components (#use-standalone-forms)

For more control, use `<ForgotPasswordForm>` and `<ResetPasswordForm>` separately:

```tsx filename="src/App.tsx"
import { useState } from 'react';
import { ForgotPasswordForm, ResetPasswordForm } from '@neondatabase/neon-js/auth/react/ui';
import { authClient } from './auth';

export default function App() {
  const [step, setStep] = useState<'forgot' | 'reset'>('forgot');
  const [email, setEmail] = useState('');

  if (step === 'forgot') {
    return (
      <ForgotPasswordForm
        authClient={authClient}
        redirectTo={`${window.location.origin}/reset-password`}
        onSuccess={(data) => {
          setEmail(data.email);
          setStep('reset');
        }}
      />
    );
  }

  return (
    <ResetPasswordForm
      authClient={authClient}
      email={email}
      onSuccess={() => {
        setStep('forgot');
        // Redirect to sign-in or show success message
      }}
    />
  );
}
```

**Note:** SDK methods for password reset (`resetPasswordForEmail`) are not fully supported yet. Use the UI components (`<ForgotPasswordForm>` and `<ResetPasswordForm>`) for password reset functionality.

## Password reset flow

The complete password reset flow works as follows:

1. **User requests reset**: User enters their email and clicks "Send reset link"
2. **Email sent**: User receives a verification link with a reset token
3. **User clicks link**: User is redirected to your app's reset password page
4. **User enters new password**: User submits the new password
5. **Password reset**: Password is updated and user is signed in (if auto-sign-in is enabled)

## Reset link expiration

Password reset links expire after **15 minutes**. If a link expires, users need to request a new one.

## Next steps

- [Add email verification](https://neon.com/docs/auth/guides/email-verification) to ensure users own their email addresses
- [Learn how to branch your auth](https://neon.com/docs/auth/branching-authentication) to use database branches with isolated auth environments

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)

> This page location: Backend > Neon Auth > Guides > User management
> Full Neon documentation index: https://neon.com/docs/llms.txt

# User management

Update profiles, change passwords, and manage account settings

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Manage user profiles and account settings after users sign in. This guide covers:

- Updating profile information (name, image, phone number)
- Changing passwords securely
- Changing email addresses with verification
- Deleting user accounts

## Update user profile

Update user profile fields like name, image, or phone number using `updateUser()`:

```jsx filename="src/App.jsx"
import { authClient } from './auth';

const handleUpdateProfile = async (e) => {
  e.preventDefault();
  setMessage('');

  try {
    const { data, error } = await authClient.updateUser({
      name: 'New Name',
    });

    if (error) throw error;

    // Refresh session to get updated user data
    const sessionResult = await authClient.getSession();
    if (sessionResult.data?.session) {
      setUser(sessionResult.data.session.user);
      setMessage('Profile updated successfully!');
    }
  } catch (error) {
    setMessage(error?.message || 'Update failed');
  }
};
```

### Available profile fields

You can update these fields with `updateUser()`:

- `name` (string) - User's display name

**Note:** Email address changes are not currently supported. To reset a forgotten password, see [Password Reset](https://neon.com/docs/auth/guides/password-reset).

## Change password

Change a user's password while they are logged in using `changePassword()`. This requires the current password for security:

```jsx filename="src/App.jsx"
import { authClient } from './auth';

const handleChangePassword = async (e) => {
  e.preventDefault();
  setMessage('');

  try {
    const { data, error } = await authClient.changePassword({
      newPassword: 'new-secure-password',
      currentPassword: 'current-password',
    });

    if (error) throw error;
    setMessage('Password changed successfully!');
  } catch (error) {
    setMessage(error?.message || 'Password change failed');
  }
};
```

### Revoke other sessions

Optionally sign out from all other devices when changing the password:

```jsx filename="src/App.jsx"
const { data, error } = await authClient.changePassword({
  newPassword: 'new-secure-password',
  currentPassword: 'current-password',
  revokeOtherSessions: true, // Signs out all other devices
});
```

**Note:** If a user forgot their password, use the password reset flow (`requestPasswordReset()` and `resetPassword()`) instead. See [Password Reset](https://neon.com/docs/auth/guides/password-reset).

## Refresh user data

After updating profile information, refresh the session to get the latest user data:

```jsx filename="src/App.jsx"
import { authClient } from './auth';

const refreshUser = async () => {
  const { data } = await authClient.getSession();
  if (data?.session) {
    setUser(data.session.user);
  }
};
```

Call `refreshUser()` after successful `updateUser()` calls to ensure your UI displays the latest information.

- [Password Reset](https://neon.com/docs/auth/guides/password-reset): Reset forgotten passwords
- [Email Verification](https://neon.com/docs/auth/guides/email-verification): Verify email addresses
- [Authentication Flow](https://neon.com/docs/auth/authentication-flow): Understand the auth flow

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)

> This page location: Backend > Neon Auth > Guides > Configure domains
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Configure trusted domains

Add your application domains to enable secure authentication redirects

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Add your application domains to Neon Auth's allowlist to enable OAuth and email verification redirects in production.

## Why domains are required

Neon Auth only redirects to domains in your allowlist. This prevents phishing attacks and unauthorized redirects by ensuring users are only sent to your legitimate application URLs.

Without adding your production domain, OAuth sign-in and verification links will fail when users try to access your application.

## Add a domain

1. Go to **Console → Auth → Configuration → Domains**
2. Enter your domain with protocol: `https://myapp.com`
3. Click **Add domain**

Repeat for each domain where your app runs.

**Note:** Include the protocol (`https://`) and omit trailing slashes. For example: `https://myapp.com` not `https://myapp.com/`

## Localhost is pre-configured

Development domains are automatically allowed, so you don't need to add them:

- `http://localhost:3000`
- `http://localhost:5173`
- Any `localhost` port

## Production domains

Add all domains where users access your application:

- `https://myapp.com`
- `https://www.myapp.com` (if you support www subdomain)
- `https://app.myapp.com` (if using a subdomain)

**Important:** Add each subdomain explicitly. Wildcards like `*.myapp.com` are not supported.

## Common issues

**Redirect blocked after OAuth sign-in:**

- Verify the domain is in your allowlist
- Ensure you included `https://` (not `http://` for production)
- Check spelling matches exactly (including www vs non-www)

**Verification link doesn't redirect:**

- Verification links use the same domain allowlist
- Add the domain where users should land after clicking the verification link

## Next steps

- [Production checklist](https://neon.com/docs/auth/production-checklist) - Complete setup for launch

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)
> This page location: Backend > Neon Auth > Guides > Webhooks
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Webhooks

Handle authentication events with custom server logic

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Neon Auth webhooks send HTTP POST requests to your server when authentication events occur.

By default, Neon Auth handles OTP and magic link delivery through its built-in email provider. Webhooks let you replace this with your own delivery channels (SMS, custom email templates, WhatsApp) so you control how verification messages reach your users. Webhooks also let you hook into the user creation lifecycle to validate signups before they happen or sync new user data to external systems like CRMs and analytics platforms.

## Supported events

| Event                | Type         | Trigger                                          | Use case                                            |
| -------------------- | ------------ | ------------------------------------------------ | --------------------------------------------------- |
| `send.otp`           | Blocking     | OTP code needs delivery                          | Custom OTP delivery via SMS or email service        |
| `send.magic_link`    | Blocking     | Magic link needs delivery                        | Custom link delivery via any channel                |
| `user.before_create` | Blocking     | User attempts to sign up (before database write) | Signup validation, allowlists, user data enrichment |
| `user.created`       | Non-blocking | User created in the database                     | Sync to CRM, analytics, post-signup workflows       |

**Blocking** events pause the auth flow until your server responds (or the timeout expires). **Non-blocking** events are fire-and-forget; failures do not affect the user.

When you subscribe to `send.otp` or `send.magic_link`, Neon Auth skips its built-in email delivery for that event. Your webhook handler is responsible for delivering the code or link.

## Configure webhooks

Configure webhooks per project and branch using the Neon API. Your webhook URL must use HTTPS protocol. See the API reference for [Get webhook configuration](https://api-docs.neon.tech/reference/getneonauthwebhookconfig) and [Update webhook configuration](https://api-docs.neon.tech/reference/updateneonauthwebhookconfig).

```bash
PUT /projects/{project_id}/branches/{branch_id}/auth/webhooks
GET /projects/{project_id}/branches/{branch_id}/auth/webhooks
```

Both endpoints use the following fields:

| Field             | Type               | Description                                                                                                                                                                                   |
| ----------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`         | boolean (required) | Enable or disable webhook delivery                                                                                                                                                            |
| `webhook_url`     | string             | HTTPS endpoint to receive webhook POST requests                                                                                                                                               |
| `enabled_events`  | string\[]          | Event types to subscribe to: `send.otp`, `send.magic_link`, `user.before_create`, `user.created`                                                                                              |
| `timeout_seconds` | integer (1-10)     | Per-attempt timeout in seconds. Default: 5. Total delivery time across all attempts is capped at 15 seconds. See [Retry behavior](https://neon.com/docs/auth/guides/webhooks#retry-behavior). |

### Set or update configuration

```bash
curl -X PUT "https://console.neon.tech/api/v2/projects/{project_id}/branches/{branch_id}/auth/webhooks" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $NEON_API_KEY" \
  -d '{
    "enabled": true,
    "webhook_url": "https://your-app.com/webhooks/neon-auth",
    "enabled_events": ["send.otp", "send.magic_link", "user.before_create", "user.created"],
    "timeout_seconds": 5
  }'
```

### Get current configuration

```bash
curl "https://console.neon.tech/api/v2/projects/{project_id}/branches/{branch_id}/auth/webhooks" \
  -H "Authorization: Bearer $NEON_API_KEY"
```

Both endpoints return the configuration in the same format:

```json
{
  "enabled": true,
  "webhook_url": "https://your-app.com/webhooks/neon-auth",
  "enabled_events": [
    "send.otp",
    "send.magic_link",
    "user.before_create",
    "user.created"
  ],
  "timeout_seconds": 5
}
```

## Payload structure

All events share a common JSON envelope:

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "send.otp",
  "timestamp": "2026-02-23T12:00:00.000Z",
  "context": {
    "endpoint_id": "ep-cool-sound-12345678",
    "project_name": "My SaaS App"
  },
  "user": {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "email": "user@example.com",
    "name": "Jane Smith",
    "email_verified": false,
    "created_at": "2026-02-23T12:00:00.000Z"
  },
  "event_data": {
    "otp_code": "123456",
    "otp_type": "sign-in",
    "expires_at": "2026-02-23T12:10:00.000Z",
    "ip_address": "192.0.2.1",
    "user_agent": "Mozilla/5.0"
  }
}
```

The `user` object fields are all optional and vary by event. Available fields: `id`, `email`, `name`, `phone_number`, `image`, `email_verified`, `phone_number_verified`, `created_at`.

### `send.otp` event data

| Field                 | Type              | Description                                                 |
| --------------------- | ----------------- | ----------------------------------------------------------- |
| `otp_code`            | string            | 6-digit OTP code                                            |
| `otp_type`            | string            | `"sign-in"`, `"email-verification"`, or `"forget-password"` |
| `delivery_preference` | string (optional) | `"email"` or `"sms"`                                        |
| `expires_at`          | ISO datetime      | Expiry time                                                 |
| `ip_address`          | string            | Requester's IP address                                      |
| `user_agent`          | string            | Requester's user agent                                      |

### `send.magic_link` event data

| Field        | Type         | Description                                   |
| ------------ | ------------ | --------------------------------------------- |
| `link_type`  | string       | `"email-verification"` or `"forget-password"` |
| `link_url`   | string       | Full verification URL with embedded token     |
| `token`      | string       | Raw token for building custom redirect URLs   |
| `expires_at` | ISO datetime | Expiry time                                   |
| `ip_address` | string       | Requester's IP address                        |
| `user_agent` | string       | Requester's user agent                        |

Magic links do not include a `delivery_preference` field. Your webhook handler determines the delivery channel.

### `user.before_create` and `user.created` event data

These events fire only when a new user record is created in the database. They do not fire on subsequent sign-ins, including returning OAuth users.

| Field           | Type   | Description                                           |
| --------------- | ------ | ----------------------------------------------------- |
| `auth_provider` | string | `"credential"`, `"google"`, `"github"`, or `"vercel"` |
| `ip_address`    | string | Requester's IP address                                |
| `user_agent`    | string | Requester's user agent                                |

## Signature verification

Neon Auth uses asymmetric EdDSA (Ed25519) signatures with detached JWS, so key rotation does not require reconfiguring your endpoint. Verify signatures before processing webhooks.

### Request headers

Each webhook request includes the following headers:

| Header                    | Description                                    |
| ------------------------- | ---------------------------------------------- |
| `X-Neon-Signature`        | Detached JWS signature (`header..signature`)   |
| `X-Neon-Signature-Kid`    | Key ID for looking up the public key from JWKS |
| `X-Neon-Timestamp`        | Unix timestamp in milliseconds                 |
| `X-Neon-Event-Type`       | Event type (for example, `user.created`)       |
| `X-Neon-Event-Id`         | Unique event UUID                              |
| `X-Neon-Delivery-Attempt` | Attempt number: 1, 2, or 3                     |

Example incoming webhook request:

```http
POST /webhooks/neon-auth HTTP/1.1
Content-Type: application/json
X-Neon-Signature: eyJhbGciOiJFZERTQSIsInR5cCI6IkpXUyIsImtpZCI6IjAxZGVjNTJiIn0..MEUCIQDZ8Qs
X-Neon-Signature-Kid: 01dec52b-4666-40f7-87ed-6423552eecaf
X-Neon-Timestamp: 1740312000000
X-Neon-Event-Type: send.otp
X-Neon-Event-Id: 550e8400-e29b-41d4-a716-446655440000
X-Neon-Delivery-Attempt: 1

{"event_id":"550e8400-e29b-41d4-a716-446655440000","event_type":"send.otp",...}
```

### Verification steps

1. Fetch your JWKS from `<NEON_AUTH_URL>/.well-known/jwks.json`. Find the key where `kid` matches the `X-Neon-Signature-Kid` header.
2. Parse the detached JWS from `X-Neon-Signature`. The format is `header..signature` (empty middle section).
3. Reconstruct the signing input using standard JWS with double base64url encoding:
   - `payloadB64 = base64url(rawRequestBody)`
   - `signaturePayload = timestamp + "." + payloadB64`
   - `signaturePayloadB64 = base64url(signaturePayload)`
   - `signingInput = header + "." + signaturePayloadB64`
4. Verify the Ed25519 signature against the signing input using the public key.

The double base64url encoding occurs because the timestamp is bound into the JWS payload per RFC 7515 Compact Serialization.

### Idempotency and additional checks

Retries send the same `X-Neon-Event-Id`. Your endpoint should track this value and return the same response for duplicate deliveries. This is especially important for `user.before_create`, where a lost response triggers a retry with the same event.

Consider rejecting requests where `X-Neon-Timestamp` is more than 5 minutes old to prevent replay attacks.

### Node.js example

```javascript
import crypto from 'node:crypto';

async function verifyWebhook(rawBody, headers) {
  const signature = headers['x-neon-signature'];
  const kid = headers['x-neon-signature-kid'];
  const timestamp = headers['x-neon-timestamp'];

  // 1. Fetch JWKS and find the matching key
  const res = await fetch(`${process.env.NEON_AUTH_URL}/.well-known/jwks.json`);
  const jwks = await res.json();
  const jwk = jwks.keys.find((k) => k.kid === kid);
  if (!jwk) throw new Error(`Key ${kid} not found in JWKS`);

  // 2. Import the Ed25519 public key
  const publicKey = crypto.createPublicKey({ key: jwk, format: 'jwk' });

  // 3. Parse detached JWS (header..signature)
  const [headerB64, emptyPayload, signatureB64] = signature.split('.');
  if (emptyPayload !== '') throw new Error('Expected detached JWS format');

  // 4. Reconstruct signing input (standard JWS, double base64url encoding)
  const payloadB64 = Buffer.from(rawBody, 'utf8').toString('base64url');
  const signaturePayload = `${timestamp}.${payloadB64}`;
  const signaturePayloadB64 = Buffer.from(signaturePayload, 'utf8').toString('base64url');
  const signingInput = `${headerB64}.${signaturePayloadB64}`;

  // 5. Verify Ed25519 signature
  const isValid = crypto.verify(
    null,
    Buffer.from(signingInput),
    publicKey,
    Buffer.from(signatureB64, 'base64url')
  );

  if (!isValid) throw new Error('Invalid webhook signature');

  // 6. Check timestamp freshness (recommended)
  const ageMs = Date.now() - parseInt(timestamp, 10);
  if (ageMs > 5 * 60 * 1000) throw new Error('Webhook timestamp too old');

  return JSON.parse(rawBody);
}
```

**Important:** Preserve the raw request body before JSON parsing. If your framework parses the body automatically, save the raw bytes first. Re-serialized JSON may differ from the original bytes and cause signature verification to fail.

**Next.js App Router example:**

```javascript
// app/webhooks/neon-auth/route.js
export async function POST(request) {
  const rawBody = await request.text();
  const payload = await verifyWebhook(
    rawBody,
    Object.fromEntries(request.headers)
  );
  // process payload
  return Response.json({ allowed: true });
}
```

**Tip:** In production, cache the JWKS response and refresh it when you encounter an unknown key ID. Rate-limit refresh attempts to avoid excessive requests to the JWKS endpoint.

## Expected responses

Webhook responses must not exceed 10KB.

### `send.otp` and `send.magic_link`

Return any 2xx status code. The response body is ignored.

If all 3 delivery attempts fail or the 15-second global timeout expires, the auth flow fails and the user sees an error.

### `user.before_create`

Return a 2xx status code with a JSON body.

**Allow signup:**

```json
{
  "allowed": true
}
```

**Reject signup:**

```json
{
  "allowed": false,
  "error_message": "Signups from this domain are not allowed.",
  "error_code": "DOMAIN_BLOCKED"
}
```

| Field           | Type               | Description                                        |
| --------------- | ------------------ | -------------------------------------------------- |
| `allowed`       | boolean (required) | Whether to permit user creation                    |
| `error_message` | string (optional)  | User-facing rejection message (max 500 characters) |
| `error_code`    | string (optional)  | Machine-readable code for client-side handling     |

If the webhook fails or returns an invalid response, signup is rejected. This fail-closed behavior prevents bypassing your validation logic.

**Important:** If your webhook endpoint is unreachable, all signups fail. Monitor your endpoint availability and keep response times well under the configured timeout to leave room for network latency and retries.

### `user.created`

Return any 2xx status code. The response body is ignored.

This event is non-blocking. Failures are logged but do not affect the user creation. Return 200 immediately and process the event asynchronously (for example, via a job queue). This prevents timeouts under load.

## Retry behavior

Because blocking events pause the user's auth flow, retries happen immediately rather than using exponential backoff. The user cannot wait minutes for a retry.

The 15-second global timeout runs from the start of the first attempt. Each attempt uses the lesser of `timeout_seconds` or the remaining global time. If earlier attempts consume the budget, later attempts get reduced timeouts or are skipped.

| Property       | Value                                                                                        |
| -------------- | -------------------------------------------------------------------------------------------- |
| Max attempts   | 3 (1 initial + 2 retries, no backoff)                                                        |
| Global timeout | 15 seconds across all attempts                                                               |
| Retryable      | 5xx, 429, 408, network errors (ECONNREFUSED, ETIMEDOUT, ECONNRESET, ENOTFOUND, ECONNABORTED) |
| Non-retryable  | 4xx (except 408 and 429)                                                                     |

## Testing and debugging

Neon Auth does not currently support test events, event logs, or redelivery. To test webhooks during development, expose a local server using a tunneling tool (for example, ngrok) and configure it as your webhook URL. Neon Auth rejects webhook URLs that point to localhost or private IP addresses.

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Production checklist](https://neon.com/docs/auth/production-checklist)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)

> This page location: Backend > Neon Auth > Guides > Production checklist
> Full Neon documentation index: https://neon.com/docs/llms.txt

# Auth production checklist

Required configuration before launching with Neon Auth

**Note: Beta** The **Neon Auth with Better Auth** is in Beta. Share your feedback on [Discord](https://discord.gg/92vNTzKDGp) or via the [Neon Console](https://console.neon.tech/app/projects?modal=feedback).

Complete these steps before taking your application to production with Neon Auth.

## Auth production checklist

- [ ] [1. Configure trusted domains](https://neon.com/docs/auth/guides/configure-domains)
    Add your production domain(s) to enable OAuth and email verification redirects. See [Configure trusted domains](https://neon.com/docs/auth/guides/configure-domains).
- [ ] [2. Set up custom email provider](https://neon.com/docs/auth/production-checklist#email-provider)
    Replace shared SMTP (`auth@mail.myneon.app`) with your own email service for reliable delivery and higher limits. A custom email provider is also required if you want to use verification links instead of verification codes. See [Email provider configuration](https://neon.com/docs/auth/production-checklist#email-provider) below.
- [ ] [3. Configure OAuth credentials (if using OAuth)](https://neon.com/docs/auth/guides/setup-oauth#production-setup)
    Set up your own Google and GitHub OAuth apps to replace shared development keys. See [OAuth production setup](https://neon.com/docs/auth/guides/setup-oauth#production-setup).
- [ ] [4. Enable email verification (recommended)](https://neon.com/docs/auth/guides/email-verification)
    **Email verification is not enabled by default.** Since anyone can sign up for your application, enabling email verification adds an important verification step to ensure users own their email address. See [Email verification guide](https://neon.com/docs/auth/guides/email-verification).
- [ ] [5. Disable localhost access](https://neon.com/docs/auth/production-checklist#localhost-access)
    Disable the "Allow Localhost" setting in your project's **Settings** → **Auth** page. This setting is enabled by default for development but should be disabled in production to improve security. See [Localhost access](https://neon.com/docs/auth/production-checklist#localhost-access) below.

## Email provider (#email-provider)

Neon Auth uses a shared SMTP provider (`auth@mail.myneon.app`) by default for development and testing. For production, configure your own email provider for better deliverability and higher sending limits.

### Configure custom SMTP

In your project's **Settings** → **Auth** page, configure your email provider:

1. Select **Custom SMTP provider**
2. Enter your SMTP credentials:
   - **Host**: Your SMTP server hostname (for example, `smtp.gmail.com`)
   - **Port**: SMTP port (typically `465` for SSL or `587` for TLS)
   - **Username**: Your SMTP username
   - **Password**: Your SMTP password or app-specific password
   - **Sender email**: Email address to send from
   - **Sender name**: Display name for sent emails
3. Click **Save**

### Email provider requirements

- **Verification links**: Require a custom email provider
- **Verification codes**: Work with shared or custom email providers
- **Password reset**: Works with shared or custom email providers

**Note:** The shared email provider (`auth@mail.myneon.app`) is suitable for development and testing. For production applications, use a custom email provider for better deliverability and to avoid rate limits.

## Localhost access (#localhost-access)

The "Allow Localhost" setting in your project's **Settings** → **Auth** page is enabled by default to allow authentication requests from localhost during development.

### Disable for production

For production environments, disable this setting to improve security:

1. Go to **Settings** → **Auth** in your Neon project
2. Find the **Allow Localhost** toggle
3. Disable the toggle

**Important:** Only enable "Allow Localhost" for local development. Disabling this setting in production prevents unauthorized authentication requests from localhost, improving your application's security posture.

---

## Related docs (Guides)

- [Email verification](https://neon.com/docs/auth/guides/email-verification)
- [Set up OAuth](https://neon.com/docs/auth/guides/setup-oauth)
- [Password reset](https://neon.com/docs/auth/guides/password-reset)
- [User management](https://neon.com/docs/auth/guides/user-management)
- [Configure domains](https://neon.com/docs/auth/guides/configure-domains)
- [Webhooks](https://neon.com/docs/auth/guides/webhooks)
- [Manage Auth via the API](https://neon.com/docs/auth/guides/manage-auth-api)