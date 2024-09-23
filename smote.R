library(RANN)
library(plotly)

df <- read.csv('../data/mammography.csv')

pca <- prcomp(df[1:6])
df_pca <- as.data.frame(predict(pca, df[1:6]))
df_pca <- df_pca[1:3]
df <- cbind(df_pca, df[7])

fig <- plot_ly(df, x=~PC1, y=~PC2, z=~PC3)
fig <- fig %>% add_markers()

fig

df$color <- ifelse(df$y == 0, 'blue', 'red')

plot(df$PC1, df$PC2, pch=15, col=df$color)

df <- data.frame(x1=rnorm(20, 2, 2), x2=rnorm(20, 2, 2))
df$source <- 'observed'

t <- 20
n <- 200
k <- 3
p <- 2

n <- as.integer(n / 100)

nn <- RANN::nn2(df, df, k=k+1)

for (i in 1:t) {
  for (n_ in 1:n) {
    nn_idx <- nn$nn.idx[i, ][sample.int(k, 1) + 1]
    nn_diff <- df[nn_idx, (1:p)] - df[i, (1:p)]
    new_point <- df[i, (1:p)] + runif(p) * nn_diff
    new_point$source <- 'synthetic'
    df <- rbind(df, new_point)
  }
}

# reset row indices
row.names(df) <- NULL

df$color <- ifelse(df$source == 'observed', 'blue', 'red')
plot(df$x1, df$x2, pch=15, col=df$color, )
