package dev.meklab;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import java.util.Arrays;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.ListTag;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.MinecraftServer;
import net.minecraft.tags.TagKey;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.neoforge.capabilities.Capabilities;

/** Administrative setup and read-only witnesses for the pinned Basic Enriching Factory. */
final class FactoryProbe {
    static final String BLOCK = "mekanism:basic_enriching_factory";
    private static final String RECIPE = "mekanism:processing/iron/dust/from_ore";
    private static final String ENRICHING = "mekanism:enriching";
    private static final ResourceLocation DUST = ResourceLocation.parse("mekanism:dust_iron");
    private static final String[] LANES = {"A", "B", "C"};

    static void setup(MinecraftServer server, boolean blocked) {
        if (!server.isSameThread() || !server.tickRateManager().isFrozen()
                || !server.getPlayerList().getPlayers().isEmpty()) {
            throw new IllegalStateException("factory fixture requires a frozen dedicated server");
        }
        MachineProbe.setup(server, BLOCK);
        var level = server.overworld();
        var entity = level.getBlockEntity(ServerBridge.TARGET);
        var tag = entity.saveWithFullMetadata(server.registryAccess());
        if (!tag.getList("items", 10).isEmpty()) {
            throw new IllegalStateException("factory fixture requires an empty inventory");
        }
        var handler = level.getCapability(Capabilities.ItemHandler.BLOCK, ServerBridge.TARGET, (Direction) null);
        var ore = new ItemStack(Items.IRON_ORE);
        if (handler == null || handler.getSlots() != 7
                || !ore.is(TagKey.create(Registries.ITEM, ResourceLocation.parse("c:ores/iron")))
                || !handler.isItemValid(0, ore) || !handler.isItemValid(2, ore)) {
            throw new IllegalStateException("factory input slots differ from the pinned layout");
        }
        var dust = new ItemStack(BuiltInRegistries.ITEM.get(DUST));
        int limit = Math.min(dust.getMaxStackSize(), handler.getSlotLimit(1));
        if (limit < 2 || dust.isEmpty()) {
            throw new IllegalStateException("factory output stack limit unavailable");
        }
        var items = new ListTag();
        items.add(slot(server, 0, ore));
        items.add(slot(server, 2, ore));
        if (blocked) {
            items.add(slot(server, 1, dust.copyWithCount(limit)));
        }
        tag.put("items", items);
        entity.loadWithComponents(tag, server.registryAccess());
        entity.setChanged();
    }

    private static CompoundTag slot(MinecraftServer server, int slot, ItemStack stack) {
        var entry = new CompoundTag();
        entry.putByte("slot", (byte) slot);
        entry.put("item", stack.save(server.registryAccess()));
        return entry;
    }

    private static JsonObject stack(ItemStack stack, int slot) {
        var result = new JsonObject();
        result.addProperty("slot", slot);
        result.addProperty("id", BuiltInRegistries.ITEM.getKey(stack.getItem()).toString());
        result.addProperty("count", stack.getCount());
        return result;
    }

    static JsonObject snapshot(MinecraftServer server) {
        var level = server.overworld();
        var entity = level.getBlockEntity(ServerBridge.TARGET);
        var tag = entity.saveWithFullMetadata(server.registryAccess());
        int[] progress = tag.getIntArray("progress");
        var items = level.getCapability(Capabilities.ItemHandler.BLOCK, ServerBridge.TARGET, (Direction) null);
        var energy = level.getCapability(Capabilities.EnergyStorage.BLOCK, ServerBridge.TARGET, Direction.UP);
        if (progress.length != LANES.length || items == null || items.getSlots() != 7 || energy == null
                || !tag.contains("sorting", 1) || tag.getBoolean("sorting")) {
            throw new IllegalStateException("factory lane, sorting or FE bindings unavailable");
        }
        var saved = new ItemStack[items.getSlots()];
        Arrays.fill(saved, ItemStack.EMPTY);
        boolean[] present = new boolean[saved.length];
        for (var entry : tag.getList("items", 10)) {
            var compound = (CompoundTag) entry;
            int slot = compound.getByte("slot");
            if (!compound.contains("slot", 1) || slot < 0 || slot >= saved.length || present[slot]) {
                throw new IllegalStateException("invalid persisted factory slot " + slot);
            }
            present[slot] = true;
            saved[slot] = ItemStack.parseOptional(server.registryAccess(), compound.getCompound("item"));
        }
        for (int slot = 0; slot < saved.length; slot++) {
            if (!ItemStack.matches(saved[slot], items.getStackInSlot(slot))) {
                throw new IllegalStateException("factory handler differs from NBT at slot " + slot);
            }
        }
        var lanes = new JsonObject();
        for (int i = 0; i < LANES.length; i++) {
            var lane = new JsonObject();
            int input = i * 2;
            lane.addProperty("progress", progress[i]);
            lane.add("input", stack(items.getStackInSlot(input), input));
            lane.add("output", stack(items.getStackInSlot(input + 1), input + 1));
            lanes.add(LANES[i], lane);
        }
        var recipe = server.getRecipeManager().getRecipes().stream()
                .filter(holder -> holder.id().toString().equals(RECIPE))
                .findFirst().orElseThrow(() -> new IllegalStateException("factory reference recipe missing")).value();
        if (!ENRICHING.equals(BuiltInRegistries.RECIPE_TYPE.getKey(recipe.getType()).toString())
                || !ENRICHING.equals(BuiltInRegistries.RECIPE_SERIALIZER.getKey(recipe.getSerializer()).toString())) {
            throw new IllegalStateException("factory reference recipe type or serializer drifted");
        }
        var result = new JsonObject();
        result.add("lanes", lanes);
        result.add("nbt", NbtJson.encode(tag));
        result.addProperty("recipe", RECIPE);
        var packs = new JsonArray();
        server.getPackRepository().getSelectedIds().stream().sorted().forEach(packs::add);
        result.add("packs", packs);
        result.addProperty("energy_fe", Integer.toString(energy.getEnergyStored()));
        result.addProperty("capacity_fe", Integer.toString(energy.getMaxEnergyStored()));
        result.addProperty("output_limit", Math.min(items.getSlotLimit(1),
                new ItemStack(BuiltInRegistries.ITEM.get(DUST)).getMaxStackSize()));
        result.addProperty("chunk_forced", level.getForcedChunks()
                .contains(new net.minecraft.world.level.ChunkPos(ServerBridge.TARGET).toLong()));
        return result;
    }
}
