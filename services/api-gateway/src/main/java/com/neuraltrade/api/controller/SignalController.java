package com.neuraltrade.api.controller;

import java.util.List;
import java.util.Map;

public class SignalController {
    public List<Map<String, Object>> todaysSignals() {
        return List.of(
                Map.of("id", "SIG-RELIANCE", "symbol", "RELIANCE", "status", "ACTIVE", "score", 88),
                Map.of("id", "SIG-TCS", "symbol", "TCS", "status", "WATCHLIST", "score", 79)
        );
    }

    public Map<String, Object> signalDetail(String id) {
        return todaysSignals().stream()
                .filter(signal -> id.equals(signal.get("id")))
                .findFirst()
                .orElse(Map.of("id", id, "status", "NOT_FOUND"));
    }

    public Map<String, Object> marketContext() {
        return Map.of(
                "regime", "BULL_TRENDING",
                "nifty", 24186.4,
                "vix", 13.7,
                "fiiFlowCr", 742
        );
    }
}
