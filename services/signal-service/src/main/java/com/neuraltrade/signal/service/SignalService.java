package com.neuraltrade.signal.service;

import java.time.Instant;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;

import com.neuraltrade.signal.model.Signal;
import com.neuraltrade.signal.tracker.AccuracyTracker;

public class SignalService {
    private final Map<String, Signal> signals = new HashMap<>();
    private final AccuracyTracker accuracyTracker = new AccuracyTracker();

    public Signal create(String id, String symbol, double entry, double stopLoss, double target1, double target2, double target3) {
        Signal signal = new Signal(id, symbol, entry, stopLoss, target1, target2, target3, Instant.now().plusSeconds(60 * 60 * 6));
        signals.put(id, signal);
        return signal;
    }

    public Signal updatePrice(String id, double high, double low) {
        Signal signal = signals.get(id);
        if (signal == null) {
            return null;
        }
        if (low <= signal.getStopLoss()) {
            signal.setStatus("STOPPED");
            accuracyTracker.record(id, false);
        } else if (high >= signal.getTarget3()) {
            signal.setStatus("T3_HIT");
            accuracyTracker.record(id, true);
        } else if (high >= signal.getTarget2()) {
            signal.setStatus("T2_HIT");
        } else if (high >= signal.getTarget1()) {
            signal.setStatus("T1_HIT");
        } else if (Instant.now().isAfter(signal.getValidUntil())) {
            signal.setStatus("EXPIRED");
        }
        return signal;
    }

    public Collection<Signal> activeSignals() {
        return signals.values();
    }
}
