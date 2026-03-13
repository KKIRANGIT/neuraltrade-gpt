package com.neuraltrade.signal.model;

import java.time.Instant;

public class Signal {
    private final String id;
    private final String symbol;
    private final double entry;
    private final double stopLoss;
    private final double target1;
    private final double target2;
    private final double target3;
    private String status;
    private final Instant validUntil;

    public Signal(String id, String symbol, double entry, double stopLoss, double target1, double target2, double target3, Instant validUntil) {
        this.id = id;
        this.symbol = symbol;
        this.entry = entry;
        this.stopLoss = stopLoss;
        this.target1 = target1;
        this.target2 = target2;
        this.target3 = target3;
        this.validUntil = validUntil;
        this.status = "ACTIVE";
    }

    public String getId() { return id; }
    public String getSymbol() { return symbol; }
    public double getEntry() { return entry; }
    public double getStopLoss() { return stopLoss; }
    public double getTarget1() { return target1; }
    public double getTarget2() { return target2; }
    public double getTarget3() { return target3; }
    public String getStatus() { return status; }
    public Instant getValidUntil() { return validUntil; }
    public void setStatus(String status) { this.status = status; }
}
