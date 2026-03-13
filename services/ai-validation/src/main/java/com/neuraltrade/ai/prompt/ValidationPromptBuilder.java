package com.neuraltrade.ai.prompt;

import java.util.Map;

public class ValidationPromptBuilder {
    public String build(Map<String, Object> signalPayload) {
        return """
                Validate this trading signal.
                Symbol: %s
                Direction: %s
                Entry: %s
                Stop: %s
                Confluence: %s
                """.formatted(
                signalPayload.getOrDefault("symbol", "UNKNOWN"),
                signalPayload.getOrDefault("direction", "BULL"),
                signalPayload.getOrDefault("entry", 0),
                signalPayload.getOrDefault("stop", 0),
                signalPayload.getOrDefault("confluenceScore", 0)
        );
    }
}
