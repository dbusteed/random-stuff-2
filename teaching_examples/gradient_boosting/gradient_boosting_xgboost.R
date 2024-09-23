#
#   gradient boosting using the `xgboost` library
#

library(xgboost)
library(tidyverse)
library(caret)

sample <- sample.int(n=nrow(mtcars), size=.75 * (nrow(mtcars)))
train <- mtcars[sample, ]
test  <- mtcars[-sample, ]

train_x <- train %>% select(-mpg) %>% as.matrix
train_y <- train %>% select(mpg) %>% as.matrix %>% as.numeric
test_x <- test %>% select(-mpg) %>% as.matrix
test_y <- test %>% select(mpg) %>% as.matrix %>% as.numeric

cv <- trainControl(method = "cv",
                   number = 5)

xgb_grid <- expand.grid(
  nrounds = c(100),
  eta = c(0.1),
  max_depth = c(10),
  gamma = 0,
  colsample_bytree = 1,
  min_child_weight = 1,
  subsample = 1
)

options(warn=-1)
xgb_fit <- train(x = train_x,
                 y = train_y,
                 trControl = cv,
                 tuneGrid = xgb_grid,
                 method = "xgbTree")
                 
# objective = "reg:squarederror")

y_hat <- predict(xgb_fit, test_x)
r2 <- R2(y_hat, test_y)

print(r2)