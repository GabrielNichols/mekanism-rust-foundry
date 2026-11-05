package dev.meklab;

import com.google.gson.*;
import net.minecraft.client.Minecraft;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Screenshot;
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.phys.BlockHitResult;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.client.event.ClientTickEvent;
import net.neoforged.neoforge.client.event.RenderFrameEvent;
import java.io.File;
import org.lwjgl.glfw.GLFW;

/** Real client bridge. Keybinding and Screen ingress are labelled separately from OS input. */
public final class ClientBridge {
    private static final boolean OFFSCREEN = "1".equals(System.getenv("MEKLAB_OFFSCREEN"));
    private static Wire wire;
    private static long frames;
    private static long clientTicks;
    private static int armedMenu = -1;
    private static int armedRevision = -1;
    private static long armedFrame;
    private static JsonObject pendingCapture;
    public static void register() {
        NeoForge.EVENT_BUS.addListener(ClientBridge::tick);
        NeoForge.EVENT_BUS.addListener(ClientBridge::rendered);
    }
    private static JsonObject stack(ItemStack stack) {
        JsonObject out=new JsonObject(); out.addProperty("item",BuiltInRegistries.ITEM.getKey(stack.getItem()).toString()); out.addProperty("count",stack.getCount()); return out;
    }
    private static JsonObject snapshot(Minecraft mc) {
        JsonObject out=new JsonObject(); out.addProperty("tick",Long.toString(mc.level.getGameTime()));
        out.addProperty("client_tick",Long.toString(clientTicks));
        out.addProperty("frame",Long.toString(frames));
        out.addProperty("target_block",BuiltInRegistries.BLOCK.getKey(mc.level.getBlockState(ServerBridge.TARGET).getBlock()).toString());
        out.addProperty("screen",mc.screen==null?"game":mc.screen.getClass().getName());
        out.addProperty("menu_id",mc.player.containerMenu.containerId);
        out.addProperty("menu_state_id", mc.player.containerMenu.getStateId());
        out.add("source", stack(mc.player.getInventory().getItem(9)));
        out.add("hotbar",stack(mc.player.getInventory().getItem(0))); out.add("cursor",stack(mc.player.containerMenu.getCarried()));
        out.addProperty("container_screen",mc.screen instanceof AbstractContainerScreen<?>);
        int machineOre=0;
        for(var slot:mc.player.containerMenu.slots) if(slot.container!=mc.player.getInventory()&&slot.getItem().is(Items.IRON_ORE)) machineOre+=slot.getItem().getCount();
        out.addProperty("machine_iron_ore",machineOre);
        int machineDust=0;for(var slot:mc.player.containerMenu.slots)if(slot.container!=mc.player.getInventory()&&BuiltInRegistries.ITEM.getKey(slot.getItem().getItem()).toString().equals("mekanism:dust_iron"))machineDust+=slot.getItem().getCount();out.addProperty("machine_dust_iron",machineDust);
        JsonArray slots=new JsonArray(); for(var slot:mc.player.containerMenu.slots) slots.add(stack(slot.getItem())); out.add("slots",slots);
        if(mc.hitResult instanceof BlockHitResult hit) {out.addProperty("hit",hit.getBlockPos().toShortString());out.addProperty("face",hit.getDirection().toString());}
        return out;
    }
    private static void tick(ClientTickEvent.Post event) {
        Minecraft mc=Minecraft.getInstance();
        clientTicks++;
        if(!mc.isSameThread())throw new IllegalStateException("client capture outside owner thread");
        if(mc.player==null||mc.level==null) return;
        if (OFFSCREEN) requireHidden(mc);
        if(wire==null) {JsonObject hello=new JsonObject();hello.addProperty("player",mc.player.getGameProfile().getName());hello.addProperty("ingress","keybinding_and_screen_handler");hello.addProperty("presentation", OFFSCREEN ? "hidden_glfw" : "windowed");wire=new Wire("client",hello);}
        JsonObject command;
        while((command=wire.queue.poll())!=null) {
            try {
                switch(command.get("action").getAsString()) {
                    case "read": break;
                    case "aim": mc.player.setYRot(180);mc.player.setXRot(28.4f);break;
                    case "click_use": KeyMapping.click(mc.options.keyUse.getKey());break;
                    case "click_slot": {
                        if(!(mc.screen instanceof AbstractContainerScreen<?> screen)) throw new IllegalStateException("container screen absent");
                        String selector=command.get("selector").getAsString();
                        var slot=screen.getMenu().slots.stream().filter(s->selector.equals("source")?
                            s.container==mc.player.getInventory()&&s.getContainerSlot()==9:
                            selector.equals("output")?s.container!=mc.player.getInventory()&&BuiltInRegistries.ITEM.getKey(s.getItem().getItem()).toString().equals("mekanism:dust_iron"):
                            s.container!=mc.player.getInventory()&&s.mayPlace(new ItemStack(Items.IRON_ORE))).findFirst().orElseThrow();
                        double x=screen.getGuiLeft()+slot.x+8,y=screen.getGuiTop()+slot.y+8;
                        boolean consumed=screen.mouseClicked(x,y,0);screen.mouseReleased(x,y,0);
                        if(!consumed) throw new IllegalStateException("Screen rejected click");
                        break;
                    }
                    case "barrier_arm": {
                        if (armedMenu != -1 || !(mc.screen instanceof AbstractContainerScreen<?>)
                                || command.get("menu_id").getAsInt() != mc.player.containerMenu.containerId
                                || command.get("menu_state_id").getAsInt() != mc.player.containerMenu.getStateId()) {
                            throw new IllegalStateException("authoritative revision has not reached the open menu");
                        }
                        armedMenu = mc.player.containerMenu.containerId;
                        armedRevision = mc.player.containerMenu.getStateId();
                        armedFrame = frames;
                        break;
                    }
                    case "barrier_capture": {
                        if (armedMenu == -1) {
                            wire.reply(command, snapshot(mc), captureReceipt("blocked", false, mc));
                            continue;
                        }
                        if (pendingCapture != null) throw new IllegalStateException("capture already pending");
                        pendingCapture = command;
                        continue;
                    }
                    case "capture": {
                        try(var image=Screenshot.takeScreenshot(mc.getMainRenderTarget())) {image.writeToFile(new File(mc.gameDirectory,"meklab-final.png"));}
                        break;
                    }
                    case "close": wire.reply(command,snapshot(mc));mc.stop();return;
                    default:throw new IllegalArgumentException("unsupported client action");
                }
                JsonObject receipt = new JsonObject();
                receipt.addProperty("owner_thread", mc.isSameThread());
                wire.reply(command,snapshot(mc),receipt);
            } catch(Exception error) {wire.error(command,error);}
        }
    }
    private static JsonObject captureReceipt(String gate, boolean captured, Minecraft mc) {
        JsonObject receipt = new JsonObject();
        receipt.addProperty("owner_thread", mc.isSameThread());
        JsonObject capture = new JsonObject();
        capture.addProperty("gate", gate);
        capture.addProperty("captured", captured);
        receipt.add("capture", capture);
        return receipt;
    }

