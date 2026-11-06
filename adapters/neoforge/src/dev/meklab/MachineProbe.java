package dev.meklab;
import com.google.gson.*;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.MinecraftServer;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.neoforge.capabilities.Capabilities;
import net.neoforged.neoforge.items.IItemHandler;

/** Uses public game/NeoForge APIs only. No Mekanism implementation calls. */
final class MachineProbe {
    static void setup(MinecraftServer server) {
        setup(server, "mekanism:enrichment_chamber");
    }
    static void setup(MinecraftServer server, String block) {
        server.getCommands().performPrefixedCommand(server.createCommandSourceStack().withSuppressedOutput(),"forceload add 0 0");
        if(!server.overworld().getForcedChunks().contains(new net.minecraft.world.level.ChunkPos(ServerBridge.TARGET).toLong()))throw new IllegalStateException("target chunk anchor not established");
        var level=server.overworld();level.setBlock(ServerBridge.TARGET,Blocks.AIR.defaultBlockState(),3);
        level.setBlock(ServerBridge.TARGET,BuiltInRegistries.BLOCK.get(ResourceLocation.parse(block)).defaultBlockState(),3);
        var state=level.getBlockState(ServerBridge.TARGET);var stack=new ItemStack(state.getBlock().asItem());
        var entity=level.getBlockEntity(ServerBridge.TARGET);if(entity==null)throw new IllegalStateException("placement missing block entity");
        entity.applyComponentsFromItemStack(stack);
        state.getBlock().setPlacedBy(level,ServerBridge.TARGET,state,null,stack);
        entity.setChanged();
    }
    static Direction side(JsonObject request) {String s=request.get("side").getAsString();if(s.equals("unsided"))return null;Direction d=Direction.byName(s);if(d==null)throw new IllegalArgumentException("unknown side");return d;}
    static void insert(MinecraftServer server,JsonObject request) {
        var handler=server.overworld().getCapability(Capabilities.ItemHandler.BLOCK,ServerBridge.TARGET,side(request));
        if(handler==null)throw new IllegalStateException("item capability unavailable");
        int slot=request.get("slot").getAsInt(),count=request.get("count").getAsInt();
        var key=ResourceLocation.parse(request.get("item").getAsString());
        if(slot<0||slot>=handler.getSlots()||count<1||count>64||!BuiltInRegistries.ITEM.containsKey(key))throw new IllegalArgumentException("invalid input binding");
        ItemStack left=handler.insertItem(slot,new ItemStack(BuiltInRegistries.ITEM.get(key),count),false);
        if(!left.isEmpty())throw new IllegalStateException("input rejected or partial; no retry");
    }
    static JsonObject charge(MinecraftServer server,JsonObject request) {
        var energy=server.overworld().getCapability(Capabilities.EnergyStorage.BLOCK,ServerBridge.TARGET,side(request));
        if(energy==null)throw new IllegalStateException("FE capability unavailable");
        int amount=request.get("amount").getAsInt();if(amount<1||amount>1_000_000)throw new IllegalArgumentException("invalid energy amount");
        int before=energy.getEnergyStored(),accepted=energy.receiveEnergy(amount,false),after=energy.getEnergyStored();
        JsonObject receipt=new JsonObject();receipt.addProperty("unit","FE");receipt.addProperty("requested",Integer.toString(amount));receipt.addProperty("accepted",Integer.toString(accepted));receipt.addProperty("before",Integer.toString(before));receipt.addProperty("after",Integer.toString(after));return receipt;
    }
    static JsonObject snapshot(MinecraftServer server) {
        var entity=server.overworld().getBlockEntity(ServerBridge.TARGET);if(entity==null)throw new IllegalStateException("machine missing");
        JsonObject result=new JsonObject();JsonArray sides=new JsonArray();
        result.addProperty("chunk_forced",server.overworld().getForcedChunks().contains(new net.minecraft.world.level.ChunkPos(ServerBridge.TARGET).toLong()));
        for(String name:new String[]{"unsided","down","up","north","south","west","east"}){
            Direction d=name.equals("unsided")?null:Direction.byName(name);JsonObject s=new JsonObject();s.addProperty("side",name);
            var energy=server.overworld().getCapability(Capabilities.EnergyStorage.BLOCK,ServerBridge.TARGET,d);s.addProperty("energy_available",energy!=null);
            if(energy!=null){s.addProperty("energy_unit","FE");s.addProperty("energy",Integer.toString(energy.getEnergyStored()));s.addProperty("energy_capacity",Integer.toString(energy.getMaxEnergyStored()));s.addProperty("can_receive",energy.canReceive());s.addProperty("can_extract",energy.canExtract());s.addProperty("simulated_fe_extractable",energy.extractEnergy(1000000,true));s.addProperty("simulated_fe_receivable",energy.receiveEnergy(1000000,true));}
            var items=server.overworld().getCapability(Capabilities.ItemHandler.BLOCK,ServerBridge.TARGET,d);s.addProperty("items_available",items!=null);
            if(items!=null){JsonArray slots=new JsonArray();for(int i=0;i<items.getSlots();i++){JsonObject slot=new JsonObject();ItemStack item=items.getStackInSlot(i);slot.addProperty("slot",i);slot.addProperty("id",BuiltInRegistries.ITEM.getKey(item.getItem()).toString());slot.addProperty("count",item.getCount());slot.addProperty("limit",items.getSlotLimit(i));slot.addProperty("item_max_stack",item.getMaxStackSize());slot.addProperty("accepts_iron_ore",items.isItemValid(i,new ItemStack(net.minecraft.world.item.Items.IRON_ORE)));slot.addProperty("iron_ore_simulated_accepted",1-items.insertItem(i,new ItemStack(net.minecraft.world.item.Items.IRON_ORE),true).getCount());slot.addProperty("simulated_extractable_count",items.extractItem(i,64,true).getCount());slots.add(slot);}s.add("slots",slots);}
            sides.add(s);
        }
        result.add("sides",sides);
        var all=server.overworld().getCapability(Capabilities.ItemHandler.BLOCK,ServerBridge.TARGET,(Direction)null);int ore=0,dust=0;
        if(all==null)throw new IllegalStateException("unsided inventory witness unavailable");
        for(int i=0;i<all.getSlots();i++){var stack=all.getStackInSlot(i);var id=BuiltInRegistries.ITEM.getKey(stack.getItem()).toString();if(id.equals("minecraft:iron_ore"))ore+=stack.getCount();if(id.equals("mekanism:dust_iron"))dust+=stack.getCount();}
        result.addProperty("iron_ore",ore);result.addProperty("dust_iron",dust);
        var nbt=entity.saveWithFullMetadata(server.registryAccess());
        if(!nbt.contains("progress",3))throw new IllegalStateException("progress int not observable");
        result.addProperty("progress",nbt.getInt("progress"));result.add("nbt",NbtJson.encode(nbt));
        return result;
    }
    static JsonObject drain(MinecraftServer server,JsonObject request) {
        var energy=server.overworld().getCapability(Capabilities.EnergyStorage.BLOCK,ServerBridge.TARGET,side(request));
        if(energy==null)throw new IllegalStateException("FE capability unavailable");
        int amount=request.get("amount").getAsInt();if(amount<1||amount>1_000_000)throw new IllegalArgumentException("invalid energy amount");
        int before=energy.getEnergyStored(),removed=energy.extractEnergy(amount,false),after=energy.getEnergyStored();
        JsonObject receipt=new JsonObject();receipt.addProperty("unit","FE");receipt.addProperty("requested",Integer.toString(amount));receipt.addProperty("removed",Integer.toString(removed));receipt.addProperty("before",Integer.toString(before));receipt.addProperty("after",Integer.toString(after));return receipt;
    }
    static JsonObject extract(MinecraftServer server,JsonObject request) {
        var handler=server.overworld().getCapability(Capabilities.ItemHandler.BLOCK,ServerBridge.TARGET,side(request));
        if(handler==null)throw new IllegalStateException("item capability unavailable");
        int slot=request.get("slot").getAsInt(),count=request.get("count").getAsInt();
        if(slot<0||slot>=handler.getSlots()||count<1||count>64)throw new IllegalArgumentException("invalid extraction binding");
        ItemStack extracted=handler.extractItem(slot,count,false);
        if(extracted.getCount()!=count)throw new IllegalStateException("partial extraction; no retry");
        JsonObject out=new JsonObject();out.addProperty("id",BuiltInRegistries.ITEM.getKey(extracted.getItem()).toString());out.addProperty("count",extracted.getCount());return out;
    }
    static void blockOutput(MinecraftServer server) {
        var entity=server.overworld().getBlockEntity(ServerBridge.TARGET);if(entity==null)throw new IllegalStateException("machine missing");
        var tag=entity.saveWithFullMetadata(server.registryAccess());var items=tag.getList("items",10);int matched=0;
        for(int i=0;i<items.size();i++){var item=items.getCompound(i).getCompound("item");if(item.getString("id").equals("mekanism:dust_iron")){if(item.getInt("count")!=2)throw new IllegalStateException("fixture requires actual two-dust output");item.putInt("count",64);matched++;}}
        if(matched!=1)throw new IllegalStateException("fixture requires unique actual recipe output");
        entity.loadWithComponents(tag,server.registryAccess());entity.setChanged();
        var handler=server.overworld().getCapability(Capabilities.ItemHandler.BLOCK,ServerBridge.TARGET,(Direction)null);
        if(handler==null)throw new IllegalStateException("output capability missing after fixture");
        int dust=0;for(int i=0;i<handler.getSlots();i++)if(BuiltInRegistries.ITEM.getKey(handler.getStackInSlot(i).getItem()).toString().equals("mekanism:dust_iron"))dust+=handler.getStackInSlot(i).getCount();
        if(dust!=64)throw new IllegalStateException("blocked output fixture not applied");
    }
}




