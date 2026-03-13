// TeluguTranslationService
// Two modes:
//   1. TEMPLATE mode: use pre-written TeluguTemplates (free, instant)
//   2. CLAUDE mode: send English story to Claude API for rich translation
// translateWithClaude(englishStory) -> String (Telugu)
//   System prompt: Telugu-speaking analyst, Hyderabad style, keep numbers/indicators in English
//   Uses Claude Haiku for cost efficiency
//   Cache result in Redis for 1 hour
// selectMode(): TEMPLATE for routine, CLAUDE for on-demand user requests
