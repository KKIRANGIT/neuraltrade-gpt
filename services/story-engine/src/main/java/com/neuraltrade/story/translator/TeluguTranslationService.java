package com.neuraltrade.story.translator;

import java.util.List;

import com.neuraltrade.story.templates.TeluguTemplates;

public class TeluguTranslationService {
    public List<String> translateBullishTemplatePack() {
        return TeluguTemplates.bullish();
    }

    public String translateSentence(String english) {
        return "Telugu reference: " + english;
    }
}
