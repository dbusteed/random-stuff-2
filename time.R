# https://anomaly.io/seasonal-trend-decomposition-in-r/index.html

install.packages("fpp")

library(fpp)
library(forecast)

data(ausbeer)
ts_beer <- tail(head(ausbeer, 17*4+2), 17*4-4)
plot(as.ts(ts_beer))

t <- ma(ts_beer, order=4, centre=T)
plot(as.ts(t))
