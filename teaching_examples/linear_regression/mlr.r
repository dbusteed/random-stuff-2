#----------
#
#   This R script looks into how linear regression (OLS) works!
#   It computes OLS "manually", then compares these regression
#   coefficients to the built in R function for computing OLS,
#   (spoiler: they are the same). 
#
#----------

#
#   create a subset of the `mtcars` dataset.
#   keeping things simple by using `mpg` as the
#   response variable, and the others as the 
#   predictors
#
cars <- mtcars[,c("mpg", "disp", "hp", "wt", "cyl")]

#
#   calculate regression coefficients
#   using the formula derived when we
#   minimized the sum of square residuals (OLS)
#
y <- cars[1]
X <- cars[2:5]
y <- t(matrix(unlist(y), ncol=nrow(y), byrow=TRUE))
X <- t(matrix(unlist(X), ncol=nrow(X), byrow=TRUE))
X <- cbind(1, X)

# OLS defines the coefficients (beta) to be:
#   beta = ((X.T * X)).I * X.T * Y
beta <- solve((t(X) %*% X)) %*% t(X) %*% y


#
#   calulate regression coefficients
#   using the built-in lm() function
#
model <- lm(mpg~disp+hp+wt+cyl, data=cars)
lm.coefs <- coef(model)

#
#   print results
#
cat("coefs from manually calculated OLS:", "\n")
cat("  ", t(beta), "\n")
cat("\n")
cat("coefs from built-in lm() function:", "\n")
cat("  ", lm.coefs, "\n")
