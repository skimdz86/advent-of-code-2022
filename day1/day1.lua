
local file, err = io.open("input.txt", "r")

if file==nil then
    print("Couldn't open file: "..err)
else

    local lines = file:lines()

    print("Contents of file:");

    local index = 0
    local global_max = 0
    local local_max = 0

    local top_three = {["top1"] = 0, ["top2"] = 0, ["top3"] = 0}

    for line in lines do 
        --print(line)

        if line == "" then 
            print("Counting..."..local_max)
            if local_max > top_three["top1"] then
                top_three["top3"] = top_three["top2"]
                top_three["top2"] = top_three["top1"]
                top_three["top1"] = local_max
            elseif local_max > top_three["top2"] then
                top_three["top3"] = top_three["top2"]
                top_three["top2"] = local_max
            elseif local_max > top_three["top3"] then           
                top_three["top3"] = local_max
            end
            local_max = 0
        else
            local_max = local_max + line
            --print("line "..line)
            --print("local_max_counting "..local_max)
        end


        index = index + 1
        
        --safety net
        --if index > 30 then
        --    break
        --end
    end

    --print("Global max: "..global_max)
    local sum = 0
    for k,v in pairs(top_three) do
        sum = sum + v
    end
    
    print("Total snacks in top three: "..sum)

end
