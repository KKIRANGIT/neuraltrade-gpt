package com.neuraltrade.alert.service;

import com.neuraltrade.alert.telegram.TelegramAlertSender;

public class AlertOrchestrator {
    private final TelegramAlertSender telegramAlertSender = new TelegramAlertSender();

    public String sendSignalAlert(String chatId, String symbol, int score, String snippet) {
        String message = """
                STRONG BULL - %s
                Score: %d/100
                %s
                """.formatted(symbol, score, snippet);
        return telegramAlertSender.send(chatId, message);
    }
}
