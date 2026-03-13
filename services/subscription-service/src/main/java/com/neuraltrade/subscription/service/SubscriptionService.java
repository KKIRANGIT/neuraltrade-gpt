// SubscriptionService
// Plans:
//   BASIC ₹499/month: rule-based stories, 5 signals/day, no Telugu
//   PRO ₹999/month: AI stories, all signals, Telugu, Telegram alerts
//   ELITE ₹1999/month: everything + weekly deep reports + priority support
// createSubscription(userId, plan) -> integrates with Razorpay
// checkAccess(userId, feature) -> boolean (used by API gateway)
// handleWebhook(razorpayPayload) -> activate/deactivate subscription
// cancelSubscription(userId) -> marks cancelled, active till period end
