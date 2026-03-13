# ModelTrainer — runs every Sunday 9 PM on MacBook M2
# train_xgboost(): load 3yr data → train → evaluate → save if improved
# train_lightgbm(): same pipeline
# train_lstm(): PyTorch training loop with MPS device
#               EarlyStopping patience=10
#               Save best model checkpoint
# train_regime(): load Nifty+macro data → train RF → save
# train_anomaly(): unsupervised, retrain on latest 6 months
# generate_training_report(): accuracy, feature importance, confusion matrix
