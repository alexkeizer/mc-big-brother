local component = require("component")
local sides = require("sides")
trans = {}
ass = {}

for addy, name in component.list() do
  if(name == "transposer") then
    table.insert(trans, component.proxy(addy))
  elseif(name == "assembler") then
    table.insert(ass, component.proxy(addy))
  end
end

--transferItem(sourceSide:int, sinkSide:int, count:int, sourceSlot:int, sinkSlot:int):transfers:int
function fillSlots(comp)
  comp.transferItem(5,1,1,5,1) --case
  comp.transferItem(5,1,1,1,20) --eeprom
  comp.transferItem(4,1,1,1,17) --cpu
  comp.transferItem(4,1,1,2,18) --ram
  comp.transferItem(4,1,1,2,19)
  comp.transferItem(0,1,1,1,21) --disk
  comp.transferItem(4,1,1,4,7) --xp
  comp.transferItem(4,1,1,5,14) --internet
  comp.transferItem(4,1,1,6,15) --wireless
  comp.transferItem(4,1,1,8,10) --hover
  comp.transferItem(4,1,1,9,5) --angel
  --comp.transferItem(5,1,1,2,2) --tier 1 upgrade slots
  comp.transferItem(5,1,1,2,3)
  comp.transferItem(5,1,1,3,8) --interaction upgrade
  --comp.transferItem(sides.left,sides.bottom,1,4,11) --inv upgrade
  comp.transferItem(5,1,1,4,12)
  comp.transferItem(5,1,1,6,13) --geo
  comp.transferItem(5,1,1,7,9) --solar
  --comp.transferItemttom,1,8,9) --coal
  comp.transferItem(5,1,1,9,6) --database
end

-- fills assembler slots
for i, transposer in ipairs(trans) do
  print(i)
  fillSlots(transposer)
end

-- starts assembling
for i, assembler in ipairs(ass) do
  print(i)
  assembler.start()
end

os.sleep(330)

-- transfer to the me system
for i, transposer in ipairs(trans) do
  transposer.transferItem(1,5,1,1,8)
end