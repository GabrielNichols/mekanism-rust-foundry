package dev.meklab;

import com.google.gson.*;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.Container;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.ModList;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.server.ServerStartedEvent;
import net.neoforged.neoforge.event.tick.ServerTickEvent;

@Mod("meklab_bridge")
public final class ServerBridge {
    static final BlockPos TARGET = new BlockPos(0,80,0);
    static final BlockPos BARREL = new BlockPos(4,80,0);
    private final String auditMode = System.getenv("MEKLAB_AUDIT");
    private JsonObject auditInitial;
    private boolean auditComplete;
    private final ChunkProbe chunks = new ChunkProbe(System.getenv("MEKLAB_CHUNK_CONTROL"));
    private Wire wire;
    private JsonObject pending;
    private long targetTime;
    private long previousTime;
    private String observerMode;
    private JsonArray samples;
    private JsonArray timings;
    private long tickStarted;
    private long batchStarted;
    private long cpuStarted;
    private long gcStarted;
    private long gcCountStarted;
    public ServerBridge(IEventBus bus) {
        if ("disabled".equals(System.getenv("MEKLAB_OBSERVER"))) {
            System.out.println("MekLab observer disabled before hook registration");
            return;
        }
        if (chunks.enabled()) NeoForge.EVENT_BUS.addListener(chunks::loaded);
        NeoForge.EVENT_BUS.addListener(this::started);
        NeoForge.EVENT_BUS.addListener(this::tick);
        NeoForge.EVENT_BUS.addListener((ServerTickEvent.Pre event)->tickStarted=System.nanoTime());
        if (System.getenv("MEKLAB_ROLE") != null && System.getenv("MEKLAB_ROLE").equals("client")) {
            try { Class.forName("dev.meklab.ClientBridge").getMethod("register").invoke(null); }
            catch (ReflectiveOperationException error) { throw new IllegalStateException(error); }
        }
    }
    private void started(ServerStartedEvent event) {
        MinecraftServer server=event.getServer();
        server.getCommands().performPrefixedCommand(server.createCommandSourceStack().withSuppressedOutput(), "tick freeze");
        JsonObject hello=new JsonObject(); hello.addProperty("minecraft", server.getServerVersion());
        hello.addProperty("mekanism", ModList.get().getModContainerById("mekanism").orElseThrow().getModInfo().getVersion().toString());
        hello.addProperty("port", server.getPort());
        JsonArray capabilities=new JsonArray();for(String c:new String[]{"tick_samples_v1","machine_probe_v1","neoforge_fe_v1","nbt_typed_v1","tick_measurements_v1"})capabilities.add(c);hello.add("capabilities",capabilities);
        wire=new Wire("server", hello);
    }
    private void tick(ServerTickEvent.Post event) {
        if(wire==null) return;
        MinecraftServer server=event.getServer();
        if(!server.isSameThread())throw new IllegalStateException("control outside server owner thread");
        if(pending!=null && server.overworld().getGameTime()>previousTime) {
            long now=server.overworld().getGameTime();
            if(now!=previousTime+1)throw new IllegalStateException("uncaptured world tick gap");
            long tickNs=System.nanoTime()-tickStarted,captureStart=System.nanoTime();
            JsonObject state=null;
            if(observerMode.equals("every_tick")||now>=targetTime) {
                state=snapshot(server);JsonObject sample=new JsonObject();sample.addProperty("tick",Long.toString(now));sample.addProperty("phase","end_server_tick");sample.addProperty("owner_thread",server.isSameThread());sample.add("state",state);samples.add(sample);
            }
            JsonObject timing=new JsonObject();timing.addProperty("tick",Long.toString(now));timing.addProperty("bridge_pre_to_post_ns",Long.toString(tickNs));timing.addProperty("snapshot_ns",Long.toString(state==null?0:System.nanoTime()-captureStart));timings.add(timing);previousTime=now;
            if(now>=targetTime) {
                JsonObject measured=new JsonObject();measured.add("ticks",timings);measured.addProperty("batch_wall_ns",Long.toString(System.nanoTime()-batchStarted));measured.addProperty("process_cpu_ns",Long.toString(cpuTime()-cpuStarted));measured.addProperty("gc_pause_ms",Long.toString(gcTime()-gcStarted));measured.addProperty("gc_collections",Long.toString(gcCount()-gcCountStarted));measured.addProperty("heap_used_bytes",Long.toString(java.lang.management.ManagementFactory.getMemoryMXBean().getHeapMemoryUsage().getUsed()));
                JsonObject extra=new JsonObject();extra.add("tick_samples",samples);extra.add("measurement",measured);wire.reply(pending,state,extra);pending=null;
            }
        }
        if(pending!=null) return;
        JsonObject cmd;
        while((cmd=wire.queue.poll())!=null) {
            try {
                String action=cmd.get("action").getAsString();
                JsonObject extra=new JsonObject();
                switch(action) {
                    case "setup_machine": MachineProbe.setup(server);break;
                    case "insert_input": MachineProbe.insert(server,cmd);break;
                    case "charge_machine": extra.add("energy_receipt",MachineProbe.charge(server,cmd));break;
                    case "drain_machine": extra.add("energy_receipt",MachineProbe.drain(server,cmd));break;
                    case "extract_output": extra.add("extraction_receipt",MachineProbe.extract(server,cmd));break;
                    case "fixture_block_output": MachineProbe.blockOutput(server);extra.addProperty("fixture_level","administrative_nbt");break;
                    case "save": server.saveEverything(true,true,true);break;
                    case "read": break;
                    case "effective_catalog": extra.add("catalog", EffectiveCatalog.capture(server)); break;
                    case "barrier_sync": {
                        var player = server.getPlayerList().getPlayers().getFirst();
                        if (player.containerMenu == player.inventoryMenu || !server.tickRateManager().isFrozen()) {
                            throw new IllegalStateException("barrier requires a frozen open container");
                        }
                        player.containerMenu.sendAllDataToRemote();
                        break;
                    }
                    case "chunk_probe": extra.add("chunk_probe", chunks.run(server)); break;
                    case "chunk_witness": extra.add("chunk_witness", chunks.followup(server)); break;
                    case "audit_read": auditRead(server,cmd); continue;
                    case "fixture_barrel": server.overworld().setBlock(BARREL,Blocks.BARREL.defaultBlockState(),3); break;
                    case "insert_barrel":
                        if(!(server.overworld().getBlockEntity(BARREL) instanceof Container container)) throw new IllegalStateException("barrel fixture missing");
                        container.setItem(0,new ItemStack(Items.DIAMOND,13));
                        server.overworld().getBlockEntity(BARREL).setChanged();
                        break;
                    case "setup_player": {
                        ServerPlayer player=server.getPlayerList().getPlayers().getFirst();
                        for(int x=-1;x<=1;x++) for(int z=2;z<=4;z++) server.overworld().setBlock(new BlockPos(x,79,z),Blocks.STONE.defaultBlockState(),3);
                        server.overworld().setBlock(new BlockPos(0,79,0),Blocks.STONE.defaultBlockState(),3);
                        server.overworld().setBlock(TARGET,Blocks.AIR.defaultBlockState(),3);
                        player.connection.teleport(0.5,80,3.5,180,28.4f);
                        player.getInventory().clearContent();
                        player.getInventory().setItem(0,new ItemStack(BuiltInRegistries.ITEM.get(ResourceLocation.parse("mekanism:enrichment_chamber")),2));
                        player.getInventory().setItem(9,new ItemStack(Items.IRON_ORE,3));
                        player.inventoryMenu.broadcastChanges();
                        break;
                    }
                    case "step": {
                        int ticks=cmd.get("ticks").getAsInt(); if(ticks<1||ticks>20) throw new IllegalArgumentException("ticks outside 1..20");
                        targetTime=server.overworld().getGameTime()+ticks;
                        previousTime=server.overworld().getGameTime();observerMode=cmd.get("observer_mode").getAsString();if(!observerMode.equals("every_tick")&&!observerMode.equals("terminal"))throw new IllegalArgumentException("observer mode");
                        samples=new JsonArray();timings=new JsonArray();batchStarted=System.nanoTime();cpuStarted=cpuTime();gcStarted=gcTime();gcCountStarted=gcCount();
                        server.getCommands().performPrefixedCommand(server.createCommandSourceStack().withSuppressedOutput(),"tick step "+ticks);
                        pending=cmd; return;
                    }
                    case "close": wire.reply(cmd,snapshot(server)); server.halt(false); return;
                    default: throw new IllegalArgumentException("unsupported action");
                }
                wire.reply(cmd,snapshot(server),extra);
            } catch(Exception error) { wire.error(cmd,error); }
        }
    }
    private void auditRead(MinecraftServer server, JsonObject request) {
        if (!"server".equals(System.getenv("MEKLAB_ROLE")) || auditMode == null
                || !java.util.Set.of("clean", "stale_snapshot", "after_read_inventory").contains(auditMode)
                || !server.isSameThread() || !server.tickRateManager().isFrozen()
                || !server.getPlayerList().getPlayers().isEmpty()) {
            throw new IllegalStateException("audit requires a dedicated frozen test server");
        }
        String checkpoint = request.get("checkpoint").getAsString();
        JsonObject state = snapshot(server);
        if (checkpoint.equals("initial") && auditInitial == null) {
            auditInitial = state.deepCopy();
        } else if (checkpoint.equals("cut") && auditInitial != null && !auditComplete) {
            JsonObject machine = state.getAsJsonObject("machine");
            if (machine.get("progress").getAsInt() != 73 || machine.get("iron_ore").getAsInt() != 1) {
                throw new IllegalStateException("audit requires one ore and progress 73");
            }
            auditComplete = true;
            if (auditMode.equals("stale_snapshot")) {
                JsonElement currentTick = state.get("tick");
                state = auditInitial.deepCopy();
                state.add("tick", currentTick);
            } else if (auditMode.equals("after_read_inventory")) {
                // Deliberate one-shot mutation after the captured state has been detached.
                server.getCommands().performPrefixedCommand(server.createCommandSourceStack().withSuppressedOutput(),
                        "data modify block 0 80 0 items[0].item.count set value 2");
            }
        } else {
            throw new IllegalStateException("unexpected audit checkpoint sequence");
        }
        JsonObject envelope = new JsonObject();
        envelope.addProperty("checkpoint", checkpoint);
        envelope.addProperty("phase", "server_post_command");
        envelope.addProperty("owner_thread", server.isSameThread());
        JsonObject extra = new JsonObject();
        extra.add("audit", envelope);
        wire.reply(request, state, extra);
    }
    private static JsonObject stack(ItemStack stack) {
        JsonObject out=new JsonObject(); out.addProperty("item",BuiltInRegistries.ITEM.getKey(stack.getItem()).toString()); out.addProperty("count",stack.getCount()); return out;
    }
    static JsonObject snapshot(MinecraftServer server) {
        if(!server.isSameThread())throw new IllegalStateException("snapshot outside owner thread");
        JsonObject out=new JsonObject(); out.addProperty("tick",Long.toString(server.overworld().getGameTime()));
        out.addProperty("owner_thread", server.isSameThread());
        out.addProperty("target_block",BuiltInRegistries.BLOCK.getKey(server.overworld().getBlockState(TARGET).getBlock()).toString());
        var blockEntity=server.overworld().getBlockEntity(TARGET);
        if(blockEntity!=null) out.addProperty("target_nbt",blockEntity.saveWithFullMetadata(server.registryAccess()).toString());
        if(blockEntity!=null&&BuiltInRegistries.BLOCK.getKey(server.overworld().getBlockState(TARGET).getBlock()).toString().equals("mekanism:enrichment_chamber"))out.add("machine",MachineProbe.snapshot(server));
        var barrel=server.overworld().getBlockEntity(BARREL);
        out.addProperty("barrel_count",barrel instanceof Container c?c.getItem(0).getCount():0);
        JsonArray players=new JsonArray();
        for(ServerPlayer player:server.getPlayerList().getPlayers()) {
            JsonObject p=new JsonObject(); p.addProperty("name",player.getGameProfile().getName()); p.addProperty("menu_id",player.containerMenu.containerId);
            p.addProperty("menu_open",player.containerMenu!=player.inventoryMenu);
            int machineOre=0;
            for(var slot:player.containerMenu.slots) if(slot.container!=player.getInventory()&&slot.getItem().is(Items.IRON_ORE)) machineOre+=slot.getItem().getCount();
            p.addProperty("machine_iron_ore",machineOre);
            p.addProperty("menu_state_id", player.containerMenu.getStateId());
            p.add("source", stack(player.getInventory().getItem(9)));
            p.add("hotbar",stack(player.getInventory().getItem(0))); p.add("cursor",stack(player.containerMenu.getCarried()));
            JsonArray slots=new JsonArray(); for(var slot:player.containerMenu.slots) slots.add(stack(slot.getItem()));
            p.add("slots",slots); players.add(p);
        }
        out.add("players",players); return out;
    }
    private static long cpuTime(){return ((com.sun.management.OperatingSystemMXBean)java.lang.management.ManagementFactory.getOperatingSystemMXBean()).getProcessCpuTime();}
    private static long gcTime(){return java.lang.management.ManagementFactory.getGarbageCollectorMXBeans().stream().mapToLong(b->Math.max(0,b.getCollectionTime())).sum();}
    private static long gcCount(){return java.lang.management.ManagementFactory.getGarbageCollectorMXBeans().stream().mapToLong(b->Math.max(0,b.getCollectionCount())).sum();}
}
