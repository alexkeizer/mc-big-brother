use std::convert::Infallible;
use std::time::Duration;

use log::{info, warn};
use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::*;


const MAGIC: u32 = 0x6B7109BA;

/// Reads the initial header sent by clients when establishing new connections
/// Expected format (all ints in big-endian order):
///     4 bytes                 magic value (must always be 0x6B7109BA)
///     4 bytes unsigned int    client version
///     2 bytes unsigned int    world     (i.e., minecraft server)
///     2 bytes signed int      dimension (e.g., 0 for Overworld, 1 for End, -1 for Nether)
///     4 bytes signed int      x coordinate
///     4 bytes signed int      z coordinate
async fn read_connection_header(socket: &mut TcpStream) -> anyhow::Result<ComputerData> {
    let magic = socket.read_u32().await?;
    if magic != MAGIC {
        anyhow::bail!("Invalid magic bytes. Expected {MAGIC:x}, found {magic:x}");
    }

    Ok(ComputerData {
        version: socket.read_u32().await?,
        world: WorldId(socket.read_u16().await?),
        dimension: DimensionId(socket.read_i16().await?),
        pos_x: socket.read_i32().await?,
        pos_z: socket.read_i32().await?,
    })
}

impl Server {
    async fn handle_connection(&self, mut socket: TcpStream) -> anyhow::Result<()> {
        info!("Handling new connection");

        let data = read_connection_header(&mut socket).await?;
        let computer = self.computer_repo.upsert(data).await?;

        info!("New connection from {:?}", computer);

        // In a loop, read data from the socket and write the data back.
        loop {


            //commands preferably just execute file
            let resmsg =r###"
                    local component = require("component")
                    local geolyzer = component.geolyzer
                    local robot = component.robot

                    local version = loadfile("/home/skydaddy/VERSION.lua")()
                    if version > 0 then
                        return true
                    end
                    file_descriptor = io.open("/home/skydaddy/config.lua", "w")
                    file_descriptor:write([[return {
                           -- server_host = 'keizer.dev',
                           server_host = '80.112.165.44',
                           server_port = 7777,
                           world = 1,
                           dimension = 1,
                           pos_x = -300,
                           pos_z = -700,
                           pos_y = 201,
                       }]])
file_descriptor:close()

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
                j = j +1
                i = i + 1

            end
        end
    end
end

local y = 13
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


end

--define new target pos by y bot
local spacing = 11
local chunk = 16
local moves = (y - offsety)*spacing*chunk
local cfg = loadfile("/home/skydaddy/config.lua")()
local mov_x = 0
local mov_z = 0
if side == 2 then
    mov_z = -moves
elseif side == 3 then
    mov_z = moves
elseif side == 4 then
    mov_x = -moves
elseif side == 5 then
    mov_x = moves
end
local pos_x = cfg.pos_x + mov_x + posx1
local pos_z = cfg.pos_z + mov_z  + posz1
local pos_y  = cfg.pos_y + y - offsety
for i =  1,moves do  while not robot.move(3) do end   end
                       file_descriptor = io.open("/home/skydaddy/config.lua", "w")
                        file_descriptor:write([[return {
                           -- server_host = 'keizer.dev',
                           server_host = '80.112.165.44',
                           server_port = 7777,
                           world = 1,
                           dimension = 1,
                           pos_x = ]] ..  pos_x .. [[,
                           pos_z = ]] ..  pos_z .. [[,
                           pos_y = ]] ..  pos_y .. [[,
                       }]])
                       file_descriptor:close()

                       file_descriptor = io.open("/home/skydaddy/VERSION.lua", "w")
                       file_descriptor:write([[return 1;]])
                       file_descriptor:close()
                       return true"###;
            let  res = EvalResponse::from(resmsg);

            Response::Eval(res).send_over(&mut socket).await;

            let mut buf = [0; 128];
            let read_fut = tokio::time::timeout(
                Duration::from_secs(5),
                socket.read(&mut buf),
            ).await;

            match read_fut {
                // timeout elapsed
                Err(_) => {
                    info!("Timeout elapsed, polling: {:?}", computer);
                    break;
                }

                Ok(r) => match r {
                    // socket closed
                    Ok(n) if n == 0 => {
                        warn!("socket closed by client {:?}", computer);
                        break;
                    }

                    Err(e) => {
                        warn!("failed to read from socket ({:?}); err = {:?}", computer, e);
                        break;
                    }
                    Ok(_) => {
                        info!("POLL from {:?}", computer);
                        self.computer_repo.insert_ping(&computer).await?;


                    }
                }
            };

        }

        Ok(())
    }

    pub async fn run_tcp(&'static self, addr: impl ToSocketAddrs) -> anyhow::Result<Infallible> {
        let listener = TcpListener::bind(addr).await?;
        info!("Bound socket to {}", listener.local_addr()?);

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(
                async {
                    match self.handle_connection(socket).await {
                        Ok(_) => {

                        }
                        Err(e) => {
                            warn!("Connection died unexpectedly: {}", e)
                        }
                    };
                }
            );
        }
    }
}