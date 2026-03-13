package com.neuraltrade.story.chapters;

import java.util.ArrayList;
import java.util.List;

import com.neuraltrade.story.model.StockStory.Chapter;

public class ChapterBuilder {
    public List<Chapter> build(String symbol, List<String> templates) {
        List<Chapter> chapters = new ArrayList<>();
        for (int index = 0; index < templates.size(); index++) {
            chapters.add(new Chapter(index + 1, "Chapter " + (index + 1), symbol + ": " + templates.get(index)));
        }
        return chapters;
    }
}
