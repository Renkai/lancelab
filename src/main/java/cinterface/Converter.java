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
        long[] arr = ConverterJni.getInt32Arr();
        try (ArrowSchema arrowSchema = ArrowSchema.wrap(arr[0]); ArrowArray array = ArrowArray.wrap(arr[1])) {
            var vec = Data.importVector(allocator, array, arrowSchema, null);
            System.out.println("rust allocated array: " + vec);
        }
    }
}
