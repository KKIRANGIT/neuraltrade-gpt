// NEURALTRADE — Story Generation Service
// Consumes signals + scoreboards from Kafka
// Generates 3 types of stories:
//   1. RULE_BASED: pre-written templates filled with indicator values (free, instant)
//   2. AI_ENHANCED: rule-based + Claude API enrichment (top 50 stocks only)
//   3. ON_DEMAND: full deep story via Claude API when user clicks stock
// Stores generated stories in Redis with 15-minute TTL
// Publishes story events to Kafka: stories.generated
// Supports English and Telugu output
