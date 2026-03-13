package com.neuraltrade.alert.telegram;

public class TelegramAlertSender {
    public String send(String chatId, String message) {
        return "telegram:sent:" + chatId + ":" + message.length();
    }
}
