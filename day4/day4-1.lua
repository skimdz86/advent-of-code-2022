package.path = package.path .. ";../?.lua"
local inspect = require('inspect')

local file = io.open("input.txt", "r")

lines = file:lines()
print("Contents of file:");

local total_overlap_counter = 0
local overlap_counter = 0
local index = 0
for line in lines do 
    --print(line)

    local separator_index = line:find(',') 

    local first = line:sub(1, separator_index - 1)
    local second = line:sub(separator_index + 1, #line)

    --print("First elf: "..first)
    --print("Second elf: "..second)

    local dash_1_index = first:find('-')
    local dash_2_index = second:find('-')

    local first_left = tonumber(first:sub(1, dash_1_index - 1)) -- to number indispensabile: non è come i linguaggi moderni, se è una stringa e faccio >= tira fuori risultati a caso
    local first_right = tonumber(first:sub(dash_1_index + 1, #first))

    local second_left = tonumber(second:sub(1, dash_2_index - 1))
    local second_right = tonumber(second:sub(dash_2_index + 1, #second))

    --print("type "..type(first_left))

    if second_left >= first_left and second_right <= first_right or first_left >= second_left and first_right <= second_right then
        total_overlap_counter = total_overlap_counter + 1
        --print("s_l="..second_left..", s_r="..second_right..", f_l="..first_left..", f_r="..first_right)
        print("Total Overlapping! "..line)
    end
    
    if second_left <= first_right and second_left >= first_left or first_left <= second_right and first_left >= second_left then
        overlap_counter = overlap_counter + 1
        --print("s_l="..second_left..", s_r="..second_right..", f_l="..first_left..", f_r="..first_right)
        print("Overlapping! "..line)
    end

    --[=====[
    index = index + 1
    if index > 10 then
        break
    end
    --]=====]
end

print("Complete Overlapping: #"..total_overlap_counter)
print("Overlapping: #"..overlap_counter)