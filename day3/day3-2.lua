package.path = package.path .. ";../?.lua"
local inspect = require('inspect')

local file = io.open("input2.txt", "r")

lines = file:lines()
print("Contents of file:");

local total_points = 0

local index = 1
local group_lines = {}
local group_points = 0

for line in lines do 
    print(line)

    table.insert(group_lines,line)

    if index % 3 == 0 then
        local badge_item = ''

        local group_line_1 = group_lines[1]
        --print(group_line_1)

        for i = 1, #group_line_1 do
            local c = group_line_1:sub(i,i)
            --print(c)
            if group_lines[2]:find(c) and group_lines[3]:find(c) then
                badge_item = c
                break
            end
        end
        
        print("Badge item: "..badge_item)

        -- calculate priorities for the group elves --> INUTILE, qua non avevo capito il problema, non serviva
        --[=====[ 
        for _, group_line in ipairs(group_lines) do
            
            local length = #group_line
            local compartment1 = string.sub(group_line, 1, length / 2)
            local compartment2 = string.sub(group_line, length / 2 + 1, length)

            local shared_item = ''

            print(compartment1.." - "..compartment2)

            for i = 1, #compartment1 do
                local c = compartment1:sub(i,i)
                --print(c)
                if c ~= badge_item then
                    if compartment2:find(c) then
                        shared_item = c
                        break
                    end
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
            
        end
        --]=====]

        local shared_item_priority
        if badge_item == string.lower(badge_item) then
            shared_item_priority = string.byte(badge_item) - 96 -- calcolo l'offset dal codice ascii dei caratteri per far prima e non tenere una mappa
            print("lower case: "..badge_item.." - "..shared_item_priority)
        else 
            shared_item_priority = string.byte(badge_item) - 38
            print("upper case: "..badge_item.." - "..shared_item_priority)
        end

        total_points = total_points + shared_item_priority

        -- reset counters e aggiunge al totale, finito il gruppo dei 3
        for k in pairs (group_lines) do
            group_lines[k] = nil
        end



    end

    index = index + 1
end

print("Total points: "..total_points)