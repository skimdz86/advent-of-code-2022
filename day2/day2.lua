package.path = package.path .. ";../?.lua"
local inspect = require('inspect')

local map_move = {["A"] = "rock", ["B"] = "paper", ["C"] = "scissors", ["X"] = "rock", ["Y"] = "paper", ["Z"] = "scissors"}
local move_value = {["X"] = 1, ["Y"] = 2, ["Z"] = 3}

local Outcome = {
    ["WIN"] = 6,
    ["DRAW"] = 3,
    ["LOSE"] = 0
 }

local file = io.open("input.txt", "r")

lines = file:lines()
print("Contents of file:");

local total_points = 0

for line in lines do 
    print(line)

    local move = string.sub(line, 1, 1)
    local response = string.sub(line, 3, 3)

    local row_points = 0
    local outcome

    --print("DEBUG map "..inspect(map_move))
    --print("DEBUG move #"..move.."#")
    --print("DEBUG response "..response)
    --print("DEBUG map move "..map_move[move])
    --print("DEBUG map response "..map_move[response])

    if map_move[move] == map_move[response] then
        outcome = Outcome.DRAW
    elseif map_move[move] == "rock" then
        if map_move[response] == "paper" then
            outcome = Outcome.WIN
        elseif map_move[response] == "scissors" then
            outcome = Outcome.LOSE
        end
    elseif map_move[move] == "paper" then
        if map_move[response] == "rock" then
            outcome = Outcome.LOSE
        elseif map_move[response] == "scissors" then
            outcome = Outcome.WIN
        end
    elseif map_move[move] == "scissors" then
        if map_move[response] == "paper" then
            outcome = Outcome.LOSE
        elseif map_move[response] == "rock" then
            outcome = Outcome.WIN
        end
    end

    print("DEBUG outcome: "..outcome)

    row_points = move_value[response] + outcome

    total_points = total_points + row_points
end

print("Total points: "..total_points)