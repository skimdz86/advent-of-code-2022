package.path = package.path .. ";../?.lua"
local inspect = require('inspect')

local file = io.open("input.txt", "r")

lines = file:lines()
print("Contents of file:");

local total_points = 0

for line in lines do 
    print(line)

    local length = #line
    local compartment1 = string.sub(line, 1, length / 2)
    local compartment2 = string.sub(line, length / 2 + 1, length)

    local shared_item = ''

    print(compartment1.." - "..compartment2)

    for i = 1, #compartment1 do
        local c = compartment1:sub(i,i)
        --print(c)
        if compartment2:find(c) then
            shared_item = c
            break
        end
    end

    print("Shared item: "..shared_item)

    local shared_item_priority
    if shared_item == string.lower(shared_item) then
        shared_item_priority = string.byte(shared_item) - 96 -- calcolo l'offset dal codice ascii dei caratteri per far prima e non tenere una mappa
        print("lower case: "..shared_item.." - "..shared_item_priority)
    else 
        shared_item_priority = string.byte(shared_item) - 38
        print("upper case: "..shared_item.." - "..shared_item_priority)
    end

    total_points = total_points + shared_item_priority
    print("Total points: "..total_points)

end