    private static void requireHidden(Minecraft mc) {
        if (GLFW.glfwGetWindowAttrib(mc.getWindow().getWindow(), GLFW.GLFW_VISIBLE) != GLFW.GLFW_FALSE) {
            throw new IllegalStateException("offscreen laboratory window became visible");
        }
    }

    private static void rendered(RenderFrameEvent.Post event) {
        if (OFFSCREEN) requireHidden(Minecraft.getInstance());
        frames++;
        if (pendingCapture == null || frames <= armedFrame) return;
        Minecraft mc = Minecraft.getInstance();
        JsonObject command = pendingCapture;
        pendingCapture = null;
        try {
            if (!mc.isSameThread() || mc.player == null || mc.level == null
                    || !(mc.screen instanceof AbstractContainerScreen<?>)
                    || mc.player.containerMenu.containerId != armedMenu
                    || mc.player.containerMenu.getStateId() != armedRevision) {
                throw new IllegalStateException("menu changed before the rendered frame");
            }
            try (var image = Screenshot.takeScreenshot(mc.getMainRenderTarget())) {
                image.writeToFile(new File(mc.gameDirectory, "meklab-causal.png"));
            }
            wire.reply(command, snapshot(mc), captureReceipt("rendered", true, mc));
        } catch (Exception error) {
            wire.error(command, error);
        } finally {
            armedMenu = -1;
            armedRevision = -1;
        }
    }
}
