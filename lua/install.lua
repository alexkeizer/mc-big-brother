-- Setup an rc script to start open_socket on boot

local script=[==[

local thread = require("thread")

function start()
    loadfile("/home/skydaddy/open_socket.lua")()
end

]==]

f=io.open("/etc/rc.d/skydaddy.lua", "w");
f:write(script);
f:close();