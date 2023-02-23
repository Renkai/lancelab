package cinterface;

import org.apache.arrow.c.ArrowArray;
import org.apache.arrow.c.ArrowSchema;
import org.apache.arrow.c.Data;
import org.apache.arrow.memory.BufferAllocator;
import org.apache.arrow.memory.RootAllocator;
import org.apache.arrow.vector.BigIntVector;
import org.apache.arrow.vector.FieldVector;

public class Converter {

    static {
        System.loadLibrary("lance_jni");
    }

    public static void getInt32ArrayExample() {
        BufferAllocator allocator = new RootAllocator();
        ArrowSchema arrowSchema = ArrowSchema.allocateNew(allocator);
        ArrowArray arrowArray = ArrowArray.allocateNew(allocator);
        ConverterJni.fill_arr(arrowArray.memoryAddress(), arrowSchema.memoryAddress());
        var vector = Data.importVector(allocator, arrowArray, arrowSchema, null);
        System.out.println("Rust allocated array: " + vector);
    }
}
