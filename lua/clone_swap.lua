local fs = require("filesystem")
local shell = require("shell")
local component = require("component")
local sides = require("sides")
local pc = require("computer")

local pc_case_side = sides.top
local iface_side = sides.bottom
local chest_side = sides.right

local master_hdd_slot = 7
local copy_hdd_slot = 6

local function getRootDriveGuid()
  for filesys_comp, path in fs.mounts() do
    if(path == "/") then
      return filesys_comp.address
    end
  end
  return nil
end

local function getSecondaryDriveGuid(root_prefix)
  for filesys_comp, path in fs.mounts() do
    if(string.find(path, "mnt") and string.find(path, root_prefix) == nil) then
      return filesys_comp.address
    end
  end
  return nil
end

local function getTransposer()
  for addy, name in component.list("transposer") do
    if(name == "transposer") then
      return component.proxy(addy)
    end
  end
  return nil
end

local src_prefix = getRootDriveGuid():sub(0,3)
local dest_prefix = nil
local transposer = getTransposer()
local drive_count = 0

local chest_slot = 1
if(transposer ~= nil) then
  while true do
    transposer.transferItem(iface_side, pc_case_side, 1, 1, copy_hdd_slot)
    os.sleep(1)
    dest_prefix = getSecondaryDriveGuid(src_prefix):sub(0,3)

    shell.execute("hddclone.lua " .. src_prefix ..  " " .. dest_prefix .. " SkyDaddy")
    os.sleep(1)
    --chest_slot = 1
    transposer.transferItem(pc_case_side, chest_side, 1, copy_hdd_slot, chest_slot)
    --  chest_slot = chest_slot +1

    pc.beep(881)
    drive_count = drive_count + 1
    print("Drives finished: " .. drive_count)
    os.sleep(.5)
  end
else
  print("maybe attach a transposer")