package com.neuraltrade.story.model;

import java.time.Instant;
import java.util.List;

public record StockStory(
        String symbol,
        String language,
        String storyType,
        int bullScore,
        int bearScore,
        Instant generatedAt,
        List<Chapter> chapters
) {
    public record Chapter(int number, String title, String content) {}
}
