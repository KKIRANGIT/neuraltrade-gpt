package com.neuraltrade.signal.tracker;

import java.util.HashMap;
import java.util.Map;

public class AccuracyTracker {
    private final Map<String, Integer> wins = new HashMap<>();
    private final Map<String, Integer> totals = new HashMap<>();

    public void record(String screenerId, boolean win) {
        totals.merge(screenerId, 1, Integer::sum);
        if (win) {
            wins.merge(screenerId, 1, Integer::sum);
        }
    }

    public double accuracy(String screenerId) {
        return wins.getOrDefault(screenerId, 0) / (double) Math.max(totals.getOrDefault(screenerId, 0), 1);
    }
}
