-- Load test script for wrk
-- Usage: wrk -t4 -c100 -d30s -s loadtest.lua http://localhost:8080

-- Get hashids file from environment or use default
local hashids_file = os.getenv("HASHIDS_FILE") or "tests/loadtest/sample_hashids_100.txt"

-- Load URLs from file
local urls = {}
local file = io.open(hashids_file, "r")
if file then
    for line in file:lines() do
        if line ~= "" then
            table.insert(urls, "/" .. line)
        end
    end
    file:close()
    print("Loaded " .. #urls .. " URLs from " .. hashids_file)
else
    print("Warning: Could not open " .. hashids_file)
    -- Fallback to a single test URL
    table.insert(urls, "/test")
end

local counter = 1

request = function()
    local path = urls[counter]
    counter = counter + 1
    if counter > #urls then
        counter = 1
    end
    return wrk.format("GET", path)
end
