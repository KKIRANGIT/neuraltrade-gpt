package com.neuraltrade.ai.claude;

import com.neuraltrade.ai.model.ValidationResult;

public class ClaudeApiClient {
    public ValidationResult validate(String prompt, int confluenceScore, double xgbProbability) {
        if (xgbProbability < 55.0) {
            return new ValidationResult("REJECT", "MEDIUM", "ML probability below threshold.", "LOW_XGB", "Probability takkuva undi.");
        }
        if (confluenceScore >= 80) {
            return new ValidationResult("APPROVE", "HIGH", "Multi-factor alignment is strong.", "NONE", "Signal strong ga undi.");
        }
        return new ValidationResult("WATCHLIST", "MEDIUM", "Structure is promising but not elite.", "WAIT_FOR_CONFIRMATION", "Inka confirmation kavali.");
    }
}
