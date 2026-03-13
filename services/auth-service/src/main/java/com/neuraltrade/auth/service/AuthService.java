package com.neuraltrade.auth.service;

import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

public class AuthService {
    private final Map<String, String> users = new HashMap<>();
    private final Map<String, String> refreshTokens = new HashMap<>();

    public String register(String email, String password, String name, String phone) {
        users.put(email, hash(password + ":" + name + ":" + phone));
        return issueJwt(email);
    }

    public Map<String, String> login(String email, String password) {
        if (!users.containsKey(email)) {
            throw new IllegalArgumentException("Unknown user");
        }
        String jwt = issueJwt(email);
        String refreshToken = UUID.randomUUID().toString();
        refreshTokens.put(refreshToken, email);
        return Map.of("jwt", jwt, "refreshToken", refreshToken);
    }

    public String refreshToken(String refreshToken) {
        String email = refreshTokens.get(refreshToken);
        if (email == null) {
            throw new IllegalArgumentException("Invalid refresh token");
        }
        return issueJwt(email);
    }

    public void logout(String email) {
        refreshTokens.entrySet().removeIf(entry -> entry.getValue().equals(email));
    }

    private String issueJwt(String email) {
        return "jwt-" + UUID.nameUUIDFromBytes(email.getBytes());
    }

    private String hash(String value) {
        return Integer.toHexString(value.hashCode());
    }
}
