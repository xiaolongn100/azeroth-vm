
public class Sum {

    static {
        int sum = HelloWorld.count;
        for (int i = 0; i < 1000; i++) {
            sum += i;
        }
    }

    public static void main(String[] args) {
        int sum = 0;
        for (int i = 0; i < 1000; i++) {
            sum += i;
        }
    }
}
