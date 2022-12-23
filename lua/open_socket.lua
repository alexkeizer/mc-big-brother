--
-- imports
--
local component = require("component")
local internet = require("internet")


--
-- functions
--
local logfile = io.open("/home/skydaddy.log", "a");

function log(msg)
    logfile:write(
        string.format("[%s] %s\n", os.date("%Y-%m-%d %X"), msg)
    )
    logfile:flush();
end

function log_response(msg)
    log("RESPONSE: " .. msg)
end

function handle_responses(socket)
    function handle()
        -- handlers for all response codes
        local handlers = {
            -- noop
            [0] = function(socket)
                log_response("noop")
            end,

            -- eval
            [1] = function(socket)
                local n = socket:read(4);
                n = string.unpack(">I4", n);

                local data = socket:read(n);
                local f = load(data);

                local reboot = f();
                if (reboot == true) then
                    os.execute("reboot");
                end
            end,
        }


        local code = socket:read(1);

        -- If code is an empty string, there are no more responses to be handled
        if code == nil or code:len() == 0 then
            return false;
        end


        code = code:byte();
        if handlers[code] then
            handlers[code](socket)
        else
            log_response(string.format("illegal response code (%sd", code))
        end

        return true;
    end

    -- We do "blocking" reads by setting the timeout to a very small value
    local timeout = socket.readTimeout;
    socket:setTimeout(0.1);


    local res = true;
    local value = "";
    while (res and value) do
        res, value = pcall(handle);
    end

    -- Restore timeout to original value
    socket:setTimeout(timeout)

    -- If the loop was broken with `res == false`, then `value` is an error
    -- If the error was anything but a timeout, propagate
    if (not res and not value:match("timeout")) then
        error(value)
    end
end


--
-- configuration
--
local MAGIC = 0x6B7109BA
local version = loadfile("/home/skydaddy/VERSION.lua")()
local cfg = loadfile("/home/skydaddy/config.lua")
if (cfg ~= nil) then
    cfg = cfg()
end

if (cfg == nil) then
    log("config.lua not found, falling back to defaults");
    cfg = {
        server_host = "127.0.0.1",
        server_port = "7777",
        world = 0,
        dimension = 0,
        chunk_x = 0,
        chunk_y = 0,
    }
end

local max_retries = 3;


--
-- main code
--
function main()
    local retries = max_retries;

    while (true) do
        log(string.format("Connecting to %s:%s", cfg.server_host, cfg.server_port));

        local pings = 0
        local socket = internet.open(cfg.server_host, cfg.server_port);
        socket:setTimeout(2);


        -- u16, u16, i32, i32
        local header = string.pack(">I4I4I2I2i4i4", MAGIC, version, cfg.world, cfg.dimension, cfg.pos_x, cfg.pos_z)

        if (socket:write(header) and socket:flush()) then
            while (socket:write("\0") and socket:flush()) do
                pings = pings + 1
                log("Ping")

                handle_responses(socket)

                os.sleep(3)

                if (pings > 5) then
                    os.exit()
                end
            end
        end

        if (pings < 3 and retries <= 0) then
            local delay = 8 + (math.random() * 4);
            log(string.format("Server is down, waiting %f seconds before retrying", delay));
            os.sleep(delay);
        else 
            if (pings < 3 and retries > 0) then
                retries = retries - 1;
            end

            if (pings > 10) then
                retries = max_retries;
            end



            log(string.format("Socket closed"))
        end

    end
end

main()