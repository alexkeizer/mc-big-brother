

--
-- imports
--
local component = require("component")

--
-- configuration
--
local MAGIC = 0x6B7109BA
local cfg = loadfile("config.lua")
if (cfg ~= nil) then
    cfg = cfg()
end

if (cfg == nil) then
    print("config.lua not found, falling back to defaults");
    cfg = {
        server_addr = "0.tcp.ngrok.io:19382",
        world = 0,
        dimension = 0,
        chunk_x = 0,
        chunk_y = 0,
    }
end

local max_retries = 3;

--
-- functions
--
function write_all(socket, msg)
    while (string.len(msg) > 0) do
        n = socket.write(msg);
        if (n == nil) then
            return false;
        end
        msg = string.sub(msg, n+1);
    end
    return true;
end


--
-- main code
--
function main()
    local retries = max_retries;
    local addr = cfg.server_host .. cfg.server_addr

    while (true) do
        print(string.format("Connecting to %s", cfg.server_addr))

        local pings = 0
        local socket = component.internet.connect(cfg.server_addr);        
        socket.finishConnect();

        -- u16, u16, i32, i32
        local header = string.pack(">I4I2I2i4i4", MAGIC, cfg.world, cfg.dimension, cfg.pos_x, cfg.pos_z)

        if (write_all(socket, header)) then

            while (write_all(socket, "\0")) do
                pings = pings + 1
                print(string.format("[%s] Ping", os.date("%Y-%m-%d %X")))
                os.sleep(1)    
            end
        end

        if (pings < 3 and retries <= 0) then
            local delay = 8 + (math.random() * 4);
            print(string.format("Server is down, waiting %f seconds before retrying", delay));
            os.sleep(delay);
        else 
            if (pings < 3 and retries > 0) then
                retries = retries - 1;
            end

            if (pings > 10) then
                retries = max_retries;
            end



            print(string.format("Socket closed"))
        end

    end
end

main()