package dev.meklab.mixin;

import com.mojang.blaze3d.platform.Window;
import java.util.function.IntSupplier;
import java.util.function.LongSupplier;
import java.util.function.Supplier;
import net.neoforged.fml.loading.ImmediateWindowHandler;
import org.lwjgl.glfw.GLFW;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Redirect;

@Mixin(Window.class)
abstract class OffscreenWindowMixin {
    @Redirect(
        method = "<init>",
        at = @At(value = "INVOKE", target = "Lnet/neoforged/fml/loading/ImmediateWindowHandler;setupMinecraftWindow(Ljava/util/function/IntSupplier;Ljava/util/function/IntSupplier;Ljava/util/function/Supplier;Ljava/util/function/LongSupplier;)J", remap = false),
        require = 1,
        remap = false
    )
    private long meklab$createWindow(IntSupplier width, IntSupplier height, Supplier<String> title, LongSupplier monitor) {
        if (!"1".equals(System.getenv("MEKLAB_OFFSCREEN"))) {
            return ImmediateWindowHandler.setupMinecraftWindow(width, height, title, monitor);
        }
        GLFW.glfwWindowHint(GLFW.GLFW_VISIBLE, GLFW.GLFW_FALSE);
        GLFW.glfwWindowHint(GLFW.GLFW_FOCUSED, GLFW.GLFW_FALSE);
        long window = GLFW.glfwCreateWindow(width.getAsInt(), height.getAsInt(), title.get(), 0, 0);
        if (window == 0 || GLFW.glfwGetWindowAttrib(window, GLFW.GLFW_VISIBLE) != GLFW.GLFW_FALSE) {
            throw new IllegalStateException("laboratory client requires a hidden GLFW context");
        }
        System.out.println("MekLab GLFW context created hidden");
        return window;
    }
}
