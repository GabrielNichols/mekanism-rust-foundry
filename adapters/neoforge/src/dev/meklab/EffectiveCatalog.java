package dev.meklab;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import java.util.Collection;
import java.util.TreeMap;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.MinecraftServer;
import net.neoforged.fml.ModList;

final class EffectiveCatalog {
    private static boolean mekanism(ResourceLocation id) {
        return id != null && id.getNamespace().startsWith("mekanism");
    }

    private static JsonArray ids(Registry<?> registry) {
        return strings(registry.keySet().stream().filter(EffectiveCatalog::mekanism)
                .map(ResourceLocation::toString).toList());
    }

    private static JsonArray strings(Collection<String> values) {
        JsonArray result = new JsonArray();
        values.stream().sorted().forEach(result::add);
        return result;
    }

    static JsonObject capture(MinecraftServer server) {
        if (!"1".equals(System.getenv("MEKLAB_CATALOG")) || !server.isSameThread()
                || !server.tickRateManager().isFrozen() || !server.getPlayerList().getPlayers().isEmpty()) {
            throw new IllegalStateException("effective catalog requires the dedicated frozen bootstrap");
        }
        JsonObject modules = new JsonObject();
        ModList.get().getMods().forEach(mod -> modules.addProperty(mod.getModId(), mod.getVersion().toString()));
        JsonObject registries = new JsonObject();
        registries.add("blocks", ids(BuiltInRegistries.BLOCK));
        registries.add("items", ids(BuiltInRegistries.ITEM));
        registries.add("block_entities", ids(BuiltInRegistries.BLOCK_ENTITY_TYPE));
        registries.add("menus", ids(BuiltInRegistries.MENU));
        registries.add("fluids", ids(BuiltInRegistries.FLUID));
        registries.add("recipe_types", ids(BuiltInRegistries.RECIPE_TYPE));
        registries.add("recipe_serializers", ids(BuiltInRegistries.RECIPE_SERIALIZER));
        var chemical = BuiltInRegistries.REGISTRY.get(ResourceLocation.parse("mekanism:chemical"));
        if (chemical == null) throw new IllegalStateException("Mekanism chemical registry unavailable");
        registries.add("chemicals", ids(chemical));

        var sortedRecipes = new TreeMap<String, JsonObject>();
        for (var holder : server.getRecipeManager().getRecipes()) {
            var recipe = holder.value();
            var type = BuiltInRegistries.RECIPE_TYPE.getKey(recipe.getType());
            var serializer = BuiltInRegistries.RECIPE_SERIALIZER.getKey(recipe.getSerializer());
            if (mekanism(holder.id()) || mekanism(type)) {
                if (type == null || serializer == null) throw new IllegalStateException("Unregistered recipe binding");
                JsonObject binding = new JsonObject();
                binding.addProperty("type", type.toString());
                binding.addProperty("serializer", serializer.toString());
                sortedRecipes.put(holder.id().toString(), binding);
            }
        }
        JsonObject recipes = new JsonObject();
        sortedRecipes.forEach(recipes::add);
        JsonObject result = new JsonObject();
        result.addProperty("owner_thread", server.isSameThread());
        result.addProperty("tick", Long.toString(server.overworld().getGameTime()));
        result.add("modules", modules);
        result.add("registries", registries);
        result.add("recipes", recipes);
        result.add("packs", strings(server.getPackRepository().getSelectedIds()));
        result.add("resource_namespaces", strings(server.getResourceManager().getNamespaces()));
        return result;
    }
}
