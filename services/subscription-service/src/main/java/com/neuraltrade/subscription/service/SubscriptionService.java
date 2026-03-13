package com.neuraltrade.subscription.service;

import java.time.Instant;
import java.util.EnumMap;
import java.util.Map;

public class SubscriptionService {
    public enum Plan {
        FREE, BASIC, PRO, ELITE
    }

    private final Map<String, Plan> subscriptions = new java.util.HashMap<>();
    private final Map<Plan, java.util.List<String>> entitlements = new EnumMap<>(Plan.class);

    public SubscriptionService() {
        entitlements.put(Plan.BASIC, java.util.List.of("RULE_BASED_STORY", "MORNING_BRIEFING"));
        entitlements.put(Plan.PRO, java.util.List.of("RULE_BASED_STORY", "AI_STORY", "SIGNALS", "TELUGU"));
        entitlements.put(Plan.ELITE, java.util.List.of("RULE_BASED_STORY", "AI_STORY", "SIGNALS", "TELUGU", "DEEP_ANALYSIS", "PRIORITY"));
    }

    public String createSubscription(String userId, Plan plan) {
        subscriptions.put(userId, plan);
        return "rzp_" + userId + "_" + plan + "_" + Instant.now().toEpochMilli();
    }

    public boolean checkAccess(String userId, String feature) {
        Plan plan = subscriptions.getOrDefault(userId, Plan.FREE);
        return entitlements.getOrDefault(plan, java.util.List.of()).contains(feature);
    }

    public void handleWebhook(String userId, String status) {
        if ("subscription.cancelled".equals(status)) {
            subscriptions.put(userId, Plan.FREE);
        }
    }

    public void cancelSubscription(String userId) {
        subscriptions.put(userId, Plan.FREE);
    }
}
