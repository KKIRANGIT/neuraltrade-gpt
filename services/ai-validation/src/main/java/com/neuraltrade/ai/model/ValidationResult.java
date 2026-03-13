package com.neuraltrade.ai.model;

public record ValidationResult(
        String verdict,
        String confidence,
        String reason,
        String redFlags,
        String telugu
) {}
