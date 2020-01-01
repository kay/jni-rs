public class LongToDoubleSwissTable {
    private static native long newLongToDoubleSwissTable(long capacity);

    private static native void destroyTable(long pointer);

    private static native void putAndAdd(long pointer, long key, double delta);

    static {
        System.loadLibrary("mylib");
    }

    private final long pointer;

    public LongToDoubleSwissTable(long capacity) {
        this.pointer = newLongToDoubleSwissTable(capacity);
    }

    public void putAndAdd(long key, double delta) {
        putAndAdd(pointer, key, delta);
    }

    public void destroy() {
        destroyTable(pointer);
    }

    public static void main(String[] args) {
        for (int i = 0; i < 20_000; i++) {
            var map = new LongToDoubleSwissTable(16);
            map.putAndAdd(17, 42);
            map.putAndAdd(17, 2);
            map.putAndAdd(17, 3);
            map.destroy();
        }
    }
}
