package dev.meklab;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.neoforged.neoforge.event.level.ChunkEvent;

final class ChunkProbe {
    private static final ChunkPos TARGET = new ChunkPos(2000, 2000);
    private final String mode;
    private int loads;
    private boolean used;

    ChunkProbe(String mode) {
        this.mode = mode;
    }

    boolean enabled() {
        return "read".equals(mode) || "load".equals(mode);
    }

    void loaded(ChunkEvent.Load event) {
        if (event.getLevel() instanceof ServerLevel level
                && level.dimension().equals(net.minecraft.world.level.Level.OVERWORLD)
                && event.getChunk().getPos().equals(TARGET)) {
            loads++;
        }
    }

    private JsonObject witness(ServerLevel level) {
        var source = level.getChunkSource();
        JsonObject state = new JsonObject();
        state.addProperty("tick", Long.toString(level.getGameTime()));
        state.addProperty("resident", source.getChunkNow(TARGET.x, TARGET.z) != null);
        state.addProperty("holder", source.chunkMap.getVisibleChunkIfPresent(TARGET.toLong()) != null);
        state.addProperty("forced", level.getForcedChunks().contains(TARGET.toLong()));
        state.addProperty("debug", source.getChunkDebugData(TARGET));
        return state;
    }

    JsonObject followup(MinecraftServer server) {
        if (!enabled() || !used || !server.isSameThread()) {
            throw new IllegalStateException("chunk followup requires a completed probe on the server thread");
        }
        JsonObject result = new JsonObject();
        result.addProperty("owner_thread", server.isSameThread());
        result.addProperty("load_events", loads);
        result.add("state", witness(server.overworld()));
        return result;
    }

    JsonObject run(MinecraftServer server) {
        if (!enabled() || used || !server.isSameThread()
                || !server.tickRateManager().isFrozen() || !server.getPlayerList().getPlayers().isEmpty()) {
            throw new IllegalStateException("chunk probe requires a one-shot frozen server fixture");
        }
        used = true;
        ServerLevel level = server.overworld();
        JsonObject before = witness(level);
        if (loads != 0 || before.get("resident").getAsBoolean() || before.get("holder").getAsBoolean()) {
            throw new IllegalStateException("chunk probe target is already loaded");
        }
        var source = level.getChunkSource();
        long start = System.nanoTime();
        if ("load".equals(mode)) {
            source.getChunk(TARGET.x, TARGET.z, ChunkStatus.FULL, true);
        } else {
            source.getChunkNow(TARGET.x, TARGET.z);
        }
        long lookupNs = System.nanoTime() - start;
        JsonObject result = new JsonObject();
        JsonArray coordinates = new JsonArray();
        coordinates.add(TARGET.x);
        coordinates.add(TARGET.z);
        result.add("chunk", coordinates);
        result.addProperty("owner_thread", server.isSameThread());
        result.add("before", before);
        result.add("after", witness(level));
        result.addProperty("load_events", loads);
        result.addProperty("lookup_ns", Long.toString(lookupNs));
        return result;
    }
}
