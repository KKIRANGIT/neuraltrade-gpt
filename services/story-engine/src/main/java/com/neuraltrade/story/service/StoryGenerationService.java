package com.neuraltrade.story.service;

import java.time.Instant;
import java.util.List;

import com.neuraltrade.story.chapters.ChapterBuilder;
import com.neuraltrade.story.model.StockStory;
import com.neuraltrade.story.templates.BearishTemplates;
import com.neuraltrade.story.templates.BullishTemplates;
import com.neuraltrade.story.translator.TeluguTranslationService;

public class StoryGenerationService {
    private final ChapterBuilder chapterBuilder = new ChapterBuilder();
    private final TeluguTranslationService translationService = new TeluguTranslationService();

    public StockStory generateRuleBased(String symbol, int bullScore, int bearScore, String language) {
        boolean bullish = bullScore >= bearScore;
        List<String> templates = bullish ? BullishTemplates.defaultEnglish() : BearishTemplates.defaultEnglish();
        if ("te".equalsIgnoreCase(language) && bullish) {
            templates = translationService.translateBullishTemplatePack();
        }
        return new StockStory(
                symbol,
                language,
                "RULE_BASED",
                bullScore,
                bearScore,
                Instant.now(),
                chapterBuilder.build(symbol, templates)
        );
    }

    public StockStory generateOnDemandDeepStory(String symbol, int bullScore, int bearScore, String language) {
        return generateRuleBased(symbol, bullScore, bearScore, language);
    }
}
