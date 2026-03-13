// ClaudeApiClient — wraps Anthropic API
// Uses claude-haiku-4-5-20251001 for signal validation (cheapest+fastest)
// Uses claude-sonnet-4-6 for deep on-demand analysis (user clicks)
// Methods:
//   validateSignal(SignalValidationRequest) -> ValidationResult
//   generateDeepAnalysis(StockAnalysisRequest) -> String
//   translateToTelugu(String englishText) -> String
// Handles: rate limits, retries (3x with exponential backoff)
// Prompt caching: system prompts cached to save 60% input tokens
// Async: all calls non-blocking (Spring WebFlux)
