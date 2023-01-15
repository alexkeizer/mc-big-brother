local component = require("component")
local geolyzer = component.geolyzer

local offsetx = -2
local offsetz = -2
local offsety = 0

local sizex = 5
local sizez = 5
local sizey = 1

local map = {}
local i = 1
--scan deployment area
for offsety = 1,14 do
    local scanData = geolyzer.scan(offsetx, offsetz, offsety, sizex, sizez, sizey)
    local j = 1
    for y = 0, sizey - 1 do
        for z = 0, sizez - 1 do
            for x = 0, sizex - 1 do
                -- alternatively when thinking in terms of 3-dimensional table: map[offsety + y][offsetz + z][offsetx + x] = scanData[i]
                map[i] = {posx = offsetx + x, posy = offsety + y, posz = offsetz + z, hardness = scanData[j]}
                --if map[i].hardness > 1 then
                --  print(offsety)
                --end
                j = j +1
                i = i + 1

            end
        end
    end
end

local y = 13
print(#(map))
local posx1,posz1 = 0,0
local posx2,posz2 = 0,0

-- deploment area tower n bots high topped with 2 blocks pointing north
-- find Y level bot and direction
for offsety = 1,13 do
    local j = sizex*sizez*sizey*offsety
    local block = false
    for i = 1, sizex*sizez*sizey do
        if map[i + j].hardness > 3 then
            block = true
            posx2,posz2 = posx1,posz1
            posx1,posz1 = map[i + j].posx, map[i + j].posz
            print(map[i + j].posx, map[i + j].posy, map[i + j].posz, map[i + j].hardness)
        end
    end
    if not block then
        y = y - offsety
        if posx1 == 0 then
           if posz1 == 1 then
            side = 2
           else
            side = 3
           end
        elseif posx1 == 1 then
            side = 4
        else
            side = 5
        end
        break
    end

    local cfg = loadfile("/home/skydaddy/config.lua")
    print(cfg.pos_x - posx1, cfg.pos_z - posz1,y - offsety, side)

end
--define new target pos by y bot
local spacing = 11
local chunk = 16
local robot = component.robot
for i in 1,y*spacing*chunk do robot.forward() end
