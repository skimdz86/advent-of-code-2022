package.path = package.path .. ";../?.lua"
local inspect = require('inspect')

local map_move = {["A"] = "rock", ["B"] = "paper", ["C"] = "scissors", ["X"] = "rock", ["Y"] = "paper", ["Z"] = "scissors"}
local move_value = {["rock"] = 1, ["paper"] = 2, ["scissors"] = 3}

local Outcome = {
    ["WIN"] = 6,
    ["DRAW"] = 3,
    ["LOSE"] = 0
}

local map_outcome = {["X"] = Outcome.LOSE, ["Y"] = Outcome.DRAW, ["Z"] = Outcome.WIN}

local file = io.open("input.txt", "r")

lines = file:lines()
print("Contents of file:");

local total_points = 0

for line in lines do 
    print(line)

    local move = string.sub(line, 1, 1)
    local line_outcome = string.sub(line, 3, 3)

    local row_points = 0
    local next_move = ""
    
    if map_outcome[line_outcome] == Outcome.WIN then
        if map_move[move] == "rock" then
            next_move = "paper"
        elseif map_move[move] == "paper" then
            next_move = "scissors"
        else -- scissors
            next_move = "rock"
        end
    elseif map_outcome[line_outcome] == Outcome.DRAW then
        next_move = map_move[move]
    else
        if map_move[move] == "rock" then
            next_move = "scissors"
        elseif map_move[move] == "paper" then
            next_move = "rock"
        else -- scissors
            next_move = "paper"
        end
    end

    print("DEBUG next_move: "..next_move)

    --print("DEBUG line_outcome: "..line_outcome)
    --print("DEBUG map line: "..map_outcome[line_outcome])

    row_points = move_value[next_move] + map_outcome[line_outcome]

    total_points = total_points + row_points
end

print("Total points: "..total_points)