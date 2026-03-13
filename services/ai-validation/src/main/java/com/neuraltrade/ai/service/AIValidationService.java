package com.neuraltrade.ai.service;

import java.util.Map;

import com.neuraltrade.ai.claude.ClaudeApiClient;
import com.neuraltrade.ai.model.ValidationResult;
import com.neuraltrade.ai.prompt.ValidationPromptBuilder;

public class AIValidationService {
    private final ValidationPromptBuilder promptBuilder = new ValidationPromptBuilder();
    private final ClaudeApiClient claudeApiClient = new ClaudeApiClient();

    public ValidationResult validate(Map<String, Object> signalPayload) {
        boolean eventRisk = (boolean) signalPayload.getOrDefault("eventsRisk", false);
        if (eventRisk) {
            return new ValidationResult("REJECT", "HIGH", "Event risk is active for this symbol.", "EVENT_RISK", "Event risk undi.");
        }

        int confluenceScore = ((Number) signalPayload.getOrDefault("confluenceScore", 0)).intValue();
        double xgbProbability = ((Number) signalPayload.getOrDefault("xgbBullProbability", 50)).doubleValue();
        String prompt = promptBuilder.build(signalPayload);
        return claudeApiClient.validate(prompt, confluenceScore, xgbProbability);
    }
}
