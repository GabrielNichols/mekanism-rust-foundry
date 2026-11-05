package dev.meklab;
import com.google.gson.*;
import net.minecraft.nbt.*;
import java.util.TreeSet;

/** Preserve NBT widths, arrays, homogeneous lists and floating-point bits. */
final class NbtJson {
    static JsonObject encode(Tag tag) {
        JsonObject out=new JsonObject();out.addProperty("tag",tag.getId());
        JsonElement value;
        switch(tag.getId()) {
            case 1,2,3,4 -> value=new JsonPrimitive(((NumericTag)tag).getAsNumber().toString());
            case 5 -> value=new JsonPrimitive(String.format("%08x",Float.floatToRawIntBits(((NumericTag)tag).getAsFloat())));
            case 6 -> value=new JsonPrimitive(String.format("%016x",Double.doubleToRawLongBits(((NumericTag)tag).getAsDouble())));
            case 8 -> value=new JsonPrimitive(tag.getAsString());
            case 7 -> {JsonArray a=new JsonArray();for(byte v:((ByteArrayTag)tag).getAsByteArray())a.add(Byte.toString(v));value=a;}
            case 11 -> {JsonArray a=new JsonArray();for(int v:((IntArrayTag)tag).getAsIntArray())a.add(Integer.toString(v));value=a;}
            case 12 -> {JsonArray a=new JsonArray();for(long v:((LongArrayTag)tag).getAsLongArray())a.add(Long.toString(v));value=a;}
            case 9 -> {ListTag list=(ListTag)tag;out.addProperty("element_tag",list.getElementType());JsonArray a=new JsonArray();for(Tag v:list)a.add(encode(v));value=a;}
            case 10 -> {CompoundTag compound=(CompoundTag)tag;JsonObject a=new JsonObject();for(String key:new TreeSet<>(compound.getAllKeys()))a.add(key,encode(compound.get(key)));value=a;}
            default -> throw new IllegalArgumentException("unsupported NBT tag");
        }
        out.add("value",value);return out;
    }
}
