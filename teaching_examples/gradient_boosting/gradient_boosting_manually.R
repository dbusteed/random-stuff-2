#
#   A simple, manual implemtation of the
#   gradient boosting algorithm
#
#   https://www.youtube.com/watch?v=d6CzVXoVZCc
#

library(tree)     # decision tree model
library(caret)    # RMSE & R2 functions
library(ggplot2)  # plotting RMSE over time
library(randomForest)  # for comparing a RF model

# model parameters
LR <- 0.15
nrounds <- 30

# grab the data, prep x_vars
df <- mtcars
x_vars <- names(df[2:ncol(df)])
x_vars <- paste(x_vars, collapse=" + ")

# add in blank column to contain predictions
prediction <- NaN
df <- cbind(df[1], prediction, df[2:ncol(df)])

# ROUND 1
#  use the average of the y_var
#  for our first prediction
df$pred_1 <- mean(df$mpg)
df$prediction <- df$pred_1
df$resd_1 <- (df$mpg - df$prediction)

# calculate the RMSE for this round
# of predictions
rmse <- RMSE(df$mpg, df$prediction)
r2   <- R2(df$mpg, df$prediction) 
results <- data.frame("Round"=c(1), "R2"=c(r2), "RMSE"=c(rmse))

# ROUND 2 through ROUND N
for (i in 2:nrounds) {
  
  # for each round, use the x_vars to predict
  # the residuals of the previous round
  mdl <- eval(parse(text=paste0("tree(resd_", i-1, "~", x_vars, ", data=df)")))
  df[[paste0("pred_", i)]] <- predict(mdl, df)
  
  # update the `prediction` column by adding the previous
  # prediction the learning rate * the predicted residuals
  df$prediction <- df$prediction + (LR * df[[paste0("pred_", i)]])
  df[[paste0("resd_", i)]] <- (df$mpg - df$prediction)
  
  # calculate the RMSE for each round
  rmse <- RMSE(df$mpg, df$prediction)
  r2   <- R2(df$mpg, df$prediction) 
  results <- rbind(results, list("Round"=c(i), "R2"=c(r2), "RMSE"=c(rmse)))
}


# create a basic tree model for comparison
mdl <- eval(parse(text=paste0("tree(mpg~", x_vars, ", data=df)")))
prediction <- predict(mdl, df)
tree_rmse <- RMSE(df$mpg, prediction)
tree_r2   <- R2(df$mpg, prediction) 

# create a random forest model for comparison
mdl <- eval(parse(text=paste0("randomForest(mpg~", x_vars, ", data=df)")))
prediction <- predict(mdl, df)
rf_rmse <- RMSE(df$mpg, prediction)
rf_r2   <- R2(df$mpg, prediction) 

# plot the RMSE and R2 for the gradient boosting approach 
# as it was trained, as well as a one-time-score 
# for a Decision Tree and Random Forest model
ggplot() + 
  geom_line(data=results, aes(x=Round, y=RMSE), color="black") +
  geom_hline(yintercept=tree_rmse, color="red", linetype="dashed") +
  geom_hline(yintercept=rf_rmse, color="blue", linetype="dashed")

ggplot() + 
  geom_line(data=results, aes(x=Round, y=R2), color="black") +
  geom_hline(yintercept=tree_r2, color="red", linetype="dashed") +
  geom_hline(yintercept=rf_r2, color="blue", linetype="dashed")