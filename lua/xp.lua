local component = require('component')  \r\n  \
local robot = component.robot  \r\n \
local xp = component.experience \r\n\
for i=1,16 do robot.select(i) xp.consume() end \r\n \
if xp.level() > 29 then \r\n \
    robot.move(2) \r\n \
else \r\n \
    robot.move(1) \r\n \
end \r\n \
return true