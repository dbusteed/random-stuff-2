library(ggplot2)

raw <- serialize(mtcars, NULL)
res <- data.frame()

orig_size <- as.numeric(object.size(raw) / 1000000)

algos <- c("gzip", "bzip2", "xz")
for (algo in algos) {
    t <- Sys.time()
    comp <- memCompress(raw, algo)
    size <- as.numeric(object.size(comp) / 1000000)
    perc <- size / orig_size
    elap <- as.numeric((Sys.time() - t), units="secs")
    res <- rbind(res, c(algo, size, perc, elap))
} 

names(res) <- c("algo", "size", "perc", "time")
res$size <- as.numeric(res$size)
res$perc <- as.numeric(res$perc)
res$time <- as.numeric(res$time)

g <- ggplot(res, aes(x=time, y=perc)) +
    geom_point(aes(color=algo), size=5)

print(g)

system.time({ raw_dump <- serialize(mdl, NULL) })
system.time({ compress <- memCompress(raw_dump, "xz") })
system.time({ encoded_model <- base64encode(compress) })
system.time({ bounds <- seq(1, nchar(encoded_model), row_len) })
system.time({
    chunks <- lapply(bounds,
                    chunk_string_v2,
                    string = encoded_model,
                    row_len = row_len)
})
system.time({ df <- data.frame(vals=unlist(chunks)) })