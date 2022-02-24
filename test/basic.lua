function wait(seconds)
    local start = os.time()
    repeat until os.time() > start + seconds
end

print(misc.used_memory())
window.create(500,500,'hi')
wait(100)