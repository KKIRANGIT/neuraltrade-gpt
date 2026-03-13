// AuthService
// register(email, password, name, phone) -> JWT token
// login(email, password) -> JWT token + refresh token
// refreshToken(refreshToken) -> new JWT token
// logout(userId) -> invalidate token
// JWT validation middleware for all protected endpoints
// Password: bcrypt hashed, never stored plain
// Sessions: JWT with 24h expiry, refresh token 30 days
