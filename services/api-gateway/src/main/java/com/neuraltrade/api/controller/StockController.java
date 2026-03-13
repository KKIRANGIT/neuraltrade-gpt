package com.neuraltrade.api.controller;

import java.util.List;
import java.util.Map;

public class StockController {
    private final List<Map<String, Object>> stocks = List.of(
            Map.of("symbol", "RELIANCE", "bullScore", 86, "bearScore", 18, "signal", "SCR10"),
            Map.of("symbol", "TCS", "bullScore", 81, "bearScore", 22, "signal", "SCR05"),
            Map.of("symbol", "HDFCBANK", "bullScore", 78, "bearScore", 28, "signal", "SCR14")
    );

    public List<Map<String, Object>> listStocks() {
        return stocks;
    }

    public Map<String, Object> getStock(String symbol) {
        return stocks.stream()
                .filter(stock -> symbol.equals(stock.get("symbol")))
                .findFirst()
                .orElse(Map.of("symbol", symbol, "status", "NOT_FOUND"));
    }

    public List<Map<String, Object>> topBull() {
        return stocks;
    }
}
