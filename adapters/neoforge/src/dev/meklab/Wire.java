package dev.meklab;

import com.google.gson.*;
import java.io.*;
import java.net.*;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ConcurrentLinkedQueue;

/** One local connection; game mutations are dispatched by the owning game thread. */
final class Wire {
    static final Gson JSON = new Gson();
    final ConcurrentLinkedQueue<JsonObject> queue = new ConcurrentLinkedQueue<>();
    private final String session = System.getenv("MEKLAB_SESSION");
    private volatile PrintWriter writer;
    private long expectedSequence = 0;

    Wire(String role, JsonObject hello) {
        Thread thread = new Thread(() -> {
            try (Socket socket = new Socket(InetAddress.getLoopbackAddress(), Integer.parseInt(System.getenv("MEKLAB_PORT")))) {
                socket.setSoTimeout(120000);
                writer = new PrintWriter(new OutputStreamWriter(socket.getOutputStream(), StandardCharsets.UTF_8), true);
                hello.addProperty("kind", "hello"); hello.addProperty("protocol", 1);
                hello.addProperty("role", role); hello.addProperty("session", session);
                writer.println(JSON.toJson(hello));
                BufferedReader input = new BufferedReader(new InputStreamReader(socket.getInputStream(), StandardCharsets.UTF_8));
                String line;
                while ((line = input.readLine()) != null) {
                    if (line.length() > 65536) throw new IOException("oversize command");
                    JsonObject command = JsonParser.parseString(line).getAsJsonObject();
                    if (!session.equals(command.get("session").getAsString())) throw new IOException("wrong session");
                    if (command.get("sequence").getAsLong() != expectedSequence++) throw new IOException("sequence gap");
                    queue.add(command);
                }
            } catch (Exception error) { System.err.println("MekLab transport failed: " + error.getClass().getSimpleName()); }
        }, "meklab-local-control");
        thread.setDaemon(true); thread.start();
    }
    void reply(JsonObject command, JsonObject state) {
        reply(command,state,new JsonObject());
    }
    void reply(JsonObject command, JsonObject state,JsonObject extra) {
        JsonObject out = new JsonObject();
        out.addProperty("kind", "observation"); out.addProperty("sequence", command.get("sequence").getAsLong());
        out.addProperty("action", command.get("action").getAsString()); out.add("state", state);
        for(var entry:extra.entrySet())out.add(entry.getKey(),entry.getValue());
        writer.println(JSON.toJson(out));
    }
    void error(JsonObject command, Exception error) {
        JsonObject out = new JsonObject(); out.addProperty("error", error.getClass().getSimpleName() + ": " + error.getMessage());
        reply(command, out);
    }
}
